#![cfg(feature = "serde")]
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
use oxicode::config;
use oxicode::serde::{decode_owned_from_slice, encode_to_vec};

// ---------------------------------------------------------------------------
// Domain types: API configuration / REST endpoint definitions
// ---------------------------------------------------------------------------

#[derive(Debug, PartialEq, serde::Serialize, serde::Deserialize)]
enum HttpMethod {
    Get,
    Post,
    Put,
    Delete,
    Patch,
    Options,
    Head,
}

#[derive(Debug, PartialEq, serde::Serialize, serde::Deserialize)]
enum AuthType {
    None,
    ApiKey(String),
    Bearer(String),
    Basic { username: String, password: String },
}

#[derive(Debug, PartialEq, serde::Serialize, serde::Deserialize)]
struct Endpoint {
    path: String,
    method: HttpMethod,
    auth: AuthType,
    timeout_ms: u32,
    retry_count: u8,
}

#[derive(Debug, PartialEq, serde::Serialize, serde::Deserialize)]
struct ApiConfig {
    base_url: String,
    endpoints: Vec<Endpoint>,
    global_timeout_ms: u32,
    max_connections: u16,
}

// ---------------------------------------------------------------------------
// Test 1: HttpMethod::Get roundtrip
// ---------------------------------------------------------------------------
#[test]
fn test_http_method_get_roundtrip() {
    let cfg = config::standard();
    let value = HttpMethod::Get;
    let bytes = encode_to_vec(&value, cfg).expect("encode HttpMethod::Get");
    let (decoded, _): (HttpMethod, usize) =
        decode_owned_from_slice(&bytes, cfg).expect("decode HttpMethod::Get");
    assert_eq!(value, decoded);
}

// ---------------------------------------------------------------------------
// Test 2: HttpMethod::Post roundtrip
// ---------------------------------------------------------------------------
#[test]
fn test_http_method_post_roundtrip() {
    let cfg = config::standard();
    let value = HttpMethod::Post;
    let bytes = encode_to_vec(&value, cfg).expect("encode HttpMethod::Post");
    let (decoded, _): (HttpMethod, usize) =
        decode_owned_from_slice(&bytes, cfg).expect("decode HttpMethod::Post");
    assert_eq!(value, decoded);
}

// ---------------------------------------------------------------------------
// Test 3: All HttpMethod variants roundtrip
// ---------------------------------------------------------------------------
#[test]
fn test_http_method_all_variants_roundtrip() {
    let cfg = config::standard();
    let variants = [
        HttpMethod::Get,
        HttpMethod::Post,
        HttpMethod::Put,
        HttpMethod::Delete,
        HttpMethod::Patch,
        HttpMethod::Options,
        HttpMethod::Head,
    ];
    for method in variants {
        let bytes = encode_to_vec(&method, cfg).expect("encode HttpMethod variant");
        let (decoded, consumed): (HttpMethod, usize) =
            decode_owned_from_slice(&bytes, cfg).expect("decode HttpMethod variant");
        assert_eq!(method, decoded);
        assert_eq!(consumed, bytes.len());
    }
}

// ---------------------------------------------------------------------------
// Test 4: AuthType::None roundtrip
// ---------------------------------------------------------------------------
#[test]
fn test_auth_type_none_roundtrip() {
    let cfg = config::standard();
    let value = AuthType::None;
    let bytes = encode_to_vec(&value, cfg).expect("encode AuthType::None");
    let (decoded, consumed): (AuthType, usize) =
        decode_owned_from_slice(&bytes, cfg).expect("decode AuthType::None");
    assert_eq!(value, decoded);
    assert_eq!(consumed, bytes.len());
}

// ---------------------------------------------------------------------------
// Test 5: AuthType::ApiKey roundtrip
// ---------------------------------------------------------------------------
#[test]
fn test_auth_type_api_key_roundtrip() {
    let cfg = config::standard();
    let value = AuthType::ApiKey("sk-live-abc123XYZ".to_string());
    let bytes = encode_to_vec(&value, cfg).expect("encode AuthType::ApiKey");
    let (decoded, consumed): (AuthType, usize) =
        decode_owned_from_slice(&bytes, cfg).expect("decode AuthType::ApiKey");
    assert_eq!(value, decoded);
    assert_eq!(consumed, bytes.len());
}

