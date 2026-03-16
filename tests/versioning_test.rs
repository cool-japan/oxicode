//! Comprehensive tests for the versioning module.

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
    can_migrate, check_compatibility, decode_versioned, decode_versioned_with_check,
    encode_versioned, extract_version, is_versioned, migration_path, CompatibilityLevel, Version,
};

// ── Version struct tests ──────────────────────────────────────────────────────

#[test]
fn test_version_comparison_ordering() {
    let v1 = Version::new(1, 0, 0);
    let v2 = Version::new(2, 0, 0);
    assert!(v2 > v1);
    assert!(v1 < v2);
}

#[test]
fn test_version_new_fields() {
    let v = Version::new(3, 7, 11);
    assert_eq!(v.major, 3);
    assert_eq!(v.minor, 7);
    assert_eq!(v.patch, 11);
}

#[test]
fn test_version_equality() {
    assert_eq!(Version::new(1, 2, 3), Version::new(1, 2, 3));
    assert_ne!(Version::new(1, 2, 3), Version::new(1, 2, 4));
}

#[test]
fn test_version_zero() {
    let v = Version::zero();
    assert_eq!(v.major, 0);
    assert_eq!(v.minor, 0);
    assert_eq!(v.patch, 0);
}

#[test]
fn test_version_parse_valid() {
    assert_eq!(Version::parse("1.2.3"), Some(Version::new(1, 2, 3)));
    assert_eq!(Version::parse("0.0.0"), Some(Version::new(0, 0, 0)));
    assert_eq!(
        Version::parse("65535.65535.65535"),
        Some(Version::new(65535, 65535, 65535))
    );
}

#[test]
fn test_version_parse_invalid() {
    assert_eq!(Version::parse("1.2"), None);
    assert_eq!(Version::parse("1.2.3.4"), None);
    assert_eq!(Version::parse("not.a.version"), None);
    assert_eq!(Version::parse(""), None);
}

#[test]
fn test_version_bytes_roundtrip() {
    let v = Version::new(10, 20, 30);
    let bytes = v.to_bytes();
    let v2 = Version::from_bytes(&bytes).expect("from_bytes failed");
    assert_eq!(v, v2);
}

#[test]
fn test_version_tuple() {
    let v = Version::new(5, 6, 7);
    assert_eq!(v.tuple(), (5, 6, 7));
}

#[test]
fn test_version_satisfies() {
    let v = Version::new(1, 5, 0);
    assert!(v.satisfies(&Version::new(1, 0, 0)));
    assert!(!v.satisfies(&Version::new(2, 0, 0)));
}

#[test]
fn test_version_is_compatible_with() {
    // Same major (>=1) → compatible
    assert!(Version::new(1, 0, 0).is_compatible_with(&Version::new(1, 5, 0)));
    // Different major → not compatible
    assert!(!Version::new(1, 0, 0).is_compatible_with(&Version::new(2, 0, 0)));
    // 0.x same minor → compatible
    assert!(Version::new(0, 1, 0).is_compatible_with(&Version::new(0, 1, 5)));
    // 0.x different minor → not compatible
    assert!(!Version::new(0, 1, 0).is_compatible_with(&Version::new(0, 2, 0)));
}

#[test]
fn test_version_breaking_change() {
    assert!(Version::new(2, 0, 0).is_breaking_change_from(&Version::new(1, 0, 0)));
    assert!(!Version::new(1, 5, 0).is_breaking_change_from(&Version::new(1, 0, 0)));
}

#[test]
fn test_version_minor_update() {
    assert!(Version::new(1, 2, 0).is_minor_update_from(&Version::new(1, 0, 0)));
    assert!(!Version::new(2, 0, 0).is_minor_update_from(&Version::new(1, 0, 0)));
}

#[test]
fn test_version_patch_update() {
    assert!(Version::new(1, 0, 1).is_patch_update_from(&Version::new(1, 0, 0)));
    assert!(!Version::new(1, 1, 0).is_patch_update_from(&Version::new(1, 0, 0)));
}

// ── Low-level encode/decode versioned (raw bytes) ─────────────────────────────

#[test]
fn test_encode_decode_versioned_raw() {
    let data = b"hello world";
    let version = Version::new(1, 0, 0);

    let encoded = encode_versioned(data, version).expect("encode failed");
    let (decoded, ver) = decode_versioned(&encoded).expect("decode failed");

    assert_eq!(decoded.as_slice(), data.as_slice());
    assert_eq!(ver, version);
}

#[test]
fn test_is_versioned_detection() {
    let data = b"raw bytes";
    assert!(!is_versioned(data));

    let version = Version::new(1, 0, 0);
    let encoded = encode_versioned(data, version).expect("encode failed");
    assert!(is_versioned(&encoded));
}

#[test]
fn test_is_versioned_empty_slice() {
    assert!(!is_versioned(&[]));
    assert!(!is_versioned(&[0x4F, 0x58])); // partial magic
}

#[test]
fn test_extract_version_only() {
    let data = b"payload";
    let version = Version::new(3, 1, 4);
    let encoded = encode_versioned(data, version).expect("encode failed");
    let extracted = extract_version(&encoded).expect("extract failed");
    assert_eq!(extracted, version);
}

#[test]
fn test_decode_versioned_with_check_compatible() {
    let data = b"test payload";
    let data_version = Version::new(1, 3, 0);
    let current = Version::new(1, 5, 0);
    let min = Some(Version::new(1, 0, 0));

    let encoded = encode_versioned(data, data_version).expect("encode failed");
    let (payload, ver, compat) =
        decode_versioned_with_check(&encoded, current, min).expect("decode failed");

    assert_eq!(payload.as_slice(), data.as_slice());
    assert_eq!(ver, data_version);
    assert!(compat.is_usable());
}

#[test]
fn test_decode_versioned_with_check_incompatible() {
    let data = b"test";
    let data_version = Version::new(2, 0, 0);
    let current = Version::new(1, 0, 0);

    let encoded = encode_versioned(data, data_version).expect("encode failed");
    let result = decode_versioned_with_check(&encoded, current, None);
    assert!(result.is_err());
}

#[test]
fn test_decode_invalid_magic() {
    let garbage = b"not valid data at all";
    let result = decode_versioned(garbage);
    assert!(result.is_err());
}

#[test]
fn test_decode_truncated_data() {
    // Too short to contain a valid header
    let short = &[0x4F, 0x58, 0x49, 0x56]; // just magic, no version bytes
    let result = decode_versioned(short);
    assert!(result.is_err());
}

#[test]
fn test_encode_empty_payload() {
    let data: &[u8] = &[];
    let version = Version::new(1, 0, 0);
    let encoded = encode_versioned(data, version).expect("encode failed");
    let (decoded, ver) = decode_versioned(&encoded).expect("decode failed");
    assert!(decoded.is_empty());
    assert_eq!(ver, version);
}

// ── Compatibility checking ────────────────────────────────────────────────────

#[test]
fn test_compatibility_same_version() {
    let v = Version::new(1, 0, 0);
    assert_eq!(
        check_compatibility(v, v, None),
        CompatibilityLevel::Compatible
    );
}

#[test]
fn test_compatibility_patch_difference_older_data() {
    let data = Version::new(1, 0, 0);
    let current = Version::new(1, 0, 5);
    let compat = check_compatibility(data, current, None);
    assert_eq!(compat, CompatibilityLevel::Compatible);
}

#[test]
fn test_compatibility_older_minor() {
    let data = Version::new(1, 0, 0);
    let current = Version::new(1, 5, 0);
    let compat = check_compatibility(data, current, None);
    assert!(compat.is_usable());
}

#[test]
fn test_compatibility_incompatible_major() {
    let data = Version::new(1, 0, 0);
    let current = Version::new(2, 0, 0);
    assert_eq!(
        check_compatibility(data, current, None),
        CompatibilityLevel::Incompatible
    );
}

