//! Advanced tuple encoding tests — edge cases not covered by tuple_test.rs or tuple_extended_test.rs.
//! Focuses on: mixed signed/unsigned, bytes, Result, big-endian config, fixed-int config,
//! char, unit type, i128/u128, fixed arrays, HashMap-key semantics, struct fields, and more.

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
#[allow(unused_imports)]
use std::collections::HashMap;

// ── 1. Mixed signed/unsigned: (i8, u8, i16, u16) ────────────────────────────

#[test]
fn test_tuple_mixed_signed_unsigned() {
    let original: (i8, u8, i16, u16) = (-1i8, 255u8, -1000i16, 60000u16);
    let encoded = encode_to_vec(&original).expect("encode (i8,u8,i16,u16) failed");
    #[allow(clippy::type_complexity)]
    let (decoded, consumed): ((i8, u8, i16, u16), _) =
        decode_from_slice(&encoded).expect("decode (i8,u8,i16,u16) failed");
    assert_eq!(decoded, (-1i8, 255u8, -1000i16, 60000u16));
    assert_eq!(consumed, encoded.len());
}

// ── 2. String and bytes: (String, Vec<u8>) ───────────────────────────────────

#[test]
fn test_tuple_string_and_bytes() {
    let original: (String, Vec<u8>) = (String::from("oxicode bytes"), vec![0u8, 1, 127, 128, 255]);
    let encoded = encode_to_vec(&original).expect("encode (String, Vec<u8>) failed");
    let (decoded, consumed): ((String, Vec<u8>), _) =
        decode_from_slice(&encoded).expect("decode (String, Vec<u8>) failed");
    assert_eq!(decoded.0, original.0);
    assert_eq!(decoded.1, original.1);
    assert_eq!(consumed, encoded.len());
}

// ── 3. Nested tuple: ((u8, u16), (i32, f64)) ────────────────────────────────

#[test]
fn test_tuple_nested_mixed_types() {
    let original: ((u8, u16), (i32, f64)) = ((42u8, 1000u16), (-99i32, std::f64::consts::PI));
    let encoded = encode_to_vec(&original).expect("encode nested tuple failed");
    #[allow(clippy::type_complexity)]
    let (decoded, consumed): (((u8, u16), (i32, f64)), _) =
        decode_from_slice(&encoded).expect("decode nested tuple failed");
    assert_eq!(decoded.0, original.0);
    assert_eq!(decoded.1 .0, original.1 .0);
    assert_eq!(decoded.1 .1, original.1 .1);
    assert_eq!(consumed, encoded.len());
}

// ── 4. Tuple containing Option: (Option<u32>, Option<String>) ────────────────

#[test]
fn test_tuple_with_options() {
    let some_some: (Option<u32>, Option<String>) = (Some(12345u32), Some(String::from("present")));
    let encoded = encode_to_vec(&some_some).expect("encode (Option<u32>, Option<String>) failed");
    let (decoded, consumed): ((Option<u32>, Option<String>), _) =
        decode_from_slice(&encoded).expect("decode (Option<u32>, Option<String>) failed");
    assert_eq!(decoded, some_some);
    assert_eq!(consumed, encoded.len());

    let none_none: (Option<u32>, Option<String>) = (None, None);
    let encoded2 = encode_to_vec(&none_none).expect("encode (None, None) failed");
    let (decoded2, consumed2): ((Option<u32>, Option<String>), _) =
        decode_from_slice(&encoded2).expect("decode (None, None) failed");
    assert_eq!(decoded2, none_none);
    assert_eq!(consumed2, encoded2.len());
}

// ── 5. Tuple containing Result: (Result<u32, String>, bool) ─────────────────

#[test]
fn test_tuple_with_result() {
    let ok_case: (Result<u32, String>, bool) = (Ok(9999u32), true);
    let encoded_ok = encode_to_vec(&ok_case).expect("encode Ok result tuple failed");
    let (decoded_ok, consumed_ok): ((Result<u32, String>, bool), _) =
        decode_from_slice(&encoded_ok).expect("decode Ok result tuple failed");
    assert_eq!(decoded_ok, ok_case);
    assert_eq!(consumed_ok, encoded_ok.len());

    let err_case: (Result<u32, String>, bool) = (Err(String::from("an error")), false);
    let encoded_err = encode_to_vec(&err_case).expect("encode Err result tuple failed");
    let (decoded_err, consumed_err): ((Result<u32, String>, bool), _) =
        decode_from_slice(&encoded_err).expect("decode Err result tuple failed");
    assert_eq!(decoded_err, err_case);
    assert_eq!(consumed_err, encoded_err.len());
}

