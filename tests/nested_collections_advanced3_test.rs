//! Advanced tests for deeply nested collection type encoding/decoding in OxiCode.

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
use std::collections::{BTreeMap, HashMap};

use oxicode::{decode_from_slice, encode_to_vec, Decode, Encode};

// ---------------------------------------------------------------------------
// Struct used in test 20
// ---------------------------------------------------------------------------
#[derive(Debug, PartialEq, Encode, Decode)]
struct NestedCollStruct {
    matrix: Vec<Vec<u32>>,
    index: std::collections::HashMap<String, u32>,
    depth: u8,
}

// ---------------------------------------------------------------------------
// 1. Vec<Vec<u32>> roundtrip
// ---------------------------------------------------------------------------
#[test]
fn test_vec_vec_u32_roundtrip() {
    let original: Vec<Vec<u32>> =
        vec![vec![1, 2, 3], vec![4, 5, 6, 7], vec![], vec![100, 200, 300]];
    let enc = encode_to_vec(&original).expect("encode Vec<Vec<u32>>");
    let (val, _): (Vec<Vec<u32>>, _) = decode_from_slice(&enc).expect("decode Vec<Vec<u32>>");
    assert_eq!(original, val);
}

// ---------------------------------------------------------------------------
// 2. Vec<Vec<String>> roundtrip
// ---------------------------------------------------------------------------
#[test]
fn test_vec_vec_string_roundtrip() {
    let original: Vec<Vec<String>> = vec![
        vec!["hello".to_string(), "world".to_string()],
        vec!["foo".to_string()],
        vec![],
        vec!["a".to_string(), "b".to_string(), "c".to_string()],
    ];
    let enc = encode_to_vec(&original).expect("encode Vec<Vec<String>>");
    let (val, _): (Vec<Vec<String>>, _) = decode_from_slice(&enc).expect("decode Vec<Vec<String>>");
    assert_eq!(original, val);
}

// ---------------------------------------------------------------------------
// 3. Vec<Vec<Vec<u8>>> triple nesting
// ---------------------------------------------------------------------------
#[test]
fn test_vec_vec_vec_u8_roundtrip() {
    let original: Vec<Vec<Vec<u8>>> = vec![
        vec![vec![1, 2], vec![3, 4, 5]],
        vec![vec![], vec![0xFF]],
        vec![],
        vec![vec![10, 20, 30]],
    ];
    let enc = encode_to_vec(&original).expect("encode Vec<Vec<Vec<u8>>>");
    let (val, _): (Vec<Vec<Vec<u8>>>, _) =
        decode_from_slice(&enc).expect("decode Vec<Vec<Vec<u8>>>");
    assert_eq!(original, val);
}

// ---------------------------------------------------------------------------
// 4. HashMap<String, Vec<u32>> roundtrip
// ---------------------------------------------------------------------------
#[test]
fn test_hashmap_string_vec_u32_roundtrip() {
    let mut original: HashMap<String, Vec<u32>> = HashMap::new();
    original.insert("primes".to_string(), vec![2, 3, 5, 7, 11]);
    original.insert("empty".to_string(), vec![]);
    original.insert("single".to_string(), vec![42]);
    let enc = encode_to_vec(&original).expect("encode HashMap<String, Vec<u32>>");
    let (val, _): (HashMap<String, Vec<u32>>, _) =
        decode_from_slice(&enc).expect("decode HashMap<String, Vec<u32>>");
    assert_eq!(original, val);
}

// ---------------------------------------------------------------------------
// 5. HashMap<String, Vec<String>> roundtrip
// ---------------------------------------------------------------------------
#[test]
fn test_hashmap_string_vec_string_roundtrip() {
    let mut original: HashMap<String, Vec<String>> = HashMap::new();
    original.insert(
        "fruits".to_string(),
        vec!["apple".to_string(), "banana".to_string()],
    );
    original.insert("veggies".to_string(), vec!["carrot".to_string()]);
    original.insert("empty".to_string(), vec![]);
    let enc = encode_to_vec(&original).expect("encode HashMap<String, Vec<String>>");
    let (val, _): (HashMap<String, Vec<String>>, _) =
        decode_from_slice(&enc).expect("decode HashMap<String, Vec<String>>");
    assert_eq!(original, val);
}

// ---------------------------------------------------------------------------
// 6. Vec<HashMap<String, u32>> roundtrip
// ---------------------------------------------------------------------------
#[test]
fn test_vec_hashmap_string_u32_roundtrip() {
    let mut m1: HashMap<String, u32> = HashMap::new();
    m1.insert("x".to_string(), 1);
    m1.insert("y".to_string(), 2);
    let m2: HashMap<String, u32> = HashMap::new();
    let mut m3: HashMap<String, u32> = HashMap::new();
    m3.insert("z".to_string(), 99);
    let original: Vec<HashMap<String, u32>> = vec![m1, m2, m3];
    let enc = encode_to_vec(&original).expect("encode Vec<HashMap<String, u32>>");
    let (val, _): (Vec<HashMap<String, u32>>, _) =
        decode_from_slice(&enc).expect("decode Vec<HashMap<String, u32>>");
    assert_eq!(original, val);
}

