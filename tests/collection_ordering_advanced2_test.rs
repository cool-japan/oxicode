//! Advanced tests for collection ordering and ordering-related types in OxiCode.

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
use std::cmp::Ordering;
use std::collections::{BTreeMap, BTreeSet};

#[test]
fn test_ordering_less_roundtrip() {
    let enc = encode_to_vec(&Ordering::Less).expect("encode Ordering::Less");
    let (val, _): (Ordering, usize) = decode_from_slice(&enc).expect("decode Ordering::Less");
    assert_eq!(val, Ordering::Less);
}

#[test]
fn test_ordering_equal_roundtrip() {
    let enc = encode_to_vec(&Ordering::Equal).expect("encode Ordering::Equal");
    let (val, _): (Ordering, usize) = decode_from_slice(&enc).expect("decode Ordering::Equal");
    assert_eq!(val, Ordering::Equal);
}

#[test]
fn test_ordering_greater_roundtrip() {
    let enc = encode_to_vec(&Ordering::Greater).expect("encode Ordering::Greater");
    let (val, _): (Ordering, usize) = decode_from_slice(&enc).expect("decode Ordering::Greater");
    assert_eq!(val, Ordering::Greater);
}

#[test]
fn test_ordering_values_distinct() {
    let less_enc = encode_to_vec(&Ordering::Less).expect("encode Less");
    let equal_enc = encode_to_vec(&Ordering::Equal).expect("encode Equal");
    let greater_enc = encode_to_vec(&Ordering::Greater).expect("encode Greater");

    assert_ne!(
        less_enc, equal_enc,
        "Less and Equal must have distinct encodings"
    );
    assert_ne!(
        equal_enc, greater_enc,
        "Equal and Greater must have distinct encodings"
    );
    assert_ne!(
        less_enc, greater_enc,
        "Less and Greater must have distinct encodings"
    );
}

#[test]
fn test_vec_ordering_roundtrip() {
    let original: Vec<Ordering> = vec![
        Ordering::Less,
        Ordering::Equal,
        Ordering::Greater,
        Ordering::Less,
    ];
    let enc = encode_to_vec(&original).expect("encode Vec<Ordering>");
    let (val, _): (Vec<Ordering>, usize) = decode_from_slice(&enc).expect("decode Vec<Ordering>");
    assert_eq!(val, original);
    assert_eq!(val.len(), 4);
}

#[test]
fn test_option_ordering_some_roundtrip() {
    let original: Option<Ordering> = Some(Ordering::Less);
    let enc = encode_to_vec(&original).expect("encode Option<Ordering> Some");
    let (val, _): (Option<Ordering>, usize) =
        decode_from_slice(&enc).expect("decode Option<Ordering> Some");
    assert_eq!(val, Some(Ordering::Less));
}

#[test]
fn test_option_ordering_none_roundtrip() {
    let original: Option<Ordering> = None;
    let enc = encode_to_vec(&original).expect("encode Option<Ordering> None");
    let (val, _): (Option<Ordering>, usize) =
        decode_from_slice(&enc).expect("decode Option<Ordering> None");
    assert_eq!(val, None);
}

#[test]
fn test_btreeset_i32_roundtrip() {
    let original: BTreeSet<i32> = vec![-100, -42, -1, 0, 1, 7, 42, 100, 999]
        .into_iter()
        .collect();
    let enc = encode_to_vec(&original).expect("encode BTreeSet<i32>");
    let (val, _): (BTreeSet<i32>, usize) = decode_from_slice(&enc).expect("decode BTreeSet<i32>");
    assert_eq!(val, original);
    assert_eq!(val.len(), 9);
}

#[test]
fn test_btreeset_string_roundtrip() {
    let original: BTreeSet<String> = vec![
        "zebra".to_string(),
        "apple".to_string(),
        "mango".to_string(),
        "cherry".to_string(),
        "banana".to_string(),
    ]
    .into_iter()
    .collect();
    let enc = encode_to_vec(&original).expect("encode BTreeSet<String>");
    let (val, _): (BTreeSet<String>, usize) =
        decode_from_slice(&enc).expect("decode BTreeSet<String>");
    assert_eq!(val, original);
}

