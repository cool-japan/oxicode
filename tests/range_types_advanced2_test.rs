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
use std::ops::{Range, RangeInclusive};

#[test]
fn test_range_u32_roundtrip_basic() {
    let val: Range<u32> = 10..20;
    let encoded = encode_to_vec(&val).expect("encode Range<u32> basic");
    let (decoded, _): (Range<u32>, usize) =
        decode_from_slice(&encoded).expect("decode Range<u32> basic");
    assert_eq!(decoded, 10..20);
}

#[test]
fn test_range_i32_roundtrip_negative_range() {
    let val: Range<i32> = -100..50;
    let encoded = encode_to_vec(&val).expect("encode Range<i32> negative range");
    let (decoded, _): (Range<i32>, usize) =
        decode_from_slice(&encoded).expect("decode Range<i32> negative range");
    assert_eq!(decoded, -100..50);
}

#[test]
fn test_range_u64_roundtrip_large_values() {
    let val: Range<u64> = 1_000_000_000_000..9_999_999_999_999;
    let encoded = encode_to_vec(&val).expect("encode Range<u64> large values");
    let (decoded, _): (Range<u64>, usize) =
        decode_from_slice(&encoded).expect("decode Range<u64> large values");
    assert_eq!(decoded, 1_000_000_000_000..9_999_999_999_999);
}

#[test]
fn test_range_inclusive_u32_roundtrip_basic() {
    let val: RangeInclusive<u32> = 5..=15;
    let encoded = encode_to_vec(&val).expect("encode RangeInclusive<u32> basic");
    let (decoded, _): (RangeInclusive<u32>, usize) =
        decode_from_slice(&encoded).expect("decode RangeInclusive<u32> basic");
    assert_eq!(decoded, 5..=15);
}

#[test]
fn test_range_inclusive_i64_roundtrip() {
    let val: RangeInclusive<i64> = -500..=500;
    let encoded = encode_to_vec(&val).expect("encode RangeInclusive<i64>");
    let (decoded, _): (RangeInclusive<i64>, usize) =
        decode_from_slice(&encoded).expect("decode RangeInclusive<i64>");
    assert_eq!(decoded, -500..=500);
}

#[test]
fn test_range_u8_empty_roundtrip() {
    let val: Range<u8> = 0..0;
    let encoded = encode_to_vec(&val).expect("encode Range<u8> empty 0..0");
    let (decoded, _): (Range<u8>, usize) =
        decode_from_slice(&encoded).expect("decode Range<u8> empty 0..0");
    assert_eq!(decoded, 0..0);
    assert!(decoded.is_empty());
}

#[test]
fn test_range_inclusive_u8_full_roundtrip() {
    let val: RangeInclusive<u8> = 0..=255;
    let encoded = encode_to_vec(&val).expect("encode RangeInclusive<u8> full 0..=255");
    let (decoded, _): (RangeInclusive<u8>, usize) =
        decode_from_slice(&encoded).expect("decode RangeInclusive<u8> full 0..=255");
    assert_eq!(decoded, 0..=255);
}

#[test]
fn test_range_u32_consumed_bytes_equals_encoded_len() {
    let val: Range<u32> = 1..100;
    let encoded = encode_to_vec(&val).expect("encode Range<u32> for consumed bytes check");
    let (_, consumed): (Range<u32>, usize) =
        decode_from_slice(&encoded).expect("decode Range<u32> for consumed bytes check");
    assert_eq!(consumed, encoded.len());
}

#[test]
fn test_range_inclusive_u32_consumed_bytes_equals_encoded_len() {
    let val: RangeInclusive<u32> = 1..=100;
    let encoded = encode_to_vec(&val).expect("encode RangeInclusive<u32> for consumed bytes check");
    let (_, consumed): (RangeInclusive<u32>, usize) =
        decode_from_slice(&encoded).expect("decode RangeInclusive<u32> for consumed bytes check");
    assert_eq!(consumed, encoded.len());
}

#[test]
fn test_vec_range_u32_multiple_roundtrip() {
    let val: Vec<Range<u32>> = vec![0..10, 20..30, 40..50];
    let encoded = encode_to_vec(&val).expect("encode Vec<Range<u32>>");
    let (decoded, _): (Vec<Range<u32>>, usize) =
        decode_from_slice(&encoded).expect("decode Vec<Range<u32>>");
    assert_eq!(decoded, vec![0..10, 20..30, 40..50]);
}

#[test]
fn test_option_range_u32_some_roundtrip() {
    let val: Option<Range<u32>> = Some(5..25);
    let encoded = encode_to_vec(&val).expect("encode Option<Range<u32>> Some");
    let (decoded, _): (Option<Range<u32>>, usize) =
        decode_from_slice(&encoded).expect("decode Option<Range<u32>> Some");
    assert_eq!(decoded, Some(5..25));
}

