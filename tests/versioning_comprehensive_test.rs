//! Comprehensive versioning tests covering Version semantics, encode/decode roundtrips,
//! migration paths, compatibility rules, header format, and byte-level inspection.
//!
//! These tests are designed to be distinct from versioning_advanced_test.rs and
//! versioning_basic_test.rs, exercising deeper API surface and edge cases.

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
    VERSIONED_MAGIC,
};
use oxicode::{decode_versioned_value, encode_versioned_value, Decode, Encode};

// ─────────────────────────────────────────────────────────────────────────────
// Test 1: Version ordering — 1.0.0 < 2.0.0
// ─────────────────────────────────────────────────────────────────────────────
#[test]
fn test_version_major_ordering_1_lt_2() {
    let v1 = Version::new(1, 0, 0);
    let v2 = Version::new(2, 0, 0);
    assert!(v1 < v2, "1.0.0 must be strictly less than 2.0.0");
    assert!(v2 > v1, "2.0.0 must be strictly greater than 1.0.0");
    assert!(v1 != v2, "1.0.0 must not equal 2.0.0");
}

// ─────────────────────────────────────────────────────────────────────────────
// Test 2: Version ordering — 1.0.0 < 1.1.0
// ─────────────────────────────────────────────────────────────────────────────
#[test]
fn test_version_minor_ordering_1_0_0_lt_1_1_0() {
    let v1 = Version::new(1, 0, 0);
    let v2 = Version::new(1, 1, 0);
    assert!(v1 < v2, "1.0.0 must be less than 1.1.0");
    assert!(v2 > v1, "1.1.0 must be greater than 1.0.0");
    // Both share the same major: minor decides order
    assert_eq!(v1.major, v2.major, "major versions must be equal");
}

// ─────────────────────────────────────────────────────────────────────────────
// Test 3: Version ordering — 1.0.0 < 1.0.1
// ─────────────────────────────────────────────────────────────────────────────
#[test]
fn test_version_patch_ordering_1_0_0_lt_1_0_1() {
    let v1 = Version::new(1, 0, 0);
    let v2 = Version::new(1, 0, 1);
    assert!(v1 < v2, "1.0.0 must be less than 1.0.1");
    assert_eq!(v1.major, v2.major, "major must be equal");
    assert_eq!(v1.minor, v2.minor, "minor must be equal");
    assert!(v1.patch < v2.patch, "patch alone differentiates them");
}

// ─────────────────────────────────────────────────────────────────────────────
// Test 4: Version equality — 1.2.3 == 1.2.3
// ─────────────────────────────────────────────────────────────────────────────
#[test]
fn test_version_equality_1_2_3() {
    let a = Version::new(1, 2, 3);
    let b = Version::new(1, 2, 3);
    assert_eq!(a, b, "identical versions must be equal");
    assert!(a >= b, "equal versions must not satisfy less-than");
    assert!(a <= b, "equal versions must not satisfy greater-than");
}

// ─────────────────────────────────────────────────────────────────────────────
// Test 5: encode_versioned_value u32 roundtrip
// ─────────────────────────────────────────────────────────────────────────────
#[test]
fn test_encode_versioned_value_u32_roundtrip() {
    let value: u32 = 123_456;
    let version = Version::new(1, 0, 0);

    let encoded = encode_versioned_value(&value, version).expect("encode u32 failed");
    let (decoded, recovered, _consumed): (u32, _, _) =
        decode_versioned_value(&encoded).expect("decode u32 failed");

    assert_eq!(decoded, value, "u32 value must survive roundtrip");
    assert_eq!(recovered, version, "version must survive roundtrip");
}

// ─────────────────────────────────────────────────────────────────────────────
// Test 6: encode_versioned_value String roundtrip
// ─────────────────────────────────────────────────────────────────────────────
#[test]
fn test_encode_versioned_value_string_roundtrip() {
    let value = String::from("oxicode versioning test string");
    let version = Version::new(2, 3, 1);

    let encoded = encode_versioned_value(&value, version).expect("encode String failed");
    let (decoded, recovered, _): (String, _, _) =
        decode_versioned_value(&encoded).expect("decode String failed");

    assert_eq!(decoded, value, "String must survive versioned roundtrip");
    assert_eq!(recovered, version);
}

// ─────────────────────────────────────────────────────────────────────────────
// Test 7: encode_versioned_value struct roundtrip
// ─────────────────────────────────────────────────────────────────────────────
#[cfg(feature = "derive")]
mod struct_roundtrip_tests {
    use super::*;

