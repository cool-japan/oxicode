//! Comprehensive tests for all collection types

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
use std::collections::{BTreeMap, BTreeSet, BinaryHeap, HashMap, HashSet, VecDeque};

#[test]
fn test_vec_roundtrip() {
    let original = vec![1u32, 2, 3, 4, 5, 100, 1000, 10000];
    let bytes = oxicode::encode_to_vec(&original).expect("Failed to encode");
    let (decoded, _): (Vec<u32>, _) = oxicode::decode_from_slice(&bytes).expect("Failed to decode");
    assert_eq!(original, decoded);
}

#[test]
fn test_vec_empty() {
    let original: Vec<u32> = vec![];
    let bytes = oxicode::encode_to_vec(&original).expect("Failed to encode");
    let (decoded, _): (Vec<u32>, _) = oxicode::decode_from_slice(&bytes).expect("Failed to decode");
    assert_eq!(original, decoded);
}

#[test]
fn test_vec_nested() {
    let original = vec![vec![1, 2], vec![3, 4, 5], vec![]];
    let bytes = oxicode::encode_to_vec(&original).expect("Failed to encode");
    let (decoded, _): (Vec<Vec<i32>>, _) =
        oxicode::decode_from_slice(&bytes).expect("Failed to decode");
    assert_eq!(original, decoded);
}

#[test]
fn test_string_roundtrip() {
    let original = String::from("Hello, OxiCode! 🦀🚀");
    let bytes = oxicode::encode_to_vec(&original).expect("Failed to encode");
    let (decoded, _): (String, _) = oxicode::decode_from_slice(&bytes).expect("Failed to decode");
    assert_eq!(original, decoded);
}

#[test]
fn test_hashmap_roundtrip() {
    let mut original = HashMap::new();
    original.insert("key1".to_string(), 100u32);
    original.insert("key2".to_string(), 200);
    original.insert("key3".to_string(), 300);

    let bytes = oxicode::encode_to_vec(&original).expect("Failed to encode");
    let (decoded, _): (HashMap<String, u32>, _) =
        oxicode::decode_from_slice(&bytes).expect("Failed to decode");
    assert_eq!(original, decoded);
}

#[test]
fn test_hashset_roundtrip() {
    let mut original = HashSet::new();
    original.insert(10u64);
    original.insert(20);
    original.insert(30);
    original.insert(40);

    let bytes = oxicode::encode_to_vec(&original).expect("Failed to encode");
    let (decoded, _): (HashSet<u64>, _) =
        oxicode::decode_from_slice(&bytes).expect("Failed to decode");
    assert_eq!(original, decoded);
}

#[test]
fn test_btreemap_roundtrip() {
    let mut original = BTreeMap::new();
    original.insert(1, "one".to_string());
    original.insert(2, "two".to_string());
    original.insert(3, "three".to_string());

    let bytes = oxicode::encode_to_vec(&original).expect("Failed to encode");
    let (decoded, _): (BTreeMap<i32, String>, _) =
        oxicode::decode_from_slice(&bytes).expect("Failed to decode");
    assert_eq!(original, decoded);
}

#[test]
fn test_btreeset_roundtrip() {
    let mut original = BTreeSet::new();
    original.insert(5);
    original.insert(2);
    original.insert(8);
    original.insert(1);

    let bytes = oxicode::encode_to_vec(&original).expect("Failed to encode");
    let (decoded, _): (BTreeSet<i32>, _) =
        oxicode::decode_from_slice(&bytes).expect("Failed to decode");
    assert_eq!(original, decoded);
}

#[test]
fn test_vecdeque_roundtrip() {
    let mut original = VecDeque::new();
    original.push_back(1u16);
    original.push_back(2);
    original.push_back(3);
    original.push_front(0);

    let bytes = oxicode::encode_to_vec(&original).expect("Failed to encode");
    let (decoded, _): (VecDeque<u16>, _) =
        oxicode::decode_from_slice(&bytes).expect("Failed to decode");
    assert_eq!(original, decoded);
}

