//! Advanced floating-point edge case tests for OxiCode.
//!
//! Covers special IEEE 754 values (infinities, NaN, signed zero, extremes)
//! and mathematical constants, verifying bit-exact roundtrip behaviour as well
//! as encoded size expectations under the fixed-int configuration.

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

// ---------------------------------------------------------------------------
// 1. f32::INFINITY roundtrip — bit-exact comparison
// ---------------------------------------------------------------------------

#[test]
fn test_f32_infinity_roundtrip_bits() {
    let original = f32::INFINITY;
    let encoded = encode_to_vec(&original).expect("encode f32::INFINITY");
    let (decoded, _consumed): (f32, usize) =
        decode_from_slice(&encoded).expect("decode f32::INFINITY");
    assert_eq!(
        decoded.to_bits(),
        original.to_bits(),
        "bits must match exactly for f32::INFINITY"
    );
}

// ---------------------------------------------------------------------------
// 2. f32::NEG_INFINITY roundtrip — bit-exact comparison
// ---------------------------------------------------------------------------

#[test]
fn test_f32_neg_infinity_roundtrip_bits() {
    let original = f32::NEG_INFINITY;
    let encoded = encode_to_vec(&original).expect("encode f32::NEG_INFINITY");
    let (decoded, _consumed): (f32, usize) =
        decode_from_slice(&encoded).expect("decode f32::NEG_INFINITY");
    assert_eq!(
        decoded.to_bits(),
        original.to_bits(),
        "bits must match exactly for f32::NEG_INFINITY"
    );
}

// ---------------------------------------------------------------------------
// 3. f32::NAN roundtrip — bit-exact comparison via f32::to_bits
// ---------------------------------------------------------------------------

#[test]
fn test_f32_nan_roundtrip_bits() {
    let original = f32::NAN;
    let encoded = encode_to_vec(&original).expect("encode f32::NAN");
    let (decoded, _consumed): (f32, usize) = decode_from_slice(&encoded).expect("decode f32::NAN");
    assert_eq!(
        decoded.to_bits(),
        original.to_bits(),
        "bits must match exactly for f32::NAN"
    );
}

// ---------------------------------------------------------------------------
// 4. f32::MAX roundtrip
// ---------------------------------------------------------------------------

#[test]
fn test_f32_max_roundtrip() {
    let original = f32::MAX;
    let encoded = encode_to_vec(&original).expect("encode f32::MAX");
    let (decoded, _consumed): (f32, usize) = decode_from_slice(&encoded).expect("decode f32::MAX");
    assert_eq!(
        decoded.to_bits(),
        original.to_bits(),
        "bits must match exactly for f32::MAX"
    );
}

// ---------------------------------------------------------------------------
// 5. f32::MIN roundtrip (most negative finite f32)
// ---------------------------------------------------------------------------

#[test]
fn test_f32_min_roundtrip() {
    let original = f32::MIN;
    let encoded = encode_to_vec(&original).expect("encode f32::MIN");
    let (decoded, _consumed): (f32, usize) = decode_from_slice(&encoded).expect("decode f32::MIN");
    assert_eq!(
        decoded.to_bits(),
        original.to_bits(),
        "bits must match exactly for f32::MIN"
    );
}

// ---------------------------------------------------------------------------
// 6. f32::MIN_POSITIVE roundtrip
// ---------------------------------------------------------------------------

#[test]
fn test_f32_min_positive_roundtrip() {
    let original = f32::MIN_POSITIVE;
    let encoded = encode_to_vec(&original).expect("encode f32::MIN_POSITIVE");
    let (decoded, _consumed): (f32, usize) =
        decode_from_slice(&encoded).expect("decode f32::MIN_POSITIVE");
    assert_eq!(
        decoded.to_bits(),
        original.to_bits(),
        "bits must match exactly for f32::MIN_POSITIVE"
    );
}

// ---------------------------------------------------------------------------
// 7. f32 positive zero (+0.0) roundtrip
// ---------------------------------------------------------------------------

