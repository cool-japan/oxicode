//! Advanced plain-tuple encoding tests for OxiCode — 22 top-level #[test] functions.

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
    encode_to_vec_with_config,
};

// ── Test 1: (u32,) single-element tuple roundtrip ─────────────────────────────

#[test]
fn test_single_element_tuple_roundtrip() {
    let original: (u32,) = (42u32,);
    let enc = encode_to_vec(&original).expect("encode (u32,)");
    let (val, _): ((u32,), usize) = decode_from_slice(&enc).expect("decode (u32,)");
    assert_eq!(original, val);
}

// ── Test 2: (u32, u32) two identical types roundtrip ─────────────────────────

#[test]
fn test_two_identical_types_roundtrip() {
    let original: (u32, u32) = (100u32, 200u32);
    let enc = encode_to_vec(&original).expect("encode (u32, u32)");
    let (val, _): ((u32, u32), usize) = decode_from_slice(&enc).expect("decode (u32, u32)");
    assert_eq!(original, val);
}

// ── Test 3: (u32, String) mixed types roundtrip ───────────────────────────────

#[test]
fn test_u32_string_tuple_roundtrip() {
    let original: (u32, String) = (7u32, String::from("oxicode"));
    let enc = encode_to_vec(&original).expect("encode (u32, String)");
    let (val, _): ((u32, String), usize) = decode_from_slice(&enc).expect("decode (u32, String)");
    assert_eq!(original, val);
}

// ── Test 4: (u32, String, bool) 3-tuple roundtrip ────────────────────────────

#[test]
fn test_u32_string_bool_3tuple_roundtrip() {
    let original: (u32, String, bool) = (99u32, String::from("hello"), true);
    let enc = encode_to_vec(&original).expect("encode (u32, String, bool)");
    let (val, _): ((u32, String, bool), usize) =
        decode_from_slice(&enc).expect("decode (u32, String, bool)");
    assert_eq!(original, val);
}

// ── Test 5: (u8, u16, u32, u64) 4-tuple with different integer sizes ──────────

#[test]
fn test_4tuple_different_integer_sizes_roundtrip() {
    let original: (u8, u16, u32, u64) = (0xFFu8, 0x1234u16, 0xDEAD_BEEFu32, u64::MAX);
    let enc = encode_to_vec(&original).expect("encode (u8, u16, u32, u64)");
    let (val, _): ((u8, u16, u32, u64), usize) =
        decode_from_slice(&enc).expect("decode (u8, u16, u32, u64)");
    assert_eq!(original, val);
}

// ── Test 6: (bool, bool, bool) tuple of bools ─────────────────────────────────

#[test]
fn test_3tuple_of_bools_roundtrip() {
    let original: (bool, bool, bool) = (true, false, true);
    let enc = encode_to_vec(&original).expect("encode (bool, bool, bool)");
    let (val, _): ((bool, bool, bool), usize) =
        decode_from_slice(&enc).expect("decode (bool, bool, bool)");
    assert_eq!(original, val);
}

// ── Test 7: (String, String) two strings roundtrip ───────────────────────────

#[test]
fn test_two_strings_tuple_roundtrip() {
    let original: (String, String) = (String::from("foo"), String::from("bar"));
    let enc = encode_to_vec(&original).expect("encode (String, String)");
    let (val, _): ((String, String), usize) =
        decode_from_slice(&enc).expect("decode (String, String)");
    assert_eq!(original, val);
}

// ── Test 8: (Vec<u8>, Vec<u8>) two vecs roundtrip ────────────────────────────

#[test]
fn test_two_vecs_tuple_roundtrip() {
    let original: (Vec<u8>, Vec<u8>) = (vec![0xDE, 0xAD], vec![0xBE, 0xEF]);
    let enc = encode_to_vec(&original).expect("encode (Vec<u8>, Vec<u8>)");
    let (val, _): ((Vec<u8>, Vec<u8>), usize) =
        decode_from_slice(&enc).expect("decode (Vec<u8>, Vec<u8>)");
    assert_eq!(original, val);
}