// ── 6. Tuple with Vec<u32>: (Vec<u32>, u64) ─────────────────────────────────

#[test]
fn test_tuple_vec_u32_and_u64() {
    let original: (Vec<u32>, u64) = (vec![10u32, 20, 30, 40, u32::MAX], u64::MAX);
    let encoded = encode_to_vec(&original).expect("encode (Vec<u32>, u64) failed");
    let (decoded, consumed): ((Vec<u32>, u64), _) =
        decode_from_slice(&encoded).expect("decode (Vec<u32>, u64) failed");
    assert_eq!(decoded, original);
    assert_eq!(consumed, encoded.len());
}

// ── 7. 16-element tuple all u8 values ────────────────────────────────────────

#[test]
fn test_tuple_16_all_u8() {
    type T16u8 = (
        u8,
        u8,
        u8,
        u8,
        u8,
        u8,
        u8,
        u8,
        u8,
        u8,
        u8,
        u8,
        u8,
        u8,
        u8,
        u8,
    );
    let original: T16u8 = (0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 255);
    let encoded = encode_to_vec(&original).expect("encode 16-u8 tuple failed");
    let (decoded, consumed): (T16u8, _) =
        decode_from_slice(&encoded).expect("decode 16-u8 tuple failed");
    assert_eq!(original.0, decoded.0);
    assert_eq!(original.7, decoded.7);
    assert_eq!(original.15, decoded.15);
    assert_eq!(consumed, encoded.len());
}

// ── 8. Tuple with bool fields: (bool, bool, bool, bool) ──────────────────────

#[test]
fn test_tuple_four_bools() {
    let original: (bool, bool, bool, bool) = (true, false, true, false);
    let encoded = encode_to_vec(&original).expect("encode (bool,bool,bool,bool) failed");
    let (decoded, consumed): ((bool, bool, bool, bool), _) =
        decode_from_slice(&encoded).expect("decode (bool,bool,bool,bool) failed");
    assert_eq!(decoded, original);
    assert_eq!(consumed, encoded.len());

    // All true variant
    let all_true: (bool, bool, bool, bool) = (true, true, true, true);
    let enc_t = encode_to_vec(&all_true).expect("encode all-true bools failed");
    let (dec_t, _): ((bool, bool, bool, bool), _) =
        decode_from_slice(&enc_t).expect("decode all-true bools failed");
    assert_eq!(dec_t, all_true);
}

// ── 9. Tuple byte size: (u32, u32) — sum of component sizes ─────────────────

#[test]
fn test_tuple_byte_size_fixed_int_config() {
    // With legacy (fixed-int, little-endian) config, each u32 is exactly 4 bytes.
    // A (u32, u32) tuple must therefore encode to exactly 8 bytes.
    let original: (u32, u32) = (0xDEAD_BEEFu32, 0xCAFE_BABEu32);
    let encoded =
        encode_to_vec_with_config(&original, config::legacy()).expect("encode (u32,u32) failed");
    assert_eq!(
        encoded.len(),
        8,
        "(u32, u32) with fixed-int config must be 8 bytes"
    );
    let (decoded, consumed): ((u32, u32), _) =
        decode_from_slice_with_config(&encoded, config::legacy()).expect("decode (u32,u32) failed");
    assert_eq!(decoded, original);
    assert_eq!(consumed, 8);
}

// ── 10. Tuple with big-endian config: (u32, u64) roundtrip ───────────────────

#[test]
fn test_tuple_big_endian_config() {
    let original: (u32, u64) = (0x0102_0304u32, 0x0506_0708_090A_0B0Cu64);
    let cfg = config::standard().with_big_endian();
    let encoded =
        encode_to_vec_with_config(&original, cfg).expect("encode big-endian tuple failed");
    let (decoded, consumed): ((u32, u64), _) =
        decode_from_slice_with_config(&encoded, cfg).expect("decode big-endian tuple failed");
    assert_eq!(decoded, original);
    assert_eq!(consumed, encoded.len());
}

