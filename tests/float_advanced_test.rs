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

// ── test 1 ──────────────────────────────────────────────────────────────────

#[test]
fn test_f64_positive_infinity_roundtrip() {
    let original: f64 = f64::INFINITY;
    let encoded = encode_to_vec(&original).expect("encode f64::INFINITY failed");
    assert_eq!(encoded.len(), 8, "f64 should be exactly 8 bytes");
    let (decoded, consumed): (f64, _) =
        decode_from_slice(&encoded).expect("decode f64::INFINITY failed");
    assert!(
        decoded.is_infinite() && decoded.is_sign_positive(),
        "decoded value should be +Infinity"
    );
    assert_eq!(consumed, encoded.len());
}

// ── test 2 ──────────────────────────────────────────────────────────────────

#[test]
fn test_f64_negative_infinity_roundtrip() {
    let original: f64 = f64::NEG_INFINITY;
    let encoded = encode_to_vec(&original).expect("encode f64::NEG_INFINITY failed");
    assert_eq!(encoded.len(), 8, "f64 should be exactly 8 bytes");
    let (decoded, consumed): (f64, _) =
        decode_from_slice(&encoded).expect("decode f64::NEG_INFINITY failed");
    assert!(
        decoded.is_infinite() && decoded.is_sign_negative(),
        "decoded value should be -Infinity"
    );
    assert_eq!(consumed, encoded.len());
}

// ── test 3 ──────────────────────────────────────────────────────────────────

#[test]
fn test_f64_nan_roundtrip() {
    let original: f64 = f64::NAN;
    let encoded = encode_to_vec(&original).expect("encode f64::NAN failed");
    assert_eq!(encoded.len(), 8, "f64 should be exactly 8 bytes");
    let (decoded, consumed): (f64, _) =
        decode_from_slice(&encoded).expect("decode f64::NAN failed");
    assert!(decoded.is_nan(), "decoded value should be NaN");
    assert_eq!(consumed, encoded.len());
}

// ── test 4 ──────────────────────────────────────────────────────────────────

#[test]
fn test_f64_positive_zero_roundtrip() {
    let original: f64 = 0.0_f64;
    let encoded = encode_to_vec(&original).expect("encode f64 zero failed");
    assert_eq!(encoded.len(), 8, "f64 should be exactly 8 bytes");
    let (decoded, consumed): (f64, _) =
        decode_from_slice(&encoded).expect("decode f64 zero failed");
    assert_eq!(decoded, 0.0_f64, "decoded value should be 0.0");
    assert!(
        decoded.is_sign_positive(),
        "decoded value should be positive zero"
    );
    assert_eq!(consumed, encoded.len());
}

// ── test 5 ──────────────────────────────────────────────────────────────────

#[test]
fn test_f64_negative_zero_roundtrip() {
    let original: f64 = -0.0_f64;
    let encoded = encode_to_vec(&original).expect("encode f64 negative zero failed");
    assert_eq!(encoded.len(), 8, "f64 should be exactly 8 bytes");
    let (decoded, consumed): (f64, _) =
        decode_from_slice(&encoded).expect("decode f64 negative zero failed");
    assert_eq!(
        decoded, -0.0_f64,
        "decoded value should equal negative zero"
    );
    assert!(
        decoded.is_sign_negative(),
        "decoded value should preserve negative sign of -0.0"
    );
    assert_eq!(consumed, encoded.len());
}

// ── test 6 ──────────────────────────────────────────────────────────────────

#[test]
fn test_f64_min_positive_roundtrip() {
    let original: f64 = f64::MIN_POSITIVE;
    let encoded = encode_to_vec(&original).expect("encode f64::MIN_POSITIVE failed");
    assert_eq!(encoded.len(), 8, "f64 should be exactly 8 bytes");
    let (decoded, consumed): (f64, _) =
        decode_from_slice(&encoded).expect("decode f64::MIN_POSITIVE failed");
    assert_eq!(
        decoded, original,
        "decoded f64::MIN_POSITIVE must equal original"
    );
    assert_eq!(consumed, encoded.len());
}

