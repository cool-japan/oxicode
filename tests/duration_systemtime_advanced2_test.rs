//! Advanced Duration and SystemTime encoding tests for OxiCode — batch 2.
//! 22 tests covering new angles: wire sizes, addition/composition, cross-type
//! structs, big-endian config, Option/Vec wrappers, and pre-epoch SystemTime.

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

// ===== Test 1: Duration::ZERO roundtrip with consumed == encoded.len() =====

#[test]
fn test_duration_zero_roundtrip() {
    let original = Duration::ZERO;
    let encoded = encode_to_vec(&original).expect("encode Duration::ZERO failed");
    let (decoded, consumed): (Duration, _) =
        decode_from_slice(&encoded).expect("decode Duration::ZERO failed");
    assert_eq!(decoded, Duration::ZERO);
    assert_eq!(consumed, encoded.len());
}

// ===== Test 2: Duration::MAX roundtrip =====

#[test]
fn test_duration_max_roundtrip() {
    let original = Duration::MAX;
    let encoded = encode_to_vec(&original).expect("encode Duration::MAX failed");
    let (decoded, consumed): (Duration, _) =
        decode_from_slice(&encoded).expect("decode Duration::MAX failed");
    assert_eq!(decoded, original);
    assert_eq!(consumed, encoded.len());
    assert_eq!(decoded.as_secs(), u64::MAX);
    assert_eq!(decoded.subsec_nanos(), 999_999_999);
}

// ===== Test 3: Duration::from_secs(1) roundtrip =====

#[test]
fn test_duration_from_secs_one_roundtrip() {
    let original = Duration::from_secs(1);
    let encoded = encode_to_vec(&original).expect("encode Duration::from_secs(1) failed");
    let (decoded, consumed): (Duration, _) =
        decode_from_slice(&encoded).expect("decode Duration::from_secs(1) failed");
    assert_eq!(decoded, original);
    assert_eq!(consumed, encoded.len());
    assert_eq!(decoded.as_secs(), 1);
    assert_eq!(decoded.subsec_nanos(), 0);
}

// ===== Test 4: Duration::from_millis(1000) == Duration::from_secs(1) after roundtrip =====

#[test]
fn test_duration_millis_1000_equals_secs_1_roundtrip() {
    let from_millis = Duration::from_millis(1000);
    let from_secs = Duration::from_secs(1);
    assert_eq!(from_millis, from_secs, "must be equal before encoding");

    let enc_millis = encode_to_vec(&from_millis).expect("encode from_millis(1000) failed");
    let enc_secs = encode_to_vec(&from_secs).expect("encode from_secs(1) failed");
    assert_eq!(
        enc_millis, enc_secs,
        "from_millis(1000) and from_secs(1) must produce identical encodings"
    );

    let (decoded, consumed): (Duration, _) = decode_from_slice(&enc_millis).expect("decode failed");
    assert_eq!(decoded, from_secs);
    assert_eq!(consumed, enc_millis.len());
}

// ===== Test 5: Duration::from_nanos(1) roundtrip (smallest nonzero unit) =====

#[test]
fn test_duration_nanos_smallest_unit() {
    let original = Duration::from_nanos(1);
    let encoded = encode_to_vec(&original).expect("encode Duration::from_nanos(1) failed");
    let (decoded, consumed): (Duration, _) =
        decode_from_slice(&encoded).expect("decode Duration::from_nanos(1) failed");
    assert_eq!(decoded, original);
    assert_eq!(consumed, encoded.len());
    assert_eq!(decoded.as_secs(), 0);
    assert_eq!(decoded.subsec_nanos(), 1);
}

// ===== Test 6: Duration::from_secs(3600) one hour roundtrip =====

#[test]
fn test_duration_one_hour_roundtrip() {
    let original = Duration::from_secs(3600);
    let encoded = encode_to_vec(&original).expect("encode 1h Duration failed");
    let (decoded, consumed): (Duration, _) =
        decode_from_slice(&encoded).expect("decode 1h Duration failed");
    assert_eq!(decoded, original);
    assert_eq!(consumed, encoded.len());
    assert_eq!(decoded.as_secs(), 3600);
}

// ===== Test 7: Duration::from_secs(86400) one day roundtrip =====

#[test]
fn test_duration_one_day_roundtrip() {
    let original = Duration::from_secs(86400);
    let encoded = encode_to_vec(&original).expect("encode 1-day Duration failed");
    let (decoded, consumed): (Duration, _) =
        decode_from_slice(&encoded).expect("decode 1-day Duration failed");
    assert_eq!(decoded, original);
    assert_eq!(consumed, encoded.len());
    assert_eq!(decoded.as_secs(), 86400);
}

