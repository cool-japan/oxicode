//! Tests for generic collection patterns in OxiCode.
//!
//! Covers smart pointer wrappers (Box, Rc, Arc), deeply nested maps,
//! mixed Option/Result inside collections, large-scale collection roundtrips,
//! and encoded_size comparisons for empty vs populated nested containers.

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
use std::rc::Rc;
use std::sync::Arc;

use oxicode::{config, decode_from_slice, encode_to_vec, encoded_size};

mod generic_collections_tests {
    use super::*;

    // -----------------------------------------------------------------------
    // 1. Vec<Box<u32>> roundtrip
    // -----------------------------------------------------------------------
    #[test]
    fn test_vec_box_u32_roundtrip() {
        let original: Vec<Box<u32>> = vec![
            Box::new(0u32),
            Box::new(1u32),
            Box::new(u32::MAX),
            Box::new(42u32),
            Box::new(1_000_000u32),
        ];
        let bytes = encode_to_vec(&original).expect("Failed to encode Vec<Box<u32>>");
        let (decoded, consumed): (Vec<Box<u32>>, _) =
            decode_from_slice(&bytes).expect("Failed to decode Vec<Box<u32>>");
        assert_eq!(original, decoded);
        assert_eq!(consumed, bytes.len());
    }

    // -----------------------------------------------------------------------
    // 2. Vec<Rc<String>> roundtrip
    // -----------------------------------------------------------------------
    #[test]
    fn test_vec_rc_string_roundtrip() {
        let original: Vec<Rc<String>> = vec![
            Rc::new("hello".to_string()),
            Rc::new("world".to_string()),
            Rc::new(String::new()),
            Rc::new("oxicode".to_string()),
        ];
        let bytes = encode_to_vec(&original).expect("Failed to encode Vec<Rc<String>>");
        let (decoded, consumed): (Vec<Rc<String>>, _) =
            decode_from_slice(&bytes).expect("Failed to decode Vec<Rc<String>>");
        // Compare inner values
        assert_eq!(original.len(), decoded.len());
        for (o, d) in original.iter().zip(decoded.iter()) {
            assert_eq!(o.as_ref(), d.as_ref());
        }
        assert_eq!(consumed, bytes.len());
    }

    // -----------------------------------------------------------------------
    // 3. Vec<Arc<Vec<u8>>> roundtrip
    // -----------------------------------------------------------------------
    #[test]
    fn test_vec_arc_vec_u8_roundtrip() {
        let original: Vec<Arc<Vec<u8>>> = vec![
            Arc::new(vec![0x01, 0x02, 0x03]),
            Arc::new(vec![]),
            Arc::new((0u8..128).collect()),
            Arc::new(vec![0xFF, 0xFE, 0xFD]),
        ];
        let bytes = encode_to_vec(&original).expect("Failed to encode Vec<Arc<Vec<u8>>>");
        let (decoded, consumed): (Vec<Arc<Vec<u8>>>, _) =
            decode_from_slice(&bytes).expect("Failed to decode Vec<Arc<Vec<u8>>>");
        assert_eq!(original.len(), decoded.len());
        for (o, d) in original.iter().zip(decoded.iter()) {
            assert_eq!(o.as_ref(), d.as_ref());
        }
        assert_eq!(consumed, bytes.len());
    }

    // -----------------------------------------------------------------------
    // 4. HashMap<String, Box<Vec<u32>>> roundtrip
    // -----------------------------------------------------------------------
    #[test]
    fn test_hashmap_string_box_vec_u32_roundtrip() {
        let mut original: HashMap<String, Box<Vec<u32>>> = HashMap::new();
        original.insert("evens".to_string(), Box::new(vec![2, 4, 6, 8, 10]));
        original.insert("odds".to_string(), Box::new(vec![1, 3, 5, 7, 9]));
        original.insert("empty".to_string(), Box::new(vec![]));
        original.insert("single".to_string(), Box::new(vec![999]));

        let bytes =
            encode_to_vec(&original).expect("Failed to encode HashMap<String, Box<Vec<u32>>>");
        let (decoded, consumed): (HashMap<String, Box<Vec<u32>>>, _) =
            decode_from_slice(&bytes).expect("Failed to decode HashMap<String, Box<Vec<u32>>>");
        assert_eq!(original, decoded);
        assert_eq!(consumed, bytes.len());
    }

