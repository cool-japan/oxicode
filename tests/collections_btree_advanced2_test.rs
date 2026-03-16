//! Advanced tests for BTreeMap and BTreeSet encoding in OxiCode — set 2.

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
use std::collections::{BTreeMap, BTreeSet};

use oxicode::{config, Decode, Encode};

// ---------------------------------------------------------------------------
// 1. BTreeMap<u32, String> empty roundtrip
// ---------------------------------------------------------------------------
#[test]
fn test_btreemap_u32_string_empty_roundtrip() {
    let original: BTreeMap<u32, String> = BTreeMap::new();
    let bytes = oxicode::encode_to_vec(&original).expect("encode failed");
    let (decoded, _bytes): (BTreeMap<u32, String>, usize) =
        oxicode::decode_from_slice(&bytes).expect("decode failed");
    assert_eq!(original, decoded);
}

// ---------------------------------------------------------------------------
// 2. BTreeMap<u32, String> single entry roundtrip
// ---------------------------------------------------------------------------
#[test]
fn test_btreemap_u32_string_single_entry_roundtrip() {
    let mut original: BTreeMap<u32, String> = BTreeMap::new();
    original.insert(42, "hello".to_string());
    let bytes = oxicode::encode_to_vec(&original).expect("encode failed");
    let (decoded, _bytes): (BTreeMap<u32, String>, usize) =
        oxicode::decode_from_slice(&bytes).expect("decode failed");
    assert_eq!(original, decoded);
    assert_eq!(decoded.get(&42), Some(&"hello".to_string()));
}

// ---------------------------------------------------------------------------
// 3. BTreeMap<String, Vec<u8>> roundtrip
// ---------------------------------------------------------------------------
#[test]
fn test_btreemap_string_vecu8_roundtrip() {
    let mut original: BTreeMap<String, Vec<u8>> = BTreeMap::new();
    original.insert("alpha".to_string(), vec![1, 2, 3]);
    original.insert("beta".to_string(), vec![]);
    original.insert("gamma".to_string(), vec![255, 0, 128]);
    let bytes = oxicode::encode_to_vec(&original).expect("encode failed");
    let (decoded, _bytes): (BTreeMap<String, Vec<u8>>, usize) =
        oxicode::decode_from_slice(&bytes).expect("decode failed");
    assert_eq!(original, decoded);
}

// ---------------------------------------------------------------------------
// 4. BTreeMap<u8, u8> 10 entries roundtrip
// ---------------------------------------------------------------------------
#[test]
fn test_btreemap_u8_u8_10_entries_roundtrip() {
    let original: BTreeMap<u8, u8> = (0u8..10).map(|i| (i, i * 2)).collect();
    let bytes = oxicode::encode_to_vec(&original).expect("encode failed");
    let (decoded, _bytes): (BTreeMap<u8, u8>, usize) =
        oxicode::decode_from_slice(&bytes).expect("decode failed");
    assert_eq!(original, decoded);
    assert_eq!(decoded.len(), 10);
}

// ---------------------------------------------------------------------------
// 5. BTreeMap sorted order preserved after roundtrip
// ---------------------------------------------------------------------------
#[test]
fn test_btreemap_sorted_order_preserved() {
    let mut original: BTreeMap<u32, String> = BTreeMap::new();
    for &k in &[50u32, 10, 90, 30, 70, 20, 80, 40, 60] {
        original.insert(k, format!("v{}", k));
    }
    let bytes = oxicode::encode_to_vec(&original).expect("encode failed");
    let (decoded, _bytes): (BTreeMap<u32, String>, usize) =
        oxicode::decode_from_slice(&bytes).expect("decode failed");
    let original_keys: Vec<u32> = original.keys().copied().collect();
    let decoded_keys: Vec<u32> = decoded.keys().copied().collect();
    assert_eq!(
        original_keys, decoded_keys,
        "sorted key order must be preserved"
    );
    assert_eq!(original, decoded);
}

// ---------------------------------------------------------------------------
// 6. BTreeSet<u32> empty roundtrip
// ---------------------------------------------------------------------------
#[test]
fn test_btreeset_u32_empty_roundtrip() {
    let original: BTreeSet<u32> = BTreeSet::new();
    let bytes = oxicode::encode_to_vec(&original).expect("encode failed");
    let (decoded, _bytes): (BTreeSet<u32>, usize) =
        oxicode::decode_from_slice(&bytes).expect("decode failed");
    assert_eq!(original, decoded);
    assert!(decoded.is_empty());
}

