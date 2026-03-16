//! Advanced tests for alloc-specific types: BinaryHeap, Rc, Arc, and nested combinations.

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
use std::collections::{BTreeMap, BinaryHeap, HashMap, HashSet};
use std::rc::Rc;
use std::sync::Arc;

fn heap_to_sorted_vec<T: Ord>(heap: BinaryHeap<T>) -> Vec<T> {
    let mut v: Vec<T> = heap.into_iter().collect();
    v.sort();
    v
}

// ---------------------------------------------------------------------------
// 1. BinaryHeap<u32> roundtrip
// ---------------------------------------------------------------------------

#[test]
fn test_binary_heap_u32_roundtrip() {
    let original: BinaryHeap<u32> = [5u32, 3, 8, 1, 9].iter().copied().collect();
    let bytes = encode_to_vec(&original).expect("encode BinaryHeap<u32>");
    let (decoded, _): (BinaryHeap<u32>, usize) =
        decode_from_slice(&bytes).expect("decode BinaryHeap<u32>");
    assert_eq!(
        heap_to_sorted_vec(original),
        heap_to_sorted_vec(decoded),
        "BinaryHeap<u32> element sets must match after roundtrip"
    );
}

// ---------------------------------------------------------------------------
// 2. BinaryHeap<i32> with negatives roundtrip
// ---------------------------------------------------------------------------

#[test]
fn test_binary_heap_i32_negatives_roundtrip() {
    let original: BinaryHeap<i32> = [-5i32, -1, 0, 3, -100].iter().copied().collect();
    let bytes = encode_to_vec(&original).expect("encode BinaryHeap<i32>");
    let (decoded, _): (BinaryHeap<i32>, usize) =
        decode_from_slice(&bytes).expect("decode BinaryHeap<i32>");
    assert_eq!(
        heap_to_sorted_vec(original),
        heap_to_sorted_vec(decoded),
        "BinaryHeap<i32> with negatives must roundtrip correctly"
    );
}

// ---------------------------------------------------------------------------
// 3. BinaryHeap<String> roundtrip
// ---------------------------------------------------------------------------

#[test]
fn test_binary_heap_string_roundtrip() {
    let original: BinaryHeap<String> = ["cherry", "apple", "banana"]
        .iter()
        .map(|s| s.to_string())
        .collect();
    let bytes = encode_to_vec(&original).expect("encode BinaryHeap<String>");
    let (decoded, _): (BinaryHeap<String>, usize) =
        decode_from_slice(&bytes).expect("decode BinaryHeap<String>");
    assert_eq!(
        heap_to_sorted_vec(original),
        heap_to_sorted_vec(decoded),
        "BinaryHeap<String> must roundtrip correctly"
    );
}

// ---------------------------------------------------------------------------
// 4. Rc<u32> roundtrip
// ---------------------------------------------------------------------------

#[test]
fn test_rc_u32_roundtrip() {
    let original: Rc<u32> = Rc::new(42u32);
    let bytes = encode_to_vec(&original).expect("encode Rc<u32>");
    let (decoded, _): (Rc<u32>, usize) = decode_from_slice(&bytes).expect("decode Rc<u32>");
    assert_eq!(*original, *decoded, "Rc<u32> must roundtrip to same value");
}

// ---------------------------------------------------------------------------
// 5. Rc<String> roundtrip
// ---------------------------------------------------------------------------

#[test]
fn test_rc_string_roundtrip() {
    let original: Rc<String> = Rc::new("hello rc".to_string());
    let bytes = encode_to_vec(&original).expect("encode Rc<String>");
    let (decoded, _): (Rc<String>, usize) = decode_from_slice(&bytes).expect("decode Rc<String>");
    assert_eq!(
        *original, *decoded,
        "Rc<String> must roundtrip to same value"
    );
}

// ---------------------------------------------------------------------------
// 6. Rc<Vec<u8>> roundtrip
// ---------------------------------------------------------------------------

