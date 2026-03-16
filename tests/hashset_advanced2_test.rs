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
use std::collections::{HashMap, HashSet};

#[test]
fn test_hashset_u32_roundtrip() {
    let mut original: HashSet<u32> = HashSet::new();
    original.insert(1);
    original.insert(2);
    original.insert(3);
    original.insert(4);
    original.insert(5);

    let enc = encode_to_vec(&original).expect("encode failed");
    let (decoded, _): (HashSet<u32>, usize) = decode_from_slice(&enc).expect("decode failed");

    assert_eq!(decoded, original);
}

#[test]
fn test_hashset_u32_empty_roundtrip() {
    let original: HashSet<u32> = HashSet::new();

    let enc = encode_to_vec(&original).expect("encode failed");
    let (decoded, _): (HashSet<u32>, usize) = decode_from_slice(&enc).expect("decode failed");

    assert_eq!(decoded, original);
    assert!(decoded.is_empty());
}

#[test]
fn test_hashset_string_roundtrip() {
    let mut original: HashSet<String> = HashSet::new();
    original.insert("alpha".to_string());
    original.insert("beta".to_string());
    original.insert("gamma".to_string());

    let enc = encode_to_vec(&original).expect("encode failed");
    let (decoded, _): (HashSet<String>, usize) = decode_from_slice(&enc).expect("decode failed");

    assert_eq!(decoded, original);
}

#[test]
fn test_hashset_u32_preserved() {
    let mut original: HashSet<u32> = HashSet::new();
    original.insert(10);
    original.insert(20);
    original.insert(30);
    original.insert(40);
    original.insert(50);

    let enc = encode_to_vec(&original).expect("encode failed");
    let (decoded, _): (HashSet<u32>, usize) = decode_from_slice(&enc).expect("decode failed");

    for val in &original {
        assert!(
            decoded.contains(val),
            "decoded HashSet missing element: {}",
            val
        );
    }
    assert_eq!(decoded.len(), original.len());
}

#[test]
fn test_hashset_consumed_equals_len() {
    let mut original: HashSet<u32> = HashSet::new();
    original.insert(100);
    original.insert(200);
    original.insert(300);

    let enc = encode_to_vec(&original).expect("encode failed");
    let (_, consumed): (HashSet<u32>, usize) = decode_from_slice(&enc).expect("decode failed");

    assert_eq!(consumed, enc.len());
}

#[test]
fn test_hashset_fixed_int_config() {
    let cfg = config::standard().with_fixed_int_encoding();

    let mut original: HashSet<u32> = HashSet::new();
    original.insert(1);
    original.insert(2);
    original.insert(3);

    let enc = encode_to_vec_with_config(&original, cfg).expect("encode with config failed");
    let (decoded, _): (HashSet<u32>, usize) =
        decode_from_slice_with_config(&enc, cfg).expect("decode with config failed");

    assert_eq!(decoded, original);
}

#[test]
fn test_hashset_u8_roundtrip() {
    let original: HashSet<u8> = (0u8..=10).collect();

    let enc = encode_to_vec(&original).expect("encode failed");
    let (decoded, _): (HashSet<u8>, usize) = decode_from_slice(&enc).expect("decode failed");

    assert_eq!(decoded, original);
    assert_eq!(decoded.len(), 11);
}

#[test]
fn test_hashmap_u32_u32_roundtrip() {
    let mut original: HashMap<u32, u32> = HashMap::new();
    original.insert(1, 100);
    original.insert(2, 200);
    original.insert(3, 300);

    let enc = encode_to_vec(&original).expect("encode failed");
    let (decoded, _): (HashMap<u32, u32>, usize) = decode_from_slice(&enc).expect("decode failed");

    assert_eq!(decoded, original);
}

#[test]
fn test_hashmap_u32_u32_empty_roundtrip() {
    let original: HashMap<u32, u32> = HashMap::new();

    let enc = encode_to_vec(&original).expect("encode failed");
    let (decoded, _): (HashMap<u32, u32>, usize) = decode_from_slice(&enc).expect("decode failed");

    assert_eq!(decoded, original);
    assert!(decoded.is_empty());
}

#[test]
fn test_hashmap_string_u32_roundtrip() {
    let mut original: HashMap<String, u32> = HashMap::new();
    original.insert("one".to_string(), 1);
    original.insert("two".to_string(), 2);
    original.insert("three".to_string(), 3);

    let enc = encode_to_vec(&original).expect("encode failed");
    let (decoded, _): (HashMap<String, u32>, usize) =
        decode_from_slice(&enc).expect("decode failed");

    assert_eq!(decoded, original);
}

#[test]
fn test_hashmap_preserved() {
    let mut original: HashMap<u32, u32> = HashMap::new();
    original.insert(7, 49);
    original.insert(11, 121);
    original.insert(13, 169);

    let enc = encode_to_vec(&original).expect("encode failed");
    let (decoded, _): (HashMap<u32, u32>, usize) = decode_from_slice(&enc).expect("decode failed");

    for (k, v) in &original {
        let decoded_val = decoded.get(k).expect("key missing from decoded map");
        assert_eq!(decoded_val, v, "value mismatch for key {}", k);
    }
    assert_eq!(decoded.len(), original.len());
}

