#![cfg(feature = "compression-zstd")]
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

#[derive(Debug, PartialEq, Encode, Decode)]
struct MetricsSnapshot {
    timestamp_ms: u64,
    cpu_pct: u8,
    mem_mb: u32,
    disk_io_kbps: u32,
    labels: Vec<String>,
}

#[derive(Debug, PartialEq, Encode, Decode)]
enum AlertLevel {
    Info,
    Warning(String),
    Critical { code: u32, msg: String },
}

fn compress_zstd(data: &[u8]) -> Result<Vec<u8>, Box<dyn std::error::Error>> {
    compress(data, Compression::Zstd).map_err(|e| Box::new(e) as Box<dyn std::error::Error>)
}

fn decompress_zstd(data: &[u8]) -> Result<Vec<u8>, Box<dyn std::error::Error>> {
    decompress(data).map_err(|e| Box::new(e) as Box<dyn std::error::Error>)
}

// Test 1: MetricsSnapshot roundtrip with minimal fields
#[test]
fn test_metrics_snapshot_roundtrip_minimal() {
    let snap = MetricsSnapshot {
        timestamp_ms: 0,
        cpu_pct: 0,
        mem_mb: 0,
        disk_io_kbps: 0,
        labels: vec![],
    };
    let encoded = encode_to_vec(&snap).expect("encode MetricsSnapshot minimal");
    let compressed = compress_zstd(&encoded).expect("compress MetricsSnapshot minimal");
    let decompressed = decompress_zstd(&compressed).expect("decompress MetricsSnapshot minimal");
    let (decoded, _): (MetricsSnapshot, _) =
        decode_from_slice(&decompressed).expect("decode MetricsSnapshot minimal");
    assert_eq!(snap, decoded);
}

// Test 2: MetricsSnapshot roundtrip with typical values
#[test]
fn test_metrics_snapshot_roundtrip_typical() {
    let snap = MetricsSnapshot {
        timestamp_ms: 1_700_000_000_000,
        cpu_pct: 73,
        mem_mb: 4096,
        disk_io_kbps: 12345,
        labels: vec!["host:web01".to_string(), "env:prod".to_string()],
    };
    let encoded = encode_to_vec(&snap).expect("encode MetricsSnapshot typical");
    let compressed = compress_zstd(&encoded).expect("compress MetricsSnapshot typical");
    let decompressed = decompress_zstd(&compressed).expect("decompress MetricsSnapshot typical");
    let (decoded, _): (MetricsSnapshot, _) =
        decode_from_slice(&decompressed).expect("decode MetricsSnapshot typical");
    assert_eq!(snap, decoded);
}

// Test 3: MetricsSnapshot roundtrip with many labels
#[test]
fn test_metrics_snapshot_roundtrip_many_labels() {
    let labels: Vec<String> = (0..50)
        .map(|i| format!("label_key_{}:value_{}", i, i * 7))
        .collect();
    let snap = MetricsSnapshot {
        timestamp_ms: u64::MAX / 2,
        cpu_pct: 100,
        mem_mb: u32::MAX,
        disk_io_kbps: 999_999,
        labels,
    };
    let encoded = encode_to_vec(&snap).expect("encode MetricsSnapshot many labels");
    let compressed = compress_zstd(&encoded).expect("compress MetricsSnapshot many labels");
    let decompressed =
        decompress_zstd(&compressed).expect("decompress MetricsSnapshot many labels");
    let (decoded, _): (MetricsSnapshot, _) =
        decode_from_slice(&decompressed).expect("decode MetricsSnapshot many labels");
    assert_eq!(snap, decoded);
}

// Test 4: AlertLevel::Info roundtrip
#[test]
fn test_alert_level_info_roundtrip() {
    let alert = AlertLevel::Info;
    let encoded = encode_to_vec(&alert).expect("encode AlertLevel::Info");
    let compressed = compress_zstd(&encoded).expect("compress AlertLevel::Info");
    let decompressed = decompress_zstd(&compressed).expect("decompress AlertLevel::Info");
    let (decoded, _): (AlertLevel, _) =
        decode_from_slice(&decompressed).expect("decode AlertLevel::Info");
    assert_eq!(alert, decoded);
}

