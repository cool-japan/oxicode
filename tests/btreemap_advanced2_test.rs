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

use std::collections::{BTreeMap, BTreeSet};

#[test]
fn test_btreemap_string_u32_roundtrip() {
    let mut map = BTreeMap::new();
    map.insert("alpha".to_string(), 1u32);
    map.insert("beta".to_string(), 2u32);
    map.insert("gamma".to_string(), 3u32);
    let enc = encode_to_vec(&map).expect("encode btreemap string u32");
    let (dec, _): (BTreeMap<String, u32>, usize) =
        decode_from_slice(&enc).expect("decode btreemap string u32");
    assert_eq!(map, dec);
}

#[test]
fn test_btreemap_u32_string_empty_roundtrip() {
    let map: BTreeMap<u32, String> = BTreeMap::new();
    let enc = encode_to_vec(&map).expect("encode empty btreemap u32 string");
    let (dec, _): (BTreeMap<u32, String>, usize) =
        decode_from_slice(&enc).expect("decode empty btreemap u32 string");
    assert_eq!(map, dec);
}

#[test]
fn test_btreemap_string_string_single_entry_roundtrip() {
    let mut map = BTreeMap::new();
    map.insert("key".to_string(), "value".to_string());
    let enc = encode_to_vec(&map).expect("encode single entry btreemap");
    let (dec, _): (BTreeMap<String, String>, usize) =
        decode_from_slice(&enc).expect("decode single entry btreemap");
    assert_eq!(map, dec);
}

#[test]
fn test_btreeset_u32_roundtrip() {
    let mut set = BTreeSet::new();
    set.insert(1u32);
    set.insert(2u32);
    set.insert(3u32);
    let enc = encode_to_vec(&set).expect("encode btreeset u32");
    let (dec, _): (BTreeSet<u32>, usize) = decode_from_slice(&enc).expect("decode btreeset u32");
    assert_eq!(set, dec);
}

#[test]
fn test_btreeset_string_roundtrip() {
    let mut set = BTreeSet::new();
    set.insert("apple".to_string());
    set.insert("banana".to_string());
    set.insert("cherry".to_string());
    let enc = encode_to_vec(&set).expect("encode btreeset string");
    let (dec, _): (BTreeSet<String>, usize) =
        decode_from_slice(&enc).expect("decode btreeset string");
    assert_eq!(set, dec);
}

#[test]
fn test_btreeset_u32_empty_roundtrip() {
    let set: BTreeSet<u32> = BTreeSet::new();
    let enc = encode_to_vec(&set).expect("encode empty btreeset u32");
    let (dec, _): (BTreeSet<u32>, usize) =
        decode_from_slice(&enc).expect("decode empty btreeset u32");
    assert_eq!(set, dec);
}

#[test]
fn test_btreemap_u32_vec_u8_roundtrip() {
    let mut map: BTreeMap<u32, Vec<u8>> = BTreeMap::new();
    map.insert(1, vec![10, 20, 30]);
    map.insert(2, vec![]);
    map.insert(3, vec![255, 128, 64]);
    let enc = encode_to_vec(&map).expect("encode btreemap u32 vec_u8");
    let (dec, _): (BTreeMap<u32, Vec<u8>>, usize) =
        decode_from_slice(&enc).expect("decode btreemap u32 vec_u8");
    assert_eq!(map, dec);
}

#[test]
fn test_btreemap_string_vec_string_roundtrip() {
    let mut map: BTreeMap<String, Vec<String>> = BTreeMap::new();
    map.insert(
        "fruits".to_string(),
        vec!["apple".to_string(), "banana".to_string()],
    );
    map.insert("vegs".to_string(), vec!["carrot".to_string()]);
    map.insert("empty".to_string(), vec![]);
    let enc = encode_to_vec(&map).expect("encode btreemap string vec_string");
    let (dec, _): (BTreeMap<String, Vec<String>>, usize) =
        decode_from_slice(&enc).expect("decode btreemap string vec_string");
    assert_eq!(map, dec);
}