    // -----------------------------------------------------------------------
    // 5. BTreeMap<u32, Arc<String>> roundtrip
    // -----------------------------------------------------------------------
    #[test]
    fn test_btreemap_u32_arc_string_roundtrip() {
        let original: BTreeMap<u32, Arc<String>> = [
            (1u32, Arc::new("one".to_string())),
            (2u32, Arc::new("two".to_string())),
            (100u32, Arc::new("hundred".to_string())),
            (0u32, Arc::new(String::new())),
            (u32::MAX, Arc::new("max".to_string())),
        ]
        .into_iter()
        .collect();

        let bytes = encode_to_vec(&original).expect("Failed to encode BTreeMap<u32, Arc<String>>");
        let (decoded, consumed): (BTreeMap<u32, Arc<String>>, _) =
            decode_from_slice(&bytes).expect("Failed to decode BTreeMap<u32, Arc<String>>");
        assert_eq!(original.len(), decoded.len());
        for (k, v) in &original {
            let dv = decoded.get(k).expect("key missing after decode");
            assert_eq!(v.as_ref(), dv.as_ref());
        }
        assert_eq!(consumed, bytes.len());
    }

    // -----------------------------------------------------------------------
    // 6. Vec<(u32, Vec<String>)> roundtrip
    // -----------------------------------------------------------------------
    #[test]
    fn test_vec_tuple_u32_vec_string_roundtrip() {
        let original: Vec<(u32, Vec<String>)> = vec![
            (1u32, vec!["a".to_string(), "b".to_string()]),
            (2u32, vec![]),
            (
                3u32,
                vec!["alpha".to_string(), "beta".to_string(), "gamma".to_string()],
            ),
            (0u32, vec!["only".to_string()]),
        ];
        let bytes = encode_to_vec(&original).expect("Failed to encode Vec<(u32, Vec<String>)>");
        let (decoded, consumed): (Vec<(u32, Vec<String>)>, _) =
            decode_from_slice(&bytes).expect("Failed to decode Vec<(u32, Vec<String>)>");
        assert_eq!(original, decoded);
        assert_eq!(consumed, bytes.len());
    }

    // -----------------------------------------------------------------------
    // 7. Vec<HashMap<String, u32>> (vector of maps) roundtrip
    // -----------------------------------------------------------------------
    #[test]
    fn test_vec_of_hashmaps_string_u32_roundtrip() {
        let map_a: HashMap<String, u32> = [("x".to_string(), 10u32), ("y".to_string(), 20u32)]
            .into_iter()
            .collect();
        let map_b: HashMap<String, u32> = HashMap::new();
        let map_c: HashMap<String, u32> = [("z".to_string(), 999u32)].into_iter().collect();

        let original: Vec<HashMap<String, u32>> = vec![map_a, map_b, map_c];
        let bytes = encode_to_vec(&original).expect("Failed to encode Vec<HashMap<String, u32>>");
        let (decoded, consumed): (Vec<HashMap<String, u32>>, _) =
            decode_from_slice(&bytes).expect("Failed to decode Vec<HashMap<String, u32>>");
        assert_eq!(original, decoded);
        assert_eq!(consumed, bytes.len());
    }