#[test]
fn test_btreeset_empty_roundtrip() {
    let original: BTreeSet<u32> = BTreeSet::new();
    let enc = encode_to_vec(&original).expect("encode empty BTreeSet<u32>");
    let (val, _): (BTreeSet<u32>, usize) =
        decode_from_slice(&enc).expect("decode empty BTreeSet<u32>");
    assert!(val.is_empty());
    assert_eq!(val, original);
}

#[test]
fn test_btreeset_sorted_order_preserved() {
    let original: BTreeSet<i32> = vec![50, 10, 90, 30, 70, 20, 80, 40, 60, -5, 0]
        .into_iter()
        .collect();
    let enc = encode_to_vec(&original).expect("encode BTreeSet for order test");
    let (val, _): (BTreeSet<i32>, usize) =
        decode_from_slice(&enc).expect("decode BTreeSet for order test");

    let elements: Vec<i32> = val.iter().copied().collect();
    let mut sorted = elements.clone();
    sorted.sort_unstable();
    assert_eq!(
        elements, sorted,
        "BTreeSet must iterate in sorted ascending order after roundtrip"
    );
    assert_eq!(val, original);
}

#[test]
fn test_btreemap_i32_string_roundtrip() {
    let mut original: BTreeMap<i32, String> = BTreeMap::new();
    original.insert(-50, "neg_fifty".to_string());
    original.insert(-1, "neg_one".to_string());
    original.insert(0, "zero".to_string());
    original.insert(1, "one".to_string());
    original.insert(100, "hundred".to_string());
    let enc = encode_to_vec(&original).expect("encode BTreeMap<i32, String>");
    let (val, _): (BTreeMap<i32, String>, usize) =
        decode_from_slice(&enc).expect("decode BTreeMap<i32, String>");
    assert_eq!(val, original);
    assert_eq!(val.len(), 5);
}

#[test]
fn test_btreemap_sorted_keys() {
    let mut original: BTreeMap<i32, String> = BTreeMap::new();
    for &k in &[50i32, 10, 90, 30, 70, 20, 80, 40, 60, -5, 0] {
        original.insert(k, format!("val_{}", k));
    }
    let enc = encode_to_vec(&original).expect("encode BTreeMap for sorted keys test");
    let (val, _): (BTreeMap<i32, String>, usize) =
        decode_from_slice(&enc).expect("decode BTreeMap for sorted keys test");

    let keys: Vec<i32> = val.keys().copied().collect();
    let mut sorted_keys = keys.clone();
    sorted_keys.sort_unstable();
    assert_eq!(
        keys, sorted_keys,
        "BTreeMap keys must be in sorted order after roundtrip"
    );
    assert_eq!(val, original);
}

#[test]
fn test_btreemap_empty_roundtrip() {
    let original: BTreeMap<String, u32> = BTreeMap::new();
    let enc = encode_to_vec(&original).expect("encode empty BTreeMap<String, u32>");
    let (val, _): (BTreeMap<String, u32>, usize) =
        decode_from_slice(&enc).expect("decode empty BTreeMap<String, u32>");
    assert!(val.is_empty());
    assert_eq!(val, original);
}

#[test]
fn test_btreeset_consumed_equals_len() {
    let original: BTreeSet<u32> = vec![1, 2, 3, 4, 5, 6, 7, 8].into_iter().collect();
    let enc = encode_to_vec(&original).expect("encode BTreeSet for consumed test");
    let (_, consumed): (BTreeSet<u32>, usize) =
        decode_from_slice(&enc).expect("decode BTreeSet for consumed test");
    assert_eq!(
        consumed,
        enc.len(),
        "consumed bytes must equal total encoded length"
    );
}

#[test]
fn test_btreemap_consumed_equals_len() {
    let mut original: BTreeMap<String, u64> = BTreeMap::new();
    original.insert("alpha".to_string(), 1u64);
    original.insert("beta".to_string(), 2u64);
    original.insert("gamma".to_string(), 3u64);
    let enc = encode_to_vec(&original).expect("encode BTreeMap for consumed test");
    let (_, consumed): (BTreeMap<String, u64>, usize) =
        decode_from_slice(&enc).expect("decode BTreeMap for consumed test");
    assert_eq!(
        consumed,
        enc.len(),
        "consumed bytes must equal total encoded length"
    );
}

