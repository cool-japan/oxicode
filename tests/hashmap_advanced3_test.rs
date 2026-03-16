//! Advanced tests for HashMap and HashSet encoding in OxiCode — scenario set 3.

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
    encode_to_vec_with_config, Decode, Encode,
};
use std::collections::{HashMap, HashSet};

#[derive(Debug, PartialEq, Eq, Hash, Encode, Decode, Clone)]
struct Key {
    namespace: String,
    id: u32,
}

#[derive(Debug, PartialEq, Encode, Decode, Clone)]
struct Value {
    data: Vec<u8>,
    score: f32,
}

// ============================================================================
// Test 1: HashMap<String, u32> empty roundtrip
// ============================================================================

#[test]
fn test_hashmap_string_u32_empty_roundtrip() {
    let original: HashMap<String, u32> = HashMap::new();
    let bytes = encode_to_vec(&original).expect("encode empty HashMap<String, u32>");
    let (decoded, _): (HashMap<String, u32>, usize) =
        decode_from_slice(&bytes).expect("decode empty HashMap<String, u32>");
    assert_eq!(original, decoded);
}

// ============================================================================
// Test 2: HashMap<String, u32> one entry roundtrip
// ============================================================================

#[test]
fn test_hashmap_string_u32_one_entry_roundtrip() {
    let mut original: HashMap<String, u32> = HashMap::new();
    original.insert("alpha".to_string(), 42);
    let bytes = encode_to_vec(&original).expect("encode one-entry HashMap<String, u32>");
    let (decoded, _): (HashMap<String, u32>, usize) =
        decode_from_slice(&bytes).expect("decode one-entry HashMap<String, u32>");
    assert_eq!(decoded.get("alpha"), Some(&42u32));
    assert_eq!(decoded.len(), 1);
}

// ============================================================================
// Test 3: HashMap<String, u32> five entries roundtrip (sort by key to verify)
// ============================================================================

#[test]
fn test_hashmap_string_u32_five_entries_roundtrip() {
    let mut original: HashMap<String, u32> = HashMap::new();
    original.insert("apple".to_string(), 1);
    original.insert("banana".to_string(), 2);
    original.insert("cherry".to_string(), 3);
    original.insert("date".to_string(), 4);
    original.insert("elderberry".to_string(), 5);

    let bytes = encode_to_vec(&original).expect("encode five-entry HashMap<String, u32>");
    let (decoded, _): (HashMap<String, u32>, usize) =
        decode_from_slice(&bytes).expect("decode five-entry HashMap<String, u32>");

    assert_eq!(decoded.len(), 5);
    let mut orig_pairs: Vec<(&String, &u32)> = original.iter().collect();
    orig_pairs.sort_by_key(|(k, _)| k.as_str());
    let mut dec_pairs: Vec<(&String, &u32)> = decoded.iter().collect();
    dec_pairs.sort_by_key(|(k, _)| k.as_str());
    assert_eq!(orig_pairs, dec_pairs);
}

// ============================================================================
// Test 4: HashMap<u32, String> roundtrip
// ============================================================================

#[test]
fn test_hashmap_u32_string_roundtrip() {
    let mut original: HashMap<u32, String> = HashMap::new();
    original.insert(1, "one".to_string());
    original.insert(2, "two".to_string());
    original.insert(3, "three".to_string());

    let bytes = encode_to_vec(&original).expect("encode HashMap<u32, String>");
    let (decoded, _): (HashMap<u32, String>, usize) =
        decode_from_slice(&bytes).expect("decode HashMap<u32, String>");

    assert_eq!(decoded.len(), 3);
    assert_eq!(decoded.get(&1), Some(&"one".to_string()));
    assert_eq!(decoded.get(&2), Some(&"two".to_string()));
    assert_eq!(decoded.get(&3), Some(&"three".to_string()));
}

// ============================================================================
// Test 5: HashMap<u32, u32> roundtrip with 10 entries
// ============================================================================

#[test]
fn test_hashmap_u32_u32_ten_entries_roundtrip() {
    let original: HashMap<u32, u32> = (0u32..10).map(|i| (i, i * i)).collect();

    let bytes = encode_to_vec(&original).expect("encode 10-entry HashMap<u32, u32>");
    let (decoded, _): (HashMap<u32, u32>, usize) =
        decode_from_slice(&bytes).expect("decode 10-entry HashMap<u32, u32>");

    assert_eq!(decoded.len(), 10);
    for i in 0u32..10 {
        assert_eq!(decoded.get(&i), Some(&(i * i)));
    }
}

// ============================================================================
// Test 6: HashSet<u32> empty roundtrip
// ============================================================================

