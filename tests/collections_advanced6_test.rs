//! Advanced collection type encoding tests (set 6)

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
use oxicode::{decode_from_slice, encode_to_vec, Decode, Encode};
use std::collections::{BTreeMap, BTreeSet, HashMap, HashSet, VecDeque};

#[derive(Debug, PartialEq, Eq, Hash, PartialOrd, Ord, Clone, Encode, Decode)]
struct Tag {
    key: String,
    value: String,
}

// Test 1: BTreeMap<String, u32> roundtrip (5 entries)
#[test]
fn test_btreemap_string_u32_roundtrip() {
    let mut map: BTreeMap<String, u32> = BTreeMap::new();
    map.insert("alpha".to_string(), 1);
    map.insert("beta".to_string(), 2);
    map.insert("gamma".to_string(), 3);
    map.insert("delta".to_string(), 4);
    map.insert("epsilon".to_string(), 5);

    let bytes = encode_to_vec(&map).expect("Failed to encode BTreeMap<String, u32>");
    let (decoded, _): (BTreeMap<String, u32>, usize) =
        decode_from_slice(&bytes).expect("Failed to decode BTreeMap<String, u32>");

    assert_eq!(map, decoded);
}

// Test 2: BTreeMap<u32, String> roundtrip
#[test]
fn test_btreemap_u32_string_roundtrip() {
    let mut map: BTreeMap<u32, String> = BTreeMap::new();
    map.insert(10, "ten".to_string());
    map.insert(20, "twenty".to_string());
    map.insert(30, "thirty".to_string());

    let bytes = encode_to_vec(&map).expect("Failed to encode BTreeMap<u32, String>");
    let (decoded, _): (BTreeMap<u32, String>, usize) =
        decode_from_slice(&bytes).expect("Failed to decode BTreeMap<u32, String>");

    assert_eq!(map, decoded);
}

// Test 3: BTreeMap<String, Vec<u8>> roundtrip
#[test]
fn test_btreemap_string_vec_u8_roundtrip() {
    let mut map: BTreeMap<String, Vec<u8>> = BTreeMap::new();
    map.insert("bytes_a".to_string(), vec![0x01, 0x02, 0x03]);
    map.insert("bytes_b".to_string(), vec![0xAA, 0xBB, 0xCC, 0xDD]);
    map.insert("bytes_c".to_string(), vec![]);

    let bytes = encode_to_vec(&map).expect("Failed to encode BTreeMap<String, Vec<u8>>");
    let (decoded, _): (BTreeMap<String, Vec<u8>>, usize) =
        decode_from_slice(&bytes).expect("Failed to decode BTreeMap<String, Vec<u8>>");

    assert_eq!(map, decoded);
}

// Test 4: BTreeSet<u32> roundtrip (5 elements)
#[test]
fn test_btreeset_u32_roundtrip() {
    let mut set: BTreeSet<u32> = BTreeSet::new();
    set.insert(100);
    set.insert(200);
    set.insert(300);
    set.insert(400);
    set.insert(500);

    let bytes = encode_to_vec(&set).expect("Failed to encode BTreeSet<u32>");
    let (decoded, _): (BTreeSet<u32>, usize) =
        decode_from_slice(&bytes).expect("Failed to decode BTreeSet<u32>");

    assert_eq!(set, decoded);
}

// Test 5: BTreeSet<String> roundtrip
#[test]
fn test_btreeset_string_roundtrip() {
    let mut set: BTreeSet<String> = BTreeSet::new();
    set.insert("apple".to_string());
    set.insert("banana".to_string());
    set.insert("cherry".to_string());

    let bytes = encode_to_vec(&set).expect("Failed to encode BTreeSet<String>");
    let (decoded, _): (BTreeSet<String>, usize) =
        decode_from_slice(&bytes).expect("Failed to decode BTreeSet<String>");

    assert_eq!(set, decoded);
}

