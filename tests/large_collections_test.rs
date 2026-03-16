//! Comprehensive tests for large-collection encoding and decoding in OxiCode.
//!
//! Focuses on scale correctness, varint length-prefix boundaries, multi-level
//! nesting at realistic sizes, and error behaviour when data is truncated.

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
use std::collections::{BTreeMap, BTreeSet, HashMap, HashSet, LinkedList, VecDeque};

use oxicode::{config, decode_from_slice, encode_to_vec};

mod large_collections_tests {
    use super::*;

    // -----------------------------------------------------------------------
    // 1. Vec<u8> with 65535 elements — last value in the u16 varint tier
    //    Length prefix must be [251, 0xFF, 0xFF] (U16_BYTE tag + LE u16)
    // -----------------------------------------------------------------------
    #[test]
    fn test_vec_u8_65535_roundtrip() {
        let original: Vec<u8> = (0u32..65535).map(|i| (i % 256) as u8).collect();
        assert_eq!(original.len(), 65535);

        let bytes = encode_to_vec(&original).expect("encode Vec<u8> 65535 elements");
        let (decoded, consumed): (Vec<u8>, _) =
            decode_from_slice(&bytes).expect("decode Vec<u8> 65535 elements");

        assert_eq!(decoded.len(), 65535);
        assert_eq!(original, decoded);
        assert_eq!(consumed, bytes.len());
    }

    // -----------------------------------------------------------------------
    // 2. Vec<u8> with 65536 elements — first value in the u32 varint tier
    //    Length prefix must be [252, 0x00, 0x00, 0x01, 0x00] (U32_BYTE + LE u32)
    // -----------------------------------------------------------------------
    #[test]
    fn test_vec_u8_65536_roundtrip() {
        let original: Vec<u8> = (0u32..65536).map(|i| (i % 256) as u8).collect();
        assert_eq!(original.len(), 65536);

        let bytes = encode_to_vec(&original).expect("encode Vec<u8> 65536 elements");
        let (decoded, consumed): (Vec<u8>, _) =
            decode_from_slice(&bytes).expect("decode Vec<u8> 65536 elements");

        assert_eq!(decoded.len(), 65536);
        assert_eq!(original, decoded);
        assert_eq!(consumed, bytes.len());
    }

    // -----------------------------------------------------------------------
    // 3. Vec<u32> with 10000 elements — all values correct after roundtrip
    // -----------------------------------------------------------------------
    #[test]
    fn test_vec_u32_10000_roundtrip() {
        let original: Vec<u32> = (0u32..10000).map(|i| i * 3 + 7).collect();
        assert_eq!(original.len(), 10000);

        let bytes = encode_to_vec(&original).expect("encode Vec<u32> 10000 elements");
        let (decoded, consumed): (Vec<u32>, _) =
            decode_from_slice(&bytes).expect("decode Vec<u32> 10000 elements");

        assert_eq!(decoded.len(), 10000);
        assert_eq!(original, decoded);
        assert_eq!(consumed, bytes.len());
        // Spot-check a handful of entries
        assert_eq!(decoded[0], 7u32);
        assert_eq!(decoded[999], 999 * 3 + 7);
        assert_eq!(decoded[9999], 9999 * 3 + 7);
    }

    // -----------------------------------------------------------------------
    // 4. HashMap<String, Vec<u32>> with 500 entries
    // -----------------------------------------------------------------------
    #[test]
    fn test_hashmap_string_vec_u32_500_roundtrip() {
        let original: HashMap<String, Vec<u32>> = (0u32..500)
            .map(|i| {
                let key = format!("key_{:04}", i);
                let val: Vec<u32> = (0..5).map(|j| i * 5 + j).collect();
                (key, val)
            })
            .collect();

        assert_eq!(original.len(), 500);

        let bytes = encode_to_vec(&original).expect("encode HashMap<String, Vec<u32>> 500 entries");
        let (decoded, consumed): (HashMap<String, Vec<u32>>, _) =
            decode_from_slice(&bytes).expect("decode HashMap<String, Vec<u32>> 500 entries");

        assert_eq!(decoded.len(), 500);
        assert_eq!(original, decoded);
        assert_eq!(consumed, bytes.len());

        // Validate a specific entry
        let key = "key_0042".to_string();
        assert_eq!(
            decoded.get(&key).map(|v| v.as_slice()),
            Some([210u32, 211, 212, 213, 214].as_slice()),
            "entry key_0042 must hold [210..214]"
        );
    }

