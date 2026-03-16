//! Extended integration tests for the oxicode compression module.
//!
//! Covers edge cases, cross-codec behaviour, large data, magic-byte layout,
//! and serialized-struct round-trips that complement the baseline
//! `compression_test.rs` suite.

// ---------------------------------------------------------------------------
// LZ4 tests
// ---------------------------------------------------------------------------

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
mod lz4_extended {
    use oxicode::compression::{
        compress, compress_with_stats, decompress, decompress_or_passthrough, detect_compression,
        is_compressed, Compression,
    };
    use oxicode::{Decode, Encode};

    // ------------------------------------------------------------------
    // Helper structs
    // ------------------------------------------------------------------

    #[derive(Debug, PartialEq, Encode, Decode)]
    struct SimpleStruct {
        id: u32,
        value: f64,
        label: String,
    }

    // ------------------------------------------------------------------
    // Test 1 – compress / decompress 10 KB of all-zero bytes
    // ------------------------------------------------------------------

    #[test]
    fn test_lz4_roundtrip_10kb_zeros() {
        let input: Vec<u8> = vec![0u8; 10 * 1024];
        let compressed = compress(&input, Compression::Lz4).expect("compress failed");
        let decompressed = decompress(&compressed).expect("decompress failed");
        assert_eq!(
            input, decompressed,
            "10 KB of zeros must survive LZ4 round-trip"
        );
    }

    // ------------------------------------------------------------------
    // Test 2 – highly compressible data: compressed < original
    // ------------------------------------------------------------------

    #[test]
    fn test_lz4_compressible_data_shrinks() {
        // 8 000 copies of a 4-byte pattern = highly compressible
        let pattern: &[u8] = b"ABCD";
        let input: Vec<u8> = pattern.iter().cycle().take(8_000).copied().collect();
        let compressed = compress(&input, Compression::Lz4).expect("compress failed");
        assert!(
            compressed.len() < input.len(),
            "compressed ({}) must be smaller than original ({})",
            compressed.len(),
            input.len()
        );
    }

    // ------------------------------------------------------------------
    // Test 3 – incompressible data still decompresses correctly
    // ------------------------------------------------------------------

    #[test]
    fn test_lz4_incompressible_data_roundtrip() {
        // Use a deterministic pseudo-random-looking sequence (LCG) that does
        // not compress well, but is still fully deterministic.
        let mut state: u64 = 0xDEAD_BEEF_CAFE_BABE;
        let input: Vec<u8> = (0..4_096)
            .map(|_| {
                state = state
                    .wrapping_mul(6_364_136_223_846_793_005)
                    .wrapping_add(1_442_695_040_888_963_407);
                (state >> 56) as u8
            })
            .collect();

        let compressed = compress(&input, Compression::Lz4).expect("compress failed");
        let decompressed = decompress(&compressed).expect("decompress failed");
        assert_eq!(
            input, decompressed,
            "incompressible data must survive round-trip"
        );
    }

    // ------------------------------------------------------------------
    // Test 7 – compressed empty data round-trip
    // ------------------------------------------------------------------

    #[test]
    fn test_lz4_empty_bytes_roundtrip() {
        let input: Vec<u8> = Vec::new();
        let compressed = compress(&input, Compression::Lz4).expect("compress failed");
        let decompressed = decompress(&compressed).expect("decompress failed");
        assert_eq!(
            input, decompressed,
            "empty byte slice must survive LZ4 round-trip"
        );
    }

    // ------------------------------------------------------------------
    // Test 8 – compress 1 MB of zeros with LZ4
    // ------------------------------------------------------------------

    #[test]
    fn test_lz4_large_data_1mb_zeros() {
        let input: Vec<u8> = vec![0u8; 1024 * 1024];
        let compressed = compress(&input, Compression::Lz4).expect("compress 1 MB failed");
        // 1 MB of zeros must compress to far less than 1 MB
        assert!(
            compressed.len() < input.len() / 10,
            "1 MB of zeros should compress by at least 10×, got {} → {}",
            input.len(),
            compressed.len()
        );
        let decompressed = decompress(&compressed).expect("decompress 1 MB failed");
        assert_eq!(input, decompressed);
    }

    // ------------------------------------------------------------------
    // Test 9 – compress serialized struct bytes with LZ4
    // ------------------------------------------------------------------