#[test]
fn test_f32_positive_zero_roundtrip() {
    let original = 0.0f32;
    let encoded = encode_to_vec(&original).expect("encode f32 +0.0");
    let (decoded, _consumed): (f32, usize) = decode_from_slice(&encoded).expect("decode f32 +0.0");
    assert_eq!(
        decoded.to_bits(),
        original.to_bits(),
        "bits must match exactly for f32 +0.0"
    );
}

// ---------------------------------------------------------------------------
// 8. f32 negative zero (-0.0) roundtrip — bit-exact comparison
// ---------------------------------------------------------------------------

#[test]
fn test_f32_negative_zero_roundtrip_bits() {
    let original = -0.0f32;
    let encoded = encode_to_vec(&original).expect("encode f32 -0.0");
    let (decoded, _consumed): (f32, usize) = decode_from_slice(&encoded).expect("decode f32 -0.0");
    assert_eq!(
        decoded.to_bits(),
        original.to_bits(),
        "bits must match exactly for f32 -0.0"
    );
    // Ensure -0.0 and +0.0 have distinct bit patterns
    assert_ne!(
        (-0.0f32).to_bits(),
        0.0f32.to_bits(),
        "-0.0 and +0.0 must have distinct bit patterns"
    );
}

// ---------------------------------------------------------------------------
// 9. f64::INFINITY roundtrip — bit-exact comparison
// ---------------------------------------------------------------------------

#[test]
fn test_f64_infinity_roundtrip_bits() {
    let original = f64::INFINITY;
    let encoded = encode_to_vec(&original).expect("encode f64::INFINITY");
    let (decoded, _consumed): (f64, usize) =
        decode_from_slice(&encoded).expect("decode f64::INFINITY");
    assert_eq!(
        decoded.to_bits(),
        original.to_bits(),
        "bits must match exactly for f64::INFINITY"
    );
}

// ---------------------------------------------------------------------------
// 10. f64::NEG_INFINITY roundtrip — bit-exact comparison
// ---------------------------------------------------------------------------

#[test]
fn test_f64_neg_infinity_roundtrip_bits() {
    let original = f64::NEG_INFINITY;
    let encoded = encode_to_vec(&original).expect("encode f64::NEG_INFINITY");
    let (decoded, _consumed): (f64, usize) =
        decode_from_slice(&encoded).expect("decode f64::NEG_INFINITY");
    assert_eq!(
        decoded.to_bits(),
        original.to_bits(),
        "bits must match exactly for f64::NEG_INFINITY"
    );
}

// ---------------------------------------------------------------------------
// 11. f64::NAN roundtrip — bit-exact comparison via f64::to_bits
// ---------------------------------------------------------------------------

#[test]
fn test_f64_nan_roundtrip_bits() {
    let original = f64::NAN;
    let encoded = encode_to_vec(&original).expect("encode f64::NAN");
    let (decoded, _consumed): (f64, usize) = decode_from_slice(&encoded).expect("decode f64::NAN");
    assert_eq!(
        decoded.to_bits(),
        original.to_bits(),
        "bits must match exactly for f64::NAN"
    );
}

// ---------------------------------------------------------------------------
// 12. f64::MAX roundtrip
// ---------------------------------------------------------------------------

#[test]
fn test_f64_max_roundtrip() {
    let original = f64::MAX;
    let encoded = encode_to_vec(&original).expect("encode f64::MAX");
    let (decoded, _consumed): (f64, usize) = decode_from_slice(&encoded).expect("decode f64::MAX");
    assert_eq!(
        decoded.to_bits(),
        original.to_bits(),
        "bits must match exactly for f64::MAX"
    );
}

// ---------------------------------------------------------------------------
// 13. f64::MIN roundtrip (most negative finite f64)
// ---------------------------------------------------------------------------