// Test 6: BTreeSet<Tag> roundtrip
#[test]
fn test_btreeset_tag_roundtrip() {
    let mut set: BTreeSet<Tag> = BTreeSet::new();
    set.insert(Tag {
        key: "env".to_string(),
        value: "production".to_string(),
    });
    set.insert(Tag {
        key: "region".to_string(),
        value: "us-east-1".to_string(),
    });
    set.insert(Tag {
        key: "tier".to_string(),
        value: "web".to_string(),
    });

    let bytes = encode_to_vec(&set).expect("Failed to encode BTreeSet<Tag>");
    let (decoded, _): (BTreeSet<Tag>, usize) =
        decode_from_slice(&bytes).expect("Failed to decode BTreeSet<Tag>");

    assert_eq!(set, decoded);
}

// Test 7: HashMap<String, u32> roundtrip
#[test]
fn test_hashmap_string_u32_roundtrip() {
    let mut map: HashMap<String, u32> = HashMap::new();
    map.insert("one".to_string(), 1);
    map.insert("two".to_string(), 2);
    map.insert("three".to_string(), 3);

    let bytes = encode_to_vec(&map).expect("Failed to encode HashMap<String, u32>");
    let (decoded, _): (HashMap<String, u32>, usize) =
        decode_from_slice(&bytes).expect("Failed to decode HashMap<String, u32>");

    assert_eq!(map, decoded);
}

// Test 8: HashSet<u32> roundtrip
#[test]
fn test_hashset_u32_roundtrip() {
    let mut set: HashSet<u32> = HashSet::new();
    set.insert(7);
    set.insert(14);
    set.insert(21);
    set.insert(28);

    let bytes = encode_to_vec(&set).expect("Failed to encode HashSet<u32>");
    let (decoded, _): (HashSet<u32>, usize) =
        decode_from_slice(&bytes).expect("Failed to decode HashSet<u32>");

    assert_eq!(set, decoded);
}

// Test 9: VecDeque<u32> roundtrip
#[test]
fn test_vecdeque_u32_roundtrip() {
    let mut deque: VecDeque<u32> = VecDeque::new();
    deque.push_back(1);
    deque.push_back(2);
    deque.push_back(3);
    deque.push_back(4);
    deque.push_back(5);

    let bytes = encode_to_vec(&deque).expect("Failed to encode VecDeque<u32>");
    let (decoded, _): (VecDeque<u32>, usize) =
        decode_from_slice(&bytes).expect("Failed to decode VecDeque<u32>");

    assert_eq!(deque, decoded);
}

// Test 10: VecDeque<String> roundtrip
#[test]
fn test_vecdeque_string_roundtrip() {
    let mut deque: VecDeque<String> = VecDeque::new();
    deque.push_back("first".to_string());
    deque.push_back("second".to_string());
    deque.push_back("third".to_string());

    let bytes = encode_to_vec(&deque).expect("Failed to encode VecDeque<String>");
    let (decoded, _): (VecDeque<String>, usize) =
        decode_from_slice(&bytes).expect("Failed to decode VecDeque<String>");

    assert_eq!(deque, decoded);
}

// Test 11: Empty BTreeMap roundtrip
#[test]
fn test_empty_btreemap_roundtrip() {
    let map: BTreeMap<String, u32> = BTreeMap::new();

    let bytes = encode_to_vec(&map).expect("Failed to encode empty BTreeMap");
    let (decoded, _): (BTreeMap<String, u32>, usize) =
        decode_from_slice(&bytes).expect("Failed to decode empty BTreeMap");

    assert_eq!(map, decoded);
    assert!(decoded.is_empty());
}

// Test 12: Empty BTreeSet roundtrip
#[test]
fn test_empty_btreeset_roundtrip() {
    let set: BTreeSet<u32> = BTreeSet::new();

    let bytes = encode_to_vec(&set).expect("Failed to encode empty BTreeSet");
    let (decoded, _): (BTreeSet<u32>, usize) =
        decode_from_slice(&bytes).expect("Failed to decode empty BTreeSet");

    assert_eq!(set, decoded);
    assert!(decoded.is_empty());
}

// Test 13: Empty HashMap roundtrip
#[test]
fn test_empty_hashmap_roundtrip() {
    let map: HashMap<String, u32> = HashMap::new();

    let bytes = encode_to_vec(&map).expect("Failed to encode empty HashMap");
    let (decoded, _): (HashMap<String, u32>, usize) =
        decode_from_slice(&bytes).expect("Failed to decode empty HashMap");

    assert_eq!(map, decoded);
    assert!(decoded.is_empty());
}

