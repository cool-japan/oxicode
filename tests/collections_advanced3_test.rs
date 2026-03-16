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
use std::collections::{BTreeMap, BTreeSet, BinaryHeap, HashMap, HashSet, LinkedList, VecDeque};

#[test]
fn test_vecdeque_u32_roundtrip() {
    let original: VecDeque<u32> = vec![10, 20, 30, 40, 50].into();
    let bytes = encode_to_vec(&original).expect("encode VecDeque<u32>");
    let (decoded, _): (VecDeque<u32>, usize) =
        decode_from_slice(&bytes).expect("decode VecDeque<u32>");
    assert_eq!(original, decoded);
}

#[test]
fn test_vecdeque_string_roundtrip() {
    let original: VecDeque<String> =
        vec!["alpha".to_string(), "beta".to_string(), "gamma".to_string()].into();
    let bytes = encode_to_vec(&original).expect("encode VecDeque<String>");
    let (decoded, _): (VecDeque<String>, usize) =
        decode_from_slice(&bytes).expect("decode VecDeque<String>");
    assert_eq!(original, decoded);
}

#[test]
fn test_linkedlist_u32_roundtrip() {
    let mut original: LinkedList<u32> = LinkedList::new();
    for v in [1u32, 2, 3, 4, 5] {
        original.push_back(v);
    }
    let bytes = encode_to_vec(&original).expect("encode LinkedList<u32>");
    let (decoded, _): (LinkedList<u32>, usize) =
        decode_from_slice(&bytes).expect("decode LinkedList<u32>");
    assert_eq!(original, decoded);
}

#[test]
fn test_linkedlist_string_roundtrip() {
    let mut original: LinkedList<String> = LinkedList::new();
    for s in ["hello", "world", "oxicode"] {
        original.push_back(s.to_string());
    }
    let bytes = encode_to_vec(&original).expect("encode LinkedList<String>");
    let (decoded, _): (LinkedList<String>, usize) =
        decode_from_slice(&bytes).expect("decode LinkedList<String>");
    assert_eq!(original, decoded);
}

#[test]
fn test_binaryheap_u32_elements_present() {
    let original: BinaryHeap<u32> = vec![5u32, 3, 8, 1, 9, 2].into();
    let bytes = encode_to_vec(&original).expect("encode BinaryHeap<u32>");
    let (decoded, _): (BinaryHeap<u32>, usize) =
        decode_from_slice(&bytes).expect("decode BinaryHeap<u32>");
    let mut orig_vec: Vec<u32> = original.into_iter().collect();
    orig_vec.sort();
    let mut dec_vec: Vec<u32> = decoded.into_iter().collect();
    dec_vec.sort();
    assert_eq!(orig_vec, dec_vec);
}

#[test]
fn test_hashset_u32_roundtrip() {
    let original: HashSet<u32> = vec![100u32, 200, 300, 400].into_iter().collect();
    let bytes = encode_to_vec(&original).expect("encode HashSet<u32>");
    let (decoded, _): (HashSet<u32>, usize) =
        decode_from_slice(&bytes).expect("decode HashSet<u32>");
    assert_eq!(original, decoded);
}

#[test]
fn test_hashset_string_roundtrip() {
    let original: HashSet<String> =
        vec!["rust".to_string(), "lang".to_string(), "encode".to_string()]
            .into_iter()
            .collect();
    let bytes = encode_to_vec(&original).expect("encode HashSet<String>");
    let (decoded, _): (HashSet<String>, usize) =
        decode_from_slice(&bytes).expect("decode HashSet<String>");
    assert_eq!(original, decoded);
}

#[test]
fn test_hashmap_string_vec_u32_roundtrip() {
    let mut original: HashMap<String, Vec<u32>> = HashMap::new();
    original.insert("evens".to_string(), vec![2, 4, 6, 8]);
    original.insert("odds".to_string(), vec![1, 3, 5, 7]);
    let bytes = encode_to_vec(&original).expect("encode HashMap<String, Vec<u32>>");
    let (decoded, _): (HashMap<String, Vec<u32>>, usize) =
        decode_from_slice(&bytes).expect("decode HashMap<String, Vec<u32>>");
    assert_eq!(original, decoded);
}