#[test]
fn test_f64_min_roundtrip() {
    let original = f64::MIN;
    let encoded = encode_to_vec(&original).expect("encode f64::MIN");
    let (decoded, _consumed): (f64, usize) = decode_from_slice(&encoded).expect("decode f64::MIN");
    assert_eq!(
        decoded.to_bits(),
        original.to_bits(),
        "bits must match exactly for f64::MIN"
    );
}

// ---------------------------------------------------------------------------
// 14. f64::MIN_POSITIVE roundtrip
// ---------------------------------------------------------------------------

#[test]
fn test_f64_min_positive_roundtrip() {
    let original = f64::MIN_POSITIVE;
    let encoded = encode_to_vec(&original).expect("encode f64::MIN_POSITIVE");
    let (decoded, _consumed): (f64, usize) =
        decode_from_slice(&encoded).expect("decode f64::MIN_POSITIVE");
    assert_eq!(
        decoded.to_bits(),
        original.to_bits(),
        "bits must match exactly for f64::MIN_POSITIVE"
    );
}

// ---------------------------------------------------------------------------
// 15. f64 positive zero (+0.0) roundtrip
// ---------------------------------------------------------------------------

#[test]
fn test_f64_positive_zero_roundtrip() {
    let original = 0.0f64;
    let encoded = encode_to_vec(&original).expect("encode f64 +0.0");
    let (decoded, _consumed): (f64, usize) = decode_from_slice(&encoded).expect("decode f64 +0.0");
    assert_eq!(
        decoded.to_bits(),
        original.to_bits(),
        "bits must match exactly for f64 +0.0"
    );
}

// ---------------------------------------------------------------------------
// 16. f64 negative zero (-0.0) roundtrip — bit-exact comparison
// ---------------------------------------------------------------------------

#[test]
fn test_f64_negative_zero_roundtrip_bits() {
    let original = -0.0f64;
    let encoded = encode_to_vec(&original).expect("encode f64 -0.0");
    let (decoded, _consumed): (f64, usize) = decode_from_slice(&encoded).expect("decode f64 -0.0");
    assert_eq!(
        decoded.to_bits(),
        original.to_bits(),
        "bits must match exactly for f64 -0.0"
    );
    // Ensure -0.0 and +0.0 have distinct bit patterns
    assert_ne!(
        (-0.0f64).to_bits(),
        0.0f64.to_bits(),
        "-0.0 and +0.0 must have distinct bit patterns"
    );
}

// ---------------------------------------------------------------------------
// 17. std::f64::consts::PI roundtrip — bit-exact comparison
// ---------------------------------------------------------------------------

#[test]
fn test_f64_pi_roundtrip_bits() {
    let original = std::f64::consts::PI;
    let encoded = encode_to_vec(&original).expect("encode PI");
    let (decoded, _consumed): (f64, usize) = decode_from_slice(&encoded).expect("decode PI");
    assert_eq!(
        decoded.to_bits(),
        original.to_bits(),
        "bits must match exactly for std::f64::consts::PI"
    );
}

// ---------------------------------------------------------------------------
// 18. std::f64::consts::E roundtrip — bit-exact comparison
// ---------------------------------------------------------------------------

#[test]
fn test_f64_e_roundtrip_bits() {
    let original = std::f64::consts::E;
    let encoded = encode_to_vec(&original).expect("encode E");
    let (decoded, _consumed): (f64, usize) = decode_from_slice(&encoded).expect("decode E");
    assert_eq!(
        decoded.to_bits(),
        original.to_bits(),
        "bits must match exactly for std::f64::consts::E"
    );
}

// ---------------------------------------------------------------------------
// 19. std::f64::consts::SQRT_2 roundtrip — bit-exact comparison
// ---------------------------------------------------------------------------

#[test]
fn test_f64_sqrt2_roundtrip_bits() {
    let original = std::f64::consts::SQRT_2;
    let encoded = encode_to_vec(&original).expect("encode SQRT_2");
    let (decoded, _consumed): (f64, usize) = decode_from_slice(&encoded).expect("decode SQRT_2");
    assert_eq!(
        decoded.to_bits(),
        original.to_bits(),
        "bits must match exactly for std::f64::consts::SQRT_2"
    );
}