#[test]
fn test_hashset_u32_empty_roundtrip() {
    let original: HashSet<u32> = HashSet::new();
    let bytes = encode_to_vec(&original).expect("encode empty HashSet<u32>");
    let (decoded, _): (HashSet<u32>, usize) =
        decode_from_slice(&bytes).expect("decode empty HashSet<u32>");
    assert_eq!(original, decoded);
}

// ============================================================================
// Test 7: HashSet<u32> five elements roundtrip
// ============================================================================

#[test]
fn test_hashset_u32_five_elements_roundtrip() {
    let original: HashSet<u32> = [10u32, 20, 30, 40, 50].iter().cloned().collect();
    let bytes = encode_to_vec(&original).expect("encode five-element HashSet<u32>");
    let (decoded, _): (HashSet<u32>, usize) =
        decode_from_slice(&bytes).expect("decode five-element HashSet<u32>");

    assert_eq!(original, decoded);
}

// ============================================================================
// Test 8: HashSet<String> roundtrip
// ============================================================================

#[test]
fn test_hashset_string_roundtrip() {
    let original: HashSet<String> = ["foo", "bar", "baz", "qux"]
        .iter()
        .map(|s| s.to_string())
        .collect();
    let bytes = encode_to_vec(&original).expect("encode HashSet<String>");
    let (decoded, _): (HashSet<String>, usize) =
        decode_from_slice(&bytes).expect("decode HashSet<String>");

    assert_eq!(original, decoded);
}

// ============================================================================
// Test 9: HashMap<String, Vec<u8>> roundtrip
// ============================================================================

#[test]
fn test_hashmap_string_vec_u8_roundtrip() {
    let mut original: HashMap<String, Vec<u8>> = HashMap::new();
    original.insert("empty".to_string(), vec![]);
    original.insert("short".to_string(), vec![1, 2, 3]);
    original.insert("longer".to_string(), vec![10, 20, 30, 40, 50, 60]);

    let bytes = encode_to_vec(&original).expect("encode HashMap<String, Vec<u8>>");
    let (decoded, _): (HashMap<String, Vec<u8>>, usize) =
        decode_from_slice(&bytes).expect("decode HashMap<String, Vec<u8>>");

    assert_eq!(decoded.len(), 3);
    assert_eq!(decoded.get("empty"), Some(&vec![]));
    assert_eq!(decoded.get("short"), Some(&vec![1u8, 2, 3]));
    assert_eq!(decoded.get("longer"), Some(&vec![10u8, 20, 30, 40, 50, 60]));
}

// ============================================================================
// Test 10: HashMap<Key, u32> roundtrip
// ============================================================================

#[test]
fn test_hashmap_custom_key_roundtrip() {
    let mut original: HashMap<Key, u32> = HashMap::new();
    original.insert(
        Key {
            namespace: "ns1".to_string(),
            id: 1,
        },
        100,
    );
    original.insert(
        Key {
            namespace: "ns1".to_string(),
            id: 2,
        },
        200,
    );
    original.insert(
        Key {
            namespace: "ns2".to_string(),
            id: 1,
        },
        300,
    );

    let bytes = encode_to_vec(&original).expect("encode HashMap<Key, u32>");
    let (decoded, _): (HashMap<Key, u32>, usize) =
        decode_from_slice(&bytes).expect("decode HashMap<Key, u32>");

    assert_eq!(decoded.len(), 3);
    assert_eq!(
        decoded.get(&Key {
            namespace: "ns1".to_string(),
            id: 1
        }),
        Some(&100u32)
    );
    assert_eq!(
        decoded.get(&Key {
            namespace: "ns1".to_string(),
            id: 2
        }),
        Some(&200u32)
    );
    assert_eq!(
        decoded.get(&Key {
            namespace: "ns2".to_string(),
            id: 1
        }),
        Some(&300u32)
    );
}

// ============================================================================
// Test 11: HashMap<u32, Value> roundtrip
// ============================================================================

#[test]
fn test_hashmap_u32_custom_value_roundtrip() {
    let mut original: HashMap<u32, Value> = HashMap::new();
    original.insert(
        1,
        Value {
            data: vec![0xDE, 0xAD],
            score: 1.5,
        },
    );
    original.insert(
        2,
        Value {
            data: vec![],
            score: 0.0,
        },
    );
    original.insert(
        3,
        Value {
            data: vec![255, 0, 128],
            score: -3.14,
        },
    );

    let bytes = encode_to_vec(&original).expect("encode HashMap<u32, Value>");
    let (decoded, _): (HashMap<u32, Value>, usize) =
        decode_from_slice(&bytes).expect("decode HashMap<u32, Value>");

    assert_eq!(decoded.len(), 3);
    let v1 = decoded.get(&1).expect("key 1 missing");
    assert_eq!(v1.data, vec![0xDEu8, 0xAD]);
    assert!((v1.score - 1.5f32).abs() < f32::EPSILON);
    let v2 = decoded.get(&2).expect("key 2 missing");
    assert_eq!(v2.data, Vec::<u8>::new());
    assert_eq!(v2.score, 0.0f32);
}

