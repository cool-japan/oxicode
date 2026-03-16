//! Advanced compression tests for OxiCode — series 2.
//!
//! These tests cover angles not exercised by compression_test.rs or
//! compression_advanced_test.rs: header structure internals, codec metadata
//! helpers, single-byte payloads, all-0xFF data, determinism, corruption
//! detection at specific byte offsets, stats field semantics, and more.

// ─────────────────────────────────────────────────────────────────────────────
// Test 1 – compress a single raw byte (0x42) and recover it exactly
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
fn test_adv2_lz4_single_byte_roundtrip() {
    use oxicode::compression::{compress, decompress, Compression};

    let original = [0x42u8];
    let compressed =
        compress(&original, Compression::Lz4).expect("lz4 compress single byte failed");
    let decompressed = decompress(&compressed).expect("lz4 decompress single byte failed");
    assert_eq!(decompressed.len(), 1, "decompressed length must be 1");
    assert_eq!(
        decompressed[0], 0x42,
        "decompressed value must match original"
    );
}

// ─────────────────────────────────────────────────────────────────────────────
// Test 2 – compress an empty raw slice and recover it exactly
// ─────────────────────────────────────────────────────────────────────────────

#[cfg(feature = "compression-lz4")]
#[test]
fn test_adv2_lz4_empty_raw_slice_roundtrip() {
    use oxicode::compression::{compress, decompress, Compression};

    let original: &[u8] = &[];
    let compressed = compress(original, Compression::Lz4).expect("lz4 compress empty slice failed");
    let decompressed = decompress(&compressed).expect("lz4 decompress empty slice failed");
    assert!(
        decompressed.is_empty(),
        "decompressed empty slice must itself be empty, got {} bytes",
        decompressed.len()
    );
}

// ─────────────────────────────────────────────────────────────────────────────
// Test 3 – header magic bytes are exactly [0x4F, 0x58, 0x43] ('O','X','C')
// ─────────────────────────────────────────────────────────────────────────────

#[cfg(feature = "compression-lz4")]
#[test]
fn test_adv2_lz4_header_magic_bytes() {
    use oxicode::compression::{compress, Compression};

    let data = b"magic byte inspection";
    let compressed = compress(data, Compression::Lz4).expect("compress for magic check");
    assert_eq!(
        &compressed[0..3],
        &[0x4F, 0x58, 0x43],
        "first three bytes must be OXC magic"
    );
}

// ─────────────────────────────────────────────────────────────────────────────
// Test 4 – header version byte is 1
// ─────────────────────────────────────────────────────────────────────────────

#[cfg(feature = "compression-lz4")]
#[test]
fn test_adv2_lz4_header_version_byte() {
    use oxicode::compression::{compress, Compression};

    let data = b"version byte inspection";
    let compressed = compress(data, Compression::Lz4).expect("compress for version byte check");
    assert_eq!(
        compressed[3], 1,
        "version byte at index 3 must be 1, got {}",
        compressed[3]
    );
}

// ─────────────────────────────────────────────────────────────────────────────
// Test 5 – Compression::None codec byte is 0; Lz4 codec byte is 1
// ─────────────────────────────────────────────────────────────────────────────

#[cfg(feature = "compression-lz4")]
#[test]
fn test_adv2_codec_id_in_header() {
    use oxicode::compression::{compress, Compression};

    let data = b"codec id check";
    let none_compressed = compress(data, Compression::None).expect("compress with None failed");
    let lz4_compressed = compress(data, Compression::Lz4).expect("compress with Lz4 failed");

    assert_eq!(
        none_compressed[4], 0,
        "Compression::None codec byte must be 0"
    );
    assert_eq!(
        lz4_compressed[4], 1,
        "Compression::Lz4 codec byte must be 1"
    );
}

// ─────────────────────────────────────────────────────────────────────────────
// Test 6 – Compression::None.is_none() returns true; Lz4.is_none() returns false
// ─────────────────────────────────────────────────────────────────────────────

#[cfg(feature = "compression-lz4")]
#[test]
fn test_adv2_is_none_helper() {
    use oxicode::compression::Compression;

    assert!(
        Compression::None.is_none(),
        "Compression::None.is_none() must return true"
    );
    assert!(
        !Compression::Lz4.is_none(),
        "Compression::Lz4.is_none() must return false"
    );
}

