//! Advanced tests for Range, RangeInclusive, and Bound types in OxiCode.

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
use std::ops::{Bound, Range, RangeInclusive};

// ===== Test 1: Range<u32> basic roundtrip =====

#[test]
fn test_range_u32_basic_roundtrip() {
    let original: Range<u32> = 0u32..10;
    let encoded = encode_to_vec(&original).expect("encode Range<u32> 0..10");
    let (decoded, _): (Range<u32>, _) =
        decode_from_slice(&encoded).expect("decode Range<u32> 0..10");
    assert_eq!(decoded.start, 0);
    assert_eq!(decoded.end, 10);
}

// ===== Test 2: Range<u32> offset roundtrip =====

#[test]
fn test_range_u32_offset_roundtrip() {
    let original: Range<u32> = 100u32..200;
    let encoded = encode_to_vec(&original).expect("encode Range<u32> 100..200");
    let (decoded, _): (Range<u32>, _) =
        decode_from_slice(&encoded).expect("decode Range<u32> 100..200");
    assert_eq!(decoded.start, 100);
    assert_eq!(decoded.end, 200);
}

// ===== Test 3: Range<i64> negative roundtrip =====

#[test]
fn test_range_i64_negative_roundtrip() {
    let original: Range<i64> = (-100i64)..100i64;
    let encoded = encode_to_vec(&original).expect("encode Range<i64> -100..100");
    let (decoded, _): (Range<i64>, _) =
        decode_from_slice(&encoded).expect("decode Range<i64> -100..100");
    assert_eq!(decoded.start, -100);
    assert_eq!(decoded.end, 100);
}

// ===== Test 4: Range<u64> max roundtrip =====

#[test]
fn test_range_u64_max_roundtrip() {
    let original: Range<u64> = 0u64..u64::MAX;
    let encoded = encode_to_vec(&original).expect("encode Range<u64> 0..MAX");
    let (decoded, _): (Range<u64>, _) =
        decode_from_slice(&encoded).expect("decode Range<u64> 0..MAX");
    assert_eq!(decoded.start, 0);
    assert_eq!(decoded.end, u64::MAX);
}

// ===== Test 5: Range<u32> empty range roundtrip =====

#[test]
fn test_range_u32_empty_roundtrip() {
    let original: Range<u32> = 5u32..5;
    let encoded = encode_to_vec(&original).expect("encode Range<u32> 5..5");
    let (decoded, _): (Range<u32>, _) =
        decode_from_slice(&encoded).expect("decode Range<u32> 5..5");
    assert_eq!(decoded.start, 5);
    assert_eq!(decoded.end, 5);
}

// ===== Test 6: Vec<Range<u32>> roundtrip =====

#[test]
fn test_vec_of_ranges_roundtrip() {
    let original: Vec<Range<u32>> = vec![0u32..10, 20u32..30, 40u32..50];
    let encoded = encode_to_vec(&original).expect("encode Vec<Range<u32>>");
    let (decoded, _): (Vec<Range<u32>>, _) =
        decode_from_slice(&encoded).expect("decode Vec<Range<u32>>");
    assert_eq!(decoded.len(), 3);
    assert_eq!(decoded[0].start, 0);
    assert_eq!(decoded[0].end, 10);
    assert_eq!(decoded[1].start, 20);
    assert_eq!(decoded[1].end, 30);
    assert_eq!(decoded[2].start, 40);
    assert_eq!(decoded[2].end, 50);
}

// ===== Test 7: Option<Range<u32>> Some and None roundtrip =====

#[test]
fn test_option_range_some_none_roundtrip() {
    // Some case
    let some_val: Option<Range<u32>> = Some(0u32..10);
    let encoded_some = encode_to_vec(&some_val).expect("encode Option<Range<u32>> Some");
    let (decoded_some, _): (Option<Range<u32>>, _) =
        decode_from_slice(&encoded_some).expect("decode Option<Range<u32>> Some");
    assert!(decoded_some.is_some());
    let inner = decoded_some.expect("should be Some");
    assert_eq!(inner.start, 0);
    assert_eq!(inner.end, 10);

    // None case
    let none_val: Option<Range<u32>> = None;
    let encoded_none = encode_to_vec(&none_val).expect("encode Option<Range<u32>> None");
    let (decoded_none, _): (Option<Range<u32>>, _) =
        decode_from_slice(&encoded_none).expect("decode Option<Range<u32>> None");
    assert!(decoded_none.is_none());
}

// ===== Test 8: Range with fixed int encoding =====

