//! Advanced async encoding tests (sixteenth set) for OxiCode.
//!
//! All 22 tests are top-level `#[tokio::test]` functions (no module wrapper).
//! Gated by the `async-tokio` feature at the file level.
//!
//! Types unique to this file: `Metric` and `MetricKind`.
//!
//! Coverage matrix:
//!   1:  Metric async roundtrip
//!   2:  MetricKind::Counter async roundtrip
//!   3:  MetricKind::Gauge async roundtrip
//!   4:  MetricKind::Histogram async roundtrip
//!   5:  MetricKind::Summary async roundtrip
//!   6:  Vec<Metric> 3 items async roundtrip
//!   7:  Vec<MetricKind> all variants async roundtrip
//!   8:  u32 async roundtrip
//!   9:  u64 async roundtrip
//!  10:  f64 async roundtrip (pi value, bit-exact)
//!  11:  String async roundtrip
//!  12:  bool async roundtrip (true and false)
//!  13:  Vec<u8> empty async roundtrip
//!  14:  Vec<u8> 512 bytes async roundtrip
//!  15:  Option<Metric> Some async roundtrip
//!  16:  Option<Metric> None async roundtrip
//!  17:  i64 negative async roundtrip
//!  18:  u128 large value async roundtrip
//!  19:  Sequential write 4 Metrics, read back 4
//!  20:  Option<Vec<Metric>> Some with 2 items async roundtrip
//!  21:  Tuple (u32, String) async roundtrip
//!  22:  Large Vec<f64> (100 items) async roundtrip

#![cfg(feature = "async-tokio")]
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
use oxicode::async_tokio::{AsyncDecoder, AsyncEncoder};
use oxicode::{Decode, Encode};
use std::io::Cursor;
use tokio::io::BufReader;

// ---------------------------------------------------------------------------
// Types unique to this file
// ---------------------------------------------------------------------------

#[derive(Debug, PartialEq, Encode, Decode)]
struct Metric {
    name: String,
    value: f64,
    labels: Vec<(String, String)>,
    timestamp_ms: u64,
}

#[derive(Debug, PartialEq, Encode, Decode)]
enum MetricKind {
    Counter(u64),
    Gauge(f64),
    Histogram { buckets: Vec<f64>, sum: f64 },
    Summary { quantiles: Vec<(f64, f64)> },
}

// ---------------------------------------------------------------------------
// Test 1: Metric async roundtrip
// ---------------------------------------------------------------------------

#[tokio::test]
async fn test_async16_metric_roundtrip() {
    let val = Metric {
        name: String::from("http_requests_total"),
        value: 1024.5,
        labels: vec![
            (String::from("method"), String::from("GET")),
            (String::from("status"), String::from("200")),
        ],
        timestamp_ms: 1_700_000_000_000,
    };

    let mut buf = Vec::<u8>::new();
    let mut enc = AsyncEncoder::new(&mut buf);
    enc.write_item(&val).await.expect("write Metric");
    enc.finish().await.expect("finish encoder");

    let cursor = Cursor::new(buf);
    let mut reader = BufReader::new(cursor);
    let mut dec = AsyncDecoder::new(&mut reader);
    let decoded: Metric = dec
        .read_item()
        .await
        .expect("read Metric no err")
        .expect("read Metric some value");

    assert_eq!(val, decoded, "Metric async roundtrip mismatch");
}

// ---------------------------------------------------------------------------
// Test 2: MetricKind::Counter async roundtrip
// ---------------------------------------------------------------------------

#[tokio::test]
async fn test_async16_metric_kind_counter_roundtrip() {
    let val = MetricKind::Counter(9_999_999_999_u64);

    let mut buf = Vec::<u8>::new();
    let mut enc = AsyncEncoder::new(&mut buf);
    enc.write_item(&val)
        .await
        .expect("write MetricKind::Counter");
    enc.finish().await.expect("finish encoder");

    let cursor = Cursor::new(buf);
    let mut reader = BufReader::new(cursor);
    let mut dec = AsyncDecoder::new(&mut reader);
    let decoded: MetricKind = dec
        .read_item()
        .await
        .expect("read MetricKind::Counter no err")
        .expect("read MetricKind::Counter some value");

    assert_eq!(val, decoded, "MetricKind::Counter async roundtrip mismatch");
}

// ---------------------------------------------------------------------------
// Test 3: MetricKind::Gauge async roundtrip
// ---------------------------------------------------------------------------