#[test]
fn test_rc_vec_u8_roundtrip() {
    let original: Rc<Vec<u8>> = Rc::new(vec![1u8, 2, 3, 255]);
    let bytes = encode_to_vec(&original).expect("encode Rc<Vec<u8>>");
    let (decoded, _): (Rc<Vec<u8>>, usize) = decode_from_slice(&bytes).expect("decode Rc<Vec<u8>>");
    assert_eq!(*original, *decoded, "Rc<Vec<u8>> must roundtrip correctly");
}

// ---------------------------------------------------------------------------
// 7. Arc<u32> roundtrip
// ---------------------------------------------------------------------------

#[test]
fn test_arc_u32_roundtrip() {
    let original: Arc<u32> = Arc::new(99u32);
    let bytes = encode_to_vec(&original).expect("encode Arc<u32>");
    let (decoded, _): (Arc<u32>, usize) = decode_from_slice(&bytes).expect("decode Arc<u32>");
    assert_eq!(*original, *decoded, "Arc<u32> must roundtrip to same value");
}

// ---------------------------------------------------------------------------
// 8. Arc<String> roundtrip
// ---------------------------------------------------------------------------

#[test]
fn test_arc_string_roundtrip() {
    let original: Arc<String> = Arc::new("arc test".to_string());
    let bytes = encode_to_vec(&original).expect("encode Arc<String>");
    let (decoded, _): (Arc<String>, usize) = decode_from_slice(&bytes).expect("decode Arc<String>");
    assert_eq!(*original, *decoded, "Arc<String> must roundtrip correctly");
}

// ---------------------------------------------------------------------------
// 9. Arc<Vec<u8>> roundtrip
// ---------------------------------------------------------------------------

#[test]
fn test_arc_vec_u8_roundtrip() {
    let original: Arc<Vec<u8>> = Arc::new(vec![10u8, 20, 30]);
    let bytes = encode_to_vec(&original).expect("encode Arc<Vec<u8>>");
    let (decoded, _): (Arc<Vec<u8>>, usize) =
        decode_from_slice(&bytes).expect("decode Arc<Vec<u8>>");
    assert_eq!(*original, *decoded, "Arc<Vec<u8>> must roundtrip correctly");
}

// ---------------------------------------------------------------------------
// 10. Arc<Arc<u32>> nested Arc roundtrip
// ---------------------------------------------------------------------------

#[test]
fn test_arc_arc_u32_roundtrip() {
    let original: Arc<Arc<u32>> = Arc::new(Arc::new(777u32));
    let bytes = encode_to_vec(&original).expect("encode Arc<Arc<u32>>");
    let (decoded, _): (Arc<Arc<u32>>, usize) =
        decode_from_slice(&bytes).expect("decode Arc<Arc<u32>>");
    assert_eq!(
        **original, **decoded,
        "nested Arc<Arc<u32>> must roundtrip correctly"
    );
}

// ---------------------------------------------------------------------------
// 11. Vec<Rc<u32>> roundtrip
// ---------------------------------------------------------------------------

#[test]
fn test_vec_rc_u32_roundtrip() {
    let original: Vec<Rc<u32>> = vec![Rc::new(1u32), Rc::new(2u32), Rc::new(3u32)];
    let bytes = encode_to_vec(&original).expect("encode Vec<Rc<u32>>");
    let (decoded, _): (Vec<Rc<u32>>, usize) =
        decode_from_slice(&bytes).expect("decode Vec<Rc<u32>>");
    assert_eq!(original.len(), decoded.len(), "lengths must match");
    for (orig, dec) in original.iter().zip(decoded.iter()) {
        assert_eq!(*orig, *dec, "each Rc<u32> element must match");
    }
}

// ---------------------------------------------------------------------------
// 12. Vec<Arc<String>> roundtrip
// ---------------------------------------------------------------------------

#[test]
fn test_vec_arc_string_roundtrip() {
    let original: Vec<Arc<String>> = vec![
        Arc::new("a".to_string()),
        Arc::new("b".to_string()),
        Arc::new("c".to_string()),
    ];
    let bytes = encode_to_vec(&original).expect("encode Vec<Arc<String>>");
    let (decoded, _): (Vec<Arc<String>>, usize) =
        decode_from_slice(&bytes).expect("decode Vec<Arc<String>>");
    assert_eq!(original.len(), decoded.len(), "lengths must match");
    for (orig, dec) in original.iter().zip(decoded.iter()) {
        assert_eq!(*orig, *dec, "each Arc<String> element must match");
    }
}

