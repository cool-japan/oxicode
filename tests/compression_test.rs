//! Integration tests for compression features.
//!
//! These tests exercise the public compression API end-to-end, combining
//! oxicode encode/decode with compression/decompression in round-trip scenarios.

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
mod lz4_tests {
    use oxicode::compression::{
        compress, compress_with_stats, decompress, decompress_or_passthrough, detect_compression,
        is_compressed, Compression,
    };
    use oxicode::{Decode, Encode};

    /// A struct with multiple field types for round-trip testing.
    #[derive(Debug, PartialEq, Encode, Decode)]
    struct LargeData {
        items: Vec<u64>,
        name: String,
        flags: Vec<bool>,
    }

    /// Simple numeric struct to verify structural integrity after round-trip.
    #[derive(Debug, PartialEq, Encode, Decode)]
    struct Point3D {
        x: f64,
        y: f64,
        z: f64,
    }

    #[test]
    fn test_lz4_roundtrip_struct() {
        let data = LargeData {
            items: (0u64..1000).collect(),
            name: "oxicode-compression-test".into(),
            flags: (0..50).map(|i| i % 2 == 0).collect(),
        };

        let encoded = oxicode::encode_to_vec(&data).expect("encode failed");
        let compressed = compress(&encoded, Compression::Lz4).expect("compress failed");
        let decompressed = decompress(&compressed).expect("decompress failed");
        let (decoded, _): (LargeData, usize) =
            oxicode::decode_from_slice(&decompressed).expect("decode failed");

        assert_eq!(data, decoded);
    }

    #[test]
    fn test_lz4_roundtrip_primitive_vec() {
        let data: Vec<u32> = (0u32..500).collect();

        let encoded = oxicode::encode_to_vec(&data).expect("encode failed");
        let compressed = compress(&encoded, Compression::Lz4).expect("compress failed");
        let decompressed = decompress(&compressed).expect("decompress failed");
        let (decoded, _): (Vec<u32>, usize) =
            oxicode::decode_from_slice(&decompressed).expect("decode failed");

        assert_eq!(data, decoded);
    }

    #[test]
    fn test_lz4_roundtrip_float_struct() {
        let pt = Point3D {
            x: 1.23456789,
            y: -9.87654321,
            z: 0.0,
        };

        let encoded = oxicode::encode_to_vec(&pt).expect("encode failed");
        let compressed = compress(&encoded, Compression::Lz4).expect("compress failed");
        let decompressed = decompress(&compressed).expect("decompress failed");
        let (decoded, _): (Point3D, usize) =
            oxicode::decode_from_slice(&decompressed).expect("decode failed");

        assert_eq!(pt, decoded);
    }

    #[test]
    fn test_lz4_compression_reduces_size_for_repetitive_data() {
        // Highly compressible: 10 000 identical u64 zeros
        let data: Vec<u64> = vec![0u64; 10_000];
        let uncompressed = oxicode::encode_to_vec(&data).expect("encode failed");
        let compressed = compress(&uncompressed, Compression::Lz4).expect("compress failed");

        assert!(
            compressed.len() < uncompressed.len(),
            "Compressed ({} bytes) should be smaller than uncompressed ({} bytes)",
            compressed.len(),
            uncompressed.len()
        );
    }

    #[test]
    fn test_lz4_empty_payload() {
        // Encode an empty Vec, compress it, and recover it.
        let data: Vec<u8> = vec![];
        let encoded = oxicode::encode_to_vec(&data).expect("encode failed");
        let compressed = compress(&encoded, Compression::Lz4).expect("compress failed");
        let decompressed = decompress(&compressed).expect("decompress failed");
        let (decoded, _): (Vec<u8>, usize) =
            oxicode::decode_from_slice(&decompressed).expect("decode failed");
        assert_eq!(data, decoded);
    }

    #[test]
    fn test_lz4_invalid_magic_returns_error() {
        // A buffer with wrong magic bytes must be rejected by decompress().
        let garbage: Vec<u8> = vec![0xDE, 0xAD, 0xBE, 0xEF, 0x01, 0xFF, 0xFF];
        let result = decompress(&garbage);
        assert!(
            result.is_err(),
            "decompress() should fail on invalid magic header"
        );
    }

    #[test]
    fn test_lz4_truncated_header_returns_error() {
        // A buffer shorter than the 5-byte header must be rejected.
        let short: Vec<u8> = vec![0x4F, 0x58];
        let result = decompress(&short);
        assert!(
            result.is_err(),
            "decompress() should fail on truncated data"
        );
    }

    #[test]
    fn test_is_compressed_detection() {
        let raw = b"this is plain text, not compressed";
        assert!(
            !is_compressed(raw),
            "raw bytes should not be detected as compressed"
        );

        let compressed = compress(raw, Compression::Lz4).expect("compress failed");
        assert!(
            is_compressed(&compressed),
            "lz4-compressed data must be detected as compressed"
        );
    }

    #[test]
    fn test_detect_compression_lz4() {
        let raw = b"detect me";
        let compressed = compress(raw, Compression::Lz4).expect("compress failed");
        let detected = detect_compression(&compressed);
        assert_eq!(
            detected,
            Some(Compression::Lz4),
            "detected codec should be Lz4"
        );
    }

