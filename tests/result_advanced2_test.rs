//! Advanced tests (set 2) for `Result<T, E>` encoding and decoding in OxiCode.

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

// ---- Test 1: Result<u32, String> Ok roundtrip ----

#[test]
fn test_result_ok_u32_string_roundtrip() {
    let original: Result<u32, String> = Ok(1234u32);
    let encoded = encode_to_vec(&original).expect("encode Ok(u32) failed");
    let (val, _bytes): (Result<u32, String>, usize) =
        decode_from_slice(&encoded).expect("decode Ok(u32) failed");
    assert_eq!(val, Ok(1234u32));
}

// ---- Test 2: Result<u32, String> Err roundtrip ----

#[test]
fn test_result_err_u32_string_roundtrip() {
    let original: Result<u32, String> = Err(String::from("failure"));
    let encoded = encode_to_vec(&original).expect("encode Err(String) failed");
    let (val, _bytes): (Result<u32, String>, usize) =
        decode_from_slice(&encoded).expect("decode Err(String) failed");
    assert_eq!(val, Err(String::from("failure")));
}

// ---- Test 3: Result<String, u32> Ok roundtrip ----

#[test]
fn test_result_ok_string_u32_roundtrip() {
    let original: Result<String, u32> = Ok(String::from("hello"));
    let encoded = encode_to_vec(&original).expect("encode Ok(String) failed");
    let (val, _bytes): (Result<String, u32>, usize) =
        decode_from_slice(&encoded).expect("decode Ok(String) failed");
    assert_eq!(val, Ok(String::from("hello")));
}

// ---- Test 4: Result<String, u32> Err roundtrip ----

#[test]
fn test_result_err_string_u32_roundtrip() {
    let original: Result<String, u32> = Err(99u32);
    let encoded = encode_to_vec(&original).expect("encode Err(u32) failed");
    let (val, _bytes): (Result<String, u32>, usize) =
        decode_from_slice(&encoded).expect("decode Err(u32) failed");
    assert_eq!(val, Err(99u32));
}

// ---- Test 5: Result<u64, u64> Ok vs Err produce different wire bytes ----

#[test]
fn test_result_u64_ok_vs_err_different_bytes() {
    let ok_val: Result<u64, u64> = Ok(100u64);
    let err_val: Result<u64, u64> = Err(100u64);
    let ok_bytes = encode_to_vec(&ok_val).expect("encode Ok(u64) failed");
    let err_bytes = encode_to_vec(&err_val).expect("encode Err(u64) failed");
    assert_ne!(
        ok_bytes, err_bytes,
        "Ok and Err with same payload must produce different bytes"
    );
}

// ---- Test 6: Result<Vec<u8>, String> Ok roundtrip ----

#[test]
fn test_result_ok_vec_u8_string_roundtrip() {
    let payload: Vec<u8> = vec![1, 2, 3, 4, 5];
    let original: Result<Vec<u8>, String> = Ok(payload.clone());
    let encoded = encode_to_vec(&original).expect("encode Ok(Vec<u8>) failed");
    let (val, _bytes): (Result<Vec<u8>, String>, usize) =
        decode_from_slice(&encoded).expect("decode Ok(Vec<u8>) failed");
    assert_eq!(val, Ok(payload));
}

// ---- Test 7: Result<Vec<u8>, String> Err roundtrip ----

#[test]
fn test_result_err_vec_u8_string_roundtrip() {
    let original: Result<Vec<u8>, String> = Err(String::from("byte error"));
    let encoded = encode_to_vec(&original).expect("encode Err(String) for vec test failed");
    let (val, _bytes): (Result<Vec<u8>, String>, usize) =
        decode_from_slice(&encoded).expect("decode Err(String) for vec test failed");
    assert_eq!(val, Err(String::from("byte error")));
}

// ---- Test 8: Result<u32, String> Ok consumed == encoded length ----