// ---------------------------------------------------------------------------
// 7. BTreeSet<u32> single element roundtrip
// ---------------------------------------------------------------------------
#[test]
fn test_btreeset_u32_single_element_roundtrip() {
    let mut original: BTreeSet<u32> = BTreeSet::new();
    original.insert(7);
    let bytes = oxicode::encode_to_vec(&original).expect("encode failed");
    let (decoded, _bytes): (BTreeSet<u32>, usize) =
        oxicode::decode_from_slice(&bytes).expect("decode failed");
    assert_eq!(original, decoded);
    assert!(decoded.contains(&7));
}

// ---------------------------------------------------------------------------
// 8. BTreeSet<String> roundtrip
// ---------------------------------------------------------------------------
#[test]
fn test_btreeset_string_roundtrip() {
    let original: BTreeSet<String> = ["apple", "banana", "cherry", "date"]
        .iter()
        .map(|s| s.to_string())
        .collect();
    let bytes = oxicode::encode_to_vec(&original).expect("encode failed");
    let (decoded, _bytes): (BTreeSet<String>, usize) =
        oxicode::decode_from_slice(&bytes).expect("decode failed");
    assert_eq!(original, decoded);
    assert!(decoded.contains("apple"));
    assert!(decoded.contains("date"));
}

// ---------------------------------------------------------------------------
// 9. BTreeSet<u64> 100 elements roundtrip
// ---------------------------------------------------------------------------
#[test]
fn test_btreeset_u64_100_elements_roundtrip() {
    let original: BTreeSet<u64> = (0u64..100).collect();
    let bytes = oxicode::encode_to_vec(&original).expect("encode failed");
    let (decoded, _bytes): (BTreeSet<u64>, usize) =
        oxicode::decode_from_slice(&bytes).expect("decode failed");
    assert_eq!(original, decoded);
    assert_eq!(decoded.len(), 100);
    assert!(decoded.contains(&0));
    assert!(decoded.contains(&99));
}

// ---------------------------------------------------------------------------
// 10. BTreeSet sorted order preserved after roundtrip
// ---------------------------------------------------------------------------
#[test]
fn test_btreeset_sorted_order_preserved() {
    let original: BTreeSet<u32> = [50u32, 10, 90, 30, 70, 20, 80, 40, 60]
        .iter()
        .copied()
        .collect();
    let bytes = oxicode::encode_to_vec(&original).expect("encode failed");
    let (decoded, _bytes): (BTreeSet<u32>, usize) =
        oxicode::decode_from_slice(&bytes).expect("decode failed");
    let original_elems: Vec<u32> = original.iter().copied().collect();
    let decoded_elems: Vec<u32> = decoded.iter().copied().collect();
    assert_eq!(
        original_elems, decoded_elems,
        "sorted element order must be preserved"
    );
}

// ---------------------------------------------------------------------------
// 11. BTreeMap<u32, BTreeSet<String>> nested roundtrip
// ---------------------------------------------------------------------------
#[test]
fn test_btreemap_u32_btreeset_string_nested_roundtrip() {
    let mut original: BTreeMap<u32, BTreeSet<String>> = BTreeMap::new();
    let set_a: BTreeSet<String> = ["one", "two", "three"]
        .iter()
        .map(|s| s.to_string())
        .collect();
    let set_b: BTreeSet<String> = ["four", "five"].iter().map(|s| s.to_string()).collect();
    original.insert(1, set_a);
    original.insert(2, set_b);
    original.insert(3, BTreeSet::new());
    let bytes = oxicode::encode_to_vec(&original).expect("encode failed");
    let (decoded, _bytes): (BTreeMap<u32, BTreeSet<String>>, usize) =
        oxicode::decode_from_slice(&bytes).expect("decode failed");
    assert_eq!(original, decoded);
}

// ---------------------------------------------------------------------------
// 12. BTreeSet<Vec<u8>> roundtrip
// ---------------------------------------------------------------------------
#[test]
fn test_btreeset_vecu8_roundtrip() {
    let mut original: BTreeSet<Vec<u8>> = BTreeSet::new();
    original.insert(vec![1, 2, 3]);
    original.insert(vec![]);
    original.insert(vec![255]);
    original.insert(vec![10, 20]);
    let bytes = oxicode::encode_to_vec(&original).expect("encode failed");
    let (decoded, _bytes): (BTreeSet<Vec<u8>>, usize) =
        oxicode::decode_from_slice(&bytes).expect("decode failed");
    assert_eq!(original, decoded);
}