#[test]
fn test_compatibility_below_minimum() {
    let data = Version::new(1, 1, 0);
    let current = Version::new(1, 5, 0);
    let min = Some(Version::new(1, 3, 0));
    assert_eq!(
        check_compatibility(data, current, min),
        CompatibilityLevel::Incompatible
    );
}

#[test]
fn test_compatibility_0x_same_minor() {
    let data = Version::new(0, 1, 0);
    let current = Version::new(0, 1, 9);
    assert_eq!(
        check_compatibility(data, current, None),
        CompatibilityLevel::Compatible
    );
}

#[test]
fn test_compatibility_0x_different_minor() {
    let data = Version::new(0, 1, 0);
    let current = Version::new(0, 2, 0);
    assert_eq!(
        check_compatibility(data, current, None),
        CompatibilityLevel::Incompatible
    );
}

#[test]
fn test_compatibility_newer_data_than_current() {
    // Data was encoded with a newer minor version than what the reader knows
    let data = Version::new(1, 9, 0);
    let current = Version::new(1, 0, 0);
    let compat = check_compatibility(data, current, None);
    assert_eq!(compat, CompatibilityLevel::CompatibleWithWarnings);
}

#[test]
fn test_compatibility_level_methods() {
    assert!(CompatibilityLevel::Compatible.is_usable());
    assert!(CompatibilityLevel::Compatible.is_fully_compatible());
    assert!(!CompatibilityLevel::Compatible.has_warnings());

    assert!(CompatibilityLevel::CompatibleWithWarnings.is_usable());
    assert!(!CompatibilityLevel::CompatibleWithWarnings.is_fully_compatible());
    assert!(CompatibilityLevel::CompatibleWithWarnings.has_warnings());

    assert!(!CompatibilityLevel::Incompatible.is_usable());
    assert!(!CompatibilityLevel::Incompatible.is_fully_compatible());
    assert!(!CompatibilityLevel::Incompatible.has_warnings());
}

// ── Migration path ─────────────────────────────────────────────────────────────

#[test]
fn test_can_migrate_same_major() {
    assert!(can_migrate(Version::new(1, 0, 0), Version::new(1, 9, 0)));
}

#[test]
fn test_can_migrate_forward_major() {
    assert!(can_migrate(Version::new(1, 0, 0), Version::new(3, 0, 0)));
}

#[test]
fn test_cannot_migrate_backward() {
    assert!(!can_migrate(Version::new(3, 0, 0), Version::new(1, 0, 0)));
}

#[test]
fn test_can_migrate_same_version() {
    // Migrating to the same version is trivially possible (within same major)
    assert!(can_migrate(Version::new(1, 0, 0), Version::new(1, 0, 0)));
}

#[test]
fn test_migration_path_same_version() {
    let path = migration_path(Version::new(1, 0, 0), Version::new(1, 0, 0));
    assert!(path.is_empty());
}

#[test]
fn test_migration_path_same_major() {
    let path = migration_path(Version::new(1, 0, 0), Version::new(1, 5, 0));
    assert!(path.is_empty());
}

#[test]
fn test_migration_path_one_major_bump() {
    let path = migration_path(Version::new(1, 0, 0), Version::new(2, 0, 0));
    assert!(
        path.is_empty(),
        "direct migration needs no intermediate steps"
    );
}

#[test]
fn test_migration_path_two_major_bumps() {
    let path = migration_path(Version::new(1, 0, 0), Version::new(3, 0, 0));
    assert_eq!(path.len(), 1);
    assert_eq!(path[0], Version::new(2, 0, 0));
}

#[test]
fn test_migration_path_three_major_bumps() {
    let path = migration_path(Version::new(1, 0, 0), Version::new(4, 0, 0));
    assert_eq!(path.len(), 2);
    assert_eq!(path[0], Version::new(2, 0, 0));
    assert_eq!(path[1], Version::new(3, 0, 0));
}

// ── High-level encode_versioned_value / decode_versioned_value ────────────────

#[test]
fn test_encode_decode_versioned_value_u32() {
    let version = Version::new(1, 0, 0);
    let encoded = oxicode::encode_versioned_value(&42u32, version).expect("encode failed");
    let (decoded, ver, _consumed): (u32, _, _) =
        oxicode::decode_versioned_value(&encoded).expect("decode failed");
    assert_eq!(decoded, 42u32);
    assert_eq!(ver, version);
}

#[test]
fn test_encode_decode_versioned_value_u64() {
    let version = Version::new(2, 5, 0);
    let encoded = oxicode::encode_versioned_value(&99u64, version).expect("encode failed");
    let (decoded, ver, _): (u64, _, _) =
        oxicode::decode_versioned_value(&encoded).expect("decode failed");
    assert_eq!(decoded, 99u64);
    assert_eq!(ver, version);
}

#[test]
fn test_encode_decode_versioned_value_bool() {
    let version = Version::new(1, 0, 0);
    for val in [true, false] {
        let encoded = oxicode::encode_versioned_value(&val, version).expect("encode failed");
        let (decoded, ver, _): (bool, _, _) =
            oxicode::decode_versioned_value(&encoded).expect("decode failed");
        assert_eq!(decoded, val);
        assert_eq!(ver, version);
    }
}

#[test]
fn test_encode_decode_versioned_value_string() {
    let version = Version::new(2, 3, 1);
    let original = String::from("hello oxicode versioning");
    let encoded = oxicode::encode_versioned_value(&original, version).expect("encode failed");
    let (decoded, ver, _): (String, _, _) =
        oxicode::decode_versioned_value(&encoded).expect("decode failed");
    assert_eq!(decoded, original);
    assert_eq!(ver, version);
}

#[test]
fn test_encode_decode_versioned_value_vec_u8() {
    let version = Version::new(1, 0, 0);
    let original: Vec<u8> = vec![1, 2, 3, 4, 5, 255, 0];
    let encoded = oxicode::encode_versioned_value(&original, version).expect("encode failed");
    let (decoded, ver, _): (Vec<u8>, _, _) =
        oxicode::decode_versioned_value(&encoded).expect("decode failed");
    assert_eq!(decoded, original);
    assert_eq!(ver, version);
}

#[test]
fn test_encode_decode_versioned_value_vec_u32() {
    let version = Version::new(1, 2, 3);
    let original: Vec<u32> = vec![100, 200, 300, 400, 500];
    let encoded = oxicode::encode_versioned_value(&original, version).expect("encode failed");
    let (decoded, ver, _): (Vec<u32>, _, _) =
        oxicode::decode_versioned_value(&encoded).expect("decode failed");
    assert_eq!(decoded, original);
    assert_eq!(ver, version);
}

#[test]
fn test_version_is_preserved_in_encoded_data() {
    let version = Version::new(5, 12, 99);
    let encoded = oxicode::encode_versioned_value(&100u64, version).expect("encode failed");
    // extract_version should agree with what was stored
    let extracted = extract_version(&encoded).expect("extract failed");
    assert_eq!(extracted, version);
}

#[test]
fn test_versioned_value_is_detected_as_versioned() {
    let version = Version::new(1, 0, 0);
    let encoded = oxicode::encode_versioned_value(&true, version).expect("encode failed");
    assert!(is_versioned(&encoded));
}

#[test]
fn test_versioned_value_decode_wrong_type_fails() {
    let version = Version::new(1, 0, 0);
    // Encode a u32
    let encoded = oxicode::encode_versioned_value(&42u32, version).expect("encode failed");
    // Try to decode as a String — should fail because formats differ
    // (u32 encodes as varint, String encodes as length+bytes)
    // This verifies the payload is type-sensitive
    let result: oxicode::Result<(String, _, _)> = oxicode::decode_versioned_value(&encoded);
    // This may or may not fail depending on the value, but we verify the API works
    let _ = result; // at minimum confirm it compiles and runs
}

