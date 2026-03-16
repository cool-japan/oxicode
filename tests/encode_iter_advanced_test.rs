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
mod encode_iter_advanced_tests {
    use oxicode::{decode_iter_from_slice, encode_iter_to_vec, Decode, Encode};
    use std::f64::consts::{E, PI};

    // Test 1: encode_iter_to_vec with a single-element iterator
    #[test]
    fn encode_iter_single_item_u32() {
        let items = [42u32];
        let encoded =
            encode_iter_to_vec(items.iter().copied()).expect("encode single item u32 failed");
        let (decoded, _): (Vec<u32>, _) =
            oxicode::decode_from_slice(&encoded).expect("decode single item u32 failed");
        assert_eq!(decoded, vec![42u32]);
    }

    // Test 2: encode_iter_to_vec for Vec<u64> produces same bytes as encode_to_vec
    #[test]
    fn encode_iter_bytes_match_vec_u64() {
        let data: Vec<u64> = vec![100, 200, 300];
        let iter_encoded =
            encode_iter_to_vec(data.iter().copied()).expect("encode_iter u64 failed");
        let vec_encoded = oxicode::encode_to_vec(&data).expect("encode_to_vec u64 failed");
        assert_eq!(
            iter_encoded, vec_encoded,
            "encode_iter_to_vec must produce same bytes as encode_to_vec for Vec<u64>"
        );
    }

    // Test 3: decode_iter_from_slice item count matches original
    #[test]
    fn decode_iter_count_matches() {
        let data: Vec<u32> = vec![10, 20, 30, 40, 50, 60, 70];
        let encoded = oxicode::encode_to_vec(&data).expect("encode 7-item Vec<u32> failed");
        let count = decode_iter_from_slice::<u32>(&encoded)
            .expect("decode_iter init for 7 items failed")
            .count();
        assert_eq!(count, 7, "decoded item count must equal original length");
    }

    // Test 4: decode_iter lazy evaluation via manual next() calls
    #[test]
    fn decode_iter_step_by_step() {
        let data: Vec<u32> = vec![111, 222, 333, 444, 555];
        let encoded = oxicode::encode_to_vec(&data).expect("encode step-by-step data failed");
        let mut iter = decode_iter_from_slice::<u32>(&encoded)
            .expect("decode_iter init for step-by-step failed");
        let first = iter
            .next()
            .expect("first item should exist")
            .expect("first item decode failed");
        let second = iter
            .next()
            .expect("second item should exist")
            .expect("second item decode failed");
        let third = iter
            .next()
            .expect("third item should exist")
            .expect("third item decode failed");
        assert_eq!(first, 111u32);
        assert_eq!(second, 222u32);
        assert_eq!(third, 333u32);
    }

    // Test 5: encode iterator of owned Strings
    #[test]
    fn encode_iter_of_owned_strings() {
        let items: Vec<String> = vec!["hello".to_string(), "world".to_string(), "rust".to_string()];
        let encoded =
            encode_iter_to_vec(items.iter().cloned()).expect("encode owned strings failed");
        let decoded: Vec<String> = decode_iter_from_slice::<String>(&encoded)
            .expect("decode_iter init for strings failed")
            .collect::<Result<Vec<_>, _>>()
            .expect("collect strings failed");
        assert_eq!(decoded, items);
    }

    // Test 6: encode empty iterator produces minimal bytes (same as empty Vec)
    #[test]
    fn encode_empty_iter_minimal_bytes() {
        let empty_iter = std::iter::empty::<u32>();
        let iter_encoded = encode_iter_to_vec(empty_iter).expect("encode empty iter failed");
        let empty_vec: Vec<u32> = Vec::new();
        let vec_encoded = oxicode::encode_to_vec(&empty_vec).expect("encode empty Vec failed");
        assert_eq!(
            iter_encoded, vec_encoded,
            "empty iterator encoding must match empty Vec encoding"
        );
    }

    // Test 7: decode iterator next() returns None after all items consumed
    #[test]
    fn decode_iter_next_returns_none_at_end() {
        let data: Vec<u32> = vec![1, 2, 3, 4, 5];
        let encoded = oxicode::encode_to_vec(&data).expect("encode termination test data failed");
        let iter = decode_iter_from_slice::<u32>(&encoded)
            .expect("decode_iter init for termination test failed");
        let actual_count = iter.count();
        assert_eq!(
            actual_count,
            data.len(),
            "iterator must yield exactly as many items as were encoded"
        );
    }