    // -----------------------------------------------------------------------
    // 8. HashMap<String, Vec<HashMap<String, u32>>> deeply nested roundtrip
    // -----------------------------------------------------------------------
    #[test]
    fn test_deeply_nested_hashmap_roundtrip() {
        let inner1: HashMap<String, u32> = [("p".to_string(), 1u32), ("q".to_string(), 2u32)]
            .into_iter()
            .collect();
        let inner2: HashMap<String, u32> = [("r".to_string(), 3u32)].into_iter().collect();

        let mut original: HashMap<String, Vec<HashMap<String, u32>>> = HashMap::new();
        original.insert("group_a".to_string(), vec![inner1.clone(), inner2.clone()]);
        original.insert("group_b".to_string(), vec![inner2.clone()]);
        original.insert("group_empty".to_string(), vec![]);
        original.insert("group_empty_inner".to_string(), vec![HashMap::new()]);

        let bytes = encode_to_vec(&original)
            .expect("Failed to encode HashMap<String, Vec<HashMap<String, u32>>>");
        let (decoded, consumed): (HashMap<String, Vec<HashMap<String, u32>>>, _) =
            decode_from_slice(&bytes)
                .expect("Failed to decode HashMap<String, Vec<HashMap<String, u32>>>");
        assert_eq!(original, decoded);
        assert_eq!(consumed, bytes.len());
    }

    // -----------------------------------------------------------------------
    // 9. BTreeMap<String, BTreeMap<u32, Vec<u8>>> roundtrip
    // -----------------------------------------------------------------------
    #[test]
    fn test_btreemap_string_btreemap_u32_vec_u8_roundtrip() {
        let mut inner_a: BTreeMap<u32, Vec<u8>> = BTreeMap::new();
        inner_a.insert(1u32, vec![0x01, 0x02]);
        inner_a.insert(2u32, vec![]);
        inner_a.insert(3u32, vec![0xFF]);

        let inner_b: BTreeMap<u32, Vec<u8>> = BTreeMap::new();

        let mut inner_c: BTreeMap<u32, Vec<u8>> = BTreeMap::new();
        inner_c.insert(100u32, (0u8..=255).collect());

        let mut original: BTreeMap<String, BTreeMap<u32, Vec<u8>>> = BTreeMap::new();
        original.insert("section_a".to_string(), inner_a);
        original.insert("section_b".to_string(), inner_b);
        original.insert("section_c".to_string(), inner_c);

        let bytes = encode_to_vec(&original)
            .expect("Failed to encode BTreeMap<String, BTreeMap<u32, Vec<u8>>>");
        let (decoded, consumed): (BTreeMap<String, BTreeMap<u32, Vec<u8>>>, _) =
            decode_from_slice(&bytes)
                .expect("Failed to decode BTreeMap<String, BTreeMap<u32, Vec<u8>>>");
        assert_eq!(original, decoded);
        assert_eq!(consumed, bytes.len());
    }

    // -----------------------------------------------------------------------
    // 10. Vec<Option<Vec<u32>>> roundtrip
    // -----------------------------------------------------------------------
    #[test]
    fn test_vec_option_vec_u32_roundtrip() {
        let original: Vec<Option<Vec<u32>>> = vec![
            Some(vec![1u32, 2, 3]),
            None,
            Some(vec![]),
            Some(vec![u32::MAX]),
            None,
            Some(vec![10, 20, 30, 40, 50]),
        ];
        let bytes = encode_to_vec(&original).expect("Failed to encode Vec<Option<Vec<u32>>>");
        let (decoded, consumed): (Vec<Option<Vec<u32>>>, _) =
            decode_from_slice(&bytes).expect("Failed to decode Vec<Option<Vec<u32>>>");
        assert_eq!(original, decoded);
        assert_eq!(consumed, bytes.len());
        assert_eq!(decoded[0], Some(vec![1, 2, 3]));
        assert_eq!(decoded[1], None);
        assert_eq!(decoded[2], Some(vec![]));
    }

