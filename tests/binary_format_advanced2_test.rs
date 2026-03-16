//! Tests verifying exact wire bytes for OxiCode's binary format.
//!
//! Each test encodes a value with a specific configuration and asserts the
//! exact byte sequence produced, covering booleans, unsigned integers,
//! sequences, strings, options, fixed-int encoding, varint encoding,
//! tuples, fixed arrays, structs, and enums.

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

// ── helpers ──────────────────────────────────────────────────────────────────

/// Encode `value` with the standard config and return the byte vector.
fn std_encode<E: Encode>(value: &E) -> Vec<u8> {
    encode_to_vec(value).expect("encode_to_vec failed")
}

/// Encode `value` with a caller-supplied config and return the byte vector.
fn cfg_encode<E: Encode, C: config::Config>(value: &E, cfg: C) -> Vec<u8> {
    encode_to_vec_with_config(value, cfg).expect("encode_to_vec_with_config failed")
}

// ── shared derive types ───────────────────────────────────────────────────────

#[derive(Debug, PartialEq, Encode, Decode)]
struct TwoBytes {
    x: u8,
    y: u8,
}

#[derive(Debug, PartialEq, Encode, Decode)]
enum Unit2 {
    First,
    Second,
}

// ── test 1: `true` → `[0x01]` ────────────────────────────────────────────────

#[test]
fn wire_bool_true_is_one_byte_0x01() {
    let bytes = std_encode(&true);
    assert_eq!(bytes, &[0x01], "true must encode to [0x01]");
    let (val, consumed): (bool, _) = decode_from_slice(&bytes).expect("decode bool true failed");
    assert!(val);
    assert_eq!(consumed, 1);
}

// ── test 2: `false` → `[0x00]` ───────────────────────────────────────────────

#[test]
fn wire_bool_false_is_one_byte_0x00() {
    let bytes = std_encode(&false);
    assert_eq!(bytes, &[0x00], "false must encode to [0x00]");
    let (val, consumed): (bool, _) = decode_from_slice(&bytes).expect("decode bool false failed");
    assert!(!val);
    assert_eq!(consumed, 1);
}

// ── test 3: `u8(42)` → `[0x2A]` ──────────────────────────────────────────────

#[test]
fn wire_u8_42_is_0x2a() {
    let bytes = std_encode(&42u8);
    assert_eq!(bytes, &[0x2A], "u8(42) must encode to [0x2A]");
    let (val, consumed): (u8, _) = decode_from_slice(&bytes).expect("decode u8(42) failed");
    assert_eq!(val, 42u8);
    assert_eq!(consumed, 1);
}

// ── test 4: `u8(127)` → `[0x7F]` ─────────────────────────────────────────────

#[test]
fn wire_u8_127_is_0x7f() {
    let bytes = std_encode(&127u8);
    assert_eq!(bytes, &[0x7F], "u8(127) must encode to [0x7F]");
    let (val, consumed): (u8, _) = decode_from_slice(&bytes).expect("decode u8(127) failed");
    assert_eq!(val, 127u8);
    assert_eq!(consumed, 1);
}

// ── test 5: `u8(128)` → `[0x80]` ─────────────────────────────────────────────

#[test]
fn wire_u8_128_is_0x80() {
    let bytes = std_encode(&128u8);
    assert_eq!(bytes, &[0x80], "u8(128) must encode to [0x80]");
    let (val, consumed): (u8, _) = decode_from_slice(&bytes).expect("decode u8(128) failed");
    assert_eq!(val, 128u8);
    assert_eq!(consumed, 1);
}

// ── test 6: `u8(255)` → `[0xFF]` ─────────────────────────────────────────────

#[test]
fn wire_u8_255_is_0xff() {
    let bytes = std_encode(&255u8);
    assert_eq!(bytes, &[0xFF], "u8(255) must encode to [0xFF]");
    let (val, consumed): (u8, _) = decode_from_slice(&bytes).expect("decode u8(255) failed");
    assert_eq!(val, 255u8);
    assert_eq!(consumed, 1);
}

// ── test 7: empty `Vec<u8>` → `[0x00]` ───────────────────────────────────────

#[test]
fn wire_empty_vec_u8_is_0x00() {
    let v: Vec<u8> = Vec::new();
    let bytes = std_encode(&v);
    assert_eq!(
        bytes,
        &[0x00],
        "empty Vec<u8> must encode to [0x00] (varint 0 length)"
    );
    let (decoded, consumed): (Vec<u8>, _) =
        decode_from_slice(&bytes).expect("decode empty Vec<u8> failed");
    assert!(decoded.is_empty());
    assert_eq!(consumed, 1);
}

// ── test 8: `Vec<u8>([1,2,3])` → `[0x03, 0x01, 0x02, 0x03]` ─────────────────