    #[test]
    fn test_lz4_compress_serialized_struct() {
        let original = SimpleStruct {
            id: 42,
            value: std::f64::consts::PI,
            label: "oxicode-lz4-struct".into(),
        };
        let encoded = oxicode::encode_to_vec(&original).expect("encode failed");
        let compressed = compress(&encoded, Compression::Lz4).expect("compress failed");
        let decompressed = decompress(&compressed).expect("decompress failed");
        let (decoded, _): (SimpleStruct, usize) =
            oxicode::decode_from_slice(&decompressed).expect("decode failed");
        assert_eq!(original, decoded);
    }

    // ------------------------------------------------------------------
    // Test 10 – compress/decompress preserves exact byte sequence
    // ------------------------------------------------------------------

    #[test]
    fn test_lz4_exact_byte_preservation() {
        // Build a payload with every possible byte value to confirm no byte
        // is silently altered during compression.
        let input: Vec<u8> = (0u8..=255).cycle().take(512).collect();
        let compressed = compress(&input, Compression::Lz4).expect("compress failed");
        let decompressed = decompress(&compressed).expect("decompress failed");
        assert_eq!(
            input, decompressed,
            "every byte value must be preserved exactly"
        );
    }

    // ------------------------------------------------------------------
    // Test 11 – magic byte layout: header starts with 0x4F 0x58 0x43
    // ------------------------------------------------------------------

    #[test]
    fn test_lz4_compressed_magic_bytes() {
        let input = b"magic check payload";
        let compressed = compress(input, Compression::Lz4).expect("compress failed");
        // Magic = "OXC" (0x4F, 0x58, 0x43)
        assert_eq!(
            &compressed[0..3],
            &[0x4F, 0x58, 0x43],
            "first 3 bytes must be OXC magic"
        );
        // Version byte (index 3) must be 1
        assert_eq!(compressed[3], 1u8, "version byte must be 1");
        // Codec byte (index 4) for LZ4 must be 1
        assert_eq!(compressed[4], 1u8, "codec byte for LZ4 must be 1");
    }

    // ------------------------------------------------------------------
    // Test 12 – is_compressed() returns true/false correctly
    // ------------------------------------------------------------------

    #[test]
    fn test_is_compressed_lz4_true_false() {
        let plain = b"I am plain text with no compression header.";
        assert!(
            !is_compressed(plain),
            "plain bytes must not be detected as compressed"
        );

        let compressed = compress(plain, Compression::Lz4).expect("compress failed");
        assert!(
            is_compressed(&compressed),
            "LZ4-compressed bytes must be detected as compressed"
        );
    }

    // ------------------------------------------------------------------
    // Test 13 – detect_compression() identifies LZ4
    // ------------------------------------------------------------------

    #[test]
    fn test_detect_compression_lz4_codec() {
        let input = b"detect codec";
        let compressed = compress(input, Compression::Lz4).expect("compress failed");
        let codec = detect_compression(&compressed);
        assert_eq!(
            codec,
            Some(Compression::Lz4),
            "detect_compression must return Some(Lz4)"
        );

        // Plain data has no codec
        assert_eq!(
            detect_compression(input),
            None,
            "detect_compression must return None for plain data"
        );
    }

    // ------------------------------------------------------------------
    // Test 14 – decompress_or_passthrough leaves plain data untouched
    // ------------------------------------------------------------------

    #[test]
    fn test_decompress_or_passthrough_plain_stays_plain() {
        let plain: Vec<u8> = b"no header here, pass me through".to_vec();
        let result = decompress_or_passthrough(&plain).expect("passthrough failed");
        assert_eq!(plain, result, "plain data must be returned unchanged");
    }

    // ------------------------------------------------------------------
    // Test 15 – compress Vec<String> serialized, decompress, re-deserialize
    // ------------------------------------------------------------------

    #[test]
    fn test_lz4_vec_string_serialize_compress_roundtrip() {
        let strings: Vec<String> = (0..100)
            .map(|i| format!("entry-{i:04}-padding-xxxxxxxxxx"))
            .collect();

        let encoded = oxicode::encode_to_vec(&strings).expect("encode Vec<String> failed");
        let compressed = compress(&encoded, Compression::Lz4).expect("compress failed");
        let decompressed = decompress(&compressed).expect("decompress failed");
        let (decoded, _): (Vec<String>, usize) =
            oxicode::decode_from_slice(&decompressed).expect("decode Vec<String> failed");

        assert_eq!(strings, decoded, "Vec<String> must survive full round-trip");
    }

    // ------------------------------------------------------------------
    // Test 6 (LZ4 side) – compress_with_stats reflects accurate sizes
    // ------------------------------------------------------------------