// ---------------------------------------------------------------------------
// 20. Vec<f64> with NaN and Inf values — bit-exact element-wise comparison
// ---------------------------------------------------------------------------

#[test]
fn test_vec_f64_nan_inf_roundtrip_bits() {
    let original: Vec<f64> = vec![
        f64::NAN,
        f64::INFINITY,
        f64::NEG_INFINITY,
        0.0f64,
        -0.0f64,
        f64::MAX,
        f64::MIN,
        f64::MIN_POSITIVE,
        std::f64::consts::PI,
        std::f64::consts::E,
    ];
    let encoded = encode_to_vec(&original).expect("encode Vec<f64> with special values");
    assert!(!encoded.is_empty(), "encoded bytes must not be empty");
    let (decoded, consumed): (Vec<f64>, usize) =
        decode_from_slice(&encoded).expect("decode Vec<f64> with special values");
    assert_eq!(consumed, encoded.len(), "all bytes must be consumed");
    assert_eq!(
        decoded.len(),
        original.len(),
        "decoded Vec must have same length"
    );
    for (i, (orig, dec)) in original.iter().zip(decoded.iter()).enumerate() {
        assert_eq!(
            dec.to_bits(),
            orig.to_bits(),
            "element {} bits must match exactly",
            i
        );
    }
}

// ---------------------------------------------------------------------------
// 21. f32 with fixed-int config — encoded to exactly 4 bytes
// ---------------------------------------------------------------------------

#[test]
fn test_f32_fixed_int_config_exact_size() {
    let cfg = config::standard().with_fixed_int_encoding();
    let values = [
        f32::INFINITY,
        f32::NEG_INFINITY,
        f32::NAN,
        f32::MAX,
        f32::MIN,
        f32::MIN_POSITIVE,
        0.0f32,
        -0.0f32,
    ];
    for original in values {
        let encoded =
            encode_to_vec_with_config(&original, cfg).expect("encode f32 with fixed-int config");
        assert_eq!(
            encoded.len(),
            4,
            "f32 must always encode to exactly 4 bytes with fixed-int config; value bits={}",
            original.to_bits()
        );
        let (decoded, consumed): (f32, usize) =
            decode_from_slice_with_config(&encoded, cfg).expect("decode f32 with fixed-int config");
        assert_eq!(consumed, 4, "exactly 4 bytes must be consumed for f32");
        assert_eq!(
            decoded.to_bits(),
            original.to_bits(),
            "bits must match exactly for f32 with fixed-int config; value bits={}",
            original.to_bits()
        );
    }
}

// ---------------------------------------------------------------------------
// 22. f64 with fixed-int config — encoded to exactly 8 bytes
// ---------------------------------------------------------------------------

#[test]
fn test_f64_fixed_int_config_exact_size() {
    let cfg = config::standard().with_fixed_int_encoding();
    let values = [
        f64::INFINITY,
        f64::NEG_INFINITY,
        f64::NAN,
        f64::MAX,
        f64::MIN,
        f64::MIN_POSITIVE,
        0.0f64,
        -0.0f64,
        std::f64::consts::PI,
        std::f64::consts::E,
        std::f64::consts::SQRT_2,
    ];
    for original in values {
        let encoded =
            encode_to_vec_with_config(&original, cfg).expect("encode f64 with fixed-int config");
        assert_eq!(
            encoded.len(),
            8,
            "f64 must always encode to exactly 8 bytes with fixed-int config; value bits={}",
            original.to_bits()
        );
        let (decoded, consumed): (f64, usize) =
            decode_from_slice_with_config(&encoded, cfg).expect("decode f64 with fixed-int config");
        assert_eq!(consumed, 8, "exactly 8 bytes must be consumed for f64");
        assert_eq!(
            decoded.to_bits(),
            original.to_bits(),
            "bits must match exactly for f64 with fixed-int config; value bits={}",
            original.to_bits()
        );
    }
}
