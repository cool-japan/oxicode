//! Advanced Duration serialization tests for OxiCode (set 2).
//! Exactly 22 top-level #[test] functions covering roundtrips, edge cases,
//! configs, and composite types for std::time::Duration.

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
use std::time::Duration;

// ===== Test 1: Duration::ZERO roundtrip =====

#[test]
fn test_duration_zero_roundtrip() {
    let original = Duration::ZERO;
    let encoded = encode_to_vec(&original).expect("encode Duration::ZERO");
    let (decoded, _): (Duration, usize) =
        decode_from_slice(&encoded).expect("decode Duration::ZERO");
    assert_eq!(original, decoded);
}

// ===== Test 2: Duration::from_secs(1) roundtrip =====

#[test]
fn test_duration_one_second_roundtrip() {
    let original = Duration::from_secs(1);
    let encoded = encode_to_vec(&original).expect("encode Duration::from_secs(1)");
    let (decoded, _): (Duration, usize) =
        decode_from_slice(&encoded).expect("decode Duration::from_secs(1)");
    assert_eq!(original, decoded);
}

// ===== Test 3: Duration::from_millis(1) roundtrip =====

#[test]
fn test_duration_one_millisecond_roundtrip() {
    let original = Duration::from_millis(1);
    let encoded = encode_to_vec(&original).expect("encode Duration::from_millis(1)");
    let (decoded, _): (Duration, usize) =
        decode_from_slice(&encoded).expect("decode Duration::from_millis(1)");
    assert_eq!(original, decoded);
}

// ===== Test 4: Duration::from_micros(1) roundtrip =====

#[test]
fn test_duration_one_microsecond_roundtrip() {
    let original = Duration::from_micros(1);
    let encoded = encode_to_vec(&original).expect("encode Duration::from_micros(1)");
    let (decoded, _): (Duration, usize) =
        decode_from_slice(&encoded).expect("decode Duration::from_micros(1)");
    assert_eq!(original, decoded);
}

// ===== Test 5: Duration::from_nanos(1) roundtrip =====

#[test]
fn test_duration_one_nanosecond_roundtrip() {
    let original = Duration::from_nanos(1);
    let encoded = encode_to_vec(&original).expect("encode Duration::from_nanos(1)");
    let (decoded, _): (Duration, usize) =
        decode_from_slice(&encoded).expect("decode Duration::from_nanos(1)");
    assert_eq!(original, decoded);
}

// ===== Test 6: Duration::MAX roundtrip =====

#[test]
fn test_duration_max_roundtrip() {
    let original = Duration::MAX;
    let encoded = encode_to_vec(&original).expect("encode Duration::MAX");
    let (decoded, _): (Duration, usize) =
        decode_from_slice(&encoded).expect("decode Duration::MAX");
    assert_eq!(original, decoded);
}

// ===== Test 7: Duration::new(5, 500_000_000) fractional seconds roundtrip =====

#[test]
fn test_duration_fractional_seconds_roundtrip() {
    let original = Duration::new(5, 500_000_000);
    let encoded = encode_to_vec(&original).expect("encode Duration::new(5, 500_000_000)");
    let (decoded, _): (Duration, usize) =
        decode_from_slice(&encoded).expect("decode Duration::new(5, 500_000_000)");
    assert_eq!(original, decoded);
}

// ===== Test 8: Duration::from_secs(1_000_000_000) roundtrip =====

#[test]
fn test_duration_large_secs_roundtrip() {
    let original = Duration::from_secs(1_000_000_000);
    let encoded = encode_to_vec(&original).expect("encode Duration::from_secs(1_000_000_000)");
    let (decoded, _): (Duration, usize) =
        decode_from_slice(&encoded).expect("decode Duration::from_secs(1_000_000_000)");
    assert_eq!(original, decoded);
}

// ===== Test 9: bytes consumed equals encoded len =====

