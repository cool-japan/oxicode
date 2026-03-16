//! Tests for Result<T, E> encoding with Service/API result types

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
enum ServiceError {
    Timeout { duration_ms: u64 },
    RateLimited { retry_after_ms: u64 },
    NotFound { resource: String },
    Unauthorized,
    InternalError { code: u32, detail: String },
}

#[derive(Debug, PartialEq, Encode, Decode)]
struct ServiceResponse {
    request_id: u64,
    status_code: u16,
    body: Vec<u8>,
    headers: Vec<(String, String)>,
}

// Test 1: Result<ServiceResponse, ServiceError>::Ok roundtrip
#[test]
fn test_result_service_response_ok_roundtrip() {
    let val: Result<ServiceResponse, ServiceError> = Ok(ServiceResponse {
        request_id: 42,
        status_code: 200,
        body: vec![1, 2, 3, 4, 5],
        headers: vec![
            ("Content-Type".to_string(), "application/json".to_string()),
            ("X-Request-Id".to_string(), "abc-123".to_string()),
        ],
    });
    let bytes = encode_to_vec(&val).expect("Failed to encode Ok(ServiceResponse)");
    let (decoded, _): (Result<ServiceResponse, ServiceError>, usize) =
        decode_from_slice(&bytes).expect("Failed to decode Ok(ServiceResponse)");
    assert_eq!(val, decoded);
}

// Test 2: Result<ServiceResponse, ServiceError>::Err(Timeout) roundtrip
#[test]
fn test_result_service_response_err_timeout_roundtrip() {
    let val: Result<ServiceResponse, ServiceError> =
        Err(ServiceError::Timeout { duration_ms: 5000 });
    let bytes = encode_to_vec(&val).expect("Failed to encode Err(Timeout)");
    let (decoded, _): (Result<ServiceResponse, ServiceError>, usize) =
        decode_from_slice(&bytes).expect("Failed to decode Err(Timeout)");
    assert_eq!(val, decoded);
}

// Test 3: Result<ServiceResponse, ServiceError>::Err(RateLimited) roundtrip
#[test]
fn test_result_service_response_err_rate_limited_roundtrip() {
    let val: Result<ServiceResponse, ServiceError> = Err(ServiceError::RateLimited {
        retry_after_ms: 30000,
    });
    let bytes = encode_to_vec(&val).expect("Failed to encode Err(RateLimited)");
    let (decoded, _): (Result<ServiceResponse, ServiceError>, usize) =
        decode_from_slice(&bytes).expect("Failed to decode Err(RateLimited)");
    assert_eq!(val, decoded);
}

// Test 4: Result<ServiceResponse, ServiceError>::Err(NotFound) roundtrip
#[test]
fn test_result_service_response_err_not_found_roundtrip() {
    let val: Result<ServiceResponse, ServiceError> = Err(ServiceError::NotFound {
        resource: "/api/users/99".to_string(),
    });
    let bytes = encode_to_vec(&val).expect("Failed to encode Err(NotFound)");
    let (decoded, _): (Result<ServiceResponse, ServiceError>, usize) =
        decode_from_slice(&bytes).expect("Failed to decode Err(NotFound)");
    assert_eq!(val, decoded);
}

// Test 5: Result<ServiceResponse, ServiceError>::Err(Unauthorized) roundtrip
#[test]
fn test_result_service_response_err_unauthorized_roundtrip() {
    let val: Result<ServiceResponse, ServiceError> = Err(ServiceError::Unauthorized);
    let bytes = encode_to_vec(&val).expect("Failed to encode Err(Unauthorized)");
    let (decoded, _): (Result<ServiceResponse, ServiceError>, usize) =
        decode_from_slice(&bytes).expect("Failed to decode Err(Unauthorized)");
    assert_eq!(val, decoded);
}

// Test 6: Result<ServiceResponse, ServiceError>::Err(InternalError) roundtrip
#[test]
fn test_result_service_response_err_internal_error_roundtrip() {
    let val: Result<ServiceResponse, ServiceError> = Err(ServiceError::InternalError {
        code: 500,
        detail: "Database connection failed".to_string(),
    });
    let bytes = encode_to_vec(&val).expect("Failed to encode Err(InternalError)");
    let (decoded, _): (Result<ServiceResponse, ServiceError>, usize) =
        decode_from_slice(&bytes).expect("Failed to decode Err(InternalError)");
    assert_eq!(val, decoded);
}

// Test 7: Result<u64, ServiceError>::Ok roundtrip
#[test]
fn test_result_u64_service_error_ok_roundtrip() {
    let val: Result<u64, ServiceError> = Ok(9_999_999_999_u64);
    let bytes = encode_to_vec(&val).expect("Failed to encode Ok(u64)");
    let (decoded, _): (Result<u64, ServiceError>, usize) =
        decode_from_slice(&bytes).expect("Failed to decode Ok(u64)");
    assert_eq!(val, decoded);
}