    // -----------------------------------------------------------------------
    // 11. Vec<Result<u32, String>> roundtrip
    // -----------------------------------------------------------------------
    #[test]
    fn test_vec_result_u32_string_roundtrip() {
        let original: Vec<Result<u32, String>> = vec![
            Ok(42u32),
            Err("something went wrong".to_string()),
            Ok(0u32),
            Err(String::new()),
            Ok(u32::MAX),
        ];
        let bytes = encode_to_vec(&original).expect("Failed to encode Vec<Result<u32, String>>");
        let (decoded, consumed): (Vec<Result<u32, String>>, _) =
            decode_from_slice(&bytes).expect("Failed to decode Vec<Result<u32, String>>");
        assert_eq!(original.len(), decoded.len());
        for (o, d) in original.iter().zip(decoded.iter()) {
            match (o, d) {
                (Ok(a), Ok(b)) => assert_eq!(a, b),
                (Err(a), Err(b)) => assert_eq!(a, b),
                _ => panic!("Result variant mismatch after roundtrip"),
            }
        }
        assert_eq!(consumed, bytes.len());
    }

    // -----------------------------------------------------------------------
    // 12. HashMap<String, Option<Vec<u32>>> roundtrip (distinct from test 21 in
    //     collections_advanced_test.rs by exercising more edge cases)
    // -----------------------------------------------------------------------
    #[test]
    fn test_hashmap_string_option_vec_u32_all_variants_roundtrip() {
        let mut original: HashMap<String, Option<Vec<u32>>> = HashMap::new();
        original.insert(
            "full".to_string(),
            Some(vec![1u32, 2, 3, 4, 5, 6, 7, 8, 9, 10]),
        );
        original.insert("none_a".to_string(), None);
        original.insert("none_b".to_string(), None);
        original.insert("empty_inner".to_string(), Some(vec![]));
        original.insert("large".to_string(), Some((0u32..200).collect()));

        let bytes =
            encode_to_vec(&original).expect("Failed to encode HashMap<String, Option<Vec<u32>>>");
        let (decoded, consumed): (HashMap<String, Option<Vec<u32>>>, _) =
            decode_from_slice(&bytes).expect("Failed to decode HashMap<String, Option<Vec<u32>>>");
        assert_eq!(original, decoded);
        assert_eq!(consumed, bytes.len());
        assert_eq!(decoded.get("none_a"), Some(&None));
        assert_eq!(decoded.get("empty_inner"), Some(&Some(vec![])));
        let large = decoded
            .get("large")
            .expect("large key missing")
            .as_ref()
            .expect("large is Some");
        assert_eq!(large.len(), 200);
    }

    // -----------------------------------------------------------------------
    // 13. Vec<(String, Option<u32>, Vec<u8>)> roundtrip
    // -----------------------------------------------------------------------
    #[test]
    fn test_vec_triple_string_option_u32_vec_u8_roundtrip() {
        let original: Vec<(String, Option<u32>, Vec<u8>)> = vec![
            ("alpha".to_string(), Some(1u32), vec![0xAA, 0xBB]),
            ("beta".to_string(), None, vec![]),
            ("gamma".to_string(), Some(u32::MAX), vec![0x01]),
            (String::new(), Some(0u32), (0u8..64).collect()),
            ("delta".to_string(), None, vec![0xFF; 16]),
        ];
        let bytes =
            encode_to_vec(&original).expect("Failed to encode Vec<(String, Option<u32>, Vec<u8>)>");
        #[allow(clippy::type_complexity)]
        let (decoded, consumed): (Vec<(String, Option<u32>, Vec<u8>)>, _) =
            decode_from_slice(&bytes)
                .expect("Failed to decode Vec<(String, Option<u32>, Vec<u8>)>");
        assert_eq!(original, decoded);
        assert_eq!(consumed, bytes.len());
    }

