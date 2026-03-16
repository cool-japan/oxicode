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

#[derive(Debug, PartialEq, Encode, Decode)]
enum LogLevel {
    Trace,
    Debug,
    Info,
    Warn,
    Error,
    Fatal,
}

#[derive(Debug, PartialEq, Encode, Decode)]
struct LogEntry {
    timestamp: u64,
    level: LogLevel,
    service: String,
    message: String,
    trace_id: Option<u64>,
}

// --- Test 1: LogLevel::Trace roundtrip ---
#[test]
fn test_loglevel_trace_roundtrip() {
    let level = LogLevel::Trace;
    let encoded = encode_to_vec(&level).expect("encode LogLevel::Trace failed");
    let compressed = compress_lz4(&encoded).expect("compress LogLevel::Trace failed");
    let decompressed = decompress_lz4(&compressed).expect("decompress LogLevel::Trace failed");
    let (decoded, _): (LogLevel, usize) =
        decode_from_slice(&decompressed).expect("decode LogLevel::Trace failed");
    assert_eq!(level, decoded);
}

// --- Test 2: LogLevel::Debug roundtrip ---
#[test]
fn test_loglevel_debug_roundtrip() {
    let level = LogLevel::Debug;
    let encoded = encode_to_vec(&level).expect("encode LogLevel::Debug failed");
    let compressed = compress_lz4(&encoded).expect("compress LogLevel::Debug failed");
    let decompressed = decompress_lz4(&compressed).expect("decompress LogLevel::Debug failed");
    let (decoded, _): (LogLevel, usize) =
        decode_from_slice(&decompressed).expect("decode LogLevel::Debug failed");
    assert_eq!(level, decoded);
}

// --- Test 3: LogLevel::Info roundtrip ---
#[test]
fn test_loglevel_info_roundtrip() {
    let level = LogLevel::Info;
    let encoded = encode_to_vec(&level).expect("encode LogLevel::Info failed");
    let compressed = compress_lz4(&encoded).expect("compress LogLevel::Info failed");
    let decompressed = decompress_lz4(&compressed).expect("decompress LogLevel::Info failed");
    let (decoded, _): (LogLevel, usize) =
        decode_from_slice(&decompressed).expect("decode LogLevel::Info failed");
    assert_eq!(level, decoded);
}

// --- Test 4: LogLevel::Warn roundtrip ---
#[test]
fn test_loglevel_warn_roundtrip() {
    let level = LogLevel::Warn;
    let encoded = encode_to_vec(&level).expect("encode LogLevel::Warn failed");
    let compressed = compress_lz4(&encoded).expect("compress LogLevel::Warn failed");
    let decompressed = decompress_lz4(&compressed).expect("decompress LogLevel::Warn failed");
    let (decoded, _): (LogLevel, usize) =
        decode_from_slice(&decompressed).expect("decode LogLevel::Warn failed");
    assert_eq!(level, decoded);
}

// --- Test 5: LogLevel::Error roundtrip ---
#[test]
fn test_loglevel_error_roundtrip() {
    let level = LogLevel::Error;
    let encoded = encode_to_vec(&level).expect("encode LogLevel::Error failed");
    let compressed = compress_lz4(&encoded).expect("compress LogLevel::Error failed");
    let decompressed = decompress_lz4(&compressed).expect("decompress LogLevel::Error failed");
    let (decoded, _): (LogLevel, usize) =
        decode_from_slice(&decompressed).expect("decode LogLevel::Error failed");
    assert_eq!(level, decoded);
}

// --- Test 6: LogLevel::Fatal roundtrip ---
#[test]
fn test_loglevel_fatal_roundtrip() {
    let level = LogLevel::Fatal;
    let encoded = encode_to_vec(&level).expect("encode LogLevel::Fatal failed");
    let compressed = compress_lz4(&encoded).expect("compress LogLevel::Fatal failed");
    let decompressed = decompress_lz4(&compressed).expect("decompress LogLevel::Fatal failed");
    let (decoded, _): (LogLevel, usize) =
        decode_from_slice(&decompressed).expect("decode LogLevel::Fatal failed");
    assert_eq!(level, decoded);
}

// --- Test 7: LogEntry with Info level roundtrip ---
#[test]
fn test_logentry_info_level_roundtrip() {
    let entry = LogEntry {
        timestamp: 1_700_000_000,
        level: LogLevel::Info,
        service: "auth-service".to_string(),
        message: "User login successful".to_string(),
        trace_id: Some(0xABCDEF01),
    };
    let encoded = encode_to_vec(&entry).expect("encode LogEntry Info failed");
    let compressed = compress_lz4(&encoded).expect("compress LogEntry Info failed");
    let decompressed = decompress_lz4(&compressed).expect("decompress LogEntry Info failed");
    let (decoded, _): (LogEntry, usize) =
        decode_from_slice(&decompressed).expect("decode LogEntry Info failed");
    assert_eq!(entry, decoded);
}

