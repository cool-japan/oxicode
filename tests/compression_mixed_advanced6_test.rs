#![cfg(all(feature = "compression-lz4", feature = "compression-zstd"))]
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
use oxicode::compression::{compress, decompress, Compression};
use oxicode::{decode_from_slice, encode_to_vec, Decode, Encode};

// ── Domain types: log analytics / observability ──────────────────────────────

#[derive(Debug, Clone, PartialEq, Encode, Decode)]
enum LogLevel {
    Trace,
    Debug,
    Info,
    Warn,
    Error,
    Fatal,
}

#[derive(Debug, Clone, PartialEq, Encode, Decode)]
struct LogEntry {
    timestamp_ms: u64,
    level: LogLevel,
    service: String,
    message: String,
    trace_id: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Encode, Decode)]
enum MetricType {
    Counter,
    Gauge,
    Histogram,
    Summary,
}

#[derive(Debug, Clone, PartialEq, Encode, Decode)]
struct MetricPoint {
    name: String,
    metric_type: MetricType,
    value: f64,
    labels: Vec<(String, String)>,
    timestamp_ms: u64,
}

#[derive(Debug, Clone, PartialEq, Encode, Decode)]
enum SpanKind {
    Client,
    Server,
    Producer,
    Consumer,
    Internal,
}

#[derive(Debug, Clone, PartialEq, Encode, Decode)]
struct TraceSpan {
    span_id: u64,
    trace_id: u64,
    parent_id: Option<u64>,
    name: String,
    kind: SpanKind,
    start_ms: u64,
    duration_ms: u64,
}

// ── Helpers ──────────────────────────────────────────────────────────────────

fn make_log_entry(
    ts: u64,
    level: LogLevel,
    service: &str,
    msg: &str,
    tid: Option<&str>,
) -> LogEntry {
    LogEntry {
        timestamp_ms: ts,
        level,
        service: service.to_string(),
        message: msg.to_string(),
        trace_id: tid.map(|s| s.to_string()),
    }
}

fn make_metric(name: &str, mtype: MetricType, value: f64, ts: u64) -> MetricPoint {
    MetricPoint {
        name: name.to_string(),
        metric_type: mtype,
        value,
        labels: vec![
            ("env".to_string(), "prod".to_string()),
            ("region".to_string(), "us-east-1".to_string()),
        ],
        timestamp_ms: ts,
    }
}

fn make_span(
    span_id: u64,
    trace_id: u64,
    parent: Option<u64>,
    name: &str,
    kind: SpanKind,
    start: u64,
    dur: u64,
) -> TraceSpan {
    TraceSpan {
        span_id,
        trace_id,
        parent_id: parent,
        name: name.to_string(),
        kind,
        start_ms: start,
        duration_ms: dur,
    }
}

// ── Test 1: LogEntry Info level LZ4 roundtrip ────────────────────────────────
#[test]
fn test_log_entry_info_lz4_roundtrip() {
    let entry = make_log_entry(
        1_700_000_000_000,
        LogLevel::Info,
        "auth-service",
        "User login successful for user_id=42",
        Some("trace-abc-001"),
    );
    let encoded = encode_to_vec(&entry).expect("encode LogEntry Info");
    let compressed = compress(&encoded, Compression::Lz4).expect("lz4 compress LogEntry Info");
    let decompressed = decompress(&compressed).expect("lz4 decompress LogEntry Info");
    let (decoded, _): (LogEntry, usize) =
        decode_from_slice(&decompressed).expect("decode LogEntry Info lz4");
    assert_eq!(entry, decoded);
}

// ── Test 2: LogEntry Error level Zstd roundtrip ──────────────────────────────
#[test]
fn test_log_entry_error_zstd_roundtrip() {
    let entry = make_log_entry(
        1_700_000_001_000,
        LogLevel::Error,
        "payment-service",
        "Failed to charge card: insufficient funds for order_id=9999",
        Some("trace-def-002"),
    );
    let encoded = encode_to_vec(&entry).expect("encode LogEntry Error");
    let compressed = compress(&encoded, Compression::Zstd).expect("zstd compress LogEntry Error");
    let decompressed = decompress(&compressed).expect("zstd decompress LogEntry Error");
    let (decoded, _): (LogEntry, usize) =
        decode_from_slice(&decompressed).expect("decode LogEntry Error zstd");
    assert_eq!(entry, decoded);
}

