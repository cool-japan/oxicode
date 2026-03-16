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
mod iterator_encode_advanced_tests {
    use oxicode::{
        config, decode_iter_from_slice, decode_iter_from_slice_with_config, encode_iter_to_vec,
        encode_iter_to_vec_with_config,
    };

    // -----------------------------------------------------------------------
    // Test 1: Encode empty iterator produces just a length prefix (u64 varint)
    // The varint for 0 is a single byte 0x00, so the total output is 1 byte.
    // -----------------------------------------------------------------------
    #[test]
    fn encode_empty_iter_is_single_varint_zero_byte() {
        let encoded =
            encode_iter_to_vec(std::iter::empty::<u32>()).expect("encode empty iter failed");
        // A Vec<u32> of length 0 encodes as a single varint 0, which is one byte: [0x00]
        assert_eq!(
            encoded.len(),
            1,
            "empty iterator must encode to exactly 1 byte (varint 0)"
        );
        assert_eq!(
            encoded[0], 0x00,
            "the single byte must be 0x00 (varint encoding of 0)"
        );
    }

    // -----------------------------------------------------------------------
    // Test 2: Encode [1u32, 2u32, 3u32] and verify total byte count.
    // varint(3) = 1 byte; each u32 0-250 encodes as 1 varint byte => 3 bytes.
    // Total = 1 + 3 = 4 bytes.
    // -----------------------------------------------------------------------
    #[test]
    fn encode_three_small_u32s_byte_count() {
        let items = [1u32, 2u32, 3u32];
        let encoded = encode_iter_to_vec(items.iter().copied()).expect("encode [1,2,3] u32 failed");
        // length prefix varint(3) = 1 byte; each of 1,2,3 <= 250 => 1 byte each
        assert_eq!(
            encoded.len(),
            4,
            "encoding [1u32,2u32,3u32] must be exactly 4 bytes"
        );
    }

    // -----------------------------------------------------------------------
    // Test 3: Decode iterator with 5 items, collect and verify all values.
    // -----------------------------------------------------------------------
    #[test]
    fn decode_iter_five_items_collect_and_verify() {
        let source: Vec<u32> = vec![10, 20, 30, 40, 50];
        let encoded = oxicode::encode_to_vec(&source).expect("encode 5 u32s failed");
        let collected: Vec<u32> = decode_iter_from_slice::<u32>(&encoded)
            .expect("decode_iter init for 5 items failed")
            .collect::<Result<Vec<_>, _>>()
            .expect("collect 5 u32 items failed");
        assert_eq!(
            collected, source,
            "decoded 5 items must exactly match original"
        );
    }

    // -----------------------------------------------------------------------
    // Test 4: Decode until error — truncated bytes cause the iterator to
    // return an Err before yielding all items, or init itself fails.
    // -----------------------------------------------------------------------
    #[test]
    fn decode_iter_truncated_bytes_yields_error() {
        // Encode 8 u64 items (each u64 up to 250 => 1 byte, so 8 items + 1 prefix = 9 bytes)
        let source: Vec<u64> = vec![1, 2, 3, 4, 5, 6, 7, 8];
        let encoded = oxicode::encode_to_vec(&source).expect("encode 8 u64s failed");
        // Keep only the first 5 bytes (length prefix + some items, not all)
        let truncated = &encoded[..5];

        // Either init fails or the iterator returns Err partway
        let had_error = match decode_iter_from_slice::<u64>(truncated) {
            Err(_) => true,
            Ok(iter) => {
                let results: Vec<_> = iter.collect();
                results.iter().any(|r| r.is_err()) || results.len() < source.len()
            }
        };
        assert!(
            had_error,
            "truncated encoded bytes must result in an error or fewer items than expected"
        );
    }

