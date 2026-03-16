//! Advanced float type tests: scientific measurements with floating point precision

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

#[derive(Debug, PartialEq, Encode, Decode)]
struct Measurement {
    value: f64,
    uncertainty: f64,
    unit: String,
}

#[derive(Debug, PartialEq, Encode, Decode)]
struct Spectrum {
    frequencies: Vec<f32>,
    amplitudes: Vec<f32>,
    peak_freq: f32,
    resolution_hz: f64,
}

// Test 1: f32 positive normal value roundtrip
#[test]
fn test_f32_positive_normal_roundtrip() {
    let val: f32 = 3.14159_f32;
    let bytes = encode_to_vec(&val).expect("Failed to encode f32 positive normal");
    let (decoded, _): (f32, usize) =
        decode_from_slice(&bytes).expect("Failed to decode f32 positive normal");
    assert_eq!(val, decoded);
}

// Test 2: f32 negative normal value roundtrip
#[test]
fn test_f32_negative_normal_roundtrip() {
    let val: f32 = -2.71828_f32;
    let bytes = encode_to_vec(&val).expect("Failed to encode f32 negative normal");
    let (decoded, _): (f32, usize) =
        decode_from_slice(&bytes).expect("Failed to decode f32 negative normal");
    assert_eq!(val, decoded);
}

// Test 3: f32 zero roundtrip
#[test]
fn test_f32_zero_roundtrip() {
    let val: f32 = 0.0_f32;
    let bytes = encode_to_vec(&val).expect("Failed to encode f32 zero");
    let (decoded, _): (f32, usize) = decode_from_slice(&bytes).expect("Failed to decode f32 zero");
    assert_eq!(val, decoded);
}

// Test 4: f32 -0.0 roundtrip (compare via to_bits)
#[test]
fn test_f32_negative_zero_roundtrip() {
    let val: f32 = -0.0_f32;
    let bytes = encode_to_vec(&val).expect("Failed to encode f32 negative zero");
    let (decoded, _): (f32, usize) =
        decode_from_slice(&bytes).expect("Failed to decode f32 negative zero");
    assert_eq!(val.to_bits(), decoded.to_bits());
}

// Test 5: f32 positive infinity roundtrip (compare via to_bits)
#[test]
fn test_f32_positive_infinity_roundtrip() {
    let val: f32 = f32::INFINITY;
    let bytes = encode_to_vec(&val).expect("Failed to encode f32 positive infinity");
    let (decoded, _): (f32, usize) =
        decode_from_slice(&bytes).expect("Failed to decode f32 positive infinity");
    assert_eq!(val.to_bits(), decoded.to_bits());
}

// Test 6: f32 negative infinity roundtrip (compare via to_bits)
#[test]
fn test_f32_negative_infinity_roundtrip() {
    let val: f32 = f32::NEG_INFINITY;
    let bytes = encode_to_vec(&val).expect("Failed to encode f32 negative infinity");
    let (decoded, _): (f32, usize) =
        decode_from_slice(&bytes).expect("Failed to decode f32 negative infinity");
    assert_eq!(val.to_bits(), decoded.to_bits());
}

// Test 7: f32 NaN roundtrip (compare via to_bits)
#[test]
fn test_f32_nan_roundtrip() {
    let val: f32 = f32::NAN;
    let bytes = encode_to_vec(&val).expect("Failed to encode f32 NaN");
    let (decoded, _): (f32, usize) = decode_from_slice(&bytes).expect("Failed to decode f32 NaN");
    assert_eq!(val.to_bits(), decoded.to_bits());
}

// Test 8: f32 MIN and MAX roundtrip
#[test]
fn test_f32_min_max_roundtrip() {
    let min_val: f32 = f32::MIN;
    let max_val: f32 = f32::MAX;

    let min_bytes = encode_to_vec(&min_val).expect("Failed to encode f32 MIN");
    let (decoded_min, _): (f32, usize) =
        decode_from_slice(&min_bytes).expect("Failed to decode f32 MIN");
    assert_eq!(min_val, decoded_min);

    let max_bytes = encode_to_vec(&max_val).expect("Failed to encode f32 MAX");
    let (decoded_max, _): (f32, usize) =
        decode_from_slice(&max_bytes).expect("Failed to decode f32 MAX");
    assert_eq!(max_val, decoded_max);
}

