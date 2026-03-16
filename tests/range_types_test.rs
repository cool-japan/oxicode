//! Comprehensive tests for RangeFull, RangeFrom<T>, RangeTo<T>, RangeToInclusive<T>
//! encode/decode/borrow_decode implementations.

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
use oxicode::{decode_from_slice, encode_to_vec, Decode, Encode};
use std::ops::{RangeFrom, RangeFull, RangeTo, RangeToInclusive};

// ===== Test 1: RangeFull roundtrip =====

#[test]
fn test_range_full_roundtrip() {
    let original: RangeFull = ..;
    let encoded = encode_to_vec(&original).expect("encode RangeFull");
    let (decoded, _): (RangeFull, _) = decode_from_slice(&encoded).expect("decode RangeFull");
    // RangeFull is a ZST; just verifying the round-trip succeeds is sufficient
    let _ = decoded;
}

// ===== Test 2: RangeFrom<u32> with start=0 roundtrip =====

#[test]
fn test_range_from_u32_start_zero_roundtrip() {
    let original: RangeFrom<u32> = 0..;
    let encoded = encode_to_vec(&original).expect("encode RangeFrom<u32> start=0");
    let (decoded, _): (RangeFrom<u32>, _) =
        decode_from_slice(&encoded).expect("decode RangeFrom<u32> start=0");
    assert_eq!(original.start, decoded.start);
}

// ===== Test 3: RangeFrom<u32> with start=u32::MAX roundtrip =====

#[test]
fn test_range_from_u32_start_max_roundtrip() {
    let original: RangeFrom<u32> = u32::MAX..;
    let encoded = encode_to_vec(&original).expect("encode RangeFrom<u32> start=MAX");
    let (decoded, _): (RangeFrom<u32>, _) =
        decode_from_slice(&encoded).expect("decode RangeFrom<u32> start=MAX");
    assert_eq!(original.start, decoded.start);
}

// ===== Test 4: RangeTo<u32> with end=100 roundtrip =====

#[test]
fn test_range_to_u32_end_100_roundtrip() {
    let original: RangeTo<u32> = ..100u32;
    let encoded = encode_to_vec(&original).expect("encode RangeTo<u32> end=100");
    let (decoded, _): (RangeTo<u32>, _) =
        decode_from_slice(&encoded).expect("decode RangeTo<u32> end=100");
    assert_eq!(original.end, decoded.end);
}

// ===== Test 5: RangeTo<u32> with end=0 roundtrip =====

#[test]
fn test_range_to_u32_end_zero_roundtrip() {
    let original: RangeTo<u32> = ..0u32;
    let encoded = encode_to_vec(&original).expect("encode RangeTo<u32> end=0");
    let (decoded, _): (RangeTo<u32>, _) =
        decode_from_slice(&encoded).expect("decode RangeTo<u32> end=0");
    assert_eq!(original.end, decoded.end);
}

// ===== Test 6: RangeToInclusive<u32> with end=100 roundtrip =====

#[test]
fn test_range_to_inclusive_u32_end_100_roundtrip() {
    let original: RangeToInclusive<u32> = ..=100u32;
    let encoded = encode_to_vec(&original).expect("encode RangeToInclusive<u32> end=100");
    let (decoded, _): (RangeToInclusive<u32>, _) =
        decode_from_slice(&encoded).expect("decode RangeToInclusive<u32> end=100");
    assert_eq!(original.end, decoded.end);
}

// ===== Test 7: RangeToInclusive<u32> with end=u32::MAX roundtrip =====

#[test]
fn test_range_to_inclusive_u32_end_max_roundtrip() {
    let original: RangeToInclusive<u32> = ..=u32::MAX;
    let encoded = encode_to_vec(&original).expect("encode RangeToInclusive<u32> end=MAX");
    let (decoded, _): (RangeToInclusive<u32>, _) =
        decode_from_slice(&encoded).expect("decode RangeToInclusive<u32> end=MAX");
    assert_eq!(original.end, decoded.end);
}

// ===== Test 8: RangeFrom<i64> with negative start roundtrip =====

#[test]
fn test_range_from_i64_negative_start_roundtrip() {
    let original: RangeFrom<i64> = -9_223_372_036_854_775_807i64..;
    let encoded = encode_to_vec(&original).expect("encode RangeFrom<i64> negative start");
    let (decoded, _): (RangeFrom<i64>, _) =
        decode_from_slice(&encoded).expect("decode RangeFrom<i64> negative start");
    assert_eq!(original.start, decoded.start);
}

// ===== Test 9: RangeTo<i64> with negative end roundtrip =====