// ── Test 3: MetricPoint Counter LZ4 roundtrip ────────────────────────────────
#[test]
fn test_metric_point_counter_lz4_roundtrip() {
    let metric = make_metric(
        "http_requests_total",
        MetricType::Counter,
        123456.0,
        1_700_000_002_000,
    );
    let encoded = encode_to_vec(&metric).expect("encode MetricPoint Counter");
    let compressed =
        compress(&encoded, Compression::Lz4).expect("lz4 compress MetricPoint Counter");
    let decompressed = decompress(&compressed).expect("lz4 decompress MetricPoint Counter");
    let (decoded, _): (MetricPoint, usize) =
        decode_from_slice(&decompressed).expect("decode MetricPoint Counter lz4");
    assert_eq!(metric, decoded);
}

// ── Test 4: MetricPoint Gauge Zstd roundtrip ─────────────────────────────────
#[test]
fn test_metric_point_gauge_zstd_roundtrip() {
    let metric = make_metric(
        "memory_usage_bytes",
        MetricType::Gauge,
        536_870_912.0,
        1_700_000_003_000,
    );
    let encoded = encode_to_vec(&metric).expect("encode MetricPoint Gauge");
    let compressed =
        compress(&encoded, Compression::Zstd).expect("zstd compress MetricPoint Gauge");
    let decompressed = decompress(&compressed).expect("zstd decompress MetricPoint Gauge");
    let (decoded, _): (MetricPoint, usize) =
        decode_from_slice(&decompressed).expect("decode MetricPoint Gauge zstd");
    assert_eq!(metric, decoded);
}

// ── Test 5: TraceSpan Server kind LZ4 roundtrip ───────────────────────────────
#[test]
fn test_trace_span_server_lz4_roundtrip() {
    let span = make_span(
        0xDEAD_BEEF_0001,
        0xCAFE_BABE_0001,
        None,
        "HTTP POST /api/v1/orders",
        SpanKind::Server,
        1_700_000_004_000,
        42,
    );
    let encoded = encode_to_vec(&span).expect("encode TraceSpan Server");
    let compressed = compress(&encoded, Compression::Lz4).expect("lz4 compress TraceSpan Server");
    let decompressed = decompress(&compressed).expect("lz4 decompress TraceSpan Server");
    let (decoded, _): (TraceSpan, usize) =
        decode_from_slice(&decompressed).expect("decode TraceSpan Server lz4");
    assert_eq!(span, decoded);
}

// ── Test 6: TraceSpan Client kind Zstd roundtrip ──────────────────────────────
#[test]
fn test_trace_span_client_zstd_roundtrip() {
    let span = make_span(
        0xDEAD_BEEF_0002,
        0xCAFE_BABE_0001,
        Some(0xDEAD_BEEF_0001),
        "SQL SELECT users WHERE id=?",
        SpanKind::Client,
        1_700_000_004_005,
        15,
    );
    let encoded = encode_to_vec(&span).expect("encode TraceSpan Client");
    let compressed = compress(&encoded, Compression::Zstd).expect("zstd compress TraceSpan Client");
    let decompressed = decompress(&compressed).expect("zstd decompress TraceSpan Client");
    let (decoded, _): (TraceSpan, usize) =
        decode_from_slice(&decompressed).expect("decode TraceSpan Client zstd");
    assert_eq!(span, decoded);
}

// ── Test 7: LZ4 compressed size is smaller than raw for large log batch ──────
#[test]
fn test_lz4_compressed_size_smaller_for_large_log_batch() {
    let entries: Vec<LogEntry> = (0u64..500)
        .map(|i| {
            make_log_entry(
                1_700_000_000_000 + i * 100,
                LogLevel::Info,
                "order-service",
                "Processing order request for customer account within expected SLA parameters",
                Some("trace-repeated-id-0000000000000001"),
            )
        })
        .collect();
    let encoded = encode_to_vec(&entries).expect("encode large log batch for lz4 size test");
    let compressed = compress(&encoded, Compression::Lz4).expect("lz4 compress large log batch");
    assert!(
        compressed.len() < encoded.len(),
        "LZ4 compressed large log batch ({}) must be smaller than raw ({})",
        compressed.len(),
        encoded.len()
    );
}

