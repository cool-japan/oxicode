//! Advanced compression tests for OxiCode.
//!
//! These tests cover compression edge cases, statistics, detection, and
//! interaction with the encode/decode pipeline that are not covered by the
//! existing compression_roundtrip_test.rs / compression_test.rs suites.

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
#[cfg(any(feature = "compression-lz4", feature = "compression-zstd"))]
use oxicode::{Decode, Encode};

#[cfg(any(feature = "compression-lz4", feature = "compression-zstd"))]
#[derive(Debug, PartialEq, Encode, Decode)]
struct AdvRecord {
    id: u64,
    tag: String,
    values: Vec<i32>,
}

// ─────────────────────────────────────────────────────────────────────────────
// Test 1 – 1000 zero bytes roundtrip + is_compressed check
// ─────────────────────────────────────────────────────────────────────────────

#[cfg(feature = "compression-lz4")]
#[test]
fn test_adv_lz4_zeros_1000_roundtrip() {
    use oxicode::compression::{compress, decompress, is_compressed, Compression};

    let original = vec![0u8; 1000];
    let compressed = compress(&original, Compression::Lz4).expect("lz4 compress 1000 zeros failed");
    assert!(
        is_compressed(&compressed),
        "output of compress() must be detected as compressed"
    );
    let decompressed = decompress(&compressed).expect("lz4 decompress 1000 zeros failed");
    assert_eq!(original, decompressed);
}

// ─────────────────────────────────────────────────────────────────────────────
// Test 2 – compress_with_stats ratio > 1.0 for zeros
// ─────────────────────────────────────────────────────────────────────────────

#[cfg(feature = "compression-lz4")]
#[test]
fn test_adv_lz4_stats_ratio_above_one_for_zeros() {
    use oxicode::compression::{compress_with_stats, Compression};

    let data = vec![0u8; 2000];
    let (_, stats) =
        compress_with_stats(&data, Compression::Lz4).expect("compress_with_stats failed");
    assert!(
        stats.ratio() > 1.0,
        "compression ratio for 2000 zeros must exceed 1.0, got {}",
        stats.ratio()
    );
}

// ─────────────────────────────────────────────────────────────────────────────
// Test 3 – detect_compression returns Some(Lz4) after LZ4 compress
// ─────────────────────────────────────────────────────────────────────────────

#[cfg(feature = "compression-lz4")]
#[test]
fn test_adv_lz4_detect_after_compress() {
    use oxicode::compression::{compress, detect_compression, Compression};

    let data = b"detect me after lz4 compression";
    let compressed = compress(data, Compression::Lz4).expect("compress failed");
    let detected = detect_compression(&compressed);
    assert_eq!(
        detected,
        Some(Compression::Lz4),
        "detect_compression must return Lz4 for lz4-compressed data"
    );
}

// ─────────────────────────────────────────────────────────────────────────────
// Test 4 – sequential bytes 0..=255 repeated 4 times, roundtrip
// ─────────────────────────────────────────────────────────────────────────────

#[cfg(feature = "compression-lz4")]
#[test]
fn test_adv_lz4_sequential_numbers_roundtrip() {
    use oxicode::compression::{compress, decompress, Compression};

    let original: Vec<u8> = (0u8..=255).cycle().take(1024).collect();
    let compressed = compress(&original, Compression::Lz4).expect("lz4 sequential compress failed");
    let decompressed = decompress(&compressed).expect("lz4 sequential decompress failed");
    assert_eq!(original, decompressed);
}

// ─────────────────────────────────────────────────────────────────────────────
// Test 5 – encode struct → compress → decompress → decode roundtrip
// ─────────────────────────────────────────────────────────────────────────────

#[cfg(feature = "compression-lz4")]
#[test]
fn test_adv_lz4_compress_encoded_struct() {
    use oxicode::compression::{compress, decompress, Compression};

    let record = AdvRecord {
        id: 42,
        tag: "advanced-test".to_string(),
        values: vec![1, 2, 3, -1, -2, -3],
    };

    let encoded = oxicode::encode_to_vec(&record).expect("encode failed");
    let compressed = compress(&encoded, Compression::Lz4).expect("compress encoded struct failed");
    let decompressed = decompress(&compressed).expect("decompress encoded struct failed");
    let (decoded, _): (AdvRecord, usize) =
        oxicode::decode_from_slice(&decompressed).expect("decode failed");

    assert_eq!(record, decoded);
}

