//! Advanced tests for Option<T> encoding in OxiCode (set 4)

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
struct Config {
    key: String,
    value: Option<String>,
    priority: Option<u32>,
}

#[test]
fn test_option_u32_none_roundtrip() {
    let val: Option<u32> = None;
    let bytes = encode_to_vec(&val).expect("encode Option<u32> None");
    let (decoded, _): (Option<u32>, usize) =
        decode_from_slice(&bytes).expect("decode Option<u32> None");
    assert_eq!(val, decoded);
}

#[test]
fn test_option_u32_some_zero_roundtrip() {
    let val: Option<u32> = Some(0);
    let bytes = encode_to_vec(&val).expect("encode Option<u32> Some(0)");
    let (decoded, _): (Option<u32>, usize) =
        decode_from_slice(&bytes).expect("decode Option<u32> Some(0)");
    assert_eq!(val, decoded);
}

#[test]
fn test_option_u32_some_42_roundtrip() {
    let val: Option<u32> = Some(42);
    let bytes = encode_to_vec(&val).expect("encode Option<u32> Some(42)");
    let (decoded, _): (Option<u32>, usize) =
        decode_from_slice(&bytes).expect("decode Option<u32> Some(42)");
    assert_eq!(val, decoded);
}

#[test]
fn test_option_u32_some_max_roundtrip() {
    let val: Option<u32> = Some(u32::MAX);
    let bytes = encode_to_vec(&val).expect("encode Option<u32> Some(u32::MAX)");
    let (decoded, _): (Option<u32>, usize) =
        decode_from_slice(&bytes).expect("decode Option<u32> Some(u32::MAX)");
    assert_eq!(val, decoded);
}

#[test]
fn test_option_string_none_roundtrip() {
    let val: Option<String> = None;
    let bytes = encode_to_vec(&val).expect("encode Option<String> None");
    let (decoded, _): (Option<String>, usize) =
        decode_from_slice(&bytes).expect("decode Option<String> None");
    assert_eq!(val, decoded);
}

#[test]
fn test_option_string_some_hello_roundtrip() {
    let val: Option<String> = Some("hello".to_string());
    let bytes = encode_to_vec(&val).expect("encode Option<String> Some(hello)");
    let (decoded, _): (Option<String>, usize) =
        decode_from_slice(&bytes).expect("decode Option<String> Some(hello)");
    assert_eq!(val, decoded);
}

#[test]
fn test_option_string_some_empty_roundtrip() {
    let val: Option<String> = Some(String::new());
    let bytes = encode_to_vec(&val).expect("encode Option<String> Some(\"\")");
    let (decoded, _): (Option<String>, usize) =
        decode_from_slice(&bytes).expect("decode Option<String> Some(\"\")");
    assert_eq!(val, decoded);
}

#[test]
fn test_option_vec_u8_none_roundtrip() {
    let val: Option<Vec<u8>> = None;
    let bytes = encode_to_vec(&val).expect("encode Option<Vec<u8>> None");
    let (decoded, _): (Option<Vec<u8>>, usize) =
        decode_from_slice(&bytes).expect("decode Option<Vec<u8>> None");
    assert_eq!(val, decoded);
}

#[test]
fn test_option_vec_u8_some_empty_roundtrip() {
    let val: Option<Vec<u8>> = Some(vec![]);
    let bytes = encode_to_vec(&val).expect("encode Option<Vec<u8>> Some(vec![])");
    let (decoded, _): (Option<Vec<u8>>, usize) =
        decode_from_slice(&bytes).expect("decode Option<Vec<u8>> Some(vec![])");
    assert_eq!(val, decoded);
}

#[test]
fn test_option_vec_u8_some_data_roundtrip() {
    let val: Option<Vec<u8>> = Some(vec![1, 2, 3]);
    let bytes = encode_to_vec(&val).expect("encode Option<Vec<u8>> Some(vec![1,2,3])");
    let (decoded, _): (Option<Vec<u8>>, usize) =
        decode_from_slice(&bytes).expect("decode Option<Vec<u8>> Some(vec![1,2,3])");
    assert_eq!(val, decoded);
}

#[test]
fn test_option_config_none_roundtrip() {
    let val: Option<Config> = None;
    let bytes = encode_to_vec(&val).expect("encode Option<Config> None");
    let (decoded, _): (Option<Config>, usize) =
        decode_from_slice(&bytes).expect("decode Option<Config> None");
    assert_eq!(val, decoded);
}

