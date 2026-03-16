//! Advanced comprehensive tests for Range, RangeInclusive, and Bound types.
//! These tests exercise edge cases and compositions not covered in std_extra_types_test.rs.

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
use oxicode::{config, decode_from_slice, encode_to_vec, encoded_size, Decode, Encode};
use std::ops::{Bound, Range, RangeInclusive};

// ===== 1. Range<u32> (0..100) roundtrip =====

#[test]
fn test_range_u32_zero_to_hundred_roundtrip() {
    let r: Range<u32> = 0..100;
    let encoded = encode_to_vec(&r).expect("encode");
    let (decoded, consumed): (Range<u32>, _) = decode_from_slice(&encoded).expect("decode");
    assert_eq!(r, decoded);
    assert_eq!(consumed, encoded.len());
}

// ===== 2. Range<u32> empty range (5..5) roundtrip =====

#[test]
fn test_range_u32_empty_five_to_five_roundtrip() {
    let r: Range<u32> = 5..5;
    assert!(r.is_empty());
    let encoded = encode_to_vec(&r).expect("encode");
    let (decoded, consumed): (Range<u32>, _) = decode_from_slice(&encoded).expect("decode");
    assert_eq!(r, decoded);
    assert!(decoded.is_empty());
    assert_eq!(consumed, encoded.len());
}

// ===== 3. Range<u32> single element (5..6) roundtrip =====

#[test]
fn test_range_u32_single_element_five_to_six_roundtrip() {
    let r: Range<u32> = 5..6;
    let encoded = encode_to_vec(&r).expect("encode");
    let (decoded, consumed): (Range<u32>, _) = decode_from_slice(&encoded).expect("decode");
    assert_eq!(r, decoded);
    assert_eq!(decoded.len(), 1);
    assert_eq!(consumed, encoded.len());
}

// ===== 4. Range<i64> with negative values (-100..100) roundtrip =====

#[test]
fn test_range_i64_negative_to_positive_roundtrip() {
    let r: Range<i64> = -100..100;
    let encoded = encode_to_vec(&r).expect("encode");
    let (decoded, consumed): (Range<i64>, _) = decode_from_slice(&encoded).expect("decode");
    assert_eq!(r, decoded);
    assert_eq!(decoded.start, -100i64);
    assert_eq!(decoded.end, 100i64);
    assert_eq!(consumed, encoded.len());
}

// ===== 5. Range<usize> (0..1000) roundtrip =====

#[test]
fn test_range_usize_zero_to_thousand_roundtrip() {
    let r: Range<usize> = 0..1000;
    let encoded = encode_to_vec(&r).expect("encode");
    let (decoded, consumed): (Range<usize>, _) = decode_from_slice(&encoded).expect("decode");
    assert_eq!(r, decoded);
    assert_eq!(decoded.len(), 1000);
    assert_eq!(consumed, encoded.len());
}

// ===== 6. RangeInclusive<u32> (0..=100) roundtrip =====

#[test]
fn test_range_inclusive_u32_zero_to_hundred_roundtrip() {
    let r: RangeInclusive<u32> = 0..=100;
    let encoded = encode_to_vec(&r).expect("encode");
    let (decoded, consumed): (RangeInclusive<u32>, _) =
        decode_from_slice(&encoded).expect("decode");
    assert_eq!(r, decoded);
    assert_eq!(*decoded.start(), 0u32);
    assert_eq!(*decoded.end(), 100u32);
    assert_eq!(consumed, encoded.len());
}

// ===== 7. RangeInclusive<u32> single element (5..=5) roundtrip =====

#[test]
fn test_range_inclusive_u32_single_element_roundtrip() {
    let r: RangeInclusive<u32> = 5..=5;
    let encoded = encode_to_vec(&r).expect("encode");
    let (decoded, consumed): (RangeInclusive<u32>, _) =
        decode_from_slice(&encoded).expect("decode");
    assert_eq!(r, decoded);
    assert_eq!(*decoded.start(), 5u32);
    assert_eq!(*decoded.end(), 5u32);
    assert_eq!(consumed, encoded.len());
}

// ===== 8. RangeInclusive<i64> with negative values roundtrip =====

#[test]
fn test_range_inclusive_i64_negative_values_roundtrip() {
    let r: RangeInclusive<i64> = -1000..=1000;
    let encoded = encode_to_vec(&r).expect("encode");
    let (decoded, consumed): (RangeInclusive<i64>, _) =
        decode_from_slice(&encoded).expect("decode");
    assert_eq!(r, decoded);
    assert_eq!(*decoded.start(), -1000i64);
    assert_eq!(*decoded.end(), 1000i64);
    assert_eq!(consumed, encoded.len());
}

// ===== 9. Bound<u32>::Included roundtrip =====