#[test]
fn test_result_ok_consumed_equals_encoded_len() {
    let original: Result<u32, String> = Ok(7u32);
    let encoded = encode_to_vec(&original).expect("encode Ok(7u32) failed");
    let (_val, consumed): (Result<u32, String>, usize) =
        decode_from_slice(&encoded).expect("decode Ok(7u32) failed");
    assert_eq!(consumed, encoded.len());
}

// ---- Test 9: Result<u32, String> Err consumed == encoded length ----

#[test]
fn test_result_err_consumed_equals_encoded_len() {
    let original: Result<u32, String> = Err(String::from("err msg"));
    let encoded = encode_to_vec(&original).expect("encode Err(msg) failed");
    let (_val, consumed): (Result<u32, String>, usize) =
        decode_from_slice(&encoded).expect("decode Err(msg) failed");
    assert_eq!(consumed, encoded.len());
}

// ---- Test 10: Vec<Result<u32, String>> mixed Ok/Err roundtrip ----

#[test]
fn test_vec_of_results_mixed_roundtrip() {
    let original: Vec<Result<u32, String>> = vec![
        Ok(1u32),
        Err(String::from("e1")),
        Ok(2u32),
        Err(String::from("e2")),
        Ok(3u32),
    ];
    let encoded = encode_to_vec(&original).expect("encode Vec<Result> failed");
    let (val, _bytes): (Vec<Result<u32, String>>, usize) =
        decode_from_slice(&encoded).expect("decode Vec<Result> failed");
    assert_eq!(val, original);
}

// ---- Test 11: Option<Result<u32, String>> Some(Ok) roundtrip ----

#[test]
fn test_option_result_some_ok_roundtrip() {
    let original: Option<Result<u32, String>> = Some(Ok(42u32));
    let encoded = encode_to_vec(&original).expect("encode Some(Ok(42)) failed");
    let (val, _bytes): (Option<Result<u32, String>>, usize) =
        decode_from_slice(&encoded).expect("decode Some(Ok(42)) failed");
    assert_eq!(val, Some(Ok(42u32)));
}

// ---- Test 12: Option<Result<u32, String>> Some(Err) roundtrip ----

#[test]
fn test_option_result_some_err_roundtrip() {
    let original: Option<Result<u32, String>> = Some(Err(String::from("opt err")));
    let encoded = encode_to_vec(&original).expect("encode Some(Err) failed");
    let (val, _bytes): (Option<Result<u32, String>>, usize) =
        decode_from_slice(&encoded).expect("decode Some(Err) failed");
    assert_eq!(val, Some(Err(String::from("opt err"))));
}

// ---- Test 13: Option<Result<u32, String>> None roundtrip ----

#[test]
fn test_option_result_none_roundtrip() {
    let original: Option<Result<u32, String>> = None;
    let encoded = encode_to_vec(&original).expect("encode None failed");
    let (val, _bytes): (Option<Result<u32, String>>, usize) =
        decode_from_slice(&encoded).expect("decode None failed");
    assert_eq!(val, None);
}

// ---- Test 14: Result<u32, String> with fixed-int config Ok roundtrip ----

#[test]
fn test_result_ok_fixed_int_config_roundtrip() {
    let original: Result<u32, String> = Ok(555u32);
    let cfg = config::standard().with_fixed_int_encoding();
    let encoded = encode_to_vec_with_config(&original, cfg).expect("encode Ok fixed-int failed");
    let (val, _bytes): (Result<u32, String>, usize) =
        decode_from_slice_with_config(&encoded, cfg).expect("decode Ok fixed-int failed");
    assert_eq!(val, Ok(555u32));
}

// ---- Test 15: Result<u32, String> with big-endian config Ok roundtrip ----

#[test]
fn test_result_ok_big_endian_config_roundtrip() {
    let original: Result<u32, String> = Ok(777u32);
    let cfg = config::standard().with_big_endian();
    let encoded = encode_to_vec_with_config(&original, cfg).expect("encode Ok big-endian failed");
    let (val, _bytes): (Result<u32, String>, usize) =
        decode_from_slice_with_config(&encoded, cfg).expect("decode Ok big-endian failed");
    assert_eq!(val, Ok(777u32));
}

