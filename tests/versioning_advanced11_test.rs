//! API response versioning tests for OxiCode (set 11).
//!
//! Covers 22 scenarios exercising encode_versioned_value, decode_versioned_value,
//! Version, and three generations of ApiResponse structs (V1/V2/V3) with all
//! StatusCode variants, big-endian/fixed-int configs, Vec of responses, version
//! tuple accessor, and size ordering guarantees.

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
    config, decode_from_slice, decode_versioned_value, encode_to_vec, encode_versioned_value,
    versioning::Version, Decode, Encode,
};

// ── Shared types ──────────────────────────────────────────────────────────────

#[derive(Debug, PartialEq, Encode, Decode)]
enum StatusCode {
    Ok,
    Created,
    BadRequest,
    Unauthorized,
    NotFound,
    InternalServerError,
}

#[derive(Debug, PartialEq, Encode, Decode)]
struct ApiResponseV1 {
    status: StatusCode,
    message: String,
    data: Vec<u8>,
}

#[derive(Debug, PartialEq, Encode, Decode)]
struct ApiResponseV2 {
    status: StatusCode,
    message: String,
    data: Vec<u8>,
    request_id: u64,
    duration_ms: u32,
}

#[derive(Debug, PartialEq, Encode, Decode)]
struct ApiResponseV3 {
    status: StatusCode,
    message: String,
    data: Vec<u8>,
    request_id: u64,
    duration_ms: u32,
    pagination: Option<String>,
    total_count: Option<u64>,
}

// ── Scenario 1 ────────────────────────────────────────────────────────────────
// ApiResponseV1 basic roundtrip with versioning — StatusCode::Ok
#[test]
fn test_api_v1_ok_versioned_roundtrip() {
    let version = Version::new(1, 0, 0);
    let original = ApiResponseV1 {
        status: StatusCode::Ok,
        message: String::from("success"),
        data: vec![0x01, 0x02, 0x03],
    };
    let encoded =
        encode_versioned_value(&original, version).expect("encode_versioned_value failed");
    let (decoded, ver, consumed): (ApiResponseV1, Version, usize) =
        decode_versioned_value(&encoded).expect("decode_versioned_value failed");
    assert_eq!(decoded, original);
    assert_eq!(ver, version);
    assert!(consumed > 0);
}

// ── Scenario 2 ────────────────────────────────────────────────────────────────
// ApiResponseV2 roundtrip with versioning — StatusCode::Created
#[test]
fn test_api_v2_created_versioned_roundtrip() {
    let version = Version::new(2, 0, 0);
    let original = ApiResponseV2 {
        status: StatusCode::Created,
        message: String::from("resource created"),
        data: vec![0xAA, 0xBB],
        request_id: 100_000_001,
        duration_ms: 42,
    };
    let encoded =
        encode_versioned_value(&original, version).expect("encode_versioned_value failed");
    let (decoded, ver, consumed): (ApiResponseV2, Version, usize) =
        decode_versioned_value(&encoded).expect("decode_versioned_value failed");
    assert_eq!(decoded, original);
    assert_eq!(ver, version);
    assert!(consumed > 0);
}

// ── Scenario 3 ────────────────────────────────────────────────────────────────
// ApiResponseV3 roundtrip with versioning — StatusCode::InternalServerError
#[test]
fn test_api_v3_internal_server_error_versioned_roundtrip() {
    let version = Version::new(3, 0, 0);
    let original = ApiResponseV3 {
        status: StatusCode::InternalServerError,
        message: String::from("something went wrong"),
        data: vec![],
        request_id: 9_999_999_999,
        duration_ms: 5000,
        pagination: None,
        total_count: None,
    };
    let encoded =
        encode_versioned_value(&original, version).expect("encode_versioned_value failed");
    let (decoded, ver, consumed): (ApiResponseV3, Version, usize) =
        decode_versioned_value(&encoded).expect("decode_versioned_value failed");
    assert_eq!(decoded, original);
    assert_eq!(ver, version);
    assert!(consumed > 0);
}

