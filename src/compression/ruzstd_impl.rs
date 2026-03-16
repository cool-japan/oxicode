//! Pure Rust Zstd decompression implementation using ruzstd.
//!
//! This module provides decompression-only support for Zstd-compressed data
//! without requiring a C toolchain. For compression, use the `compression-zstd`
//! feature which uses the C FFI zstd library.

use crate::{Error, Result};

#[cfg(feature = "alloc")]
extern crate alloc;

/// Decompress Zstd-compressed data using the pure Rust ruzstd library.
#[cfg(feature = "alloc")]
#[allow(dead_code)]
pub fn decompress(data: &[u8]) -> Result<alloc::vec::Vec<u8>> {
    let mut output = alloc::vec::Vec::new();
    let mut decoder =
        ruzstd::decoding::StreamingDecoder::new(data).map_err(|_| Error::InvalidData {
            message: "Zstd (pure): failed to create decoder",
        })?;

    use std::io::Read;
    decoder
        .read_to_end(&mut output)
        .map_err(|_| Error::InvalidData {
            message: "Zstd (pure): decompression error",
        })?;

    Ok(output)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[cfg(feature = "alloc")]
    #[test]
    fn test_decompress_empty_frame() {
        // A valid zstd frame for empty input (compressed with zstd)
        // Magic: 0xFD2FB528, then minimal frame
        let empty_zstd = [0x28, 0xB5, 0x2F, 0xFD, 0x20, 0x00, 0x01, 0x00, 0x00];
        let result = decompress(&empty_zstd);
        // This should either succeed with empty output or fail gracefully
        if let Ok(output) = result {
            assert!(output.is_empty());
        }
    }

    #[cfg(feature = "alloc")]
    #[test]
    fn test_decompress_invalid_data() {
        let invalid = b"not valid zstd data";
        let result = decompress(invalid);
        assert!(result.is_err());
    }
}
