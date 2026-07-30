//! Async streaming encoder and decoder implementations.
//!
//! Provides async versions of `StreamingEncoder` and `StreamingDecoder`
//! for use with tokio or other async runtimes.
//!
//! # Cancellation safety
//!
//! Both the async encoder and decoder are cancellation-safe: if a `read_item`,
//! `write_item`, `read_all`, `write_all`, or `finish` future is dropped
//! mid-await (for example because it lost a `tokio::select!` race or timed out),
//! no bytes are lost or duplicated and the stream is not corrupted. Partially
//! read or written frame data is retained inside the encoder/decoder and is
//! resumed on the next call. This is achieved by persisting an in-flight fill
//! cursor in the struct and driving the transfer with the cancel-safe
//! [`AsyncReadExt::read`] / [`AsyncWriteExt::write`] primitives rather than the
//! non-cancel-safe `read_exact` / `write_all`.

use super::chunk::ChunkHeader;
use super::{StreamingConfig, StreamingProgress, MAX_CHUNK_SIZE};
use crate::config::Config;
use crate::de::{Decode, DecoderImpl, SliceReader};
use crate::enc::{Encode, EncoderImpl, VecWriter};
use crate::{config, Error, Result};

#[cfg(feature = "alloc")]
extern crate alloc;

#[cfg(feature = "async-tokio")]
use tokio::io::{AsyncRead, AsyncReadExt, AsyncWrite, AsyncWriteExt};

/// An async streaming encoder for writing items incrementally.
///
/// Uses tokio's async IO traits for non-blocking encoding operations.
///
/// The `C` type parameter selects the codec configuration; use
/// [`AsyncStreamingEncoder::new`] for the standard variable-width integer
/// encoding or [`AsyncStreamingEncoder::new_with_config`] to choose another
/// (it must match the decoder's configuration).
///
/// # Example
///
/// ```rust,ignore
/// use oxicode::streaming::AsyncStreamingEncoder;
/// use tokio::fs::File;
///
/// #[tokio::main]
/// async fn main() -> Result<(), Box<dyn std::error::Error>> {
///     let file = File::create("output.bin").await?;
///     let mut encoder = AsyncStreamingEncoder::new(file);
///
///     for i in 0..1000u32 {
///         encoder.write_item(&i).await?;
///     }
///
///     encoder.finish().await?;
///     Ok(())
/// }
/// ```
#[cfg(feature = "async-tokio")]
pub struct AsyncStreamingEncoder<W: AsyncWrite + Unpin, C: Config = config::Configuration> {
    writer: W,
    config: StreamingConfig,
    codec_config: C,
    buffer: alloc::vec::Vec<u8>,
    items_in_buffer: u32,
    progress: StreamingProgress,
    /// Fully-framed bytes (header + payload) still to be written to `writer`.
    /// Non-empty only while a flush is in flight; retained across future drops
    /// so an interrupted write resumes exactly where it left off.
    pending_frame: alloc::vec::Vec<u8>,
    /// Number of bytes of `pending_frame` already handed to `writer`.
    pending_written: usize,
}

#[cfg(feature = "async-tokio")]
impl<W: AsyncWrite + Unpin> AsyncStreamingEncoder<W> {
    /// Create a new async streaming encoder using the standard codec configuration.
    pub fn new(writer: W) -> Self {
        Self::new_with_configs(writer, StreamingConfig::default(), config::standard())
    }

    /// Create an async streaming encoder with custom chunking configuration and
    /// the standard codec configuration.
    pub fn with_config(writer: W, config: StreamingConfig) -> Self {
        Self::new_with_configs(writer, config, config::standard())
    }
}

#[cfg(feature = "async-tokio")]
impl<W: AsyncWrite + Unpin, C: Config> AsyncStreamingEncoder<W, C> {
    /// Create an async streaming encoder with a custom codec configuration.
    pub fn new_with_config(writer: W, codec_config: C) -> Self {
        Self::new_with_configs(writer, StreamingConfig::default(), codec_config)
    }