// ============================================================================
// Test 12: HashMap<String, u32> consumed bytes equals encoded len
// ============================================================================

#[test]
fn test_hashmap_string_u32_consumed_bytes_eq_encoded_len() {
    let mut original: HashMap<String, u32> = HashMap::new();
    original.insert("key1".to_string(), 111);
    original.insert("key2".to_string(), 222);

    let bytes = encode_to_vec(&original).expect("encode HashMap<String, u32> for size check");
    let (_, consumed): (HashMap<String, u32>, usize) =
        decode_from_slice(&bytes).expect("decode HashMap<String, u32> for size check");

    assert_eq!(consumed, bytes.len());
}

// ============================================================================
// Test 13: HashSet<u32> consumed bytes equals encoded len
// ============================================================================

#[test]
fn test_hashset_u32_consumed_bytes_eq_encoded_len() {
    let original: HashSet<u32> = [7u32, 14, 21, 28].iter().cloned().collect();
    let bytes = encode_to_vec(&original).expect("encode HashSet<u32> for size check");
    let (_, consumed): (HashSet<u32>, usize) =
        decode_from_slice(&bytes).expect("decode HashSet<u32> for size check");

    assert_eq!(consumed, bytes.len());
}

// ============================================================================
// Test 14: HashMap empty produces short encoding (just the 0-length varint)
// ============================================================================

#[test]
fn test_hashmap_empty_produces_short_encoding() {
    let original: HashMap<u32, u32> = HashMap::new();
    let bytes = encode_to_vec(&original).expect("encode empty HashMap<u32, u32>");
    // A varint encoding of 0 is just a single byte 0x00
    assert!(!bytes.is_empty());
    assert!(
        bytes.len() <= 4,
        "empty map encoding should be very short, got {} bytes",
        bytes.len()
    );
    assert_eq!(bytes[0], 0x00);
}

// ============================================================================
// Test 15: HashSet empty produces short encoding
// ============================================================================

#[test]
fn test_hashset_empty_produces_short_encoding() {
    let original: HashSet<u32> = HashSet::new();
    let bytes = encode_to_vec(&original).expect("encode empty HashSet<u32>");
    assert!(!bytes.is_empty());
    assert!(
        bytes.len() <= 4,
        "empty set encoding should be very short, got {} bytes",
        bytes.len()
    );
    assert_eq!(bytes[0], 0x00);
}

// ============================================================================
// Test 16: HashMap<u32, u32> with fixed int config roundtrip
// ============================================================================

#[test]
fn test_hashmap_u32_u32_fixed_int_config_roundtrip() {
    let cfg = config::standard().with_fixed_int_encoding();
    let mut original: HashMap<u32, u32> = HashMap::new();
    original.insert(0, u32::MIN);
    original.insert(1, 12345);
    original.insert(2, u32::MAX);

    let bytes = encode_to_vec_with_config(&original, cfg)
        .expect("encode HashMap<u32, u32> with fixed_int config");
    let (decoded, consumed): (HashMap<u32, u32>, usize) =
        decode_from_slice_with_config(&bytes, cfg)
            .expect("decode HashMap<u32, u32> with fixed_int config");

    assert_eq!(decoded.len(), 3);
    assert_eq!(decoded.get(&0), Some(&u32::MIN));
    assert_eq!(decoded.get(&1), Some(&12345u32));
    assert_eq!(decoded.get(&2), Some(&u32::MAX));
    assert_eq!(consumed, bytes.len());
}

// ============================================================================
// Test 17: Vec<HashMap<String, u32>> roundtrip (nested)
// ============================================================================

#[test]
fn test_vec_of_hashmaps_roundtrip() {
    let mut map1: HashMap<String, u32> = HashMap::new();
    map1.insert("x".to_string(), 1);
    map1.insert("y".to_string(), 2);

    let mut map2: HashMap<String, u32> = HashMap::new();
    map2.insert("a".to_string(), 10);

    let original: Vec<HashMap<String, u32>> = vec![map1, map2, HashMap::new()];

    let bytes = encode_to_vec(&original).expect("encode Vec<HashMap<String, u32>>");
    let (decoded, consumed): (Vec<HashMap<String, u32>>, usize) =
        decode_from_slice(&bytes).expect("decode Vec<HashMap<String, u32>>");

    assert_eq!(decoded.len(), 3);
    assert_eq!(decoded[0].get("x"), Some(&1u32));
    assert_eq!(decoded[0].get("y"), Some(&2u32));
    assert_eq!(decoded[1].get("a"), Some(&10u32));
    assert!(decoded[2].is_empty());
    assert_eq!(consumed, bytes.len());
}

