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
struct DataRecord {
    id: u64,
    category: String,
    payload: Vec<u8>,
    score: f64,
}

#[derive(Debug, PartialEq, Encode, Decode)]
enum RecordType {
    Raw,
    Processed(u32),
    Aggregated { count: u32, sum: f64 },
}

#[test]
fn test_data_record_lz4_roundtrip() {
    let record = DataRecord {
        id: 42,
        category: "science".to_string(),
        payload: vec![1, 2, 3, 4, 5, 6, 7, 8],
        score: 3.14159,
    };
    let encoded = encode_to_vec(&record).expect("encode failed");
    let compressed = compress_lz4(&encoded).expect("compress failed");
    let decompressed = decompress_lz4(&compressed).expect("decompress failed");
    let (decoded, _): (DataRecord, usize) =
        decode_from_slice(&decompressed).expect("decode failed");
    assert_eq!(record, decoded);
}

#[test]
fn test_record_type_raw_lz4_roundtrip() {
    let value = RecordType::Raw;
    let encoded = encode_to_vec(&value).expect("encode failed");
    let compressed = compress_lz4(&encoded).expect("compress failed");
    let decompressed = decompress_lz4(&compressed).expect("decompress failed");
    let (decoded, _): (RecordType, usize) =
        decode_from_slice(&decompressed).expect("decode failed");
    assert_eq!(value, decoded);
}

#[test]
fn test_record_type_processed_lz4_roundtrip() {
    let value = RecordType::Processed(1337);
    let encoded = encode_to_vec(&value).expect("encode failed");
    let compressed = compress_lz4(&encoded).expect("compress failed");
    let decompressed = decompress_lz4(&compressed).expect("decompress failed");
    let (decoded, _): (RecordType, usize) =
        decode_from_slice(&decompressed).expect("decode failed");
    assert_eq!(value, decoded);
}

#[test]
fn test_record_type_aggregated_lz4_roundtrip() {
    let value = RecordType::Aggregated {
        count: 99,
        sum: 4567.89,
    };
    let encoded = encode_to_vec(&value).expect("encode failed");
    let compressed = compress_lz4(&encoded).expect("compress failed");
    let decompressed = decompress_lz4(&compressed).expect("decompress failed");
    let (decoded, _): (RecordType, usize) =
        decode_from_slice(&decompressed).expect("decode failed");
    assert_eq!(value, decoded);
}

#[test]
fn test_vec_data_record_5_lz4_roundtrip() {
    let records: Vec<DataRecord> = (0..5)
        .map(|i| DataRecord {
            id: i as u64,
            category: format!("category-{}", i),
            payload: (0u8..=(i as u8)).collect(),
            score: i as f64 * 1.1,
        })
        .collect();
    let encoded = encode_to_vec(&records).expect("encode failed");
    let compressed = compress_lz4(&encoded).expect("compress failed");
    let decompressed = decompress_lz4(&compressed).expect("decompress failed");
    let (decoded, _): (Vec<DataRecord>, usize) =
        decode_from_slice(&decompressed).expect("decode failed");
    assert_eq!(records, decoded);
}

#[test]
fn test_vec_record_type_all_variants_lz4_roundtrip() {
    let values = vec![
        RecordType::Raw,
        RecordType::Processed(10),
        RecordType::Aggregated {
            count: 5,
            sum: 100.0,
        },
        RecordType::Raw,
        RecordType::Processed(999),
    ];
    let encoded = encode_to_vec(&values).expect("encode failed");
    let compressed = compress_lz4(&encoded).expect("compress failed");
    let decompressed = decompress_lz4(&compressed).expect("decompress failed");
    let (decoded, _): (Vec<RecordType>, usize) =
        decode_from_slice(&decompressed).expect("decode failed");
    assert_eq!(values, decoded);
}

#[test]
fn test_large_vec_data_record_100_lz4_roundtrip() {
    let records: Vec<DataRecord> = (0..100)
        .map(|i| DataRecord {
            id: i as u64 * 1000,
            category: format!("tag-{:04}", i),
            payload: vec![i as u8; 16],
            score: (i as f64) * std::f64::consts::PI,
        })
        .collect();
    let encoded = encode_to_vec(&records).expect("encode failed");
    let compressed = compress_lz4(&encoded).expect("compress failed");
    let decompressed = decompress_lz4(&compressed).expect("decompress failed");
    let (decoded, _): (Vec<DataRecord>, usize) =
        decode_from_slice(&decompressed).expect("decode failed");
    assert_eq!(records, decoded);
}

