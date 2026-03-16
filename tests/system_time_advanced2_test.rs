//! Advanced SystemTime serialization tests for OxiCode — batch 2.
//! 22 tests covering UNIX_EPOCH roundtrips, config variants, Vec/Option wrappers,
//! byte-size guarantees, consistency, and subsecond precision.

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

// ===== Test 1: UNIX_EPOCH roundtrip =====

#[test]
fn test_unix_epoch_roundtrip() {
    let original = UNIX_EPOCH;
    let encoded = encode_to_vec(&original).expect("encode UNIX_EPOCH failed");
    let (decoded, consumed): (SystemTime, usize) =
        decode_from_slice(&encoded).expect("decode UNIX_EPOCH failed");
    assert_eq!(decoded, original);
    assert_eq!(consumed, encoded.len());
    let since = decoded
        .duration_since(UNIX_EPOCH)
        .expect("duration_since UNIX_EPOCH failed");
    assert_eq!(since, Duration::ZERO);
}

// ===== Test 2: UNIX_EPOCH + 1 second roundtrip =====

#[test]
fn test_unix_epoch_plus_1sec_roundtrip() {
    let original = UNIX_EPOCH + Duration::from_secs(1);
    let encoded = encode_to_vec(&original).expect("encode UNIX_EPOCH+1s failed");
    let (decoded, consumed): (SystemTime, usize) =
        decode_from_slice(&encoded).expect("decode UNIX_EPOCH+1s failed");
    assert_eq!(decoded, original);
    assert_eq!(consumed, encoded.len());
    let since = decoded
        .duration_since(UNIX_EPOCH)
        .expect("duration_since UNIX_EPOCH+1s failed");
    assert_eq!(since.as_secs(), 1);
    assert_eq!(since.subsec_nanos(), 0);
}

// ===== Test 3: UNIX_EPOCH + 1_000_000_000 seconds roundtrip =====

#[test]
fn test_unix_epoch_plus_large_duration_roundtrip() {
    let secs: u64 = 1_000_000_000;
    let original = UNIX_EPOCH + Duration::from_secs(secs);
    let encoded = encode_to_vec(&original).expect("encode UNIX_EPOCH+1e9s failed");
    let (decoded, consumed): (SystemTime, usize) =
        decode_from_slice(&encoded).expect("decode UNIX_EPOCH+1e9s failed");
    assert_eq!(decoded, original);
    assert_eq!(consumed, encoded.len());
    let since = decoded
        .duration_since(UNIX_EPOCH)
        .expect("duration_since large duration failed");
    assert_eq!(since.as_secs(), secs);
}

// ===== Test 4: UNIX_EPOCH + Duration::from_nanos(999_999_999) roundtrip =====

#[test]
fn test_unix_epoch_plus_nanos_roundtrip() {
    let original = UNIX_EPOCH + Duration::from_nanos(999_999_999);
    let encoded = encode_to_vec(&original).expect("encode UNIX_EPOCH+999999999ns failed");
    let (decoded, consumed): (SystemTime, usize) =
        decode_from_slice(&encoded).expect("decode UNIX_EPOCH+999999999ns failed");
    assert_eq!(decoded, original);
    assert_eq!(consumed, encoded.len());
    let since = decoded
        .duration_since(UNIX_EPOCH)
        .expect("duration_since nanos failed");
    assert_eq!(since.as_secs(), 0);
    assert_eq!(since.subsec_nanos(), 999_999_999);
}

// ===== Test 5: bytes consumed equals encoded length =====

#[test]
fn test_system_time_consumed_equals_len() {
    let t = UNIX_EPOCH + Duration::from_secs(42);
    let encoded = encode_to_vec(&t).expect("encode for consumed-check failed");
    let (_, consumed): (SystemTime, usize) =
        decode_from_slice(&encoded).expect("decode for consumed-check failed");
    assert_eq!(
        consumed,
        encoded.len(),
        "consumed must equal full encoded length"
    );
}

// ===== Test 6: SystemTime with fixed_int_encoding config roundtrip =====

