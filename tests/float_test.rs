//! Tests for float encode/decode including special values.

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
use oxicode::{decode_from_slice, encode_to_vec};

#[test]
fn test_f32_zero_roundtrip() {
    for v in [0.0f32, -0.0f32] {
        let enc = encode_to_vec(&v).expect("encode");
        let (dec, _): (f32, _) = decode_from_slice(&enc).expect("decode");
        // For -0.0 and 0.0, bits should be preserved
        assert_eq!(v.to_bits(), dec.to_bits());
    }
}

#[test]
fn test_f32_infinity() {
    for v in [f32::INFINITY, f32::NEG_INFINITY] {
        let enc = encode_to_vec(&v).expect("encode");
        let (dec, _): (f32, _) = decode_from_slice(&enc).expect("decode");
        assert_eq!(v, dec);
    }
}

#[test]
fn test_f32_nan() {
    let v = f32::NAN;
    let enc = encode_to_vec(&v).expect("encode");
    let (dec, _): (f32, _) = decode_from_slice(&enc).expect("decode");
    assert!(dec.is_nan());
}

#[test]
fn test_f64_special_values() {
    let values = [
        f64::INFINITY,
        f64::NEG_INFINITY,
        f64::MAX,
        f64::MIN,
        f64::MIN_POSITIVE,
        0.0f64,
        -0.0f64,
    ];
    for v in values {
        let enc = encode_to_vec(&v).expect("encode");
        let (dec, _): (f64, _) = decode_from_slice(&enc).expect("decode");
        assert_eq!(v.to_bits(), dec.to_bits(), "bits mismatch for {}", v);
    }
}

#[test]
fn test_f64_nan_bits_preserved() {
    // NaN with specific payload bits should be preserved
    let v = f64::NAN;
    let enc = encode_to_vec(&v).expect("encode");
    let (dec, _): (f64, _) = decode_from_slice(&enc).expect("decode");
    assert_eq!(v.to_bits(), dec.to_bits());
}

#[test]
fn test_f32_f64_size() {
    // f32 should always encode to 4 bytes, f64 to 8 bytes
    let f32_enc = encode_to_vec(&1.0f32).expect("encode");
    let f64_enc = encode_to_vec(&1.0f64).expect("encode");
    assert_eq!(f32_enc.len(), 4, "f32 should be 4 bytes");
    assert_eq!(f64_enc.len(), 8, "f64 should be 8 bytes");
}
