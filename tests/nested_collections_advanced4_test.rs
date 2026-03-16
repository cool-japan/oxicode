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
use oxicode::{decode_from_slice, encode_to_vec};
use std::collections::{BTreeMap, BTreeSet, HashMap, LinkedList, VecDeque};

#[test]
fn test_vec_vec_u32_3x3_matrix_roundtrip() {
    let matrix: Vec<Vec<u32>> = vec![vec![1, 2, 3], vec![4, 5, 6], vec![7, 8, 9]];
    let enc = encode_to_vec(&matrix).expect("encode vec<vec<u32>> 3x3");
    let (val, _): (Vec<Vec<u32>>, usize) =
        decode_from_slice(&enc).expect("decode vec<vec<u32>> 3x3");
    assert_eq!(val, matrix);
}

#[test]
fn test_vec_vec_vec_u8_3_deep_roundtrip() {
    let data: Vec<Vec<Vec<u8>>> = vec![
        vec![vec![1, 2], vec![3, 4]],
        vec![vec![5], vec![6, 7, 8]],
        vec![vec![], vec![9]],
    ];
    let enc = encode_to_vec(&data).expect("encode vec<vec<vec<u8>>>");
    let (val, _): (Vec<Vec<Vec<u8>>>, usize) =
        decode_from_slice(&enc).expect("decode vec<vec<vec<u8>>>");
    assert_eq!(val, data);
}

#[test]
fn test_vec_hashmap_string_u32_roundtrip() {
    let mut m1: HashMap<String, u32> = HashMap::new();
    m1.insert("alpha".to_string(), 10);
    m1.insert("beta".to_string(), 20);

    let mut m2: HashMap<String, u32> = HashMap::new();
    m2.insert("gamma".to_string(), 30);

    let data: Vec<HashMap<String, u32>> = vec![m1, m2];
    let enc = encode_to_vec(&data).expect("encode vec<hashmap<string, u32>>");
    let (val, _): (Vec<HashMap<String, u32>>, usize) =
        decode_from_slice(&enc).expect("decode vec<hashmap<string, u32>>");
    assert_eq!(val, data);
}

#[test]
fn test_hashmap_string_vec_u32_roundtrip() {
    let mut data: HashMap<String, Vec<u32>> = HashMap::new();
    data.insert("primes".to_string(), vec![2, 3, 5, 7, 11]);
    data.insert("evens".to_string(), vec![0, 2, 4, 6]);
    data.insert("empty".to_string(), vec![]);

    let enc = encode_to_vec(&data).expect("encode hashmap<string, vec<u32>>");
    let (val, _): (HashMap<String, Vec<u32>>, usize) =
        decode_from_slice(&enc).expect("decode hashmap<string, vec<u32>>");
    assert_eq!(val, data);
}

#[test]
fn test_hashmap_string_hashmap_string_u32_roundtrip() {
    let mut inner1: HashMap<String, u32> = HashMap::new();
    inner1.insert("x".to_string(), 1);
    inner1.insert("y".to_string(), 2);

    let mut inner2: HashMap<String, u32> = HashMap::new();
    inner2.insert("a".to_string(), 100);

    let mut data: HashMap<String, HashMap<String, u32>> = HashMap::new();
    data.insert("first".to_string(), inner1);
    data.insert("second".to_string(), inner2);

    let enc = encode_to_vec(&data).expect("encode hashmap<string, hashmap<string, u32>>");
    let (val, _): (HashMap<String, HashMap<String, u32>>, usize) =
        decode_from_slice(&enc).expect("decode hashmap<string, hashmap<string, u32>>");
    assert_eq!(val, data);
}

#[test]
fn test_btreemap_u32_vec_string_roundtrip() {
    let mut data: BTreeMap<u32, Vec<String>> = BTreeMap::new();
    data.insert(1, vec!["one".to_string(), "uno".to_string()]);
    data.insert(2, vec!["two".to_string(), "dos".to_string()]);
    data.insert(3, vec![]);

    let enc = encode_to_vec(&data).expect("encode btreemap<u32, vec<string>>");
    let (val, _): (BTreeMap<u32, Vec<String>>, usize) =
        decode_from_slice(&enc).expect("decode btreemap<u32, vec<string>>");
    assert_eq!(val, data);
}

#[test]
fn test_btreemap_string_btreeset_u32_roundtrip() {
    let mut data: BTreeMap<String, BTreeSet<u32>> = BTreeMap::new();
    let mut s1: BTreeSet<u32> = BTreeSet::new();
    s1.insert(10);
    s1.insert(20);
    s1.insert(30);
    data.insert("set_a".to_string(), s1);

    let mut s2: BTreeSet<u32> = BTreeSet::new();
    s2.insert(1);
    data.insert("set_b".to_string(), s2);

    let enc = encode_to_vec(&data).expect("encode btreemap<string, btreeset<u32>>");
    let (val, _): (BTreeMap<String, BTreeSet<u32>>, usize) =
        decode_from_slice(&enc).expect("decode btreemap<string, btreeset<u32>>");
    assert_eq!(val, data);
}