// ---------------------------------------------------------------------------
// 13. Vec<BTreeMap<u8, u8>> roundtrip
// ---------------------------------------------------------------------------
#[test]
fn test_vec_of_btreemaps_roundtrip() {
    let map_a: BTreeMap<u8, u8> = [(1u8, 10u8), (2, 20), (3, 30)].iter().copied().collect();
    let map_b: BTreeMap<u8, u8> = BTreeMap::new();
    let map_c: BTreeMap<u8, u8> = [(255u8, 0u8)].iter().copied().collect();
    let original: Vec<BTreeMap<u8, u8>> = vec![map_a, map_b, map_c];
    let bytes = oxicode::encode_to_vec(&original).expect("encode failed");
    let (decoded, _bytes): (Vec<BTreeMap<u8, u8>>, usize) =
        oxicode::decode_from_slice(&bytes).expect("decode failed");
    assert_eq!(original, decoded);
    assert_eq!(decoded.len(), 3);
    assert!(decoded[1].is_empty());
}

// ---------------------------------------------------------------------------
// 14. Option<BTreeMap<u32, String>> Some roundtrip
// ---------------------------------------------------------------------------
#[test]
fn test_option_btreemap_some_roundtrip() {
    let mut map: BTreeMap<u32, String> = BTreeMap::new();
    map.insert(1, "one".to_string());
    map.insert(2, "two".to_string());
    let original: Option<BTreeMap<u32, String>> = Some(map);
    let bytes = oxicode::encode_to_vec(&original).expect("encode failed");
    let (decoded, _bytes): (Option<BTreeMap<u32, String>>, usize) =
        oxicode::decode_from_slice(&bytes).expect("decode failed");
    assert_eq!(original, decoded);
    assert!(decoded.is_some());
}

// ---------------------------------------------------------------------------
// 15. Option<BTreeSet<u32>> None roundtrip
// ---------------------------------------------------------------------------
#[test]
fn test_option_btreeset_none_roundtrip() {
    let original: Option<BTreeSet<u32>> = None;
    let bytes = oxicode::encode_to_vec(&original).expect("encode failed");
    let (decoded, _bytes): (Option<BTreeSet<u32>>, usize) =
        oxicode::decode_from_slice(&bytes).expect("decode failed");
    assert_eq!(original, decoded);
    assert!(decoded.is_none());
}

// ---------------------------------------------------------------------------
// 16. BTreeMap<u32, String> with fixed-int config roundtrip
// ---------------------------------------------------------------------------
#[test]
fn test_btreemap_u32_string_fixed_int_config_roundtrip() {
    let mut original: BTreeMap<u32, String> = BTreeMap::new();
    original.insert(0, "zero".to_string());
    original.insert(1000, "thousand".to_string());
    original.insert(u32::MAX, "max".to_string());
    let cfg = config::standard().with_fixed_int_encoding();
    let bytes =
        oxicode::encode_to_vec_with_config(&original, cfg).expect("encode with fixed-int failed");
    let (decoded, _bytes): (BTreeMap<u32, String>, usize) =
        oxicode::decode_from_slice_with_config(&bytes, cfg).expect("decode with fixed-int failed");
    assert_eq!(original, decoded);
}

// ---------------------------------------------------------------------------
// 17. BTreeMap<i32, i32> with negative keys roundtrip
// ---------------------------------------------------------------------------
#[test]
fn test_btreemap_negative_keys_roundtrip() {
    let original: BTreeMap<i32, i32> = [(-100i32, 1), (-1, 2), (0, 3), (1, 4), (100, 5)]
        .iter()
        .copied()
        .collect();
    let bytes = oxicode::encode_to_vec(&original).expect("encode failed");
    let (decoded, _bytes): (BTreeMap<i32, i32>, usize) =
        oxicode::decode_from_slice(&bytes).expect("decode failed");
    assert_eq!(original, decoded);
    assert_eq!(decoded.get(&-100), Some(&1));
    assert_eq!(decoded.get(&0), Some(&3));
}

// ---------------------------------------------------------------------------
// 18. Struct { map: BTreeMap<u32, String>, set: BTreeSet<u32> } roundtrip
// ---------------------------------------------------------------------------
#[derive(Debug, PartialEq, Encode, Decode)]
struct MapAndSet {
    map: BTreeMap<u32, String>,
    set: BTreeSet<u32>,
}

