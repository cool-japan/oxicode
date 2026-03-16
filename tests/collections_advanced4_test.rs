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
use std::collections::{BTreeMap, BTreeSet, BinaryHeap, LinkedList, VecDeque};

#[test]
fn test_vecdeque_u32_empty_roundtrip() {
    let val: VecDeque<u32> = VecDeque::new();
    let enc = encode_to_vec(&val).expect("encode VecDeque<u32> empty");
    let (decoded, _): (VecDeque<u32>, usize) =
        decode_from_slice(&enc).expect("decode VecDeque<u32> empty");
    assert_eq!(val, decoded);
}

#[test]
fn test_vecdeque_u32_5_elements_roundtrip() {
    let val: VecDeque<u32> = vec![10u32, 20, 30, 40, 50].into_iter().collect();
    let enc = encode_to_vec(&val).expect("encode VecDeque<u32> 5 elements");
    let (decoded, _): (VecDeque<u32>, usize) =
        decode_from_slice(&enc).expect("decode VecDeque<u32> 5 elements");
    assert_eq!(val, decoded);
}

#[test]
fn test_vecdeque_string_roundtrip() {
    let val: VecDeque<String> = vec!["alpha".to_string(), "beta".to_string(), "gamma".to_string()]
        .into_iter()
        .collect();
    let enc = encode_to_vec(&val).expect("encode VecDeque<String>");
    let (decoded, _): (VecDeque<String>, usize) =
        decode_from_slice(&enc).expect("decode VecDeque<String>");
    assert_eq!(val, decoded);
}

#[test]
fn test_linkedlist_u32_empty_roundtrip() {
    let val: LinkedList<u32> = LinkedList::new();
    let enc = encode_to_vec(&val).expect("encode LinkedList<u32> empty");
    let (decoded, _): (LinkedList<u32>, usize) =
        decode_from_slice(&enc).expect("decode LinkedList<u32> empty");
    assert_eq!(val, decoded);
}

#[test]
fn test_linkedlist_u32_5_elements_roundtrip() {
    let val: LinkedList<u32> = vec![1u32, 2, 3, 4, 5].into_iter().collect();
    let enc = encode_to_vec(&val).expect("encode LinkedList<u32> 5 elements");
    let (decoded, _): (LinkedList<u32>, usize) =
        decode_from_slice(&enc).expect("decode LinkedList<u32> 5 elements");
    assert_eq!(val, decoded);
}

#[test]
fn test_linkedlist_string_roundtrip() {
    let val: LinkedList<String> = vec!["one".to_string(), "two".to_string(), "three".to_string()]
        .into_iter()
        .collect();
    let enc = encode_to_vec(&val).expect("encode LinkedList<String>");
    let (decoded, _): (LinkedList<String>, usize) =
        decode_from_slice(&enc).expect("decode LinkedList<String>");
    assert_eq!(val, decoded);
}

#[test]
fn test_binaryheap_u32_roundtrip() {
    let val: BinaryHeap<u32> = vec![3u32, 1, 4, 1, 5, 9, 2, 6].into_iter().collect();
    let enc = encode_to_vec(&val).expect("encode BinaryHeap<u32>");
    let (decoded, _): (BinaryHeap<u32>, usize) =
        decode_from_slice(&enc).expect("decode BinaryHeap<u32>");
    let mut orig: Vec<u32> = val.into_vec();
    let mut dec: Vec<u32> = decoded.into_vec();
    orig.sort();
    dec.sort();
    assert_eq!(orig, dec);
}

#[test]
fn test_binaryheap_i32_roundtrip() {
    let val: BinaryHeap<i32> = vec![-5i32, 0, 3, -1, 7, 2].into_iter().collect();
    let enc = encode_to_vec(&val).expect("encode BinaryHeap<i32>");
    let (decoded, _): (BinaryHeap<i32>, usize) =
        decode_from_slice(&enc).expect("decode BinaryHeap<i32>");
    let mut orig: Vec<i32> = val.into_vec();
    let mut dec: Vec<i32> = decoded.into_vec();
    orig.sort();
    dec.sort();
    assert_eq!(orig, dec);
}

