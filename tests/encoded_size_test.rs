//! Tests that encoded_size matches actual encode_to_vec length.

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
use oxicode::{Decode, Encode};

#[derive(Encode, Decode)]
struct Point {
    x: f32,
    y: f32,
}

#[test]
fn test_encoded_size_u8() {
    let n = oxicode::encoded_size(&42u8).expect("encoded_size");
    let actual = oxicode::encode_to_vec(&42u8).expect("encode").len();
    assert_eq!(n, actual);
}

#[test]
fn test_encoded_size_u64() {
    let n = oxicode::encoded_size(&u64::MAX).expect("encoded_size");
    let actual = oxicode::encode_to_vec(&u64::MAX).expect("encode").len();
    assert_eq!(n, actual);
}

#[test]
fn test_encoded_size_string() {
    let s = "hello, oxicode!".to_string();
    let n = oxicode::encoded_size(&s).expect("encoded_size");
    let actual = oxicode::encode_to_vec(&s).expect("encode").len();
    assert_eq!(n, actual);
}

#[test]
fn test_encoded_size_vec() {
    let v: Vec<u32> = (0..100).collect();
    let n = oxicode::encoded_size(&v).expect("encoded_size");
    let actual = oxicode::encode_to_vec(&v).expect("encode").len();
    assert_eq!(n, actual);
}

#[test]
fn test_encoded_size_struct() {
    let p = Point { x: 1.0, y: 2.0 };
    let n = oxicode::encoded_size(&p).expect("encoded_size");
    let actual = oxicode::encode_to_vec(&p).expect("encode").len();
    assert_eq!(n, actual);
}

#[test]
fn test_encoded_size_nested_vec_string() {
    let v: Vec<String> = vec!["one".to_string(), "two".to_string(), "three".to_string()];
    let n = oxicode::encoded_size(&v).expect("encoded_size");
    let actual = oxicode::encode_to_vec(&v).expect("encode").len();
    assert_eq!(n, actual);
}

#[test]
fn test_encoded_size_matches_size_hint() {
    let v: Vec<u8> = (0..=255).collect();
    let n = oxicode::encoded_size(&v).expect("encoded_size");
    let buf = oxicode::encode_to_vec_with_size_hint(&v, n).expect("encode_hint");
    assert_eq!(n, buf.len());
}

#[test]
fn test_encoded_size_zero_value() {
    let n = oxicode::encoded_size(&0u64).expect("encoded_size");
    let actual = oxicode::encode_to_vec(&0u64).expect("encode").len();
    assert_eq!(n, actual);
}

#[test]
fn test_encoded_size_bool_true() {
    let n = oxicode::encoded_size(&true).expect("encoded_size");
    let actual = oxicode::encode_to_vec(&true).expect("encode").len();
    assert_eq!(n, actual);
    assert_eq!(n, 1);
}

#[test]
fn test_encoded_size_bool_false() {
    let n = oxicode::encoded_size(&false).expect("encoded_size");
    let actual = oxicode::encode_to_vec(&false).expect("encode").len();
    assert_eq!(n, actual);
    assert_eq!(n, 1);
}

#[test]
fn test_encoded_size_option_some() {
    let v: Option<u32> = Some(999);
    let n = oxicode::encoded_size(&v).expect("encoded_size");
    let actual = oxicode::encode_to_vec(&v).expect("encode").len();
    assert_eq!(n, actual);
}

#[test]
fn test_encoded_size_option_none() {
    let v: Option<u32> = None;
    let n = oxicode::encoded_size(&v).expect("encoded_size");
    let actual = oxicode::encode_to_vec(&v).expect("encode").len();
    assert_eq!(n, actual);
}

#[test]
fn test_encoded_size_empty_string() {
    let s = String::new();
    let n = oxicode::encoded_size(&s).expect("encoded_size");
    let actual = oxicode::encode_to_vec(&s).expect("encode").len();
    assert_eq!(n, actual);
}

#[test]
fn test_encoded_size_with_config_fixed() {
    let v = 42u32;
    let config = oxicode::config::standard().with_fixed_int_encoding();
    let n = oxicode::encoded_size_with_config(&v, config).expect("encoded_size");
    let actual = oxicode::encode_to_vec_with_config(&v, config)
        .expect("encode")
        .len();
    assert_eq!(n, actual);
    assert_eq!(n, 4); // fixed u32 is always 4 bytes
}