#[test]
fn test_hashmap_consumed_equals_len() {
    let mut original: HashMap<u32, u32> = HashMap::new();
    original.insert(1, 10);
    original.insert(2, 20);
    original.insert(3, 30);

    let enc = encode_to_vec(&original).expect("encode failed");
    let (_, consumed): (HashMap<u32, u32>, usize) = decode_from_slice(&enc).expect("decode failed");

    assert_eq!(consumed, enc.len());
}

#[test]
fn test_hashmap_fixed_int_config() {
    let cfg = config::standard().with_fixed_int_encoding();

    let mut original: HashMap<u32, u32> = HashMap::new();
    original.insert(1, 111);
    original.insert(2, 222);
    original.insert(3, 333);

    let enc = encode_to_vec_with_config(&original, cfg).expect("encode with config failed");
    let (decoded, _): (HashMap<u32, u32>, usize) =
        decode_from_slice_with_config(&enc, cfg).expect("decode with config failed");

    assert_eq!(decoded, original);
}

#[test]
fn test_hashmap_string_string_roundtrip() {
    let mut original: HashMap<String, String> = HashMap::new();
    original.insert("key1".to_string(), "value1".to_string());
    original.insert("key2".to_string(), "value2".to_string());
    original.insert("key3".to_string(), "value3".to_string());

    let enc = encode_to_vec(&original).expect("encode failed");
    let (decoded, _): (HashMap<String, String>, usize) =
        decode_from_slice(&enc).expect("decode failed");

    assert_eq!(decoded, original);
}

#[test]
fn test_option_hashset_some_roundtrip() {
    let mut inner: HashSet<u32> = HashSet::new();
    inner.insert(1);
    inner.insert(2);
    inner.insert(3);
    let original: Option<HashSet<u32>> = Some(inner);

    let enc = encode_to_vec(&original).expect("encode failed");
    let (decoded, _): (Option<HashSet<u32>>, usize) =
        decode_from_slice(&enc).expect("decode failed");

    assert_eq!(decoded, original);
    assert!(decoded.is_some());
}

#[test]
fn test_option_hashset_none_roundtrip() {
    let original: Option<HashSet<u32>> = None;

    let enc = encode_to_vec(&original).expect("encode failed");
    let (decoded, _): (Option<HashSet<u32>>, usize) =
        decode_from_slice(&enc).expect("decode failed");

    assert_eq!(decoded, original);
    assert!(decoded.is_none());
}

#[test]
fn test_option_hashmap_some_roundtrip() {
    let mut inner: HashMap<u32, u32> = HashMap::new();
    inner.insert(10, 100);
    inner.insert(20, 200);
    let original: Option<HashMap<u32, u32>> = Some(inner);

    let enc = encode_to_vec(&original).expect("encode failed");
    let (decoded, _): (Option<HashMap<u32, u32>>, usize) =
        decode_from_slice(&enc).expect("decode failed");

    assert_eq!(decoded, original);
    assert!(decoded.is_some());
}

#[test]
fn test_option_hashmap_none_roundtrip() {
    let original: Option<HashMap<u32, u32>> = None;

    let enc = encode_to_vec(&original).expect("encode failed");
    let (decoded, _): (Option<HashMap<u32, u32>>, usize) =
        decode_from_slice(&enc).expect("decode failed");

    assert_eq!(decoded, original);
    assert!(decoded.is_none());
}

#[test]
fn test_hashset_large_roundtrip() {
    let original: HashSet<u32> = (0u32..50).collect();

    let enc = encode_to_vec(&original).expect("encode failed");
    let (decoded, _): (HashSet<u32>, usize) = decode_from_slice(&enc).expect("decode failed");

    assert_eq!(decoded, original);
    assert_eq!(decoded.len(), 50);
}

#[test]
fn test_hashmap_large_roundtrip() {
    let mut original: HashMap<u32, u32> = HashMap::with_capacity(30);
    for i in 0u32..30 {
        original.insert(i, i * i);
    }

    let enc = encode_to_vec(&original).expect("encode failed");
    let (decoded, _): (HashMap<u32, u32>, usize) = decode_from_slice(&enc).expect("decode failed");

    assert_eq!(decoded, original);
    assert_eq!(decoded.len(), 30);
}

#[test]
fn test_hashmap_nested_value_roundtrip() {
    let mut original: HashMap<String, Vec<u32>> = HashMap::new();
    original.insert("primes".to_string(), vec![2, 3, 5, 7, 11]);
    original.insert("squares".to_string(), vec![1, 4, 9, 16, 25]);
    original.insert("empty".to_string(), vec![]);

    let enc = encode_to_vec(&original).expect("encode failed");
    let (decoded, _): (HashMap<String, Vec<u32>>, usize) =
        decode_from_slice(&enc).expect("decode failed");

    assert_eq!(decoded, original);
}

#[test]
fn test_hashset_reencode_stability() {
    let mut original: HashSet<u32> = HashSet::new();
    original.insert(10);
    original.insert(20);
    original.insert(30);
    original.insert(40);
    original.insert(50);

    let enc1 = encode_to_vec(&original).expect("first encode failed");
    let (decoded1, _): (HashSet<u32>, usize) =
        decode_from_slice(&enc1).expect("first decode failed");

    let enc2 = encode_to_vec(&decoded1).expect("re-encode failed");
    let (decoded2, _): (HashSet<u32>, usize) =
        decode_from_slice(&enc2).expect("second decode failed");

    assert_eq!(decoded2, decoded1);
    assert_eq!(decoded2, original);
}
