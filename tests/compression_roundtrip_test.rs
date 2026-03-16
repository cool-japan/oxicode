//! Comprehensive roundtrip tests for OxiCode compression.

#![cfg(any(feature = "compression-lz4", feature = "compression-zstd"))]
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
use oxicode::compression::{
    compress, compress_with_stats, decompress, decompress_or_passthrough, detect_compression,
    is_compressed, Compression,
};

// ──────────────────────────────────────────────────────────────────────────────
// Helper struct shared across tests that need a serialisable type.
// ──────────────────────────────────────────────────────────────────────────────

#[derive(Debug, PartialEq, oxicode::Encode, oxicode::Decode)]
struct SensorRecord {
    id: u32,
    label: String,
    reading: u64,
    active: bool,
}

// ──────────────────────────────────────────────────────────────────────────────
// Test 1 – Basic bytes roundtrip with LZ4
// ──────────────────────────────────────────────────────────────────────────────

#[cfg(feature = "compression-lz4")]
#[test]
fn test_lz4_basic_bytes_roundtrip() {
    let original: &[u8] = b"Hello, OxiCode! Basic LZ4 roundtrip.";
    let compressed = compress(original, Compression::Lz4).expect("lz4 compress basic bytes failed");
    let decompressed = decompress(&compressed).expect("lz4 decompress basic bytes failed");
    assert_eq!(original, decompressed.as_slice());
}

// ──────────────────────────────────────────────────────────────────────────────
// Test 2 – Empty bytes compression roundtrip
// ──────────────────────────────────────────────────────────────────────────────

#[cfg(feature = "compression-lz4")]
#[test]
fn test_lz4_empty_bytes_roundtrip() {
    let original: &[u8] = b"";
    let compressed = compress(original, Compression::Lz4).expect("lz4 compress empty bytes failed");
    let decompressed = decompress(&compressed).expect("lz4 decompress empty bytes failed");
    assert!(
        decompressed.is_empty(),
        "decompressing empty payload must yield an empty vec"
    );
}

// ──────────────────────────────────────────────────────────────────────────────
// Test 3 – Small data (< 10 bytes) roundtrip
// ──────────────────────────────────────────────────────────────────────────────

#[cfg(feature = "compression-lz4")]
#[test]
fn test_lz4_small_data_roundtrip() {
    let original: &[u8] = b"tiny";
    assert!(
        original.len() < 10,
        "precondition: payload must be < 10 bytes"
    );
    let compressed = compress(original, Compression::Lz4).expect("lz4 compress small data failed");
    let decompressed = decompress(&compressed).expect("lz4 decompress small data failed");
    assert_eq!(original, decompressed.as_slice());
}

// ──────────────────────────────────────────────────────────────────────────────
// Test 4 – Large compressible data: 1000 repeated bytes, compressed < original
// ──────────────────────────────────────────────────────────────────────────────

#[cfg(feature = "compression-lz4")]
#[test]
fn test_lz4_large_compressible_data_smaller_after_compression() {
    let original: Vec<u8> = vec![0xABu8; 1000];
    let compressed =
        compress(&original, Compression::Lz4).expect("lz4 compress large compressible failed");
    let decompressed = decompress(&compressed).expect("lz4 decompress large compressible failed");
    assert_eq!(original, decompressed);
    assert!(
        compressed.len() < original.len(),
        "compressed ({}) must be smaller than original ({})",
        compressed.len(),
        original.len()
    );
}

// ──────────────────────────────────────────────────────────────────────────────
// Test 5 – Large incompressible (pseudo-random) data roundtrip
// ──────────────────────────────────────────────────────────────────────────────

#[cfg(feature = "compression-lz4")]
#[test]
fn test_lz4_large_incompressible_roundtrip() {
    // LCG-derived pseudo-random bytes — won't compress well but must round-trip.
    let original: Vec<u8> = (0u64..1000)
        .map(|i| {
            i.wrapping_mul(6_364_136_223_846_793_005)
                .wrapping_add(1_442_695_040_888_963_407)
                .wrapping_shr(33) as u8
        })
        .collect();
    let compressed =
        compress(&original, Compression::Lz4).expect("lz4 compress incompressible failed");
    let decompressed = decompress(&compressed).expect("lz4 decompress incompressible failed");
    assert_eq!(original, decompressed);
}

// ──────────────────────────────────────────────────────────────────────────────
// Test 6 – String (UTF-8) data roundtrip
// ──────────────────────────────────────────────────────────────────────────────