    // -----------------------------------------------------------------------
    // Test 5: Mixed sizes — strings of varying lengths encode and decode
    // correctly, demonstrating variable-length element handling.
    // -----------------------------------------------------------------------
    #[test]
    fn encode_decode_iter_mixed_string_sizes() {
        let items: Vec<String> = vec![
            String::new(),                    // empty string
            "a".to_string(),                  // 1-char
            "hello".to_string(),              // 5-char
            "a".repeat(100),                  // 100-char
            "unicode: \u{1F600}".to_string(), // multi-byte unicode
        ];
        let encoded =
            encode_iter_to_vec(items.iter().cloned()).expect("encode mixed-size strings failed");
        let decoded: Vec<String> = decode_iter_from_slice::<String>(&encoded)
            .expect("decode_iter init for mixed strings failed")
            .collect::<Result<Vec<_>, _>>()
            .expect("collect mixed strings failed");
        assert_eq!(
            decoded, items,
            "mixed-size string roundtrip must preserve all items exactly"
        );
    }

    // -----------------------------------------------------------------------
    // Test 6: String iterator encode/decode with a diverse string set.
    // -----------------------------------------------------------------------
    #[test]
    fn encode_decode_iter_string_diverse_set() {
        let items: Vec<String> = vec![
            "foo".to_string(),
            "bar".to_string(),
            "baz".to_string(),
            "qux quux".to_string(),
            "the quick brown fox".to_string(),
        ];
        let encoded =
            encode_iter_to_vec(items.iter().cloned()).expect("encode diverse strings failed");
        let decoded: Vec<String> = decode_iter_from_slice::<String>(&encoded)
            .expect("decode_iter init for diverse strings failed")
            .collect::<Result<Vec<_>, _>>()
            .expect("collect diverse strings failed");
        assert_eq!(decoded, items);
    }

    // -----------------------------------------------------------------------
    // Test 7: Encode 1000 items and verify count by decoding with decode_iter.
    // -----------------------------------------------------------------------
    #[test]
    fn encode_1000_items_verify_count_via_decode_iter() {
        let encoded = encode_iter_to_vec(0u32..1000).expect("encode 0..1000 u32 failed");
        let count = decode_iter_from_slice::<u32>(&encoded)
            .expect("decode_iter init for 1000 items failed")
            .count();
        assert_eq!(count, 1000, "decoded count must be exactly 1000");
    }

    // -----------------------------------------------------------------------
    // Test 8: Iterator encode then decode is identity — values preserved.
    // -----------------------------------------------------------------------
    #[test]
    fn encode_iter_then_decode_iter_is_identity() {
        let original: Vec<i64> = vec![-1000, -1, 0, 1, 1000, i64::MAX, i64::MIN];
        let encoded = encode_iter_to_vec(original.iter().copied()).expect("encode i64 iter failed");
        let decoded: Vec<i64> = decode_iter_from_slice::<i64>(&encoded)
            .expect("decode_iter init for i64 identity test failed")
            .collect::<Result<Vec<_>, _>>()
            .expect("collect i64 identity items failed");
        assert_eq!(
            decoded, original,
            "encode-then-decode must be a lossless identity transform"
        );
    }

    // -----------------------------------------------------------------------
    // Test 9: Non-consuming decode iterator — the original byte slice is
    // not modified and can be decoded again after the first pass.
    // -----------------------------------------------------------------------
    #[test]
    fn decode_iter_non_consuming_byte_slice() {
        let source: Vec<u32> = vec![7, 14, 21, 28, 35];
        let encoded =
            oxicode::encode_to_vec(&source).expect("encode for non-consuming test failed");

        // First pass
        let first_pass: Vec<u32> = decode_iter_from_slice::<u32>(&encoded)
            .expect("first decode_iter init failed")
            .collect::<Result<Vec<_>, _>>()
            .expect("first collect failed");

        // Second pass — same slice is still intact
        let second_pass: Vec<u32> = decode_iter_from_slice::<u32>(&encoded)
            .expect("second decode_iter init failed")
            .collect::<Result<Vec<_>, _>>()
            .expect("second collect failed");

        assert_eq!(
            first_pass, second_pass,
            "byte slice must be reusable; both passes must yield identical results"
        );
        assert_eq!(first_pass, source);
    }