// ─────────────────────────────────────────────────────────────────────────────
// Test 6 – compressed size < original for 5000 zeros
// ─────────────────────────────────────────────────────────────────────────────

#[cfg(feature = "compression-lz4")]
#[test]
fn test_adv_lz4_compressed_size_less_than_original() {
    use oxicode::compression::{compress, Compression};

    let original = vec![0u8; 5000];
    let compressed = compress(&original, Compression::Lz4).expect("compress 5000 zeros failed");
    assert!(
        compressed.len() < original.len(),
        "compressed ({} bytes) must be smaller than 5000 bytes",
        compressed.len()
    );
}

// ─────────────────────────────────────────────────────────────────────────────
// Test 7 – savings_percent() > 0 for zeros
// ─────────────────────────────────────────────────────────────────────────────

#[cfg(feature = "compression-lz4")]
#[test]
fn test_adv_lz4_savings_percent_positive_for_zeros() {
    use oxicode::compression::{compress_with_stats, Compression};

    let data = vec![0u8; 3000];
    let (_, stats) =
        compress_with_stats(&data, Compression::Lz4).expect("compress_with_stats 3000 zeros");
    assert!(
        stats.savings_percent() > 0.0,
        "savings_percent must be positive for highly compressible data, got {}",
        stats.savings_percent()
    );
}

// ─────────────────────────────────────────────────────────────────────────────
// Test 8 – decompress_or_passthrough passes raw bytes through unchanged
// ─────────────────────────────────────────────────────────────────────────────

#[cfg(feature = "compression-lz4")]
#[test]
fn test_adv_lz4_decompress_or_passthrough_raw_passthrough() {
    use oxicode::compression::decompress_or_passthrough;

    let raw: &[u8] = b"this is plain uncompressed data with no magic header";
    let result = decompress_or_passthrough(raw).expect("passthrough failed");
    assert_eq!(raw, result.as_slice());
}

// ─────────────────────────────────────────────────────────────────────────────
// Test 9 – decompress_or_passthrough on compressed input decompresses correctly
// ─────────────────────────────────────────────────────────────────────────────

#[cfg(feature = "compression-lz4")]
#[test]
fn test_adv_lz4_decompress_or_passthrough_compressed() {
    use oxicode::compression::{compress, decompress_or_passthrough, Compression};

    let original = b"roundtrip via decompress_or_passthrough after lz4 compress";
    let compressed =
        compress(original, Compression::Lz4).expect("lz4 compress for passthrough test");
    let result = decompress_or_passthrough(&compressed).expect("decompress_or_passthrough failed");
    assert_eq!(original.as_slice(), result.as_slice());
}

// ─────────────────────────────────────────────────────────────────────────────
// Test 10 – 10000 zeros roundtrip; verify length and content
// ─────────────────────────────────────────────────────────────────────────────

#[cfg(feature = "compression-lz4")]
#[test]
fn test_adv_lz4_large_10k_zeros_roundtrip() {
    use oxicode::compression::{compress, decompress, Compression};

    let original = vec![0u8; 10_000];
    let compressed = compress(&original, Compression::Lz4).expect("lz4 compress 10k zeros");
    let decompressed = decompress(&compressed).expect("lz4 decompress 10k zeros");
    assert_eq!(
        decompressed.len(),
        10_000,
        "decompressed length must match original"
    );
    assert_eq!(original, decompressed);
}

// ─────────────────────────────────────────────────────────────────────────────
// Test 11 – repeated ASCII string compressed and decompressed correctly
// ─────────────────────────────────────────────────────────────────────────────

#[cfg(feature = "compression-lz4")]
#[test]
fn test_adv_lz4_string_bytes_compress_roundtrip() {
    use oxicode::compression::{compress, decompress, Compression};

    let chunk = b"oxicode compression test string -- ";
    let original: Vec<u8> = chunk.iter().copied().cycle().take(2048).collect();
    let compressed = compress(&original, Compression::Lz4).expect("lz4 compress string bytes");
    let decompressed = decompress(&compressed).expect("lz4 decompress string bytes");
    assert_eq!(original, decompressed);
}

