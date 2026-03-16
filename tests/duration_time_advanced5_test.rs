#![cfg(feature = "std")]
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
use std::time::{Duration, SystemTime, UNIX_EPOCH};

#[derive(Debug, PartialEq, Encode, Decode)]
struct TimedEvent {
    name: String,
    timestamp: SystemTime,
    duration: Duration,
    retries: u32,
}

// Test 1: Duration::ZERO roundtrip
#[test]
fn test_duration_zero_roundtrip() {
    let val = Duration::ZERO;
    let bytes = encode_to_vec(&val).expect("Failed to encode Duration::ZERO");
    let (decoded, _): (Duration, usize) =
        decode_from_slice(&bytes).expect("Failed to decode Duration::ZERO");
    assert_eq!(val, decoded);
}

// Test 2: Duration::from_secs(1) roundtrip
#[test]
fn test_duration_from_secs_1_roundtrip() {
    let val = Duration::from_secs(1);
    let bytes = encode_to_vec(&val).expect("Failed to encode Duration::from_secs(1)");
    let (decoded, _): (Duration, usize) =
        decode_from_slice(&bytes).expect("Failed to decode Duration::from_secs(1)");
    assert_eq!(val, decoded);
}

// Test 3: Duration::from_millis(500) roundtrip
#[test]
fn test_duration_from_millis_500_roundtrip() {
    let val = Duration::from_millis(500);
    let bytes = encode_to_vec(&val).expect("Failed to encode Duration::from_millis(500)");
    let (decoded, _): (Duration, usize) =
        decode_from_slice(&bytes).expect("Failed to decode Duration::from_millis(500)");
    assert_eq!(val, decoded);
}

// Test 4: Duration::from_nanos(1) roundtrip
#[test]
fn test_duration_from_nanos_1_roundtrip() {
    let val = Duration::from_nanos(1);
    let bytes = encode_to_vec(&val).expect("Failed to encode Duration::from_nanos(1)");
    let (decoded, _): (Duration, usize) =
        decode_from_slice(&bytes).expect("Failed to decode Duration::from_nanos(1)");
    assert_eq!(val, decoded);
}

// Test 5: Duration::MAX roundtrip
#[test]
fn test_duration_max_roundtrip() {
    let val = Duration::MAX;
    let bytes = encode_to_vec(&val).expect("Failed to encode Duration::MAX");
    let (decoded, _): (Duration, usize) =
        decode_from_slice(&bytes).expect("Failed to decode Duration::MAX");
    assert_eq!(val, decoded);
}

// Test 6: Duration::from_secs(3600 * 24 * 365) roundtrip (1 year)
#[test]
fn test_duration_one_year_roundtrip() {
    let val = Duration::from_secs(3600 * 24 * 365);
    let bytes = encode_to_vec(&val).expect("Failed to encode Duration of 1 year");
    let (decoded, _): (Duration, usize) =
        decode_from_slice(&bytes).expect("Failed to decode Duration of 1 year");
    assert_eq!(val, decoded);
}

// Test 7: Vec<Duration> roundtrip (5 items)
#[test]
fn test_vec_duration_5_items_roundtrip() {
    let val: Vec<Duration> = vec![
        Duration::ZERO,
        Duration::from_secs(1),
        Duration::from_millis(500),
        Duration::from_nanos(999_999_999),
        Duration::from_secs(86400),
    ];
    let bytes = encode_to_vec(&val).expect("Failed to encode Vec<Duration> with 5 items");
    let (decoded, _): (Vec<Duration>, usize) =
        decode_from_slice(&bytes).expect("Failed to decode Vec<Duration> with 5 items");
    assert_eq!(val, decoded);
}

// Test 8: Option<Duration> Some roundtrip
#[test]
fn test_option_duration_some_roundtrip() {
    let val: Option<Duration> = Some(Duration::from_secs(42));
    let bytes = encode_to_vec(&val).expect("Failed to encode Option<Duration> Some");
    let (decoded, _): (Option<Duration>, usize) =
        decode_from_slice(&bytes).expect("Failed to decode Option<Duration> Some");
    assert_eq!(val, decoded);
}

