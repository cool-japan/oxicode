//! Streaming decoder implementation.

use super::chunk::ChunkHeader;
use super::StreamingProgress;
#[cfg(feature = "alloc")]
use super::MAX_CHUNK_SIZE;
#[cfg(feature = "alloc")]
use crate::config::Config;
use crate::de::{Decode, DecoderImpl, SliceReader};
use crate::{config, Error, Result};

#[cfg(feature = "alloc")]
extern crate alloc;

#[cfg(feature = "std")]
use std::io::Read;

/// Largest chunk-payload slice read (and thus allocated) in one step when
/// materializing a chunk payload from a `std::io::Read` / `AsyncRead` stream.
///
/// A chunk header only carries a claimed `payload_len` (up to `MAX_CHUNK_SIZE`
/// after the bound check), not the bytes themselves; bounding the per-step
/// allocation means a header that lies about its payload — or a connection
/// that sends the header and then stalls — can only ever commit this much
/// memory before the reader has to actually produce more bytes, rather than
/// the full claimed length up front. `pub(crate)` because `AsyncStreamingDecoder`
/// (in `super::async_io`) uses the same bound for the identical reason.
#[cfg(feature = "std")]
pub(crate) const PAYLOAD_READ_STEP: usize = 64 * 1024;

/// A streaming decoder for reading items incrementally.
///
/// Reads chunks from the input and decodes items one at a time,
/// allowing processing of very large streams without loading
/// everything into memory.
///
/// The `C` type parameter controls the codec configuration (integer encoding,
/// endianness, byte limit).  Use [`StreamingDecoder::new`] to get the default
/// variable-width integer encoding, or [`StreamingDecoder::new_with_config`]
/// to select an alternative that matches the encoder's configuration.
///
/// # Robustness
///
/// * Chunk payloads are bounded before allocation: any header declaring a
///   payload larger than the acceptance bound (the smaller of [`MAX_CHUNK_SIZE`],
///   the configured `max_buffer_size`, and any codec byte limit) is rejected
///   with [`Error::LimitExceeded`] before a single byte is allocated.
/// * A stream that ends without the mandatory End chunk (truncation) is reported
///   as [`Error::UnexpectedEnd`], never as a clean end-of-stream.
/// * Metadata and zero-item chunks are transparently skipped so they cannot
///   truncate [`read_all`](Self::read_all).
/// * Once any decode error occurs the decoder is *poisoned*: every subsequent
///   call returns a deterministic error rather than misinterpreting payload
///   bytes as a fresh chunk header.
#[cfg(feature = "std")]
pub struct StreamingDecoder<R: Read, C: Config = config::Configuration> {
    reader: R,
    codec_config: C,
    current_chunk: Option<ChunkData>,
    progress: StreamingProgress,
    finished: bool,
    end_seen: bool,
    poisoned: bool,
    max_chunk_size: usize,
}

#[cfg(feature = "std")]
struct ChunkData {
    data: alloc::vec::Vec<u8>,
    offset: usize,
    items_remaining: u32,
}

#[cfg(feature = "std")]
impl<R: Read> StreamingDecoder<R> {
    /// Create a new streaming decoder using the standard codec configuration
    /// (little-endian, variable-width integer encoding).
    pub fn new(reader: R) -> Self {
        Self::new_with_config(reader, config::standard())
    }
}

#[cfg(feature = "std")]
impl<R: Read, C: Config> StreamingDecoder<R, C> {
    /// Create a streaming decoder with a custom codec configuration.
    ///
    /// The codec configuration **must match** the one used by the encoder,
    /// otherwise decoding will produce incorrect values or errors.
    ///
    /// ```rust,ignore
    /// use oxicode::streaming::{StreamingDecoder, StreamingEncoder};
    ///
    /// let codec = oxicode::config::standard().with_fixed_int_encoding();
    /// let mut encoder = StreamingEncoder::new_with_config(&mut buf, codec);
    /// // …encode…
    /// let mut decoder = StreamingDecoder::new_with_config(Cursor::new(buf), codec);
    /// ```
    pub fn new_with_config(reader: R, codec_config: C) -> Self {
        Self {
            reader,
            codec_config,
            current_chunk: None,
            progress: StreamingProgress::default(),
            finished: false,
            end_seen: false,
            poisoned: false,
            max_chunk_size: MAX_CHUNK_SIZE,
        }
    }