// Test 5: AlertLevel::Warning roundtrip
#[test]
fn test_alert_level_warning_roundtrip() {
    let alert = AlertLevel::Warning("disk usage above 90%".to_string());
    let encoded = encode_to_vec(&alert).expect("encode AlertLevel::Warning");
    let compressed = compress_zstd(&encoded).expect("compress AlertLevel::Warning");
    let decompressed = decompress_zstd(&compressed).expect("decompress AlertLevel::Warning");
    let (decoded, _): (AlertLevel, _) =
        decode_from_slice(&decompressed).expect("decode AlertLevel::Warning");
    assert_eq!(alert, decoded);
}

// Test 6: AlertLevel::Critical roundtrip
#[test]
fn test_alert_level_critical_roundtrip() {
    let alert = AlertLevel::Critical {
        code: 500,
        msg: "service unavailable: connection pool exhausted".to_string(),
    };
    let encoded = encode_to_vec(&alert).expect("encode AlertLevel::Critical");
    let compressed = compress_zstd(&encoded).expect("compress AlertLevel::Critical");
    let decompressed = decompress_zstd(&compressed).expect("decompress AlertLevel::Critical");
    let (decoded, _): (AlertLevel, _) =
        decode_from_slice(&decompressed).expect("decode AlertLevel::Critical");
    assert_eq!(alert, decoded);
}

// Test 7: u32 roundtrip
#[test]
fn test_u32_roundtrip() {
    let value: u32 = 3_141_592_653;
    let encoded = encode_to_vec(&value).expect("encode u32");
    let compressed = compress_zstd(&encoded).expect("compress u32");
    let decompressed = decompress_zstd(&compressed).expect("decompress u32");
    let (decoded, _): (u32, _) = decode_from_slice(&decompressed).expect("decode u32");
    assert_eq!(value, decoded);
}

// Test 8: String roundtrip
#[test]
fn test_string_roundtrip() {
    let value =
        "The quick brown fox jumps over the lazy dog. 素早い茶色のキツネが怠け者の犬を飛び越えた。"
            .to_string();
    let encoded = encode_to_vec(&value).expect("encode String");
    let compressed = compress_zstd(&encoded).expect("compress String");
    let decompressed = decompress_zstd(&compressed).expect("decompress String");
    let (decoded, _): (String, _) = decode_from_slice(&decompressed).expect("decode String");
    assert_eq!(value, decoded);
}

// Test 9: Vec<u8> roundtrip
#[test]
fn test_vec_u8_roundtrip() {
    let value: Vec<u8> = (0u8..=255).cycle().take(512).collect();
    let encoded = encode_to_vec(&value).expect("encode Vec<u8>");
    let compressed = compress_zstd(&encoded).expect("compress Vec<u8>");
    let decompressed = decompress_zstd(&compressed).expect("decompress Vec<u8>");
    let (decoded, _): (Vec<u8>, _) = decode_from_slice(&decompressed).expect("decode Vec<u8>");
    assert_eq!(value, decoded);
}

// Test 10: Large repetitive vec compresses smaller than original
#[test]
fn test_large_repetitive_compresses_smaller() {
    let repetitive: Vec<u8> = b"AAAAAAAAAA"
        .iter()
        .cycle()
        .take(100_000)
        .copied()
        .collect();
    let encoded = encode_to_vec(&repetitive).expect("encode repetitive Vec<u8>");
    let compressed = compress_zstd(&encoded).expect("compress repetitive Vec<u8>");
    assert!(
        compressed.len() < encoded.len(),
        "compressed ({} bytes) should be smaller than encoded ({} bytes)",
        compressed.len(),
        encoded.len()
    );
}

// Test 11: Compressed bytes differ from uncompressed encoded bytes
#[test]
fn test_compressed_differs_from_uncompressed() {
    let snap = MetricsSnapshot {
        timestamp_ms: 42,
        cpu_pct: 10,
        mem_mb: 1024,
        disk_io_kbps: 500,
        labels: vec!["region:us-east-1".to_string()],
    };
    let encoded = encode_to_vec(&snap).expect("encode for diff check");
    let compressed = compress_zstd(&encoded).expect("compress for diff check");
    assert_ne!(
        encoded, compressed,
        "compressed bytes must differ from raw encoded bytes"
    );
}

