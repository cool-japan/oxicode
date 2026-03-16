//! Tests for Option and Result encoding/decoding.

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
fn test_option_none_roundtrip() {
    let v: Option<u32> = None;
    let enc = encode_to_vec(&v).expect("encode");
    let (dec, _): (Option<u32>, _) = decode_from_slice(&enc).expect("decode");
    assert_eq!(v, dec);
}

#[test]
fn test_option_some_roundtrip() {
    let v: Option<u32> = Some(42);
    let enc = encode_to_vec(&v).expect("encode");
    let (dec, _): (Option<u32>, _) = decode_from_slice(&enc).expect("decode");
    assert_eq!(v, dec);
}

#[test]
fn test_option_some_string_roundtrip() {
    let v: Option<String> = Some("hello oxicode".to_string());
    let enc = encode_to_vec(&v).expect("encode");
    let (dec, _): (Option<String>, _) = decode_from_slice(&enc).expect("decode");
    assert_eq!(v, dec);
}

#[test]
fn test_option_vec_roundtrip() {
    let v: Option<Vec<u8>> = Some(vec![1, 2, 3, 4, 5]);
    let enc = encode_to_vec(&v).expect("encode");
    let (dec, _): (Option<Vec<u8>>, _) = decode_from_slice(&enc).expect("decode");
    assert_eq!(v, dec);
}

#[test]
fn test_nested_option_roundtrip() {
    for v in [None, Some(None), Some(Some(42u32))] {
        let enc = encode_to_vec(&v).expect("encode");
        let (dec, _): (Option<Option<u32>>, _) = decode_from_slice(&enc).expect("decode");
        assert_eq!(v, dec);
    }
}

#[test]
fn test_option_none_size() {
    let enc = encode_to_vec(&Option::<Vec<String>>::None).expect("encode");
    assert_eq!(enc.len(), 1, "None should encode to 1 byte");
}

#[test]
fn test_vec_of_options() {
    let v = vec![Some(1u32), None, Some(3), None, Some(5)];
    let enc = encode_to_vec(&v).expect("encode");
    let (dec, _): (Vec<Option<u32>>, _) = decode_from_slice(&enc).expect("decode");
    assert_eq!(v, dec);
}

#[test]
fn test_result_ok_roundtrip() {
    let v: Result<u32, String> = Ok(100);
    let enc = encode_to_vec(&v).expect("encode");
    let (dec, _): (Result<u32, String>, _) = decode_from_slice(&enc).expect("decode");
    assert_eq!(v, dec);
}

#[test]
fn test_result_err_roundtrip() {
    let v: Result<u32, String> = Err("something went wrong".to_string());
    let enc = encode_to_vec(&v).expect("encode");
    let (dec, _): (Result<u32, String>, _) = decode_from_slice(&enc).expect("decode");
    assert_eq!(v, dec);
}

#[test]
fn test_result_ok_unit_err() {
    let v: Result<u64, ()> = Ok(9999);
    let enc = encode_to_vec(&v).expect("encode");
    let (dec, _): (Result<u64, ()>, _) = decode_from_slice(&enc).expect("decode");
    assert_eq!(v, dec);
}

#[test]
fn test_result_err_unit_ok() {
    let v: Result<(), String> = Err("unit ok".to_string());
    let enc = encode_to_vec(&v).expect("encode");
    let (dec, _): (Result<(), String>, _) = decode_from_slice(&enc).expect("decode");
    assert_eq!(v, dec);
}

#[test]
fn test_option_in_struct() {
    use oxicode_derive::{Decode, Encode};
    #[derive(Encode, Decode, Debug, PartialEq)]
    struct Wrapper {
        value: Option<i64>,
        label: Option<String>,
    }
    let w = Wrapper {
        value: Some(-42),
        label: None,
    };
    let enc = encode_to_vec(&w).expect("encode");
    let (dec, _): (Wrapper, _) = decode_from_slice(&enc).expect("decode");
    assert_eq!(w, dec);
}

#[test]
fn test_result_nested_option() {
    let v: Result<Option<u32>, String> = Ok(Some(7));
    let enc = encode_to_vec(&v).expect("encode");
    let (dec, _): (Result<Option<u32>, String>, _) = decode_from_slice(&enc).expect("decode");
    assert_eq!(v, dec);

    let v2: Result<Option<u32>, String> = Ok(None);
    let enc2 = encode_to_vec(&v2).expect("encode");
    let (dec2, _): (Result<Option<u32>, String>, _) = decode_from_slice(&enc2).expect("decode");
    assert_eq!(v2, dec2);
}
