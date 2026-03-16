//! Advanced comprehensive tests for collection type encoding in OxiCode.
//!
//! Covers nested collections, large collections, ordering guarantees,
//! edge cases with empty containers, and the lazy decode iterator API.

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

use oxicode::{decode_from_slice, encode_to_vec};

mod collections_advanced_tests {
    use super::*;

    // -----------------------------------------------------------------------
    // 1. Empty HashMap roundtrip
    // -----------------------------------------------------------------------
    #[test]
    fn test_empty_hashmap_roundtrip() {
        let original: HashMap<String, u32> = HashMap::new();
        let bytes = encode_to_vec(&original).expect("Failed to encode empty HashMap");
        let (decoded, consumed): (HashMap<String, u32>, _) =
            decode_from_slice(&bytes).expect("Failed to decode empty HashMap");
        assert_eq!(original, decoded);
        assert_eq!(consumed, bytes.len());
    }

    // -----------------------------------------------------------------------
    // 2. HashMap<String, Vec<u32>> roundtrip
    // -----------------------------------------------------------------------
    #[test]
    fn test_hashmap_string_to_vec_u32_roundtrip() {
        let mut original: HashMap<String, Vec<u32>> = HashMap::new();
        original.insert("fibonacci".to_string(), vec![1, 1, 2, 3, 5, 8, 13, 21]);
        original.insert("primes".to_string(), vec![2, 3, 5, 7, 11, 13, 17, 19]);
        original.insert("squares".to_string(), vec![1, 4, 9, 16, 25, 36, 49]);
        original.insert("empty".to_string(), vec![]);

        let bytes = encode_to_vec(&original).expect("Failed to encode HashMap<String, Vec<u32>>");
        let (decoded, consumed): (HashMap<String, Vec<u32>>, _) =
            decode_from_slice(&bytes).expect("Failed to decode HashMap<String, Vec<u32>>");
        assert_eq!(original, decoded);
        assert_eq!(consumed, bytes.len());
    }

    // -----------------------------------------------------------------------
    // 3. HashMap<u32, HashMap<String, u8>> nested roundtrip
    // -----------------------------------------------------------------------
    #[test]
    fn test_nested_hashmap_roundtrip() {
        let mut inner_a: HashMap<String, u8> = HashMap::new();
        inner_a.insert("alpha".to_string(), 1);
        inner_a.insert("beta".to_string(), 2);

        let mut inner_b: HashMap<String, u8> = HashMap::new();
        inner_b.insert("gamma".to_string(), 3);
        inner_b.insert("delta".to_string(), 4);
        inner_b.insert("epsilon".to_string(), 5);

        let mut original: HashMap<u32, HashMap<String, u8>> = HashMap::new();
        original.insert(100, inner_a);
        original.insert(200, inner_b);
        original.insert(300, HashMap::new());

        let bytes =
            encode_to_vec(&original).expect("Failed to encode HashMap<u32, HashMap<String, u8>>");
        let (decoded, consumed): (HashMap<u32, HashMap<String, u8>>, _) =
            decode_from_slice(&bytes).expect("Failed to decode HashMap<u32, HashMap<String, u8>>");
        assert_eq!(original, decoded);
        assert_eq!(consumed, bytes.len());
    }

    // -----------------------------------------------------------------------
    // 4. BTreeMap with 1000 entries roundtrip
    // -----------------------------------------------------------------------
    #[test]
    fn test_btreemap_1000_entries_roundtrip() {
        let original: BTreeMap<u32, String> = (0u32..1000)
            .map(|i| (i, format!("value_{:04}", i)))
            .collect();

        let bytes = encode_to_vec(&original).expect("Failed to encode BTreeMap with 1000 entries");
        let (decoded, consumed): (BTreeMap<u32, String>, _) =
            decode_from_slice(&bytes).expect("Failed to decode BTreeMap with 1000 entries");
        assert_eq!(original.len(), decoded.len());
        assert_eq!(original, decoded);
        assert_eq!(consumed, bytes.len());
    }