// ---------------------------------------------------------------------------
// 13. HashMap<String, Arc<u32>> roundtrip
// ---------------------------------------------------------------------------

#[test]
fn test_hashmap_string_arc_u32_roundtrip() {
    let mut original: HashMap<String, Arc<u32>> = HashMap::new();
    original.insert("alpha".to_string(), Arc::new(10u32));
    original.insert("beta".to_string(), Arc::new(20u32));
    original.insert("gamma".to_string(), Arc::new(30u32));

    let bytes = encode_to_vec(&original).expect("encode HashMap<String, Arc<u32>>");
    let (decoded, _): (HashMap<String, Arc<u32>>, usize) =
        decode_from_slice(&bytes).expect("decode HashMap<String, Arc<u32>>");

    assert_eq!(original.len(), decoded.len(), "map lengths must match");
    for (key, orig_val) in &original {
        let dec_val = decoded.get(key).expect("key must exist in decoded map");
        assert_eq!(
            *orig_val, *dec_val,
            "Arc<u32> values must match for key={key}"
        );
    }
}

// ---------------------------------------------------------------------------
// 14. BinaryHeap<u64> empty roundtrip
// ---------------------------------------------------------------------------

#[test]
fn test_binary_heap_u64_empty_roundtrip() {
    let original: BinaryHeap<u64> = BinaryHeap::new();
    let bytes = encode_to_vec(&original).expect("encode empty BinaryHeap<u64>");
    let (decoded, _): (BinaryHeap<u64>, usize) =
        decode_from_slice(&bytes).expect("decode empty BinaryHeap<u64>");
    assert!(
        decoded.is_empty(),
        "decoded empty BinaryHeap<u64> must be empty"
    );
}

// ---------------------------------------------------------------------------
// 15. BinaryHeap<u32> with 100 elements roundtrip
// ---------------------------------------------------------------------------

#[test]
fn test_binary_heap_u32_100_elements_roundtrip() {
    let original: BinaryHeap<u32> = (0u32..100).collect();
    let bytes = encode_to_vec(&original).expect("encode 100-element BinaryHeap<u32>");
    let (decoded, _): (BinaryHeap<u32>, usize) =
        decode_from_slice(&bytes).expect("decode 100-element BinaryHeap<u32>");
    assert_eq!(original.len(), decoded.len(), "lengths must match");
    assert_eq!(
        heap_to_sorted_vec(original),
        heap_to_sorted_vec(decoded),
        "100-element BinaryHeap<u32> must roundtrip correctly"
    );
}

// ---------------------------------------------------------------------------
// 16. Rc<Option<String>> Some roundtrip
// ---------------------------------------------------------------------------

#[test]
fn test_rc_option_string_some_roundtrip() {
    let original: Rc<Option<String>> = Rc::new(Some("inner".to_string()));
    let bytes = encode_to_vec(&original).expect("encode Rc<Option<String>> Some");
    let (decoded, _): (Rc<Option<String>>, usize) =
        decode_from_slice(&bytes).expect("decode Rc<Option<String>> Some");
    assert_eq!(
        *original, *decoded,
        "Rc<Option<String>> Some must roundtrip correctly"
    );
}

// ---------------------------------------------------------------------------
// 17. Rc<Option<String>> None roundtrip
// ---------------------------------------------------------------------------

#[test]
fn test_rc_option_string_none_roundtrip() {
    let original: Rc<Option<String>> = Rc::new(None);
    let bytes = encode_to_vec(&original).expect("encode Rc<Option<String>> None");
    let (decoded, _): (Rc<Option<String>>, usize) =
        decode_from_slice(&bytes).expect("decode Rc<Option<String>> None");
    assert_eq!(
        *original, *decoded,
        "Rc<Option<String>> None must roundtrip correctly"
    );
}

// ---------------------------------------------------------------------------
// 18. Arc<Option<Vec<u8>>> Some roundtrip
// ---------------------------------------------------------------------------

