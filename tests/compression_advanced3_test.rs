//! Advanced compression tests for OxiCode — series 3.
//!
//! These 22 tests cover angles not exercised by any of the preceding suites:
//! 1 MB zeros, corrupt-magic at each of the three magic bytes, boundary-size
//! payloads (3/4/5/7/8/9 bytes), u8::MAX single-byte roundtrip, Vec<i64> with
//! negative values, HashMap encode+compress, CompressionStats default/expansion
//! edge-cases, sawtooth pattern, multi-level Zstd comparison, detect on short
//! slices, double-nested compress, and more.

// ─────────────────────────────────────────────────────────────────────────────
// Test 1 – 1 MB of zeros roundtrip; compressed output must be tiny
// ─────────────────────────────────────────────────────────────────────────────

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
#[test]
fn test_adv3_lz4_1mb_zeros_roundtrip() {
    use oxicode::compression::{compress, decompress, Compression};

    let original = vec![0u8; 1_048_576]; // 1 MiB
    let compressed = compress(&original, Compression::Lz4).expect("lz4 compress 1 MB zeros failed");
    assert!(
        compressed.len() < 10_000,
        "1 MB of zeros must compress to under 10 000 bytes via LZ4, got {} bytes",
        compressed.len()
    );
    let decompressed = decompress(&compressed).expect("lz4 decompress 1 MB zeros failed");
    assert_eq!(
        decompressed.len(),
        1_048_576,
        "decompressed length must be 1 048 576, got {}",
        decompressed.len()
    );
    assert_eq!(
        original, decompressed,
        "1 MB zeros must survive LZ4 roundtrip byte-for-byte"
    );
}

// ─────────────────────────────────────────────────────────────────────────────
// Test 2 – corrupt first magic byte (index 0) → decompress returns Err
// ─────────────────────────────────────────────────────────────────────────────

#[cfg(feature = "compression-lz4")]
#[test]
fn test_adv3_lz4_corrupt_magic_byte_0_returns_error() {
    use oxicode::compression::{compress, decompress, Compression};

    let data = b"payload for magic[0] corruption test";
    let mut tampered = compress(data, Compression::Lz4).expect("compress for magic[0] test");
    tampered[0] ^= 0xFF; // flip bits of first magic byte
    let result = decompress(&tampered);
    assert!(
        result.is_err(),
        "decompress must fail when magic byte[0] is corrupted"
    );
}

// ─────────────────────────────────────────────────────────────────────────────
// Test 3 – corrupt second magic byte (index 1) → decompress returns Err
// ─────────────────────────────────────────────────────────────────────────────

#[cfg(feature = "compression-lz4")]
#[test]
fn test_adv3_lz4_corrupt_magic_byte_1_returns_error() {
    use oxicode::compression::{compress, decompress, Compression};

    let data = b"payload for magic[1] corruption test";
    let mut tampered = compress(data, Compression::Lz4).expect("compress for magic[1] test");
    tampered[1] ^= 0xFF;
    let result = decompress(&tampered);
    assert!(
        result.is_err(),
        "decompress must fail when magic byte[1] is corrupted"
    );
}

// ─────────────────────────────────────────────────────────────────────────────
// Test 4 – corrupt third magic byte (index 2) → decompress returns Err
// ─────────────────────────────────────────────────────────────────────────────

#[cfg(feature = "compression-lz4")]
#[test]
fn test_adv3_lz4_corrupt_magic_byte_2_returns_error() {
    use oxicode::compression::{compress, decompress, Compression};

    let data = b"payload for magic[2] corruption test";
    let mut tampered = compress(data, Compression::Lz4).expect("compress for magic[2] test");
    tampered[2] ^= 0xFF;
    let result = decompress(&tampered);
    assert!(
        result.is_err(),
        "decompress must fail when magic byte[2] is corrupted"
    );
}

// ─────────────────────────────────────────────────────────────────────────────
// Test 5 – boundary payload sizes 3, 4, 5, 7, 8, 9 bytes all roundtrip
// ─────────────────────────────────────────────────────────────────────────────