// ── 11. Tuple with fixed-int config: (u32, u64) → exactly 12 bytes ───────────

#[test]
fn test_tuple_fixed_int_config_exact_size() {
    let original: (u32, u64) = (42u32, 9999u64);
    // legacy = fixed-int little-endian: u32 → 4 bytes, u64 → 8 bytes = 12 total
    let encoded =
        encode_to_vec_with_config(&original, config::legacy()).expect("encode fixed-int failed");
    assert_eq!(
        encoded.len(),
        12,
        "(u32, u64) with fixed-int config must be exactly 12 bytes"
    );
    let (decoded, consumed): ((u32, u64), _) =
        decode_from_slice_with_config(&encoded, config::legacy()).expect("decode fixed-int failed");
    assert_eq!(decoded, original);
    assert_eq!(consumed, 12);
}

// ── 12. Vec<(u8, u8)> roundtrip (vec of tuples) ──────────────────────────────

#[test]
fn test_vec_of_u8_u8_tuples() {
    let original: Vec<(u8, u8)> = vec![(0, 0), (1, 255), (127, 128), (255, 0)];
    let encoded = encode_to_vec(&original).expect("encode Vec<(u8,u8)> failed");
    let (decoded, consumed): (Vec<(u8, u8)>, _) =
        decode_from_slice(&encoded).expect("decode Vec<(u8,u8)> failed");
    assert_eq!(decoded, original);
    assert_eq!(consumed, encoded.len());
}

// ── 13. Option<(u32, String)> Some and None roundtrip ────────────────────────

#[test]
fn test_option_of_tuple_some_and_none() {
    let some_val: Option<(u32, String)> = Some((777u32, String::from("hello option")));
    let encoded_some = encode_to_vec(&some_val).expect("encode Some((u32,String)) failed");
    let (decoded_some, consumed_some): (Option<(u32, String)>, _) =
        decode_from_slice(&encoded_some).expect("decode Some((u32,String)) failed");
    assert_eq!(decoded_some, some_val);
    assert_eq!(consumed_some, encoded_some.len());

    let none_val: Option<(u32, String)> = None;
    let encoded_none = encode_to_vec(&none_val).expect("encode None<(u32,String)> failed");
    let (decoded_none, consumed_none): (Option<(u32, String)>, _) =
        decode_from_slice(&encoded_none).expect("decode None<(u32,String)> failed");
    assert_eq!(decoded_none, none_val);
    assert_eq!(consumed_none, encoded_none.len());
}

// ── 14. Tuple with char: (char, u32) roundtrip ───────────────────────────────

#[test]
fn test_tuple_with_char() {
    let ascii: (char, u32) = ('A', 65u32);
    let encoded_ascii = encode_to_vec(&ascii).expect("encode (char, u32) ASCII failed");
    let (decoded_ascii, consumed_ascii): ((char, u32), _) =
        decode_from_slice(&encoded_ascii).expect("decode (char, u32) ASCII failed");
    assert_eq!(decoded_ascii, ascii);
    assert_eq!(consumed_ascii, encoded_ascii.len());

    let unicode: (char, u32) = ('界', 30028u32);
    let encoded_unicode = encode_to_vec(&unicode).expect("encode (char, u32) unicode failed");
    let (decoded_unicode, consumed_unicode): ((char, u32), _) =
        decode_from_slice(&encoded_unicode).expect("decode (char, u32) unicode failed");
    assert_eq!(decoded_unicode, unicode);
    assert_eq!(consumed_unicode, encoded_unicode.len());

    let emoji: (char, u32) = ('🦀', 129408u32);
    let encoded_emoji = encode_to_vec(&emoji).expect("encode (char, u32) emoji failed");
    let (decoded_emoji, consumed_emoji): ((char, u32), _) =
        decode_from_slice(&encoded_emoji).expect("decode (char, u32) emoji failed");
    assert_eq!(decoded_emoji, emoji);
    assert_eq!(consumed_emoji, encoded_emoji.len());
}