#[test]
fn test_btreemap_key_ordering_preserved() {
    let mut map = BTreeMap::new();
    map.insert("z_last".to_string(), 100u32);
    map.insert("a_first".to_string(), 1u32);
    map.insert("m_middle".to_string(), 50u32);
    let enc = encode_to_vec(&map).expect("encode btreemap for ordering test");
    let (dec, _): (BTreeMap<String, u32>, usize) =
        decode_from_slice(&enc).expect("decode btreemap for ordering test");
    let original_keys: Vec<&String> = map.keys().collect();
    let decoded_keys: Vec<&String> = dec.keys().collect();
    assert_eq!(
        original_keys, decoded_keys,
        "key ordering must be preserved after roundtrip"
    );
}

#[test]
fn test_btreeset_ordering_preserved() {
    let mut set = BTreeSet::new();
    set.insert(50u32);
    set.insert(1u32);
    set.insert(999u32);
    set.insert(10u32);
    let enc = encode_to_vec(&set).expect("encode btreeset for ordering test");
    let (dec, _): (BTreeSet<u32>, usize) =
        decode_from_slice(&enc).expect("decode btreeset for ordering test");
    let original_vals: Vec<&u32> = set.iter().collect();
    let decoded_vals: Vec<&u32> = dec.iter().collect();
    assert_eq!(
        original_vals, decoded_vals,
        "btreeset ordering must be preserved after roundtrip"
    );
}

#[test]
fn test_btreemap_100_entries_roundtrip() {
    let mut map: BTreeMap<u32, u32> = BTreeMap::new();
    for i in 0..100u32 {
        map.insert(i, i * i);
    }
    let enc = encode_to_vec(&map).expect("encode btreemap 100 entries");
    let (dec, _): (BTreeMap<u32, u32>, usize) =
        decode_from_slice(&enc).expect("decode btreemap 100 entries");
    assert_eq!(map, dec);
}

#[test]
fn test_btreeset_100_entries_roundtrip() {
    let mut set: BTreeSet<u32> = BTreeSet::new();
    for i in 0..100u32 {
        set.insert(i * 3 + 7);
    }
    let enc = encode_to_vec(&set).expect("encode btreeset 100 entries");
    let (dec, _): (BTreeSet<u32>, usize) =
        decode_from_slice(&enc).expect("decode btreeset 100 entries");
    assert_eq!(set, dec);
}

#[test]
fn test_nested_btreemap_roundtrip() {
    let mut inner1 = BTreeMap::new();
    inner1.insert("x".to_string(), 10u32);
    inner1.insert("y".to_string(), 20u32);
    let mut inner2 = BTreeMap::new();
    inner2.insert("a".to_string(), 100u32);
    let mut outer: BTreeMap<String, BTreeMap<String, u32>> = BTreeMap::new();
    outer.insert("first".to_string(), inner1);
    outer.insert("second".to_string(), inner2);
    let enc = encode_to_vec(&outer).expect("encode nested btreemap");
    let (dec, _): (BTreeMap<String, BTreeMap<String, u32>>, usize) =
        decode_from_slice(&enc).expect("decode nested btreemap");
    assert_eq!(outer, dec);
}

#[test]
fn test_btreemap_fixed_int_encoding_config_roundtrip() {
    let mut map: BTreeMap<u32, u32> = BTreeMap::new();
    map.insert(1, 100);
    map.insert(2, 200);
    map.insert(3, 300);
    let cfg = config::standard().with_fixed_int_encoding();
    let enc = encode_to_vec_with_config(&map, cfg).expect("encode btreemap fixed_int config");
    let (dec, _): (BTreeMap<u32, u32>, usize) =
        decode_from_slice_with_config(&enc, cfg).expect("decode btreemap fixed_int config");
    assert_eq!(map, dec);
}

#[test]
fn test_btreemap_negative_keys_roundtrip() {
    let mut map: BTreeMap<i32, i32> = BTreeMap::new();
    map.insert(-10, -100);
    map.insert(-1, -1);
    map.insert(0, 0);
    map.insert(5, 50);
    let enc = encode_to_vec(&map).expect("encode btreemap negative keys");
    let (dec, _): (BTreeMap<i32, i32>, usize) =
        decode_from_slice(&enc).expect("decode btreemap negative keys");
    assert_eq!(map, dec);
}