    // -----------------------------------------------------------------------
    // 5. BTreeSet<String> roundtrip
    // -----------------------------------------------------------------------
    #[test]
    fn test_btreeset_string_roundtrip() {
        let original: BTreeSet<String> = vec![
            "zebra".to_string(),
            "apple".to_string(),
            "mango".to_string(),
            "banana".to_string(),
            "cherry".to_string(),
        ]
        .into_iter()
        .collect();

        let bytes = encode_to_vec(&original).expect("Failed to encode BTreeSet<String>");
        let (decoded, consumed): (BTreeSet<String>, _) =
            decode_from_slice(&bytes).expect("Failed to decode BTreeSet<String>");
        assert_eq!(original, decoded);
        assert_eq!(consumed, bytes.len());
    }

    // -----------------------------------------------------------------------
    // 6. HashSet<u32> roundtrip
    // -----------------------------------------------------------------------
    #[test]
    fn test_hashset_u32_roundtrip() {
        let original: HashSet<u32> = vec![42, 7, 1337, 0, 99999, 1, 2, 3].into_iter().collect();

        let bytes = encode_to_vec(&original).expect("Failed to encode HashSet<u32>");
        let (decoded, consumed): (HashSet<u32>, _) =
            decode_from_slice(&bytes).expect("Failed to decode HashSet<u32>");
        assert_eq!(original, decoded);
        assert_eq!(consumed, bytes.len());
    }

    // -----------------------------------------------------------------------
    // 7. VecDeque<String> roundtrip
    // -----------------------------------------------------------------------
    #[test]
    fn test_vecdeque_string_roundtrip() {
        let mut original: VecDeque<String> = VecDeque::new();
        original.push_back("first".to_string());
        original.push_back("second".to_string());
        original.push_front("zeroth".to_string());
        original.push_back("third".to_string());
        original.push_front("minus_one".to_string());

        let bytes = encode_to_vec(&original).expect("Failed to encode VecDeque<String>");
        let (decoded, consumed): (VecDeque<String>, _) =
            decode_from_slice(&bytes).expect("Failed to decode VecDeque<String>");
        assert_eq!(original, decoded);
        assert_eq!(consumed, bytes.len());
    }

    // -----------------------------------------------------------------------
    // 8. VecDeque<Vec<u8>> roundtrip
    // -----------------------------------------------------------------------
    #[test]
    fn test_vecdeque_vec_u8_roundtrip() {
        let mut original: VecDeque<Vec<u8>> = VecDeque::new();
        original.push_back(vec![0x00, 0xFF, 0x42]);
        original.push_back(vec![]);
        original.push_back((0u8..=127).collect());
        original.push_front(vec![0xDE, 0xAD, 0xBE, 0xEF]);

        let bytes = encode_to_vec(&original).expect("Failed to encode VecDeque<Vec<u8>>");
        let (decoded, consumed): (VecDeque<Vec<u8>>, _) =
            decode_from_slice(&bytes).expect("Failed to decode VecDeque<Vec<u8>>");
        assert_eq!(original, decoded);
        assert_eq!(consumed, bytes.len());
    }

    // -----------------------------------------------------------------------
    // 9. LinkedList<u32> roundtrip
    // -----------------------------------------------------------------------
    #[test]
    fn test_linkedlist_u32_roundtrip() {
        let original: LinkedList<u32> = vec![10, 20, 30, 40, 50, 100, 200, 300]
            .into_iter()
            .collect();

        let bytes = encode_to_vec(&original).expect("Failed to encode LinkedList<u32>");
        let (decoded, consumed): (LinkedList<u32>, _) =
            decode_from_slice(&bytes).expect("Failed to decode LinkedList<u32>");
        assert_eq!(original, decoded);
        assert_eq!(consumed, bytes.len());
    }

    // -----------------------------------------------------------------------
    // 10. LinkedList<String> roundtrip
    // -----------------------------------------------------------------------
    #[test]
    fn test_linkedlist_string_roundtrip() {
        let original: LinkedList<String> = vec![
            "one".to_string(),
            "two".to_string(),
            "three".to_string(),
            "four".to_string(),
            "five".to_string(),
        ]
        .into_iter()
        .collect();

        let bytes = encode_to_vec(&original).expect("Failed to encode LinkedList<String>");
        let (decoded, consumed): (LinkedList<String>, _) =
            decode_from_slice(&bytes).expect("Failed to decode LinkedList<String>");
        assert_eq!(original, decoded);
        assert_eq!(consumed, bytes.len());
    }