// ---------------------------------------------------------------------------
// Test 6: AuthType::Bearer roundtrip
// ---------------------------------------------------------------------------
#[test]
fn test_auth_type_bearer_roundtrip() {
    let cfg = config::standard();
    let value = AuthType::Bearer("eyJhbGciOiJSUzI1NiJ9.payload.sig".to_string());
    let bytes = encode_to_vec(&value, cfg).expect("encode AuthType::Bearer");
    let (decoded, consumed): (AuthType, usize) =
        decode_owned_from_slice(&bytes, cfg).expect("decode AuthType::Bearer");
    assert_eq!(value, decoded);
    assert_eq!(consumed, bytes.len());
}

// ---------------------------------------------------------------------------
// Test 7: AuthType::Basic (struct variant) roundtrip
// ---------------------------------------------------------------------------
#[test]
fn test_auth_type_basic_roundtrip() {
    let cfg = config::standard();
    let value = AuthType::Basic {
        username: "admin".to_string(),
        password: "s3cr3t!".to_string(),
    };
    let bytes = encode_to_vec(&value, cfg).expect("encode AuthType::Basic");
    let (decoded, consumed): (AuthType, usize) =
        decode_owned_from_slice(&bytes, cfg).expect("decode AuthType::Basic");
    assert_eq!(value, decoded);
    assert_eq!(consumed, bytes.len());
}

// ---------------------------------------------------------------------------
// Test 8: Endpoint basic roundtrip (GET, no auth)
// ---------------------------------------------------------------------------
#[test]
fn test_endpoint_basic_get_no_auth_roundtrip() {
    let cfg = config::standard();
    let value = Endpoint {
        path: "/api/v1/health".to_string(),
        method: HttpMethod::Get,
        auth: AuthType::None,
        timeout_ms: 5_000,
        retry_count: 3,
    };
    let bytes = encode_to_vec(&value, cfg).expect("encode Endpoint basic GET");
    let (decoded, consumed): (Endpoint, usize) =
        decode_owned_from_slice(&bytes, cfg).expect("decode Endpoint basic GET");
    assert_eq!(value, decoded);
    assert_eq!(consumed, bytes.len());
}

// ---------------------------------------------------------------------------
// Test 9: Endpoint with Bearer auth and POST method roundtrip
// ---------------------------------------------------------------------------
#[test]
fn test_endpoint_post_bearer_roundtrip() {
    let cfg = config::standard();
    let value = Endpoint {
        path: "/api/v2/users".to_string(),
        method: HttpMethod::Post,
        auth: AuthType::Bearer("token-for-users-endpoint".to_string()),
        timeout_ms: 15_000,
        retry_count: 1,
    };
    let bytes = encode_to_vec(&value, cfg).expect("encode Endpoint POST Bearer");
    let (decoded, _): (Endpoint, usize) =
        decode_owned_from_slice(&bytes, cfg).expect("decode Endpoint POST Bearer");
    assert_eq!(value, decoded);
}

// ---------------------------------------------------------------------------
// Test 10: Endpoint with Basic auth and DELETE method roundtrip
// ---------------------------------------------------------------------------
#[test]
fn test_endpoint_delete_basic_auth_roundtrip() {
    let cfg = config::standard();
    let value = Endpoint {
        path: "/api/v1/resources/42".to_string(),
        method: HttpMethod::Delete,
        auth: AuthType::Basic {
            username: "service_account".to_string(),
            password: "p@$$w0rd!".to_string(),
        },
        timeout_ms: 10_000,
        retry_count: 0,
    };
    let bytes = encode_to_vec(&value, cfg).expect("encode Endpoint DELETE Basic");
    let (decoded, _): (Endpoint, usize) =
        decode_owned_from_slice(&bytes, cfg).expect("decode Endpoint DELETE Basic");
    assert_eq!(value, decoded);
}

// ---------------------------------------------------------------------------
// Test 11: ApiConfig with empty endpoints roundtrip
// ---------------------------------------------------------------------------
#[test]
fn test_api_config_empty_endpoints_roundtrip() {
    let cfg = config::standard();
    let value = ApiConfig {
        base_url: "https://api.example.com".to_string(),
        endpoints: vec![],
        global_timeout_ms: 30_000,
        max_connections: 100,
    };
    let bytes = encode_to_vec(&value, cfg).expect("encode ApiConfig empty endpoints");
    let (decoded, consumed): (ApiConfig, usize) =
        decode_owned_from_slice(&bytes, cfg).expect("decode ApiConfig empty endpoints");
    assert_eq!(value, decoded);
    assert_eq!(consumed, bytes.len());
}