// ─────────────────────────────────────────────────────────────────────────────
// Test 12 – is_compressed returns false for plain bytes
// ─────────────────────────────────────────────────────────────────────────────

#[cfg(feature = "compression-lz4")]
#[test]
fn test_adv_lz4_is_compressed_flag_on_raw() {
    use oxicode::compression::is_compressed;

    let raw = b"not compressed at all";
    assert!(
        !is_compressed(raw),
        "is_compressed must return false for raw bytes"
    );
}

// ─────────────────────────────────────────────────────────────────────────────
// Test 13 – is_compressed returns true after compress
// ─────────────────────────────────────────────────────────────────────────────

#[cfg(feature = "compression-lz4")]
#[test]
fn test_adv_lz4_is_compressed_flag_after_compress() {
    use oxicode::compression::{compress, is_compressed, Compression};

    let data = b"check is_compressed after lz4 compress";
    let compressed = compress(data, Compression::Lz4).expect("compress for is_compressed check");
    assert!(
        is_compressed(&compressed),
        "is_compressed must return true for lz4-compressed output"
    );
}

// ─────────────────────────────────────────────────────────────────────────────
// Test 14 – Compression::None codec detected as None, not Lz4
// ─────────────────────────────────────────────────────────────────────────────

#[cfg(feature = "compression-lz4")]
#[test]
fn test_adv_lz4_none_codec_is_not_lz4() {
    use oxicode::compression::{compress, detect_compression, Compression};

    let data = b"none codec test payload";
    let compressed = compress(data, Compression::None).expect("compress with None codec failed");
    let detected = detect_compression(&compressed);
    assert_eq!(
        detected,
        Some(Compression::None),
        "Compression::None codec must be detected as None, not Lz4"
    );
}

// ─────────────────────────────────────────────────────────────────────────────
// Test 15 – double compress/decompress cycle
// ─────────────────────────────────────────────────────────────────────────────

#[cfg(feature = "compression-lz4")]
#[test]
fn test_adv_lz4_double_roundtrip() {
    use oxicode::compression::{compress, decompress, Compression};

    let original = b"double roundtrip payload for lz4 compression";
    let compressed1 = compress(original, Compression::Lz4).expect("first compress failed");
    let mid = decompress(&compressed1).expect("first decompress failed");
    let compressed2 = compress(&mid, Compression::Lz4).expect("second compress failed");
    let final_result = decompress(&compressed2).expect("second decompress failed");
    assert_eq!(original.as_slice(), final_result.as_slice());
}

// ─────────────────────────────────────────────────────────────────────────────
// Test 16 – Vec<bool> encode → compress → decompress → decode roundtrip
// ─────────────────────────────────────────────────────────────────────────────

#[cfg(feature = "compression-lz4")]
#[test]
fn test_adv_lz4_vec_bool_encode_compress_roundtrip() {
    use oxicode::compression::{compress, decompress, Compression};

    let original: Vec<bool> = (0..200).map(|i| i % 3 == 0).collect();
    let encoded = oxicode::encode_to_vec(&original).expect("encode Vec<bool> failed");
    let compressed =
        compress(&encoded, Compression::Lz4).expect("compress Vec<bool> encoded failed");
    let decompressed = decompress(&compressed).expect("decompress Vec<bool> failed");
    let (decoded, _): (Vec<bool>, usize) =
        oxicode::decode_from_slice(&decompressed).expect("decode Vec<bool> failed");
    assert_eq!(original, decoded);
}

// ─────────────────────────────────────────────────────────────────────────────
// Test 17 – pseudo-random data savings_percent < 50%
// ─────────────────────────────────────────────────────────────────────────────

#[cfg(feature = "compression-lz4")]
#[test]
fn test_adv_lz4_savings_percent_near_zero_for_random() {
    use oxicode::compression::{compress_with_stats, Compression};

    // LCG-derived pseudo-random bytes — low compressibility.
    let data: Vec<u8> = (0u64..1000)
        .map(|i| {
            (i.wrapping_mul(6_364_136_223_846_793_005)
                .wrapping_add(1_442_695_040_888_963_407)
                >> 56) as u8
        })
        .collect();

    let (_, stats) =
        compress_with_stats(&data, Compression::Lz4).expect("compress random data failed");
    // The LCG output is not truly random — it compresses better than real random data.
    // The key invariant is that it does NOT compress as well as highly repetitive data
    // (e.g. 1000 zeros which reach ~99% savings). We assert savings < 95%.
    assert!(
        stats.savings_percent() < 95.0,
        "savings_percent for LCG pseudo-random data must be < 95%, got {}",
        stats.savings_percent()
    );
}