// Test 12: Same data compresses identically (deterministic)
#[test]
fn test_same_data_compresses_identically() {
    let snap = MetricsSnapshot {
        timestamp_ms: 1_620_000_000_000,
        cpu_pct: 55,
        mem_mb: 2048,
        disk_io_kbps: 8000,
        labels: vec!["svc:api".to_string(), "dc:eu-west".to_string()],
    };
    let encoded = encode_to_vec(&snap).expect("encode for determinism check");
    let compressed_a = compress_zstd(&encoded).expect("compress first time");
    let compressed_b = compress_zstd(&encoded).expect("compress second time");
    assert_eq!(
        compressed_a, compressed_b,
        "compression must be deterministic"
    );
}

// Test 13: Bad/random data decompresses to error
#[test]
fn test_bad_data_decompresses_to_error() {
    let garbage: Vec<u8> = (0u8..=255).cycle().take(256).collect();
    let result = decompress_zstd(&garbage);
    assert!(
        result.is_err(),
        "decompressing garbage data should return an error"
    );
}

// Test 14: Empty vec roundtrip
#[test]
fn test_empty_vec_roundtrip() {
    let value: Vec<u8> = vec![];
    let encoded = encode_to_vec(&value).expect("encode empty Vec<u8>");
    let compressed = compress_zstd(&encoded).expect("compress empty Vec<u8>");
    let decompressed = decompress_zstd(&compressed).expect("decompress empty Vec<u8>");
    let (decoded, _): (Vec<u8>, _) =
        decode_from_slice(&decompressed).expect("decode empty Vec<u8>");
    assert_eq!(value, decoded);
}

// Test 15: Vec<MetricsSnapshot> with 10 items roundtrip
#[test]
fn test_vec_metrics_snapshot_10_items_roundtrip() {
    let snapshots: Vec<MetricsSnapshot> = (0..10)
        .map(|i| MetricsSnapshot {
            timestamp_ms: 1_700_000_000_000 + i * 1000,
            cpu_pct: (i * 10) as u8,
            mem_mb: 512 + i as u32 * 128,
            disk_io_kbps: 100 * i as u32,
            labels: vec![format!("instance:server{}", i), format!("shard:{}", i % 3)],
        })
        .collect();
    let encoded = encode_to_vec(&snapshots).expect("encode Vec<MetricsSnapshot>");
    let compressed = compress_zstd(&encoded).expect("compress Vec<MetricsSnapshot>");
    let decompressed = decompress_zstd(&compressed).expect("decompress Vec<MetricsSnapshot>");
    let (decoded, _): (Vec<MetricsSnapshot>, _) =
        decode_from_slice(&decompressed).expect("decode Vec<MetricsSnapshot>");
    assert_eq!(snapshots, decoded);
}

// Test 16: Option<MetricsSnapshot> Some roundtrip
#[test]
fn test_option_metrics_snapshot_some_roundtrip() {
    let value: Option<MetricsSnapshot> = Some(MetricsSnapshot {
        timestamp_ms: 9_999_999_999_999,
        cpu_pct: 88,
        mem_mb: 16384,
        disk_io_kbps: 204800,
        labels: vec!["tier:premium".to_string()],
    });
    let encoded = encode_to_vec(&value).expect("encode Option<MetricsSnapshot> Some");
    let compressed = compress_zstd(&encoded).expect("compress Option<MetricsSnapshot> Some");
    let decompressed =
        decompress_zstd(&compressed).expect("decompress Option<MetricsSnapshot> Some");
    let (decoded, _): (Option<MetricsSnapshot>, _) =
        decode_from_slice(&decompressed).expect("decode Option<MetricsSnapshot> Some");
    assert_eq!(value, decoded);
}

// Test 17: Option<MetricsSnapshot> None roundtrip
#[test]
fn test_option_metrics_snapshot_none_roundtrip() {
    let value: Option<MetricsSnapshot> = None;
    let encoded = encode_to_vec(&value).expect("encode Option<MetricsSnapshot> None");
    let compressed = compress_zstd(&encoded).expect("compress Option<MetricsSnapshot> None");
    let decompressed =
        decompress_zstd(&compressed).expect("decompress Option<MetricsSnapshot> None");
    let (decoded, _): (Option<MetricsSnapshot>, _) =
        decode_from_slice(&decompressed).expect("decode Option<MetricsSnapshot> None");
    assert_eq!(value, decoded);
}