    // -----------------------------------------------------------------------
    // 5. BTreeMap with 10000 u64 entries
    // -----------------------------------------------------------------------
    #[test]
    fn test_btreemap_u64_10000_roundtrip() {
        let original: BTreeMap<u64, u64> = (0u64..10000).map(|i| (i, i * i)).collect();
        assert_eq!(original.len(), 10000);

        let bytes = encode_to_vec(&original).expect("encode BTreeMap<u64, u64> 10000 entries");
        let (decoded, consumed): (BTreeMap<u64, u64>, _) =
            decode_from_slice(&bytes).expect("decode BTreeMap<u64, u64> 10000 entries");

        assert_eq!(decoded.len(), 10000);
        assert_eq!(original, decoded);
        assert_eq!(consumed, bytes.len());
        assert_eq!(decoded.get(&9999), Some(&(9999 * 9999)));
    }

    // -----------------------------------------------------------------------
    // 6. Nested Vec<Vec<u8>> 100×100
    // -----------------------------------------------------------------------
    #[test]
    fn test_nested_vec_vec_u8_100x100_roundtrip() {
        let original: Vec<Vec<u8>> = (0u8..100)
            .map(|row| (0u8..100).map(|col| row.wrapping_add(col)).collect())
            .collect();
        assert_eq!(original.len(), 100);
        assert_eq!(original[0].len(), 100);

        let bytes = encode_to_vec(&original).expect("encode Vec<Vec<u8>> 100x100");
        let (decoded, consumed): (Vec<Vec<u8>>, _) =
            decode_from_slice(&bytes).expect("decode Vec<Vec<u8>> 100x100");

        assert_eq!(decoded.len(), 100);
        assert_eq!(decoded[0].len(), 100);
        assert_eq!(original, decoded);
        assert_eq!(consumed, bytes.len());
        // Spot-check corner values
        assert_eq!(decoded[0][0], 0u8);
        assert_eq!(decoded[99][99], 198u8); // 99 + 99 = 198
    }

    // -----------------------------------------------------------------------
    // 7. Vec<String> 1000 elements each 100 chars long
    // -----------------------------------------------------------------------
    #[test]
    fn test_vec_string_1000_x_100chars_roundtrip() {
        let original: Vec<String> = (0u32..1000)
            .map(|i| format!("{:0>100}", i)) // zero-padded to exactly 100 chars
            .collect();
        assert_eq!(original.len(), 1000);
        assert_eq!(original[0].len(), 100);
        assert_eq!(original[999].len(), 100);

        let bytes = encode_to_vec(&original).expect("encode Vec<String> 1000 x 100 chars");
        let (decoded, consumed): (Vec<String>, _) =
            decode_from_slice(&bytes).expect("decode Vec<String> 1000 x 100 chars");

        assert_eq!(decoded.len(), 1000);
        assert_eq!(original, decoded);
        assert_eq!(consumed, bytes.len());
        assert_eq!(decoded[42].len(), 100);
    }

    // -----------------------------------------------------------------------
    // 8. HashSet<u64> with 10000 entries
    // -----------------------------------------------------------------------
    #[test]
    fn test_hashset_u64_10000_roundtrip() {
        let original: HashSet<u64> = (0u64..10000).map(|i| i * 2 + 1).collect(); // all odd numbers
        assert_eq!(original.len(), 10000);

        let bytes = encode_to_vec(&original).expect("encode HashSet<u64> 10000 entries");
        let (decoded, consumed): (HashSet<u64>, _) =
            decode_from_slice(&bytes).expect("decode HashSet<u64> 10000 entries");

        assert_eq!(decoded.len(), 10000);
        assert_eq!(original, decoded);
        assert_eq!(consumed, bytes.len());
        // Verify all elements are odd
        assert!(decoded.iter().all(|v| v % 2 == 1));
        assert!(decoded.contains(&19999u64)); // last element: (9999*2+1)
    }