#[test]
fn test_vec_btreemap_u32_u32_roundtrip() {
    let mut m1: BTreeMap<u32, u32> = BTreeMap::new();
    m1.insert(1, 100);
    m1.insert(2, 200);

    let mut m2: BTreeMap<u32, u32> = BTreeMap::new();
    m2.insert(3, 300);

    let data: Vec<BTreeMap<u32, u32>> = vec![m1, m2, BTreeMap::new()];
    let enc = encode_to_vec(&data).expect("encode vec<btreemap<u32, u32>>");
    let (val, _): (Vec<BTreeMap<u32, u32>>, usize) =
        decode_from_slice(&enc).expect("decode vec<btreemap<u32, u32>>");
    assert_eq!(val, data);
}

#[test]
fn test_option_vec_vec_u8_some_roundtrip() {
    let inner: Option<Vec<Vec<u8>>> = Some(vec![vec![1, 2, 3], vec![4, 5], vec![]]);
    let enc = encode_to_vec(&inner).expect("encode option<vec<vec<u8>>> some");
    let (val, _): (Option<Vec<Vec<u8>>>, usize) =
        decode_from_slice(&enc).expect("decode option<vec<vec<u8>>> some");
    assert_eq!(val, inner);
}

#[test]
fn test_option_hashmap_string_u32_none_roundtrip() {
    let data: Option<HashMap<String, u32>> = None;
    let enc = encode_to_vec(&data).expect("encode option<hashmap<string, u32>> none");
    let (val, _): (Option<HashMap<String, u32>>, usize) =
        decode_from_slice(&enc).expect("decode option<hashmap<string, u32>> none");
    assert_eq!(val, None);
}

#[test]
fn test_vec_option_string_mix_roundtrip() {
    let data: Vec<Option<String>> = vec![
        Some("hello".to_string()),
        None,
        Some("world".to_string()),
        None,
        Some("foo".to_string()),
    ];
    let enc = encode_to_vec(&data).expect("encode vec<option<string>>");
    let (val, _): (Vec<Option<String>>, usize) =
        decode_from_slice(&enc).expect("decode vec<option<string>>");
    assert_eq!(val, data);
}

#[test]
fn test_hashmap_string_option_u32_roundtrip() {
    let mut data: HashMap<String, Option<u32>> = HashMap::new();
    data.insert("present".to_string(), Some(42));
    data.insert("absent".to_string(), None);
    data.insert("also_present".to_string(), Some(999));

    let enc = encode_to_vec(&data).expect("encode hashmap<string, option<u32>>");
    let (val, _): (HashMap<String, Option<u32>>, usize) =
        decode_from_slice(&enc).expect("decode hashmap<string, option<u32>>");
    assert_eq!(val, data);
}

#[test]
fn test_vec_tuple_string_vec_u8_roundtrip() {
    let data: Vec<(String, Vec<u8>)> = vec![
        ("first".to_string(), vec![1, 2, 3]),
        ("second".to_string(), vec![4, 5, 6, 7]),
        ("empty".to_string(), vec![]),
    ];
    let enc = encode_to_vec(&data).expect("encode vec<(string, vec<u8>)>");
    let (val, _): (Vec<(String, Vec<u8>)>, usize) =
        decode_from_slice(&enc).expect("decode vec<(string, vec<u8>)>");
    assert_eq!(val, data);
}

#[test]
fn test_btreemap_u32_option_string_roundtrip() {
    let mut data: BTreeMap<u32, Option<String>> = BTreeMap::new();
    data.insert(1, Some("one".to_string()));
    data.insert(2, None);
    data.insert(3, Some("three".to_string()));
    data.insert(4, None);

    let enc = encode_to_vec(&data).expect("encode btreemap<u32, option<string>>");
    let (val, _): (BTreeMap<u32, Option<String>>, usize) =
        decode_from_slice(&enc).expect("decode btreemap<u32, option<string>>");
    assert_eq!(val, data);
}

#[test]
fn test_vec_vecdeque_u32_roundtrip() {
    let mut vd1: VecDeque<u32> = VecDeque::new();
    vd1.push_back(10);
    vd1.push_back(20);
    vd1.push_front(5);

    let mut vd2: VecDeque<u32> = VecDeque::new();
    vd2.push_back(100);

    let data: Vec<VecDeque<u32>> = vec![vd1, vd2, VecDeque::new()];
    let enc = encode_to_vec(&data).expect("encode vec<vecdeque<u32>>");
    let (val, _): (Vec<VecDeque<u32>>, usize) =
        decode_from_slice(&enc).expect("decode vec<vecdeque<u32>>");
    assert_eq!(val, data);
}

#[test]
fn test_hashmap_u32_btreeset_string_roundtrip() {
    let mut data: HashMap<u32, BTreeSet<String>> = HashMap::new();
    let mut s1: BTreeSet<String> = BTreeSet::new();
    s1.insert("apple".to_string());
    s1.insert("banana".to_string());
    data.insert(1, s1);

    let mut s2: BTreeSet<String> = BTreeSet::new();
    s2.insert("cherry".to_string());
    data.insert(2, s2);

    data.insert(3, BTreeSet::new());

    let enc = encode_to_vec(&data).expect("encode hashmap<u32, btreeset<string>>");
    let (val, _): (HashMap<u32, BTreeSet<String>>, usize) =
        decode_from_slice(&enc).expect("decode hashmap<u32, btreeset<string>>");
    assert_eq!(val, data);
}