// ---------------------------------------------------------------------------
// 7. HashMap<String, HashMap<String, u32>> nested maps
// ---------------------------------------------------------------------------
#[test]
fn test_hashmap_nested_maps_roundtrip() {
    let mut inner_a: HashMap<String, u32> = HashMap::new();
    inner_a.insert("alpha".to_string(), 10);
    inner_a.insert("beta".to_string(), 20);
    let mut inner_b: HashMap<String, u32> = HashMap::new();
    inner_b.insert("gamma".to_string(), 30);
    let mut original: HashMap<String, HashMap<String, u32>> = HashMap::new();
    original.insert("group_a".to_string(), inner_a);
    original.insert("group_b".to_string(), inner_b);
    original.insert("empty_group".to_string(), HashMap::new());
    let enc = encode_to_vec(&original).expect("encode HashMap<String, HashMap<String, u32>>");
    let (val, _): (HashMap<String, HashMap<String, u32>>, _) =
        decode_from_slice(&enc).expect("decode HashMap<String, HashMap<String, u32>>");
    assert_eq!(original, val);
}

// ---------------------------------------------------------------------------
// 8. Vec<Option<u32>> roundtrip with Some and None values
// ---------------------------------------------------------------------------
#[test]
fn test_vec_option_u32_roundtrip() {
    let original: Vec<Option<u32>> = vec![Some(1), None, Some(42), None, Some(0), Some(u32::MAX)];
    let enc = encode_to_vec(&original).expect("encode Vec<Option<u32>>");
    let (val, _): (Vec<Option<u32>>, _) = decode_from_slice(&enc).expect("decode Vec<Option<u32>>");
    assert_eq!(original, val);
}

// ---------------------------------------------------------------------------
// 9. Vec<Option<String>> roundtrip
// ---------------------------------------------------------------------------
#[test]
fn test_vec_option_string_roundtrip() {
    let original: Vec<Option<String>> = vec![
        Some("hello".to_string()),
        None,
        Some("world".to_string()),
        None,
        Some(String::new()),
    ];
    let enc = encode_to_vec(&original).expect("encode Vec<Option<String>>");
    let (val, _): (Vec<Option<String>>, _) =
        decode_from_slice(&enc).expect("decode Vec<Option<String>>");
    assert_eq!(original, val);
}

// ---------------------------------------------------------------------------
// 10. HashMap<String, Option<Vec<u8>>> complex nested type
// ---------------------------------------------------------------------------
#[test]
fn test_hashmap_string_option_vec_u8_roundtrip() {
    let mut original: HashMap<String, Option<Vec<u8>>> = HashMap::new();
    original.insert("data".to_string(), Some(vec![0xDE, 0xAD, 0xBE, 0xEF]));
    original.insert("absent".to_string(), None);
    original.insert("empty_vec".to_string(), Some(vec![]));
    let enc = encode_to_vec(&original).expect("encode HashMap<String, Option<Vec<u8>>>");
    let (val, _): (HashMap<String, Option<Vec<u8>>>, _) =
        decode_from_slice(&enc).expect("decode HashMap<String, Option<Vec<u8>>>");
    assert_eq!(original, val);
}

// ---------------------------------------------------------------------------
// 11. Option<Vec<Vec<u8>>> roundtrip
// ---------------------------------------------------------------------------
#[test]
fn test_option_vec_vec_u8_roundtrip() {
    let original_some: Option<Vec<Vec<u8>>> = Some(vec![vec![1, 2, 3], vec![], vec![4, 5]]);
    let enc = encode_to_vec(&original_some).expect("encode Option<Vec<Vec<u8>>> Some");
    let (val, _): (Option<Vec<Vec<u8>>>, _) =
        decode_from_slice(&enc).expect("decode Option<Vec<Vec<u8>>> Some");
    assert_eq!(original_some, val);

    let original_none: Option<Vec<Vec<u8>>> = None;
    let enc2 = encode_to_vec(&original_none).expect("encode Option<Vec<Vec<u8>>> None");
    let (val2, _): (Option<Vec<Vec<u8>>>, _) =
        decode_from_slice(&enc2).expect("decode Option<Vec<Vec<u8>>> None");
    assert_eq!(original_none, val2);
}