#[cfg(feature = "compression-lz4")]
#[test]
fn test_lz4_string_data_roundtrip() {
    let text = "The quick brown fox jumps over the lazy dog. \
                Pack my box with five dozen liquor jugs. \
                How vexingly quick daft zebras jump!";
    let original = text.as_bytes();
    let compressed = compress(original, Compression::Lz4).expect("lz4 compress string data failed");
    let decompressed = decompress(&compressed).expect("lz4 decompress string data failed");
    let recovered =
        std::str::from_utf8(&decompressed).expect("decompressed bytes are not valid UTF-8");
    assert_eq!(text, recovered);
}

// ──────────────────────────────────────────────────────────────────────────────
// Test 7 – Vec<u32> roundtrip: encode_to_vec then compress, decompress then decode
// ──────────────────────────────────────────────────────────────────────────────

#[cfg(feature = "compression-lz4")]
#[test]
fn test_lz4_vec_u32_encode_compress_decompress_decode_roundtrip() {
    let values: Vec<u32> = (0u32..256).collect();
    let encoded = oxicode::encode_to_vec(&values).expect("encode Vec<u32> failed");
    let compressed =
        compress(&encoded, Compression::Lz4).expect("lz4 compress Vec<u32> bytes failed");
    let decompressed = decompress(&compressed).expect("lz4 decompress Vec<u32> bytes failed");
    let (decoded, _): (Vec<u32>, usize) = oxicode::decode_from_slice(&decompressed)
        .expect("decode Vec<u32> after lz4 decompress failed");
    assert_eq!(values, decoded);
}

// ──────────────────────────────────────────────────────────────────────────────
// Test 8 – encode_to_vec then compress (combined pipeline) for scalar value
// ──────────────────────────────────────────────────────────────────────────────

#[cfg(feature = "compression-lz4")]
#[test]
fn test_lz4_encode_to_vec_then_compress_pipeline() {
    let payload: u64 = u64::MAX / 7;
    let encoded = oxicode::encode_to_vec(&payload).expect("encode u64 failed");
    let compressed = compress(&encoded, Compression::Lz4).expect("lz4 compress encoded u64 failed");
    let decompressed = decompress(&compressed).expect("lz4 decompress encoded u64 failed");
    let (decoded, consumed): (u64, usize) = oxicode::decode_from_slice(&decompressed)
        .expect("decode u64 from decompressed bytes failed");
    assert_eq!(payload, decoded);
    assert_eq!(consumed, decompressed.len());
}

// ──────────────────────────────────────────────────────────────────────────────
// Test 9 – Decompress then decode pattern (SensorRecord struct)
// ──────────────────────────────────────────────────────────────────────────────

#[cfg(feature = "compression-lz4")]
#[test]
fn test_lz4_decompress_then_decode_struct() {
    let record = SensorRecord {
        id: 99,
        label: "gyroscope-Z".into(),
        reading: 8_192_000,
        active: true,
    };
    let encoded = oxicode::encode_to_vec(&record).expect("encode SensorRecord failed");
    let compressed =
        compress(&encoded, Compression::Lz4).expect("lz4 compress SensorRecord bytes failed");

    // Simulate the receiver side: decompress then decode.
    let decompressed = decompress(&compressed).expect("lz4 decompress SensorRecord bytes failed");
    let (recovered, _): (SensorRecord, usize) = oxicode::decode_from_slice(&decompressed)
        .expect("decode SensorRecord after lz4 decompress failed");
    assert_eq!(record, recovered);
}

// ──────────────────────────────────────────────────────────────────────────────
// Test 10 – Compression ratio: highly repetitive data compresses well
// ──────────────────────────────────────────────────────────────────────────────

#[cfg(feature = "compression-lz4")]
#[test]
fn test_lz4_compression_ratio_repetitive_data() {
    let original: Vec<u8> = vec![0x42u8; 10_000];
    let (compressed, stats) = compress_with_stats(&original, Compression::Lz4)
        .expect("lz4 compress_with_stats repetitive failed");
    assert_eq!(stats.original_size, original.len());
    assert_eq!(stats.compressed_size, compressed.len());
    assert!(
        stats.ratio() > 10.0,
        "expected ratio > 10 for all-identical bytes, got {:.2}",
        stats.ratio()
    );
    assert!(stats.savings_percent() > 80.0);

    let decompressed =
        decompress(&compressed).expect("lz4 decompress repetitive stats test failed");
    assert_eq!(original, decompressed);
}

