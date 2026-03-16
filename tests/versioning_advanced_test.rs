//! Advanced tests for schema evolution and versioning (22 tests).
//!
//! Covers Version semantics, encode/decode roundtrips, migration paths,
//! compatibility checking, header magic, and schema evolution patterns.
//! All tests are top-level; no cfg(test) module wrapper.

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
    decode_versioned, decode_versioned_with_check, encode_versioned, extract_version, is_versioned,
    Version, VERSIONED_MAGIC,
};
use oxicode::{decode_versioned_value, encode_versioned_value, Decode, Encode};

// ─────────────────────────────────────────────────────────────────────────────
// Shared derive types
// ─────────────────────────────────────────────────────────────────────────────

#[derive(Encode, Decode, PartialEq, Debug)]
struct SampleStruct {
    id: u32,
    value: u64,
    flag: bool,
}

#[derive(Encode, Decode, PartialEq, Debug)]
struct MathConstants {
    pi: f64,
    e: f64,
}

#[derive(Encode, Decode, PartialEq, Debug)]
struct OuterStruct {
    inner_a: InnerA,
    inner_b: InnerB,
}

#[derive(Encode, Decode, PartialEq, Debug)]
struct InnerA {
    x: i32,
    y: i32,
}

#[derive(Encode, Decode, PartialEq, Debug)]
struct InnerB {
    label: u64,
    active: bool,
}

#[derive(Encode, Decode, PartialEq, Debug)]
enum SimpleEnum {
    Alpha,
    Beta(u32),
    Gamma { x: i64 },
}

// ─────────────────────────────────────────────────────────────────────────────
// Test 1 — encode/decode with version tag v1
// ─────────────────────────────────────────────────────────────────────────────

#[test]
fn test_encode_decode_version_tag_v1() {
    let original = SampleStruct {
        id: 1,
        value: 100,
        flag: false,
    };
    let version = Version::new(1, 0, 0);

    let encoded =
        encode_versioned_value(&original, version).expect("encode_versioned_value v1 failed");
    let (decoded, recovered_ver, _consumed): (SampleStruct, _, _) =
        decode_versioned_value(&encoded).expect("decode_versioned_value v1 failed");

    assert_eq!(decoded, original, "decoded must match original");
    assert_eq!(recovered_ver, version, "recovered version must be 1.0.0");
}

// ─────────────────────────────────────────────────────────────────────────────
// Test 2 — encode/decode with version tag v2
// ─────────────────────────────────────────────────────────────────────────────

#[test]
fn test_encode_decode_version_tag_v2() {
    let original = SampleStruct {
        id: 2,
        value: 200,
        flag: true,
    };
    let version = Version::new(2, 0, 0);

    let encoded =
        encode_versioned_value(&original, version).expect("encode_versioned_value v2 failed");
    let (decoded, recovered_ver, _): (SampleStruct, _, _) =
        decode_versioned_value(&encoded).expect("decode_versioned_value v2 failed");

    assert_eq!(decoded, original);
    assert_eq!(recovered_ver, version);
}

// ─────────────────────────────────────────────────────────────────────────────
// Test 3 — version tag is embedded: encoded bytes start with VERSIONED_MAGIC
// ─────────────────────────────────────────────────────────────────────────────

#[test]
fn test_version_tag_embedded_in_encoded_bytes() {
    let data = b"embedding test";
    let version = Version::new(1, 2, 3);

    let versioned = encode_versioned(data, version).expect("encode_versioned failed");

    assert!(
        versioned.len() >= VERSIONED_MAGIC.len(),
        "versioned output too short"
    );
    assert_eq!(
        &versioned[..VERSIONED_MAGIC.len()],
        &VERSIONED_MAGIC,
        "encoded bytes must start with VERSIONED_MAGIC"
    );
}

// ─────────────────────────────────────────────────────────────────────────────
// Test 4 — decoding with wrong (incompatible) version returns an error
// ─────────────────────────────────────────────────────────────────────────────

#[test]
fn test_decode_wrong_version_returns_error() {
    let data = b"payload";
    let data_version = Version::new(2, 0, 0);
    let current_version = Version::new(1, 0, 0);

    let encoded = encode_versioned(data, data_version).expect("encode_versioned failed");
    let result = decode_versioned_with_check(&encoded, current_version, None);

    assert!(
        result.is_err(),
        "decoding v2 data against v1 reader must return an error"
    );
}

// ─────────────────────────────────────────────────────────────────────────────
// Test 5 — versioned encoding of primitives (u64)
// ─────────────────────────────────────────────────────────────────────────────

