//! Advanced tuple encoding tests — 22 test cases for OxiCode tuple roundtrips,
//! config variations, struct embedding, Vec/Option containers, and byte-layout checks.

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
    config, decode_from_slice, decode_from_slice_with_config, encode_to_vec,
    encode_to_vec_with_config, Decode, Encode,
};

// ── Struct used in test 15 ────────────────────────────────────────────────────

#[derive(Debug, PartialEq, Encode, Decode)]
struct Point {
    x: f64,
    y: f64,
}

// ── Test 1: 1-tuple (u32,) roundtrip ─────────────────────────────────────────

#[test]
fn test_tuple_adv4_1_tuple_u32_roundtrip() {
    let val: (u32,) = (987654u32,);
    let bytes = encode_to_vec(&val).expect("encode 1-tuple u32");
    let (decoded, consumed): ((u32,), usize) =
        decode_from_slice(&bytes).expect("decode 1-tuple u32");
    assert_eq!(val, decoded, "1-tuple value mismatch");
    assert_eq!(consumed, bytes.len(), "consumed bytes mismatch");
}

// ── Test 2: 2-tuple (u32, String) roundtrip ──────────────────────────────────

#[test]
fn test_tuple_adv4_2_tuple_u32_string_roundtrip() {
    let val: (u32, String) = (42u32, "hello oxicode".to_string());
    let bytes = encode_to_vec(&val).expect("encode 2-tuple (u32, String)");
    let (decoded, _): ((u32, String), usize) =
        decode_from_slice(&bytes).expect("decode 2-tuple (u32, String)");
    assert_eq!(val, decoded);
}

// ── Test 3: 3-tuple (u8, u16, u32) roundtrip ─────────────────────────────────

#[test]
fn test_tuple_adv4_3_tuple_u8_u16_u32_roundtrip() {
    let val: (u8, u16, u32) = (255u8, 65535u16, 4294967295u32);
    let bytes = encode_to_vec(&val).expect("encode 3-tuple unsigned");
    let (decoded, _): ((u8, u16, u32), usize) =
        decode_from_slice(&bytes).expect("decode 3-tuple unsigned");
    assert_eq!(val, decoded);
}

// ── Test 4: 4-tuple (u64, i64, f32, f64) roundtrip ───────────────────────────

#[test]
fn test_tuple_adv4_4_tuple_u64_i64_f32_f64_roundtrip() {
    let val: (u64, i64, f32, f64) = (u64::MAX, i64::MIN, 3.14f32, 2.718281828f64);
    let bytes = encode_to_vec(&val).expect("encode 4-tuple mixed numeric");
    let (decoded, _): ((u64, i64, f32, f64), usize) =
        decode_from_slice(&bytes).expect("decode 4-tuple mixed numeric");
    assert_eq!(val.0, decoded.0);
    assert_eq!(val.1, decoded.1);
    assert!((val.2 - decoded.2).abs() < f32::EPSILON, "f32 mismatch");
    assert!((val.3 - decoded.3).abs() < f64::EPSILON, "f64 mismatch");
}

// ── Test 5: 5-tuple roundtrip ─────────────────────────────────────────────────

#[test]
fn test_tuple_adv4_5_tuple_roundtrip() {
    let val: (u8, u16, u32, u64, i32) = (1u8, 2u16, 3u32, 4u64, -5i32);
    let bytes = encode_to_vec(&val).expect("encode 5-tuple");
    let (decoded, _): ((u8, u16, u32, u64, i32), usize) =
        decode_from_slice(&bytes).expect("decode 5-tuple");
    assert_eq!(val, decoded);
}

// ── Test 6: 6-tuple roundtrip ─────────────────────────────────────────────────

#[test]
fn test_tuple_adv4_6_tuple_roundtrip() {
    let val: (u8, u16, u32, u64, i32, i64) = (10u8, 20u16, 30u32, 40u64, -50i32, -60i64);
    let bytes = encode_to_vec(&val).expect("encode 6-tuple");
    let (decoded, _): ((u8, u16, u32, u64, i32, i64), usize) =
        decode_from_slice(&bytes).expect("decode 6-tuple");
    assert_eq!(val, decoded);
}

// ── Test 7: 7-tuple roundtrip ─────────────────────────────────────────────────