#[test]
fn test_hashmap_nested_u32_roundtrip() {
    let mut inner1: HashMap<u32, u32> = HashMap::new();
    inner1.insert(1, 10);
    inner1.insert(2, 20);
    let mut inner2: HashMap<u32, u32> = HashMap::new();
    inner2.insert(3, 30);
    inner2.insert(4, 40);
    let mut original: HashMap<u32, HashMap<u32, u32>> = HashMap::new();
    original.insert(100, inner1);
    original.insert(200, inner2);
    let bytes = encode_to_vec(&original).expect("encode HashMap<u32, HashMap<u32, u32>>");
    let (decoded, _): (HashMap<u32, HashMap<u32, u32>>, usize) =
        decode_from_slice(&bytes).expect("decode HashMap<u32, HashMap<u32, u32>>");
    assert_eq!(original, decoded);
}

#[test]
fn test_btreemap_string_btreeset_u32_roundtrip() {
    let mut original: BTreeMap<String, BTreeSet<u32>> = BTreeMap::new();
    let mut set1: BTreeSet<u32> = BTreeSet::new();
    set1.insert(1);
    set1.insert(2);
    set1.insert(3);
    let mut set2: BTreeSet<u32> = BTreeSet::new();
    set2.insert(10);
    set2.insert(20);
    original.insert("first".to_string(), set1);
    original.insert("second".to_string(), set2);
    let bytes = encode_to_vec(&original).expect("encode BTreeMap<String, BTreeSet<u32>>");
    let (decoded, _): (BTreeMap<String, BTreeSet<u32>>, usize) =
        decode_from_slice(&bytes).expect("decode BTreeMap<String, BTreeSet<u32>>");
    assert_eq!(original, decoded);
}

#[test]
fn test_vecdeque_vec_u8_roundtrip() {
    let original: VecDeque<Vec<u8>> =
        vec![vec![0u8, 1, 2], vec![255u8, 254, 253], vec![128u8, 64, 32]].into();
    let bytes = encode_to_vec(&original).expect("encode VecDeque<Vec<u8>>");
    let (decoded, _): (VecDeque<Vec<u8>>, usize) =
        decode_from_slice(&bytes).expect("decode VecDeque<Vec<u8>>");
    assert_eq!(original, decoded);
}

#[test]
fn test_empty_vecdeque_u32_roundtrip() {
    let original: VecDeque<u32> = VecDeque::new();
    let bytes = encode_to_vec(&original).expect("encode empty VecDeque<u32>");
    let (decoded, _): (VecDeque<u32>, usize) =
        decode_from_slice(&bytes).expect("decode empty VecDeque<u32>");
    assert_eq!(original, decoded);
    assert!(decoded.is_empty());
}

#[test]
fn test_empty_linkedlist_string_roundtrip() {
    let original: LinkedList<String> = LinkedList::new();
    let bytes = encode_to_vec(&original).expect("encode empty LinkedList<String>");
    let (decoded, _): (LinkedList<String>, usize) =
        decode_from_slice(&bytes).expect("decode empty LinkedList<String>");
    assert_eq!(original, decoded);
    assert!(decoded.is_empty());
}

#[test]
fn test_empty_binaryheap_u32_roundtrip() {
    let original: BinaryHeap<u32> = BinaryHeap::new();
    let bytes = encode_to_vec(&original).expect("encode empty BinaryHeap<u32>");
    let (decoded, _): (BinaryHeap<u32>, usize) =
        decode_from_slice(&bytes).expect("decode empty BinaryHeap<u32>");
    assert!(decoded.is_empty());
}

#[test]
fn test_large_vecdeque_roundtrip() {
    let original: VecDeque<u32> = (0u32..1000).collect();
    let bytes = encode_to_vec(&original).expect("encode large VecDeque<u32>");
    let (decoded, _): (VecDeque<u32>, usize) =
        decode_from_slice(&bytes).expect("decode large VecDeque<u32>");
    assert_eq!(original, decoded);
    assert_eq!(decoded.len(), 1000);
}

