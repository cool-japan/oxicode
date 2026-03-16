//! Advanced backward/forward compatibility tests for OxiCode versioning (set 2).
//!
//! Covers 22 scenarios exercising Version, VersionedHeader, encode_versioned_value,
//! decode_versioned_value, decode_versioned_with_check, is_versioned, extract_version,
//! and check_compatibility.

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
use oxicode::{
    decode_from_slice, encode_to_vec,
    versioning::{
        decode_versioned, decode_versioned_with_check, encode_versioned, extract_version,
        is_versioned, CompatibilityLevel, Version, VersionedHeader,
    },
    Decode, Encode,
};

// ── Scenario 1 ───────────────────────────────────────────────────────────────
// Version::new(1, 0, 0) roundtrip via encode_versioned / decode_versioned
#[test]
fn test_version_1_0_0_roundtrip() {
    let version = Version::new(1, 0, 0);
    let payload = b"hello";
    let encoded = encode_versioned(payload, version).expect("encode_versioned failed");
    let (decoded_payload, decoded_ver) =
        decode_versioned(&encoded).expect("decode_versioned failed");
    assert_eq!(decoded_payload.as_slice(), payload.as_slice());
    assert_eq!(decoded_ver, version);
}

// ── Scenario 2 ───────────────────────────────────────────────────────────────
// Version::new(2, 3, 4) roundtrip
#[test]
fn test_version_2_3_4_roundtrip() {
    let version = Version::new(2, 3, 4);
    let payload = b"data";
    let encoded = encode_versioned(payload, version).expect("encode_versioned failed");
    let (decoded_payload, decoded_ver) =
        decode_versioned(&encoded).expect("decode_versioned failed");
    assert_eq!(decoded_payload.as_slice(), payload.as_slice());
    assert_eq!(decoded_ver, version);
}

// ── Scenario 3 ───────────────────────────────────────────────────────────────
// Version::new(0, 0, 1) roundtrip (patch-only)
#[test]
fn test_version_0_0_1_patch_only_roundtrip() {
    let version = Version::new(0, 0, 1);
    let payload = b"patch";
    let encoded = encode_versioned(payload, version).expect("encode_versioned failed");
    let (decoded_payload, decoded_ver) =
        decode_versioned(&encoded).expect("decode_versioned failed");
    assert_eq!(decoded_payload.as_slice(), payload.as_slice());
    assert_eq!(decoded_ver, version);
}

// ── Scenario 4 ───────────────────────────────────────────────────────────────
// encode_versioned_value on u32 — verify version in header
#[test]
fn test_encode_versioned_value_u32_version_in_header() {
    let version = Version::new(1, 0, 0);
    let value: u32 = 42;
    let payload_bytes = encode_to_vec(&value).expect("encode_to_vec failed");
    let encoded = encode_versioned(&payload_bytes, version).expect("encode_versioned failed");
    // The header magic must be present
    assert!(is_versioned(&encoded));
    // The version must be extractable
    let extracted = extract_version(&encoded).expect("extract_version failed");
    assert_eq!(extracted, version);
}

// ── Scenario 5 ───────────────────────────────────────────────────────────────
// encode_versioned_value on String roundtrip
#[test]
fn test_encode_versioned_value_string_roundtrip() {
    let version = Version::new(1, 2, 0);
    let original = String::from("OxiCode versioning test");
    let payload_bytes = encode_to_vec(&original).expect("encode_to_vec failed");
    let encoded = encode_versioned(&payload_bytes, version).expect("encode_versioned failed");
    let (raw_payload, ver) = decode_versioned(&encoded).expect("decode_versioned failed");
    let (decoded_string, _consumed): (String, usize) =
        decode_from_slice(&raw_payload).expect("decode_from_slice failed");
    assert_eq!(decoded_string, original);
    assert_eq!(ver, version);
}