#[test]
fn test_system_time_fixed_int_config_roundtrip() {
    let cfg = config::standard().with_fixed_int_encoding();
    let original = UNIX_EPOCH + Duration::from_secs(86_400);
    let encoded =
        encode_to_vec_with_config(&original, cfg).expect("encode fixed-int SystemTime failed");
    let (decoded, consumed): (SystemTime, usize) =
        decode_from_slice_with_config(&encoded, cfg).expect("decode fixed-int SystemTime failed");
    assert_eq!(decoded, original);
    assert_eq!(consumed, encoded.len());
}

// ===== Test 7: SystemTime with big_endian + fixed_int config roundtrip =====

#[test]
fn test_system_time_big_endian_config_roundtrip() {
    let cfg = config::standard()
        .with_big_endian()
        .with_fixed_int_encoding();
    let original = UNIX_EPOCH + Duration::from_secs(1_234_567_890);
    let encoded =
        encode_to_vec_with_config(&original, cfg).expect("encode big-endian SystemTime failed");
    // Fixed-int: i64 secs (8 bytes) + u32 nanos (4 bytes) = 12 bytes
    assert_eq!(
        encoded.len(),
        12,
        "big-endian fixed-int SystemTime must be 12 bytes"
    );
    let (decoded, consumed): (SystemTime, usize) =
        decode_from_slice_with_config(&encoded, cfg).expect("decode big-endian SystemTime failed");
    assert_eq!(decoded, original);
    assert_eq!(consumed, 12);
}

// ===== Test 8: Vec<SystemTime> with 3 values roundtrip =====

#[test]
fn test_vec_system_time_roundtrip() {
    let original: Vec<SystemTime> = vec![
        UNIX_EPOCH,
        UNIX_EPOCH + Duration::from_secs(1_000),
        UNIX_EPOCH + Duration::from_secs(2_000_000_000),
    ];
    let encoded = encode_to_vec(&original).expect("encode Vec<SystemTime> failed");
    let (decoded, consumed): (Vec<SystemTime>, usize) =
        decode_from_slice(&encoded).expect("decode Vec<SystemTime> failed");
    assert_eq!(decoded, original);
    assert_eq!(decoded.len(), 3);
    assert_eq!(consumed, encoded.len());
}

// ===== Test 9: Option<SystemTime> Some roundtrip =====

#[test]
fn test_option_system_time_some_roundtrip() {
    let original: Option<SystemTime> = Some(UNIX_EPOCH + Duration::from_secs(1_609_459_200));
    let encoded = encode_to_vec(&original).expect("encode Some(SystemTime) failed");
    let (decoded, consumed): (Option<SystemTime>, usize) =
        decode_from_slice(&encoded).expect("decode Some(SystemTime) failed");
    assert_eq!(decoded, original);
    assert_eq!(consumed, encoded.len());
    assert!(decoded.is_some(), "decoded must be Some");
}

// ===== Test 10: Option<SystemTime> None roundtrip =====

#[test]
fn test_option_system_time_none_roundtrip() {
    let original: Option<SystemTime> = None;
    let encoded = encode_to_vec(&original).expect("encode None::<SystemTime> failed");
    let (decoded, consumed): (Option<SystemTime>, usize) =
        decode_from_slice(&encoded).expect("decode None::<SystemTime> failed");
    assert_eq!(decoded, original);
    assert_eq!(consumed, encoded.len());
    assert!(decoded.is_none(), "decoded must be None");
}

// ===== Test 11: (SystemTime, u32) tuple roundtrip =====

#[test]
fn test_system_time_tuple_roundtrip() {
    let original: (SystemTime, u32) = (UNIX_EPOCH + Duration::from_secs(500_000), 42u32);
    let encoded = encode_to_vec(&original).expect("encode (SystemTime, u32) failed");
    let (decoded, consumed): ((SystemTime, u32), usize) =
        decode_from_slice(&encoded).expect("decode (SystemTime, u32) failed");
    assert_eq!(decoded, original);
    assert_eq!(consumed, encoded.len());
    assert_eq!(decoded.1, 42u32);
}

// ===== Test 12: same time encodes to same bytes =====

