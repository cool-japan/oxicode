//! Advanced tests for Duration and SystemTime encoding in OxiCode.
//! 22 tests covering roundtrips, edge cases, configs, and composite types.

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
use oxicode::{config, decode_from_slice, encode_to_vec};
use std::time::{Duration, SystemTime, UNIX_EPOCH};

// ===== Test 1: Duration::from_secs(0) roundtrip =====

#[test]
fn test_duration_secs_zero_roundtrip() {
    let original = Duration::from_secs(0);
    let encoded = encode_to_vec(&original).expect("encode Duration::from_secs(0)");
    let (decoded, consumed): (Duration, _) =
        decode_from_slice(&encoded).expect("decode Duration::from_secs(0)");
    assert_eq!(original, decoded);
    assert_eq!(consumed, encoded.len());
}

// ===== Test 2: Duration::from_secs(1) roundtrip =====

#[test]
fn test_duration_secs_one_roundtrip() {
    let original = Duration::from_secs(1);
    let encoded = encode_to_vec(&original).expect("encode Duration::from_secs(1)");
    let (decoded, consumed): (Duration, _) =
        decode_from_slice(&encoded).expect("decode Duration::from_secs(1)");
    assert_eq!(original, decoded);
    assert_eq!(consumed, encoded.len());
    assert_eq!(decoded.as_secs(), 1);
    assert_eq!(decoded.subsec_nanos(), 0);
}

// ===== Test 3: Duration::from_secs(3600) one hour =====

#[test]
fn test_duration_one_hour_roundtrip() {
    let original = Duration::from_secs(3600);
    let encoded = encode_to_vec(&original).expect("encode Duration::from_secs(3600)");
    let (decoded, _): (Duration, _) =
        decode_from_slice(&encoded).expect("decode Duration::from_secs(3600)");
    assert_eq!(original, decoded);
    assert_eq!(decoded.as_secs(), 3600);
    assert_eq!(decoded.subsec_nanos(), 0);
}

// ===== Test 4: Duration::from_secs(86400) one day =====

#[test]
fn test_duration_one_day_roundtrip() {
    let original = Duration::from_secs(86400);
    let encoded = encode_to_vec(&original).expect("encode Duration::from_secs(86400)");
    let (decoded, _): (Duration, _) =
        decode_from_slice(&encoded).expect("decode Duration::from_secs(86400)");
    assert_eq!(original, decoded);
    assert_eq!(decoded.as_secs(), 86400);
}

// ===== Test 5: Duration::from_millis(500) half second =====

#[test]
fn test_duration_half_second_roundtrip() {
    let original = Duration::from_millis(500);
    let encoded = encode_to_vec(&original).expect("encode Duration::from_millis(500)");
    let (decoded, _): (Duration, _) =
        decode_from_slice(&encoded).expect("decode Duration::from_millis(500)");
    assert_eq!(original, decoded);
    assert_eq!(decoded.as_secs(), 0);
    assert_eq!(decoded.subsec_nanos(), 500_000_000);
}

// ===== Test 6: Duration::from_micros(1000) microseconds =====

#[test]
fn test_duration_microseconds_roundtrip() {
    let original = Duration::from_micros(1000);
    let encoded = encode_to_vec(&original).expect("encode Duration::from_micros(1000)");
    let (decoded, _): (Duration, _) =
        decode_from_slice(&encoded).expect("decode Duration::from_micros(1000)");
    assert_eq!(original, decoded);
    // 1000 µs = 1_000_000 ns
    assert_eq!(decoded.subsec_nanos(), 1_000_000);
    assert_eq!(decoded.as_secs(), 0);
}

// ===== Test 7: Duration::from_nanos(999_999_999) nanoseconds =====

#[test]
fn test_duration_nanos_just_under_one_second_roundtrip() {
    let original = Duration::from_nanos(999_999_999);
    let encoded = encode_to_vec(&original).expect("encode Duration::from_nanos(999_999_999)");
    let (decoded, _): (Duration, _) =
        decode_from_slice(&encoded).expect("decode Duration::from_nanos(999_999_999)");
    assert_eq!(original, decoded);
    assert_eq!(decoded.as_secs(), 0);
    assert_eq!(decoded.subsec_nanos(), 999_999_999);
}

// ===== Test 8: Duration::MAX maximum Duration =====