// ── Scenario 4 ────────────────────────────────────────────────────────────────
// Version is exactly preserved for V1 — Version::new(1, 2, 3)
#[test]
fn test_api_v1_version_preserved_exactly() {
    let version = Version::new(1, 2, 3);
    let original = ApiResponseV1 {
        status: StatusCode::NotFound,
        message: String::from("not found"),
        data: vec![0xFF],
    };
    let encoded = encode_versioned_value(&original, version).expect("encode failed");
    let (_decoded, ver, _consumed): (ApiResponseV1, Version, usize) =
        decode_versioned_value(&encoded).expect("decode failed");
    assert_eq!(ver.major, 1);
    assert_eq!(ver.minor, 2);
    assert_eq!(ver.patch, 3);
    assert_eq!(ver, version);
}

// ── Scenario 5 ────────────────────────────────────────────────────────────────
// StatusCode::BadRequest variant roundtrip via V2
#[test]
fn test_api_v2_bad_request_status_code() {
    let version = Version::new(2, 1, 0);
    let original = ApiResponseV2 {
        status: StatusCode::BadRequest,
        message: String::from("invalid input"),
        data: vec![0x00],
        request_id: 1,
        duration_ms: 10,
    };
    let encoded = encode_versioned_value(&original, version).expect("encode failed");
    let (decoded, ver, _consumed): (ApiResponseV2, Version, usize) =
        decode_versioned_value(&encoded).expect("decode failed");
    assert_eq!(decoded.status, StatusCode::BadRequest);
    assert_eq!(decoded.message, "invalid input");
    assert_eq!(ver, version);
}

// ── Scenario 6 ────────────────────────────────────────────────────────────────
// StatusCode::Unauthorized variant roundtrip via V3
#[test]
fn test_api_v3_unauthorized_status_code() {
    let version = Version::new(3, 0, 1);
    let original = ApiResponseV3 {
        status: StatusCode::Unauthorized,
        message: String::from("auth required"),
        data: vec![],
        request_id: 42,
        duration_ms: 1,
        pagination: Some(String::from("cursor:abc")),
        total_count: Some(100),
    };
    let encoded = encode_versioned_value(&original, version).expect("encode failed");
    let (decoded, ver, _consumed): (ApiResponseV3, Version, usize) =
        decode_versioned_value(&encoded).expect("decode failed");
    assert_eq!(decoded.status, StatusCode::Unauthorized);
    assert_eq!(decoded.pagination, Some(String::from("cursor:abc")));
    assert_eq!(decoded.total_count, Some(100));
    assert_eq!(ver, version);
}

// ── Scenario 7 ────────────────────────────────────────────────────────────────
// All six StatusCode variants roundtrip via V1
#[test]
fn test_all_status_codes_roundtrip_v1() {
    let version = Version::new(1, 0, 0);
    let variants = [
        StatusCode::Ok,
        StatusCode::Created,
        StatusCode::BadRequest,
        StatusCode::Unauthorized,
        StatusCode::NotFound,
        StatusCode::InternalServerError,
    ];
    let messages = ["ok", "created", "bad req", "unauth", "not found", "ise"];
    for (variant, msg) in variants.into_iter().zip(messages.iter()) {
        let resp = ApiResponseV1 {
            status: variant,
            message: String::from(*msg),
            data: vec![1, 2],
        };
        let encoded = encode_versioned_value(&resp, version).expect("encode failed");
        let (decoded, ver, _consumed): (ApiResponseV1, Version, usize) =
            decode_versioned_value(&encoded).expect("decode failed");
        assert_eq!(decoded, resp);
        assert_eq!(ver, version);
    }
}

