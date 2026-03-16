//! Compression interoperability tests for OxiCode.
//!
//! All 22 tests exercise the LZ4 compress/decompress pipeline in combination
//! with the standard oxicode encode/decode API, verifying correctness, size
//! reduction properties, and error-handling behaviour.

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
#[cfg(feature = "compression-lz4")]
use oxicode::{
    compression::{compress, decompress, Compression},
    decode_from_slice, encode_to_vec, Decode, Encode,
};

// ---------------------------------------------------------------------------
// Shared test types (module-level, feature-gated)
// ---------------------------------------------------------------------------

#[cfg(feature = "compression-lz4")]
#[derive(Debug, PartialEq, Encode, Decode)]
struct CompressPoint {
    x: f64,
    y: f64,
    label: String,
}

#[cfg(feature = "compression-lz4")]
#[derive(Debug, PartialEq, Encode, Decode)]
struct NestedOuter {
    inner: CompressPoint,
    count: u64,
    tags: Vec<String>,
}

// ---------------------------------------------------------------------------
// Test 1 – Compress/decompress u32 value roundtrip
// ---------------------------------------------------------------------------

#[cfg(feature = "compression-lz4")]
#[test]
fn test_compress_decompress_u32_roundtrip() {
    let value: u32 = 0xDEAD_BEEF;
    let encoded = encode_to_vec(&value).expect("encode u32");
    let compressed = compress(&encoded, Compression::Lz4).expect("compress u32");
    let decompressed = decompress(&compressed).expect("decompress u32");
    let (decoded, _consumed): (u32, usize) = decode_from_slice(&decompressed).expect("decode u32");
    assert_eq!(value, decoded);
}

// ---------------------------------------------------------------------------
// Test 2 – Compress/decompress String roundtrip
// ---------------------------------------------------------------------------

#[cfg(feature = "compression-lz4")]
#[test]
fn test_compress_decompress_string_roundtrip() {
    let value = String::from("Hello, OxiCode compression!");
    let encoded = encode_to_vec(&value).expect("encode String");
    let compressed = compress(&encoded, Compression::Lz4).expect("compress String");
    let decompressed = decompress(&compressed).expect("decompress String");
    let (decoded, _consumed): (String, usize) =
        decode_from_slice(&decompressed).expect("decode String");
    assert_eq!(value, decoded);
}

// ---------------------------------------------------------------------------
// Test 3 – Compress/decompress Vec<u8> roundtrip
// ---------------------------------------------------------------------------

#[cfg(feature = "compression-lz4")]
#[test]
fn test_compress_decompress_vec_u8_roundtrip() {
    let value: Vec<u8> = (0u8..=255).collect();
    let encoded = encode_to_vec(&value).expect("encode Vec<u8>");
    let compressed = compress(&encoded, Compression::Lz4).expect("compress Vec<u8>");
    let decompressed = decompress(&compressed).expect("decompress Vec<u8>");
    let (decoded, _consumed): (Vec<u8>, usize) =
        decode_from_slice(&decompressed).expect("decode Vec<u8>");
    assert_eq!(value, decoded);
}

// ---------------------------------------------------------------------------
// Test 4 – Compressed size <= original for repetitive data
// ---------------------------------------------------------------------------

#[cfg(feature = "compression-lz4")]
#[test]
fn test_compressed_size_le_original_for_repetitive_data() {
    let value: Vec<u8> = vec![0xABu8; 4096];
    let encoded = encode_to_vec(&value).expect("encode repetitive Vec<u8>");
    let compressed = compress(&encoded, Compression::Lz4).expect("compress repetitive data");
    assert!(
        compressed.len() <= encoded.len(),
        "compressed ({} bytes) should be <= original ({} bytes) for repetitive data",
        compressed.len(),
        encoded.len()
    );
}

// ---------------------------------------------------------------------------
// Test 5 – Compress/decompress Vec<u32> 1000 elements roundtrip
// ---------------------------------------------------------------------------

#[cfg(feature = "compression-lz4")]
#[test]
fn test_compress_decompress_vec_u32_1000_roundtrip() {
    let value: Vec<u32> = (0u32..1000).collect();
    let encoded = encode_to_vec(&value).expect("encode Vec<u32>");
    let compressed = compress(&encoded, Compression::Lz4).expect("compress Vec<u32>");
    let decompressed = decompress(&compressed).expect("decompress Vec<u32>");
    let (decoded, _consumed): (Vec<u32>, usize) =
        decode_from_slice(&decompressed).expect("decode Vec<u32>");
    assert_eq!(value, decoded);
}

// ---------------------------------------------------------------------------
// Test 6 – Compress/decompress empty Vec<u8> roundtrip
// ---------------------------------------------------------------------------

