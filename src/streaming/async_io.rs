//! Async streaming encoder and decoder implementations.
//!
//! Provides async versions of `StreamingEncoder` and `StreamingDecoder`
//! for use with tokio or other async runtimes.

use super::chunk::ChunkHeader;
use super::{StreamingConfig, StreamingProgress};
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
pub struct AsyncStreamingEncoder<W: AsyncWrite + Unpin> {
    writer: W,
    config: StreamingConfig,
    buffer: alloc::vec::Vec<u8>,
    items_in_buffer: u32,
    progress: StreamingProgress,
}

#[cfg(feature = "async-tokio")]
impl<W: AsyncWrite + Unpin> AsyncStreamingEncoder<W> {
    /// Create a new async streaming encoder.
    pub fn new(writer: W) -> Self {
        Self::with_config(writer, StreamingConfig::default())
    }

    /// Create an async streaming encoder with custom configuration.
    pub fn with_config(writer: W, config: StreamingConfig) -> Self {
        Self {
            writer,
            config,
            buffer: alloc::vec::Vec::new(),
            items_in_buffer: 0,
            progress: StreamingProgress::default(),
        }
    }

    /// Set the estimated total number of items (for progress reporting).
    pub fn set_estimated_total(&mut self, total: u64) {
        self.progress.estimated_total = Some(total);
    }

    /// Write a single item to the stream asynchronously.
    pub async fn write_item<T: Encode>(&mut self, item: &T) -> Result<()> {
        // Encode item to temporary buffer
        let item_writer = VecWriter::new();
        let mut encoder = EncoderImpl::new(item_writer, config::standard());
        item.encode(&mut encoder)?;
        let item_bytes = encoder.into_writer().into_vec();

        // Check if adding this item would exceed chunk size
        if !self.buffer.is_empty() && self.buffer.len() + item_bytes.len() > self.config.chunk_size
        {
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
        if self.buffer.is_empty() {
            return Ok(());
        }

        // Write chunk header
        let header = ChunkHeader::data(self.buffer.len() as u32, self.items_in_buffer);
        self.writer
            .write_all(&header.to_bytes())
            .await
            .map_err(|e| Error::Io {
                kind: e.kind(),
                message: e.to_string(),
            })?;

        // Write payload
        self.writer
            .write_all(&self.buffer)
            .await
            .map_err(|e| Error::Io {
                kind: e.kind(),
                message: e.to_string(),
            })?;

        // Update progress
        self.progress.items_processed += self.items_in_buffer as u64;
        self.progress.bytes_processed += self.buffer.len() as u64;
        self.progress.chunks_processed += 1;

        // Clear buffer
        self.buffer.clear();
        self.items_in_buffer = 0;

        Ok(())
    }

    /// Finish the stream, writing any remaining data and the end marker.
    pub async fn finish(mut self) -> Result<W> {
        // Flush remaining buffer
        self.flush_chunk().await?;

        // Write end chunk
        let end_header = ChunkHeader::end();
        self.writer
            .write_all(&end_header.to_bytes())
            .await
            .map_err(|e| Error::Io {
                kind: e.kind(),
                message: e.to_string(),
            })?;

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
pub struct AsyncStreamingDecoder<R: AsyncRead + Unpin> {
    reader: R,
    current_chunk: Option<ChunkData>,
    progress: StreamingProgress,
    finished: bool,
}

#[cfg(feature = "async-tokio")]
struct ChunkData {
    data: alloc::vec::Vec<u8>,
    offset: usize,
    items_remaining: u32,
}

#[cfg(feature = "async-tokio")]
impl<R: AsyncRead + Unpin> AsyncStreamingDecoder<R> {
    /// Create a new async streaming decoder.
    pub fn new(reader: R) -> Self {
        Self {
            reader,
            current_chunk: None,
            progress: StreamingProgress::default(),
            finished: false,
        }
    }

    /// Create an async streaming decoder with explicit configuration.
    ///
    /// The configuration is accepted for API symmetry with [`AsyncStreamingEncoder::with_config`]
    /// but the decoder determines framing from the on-wire chunk format, so most config fields
    /// (e.g. `chunk_size`) are advisory only.
    pub fn with_config(reader: R, _config: StreamingConfig) -> Self {
        Self::new(reader)
    }

    /// Read the next item from the stream asynchronously.
    ///
    /// Returns `None` when the stream is exhausted.
    pub async fn read_item<T: Decode>(&mut self) -> Result<Option<T>> {
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
        if needs_chunk && !self.load_next_chunk().await? {
            return Ok(None);
        }

        // Decode item from current chunk
        let chunk = self.current_chunk.as_mut().ok_or(Error::InvalidData {
            message: "no chunk available",
        })?;

        if chunk.items_remaining == 0 {
            return Ok(None);
        }

        // Create reader from remaining chunk data
        let reader = SliceReader::new(&chunk.data[chunk.offset..]);
        let mut decoder = DecoderImpl::new(reader, config::standard());
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
    pub async fn read_all<T: Decode>(&mut self) -> Result<alloc::vec::Vec<T>> {
        let mut items = alloc::vec::Vec::new();
        while let Some(item) = self.read_item().await? {
            items.push(item);
        }
        Ok(items)
    }

    /// Load the next chunk from the reader.
    async fn load_next_chunk(&mut self) -> Result<bool> {
        // Read chunk header
        let mut header_bytes = [0u8; ChunkHeader::SIZE];
        match self.reader.read_exact(&mut header_bytes).await {
            Ok(_) => {}
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
        self.reader
            .read_exact(&mut data)
            .await
            .map_err(|e| Error::Io {
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
pub struct CancellableAsyncEncoder<W: AsyncWrite + Unpin> {
    inner: AsyncStreamingEncoder<W>,
    token: CancellationToken,
}

#[cfg(feature = "async-tokio")]
impl<W: AsyncWrite + Unpin> CancellableAsyncEncoder<W> {
    /// Create a new cancellable async encoder.
    pub fn new(writer: W, token: CancellationToken) -> Self {
        Self {
            inner: AsyncStreamingEncoder::new(writer),
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
pub struct CancellableAsyncDecoder<R: AsyncRead + Unpin> {
    inner: AsyncStreamingDecoder<R>,
    token: CancellationToken,
}

#[cfg(feature = "async-tokio")]
impl<R: AsyncRead + Unpin> CancellableAsyncDecoder<R> {
    /// Create a new cancellable async decoder.
    pub fn new(reader: R, token: CancellationToken) -> Self {
        Self {
            inner: AsyncStreamingDecoder::new(reader),
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