#[test]
fn test_large_linkedlist_roundtrip() {
    let mut original: LinkedList<u32> = LinkedList::new();
    for i in 0u32..500 {
        original.push_back(i);
    }
    let bytes = encode_to_vec(&original).expect("encode large LinkedList<u32>");
    let (decoded, _): (LinkedList<u32>, usize) =
        decode_from_slice(&bytes).expect("decode large LinkedList<u32>");
    assert_eq!(original, decoded);
    assert_eq!(decoded.len(), 500);
}

#[test]
fn test_vecdeque_with_config_fixed_int_encoding_roundtrip() {
    let original: VecDeque<u32> = vec![7u32, 14, 21, 28].into();
    let cfg = config::standard().with_fixed_int_encoding();
    let bytes = encode_to_vec_with_config(&original, cfg).expect("encode VecDeque<u32> fixed int");
    let (decoded, _): (VecDeque<u32>, usize) =
        decode_from_slice_with_config(&bytes, cfg).expect("decode VecDeque<u32> fixed int");
    assert_eq!(original, decoded);
}

#[test]
fn test_vecdeque_consumed_bytes_equals_encoded_length() {
    let original: VecDeque<u32> = vec![1u32, 2, 3, 4, 5].into();
    let bytes = encode_to_vec(&original).expect("encode VecDeque<u32> for bytes check");
    let (_, consumed): (VecDeque<u32>, usize) =
        decode_from_slice(&bytes).expect("decode VecDeque<u32> for bytes check");
    assert_eq!(consumed, bytes.len());
}

#[test]
fn test_option_vecdeque_some_roundtrip() {
    let inner: VecDeque<u32> = vec![11u32, 22, 33].into();
    let original: Option<VecDeque<u32>> = Some(inner);
    let bytes = encode_to_vec(&original).expect("encode Option<VecDeque<u32>> Some");
    let (decoded, _): (Option<VecDeque<u32>>, usize) =
        decode_from_slice(&bytes).expect("decode Option<VecDeque<u32>> Some");
    assert_eq!(original, decoded);
    assert!(decoded.is_some());
}

#[test]
fn test_option_linkedlist_none_roundtrip() {
    let original: Option<LinkedList<String>> = None;
    let bytes = encode_to_vec(&original).expect("encode Option<LinkedList<String>> None");
    let (decoded, _): (Option<LinkedList<String>>, usize) =
        decode_from_slice(&bytes).expect("decode Option<LinkedList<String>> None");
    assert_eq!(original, decoded);
    assert!(decoded.is_none());
}

#[test]
fn test_vec_of_vecdeque_roundtrip() {
    let original: Vec<VecDeque<u32>> = vec![
        vec![1u32, 2, 3].into(),
        vec![4u32, 5, 6].into(),
        vec![7u32, 8, 9].into(),
    ];
    let bytes = encode_to_vec(&original).expect("encode Vec<VecDeque<u32>>");
    let (decoded, _): (Vec<VecDeque<u32>>, usize) =
        decode_from_slice(&bytes).expect("decode Vec<VecDeque<u32>>");
    assert_eq!(original, decoded);
}

#[test]
fn test_binaryheap_decoded_sorted_equals_sorted_original() {
    let original: BinaryHeap<u32> = vec![42u32, 17, 99, 3, 56, 28, 71].into();
    let bytes = encode_to_vec(&original).expect("encode BinaryHeap<u32> for sort check");
    let (decoded, _): (BinaryHeap<u32>, usize) =
        decode_from_slice(&bytes).expect("decode BinaryHeap<u32> for sort check");
    let mut orig_vec: Vec<u32> = original.into_iter().collect();
    orig_vec.sort();
    let mut dec_vec: Vec<u32> = decoded.into_iter().collect();
    dec_vec.sort();
    assert_eq!(orig_vec, dec_vec);
}
