//! Advanced float encoding/decoding edge-case tests.
//!
//! Covers IEEE 754 special values, exact byte-layout verification,
//! collections, tuples, big-endian configuration, and derive macros.

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
use std::f32::consts as f32c;
use std::f64::consts as f64c;

// ── helpers ──────────────────────────────────────────────────────────────────

fn roundtrip_f32(v: f32) -> f32 {
    let enc = encode_to_vec(&v).expect("f32 encode");
    let (dec, _): (f32, _) = decode_from_slice(&enc).expect("f32 decode");
    dec
}

fn roundtrip_f64(v: f64) -> f64 {
    let enc = encode_to_vec(&v).expect("f64 encode");
    let (dec, _): (f64, _) = decode_from_slice(&enc).expect("f64 decode");
    dec
}

// ── test 1: f32::NAN roundtrip via to_bits() ─────────────────────────────────

#[test]
fn test_f32_nan_bits_roundtrip() {
    let original = f32::NAN;
    let decoded = roundtrip_f32(original);
    assert!(decoded.is_nan(), "decoded value must be NaN");
    assert_eq!(
        decoded.to_bits(),
        original.to_bits(),
        "NaN payload bits must be preserved"
    );
}

// ── test 2: f32::INFINITY roundtrip ──────────────────────────────────────────

#[test]
fn test_f32_positive_infinity_roundtrip() {
    let original = f32::INFINITY;
    let decoded = roundtrip_f32(original);
    assert!(decoded.is_infinite() && decoded.is_sign_positive());
    assert_eq!(decoded.to_bits(), original.to_bits());
}

// ── test 3: f32::NEG_INFINITY roundtrip ──────────────────────────────────────

#[test]
fn test_f32_negative_infinity_roundtrip() {
    let original = f32::NEG_INFINITY;
    let decoded = roundtrip_f32(original);
    assert!(decoded.is_infinite() && decoded.is_sign_negative());
    assert_eq!(decoded.to_bits(), original.to_bits());
}

// ── test 4: f32::MIN roundtrip ───────────────────────────────────────────────

#[test]
fn test_f32_min_roundtrip() {
    let original = f32::MIN;
    let decoded = roundtrip_f32(original);
    assert_eq!(decoded.to_bits(), original.to_bits());
    assert_eq!(decoded, original);
}

// ── test 5: f32::MAX roundtrip ───────────────────────────────────────────────

#[test]
fn test_f32_max_roundtrip() {
    let original = f32::MAX;
    let decoded = roundtrip_f32(original);
    assert_eq!(decoded.to_bits(), original.to_bits());
    assert_eq!(decoded, original);
}

// ── test 6: f32::MIN_POSITIVE roundtrip ──────────────────────────────────────

#[test]
fn test_f32_min_positive_roundtrip() {
    let original = f32::MIN_POSITIVE;
    let decoded = roundtrip_f32(original);
    assert_eq!(decoded.to_bits(), original.to_bits());
    assert_eq!(decoded, original);
}

// ── test 7: f32::EPSILON roundtrip ───────────────────────────────────────────

#[test]
fn test_f32_epsilon_roundtrip() {
    // Use the standard-library constant, not a literal approximation.
    let original = f32::EPSILON;
    let decoded = roundtrip_f32(original);
    assert_eq!(decoded.to_bits(), original.to_bits());
    assert_eq!(decoded, original);
    // Also verify against a well-known mathematical constant ratio to confirm
    // we're testing a meaningful value without introducing literal approximations.
    let _ = f32c::PI; // confirm std::f32::consts is in scope
}

// ── test 8: f32 exact byte format (4-byte little-endian IEEE 754) ────────────

#[test]
fn test_f32_exact_byte_format_little_endian() {
    // Use PI from std consts — no literal approximation.
    let value = f32c::PI;
    let enc = encode_to_vec(&value).expect("encode");
    assert_eq!(enc.len(), 4, "f32 must encode to exactly 4 bytes");
    // Standard config is little-endian; verify byte order matches LE IEEE 754.
    let expected_le = value.to_bits().to_le_bytes();
    assert_eq!(
        enc.as_slice(),
        expected_le.as_slice(),
        "encoded bytes must match little-endian IEEE 754 bit pattern"
    );
}

// ── test 9: f64::NAN roundtrip via to_bits() ─────────────────────────────────

#[test]
fn test_f64_nan_bits_roundtrip() {
    let original = f64::NAN;
    let decoded = roundtrip_f64(original);
    assert!(decoded.is_nan(), "decoded value must be NaN");
    assert_eq!(
        decoded.to_bits(),
        original.to_bits(),
        "NaN payload bits must be preserved"
    );
}

// ── test 10: f64::INFINITY roundtrip ─────────────────────────────────────────

#[test]
fn test_f64_positive_infinity_roundtrip() {
    let original = f64::INFINITY;
    let decoded = roundtrip_f64(original);
    assert!(decoded.is_infinite() && decoded.is_sign_positive());
    assert_eq!(decoded.to_bits(), original.to_bits());
}

