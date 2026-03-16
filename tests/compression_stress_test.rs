//! Stress tests for oxicode compression: correctness and edge cases.
//!
//! These 20 tests focus on large data volumes, structural diversity, PI/E
//! float precision, multi-struct batches, nested collections, and stats
//! accuracy — none of which are covered by the existing suites.

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
mod compression_stress_tests {
    // -----------------------------------------------------------------------
    // LZ4 tests (12)
    // -----------------------------------------------------------------------

    #[cfg(feature = "compression-lz4")]
    mod lz4 {
        use oxicode::compression::{compress, compress_with_stats, decompress, Compression};
        use oxicode::{Decode, Encode};
        use std::collections::BTreeMap;
        use std::f64::consts::{E, PI};

        // ---- shared helper structs ----------------------------------------

        #[derive(Debug, PartialEq, Encode, Decode)]
        struct BigStringStruct {
            id: u64,
            payload: String,
        }

        #[derive(Debug, PartialEq, Encode, Decode)]
        struct RepetitiveUnit {
            counter: u32,
            label: String,
        }

        #[derive(Debug, PartialEq, Encode, Decode)]
        struct PiStruct {
            pi: f64,
            e: f64,
            derived: f64,
        }

        #[derive(Debug, PartialEq, Encode, Decode)]
        struct EmptyCompressStruct {}

        #[derive(Debug, PartialEq, Encode, Decode)]
        struct BatchItem {
            index: u32,
            value: f64,
            tag: String,
        }

        // ------------------------------------------------------------------
        // Test 1 – LZ4 roundtrip for 50 000-element Vec<u32>
        // ------------------------------------------------------------------

        #[test]
        fn test_lz4_roundtrip_50000_vec_u32() {
            let data: Vec<u32> = (0u32..50_000).collect();
            let encoded = oxicode::encode_to_vec(&data).expect("encode Vec<u32> failed");
            let compressed = compress(&encoded, Compression::Lz4).expect("compress failed");
            let decompressed = decompress(&compressed).expect("decompress failed");
            let (decoded, _): (Vec<u32>, usize) =
                oxicode::decode_from_slice(&decompressed).expect("decode Vec<u32> failed");
            assert_eq!(
                data, decoded,
                "50 000-element Vec<u32> must survive LZ4 roundtrip"
            );
        }

        // ------------------------------------------------------------------
        // Test 2 – LZ4 roundtrip for struct with 1000-char string
        // ------------------------------------------------------------------

        #[test]
        fn test_lz4_roundtrip_struct_with_1000_char_string() {
            let payload: String = "oxicode-lz4-stress-".repeat(53); // 19*53 = 1007 chars
            let data = BigStringStruct {
                id: 0xABCD_1234,
                payload,
            };
            let encoded = oxicode::encode_to_vec(&data).expect("encode BigStringStruct failed");
            let compressed = compress(&encoded, Compression::Lz4).expect("compress failed");
            let decompressed = decompress(&compressed).expect("decompress failed");
            let (decoded, _): (BigStringStruct, usize) =
                oxicode::decode_from_slice(&decompressed).expect("decode BigStringStruct failed");
            assert_eq!(
                data, decoded,
                "struct with 1000-char string must survive LZ4 roundtrip"
            );
        }

        // ------------------------------------------------------------------
        // Test 3 – LZ4 compress Vec<u8> of sequential bytes (0..255 repeated)
        // ------------------------------------------------------------------

        #[test]
        fn test_lz4_compress_sequential_bytes_0_255_repeated() {
            // 256 * 40 = 10 240 bytes; 0..255 repeated 40 times
            let data: Vec<u8> = (0u8..=255).cycle().take(10_240).collect();
            let encoded = oxicode::encode_to_vec(&data).expect("encode sequential bytes failed");
            let compressed = compress(&encoded, Compression::Lz4).expect("compress failed");
            let decompressed = decompress(&compressed).expect("decompress failed");
            let (decoded, _): (Vec<u8>, usize) =
                oxicode::decode_from_slice(&decompressed).expect("decode sequential bytes failed");
            assert_eq!(
                data, decoded,
                "sequential 0..255 repeated bytes must survive LZ4 roundtrip"
            );
        }

        // ------------------------------------------------------------------
        // Test 4 – LZ4 compress highly repetitive data: 10 000 copies of same struct
        // ------------------------------------------------------------------