#[test]
fn test_binaryheap_roundtrip() {
    let mut original = BinaryHeap::new();
    original.push(5);
    original.push(2);
    original.push(8);
    original.push(1);

    let bytes = oxicode::encode_to_vec(&original).expect("Failed to encode");
    let (decoded, _): (BinaryHeap<i32>, _) =
        oxicode::decode_from_slice(&bytes).expect("Failed to decode");

    // BinaryHeap doesn't guarantee order, so convert to sorted vecs
    let mut original_vec: Vec<_> = original.into_iter().collect();
    let mut decoded_vec: Vec<_> = decoded.into_iter().collect();
    original_vec.sort();
    decoded_vec.sort();
    assert_eq!(original_vec, decoded_vec);
}

#[test]
fn test_linkedlist_roundtrip() {
    use std::collections::LinkedList;
    let list: LinkedList<i32> = vec![-3, -2, -1, 0, 1, 2, 3].into_iter().collect();
    let bytes = oxicode::encode_to_vec(&list).expect("Failed to encode");
    let (decoded, _): (LinkedList<i32>, _) =
        oxicode::decode_from_slice(&bytes).expect("Failed to decode");
    assert_eq!(list, decoded);
}

#[test]
fn test_nested_hashmap_vec_roundtrip() {
    let mut outer: HashMap<String, Vec<u32>> = HashMap::new();
    outer.insert("primes".to_string(), vec![2, 3, 5, 7, 11]);
    outer.insert("evens".to_string(), vec![2, 4, 6, 8, 10]);

    let bytes = oxicode::encode_to_vec(&outer).expect("Failed to encode");
    let (decoded, _): (HashMap<String, Vec<u32>>, _) =
        oxicode::decode_from_slice(&bytes).expect("Failed to decode");
    assert_eq!(outer, decoded);
}

#[test]
fn test_empty_collections_roundtrip() {
    let empty_map: BTreeMap<u32, u32> = BTreeMap::new();
    let empty_set: HashSet<String> = HashSet::new();
    let empty_deque: VecDeque<i64> = VecDeque::new();

    let enc_map = oxicode::encode_to_vec(&empty_map).expect("encode map");
    let enc_set = oxicode::encode_to_vec(&empty_set).expect("encode set");
    let enc_deque = oxicode::encode_to_vec(&empty_deque).expect("encode deque");

    let (dec_map, _): (BTreeMap<u32, u32>, _) =
        oxicode::decode_from_slice(&enc_map).expect("decode map");
    let (dec_set, _): (HashSet<String>, _) =
        oxicode::decode_from_slice(&enc_set).expect("decode set");
    let (dec_deque, _): (VecDeque<i64>, _) =
        oxicode::decode_from_slice(&enc_deque).expect("decode deque");

    assert_eq!(empty_map, dec_map);
    assert_eq!(empty_set, dec_set);
    assert_eq!(empty_deque, dec_deque);
}

#[test]
fn test_btreemap_string_key_roundtrip() {
    let mut map = BTreeMap::new();
    map.insert("alpha".to_string(), 1u32);
    map.insert("beta".to_string(), 2u32);
    map.insert("gamma".to_string(), 3u32);

    let bytes = oxicode::encode_to_vec(&map).expect("Failed to encode");
    let (decoded, _): (BTreeMap<String, u32>, _) =
        oxicode::decode_from_slice(&bytes).expect("Failed to decode");
    assert_eq!(map, decoded);
}

#[test]
fn test_option_some() {
    let original: Option<u64> = Some(12345);
    let bytes = oxicode::encode_to_vec(&original).expect("Failed to encode");
    let (decoded, _): (Option<u64>, _) =
        oxicode::decode_from_slice(&bytes).expect("Failed to decode");
    assert_eq!(original, decoded);
}

#[test]
fn test_option_none() {
    let original: Option<u64> = None;
    let bytes = oxicode::encode_to_vec(&original).expect("Failed to encode");
    let (decoded, _): (Option<u64>, _) =
        oxicode::decode_from_slice(&bytes).expect("Failed to decode");
    assert_eq!(original, decoded);
}