    #[derive(Encode, Decode, PartialEq, Debug)]
    struct MetricRecord {
        sensor_id: u64,
        reading: f32,
        valid: bool,
    }

    #[test]
    fn test_encode_versioned_value_struct_roundtrip() {
        let original = MetricRecord {
            sensor_id: 0xDEAD_BEEF,
            reading: std::f32::consts::PI,
            valid: true,
        };
        let version = Version::new(1, 2, 0);

        let encoded =
            encode_versioned_value(&original, version).expect("encode MetricRecord failed");
        let (decoded, recovered, _): (MetricRecord, _, _) =
            decode_versioned_value(&encoded).expect("decode MetricRecord failed");

        assert_eq!(decoded, original, "struct must survive versioned roundtrip");
        assert_eq!(recovered, version);
    }

    // ─────────────────────────────────────────────────────────────────────────
    // Test 8: encode_versioned_value with different version tags
    // ─────────────────────────────────────────────────────────────────────────
    #[test]
    fn test_encode_versioned_value_different_version_tags() {
        let value: u64 = 42;
        let versions = [
            Version::new(0, 1, 0),
            Version::new(1, 0, 0),
            Version::new(1, 5, 3),
            Version::new(3, 0, 0),
        ];

        for &ver in &versions {
            let encoded =
                encode_versioned_value(&value, ver).expect("encode with version tag failed");
            let (decoded, recovered, _): (u64, _, _) =
                decode_versioned_value(&encoded).expect("decode with version tag failed");
            assert_eq!(decoded, value, "value must match for version {ver}");
            assert_eq!(recovered, ver, "version tag must be preserved for {ver}");
        }
    }
}

// ─────────────────────────────────────────────────────────────────────────────
// Test 9: can_migrate — same version returns true
// ─────────────────────────────────────────────────────────────────────────────
#[test]
fn test_can_migrate_same_version() {
    let v = Version::new(1, 0, 0);
    assert!(
        can_migrate(v, v),
        "migration from a version to itself must be allowed"
    );
}

// ─────────────────────────────────────────────────────────────────────────────
// Test 10: can_migrate — minor bump within same major returns true
// ─────────────────────────────────────────────────────────────────────────────
#[test]
fn test_can_migrate_minor_bump_same_major() {
    let from = Version::new(1, 0, 0);
    let to = Version::new(1, 9, 0);
    assert!(
        can_migrate(from, to),
        "minor-bump migration within the same major must be allowed"
    );
}

// ─────────────────────────────────────────────────────────────────────────────
// Test 11: can_migrate — major bump forward returns true; backward returns false
// ─────────────────────────────────────────────────────────────────────────────
#[test]
fn test_can_migrate_major_bump_directions() {
    let v1 = Version::new(1, 0, 0);
    let v2 = Version::new(2, 0, 0);

    assert!(
        can_migrate(v1, v2),
        "forward major-bump migration must be allowed"
    );
    assert!(
        !can_migrate(v2, v1),
        "backward major-bump migration must be rejected"
    );
}

// ─────────────────────────────────────────────────────────────────────────────
// Test 12: migration_path — same version yields empty path
// ─────────────────────────────────────────────────────────────────────────────
#[test]
fn test_migration_path_same_version_is_empty() {
    let v = Version::new(1, 0, 0);
    let path = migration_path(v, v);
    assert!(
        path.is_empty(),
        "migration path from a version to itself must be empty"
    );
}

// ─────────────────────────────────────────────────────────────────────────────
// Test 13: migration_path — adjacent major versions (1→2) yields empty path
// ─────────────────────────────────────────────────────────────────────────────
#[test]
fn test_migration_path_adjacent_major_is_direct() {
    let from = Version::new(1, 0, 0);
    let to = Version::new(2, 0, 0);
    let path = migration_path(from, to);
    assert!(
        path.is_empty(),
        "direct adjacent major migration must have no intermediates"
    );
}

// ─────────────────────────────────────────────────────────────────────────────
// Test 14: check_compatibility — same version is Compatible
// ─────────────────────────────────────────────────────────────────────────────
#[test]
fn test_check_compatibility_same_version_is_compatible() {
    let v = Version::new(1, 0, 0);
    let level = check_compatibility(v, v, None);
    assert_eq!(
        level,
        CompatibilityLevel::Compatible,
        "same-version check must return Compatible"
    );
    assert!(level.is_usable());
    assert!(level.is_fully_compatible());
}