// ── 15. Tuple with unit type: ((), u32, ()) roundtrip ────────────────────────

#[test]
fn test_tuple_with_unit_type() {
    let original: ((), u32, ()) = ((), 42u32, ());
    let encoded = encode_to_vec(&original).expect("encode ((), u32, ()) failed");
    let (decoded, consumed): (((), u32, ()), _) =
        decode_from_slice(&encoded).expect("decode ((), u32, ()) failed");
    assert_eq!(decoded, original);
    assert_eq!(consumed, encoded.len());
}

// ── 16. 2-tuple of large vecs: (Vec<u8>, Vec<u8>) roundtrip ──────────────────

#[test]
fn test_tuple_of_large_vecs() {
    let vec_a: Vec<u8> = (0u8..=255u8).collect();
    let vec_b: Vec<u8> = (0u8..=255u8).rev().collect();
    let original: (Vec<u8>, Vec<u8>) = (vec_a, vec_b);
    let encoded = encode_to_vec(&original).expect("encode (Vec<u8>, Vec<u8>) large failed");
    let (decoded, consumed): ((Vec<u8>, Vec<u8>), _) =
        decode_from_slice(&encoded).expect("decode (Vec<u8>, Vec<u8>) large failed");
    assert_eq!(decoded.0, original.0);
    assert_eq!(decoded.1, original.1);
    assert_eq!(consumed, encoded.len());
}

// ── 17. Deeply nested tuples: (((u8,), u8), u8) roundtrip ────────────────────

#[test]
fn test_tuple_deeply_nested() {
    let original: (((u8,), u8), u8) = (((7u8,), 14u8), 21u8);
    let encoded = encode_to_vec(&original).expect("encode (((u8,), u8), u8) failed");
    let (decoded, consumed): ((((u8,), u8), u8), _) =
        decode_from_slice(&encoded).expect("decode (((u8,), u8), u8) failed");
    assert_eq!(decoded.0 .0 .0, original.0 .0 .0);
    assert_eq!(decoded.0 .1, original.0 .1);
    assert_eq!(decoded.1, original.1);
    assert_eq!(consumed, encoded.len());
}

// ── 18. Tuple with i128 and u128: (i128, u128) roundtrip ─────────────────────

#[test]
fn test_tuple_i128_u128() {
    let original: (i128, u128) = (i128::MIN, u128::MAX);
    let encoded = encode_to_vec(&original).expect("encode (i128, u128) failed");
    let (decoded, consumed): ((i128, u128), _) =
        decode_from_slice(&encoded).expect("decode (i128, u128) failed");
    assert_eq!(decoded, original);
    assert_eq!(consumed, encoded.len());

    let zero_half: (i128, u128) = (0i128, u128::MAX / 2);
    let encoded2 = encode_to_vec(&zero_half).expect("encode (i128, u128) zero_half failed");
    let (decoded2, consumed2): ((i128, u128), _) =
        decode_from_slice(&encoded2).expect("decode (i128, u128) zero_half failed");
    assert_eq!(decoded2, zero_half);
    assert_eq!(consumed2, encoded2.len());
}

// ── 19. Tuple with [u8; 8] array: ([u8; 8], u64) roundtrip ───────────────────

#[test]
fn test_tuple_fixed_array_and_u64() {
    let original: ([u8; 8], u64) = (
        [0xDE, 0xAD, 0xBE, 0xEF, 0xCA, 0xFE, 0xBA, 0xBE],
        12345678u64,
    );
    let encoded = encode_to_vec(&original).expect("encode ([u8;8], u64) failed");
    let (decoded, consumed): (([u8; 8], u64), _) =
        decode_from_slice(&encoded).expect("decode ([u8;8], u64) failed");
    assert_eq!(decoded.0, original.0);
    assert_eq!(decoded.1, original.1);
    assert_eq!(consumed, encoded.len());

    let zeros: ([u8; 8], u64) = ([0u8; 8], 0u64);
    let enc_zeros = encode_to_vec(&zeros).expect("encode ([0u8;8], 0u64) failed");
    let (dec_zeros, _): (([u8; 8], u64), _) =
        decode_from_slice(&enc_zeros).expect("decode ([0u8;8], 0u64) failed");
    assert_eq!(dec_zeros, zeros);
}