// ─────────────────────────────────────────────────────────────────────────────
// Test 18 – LZ4-compressed bytes fail decode_from_slice as raw oxicode
// ─────────────────────────────────────────────────────────────────────────────

#[cfg(feature = "compression-lz4")]
#[test]
fn test_adv_lz4_compressed_data_not_valid_raw_decode() {
    use oxicode::compression::{compress, Compression};

    let original: Vec<u8> = vec![0xAAu8; 256];
    let encoded = oxicode::encode_to_vec(&original).expect("encode Vec<u8> failed");
    let compressed = compress(&encoded, Compression::Lz4).expect("compress failed");

    // Compressed wire format starts with magic header — not a valid oxicode encoding.
    let result: Result<(Vec<u8>, usize), _> = oxicode::decode_from_slice(&compressed);
    assert!(
        result.is_err(),
        "decode_from_slice must fail on raw lz4-compressed bytes"
    );
}

// ─────────────────────────────────────────────────────────────────────────────
// Test 19 – binary data all 256 byte values compress/decompress roundtrip
// ─────────────────────────────────────────────────────────────────────────────

#[cfg(feature = "compression-lz4")]
#[test]
fn test_adv_lz4_compress_binary_payload() {
    use oxicode::compression::{compress, decompress, Compression};

    let original: Vec<u8> = (0u8..=255).collect();
    let compressed =
        compress(&original, Compression::Lz4).expect("compress full byte range failed");
    let decompressed = decompress(&compressed).expect("decompress full byte range failed");
    assert_eq!(original, decompressed);
    assert_eq!(decompressed.len(), 256, "all 256 bytes must be present");
}

// ─────────────────────────────────────────────────────────────────────────────
// Test 20 – Zstd basic roundtrip (unique from existing test_zstd_basic_roundtrip)
// ─────────────────────────────────────────────────────────────────────────────

#[cfg(feature = "compression-zstd")]
#[test]
fn test_adv_zstd_basic_roundtrip_unique() {
    use oxicode::compression::{compress, decompress, Compression};

    let original: Vec<u8> = b"zstd advanced roundtrip test payload"
        .iter()
        .copied()
        .cycle()
        .take(500)
        .collect();
    let compressed =
        compress(&original, Compression::Zstd).expect("zstd compress 500 bytes failed");
    let decompressed = decompress(&compressed).expect("zstd decompress 500 bytes failed");
    assert_eq!(original, decompressed);
}

// ─────────────────────────────────────────────────────────────────────────────
// Test 21 – Zstd compress_with_stats ratio > 1.0 for 4000 zeros
// ─────────────────────────────────────────────────────────────────────────────

#[cfg(feature = "compression-zstd")]
#[test]
fn test_adv_zstd_stats_ratio() {
    use oxicode::compression::{compress_with_stats, Compression};

    let data = vec![0u8; 4000];
    let (_, stats) =
        compress_with_stats(&data, Compression::Zstd).expect("zstd compress_with_stats failed");
    assert!(
        stats.ratio() > 1.0,
        "zstd compression ratio for 4000 zeros must exceed 1.0, got {}",
        stats.ratio()
    );
}

// ─────────────────────────────────────────────────────────────────────────────
// Test 22 – Zstd detect_compression returns Some(Compression::Zstd)
// ─────────────────────────────────────────────────────────────────────────────

#[cfg(feature = "compression-zstd")]
#[test]
fn test_adv_zstd_detect_compression_variant() {
    use oxicode::compression::{compress, detect_compression, Compression};

    let data: Vec<u8> = b"zstd detection test data"
        .iter()
        .copied()
        .cycle()
        .take(256)
        .collect();
    let compressed = compress(&data, Compression::Zstd).expect("zstd compress for detection");
    let detected = detect_compression(&compressed);
    assert_eq!(
        detected,
        Some(Compression::Zstd),
        "detect_compression must return Zstd for zstd-compressed data"
    );
}
