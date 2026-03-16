//! Built-in compression support for oxicode.
//!
//! This module provides transparent compression/decompression for serialized data.
//! Compression can be applied globally or per-message.
//!
//! ## Supported Codecs
//!
//! - **LZ4** (`compression-lz4` feature): Extremely fast compression/decompression.
//!   Good for real-time applications where speed matters more than ratio.
//!   Decompression speed: ~4 GB/s.
//!
//! - **Zstd** (`compression-zstd` feature): Better compression ratio with still
//!   fast performance. Good for storage and network transmission.
//!
//! ## Example
//!
//! ```rust,ignore
//! use oxicode::compression::{Compression, compress, decompress};
//!
//! let data = b"Hello, World! This is some data to compress.";
//!
//! // Compress with LZ4 (default)
//! let compressed = compress(data, Compression::Lz4)?;
//!
//! // Decompress
//! let decompressed = decompress(&compressed)?;
//! assert_eq!(data.as_slice(), decompressed.as_slice());
//! ```
//!
//! ## Automatic Detection
//!
//! Compressed data includes a magic header that allows automatic detection
//! of the compression format. The `decompress_auto` function can detect
//! and decompress any supported format.

#[cfg(feature = "alloc")]
extern crate alloc;

use crate::{Error, Result};

#[cfg(feature = "compression-lz4")]
mod lz4;

#[cfg(feature = "compression-zstd")]
mod zstd_impl;

#[cfg(feature = "compression-zstd-pure")]
mod ruzstd_impl;

/// Magic bytes for identifying compressed data.
/// Format: [0x4F, 0x58, 0x43, version, codec_id]
/// OXC = "OXiCode Compressed"
const MAGIC: [u8; 3] = [0x4F, 0x58, 0x43];
const VERSION: u8 = 1;
const HEADER_SIZE: usize = 5; // MAGIC (3) + VERSION (1) + CODEC (1)

/// Compression algorithm selection.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum Compression {
    /// No compression (passthrough)
    #[default]
    None,

    /// LZ4 compression - fastest, good for real-time
    #[cfg(feature = "compression-lz4")]
    Lz4,

    /// Zstd compression - better ratio, still fast
    #[cfg(any(feature = "compression-zstd", feature = "compression-zstd-pure"))]
    Zstd,

    /// Zstd with specified compression level (1-22, default 3)
    #[cfg(any(feature = "compression-zstd", feature = "compression-zstd-pure"))]
    ZstdLevel(u8),
}

impl Compression {
    /// Returns the codec ID for this compression type.
    const fn codec_id(self) -> u8 {
        match self {
            Compression::None => 0,
            #[cfg(feature = "compression-lz4")]
            Compression::Lz4 => 1,
            #[cfg(any(feature = "compression-zstd", feature = "compression-zstd-pure"))]
            Compression::Zstd => 2,
            #[cfg(any(feature = "compression-zstd", feature = "compression-zstd-pure"))]
            Compression::ZstdLevel(_) => 2,
        }
    }

    /// Returns the name of this compression type.
    pub const fn name(self) -> &'static str {
        match self {
            Compression::None => "none",
            #[cfg(feature = "compression-lz4")]
            Compression::Lz4 => "lz4",
            #[cfg(any(feature = "compression-zstd", feature = "compression-zstd-pure"))]
            Compression::Zstd => "zstd",
            #[cfg(any(feature = "compression-zstd", feature = "compression-zstd-pure"))]
            Compression::ZstdLevel(_) => "zstd",
        }
    }

    /// Returns true if this is no compression.
    pub const fn is_none(self) -> bool {
        matches!(self, Compression::None)
    }

    /// Parse codec from header byte.
    fn from_codec_id(id: u8) -> Option<Self> {
        match id {
            0 => Some(Compression::None),
            #[cfg(feature = "compression-lz4")]
            1 => Some(Compression::Lz4),
            #[cfg(any(feature = "compression-zstd", feature = "compression-zstd-pure"))]
            2 => Some(Compression::Zstd),
            _ => None,
        }
    }
}

/// Compression statistics.
#[derive(Debug, Clone, Copy, Default)]
pub struct CompressionStats {
    /// Original (uncompressed) size in bytes.
    pub original_size: usize,
    /// Compressed size in bytes (including header).
    pub compressed_size: usize,
}

impl CompressionStats {
    /// Returns the compression ratio (original / compressed).
    /// A ratio > 1.0 means compression saved space.
    pub fn ratio(&self) -> f64 {
        if self.compressed_size == 0 {
            0.0
        } else {
            self.original_size as f64 / self.compressed_size as f64
        }
    }

    /// Returns the space saving as a percentage (0-100).
    pub fn savings_percent(&self) -> f64 {
        if self.original_size == 0 {
            0.0
        } else {
            100.0 * (1.0 - (self.compressed_size as f64 / self.original_size as f64))
        }
    }
}