#[tokio::test]
async fn test_async16_metric_kind_gauge_roundtrip() {
    let val = MetricKind::Gauge(std::f64::consts::TAU);

    let mut buf = Vec::<u8>::new();
    let mut enc = AsyncEncoder::new(&mut buf);
    enc.write_item(&val).await.expect("write MetricKind::Gauge");
    enc.finish().await.expect("finish encoder");

    let cursor = Cursor::new(buf);
    let mut reader = BufReader::new(cursor);
    let mut dec = AsyncDecoder::new(&mut reader);
    let decoded: MetricKind = dec
        .read_item()
        .await
        .expect("read MetricKind::Gauge no err")
        .expect("read MetricKind::Gauge some value");

    assert_eq!(val, decoded, "MetricKind::Gauge async roundtrip mismatch");
}

// ---------------------------------------------------------------------------
// Test 4: MetricKind::Histogram async roundtrip
// ---------------------------------------------------------------------------

#[tokio::test]
async fn test_async16_metric_kind_histogram_roundtrip() {
    let val = MetricKind::Histogram {
        buckets: vec![
            0.005, 0.01, 0.025, 0.05, 0.1, 0.25, 0.5, 1.0, 2.5, 5.0, 10.0,
        ],
        sum: 42.75,
    };

    let mut buf = Vec::<u8>::new();
    let mut enc = AsyncEncoder::new(&mut buf);
    enc.write_item(&val)
        .await
        .expect("write MetricKind::Histogram");
    enc.finish().await.expect("finish encoder");

    let cursor = Cursor::new(buf);
    let mut reader = BufReader::new(cursor);
    let mut dec = AsyncDecoder::new(&mut reader);
    let decoded: MetricKind = dec
        .read_item()
        .await
        .expect("read MetricKind::Histogram no err")
        .expect("read MetricKind::Histogram some value");

    assert_eq!(
        val, decoded,
        "MetricKind::Histogram async roundtrip mismatch"
    );
}

// ---------------------------------------------------------------------------
// Test 5: MetricKind::Summary async roundtrip
// ---------------------------------------------------------------------------

#[tokio::test]
async fn test_async16_metric_kind_summary_roundtrip() {
    let val = MetricKind::Summary {
        quantiles: vec![(0.5, 0.023), (0.9, 0.046), (0.99, 0.1), (1.0, 0.25)],
    };

    let mut buf = Vec::<u8>::new();
    let mut enc = AsyncEncoder::new(&mut buf);
    enc.write_item(&val)
        .await
        .expect("write MetricKind::Summary");
    enc.finish().await.expect("finish encoder");

    let cursor = Cursor::new(buf);
    let mut reader = BufReader::new(cursor);
    let mut dec = AsyncDecoder::new(&mut reader);
    let decoded: MetricKind = dec
        .read_item()
        .await
        .expect("read MetricKind::Summary no err")
        .expect("read MetricKind::Summary some value");

    assert_eq!(val, decoded, "MetricKind::Summary async roundtrip mismatch");
}

// ---------------------------------------------------------------------------
// Test 6: Vec<Metric> 3 items async roundtrip
// ---------------------------------------------------------------------------

#[tokio::test]
async fn test_async16_vec_metric_three_items_roundtrip() {
    let val: Vec<Metric> = vec![
        Metric {
            name: String::from("cpu_usage"),
            value: 72.3,
            labels: vec![(String::from("core"), String::from("0"))],
            timestamp_ms: 1_700_000_001_000,
        },
        Metric {
            name: String::from("memory_bytes"),
            value: 4_294_967_296.0,
            labels: vec![
                (String::from("host"), String::from("node-01")),
                (String::from("type"), String::from("rss")),
            ],
            timestamp_ms: 1_700_000_002_000,
        },
        Metric {
            name: String::from("disk_io_ops"),
            value: 0.0,
            labels: vec![],
            timestamp_ms: 1_700_000_003_000,
        },
    ];

    let mut buf = Vec::<u8>::new();
    let mut enc = AsyncEncoder::new(&mut buf);
    enc.write_item(&val).await.expect("write Vec<Metric>");
    enc.finish().await.expect("finish encoder");

    let cursor = Cursor::new(buf);
    let mut reader = BufReader::new(cursor);
    let mut dec = AsyncDecoder::new(&mut reader);
    let decoded: Vec<Metric> = dec
        .read_item()
        .await
        .expect("read Vec<Metric> no err")
        .expect("read Vec<Metric> some value");

    assert_eq!(val, decoded, "Vec<Metric> 3-item async roundtrip mismatch");
}

