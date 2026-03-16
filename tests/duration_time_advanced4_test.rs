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
    encode_to_vec_with_config,
};
use std::time::{Duration, SystemTime, UNIX_EPOCH};

#[test]
fn test_duration_zero_roundtrip() {
    let val = Duration::ZERO;
    let bytes = encode_to_vec(&val).expect("Failed to encode Duration::ZERO");
    let (decoded, _): (Duration, usize) =
        decode_from_slice(&bytes).expect("Failed to decode Duration::ZERO");
    assert_eq!(val, decoded);
}

#[test]
fn test_duration_one_second_roundtrip() {
    let val = Duration::from_secs(1);
    let bytes = encode_to_vec(&val).expect("Failed to encode Duration::from_secs(1)");
    let (decoded, _): (Duration, usize) =
        decode_from_slice(&bytes).expect("Failed to decode Duration::from_secs(1)");
    assert_eq!(val, decoded);
}

#[test]
fn test_duration_with_nanos_roundtrip() {
    let val = Duration::new(1, 500_000_000);
    let bytes = encode_to_vec(&val).expect("Failed to encode Duration::new(1, 500_000_000)");
    let (decoded, _): (Duration, usize) =
        decode_from_slice(&bytes).expect("Failed to decode Duration::new(1, 500_000_000)");
    assert_eq!(val, decoded);
}

#[test]
fn test_duration_max_roundtrip() {
    let val = Duration::MAX;
    let bytes = encode_to_vec(&val).expect("Failed to encode Duration::MAX");
    let (decoded, _): (Duration, usize) =
        decode_from_slice(&bytes).expect("Failed to decode Duration::MAX");
    assert_eq!(val, decoded);
}

#[test]
fn test_duration_from_millis_1000_roundtrip() {
    let val = Duration::from_millis(1000);
    let bytes = encode_to_vec(&val).expect("Failed to encode Duration::from_millis(1000)");
    let (decoded, _): (Duration, usize) =
        decode_from_slice(&bytes).expect("Failed to decode Duration::from_millis(1000)");
    assert_eq!(val, decoded);
}

#[test]
fn test_duration_from_micros_roundtrip() {
    let val = Duration::from_micros(1_500_000);
    let bytes = encode_to_vec(&val).expect("Failed to encode Duration::from_micros(1_500_000)");
    let (decoded, _): (Duration, usize) =
        decode_from_slice(&bytes).expect("Failed to decode Duration::from_micros(1_500_000)");
    assert_eq!(val, decoded);
}

#[test]
fn test_duration_from_nanos_roundtrip() {
    let val = Duration::from_nanos(1_000_000_000);
    let bytes = encode_to_vec(&val).expect("Failed to encode Duration::from_nanos(1_000_000_000)");
    let (decoded, _): (Duration, usize) =
        decode_from_slice(&bytes).expect("Failed to decode Duration::from_nanos(1_000_000_000)");
    assert_eq!(val, decoded);
}

#[test]
fn test_systemtime_unix_epoch_roundtrip() {
    let val = UNIX_EPOCH;
    let bytes = encode_to_vec(&val).expect("Failed to encode UNIX_EPOCH");
    let (decoded, _): (SystemTime, usize) =
        decode_from_slice(&bytes).expect("Failed to decode UNIX_EPOCH");
    assert_eq!(val, decoded);
}

#[test]
fn test_systemtime_unix_epoch_plus_1s_roundtrip() {
    let val = UNIX_EPOCH + Duration::from_secs(1);
    let bytes = encode_to_vec(&val).expect("Failed to encode UNIX_EPOCH + 1s");
    let (decoded, _): (SystemTime, usize) =
        decode_from_slice(&bytes).expect("Failed to decode UNIX_EPOCH + 1s");
    assert_eq!(val, decoded);
}

#[test]
fn test_systemtime_known_duration_roundtrip() {
    let known_duration = Duration::from_secs(1_700_000_000);
    let val = UNIX_EPOCH + known_duration;
    let bytes = encode_to_vec(&val).expect("Failed to encode SystemTime with known duration");
    let (decoded, _): (SystemTime, usize) =
        decode_from_slice(&bytes).expect("Failed to decode SystemTime with known duration");
    assert_eq!(val, decoded);
}

#[test]
fn test_vec_duration_roundtrip() {
    let val: Vec<Duration> = vec![
        Duration::from_secs(0),
        Duration::from_secs(1),
        Duration::from_millis(500),
        Duration::from_nanos(999_999_999),
    ];
    let bytes = encode_to_vec(&val).expect("Failed to encode Vec<Duration>");
    let (decoded, _): (Vec<Duration>, usize) =
        decode_from_slice(&bytes).expect("Failed to decode Vec<Duration>");
    assert_eq!(val, decoded);
}