// Test 14: BTreeMap produces consistent ordering (sorted by key)
#[test]
fn test_btreemap_consistent_ordering() {
    let mut map1: BTreeMap<String, u32> = BTreeMap::new();
    map1.insert("zebra".to_string(), 26);
    map1.insert("apple".to_string(), 1);
    map1.insert("mango".to_string(), 13);

    let mut map2: BTreeMap<String, u32> = BTreeMap::new();
    map2.insert("apple".to_string(), 1);
    map2.insert("mango".to_string(), 13);
    map2.insert("zebra".to_string(), 26);

    let bytes1 = encode_to_vec(&map1).expect("Failed to encode map1");
    let bytes2 = encode_to_vec(&map2).expect("Failed to encode map2");

    // BTreeMap always iterates in sorted key order, so encoding should be identical
    assert_eq!(
        bytes1, bytes2,
        "BTreeMap encoding must be order-deterministic"
    );

    let (decoded1, _): (BTreeMap<String, u32>, usize) =
        decode_from_slice(&bytes1).expect("Failed to decode map1");
    assert_eq!(map1, decoded1);
}

// Test 15: Vec<BTreeMap<String, u32>> roundtrip
#[test]
fn test_vec_of_btreemap_roundtrip() {
    let mut map_a: BTreeMap<String, u32> = BTreeMap::new();
    map_a.insert("x".to_string(), 10);
    map_a.insert("y".to_string(), 20);

    let mut map_b: BTreeMap<String, u32> = BTreeMap::new();
    map_b.insert("p".to_string(), 100);
    map_b.insert("q".to_string(), 200);
    map_b.insert("r".to_string(), 300);

    let vec_of_maps: Vec<BTreeMap<String, u32>> = vec![map_a, map_b, BTreeMap::new()];

    let bytes = encode_to_vec(&vec_of_maps).expect("Failed to encode Vec<BTreeMap<String, u32>>");
    let (decoded, _): (Vec<BTreeMap<String, u32>>, usize) =
        decode_from_slice(&bytes).expect("Failed to decode Vec<BTreeMap<String, u32>>");

    assert_eq!(vec_of_maps, decoded);
}

// Test 16: BTreeMap<String, BTreeSet<u32>> nested roundtrip
#[test]
fn test_btreemap_nested_btreeset_roundtrip() {
    let mut map: BTreeMap<String, BTreeSet<u32>> = BTreeMap::new();

    let mut set_a: BTreeSet<u32> = BTreeSet::new();
    set_a.insert(1);
    set_a.insert(2);
    set_a.insert(3);
    map.insert("group_a".to_string(), set_a);

    let mut set_b: BTreeSet<u32> = BTreeSet::new();
    set_b.insert(10);
    set_b.insert(20);
    map.insert("group_b".to_string(), set_b);

    map.insert("group_empty".to_string(), BTreeSet::new());

    let bytes = encode_to_vec(&map).expect("Failed to encode BTreeMap<String, BTreeSet<u32>>");
    let (decoded, _): (BTreeMap<String, BTreeSet<u32>>, usize) =
        decode_from_slice(&bytes).expect("Failed to decode BTreeMap<String, BTreeSet<u32>>");

    assert_eq!(map, decoded);
}

// Test 17: HashMap<u32, Vec<Tag>> roundtrip
#[test]
fn test_hashmap_u32_vec_tag_roundtrip() {
    let mut map: HashMap<u32, Vec<Tag>> = HashMap::new();
    map.insert(
        1,
        vec![
            Tag {
                key: "color".to_string(),
                value: "red".to_string(),
            },
            Tag {
                key: "size".to_string(),
                value: "large".to_string(),
            },
        ],
    );
    map.insert(
        2,
        vec![Tag {
            key: "status".to_string(),
            value: "active".to_string(),
        }],
    );
    map.insert(3, vec![]);

    let bytes = encode_to_vec(&map).expect("Failed to encode HashMap<u32, Vec<Tag>>");
    let (decoded, _): (HashMap<u32, Vec<Tag>>, usize) =
        decode_from_slice(&bytes).expect("Failed to decode HashMap<u32, Vec<Tag>>");

    assert_eq!(map, decoded);
}

