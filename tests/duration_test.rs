//! Tests for Duration and SystemTime encode/decode implementations.

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
use oxicode::{config, decode_from_slice, decode_from_slice_with_config, encode_to_vec};
use std::time::Duration;

// ===== Duration roundtrip tests =====

#[test]
fn test_duration_zero_roundtrip() {
    let original = Duration::ZERO;
    let encoded = encode_to_vec(&original).expect("encode Duration::ZERO");
    let (decoded, consumed): (Duration, _) =
        decode_from_slice(&encoded).expect("decode Duration::ZERO");
    assert_eq!(original, decoded);
    assert_eq!(consumed, encoded.len());
}

#[test]
fn test_duration_one_second_roundtrip() {
    let original = Duration::from_secs(1);
    let encoded = encode_to_vec(&original).expect("encode Duration::from_secs(1)");
    let (decoded, _): (Duration, _) =
        decode_from_slice(&encoded).expect("decode Duration::from_secs(1)");
    assert_eq!(original, decoded);
}

#[test]
fn test_duration_millis_1500_roundtrip() {
    let original = Duration::from_millis(1500);
    let encoded = encode_to_vec(&original).expect("encode Duration::from_millis(1500)");
    let (decoded, _): (Duration, _) =
        decode_from_slice(&encoded).expect("decode Duration::from_millis(1500)");
    assert_eq!(original, decoded);
    // 1500ms = 1s + 500_000_000ns
    assert_eq!(decoded.as_secs(), 1);
    assert_eq!(decoded.subsec_nanos(), 500_000_000);
}

#[test]
fn test_duration_max_roundtrip() {
    let original = Duration::MAX;
    let encoded = encode_to_vec(&original).expect("encode Duration::MAX");
    let (decoded, _): (Duration, _) = decode_from_slice(&encoded).expect("decode Duration::MAX");
    assert_eq!(original, decoded);
}

#[test]
fn test_duration_nanos_999999999_roundtrip() {
    let original = Duration::from_nanos(999_999_999);
    let encoded = encode_to_vec(&original).expect("encode Duration::from_nanos(999_999_999)");
    let (decoded, _): (Duration, _) =
        decode_from_slice(&encoded).expect("decode Duration::from_nanos(999_999_999)");
    assert_eq!(original, decoded);
    assert_eq!(decoded.as_secs(), 0);
    assert_eq!(decoded.subsec_nanos(), 999_999_999);
}

// ===== Duration encode-size test =====

#[test]
fn test_duration_encode_size_fixed_config() {
    // With fixed-int encoding: u64 secs (8 bytes) + u32 nanos (4 bytes) = 12 bytes.
    let config = config::legacy(); // Fixed int encoding, little endian
    let mut buf = [0u8; 32];
    let written = oxicode::encode_into_slice(Duration::ZERO, &mut buf, config)
        .expect("encode Duration::ZERO with fixed config");
    assert_eq!(
        written, 12,
        "Duration with fixed-int encoding should be 12 bytes (u64 secs + u32 nanos)"
    );
}

// ===== Duration distinct encoding test =====

#[test]
fn test_duration_distinct_values_differ_in_encoding() {
    let a = Duration::from_secs(1);
    let b = Duration::from_secs(2);
    let enc_a = encode_to_vec(&a).expect("encode a");
    let enc_b = encode_to_vec(&b).expect("encode b");
    assert_ne!(
        enc_a, enc_b,
        "Different Duration values must produce different encodings"
    );

    let c = Duration::from_millis(1000); // = 1s exactly
    let enc_c = encode_to_vec(&c).expect("encode c");
    assert_eq!(
        enc_a, enc_c,
        "Duration::from_secs(1) and Duration::from_millis(1000) must encode identically"
    );
}

// ===== Duration in a struct roundtrip =====

#[derive(Debug, PartialEq)]
struct Timed {
    name: String,
    elapsed: Duration,
}

// Manual Encode/Decode for Timed (no derive needed for test)
impl oxicode::Encode for Timed {
    fn encode<E: oxicode::enc::Encoder>(&self, encoder: &mut E) -> Result<(), oxicode::Error> {
        self.name.encode(encoder)?;
        self.elapsed.encode(encoder)
    }
}

impl oxicode::Decode for Timed {
    fn decode<D: oxicode::de::Decoder<Context = ()>>(
        decoder: &mut D,
    ) -> Result<Self, oxicode::Error> {
        Ok(Timed {
            name: String::decode(decoder)?,
            elapsed: Duration::decode(decoder)?,
        })
    }
}

#[test]
fn test_timed_struct_roundtrip() {
    let original = Timed {
        name: "benchmark_run".to_string(),
        elapsed: Duration::from_millis(42),
    };
    let encoded = encode_to_vec(&original).expect("encode Timed");
    let (decoded, _): (Timed, _) = decode_from_slice(&encoded).expect("decode Timed");
    assert_eq!(original, decoded);
}

#[test]
fn test_timed_struct_zero_elapsed_roundtrip() {
    let original = Timed {
        name: String::new(),
        elapsed: Duration::ZERO,
    };
    let encoded = encode_to_vec(&original).expect("encode Timed with zero elapsed");
    let (decoded, _): (Timed, _) =
        decode_from_slice(&encoded).expect("decode Timed with zero elapsed");
    assert_eq!(original, decoded);
}

// ===== Vec<Duration> roundtrip =====

#[test]
fn test_vec_duration_roundtrip() {
    let original: Vec<Duration> = vec![
        Duration::ZERO,
        Duration::from_nanos(1),
        Duration::from_micros(500),
        Duration::from_millis(100),
        Duration::from_secs(3600),
        Duration::MAX,
    ];
    let encoded = encode_to_vec(&original).expect("encode Vec<Duration>");
    let (decoded, _): (Vec<Duration>, _) =
        decode_from_slice(&encoded).expect("decode Vec<Duration>");
    assert_eq!(original, decoded);
}