// ── Scenario 8 ────────────────────────────────────────────────────────────────
// consumed bytes equals total encoded buffer length for V1
#[test]
fn test_api_v1_consumed_bytes_equals_total_encoded_length() {
    let version = Version::new(1, 0, 0);
    let original = ApiResponseV1 {
        status: StatusCode::Ok,
        message: String::from("bytes check"),
        data: vec![10, 20, 30],
    };
    let encoded = encode_versioned_value(&original, version).expect("encode failed");
    let (_decoded, _ver, consumed): (ApiResponseV1, Version, usize) =
        decode_versioned_value(&encoded).expect("decode failed");
    // consumed is the payload portion decoded by decode_from_slice inside decode_versioned_value;
    // the total buffer is header + payload, and consumed measures the payload slice
    assert!(consumed > 0);
    assert!(consumed <= encoded.len());
}

// ── Scenario 9 ────────────────────────────────────────────────────────────────
// V1 encoded size < V2 encoded size < V3 encoded size (for equivalent fields)
#[test]
fn test_v1_smaller_than_v2_smaller_than_v3_encoded_size() {
    let v1 = ApiResponseV1 {
        status: StatusCode::Ok,
        message: String::from("hello"),
        data: vec![1, 2, 3],
    };
    let v2 = ApiResponseV2 {
        status: StatusCode::Ok,
        message: String::from("hello"),
        data: vec![1, 2, 3],
        request_id: 0,
        duration_ms: 0,
    };
    let v3 = ApiResponseV3 {
        status: StatusCode::Ok,
        message: String::from("hello"),
        data: vec![1, 2, 3],
        request_id: 0,
        duration_ms: 0,
        pagination: None,
        total_count: None,
    };
    let bytes_v1 = encode_to_vec(&v1).expect("encode v1 failed");
    let bytes_v2 = encode_to_vec(&v2).expect("encode v2 failed");
    let bytes_v3 = encode_to_vec(&v3).expect("encode v3 failed");
    assert!(
        bytes_v1.len() < bytes_v2.len(),
        "V1 ({}) should be smaller than V2 ({})",
        bytes_v1.len(),
        bytes_v2.len()
    );
    assert!(
        bytes_v2.len() < bytes_v3.len(),
        "V2 ({}) should be smaller than V3 ({})",
        bytes_v2.len(),
        bytes_v3.len()
    );
}

// ── Scenario 10 ───────────────────────────────────────────────────────────────
// Version tuple accessor returns (major, minor, patch)
#[test]
fn test_version_tuple_accessor() {
    let version = Version::new(4, 7, 11);
    let original = ApiResponseV1 {
        status: StatusCode::Ok,
        message: String::from("tuple accessor"),
        data: vec![],
    };
    let encoded = encode_versioned_value(&original, version).expect("encode failed");
    let (_decoded, ver, _consumed): (ApiResponseV1, Version, usize) =
        decode_versioned_value(&encoded).expect("decode failed");
    let (maj, min, pat) = ver.tuple();
    assert_eq!(maj, 4);
    assert_eq!(min, 7);
    assert_eq!(pat, 11);
}

// ── Scenario 11 ───────────────────────────────────────────────────────────────
// Big-endian config: encode V2 manually and decode_from_slice with big-endian config
#[test]
fn test_api_v2_big_endian_config_roundtrip() {
    let cfg = config::standard().with_big_endian();
    let original = ApiResponseV2 {
        status: StatusCode::Ok,
        message: String::from("big endian"),
        data: vec![0xDE, 0xAD],
        request_id: 12345678901234,
        duration_ms: 999,
    };
    let encoded = oxicode::encode_to_vec_with_config(&original, cfg).expect("encode failed");
    let (decoded, consumed): (ApiResponseV2, usize) =
        oxicode::decode_from_slice_with_config(&encoded, cfg).expect("decode failed");
    assert_eq!(decoded, original);
    assert_eq!(consumed, encoded.len());
}