    // -----------------------------------------------------------------------
    // 9. BTreeSet<i64> with 1000 entries — decoded order must be ascending
    // -----------------------------------------------------------------------
    #[test]
    fn test_btreeset_i64_1000_sorted_roundtrip() {
        // Insert in reverse order to stress the sorted-order guarantee
        let original: BTreeSet<i64> = (0i64..1000).map(|i| -500 + i).collect();
        assert_eq!(original.len(), 1000);

        let bytes = encode_to_vec(&original).expect("encode BTreeSet<i64> 1000 entries");
        let (decoded, consumed): (BTreeSet<i64>, _) =
            decode_from_slice(&bytes).expect("decode BTreeSet<i64> 1000 entries");

        assert_eq!(decoded.len(), 1000);
        assert_eq!(original, decoded);
        assert_eq!(consumed, bytes.len());

        // Verify ascending iteration order
        let elements: Vec<i64> = decoded.iter().copied().collect();
        let mut sorted = elements.clone();
        sorted.sort_unstable();
        assert_eq!(
            elements, sorted,
            "BTreeSet<i64> must iterate in sorted order"
        );
        assert_eq!(*elements.first().expect("non-empty"), -500i64);
        assert_eq!(*elements.last().expect("non-empty"), 499i64);
    }

    // -----------------------------------------------------------------------
    // 10. Vec<(u32, String)> — 1000 tuples
    // -----------------------------------------------------------------------
    #[test]
    fn test_vec_tuple_u32_string_1000_roundtrip() {
        let original: Vec<(u32, String)> =
            (0u32..1000).map(|i| (i, format!("item_{}", i))).collect();
        assert_eq!(original.len(), 1000);

        let bytes = encode_to_vec(&original).expect("encode Vec<(u32, String)> 1000 tuples");
        let (decoded, consumed): (Vec<(u32, String)>, _) =
            decode_from_slice(&bytes).expect("decode Vec<(u32, String)> 1000 tuples");

        assert_eq!(decoded.len(), 1000);
        assert_eq!(original, decoded);
        assert_eq!(consumed, bytes.len());
        assert_eq!(decoded[500], (500u32, "item_500".to_string()));
    }

    // -----------------------------------------------------------------------
    // 11. Vec<Option<u64>> with mixed None/Some — 1000 items
    // -----------------------------------------------------------------------
    #[test]
    fn test_vec_option_u64_mixed_1000_roundtrip() {
        let original: Vec<Option<u64>> = (0u64..1000)
            .map(|i| if i % 3 == 0 { None } else { Some(i * i) })
            .collect();
        assert_eq!(original.len(), 1000);

        let nones = original.iter().filter(|o| o.is_none()).count();
        let somes = original.iter().filter(|o| o.is_some()).count();
        assert!(nones > 0, "should have None entries");
        assert!(somes > 0, "should have Some entries");

        let bytes = encode_to_vec(&original).expect("encode Vec<Option<u64>> 1000 mixed");
        let (decoded, consumed): (Vec<Option<u64>>, _) =
            decode_from_slice(&bytes).expect("decode Vec<Option<u64>> 1000 mixed");

        assert_eq!(decoded.len(), 1000);
        assert_eq!(original, decoded);
        assert_eq!(consumed, bytes.len());
        // i=0: 0 % 3 == 0 → None; i=1 → Some(1); i=2 → Some(4)
        // i=999: 999 % 3 == 0 → None; i=998 → Some(998*998)
        assert_eq!(decoded[0], None);
        assert_eq!(decoded[1], Some(1u64));
        assert_eq!(decoded[2], Some(4u64));
        assert_eq!(decoded[999], None);
        assert_eq!(decoded[998], Some(998u64 * 998));
    }