    /// Create a streaming decoder selecting both the streaming configuration
    /// (whose `max_buffer_size` bounds the largest chunk that will be accepted)
    /// and the codec configuration.
    pub fn new_with_configs(
        reader: R,
        streaming_config: super::StreamingConfig,
        codec_config: C,
    ) -> Self {
        let mut decoder = Self::new_with_config(reader, codec_config);
        decoder.max_chunk_size = MAX_CHUNK_SIZE.min(streaming_config.max_buffer_size.max(1));
        decoder
    }

    /// The maximum chunk payload this decoder will accept before allocating.
    fn decode_bound(&self) -> usize {
        let mut bound = self.max_chunk_size;
        if let Some(limit) = self.codec_config.limit() {
            bound = bound.min(limit);
        }
        bound
    }

    /// Read the next item from the stream.
    ///
    /// Returns `None` only when the stream has been cleanly terminated by an
    /// End chunk.  Truncation or malformed input yields an error.
    pub fn read_item<T: Decode>(&mut self) -> Result<Option<T>> {
        if self.poisoned {
            return Err(Error::InvalidData {
                message: "streaming decoder in failed state",
            });
        }
        if self.finished {
            return Ok(None);
        }

        // Load chunks until we reach one that actually contains items, skipping
        // Metadata / empty (zero-item) chunks so they cannot truncate reads.
        loop {
            let has_items = self
                .current_chunk
                .as_ref()
                .map(|c| c.items_remaining != 0)
                .unwrap_or(false);
            if has_items {
                break;
            }
            if !self.load_next_chunk()? {
                return Ok(None);
            }
        }

        // Decode item from current chunk
        let chunk = self.current_chunk.as_mut().ok_or(Error::InvalidData {
            message: "no chunk available",
        })?;

        // Create reader from remaining chunk data, using the stored codec config.
        let reader = SliceReader::new(&chunk.data[chunk.offset..]);
        let mut decoder = DecoderImpl::new(reader, self.codec_config);
        let item = match T::decode(&mut decoder) {
            Ok(item) => item,
            Err(e) => {
                // Poison on item-level errors too, matching the documented
                // contract ("once any decode error occurs the decoder is
                // poisoned") and the chunk-level error sites below. Without
                // this, `chunk.offset` is never advanced on a failed decode,
                // so a caller that loops past the error would re-decode the
                // exact same bytes forever instead of getting a deterministic
                // failed-state error on the next call.
                self.poisoned = true;
                self.finished = true;
                return Err(e);
            }
        };

        // Update offset based on how much was read
        let bytes_consumed = chunk.data[chunk.offset..].len() - decoder.reader().slice.len();
        chunk.offset += bytes_consumed;
        chunk.items_remaining -= 1;

        self.progress.items_processed += 1;
        self.progress.bytes_processed += bytes_consumed as u64;

        Ok(Some(item))
    }

    /// Read all remaining items into a vector.
    #[cfg(feature = "alloc")]
    pub fn read_all<T: Decode>(&mut self) -> Result<alloc::vec::Vec<T>> {
        let mut items = alloc::vec::Vec::new();
        while let Some(item) = self.read_item()? {
            items.push(item);
        }
        Ok(items)
    }

