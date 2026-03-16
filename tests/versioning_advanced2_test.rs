//! Advanced versioning tests — second batch (22 tests).
//!
//! Covers migration paths, compatibility level methods, CompatibilityLevel usability,
//! sequential versioned value streams, version tuple/parse/display, patch/minor update
//! predicates, header corruption detection, is_versioned edge cases, min-version
//! threshold enforcement, version satisfies, 0.x strictness, and schema-evolution
//! multi-version decode patterns.
//!
//! All tests are top-level; no `#[cfg(test)]` module wrapper.

#![allow(
    clippy::approx_constant,
    clippy::useless_vec,
    clippy::len_zero,
    clippy::unnecessary_cast,
    clippy::redundant_closure,
    clippy::too_many_arguments,
    clippy::type_complexity,
    clippy::needless_borrow,
    clippy::enum_variant_names,
    clippy::upper_case_acronyms,
    clippy::inconsistent_digit_grouping,
    clippy::unit_cmp,
    clippy::assertions_on_constants,
    clippy::iter_on_single_items,
    clippy::expect_fun_call,
    clippy::redundant_pattern_matching,
    variant_size_differences,
    clippy::absurd_extreme_comparisons,
    clippy::nonminimal_bool,
    clippy::for_kv_map,
    clippy::needless_range_loop,
    clippy::single_match,
    clippy::collapsible_if,
    clippy::needless_return,
    clippy::redundant_clone,
    clippy::map_entry,
    clippy::match_single_binding,
    clippy::bool_comparison,
    clippy::derivable_impls,
    clippy::manual_range_contains,
    clippy::needless_borrows_for_generic_args,
    clippy::manual_map,
    clippy::vec_init_then_push,
    clippy::identity_op,
    clippy::manual_flatten,
    clippy::single_char_pattern,
    clippy::search_is_some,
    clippy::option_map_unit_fn,
    clippy::while_let_on_iterator,
    clippy::clone_on_copy,
    clippy::box_collection,
    clippy::redundant_field_names,
    clippy::ptr_arg,
    clippy::large_enum_variant,
    clippy::match_ref_pats,
    clippy::needless_pass_by_value,
    clippy::unused_unit,
    clippy::let_and_return,
    clippy::suspicious_else_formatting,
    clippy::manual_strip,
    clippy::match_like_matches_macro,
    clippy::from_over_into,
    clippy::wrong_self_convention,
    clippy::inherent_to_string,
    clippy::new_without_default,
    clippy::unnecessary_wraps,
    clippy::field_reassign_with_default,
    clippy::manual_find,
    clippy::unnecessary_lazy_evaluations,
    clippy::should_implement_trait,
    clippy::missing_safety_doc,
    clippy::unusual_byte_groupings,
    clippy::bool_assert_comparison,
    clippy::zero_prefixed_literal,
    clippy::await_holding_lock,
    clippy::manual_saturating_arithmetic,
    clippy::explicit_counter_loop,
    clippy::needless_lifetimes,
    clippy::single_component_path_imports,
    clippy::uninlined_format_args,
    clippy::iter_cloned_collect,
    clippy::manual_str_repeat,
    clippy::excessive_precision,
    clippy::precedence,
    clippy::unnecessary_literal_unwrap
)]
use oxicode::versioning::{
    can_migrate, check_compatibility, decode_versioned_with_check, encode_versioned, is_versioned,
    migration_path, CompatibilityLevel, Version, VersionedHeader, VERSIONED_MAGIC,
};
use oxicode::{decode_versioned_value, encode_versioned_value, Decode, Encode};

// ─────────────────────────────────────────────────────────────────────────────
// Shared types used across multiple tests
// ─────────────────────────────────────────────────────────────────────────────

#[derive(Encode, Decode, PartialEq, Debug, Clone)]
struct Record {
    id: u64,
    name: String,
    score: f32,
}

// ─────────────────────────────────────────────────────────────────────────────
// Test 1 — migration_path returns empty for same version
// ─────────────────────────────────────────────────────────────────────────────

#[test]
fn test_migration_path_same_version_is_empty() {
    let v = Version::new(1, 5, 3);
    let path = migration_path(v, v);
    assert!(
        path.is_empty(),
        "migration path from a version to itself must be empty"
    );
}

// ─────────────────────────────────────────────────────────────────────────────
// Test 2 — migration_path returns empty for same-major forward migration
// ─────────────────────────────────────────────────────────────────────────────

#[test]
fn test_migration_path_same_major_is_empty() {
    let from = Version::new(2, 0, 0);
    let to = Version::new(2, 7, 3);
    let path = migration_path(from, to);
    assert!(
        path.is_empty(),
        "intra-major migration needs no intermediate steps"
    );
}