#[test]
fn test_bound_u32_included_roundtrip() {
    let b: Bound<u32> = Bound::Included(77);
    let encoded = encode_to_vec(&b).expect("encode");
    let (decoded, consumed): (Bound<u32>, _) = decode_from_slice(&encoded).expect("decode");
    assert_eq!(b, decoded);
    match decoded {
        Bound::Included(v) => assert_eq!(v, 77u32),
        _ => panic!("Expected Bound::Included"),
    }
    assert_eq!(consumed, encoded.len());
}

// ===== 10. Bound<u32>::Excluded roundtrip =====

#[test]
fn test_bound_u32_excluded_roundtrip() {
    let b: Bound<u32> = Bound::Excluded(255);
    let encoded = encode_to_vec(&b).expect("encode");
    let (decoded, consumed): (Bound<u32>, _) = decode_from_slice(&encoded).expect("decode");
    assert_eq!(b, decoded);
    match decoded {
        Bound::Excluded(v) => assert_eq!(v, 255u32),
        _ => panic!("Expected Bound::Excluded"),
    }
    assert_eq!(consumed, encoded.len());
}

// ===== 11. Bound<u32>::Unbounded roundtrip =====

#[test]
fn test_bound_u32_unbounded_roundtrip() {
    let b: Bound<u32> = Bound::Unbounded;
    let encoded = encode_to_vec(&b).expect("encode");
    let (decoded, consumed): (Bound<u32>, _) = decode_from_slice(&encoded).expect("decode");
    assert_eq!(b, decoded);
    assert!(matches!(decoded, Bound::Unbounded));
    assert_eq!(consumed, encoded.len());
}

// ===== 12. Range<u32> in struct (derive) =====

#[derive(Debug, PartialEq, Encode, Decode)]
struct RangeHolder {
    name: String,
    range: Range<u32>,
    priority: u8,
}

#[test]
fn test_struct_with_range_u32_derive_roundtrip() {
    let holder = RangeHolder {
        name: "test-range".to_string(),
        range: 10..50,
        priority: 3,
    };
    let encoded = encode_to_vec(&holder).expect("encode");
    let (decoded, consumed): (RangeHolder, _) = decode_from_slice(&encoded).expect("decode");
    assert_eq!(holder, decoded);
    assert_eq!(decoded.range, 10..50);
    assert_eq!(consumed, encoded.len());
}

// ===== 13. Vec<Range<u32>> roundtrip =====

#[test]
fn test_vec_of_range_u32_roundtrip() {
    let ranges: Vec<Range<u32>> = vec![0..10, 20..30, 50..100, 200..201, 999..1000];
    let encoded = encode_to_vec(&ranges).expect("encode");
    let (decoded, consumed): (Vec<Range<u32>>, _) = decode_from_slice(&encoded).expect("decode");
    assert_eq!(ranges, decoded);
    assert_eq!(decoded.len(), 5);
    assert_eq!(consumed, encoded.len());
}

// ===== 14. Option<Range<u32>> roundtrip =====

#[test]
fn test_option_range_u32_some_and_none_roundtrip() {
    let some_range: Option<Range<u32>> = Some(42..100);
    let encoded_some = encode_to_vec(&some_range).expect("encode some");
    let (decoded_some, _): (Option<Range<u32>>, _) =
        decode_from_slice(&encoded_some).expect("decode some");
    assert_eq!(some_range, decoded_some);

    let none_range: Option<Range<u32>> = None;
    let encoded_none = encode_to_vec(&none_range).expect("encode none");
    let (decoded_none, _): (Option<Range<u32>>, _) =
        decode_from_slice(&encoded_none).expect("decode none");
    assert_eq!(none_range, decoded_none);

    // Ensure encoded lengths differ (Some vs None)
    assert_ne!(encoded_some.len(), encoded_none.len());
}

// ===== 15. Range<String> roundtrip =====

#[test]
fn test_range_string_unicode_roundtrip() {
    let r: Range<String> = "alpha".to_string().."omega".to_string();
    let encoded = encode_to_vec(&r).expect("encode");
    let (decoded, consumed): (Range<String>, _) = decode_from_slice(&encoded).expect("decode");
    assert_eq!(r, decoded);
    assert_eq!(decoded.start, "alpha");
    assert_eq!(decoded.end, "omega");
    assert_eq!(consumed, encoded.len());
}

// ===== 16. RangeInclusive<String> roundtrip =====

#[test]
fn test_range_inclusive_string_roundtrip() {
    let r: RangeInclusive<String> = "aardvark".to_string()..="zebra".to_string();
    let encoded = encode_to_vec(&r).expect("encode");
    let (decoded, consumed): (RangeInclusive<String>, _) =
        decode_from_slice(&encoded).expect("decode");
    assert_eq!(r, decoded);
    assert_eq!(decoded.start(), "aardvark");
    assert_eq!(decoded.end(), "zebra");
    assert_eq!(consumed, encoded.len());
}

// ===== 17. Vec<RangeInclusive<u32>> 100 elements roundtrip =====

