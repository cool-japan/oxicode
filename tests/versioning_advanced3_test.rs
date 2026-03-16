//! Advanced versioning tests — third batch (22 tests).
//!
//! Covers scenarios not exercised by the first two advanced batches:
//! Version::zero() fields, Version::from_bytes with exact 6-byte slice,
//! Version::from_bytes with insufficient bytes, Version default field access,
//! VersionedHeader default construction, VersionedHeader equality,
//! VERSIONED_MAGIC byte values, extract_version, is_versioned on encoded value,
//! decode_versioned raw payload equality, encode_versioned_value bool roundtrip,
//! encode_versioned_value tuple roundtrip, encode_versioned_value i128 roundtrip,
//! multiple-struct-fields struct roundtrip with version bump,
//! patch-only bump is Compatible, same-version decode_versioned_with_check returns Compatible,
//! check_compatibility identical zero versions, Version::is_breaking_change_from for 0.x minor bump,
//! Version::is_compatible_with for same major, can_migrate same version, and
//! migration_path one-major-bump returns empty.
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
    can_migrate, check_compatibility, decode_versioned, decode_versioned_with_check,
    encode_versioned, extract_version, is_versioned, migration_path, CompatibilityLevel, Version,
    VersionedHeader, VERSIONED_MAGIC,
};
use oxicode::{decode_versioned_value, encode_versioned_value, Decode, Encode};

// ─────────────────────────────────────────────────────────────────────────────
// Shared types
// ─────────────────────────────────────────────────────────────────────────────

#[derive(Encode, Decode, PartialEq, Debug, Clone)]
struct MultiFieldRecord {
    version_field: u32,
    schema_rev: u16,
    name: String,
    active: bool,
    score: f64,
}

#[derive(Encode, Decode, PartialEq, Debug, Clone)]
struct TupleWrapper(u64, u32, i16);

// ─────────────────────────────────────────────────────────────────────────────
// Test 1 — Version::zero() has all fields equal to zero
// ─────────────────────────────────────────────────────────────────────────────

#[test]
fn test_version_zero_has_all_zero_fields() {
    let z = Version::zero();
    assert_eq!(z.major, 0, "major must be 0");
    assert_eq!(z.minor, 0, "minor must be 0");
    assert_eq!(z.patch, 0, "patch must be 0");
    assert_eq!(z, Version::new(0, 0, 0), "zero() must equal new(0,0,0)");
}

// ─────────────────────────────────────────────────────────────────────────────
// Test 2 — Version::from_bytes with exact 6-byte slice succeeds
// ─────────────────────────────────────────────────────────────────────────────

#[test]
fn test_version_from_bytes_exact_slice_succeeds() {
    let v = Version::new(3, 7, 11);
    let bytes = v.to_bytes(); // exactly 6 bytes
    let recovered = Version::from_bytes(&bytes).expect("from_bytes must succeed for 6 bytes");
    assert_eq!(
        recovered, v,
        "version must survive to_bytes/from_bytes roundtrip"
    );
}

// ─────────────────────────────────────────────────────────────────────────────
// Test 3 — Version::from_bytes with fewer than 6 bytes returns None
// ─────────────────────────────────────────────────────────────────────────────

#[test]
fn test_version_from_bytes_insufficient_returns_none() {
    let short: &[u8] = &[0x01, 0x00, 0x02]; // only 3 bytes
    let result = Version::from_bytes(short);
    assert!(
        result.is_none(),
        "from_bytes with fewer than 6 bytes must return None"
    );
}

// ─────────────────────────────────────────────────────────────────────────────
// Test 4 — Version little-endian byte layout is correct
// ─────────────────────────────────────────────────────────────────────────────

#[test]
fn test_version_bytes_little_endian_layout() {
    let v = Version::new(1, 2, 3);
    let bytes = v.to_bytes();
    // major at bytes[0..2], minor at bytes[2..4], patch at bytes[4..6], all little-endian
    assert_eq!(
        u16::from_le_bytes([bytes[0], bytes[1]]),
        1u16,
        "major bytes LE"
    );
    assert_eq!(
        u16::from_le_bytes([bytes[2], bytes[3]]),
        2u16,
        "minor bytes LE"
    );
    assert_eq!(
        u16::from_le_bytes([bytes[4], bytes[5]]),
        3u16,
        "patch bytes LE"
    );
}