#[test]
fn test_duration_consumed_equals_encoded_len() {
    let original = Duration::from_secs(42);
    let encoded = encode_to_vec(&original).expect("encode Duration for consumed check");
    let (_, consumed): (Duration, usize) =
        decode_from_slice(&encoded).expect("decode Duration for consumed check");
    assert_eq!(
        consumed,
        encoded.len(),
        "consumed bytes must equal encoded length"
    );
}

// ===== Test 10: fixed_int_encoding config roundtrip =====

#[test]
fn test_duration_fixed_int_config_roundtrip() {
    let cfg = config::standard().with_fixed_int_encoding();
    let original = Duration::from_millis(12345);
    let encoded =
        encode_to_vec_with_config(&original, cfg).expect("encode Duration with fixed_int config");
    let (decoded, _): (Duration, usize) = decode_from_slice_with_config(&encoded, cfg)
        .expect("decode Duration with fixed_int config");
    assert_eq!(original, decoded);
}

// ===== Test 11: big_endian + fixed_int config roundtrip =====

#[test]
fn test_duration_big_endian_config_roundtrip() {
    let cfg = config::standard()
        .with_big_endian()
        .with_fixed_int_encoding();
    let original = Duration::from_secs(99);
    let encoded =
        encode_to_vec_with_config(&original, cfg).expect("encode Duration big_endian+fixed_int");
    let (decoded, _): (Duration, usize) =
        decode_from_slice_with_config(&encoded, cfg).expect("decode Duration big_endian+fixed_int");
    assert_eq!(original, decoded);
}

// ===== Test 12: Vec<Duration> with 3 elements roundtrip =====

#[test]
fn test_vec_duration_roundtrip() {
    let original: Vec<Duration> = vec![
        Duration::ZERO,
        Duration::from_secs(1),
        Duration::from_millis(500),
    ];
    let encoded = encode_to_vec(&original).expect("encode Vec<Duration>");
    let (decoded, _): (Vec<Duration>, usize) =
        decode_from_slice(&encoded).expect("decode Vec<Duration>");
    assert_eq!(original, decoded);
    assert_eq!(decoded.len(), 3);
}

// ===== Test 13: Option<Duration> Some roundtrip =====

#[test]
fn test_option_duration_some_roundtrip() {
    let original: Option<Duration> = Some(Duration::from_secs(7));
    let encoded = encode_to_vec(&original).expect("encode Option<Duration> Some");
    let (decoded, _): (Option<Duration>, usize) =
        decode_from_slice(&encoded).expect("decode Option<Duration> Some");
    assert_eq!(original, decoded);
    assert!(decoded.is_some());
}

// ===== Test 14: Option<Duration> None roundtrip =====

#[test]
fn test_option_duration_none_roundtrip() {
    let original: Option<Duration> = None;
    let encoded = encode_to_vec(&original).expect("encode Option<Duration> None");
    let (decoded, _): (Option<Duration>, usize) =
        decode_from_slice(&encoded).expect("decode Option<Duration> None");
    assert_eq!(original, decoded);
    assert!(decoded.is_none());
}

// ===== Test 15: (Duration, Duration) tuple roundtrip =====

#[test]
fn test_duration_tuple_roundtrip() {
    let original: (Duration, Duration) = (Duration::from_secs(3), Duration::from_millis(750));
    let encoded = encode_to_vec(&original).expect("encode (Duration, Duration)");
    let (decoded, _): ((Duration, Duration), usize) =
        decode_from_slice(&encoded).expect("decode (Duration, Duration)");
    assert_eq!(original, decoded);
}

// ===== Test 16: Duration encoded size with standard config =====

#[test]
fn test_duration_encode_size_standard() {
    // With standard (varint) encoding, Duration::ZERO encodes as secs=0 (1 byte) + nanos=0 (1 byte) = 2 bytes min.
    let original = Duration::ZERO;
    let encoded = encode_to_vec(&original).expect("encode Duration::ZERO standard");
    assert!(
        encoded.len() >= 2,
        "Duration::ZERO with varint encoding should be at least 2 bytes; got {}",
        encoded.len()
    );
}