#[test]
fn test_vec_of_range_inclusive_u32_100_elements_roundtrip() {
    let ranges: Vec<RangeInclusive<u32>> = (0u32..100).map(|i| i..=(i + 10)).collect();
    assert_eq!(ranges.len(), 100);

    let encoded = encode_to_vec(&ranges).expect("encode");
    let (decoded, consumed): (Vec<RangeInclusive<u32>>, _) =
        decode_from_slice(&encoded).expect("decode");
    assert_eq!(ranges, decoded);
    assert_eq!(decoded.len(), 100);
    assert_eq!(*decoded[0].start(), 0u32);
    assert_eq!(*decoded[0].end(), 10u32);
    assert_eq!(*decoded[99].start(), 99u32);
    assert_eq!(*decoded[99].end(), 109u32);
    assert_eq!(consumed, encoded.len());
}

// ===== 18. Range<f64> roundtrip =====

#[test]
fn test_range_f64_roundtrip() {
    let r: Range<f64> = -std::f64::consts::PI..std::f64::consts::E;
    let encoded = encode_to_vec(&r).expect("encode");
    let (decoded, consumed): (Range<f64>, _) = decode_from_slice(&encoded).expect("decode");
    // f64 exact equality is valid here since we're just serializing/deserializing bits
    assert_eq!(r.start.to_bits(), decoded.start.to_bits());
    assert_eq!(r.end.to_bits(), decoded.end.to_bits());
    assert_eq!(consumed, encoded.len());
}

// ===== 19. RangeInclusive<f64> roundtrip =====

#[test]
fn test_range_inclusive_f64_special_values_roundtrip() {
    let r: RangeInclusive<f64> = f64::MIN..=f64::MAX;
    let encoded = encode_to_vec(&r).expect("encode");
    let (decoded, consumed): (RangeInclusive<f64>, _) =
        decode_from_slice(&encoded).expect("decode");
    assert_eq!(decoded.start().to_bits(), f64::MIN.to_bits());
    assert_eq!(decoded.end().to_bits(), f64::MAX.to_bits());
    assert_eq!(consumed, encoded.len());
}

// ===== 20. encoded_size for Range<u32> =====

#[test]
fn test_encoded_size_range_u32() {
    let r: Range<u32> = 0..100;
    let size = encoded_size(&r).expect("encoded_size");

    // Verify consistency: encoded_size must match the actual encoded byte length
    let encoded = encode_to_vec(&r).expect("encode");
    assert_eq!(
        size,
        encoded.len(),
        "encoded_size must match actual encoded length"
    );

    // With fixed int encoding, Range<u32> should encode as two u32 values = 8 bytes
    let fixed_size = {
        let cfg = config::standard().with_fixed_int_encoding();
        let encoded_fixed = oxicode::encode_to_vec_with_config(&r, cfg).expect("encode fixed");
        encoded_fixed.len()
    };
    assert_eq!(
        fixed_size, 8,
        "Range<u32> with fixed encoding should be 8 bytes (2 × u32)"
    );
}

// ===== 21. encoded_size for RangeInclusive<u32> =====

#[test]
fn test_encoded_size_range_inclusive_u32() {
    let r: RangeInclusive<u32> = 0..=255;
    let size = encoded_size(&r).expect("encoded_size");

    // Verify consistency with actual encoding
    let encoded = encode_to_vec(&r).expect("encode");
    assert_eq!(
        size,
        encoded.len(),
        "encoded_size must match actual encoded length"
    );

    // With fixed int encoding, RangeInclusive<u32> should encode as two u32 values = 8 bytes
    let fixed_size = {
        let cfg = config::standard().with_fixed_int_encoding();
        let encoded_fixed = oxicode::encode_to_vec_with_config(&r, cfg).expect("encode fixed");
        encoded_fixed.len()
    };
    assert_eq!(
        fixed_size, 8,
        "RangeInclusive<u32> with fixed encoding should be 8 bytes (2 × u32)"
    );
}

// ===== 22. Range<u64> with large boundaries roundtrip =====

#[test]
fn test_range_u64_large_boundaries_roundtrip() {
    let r: Range<u64> = (u64::MAX / 2)..(u64::MAX);
    let encoded = encode_to_vec(&r).expect("encode");
    let (decoded, consumed): (Range<u64>, _) = decode_from_slice(&encoded).expect("decode");
    assert_eq!(r, decoded);
    assert_eq!(decoded.start, u64::MAX / 2);
    assert_eq!(decoded.end, u64::MAX);
    assert_eq!(consumed, encoded.len());

    // Also verify that min..max roundtrips correctly
    let full_range: Range<u64> = u64::MIN..u64::MAX;
    let encoded_full = encode_to_vec(&full_range).expect("encode full");
    let (decoded_full, _): (Range<u64>, _) = decode_from_slice(&encoded_full).expect("decode full");
    assert_eq!(full_range, decoded_full);
}