#[test]
fn test_option_range_u32_none_roundtrip() {
    let val: Option<Range<u32>> = None;
    let encoded = encode_to_vec(&val).expect("encode Option<Range<u32>> None");
    let (decoded, _): (Option<Range<u32>>, usize) =
        decode_from_slice(&encoded).expect("decode Option<Range<u32>> None");
    assert_eq!(decoded, None);
}

#[test]
fn test_range_u32_with_fixed_int_config_roundtrip() {
    let val: Range<u32> = 3..42;
    let cfg = config::standard().with_fixed_int_encoding();
    let encoded = encode_to_vec_with_config(&val, cfg).expect("encode Range<u32> fixed int config");
    let (decoded, _): (Range<u32>, usize) =
        decode_from_slice_with_config(&encoded, cfg).expect("decode Range<u32> fixed int config");
    assert_eq!(decoded, 3..42);
}

#[test]
fn test_range_inclusive_u32_with_fixed_int_config_roundtrip() {
    let val: RangeInclusive<u32> = 7..=77;
    let cfg = config::standard().with_fixed_int_encoding();
    let encoded =
        encode_to_vec_with_config(&val, cfg).expect("encode RangeInclusive<u32> fixed int config");
    let (decoded, _): (RangeInclusive<u32>, usize) = decode_from_slice_with_config(&encoded, cfg)
        .expect("decode RangeInclusive<u32> fixed int config");
    assert_eq!(decoded, 7..=77);
}

#[test]
fn test_different_range_u32_values_encode_differently() {
    let val_a: Range<u32> = 0..10;
    let val_b: Range<u32> = 100..200;
    let encoded_a = encode_to_vec(&val_a).expect("encode Range<u32> value a");
    let encoded_b = encode_to_vec(&val_b).expect("encode Range<u32> value b");
    assert_ne!(encoded_a, encoded_b);
}

#[test]
fn test_range_f32_roundtrip_bit_exact() {
    let start: f32 = 1.5_f32;
    let end: f32 = 3.14_f32;
    let val: Range<f32> = start..end;
    let encoded = encode_to_vec(&val).expect("encode Range<f32>");
    let (decoded, _): (Range<f32>, usize) = decode_from_slice(&encoded).expect("decode Range<f32>");
    assert_eq!(decoded.start.to_bits(), start.to_bits());
    assert_eq!(decoded.end.to_bits(), end.to_bits());
}

#[test]
fn test_range_i32_with_negative_start() {
    let val: Range<i32> = -999..0;
    let encoded = encode_to_vec(&val).expect("encode Range<i32> with negative start");
    let (decoded, _): (Range<i32>, usize) =
        decode_from_slice(&encoded).expect("decode Range<i32> with negative start");
    assert_eq!(decoded, -999..0);
}

#[test]
fn test_vec_of_range_inclusive_roundtrip() {
    let val: Vec<RangeInclusive<u32>> = vec![0..=9, 100..=199, 1000..=1999];
    let encoded = encode_to_vec(&val).expect("encode Vec<RangeInclusive<u32>>");
    let (decoded, _): (Vec<RangeInclusive<u32>>, usize) =
        decode_from_slice(&encoded).expect("decode Vec<RangeInclusive<u32>>");
    assert_eq!(decoded, vec![0..=9, 100..=199, 1000..=1999]);
}

#[test]
fn test_range_u32_fixed_int_encoding_produces_correct_size() {
    let val: Range<u32> = 0..1;
    let cfg = config::standard().with_fixed_int_encoding();
    let encoded =
        encode_to_vec_with_config(&val, cfg).expect("encode Range<u32> fixed int size check");
    // A Range<u32> has two u32 fields; with fixed int encoding each u32 takes 4 bytes => 8 bytes total
    assert_eq!(
        encoded.len(),
        8,
        "Expected 8 bytes for Range<u32> with fixed int encoding, got {}",
        encoded.len()
    );
}

#[test]
fn test_option_vec_range_u32_some_roundtrip() {
    let val: Option<Vec<Range<u32>>> = Some(vec![1..10, 20..30]);
    let encoded = encode_to_vec(&val).expect("encode Option<Vec<Range<u32>>> Some");
    let (decoded, _): (Option<Vec<Range<u32>>>, usize) =
        decode_from_slice(&encoded).expect("decode Option<Vec<Range<u32>>> Some");
    assert_eq!(decoded, Some(vec![1..10, 20..30]));
}

#[test]
fn test_range_u16_roundtrip() {
    let val: Range<u16> = 100..200;
    let encoded = encode_to_vec(&val).expect("encode Range<u16>");
    let (decoded, _): (Range<u16>, usize) = decode_from_slice(&encoded).expect("decode Range<u16>");
    assert_eq!(decoded, 100..200);
}

#[test]
fn test_range_inclusive_u16_roundtrip() {
    let val: RangeInclusive<u16> = 300..=400;
    let encoded = encode_to_vec(&val).expect("encode RangeInclusive<u16>");
    let (decoded, _): (RangeInclusive<u16>, usize) =
        decode_from_slice(&encoded).expect("decode RangeInclusive<u16>");
    assert_eq!(decoded, 300..=400);
}
