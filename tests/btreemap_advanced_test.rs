//! Advanced tests for BTreeMap encode/decode roundtrips

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
use std::collections::BTreeMap;

#[test]
fn test_btreemap_string_vec_roundtrip() {
    let mut original: BTreeMap<String, Vec<u32>> = BTreeMap::new();
    original.insert("fibonacci".to_string(), vec![1, 1, 2, 3, 5, 8, 13]);
    original.insert("primes".to_string(), vec![2, 3, 5, 7, 11, 13]);
    original.insert("empty".to_string(), vec![]);
    original.insert("single".to_string(), vec![42]);

    let bytes = oxicode::encode_to_vec(&original).expect("encode failed");
    let (decoded, _): (BTreeMap<String, Vec<u32>>, _) =
        oxicode::decode_from_slice(&bytes).expect("decode failed");

    assert_eq!(
        original, decoded,
        "BTreeMap<String, Vec<u32>> must roundtrip exactly"
    );
}

#[test]
fn test_btreemap_nested_roundtrip() {
    // BTreeMap<u64, BTreeMap<u32, String>>
    let mut outer: BTreeMap<u64, BTreeMap<u32, String>> = BTreeMap::new();

    let mut inner1: BTreeMap<u32, String> = BTreeMap::new();
    inner1.insert(1, "one".to_string());
    inner1.insert(2, "two".to_string());
    inner1.insert(3, "three".to_string());

    let mut inner2: BTreeMap<u32, String> = BTreeMap::new();
    inner2.insert(10, "ten".to_string());
    inner2.insert(20, "twenty".to_string());

    let inner_empty: BTreeMap<u32, String> = BTreeMap::new();

    outer.insert(100u64, inner1);
    outer.insert(200u64, inner2);
    outer.insert(300u64, inner_empty);

    let bytes = oxicode::encode_to_vec(&outer).expect("encode failed");
    let (decoded, _): (BTreeMap<u64, BTreeMap<u32, String>>, _) =
        oxicode::decode_from_slice(&bytes).expect("decode failed");

    assert_eq!(outer, decoded, "nested BTreeMap must roundtrip exactly");
}

#[test]
fn test_btreemap_sorted_iteration_preserved() {
    // BTreeMap always iterates in sorted key order; verify this is preserved post-decode
    let mut original: BTreeMap<i32, String> = BTreeMap::new();
    // Insert in non-sorted order deliberately
    for &k in &[50, 10, 90, 30, 70, 20, 80, 40, 60, 100, -5, 0] {
        original.insert(k, format!("value_{}", k));
    }

    let bytes = oxicode::encode_to_vec(&original).expect("encode failed");
    let (decoded, _): (BTreeMap<i32, String>, _) =
        oxicode::decode_from_slice(&bytes).expect("decode failed");

    // Collect keys in iteration order from both maps
    let original_keys: Vec<i32> = original.keys().copied().collect();
    let decoded_keys: Vec<i32> = decoded.keys().copied().collect();

    assert_eq!(
        original_keys, decoded_keys,
        "key iteration order must be sorted and identical after roundtrip"
    );
    assert_eq!(original, decoded, "full map content must be identical");
}

#[test]
fn test_btreemap_large_roundtrip() {
    // 500 entries
    let mut original: BTreeMap<u32, String> = BTreeMap::new();
    for i in 0u32..500 {
        original.insert(i, format!("entry_{:04}", i));
    }

    let bytes = oxicode::encode_to_vec(&original).expect("encode failed");
    let (decoded, _): (BTreeMap<u32, String>, _) =
        oxicode::decode_from_slice(&bytes).expect("decode failed");

    assert_eq!(original.len(), decoded.len(), "lengths must match");
    assert_eq!(
        original, decoded,
        "500-entry BTreeMap must roundtrip exactly"
    );

    // Spot-check a few entries
    assert_eq!(decoded.get(&0), Some(&"entry_0000".to_string()));
    assert_eq!(decoded.get(&249), Some(&"entry_0249".to_string()));
    assert_eq!(decoded.get(&499), Some(&"entry_0499".to_string()));
}
