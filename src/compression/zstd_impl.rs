//! Zstd compression implementation.
//!
//! Zstd (Zstandard) offers excellent compression ratios with still fast performance.
//! It's ideal for storage and network transmission where better compression is valuable.
//!
//! Features:
//! - Compression levels 1-22 (default: 3)
//! - Fast decompression (~1 GB/s)
//! - Better ratio than LZ4 (typically 2-3x smaller)

use crate::{Error, Result};

#[cfg(feature = "alloc")]
extern crate alloc;

/// Default compression level for Zstd.
#[allow(dead_code)]
pub const DEFAULT_LEVEL: i32 = 3;

/// Compress data using Zstd with the specified compression level.
#[cfg(feature = "alloc")]
pub fn compress(data: &[u8], level: i32) -> Result<alloc::vec::Vec<u8>> {
    // Clamp level to valid range (1-22)
    let level = level.clamp(1, 22);

    zstd::bulk::compress(data, level).map_err(|e| Error::Custom {
        message: if e.to_string().contains("memory") {
            "Zstd compression: out of memory"
        } else {
            "Zstd compression error"
        },
    })
}

/// Decompress Zstd-compressed data.
#[cfg(feature = "alloc")]
pub fn decompress(data: &[u8]) -> Result<alloc::vec::Vec<u8>> {
    // Use streaming decoder which handles variable-size output better
    use std::io::Read;

    let mut decoder = zstd::Decoder::new(data).map_err(|_| Error::InvalidData {
        message: "Zstd: failed to create decoder",
    })?;

    let mut output = alloc::vec::Vec::new();
    decoder
        .read_to_end(&mut output)
        .map_err(|_| Error::InvalidData {
            message: "Zstd decompression error",
        })?;

    Ok(output)
}

/// Compress data with default level.
#[cfg(feature = "alloc")]
#[allow(dead_code)]
pub fn compress_default(data: &[u8]) -> Result<alloc::vec::Vec<u8>> {
    compress(data, DEFAULT_LEVEL)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[cfg(feature = "alloc")]
    #[test]
    fn test_compress_decompress() {
        let data = b"Hello, World! This is a test of Zstd compression.";
        let compressed = compress(data, DEFAULT_LEVEL).expect("compress failed");
        let decompressed = decompress(&compressed).expect("decompress failed");
        assert_eq!(data.as_slice(), decompressed.as_slice());
    }

    #[cfg(feature = "alloc")]
    #[test]
    fn test_empty_data() {
        let data: &[u8] = b"";
        let compressed = compress(data, DEFAULT_LEVEL).expect("compress failed");
        let decompressed = decompress(&compressed).expect("decompress failed");
        assert_eq!(data, decompressed.as_slice());
    }

    #[cfg(feature = "alloc")]
    #[test]
    fn test_large_data() {
        let data: alloc::vec::Vec<u8> = (0..100000).map(|i| (i % 256) as u8).collect();
        let compressed = compress(&data, DEFAULT_LEVEL).expect("compress failed");
        let decompressed = decompress(&compressed).expect("decompress failed");
        assert_eq!(data, decompressed);

        // Should have actually compressed
        assert!(compressed.len() < data.len());
    }

    #[cfg(feature = "alloc")]
    #[test]
    fn test_compression_levels() {
        let data: alloc::vec::Vec<u8> = (0..10000).map(|i| (i % 256) as u8).collect();

        // Test various levels
        for level in [1, 3, 9, 19, 22] {
            let compressed = compress(&data, level).expect("compress failed");
            let decompressed = decompress(&compressed).expect("decompress failed");
            assert_eq!(data, decompressed);
        }
    }

    #[cfg(feature = "alloc")]
    #[test]
    fn test_level_clamping() {
        let data = b"test data";

        // Level 0 should be clamped to 1
        let _ = compress(data, 0).expect("compress failed");

        // Level 30 should be clamped to 22
        let _ = compress(data, 30).expect("compress failed");
    }
}