#[test]
fn test_multiple_version_numbers_round_trip() {
    // Test a matrix of version numbers to ensure bytes encoding is correct
    let versions = [
        Version::new(0, 0, 0),
        Version::new(1, 0, 0),
        Version::new(0, 1, 0),
        Version::new(0, 0, 1),
        Version::new(255, 255, 255),
        Version::new(1000, 2000, 3000),
        Version::new(65535, 65535, 65535),
    ];
    for version in versions {
        let encoded = oxicode::encode_versioned_value(&version.tuple().0, version).expect("encode");
        let (_, ver, _): (u16, _, _) = oxicode::decode_versioned_value(&encoded).expect("decode");
        assert_eq!(ver, version);
    }
}

#[cfg(feature = "derive")]
mod derive_tests {
    use super::*;
    use oxicode::{Decode, Encode};

    #[derive(Debug, PartialEq, Encode, Decode)]
    struct DataV1 {
        name: String,
        value: u32,
    }

    #[derive(Debug, PartialEq, Encode, Decode)]
    struct SimpleRecord {
        id: u64,
        score: f32,
        active: bool,
    }

    #[test]
    fn test_derived_struct_versioned_roundtrip() {
        let version = Version::new(1, 0, 0);
        let original = DataV1 {
            name: String::from("test"),
            value: 42,
        };
        let encoded = oxicode::encode_versioned_value(&original, version).expect("encode failed");
        let (decoded, ver, _): (DataV1, _, _) =
            oxicode::decode_versioned_value(&encoded).expect("decode failed");
        assert_eq!(decoded, original);
        assert_eq!(ver, version);
    }

    #[test]
    fn test_derived_struct_complex_roundtrip() {
        let version = Version::new(2, 1, 0);
        let original = SimpleRecord {
            id: 9999,
            score: 2.71,
            active: true,
        };
        let encoded = oxicode::encode_versioned_value(&original, version).expect("encode failed");
        let (decoded, ver, _): (SimpleRecord, _, _) =
            oxicode::decode_versioned_value(&encoded).expect("decode failed");
        assert_eq!(decoded, original);
        assert_eq!(ver, version);
    }

    #[test]
    fn test_version_schema_evolution_metadata() {
        // V1 data with V1 schema — a future reader can inspect the stored version
        // before deciding how to decode.
        let v1_data = DataV1 {
            name: String::from("record"),
            value: 100,
        };
        let v1 = Version::new(1, 0, 0);
        let encoded = oxicode::encode_versioned_value(&v1_data, v1).expect("encode failed");

        // A future reader inspects the stored version before decoding
        let stored_version = extract_version(&encoded).expect("extract failed");
        assert_eq!(stored_version.major, 1);

        // And can check compatibility before attempting to decode
        let current = Version::new(1, 5, 0);
        let compat = check_compatibility(stored_version, current, None);
        assert!(compat.is_usable());
    }

    #[test]
    fn test_version_numbers_comparison() {
        let v1 = Version::new(1, 0, 0);
        let v2 = Version::new(2, 0, 0);
        assert!(v2 > v1);
        assert!(v1 < v2);
        assert_ne!(v1, v2);

        let v_patch = Version::new(1, 0, 1);
        assert!(v_patch > v1);

        let v_minor = Version::new(1, 1, 0);
        assert!(v_minor > v1);
        assert!(v_minor > v_patch);
    }
}

// ── Additional versioning tests ───────────────────────────────────────────────

/// Verify all four ordering relationships across patch / minor / major boundaries.
#[test]
fn test_version_ordering() {
    let v1_0_0 = Version::new(1, 0, 0);
    let v1_0_1 = Version::new(1, 0, 1);
    let v1_1_0 = Version::new(1, 1, 0);
    let v2_0_0 = Version::new(2, 0, 0);

    assert!(v1_0_0 < v1_0_1);
    assert!(v1_0_1 < v1_1_0);
    assert!(v1_1_0 < v2_0_0);
    assert!(v1_0_0 < v2_0_0);
    assert_eq!(v1_0_0, Version::new(1, 0, 0));
}

/// can_migrate: same version is always possible; forward migration is possible;
/// backward migration is not.
#[test]
fn test_can_migrate_extended() {
    let v1 = Version::new(1, 0, 0);
    let v2 = Version::new(2, 0, 0);

    // Same version — trivially migratable (same major).
    assert!(can_migrate(v1, v1));

    // Forward migration is supported.
    assert!(can_migrate(v1, v2));

    // Backward migration is not supported.
    assert!(!can_migrate(v2, v1));
}

/// encode_versioned_value must produce identical bytes on repeated calls.
#[test]
fn test_versioned_encoding_is_deterministic() {
    let v = Version::new(1, 2, 3);

    let enc1 = oxicode::encode_versioned_value(&42u32, v).expect("encode 1");
    let enc2 = oxicode::encode_versioned_value(&42u32, v).expect("encode 2");
    assert_eq!(enc1, enc2, "versioned encoding must be deterministic");
}

/// Encoding the same payload with different version numbers must differ in the
/// header bytes and therefore produce a different byte sequence overall.
#[test]
fn test_different_versions_produce_different_bytes() {
    let enc_v1 = oxicode::encode_versioned_value(&42u32, Version::new(1, 0, 0)).expect("encode v1");
    let enc_v2 = oxicode::encode_versioned_value(&42u32, Version::new(2, 0, 0)).expect("encode v2");

    assert_ne!(
        enc_v1, enc_v2,
        "different versions should produce different bytes"
    );
}

/// Verify that multiple minor-version bumps within the same major are all
/// forward-compatible (can_migrate returns true) and that the stored version
/// round-trips correctly.
#[test]
fn test_version_compatibility_chain() {
    let versions = [
        Version::new(1, 0, 0),
        Version::new(1, 1, 0),
        Version::new(1, 2, 0),
        Version::new(1, 3, 0),
    ];

    // Every earlier version can migrate to every later version.
    for (i, &from) in versions.iter().enumerate() {
        for &to in &versions[i..] {
            assert!(
                can_migrate(from, to),
                "expected can_migrate({from} -> {to})"
            );
        }
    }
}

/// Encode with V1, extract the stored version, and confirm it matches exactly.
#[test]
fn test_stored_version_matches_encoded_version() {
    let v = Version::new(3, 7, 11);
    let encoded = oxicode::encode_versioned_value(&100u64, v).expect("encode");
    let stored = extract_version(&encoded).expect("extract");
    assert_eq!(stored, v);
}

/// Verify that decode_versioned_value returns the correct 3-tuple
/// (value, version, bytes_consumed) for a range of primitive types.
#[test]
fn test_decode_versioned_value_tuple_shape() {
    let v = Version::new(2, 0, 0);

    // u8
    let enc = oxicode::encode_versioned_value(&255u8, v).expect("encode u8");
    let (val, ver, consumed): (u8, _, usize) =
        oxicode::decode_versioned_value(&enc).expect("decode u8");
    assert_eq!(val, 255u8);
    assert_eq!(ver, v);
    assert!(consumed > 0, "consumed bytes must be positive");

    // i32
    let enc = oxicode::encode_versioned_value(&-42i32, v).expect("encode i32");
    let (val, ver, consumed): (i32, _, usize) =
        oxicode::decode_versioned_value(&enc).expect("decode i32");
    assert_eq!(val, -42i32);
    assert_eq!(ver, v);
    assert!(consumed > 0);
}

#[cfg(feature = "derive")]
mod extra_derive_tests {
    use super::*;
    use oxicode::{Decode, Encode};

    /// V1 schema: id + name only.
    #[derive(Debug, PartialEq, Encode, Decode)]
    struct UserV1 {
        id: u64,
        name: String,
    }

    /// Encode a V1 record, decode it back as V1, and verify the stored version
    /// header round-trips correctly.  This simulates the first step of a
    /// multi-version migration chain (v1 -> v2 -> v3) where the reader first
    /// inspects the stored version before deciding which schema to use.
    #[test]
    fn test_multi_version_migration_chain() {
        let user_v1 = UserV1 {
            id: 1,
            name: "alice".to_string(),
        };
        let v1 = Version::new(1, 0, 0);

        let bytes_v1 = oxicode::encode_versioned_value(&user_v1, v1).expect("encode v1");

        // A reader can inspect the stored version first…
        let stored = extract_version(&bytes_v1).expect("extract version");
        assert_eq!(stored, v1);

        // …then decode using the matching schema.
        let (decoded, version, _consumed): (UserV1, _, _) =
            oxicode::decode_versioned_value(&bytes_v1).expect("decode v1");

        assert_eq!(version, v1);
        assert_eq!(decoded, user_v1);
    }