    /// Create an async streaming encoder selecting both the chunking and codec
    /// configurations.
    pub fn new_with_configs(writer: W, config: StreamingConfig, codec_config: C) -> Self {
        Self {
            writer,
            config,
            codec_config,
            buffer: alloc::vec::Vec::new(),
            items_in_buffer: 0,
            progress: StreamingProgress::default(),
            pending_frame: alloc::vec::Vec::new(),
            pending_written: 0,
        }
    }

    /// Set the estimated total number of items (for progress reporting).
    pub fn set_estimated_total(&mut self, total: u64) {
        self.progress.estimated_total = Some(total);
    }

    /// Drain any bytes still pending from a previous (possibly interrupted)
    /// flush. Cancellation-safe: each `write` is cancel-safe and the cursor
    /// lives in `self`, so a dropped future simply resumes here next time.
    async fn drain_pending(&mut self) -> Result<()> {
        while self.pending_written < self.pending_frame.len() {
            let n = self
                .writer
                .write(&self.pending_frame[self.pending_written..])
                .await
                .map_err(|e| Error::Io {
                    kind: e.kind(),
                    message: e.to_string(),
                })?;
            if n == 0 {
                return Err(Error::Io {
                    kind: std::io::ErrorKind::WriteZero,
                    message: "async writer accepted zero bytes".to_string(),
                });
            }
            self.pending_written += n;
        }
        self.pending_frame.clear();
        self.pending_written = 0;
        Ok(())
    }

    /// Write a single item to the stream asynchronously.
    pub async fn write_item<T: Encode>(&mut self, item: &T) -> Result<()> {
        // Finish any interrupted flush before staging new data so on-wire order
        // is preserved even after a mid-flush cancellation.
        self.drain_pending().await?;

        // Encode item to temporary buffer using the stored codec configuration.
        let item_writer = VecWriter::new();
        let mut encoder = EncoderImpl::new(item_writer, self.codec_config);
        item.encode(&mut encoder)?;
        let item_bytes = encoder.into_writer().into_vec();

        // Reject items too large for a single chunk (keeps chunks within
        // MAX_CHUNK_SIZE and length fields within u32).
        if item_bytes.len() > MAX_CHUNK_SIZE {
            return Err(Error::LimitExceeded {
                limit: MAX_CHUNK_SIZE as u64,
                found: item_bytes.len() as u64,
            });
        }

        // Flush before the pending buffer would exceed the effective threshold.
        let threshold = self.config.chunk_size.min(self.config.max_buffer_size);
        if !self.buffer.is_empty() && self.buffer.len() + item_bytes.len() > threshold {
            self.flush_chunk().await?;
        }

        // Guard against item_count overflow for zero-sized items.
        if self.items_in_buffer == u32::MAX {
            self.flush_chunk().await?;
        }

        // Add item to buffer
        self.buffer.extend_from_slice(&item_bytes);
        self.items_in_buffer += 1;

        // Flush if configured to flush per item
        if self.config.flush_per_item {
            self.flush_chunk().await?;
        }

        Ok(())
    }

    /// Write multiple items from an iterator asynchronously.
    pub async fn write_all<T: Encode, I: IntoIterator<Item = T>>(
        &mut self,
        items: I,
    ) -> Result<()> {
        for item in items {
            self.write_item(&item).await?;
        }
        Ok(())
    }

    /// Flush the current buffer as a chunk.
    async fn flush_chunk(&mut self) -> Result<()> {
        // Complete any interrupted previous flush first.
        self.drain_pending().await?;

        // Emit based on the item count so zero-sized-item chunks are preserved.
        if self.items_in_buffer == 0 {
            return Ok(());
        }

        // Checked length conversion (buffer is bounded by write_item).
        let payload_len = u32::try_from(self.buffer.len()).map_err(|_| Error::LimitExceeded {
            limit: MAX_CHUNK_SIZE as u64,
            found: self.buffer.len() as u64,
        })?;

        // Build the complete frame (header + payload) into the pending buffer so
        // the write becomes a single resumable transfer.
        let header = ChunkHeader::data(payload_len, self.items_in_buffer);
        self.pending_frame.clear();
        self.pending_frame.extend_from_slice(&header.to_bytes());
        self.pending_frame.extend_from_slice(&self.buffer);
        self.pending_written = 0;

        // Progress + reset staging (data is now committed to pending_frame).
        self.progress.items_processed += self.items_in_buffer as u64;
        self.progress.bytes_processed += self.buffer.len() as u64;
        self.progress.chunks_processed += 1;
        self.buffer.clear();
        self.items_in_buffer = 0;

        self.drain_pending().await
    }