#[test]
fn test_system_time_encode_twice_consistent() {
    let t = UNIX_EPOCH + Duration::from_secs(987_654_321);
    let enc1 = encode_to_vec(&t).expect("encode first time failed");
    let enc2 = encode_to_vec(&t).expect("encode second time failed");
    assert_eq!(
        enc1, enc2,
        "same SystemTime must produce identical byte sequences"
    );
}

// ===== Test 13: different times produce different bytes =====

#[test]
fn test_different_times_different_bytes() {
    let t1 = UNIX_EPOCH;
    let t2 = UNIX_EPOCH + Duration::from_secs(1);
    let enc1 = encode_to_vec(&t1).expect("encode UNIX_EPOCH failed");
    let enc2 = encode_to_vec(&t2).expect("encode UNIX_EPOCH+1s failed");
    assert_ne!(
        enc1, enc2,
        "UNIX_EPOCH and UNIX_EPOCH+1s must produce different bytes"
    );
}

// ===== Test 14: encode a time, decode it, verify duration_since works =====

#[test]
fn test_system_time_duration_since_roundtrip() {
    let secs: u64 = 1_700_000_000;
    let nanos: u32 = 123_456_789;
    let original = UNIX_EPOCH + Duration::new(secs, nanos);
    let encoded =
        encode_to_vec(&original).expect("encode SystemTime for duration_since test failed");
    let (decoded, _): (SystemTime, usize) =
        decode_from_slice(&encoded).expect("decode SystemTime for duration_since test failed");
    let since = decoded
        .duration_since(UNIX_EPOCH)
        .expect("duration_since after decode failed");
    assert_eq!(since.as_secs(), secs);
    assert_eq!(since.subsec_nanos(), nanos);
}

// ===== Test 15: UNIX_EPOCH with fixed_int is exactly 12 bytes (8+4) =====

#[test]
fn test_unix_epoch_fixed_int_size() {
    let cfg = config::standard().with_fixed_int_encoding();
    let encoded =
        encode_to_vec_with_config(&UNIX_EPOCH, cfg).expect("encode UNIX_EPOCH fixed-int failed");
    assert_eq!(
        encoded.len(),
        12,
        "UNIX_EPOCH with fixed_int_encoding must be exactly 12 bytes (i64 secs + u32 nanos)"
    );
}

// ===== Test 16: fractional second with 500_000_000 nanos roundtrip =====

#[test]
fn test_system_time_large_nanos_roundtrip() {
    let original = UNIX_EPOCH + Duration::new(100, 500_000_000);
    let encoded = encode_to_vec(&original).expect("encode SystemTime with 500ms nanos failed");
    let (decoded, consumed): (SystemTime, usize) =
        decode_from_slice(&encoded).expect("decode SystemTime with 500ms nanos failed");
    assert_eq!(decoded, original);
    assert_eq!(consumed, encoded.len());
    let since = decoded
        .duration_since(UNIX_EPOCH)
        .expect("duration_since 500ms nanos failed");
    assert_eq!(since.as_secs(), 100);
    assert_eq!(since.subsec_nanos(), 500_000_000);
}

// ===== Test 17: times at exact second boundaries =====

#[test]
fn test_system_time_second_boundary() {
    let boundaries: Vec<u64> = vec![0, 1, 59, 60, 3599, 3600, 86399, 86400];
    for secs in boundaries {
        let t = UNIX_EPOCH + Duration::from_secs(secs);
        let encoded = encode_to_vec(&t).expect("encode second boundary failed");
        let (decoded, consumed): (SystemTime, usize) =
            decode_from_slice(&encoded).expect("decode second boundary failed");
        assert_eq!(decoded, t, "roundtrip failed for secs={secs}");
        assert_eq!(consumed, encoded.len(), "consumed mismatch for secs={secs}");
        let since = decoded
            .duration_since(UNIX_EPOCH)
            .expect("duration_since boundary failed");
        assert_eq!(since.as_secs(), secs, "secs mismatch for secs={secs}");
        assert_eq!(
            since.subsec_nanos(),
            0,
            "nanos must be 0 for pure second boundaries"
        );
    }
}