// ---------------------------------------------------------------------------
// Test 7: Vec<MetricKind> all variants async roundtrip
// ---------------------------------------------------------------------------

#[tokio::test]
async fn test_async16_vec_metric_kind_all_variants_roundtrip() {
    let val: Vec<MetricKind> = vec![
        MetricKind::Counter(42),
        MetricKind::Gauge(std::f64::consts::E),
        MetricKind::Histogram {
            buckets: vec![0.1, 1.0, 10.0],
            sum: 11.1,
        },
        MetricKind::Summary {
            quantiles: vec![(0.5, 0.5), (0.95, 0.95)],
        },
    ];

    let mut buf = Vec::<u8>::new();
    let mut enc = AsyncEncoder::new(&mut buf);
    enc.write_item(&val).await.expect("write Vec<MetricKind>");
    enc.finish().await.expect("finish encoder");

    let cursor = Cursor::new(buf);
    let mut reader = BufReader::new(cursor);
    let mut dec = AsyncDecoder::new(&mut reader);
    let decoded: Vec<MetricKind> = dec
        .read_item()
        .await
        .expect("read Vec<MetricKind> no err")
        .expect("read Vec<MetricKind> some value");

    assert_eq!(
        val, decoded,
        "Vec<MetricKind> all-variants async roundtrip mismatch"
    );
}

// ---------------------------------------------------------------------------
// Test 8: u32 async roundtrip
// ---------------------------------------------------------------------------

#[tokio::test]
async fn test_async16_u32_roundtrip() {
    let val: u32 = 2_718_281_828;

    let mut buf = Vec::<u8>::new();
    let mut enc = AsyncEncoder::new(&mut buf);
    enc.write_item(&val).await.expect("write u32");
    enc.finish().await.expect("finish encoder");

    let cursor = Cursor::new(buf);
    let mut reader = BufReader::new(cursor);
    let mut dec = AsyncDecoder::new(&mut reader);
    let decoded: u32 = dec
        .read_item()
        .await
        .expect("read u32 no err")
        .expect("read u32 some value");

    assert_eq!(val, decoded, "u32 async roundtrip mismatch");
}

// ---------------------------------------------------------------------------
// Test 9: u64 async roundtrip
// ---------------------------------------------------------------------------

#[tokio::test]
async fn test_async16_u64_roundtrip() {
    let val: u64 = 9_223_372_036_854_775_807;

    let mut buf = Vec::<u8>::new();
    let mut enc = AsyncEncoder::new(&mut buf);
    enc.write_item(&val).await.expect("write u64");
    enc.finish().await.expect("finish encoder");

    let cursor = Cursor::new(buf);
    let mut reader = BufReader::new(cursor);
    let mut dec = AsyncDecoder::new(&mut reader);
    let decoded: u64 = dec
        .read_item()
        .await
        .expect("read u64 no err")
        .expect("read u64 some value");

    assert_eq!(val, decoded, "u64 async roundtrip mismatch");
}

// ---------------------------------------------------------------------------
// Test 10: f64 async roundtrip (pi value, bit-exact)
// ---------------------------------------------------------------------------

#[tokio::test]
async fn test_async16_f64_pi_bit_exact_roundtrip() {
    let val: f64 = std::f64::consts::PI;

    let mut buf = Vec::<u8>::new();
    let mut enc = AsyncEncoder::new(&mut buf);
    enc.write_item(&val).await.expect("write f64 pi");
    enc.finish().await.expect("finish encoder");

    let cursor = Cursor::new(buf);
    let mut reader = BufReader::new(cursor);
    let mut dec = AsyncDecoder::new(&mut reader);
    let decoded: f64 = dec
        .read_item()
        .await
        .expect("read f64 pi no err")
        .expect("read f64 pi some value");

    assert_eq!(
        val.to_bits(),
        decoded.to_bits(),
        "f64 pi async roundtrip bit mismatch"
    );
}

// ---------------------------------------------------------------------------
// Test 11: String async roundtrip
// ---------------------------------------------------------------------------

