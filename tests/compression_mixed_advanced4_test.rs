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

fn compress_lz4(data: &[u8]) -> Result<Vec<u8>, Box<dyn std::error::Error>> {
    compress(data, Compression::Lz4).map_err(|e| Box::new(e) as Box<dyn std::error::Error>)
}

fn compress_zstd(data: &[u8]) -> Result<Vec<u8>, Box<dyn std::error::Error>> {
    compress(data, Compression::Zstd).map_err(|e| Box::new(e) as Box<dyn std::error::Error>)
}

fn decompress_data(data: &[u8]) -> Result<Vec<u8>, Box<dyn std::error::Error>> {
    decompress(data).map_err(|e| Box::new(e) as Box<dyn std::error::Error>)
}

#[derive(Debug, PartialEq, Encode, Decode)]
struct DataChunk {
    id: u32,
    data: Vec<u8>,
    checksum: u32,
}

#[derive(Debug, PartialEq, Encode, Decode)]
enum CompressionTestPayload {
    Raw(Vec<u8>),
    Structured { key: String, values: Vec<u64> },
    Nested { outer: u32, inner: Vec<DataChunk> },
}

fn make_data_chunk(id: u32, size: usize) -> DataChunk {
    let data: Vec<u8> = (0..size).map(|i| (i * 7 + 13) as u8).collect();
    let checksum: u32 = data.iter().fold(0u32, |acc, &b| acc.wrapping_add(b as u32));
    DataChunk { id, data, checksum }
}

// Test 1: DataChunk LZ4 roundtrip
#[test]
fn test_data_chunk_lz4_roundtrip() {
    let chunk = make_data_chunk(1, 64);
    let encoded = encode_to_vec(&chunk).expect("encode DataChunk");
    let compressed = compress_lz4(&encoded).expect("compress DataChunk lz4");
    let decompressed = decompress_data(&compressed).expect("decompress DataChunk lz4");
    let (decoded, _): (DataChunk, usize) =
        decode_from_slice(&decompressed).expect("decode DataChunk lz4");
    assert_eq!(chunk, decoded);
}

// Test 2: DataChunk Zstd roundtrip
#[test]
fn test_data_chunk_zstd_roundtrip() {
    let chunk = make_data_chunk(2, 64);
    let encoded = encode_to_vec(&chunk).expect("encode DataChunk");
    let compressed = compress_zstd(&encoded).expect("compress DataChunk zstd");
    let decompressed = decompress_data(&compressed).expect("decompress DataChunk zstd");
    let (decoded, _): (DataChunk, usize) =
        decode_from_slice(&decompressed).expect("decode DataChunk zstd");
    assert_eq!(chunk, decoded);
}

// Test 3: CompressionTestPayload::Raw LZ4 roundtrip
#[test]
fn test_payload_raw_lz4_roundtrip() {
    let payload = CompressionTestPayload::Raw(vec![10u8, 20, 30, 40, 50, 60, 70, 80]);
    let encoded = encode_to_vec(&payload).expect("encode Payload::Raw");
    let compressed = compress_lz4(&encoded).expect("compress Payload::Raw lz4");
    let decompressed = decompress_data(&compressed).expect("decompress Payload::Raw lz4");
    let (decoded, _): (CompressionTestPayload, usize) =
        decode_from_slice(&decompressed).expect("decode Payload::Raw lz4");
    assert_eq!(payload, decoded);
}

// Test 4: CompressionTestPayload::Raw Zstd roundtrip
#[test]
fn test_payload_raw_zstd_roundtrip() {
    let payload = CompressionTestPayload::Raw(vec![10u8, 20, 30, 40, 50, 60, 70, 80]);
    let encoded = encode_to_vec(&payload).expect("encode Payload::Raw");
    let compressed = compress_zstd(&encoded).expect("compress Payload::Raw zstd");
    let decompressed = decompress_data(&compressed).expect("decompress Payload::Raw zstd");
    let (decoded, _): (CompressionTestPayload, usize) =
        decode_from_slice(&decompressed).expect("decode Payload::Raw zstd");
    assert_eq!(payload, decoded);
}

// Test 5: CompressionTestPayload::Structured LZ4 roundtrip
#[test]
fn test_payload_structured_lz4_roundtrip() {
    let payload = CompressionTestPayload::Structured {
        key: "metrics.latency".to_string(),
        values: vec![100, 200, 300, 400, 500],
    };
    let encoded = encode_to_vec(&payload).expect("encode Payload::Structured");
    let compressed = compress_lz4(&encoded).expect("compress Payload::Structured lz4");
    let decompressed = decompress_data(&compressed).expect("decompress Payload::Structured lz4");
    let (decoded, _): (CompressionTestPayload, usize) =
        decode_from_slice(&decompressed).expect("decode Payload::Structured lz4");
    assert_eq!(payload, decoded);
}

