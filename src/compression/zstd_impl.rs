//! Zstd compression implementation using oxiarc-zstd (pure Rust).
//!
//! Zstd (Zstandard) offers excellent compression ratios with still fast performance.
//! It's ideal for storage and network transmission where better compression is valuable.
//!
//! Features:
//! - Compression levels 1-22 (default: 3)
//! - Fast decompression (~1 GB/s)
//! - Better ratio than LZ4 (typically 2-3x smaller)
//! - Pure Rust implementation (no C toolchain required)

use crate::{Error, Result};

#[cfg(feature = "alloc")]
extern crate alloc;

/// Default compression level for Zstd.
#[allow(dead_code)]
pub const DEFAULT_LEVEL: i32 = 3;

/// Zstandard frame magic number in little-endian byte order (0xFD2FB528).
const ZSTD_MAGIC: [u8; 4] = [0x28, 0xB5, 0x2F, 0xFD];

/// Maximum regenerated size of a single Zstandard block (128 KiB), mirrored
/// from `oxiarc_zstd::MAX_BLOCK_SIZE`. A compressed block can regenerate at
/// most this many bytes regardless of its stored (compressed) size.
const ZSTD_MAX_BLOCK_SIZE: usize = 128 * 1024;

/// Frame Header Descriptor flag bits (RFC 8878 §3.1.1.1).
const FHD_SINGLE_SEGMENT: u8 = 0x20;
const FHD_DICT_ID_FLAG_MASK: u8 = 0x03;
const FHD_CONTENT_SIZE_FLAG_MASK: u8 = 0xC0;

/// Compress data using Zstd with the specified compression level.
#[cfg(feature = "alloc")]
pub fn compress(data: &[u8], level: i32) -> Result<alloc::vec::Vec<u8>> {
    // Clamp level to valid range (1-22)
    let level = level.clamp(1, 22);

    oxiarc_zstd::compress_with_level(data, level).map_err(|e| Error::OwnedCustom {
        message: alloc::format!("Zstd compression error: {e}"),
    })
}

/// Decompress Zstd-compressed data with a hard cap on the regenerated size.
///
/// `oxiarc_zstd::decompress` appends every block to its output buffer without
/// any running total bound and only verifies the declared `Frame_Content_Size`
/// *after* the whole frame has been expanded. A hostile frame can therefore
/// declare a tiny content size (or omit it entirely) yet stream many RLE/raw
/// blocks — regenerating up to 128 KiB per 4-byte block header — and force
/// gigabytes of allocation before the mismatch is detected. This is a classic
/// decompression bomb.
///
/// To keep memory bounded on untrusted input we do a header-only pre-scan
/// *before* invoking the decoder:
///
/// 1. Parse the frame header. oxicode's own encoder always emits
///    `Frame_Content_Size`, so a frame that omits it is rejected outright.
/// 2. Reject a declared content size larger than `max_output`.
/// 3. Walk the block headers (never their contents), summing an upper bound on
///    the regenerated size (`block_size` for raw/RLE blocks, [`ZSTD_MAX_BLOCK_SIZE`]
///    for compressed blocks) and rejecting as soon as that sum exceeds the
///    declared content size by more than one block. An honest frame overshoots
///    by at most its last partial block; a bomb — which under-declares its
///    content size while streaming many full blocks — trips the check on the
///    second block.
///
/// An accepted frame therefore expands to at most `content_size` +
/// `ZSTD_MAX_BLOCK_SIZE`, i.e. `max_output` plus at most one 128 KiB block of
/// slack. Only after the pre-scan do we hand the frame to
/// `oxiarc_zstd::decompress`.
#[cfg(feature = "alloc")]
pub fn decompress(data: &[u8], max_output: usize) -> Result<alloc::vec::Vec<u8>> {
    validate_zstd_frame_bound(data, max_output)?;

    oxiarc_zstd::decompress(data).map_err(|e| Error::OwnedCustom {
        message: alloc::format!("Zstd decompression error: {e}"),
    })
}