#[test]
fn test_tuple_adv4_7_tuple_roundtrip() {
    let val: (u8, u16, u32, u64, i8, i16, i32) = (1, 2, 3, 4, -5, -6, -7);
    let bytes = encode_to_vec(&val).expect("encode 7-tuple");
    let (decoded, _): ((u8, u16, u32, u64, i8, i16, i32), usize) =
        decode_from_slice(&bytes).expect("decode 7-tuple");
    assert_eq!(val, decoded);
}

// ── Test 8: 8-tuple roundtrip ─────────────────────────────────────────────────

#[test]
fn test_tuple_adv4_8_tuple_roundtrip() {
    let val: (u8, u16, u32, u64, i8, i16, i32, i64) = (1, 2, 3, 4, -5, -6, -7, -8);
    let bytes = encode_to_vec(&val).expect("encode 8-tuple");
    let (decoded, _): ((u8, u16, u32, u64, i8, i16, i32, i64), usize) =
        decode_from_slice(&bytes).expect("decode 8-tuple");
    assert_eq!(val, decoded);
}

// ── Test 9: Tuple with bool values roundtrip ──────────────────────────────────

#[test]
fn test_tuple_adv4_9_tuple_bools_roundtrip() {
    let val: (bool, bool, bool, bool) = (true, false, true, false);
    let bytes = encode_to_vec(&val).expect("encode bool 4-tuple");
    let (decoded, _): ((bool, bool, bool, bool), usize) =
        decode_from_slice(&bytes).expect("decode bool 4-tuple");
    assert_eq!(val, decoded);
}

// ── Test 10: Tuple with Option<String> roundtrip ──────────────────────────────

#[test]
fn test_tuple_adv4_10_tuple_option_string_roundtrip() {
    let val_some: (u32, Option<String>) = (7u32, Some("oxicode rules".to_string()));
    let bytes_some = encode_to_vec(&val_some).expect("encode tuple with Some(String)");
    let (decoded_some, _): ((u32, Option<String>), usize) =
        decode_from_slice(&bytes_some).expect("decode tuple with Some(String)");
    assert_eq!(val_some, decoded_some);

    let val_none: (u32, Option<String>) = (99u32, None);
    let bytes_none = encode_to_vec(&val_none).expect("encode tuple with None");
    let (decoded_none, _): ((u32, Option<String>), usize) =
        decode_from_slice(&bytes_none).expect("decode tuple with None");
    assert_eq!(val_none, decoded_none);
}

// ── Test 11: Nested tuple ((u32, u32), (String, bool)) roundtrip ───────────────

#[test]
fn test_tuple_adv4_11_nested_tuple_roundtrip() {
    let val: ((u32, u32), (String, bool)) = ((100u32, 200u32), ("nested".to_string(), true));
    let bytes = encode_to_vec(&val).expect("encode nested tuple");
    let (decoded, _): (((u32, u32), (String, bool)), usize) =
        decode_from_slice(&bytes).expect("decode nested tuple");
    assert_eq!(val, decoded);
}

// ── Test 12: Tuple with Vec<u8> roundtrip ────────────────────────────────────

#[test]
fn test_tuple_adv4_12_tuple_with_vec_u8_roundtrip() {
    let val: (u32, Vec<u8>) = (42u32, vec![0xDE, 0xAD, 0xBE, 0xEF, 0xCA, 0xFE]);
    let bytes = encode_to_vec(&val).expect("encode tuple with Vec<u8>");
    let (decoded, _): ((u32, Vec<u8>), usize) =
        decode_from_slice(&bytes).expect("decode tuple with Vec<u8>");
    assert_eq!(val, decoded);
}

// ── Test 13: 2-tuple with fixed-int config ────────────────────────────────────

#[test]
fn test_tuple_adv4_13_tuple_fixed_int_config() {
    let cfg = config::standard().with_fixed_int_encoding();
    let val: (u32, u64) = (1234u32, 5678u64);
    let bytes = encode_to_vec_with_config(&val, cfg).expect("encode with fixed-int config");
    // u32 = 4 bytes, u64 = 8 bytes → total 12 bytes with fixed encoding
    assert_eq!(bytes.len(), 12, "fixed-int encoded size should be 12 bytes");
    let (decoded, _): ((u32, u64), usize) =
        decode_from_slice_with_config(&bytes, cfg).expect("decode with fixed-int config");
    assert_eq!(val, decoded);
}

// ── Test 14: 2-tuple with big-endian config ───────────────────────────────────

