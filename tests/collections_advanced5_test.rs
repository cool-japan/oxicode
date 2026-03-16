//! Advanced collection type encoding tests - set 5
//! Focuses on BTreeMap, BTreeSet, VecDeque, LinkedList, HashMap, HashSet,
//! nested structures, config variants, and Option wrappers.

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

// ── test 1 ──────────────────────────────────────────────────────────────────

#[test]
fn test_btreemap_string_u32_5_entries_roundtrip() {
    let mut val: BTreeMap<String, u32> = BTreeMap::new();
    val.insert("cherry".to_string(), 3);
    val.insert("apple".to_string(), 1);
    val.insert("elderberry".to_string(), 5);
    val.insert("date".to_string(), 4);
    val.insert("banana".to_string(), 2);
    // BTreeMap stores entries in sorted key order; verify that is preserved
    let enc = encode_to_vec(&val).expect("encode BTreeMap<String, u32> 5 entries");
    let (decoded, _): (BTreeMap<String, u32>, usize) =
        decode_from_slice(&enc).expect("decode BTreeMap<String, u32> 5 entries");
    assert_eq!(val, decoded);
    // Confirm sorted order is maintained
    let keys: Vec<&String> = decoded.keys().collect();
    assert_eq!(keys[0], "apple");
    assert_eq!(keys[4], "elderberry");
}

// ── test 2 ──────────────────────────────────────────────────────────────────

#[test]
fn test_btreemap_u32_string_5_entries_roundtrip() {
    let mut val: BTreeMap<u32, String> = BTreeMap::new();
    val.insert(42, "forty-two".to_string());
    val.insert(7, "seven".to_string());
    val.insert(100, "hundred".to_string());
    val.insert(1, "one".to_string());
    val.insert(55, "fifty-five".to_string());
    let enc = encode_to_vec(&val).expect("encode BTreeMap<u32, String> 5 entries");
    let (decoded, _): (BTreeMap<u32, String>, usize) =
        decode_from_slice(&enc).expect("decode BTreeMap<u32, String> 5 entries");
    assert_eq!(val, decoded);
    // u32 keys must appear in ascending order
    let keys: Vec<u32> = decoded.keys().copied().collect();
    assert_eq!(keys, vec![1, 7, 42, 55, 100]);
}

// ── test 3 ──────────────────────────────────────────────────────────────────

#[test]
fn test_btreeset_u32_5_elements_distinct_roundtrip() {
    let val: BTreeSet<u32> = [17u32, 3, 99, 42, 8].iter().copied().collect();
    let enc = encode_to_vec(&val).expect("encode BTreeSet<u32> 5 elements");
    let (decoded, _): (BTreeSet<u32>, usize) =
        decode_from_slice(&enc).expect("decode BTreeSet<u32> 5 elements");
    assert_eq!(val, decoded);
    assert_eq!(decoded.len(), 5);
    // Verify the minimum and maximum survive
    assert_eq!(*decoded.iter().next().expect("min"), 3);
    assert_eq!(*decoded.iter().next_back().expect("max"), 99);
}

// ── test 4 ──────────────────────────────────────────────────────────────────

#[test]
fn test_btreeset_string_5_elements_roundtrip() {
    let val: BTreeSet<String> = ["mango", "kiwi", "peach", "fig", "guava"]
        .iter()
        .map(|s| s.to_string())
        .collect();
    let enc = encode_to_vec(&val).expect("encode BTreeSet<String> 5 elements");
    let (decoded, _): (BTreeSet<String>, usize) =
        decode_from_slice(&enc).expect("decode BTreeSet<String> 5 elements");
    assert_eq!(val, decoded);
    assert_eq!(decoded.len(), 5);
}

// ── test 5 ──────────────────────────────────────────────────────────────────

#[test]
fn test_vecdeque_u32_5_push_back_roundtrip() {
    let mut val: VecDeque<u32> = VecDeque::new();
    val.push_back(101);
    val.push_back(202);
    val.push_back(303);
    val.push_back(404);
    val.push_back(505);
    let enc = encode_to_vec(&val).expect("encode VecDeque<u32> push_back");
    let (decoded, _): (VecDeque<u32>, usize) =
        decode_from_slice(&enc).expect("decode VecDeque<u32> push_back");
    assert_eq!(val, decoded);
    assert_eq!(decoded.len(), 5);
}

