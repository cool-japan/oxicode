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
struct AppError {
    code: u32,
    message: String,
}

#[derive(Debug, PartialEq, Encode, Decode)]
enum ApiError {
    NotFound,
    Unauthorized,
    ServerError(String),
}

#[test]
fn test_result_u32_string_ok_roundtrip() {
    let val: Result<u32, String> = Ok(42);
    let enc = encode_to_vec(&val).expect("encode ok");
    let (decoded, _): (Result<u32, String>, usize) = decode_from_slice(&enc).expect("decode ok");
    assert_eq!(decoded, Ok(42));
}

#[test]
fn test_result_u32_string_err_roundtrip() {
    let val: Result<u32, String> = Err("error".into());
    let enc = encode_to_vec(&val).expect("encode err");
    let (decoded, _): (Result<u32, String>, usize) = decode_from_slice(&enc).expect("decode err");
    assert_eq!(decoded, Err("error".to_string()));
}

#[test]
fn test_result_string_u32_ok_roundtrip() {
    let val: Result<String, u32> = Ok("hello".into());
    let enc = encode_to_vec(&val).expect("encode ok");
    let (decoded, _): (Result<String, u32>, usize) = decode_from_slice(&enc).expect("decode ok");
    assert_eq!(decoded, Ok("hello".to_string()));
}

#[test]
fn test_result_string_u32_err_roundtrip() {
    let val: Result<String, u32> = Err(99);
    let enc = encode_to_vec(&val).expect("encode err");
    let (decoded, _): (Result<String, u32>, usize) = decode_from_slice(&enc).expect("decode err");
    assert_eq!(decoded, Err(99u32));
}

#[test]
fn test_result_vec_u8_string_ok_roundtrip() {
    let val: Result<Vec<u8>, String> = Ok(vec![1, 2, 3, 4, 5]);
    let enc = encode_to_vec(&val).expect("encode ok");
    let (decoded, _): (Result<Vec<u8>, String>, usize) =
        decode_from_slice(&enc).expect("decode ok");
    assert_eq!(decoded, Ok(vec![1u8, 2, 3, 4, 5]));
}

#[test]
fn test_result_u32_apperror_ok_roundtrip() {
    let val: Result<u32, AppError> = Ok(100);
    let enc = encode_to_vec(&val).expect("encode ok");
    let (decoded, _): (Result<u32, AppError>, usize) = decode_from_slice(&enc).expect("decode ok");
    assert_eq!(decoded, Ok(100u32));
}

#[test]
fn test_result_u32_apperror_err_roundtrip() {
    let val: Result<u32, AppError> = Err(AppError {
        code: 404,
        message: "not found".to_string(),
    });
    let enc = encode_to_vec(&val).expect("encode err");
    let (decoded, _): (Result<u32, AppError>, usize) = decode_from_slice(&enc).expect("decode err");
    assert_eq!(
        decoded,
        Err(AppError {
            code: 404,
            message: "not found".to_string(),
        })
    );
}

#[test]
fn test_result_string_apierror_ok_roundtrip() {
    let val: Result<String, ApiError> = Ok("success".into());
    let enc = encode_to_vec(&val).expect("encode ok");
    let (decoded, _): (Result<String, ApiError>, usize) =
        decode_from_slice(&enc).expect("decode ok");
    assert_eq!(decoded, Ok("success".to_string()));
}

#[test]
fn test_result_string_apierror_err_notfound_roundtrip() {
    let val: Result<String, ApiError> = Err(ApiError::NotFound);
    let enc = encode_to_vec(&val).expect("encode err");
    let (decoded, _): (Result<String, ApiError>, usize) =
        decode_from_slice(&enc).expect("decode err");
    assert_eq!(decoded, Err(ApiError::NotFound));
}

#[test]
fn test_result_string_apierror_err_unauthorized_roundtrip() {
    let val: Result<String, ApiError> = Err(ApiError::Unauthorized);
    let enc = encode_to_vec(&val).expect("encode err");
    let (decoded, _): (Result<String, ApiError>, usize) =
        decode_from_slice(&enc).expect("decode err");
    assert_eq!(decoded, Err(ApiError::Unauthorized));
}

#[test]
fn test_result_string_apierror_err_servererror_roundtrip() {
    let val: Result<String, ApiError> = Err(ApiError::ServerError("internal".into()));
    let enc = encode_to_vec(&val).expect("encode err");
    let (decoded, _): (Result<String, ApiError>, usize) =
        decode_from_slice(&enc).expect("decode err");
    assert_eq!(decoded, Err(ApiError::ServerError("internal".to_string())));
}

#[test]
fn test_result_ok_discriminant_is_zero() {
    let val: Result<u32, String> = Ok(42);
    let enc = encode_to_vec(&val).expect("encode ok");
    assert_eq!(enc[0], 0, "Ok variant discriminant should be 0");
}