// ── Scenario 6 ───────────────────────────────────────────────────────────────
// encode_versioned_value on Vec<u8> roundtrip
#[test]
fn test_encode_versioned_value_vec_u8_roundtrip() {
    let version = Version::new(3, 0, 0);
    let original: Vec<u8> = vec![0x01, 0x02, 0x03, 0xFF, 0xFE];
    let payload_bytes = encode_to_vec(&original).expect("encode_to_vec failed");
    let encoded = encode_versioned(&payload_bytes, version).expect("encode_versioned failed");
    let (raw_payload, ver) = decode_versioned(&encoded).expect("decode_versioned failed");
    let (decoded_vec, _consumed): (Vec<u8>, usize) =
        decode_from_slice(&raw_payload).expect("decode_from_slice failed");
    assert_eq!(decoded_vec, original);
    assert_eq!(ver, version);
}

// ── Scenario 7 ───────────────────────────────────────────────────────────────
// decode_versioned_with_check same version succeeds
#[test]
fn test_decode_versioned_with_check_same_version_succeeds() {
    let version = Version::new(1, 0, 0);
    let payload = b"same version";
    let encoded = encode_versioned(payload, version).expect("encode_versioned failed");
    let (decoded_payload, decoded_ver, compat) =
        decode_versioned_with_check(&encoded, version, None)
            .expect("decode_versioned_with_check failed");
    assert_eq!(decoded_payload.as_slice(), payload.as_slice());
    assert_eq!(decoded_ver, version);
    assert!(compat.is_usable());
}

// ── Scenario 8 ───────────────────────────────────────────────────────────────
// decode_versioned_with_check patch bump succeeds (compatible)
#[test]
fn test_decode_versioned_with_check_patch_bump_succeeds() {
    // Data was encoded at 1.0.0; we now read with target 1.0.5 (patch bump).
    let data_version = Version::new(1, 0, 0);
    let current_version = Version::new(1, 0, 5);
    let payload = b"patch bump";
    let encoded = encode_versioned(payload, data_version).expect("encode_versioned failed");
    let (decoded_payload, decoded_ver, compat) =
        decode_versioned_with_check(&encoded, current_version, None)
            .expect("patch bump should be compatible");
    assert_eq!(decoded_payload.as_slice(), payload.as_slice());
    assert_eq!(decoded_ver, data_version);
    // Patch bump is compatible (no warnings for same major/minor)
    assert!(compat.is_usable());
    assert_eq!(compat, CompatibilityLevel::Compatible);
}

// ── Scenario 9 ───────────────────────────────────────────────────────────────
// decode_versioned_with_check: data at 1.0.0, target 1.1.0 (minor bump, post-1.0).
// By check_compatibility logic: data_version (1.0.0) < current (1.1.0), same major →
// CompatibleWithWarnings (older minor).  The call succeeds, compat has warnings.
#[test]
fn test_decode_versioned_with_check_minor_bump_post_1_0_compat_with_warnings() {
    let data_version = Version::new(1, 0, 0);
    let current_version = Version::new(1, 1, 0);
    let payload = b"minor bump";
    let encoded = encode_versioned(payload, data_version).expect("encode_versioned failed");
    // With post-1.0, same major → compatible (possibly with warnings).
    // This SUCCEEDS because major versions match (not Incompatible).
    let result = decode_versioned_with_check(&encoded, current_version, None);
    let (decoded_payload, _decoded_ver, compat) =
        result.expect("minor bump within same major should be usable");
    assert_eq!(decoded_payload.as_slice(), payload.as_slice());
    // Older minor version → CompatibleWithWarnings
    assert_eq!(compat, CompatibilityLevel::CompatibleWithWarnings);
    assert!(compat.is_usable());
    assert!(!compat.is_fully_compatible());
}

// ── Scenario 10 ──────────────────────────────────────────────────────────────
// encode_versioned_value with struct roundtrip
#[derive(Encode, Decode, Debug, PartialEq)]
struct VersionedPoint {
    x: i32,
    y: i32,
}

#[test]
fn test_encode_versioned_value_struct_roundtrip() {
    let version = Version::new(1, 0, 0);
    let original = VersionedPoint { x: 10, y: -20 };
    let payload_bytes = encode_to_vec(&original).expect("encode_to_vec failed");
    let encoded = encode_versioned(&payload_bytes, version).expect("encode_versioned failed");
    let (raw_payload, ver) = decode_versioned(&encoded).expect("decode_versioned failed");
    let (decoded_point, _consumed): (VersionedPoint, usize) =
        decode_from_slice(&raw_payload).expect("decode_from_slice failed");
    assert_eq!(decoded_point, original);
    assert_eq!(ver, version);
}

