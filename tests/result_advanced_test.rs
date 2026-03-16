//! Advanced tests for `Result<T, E>` encoding and decoding in OxiCode.

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
    encode_to_vec_with_config,
};

// ---- Test 1: Result<u32, String> Ok variant roundtrip ----

#[test]
fn test_result_ok_u32_string_roundtrip() {
    let original: Result<u32, String> = Ok(42u32);
    let encoded = encode_to_vec(&original).expect("encode Ok(42u32) failed");
    let (decoded, consumed): (Result<u32, String>, _) =
        decode_from_slice(&encoded).expect("decode Ok(42u32) failed");
    assert_eq!(decoded, Ok(42u32));
    assert_eq!(consumed, encoded.len());
}

// ---- Test 2: Result<u32, String> Err variant roundtrip ----

#[test]
fn test_result_err_u32_string_roundtrip() {
    let original: Result<u32, String> = Err(String::from("error message"));
    let encoded = encode_to_vec(&original).expect("encode Err(String) failed");
    let (decoded, consumed): (Result<u32, String>, _) =
        decode_from_slice(&encoded).expect("decode Err(String) failed");
    assert_eq!(decoded, Err(String::from("error message")));
    assert_eq!(consumed, encoded.len());
}

// ---- Test 3: Result<(), ()> Ok and Err ----

#[test]
fn test_result_unit_ok_and_err() {
    let ok_val: Result<(), ()> = Ok(());
    let encoded_ok = encode_to_vec(&ok_val).expect("encode Ok(()) failed");
    let (decoded_ok, _): (Result<(), ()>, _) =
        decode_from_slice(&encoded_ok).expect("decode Ok(()) failed");
    assert_eq!(decoded_ok, Ok(()));

    let err_val: Result<(), ()> = Err(());
    let encoded_err = encode_to_vec(&err_val).expect("encode Err(()) failed");
    let (decoded_err, _): (Result<(), ()>, _) =
        decode_from_slice(&encoded_err).expect("decode Err(()) failed");
    assert_eq!(decoded_err, Err(()));
}

// ---- Test 4: Result<Vec<u8>, i32> large Ok ----

#[test]
fn test_result_ok_large_vec() {
    let payload: Vec<u8> = (0u8..=255).collect();
    let original: Result<Vec<u8>, i32> = Ok(payload.clone());
    let encoded = encode_to_vec(&original).expect("encode Ok(large vec) failed");
    let (decoded, consumed): (Result<Vec<u8>, i32>, _) =
        decode_from_slice(&encoded).expect("decode Ok(large vec) failed");
    assert_eq!(decoded, Ok(payload));
    assert_eq!(consumed, encoded.len());
}

// ---- Test 5: Result<String, Vec<u8>> Err with bytes ----

#[test]
fn test_result_err_with_bytes() {
    let original: Result<String, Vec<u8>> = Err(vec![1u8, 2, 3, 4, 5]);
    let encoded = encode_to_vec(&original).expect("encode Err(Vec<u8>) failed");
    let (decoded, consumed): (Result<String, Vec<u8>>, _) =
        decode_from_slice(&encoded).expect("decode Err(Vec<u8>) failed");
    assert_eq!(decoded, Err(vec![1u8, 2, 3, 4, 5]));
    assert_eq!(consumed, encoded.len());
}

// ---- Test 6: Nested Result<Result<u32, String>, bool> Ok(Ok) ----

#[test]
fn test_result_nested_ok() {
    let original: Result<Result<u32, String>, bool> = Ok(Ok(99u32));
    let encoded = encode_to_vec(&original).expect("encode nested Ok(Ok) failed");
    let (decoded, consumed): (Result<Result<u32, String>, bool>, _) =
        decode_from_slice(&encoded).expect("decode nested Ok(Ok) failed");
    assert_eq!(decoded, Ok(Ok(99u32)));
    assert_eq!(consumed, encoded.len());
}