    // -----------------------------------------------------------------------
    // 14. BTreeMap with tuple values (String, u64) roundtrip
    // -----------------------------------------------------------------------
    #[test]
    fn test_btreemap_with_tuple_values_roundtrip() {
        let original: BTreeMap<u32, (String, u64)> = [
            (1u32, ("first".to_string(), 100u64)),
            (2u32, ("second".to_string(), 200u64)),
            (3u32, (String::new(), 0u64)),
            (100u32, ("hundred".to_string(), u64::MAX)),
        ]
        .into_iter()
        .collect();

        let bytes =
            encode_to_vec(&original).expect("Failed to encode BTreeMap<u32, (String, u64)>");
        let (decoded, consumed): (BTreeMap<u32, (String, u64)>, _) =
            decode_from_slice(&bytes).expect("Failed to decode BTreeMap<u32, (String, u64)>");
        assert_eq!(original, decoded);
        assert_eq!(consumed, bytes.len());
        // Spot check ordering
        let keys: Vec<u32> = decoded.keys().copied().collect();
        assert_eq!(keys, vec![1, 2, 3, 100]);
    }

    // -----------------------------------------------------------------------
    // 15. Vec of 500 HashMaps roundtrip
    // -----------------------------------------------------------------------
    #[test]
    fn test_vec_of_500_hashmaps_roundtrip() {
        let original: Vec<HashMap<String, u32>> = (0u32..500)
            .map(|i| {
                let mut m = HashMap::new();
                m.insert(format!("key_{}", i), i);
                m.insert(format!("double_{}", i), i * 2);
                m
            })
            .collect();

        let bytes = encode_to_vec(&original).expect("Failed to encode Vec of 500 HashMaps");
        let (decoded, consumed): (Vec<HashMap<String, u32>>, _) =
            decode_from_slice(&bytes).expect("Failed to decode Vec of 500 HashMaps");
        assert_eq!(original.len(), 500);
        assert_eq!(decoded.len(), 500);
        assert_eq!(original, decoded);
        assert_eq!(consumed, bytes.len());
        // Spot-check a few entries
        assert_eq!(decoded[0].get("key_0"), Some(&0u32));
        assert_eq!(decoded[499].get("key_499"), Some(&499u32));
        assert_eq!(decoded[250].get("double_250"), Some(&500u32));
    }

    // -----------------------------------------------------------------------
    // 16. Nested Option: Option<Option<Vec<u8>>> all cases
    // -----------------------------------------------------------------------
    #[test]
    fn test_nested_option_option_vec_u8_all_cases() {
        let cases: Vec<Option<Option<Vec<u8>>>> = vec![
            None,
            Some(None),
            Some(Some(vec![])),
            Some(Some(vec![0x01, 0x02, 0x03])),
            Some(Some((0u8..=255).collect())),
        ];

        for original in &cases {
            let bytes = encode_to_vec(original).expect("Failed to encode Option<Option<Vec<u8>>>");
            let (decoded, consumed): (Option<Option<Vec<u8>>>, _) =
                decode_from_slice(&bytes).expect("Failed to decode Option<Option<Vec<u8>>>");
            assert_eq!(original, &decoded, "Mismatch for case: {:?}", original);
            assert_eq!(consumed, bytes.len());
        }

        // Encode all cases together as a Vec
        let bytes = encode_to_vec(&cases).expect("Failed to encode Vec<Option<Option<Vec<u8>>>>");
        let (decoded_all, consumed): (Vec<Option<Option<Vec<u8>>>>, _) =
            decode_from_slice(&bytes).expect("Failed to decode Vec<Option<Option<Vec<u8>>>>");
        assert_eq!(cases, decoded_all);
        assert_eq!(consumed, bytes.len());
    }