// ── test 11: f64::NEG_INFINITY roundtrip ─────────────────────────────────────

#[test]
fn test_f64_negative_infinity_roundtrip() {
    let original = f64::NEG_INFINITY;
    let decoded = roundtrip_f64(original);
    assert!(decoded.is_infinite() && decoded.is_sign_negative());
    assert_eq!(decoded.to_bits(), original.to_bits());
}

// ── test 12: f64::MIN roundtrip ──────────────────────────────────────────────

#[test]
fn test_f64_min_roundtrip() {
    let original = f64::MIN;
    let decoded = roundtrip_f64(original);
    assert_eq!(decoded.to_bits(), original.to_bits());
    assert_eq!(decoded, original);
}

// ── test 13: f64::MAX roundtrip ──────────────────────────────────────────────

#[test]
fn test_f64_max_roundtrip() {
    let original = f64::MAX;
    let decoded = roundtrip_f64(original);
    assert_eq!(decoded.to_bits(), original.to_bits());
    assert_eq!(decoded, original);
}

// ── test 14: f64::MIN_POSITIVE roundtrip ─────────────────────────────────────

#[test]
fn test_f64_min_positive_roundtrip() {
    let original = f64::MIN_POSITIVE;
    let decoded = roundtrip_f64(original);
    assert_eq!(decoded.to_bits(), original.to_bits());
    assert_eq!(decoded, original);
}

// ── test 15: f64::EPSILON roundtrip ──────────────────────────────────────────

#[test]
fn test_f64_epsilon_roundtrip() {
    let original = f64::EPSILON;
    let decoded = roundtrip_f64(original);
    assert_eq!(decoded.to_bits(), original.to_bits());
    assert_eq!(decoded, original);
    let _ = f64c::E; // confirm std::f64::consts is in scope
}

// ── test 16: f64 exact byte format (8-byte little-endian IEEE 754) ───────────

#[test]
fn test_f64_exact_byte_format_little_endian() {
    let value = f64c::TAU;
    let enc = encode_to_vec(&value).expect("encode");
    assert_eq!(enc.len(), 8, "f64 must encode to exactly 8 bytes");
    let expected_le = value.to_bits().to_le_bytes();
    assert_eq!(
        enc.as_slice(),
        expected_le.as_slice(),
        "encoded bytes must match little-endian IEEE 754 bit pattern"
    );
}

// ── test 17: Vec<f32> containing NaN and Inf values ──────────────────────────

#[test]
fn test_vec_f32_nan_inf_roundtrip() {
    let original: Vec<f32> = vec![
        f32::NAN,
        f32::INFINITY,
        f32::NEG_INFINITY,
        f32::MIN,
        f32::MAX,
        f32::MIN_POSITIVE,
        f32c::PI,
        f32c::E,
        f32::EPSILON,
        -f32::EPSILON,
    ];
    let enc = encode_to_vec(&original).expect("encode Vec<f32>");
    let (decoded, _): (Vec<f32>, _) = decode_from_slice(&enc).expect("decode Vec<f32>");

    assert_eq!(
        original.len(),
        decoded.len(),
        "decoded Vec<f32> length must match"
    );
    for (i, (orig, dec)) in original.iter().zip(decoded.iter()).enumerate() {
        assert_eq!(
            orig.to_bits(),
            dec.to_bits(),
            "Vec<f32> element {} bits must match (orig={:?}, dec={:?})",
            i,
            orig,
            dec
        );
    }
}

// ── test 18: Vec<f64> containing NaN and Inf values ──────────────────────────

#[test]
fn test_vec_f64_nan_inf_roundtrip() {
    let original: Vec<f64> = vec![
        f64::NAN,
        f64::INFINITY,
        f64::NEG_INFINITY,
        f64::MIN,
        f64::MAX,
        f64::MIN_POSITIVE,
        f64c::PI,
        f64c::E,
        f64::EPSILON,
        -f64::EPSILON,
    ];
    let enc = encode_to_vec(&original).expect("encode Vec<f64>");
    let (decoded, _): (Vec<f64>, _) = decode_from_slice(&enc).expect("decode Vec<f64>");

    assert_eq!(
        original.len(),
        decoded.len(),
        "decoded Vec<f64> length must match"
    );
    for (i, (orig, dec)) in original.iter().zip(decoded.iter()).enumerate() {
        assert_eq!(
            orig.to_bits(),
            dec.to_bits(),
            "Vec<f64> element {} bits must match (orig={:?}, dec={:?})",
            i,
            orig,
            dec
        );
    }
}

// ── test 19: (f32, f64) tuple roundtrip ──────────────────────────────────────

