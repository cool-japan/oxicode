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
use oxicode::{config, decode_from_slice, encode_to_vec, encode_to_vec_with_config};

#[test]
fn test_f32_max_roundtrip() {
    let val = f32::MAX;
    let bytes = encode_to_vec(&val).expect("encode f32::MAX failed");
    let (decoded, _): (f32, _) = decode_from_slice(&bytes).expect("decode f32::MAX failed");
    assert_eq!(val, decoded);
}

#[test]
fn test_f32_min_roundtrip() {
    let val = f32::MIN;
    let bytes = encode_to_vec(&val).expect("encode f32::MIN failed");
    let (decoded, _): (f32, _) = decode_from_slice(&bytes).expect("decode f32::MIN failed");
    assert_eq!(val, decoded);
}

#[test]
fn test_f32_min_positive_roundtrip() {
    let val = f32::MIN_POSITIVE;
    let bytes = encode_to_vec(&val).expect("encode f32::MIN_POSITIVE failed");
    let (decoded, _): (f32, _) =
        decode_from_slice(&bytes).expect("decode f32::MIN_POSITIVE failed");
    assert_eq!(val, decoded);
}

#[test]
fn test_f64_max_roundtrip() {
    let val = f64::MAX;
    let bytes = encode_to_vec(&val).expect("encode f64::MAX failed");
    let (decoded, _): (f64, _) = decode_from_slice(&bytes).expect("decode f64::MAX failed");
    assert_eq!(val, decoded);
}

#[test]
fn test_f64_min_roundtrip() {
    let val = f64::MIN;
    let bytes = encode_to_vec(&val).expect("encode f64::MIN failed");
    let (decoded, _): (f64, _) = decode_from_slice(&bytes).expect("decode f64::MIN failed");
    assert_eq!(val, decoded);
}

#[test]
fn test_f64_infinity_roundtrip() {
    let val = f64::INFINITY;
    let bytes = encode_to_vec(&val).expect("encode f64::INFINITY failed");
    let (decoded, _): (f64, _) = decode_from_slice(&bytes).expect("decode f64::INFINITY failed");
    assert_eq!(val, decoded);
}

#[test]
fn test_f64_neg_infinity_roundtrip() {
    let val = f64::NEG_INFINITY;
    let bytes = encode_to_vec(&val).expect("encode f64::NEG_INFINITY failed");
    let (decoded, _): (f64, _) =
        decode_from_slice(&bytes).expect("decode f64::NEG_INFINITY failed");
    assert_eq!(val, decoded);
}

#[test]
fn test_f64_nan_roundtrip() {
    let val = f64::NAN;
    let bytes = encode_to_vec(&val).expect("encode f64::NAN failed");
    let (decoded, _): (f64, _) = decode_from_slice(&bytes).expect("decode f64::NAN failed");
    assert_eq!(val.to_bits(), decoded.to_bits());
}

#[test]
fn test_f32_nan_roundtrip() {
    let val = f32::NAN;
    let bytes = encode_to_vec(&val).expect("encode f32::NAN failed");
    let (decoded, _): (f32, _) = decode_from_slice(&bytes).expect("decode f32::NAN failed");
    assert_eq!(val.to_bits(), decoded.to_bits());
}

#[test]
fn test_f32_zero_roundtrip() {
    let val: f32 = 0.0;
    let bytes = encode_to_vec(&val).expect("encode f32 0.0 failed");
    let (decoded, _): (f32, _) = decode_from_slice(&bytes).expect("decode f32 0.0 failed");
    assert_eq!(val, decoded);
}

#[test]
fn test_f64_zero_roundtrip() {
    let val: f64 = 0.0;
    let bytes = encode_to_vec(&val).expect("encode f64 0.0 failed");
    let (decoded, _): (f64, _) = decode_from_slice(&bytes).expect("decode f64 0.0 failed");
    assert_eq!(val, decoded);
}

#[test]
fn test_f32_negative_zero_roundtrip() {
    let val: f32 = -0.0_f32;
    let bytes = encode_to_vec(&val).expect("encode f32 -0.0 failed");
    let (decoded, _): (f32, _) = decode_from_slice(&bytes).expect("decode f32 -0.0 failed");
    assert_eq!(val.to_bits(), decoded.to_bits());
}

