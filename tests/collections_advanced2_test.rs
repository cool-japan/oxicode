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
use oxicode::{
    config, decode_from_slice, decode_from_slice_with_config, encode_to_vec,
    encode_to_vec_with_config,
};
use std::collections::{BTreeMap, BTreeSet, HashMap, HashSet, LinkedList, VecDeque};

// -----------------------------------------------------------------------
// 1. BTreeSet<u32> roundtrip (preserves ordering)
// -----------------------------------------------------------------------
#[test]
fn test_btreeset_u32_roundtrip() {
    let mut original: BTreeSet<u32> = BTreeSet::new();
    original.insert(42);
    original.insert(7);
    original.insert(100);
    let encoded = encode_to_vec(&original).expect("encode BTreeSet<u32> failed");
    let (decoded, consumed): (BTreeSet<u32>, _) =
        decode_from_slice(&encoded).expect("decode BTreeSet<u32> failed");
    assert_eq!(decoded, original);
    assert_eq!(consumed, encoded.len());
    // BTreeSet preserves sorted order
    let v: Vec<u32> = decoded.iter().copied().collect();
    assert_eq!(v, vec![7, 42, 100]);
}

// -----------------------------------------------------------------------
// 2. BTreeSet<String> roundtrip (strings sorted alphabetically)
// -----------------------------------------------------------------------
#[test]
fn test_btreeset_string_alphabetical_order() {
    let mut original: BTreeSet<String> = BTreeSet::new();
    original.insert("mango".to_string());
    original.insert("apple".to_string());
    original.insert("cherry".to_string());
    original.insert("banana".to_string());
    let encoded = encode_to_vec(&original).expect("encode BTreeSet<String> failed");
    let (decoded, consumed): (BTreeSet<String>, _) =
        decode_from_slice(&encoded).expect("decode BTreeSet<String> failed");
    assert_eq!(decoded, original);
    assert_eq!(consumed, encoded.len());
    let v: Vec<&str> = decoded.iter().map(|s| s.as_str()).collect();
    assert_eq!(v, vec!["apple", "banana", "cherry", "mango"]);
}

// -----------------------------------------------------------------------
// 3. Empty BTreeSet<u32> roundtrip
// -----------------------------------------------------------------------
#[test]
fn test_btreeset_empty_roundtrip() {
    let original: BTreeSet<u32> = BTreeSet::new();
    let encoded = encode_to_vec(&original).expect("encode empty BTreeSet<u32> failed");
    let (decoded, consumed): (BTreeSet<u32>, _) =
        decode_from_slice(&encoded).expect("decode empty BTreeSet<u32> failed");
    assert!(decoded.is_empty());
    assert_eq!(decoded, original);
    assert_eq!(consumed, encoded.len());
}

// -----------------------------------------------------------------------
// 4. BTreeMap<u32, String> roundtrip
// -----------------------------------------------------------------------
#[test]
fn test_btreemap_u32_string_roundtrip() {
    let mut original: BTreeMap<u32, String> = BTreeMap::new();
    original.insert(3, "three".to_string());
    original.insert(1, "one".to_string());
    original.insert(2, "two".to_string());
    original.insert(100, "hundred".to_string());
    let encoded = encode_to_vec(&original).expect("encode BTreeMap<u32, String> failed");
    let (decoded, consumed): (BTreeMap<u32, String>, _) =
        decode_from_slice(&encoded).expect("decode BTreeMap<u32, String> failed");
    assert_eq!(decoded, original);
    assert_eq!(consumed, encoded.len());
    // Keys must be in ascending sorted order
    let keys: Vec<u32> = decoded.keys().copied().collect();
    assert_eq!(keys, vec![1, 2, 3, 100]);
}