#[test]
fn test_range_fixed_int_encoding() {
    let cfg = config::standard().with_fixed_int_encoding();
    let original: Range<u32> = 0u32..10;
    let encoded =
        encode_to_vec_with_config(&original, cfg).expect("encode Range<u32> with fixed_int");
    let (decoded, _): (Range<u32>, _) =
        decode_from_slice_with_config(&encoded, cfg).expect("decode Range<u32> with fixed_int");
    assert_eq!(decoded.start, original.start);
    assert_eq!(decoded.end, original.end);
}

// ===== Test 9: Range with big endian config =====

#[test]
fn test_range_big_endian_config() {
    let cfg = config::standard().with_big_endian();
    let original: Range<u32> = 1u32..5;
    let encoded =
        encode_to_vec_with_config(&original, cfg).expect("encode Range<u32> with big_endian");
    let (decoded, _): (Range<u32>, _) =
        decode_from_slice_with_config(&encoded, cfg).expect("decode Range<u32> with big_endian");
    assert_eq!(decoded.start, original.start);
    assert_eq!(decoded.end, original.end);
}

// ===== Test 10: RangeInclusive<u32> basic roundtrip =====

#[test]
fn test_range_inclusive_basic_roundtrip() {
    let original: RangeInclusive<u32> = 0u32..=10;
    let encoded = encode_to_vec(&original).expect("encode RangeInclusive<u32> 0..=10");
    let (decoded, _): (RangeInclusive<u32>, _) =
        decode_from_slice(&encoded).expect("decode RangeInclusive<u32> 0..=10");
    assert_eq!(*decoded.start(), 0);
    assert_eq!(*decoded.end(), 10);
}

// ===== Test 11: RangeInclusive<u32> single element =====

#[test]
fn test_range_inclusive_single_element() {
    let original: RangeInclusive<u32> = 1u32..=1;
    let encoded = encode_to_vec(&original).expect("encode RangeInclusive<u32> 1..=1");
    let (decoded, _): (RangeInclusive<u32>, _) =
        decode_from_slice(&encoded).expect("decode RangeInclusive<u32> 1..=1");
    assert_eq!(*decoded.start(), 1);
    assert_eq!(*decoded.end(), 1);
}

// ===== Test 12: RangeInclusive<i32> negative =====

#[test]
fn test_range_inclusive_i32_negative() {
    let original: RangeInclusive<i32> = (-50i32)..=50i32;
    let encoded = encode_to_vec(&original).expect("encode RangeInclusive<i32> -50..=50");
    let (decoded, _): (RangeInclusive<i32>, _) =
        decode_from_slice(&encoded).expect("decode RangeInclusive<i32> -50..=50");
    assert_eq!(*decoded.start(), -50);
    assert_eq!(*decoded.end(), 50);
}

// ===== Test 13: Vec<RangeInclusive<u32>> roundtrip =====

#[test]
fn test_vec_of_range_inclusive_roundtrip() {
    let original: Vec<RangeInclusive<u32>> = vec![0u32..=5, 10u32..=20];
    let encoded = encode_to_vec(&original).expect("encode Vec<RangeInclusive<u32>>");
    let (decoded, _): (Vec<RangeInclusive<u32>>, _) =
        decode_from_slice(&encoded).expect("decode Vec<RangeInclusive<u32>>");
    assert_eq!(decoded.len(), 2);
    assert_eq!(*decoded[0].start(), 0);
    assert_eq!(*decoded[0].end(), 5);
    assert_eq!(*decoded[1].start(), 10);
    assert_eq!(*decoded[1].end(), 20);
}

// ===== Test 14: Option<RangeInclusive<u32>> Some and None roundtrip =====

#[test]
fn test_option_range_inclusive_some_none() {
    // Some case
    let some_val: Option<RangeInclusive<u32>> = Some(0u32..=10);
    let encoded_some = encode_to_vec(&some_val).expect("encode Option<RangeInclusive<u32>> Some");
    let (decoded_some, _): (Option<RangeInclusive<u32>>, _) =
        decode_from_slice(&encoded_some).expect("decode Option<RangeInclusive<u32>> Some");
    assert!(decoded_some.is_some());
    let inner = decoded_some.expect("should be Some");
    assert_eq!(*inner.start(), 0);
    assert_eq!(*inner.end(), 10);

    // None case
    let none_val: Option<RangeInclusive<u32>> = None;
    let encoded_none = encode_to_vec(&none_val).expect("encode Option<RangeInclusive<u32>> None");
    let (decoded_none, _): (Option<RangeInclusive<u32>>, _) =
        decode_from_slice(&encoded_none).expect("decode Option<RangeInclusive<u32>> None");
    assert!(decoded_none.is_none());
}