#[cfg(feature = "compression-lz4")]
#[test]
fn test_compress_decompress_empty_vec_roundtrip() {
    let value: Vec<u8> = Vec::new();
    let encoded = encode_to_vec(&value).expect("encode empty Vec<u8>");
    let compressed = compress(&encoded, Compression::Lz4).expect("compress empty Vec<u8>");
    let decompressed = decompress(&compressed).expect("decompress empty Vec<u8>");
    let (decoded, _consumed): (Vec<u8>, usize) =
        decode_from_slice(&decompressed).expect("decode empty Vec<u8>");
    assert_eq!(value, decoded);
    assert!(decoded.is_empty());
}

// ---------------------------------------------------------------------------
// Test 7 – Compress/decompress single byte roundtrip
// ---------------------------------------------------------------------------

#[cfg(feature = "compression-lz4")]
#[test]
fn test_compress_decompress_single_byte_roundtrip() {
    let value: u8 = 42u8;
    let encoded = encode_to_vec(&value).expect("encode u8");
    let compressed = compress(&encoded, Compression::Lz4).expect("compress u8");
    let decompressed = decompress(&compressed).expect("decompress u8");
    let (decoded, _consumed): (u8, usize) = decode_from_slice(&decompressed).expect("decode u8");
    assert_eq!(value, decoded);
}

// ---------------------------------------------------------------------------
// Test 8 – Compress/decompress large string (10000 chars of 'a') roundtrip
// ---------------------------------------------------------------------------

#[cfg(feature = "compression-lz4")]
#[test]
fn test_compress_decompress_large_string_roundtrip() {
    let value: String = "a".repeat(10_000);
    let encoded = encode_to_vec(&value).expect("encode large String");
    let compressed = compress(&encoded, Compression::Lz4).expect("compress large String");
    let decompressed = decompress(&compressed).expect("decompress large String");
    let (decoded, _consumed): (String, usize) =
        decode_from_slice(&decompressed).expect("decode large String");
    assert_eq!(value, decoded);
    assert_eq!(decoded.len(), 10_000);
}

// ---------------------------------------------------------------------------
// Test 9 – Compress/decompress struct with derive roundtrip
// ---------------------------------------------------------------------------

#[cfg(feature = "compression-lz4")]
#[test]
fn test_compress_decompress_derived_struct_roundtrip() {
    let value = CompressPoint {
        x: 1.234_567_890_123_456,
        y: -9.876_543_210_987_654,
        label: String::from("origin"),
    };
    let encoded = encode_to_vec(&value).expect("encode CompressPoint");
    let compressed = compress(&encoded, Compression::Lz4).expect("compress CompressPoint");
    let decompressed = decompress(&compressed).expect("decompress CompressPoint");
    let (decoded, _consumed): (CompressPoint, usize) =
        decode_from_slice(&decompressed).expect("decode CompressPoint");
    assert_eq!(value, decoded);
}

// ---------------------------------------------------------------------------
// Test 10 – Compress/decompress Vec<String> roundtrip
// ---------------------------------------------------------------------------

#[cfg(feature = "compression-lz4")]
#[test]
fn test_compress_decompress_vec_string_roundtrip() {
    let value: Vec<String> = (0u32..20).map(|i| format!("item-{}", i)).collect();
    let encoded = encode_to_vec(&value).expect("encode Vec<String>");
    let compressed = compress(&encoded, Compression::Lz4).expect("compress Vec<String>");
    let decompressed = decompress(&compressed).expect("decompress Vec<String>");
    let (decoded, _consumed): (Vec<String>, usize) =
        decode_from_slice(&decompressed).expect("decode Vec<String>");
    assert_eq!(value, decoded);
}

// ---------------------------------------------------------------------------
// Test 11 – Two identical inputs produce same compressed output (deterministic)
// ---------------------------------------------------------------------------

#[cfg(feature = "compression-lz4")]
#[test]
fn test_compression_is_deterministic() {
    let value: Vec<u32> = (0u32..256).collect();
    let encoded = encode_to_vec(&value).expect("encode for determinism check");
    let compressed_a = compress(&encoded, Compression::Lz4).expect("compress first copy");
    let compressed_b = compress(&encoded, Compression::Lz4).expect("compress second copy");
    assert_eq!(
        compressed_a, compressed_b,
        "same input must yield identical compressed output"
    );
}

// ---------------------------------------------------------------------------
// Test 12 – Two different inputs produce different compressed output
// ---------------------------------------------------------------------------