// ── Test 8: Zstd compressed size is smaller than raw for large log batch ─────
#[test]
fn test_zstd_compressed_size_smaller_for_large_log_batch() {
    let entries: Vec<LogEntry> = (0u64..500)
        .map(|i| {
            make_log_entry(
            1_700_000_000_000 + i * 100,
            LogLevel::Warn,
            "inventory-service",
            "Stock level below threshold for SKU product-item-reorder-alert-generated-by-system",
            Some("trace-repeated-id-0000000000000002"),
        )
        })
        .collect();
    let encoded = encode_to_vec(&entries).expect("encode large log batch for zstd size test");
    let compressed = compress(&encoded, Compression::Zstd).expect("zstd compress large log batch");
    assert!(
        compressed.len() < encoded.len(),
        "Zstd compressed large log batch ({}) must be smaller than raw ({})",
        compressed.len(),
        encoded.len()
    );
}

// ── Test 9: Cross-algorithm: LZ4-encode → Zstd verify bytes match ─────────────
#[test]
fn test_cross_algorithm_lz4_then_zstd_decompressed_bytes_identical() {
    let span = make_span(
        0xABCD_EF01_2345,
        0x1234_5678_9ABC,
        None,
        "gRPC Unary /TraceService/Export",
        SpanKind::Internal,
        1_700_000_010_000,
        7,
    );
    let original = encode_to_vec(&span).expect("encode TraceSpan for cross-algorithm test");
    let lz4_comp = compress(&original, Compression::Lz4).expect("lz4 compress cross-algorithm");
    let zstd_comp = compress(&original, Compression::Zstd).expect("zstd compress cross-algorithm");
    let lz4_out = decompress(&lz4_comp).expect("lz4 decompress cross-algorithm");
    let zstd_out = decompress(&zstd_comp).expect("zstd decompress cross-algorithm");
    assert_eq!(
        lz4_out, zstd_out,
        "LZ4 and Zstd decompressed output must be identical bytes"
    );
    assert_eq!(original, lz4_out);
}

// ── Test 10: Large batch of TraceSpans LZ4 roundtrip ─────────────────────────
#[test]
fn test_large_trace_span_batch_lz4_roundtrip() {
    let spans: Vec<TraceSpan> = (0u64..200)
        .map(|i| {
            make_span(
                i + 1,
                0xF000_0000_0000 + i,
                if i == 0 { None } else { Some(i) },
                "kafka.produce",
                SpanKind::Producer,
                1_700_000_020_000 + i * 5,
                2 + i % 10,
            )
        })
        .collect();
    let encoded = encode_to_vec(&spans).expect("encode large TraceSpan batch");
    let compressed =
        compress(&encoded, Compression::Lz4).expect("lz4 compress large TraceSpan batch");
    let decompressed = decompress(&compressed).expect("lz4 decompress large TraceSpan batch");
    let (decoded, _): (Vec<TraceSpan>, usize) =
        decode_from_slice(&decompressed).expect("decode large TraceSpan batch lz4");
    assert_eq!(spans, decoded);
    assert_eq!(decoded.len(), 200);
}

// ── Test 11: All LogLevel variants roundtrip via LZ4 ─────────────────────────
#[test]
fn test_all_log_level_variants_lz4_roundtrip() {
    let levels = [
        LogLevel::Trace,
        LogLevel::Debug,
        LogLevel::Info,
        LogLevel::Warn,
        LogLevel::Error,
        LogLevel::Fatal,
    ];
    for (idx, level) in levels.into_iter().enumerate() {
        let entry = make_log_entry(
            1_700_000_030_000 + idx as u64,
            level,
            "test-service",
            &format!("Log message for level variant index {idx}"),
            None,
        );
        let encoded = encode_to_vec(&entry).expect("encode LogEntry for all-levels lz4");
        let compressed = compress(&encoded, Compression::Lz4).expect("lz4 compress all-levels");
        let decompressed = decompress(&compressed).expect("lz4 decompress all-levels");
        let (decoded, _): (LogEntry, usize) =
            decode_from_slice(&decompressed).expect("decode all-levels lz4");
        assert_eq!(
            entry, decoded,
            "LogLevel variant index {idx} failed lz4 roundtrip"
        );
    }
}

