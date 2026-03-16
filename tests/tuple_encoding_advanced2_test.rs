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

#[derive(Debug, PartialEq, Encode, Decode)]
struct Pair(u32, u32);

#[derive(Debug, PartialEq, Encode, Decode)]
struct Triple(u8, u16, u32);

// ── Test 1: (u32,) single-element tuple roundtrip ─────────────────────────────

#[test]
fn test_tuple_1_roundtrip() {
    let val: (u32,) = (42,);
    let enc = encode_to_vec(&val).expect("encode tuple 1");
    let (dec, _): ((u32,), usize) = decode_from_slice(&enc).expect("decode tuple 1");
    assert_eq!(val, dec);
}

// ── Test 2: (u32, u32) two-element tuple roundtrip ────────────────────────────

#[test]
fn test_tuple_2_u32_u32_roundtrip() {
    let val: (u32, u32) = (10u32, 20u32);
    let enc = encode_to_vec(&val).expect("encode (u32, u32)");
    let (dec, _): ((u32, u32), usize) = decode_from_slice(&enc).expect("decode (u32, u32)");
    assert_eq!(val, dec);
}

// ── Test 3: (u32, String) mixed-type roundtrip ────────────────────────────────

#[test]
fn test_tuple_u32_string_roundtrip() {
    let val: (u32, String) = (7u32, String::from("oxicode"));
    let enc = encode_to_vec(&val).expect("encode (u32, String)");
    let (dec, _): ((u32, String), usize) = decode_from_slice(&enc).expect("decode (u32, String)");
    assert_eq!(val, dec);
}

// ── Test 4: (String, u32) reversed order roundtrip ───────────────────────────

#[test]
fn test_tuple_string_u32_roundtrip() {
    let val: (String, u32) = (String::from("hello"), 99u32);
    let enc = encode_to_vec(&val).expect("encode (String, u32)");
    let (dec, _): ((String, u32), usize) = decode_from_slice(&enc).expect("decode (String, u32)");
    assert_eq!(val, dec);
}

// ── Test 5: (bool, u8, u16, u32, u64) five-element mixed-integer roundtrip ────

#[test]
fn test_tuple_5_mixed_integers_roundtrip() {
    let val: (bool, u8, u16, u32, u64) = (true, 0xFFu8, 0x1234u16, 0xDEAD_BEEFu32, u64::MAX);
    let enc = encode_to_vec(&val).expect("encode (bool, u8, u16, u32, u64)");
    let (dec, _): ((bool, u8, u16, u32, u64), usize) =
        decode_from_slice(&enc).expect("decode (bool, u8, u16, u32, u64)");
    assert_eq!(val, dec);
}

// ── Test 6: (String, Vec<u8>) roundtrip ───────────────────────────────────────

#[test]
fn test_tuple_string_vec_u8_roundtrip() {
    let val: (String, Vec<u8>) = (String::from("bytes"), vec![0xDE, 0xAD, 0xBE, 0xEF]);
    let enc = encode_to_vec(&val).expect("encode (String, Vec<u8>)");
    let (dec, _): ((String, Vec<u8>), usize) =
        decode_from_slice(&enc).expect("decode (String, Vec<u8>)");
    assert_eq!(val, dec);
}

// ── Test 7: (u32, u32, u32) where all three fields carry the same value ───────

#[test]
fn test_tuple_3_same_value_encoding() {
    let val: (u32, u32, u32) = (77u32, 77u32, 77u32);
    let enc = encode_to_vec(&val).expect("encode (u32, u32, u32) same value");
    let single = encode_to_vec(&77u32).expect("encode single u32");
    // each field encoded independently — concatenation of three identical encodings
    let expected: Vec<u8> = [single.clone(), single.clone(), single].concat();
    assert_eq!(
        enc, expected,
        "three identical u32 values must produce triple-encoded bytes"
    );
    let (dec, _): ((u32, u32, u32), usize) =
        decode_from_slice(&enc).expect("decode (u32, u32, u32) same value");
    assert_eq!(val, dec);
}

// ── Test 8: tuple of 8 different types roundtrip ─────────────────────────────

#[test]
fn test_tuple_8_different_types_roundtrip() {
    let val: (u8, u16, u32, u64, i8, i16, i32, i64) =
        (1u8, 2u16, 3u32, 4u64, -1i8, -2i16, -3i32, -4i64);
    let enc = encode_to_vec(&val).expect("encode 8-type tuple");
    let (dec, _): ((u8, u16, u32, u64, i8, i16, i32, i64), usize) =
        decode_from_slice(&enc).expect("decode 8-type tuple");
    assert_eq!(val, dec);
}