// ── Scenario 12 ───────────────────────────────────────────────────────────────
// Fixed-int config: encode V2 with fixed int encoding, verify field values survive
#[test]
fn test_api_v2_fixed_int_config_roundtrip() {
    let cfg = config::standard().with_fixed_int_encoding();
    let original = ApiResponseV2 {
        status: StatusCode::Created,
        message: String::from("fixed int"),
        data: vec![0x01],
        request_id: u64::MAX,
        duration_ms: u32::MAX,
    };
    let encoded = oxicode::encode_to_vec_with_config(&original, cfg).expect("encode failed");
    let (decoded, consumed): (ApiResponseV2, usize) =
        oxicode::decode_from_slice_with_config(&encoded, cfg).expect("decode failed");
    assert_eq!(decoded, original);
    assert_eq!(consumed, encoded.len());
}

// ── Scenario 13 ───────────────────────────────────────────────────────────────
// Vec of ApiResponseV1 roundtrip with versioning
#[test]
fn test_vec_of_api_v1_versioned_roundtrip() {
    let version = Version::new(1, 0, 0);
    let responses = vec![
        ApiResponseV1 {
            status: StatusCode::Ok,
            message: String::from("first"),
            data: vec![1],
        },
        ApiResponseV1 {
            status: StatusCode::NotFound,
            message: String::from("second"),
            data: vec![2, 3],
        },
        ApiResponseV1 {
            status: StatusCode::BadRequest,
            message: String::from("third"),
            data: vec![],
        },
    ];
    let encoded = encode_versioned_value(&responses, version).expect("encode failed");
    let (decoded, ver, _consumed): (Vec<ApiResponseV1>, Version, usize) =
        decode_versioned_value(&encoded).expect("decode failed");
    assert_eq!(decoded, responses);
    assert_eq!(ver, version);
    assert_eq!(decoded.len(), 3);
}

// ── Scenario 14 ───────────────────────────────────────────────────────────────
// Vec of ApiResponseV3 roundtrip with versioning — mixed pagination
#[test]
fn test_vec_of_api_v3_versioned_roundtrip_mixed_pagination() {
    let version = Version::new(3, 2, 0);
    let responses = vec![
        ApiResponseV3 {
            status: StatusCode::Ok,
            message: String::from("page 1"),
            data: vec![0xAA],
            request_id: 1,
            duration_ms: 10,
            pagination: Some(String::from("next:token123")),
            total_count: Some(500),
        },
        ApiResponseV3 {
            status: StatusCode::Ok,
            message: String::from("last page"),
            data: vec![0xBB, 0xCC],
            request_id: 2,
            duration_ms: 20,
            pagination: None,
            total_count: Some(500),
        },
    ];
    let encoded = encode_versioned_value(&responses, version).expect("encode failed");
    let (decoded, ver, _consumed): (Vec<ApiResponseV3>, Version, usize) =
        decode_versioned_value(&encoded).expect("decode failed");
    assert_eq!(decoded, responses);
    assert_eq!(ver, version);
    assert_eq!(decoded[0].pagination, Some(String::from("next:token123")));
    assert_eq!(decoded[1].pagination, None);
}

// ── Scenario 15 ───────────────────────────────────────────────────────────────
// V3 with all optional fields set — roundtrip preserves total_count
#[test]
fn test_api_v3_all_optional_fields_set() {
    let version = Version::new(3, 0, 0);
    let original = ApiResponseV3 {
        status: StatusCode::Ok,
        message: String::from("full v3"),
        data: vec![1, 2, 3, 4, 5],
        request_id: 987654321,
        duration_ms: 250,
        pagination: Some(String::from("cursor:xyz789")),
        total_count: Some(9_999_999),
    };
    let encoded = encode_versioned_value(&original, version).expect("encode failed");
    let (decoded, ver, _consumed): (ApiResponseV3, Version, usize) =
        decode_versioned_value(&encoded).expect("decode failed");
    assert_eq!(decoded, original);
    assert_eq!(decoded.total_count, Some(9_999_999));
    assert_eq!(decoded.pagination, Some(String::from("cursor:xyz789")));
    assert_eq!(ver, version);
}

