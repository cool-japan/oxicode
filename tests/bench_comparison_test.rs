//! Performance-verification tests for OxiCode encoding/decoding.
//!
//! These are correctness tests that verify encoding behaviour at scale —
//! they exercise the same code paths used by benchmarks but assert on
//! functional correctness rather than wall-clock time.

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
mod bench_comparison_tests {
    use oxicode::{decode_from_slice, encode_to_vec, Decode, Encode};

    // -----------------------------------------------------------------------
    // Shared struct definitions used across multiple tests
    // -----------------------------------------------------------------------

    #[derive(Debug, PartialEq, Encode, Decode)]
    struct SimpleRecord {
        id: u32,
        value: f64,
    }

    #[derive(Debug, PartialEq, Encode, Decode)]
    struct BenchStruct {
        id: u64,
        name: String,
        values: Vec<f64>,
        flag: bool,
    }

    // -----------------------------------------------------------------------
    // Test 1: Encode 10000 u32 values in a loop — verify all roundtrip
    // -----------------------------------------------------------------------
    #[test]
    fn test_encode_10000_u32_roundtrip() {
        for i in 0u32..10_000 {
            let encoded = encode_to_vec(&i).expect("encode u32 failed");
            let (decoded, _): (u32, _) = decode_from_slice(&encoded).expect("decode u32 failed");
            assert_eq!(decoded, i, "roundtrip mismatch at index {i}");
        }
    }

    // -----------------------------------------------------------------------
    // Test 2: Encode large struct 1000 times — verify consistency
    // -----------------------------------------------------------------------
    #[test]
    fn test_encode_large_struct_1000_times() {
        let pi = std::f64::consts::PI;
        let e = std::f64::consts::E;
        let original = BenchStruct {
            id: 0xDEAD_BEEF_u64,
            name: "oxicode-bench-struct".to_string(),
            values: vec![pi, e, pi * e, pi / e, e.powi(2)],
            flag: true,
        };
        for i in 0..1000u32 {
            let encoded = encode_to_vec(&original).expect("encode BenchStruct failed");
            let (decoded, _): (BenchStruct, _) =
                decode_from_slice(&encoded).expect("decode BenchStruct failed");
            assert_eq!(decoded, original, "BenchStruct mismatch at iteration {i}");
        }
    }

    // -----------------------------------------------------------------------
    // Test 3: Encode Vec<u64> with 100000 elements
    // -----------------------------------------------------------------------
    #[test]
    fn test_encode_vec_u64_100000_elements() {
        let data: Vec<u64> = (0u64..100_000).collect();
        let encoded = encode_to_vec(&data).expect("encode Vec<u64> failed");
        let (decoded, _): (Vec<u64>, _) =
            decode_from_slice(&encoded).expect("decode Vec<u64> failed");
        assert_eq!(decoded, data, "Vec<u64> roundtrip mismatch");
    }

    // -----------------------------------------------------------------------
    // Test 4: Decode 1000 structs sequentially
    // -----------------------------------------------------------------------
    #[test]
    fn test_decode_1000_structs_sequentially() {
        let pi = std::f64::consts::PI;
        let records: Vec<SimpleRecord> = (0u32..1000)
            .map(|i| SimpleRecord {
                id: i,
                value: pi * f64::from(i),
            })
            .collect();

        let encodings: Vec<Vec<u8>> = records
            .iter()
            .map(|r| encode_to_vec(r).expect("encode SimpleRecord failed"))
            .collect();

        for (i, (enc, orig)) in encodings.iter().zip(records.iter()).enumerate() {
            let (decoded, _): (SimpleRecord, _) =
                decode_from_slice(enc).expect("decode SimpleRecord failed");
            assert_eq!(&decoded, orig, "SimpleRecord mismatch at index {i}");
        }
    }

    // -----------------------------------------------------------------------
    // Test 5: Encode/decode with SIMD feature: large [f64; 1024]
    // -----------------------------------------------------------------------
    #[cfg(feature = "simd")]
    #[test]
    fn test_simd_large_f64_array() {
        let pi = std::f64::consts::PI;
        let data: [f64; 1024] = core::array::from_fn(|i| pi * (i as f64));
        let encoded = encode_to_vec(&data).expect("encode [f64; 1024] failed");
        let (decoded, _): ([f64; 1024], _) =
            decode_from_slice(&encoded).expect("decode [f64; 1024] failed");
        for (i, (a, b)) in data.iter().zip(decoded.iter()).enumerate() {
            assert_eq!(a.to_bits(), b.to_bits(), "f64 bit mismatch at index {i}");
        }
    }