// ===== Test 8: Duration::from_millis(500) sub-second roundtrip =====

#[test]
fn test_duration_sub_second_half_second() {
    let original = Duration::from_millis(500);
    let encoded = encode_to_vec(&original).expect("encode 500ms Duration failed");
    let (decoded, consumed): (Duration, _) =
        decode_from_slice(&encoded).expect("decode 500ms Duration failed");
    assert_eq!(decoded, original);
    assert_eq!(consumed, encoded.len());
    assert_eq!(decoded.as_secs(), 0);
    assert_eq!(decoded.subsec_millis(), 500);
    assert_eq!(decoded.subsec_nanos(), 500_000_000);
}

// ===== Test 9: Duration with both secs and nanos: 1s + 500_000_000ns = 1.5s =====

#[test]
fn test_duration_combined_secs_and_nanos() {
    let original = Duration::from_secs(1) + Duration::from_nanos(500_000_000);
    assert_eq!(original.as_secs(), 1);
    assert_eq!(original.subsec_nanos(), 500_000_000);

    let encoded = encode_to_vec(&original).expect("encode 1.5s Duration failed");
    let (decoded, consumed): (Duration, _) =
        decode_from_slice(&encoded).expect("decode 1.5s Duration failed");
    assert_eq!(decoded, original);
    assert_eq!(consumed, encoded.len());
    assert_eq!(decoded.as_secs(), 1);
    assert_eq!(decoded.subsec_nanos(), 500_000_000);
}

// ===== Test 10: Vec<Duration> roundtrip =====

#[test]
fn test_vec_duration_roundtrip() {
    let original: Vec<Duration> = vec![
        Duration::ZERO,
        Duration::from_nanos(1),
        Duration::from_millis(500),
        Duration::from_secs(3600),
        Duration::MAX,
    ];
    let encoded = encode_to_vec(&original).expect("encode Vec<Duration> failed");
    let (decoded, consumed): (Vec<Duration>, _) =
        decode_from_slice(&encoded).expect("decode Vec<Duration> failed");
    assert_eq!(decoded, original);
    assert_eq!(decoded.len(), 5);
    assert_eq!(consumed, encoded.len());
}

// ===== Test 11: Option<Duration> Some and None roundtrip =====

#[test]
fn test_option_duration_some_and_none_roundtrip() {
    let some_val: Option<Duration> = Some(Duration::from_secs(99));
    let none_val: Option<Duration> = None;

    let enc_some = encode_to_vec(&some_val).expect("encode Some(Duration) failed");
    let (dec_some, _): (Option<Duration>, _) =
        decode_from_slice(&enc_some).expect("decode Some(Duration) failed");
    assert_eq!(dec_some, some_val);

    let enc_none = encode_to_vec(&none_val).expect("encode None::<Duration> failed");
    let (dec_none, _): (Option<Duration>, _) =
        decode_from_slice(&enc_none).expect("decode None::<Duration> failed");
    assert_eq!(dec_none, none_val);

    assert_ne!(
        enc_some, enc_none,
        "Some and None must produce different byte sequences"
    );
}

// ===== Test 12: Duration byte size check with fixed-int config =====

#[test]
fn test_duration_encoded_byte_size_fixed_int() {
    // With fixed-int config: u64 secs (8 bytes) + u32 nanos (4 bytes) = 12 bytes exactly.
    let cfg = config::standard().with_fixed_int_encoding();
    let dur = Duration::from_secs(42);
    let mut buf = [0u8; 32];
    let written = oxicode::encode_into_slice(dur, &mut buf, cfg)
        .expect("encode Duration with fixed-int config failed");
    assert_eq!(
        written, 12,
        "Duration with fixed-int encoding must be exactly 12 bytes (u64 + u32)"
    );
}

// ===== Test 13: UNIX_EPOCH roundtrip =====

#[test]
fn test_unix_epoch_roundtrip() {
    let original = UNIX_EPOCH;
    let encoded = encode_to_vec(&original).expect("encode UNIX_EPOCH failed");
    let (decoded, consumed): (SystemTime, _) =
        decode_from_slice(&encoded).expect("decode UNIX_EPOCH failed");
    assert_eq!(decoded, original);
    assert_eq!(consumed, encoded.len());
    // Verify the decoded time is exactly epoch
    let since = decoded
        .duration_since(UNIX_EPOCH)
        .expect("decoded UNIX_EPOCH duration_since failed");
    assert_eq!(since, Duration::ZERO);
}