// ── Test 12: All LogLevel variants roundtrip via Zstd ────────────────────────
#[test]
fn test_all_log_level_variants_zstd_roundtrip() {
    let levels = [
        LogLevel::Trace,
        LogLevel::Debug,
        LogLevel::Info,
        LogLevel::Warn,
        LogLevel::Error,
        LogLevel::Fatal,
    ];
    for (idx, level) in levels.into_iter().enumerate() {
        let entry = make_log_entry(
            1_700_000_040_000 + idx as u64,
            level,
            "monitoring-agent",
            &format!("Observability event captured at level index {idx}"),
            Some(&format!("trace-{idx:08x}")),
        );
        let encoded = encode_to_vec(&entry).expect("encode LogEntry for all-levels zstd");
        let compressed = compress(&encoded, Compression::Zstd).expect("zstd compress all-levels");
        let decompressed = decompress(&compressed).expect("zstd decompress all-levels");
        let (decoded, _): (LogEntry, usize) =
            decode_from_slice(&decompressed).expect("decode all-levels zstd");
        assert_eq!(
            entry, decoded,
            "LogLevel variant index {idx} failed zstd roundtrip"
        );
    }
}

// ── Test 13: All MetricType variants LZ4 roundtrip ───────────────────────────
#[test]
fn test_all_metric_type_variants_lz4_roundtrip() {
    let types = [
        MetricType::Counter,
        MetricType::Gauge,
        MetricType::Histogram,
        MetricType::Summary,
    ];
    for (idx, mtype) in types.into_iter().enumerate() {
        let metric = make_metric(
            &format!("metric_variant_{idx}"),
            mtype,
            (idx as f64 + 1.0) * 100.0,
            1_700_000_050_000 + idx as u64,
        );
        let encoded = encode_to_vec(&metric).expect("encode MetricPoint all-types lz4");
        let compressed =
            compress(&encoded, Compression::Lz4).expect("lz4 compress MetricPoint all-types");
        let decompressed = decompress(&compressed).expect("lz4 decompress MetricPoint all-types");
        let (decoded, _): (MetricPoint, usize) =
            decode_from_slice(&decompressed).expect("decode MetricPoint all-types lz4");
        assert_eq!(
            metric, decoded,
            "MetricType variant {idx} failed lz4 roundtrip"
        );
    }
}

// ── Test 14: All MetricType variants Zstd roundtrip ──────────────────────────
#[test]
fn test_all_metric_type_variants_zstd_roundtrip() {
    let types = [
        MetricType::Counter,
        MetricType::Gauge,
        MetricType::Histogram,
        MetricType::Summary,
    ];
    for (idx, mtype) in types.into_iter().enumerate() {
        let metric = make_metric(
            &format!("metric_zstd_variant_{idx}"),
            mtype,
            (idx as f64 + 1.5) * 250.0,
            1_700_000_060_000 + idx as u64,
        );
        let encoded = encode_to_vec(&metric).expect("encode MetricPoint all-types zstd");
        let compressed =
            compress(&encoded, Compression::Zstd).expect("zstd compress MetricPoint all-types");
        let decompressed = decompress(&compressed).expect("zstd decompress MetricPoint all-types");
        let (decoded, _): (MetricPoint, usize) =
            decode_from_slice(&decompressed).expect("decode MetricPoint all-types zstd");
        assert_eq!(
            metric, decoded,
            "MetricType variant {idx} failed zstd roundtrip"
        );
    }
}