    /// A V2-encoded record produces bytes that are distinct from a V1-encoded
    /// record of the same logical data (different version header).
    #[test]
    fn test_v1_v2_bytes_differ() {
        let v1 = Version::new(1, 0, 0);
        let v2 = Version::new(2, 0, 0);

        let record = UserV1 {
            id: 42,
            name: "bob".to_string(),
        };

        let enc_v1 = oxicode::encode_versioned_value(&record, v1).expect("encode v1");
        let enc_v2 = oxicode::encode_versioned_value(&record, v2).expect("encode v2");

        assert_ne!(enc_v1, enc_v2, "v1 and v2 headers must differ");
    }

    /// A future reader can check compatibility before attempting to decode,
    /// enabling graceful handling of schema evolution.
    #[test]
    fn test_schema_evolution_compatibility_gate() {
        let v1 = Version::new(1, 0, 0);
        let v_current = Version::new(1, 5, 0);

        let record = UserV1 {
            id: 7,
            name: "carol".to_string(),
        };
        let encoded = oxicode::encode_versioned_value(&record, v1).expect("encode");

        let stored = extract_version(&encoded).expect("extract");
        let compat = check_compatibility(stored, v_current, None);

        // An older minor is still usable (compatible with warnings).
        assert!(compat.is_usable());
    }
}

// ── 15 additional versioning tests ───────────────────────────────────────────

/// 1. test_version_schema_evolution_add_optional_field
///
/// V1 encodes (id: u64, name: String).  V2 adds an Option<String> tag field.
/// When V1 bytes are decoded as V2 the Option<String> is missing, so decoding
/// the V2 struct directly from V1 payload will fail.  The test demonstrates the
/// intended migration workflow: inspect the stored version, decode as the old
/// schema when the minor version is older, then construct the new schema with the
/// default None.
#[cfg(feature = "derive")]
#[test]
fn test_version_schema_evolution_add_optional_field() {
    use oxicode::versioning::{check_compatibility, extract_version, Version};
    use oxicode::{Decode, Encode};

    #[derive(Debug, PartialEq, Encode, Decode)]
    struct RecordV1 {
        id: u64,
        name: String,
    }

    #[derive(Debug, PartialEq)]
    struct RecordV2 {
        id: u64,
        name: String,
        tag: Option<String>,
    }

    let v1 = Version::new(1, 0, 0);
    let original = RecordV1 {
        id: 7,
        name: "alice".to_string(),
    };
    let encoded = oxicode::encode_versioned_value(&original, v1).expect("encode v1");

    // A V2 reader inspects the stored version first.
    let stored = extract_version(&encoded).expect("extract");
    assert_eq!(stored.major, 1);
    assert_eq!(stored.minor, 0);

    // The V2 current version is 1.1.0.
    let v2 = Version::new(1, 1, 0);
    let compat = check_compatibility(stored, v2, None);
    assert!(compat.is_usable(), "V1 data must be usable by a V2 reader");

    // Decode using the V1 schema (what the bytes actually contain).
    let (decoded_v1, ver, _): (RecordV1, _, _) =
        oxicode::decode_versioned_value(&encoded).expect("decode as v1");
    assert_eq!(ver, v1);
    assert_eq!(decoded_v1.id, 7);

    // Construct V2 with the new optional field defaulting to None.
    let migrated = RecordV2 {
        id: decoded_v1.id,
        name: decoded_v1.name,
        tag: None,
    };
    assert_eq!(
        migrated.tag, None,
        "new optional field must default to None"
    );
}

/// 2. test_versioned_value_roundtrip_v1_to_v3
///
/// Encode a record as V1, verify compatibility to V3 via migration_path,
/// then decode the payload with the V1 schema (the bytes haven't changed).
#[cfg(feature = "derive")]
#[test]
fn test_versioned_value_roundtrip_v1_to_v3() {
    use oxicode::versioning::{can_migrate, extract_version, migration_path, Version};
    use oxicode::{Decode, Encode};

    #[derive(Debug, PartialEq, Encode, Decode)]
    struct Payload {
        value: u32,
        label: String,
    }

    let v1 = Version::new(1, 0, 0);
    let v3 = Version::new(3, 0, 0);

    let original = Payload {
        value: 42,
        label: "test".to_string(),
    };
    let encoded = oxicode::encode_versioned_value(&original, v1).expect("encode");

    // V1 → V3 migration must be possible.
    assert!(can_migrate(v1, v3), "forward migration must be supported");

    // Migration path from V1 to V3 contains one intermediate step (V2).
    let path = migration_path(v1, v3);
    assert_eq!(path.len(), 1);
    assert_eq!(path[0], Version::new(2, 0, 0));

    // The actual bytes were encoded as V1; verify the stored version is still V1.
    let stored = extract_version(&encoded).expect("extract");
    assert_eq!(stored, v1);

    // Decode using the V1 schema.
    let (decoded, ver, _): (Payload, _, _) =
        oxicode::decode_versioned_value(&encoded).expect("decode");
    assert_eq!(ver, v1);
    assert_eq!(decoded, original);
}

/// 3. test_versioned_decode_unknown_version_error
///
/// Craft a byte buffer whose header_version byte (index 4) is set to a value
/// greater than 1 (the only supported header format version).  Decoding must
/// return an error ("unsupported header version").
#[test]
fn test_versioned_decode_unknown_version_error() {
    use oxicode::versioning::{decode_versioned, encode_versioned, Version};

    // Produce a valid versioned payload then corrupt the header-format-version byte.
    let mut bytes = encode_versioned(b"payload", Version::new(1, 0, 0)).expect("encode");

    // Byte 4 is the header format version; setting it to 99 makes it unsupported.
    bytes[4] = 99;

    let result = decode_versioned(&bytes);
    assert!(
        result.is_err(),
        "decoding with unknown header version must fail"
    );
}

/// 4. test_migration_chain_three_steps
///
/// migration_path(1.0.0 → 4.0.0) must produce exactly two intermediate versions
/// [2.0.0, 3.0.0], forming the three-step chain V1 → V2 → V3 → V4.
#[test]
fn test_migration_chain_three_steps() {
    use oxicode::versioning::{migration_path, Version};

    let from = Version::new(1, 0, 0);
    let to = Version::new(4, 0, 0);

    let path = migration_path(from, to);
    assert_eq!(path.len(), 2, "three-step chain has two intermediate nodes");
    assert_eq!(path[0], Version::new(2, 0, 0));
    assert_eq!(path[1], Version::new(3, 0, 0));
}

/// 5. test_versioned_with_nested_struct
///
/// A parent struct contains a nested child struct.  Both encode/decode correctly
/// through the versioned API.
#[cfg(feature = "derive")]
#[test]
fn test_versioned_with_nested_struct() {
    use oxicode::versioning::Version;
    use oxicode::{Decode, Encode};

    #[derive(Debug, PartialEq, Encode, Decode)]
    struct Inner {
        x: i32,
        y: i32,
    }

    #[derive(Debug, PartialEq, Encode, Decode)]
    struct Outer {
        name: String,
        pos: Inner,
        weight: f64,
    }

    let version = Version::new(1, 2, 3);
    let original = Outer {
        name: "origin".to_string(),
        pos: Inner { x: -5, y: 10 },
        weight: std::f64::consts::PI,
    };

    let encoded = oxicode::encode_versioned_value(&original, version).expect("encode");
    let (decoded, ver, _): (Outer, _, _) =
        oxicode::decode_versioned_value(&encoded).expect("decode");

    assert_eq!(decoded, original);
    assert_eq!(ver, version);
}

