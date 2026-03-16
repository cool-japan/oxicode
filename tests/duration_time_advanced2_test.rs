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

// ===== Test 1: Duration::from_secs(0) roundtrip =====

#[test]
fn test_duration_from_secs_zero_roundtrip() {
    let val = Duration::from_secs(0);
    let enc = encode_to_vec(&val).expect("encode Duration::from_secs(0)");
    let (dec, _): (Duration, usize) =
        decode_from_slice(&enc).expect("decode Duration::from_secs(0)");
    assert_eq!(val, dec);
}

// ===== Test 2: Duration::from_secs(3600) roundtrip =====

#[test]
fn test_duration_from_secs_3600_roundtrip() {
    let val = Duration::from_secs(3600);
    let enc = encode_to_vec(&val).expect("encode Duration::from_secs(3600)");
    let (dec, _): (Duration, usize) =
        decode_from_slice(&enc).expect("decode Duration::from_secs(3600)");
    assert_eq!(val, dec);
    assert_eq!(dec.as_secs(), 3600);
    assert_eq!(dec.subsec_nanos(), 0);
}

// ===== Test 3: Duration::from_millis(1500) roundtrip =====

#[test]
fn test_duration_from_millis_1500_roundtrip() {
    let val = Duration::from_millis(1500);
    let enc = encode_to_vec(&val).expect("encode Duration::from_millis(1500)");
    let (dec, _): (Duration, usize) =
        decode_from_slice(&enc).expect("decode Duration::from_millis(1500)");
    assert_eq!(val, dec);
    assert_eq!(dec.as_secs(), 1);
    assert_eq!(dec.subsec_millis(), 500);
}

// ===== Test 4: Duration::from_nanos(999_999_999) roundtrip =====

#[test]
fn test_duration_from_nanos_999999999_roundtrip() {
    let val = Duration::from_nanos(999_999_999);
    let enc = encode_to_vec(&val).expect("encode Duration::from_nanos(999_999_999)");
    let (dec, _): (Duration, usize) =
        decode_from_slice(&enc).expect("decode Duration::from_nanos(999_999_999)");
    assert_eq!(val, dec);
    assert_eq!(dec.as_secs(), 0);
    assert_eq!(dec.subsec_nanos(), 999_999_999);
}

// ===== Test 5: Duration::MAX roundtrip =====

#[test]
fn test_duration_max_roundtrip() {
    let val = Duration::MAX;
    let enc = encode_to_vec(&val).expect("encode Duration::MAX");
    let (dec, _): (Duration, usize) = decode_from_slice(&enc).expect("decode Duration::MAX");
    assert_eq!(val, dec);
    assert_eq!(dec.as_secs(), u64::MAX);
    assert_eq!(dec.subsec_nanos(), 999_999_999);
}

// ===== Test 6: Duration::ZERO roundtrip =====

#[test]
fn test_duration_zero_roundtrip() {
    let val = Duration::ZERO;
    let enc = encode_to_vec(&val).expect("encode Duration::ZERO");
    let (dec, _): (Duration, usize) = decode_from_slice(&enc).expect("decode Duration::ZERO");
    assert_eq!(val, dec);
}

// ===== Test 7: Duration::from_secs_f64(1.5) roundtrip (bit-exact via secs+nanos) =====

#[test]
fn test_duration_from_secs_f64_1_5_roundtrip() {
    let val = Duration::from_secs_f64(1.5);
    assert_eq!(val.as_secs(), 1);
    assert_eq!(val.subsec_nanos(), 500_000_000);
    let enc = encode_to_vec(&val).expect("encode Duration::from_secs_f64(1.5)");
    let (dec, _): (Duration, usize) =
        decode_from_slice(&enc).expect("decode Duration::from_secs_f64(1.5)");
    // Compare via secs and nanos to ensure bit-exact roundtrip
    assert_eq!(val.as_secs(), dec.as_secs());
    assert_eq!(val.subsec_nanos(), dec.subsec_nanos());
}

// ===== Test 8: Vec<Duration> roundtrip =====

#[test]
fn test_vec_duration_roundtrip() {
    let val: Vec<Duration> = vec![
        Duration::ZERO,
        Duration::from_secs(1),
        Duration::from_millis(250),
        Duration::from_nanos(123_456_789),
        Duration::from_secs(86400),
    ];
    let enc = encode_to_vec(&val).expect("encode Vec<Duration>");
    let (dec, _): (Vec<Duration>, usize) = decode_from_slice(&enc).expect("decode Vec<Duration>");
    assert_eq!(val, dec);
}