// ── test 7 ──────────────────────────────────────────────────────────────────

#[test]
fn test_f64_max_roundtrip() {
    let original: f64 = f64::MAX;
    let encoded = encode_to_vec(&original).expect("encode f64::MAX failed");
    assert_eq!(encoded.len(), 8, "f64 should be exactly 8 bytes");
    let (decoded, consumed): (f64, _) =
        decode_from_slice(&encoded).expect("decode f64::MAX failed");
    assert_eq!(decoded, original, "decoded f64::MAX must equal original");
    assert_eq!(consumed, encoded.len());
}

// ── test 8 ──────────────────────────────────────────────────────────────────

#[test]
fn test_f64_min_roundtrip() {
    let original: f64 = f64::MIN;
    let encoded = encode_to_vec(&original).expect("encode f64::MIN failed");
    assert_eq!(encoded.len(), 8, "f64 should be exactly 8 bytes");
    let (decoded, consumed): (f64, _) =
        decode_from_slice(&encoded).expect("decode f64::MIN failed");
    assert_eq!(decoded, original, "decoded f64::MIN must equal original");
    assert_eq!(consumed, encoded.len());
}

// ── test 9 ──────────────────────────────────────────────────────────────────

#[test]
fn test_f32_positive_infinity_roundtrip() {
    let original: f32 = f32::INFINITY;
    let encoded = encode_to_vec(&original).expect("encode f32::INFINITY failed");
    assert_eq!(encoded.len(), 4, "f32 should be exactly 4 bytes");
    let (decoded, consumed): (f32, _) =
        decode_from_slice(&encoded).expect("decode f32::INFINITY failed");
    assert!(
        decoded.is_infinite() && decoded.is_sign_positive(),
        "decoded value should be +Infinity (f32)"
    );
    assert_eq!(consumed, encoded.len());
}

// ── test 10 ─────────────────────────────────────────────────────────────────

#[test]
fn test_f32_nan_roundtrip() {
    let original: f32 = f32::NAN;
    let encoded = encode_to_vec(&original).expect("encode f32::NAN failed");
    assert_eq!(encoded.len(), 4, "f32 should be exactly 4 bytes");
    let (decoded, consumed): (f32, _) =
        decode_from_slice(&encoded).expect("decode f32::NAN failed");
    assert!(decoded.is_nan(), "decoded f32 value should be NaN");
    assert_eq!(consumed, encoded.len());
}

// ── test 11 ─────────────────────────────────────────────────────────────────

#[test]
fn test_f32_zero_roundtrip() {
    let original: f32 = 0.0_f32;
    let encoded = encode_to_vec(&original).expect("encode f32 zero failed");
    assert_eq!(encoded.len(), 4, "f32 should be exactly 4 bytes");
    let (decoded, consumed): (f32, _) =
        decode_from_slice(&encoded).expect("decode f32 zero failed");
    assert_eq!(decoded, 0.0_f32, "decoded f32 zero must equal original");
    assert_eq!(consumed, encoded.len());
}

// ── test 12 ─────────────────────────────────────────────────────────────────

#[test]
fn test_f32_max_roundtrip() {
    let original: f32 = f32::MAX;
    let encoded = encode_to_vec(&original).expect("encode f32::MAX failed");
    assert_eq!(encoded.len(), 4, "f32 should be exactly 4 bytes");
    let (decoded, consumed): (f32, _) =
        decode_from_slice(&encoded).expect("decode f32::MAX failed");
    assert_eq!(decoded, original, "decoded f32::MAX must equal original");
    assert_eq!(consumed, encoded.len());
}

// ── test 13 ─────────────────────────────────────────────────────────────────

#[test]
fn test_f64_math_constants_roundtrip() {
    let pi: f64 = std::f64::consts::PI;
    let e: f64 = std::f64::consts::E;
    let sqrt2: f64 = std::f64::consts::SQRT_2;

    for (label, original) in [("PI", pi), ("E", e), ("SQRT_2", sqrt2)] {
        let encoded = encode_to_vec(&original).expect("encode f64 constant failed");
        assert_eq!(encoded.len(), 8, "f64 {} should be exactly 8 bytes", label);
        let (decoded, consumed): (f64, _) =
            decode_from_slice(&encoded).expect("decode f64 constant failed");
        assert_eq!(
            decoded, original,
            "roundtrip of f64::{} must be bit-identical",
            label
        );
        assert_eq!(consumed, encoded.len());
    }
}