#[test]
fn test_repetitive_vec_u32_compresses_smaller() {
    let data: Vec<u32> = vec![0xDEAD_BEEFu32; 2000];
    let encoded = encode_to_vec(&data).expect("encode failed");
    let compressed = compress_lz4(&encoded).expect("compress failed");
    assert!(
        compressed.len() < encoded.len(),
        "compressed ({} bytes) should be smaller than encoded ({} bytes) for repetitive data",
        compressed.len(),
        encoded.len()
    );
}

#[test]
fn test_repetitive_vec_string_compresses_smaller() {
    let data: Vec<String> = vec!["repetitive-entry".to_string(); 50];
    let encoded = encode_to_vec(&data).expect("encode failed");
    let compressed = compress_lz4(&encoded).expect("compress failed");
    assert!(
        compressed.len() < encoded.len(),
        "compressed ({} bytes) should be smaller than encoded ({} bytes) for repetitive strings",
        compressed.len(),
        encoded.len()
    );
}

#[test]
fn test_compressed_data_record_is_non_empty() {
    let record = DataRecord {
        id: 1,
        category: "test".to_string(),
        payload: vec![0xFF; 32],
        score: 1.0,
    };
    let encoded = encode_to_vec(&record).expect("encode failed");
    let compressed = compress_lz4(&encoded).expect("compress failed");
    assert!(
        !compressed.is_empty(),
        "compressed output must not be empty"
    );
}

#[test]
fn test_decompress_then_encode_equals_original_encode() {
    let record = DataRecord {
        id: 7,
        category: "verify".to_string(),
        payload: vec![10, 20, 30],
        score: 2.718,
    };
    let original_encoded = encode_to_vec(&record).expect("encode failed");
    let compressed = compress_lz4(&original_encoded).expect("compress failed");
    let decompressed = decompress_lz4(&compressed).expect("decompress failed");
    let (decoded, _): (DataRecord, usize) =
        decode_from_slice(&decompressed).expect("decode failed");
    let re_encoded = encode_to_vec(&decoded).expect("re-encode failed");
    assert_eq!(original_encoded, re_encoded);
}

#[test]
fn test_compress_then_decompress_bytes_match_original() {
    let record = DataRecord {
        id: 99,
        category: "bytes-match".to_string(),
        payload: (0u8..128).collect(),
        score: -0.001,
    };
    let original_encoded = encode_to_vec(&record).expect("encode failed");
    let compressed = compress_lz4(&original_encoded).expect("compress failed");
    let decompressed = decompress_lz4(&compressed).expect("decompress failed");
    assert_eq!(original_encoded, decompressed);
}

#[test]
fn test_option_data_record_some_lz4_roundtrip() {
    let value: Option<DataRecord> = Some(DataRecord {
        id: 55,
        category: "optional".to_string(),
        payload: vec![0xAA, 0xBB, 0xCC],
        score: 9.81,
    });
    let encoded = encode_to_vec(&value).expect("encode failed");
    let compressed = compress_lz4(&encoded).expect("compress failed");
    let decompressed = decompress_lz4(&compressed).expect("decompress failed");
    let (decoded, _): (Option<DataRecord>, usize) =
        decode_from_slice(&decompressed).expect("decode failed");
    assert_eq!(value, decoded);
}

#[test]
fn test_option_data_record_none_lz4_roundtrip() {
    let value: Option<DataRecord> = None;
    let encoded = encode_to_vec(&value).expect("encode failed");
    let compressed = compress_lz4(&encoded).expect("compress failed");
    let decompressed = decompress_lz4(&compressed).expect("decompress failed");
    let (decoded, _): (Option<DataRecord>, usize) =
        decode_from_slice(&decompressed).expect("decode failed");
    assert_eq!(value, decoded);
}