// ─────────────────────────────────────────────────────────────────────────────
// Test 5 — VersionedHeader default uses Version::zero()
// ─────────────────────────────────────────────────────────────────────────────

#[test]
fn test_versioned_header_default_uses_zero_version() {
    let hdr: VersionedHeader = Default::default();
    assert_eq!(
        hdr.version(),
        Version::zero(),
        "default VersionedHeader must embed Version::zero()"
    );
    assert_eq!(
        hdr.header_version(),
        1u8,
        "default header_version must be 1"
    );
}

// ─────────────────────────────────────────────────────────────────────────────
// Test 6 — Two VersionedHeader with same version are equal
// ─────────────────────────────────────────────────────────────────────────────

#[test]
fn test_versioned_header_equality_same_version() {
    let v = Version::new(4, 8, 16);
    let h1 = VersionedHeader::new(v);
    let h2 = VersionedHeader::new(v);
    assert_eq!(
        h1, h2,
        "two VersionedHeaders with the same version must be equal"
    );
}

// ─────────────────────────────────────────────────────────────────────────────
// Test 7 — VERSIONED_MAGIC spells "OXIV" in ASCII
// ─────────────────────────────────────────────────────────────────────────────

#[test]
fn test_versioned_magic_ascii_oxiv() {
    assert_eq!(
        VERSIONED_MAGIC,
        [b'O', b'X', b'I', b'V'],
        "magic must be ASCII 'OXIV'"
    );
    assert_eq!(VERSIONED_MAGIC.len(), 4, "magic must be 4 bytes");
}

// ─────────────────────────────────────────────────────────────────────────────
// Test 8 — extract_version returns the exact version embedded in the header
// ─────────────────────────────────────────────────────────────────────────────

#[test]
fn test_extract_version_returns_embedded_version() {
    let version = Version::new(9, 14, 2);
    let payload = b"content does not matter";
    let encoded = encode_versioned(payload, version).expect("encode_versioned failed");
    let extracted = extract_version(&encoded).expect("extract_version must succeed");
    assert_eq!(
        extracted, version,
        "extract_version must return the version used during encoding"
    );
}

// ─────────────────────────────────────────────────────────────────────────────
// Test 9 — is_versioned returns true for output of encode_versioned_value
// ─────────────────────────────────────────────────────────────────────────────

#[test]
fn test_is_versioned_true_for_encode_versioned_value_output() {
    let version = Version::new(1, 0, 0);
    let encoded =
        encode_versioned_value(&12345u32, version).expect("encode_versioned_value failed");
    assert!(
        is_versioned(&encoded),
        "output of encode_versioned_value must be recognised as versioned"
    );
}

// ─────────────────────────────────────────────────────────────────────────────
// Test 10 — decode_versioned payload bytes equal original raw bytes
// ─────────────────────────────────────────────────────────────────────────────

#[test]
fn test_decode_versioned_payload_equals_original_bytes() {
    let original: &[u8] = b"raw bytes that must survive intact";
    let version = Version::new(2, 0, 0);
    let encoded = encode_versioned(original, version).expect("encode_versioned failed");
    let (payload, ver) = decode_versioned(&encoded).expect("decode_versioned failed");
    assert_eq!(
        payload.as_slice(),
        original,
        "decoded payload must be byte-identical to original"
    );
    assert_eq!(
        ver, version,
        "decoded version must equal the encoding version"
    );
}

// ─────────────────────────────────────────────────────────────────────────────
// Test 11 — encode_versioned_value / decode_versioned_value roundtrip for bool
// ─────────────────────────────────────────────────────────────────────────────

#[test]
fn test_versioned_value_bool_roundtrip() {
    let version = Version::new(1, 0, 0);

    for &b in &[true, false] {
        let encoded = encode_versioned_value(&b, version).expect("encode bool failed");
        let (decoded, ver, _): (bool, _, _) =
            decode_versioned_value(&encoded).expect("decode bool failed");
        assert_eq!(decoded, b, "bool must survive versioned roundtrip");
        assert_eq!(ver, version, "version must survive roundtrip");
    }
}