#[test]
fn test_arc_option_vec_u8_roundtrip() {
    let original: Arc<Option<Vec<u8>>> = Arc::new(Some(vec![1u8, 2, 3]));
    let bytes = encode_to_vec(&original).expect("encode Arc<Option<Vec<u8>>>");
    let (decoded, _): (Arc<Option<Vec<u8>>>, usize) =
        decode_from_slice(&bytes).expect("decode Arc<Option<Vec<u8>>>");
    assert_eq!(
        *original, *decoded,
        "Arc<Option<Vec<u8>>> must roundtrip correctly"
    );
}

// ---------------------------------------------------------------------------
// 19. Rc<BTreeMap<String, u32>> roundtrip
// ---------------------------------------------------------------------------

#[test]
fn test_rc_btreemap_roundtrip() {
    let mut map: BTreeMap<String, u32> = BTreeMap::new();
    map.insert("one".to_string(), 1u32);
    map.insert("two".to_string(), 2u32);
    map.insert("three".to_string(), 3u32);
    let original: Rc<BTreeMap<String, u32>> = Rc::new(map);

    let bytes = encode_to_vec(&original).expect("encode Rc<BTreeMap<String, u32>>");
    let (decoded, _): (Rc<BTreeMap<String, u32>>, usize) =
        decode_from_slice(&bytes).expect("decode Rc<BTreeMap<String, u32>>");
    assert_eq!(
        *original, *decoded,
        "Rc<BTreeMap<String, u32>> must roundtrip correctly"
    );
}

// ---------------------------------------------------------------------------
// 20. Arc<HashSet<u64>> roundtrip
// ---------------------------------------------------------------------------

#[test]
fn test_arc_hashset_u64_roundtrip() {
    let set: HashSet<u64> = [1u64, 2, 3, 4, 5].iter().copied().collect();
    let original: Arc<HashSet<u64>> = Arc::new(set);

    let bytes = encode_to_vec(&original).expect("encode Arc<HashSet<u64>>");
    let (decoded, _): (Arc<HashSet<u64>>, usize) =
        decode_from_slice(&bytes).expect("decode Arc<HashSet<u64>>");
    assert_eq!(
        *original, *decoded,
        "Arc<HashSet<u64>> must roundtrip correctly"
    );
}

// ---------------------------------------------------------------------------
// 21. BinaryHeap<u32> with fixed int encoding
// ---------------------------------------------------------------------------

#[test]
fn test_binary_heap_u32_fixed_int_encoding() {
    let original: BinaryHeap<u32> = [10u32, 20, 30].iter().copied().collect();
    let cfg = config::standard().with_fixed_int_encoding();
    let bytes =
        encode_to_vec_with_config(&original, cfg).expect("encode BinaryHeap<u32> fixed int");
    let (decoded, _): (BinaryHeap<u32>, usize) =
        decode_from_slice_with_config(&bytes, cfg).expect("decode BinaryHeap<u32> fixed int");
    assert_eq!(
        heap_to_sorted_vec(original),
        heap_to_sorted_vec(decoded),
        "BinaryHeap<u32> with fixed int encoding must roundtrip correctly"
    );
}

// ---------------------------------------------------------------------------
// 22. Arc<Vec<Arc<String>>> deeply nested roundtrip
// ---------------------------------------------------------------------------

#[test]
fn test_arc_vec_arc_string_nested_roundtrip() {
    let original: Arc<Vec<Arc<String>>> = Arc::new(vec![
        Arc::new("hello".to_string()),
        Arc::new("world".to_string()),
    ]);
    let bytes = encode_to_vec(&original).expect("encode Arc<Vec<Arc<String>>>");
    let (decoded, _): (Arc<Vec<Arc<String>>>, usize) =
        decode_from_slice(&bytes).expect("decode Arc<Vec<Arc<String>>>");

    assert_eq!(
        original.len(),
        decoded.len(),
        "inner Vec lengths must match"
    );
    for (orig, dec) in original.iter().zip(decoded.iter()) {
        assert_eq!(
            *orig, *dec,
            "each nested Arc<String> must match after roundtrip"
        );
    }
}