#[test]
fn test_ordering_tuple_roundtrip() {
    let original: (Ordering, u32) = (Ordering::Greater, 42u32);
    let enc = encode_to_vec(&original).expect("encode (Ordering, u32)");
    let (val, _): ((Ordering, u32), usize) =
        decode_from_slice(&enc).expect("decode (Ordering, u32)");
    assert_eq!(val.0, Ordering::Greater);
    assert_eq!(val.1, 42u32);
}

#[test]
fn test_btreemap_nested_value_roundtrip() {
    let mut original: BTreeMap<String, Vec<u32>> = BTreeMap::new();
    original.insert("fibonacci".to_string(), vec![1, 1, 2, 3, 5, 8, 13]);
    original.insert("primes".to_string(), vec![2, 3, 5, 7, 11]);
    original.insert("empty".to_string(), vec![]);
    original.insert("single".to_string(), vec![99]);
    let enc = encode_to_vec(&original).expect("encode BTreeMap<String, Vec<u32>>");
    let (val, _): (BTreeMap<String, Vec<u32>>, usize) =
        decode_from_slice(&enc).expect("decode BTreeMap<String, Vec<u32>>");
    assert_eq!(val, original);
    assert_eq!(val.get("fibonacci"), Some(&vec![1u32, 1, 2, 3, 5, 8, 13]));
}

#[test]
fn test_btreeset_large_roundtrip() {
    let original: BTreeSet<u32> = (0u32..20).collect();
    let enc = encode_to_vec(&original).expect("encode large BTreeSet<u32>");
    let (val, _): (BTreeSet<u32>, usize) =
        decode_from_slice(&enc).expect("decode large BTreeSet<u32>");
    assert_eq!(val, original);
    assert_eq!(val.len(), 20);

    let elements: Vec<u32> = val.iter().copied().collect();
    let expected: Vec<u32> = (0u32..20).collect();
    assert_eq!(
        elements, expected,
        "BTreeSet<u32> with 20 elements must be sorted after roundtrip"
    );
}

#[test]
fn test_btreemap_large_roundtrip() {
    let original: BTreeMap<u32, u32> = (0u32..20).map(|i| (i, i * i)).collect();
    let enc = encode_to_vec(&original).expect("encode large BTreeMap<u32, u32>");
    let (val, _): (BTreeMap<u32, u32>, usize) =
        decode_from_slice(&enc).expect("decode large BTreeMap<u32, u32>");
    assert_eq!(val, original);
    assert_eq!(val.len(), 20);

    for i in 0u32..20 {
        assert_eq!(
            val.get(&i),
            Some(&(i * i)),
            "entry {i} must survive roundtrip"
        );
    }
}

#[test]
fn test_ordering_fixed_int_config() {
    let cfg = config::standard().with_fixed_int_encoding();
    let enc = encode_to_vec_with_config(&Ordering::Equal, cfg)
        .expect("encode Ordering with fixed_int config");
    let (val, _): (Ordering, usize) =
        decode_from_slice_with_config(&enc, cfg).expect("decode Ordering with fixed_int config");
    assert_eq!(val, Ordering::Equal);
}

#[test]
fn test_btreeset_u8_all_values() {
    let original: BTreeSet<u8> = (0u8..=255).collect();
    assert_eq!(original.len(), 256);
    let enc = encode_to_vec(&original).expect("encode BTreeSet<u8> with 0..=255");
    let (val, consumed): (BTreeSet<u8>, usize) =
        decode_from_slice(&enc).expect("decode BTreeSet<u8> with 0..=255");
    assert_eq!(val, original);
    assert_eq!(val.len(), 256);
    assert_eq!(
        consumed,
        enc.len(),
        "consumed bytes must equal encoded length for BTreeSet<u8>"
    );

    let elements: Vec<u8> = val.iter().copied().collect();
    let expected: Vec<u8> = (0u8..=255).collect();
    assert_eq!(
        elements, expected,
        "BTreeSet<u8> must contain all values 0..=255 in order"
    );
}