#[test]
fn test_vec_linkedlist_u32_roundtrip() {
    let mut ll1: LinkedList<u32> = LinkedList::new();
    ll1.push_back(1);
    ll1.push_back(2);
    ll1.push_back(3);

    let mut ll2: LinkedList<u32> = LinkedList::new();
    ll2.push_back(99);

    let data: Vec<LinkedList<u32>> = vec![ll1, ll2, LinkedList::new()];
    let enc = encode_to_vec(&data).expect("encode vec<linkedlist<u32>>");
    let (val, _): (Vec<LinkedList<u32>>, usize) =
        decode_from_slice(&enc).expect("decode vec<linkedlist<u32>>");
    assert_eq!(val, data);
}

#[test]
fn test_hashmap_string_vec_hashmap_u32_bool_3_deep_roundtrip() {
    let mut inner1: HashMap<u32, bool> = HashMap::new();
    inner1.insert(1, true);
    inner1.insert(2, false);

    let mut inner2: HashMap<u32, bool> = HashMap::new();
    inner2.insert(3, true);

    let mut data: HashMap<String, Vec<HashMap<u32, bool>>> = HashMap::new();
    data.insert("key1".to_string(), vec![inner1, inner2]);
    data.insert("key2".to_string(), vec![]);

    let enc = encode_to_vec(&data).expect("encode hashmap<string, vec<hashmap<u32, bool>>>");
    let (val, _): (HashMap<String, Vec<HashMap<u32, bool>>>, usize) =
        decode_from_slice(&enc).expect("decode hashmap<string, vec<hashmap<u32, bool>>>");
    assert_eq!(val, data);
}

#[test]
fn test_btreemap_string_btreemap_u32_vec_u8_roundtrip() {
    let mut inner1: BTreeMap<u32, Vec<u8>> = BTreeMap::new();
    inner1.insert(1, vec![10, 20, 30]);
    inner1.insert(2, vec![40, 50]);

    let mut inner2: BTreeMap<u32, Vec<u8>> = BTreeMap::new();
    inner2.insert(99, vec![]);

    let mut data: BTreeMap<String, BTreeMap<u32, Vec<u8>>> = BTreeMap::new();
    data.insert("outer_a".to_string(), inner1);
    data.insert("outer_b".to_string(), inner2);

    let enc = encode_to_vec(&data).expect("encode btreemap<string, btreemap<u32, vec<u8>>>");
    let (val, _): (BTreeMap<String, BTreeMap<u32, Vec<u8>>>, usize) =
        decode_from_slice(&enc).expect("decode btreemap<string, btreemap<u32, vec<u8>>>");
    assert_eq!(val, data);
}

#[test]
fn test_large_vec_vec_u8_100x10_roundtrip() {
    let data: Vec<Vec<u8>> = (0u8..100)
        .map(|i| (0u8..10).map(|j| i.wrapping_add(j)).collect())
        .collect();
    let enc = encode_to_vec(&data).expect("encode large vec<vec<u8>> 100x10");
    let (val, _): (Vec<Vec<u8>>, usize) =
        decode_from_slice(&enc).expect("decode large vec<vec<u8>> 100x10");
    assert_eq!(val, data);
    assert_eq!(val.len(), 100);
    assert!(val.iter().all(|v| v.len() == 10));
}

#[test]
fn test_empty_inner_vecs_roundtrip() {
    let data: Vec<Vec<u8>> = vec![vec![], vec![], vec![], vec![]];
    let enc = encode_to_vec(&data).expect("encode vec<vec<u8>> all empty inner");
    let (val, _): (Vec<Vec<u8>>, usize) =
        decode_from_slice(&enc).expect("decode vec<vec<u8>> all empty inner");
    assert_eq!(val, data);
    assert!(val.iter().all(|v| v.is_empty()));
}

#[test]
fn test_option_vec_hashmap_string_u32_some_3_maps_roundtrip() {
    let mut m1: HashMap<String, u32> = HashMap::new();
    m1.insert("a".to_string(), 1);
    m1.insert("b".to_string(), 2);

    let mut m2: HashMap<String, u32> = HashMap::new();
    m2.insert("c".to_string(), 3);

    let mut m3: HashMap<String, u32> = HashMap::new();
    m3.insert("d".to_string(), 4);
    m3.insert("e".to_string(), 5);
    m3.insert("f".to_string(), 6);

    let data: Option<Vec<HashMap<String, u32>>> = Some(vec![m1, m2, m3]);
    let enc = encode_to_vec(&data).expect("encode option<vec<hashmap<string, u32>>> some 3 maps");
    let (val, _): (Option<Vec<HashMap<String, u32>>>, usize) =
        decode_from_slice(&enc).expect("decode option<vec<hashmap<string, u32>>> some 3 maps");
    assert_eq!(val, data);
    assert_eq!(val.as_ref().expect("should be some").len(), 3);
}