    // -----------------------------------------------------------------------
    // 12. HashMap<u32, String> round-trip identity check
    //     Every key maps to a unique string and is recovered exactly
    // -----------------------------------------------------------------------
    #[test]
    fn test_hashmap_u32_string_identity_roundtrip() {
        let original: HashMap<u32, String> = (0u32..200)
            .map(|i| (i, format!("value_for_{}", i)))
            .collect();
        assert_eq!(original.len(), 200);

        let bytes = encode_to_vec(&original).expect("encode HashMap<u32, String> identity");
        let (decoded, consumed): (HashMap<u32, String>, _) =
            decode_from_slice(&bytes).expect("decode HashMap<u32, String> identity");

        assert_eq!(decoded.len(), 200);
        assert_eq!(consumed, bytes.len());

        // Every key must yield the exact expected string
        for i in 0u32..200 {
            let expected = format!("value_for_{}", i);
            assert_eq!(
                decoded.get(&i).map(String::as_str),
                Some(expected.as_str()),
                "HashMap entry {i} mismatch"
            );
        }
    }

    // -----------------------------------------------------------------------
    // 13. Multi-level nested: Vec<HashMap<String, Vec<u32>>>
    // -----------------------------------------------------------------------
    #[test]
    fn test_multilevel_nested_vec_hashmap_vec_roundtrip() {
        let original: Vec<HashMap<String, Vec<u32>>> = (0u32..20)
            .map(|outer| {
                let mut map: HashMap<String, Vec<u32>> = HashMap::new();
                for inner in 0u32..10 {
                    let key = format!("k_{}_{}", outer, inner);
                    let val: Vec<u32> = (0..5).map(|x| outer * 100 + inner * 10 + x).collect();
                    map.insert(key, val);
                }
                map
            })
            .collect();
        assert_eq!(original.len(), 20);
        assert_eq!(original[0].len(), 10);

        let bytes = encode_to_vec(&original).expect("encode Vec<HashMap<String, Vec<u32>>>");
        let (decoded, consumed): (Vec<HashMap<String, Vec<u32>>>, _) =
            decode_from_slice(&bytes).expect("decode Vec<HashMap<String, Vec<u32>>>");

        assert_eq!(decoded.len(), 20);
        assert_eq!(original, decoded);
        assert_eq!(consumed, bytes.len());
    }

    // -----------------------------------------------------------------------
    // 14. Large byte payload: Vec<u8> with 100000 elements
    // -----------------------------------------------------------------------
    #[test]
    fn test_vec_u8_100000_large_roundtrip() {
        let original: Vec<u8> = (0u32..100000).map(|i| (i % 256) as u8).collect();
        assert_eq!(original.len(), 100000);

        let bytes = encode_to_vec(&original).expect("encode Vec<u8> 100000 elements");
        let (decoded, consumed): (Vec<u8>, _) =
            decode_from_slice(&bytes).expect("decode Vec<u8> 100000 elements");

        assert_eq!(decoded.len(), 100000);
        assert_eq!(original, decoded);
        assert_eq!(consumed, bytes.len());
        // The total encoded size is: 5 (U32_BYTE + 4 LE bytes) + 100000 data bytes
        assert_eq!(bytes.len(), 100000 + 5);
    }

    // -----------------------------------------------------------------------
    // 15. Length prefix verification for 251-element Vec (first u16 varint)
    //     251 > SINGLE_BYTE_MAX(250) → prefix is [251, 0xFB, 0x00] (LE u16)
    // -----------------------------------------------------------------------
    #[test]
    fn test_vec_251_elements_length_prefix_is_u16_varint() {
        let original: Vec<u8> = vec![0xAAu8; 251];
        assert_eq!(original.len(), 251);

        let bytes = encode_to_vec(&original).expect("encode Vec<u8> 251 elements");

        // Varint for 251: tag byte 251 (U16_BYTE), then 251 as LE u16 = [0xFB, 0x00]
        assert_eq!(bytes[0], 251u8, "tag byte must be U16_BYTE (251)");
        let len_le = u16::from_le_bytes([bytes[1], bytes[2]]);
        assert_eq!(len_le, 251u16, "LE u16 payload must equal 251");
        // Remaining bytes are the actual data
        assert_eq!(bytes.len(), 3 + 251);
        assert!(bytes[3..].iter().all(|&b| b == 0xAA));

        let (decoded, consumed): (Vec<u8>, _) =
            decode_from_slice(&bytes).expect("decode Vec<u8> 251 elements");
        assert_eq!(decoded, original);
        assert_eq!(consumed, bytes.len());
    }