    /// Finish the stream, writing any remaining data and the end marker.
    pub async fn finish(mut self) -> Result<W> {
        // Flush remaining buffer (also drains any interrupted flush).
        self.flush_chunk().await?;

        // Write end chunk as a resumable transfer.
        let end_header = ChunkHeader::end();
        self.pending_frame.clear();
        self.pending_frame.extend_from_slice(&end_header.to_bytes());
        self.pending_written = 0;
        self.drain_pending().await?;

        // Flush the writer
        self.writer.flush().await.map_err(|e| Error::Io {
            kind: e.kind(),
            message: e.to_string(),
        })?;

        Ok(self.writer)
    }

    /// Get current progress.
    pub fn progress(&self) -> &StreamingProgress {
        &self.progress
    }

    /// Get a reference to the underlying writer.
    pub fn get_ref(&self) -> &W {
        &self.writer
    }
}

/// An async streaming decoder for reading items incrementally.
///
/// Uses tokio's async IO traits for non-blocking decoding operations.
///
/// The `C` type parameter selects the codec configuration; it must match the
/// encoder's. See the module-level docs above for the robustness and
/// cancellation-safety guarantees.
///
/// # Example
///
/// ```rust,ignore
/// use oxicode::streaming::AsyncStreamingDecoder;
/// use tokio::fs::File;
///
/// #[tokio::main]
/// async fn main() -> Result<(), Box<dyn std::error::Error>> {
///     let file = File::open("input.bin").await?;
///     let mut decoder = AsyncStreamingDecoder::new(file);
///
///     while let Some(item) = decoder.read_item::<u32>().await? {
///         println!("{}", item);
///     }
///     Ok(())
/// }
/// ```
#[cfg(feature = "async-tokio")]
pub struct AsyncStreamingDecoder<R: AsyncRead + Unpin, C: Config = config::Configuration> {
    reader: R,
    codec_config: C,
    current_chunk: Option<ChunkData>,
    /// Resumable in-flight read state (header or payload fill cursor).
    pending: PendingRead,
    progress: StreamingProgress,
    finished: bool,
    end_seen: bool,
    poisoned: bool,
    max_chunk_size: usize,
}

#[cfg(feature = "async-tokio")]
struct ChunkData {
    data: alloc::vec::Vec<u8>,
    offset: usize,
    items_remaining: u32,
}

/// Resumable read state for the async decoder. Holds partially filled buffers
/// so a future dropped mid-read resumes from the same cursor.
#[cfg(feature = "async-tokio")]
enum PendingRead {
    /// Filling the fixed-size chunk header.
    Header {
        buf: [u8; ChunkHeader::SIZE],
        filled: usize,
    },
    /// Filling the chunk payload (allocated after the header was parsed).
    Payload {
        buf: alloc::vec::Vec<u8>,
        filled: usize,
        item_count: u32,
    },
}

#[cfg(feature = "async-tokio")]
impl PendingRead {
    #[inline]
    fn new_header() -> Self {
        PendingRead::Header {
            buf: [0u8; ChunkHeader::SIZE],
            filled: 0,
        }
    }
}

#[cfg(feature = "async-tokio")]
impl<R: AsyncRead + Unpin> AsyncStreamingDecoder<R> {
    /// Create a new async streaming decoder using the standard codec configuration.
    pub fn new(reader: R) -> Self {
        Self::new_with_config(reader, config::standard())
    }