#[test]
fn test_tuple_adv4_14_tuple_big_endian_config() {
    let cfg = config::standard()
        .with_big_endian()
        .with_fixed_int_encoding();
    let val: (u32, u32) = (0x01020304u32, 0xDEADBEEFu32);
    let bytes = encode_to_vec_with_config(&val, cfg).expect("encode big-endian");
    // Verify big-endian byte order for first u32
    assert_eq!(bytes[0], 0x01, "big-endian MSB of first u32");
    assert_eq!(bytes[1], 0x02);
    assert_eq!(bytes[2], 0x03);
    assert_eq!(bytes[3], 0x04);
    let (decoded, _): ((u32, u32), usize) =
        decode_from_slice_with_config(&bytes, cfg).expect("decode big-endian");
    assert_eq!(val, decoded);
}

// ── Test 15: Tuple containing struct Point ────────────────────────────────────

#[test]
fn test_tuple_adv4_15_tuple_with_derived_struct_roundtrip() {
    let val: (String, Point) = (
        "origin-shift".to_string(),
        Point {
            x: 1.5f64,
            y: -2.75f64,
        },
    );
    let bytes = encode_to_vec(&val).expect("encode tuple with Point struct");
    let (decoded, _): ((String, Point), usize) =
        decode_from_slice(&bytes).expect("decode tuple with Point struct");
    assert_eq!(val.0, decoded.0, "String field mismatch");
    assert!(
        (val.1.x - decoded.1.x).abs() < f64::EPSILON,
        "Point.x mismatch"
    );
    assert!(
        (val.1.y - decoded.1.y).abs() < f64::EPSILON,
        "Point.y mismatch"
    );
}

// ── Test 16: Vec<(u32, String)> roundtrip (3 items) ───────────────────────────

#[test]
fn test_tuple_adv4_16_vec_of_tuples_roundtrip() {
    let val: Vec<(u32, String)> = vec![
        (1u32, "alpha".to_string()),
        (2u32, "beta".to_string()),
        (3u32, "gamma".to_string()),
    ];
    let bytes = encode_to_vec(&val).expect("encode Vec<(u32, String)>");
    let (decoded, _): (Vec<(u32, String)>, usize) =
        decode_from_slice(&bytes).expect("decode Vec<(u32, String)>");
    assert_eq!(val, decoded);
}

// ── Test 17: Option<(u32, u32)> Some roundtrip ────────────────────────────────

#[test]
fn test_tuple_adv4_17_option_tuple_some_roundtrip() {
    let val: Option<(u32, u32)> = Some((111u32, 222u32));
    let bytes = encode_to_vec(&val).expect("encode Option<(u32,u32)> Some");
    let (decoded, _): (Option<(u32, u32)>, usize) =
        decode_from_slice(&bytes).expect("decode Option<(u32,u32)> Some");
    assert_eq!(val, decoded);
}

// ── Test 18: Option<(u32, u32)> None roundtrip ────────────────────────────────

#[test]
fn test_tuple_adv4_18_option_tuple_none_roundtrip() {
    let val: Option<(u32, u32)> = None;
    let bytes = encode_to_vec(&val).expect("encode Option<(u32,u32)> None");
    let (decoded, _): (Option<(u32, u32)>, usize) =
        decode_from_slice(&bytes).expect("decode Option<(u32,u32)> None");
    assert_eq!(val, decoded);
}

// ── Test 19: Field order affects encoded bytes ────────────────────────────────

#[test]
fn test_tuple_adv4_19_field_order_produces_different_bytes() {
    // (u32, u64, u8) and (u8, u32, u64) with same numeric values but different
    // field ordering should produce different byte sequences due to varint encoding
    // and differing type widths at each position.
    let cfg = config::standard().with_fixed_int_encoding();
    let val_a: (u32, u64, u8) = (1u32, 2u64, 3u8);
    let val_b: (u8, u32, u64) = (1u8, 2u32, 3u64);
    let bytes_a = encode_to_vec_with_config(&val_a, cfg).expect("encode (u32, u64, u8)");
    let bytes_b = encode_to_vec_with_config(&val_b, cfg).expect("encode (u8, u32, u64)");
    // u32=4, u64=8, u8=1 → 13 bytes vs u8=1, u32=4, u64=8 → 13 bytes total
    // but the byte layouts differ in position
    assert_ne!(
        bytes_a, bytes_b,
        "different field order must produce different bytes"
    );
}

// ── Test 20: Consumed bytes equals encoded length for 4-tuple ─────────────────

