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
    let d = Duration::ZERO;
    let enc = encode_to_vec(&d).expect("encode Duration::ZERO");
    let (decoded, _): (Duration, usize) = decode_from_slice(&enc).expect("decode Duration::ZERO");
    assert_eq!(d, decoded);
}

#[test]
fn test_duration_one_second_roundtrip() {
    let d = Duration::from_secs(1);
    let enc = encode_to_vec(&d).expect("encode Duration::from_secs(1)");
    let (decoded, _): (Duration, usize) =
        decode_from_slice(&enc).expect("decode Duration::from_secs(1)");
    assert_eq!(d, decoded);
}

#[test]
fn test_duration_one_hour_roundtrip() {
    let d = Duration::from_secs(3600);
    let enc = encode_to_vec(&d).expect("encode Duration 1 hour");
    let (decoded, _): (Duration, usize) = decode_from_slice(&enc).expect("decode Duration 1 hour");
    assert_eq!(d, decoded);
}

#[test]
fn test_duration_half_second_millis_roundtrip() {
    let d = Duration::from_millis(500);
    let enc = encode_to_vec(&d).expect("encode Duration::from_millis(500)");
    let (decoded, _): (Duration, usize) =
        decode_from_slice(&enc).expect("decode Duration::from_millis(500)");
    assert_eq!(d, decoded);
}

#[test]
fn test_duration_one_second_nanos_roundtrip() {
    let d = Duration::from_nanos(1_000_000_000);
    let enc = encode_to_vec(&d).expect("encode Duration::from_nanos(1_000_000_000)");
    let (decoded, _): (Duration, usize) =
        decode_from_slice(&enc).expect("decode Duration::from_nanos(1_000_000_000)");
    assert_eq!(d, decoded);
}

#[test]
fn test_duration_max_roundtrip() {
    let d = Duration::MAX;
    let enc = encode_to_vec(&d).expect("encode Duration::MAX");
    let (decoded, _): (Duration, usize) = decode_from_slice(&enc).expect("decode Duration::MAX");
    assert_eq!(d, decoded);
}

#[test]
fn test_duration_one_year_roundtrip() {
    let d = Duration::from_secs(86400 * 365);
    let enc = encode_to_vec(&d).expect("encode Duration 1 year");
    let (decoded, _): (Duration, usize) = decode_from_slice(&enc).expect("decode Duration 1 year");
    assert_eq!(d, decoded);
}

#[test]
fn test_duration_consumed_bytes_equals_encoded_len() {
    let d = Duration::from_secs(42);
    let enc = encode_to_vec(&d).expect("encode Duration for bytes check");
    let (_, consumed): (Duration, usize) =
        decode_from_slice(&enc).expect("decode Duration for bytes check");
    assert_eq!(consumed, enc.len());
}

#[test]
fn test_two_different_durations_produce_different_encodings() {
    let d1 = Duration::from_secs(1);
    let d2 = Duration::from_secs(2);
    let enc1 = encode_to_vec(&d1).expect("encode Duration d1");
    let enc2 = encode_to_vec(&d2).expect("encode Duration d2");
    assert_ne!(enc1, enc2);
}

#[test]
fn test_duration_fixed_int_config_roundtrip() {
    let d = Duration::from_secs(100);
    let cfg = config::standard().with_fixed_int_encoding();
    let enc = encode_to_vec_with_config(&d, cfg).expect("encode Duration fixed int config");
    let (decoded, _): (Duration, usize) =
        decode_from_slice_with_config(&enc, cfg).expect("decode Duration fixed int config");
    assert_eq!(d, decoded);
}

#[test]
fn test_vec_of_five_durations_roundtrip() {
    let durations: Vec<Duration> = vec![
        Duration::from_secs(0),
        Duration::from_millis(250),
        Duration::from_secs(60),
        Duration::from_nanos(123_456_789),
        Duration::from_secs(86400),
    ];
    let enc = encode_to_vec(&durations).expect("encode Vec<Duration> 5 items");
    let (decoded, _): (Vec<Duration>, usize) =
        decode_from_slice(&enc).expect("decode Vec<Duration> 5 items");
    assert_eq!(durations, decoded);
}

#[test]
fn test_option_duration_some_roundtrip() {
    let d: Option<Duration> = Some(Duration::from_secs(7));
    let enc = encode_to_vec(&d).expect("encode Option<Duration> Some");
    let (decoded, _): (Option<Duration>, usize) =
        decode_from_slice(&enc).expect("decode Option<Duration> Some");
    assert_eq!(d, decoded);
}