    /// Load the next chunk from the reader.
    ///
    /// Returns `Ok(true)` when a chunk was loaded, `Ok(false)` when the stream
    /// was cleanly terminated by an End chunk, and `Err` on truncation, an
    /// over-large payload, or an I/O error (which also poisons the decoder).
    fn load_next_chunk(&mut self) -> Result<bool> {
        // Read chunk header
        let mut header_bytes = [0u8; ChunkHeader::SIZE];
        match self.reader.read_exact(&mut header_bytes) {
            Ok(()) => {}
            Err(e) if e.kind() == std::io::ErrorKind::UnexpectedEof => {
                // EOF before an End chunk: the stream is truncated.  The End
                // chunk is mandatory, so this is data loss, not a clean finish.
                self.poisoned = true;
                self.finished = true;
                return Err(Error::UnexpectedEnd {
                    additional: ChunkHeader::SIZE,
                });
            }
            Err(e) => {
                self.poisoned = true;
                self.finished = true;
                return Err(Error::Io {
                    kind: e.kind(),
                    message: e.to_string(),
                });
            }
        }

        let header = match ChunkHeader::from_bytes(&header_bytes) {
            Ok(h) => h,
            Err(e) => {
                self.poisoned = true;
                self.finished = true;
                return Err(e);
            }
        };

        // Check for end chunk
        if header.is_end() {
            self.finished = true;
            self.end_seen = true;
            return Ok(false);
        }

        // Enforce the chunk-size bound before allocating anything.
        let bound = self.decode_bound();
        if header.payload_len as usize > bound {
            self.poisoned = true;
            self.finished = true;
            return Err(Error::LimitExceeded {
                limit: bound as u64,
                found: header.payload_len as u64,
            });
        }

        // Read chunk payload (payload_len is now bounded by `bound`, so the
        // *ceiling* on this allocation is already capped). Still materialize
        // it incrementally rather than `vec![0u8; payload_len]` + `read_exact`:
        // a header is only 13 bytes, so without this a connection that sends a
        // header and then stalls (or a hostile peer that never sends the
        // payload at all) would still cost the receiver a full `bound`-sized
        // allocation — up to MAX_CHUNK_SIZE (16 MiB) by default — before a
        // single payload byte has actually arrived. Growing in bounded
        // `PAYLOAD_READ_STEP` increments means the allocation tracks bytes
        // actually received off the wire instead of the header's claim.
        let payload_len = header.payload_len as usize;
        let mut data: alloc::vec::Vec<u8> = alloc::vec::Vec::new();
        let mut filled = 0usize;
        while filled < payload_len {
            // `resize` only ever grows here: `data.len()` is always <=
            // `payload_len` (by construction of `step` below) and never
            // shrinks below `filled`, so re-resizing to the same or a larger
            // target on the next iteration (including after retrying an
            // `Interrupted` read below) is a cheap no-op / pure growth, never
            // a truncation of bytes already read into `data[..filled]`.
            let step = core::cmp::min(PAYLOAD_READ_STEP, payload_len - filled);
            data.resize(filled + step, 0u8);
            match self.reader.read(&mut data[filled..filled + step]) {
                Ok(0) => {
                    // EOF before the payload was fully received: truncated stream.
                    self.poisoned = true;
                    self.finished = true;
                    return Err(Error::UnexpectedEnd {
                        additional: payload_len - filled,
                    });
                }
                Ok(n) => filled += n,
                Err(e) if e.kind() == std::io::ErrorKind::Interrupted => {}
                Err(e) => {
                    self.poisoned = true;
                    self.finished = true;
                    return Err(Error::Io {
                        kind: e.kind(),
                        message: e.to_string(),
                    });
                }
            }
        }

        self.current_chunk = Some(ChunkData {
            data,
            offset: 0,
            items_remaining: header.item_count,
        });

        self.progress.chunks_processed += 1;

        Ok(true)
    }

    /// Get current progress.
    pub fn progress(&self) -> &StreamingProgress {
        &self.progress
    }

    /// Check if the stream is finished.
    pub fn is_finished(&self) -> bool {
        self.finished
    }

    /// Whether a valid End chunk terminated the stream.
    pub fn end_marker_seen(&self) -> bool {
        self.end_seen
    }

    /// Get a reference to the underlying reader.
    pub fn get_ref(&self) -> &R {
        &self.reader
    }
}

/// Streaming decoder for in-memory buffers (no std required).
///
/// The `C` type parameter selects the codec configuration used to decode items.
///
/// Shares [`StreamingDecoder`]'s poisoning guarantee: once any decode error
/// occurs (chunk-level or item-level), the decoder is poisoned and every
/// subsequent call returns a deterministic error rather than misinterpreting
/// payload bytes as a fresh chunk header or re-attempting a decode that will
/// only fail the same way again.
#[cfg(feature = "alloc")]
pub struct BufferStreamingDecoder<'a, C: Config = config::Configuration> {
    data: &'a [u8],
    codec_config: C,
    offset: usize,
    current_chunk_end: usize,
    items_remaining_in_chunk: u32,
    progress: StreamingProgress,
    finished: bool,
    end_seen: bool,
    poisoned: bool,
    max_chunk_size: usize,
}

#[cfg(feature = "alloc")]
impl<'a> BufferStreamingDecoder<'a> {
    /// Create a new buffer streaming decoder using the standard codec configuration.
    pub fn new(data: &'a [u8]) -> Self {
        Self::new_with_config(data, config::standard())
    }
}

#[cfg(feature = "alloc")]
impl<'a, C: Config> BufferStreamingDecoder<'a, C> {
    /// Create a buffer streaming decoder with a custom codec configuration.
    pub fn new_with_config(data: &'a [u8], codec_config: C) -> Self {
        Self {
            data,
            codec_config,
            offset: 0,
            current_chunk_end: 0,
            items_remaining_in_chunk: 0,
            progress: StreamingProgress::default(),
            finished: false,
            end_seen: false,
            poisoned: false,
            max_chunk_size: MAX_CHUNK_SIZE,
        }
    }