    #[test]
    fn test_lz4_compress_with_stats_accuracy() {
        let data: Vec<u8> = vec![0u8; 4_096];
        let original_len = data.len();
        let (compressed, stats) =
            compress_with_stats(&data, Compression::Lz4).expect("compress_with_stats failed");

        assert_eq!(
            stats.original_size, original_len,
            "stats.original_size must equal input length"
        );
        assert_eq!(
            stats.compressed_size,
            compressed.len(),
            "stats.compressed_size must equal returned buffer length"
        );
        assert!(
            stats.ratio() > 1.0,
            "compression ratio must be > 1.0 for all-zero data"
        );
        assert!(
            stats.savings_percent() > 0.0,
            "savings_percent must be positive"
        );
    }
}

// ---------------------------------------------------------------------------
// Cross-codec test (requires both LZ4 and Zstd disabled on Zstd side — or
// just LZ4 available): verify decompress rejects truncated payload gracefully.
// ---------------------------------------------------------------------------

#[cfg(feature = "compression-lz4")]
mod cross_codec {
    use oxicode::compression::{compress, decompress, Compression};

    // ------------------------------------------------------------------
    // Test 6 – LZ4-compressed data cannot be decoded as a raw Zstd stream
    // by the lower-level path; confirm decompress() succeeds only for the
    // correct codec and that the header codec byte is authoritative.
    // ------------------------------------------------------------------

    #[test]
    fn test_lz4_header_with_corrupt_codec_byte_returns_error() {
        let input = b"cross-codec mismatch test payload";
        let mut compressed = compress(input, Compression::Lz4).expect("compress failed");

        // Corrupt the codec byte (index 4) to an unknown value (0xFF).
        compressed[4] = 0xFF;

        let result = decompress(&compressed);
        assert!(
            result.is_err(),
            "decompress must fail when codec byte is unknown (0xFF)"
        );
    }
}

// ---------------------------------------------------------------------------
// Zstd tests
// ---------------------------------------------------------------------------

#[cfg(feature = "compression-zstd")]
mod zstd_extended {
    use oxicode::compression::{compress, decompress, detect_compression, Compression};
    use oxicode::{Decode, Encode};

    #[derive(Debug, PartialEq, Encode, Decode)]
    struct TaggedPayload {
        tag: u64,
        body: Vec<u8>,
    }

    // ------------------------------------------------------------------
    // Test 4 – Zstd compress then decompress basic round-trip
    // ------------------------------------------------------------------

    #[test]
    fn test_zstd_basic_roundtrip() {
        let input = b"Hello from Zstd extended tests! ".repeat(50);
        let compressed = compress(&input, Compression::Zstd).expect("zstd compress failed");
        let decompressed = decompress(&compressed).expect("zstd decompress failed");
        assert_eq!(input.as_slice(), decompressed.as_slice());
    }

    // ------------------------------------------------------------------
    // Test 5 – Zstd different compression levels all round-trip correctly
    // ------------------------------------------------------------------

    #[test]
    fn test_zstd_multiple_levels_roundtrip() {
        let data: Vec<u64> = (0u64..300).collect();
        let encoded = oxicode::encode_to_vec(&data).expect("encode failed");

        for level in [1u8, 3, 6, 9, 12, 15, 19, 22] {
            let compressed =
                compress(&encoded, Compression::ZstdLevel(level)).expect("compress failed");
            let decompressed = decompress(&compressed).expect("decompress failed");
            let (decoded, _): (Vec<u64>, usize) =
                oxicode::decode_from_slice(&decompressed).expect("decode failed");
            assert_eq!(data, decoded, "round-trip failed at zstd level {level}");
        }
    }

    // ------------------------------------------------------------------
    // Test 13 (Zstd variant) – detect_compression identifies Zstd
    // ------------------------------------------------------------------

    #[test]
    fn test_detect_compression_zstd_codec() {
        let input = b"zstd codec detection";
        let compressed = compress(input, Compression::Zstd).expect("compress failed");
        let codec = detect_compression(&compressed);
        assert_eq!(
            codec,
            Some(Compression::Zstd),
            "detect_compression must return Some(Zstd)"
        );
    }

    // ------------------------------------------------------------------
    // Zstd struct round-trip to complement test 9 for LZ4
    // ------------------------------------------------------------------