#[test]
fn test_option_duration_none_roundtrip() {
    let d: Option<Duration> = None;
    let enc = encode_to_vec(&d).expect("encode Option<Duration> None");
    let (decoded, _): (Option<Duration>, usize) =
        decode_from_slice(&enc).expect("decode Option<Duration> None");
    assert_eq!(d, decoded);
}

#[test]
fn test_systemtime_unix_epoch_roundtrip() {
    let t = UNIX_EPOCH;
    let enc = encode_to_vec(&t).expect("encode SystemTime::UNIX_EPOCH");
    let (decoded, _): (SystemTime, usize) =
        decode_from_slice(&enc).expect("decode SystemTime::UNIX_EPOCH");
    assert_eq!(t, decoded);
}

#[test]
fn test_systemtime_one_billion_seconds_roundtrip() {
    let t = UNIX_EPOCH + Duration::from_secs(1_000_000_000);
    let enc = encode_to_vec(&t).expect("encode SystemTime 1e9 secs");
    let (decoded, _): (SystemTime, usize) =
        decode_from_slice(&enc).expect("decode SystemTime 1e9 secs");
    assert_eq!(t, decoded);
}

#[test]
fn test_systemtime_consumed_bytes_equals_encoded_len() {
    let t = UNIX_EPOCH + Duration::from_secs(500_000);
    let enc = encode_to_vec(&t).expect("encode SystemTime for bytes check");
    let (_, consumed): (SystemTime, usize) =
        decode_from_slice(&enc).expect("decode SystemTime for bytes check");
    assert_eq!(consumed, enc.len());
}

#[test]
fn test_vec_of_three_systemtimes_roundtrip() {
    let times: Vec<SystemTime> = vec![
        UNIX_EPOCH,
        UNIX_EPOCH + Duration::from_secs(1_000),
        UNIX_EPOCH + Duration::from_secs(1_000_000),
    ];
    let enc = encode_to_vec(&times).expect("encode Vec<SystemTime> 3 items");
    let (decoded, _): (Vec<SystemTime>, usize) =
        decode_from_slice(&enc).expect("decode Vec<SystemTime> 3 items");
    assert_eq!(times, decoded);
}

#[test]
fn test_option_systemtime_some_roundtrip() {
    let t: Option<SystemTime> = Some(UNIX_EPOCH + Duration::from_secs(9_999));
    let enc = encode_to_vec(&t).expect("encode Option<SystemTime> Some");
    let (decoded, _): (Option<SystemTime>, usize) =
        decode_from_slice(&enc).expect("decode Option<SystemTime> Some");
    assert_eq!(t, decoded);
}

#[test]
fn test_duration_micros_roundtrip() {
    let d = Duration::from_micros(12345);
    let enc = encode_to_vec(&d).expect("encode Duration::from_micros(12345)");
    let (decoded, _): (Duration, usize) =
        decode_from_slice(&enc).expect("decode Duration::from_micros(12345)");
    assert_eq!(d, decoded);
}

#[test]
fn test_duration_zero_secs_nonzero_nanos_roundtrip() {
    let d = Duration::from_nanos(999);
    let enc = encode_to_vec(&d).expect("encode Duration::from_nanos(999)");
    let (decoded, _): (Duration, usize) =
        decode_from_slice(&enc).expect("decode Duration::from_nanos(999)");
    assert_eq!(d, decoded);
}

#[test]
fn test_duration_both_secs_and_nanos_roundtrip() {
    let d = Duration::new(5, 500_000_000);
    let enc = encode_to_vec(&d).expect("encode Duration::new(5, 500_000_000)");
    let (decoded, _): (Duration, usize) =
        decode_from_slice(&enc).expect("decode Duration::new(5, 500_000_000)");
    assert_eq!(d, decoded);
}

#[test]
fn test_large_vec_duration_100_items_roundtrip() {
    let durations: Vec<Duration> = (0u64..100).map(|i| Duration::from_secs(i * 1000)).collect();
    let enc = encode_to_vec(&durations).expect("encode Vec<Duration> 100 items");
    let (decoded, _): (Vec<Duration>, usize) =
        decode_from_slice(&enc).expect("decode Vec<Duration> 100 items");
    assert_eq!(durations, decoded);
}