// ──────────────────────────────────────────────────────────────────────────────
// Test 11 – Multiple sequential compress/decompress operations are independent
// ──────────────────────────────────────────────────────────────────────────────

#[cfg(feature = "compression-lz4")]
#[test]
fn test_lz4_multiple_sequential_roundtrips() {
    let payloads: &[&[u8]] = &[
        b"first payload",
        b"second payload with more bytes here",
        b"third",
        b"",
        b"fifth payload: aaaaaaaaaaaaaaaaaaaaaaaaaaaa",
    ];
    for &payload in payloads {
        let compressed =
            compress(payload, Compression::Lz4).expect("sequential lz4 compress failed");
        let decompressed = decompress(&compressed).expect("sequential lz4 decompress failed");
        assert_eq!(payload, decompressed.as_slice());
    }
}

// ──────────────────────────────────────────────────────────────────────────────
// Test 12 – is_compressed correctly identifies LZ4-wrapped vs raw bytes
// ──────────────────────────────────────────────────────────────────────────────

#[cfg(feature = "compression-lz4")]
#[test]
fn test_lz4_is_compressed_detection() {
    let raw = b"plain bytes - no magic header";
    assert!(
        !is_compressed(raw),
        "raw bytes must not be flagged as compressed"
    );
    assert!(
        !is_compressed(&[]),
        "empty slice must not be flagged as compressed"
    );

    let compressed =
        compress(raw, Compression::Lz4).expect("lz4 compress for detection test failed");
    assert!(
        is_compressed(&compressed),
        "lz4-compressed bytes must be flagged as compressed"
    );
}

// ──────────────────────────────────────────────────────────────────────────────
// Test 13 – detect_compression returns Some(Lz4) for LZ4-wrapped data
// ──────────────────────────────────────────────────────────────────────────────

#[cfg(feature = "compression-lz4")]
#[test]
fn test_lz4_detect_compression_variant() {
    let data = b"codec variant detection test data for lz4";
    let compressed =
        compress(data, Compression::Lz4).expect("lz4 compress for variant detection failed");
    assert_eq!(
        detect_compression(&compressed),
        Some(Compression::Lz4),
        "detect_compression must return Some(Lz4)"
    );
    assert_eq!(
        detect_compression(data),
        None,
        "detect_compression on raw bytes must return None"
    );
}

// ──────────────────────────────────────────────────────────────────────────────
// Test 14 – decompress_or_passthrough with compressed input returns original
// ──────────────────────────────────────────────────────────────────────────────

#[cfg(feature = "compression-lz4")]
#[test]
fn test_lz4_decompress_or_passthrough_with_compressed_input() {
    let original = b"passthrough test - compressed path for lz4";
    let compressed =
        compress(original, Compression::Lz4).expect("lz4 compress for passthrough test failed");
    let result = decompress_or_passthrough(&compressed)
        .expect("decompress_or_passthrough compressed lz4 failed");
    assert_eq!(original.as_slice(), result.as_slice());
}

// ──────────────────────────────────────────────────────────────────────────────
// Test 15 – decompress_or_passthrough with raw bytes passes them through unchanged
// ──────────────────────────────────────────────────────────────────────────────

#[cfg(any(feature = "compression-lz4", feature = "compression-zstd"))]
#[test]
fn test_decompress_or_passthrough_with_raw_input() {
    let raw = b"uncompressed - must survive passthrough unchanged";
    let result =
        decompress_or_passthrough(raw).expect("decompress_or_passthrough raw bytes failed");
    assert_eq!(raw.as_slice(), result.as_slice());
}

// ──────────────────────────────────────────────────────────────────────────────
// Test 16 – All 256 byte values survive an LZ4 roundtrip
// ──────────────────────────────────────────────────────────────────────────────

#[cfg(feature = "compression-lz4")]
#[test]
fn test_lz4_all_byte_values_roundtrip() {
    // Repeat each byte 10 times so there is some redundancy for LZ4 to exploit.
    let original: Vec<u8> = (0u8..=255)
        .flat_map(|b| std::iter::repeat(b).take(10))
        .collect();
    let compressed =
        compress(&original, Compression::Lz4).expect("lz4 compress all-byte-values failed");
    let decompressed = decompress(&compressed).expect("lz4 decompress all-byte-values failed");
    assert_eq!(original, decompressed);
}