// Test 6: CompressionTestPayload::Structured Zstd roundtrip
#[test]
fn test_payload_structured_zstd_roundtrip() {
    let payload = CompressionTestPayload::Structured {
        key: "metrics.latency".to_string(),
        values: vec![100, 200, 300, 400, 500],
    };
    let encoded = encode_to_vec(&payload).expect("encode Payload::Structured");
    let compressed = compress_zstd(&encoded).expect("compress Payload::Structured zstd");
    let decompressed = decompress_data(&compressed).expect("decompress Payload::Structured zstd");
    let (decoded, _): (CompressionTestPayload, usize) =
        decode_from_slice(&decompressed).expect("decode Payload::Structured zstd");
    assert_eq!(payload, decoded);
}

// Test 7: CompressionTestPayload::Nested LZ4 roundtrip
#[test]
fn test_payload_nested_lz4_roundtrip() {
    let payload = CompressionTestPayload::Nested {
        outer: 0xABCD,
        inner: vec![make_data_chunk(10, 32), make_data_chunk(11, 48)],
    };
    let encoded = encode_to_vec(&payload).expect("encode Payload::Nested");
    let compressed = compress_lz4(&encoded).expect("compress Payload::Nested lz4");
    let decompressed = decompress_data(&compressed).expect("decompress Payload::Nested lz4");
    let (decoded, _): (CompressionTestPayload, usize) =
        decode_from_slice(&decompressed).expect("decode Payload::Nested lz4");
    assert_eq!(payload, decoded);
}

// Test 8: CompressionTestPayload::Nested Zstd roundtrip
#[test]
fn test_payload_nested_zstd_roundtrip() {
    let payload = CompressionTestPayload::Nested {
        outer: 0xABCD,
        inner: vec![make_data_chunk(20, 32), make_data_chunk(21, 48)],
    };
    let encoded = encode_to_vec(&payload).expect("encode Payload::Nested");
    let compressed = compress_zstd(&encoded).expect("compress Payload::Nested zstd");
    let decompressed = decompress_data(&compressed).expect("decompress Payload::Nested zstd");
    let (decoded, _): (CompressionTestPayload, usize) =
        decode_from_slice(&decompressed).expect("decode Payload::Nested zstd");
    assert_eq!(payload, decoded);
}

// Test 9: LZ4 compressed != Zstd compressed (different magic bytes)
#[test]
fn test_lz4_and_zstd_compressed_differ() {
    let chunk = make_data_chunk(99, 128);
    let encoded = encode_to_vec(&chunk).expect("encode DataChunk for magic check");
    let lz4_compressed = compress_lz4(&encoded).expect("compress lz4 magic check");
    let zstd_compressed = compress_zstd(&encoded).expect("compress zstd magic check");
    // Both use OxiCode's wrapper format but the compressed payload differs
    assert_ne!(
        lz4_compressed, zstd_compressed,
        "LZ4 and Zstd must produce different compressed bytes"
    );
    // Verify decompressed bytes are identical despite different compressed forms
    let lz4_decompressed = decompress_data(&lz4_compressed).expect("decompress lz4");
    let zstd_decompressed = decompress_data(&zstd_compressed).expect("decompress zstd");
    assert_eq!(
        lz4_decompressed, zstd_decompressed,
        "Both must decompress to identical bytes"
    );
}

// Test 10: Both LZ4 and Zstd decompress back to the same bytes
#[test]
fn test_lz4_and_zstd_both_decompress_to_same_bytes() {
    let chunk = make_data_chunk(50, 96);
    let original_encoded = encode_to_vec(&chunk).expect("encode DataChunk for cross-decompress");
    let lz4_compressed = compress_lz4(&original_encoded).expect("compress lz4 cross");
    let zstd_compressed = compress_zstd(&original_encoded).expect("compress zstd cross");
    let lz4_decompressed = decompress_data(&lz4_compressed).expect("decompress lz4 cross");
    let zstd_decompressed = decompress_data(&zstd_compressed).expect("decompress zstd cross");
    assert_eq!(
        lz4_decompressed, zstd_decompressed,
        "LZ4 and Zstd must decompress to identical bytes"
    );
    assert_eq!(
        original_encoded, lz4_decompressed,
        "LZ4 decompressed must match original encoded"
    );
}

