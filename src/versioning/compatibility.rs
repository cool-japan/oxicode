//! Version compatibility checking.

use super::version::Version;

/// Level of compatibility between versions.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CompatibilityLevel {
    /// Fully compatible - no changes needed
    Compatible,

    /// Compatible but with warnings (minor version difference)
    CompatibleWithWarnings,

    /// Incompatible - cannot be used together
    Incompatible,
}

impl CompatibilityLevel {
    /// Returns true if this level allows use.
    #[inline]
    pub const fn is_usable(&self) -> bool {
        !matches!(self, CompatibilityLevel::Incompatible)
    }

    /// Returns true if this level is fully compatible.
    #[inline]
    pub const fn is_fully_compatible(&self) -> bool {
        matches!(self, CompatibilityLevel::Compatible)
    }

    /// Returns true if there are warnings.
    #[inline]
    pub const fn has_warnings(&self) -> bool {
        matches!(self, CompatibilityLevel::CompatibleWithWarnings)
    }
}

/// Check compatibility between a data version and expected version.
///
/// # Arguments
///
/// * `data_version` - The version of the data being read
/// * `current_version` - The current expected version
/// * `min_compatible` - Optional minimum compatible version
///
/// # Returns
///
/// The compatibility level between the versions.
pub fn check_compatibility(
    data_version: Version,
    current_version: Version,
    min_compatible: Option<Version>,
) -> CompatibilityLevel {
    // Check minimum version if specified
    if let Some(min) = min_compatible {
        if data_version < min {
            return CompatibilityLevel::Incompatible;
        }
    }

    // Pre-1.0 versions have stricter compatibility
    if current_version.major == 0 && data_version.major == 0 {
        // In 0.x, minor version must match
        if data_version.minor != current_version.minor {
            return CompatibilityLevel::Incompatible;
        }
        // Patch differences are allowed
        return CompatibilityLevel::Compatible;
    }

    // Major version must match for compatibility
    if data_version.major != current_version.major {
        return CompatibilityLevel::Incompatible;
    }

    // If data version is newer than current, might have issues
    if data_version > current_version {
        return CompatibilityLevel::CompatibleWithWarnings;
    }

    // Data version is older or same, should be compatible
    if data_version.minor < current_version.minor {
        // Older minor version - compatible with warnings
        return CompatibilityLevel::CompatibleWithWarnings;
    }

    CompatibilityLevel::Compatible
}

/// Check if a data version can be migrated to a target version.
///
/// Returns true if migration is possible (not necessarily automatic).
pub fn can_migrate(from: Version, to: Version) -> bool {
    // Can always migrate within same major version
    if from.major == to.major {
        return true;
    }

    // Can migrate forward through breaking changes (with explicit handling)
    from < to
}

/// Determine the migration path between versions.
///
/// Returns a list of intermediate versions that should be migrated through.
/// Empty list means direct migration is possible.
#[cfg(feature = "alloc")]
pub fn migration_path(from: Version, to: Version) -> alloc::vec::Vec<Version> {
    let mut path = alloc::vec::Vec::new();

    if from == to {
        return path;
    }

    // If major versions differ, we need intermediate major version bumps
    let mut current = from;
    while current.major < to.major {
        // Add the next major version as a migration step
        current = Version::new(current.major + 1, 0, 0);
        if current != to {
            path.push(current);
        }
    }

    path
}

#[cfg(feature = "alloc")]
extern crate alloc;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_compatible_same_version() {
        let v = Version::new(1, 0, 0);
        let compat = check_compatibility(v, v, None);
        assert_eq!(compat, CompatibilityLevel::Compatible);
    }

    #[test]
    fn test_compatible_patch_difference() {
        let data = Version::new(1, 0, 0);
        let current = Version::new(1, 0, 5);
        let compat = check_compatibility(data, current, None);
        assert_eq!(compat, CompatibilityLevel::Compatible);
    }

    #[test]
    fn test_compatible_with_warnings_minor() {
        let data = Version::new(1, 0, 0);
        let current = Version::new(1, 5, 0);
        let compat = check_compatibility(data, current, None);
        assert_eq!(compat, CompatibilityLevel::CompatibleWithWarnings);
    }

    #[test]
    fn test_compatible_with_warnings_newer() {
        let data = Version::new(1, 5, 0);
        let current = Version::new(1, 0, 0);
        let compat = check_compatibility(data, current, None);
        assert_eq!(compat, CompatibilityLevel::CompatibleWithWarnings);
    }

    #[test]
    fn test_incompatible_major() {
        let data = Version::new(1, 0, 0);
        let current = Version::new(2, 0, 0);
        let compat = check_compatibility(data, current, None);
        assert_eq!(compat, CompatibilityLevel::Incompatible);
    }

    #[test]
    fn test_incompatible_below_minimum() {
        let data = Version::new(1, 0, 0);
        let current = Version::new(1, 5, 0);
        let min = Some(Version::new(1, 2, 0));
        let compat = check_compatibility(data, current, min);
        assert_eq!(compat, CompatibilityLevel::Incompatible);
    }

    #[test]
    fn test_0x_compatibility() {
        // Same minor in 0.x is compatible
        let data = Version::new(0, 1, 0);
        let current = Version::new(0, 1, 5);
        assert_eq!(
            check_compatibility(data, current, None),
            CompatibilityLevel::Compatible
        );

        // Different minor in 0.x is incompatible
        let current2 = Version::new(0, 2, 0);
        assert_eq!(
            check_compatibility(data, current2, None),
            CompatibilityLevel::Incompatible
        );
    }

    #[test]
    fn test_can_migrate() {
        // Same major can migrate
        assert!(can_migrate(Version::new(1, 0, 0), Version::new(1, 5, 0)));

        // Forward through major bump can migrate
        assert!(can_migrate(Version::new(1, 0, 0), Version::new(2, 0, 0)));

        // Backward cannot migrate
        assert!(!can_migrate(Version::new(2, 0, 0), Version::new(1, 0, 0)));
    }

    #[cfg(feature = "alloc")]
    #[test]
    fn test_migration_path() {
        // Same version - empty path
        let path = migration_path(Version::new(1, 0, 0), Version::new(1, 0, 0));
        assert!(path.is_empty());

        // Same major - empty path
        let path = migration_path(Version::new(1, 0, 0), Version::new(1, 5, 0));
        assert!(path.is_empty());

        // One major bump - empty path (direct)
        let path = migration_path(Version::new(1, 0, 0), Version::new(2, 0, 0));
        assert!(path.is_empty());

        // Two major bumps - one intermediate
        let path = migration_path(Version::new(1, 0, 0), Version::new(3, 0, 0));
        assert_eq!(path.len(), 1);
        assert_eq!(path[0], Version::new(2, 0, 0));
    }

    #[test]
    fn test_compatibility_level_methods() {
        assert!(CompatibilityLevel::Compatible.is_usable());
        assert!(CompatibilityLevel::CompatibleWithWarnings.is_usable());
        assert!(!CompatibilityLevel::Incompatible.is_usable());

        assert!(CompatibilityLevel::Compatible.is_fully_compatible());
        assert!(!CompatibilityLevel::CompatibleWithWarnings.is_fully_compatible());

        assert!(!CompatibilityLevel::Compatible.has_warnings());
        assert!(CompatibilityLevel::CompatibleWithWarnings.has_warnings());
    }
}
