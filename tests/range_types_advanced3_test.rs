//! Advanced tests (set 3) for Range<T>, RangeInclusive<T>, and custom range-like structs.
//! Covers basic roundtrips, empty ranges, negative values, Vec/Option wrappers,
//! derived Encode/Decode on structs containing range semantics, and encoding invariants.

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
use std::ops::{Range, RangeInclusive};

// ===== Custom structs used across multiple tests =====

#[derive(Debug, PartialEq, Encode, Decode)]
struct Segment {
    start: u32,
    end: u32,
}

#[derive(Debug, PartialEq, Encode, Decode)]
struct Interval {
    lo: i64,
    hi: i64,
    inclusive: bool,
}

#[derive(Debug, PartialEq, Encode, Decode)]
struct NamedWindow {
    name: String,
    lower: u64,
    upper: u64,
}

#[derive(Debug, PartialEq, Encode, Decode)]
struct BandedRange {
    band_id: u8,
    segment: Segment,
}

#[derive(Debug, PartialEq, Encode, Decode)]
struct MultiInterval {
    primary: Interval,
    secondary: Interval,
}

#[derive(Debug, PartialEq, Encode, Decode)]
struct SliceDescriptor {
    offset: u64,
    length: u64,
    stride: u32,
}

#[derive(Debug, PartialEq, Encode, Decode)]
struct SparseRange {
    base: u32,
    step: u32,
    count: u32,
}

// ===== Test 1: Range<u32> basic roundtrip =====

#[test]
fn test_range_u32_basic_roundtrip() {
    let original: Range<u32> = 0..10;
    let encoded = encode_to_vec(&original).expect("encode Range<u32> 0..10");
    let (decoded, _): (Range<u32>, _) =
        decode_from_slice(&encoded).expect("decode Range<u32> 0..10");
    assert_eq!(original, decoded);
}

// ===== Test 2: Range<u64> roundtrip =====

#[test]
fn test_range_u64_roundtrip() {
    let original: Range<u64> = 1_000_000_000u64..9_999_999_999u64;
    let encoded = encode_to_vec(&original).expect("encode Range<u64>");
    let (decoded, _): (Range<u64>, _) = decode_from_slice(&encoded).expect("decode Range<u64>");
    assert_eq!(original, decoded);
}

// ===== Test 3: Range<i32> with negative values =====

#[test]
fn test_range_i32_negative_values_roundtrip() {
    let original: Range<i32> = -500..500;
    let encoded = encode_to_vec(&original).expect("encode Range<i32> negative");
    let (decoded, _): (Range<i32>, _) =
        decode_from_slice(&encoded).expect("decode Range<i32> negative");
    assert_eq!(original, decoded);
}

// ===== Test 4: Range<u32> empty range (5..5) =====

#[test]
fn test_range_u32_empty_roundtrip() {
    let original: Range<u32> = 5..5;
    assert!(original.is_empty(), "5..5 must be an empty range");
    let encoded = encode_to_vec(&original).expect("encode Range<u32> empty");
    let (decoded, _): (Range<u32>, _) =
        decode_from_slice(&encoded).expect("decode Range<u32> empty");
    assert_eq!(original, decoded);
    assert!(
        decoded.is_empty(),
        "decoded empty range must still be empty"
    );
}

// ===== Test 5: RangeInclusive<u32> basic roundtrip =====

#[test]
fn test_range_inclusive_u32_basic_roundtrip() {
    let original: RangeInclusive<u32> = 0..=10;
    let encoded = encode_to_vec(&original).expect("encode RangeInclusive<u32> 0..=10");
    let (decoded, _): (RangeInclusive<u32>, _) =
        decode_from_slice(&encoded).expect("decode RangeInclusive<u32> 0..=10");
    assert_eq!(original, decoded);
}

// ===== Test 6: RangeInclusive<i32> with negative values =====

#[test]
fn test_range_inclusive_i32_negative_roundtrip() {
    let original: RangeInclusive<i32> = -1000..=1000;
    let encoded = encode_to_vec(&original).expect("encode RangeInclusive<i32> negative");
    let (decoded, _): (RangeInclusive<i32>, _) =
        decode_from_slice(&encoded).expect("decode RangeInclusive<i32> negative");
    assert_eq!(original, decoded);
}

// ===== Test 7: Segment struct (custom range-like) roundtrip =====

#[test]
fn test_segment_struct_roundtrip() {
    let original = Segment {
        start: 42,
        end: 100,
    };
    let encoded = encode_to_vec(&original).expect("encode Segment struct");
    let (decoded, _): (Segment, _) = decode_from_slice(&encoded).expect("decode Segment struct");
    assert_eq!(original, decoded);
}

// ===== Test 8: Vec<Range<u32>> roundtrip =====