    #[test]
    fn test_zstd_compress_serialized_struct() {
        let original = TaggedPayload {
            tag: 0xCAFE_BABE,
            body: vec![1u8, 2, 3, 4, 5, 6, 7, 8],
        };
        let encoded = oxicode::encode_to_vec(&original).expect("encode failed");
        let compressed = compress(&encoded, Compression::Zstd).expect("compress failed");
        let decompressed = decompress(&compressed).expect("decompress failed");
        let (decoded, _): (TaggedPayload, usize) =
            oxicode::decode_from_slice(&decompressed).expect("decode failed");
        assert_eq!(original, decoded);
    }
}

// ---------------------------------------------------------------------------
// LZ4 extended tests – batch 2
// ---------------------------------------------------------------------------

#[cfg(feature = "compression-lz4")]
mod lz4_extended2 {
    use oxicode::compression::{compress, compress_with_stats, decompress, Compression};
    use oxicode::{Decode, Encode};
    use std::collections::BTreeMap;

    // ------------------------------------------------------------------
    // Helper structs
    // ------------------------------------------------------------------

    #[derive(Debug, PartialEq, Encode, Decode)]
    struct StringStruct {
        name: String,
        description: String,
        tags: Vec<String>,
    }

    #[derive(Debug, PartialEq, Encode, Decode)]
    struct TupleStruct(u32, f64, String);

    #[derive(Debug, PartialEq, Encode, Decode)]
    struct EmptyStruct {}

    #[derive(Debug, PartialEq, Encode, Decode)]
    struct AllNumericTypes {
        a_u8: u8,
        a_u16: u16,
        a_u32: u32,
        a_u64: u64,
        a_i8: i8,
        a_i16: i16,
        a_i32: i32,
        a_i64: i64,
        a_f32: f32,
        a_f64: f64,
    }

    // ------------------------------------------------------------------
    // Test LZ4-1: compression ratio for repetitive data
    // ------------------------------------------------------------------

    #[test]
    fn test_lz4_compression_ratio_repetitive_data() {
        // 5000 repetitions of an 8-byte pattern is highly compressible
        let pattern = b"OXICODE!";
        let input: Vec<u8> = pattern.iter().cycle().take(5_000 * 8).copied().collect();
        let compressed = compress(&input, Compression::Lz4).expect("compress failed");
        assert!(
            compressed.len() < input.len(),
            "compressed ({}) must be smaller than original ({})",
            compressed.len(),
            input.len()
        );
    }

    // ------------------------------------------------------------------
    // Test LZ4-2: compress then decompress struct with strings
    // ------------------------------------------------------------------

    #[test]
    fn test_lz4_compress_decompress_struct_with_strings() {
        let original = StringStruct {
            name: "OxiCode Compression".into(),
            description: "A fast, efficient binary serialization library".repeat(10),
            tags: vec!["lz4".into(), "compression".into(), "rust".into()],
        };
        let encoded = oxicode::encode_to_vec(&original).expect("encode failed");
        let compressed = compress(&encoded, Compression::Lz4).expect("compress failed");
        let decompressed = decompress(&compressed).expect("decompress failed");
        let (decoded, _): (StringStruct, usize) =
            oxicode::decode_from_slice(&decompressed).expect("decode failed");
        assert_eq!(original, decoded);
    }

    // ------------------------------------------------------------------
    // Test LZ4-3: compress Vec<Vec<u8>> roundtrip
    // ------------------------------------------------------------------

    #[test]
    fn test_lz4_compress_vec_of_vec_u8_roundtrip() {
        let original: Vec<Vec<u8>> = (0u8..16).map(|i| vec![i; (i as usize + 1) * 4]).collect();
        let encoded = oxicode::encode_to_vec(&original).expect("encode Vec<Vec<u8>> failed");
        let compressed = compress(&encoded, Compression::Lz4).expect("compress failed");
        let decompressed = decompress(&compressed).expect("decompress failed");
        let (decoded, _): (Vec<Vec<u8>>, usize) =
            oxicode::decode_from_slice(&decompressed).expect("decode Vec<Vec<u8>> failed");
        assert_eq!(original, decoded);
    }

    // ------------------------------------------------------------------
    // Test LZ4-4: compress BTreeMap<String, u64> roundtrip
    // ------------------------------------------------------------------

