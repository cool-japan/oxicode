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
//! ```rust
//! use oxicode::versioning::{Version, VersionedEncoder, VersionedDecoder};
//!
//! // Encode with version header
//! let data = b"Hello, World!";
//! let version = Version::new(1, 2, 0);
//! let encoded = VersionedEncoder::encode_with_version(data, version).expect("encode failed");
//!
//! // Decode with version checking
//! let (decoded, version) = VersionedDecoder::decode_with_version(&encoded).expect("decode failed");
//! assert_eq!(decoded, data);
//! println!("Decoded data from version {}", version);
//! ```
//!
//! `VersionedEncoder` and `VersionedDecoder` are thin, stateful wrappers over
//! the [`encode_versioned`] / [`decode_versioned`] (and
//! [`decode_versioned_with_check`]) free functions, for callers who prefer to
//! configure a version (and, for decoding, compatibility requirements) once
//! and reuse it across multiple calls:
//!
//! ```rust
//! use oxicode::versioning::{Version, VersionedEncoder, VersionedDecoder};
//!
//! let encoder = VersionedEncoder::new(Version::new(1, 2, 0));
//! let encoded = encoder.encode(b"payload").expect("encode failed");
//!
//! let decoder = VersionedDecoder::new()
//!     .expect_version(Version::new(1, 0, 0))
//!     .min_compatible(Version::new(1, 0, 0));
//! let (decoded, version, compat) = decoder.decode(&encoded).expect("decode failed");
//! assert_eq!(decoded, b"payload");
//! assert_eq!(version, Version::new(1, 2, 0));
//! assert!(compat.is_usable());
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

/// A stateful, type-based wrapper for encoding data with a version header.
///
/// This is a thin convenience wrapper over [`encode_versioned`], useful when
/// a version is configured once and reused across multiple encode calls.
#[cfg(feature = "alloc")]
#[derive(Debug, Clone, Copy)]
pub struct VersionedEncoder {
    version: Version,
}

#[cfg(feature = "alloc")]
impl VersionedEncoder {
    /// Create a new encoder that stamps encoded data with `version`.
    #[inline]
    pub const fn new(version: Version) -> Self {
        Self { version }
    }

    /// The version this encoder stamps onto encoded data.
    #[inline]
    pub const fn version(&self) -> Version {
        self.version
    }

    /// Encode `data` with this encoder's version header.
    ///
    /// Equivalent to `encode_versioned(data, self.version())`.
    pub fn encode(&self, data: &[u8]) -> Result<alloc::vec::Vec<u8>> {
        encode_versioned(data, self.version)
    }

    /// Encode `data` with a version header in one call, without
    /// constructing a [`VersionedEncoder`] first.
    ///
    /// Equivalent to `encode_versioned(data, version)`.
    pub fn encode_with_version(data: &[u8], version: Version) -> Result<alloc::vec::Vec<u8>> {
        encode_versioned(data, version)
    }
}

/// A stateful, type-based wrapper for decoding versioned data, optionally
/// enforcing a compatibility requirement.
///
/// This is a thin convenience wrapper over [`decode_versioned`] and
/// [`decode_versioned_with_check`], useful when expected/minimum versions
/// are configured once and reused across multiple decode calls.
#[cfg(feature = "alloc")]
#[derive(Debug, Clone, Copy, Default)]
pub struct VersionedDecoder {
    expected: Option<Version>,
    min_compatible: Option<Version>,
}

#[cfg(feature = "alloc")]
impl VersionedDecoder {
    /// Create a decoder with no compatibility requirements: any
    /// well-formed versioned payload decodes successfully regardless of its
    /// embedded version.
    #[inline]
    pub const fn new() -> Self {
        Self {
            expected: None,
            min_compatible: None,
        }
    }

    /// Require decoded data to be compatible with `expected` (checked via
    /// [`check_compatibility`]). Once set, [`VersionedDecoder::decode`]
    /// rejects data whose version is [`CompatibilityLevel::Incompatible`]
    /// with `expected`.
    #[inline]
    pub const fn expect_version(mut self, expected: Version) -> Self {
        self.expected = Some(expected);
        self
    }

    /// Additionally require the decoded data's version to be at least
    /// `min_compatible`. Only consulted once
    /// [`VersionedDecoder::expect_version`] has also been set.
    #[inline]
    pub const fn min_compatible(mut self, min_compatible: Version) -> Self {
        self.min_compatible = Some(min_compatible);
        self
    }

    /// Decode versioned `data`, applying this decoder's compatibility
    /// requirements (if any).
    ///
    /// Returns the payload, its version, and the resulting
    /// [`CompatibilityLevel`]. When no `expected` version has been
    /// configured (the default), compatibility is not checked and the level
    /// is always reported as [`CompatibilityLevel::Compatible`].
    pub fn decode(
        &self,
        data: &[u8],
    ) -> Result<(alloc::vec::Vec<u8>, Version, CompatibilityLevel)> {
        match self.expected {
            Some(expected) => decode_versioned_with_check(data, expected, self.min_compatible),
            None => {
                let (payload, version) = decode_versioned(data)?;
                Ok((payload, version, CompatibilityLevel::Compatible))
            }
        }
    }

    /// Decode versioned `data` in one call, without constructing a
    /// [`VersionedDecoder`] first and without any compatibility requirement.
    ///
    /// Equivalent to `decode_versioned(data)`.
    pub fn decode_with_version(data: &[u8]) -> Result<(alloc::vec::Vec<u8>, Version)> {
        decode_versioned(data)
    }
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