// ─────────────────────────────────────────────────────────────────────────────
// Test 12 — encode_versioned_value / decode_versioned_value roundtrip for tuple-struct
// ─────────────────────────────────────────────────────────────────────────────

#[test]
fn test_versioned_value_tuple_struct_roundtrip() {
    let original = TupleWrapper(0xDEAD_BEEF_CAFE_BABEu64, 42u32, -128i16);
    let version = Version::new(1, 2, 0);

    let encoded = encode_versioned_value(&original, version).expect("encode TupleWrapper failed");
    let (decoded, ver, _): (TupleWrapper, _, _) =
        decode_versioned_value(&encoded).expect("decode TupleWrapper failed");

    assert_eq!(
        decoded, original,
        "TupleWrapper must survive versioned roundtrip"
    );
    assert_eq!(ver, version, "version must be preserved");
}

// ─────────────────────────────────────────────────────────────────────────────
// Test 13 — encode_versioned_value / decode_versioned_value roundtrip for i128
// ─────────────────────────────────────────────────────────────────────────────

#[test]
fn test_versioned_value_i128_roundtrip() {
    let value: i128 = i128::MIN;
    let version = Version::new(3, 0, 0);

    let encoded = encode_versioned_value(&value, version).expect("encode i128::MIN failed");
    let (decoded, ver, _): (i128, _, _) =
        decode_versioned_value(&encoded).expect("decode i128::MIN failed");

    assert_eq!(decoded, value, "i128::MIN must survive versioned roundtrip");
    assert_eq!(ver, version);
}

// ─────────────────────────────────────────────────────────────────────────────
// Test 14 — multiple-field struct round-trips with a v2 version tag
// ─────────────────────────────────────────────────────────────────────────────

#[test]
fn test_multi_field_struct_roundtrip_v2() {
    let original = MultiFieldRecord {
        version_field: 2,
        schema_rev: 7,
        name: String::from("schema-evolution"),
        active: true,
        score: std::f64::consts::PI,
    };
    let version = Version::new(2, 0, 0);

    let encoded =
        encode_versioned_value(&original, version).expect("encode MultiFieldRecord failed");
    let (decoded, ver, _): (MultiFieldRecord, _, _) =
        decode_versioned_value(&encoded).expect("decode MultiFieldRecord failed");

    assert_eq!(decoded, original, "MultiFieldRecord must survive roundtrip");
    assert_eq!(ver, version, "v2.0.0 version tag must be preserved");
    assert_eq!(decoded.schema_rev, 7, "schema_rev field must be intact");
    assert_eq!(decoded.version_field, 2, "version_field must be intact");
}

// ─────────────────────────────────────────────────────────────────────────────
// Test 15 — patch-only bump yields Compatible
// ─────────────────────────────────────────────────────────────────────────────

#[test]
fn test_patch_only_bump_is_compatible() {
    let data_ver = Version::new(1, 3, 0);
    let current = Version::new(1, 3, 9);
    let level = check_compatibility(data_ver, current, None);
    assert_eq!(
        level,
        CompatibilityLevel::Compatible,
        "patch-only bump (same major.minor) must be Compatible"
    );
}

// ─────────────────────────────────────────────────────────────────────────────
// Test 16 — decode_versioned_with_check same-version returns Compatible
// ─────────────────────────────────────────────────────────────────────────────

#[test]
fn test_decode_versioned_with_check_same_version_returns_compatible() {
    let version = Version::new(1, 5, 2);
    let payload = b"exact version match payload";

    let encoded = encode_versioned(payload, version).expect("encode_versioned failed");
    let (recovered_payload, recovered_ver, compat) =
        decode_versioned_with_check(&encoded, version, None)
            .expect("decode_versioned_with_check must succeed");

    assert_eq!(
        recovered_payload.as_slice(),
        payload,
        "payload must be intact"
    );
    assert_eq!(recovered_ver, version, "version must match");
    assert_eq!(
        compat,
        CompatibilityLevel::Compatible,
        "same version must yield Compatible"
    );
}

