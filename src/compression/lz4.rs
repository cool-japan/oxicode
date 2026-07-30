//! LZ4 compression implementation using oxiarc-lz4 (pure Rust).
//!
//! LZ4 is an extremely fast compression algorithm with good compression ratios.
//! It's ideal for real-time applications where decompression speed is critical.
//!
//! Decompression speed: ~4 GB/s on modern hardware.

use crate::{Error, Result};

#[cfg(feature = "alloc")]
extern crate alloc;

/// Standard LZ4 frame magic number (little-endian value 0x184D2204).
const LZ4_FRAME_MAGIC: u32 = 0x184D2204;

/// FLG byte bit indicating the optional Content_Size field is present.
const LZ4_FLG_CONTENT_SIZE: u8 = 0x08;

/// Compress data using LZ4 frame format.
#[cfg(feature = "alloc")]
pub fn compress(data: &[u8]) -> Result<alloc::vec::Vec<u8>> {
    oxiarc_lz4::compress(data).map_err(|e| Error::OwnedCustom {
        message: alloc::format!("LZ4 compression error: {e}"),
    })
}

/// Decompress LZ4-compressed data (frame format) with a hard output cap.
///
/// `oxiarc_lz4::decompress` enforces `max_output` inside its block loop, so the
/// regenerated size is already bounded. The one remaining hazard is the *initial*
/// capacity reservation: when a frame omits the optional Content_Size field the
/// decoder seeds its output buffer with the full `max_output`, so a ~12-byte
/// hostile frame would trigger a `max_output`-sized reservation up front.
/// oxicode's own encoder always emits the Content_Size field, so we reject any
/// frame that lacks it (or uses a non-standard frame variant) before handing the
/// data to the decoder.
#[cfg(feature = "alloc")]
pub fn decompress(data: &[u8], max_output: usize) -> Result<alloc::vec::Vec<u8>> {
    reject_unbounded_lz4_frame(data)?;

    oxiarc_lz4::decompress(data, max_output).map_err(|e| Error::OwnedCustom {
        message: alloc::format!("LZ4 decompression error: {e}"),
    })
}

/// Reject LZ4 frames that would force an unbounded up-front reservation.
///
/// Returns an error for a standard frame whose FLG byte clears the Content_Size
/// bit, or for any non-standard frame magic (legacy/simple frames also reserve
/// the full `max_output`). oxicode never produces such frames.
#[cfg(feature = "alloc")]
fn reject_unbounded_lz4_frame(data: &[u8]) -> Result<()> {
    // Frames shorter than a full standard header cannot reach the decoder's
    // reservation step (it errors on "frame too short" first); let the decoder
    // produce the precise diagnostic in that case.
    if data.len() < 6 {
        return Ok(());
    }
    let magic = u32::from_le_bytes([data[0], data[1], data[2], data[3]]);
    if magic != LZ4_FRAME_MAGIC {
        return Err(Error::InvalidData {
            message: "unsupported LZ4 frame variant (expected standard frame with content size)",
        });
    }
    let flg = data[4];
    if flg & LZ4_FLG_CONTENT_SIZE == 0 {
        return Err(Error::InvalidData {
            message:
                "LZ4 frame omits content size; refusing to decompress (possible decompression bomb)",
        });
    }
    Ok(())
}

/// Compress data into a pre-allocated buffer.
/// Returns the number of bytes written.
#[allow(dead_code)]
#[cfg(feature = "alloc")]
pub fn compress_into(src: &[u8], dst: &mut [u8]) -> Result<usize> {
    let compressed = oxiarc_lz4::compress(src).map_err(|e| Error::OwnedCustom {
        message: alloc::format!("LZ4 compression failed: {e}"),
    })?;

    if dst.len() < compressed.len() {
        return Err(Error::UnexpectedEnd {
            additional: compressed.len() - dst.len(),
        });
    }

    dst[..compressed.len()].copy_from_slice(&compressed);
    Ok(compressed.len())
}

/// Get the maximum compressed size for a given input size.
/// LZ4 worst case is approximately input_size + (input_size / 255) + 16 + frame overhead.
#[allow(dead_code)]
pub fn max_compressed_size(input_size: usize) -> usize {
    // LZ4 frame overhead (header + end mark + optional checksum) + worst-case block expansion
    // Conservative estimate matching LZ4 spec
    input_size + (input_size / 255) + 32
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Cap used by the unit tests.
    #[cfg(feature = "alloc")]
    const TEST_CAP: usize = 8 * 1024 * 1024;

    #[cfg(feature = "alloc")]
    #[test]
    fn test_compress_decompress() {
        let data = b"Hello, World! This is a test of LZ4 compression.";
        let compressed = compress(data).expect("compress failed");
        let decompressed = decompress(&compressed, TEST_CAP).expect("decompress failed");
        assert_eq!(data.as_slice(), decompressed.as_slice());
    }

    #[cfg(feature = "alloc")]
    #[test]
    fn test_empty_data() {
        let data: &[u8] = b"";
        let compressed = compress(data).expect("compress failed");
        let decompressed = decompress(&compressed, TEST_CAP).expect("decompress failed");
        assert_eq!(data, decompressed.as_slice());
    }

    #[cfg(feature = "alloc")]
    #[test]
    fn test_large_data() {
        // Create repetitive data that compresses well
        let data: alloc::vec::Vec<u8> = (0..100000).map(|i| (i % 256) as u8).collect();
        let compressed = compress(&data).expect("compress failed");
        let decompressed = decompress(&compressed, TEST_CAP).expect("decompress failed");
        assert_eq!(data, decompressed);

        // Should have actually compressed
        assert!(compressed.len() < data.len());
    }

    #[cfg(feature = "alloc")]
    #[test]
    fn test_incompressible_data() {
        // Random-ish data that doesn't compress well
        let data: alloc::vec::Vec<u8> = (0..1000).map(|i| ((i * 17 + 31) % 256) as u8).collect();
        let compressed = compress(&data).expect("compress failed");
        let decompressed = decompress(&compressed, TEST_CAP).expect("decompress failed");
        assert_eq!(data, decompressed);
    }

    #[test]
    fn test_max_compressed_size() {
        let size = max_compressed_size(1000);
        // Should be larger than input (overhead + worst case)
        assert!(size > 1000);
    }
}