// ──────────────────────────────────────────────────────────────────────────────
// Test 17 – Corruption of the magic header causes decompress to return an error
// ──────────────────────────────────────────────────────────────────────────────

#[cfg(feature = "compression-lz4")]
#[test]
fn test_lz4_corrupt_magic_header_causes_error() {
    let data = b"data compressed then deliberately corrupted";
    let mut compressed =
        compress(data, Compression::Lz4).expect("lz4 compress for corruption test failed");
    // Overwrite magic bytes with garbage.
    compressed[0] = 0xDE;
    compressed[1] = 0xAD;
    compressed[2] = 0xBE;
    let result = decompress(&compressed);
    assert!(
        result.is_err(),
        "decompress on corrupted magic must return an error"
    );
}

// ──────────────────────────────────────────────────────────────────────────────
// Test 18 – Compression::None (no-op codec) roundtrip via the unified API
// ──────────────────────────────────────────────────────────────────────────────

#[cfg(any(feature = "compression-lz4", feature = "compression-zstd"))]
#[test]
fn test_none_codec_roundtrip() {
    let original = b"no compression, just header wrapping for the None codec";
    let wrapped = compress(original, Compression::None).expect("compress None failed");
    let recovered = decompress(&wrapped).expect("decompress None failed");
    assert_eq!(original.as_slice(), recovered.as_slice());
    assert!(
        is_compressed(&wrapped),
        "None-wrapped data must carry the OXC magic header"
    );
}

// ──────────────────────────────────────────────────────────────────────────────
// Test 19 – compress_with_stats for None codec: compressed_size == original + 5
// ──────────────────────────────────────────────────────────────────────────────

#[cfg(any(feature = "compression-lz4", feature = "compression-zstd"))]
#[test]
fn test_none_codec_stats_sizes() {
    let original = b"stats test with None codec - 5-byte header overhead expected";
    let (wrapped, stats) =
        compress_with_stats(original, Compression::None).expect("compress_with_stats None failed");
    assert_eq!(stats.original_size, original.len());
    assert_eq!(stats.compressed_size, wrapped.len());
    // None codec adds exactly 5 header bytes (MAGIC[3] + VERSION[1] + CODEC[1]).
    assert_eq!(
        stats.compressed_size,
        original.len() + 5,
        "None codec overhead must be exactly 5 bytes"
    );
}

// ──────────────────────────────────────────────────────────────────────────────
// Test 20 – Zstd basic roundtrip
// ──────────────────────────────────────────────────────────────────────────────

#[cfg(feature = "compression-zstd")]
#[test]
fn test_zstd_basic_roundtrip() {
    let original: &[u8] = b"Zstd compression roundtrip: hello from OxiCode!";
    let compressed = compress(original, Compression::Zstd).expect("zstd compress basic failed");
    let decompressed = decompress(&compressed).expect("zstd decompress basic failed");
    assert_eq!(original, decompressed.as_slice());
}

// ──────────────────────────────────────────────────────────────────────────────
// Test 21 – Zstd empty data roundtrip
// ──────────────────────────────────────────────────────────────────────────────

#[cfg(feature = "compression-zstd")]
#[test]
fn test_zstd_empty_data_roundtrip() {
    let original: &[u8] = &[];
    let compressed = compress(original, Compression::Zstd).expect("zstd compress empty failed");
    let decompressed = decompress(&compressed).expect("zstd decompress empty failed");
    assert!(
        decompressed.is_empty(),
        "zstd round-trip of empty slice must yield empty vec"
    );
}

// ──────────────────────────────────────────────────────────────────────────────
// Test 22 – Zstd with explicit level and detect_compression returns Zstd
// ──────────────────────────────────────────────────────────────────────────────

#[cfg(feature = "compression-zstd")]
#[test]
fn test_zstd_level_roundtrip_and_detection() {
    let original: Vec<u8> = b"Zstd level test payload: "
        .iter()
        .copied()
        .cycle()
        .take(512)
        .collect();

    let compressed =
        compress(&original, Compression::ZstdLevel(5)).expect("zstd level-5 compress failed");
    let decompressed = decompress(&compressed).expect("zstd level-5 decompress failed");
    assert_eq!(original, decompressed);

    // Both Zstd and ZstdLevel share codec_id 2, so detection always returns Zstd.
    let detected = detect_compression(&compressed);
    assert_eq!(
        detected,
        Some(Compression::Zstd),
        "ZstdLevel compressed data must be detected as Zstd"
    );
}