// ── Scenario 11 ──────────────────────────────────────────────────────────────
// Version comparison: 1.0.0 vs 2.0.0 — major bump is breaking
#[test]
fn test_version_comparison_major_bump_is_breaking() {
    let v1 = Version::new(1, 0, 0);
    let v2 = Version::new(2, 0, 0);
    assert!(v2.is_breaking_change_from(&v1));
    assert!(!v1.is_compatible_with(&v2));
    assert!(!v2.is_compatible_with(&v1));
    assert!(v2 > v1);
}

// ── Scenario 12 ──────────────────────────────────────────────────────────────
// Version comparison: 0.1.0 vs 0.2.0 — minor bump is breaking in pre-1.0
#[test]
fn test_version_comparison_minor_bump_pre_1_0_is_breaking() {
    let v1 = Version::new(0, 1, 0);
    let v2 = Version::new(0, 2, 0);
    // Pre-1.0: any minor difference is a breaking change
    assert!(v2.is_breaking_change_from(&v1));
    assert!(!v1.is_compatible_with(&v2));
    // Minor update direction
    assert!(v2.is_minor_update_from(&v1));
}

// ── Scenario 13 ──────────────────────────────────────────────────────────────
// is_versioned check on encode_versioned output (should be true)
#[test]
fn test_is_versioned_on_encoded_versioned_output() {
    let version = Version::new(1, 0, 0);
    let payload = b"check magic";
    let encoded = encode_versioned(payload, version).expect("encode_versioned failed");
    assert!(is_versioned(&encoded));
}

// ── Scenario 14 ──────────────────────────────────────────────────────────────
// is_versioned check on plain encode_to_vec output (should be false)
#[test]
fn test_is_versioned_on_plain_encode_is_false() {
    let plain_bytes = encode_to_vec(&9999u64).expect("encode_to_vec failed");
    assert!(!is_versioned(&plain_bytes));
}

// ── Scenario 15 ──────────────────────────────────────────────────────────────
// extract_version from versioned data
#[test]
fn test_extract_version_from_versioned_data() {
    let version = Version::new(7, 13, 42);
    let payload = b"extract me";
    let encoded = encode_versioned(payload, version).expect("encode_versioned failed");
    let extracted = extract_version(&encoded).expect("extract_version failed");
    assert_eq!(extracted, version);
    assert_eq!(extracted.major, 7);
    assert_eq!(extracted.minor, 13);
    assert_eq!(extracted.patch, 42);
}

// ── Scenario 16 ──────────────────────────────────────────────────────────────
// VersionedHeader default version is Version::zero()
#[test]
fn test_versioned_header_default_version_is_zero() {
    let header = VersionedHeader::default();
    assert_eq!(header.version(), Version::zero());
    assert_eq!(header.version().major, 0);
    assert_eq!(header.version().minor, 0);
    assert_eq!(header.version().patch, 0);
}

// ── Scenario 17 ──────────────────────────────────────────────────────────────
// Version::from_bytes roundtrip
#[test]
fn test_version_from_bytes_roundtrip() {
    let original = Version::new(5, 10, 255);
    let bytes = original.to_bytes();
    let restored = Version::from_bytes(&bytes).expect("from_bytes returned None");
    assert_eq!(restored, original);
    assert_eq!(restored.major, 5);
    assert_eq!(restored.minor, 10);
    assert_eq!(restored.patch, 255);
}

// ── Scenario 18 ──────────────────────────────────────────────────────────────
// Version zero comparison
#[test]
fn test_version_zero_comparison() {
    let zero = Version::zero();
    let one_patch = Version::new(0, 0, 1);
    let one_minor = Version::new(0, 1, 0);
    let one_major = Version::new(1, 0, 0);
    assert!(zero < one_patch);
    assert!(one_patch < one_minor);
    assert!(one_minor < one_major);
    assert_eq!(zero, Version::new(0, 0, 0));
}