    /// Create an async streaming decoder with an explicit streaming configuration.
    ///
    /// The `max_buffer_size` of the configuration bounds the largest chunk
    /// payload the decoder will accept before allocating (backpressure); the
    /// codec configuration stays the standard variable-width encoding.
    pub fn with_config(reader: R, config: StreamingConfig) -> Self {
        Self::new_with_configs(reader, config, config::standard())
    }
}

#[cfg(feature = "async-tokio")]
impl<R: AsyncRead + Unpin, C: Config> AsyncStreamingDecoder<R, C> {
    /// Create an async streaming decoder with a custom codec configuration.
    pub fn new_with_config(reader: R, codec_config: C) -> Self {
        Self {
            reader,
            codec_config,
            current_chunk: None,
            pending: PendingRead::new_header(),
            progress: StreamingProgress::default(),
            finished: false,
            end_seen: false,
            poisoned: false,
            max_chunk_size: MAX_CHUNK_SIZE,
        }
    }

    /// Create an async streaming decoder selecting both the streaming
    /// configuration (whose `max_buffer_size` bounds the largest accepted chunk)
    /// and the codec configuration.
    pub fn new_with_configs(reader: R, streaming_config: StreamingConfig, codec_config: C) -> Self {
        let mut decoder = Self::new_with_config(reader, codec_config);
        decoder.max_chunk_size = MAX_CHUNK_SIZE.min(streaming_config.max_buffer_size.max(1));
        decoder
    }

    /// Read the next item from the stream asynchronously.
    ///
    /// Returns `None` only when the stream was cleanly terminated by an End
    /// chunk; truncation or malformed input yields an error.
    pub async fn read_item<T: Decode>(&mut self) -> Result<Option<T>> {
        if self.poisoned {
            return Err(Error::InvalidData {
                message: "streaming decoder in failed state",
            });
        }
        if self.finished {
            return Ok(None);
        }

        // Load chunks until one contains items, skipping Metadata / empty chunks.
        loop {
            let has_items = self
                .current_chunk
                .as_ref()
                .map(|c| c.items_remaining != 0)
                .unwrap_or(false);
            if has_items {
                break;
            }
            if !self.load_next_chunk().await? {
                return Ok(None);
            }
        }

        // Decode item from current chunk
        let chunk = self.current_chunk.as_mut().ok_or(Error::InvalidData {
            message: "no chunk available",
        })?;

        let reader = SliceReader::new(&chunk.data[chunk.offset..]);
        let mut decoder = DecoderImpl::new(reader, self.codec_config);
        let item = T::decode(&mut decoder)?;

        let bytes_consumed = chunk.data[chunk.offset..].len() - decoder.reader().slice.len();
        chunk.offset += bytes_consumed;
        chunk.items_remaining -= 1;

        self.progress.items_processed += 1;
        self.progress.bytes_processed += bytes_consumed as u64;

        Ok(Some(item))
    }

    /// Read all remaining items into a vector.
    #[cfg(feature = "alloc")]
    pub async fn read_all<T: Decode>(&mut self) -> Result<alloc::vec::Vec<T>> {
        let mut items = alloc::vec::Vec::new();
        while let Some(item) = self.read_item().await? {
            items.push(item);
        }
        Ok(items)
    }

    /// Load the next chunk from the reader, poisoning the decoder on any error.
    async fn load_next_chunk(&mut self) -> Result<bool> {
        match self.load_next_chunk_inner().await {
            Ok(true) => Ok(true),
            Ok(false) => {
                self.finished = true;
                self.end_seen = true;
                Ok(false)
            }
            Err(e) => {
                self.poisoned = true;
                self.finished = true;
                Err(e)
            }
        }
    }

