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
use proptest::prelude::*;

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct TimeSeriesPoint {
    ts_ms: u64,
    value: i64,
    quality: u8,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
enum AggregationFunc {
    Sum,
    Avg,
    Min,
    Max,
    Count,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct TimeSeries {
    name: String,
    points: Vec<TimeSeriesPoint>,
    func: AggregationFunc,
}

fn arb_quality() -> impl Strategy<Value = u8> {
    0u8..=4u8
}

fn arb_aggregation_func() -> impl Strategy<Value = AggregationFunc> {
    prop_oneof![
        Just(AggregationFunc::Sum),
        Just(AggregationFunc::Avg),
        Just(AggregationFunc::Min),
        Just(AggregationFunc::Max),
        Just(AggregationFunc::Count),
    ]
}

fn arb_point() -> impl Strategy<Value = TimeSeriesPoint> {
    (any::<u64>(), any::<i64>(), arb_quality()).prop_map(|(ts_ms, value, quality)| {
        TimeSeriesPoint {
            ts_ms,
            value,
            quality,
        }
    })
}

fn arb_timeseries() -> impl Strategy<Value = TimeSeries> {
    (
        "[a-zA-Z][a-zA-Z0-9_]{0,31}",
        prop::collection::vec(arb_point(), 0..=16),
        arb_aggregation_func(),
    )
        .prop_map(|(name, points, func)| TimeSeries { name, points, func })
}

// Test 1: TimeSeriesPoint round-trip
#[test]
fn test_ts_point_roundtrip() {
    proptest!(|(ts_ms in any::<u64>(), value in any::<i64>(), quality in arb_quality())| {
        let point = TimeSeriesPoint { ts_ms, value, quality };
        let bytes = encode_to_vec(&point).expect("encode failed");
        let (decoded, consumed): (TimeSeriesPoint, usize) =
            decode_from_slice(&bytes).expect("decode failed");
        prop_assert_eq!(&point, &decoded);
        prop_assert_eq!(consumed, bytes.len());
    });
}

// Test 2: AggregationFunc round-trip
#[test]
fn test_aggregation_func_roundtrip() {
    proptest!(|(func in arb_aggregation_func())| {
        let bytes = encode_to_vec(&func).expect("encode failed");
        let (decoded, consumed): (AggregationFunc, usize) =
            decode_from_slice(&bytes).expect("decode failed");
        prop_assert_eq!(&func, &decoded);
        prop_assert_eq!(consumed, bytes.len());
    });
}

// Test 3: TimeSeries round-trip
#[test]
fn test_timeseries_roundtrip() {
    proptest!(|(ts in arb_timeseries())| {
        let bytes = encode_to_vec(&ts).expect("encode failed");
        let (decoded, consumed): (TimeSeries, usize) =
            decode_from_slice(&bytes).expect("decode failed");
        prop_assert_eq!(&ts, &decoded);
        prop_assert_eq!(consumed, bytes.len());
    });
}

// Test 4: Empty points vector round-trip
#[test]
fn test_timeseries_empty_points() {
    proptest!(|(name in "[a-zA-Z][a-zA-Z0-9_]{0,15}", func in arb_aggregation_func())| {
        let ts = TimeSeries { name, points: vec![], func };
        let bytes = encode_to_vec(&ts).expect("encode failed");
        let (decoded, _): (TimeSeries, usize) =
            decode_from_slice(&bytes).expect("decode failed");
        prop_assert_eq!(&ts, &decoded);
        prop_assert!(decoded.points.is_empty());
    });
}

// Test 5: Large points vector round-trip
#[test]
fn test_timeseries_large_points() {
    proptest!(|(points in prop::collection::vec(arb_point(), 50..=100))| {
        let ts = TimeSeries {
            name: "large_series".to_string(),
            points: points.clone(),
            func: AggregationFunc::Sum,
        };
        let bytes = encode_to_vec(&ts).expect("encode failed");
        let (decoded, _): (TimeSeries, usize) =
            decode_from_slice(&bytes).expect("decode failed");
        prop_assert_eq!(decoded.points.len(), points.len());
        prop_assert_eq!(&ts, &decoded);
    });
}

// Test 6: Encoded byte length is deterministic
#[test]
fn test_encode_deterministic() {
    proptest!(|(ts in arb_timeseries())| {
        let bytes1 = encode_to_vec(&ts).expect("encode failed");
        let bytes2 = encode_to_vec(&ts).expect("encode failed");
        prop_assert_eq!(bytes1, bytes2);
    });
}

// Test 7: Two distinct points encode differently
#[test]
fn test_distinct_points_encode_differently() {
    proptest!(|(ts_ms1 in any::<u64>(), ts_ms2 in any::<u64>())| {
        prop_assume!(ts_ms1 != ts_ms2);
        let p1 = TimeSeriesPoint { ts_ms: ts_ms1, value: 0, quality: 0 };
        let p2 = TimeSeriesPoint { ts_ms: ts_ms2, value: 0, quality: 0 };
        let bytes1 = encode_to_vec(&p1).expect("encode failed");
        let bytes2 = encode_to_vec(&p2).expect("encode failed");
        prop_assert_ne!(bytes1, bytes2);
    });
}

// Test 8: Point value field round-trip
#[test]
fn test_ts_point_value_preserved() {
    proptest!(|(value in any::<i64>())| {
        let point = TimeSeriesPoint { ts_ms: 0, value, quality: 0 };
        let bytes = encode_to_vec(&point).expect("encode failed");
        let (decoded, _): (TimeSeriesPoint, usize) =
            decode_from_slice(&bytes).expect("decode failed");
        prop_assert_eq!(decoded.value, value);
    });
}

// Test 9: Point quality field round-trip
#[test]
fn test_ts_point_quality_preserved() {
    proptest!(|(quality in arb_quality())| {
        let point = TimeSeriesPoint { ts_ms: 42, value: -1, quality };
        let bytes = encode_to_vec(&point).expect("encode failed");
        let (decoded, _): (TimeSeriesPoint, usize) =
            decode_from_slice(&bytes).expect("decode failed");
        prop_assert_eq!(decoded.quality, quality);
    });
}

// Test 10: TimeSeries name preserved
#[test]
fn test_timeseries_name_preserved() {
    proptest!(|(name in "[a-zA-Z][a-zA-Z0-9_]{0,31}")| {
        let ts = TimeSeries {
            name: name.clone(),
            points: vec![],
            func: AggregationFunc::Count,
        };
        let bytes = encode_to_vec(&ts).expect("encode failed");
        let (decoded, _): (TimeSeries, usize) =
            decode_from_slice(&bytes).expect("decode failed");
        prop_assert_eq!(decoded.name, name);
    });
}

// Test 11: Consumed == encoded length for TimeSeries
#[test]
fn test_consumed_equals_encoded_len() {
    proptest!(|(ts in arb_timeseries())| {
        let bytes = encode_to_vec(&ts).expect("encode failed");
        let expected_len = bytes.len();
        let (_, consumed): (TimeSeries, usize) =
            decode_from_slice(&bytes).expect("decode failed");
        prop_assert_eq!(consumed, expected_len);
    });
}

// Test 12: Appending garbage after encoded bytes does not affect decode
#[test]
fn test_decode_ignores_trailing_bytes() {
    proptest!(|(ts in arb_timeseries(), extra in prop::collection::vec(any::<u8>(), 1..=16))| {
        let mut bytes = encode_to_vec(&ts).expect("encode failed");
        let original_len = bytes.len();
        bytes.extend_from_slice(&extra);
        let (decoded, consumed): (TimeSeries, usize) =
            decode_from_slice(&bytes).expect("decode failed");
        prop_assert_eq!(&ts, &decoded);
        prop_assert_eq!(consumed, original_len);
    });
}

// Test 13: Two identical TimeSeries encode to same bytes
#[test]
fn test_identical_timeseries_same_encoding() {
    proptest!(|(ts in arb_timeseries())| {
        let cloned = ts.clone();
        let bytes_a = encode_to_vec(&ts).expect("encode failed");
        let bytes_b = encode_to_vec(&cloned).expect("encode failed");
        prop_assert_eq!(bytes_a, bytes_b);
    });
}

// Test 14: All AggregationFunc variants round-trip with points
#[test]
fn test_all_agg_funcs_with_points() {
    proptest!(|(points in prop::collection::vec(arb_point(), 0..=8), func in arb_aggregation_func())| {
        let ts = TimeSeries {
            name: "agg_test".to_string(),
            points,
            func,
        };
        let bytes = encode_to_vec(&ts).expect("encode failed");
        let (decoded, _): (TimeSeries, usize) =
            decode_from_slice(&bytes).expect("decode failed");
        prop_assert_eq!(&ts, &decoded);
    });
}

// Test 15: TimeSeries with single point
#[test]
fn test_timeseries_single_point() {
    proptest!(|(point in arb_point(), func in arb_aggregation_func())| {
        let ts = TimeSeries {
            name: "single".to_string(),
            points: vec![point.clone()],
            func,
        };
        let bytes = encode_to_vec(&ts).expect("encode failed");
        let (decoded, _): (TimeSeries, usize) =
            decode_from_slice(&bytes).expect("decode failed");
        prop_assert_eq!(decoded.points.len(), 1);
        prop_assert_eq!(&decoded.points[0], &point);
    });
}

// Test 16: Changing a single point changes the encoding
#[test]
fn test_modified_point_changes_encoding() {
    proptest!(|(ts_ms in any::<u64>(), v1 in any::<i64>(), v2 in any::<i64>())| {
        prop_assume!(v1 != v2);
        let ts1 = TimeSeries {
            name: "m".to_string(),
            points: vec![TimeSeriesPoint { ts_ms, value: v1, quality: 0 }],
            func: AggregationFunc::Avg,
        };
        let ts2 = TimeSeries {
            name: "m".to_string(),
            points: vec![TimeSeriesPoint { ts_ms, value: v2, quality: 0 }],
            func: AggregationFunc::Avg,
        };
        let bytes1 = encode_to_vec(&ts1).expect("encode failed");
        let bytes2 = encode_to_vec(&ts2).expect("encode failed");
        prop_assert_ne!(bytes1, bytes2);
    });
}

// Test 17: Encoded bytes are non-empty for any input
#[test]
fn test_encoded_bytes_nonempty() {
    proptest!(|(ts in arb_timeseries())| {
        let bytes = encode_to_vec(&ts).expect("encode failed");
        prop_assert!(!bytes.is_empty());
    });
}

// Test 18: Vec of TimeSeries round-trip
#[test]
fn test_vec_timeseries_roundtrip() {
    proptest!(|(series in prop::collection::vec(arb_timeseries(), 0..=4))| {
        let bytes = encode_to_vec(&series).expect("encode failed");
        let (decoded, consumed): (Vec<TimeSeries>, usize) =
            decode_from_slice(&bytes).expect("decode failed");
        prop_assert_eq!(&series, &decoded);
        prop_assert_eq!(consumed, bytes.len());
    });
}

// Test 19: Optional point round-trip
#[test]
fn test_option_point_roundtrip() {
    proptest!(|(opt_point in prop::option::of(arb_point()))| {
        let bytes = encode_to_vec(&opt_point).expect("encode failed");
        let (decoded, consumed): (Option<TimeSeriesPoint>, usize) =
            decode_from_slice(&bytes).expect("decode failed");
        prop_assert_eq!(&opt_point, &decoded);
        prop_assert_eq!(consumed, bytes.len());
    });
}

// Test 20: Boundary timestamps (min/max u64)
#[test]
fn test_boundary_timestamps() {
    let boundary_values: Vec<u64> = vec![0, 1, u64::MAX - 1, u64::MAX];
    for &ts_ms in &boundary_values {
        let point = TimeSeriesPoint {
            ts_ms,
            value: 0,
            quality: 0,
        };
        let bytes = encode_to_vec(&point).expect("encode failed");
        let (decoded, consumed): (TimeSeriesPoint, usize) =
            decode_from_slice(&bytes).expect("decode failed");
        assert_eq!(
            &point, &decoded,
            "boundary timestamp {ts_ms} failed roundtrip"
        );
        assert_eq!(consumed, bytes.len(), "consumed mismatch for ts_ms={ts_ms}");
    }
}

// Test 21: Boundary i64 values
#[test]
fn test_boundary_i64_values() {
    let boundary_values: Vec<i64> = vec![i64::MIN, i64::MIN + 1, -1, 0, 1, i64::MAX - 1, i64::MAX];
    for &value in &boundary_values {
        let point = TimeSeriesPoint {
            ts_ms: 0,
            value,
            quality: 0,
        };
        let bytes = encode_to_vec(&point).expect("encode failed");
        let (decoded, consumed): (TimeSeriesPoint, usize) =
            decode_from_slice(&bytes).expect("decode failed");
        assert_eq!(&point, &decoded, "boundary value {value} failed roundtrip");
        assert_eq!(consumed, bytes.len(), "consumed mismatch for value={value}");
    }
}

// Test 22: Nested double encode-decode is identity
#[test]
fn test_double_encode_decode_identity() {
    proptest!(|(ts in arb_timeseries())| {
        let bytes = encode_to_vec(&ts).expect("encode failed");
        let (decoded1, _): (TimeSeries, usize) =
            decode_from_slice(&bytes).expect("first decode failed");
        let bytes2 = encode_to_vec(&decoded1).expect("re-encode failed");
        let (decoded2, consumed2): (TimeSeries, usize) =
            decode_from_slice(&bytes2).expect("second decode failed");
        prop_assert_eq!(&ts, &decoded2);
        prop_assert_eq!(consumed2, bytes2.len());
        prop_assert_eq!(bytes, bytes2);
    });
}
