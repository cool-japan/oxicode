//! Advanced tests for Range, RangeInclusive, and Bound serialization – second set.
//! Covers new angles: large/boundary values, configs, struct composition, and size invariants.

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
use std::ops::{Bound, Range, RangeInclusive};

// ===== 1. Range<u32> simple roundtrip (0..10) =====

#[test]
fn test_range_u32_zero_to_ten_roundtrip() {
    let r: Range<u32> = 0..10;
    let encoded = encode_to_vec(&r).expect("encode Range<u32> 0..10");
    let (decoded, _): (Range<u32>, usize) =
        decode_from_slice(&encoded).expect("decode Range<u32> 0..10");
    assert_eq!(r, decoded);
}

// ===== 2. Range<u32> empty range (5..5) =====

#[test]
fn test_range_u32_empty_range_roundtrip() {
    let r: Range<u32> = 5..5;
    assert!(r.is_empty());
    let encoded = encode_to_vec(&r).expect("encode Range<u32> empty");
    let (decoded, _): (Range<u32>, usize) =
        decode_from_slice(&encoded).expect("decode Range<u32> empty");
    assert_eq!(r, decoded);
    assert!(decoded.is_empty());
}

// ===== 3. Range<u32> large range (0..u32::MAX) =====

#[test]
fn test_range_u32_zero_to_max_roundtrip() {
    let r: Range<u32> = 0..u32::MAX;
    let encoded = encode_to_vec(&r).expect("encode Range<u32> 0..MAX");
    let (decoded, _): (Range<u32>, usize) =
        decode_from_slice(&encoded).expect("decode Range<u32> 0..MAX");
    assert_eq!(decoded.start, 0u32);
    assert_eq!(decoded.end, u32::MAX);
    assert_eq!(r, decoded);
}

// ===== 4. Range<i32> negative values (-10..10) =====

#[test]
fn test_range_i32_negative_values_roundtrip() {
    let r: Range<i32> = -10..10;
    let encoded = encode_to_vec(&r).expect("encode Range<i32> -10..10");
    let (decoded, _): (Range<i32>, usize) =
        decode_from_slice(&encoded).expect("decode Range<i32> -10..10");
    assert_eq!(r, decoded);
    assert_eq!(decoded.start, -10i32);
    assert_eq!(decoded.end, 10i32);
}

// ===== 5. Range<u64> roundtrip =====

#[test]
fn test_range_u64_roundtrip() {
    let r: Range<u64> = 1_000_000u64..9_000_000_000u64;
    let encoded = encode_to_vec(&r).expect("encode Range<u64>");
    let (decoded, _): (Range<u64>, usize) = decode_from_slice(&encoded).expect("decode Range<u64>");
    assert_eq!(r, decoded);
}

// ===== 6. Range<i64> with min/max values =====

#[test]
fn test_range_i64_min_max_roundtrip() {
    let r: Range<i64> = i64::MIN..i64::MAX;
    let encoded = encode_to_vec(&r).expect("encode Range<i64> MIN..MAX");
    let (decoded, _): (Range<i64>, usize) =
        decode_from_slice(&encoded).expect("decode Range<i64> MIN..MAX");
    assert_eq!(decoded.start, i64::MIN);
    assert_eq!(decoded.end, i64::MAX);
    assert_eq!(r, decoded);
}

// ===== 7. RangeInclusive<u32> simple (0..=10) =====

#[test]
fn test_range_inclusive_u32_zero_to_ten_roundtrip() {
    let r: RangeInclusive<u32> = 0..=10;
    let encoded = encode_to_vec(&r).expect("encode RangeInclusive<u32> 0..=10");
    let (decoded, _): (RangeInclusive<u32>, usize) =
        decode_from_slice(&encoded).expect("decode RangeInclusive<u32> 0..=10");
    assert_eq!(r, decoded);
    assert_eq!(*decoded.start(), 0u32);
    assert_eq!(*decoded.end(), 10u32);
}

// ===== 8. RangeInclusive<u32> single element (5..=5) =====

#[test]
fn test_range_inclusive_u32_single_element_roundtrip() {
    let r: RangeInclusive<u32> = 5..=5;
    let encoded = encode_to_vec(&r).expect("encode RangeInclusive<u32> single");
    let (decoded, _): (RangeInclusive<u32>, usize) =
        decode_from_slice(&encoded).expect("decode RangeInclusive<u32> single");
    assert_eq!(r, decoded);
    assert_eq!(*decoded.start(), *decoded.end());
}

// ===== 9. RangeInclusive<i32> crossing zero (-5..=5) =====