// ===== Test 9: Option<Duration> Some roundtrip =====

#[test]
fn test_option_duration_some_roundtrip() {
    let val: Option<Duration> = Some(Duration::from_secs(42));
    let enc = encode_to_vec(&val).expect("encode Option<Duration> Some");
    let (dec, _): (Option<Duration>, usize) =
        decode_from_slice(&enc).expect("decode Option<Duration> Some");
    assert_eq!(val, dec);
}

// ===== Test 10: Option<Duration> None roundtrip =====

#[test]
fn test_option_duration_none_roundtrip() {
    let val: Option<Duration> = None;
    let enc = encode_to_vec(&val).expect("encode Option<Duration> None");
    let (dec, _): (Option<Duration>, usize) =
        decode_from_slice(&enc).expect("decode Option<Duration> None");
    assert_eq!(val, dec);
}

// ===== Test 11: Struct with Duration field roundtrip =====

#[derive(Debug, PartialEq, Encode, Decode)]
struct ScheduledJob {
    id: u32,
    timeout: Duration,
    retries: u8,
}

#[test]
fn test_struct_with_duration_field_roundtrip() {
    let val = ScheduledJob {
        id: 7,
        timeout: Duration::from_millis(750),
        retries: 3,
    };
    let enc = encode_to_vec(&val).expect("encode ScheduledJob");
    let (dec, _): (ScheduledJob, usize) = decode_from_slice(&enc).expect("decode ScheduledJob");
    assert_eq!(val, dec);
}

// ===== Test 12: Duration with fixed_int_encoding config roundtrip =====

#[test]
fn test_duration_fixed_int_encoding_config_roundtrip() {
    let val = Duration::from_secs(12345);
    let cfg = config::standard().with_fixed_int_encoding();
    let enc =
        encode_to_vec_with_config(&val, cfg).expect("encode Duration with fixed_int_encoding");
    let (dec, _): (Duration, usize) =
        decode_from_slice_with_config(&enc, cfg).expect("decode Duration with fixed_int_encoding");
    assert_eq!(val, dec);
}

// ===== Test 13: Consumed bytes == encoded length for Duration =====

#[test]
fn test_duration_consumed_bytes_equals_encoded_len() {
    let val = Duration::from_secs(9999);
    let enc = encode_to_vec(&val).expect("encode Duration for consumed bytes check");
    let (_, consumed): (Duration, usize) =
        decode_from_slice(&enc).expect("decode Duration for consumed bytes check");
    assert_eq!(
        consumed,
        enc.len(),
        "consumed bytes must equal total encoded length"
    );
}

// ===== Test 14: Two different Duration values produce different encodings =====

#[test]
fn test_different_durations_produce_different_encodings() {
    let val_a = Duration::from_secs(1);
    let val_b = Duration::from_secs(2);
    let enc_a = encode_to_vec(&val_a).expect("encode Duration 1s");
    let enc_b = encode_to_vec(&val_b).expect("encode Duration 2s");
    assert_ne!(
        enc_a, enc_b,
        "different Duration values must produce different encodings"
    );
}

// ===== Test 15: SystemTime::UNIX_EPOCH roundtrip =====

#[test]
fn test_systemtime_unix_epoch_roundtrip() {
    let val = UNIX_EPOCH;
    let enc = encode_to_vec(&val).expect("encode SystemTime::UNIX_EPOCH");
    let (dec, _): (SystemTime, usize) =
        decode_from_slice(&enc).expect("decode SystemTime::UNIX_EPOCH");
    assert_eq!(val, dec);
}

// ===== Test 16: SystemTime after UNIX_EPOCH (fixed timestamp) roundtrip =====

#[test]
fn test_systemtime_after_epoch_fixed_roundtrip() {
    // Fixed timestamp: 2001-09-09T01:46:40Z (1_000_000_000 seconds after epoch)
    let val = UNIX_EPOCH + Duration::from_secs(1_000_000_000);
    let enc = encode_to_vec(&val).expect("encode SystemTime after epoch");
    let (dec, _): (SystemTime, usize) =
        decode_from_slice(&enc).expect("decode SystemTime after epoch");
    assert_eq!(val, dec);
    let elapsed = dec
        .duration_since(UNIX_EPOCH)
        .expect("duration_since UNIX_EPOCH");
    assert_eq!(elapsed.as_secs(), 1_000_000_000);
}