// ===== Test 17: Duration with fixed_int is exactly 12 bytes (8 + 4) =====

#[test]
fn test_duration_encode_size_fixed_int() {
    let cfg = config::standard().with_fixed_int_encoding();
    let original = Duration::ZERO;
    let encoded =
        encode_to_vec_with_config(&original, cfg).expect("encode Duration with fixed_int");
    assert_eq!(
        encoded.len(),
        12,
        "Duration with fixed_int encoding must be exactly 12 bytes (u64 secs + u32 nanos)"
    );
}

// ===== Test 18: Duration nanos=999_999_999 (max valid nanos) roundtrip =====

#[test]
fn test_duration_nanos_boundary() {
    let original = Duration::new(0, 999_999_999);
    let encoded = encode_to_vec(&original).expect("encode Duration nanos=999_999_999");
    let (decoded, _): (Duration, usize) =
        decode_from_slice(&encoded).expect("decode Duration nanos=999_999_999");
    assert_eq!(original, decoded);
    assert_eq!(decoded.subsec_nanos(), 999_999_999);
}

// ===== Test 19: Duration::from_nanos(123_456_789) preserves nanos =====

#[test]
fn test_duration_sub_second_precision() {
    let original = Duration::from_nanos(123_456_789);
    let encoded = encode_to_vec(&original).expect("encode Duration::from_nanos(123_456_789)");
    let (decoded, _): (Duration, usize) =
        decode_from_slice(&encoded).expect("decode Duration::from_nanos(123_456_789)");
    assert_eq!(original, decoded);
    assert_eq!(decoded.subsec_nanos(), 123_456_789);
}

// ===== Test 20: verify secs field preserved after roundtrip =====

#[test]
fn test_duration_secs_preserved() {
    let secs_value: u64 = 86_400;
    let original = Duration::from_secs(secs_value);
    let encoded = encode_to_vec(&original).expect("encode Duration secs");
    let (decoded, _): (Duration, usize) =
        decode_from_slice(&encoded).expect("decode Duration secs");
    assert_eq!(decoded.as_secs(), secs_value);
}

// ===== Test 21: verify nanos field preserved after roundtrip =====

#[test]
fn test_duration_nanos_preserved() {
    let nanos_value: u32 = 987_654_321;
    let original = Duration::new(0, nanos_value);
    let encoded = encode_to_vec(&original).expect("encode Duration nanos");
    let (decoded, _): (Duration, usize) =
        decode_from_slice(&encoded).expect("decode Duration nanos");
    assert_eq!(decoded.subsec_nanos(), nanos_value);
}

// ===== Test 22: larger Duration secs uses more bytes in varint encoding =====

#[test]
fn test_duration_ordering_preserved() {
    // With standard (varint) encoding:
    //   Duration::from_secs(1)    => secs varint(1) = 1 byte  + nanos varint(0) = 1 byte = 2 bytes total
    //   Duration::from_secs(1000) => secs varint(1000) = 2 bytes + nanos varint(0) = 1 byte = 3 bytes total
    // The larger secs value must produce an equal or larger encoded size.
    let small = Duration::from_secs(1);
    let large = Duration::from_secs(1000);

    let enc_small = encode_to_vec(&small).expect("encode small Duration");
    let enc_large = encode_to_vec(&large).expect("encode large Duration");

    let (val_small, _): (Duration, usize) =
        decode_from_slice(&enc_small).expect("decode small Duration");
    let (val_large, _): (Duration, usize) =
        decode_from_slice(&enc_large).expect("decode large Duration");
    assert_eq!(val_small, small);
    assert_eq!(val_large, large);

    assert!(
        enc_large.len() >= enc_small.len(),
        "larger Duration secs should use at least as many encoded bytes as smaller: {} vs {}",
        enc_large.len(),
        enc_small.len()
    );
}