#[test]
fn test_vec_of_range_u32_roundtrip() {
    let original: Vec<Range<u32>> = vec![0..5, 10..20, 100..200, 0..0];
    let encoded = encode_to_vec(&original).expect("encode Vec<Range<u32>>");
    let (decoded, _): (Vec<Range<u32>>, _) =
        decode_from_slice(&encoded).expect("decode Vec<Range<u32>>");
    assert_eq!(original, decoded);
}

// ===== Test 9: Vec<RangeInclusive<u32>> roundtrip =====

#[test]
fn test_vec_of_range_inclusive_u32_roundtrip() {
    let original: Vec<RangeInclusive<u32>> = vec![0..=5, 10..=20, 100..=200, 7..=7];
    let encoded = encode_to_vec(&original).expect("encode Vec<RangeInclusive<u32>>");
    let (decoded, _): (Vec<RangeInclusive<u32>>, _) =
        decode_from_slice(&encoded).expect("decode Vec<RangeInclusive<u32>>");
    assert_eq!(original, decoded);
}

// ===== Test 10: Option<Range<u32>> Some roundtrip =====

#[test]
fn test_option_range_u32_some_roundtrip() {
    let original: Option<Range<u32>> = Some(3..99);
    let encoded = encode_to_vec(&original).expect("encode Option<Range<u32>> Some");
    let (decoded, _): (Option<Range<u32>>, _) =
        decode_from_slice(&encoded).expect("decode Option<Range<u32>> Some");
    assert_eq!(original, decoded);
    assert!(decoded.is_some(), "decoded must be Some");
}

// ===== Test 11: Option<Range<u32>> None roundtrip =====

#[test]
fn test_option_range_u32_none_roundtrip() {
    let original: Option<Range<u32>> = None;
    let encoded = encode_to_vec(&original).expect("encode Option<Range<u32>> None");
    let (decoded, _): (Option<Range<u32>>, _) =
        decode_from_slice(&encoded).expect("decode Option<Range<u32>> None");
    assert_eq!(original, decoded);
    assert!(decoded.is_none(), "decoded must be None");
}

// ===== Test 12: Derived Encode/Decode struct containing Range<u32> =====

#[derive(Debug, PartialEq, Encode, Decode)]
struct RangeHolder {
    label: u8,
    span: Range<u32>,
}

#[test]
fn test_struct_containing_range_u32_roundtrip() {
    let original = RangeHolder {
        label: 7,
        span: 50..150,
    };
    let encoded = encode_to_vec(&original).expect("encode RangeHolder struct");
    let (decoded, _): (RangeHolder, _) =
        decode_from_slice(&encoded).expect("decode RangeHolder struct");
    assert_eq!(original, decoded);
    assert_eq!(decoded.span, 50..150);
}

// ===== Test 13: Range<u32> and RangeInclusive<u32> each decode back to their own type =====

#[test]
fn test_range_vs_range_inclusive_independent_roundtrips() {
    let range: Range<u32> = 1..10;
    let range_incl: RangeInclusive<u32> = 1..=10;

    let encoded_range = encode_to_vec(&range).expect("encode Range<u32> 1..10");
    let encoded_range_incl = encode_to_vec(&range_incl).expect("encode RangeInclusive<u32> 1..=10");

    let (decoded_range, consumed_range): (Range<u32>, _) =
        decode_from_slice(&encoded_range).expect("decode Range<u32> 1..10");
    let (decoded_range_incl, consumed_range_incl): (RangeInclusive<u32>, _) =
        decode_from_slice(&encoded_range_incl).expect("decode RangeInclusive<u32> 1..=10");

    assert_eq!(decoded_range, 1..10);
    assert_eq!(decoded_range_incl, 1..=10);
    assert_eq!(consumed_range, encoded_range.len());
    assert_eq!(consumed_range_incl, encoded_range_incl.len());
}

// ===== Test 14: Range<u32> consumed bytes equals encoded length =====

#[test]
fn test_range_u32_consumed_bytes_equals_encoded_length() {
    let original: Range<u32> = 0..255;
    let encoded = encode_to_vec(&original).expect("encode Range<u32> for consumed check");
    let (_decoded, consumed): (Range<u32>, _) =
        decode_from_slice(&encoded).expect("decode Range<u32> for consumed check");
    assert_eq!(
        consumed,
        encoded.len(),
        "consumed bytes must equal the full encoded buffer length"
    );
}

// ===== Test 15: Range<i64> large values roundtrip =====

#[test]
fn test_range_i64_large_values_roundtrip() {
    let original: Range<i64> = i64::MIN..i64::MAX;
    let encoded = encode_to_vec(&original).expect("encode Range<i64> large values");
    let (decoded, consumed): (Range<i64>, _) =
        decode_from_slice(&encoded).expect("decode Range<i64> large values");
    assert_eq!(original, decoded);
    assert_eq!(consumed, encoded.len());
}

// ===== Test 16: Interval struct (custom, with inclusive flag) roundtrip =====