    /// Resumable inner load. Never mutates the poison/finish flags itself; the
    /// caller ([`load_next_chunk`](Self::load_next_chunk)) does that so a future
    /// dropped mid-await leaves the fill cursor intact and resumes cleanly.
    async fn load_next_chunk_inner(&mut self) -> Result<bool> {
        // Phase 1: fill and parse the header (resuming from a partial fill).
        loop {
            let Self {
                reader,
                pending,
                max_chunk_size,
                codec_config,
                ..
            } = &mut *self;

            match pending {
                PendingRead::Header { buf, filled } => {
                    if *filled < ChunkHeader::SIZE {
                        let n = reader
                            .read(&mut buf[*filled..])
                            .await
                            .map_err(|e| Error::Io {
                                kind: e.kind(),
                                message: e.to_string(),
                            })?;
                        if n == 0 {
                            // EOF before a complete header and before an End
                            // chunk: the stream is truncated.
                            return Err(Error::UnexpectedEnd {
                                additional: ChunkHeader::SIZE - *filled,
                            });
                        }
                        *filled += n;
                        continue;
                    }

                    // Header complete — copy it out and parse.
                    let header_bytes = *buf;
                    let header = ChunkHeader::from_bytes(&header_bytes)?;

                    if header.is_end() {
                        *pending = PendingRead::new_header();
                        return Ok(false);
                    }

                    let mut bound = *max_chunk_size;
                    if let Some(limit) = codec_config.limit() {
                        bound = bound.min(limit);
                    }
                    if header.payload_len as usize > bound {
                        return Err(Error::LimitExceeded {
                            limit: bound as u64,
                            found: header.payload_len as u64,
                        });
                    }

                    *pending = PendingRead::Payload {
                        buf: alloc::vec![0u8; header.payload_len as usize],
                        filled: 0,
                        item_count: header.item_count,
                    };
                    break;
                }
                PendingRead::Payload { .. } => break,
            }
        }

        // Phase 2: fill the payload (resuming from a partial fill).
        loop {
            let Self {
                reader, pending, ..
            } = &mut *self;
            match pending {
                PendingRead::Payload { buf, filled, .. } => {
                    if *filled >= buf.len() {
                        break;
                    }
                    let n = reader
                        .read(&mut buf[*filled..])
                        .await
                        .map_err(|e| Error::Io {
                            kind: e.kind(),
                            message: e.to_string(),
                        })?;
                    if n == 0 {
                        return Err(Error::UnexpectedEnd {
                            additional: buf.len() - *filled,
                        });
                    }
                    *filled += n;
                }
                PendingRead::Header { .. } => {
                    return Err(Error::InvalidData {
                        message: "streaming decoder in inconsistent state",
                    });
                }
            }
        }

        // Payload complete — hand it off and reset for the next header.
        let taken = core::mem::replace(&mut self.pending, PendingRead::new_header());
        match taken {
            PendingRead::Payload {
                buf, item_count, ..
            } => {
                self.current_chunk = Some(ChunkData {
                    data: buf,
                    offset: 0,
                    items_remaining: item_count,
                });
                self.progress.chunks_processed += 1;
                Ok(true)
            }
            PendingRead::Header { .. } => Err(Error::InvalidData {
                message: "streaming decoder in inconsistent state",
            }),
        }
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

/// A cancellation token for async streaming operations.
///
/// Allows cancelling long-running async streaming operations.
#[derive(Debug, Clone, Default)]
pub struct CancellationToken {
    cancelled: std::sync::Arc<std::sync::atomic::AtomicBool>,
}

impl CancellationToken {
    /// Create a new cancellation token.
    pub fn new() -> Self {
        Self {
            cancelled: std::sync::Arc::new(std::sync::atomic::AtomicBool::new(false)),
        }
    }

    /// Cancel the operation.
    pub fn cancel(&self) {
        self.cancelled
            .store(true, std::sync::atomic::Ordering::SeqCst);
    }

    /// Check if the operation has been cancelled.
    pub fn is_cancelled(&self) -> bool {
        self.cancelled.load(std::sync::atomic::Ordering::SeqCst)
    }

    /// Create a child token that shares the same cancelled state.
    pub fn child(&self) -> Self {
        Self {
            cancelled: self.cancelled.clone(),
        }
    }
}

/// An async streaming encoder with cancellation support.
#[cfg(feature = "async-tokio")]
pub struct CancellableAsyncEncoder<W: AsyncWrite + Unpin, C: Config = config::Configuration> {
    inner: AsyncStreamingEncoder<W, C>,
    token: CancellationToken,
}

#[cfg(feature = "async-tokio")]
impl<W: AsyncWrite + Unpin> CancellableAsyncEncoder<W> {
    /// Create a new cancellable async encoder using the standard codec configuration.
    pub fn new(writer: W, token: CancellationToken) -> Self {
        Self {
            inner: AsyncStreamingEncoder::new(writer),
            token,
        }
    }
}

#[cfg(feature = "async-tokio")]
impl<W: AsyncWrite + Unpin, C: Config> CancellableAsyncEncoder<W, C> {
    /// Create a new cancellable async encoder with a custom codec configuration.
    pub fn new_with_config(writer: W, token: CancellationToken, codec_config: C) -> Self {
        Self {
            inner: AsyncStreamingEncoder::new_with_config(writer, codec_config),
            token,
        }
    }

    /// Write an item, checking for cancellation.
    pub async fn write_item<T: Encode>(&mut self, item: &T) -> Result<()> {
        if self.token.is_cancelled() {
            return Err(Error::Custom {
                message: "operation cancelled",
            });
        }
        self.inner.write_item(item).await
    }

    /// Finish the stream.
    pub async fn finish(self) -> Result<W> {
        if self.token.is_cancelled() {
            return Err(Error::Custom {
                message: "operation cancelled",
            });
        }
        self.inner.finish().await
    }

    /// Get current progress.
    pub fn progress(&self) -> &StreamingProgress {
        self.inner.progress()
    }
}

/// An async streaming decoder with cancellation support.
#[cfg(feature = "async-tokio")]
pub struct CancellableAsyncDecoder<R: AsyncRead + Unpin, C: Config = config::Configuration> {
    inner: AsyncStreamingDecoder<R, C>,
    token: CancellationToken,
}

#[cfg(feature = "async-tokio")]
impl<R: AsyncRead + Unpin> CancellableAsyncDecoder<R> {
    /// Create a new cancellable async decoder using the standard codec configuration.
    pub fn new(reader: R, token: CancellationToken) -> Self {
        Self {
            inner: AsyncStreamingDecoder::new(reader),
            token,
        }
    }
}

#[cfg(feature = "async-tokio")]
impl<R: AsyncRead + Unpin, C: Config> CancellableAsyncDecoder<R, C> {
    /// Create a new cancellable async decoder with a custom codec configuration.
    pub fn new_with_config(reader: R, token: CancellationToken, codec_config: C) -> Self {
        Self {
            inner: AsyncStreamingDecoder::new_with_config(reader, codec_config),
            token,
        }
    }

    /// Read the next item, checking for cancellation.
    pub async fn read_item<T: Decode>(&mut self) -> Result<Option<T>> {
        if self.token.is_cancelled() {
            return Err(Error::Custom {
                message: "operation cancelled",
            });
        }
        self.inner.read_item().await
    }

    /// Read all remaining items.
    #[cfg(feature = "alloc")]
    pub async fn read_all<T: Decode>(&mut self) -> Result<alloc::vec::Vec<T>> {
        let mut items = alloc::vec::Vec::new();
        while let Some(item) = self.read_item().await? {
            items.push(item);
        }
        Ok(items)
    }

    /// Get current progress.
    pub fn progress(&self) -> &StreamingProgress {
        self.inner.progress()
    }

    /// Check if finished.
    pub fn is_finished(&self) -> bool {
        self.inner.is_finished()
    }
}

#[cfg(all(test, feature = "async-tokio"))]
mod tests {
    use super::*;
    use std::io::Cursor;

    #[tokio::test]
    async fn test_async_roundtrip() {
        // Encode
        let mut buffer = alloc::vec::Vec::new();
        {
            let cursor = Cursor::new(&mut buffer);
            let mut encoder = AsyncStreamingEncoder::new(cursor);

            for i in 0..50u32 {
                encoder.write_item(&i).await.expect("write failed");
            }

            encoder.finish().await.expect("finish failed");
        }

        // Decode
        let cursor = Cursor::new(buffer);
        let mut decoder = AsyncStreamingDecoder::new(cursor);
        let decoded: alloc::vec::Vec<u32> = decoder.read_all().await.expect("read failed");

        let expected: alloc::vec::Vec<u32> = (0..50).collect();
        assert_eq!(expected, decoded);
        assert!(decoder.is_finished());
    }

    #[tokio::test]
    async fn test_async_item_by_item() {
        let mut buffer = alloc::vec::Vec::new();
        {
            let cursor = Cursor::new(&mut buffer);
            let mut encoder = AsyncStreamingEncoder::new(cursor);
            encoder.write_item(&1u32).await.expect("write failed");
            encoder.write_item(&2u32).await.expect("write failed");
            encoder.write_item(&3u32).await.expect("write failed");
            encoder.finish().await.expect("finish failed");
        }

        let cursor = Cursor::new(buffer);
        let mut decoder = AsyncStreamingDecoder::new(cursor);

        assert_eq!(
            decoder.read_item::<u32>().await.expect("read failed"),
            Some(1)
        );
        assert_eq!(
            decoder.read_item::<u32>().await.expect("read failed"),
            Some(2)
        );
        assert_eq!(
            decoder.read_item::<u32>().await.expect("read failed"),
            Some(3)
        );
        assert_eq!(decoder.read_item::<u32>().await.expect("read failed"), None);
    }

    #[tokio::test]
    async fn test_cancellation() {
        let token = CancellationToken::new();

        let mut buffer = alloc::vec::Vec::new();
        let cursor = Cursor::new(&mut buffer);
        let mut encoder = CancellableAsyncEncoder::new(cursor, token.child());

        // Write some items
        encoder.write_item(&1u32).await.expect("write failed");
        encoder.write_item(&2u32).await.expect("write failed");

        // Cancel
        token.cancel();

        // Next write should fail
        let result = encoder.write_item(&3u32).await;
        assert!(result.is_err());
    }

    #[test]
    fn test_cancellation_token() {
        let token = CancellationToken::new();
        assert!(!token.is_cancelled());

        let child = token.child();
        token.cancel();

        assert!(token.is_cancelled());
        assert!(child.is_cancelled());
    }

    #[tokio::test]
    async fn test_async_progress_tracking() {
        let mut buffer = alloc::vec::Vec::new();
        {
            let cursor = Cursor::new(&mut buffer);
            let mut encoder = AsyncStreamingEncoder::new(cursor);
            encoder.set_estimated_total(10);

            for i in 0..10u32 {
                encoder.write_item(&i).await.expect("write failed");
            }

            encoder.finish().await.expect("finish failed");
        }

        let cursor = Cursor::new(buffer);
        let mut decoder = AsyncStreamingDecoder::new(cursor);
        let _: alloc::vec::Vec<u32> = decoder.read_all().await.expect("read failed");

        assert_eq!(decoder.progress().items_processed, 10);
        assert!(decoder.progress().chunks_processed >= 1);
    }

    #[tokio::test]
    async fn test_async_large_data() {
        // Use small chunk size to force multiple chunks
        let config = StreamingConfig::new().with_chunk_size(1024);

        let mut buffer = alloc::vec::Vec::new();
        {
            let cursor = Cursor::new(&mut buffer);
            let mut encoder = AsyncStreamingEncoder::with_config(cursor, config);

            for i in 0..1000u32 {
                encoder.write_item(&i).await.expect("write failed");
            }

            encoder.finish().await.expect("finish failed");
        }

        let cursor = Cursor::new(buffer);
        let mut decoder = AsyncStreamingDecoder::new(cursor);
        let decoded: alloc::vec::Vec<u32> = decoder.read_all().await.expect("read failed");

        let expected: alloc::vec::Vec<u32> = (0..1000).collect();
        assert_eq!(expected, decoded);

        // Should have processed multiple chunks
        assert!(decoder.progress().chunks_processed > 1);
    }
}