// ─────────────────────────────────────────────────────────────────────────────
// Test 15: VersionedValue contains version header bytes (magic prefix)
// ─────────────────────────────────────────────────────────────────────────────
#[test]
fn test_versioned_value_contains_version_header() {
    let value: u32 = 7;
    let version = Version::new(1, 0, 0);
    let encoded = encode_versioned_value(&value, version).expect("encode failed");

    // The output must start with the OXIV magic bytes
    assert!(
        encoded.len() >= VERSIONED_MAGIC.len(),
        "encoded output must be at least as long as magic"
    );
    assert_eq!(
        &encoded[..4],
        &VERSIONED_MAGIC,
        "first 4 bytes must be OXIV magic"
    );
}

// ─────────────────────────────────────────────────────────────────────────────
// Test 16: encode_versioned byte format has version prefix detectable by is_versioned
// ─────────────────────────────────────────────────────────────────────────────
#[test]
fn test_encode_versioned_byte_format_detected_by_is_versioned() {
    let payload = b"binary payload for format test";
    let version = Version::new(2, 0, 0);

    let raw: &[u8] = payload;
    assert!(
        !is_versioned(raw),
        "raw bytes must not be detected as versioned"
    );

    let wrapped = encode_versioned(payload, version).expect("encode_versioned failed");
    assert!(
        is_versioned(&wrapped),
        "output of encode_versioned must be detected as versioned"
    );

    // Extracting the version must succeed and return the correct value
    let extracted = extract_version(&wrapped).expect("extract_version failed");
    assert_eq!(
        extracted, version,
        "extracted version must match encoded version"
    );
}

// ─────────────────────────────────────────────────────────────────────────────
// Test 17: Roundtrip 10 distinct versions — each encodes a u64 equal to its index
// ─────────────────────────────────────────────────────────────────────────────
#[test]
fn test_ten_versions_roundtrip_each() {
    let versions: [(u16, u16, u16); 10] = [
        (0, 1, 0),
        (0, 2, 0),
        (1, 0, 0),
        (1, 1, 0),
        (1, 2, 5),
        (2, 0, 0),
        (2, 3, 1),
        (5, 0, 0),
        (10, 0, 0),
        (u16::MAX, u16::MAX, u16::MAX),
    ];

    for (i, &(major, minor, patch)) in versions.iter().enumerate() {
        let ver = Version::new(major, minor, patch);
        let payload: u64 = i as u64 * 1000 + 7;

        let encoded =
            encode_versioned_value(&payload, ver).expect("encode in ten-version loop failed");
        let (decoded, recovered, _): (u64, _, _) =
            decode_versioned_value(&encoded).expect("decode in ten-version loop failed");

        assert_eq!(decoded, payload, "payload mismatch at index {i}");
        assert_eq!(recovered, ver, "version mismatch at index {i}");
    }
}

// ─────────────────────────────────────────────────────────────────────────────
// Test 18: Vec of versioned values — each element independently versioned
// ─────────────────────────────────────────────────────────────────────────────
#[test]
fn test_vec_of_versioned_values() {
    let version = Version::new(1, 0, 0);
    let items: Vec<u32> = (0..20).map(|i| i * i).collect();

    // Encode every element individually with a version header
    let encoded_items: Vec<Vec<u8>> = items
        .iter()
        .map(|v| encode_versioned_value(v, version).expect("encode item failed"))
        .collect();

    assert_eq!(encoded_items.len(), items.len());

    // Decode each and verify
    for (idx, (original, bytes)) in items.iter().zip(encoded_items.iter()).enumerate() {
        let (decoded, recovered, _): (u32, _, _) =
            decode_versioned_value(bytes).expect("decode item failed");
        assert_eq!(decoded, *original, "item {idx} value mismatch");
        assert_eq!(recovered, version, "item {idx} version mismatch");
    }
}

// ─────────────────────────────────────────────────────────────────────────────
// Test 19: Versioned struct evolution — V1 payload decodable by V1 reader
//          and version header correctly distinguishes the two schemas
// ─────────────────────────────────────────────────────────────────────────────
#[cfg(feature = "derive")]
mod evolution_test {
    use super::*;

    #[derive(Encode, Decode, PartialEq, Debug)]
    struct EventV1 {
        event_id: u32,
        timestamp: u64,
    }