// Test 9: Option<Duration> None roundtrip
#[test]
fn test_option_duration_none_roundtrip() {
    let val: Option<Duration> = None;
    let bytes = encode_to_vec(&val).expect("Failed to encode Option<Duration> None");
    let (decoded, _): (Option<Duration>, usize) =
        decode_from_slice(&bytes).expect("Failed to decode Option<Duration> None");
    assert_eq!(val, decoded);
}

// Test 10: SystemTime::UNIX_EPOCH roundtrip
#[test]
fn test_systemtime_unix_epoch_roundtrip() {
    let val = UNIX_EPOCH;
    let bytes = encode_to_vec(&val).expect("Failed to encode SystemTime::UNIX_EPOCH");
    let (decoded, _): (SystemTime, usize) =
        decode_from_slice(&bytes).expect("Failed to decode SystemTime::UNIX_EPOCH");
    assert_eq!(val, decoded);
}

// Test 11: SystemTime after epoch (+1000 secs) roundtrip
#[test]
fn test_systemtime_after_epoch_1000_secs_roundtrip() {
    let val = UNIX_EPOCH + Duration::from_secs(1000);
    let bytes = encode_to_vec(&val).expect("Failed to encode SystemTime (epoch + 1000s)");
    let (decoded, _): (SystemTime, usize) =
        decode_from_slice(&bytes).expect("Failed to decode SystemTime (epoch + 1000s)");
    assert_eq!(val, decoded);
}

// Test 12: Vec<SystemTime> roundtrip (3 items)
#[test]
fn test_vec_systemtime_3_items_roundtrip() {
    let val: Vec<SystemTime> = vec![
        UNIX_EPOCH,
        UNIX_EPOCH + Duration::from_secs(86400),
        UNIX_EPOCH + Duration::from_secs(1_000_000_000),
    ];
    let bytes = encode_to_vec(&val).expect("Failed to encode Vec<SystemTime> with 3 items");
    let (decoded, _): (Vec<SystemTime>, usize) =
        decode_from_slice(&bytes).expect("Failed to decode Vec<SystemTime> with 3 items");
    assert_eq!(val, decoded);
}

// Test 13: TimedEvent roundtrip
#[test]
fn test_timed_event_roundtrip() {
    let val = TimedEvent {
        name: "event_alpha".to_string(),
        timestamp: UNIX_EPOCH + Duration::from_secs(1_700_000_000),
        duration: Duration::from_millis(250),
        retries: 3,
    };
    let bytes = encode_to_vec(&val).expect("Failed to encode TimedEvent");
    let (decoded, _): (TimedEvent, usize) =
        decode_from_slice(&bytes).expect("Failed to decode TimedEvent");
    assert_eq!(val, decoded);
}

// Test 14: TimedEvent with Duration::ZERO
#[test]
fn test_timed_event_with_duration_zero() {
    let val = TimedEvent {
        name: "instant_event".to_string(),
        timestamp: UNIX_EPOCH,
        duration: Duration::ZERO,
        retries: 0,
    };
    let bytes = encode_to_vec(&val).expect("Failed to encode TimedEvent with Duration::ZERO");
    let (decoded, _): (TimedEvent, usize) =
        decode_from_slice(&bytes).expect("Failed to decode TimedEvent with Duration::ZERO");
    assert_eq!(val, decoded);
}

// Test 15: Consumed bytes equals encoded length for Duration
#[test]
fn test_duration_consumed_bytes_equals_encoded_length() {
    let val = Duration::from_secs(123);
    let bytes = encode_to_vec(&val).expect("Failed to encode Duration for consumed bytes check");
    let (_, consumed): (Duration, usize) =
        decode_from_slice(&bytes).expect("Failed to decode Duration for consumed bytes check");
    assert_eq!(consumed, bytes.len());
}

// Test 16: Consumed bytes equals encoded length for SystemTime
#[test]
fn test_systemtime_consumed_bytes_equals_encoded_length() {
    let val = UNIX_EPOCH + Duration::from_secs(500_000);
    let bytes = encode_to_vec(&val).expect("Failed to encode SystemTime for consumed bytes check");
    let (_, consumed): (SystemTime, usize) =
        decode_from_slice(&bytes).expect("Failed to decode SystemTime for consumed bytes check");
    assert_eq!(consumed, bytes.len());
}