// ── Test 15: All SpanKind variants LZ4 roundtrip ─────────────────────────────
#[test]
fn test_all_span_kind_variants_lz4_roundtrip() {
    let kinds = [
        SpanKind::Client,
        SpanKind::Server,
        SpanKind::Producer,
        SpanKind::Consumer,
        SpanKind::Internal,
    ];
    for (idx, kind) in kinds.into_iter().enumerate() {
        let span = make_span(
            idx as u64 + 1000,
            0xBEEF_0000_0000 + idx as u64,
            None,
            &format!("span-kind-{idx}"),
            kind,
            1_700_000_070_000 + idx as u64 * 10,
            idx as u64 + 1,
        );
        let encoded = encode_to_vec(&span).expect("encode TraceSpan all-kinds lz4");
        let compressed =
            compress(&encoded, Compression::Lz4).expect("lz4 compress TraceSpan all-kinds");
        let decompressed = decompress(&compressed).expect("lz4 decompress TraceSpan all-kinds");
        let (decoded, _): (TraceSpan, usize) =
            decode_from_slice(&decompressed).expect("decode TraceSpan all-kinds lz4");
        assert_eq!(span, decoded, "SpanKind variant {idx} failed lz4 roundtrip");
    }
}

// ── Test 16: All SpanKind variants Zstd roundtrip ────────────────────────────
#[test]
fn test_all_span_kind_variants_zstd_roundtrip() {
    let kinds = [
        SpanKind::Client,
        SpanKind::Server,
        SpanKind::Producer,
        SpanKind::Consumer,
        SpanKind::Internal,
    ];
    for (idx, kind) in kinds.into_iter().enumerate() {
        let span = make_span(
            idx as u64 + 2000,
            0xFACE_0000_0000 + idx as u64,
            Some(idx as u64 + 1),
            &format!("span-kind-zstd-{idx}"),
            kind,
            1_700_000_080_000 + idx as u64 * 10,
            idx as u64 + 2,
        );
        let encoded = encode_to_vec(&span).expect("encode TraceSpan all-kinds zstd");
        let compressed =
            compress(&encoded, Compression::Zstd).expect("zstd compress TraceSpan all-kinds");
        let decompressed = decompress(&compressed).expect("zstd decompress TraceSpan all-kinds");
        let (decoded, _): (TraceSpan, usize) =
            decode_from_slice(&decompressed).expect("decode TraceSpan all-kinds zstd");
        assert_eq!(
            span, decoded,
            "SpanKind variant {idx} failed zstd roundtrip"
        );
    }
}

// ── Test 17: Vec<MetricPoint> mixed types LZ4 roundtrip ──────────────────────
#[test]
fn test_vec_metric_points_mixed_lz4_roundtrip() {
    let metrics = vec![
        make_metric(
            "cpu_usage_percent",
            MetricType::Gauge,
            73.5,
            1_700_000_090_000,
        ),
        make_metric(
            "api_calls_total",
            MetricType::Counter,
            9_000_000.0,
            1_700_000_090_001,
        ),
        make_metric(
            "response_time_ms",
            MetricType::Histogram,
            42.7,
            1_700_000_090_002,
        ),
        make_metric(
            "error_rate_p99",
            MetricType::Summary,
            0.0012,
            1_700_000_090_003,
        ),
    ];
    let encoded = encode_to_vec(&metrics).expect("encode Vec<MetricPoint> lz4");
    let compressed = compress(&encoded, Compression::Lz4).expect("lz4 compress Vec<MetricPoint>");
    let decompressed = decompress(&compressed).expect("lz4 decompress Vec<MetricPoint>");
    let (decoded, _): (Vec<MetricPoint>, usize) =
        decode_from_slice(&decompressed).expect("decode Vec<MetricPoint> lz4");
    assert_eq!(metrics, decoded);
}

// ── Test 18: Vec<MetricPoint> mixed types Zstd roundtrip ─────────────────────
#[test]
fn test_vec_metric_points_mixed_zstd_roundtrip() {
    let metrics = vec![
        make_metric(
            "disk_read_bytes",
            MetricType::Counter,
            1_048_576.0,
            1_700_000_100_000,
        ),
        make_metric(
            "open_connections",
            MetricType::Gauge,
            512.0,
            1_700_000_100_001,
        ),
        make_metric(
            "db_query_latency",
            MetricType::Histogram,
            5.3,
            1_700_000_100_002,
        ),
        make_metric(
            "cache_hit_ratio",
            MetricType::Summary,
            0.94,
            1_700_000_100_003,
        ),
    ];
    let encoded = encode_to_vec(&metrics).expect("encode Vec<MetricPoint> zstd");
    let compressed = compress(&encoded, Compression::Zstd).expect("zstd compress Vec<MetricPoint>");
    let decompressed = decompress(&compressed).expect("zstd decompress Vec<MetricPoint>");
    let (decoded, _): (Vec<MetricPoint>, usize) =
        decode_from_slice(&decompressed).expect("decode Vec<MetricPoint> zstd");
    assert_eq!(metrics, decoded);
}

