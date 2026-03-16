//! Advanced tests for std::convert::Infallible and std::ops::ControlFlow encode/decode.

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
use std::ops::ControlFlow;

// ===== Infallible via Result<T, Infallible> =====

#[test]
fn test_result_ok_infallible_u32_roundtrip() {
    let val: Result<u32, std::convert::Infallible> = Ok(42);
    let enc = encode_to_vec(&val).expect("encode Result<u32, Infallible>");
    let (dec, _): (Result<u32, std::convert::Infallible>, _) =
        decode_from_slice(&enc).expect("decode Result<u32, Infallible>");
    assert_eq!(val, dec);
}

#[test]
fn test_result_ok_infallible_zero_roundtrip() {
    let val: Result<u32, std::convert::Infallible> = Ok(0);
    let enc = encode_to_vec(&val).expect("encode Result<u32, Infallible> zero");
    let (dec, _): (Result<u32, std::convert::Infallible>, _) =
        decode_from_slice(&enc).expect("decode Result<u32, Infallible> zero");
    assert_eq!(val, dec);
}

#[test]
fn test_result_ok_infallible_max_roundtrip() {
    let val: Result<u64, std::convert::Infallible> = Ok(u64::MAX);
    let enc = encode_to_vec(&val).expect("encode Result<u64, Infallible> MAX");
    let (dec, _): (Result<u64, std::convert::Infallible>, _) =
        decode_from_slice(&enc).expect("decode Result<u64, Infallible> MAX");
    assert_eq!(val, dec);
}

// ===== ControlFlow tests =====

#[test]
fn test_control_flow_continue_u32_roundtrip() {
    let cf: ControlFlow<u32, u32> = ControlFlow::Continue(42);
    let enc = encode_to_vec(&cf).expect("encode ControlFlow::Continue(42)");
    let (dec, _): (ControlFlow<u32, u32>, _) =
        decode_from_slice(&enc).expect("decode ControlFlow::Continue(42)");
    assert_eq!(cf, dec);
}

#[test]
fn test_control_flow_break_u32_roundtrip() {
    let cf: ControlFlow<u32, u32> = ControlFlow::Break(99);
    let enc = encode_to_vec(&cf).expect("encode ControlFlow::Break(99)");
    let (dec, _): (ControlFlow<u32, u32>, _) =
        decode_from_slice(&enc).expect("decode ControlFlow::Break(99)");
    assert_eq!(cf, dec);
}

#[test]
fn test_control_flow_continue_string_break_u32_roundtrip() {
    let cf: ControlFlow<u32, String> = ControlFlow::Continue("hello".to_string());
    let enc = encode_to_vec(&cf).expect("encode ControlFlow::Continue(String)");
    let (dec, _): (ControlFlow<u32, String>, _) =
        decode_from_slice(&enc).expect("decode ControlFlow::Continue(String)");
    assert_eq!(cf, dec);
}

#[test]
fn test_control_flow_break_string_continue_u32_roundtrip() {
    let cf: ControlFlow<String, u32> = ControlFlow::Break("error".to_string());
    let enc = encode_to_vec(&cf).expect("encode ControlFlow::Break(String)");
    let (dec, _): (ControlFlow<String, u32>, _) =
        decode_from_slice(&enc).expect("decode ControlFlow::Break(String)");
    assert_eq!(cf, dec);
}

#[test]
fn test_control_flow_continue_and_break_produce_different_bytes() {
    let cont: ControlFlow<u32, u32> = ControlFlow::Continue(42);
    let brk: ControlFlow<u32, u32> = ControlFlow::Break(42);
    let enc_cont = encode_to_vec(&cont).expect("encode Continue");
    let enc_brk = encode_to_vec(&brk).expect("encode Break");
    assert_ne!(
        enc_cont, enc_brk,
        "Continue and Break must produce different encodings"
    );
}

#[test]
fn test_vec_of_control_flow_roundtrip() {
    let vec: Vec<ControlFlow<u32, u32>> = vec![
        ControlFlow::Continue(1),
        ControlFlow::Break(2),
        ControlFlow::Continue(3),
        ControlFlow::Break(4),
    ];
    let enc = encode_to_vec(&vec).expect("encode Vec<ControlFlow>");
    let (dec, _): (Vec<ControlFlow<u32, u32>>, _) =
        decode_from_slice(&enc).expect("decode Vec<ControlFlow>");
    assert_eq!(vec, dec);
}

#[test]
fn test_option_some_control_flow_roundtrip() {
    let opt: Option<ControlFlow<u32, u32>> = Some(ControlFlow::Continue(77));
    let enc = encode_to_vec(&opt).expect("encode Option<ControlFlow>");
    let (dec, _): (Option<ControlFlow<u32, u32>>, _) =
        decode_from_slice(&enc).expect("decode Option<ControlFlow>");
    assert_eq!(opt, dec);
}

#[test]
fn test_control_flow_continue_zero_roundtrip() {
    let cf: ControlFlow<u32, u32> = ControlFlow::Continue(0u32);
    let enc = encode_to_vec(&cf).expect("encode ControlFlow::Continue(0)");
    let (dec, _): (ControlFlow<u32, u32>, _) =
        decode_from_slice(&enc).expect("decode ControlFlow::Continue(0)");
    assert_eq!(cf, dec);
}