#[test]
fn test_tuple_adv4_20_consumed_bytes_equals_encoded_len() {
    let val: (u8, u16, u32, u64) = (10u8, 200u16, 30000u32, 4000000u64);
    let bytes = encode_to_vec(&val).expect("encode 4-tuple for byte count check");
    let (_, consumed): ((u8, u16, u32, u64), usize) =
        decode_from_slice(&bytes).expect("decode 4-tuple for byte count check");
    assert_eq!(
        consumed,
        bytes.len(),
        "consumed byte count must equal total encoded length"
    );
}

// ── Test 21: Large 12-tuple roundtrip ────────────────────────────────────────

#[test]
fn test_tuple_adv4_21_large_12_tuple_roundtrip() {
    type T12 = (u8, u16, u32, u64, i8, i16, i32, i64, f32, f64, bool, String);
    let val: T12 = (
        255u8,
        65535u16,
        4294967295u32,
        u64::MAX,
        -128i8,
        -32768i16,
        i32::MIN,
        i64::MIN,
        1.23456789f32,
        9.87654321098765f64,
        true,
        "twelve-tuple-test".to_string(),
    );
    let bytes = encode_to_vec(&val).expect("encode 12-tuple");
    let (decoded, consumed): (T12, usize) = decode_from_slice(&bytes).expect("decode 12-tuple");
    assert_eq!(val.0, decoded.0, "field 0 (u8)");
    assert_eq!(val.1, decoded.1, "field 1 (u16)");
    assert_eq!(val.2, decoded.2, "field 2 (u32)");
    assert_eq!(val.3, decoded.3, "field 3 (u64)");
    assert_eq!(val.4, decoded.4, "field 4 (i8)");
    assert_eq!(val.5, decoded.5, "field 5 (i16)");
    assert_eq!(val.6, decoded.6, "field 6 (i32)");
    assert_eq!(val.7, decoded.7, "field 7 (i64)");
    assert!((val.8 - decoded.8).abs() < f32::EPSILON, "field 8 (f32)");
    assert!((val.9 - decoded.9).abs() < f64::EPSILON, "field 9 (f64)");
    assert_eq!(val.10, decoded.10, "field 10 (bool)");
    assert_eq!(val.11, decoded.11, "field 11 (String)");
    assert_eq!(
        consumed,
        bytes.len(),
        "consumed bytes must match total encoded"
    );
}

// ── Test 22: 16-tuple roundtrip ───────────────────────────────────────────────

#[test]
fn test_tuple_adv4_22_16_tuple_roundtrip() {
    type T16 = (
        u8,
        u16,
        u32,
        u64,
        i8,
        i16,
        i32,
        i64,
        f32,
        f64,
        bool,
        String,
        u8,
        u16,
        u32,
        u64,
    );
    let val: T16 = (
        1u8,
        2u16,
        3u32,
        4u64,
        -5i8,
        -6i16,
        -7i32,
        -8i64,
        9.0f32,
        10.0f64,
        true,
        "sixteen".to_string(),
        11u8,
        12u16,
        13u32,
        14u64,
    );
    let bytes = encode_to_vec(&val).expect("encode 16-tuple");
    let (decoded, consumed): (T16, usize) = decode_from_slice(&bytes).expect("decode 16-tuple");
    assert_eq!(val.0, decoded.0, "field 0");
    assert_eq!(val.1, decoded.1, "field 1");
    assert_eq!(val.2, decoded.2, "field 2");
    assert_eq!(val.3, decoded.3, "field 3");
    assert_eq!(val.4, decoded.4, "field 4");
    assert_eq!(val.5, decoded.5, "field 5");
    assert_eq!(val.6, decoded.6, "field 6");
    assert_eq!(val.7, decoded.7, "field 7");
    assert!((val.8 - decoded.8).abs() < f32::EPSILON, "field 8 (f32)");
    assert!((val.9 - decoded.9).abs() < f64::EPSILON, "field 9 (f64)");
    assert_eq!(val.10, decoded.10, "field 10 (bool)");
    assert_eq!(val.11, decoded.11, "field 11 (String)");
    assert_eq!(val.12, decoded.12, "field 12");
    assert_eq!(val.13, decoded.13, "field 13");
    assert_eq!(val.14, decoded.14, "field 14");
    assert_eq!(val.15, decoded.15, "field 15");
    assert_eq!(
        consumed,
        bytes.len(),
        "consumed bytes must match total encoded"
    );
}