#[test]
fn test_range_inclusive_i32_crossing_zero_roundtrip() {
    let r: RangeInclusive<i32> = -5..=5;
    let encoded = encode_to_vec(&r).expect("encode RangeInclusive<i32> -5..=5");
    let (decoded, _): (RangeInclusive<i32>, usize) =
        decode_from_slice(&encoded).expect("decode RangeInclusive<i32> -5..=5");
    assert_eq!(r, decoded);
    assert_eq!(*decoded.start(), -5i32);
    assert_eq!(*decoded.end(), 5i32);
}

// ===== 10. RangeInclusive<u64> =====

#[test]
fn test_range_inclusive_u64_roundtrip() {
    let r: RangeInclusive<u64> = 0u64..=u64::MAX;
    let encoded = encode_to_vec(&r).expect("encode RangeInclusive<u64>");
    let (decoded, _): (RangeInclusive<u64>, usize) =
        decode_from_slice(&encoded).expect("decode RangeInclusive<u64>");
    assert_eq!(*decoded.start(), 0u64);
    assert_eq!(*decoded.end(), u64::MAX);
    assert_eq!(r, decoded);
}

// ===== 11. Bound<u32> Included variant =====

#[test]
fn test_bound_u32_included_roundtrip() {
    let b: Bound<u32> = Bound::Included(42u32);
    let encoded = encode_to_vec(&b).expect("encode Bound::Included");
    let (decoded, _): (Bound<u32>, usize) =
        decode_from_slice(&encoded).expect("decode Bound::Included");
    assert_eq!(b, decoded);
    match decoded {
        Bound::Included(v) => assert_eq!(v, 42u32),
        _ => panic!("expected Bound::Included"),
    }
}

// ===== 12. Bound<u32> Excluded variant =====

#[test]
fn test_bound_u32_excluded_roundtrip() {
    let b: Bound<u32> = Bound::Excluded(99u32);
    let encoded = encode_to_vec(&b).expect("encode Bound::Excluded");
    let (decoded, _): (Bound<u32>, usize) =
        decode_from_slice(&encoded).expect("decode Bound::Excluded");
    assert_eq!(b, decoded);
    match decoded {
        Bound::Excluded(v) => assert_eq!(v, 99u32),
        _ => panic!("expected Bound::Excluded"),
    }
}

// ===== 13. Bound<u32> Unbounded variant =====

#[test]
fn test_bound_u32_unbounded_roundtrip() {
    let b: Bound<u32> = Bound::Unbounded;
    let encoded = encode_to_vec(&b).expect("encode Bound::Unbounded");
    let (decoded, _): (Bound<u32>, usize) =
        decode_from_slice(&encoded).expect("decode Bound::Unbounded");
    assert_eq!(b, decoded);
    assert!(matches!(decoded, Bound::Unbounded));
}

// ===== 14. Bound<String> Included =====

#[test]
fn test_bound_string_included_roundtrip() {
    let b: Bound<String> = Bound::Included("hello".to_string());
    let encoded = encode_to_vec(&b).expect("encode Bound<String> Included");
    let (decoded, _): (Bound<String>, usize) =
        decode_from_slice(&encoded).expect("decode Bound<String> Included");
    assert_eq!(b, decoded);
    match decoded {
        Bound::Included(s) => assert_eq!(s, "hello"),
        _ => panic!("expected Bound::Included"),
    }
}

// ===== 15. Vec<Range<u32>> roundtrip =====

#[test]
fn test_vec_range_u32_roundtrip() {
    let ranges: Vec<Range<u32>> = vec![0..1, 10..20, 100..200, 1000..2000, u32::MAX - 1..u32::MAX];
    let encoded = encode_to_vec(&ranges).expect("encode Vec<Range<u32>>");
    let (decoded, _): (Vec<Range<u32>>, usize) =
        decode_from_slice(&encoded).expect("decode Vec<Range<u32>>");
    assert_eq!(ranges, decoded);
    assert_eq!(decoded.len(), 5);
}

// ===== 16. Option<Range<u32>> Some =====

#[test]
fn test_option_range_u32_some_roundtrip() {
    let opt: Option<Range<u32>> = Some(7..77);
    let encoded = encode_to_vec(&opt).expect("encode Option<Range<u32>> Some");
    let (decoded, _): (Option<Range<u32>>, usize) =
        decode_from_slice(&encoded).expect("decode Option<Range<u32>> Some");
    assert!(decoded.is_some());
    assert_eq!(opt, decoded);
}

// ===== 17. Option<Range<u32>> None =====