// ---- Test 16: Struct containing Result<u32, String> and id: u64 roundtrip ----

#[derive(Encode, Decode, PartialEq, Debug)]
struct ResultHolder {
    result: Result<u32, String>,
    id: u64,
}

#[test]
fn test_struct_with_result_field_roundtrip() {
    let original = ResultHolder {
        result: Ok(100u32),
        id: 9999u64,
    };
    let encoded = encode_to_vec(&original).expect("encode ResultHolder failed");
    let (val, _bytes): (ResultHolder, usize) =
        decode_from_slice(&encoded).expect("decode ResultHolder failed");
    assert_eq!(val, original);
}

// ---- Test 17: Result<bool, bool> all 4 combinations ----

#[test]
fn test_result_bool_bool_all_combinations() {
    let combos: Vec<Result<bool, bool>> = vec![Ok(true), Ok(false), Err(true), Err(false)];
    for combo in &combos {
        let encoded = encode_to_vec(combo).expect("encode Result<bool,bool> failed");
        let (val, _bytes): (Result<bool, bool>, usize) =
            decode_from_slice(&encoded).expect("decode Result<bool,bool> failed");
        assert_eq!(&val, combo);
    }
}

// ---- Test 18: Result<Result<u32, String>, String> nested Ok(Ok) roundtrip ----

#[test]
fn test_nested_result_ok_ok_roundtrip() {
    let original: Result<Result<u32, String>, String> = Ok(Ok(88u32));
    let encoded = encode_to_vec(&original).expect("encode Ok(Ok(88)) failed");
    let (val, _bytes): (Result<Result<u32, String>, String>, usize) =
        decode_from_slice(&encoded).expect("decode Ok(Ok(88)) failed");
    assert_eq!(val, Ok(Ok(88u32)));
}

// ---- Test 19: Result<Result<u32, String>, String> nested Ok(Err) roundtrip ----

#[test]
fn test_nested_result_ok_err_roundtrip() {
    let original: Result<Result<u32, String>, String> = Ok(Err(String::from("inner err")));
    let encoded = encode_to_vec(&original).expect("encode Ok(Err) failed");
    let (val, _bytes): (Result<Result<u32, String>, String>, usize) =
        decode_from_slice(&encoded).expect("decode Ok(Err) failed");
    assert_eq!(val, Ok(Err(String::from("inner err"))));
}

// ---- Test 20: Result<(), String> Ok(()) roundtrip ----

#[test]
fn test_result_ok_unit_string_roundtrip() {
    let original: Result<(), String> = Ok(());
    let encoded = encode_to_vec(&original).expect("encode Ok(()) failed");
    let (val, _bytes): (Result<(), String>, usize) =
        decode_from_slice(&encoded).expect("decode Ok(()) failed");
    assert_eq!(val, Ok(()));
}

// ---- Test 21: Result<u32, ()> Err(()) roundtrip ----

#[test]
fn test_result_err_u32_unit_roundtrip() {
    let original: Result<u32, ()> = Err(());
    let encoded = encode_to_vec(&original).expect("encode Err(()) failed");
    let (val, _bytes): (Result<u32, ()>, usize) =
        decode_from_slice(&encoded).expect("decode Err(()) failed");
    assert_eq!(val, Err(()));
}

// ---- Test 22: Re-encode decoded Result gives same bytes ----

#[test]
fn test_reencode_decoded_result_gives_same_bytes() {
    let original: Result<u32, String> = Ok(321u32);
    let encoded_first = encode_to_vec(&original).expect("first encode failed");
    let (decoded, _bytes): (Result<u32, String>, usize) =
        decode_from_slice(&encoded_first).expect("first decode failed");
    let encoded_second = encode_to_vec(&decoded).expect("re-encode failed");
    assert_eq!(
        encoded_first, encoded_second,
        "re-encoded bytes must match original encoding"
    );
}