#[test]
fn test_duration_max_roundtrip() {
    let original = Duration::MAX;
    let encoded = encode_to_vec(&original).expect("encode Duration::MAX");
    let (decoded, consumed): (Duration, _) =
        decode_from_slice(&encoded).expect("decode Duration::MAX");
    assert_eq!(original, decoded);
    assert_eq!(consumed, encoded.len());
}

// ===== Test 9: Duration::ZERO =====

#[test]
fn test_duration_zero_const_roundtrip() {
    let original = Duration::ZERO;
    let encoded = encode_to_vec(&original).expect("encode Duration::ZERO");
    let (decoded, _): (Duration, _) = decode_from_slice(&encoded).expect("decode Duration::ZERO");
    assert_eq!(original, decoded);
    assert_eq!(decoded.as_secs(), 0);
    assert_eq!(decoded.subsec_nanos(), 0);
}

// ===== Test 10: Vec<Duration> with 3 durations roundtrip =====

#[test]
fn test_vec_of_three_durations_roundtrip() {
    let original: Vec<Duration> = vec![
        Duration::from_secs(10),
        Duration::from_millis(250),
        Duration::from_nanos(1),
    ];
    let encoded = encode_to_vec(&original).expect("encode Vec<Duration> with 3 items");
    let (decoded, _): (Vec<Duration>, _) =
        decode_from_slice(&encoded).expect("decode Vec<Duration> with 3 items");
    assert_eq!(original, decoded);
    assert_eq!(decoded.len(), 3);
}

// ===== Test 11: Option<Duration> Some/None roundtrip =====

#[test]
fn test_option_duration_some_none_roundtrip() {
    let some_val: Option<Duration> = Some(Duration::from_secs(42));
    let none_val: Option<Duration> = None;

    let enc_some = encode_to_vec(&some_val).expect("encode Some(Duration)");
    let (dec_some, _): (Option<Duration>, _) =
        decode_from_slice(&enc_some).expect("decode Some(Duration)");
    assert_eq!(some_val, dec_some);

    let enc_none = encode_to_vec(&none_val).expect("encode None::<Duration>");
    let (dec_none, _): (Option<Duration>, _) =
        decode_from_slice(&enc_none).expect("decode None::<Duration>");
    assert_eq!(none_val, dec_none);

    // None and Some must produce different encodings
    assert_ne!(enc_some, enc_none);
}

// ===== Test 12: Duration with fixed int encoding =====

#[test]
fn test_duration_fixed_int_encoding_roundtrip() {
    let cfg = config::standard().with_fixed_int_encoding();
    let original = Duration::from_secs(12345);

    let mut buf = [0u8; 32];
    let written =
        oxicode::encode_into_slice(original, &mut buf, cfg).expect("encode with fixed int config");
    // fixed u64 (8) + fixed u32 (4) = 12 bytes
    assert_eq!(written, 12);

    let (decoded, consumed): (Duration, _) =
        oxicode::decode_from_slice_with_config(&buf[..written], cfg)
            .expect("decode with fixed int config");
    assert_eq!(original, decoded);
    assert_eq!(consumed, 12);
}

// ===== Test 13: Duration with big endian config =====

#[test]
fn test_duration_big_endian_config_roundtrip() {
    let cfg = config::standard()
        .with_big_endian()
        .with_fixed_int_encoding();
    let original = Duration::new(7, 123_456_789);

    let mut buf = [0u8; 32];
    let written = oxicode::encode_into_slice(original, &mut buf, cfg)
        .expect("encode Duration with big endian + fixed");
    assert_eq!(
        written, 12,
        "big endian fixed: u64 secs + u32 nanos = 12 bytes"
    );

    // Verify big-endian byte order for secs (7 as u64 big-endian)
    let secs_be = u64::from_be_bytes(buf[..8].try_into().expect("secs slice"));
    assert_eq!(secs_be, 7u64);

    let nanos_be = u32::from_be_bytes(buf[8..12].try_into().expect("nanos slice"));
    assert_eq!(nanos_be, 123_456_789u32);

    let (decoded, _): (Duration, _) = oxicode::decode_from_slice_with_config(&buf[..written], cfg)
        .expect("decode Duration with big endian + fixed");
    assert_eq!(original, decoded);
}

// ===== Test 14: Duration 1s + 500ms (1.5 seconds, no floats) =====

