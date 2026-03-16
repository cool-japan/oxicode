#![cfg(feature = "compression-lz4")]
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

fn compress_lz4(data: &[u8]) -> Result<Vec<u8>, Box<dyn std::error::Error>> {
    compress(data, Compression::Lz4).map_err(|e| Box::new(e) as Box<dyn std::error::Error>)
}

fn decompress_lz4(data: &[u8]) -> Result<Vec<u8>, Box<dyn std::error::Error>> {
    decompress(data).map_err(|e| Box::new(e) as Box<dyn std::error::Error>)
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct TraceSpan {
    trace_id: u64,
    span_id: u32,
    name: String,
    duration_us: u64,
    tags: Vec<(String, String)>,
}

#[derive(Debug, PartialEq, Encode, Decode)]
enum SpanStatus {
    Ok,
    Error(String),
    Timeout { limit_ms: u32 },
}

fn make_trace_span(trace_id: u64, span_id: u32, name: &str, duration_us: u64) -> TraceSpan {
    TraceSpan {
        trace_id,
        span_id,
        name: name.to_string(),
        duration_us,
        tags: vec![
            ("service".to_string(), "api".to_string()),
            ("env".to_string(), "prod".to_string()),
        ],
    }
}

// Test 1: TraceSpan roundtrip via LZ4
#[test]
fn test_trace_span_lz4_roundtrip() {
    let span = make_trace_span(1234567890, 42, "http.request", 1500);
    let encoded = encode_to_vec(&span).expect("encode TraceSpan");
    let compressed = compress_lz4(&encoded).expect("compress TraceSpan");
    let decompressed = decompress_lz4(&compressed).expect("decompress TraceSpan");
    let (decoded, _): (TraceSpan, usize) =
        decode_from_slice(&decompressed).expect("decode TraceSpan");
    assert_eq!(span, decoded);
}

// Test 2: SpanStatus::Ok roundtrip via LZ4
#[test]
fn test_span_status_ok_lz4_roundtrip() {
    let status = SpanStatus::Ok;
    let encoded = encode_to_vec(&status).expect("encode SpanStatus::Ok");
    let compressed = compress_lz4(&encoded).expect("compress SpanStatus::Ok");
    let decompressed = decompress_lz4(&compressed).expect("decompress SpanStatus::Ok");
    let (decoded, _): (SpanStatus, usize) =
        decode_from_slice(&decompressed).expect("decode SpanStatus::Ok");
    assert_eq!(status, decoded);
}

// Test 3: SpanStatus::Error roundtrip via LZ4
#[test]
fn test_span_status_error_lz4_roundtrip() {
    let status = SpanStatus::Error("connection refused".to_string());
    let encoded = encode_to_vec(&status).expect("encode SpanStatus::Error");
    let compressed = compress_lz4(&encoded).expect("compress SpanStatus::Error");
    let decompressed = decompress_lz4(&compressed).expect("decompress SpanStatus::Error");
    let (decoded, _): (SpanStatus, usize) =
        decode_from_slice(&decompressed).expect("decode SpanStatus::Error");
    assert_eq!(status, decoded);
}

// Test 4: SpanStatus::Timeout roundtrip via LZ4
#[test]
fn test_span_status_timeout_lz4_roundtrip() {
    let status = SpanStatus::Timeout { limit_ms: 3000 };
    let encoded = encode_to_vec(&status).expect("encode SpanStatus::Timeout");
    let compressed = compress_lz4(&encoded).expect("compress SpanStatus::Timeout");
    let decompressed = decompress_lz4(&compressed).expect("decompress SpanStatus::Timeout");
    let (decoded, _): (SpanStatus, usize) =
        decode_from_slice(&decompressed).expect("decode SpanStatus::Timeout");
    assert_eq!(status, decoded);
}

// Test 5: Vec<TraceSpan> 5 items LZ4 roundtrip
#[test]
fn test_vec_trace_span_5_lz4_roundtrip() {
    let spans: Vec<TraceSpan> = (0..5)
        .map(|i| {
            make_trace_span(
                i as u64 * 1000,
                i as u32,
                &format!("span_{}", i),
                i as u64 * 100,
            )
        })
        .collect();
    let encoded = encode_to_vec(&spans).expect("encode Vec<TraceSpan> 5");
    let compressed = compress_lz4(&encoded).expect("compress Vec<TraceSpan> 5");
    let decompressed = decompress_lz4(&compressed).expect("decompress Vec<TraceSpan> 5");
    let (decoded, _): (Vec<TraceSpan>, usize) =
        decode_from_slice(&decompressed).expect("decode Vec<TraceSpan> 5");
    assert_eq!(spans, decoded);
}

// Test 6: Vec<SpanStatus> all 3 variants LZ4 roundtrip
#[test]
fn test_vec_span_status_all_variants_lz4_roundtrip() {
    let statuses = vec![
        SpanStatus::Ok,
        SpanStatus::Error("timeout waiting for upstream".to_string()),
        SpanStatus::Timeout { limit_ms: 500 },
    ];
    let encoded = encode_to_vec(&statuses).expect("encode Vec<SpanStatus>");
    let compressed = compress_lz4(&encoded).expect("compress Vec<SpanStatus>");
    let decompressed = decompress_lz4(&compressed).expect("decompress Vec<SpanStatus>");
    let (decoded, _): (Vec<SpanStatus>, usize) =
        decode_from_slice(&decompressed).expect("decode Vec<SpanStatus>");
    assert_eq!(statuses, decoded);
}

// Test 7: TraceSpan with empty tags LZ4 roundtrip
#[test]
fn test_trace_span_empty_tags_lz4_roundtrip() {
    let span = TraceSpan {
        trace_id: 999,
        span_id: 1,
        name: "empty.tags".to_string(),
        duration_us: 50,
        tags: vec![],
    };
    let encoded = encode_to_vec(&span).expect("encode TraceSpan empty tags");
    let compressed = compress_lz4(&encoded).expect("compress TraceSpan empty tags");
    let decompressed = decompress_lz4(&compressed).expect("decompress TraceSpan empty tags");
    let (decoded, _): (TraceSpan, usize) =
        decode_from_slice(&decompressed).expect("decode TraceSpan empty tags");
    assert_eq!(span, decoded);
}

// Test 8: TraceSpan with 10 tags LZ4 roundtrip
#[test]
fn test_trace_span_10_tags_lz4_roundtrip() {
    let tags: Vec<(String, String)> = (0..10)
        .map(|i| (format!("key_{}", i), format!("value_{}", i)))
        .collect();
    let span = TraceSpan {
        trace_id: 0xdeadbeef,
        span_id: 77,
        name: "db.query".to_string(),
        duration_us: 2500,
        tags,
    };
    let encoded = encode_to_vec(&span).expect("encode TraceSpan 10 tags");
    let compressed = compress_lz4(&encoded).expect("compress TraceSpan 10 tags");
    let decompressed = decompress_lz4(&compressed).expect("decompress TraceSpan 10 tags");
    let (decoded, _): (TraceSpan, usize) =
        decode_from_slice(&decompressed).expect("decode TraceSpan 10 tags");
    assert_eq!(span, decoded);
}

// Test 9: Large Vec of 100 TraceSpans LZ4 roundtrip
#[test]
fn test_large_vec_100_trace_spans_lz4_roundtrip() {
    let spans: Vec<TraceSpan> = (0..100)
        .map(|i| TraceSpan {
            trace_id: i as u64 * 0x10000,
            span_id: i as u32,
            name: format!("operation_{}", i),
            duration_us: i as u64 * 10 + 1,
            tags: vec![
                ("index".to_string(), i.to_string()),
                ("batch".to_string(), "large".to_string()),
            ],
        })
        .collect();
    let encoded = encode_to_vec(&spans).expect("encode 100 TraceSpans");
    let compressed = compress_lz4(&encoded).expect("compress 100 TraceSpans");
    let decompressed = decompress_lz4(&compressed).expect("decompress 100 TraceSpans");
    let (decoded, _): (Vec<TraceSpan>, usize) =
        decode_from_slice(&decompressed).expect("decode 100 TraceSpans");
    assert_eq!(spans, decoded);
}

// Test 10: u32 LZ4 roundtrip
#[test]
fn test_u32_lz4_roundtrip() {
    let val: u32 = 0xCAFEBABE;
    let encoded = encode_to_vec(&val).expect("encode u32");
    let compressed = compress_lz4(&encoded).expect("compress u32");
    let decompressed = decompress_lz4(&compressed).expect("decompress u32");
    let (decoded, _): (u32, usize) = decode_from_slice(&decompressed).expect("decode u32");
    assert_eq!(val, decoded);
}

// Test 11: String LZ4 roundtrip
#[test]
fn test_string_lz4_roundtrip() {
    let val = "Hello, OxiCode LZ4 compression test!".to_string();
    let encoded = encode_to_vec(&val).expect("encode String");
    let compressed = compress_lz4(&encoded).expect("compress String");
    let decompressed = decompress_lz4(&compressed).expect("decompress String");
    let (decoded, _): (String, usize) = decode_from_slice(&decompressed).expect("decode String");
    assert_eq!(val, decoded);
}

// Test 12: Vec<u8> LZ4 roundtrip
#[test]
fn test_vec_u8_lz4_roundtrip() {
    let val: Vec<u8> = (0u8..=255u8).collect();
    let encoded = encode_to_vec(&val).expect("encode Vec<u8>");
    let compressed = compress_lz4(&encoded).expect("compress Vec<u8>");
    let decompressed = decompress_lz4(&compressed).expect("decompress Vec<u8>");
    let (decoded, _): (Vec<u8>, usize) = decode_from_slice(&decompressed).expect("decode Vec<u8>");
    assert_eq!(val, decoded);
}

// Test 13: Large repetitive Vec<TraceSpan> (50 identical spans) compresses smaller
#[test]
fn test_repetitive_spans_lz4_compresses_smaller() {
    let span = make_trace_span(42, 7, "repeated.operation", 999);
    let spans: Vec<TraceSpan> = std::iter::repeat(span).take(50).collect();
    let encoded = encode_to_vec(&spans).expect("encode 50 identical TraceSpans");
    let compressed = compress_lz4(&encoded).expect("compress 50 identical TraceSpans");
    assert!(
        compressed.len() < encoded.len(),
        "LZ4 should compress repetitive data: compressed={} vs uncompressed={}",
        compressed.len(),
        encoded.len()
    );
}

// Test 14: LZ4 compressed output differs from uncompressed for TraceSpan
#[test]
fn test_lz4_output_differs_from_uncompressed() {
    let span = make_trace_span(111, 22, "check.compression", 333);
    let encoded = encode_to_vec(&span).expect("encode TraceSpan for diff check");
    let compressed = compress_lz4(&encoded).expect("compress TraceSpan for diff check");
    assert_ne!(
        encoded, compressed,
        "Compressed bytes must differ from raw encoded bytes"
    );
}

// Test 15: Decompress bad data returns error
#[test]
fn test_decompress_bad_data_returns_error() {
    let garbage: Vec<u8> = vec![0xFF, 0xFE, 0xFD, 0x00, 0x01, 0x02, 0x03, 0x04];
    let result = decompress_lz4(&garbage);
    assert!(
        result.is_err(),
        "Decompressing garbage should return an error"
    );
}

// Test 16: Compress same TraceSpan twice - identical output
#[test]
fn test_compress_same_span_twice_identical_output() {
    let span = make_trace_span(777, 88, "idempotent.check", 12345);
    let encoded = encode_to_vec(&span).expect("encode TraceSpan for idempotent check");
    let compressed1 = compress_lz4(&encoded).expect("first compress");
    let compressed2 = compress_lz4(&encoded).expect("second compress");
    assert_eq!(
        compressed1, compressed2,
        "Compressing the same data twice must produce identical output"
    );
}

// Test 17: Empty Vec<TraceSpan> LZ4 roundtrip
#[test]
fn test_empty_vec_trace_span_lz4_roundtrip() {
    let spans: Vec<TraceSpan> = vec![];
    let encoded = encode_to_vec(&spans).expect("encode empty Vec<TraceSpan>");
    let compressed = compress_lz4(&encoded).expect("compress empty Vec<TraceSpan>");
    let decompressed = decompress_lz4(&compressed).expect("decompress empty Vec<TraceSpan>");
    let (decoded, _): (Vec<TraceSpan>, usize) =
        decode_from_slice(&decompressed).expect("decode empty Vec<TraceSpan>");
    assert_eq!(spans, decoded);
    assert!(decoded.is_empty());
}

// Test 18: Option<TraceSpan> Some LZ4 roundtrip
#[test]
fn test_option_trace_span_some_lz4_roundtrip() {
    let span = Some(make_trace_span(555, 66, "option.some", 8888));
    let encoded = encode_to_vec(&span).expect("encode Option<TraceSpan> Some");
    let compressed = compress_lz4(&encoded).expect("compress Option<TraceSpan> Some");
    let decompressed = decompress_lz4(&compressed).expect("decompress Option<TraceSpan> Some");
    let (decoded, _): (Option<TraceSpan>, usize) =
        decode_from_slice(&decompressed).expect("decode Option<TraceSpan> Some");
    assert_eq!(span, decoded);
}

// Test 19: Option<TraceSpan> None LZ4 roundtrip
#[test]
fn test_option_trace_span_none_lz4_roundtrip() {
    let span: Option<TraceSpan> = None;
    let encoded = encode_to_vec(&span).expect("encode Option<TraceSpan> None");
    let compressed = compress_lz4(&encoded).expect("compress Option<TraceSpan> None");
    let decompressed = decompress_lz4(&compressed).expect("decompress Option<TraceSpan> None");
    let (decoded, _): (Option<TraceSpan>, usize) =
        decode_from_slice(&decompressed).expect("decode Option<TraceSpan> None");
    assert_eq!(span, decoded);
    assert!(decoded.is_none());
}

// Test 20: Decompressed bytes match original encoded bytes
#[test]
fn test_decompressed_bytes_match_encoded() {
    let span = make_trace_span(321, 12, "byte.integrity", 4567);
    let original_encoded = encode_to_vec(&span).expect("encode for byte integrity check");
    let compressed = compress_lz4(&original_encoded).expect("compress for byte integrity check");
    let decompressed = decompress_lz4(&compressed).expect("decompress for byte integrity check");
    assert_eq!(
        original_encoded, decompressed,
        "Decompressed bytes must exactly match the original encoded bytes"
    );
}

// Test 21: LCG random Vec<u64> (100 items) LZ4 roundtrip
#[test]
fn test_lcg_random_vec_u64_lz4_roundtrip() {
    // Linear congruential generator: no external crate needed
    let mut state: u64 = 0xACE1ACE1ACE1ACE1;
    let data: Vec<u64> = (0..100)
        .map(|_| {
            state = state
                .wrapping_mul(6364136223846793005)
                .wrapping_add(1442695040888963407);
            state
        })
        .collect();
    let encoded = encode_to_vec(&data).expect("encode LCG Vec<u64>");
    let compressed = compress_lz4(&encoded).expect("compress LCG Vec<u64>");
    let decompressed = decompress_lz4(&compressed).expect("decompress LCG Vec<u64>");
    let (decoded, _): (Vec<u64>, usize) =
        decode_from_slice(&decompressed).expect("decode LCG Vec<u64>");
    assert_eq!(data, decoded);
}

// Test 22: Vec<(u64, SpanStatus)> LZ4 roundtrip
#[test]
fn test_vec_tuple_u64_span_status_lz4_roundtrip() {
    let pairs: Vec<(u64, SpanStatus)> = vec![
        (1000, SpanStatus::Ok),
        (2000, SpanStatus::Error("upstream failure".to_string())),
        (3000, SpanStatus::Timeout { limit_ms: 1000 }),
        (4000, SpanStatus::Ok),
        (5000, SpanStatus::Error("disk full".to_string())),
        (6000, SpanStatus::Timeout { limit_ms: 250 }),
    ];
    let encoded = encode_to_vec(&pairs).expect("encode Vec<(u64, SpanStatus)>");
    let compressed = compress_lz4(&encoded).expect("compress Vec<(u64, SpanStatus)>");
    let decompressed = decompress_lz4(&compressed).expect("decompress Vec<(u64, SpanStatus)>");
    let (decoded, _): (Vec<(u64, SpanStatus)>, usize) =
        decode_from_slice(&decompressed).expect("decode Vec<(u64, SpanStatus)>");
    assert_eq!(pairs, decoded);
}