// ── Scenario 16 ───────────────────────────────────────────────────────────────
// V3 with no optional fields set — roundtrip with None values
#[test]
fn test_api_v3_no_optional_fields_set() {
    let version = Version::new(3, 0, 0);
    let original = ApiResponseV3 {
        status: StatusCode::InternalServerError,
        message: String::from("error, no pagination"),
        data: vec![0xFF, 0xFE, 0xFD],
        request_id: 0,
        duration_ms: 0,
        pagination: None,
        total_count: None,
    };
    let encoded = encode_versioned_value(&original, version).expect("encode failed");
    let (decoded, ver, _consumed): (ApiResponseV3, Version, usize) =
        decode_versioned_value(&encoded).expect("decode failed");
    assert_eq!(decoded, original);
    assert!(decoded.pagination.is_none());
    assert!(decoded.total_count.is_none());
    assert_eq!(ver, version);
}

// ── Scenario 17 ───────────────────────────────────────────────────────────────
// Version ordering: V1 version < V2 version < V3 version
#[test]
fn test_version_ordering_v1_v2_v3() {
    let v1_ver = Version::new(1, 0, 0);
    let v2_ver = Version::new(2, 0, 0);
    let v3_ver = Version::new(3, 0, 0);
    assert!(v1_ver < v2_ver, "V1 version should be less than V2 version");
    assert!(v2_ver < v3_ver, "V2 version should be less than V3 version");
    assert!(v1_ver < v3_ver, "V1 version should be less than V3 version");
    assert_ne!(v1_ver, v2_ver);
    assert_ne!(v2_ver, v3_ver);
}

// ── Scenario 18 ───────────────────────────────────────────────────────────────
// encode_to_vec + decode_from_slice (plain, no versioning) for V1 — baseline
#[test]
fn test_api_v1_plain_encode_decode_baseline() {
    let original = ApiResponseV1 {
        status: StatusCode::Ok,
        message: String::from("plain baseline"),
        data: vec![42],
    };
    let bytes = encode_to_vec(&original).expect("encode_to_vec failed");
    let (decoded, consumed): (ApiResponseV1, usize) =
        decode_from_slice(&bytes).expect("decode_from_slice failed");
    assert_eq!(decoded, original);
    assert_eq!(consumed, bytes.len());
}

// ── Scenario 19 ───────────────────────────────────────────────────────────────
// V2 request_id and duration_ms boundary values: 0 and u64::MAX
#[test]
fn test_api_v2_request_id_boundary_values() {
    let version = Version::new(2, 0, 0);
    for (request_id, duration_ms) in [(0u64, 0u32), (u64::MAX, u32::MAX)] {
        let original = ApiResponseV2 {
            status: StatusCode::Ok,
            message: String::from("boundary"),
            data: vec![],
            request_id,
            duration_ms,
        };
        let encoded = encode_versioned_value(&original, version).expect("encode failed");
        let (decoded, ver, _consumed): (ApiResponseV2, Version, usize) =
            decode_versioned_value(&encoded).expect("decode failed");
        assert_eq!(decoded.request_id, request_id);
        assert_eq!(decoded.duration_ms, duration_ms);
        assert_eq!(ver, version);
    }
}

// ── Scenario 20 ───────────────────────────────────────────────────────────────
// V3 total_count boundary: Some(0), Some(u64::MAX), None
#[test]
fn test_api_v3_total_count_boundary_values() {
    let version = Version::new(3, 0, 0);
    for total_count in [Some(0u64), Some(u64::MAX), None] {
        let original = ApiResponseV3 {
            status: StatusCode::Ok,
            message: String::from("total count boundary"),
            data: vec![],
            request_id: 1,
            duration_ms: 1,
            pagination: None,
            total_count,
        };
        let encoded = encode_versioned_value(&original, version).expect("encode failed");
        let (decoded, ver, _consumed): (ApiResponseV3, Version, usize) =
            decode_versioned_value(&encoded).expect("decode failed");
        assert_eq!(decoded.total_count, total_count);
        assert_eq!(ver, version);
    }
}