// -----------------------------------------------------------------------
// 5. BTreeMap<String, Vec<u8>> roundtrip
// -----------------------------------------------------------------------
#[test]
fn test_btreemap_string_vec_u8_roundtrip() {
    let mut original: BTreeMap<String, Vec<u8>> = BTreeMap::new();
    original.insert("bytes_a".to_string(), vec![0x00, 0x01, 0xFF]);
    original.insert("bytes_b".to_string(), vec![]);
    original.insert("bytes_c".to_string(), (0u8..=15).collect());
    let encoded = encode_to_vec(&original).expect("encode BTreeMap<String, Vec<u8>> failed");
    let (decoded, consumed): (BTreeMap<String, Vec<u8>>, _) =
        decode_from_slice(&encoded).expect("decode BTreeMap<String, Vec<u8>> failed");
    assert_eq!(decoded, original);
    assert_eq!(consumed, encoded.len());
    assert_eq!(decoded.get("bytes_b"), Some(&vec![]));
}

// -----------------------------------------------------------------------
// 6. Empty BTreeMap<u32, u32> roundtrip
// -----------------------------------------------------------------------
#[test]
fn test_btreemap_empty_roundtrip() {
    let original: BTreeMap<u32, u32> = BTreeMap::new();
    let encoded = encode_to_vec(&original).expect("encode empty BTreeMap<u32, u32> failed");
    let (decoded, consumed): (BTreeMap<u32, u32>, _) =
        decode_from_slice(&encoded).expect("decode empty BTreeMap<u32, u32> failed");
    assert!(decoded.is_empty());
    assert_eq!(decoded, original);
    assert_eq!(consumed, encoded.len());
}

// -----------------------------------------------------------------------
// 7. VecDeque<u32> roundtrip
// -----------------------------------------------------------------------
#[test]
fn test_vecdeque_u32_roundtrip() {
    let original: VecDeque<u32> = vec![10u32, 20, 30, 40, 50].into_iter().collect();
    let encoded = encode_to_vec(&original).expect("encode VecDeque<u32> failed");
    let (decoded, consumed): (VecDeque<u32>, _) =
        decode_from_slice(&encoded).expect("decode VecDeque<u32> failed");
    assert_eq!(decoded, original);
    assert_eq!(consumed, encoded.len());
}

// -----------------------------------------------------------------------
// 8. VecDeque<String> roundtrip with push_front/push_back values
// -----------------------------------------------------------------------
#[test]
fn test_vecdeque_string_push_front_back_roundtrip() {
    let mut original: VecDeque<String> = VecDeque::new();
    original.push_back("middle".to_string());
    original.push_front("front_1".to_string());
    original.push_back("back_1".to_string());
    original.push_front("front_2".to_string());
    original.push_back("back_2".to_string());
    let encoded = encode_to_vec(&original).expect("encode VecDeque<String> failed");
    let (decoded, consumed): (VecDeque<String>, _) =
        decode_from_slice(&encoded).expect("decode VecDeque<String> failed");
    assert_eq!(decoded, original);
    assert_eq!(consumed, encoded.len());
    // Verify the sequence preserved by the deque ordering
    let v: Vec<&str> = decoded.iter().map(|s| s.as_str()).collect();
    assert_eq!(v, vec!["front_2", "front_1", "middle", "back_1", "back_2"]);
}

// -----------------------------------------------------------------------
// 9. Empty VecDeque<u8> roundtrip
// -----------------------------------------------------------------------
#[test]
fn test_vecdeque_empty_u8_roundtrip() {
    let original: VecDeque<u8> = VecDeque::new();
    let encoded = encode_to_vec(&original).expect("encode empty VecDeque<u8> failed");
    let (decoded, consumed): (VecDeque<u8>, _) =
        decode_from_slice(&encoded).expect("decode empty VecDeque<u8> failed");
    assert!(decoded.is_empty());
    assert_eq!(decoded, original);
    assert_eq!(consumed, encoded.len());
}

// -----------------------------------------------------------------------
// 10. LinkedList<u64> roundtrip
// -----------------------------------------------------------------------
#[test]
fn test_linkedlist_u64_roundtrip() {
    let original: LinkedList<u64> = vec![u64::MAX, 0, 1, 1_000_000_000, 42]
        .into_iter()
        .collect();
    let encoded = encode_to_vec(&original).expect("encode LinkedList<u64> failed");
    let (decoded, consumed): (LinkedList<u64>, _) =
        decode_from_slice(&encoded).expect("decode LinkedList<u64> failed");
    assert_eq!(decoded, original);
    assert_eq!(consumed, encoded.len());
}