    // Test 8: encode/decode iterator roundtrip for structs (requires derive feature)
    #[cfg(feature = "derive")]
    #[test]
    fn encode_decode_iter_roundtrip_struct() {
        #[derive(Encode, Decode, PartialEq, Debug, Clone)]
        struct Point {
            x: i32,
            y: i32,
        }

        let points = vec![
            Point { x: 1, y: 2 },
            Point { x: -3, y: 4 },
            Point { x: 0, y: -100 },
        ];
        let encoded =
            encode_iter_to_vec(points.iter().cloned()).expect("encode Point structs failed");
        let decoded: Vec<Point> = decode_iter_from_slice::<Point>(&encoded)
            .expect("decode_iter init for Point failed")
            .collect::<Result<Vec<_>, _>>()
            .expect("collect Points failed");
        assert_eq!(decoded, points);
    }

    // Test 9: encode iterator with map/filter operations
    #[test]
    fn encode_iter_with_map_filter() {
        let filtered: Vec<u32> = (0u32..20).filter(|x| x % 2 == 0).collect();
        let encoded = encode_iter_to_vec((0u32..20).filter(|x| x % 2 == 0))
            .expect("encode map/filter failed");
        let decoded: Vec<u32> = decode_iter_from_slice::<u32>(&encoded)
            .expect("decode_iter init for filtered data failed")
            .collect::<Result<Vec<_>, _>>()
            .expect("collect filtered items failed");
        assert_eq!(decoded, filtered);
        assert_eq!(decoded, vec![0, 2, 4, 6, 8, 10, 12, 14, 16, 18]);
    }

    // Test 10: decode iterator terminates correctly at boundary
    #[test]
    fn decode_iter_terminates_at_boundary() {
        let data: Vec<u32> = vec![1, 2, 3];
        let encoded = oxicode::encode_to_vec(&data).expect("encode boundary test failed");
        let items: Vec<u32> = decode_iter_from_slice::<u32>(&encoded)
            .expect("decode_iter init for boundary test failed")
            .collect::<Result<Vec<_>, _>>()
            .expect("collect boundary items failed");
        assert_eq!(items.len(), 3);
        assert_eq!(items, data);
    }

    // Test 11: encode_iter_to_vec of 1000 u32 items
    #[test]
    fn encode_iter_1000_u32() {
        let encoded = encode_iter_to_vec(0u32..1000).expect("encode 1000 u32 failed");
        let (decoded, _): (Vec<u32>, _) =
            oxicode::decode_from_slice(&encoded).expect("decode 1000 u32 failed");
        assert_eq!(decoded.len(), 1000);
        assert_eq!(decoded[0], 0u32);
        assert_eq!(decoded[999], 999u32);
    }

    // Test 12: decode 1000 u32 items via decode_iter_from_slice
    #[test]
    fn decode_iter_1000_u32() {
        let data: Vec<u32> = (0u32..1000).collect();
        let encoded =
            oxicode::encode_to_vec(&data).expect("encode for 1000-item decode test failed");
        let sum: u64 = decode_iter_from_slice::<u32>(&encoded)
            .expect("decode_iter init for 1000 items failed")
            .map(|r| r.expect("item decode failed in 1000-item test") as u64)
            .sum();
        assert_eq!(sum, 999 * 1000 / 2);
    }

    // Test 13: encode_iter_to_vec with chained iterators
    #[test]
    fn encode_iter_chained() {
        let a = [1u32, 2, 3];
        let b = [4u32, 5, 6];
        let chained = a.iter().copied().chain(b.iter().copied());
        let encoded = encode_iter_to_vec(chained).expect("encode chained iter failed");
        let (decoded, _): (Vec<u32>, _) =
            oxicode::decode_from_slice(&encoded).expect("decode chained iter failed");
        assert_eq!(decoded, vec![1u32, 2, 3, 4, 5, 6]);
    }

    // Test 14: decode_iter_from_slice with complex struct type (requires derive feature)
    #[cfg(feature = "derive")]
    #[test]
    fn decode_iter_complex_struct() {
        #[derive(Encode, Decode, PartialEq, Debug, Clone)]
        struct NamedItem {
            id: u32,
            value: i64,
        }

        let items: Vec<NamedItem> = (0u32..5)
            .map(|i| NamedItem {
                id: i,
                value: (i as i64) * -10,
            })
            .collect();
        let encoded = oxicode::encode_to_vec(&items).expect("encode NamedItem vec failed");
        let decoded: Vec<NamedItem> = decode_iter_from_slice::<NamedItem>(&encoded)
            .expect("decode_iter init for NamedItem failed")
            .collect::<Result<Vec<_>, _>>()
            .expect("collect NamedItems failed");
        assert_eq!(decoded, items);
    }