// ===== Test 18: verify nanos preserved after roundtrip =====

#[test]
fn test_system_time_subsecond_precision() {
    let nanos_cases: Vec<u32> = vec![1, 999, 1_000_000, 500_000_000, 999_999_999];
    for nanos in nanos_cases {
        let t = UNIX_EPOCH + Duration::new(1000, nanos);
        let encoded = encode_to_vec(&t).expect("encode subsecond precision failed");
        let (decoded, _): (SystemTime, usize) =
            decode_from_slice(&encoded).expect("decode subsecond precision failed");
        let since = decoded
            .duration_since(UNIX_EPOCH)
            .expect("duration_since subsecond failed");
        assert_eq!(
            since.subsec_nanos(),
            nanos,
            "nanos must be preserved for nanos={nanos}"
        );
    }
}

// ===== Test 19: UNIX_EPOCH + 9_999_999_999 seconds (far future) roundtrip =====

#[test]
fn test_system_time_far_future_roundtrip() {
    let secs: u64 = 9_999_999_999;
    let original = UNIX_EPOCH + Duration::from_secs(secs);
    let encoded = encode_to_vec(&original).expect("encode far-future SystemTime failed");
    let (decoded, consumed): (SystemTime, usize) =
        decode_from_slice(&encoded).expect("decode far-future SystemTime failed");
    assert_eq!(decoded, original);
    assert_eq!(consumed, encoded.len());
    let since = decoded
        .duration_since(UNIX_EPOCH)
        .expect("duration_since far future failed");
    assert_eq!(since.as_secs(), secs);
}

// ===== Test 20: UNIX_EPOCH + 1_700_000_000 seconds (~2023) roundtrip =====

#[test]
fn test_system_time_recent_roundtrip() {
    let secs: u64 = 1_700_000_000;
    let original = UNIX_EPOCH + Duration::from_secs(secs);
    let encoded = encode_to_vec(&original).expect("encode recent SystemTime failed");
    let (decoded, consumed): (SystemTime, usize) =
        decode_from_slice(&encoded).expect("decode recent SystemTime failed");
    assert_eq!(decoded, original);
    assert_eq!(consumed, encoded.len());
    let since = decoded
        .duration_since(UNIX_EPOCH)
        .expect("duration_since recent failed");
    assert_eq!(since.as_secs(), secs);
    assert_eq!(since.subsec_nanos(), 0);
}

// ===== Test 21: UNIX_EPOCH with standard config (varint encoding of secs=0) =====

#[test]
fn test_unix_epoch_standard_config() {
    let cfg = config::standard();
    let encoded = encode_to_vec_with_config(&UNIX_EPOCH, cfg)
        .expect("encode UNIX_EPOCH standard config failed");
    let (decoded, consumed): (SystemTime, usize) = decode_from_slice_with_config(&encoded, cfg)
        .expect("decode UNIX_EPOCH standard config failed");
    assert_eq!(decoded, UNIX_EPOCH);
    assert_eq!(consumed, encoded.len());
    // Varint encoding of 0 is very compact
    assert!(
        encoded.len() < 12,
        "varint-encoded UNIX_EPOCH must be smaller than fixed-int (12 bytes)"
    );
}

// ===== Test 22: verify secs (duration_since EPOCH) is preserved =====

#[test]
fn test_system_time_secs_preserved() {
    let test_cases: Vec<u64> = vec![0, 1, 1_000, 1_000_000, 1_000_000_000, 2_000_000_000];
    for secs in test_cases {
        let original = UNIX_EPOCH + Duration::from_secs(secs);
        let encoded = encode_to_vec(&original).expect("encode for secs-preserved test failed");
        let (decoded, _): (SystemTime, usize) =
            decode_from_slice(&encoded).expect("decode for secs-preserved test failed");
        let since = decoded
            .duration_since(UNIX_EPOCH)
            .expect("duration_since secs-preserved failed");
        assert_eq!(
            since.as_secs(),
            secs,
            "secs must be preserved for secs={secs}"
        );
    }
}