// -----------------------------------------------------------------------
// 11. Empty LinkedList<u8> roundtrip
// -----------------------------------------------------------------------
#[test]
fn test_linkedlist_empty_u8_roundtrip() {
    let original: LinkedList<u8> = LinkedList::new();
    let encoded = encode_to_vec(&original).expect("encode empty LinkedList<u8> failed");
    let (decoded, consumed): (LinkedList<u8>, _) =
        decode_from_slice(&encoded).expect("decode empty LinkedList<u8> failed");
    assert!(decoded.is_empty());
    assert_eq!(decoded, original);
    assert_eq!(consumed, encoded.len());
}

// -----------------------------------------------------------------------
// 12. HashMap<u32, u32> single-element roundtrip (deterministic)
// -----------------------------------------------------------------------
#[test]
fn test_hashmap_single_entry_roundtrip() {
    let mut original: HashMap<u32, u32> = HashMap::new();
    original.insert(42, 100);
    let encoded = encode_to_vec(&original).expect("encode HashMap single entry failed");
    let (decoded, consumed): (HashMap<u32, u32>, _) =
        decode_from_slice(&encoded).expect("decode HashMap single entry failed");
    assert_eq!(decoded.get(&42), Some(&100));
    assert_eq!(decoded.len(), 1);
    assert_eq!(consumed, encoded.len());
}

// -----------------------------------------------------------------------
// 13. HashSet<u8> with all 0-9 values roundtrip
// -----------------------------------------------------------------------
#[test]
fn test_hashset_u8_zero_to_nine_roundtrip() {
    let original: HashSet<u8> = (0u8..=9).collect();
    let encoded = encode_to_vec(&original).expect("encode HashSet<u8> 0-9 failed");
    let (decoded, consumed): (HashSet<u8>, _) =
        decode_from_slice(&encoded).expect("decode HashSet<u8> 0-9 failed");
    assert_eq!(decoded, original);
    assert_eq!(decoded.len(), 10);
    assert_eq!(consumed, encoded.len());
    // Verify all values 0-9 are present
    for i in 0u8..=9 {
        assert!(decoded.contains(&i), "HashSet must contain {i}");
    }
}

// -----------------------------------------------------------------------
// 14. Nested: BTreeMap<String, BTreeSet<u32>> roundtrip
// -----------------------------------------------------------------------
#[test]
fn test_btreemap_string_btreeset_u32_roundtrip() {
    let mut original: BTreeMap<String, BTreeSet<u32>> = BTreeMap::new();

    let mut set_a: BTreeSet<u32> = BTreeSet::new();
    set_a.insert(5);
    set_a.insert(3);
    set_a.insert(8);

    let mut set_b: BTreeSet<u32> = BTreeSet::new();
    set_b.insert(100);
    set_b.insert(200);

    original.insert("first".to_string(), set_a);
    original.insert("second".to_string(), set_b);
    original.insert("empty".to_string(), BTreeSet::new());

    let encoded = encode_to_vec(&original).expect("encode BTreeMap<String, BTreeSet<u32>> failed");
    #[allow(clippy::type_complexity)]
    let (decoded, consumed): (BTreeMap<String, BTreeSet<u32>>, _) =
        decode_from_slice(&encoded).expect("decode BTreeMap<String, BTreeSet<u32>> failed");
    assert_eq!(decoded, original);
    assert_eq!(consumed, encoded.len());

    // Inner BTreeSet must also be sorted
    let first_vals: Vec<u32> = decoded
        .get("first")
        .expect("key 'first' missing")
        .iter()
        .copied()
        .collect();
    assert_eq!(first_vals, vec![3, 5, 8]);
}

// -----------------------------------------------------------------------
// 15. Nested: Vec<BTreeMap<u8, u8>> roundtrip
// -----------------------------------------------------------------------
#[test]
fn test_vec_btreemap_u8_u8_roundtrip() {
    let map1: BTreeMap<u8, u8> = [(1u8, 10u8), (2, 20), (3, 30)].into_iter().collect();
    let map2: BTreeMap<u8, u8> = [(100u8, 200u8)].into_iter().collect();
    let map3: BTreeMap<u8, u8> = BTreeMap::new();

    let original: Vec<BTreeMap<u8, u8>> = vec![map1, map2, map3];
    let encoded = encode_to_vec(&original).expect("encode Vec<BTreeMap<u8, u8>> failed");
    let (decoded, consumed): (Vec<BTreeMap<u8, u8>>, _) =
        decode_from_slice(&encoded).expect("decode Vec<BTreeMap<u8, u8>> failed");
    assert_eq!(decoded, original);
    assert_eq!(consumed, encoded.len());
    assert!(decoded[2].is_empty(), "third map must remain empty");
}