#[test]
fn test_btreemap_consumed_bytes_equals_encoded_length() {
    let mut map: BTreeMap<String, u32> = BTreeMap::new();
    map.insert("foo".to_string(), 42u32);
    map.insert("bar".to_string(), 99u32);
    let enc = encode_to_vec(&map).expect("encode btreemap for consumed bytes test");
    let (_, consumed): (BTreeMap<String, u32>, usize) =
        decode_from_slice(&enc).expect("decode btreemap for consumed bytes test");
    assert_eq!(
        consumed,
        enc.len(),
        "consumed bytes must equal encoded length"
    );
}

#[test]
fn test_btreemap_u8_bool_roundtrip() {
    let mut map: BTreeMap<u8, bool> = BTreeMap::new();
    map.insert(0, false);
    map.insert(1, true);
    map.insert(127, false);
    map.insert(255, true);
    let enc = encode_to_vec(&map).expect("encode btreemap u8 bool");
    let (dec, _): (BTreeMap<u8, bool>, usize) =
        decode_from_slice(&enc).expect("decode btreemap u8 bool");
    assert_eq!(map, dec);
}

#[test]
fn test_btreeset_bool_roundtrip() {
    let mut set: BTreeSet<bool> = BTreeSet::new();
    set.insert(false);
    set.insert(true);
    let enc = encode_to_vec(&set).expect("encode btreeset bool");
    let (dec, _): (BTreeSet<bool>, usize) = decode_from_slice(&enc).expect("decode btreeset bool");
    assert_eq!(set, dec);
}

#[test]
fn test_btreemap_different_content_produces_different_bytes() {
    let mut map_a: BTreeMap<String, u32> = BTreeMap::new();
    map_a.insert("key".to_string(), 1u32);
    let mut map_b: BTreeMap<String, u32> = BTreeMap::new();
    map_b.insert("key".to_string(), 2u32);
    let enc_a = encode_to_vec(&map_a).expect("encode btreemap a");
    let enc_b = encode_to_vec(&map_b).expect("encode btreemap b");
    assert_ne!(
        enc_a, enc_b,
        "different content must produce different encoded bytes"
    );
}

#[test]
fn test_option_btreemap_some_roundtrip() {
    let mut map: BTreeMap<String, u32> = BTreeMap::new();
    map.insert("hello".to_string(), 42u32);
    let opt: Option<BTreeMap<String, u32>> = Some(map);
    let enc = encode_to_vec(&opt).expect("encode option some btreemap");
    let (dec, _): (Option<BTreeMap<String, u32>>, usize) =
        decode_from_slice(&enc).expect("decode option some btreemap");
    assert_eq!(opt, dec);
}

#[test]
fn test_option_btreemap_none_roundtrip() {
    let opt: Option<BTreeMap<String, u32>> = None;
    let enc = encode_to_vec(&opt).expect("encode option none btreemap");
    let (dec, _): (Option<BTreeMap<String, u32>>, usize) =
        decode_from_slice(&enc).expect("decode option none btreemap");
    assert_eq!(opt, dec);
}

#[test]
fn test_vec_of_btreemap_roundtrip() {
    let mut map1: BTreeMap<String, u32> = BTreeMap::new();
    map1.insert("a".to_string(), 1u32);
    let mut map2: BTreeMap<String, u32> = BTreeMap::new();
    map2.insert("b".to_string(), 2u32);
    map2.insert("c".to_string(), 3u32);
    let vec_maps = vec![map1, map2, BTreeMap::new()];
    let enc = encode_to_vec(&vec_maps).expect("encode vec of btreemap");
    let (dec, _): (Vec<BTreeMap<String, u32>>, usize) =
        decode_from_slice(&enc).expect("decode vec of btreemap");
    assert_eq!(vec_maps, dec);
}