#[tokio::test]
async fn test_async16_string_roundtrip() {
    let val = String::from("metric-collector-v2.oxicode-async-wave16");

    let mut buf = Vec::<u8>::new();
    let mut enc = AsyncEncoder::new(&mut buf);
    enc.write_item(&val).await.expect("write String");
    enc.finish().await.expect("finish encoder");

    let cursor = Cursor::new(buf);
    let mut reader = BufReader::new(cursor);
    let mut dec = AsyncDecoder::new(&mut reader);
    let decoded: String = dec
        .read_item()
        .await
        .expect("read String no err")
        .expect("read String some value");

    assert_eq!(val, decoded, "String async roundtrip mismatch");
}

// ---------------------------------------------------------------------------
// Test 12: bool async roundtrip (true and false)
// ---------------------------------------------------------------------------

#[tokio::test]
async fn test_async16_bool_true_and_false_roundtrip() {
    for &original in &[true, false] {
        let mut buf = Vec::<u8>::new();
        let mut enc = AsyncEncoder::new(&mut buf);
        enc.write_item(&original).await.expect("write bool");
        enc.finish().await.expect("finish encoder");

        let cursor = Cursor::new(buf);
        let mut reader = BufReader::new(cursor);
        let mut dec = AsyncDecoder::new(&mut reader);
        let decoded: bool = dec
            .read_item()
            .await
            .expect("read bool no err")
            .expect("read bool some value");

        assert_eq!(
            original, decoded,
            "bool {} async roundtrip mismatch",
            original
        );
    }
}

// ---------------------------------------------------------------------------
// Test 13: Vec<u8> empty async roundtrip
// ---------------------------------------------------------------------------

#[tokio::test]
async fn test_async16_vec_u8_empty_roundtrip() {
    let val: Vec<u8> = Vec::new();

    let mut buf = Vec::<u8>::new();
    let mut enc = AsyncEncoder::new(&mut buf);
    enc.write_item(&val).await.expect("write empty Vec<u8>");
    enc.finish().await.expect("finish encoder");

    let cursor = Cursor::new(buf);
    let mut reader = BufReader::new(cursor);
    let mut dec = AsyncDecoder::new(&mut reader);
    let decoded: Vec<u8> = dec
        .read_item()
        .await
        .expect("read empty Vec<u8> no err")
        .expect("read empty Vec<u8> some value");

    assert_eq!(val, decoded, "empty Vec<u8> async roundtrip mismatch");
    assert!(decoded.is_empty(), "decoded Vec<u8> must be empty");
}

// ---------------------------------------------------------------------------
// Test 14: Vec<u8> 512 bytes async roundtrip
// ---------------------------------------------------------------------------

#[tokio::test]
async fn test_async16_vec_u8_512_bytes_roundtrip() {
    let val: Vec<u8> = (0u16..512).map(|i| (i % 256) as u8).collect();
    assert_eq!(val.len(), 512, "test data must be exactly 512 bytes");

    let mut buf = Vec::<u8>::new();
    let mut enc = AsyncEncoder::new(&mut buf);
    enc.write_item(&val).await.expect("write 512-byte Vec<u8>");
    enc.finish().await.expect("finish encoder");

    let cursor = Cursor::new(buf);
    let mut reader = BufReader::new(cursor);
    let mut dec = AsyncDecoder::new(&mut reader);
    let decoded: Vec<u8> = dec
        .read_item()
        .await
        .expect("read 512-byte Vec<u8> no err")
        .expect("read 512-byte Vec<u8> some value");

    assert_eq!(val, decoded, "Vec<u8> 512-byte async roundtrip mismatch");
}

// ---------------------------------------------------------------------------
// Test 15: Option<Metric> Some async roundtrip
// ---------------------------------------------------------------------------

#[tokio::test]
async fn test_async16_option_metric_some_roundtrip() {
    let val: Option<Metric> = Some(Metric {
        name: String::from("network_bytes_sent"),
        value: 8_388_608.0,
        labels: vec![(String::from("interface"), String::from("eth0"))],
        timestamp_ms: 1_700_000_100_000,
    });

    let mut buf = Vec::<u8>::new();
    let mut enc = AsyncEncoder::new(&mut buf);
    enc.write_item(&val)
        .await
        .expect("write Option<Metric> Some");
    enc.finish().await.expect("finish encoder");

    let cursor = Cursor::new(buf);
    let mut reader = BufReader::new(cursor);
    let mut dec = AsyncDecoder::new(&mut reader);
    let decoded: Option<Metric> = dec
        .read_item()
        .await
        .expect("read Option<Metric> Some no err")
        .expect("read Option<Metric> Some some value");

    assert_eq!(val, decoded, "Option<Metric> Some async roundtrip mismatch");
}