// --- Test 8: LogEntry with Warn level roundtrip ---
#[test]
fn test_logentry_warn_level_roundtrip() {
    let entry = LogEntry {
        timestamp: 1_700_001_000,
        level: LogLevel::Warn,
        service: "payment-service".to_string(),
        message: "Retry limit approaching".to_string(),
        trace_id: Some(0x12345678),
    };
    let encoded = encode_to_vec(&entry).expect("encode LogEntry Warn failed");
    let compressed = compress_lz4(&encoded).expect("compress LogEntry Warn failed");
    let decompressed = decompress_lz4(&compressed).expect("decompress LogEntry Warn failed");
    let (decoded, _): (LogEntry, usize) =
        decode_from_slice(&decompressed).expect("decode LogEntry Warn failed");
    assert_eq!(entry, decoded);
}

// --- Test 9: LogEntry with Error level roundtrip ---
#[test]
fn test_logentry_error_level_roundtrip() {
    let entry = LogEntry {
        timestamp: 1_700_002_000,
        level: LogLevel::Error,
        service: "database-service".to_string(),
        message: "Connection pool exhausted".to_string(),
        trace_id: Some(0xDEADBEEF),
    };
    let encoded = encode_to_vec(&entry).expect("encode LogEntry Error failed");
    let compressed = compress_lz4(&encoded).expect("compress LogEntry Error failed");
    let decompressed = decompress_lz4(&compressed).expect("decompress LogEntry Error failed");
    let (decoded, _): (LogEntry, usize) =
        decode_from_slice(&decompressed).expect("decode LogEntry Error failed");
    assert_eq!(entry, decoded);
}

// --- Test 10: LogEntry with Trace level roundtrip ---
#[test]
fn test_logentry_trace_level_roundtrip() {
    let entry = LogEntry {
        timestamp: 1_700_003_000,
        level: LogLevel::Trace,
        service: "api-gateway".to_string(),
        message: "Request headers parsed".to_string(),
        trace_id: None,
    };
    let encoded = encode_to_vec(&entry).expect("encode LogEntry Trace failed");
    let compressed = compress_lz4(&encoded).expect("compress LogEntry Trace failed");
    let decompressed = decompress_lz4(&compressed).expect("decompress LogEntry Trace failed");
    let (decoded, _): (LogEntry, usize) =
        decode_from_slice(&decompressed).expect("decode LogEntry Trace failed");
    assert_eq!(entry, decoded);
}

// --- Test 11: LogEntry with None trace_id ---
#[test]
fn test_logentry_none_trace_id_roundtrip() {
    let entry = LogEntry {
        timestamp: 1_700_004_000,
        level: LogLevel::Debug,
        service: "cache-service".to_string(),
        message: "Cache miss for key user:42".to_string(),
        trace_id: None,
    };
    let encoded = encode_to_vec(&entry).expect("encode LogEntry None trace_id failed");
    let compressed = compress_lz4(&encoded).expect("compress LogEntry None trace_id failed");
    let decompressed =
        decompress_lz4(&compressed).expect("decompress LogEntry None trace_id failed");
    let (decoded, _): (LogEntry, usize) =
        decode_from_slice(&decompressed).expect("decode LogEntry None trace_id failed");
    assert_eq!(entry, decoded);
    assert!(
        decoded.trace_id.is_none(),
        "trace_id should be None after roundtrip"
    );
}

// --- Test 12: LogEntry with Some trace_id ---
#[test]
fn test_logentry_some_trace_id_roundtrip() {
    let entry = LogEntry {
        timestamp: 1_700_005_000,
        level: LogLevel::Info,
        service: "order-service".to_string(),
        message: "Order placed successfully".to_string(),
        trace_id: Some(u64::MAX),
    };
    let encoded = encode_to_vec(&entry).expect("encode LogEntry Some trace_id failed");
    let compressed = compress_lz4(&encoded).expect("compress LogEntry Some trace_id failed");
    let decompressed =
        decompress_lz4(&compressed).expect("decompress LogEntry Some trace_id failed");
    let (decoded, _): (LogEntry, usize) =
        decode_from_slice(&decompressed).expect("decode LogEntry Some trace_id failed");
    assert_eq!(entry, decoded);
    assert_eq!(
        decoded.trace_id,
        Some(u64::MAX),
        "trace_id u64::MAX should survive roundtrip"
    );
}

