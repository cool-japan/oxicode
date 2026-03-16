//! Proptest-based tests for high-frequency trading / market microstructure domain.

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
enum TickSide {
    Bid,
    Ask,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
enum ExecStatus {
    New,
    PartiallyFilled,
    Filled,
    Cancelled,
    Rejected,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct TickData {
    instrument_id: u32,
    side: TickSide,
    price_ticks: u64,
    quantity: u32,
    timestamp_ns: u64,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct ExecutionReport {
    order_id: u64,
    instrument_id: u32,
    status: ExecStatus,
    filled_qty: u32,
    avg_price_ticks: u64,
    latency_ns: u32,
}

fn tick_side_strategy() -> impl Strategy<Value = TickSide> {
    (0u8..2).prop_map(|v| match v {
        0 => TickSide::Bid,
        _ => TickSide::Ask,
    })
}

fn exec_status_strategy() -> impl Strategy<Value = ExecStatus> {
    (0u8..5).prop_map(|v| match v {
        0 => ExecStatus::New,
        1 => ExecStatus::PartiallyFilled,
        2 => ExecStatus::Filled,
        3 => ExecStatus::Cancelled,
        _ => ExecStatus::Rejected,
    })
}

fn tick_data_strategy() -> impl Strategy<Value = TickData> {
    (
        any::<u32>(),
        tick_side_strategy(),
        any::<u64>(),
        any::<u32>(),
        any::<u64>(),
    )
        .prop_map(
            |(instrument_id, side, price_ticks, quantity, timestamp_ns)| TickData {
                instrument_id,
                side,
                price_ticks,
                quantity,
                timestamp_ns,
            },
        )
}

fn execution_report_strategy() -> impl Strategy<Value = ExecutionReport> {
    (
        any::<u64>(),
        any::<u32>(),
        exec_status_strategy(),
        any::<u32>(),
        any::<u64>(),
        any::<u32>(),
    )
        .prop_map(
            |(order_id, instrument_id, status, filled_qty, avg_price_ticks, latency_ns)| {
                ExecutionReport {
                    order_id,
                    instrument_id,
                    status,
                    filled_qty,
                    avg_price_ticks,
                    latency_ns,
                }
            },
        )
}

proptest! {
    #[test]
    fn test_tick_data_roundtrip(tick in tick_data_strategy()) {
        let encoded = encode_to_vec(&tick).expect("TickData encode failed");
        let (decoded, _): (TickData, usize) =
            decode_from_slice(&encoded).expect("TickData decode failed");
        prop_assert_eq!(tick, decoded);
    }

    #[test]
    fn test_execution_report_roundtrip(report in execution_report_strategy()) {
        let encoded = encode_to_vec(&report).expect("ExecutionReport encode failed");
        let (decoded, _): (ExecutionReport, usize) =
            decode_from_slice(&encoded).expect("ExecutionReport decode failed");
        prop_assert_eq!(report, decoded);
    }

    #[test]
    fn test_tick_data_consumed_bytes_equals_encoded_length(tick in tick_data_strategy()) {
        let encoded = encode_to_vec(&tick).expect("TickData encode failed");
        let (_, consumed): (TickData, usize) =
            decode_from_slice(&encoded).expect("TickData decode consumed bytes failed");
        prop_assert_eq!(consumed, encoded.len());
    }

    #[test]
    fn test_execution_report_consumed_bytes_equals_encoded_length(report in execution_report_strategy()) {
        let encoded = encode_to_vec(&report).expect("ExecutionReport encode failed");
        let (_, consumed): (ExecutionReport, usize) =
            decode_from_slice(&encoded).expect("ExecutionReport decode consumed bytes failed");
        prop_assert_eq!(consumed, encoded.len());
    }

    #[test]
    fn test_tick_data_encode_deterministic(tick in tick_data_strategy()) {
        let encoded_a = encode_to_vec(&tick).expect("TickData first encode failed");
        let encoded_b = encode_to_vec(&tick).expect("TickData second encode failed");
        prop_assert_eq!(encoded_a, encoded_b);
    }

    #[test]
    fn test_execution_report_encode_deterministic(report in execution_report_strategy()) {
        let encoded_a = encode_to_vec(&report).expect("ExecutionReport first encode failed");
        let encoded_b = encode_to_vec(&report).expect("ExecutionReport second encode failed");
        prop_assert_eq!(encoded_a, encoded_b);
    }

    #[test]
    fn test_vec_tick_data_roundtrip(ticks in prop::collection::vec(tick_data_strategy(), 0..8)) {
        let encoded = encode_to_vec(&ticks).expect("Vec<TickData> encode failed");
        let (decoded, _): (Vec<TickData>, usize) =
            decode_from_slice(&encoded).expect("Vec<TickData> decode failed");
        prop_assert_eq!(ticks, decoded);
    }

    #[test]
    fn test_option_execution_report_roundtrip(
        report in proptest::option::of(execution_report_strategy()),
    ) {
        let encoded = encode_to_vec(&report).expect("Option<ExecutionReport> encode failed");
        let (decoded, _): (Option<ExecutionReport>, usize) =
            decode_from_slice(&encoded).expect("Option<ExecutionReport> decode failed");
        prop_assert_eq!(report, decoded);
    }

    #[test]
    fn test_tick_side_variant_roundtrip(idx in 0u8..2u8) {
        let side = match idx {
            0 => TickSide::Bid,
            _ => TickSide::Ask,
        };
        let encoded = encode_to_vec(&side).expect("TickSide encode failed");
        let (decoded, _): (TickSide, usize) =
            decode_from_slice(&encoded).expect("TickSide decode failed");
        prop_assert_eq!(side, decoded);
    }

    #[test]
    fn test_exec_status_variant_roundtrip(idx in 0u8..5u8) {
        let status = match idx {
            0 => ExecStatus::New,
            1 => ExecStatus::PartiallyFilled,
            2 => ExecStatus::Filled,
            3 => ExecStatus::Cancelled,
            _ => ExecStatus::Rejected,
        };
        let encoded = encode_to_vec(&status).expect("ExecStatus encode failed");
        let (decoded, _): (ExecStatus, usize) =
            decode_from_slice(&encoded).expect("ExecStatus decode failed");
        prop_assert_eq!(status, decoded);
    }

    #[test]
    fn test_u8_basic_roundtrip(val in any::<u8>()) {
        let encoded = encode_to_vec(&val).expect("u8 encode failed");
        let (decoded, _): (u8, usize) = decode_from_slice(&encoded).expect("u8 decode failed");
        prop_assert_eq!(val, decoded);
    }

    #[test]
    fn test_i32_basic_roundtrip(val in any::<i32>()) {
        let encoded = encode_to_vec(&val).expect("i32 encode failed");
        let (decoded, _): (i32, usize) = decode_from_slice(&encoded).expect("i32 decode failed");
        prop_assert_eq!(val, decoded);
    }

    #[test]
    fn test_u64_basic_roundtrip(val in any::<u64>()) {
        let encoded = encode_to_vec(&val).expect("u64 encode failed");
        let (decoded, _): (u64, usize) = decode_from_slice(&encoded).expect("u64 decode failed");
        prop_assert_eq!(val, decoded);
    }

    #[test]
    fn test_i64_basic_roundtrip(val in any::<i64>()) {
        let encoded = encode_to_vec(&val).expect("i64 encode failed");
        let (decoded, _): (i64, usize) = decode_from_slice(&encoded).expect("i64 decode failed");
        prop_assert_eq!(val, decoded);
    }

    #[test]
    fn test_bool_basic_roundtrip(val in any::<bool>()) {
        let encoded = encode_to_vec(&val).expect("bool encode failed");
        let (decoded, _): (bool, usize) =
            decode_from_slice(&encoded).expect("bool decode failed");
        prop_assert_eq!(val, decoded);
    }

    #[test]
    fn test_string_basic_roundtrip(val in "\\PC*") {
        let encoded = encode_to_vec(&val).expect("String encode failed");
        let (decoded, _): (String, usize) =
            decode_from_slice(&encoded).expect("String decode failed");
        prop_assert_eq!(val, decoded);
    }

    #[test]
    fn test_f32_basic_roundtrip(val in any::<f32>()) {
        let encoded = encode_to_vec(&val).expect("f32 encode failed");
        let (decoded, _): (f32, usize) = decode_from_slice(&encoded).expect("f32 decode failed");
        prop_assert_eq!(val.to_bits(), decoded.to_bits());
    }

    #[test]
    fn test_f64_basic_roundtrip(val in any::<f64>()) {
        let encoded = encode_to_vec(&val).expect("f64 encode failed");
        let (decoded, _): (f64, usize) = decode_from_slice(&encoded).expect("f64 decode failed");
        prop_assert_eq!(val.to_bits(), decoded.to_bits());
    }

    #[test]
    fn test_vec_u8_roundtrip(val in prop::collection::vec(any::<u8>(), 0..64)) {
        let encoded = encode_to_vec(&val).expect("Vec<u8> encode failed");
        let (decoded, _): (Vec<u8>, usize) =
            decode_from_slice(&encoded).expect("Vec<u8> decode failed");
        prop_assert_eq!(val, decoded);
    }

    #[test]
    fn test_vec_string_roundtrip(val in prop::collection::vec("\\PC*", 0..8)) {
        let encoded = encode_to_vec(&val).expect("Vec<String> encode failed");
        let (decoded, _): (Vec<String>, usize) =
            decode_from_slice(&encoded).expect("Vec<String> decode failed");
        prop_assert_eq!(val, decoded);
    }

    #[test]
    fn test_option_u64_roundtrip(val in proptest::option::of(any::<u64>())) {
        let encoded = encode_to_vec(&val).expect("Option<u64> encode failed");
        let (decoded, _): (Option<u64>, usize) =
            decode_from_slice(&encoded).expect("Option<u64> decode failed");
        prop_assert_eq!(val, decoded);
    }

    #[test]
    fn test_distinct_tick_data_encode_differently(
        instrument_id_a in 0u32..500_000u32,
        instrument_id_b in 500_001u32..u32::MAX,
        price_ticks in any::<u64>(),
        quantity in any::<u32>(),
        timestamp_ns in any::<u64>(),
    ) {
        let tick_a = TickData {
            instrument_id: instrument_id_a,
            side: TickSide::Bid,
            price_ticks,
            quantity,
            timestamp_ns,
        };
        let tick_b = TickData {
            instrument_id: instrument_id_b,
            side: TickSide::Bid,
            price_ticks,
            quantity,
            timestamp_ns,
        };
        let encoded_a = encode_to_vec(&tick_a).expect("tick_a encode failed");
        let encoded_b = encode_to_vec(&tick_b).expect("tick_b encode failed");
        prop_assert_ne!(encoded_a, encoded_b);
    }

    #[test]
    fn test_zero_latency_execution_report_roundtrip(
        order_id in any::<u64>(),
        instrument_id in any::<u32>(),
        filled_qty in any::<u32>(),
        avg_price_ticks in any::<u64>(),
        status_idx in 0u8..5u8,
    ) {
        let status = match status_idx {
            0 => ExecStatus::New,
            1 => ExecStatus::PartiallyFilled,
            2 => ExecStatus::Filled,
            3 => ExecStatus::Cancelled,
            _ => ExecStatus::Rejected,
        };
        let report = ExecutionReport {
            order_id,
            instrument_id,
            status,
            filled_qty,
            avg_price_ticks,
            latency_ns: 0,
        };
        let encoded = encode_to_vec(&report).expect("zero latency ExecutionReport encode failed");
        let (decoded, _): (ExecutionReport, usize) =
            decode_from_slice(&encoded).expect("zero latency ExecutionReport decode failed");
        prop_assert_eq!(report, decoded);
    }

    #[test]
    fn test_max_price_ticks_tick_data_roundtrip(
        instrument_id in any::<u32>(),
        quantity in any::<u32>(),
        timestamp_ns in any::<u64>(),
        side_idx in 0u8..2u8,
    ) {
        let side = match side_idx {
            0 => TickSide::Bid,
            _ => TickSide::Ask,
        };
        let tick = TickData {
            instrument_id,
            side,
            price_ticks: u64::MAX,
            quantity,
            timestamp_ns,
        };
        let encoded = encode_to_vec(&tick).expect("max price_ticks TickData encode failed");
        let (decoded, _): (TickData, usize) =
            decode_from_slice(&encoded).expect("max price_ticks TickData decode failed");
        prop_assert_eq!(tick, decoded);
    }
}