// ===== Test 15: RangeInclusive with fixed int encoding =====

#[test]
fn test_range_inclusive_fixed_int() {
    let cfg = config::standard().with_fixed_int_encoding();
    let original: RangeInclusive<u32> = 0u32..=100;
    let encoded = encode_to_vec_with_config(&original, cfg)
        .expect("encode RangeInclusive<u32> with fixed_int");
    let (decoded, _): (RangeInclusive<u32>, _) = decode_from_slice_with_config(&encoded, cfg)
        .expect("decode RangeInclusive<u32> with fixed_int");
    assert_eq!(*decoded.start(), *original.start());
    assert_eq!(*decoded.end(), *original.end());
}

// ===== Test 16: Bound::Unbounded roundtrip =====

#[test]
fn test_bound_unbounded_roundtrip() {
    let original: Bound<u32> = Bound::Unbounded;
    let encoded = encode_to_vec(&original).expect("encode Bound::Unbounded");
    let (decoded, _): (Bound<u32>, _) =
        decode_from_slice(&encoded).expect("decode Bound::Unbounded");
    assert_eq!(decoded, Bound::Unbounded);
}

// ===== Test 17: Bound::Included roundtrip =====

#[test]
fn test_bound_included_roundtrip() {
    let original: Bound<u32> = Bound::Included(42);
    let encoded = encode_to_vec(&original).expect("encode Bound::Included(42)");
    let (decoded, _): (Bound<u32>, _) =
        decode_from_slice(&encoded).expect("decode Bound::Included(42)");
    assert_eq!(decoded, Bound::Included(42));
}

// ===== Test 18: Bound::Excluded roundtrip =====

#[test]
fn test_bound_excluded_roundtrip() {
    let original: Bound<u32> = Bound::Excluded(100);
    let encoded = encode_to_vec(&original).expect("encode Bound::Excluded(100)");
    let (decoded, _): (Bound<u32>, _) =
        decode_from_slice(&encoded).expect("decode Bound::Excluded(100)");
    assert_eq!(decoded, Bound::Excluded(100));
}

// ===== Test 19: Bound<String>::Included roundtrip =====

#[test]
fn test_bound_string_included_roundtrip() {
    let original: Bound<String> = Bound::Included("hello".to_string());
    let encoded = encode_to_vec(&original).expect("encode Bound<String>::Included");
    let (decoded, _): (Bound<String>, _) =
        decode_from_slice(&encoded).expect("decode Bound<String>::Included");
    assert_eq!(decoded, Bound::Included("hello".to_string()));
}

// ===== Test 20: Vec<Bound<u32>> all three variants roundtrip =====

#[test]
fn test_vec_of_bounds_all_variants() {
    let original: Vec<Bound<u32>> = vec![Bound::Unbounded, Bound::Included(5), Bound::Excluded(10)];
    let encoded = encode_to_vec(&original).expect("encode Vec<Bound<u32>>");
    let (decoded, _): (Vec<Bound<u32>>, _) =
        decode_from_slice(&encoded).expect("decode Vec<Bound<u32>>");
    assert_eq!(decoded.len(), 3);
    assert_eq!(decoded[0], Bound::Unbounded);
    assert_eq!(decoded[1], Bound::Included(5));
    assert_eq!(decoded[2], Bound::Excluded(10));
}

// ===== Test 21: Tuple of Bound<u32> roundtrip =====

#[test]
fn test_tuple_of_bounds_roundtrip() {
    let original: (Bound<u32>, Bound<u32>) = (Bound::Included(0), Bound::Excluded(100));
    let encoded = encode_to_vec(&original).expect("encode (Bound::Included, Bound::Excluded)");
    let (decoded, _): ((Bound<u32>, Bound<u32>), _) =
        decode_from_slice(&encoded).expect("decode (Bound::Included, Bound::Excluded)");
    assert_eq!(decoded.0, Bound::Included(0));
    assert_eq!(decoded.1, Bound::Excluded(100));
}

// ===== Test 22: Nested Range<Option<u32>> roundtrip =====

#[test]
fn test_range_option_inner_roundtrip() {
    let original: Range<Option<u32>> = Range {
        start: Some(1),
        end: None,
    };
    let encoded = encode_to_vec(&original).expect("encode Range<Option<u32>>");
    let (decoded, _): (Range<Option<u32>>, _) =
        decode_from_slice(&encoded).expect("decode Range<Option<u32>>");
    assert_eq!(decoded.start, Some(1));
    assert_eq!(decoded.end, None);
}