    // -----------------------------------------------------------------------
    // Test 10: Chained iterators — two separate iterators chained together
    // encode as if they were one contiguous sequence.
    // -----------------------------------------------------------------------
    #[test]
    fn encode_chained_iter_produces_unified_sequence() {
        let first_half = 0u32..5;
        let second_half = 5u32..10;
        let chained = first_half.chain(second_half);
        let encoded = encode_iter_to_vec(chained).expect("encode chained iter failed");

        let decoded: Vec<u32> = decode_iter_from_slice::<u32>(&encoded)
            .expect("decode_iter init for chained sequence failed")
            .collect::<Result<Vec<_>, _>>()
            .expect("collect chained items failed");

        let expected: Vec<u32> = (0u32..10).collect();
        assert_eq!(
            decoded, expected,
            "chained iterators must produce a single unified sequence"
        );
    }

    // -----------------------------------------------------------------------
    // Test 11: filter_map on decoded iterator — only even numbers survive.
    // -----------------------------------------------------------------------
    #[test]
    fn decode_iter_filter_map_even_only() {
        let source: Vec<u32> = (0u32..20).collect();
        let encoded = oxicode::encode_to_vec(&source).expect("encode 0..20 failed");

        let evens: Vec<u32> = decode_iter_from_slice::<u32>(&encoded)
            .expect("decode_iter init for filter_map test failed")
            .filter_map(|r| r.ok().filter(|v| v % 2 == 0))
            .collect();

        let expected: Vec<u32> = (0u32..20).filter(|v| v % 2 == 0).collect();
        assert_eq!(
            evens, expected,
            "filter_map on decoded iterator must yield only even numbers"
        );
    }

    // -----------------------------------------------------------------------
    // Test 12: take(N) on decode iterator — only the first N items are decoded.
    // -----------------------------------------------------------------------
    #[test]
    fn decode_iter_take_n_items() {
        let source: Vec<u32> = (100u32..200).collect();
        let encoded = oxicode::encode_to_vec(&source).expect("encode 100-item source failed");

        let first_five: Vec<u32> = decode_iter_from_slice::<u32>(&encoded)
            .expect("decode_iter init for take test failed")
            .take(5)
            .collect::<Result<Vec<_>, _>>()
            .expect("collect first 5 items failed");

        assert_eq!(first_five.len(), 5, "take(5) must yield exactly 5 items");
        assert_eq!(
            first_five,
            vec![100u32, 101, 102, 103, 104],
            "take(5) must yield the first 5 items in order"
        );
    }

    // -----------------------------------------------------------------------
    // Test 13: Encode nested Vec<Vec<u32>> via iterator — each item is a Vec.
    // -----------------------------------------------------------------------
    #[test]
    fn encode_decode_iter_nested_vec() {
        let items: Vec<Vec<u32>> = vec![
            vec![1, 2, 3],
            vec![],
            vec![10, 20, 30, 40, 50],
            vec![u32::MAX],
        ];
        let encoded =
            encode_iter_to_vec(items.iter().cloned()).expect("encode nested Vec iter failed");
        let decoded: Vec<Vec<u32>> = decode_iter_from_slice::<Vec<u32>>(&encoded)
            .expect("decode_iter init for nested Vec failed")
            .collect::<Result<Vec<_>, _>>()
            .expect("collect nested Vec items failed");
        assert_eq!(
            decoded, items,
            "nested Vec<u32> roundtrip via iterator must preserve all items"
        );
    }

    // -----------------------------------------------------------------------
    // Test 14: encode_iter_to_vec_with_config using fixed int encoding.
    // With fixed encoding, each u32 is exactly 4 bytes; 3 items + 8-byte count prefix.
    // -----------------------------------------------------------------------
    #[test]
    fn encode_iter_to_vec_with_fixed_int_config() {
        let items = [1u32, 2u32, 3u32];
        let cfg = config::standard().with_fixed_int_encoding();
        let encoded = encode_iter_to_vec_with_config(items.iter().copied(), cfg)
            .expect("encode with fixed int config failed");

        // Decode with the same config to verify correctness
        let decoded: Vec<u32> = decode_iter_from_slice_with_config::<u32, _>(&encoded, cfg)
            .expect("decode_iter init with fixed config failed")
            .collect::<Result<Vec<_>, _>>()
            .expect("collect with fixed config failed");
        assert_eq!(
            decoded,
            vec![1u32, 2u32, 3u32],
            "fixed-int config encode/decode must roundtrip correctly"
        );
        // With fixed int encoding: length is 8 bytes (u64 fixed), 3 × 4-byte u32 = 12 bytes
        assert_eq!(
            encoded.len(),
            8 + 3 * 4,
            "with fixed int encoding: 8-byte length prefix + 3 × 4 bytes = 20 bytes total"
        );
    }

