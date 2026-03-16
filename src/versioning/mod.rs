//! Schema versioning and evolution support for oxicode.
//!
//! This module enables safe format evolution without breaking backward compatibility.
//! It provides version headers, compatibility checking, and migration hooks.
//!
//! ## Features
//!
//! - **Semantic Versioning**: Major.minor.patch version tracking
//! - **Compatibility Checking**: Automatic validation during deserialization
//! - **Migration Hooks**: Custom conversion for old → new format
//! - **Breaking Change Detection**: Identify incompatible changes
//!
//! ## Example
//!
//! ```rust,ignore
//! use oxicode::versioning::{Version, VersionedEncoder, VersionedDecoder};
//!
//! // Encode with version header
//! let data = b"Hello, World!";
//! let version = Version::new(1, 2, 0);
//! let encoded = VersionedEncoder::encode_with_version(data, version)?;
//!
//! // Decode with version checking
//! let (decoded, version) = VersionedDecoder::decode_with_version(&encoded)?;
//! println!("Decoded data from version {}", version);
//! ```

mod compatibility;
mod header;
mod version;

pub use compatibility::{can_migrate, check_compatibility, CompatibilityLevel};

#[cfg(feature = "alloc")]
pub use compatibility::migration_path;
pub use header::{VersionedHeader, VERSIONED_MAGIC};
pub use version::Version;

#[cfg(feature = "alloc")]
extern crate alloc;

#[cfg(feature = "alloc")]
use crate::Error;
use crate::Result;

/// Encode data with a version header.
#[cfg(feature = "alloc")]
pub fn encode_versioned(data: &[u8], version: Version) -> Result<alloc::vec::Vec<u8>> {
    let header = VersionedHeader::new(version);
    let header_bytes = header.to_bytes();

    let mut output = alloc::vec::Vec::with_capacity(header_bytes.len() + data.len());
    output.extend_from_slice(&header_bytes);
    output.extend_from_slice(data);

    Ok(output)
}

/// Decode versioned data, returning the version and payload.
#[cfg(feature = "alloc")]
pub fn decode_versioned(data: &[u8]) -> Result<(alloc::vec::Vec<u8>, Version)> {
    let header = VersionedHeader::from_bytes(data)?;
    let payload_start = header.header_size();

    if data.len() < payload_start {
        return Err(Error::UnexpectedEnd {
            additional: payload_start - data.len(),
        });
    }

    let payload = data[payload_start..].to_vec();
    Ok((payload, header.version()))
}

/// Decode versioned data with compatibility checking.
///
/// Returns an error if the data version is not compatible with the expected version.
#[cfg(feature = "alloc")]
pub fn decode_versioned_with_check(
    data: &[u8],
    expected: Version,
    min_compatible: Option<Version>,
) -> Result<(alloc::vec::Vec<u8>, Version, CompatibilityLevel)> {
    let (payload, version) = decode_versioned(data)?;

    let compat = check_compatibility(version, expected, min_compatible);

    if matches!(compat, CompatibilityLevel::Incompatible) {
        return Err(Error::InvalidData {
            message: "version incompatible",
        });
    }

    Ok((payload, version, compat))
}

/// Check if data appears to be versioned (has valid magic header).
pub fn is_versioned(data: &[u8]) -> bool {
    data.len() >= VERSIONED_MAGIC.len() && data[..VERSIONED_MAGIC.len()] == VERSIONED_MAGIC
}

/// Extract version from versioned data without decoding the payload.
pub fn extract_version(data: &[u8]) -> Result<Version> {
    let header = VersionedHeader::from_bytes(data)?;
    Ok(header.version())
}

#[cfg(test)]
mod tests {
    #[cfg(feature = "alloc")]
    use super::*;

    #[cfg(feature = "alloc")]
    #[test]
    fn test_versioned_roundtrip() {
        let data = b"Hello, World!";
        let version = Version::new(1, 2, 3);

        let encoded = encode_versioned(data, version).expect("encode failed");
        let (decoded, ver) = decode_versioned(&encoded).expect("decode failed");

        assert_eq!(data.as_slice(), decoded.as_slice());
        assert_eq!(version, ver);
    }

    #[cfg(feature = "alloc")]
    #[test]
    fn test_is_versioned() {
        let data = b"Hello, World!";
        let version = Version::new(1, 0, 0);

        // Raw data is not versioned
        assert!(!is_versioned(data));

        // Encoded data is versioned
        let encoded = encode_versioned(data, version).expect("encode failed");
        assert!(is_versioned(&encoded));
    }

    #[cfg(feature = "alloc")]
    #[test]
    fn test_extract_version() {
        let data = b"test";
        let version = Version::new(2, 5, 10);

        let encoded = encode_versioned(data, version).expect("encode failed");
        let extracted = extract_version(&encoded).expect("extract failed");

        assert_eq!(version, extracted);
    }

    #[cfg(feature = "alloc")]
    #[test]
    fn test_compatibility_check() {
        let data = b"test";
        let data_version = Version::new(1, 5, 0);
        let current = Version::new(1, 6, 0);
        let min_compat = Some(Version::new(1, 0, 0));

        let encoded = encode_versioned(data, data_version).expect("encode failed");
        let (_, ver, compat) =
            decode_versioned_with_check(&encoded, current, min_compat).expect("decode failed");

        assert_eq!(ver, data_version);
        assert!(matches!(
            compat,
            CompatibilityLevel::Compatible | CompatibilityLevel::CompatibleWithWarnings
        ));
    }

    #[cfg(feature = "alloc")]
    #[test]
    fn test_incompatible_version() {
        let data = b"test";
        let data_version = Version::new(2, 0, 0); // Major version bump
        let current = Version::new(1, 0, 0);

        let encoded = encode_versioned(data, data_version).expect("encode failed");
        let result = decode_versioned_with_check(&encoded, current, None);

        assert!(result.is_err());
    }
}