    #[test]
    fn test_lz4_compress_btreemap_string_u64_roundtrip() {
        let mut original: BTreeMap<String, u64> = BTreeMap::new();
        for i in 0u64..64 {
            original.insert(format!("key_{i:04}"), i * i);
        }
        let encoded = oxicode::encode_to_vec(&original).expect("encode BTreeMap failed");
        let compressed = compress(&encoded, Compression::Lz4).expect("compress failed");
        let decompressed = decompress(&compressed).expect("decompress failed");
        let (decoded, _): (BTreeMap<String, u64>, usize) =
            oxicode::decode_from_slice(&decompressed).expect("decode BTreeMap failed");
        assert_eq!(original, decoded);
    }

    // ------------------------------------------------------------------
    // Test LZ4-5: compress Option<String> roundtrip (Some and None)
    // ------------------------------------------------------------------

    #[test]
    fn test_lz4_compress_option_string_roundtrip() {
        // Some variant
        let some_val: Option<String> = Some("hello oxicode lz4 option".into());
        let encoded = oxicode::encode_to_vec(&some_val).expect("encode Some failed");
        let compressed = compress(&encoded, Compression::Lz4).expect("compress Some failed");
        let decompressed = decompress(&compressed).expect("decompress Some failed");
        let (decoded, _): (Option<String>, usize) =
            oxicode::decode_from_slice(&decompressed).expect("decode Some failed");
        assert_eq!(some_val, decoded);

        // None variant
        let none_val: Option<String> = None;
        let encoded = oxicode::encode_to_vec(&none_val).expect("encode None failed");
        let compressed = compress(&encoded, Compression::Lz4).expect("compress None failed");
        let decompressed = decompress(&compressed).expect("decompress None failed");
        let (decoded, _): (Option<String>, usize) =
            oxicode::decode_from_slice(&decompressed).expect("decode None failed");
        assert_eq!(none_val, decoded);
    }

    // ------------------------------------------------------------------
    // Test LZ4-6: compress empty Vec roundtrip
    // ------------------------------------------------------------------

    #[test]
    fn test_lz4_compress_empty_vec_roundtrip() {
        let original: Vec<u64> = Vec::new();
        let encoded = oxicode::encode_to_vec(&original).expect("encode empty Vec failed");
        let compressed = compress(&encoded, Compression::Lz4).expect("compress failed");
        let decompressed = decompress(&compressed).expect("decompress failed");
        let (decoded, _): (Vec<u64>, usize) =
            oxicode::decode_from_slice(&decompressed).expect("decode empty Vec failed");
        assert_eq!(original, decoded);
    }

    // ------------------------------------------------------------------
    // Test LZ4-7: compress very large Vec<u64> (10000 items)
    // ------------------------------------------------------------------

    #[test]
    fn test_lz4_compress_large_vec_u64_10000() {
        let original: Vec<u64> = (0u64..10_000).collect();
        let encoded = oxicode::encode_to_vec(&original).expect("encode large Vec<u64> failed");
        let compressed = compress(&encoded, Compression::Lz4).expect("compress failed");
        let decompressed = decompress(&compressed).expect("decompress failed");
        let (decoded, _): (Vec<u64>, usize) =
            oxicode::decode_from_slice(&decompressed).expect("decode large Vec<u64> failed");
        assert_eq!(original, decoded);
    }

    // ------------------------------------------------------------------
    // Test LZ4-8: compress tuple struct roundtrip
    // ------------------------------------------------------------------

    #[test]
    fn test_lz4_compress_tuple_struct_roundtrip() {
        let original = TupleStruct(
            0xDEAD_BEEF,
            std::f64::consts::PI * std::f64::consts::E,
            "tuple-struct-lz4-test".into(),
        );
        let encoded = oxicode::encode_to_vec(&original).expect("encode TupleStruct failed");
        let compressed = compress(&encoded, Compression::Lz4).expect("compress failed");
        let decompressed = decompress(&compressed).expect("decompress failed");
        let (decoded, _): (TupleStruct, usize) =
            oxicode::decode_from_slice(&decompressed).expect("decode TupleStruct failed");
        assert_eq!(original, decoded);
    }

    // ------------------------------------------------------------------
    // Test LZ4-15: compress empty struct
    // ------------------------------------------------------------------

    #[test]
    fn test_lz4_compress_empty_struct() {
        let original = EmptyStruct {};
        let encoded = oxicode::encode_to_vec(&original).expect("encode EmptyStruct failed");
        let compressed = compress(&encoded, Compression::Lz4).expect("compress failed");
        let decompressed = decompress(&compressed).expect("decompress failed");
        let (decoded, _): (EmptyStruct, usize) =
            oxicode::decode_from_slice(&decompressed).expect("decode EmptyStruct failed");
        assert_eq!(original, decoded);
    }