// -----------------------------------------------------------------------
// 16. BTreeSet<u32> byte size check (len varint + sorted elements)
// -----------------------------------------------------------------------
#[test]
fn test_btreeset_u32_byte_size() {
    // With standard config, a BTreeSet of three small u32 values encodes as:
    // 1 byte for length varint (3 < 251), plus 1 byte each for values < 251
    let mut original: BTreeSet<u32> = BTreeSet::new();
    original.insert(1);
    original.insert(2);
    original.insert(3);
    let encoded = encode_to_vec(&original).expect("encode BTreeSet<u32> size check failed");
    // 1 byte length + 3 x 1 byte values = 4 bytes total
    assert_eq!(
        encoded.len(),
        4,
        "BTreeSet of 3 small u32s must encode to 4 bytes"
    );
    let (decoded, consumed): (BTreeSet<u32>, _) =
        decode_from_slice(&encoded).expect("decode BTreeSet<u32> size check failed");
    assert_eq!(decoded, original);
    assert_eq!(consumed, 4);
}

// -----------------------------------------------------------------------
// 17. VecDeque<u32> with big-endian config roundtrip
// -----------------------------------------------------------------------
#[test]
fn test_vecdeque_u32_big_endian_config_roundtrip() {
    let original: VecDeque<u32> = vec![1u32, 256, 65535, u32::MAX].into_iter().collect();
    let cfg = config::standard().with_big_endian();
    let encoded =
        encode_to_vec_with_config(&original, cfg).expect("encode VecDeque<u32> big-endian failed");
    let (decoded, consumed): (VecDeque<u32>, _) = decode_from_slice_with_config(&encoded, cfg)
        .expect("decode VecDeque<u32> big-endian failed");
    assert_eq!(decoded, original);
    assert_eq!(consumed, encoded.len());
}

// -----------------------------------------------------------------------
// 18. BTreeMap<u8, u8> with fixed-int encoding config roundtrip
// -----------------------------------------------------------------------
#[test]
fn test_btreemap_u8_u8_fixed_int_config_roundtrip() {
    let mut original: BTreeMap<u8, u8> = BTreeMap::new();
    original.insert(0, 255);
    original.insert(128, 64);
    original.insert(255, 0);
    let cfg = config::legacy();
    let encoded =
        encode_to_vec_with_config(&original, cfg).expect("encode BTreeMap<u8, u8> legacy failed");
    let (decoded, consumed): (BTreeMap<u8, u8>, _) = decode_from_slice_with_config(&encoded, cfg)
        .expect("decode BTreeMap<u8, u8> legacy failed");
    assert_eq!(decoded, original);
    assert_eq!(consumed, encoded.len());
}

// -----------------------------------------------------------------------
// 19. Collection inside struct: struct with BTreeSet<String> field roundtrip
// -----------------------------------------------------------------------
#[derive(Debug, PartialEq, oxicode::Encode, oxicode::Decode)]
struct TaggedItem {
    name: String,
    tags: BTreeSet<String>,
}

#[test]
fn test_struct_with_btreeset_field_roundtrip() {
    let mut tags: BTreeSet<String> = BTreeSet::new();
    tags.insert("rust".to_string());
    tags.insert("serialization".to_string());
    tags.insert("binary".to_string());

    let original = TaggedItem {
        name: "oxicode".to_string(),
        tags,
    };

    let encoded = encode_to_vec(&original).expect("encode TaggedItem failed");
    let (decoded, consumed): (TaggedItem, _) =
        decode_from_slice(&encoded).expect("decode TaggedItem failed");
    assert_eq!(decoded, original);
    assert_eq!(consumed, encoded.len());

    // Tags must be sorted alphabetically
    let sorted_tags: Vec<&str> = decoded.tags.iter().map(|s| s.as_str()).collect();
    assert_eq!(sorted_tags, vec!["binary", "rust", "serialization"]);
}

