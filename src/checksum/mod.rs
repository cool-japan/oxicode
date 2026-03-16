//! CRC32 checksum/integrity verification for OxiCode.
//!
//! This module provides transparent integrity checking for serialized data.
//! A CRC32 checksum is computed over the payload and prepended as a header,
//! allowing detection of data corruption or tampering.
//!
//! ## Wire Format
//!
//! ```text
//! [MAGIC: 3 bytes][VERSION: 1 byte][LEN: 8 bytes LE u64][CRC32: 4 bytes LE u32][PAYLOAD: N bytes]
//! ```
//!
//! - **MAGIC**: `[0x4F, 0x58, 0x48]` ("OXH" = OXicode Hash)
//! - **VERSION**: format version byte (currently `1`)
//! - **LEN**: payload length as little-endian u64
//! - **CRC32**: CRC32 checksum of payload bytes as little-endian u32
//! - **PAYLOAD**: the encoded data
//!
//! ## Example
//!
//! ```rust,ignore
//! use oxicode::checksum::{encode_with_checksum, decode_with_checksum};
//!
//! let value = 42u32;
//! let wrapped = encode_with_checksum(&value)?;
//! let (decoded, _): (u32, _) = decode_with_checksum(&wrapped)?;
//! assert_eq!(value, decoded);
//! ```

#[cfg(feature = "alloc")]
extern crate alloc;

use crate::{Error, Result};

/// Magic bytes identifying checksummed OxiCode data.
/// "OXH" = OXicode Hash
pub const MAGIC: [u8; 3] = [0x4F, 0x58, 0x48];

/// Version byte for the checksum format.
pub const FORMAT_VERSION: u8 = 1;

/// Total header size in bytes: MAGIC(3) + VERSION(1) + LEN(8) + CRC32(4) = 16
pub const HEADER_SIZE: usize = 16;

/// Offset of the VERSION byte in the header.
const VERSION_OFFSET: usize = 3;

/// Offset of the LEN field in the header.
const LEN_OFFSET: usize = 4;

/// Offset of the CRC32 field in the header.
const CRC_OFFSET: usize = 12;

/// Wrap raw bytes with a CRC32 checksum header.
///
/// The output format is:
/// `[MAGIC(3)][VERSION(1)][LEN(8 LE)][CRC32(4 LE)][PAYLOAD]`
#[cfg(feature = "alloc")]
pub fn wrap_with_checksum(data: &[u8]) -> alloc::vec::Vec<u8> {
    let crc = crc32fast::hash(data);
    let len = data.len() as u64;

    let mut output = alloc::vec::Vec::with_capacity(HEADER_SIZE + data.len());
    output.extend_from_slice(&MAGIC);
    output.push(FORMAT_VERSION);
    output.extend_from_slice(&len.to_le_bytes());
    output.extend_from_slice(&crc.to_le_bytes());
    output.extend_from_slice(data);
    output
}

/// Verify the CRC32 checksum of wrapped data and return the payload slice.
///
/// Returns a reference to the payload portion of `data` (zero-copy).
///
/// # Errors
///
/// - [`Error::UnexpectedEnd`] if `data` is too short to contain a header
/// - [`Error::InvalidData`] if the magic bytes or version are wrong
/// - [`Error::ChecksumMismatch`] if the CRC32 does not match
pub fn verify_checksum(data: &[u8]) -> Result<&[u8]> {
    if data.len() < HEADER_SIZE {
        return Err(Error::UnexpectedEnd {
            additional: HEADER_SIZE - data.len(),
        });
    }

    // Validate magic
    if data[..3] != MAGIC {
        return Err(Error::InvalidData {
            message: "invalid checksum header magic",
        });
    }

    // Validate version
    let version = data[VERSION_OFFSET];
    if version != FORMAT_VERSION {
        return Err(Error::InvalidData {
            message: "unsupported checksum format version",
        });
    }

    // Read stored payload length
    let stored_len =
        u64::from_le_bytes(data[LEN_OFFSET..LEN_OFFSET + 8].try_into().map_err(|_| {
            Error::InvalidData {
                message: "failed to read checksum payload length",
            }
        })?) as usize;

    // Validate total length
    let expected_total = HEADER_SIZE + stored_len;
    if data.len() < expected_total {
        return Err(Error::UnexpectedEnd {
            additional: expected_total - data.len(),
        });
    }

    // Read stored CRC32
    let stored_crc =
        u32::from_le_bytes(data[CRC_OFFSET..CRC_OFFSET + 4].try_into().map_err(|_| {
            Error::InvalidData {
                message: "failed to read checksum value",
            }
        })?);

    // Extract payload
    let payload = &data[HEADER_SIZE..HEADER_SIZE + stored_len];

    // Compute and verify CRC32
    let computed_crc = crc32fast::hash(payload);
    if computed_crc != stored_crc {
        return Err(Error::ChecksumMismatch {
            expected: stored_crc,
            found: computed_crc,
        });
    }

    Ok(payload)
}