#[test]
fn test_vec_duration_empty_roundtrip() {
    let original: Vec<Duration> = vec![];
    let encoded = encode_to_vec(&original).expect("encode empty Vec<Duration>");
    let (decoded, _): (Vec<Duration>, _) =
        decode_from_slice(&encoded).expect("decode empty Vec<Duration>");
    assert_eq!(original, decoded);
}

// ===== Various subsecond precisions =====

#[test]
fn test_duration_subsecond_precisions() {
    let cases = [
        Duration::from_nanos(1),
        Duration::from_nanos(1_000),       // 1 microsecond
        Duration::from_nanos(1_000_000),   // 1 millisecond
        Duration::from_nanos(999_999_999), // just under 1 second
    ];
    for &d in &cases {
        let encoded = encode_to_vec(&d).expect("encode");
        let (decoded, _): (Duration, _) = decode_from_slice(&encoded).expect("decode");
        assert_eq!(d, decoded, "roundtrip failed for {:?}", d);
    }
}

// ===== Wire format sanity: secs and nanos fields =====

#[test]
fn test_duration_wire_format_fixed_encoding() {
    // With legacy (fixed) config, verify byte layout: [secs: 8 LE bytes][nanos: 4 LE bytes]
    let config = config::legacy();
    let dur = Duration::new(1, 500_000_000); // 1.5 seconds

    let mut buf = [0u8; 12];
    let written = oxicode::encode_into_slice(dur, &mut buf, config).expect("encode");
    assert_eq!(written, 12);

    // secs = 1 as u64 little-endian
    let secs_bytes = &buf[..8];
    assert_eq!(
        u64::from_le_bytes(secs_bytes.try_into().expect("slice")),
        1u64
    );

    // nanos = 500_000_000 as u32 little-endian
    let nanos_bytes = &buf[8..12];
    assert_eq!(
        u32::from_le_bytes(nanos_bytes.try_into().expect("slice")),
        500_000_000u32
    );
}

// ===== SystemTime tests =====

#[cfg(feature = "std")]
mod system_time_tests {
    use super::*;
    use std::time::SystemTime;

    #[test]
    fn test_systemtime_unix_epoch_roundtrip() {
        let original = SystemTime::UNIX_EPOCH;
        let encoded = encode_to_vec(&original).expect("encode SystemTime::UNIX_EPOCH");
        let (decoded, consumed): (SystemTime, _) =
            decode_from_slice(&encoded).expect("decode SystemTime::UNIX_EPOCH");
        assert_eq!(original, decoded);
        assert_eq!(consumed, encoded.len());
    }

    #[test]
    fn test_systemtime_one_second_after_epoch_roundtrip() {
        let original = SystemTime::UNIX_EPOCH + Duration::from_secs(1);
        let encoded = encode_to_vec(&original).expect("encode SystemTime 1s after epoch");
        let (decoded, _): (SystemTime, _) =
            decode_from_slice(&encoded).expect("decode SystemTime 1s after epoch");
        assert_eq!(original, decoded);
    }

    #[test]
    fn test_systemtime_large_timestamp_roundtrip() {
        // Year ~2100: ~4_102_444_800 seconds after epoch
        let original = SystemTime::UNIX_EPOCH + Duration::from_secs(4_102_444_800);
        let encoded = encode_to_vec(&original).expect("encode future SystemTime");
        let (decoded, _): (SystemTime, _) =
            decode_from_slice(&encoded).expect("decode future SystemTime");
        assert_eq!(original, decoded);
    }

    #[test]
    fn test_systemtime_with_subsecond_nanos_roundtrip() {
        let original = SystemTime::UNIX_EPOCH + Duration::new(1_000_000, 123_456_789);
        let encoded = encode_to_vec(&original).expect("encode SystemTime with nanos");
        let (decoded, _): (SystemTime, _) =
            decode_from_slice(&encoded).expect("decode SystemTime with nanos");
        assert_eq!(original, decoded);
    }

    #[test]
    fn test_systemtime_distinct_values_differ_in_encoding() {
        let a = SystemTime::UNIX_EPOCH + Duration::from_secs(100);
        let b = SystemTime::UNIX_EPOCH + Duration::from_secs(200);
        let enc_a = encode_to_vec(&a).expect("encode a");
        let enc_b = encode_to_vec(&b).expect("encode b");
        assert_ne!(
            enc_a, enc_b,
            "Different SystemTime values must differ in encoding"
        );
    }

    #[test]
    fn test_systemtime_wire_format_epoch() {
        // UNIX_EPOCH should encode as 0i64 secs + 0u32 nanos
        // With fixed config: 8 bytes secs + 4 bytes nanos = 12 bytes
        let config = config::legacy();
        let mut buf = [0u8; 12];
        let written =
            oxicode::encode_into_slice(SystemTime::UNIX_EPOCH, &mut buf, config).expect("encode");
        assert_eq!(written, 12);
        // All zeros for epoch
        assert_eq!(&buf, &[0u8; 12]);
    }
}

// ===== Duration decode-from-slice with config =====

#[test]
fn test_duration_fixed_config_roundtrip() {
    let config = config::legacy();
    let original = Duration::from_secs(12345);
    let mut buf = [0u8; 16];
    let written =
        oxicode::encode_into_slice(original, &mut buf, config).expect("encode with fixed config");
    assert_eq!(written, 12);
    let (decoded, consumed): (Duration, _) =
        decode_from_slice_with_config(&buf[..written], config).expect("decode with fixed config");
    assert_eq!(original, decoded);
    assert_eq!(consumed, 12);
}