    /// Create a buffer streaming decoder selecting both the streaming
    /// configuration (whose `max_buffer_size` bounds the largest chunk that will
    /// be accepted) and the codec configuration.
    pub fn new_with_configs(
        data: &'a [u8],
        streaming_config: super::StreamingConfig,
        codec_config: C,
    ) -> Self {
        let mut decoder = Self::new_with_config(data, codec_config);
        decoder.max_chunk_size = MAX_CHUNK_SIZE.min(streaming_config.max_buffer_size.max(1));
        decoder
    }

    fn decode_bound(&self) -> usize {
        let mut bound = self.max_chunk_size;
        if let Some(limit) = self.codec_config.limit() {
            bound = bound.min(limit);
        }
        bound
    }

    /// Read the next item from the buffer.
    pub fn read_item<T: Decode>(&mut self) -> Result<Option<T>> {
        if self.poisoned {
            return Err(Error::InvalidData {
                message: "streaming decoder in failed state",
            });
        }
        if self.finished {
            return Ok(None);
        }

        // Skip Metadata / zero-item chunks until a chunk with items is found.
        loop {
            if self.items_remaining_in_chunk != 0 {
                break;
            }
            if !self.load_next_chunk()? {
                return Ok(None);
            }
        }

        // Decode item
        let reader = SliceReader::new(&self.data[self.offset..self.current_chunk_end]);
        let mut decoder = DecoderImpl::new(reader, self.codec_config);
        let item = match T::decode(&mut decoder) {
            Ok(item) => item,
            Err(e) => {
                // See the identical poisoning at the same point in
                // `StreamingDecoder::read_item` for the rationale: item-level
                // errors must poison too, or a caller looping past the error
                // spins on the same unadvanced offset forever.
                self.poisoned = true;
                self.finished = true;
                return Err(e);
            }
        };

        let bytes_consumed = (self.current_chunk_end - self.offset) - decoder.reader().slice.len();
        self.offset += bytes_consumed;
        self.items_remaining_in_chunk -= 1;

        self.progress.items_processed += 1;
        self.progress.bytes_processed += bytes_consumed as u64;

        Ok(Some(item))
    }

    /// Read all remaining items.
    pub fn read_all<T: Decode>(&mut self) -> Result<alloc::vec::Vec<T>> {
        let mut items = alloc::vec::Vec::new();
        while let Some(item) = self.read_item()? {
            items.push(item);
        }
        Ok(items)
    }

    /// Load the next chunk.
    fn load_next_chunk(&mut self) -> Result<bool> {
        if self.offset >= self.data.len() {
            // Buffer exhausted without an End chunk: truncated stream.
            self.poisoned = true;
            self.finished = true;
            return Err(Error::UnexpectedEnd {
                additional: ChunkHeader::SIZE,
            });
        }

        let remaining = &self.data[self.offset..];
        if remaining.len() < ChunkHeader::SIZE {
            // A partial trailing header is a truncation, not a clean end.
            self.poisoned = true;
            self.finished = true;
            return Err(Error::UnexpectedEnd {
                additional: ChunkHeader::SIZE - remaining.len(),
            });
        }

        let header = match ChunkHeader::from_bytes(remaining) {
            Ok(h) => h,
            Err(e) => {
                self.poisoned = true;
                self.finished = true;
                return Err(e);
            }
        };
        self.offset += ChunkHeader::SIZE;

        if header.is_end() {
            self.finished = true;
            self.end_seen = true;
            return Ok(false);
        }

        let bound = self.decode_bound();
        if header.payload_len as usize > bound {
            self.poisoned = true;
            self.finished = true;
            return Err(Error::LimitExceeded {
                limit: bound as u64,
                found: header.payload_len as u64,
            });
        }

        if self.data.len() < self.offset + header.payload_len as usize {
            self.poisoned = true;
            self.finished = true;
            return Err(Error::UnexpectedEnd {
                additional: (self.offset + header.payload_len as usize) - self.data.len(),
            });
        }

        self.current_chunk_end = self.offset + header.payload_len as usize;
        self.items_remaining_in_chunk = header.item_count;
        self.progress.chunks_processed += 1;

        // A zero-item chunk (e.g. Metadata) carries no items to decode, so no
        // read_item call will advance `offset` past its payload.  Skip the
        // payload here so the next chunk header is read from the right position.
        if header.item_count == 0 {
            self.offset = self.current_chunk_end;
        }

        Ok(true)
    }