// ===== Test 17: SystemTime — use post-epoch only, verify 1s after epoch =====

#[test]
fn test_systemtime_post_epoch_one_second_roundtrip() {
    // Use post-epoch time only to avoid platform-specific pre-epoch issues
    let val = UNIX_EPOCH + Duration::from_secs(1);
    let enc = encode_to_vec(&val).expect("encode SystemTime 1s after epoch");
    let (dec, _): (SystemTime, usize) =
        decode_from_slice(&enc).expect("decode SystemTime 1s after epoch");
    assert_eq!(val, dec);
    let dur = dec
        .duration_since(UNIX_EPOCH)
        .expect("duration_since UNIX_EPOCH");
    assert_eq!(dur, Duration::from_secs(1));
}

// ===== Test 18: Vec<SystemTime> roundtrip with post-epoch times =====

#[test]
fn test_vec_systemtime_post_epoch_roundtrip() {
    let val: Vec<SystemTime> = vec![
        UNIX_EPOCH,
        UNIX_EPOCH + Duration::from_secs(1_000),
        UNIX_EPOCH + Duration::from_secs(1_000_000),
        UNIX_EPOCH + Duration::from_secs(1_700_000_000),
    ];
    let enc = encode_to_vec(&val).expect("encode Vec<SystemTime>");
    let (dec, _): (Vec<SystemTime>, usize) =
        decode_from_slice(&enc).expect("decode Vec<SystemTime>");
    assert_eq!(val, dec);
}

// ===== Test 19: Option<SystemTime> Some roundtrip =====

#[test]
fn test_option_systemtime_some_roundtrip() {
    let val: Option<SystemTime> = Some(UNIX_EPOCH + Duration::from_secs(500_000));
    let enc = encode_to_vec(&val).expect("encode Option<SystemTime> Some");
    let (dec, _): (Option<SystemTime>, usize) =
        decode_from_slice(&enc).expect("decode Option<SystemTime> Some");
    assert_eq!(val, dec);
}

// ===== Test 20: Struct with both Duration and SystemTime fields roundtrip =====

#[derive(Debug, PartialEq, Encode, Decode)]
struct ScheduledTask {
    id: u64,
    scheduled_at: SystemTime,
    timeout: Duration,
    priority: u8,
}

#[test]
fn test_struct_with_duration_and_systemtime_roundtrip() {
    let val = ScheduledTask {
        id: 42,
        scheduled_at: UNIX_EPOCH + Duration::from_secs(1_700_000_000),
        timeout: Duration::from_secs(30),
        priority: 5,
    };
    let enc = encode_to_vec(&val).expect("encode ScheduledTask");
    let (dec, _): (ScheduledTask, usize) = decode_from_slice(&enc).expect("decode ScheduledTask");
    assert_eq!(val, dec);
}

// ===== Test 21: Duration::from_secs(1) + Duration::from_secs(1) encodes same as Duration::from_secs(2) =====

#[test]
fn test_duration_addition_encodes_same_as_direct() {
    let added = Duration::from_secs(1) + Duration::from_secs(1);
    let direct = Duration::from_secs(2);
    assert_eq!(added, direct, "1s + 1s should equal 2s");
    let enc_added = encode_to_vec(&added).expect("encode added Duration");
    let enc_direct = encode_to_vec(&direct).expect("encode direct Duration");
    assert_eq!(
        enc_added, enc_direct,
        "1s+1s and 2s must produce identical encodings"
    );
}

// ===== Test 22: SystemTime consumed bytes == encoded length =====

#[test]
fn test_systemtime_consumed_bytes_equals_encoded_len() {
    // 2021-01-01T00:00:00Z
    let val = UNIX_EPOCH + Duration::from_secs(1_609_459_200);
    let enc = encode_to_vec(&val).expect("encode SystemTime for consumed bytes check");
    let (_, consumed): (SystemTime, usize) =
        decode_from_slice(&enc).expect("decode SystemTime for consumed bytes check");
    assert_eq!(
        consumed,
        enc.len(),
        "consumed bytes must equal total encoded length for SystemTime"
    );
}