// Test 9: f64 positive normal value roundtrip
#[test]
fn test_f64_positive_normal_roundtrip() {
    let val: f64 = 2.998e8_f64; // speed of light m/s
    let bytes = encode_to_vec(&val).expect("Failed to encode f64 positive normal");
    let (decoded, _): (f64, usize) =
        decode_from_slice(&bytes).expect("Failed to decode f64 positive normal");
    assert_eq!(val, decoded);
}

// Test 10: f64 negative normal value roundtrip
#[test]
fn test_f64_negative_normal_roundtrip() {
    let val: f64 = -1.380649e-23_f64; // negative Boltzmann constant
    let bytes = encode_to_vec(&val).expect("Failed to encode f64 negative normal");
    let (decoded, _): (f64, usize) =
        decode_from_slice(&bytes).expect("Failed to decode f64 negative normal");
    assert_eq!(val, decoded);
}

// Test 11: f64 zero roundtrip
#[test]
fn test_f64_zero_roundtrip() {
    let val: f64 = 0.0_f64;
    let bytes = encode_to_vec(&val).expect("Failed to encode f64 zero");
    let (decoded, _): (f64, usize) = decode_from_slice(&bytes).expect("Failed to decode f64 zero");
    assert_eq!(val, decoded);
}

// Test 12: f64 -0.0 roundtrip (compare via to_bits)
#[test]
fn test_f64_negative_zero_roundtrip() {
    let val: f64 = -0.0_f64;
    let bytes = encode_to_vec(&val).expect("Failed to encode f64 negative zero");
    let (decoded, _): (f64, usize) =
        decode_from_slice(&bytes).expect("Failed to decode f64 negative zero");
    assert_eq!(val.to_bits(), decoded.to_bits());
}

// Test 13: f64 positive infinity roundtrip (compare via to_bits)
#[test]
fn test_f64_positive_infinity_roundtrip() {
    let val: f64 = f64::INFINITY;
    let bytes = encode_to_vec(&val).expect("Failed to encode f64 positive infinity");
    let (decoded, _): (f64, usize) =
        decode_from_slice(&bytes).expect("Failed to decode f64 positive infinity");
    assert_eq!(val.to_bits(), decoded.to_bits());
}

// Test 14: f64 NaN roundtrip (compare via to_bits)
#[test]
fn test_f64_nan_roundtrip() {
    let val: f64 = f64::NAN;
    let bytes = encode_to_vec(&val).expect("Failed to encode f64 NaN");
    let (decoded, _): (f64, usize) = decode_from_slice(&bytes).expect("Failed to decode f64 NaN");
    assert_eq!(val.to_bits(), decoded.to_bits());
}

// Test 15: f64 MIN and MAX roundtrip
#[test]
fn test_f64_min_max_roundtrip() {
    let min_val: f64 = f64::MIN;
    let max_val: f64 = f64::MAX;

    let min_bytes = encode_to_vec(&min_val).expect("Failed to encode f64 MIN");
    let (decoded_min, _): (f64, usize) =
        decode_from_slice(&min_bytes).expect("Failed to decode f64 MIN");
    assert_eq!(min_val, decoded_min);

    let max_bytes = encode_to_vec(&max_val).expect("Failed to encode f64 MAX");
    let (decoded_max, _): (f64, usize) =
        decode_from_slice(&max_bytes).expect("Failed to decode f64 MAX");
    assert_eq!(max_val, decoded_max);
}

// Test 16: Measurement struct roundtrip (normal values)
#[test]
fn test_measurement_normal_roundtrip() {
    let measurement = Measurement {
        value: 9.80665_f64,
        uncertainty: 0.00001_f64,
        unit: String::from("m/s^2"),
    };
    let bytes = encode_to_vec(&measurement).expect("Failed to encode Measurement");
    let (decoded, _): (Measurement, usize) =
        decode_from_slice(&bytes).expect("Failed to decode Measurement");
    assert_eq!(measurement, decoded);
}