#[test]
fn test_option_config_some_roundtrip() {
    let val: Option<Config> = Some(Config {
        key: "timeout".to_string(),
        value: Some("30s".to_string()),
        priority: Some(5),
    });
    let bytes = encode_to_vec(&val).expect("encode Option<Config> Some");
    let (decoded, _): (Option<Config>, usize) =
        decode_from_slice(&bytes).expect("decode Option<Config> Some");
    assert_eq!(val, decoded);
}

#[test]
fn test_config_all_none_fields_roundtrip() {
    let val = Config {
        key: "empty".to_string(),
        value: None,
        priority: None,
    };
    let bytes = encode_to_vec(&val).expect("encode Config all None fields");
    let (decoded, _): (Config, usize) =
        decode_from_slice(&bytes).expect("decode Config all None fields");
    assert_eq!(val, decoded);
}

#[test]
fn test_config_all_some_fields_roundtrip() {
    let val = Config {
        key: "retries".to_string(),
        value: Some("3".to_string()),
        priority: Some(10),
    };
    let bytes = encode_to_vec(&val).expect("encode Config all Some fields");
    let (decoded, _): (Config, usize) =
        decode_from_slice(&bytes).expect("decode Config all Some fields");
    assert_eq!(val, decoded);
}

#[test]
fn test_none_encodes_as_single_byte_zero() {
    let val: Option<u32> = None;
    let bytes = encode_to_vec(&val).expect("encode Option<u32> None for byte check");
    assert_eq!(bytes[0], 0u8, "None must encode with a leading zero byte");
}

#[test]
fn test_some_zero_u32_encodes_with_byte_one_prefix() {
    let val: Option<u32> = Some(0u32);
    let bytes = encode_to_vec(&val).expect("encode Some(0u32) for byte check");
    assert_eq!(
        bytes[0], 1u8,
        "Some(...) must encode with a leading byte of 1"
    );
}

#[test]
fn test_vec_option_u32_roundtrip() {
    let val: Vec<Option<u32>> = vec![Some(10), None, Some(20), None, Some(30)];
    let bytes = encode_to_vec(&val).expect("encode Vec<Option<u32>>");
    let (decoded, _): (Vec<Option<u32>>, usize) =
        decode_from_slice(&bytes).expect("decode Vec<Option<u32>>");
    assert_eq!(val, decoded);
}

#[test]
fn test_option_option_u32_some_some_roundtrip() {
    let val: Option<Option<u32>> = Some(Some(42));
    let bytes = encode_to_vec(&val).expect("encode Option<Option<u32>> Some(Some(42))");
    let (decoded, _): (Option<Option<u32>>, usize) =
        decode_from_slice(&bytes).expect("decode Option<Option<u32>> Some(Some(42))");
    assert_eq!(val, decoded);
}

#[test]
fn test_option_option_u32_some_none_roundtrip() {
    let val: Option<Option<u32>> = Some(None);
    let bytes = encode_to_vec(&val).expect("encode Option<Option<u32>> Some(None)");
    let (decoded, _): (Option<Option<u32>>, usize) =
        decode_from_slice(&bytes).expect("decode Option<Option<u32>> Some(None)");
    assert_eq!(val, decoded);
}

#[test]
fn test_option_option_u32_none_roundtrip() {
    let val: Option<Option<u32>> = None;
    let bytes = encode_to_vec(&val).expect("encode Option<Option<u32>> None");
    let (decoded, _): (Option<Option<u32>>, usize) =
        decode_from_slice(&bytes).expect("decode Option<Option<u32>> None");
    assert_eq!(val, decoded);
}

#[test]
fn test_option_u32_with_fixed_int_config() {
    let cfg = config::standard().with_fixed_int_encoding();
    let val: Option<u32> = Some(255);
    let bytes =
        encode_to_vec_with_config(&val, cfg).expect("encode Option<u32> with fixed-int config");
    let (decoded, _): (Option<u32>, usize) = decode_from_slice_with_config(&bytes, cfg)
        .expect("decode Option<u32> with fixed-int config");
    assert_eq!(val, decoded);
}

#[test]
fn test_consumed_bytes_equals_encoded_length_for_option_config() {
    let val: Option<Config> = Some(Config {
        key: "max_conn".to_string(),
        value: Some("100".to_string()),
        priority: Some(1),
    });
    let bytes = encode_to_vec(&val).expect("encode Option<Config> for length check");
    let (_, consumed): (Option<Config>, usize) =
        decode_from_slice(&bytes).expect("decode Option<Config> for length check");
    assert_eq!(
        consumed,
        bytes.len(),
        "consumed bytes must equal the total encoded length"
    );
}