    // -----------------------------------------------------------------------
    // Test 6: Encode then decode 5000 strings
    // -----------------------------------------------------------------------
    #[test]
    fn test_encode_decode_5000_strings() {
        for i in 0u32..5000 {
            let s = format!("item_{i}");
            let encoded = encode_to_vec(&s).expect("encode String failed");
            let (decoded, _): (String, _) =
                decode_from_slice(&encoded).expect("decode String failed");
            assert_eq!(decoded, s, "String mismatch at index {i}");
        }
    }

    // -----------------------------------------------------------------------
    // Test 7: Encode HashMap with 1000 entries
    // -----------------------------------------------------------------------
    #[test]
    fn test_encode_hashmap_1000_entries() {
        use std::collections::HashMap;

        let mut map: HashMap<String, u64> = HashMap::new();
        for i in 0u64..1000 {
            map.insert(format!("key_{i}"), i * 7 + 13);
        }
        let encoded = encode_to_vec(&map).expect("encode HashMap failed");
        let (decoded, _): (HashMap<String, u64>, _) =
            decode_from_slice(&encoded).expect("decode HashMap failed");
        assert_eq!(decoded, map, "HashMap roundtrip mismatch");
    }

    // -----------------------------------------------------------------------
    // Test 8: Batch encode Vec<(String, u64, Vec<u8>)> with 200 items
    // -----------------------------------------------------------------------
    #[test]
    fn test_batch_encode_200_complex_tuples() {
        let data: Vec<(String, u64, Vec<u8>)> = (0u64..200)
            .map(|i| {
                let label = format!("label_{i}");
                let payload: Vec<u8> = (0u8..20).map(|b| b.wrapping_add(i as u8)).collect();
                (label, i * 31, payload)
            })
            .collect();

        let encoded = encode_to_vec(&data).expect("encode Vec<(String,u64,Vec<u8>)> failed");
        let (decoded, _): (Vec<(String, u64, Vec<u8>)>, _) =
            decode_from_slice(&encoded).expect("decode Vec<(String,u64,Vec<u8>)> failed");
        assert_eq!(decoded, data, "complex tuple Vec roundtrip mismatch");
    }

    // -----------------------------------------------------------------------
    // Test 9: Encode with fixed_int (legacy) config is correct for 10000 u32 values
    // -----------------------------------------------------------------------
    #[test]
    fn test_fixed_int_config_10000_u32() {
        use oxicode::{config, decode_from_slice_with_config, encode_to_vec_with_config};

        let cfg = config::legacy();
        for i in 0u32..10_000 {
            let encoded =
                encode_to_vec_with_config(&i, cfg).expect("encode u32 with legacy config failed");
            let (decoded, _): (u32, _) = decode_from_slice_with_config(&encoded, cfg)
                .expect("decode u32 with legacy config failed");
            assert_eq!(decoded, i, "legacy config roundtrip mismatch at {i}");
        }
    }

    // -----------------------------------------------------------------------
    // Test 10: Encode with legacy config is correct for 1000 i64 values
    // -----------------------------------------------------------------------
    #[test]
    fn test_legacy_config_1000_i64() {
        use oxicode::{config, decode_from_slice_with_config, encode_to_vec_with_config};

        let cfg = config::legacy();
        let values: Vec<i64> = (0i64..1000).map(|i| i64::MIN / 1000 * i).collect();
        for (idx, &v) in values.iter().enumerate() {
            let encoded =
                encode_to_vec_with_config(&v, cfg).expect("encode i64 with legacy config failed");
            let (decoded, _): (i64, _) = decode_from_slice_with_config(&encoded, cfg)
                .expect("decode i64 with legacy config failed");
            assert_eq!(decoded, v, "legacy i64 config mismatch at index {idx}");
        }
    }

    // -----------------------------------------------------------------------
    // Test 11: Large BTreeMap<String, Vec<u32>> encode/decode correctness
    // -----------------------------------------------------------------------
    #[test]
    fn test_large_btreemap_encode_decode() {
        use std::collections::BTreeMap;

        let mut map: BTreeMap<String, Vec<u32>> = BTreeMap::new();
        for i in 0u32..100 {
            let key = format!("key_{i:03}");
            let vals: Vec<u32> = (0..10).map(|j| i * 10 + j).collect();
            map.insert(key, vals);
        }
        let encoded = encode_to_vec(&map).expect("encode BTreeMap failed");
        let (decoded, _): (BTreeMap<String, Vec<u32>>, _) =
            decode_from_slice(&encoded).expect("decode BTreeMap failed");
        assert_eq!(decoded, map, "BTreeMap roundtrip mismatch");
    }