// ============================================================================
// Test 18: Option<HashMap<String, u32>> Some roundtrip
// ============================================================================

#[test]
fn test_option_hashmap_some_roundtrip() {
    let mut inner: HashMap<String, u32> = HashMap::new();
    inner.insert("hello".to_string(), 99);
    let original: Option<HashMap<String, u32>> = Some(inner);

    let bytes = encode_to_vec(&original).expect("encode Option<HashMap<String, u32>> Some");
    let (decoded, consumed): (Option<HashMap<String, u32>>, usize) =
        decode_from_slice(&bytes).expect("decode Option<HashMap<String, u32>> Some");

    let inner_decoded = decoded.expect("expected Some after decode");
    assert_eq!(inner_decoded.get("hello"), Some(&99u32));
    assert_eq!(consumed, bytes.len());
}

// ============================================================================
// Test 19: Option<HashSet<u32>> None roundtrip
// ============================================================================

#[test]
fn test_option_hashset_none_roundtrip() {
    let original: Option<HashSet<u32>> = None;
    let bytes = encode_to_vec(&original).expect("encode Option<HashSet<u32>> None");
    let (decoded, consumed): (Option<HashSet<u32>>, usize) =
        decode_from_slice(&bytes).expect("decode Option<HashSet<u32>> None");

    assert!(decoded.is_none());
    assert_eq!(consumed, bytes.len());
}

// ============================================================================
// Test 20: HashMap<String, u32> re-encoding gives same key-value pairs on decode
// ============================================================================

#[test]
fn test_hashmap_reencode_gives_same_pairs() {
    let mut original: HashMap<String, u32> = HashMap::new();
    original.insert("foo".to_string(), 1);
    original.insert("bar".to_string(), 2);
    original.insert("baz".to_string(), 3);

    let bytes1 = encode_to_vec(&original).expect("first encode");
    let (decoded1, _): (HashMap<String, u32>, usize) =
        decode_from_slice(&bytes1).expect("first decode");

    let bytes2 = encode_to_vec(&decoded1).expect("second encode");
    let (decoded2, consumed2): (HashMap<String, u32>, usize) =
        decode_from_slice(&bytes2).expect("second decode");

    assert_eq!(decoded2.len(), 3);
    assert_eq!(decoded2.get("foo"), Some(&1u32));
    assert_eq!(decoded2.get("bar"), Some(&2u32));
    assert_eq!(decoded2.get("baz"), Some(&3u32));
    assert_eq!(consumed2, bytes2.len());
}

// ============================================================================
// Test 21: HashSet<u32> 100 elements roundtrip
// ============================================================================

#[test]
fn test_hashset_u32_100_elements_roundtrip() {
    let original: HashSet<u32> = (0u32..100).collect();
    let bytes = encode_to_vec(&original).expect("encode HashSet<u32> 100 elements");
    let (decoded, consumed): (HashSet<u32>, usize) =
        decode_from_slice(&bytes).expect("decode HashSet<u32> 100 elements");

    assert_eq!(decoded.len(), 100);
    for i in 0u32..100 {
        assert!(decoded.contains(&i), "missing element {}", i);
    }
    assert_eq!(consumed, bytes.len());
}

// ============================================================================
// Test 22: HashMap<u32, HashMap<u32, u32>> nested maps roundtrip
// ============================================================================

#[test]
fn test_hashmap_nested_maps_roundtrip() {
    let mut inner1: HashMap<u32, u32> = HashMap::new();
    inner1.insert(10, 100);
    inner1.insert(20, 200);

    let mut inner2: HashMap<u32, u32> = HashMap::new();
    inner2.insert(30, 300);

    let mut original: HashMap<u32, HashMap<u32, u32>> = HashMap::new();
    original.insert(1, inner1);
    original.insert(2, inner2);
    original.insert(3, HashMap::new());

    let bytes = encode_to_vec(&original).expect("encode HashMap<u32, HashMap<u32, u32>>");
    let (decoded, consumed): (HashMap<u32, HashMap<u32, u32>>, usize) =
        decode_from_slice(&bytes).expect("decode HashMap<u32, HashMap<u32, u32>>");

    assert_eq!(decoded.len(), 3);

    let d1 = decoded.get(&1).expect("outer key 1 missing");
    assert_eq!(d1.get(&10), Some(&100u32));
    assert_eq!(d1.get(&20), Some(&200u32));

    let d2 = decoded.get(&2).expect("outer key 2 missing");
    assert_eq!(d2.get(&30), Some(&300u32));

    let d3 = decoded.get(&3).expect("outer key 3 missing");
    assert!(d3.is_empty());

    assert_eq!(consumed, bytes.len());
}