    // ------------------------------------------------------------------
    // Test LZ4-16: compress struct with all numeric types
    // ------------------------------------------------------------------

    #[test]
    fn test_lz4_compress_struct_all_numeric_types() {
        let original = AllNumericTypes {
            a_u8: 255,
            a_u16: 65535,
            a_u32: 0xDEAD_BEEF,
            a_u64: u64::MAX,
            a_i8: -128,
            a_i16: -32768,
            a_i32: i32::MIN,
            a_i64: i64::MIN,
            a_f32: std::f32::consts::PI,
            a_f64: std::f64::consts::E,
        };
        let encoded = oxicode::encode_to_vec(&original).expect("encode AllNumericTypes failed");
        let compressed = compress(&encoded, Compression::Lz4).expect("compress failed");
        let decompressed = decompress(&compressed).expect("decompress failed");
        let (decoded, _): (AllNumericTypes, usize) =
            oxicode::decode_from_slice(&decompressed).expect("decode AllNumericTypes failed");
        assert_eq!(original, decoded);
    }

    // ------------------------------------------------------------------
    // Test LZ4-17: decompress corrupted data returns error
    // ------------------------------------------------------------------

    #[test]
    fn test_lz4_decompress_corrupted_data_returns_error() {
        let input = b"some data to compress for corruption test";
        let mut compressed = compress(input, Compression::Lz4).expect("compress failed");
        // Corrupt the LZ4 payload (everything after the 5-byte header)
        let header_size = 5;
        if compressed.len() > header_size + 4 {
            compressed[header_size + 2] ^= 0xFF;
            compressed[header_size + 3] ^= 0xFF;
            compressed[header_size + 4] ^= 0xFF;
        }
        // Either it errors out or decompresses to different data — we require error
        let result = decompress(&compressed);
        assert!(
            result.is_err(),
            "decompressing heavily corrupted LZ4 payload must return an error"
        );
    }

    // ------------------------------------------------------------------
    // Test LZ4-18: compress with standard config (via encode_to_vec_with_config)
    // ------------------------------------------------------------------

    #[test]
    fn test_lz4_compress_with_standard_config() {
        let data: Vec<u32> = (0u32..256).collect();
        let encoded = oxicode::encode_to_vec_with_config(&data, oxicode::config::standard())
            .expect("encode with standard config failed");
        let compressed = compress(&encoded, Compression::Lz4).expect("compress failed");
        let decompressed = decompress(&compressed).expect("decompress failed");
        let (decoded, _): (Vec<u32>, usize) =
            oxicode::decode_from_slice_with_config(&decompressed, oxicode::config::standard())
                .expect("decode with standard config failed");
        assert_eq!(data, decoded);
    }

    // ------------------------------------------------------------------
    // Test LZ4-19: compress with fixed_int config
    // ------------------------------------------------------------------

    #[test]
    fn test_lz4_compress_with_fixed_int_config() {
        let data: Vec<u32> = (0u32..128).collect();
        let config = oxicode::config::standard().with_fixed_int_encoding();
        let encoded = oxicode::encode_to_vec_with_config(&data, config)
            .expect("encode with fixed config failed");
        let compressed = compress(&encoded, Compression::Lz4).expect("compress failed");
        let decompressed = decompress(&compressed).expect("decompress failed");
        let (decoded, _): (Vec<u32>, usize) =
            oxicode::decode_from_slice_with_config(&decompressed, config)
                .expect("decode with fixed config failed");
        assert_eq!(data, decoded);
    }

    // ------------------------------------------------------------------
    // Test LZ4-20: compress large repetitive string
    // ------------------------------------------------------------------

    #[test]
    fn test_lz4_compress_large_repetitive_string() {
        let original: String = "OxiCode is a fast binary serialization library! ".repeat(500);
        let encoded = oxicode::encode_to_vec(&original).expect("encode large String failed");
        let compressed = compress(&encoded, Compression::Lz4).expect("compress failed");
        // Highly repetitive string should compress
        assert!(
            compressed.len() < encoded.len(),
            "repetitive string should compress: {} -> {}",
            encoded.len(),
            compressed.len()
        );
        let decompressed = decompress(&compressed).expect("decompress failed");
        let (decoded, _): (String, usize) =
            oxicode::decode_from_slice(&decompressed).expect("decode large String failed");
        assert_eq!(original, decoded);
    }

    // ------------------------------------------------------------------
    // Test LZ4-stats: compress_with_stats reflects accurate sizes for struct
    // ------------------------------------------------------------------