// ===== Test 14: SystemTime::now() roundtrip =====

#[test]
fn test_systemtime_now_roundtrip() {
    let before = SystemTime::now();
    let encoded = encode_to_vec(&before).expect("encode SystemTime::now() failed");
    let (decoded, _): (SystemTime, _) =
        decode_from_slice(&encoded).expect("decode SystemTime::now() failed");
    // Should be exact since we encode/decode the same instant
    assert_eq!(decoded, before);
}

// ===== Test 15: SystemTime from UNIX_EPOCH + Duration roundtrip =====

#[test]
fn test_systemtime_from_epoch_plus_duration_roundtrip() {
    let offset = Duration::new(1_609_459_200, 123_456_789); // 2021-01-01 + nanos
    let original = UNIX_EPOCH + offset;
    let encoded = encode_to_vec(&original).expect("encode SystemTime from epoch + duration failed");
    let (decoded, consumed): (SystemTime, _) =
        decode_from_slice(&encoded).expect("decode SystemTime from epoch + duration failed");
    assert_eq!(decoded, original);
    assert_eq!(consumed, encoded.len());

    let dec_since = decoded
        .duration_since(UNIX_EPOCH)
        .expect("decoded duration_since failed");
    assert_eq!(dec_since.as_secs(), 1_609_459_200);
    assert_eq!(dec_since.subsec_nanos(), 123_456_789);
}

// ===== Test 16: SystemTime before UNIX_EPOCH (pre-epoch) =====

#[test]
fn test_systemtime_pre_epoch_roundtrip() {
    // SystemTime before UNIX_EPOCH: UNIX_EPOCH - 1 second.
    // OxiCode encodes pre-epoch times with a negative offset variant.
    let original = UNIX_EPOCH - Duration::from_secs(1);
    let encoded =
        encode_to_vec(&original).expect("encode pre-epoch SystemTime failed — not supported?");
    let (decoded, consumed): (SystemTime, _) =
        decode_from_slice(&encoded).expect("decode pre-epoch SystemTime failed");
    assert_eq!(decoded, original);
    assert_eq!(consumed, encoded.len());
}

// ===== Test 17: Vec<SystemTime> roundtrip =====

#[test]
fn test_vec_systemtime_roundtrip() {
    let original: Vec<SystemTime> = vec![
        UNIX_EPOCH,
        UNIX_EPOCH + Duration::from_secs(1_000),
        UNIX_EPOCH + Duration::from_secs(1_000_000),
        UNIX_EPOCH + Duration::new(1_700_000_000, 500_000_000),
    ];
    let encoded = encode_to_vec(&original).expect("encode Vec<SystemTime> failed");
    let (decoded, consumed): (Vec<SystemTime>, _) =
        decode_from_slice(&encoded).expect("decode Vec<SystemTime> failed");
    assert_eq!(decoded, original);
    assert_eq!(decoded.len(), 4);
    assert_eq!(consumed, encoded.len());
}

// ===== Test 18: Option<SystemTime> Some and None roundtrip =====

#[test]
fn test_option_systemtime_some_and_none_roundtrip() {
    let some_val: Option<SystemTime> = Some(UNIX_EPOCH + Duration::from_secs(2_000_000_000));
    let none_val: Option<SystemTime> = None;

    let enc_some = encode_to_vec(&some_val).expect("encode Some(SystemTime) failed");
    let (dec_some, _): (Option<SystemTime>, _) =
        decode_from_slice(&enc_some).expect("decode Some(SystemTime) failed");
    assert_eq!(dec_some, some_val);

    let enc_none = encode_to_vec(&none_val).expect("encode None::<SystemTime> failed");
    let (dec_none, _): (Option<SystemTime>, _) =
        decode_from_slice(&enc_none).expect("decode None::<SystemTime> failed");
    assert_eq!(dec_none, none_val);

    assert_ne!(enc_some, enc_none, "Some and None must differ in encoding");
}

// ===== Test 19: Duration with big-endian + fixed-int config roundtrip =====