// Test 17: Duration fixed-int config roundtrip
#[test]
fn test_duration_fixed_int_config_roundtrip() {
    let cfg = config::standard().with_fixed_int_encoding();
    let val = Duration::from_secs(7);
    let bytes = encode_to_vec_with_config(&val, cfg)
        .expect("Failed to encode Duration with fixed-int config");
    let (decoded, _): (Duration, usize) = decode_from_slice_with_config(&bytes, cfg)
        .expect("Failed to decode Duration with fixed-int config");
    assert_eq!(val, decoded);
}

// Test 18: Duration and SystemTime produce different bytes for different values
#[test]
fn test_duration_and_systemtime_different_values_produce_different_bytes() {
    let dur_a = Duration::from_secs(1);
    let dur_b = Duration::from_secs(2);
    let bytes_dur_a = encode_to_vec(&dur_a).expect("Failed to encode dur_a");
    let bytes_dur_b = encode_to_vec(&dur_b).expect("Failed to encode dur_b");
    assert_ne!(
        bytes_dur_a, bytes_dur_b,
        "Different Duration values must produce different bytes"
    );

    let st_a = UNIX_EPOCH + Duration::from_secs(100);
    let st_b = UNIX_EPOCH + Duration::from_secs(200);
    let bytes_st_a = encode_to_vec(&st_a).expect("Failed to encode st_a");
    let bytes_st_b = encode_to_vec(&st_b).expect("Failed to encode st_b");
    assert_ne!(
        bytes_st_a, bytes_st_b,
        "Different SystemTime values must produce different bytes"
    );
}

// Test 19: Vec<TimedEvent> roundtrip (3 items)
#[test]
fn test_vec_timed_event_3_items_roundtrip() {
    let val: Vec<TimedEvent> = vec![
        TimedEvent {
            name: "first".to_string(),
            timestamp: UNIX_EPOCH,
            duration: Duration::from_secs(1),
            retries: 0,
        },
        TimedEvent {
            name: "second".to_string(),
            timestamp: UNIX_EPOCH + Duration::from_secs(60),
            duration: Duration::from_millis(750),
            retries: 1,
        },
        TimedEvent {
            name: "third".to_string(),
            timestamp: UNIX_EPOCH + Duration::from_secs(3600),
            duration: Duration::from_secs(30),
            retries: 5,
        },
    ];
    let bytes = encode_to_vec(&val).expect("Failed to encode Vec<TimedEvent> with 3 items");
    let (decoded, _): (Vec<TimedEvent>, usize) =
        decode_from_slice(&bytes).expect("Failed to decode Vec<TimedEvent> with 3 items");
    assert_eq!(val, decoded);
}

// Test 20: Option<SystemTime> Some roundtrip
#[test]
fn test_option_systemtime_some_roundtrip() {
    let val: Option<SystemTime> = Some(UNIX_EPOCH + Duration::from_secs(9999));
    let bytes = encode_to_vec(&val).expect("Failed to encode Option<SystemTime> Some");
    let (decoded, _): (Option<SystemTime>, usize) =
        decode_from_slice(&bytes).expect("Failed to decode Option<SystemTime> Some");
    assert_eq!(val, decoded);
}

// Test 21: Duration::from_micros(999999) roundtrip
#[test]
fn test_duration_from_micros_999999_roundtrip() {
    let val = Duration::from_micros(999_999);
    let bytes = encode_to_vec(&val).expect("Failed to encode Duration::from_micros(999999)");
    let (decoded, _): (Duration, usize) =
        decode_from_slice(&bytes).expect("Failed to decode Duration::from_micros(999999)");
    assert_eq!(val, decoded);
}

// Test 22: TimedEvent with very long name (500 chars)
#[test]
fn test_timed_event_with_very_long_name_roundtrip() {
    let long_name = "x".repeat(500);
    let val = TimedEvent {
        name: long_name,
        timestamp: UNIX_EPOCH + Duration::from_secs(1_600_000_000),
        duration: Duration::from_nanos(123_456_789),
        retries: 99,
    };
    let bytes = encode_to_vec(&val).expect("Failed to encode TimedEvent with 500-char name");
    let (decoded, _): (TimedEvent, usize) =
        decode_from_slice(&bytes).expect("Failed to decode TimedEvent with 500-char name");
    assert_eq!(val, decoded);
}