// ─────────────────────────────────────────────────────────────────────────────
// Test 7 – Compression::name() returns correct string literals
// ─────────────────────────────────────────────────────────────────────────────

#[cfg(feature = "compression-lz4")]
#[test]
fn test_adv2_compression_name_strings() {
    use oxicode::compression::Compression;

    assert_eq!(Compression::None.name(), "none");
    assert_eq!(Compression::Lz4.name(), "lz4");
}

// ─────────────────────────────────────────────────────────────────────────────
// Test 8 – all-0xFF bytes (256 of them) roundtrip
// ─────────────────────────────────────────────────────────────────────────────

#[cfg(feature = "compression-lz4")]
#[test]
fn test_adv2_lz4_all_0xff_roundtrip() {
    use oxicode::compression::{compress, decompress, Compression};

    let original = vec![0xFFu8; 256];
    let compressed = compress(&original, Compression::Lz4).expect("lz4 compress all-0xFF failed");
    let decompressed = decompress(&compressed).expect("lz4 decompress all-0xFF failed");
    assert_eq!(original, decompressed);
}

// ─────────────────────────────────────────────────────────────────────────────
// Test 9 – alternating 0x55/0xAA pattern (2000 bytes) roundtrip
// ─────────────────────────────────────────────────────────────────────────────

#[cfg(feature = "compression-lz4")]
#[test]
fn test_adv2_lz4_alternating_pattern_roundtrip() {
    use oxicode::compression::{compress, decompress, Compression};

    let original: Vec<u8> = (0u16..2000)
        .map(|i| if i % 2 == 0 { 0x55 } else { 0xAA })
        .collect();
    let compressed =
        compress(&original, Compression::Lz4).expect("lz4 compress alternating pattern failed");
    let decompressed = decompress(&compressed).expect("lz4 decompress alternating pattern failed");
    assert_eq!(original, decompressed);
}

// ─────────────────────────────────────────────────────────────────────────────
// Test 10 – 100 000 zero bytes roundtrip (larger than existing 10k test)
// ─────────────────────────────────────────────────────────────────────────────

#[cfg(feature = "compression-lz4")]
#[test]
fn test_adv2_lz4_100k_zeros_roundtrip() {
    use oxicode::compression::{compress, decompress, Compression};

    let original = vec![0u8; 100_000];
    let compressed = compress(&original, Compression::Lz4).expect("lz4 compress 100k zeros failed");
    let decompressed = decompress(&compressed).expect("lz4 decompress 100k zeros failed");
    assert_eq!(
        decompressed.len(),
        100_000,
        "decompressed length must be 100 000"
    );
    assert!(
        compressed.len() < 1_000,
        "100k zeros must compress to under 1000 bytes, got {} bytes",
        compressed.len()
    );
    assert_eq!(original, decompressed);
}

// ─────────────────────────────────────────────────────────────────────────────
// Test 11 – truncate the compressed output to half its length → decompress Err
// ─────────────────────────────────────────────────────────────────────────────

#[cfg(feature = "compression-lz4")]
#[test]
fn test_adv2_lz4_truncated_payload_returns_error() {
    use oxicode::compression::{compress, decompress, Compression};

    // Use a large, varied payload so the compressed stream is meaningfully long.
    let original: Vec<u8> = (0u8..=255).cycle().take(2048).collect();
    let compressed = compress(&original, Compression::Lz4).expect("compress for truncation test");
    // Keep only the header (5 bytes) plus a few payload bytes — not enough to decode.
    let truncated = &compressed[..8.min(compressed.len())];
    let result = decompress(truncated);
    assert!(
        result.is_err(),
        "decompress must fail on truncated payload (kept {} of {} bytes)",
        truncated.len(),
        compressed.len()
    );
}

// ─────────────────────────────────────────────────────────────────────────────
// Test 12 – wrong version byte in header returns Err
// ─────────────────────────────────────────────────────────────────────────────