#[test]
fn test_interval_struct_roundtrip() {
    let original = Interval {
        lo: -1_000_000,
        hi: 1_000_000,
        inclusive: true,
    };
    let encoded = encode_to_vec(&original).expect("encode Interval struct");
    let (decoded, _): (Interval, _) = decode_from_slice(&encoded).expect("decode Interval struct");
    assert_eq!(original, decoded);
    assert!(
        decoded.inclusive,
        "inclusive flag must round-trip correctly"
    );
}

// ===== Test 17: NamedWindow struct roundtrip with u64 bounds =====

#[test]
fn test_named_window_struct_roundtrip() {
    let original = NamedWindow {
        name: "main_window".to_string(),
        lower: 0,
        upper: u64::MAX / 2,
    };
    let encoded = encode_to_vec(&original).expect("encode NamedWindow struct");
    let (decoded, _): (NamedWindow, _) =
        decode_from_slice(&encoded).expect("decode NamedWindow struct");
    assert_eq!(original, decoded);
}

// ===== Test 18: BandedRange struct (nested struct with Segment) roundtrip =====

#[test]
fn test_banded_range_nested_struct_roundtrip() {
    let original = BandedRange {
        band_id: 3,
        segment: Segment {
            start: 1024,
            end: 2048,
        },
    };
    let encoded = encode_to_vec(&original).expect("encode BandedRange struct");
    let (decoded, _): (BandedRange, _) =
        decode_from_slice(&encoded).expect("decode BandedRange struct");
    assert_eq!(original, decoded);
    assert_eq!(decoded.segment.start, 1024);
    assert_eq!(decoded.segment.end, 2048);
}

// ===== Test 19: MultiInterval struct (two nested Interval fields) roundtrip =====

#[test]
fn test_multi_interval_struct_roundtrip() {
    let original = MultiInterval {
        primary: Interval {
            lo: 0,
            hi: 100,
            inclusive: false,
        },
        secondary: Interval {
            lo: -200,
            hi: -100,
            inclusive: true,
        },
    };
    let encoded = encode_to_vec(&original).expect("encode MultiInterval struct");
    let (decoded, _): (MultiInterval, _) =
        decode_from_slice(&encoded).expect("decode MultiInterval struct");
    assert_eq!(original, decoded);
    assert!(!decoded.primary.inclusive);
    assert!(decoded.secondary.inclusive);
}

// ===== Test 20: SliceDescriptor struct roundtrip =====

#[test]
fn test_slice_descriptor_struct_roundtrip() {
    let original = SliceDescriptor {
        offset: 4096,
        length: 8192,
        stride: 64,
    };
    let encoded = encode_to_vec(&original).expect("encode SliceDescriptor struct");
    let (decoded, consumed): (SliceDescriptor, _) =
        decode_from_slice(&encoded).expect("decode SliceDescriptor struct");
    assert_eq!(original, decoded);
    assert_eq!(consumed, encoded.len());
}

// ===== Test 21: SparseRange struct roundtrip =====

#[test]
fn test_sparse_range_struct_roundtrip() {
    let original = SparseRange {
        base: 10,
        step: 5,
        count: 100,
    };
    let encoded = encode_to_vec(&original).expect("encode SparseRange struct");
    let (decoded, consumed): (SparseRange, _) =
        decode_from_slice(&encoded).expect("decode SparseRange struct");
    assert_eq!(original, decoded);
    assert_eq!(consumed, encoded.len());
}

// ===== Test 22: Vec<Segment> and Vec<Interval> each decode to their original values =====

#[test]
fn test_vec_segment_and_interval_roundtrip_with_length_check() {
    let segments: Vec<Segment> = vec![
        Segment { start: 0, end: 10 },
        Segment { start: 20, end: 30 },
        Segment { start: 40, end: 50 },
    ];
    let intervals: Vec<Interval> = vec![
        Interval {
            lo: 0,
            hi: 10,
            inclusive: false,
        },
        Interval {
            lo: -100,
            hi: 100,
            inclusive: true,
        },
    ];

    let encoded_segments = encode_to_vec(&segments).expect("encode Vec<Segment>");
    let encoded_intervals = encode_to_vec(&intervals).expect("encode Vec<Interval>");

    let (decoded_segments, consumed_s): (Vec<Segment>, _) =
        decode_from_slice(&encoded_segments).expect("decode Vec<Segment>");
    let (decoded_intervals, consumed_i): (Vec<Interval>, _) =
        decode_from_slice(&encoded_intervals).expect("decode Vec<Interval>");

    assert_eq!(decoded_segments.len(), 3);
    assert_eq!(decoded_intervals.len(), 2);
    assert_eq!(consumed_s, encoded_segments.len());
    assert_eq!(consumed_i, encoded_intervals.len());

    for (orig, dec) in segments.iter().zip(decoded_segments.iter()) {
        assert_eq!(orig, dec);
    }
    for (orig, dec) in intervals.iter().zip(decoded_intervals.iter()) {
        assert_eq!(orig, dec);
    }
}
