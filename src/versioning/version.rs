//! Semantic version representation.

use core::cmp::Ordering;
use core::fmt;

/// A semantic version (major.minor.patch).
///
/// Follows the semver 2.0 specification for version comparison:
/// - Major version changes indicate breaking changes
/// - Minor version changes add functionality in a backward-compatible manner
/// - Patch version changes are backward-compatible bug fixes
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct Version {
    /// Major version - incompatible API changes
    pub major: u16,
    /// Minor version - backward-compatible functionality
    pub minor: u16,
    /// Patch version - backward-compatible bug fixes
    pub patch: u16,
}

impl Version {
    /// Create a new version.
    #[inline]
    pub const fn new(major: u16, minor: u16, patch: u16) -> Self {
        Self {
            major,
            minor,
            patch,
        }
    }

    /// Create version 0.0.0.
    #[inline]
    pub const fn zero() -> Self {
        Self::new(0, 0, 0)
    }

    /// Parse a version from "major.minor.patch" string format.
    ///
    /// Returns None if parsing fails.
    pub fn parse(s: &str) -> Option<Self> {
        let mut parts = s.split('.');
        let major: u16 = parts.next()?.parse().ok()?;
        let minor: u16 = parts.next()?.parse().ok()?;
        let patch: u16 = parts.next()?.parse().ok()?;

        // Ensure no extra parts
        if parts.next().is_some() {
            return None;
        }

        Some(Self::new(major, minor, patch))
    }

    /// Check if this version is compatible with another version.
    ///
    /// By semver rules:
    /// - Same major version (if major > 0): compatible
    /// - Major version 0: only exact minor match is compatible
    #[inline]
    pub fn is_compatible_with(&self, other: &Self) -> bool {
        if self.major == 0 && other.major == 0 {
            // Pre-1.0: minor versions may break compatibility
            self.minor == other.minor
        } else {
            // Post-1.0: same major version is compatible
            self.major == other.major
        }
    }

    /// Check if this version represents a breaking change from another.
    #[inline]
    pub fn is_breaking_change_from(&self, other: &Self) -> bool {
        if self.major == 0 && other.major == 0 {
            // Pre-1.0: any minor bump could be breaking
            self.minor != other.minor
        } else {
            // Post-1.0: major bump is breaking
            self.major != other.major
        }
    }

    /// Check if this version is a minor update from another.
    #[inline]
    pub fn is_minor_update_from(&self, other: &Self) -> bool {
        self.major == other.major && self.minor > other.minor
    }

    /// Check if this version is a patch update from another.
    #[inline]
    pub fn is_patch_update_from(&self, other: &Self) -> bool {
        self.major == other.major && self.minor == other.minor && self.patch > other.patch
    }

    /// Check if this version satisfies a minimum requirement.
    #[inline]
    pub fn satisfies(&self, minimum: &Self) -> bool {
        *self >= *minimum
    }

    /// Convert to bytes for encoding (6 bytes: 2 each for major, minor, patch).
    #[inline]
    pub fn to_bytes(self) -> [u8; 6] {
        let mut bytes = [0u8; 6];
        bytes[0..2].copy_from_slice(&self.major.to_le_bytes());
        bytes[2..4].copy_from_slice(&self.minor.to_le_bytes());
        bytes[4..6].copy_from_slice(&self.patch.to_le_bytes());
        bytes
    }

    /// Parse from bytes.
    pub fn from_bytes(bytes: &[u8]) -> Option<Self> {
        if bytes.len() < 6 {
            return None;
        }

        let major = u16::from_le_bytes([bytes[0], bytes[1]]);
        let minor = u16::from_le_bytes([bytes[2], bytes[3]]);
        let patch = u16::from_le_bytes([bytes[4], bytes[5]]);

        Some(Self::new(major, minor, patch))
    }

    /// Get the version tuple.
    #[inline]
    pub const fn tuple(self) -> (u16, u16, u16) {
        (self.major, self.minor, self.patch)
    }
}