    // -----------------------------------------------------------------------
    // 16. Length prefix for 1000-element Vec — also a 3-byte u16 varint
    //     1000 fits in u16: tag byte 251 then 0xE8 0x03 (LE)
    // -----------------------------------------------------------------------
    #[test]
    fn test_vec_1000_elements_length_prefix_is_3byte_varint() {
        let original: Vec<u8> = vec![0x5Au8; 1000];
        assert_eq!(original.len(), 1000);

        let bytes = encode_to_vec(&original).expect("encode Vec<u8> 1000 elements");

        // Varint for 1000: U16_BYTE tag (251), then 1000 as LE u16 = [0xE8, 0x03]
        assert_eq!(bytes[0], 251u8, "tag byte must be U16_BYTE (251)");
        let len_le = u16::from_le_bytes([bytes[1], bytes[2]]);
        assert_eq!(len_le, 1000u16, "LE u16 payload must equal 1000");
        assert_eq!(bytes.len(), 3 + 1000);

        let (decoded, consumed): (Vec<u8>, _) =
            decode_from_slice(&bytes).expect("decode Vec<u8> 1000 elements");
        assert_eq!(decoded, original);
        assert_eq!(consumed, bytes.len());
    }

    // -----------------------------------------------------------------------
    // 17. Empty HashMap encoded size — must be exactly 1 byte (varint 0)
    // -----------------------------------------------------------------------
    #[test]
    fn test_empty_hashmap_encoded_size_is_one_byte() {
        let original: HashMap<String, u64> = HashMap::new();

        let bytes = encode_to_vec(&original).expect("encode empty HashMap<String, u64>");

        // An empty collection encodes its length (0) as a single byte varint
        assert_eq!(
            bytes.len(),
            1,
            "empty HashMap must encode to exactly 1 byte (varint 0)"
        );
        assert_eq!(bytes[0], 0u8, "varint for length 0 must be the byte 0x00");

        let (decoded, consumed): (HashMap<String, u64>, _) =
            decode_from_slice(&bytes).expect("decode empty HashMap<String, u64>");
        assert!(decoded.is_empty());
        assert_eq!(consumed, 1);
    }

    // -----------------------------------------------------------------------
    // 18. Large BTreeMap sorted order preserved after encode/decode
    // -----------------------------------------------------------------------
    #[test]
    fn test_large_btreemap_sorted_order_preserved() {
        // Insert 2000 entries with keys that have no natural insertion order
        let original: BTreeMap<u32, String> = (0u32..2000)
            .rev() // insert in descending order
            .map(|i| (i, format!("v{}", i)))
            .collect();
        assert_eq!(original.len(), 2000);

        let bytes = encode_to_vec(&original).expect("encode BTreeMap sorted order 2000 entries");
        let (decoded, consumed): (BTreeMap<u32, String>, _) =
            decode_from_slice(&bytes).expect("decode BTreeMap sorted order 2000 entries");

        assert_eq!(decoded.len(), 2000);
        assert_eq!(consumed, bytes.len());

        // BTreeMap must iterate in strictly ascending key order
        let keys: Vec<u32> = decoded.keys().copied().collect();
        let mut sorted = keys.clone();
        sorted.sort_unstable();
        assert_eq!(keys, sorted, "BTreeMap keys must be in ascending order");
        assert_eq!(keys[0], 0u32);
        assert_eq!(keys[1999], 1999u32);
    }

    // -----------------------------------------------------------------------
    // 19. VecDeque<u32> with 1000 elements — order preserved
    // -----------------------------------------------------------------------
    #[test]
    fn test_vecdeque_u32_1000_roundtrip() {
        // Build deque with elements pushed to both ends to create a non-trivial
        // internal ring-buffer layout before encoding
        let mut original: VecDeque<u32> = VecDeque::with_capacity(1000);
        for i in 0u32..500 {
            original.push_back(500 + i); // 500..999
        }
        for i in (0u32..500).rev() {
            original.push_front(i); // 0..499 prepended in reverse
        }
        assert_eq!(original.len(), 1000);
        // After the above the deque must be 0..999 in order
        let as_vec: Vec<u32> = original.iter().copied().collect();
        assert_eq!(as_vec, (0u32..1000).collect::<Vec<_>>());

        let bytes = encode_to_vec(&original).expect("encode VecDeque<u32> 1000 elements");
        let (decoded, consumed): (VecDeque<u32>, _) =
            decode_from_slice(&bytes).expect("decode VecDeque<u32> 1000 elements");

        assert_eq!(decoded.len(), 1000);
        assert_eq!(original, decoded);
        assert_eq!(consumed, bytes.len());
    }