    // -----------------------------------------------------------------------
    // Test 12: Streaming encode 10000 items then decode — verify count
    // -----------------------------------------------------------------------
    #[test]
    fn test_streaming_encode_10000_items_verify_count() {
        use oxicode::streaming::{BufferStreamingDecoder, BufferStreamingEncoder};

        let mut encoder = BufferStreamingEncoder::new();
        for i in 0u32..10_000 {
            encoder.write_item(&i).expect("stream write_item failed");
        }
        let encoded = encoder.finish();

        let mut decoder = BufferStreamingDecoder::new(&encoded);
        let decoded: Vec<u32> = decoder.read_all().expect("stream read_all failed");

        assert_eq!(decoded.len(), 10_000, "streaming decoded count mismatch");
        let expected: Vec<u32> = (0u32..10_000).collect();
        assert_eq!(decoded, expected, "streaming decoded values mismatch");
    }

    // -----------------------------------------------------------------------
    // Test 13: Compressed encode of repetitive data is smaller than uncompressed
    // -----------------------------------------------------------------------
    #[cfg(feature = "compression-lz4")]
    #[test]
    fn test_compressed_repetitive_data_smaller() {
        use oxicode::compression::{compress, Compression};

        let data: Vec<u64> = vec![0u64; 5000];
        let encoded = encode_to_vec(&data).expect("encode repetitive Vec<u64> failed");
        let compressed = compress(&encoded, Compression::Lz4).expect("lz4 compress failed");

        assert!(
            compressed.len() < encoded.len(),
            "compressed ({} bytes) should be < encoded ({} bytes) for all-zero data",
            compressed.len(),
            encoded.len()
        );
    }

    // -----------------------------------------------------------------------
    // Test 14: Checksum encode of 1000 structs — all verify correctly
    // -----------------------------------------------------------------------
    #[cfg(feature = "checksum")]
    #[test]
    fn test_checksum_1000_structs_all_verify() {
        use oxicode::checksum::{decode_with_checksum, encode_with_checksum};

        let pi = std::f64::consts::PI;
        for i in 0u32..1000 {
            let record = SimpleRecord {
                id: i,
                value: pi * f64::from(i),
            };
            let wrapped = encode_with_checksum(&record).expect("checksum encode failed");
            let (decoded, _): (SimpleRecord, _) =
                decode_with_checksum(&wrapped).expect("checksum decode failed");
            assert_eq!(decoded, record, "checksum struct mismatch at index {i}");
        }
    }

    // -----------------------------------------------------------------------
    // Test 15: Encode 100 nested structs (5 levels deep)
    // -----------------------------------------------------------------------
    #[test]
    fn test_encode_100_nested_structs_5_levels() {
        #[derive(Debug, PartialEq, Encode, Decode)]
        struct Level5 {
            depth: u64,
            val: f64,
        }
        #[derive(Debug, PartialEq, Encode, Decode)]
        struct Level4 {
            inner: Level5,
            tag: u32,
        }
        #[derive(Debug, PartialEq, Encode, Decode)]
        struct Level3 {
            inner: Level4,
            label: String,
        }
        #[derive(Debug, PartialEq, Encode, Decode)]
        struct Level2 {
            inner: Level3,
            count: u64,
        }
        #[derive(Debug, PartialEq, Encode, Decode)]
        struct Level1 {
            inner: Level2,
            active: bool,
        }

        let e = std::f64::consts::E;
        for i in 0u64..100 {
            let original = Level1 {
                inner: Level2 {
                    inner: Level3 {
                        inner: Level4 {
                            inner: Level5 {
                                depth: i,
                                val: e * (i as f64 + 1.0),
                            },
                            tag: i as u32 * 3,
                        },
                        label: format!("node_{i}"),
                    },
                    count: i * 1000,
                },
                active: i % 2 == 0,
            };
            let encoded = encode_to_vec(&original).expect("encode Level1 failed");
            let (decoded, _): (Level1, _) =
                decode_from_slice(&encoded).expect("decode Level1 failed");
            assert_eq!(decoded, original, "Level1 mismatch at iteration {i}");
        }
    }