#[test]
fn test_duration_big_endian_config_roundtrip() {
    let cfg = config::standard()
        .with_big_endian()
        .with_fixed_int_encoding();
    let original = Duration::new(999, 888_777_666);

    let enc = encode_to_vec_with_config(&original, cfg).expect("encode Duration big-endian failed");
    // With fixed-int big-endian: 8 bytes for u64 secs + 4 bytes for u32 nanos = 12 bytes
    assert_eq!(enc.len(), 12);

    // Verify big-endian byte order for secs field
    let secs_be = u64::from_be_bytes(enc[..8].try_into().expect("secs slice failed"));
    assert_eq!(secs_be, 999u64);
    let nanos_be = u32::from_be_bytes(enc[8..12].try_into().expect("nanos slice failed"));
    assert_eq!(nanos_be, 888_777_666u32);

    let (decoded, consumed): (Duration, _) =
        decode_from_slice_with_config(&enc, cfg).expect("decode Duration big-endian failed");
    assert_eq!(decoded, original);
    assert_eq!(consumed, 12);
}

// ===== Test 20: SystemTime with big-endian + fixed-int config roundtrip =====

#[test]
fn test_systemtime_big_endian_config_roundtrip() {
    let cfg = config::standard()
        .with_big_endian()
        .with_fixed_int_encoding();
    let original = UNIX_EPOCH + Duration::from_secs(1_234_567_890);

    let enc =
        encode_to_vec_with_config(&original, cfg).expect("encode SystemTime big-endian failed");
    // Fixed-int SystemTime: i64 secs (8) + u32 nanos (4) = 12 bytes
    assert_eq!(enc.len(), 12);

    let (decoded, consumed): (SystemTime, _) =
        decode_from_slice_with_config(&enc, cfg).expect("decode SystemTime big-endian failed");
    assert_eq!(decoded, original);
    assert_eq!(consumed, 12);
}

// ===== Test 21: Struct with Duration and SystemTime fields roundtrip =====

#[derive(Debug, PartialEq, oxicode::Encode, oxicode::Decode)]
struct TimeRecord {
    created_at: SystemTime,
    duration: Duration,
    id: u64,
}

#[test]
fn test_struct_with_duration_and_systemtime_roundtrip() {
    let original = TimeRecord {
        created_at: UNIX_EPOCH + Duration::from_secs(1_700_000_000),
        duration: Duration::from_millis(42),
        id: 12345u64,
    };
    let encoded = encode_to_vec(&original).expect("encode TimeRecord failed");
    let (decoded, consumed): (TimeRecord, _) =
        decode_from_slice(&encoded).expect("decode TimeRecord failed");
    assert_eq!(decoded, original);
    assert_eq!(consumed, encoded.len());
    assert_eq!(decoded.id, 12345u64);
    assert_eq!(decoded.duration, Duration::from_millis(42));
}

// ===== Test 22: Duration comparison: two different durations decode as different values =====

#[test]
fn test_duration_different_values_decode_as_different() {
    let dur_a = Duration::from_nanos(1);
    let dur_b = Duration::from_nanos(2);
    let dur_c = Duration::from_secs(1);
    let dur_d = Duration::from_millis(1); // 1_000_000 ns

    let enc_a = encode_to_vec(&dur_a).expect("encode dur_a failed");
    let enc_b = encode_to_vec(&dur_b).expect("encode dur_b failed");
    let enc_c = encode_to_vec(&dur_c).expect("encode dur_c failed");
    let enc_d = encode_to_vec(&dur_d).expect("encode dur_d failed");

    // All four must encode differently from each other
    assert_ne!(enc_a, enc_b, "1ns and 2ns must differ");
    assert_ne!(enc_a, enc_c, "1ns and 1s must differ");
    assert_ne!(enc_b, enc_c, "2ns and 1s must differ");
    assert_ne!(enc_c, enc_d, "1s and 1ms must differ");

    // Verify decoded values are also distinct
    let (dec_a, _): (Duration, _) = decode_from_slice(&enc_a).expect("decode dur_a failed");
    let (dec_b, _): (Duration, _) = decode_from_slice(&enc_b).expect("decode dur_b failed");
    let (dec_c, _): (Duration, _) = decode_from_slice(&enc_c).expect("decode dur_c failed");
    let (dec_d, _): (Duration, _) = decode_from_slice(&enc_d).expect("decode dur_d failed");

    assert_eq!(dec_a, dur_a);
    assert_eq!(dec_b, dur_b);
    assert_eq!(dec_c, dur_c);
    assert_eq!(dec_d, dur_d);

    assert_ne!(dec_a, dec_b);
    assert_ne!(dec_c, dec_d);
}