#[test]
fn test_range_to_i64_negative_end_roundtrip() {
    let original: RangeTo<i64> = ..-42i64;
    let encoded = encode_to_vec(&original).expect("encode RangeTo<i64> negative end");
    let (decoded, _): (RangeTo<i64>, _) =
        decode_from_slice(&encoded).expect("decode RangeTo<i64> negative end");
    assert_eq!(original.end, decoded.end);
}

// ===== Test 10: RangeToInclusive<String> roundtrip =====

#[test]
fn test_range_to_inclusive_string_roundtrip() {
    let original: RangeToInclusive<String> = ..="hello, world".to_string();
    let encoded = encode_to_vec(&original).expect("encode RangeToInclusive<String>");
    let (decoded, _): (RangeToInclusive<String>, _) =
        decode_from_slice(&encoded).expect("decode RangeToInclusive<String>");
    assert_eq!(original.end, decoded.end);
}

// ===== Test 11: RangeFull encodes to 0 bytes =====

#[test]
fn test_range_full_encodes_to_zero_bytes() {
    let original: RangeFull = ..;
    let encoded = encode_to_vec(&original).expect("encode RangeFull for size check");
    assert_eq!(
        encoded.len(),
        0,
        "RangeFull (a ZST/unit type) must encode to exactly 0 bytes, got {}",
        encoded.len()
    );
}

// ===== Test 12: RangeFrom<u8> encodes to exactly 1 byte =====

#[test]
fn test_range_from_u8_encodes_to_one_byte() {
    // u8 always encodes as a single raw byte regardless of config
    let original: RangeFrom<u8> = 0xAB..;
    let encoded = encode_to_vec(&original).expect("encode RangeFrom<u8> for size check");
    assert_eq!(
        encoded.len(),
        1,
        "RangeFrom<u8> must encode to exactly 1 byte (the u8 start value), got {}",
        encoded.len()
    );
    assert_eq!(encoded[0], 0xAB, "encoded byte must equal the start value");
}

// ===== Test 13: Vec<RangeFrom<u32>> roundtrip =====

#[test]
fn test_vec_of_range_from_u32_roundtrip() {
    let original: Vec<RangeFrom<u32>> = vec![0.., 1.., 100.., u32::MAX..];
    let encoded = encode_to_vec(&original).expect("encode Vec<RangeFrom<u32>>");
    let (decoded, _): (Vec<RangeFrom<u32>>, _) =
        decode_from_slice(&encoded).expect("decode Vec<RangeFrom<u32>>");
    assert_eq!(original.len(), decoded.len());
    for (orig, dec) in original.iter().zip(decoded.iter()) {
        assert_eq!(orig.start, dec.start);
    }
}

// ===== Test 14: Option<RangeTo<u32>> roundtrip =====

#[test]
fn test_option_range_to_u32_roundtrip() {
    // Some case
    let some_val: Option<RangeTo<u32>> = Some(..42u32);
    let encoded = encode_to_vec(&some_val).expect("encode Option<RangeTo<u32>> Some");
    let (decoded, _): (Option<RangeTo<u32>>, _) =
        decode_from_slice(&encoded).expect("decode Option<RangeTo<u32>> Some");
    assert!(decoded.is_some());
    assert_eq!(some_val.as_ref().map(|r| r.end), decoded.map(|r| r.end));

    // None case
    let none_val: Option<RangeTo<u32>> = None;
    let encoded_none = encode_to_vec(&none_val).expect("encode Option<RangeTo<u32>> None");
    let (decoded_none, _): (Option<RangeTo<u32>>, _) =
        decode_from_slice(&encoded_none).expect("decode Option<RangeTo<u32>> None");
    assert!(decoded_none.is_none());
}

// ===== Test 15: Struct with derive containing all four range types =====

#[derive(Debug, PartialEq, Encode, Decode)]
struct AllRangeTypes {
    full: RangeFull,
    from: RangeFrom<u32>,
    to: RangeTo<u32>,
    to_inclusive: RangeToInclusive<u32>,
}

#[test]
fn test_struct_with_all_range_types_roundtrip() {
    let original = AllRangeTypes {
        full: ..,
        from: 10..,
        to: ..20u32,
        to_inclusive: ..=30u32,
    };
    let encoded = encode_to_vec(&original).expect("encode AllRangeTypes struct");
    let (decoded, _): (AllRangeTypes, _) =
        decode_from_slice(&encoded).expect("decode AllRangeTypes struct");

    // RangeFull: ZST, just confirm decode succeeded (value is always ..)
    let _ = decoded.full;
    assert_eq!(original.from.start, decoded.from.start);
    assert_eq!(original.to.end, decoded.to.end);
    assert_eq!(original.to_inclusive.end, decoded.to_inclusive.end);
}
