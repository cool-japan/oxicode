//! Streaming serialization support for oxicode.
//!
//! This module provides incremental encoding and decoding for large data sets
//! that don't fit in memory all at once.
//!
//! ## Features
//!
//! - **Chunked Encoding**: Encode large collections in chunks
//! - **Incremental Decoding**: Decode items one at a time
//! - **Backpressure**: Control memory usage during streaming
//! - **Progress Callbacks**: Monitor long operations
//!
//! ## Example
//!
//! ```rust,ignore
//! use oxicode::streaming::{StreamingEncoder, StreamingDecoder};
//!
//! // Stream encode a large dataset
//! let mut encoder = StreamingEncoder::new(writer);
//! for item in large_dataset {
//!     encoder.write_item(&item)?;
//! }
//! encoder.finish()?;
//!
//! // Stream decode
//! let mut decoder = StreamingDecoder::new(reader);
//! while let Some(item) = decoder.read_item::<MyType>()? {
//!     process(item);
//! }
//! ```

#[cfg(feature = "alloc")]
use alloc::boxed::Box;

mod chunk;
mod decoder;
mod encoder;

#[cfg(feature = "async-tokio")]
mod async_io;

pub use chunk::{ChunkHeader, CHUNK_MAGIC};
pub use decoder::BufferStreamingDecoder;
pub use encoder::BufferStreamingEncoder;

#[cfg(feature = "std")]
pub use decoder::StreamingDecoder;
#[cfg(feature = "std")]
pub use encoder::StreamingEncoder;

#[cfg(feature = "async-tokio")]
pub use async_io::{
    AsyncStreamingDecoder, AsyncStreamingEncoder, CancellableAsyncDecoder, CancellableAsyncEncoder,
    CancellationToken,
};

/// Short alias for [`AsyncStreamingEncoder`] — available when the `async-tokio` feature is enabled.
#[cfg(feature = "async-tokio")]
pub type AsyncEncoder<W> = AsyncStreamingEncoder<W>;

/// Short alias for [`AsyncStreamingDecoder`] — available when the `async-tokio` feature is enabled.
#[cfg(feature = "async-tokio")]
pub type AsyncDecoder<R> = AsyncStreamingDecoder<R>;

/// Default chunk size for streaming operations (64KB).
pub const DEFAULT_CHUNK_SIZE: usize = 64 * 1024;

/// Maximum chunk size allowed (16MB).
pub const MAX_CHUNK_SIZE: usize = 16 * 1024 * 1024;

/// Streaming configuration.
#[derive(Debug, Clone)]
pub struct StreamingConfig {
    /// Size of each chunk in bytes.
    pub chunk_size: usize,

    /// Maximum memory to use for buffering.
    pub max_buffer_size: usize,

    /// Whether to flush after each item (slower but safer).
    pub flush_per_item: bool,
}

impl Default for StreamingConfig {
    fn default() -> Self {
        Self {
            chunk_size: DEFAULT_CHUNK_SIZE,
            max_buffer_size: 4 * 1024 * 1024, // 4MB
            flush_per_item: false,
        }
    }
}

impl StreamingConfig {
    /// Create a new configuration with default values.
    pub fn new() -> Self {
        Self::default()
    }

    /// Set the chunk size.
    #[inline]
    pub fn with_chunk_size(mut self, size: usize) -> Self {
        self.chunk_size = size.clamp(1024, MAX_CHUNK_SIZE);
        self
    }

    /// Set the maximum buffer size.
    #[inline]
    pub fn with_max_buffer(mut self, size: usize) -> Self {
        self.max_buffer_size = size;
        self
    }

    /// Enable flushing after each item.
    #[inline]
    pub fn with_flush_per_item(mut self, flush: bool) -> Self {
        self.flush_per_item = flush;
        self
    }
}

/// Progress information for streaming operations.
#[derive(Debug, Clone, Default)]
pub struct StreamingProgress {
    /// Number of items processed.
    pub items_processed: u64,

    /// Number of bytes processed.
    pub bytes_processed: u64,

    /// Number of chunks processed.
    pub chunks_processed: u64,

    /// Estimated total items (if known).
    pub estimated_total: Option<u64>,
}

impl StreamingProgress {
    /// Calculate progress percentage (0.0 - 100.0).
    pub fn percentage(&self) -> Option<f64> {
        self.estimated_total.map(|total| {
            if total == 0 {
                100.0
            } else {
                100.0 * (self.items_processed as f64 / total as f64)
            }
        })
    }
}

/// Callback type for progress updates.
#[cfg(feature = "alloc")]
pub type ProgressCallback = Box<dyn FnMut(&StreamingProgress) + Send>;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_config_defaults() {
        let config = StreamingConfig::default();
        assert_eq!(config.chunk_size, DEFAULT_CHUNK_SIZE);
        assert!(!config.flush_per_item);
    }

    #[test]
    fn test_config_builder() {
        let config = StreamingConfig::new()
            .with_chunk_size(1024 * 1024)
            .with_max_buffer(8 * 1024 * 1024)
            .with_flush_per_item(true);

        assert_eq!(config.chunk_size, 1024 * 1024);
        assert_eq!(config.max_buffer_size, 8 * 1024 * 1024);
        assert!(config.flush_per_item);
    }

    #[test]
    fn test_chunk_size_clamping() {
        // Too small
        let config = StreamingConfig::new().with_chunk_size(100);
        assert_eq!(config.chunk_size, 1024);

        // Too large
        let config = StreamingConfig::new().with_chunk_size(100 * 1024 * 1024);
        assert_eq!(config.chunk_size, MAX_CHUNK_SIZE);
    }

    #[test]
    fn test_progress() {
        let progress = StreamingProgress {
            items_processed: 50,
            bytes_processed: 5000,
            chunks_processed: 5,
            estimated_total: Some(100),
        };

        assert_eq!(progress.percentage(), Some(50.0));
    }
}