#[test]
fn test_f32_f64_tuple_roundtrip() {
    // Use mathematical constants from std — no literal approximations.
    let original: (f32, f64) = (f32c::PI, f64c::E);
    let enc = encode_to_vec(&original).expect("encode (f32, f64)");
    let (decoded, _): ((f32, f64), _) = decode_from_slice(&enc).expect("decode (f32, f64)");
    assert_eq!(
        decoded.0.to_bits(),
        original.0.to_bits(),
        "f32 component of tuple must match"
    );
    assert_eq!(
        decoded.1.to_bits(),
        original.1.to_bits(),
        "f64 component of tuple must match"
    );
    // Also verify size: 4 bytes for f32 + 8 bytes for f64 = 12 bytes total.
    assert_eq!(enc.len(), 12, "(f32, f64) tuple must be exactly 12 bytes");
}

// ── test 20: f32 with big_endian config ──────────────────────────────────────

#[test]
fn test_f32_big_endian_config_roundtrip() {
    let value = f32c::SQRT_2;
    let be_config = config::standard().with_big_endian();

    let enc = encode_to_vec_with_config(&value, be_config).expect("big-endian encode f32");
    assert_eq!(
        enc.len(),
        4,
        "f32 must be exactly 4 bytes in big-endian config"
    );

    // Verify the raw bytes match big-endian IEEE 754 representation.
    let expected_be = value.to_bits().to_be_bytes();
    assert_eq!(
        enc.as_slice(),
        expected_be.as_slice(),
        "big-endian encoded bytes must match BE IEEE 754 bit pattern"
    );

    // Full roundtrip using symmetric big-endian config.
    let (decoded, _): (f32, _) =
        decode_from_slice_with_config(&enc, be_config).expect("big-endian decode f32");
    assert_eq!(
        decoded.to_bits(),
        value.to_bits(),
        "f32 big-endian roundtrip must preserve bit pattern"
    );
}

// ── test 21: f64 with big_endian config ──────────────────────────────────────

#[test]
fn test_f64_big_endian_config_roundtrip() {
    let value = f64c::LN_2;
    let be_config = config::standard().with_big_endian();

    let enc = encode_to_vec_with_config(&value, be_config).expect("big-endian encode f64");
    assert_eq!(
        enc.len(),
        8,
        "f64 must be exactly 8 bytes in big-endian config"
    );

    // Verify the raw bytes match big-endian IEEE 754 representation.
    let expected_be = value.to_bits().to_be_bytes();
    assert_eq!(
        enc.as_slice(),
        expected_be.as_slice(),
        "big-endian encoded bytes must match BE IEEE 754 bit pattern"
    );

    // Full roundtrip using symmetric big-endian config.
    let (decoded, _): (f64, _) =
        decode_from_slice_with_config(&enc, be_config).expect("big-endian decode f64");
    assert_eq!(
        decoded.to_bits(),
        value.to_bits(),
        "f64 big-endian roundtrip must preserve bit pattern"
    );
}

// ── test 22: f32 and f64 inside a struct with derive ─────────────────────────

#[derive(Debug, Encode, Decode)]
struct FloatRecord {
    label: u8,
    single: f32,
    double: f64,
    positive_inf_f32: f32,
    negative_inf_f64: f64,
    nan_f32: f32,
    nan_f64: f64,
}

#[test]
fn test_struct_with_f32_f64_derive_roundtrip() {
    let original = FloatRecord {
        label: 42,
        single: f32c::PI,
        double: f64c::PI,
        positive_inf_f32: f32::INFINITY,
        negative_inf_f64: f64::NEG_INFINITY,
        nan_f32: f32::NAN,
        nan_f64: f64::NAN,
    };

    let enc = encode_to_vec(&original).expect("encode FloatRecord");
    let (decoded, consumed): (FloatRecord, _) =
        decode_from_slice(&enc).expect("decode FloatRecord");

    // All bytes consumed.
    assert_eq!(consumed, enc.len(), "all encoded bytes must be consumed");

    // Scalar fields.
    assert_eq!(decoded.label, original.label);
    assert_eq!(
        decoded.single.to_bits(),
        original.single.to_bits(),
        "single (f32 PI) bits must match"
    );
    assert_eq!(
        decoded.double.to_bits(),
        original.double.to_bits(),
        "double (f64 PI) bits must match"
    );

    // Infinity fields.
    assert!(
        decoded.positive_inf_f32.is_infinite() && decoded.positive_inf_f32.is_sign_positive(),
        "positive_inf_f32 must decode as +Inf"
    );
    assert!(
        decoded.negative_inf_f64.is_infinite() && decoded.negative_inf_f64.is_sign_negative(),
        "negative_inf_f64 must decode as -Inf"
    );

    // NaN fields — equality via to_bits() since NaN != NaN by IEEE 754.
    assert!(decoded.nan_f32.is_nan(), "nan_f32 must decode as NaN");
    assert_eq!(
        decoded.nan_f32.to_bits(),
        original.nan_f32.to_bits(),
        "nan_f32 payload bits must be preserved"
    );
    assert!(decoded.nan_f64.is_nan(), "nan_f64 must decode as NaN");
    assert_eq!(
        decoded.nan_f64.to_bits(),
        original.nan_f64.to_bits(),
        "nan_f64 payload bits must be preserved"
    );
}