/// Compress data using the specified compression algorithm.
///
/// Returns the compressed data with a header for automatic detection.
#[cfg(feature = "alloc")]
pub fn compress(data: &[u8], compression: Compression) -> Result<alloc::vec::Vec<u8>> {
    if compression.is_none() {
        // No compression - just copy with header
        let mut output = alloc::vec::Vec::with_capacity(HEADER_SIZE + data.len());
        output.extend_from_slice(&MAGIC);
        output.push(VERSION);
        output.push(compression.codec_id());
        output.extend_from_slice(data);
        return Ok(output);
    }

    match compression {
        Compression::None => unreachable!(),

        #[cfg(feature = "compression-lz4")]
        Compression::Lz4 => {
            let compressed = lz4::compress(data)?;
            let mut output = alloc::vec::Vec::with_capacity(HEADER_SIZE + compressed.len());
            output.extend_from_slice(&MAGIC);
            output.push(VERSION);
            output.push(compression.codec_id());
            output.extend_from_slice(&compressed);
            Ok(output)
        }

        #[cfg(feature = "compression-zstd")]
        Compression::Zstd => {
            let compressed = zstd_impl::compress(data, 3)?; // Default level
            let mut output = alloc::vec::Vec::with_capacity(HEADER_SIZE + compressed.len());
            output.extend_from_slice(&MAGIC);
            output.push(VERSION);
            output.push(compression.codec_id());
            output.extend_from_slice(&compressed);
            Ok(output)
        }

        #[cfg(feature = "compression-zstd")]
        Compression::ZstdLevel(level) => {
            let compressed = zstd_impl::compress(data, level as i32)?;
            let mut output = alloc::vec::Vec::with_capacity(HEADER_SIZE + compressed.len());
            output.extend_from_slice(&MAGIC);
            output.push(VERSION);
            output.push(compression.codec_id());
            output.extend_from_slice(&compressed);
            Ok(output)
        }

        // When only pure Rust is available, compression is not supported
        #[cfg(all(feature = "compression-zstd-pure", not(feature = "compression-zstd")))]
        Compression::Zstd | Compression::ZstdLevel(_) => {
            Err(Error::Custom {
                message: "Zstd compression requires the 'compression-zstd' feature (C toolchain). The 'compression-zstd-pure' feature only supports decompression.",
            })
        }
    }
}

/// Compress data and return statistics.
#[cfg(feature = "alloc")]
pub fn compress_with_stats(
    data: &[u8],
    compression: Compression,
) -> Result<(alloc::vec::Vec<u8>, CompressionStats)> {
    let original_size = data.len();
    let compressed = compress(data, compression)?;
    let compressed_size = compressed.len();

    Ok((
        compressed,
        CompressionStats {
            original_size,
            compressed_size,
        },
    ))
}

/// Decompress data that was compressed with `compress`.
///
/// Automatically detects the compression format from the header.
#[cfg(feature = "alloc")]
pub fn decompress(data: &[u8]) -> Result<alloc::vec::Vec<u8>> {
    if data.len() < HEADER_SIZE {
        return Err(Error::UnexpectedEnd {
            additional: HEADER_SIZE - data.len(),
        });
    }

    // Check magic
    if data[0..3] != MAGIC {
        return Err(Error::InvalidData {
            message: "invalid compression header magic",
        });
    }

    // Check version
    let version = data[3];
    if version != VERSION {
        return Err(Error::InvalidData {
            message: "unsupported compression version",
        });
    }

    // Get codec
    let codec_id = data[4];
    let compression = Compression::from_codec_id(codec_id).ok_or(Error::InvalidData {
        message: "unknown compression codec",
    })?;

    let payload = &data[HEADER_SIZE..];

    match compression {
        Compression::None => Ok(payload.to_vec()),

        #[cfg(feature = "compression-lz4")]
        Compression::Lz4 => lz4::decompress(payload),

        #[cfg(feature = "compression-zstd")]
        Compression::Zstd | Compression::ZstdLevel(_) => zstd_impl::decompress(payload),

        #[cfg(all(feature = "compression-zstd-pure", not(feature = "compression-zstd")))]
        Compression::Zstd | Compression::ZstdLevel(_) => ruzstd_impl::decompress(payload),
    }
}

/// Try to decompress data, falling back to the original if not compressed.
///
/// This is useful when you're not sure if data is compressed or not.
#[cfg(feature = "alloc")]
pub fn decompress_or_passthrough(data: &[u8]) -> Result<alloc::vec::Vec<u8>> {
    if is_compressed(data) {
        decompress(data)
    } else {
        Ok(data.to_vec())
    }
}

/// Check if data appears to be compressed (has valid magic header).
pub fn is_compressed(data: &[u8]) -> bool {
    data.len() >= HEADER_SIZE && data[0..3] == MAGIC && data[3] == VERSION
}