// --- Test 13: Vec<LogEntry> roundtrip with 10 entries ---
#[test]
fn test_vec_logentry_10_roundtrip() {
    let entries: Vec<LogEntry> = (0..10)
        .map(|i| LogEntry {
            timestamp: 1_700_010_000 + i as u64 * 100,
            level: match i % 6 {
                0 => LogLevel::Trace,
                1 => LogLevel::Debug,
                2 => LogLevel::Info,
                3 => LogLevel::Warn,
                4 => LogLevel::Error,
                _ => LogLevel::Fatal,
            },
            service: format!("service-{}", i),
            message: format!("Event {} occurred in subsystem {}", i, i * 7),
            trace_id: if i % 2 == 0 {
                Some(i as u64 * 0x1000)
            } else {
                None
            },
        })
        .collect();
    let encoded = encode_to_vec(&entries).expect("encode Vec<LogEntry> 10 failed");
    let compressed = compress_lz4(&encoded).expect("compress Vec<LogEntry> 10 failed");
    let decompressed = decompress_lz4(&compressed).expect("decompress Vec<LogEntry> 10 failed");
    let (decoded, _): (Vec<LogEntry>, usize) =
        decode_from_slice(&decompressed).expect("decode Vec<LogEntry> 10 failed");
    assert_eq!(entries, decoded);
}

// --- Test 14: Large batch (100 identical entries) compresses smaller ---
#[test]
fn test_large_batch_100_identical_entries_compresses_smaller() {
    let single = LogEntry {
        timestamp: 1_700_100_000,
        level: LogLevel::Info,
        service: "batch-processor".to_string(),
        message: "Batch item processed successfully".to_string(),
        trace_id: Some(0xCAFEBABE),
    };
    let entries: Vec<LogEntry> = (0..100)
        .map(|_| LogEntry {
            timestamp: single.timestamp,
            level: LogLevel::Info,
            service: single.service.clone(),
            message: single.message.clone(),
            trace_id: single.trace_id,
        })
        .collect();
    let encoded = encode_to_vec(&entries).expect("encode 100 identical entries failed");
    let compressed = compress_lz4(&encoded).expect("compress 100 identical entries failed");
    assert!(
        compressed.len() < encoded.len(),
        "100 identical log entries ({} bytes encoded) should compress to smaller size ({} bytes)",
        encoded.len(),
        compressed.len()
    );
}

// --- Test 15: Decompressed bytes match original encoded bytes (checksum test) ---
#[test]
fn test_decompressed_bytes_match_original_encoded() {
    let entry = LogEntry {
        timestamp: 1_700_200_000,
        level: LogLevel::Warn,
        service: "health-checker".to_string(),
        message: "Health check response time exceeded threshold".to_string(),
        trace_id: Some(0x0FEED0FF),
    };
    let original_encoded = encode_to_vec(&entry).expect("encode for checksum test failed");
    let compressed = compress_lz4(&original_encoded).expect("compress for checksum test failed");
    let decompressed = decompress_lz4(&compressed).expect("decompress for checksum test failed");
    assert_eq!(
        original_encoded, decompressed,
        "decompressed bytes must be byte-for-byte identical to original encoded bytes"
    );
}

// --- Test 16: Compress same data twice → identical output ---
#[test]
fn test_compress_same_data_twice_identical_output() {
    let entry = LogEntry {
        timestamp: 1_700_300_000,
        level: LogLevel::Debug,
        service: "config-service".to_string(),
        message: "Configuration reloaded from remote source".to_string(),
        trace_id: None,
    };
    let encoded = encode_to_vec(&entry).expect("encode for determinism test failed");
    let compressed_first =
        compress_lz4(&encoded).expect("first compress for determinism test failed");
    let compressed_second =
        compress_lz4(&encoded).expect("second compress for determinism test failed");
    assert_eq!(
        compressed_first, compressed_second,
        "compressing the same data twice must yield identical output"
    );
}

// --- Test 17: Compress empty vec ---
#[test]
fn test_compress_empty_vec_roundtrip() {
    let empty: Vec<LogEntry> = vec![];
    let encoded = encode_to_vec(&empty).expect("encode empty Vec<LogEntry> failed");
    let compressed = compress_lz4(&encoded).expect("compress empty vec failed");
    let decompressed = decompress_lz4(&compressed).expect("decompress empty vec failed");
    let (decoded, _): (Vec<LogEntry>, usize) =
        decode_from_slice(&decompressed).expect("decode empty vec failed");
    assert_eq!(
        empty, decoded,
        "empty Vec<LogEntry> must survive LZ4 roundtrip"
    );
}

// --- Test 18: Invalid data returns error from decompress ---
#[test]
fn test_invalid_data_returns_error_from_decompress() {
    let garbage: Vec<u8> = vec![0xFF, 0xFE, 0xFD, 0xFC, 0xFB, 0xFA, 0x00, 0x01, 0x02];
    let result = decompress_lz4(&garbage);
    assert!(
        result.is_err(),
        "decompress_lz4 must return an error when given invalid/garbage input"
    );
}