// Test 11: Repetitive data compresses smaller with LZ4
#[test]
fn test_repetitive_data_lz4_compresses_smaller() {
    let repetitive: Vec<u8> = vec![0xABu8; 4096];
    let encoded = encode_to_vec(&repetitive).expect("encode repetitive data");
    let compressed = compress_lz4(&encoded).expect("compress repetitive data lz4");
    assert!(
        compressed.len() < encoded.len(),
        "LZ4 should compress repetitive data: compressed={} vs raw={}",
        compressed.len(),
        encoded.len()
    );
}

// Test 12: Repetitive data compresses smaller with Zstd
#[test]
fn test_repetitive_data_zstd_compresses_smaller() {
    let repetitive: Vec<u8> = vec![0xCDu8; 4096];
    let encoded = encode_to_vec(&repetitive).expect("encode repetitive data");
    let compressed = compress_zstd(&encoded).expect("compress repetitive data zstd");
    assert!(
        compressed.len() < encoded.len(),
        "Zstd should compress repetitive data: compressed={} vs raw={}",
        compressed.len(),
        encoded.len()
    );
}

// Test 13: Zstd typically compresses repetitive data more than LZ4 (zstd_size <= lz4_size * 2)
#[test]
fn test_zstd_compression_ratio_vs_lz4_on_repetitive_data() {
    let repetitive: Vec<u8> = vec![0x42u8; 8192];
    let encoded = encode_to_vec(&repetitive).expect("encode repetitive for ratio comparison");
    let lz4_size = compress_lz4(&encoded)
        .expect("compress lz4 for ratio comparison")
        .len();
    let zstd_size = compress_zstd(&encoded)
        .expect("compress zstd for ratio comparison")
        .len();
    assert!(
        zstd_size <= lz4_size * 2,
        "Zstd compressed size ({}) should be within 2x of LZ4 ({}) for repetitive data",
        zstd_size,
        lz4_size
    );
}

// Test 14: Vec<DataChunk> LZ4 roundtrip
#[test]
fn test_vec_data_chunk_lz4_roundtrip() {
    let chunks: Vec<DataChunk> = (0..8)
        .map(|i| make_data_chunk(i, 32 + i as usize * 4))
        .collect();
    let encoded = encode_to_vec(&chunks).expect("encode Vec<DataChunk>");
    let compressed = compress_lz4(&encoded).expect("compress Vec<DataChunk> lz4");
    let decompressed = decompress_data(&compressed).expect("decompress Vec<DataChunk> lz4");
    let (decoded, _): (Vec<DataChunk>, usize) =
        decode_from_slice(&decompressed).expect("decode Vec<DataChunk> lz4");
    assert_eq!(chunks, decoded);
}

// Test 15: Vec<DataChunk> Zstd roundtrip
#[test]
fn test_vec_data_chunk_zstd_roundtrip() {
    let chunks: Vec<DataChunk> = (0..8)
        .map(|i| make_data_chunk(i + 100, 32 + i as usize * 4))
        .collect();
    let encoded = encode_to_vec(&chunks).expect("encode Vec<DataChunk>");
    let compressed = compress_zstd(&encoded).expect("compress Vec<DataChunk> zstd");
    let decompressed = decompress_data(&compressed).expect("decompress Vec<DataChunk> zstd");
    let (decoded, _): (Vec<DataChunk>, usize) =
        decode_from_slice(&decompressed).expect("decode Vec<DataChunk> zstd");
    assert_eq!(chunks, decoded);
}

// Test 16: LZ4 decompress → Zstd compress → decompress back: same bytes
#[test]
fn test_lz4_decompress_then_zstd_recompress_roundtrip() {
    let chunk = make_data_chunk(77, 80);
    let original_encoded = encode_to_vec(&chunk).expect("encode DataChunk for re-compress test");
    let lz4_compressed = compress_lz4(&original_encoded).expect("initial lz4 compress");
    let lz4_decompressed = decompress_data(&lz4_compressed).expect("lz4 decompress step");
    let zstd_recompressed = compress_zstd(&lz4_decompressed).expect("zstd re-compress step");
    let final_decompressed = decompress_data(&zstd_recompressed).expect("final zstd decompress");
    assert_eq!(
        original_encoded, final_decompressed,
        "After LZ4 decompress → Zstd compress → decompress, bytes must match original"
    );
}

