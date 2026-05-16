//! Streaming decoder implementation.

use super::chunk::ChunkHeader;
use super::StreamingProgress;
use crate::config::Config;
use crate::de::{Decode, DecoderImpl, SliceReader};
use crate::{config, Error, Result};

#[cfg(feature = "alloc")]
extern crate alloc;

#[cfg(feature = "std")]
use std::io::Read;

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
#[cfg(feature = "std")]
pub struct StreamingDecoder<R: Read, C: Config = config::Configuration> {
    reader: R,
    codec_config: C,
    current_chunk: Option<ChunkData>,
    progress: StreamingProgress,
    finished: bool,
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
        }
    }

    /// Read the next item from the stream.
    ///
    /// Returns `None` when the stream is exhausted.
    pub fn read_item<T: Decode>(&mut self) -> Result<Option<T>> {
        if self.finished {
            return Ok(None);
        }

        // Load next chunk if needed
        let needs_chunk = self.current_chunk.is_none()
            || self
                .current_chunk
                .as_ref()
                .map(|c| c.items_remaining == 0)
                .unwrap_or(true);
        if needs_chunk && !self.load_next_chunk()? {
            return Ok(None);
        }

        // Decode item from current chunk
        let chunk = self.current_chunk.as_mut().ok_or(Error::InvalidData {
            message: "no chunk available",
        })?;

        if chunk.items_remaining == 0 {
            return Ok(None);
        }

        // Create reader from remaining chunk data, using the stored codec config.
        let reader = SliceReader::new(&chunk.data[chunk.offset..]);
        let mut decoder = DecoderImpl::new(reader, self.codec_config);
        let item = T::decode(&mut decoder)?;

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
    fn load_next_chunk(&mut self) -> Result<bool> {
        // Read chunk header
        let mut header_bytes = [0u8; ChunkHeader::SIZE];
        match self.reader.read_exact(&mut header_bytes) {
            Ok(()) => {}
            Err(e) if e.kind() == std::io::ErrorKind::UnexpectedEof => {
                self.finished = true;
                return Ok(false);
            }
            Err(e) => {
                return Err(Error::Io {
                    kind: e.kind(),
                    message: e.to_string(),
                });
            }
        }

        let header = ChunkHeader::from_bytes(&header_bytes)?;

        // Check for end chunk
        if header.is_end() {
            self.finished = true;
            return Ok(false);
        }

        // Read chunk payload
        let mut data = alloc::vec![0u8; header.payload_len as usize];
        self.reader.read_exact(&mut data).map_err(|e| Error::Io {
            kind: e.kind(),
            message: e.to_string(),
        })?;

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

    /// Get a reference to the underlying reader.
    pub fn get_ref(&self) -> &R {
        &self.reader
    }
}

/// Streaming decoder for in-memory buffers (no std required).
#[cfg(feature = "alloc")]
pub struct BufferStreamingDecoder<'a> {
    data: &'a [u8],
    offset: usize,
    current_chunk_end: usize,
    items_remaining_in_chunk: u32,
    progress: StreamingProgress,
    finished: bool,
}

#[cfg(feature = "alloc")]
impl<'a> BufferStreamingDecoder<'a> {
    /// Create a new buffer streaming decoder.
    pub fn new(data: &'a [u8]) -> Self {
        Self {
            data,
            offset: 0,
            current_chunk_end: 0,
            items_remaining_in_chunk: 0,
            progress: StreamingProgress::default(),
            finished: false,
        }
    }

    /// Read the next item from the buffer.
    pub fn read_item<T: Decode>(&mut self) -> Result<Option<T>> {
        if self.finished {
            return Ok(None);
        }

        // Load next chunk if needed
        if self.items_remaining_in_chunk == 0 && !self.load_next_chunk()? {
            return Ok(None);
        }

        if self.items_remaining_in_chunk == 0 {
            return Ok(None);
        }

        // Decode item
        let reader = SliceReader::new(&self.data[self.offset..self.current_chunk_end]);
        let mut decoder = DecoderImpl::new(reader, config::standard());
        let item = T::decode(&mut decoder)?;

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
            self.finished = true;
            return Ok(false);
        }

        let remaining = &self.data[self.offset..];
        if remaining.len() < ChunkHeader::SIZE {
            self.finished = true;
            return Ok(false);
        }

        let header = ChunkHeader::from_bytes(remaining)?;
        self.offset += ChunkHeader::SIZE;

        if header.is_end() {
            self.finished = true;
            return Ok(false);
        }

        if self.data.len() < self.offset + header.payload_len as usize {
            return Err(Error::UnexpectedEnd {
                additional: (self.offset + header.payload_len as usize) - self.data.len(),
            });
        }

        self.current_chunk_end = self.offset + header.payload_len as usize;
        self.items_remaining_in_chunk = header.item_count;
        self.progress.chunks_processed += 1;

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