    // -----------------------------------------------------------------------
    // 11. Vec<BTreeMap<String, u64>> roundtrip
    // -----------------------------------------------------------------------
    #[test]
    fn test_vec_of_btreemaps_roundtrip() {
        let map1: BTreeMap<String, u64> =
            [("key_a".to_string(), 100u64), ("key_b".to_string(), 200u64)]
                .into_iter()
                .collect();

        let map2: BTreeMap<String, u64> = [("key_c".to_string(), 300u64)].into_iter().collect();

        let map3: BTreeMap<String, u64> = BTreeMap::new();

        let original: Vec<BTreeMap<String, u64>> = vec![map1, map2, map3];

        let bytes = encode_to_vec(&original).expect("Failed to encode Vec<BTreeMap<String, u64>>");
        let (decoded, consumed): (Vec<BTreeMap<String, u64>>, _) =
            decode_from_slice(&bytes).expect("Failed to decode Vec<BTreeMap<String, u64>>");
        assert_eq!(original, decoded);
        assert_eq!(consumed, bytes.len());
    }

    // -----------------------------------------------------------------------
    // 12. Vec<HashSet<u32>> roundtrip
    // -----------------------------------------------------------------------
    #[test]
    fn test_vec_of_hashsets_roundtrip() {
        let set1: HashSet<u32> = vec![1, 2, 3].into_iter().collect();
        let set2: HashSet<u32> = vec![10, 20, 30, 40].into_iter().collect();
        let set3: HashSet<u32> = HashSet::new();
        let set4: HashSet<u32> = vec![999].into_iter().collect();

        let original: Vec<HashSet<u32>> = vec![set1, set2, set3, set4];

        let bytes = encode_to_vec(&original).expect("Failed to encode Vec<HashSet<u32>>");
        let (decoded, consumed): (Vec<HashSet<u32>>, _) =
            decode_from_slice(&bytes).expect("Failed to decode Vec<HashSet<u32>>");
        assert_eq!(original, decoded);
        assert_eq!(consumed, bytes.len());
    }

    // -----------------------------------------------------------------------
    // 13. Vec<VecDeque<i32>> roundtrip
    // -----------------------------------------------------------------------
    #[test]
    fn test_vec_of_vecdeques_roundtrip() {
        let mut deq1: VecDeque<i32> = VecDeque::new();
        deq1.push_back(-1);
        deq1.push_back(-2);
        deq1.push_front(-3);

        let deq2: VecDeque<i32> = vec![0, 1, 2, 3].into_iter().collect();
        let deq3: VecDeque<i32> = VecDeque::new();

        let original: Vec<VecDeque<i32>> = vec![deq1, deq2, deq3];

        let bytes = encode_to_vec(&original).expect("Failed to encode Vec<VecDeque<i32>>");
        let (decoded, consumed): (Vec<VecDeque<i32>>, _) =
            decode_from_slice(&bytes).expect("Failed to decode Vec<VecDeque<i32>>");
        assert_eq!(original, decoded);
        assert_eq!(consumed, bytes.len());
    }

    // -----------------------------------------------------------------------
    // 14. BTreeMap<u32, Vec<BTreeSet<String>>> deeply nested roundtrip
    // -----------------------------------------------------------------------
    #[test]
    fn test_deeply_nested_btreemap_roundtrip() {
        let set_a: BTreeSet<String> = vec!["alpha".to_string(), "beta".to_string()]
            .into_iter()
            .collect();
        let set_b: BTreeSet<String> = vec![
            "gamma".to_string(),
            "delta".to_string(),
            "epsilon".to_string(),
        ]
        .into_iter()
        .collect();
        let set_c: BTreeSet<String> = BTreeSet::new();

        let mut original: BTreeMap<u32, Vec<BTreeSet<String>>> = BTreeMap::new();
        original.insert(1, vec![set_a.clone(), set_b.clone()]);
        original.insert(2, vec![set_c.clone()]);
        original.insert(3, vec![set_a, set_b, set_c]);
        original.insert(4, vec![]);

        let bytes = encode_to_vec(&original)
            .expect("Failed to encode BTreeMap<u32, Vec<BTreeSet<String>>>");
        let (decoded, consumed): (BTreeMap<u32, Vec<BTreeSet<String>>>, _) =
            decode_from_slice(&bytes)
                .expect("Failed to decode BTreeMap<u32, Vec<BTreeSet<String>>>");
        assert_eq!(original, decoded);
        assert_eq!(consumed, bytes.len());
    }