// ---------------------------------------------------------------------------
// 12. Vec<(u32, String)> tuple elements in vec
// ---------------------------------------------------------------------------
#[test]
fn test_vec_tuple_u32_string_roundtrip() {
    let original: Vec<(u32, String)> = vec![
        (1, "one".to_string()),
        (2, "two".to_string()),
        (100, "hundred".to_string()),
        (0, String::new()),
    ];
    let enc = encode_to_vec(&original).expect("encode Vec<(u32, String)>");
    let (val, _): (Vec<(u32, String)>, _) =
        decode_from_slice(&enc).expect("decode Vec<(u32, String)>");
    assert_eq!(original, val);
}

// ---------------------------------------------------------------------------
// 13. HashMap<u32, Vec<(String, bool)>> complex nested
// ---------------------------------------------------------------------------
#[test]
fn test_hashmap_u32_vec_tuple_string_bool_roundtrip() {
    let mut original: HashMap<u32, Vec<(String, bool)>> = HashMap::new();
    original.insert(
        1,
        vec![("alpha".to_string(), true), ("beta".to_string(), false)],
    );
    original.insert(2, vec![("gamma".to_string(), true)]);
    original.insert(3, vec![]);
    let enc = encode_to_vec(&original).expect("encode HashMap<u32, Vec<(String, bool)>>");
    let (val, _): (HashMap<u32, Vec<(String, bool)>>, _) =
        decode_from_slice(&enc).expect("decode HashMap<u32, Vec<(String, bool)>>");
    assert_eq!(original, val);
}

// ---------------------------------------------------------------------------
// 14. BTreeMap<String, Vec<u32>> roundtrip
// ---------------------------------------------------------------------------
#[test]
fn test_btreemap_string_vec_u32_roundtrip() {
    let mut original: BTreeMap<String, Vec<u32>> = BTreeMap::new();
    original.insert("evens".to_string(), vec![2, 4, 6, 8, 10]);
    original.insert("odds".to_string(), vec![1, 3, 5, 7, 9]);
    original.insert("empty".to_string(), vec![]);
    let enc = encode_to_vec(&original).expect("encode BTreeMap<String, Vec<u32>>");
    let (val, _): (BTreeMap<String, Vec<u32>>, _) =
        decode_from_slice(&enc).expect("decode BTreeMap<String, Vec<u32>>");
    assert_eq!(original, val);
}

// ---------------------------------------------------------------------------
// 15. BTreeMap<u32, BTreeMap<u32, String>> nested btreemaps
// ---------------------------------------------------------------------------
#[test]
fn test_btreemap_nested_roundtrip() {
    let mut inner1: BTreeMap<u32, String> = BTreeMap::new();
    inner1.insert(1, "one".to_string());
    inner1.insert(2, "two".to_string());
    let mut inner2: BTreeMap<u32, String> = BTreeMap::new();
    inner2.insert(10, "ten".to_string());
    let mut original: BTreeMap<u32, BTreeMap<u32, String>> = BTreeMap::new();
    original.insert(100, inner1);
    original.insert(200, inner2);
    original.insert(300, BTreeMap::new());
    let enc = encode_to_vec(&original).expect("encode BTreeMap<u32, BTreeMap<u32, String>>");
    let (val, _): (BTreeMap<u32, BTreeMap<u32, String>>, _) =
        decode_from_slice(&enc).expect("decode BTreeMap<u32, BTreeMap<u32, String>>");
    assert_eq!(original, val);
}

// ---------------------------------------------------------------------------
// 16. Vec<BTreeMap<String, u32>> vec of maps
// ---------------------------------------------------------------------------
#[test]
fn test_vec_btreemap_string_u32_roundtrip() {
    let mut m1: BTreeMap<String, u32> = BTreeMap::new();
    m1.insert("a".to_string(), 1);
    m1.insert("b".to_string(), 2);
    let m2: BTreeMap<String, u32> = BTreeMap::new();
    let mut m3: BTreeMap<String, u32> = BTreeMap::new();
    m3.insert("z".to_string(), 26);
    let original: Vec<BTreeMap<String, u32>> = vec![m1, m2, m3];
    let enc = encode_to_vec(&original).expect("encode Vec<BTreeMap<String, u32>>");
    let (val, _): (Vec<BTreeMap<String, u32>>, _) =
        decode_from_slice(&enc).expect("decode Vec<BTreeMap<String, u32>>");
    assert_eq!(original, val);
}