#[test]
fn test_versioned_encoding_of_primitives() {
    let value: u64 = 0xDEAD_BEEF_CAFE_BABE;
    let version = Version::new(1, 0, 0);

    let encoded = encode_versioned_value(&value, version).expect("encode primitive failed");
    let (decoded, ver, _): (u64, _, _) =
        decode_versioned_value(&encoded).expect("decode primitive failed");

    assert_eq!(decoded, value);
    assert_eq!(ver, version);
}

// ─────────────────────────────────────────────────────────────────────────────
// Test 6 — versioned encoding of structs
// ─────────────────────────────────────────────────────────────────────────────

#[test]
fn test_versioned_encoding_of_structs() {
    let original = SampleStruct {
        id: 42,
        value: 9999,
        flag: true,
    };
    let version = Version::new(1, 0, 0);

    let encoded = encode_versioned_value(&original, version).expect("encode struct failed");
    let (decoded, _, _): (SampleStruct, _, _) =
        decode_versioned_value(&encoded).expect("decode struct failed");

    assert_eq!(decoded, original);
}

// ─────────────────────────────────────────────────────────────────────────────
// Test 7 — versioned encoding of enums
// ─────────────────────────────────────────────────────────────────────────────

#[test]
fn test_versioned_encoding_of_enums() {
    let version = Version::new(1, 0, 0);
    let variants: &[SimpleEnum] = &[
        SimpleEnum::Alpha,
        SimpleEnum::Beta(77),
        SimpleEnum::Gamma { x: -999 },
    ];

    for variant in variants {
        let encoded = encode_versioned_value(variant, version).expect("encode enum variant failed");
        let (decoded, _, _): (SimpleEnum, _, _) =
            decode_versioned_value(&encoded).expect("decode enum variant failed");
        assert_eq!(&decoded, variant);
    }
}

// ─────────────────────────────────────────────────────────────────────────────
// Test 8 — versioned Vec<T> roundtrip
// ─────────────────────────────────────────────────────────────────────────────

#[test]
fn test_versioned_vec_roundtrip() {
    let original: Vec<u32> = vec![10, 20, 30, 40, 50];
    let version = Version::new(1, 0, 0);

    let encoded = encode_versioned_value(&original, version).expect("encode Vec<u32> failed");
    let (decoded, ver, _): (Vec<u32>, _, _) =
        decode_versioned_value(&encoded).expect("decode Vec<u32> failed");

    assert_eq!(decoded, original);
    assert_eq!(ver, version);
}

// ─────────────────────────────────────────────────────────────────────────────
// Test 9 — versioned Option<T> Some and None
// ─────────────────────────────────────────────────────────────────────────────

#[test]
fn test_versioned_option_some_none() {
    let version = Version::new(1, 0, 0);

    let some_val: Option<u32> = Some(42);
    let encoded_some =
        encode_versioned_value(&some_val, version).expect("encode Option::Some failed");
    let (decoded_some, _, _): (Option<u32>, _, _) =
        decode_versioned_value(&encoded_some).expect("decode Option::Some failed");
    assert_eq!(decoded_some, some_val);

    let none_val: Option<u32> = None;
    let encoded_none =
        encode_versioned_value(&none_val, version).expect("encode Option::None failed");
    let (decoded_none, _, _): (Option<u32>, _, _) =
        decode_versioned_value(&encoded_none).expect("decode Option::None failed");
    assert_eq!(decoded_none, none_val);
}

// ─────────────────────────────────────────────────────────────────────────────
// Test 10 — version header byte count: encoded length > raw payload length by header size
// ─────────────────────────────────────────────────────────────────────────────

#[test]
fn test_version_byte_count_in_encoded_data() {
    let payload = b"short";
    let version = Version::new(1, 0, 0);

    let versioned = encode_versioned(payload, version).expect("encode_versioned failed");

    // Header is 11 bytes: 4 (magic) + 1 (header_ver) + 6 (version)
    let expected_header_size = 11usize;
    assert_eq!(
        versioned.len(),
        payload.len() + expected_header_size,
        "versioned length must be payload + 11-byte header"
    );
}

// ─────────────────────────────────────────────────────────────────────────────
// Test 11 — encode v1, decode expecting v1: succeeds
// ─────────────────────────────────────────────────────────────────────────────

#[test]
fn test_encode_v1_decode_expecting_v1_succeeds() {
    let data = b"compatible";
    let version = Version::new(1, 0, 0);

    let encoded = encode_versioned(data, version).expect("encode_versioned failed");
    let (payload, ver, _compat) = decode_versioned_with_check(&encoded, version, None)
        .expect("decode_versioned_with_check failed");

    assert_eq!(payload.as_slice(), data);
    assert_eq!(ver, version);
}