    // -----------------------------------------------------------------------
    // 15. Collection size: HashMap with 500 entries encodes/decodes correctly
    // -----------------------------------------------------------------------
    #[test]
    fn test_hashmap_500_entries_size_correct() {
        let original: HashMap<u32, u64> = (0u32..500)
            .map(|i| (i, u64::from(i) * u64::from(i)))
            .collect();

        let bytes = encode_to_vec(&original).expect("Failed to encode HashMap with 500 entries");
        let (decoded, consumed): (HashMap<u32, u64>, _) =
            decode_from_slice(&bytes).expect("Failed to decode HashMap with 500 entries");
        assert_eq!(original.len(), 500);
        assert_eq!(decoded.len(), 500);
        assert_eq!(original, decoded);
        assert_eq!(consumed, bytes.len());
        // Verify every entry survived the roundtrip
        for i in 0u32..500 {
            let expected_val = u64::from(i) * u64::from(i);
            assert_eq!(
                decoded.get(&i).copied(),
                Some(expected_val),
                "Entry {i} missing or wrong"
            );
        }
    }

    // -----------------------------------------------------------------------
    // 16. Collection size: Vec<u8> with 65536 elements roundtrip
    // -----------------------------------------------------------------------
    #[test]
    fn test_large_vec_u8_65536_roundtrip() {
        let original: Vec<u8> = (0u32..65536).map(|i| (i % 256) as u8).collect();
        assert_eq!(original.len(), 65536);

        let bytes = encode_to_vec(&original).expect("Failed to encode Vec<u8> with 65536 elements");
        let (decoded, consumed): (Vec<u8>, _) =
            decode_from_slice(&bytes).expect("Failed to decode Vec<u8> with 65536 elements");
        assert_eq!(decoded.len(), 65536);
        assert_eq!(original, decoded);
        assert_eq!(consumed, bytes.len());
    }

    // -----------------------------------------------------------------------
    // 17. Collection ordering: BTreeMap keys decoded in sorted order
    // -----------------------------------------------------------------------
    #[test]
    fn test_btreemap_keys_sorted_after_decode() {
        // Insert in non-sorted order
        let mut original: BTreeMap<u32, String> = BTreeMap::new();
        for &k in &[50u32, 10, 90, 30, 70, 20, 80, 40, 60] {
            original.insert(k, format!("val_{}", k));
        }

        let bytes = encode_to_vec(&original).expect("Failed to encode BTreeMap for ordering test");
        let (decoded, _): (BTreeMap<u32, String>, _) =
            decode_from_slice(&bytes).expect("Failed to decode BTreeMap for ordering test");

        // BTreeMap iterates in ascending key order
        let keys: Vec<u32> = decoded.keys().copied().collect();
        let mut sorted_keys = keys.clone();
        sorted_keys.sort_unstable();
        assert_eq!(keys, sorted_keys, "BTreeMap keys must be in sorted order");
        assert_eq!(keys, vec![10, 20, 30, 40, 50, 60, 70, 80, 90]);
    }

    // -----------------------------------------------------------------------
    // 18. Collection ordering: BTreeSet elements decoded in sorted order
    // -----------------------------------------------------------------------
    #[test]
    fn test_btreeset_elements_sorted_after_decode() {
        // Create with elements in non-sorted insertion order
        let original: BTreeSet<i32> = vec![100, -5, 42, 0, -100, 7, 1000, -1]
            .into_iter()
            .collect();

        let bytes = encode_to_vec(&original).expect("Failed to encode BTreeSet for ordering test");
        let (decoded, _): (BTreeSet<i32>, _) =
            decode_from_slice(&bytes).expect("Failed to decode BTreeSet for ordering test");

        let elements: Vec<i32> = decoded.iter().copied().collect();
        let mut sorted = elements.clone();
        sorted.sort_unstable();
        assert_eq!(elements, sorted, "BTreeSet must iterate in sorted order");
        assert_eq!(elements, vec![-100, -5, -1, 0, 7, 42, 100, 1000]);
    }

