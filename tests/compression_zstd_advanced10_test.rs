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

// ---------------------------------------------------------------------------
// Domain types: Database / SQL query results
// ---------------------------------------------------------------------------

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
enum ColumnType {
    Integer,
    Float,
    Text,
    Boolean,
    Blob,
    Null,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
enum DbValue {
    Int(i64),
    Float(f64),
    Text(String),
    Bool(bool),
    Blob(Vec<u8>),
    Null,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct Row {
    row_id: u64,
    columns: Vec<DbValue>,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct QueryResult {
    query_id: u64,
    affected_rows: u64,
    rows: Vec<Row>,
    column_names: Vec<String>,
}

// ---------------------------------------------------------------------------
// Helper: build a realistic QueryResult for a given number of rows
// ---------------------------------------------------------------------------
fn make_query_result(n_rows: usize) -> QueryResult {
    let column_names = vec![
        "id".to_string(),
        "amount".to_string(),
        "label".to_string(),
        "active".to_string(),
        "payload".to_string(),
    ];
    let rows = (0..n_rows as u64)
        .map(|i| Row {
            row_id: i,
            columns: vec![
                DbValue::Int(i as i64),
                DbValue::Float(i as f64 * 1.5),
                DbValue::Text(format!("row-label-{}", i % 100)),
                DbValue::Bool(i % 2 == 0),
                DbValue::Blob(vec![(i % 256) as u8; 8]),
            ],
        })
        .collect();
    QueryResult {
        query_id: 42,
        affected_rows: n_rows as u64,
        rows,
        column_names,
    }
}

// ---------------------------------------------------------------------------
// Test 1: single Row round-trip through Zstd compress / decompress
// ---------------------------------------------------------------------------
#[test]
fn test_row_zstd_roundtrip() {
    let row = Row {
        row_id: 1001,
        columns: vec![
            DbValue::Int(99),
            DbValue::Text("hello".to_string()),
            DbValue::Bool(true),
        ],
    };

    let encoded = encode_to_vec(&row).expect("encode Row failed");
    let compressed = compress(&encoded, Compression::Zstd).expect("compress Row failed");
    let decompressed = decompress(&compressed).expect("decompress Row failed");
    let (decoded, _): (Row, usize) = decode_from_slice(&decompressed).expect("decode Row failed");

    assert_eq!(row, decoded);
}

// ---------------------------------------------------------------------------
// Test 2: QueryResult round-trip (small result set, 5 rows)
// ---------------------------------------------------------------------------
#[test]
fn test_query_result_roundtrip_small() {
    let qr = make_query_result(5);

    let encoded = encode_to_vec(&qr).expect("encode QueryResult failed");
    let compressed = compress(&encoded, Compression::Zstd).expect("compress QueryResult failed");
    let decompressed = decompress(&compressed).expect("decompress QueryResult failed");
    let (decoded, _): (QueryResult, usize) =
        decode_from_slice(&decompressed).expect("decode QueryResult failed");

    assert_eq!(qr, decoded);
}

// ---------------------------------------------------------------------------
// Test 3: ColumnType::Integer encodes and survives Zstd round-trip
// ---------------------------------------------------------------------------
#[test]
fn test_column_type_integer_roundtrip() {
    let ct = ColumnType::Integer;
    let encoded = encode_to_vec(&ct).expect("encode ColumnType::Integer failed");
    let compressed = compress(&encoded, Compression::Zstd).expect("compress failed");
    let decompressed = decompress(&compressed).expect("decompress failed");
    let (decoded, _): (ColumnType, usize) =
        decode_from_slice(&decompressed).expect("decode failed");
    assert_eq!(ct, decoded);
}

// ---------------------------------------------------------------------------
// Test 4: ColumnType::Float round-trip
// ---------------------------------------------------------------------------
#[test]
fn test_column_type_float_roundtrip() {
    let ct = ColumnType::Float;
    let encoded = encode_to_vec(&ct).expect("encode ColumnType::Float failed");
    let compressed = compress(&encoded, Compression::Zstd).expect("compress failed");
    let decompressed = decompress(&compressed).expect("decompress failed");
    let (decoded, _): (ColumnType, usize) =
        decode_from_slice(&decompressed).expect("decode failed");
    assert_eq!(ct, decoded);
}

// ---------------------------------------------------------------------------
// Test 5: ColumnType::Text, Boolean, Blob, Null — all variants in one Vec
// ---------------------------------------------------------------------------
#[test]
fn test_all_column_type_variants_roundtrip() {
    let variants = vec![
        ColumnType::Integer,
        ColumnType::Float,
        ColumnType::Text,
        ColumnType::Boolean,
        ColumnType::Blob,
        ColumnType::Null,
    ];

    let encoded = encode_to_vec(&variants).expect("encode ColumnType variants failed");
    let compressed = compress(&encoded, Compression::Zstd).expect("compress failed");
    let decompressed = decompress(&compressed).expect("decompress failed");
    let (decoded, _): (Vec<ColumnType>, usize) =
        decode_from_slice(&decompressed).expect("decode failed");

    assert_eq!(variants, decoded);
}

// ---------------------------------------------------------------------------
// Test 6: DbValue::Int round-trip
// ---------------------------------------------------------------------------
#[test]
fn test_db_value_int_roundtrip() {
    let val = DbValue::Int(i64::MIN);
    let encoded = encode_to_vec(&val).expect("encode DbValue::Int failed");
    let compressed = compress(&encoded, Compression::Zstd).expect("compress failed");
    let decompressed = decompress(&compressed).expect("decompress failed");
    let (decoded, _): (DbValue, usize) = decode_from_slice(&decompressed).expect("decode failed");
    assert_eq!(val, decoded);
}

// ---------------------------------------------------------------------------
// Test 7: DbValue::Float round-trip (with special values)
// ---------------------------------------------------------------------------
#[test]
fn test_db_value_float_roundtrip() {
    let val = DbValue::Float(std::f64::consts::PI);
    let encoded = encode_to_vec(&val).expect("encode DbValue::Float failed");
    let compressed = compress(&encoded, Compression::Zstd).expect("compress failed");
    let decompressed = decompress(&compressed).expect("decompress failed");
    let (decoded, _): (DbValue, usize) = decode_from_slice(&decompressed).expect("decode failed");
    assert_eq!(val, decoded);
}

// ---------------------------------------------------------------------------
// Test 8: DbValue::Text round-trip (unicode content)
// ---------------------------------------------------------------------------
#[test]
fn test_db_value_text_roundtrip() {
    let val = DbValue::Text("SELECT * FROM テーブル WHERE id = 1;".to_string());
    let encoded = encode_to_vec(&val).expect("encode DbValue::Text failed");
    let compressed = compress(&encoded, Compression::Zstd).expect("compress failed");
    let decompressed = decompress(&compressed).expect("decompress failed");
    let (decoded, _): (DbValue, usize) = decode_from_slice(&decompressed).expect("decode failed");
    assert_eq!(val, decoded);
}

// ---------------------------------------------------------------------------
// Test 9: DbValue::Bool (both true and false) round-trip
// ---------------------------------------------------------------------------
#[test]
fn test_db_value_bool_roundtrip() {
    for b in [true, false] {
        let val = DbValue::Bool(b);
        let encoded = encode_to_vec(&val).expect("encode DbValue::Bool failed");
        let compressed = compress(&encoded, Compression::Zstd).expect("compress failed");
        let decompressed = decompress(&compressed).expect("decompress failed");
        let (decoded, _): (DbValue, usize) =
            decode_from_slice(&decompressed).expect("decode failed");
        assert_eq!(val, decoded, "Bool({b}) roundtrip failed");
    }
}

// ---------------------------------------------------------------------------
// Test 10: DbValue::Blob round-trip (binary payload)
// ---------------------------------------------------------------------------
#[test]
fn test_db_value_blob_roundtrip() {
    let blob: Vec<u8> = (0u8..=255).collect();
    let val = DbValue::Blob(blob);
    let encoded = encode_to_vec(&val).expect("encode DbValue::Blob failed");
    let compressed = compress(&encoded, Compression::Zstd).expect("compress failed");
    let decompressed = decompress(&compressed).expect("decompress failed");
    let (decoded, _): (DbValue, usize) = decode_from_slice(&decompressed).expect("decode failed");
    assert_eq!(val, decoded);
}

// ---------------------------------------------------------------------------
// Test 11: DbValue::Null round-trip
// ---------------------------------------------------------------------------
#[test]
fn test_db_value_null_roundtrip() {
    let val = DbValue::Null;
    let encoded = encode_to_vec(&val).expect("encode DbValue::Null failed");
    let compressed = compress(&encoded, Compression::Zstd).expect("compress failed");
    let decompressed = decompress(&compressed).expect("decompress failed");
    let (decoded, _): (DbValue, usize) = decode_from_slice(&decompressed).expect("decode failed");
    assert_eq!(val, decoded);
}

// ---------------------------------------------------------------------------
// Test 12: Large result set (1000 rows) — Zstd must reduce size
// ---------------------------------------------------------------------------
#[test]
fn test_large_result_set_compression_ratio() {
    let qr = make_query_result(1000);
    let encoded = encode_to_vec(&qr).expect("encode large QueryResult failed");
    let original_len = encoded.len();

    let compressed = compress(&encoded, Compression::Zstd).expect("compress large result failed");

    assert!(
        compressed.len() < original_len,
        "Zstd compressed ({}) should be < original encoded ({}) for 1000-row result",
        compressed.len(),
        original_len
    );

    // Verify the round-trip is still correct
    let decompressed = decompress(&compressed).expect("decompress large result failed");
    let (decoded, _): (QueryResult, usize) =
        decode_from_slice(&decompressed).expect("decode large result failed");
    assert_eq!(qr, decoded);
}

// ---------------------------------------------------------------------------
// Test 13: Very large result set (5000 rows) — Zstd must achieve >50% savings
// ---------------------------------------------------------------------------
#[test]
fn test_very_large_result_set_significant_savings() {
    let qr = make_query_result(5000);
    let encoded = encode_to_vec(&qr).expect("encode 5000-row result failed");
    let original_len = encoded.len();

    let compressed =
        compress(&encoded, Compression::Zstd).expect("compress 5000-row result failed");

    let savings_percent = 100.0 * (1.0 - compressed.len() as f64 / original_len as f64);

    assert!(
        savings_percent > 50.0,
        "Expected >50% savings for 5000-row repetitive result, got {:.1}%",
        savings_percent
    );
}

// ---------------------------------------------------------------------------
// Test 14: Empty QueryResult round-trip (zero rows)
// ---------------------------------------------------------------------------
#[test]
fn test_empty_query_result_roundtrip() {
    let qr = QueryResult {
        query_id: 0,
        affected_rows: 0,
        rows: vec![],
        column_names: vec![],
    };

    let encoded = encode_to_vec(&qr).expect("encode empty QueryResult failed");
    let compressed =
        compress(&encoded, Compression::Zstd).expect("compress empty QueryResult failed");
    let decompressed = decompress(&compressed).expect("decompress empty QueryResult failed");
    let (decoded, _): (QueryResult, usize) =
        decode_from_slice(&decompressed).expect("decode empty QueryResult failed");

    assert_eq!(qr, decoded);
}

// ---------------------------------------------------------------------------
// Test 15: Truncation error — decompress must fail on a cut buffer
// ---------------------------------------------------------------------------
#[test]
fn test_truncated_compressed_data_returns_error() {
    let qr = make_query_result(10);
    let encoded = encode_to_vec(&qr).expect("encode failed");
    let compressed = compress(&encoded, Compression::Zstd).expect("compress failed");

    // Keep only the first 8 bytes — enough for the oxicode header but too
    // short for a valid zstd frame.
    let truncated = compressed[..8].to_vec();
    let result = decompress(&truncated);
    assert!(
        result.is_err(),
        "decompress() must return Err on truncated zstd payload"
    );
}

// ---------------------------------------------------------------------------
// Test 16: Corruption detection — zero out bytes 5..12 of the compressed
//           buffer to destroy the zstd frame header.
// ---------------------------------------------------------------------------
#[test]
fn test_corrupted_zstd_frame_header_returns_error() {
    let qr = make_query_result(20);
    let encoded = encode_to_vec(&qr).expect("encode failed");
    let compressed = compress(&encoded, Compression::Zstd).expect("compress failed");

    assert!(
        compressed.len() > 12,
        "compressed buffer too small for corruption test"
    );

    let mut corrupted = compressed.clone();
    // Bytes 0..5 are the oxicode header; bytes 5..12 are inside the zstd frame.
    for b in corrupted[5..12].iter_mut() {
        *b = 0x00;
    }

    let result = decompress(&corrupted);
    assert!(
        result.is_err(),
        "decompress() must return Err when the zstd frame header is zeroed"
    );
}

// ---------------------------------------------------------------------------
// Test 17: Idempotent decompression — decompressing the decompressed bytes
//           (which have no oxicode magic) must return an error, not
//           silently produce garbage.
// ---------------------------------------------------------------------------
#[test]
fn test_idempotent_decompression_rejects_raw_bytes() {
    let row = Row {
        row_id: 7,
        columns: vec![DbValue::Int(42)],
    };
    let encoded = encode_to_vec(&row).expect("encode failed");
    let compressed = compress(&encoded, Compression::Zstd).expect("compress failed");
    let decompressed = decompress(&compressed).expect("first decompress failed");

    // The raw encoded bytes have no oxicode compression magic — a second
    // decompress call must fail.
    let second = decompress(&decompressed);
    assert!(
        second.is_err(),
        "decompress() of already-decompressed (non-magic) bytes must fail"
    );
}

// ---------------------------------------------------------------------------
// Test 18: ZstdLevel(1) — fastest level still produces correct output
// ---------------------------------------------------------------------------
#[test]
fn test_zstd_level_1_roundtrip() {
    let qr = make_query_result(50);
    let encoded = encode_to_vec(&qr).expect("encode failed");
    let compressed =
        compress(&encoded, Compression::ZstdLevel(1)).expect("compress with ZstdLevel(1) failed");
    let decompressed = decompress(&compressed).expect("decompress ZstdLevel(1) failed");
    let (decoded, _): (QueryResult, usize) =
        decode_from_slice(&decompressed).expect("decode ZstdLevel(1) failed");
    assert_eq!(qr, decoded);
}

// ---------------------------------------------------------------------------
// Test 19: ZstdLevel(19) — high ratio level still produces correct output
// ---------------------------------------------------------------------------
#[test]
fn test_zstd_level_19_roundtrip() {
    let qr = make_query_result(50);
    let encoded = encode_to_vec(&qr).expect("encode failed");
    let compressed =
        compress(&encoded, Compression::ZstdLevel(19)).expect("compress with ZstdLevel(19) failed");
    let decompressed = decompress(&compressed).expect("decompress ZstdLevel(19) failed");
    let (decoded, _): (QueryResult, usize) =
        decode_from_slice(&decompressed).expect("decode ZstdLevel(19) failed");
    assert_eq!(qr, decoded);
}

// ---------------------------------------------------------------------------
// Test 20: Row with all DbValue variants survives round-trip
// ---------------------------------------------------------------------------
#[test]
fn test_row_all_db_value_variants_roundtrip() {
    let row = Row {
        row_id: 999,
        columns: vec![
            DbValue::Int(-1_000_000_000),
            DbValue::Float(-0.0001),
            DbValue::Text("NULL IS NOT NULL".to_string()),
            DbValue::Bool(false),
            DbValue::Blob(vec![0xDE, 0xAD, 0xBE, 0xEF]),
            DbValue::Null,
        ],
    };

    let encoded = encode_to_vec(&row).expect("encode all-variant Row failed");
    let compressed = compress(&encoded, Compression::Zstd).expect("compress failed");
    let decompressed = decompress(&compressed).expect("decompress failed");
    let (decoded, _): (Row, usize) =
        decode_from_slice(&decompressed).expect("decode all-variant Row failed");

    assert_eq!(row, decoded);
}

// ---------------------------------------------------------------------------
// Test 21: QueryResult with NULL-heavy rows (e.g. sparse data) round-trip
// ---------------------------------------------------------------------------
#[test]
fn test_query_result_null_heavy_rows_roundtrip() {
    let rows: Vec<Row> = (0u64..200)
        .map(|i| Row {
            row_id: i,
            columns: vec![
                DbValue::Int(i as i64),
                DbValue::Null,
                DbValue::Null,
                DbValue::Null,
                DbValue::Bool(i % 3 == 0),
            ],
        })
        .collect();

    let qr = QueryResult {
        query_id: 77,
        affected_rows: 200,
        rows,
        column_names: vec![
            "id".to_string(),
            "col_a".to_string(),
            "col_b".to_string(),
            "col_c".to_string(),
            "flag".to_string(),
        ],
    };

    let encoded = encode_to_vec(&qr).expect("encode NULL-heavy QueryResult failed");
    let original_len = encoded.len();
    let compressed = compress(&encoded, Compression::Zstd).expect("compress failed");

    // NULL-heavy (repetitive) data should compress
    assert!(
        compressed.len() < original_len,
        "NULL-heavy result ({} rows) should compress: {} < {}",
        200,
        compressed.len(),
        original_len
    );

    let decompressed = decompress(&compressed).expect("decompress failed");
    let (decoded, _): (QueryResult, usize) =
        decode_from_slice(&decompressed).expect("decode NULL-heavy QueryResult failed");
    assert_eq!(qr, decoded);
}

// ---------------------------------------------------------------------------
// Test 22: Invalid magic bytes are rejected before any decompression attempt
// ---------------------------------------------------------------------------
#[test]
fn test_invalid_magic_bytes_rejected() {
    // Craft a buffer with completely wrong magic (not 0x4F 0x58 0x43)
    let garbage: Vec<u8> = vec![0xAA, 0xBB, 0xCC, 0x01, 0x02, 0x28, 0xB5, 0x2F, 0xFD];
    let result = decompress(&garbage);
    assert!(
        result.is_err(),
        "decompress() must reject a buffer with invalid oxicode magic bytes"
    );
}