// ─────────────────────────────────────────────────────────────────────────────
// Test 17 — check_compatibility with both versions at 0.0.0 is Compatible
// ─────────────────────────────────────────────────────────────────────────────

#[test]
fn test_check_compatibility_both_zero_versions_is_compatible() {
    let z = Version::zero();
    let level = check_compatibility(z, z, None);
    assert_eq!(
        level,
        CompatibilityLevel::Compatible,
        "two identical 0.0.0 versions must be Compatible"
    );
}

// ─────────────────────────────────────────────────────────────────────────────
// Test 18 — Version::is_breaking_change_from for 0.x minor bump
// ─────────────────────────────────────────────────────────────────────────────

#[test]
fn test_is_breaking_change_from_0x_minor_bump() {
    let old = Version::new(0, 1, 0);
    let new = Version::new(0, 2, 0);
    assert!(
        new.is_breaking_change_from(&old),
        "0.2.0 must be a breaking change from 0.1.0 in pre-1.0 semver"
    );
    // Patch bump in 0.x is NOT a breaking change
    let patch = Version::new(0, 1, 9);
    assert!(
        !patch.is_breaking_change_from(&old),
        "0.1.9 must not be a breaking change from 0.1.0"
    );
}

// ─────────────────────────────────────────────────────────────────────────────
// Test 19 — Version::is_compatible_with for same major (post-1.0)
// ─────────────────────────────────────────────────────────────────────────────

#[test]
fn test_is_compatible_with_same_major_post_1_0() {
    let a = Version::new(2, 0, 0);
    let b = Version::new(2, 99, 99);
    assert!(
        a.is_compatible_with(&b),
        "2.0.0 must be compatible with 2.99.99 (same major)"
    );
    assert!(
        b.is_compatible_with(&a),
        "2.99.99 must be compatible with 2.0.0 (same major)"
    );
    // Different major is incompatible
    let c = Version::new(3, 0, 0);
    assert!(
        !a.is_compatible_with(&c),
        "2.0.0 must not be compatible with 3.0.0"
    );
}

// ─────────────────────────────────────────────────────────────────────────────
// Test 20 — can_migrate returns true for same version (trivial migration)
// ─────────────────────────────────────────────────────────────────────────────

#[test]
fn test_can_migrate_same_version() {
    let v = Version::new(5, 3, 2);
    assert!(
        can_migrate(v, v),
        "migrating a version to itself must always be allowed"
    );
}

// ─────────────────────────────────────────────────────────────────────────────
// Test 21 — migration_path for one major bump returns empty (direct migration)
// ─────────────────────────────────────────────────────────────────────────────

#[test]
fn test_migration_path_one_major_bump_is_empty() {
    let from = Version::new(1, 0, 0);
    let to = Version::new(2, 0, 0);
    let path = migration_path(from, to);
    assert!(
        path.is_empty(),
        "1.x -> 2.x is a direct single-step migration; no intermediates needed"
    );
}

// ─────────────────────────────────────────────────────────────────────────────
// Test 22 — encode_versioned_value consumed bytes matches encoded length
// ─────────────────────────────────────────────────────────────────────────────

#[test]
fn test_decode_versioned_value_consumed_equals_encoded_length() {
    let value: u64 = 123_456_789u64;
    let version = Version::new(1, 0, 0);

    let encoded = encode_versioned_value(&value, version).expect("encode_versioned_value failed");
    let total_len = encoded.len();

    let (decoded, ver, consumed): (u64, _, _) =
        decode_versioned_value(&encoded).expect("decode_versioned_value failed");

    assert_eq!(decoded, value, "decoded value must match original");
    assert_eq!(ver, version, "version must be preserved");
    // consumed is the total number of bytes read (header + payload).
    assert_eq!(
        consumed, total_len,
        "consumed must equal the full encoded length"
    );
    assert!(
        consumed > 0,
        "consumed must be greater than zero for a non-trivial value"
    );
}