#[test]
fn wire_vec_u8_three_elements_has_length_prefix() {
    let v: Vec<u8> = vec![1, 2, 3];
    let bytes = std_encode(&v);
    assert_eq!(
        bytes,
        &[0x03, 0x01, 0x02, 0x03],
        "Vec<u8>[1,2,3] must encode to [0x03, 0x01, 0x02, 0x03]"
    );
    let (decoded, consumed): (Vec<u8>, _) =
        decode_from_slice(&bytes).expect("decode Vec<u8>[1,2,3] failed");
    assert_eq!(decoded, vec![1u8, 2, 3]);
    assert_eq!(consumed, 4);
}

// ── test 9: empty `String` → `[0x00]` ────────────────────────────────────────

#[test]
fn wire_empty_string_is_0x00() {
    let s = String::new();
    let bytes = std_encode(&s);
    assert_eq!(bytes, &[0x00], "empty String must encode to [0x00]");
    let (decoded, consumed): (String, _) =
        decode_from_slice(&bytes).expect("decode empty String failed");
    assert!(decoded.is_empty());
    assert_eq!(consumed, 1);
}

// ── test 10: `String("a")` → `[0x01, 0x61]` ──────────────────────────────────

#[test]
fn wire_string_a_is_length_then_ascii() {
    let s = String::from("a");
    let bytes = std_encode(&s);
    assert_eq!(
        bytes,
        &[0x01, 0x61],
        "String(\"a\") must encode to [0x01, 0x61]"
    );
    let (decoded, consumed): (String, _) =
        decode_from_slice(&bytes).expect("decode String(\"a\") failed");
    assert_eq!(decoded, "a");
    assert_eq!(consumed, 2);
}

// ── test 11: `Option<u8>(None)` → `[0x00]` ───────────────────────────────────

#[test]
fn wire_option_u8_none_is_0x00() {
    let opt: Option<u8> = None;
    let bytes = std_encode(&opt);
    assert_eq!(bytes, &[0x00], "Option<u8>(None) must encode to [0x00]");
    let (decoded, consumed): (Option<u8>, _) =
        decode_from_slice(&bytes).expect("decode Option<u8>(None) failed");
    assert_eq!(decoded, None);
    assert_eq!(consumed, 1);
}

// ── test 12: `Option<u8>(Some(1))` → `[0x01, 0x01]` ─────────────────────────

#[test]
fn wire_option_u8_some_1_is_0x01_0x01() {
    let opt: Option<u8> = Some(1);
    let bytes = std_encode(&opt);
    assert_eq!(
        bytes,
        &[0x01, 0x01],
        "Option<u8>(Some(1)) must encode to [0x01, 0x01]"
    );
    let (decoded, consumed): (Option<u8>, _) =
        decode_from_slice(&bytes).expect("decode Option<u8>(Some(1)) failed");
    assert_eq!(decoded, Some(1u8));
    assert_eq!(consumed, 2);
}

// ── test 13: fixed-int `u32(1)` → `[0x01, 0x00, 0x00, 0x00]` ────────────────

#[test]
fn wire_fixed_int_u32_1_little_endian() {
    let cfg = config::standard().with_fixed_int_encoding();
    let bytes = cfg_encode(&1u32, cfg);
    assert_eq!(
        bytes,
        &[0x01, 0x00, 0x00, 0x00],
        "fixed-int u32(1) must encode to [0x01, 0x00, 0x00, 0x00] (little-endian)"
    );
    let (decoded, consumed): (u32, _) =
        decode_from_slice_with_config(&bytes, cfg).expect("decode fixed u32(1) failed");
    assert_eq!(decoded, 1u32);
    assert_eq!(consumed, 4);
}

// ── test 14: fixed-int `u32(0x01020304)` → `[0x04, 0x03, 0x02, 0x01]` ───────

#[test]
fn wire_fixed_int_u32_0x01020304_little_endian() {
    let cfg = config::standard().with_fixed_int_encoding();
    let bytes = cfg_encode(&0x01020304u32, cfg);
    assert_eq!(
        bytes,
        &[0x04, 0x03, 0x02, 0x01],
        "fixed-int u32(0x01020304) must encode to [0x04, 0x03, 0x02, 0x01] (little-endian)"
    );
    let (decoded, consumed): (u32, _) =
        decode_from_slice_with_config(&bytes, cfg).expect("decode fixed u32(0x01020304) failed");
    assert_eq!(decoded, 0x01020304u32);
    assert_eq!(consumed, 4);
}

// ── test 15: big-endian fixed-int `u32(0x01020304)` → `[0x01, 0x02, 0x03, 0x04]`