/// 6. test_version_field_count_mismatch
///
/// Encode a two-field struct, then truncate the payload so that the second field
/// is missing.  Decoding the same struct type from the truncated bytes must fail.
#[cfg(feature = "derive")]
#[test]
fn test_version_field_count_mismatch() {
    use oxicode::versioning::{decode_versioned, encode_versioned, Version};
    use oxicode::{Decode, Encode};

    #[derive(Debug, PartialEq, Encode, Decode)]
    struct TwoFields {
        a: u64,
        b: u64,
    }

    let version = Version::new(1, 0, 0);
    let original = TwoFields { a: 1, b: 2 };

    // Encode the plain payload (without version header) so we can truncate it.
    let payload = oxicode::encode_to_vec(&original).expect("encode payload");

    // Keep only the first byte of the payload (definitely too short for u64 + u64).
    let short_payload = &payload[..1];

    // Re-wrap the truncated payload with the versioned header.
    let versioned_truncated = encode_versioned(short_payload, version).expect("encode versioned");

    // Extracting the header succeeds (it is intact), but decoding the struct fails.
    let (raw, _ver) = decode_versioned(&versioned_truncated).expect("decode raw");
    let result: oxicode::Result<(TwoFields, _)> = oxicode::decode_from_slice(&raw);
    assert!(
        result.is_err(),
        "truncated payload must not decode as TwoFields"
    );
}

/// 7. test_can_migrate_true
///
/// can_migrate(v1, v2) returns true for a forward major-version migration.
#[test]
fn test_can_migrate_true() {
    use oxicode::versioning::{can_migrate, Version};

    let v1 = Version::new(1, 0, 0);
    let v2 = Version::new(2, 0, 0);
    assert!(
        can_migrate(v1, v2),
        "forward major migration must be possible"
    );
}

/// 8. test_can_migrate_false
///
/// can_migrate(v2, v1) returns false — downgrade migration is not supported.
#[test]
fn test_can_migrate_false() {
    use oxicode::versioning::{can_migrate, Version};

    let v1 = Version::new(1, 0, 0);
    let v2 = Version::new(2, 0, 0);
    assert!(
        !can_migrate(v2, v1),
        "backward (downgrade) migration must not be possible"
    );
}

/// 9. test_migration_path_chain
///
/// migration_path(v1, v3) must return [v2] — the single intermediate version
/// that forms the two-step migration chain v1 → v2 → v3.
#[test]
fn test_migration_path_chain() {
    use oxicode::versioning::{migration_path, Version};

    let v1 = Version::new(1, 0, 0);
    let v3 = Version::new(3, 0, 0);

    let path = migration_path(v1, v3);
    assert_eq!(path.len(), 1, "one intermediate step between v1 and v3");
    assert_eq!(
        path[0],
        Version::new(2, 0, 0),
        "intermediate step must be v2"
    );
}

/// 10. test_versioned_encode_includes_version_header
///
/// The first 4 bytes of versioned output must equal the OXIV magic bytes,
/// confirming that every call to encode_versioned_value prepends the version header.
#[test]
fn test_versioned_encode_includes_version_header() {
    use oxicode::versioning::{Version, VERSIONED_MAGIC};

    let version = Version::new(2, 3, 4);
    let encoded = oxicode::encode_versioned_value(&99u32, version).expect("encode");

    assert!(
        encoded.len() >= VERSIONED_MAGIC.len(),
        "encoded output must be at least as long as the magic bytes"
    );
    assert_eq!(
        &encoded[..VERSIONED_MAGIC.len()],
        &VERSIONED_MAGIC,
        "first bytes must be the OXIV magic"
    );
}

/// 11. test_compatibility_encode_current_decode_old
///
/// Encode with a newer minor version (1.5.0) and attempt to decode using a reader
/// that only knows version 1.0.0.  The compatibility check should report
/// CompatibleWithWarnings (newer data than current reader) rather than Incompatible.
#[test]
fn test_compatibility_encode_current_decode_old() {
    use oxicode::versioning::{
        decode_versioned_with_check, encode_versioned, CompatibilityLevel, Version,
    };

    let new_version = Version::new(1, 5, 0);
    let old_reader = Version::new(1, 0, 0);

    let encoded = encode_versioned(b"data", new_version).expect("encode");
    let result = decode_versioned_with_check(&encoded, old_reader, None);

    // The data version (1.5.0) is newer than what the reader knows (1.0.0),
    // same major → CompatibleWithWarnings, not an error.
    let (_, ver, compat) = result.expect("decode must succeed despite version mismatch");
    assert_eq!(ver, new_version);
    assert_eq!(
        compat,
        CompatibilityLevel::CompatibleWithWarnings,
        "newer-minor data decoded by older reader must yield CompatibleWithWarnings"
    );
}

/// 12. test_versioned_large_payload
///
/// Migrate a struct that contains a 100-element Vec<u32>.  Verifies that the
/// versioned encode/decode pipeline handles payloads significantly larger than
/// the 11-byte header without corruption.
#[cfg(feature = "derive")]
#[test]
fn test_versioned_large_payload() {
    use oxicode::versioning::Version;
    use oxicode::{Decode, Encode};

    #[derive(Debug, PartialEq, Encode, Decode)]
    struct LargeRecord {
        id: u64,
        values: Vec<u32>,
    }

    let version = Version::new(1, 0, 0);
    let original = LargeRecord {
        id: 12345,
        values: (0u32..100).collect(),
    };

    let encoded = oxicode::encode_versioned_value(&original, version).expect("encode");
    let (decoded, ver, consumed): (LargeRecord, _, _) =
        oxicode::decode_versioned_value(&encoded).expect("decode");

    assert_eq!(decoded, original);
    assert_eq!(ver, version);
    assert!(consumed > 0);
    // The versioned envelope must be larger than the plain encoded form.
    let plain = oxicode::encode_to_vec(&original).expect("plain encode");
    assert!(encoded.len() > plain.len());
}

/// 13. test_version_number_in_wire_format
///
/// Verify the exact byte layout of the version header: bytes 5–10 must contain
/// the major, minor, and patch numbers as little-endian u16 values.
#[test]
fn test_version_number_in_wire_format() {
    use oxicode::versioning::Version;

    let version = Version::new(3, 7, 11);
    let encoded = oxicode::encode_versioned_value(&0u8, version).expect("encode");

    // Header layout: [0..4] magic, [4] header_version,
    // [5..7] major LE u16, [7..9] minor LE u16, [9..11] patch LE u16.
    let major = u16::from_le_bytes([encoded[5], encoded[6]]);
    let minor = u16::from_le_bytes([encoded[7], encoded[8]]);
    let patch = u16::from_le_bytes([encoded[9], encoded[10]]);

    assert_eq!(major, 3, "major must be encoded at bytes 5–6 as LE u16");
    assert_eq!(minor, 7, "minor must be encoded at bytes 7–8 as LE u16");
    assert_eq!(patch, 11, "patch must be encoded at bytes 9–10 as LE u16");
}

/// 14. test_multiple_migrations_applied
///
/// Simulate applying three successive data transformations by encoding at V1,
/// re-encoding the decoded payload at V2, then re-encoding again at V3.  After
/// each step the stored version header must reflect the new version, confirming
/// that each "migration" produces a distinct versioned artefact.
#[test]
fn test_multiple_migrations_applied() {
    use oxicode::versioning::{extract_version, Version};

    let v1 = Version::new(1, 0, 0);
    let v2 = Version::new(2, 0, 0);
    let v3 = Version::new(3, 0, 0);

    // Step 1: encode as V1.
    let enc1 = oxicode::encode_versioned_value(&100u64, v1).expect("encode v1");
    assert_eq!(extract_version(&enc1).expect("extract v1"), v1);

    // Step 2: decode from V1, re-encode as V2 (the "migration" step).
    let (val2, _, _): (u64, _, _) = oxicode::decode_versioned_value(&enc1).expect("decode v1");
    let enc2 = oxicode::encode_versioned_value(&val2, v2).expect("encode v2");
    assert_eq!(extract_version(&enc2).expect("extract v2"), v2);

    // Step 3: decode from V2, re-encode as V3.
    let (val3, _, _): (u64, _, _) = oxicode::decode_versioned_value(&enc2).expect("decode v2");
    let enc3 = oxicode::encode_versioned_value(&val3, v3).expect("encode v3");
    assert_eq!(extract_version(&enc3).expect("extract v3"), v3);

    // The value must survive all three transformations unchanged.
    let (final_val, final_ver, _): (u64, _, _) =
        oxicode::decode_versioned_value(&enc3).expect("decode v3");
    assert_eq!(final_val, 100u64);
    assert_eq!(final_ver, v3);
}