#[test]
fn test_btreemap_u32_string_empty_roundtrip() {
    let val: BTreeMap<u32, String> = BTreeMap::new();
    let enc = encode_to_vec(&val).expect("encode BTreeMap<u32, String> empty");
    let (decoded, _): (BTreeMap<u32, String>, usize) =
        decode_from_slice(&enc).expect("decode BTreeMap<u32, String> empty");
    assert_eq!(val, decoded);
}

#[test]
fn test_btreemap_u32_string_5_entries_roundtrip() {
    let mut val: BTreeMap<u32, String> = BTreeMap::new();
    val.insert(1, "one".to_string());
    val.insert(2, "two".to_string());
    val.insert(3, "three".to_string());
    val.insert(4, "four".to_string());
    val.insert(5, "five".to_string());
    let enc = encode_to_vec(&val).expect("encode BTreeMap<u32, String> 5 entries");
    let (decoded, _): (BTreeMap<u32, String>, usize) =
        decode_from_slice(&enc).expect("decode BTreeMap<u32, String> 5 entries");
    assert_eq!(val, decoded);
}

#[test]
fn test_btreeset_u32_5_elements_roundtrip() {
    let val: BTreeSet<u32> = vec![100u32, 200, 300, 400, 500].into_iter().collect();
    let enc = encode_to_vec(&val).expect("encode BTreeSet<u32> 5 elements");
    let (decoded, _): (BTreeSet<u32>, usize) =
        decode_from_slice(&enc).expect("decode BTreeSet<u32> 5 elements");
    assert_eq!(val, decoded);
}

#[test]
fn test_btreeset_string_roundtrip() {
    let val: BTreeSet<String> = vec![
        "apple".to_string(),
        "banana".to_string(),
        "cherry".to_string(),
    ]
    .into_iter()
    .collect();
    let enc = encode_to_vec(&val).expect("encode BTreeSet<String>");
    let (decoded, _): (BTreeSet<String>, usize) =
        decode_from_slice(&enc).expect("decode BTreeSet<String>");
    assert_eq!(val, decoded);
}

#[test]
fn test_vecdeque_u32_consumed_bytes_equals_encoded_len() {
    let val: VecDeque<u32> = vec![7u32, 8, 9].into_iter().collect();
    let enc = encode_to_vec(&val).expect("encode VecDeque<u32> for byte count");
    let (_, consumed): (VecDeque<u32>, usize) =
        decode_from_slice(&enc).expect("decode VecDeque<u32> for byte count");
    assert_eq!(consumed, enc.len());
}

#[test]
fn test_btreemap_string_vec_u8_roundtrip() {
    let mut val: BTreeMap<String, Vec<u8>> = BTreeMap::new();
    val.insert("key_a".to_string(), vec![0u8, 1, 2, 3]);
    val.insert("key_b".to_string(), vec![255u8, 254, 253]);
    val.insert("key_c".to_string(), vec![]);
    let enc = encode_to_vec(&val).expect("encode BTreeMap<String, Vec<u8>>");
    let (decoded, _): (BTreeMap<String, Vec<u8>>, usize) =
        decode_from_slice(&enc).expect("decode BTreeMap<String, Vec<u8>>");
    assert_eq!(val, decoded);
}

#[test]
fn test_nested_btreemap_u32_btreemap_u32_string_roundtrip() {
    let mut inner1: BTreeMap<u32, String> = BTreeMap::new();
    inner1.insert(1, "inner_one".to_string());
    inner1.insert(2, "inner_two".to_string());
    let mut inner2: BTreeMap<u32, String> = BTreeMap::new();
    inner2.insert(10, "inner_ten".to_string());
    let mut val: BTreeMap<u32, BTreeMap<u32, String>> = BTreeMap::new();
    val.insert(100, inner1);
    val.insert(200, inner2);
    let enc = encode_to_vec(&val).expect("encode nested BTreeMap");
    let (decoded, _): (BTreeMap<u32, BTreeMap<u32, String>>, usize) =
        decode_from_slice(&enc).expect("decode nested BTreeMap");
    assert_eq!(val, decoded);
}