        #[test]
        fn test_lz4_compress_10000_repetitive_structs() {
            let item = RepetitiveUnit {
                counter: 42,
                label: "repeat".into(),
            };
            let data: Vec<RepetitiveUnit> = (0..10_000)
                .map(|_| RepetitiveUnit {
                    counter: item.counter,
                    label: item.label.clone(),
                })
                .collect();
            let encoded =
                oxicode::encode_to_vec(&data).expect("encode 10 000 repetitive structs failed");
            let compressed = compress(&encoded, Compression::Lz4).expect("compress failed");
            // Highly repetitive data must compress to less than the original
            assert!(
                compressed.len() < encoded.len(),
                "10 000 identical structs must compress: encoded={} compressed={}",
                encoded.len(),
                compressed.len()
            );
            let decompressed = decompress(&compressed).expect("decompress failed");
            let (decoded, _): (Vec<RepetitiveUnit>, usize) =
                oxicode::decode_from_slice(&decompressed)
                    .expect("decode 10 000 repetitive structs failed");
            assert_eq!(data, decoded);
        }

        // ------------------------------------------------------------------
        // Test 5 – LZ4 compress random-ish data derived from PI digits
        // ------------------------------------------------------------------

        #[test]
        fn test_lz4_compress_pi_derived_data() {
            // Build pseudo-random bytes from successive multiplications of PI
            let mut val = PI;
            let data: Vec<u8> = (0..4_096)
                .map(|_| {
                    val = (val * E).fract();
                    (val * 256.0) as u8
                })
                .collect();
            let encoded = oxicode::encode_to_vec(&data).expect("encode PI-derived data failed");
            let compressed = compress(&encoded, Compression::Lz4).expect("compress failed");
            let decompressed = decompress(&compressed).expect("decompress failed");
            let (decoded, _): (Vec<u8>, usize) =
                oxicode::decode_from_slice(&decompressed).expect("decode PI-derived data failed");
            assert_eq!(
                data, decoded,
                "PI-derived bytes must survive LZ4 roundtrip byte-for-byte"
            );
        }

        // ------------------------------------------------------------------
        // Test 6 – LZ4 compress then decompress matches original byte-for-byte
        // ------------------------------------------------------------------

        #[test]
        fn test_lz4_compress_decompress_byte_for_byte() {
            // Use a payload that interleaves structured data to stress byte fidelity
            let data: Vec<u64> = (0u64..2_000)
                .map(|i| i.wrapping_mul(0x9E37_79B9_7F4A_7C15))
                .collect();
            let encoded = oxicode::encode_to_vec(&data).expect("encode u64 payload failed");
            let compressed = compress(&encoded, Compression::Lz4).expect("compress failed");
            let decompressed = decompress(&compressed).expect("decompress failed");
            assert_eq!(
                encoded, decompressed,
                "decompressed bytes must match original encoded bytes exactly"
            );
        }

        // ------------------------------------------------------------------
        // Test 7 – LZ4 compress empty struct produces decompressable bytes
        // ------------------------------------------------------------------

        #[test]
        fn test_lz4_compress_empty_struct_decompressable() {
            let data = EmptyCompressStruct {};
            let encoded = oxicode::encode_to_vec(&data).expect("encode EmptyCompressStruct failed");
            let compressed = compress(&encoded, Compression::Lz4).expect("compress failed");
            // The compressed output must itself be decompressable
            let decompressed =
                decompress(&compressed).expect("decompress EmptyCompressStruct failed");
            let (decoded, _): (EmptyCompressStruct, usize) =
                oxicode::decode_from_slice(&decompressed)
                    .expect("decode EmptyCompressStruct failed");
            assert_eq!(data, decoded);
        }

        // ------------------------------------------------------------------
        // Test 8 – LZ4 compress 100 different structs, all decompress correctly
        // ------------------------------------------------------------------

        #[test]
        fn test_lz4_compress_100_different_structs_all_correct() {
            for i in 0u32..100 {
                let item = BatchItem {
                    index: i,
                    value: PI * (i as f64) + E,
                    tag: format!("batch-item-{i:03}"),
                };
                let encoded = oxicode::encode_to_vec(&item).expect("encode BatchItem failed");
                let compressed =
                    compress(&encoded, Compression::Lz4).expect("compress BatchItem failed");
                let decompressed = decompress(&compressed).expect("decompress BatchItem failed");
                let (decoded, _): (BatchItem, usize) =
                    oxicode::decode_from_slice(&decompressed).expect("decode BatchItem failed");
                assert_eq!(item, decoded, "batch item {i} must decompress correctly");
            }
        }