// --- Test 19: Large repetitive string compresses smaller ---
#[test]
fn test_large_repetitive_message_compresses_smaller() {
    let repetitive_message = "ERROR: disk quota exceeded for user id 9999; ".repeat(500);
    let entry = LogEntry {
        timestamp: 1_700_400_000,
        level: LogLevel::Error,
        service: "storage-service".to_string(),
        message: repetitive_message,
        trace_id: Some(0xBAADF00D),
    };
    let encoded = encode_to_vec(&entry).expect("encode large repetitive entry failed");
    let compressed = compress_lz4(&encoded).expect("compress large repetitive entry failed");
    assert!(
        compressed.len() < encoded.len(),
        "large repetitive log message ({} bytes) should compress to less ({} bytes)",
        encoded.len(),
        compressed.len()
    );
    let decompressed =
        decompress_lz4(&compressed).expect("decompress large repetitive entry failed");
    let (decoded, _): (LogEntry, usize) =
        decode_from_slice(&decompressed).expect("decode large repetitive entry failed");
    assert_eq!(entry, decoded);
}

// --- Test 20: Unicode messages roundtrip ---
#[test]
fn test_unicode_messages_roundtrip() {
    let entry = LogEntry {
        timestamp: 1_700_500_000,
        level: LogLevel::Info,
        service: "i18n-service".to_string(),
        message: "ユーザーログイン成功: 田中太郎 (🔐 2FA enabled) — résumé uploaded ✓".to_string(),
        trace_id: Some(0x00C0FFEE),
    };
    let encoded = encode_to_vec(&entry).expect("encode unicode entry failed");
    let compressed = compress_lz4(&encoded).expect("compress unicode entry failed");
    let decompressed = decompress_lz4(&compressed).expect("decompress unicode entry failed");
    let (decoded, _): (LogEntry, usize) =
        decode_from_slice(&decompressed).expect("decode unicode entry failed");
    assert_eq!(
        entry, decoded,
        "unicode log message must survive LZ4 roundtrip"
    );
}

// --- Test 21: Multiple services batch roundtrip ---
#[test]
fn test_multiple_services_batch_roundtrip() {
    let services = [
        "auth-service",
        "payment-service",
        "notification-service",
        "analytics-service",
        "storage-service",
    ];
    let levels = [
        LogLevel::Trace,
        LogLevel::Debug,
        LogLevel::Info,
        LogLevel::Warn,
        LogLevel::Error,
    ];
    let entries: Vec<LogEntry> = services
        .iter()
        .zip(levels.iter())
        .enumerate()
        .map(|(i, (svc, _lvl))| LogEntry {
            timestamp: 1_700_600_000 + i as u64 * 1000,
            level: match i % 5 {
                0 => LogLevel::Trace,
                1 => LogLevel::Debug,
                2 => LogLevel::Info,
                3 => LogLevel::Warn,
                _ => LogLevel::Error,
            },
            service: svc.to_string(),
            message: format!("Service {} reported status update at index {}", svc, i),
            trace_id: Some(0xACE00000 + i as u64),
        })
        .collect();
    let encoded = encode_to_vec(&entries).expect("encode multi-service batch failed");
    let compressed = compress_lz4(&encoded).expect("compress multi-service batch failed");
    let decompressed = decompress_lz4(&compressed).expect("decompress multi-service batch failed");
    let (decoded, _): (Vec<LogEntry>, usize) =
        decode_from_slice(&decompressed).expect("decode multi-service batch failed");
    assert_eq!(entries, decoded);
    assert_eq!(decoded.len(), 5, "batch must contain exactly 5 entries");
}

// --- Test 22: Fatal log entry roundtrip ---
#[test]
fn test_fatal_log_entry_roundtrip() {
    let entry = LogEntry {
        timestamp: u64::MAX - 1,
        level: LogLevel::Fatal,
        service: "core-kernel".to_string(),
        message: "FATAL: unrecoverable state detected — initiating emergency shutdown sequence"
            .to_string(),
        trace_id: Some(0xFFFFFFFF_FFFFFFFF),
    };
    let encoded = encode_to_vec(&entry).expect("encode Fatal entry failed");
    let compressed = compress_lz4(&encoded).expect("compress Fatal entry failed");
    let decompressed = decompress_lz4(&compressed).expect("decompress Fatal entry failed");
    let (decoded, _): (LogEntry, usize) =
        decode_from_slice(&decompressed).expect("decode Fatal entry failed");
    assert_eq!(
        entry, decoded,
        "Fatal log entry must survive LZ4 roundtrip intact"
    );
    assert_eq!(
        decoded.level,
        LogLevel::Fatal,
        "log level must be Fatal after roundtrip"
    );
    assert_eq!(
        decoded.timestamp,
        u64::MAX - 1,
        "extreme timestamp must be preserved"
    );
}