// ── Test 9: (Option<u32>, Option<u32>) — both Some ───────────────────────────

#[test]
fn test_tuple_of_options_both_some_roundtrip() {
    let original: (Option<u32>, Option<u32>) = (Some(1u32), Some(2u32));
    let enc = encode_to_vec(&original).expect("encode (Option<u32>, Option<u32>) both Some");
    let (val, _): ((Option<u32>, Option<u32>), usize) =
        decode_from_slice(&enc).expect("decode (Option<u32>, Option<u32>) both Some");
    assert_eq!(original, val);
}

// ── Test 10: (Option<u32>, Option<u32>) — first None, second Some ─────────────

#[test]
fn test_tuple_of_options_first_none_second_some_roundtrip() {
    let original: (Option<u32>, Option<u32>) = (None, Some(42u32));
    let enc = encode_to_vec(&original).expect("encode (Option<u32>, Option<u32>) None+Some");
    let (val, _): ((Option<u32>, Option<u32>), usize) =
        decode_from_slice(&enc).expect("decode (Option<u32>, Option<u32>) None+Some");
    assert_eq!(original, val);
}

// ── Test 11: (u64, u64) byte size check — two u64::MAX values ─────────────────

#[test]
fn test_two_u64_max_byte_size_check() {
    let original: (u64, u64) = (u64::MAX, u64::MAX);
    let enc = encode_to_vec(&original).expect("encode (u64::MAX, u64::MAX)");
    // varint encoding of u64::MAX is 9 bytes each, so tuple is at least 2 bytes
    assert!(
        enc.len() >= 2,
        "encoded size must be at least 2 bytes, got {}",
        enc.len()
    );
    let (val, _): ((u64, u64), usize) =
        decode_from_slice(&enc).expect("decode (u64::MAX, u64::MAX)");
    assert_eq!(original, val);
}

// ── Test 12: Tuple (u32, String) consumed bytes == encoded length ─────────────

#[test]
fn test_u32_string_consumed_bytes_equals_encoded_length() {
    let original: (u32, String) = (55u32, String::from("consumed"));
    let enc = encode_to_vec(&original).expect("encode (u32, String) for consumed check");
    let (_, consumed): ((u32, String), usize) =
        decode_from_slice(&enc).expect("decode (u32, String) for consumed check");
    assert_eq!(
        consumed,
        enc.len(),
        "consumed bytes must equal encoded length"
    );
}

// ── Test 13: Tuple encoding == concatenation of individually encoded fields ───

#[test]
fn test_tuple_encoding_equals_concatenation_of_fields() {
    let original: (u32, String) = (42u32, String::from("hello"));
    let enc_tuple = encode_to_vec(&original).expect("encode tuple (u32, String)");
    let enc_u32 = encode_to_vec(&42u32).expect("encode u32 alone");
    let enc_str = encode_to_vec(&String::from("hello")).expect("encode String alone");
    let concatenated: Vec<u8> = [enc_u32, enc_str].concat();
    assert_eq!(
        enc_tuple, concatenated,
        "tuple encoding must equal concatenation of individually encoded fields"
    );
}

// ── Test 14: (i64, i64) with negative values ──────────────────────────────────

#[test]
fn test_i64_i64_negative_values_roundtrip() {
    let original: (i64, i64) = (i64::MIN, -1i64);
    let enc = encode_to_vec(&original).expect("encode (i64, i64) negative");
    let (val, _): ((i64, i64), usize) =
        decode_from_slice(&enc).expect("decode (i64, i64) negative");
    assert_eq!(original, val);
}

// ── Test 15: (f32, f64) float tuple roundtrip ─────────────────────────────────

#[test]
fn test_f32_f64_float_tuple_roundtrip() {
    let original: (f32, f64) = (core::f32::consts::PI, core::f64::consts::E);
    let enc = encode_to_vec(&original).expect("encode (f32, f64)");
    let (val, _): ((f32, f64), usize) = decode_from_slice(&enc).expect("decode (f32, f64)");
    assert_eq!(original.0.to_bits(), val.0.to_bits(), "f32 bits mismatch");
    assert_eq!(original.1.to_bits(), val.1.to_bits(), "f64 bits mismatch");
}