    /// Get current progress.
    pub fn progress(&self) -> &StreamingProgress {
        &self.progress
    }

    /// Check if finished.
    pub fn is_finished(&self) -> bool {
        self.finished
    }

    /// Whether a valid End chunk terminated the stream.
    pub fn end_marker_seen(&self) -> bool {
        self.end_seen
    }
}

#[cfg(test)]
mod tests {
    use super::super::encoder::BufferStreamingEncoder;
    use super::*;

    #[cfg(feature = "alloc")]
    #[test]
    fn test_buffer_roundtrip() {
        // Encode
        let mut encoder = BufferStreamingEncoder::new();
        let values: alloc::vec::Vec<u32> = (0..100).collect();
        for v in &values {
            encoder.write_item(v).expect("write failed");
        }
        let encoded = encoder.finish();

        // Decode
        let mut decoder = BufferStreamingDecoder::new(&encoded);
        let decoded: alloc::vec::Vec<u32> = decoder.read_all().expect("read failed");

        assert_eq!(values, decoded);
        assert!(decoder.is_finished());
    }

    #[cfg(feature = "alloc")]
    #[test]
    fn test_item_by_item() {
        let mut encoder = BufferStreamingEncoder::new();
        encoder.write_item(&1u32).expect("write failed");
        encoder.write_item(&2u32).expect("write failed");
        encoder.write_item(&3u32).expect("write failed");
        let encoded = encoder.finish();

        let mut decoder = BufferStreamingDecoder::new(&encoded);

        assert_eq!(decoder.read_item::<u32>().expect("read failed"), Some(1));
        assert_eq!(decoder.read_item::<u32>().expect("read failed"), Some(2));
        assert_eq!(decoder.read_item::<u32>().expect("read failed"), Some(3));
        assert_eq!(decoder.read_item::<u32>().expect("read failed"), None);
    }

    #[cfg(feature = "std")]
    #[test]
    fn test_io_roundtrip() {
        use super::super::encoder::StreamingEncoder;
        use std::io::Cursor;

        // Encode
        let mut buffer = alloc::vec::Vec::new();
        {
            let mut encoder = StreamingEncoder::new(&mut buffer);
            for i in 0..50u32 {
                encoder.write_item(&i).expect("write failed");
            }
            encoder.finish().expect("finish failed");
        }

        // Decode
        let cursor = Cursor::new(buffer);
        let mut decoder = StreamingDecoder::new(cursor);
        let decoded: alloc::vec::Vec<u32> = decoder.read_all().expect("read failed");

        let expected: alloc::vec::Vec<u32> = (0..50).collect();
        assert_eq!(expected, decoded);
    }

    #[cfg(feature = "alloc")]
    #[test]
    fn test_progress_tracking() {
        let mut encoder = BufferStreamingEncoder::new();
        for i in 0..10u32 {
            encoder.write_item(&i).expect("write failed");
        }
        let encoded = encoder.finish();

        let mut decoder = BufferStreamingDecoder::new(&encoded);
        let _: alloc::vec::Vec<u32> = decoder.read_all().expect("read failed");

        assert_eq!(decoder.progress().items_processed, 10);
        assert!(decoder.progress().chunks_processed >= 1);
    }

    // ── Regression tests: issue #1 — new_with_config constructors ──────────

    /// Verify that `StreamingDecoder::new_with_config` with fixed-width integer
    /// encoding correctly roundtrips values encoded by a matching encoder.
    #[cfg(feature = "std")]
    #[test]
    fn test_streaming_decoder_with_fixed_int_config() {
        use super::super::encoder::StreamingEncoder;
        use std::io::Cursor;

        let codec = crate::config::standard().with_fixed_int_encoding();

        // Encode with fixed-width integers.
        let mut buffer = alloc::vec::Vec::new();
        {
            let mut encoder = StreamingEncoder::new_with_config(&mut buffer, codec);
            for i in 0u32..20 {
                encoder.write_item(&i).expect("write failed");
            }
            encoder.finish().expect("finish failed");
        }

        // Decode with the same fixed-width config.
        let cursor = Cursor::new(buffer);
        let mut decoder = StreamingDecoder::new_with_config(cursor, codec);
        let decoded: alloc::vec::Vec<u32> = decoder.read_all().expect("read_all failed");

        let expected: alloc::vec::Vec<u32> = (0..20).collect();
        assert_eq!(expected, decoded, "fixed-int roundtrip mismatch");
    }
}
