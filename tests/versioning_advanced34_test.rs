#![cfg(feature = "versioning")]
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
use oxicode::versioning::Version;
use oxicode::{decode_from_slice, encode_to_vec, Decode, Encode};
use oxicode::{decode_versioned_value, encode_versioned_value};

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
enum SpanStatus {
    Ok,
    Error,
    Unset,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
enum ServiceTier {
    Frontend,
    Backend,
    Database,
    Cache,
    Queue,
    External,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
enum DeploymentEnv {
    Development,
    Staging,
    Production,
    Canary,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
enum HealthStatus {
    Healthy,
    Degraded,
    Unhealthy,
    Unknown,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct TraceSpan {
    span_id: u64,
    trace_id: u128,
    parent_span_id: Option<u64>,
    service_name: String,
    operation: String,
    start_us: u64,
    duration_us: u64,
    status: SpanStatus,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct ServiceInstance {
    instance_id: u64,
    service_name: String,
    tier: ServiceTier,
    env: DeploymentEnv,
    version: String,
    host: String,
    port: u16,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct HealthCheck {
    instance_id: u64,
    timestamp: u64,
    status: HealthStatus,
    latency_ms: u32,
    checks_passed: u16,
    checks_failed: u16,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct CircuitBreaker {
    service_id: u64,
    state: String,
    failure_count: u32,
    success_count: u32,
    last_state_change: u64,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct ApiGatewayLog {
    request_id: u64,
    service_id: u64,
    method: String,
    path: String,
    status_code: u16,
    response_ms: u32,
    timestamp: u64,
}

#[test]
fn test_trace_span_v1_roundtrip() {
    let span = TraceSpan {
        span_id: 0xDEADBEEF_CAFEBABE,
        trace_id: 0x0102030405060708_090A0B0C0D0E0F10u128,
        parent_span_id: Some(0x1122334455667788),
        service_name: "order-service".to_string(),
        operation: "process_order".to_string(),
        start_us: 1_700_000_000_000_000,
        duration_us: 3_450,
        status: SpanStatus::Ok,
    };
    let ver = Version::new(1, 0, 0);
    let encoded = encode_versioned_value(&span, ver).expect("encode_versioned_value failed");
    let (decoded, decoded_ver, _consumed) =
        decode_versioned_value::<TraceSpan>(&encoded).expect("decode_versioned_value failed");
    assert_eq!(decoded, span);
    assert_eq!(decoded_ver.major, 1);
    assert_eq!(decoded_ver.minor, 0);
    assert_eq!(decoded_ver.patch, 0);
}

#[test]
fn test_trace_span_no_parent_v1() {
    let span = TraceSpan {
        span_id: 0xAAAABBBBCCCCDDDD,
        trace_id: 0xFEDCBA9876543210_FEDCBA9876543210u128,
        parent_span_id: None,
        service_name: "gateway".to_string(),
        operation: "route_request".to_string(),
        start_us: 1_710_000_000_000_000,
        duration_us: 500,
        status: SpanStatus::Unset,
    };
    let ver = Version::new(1, 0, 0);
    let encoded = encode_versioned_value(&span, ver).expect("encode failed");
    let (decoded, decoded_ver, _) =
        decode_versioned_value::<TraceSpan>(&encoded).expect("decode failed");
    assert_eq!(decoded.parent_span_id, None);
    assert_eq!(decoded.status, SpanStatus::Unset);
    assert_eq!(decoded_ver.major, 1);
    assert_eq!(decoded_ver.minor, 0);
}

#[test]
fn test_trace_span_error_status_v2() {
    let span = TraceSpan {
        span_id: 0x1234567890ABCDEF,
        trace_id: 0x11112222333344445555666677778888u128,
        parent_span_id: Some(0xFEEDFACEDEAD0001),
        service_name: "payment-service".to_string(),
        operation: "charge_card".to_string(),
        start_us: 1_720_000_000_000_000,
        duration_us: 12_000,
        status: SpanStatus::Error,
    };
    let ver = Version::new(2, 0, 0);
    let encoded = encode_versioned_value(&span, ver).expect("encode failed");
    let (decoded, decoded_ver, _) =
        decode_versioned_value::<TraceSpan>(&encoded).expect("decode failed");
    assert_eq!(decoded, span);
    assert_eq!(decoded_ver.major, 2);
    assert_eq!(decoded_ver.minor, 0);
    assert_eq!(decoded_ver.patch, 0);
}

#[test]
fn test_service_instance_frontend_v1() {
    let instance = ServiceInstance {
        instance_id: 1001,
        service_name: "web-app".to_string(),
        tier: ServiceTier::Frontend,
        env: DeploymentEnv::Production,
        version: "3.4.1".to_string(),
        host: "10.0.1.50".to_string(),
        port: 8080,
    };
    let ver = Version::new(1, 0, 0);
    let encoded = encode_versioned_value(&instance, ver).expect("encode failed");
    let (decoded, decoded_ver, _) =
        decode_versioned_value::<ServiceInstance>(&encoded).expect("decode failed");
    assert_eq!(decoded, instance);
    assert_eq!(decoded_ver.major, 1);
    assert_eq!(decoded.port, 8080);
    assert_eq!(decoded.tier, ServiceTier::Frontend);
}

#[test]
fn test_service_instance_database_staging_v2() {
    let instance = ServiceInstance {
        instance_id: 2002,
        service_name: "postgres-primary".to_string(),
        tier: ServiceTier::Database,
        env: DeploymentEnv::Staging,
        version: "14.5.0".to_string(),
        host: "db-staging.internal".to_string(),
        port: 5432,
    };
    let ver = Version::new(2, 0, 0);
    let encoded = encode_versioned_value(&instance, ver).expect("encode failed");
    let (decoded, decoded_ver, _) =
        decode_versioned_value::<ServiceInstance>(&encoded).expect("decode failed");
    assert_eq!(decoded, instance);
    assert_eq!(decoded_ver.major, 2);
    assert_eq!(decoded.env, DeploymentEnv::Staging);
    assert_eq!(decoded.tier, ServiceTier::Database);
}

#[test]
fn test_service_instance_cache_canary_v1_2_0() {
    let instance = ServiceInstance {
        instance_id: 3003,
        service_name: "redis-cache".to_string(),
        tier: ServiceTier::Cache,
        env: DeploymentEnv::Canary,
        version: "7.0.11".to_string(),
        host: "cache-canary.internal".to_string(),
        port: 6379,
    };
    let ver = Version::new(1, 2, 0);
    let encoded = encode_versioned_value(&instance, ver).expect("encode failed");
    let (decoded, decoded_ver, _) =
        decode_versioned_value::<ServiceInstance>(&encoded).expect("decode failed");
    assert_eq!(decoded, instance);
    assert_eq!(decoded_ver.major, 1);
    assert_eq!(decoded_ver.minor, 2);
    assert_eq!(decoded_ver.patch, 0);
    assert_eq!(decoded.tier, ServiceTier::Cache);
    assert_eq!(decoded.env, DeploymentEnv::Canary);
}

#[test]
fn test_health_check_healthy_v1() {
    let check = HealthCheck {
        instance_id: 4004,
        timestamp: 1_700_500_000_000_000,
        status: HealthStatus::Healthy,
        latency_ms: 2,
        checks_passed: 10,
        checks_failed: 0,
    };
    let ver = Version::new(1, 0, 0);
    let encoded = encode_versioned_value(&check, ver).expect("encode failed");
    let (decoded, decoded_ver, _) =
        decode_versioned_value::<HealthCheck>(&encoded).expect("decode failed");
    assert_eq!(decoded, check);
    assert_eq!(decoded_ver.major, 1);
    assert_eq!(decoded.status, HealthStatus::Healthy);
    assert_eq!(decoded.checks_failed, 0);
}

#[test]
fn test_health_check_degraded_v2() {
    let check = HealthCheck {
        instance_id: 5005,
        timestamp: 1_711_000_000_000_000,
        status: HealthStatus::Degraded,
        latency_ms: 450,
        checks_passed: 7,
        checks_failed: 3,
    };
    let ver = Version::new(2, 0, 0);
    let encoded = encode_versioned_value(&check, ver).expect("encode failed");
    let (decoded, decoded_ver, _) =
        decode_versioned_value::<HealthCheck>(&encoded).expect("decode failed");
    assert_eq!(decoded, check);
    assert_eq!(decoded_ver.major, 2);
    assert_eq!(decoded_ver.minor, 0);
    assert_eq!(decoded.status, HealthStatus::Degraded);
    assert_eq!(decoded.checks_failed, 3);
}

#[test]
fn test_health_check_unhealthy_v1_2_0() {
    let check = HealthCheck {
        instance_id: 6006,
        timestamp: 1_722_000_000_000_000,
        status: HealthStatus::Unhealthy,
        latency_ms: 9999,
        checks_passed: 0,
        checks_failed: 10,
    };
    let ver = Version::new(1, 2, 0);
    let encoded = encode_versioned_value(&check, ver).expect("encode failed");
    let (decoded, decoded_ver, _) =
        decode_versioned_value::<HealthCheck>(&encoded).expect("decode failed");
    assert_eq!(decoded, check);
    assert_eq!(decoded_ver.major, 1);
    assert_eq!(decoded_ver.minor, 2);
    assert_eq!(decoded_ver.patch, 0);
    assert_eq!(decoded.status, HealthStatus::Unhealthy);
    assert_eq!(decoded.checks_passed, 0);
}

#[test]
fn test_circuit_breaker_open_state_v1() {
    let cb = CircuitBreaker {
        service_id: 7007,
        state: "OPEN".to_string(),
        failure_count: 50,
        success_count: 0,
        last_state_change: 1_700_600_000_000_000,
    };
    let ver = Version::new(1, 0, 0);
    let encoded = encode_versioned_value(&cb, ver).expect("encode failed");
    let (decoded, decoded_ver, _) =
        decode_versioned_value::<CircuitBreaker>(&encoded).expect("decode failed");
    assert_eq!(decoded, cb);
    assert_eq!(decoded_ver.major, 1);
    assert_eq!(decoded.state, "OPEN");
    assert_eq!(decoded.failure_count, 50);
}

#[test]
fn test_circuit_breaker_half_open_v2() {
    let cb = CircuitBreaker {
        service_id: 8008,
        state: "HALF_OPEN".to_string(),
        failure_count: 5,
        success_count: 2,
        last_state_change: 1_712_000_000_000_000,
    };
    let ver = Version::new(2, 0, 0);
    let encoded = encode_versioned_value(&cb, ver).expect("encode failed");
    let (decoded, decoded_ver, _) =
        decode_versioned_value::<CircuitBreaker>(&encoded).expect("decode failed");
    assert_eq!(decoded, cb);
    assert_eq!(decoded_ver.major, 2);
    assert_eq!(decoded.state, "HALF_OPEN");
    assert_eq!(decoded.success_count, 2);
}

#[test]
fn test_circuit_breaker_closed_v1_2_0() {
    let cb = CircuitBreaker {
        service_id: 9009,
        state: "CLOSED".to_string(),
        failure_count: 0,
        success_count: 10_000,
        last_state_change: 1_723_000_000_000_000,
    };
    let ver = Version::new(1, 2, 0);
    let encoded = encode_versioned_value(&cb, ver).expect("encode failed");
    let (decoded, decoded_ver, _) =
        decode_versioned_value::<CircuitBreaker>(&encoded).expect("decode failed");
    assert_eq!(decoded, cb);
    assert_eq!(decoded_ver.major, 1);
    assert_eq!(decoded_ver.minor, 2);
    assert_eq!(decoded_ver.patch, 0);
    assert_eq!(decoded.state, "CLOSED");
    assert_eq!(decoded.success_count, 10_000);
}

#[test]
fn test_api_gateway_log_get_200_v1() {
    let log = ApiGatewayLog {
        request_id: 100_001,
        service_id: 200_001,
        method: "GET".to_string(),
        path: "/api/v1/orders".to_string(),
        status_code: 200,
        response_ms: 45,
        timestamp: 1_700_700_000_000_000,
    };
    let ver = Version::new(1, 0, 0);
    let encoded = encode_versioned_value(&log, ver).expect("encode failed");
    let (decoded, decoded_ver, _) =
        decode_versioned_value::<ApiGatewayLog>(&encoded).expect("decode failed");
    assert_eq!(decoded, log);
    assert_eq!(decoded_ver.major, 1);
    assert_eq!(decoded.status_code, 200);
    assert_eq!(decoded.method, "GET");
}

#[test]
fn test_api_gateway_log_post_500_v2() {
    let log = ApiGatewayLog {
        request_id: 100_002,
        service_id: 200_002,
        method: "POST".to_string(),
        path: "/api/v2/payments".to_string(),
        status_code: 500,
        response_ms: 3_000,
        timestamp: 1_713_000_000_000_000,
    };
    let ver = Version::new(2, 0, 0);
    let encoded = encode_versioned_value(&log, ver).expect("encode failed");
    let (decoded, decoded_ver, _) =
        decode_versioned_value::<ApiGatewayLog>(&encoded).expect("decode failed");
    assert_eq!(decoded, log);
    assert_eq!(decoded_ver.major, 2);
    assert_eq!(decoded.status_code, 500);
    assert_eq!(decoded.response_ms, 3_000);
}

#[test]
fn test_api_gateway_log_delete_404_v1_2_0() {
    let log = ApiGatewayLog {
        request_id: 100_003,
        service_id: 200_003,
        method: "DELETE".to_string(),
        path: "/api/v1/users/42".to_string(),
        status_code: 404,
        response_ms: 12,
        timestamp: 1_724_000_000_000_000,
    };
    let ver = Version::new(1, 2, 0);
    let encoded = encode_versioned_value(&log, ver).expect("encode failed");
    let (decoded, decoded_ver, _) =
        decode_versioned_value::<ApiGatewayLog>(&encoded).expect("decode failed");
    assert_eq!(decoded, log);
    assert_eq!(decoded_ver.major, 1);
    assert_eq!(decoded_ver.minor, 2);
    assert_eq!(decoded_ver.patch, 0);
    assert_eq!(decoded.status_code, 404);
    assert_eq!(decoded.path, "/api/v1/users/42");
}

#[test]
fn test_vec_trace_spans_versioned_v1() {
    let spans = vec![
        TraceSpan {
            span_id: 0x0001,
            trace_id: 0xAAAAAAAABBBBBBBB_CCCCCCCCDDDDDDDDu128,
            parent_span_id: None,
            service_name: "svc-a".to_string(),
            operation: "op_root".to_string(),
            start_us: 1_700_800_000_000_000,
            duration_us: 1_000,
            status: SpanStatus::Ok,
        },
        TraceSpan {
            span_id: 0x0002,
            trace_id: 0xAAAAAAAABBBBBBBB_CCCCCCCCDDDDDDDDu128,
            parent_span_id: Some(0x0001),
            service_name: "svc-b".to_string(),
            operation: "op_child".to_string(),
            start_us: 1_700_800_000_000_100,
            duration_us: 500,
            status: SpanStatus::Ok,
        },
    ];
    let ver = Version::new(1, 0, 0);
    let encoded = encode_versioned_value(&spans, ver).expect("encode failed");
    let (decoded, decoded_ver, _) =
        decode_versioned_value::<Vec<TraceSpan>>(&encoded).expect("decode failed");
    assert_eq!(decoded, spans);
    assert_eq!(decoded_ver.major, 1);
    assert_eq!(decoded.len(), 2);
    assert_eq!(decoded[0].parent_span_id, None);
    assert_eq!(decoded[1].parent_span_id, Some(0x0001));
}

#[test]
fn test_vec_service_instances_v2() {
    let instances = vec![
        ServiceInstance {
            instance_id: 11,
            service_name: "alpha".to_string(),
            tier: ServiceTier::Backend,
            env: DeploymentEnv::Production,
            version: "1.0.0".to_string(),
            host: "10.0.2.11".to_string(),
            port: 9000,
        },
        ServiceInstance {
            instance_id: 12,
            service_name: "bravo".to_string(),
            tier: ServiceTier::Queue,
            env: DeploymentEnv::Production,
            version: "2.1.3".to_string(),
            host: "10.0.2.12".to_string(),
            port: 9001,
        },
        ServiceInstance {
            instance_id: 13,
            service_name: "charlie-external".to_string(),
            tier: ServiceTier::External,
            env: DeploymentEnv::Development,
            version: "0.9.9".to_string(),
            host: "ext.partner.com".to_string(),
            port: 443,
        },
    ];
    let ver = Version::new(2, 0, 0);
    let encoded = encode_versioned_value(&instances, ver).expect("encode failed");
    let (decoded, decoded_ver, _) =
        decode_versioned_value::<Vec<ServiceInstance>>(&encoded).expect("decode failed");
    assert_eq!(decoded, instances);
    assert_eq!(decoded_ver.major, 2);
    assert_eq!(decoded.len(), 3);
    assert_eq!(decoded[1].tier, ServiceTier::Queue);
    assert_eq!(decoded[2].tier, ServiceTier::External);
}

#[test]
fn test_basic_encode_decode_health_check() {
    let check = HealthCheck {
        instance_id: 9999,
        timestamp: 1_700_900_000_000_000,
        status: HealthStatus::Unknown,
        latency_ms: 0,
        checks_passed: 0,
        checks_failed: 0,
    };
    let encoded = encode_to_vec(&check).expect("encode_to_vec failed");
    let (decoded, _consumed) =
        decode_from_slice::<HealthCheck>(&encoded).expect("decode_from_slice failed");
    assert_eq!(decoded, check);
    assert_eq!(decoded.status, HealthStatus::Unknown);
}

#[test]
fn test_basic_encode_decode_circuit_breaker() {
    let cb = CircuitBreaker {
        service_id: 88888,
        state: "OPEN".to_string(),
        failure_count: 99,
        success_count: 1,
        last_state_change: 1_701_000_000_000_000,
    };
    let encoded = encode_to_vec(&cb).expect("encode_to_vec failed");
    let (decoded, _consumed) =
        decode_from_slice::<CircuitBreaker>(&encoded).expect("decode_from_slice failed");
    assert_eq!(decoded, cb);
    assert_eq!(decoded.failure_count, 99);
    assert_eq!(decoded.state, "OPEN");
}

#[test]
fn test_version_fields_v1_2_0_trace_span() {
    let span = TraceSpan {
        span_id: 0xCAFEBABE,
        trace_id: 0xDEADBEEF00000000_0000000000000000u128,
        parent_span_id: Some(0xBEEFCAFE),
        service_name: "mesh-proxy".to_string(),
        operation: "forward_request".to_string(),
        start_us: 1_714_000_000_000_000,
        duration_us: 200,
        status: SpanStatus::Ok,
    };
    let ver = Version::new(1, 2, 0);
    let encoded = encode_versioned_value(&span, ver).expect("encode failed");
    let (decoded, decoded_ver, consumed) =
        decode_versioned_value::<TraceSpan>(&encoded).expect("decode failed");
    assert_eq!(decoded, span);
    assert_eq!(decoded_ver.major, 1);
    assert_eq!(decoded_ver.minor, 2);
    assert_eq!(decoded_ver.patch, 0);
    assert!(consumed > 0);
    assert_eq!(decoded.parent_span_id, Some(0xBEEFCAFE));
}

#[test]
fn test_version_fields_v2_0_0_api_gateway_log() {
    let log = ApiGatewayLog {
        request_id: 999_999,
        service_id: 111_111,
        method: "PATCH".to_string(),
        path: "/internal/config".to_string(),
        status_code: 204,
        response_ms: 8,
        timestamp: 1_725_000_000_000_000,
    };
    let ver = Version::new(2, 0, 0);
    let encoded = encode_versioned_value(&log, ver).expect("encode failed");
    let (decoded, decoded_ver, consumed) =
        decode_versioned_value::<ApiGatewayLog>(&encoded).expect("decode failed");
    assert_eq!(decoded, log);
    assert_eq!(decoded_ver.major, 2);
    assert_eq!(decoded_ver.minor, 0);
    assert_eq!(decoded_ver.patch, 0);
    assert!(consumed > 0, "consumed bytes must be positive");
    assert_eq!(decoded.method, "PATCH");
    assert_eq!(decoded.status_code, 204);
}

#[test]
fn test_vec_health_checks_versioned_v1_2_0() {
    let checks = vec![
        HealthCheck {
            instance_id: 31,
            timestamp: 1_726_000_000_000_000,
            status: HealthStatus::Healthy,
            latency_ms: 5,
            checks_passed: 12,
            checks_failed: 0,
        },
        HealthCheck {
            instance_id: 32,
            timestamp: 1_726_000_001_000_000,
            status: HealthStatus::Degraded,
            latency_ms: 300,
            checks_passed: 9,
            checks_failed: 3,
        },
        HealthCheck {
            instance_id: 33,
            timestamp: 1_726_000_002_000_000,
            status: HealthStatus::Unhealthy,
            latency_ms: 5_000,
            checks_passed: 0,
            checks_failed: 12,
        },
    ];
    let ver = Version::new(1, 2, 0);
    let encoded = encode_versioned_value(&checks, ver).expect("encode failed");
    let (decoded, decoded_ver, _) =
        decode_versioned_value::<Vec<HealthCheck>>(&encoded).expect("decode failed");
    assert_eq!(decoded, checks);
    assert_eq!(decoded_ver.major, 1);
    assert_eq!(decoded_ver.minor, 2);
    assert_eq!(decoded_ver.patch, 0);
    assert_eq!(decoded.len(), 3);
    assert_eq!(decoded[0].status, HealthStatus::Healthy);
    assert_eq!(decoded[1].status, HealthStatus::Degraded);
    assert_eq!(decoded[2].status, HealthStatus::Unhealthy);
}