    // -----------------------------------------------------------------------
    // 19. Empty BTreeMap roundtrip
    // -----------------------------------------------------------------------
    #[test]
    fn test_empty_btreemap_roundtrip() {
        let original: BTreeMap<String, Vec<u64>> = BTreeMap::new();
        let bytes = encode_to_vec(&original).expect("Failed to encode empty BTreeMap");
        let (decoded, consumed): (BTreeMap<String, Vec<u64>>, _) =
            decode_from_slice(&bytes).expect("Failed to decode empty BTreeMap");
        assert!(decoded.is_empty());
        assert_eq!(original, decoded);
        assert_eq!(consumed, bytes.len());
    }

    // -----------------------------------------------------------------------
    // 20. Empty Vec<HashMap<String, u32>> roundtrip
    // -----------------------------------------------------------------------
    #[test]
    fn test_empty_vec_of_hashmaps_roundtrip() {
        let original: Vec<HashMap<String, u32>> = Vec::new();
        let bytes =
            encode_to_vec(&original).expect("Failed to encode empty Vec<HashMap<String, u32>>");
        let (decoded, consumed): (Vec<HashMap<String, u32>>, _) =
            decode_from_slice(&bytes).expect("Failed to decode empty Vec<HashMap<String, u32>>");
        assert!(decoded.is_empty());
        assert_eq!(original, decoded);
        assert_eq!(consumed, bytes.len());
    }

    // -----------------------------------------------------------------------
    // 21. HashMap<String, Option<Vec<u32>>> with Some and None values
    // -----------------------------------------------------------------------
    #[test]
    fn test_hashmap_option_vec_roundtrip() {
        let mut original: HashMap<String, Option<Vec<u32>>> = HashMap::new();
        original.insert("present".to_string(), Some(vec![1, 2, 3, 4, 5]));
        original.insert("absent".to_string(), None);
        original.insert("empty_vec".to_string(), Some(vec![]));
        original.insert("single".to_string(), Some(vec![42]));
        original.insert("also_absent".to_string(), None);

        let bytes =
            encode_to_vec(&original).expect("Failed to encode HashMap<String, Option<Vec<u32>>>");
        let (decoded, consumed): (HashMap<String, Option<Vec<u32>>>, _) =
            decode_from_slice(&bytes).expect("Failed to decode HashMap<String, Option<Vec<u32>>>");
        assert_eq!(original, decoded);
        assert_eq!(consumed, bytes.len());

        // Spot-check specific entries
        assert_eq!(
            decoded.get("present"),
            Some(&Some(vec![1, 2, 3, 4, 5])),
            "present key should hold Some([1,2,3,4,5])"
        );
        assert_eq!(
            decoded.get("absent"),
            Some(&None),
            "absent key should hold None"
        );
        assert_eq!(
            decoded.get("empty_vec"),
            Some(&Some(vec![])),
            "empty_vec key should hold Some([])"
        );
    }

    // -----------------------------------------------------------------------
    // 22. Encode Vec<String> then decode via lazy iterator
    // -----------------------------------------------------------------------
    #[test]
    fn test_vec_string_decode_via_iter() {
        let original: Vec<String> = vec![
            "alpha".to_string(),
            "beta".to_string(),
            "gamma".to_string(),
            "delta".to_string(),
            "epsilon".to_string(),
        ];

        let bytes = encode_to_vec(&original).expect("Failed to encode Vec<String>");

        // Use the lazy DecodeIter API to read items one at a time
        let iter = oxicode::decode_iter_from_slice::<String>(&bytes)
            .expect("Failed to initialise DecodeIter for Vec<String>");

        let decoded: Vec<String> = iter
            .map(|r| r.expect("Failed to decode String item from iterator"))
            .collect();

        assert_eq!(original, decoded);
        assert_eq!(decoded.len(), 5);
        assert_eq!(decoded[0], "alpha");
        assert_eq!(decoded[4], "epsilon");
    }
}
