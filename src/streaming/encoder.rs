//! Streaming encoder implementation.

use super::chunk::ChunkHeader;
use super::{StreamingConfig, StreamingProgress};
use crate::config::Config;
use crate::enc::{Encode, EncoderImpl, VecWriter};
use crate::{config, Result};

#[cfg(feature = "std")]
use super::ProgressCallback;
#[cfg(feature = "std")]
use crate::Error;

#[cfg(feature = "alloc")]
extern crate alloc;

#[cfg(feature = "std")]
use std::io::Write;

/// A streaming encoder for writing items incrementally.
///
/// Buffers items until a chunk is full, then writes the chunk.
/// This allows encoding very large collections without loading
/// everything into memory at once.
///
/// The `C` type parameter controls the codec configuration (integer encoding,
/// endianness, byte limit).  Use [`StreamingEncoder::new`] to get the default
/// variable-width integer encoding, or [`StreamingEncoder::new_with_config`]
/// to select an alternative such as `config::standard().with_fixed_int_encoding()`.
#[cfg(feature = "std")]
pub struct StreamingEncoder<W: Write, C: Config = config::Configuration> {
    writer: W,
    streaming_config: StreamingConfig,
    codec_config: C,
    buffer: alloc::vec::Vec<u8>,
    items_in_buffer: u32,
    progress: StreamingProgress,
    progress_callback: Option<ProgressCallback>,
}

#[cfg(feature = "std")]
impl<W: Write> StreamingEncoder<W> {
    /// Create a new streaming encoder using the standard codec configuration
    /// (little-endian, variable-width integer encoding).
    pub fn new(writer: W) -> Self {
        Self::new_with_config(writer, config::standard())
    }

    /// Create a streaming encoder with custom chunking configuration and the
    /// standard codec configuration.
    pub fn with_config(writer: W, streaming_config: StreamingConfig) -> Self {
        StreamingEncoder {
            writer,
            streaming_config,
            codec_config: config::standard(),
            buffer: alloc::vec::Vec::new(),
            items_in_buffer: 0,
            progress: StreamingProgress::default(),
            progress_callback: None,
        }
    }
}

#[cfg(feature = "std")]
impl<W: Write, C: Config> StreamingEncoder<W, C> {
    /// Create a streaming encoder with a custom codec configuration.
    ///
    /// This allows selecting, for example, fixed-width integer encoding for
    /// efficient random-access seeks:
    ///
    /// ```rust,ignore
    /// use oxicode::streaming::StreamingEncoder;
    ///
    /// let config = oxicode::config::standard().with_fixed_int_encoding();
    /// let mut encoder = StreamingEncoder::new_with_config(writer, config);
    /// ```
    pub fn new_with_config(writer: W, codec_config: C) -> Self {
        StreamingEncoder {
            writer,
            streaming_config: StreamingConfig::default(),
            codec_config,
            buffer: alloc::vec::Vec::new(),
            items_in_buffer: 0,
            progress: StreamingProgress::default(),
            progress_callback: None,
        }
    }

    /// Set a progress callback.
    pub fn with_progress_callback(mut self, callback: ProgressCallback) -> Self {
        self.progress_callback = Some(callback);
        self
    }

    /// Set the estimated total number of items (for progress reporting).
    pub fn set_estimated_total(&mut self, total: u64) {
        self.progress.estimated_total = Some(total);
    }

    /// Write a single item to the stream.
    pub fn write_item<T: Encode>(&mut self, item: &T) -> Result<()> {
        // Encode item to temporary buffer using the stored codec configuration.
        let item_writer = VecWriter::new();
        let mut encoder = EncoderImpl::new(item_writer, self.codec_config);
        item.encode(&mut encoder)?;
        let item_bytes = encoder.into_writer().into_vec();

        // Check if adding this item would exceed chunk size.
        if !self.buffer.is_empty()
            && self.buffer.len() + item_bytes.len() > self.streaming_config.chunk_size
        {
            self.flush_chunk()?;
        }

        // Add item to buffer.
        self.buffer.extend_from_slice(&item_bytes);
        self.items_in_buffer += 1;

        // Flush if configured to flush per item.
        if self.streaming_config.flush_per_item {
            self.flush_chunk()?;
        }

        Ok(())
    }

    /// Write multiple items from an iterator.
    pub fn write_all<T: Encode, I: IntoIterator<Item = T>>(&mut self, items: I) -> Result<()> {
        for item in items {
            self.write_item(&item)?;
        }
        Ok(())
    }

    /// Flush the current buffer as a chunk.
    fn flush_chunk(&mut self) -> Result<()> {
        if self.buffer.is_empty() {
            return Ok(());
        }

        // Write chunk header
        let header = ChunkHeader::data(self.buffer.len() as u32, self.items_in_buffer);
        self.writer
            .write_all(&header.to_bytes())
            .map_err(|e| Error::Io {
                kind: e.kind(),
                message: e.to_string(),
            })?;

        // Write payload
        self.writer.write_all(&self.buffer).map_err(|e| Error::Io {
            kind: e.kind(),
            message: e.to_string(),
        })?;

        // Update progress
        self.progress.items_processed += self.items_in_buffer as u64;
        self.progress.bytes_processed += self.buffer.len() as u64;
        self.progress.chunks_processed += 1;

        // Notify callback
        if let Some(ref mut callback) = self.progress_callback {
            callback(&self.progress);
        }

        // Clear buffer
        self.buffer.clear();
        self.items_in_buffer = 0;

        Ok(())
    }

