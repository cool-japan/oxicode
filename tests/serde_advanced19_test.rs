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
use oxicode::serde::{decode_owned_from_slice, encode_to_vec};
use serde::{Deserialize, Serialize};

#[derive(Debug, PartialEq, Serialize, Deserialize)]
enum MetricType {
    Counter,
    Gauge,
    Histogram,
    Summary,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
struct MetricPoint {
    name: String,
    metric_type: MetricType,
    value: f64,
    timestamp_ms: u64,
    labels: Vec<(String, String)>,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
struct MetricBatch {
    source: String,
    points: Vec<MetricPoint>,
    sequence_id: u64,
    dropped_count: u32,
}

// Test 1: MetricType::Counter roundtrip
#[test]
fn test_metric_type_counter_roundtrip() {
    let cfg = oxicode::config::standard();
    let value = MetricType::Counter;
    let bytes = encode_to_vec(&value, cfg).expect("encode MetricType::Counter");
    let (decoded, _): (MetricType, usize) =
        decode_owned_from_slice(&bytes, cfg).expect("decode MetricType::Counter");
    assert_eq!(value, decoded);
}

// Test 2: MetricType::Gauge roundtrip
#[test]
fn test_metric_type_gauge_roundtrip() {
    let cfg = oxicode::config::standard();
    let value = MetricType::Gauge;
    let bytes = encode_to_vec(&value, cfg).expect("encode MetricType::Gauge");
    let (decoded, _): (MetricType, usize) =
        decode_owned_from_slice(&bytes, cfg).expect("decode MetricType::Gauge");
    assert_eq!(value, decoded);
}

// Test 3: MetricType::Histogram roundtrip
#[test]
fn test_metric_type_histogram_roundtrip() {
    let cfg = oxicode::config::standard();
    let value = MetricType::Histogram;
    let bytes = encode_to_vec(&value, cfg).expect("encode MetricType::Histogram");
    let (decoded, _): (MetricType, usize) =
        decode_owned_from_slice(&bytes, cfg).expect("decode MetricType::Histogram");
    assert_eq!(value, decoded);
}

// Test 4: MetricType::Summary roundtrip
#[test]
fn test_metric_type_summary_roundtrip() {
    let cfg = oxicode::config::standard();
    let value = MetricType::Summary;
    let bytes = encode_to_vec(&value, cfg).expect("encode MetricType::Summary");
    let (decoded, _): (MetricType, usize) =
        decode_owned_from_slice(&bytes, cfg).expect("decode MetricType::Summary");
    assert_eq!(value, decoded);
}

// Test 5: MetricPoint basic roundtrip
#[test]
fn test_metric_point_basic_roundtrip() {
    let cfg = oxicode::config::standard();
    let value = MetricPoint {
        name: "cpu_usage".to_string(),
        metric_type: MetricType::Gauge,
        value: 72.5,
        timestamp_ms: 1_700_000_000_000,
        labels: vec![("host".to_string(), "server-01".to_string())],
    };
    let bytes = encode_to_vec(&value, cfg).expect("encode MetricPoint basic");
    let (decoded, _): (MetricPoint, usize) =
        decode_owned_from_slice(&bytes, cfg).expect("decode MetricPoint basic");
    assert_eq!(value, decoded);
}

// Test 6: MetricPoint with empty labels
#[test]
fn test_metric_point_empty_labels_roundtrip() {
    let cfg = oxicode::config::standard();
    let value = MetricPoint {
        name: "requests_total".to_string(),
        metric_type: MetricType::Counter,
        value: 1024.0,
        timestamp_ms: 1_600_000_000_000,
        labels: vec![],
    };
    let bytes = encode_to_vec(&value, cfg).expect("encode MetricPoint empty labels");
    let (decoded, _): (MetricPoint, usize) =
        decode_owned_from_slice(&bytes, cfg).expect("decode MetricPoint empty labels");
    assert_eq!(value, decoded);
}

// Test 7: MetricPoint with multiple labels
#[test]
fn test_metric_point_multiple_labels_roundtrip() {
    let cfg = oxicode::config::standard();
    let value = MetricPoint {
        name: "http_request_duration_seconds".to_string(),
        metric_type: MetricType::Histogram,
        value: 0.342,
        timestamp_ms: 1_710_000_000_000,
        labels: vec![
            ("method".to_string(), "GET".to_string()),
            ("status".to_string(), "200".to_string()),
            ("route".to_string(), "/api/v1/metrics".to_string()),
        ],
    };
    let bytes = encode_to_vec(&value, cfg).expect("encode MetricPoint multiple labels");
    let (decoded, _): (MetricPoint, usize) =
        decode_owned_from_slice(&bytes, cfg).expect("decode MetricPoint multiple labels");
    assert_eq!(value, decoded);
}

// Test 8: MetricBatch basic roundtrip
#[test]
fn test_metric_batch_basic_roundtrip() {
    let cfg = oxicode::config::standard();
    let value = MetricBatch {
        source: "collector-node-42".to_string(),
        points: vec![MetricPoint {
            name: "memory_bytes".to_string(),
            metric_type: MetricType::Gauge,
            value: 8_589_934_592.0,
            timestamp_ms: 1_705_000_000_000,
            labels: vec![("region".to_string(), "us-east-1".to_string())],
        }],
        sequence_id: 1001,
        dropped_count: 0,
    };
    let bytes = encode_to_vec(&value, cfg).expect("encode MetricBatch basic");
    let (decoded, _): (MetricBatch, usize) =
        decode_owned_from_slice(&bytes, cfg).expect("decode MetricBatch basic");
    assert_eq!(value, decoded);
}

// Test 9: MetricBatch with empty points
#[test]
fn test_metric_batch_empty_points_roundtrip() {
    let cfg = oxicode::config::standard();
    let value = MetricBatch {
        source: "idle-agent".to_string(),
        points: vec![],
        sequence_id: 0,
        dropped_count: 5,
    };
    let bytes = encode_to_vec(&value, cfg).expect("encode MetricBatch empty points");
    let (decoded, _): (MetricBatch, usize) =
        decode_owned_from_slice(&bytes, cfg).expect("decode MetricBatch empty points");
    assert_eq!(value, decoded);
}

// Test 10: MetricBatch with 5 points
#[test]
fn test_metric_batch_five_points_roundtrip() {
    let cfg = oxicode::config::standard();
    let make_point = |name: &str, mt: MetricType, v: f64, ts: u64| MetricPoint {
        name: name.to_string(),
        metric_type: mt,
        value: v,
        timestamp_ms: ts,
        labels: vec![("env".to_string(), "prod".to_string())],
    };
    let value = MetricBatch {
        source: "aggregator-west".to_string(),
        points: vec![
            make_point("cpu_usage", MetricType::Gauge, 45.1, 1_700_000_001_000),
            make_point("mem_usage", MetricType::Gauge, 60.2, 1_700_000_002_000),
            make_point("req_count", MetricType::Counter, 9999.0, 1_700_000_003_000),
            make_point("latency_p99", MetricType::Summary, 0.512, 1_700_000_004_000),
            make_point(
                "req_duration",
                MetricType::Histogram,
                0.123,
                1_700_000_005_000,
            ),
        ],
        sequence_id: 42,
        dropped_count: 2,
    };
    let bytes = encode_to_vec(&value, cfg).expect("encode MetricBatch 5 points");
    let (decoded, _): (MetricBatch, usize) =
        decode_owned_from_slice(&bytes, cfg).expect("decode MetricBatch 5 points");
    assert_eq!(value, decoded);
}

// Test 11: Vec<MetricPoint> roundtrip
#[test]
fn test_vec_metric_point_roundtrip() {
    let cfg = oxicode::config::standard();
    let value: Vec<MetricPoint> = vec![
        MetricPoint {
            name: "disk_read_bytes".to_string(),
            metric_type: MetricType::Counter,
            value: 1_048_576.0,
            timestamp_ms: 1_700_100_000_000,
            labels: vec![("device".to_string(), "sda".to_string())],
        },
        MetricPoint {
            name: "disk_write_bytes".to_string(),
            metric_type: MetricType::Counter,
            value: 524_288.0,
            timestamp_ms: 1_700_100_001_000,
            labels: vec![("device".to_string(), "sda".to_string())],
        },
    ];
    let bytes = encode_to_vec(&value, cfg).expect("encode Vec<MetricPoint>");
    let (decoded, _): (Vec<MetricPoint>, usize) =
        decode_owned_from_slice(&bytes, cfg).expect("decode Vec<MetricPoint>");
    assert_eq!(value, decoded);
}

// Test 12: Vec<MetricBatch> roundtrip
#[test]
fn test_vec_metric_batch_roundtrip() {
    let cfg = oxicode::config::standard();
    let value: Vec<MetricBatch> = vec![
        MetricBatch {
            source: "node-1".to_string(),
            points: vec![MetricPoint {
                name: "uptime_seconds".to_string(),
                metric_type: MetricType::Counter,
                value: 86400.0,
                timestamp_ms: 1_700_200_000_000,
                labels: vec![],
            }],
            sequence_id: 10,
            dropped_count: 0,
        },
        MetricBatch {
            source: "node-2".to_string(),
            points: vec![],
            sequence_id: 11,
            dropped_count: 1,
        },
    ];
    let bytes = encode_to_vec(&value, cfg).expect("encode Vec<MetricBatch>");
    let (decoded, _): (Vec<MetricBatch>, usize) =
        decode_owned_from_slice(&bytes, cfg).expect("decode Vec<MetricBatch>");
    assert_eq!(value, decoded);
}

// Test 13: MetricBatch consumed bytes == encoded length
#[test]
fn test_metric_batch_consumed_bytes_eq_encoded_len() {
    let cfg = oxicode::config::standard();
    let value = MetricBatch {
        source: "probe-alpha".to_string(),
        points: vec![MetricPoint {
            name: "network_rx_bytes".to_string(),
            metric_type: MetricType::Counter,
            value: 4096.0,
            timestamp_ms: 1_700_300_000_000,
            labels: vec![("interface".to_string(), "eth0".to_string())],
        }],
        sequence_id: 7,
        dropped_count: 0,
    };
    let bytes = encode_to_vec(&value, cfg).expect("encode MetricBatch for size check");
    let (_decoded, consumed): (MetricBatch, usize) =
        decode_owned_from_slice(&bytes, cfg).expect("decode MetricBatch for size check");
    assert_eq!(consumed, bytes.len());
}

// Test 14: Encode determinism for MetricPoint
#[test]
fn test_metric_point_encode_determinism() {
    let cfg = oxicode::config::standard();
    let value = MetricPoint {
        name: "error_rate".to_string(),
        metric_type: MetricType::Summary,
        value: 0.001,
        timestamp_ms: 1_700_400_000_000,
        labels: vec![("service".to_string(), "auth".to_string())],
    };
    let bytes_a = encode_to_vec(&value, cfg).expect("encode MetricPoint determinism A");
    let bytes_b = encode_to_vec(&value, cfg).expect("encode MetricPoint determinism B");
    assert_eq!(bytes_a, bytes_b);
}

// Test 15: Big-endian config MetricBatch roundtrip
#[test]
fn test_metric_batch_big_endian_roundtrip() {
    let cfg = oxicode::config::standard().with_big_endian();
    let value = MetricBatch {
        source: "be-collector".to_string(),
        points: vec![MetricPoint {
            name: "jvm_heap_used_bytes".to_string(),
            metric_type: MetricType::Gauge,
            value: 536_870_912.0,
            timestamp_ms: 1_700_500_000_000,
            labels: vec![("jvm_version".to_string(), "17".to_string())],
        }],
        sequence_id: 99,
        dropped_count: 3,
    };
    let bytes = encode_to_vec(&value, cfg).expect("encode MetricBatch big-endian");
    let (decoded, _): (MetricBatch, usize) =
        decode_owned_from_slice(&bytes, cfg).expect("decode MetricBatch big-endian");
    assert_eq!(value, decoded);
}

// Test 16: Fixed-int config MetricPoint roundtrip
#[test]
fn test_metric_point_fixed_int_roundtrip() {
    let cfg = oxicode::config::standard().with_fixed_int_encoding();
    let value = MetricPoint {
        name: "gc_pause_ms".to_string(),
        metric_type: MetricType::Histogram,
        value: 12.5,
        timestamp_ms: 1_700_600_000_000,
        labels: vec![("collector".to_string(), "G1GC".to_string())],
    };
    let bytes = encode_to_vec(&value, cfg).expect("encode MetricPoint fixed-int");
    let (decoded, _): (MetricPoint, usize) =
        decode_owned_from_slice(&bytes, cfg).expect("decode MetricPoint fixed-int");
    assert_eq!(value, decoded);
}

// Test 17: f64::MAX value in MetricPoint roundtrip
#[test]
fn test_metric_point_f64_max_roundtrip() {
    let cfg = oxicode::config::standard();
    let value = MetricPoint {
        name: "max_observation".to_string(),
        metric_type: MetricType::Gauge,
        value: f64::MAX,
        timestamp_ms: 1_700_700_000_000,
        labels: vec![],
    };
    let bytes = encode_to_vec(&value, cfg).expect("encode MetricPoint f64::MAX");
    let (decoded, _): (MetricPoint, usize) =
        decode_owned_from_slice(&bytes, cfg).expect("decode MetricPoint f64::MAX");
    assert_eq!(value, decoded);
}

// Test 18: f64 NaN in MetricPoint — compare via to_bits()
#[test]
fn test_metric_point_f64_nan_roundtrip() {
    let cfg = oxicode::config::standard();
    let nan_value = f64::NAN;
    let value = MetricPoint {
        name: "nan_metric".to_string(),
        metric_type: MetricType::Gauge,
        value: nan_value,
        timestamp_ms: 1_700_800_000_000,
        labels: vec![],
    };
    let bytes = encode_to_vec(&value, cfg).expect("encode MetricPoint NaN");
    let (decoded, _): (MetricPoint, usize) =
        decode_owned_from_slice(&bytes, cfg).expect("decode MetricPoint NaN");
    assert_eq!(value.value.to_bits(), decoded.value.to_bits());
    assert_eq!(value.name, decoded.name);
    assert_eq!(value.metric_type, decoded.metric_type);
    assert_eq!(value.timestamp_ms, decoded.timestamp_ms);
    assert_eq!(value.labels, decoded.labels);
}

// Test 19: Unicode label keys/values roundtrip
#[test]
fn test_metric_point_unicode_labels_roundtrip() {
    let cfg = oxicode::config::standard();
    let value = MetricPoint {
        name: "unicode_metric".to_string(),
        metric_type: MetricType::Counter,
        value: 1.0,
        timestamp_ms: 1_700_900_000_000,
        labels: vec![
            ("データセンター".to_string(), "東京".to_string()),
            ("サービス".to_string(), "認証".to_string()),
            ("emoji_key".to_string(), "🚀🔥💡".to_string()),
        ],
    };
    let bytes = encode_to_vec(&value, cfg).expect("encode MetricPoint unicode labels");
    let (decoded, _): (MetricPoint, usize) =
        decode_owned_from_slice(&bytes, cfg).expect("decode MetricPoint unicode labels");
    assert_eq!(value, decoded);
}

// Test 20: Large labels list (10 label pairs) roundtrip
#[test]
fn test_metric_point_large_labels_roundtrip() {
    let cfg = oxicode::config::standard();
    let labels: Vec<(String, String)> = (0..10)
        .map(|i| (format!("label_key_{i:02}"), format!("label_value_{i:02}")))
        .collect();
    let value = MetricPoint {
        name: "high_cardinality_metric".to_string(),
        metric_type: MetricType::Summary,
        value: 99.9,
        timestamp_ms: 1_701_000_000_000,
        labels,
    };
    let bytes = encode_to_vec(&value, cfg).expect("encode MetricPoint large labels");
    let (decoded, _): (MetricPoint, usize) =
        decode_owned_from_slice(&bytes, cfg).expect("decode MetricPoint large labels");
    assert_eq!(value, decoded);
}

// Test 21: Zero timestamp MetricPoint roundtrip
#[test]
fn test_metric_point_zero_timestamp_roundtrip() {
    let cfg = oxicode::config::standard();
    let value = MetricPoint {
        name: "epoch_origin".to_string(),
        metric_type: MetricType::Counter,
        value: 0.0,
        timestamp_ms: 0,
        labels: vec![("note".to_string(), "unix_epoch".to_string())],
    };
    let bytes = encode_to_vec(&value, cfg).expect("encode MetricPoint zero timestamp");
    let (decoded, _): (MetricPoint, usize) =
        decode_owned_from_slice(&bytes, cfg).expect("decode MetricPoint zero timestamp");
    assert_eq!(value, decoded);
}

// Test 22: u64::MAX sequence_id MetricBatch roundtrip
#[test]
fn test_metric_batch_u64_max_sequence_id_roundtrip() {
    let cfg = oxicode::config::standard();
    let value = MetricBatch {
        source: "overflow-detector".to_string(),
        points: vec![MetricPoint {
            name: "rollover_counter".to_string(),
            metric_type: MetricType::Counter,
            value: 1.0,
            timestamp_ms: 1_701_100_000_000,
            labels: vec![],
        }],
        sequence_id: u64::MAX,
        dropped_count: 0,
    };
    let bytes = encode_to_vec(&value, cfg).expect("encode MetricBatch u64::MAX sequence_id");
    let (decoded, _): (MetricBatch, usize) =
        decode_owned_from_slice(&bytes, cfg).expect("decode MetricBatch u64::MAX sequence_id");
    assert_eq!(value, decoded);
}