// ---------------------------------------------------------------------------
// Test 16: Option<Metric> None async roundtrip
// ---------------------------------------------------------------------------

#[tokio::test]
async fn test_async16_option_metric_none_roundtrip() {
    let val: Option<Metric> = None;

    let mut buf = Vec::<u8>::new();
    let mut enc = AsyncEncoder::new(&mut buf);
    enc.write_item(&val)
        .await
        .expect("write Option<Metric> None");
    enc.finish().await.expect("finish encoder");

    let cursor = Cursor::new(buf);
    let mut reader = BufReader::new(cursor);
    let mut dec = AsyncDecoder::new(&mut reader);
    let decoded: Option<Metric> = dec
        .read_item()
        .await
        .expect("read Option<Metric> None no err")
        .expect("read Option<Metric> None some value");

    assert_eq!(val, decoded, "Option<Metric> None async roundtrip mismatch");
    assert!(decoded.is_none(), "decoded Option<Metric> must be None");
}

// ---------------------------------------------------------------------------
// Test 17: i64 negative async roundtrip
// ---------------------------------------------------------------------------

#[tokio::test]
async fn test_async16_i64_negative_roundtrip() {
    let val: i64 = -9_007_199_254_740_992_i64;

    let mut buf = Vec::<u8>::new();
    let mut enc = AsyncEncoder::new(&mut buf);
    enc.write_item(&val).await.expect("write i64 negative");
    enc.finish().await.expect("finish encoder");

    let cursor = Cursor::new(buf);
    let mut reader = BufReader::new(cursor);
    let mut dec = AsyncDecoder::new(&mut reader);
    let decoded: i64 = dec
        .read_item()
        .await
        .expect("read i64 negative no err")
        .expect("read i64 negative some value");

    assert_eq!(val, decoded, "i64 negative async roundtrip mismatch");
}

// ---------------------------------------------------------------------------
// Test 18: u128 large value async roundtrip
// ---------------------------------------------------------------------------

#[tokio::test]
async fn test_async16_u128_large_value_roundtrip() {
    let val: u128 = u128::MAX - 1_000_000_000_000_000_000_u128;

    let mut buf = Vec::<u8>::new();
    let mut enc = AsyncEncoder::new(&mut buf);
    enc.write_item(&val).await.expect("write u128 large");
    enc.finish().await.expect("finish encoder");

    let cursor = Cursor::new(buf);
    let mut reader = BufReader::new(cursor);
    let mut dec = AsyncDecoder::new(&mut reader);
    let decoded: u128 = dec
        .read_item()
        .await
        .expect("read u128 large no err")
        .expect("read u128 large some value");

    assert_eq!(val, decoded, "u128 large value async roundtrip mismatch");
}

// ---------------------------------------------------------------------------
// Test 19: Sequential write 4 Metrics, read back 4
// ---------------------------------------------------------------------------

#[tokio::test]
async fn test_async16_sequential_four_metrics_roundtrip() {
    let metrics = [
        Metric {
            name: String::from("latency_p50"),
            value: 12.0,
            labels: vec![(String::from("service"), String::from("api"))],
            timestamp_ms: 1_700_001_000_000,
        },
        Metric {
            name: String::from("latency_p95"),
            value: 48.7,
            labels: vec![(String::from("service"), String::from("api"))],
            timestamp_ms: 1_700_001_001_000,
        },
        Metric {
            name: String::from("latency_p99"),
            value: 120.3,
            labels: vec![(String::from("service"), String::from("api"))],
            timestamp_ms: 1_700_001_002_000,
        },
        Metric {
            name: String::from("latency_max"),
            value: 999.9,
            labels: vec![
                (String::from("service"), String::from("api")),
                (String::from("region"), String::from("eu-west")),
            ],
            timestamp_ms: 1_700_001_003_000,
        },
    ];

    let mut buf = Vec::<u8>::new();
    {
        let mut enc = AsyncEncoder::new(&mut buf);
        for m in &metrics {
            enc.write_item(m).await.expect("write Metric in sequence");
        }
        enc.finish().await.expect("finish encoder");
    }

    let cursor = Cursor::new(buf);
    let mut reader = BufReader::new(cursor);
    let mut dec = AsyncDecoder::new(&mut reader);

    for (idx, expected) in metrics.iter().enumerate() {
        let decoded: Metric = dec
            .read_item()
            .await
            .expect("read sequential Metric no err")
            .expect("read sequential Metric some value");
        assert_eq!(
            *expected, decoded,
            "sequential Metric at index {idx} mismatch"
        );
    }

    let eof: Option<Metric> = dec.read_item().await.expect("read after 4th Metric no err");
    assert_eq!(eof, None, "expected None after 4 sequential Metrics");
}