#[cfg(feature = "compression-lz4")]
#[test]
fn test_adv2_wrong_version_byte_returns_error() {
    use oxicode::compression::{compress, decompress, Compression};

    let data = b"version byte tampering test";
    let mut tampered = compress(data, Compression::Lz4).expect("compress for version test");
    // Overwrite version byte (index 3) with an unsupported value.
    tampered[3] = 99;
    let result = decompress(&tampered);
    assert!(
        result.is_err(),
        "decompress must fail on wrong version byte"
    );
}

// ─────────────────────────────────────────────────────────────────────────────
// Test 13 – unknown codec byte in header returns Err
// ─────────────────────────────────────────────────────────────────────────────

#[cfg(feature = "compression-lz4")]
#[test]
fn test_adv2_unknown_codec_byte_returns_error() {
    use oxicode::compression::{compress, decompress, Compression};

    let data = b"unknown codec byte test";
    let mut tampered = compress(data, Compression::Lz4).expect("compress for codec test");
    // Overwrite codec byte (index 4) with an unknown id.
    tampered[4] = 127;
    let result = decompress(&tampered);
    assert!(
        result.is_err(),
        "decompress must fail on unknown codec byte"
    );
}

// ─────────────────────────────────────────────────────────────────────────────
// Test 14 – decompress of a zero-length slice returns Err
// ─────────────────────────────────────────────────────────────────────────────

#[cfg(feature = "compression-lz4")]
#[test]
fn test_adv2_decompress_zero_length_returns_error() {
    use oxicode::compression::decompress;

    let result = decompress(&[]);
    assert!(result.is_err(), "decompress of empty slice must return Err");
}

// ─────────────────────────────────────────────────────────────────────────────
// Test 15 – compress_with_stats: original_size field equals input length
// ─────────────────────────────────────────────────────────────────────────────

#[cfg(feature = "compression-lz4")]
#[test]
fn test_adv2_stats_original_size_field() {
    use oxicode::compression::{compress_with_stats, Compression};

    let data: Vec<u8> = (0u8..200).collect();
    let (_, stats) =
        compress_with_stats(&data, Compression::Lz4).expect("compress_with_stats failed");
    assert_eq!(
        stats.original_size, 200,
        "stats.original_size must equal the input length (200), got {}",
        stats.original_size
    );
}

// ─────────────────────────────────────────────────────────────────────────────
// Test 16 – compress_with_stats: compressed_size field equals returned vec length
// ─────────────────────────────────────────────────────────────────────────────

#[cfg(feature = "compression-lz4")]
#[test]
fn test_adv2_stats_compressed_size_field_matches_vec() {
    use oxicode::compression::{compress_with_stats, Compression};

    let data = vec![0xBBu8; 400];
    let (compressed_vec, stats) =
        compress_with_stats(&data, Compression::Lz4).expect("compress_with_stats failed");
    assert_eq!(
        stats.compressed_size,
        compressed_vec.len(),
        "stats.compressed_size must match the length of the returned Vec"
    );
}

// ─────────────────────────────────────────────────────────────────────────────
// Test 17 – compression is deterministic: two calls produce identical output
// ─────────────────────────────────────────────────────────────────────────────

#[cfg(feature = "compression-lz4")]
#[test]
fn test_adv2_lz4_deterministic_output() {
    use oxicode::compression::{compress, Compression};

    let data: Vec<u8> = (0u8..=255).cycle().take(1024).collect();
    let first = compress(&data, Compression::Lz4).expect("first compress call failed");
    let second = compress(&data, Compression::Lz4).expect("second compress call failed");
    assert_eq!(
        first, second,
        "two compress calls on the same input must produce identical output"
    );
}

// ─────────────────────────────────────────────────────────────────────────────
// Test 18 – Vec<String> encode→compress→decompress→decode roundtrip
// ─────────────────────────────────────────────────────────────────────────────