// ── Test 19: LogEntry with no trace_id (None) LZ4 roundtrip ──────────────────
#[test]
fn test_log_entry_no_trace_id_lz4_roundtrip() {
    let entry = make_log_entry(
        1_700_000_110_000,
        LogLevel::Debug,
        "config-loader",
        "Loaded configuration from /etc/app/config.yaml",
        None,
    );
    let encoded = encode_to_vec(&entry).expect("encode LogEntry no trace_id lz4");
    let compressed =
        compress(&encoded, Compression::Lz4).expect("lz4 compress LogEntry no trace_id");
    let decompressed = decompress(&compressed).expect("lz4 decompress LogEntry no trace_id");
    let (decoded, _): (LogEntry, usize) =
        decode_from_slice(&decompressed).expect("decode LogEntry no trace_id lz4");
    assert_eq!(entry, decoded);
    assert!(
        decoded.trace_id.is_none(),
        "trace_id must be None after lz4 roundtrip"
    );
}

// ── Test 20: LogEntry with no trace_id (None) Zstd roundtrip ─────────────────
#[test]
fn test_log_entry_no_trace_id_zstd_roundtrip() {
    let entry = make_log_entry(
        1_700_000_120_000,
        LogLevel::Trace,
        "health-checker",
        "Performing readiness probe against /healthz endpoint at upstream",
        None,
    );
    let encoded = encode_to_vec(&entry).expect("encode LogEntry no trace_id zstd");
    let compressed =
        compress(&encoded, Compression::Zstd).expect("zstd compress LogEntry no trace_id");
    let decompressed = decompress(&compressed).expect("zstd decompress LogEntry no trace_id");
    let (decoded, _): (LogEntry, usize) =
        decode_from_slice(&decompressed).expect("decode LogEntry no trace_id zstd");
    assert_eq!(entry, decoded);
    assert!(
        decoded.trace_id.is_none(),
        "trace_id must be None after zstd roundtrip"
    );
}

// ── Test 21: Full observability pipeline: logs + metrics + spans LZ4 ─────────
#[test]
fn test_full_observability_pipeline_lz4() {
    let logs: Vec<LogEntry> = vec![
        make_log_entry(
            1_700_000_130_000,
            LogLevel::Info,
            "api-gateway",
            "Request received",
            Some("trace-pipe-001"),
        ),
        make_log_entry(
            1_700_000_130_050,
            LogLevel::Warn,
            "rate-limiter",
            "Throttle threshold approaching 80%",
            Some("trace-pipe-001"),
        ),
    ];
    let metrics: Vec<MetricPoint> = vec![
        make_metric(
            "gateway_requests_total",
            MetricType::Counter,
            100.0,
            1_700_000_130_000,
        ),
        make_metric(
            "gateway_latency_ms",
            MetricType::Histogram,
            18.4,
            1_700_000_130_001,
        ),
    ];
    let spans: Vec<TraceSpan> = vec![
        make_span(
            1,
            0xA1BE_0001,
            None,
            "api-gateway.handle",
            SpanKind::Server,
            1_700_000_130_000,
            50,
        ),
        make_span(
            2,
            0xA1BE_0001,
            Some(1),
            "db.query",
            SpanKind::Client,
            1_700_000_130_010,
            20,
        ),
    ];

    let enc_logs = encode_to_vec(&logs).expect("encode logs pipeline lz4");
    let enc_metrics = encode_to_vec(&metrics).expect("encode metrics pipeline lz4");
    let enc_spans = encode_to_vec(&spans).expect("encode spans pipeline lz4");

    let comp_logs = compress(&enc_logs, Compression::Lz4).expect("lz4 compress logs pipeline");
    let comp_metrics =
        compress(&enc_metrics, Compression::Lz4).expect("lz4 compress metrics pipeline");
    let comp_spans = compress(&enc_spans, Compression::Lz4).expect("lz4 compress spans pipeline");

    let decomp_logs = decompress(&comp_logs).expect("lz4 decompress logs pipeline");
    let decomp_metrics = decompress(&comp_metrics).expect("lz4 decompress metrics pipeline");
    let decomp_spans = decompress(&comp_spans).expect("lz4 decompress spans pipeline");

    let (dec_logs, _): (Vec<LogEntry>, usize) =
        decode_from_slice(&decomp_logs).expect("decode logs pipeline lz4");
    let (dec_metrics, _): (Vec<MetricPoint>, usize) =
        decode_from_slice(&decomp_metrics).expect("decode metrics pipeline lz4");
    let (dec_spans, _): (Vec<TraceSpan>, usize) =
        decode_from_slice(&decomp_spans).expect("decode spans pipeline lz4");

    assert_eq!(logs, dec_logs);
    assert_eq!(metrics, dec_metrics);
    assert_eq!(spans, dec_spans);
}