#[cfg(feature = "compression-lz4")]
#[test]
fn test_adv3_lz4_boundary_sizes_roundtrip() {
    use oxicode::compression::{compress, decompress, Compression};

    for size in [3usize, 4, 5, 7, 8, 9] {
        let original: Vec<u8> = (0u8..).take(size).collect();
        let compressed = compress(&original, Compression::Lz4)
            .unwrap_or_else(|e| panic!("lz4 compress size={size} failed: {e}"));
        let decompressed = decompress(&compressed)
            .unwrap_or_else(|e| panic!("lz4 decompress size={size} failed: {e}"));
        assert_eq!(
            original, decompressed,
            "payload of {size} bytes must survive LZ4 roundtrip"
        );
    }
}

// ─────────────────────────────────────────────────────────────────────────────
// Test 6 – single byte u8::MAX (0xFF) roundtrip
// ─────────────────────────────────────────────────────────────────────────────

#[cfg(feature = "compression-lz4")]
#[test]
fn test_adv3_lz4_single_byte_u8_max_roundtrip() {
    use oxicode::compression::{compress, decompress, Compression};

    let original = [u8::MAX];
    let compressed = compress(&original, Compression::Lz4).expect("lz4 compress u8::MAX failed");
    let decompressed = decompress(&compressed).expect("lz4 decompress u8::MAX failed");
    assert_eq!(
        decompressed.len(),
        1,
        "decompressed single u8::MAX must have length 1, got {}",
        decompressed.len()
    );
    assert_eq!(
        decompressed[0],
        u8::MAX,
        "decompressed value must be 0xFF (u8::MAX)"
    );
}

// ─────────────────────────────────────────────────────────────────────────────
// Test 7 – Vec<i64> with negative values encode+compress+decompress+decode
// ─────────────────────────────────────────────────────────────────────────────

#[cfg(feature = "compression-lz4")]
#[test]
fn test_adv3_lz4_vec_i64_negative_values_roundtrip() {
    use oxicode::compression::{compress, decompress, Compression};

    let original: Vec<i64> = (-500i64..=500).collect();
    let encoded = oxicode::encode_to_vec(&original).expect("encode Vec<i64> with negatives failed");
    let compressed = compress(&encoded, Compression::Lz4).expect("lz4 compress Vec<i64> failed");
    let decompressed = decompress(&compressed).expect("lz4 decompress Vec<i64> failed");
    let (decoded, _): (Vec<i64>, usize) =
        oxicode::decode_from_slice(&decompressed).expect("decode Vec<i64> failed");
    assert_eq!(
        original, decoded,
        "Vec<i64> with negative values must survive encode+lz4+decode roundtrip"
    );
}

// ─────────────────────────────────────────────────────────────────────────────
// Test 8 – sawtooth byte pattern (0..=255 once) roundtrip
// ─────────────────────────────────────────────────────────────────────────────

#[cfg(feature = "compression-lz4")]
#[test]
fn test_adv3_lz4_sawtooth_pattern_roundtrip() {
    use oxicode::compression::{compress, decompress, Compression};

    // One full sawtooth: 0x00, 0x01, ..., 0xFF — exactly 256 bytes
    let original: Vec<u8> = (0u8..=255).collect();
    let compressed = compress(&original, Compression::Lz4).expect("lz4 compress sawtooth failed");
    let decompressed = decompress(&compressed).expect("lz4 decompress sawtooth failed");
    assert_eq!(
        original, decompressed,
        "sawtooth 0..=255 pattern must survive LZ4 roundtrip"
    );
    assert_eq!(
        decompressed.len(),
        256,
        "decompressed sawtooth must have 256 bytes, got {}",
        decompressed.len()
    );
}

// ─────────────────────────────────────────────────────────────────────────────
// Test 9 – detect_compression on slice shorter than 5 bytes returns None
// ─────────────────────────────────────────────────────────────────────────────

#[cfg(feature = "compression-lz4")]
#[test]
fn test_adv3_detect_compression_on_short_slice_returns_none() {
    use oxicode::compression::detect_compression;

    for len in 0usize..5 {
        let short: Vec<u8> = (0u8..).take(len).collect();
        let result = detect_compression(&short);
        assert!(
            result.is_none(),
            "detect_compression on {len}-byte slice must return None, got Some(_)"
        );
    }
}