// ── Test 9: (Option<u32>, Option<String>) both Some and None variants ─────────

#[test]
fn test_tuple_option_u32_option_string_roundtrip() {
    let val_some: (Option<u32>, Option<String>) = (Some(42u32), Some(String::from("present")));
    let enc_some = encode_to_vec(&val_some).expect("encode (Some(u32), Some(String))");
    let (dec_some, _): ((Option<u32>, Option<String>), usize) =
        decode_from_slice(&enc_some).expect("decode (Some(u32), Some(String))");
    assert_eq!(val_some, dec_some);

    let val_none: (Option<u32>, Option<String>) = (None, None);
    let enc_none = encode_to_vec(&val_none).expect("encode (None, None)");
    let (dec_none, _): ((Option<u32>, Option<String>), usize) =
        decode_from_slice(&enc_none).expect("decode (None, None)");
    assert_eq!(val_none, dec_none);
}

// ── Test 10: consumed bytes == encoded length ─────────────────────────────────

#[test]
fn test_tuple_consumed_bytes_equals_encoded_length() {
    let val: (u32, String) = (55u32, String::from("consumed"));
    let enc = encode_to_vec(&val).expect("encode for consumed check");
    let (_, consumed): ((u32, String), usize) =
        decode_from_slice(&enc).expect("decode for consumed check");
    assert_eq!(
        consumed,
        enc.len(),
        "consumed bytes must equal encoded length"
    );
}

// ── Test 11: fixed_int_encoding config roundtrip ──────────────────────────────

#[test]
fn test_tuple_fixed_int_encoding_config_roundtrip() {
    let cfg = config::standard().with_fixed_int_encoding();
    let val: (u32, u32) = (0u32, u32::MAX);
    let enc = encode_to_vec_with_config(&val, cfg).expect("encode (u32, u32) fixed-int");
    assert_eq!(
        enc.len(),
        8,
        "fixed-int (u32, u32) must encode to 8 bytes, got {}",
        enc.len()
    );
    let (dec, _): ((u32, u32), usize) =
        decode_from_slice_with_config(&enc, cfg).expect("decode (u32, u32) fixed-int");
    assert_eq!(val, dec);
}

// ── Test 12: big_endian config roundtrip ─────────────────────────────────────

#[test]
fn test_tuple_big_endian_config_roundtrip() {
    let cfg = config::standard().with_big_endian();
    let val: (u32, u32) = (0x0102_0304u32, 0xDEAD_BEEFu32);
    let enc = encode_to_vec_with_config(&val, cfg).expect("encode (u32, u32) big-endian");
    let (dec, _): ((u32, u32), usize) =
        decode_from_slice_with_config(&enc, cfg).expect("decode (u32, u32) big-endian");
    assert_eq!(val, dec);
}

// ── Test 13: nested tuple ((u32, u32), (String, String)) roundtrip ────────────

#[test]
fn test_nested_tuple_u32_u32_string_string_roundtrip() {
    let val: ((u32, u32), (String, String)) =
        ((1u32, 2u32), (String::from("alpha"), String::from("beta")));
    let enc = encode_to_vec(&val).expect("encode ((u32, u32), (String, String))");
    let (dec, _): (((u32, u32), (String, String)), usize) =
        decode_from_slice(&enc).expect("decode ((u32, u32), (String, String))");
    assert_eq!(val, dec);
}

// ── Test 14: Vec<(u32, String)> roundtrip ────────────────────────────────────

#[test]
fn test_vec_of_tuples_u32_string_roundtrip() {
    let val: Vec<(u32, String)> = vec![
        (1u32, String::from("one")),
        (2u32, String::from("two")),
        (3u32, String::from("three")),
    ];
    let enc = encode_to_vec(&val).expect("encode Vec<(u32, String)>");
    let (dec, _): (Vec<(u32, String)>, usize) =
        decode_from_slice(&enc).expect("decode Vec<(u32, String)>");
    assert_eq!(val, dec);
}

// ── Test 15: (Vec<u8>, Vec<u8>) roundtrip ────────────────────────────────────

#[test]
fn test_tuple_two_vecs_roundtrip() {
    let val: (Vec<u8>, Vec<u8>) = (vec![0xCAu8, 0xFEu8], vec![0xBAu8, 0xBEu8]);
    let enc = encode_to_vec(&val).expect("encode (Vec<u8>, Vec<u8>)");
    let (dec, _): ((Vec<u8>, Vec<u8>), usize) =
        decode_from_slice(&enc).expect("decode (Vec<u8>, Vec<u8>)");
    assert_eq!(val, dec);
}