#[test]
fn test_duration_one_and_half_seconds_roundtrip() {
    let original = Duration::from_secs(1) + Duration::from_millis(500);
    assert_eq!(original.as_secs(), 1);
    assert_eq!(original.subsec_nanos(), 500_000_000);

    let encoded = encode_to_vec(&original).expect("encode 1.5s duration");
    let (decoded, _): (Duration, _) = decode_from_slice(&encoded).expect("decode 1.5s duration");
    assert_eq!(original, decoded);
    assert_eq!(decoded.as_secs(), 1);
    assert_eq!(decoded.subsec_nanos(), 500_000_000);
}

// ===== Test 15: SystemTime::UNIX_EPOCH roundtrip =====

#[test]
fn test_systemtime_unix_epoch_roundtrip() {
    let original = SystemTime::UNIX_EPOCH;
    let encoded = encode_to_vec(&original).expect("encode SystemTime::UNIX_EPOCH");
    let (decoded, consumed): (SystemTime, _) =
        decode_from_slice(&encoded).expect("decode SystemTime::UNIX_EPOCH");

    let orig_since = original
        .duration_since(UNIX_EPOCH)
        .expect("UNIX_EPOCH since UNIX_EPOCH");
    let dec_since = decoded
        .duration_since(UNIX_EPOCH)
        .expect("decoded UNIX_EPOCH since UNIX_EPOCH");
    assert_eq!(orig_since, dec_since);
    assert_eq!(consumed, encoded.len());
}

// ===== Test 16: SystemTime::UNIX_EPOCH + Duration::from_secs(1_000_000_000) roundtrip =====

#[test]
fn test_systemtime_one_billion_secs_after_epoch_roundtrip() {
    let offset = Duration::from_secs(1_000_000_000);
    let original = SystemTime::UNIX_EPOCH + offset;
    let encoded = encode_to_vec(&original).expect("encode SystemTime 1e9s after epoch");
    let (decoded, _): (SystemTime, _) =
        decode_from_slice(&encoded).expect("decode SystemTime 1e9s after epoch");

    let orig_since = original
        .duration_since(UNIX_EPOCH)
        .expect("original duration_since UNIX_EPOCH");
    let dec_since = decoded
        .duration_since(UNIX_EPOCH)
        .expect("decoded duration_since UNIX_EPOCH");
    assert_eq!(orig_since, dec_since);
    assert_eq!(orig_since.as_secs(), 1_000_000_000);
}

// ===== Test 17: SystemTime::UNIX_EPOCH + Duration::from_secs(0) roundtrip =====

#[test]
fn test_systemtime_epoch_plus_zero_roundtrip() {
    let original = SystemTime::UNIX_EPOCH + Duration::from_secs(0);
    let encoded = encode_to_vec(&original).expect("encode UNIX_EPOCH + 0s");
    let (decoded, _): (SystemTime, _) =
        decode_from_slice(&encoded).expect("decode UNIX_EPOCH + 0s");

    let orig_since = original
        .duration_since(UNIX_EPOCH)
        .expect("original duration_since");
    let dec_since = decoded
        .duration_since(UNIX_EPOCH)
        .expect("decoded duration_since");
    assert_eq!(orig_since, dec_since);
    assert_eq!(dec_since.as_secs(), 0);
    assert_eq!(dec_since.subsec_nanos(), 0);
}

// ===== Test 18: Vec<SystemTime> with 2 times roundtrip =====

#[test]
fn test_vec_of_two_systemtimes_roundtrip() {
    let original: Vec<SystemTime> = vec![
        SystemTime::UNIX_EPOCH + Duration::from_secs(100),
        SystemTime::UNIX_EPOCH + Duration::from_secs(200),
    ];
    let encoded = encode_to_vec(&original).expect("encode Vec<SystemTime>");
    let (decoded, _): (Vec<SystemTime>, _) =
        decode_from_slice(&encoded).expect("decode Vec<SystemTime>");
    assert_eq!(decoded.len(), 2);

    for (orig, dec) in original.iter().zip(decoded.iter()) {
        let orig_since = orig
            .duration_since(UNIX_EPOCH)
            .expect("orig duration_since");
        let dec_since = dec.duration_since(UNIX_EPOCH).expect("dec duration_since");
        assert_eq!(orig_since, dec_since);
    }
}