// ---------------------------------------------------------------------------
// 17. Option<HashMap<String, Vec<u32>>> deeply nested optional
// ---------------------------------------------------------------------------
#[test]
fn test_option_hashmap_string_vec_u32_roundtrip() {
    let mut inner: HashMap<String, Vec<u32>> = HashMap::new();
    inner.insert("nums".to_string(), vec![1, 2, 3]);
    inner.insert("more".to_string(), vec![]);

    let original_some: Option<HashMap<String, Vec<u32>>> = Some(inner);
    let enc = encode_to_vec(&original_some).expect("encode Option<HashMap<String, Vec<u32>>> Some");
    let (val, _): (Option<HashMap<String, Vec<u32>>>, _) =
        decode_from_slice(&enc).expect("decode Option<HashMap<String, Vec<u32>>> Some");
    assert_eq!(original_some, val);

    let original_none: Option<HashMap<String, Vec<u32>>> = None;
    let enc2 =
        encode_to_vec(&original_none).expect("encode Option<HashMap<String, Vec<u32>>> None");
    let (val2, _): (Option<HashMap<String, Vec<u32>>>, _) =
        decode_from_slice(&enc2).expect("decode Option<HashMap<String, Vec<u32>>> None");
    assert_eq!(original_none, val2);
}

// ---------------------------------------------------------------------------
// 18. Vec<Vec<Option<u32>>> 3-level nesting with options
// ---------------------------------------------------------------------------
#[test]
fn test_vec_vec_option_u32_roundtrip() {
    let original: Vec<Vec<Option<u32>>> = vec![
        vec![Some(1), None, Some(3)],
        vec![None, None],
        vec![],
        vec![Some(u32::MAX), Some(0)],
    ];
    let enc = encode_to_vec(&original).expect("encode Vec<Vec<Option<u32>>>");
    let (val, _): (Vec<Vec<Option<u32>>>, _) =
        decode_from_slice(&enc).expect("decode Vec<Vec<Option<u32>>>");
    assert_eq!(original, val);
}

// ---------------------------------------------------------------------------
// 19. HashMap<String, Vec<Option<String>>> mixed nesting
// ---------------------------------------------------------------------------
#[test]
fn test_hashmap_string_vec_option_string_roundtrip() {
    let mut original: HashMap<String, Vec<Option<String>>> = HashMap::new();
    original.insert(
        "mixed".to_string(),
        vec![Some("hello".to_string()), None, Some("world".to_string())],
    );
    original.insert("all_none".to_string(), vec![None, None, None]);
    original.insert("empty".to_string(), vec![]);
    let enc = encode_to_vec(&original).expect("encode HashMap<String, Vec<Option<String>>>");
    let (val, _): (HashMap<String, Vec<Option<String>>>, _) =
        decode_from_slice(&enc).expect("decode HashMap<String, Vec<Option<String>>>");
    assert_eq!(original, val);
}

// ---------------------------------------------------------------------------
// 20. Struct containing Vec<Vec<u32>> and HashMap<String, u32>
// ---------------------------------------------------------------------------
#[test]
fn test_nested_coll_struct_roundtrip() {
    let mut index: HashMap<String, u32> = HashMap::new();
    index.insert("first".to_string(), 0);
    index.insert("second".to_string(), 1);
    let original = NestedCollStruct {
        matrix: vec![vec![1, 2, 3], vec![4, 5], vec![]],
        index,
        depth: 3,
    };
    let enc = encode_to_vec(&original).expect("encode NestedCollStruct");
    let (val, _): (NestedCollStruct, _) = decode_from_slice(&enc).expect("decode NestedCollStruct");
    assert_eq!(original, val);
}

// ---------------------------------------------------------------------------
// 21. Empty nested collections - Vec<Vec<u32>> where outer is empty
// ---------------------------------------------------------------------------
#[test]
fn test_empty_outer_vec_vec_u32_roundtrip() {
    let original: Vec<Vec<u32>> = vec![];
    let enc = encode_to_vec(&original).expect("encode empty Vec<Vec<u32>>");
    let (val, _): (Vec<Vec<u32>>, _) = decode_from_slice(&enc).expect("decode empty Vec<Vec<u32>>");
    assert_eq!(original, val);
    assert!(val.is_empty());
}

// ---------------------------------------------------------------------------
// 22. Empty nested collections - HashMap<String, Vec<u32>> where values are empty vecs
// ---------------------------------------------------------------------------
#[test]
fn test_hashmap_empty_vec_values_roundtrip() {
    let mut original: HashMap<String, Vec<u32>> = HashMap::new();
    original.insert("first".to_string(), vec![]);
    original.insert("second".to_string(), vec![]);
    original.insert("third".to_string(), vec![]);
    let enc = encode_to_vec(&original).expect("encode HashMap<String, Vec<u32>> with empty vecs");
    let (val, _): (HashMap<String, Vec<u32>>, _) =
        decode_from_slice(&enc).expect("decode HashMap<String, Vec<u32>> with empty vecs");
    assert_eq!(original, val);
    for (_, v) in &val {
        assert!(v.is_empty(), "all values should be empty vecs");
    }
}