    // -----------------------------------------------------------------------
    // 17. Vec<BTreeMap<String, u32>> roundtrip
    //     (distinct from test 11 in collections_advanced_test.rs which uses u64)
    // -----------------------------------------------------------------------
    #[test]
    fn test_vec_btreemap_string_u32_roundtrip() {
        let maps: Vec<BTreeMap<String, u32>> = (0u32..10)
            .map(|i| {
                let mut m = BTreeMap::new();
                m.insert(format!("alpha_{}", i), i);
                m.insert(format!("beta_{}", i), i * 10);
                m.insert(format!("gamma_{}", i), i * 100);
                m
            })
            .collect();

        let bytes = encode_to_vec(&maps).expect("Failed to encode Vec<BTreeMap<String, u32>>");
        let (decoded, consumed): (Vec<BTreeMap<String, u32>>, _) =
            decode_from_slice(&bytes).expect("Failed to decode Vec<BTreeMap<String, u32>>");
        assert_eq!(maps, decoded);
        assert_eq!(consumed, bytes.len());
        // Each BTreeMap should preserve sorted key order
        for m in &decoded {
            let keys: Vec<&String> = m.keys().collect();
            let mut sorted = keys.clone();
            sorted.sort();
            assert_eq!(keys, sorted);
        }
    }

    // -----------------------------------------------------------------------
    // 18. HashSet<String> roundtrip
    //     (distinct from test 6 in collections_advanced_test.rs which uses u32)
    // -----------------------------------------------------------------------
    #[test]
    fn test_hashset_string_roundtrip() {
        let original: HashSet<String> = vec![
            "rust".to_string(),
            "oxicode".to_string(),
            "binary".to_string(),
            "serialization".to_string(),
            "encode".to_string(),
            "decode".to_string(),
        ]
        .into_iter()
        .collect();

        let bytes = encode_to_vec(&original).expect("Failed to encode HashSet<String>");
        let (decoded, consumed): (HashSet<String>, _) =
            decode_from_slice(&bytes).expect("Failed to decode HashSet<String>");
        assert_eq!(original, decoded);
        assert_eq!(original.len(), decoded.len());
        assert!(decoded.contains("rust"));
        assert!(decoded.contains("decode"));
        assert_eq!(consumed, bytes.len());
    }

    // -----------------------------------------------------------------------
    // 19. BTreeSet<String> roundtrip with ordering guarantee
    //     (distinct from test 5 in collections_advanced_test.rs)
    // -----------------------------------------------------------------------
    #[test]
    fn test_btreeset_string_ordered_roundtrip() {
        let original: BTreeSet<String> = vec![
            "zulu".to_string(),
            "alpha".to_string(),
            "mike".to_string(),
            "foxtrot".to_string(),
            "oscar".to_string(),
            "bravo".to_string(),
        ]
        .into_iter()
        .collect();

        let bytes = encode_to_vec(&original).expect("Failed to encode BTreeSet<String>");
        let (decoded, consumed): (BTreeSet<String>, _) =
            decode_from_slice(&bytes).expect("Failed to decode BTreeSet<String>");
        assert_eq!(original, decoded);
        assert_eq!(consumed, bytes.len());
        // Verify sorted iteration order
        let items: Vec<&String> = decoded.iter().collect();
        assert_eq!(items[0].as_str(), "alpha");
        assert_eq!(items[1].as_str(), "bravo");
        assert_eq!(items[5].as_str(), "zulu");
    }

    // -----------------------------------------------------------------------
    // 20. VecDeque<BTreeMap<String, u32>> roundtrip
    // -----------------------------------------------------------------------
    #[test]
    fn test_vecdeque_btreemap_string_u32_roundtrip() {
        let mut original: VecDeque<BTreeMap<String, u32>> = VecDeque::new();

        let mut m1 = BTreeMap::new();
        m1.insert("a".to_string(), 1u32);
        m1.insert("b".to_string(), 2u32);

        let m2: BTreeMap<String, u32> = BTreeMap::new();

        let mut m3 = BTreeMap::new();
        m3.insert("z".to_string(), 999u32);

        original.push_back(m1);
        original.push_front(m2);
        original.push_back(m3);

        let bytes =
            encode_to_vec(&original).expect("Failed to encode VecDeque<BTreeMap<String, u32>>");
        let (decoded, consumed): (VecDeque<BTreeMap<String, u32>>, _) =
            decode_from_slice(&bytes).expect("Failed to decode VecDeque<BTreeMap<String, u32>>");
        assert_eq!(original, decoded);
        assert_eq!(consumed, bytes.len());
    }