// ---- Test 7: Nested Result<Result<u32, String>, bool> Ok(Err) ----

#[test]
fn test_result_nested_err_inner() {
    let original: Result<Result<u32, String>, bool> = Ok(Err(String::from("inner error")));
    let encoded = encode_to_vec(&original).expect("encode nested Ok(Err) failed");
    let (decoded, consumed): (Result<Result<u32, String>, bool>, _) =
        decode_from_slice(&encoded).expect("decode nested Ok(Err) failed");
    assert_eq!(decoded, Ok(Err(String::from("inner error"))));
    assert_eq!(consumed, encoded.len());
}

// ---- Test 8: Result<Option<u64>, String> Ok(Some) ----

#[test]
fn test_result_option_some_ok() {
    let original: Result<Option<u64>, String> = Ok(Some(12345u64));
    let encoded = encode_to_vec(&original).expect("encode Ok(Some(u64)) failed");
    let (decoded, consumed): (Result<Option<u64>, String>, _) =
        decode_from_slice(&encoded).expect("decode Ok(Some(u64)) failed");
    assert_eq!(decoded, Ok(Some(12345u64)));
    assert_eq!(consumed, encoded.len());
}

// ---- Test 9: Result<Option<u64>, String> Ok(None) ----

#[test]
fn test_result_option_none_ok() {
    let original: Result<Option<u64>, String> = Ok(None);
    let encoded = encode_to_vec(&original).expect("encode Ok(None) failed");
    let (decoded, consumed): (Result<Option<u64>, String>, _) =
        decode_from_slice(&encoded).expect("decode Ok(None) failed");
    assert_eq!(decoded, Ok(None));
    assert_eq!(consumed, encoded.len());
}

// ---- Test 10: Result<Option<u64>, String> Err ----

#[test]
fn test_result_option_err() {
    let original: Result<Option<u64>, String> = Err(String::from("failure"));
    let encoded = encode_to_vec(&original).expect("encode Err(String) for Option result failed");
    let (decoded, consumed): (Result<Option<u64>, String>, _) =
        decode_from_slice(&encoded).expect("decode Err(String) for Option result failed");
    assert_eq!(decoded, Err(String::from("failure")));
    assert_eq!(consumed, encoded.len());
}

// ---- Test 11: Vec of Results roundtrip ----

#[test]
fn test_result_vec_of_results() {
    let original: Vec<Result<u32, String>> = vec![
        Ok(1u32),
        Err(String::from("e")),
        Ok(3u32),
        Err(String::from("oops")),
        Ok(0u32),
    ];
    let encoded = encode_to_vec(&original).expect("encode Vec<Result> failed");
    let (decoded, consumed): (Vec<Result<u32, String>>, _) =
        decode_from_slice(&encoded).expect("decode Vec<Result> failed");
    assert_eq!(decoded, original);
    assert_eq!(consumed, encoded.len());
}

// ---- Test 12: Result<(u32, u64), String> tuple Ok ----

#[test]
fn test_result_tuple_ok() {
    let original: Result<(u32, u64), String> = Ok((100u32, 200u64));
    let encoded = encode_to_vec(&original).expect("encode Ok((u32, u64)) failed");
    let (decoded, consumed): (Result<(u32, u64), String>, _) =
        decode_from_slice(&encoded).expect("decode Ok((u32, u64)) failed");
    assert_eq!(decoded, Ok((100u32, 200u64)));
    assert_eq!(consumed, encoded.len());
}

// ---- Test 13: Result<u32, u32> same type both variants ----

#[test]
fn test_result_same_type_both_variants() {
    let ok_val: Result<u32, u32> = Ok(5u32);
    let encoded_ok = encode_to_vec(&ok_val).expect("encode Ok(5u32) same type failed");
    let (decoded_ok, _): (Result<u32, u32>, _) =
        decode_from_slice(&encoded_ok).expect("decode Ok(5u32) same type failed");
    assert_eq!(decoded_ok, Ok(5u32));

    let err_val: Result<u32, u32> = Err(7u32);
    let encoded_err = encode_to_vec(&err_val).expect("encode Err(7u32) same type failed");
    let (decoded_err, _): (Result<u32, u32>, _) =
        decode_from_slice(&encoded_err).expect("decode Err(7u32) same type failed");
    assert_eq!(decoded_err, Err(7u32));
}