/// Encode a value and wrap with CRC32 checksum.
///
/// Equivalent to `encode_to_vec` followed by `wrap_with_checksum`.
#[cfg(feature = "alloc")]
pub fn encode_with_checksum<E: crate::Encode>(value: &E) -> Result<alloc::vec::Vec<u8>> {
    encode_with_checksum_config(value, crate::config::standard())
}

/// Encode a value with a custom configuration and wrap with CRC32 checksum.
#[cfg(feature = "alloc")]
pub fn encode_with_checksum_config<E: crate::Encode, C: crate::config::Config>(
    value: &E,
    config: C,
) -> Result<alloc::vec::Vec<u8>> {
    let payload = crate::encode_to_vec_with_config(value, config)?;
    Ok(wrap_with_checksum(&payload))
}

/// Decode a value from checksummed data using the standard configuration.
///
/// Verifies the CRC32 checksum before decoding. Returns both the decoded value
/// and the total number of bytes consumed (including the header).
#[cfg(feature = "alloc")]
pub fn decode_with_checksum<D: crate::Decode>(data: &[u8]) -> Result<(D, usize)> {
    decode_with_checksum_config(data, crate::config::standard())
}

/// Unwrap checksummed data and return the payload bytes as an owned `Vec<u8>`.
///
/// This is the owned-output counterpart of [`verify_checksum`], which returns a
/// borrowed slice.  Useful when the caller needs to store or further process the
/// payload independently of the original buffer.
///
/// # Errors
///
/// Propagates the same errors as [`verify_checksum`].
#[cfg(feature = "alloc")]
pub fn unwrap_with_checksum(data: &[u8]) -> Result<alloc::vec::Vec<u8>> {
    verify_checksum(data).map(|payload| payload.to_vec())
}

/// Type alias for the error type returned by checksum operations.
///
/// All checksum functions return [`crate::Error`]; this alias exists as a
/// convenience import so callers do not need to import the root error type
/// separately.
pub type ChecksumError = crate::Error;

/// Decode a value from checksummed data with a custom configuration.
///
/// Returns `(value, total_bytes_consumed)` where total includes the header.
#[cfg(feature = "alloc")]
pub fn decode_with_checksum_config<D: crate::Decode, C: crate::config::Config>(
    data: &[u8],
    config: C,
) -> Result<(D, usize)> {
    let payload = verify_checksum(data)?;
    let (value, inner_consumed) = crate::decode_from_slice_with_config(payload, config)?;
    Ok((value, HEADER_SIZE + inner_consumed))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_wrap_verify_roundtrip() {
        let data = b"Hello, OxiCode!";
        let wrapped = wrap_with_checksum(data);
        let payload = verify_checksum(&wrapped).expect("verify failed");
        assert_eq!(payload, data);
    }

    #[test]
    fn test_header_size() {
        let wrapped = wrap_with_checksum(b"test");
        assert_eq!(wrapped.len(), HEADER_SIZE + 4);
    }

    #[test]
    fn test_wrong_magic() {
        let mut wrapped = wrap_with_checksum(b"test");
        wrapped[0] = 0xFF; // corrupt magic
        assert!(verify_checksum(&wrapped).is_err());
    }

    #[test]
    fn test_corrupted_payload() {
        let mut wrapped = wrap_with_checksum(b"Hello, OxiCode!");
        // Corrupt the payload
        let payload_start = HEADER_SIZE;
        wrapped[payload_start] ^= 0xFF;
        let result = verify_checksum(&wrapped);
        assert!(result.is_err());
        if let Err(Error::ChecksumMismatch { .. }) = result {
            // expected
        } else {
            panic!("expected ChecksumMismatch error");
        }
    }

    #[test]
    fn test_too_short() {
        let result = verify_checksum(&[0x4F, 0x58]);
        assert!(matches!(result, Err(Error::UnexpectedEnd { .. })));
    }

    #[test]
    fn test_empty_payload() {
        let wrapped = wrap_with_checksum(b"");
        let payload = verify_checksum(&wrapped).expect("verify failed");
        assert_eq!(payload, b"");
    }

    #[test]
    fn test_large_payload() {
        let data: alloc::vec::Vec<u8> = (0u8..=255).cycle().take(100000).collect();
        let wrapped = wrap_with_checksum(&data);
        let payload = verify_checksum(&wrapped).expect("verify failed");
        assert_eq!(payload, data.as_slice());
    }
}