    // -----------------------------------------------------------------------
    // 20. LinkedList<String> with 100 elements — insertion order preserved
    // -----------------------------------------------------------------------
    #[test]
    fn test_linkedlist_string_100_roundtrip() {
        let original: LinkedList<String> = (0u32..100).map(|i| format!("node_{:03}", i)).collect();
        assert_eq!(original.len(), 100);

        let bytes = encode_to_vec(&original).expect("encode LinkedList<String> 100 elements");
        let (decoded, consumed): (LinkedList<String>, _) =
            decode_from_slice(&bytes).expect("decode LinkedList<String> 100 elements");

        assert_eq!(decoded.len(), 100);
        assert_eq!(original, decoded);
        assert_eq!(consumed, bytes.len());

        // Verify order by iterating both simultaneously
        for (orig, dec) in original.iter().zip(decoded.iter()) {
            assert_eq!(orig, dec);
        }
    }

    // -----------------------------------------------------------------------
    // 21. Vec<f64> with 10000 elements — bit-exact values after roundtrip
    // -----------------------------------------------------------------------
    #[test]
    fn test_vec_f64_10000_roundtrip() {
        use std::f64::consts::PI;

        let original: Vec<f64> = (0u32..10000)
            .map(|i| {
                let t = f64::from(i) / 1000.0;
                t * PI + (t * t).ln_1p()
            })
            .collect();
        assert_eq!(original.len(), 10000);

        let bytes = encode_to_vec(&original).expect("encode Vec<f64> 10000 elements");
        let (decoded, consumed): (Vec<f64>, _) =
            decode_from_slice(&bytes).expect("decode Vec<f64> 10000 elements");

        assert_eq!(decoded.len(), 10000);
        assert_eq!(consumed, bytes.len());

        // Every f64 must be bit-exact (no lossy conversion)
        for (idx, (orig, dec)) in original.iter().zip(decoded.iter()).enumerate() {
            assert_eq!(
                orig.to_bits(),
                dec.to_bits(),
                "f64 at index {idx} must be bit-exact"
            );
        }
    }

    // -----------------------------------------------------------------------
    // 22. Decode partial (truncated) data triggers an error
    //     Encoding a large Vec then slicing off the last bytes must fail
    // -----------------------------------------------------------------------
    #[test]
    fn test_decode_partial_data_triggers_error() {
        // Encode a reasonably sized collection
        let original: Vec<u32> = (0u32..300).collect();
        let bytes = encode_to_vec(&original).expect("encode Vec<u32> 300 for truncation test");

        // Confirm the complete roundtrip works first
        let (decoded_full, _): (Vec<u32>, _) =
            decode_from_slice(&bytes).expect("full decode must succeed");
        assert_eq!(decoded_full, original);

        // Now truncate to half the encoded bytes — must fail
        let truncated = &bytes[..bytes.len() / 2];
        let result: oxicode::Result<(Vec<u32>, usize)> = decode_from_slice(truncated);
        assert!(
            result.is_err(),
            "decoding truncated data must return Err, but got Ok"
        );

        // Truncate to just the length prefix (first few bytes) — must also fail
        let prefix_only = &bytes[..3];
        let result2: oxicode::Result<(Vec<u32>, usize)> = decode_from_slice(prefix_only);
        assert!(
            result2.is_err(),
            "decoding only the length prefix must return Err"
        );

        // Verify the standard configuration also produces errors with a custom config
        let result3: oxicode::Result<(Vec<u32>, usize)> =
            oxicode::decode_from_slice_with_config(truncated, config::standard());
        assert!(
            result3.is_err(),
            "decode_from_slice_with_config on truncated data must return Err"
        );
    }
}