// ===== Test 19: Option<SystemTime> Some/None roundtrip =====

#[test]
fn test_option_systemtime_some_none_roundtrip() {
    let some_val: Option<SystemTime> =
        Some(SystemTime::UNIX_EPOCH + Duration::from_secs(1_609_459_200)); // 2021-01-01
    let none_val: Option<SystemTime> = None;

    let enc_some = encode_to_vec(&some_val).expect("encode Some(SystemTime)");
    let (dec_some, _): (Option<SystemTime>, _) =
        decode_from_slice(&enc_some).expect("decode Some(SystemTime)");

    if let (Some(orig), Some(dec)) = (some_val, dec_some) {
        let orig_since = orig
            .duration_since(UNIX_EPOCH)
            .expect("orig duration_since");
        let dec_since = dec.duration_since(UNIX_EPOCH).expect("dec duration_since");
        assert_eq!(orig_since, dec_since);
    } else {
        panic!("Expected both Some(SystemTime)");
    }

    let enc_none = encode_to_vec(&none_val).expect("encode None::<SystemTime>");
    let (dec_none, _): (Option<SystemTime>, _) =
        decode_from_slice(&enc_none).expect("decode None::<SystemTime>");
    assert!(dec_none.is_none());

    assert_ne!(enc_some, enc_none);
}

// ===== Test 20: SystemTime with fixed int encoding =====

#[test]
fn test_systemtime_fixed_int_encoding_roundtrip() {
    let cfg = config::standard().with_fixed_int_encoding();
    let original = SystemTime::UNIX_EPOCH + Duration::from_secs(9999);

    let mut buf = [0u8; 32];
    let written = oxicode::encode_into_slice(original, &mut buf, cfg)
        .expect("encode SystemTime with fixed int");
    // i64 secs (8) + u32 nanos (4) = 12 bytes
    assert_eq!(written, 12);

    let (decoded, consumed): (SystemTime, _) =
        oxicode::decode_from_slice_with_config(&buf[..written], cfg)
            .expect("decode SystemTime with fixed int");
    assert_eq!(consumed, 12);

    let orig_since = original
        .duration_since(UNIX_EPOCH)
        .expect("original duration_since");
    let dec_since = decoded
        .duration_since(UNIX_EPOCH)
        .expect("decoded duration_since");
    assert_eq!(orig_since, dec_since);
}

// ===== Test 21: (Duration, SystemTime) tuple roundtrip =====

#[test]
fn test_duration_systemtime_tuple_roundtrip() {
    let dur = Duration::from_millis(750);
    let st = SystemTime::UNIX_EPOCH + Duration::from_secs(1_700_000_000);
    let original = (dur, st);

    let encoded = encode_to_vec(&original).expect("encode (Duration, SystemTime) tuple");
    let (decoded, consumed): ((Duration, SystemTime), _) =
        decode_from_slice(&encoded).expect("decode (Duration, SystemTime) tuple");
    assert_eq!(consumed, encoded.len());

    assert_eq!(original.0, decoded.0);

    let orig_st_since = original
        .1
        .duration_since(UNIX_EPOCH)
        .expect("original SystemTime duration_since");
    let dec_st_since = decoded
        .1
        .duration_since(UNIX_EPOCH)
        .expect("decoded SystemTime duration_since");
    assert_eq!(orig_st_since, dec_st_since);
}

// ===== Test 22: Duration::new(secs, nanos) fields verified after decode =====

#[test]
fn test_duration_new_secs_nanos_fields_after_decode() {
    let secs: u64 = 123_456_789;
    let nanos: u32 = 987_654_321;
    let original = Duration::new(secs, nanos);

    assert_eq!(original.as_secs(), secs);
    assert_eq!(original.subsec_nanos(), nanos);

    let encoded = encode_to_vec(&original).expect("encode Duration::new(secs, nanos)");
    let (decoded, consumed): (Duration, _) =
        decode_from_slice(&encoded).expect("decode Duration::new(secs, nanos)");
    assert_eq!(consumed, encoded.len());

    assert_eq!(decoded.as_secs(), secs, "secs field must survive roundtrip");
    assert_eq!(
        decoded.subsec_nanos(),
        nanos,
        "subsec_nanos field must survive roundtrip"
    );
    assert_eq!(original, decoded);
}