// ---------------------------------------------------------------------------
// Test 20: Option<Vec<Metric>> Some with 2 items async roundtrip
// ---------------------------------------------------------------------------

#[tokio::test]
async fn test_async16_option_vec_metric_some_two_items_roundtrip() {
    let val: Option<Vec<Metric>> = Some(vec![
        Metric {
            name: String::from("error_rate"),
            value: 0.0023,
            labels: vec![(String::from("endpoint"), String::from("/health"))],
            timestamp_ms: 1_700_002_000_000,
        },
        Metric {
            name: String::from("request_count"),
            value: 150_000.0,
            labels: vec![
                (String::from("endpoint"), String::from("/api/v1/data")),
                (String::from("method"), String::from("POST")),
            ],
            timestamp_ms: 1_700_002_001_000,
        },
    ]);

    let mut buf = Vec::<u8>::new();
    let mut enc = AsyncEncoder::new(&mut buf);
    enc.write_item(&val)
        .await
        .expect("write Option<Vec<Metric>> Some");
    enc.finish().await.expect("finish encoder");

    let cursor = Cursor::new(buf);
    let mut reader = BufReader::new(cursor);
    let mut dec = AsyncDecoder::new(&mut reader);
    let decoded: Option<Vec<Metric>> = dec
        .read_item()
        .await
        .expect("read Option<Vec<Metric>> Some no err")
        .expect("read Option<Vec<Metric>> Some some value");

    assert_eq!(
        val, decoded,
        "Option<Vec<Metric>> Some with 2 items async roundtrip mismatch"
    );
}

// ---------------------------------------------------------------------------
// Test 21: Tuple (u32, String) async roundtrip
// ---------------------------------------------------------------------------

#[tokio::test]
async fn test_async16_tuple_u32_string_roundtrip() {
    let val: (u32, String) = (65_535, String::from("metric-label-wave16"));

    let mut buf = Vec::<u8>::new();
    let mut enc = AsyncEncoder::new(&mut buf);
    enc.write_item(&val).await.expect("write (u32, String)");
    enc.finish().await.expect("finish encoder");

    let cursor = Cursor::new(buf);
    let mut reader = BufReader::new(cursor);
    let mut dec = AsyncDecoder::new(&mut reader);
    let decoded: (u32, String) = dec
        .read_item()
        .await
        .expect("read (u32, String) no err")
        .expect("read (u32, String) some value");

    assert_eq!(val, decoded, "(u32, String) async roundtrip mismatch");
}

// ---------------------------------------------------------------------------
// Test 22: Large Vec<f64> (100 items) async roundtrip
// ---------------------------------------------------------------------------

#[tokio::test]
async fn test_async16_large_vec_f64_100_items_roundtrip() {
    let val: Vec<f64> = (0..100)
        .map(|i| {
            let t = i as f64 * std::f64::consts::PI / 50.0;
            t.sin() * 1000.0 + (i as f64 * 0.1)
        })
        .collect();
    assert_eq!(val.len(), 100, "test data must be exactly 100 f64 values");

    let mut buf = Vec::<u8>::new();
    let mut enc = AsyncEncoder::new(&mut buf);
    enc.write_item(&val).await.expect("write large Vec<f64>");
    enc.finish().await.expect("finish encoder");

    let cursor = Cursor::new(buf);
    let mut reader = BufReader::new(cursor);
    let mut dec = AsyncDecoder::new(&mut reader);
    let decoded: Vec<f64> = dec
        .read_item()
        .await
        .expect("read large Vec<f64> no err")
        .expect("read large Vec<f64> some value");

    assert_eq!(val.len(), decoded.len(), "Vec<f64> length mismatch");
    for (idx, (a, b)) in val.iter().zip(decoded.iter()).enumerate() {
        assert_eq!(
            a.to_bits(),
            b.to_bits(),
            "Vec<f64> bit mismatch at index {idx}: {a} vs {b}"
        );
    }
}