// ---------------------------------------------------------------------------
// Test 12: ApiConfig full roundtrip with multiple diverse endpoints
// ---------------------------------------------------------------------------
#[test]
fn test_api_config_full_roundtrip() {
    let cfg = config::standard();
    let value = ApiConfig {
        base_url: "https://gateway.prod.internal".to_string(),
        endpoints: vec![
            Endpoint {
                path: "/health".to_string(),
                method: HttpMethod::Get,
                auth: AuthType::None,
                timeout_ms: 2_000,
                retry_count: 0,
            },
            Endpoint {
                path: "/auth/token".to_string(),
                method: HttpMethod::Post,
                auth: AuthType::Basic {
                    username: "oauth_client".to_string(),
                    password: "client_secret_xyz".to_string(),
                },
                timeout_ms: 5_000,
                retry_count: 2,
            },
            Endpoint {
                path: "/data/ingest".to_string(),
                method: HttpMethod::Put,
                auth: AuthType::Bearer("ingest-bearer-token".to_string()),
                timeout_ms: 60_000,
                retry_count: 5,
            },
            Endpoint {
                path: "/config/reload".to_string(),
                method: HttpMethod::Patch,
                auth: AuthType::ApiKey("admin-api-key-0xDEAD".to_string()),
                timeout_ms: 10_000,
                retry_count: 1,
            },
        ],
        global_timeout_ms: 120_000,
        max_connections: 512,
    };
    let bytes = encode_to_vec(&value, cfg).expect("encode ApiConfig full");
    let (decoded, consumed): (ApiConfig, usize) =
        decode_owned_from_slice(&bytes, cfg).expect("decode ApiConfig full");
    assert_eq!(value, decoded);
    assert_eq!(consumed, bytes.len());
}

// ---------------------------------------------------------------------------
// Test 13: Vec<Endpoint> roundtrip
// ---------------------------------------------------------------------------
#[test]
fn test_vec_endpoint_roundtrip() {
    let cfg = config::standard();
    let value: Vec<Endpoint> = vec![
        Endpoint {
            path: "/metrics".to_string(),
            method: HttpMethod::Get,
            auth: AuthType::ApiKey("metrics-key-abc".to_string()),
            timeout_ms: 3_000,
            retry_count: 2,
        },
        Endpoint {
            path: "/events".to_string(),
            method: HttpMethod::Post,
            auth: AuthType::Bearer("events-jwt".to_string()),
            timeout_ms: 8_000,
            retry_count: 3,
        },
        Endpoint {
            path: "/status".to_string(),
            method: HttpMethod::Head,
            auth: AuthType::None,
            timeout_ms: 1_000,
            retry_count: 0,
        },
    ];
    let bytes = encode_to_vec(&value, cfg).expect("encode Vec<Endpoint>");
    let (decoded, consumed): (Vec<Endpoint>, usize) =
        decode_owned_from_slice(&bytes, cfg).expect("decode Vec<Endpoint>");
    assert_eq!(value, decoded);
    assert_eq!(consumed, bytes.len());
}

// ---------------------------------------------------------------------------
// Test 14: Vec<ApiConfig> roundtrip
// ---------------------------------------------------------------------------
#[test]
fn test_vec_api_config_roundtrip() {
    let cfg = config::standard();
    let make_config = |base: &str, max: u16| ApiConfig {
        base_url: base.to_string(),
        endpoints: vec![Endpoint {
            path: "/ping".to_string(),
            method: HttpMethod::Get,
            auth: AuthType::None,
            timeout_ms: 1_000,
            retry_count: 0,
        }],
        global_timeout_ms: 30_000,
        max_connections: max,
    };
    let value: Vec<ApiConfig> = vec![
        make_config("https://us-east.api.example.com", 256),
        make_config("https://eu-west.api.example.com", 128),
        make_config("https://ap-south.api.example.com", 64),
    ];
    let bytes = encode_to_vec(&value, cfg).expect("encode Vec<ApiConfig>");
    let (decoded, consumed): (Vec<ApiConfig>, usize) =
        decode_owned_from_slice(&bytes, cfg).expect("decode Vec<ApiConfig>");
    assert_eq!(value, decoded);
    assert_eq!(consumed, bytes.len());
}

