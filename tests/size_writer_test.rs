//! Tests for SizeWriter and encoded_size API

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

#[derive(Debug, PartialEq, Encode, Decode)]
struct TestStruct {
    a: u32,
    b: String,
    c: bool,
}

#[test]
fn test_encoded_size_matches_u8() {
    let value: u8 = 42;
    let size = oxicode::encoded_size(&value).expect("encoded_size failed");
    let encoded = oxicode::encode_to_vec(&value).expect("encode failed");
    assert_eq!(size, encoded.len());
}

#[test]
fn test_encoded_size_matches_u64() {
    let value: u64 = 123456789;
    let size = oxicode::encoded_size(&value).expect("encoded_size failed");
    let encoded = oxicode::encode_to_vec(&value).expect("encode failed");
    assert_eq!(size, encoded.len());
}

#[test]
fn test_encoded_size_matches_string() {
    let value = String::from("hello world");
    let size = oxicode::encoded_size(&value).expect("encoded_size failed");
    let encoded = oxicode::encode_to_vec(&value).expect("encode failed");
    assert_eq!(size, encoded.len());
}

#[test]
fn test_encoded_size_matches_vec() {
    let value: Vec<i32> = vec![1, 2, 3, 4, 5];
    let size = oxicode::encoded_size(&value).expect("encoded_size failed");
    let encoded = oxicode::encode_to_vec(&value).expect("encode failed");
    assert_eq!(size, encoded.len());
}

#[test]
fn test_encoded_size_matches_struct() {
    let value = TestStruct {
        a: 42,
        b: String::from("test"),
        c: true,
    };
    let size = oxicode::encoded_size(&value).expect("encoded_size failed");
    let encoded = oxicode::encode_to_vec(&value).expect("encode failed");
    assert_eq!(size, encoded.len());
}

#[test]
fn test_encoded_size_matches_tuple() {
    let value = (1u32, 2u64, true, String::from("tuple"));
    let size = oxicode::encoded_size(&value).expect("encoded_size failed");
    let encoded = oxicode::encode_to_vec(&value).expect("encode failed");
    assert_eq!(size, encoded.len());
}

#[test]
fn test_encoded_size_matches_option_some() {
    let some_val: Option<u32> = Some(42);
    let size = oxicode::encoded_size(&some_val).expect("encoded_size failed");
    let encoded = oxicode::encode_to_vec(&some_val).expect("encode failed");
    assert_eq!(size, encoded.len());
}

#[test]
fn test_encoded_size_matches_option_none() {
    let none_val: Option<u32> = None;
    let size = oxicode::encoded_size(&none_val).expect("encoded_size failed");
    let encoded = oxicode::encode_to_vec(&none_val).expect("encode failed");
    assert_eq!(size, encoded.len());
}

#[test]
fn test_encoded_size_matches_empty_vec() {
    let value: Vec<u8> = vec![];
    let size = oxicode::encoded_size(&value).expect("encoded_size failed");
    let encoded = oxicode::encode_to_vec(&value).expect("encode failed");
    assert_eq!(size, encoded.len());
}

#[test]
fn test_encoded_size_with_config() {
    use oxicode::config;
    let value = 42u32;
    let size =
        oxicode::encoded_size_with_config(&value, config::legacy()).expect("encoded_size failed");
    let encoded =
        oxicode::encode_to_vec_with_config(&value, config::legacy()).expect("encode failed");
    assert_eq!(size, encoded.len());
}