// ---- Test 14: Result<i128, u128> large int results ----

#[test]
fn test_result_i128_u128() {
    let ok_val: Result<i128, u128> = Ok(i128::MIN);
    let encoded_ok = encode_to_vec(&ok_val).expect("encode Ok(i128::MIN) failed");
    let (decoded_ok, _): (Result<i128, u128>, _) =
        decode_from_slice(&encoded_ok).expect("decode Ok(i128::MIN) failed");
    assert_eq!(decoded_ok, Ok(i128::MIN));

    let err_val: Result<i128, u128> = Err(u128::MAX);
    let encoded_err = encode_to_vec(&err_val).expect("encode Err(u128::MAX) failed");
    let (decoded_err, _): (Result<i128, u128>, _) =
        decode_from_slice(&encoded_err).expect("decode Err(u128::MAX) failed");
    assert_eq!(decoded_err, Err(u128::MAX));
}

// ---- Test 15: Fixed int encoding with Result ----

#[test]
fn test_result_fixed_int_encoding() {
    let cfg = config::standard().with_fixed_int_encoding();
    let original: Result<u32, String> = Ok(42u32);
    let encoded =
        encode_to_vec_with_config(&original, cfg).expect("encode with fixed int config failed");
    let (decoded, consumed): (Result<u32, String>, _) =
        decode_from_slice_with_config(&encoded, cfg).expect("decode with fixed int config failed");
    assert_eq!(decoded, Ok(42u32));
    assert_eq!(consumed, encoded.len());
}

// ---- Test 16: Big endian config with Result ----

#[test]
fn test_result_big_endian_config() {
    let cfg = config::standard().with_big_endian();
    let original: Result<u32, String> = Ok(1234u32);
    let encoded =
        encode_to_vec_with_config(&original, cfg).expect("encode with big endian config failed");
    let (decoded, consumed): (Result<u32, String>, _) =
        decode_from_slice_with_config(&encoded, cfg).expect("decode with big endian config failed");
    assert_eq!(decoded, Ok(1234u32));
    assert_eq!(consumed, encoded.len());
}

// ---- Test 17: Result<bool, bool> tiny types ----

#[test]
fn test_result_bool_variants() {
    let ok_val: Result<bool, bool> = Ok(true);
    let encoded_ok = encode_to_vec(&ok_val).expect("encode Ok(true) bool failed");
    let (decoded_ok, _): (Result<bool, bool>, _) =
        decode_from_slice(&encoded_ok).expect("decode Ok(true) bool failed");
    assert_eq!(decoded_ok, Ok(true));

    let err_val: Result<bool, bool> = Err(false);
    let encoded_err = encode_to_vec(&err_val).expect("encode Err(false) bool failed");
    let (decoded_err, _): (Result<bool, bool>, _) =
        decode_from_slice(&encoded_err).expect("decode Err(false) bool failed");
    assert_eq!(decoded_err, Err(false));
}

// ---- Test 18: Result<f64, f64> float results ----

#[test]
fn test_result_float_variants() {
    let ok_val: Result<f64, f64> = Ok(std::f64::consts::PI);
    let encoded_ok = encode_to_vec(&ok_val).expect("encode Ok(PI) failed");
    let (decoded_ok, _): (Result<f64, f64>, _) =
        decode_from_slice(&encoded_ok).expect("decode Ok(PI) failed");
    assert_eq!(decoded_ok, Ok(std::f64::consts::PI));

    let err_val: Result<f64, f64> = Err(std::f64::consts::E);
    let encoded_err = encode_to_vec(&err_val).expect("encode Err(E) failed");
    let (decoded_err, _): (Result<f64, f64>, _) =
        decode_from_slice(&encoded_err).expect("decode Err(E) failed");
    assert_eq!(decoded_err, Err(std::f64::consts::E));
}