    // -----------------------------------------------------------------------
    // 21. LinkedList<Vec<u8>> roundtrip
    // -----------------------------------------------------------------------
    #[test]
    fn test_linkedlist_vec_u8_roundtrip() {
        let original: LinkedList<Vec<u8>> = vec![
            vec![0x01, 0x02, 0x03],
            vec![],
            vec![0xFF; 32],
            (128u8..=255).collect(),
            vec![0xDE, 0xAD, 0xBE, 0xEF],
        ]
        .into_iter()
        .collect();

        let bytes = encode_to_vec(&original).expect("Failed to encode LinkedList<Vec<u8>>");
        let (decoded, consumed): (LinkedList<Vec<u8>>, _) =
            decode_from_slice(&bytes).expect("Failed to decode LinkedList<Vec<u8>>");
        assert_eq!(original, decoded);
        assert_eq!(consumed, bytes.len());
    }

    // -----------------------------------------------------------------------
    // 22. encoded_size for empty vs populated nested collections
    // -----------------------------------------------------------------------
    #[test]
    fn test_encoded_size_empty_vs_populated_nested_collections() {
        // Test 1: empty Vec<HashMap<String, u32>> is smaller than a populated one
        let empty_vec: Vec<HashMap<String, u32>> = Vec::new();
        let mut populated_map = HashMap::new();
        populated_map.insert("key".to_string(), 42u32);
        let populated_vec: Vec<HashMap<String, u32>> = vec![populated_map];

        let empty_size = encoded_size(&empty_vec).expect("Failed to get encoded_size of empty Vec");
        let populated_size =
            encoded_size(&populated_vec).expect("Failed to get encoded_size of populated Vec");
        assert!(
            empty_size < populated_size,
            "Empty Vec ({} bytes) should be smaller than populated ({} bytes)",
            empty_size,
            populated_size
        );

        // Test 2: empty BTreeMap is smaller than populated BTreeMap
        let empty_map: BTreeMap<String, Vec<u32>> = BTreeMap::new();
        let mut full_map: BTreeMap<String, Vec<u32>> = BTreeMap::new();
        full_map.insert("numbers".to_string(), vec![1, 2, 3, 4, 5]);

        let empty_map_size =
            encoded_size(&empty_map).expect("Failed to get encoded_size of empty BTreeMap");
        let full_map_size =
            encoded_size(&full_map).expect("Failed to get encoded_size of populated BTreeMap");
        assert!(
            empty_map_size < full_map_size,
            "Empty BTreeMap ({} bytes) should be smaller than populated ({} bytes)",
            empty_map_size,
            full_map_size
        );

        // Test 3: encoded_size matches actual encode_to_vec length
        let nested: BTreeMap<String, Vec<Option<u32>>> = {
            let mut m = BTreeMap::new();
            m.insert("mixed".to_string(), vec![Some(1), None, Some(3)]);
            m.insert("all_none".to_string(), vec![None, None]);
            m
        };
        let computed_size =
            encoded_size(&nested).expect("Failed to compute encoded_size for nested BTreeMap");
        let actual_bytes = encode_to_vec(&nested).expect("Failed to encode nested BTreeMap");
        assert_eq!(
            computed_size,
            actual_bytes.len(),
            "encoded_size ({}) must equal actual encoded length ({})",
            computed_size,
            actual_bytes.len()
        );

        // Test 4: encoded_size with custom config
        let value: Vec<u32> = vec![1, 2, 3, 4, 5];
        let cfg = config::standard().with_fixed_int_encoding();
        let size_with_config = oxicode::encoded_size_with_config(&value, cfg)
            .expect("Failed to compute encoded_size_with_config");
        let bytes_with_config =
            oxicode::encode_to_vec_with_config(&value, cfg).expect("Failed to encode with config");
        assert_eq!(
            size_with_config,
            bytes_with_config.len(),
            "encoded_size_with_config must match actual encoded bytes length"
        );
    }
}