#[test]
fn wire_fixed_int_u32_big_endian() {
    let cfg = config::standard()
        .with_fixed_int_encoding()
        .with_big_endian();
    let bytes = cfg_encode(&0x01020304u32, cfg);
    assert_eq!(
        bytes,
        &[0x01, 0x02, 0x03, 0x04],
        "big-endian fixed-int u32(0x01020304) must encode to [0x01, 0x02, 0x03, 0x04]"
    );
    let (decoded, consumed): (u32, _) =
        decode_from_slice_with_config(&bytes, cfg).expect("decode big-endian u32 failed");
    assert_eq!(decoded, 0x01020304u32);
    assert_eq!(consumed, 4);
}

// ── test 16: varint `u32(1)` → `[0x01]` (1 byte) ─────────────────────────────

#[test]
fn wire_varint_u32_1_is_one_byte() {
    let bytes = std_encode(&1u32);
    assert_eq!(
        bytes,
        &[0x01],
        "varint u32(1) must encode to [0x01] (1 byte)"
    );
    let (decoded, consumed): (u32, _) =
        decode_from_slice(&bytes).expect("decode varint u32(1) failed");
    assert_eq!(decoded, 1u32);
    assert_eq!(consumed, 1);
}

// ── test 17: varint `u64(0)` → `[0x00]` (1 byte) ─────────────────────────────

#[test]
fn wire_varint_u64_0_is_one_byte() {
    let bytes = std_encode(&0u64);
    assert_eq!(
        bytes,
        &[0x00],
        "varint u64(0) must encode to [0x00] (1 byte)"
    );
    let (decoded, consumed): (u64, _) =
        decode_from_slice(&bytes).expect("decode varint u64(0) failed");
    assert_eq!(decoded, 0u64);
    assert_eq!(consumed, 1);
}

// ── test 18: empty tuple `()` → 0 bytes ──────────────────────────────────────

#[test]
fn wire_unit_tuple_encodes_to_zero_bytes() {
    let bytes = std_encode(&());
    assert!(
        bytes.is_empty(),
        "unit tuple () must encode to 0 bytes, got {:?}",
        bytes
    );
    let ((), consumed): ((), _) = decode_from_slice(&bytes).expect("decode () failed");
    assert_eq!(consumed, 0);
}

// ── test 19: `[u8; 0]` → 0 bytes ─────────────────────────────────────────────

#[test]
fn wire_fixed_array_empty_encodes_to_zero_bytes() {
    let arr: [u8; 0] = [];
    let bytes = std_encode(&arr);
    assert!(
        bytes.is_empty(),
        "[u8; 0] must encode to 0 bytes, got {:?}",
        bytes
    );
    let (decoded, consumed): ([u8; 0], _) =
        decode_from_slice(&bytes).expect("decode [u8; 0] failed");
    assert_eq!(decoded, []);
    assert_eq!(consumed, 0);
}

// ── test 20: `[u8; 3]([1,2,3])` → `[0x01, 0x02, 0x03]` (no length prefix) ───

#[test]
fn wire_fixed_array_three_bytes_has_no_length_prefix() {
    let arr: [u8; 3] = [1, 2, 3];
    let bytes = std_encode(&arr);
    assert_eq!(
        bytes,
        &[0x01, 0x02, 0x03],
        "[u8; 3]([1,2,3]) must encode to [0x01, 0x02, 0x03] without a length prefix"
    );
    let (decoded, consumed): ([u8; 3], _) =
        decode_from_slice(&bytes).expect("decode [u8; 3] failed");
    assert_eq!(decoded, [1u8, 2, 3]);
    assert_eq!(consumed, 3);
}

// ── test 21: struct `{x:1, y:2}` → `[0x01, 0x02]` ───────────────────────────

#[test]
fn wire_struct_two_u8_fields_concatenated() {
    let s = TwoBytes { x: 1, y: 2 };
    let bytes = std_encode(&s);
    assert_eq!(
        bytes,
        &[0x01, 0x02],
        "struct {{x:1, y:2}} must encode to [0x01, 0x02] (fields concatenated)"
    );
    let (decoded, consumed): (TwoBytes, _) =
        decode_from_slice(&bytes).expect("decode TwoBytes failed");
    assert_eq!(decoded, TwoBytes { x: 1, y: 2 });
    assert_eq!(consumed, 2);
}

// ── test 22: first unit enum variant → `[0x00]` ──────────────────────────────

#[test]
fn wire_enum_first_unit_variant_is_0x00() {
    let v = Unit2::First;
    let bytes = std_encode(&v);
    assert_eq!(
        bytes,
        &[0x00],
        "first unit enum variant must encode to [0x00] (varint discriminant 0)"
    );
    let (decoded, consumed): (Unit2, _) =
        decode_from_slice(&bytes).expect("decode Unit2::First failed");
    assert_eq!(decoded, Unit2::First);
    assert_eq!(consumed, 1);
}