#[test]
fn test_i64_min_lz4_roundtrip() {
    let value = i64::MIN;
    let encoded = encode_to_vec(&value).expect("encode failed");
    let compressed = compress_lz4(&encoded).expect("compress failed");
    let decompressed = decompress_lz4(&compressed).expect("decompress failed");
    let (decoded, _): (i64, usize) = decode_from_slice(&decompressed).expect("decode failed");
    assert_eq!(value, decoded);
}

#[test]
fn test_u128_max_lz4_roundtrip() {
    let value = u128::MAX;
    let encoded = encode_to_vec(&value).expect("encode failed");
    let compressed = compress_lz4(&encoded).expect("compress failed");
    let decompressed = decompress_lz4(&compressed).expect("decompress failed");
    let (decoded, _): (u128, usize) = decode_from_slice(&decompressed).expect("decode failed");
    assert_eq!(value, decoded);
}

#[test]
fn test_vec_vec_u8_nested_lz4_roundtrip() {
    let value: Vec<Vec<u8>> = vec![
        vec![1, 2, 3],
        vec![],
        vec![255, 0, 128, 64],
        (0u8..=100).collect(),
    ];
    let encoded = encode_to_vec(&value).expect("encode failed");
    let compressed = compress_lz4(&encoded).expect("compress failed");
    let decompressed = decompress_lz4(&compressed).expect("decompress failed");
    let (decoded, _): (Vec<Vec<u8>>, usize) =
        decode_from_slice(&decompressed).expect("decode failed");
    assert_eq!(value, decoded);
}

#[test]
fn test_decompress_random_non_lz4_bytes_returns_error() {
    // These bytes have a pattern that won't match any valid compression header,
    // so decompress must return an error rather than panic or silently succeed.
    let garbage: Vec<u8> = vec![
        0xDE, 0xAD, 0xBE, 0xEF, 0xCA, 0xFE, 0xBA, 0xBE, 0x00, 0x11, 0x22, 0x33,
    ];
    let result = decompress_lz4(&garbage);
    assert!(
        result.is_err(),
        "decompress of random non-lz4 bytes must return an error"
    );
}

#[test]
fn test_large_string_same_char_lz4_roundtrip() {
    let value: String = "X".repeat(1000);
    let encoded = encode_to_vec(&value).expect("encode failed");
    let compressed = compress_lz4(&encoded).expect("compress failed");
    let decompressed = decompress_lz4(&compressed).expect("decompress failed");
    let (decoded, _): (String, usize) = decode_from_slice(&decompressed).expect("decode failed");
    assert_eq!(value, decoded);
}

#[test]
fn test_data_record_empty_payload_lz4_roundtrip() {
    let record = DataRecord {
        id: 0,
        category: "empty-payload".to_string(),
        payload: vec![],
        score: 0.0,
    };
    let encoded = encode_to_vec(&record).expect("encode failed");
    let compressed = compress_lz4(&encoded).expect("compress failed");
    let decompressed = decompress_lz4(&compressed).expect("decompress failed");
    let (decoded, _): (DataRecord, usize) =
        decode_from_slice(&decompressed).expect("decode failed");
    assert_eq!(record, decoded);
}

#[test]
fn test_data_record_unicode_category_lz4_roundtrip() {
    let record = DataRecord {
        id: 12345,
        category: "日本語テスト/Ünïcödë/中文".to_string(),
        payload: vec![0x01, 0x02, 0x03],
        score: std::f64::consts::E,
    };
    let encoded = encode_to_vec(&record).expect("encode failed");
    let compressed = compress_lz4(&encoded).expect("compress failed");
    let decompressed = decompress_lz4(&compressed).expect("decompress failed");
    let (decoded, _): (DataRecord, usize) =
        decode_from_slice(&decompressed).expect("decode failed");
    assert_eq!(record, decoded);
}

#[test]
fn test_compress_deterministic_same_data_same_output() {
    let record = DataRecord {
        id: 777,
        category: "deterministic".to_string(),
        payload: vec![42u8; 64],
        score: 1.41421356,
    };
    let encoded = encode_to_vec(&record).expect("encode failed");
    let compressed_first = compress_lz4(&encoded).expect("first compress failed");
    let compressed_second = compress_lz4(&encoded).expect("second compress failed");
    assert_eq!(
        compressed_first, compressed_second,
        "LZ4 compression must be deterministic: same input must produce same output"
    );
}