// ── test 6 ──────────────────────────────────────────────────────────────────

#[test]
fn test_vecdeque_string_5_elements_roundtrip() {
    let val: VecDeque<String> = ["north", "south", "east", "west", "center"]
        .iter()
        .map(|s| s.to_string())
        .collect();
    let enc = encode_to_vec(&val).expect("encode VecDeque<String> 5 elements");
    let (decoded, _): (VecDeque<String>, usize) =
        decode_from_slice(&enc).expect("decode VecDeque<String> 5 elements");
    assert_eq!(val, decoded);
}

// ── test 7 ──────────────────────────────────────────────────────────────────

#[test]
fn test_linkedlist_u32_mixed_push_roundtrip() {
    let mut val: LinkedList<u32> = LinkedList::new();
    val.push_back(10);
    val.push_back(20);
    val.push_front(5);
    val.push_back(30);
    val.push_front(1);
    // order: [1, 5, 10, 20, 30]
    let enc = encode_to_vec(&val).expect("encode LinkedList<u32>");
    let (decoded, _): (LinkedList<u32>, usize) =
        decode_from_slice(&enc).expect("decode LinkedList<u32>");
    assert_eq!(val, decoded);
    let decoded_vec: Vec<u32> = decoded.into_iter().collect();
    assert_eq!(decoded_vec, vec![1, 5, 10, 20, 30]);
}

// ── test 8 ──────────────────────────────────────────────────────────────────

#[test]
fn test_linkedlist_string_5_elements_roundtrip() {
    let val: LinkedList<String> = ["rust", "cargo", "crate", "trait", "lifetime"]
        .iter()
        .map(|s| s.to_string())
        .collect();
    let enc = encode_to_vec(&val).expect("encode LinkedList<String>");
    let (decoded, _): (LinkedList<String>, usize) =
        decode_from_slice(&enc).expect("decode LinkedList<String>");
    assert_eq!(val, decoded);
}

// ── test 9 ──────────────────────────────────────────────────────────────────

#[test]
fn test_btreemap_u32_btreeset_string_roundtrip() {
    // Replaces BinaryHeap (which is covered elsewhere) with a compound type
    let mut val: BTreeMap<u32, BTreeSet<String>> = BTreeMap::new();
    let set_a: BTreeSet<String> = ["foo", "bar", "baz"]
        .iter()
        .map(|s| s.to_string())
        .collect();
    let set_b: BTreeSet<String> = ["qux", "quux"].iter().map(|s| s.to_string()).collect();
    val.insert(1, set_a);
    val.insert(2, set_b);
    let enc = encode_to_vec(&val).expect("encode BTreeMap<u32, BTreeSet<String>>");
    let (decoded, _): (BTreeMap<u32, BTreeSet<String>>, usize) =
        decode_from_slice(&enc).expect("decode BTreeMap<u32, BTreeSet<String>>");
    assert_eq!(val, decoded);
    assert_eq!(decoded[&1].len(), 3);
    assert_eq!(decoded[&2].len(), 2);
}

// ── test 10 ─────────────────────────────────────────────────────────────────

#[test]
fn test_hashmap_string_u32_5_entries_roundtrip() {
    let mut val: HashMap<String, u32> = HashMap::new();
    val.insert("alpha".to_string(), 10);
    val.insert("beta".to_string(), 20);
    val.insert("gamma".to_string(), 30);
    val.insert("delta".to_string(), 40);
    val.insert("epsilon".to_string(), 50);
    let enc = encode_to_vec(&val).expect("encode HashMap<String, u32> 5 entries");
    let (decoded, _): (HashMap<String, u32>, usize) =
        decode_from_slice(&enc).expect("decode HashMap<String, u32> 5 entries");
    assert_eq!(val, decoded);
    assert_eq!(decoded.len(), 5);
}

// ── test 11 ─────────────────────────────────────────────────────────────────