    #[derive(Encode, Decode, PartialEq, Debug)]
    struct EventV1Reader {
        event_id: u32,
        timestamp: u64,
    }

    #[test]
    fn test_versioned_struct_evolution_v1_roundtrip() {
        let v1_event = EventV1 {
            event_id: 42,
            timestamp: 1_700_000_000,
        };
        let v1_ver = Version::new(1, 0, 0);
        let v2_ver = Version::new(2, 0, 0);

        // Encode as v1
        let v1_bytes = encode_versioned_value(&v1_event, v1_ver).expect("encode V1 event failed");

        // Extract version — must confirm v1
        let stored_ver = extract_version(&v1_bytes).expect("extract version failed");
        assert_eq!(stored_ver, v1_ver, "stored version must be v1");
        assert_ne!(stored_ver, v2_ver, "stored version must not be v2");

        // Decode back with exact type
        let (payload, _ver) = decode_versioned(&v1_bytes).expect("decode_versioned failed");
        let (decoded, _): (EventV1Reader, _) =
            oxicode::decode_from_slice(&payload).expect("decode EventV1Reader failed");

        assert_eq!(decoded.event_id, v1_event.event_id);
        assert_eq!(decoded.timestamp, v1_event.timestamp);
    }
}

// ─────────────────────────────────────────────────────────────────────────────
// Test 20: CompatibilityLevel — Debug/Display/method behaviour across all variants
// ─────────────────────────────────────────────────────────────────────────────
#[test]
fn test_compatibility_level_variants_debug_and_methods() {
    let levels = [
        CompatibilityLevel::Compatible,
        CompatibilityLevel::CompatibleWithWarnings,
        CompatibilityLevel::Incompatible,
    ];

    // Debug formatting must produce non-empty strings
    for level in &levels {
        let debug_str = format!("{level:?}");
        assert!(!debug_str.is_empty(), "Debug output must not be empty");
    }

    // is_usable(): Compatible and CompatibleWithWarnings are usable; Incompatible is not
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

    // is_fully_compatible(): only Compatible is fully compatible
    assert!(
        CompatibilityLevel::Compatible.is_fully_compatible(),
        "Compatible must be fully compatible"
    );
    assert!(
        !CompatibilityLevel::CompatibleWithWarnings.is_fully_compatible(),
        "CompatibleWithWarnings must not be fully compatible"
    );
    assert!(
        !CompatibilityLevel::Incompatible.is_fully_compatible(),
        "Incompatible must not be fully compatible"
    );

    // has_warnings(): only CompatibleWithWarnings has warnings
    assert!(
        !CompatibilityLevel::Compatible.has_warnings(),
        "Compatible must not have warnings"
    );
    assert!(
        CompatibilityLevel::CompatibleWithWarnings.has_warnings(),
        "CompatibleWithWarnings must have warnings"
    );
    assert!(
        !CompatibilityLevel::Incompatible.has_warnings(),
        "Incompatible must not have warnings"
    );

    // Verify check_compatibility produces each level in appropriate scenarios

    // Same version → Compatible
    let same = Version::new(1, 0, 0);
    assert_eq!(
        check_compatibility(same, same, None),
        CompatibilityLevel::Compatible
    );

    // Older minor data against newer reader → CompatibleWithWarnings
    let old_minor = Version::new(1, 0, 0);
    let new_minor = Version::new(1, 5, 0);
    assert_eq!(
        check_compatibility(old_minor, new_minor, None),
        CompatibilityLevel::CompatibleWithWarnings
    );

    // Different major → Incompatible
    let v1 = Version::new(1, 0, 0);
    let v2 = Version::new(2, 0, 0);
    assert_eq!(
        check_compatibility(v1, v2, None),
        CompatibilityLevel::Incompatible
    );

    // decode_versioned_with_check returns the correct CompatibilityLevel
    let payload = b"level verification payload";
    let data_ver = Version::new(1, 3, 0);
    let current_ver = Version::new(1, 5, 0);
    let wrapped =
        encode_versioned(payload, data_ver).expect("encode_versioned for level check failed");
    let (_, found_ver, level) = decode_versioned_with_check(&wrapped, current_ver, None)
        .expect("decode_versioned_with_check failed");
    assert_eq!(found_ver, data_ver);
    assert!(
        level.is_usable(),
        "data version 1.3.0 against reader 1.5.0 must be usable"
    );
}