#[cfg(feature = "compression-lz4")]
#[test]
fn test_different_inputs_produce_different_compressed_output() {
    let value_a: u64 = 1_111_111_111;
    let value_b: u64 = 9_999_999_999;
    let encoded_a = encode_to_vec(&value_a).expect("encode value_a");
    let encoded_b = encode_to_vec(&value_b).expect("encode value_b");
    let compressed_a = compress(&encoded_a, Compression::Lz4).expect("compress value_a");
    let compressed_b = compress(&encoded_b, Compression::Lz4).expect("compress value_b");
    assert_ne!(
        compressed_a, compressed_b,
        "distinct inputs must produce distinct compressed bytes"
    );
}

// ---------------------------------------------------------------------------
// Test 13 – Compressed data cannot be decoded directly as original type (error)
// ---------------------------------------------------------------------------

#[cfg(feature = "compression-lz4")]
#[test]
fn test_compressed_bytes_cannot_be_decoded_directly() {
    let value: u32 = 12345u32;
    let encoded = encode_to_vec(&value).expect("encode u32 for negative test");
    let compressed = compress(&encoded, Compression::Lz4).expect("compress for negative test");
    // The compressed bytes have a 5-byte OXC header followed by LZ4 payload,
    // which is not valid oxicode u32 encoding — decoding must fail or produce
    // a different value.  We check that the bytes differ from the original.
    assert_ne!(
        encoded, compressed,
        "compressed payload must differ from the raw encoded bytes"
    );
    // Attempting to decode the compressed bytes as u32 should either error or
    // yield a value != the original (the header bytes will be mis-interpreted).
    let decode_result: oxicode::Result<(u32, usize)> = decode_from_slice(&compressed);
    match decode_result {
        Ok((decoded_val, _)) => assert_ne!(
            decoded_val, value,
            "decoding compressed bytes directly must not reproduce the original value"
        ),
        Err(_) => { /* expected: raw compressed bytes are not valid u32 encoding */ }
    }
}

// ---------------------------------------------------------------------------
// Test 14 – Decompress invalid bytes fails gracefully
// ---------------------------------------------------------------------------

#[cfg(feature = "compression-lz4")]
#[test]
fn test_decompress_invalid_bytes_fails_gracefully() {
    let garbage: Vec<u8> = vec![0xFF, 0xFE, 0xFD, 0xFC, 0xFB, 0x00, 0x01, 0x02];
    let result = decompress(&garbage);
    assert!(
        result.is_err(),
        "decompress must return Err for arbitrary invalid bytes"
    );
}

// ---------------------------------------------------------------------------
// Test 15 – Compress then decompress all-zeros (1000 bytes) roundtrip
// ---------------------------------------------------------------------------

#[cfg(feature = "compression-lz4")]
#[test]
fn test_compress_decompress_all_zeros_roundtrip() {
    let value: Vec<u8> = vec![0u8; 1000];
    let encoded = encode_to_vec(&value).expect("encode all-zeros");
    let compressed = compress(&encoded, Compression::Lz4).expect("compress all-zeros");
    let decompressed = decompress(&compressed).expect("decompress all-zeros");
    let (decoded, _consumed): (Vec<u8>, usize) =
        decode_from_slice(&decompressed).expect("decode all-zeros");
    assert_eq!(value, decoded);
}

// ---------------------------------------------------------------------------
// Test 16 – Compress then decompress all-255 (1000 bytes) roundtrip
// ---------------------------------------------------------------------------

#[cfg(feature = "compression-lz4")]
#[test]
fn test_compress_decompress_all_max_bytes_roundtrip() {
    let value: Vec<u8> = vec![255u8; 1000];
    let encoded = encode_to_vec(&value).expect("encode all-255");
    let compressed = compress(&encoded, Compression::Lz4).expect("compress all-255");
    let decompressed = decompress(&compressed).expect("decompress all-255");
    let (decoded, _consumed): (Vec<u8>, usize) =
        decode_from_slice(&decompressed).expect("decode all-255");
    assert_eq!(value, decoded);
}

// ---------------------------------------------------------------------------
// Test 17 – Compress then decompress alternating pattern roundtrip
// ---------------------------------------------------------------------------

#[cfg(feature = "compression-lz4")]
#[test]
fn test_compress_decompress_alternating_pattern_roundtrip() {
    let value: Vec<u8> = (0u16..1000)
        .map(|i| if i % 2 == 0 { 0u8 } else { 1u8 })
        .collect();
    let encoded = encode_to_vec(&value).expect("encode alternating pattern");
    let compressed = compress(&encoded, Compression::Lz4).expect("compress alternating");
    let decompressed = decompress(&compressed).expect("decompress alternating");
    let (decoded, _consumed): (Vec<u8>, usize) =
        decode_from_slice(&decompressed).expect("decode alternating");
    assert_eq!(value, decoded);
}