// ─────────────────────────────────────────────────────────────────────────────
// Test 10 – is_compressed on exactly-5-byte valid header with empty payload
// ─────────────────────────────────────────────────────────────────────────────

#[cfg(feature = "compression-lz4")]
#[test]
fn test_adv3_is_compressed_exact_5_byte_header_empty_payload() {
    use oxicode::compression::{compress, is_compressed, Compression};

    // Compress empty slice to get a valid 5-byte header + nothing
    let compressed = compress(&[], Compression::Lz4).expect("compress empty slice failed");
    // The result is exactly HEADER_SIZE (5) bytes when payload is empty
    assert!(
        is_compressed(&compressed),
        "compress output of empty slice must be detected as compressed"
    );
}

// ─────────────────────────────────────────────────────────────────────────────
// Test 11 – double-nested compress: compress(compress(data)) roundtrip
// ─────────────────────────────────────────────────────────────────────────────

#[cfg(feature = "compression-lz4")]
#[test]
fn test_adv3_lz4_double_nested_compress_roundtrip() {
    use oxicode::compression::{compress, decompress, Compression};

    let original: Vec<u8> = b"double nested compression test payload oxicode"
        .iter()
        .copied()
        .cycle()
        .take(1024)
        .collect();
    // First compression layer
    let layer1 = compress(&original, Compression::Lz4).expect("layer1 compress failed");
    // Second compression layer (compress the already-compressed data)
    let layer2 = compress(&layer1, Compression::Lz4).expect("layer2 compress failed");
    // Peel both layers off
    let mid = decompress(&layer2).expect("layer2 decompress failed");
    let recovered = decompress(&mid).expect("layer1 decompress failed");
    assert_eq!(
        original, recovered,
        "double-nested LZ4 compress/decompress must recover original data"
    );
}

// ─────────────────────────────────────────────────────────────────────────────
// Test 12 – CompressionStats default() yields all-zero fields
// ─────────────────────────────────────────────────────────────────────────────

#[test]
fn test_adv3_compression_stats_default_is_zero() {
    use oxicode::compression::CompressionStats;

    let stats = CompressionStats::default();
    assert_eq!(
        stats.original_size, 0,
        "default CompressionStats.original_size must be 0"
    );
    assert_eq!(
        stats.compressed_size, 0,
        "default CompressionStats.compressed_size must be 0"
    );
    assert_eq!(
        stats.ratio(),
        0.0,
        "ratio() on zero-sized default stats must return 0.0"
    );
    assert_eq!(
        stats.savings_percent(),
        0.0,
        "savings_percent() on zero-sized default stats must return 0.0"
    );
}

// ─────────────────────────────────────────────────────────────────────────────
// Test 13 – CompressionStats when compressed_size > original_size (expansion)
// ─────────────────────────────────────────────────────────────────────────────

#[test]
fn test_adv3_compression_stats_expansion_scenario() {
    use oxicode::compression::CompressionStats;

    // Simulate a scenario where compression expanded the data (ratio < 1.0)
    let stats = CompressionStats {
        original_size: 10,
        compressed_size: 20,
    };
    assert!(
        stats.ratio() < 1.0,
        "ratio must be < 1.0 when compressed_size > original_size, got {}",
        stats.ratio()
    );
    assert!(
        stats.savings_percent() < 0.0,
        "savings_percent must be negative when compressed_size > original_size, got {}",
        stats.savings_percent()
    );
}

// ─────────────────────────────────────────────────────────────────────────────
// Test 14 – Compression::None.name() is "none"; LZ4.is_none() is false
// ─────────────────────────────────────────────────────────────────────────────

#[cfg(feature = "compression-lz4")]
#[test]
fn test_adv3_none_compression_metadata() {
    use oxicode::compression::Compression;

    assert_eq!(
        Compression::None.name(),
        "none",
        "Compression::None.name() must return \"none\""
    );
    assert!(
        Compression::None.is_none(),
        "Compression::None.is_none() must be true"
    );
    assert!(
        !Compression::Lz4.is_none(),
        "Compression::Lz4.is_none() must be false"
    );
    assert_eq!(
        Compression::Lz4.name(),
        "lz4",
        "Compression::Lz4.name() must return \"lz4\""
    );
}