// ---------------------------------------------------------------------------
// Test 15: Consumed bytes equals encoded length for ApiConfig
// ---------------------------------------------------------------------------
#[test]
fn test_api_config_consumed_bytes_equals_encoded_len() {
    let cfg = config::standard();
    let value = ApiConfig {
        base_url: "https://probe.service.local".to_string(),
        endpoints: vec![Endpoint {
            path: "/api/v3/check".to_string(),
            method: HttpMethod::Options,
            auth: AuthType::ApiKey("probe-key-007".to_string()),
            timeout_ms: 4_000,
            retry_count: 1,
        }],
        global_timeout_ms: 20_000,
        max_connections: 32,
    };
    let bytes = encode_to_vec(&value, cfg).expect("encode ApiConfig for size check");
    let (_decoded, consumed): (ApiConfig, usize) =
        decode_owned_from_slice(&bytes, cfg).expect("decode ApiConfig for size check");
    assert_eq!(consumed, bytes.len());
}

// ---------------------------------------------------------------------------
// Test 16: Encode determinism — same ApiConfig produces identical bytes
// ---------------------------------------------------------------------------
#[test]
fn test_api_config_encode_determinism() {
    let cfg = config::standard();
    let value = ApiConfig {
        base_url: "https://deterministic.api.test".to_string(),
        endpoints: vec![Endpoint {
            path: "/idempotent".to_string(),
            method: HttpMethod::Put,
            auth: AuthType::Bearer("stable-token".to_string()),
            timeout_ms: 7_500,
            retry_count: 4,
        }],
        global_timeout_ms: 45_000,
        max_connections: 200,
    };
    let bytes_a = encode_to_vec(&value, cfg).expect("encode ApiConfig determinism A");
    let bytes_b = encode_to_vec(&value, cfg).expect("encode ApiConfig determinism B");
    assert_eq!(bytes_a, bytes_b, "encoding must be deterministic");
}

// ---------------------------------------------------------------------------
// Test 17: Big-endian config ApiConfig roundtrip
// ---------------------------------------------------------------------------
#[test]
fn test_api_config_big_endian_roundtrip() {
    let cfg = config::standard().with_big_endian();
    let value = ApiConfig {
        base_url: "https://big-endian.api.example.org".to_string(),
        endpoints: vec![
            Endpoint {
                path: "/be/v1/resource".to_string(),
                method: HttpMethod::Get,
                auth: AuthType::ApiKey("be-key-9876".to_string()),
                timeout_ms: 6_000,
                retry_count: 2,
            },
            Endpoint {
                path: "/be/v1/submit".to_string(),
                method: HttpMethod::Post,
                auth: AuthType::None,
                timeout_ms: 12_000,
                retry_count: 3,
            },
        ],
        global_timeout_ms: 90_000,
        max_connections: 1024,
    };
    let bytes = encode_to_vec(&value, cfg).expect("encode ApiConfig big-endian");
    let (decoded, consumed): (ApiConfig, usize) =
        decode_owned_from_slice(&bytes, cfg).expect("decode ApiConfig big-endian");
    assert_eq!(value, decoded);
    assert_eq!(consumed, bytes.len());
}

// ---------------------------------------------------------------------------
// Test 18: Fixed-int encoding config Endpoint roundtrip
// ---------------------------------------------------------------------------
#[test]
fn test_endpoint_fixed_int_encoding_roundtrip() {
    let cfg = config::standard().with_fixed_int_encoding();
    let value = Endpoint {
        path: "/fixed-int/v2/items".to_string(),
        method: HttpMethod::Patch,
        auth: AuthType::Basic {
            username: "svc_user".to_string(),
            password: "fixed_int_pass".to_string(),
        },
        timeout_ms: u32::MAX,
        retry_count: u8::MAX,
    };
    let bytes = encode_to_vec(&value, cfg).expect("encode Endpoint fixed-int");
    let (decoded, consumed): (Endpoint, usize) =
        decode_owned_from_slice(&bytes, cfg).expect("decode Endpoint fixed-int");
    assert_eq!(value, decoded);
    assert_eq!(consumed, bytes.len());
}

// ---------------------------------------------------------------------------
// Test 19: Big-endian + fixed-int combined config Endpoint roundtrip
// ---------------------------------------------------------------------------
#[test]
fn test_endpoint_big_endian_fixed_int_combined_roundtrip() {
    let cfg = config::standard()
        .with_big_endian()
        .with_fixed_int_encoding();
    let value = Endpoint {
        path: "/combined/endpoint".to_string(),
        method: HttpMethod::Delete,
        auth: AuthType::Bearer("combined-config-bearer".to_string()),
        timeout_ms: 25_000,
        retry_count: 7,
    };
    let bytes = encode_to_vec(&value, cfg).expect("encode Endpoint big-endian+fixed-int");
    let (decoded, consumed): (Endpoint, usize) =
        decode_owned_from_slice(&bytes, cfg).expect("decode Endpoint big-endian+fixed-int");
    assert_eq!(value, decoded);
    assert_eq!(consumed, bytes.len());
}