// Test 17: Measurement with infinite uncertainty (compare via to_bits)
#[test]
fn test_measurement_infinite_uncertainty_roundtrip() {
    let measurement = Measurement {
        value: 1.0_f64,
        uncertainty: f64::INFINITY,
        unit: String::from("unknown"),
    };
    let bytes = encode_to_vec(&measurement)
        .expect("Failed to encode Measurement with infinite uncertainty");
    let (decoded, _): (Measurement, usize) =
        decode_from_slice(&bytes).expect("Failed to decode Measurement with infinite uncertainty");
    assert_eq!(measurement.value, decoded.value);
    assert_eq!(
        measurement.uncertainty.to_bits(),
        decoded.uncertainty.to_bits()
    );
    assert_eq!(measurement.unit, decoded.unit);
}

// Test 18: Spectrum roundtrip (empty vecs)
#[test]
fn test_spectrum_empty_roundtrip() {
    let spectrum = Spectrum {
        frequencies: vec![],
        amplitudes: vec![],
        peak_freq: 0.0_f32,
        resolution_hz: 1.0_f64,
    };
    let bytes = encode_to_vec(&spectrum).expect("Failed to encode empty Spectrum");
    let (decoded, _): (Spectrum, usize) =
        decode_from_slice(&bytes).expect("Failed to decode empty Spectrum");
    assert_eq!(spectrum, decoded);
}

// Test 19: Spectrum roundtrip (5 frequencies and amplitudes)
#[test]
fn test_spectrum_five_elements_roundtrip() {
    let spectrum = Spectrum {
        frequencies: vec![440.0_f32, 880.0_f32, 1320.0_f32, 1760.0_f32, 2200.0_f32],
        amplitudes: vec![1.0_f32, 0.5_f32, 0.333_f32, 0.25_f32, 0.2_f32],
        peak_freq: 440.0_f32,
        resolution_hz: 0.5_f64,
    };
    let bytes = encode_to_vec(&spectrum).expect("Failed to encode Spectrum with 5 elements");
    let (decoded, _): (Spectrum, usize) =
        decode_from_slice(&bytes).expect("Failed to decode Spectrum with 5 elements");
    assert_eq!(spectrum, decoded);
}

// Test 20: Vec<f32> with boundary values roundtrip
#[test]
fn test_vec_f32_boundary_values_roundtrip() {
    let values: Vec<f32> = vec![
        f32::MIN,
        f32::MIN_POSITIVE,
        -1.0_f32,
        0.0_f32,
        1.0_f32,
        f32::MAX,
    ];
    let bytes = encode_to_vec(&values).expect("Failed to encode Vec<f32> boundary values");
    let (decoded, _): (Vec<f32>, usize) =
        decode_from_slice(&bytes).expect("Failed to decode Vec<f32> boundary values");
    assert_eq!(values.len(), decoded.len());
    for (original, dec) in values.iter().zip(decoded.iter()) {
        assert_eq!(original.to_bits(), dec.to_bits());
    }
}

// Test 21: Big-endian config f64 roundtrip
#[test]
fn test_f64_big_endian_config_roundtrip() {
    let val: f64 = 6.674e-11_f64; // gravitational constant
    let cfg = config::standard().with_big_endian();
    let bytes = encode_to_vec_with_config(&val, cfg).expect("Failed to encode f64 big-endian");
    let (decoded, _): (f64, usize) =
        decode_from_slice_with_config(&bytes, cfg).expect("Failed to decode f64 big-endian");
    assert_eq!(val, decoded);
}

// Test 22: Fixed-int encoding of Measurement struct (verify roundtrip works correctly)
#[test]
fn test_measurement_fixed_int_encoding_roundtrip() {
    let measurement = Measurement {
        value: 1.602176634e-19_f64, // elementary charge in Coulombs
        uncertainty: 0.0_f64,
        unit: String::from("C"),
    };
    let cfg = config::legacy(); // fixed-int, little-endian
    let bytes =
        encode_to_vec_with_config(&measurement, cfg).expect("Failed to encode Measurement fixed");
    let (decoded, _): (Measurement, usize) =
        decode_from_slice_with_config(&bytes, cfg).expect("Failed to decode Measurement fixed");
    assert_eq!(measurement.value, decoded.value);
    assert_eq!(measurement.uncertainty, decoded.uncertainty);
    assert_eq!(measurement.unit, decoded.unit);
}