// ─────────────────────────────────────────────────────────────────────────────
// Test 15 – LZ4 compress Vec<f32> values roundtrip via encode/decode
// ─────────────────────────────────────────────────────────────────────────────

#[cfg(feature = "compression-lz4")]
#[test]
fn test_adv3_lz4_vec_f32_encode_compress_roundtrip() {
    use oxicode::compression::{compress, decompress, Compression};

    // Use a known sequence of f32 values (avoid NaN for PartialEq comparison)
    let original: Vec<f32> = (0u32..200).map(|i| (i as f32) * 0.1_f32).collect();
    let encoded = oxicode::encode_to_vec(&original).expect("encode Vec<f32> failed");
    let compressed = compress(&encoded, Compression::Lz4).expect("lz4 compress Vec<f32> failed");
    let decompressed = decompress(&compressed).expect("lz4 decompress Vec<f32> failed");
    let (decoded, _): (Vec<f32>, usize) =
        oxicode::decode_from_slice(&decompressed).expect("decode Vec<f32> failed");
    assert_eq!(
        original.len(),
        decoded.len(),
        "decoded Vec<f32> must have same length as original"
    );
    for (i, (a, b)) in original.iter().zip(decoded.iter()).enumerate() {
        assert_eq!(
            a.to_bits(),
            b.to_bits(),
            "Vec<f32>[{i}]: bit pattern must match after LZ4 roundtrip"
        );
    }
}

// ─────────────────────────────────────────────────────────────────────────────
// Test 16 – LZ4 compress/decompress binary data with all byte values × 4
// ─────────────────────────────────────────────────────────────────────────────

#[cfg(feature = "compression-lz4")]
#[test]
fn test_adv3_lz4_all_bytes_times_4_roundtrip() {
    use oxicode::compression::{compress, decompress, Compression};

    // Each byte value appears exactly 4 times — 1024 bytes total
    let original: Vec<u8> = (0u8..=255).flat_map(|b| [b, b, b, b]).collect();
    assert_eq!(original.len(), 1024, "precondition: 1024 bytes");
    let compressed =
        compress(&original, Compression::Lz4).expect("lz4 compress all-bytes×4 failed");
    let decompressed = decompress(&compressed).expect("lz4 decompress all-bytes×4 failed");
    assert_eq!(
        original, decompressed,
        "all byte values × 4 must survive LZ4 roundtrip"
    );
}

// ─────────────────────────────────────────────────────────────────────────────
// Test 17 – LZ4 compressed output header contains correct codec byte (1)
// ─────────────────────────────────────────────────────────────────────────────

#[cfg(feature = "compression-lz4")]
#[test]
fn test_adv3_lz4_header_codec_byte_value() {
    use oxicode::compression::{compress, Compression};

    let data = b"codec byte value check for LZ4 header";
    let compressed = compress(data, Compression::Lz4).expect("lz4 compress for header check");
    // Byte at index 4 is the codec ID — LZ4 must be 1
    assert_eq!(
        compressed[4], 1u8,
        "LZ4 codec byte at index 4 must be 1, got {}",
        compressed[4]
    );
    // Bytes 0..3 are magic + version; total header size is 5
    assert!(
        compressed.len() >= 5,
        "compressed output must be at least 5 bytes (header), got {}",
        compressed.len()
    );
}

// ─────────────────────────────────────────────────────────────────────────────
// Test 18 – Compression::None encoded size = original + 5 (header only)
// ─────────────────────────────────────────────────────────────────────────────

#[cfg(feature = "compression-lz4")]
#[test]
fn test_adv3_none_codec_output_size_equals_input_plus_header() {
    use oxicode::compression::{compress, Compression};

    let data: Vec<u8> = (0u8..100).collect();
    let wrapped = compress(&data, Compression::None).expect("compress(None) 100-byte data failed");
    assert_eq!(
        wrapped.len(),
        data.len() + 5,
        "Compression::None output must be exactly input_len+5 bytes, got {} (expected {})",
        wrapped.len(),
        data.len() + 5
    );
}