// ── 20. Vec of tuple pairs as sorted map substitute ───────────────────────────
//
// Uses Vec<((u8, u8), String)> to simulate sorted-key map encoding, verifying
// that tuple-structured composite keys round-trip correctly.

#[test]
fn test_vec_of_tuple_pairs() {
    let pairs: Vec<((u8, u8), String)> = vec![
        ((1u8, 2u8), String::from("one-two")),
        ((3u8, 4u8), String::from("three-four")),
        ((255u8, 0u8), String::from("max-zero")),
    ];
    let encoded = encode_to_vec(&pairs).expect("encode vec of tuple pairs failed");
    let (decoded, consumed): (Vec<((u8, u8), String)>, _) =
        decode_from_slice(&encoded).expect("decode vec of tuple pairs failed");
    assert_eq!(decoded, pairs);
    assert_eq!(consumed, encoded.len());
}

// ── 21. Struct containing a tuple field roundtrip ────────────────────────────

#[derive(Debug, PartialEq, oxicode::Encode, oxicode::Decode)]
struct WithTuple {
    coords: (f32, f32),
    label: String,
}

#[test]
fn test_struct_with_tuple_field() {
    let original = WithTuple {
        coords: (1.5f32, -2.75f32),
        label: String::from("origin shift"),
    };
    let encoded = encode_to_vec(&original).expect("encode WithTuple failed");
    let (decoded, consumed): (WithTuple, _) =
        decode_from_slice(&encoded).expect("decode WithTuple failed");
    assert_eq!(decoded, original);
    assert_eq!(consumed, encoded.len());

    // Boundary coordinates
    let boundary = WithTuple {
        coords: (f32::MAX, f32::MIN_POSITIVE),
        label: String::from("boundary"),
    };
    let enc_b = encode_to_vec(&boundary).expect("encode WithTuple boundary failed");
    let (dec_b, _): (WithTuple, _) =
        decode_from_slice(&enc_b).expect("decode WithTuple boundary failed");
    assert_eq!(dec_b, boundary);

    // Vec of structs containing tuple fields
    let items: Vec<WithTuple> = vec![
        WithTuple {
            coords: (0.0f32, 0.0f32),
            label: String::from("zero"),
        },
        WithTuple {
            coords: (-1.0f32, 1.0f32),
            label: String::from("unit"),
        },
    ];
    let enc_items = encode_to_vec(&items).expect("encode Vec<WithTuple> failed");
    let (dec_items, _): (Vec<WithTuple>, _) =
        decode_from_slice(&enc_items).expect("decode Vec<WithTuple> failed");
    assert_eq!(dec_items, items);
}

// ── 22. Tuple of strings: (String, String, String, String) roundtrip ──────────

#[test]
fn test_tuple_four_strings() {
    let original: (String, String, String, String) = (
        String::from("first"),
        String::from("second"),
        String::from("third"),
        String::from("fourth"),
    );
    let encoded = encode_to_vec(&original).expect("encode (String,String,String,String) failed");
    let (decoded, consumed): ((String, String, String, String), _) =
        decode_from_slice(&encoded).expect("decode (String,String,String,String) failed");
    assert_eq!(decoded, original);
    assert_eq!(consumed, encoded.len());

    // Empty strings
    let empty: (String, String, String, String) =
        (String::new(), String::new(), String::new(), String::new());
    let enc_empty = encode_to_vec(&empty).expect("encode empty strings tuple failed");
    let (dec_empty, consumed_empty): ((String, String, String, String), _) =
        decode_from_slice(&enc_empty).expect("decode empty strings tuple failed");
    assert_eq!(dec_empty, empty);
    assert_eq!(consumed_empty, enc_empty.len());

    // Unicode strings
    let unicode: (String, String, String, String) = (
        String::from("日本語"),
        String::from("한국어"),
        String::from("中文"),
        String::from("العربية"),
    );
    let enc_unicode = encode_to_vec(&unicode).expect("encode unicode strings tuple failed");
    let (dec_unicode, _): ((String, String, String, String), _) =
        decode_from_slice(&enc_unicode).expect("decode unicode strings tuple failed");
    assert_eq!(dec_unicode, unicode);
}