/// Detect the compression type from compressed data.
pub fn detect_compression(data: &[u8]) -> Option<Compression> {
    if data.len() < HEADER_SIZE || data[0..3] != MAGIC || data[3] != VERSION {
        return None;
    }
    Compression::from_codec_id(data[4])
}

#[cfg(test)]
mod tests {
    use super::*;

    #[cfg(all(feature = "alloc", feature = "compression-lz4"))]
    #[test]
    fn test_lz4_roundtrip() {
        let data = b"Hello, World! This is some test data for compression.";
        let compressed = compress(data, Compression::Lz4).expect("compress failed");
        let decompressed = decompress(&compressed).expect("decompress failed");
        assert_eq!(data.as_slice(), decompressed.as_slice());
    }

    #[cfg(all(feature = "alloc", feature = "compression-lz4"))]
    #[test]
    fn test_lz4_large_data() {
        // Create large repetitive data (compresses well)
        let data: alloc::vec::Vec<u8> = (0..100000).map(|i| (i % 256) as u8).collect();
        let compressed = compress(&data, Compression::Lz4).expect("compress failed");
        let decompressed = decompress(&compressed).expect("decompress failed");
        assert_eq!(data, decompressed);

        // Should have actually compressed
        assert!(compressed.len() < data.len());
    }

    #[cfg(all(feature = "alloc", feature = "compression-zstd"))]
    #[test]
    fn test_zstd_roundtrip() {
        let data = b"Hello, World! This is some test data for compression.";
        let compressed = compress(data, Compression::Zstd).expect("compress failed");
        let decompressed = decompress(&compressed).expect("decompress failed");
        assert_eq!(data.as_slice(), decompressed.as_slice());
    }

    #[cfg(all(feature = "alloc", feature = "compression-zstd"))]
    #[test]
    fn test_zstd_level() {
        let data = b"Hello, World! This is some test data for compression.";
        let compressed = compress(data, Compression::ZstdLevel(19)).expect("compress failed");
        let decompressed = decompress(&compressed).expect("decompress failed");
        assert_eq!(data.as_slice(), decompressed.as_slice());
    }

    #[cfg(feature = "alloc")]
    #[test]
    fn test_none_roundtrip() {
        let data = b"Hello, World!";
        let compressed = compress(data, Compression::None).expect("compress failed");
        let decompressed = decompress(&compressed).expect("decompress failed");
        assert_eq!(data.as_slice(), decompressed.as_slice());
    }

    #[cfg(feature = "alloc")]
    #[test]
    fn test_is_compressed() {
        let data = b"Hello, World!";
        assert!(!is_compressed(data));

        let compressed = compress(data, Compression::None).expect("compress failed");
        assert!(is_compressed(&compressed));
    }

    #[cfg(all(feature = "alloc", feature = "compression-lz4"))]
    #[test]
    fn test_detect_compression() {
        let data = b"Hello, World!";
        let compressed = compress(data, Compression::Lz4).expect("compress failed");
        assert_eq!(detect_compression(&compressed), Some(Compression::Lz4));
    }

    #[cfg(all(feature = "alloc", feature = "compression-lz4"))]
    #[test]
    fn test_compression_stats() {
        let data: alloc::vec::Vec<u8> = (0..10000).map(|i| (i % 256) as u8).collect();
        let (compressed, stats) =
            compress_with_stats(&data, Compression::Lz4).expect("compress failed");

        assert_eq!(stats.original_size, data.len());
        assert_eq!(stats.compressed_size, compressed.len());
        assert!(stats.ratio() > 1.0); // Should have compressed
        assert!(stats.savings_percent() > 0.0);
    }

    #[cfg(feature = "alloc")]
    #[test]
    fn test_decompress_or_passthrough() {
        // Uncompressed data
        let raw = b"Hello, World!";
        let result = decompress_or_passthrough(raw).expect("failed");
        assert_eq!(raw.as_slice(), result.as_slice());

        // Compressed data
        let compressed = compress(raw, Compression::None).expect("compress failed");
        let result = decompress_or_passthrough(&compressed).expect("failed");
        assert_eq!(raw.as_slice(), result.as_slice());
    }

    #[test]
    fn test_compression_codec_id() {
        assert_eq!(Compression::None.codec_id(), 0);
        #[cfg(feature = "compression-lz4")]
        assert_eq!(Compression::Lz4.codec_id(), 1);
        #[cfg(any(feature = "compression-zstd", feature = "compression-zstd-pure"))]
        assert_eq!(Compression::Zstd.codec_id(), 2);
    }

    #[test]
    fn test_compression_name() {
        assert_eq!(Compression::None.name(), "none");
        #[cfg(feature = "compression-lz4")]
        assert_eq!(Compression::Lz4.name(), "lz4");
        #[cfg(any(feature = "compression-zstd", feature = "compression-zstd-pure"))]
        assert_eq!(Compression::Zstd.name(), "zstd");
    }
}