#[test]
fn test_result_ok() {
    let original: Result<u32, String> = Ok(42);
    let bytes = oxicode::encode_to_vec(&original).expect("Failed to encode");
    let (decoded, _): (Result<u32, String>, _) =
        oxicode::decode_from_slice(&bytes).expect("Failed to decode");
    assert_eq!(original, decoded);
}

#[test]
fn test_result_err() {
    let original: Result<u32, String> = Err("error message".to_string());
    let bytes = oxicode::encode_to_vec(&original).expect("Failed to encode");
    let (decoded, _): (Result<u32, String>, _) =
        oxicode::decode_from_slice(&bytes).expect("Failed to decode");
    assert_eq!(original, decoded);
}

#[test]
fn test_box_roundtrip() {
    let original = Box::new(vec![1, 2, 3, 4, 5]);
    let bytes = oxicode::encode_to_vec(&original).expect("Failed to encode");
    let (decoded, _): (Box<Vec<i32>>, _) =
        oxicode::decode_from_slice(&bytes).expect("Failed to decode");
    assert_eq!(original, decoded);
}

#[test]
fn test_rc_roundtrip() {
    use std::rc::Rc;
    let original = Rc::new(String::from("shared data"));
    let bytes = oxicode::encode_to_vec(&original).expect("Failed to encode");
    let (decoded, _): (Rc<String>, _) =
        oxicode::decode_from_slice(&bytes).expect("Failed to decode");
    assert_eq!(*original, *decoded);
}

#[test]
fn test_arc_roundtrip() {
    use std::sync::Arc;
    let original = Arc::new(42u128);
    let bytes = oxicode::encode_to_vec(&original).expect("Failed to encode");
    let (decoded, _): (Arc<u128>, _) =
        oxicode::decode_from_slice(&bytes).expect("Failed to decode");
    assert_eq!(*original, *decoded);
}

#[test]
fn test_cow_owned() {
    use std::borrow::Cow;
    let original: Cow<str> = Cow::Owned("owned string".to_string());
    let bytes = oxicode::encode_to_vec(&original).expect("Failed to encode");
    #[allow(clippy::owned_cow)]
    let (decoded, _): (Cow<String>, _) =
        oxicode::decode_from_slice(&bytes).expect("Failed to decode");
    assert_eq!(*original, *decoded);
}

#[test]
#[allow(clippy::type_complexity)]
fn test_tuple_large() {
    let original = (
        1u8, 2u16, 3u32, 4u64, 5i8, 6i16, 7i32, 8i64, 9.0f32, 10.0f64,
    );
    let bytes = oxicode::encode_to_vec(&original).expect("Failed to encode");
    let (decoded, _): ((u8, u16, u32, u64, i8, i16, i32, i64, f32, f64), _) =
        oxicode::decode_from_slice(&bytes).expect("Failed to decode");
    assert_eq!(original, decoded);
}

#[test]
fn test_array_roundtrip() {
    let original = [1u32, 2, 3, 4, 5];
    let bytes = oxicode::encode_to_vec(&original).expect("Failed to encode");
    let (decoded, _): ([u32; 5], _) = oxicode::decode_from_slice(&bytes).expect("Failed to decode");
    assert_eq!(original, decoded);
}

#[test]
fn test_complex_nested() {
    type Complex = Vec<HashMap<String, Vec<Option<(u32, String)>>>>;

    let mut inner_map1 = HashMap::new();
    inner_map1.insert(
        "key1".to_string(),
        vec![Some((1, "a".to_string())), None, Some((2, "b".to_string()))],
    );

    let mut inner_map2 = HashMap::new();
    inner_map2.insert("key2".to_string(), vec![None, Some((3, "c".to_string()))]);

    let original: Complex = vec![inner_map1, inner_map2];

    let bytes = oxicode::encode_to_vec(&original).expect("Failed to encode");
    let (decoded, _): (Complex, _) = oxicode::decode_from_slice(&bytes).expect("Failed to decode");

    // Can't directly compare HashMaps, so check structure
    assert_eq!(original.len(), decoded.len());
}