// ─────────────────────────────────────────────────────────────────────────────
// Test 3 — migration_path for two-major-bump has one intermediate step
// ─────────────────────────────────────────────────────────────────────────────

#[test]
fn test_migration_path_two_major_bump_has_one_intermediate() {
    let from = Version::new(1, 0, 0);
    let to = Version::new(3, 0, 0);
    let path = migration_path(from, to);
    assert_eq!(
        path.len(),
        1,
        "1.x -> 3.x migration must contain exactly one intermediate version"
    );
    assert_eq!(
        path[0],
        Version::new(2, 0, 0),
        "intermediate step must be 2.0.0"
    );
}

// ─────────────────────────────────────────────────────────────────────────────
// Test 4 — migration_path for three-major-bump has two intermediate steps
// ─────────────────────────────────────────────────────────────────────────────

#[test]
fn test_migration_path_three_major_bump_has_two_intermediates() {
    let from = Version::new(1, 0, 0);
    let to = Version::new(4, 0, 0);
    let path = migration_path(from, to);
    assert_eq!(
        path.len(),
        2,
        "1.x -> 4.x migration must contain two intermediate versions"
    );
    assert_eq!(path[0], Version::new(2, 0, 0));
    assert_eq!(path[1], Version::new(3, 0, 0));
}

// ─────────────────────────────────────────────────────────────────────────────
// Test 5 — can_migrate returns true for forward minor migration
// ─────────────────────────────────────────────────────────────────────────────

#[test]
fn test_can_migrate_forward_minor() {
    let from = Version::new(1, 0, 0);
    let to = Version::new(1, 9, 0);
    assert!(
        can_migrate(from, to),
        "forward minor-version migration must be allowed"
    );
}

// ─────────────────────────────────────────────────────────────────────────────
// Test 6 — can_migrate returns false for backward migration
// ─────────────────────────────────────────────────────────────────────────────

#[test]
fn test_can_migrate_backward_is_false() {
    let from = Version::new(3, 0, 0);
    let to = Version::new(1, 0, 0);
    assert!(
        !can_migrate(from, to),
        "backward migration (newer to older) must not be allowed"
    );
}

// ─────────────────────────────────────────────────────────────────────────────
// Test 7 — CompatibilityLevel::is_usable covers Compatible and CompatibleWithWarnings
// ─────────────────────────────────────────────────────────────────────────────

#[test]
fn test_compatibility_level_is_usable() {
    assert!(
        CompatibilityLevel::Compatible.is_usable(),
        "Compatible must be usable"
    );
    assert!(
        CompatibilityLevel::CompatibleWithWarnings.is_usable(),
        "CompatibleWithWarnings must be usable"
    );
    assert!(
        !CompatibilityLevel::Incompatible.is_usable(),
        "Incompatible must not be usable"
    );
}

// ─────────────────────────────────────────────────────────────────────────────
// Test 8 — CompatibilityLevel::is_fully_compatible and has_warnings
// ─────────────────────────────────────────────────────────────────────────────

#[test]
fn test_compatibility_level_predicates() {
    assert!(CompatibilityLevel::Compatible.is_fully_compatible());
    assert!(!CompatibilityLevel::CompatibleWithWarnings.is_fully_compatible());
    assert!(!CompatibilityLevel::Incompatible.is_fully_compatible());

    assert!(!CompatibilityLevel::Compatible.has_warnings());
    assert!(CompatibilityLevel::CompatibleWithWarnings.has_warnings());
    assert!(!CompatibilityLevel::Incompatible.has_warnings());
}

// ─────────────────────────────────────────────────────────────────────────────
// Test 9 — check_compatibility: same version yields Compatible
// ─────────────────────────────────────────────────────────────────────────────

#[test]
fn test_check_compatibility_same_version_is_compatible() {
    let v = Version::new(2, 3, 4);
    let level = check_compatibility(v, v, None);
    assert_eq!(
        level,
        CompatibilityLevel::Compatible,
        "same version must be fully compatible"
    );
}

// ─────────────────────────────────────────────────────────────────────────────
// Test 10 — check_compatibility: data older minor gives CompatibleWithWarnings
// ─────────────────────────────────────────────────────────────────────────────

#[test]
fn test_check_compatibility_older_minor_gives_warnings() {
    let data_ver = Version::new(1, 0, 0);
    let current = Version::new(1, 3, 0);
    let level = check_compatibility(data_ver, current, None);
    assert_eq!(
        level,
        CompatibilityLevel::CompatibleWithWarnings,
        "older minor version must yield CompatibleWithWarnings"
    );
}