// ── Scenario 19 ──────────────────────────────────────────────────────────────
// encode_versioned_value on bool true roundtrip
#[test]
fn test_encode_versioned_value_bool_true_roundtrip() {
    let version = Version::new(1, 0, 0);
    let original: bool = true;
    let payload_bytes = encode_to_vec(&original).expect("encode_to_vec failed");
    let encoded = encode_versioned(&payload_bytes, version).expect("encode_versioned failed");
    let (raw_payload, ver) = decode_versioned(&encoded).expect("decode_versioned failed");
    let (decoded_bool, _consumed): (bool, usize) =
        decode_from_slice(&raw_payload).expect("decode_from_slice failed");
    assert!(decoded_bool);
    assert_eq!(ver, version);
}

// ── Scenario 20 ──────────────────────────────────────────────────────────────
// encode_versioned_value on (u32, String) tuple roundtrip
#[test]
fn test_encode_versioned_value_tuple_roundtrip() {
    let version = Version::new(2, 0, 0);
    let original: (u32, String) = (99, String::from("tuple"));
    let payload_bytes = encode_to_vec(&original).expect("encode_to_vec failed");
    let encoded = encode_versioned(&payload_bytes, version).expect("encode_versioned failed");
    let (raw_payload, ver) = decode_versioned(&encoded).expect("decode_versioned failed");
    let (decoded_tuple, _consumed): ((u32, String), usize) =
        decode_from_slice(&raw_payload).expect("decode_from_slice failed");
    assert_eq!(decoded_tuple.0, 99u32);
    assert_eq!(decoded_tuple.1, "tuple");
    assert_eq!(ver, version);
}

// ── Scenario 21 ──────────────────────────────────────────────────────────────
// decode_versioned consumed equals total encoded length
#[test]
fn test_decode_versioned_consumed_equals_encoded_length() {
    let version = Version::new(1, 0, 0);
    let value: u64 = 12345678;
    let payload_bytes = encode_to_vec(&value).expect("encode_to_vec failed");
    let encoded = encode_versioned(&payload_bytes, version).expect("encode_versioned failed");
    // decode_versioned returns the full payload; the header + payload = total encoded
    let (raw_payload, _ver) = decode_versioned(&encoded).expect("decode_versioned failed");
    // The versioned header is 11 bytes (magic 4 + header_ver 1 + version 6)
    let header_size = 11usize;
    assert_eq!(header_size + raw_payload.len(), encoded.len());
    // The raw_payload length equals the payload bytes we encoded
    assert_eq!(raw_payload.len(), payload_bytes.len());
}

// ── Scenario 22 ──────────────────────────────────────────────────────────────
// Multiple versioned values decoded sequentially
#[test]
fn test_multiple_versioned_values_decoded_sequentially() {
    // Encode three separate versioned buffers and decode them independently.
    let v1 = Version::new(1, 0, 0);
    let v2 = Version::new(2, 0, 0);
    let v3 = Version::new(3, 1, 5);

    let p1 = encode_to_vec(&100u32).expect("encode p1");
    let p2 = encode_to_vec(&String::from("second")).expect("encode p2");
    let p3 = encode_to_vec(&false).expect("encode p3");

    let e1 = encode_versioned(&p1, v1).expect("versioned e1");
    let e2 = encode_versioned(&p2, v2).expect("versioned e2");
    let e3 = encode_versioned(&p3, v3).expect("versioned e3");

    let (raw1, ver1) = decode_versioned(&e1).expect("decode e1");
    let (raw2, ver2) = decode_versioned(&e2).expect("decode e2");
    let (raw3, ver3) = decode_versioned(&e3).expect("decode e3");

    let (val1, _): (u32, usize) = decode_from_slice(&raw1).expect("decode val1");
    let (val2, _): (String, usize) = decode_from_slice(&raw2).expect("decode val2");
    let (val3, _): (bool, usize) = decode_from_slice(&raw3).expect("decode val3");

    assert_eq!(val1, 100u32);
    assert_eq!(val2, "second");
    assert!(!val3);
    assert_eq!(ver1, v1);
    assert_eq!(ver2, v2);
    assert_eq!(ver3, v3);

    // Verify each buffer is independently recognized as versioned
    assert!(is_versioned(&e1));
    assert!(is_versioned(&e2));
    assert!(is_versioned(&e3));

    // Verify plain-encoded buffers are not versioned
    assert!(!is_versioned(&p1));
    assert!(!is_versioned(&p2));
    assert!(!is_versioned(&p3));
}