    // -----------------------------------------------------------------------
    // Test 16: Encode 10000 booleans in Vec<bool>
    // -----------------------------------------------------------------------
    #[test]
    fn test_encode_decode_10000_booleans() {
        let data: Vec<bool> = (0u32..10_000).map(|i| i % 2 == 0).collect();
        let encoded = encode_to_vec(&data).expect("encode Vec<bool> failed");
        let (decoded, _): (Vec<bool>, _) =
            decode_from_slice(&encoded).expect("decode Vec<bool> failed");
        assert_eq!(decoded, data, "Vec<bool> roundtrip mismatch");
    }

    // -----------------------------------------------------------------------
    // Test 17: Encode/decode 1000 Option<u64> values
    // -----------------------------------------------------------------------
    #[test]
    fn test_encode_decode_1000_option_u64() {
        let values: Vec<Option<u64>> = (0u64..1000)
            .map(|i| if i % 3 == 0 { None } else { Some(i * 97 + 5) })
            .collect();

        for (idx, v) in values.iter().enumerate() {
            let encoded = encode_to_vec(v).expect("encode Option<u64> failed");
            let (decoded, _): (Option<u64>, _) =
                decode_from_slice(&encoded).expect("decode Option<u64> failed");
            assert_eq!(&decoded, v, "Option<u64> mismatch at index {idx}");
        }
    }

    // -----------------------------------------------------------------------
    // Test 18: Encode mixed-type array repeatedly 100 times — identical bytes
    // -----------------------------------------------------------------------
    #[test]
    fn test_mixed_type_array_100_times_identical_bytes() {
        #[derive(Debug, PartialEq, Encode, Decode)]
        struct MixedRecord {
            a: u32,
            b: String,
            c: Vec<u8>,
            d: f64,
        }

        let pi = std::f64::consts::PI;
        let record = MixedRecord {
            a: 0xCAFE_BABE,
            b: "oxicode-mixed".to_string(),
            c: vec![1, 2, 3, 4, 5, 6, 7, 8],
            d: pi,
        };

        let first = encode_to_vec(&record).expect("encode MixedRecord initial failed");
        for i in 1..100u32 {
            let encoded = encode_to_vec(&record).expect("encode MixedRecord repeated failed");
            assert_eq!(
                encoded, first,
                "MixedRecord encoding not identical at iteration {i}"
            );
        }
    }

    // -----------------------------------------------------------------------
    // Test 19: Encode large string repeatedly 100 times — identical bytes
    // -----------------------------------------------------------------------
    #[test]
    fn test_large_string_100_times_identical_bytes() {
        let pattern = "oxicode-performance-test-pattern-";
        let large_string: String = pattern.repeat(30); // ~990 characters

        let first = encode_to_vec(&large_string).expect("encode large String initial failed");
        for i in 1..100u32 {
            let encoded =
                encode_to_vec(&large_string).expect("encode large String repeated failed");
            assert_eq!(
                encoded, first,
                "large String encoding not identical at iteration {i}"
            );
        }
    }

    // -----------------------------------------------------------------------
    // Test 20: Encode/decode cycle produces identical results for complex struct
    // -----------------------------------------------------------------------
    #[test]
    fn test_complex_struct_encode_decode_cycle_identical() {
        #[derive(Debug, PartialEq, Encode, Decode)]
        struct ComplexStruct {
            id: u64,
            name: String,
            data: Vec<u32>,
            nested: SimpleRecord,
            tags: Vec<String>,
            ratio: f64,
        }

        let pi = std::f64::consts::PI;
        let original = ComplexStruct {
            id: 999_999_999,
            name: "oxicode-complex-roundtrip".to_string(),
            data: (0u32..50).collect(),
            nested: SimpleRecord { id: 42, value: pi },
            tags: vec!["alpha".to_string(), "beta".to_string(), "gamma".to_string()],
            ratio: pi,
        };

        let first_encoded =
            encode_to_vec(&original).expect("encode ComplexStruct first pass failed");
        let (decoded, _): (ComplexStruct, _) =
            decode_from_slice(&first_encoded).expect("decode ComplexStruct failed");
        assert_eq!(decoded, original, "ComplexStruct decode mismatch");

        let second_encoded =
            encode_to_vec(&decoded).expect("encode ComplexStruct second pass failed");
        assert_eq!(
            second_encoded, first_encoded,
            "ComplexStruct re-encoded bytes differ from first encoding"
        );
    }
}