// Test 18: Consumed bytes equals encoded length for BTreeMap
#[test]
fn test_btreemap_consumed_bytes_equals_encoded_length() {
    let mut map: BTreeMap<String, u32> = BTreeMap::new();
    map.insert("a".to_string(), 1);
    map.insert("b".to_string(), 2);
    map.insert("c".to_string(), 3);

    let bytes = encode_to_vec(&map).expect("Failed to encode BTreeMap");
    let (_, consumed): (BTreeMap<String, u32>, usize) =
        decode_from_slice(&bytes).expect("Failed to decode BTreeMap");

    assert_eq!(
        consumed,
        bytes.len(),
        "Consumed bytes ({consumed}) must equal encoded length ({})",
        bytes.len()
    );
}

// Test 19: VecDeque same encoding as Vec for same elements
#[test]
fn test_vecdeque_same_encoding_as_vec() {
    let elements: Vec<u32> = vec![10, 20, 30, 40, 50];
    let deque: VecDeque<u32> = elements.iter().copied().collect();

    let vec_bytes = encode_to_vec(&elements).expect("Failed to encode Vec<u32>");
    let deque_bytes = encode_to_vec(&deque).expect("Failed to encode VecDeque<u32>");

    assert_eq!(
        vec_bytes, deque_bytes,
        "VecDeque and Vec with same elements must produce identical encoding"
    );
}

// Test 20: BTreeMap with 100 entries roundtrip
#[test]
fn test_btreemap_100_entries_roundtrip() {
    let mut map: BTreeMap<String, u32> = BTreeMap::new();
    for i in 0u32..100 {
        map.insert(format!("key_{i:04}"), i * 7);
    }

    assert_eq!(map.len(), 100);

    let bytes = encode_to_vec(&map).expect("Failed to encode BTreeMap with 100 entries");
    let (decoded, consumed): (BTreeMap<String, u32>, usize) =
        decode_from_slice(&bytes).expect("Failed to decode BTreeMap with 100 entries");

    assert_eq!(map, decoded);
    assert_eq!(decoded.len(), 100);
    assert_eq!(consumed, bytes.len());
}

// Test 21: BTreeSet with 50 strings roundtrip
#[test]
fn test_btreeset_50_strings_roundtrip() {
    let mut set: BTreeSet<String> = BTreeSet::new();
    for i in 0u32..50 {
        set.insert(format!("item_{i:03}"));
    }

    assert_eq!(set.len(), 50);

    let bytes = encode_to_vec(&set).expect("Failed to encode BTreeSet with 50 strings");
    let (decoded, consumed): (BTreeSet<String>, usize) =
        decode_from_slice(&bytes).expect("Failed to decode BTreeSet with 50 strings");

    assert_eq!(set, decoded);
    assert_eq!(decoded.len(), 50);
    assert_eq!(consumed, bytes.len());
}

// Test 22: Option<BTreeMap<String, u32>> roundtrip
#[test]
fn test_option_btreemap_roundtrip() {
    // Some case
    let mut inner: BTreeMap<String, u32> = BTreeMap::new();
    inner.insert("hello".to_string(), 42);
    inner.insert("world".to_string(), 99);

    let some_val: Option<BTreeMap<String, u32>> = Some(inner.clone());
    let bytes_some = encode_to_vec(&some_val).expect("Failed to encode Some(BTreeMap)");
    let (decoded_some, consumed_some): (Option<BTreeMap<String, u32>>, usize) =
        decode_from_slice(&bytes_some).expect("Failed to decode Some(BTreeMap)");

    assert_eq!(some_val, decoded_some);
    assert_eq!(consumed_some, bytes_some.len());

    // None case
    let none_val: Option<BTreeMap<String, u32>> = None;
    let bytes_none = encode_to_vec(&none_val).expect("Failed to encode None");
    let (decoded_none, consumed_none): (Option<BTreeMap<String, u32>>, usize) =
        decode_from_slice(&bytes_none).expect("Failed to decode None");

    assert_eq!(none_val, decoded_none);
    assert_eq!(consumed_none, bytes_none.len());
    assert!(decoded_none.is_none());
}