// ── test 14 ─────────────────────────────────────────────────────────────────

#[test]
fn test_f32_math_constants_roundtrip() {
    let pi: f32 = std::f32::consts::PI;
    let e: f32 = std::f32::consts::E;

    for (label, original) in [("PI", pi), ("E", e)] {
        let encoded = encode_to_vec(&original).expect("encode f32 constant failed");
        assert_eq!(encoded.len(), 4, "f32 {} should be exactly 4 bytes", label);
        let (decoded, consumed): (f32, _) =
            decode_from_slice(&encoded).expect("decode f32 constant failed");
        assert_eq!(
            decoded, original,
            "roundtrip of f32::{} must be bit-identical",
            label
        );
        assert_eq!(consumed, encoded.len());
    }
}

// ── test 15 ─────────────────────────────────────────────────────────────────

#[test]
fn test_vec_f64_special_values() {
    let original: Vec<f64> = vec![f64::INFINITY, f64::NEG_INFINITY, f64::NAN, 0.0, 1.0];
    let encoded = encode_to_vec(&original).expect("encode Vec<f64> special failed");
    let (decoded, _): (Vec<f64>, _) =
        decode_from_slice(&encoded).expect("decode Vec<f64> special failed");
    assert_eq!(decoded.len(), 5, "decoded Vec should have 5 elements");
    assert!(
        decoded[0].is_infinite() && decoded[0].is_sign_positive(),
        "element 0 should be +Infinity"
    );
    assert!(
        decoded[1].is_infinite() && decoded[1].is_sign_negative(),
        "element 1 should be -Infinity"
    );
    assert!(decoded[2].is_nan(), "element 2 should be NaN");
    assert_eq!(decoded[3], 0.0_f64, "element 3 should be 0.0");
    assert_eq!(decoded[4], 1.0_f64, "element 4 should be 1.0");
}

// ── test 16 ─────────────────────────────────────────────────────────────────

#[test]
fn test_f64_byte_size_is_always_8() {
    let values: &[f64] = &[
        0.0,
        -0.0,
        1.0,
        -1.0,
        f64::MAX,
        f64::MIN,
        f64::MIN_POSITIVE,
        f64::INFINITY,
        f64::NEG_INFINITY,
        std::f64::consts::PI,
        std::f64::consts::E,
        std::f64::consts::SQRT_2,
    ];
    for &v in values {
        let encoded = encode_to_vec(&v).expect("encode f64 failed");
        assert_eq!(
            encoded.len(),
            8,
            "f64 value {:?} must encode to exactly 8 bytes",
            v
        );
    }
    // NaN requires separate handling due to NaN != NaN
    let nan_encoded = encode_to_vec(&f64::NAN).expect("encode f64::NAN failed");
    assert_eq!(
        nan_encoded.len(),
        8,
        "f64::NAN must encode to exactly 8 bytes"
    );
}

// ── test 17 ─────────────────────────────────────────────────────────────────

#[test]
fn test_f32_byte_size_is_always_4() {
    let values: &[f32] = &[
        0.0,
        -0.0,
        1.0,
        -1.0,
        f32::MAX,
        f32::MIN,
        f32::MIN_POSITIVE,
        f32::INFINITY,
        f32::NEG_INFINITY,
        std::f32::consts::PI,
        std::f32::consts::E,
    ];
    for &v in values {
        let encoded = encode_to_vec(&v).expect("encode f32 failed");
        assert_eq!(
            encoded.len(),
            4,
            "f32 value {:?} must encode to exactly 4 bytes",
            v
        );
    }
    // NaN requires separate handling due to NaN != NaN
    let nan_encoded = encode_to_vec(&f32::NAN).expect("encode f32::NAN failed");
    assert_eq!(
        nan_encoded.len(),
        4,
        "f32::NAN must encode to exactly 4 bytes"
    );
}