// ─────────────────────────────────────────────────────────────────────────────
// Test 12 — versioned encoding with fixed_int config payload
// ─────────────────────────────────────────────────────────────────────────────

#[test]
fn test_versioned_encoding_with_fixed_int_config() {
    use oxicode::config;

    let data = 42u32;
    let version = Version::new(1, 0, 0);
    let fixed_cfg = config::standard().with_fixed_int_encoding();

    let payload =
        oxicode::encode_to_vec_with_config(&data, fixed_cfg).expect("fixed_int encode failed");
    assert_eq!(
        payload.len(),
        4,
        "u32 with fixed_int must be exactly 4 bytes"
    );

    let versioned = encode_versioned(&payload, version).expect("encode_versioned failed");
    let (recovered_payload, recovered_ver) =
        decode_versioned(&versioned).expect("decode_versioned failed");

    assert_eq!(recovered_ver, version);

    let (decoded, _): (u32, _) =
        oxicode::decode_from_slice_with_config(&recovered_payload, fixed_cfg)
            .expect("decode with fixed_int config failed");
    assert_eq!(decoded, data);
}

// ─────────────────────────────────────────────────────────────────────────────
// Test 13 — versioned encoding with big endian config payload
// ─────────────────────────────────────────────────────────────────────────────

#[test]
fn test_versioned_encoding_with_big_endian_config() {
    use oxicode::config;

    let data = 0xAABBCCDDu32;
    let version = Version::new(1, 0, 0);
    let be_cfg = config::standard().with_big_endian();

    let payload =
        oxicode::encode_to_vec_with_config(&data, be_cfg).expect("big endian encode failed");

    let versioned = encode_versioned(&payload, version).expect("encode_versioned failed");
    let (recovered_payload, recovered_ver) =
        decode_versioned(&versioned).expect("decode_versioned failed");

    assert_eq!(recovered_ver, version);

    let (decoded, _): (u32, _) = oxicode::decode_from_slice_with_config(&recovered_payload, be_cfg)
        .expect("decode with big endian config failed");
    assert_eq!(decoded, data);
}

// ─────────────────────────────────────────────────────────────────────────────
// Test 14 — versioned encoding of large data (1 MiB)
// ─────────────────────────────────────────────────────────────────────────────

#[test]
fn test_versioned_encoding_of_large_data() {
    let large: Vec<u8> = (0u32..1024 * 1024).map(|i| (i % 256) as u8).collect();
    let version = Version::new(3, 1, 4);

    let versioned = encode_versioned(&large, version).expect("encode large data failed");
    let (recovered, ver) = decode_versioned(&versioned).expect("decode large data failed");

    assert_eq!(ver, version);
    assert_eq!(recovered, large);
}

// ─────────────────────────────────────────────────────────────────────────────
// Test 15 — versioned encoding of String
// ─────────────────────────────────────────────────────────────────────────────

#[test]
fn test_versioned_encoding_of_string() {
    let original = String::from("Hello, OxiCode versioning!");
    let version = Version::new(1, 0, 0);

    let encoded = encode_versioned_value(&original, version).expect("encode String failed");
    let (decoded, ver, _): (String, _, _) =
        decode_versioned_value(&encoded).expect("decode String failed");

    assert_eq!(decoded, original);
    assert_eq!(ver, version);
}

// ─────────────────────────────────────────────────────────────────────────────
// Test 16 — versioned encoding of Vec<String>
// ─────────────────────────────────────────────────────────────────────────────

#[test]
fn test_versioned_encoding_of_vec_string() {
    let original: Vec<String> = vec![
        String::from("alpha"),
        String::from("beta"),
        String::from("gamma"),
        String::from("delta"),
    ];
    let version = Version::new(2, 1, 0);

    let encoded = encode_versioned_value(&original, version).expect("encode Vec<String> failed");
    let (decoded, ver, _): (Vec<String>, _, _) =
        decode_versioned_value(&encoded).expect("decode Vec<String> failed");

    assert_eq!(decoded, original);
    assert_eq!(ver, version);
}

// ─────────────────────────────────────────────────────────────────────────────
// Test 17 — versioned encoding of complex struct (MathConstants with f64)
// ─────────────────────────────────────────────────────────────────────────────

#[test]
fn test_versioned_encoding_of_complex_struct() {
    use std::f64::consts::{E, PI};

    let original = MathConstants { pi: PI, e: E };
    let version = Version::new(1, 0, 0);

    let encoded = encode_versioned_value(&original, version).expect("encode MathConstants failed");
    let (decoded, _, _): (MathConstants, _, _) =
        decode_versioned_value(&encoded).expect("decode MathConstants failed");

    assert_eq!(decoded.pi, PI, "PI must roundtrip exactly");
    assert_eq!(decoded.e, E, "E must roundtrip exactly");
}