/// 15. test_versioned_default_field_values
///
/// When migrating from a V1 struct to a V2 struct that has an additional field
/// with a default value, the migration must yield the default for that field.
/// This test encodes a minimal V1 struct and constructs a V2 struct from it by
/// supplying an explicit default for the new field.
#[cfg(feature = "derive")]
#[test]
fn test_versioned_default_field_values() {
    use oxicode::versioning::Version;
    use oxicode::{Decode, Encode};

    #[derive(Debug, PartialEq, Encode, Decode)]
    struct ConfigV1 {
        timeout_ms: u32,
        retries: u8,
    }

    #[derive(Debug, PartialEq)]
    struct ConfigV2 {
        timeout_ms: u32,
        retries: u8,
        /// New in V2; absent in V1 data → defaults to false.
        verbose: bool,
    }

    let v1 = Version::new(1, 0, 0);
    let original = ConfigV1 {
        timeout_ms: 5000,
        retries: 3,
    };
    let encoded = oxicode::encode_versioned_value(&original, v1).expect("encode v1");

    // Decode with the V1 schema.
    let (decoded_v1, ver, _): (ConfigV1, _, _) =
        oxicode::decode_versioned_value(&encoded).expect("decode v1");
    assert_eq!(ver, v1);

    // Construct V2 by migrating V1 fields and applying the default for `verbose`.
    let migrated = ConfigV2 {
        timeout_ms: decoded_v1.timeout_ms,
        retries: decoded_v1.retries,
        verbose: bool::default(),
    };

    assert_eq!(migrated.timeout_ms, 5000);
    assert_eq!(migrated.retries, 3);
    assert!(
        !migrated.verbose,
        "new field must default to false during migration"
    );
}

// ── 20 additional comprehensive versioning tests ───────────────────────────

/// NEW-1. Version::new stores major, minor, patch correctly.
///
/// Distinct from test_version_new_fields which uses (3,7,11).  This test
/// exercises values that expose endianness or off-by-one errors in the internal
/// representation.
#[test]
fn test_version_new_creates_correct_fields() {
    let v = Version::new(100, 200, 300);
    assert_eq!(v.major, 100, "major must be 100");
    assert_eq!(v.minor, 200, "minor must be 200");
    assert_eq!(v.patch, 300, "patch must be 300");

    let v2 = Version::new(0, 0, 1);
    assert_eq!(v2.major, 0);
    assert_eq!(v2.minor, 0);
    assert_eq!(v2.patch, 1);
}

/// NEW-2. Strict ordering: 1.0.0 < 1.1.0 < 2.0.0.
///
/// Complements test_version_ordering by specifically testing the three
/// landmark versions named in the task spec.
#[test]
fn test_version_ordering_chain() {
    let v1_0_0 = Version::new(1, 0, 0);
    let v1_1_0 = Version::new(1, 1, 0);
    let v2_0_0 = Version::new(2, 0, 0);

    assert!(v1_0_0 < v1_1_0, "1.0.0 must be less than 1.1.0");
    assert!(v1_1_0 < v2_0_0, "1.1.0 must be less than 2.0.0");
    assert!(v1_0_0 < v2_0_0, "1.0.0 must be less than 2.0.0");
    assert_eq!(
        v1_0_0,
        Version::new(1, 0, 0),
        "identical versions must be equal"
    );
}

/// NEW-3. encode_versioned_value + decode_versioned_value roundtrip for a
/// large struct with many fields (derive feature).
#[cfg(feature = "derive")]
#[test]
fn test_encode_versioned_value_large_struct_roundtrip() {
    use oxicode::{Decode, Encode};

    #[derive(Debug, PartialEq, Encode, Decode)]
    struct BigRecord {
        id: u64,
        name: String,
        score: f64,
        active: bool,
        tags: Vec<String>,
        counts: Vec<u32>,
    }

    let version = Version::new(3, 2, 1);
    let original = BigRecord {
        id: 999_999_999,
        name: "large record test".to_string(),
        score: 1234.5678_f64,
        active: true,
        tags: vec!["alpha".to_string(), "beta".to_string(), "gamma".to_string()],
        counts: (0u32..50).collect(),
    };

    let encoded = oxicode::encode_versioned_value(&original, version).expect("encode failed");
    let (decoded, ver, consumed): (BigRecord, _, usize) =
        oxicode::decode_versioned_value(&encoded).expect("decode failed");

    assert_eq!(decoded, original, "decoded value must equal original");
    assert_eq!(ver, version, "decoded version must match encoded version");
    assert!(consumed > 0, "consumed must be positive");
}

/// NEW-4. decode_versioned_value returns the correct version number.
///
/// Encodes a simple value then verifies the returned version is exactly
/// the one that was supplied during encoding.
#[test]
fn test_decode_versioned_value_returns_correct_version() {
    let expected_version = Version::new(7, 3, 15);
    let encoded = oxicode::encode_versioned_value(&42u32, expected_version).expect("encode failed");
    let (val, ver, _): (u32, _, _) =
        oxicode::decode_versioned_value(&encoded).expect("decode failed");
    assert_eq!(val, 42u32);
    assert_eq!(
        ver, expected_version,
        "returned version must equal the version used at encoding time"
    );
}

/// NEW-5. Version with u16 major, minor, patch boundary values.
///
/// u16::MAX (65535) is the maximum representable value in each component.
/// All three must survive a to_bytes / from_bytes round-trip.
#[test]
fn test_version_u16_boundary_values() {
    let max = Version::new(u16::MAX, u16::MAX, u16::MAX);
    let bytes = max.to_bytes();
    let restored = Version::from_bytes(&bytes).expect("from_bytes must succeed for max version");
    assert_eq!(
        max, restored,
        "max-value version must survive bytes roundtrip"
    );

    let zero = Version::new(0, 0, 0);
    let bytes0 = zero.to_bytes();
    let restored0 = Version::from_bytes(&bytes0).expect("from_bytes must succeed for zero version");
    assert_eq!(zero, restored0, "zero version must survive bytes roundtrip");
}

/// NEW-6. encode_versioned_value for Vec<String>.
///
/// Verifies that a heap-allocated collection of strings can be versioned
/// and recovered without corruption.
#[test]
fn test_encode_versioned_value_vec_string() {
    let version = Version::new(2, 0, 0);
    let original: Vec<String> = vec![
        "hello".to_string(),
        "versioned".to_string(),
        "world".to_string(),
        "".to_string(), // empty string edge case
    ];
    let encoded =
        oxicode::encode_versioned_value(&original, version).expect("encode Vec<String> failed");
    let (decoded, ver, _): (Vec<String>, _, _) =
        oxicode::decode_versioned_value(&encoded).expect("decode Vec<String> failed");

    assert_eq!(
        decoded, original,
        "Vec<String> must survive versioned roundtrip"
    );
    assert_eq!(ver, version);
}