    // -----------------------------------------------------------------------
    // Test 15: Verify byte format — the first byte(s) encode the item count
    // as a varint.  For count = 5 (<=250), exactly byte [0x05] comes first.
    // -----------------------------------------------------------------------
    #[test]
    fn encode_iter_first_bytes_are_varint_item_count() {
        let items: Vec<u32> = vec![100u32, 200u32, 50u32, 75u32, 25u32]; // 5 items
        let encoded =
            encode_iter_to_vec(items.iter().copied()).expect("encode 5 u32s for format test");
        // varint(5) = single byte 0x05
        assert_eq!(
            encoded[0], 0x05,
            "first byte must be varint(5) = 0x05 for a 5-item sequence"
        );
    }

    // -----------------------------------------------------------------------
    // Test 16: Decode exact item count — iterator must stop after exactly
    // as many items as the encoded length prefix specifies.
    // -----------------------------------------------------------------------
    #[test]
    fn decode_iter_stops_after_exact_item_count() {
        // Encode a 7-element sequence
        let source: Vec<u16> = vec![10, 20, 30, 40, 50, 60, 70];
        let encoded = oxicode::encode_to_vec(&source).expect("encode 7 u16s failed");

        let mut iter = decode_iter_from_slice::<u16>(&encoded)
            .expect("decode_iter init for exact count test failed");

        let mut collected = Vec::new();
        for result in iter.by_ref() {
            collected.push(result.expect("item decode failed in exact count test"));
        }
        // Explicitly verify next() is now None
        assert!(
            iter.next().is_none(),
            "iterator must return None after all items are consumed"
        );
        assert_eq!(
            collected.len(),
            7,
            "exactly 7 items must be decoded — no more, no fewer"
        );
        assert_eq!(collected, source);
    }

    // -----------------------------------------------------------------------
    // Test 17: Large items — strings of 256 bytes encode and decode correctly.
    // -----------------------------------------------------------------------
    #[test]
    fn encode_decode_iter_large_256_byte_strings() {
        let large_string = "x".repeat(256);
        let items: Vec<String> = vec![large_string.clone(), large_string.clone(), large_string];
        let encoded =
            encode_iter_to_vec(items.iter().cloned()).expect("encode large strings failed");
        let decoded: Vec<String> = decode_iter_from_slice::<String>(&encoded)
            .expect("decode_iter init for large strings failed")
            .collect::<Result<Vec<_>, _>>()
            .expect("collect large strings failed");
        assert_eq!(decoded, items, "256-byte strings must roundtrip correctly");
        for s in &decoded {
            assert_eq!(
                s.len(),
                256,
                "each decoded string must be exactly 256 bytes"
            );
        }
    }

    // -----------------------------------------------------------------------
    // Test 18: Boolean iterator — true/false values roundtrip correctly.
    // -----------------------------------------------------------------------
    #[test]
    fn encode_decode_iter_booleans() {
        let items: Vec<bool> = vec![true, false, true, true, false, false, true];
        let encoded =
            encode_iter_to_vec(items.iter().copied()).expect("encode bool iterator failed");
        let decoded: Vec<bool> = decode_iter_from_slice::<bool>(&encoded)
            .expect("decode_iter init for booleans failed")
            .collect::<Result<Vec<_>, _>>()
            .expect("collect booleans failed");
        assert_eq!(
            decoded, items,
            "boolean iterator roundtrip must preserve all values"
        );
    }

