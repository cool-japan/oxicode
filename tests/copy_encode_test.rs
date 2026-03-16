//! Tests for encode_copy convenience function.

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
#[test]
fn test_encode_copy_u8() {
    let bytes = oxicode::encode_copy(42u8).expect("encode");
    let (dec, _): (u8, _) = oxicode::decode_from_slice(&bytes).expect("decode");
    assert_eq!(dec, 42u8);
}

#[test]
fn test_encode_copy_bool() {
    for &b in &[true, false] {
        let bytes = oxicode::encode_copy(b).expect("encode");
        let (dec, _): (bool, _) = oxicode::decode_from_slice(&bytes).expect("decode");
        assert_eq!(dec, b);
    }
}

#[test]
fn test_encode_copy_matches_encode_to_vec() {
    let val = 12345678u64;
    let enc_copy = oxicode::encode_copy(val).expect("encode_copy");
    let enc_ref = oxicode::encode_to_vec(&val).expect("encode_to_vec");
    assert_eq!(enc_copy, enc_ref);
}

#[test]
fn test_encode_copy_f64() {
    let val = std::f64::consts::PI;
    let bytes = oxicode::encode_copy(val).expect("encode");
    let (dec, _): (f64, _) = oxicode::decode_from_slice(&bytes).expect("decode");
    assert_eq!(dec, val);
}