#[test]
fn test_f64_negative_zero_roundtrip() {
    let val: f64 = -0.0_f64;
    let bytes = encode_to_vec(&val).expect("encode f64 -0.0 failed");
    let (decoded, _): (f64, _) = decode_from_slice(&bytes).expect("decode f64 -0.0 failed");
    assert_eq!(val.to_bits(), decoded.to_bits());
}

#[test]
fn test_vec_f32_roundtrip() {
    let val: Vec<f32> = vec![1.0_f32, -1.0, 0.5, f32::MAX, f32::MIN_POSITIVE];
    let bytes = encode_to_vec(&val).expect("encode Vec<f32> failed");
    let (decoded, _): (Vec<f32>, _) = decode_from_slice(&bytes).expect("decode Vec<f32> failed");
    assert_eq!(val, decoded);
}

#[test]
fn test_vec_f64_roundtrip() {
    let val: Vec<f64> = vec![1.0_f64, -1.0, 0.5, f64::MAX, f64::MIN_POSITIVE];
    let bytes = encode_to_vec(&val).expect("encode Vec<f64> failed");
    let (decoded, _): (Vec<f64>, _) = decode_from_slice(&bytes).expect("decode Vec<f64> failed");
    assert_eq!(val, decoded);
}

#[test]
fn test_f32_encoding_is_4_bytes_fixed_int() {
    let val: f32 = 1.0_f32;
    let cfg = config::standard().with_fixed_int_encoding();
    let bytes =
        encode_to_vec_with_config(&val, cfg).expect("encode f32 with fixed-int config failed");
    assert_eq!(
        bytes.len(),
        4,
        "f32 with fixed-int config must be exactly 4 bytes"
    );
}

#[test]
fn test_f64_encoding_is_8_bytes_fixed_int() {
    let val: f64 = 1.0_f64;
    let cfg = config::standard().with_fixed_int_encoding();
    let bytes =
        encode_to_vec_with_config(&val, cfg).expect("encode f64 with fixed-int config failed");
    assert_eq!(
        bytes.len(),
        8,
        "f64 with fixed-int config must be exactly 8 bytes"
    );
}

#[test]
fn test_f32_pi_roundtrip_bit_exact() {
    let val = std::f32::consts::PI;
    let bytes = encode_to_vec(&val).expect("encode f32 PI failed");
    let (decoded, _): (f32, _) = decode_from_slice(&bytes).expect("decode f32 PI failed");
    assert_eq!(
        val.to_bits(),
        decoded.to_bits(),
        "f32 PI roundtrip must be bit-exact"
    );
}

#[test]
fn test_f64_e_roundtrip_bit_exact() {
    let val = std::f64::consts::E;
    let bytes = encode_to_vec(&val).expect("encode f64 E failed");
    let (decoded, _): (f64, _) = decode_from_slice(&bytes).expect("decode f64 E failed");
    assert_eq!(
        val.to_bits(),
        decoded.to_bits(),
        "f64 E roundtrip must be bit-exact"
    );
}

#[test]
fn test_option_f64_some_roundtrip() {
    let val: Option<f64> = Some(std::f64::consts::PI);
    let bytes = encode_to_vec(&val).expect("encode Option<f64> Some failed");
    let (decoded, _): (Option<f64>, _) =
        decode_from_slice(&bytes).expect("decode Option<f64> Some failed");
    assert_eq!(val, decoded);
}

#[test]
fn test_option_f32_none_roundtrip() {
    let val: Option<f32> = None;
    let bytes = encode_to_vec(&val).expect("encode Option<f32> None failed");
    let (decoded, _): (Option<f32>, _) =
        decode_from_slice(&bytes).expect("decode Option<f32> None failed");
    assert_eq!(val, decoded);
}

#[test]
fn test_vec_f64_special_values_roundtrip() {
    let val: Vec<f64> = vec![f64::INFINITY, f64::NEG_INFINITY, f64::NAN, 0.0, -0.0];
    let bytes = encode_to_vec(&val).expect("encode Vec<f64> special values failed");
    let (decoded, _): (Vec<f64>, _) =
        decode_from_slice(&bytes).expect("decode Vec<f64> special values failed");
    assert_eq!(
        val.len(),
        decoded.len(),
        "decoded Vec<f64> must have same length"
    );
    for (i, (orig, dec)) in val.iter().zip(decoded.iter()).enumerate() {
        assert_eq!(
            orig.to_bits(),
            dec.to_bits(),
            "element {} of Vec<f64> special values roundtrip must be bit-exact",
            i
        );
    }
}