// ---------------------------------------------------------------------------
// Test 20: ApiConfig with unicode base_url and paths roundtrip
// ---------------------------------------------------------------------------
#[test]
fn test_api_config_unicode_fields_roundtrip() {
    let cfg = config::standard();
    let value = ApiConfig {
        base_url: "https://例え.テスト/api".to_string(),
        endpoints: vec![Endpoint {
            path: "/リソース/一覧".to_string(),
            method: HttpMethod::Get,
            auth: AuthType::ApiKey("キー-🔑-abc".to_string()),
            timeout_ms: 5_000,
            retry_count: 1,
        }],
        global_timeout_ms: 60_000,
        max_connections: 50,
    };
    let bytes = encode_to_vec(&value, cfg).expect("encode ApiConfig unicode");
    let (decoded, consumed): (ApiConfig, usize) =
        decode_owned_from_slice(&bytes, cfg).expect("decode ApiConfig unicode");
    assert_eq!(value, decoded);
    assert_eq!(consumed, bytes.len());
}

// ---------------------------------------------------------------------------
// Test 21: ApiConfig with u16::MAX max_connections and u32::MAX global timeout
// ---------------------------------------------------------------------------
#[test]
fn test_api_config_boundary_values_roundtrip() {
    let cfg = config::standard();
    let value = ApiConfig {
        base_url: "https://boundary.test.internal".to_string(),
        endpoints: vec![
            Endpoint {
                path: "/max-timeout".to_string(),
                method: HttpMethod::Post,
                auth: AuthType::None,
                timeout_ms: u32::MAX,
                retry_count: u8::MAX,
            },
            Endpoint {
                path: "/zero-timeout".to_string(),
                method: HttpMethod::Get,
                auth: AuthType::ApiKey("zero-key".to_string()),
                timeout_ms: 0,
                retry_count: 0,
            },
        ],
        global_timeout_ms: u32::MAX,
        max_connections: u16::MAX,
    };
    let bytes = encode_to_vec(&value, cfg).expect("encode ApiConfig boundary values");
    let (decoded, consumed): (ApiConfig, usize) =
        decode_owned_from_slice(&bytes, cfg).expect("decode ApiConfig boundary values");
    assert_eq!(value, decoded);
    assert_eq!(consumed, bytes.len());
}

// ---------------------------------------------------------------------------
// Test 22: Large ApiConfig with 10 diverse endpoints roundtrip
// ---------------------------------------------------------------------------
#[test]
fn test_api_config_large_ten_endpoints_roundtrip() {
    let cfg = config::standard();
    let endpoints: Vec<Endpoint> = (0..10)
        .map(|i| {
            let method = match i % 7 {
                0 => HttpMethod::Get,
                1 => HttpMethod::Post,
                2 => HttpMethod::Put,
                3 => HttpMethod::Delete,
                4 => HttpMethod::Patch,
                5 => HttpMethod::Options,
                _ => HttpMethod::Head,
            };
            let auth = match i % 4 {
                0 => AuthType::None,
                1 => AuthType::ApiKey(format!("api-key-endpoint-{i:02}")),
                2 => AuthType::Bearer(format!("bearer-token-ep-{i:02}")),
                _ => AuthType::Basic {
                    username: format!("user_{i:02}"),
                    password: format!("pass_{i:02}!"),
                },
            };
            Endpoint {
                path: format!("/api/v1/endpoint/{i:02}"),
                method,
                auth,
                timeout_ms: 1_000 * (i as u32 + 1),
                retry_count: i as u8,
            }
        })
        .collect();
    let value = ApiConfig {
        base_url: "https://multi-endpoint.example.com".to_string(),
        endpoints,
        global_timeout_ms: 300_000,
        max_connections: 2048,
    };
    let bytes = encode_to_vec(&value, cfg).expect("encode ApiConfig 10 endpoints");
    let (decoded, consumed): (ApiConfig, usize) =
        decode_owned_from_slice(&bytes, cfg).expect("decode ApiConfig 10 endpoints");
    assert_eq!(value, decoded);
    assert_eq!(consumed, bytes.len());
}