impl Ord for Version {
    fn cmp(&self, other: &Self) -> Ordering {
        match self.major.cmp(&other.major) {
            Ordering::Equal => match self.minor.cmp(&other.minor) {
                Ordering::Equal => self.patch.cmp(&other.patch),
                ord => ord,
            },
            ord => ord,
        }
    }
}

impl PartialOrd for Version {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl fmt::Display for Version {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}.{}.{}", self.major, self.minor, self.patch)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[cfg(feature = "alloc")]
    use alloc::format;

    #[test]
    fn test_version_new() {
        let v = Version::new(1, 2, 3);
        assert_eq!(v.major, 1);
        assert_eq!(v.minor, 2);
        assert_eq!(v.patch, 3);
    }

    #[test]
    fn test_version_parse() {
        assert_eq!(Version::parse("1.2.3"), Some(Version::new(1, 2, 3)));
        assert_eq!(Version::parse("0.0.0"), Some(Version::new(0, 0, 0)));
        assert_eq!(
            Version::parse("100.200.300"),
            Some(Version::new(100, 200, 300))
        );

        // Invalid formats
        assert_eq!(Version::parse("1.2"), None);
        assert_eq!(Version::parse("1.2.3.4"), None);
        assert_eq!(Version::parse("a.b.c"), None);
        assert_eq!(Version::parse(""), None);
    }

    #[cfg(feature = "alloc")]
    #[test]
    fn test_version_display() {
        let v = Version::new(1, 2, 3);
        assert_eq!(format!("{}", v), "1.2.3");
    }

    #[test]
    fn test_version_ordering() {
        let v1 = Version::new(1, 0, 0);
        let v2 = Version::new(1, 1, 0);
        let v3 = Version::new(1, 1, 1);
        let v4 = Version::new(2, 0, 0);

        assert!(v1 < v2);
        assert!(v2 < v3);
        assert!(v3 < v4);
        assert!(v1 < v4);
    }

    #[test]
    fn test_version_compatibility() {
        let v1 = Version::new(1, 0, 0);
        let v2 = Version::new(1, 5, 0);
        let v3 = Version::new(2, 0, 0);

        // Same major version is compatible
        assert!(v1.is_compatible_with(&v2));
        assert!(v2.is_compatible_with(&v1));

        // Different major version is not compatible
        assert!(!v1.is_compatible_with(&v3));
        assert!(!v3.is_compatible_with(&v1));
    }

    #[test]
    fn test_version_0x_compatibility() {
        // Pre-1.0 versions are stricter
        let v1 = Version::new(0, 1, 0);
        let v2 = Version::new(0, 1, 5);
        let v3 = Version::new(0, 2, 0);

        // Same minor in 0.x is compatible
        assert!(v1.is_compatible_with(&v2));

        // Different minor in 0.x is not compatible
        assert!(!v1.is_compatible_with(&v3));
    }

    #[test]
    fn test_version_bytes() {
        let v = Version::new(1, 2, 3);
        let bytes = v.to_bytes();
        let v2 = Version::from_bytes(&bytes).expect("parse failed");
        assert_eq!(v, v2);
    }

    #[test]
    fn test_breaking_change() {
        let v1 = Version::new(1, 0, 0);
        let v2 = Version::new(2, 0, 0);

        assert!(v2.is_breaking_change_from(&v1));
        assert!(!v1.is_breaking_change_from(&v1));
    }

    #[test]
    fn test_minor_update() {
        let v1 = Version::new(1, 0, 0);
        let v2 = Version::new(1, 1, 0);
        let v3 = Version::new(2, 0, 0);

        assert!(v2.is_minor_update_from(&v1));
        assert!(!v1.is_minor_update_from(&v1));
        assert!(!v3.is_minor_update_from(&v1));
    }

    #[test]
    fn test_patch_update() {
        let v1 = Version::new(1, 0, 0);
        let v2 = Version::new(1, 0, 1);

        assert!(v2.is_patch_update_from(&v1));
        assert!(!v1.is_patch_update_from(&v1));
    }

    #[test]
    fn test_satisfies() {
        let v = Version::new(1, 5, 0);
        let min = Version::new(1, 0, 0);

        assert!(v.satisfies(&min));
        assert!(!min.satisfies(&v));
    }
}