#[test]
fn test_vec_systemtime_roundtrip() {
    let val: Vec<SystemTime> = vec![
        UNIX_EPOCH,
        UNIX_EPOCH + Duration::from_secs(86400),
        UNIX_EPOCH + Duration::from_secs(1_000_000_000),
    ];
    let bytes = encode_to_vec(&val).expect("Failed to encode Vec<SystemTime>");
    let (decoded, _): (Vec<SystemTime>, usize) =
        decode_from_slice(&bytes).expect("Failed to decode Vec<SystemTime>");
    assert_eq!(val, decoded);
}

#[test]
fn test_option_duration_some_roundtrip() {
    let val: Option<Duration> = Some(Duration::from_secs(42));
    let bytes = encode_to_vec(&val).expect("Failed to encode Option<Duration> Some");
    let (decoded, _): (Option<Duration>, usize) =
        decode_from_slice(&bytes).expect("Failed to decode Option<Duration> Some");
    assert_eq!(val, decoded);
}

#[test]
fn test_option_duration_none_roundtrip() {
    let val: Option<Duration> = None;
    let bytes = encode_to_vec(&val).expect("Failed to encode Option<Duration> None");
    let (decoded, _): (Option<Duration>, usize) =
        decode_from_slice(&bytes).expect("Failed to decode Option<Duration> None");
    assert_eq!(val, decoded);
}

#[test]
fn test_option_systemtime_some_roundtrip() {
    let val: Option<SystemTime> = Some(UNIX_EPOCH + Duration::from_secs(100));
    let bytes = encode_to_vec(&val).expect("Failed to encode Option<SystemTime> Some");
    let (decoded, _): (Option<SystemTime>, usize) =
        decode_from_slice(&bytes).expect("Failed to decode Option<SystemTime> Some");
    assert_eq!(val, decoded);
}

#[test]
fn test_option_systemtime_none_roundtrip() {
    let val: Option<SystemTime> = None;
    let bytes = encode_to_vec(&val).expect("Failed to encode Option<SystemTime> None");
    let (decoded, _): (Option<SystemTime>, usize) =
        decode_from_slice(&bytes).expect("Failed to decode Option<SystemTime> None");
    assert_eq!(val, decoded);
}

#[test]
fn test_duration_with_fixed_int_config() {
    let cfg = config::standard().with_fixed_int_encoding();
    let val = Duration::from_secs(7);
    let bytes = encode_to_vec_with_config(&val, cfg)
        .expect("Failed to encode Duration with fixed-int config");
    let (decoded, _): (Duration, usize) = decode_from_slice_with_config(&bytes, cfg)
        .expect("Failed to decode Duration with fixed-int config");
    assert_eq!(val, decoded);
}

#[test]
fn test_duration_consumed_bytes_equals_encoded_length() {
    let val = Duration::from_secs(123);
    let bytes = encode_to_vec(&val).expect("Failed to encode Duration for consumed bytes test");
    let (_, consumed): (Duration, usize) =
        decode_from_slice(&bytes).expect("Failed to decode Duration for consumed bytes test");
    assert_eq!(consumed, bytes.len());
}

#[test]
fn test_two_durations_same_value_produce_same_bytes() {
    let val_a = Duration::from_secs(999);
    let val_b = Duration::from_secs(999);
    let bytes_a = encode_to_vec(&val_a).expect("Failed to encode val_a");
    let bytes_b = encode_to_vec(&val_b).expect("Failed to encode val_b");
    assert_eq!(bytes_a, bytes_b);
}

#[test]
fn test_durations_different_values_produce_different_bytes() {
    let val_a = Duration::from_secs(1);
    let val_b = Duration::from_secs(2);
    let bytes_a = encode_to_vec(&val_a).expect("Failed to encode val_a");
    let bytes_b = encode_to_vec(&val_b).expect("Failed to encode val_b");
    assert_ne!(bytes_a, bytes_b);
}

#[test]
fn test_systemtime_consumed_bytes_equals_encoded_length() {
    let val = UNIX_EPOCH + Duration::from_secs(500_000);
    let bytes = encode_to_vec(&val).expect("Failed to encode SystemTime for consumed bytes test");
    let (_, consumed): (SystemTime, usize) =
        decode_from_slice(&bytes).expect("Failed to decode SystemTime for consumed bytes test");
    assert_eq!(consumed, bytes.len());
}

#[test]
fn test_large_duration_many_days_roundtrip() {
    let val = Duration::from_secs(365 * 24 * 3600 * 100);
    let bytes = encode_to_vec(&val).expect("Failed to encode large Duration (100 years)");
    let (decoded, _): (Duration, usize) =
        decode_from_slice(&bytes).expect("Failed to decode large Duration (100 years)");
    assert_eq!(val, decoded);
}