#[test]
fn test_vec_of_vecdeque_u32_roundtrip() {
    let val: Vec<VecDeque<u32>> = vec![
        vec![1u32, 2, 3].into_iter().collect(),
        VecDeque::new(),
        vec![4u32, 5].into_iter().collect(),
    ];
    let enc = encode_to_vec(&val).expect("encode Vec<VecDeque<u32>>");
    let (decoded, _): (Vec<VecDeque<u32>>, usize) =
        decode_from_slice(&enc).expect("decode Vec<VecDeque<u32>>");
    assert_eq!(val, decoded);
}

#[test]
fn test_option_vecdeque_u32_some_roundtrip() {
    let inner: VecDeque<u32> = vec![11u32, 22, 33].into_iter().collect();
    let val: Option<VecDeque<u32>> = Some(inner);
    let enc = encode_to_vec(&val).expect("encode Option<VecDeque<u32>> Some");
    let (decoded, _): (Option<VecDeque<u32>>, usize) =
        decode_from_slice(&enc).expect("decode Option<VecDeque<u32>> Some");
    assert_eq!(val, decoded);
}

#[test]
fn test_option_btreemap_u32_u32_none_roundtrip() {
    let val: Option<BTreeMap<u32, u32>> = None;
    let enc = encode_to_vec(&val).expect("encode Option<BTreeMap<u32, u32>> None");
    let (decoded, _): (Option<BTreeMap<u32, u32>>, usize) =
        decode_from_slice(&enc).expect("decode Option<BTreeMap<u32, u32>> None");
    assert_eq!(val, decoded);
}

#[test]
fn test_btreemap_u32_u32_fixed_int_config_roundtrip() {
    let mut val: BTreeMap<u32, u32> = BTreeMap::new();
    val.insert(1, 100);
    val.insert(2, 200);
    val.insert(3, 300);
    let cfg = config::standard().with_fixed_int_encoding();
    let enc = encode_to_vec_with_config(&val, cfg).expect("encode BTreeMap<u32, u32> fixed int");
    let (decoded, _): (BTreeMap<u32, u32>, usize) =
        decode_from_slice_with_config(&enc, cfg).expect("decode BTreeMap<u32, u32> fixed int");
    assert_eq!(val, decoded);
}

#[test]
fn test_large_vecdeque_u8_1000_elements_roundtrip() {
    let val: VecDeque<u8> = (0u8..=255).cycle().take(1000).collect();
    let enc = encode_to_vec(&val).expect("encode large VecDeque<u8> 1000 elements");
    let (decoded, _): (VecDeque<u8>, usize) =
        decode_from_slice(&enc).expect("decode large VecDeque<u8> 1000 elements");
    assert_eq!(val, decoded);
}

#[test]
fn test_btreeset_u32_100_elements_roundtrip() {
    let val: BTreeSet<u32> = (0u32..100).collect();
    let enc = encode_to_vec(&val).expect("encode BTreeSet<u32> 100 elements");
    let (decoded, _): (BTreeSet<u32>, usize) =
        decode_from_slice(&enc).expect("decode BTreeSet<u32> 100 elements");
    assert_eq!(val, decoded);
}

#[test]
fn test_linkedlist_vecdeque_u32_nested_roundtrip() {
    let dq1: VecDeque<u32> = vec![1u32, 2, 3].into_iter().collect();
    let dq2: VecDeque<u32> = VecDeque::new();
    let dq3: VecDeque<u32> = vec![4u32, 5, 6, 7].into_iter().collect();
    let mut val: LinkedList<VecDeque<u32>> = LinkedList::new();
    val.push_back(dq1);
    val.push_back(dq2);
    val.push_back(dq3);
    let enc = encode_to_vec(&val).expect("encode LinkedList<VecDeque<u32>>");
    let (decoded, _): (LinkedList<VecDeque<u32>>, usize) =
        decode_from_slice(&enc).expect("decode LinkedList<VecDeque<u32>>");
    assert_eq!(val, decoded);
}