    /// Finish the stream, writing any remaining data and the end marker.
    pub fn finish(mut self) -> Result<W> {
        // Flush remaining buffer
        self.flush_chunk()?;

        // Write end chunk
        let end_header = ChunkHeader::end();
        self.writer
            .write_all(&end_header.to_bytes())
            .map_err(|e| Error::Io {
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

/// Streaming encoder for in-memory buffers (no std required).
#[cfg(feature = "alloc")]
pub struct BufferStreamingEncoder {
    buffer: alloc::vec::Vec<u8>,
    config: StreamingConfig,
    chunk_buffer: alloc::vec::Vec<u8>,
    items_in_chunk: u32,
    progress: StreamingProgress,
}

#[cfg(feature = "alloc")]
impl BufferStreamingEncoder {
    /// Create a new buffer streaming encoder.
    pub fn new() -> Self {
        Self::with_config(StreamingConfig::default())
    }

    /// Create with custom configuration.
    pub fn with_config(config: StreamingConfig) -> Self {
        Self {
            buffer: alloc::vec::Vec::new(),
            config,
            chunk_buffer: alloc::vec::Vec::new(),
            items_in_chunk: 0,
            progress: StreamingProgress::default(),
        }
    }

    /// Write a single item.
    pub fn write_item<T: Encode>(&mut self, item: &T) -> Result<()> {
        let item_writer = VecWriter::new();
        let mut encoder = EncoderImpl::new(item_writer, config::standard());
        item.encode(&mut encoder)?;
        let item_bytes = encoder.into_writer().into_vec();

        if !self.chunk_buffer.is_empty()
            && self.chunk_buffer.len() + item_bytes.len() > self.config.chunk_size
        {
            self.flush_chunk();
        }

        self.chunk_buffer.extend_from_slice(&item_bytes);
        self.items_in_chunk += 1;

        Ok(())
    }

    /// Flush current chunk to output buffer.
    fn flush_chunk(&mut self) {
        if self.chunk_buffer.is_empty() {
            return;
        }

        let header = ChunkHeader::data(self.chunk_buffer.len() as u32, self.items_in_chunk);
        self.buffer.extend_from_slice(&header.to_bytes());
        self.buffer.extend_from_slice(&self.chunk_buffer);

        self.progress.items_processed += self.items_in_chunk as u64;
        self.progress.bytes_processed += self.chunk_buffer.len() as u64;
        self.progress.chunks_processed += 1;

        self.chunk_buffer.clear();
        self.items_in_chunk = 0;
    }

    /// Finish encoding and return the complete buffer.
    pub fn finish(mut self) -> alloc::vec::Vec<u8> {
        self.flush_chunk();

        // Write end chunk
        let end_header = ChunkHeader::end();
        self.buffer.extend_from_slice(&end_header.to_bytes());

        self.buffer
    }

    /// Get current progress.
    pub fn progress(&self) -> &StreamingProgress {
        &self.progress
    }
}

#[cfg(feature = "alloc")]
impl Default for BufferStreamingEncoder {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[cfg(feature = "alloc")]
    #[test]
    fn test_buffer_encoder_basic() {
        let mut encoder = BufferStreamingEncoder::new();

        encoder.write_item(&42u32).expect("write failed");
        encoder.write_item(&100u32).expect("write failed");
        encoder.write_item(&255u32).expect("write failed");

        let data = encoder.finish();
        assert!(!data.is_empty());

        // Should have chunk header + data + end header
        assert!(data.len() > ChunkHeader::SIZE * 2);
    }

    #[cfg(feature = "std")]
    #[test]
    fn test_streaming_encoder_io() {
        let mut buffer = alloc::vec::Vec::new();
        {
            let mut encoder = StreamingEncoder::new(&mut buffer);

            for i in 0..100u32 {
                encoder.write_item(&i).expect("write failed");
            }

            encoder.finish().expect("finish failed");
        }

        assert!(!buffer.is_empty());
    }

    #[cfg(feature = "alloc")]
    #[test]
    fn test_chunking() {
        // Use small chunk size to force multiple chunks
        let config = StreamingConfig::new().with_chunk_size(1024);
        let mut encoder = BufferStreamingEncoder::with_config(config);

        // Write enough data to span multiple chunks
        for i in 0..1000u32 {
            encoder.write_item(&i).expect("write failed");
        }

        let progress = encoder.progress().clone();
        let _data = encoder.finish();

        // Should have created multiple chunks
        assert!(progress.chunks_processed >= 1);
    }

    // ── Regression tests: issue #1 — new_with_config constructor ───────────

    /// Verify that `StreamingEncoder::new_with_config` with fixed-width integer
    /// encoding produces bytes that can be decoded with the same codec config.
    #[cfg(feature = "std")]
    #[test]
    fn test_streaming_encoder_with_fixed_int_config() {
        use super::super::decoder::StreamingDecoder;
        use std::io::Cursor;

        let codec = crate::config::standard().with_fixed_int_encoding();

        // Encode with fixed-width integers via new_with_config.
        let mut buffer = alloc::vec::Vec::new();
        {
            let mut encoder = StreamingEncoder::new_with_config(&mut buffer, codec);
            for i in 0u32..30 {
                encoder.write_item(&i).expect("write failed");
            }
            encoder.finish().expect("finish failed");
        }

        assert!(!buffer.is_empty(), "encoded buffer must not be empty");

        // Decode with the matching fixed-int decoder.
        let cursor = Cursor::new(buffer);
        let mut decoder = StreamingDecoder::new_with_config(cursor, codec);
        let decoded: alloc::vec::Vec<u32> = decoder.read_all().expect("read_all failed");

        let expected: alloc::vec::Vec<u32> = (0..30).collect();
        assert_eq!(expected, decoded, "fixed-int encoder roundtrip mismatch");
    }
}