#[test]
fn test_hashset_u32_5_elements_roundtrip() {
    let val: HashSet<u32> = [11u32, 22, 33, 44, 55].iter().copied().collect();
    let enc = encode_to_vec(&val).expect("encode HashSet<u32> 5 elements");
    let (decoded, _): (HashSet<u32>, usize) =
        decode_from_slice(&enc).expect("decode HashSet<u32> 5 elements");
    assert_eq!(val, decoded);
    assert_eq!(decoded.len(), 5);
}

// ── test 12 ─────────────────────────────────────────────────────────────────

#[test]
fn test_empty_btreemap_string_u32_roundtrip() {
    let val: BTreeMap<String, u32> = BTreeMap::new();
    let enc = encode_to_vec(&val).expect("encode empty BTreeMap<String, u32>");
    let (decoded, _): (BTreeMap<String, u32>, usize) =
        decode_from_slice(&enc).expect("decode empty BTreeMap<String, u32>");
    assert_eq!(val, decoded);
    assert!(decoded.is_empty());
}

// ── test 13 ─────────────────────────────────────────────────────────────────

#[test]
fn test_empty_vecdeque_string_roundtrip() {
    let val: VecDeque<String> = VecDeque::new();
    let enc = encode_to_vec(&val).expect("encode empty VecDeque<String>");
    let (decoded, _): (VecDeque<String>, usize) =
        decode_from_slice(&enc).expect("decode empty VecDeque<String>");
    assert_eq!(val, decoded);
    assert!(decoded.is_empty());
}

// ── test 14 ─────────────────────────────────────────────────────────────────

#[test]
fn test_empty_linkedlist_u32_roundtrip() {
    let val: LinkedList<u32> = LinkedList::new();
    let enc = encode_to_vec(&val).expect("encode empty LinkedList<u32>");
    let (decoded, _): (LinkedList<u32>, usize) =
        decode_from_slice(&enc).expect("decode empty LinkedList<u32>");
    assert_eq!(val, decoded);
    assert!(decoded.is_empty());
}

// ── test 15 ─────────────────────────────────────────────────────────────────

#[test]
fn test_nested_btreemap_string_vec_u32_roundtrip() {
    let mut val: BTreeMap<String, Vec<u32>> = BTreeMap::new();
    val.insert("primes".to_string(), vec![2, 3, 5, 7, 11]);
    val.insert("squares".to_string(), vec![1, 4, 9, 16, 25]);
    val.insert("empty".to_string(), vec![]);
    let enc = encode_to_vec(&val).expect("encode BTreeMap<String, Vec<u32>>");
    let (decoded, _): (BTreeMap<String, Vec<u32>>, usize) =
        decode_from_slice(&enc).expect("decode BTreeMap<String, Vec<u32>>");
    assert_eq!(val, decoded);
    assert_eq!(decoded["primes"], vec![2, 3, 5, 7, 11]);
    assert!(decoded["empty"].is_empty());
}

// ── test 16 ─────────────────────────────────────────────────────────────────

#[test]
fn test_vec_of_btreemap_string_u32_3_maps_roundtrip() {
    let mut map0: BTreeMap<String, u32> = BTreeMap::new();
    map0.insert("x".to_string(), 1);
    map0.insert("y".to_string(), 2);
    let mut map1: BTreeMap<String, u32> = BTreeMap::new();
    map1.insert("a".to_string(), 10);
    let map2: BTreeMap<String, u32> = BTreeMap::new(); // empty
    let val: Vec<BTreeMap<String, u32>> = vec![map0, map1, map2];
    let enc = encode_to_vec(&val).expect("encode Vec<BTreeMap<String, u32>> 3 maps");
    let (decoded, _): (Vec<BTreeMap<String, u32>>, usize) =
        decode_from_slice(&enc).expect("decode Vec<BTreeMap<String, u32>> 3 maps");
    assert_eq!(val, decoded);
    assert_eq!(decoded.len(), 3);
    assert_eq!(decoded[0]["x"], 1);
    assert!(decoded[2].is_empty());
}

// ── test 17 ─────────────────────────────────────────────────────────────────