/// Header-only pre-scan enforcing a hard upper bound on the regenerated size.
///
/// Returns `Ok(())` when the frame provably cannot regenerate more than
/// `max_output` bytes, and an error otherwise (omitted content size, declared
/// size over the cap, a block bound that exceeds the cap, or a malformed
/// header). Content bytes are never decoded here — this only reads frame- and
/// block-header fields, so it runs in `O(number_of_blocks)` time and constant
/// extra memory.
#[cfg(feature = "alloc")]
fn validate_zstd_frame_bound(data: &[u8], max_output: usize) -> Result<()> {
    if data.len() < 5 {
        return Err(Error::UnexpectedEnd {
            additional: 5usize.saturating_sub(data.len()),
        });
    }
    if data[0..4] != ZSTD_MAGIC {
        return Err(Error::InvalidData {
            message: "invalid Zstd frame magic",
        });
    }

    let descriptor = data[4];
    let single_segment = (descriptor & FHD_SINGLE_SEGMENT) != 0;
    let dict_id_flag = descriptor & FHD_DICT_ID_FLAG_MASK;
    let content_size_flag = (descriptor & FHD_CONTENT_SIZE_FLAG_MASK) >> 6;

    let mut pos = 5usize;

    // Window descriptor (present only when not a single-segment frame).
    if !single_segment {
        if data.len() <= pos {
            return Err(Error::UnexpectedEnd { additional: 1 });
        }
        pos += 1;
    }

    // Dictionary ID (0/1/2/4 bytes depending on the flag).
    let dict_id_bytes = match dict_id_flag {
        0 => 0usize,
        1 => 1,
        2 => 2,
        _ => 4,
    };
    pos = pos.checked_add(dict_id_bytes).ok_or(Error::InvalidData {
        message: "malformed Zstd frame header",
    })?;
    if data.len() < pos {
        return Err(Error::UnexpectedEnd {
            additional: pos - data.len(),
        });
    }

    // Frame Content Size. oxicode always emits it; a frame that omits it is
    // treated as hostile (unbounded output) and rejected.
    let content_size = if single_segment || content_size_flag != 0 {
        let size_bytes = match content_size_flag {
            0 => 1usize, // single-segment implies a 1-byte field
            1 => 2,
            2 => 4,
            _ => 8,
        };
        let end = pos.checked_add(size_bytes).ok_or(Error::InvalidData {
            message: "malformed Zstd frame header",
        })?;
        if data.len() < end {
            return Err(Error::UnexpectedEnd {
                additional: end - data.len(),
            });
        }
        let size = match size_bytes {
            1 => data[pos] as u64,
            2 => u16::from_le_bytes([data[pos], data[pos + 1]]) as u64 + 256,
            4 => {
                u32::from_le_bytes([data[pos], data[pos + 1], data[pos + 2], data[pos + 3]]) as u64
            }
            _ => u64::from_le_bytes([
                data[pos],
                data[pos + 1],
                data[pos + 2],
                data[pos + 3],
                data[pos + 4],
                data[pos + 5],
                data[pos + 6],
                data[pos + 7],
            ]),
        };
        pos = end;
        size
    } else {
        return Err(Error::InvalidData {
            message: "Zstd frame omits content size; refusing to decompress (possible decompression bomb)",
        });
    };

    let max_output_u64 = max_output as u64;
    if content_size > max_output_u64 {
        return Err(Error::LimitExceeded {
            limit: max_output_u64,
            found: content_size,
        });
    }

    // The decoder appends every block's regenerated bytes and only verifies the
    // declared content size at the very end. To keep peak memory bounded we
    // require the summed regenerated-size upper bound to stay within one block
    // of the declared content size. An honest frame overshoots that bound by at
    // most its last (partial) block — every full block regenerates exactly its
    // chunk size, and only the compressed-block case is estimated, at its 128 KiB
    // ceiling. A bomb, by contrast, lies about its content size while streaming
    // many full blocks, so it blows past the threshold on the second block.
    //
    // Consequently an accepted frame expands to at most
    // `content_size + ZSTD_MAX_BLOCK_SIZE`, i.e. `<= max_output` plus at most one
    // 128 KiB block of slack.
    let regen_ceiling = content_size.saturating_add(ZSTD_MAX_BLOCK_SIZE as u64);

    // Walk block headers, accumulating an upper bound on the regenerated size.
    let mut total: u64 = 0;
    loop {
        if data.len() < pos + 3 {
            return Err(Error::UnexpectedEnd {
                additional: (pos + 3).saturating_sub(data.len()),
            });
        }
        let block_header = u32::from_le_bytes([data[pos], data[pos + 1], data[pos + 2], 0]);
        pos += 3;

        let last_block = (block_header & 1) != 0;
        let block_type = (block_header >> 1) & 0x03;
        let block_size = ((block_header >> 3) & 0x1F_FFFF) as usize;

        if block_size > ZSTD_MAX_BLOCK_SIZE {
            return Err(Error::InvalidData {
                message: "Zstd block size exceeds maximum",
            });
        }

        // Regenerated-size upper bound and stored (compressed) size per block.
        let (regen_bound, stored) = match block_type {
            0 => (block_size as u64, block_size),          // Raw block
            1 => (block_size as u64, 1usize),              // RLE block (1 data byte)
            2 => (ZSTD_MAX_BLOCK_SIZE as u64, block_size), // Compressed block
            _ => {
                return Err(Error::InvalidData {
                    message: "Zstd reserved block type",
                });
            }
        };

        total = total.saturating_add(regen_bound);
        if total > regen_ceiling {
            return Err(Error::LimitExceeded {
                limit: regen_ceiling,
                found: total,
            });
        }

        pos = pos.checked_add(stored).ok_or(Error::InvalidData {
            message: "malformed Zstd block header",
        })?;
        if data.len() < pos {
            return Err(Error::UnexpectedEnd {
                additional: pos - data.len(),
            });
        }

        if last_block {
            break;
        }
    }

    Ok(())
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

    /// Cap used by the unit tests; large enough for the fixtures, small enough
    /// to keep the bomb tests cheap.
    #[cfg(feature = "alloc")]
    const TEST_CAP: usize = 8 * 1024 * 1024;

    #[cfg(feature = "alloc")]
    #[test]
    fn test_compress_decompress() {
        let data = b"Hello, World! This is a test of Zstd compression.";
        let compressed = compress(data, DEFAULT_LEVEL).expect("compress failed");
        let decompressed = decompress(&compressed, TEST_CAP).expect("decompress failed");
        assert_eq!(data.as_slice(), decompressed.as_slice());
    }

    #[cfg(feature = "alloc")]
    #[test]
    fn test_empty_data() {
        let data: &[u8] = b"";
        let compressed = compress(data, DEFAULT_LEVEL).expect("compress failed");
        let decompressed = decompress(&compressed, TEST_CAP).expect("decompress failed");
        assert_eq!(data, decompressed.as_slice());
    }

    #[cfg(feature = "alloc")]
    #[test]
    fn test_large_data() {
        let data: alloc::vec::Vec<u8> = (0..100000).map(|i| (i % 256) as u8).collect();
        let compressed = compress(&data, DEFAULT_LEVEL).expect("compress failed");
        let decompressed = decompress(&compressed, TEST_CAP).expect("decompress failed");
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
            let decompressed = decompress(&compressed, TEST_CAP).expect("decompress failed");
            assert_eq!(data, decompressed);
        }
    }

    #[cfg(feature = "alloc")]
    #[test]
    fn test_rle_bomb_rejected_by_cap() {
        // Craft a single-segment frame that declares a tiny content size but
        // streams several maximum-size RLE blocks. Each block regenerates
        // ZSTD_MAX_BLOCK_SIZE (128 KiB) from a single data byte — the classic
        // RLE amplification — so their sum blows past a small cap. The pre-scan
        // must reject the frame before any large allocation happens.
        //
        // Header: magic + descriptor (single-segment, 1-byte content size).
        let mut frame = alloc::vec::Vec::new();
        frame.extend_from_slice(&ZSTD_MAGIC);
        frame.push(FHD_SINGLE_SEGMENT); // single-segment, content_size_flag=0 => 1 byte FCS
        frame.push(1u8); // declared content size = 1 byte (a lie)

        // Eight max-size RLE blocks => ~1 MiB of regenerated output from ~40 bytes.
        let regen: u32 = ZSTD_MAX_BLOCK_SIZE as u32;
        for i in 0..8u32 {
            let last = if i == 7 { 1 } else { 0 };
            let block_header: u32 = (regen << 3) | (1 << 1) | last; // type=RLE(1)
            frame.push((block_header & 0xFF) as u8);
            frame.push(((block_header >> 8) & 0xFF) as u8);
            frame.push(((block_header >> 16) & 0xFF) as u8);
            frame.push(0xAB); // single RLE data byte
        }

        // Small cap: the ~1 MiB of RLE output must be rejected.
        let err = decompress(&frame, 64 * 1024).expect_err("bomb must be rejected");
        assert!(matches!(err, Error::LimitExceeded { .. }));
    }

    #[cfg(feature = "alloc")]
    #[test]
    fn test_omitted_content_size_rejected() {
        // A frame with neither single-segment nor a content-size flag omits the
        // Frame_Content_Size field; oxicode always emits it, so this is refused.
        let mut frame = alloc::vec::Vec::new();
        frame.extend_from_slice(&ZSTD_MAGIC);
        frame.push(0u8); // descriptor: no single-segment, no content size
        frame.push(0u8); // window descriptor
                         // A trivial last raw block of size 0.
        let block_header: u32 = 1; // type=Raw(0), size=0, last=1
        frame.push((block_header & 0xFF) as u8);
        frame.push(((block_header >> 8) & 0xFF) as u8);
        frame.push(((block_header >> 16) & 0xFF) as u8);

        let err = decompress(&frame, TEST_CAP).expect_err("omitted content size must be rejected");
        assert!(matches!(err, Error::InvalidData { .. }));
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