        // ------------------------------------------------------------------
        // Test 9 – LZ4 compression preserves exact f64 values (PI, E)
        // ------------------------------------------------------------------

        #[test]
        fn test_lz4_preserves_exact_f64_pi_e() {
            let data = PiStruct {
                pi: PI,
                e: E,
                derived: PI.powf(E),
            };
            let encoded = oxicode::encode_to_vec(&data).expect("encode PiStruct failed");
            let compressed = compress(&encoded, Compression::Lz4).expect("compress failed");
            let decompressed = decompress(&compressed).expect("decompress failed");
            let (decoded, _): (PiStruct, usize) =
                oxicode::decode_from_slice(&decompressed).expect("decode PiStruct failed");
            assert_eq!(
                data.pi.to_bits(),
                decoded.pi.to_bits(),
                "PI must be bit-exact after LZ4"
            );
            assert_eq!(
                data.e.to_bits(),
                decoded.e.to_bits(),
                "E must be bit-exact after LZ4"
            );
            assert_eq!(
                data.derived.to_bits(),
                decoded.derived.to_bits(),
                "PI^E must be bit-exact after LZ4"
            );
        }

        // ------------------------------------------------------------------
        // Test 10 – LZ4 compress BTreeMap with 500 entries
        // ------------------------------------------------------------------

        #[test]
        fn test_lz4_compress_btreemap_500_entries() {
            let mut data: BTreeMap<String, u64> = BTreeMap::new();
            for i in 0u64..500 {
                data.insert(format!("stress-key-{i:05}"), i.wrapping_mul(PI.to_bits()));
            }
            let encoded = oxicode::encode_to_vec(&data).expect("encode BTreeMap<500> failed");
            let compressed = compress(&encoded, Compression::Lz4).expect("compress failed");
            let decompressed = decompress(&compressed).expect("decompress failed");
            let (decoded, _): (BTreeMap<String, u64>, usize) =
                oxicode::decode_from_slice(&decompressed).expect("decode BTreeMap<500> failed");
            assert_eq!(
                data, decoded,
                "BTreeMap with 500 entries must survive LZ4 roundtrip"
            );
        }

        // ------------------------------------------------------------------
        // Test 11 – LZ4 compress nested Vec<Vec<String>>
        // ------------------------------------------------------------------

        #[test]
        fn test_lz4_compress_nested_vec_vec_string() {
            let data: Vec<Vec<String>> = (0u32..20)
                .map(|outer| {
                    (0u32..10)
                        .map(|inner| format!("row{outer:02}-col{inner:02}-value"))
                        .collect()
                })
                .collect();
            let encoded = oxicode::encode_to_vec(&data).expect("encode Vec<Vec<String>> failed");
            let compressed = compress(&encoded, Compression::Lz4).expect("compress failed");
            let decompressed = decompress(&compressed).expect("decompress failed");
            let (decoded, _): (Vec<Vec<String>>, usize) =
                oxicode::decode_from_slice(&decompressed).expect("decode Vec<Vec<String>> failed");
            assert_eq!(
                data, decoded,
                "nested Vec<Vec<String>> must survive LZ4 roundtrip"
            );
        }

        // ------------------------------------------------------------------
        // Test 12 – LZ4 compress_with_stats returns accurate sizes
        // ------------------------------------------------------------------