    #[test]
    fn test_detect_compression_none_on_raw() {
        let raw = b"no magic header here";
        assert_eq!(detect_compression(raw), None);
    }

    #[test]
    fn test_compress_with_stats_lz4() {
        let data: Vec<u64> = vec![42u64; 2_000];
        let encoded = oxicode::encode_to_vec(&data).expect("encode failed");
        let original_size = encoded.len();

        let (compressed, stats) =
            compress_with_stats(&encoded, Compression::Lz4).expect("compress_with_stats failed");

        assert_eq!(stats.original_size, original_size);
        assert_eq!(stats.compressed_size, compressed.len());
        assert!(
            stats.ratio() > 1.0,
            "ratio() should be > 1.0 for compressible data, got {}",
            stats.ratio()
        );
        assert!(
            stats.savings_percent() > 0.0,
            "savings_percent() should be positive for compressible data, got {}",
            stats.savings_percent()
        );
    }

    #[test]
    fn test_compression_stats_zero_size() {
        use oxicode::compression::CompressionStats;
        let stats = CompressionStats {
            original_size: 0,
            compressed_size: 0,
        };
        assert_eq!(stats.ratio(), 0.0);
        assert_eq!(stats.savings_percent(), 0.0);
    }

    #[test]
    fn test_decompress_or_passthrough_with_raw_data() {
        let raw = b"just raw bytes, no header";
        let result = decompress_or_passthrough(raw).expect("decompress_or_passthrough failed");
        assert_eq!(raw.as_ref(), result.as_slice());
    }

    #[test]
    fn test_decompress_or_passthrough_with_compressed_data() {
        let original = b"some data to compress";
        let compressed = compress(original, Compression::Lz4).expect("compress failed");
        let result =
            decompress_or_passthrough(&compressed).expect("decompress_or_passthrough failed");
        assert_eq!(original.as_ref(), result.as_slice());
    }

    #[test]
    fn test_none_compression_passthrough() {
        // Compression::None wraps with a header but does not modify payload bytes.
        let data = b"pass through unchanged";
        let wrapped = compress(data, Compression::None).expect("compress(None) failed");
        assert!(
            is_compressed(&wrapped),
            "None-wrapped data should still have header"
        );
        let recovered = decompress(&wrapped).expect("decompress failed");
        assert_eq!(data.as_ref(), recovered.as_slice());
    }

    #[test]
    fn test_lz4_string_roundtrip() {
        let s = "The quick brown fox jumps over the lazy dog. ".repeat(200);
        let encoded = oxicode::encode_to_vec(&s).expect("encode failed");
        let compressed = compress(&encoded, Compression::Lz4).expect("compress failed");

        // Repetitive text should compress well.
        assert!(
            compressed.len() < encoded.len(),
            "Compressed ({}) should be < encoded ({}) for repetitive text",
            compressed.len(),
            encoded.len()
        );

        let decompressed = decompress(&compressed).expect("decompress failed");
        let (decoded, _): (String, usize) =
            oxicode::decode_from_slice(&decompressed).expect("decode failed");
        assert_eq!(s, decoded);
    }
}

#[cfg(feature = "compression-zstd")]
mod zstd_tests {
    use oxicode::compression::{compress, decompress, Compression};

    #[test]
    fn test_zstd_roundtrip_simple() {
        let data = b"Hello from zstd compression!";
        let compressed = compress(data, Compression::Zstd).expect("zstd compress failed");
        let decompressed = decompress(&compressed).expect("zstd decompress failed");
        assert_eq!(data.as_ref(), decompressed.as_slice());
    }

    #[test]
    fn test_zstd_level_roundtrip() {
        let data: Vec<u64> = (0u64..500).collect();
        let encoded = oxicode::encode_to_vec(&data).expect("encode failed");

        for level in [1u8, 3, 9, 15, 19] {
            let compressed =
                compress(&encoded, Compression::ZstdLevel(level)).expect("compress failed");
            let decompressed = decompress(&compressed).expect("decompress failed");
            let (decoded, _): (Vec<u64>, usize) =
                oxicode::decode_from_slice(&decompressed).expect("decode failed");
            assert_eq!(data, decoded, "roundtrip failed at zstd level {level}");
        }
    }

    #[test]
    fn test_zstd_reduces_size_for_repetitive_data() {
        let data: Vec<u64> = vec![0u64; 10_000];
        let encoded = oxicode::encode_to_vec(&data).expect("encode failed");
        let compressed = compress(&encoded, Compression::Zstd).expect("compress failed");
        assert!(
            compressed.len() < encoded.len(),
            "Zstd compressed ({}) should be < encoded ({})",
            compressed.len(),
            encoded.len()
        );
    }
}

#[cfg(all(feature = "compression-zstd-pure", not(feature = "compression-zstd")))]
mod zstd_pure_decompression_tests {
    use oxicode::compression::{decompress, is_compressed};

    #[test]
    fn test_zstd_pure_module_is_accessible() {
        // When only compression-zstd-pure is enabled (no C zstd), the decompress
        // function is still callable but compression is not supported. We verify
        // that a buffer without a valid header returns an error gracefully.
        let garbage = vec![0xAAu8, 0xBB, 0xCC, 0xDD, 0xEE];
        assert!(!is_compressed(&garbage));
        let result = decompress(&garbage);
        assert!(result.is_err());
    }
}