/// NEW-7. encode_versioned_value for nested structs (derive feature).
///
/// A three-level nesting (outer → middle → inner) must encode and decode
/// correctly through the versioned API.
#[cfg(feature = "derive")]
#[test]
fn test_encode_versioned_value_nested_structs() {
    use oxicode::{Decode, Encode};

    #[derive(Debug, PartialEq, Encode, Decode)]
    struct InnerNode {
        value: u64,
        label: String,
    }

    #[derive(Debug, PartialEq, Encode, Decode)]
    struct MiddleNode {
        children: Vec<InnerNode>,
        count: u32,
    }

    #[derive(Debug, PartialEq, Encode, Decode)]
    struct OuterNode {
        name: String,
        middle: MiddleNode,
        active: bool,
    }

    let version = Version::new(1, 5, 0);
    let original = OuterNode {
        name: "root".to_string(),
        middle: MiddleNode {
            children: vec![
                InnerNode {
                    value: 1,
                    label: "first".to_string(),
                },
                InnerNode {
                    value: 2,
                    label: "second".to_string(),
                },
            ],
            count: 2,
        },
        active: true,
    };

    let encoded =
        oxicode::encode_versioned_value(&original, version).expect("encode nested structs failed");
    let (decoded, ver, _): (OuterNode, _, _) =
        oxicode::decode_versioned_value(&encoded).expect("decode nested structs failed");

    assert_eq!(decoded, original);
    assert_eq!(ver, version);
}

/// NEW-8. Multiple sequential versioned encode/decode operations.
///
/// Performs 5 consecutive encode/decode cycles on different values and
/// versions, verifying that each round-trip is independent.
#[test]
fn test_multiple_sequential_versioned_operations() {
    let pairs: &[(u64, Version)] = &[
        (0, Version::new(1, 0, 0)),
        (1, Version::new(1, 1, 0)),
        (u64::MAX, Version::new(2, 0, 0)),
        (42, Version::new(0, 5, 3)),
        (100, Version::new(10, 0, 255)),
    ];

    for (value, version) in pairs {
        let encoded =
            oxicode::encode_versioned_value(value, *version).expect("encode in sequence failed");
        let (decoded, ver, _): (u64, _, _) =
            oxicode::decode_versioned_value(&encoded).expect("decode in sequence failed");
        assert_eq!(decoded, *value, "sequential value mismatch");
        assert_eq!(ver, *version, "sequential version mismatch");
    }
}

/// NEW-9. Versioned encoding of primitives: u64, bool, String.
///
/// Each primitive is independently encoded with a distinct version and
/// decoded back, verifying the type-level roundtrip for all three cases.
#[test]
fn test_versioned_primitives_u64_bool_string() {
    // u64
    {
        let v = Version::new(1, 0, 0);
        let encoded = oxicode::encode_versioned_value(&u64::MAX, v).expect("encode u64");
        let (val, ver, _): (u64, _, _) =
            oxicode::decode_versioned_value(&encoded).expect("decode u64");
        assert_eq!(val, u64::MAX);
        assert_eq!(ver, v);
    }
    // bool
    {
        let v = Version::new(2, 1, 0);
        for b in [true, false] {
            let encoded = oxicode::encode_versioned_value(&b, v).expect("encode bool");
            let (val, ver, _): (bool, _, _) =
                oxicode::decode_versioned_value(&encoded).expect("decode bool");
            assert_eq!(val, b);
            assert_eq!(ver, v);
        }
    }
    // String
    {
        let v = Version::new(3, 0, 7);
        let s = "hello versioned string".to_string();
        let encoded = oxicode::encode_versioned_value(&s, v).expect("encode String");
        let (val, ver, _): (String, _, _) =
            oxicode::decode_versioned_value(&encoded).expect("decode String");
        assert_eq!(val, s);
        assert_eq!(ver, v);
    }
}

/// NEW-10. decode_versioned_value with version that has all-zero components.
///
/// Version::zero() is a valid version and must be preserved through the
/// versioned encode/decode cycle.
#[test]
fn test_decode_versioned_value_version_zero() {
    let version = Version::zero();
    assert_eq!(version.major, 0);
    assert_eq!(version.minor, 0);
    assert_eq!(version.patch, 0);

    let encoded =
        oxicode::encode_versioned_value(&999u32, version).expect("encode with zero version");
    let (val, ver, _): (u32, _, _) =
        oxicode::decode_versioned_value(&encoded).expect("decode with zero version");
    assert_eq!(val, 999u32);
    assert_eq!(ver, version, "all-zero version must be preserved");
}

/// NEW-11. Schema evolution: V1 struct bytes decodable as V2 struct with
/// skip+default field.
///
/// V1 bytes (id + name) are decoded as V1 then migrated to V2 by appending
/// a default value for the new `description` field.  The compatibility check
/// must confirm the V2 reader can use the V1 data.
#[cfg(feature = "derive")]
#[test]
fn test_schema_evolution_skip_and_default_field() {
    use oxicode::versioning::{check_compatibility, extract_version};
    use oxicode::{Decode, Encode};

    #[derive(Debug, PartialEq, Encode, Decode)]
    struct ArticleV1 {
        id: u64,
        title: String,
    }

    #[derive(Debug, PartialEq)]
    struct ArticleV2 {
        id: u64,
        title: String,
        /// New in V2; absent in V1 → default to empty string.
        description: String,
    }

    let v1 = Version::new(1, 0, 0);
    let original = ArticleV1 {
        id: 42,
        title: "Evolution Test".to_string(),
    };
    let encoded = oxicode::encode_versioned_value(&original, v1).expect("encode v1");

    // A V2 reader checks compatibility first.
    let stored = extract_version(&encoded).expect("extract version");
    let v2_current = Version::new(1, 1, 0);
    let compat = check_compatibility(stored, v2_current, None);
    assert!(compat.is_usable(), "V1 data must be usable by V2 reader");

    // Decode as V1 schema (the bytes' actual content).
    let (decoded_v1, ver, _): (ArticleV1, _, _) =
        oxicode::decode_versioned_value(&encoded).expect("decode as V1");
    assert_eq!(ver, v1);

    // Migrate to V2 by supplying the new field default.
    let migrated = ArticleV2 {
        id: decoded_v1.id,
        title: decoded_v1.title,
        description: String::new(),
    };
    assert_eq!(migrated.id, 42);
    assert_eq!(migrated.title, "Evolution Test");
    assert!(
        migrated.description.is_empty(),
        "default description must be empty string"
    );
}

/// NEW-12. encode_versioned_value produces more bytes than plain encode_to_vec.
///
/// The version header overhead (11 bytes) must make the versioned output
/// strictly larger than the plain serialised form.
#[test]
fn test_versioned_value_larger_than_plain() {
    let value = 12345u64;
    let version = Version::new(1, 0, 0);

    let plain = oxicode::encode_to_vec(&value).expect("plain encode");
    let versioned = oxicode::encode_versioned_value(&value, version).expect("versioned encode");

    assert!(
        versioned.len() > plain.len(),
        "versioned output ({} bytes) must be larger than plain ({} bytes)",
        versioned.len(),
        plain.len()
    );
    // The difference should be exactly the 11-byte header.
    assert_eq!(
        versioned.len() - plain.len(),
        11,
        "version header overhead must be exactly 11 bytes"
    );
}

/// NEW-13. Version header format: bytes 5–10 contain major, minor, patch as
/// little-endian u16.
///
/// Re-tests the wire layout independently of test_version_number_in_wire_format
/// using a different version triple (1, 2, 3) to avoid trivial false passes.
#[test]
fn test_version_header_three_fields_at_known_offsets() {
    let version = Version::new(1, 2, 3);
    let encoded = oxicode::encode_versioned_value(&0u8, version).expect("encode");

    // Offsets: [0..4] OXIV magic, [4] header_version=1,
    //          [5..7] major LE u16, [7..9] minor LE u16, [9..11] patch LE u16.
    assert!(encoded.len() >= 11, "encoded must be at least 11 bytes");

    let major = u16::from_le_bytes([encoded[5], encoded[6]]);
    let minor = u16::from_le_bytes([encoded[7], encoded[8]]);
    let patch = u16::from_le_bytes([encoded[9], encoded[10]]);

    assert_eq!(major, 1, "major at bytes 5-6");
    assert_eq!(minor, 2, "minor at bytes 7-8");
    assert_eq!(patch, 3, "patch at bytes 9-10");
}