// ─────────────────────────────────────────────────────────────────────────────
// Test 11 — check_compatibility: data newer than current gives CompatibleWithWarnings
// ─────────────────────────────────────────────────────────────────────────────

#[test]
fn test_check_compatibility_newer_data_gives_warnings() {
    let data_ver = Version::new(1, 5, 0);
    let current = Version::new(1, 2, 0);
    let level = check_compatibility(data_ver, current, None);
    assert_eq!(
        level,
        CompatibilityLevel::CompatibleWithWarnings,
        "data from a newer minor version must yield CompatibleWithWarnings"
    );
}

// ─────────────────────────────────────────────────────────────────────────────
// Test 12 — check_compatibility: below minimum required version is Incompatible
// ─────────────────────────────────────────────────────────────────────────────

#[test]
fn test_check_compatibility_below_min_is_incompatible() {
    let data_ver = Version::new(1, 1, 0);
    let current = Version::new(1, 5, 0);
    let min = Some(Version::new(1, 3, 0));
    let level = check_compatibility(data_ver, current, min);
    assert_eq!(
        level,
        CompatibilityLevel::Incompatible,
        "data below minimum required version must be Incompatible"
    );
}

// ─────────────────────────────────────────────────────────────────────────────
// Test 13 — 0.x minor mismatch is Incompatible
// ─────────────────────────────────────────────────────────────────────────────

#[test]
fn test_check_compatibility_0x_minor_mismatch_is_incompatible() {
    let data_ver = Version::new(0, 1, 5);
    let current = Version::new(0, 2, 0);
    let level = check_compatibility(data_ver, current, None);
    assert_eq!(
        level,
        CompatibilityLevel::Incompatible,
        "0.x minor mismatch must be Incompatible"
    );
}

// ─────────────────────────────────────────────────────────────────────────────
// Test 14 — 0.x same minor different patch is Compatible
// ─────────────────────────────────────────────────────────────────────────────

#[test]
fn test_check_compatibility_0x_same_minor_different_patch_is_compatible() {
    let data_ver = Version::new(0, 3, 0);
    let current = Version::new(0, 3, 7);
    let level = check_compatibility(data_ver, current, None);
    assert_eq!(
        level,
        CompatibilityLevel::Compatible,
        "0.x same minor, different patch must be Compatible"
    );
}

// ─────────────────────────────────────────────────────────────────────────────
// Test 15 — Version::satisfies checks minimum requirement correctly
// ─────────────────────────────────────────────────────────────────────────────

#[test]
fn test_version_satisfies_minimum() {
    let v = Version::new(2, 5, 0);
    let min_ok = Version::new(2, 0, 0);
    let min_fail = Version::new(2, 6, 0);

    assert!(v.satisfies(&min_ok), "2.5.0 must satisfy minimum 2.0.0");
    assert!(
        !v.satisfies(&min_fail),
        "2.5.0 must not satisfy minimum 2.6.0"
    );
}

// ─────────────────────────────────────────────────────────────────────────────
// Test 16 — Version::tuple returns the correct (major, minor, patch) tuple
// ─────────────────────────────────────────────────────────────────────────────

#[test]
fn test_version_tuple_extraction() {
    let v = Version::new(7, 13, 42);
    assert_eq!(
        v.tuple(),
        (7, 13, 42),
        "tuple() must return the correct (major, minor, patch)"
    );
}

// ─────────────────────────────────────────────────────────────────────────────
// Test 17 — Version::parse then Display roundtrip
// ─────────────────────────────────────────────────────────────────────────────

#[test]
fn test_version_parse_then_display_roundtrip() {
    let original_str = "3.14.159";
    let parsed = Version::parse(original_str).expect("Version::parse must succeed for '3.14.159'");
    let displayed = format!("{}", parsed);
    assert_eq!(
        displayed, original_str,
        "Display output must match original parse input"
    );
}

// ─────────────────────────────────────────────────────────────────────────────
// Test 18 — Version::is_patch_update_from and is_minor_update_from
// ─────────────────────────────────────────────────────────────────────────────

#[test]
fn test_version_update_type_predicates() {
    let base = Version::new(1, 2, 0);
    let patch = Version::new(1, 2, 5);
    let minor = Version::new(1, 4, 0);
    let major = Version::new(2, 0, 0);

    assert!(
        patch.is_patch_update_from(&base),
        "1.2.5 must be a patch update of 1.2.0"
    );
    assert!(
        !minor.is_patch_update_from(&base),
        "1.4.0 must not be a patch update of 1.2.0"
    );
    assert!(
        minor.is_minor_update_from(&base),
        "1.4.0 must be a minor update of 1.2.0"
    );
    assert!(
        !major.is_minor_update_from(&base),
        "2.0.0 must not be a minor update of 1.2.0"
    );
}