#[test]
fn test_btreemap_string_u32_fixed_int_config_roundtrip() {
    let mut val: BTreeMap<String, u32> = BTreeMap::new();
    val.insert("uno".to_string(), 1);
    val.insert("dos".to_string(), 2);
    val.insert("tres".to_string(), 3);
    let cfg = config::standard().with_fixed_int_encoding();
    let enc = encode_to_vec_with_config(&val, cfg)
        .expect("encode BTreeMap<String, u32> fixed int config");
    let (decoded, _): (BTreeMap<String, u32>, usize) = decode_from_slice_with_config(&enc, cfg)
        .expect("decode BTreeMap<String, u32> fixed int config");
    assert_eq!(val, decoded);
}

// ── test 18 ─────────────────────────────────────────────────────────────────

#[test]
fn test_vecdeque_u32_fixed_int_config_roundtrip() {
    let val: VecDeque<u32> = [1000u32, 2000, 3000, 4000, 5000].iter().copied().collect();
    let cfg = config::standard().with_fixed_int_encoding();
    let enc = encode_to_vec_with_config(&val, cfg).expect("encode VecDeque<u32> fixed int config");
    let (decoded, _): (VecDeque<u32>, usize) =
        decode_from_slice_with_config(&enc, cfg).expect("decode VecDeque<u32> fixed int config");
    assert_eq!(val, decoded);
}

// ── test 19 ─────────────────────────────────────────────────────────────────

#[test]
fn test_btreemap_consumed_bytes_equals_encoded_length() {
    let mut val: BTreeMap<String, u32> = BTreeMap::new();
    val.insert("height".to_string(), 180);
    val.insert("weight".to_string(), 75);
    val.insert("age".to_string(), 30);
    let enc = encode_to_vec(&val).expect("encode BTreeMap for byte-count check");
    let (_, consumed): (BTreeMap<String, u32>, usize) =
        decode_from_slice(&enc).expect("decode BTreeMap for byte-count check");
    assert_eq!(
        consumed,
        enc.len(),
        "consumed bytes must equal the total encoded length"
    );
}

// ── test 20 ─────────────────────────────────────────────────────────────────

#[test]
fn test_option_btreemap_string_u32_some_roundtrip() {
    let mut inner: BTreeMap<String, u32> = BTreeMap::new();
    inner.insert("red".to_string(), 255);
    inner.insert("green".to_string(), 128);
    inner.insert("blue".to_string(), 64);
    let val: Option<BTreeMap<String, u32>> = Some(inner);
    let enc = encode_to_vec(&val).expect("encode Option<BTreeMap<String, u32>> Some");
    let (decoded, _): (Option<BTreeMap<String, u32>>, usize) =
        decode_from_slice(&enc).expect("decode Option<BTreeMap<String, u32>> Some");
    assert_eq!(val, decoded);
    let inner_dec = decoded.expect("decoded Option must be Some");
    assert_eq!(inner_dec["red"], 255);
}

// ── test 21 ─────────────────────────────────────────────────────────────────

#[test]
fn test_option_btreemap_string_u32_none_roundtrip() {
    let val: Option<BTreeMap<String, u32>> = None;
    let enc = encode_to_vec(&val).expect("encode Option<BTreeMap<String, u32>> None");
    let (decoded, _): (Option<BTreeMap<String, u32>>, usize) =
        decode_from_slice(&enc).expect("decode Option<BTreeMap<String, u32>> None");
    assert_eq!(val, decoded);
    assert!(decoded.is_none());
}

// ── test 22 ─────────────────────────────────────────────────────────────────

#[test]
fn test_large_btreemap_100_entries_roundtrip() {
    let val: BTreeMap<String, u32> = (0u32..100)
        .map(|i| (format!("key_{:03}", i), i * 7))
        .collect();
    assert_eq!(val.len(), 100);
    let enc = encode_to_vec(&val).expect("encode large BTreeMap 100 entries");
    let (decoded, consumed): (BTreeMap<String, u32>, usize) =
        decode_from_slice(&enc).expect("decode large BTreeMap 100 entries");
    assert_eq!(val, decoded);
    assert_eq!(decoded.len(), 100);
    assert_eq!(
        consumed,
        enc.len(),
        "consumed bytes must equal encoded length for 100-entry map"
    );
    // Spot-check a few entries
    assert_eq!(decoded["key_000"], 0);
    assert_eq!(decoded["key_007"], 49);
    assert_eq!(decoded["key_099"], 693);
}