#[test]
fn test_result_err_discriminant_is_one() {
    let val: Result<u32, String> = Err("fail".into());
    let enc = encode_to_vec(&val).expect("encode err");
    assert_eq!(enc[0], 1, "Err variant discriminant should be 1");
}

#[test]
fn test_result_ok_and_err_produce_different_encodings() {
    let ok_val: Result<u32, u32> = Ok(7);
    let err_val: Result<u32, u32> = Err(7);
    let enc_ok = encode_to_vec(&ok_val).expect("encode ok");
    let enc_err = encode_to_vec(&err_val).expect("encode err");
    assert_ne!(enc_ok, enc_err, "Ok and Err encodings must differ");
}

#[test]
fn test_vec_of_results_mixed_roundtrip() {
    let val: Vec<Result<u32, String>> =
        vec![Ok(1), Err("bad".into()), Ok(2), Err("worse".into()), Ok(3)];
    let enc = encode_to_vec(&val).expect("encode vec");
    let (decoded, _): (Vec<Result<u32, String>>, usize) =
        decode_from_slice(&enc).expect("decode vec");
    assert_eq!(decoded[0], Ok(1u32));
    assert_eq!(decoded[1], Err("bad".to_string()));
    assert_eq!(decoded[2], Ok(2u32));
    assert_eq!(decoded[3], Err("worse".to_string()));
    assert_eq!(decoded[4], Ok(3u32));
}

#[test]
fn test_option_result_some_ok_roundtrip() {
    let val: Option<Result<u32, String>> = Some(Ok(1));
    let enc = encode_to_vec(&val).expect("encode some ok");
    let (decoded, _): (Option<Result<u32, String>>, usize) =
        decode_from_slice(&enc).expect("decode some ok");
    assert_eq!(decoded, Some(Ok(1u32)));
}

#[test]
fn test_option_result_some_err_roundtrip() {
    let val: Option<Result<u32, String>> = Some(Err("x".into()));
    let enc = encode_to_vec(&val).expect("encode some err");
    let (decoded, _): (Option<Result<u32, String>>, usize) =
        decode_from_slice(&enc).expect("decode some err");
    assert_eq!(decoded, Some(Err("x".to_string())));
}

#[test]
fn test_option_result_none_roundtrip() {
    let val: Option<Result<u32, String>> = None;
    let enc = encode_to_vec(&val).expect("encode none");
    let (decoded, _): (Option<Result<u32, String>>, usize) =
        decode_from_slice(&enc).expect("decode none");
    assert_eq!(decoded, None);
}

#[test]
fn test_result_unit_ok_roundtrip() {
    let val: Result<(), String> = Ok(());
    let enc = encode_to_vec(&val).expect("encode unit ok");
    let (decoded, _): (Result<(), String>, usize) =
        decode_from_slice(&enc).expect("decode unit ok");
    assert_eq!(decoded, Ok(()));
    // Ok(()) encodes just the discriminant byte (0) with no additional payload
    assert_eq!(enc[0], 0, "Ok(()) discriminant must be 0");
    assert_eq!(enc.len(), 1, "Ok(()) must encode to exactly 1 byte");
}

#[test]
fn test_result_u32_unit_err_roundtrip() {
    let val: Result<u32, ()> = Err(());
    let enc = encode_to_vec(&val).expect("encode unit err");
    let (decoded, _): (Result<u32, ()>, usize) = decode_from_slice(&enc).expect("decode unit err");
    assert_eq!(decoded, Err(()));
}

#[test]
fn test_result_u32_u32_fixed_int_config_roundtrip() {
    let cfg = config::standard().with_fixed_int_encoding();
    let ok_val: Result<u32, u32> = Ok(255);
    let err_val: Result<u32, u32> = Err(255);
    let enc_ok = encode_to_vec_with_config(&ok_val, cfg).expect("encode ok fixed");
    let enc_err = encode_to_vec_with_config(&err_val, cfg).expect("encode err fixed");
    let (decoded_ok, _): (Result<u32, u32>, usize) =
        decode_from_slice_with_config(&enc_ok, cfg).expect("decode ok fixed");
    let (decoded_err, _): (Result<u32, u32>, usize) =
        decode_from_slice_with_config(&enc_err, cfg).expect("decode err fixed");
    assert_eq!(decoded_ok, Ok(255u32));
    assert_eq!(decoded_err, Err(255u32));
}

#[test]
fn test_result_u64_string_consumed_bytes_equals_encoded_length() {
    let val: Result<u64, String> = Ok(123456789u64);
    let enc = encode_to_vec(&val).expect("encode u64");
    let (_, consumed): (Result<u64, String>, usize) = decode_from_slice(&enc).expect("decode u64");
    assert_eq!(
        consumed,
        enc.len(),
        "consumed bytes must equal total encoded length"
    );
}