        #[test]
        fn test_lz4_compress_with_stats_accurate_sizes() {
            // Use a large, compressible payload so ratio is clearly > 1.0
            let data: Vec<u32> = vec![0xCAFE_BABEu32; 5_000];
            let encoded = oxicode::encode_to_vec(&data).expect("encode stats payload failed");
            let original_len = encoded.len();
            let (compressed, stats) = compress_with_stats(&encoded, Compression::Lz4)
                .expect("compress_with_stats failed");
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
                "compression ratio must be > 1.0 for highly repetitive data, got {}",
                stats.ratio()
            );
            assert!(
                stats.savings_percent() > 0.0,
                "savings_percent must be positive, got {}",
                stats.savings_percent()
            );
            // Sanity: decompressed result still matches original
            let decompressed = decompress(&compressed).expect("decompress stats payload failed");
            let (decoded, _): (Vec<u32>, usize) =
                oxicode::decode_from_slice(&decompressed).expect("decode stats payload failed");
            assert_eq!(data, decoded);
        }
    } // mod lz4

    // -----------------------------------------------------------------------
    // Zstd tests (8)
    // -----------------------------------------------------------------------

    #[cfg(feature = "compression-zstd")]
    mod zstd {
        use oxicode::compression::{compress, decompress, Compression};
        use oxicode::{Decode, Encode};
        use std::collections::BTreeMap;
        use std::f64::consts::{E, PI};

        // ---- shared helper structs ----------------------------------------

        #[derive(Debug, PartialEq, Encode, Decode)]
        struct LongStringZstd {
            id: u32,
            body: String,
        }

        #[derive(Debug, PartialEq, Encode, Decode)]
        struct FloatPrecisionZstd {
            pi: f64,
            e: f64,
            combo: f64,
        }

        #[derive(Debug, PartialEq, Encode, Decode)]
        struct EmptyZstdStress {}

        // ------------------------------------------------------------------
        // Test 13 – Zstd roundtrip for 50 000-element Vec<u32>
        // ------------------------------------------------------------------

        #[test]
        fn test_zstd_roundtrip_50000_vec_u32() {
            let data: Vec<u32> = (0u32..50_000).collect();
            let encoded = oxicode::encode_to_vec(&data).expect("encode Vec<u32> failed");
            let compressed = compress(&encoded, Compression::Zstd).expect("zstd compress failed");
            let decompressed = decompress(&compressed).expect("zstd decompress failed");
            let (decoded, _): (Vec<u32>, usize) =
                oxicode::decode_from_slice(&decompressed).expect("decode Vec<u32> failed");
            assert_eq!(
                data, decoded,
                "50 000-element Vec<u32> must survive Zstd roundtrip"
            );
        }

        // ------------------------------------------------------------------
        // Test 14 – Zstd roundtrip for struct with long string
        // ------------------------------------------------------------------

        #[test]
        fn test_zstd_roundtrip_struct_with_long_string() {
            let body: String = "zstd-long-string-payload-".repeat(60); // 1500 chars
            let data = LongStringZstd {
                id: 0xFEDC_BA98,
                body,
            };
            let encoded = oxicode::encode_to_vec(&data).expect("encode LongStringZstd failed");
            let compressed = compress(&encoded, Compression::Zstd).expect("zstd compress failed");
            let decompressed = decompress(&compressed).expect("zstd decompress failed");
            let (decoded, _): (LongStringZstd, usize) =
                oxicode::decode_from_slice(&decompressed).expect("decode LongStringZstd failed");
            assert_eq!(
                data, decoded,
                "struct with long string must survive Zstd roundtrip"
            );
        }

        // ------------------------------------------------------------------
        // Test 15 – Zstd compress repetitive data is smaller than original
        // ------------------------------------------------------------------

        #[test]
        fn test_zstd_repetitive_data_compresses_smaller() {
            // 20 000 bytes of one repeated byte value is maximally compressible
            let data: Vec<u8> = vec![0x5Au8; 20_000];
            let encoded = oxicode::encode_to_vec(&data).expect("encode repetitive bytes failed");
            let compressed = compress(&encoded, Compression::Zstd).expect("zstd compress failed");
            assert!(
                compressed.len() < encoded.len(),
                "Zstd must compress repetitive data: encoded={} compressed={}",
                encoded.len(),
                compressed.len()
            );
        }

        // ------------------------------------------------------------------
        // Test 16 – Zstd compress Vec<u8> of all zeros
        // ------------------------------------------------------------------

        #[test]
        fn test_zstd_compress_vec_u8_all_zeros() {
            let data: Vec<u8> = vec![0u8; 8_000];
            let encoded = oxicode::encode_to_vec(&data).expect("encode all-zeros failed");
            let compressed =
                compress(&encoded, Compression::Zstd).expect("zstd compress all-zeros failed");
            let decompressed = decompress(&compressed).expect("zstd decompress all-zeros failed");
            let (decoded, _): (Vec<u8>, usize) =
                oxicode::decode_from_slice(&decompressed).expect("decode all-zeros failed");
            assert_eq!(
                data, decoded,
                "Vec<u8> of all zeros must survive Zstd roundtrip"
            );
        }

        // ------------------------------------------------------------------
        // Test 17 – Zstd compress then decompress matches original byte-for-byte
        // ------------------------------------------------------------------

        #[test]
        fn test_zstd_compress_decompress_byte_for_byte() {
            // Use a non-trivial payload: interleaved ascending/descending values
            let data: Vec<u8> = (0u8..=255)
                .chain((0u8..=255).rev())
                .cycle()
                .take(6_144)
                .collect();
            let encoded = oxicode::encode_to_vec(&data).expect("encode interleaved bytes failed");
            let compressed = compress(&encoded, Compression::Zstd).expect("zstd compress failed");
            let decompressed = decompress(&compressed).expect("zstd decompress failed");
            assert_eq!(
                encoded, decompressed,
                "Zstd: decompressed bytes must match original encoded bytes exactly"
            );
        }

        // ------------------------------------------------------------------
        // Test 18 – Zstd compress empty struct
        // ------------------------------------------------------------------

        #[test]
        fn test_zstd_compress_empty_struct() {
            let data = EmptyZstdStress {};
            let encoded = oxicode::encode_to_vec(&data).expect("encode EmptyZstdStress failed");
            let compressed = compress(&encoded, Compression::Zstd)
                .expect("zstd compress EmptyZstdStress failed");
            let decompressed =
                decompress(&compressed).expect("zstd decompress EmptyZstdStress failed");
            let (decoded, _): (EmptyZstdStress, usize) =
                oxicode::decode_from_slice(&decompressed).expect("decode EmptyZstdStress failed");
            assert_eq!(data, decoded, "empty struct must survive Zstd roundtrip");
        }

        // ------------------------------------------------------------------
        // Test 19 – Zstd compression preserves exact f64 values (PI, E)
        // ------------------------------------------------------------------

        #[test]
        fn test_zstd_preserves_exact_f64_pi_e() {
            let data = FloatPrecisionZstd {
                pi: PI,
                e: E,
                combo: E.powf(PI),
            };
            let encoded = oxicode::encode_to_vec(&data).expect("encode FloatPrecisionZstd failed");
            let compressed = compress(&encoded, Compression::Zstd).expect("zstd compress failed");
            let decompressed = decompress(&compressed).expect("zstd decompress failed");
            let (decoded, _): (FloatPrecisionZstd, usize) =
                oxicode::decode_from_slice(&decompressed)
                    .expect("decode FloatPrecisionZstd failed");
            assert_eq!(
                data.pi.to_bits(),
                decoded.pi.to_bits(),
                "PI must be bit-exact after Zstd"
            );
            assert_eq!(
                data.e.to_bits(),
                decoded.e.to_bits(),
                "E must be bit-exact after Zstd"
            );
            assert_eq!(
                data.combo.to_bits(),
                decoded.combo.to_bits(),
                "E^PI must be bit-exact after Zstd"
            );
        }

        // ------------------------------------------------------------------
        // Test 20 – Zstd compress BTreeMap with 200 entries
        // ------------------------------------------------------------------

        #[test]
        fn test_zstd_compress_btreemap_200_entries() {
            let mut data: BTreeMap<String, f64> = BTreeMap::new();
            for i in 0u32..200 {
                data.insert(format!("zstd-stress-key-{i:04}"), PI * (i as f64) / E);
            }
            let encoded = oxicode::encode_to_vec(&data).expect("encode BTreeMap<200> failed");
            let compressed =
                compress(&encoded, Compression::Zstd).expect("zstd compress BTreeMap failed");
            let decompressed = decompress(&compressed).expect("zstd decompress BTreeMap failed");
            let (decoded, _): (BTreeMap<String, f64>, usize) =
                oxicode::decode_from_slice(&decompressed).expect("decode BTreeMap<200> failed");
            assert_eq!(
                data.len(),
                decoded.len(),
                "BTreeMap entry count must be preserved"
            );
            for (k, v) in &data {
                let dv = decoded.get(k).expect("key must be present in decoded map");
                assert_eq!(
                    v.to_bits(),
                    dv.to_bits(),
                    "f64 value for key {k} must be bit-exact after Zstd"
                );
            }
        }
    } // mod zstd
} // mod compression_stress_tests