// ─────────────────────────────────────────────────────────────────────────────
// Test 19 — VersionedHeader: to_bytes / from_bytes roundtrip preserves all fields
// ─────────────────────────────────────────────────────────────────────────────

#[test]
fn test_versioned_header_roundtrip_preserves_fields() {
    let version = Version::new(5, 11, 99);
    let header = VersionedHeader::new(version);
    let bytes = header.to_bytes();

    let recovered =
        VersionedHeader::from_bytes(&bytes).expect("VersionedHeader::from_bytes must succeed");

    assert_eq!(
        recovered.version(),
        version,
        "recovered version must match original"
    );
    assert_eq!(
        recovered.header_version(),
        1u8,
        "header format version must be 1"
    );
    assert_eq!(
        recovered.header_size(),
        11,
        "header size must be exactly 11 bytes"
    );
}

// ─────────────────────────────────────────────────────────────────────────────
// Test 20 — is_versioned returns false for unversioned and truncated data
// ─────────────────────────────────────────────────────────────────────────────

#[test]
fn test_is_versioned_false_for_unversioned_and_truncated_data() {
    assert!(
        !is_versioned(b"raw data without header"),
        "raw bytes must not appear versioned"
    );
    assert!(!is_versioned(b""), "empty slice must not appear versioned");

    // Partial magic (first 3 bytes only)
    let partial = &VERSIONED_MAGIC[..3];
    assert!(
        !is_versioned(partial),
        "partial magic bytes must not appear versioned"
    );
}

// ─────────────────────────────────────────────────────────────────────────────
// Test 21 — sequential versioned values are independently decodable
// ─────────────────────────────────────────────────────────────────────────────

#[test]
fn test_sequential_versioned_values_are_independent() {
    let version_a = Version::new(1, 0, 0);
    let version_b = Version::new(2, 0, 0);

    let record_a = Record {
        id: 1,
        name: String::from("Alice"),
        score: 9.5,
    };
    let record_b = Record {
        id: 2,
        name: String::from("Bob"),
        score: 7.3,
    };

    let enc_a = encode_versioned_value(&record_a, version_a).expect("encode record_a failed");
    let enc_b = encode_versioned_value(&record_b, version_b).expect("encode record_b failed");

    let (dec_a, ver_a, _): (Record, _, _) =
        decode_versioned_value(&enc_a).expect("decode record_a failed");
    let (dec_b, ver_b, _): (Record, _, _) =
        decode_versioned_value(&enc_b).expect("decode record_b failed");

    assert_eq!(dec_a, record_a, "record_a must roundtrip correctly");
    assert_eq!(ver_a, version_a, "version_a must be preserved");
    assert_eq!(dec_b, record_b, "record_b must roundtrip correctly");
    assert_eq!(ver_b, version_b, "version_b must be preserved");
}

// ─────────────────────────────────────────────────────────────────────────────
// Test 22 — versioned decode with check: compatible-with-warnings does not fail
// ─────────────────────────────────────────────────────────────────────────────

#[test]
fn test_versioned_decode_with_check_compatible_with_warnings_succeeds() {
    // Encode data under v1.0.0; decode expecting v1.3.0 with min 1.0.0.
    // This must succeed because same major, data is older-minor → CompatibleWithWarnings.
    let data = b"schema evolves gracefully";
    let data_version = Version::new(1, 0, 0);
    let current_version = Version::new(1, 3, 0);
    let min_compat = Some(Version::new(1, 0, 0));

    let encoded = encode_versioned(data, data_version)
        .expect("encode_versioned for compat-with-warnings test failed");

    let (payload, ver, compat) = decode_versioned_with_check(&encoded, current_version, min_compat)
        .expect("decode_versioned_with_check must succeed for CompatibleWithWarnings");

    assert_eq!(
        payload.as_slice(),
        data,
        "payload must be recovered unchanged"
    );
    assert_eq!(
        ver, data_version,
        "version embedded in header must be returned"
    );
    assert_eq!(
        compat,
        CompatibilityLevel::CompatibleWithWarnings,
        "compatibility level must be CompatibleWithWarnings"
    );
    assert!(compat.is_usable(), "CompatibleWithWarnings must be usable");
    assert!(
        !compat.is_fully_compatible(),
        "CompatibleWithWarnings must not be fully compatible"
    );
}