// ── Test 22: Full observability pipeline: logs + metrics + spans Zstd ────────
#[test]
fn test_full_observability_pipeline_zstd() {
    let logs: Vec<LogEntry> = vec![
        make_log_entry(
            1_700_000_140_000,
            LogLevel::Fatal,
            "crash-reporter",
            "Out of memory: process killed by OOM killer",
            Some("trace-oom-999"),
        ),
        make_log_entry(
            1_700_000_140_010,
            LogLevel::Error,
            "alertmanager",
            "Firing alert: MemoryHighWatermark for pod worker-7",
            Some("trace-oom-999"),
        ),
    ];
    let metrics: Vec<MetricPoint> = vec![
        make_metric(
            "process_resident_bytes",
            MetricType::Gauge,
            2_147_483_648.0,
            1_700_000_140_000,
        ),
        make_metric(
            "oom_kills_total",
            MetricType::Counter,
            3.0,
            1_700_000_140_001,
        ),
    ];
    let spans: Vec<TraceSpan> = vec![
        make_span(
            10,
            0x00AB_00001,
            None,
            "worker.run_batch",
            SpanKind::Internal,
            1_700_000_139_000,
            1000,
        ),
        make_span(
            11,
            0x00AB_00001,
            Some(10),
            "queue.consume",
            SpanKind::Consumer,
            1_700_000_139_100,
            900,
        ),
    ];

    let enc_logs = encode_to_vec(&logs).expect("encode logs pipeline zstd");
    let enc_metrics = encode_to_vec(&metrics).expect("encode metrics pipeline zstd");
    let enc_spans = encode_to_vec(&spans).expect("encode spans pipeline zstd");

    let comp_logs = compress(&enc_logs, Compression::Zstd).expect("zstd compress logs pipeline");
    let comp_metrics =
        compress(&enc_metrics, Compression::Zstd).expect("zstd compress metrics pipeline");
    let comp_spans = compress(&enc_spans, Compression::Zstd).expect("zstd compress spans pipeline");

    let decomp_logs = decompress(&comp_logs).expect("zstd decompress logs pipeline");
    let decomp_metrics = decompress(&comp_metrics).expect("zstd decompress metrics pipeline");
    let decomp_spans = decompress(&comp_spans).expect("zstd decompress spans pipeline");

    let (dec_logs, _): (Vec<LogEntry>, usize) =
        decode_from_slice(&decomp_logs).expect("decode logs pipeline zstd");
    let (dec_metrics, _): (Vec<MetricPoint>, usize) =
        decode_from_slice(&decomp_metrics).expect("decode metrics pipeline zstd");
    let (dec_spans, _): (Vec<TraceSpan>, usize) =
        decode_from_slice(&decomp_spans).expect("decode spans pipeline zstd");

    assert_eq!(logs, dec_logs);
    assert_eq!(metrics, dec_metrics);
    assert_eq!(spans, dec_spans);
}