// Test 8: Result<u64, ServiceError>::Err(InternalError) roundtrip
#[test]
fn test_result_u64_service_error_err_internal_error_roundtrip() {
    let val: Result<u64, ServiceError> = Err(ServiceError::InternalError {
        code: 503,
        detail: "Service unavailable".to_string(),
    });
    let bytes = encode_to_vec(&val).expect("Failed to encode Err(InternalError) for u64 result");
    let (decoded, _): (Result<u64, ServiceError>, usize) =
        decode_from_slice(&bytes).expect("Failed to decode Err(InternalError) for u64 result");
    assert_eq!(val, decoded);
}

// Test 9: Result<String, String>::Ok roundtrip
#[test]
fn test_result_string_string_ok_roundtrip() {
    let val: Result<String, String> = Ok("Success response body".to_string());
    let bytes = encode_to_vec(&val).expect("Failed to encode Ok(String)");
    let (decoded, _): (Result<String, String>, usize) =
        decode_from_slice(&bytes).expect("Failed to decode Ok(String)");
    assert_eq!(val, decoded);
}

// Test 10: Result<String, String>::Err roundtrip
#[test]
fn test_result_string_string_err_roundtrip() {
    let val: Result<String, String> = Err("Error: connection refused".to_string());
    let bytes = encode_to_vec(&val).expect("Failed to encode Err(String)");
    let (decoded, _): (Result<String, String>, usize) =
        decode_from_slice(&bytes).expect("Failed to decode Err(String)");
    assert_eq!(val, decoded);
}

// Test 11: Vec<Result<ServiceResponse, ServiceError>> mixed roundtrip
#[test]
fn test_vec_of_service_results_mixed_roundtrip() {
    let val: Vec<Result<ServiceResponse, ServiceError>> = vec![
        Ok(ServiceResponse {
            request_id: 1,
            status_code: 200,
            body: vec![10, 20, 30],
            headers: vec![("Accept".to_string(), "text/html".to_string())],
        }),
        Err(ServiceError::Timeout { duration_ms: 3000 }),
        Ok(ServiceResponse {
            request_id: 2,
            status_code: 201,
            body: vec![],
            headers: vec![],
        }),
        Err(ServiceError::Unauthorized),
        Err(ServiceError::NotFound {
            resource: "/api/items/7".to_string(),
        }),
    ];
    let bytes =
        encode_to_vec(&val).expect("Failed to encode Vec<Result<ServiceResponse, ServiceError>>");
    let (decoded, _): (Vec<Result<ServiceResponse, ServiceError>>, usize) =
        decode_from_slice(&bytes)
            .expect("Failed to decode Vec<Result<ServiceResponse, ServiceError>>");
    assert_eq!(val, decoded);
}

// Test 12: Option<Result<u64, String>> Some(Ok) roundtrip
#[test]
fn test_option_result_u64_string_some_ok_roundtrip() {
    let val: Option<Result<u64, String>> = Some(Ok(12345_u64));
    let bytes = encode_to_vec(&val).expect("Failed to encode Some(Ok(u64))");
    let (decoded, _): (Option<Result<u64, String>>, usize) =
        decode_from_slice(&bytes).expect("Failed to decode Some(Ok(u64))");
    assert_eq!(val, decoded);
}

// Test 13: Option<Result<u64, String>> Some(Err) roundtrip
#[test]
fn test_option_result_u64_string_some_err_roundtrip() {
    let val: Option<Result<u64, String>> = Some(Err("token expired".to_string()));
    let bytes = encode_to_vec(&val).expect("Failed to encode Some(Err(String))");
    let (decoded, _): (Option<Result<u64, String>>, usize) =
        decode_from_slice(&bytes).expect("Failed to decode Some(Err(String))");
    assert_eq!(val, decoded);
}

// Test 14: Option<Result<u64, String>> None roundtrip
#[test]
fn test_option_result_u64_string_none_roundtrip() {
    let val: Option<Result<u64, String>> = None;
    let bytes = encode_to_vec(&val).expect("Failed to encode None");
    let (decoded, _): (Option<Result<u64, String>>, usize) =
        decode_from_slice(&bytes).expect("Failed to decode None");
    assert_eq!(val, decoded);
}

// Test 15: Ok variant discriminant is 0 (check enc[0] == 0)
#[test]
fn test_ok_variant_discriminant_is_zero() {
    let val: Result<u64, ServiceError> = Ok(1_u64);
    let bytes = encode_to_vec(&val).expect("Failed to encode Ok for discriminant check");
    assert_eq!(
        bytes[0], 0,
        "Ok variant discriminant should be 0, got {}",
        bytes[0]
    );
}

// Test 16: Err variant discriminant is 1 (check enc[0] == 1)
#[test]
fn test_err_variant_discriminant_is_one() {
    let val: Result<u64, ServiceError> = Err(ServiceError::Unauthorized);
    let bytes = encode_to_vec(&val).expect("Failed to encode Err for discriminant check");
    assert_eq!(
        bytes[0], 1,
        "Err variant discriminant should be 1, got {}",
        bytes[0]
    );
}