// ── Test 16: (u128, i128) large integer tuple ─────────────────────────────────

#[test]
fn test_u128_i128_large_integer_tuple_roundtrip() {
    let original: (u128, i128) = (u128::MAX, i128::MIN);
    let enc = encode_to_vec(&original).expect("encode (u128, i128)");
    let (val, _): ((u128, i128), usize) = decode_from_slice(&enc).expect("decode (u128, i128)");
    assert_eq!(original, val);
}

// ── Test 17: Nested tuple ((u32, u32), u32) roundtrip ─────────────────────────

#[test]
fn test_nested_tuple_roundtrip() {
    let original: ((u32, u32), u32) = ((10u32, 20u32), 30u32);
    let enc = encode_to_vec(&original).expect("encode ((u32, u32), u32)");
    let (val, _): (((u32, u32), u32), usize) =
        decode_from_slice(&enc).expect("decode ((u32, u32), u32)");
    assert_eq!(original, val);
}

// ── Test 18: Tuple in Vec: Vec<(u32, String)> roundtrip ──────────────────────

#[test]
fn test_vec_of_tuples_roundtrip() {
    let original: Vec<(u32, String)> = vec![
        (1u32, String::from("alpha")),
        (2u32, String::from("beta")),
        (3u32, String::from("gamma")),
    ];
    let enc = encode_to_vec(&original).expect("encode Vec<(u32, String)>");
    let (val, _): (Vec<(u32, String)>, usize) =
        decode_from_slice(&enc).expect("decode Vec<(u32, String)>");
    assert_eq!(original, val);
}

// ── Test 19: Tuple in Option: Option<(u32, String)> Some roundtrip ────────────

#[test]
fn test_option_tuple_some_roundtrip() {
    let original: Option<(u32, String)> = Some((99u32, String::from("present")));
    let enc = encode_to_vec(&original).expect("encode Option<(u32, String)> Some");
    let (val, _): (Option<(u32, String)>, usize) =
        decode_from_slice(&enc).expect("decode Option<(u32, String)> Some");
    assert_eq!(original, val);
}

// ── Test 20: Tuple in Option: Option<(u32, String)> None roundtrip ────────────

#[test]
fn test_option_tuple_none_roundtrip() {
    let original: Option<(u32, String)> = None;
    let enc = encode_to_vec(&original).expect("encode Option<(u32, String)> None");
    let (val, _): (Option<(u32, String)>, usize) =
        decode_from_slice(&enc).expect("decode Option<(u32, String)> None");
    assert_eq!(original, val);
}

// ── Test 21: (u32, String, Vec<u8>, bool) 4-element mixed tuple roundtrip ─────

#[test]
fn test_4element_mixed_tuple_roundtrip() {
    let original: (u32, String, Vec<u8>, bool) = (
        12345u32,
        String::from("mixed"),
        vec![0x01u8, 0x02, 0x03],
        false,
    );
    let enc = encode_to_vec(&original).expect("encode (u32, String, Vec<u8>, bool)");
    let (val, _): ((u32, String, Vec<u8>, bool), usize) =
        decode_from_slice(&enc).expect("decode (u32, String, Vec<u8>, bool)");
    assert_eq!(original, val);
}

// ── Test 22: Fixed-int config with (u32, u32) — both become 4 bytes each ──────

#[test]
fn test_fixed_int_config_u32_u32_total_8_bytes() {
    let cfg = config::standard().with_fixed_int_encoding();
    let original: (u32, u32) = (0u32, u32::MAX);
    let enc = encode_to_vec_with_config(&original, cfg).expect("encode (u32, u32) fixed-int");
    assert_eq!(
        enc.len(),
        8,
        "fixed-int (u32, u32) must encode to exactly 8 bytes, got {}",
        enc.len()
    );
    let (val, _): ((u32, u32), usize) =
        decode_from_slice_with_config(&enc, cfg).expect("decode (u32, u32) fixed-int");
    assert_eq!(original, val);
}