    // -----------------------------------------------------------------------
    // Test 19: f64 iterator — extreme and special values roundtrip exactly.
    // -----------------------------------------------------------------------
    #[test]
    fn encode_decode_iter_f64_extreme_values() {
        let items: Vec<f64> = vec![
            0.0_f64,
            -0.0_f64,
            f64::MAX,
            f64::MIN,
            f64::MIN_POSITIVE,
            f64::INFINITY,
            f64::NEG_INFINITY,
            1.23456789012345e100,
            -9.87654321098765e-100,
        ];
        let encoded =
            encode_iter_to_vec(items.iter().copied()).expect("encode f64 extreme values failed");
        let decoded: Vec<f64> = decode_iter_from_slice::<f64>(&encoded)
            .expect("decode_iter init for f64 extremes failed")
            .collect::<Result<Vec<_>, _>>()
            .expect("collect f64 extreme values failed");
        assert_eq!(
            decoded.len(),
            items.len(),
            "f64 extreme value iterator must decode the same number of items"
        );
        // IEEE 754 binary encoding is exact for finite values and special bit patterns
        for (i, (got, expected)) in decoded.iter().zip(items.iter()).enumerate() {
            assert_eq!(
                got.to_bits(),
                expected.to_bits(),
                "f64 at index {} must have identical bit pattern after roundtrip",
                i
            );
        }
    }

    // -----------------------------------------------------------------------
    // Test 20: Option<u32> iterator — Some/None mix roundtrips without loss.
    // -----------------------------------------------------------------------
    #[test]
    fn encode_decode_iter_option_u32_mixed() {
        let items: Vec<Option<u32>> =
            vec![Some(0), None, Some(u32::MAX), None, Some(42), Some(1), None];
        let encoded =
            encode_iter_to_vec(items.iter().copied()).expect("encode Option<u32> iter failed");
        let decoded: Vec<Option<u32>> = decode_iter_from_slice::<Option<u32>>(&encoded)
            .expect("decode_iter init for Option<u32> failed")
            .collect::<Result<Vec<_>, _>>()
            .expect("collect Option<u32> items failed");
        assert_eq!(
            decoded, items,
            "Option<u32> mix with Some/None must roundtrip exactly"
        );
    }

    // -----------------------------------------------------------------------
    // Test 21: (u32, String) tuple iterator — heterogeneous tuple type roundtrips.
    // -----------------------------------------------------------------------
    #[test]
    fn encode_decode_iter_u32_string_tuple() {
        let items: Vec<(u32, String)> = vec![
            (0, "zero".to_string()),
            (1, "one".to_string()),
            (42, "forty-two".to_string()),
            (u32::MAX, "max".to_string()),
            (100, String::new()),
        ];
        let encoded =
            encode_iter_to_vec(items.iter().cloned()).expect("encode (u32,String) iter failed");
        let decoded: Vec<(u32, String)> = decode_iter_from_slice::<(u32, String)>(&encoded)
            .expect("decode_iter init for (u32,String) failed")
            .collect::<Result<Vec<_>, _>>()
            .expect("collect (u32,String) tuples failed");
        assert_eq!(
            decoded, items,
            "(u32, String) tuple iterator roundtrip must preserve all tuples exactly"
        );
    }

    // -----------------------------------------------------------------------
    // Test 22: Verify decode stops after exactly N items — using a large
    // buffer that deliberately contains more data after the encoded sequence.
    // Decode iterator must not read beyond what the length prefix declares.
    // -----------------------------------------------------------------------
    #[test]
    fn decode_iter_stops_after_exactly_n_items_with_trailing_data() {
        let source: Vec<u32> = vec![11, 22, 33, 44, 55];
        let mut encoded = oxicode::encode_to_vec(&source).expect("encode 5 u32s failed");

        // Append extra arbitrary bytes after the valid encoded sequence
        let trailing: Vec<u8> = vec![0xDE, 0xAD, 0xBE, 0xEF, 0xFF, 0xFF];
        let original_len = encoded.len();
        encoded.extend_from_slice(&trailing);

        // Decode from the extended buffer — the iterator must read only the 5 declared items
        let decoded: Vec<u32> = decode_iter_from_slice::<u32>(&encoded)
            .expect("decode_iter init with trailing data failed")
            .collect::<Result<Vec<_>, _>>()
            .expect("collect items with trailing data failed");

        assert_eq!(
            decoded.len(),
            5,
            "iterator must decode exactly 5 items declared in the length prefix"
        );
        assert_eq!(
            decoded, source,
            "decoded values must match original, ignoring trailing bytes"
        );
        // The original encoded slice has not grown
        let _ = original_len; // used to construct the extended buffer above
    }
}