// ─────────────────────────────────────────────────────────────────────────────
// Test 19 – Zstd ZstdLevel(22) roundtrip (maximum compression level)
// ─────────────────────────────────────────────────────────────────────────────

#[cfg(feature = "compression-zstd")]
#[test]
fn test_adv3_zstd_level22_max_roundtrip() {
    use oxicode::compression::{compress, decompress, Compression};

    let original: Vec<u8> = b"maximum zstd compression level 22 roundtrip test"
        .iter()
        .copied()
        .cycle()
        .take(8_192)
        .collect();
    let compressed =
        compress(&original, Compression::ZstdLevel(22)).expect("ZstdLevel(22) compress failed");
    let decompressed = decompress(&compressed).expect("ZstdLevel(22) decompress failed");
    assert_eq!(
        original, decompressed,
        "data must survive Zstd ZstdLevel(22) roundtrip"
    );
}

// ─────────────────────────────────────────────────────────────────────────────
// Test 20 – Zstd level 1 vs level 9 both decode to identical output
// ─────────────────────────────────────────────────────────────────────────────

#[cfg(feature = "compression-zstd")]
#[test]
fn test_adv3_zstd_level1_and_level9_decode_identical() {
    use oxicode::compression::{compress, decompress, Compression};

    let original: Vec<u8> = b"compare zstd level 1 vs level 9 decompressed output"
        .iter()
        .copied()
        .cycle()
        .take(4_096)
        .collect();
    let compressed_l1 =
        compress(&original, Compression::ZstdLevel(1)).expect("ZstdLevel(1) compress failed");
    let compressed_l9 =
        compress(&original, Compression::ZstdLevel(9)).expect("ZstdLevel(9) compress failed");
    let decoded_l1 = decompress(&compressed_l1).expect("ZstdLevel(1) decompress failed");
    let decoded_l9 = decompress(&compressed_l9).expect("ZstdLevel(9) decompress failed");
    assert_eq!(
        decoded_l1, decoded_l9,
        "ZstdLevel(1) and ZstdLevel(9) must decompress to identical output"
    );
    assert_eq!(
        decoded_l1, original,
        "ZstdLevel(1) output must match original"
    );
}

// ─────────────────────────────────────────────────────────────────────────────
// Test 21 – Zstd codec byte in header is 2
// ─────────────────────────────────────────────────────────────────────────────

#[cfg(feature = "compression-zstd")]
#[test]
fn test_adv3_zstd_header_codec_byte_is_2() {
    use oxicode::compression::{compress, Compression};

    let data = b"zstd codec id header check";
    let compressed = compress(data, Compression::Zstd).expect("zstd compress for codec check");
    assert_eq!(
        compressed[4], 2u8,
        "Zstd codec byte at header index 4 must be 2, got {}",
        compressed[4]
    );
}

// ─────────────────────────────────────────────────────────────────────────────
// Test 22 – Zstd savings_percent for incompressible LCG data is < 50%
// ─────────────────────────────────────────────────────────────────────────────

#[cfg(feature = "compression-zstd")]
#[test]
fn test_adv3_zstd_savings_percent_low_for_incompressible_data() {
    use oxicode::compression::{compress_with_stats, Compression};

    // LCG pseudo-random bytes — low compressibility
    let mut state: u64 = 0xFEED_FACE_CAFE_BABE;
    let data: Vec<u8> = (0..2_048)
        .map(|_| {
            state = state
                .wrapping_mul(6_364_136_223_846_793_005)
                .wrapping_add(1_442_695_040_888_963_407);
            (state >> 56) as u8
        })
        .collect();
    let (_, stats) =
        compress_with_stats(&data, Compression::Zstd).expect("zstd compress_with_stats failed");
    assert!(
        stats.savings_percent() < 50.0,
        "Zstd savings_percent for LCG pseudo-random data must be < 50%, got {}",
        stats.savings_percent()
    );
}