// Test 18: Decompressed bytes match original encoded bytes exactly
#[test]
fn test_decompressed_matches_original_encoded_bytes() {
    let snap = MetricsSnapshot {
        timestamp_ms: 1_234_567_890_123,
        cpu_pct: 42,
        mem_mb: 8192,
        disk_io_kbps: 51200,
        labels: vec!["check:byte-equality".to_string()],
    };
    let encoded = encode_to_vec(&snap).expect("encode for byte equality check");
    let compressed = compress_zstd(&encoded).expect("compress for byte equality check");
    let decompressed = decompress_zstd(&compressed).expect("decompress for byte equality check");
    assert_eq!(
        encoded, decompressed,
        "decompressed bytes must exactly match original encoded bytes"
    );
}

// Test 19: u128::MAX roundtrip
#[test]
fn test_u128_max_roundtrip() {
    let value: u128 = u128::MAX;
    let encoded = encode_to_vec(&value).expect("encode u128::MAX");
    let compressed = compress_zstd(&encoded).expect("compress u128::MAX");
    let decompressed = decompress_zstd(&compressed).expect("decompress u128::MAX");
    let (decoded, _): (u128, _) = decode_from_slice(&decompressed).expect("decode u128::MAX");
    assert_eq!(value, decoded);
}

// Test 20: Vec<AlertLevel> all variants roundtrip
#[test]
fn test_vec_alert_level_all_variants_roundtrip() {
    let alerts = vec![
        AlertLevel::Info,
        AlertLevel::Warning("cpu spike detected".to_string()),
        AlertLevel::Critical {
            code: 503,
            msg: "upstream timeout".to_string(),
        },
        AlertLevel::Info,
        AlertLevel::Warning("memory pressure".to_string()),
        AlertLevel::Critical {
            code: 1001,
            msg: "data integrity violation".to_string(),
        },
    ];
    let encoded = encode_to_vec(&alerts).expect("encode Vec<AlertLevel>");
    let compressed = compress_zstd(&encoded).expect("compress Vec<AlertLevel>");
    let decompressed = decompress_zstd(&compressed).expect("decompress Vec<AlertLevel>");
    let (decoded, _): (Vec<AlertLevel>, _) =
        decode_from_slice(&decompressed).expect("decode Vec<AlertLevel>");
    assert_eq!(alerts, decoded);
}

// Test 21: bool roundtrip
#[test]
fn test_bool_roundtrip() {
    for &value in &[true, false] {
        let encoded = encode_to_vec(&value).expect("encode bool");
        let compressed = compress_zstd(&encoded).expect("compress bool");
        let decompressed = decompress_zstd(&compressed).expect("decompress bool");
        let (decoded, _): (bool, _) = decode_from_slice(&decompressed).expect("decode bool");
        assert_eq!(value, decoded, "bool roundtrip failed for value={}", value);
    }
}

// Test 22: Vec<u64> LCG pseudo-random roundtrip
#[test]
fn test_vec_u64_lcg_pseudorandom_roundtrip() {
    // Linear congruential generator: x_{n+1} = (a * x_n + c) mod m
    // Parameters from Knuth's MMIX: a=6364136223846793005, c=1442695040888963407
    let mut state: u64 = 0xDEAD_BEEF_CAFE_1234;
    let values: Vec<u64> = (0..1024)
        .map(|_| {
            state = state
                .wrapping_mul(6_364_136_223_846_793_005)
                .wrapping_add(1_442_695_040_888_963_407);
            state
        })
        .collect();
    let encoded = encode_to_vec(&values).expect("encode Vec<u64> LCG");
    let compressed = compress_zstd(&encoded).expect("compress Vec<u64> LCG");
    let decompressed = decompress_zstd(&compressed).expect("decompress Vec<u64> LCG");
    let (decoded, _): (Vec<u64>, _) =
        decode_from_slice(&decompressed).expect("decode Vec<u64> LCG");
    assert_eq!(values, decoded);
    assert_eq!(decoded.len(), 1024);
}