// ---- Test 19: Byte size of Ok vs Err (payloads differ so total sizes differ) ----

#[test]
fn test_result_byte_size_ok_vs_err() {
    // Ok(0u32): discriminant varint(0) = 1 byte, payload varint(0) = 1 byte => 2 bytes total
    // Err("hello"): discriminant varint(1) = 1 byte, len varint(5) = 1 byte, "hello" = 5 bytes => 7 bytes total
    let ok_val: Result<u32, String> = Ok(0u32);
    let err_val: Result<u32, String> = Err(String::from("hello"));

    let encoded_ok = encode_to_vec(&ok_val).expect("encode Ok(0u32) for size test failed");
    let encoded_err = encode_to_vec(&err_val).expect("encode Err(hello) for size test failed");

    // The Ok payload (u32=0) is much smaller than Err payload (5-char string)
    assert_ne!(
        encoded_ok.len(),
        encoded_err.len(),
        "Ok and Err with different payloads should have different encoded sizes"
    );
    // Ok is 2 bytes, Err is 7 bytes
    assert!(
        encoded_ok.len() < encoded_err.len(),
        "Ok(0u32) should encode smaller than Err(\"hello\")"
    );
}

// ---- Test 20: Result<char, u8> char result ----

#[test]
fn test_result_char_ok() {
    let original: Result<char, u8> = Ok('A');
    let encoded = encode_to_vec(&original).expect("encode Ok('A') failed");
    let (decoded, consumed): (Result<char, u8>, _) =
        decode_from_slice(&encoded).expect("decode Ok('A') failed");
    assert_eq!(decoded, Ok('A'));
    assert_eq!(consumed, encoded.len());

    let err_val: Result<char, u8> = Err(255u8);
    let encoded_err = encode_to_vec(&err_val).expect("encode Err(255u8) for char result failed");
    let (decoded_err, _): (Result<char, u8>, _) =
        decode_from_slice(&encoded_err).expect("decode Err(255u8) for char result failed");
    assert_eq!(decoded_err, Err(255u8));
}

// ---- Test 21: Result<[u8; 4], String> array Ok ----

#[test]
fn test_result_array_ok() {
    let original: Result<[u8; 4], String> = Ok([1u8, 2, 3, 4]);
    let encoded = encode_to_vec(&original).expect("encode Ok([u8;4]) failed");
    let (decoded, consumed): (Result<[u8; 4], String>, _) =
        decode_from_slice(&encoded).expect("decode Ok([u8;4]) failed");
    assert_eq!(decoded, Ok([1u8, 2, 3, 4]));
    assert_eq!(consumed, encoded.len());

    let err_val: Result<[u8; 4], String> = Err(String::from("array error"));
    let encoded_err = encode_to_vec(&err_val).expect("encode Err(String) for array result failed");
    let (decoded_err, _): (Result<[u8; 4], String>, _) =
        decode_from_slice(&encoded_err).expect("decode Err(String) for array result failed");
    assert_eq!(decoded_err, Err(String::from("array error")));
}

// ---- Test 22: Multiple Results in a tuple (Result<u32,String>, Result<bool,i64>) ----

#[test]
fn test_result_tuple_of_results() {
    let original: (Result<u32, String>, Result<bool, i64>) = (Ok(42u32), Err(-1i64));
    let encoded = encode_to_vec(&original).expect("encode tuple of Results failed");
    #[allow(clippy::type_complexity)]
    let (decoded, consumed): ((Result<u32, String>, Result<bool, i64>), _) =
        decode_from_slice(&encoded).expect("decode tuple of Results failed");
    assert_eq!(decoded.0, Ok(42u32));
    assert_eq!(decoded.1, Err(-1i64));
    assert_eq!(consumed, encoded.len());
}