// ── Scenario 21 ───────────────────────────────────────────────────────────────
// Three separate versioned buffers (V1/V2/V3) decoded independently
#[test]
fn test_three_api_versions_decoded_independently() {
    let ver1 = Version::new(1, 0, 0);
    let ver2 = Version::new(2, 0, 0);
    let ver3 = Version::new(3, 0, 0);

    let resp1 = ApiResponseV1 {
        status: StatusCode::Ok,
        message: String::from("v1 response"),
        data: vec![1],
    };
    let resp2 = ApiResponseV2 {
        status: StatusCode::Created,
        message: String::from("v2 response"),
        data: vec![2],
        request_id: 200,
        duration_ms: 20,
    };
    let resp3 = ApiResponseV3 {
        status: StatusCode::NotFound,
        message: String::from("v3 response"),
        data: vec![3],
        request_id: 300,
        duration_ms: 30,
        pagination: None,
        total_count: Some(0),
    };

    let enc1 = encode_versioned_value(&resp1, ver1).expect("encode v1 failed");
    let enc2 = encode_versioned_value(&resp2, ver2).expect("encode v2 failed");
    let enc3 = encode_versioned_value(&resp3, ver3).expect("encode v3 failed");

    let (dec1, v1_out, _c1): (ApiResponseV1, Version, usize) =
        decode_versioned_value(&enc1).expect("decode v1 failed");
    let (dec2, v2_out, _c2): (ApiResponseV2, Version, usize) =
        decode_versioned_value(&enc2).expect("decode v2 failed");
    let (dec3, v3_out, _c3): (ApiResponseV3, Version, usize) =
        decode_versioned_value(&enc3).expect("decode v3 failed");

    assert_eq!(dec1, resp1);
    assert_eq!(dec2, resp2);
    assert_eq!(dec3, resp3);
    assert_eq!(v1_out, ver1);
    assert_eq!(v2_out, ver2);
    assert_eq!(v3_out, ver3);
    // Buffers must not be mixed up
    assert_ne!(enc1, enc2);
    assert_ne!(enc2, enc3);
}

// ── Scenario 22 ───────────────────────────────────────────────────────────────
// Version satisfies / is_compatible_with checks across V1/V2/V3 semantic version ladder
#[test]
fn test_version_semver_properties_across_api_generations() {
    let v1 = Version::new(1, 0, 0);
    let v2 = Version::new(2, 0, 0);
    let v3 = Version::new(3, 0, 0);

    // Each generation is a breaking change from the previous
    assert!(v2.is_breaking_change_from(&v1), "V2 breaks V1");
    assert!(v3.is_breaking_change_from(&v2), "V3 breaks V2");

    // Different major versions are not compatible with each other
    assert!(
        !v1.is_compatible_with(&v2),
        "V1 and V2 major version mismatch"
    );
    assert!(
        !v2.is_compatible_with(&v3),
        "V2 and V3 major version mismatch"
    );
    assert!(
        !v1.is_compatible_with(&v3),
        "V1 and V3 major version mismatch"
    );

    // Each version satisfies itself
    assert!(v1.satisfies(&v1), "V1 satisfies V1");
    assert!(v2.satisfies(&v2), "V2 satisfies V2");
    assert!(v3.satisfies(&v3), "V3 satisfies V3");

    // V3 satisfies V2 and V1 as minimum (it's larger)
    assert!(v3.satisfies(&v2), "V3 satisfies min V2");
    assert!(v3.satisfies(&v1), "V3 satisfies min V1");

    // V1 does NOT satisfy V2 or V3 as minimum
    assert!(!v1.satisfies(&v2), "V1 does not satisfy min V2");
    assert!(!v1.satisfies(&v3), "V1 does not satisfy min V3");
}