// -----------------------------------------------------------------------
// 20. Vec<BTreeSet<u32>> roundtrip
// -----------------------------------------------------------------------
#[test]
fn test_vec_btreeset_u32_roundtrip() {
    let set1: BTreeSet<u32> = vec![30u32, 10, 20].into_iter().collect();
    let set2: BTreeSet<u32> = vec![999u32].into_iter().collect();
    let set3: BTreeSet<u32> = BTreeSet::new();
    let set4: BTreeSet<u32> = vec![1u32, 2, 3, 4, 5].into_iter().collect();

    let original: Vec<BTreeSet<u32>> = vec![set1, set2, set3, set4];
    let encoded = encode_to_vec(&original).expect("encode Vec<BTreeSet<u32>> failed");
    let (decoded, consumed): (Vec<BTreeSet<u32>>, _) =
        decode_from_slice(&encoded).expect("decode Vec<BTreeSet<u32>> failed");
    assert_eq!(decoded, original);
    assert_eq!(consumed, encoded.len());

    // First set must be in sorted order: [10, 20, 30]
    let first: Vec<u32> = decoded[0].iter().copied().collect();
    assert_eq!(first, vec![10, 20, 30]);
    // Third set must remain empty
    assert!(decoded[2].is_empty());
}

// -----------------------------------------------------------------------
// 21. BTreeMap<u32, BTreeMap<u32, String>> nested maps roundtrip
// -----------------------------------------------------------------------
#[test]
fn test_btreemap_nested_u32_btreemap_string_roundtrip() {
    let mut original: BTreeMap<u32, BTreeMap<u32, String>> = BTreeMap::new();

    let mut inner1: BTreeMap<u32, String> = BTreeMap::new();
    inner1.insert(10, "ten".to_string());
    inner1.insert(20, "twenty".to_string());

    let mut inner2: BTreeMap<u32, String> = BTreeMap::new();
    inner2.insert(100, "hundred".to_string());

    original.insert(1, inner1);
    original.insert(2, inner2);
    original.insert(3, BTreeMap::new());

    let encoded =
        encode_to_vec(&original).expect("encode BTreeMap<u32, BTreeMap<u32, String>> failed");
    #[allow(clippy::type_complexity)]
    let (decoded, consumed): (BTreeMap<u32, BTreeMap<u32, String>>, _) =
        decode_from_slice(&encoded).expect("decode BTreeMap<u32, BTreeMap<u32, String>> failed");
    assert_eq!(decoded, original);
    assert_eq!(consumed, encoded.len());

    // Inner maps must also preserve sorted key order
    let inner1_keys: Vec<u32> = decoded
        .get(&1)
        .expect("outer key 1 missing")
        .keys()
        .copied()
        .collect();
    assert_eq!(inner1_keys, vec![10, 20]);

    // Empty inner map survives
    assert!(decoded.get(&3).expect("outer key 3 missing").is_empty());
}

// -----------------------------------------------------------------------
// 22. LinkedList<Vec<u8>> roundtrip
// -----------------------------------------------------------------------
#[test]
fn test_linkedlist_vec_u8_roundtrip() {
    let mut original: LinkedList<Vec<u8>> = LinkedList::new();
    original.push_back(vec![0xDE, 0xAD, 0xBE, 0xEF]);
    original.push_back(vec![]);
    original.push_back((0u8..=10).collect());
    original.push_back(vec![255u8]);

    let encoded = encode_to_vec(&original).expect("encode LinkedList<Vec<u8>> failed");
    let (decoded, consumed): (LinkedList<Vec<u8>>, _) =
        decode_from_slice(&encoded).expect("decode LinkedList<Vec<u8>> failed");
    assert_eq!(decoded, original);
    assert_eq!(consumed, encoded.len());

    // Second element must remain empty vec
    let second = decoded
        .iter()
        .nth(1)
        .expect("LinkedList must have at least 2 elements");
    assert!(second.is_empty(), "second element must be an empty vec");
}