// Test 17: Invalid data returns error from decompress
#[test]
fn test_invalid_data_decompress_returns_error() {
    let garbage: Vec<u8> = vec![0xFF, 0xFE, 0xFD, 0x00, 0xDE, 0xAD, 0xBE, 0xEF, 0x01, 0x02];
    let result = decompress_data(&garbage);
    assert!(
        result.is_err(),
        "Decompressing garbage data must return an error"
    );
}

// Test 18: Empty Vec<u8> via LZ4
#[test]
fn test_empty_vec_u8_lz4_roundtrip() {
    let empty: Vec<u8> = vec![];
    let encoded = encode_to_vec(&empty).expect("encode empty Vec<u8>");
    let compressed = compress_lz4(&encoded).expect("compress empty Vec<u8> lz4");
    let decompressed = decompress_data(&compressed).expect("decompress empty Vec<u8> lz4");
    let (decoded, _): (Vec<u8>, usize) =
        decode_from_slice(&decompressed).expect("decode empty Vec<u8> lz4");
    assert_eq!(empty, decoded);
    assert!(decoded.is_empty());
}

// Test 19: Empty Vec<u8> via Zstd
#[test]
fn test_empty_vec_u8_zstd_roundtrip() {
    let empty: Vec<u8> = vec![];
    let encoded = encode_to_vec(&empty).expect("encode empty Vec<u8>");
    let compressed = compress_zstd(&encoded).expect("compress empty Vec<u8> zstd");
    let decompressed = decompress_data(&compressed).expect("decompress empty Vec<u8> zstd");
    let (decoded, _): (Vec<u8>, usize) =
        decode_from_slice(&decompressed).expect("decode empty Vec<u8> zstd");
    assert_eq!(empty, decoded);
    assert!(decoded.is_empty());
}

// Test 20: Large random-ish data (LCG): LZ4 roundtrip
#[test]
fn test_lcg_large_data_lz4_roundtrip() {
    let mut state: u64 = 0xFEEDFACECAFEBABE;
    let data: Vec<u8> = (0..4096)
        .map(|_| {
            state = state
                .wrapping_mul(6364136223846793005)
                .wrapping_add(1442695040888963407);
            (state >> 33) as u8
        })
        .collect();
    let encoded = encode_to_vec(&data).expect("encode LCG large data");
    let compressed = compress_lz4(&encoded).expect("compress LCG large data lz4");
    let decompressed = decompress_data(&compressed).expect("decompress LCG large data lz4");
    let (decoded, _): (Vec<u8>, usize) =
        decode_from_slice(&decompressed).expect("decode LCG large data lz4");
    assert_eq!(data, decoded);
}

// Test 21: Large random-ish data (LCG): Zstd roundtrip
#[test]
fn test_lcg_large_data_zstd_roundtrip() {
    let mut state: u64 = 0xDEADBEEFBADF00D5;
    let data: Vec<u8> = (0..4096)
        .map(|_| {
            state = state
                .wrapping_mul(6364136223846793005)
                .wrapping_add(1442695040888963407);
            (state >> 33) as u8
        })
        .collect();
    let encoded = encode_to_vec(&data).expect("encode LCG large data");
    let compressed = compress_zstd(&encoded).expect("compress LCG large data zstd");
    let decompressed = decompress_data(&compressed).expect("decompress LCG large data zstd");
    let (decoded, _): (Vec<u8>, usize) =
        decode_from_slice(&decompressed).expect("decode LCG large data zstd");
    assert_eq!(data, decoded);
}

// Test 22: Decompressed bytes exactly match original encoded bytes (both algorithms)
#[test]
fn test_decompressed_bytes_exactly_match_original_both_algorithms() {
    let payload = CompressionTestPayload::Nested {
        outer: 0x1234_5678,
        inner: vec![
            make_data_chunk(1, 40),
            make_data_chunk(2, 60),
            make_data_chunk(3, 80),
        ],
    };
    let original_encoded = encode_to_vec(&payload).expect("encode Payload::Nested for byte check");

    let lz4_compressed = compress_lz4(&original_encoded).expect("compress lz4 byte check");
    let lz4_decompressed = decompress_data(&lz4_compressed).expect("decompress lz4 byte check");
    assert_eq!(
        original_encoded, lz4_decompressed,
        "LZ4: decompressed bytes must exactly match original encoded bytes"
    );

    let zstd_compressed = compress_zstd(&original_encoded).expect("compress zstd byte check");
    let zstd_decompressed = decompress_data(&zstd_compressed).expect("decompress zstd byte check");
    assert_eq!(
        original_encoded, zstd_decompressed,
        "Zstd: decompressed bytes must exactly match original encoded bytes"
    );
}