#[cfg(feature = "compression-lz4")]
#[test]
fn test_adv2_lz4_vec_string_encode_compress_roundtrip() {
    use oxicode::compression::{compress, decompress, Compression};

    let original: Vec<String> = (0u32..50)
        .map(|i| format!("item-{:04}-padding-to-make-it-longer", i))
        .collect();
    let encoded = oxicode::encode_to_vec(&original).expect("encode Vec<String> failed");
    let compressed =
        compress(&encoded, Compression::Lz4).expect("compress Vec<String> encoded failed");
    let decompressed = decompress(&compressed).expect("decompress Vec<String> failed");
    let (decoded, _): (Vec<String>, usize) =
        oxicode::decode_from_slice(&decompressed).expect("decode Vec<String> failed");
    assert_eq!(original, decoded);
}

// ─────────────────────────────────────────────────────────────────────────────
// Test 19 – deeply nested tuple encode→compress→decompress→decode
// ─────────────────────────────────────────────────────────────────────────────

#[cfg(feature = "compression-lz4")]
#[test]
fn test_adv2_lz4_nested_tuple_roundtrip() {
    use oxicode::compression::{compress, decompress, Compression};

    let original: Vec<(u32, u64, i8)> = (0u32..100)
        .map(|i| (i, i as u64 * 1_000_000, (i % 127) as i8))
        .collect();
    let encoded = oxicode::encode_to_vec(&original).expect("encode nested tuple failed");
    let compressed = compress(&encoded, Compression::Lz4).expect("compress nested tuple failed");
    let decompressed = decompress(&compressed).expect("decompress nested tuple failed");
    let (decoded, _): (Vec<(u32, u64, i8)>, usize) =
        oxicode::decode_from_slice(&decompressed).expect("decode nested tuple failed");
    assert_eq!(original, decoded);
}

// ─────────────────────────────────────────────────────────────────────────────
// Test 20 – Compression::None wraps data; payload is byte-for-byte the original
// ─────────────────────────────────────────────────────────────────────────────

#[cfg(feature = "compression-lz4")]
#[test]
fn test_adv2_none_codec_payload_is_unmodified() {
    use oxicode::compression::{compress, Compression};

    let data = b"none codec payload must not be altered";
    let wrapped = compress(data, Compression::None).expect("compress with None failed");
    // The 5-byte header is followed by the original payload verbatim.
    assert_eq!(
        &wrapped[5..],
        data,
        "Compression::None must leave payload bytes unchanged after the 5-byte header"
    );
}

// ─────────────────────────────────────────────────────────────────────────────
// Test 21 – Zstd ZstdLevel(1) roundtrip on highly compressible data
// ─────────────────────────────────────────────────────────────────────────────

#[cfg(feature = "compression-zstd")]
#[test]
fn test_adv2_zstd_level1_roundtrip() {
    use oxicode::compression::{compress, decompress, detect_compression, Compression};

    let original: Vec<u8> = b"zstd level 1 roundtrip test"
        .iter()
        .copied()
        .cycle()
        .take(4096)
        .collect();
    let compressed =
        compress(&original, Compression::ZstdLevel(1)).expect("ZstdLevel(1) compress failed");
    // ZstdLevel still advertises codec id 2 (Zstd).
    let detected = detect_compression(&compressed);
    assert_eq!(
        detected,
        Some(Compression::Zstd),
        "ZstdLevel(1) must still be detected as Compression::Zstd"
    );
    let decompressed = decompress(&compressed).expect("ZstdLevel(1) decompress failed");
    assert_eq!(original, decompressed);
}

// ─────────────────────────────────────────────────────────────────────────────
// Test 22 – Zstd compress_with_stats: original_size equals input len
// ─────────────────────────────────────────────────────────────────────────────

#[cfg(feature = "compression-zstd")]
#[test]
fn test_adv2_zstd_stats_original_size_field() {
    use oxicode::compression::{compress_with_stats, Compression};

    let data: Vec<u8> = vec![0u8; 3000];
    let (compressed_vec, stats) =
        compress_with_stats(&data, Compression::Zstd).expect("zstd compress_with_stats failed");
    assert_eq!(
        stats.original_size, 3000,
        "stats.original_size must equal 3000, got {}",
        stats.original_size
    );
    assert_eq!(
        stats.compressed_size,
        compressed_vec.len(),
        "stats.compressed_size must match returned Vec length"
    );
    assert!(
        stats.savings_percent() > 0.0,
        "savings_percent must be positive for 3000 zeros, got {}",
        stats.savings_percent()
    );
}