// ─────────────────────────────────────────────────────────────────────────────
// Test 18 — version tag comparison: v1 < v2 < v3 (total ordering)
// ─────────────────────────────────────────────────────────────────────────────

#[test]
fn test_version_tag_comparison_ordering() {
    let v1 = Version::new(1, 0, 0);
    let v2 = Version::new(1, 5, 0);
    let v3 = Version::new(2, 0, 0);
    let v4 = Version::new(2, 0, 1);

    assert!(v1 < v2, "1.0.0 must be less than 1.5.0");
    assert!(v2 < v3, "1.5.0 must be less than 2.0.0");
    assert!(v3 < v4, "2.0.0 must be less than 2.0.1");
    assert!(v1 < v4, "1.0.0 must be less than 2.0.1");
    assert_eq!(v1, Version::new(1, 0, 0));
}

// ─────────────────────────────────────────────────────────────────────────────
// Test 19 — encode then decode various versions (v0, v1, v255)
// ─────────────────────────────────────────────────────────────────────────────

#[test]
fn test_encode_decode_various_versions() {
    let payload = b"multi-version test";
    let versions = [
        Version::new(0, 0, 0),
        Version::new(1, 0, 0),
        Version::new(0, 255, 255),
        Version::new(10, 3, 7),
    ];

    for version in &versions {
        let encoded =
            encode_versioned(payload, *version).expect("encode_versioned failed for version");
        let (recovered, ver) =
            decode_versioned(&encoded).expect("decode_versioned failed for version");

        assert_eq!(
            recovered.as_slice(),
            payload,
            "payload mismatch for {version}"
        );
        assert_eq!(ver, *version, "version mismatch");
    }
}

// ─────────────────────────────────────────────────────────────────────────────
// Test 20 — versioned encoding with limit config (via decode_from_slice_with_config)
// ─────────────────────────────────────────────────────────────────────────────

#[test]
fn test_versioned_encoding_with_limit_config() {
    use oxicode::config;

    let data = 12345u64;
    let version = Version::new(1, 0, 0);
    let limit_cfg = config::standard().with_limit::<256>();

    let payload = oxicode::encode_to_vec_with_config(&data, limit_cfg)
        .expect("encode with limit config failed");

    let versioned =
        encode_versioned(&payload, version).expect("encode_versioned with limit payload failed");

    let (recovered_payload, recovered_ver) =
        decode_versioned(&versioned).expect("decode_versioned failed");

    assert_eq!(recovered_ver, version);

    let (decoded, _): (u64, _) =
        oxicode::decode_from_slice_with_config(&recovered_payload, limit_cfg)
            .expect("decode with limit config failed");
    assert_eq!(decoded, data);
}

// ─────────────────────────────────────────────────────────────────────────────
// Test 21 — Version 0.0.0 and Version(u16::MAX, u16::MAX, u16::MAX) roundtrip
// ─────────────────────────────────────────────────────────────────────────────

#[test]
fn test_version_zero_and_max_roundtrip() {
    let v_zero = Version::zero();
    let v_max = Version::new(u16::MAX, u16::MAX, u16::MAX);

    let data = b"boundary";

    for version in &[v_zero, v_max] {
        let encoded =
            encode_versioned(data, *version).expect("encode_versioned boundary version failed");

        let extracted = extract_version(&encoded).expect("extract_version failed");

        assert_eq!(
            extracted, *version,
            "extracted version must match for {version}"
        );
        assert!(is_versioned(&encoded), "is_versioned must return true");
    }

    assert!(v_zero < v_max, "0.0.0 must be less than max version");
}

// ─────────────────────────────────────────────────────────────────────────────
// Test 22 — versioned encoding of nested structs (OuterStruct)
// ─────────────────────────────────────────────────────────────────────────────

#[test]
fn test_versioned_encoding_of_nested_structs() {
    let original = OuterStruct {
        inner_a: InnerA { x: -100, y: 200 },
        inner_b: InnerB {
            label: 0xCAFE,
            active: true,
        },
    };
    let version = Version::new(1, 1, 0);

    let encoded = encode_versioned_value(&original, version).expect("encode OuterStruct failed");
    let (decoded, ver, _): (OuterStruct, _, _) =
        decode_versioned_value(&encoded).expect("decode OuterStruct failed");

    assert_eq!(decoded.inner_a.x, original.inner_a.x);
    assert_eq!(decoded.inner_a.y, original.inner_a.y);
    assert_eq!(decoded.inner_b.label, original.inner_b.label);
    assert_eq!(decoded.inner_b.active, original.inner_b.active);
    assert_eq!(ver, version);
}