#[test]
fn test_struct_with_btreemap_and_btreeset_roundtrip() {
    let mut map: BTreeMap<u32, String> = BTreeMap::new();
    map.insert(1, "a".to_string());
    map.insert(2, "b".to_string());
    let set: BTreeSet<u32> = [10u32, 20, 30].iter().copied().collect();
    let original = MapAndSet { map, set };
    let bytes = oxicode::encode_to_vec(&original).expect("encode failed");
    let (decoded, _bytes): (MapAndSet, usize) =
        oxicode::decode_from_slice(&bytes).expect("decode failed");
    assert_eq!(original, decoded);
}

// ---------------------------------------------------------------------------
// 19. BTreeMap<u32, Option<String>> roundtrip
// ---------------------------------------------------------------------------
#[test]
fn test_btreemap_u32_option_string_roundtrip() {
    let mut original: BTreeMap<u32, Option<String>> = BTreeMap::new();
    original.insert(1, Some("present".to_string()));
    original.insert(2, None);
    original.insert(3, Some("also present".to_string()));
    let bytes = oxicode::encode_to_vec(&original).expect("encode failed");
    let (decoded, _bytes): (BTreeMap<u32, Option<String>>, usize) =
        oxicode::decode_from_slice(&bytes).expect("decode failed");
    assert_eq!(original, decoded);
    assert_eq!(decoded.get(&2), Some(&None));
}

// ---------------------------------------------------------------------------
// 20. BTreeSet<(u32, String)> roundtrip
// ---------------------------------------------------------------------------
#[test]
fn test_btreeset_tuple_u32_string_roundtrip() {
    let mut original: BTreeSet<(u32, String)> = BTreeSet::new();
    original.insert((1u32, "alpha".to_string()));
    original.insert((2u32, "beta".to_string()));
    original.insert((3u32, "gamma".to_string()));
    let bytes = oxicode::encode_to_vec(&original).expect("encode failed");
    let (decoded, _bytes): (BTreeSet<(u32, String)>, usize) =
        oxicode::decode_from_slice(&bytes).expect("decode failed");
    assert_eq!(original, decoded);
    assert!(decoded.contains(&(1u32, "alpha".to_string())));
}

// ---------------------------------------------------------------------------
// 21. BTreeMap with big-endian config roundtrip
// ---------------------------------------------------------------------------
#[test]
fn test_btreemap_big_endian_config_roundtrip() {
    let mut original: BTreeMap<u32, u64> = BTreeMap::new();
    original.insert(1, 1000);
    original.insert(2, 2000);
    original.insert(3, u64::MAX);
    let cfg = config::standard().with_big_endian();
    let bytes =
        oxicode::encode_to_vec_with_config(&original, cfg).expect("encode with big-endian failed");
    let (decoded, _bytes): (BTreeMap<u32, u64>, usize) =
        oxicode::decode_from_slice_with_config(&bytes, cfg).expect("decode with big-endian failed");
    assert_eq!(original, decoded);
}

// ---------------------------------------------------------------------------
// 22. BTreeMap<String, BTreeMap<u32, u8>> deeply nested roundtrip
// ---------------------------------------------------------------------------
#[test]
fn test_btreemap_deeply_nested_roundtrip() {
    let mut original: BTreeMap<String, BTreeMap<u32, u8>> = BTreeMap::new();
    let mut inner_a: BTreeMap<u32, u8> = BTreeMap::new();
    inner_a.insert(1, 10);
    inner_a.insert(2, 20);
    inner_a.insert(3, 30);
    let mut inner_b: BTreeMap<u32, u8> = BTreeMap::new();
    inner_b.insert(100, 255);
    original.insert("first".to_string(), inner_a);
    original.insert("second".to_string(), inner_b);
    original.insert("empty".to_string(), BTreeMap::new());
    let bytes = oxicode::encode_to_vec(&original).expect("encode failed");
    let (decoded, _bytes): (BTreeMap<String, BTreeMap<u32, u8>>, usize) =
        oxicode::decode_from_slice(&bytes).expect("decode failed");
    assert_eq!(original, decoded);
    assert_eq!(decoded.get("first").and_then(|m| m.get(&1)), Some(&10u8));
    assert!(decoded.get("empty").map(|m| m.is_empty()).unwrap_or(false));
}