#[test]
fn test_option_range_u32_none_roundtrip() {
    let opt: Option<Range<u32>> = None;
    let encoded = encode_to_vec(&opt).expect("encode Option<Range<u32>> None");
    let (decoded, _): (Option<Range<u32>>, usize) =
        decode_from_slice(&encoded).expect("decode Option<Range<u32>> None");
    assert!(decoded.is_none());
    assert_eq!(opt, decoded);
}

// ===== 18. Fixed-int config with Range<u32> =====

#[test]
fn test_range_u32_fixed_int_config_roundtrip() {
    let r: Range<u32> = 0..u32::MAX;
    let cfg = config::standard().with_fixed_int_encoding();
    let encoded = encode_to_vec_with_config(&r, cfg).expect("encode Range<u32> fixed-int");
    // With fixed encoding: two u32 values = 2 × 4 bytes = 8 bytes
    assert_eq!(encoded.len(), 8, "fixed-int Range<u32> must be 8 bytes");
    let (decoded, consumed): (Range<u32>, usize) =
        decode_from_slice_with_config(&encoded, cfg).expect("decode Range<u32> fixed-int");
    assert_eq!(r, decoded);
    assert_eq!(consumed, 8);
}

// ===== 19. Big-endian config with Range<u32> =====

#[test]
fn test_range_u32_big_endian_config_roundtrip() {
    let r: Range<u32> = 1..1000;
    let cfg = config::standard()
        .with_fixed_int_encoding()
        .with_big_endian();
    let encoded = encode_to_vec_with_config(&r, cfg).expect("encode Range<u32> big-endian");
    let (decoded, consumed): (Range<u32>, usize) =
        decode_from_slice_with_config(&encoded, cfg).expect("decode Range<u32> big-endian");
    assert_eq!(r, decoded);
    assert_eq!(consumed, encoded.len());
    // Big-endian byte order: start=1 → bytes [0, 0, 0, 1]
    assert_eq!(encoded[0], 0x00);
    assert_eq!(encoded[1], 0x00);
    assert_eq!(encoded[2], 0x00);
    assert_eq!(encoded[3], 0x01);
}

// ===== 20. Struct with multiple range fields =====

#[derive(Debug, PartialEq, Encode, Decode)]
struct MultiRangeStruct {
    x_range: Range<u32>,
    y_range: RangeInclusive<i32>,
    lower_bound: Bound<u64>,
    upper_bound: Bound<u64>,
}

#[test]
fn test_struct_with_multiple_range_fields_roundtrip() {
    let s = MultiRangeStruct {
        x_range: 10..200,
        y_range: -50..=50,
        lower_bound: Bound::Included(0u64),
        upper_bound: Bound::Excluded(1_000_000u64),
    };
    let encoded = encode_to_vec(&s).expect("encode MultiRangeStruct");
    let (decoded, _): (MultiRangeStruct, usize) =
        decode_from_slice(&encoded).expect("decode MultiRangeStruct");
    assert_eq!(s, decoded);
    assert_eq!(decoded.x_range, 10..200);
    assert_eq!(*decoded.y_range.start(), -50i32);
    assert_eq!(*decoded.y_range.end(), 50i32);
    match decoded.lower_bound {
        Bound::Included(v) => assert_eq!(v, 0u64),
        _ => panic!("expected Bound::Included for lower_bound"),
    }
    match decoded.upper_bound {
        Bound::Excluded(v) => assert_eq!(v, 1_000_000u64),
        _ => panic!("expected Bound::Excluded for upper_bound"),
    }
}

// ===== 21. Range<u32>: consumed bytes == encoded.len() =====

#[test]
fn test_range_u32_consumed_equals_encoded_len() {
    let r: Range<u32> = 100..999;
    let encoded = encode_to_vec(&r).expect("encode Range<u32> for consumed check");
    let (_, consumed): (Range<u32>, usize) =
        decode_from_slice(&encoded).expect("decode Range<u32> for consumed check");
    assert_eq!(consumed, encoded.len());
}

// ===== 22. RangeInclusive<u32>: consumed bytes == encoded.len() =====

#[test]
fn test_range_inclusive_u32_consumed_equals_encoded_len() {
    let r: RangeInclusive<u32> = 100..=999;
    let encoded = encode_to_vec(&r).expect("encode RangeInclusive<u32> for consumed check");
    let (_, consumed): (RangeInclusive<u32>, usize) =
        decode_from_slice(&encoded).expect("decode RangeInclusive<u32> for consumed check");
    assert_eq!(consumed, encoded.len());
}