// ── Test 16: tuple struct Pair(u32, u32) roundtrip ───────────────────────────

#[test]
fn test_tuple_struct_pair_roundtrip() {
    let val = Pair(100u32, 200u32);
    let enc = encode_to_vec(&val).expect("encode Pair");
    let (dec, _): (Pair, usize) = decode_from_slice(&enc).expect("decode Pair");
    assert_eq!(val, dec);
}

// ── Test 17: tuple struct Triple(u8, u16, u32) roundtrip ─────────────────────

#[test]
fn test_tuple_struct_triple_roundtrip() {
    let val = Triple(0xFFu8, 0x1234u16, 0xDEAD_BEEFu32);
    let enc = encode_to_vec(&val).expect("encode Triple");
    let (dec, _): (Triple, usize) = decode_from_slice(&enc).expect("decode Triple");
    assert_eq!(val, dec);
}

// ── Test 18: tuple struct same wire bytes as equivalent plain tuple ───────────

#[test]
fn test_tuple_struct_pair_same_bytes_as_plain_tuple() {
    let pair = Pair(42u32, 99u32);
    let plain: (u32, u32) = (42u32, 99u32);
    let enc_pair = encode_to_vec(&pair).expect("encode Pair struct");
    let enc_plain = encode_to_vec(&plain).expect("encode (u32, u32) plain");
    assert_eq!(
        enc_pair, enc_plain,
        "Pair struct must encode identically to equivalent plain tuple"
    );
}

// ── Test 19: (u32, u32) where first != second encode to different bytes ────────

#[test]
fn test_tuple_u32_u32_different_values_different_encoding() {
    let val_a: (u32, u32) = (1u32, 2u32);
    let val_b: (u32, u32) = (2u32, 1u32);
    let enc_a = encode_to_vec(&val_a).expect("encode (1, 2)");
    let enc_b = encode_to_vec(&val_b).expect("encode (2, 1)");
    assert_ne!(
        enc_a, enc_b,
        "(1, 2) and (2, 1) must produce different encodings"
    );
}

// ── Test 20: empty tuple () roundtrip — encodes to 0 bytes ───────────────────

#[test]
fn test_empty_tuple_encodes_to_zero_bytes() {
    let val: () = ();
    let enc = encode_to_vec(&val).expect("encode ()");
    assert_eq!(
        enc.len(),
        0,
        "empty tuple must encode to 0 bytes, got {}",
        enc.len()
    );
    let (dec, consumed): ((), usize) = decode_from_slice(&enc).expect("decode ()");
    assert_eq!(val, dec);
    assert_eq!(consumed, 0, "empty tuple decode must consume 0 bytes");
}

// ── Test 21: tuple of 12 elements roundtrip ───────────────────────────────────

#[test]
fn test_tuple_12_elements_roundtrip() {
    let val: (
        u8,
        u16,
        u32,
        u64,
        i8,
        i16,
        i32,
        i64,
        bool,
        bool,
        String,
        Vec<u8>,
    ) = (
        1u8,
        2u16,
        3u32,
        4u64,
        -1i8,
        -2i16,
        -3i32,
        -4i64,
        true,
        false,
        String::from("twelve"),
        vec![0xAAu8, 0xBBu8, 0xCCu8],
    );
    let enc = encode_to_vec(&val).expect("encode 12-element tuple");
    let (dec, consumed): (
        (
            u8,
            u16,
            u32,
            u64,
            i8,
            i16,
            i32,
            i64,
            bool,
            bool,
            String,
            Vec<u8>,
        ),
        usize,
    ) = decode_from_slice(&enc).expect("decode 12-element tuple");
    assert_eq!(val, dec);
    assert_eq!(
        consumed,
        enc.len(),
        "consumed must equal encoded length for 12-element tuple"
    );
}

// ── Test 22: (i32, i32) with negative values roundtrip ───────────────────────

#[test]
fn test_tuple_i32_i32_negative_values_roundtrip() {
    let val: (i32, i32) = (i32::MIN, -1i32);
    let enc = encode_to_vec(&val).expect("encode (i32::MIN, -1)");
    let (dec, _): ((i32, i32), usize) = decode_from_slice(&enc).expect("decode (i32::MIN, -1)");
    assert_eq!(val, dec);
}