    // Test 15: each item in encode_iter output is independently decodable as a Vec
    #[test]
    fn encode_iter_items_independently_decodable() {
        let items = [10u32, 20, 30];
        let encoded = encode_iter_to_vec(items.iter().copied()).expect("encode items failed");
        let (decoded, _): (Vec<u32>, _) =
            oxicode::decode_from_slice(&encoded).expect("decode items as Vec failed");
        assert_eq!(decoded[0], 10u32);
        assert_eq!(decoded[1], 20u32);
        assert_eq!(decoded[2], 30u32);
    }

    // Test 16: encode iterator of Options (Some/None mixed)
    #[test]
    fn encode_iter_of_options() {
        let items: Vec<Option<u32>> = vec![Some(1), None, Some(3), None, Some(5)];
        let encoded =
            encode_iter_to_vec(items.iter().copied()).expect("encode Option iterator failed");
        let decoded: Vec<Option<u32>> = decode_iter_from_slice::<Option<u32>>(&encoded)
            .expect("decode_iter init for Options failed")
            .collect::<Result<Vec<_>, _>>()
            .expect("collect Options failed");
        assert_eq!(decoded, items);
    }

    // Test 17: encode iterator of enums (requires derive feature)
    #[cfg(feature = "derive")]
    #[test]
    fn encode_iter_of_enums() {
        #[derive(Encode, Decode, PartialEq, Debug, Clone, Copy)]
        enum Color {
            Red,
            Green,
            Blue,
        }

        let items = vec![Color::Red, Color::Green, Color::Blue, Color::Red];
        let encoded =
            encode_iter_to_vec(items.iter().copied()).expect("encode Color enum iterator failed");
        let decoded: Vec<Color> = decode_iter_from_slice::<Color>(&encoded)
            .expect("decode_iter init for Color failed")
            .collect::<Result<Vec<_>, _>>()
            .expect("collect Colors failed");
        assert_eq!(decoded, items);
    }

    // Test 18: decode_iter_from_slice returns Err when items cannot be fully decoded
    #[test]
    fn decode_iter_error_truncated_data() {
        let data: Vec<u32> = vec![1, 2, 3, 4, 5];
        let encoded = oxicode::encode_to_vec(&data).expect("encode truncation test data failed");
        // Truncate to just after the length prefix but before all item data is present.
        // The length prefix encodes 5 items; we keep only enough bytes for ~2 items worth
        // of data so that iterating yields an error partway through.
        // We take exactly half the remaining bytes after the first byte (length prefix).
        let half_len = encoded.len() / 2;
        let truncated = &encoded[..half_len];
        // Either decode_iter_from_slice itself fails (bad length prefix),
        // OR the iterator returns Err on one of the items.
        match decode_iter_from_slice::<u32>(truncated) {
            Err(_) => {
                // Expected: init itself failed
            }
            Ok(iter) => {
                // The iterator must yield an error before finishing all 5 items
                let results: Vec<_> = iter.collect();
                let has_error = results.iter().any(|r| r.is_err());
                assert!(
                    has_error || results.len() < 5,
                    "truncated data must cause an error or shorter-than-expected sequence"
                );
            }
        }
    }

    // Test 19: encode Vec<f64> via iterator with PI and E values
    #[test]
    fn encode_iter_f64_pi_values() {
        let items = [PI, E, PI * 2.0, E * 2.0];
        let encoded =
            encode_iter_to_vec(items.iter().copied()).expect("encode f64 PI/E values failed");
        let decoded: Vec<f64> = decode_iter_from_slice::<f64>(&encoded)
            .expect("decode_iter init for f64 PI/E failed")
            .collect::<Result<Vec<_>, _>>()
            .expect("collect f64 PI/E values failed");
        assert_eq!(decoded.len(), 4);
        // Binary encoding of f64 should be exact (IEEE 754 roundtrip)
        assert_eq!(decoded[0], PI);
        assert_eq!(decoded[1], E);
        assert_eq!(decoded[2], PI * 2.0);
        assert_eq!(decoded[3], E * 2.0);
    }

    // Test 20: full roundtrip encode_iter_to_vec then decode_iter_from_slice
    #[test]
    fn encode_iter_then_decode_iter_full_roundtrip() {
        let encoded = encode_iter_to_vec(1u64..=10).expect("encode 1..=10 u64 failed");
        let sum: u64 = decode_iter_from_slice::<u64>(&encoded)
            .expect("decode_iter init for 1..=10 roundtrip failed")
            .map(|r| r.expect("item decode in roundtrip failed"))
            .sum();
        assert_eq!(sum, 55, "sum of 1..=10 must be 55");
    }
}