    #[test]
    fn test_lz4_compress_with_stats_struct_data() {
        let data: Vec<u64> = (0u64..1000).map(|x| x * x).collect();
        let encoded = oxicode::encode_to_vec(&data).expect("encode failed");
        let original_len = encoded.len();
        let (compressed, stats) =
            compress_with_stats(&encoded, Compression::Lz4).expect("compress_with_stats failed");
        assert_eq!(
            stats.original_size, original_len,
            "original_size must match input length"
        );
        assert_eq!(
            stats.compressed_size,
            compressed.len(),
            "compressed_size must match returned buffer length"
        );
        assert!(stats.ratio() > 0.0, "ratio must be positive");
    }
}

// ---------------------------------------------------------------------------
// Zstd extended tests – batch 2
// ---------------------------------------------------------------------------

#[cfg(feature = "compression-zstd")]
mod zstd_extended2 {
    use oxicode::compression::{compress, decompress, Compression};
    use oxicode::{Decode, Encode};
    use std::collections::BTreeMap;

    // ------------------------------------------------------------------
    // Helper structs
    // ------------------------------------------------------------------

    #[derive(Debug, PartialEq, Encode, Decode)]
    struct MultiFieldStruct {
        id: u64,
        name: String,
        score: f64,
        active: bool,
        tags: Vec<String>,
    }

    #[derive(Debug, PartialEq, Encode, Decode)]
    struct EmptyZstd {}

    // ------------------------------------------------------------------
    // Test Zstd-9: compression ratio for repetitive data
    // ------------------------------------------------------------------

    #[test]
    fn test_zstd_compression_ratio_repetitive_data() {
        let pattern = b"ZSTD_REPETITIVE_PATTERN_";
        let input: Vec<u8> = pattern.iter().cycle().take(8_000).copied().collect();
        let compressed = compress(&input, Compression::Zstd).expect("zstd compress failed");
        assert!(
            compressed.len() < input.len(),
            "zstd compressed ({}) must be smaller than original ({})",
            compressed.len(),
            input.len()
        );
    }

    // ------------------------------------------------------------------
    // Test Zstd-10: compress struct with multiple fields
    // ------------------------------------------------------------------

    #[test]
    fn test_zstd_compress_struct_multiple_fields() {
        let original = MultiFieldStruct {
            id: 0xCAFE_BABE_DEAD_BEEF,
            name: "oxicode zstd multi-field test".into(),
            score: std::f64::consts::PI + std::f64::consts::E,
            active: true,
            tags: vec![
                "zstd".into(),
                "oxicode".into(),
                "rust".into(),
                "fast".into(),
            ],
        };
        let encoded = oxicode::encode_to_vec(&original).expect("encode MultiFieldStruct failed");
        let compressed = compress(&encoded, Compression::Zstd).expect("zstd compress failed");
        let decompressed = decompress(&compressed).expect("zstd decompress failed");
        let (decoded, _): (MultiFieldStruct, usize) =
            oxicode::decode_from_slice(&decompressed).expect("decode MultiFieldStruct failed");
        assert_eq!(original, decoded);
    }

    // ------------------------------------------------------------------
    // Test Zstd-11: compress Vec<String> roundtrip
    // ------------------------------------------------------------------

    #[test]
    fn test_zstd_compress_vec_string_roundtrip() {
        let original: Vec<String> = (0..200)
            .map(|i| format!("zstd-entry-{i:05}-data-padding"))
            .collect();
        let encoded = oxicode::encode_to_vec(&original).expect("encode Vec<String> failed");
        let compressed = compress(&encoded, Compression::Zstd).expect("zstd compress failed");
        let decompressed = decompress(&compressed).expect("zstd decompress failed");
        let (decoded, _): (Vec<String>, usize) =
            oxicode::decode_from_slice(&decompressed).expect("decode Vec<String> failed");
        assert_eq!(original, decoded);
    }

    // ------------------------------------------------------------------
    // Test Zstd-12: compress BTreeMap roundtrip
    // ------------------------------------------------------------------

    #[test]
    fn test_zstd_compress_btreemap_roundtrip() {
        let mut original: BTreeMap<String, Vec<u8>> = BTreeMap::new();
        for i in 0u8..32 {
            original.insert(format!("zstd_key_{i}"), vec![i; 16]);
        }
        let encoded = oxicode::encode_to_vec(&original).expect("encode BTreeMap failed");
        let compressed = compress(&encoded, Compression::Zstd).expect("zstd compress failed");
        let decompressed = decompress(&compressed).expect("zstd decompress failed");
        let (decoded, _): (BTreeMap<String, Vec<u8>>, usize) =
            oxicode::decode_from_slice(&decompressed).expect("decode BTreeMap failed");
        assert_eq!(original, decoded);
    }