// ── test 18 ─────────────────────────────────────────────────────────────────

#[test]
fn test_f64_big_endian_config_roundtrip() {
    let original: f64 = std::f64::consts::PI;
    let cfg = config::standard().with_big_endian();
    let encoded = encode_to_vec_with_config(&original, cfg).expect("encode f64 big-endian failed");
    assert_eq!(
        encoded.len(),
        8,
        "f64 with big-endian config should still be 8 bytes"
    );
    let (decoded, consumed): (f64, _) =
        decode_from_slice_with_config(&encoded, cfg).expect("decode f64 big-endian failed");
    assert_eq!(
        decoded, original,
        "f64 big-endian roundtrip must be bit-identical"
    );
    assert_eq!(consumed, encoded.len());
}

// ── test 19 ─────────────────────────────────────────────────────────────────

#[test]
fn test_f64_fixed_int_encoding_config_roundtrip() {
    let original: f64 = std::f64::consts::E;
    let cfg = config::standard().with_fixed_int_encoding();
    let encoded =
        encode_to_vec_with_config(&original, cfg).expect("encode f64 fixed-int config failed");
    assert_eq!(
        encoded.len(),
        8,
        "f64 with fixed-int config should still be 8 bytes"
    );
    let (decoded, consumed): (f64, _) =
        decode_from_slice_with_config(&encoded, cfg).expect("decode f64 fixed-int config failed");
    assert_eq!(
        decoded, original,
        "f64 fixed-int config roundtrip must be bit-identical"
    );
    assert_eq!(consumed, encoded.len());
}

// ── test 20 ─────────────────────────────────────────────────────────────────

#[test]
fn test_f32_f64_tuple_roundtrip() {
    let original: (f32, f64) = (std::f32::consts::PI, std::f64::consts::E);
    let encoded = encode_to_vec(&original).expect("encode (f32, f64) tuple failed");
    assert_eq!(
        encoded.len(),
        12,
        "(f32, f64) tuple should encode to exactly 12 bytes (4 + 8)"
    );
    let (decoded, consumed): ((f32, f64), _) =
        decode_from_slice(&encoded).expect("decode (f32, f64) tuple failed");
    assert_eq!(
        decoded.0, original.0,
        "tuple f32 field must roundtrip correctly"
    );
    assert_eq!(
        decoded.1, original.1,
        "tuple f64 field must roundtrip correctly"
    );
    assert_eq!(consumed, encoded.len());
}

// ── test 21 ─────────────────────────────────────────────────────────────────

#[derive(Debug, PartialEq, Encode, Decode)]
struct FloatPair {
    single: f32,
    double: f64,
}

#[test]
fn test_struct_with_f32_and_f64_roundtrip() {
    let original = FloatPair {
        single: std::f32::consts::PI,
        double: std::f64::consts::SQRT_2,
    };
    let encoded = encode_to_vec(&original).expect("encode FloatPair failed");
    assert_eq!(
        encoded.len(),
        12,
        "FloatPair should encode to exactly 12 bytes (4 + 8)"
    );
    let (decoded, consumed): (FloatPair, _) =
        decode_from_slice(&encoded).expect("decode FloatPair failed");
    assert_eq!(
        decoded, original,
        "FloatPair roundtrip must be bit-identical"
    );
    assert_eq!(consumed, encoded.len());
}

// ── test 22 ─────────────────────────────────────────────────────────────────

#[test]
fn test_option_f64_nan_roundtrip() {
    let original: Option<f64> = Some(f64::NAN);
    let encoded = encode_to_vec(&original).expect("encode Option<f64> NaN failed");
    let (decoded, consumed): (Option<f64>, _) =
        decode_from_slice(&encoded).expect("decode Option<f64> NaN failed");
    match decoded {
        Some(v) => assert!(v.is_nan(), "decoded Option<f64> inner value should be NaN"),
        None => panic!("decoded Option<f64> should be Some, not None"),
    }
    assert_eq!(consumed, encoded.len());
}