// Test 17: Result<(), ServiceError>::Ok(()) roundtrip — encodes to exactly 1 byte
#[test]
fn test_result_unit_ok_encodes_to_one_byte() {
    let val: Result<(), ServiceError> = Ok(());
    let bytes = encode_to_vec(&val).expect("Failed to encode Ok(())");
    assert_eq!(
        bytes.len(),
        1,
        "Ok(()) should encode to exactly 1 byte, got {} bytes",
        bytes.len()
    );
    let (decoded, _): (Result<(), ServiceError>, usize) =
        decode_from_slice(&bytes).expect("Failed to decode Ok(())");
    assert_eq!(val, decoded);
}

// Test 18: Result<Vec<u8>, u32>::Ok roundtrip with large payload
#[test]
fn test_result_vec_u8_ok_large_payload_roundtrip() {
    let large_body: Vec<u8> = (0_u16..1024).map(|i| (i % 256) as u8).collect();
    let val: Result<Vec<u8>, u32> = Ok(large_body);
    let bytes = encode_to_vec(&val).expect("Failed to encode Ok(large Vec<u8>)");
    let (decoded, _): (Result<Vec<u8>, u32>, usize) =
        decode_from_slice(&bytes).expect("Failed to decode Ok(large Vec<u8>)");
    assert_eq!(val, decoded);
}

// Test 19: Consumed bytes == encoded length for Result<ServiceResponse, ServiceError>
#[test]
fn test_consumed_bytes_equals_encoded_length_service_response() {
    let val: Result<ServiceResponse, ServiceError> = Ok(ServiceResponse {
        request_id: 777,
        status_code: 204,
        body: vec![0xFF, 0x00, 0xAB],
        headers: vec![("Cache-Control".to_string(), "no-cache".to_string())],
    });
    let bytes = encode_to_vec(&val).expect("Failed to encode for consumed bytes test");
    let (_, consumed): (Result<ServiceResponse, ServiceError>, usize) =
        decode_from_slice(&bytes).expect("Failed to decode for consumed bytes test");
    assert_eq!(
        consumed,
        bytes.len(),
        "Consumed bytes ({}) should equal encoded length ({})",
        consumed,
        bytes.len()
    );
}

// Test 20: Big-endian config Result roundtrip
#[test]
fn test_big_endian_config_result_roundtrip() {
    let cfg = config::standard().with_big_endian();
    let val: Result<ServiceResponse, ServiceError> = Ok(ServiceResponse {
        request_id: 100,
        status_code: 200,
        body: vec![1, 2, 3],
        headers: vec![("X-Custom".to_string(), "value".to_string())],
    });
    let bytes =
        encode_to_vec_with_config(&val, cfg).expect("Failed to encode with big_endian config");
    let (decoded, _): (Result<ServiceResponse, ServiceError>, usize) =
        decode_from_slice_with_config(&bytes, cfg)
            .expect("Failed to decode with big_endian config");
    assert_eq!(val, decoded);
}

// Test 21: Fixed-int config Result roundtrip
#[test]
fn test_fixed_int_config_result_roundtrip() {
    let cfg = config::standard().with_fixed_int_encoding();
    let val: Result<ServiceResponse, ServiceError> = Err(ServiceError::RateLimited {
        retry_after_ms: 60000,
    });
    let bytes =
        encode_to_vec_with_config(&val, cfg).expect("Failed to encode with fixed_int_encoding");
    let (decoded, _): (Result<ServiceResponse, ServiceError>, usize) =
        decode_from_slice_with_config(&bytes, cfg)
            .expect("Failed to decode with fixed_int_encoding");
    assert_eq!(val, decoded);
}

// Test 22: Encoding determinism: Ok and Err encode deterministically
#[test]
fn test_encoding_determinism_ok_and_err() {
    let ok_val: Result<ServiceResponse, ServiceError> = Ok(ServiceResponse {
        request_id: 55,
        status_code: 200,
        body: vec![9, 8, 7],
        headers: vec![("ETag".to_string(), "\"abc123\"".to_string())],
    });
    let err_val: Result<ServiceResponse, ServiceError> = Err(ServiceError::InternalError {
        code: 500,
        detail: "unexpected EOF".to_string(),
    });

    let ok_bytes_1 = encode_to_vec(&ok_val).expect("Failed to encode Ok (first time)");
    let ok_bytes_2 = encode_to_vec(&ok_val).expect("Failed to encode Ok (second time)");
    assert_eq!(
        ok_bytes_1, ok_bytes_2,
        "Ok encoding should be deterministic"
    );

    let err_bytes_1 = encode_to_vec(&err_val).expect("Failed to encode Err (first time)");
    let err_bytes_2 = encode_to_vec(&err_val).expect("Failed to encode Err (second time)");
    assert_eq!(
        err_bytes_1, err_bytes_2,
        "Err encoding should be deterministic"
    );
}