    // ------------------------------------------------------------------
    // Test Zstd-14: compression level affects size (level 1 vs level 19)
    // ------------------------------------------------------------------

    #[test]
    fn test_zstd_compression_level_affects_size() {
        // Create highly compressible data where level difference is apparent
        let input: Vec<u8> = b"ZSTD_LEVEL_TEST_"
            .iter()
            .cycle()
            .take(32_000)
            .copied()
            .collect();
        let compressed_l1 =
            compress(&input, Compression::ZstdLevel(1)).expect("zstd level 1 compress failed");
        let compressed_l19 =
            compress(&input, Compression::ZstdLevel(19)).expect("zstd level 19 compress failed");
        // Both must round-trip correctly
        let dec_l1 = decompress(&compressed_l1).expect("zstd level 1 decompress failed");
        let dec_l19 = decompress(&compressed_l19).expect("zstd level 19 decompress failed");
        assert_eq!(
            input.as_slice(),
            dec_l1.as_slice(),
            "level 1 round-trip failed"
        );
        assert_eq!(
            input.as_slice(),
            dec_l19.as_slice(),
            "level 19 round-trip failed"
        );
        // Higher level should produce equal or smaller output on repetitive data
        assert!(
            compressed_l19.len() <= compressed_l1.len(),
            "level 19 ({}) should be <= level 1 ({})",
            compressed_l19.len(),
            compressed_l1.len()
        );
    }

    // ------------------------------------------------------------------
    // Test Zstd-15: compress empty struct
    // ------------------------------------------------------------------

    #[test]
    fn test_zstd_compress_empty_struct() {
        let original = EmptyZstd {};
        let encoded = oxicode::encode_to_vec(&original).expect("encode EmptyZstd failed");
        let compressed = compress(&encoded, Compression::Zstd).expect("zstd compress failed");
        let decompressed = decompress(&compressed).expect("zstd decompress failed");
        let (decoded, _): (EmptyZstd, usize) =
            oxicode::decode_from_slice(&decompressed).expect("decode EmptyZstd failed");
        assert_eq!(original, decoded);
    }

    // ------------------------------------------------------------------
    // Test Zstd-20: compress large repetitive string
    // ------------------------------------------------------------------

    #[test]
    fn test_zstd_compress_large_repetitive_string() {
        let original: String = "Zstd OxiCode serialization is efficient and fast! ".repeat(400);
        let encoded = oxicode::encode_to_vec(&original).expect("encode large String failed");
        let compressed = compress(&encoded, Compression::Zstd).expect("zstd compress failed");
        // Repetitive string must compress well with Zstd
        assert!(
            compressed.len() < encoded.len(),
            "zstd: repetitive string should compress: {} -> {}",
            encoded.len(),
            compressed.len()
        );
        let decompressed = decompress(&compressed).expect("zstd decompress failed");
        let (decoded, _): (String, usize) =
            oxicode::decode_from_slice(&decompressed).expect("decode large String failed");
        assert_eq!(original, decoded);
    }
}

// ---------------------------------------------------------------------------
// Cross-codec mismatch: Zstd compressed → LZ4 decompress path (requires both)
// ---------------------------------------------------------------------------

#[cfg(all(feature = "compression-lz4", feature = "compression-zstd"))]
mod cross_codec_mismatch {
    use oxicode::compression::{compress, decompress, Compression};

    // ------------------------------------------------------------------
    // Test 13: Compress with Zstd then attempt to force LZ4 decode via
    // header corruption — must produce an error (format mismatch).
    // ------------------------------------------------------------------

    #[test]
    fn test_zstd_compressed_corrupt_to_lz4_codec_returns_error() {
        let input = b"cross codec mismatch: zstd compressed, lz4 codec byte injected";
        let mut compressed = compress(input, Compression::Zstd).expect("zstd compress failed");

        // Override the codec byte (index 4) to LZ4's codec ID (1).
        // The payload is still Zstd-encoded, so LZ4 decompression must fail.
        compressed[4] = 1u8;

        let result = decompress(&compressed);
        assert!(
            result.is_err(),
            "decompressing Zstd payload with LZ4 codec byte must return an error"
        );
    }
}