// ---------------------------------------------------------------------------
// Test 18 – Large repeated pattern compresses to smaller size
// ---------------------------------------------------------------------------

#[cfg(feature = "compression-lz4")]
#[test]
fn test_large_repeated_pattern_compresses_smaller() {
    // 50 000 copies of a 10-byte pattern → highly compressible
    let pattern: Vec<u8> = b"ABCDEFGHIJ".to_vec();
    let value: Vec<u8> = pattern.iter().cycle().take(50_000).copied().collect();
    let encoded = encode_to_vec(&value).expect("encode large repeated pattern");
    let compressed = compress(&encoded, Compression::Lz4).expect("compress large pattern");
    assert!(
        compressed.len() < encoded.len(),
        "large repeated pattern ({} bytes encoded) must compress to fewer bytes ({} compressed)",
        encoded.len(),
        compressed.len()
    );
    // Verify roundtrip correctness
    let decompressed = decompress(&compressed).expect("decompress large pattern");
    let (decoded, _consumed): (Vec<u8>, usize) =
        decode_from_slice(&decompressed).expect("decode large pattern");
    assert_eq!(value, decoded);
}

// ---------------------------------------------------------------------------
// Test 19 – Compress/decompress Option<Vec<u8>> Some roundtrip
// ---------------------------------------------------------------------------

#[cfg(feature = "compression-lz4")]
#[test]
fn test_compress_decompress_option_some_roundtrip() {
    let value: Option<Vec<u8>> = Some(vec![10u8, 20, 30, 40, 50]);
    let encoded = encode_to_vec(&value).expect("encode Option Some");
    let compressed = compress(&encoded, Compression::Lz4).expect("compress Option Some");
    let decompressed = decompress(&compressed).expect("decompress Option Some");
    let (decoded, _consumed): (Option<Vec<u8>>, usize) =
        decode_from_slice(&decompressed).expect("decode Option Some");
    assert_eq!(value, decoded);
    assert!(decoded.is_some());
}

// ---------------------------------------------------------------------------
// Test 20 – Compress/decompress Option<Vec<u8>> None roundtrip
// ---------------------------------------------------------------------------

#[cfg(feature = "compression-lz4")]
#[test]
fn test_compress_decompress_option_none_roundtrip() {
    let value: Option<Vec<u8>> = None;
    let encoded = encode_to_vec(&value).expect("encode Option None");
    let compressed = compress(&encoded, Compression::Lz4).expect("compress Option None");
    let decompressed = decompress(&compressed).expect("decompress Option None");
    let (decoded, _consumed): (Option<Vec<u8>>, usize) =
        decode_from_slice(&decompressed).expect("decode Option None");
    assert_eq!(value, decoded);
    assert!(decoded.is_none());
}

// ---------------------------------------------------------------------------
// Test 21 – Multiple sequential compress/decompress operations
// ---------------------------------------------------------------------------

#[cfg(feature = "compression-lz4")]
#[test]
fn test_multiple_sequential_compress_decompress() {
    let values: Vec<u64> = (1u64..=10).map(|i| i * 1_000_000).collect();

    for (idx, &original) in values.iter().enumerate() {
        let encoded = encode_to_vec(&original).expect("encode u64 in sequence");
        let compressed = compress(&encoded, Compression::Lz4).expect("compress u64 in sequence");
        let decompressed = decompress(&compressed).expect("decompress u64 in sequence");
        let (decoded, _consumed): (u64, usize) =
            decode_from_slice(&decompressed).expect("decode u64 in sequence");
        assert_eq!(
            original, decoded,
            "sequential roundtrip failed at index {}",
            idx
        );
    }
}

// ---------------------------------------------------------------------------
// Test 22 – Compress/decompress with nested struct
// ---------------------------------------------------------------------------

#[cfg(feature = "compression-lz4")]
#[test]
fn test_compress_decompress_nested_struct_roundtrip() {
    let value = NestedOuter {
        inner: CompressPoint {
            x: 1.0,
            y: -1.0,
            label: String::from("nested-point"),
        },
        count: 42,
        tags: vec![
            String::from("alpha"),
            String::from("beta"),
            String::from("gamma"),
        ],
    };
    let encoded = encode_to_vec(&value).expect("encode NestedOuter");
    let compressed = compress(&encoded, Compression::Lz4).expect("compress NestedOuter");
    let decompressed = decompress(&compressed).expect("decompress NestedOuter");
    let (decoded, _consumed): (NestedOuter, usize) =
        decode_from_slice(&decompressed).expect("decode NestedOuter");
    assert_eq!(value, decoded);
}