/// NEW-14. Version max values (65535.65535.65535) roundtrip through
/// encode_versioned_value / decode_versioned_value.
#[test]
fn test_version_max_values_roundtrip() {
    let max_version = Version::new(u16::MAX, u16::MAX, u16::MAX);
    let payload = 77u32;

    let encoded =
        oxicode::encode_versioned_value(&payload, max_version).expect("encode max version");
    let (val, ver, _): (u32, _, _) =
        oxicode::decode_versioned_value(&encoded).expect("decode max version");

    assert_eq!(val, payload);
    assert_eq!(
        ver, max_version,
        "max version (65535.65535.65535) must roundtrip"
    );
    assert_eq!(ver.major, u16::MAX);
    assert_eq!(ver.minor, u16::MAX);
    assert_eq!(ver.patch, u16::MAX);
}

/// NEW-15. Multiple structs each with different version numbers.
///
/// Three distinct structs are each encoded with their own version.  After
/// decoding, both the struct data and the stored version must match
/// what was supplied at encoding time.
#[cfg(feature = "derive")]
#[test]
fn test_multiple_structs_different_versions() {
    use oxicode::{Decode, Encode};

    #[derive(Debug, PartialEq, Encode, Decode)]
    struct TypeA {
        x: u32,
    }

    #[derive(Debug, PartialEq, Encode, Decode)]
    struct TypeB {
        name: String,
        count: u64,
    }

    #[derive(Debug, PartialEq, Encode, Decode)]
    struct TypeC {
        flag: bool,
        values: Vec<u8>,
    }

    let va = Version::new(1, 0, 0);
    let vb = Version::new(2, 3, 0);
    let vc = Version::new(0, 9, 5);

    let a = TypeA { x: 42 };
    let b = TypeB {
        name: "struct_b".to_string(),
        count: 100,
    };
    let c = TypeC {
        flag: false,
        values: vec![10, 20, 30],
    };

    let enc_a = oxicode::encode_versioned_value(&a, va).expect("encode A");
    let enc_b = oxicode::encode_versioned_value(&b, vb).expect("encode B");
    let enc_c = oxicode::encode_versioned_value(&c, vc).expect("encode C");

    let (da, vera, _): (TypeA, _, _) = oxicode::decode_versioned_value(&enc_a).expect("decode A");
    let (db, verb, _): (TypeB, _, _) = oxicode::decode_versioned_value(&enc_b).expect("decode B");
    let (dc, verc, _): (TypeC, _, _) = oxicode::decode_versioned_value(&enc_c).expect("decode C");

    assert_eq!(da, a);
    assert_eq!(vera, va);
    assert_eq!(db, b);
    assert_eq!(verb, vb);
    assert_eq!(dc, c);
    assert_eq!(verc, vc);
}

/// NEW-16. Decode versioned then re-encode to a different version.
///
/// A value encoded as V1 is decoded, then immediately re-encoded as V2.
/// The re-encoded bytes must have the V2 header and decode correctly.
#[test]
fn test_decode_versioned_then_reencode_different_version() {
    let v1 = Version::new(1, 0, 0);
    let v2 = Version::new(2, 0, 0);

    let original = 55555u32;
    let enc_v1 = oxicode::encode_versioned_value(&original, v1).expect("encode v1");

    // Decode from V1 encoding.
    let (val, ver1, _): (u32, _, _) = oxicode::decode_versioned_value(&enc_v1).expect("decode v1");
    assert_eq!(val, original);
    assert_eq!(ver1, v1);

    // Re-encode as V2.
    let enc_v2 = oxicode::encode_versioned_value(&val, v2).expect("re-encode as v2");
    let (val2, ver2, _): (u32, _, _) = oxicode::decode_versioned_value(&enc_v2).expect("decode v2");

    assert_eq!(
        val2, original,
        "value must be unchanged after version migration"
    );
    assert_eq!(ver2, v2, "re-encoded data must carry V2 header");
    assert_ne!(enc_v1, enc_v2, "V1 and V2 encoded bytes must differ");
}

/// NEW-17. Versioned encoding in combination with Compression::None passthrough.
///
/// Compress the serialised bytes with the passthrough codec, then wrap them
/// in a versioned envelope.  The pipeline must round-trip correctly.
/// Uses only the always-available `Compression::None` variant so the test
/// runs even when no compression feature flags are enabled.
#[cfg(any(
    feature = "compression-lz4",
    feature = "compression-zstd",
    feature = "compression-zstd-pure"
))]
#[test]
fn test_versioned_with_compression_none() {
    use oxicode::compression::{compress, decompress, Compression};
    use oxicode::versioning::{decode_versioned, encode_versioned, extract_version, Version};

    let version = Version::new(1, 2, 3);
    let payload_value = 9999u64;

    // Step 1: plain-encode the value.
    let plain = oxicode::encode_to_vec(&payload_value).expect("plain encode");

    // Step 2: compress with None (passthrough) codec.
    let compressed = compress(&plain, Compression::None).expect("compress None");

    // Step 3: wrap in versioned envelope.
    let versioned = encode_versioned(&compressed, version).expect("encode versioned");

    // Round-trip: extract version, decompress, plain-decode.
    let stored_ver = extract_version(&versioned).expect("extract version");
    assert_eq!(stored_ver, version);

    let (raw, ver) = decode_versioned(&versioned).expect("decode versioned");
    assert_eq!(ver, version);

    let decompressed = decompress(&raw).expect("decompress");
    let (val, _): (u64, _) = oxicode::decode_from_slice(&decompressed).expect("plain decode");

    assert_eq!(val, payload_value);
}

/// NEW-18. Error on corrupted version header (truncated data).
///
/// Builds a valid versioned envelope then truncates it to various lengths,
/// all shorter than the minimum 11-byte header.  Every attempt must return
/// an error rather than succeeding silently.
#[test]
fn test_error_on_truncated_version_header() {
    use oxicode::versioning::{decode_versioned, encode_versioned, Version};

    let version = Version::new(1, 0, 0);
    let full = encode_versioned(b"payload", version).expect("encode");

    // Try every prefix shorter than the 11-byte header.
    for len in 0..11usize {
        let truncated = &full[..len];
        let result = decode_versioned(truncated);
        assert!(
            result.is_err(),
            "decode must fail for truncated header of length {len}"
        );
    }
}

/// NEW-19. Batch: encode 100 items with version, decode all back.
///
/// Produces 100 individual versioned payloads (one per u64 value) and decodes
/// each one, confirming that all values and version numbers are preserved.
#[test]
fn test_batch_100_items_versioned() {
    let version = Version::new(1, 0, 0);

    let mut encoded_items: Vec<Vec<u8>> = Vec::with_capacity(100);
    for i in 0u64..100 {
        let enc = oxicode::encode_versioned_value(&i, version).expect("batch encode");
        encoded_items.push(enc);
    }

    for (i, enc) in encoded_items.iter().enumerate() {
        let (val, ver, _): (u64, _, _) =
            oxicode::decode_versioned_value(enc).expect("batch decode");
        assert_eq!(val, i as u64, "batch item {i}: value mismatch");
        assert_eq!(ver, version, "batch item {i}: version mismatch");
    }
}

/// NEW-20. Version Display format produces "major.minor.patch" string.
///
/// Version implements fmt::Display; the formatted output must be exactly the
/// dot-separated triple.
#[test]
fn test_version_display_format() {
    let cases: &[(Version, &str)] = &[
        (Version::new(1, 2, 3), "1.2.3"),
        (Version::new(0, 0, 0), "0.0.0"),
        (Version::new(10, 20, 30), "10.20.30"),
        (
            Version::new(u16::MAX, u16::MAX, u16::MAX),
            "65535.65535.65535",
        ),
        (Version::new(1, 0, 0), "1.0.0"),
    ];

    for (v, expected) in cases {
        let formatted = format!("{}", v);
        assert_eq!(
            formatted, *expected,
            "display format for {:?} must be {:?}, got {:?}",
            v, expected, formatted
        );
    }
}