#[test]
fn test_control_flow_continue_u32_max_roundtrip() {
    let cf: ControlFlow<u32, u32> = ControlFlow::Continue(u32::MAX);
    let enc = encode_to_vec(&cf).expect("encode ControlFlow::Continue(u32::MAX)");
    let (dec, _): (ControlFlow<u32, u32>, _) =
        decode_from_slice(&enc).expect("decode ControlFlow::Continue(u32::MAX)");
    assert_eq!(cf, dec);
}

#[test]
fn test_control_flow_break_u32_max_roundtrip() {
    let cf: ControlFlow<u32, u32> = ControlFlow::Break(u32::MAX);
    let enc = encode_to_vec(&cf).expect("encode ControlFlow::Break(u32::MAX)");
    let (dec, _): (ControlFlow<u32, u32>, _) =
        decode_from_slice(&enc).expect("decode ControlFlow::Break(u32::MAX)");
    assert_eq!(cf, dec);
}

#[test]
fn test_control_flow_consumed_bytes_equals_encoded_length() {
    let cf: ControlFlow<u32, u32> = ControlFlow::Continue(123);
    let enc = encode_to_vec(&cf).expect("encode");
    let (_, consumed): (ControlFlow<u32, u32>, usize) = decode_from_slice(&enc).expect("decode");
    assert_eq!(
        consumed,
        enc.len(),
        "consumed bytes must equal encoded length"
    );
}

#[test]
fn test_control_flow_string_vec_u8_continue_roundtrip() {
    let cf: ControlFlow<String, Vec<u8>> = ControlFlow::Continue(vec![1u8, 2, 3, 255]);
    let enc = encode_to_vec(&cf).expect("encode ControlFlow<String, Vec<u8>>::Continue");
    let (dec, _): (ControlFlow<String, Vec<u8>>, _) =
        decode_from_slice(&enc).expect("decode ControlFlow<String, Vec<u8>>::Continue");
    assert_eq!(cf, dec);
}

#[test]
fn test_control_flow_string_vec_u8_break_roundtrip() {
    let cf: ControlFlow<String, Vec<u8>> = ControlFlow::Break("fatal error".to_string());
    let enc = encode_to_vec(&cf).expect("encode ControlFlow<String, Vec<u8>>::Break");
    let (dec, _): (ControlFlow<String, Vec<u8>>, _) =
        decode_from_slice(&enc).expect("decode ControlFlow<String, Vec<u8>>::Break");
    assert_eq!(cf, dec);
}

#[test]
fn test_nested_control_flow_continue_continue_roundtrip() {
    let inner: ControlFlow<u32, u32> = ControlFlow::Continue(10);
    let outer: ControlFlow<u32, ControlFlow<u32, u32>> = ControlFlow::Continue(inner);
    let enc = encode_to_vec(&outer).expect("encode nested ControlFlow Continue/Continue");
    let (dec, _): (ControlFlow<u32, ControlFlow<u32, u32>>, _) =
        decode_from_slice(&enc).expect("decode nested ControlFlow Continue/Continue");
    assert_eq!(outer, dec);
}

#[test]
fn test_nested_control_flow_continue_break_roundtrip() {
    let inner: ControlFlow<u32, u32> = ControlFlow::Break(99);
    let outer: ControlFlow<u32, ControlFlow<u32, u32>> = ControlFlow::Continue(inner);
    let enc = encode_to_vec(&outer).expect("encode nested ControlFlow Continue/Break");
    let (dec, _): (ControlFlow<u32, ControlFlow<u32, u32>>, _) =
        decode_from_slice(&enc).expect("decode nested ControlFlow Continue/Break");
    assert_eq!(outer, dec);
}

#[test]
fn test_control_flow_option_none_continue_roundtrip() {
    let cf: ControlFlow<u32, Option<u32>> = ControlFlow::Continue(None);
    let enc = encode_to_vec(&cf).expect("encode ControlFlow::Continue(None)");
    let (dec, _): (ControlFlow<u32, Option<u32>>, _) =
        decode_from_slice(&enc).expect("decode ControlFlow::Continue(None)");
    assert_eq!(cf, dec);
}

#[test]
fn test_control_flow_option_some_continue_roundtrip() {
    let cf: ControlFlow<u32, Option<u32>> = ControlFlow::Continue(Some(42));
    let enc = encode_to_vec(&cf).expect("encode ControlFlow::Continue(Some(42))");
    let (dec, _): (ControlFlow<u32, Option<u32>>, _) =
        decode_from_slice(&enc).expect("decode ControlFlow::Continue(Some(42))");
    assert_eq!(cf, dec);
}

#[test]
fn test_control_flow_tuple_continue_roundtrip() {
    let cf: ControlFlow<u32, (u32, String)> = ControlFlow::Continue((7u32, "seven".to_string()));
    let enc = encode_to_vec(&cf).expect("encode ControlFlow::Continue((u32, String))");
    let (dec, _): (ControlFlow<u32, (u32, String)>, _) =
        decode_from_slice(&enc).expect("decode ControlFlow::Continue((u32, String))");
    assert_eq!(cf, dec);
}

#[test]
fn test_control_flow_empty_string_break_roundtrip() {
    let cf: ControlFlow<String, u32> = ControlFlow::Break(String::new());
    let enc = encode_to_vec(&cf).expect("encode ControlFlow::Break(empty String)");
    let (dec, _): (ControlFlow<String, u32>, _) =
        decode_from_slice(&enc).expect("decode ControlFlow::Break(empty String)");
    assert_eq!(cf, dec);
}
