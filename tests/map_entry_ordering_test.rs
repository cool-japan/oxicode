//! Tests focused on HashMap and BTreeMap encoding with an emphasis on ordering
//! guarantees, determinism, and structural integrity across encode/decode
//! roundtrips.

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

use oxicode::{config, decode_from_slice, encode_to_vec};

// ---------------------------------------------------------------------------
// 1. BTreeMap<String, u32> preserves insertion/sort order after roundtrip
// ---------------------------------------------------------------------------
#[test]
fn test_btreemap_string_u32_sorted_order_after_roundtrip() {
    // Insert in deliberately scrambled order.
    let mut original: BTreeMap<String, u32> = BTreeMap::new();
    original.insert("zebra".to_string(), 26);
    original.insert("apple".to_string(), 1);
    original.insert("mango".to_string(), 13);
    original.insert("banana".to_string(), 2);
    original.insert("cherry".to_string(), 3);

    let bytes = encode_to_vec(&original).expect("encode BTreeMap<String, u32> failed");
    let (decoded, consumed): (BTreeMap<String, u32>, _) =
        decode_from_slice(&bytes).expect("decode BTreeMap<String, u32> failed");

    assert_eq!(original, decoded);
    assert_eq!(consumed, bytes.len());

    // BTreeMap must iterate in ascending lexicographic key order.
    let keys: Vec<&str> = decoded.keys().map(String::as_str).collect();
    let mut sorted = keys.clone();
    sorted.sort_unstable();
    assert_eq!(
        keys, sorted,
        "BTreeMap<String, u32> must iterate in sorted key order"
    );
    assert_eq!(keys, vec!["apple", "banana", "cherry", "mango", "zebra"]);
}

// ---------------------------------------------------------------------------
// 2. BTreeMap<u32, String> sorted order preserved
// ---------------------------------------------------------------------------
#[test]
fn test_btreemap_u32_string_sorted_order_preserved() {
    // Insert in reverse order.
    let original: BTreeMap<u32, String> = (0u32..8).rev().map(|i| (i, format!("v{}", i))).collect();

    let bytes = encode_to_vec(&original).expect("encode BTreeMap<u32, String> failed");
    let (decoded, consumed): (BTreeMap<u32, String>, _) =
        decode_from_slice(&bytes).expect("decode BTreeMap<u32, String> failed");

    assert_eq!(original, decoded);
    assert_eq!(consumed, bytes.len());

    let keys: Vec<u32> = decoded.keys().copied().collect();
    assert_eq!(keys, vec![0, 1, 2, 3, 4, 5, 6, 7]);
}

// ---------------------------------------------------------------------------
// 3. BTreeMap<i64, Vec<u8>> roundtrip
// ---------------------------------------------------------------------------
#[test]
fn test_btreemap_i64_vec_u8_roundtrip() {
    let mut original: BTreeMap<i64, Vec<u8>> = BTreeMap::new();
    original.insert(i64::MIN, vec![0x00, 0x01]);
    original.insert(-1_000_000, vec![0xFF, 0xFE]);
    original.insert(0, vec![]);
    original.insert(1_000_000, (0u8..16).collect());
    original.insert(i64::MAX, vec![0xDE, 0xAD, 0xBE, 0xEF]);

    let bytes = encode_to_vec(&original).expect("encode BTreeMap<i64, Vec<u8>> failed");
    let (decoded, consumed): (BTreeMap<i64, Vec<u8>>, _) =
        decode_from_slice(&bytes).expect("decode BTreeMap<i64, Vec<u8>> failed");

    assert_eq!(original, decoded);
    assert_eq!(consumed, bytes.len());

    // Verify that i64::MIN key survived with correct value.
    assert_eq!(decoded[&i64::MIN], vec![0x00, 0x01]);
    assert_eq!(decoded[&0], Vec::<u8>::new());
}

// ---------------------------------------------------------------------------
// 4. HashMap<String, u32> all values accessible after roundtrip
// ---------------------------------------------------------------------------
#[test]
fn test_hashmap_string_u32_all_values_accessible() {
    let mut original: HashMap<String, u32> = HashMap::new();
    original.insert("one".to_string(), 1);
    original.insert("two".to_string(), 2);
    original.insert("three".to_string(), 3);
    original.insert("four".to_string(), 4);
    original.insert("five".to_string(), 5);

    let bytes = encode_to_vec(&original).expect("encode HashMap<String, u32> failed");
    let (decoded, consumed): (HashMap<String, u32>, _) =
        decode_from_slice(&bytes).expect("decode HashMap<String, u32> failed");

    assert_eq!(original, decoded);
    assert_eq!(consumed, bytes.len());

    // Each key must map to the correct value.
    assert_eq!(decoded.get("one"), Some(&1));
    assert_eq!(decoded.get("two"), Some(&2));
    assert_eq!(decoded.get("three"), Some(&3));
    assert_eq!(decoded.get("four"), Some(&4));
    assert_eq!(decoded.get("five"), Some(&5));
    assert_eq!(decoded.get("six"), None);
}

// ---------------------------------------------------------------------------
// 5. HashMap<u32, String> all keys preserved
// ---------------------------------------------------------------------------
#[test]
fn test_hashmap_u32_string_all_keys_preserved() {
    let original: HashMap<u32, String> =
        (100u32..110).map(|i| (i, format!("item_{}", i))).collect();

    let bytes = encode_to_vec(&original).expect("encode HashMap<u32, String> failed");
    let (decoded, consumed): (HashMap<u32, String>, _) =
        decode_from_slice(&bytes).expect("decode HashMap<u32, String> failed");

    assert_eq!(original.len(), decoded.len());
    assert_eq!(consumed, bytes.len());

    for key in 100u32..110 {
        assert!(decoded.contains_key(&key), "missing key {key}");
        assert_eq!(decoded[&key], format!("item_{}", key));
    }
}

// ---------------------------------------------------------------------------
// 6. BTreeMap empty roundtrip
// ---------------------------------------------------------------------------
#[test]
fn test_btreemap_empty_roundtrip() {
    let original: BTreeMap<String, u32> = BTreeMap::new();

    let bytes = encode_to_vec(&original).expect("encode empty BTreeMap failed");
    let (decoded, consumed): (BTreeMap<String, u32>, _) =
        decode_from_slice(&bytes).expect("decode empty BTreeMap failed");

    assert!(decoded.is_empty());
    assert_eq!(original, decoded);
    assert_eq!(consumed, bytes.len());
}

// ---------------------------------------------------------------------------
// 7. HashMap empty roundtrip
// ---------------------------------------------------------------------------
#[test]
fn test_hashmap_empty_roundtrip() {
    let original: HashMap<u64, Vec<u8>> = HashMap::new();

    let bytes = encode_to_vec(&original).expect("encode empty HashMap failed");
    let (decoded, consumed): (HashMap<u64, Vec<u8>>, _) =
        decode_from_slice(&bytes).expect("decode empty HashMap failed");

    assert!(decoded.is_empty());
    assert_eq!(original, decoded);
    assert_eq!(consumed, bytes.len());
}

// ---------------------------------------------------------------------------
// 8. BTreeMap 1 entry roundtrip
// ---------------------------------------------------------------------------
#[test]
fn test_btreemap_single_entry_roundtrip() {
    let mut original: BTreeMap<String, u64> = BTreeMap::new();
    original.insert("only_key".to_string(), u64::MAX);

    let bytes = encode_to_vec(&original).expect("encode single-entry BTreeMap failed");
    let (decoded, consumed): (BTreeMap<String, u64>, _) =
        decode_from_slice(&bytes).expect("decode single-entry BTreeMap failed");

    assert_eq!(decoded.len(), 1);
    assert_eq!(original, decoded);
    assert_eq!(consumed, bytes.len());
    assert_eq!(decoded.get("only_key"), Some(&u64::MAX));
}

// ---------------------------------------------------------------------------
// 9. HashMap 1 entry roundtrip
// ---------------------------------------------------------------------------
#[test]
fn test_hashmap_single_entry_roundtrip() {
    let mut original: HashMap<u32, String> = HashMap::new();
    original.insert(42, "the answer".to_string());

    let bytes = encode_to_vec(&original).expect("encode single-entry HashMap failed");
    let (decoded, consumed): (HashMap<u32, String>, _) =
        decode_from_slice(&bytes).expect("decode single-entry HashMap failed");

    assert_eq!(decoded.len(), 1);
    assert_eq!(original, decoded);
    assert_eq!(consumed, bytes.len());
    assert_eq!(decoded.get(&42), Some(&"the answer".to_string()));
}

// ---------------------------------------------------------------------------
// 10. BTreeMap iteration order is deterministic across multiple encodes
// ---------------------------------------------------------------------------
#[test]
fn test_btreemap_iteration_order_is_deterministic() {
    let mut original: BTreeMap<u32, u32> = BTreeMap::new();
    for i in [7u32, 3, 11, 1, 9, 5, 13, 2, 8, 4] {
        original.insert(i, i * i);
    }

    // Encode twice and verify identical bytes (deterministic encoding).
    let bytes_a = encode_to_vec(&original).expect("first encode failed");
    let bytes_b = encode_to_vec(&original).expect("second encode failed");
    assert_eq!(
        bytes_a, bytes_b,
        "BTreeMap encoding must be deterministic: same bytes on repeated encode"
    );

    let (decoded_a, _): (BTreeMap<u32, u32>, _) =
        decode_from_slice(&bytes_a).expect("decode a failed");
    let (decoded_b, _): (BTreeMap<u32, u32>, _) =
        decode_from_slice(&bytes_b).expect("decode b failed");

    let keys_a: Vec<u32> = decoded_a.keys().copied().collect();
    let keys_b: Vec<u32> = decoded_b.keys().copied().collect();
    assert_eq!(
        keys_a, keys_b,
        "identical BTreeMap encodes must decode to identical iteration order"
    );
}

// ---------------------------------------------------------------------------
// 11. HashMap encoded: verify roundtrip regardless of encoding order
// ---------------------------------------------------------------------------
#[test]
fn test_hashmap_roundtrip_regardless_of_internal_order() {
    let mut original: HashMap<String, u32> = HashMap::new();
    original.insert("alpha".to_string(), 10);
    original.insert("beta".to_string(), 20);
    original.insert("gamma".to_string(), 30);
    original.insert("delta".to_string(), 40);
    original.insert("epsilon".to_string(), 50);

    let bytes = encode_to_vec(&original).expect("encode HashMap failed");
    let (decoded, consumed): (HashMap<String, u32>, _) =
        decode_from_slice(&bytes).expect("decode HashMap failed");

    // Regardless of wire-level entry order, the logical content must be identical.
    assert_eq!(original, decoded);
    assert_eq!(consumed, bytes.len());
    assert_eq!(decoded.len(), original.len());
}

// ---------------------------------------------------------------------------
// 12. BTreeMap<String, BTreeMap<String, u32>> nested roundtrip
// ---------------------------------------------------------------------------
#[test]
fn test_btreemap_nested_string_keys_roundtrip() {
    let mut inner_one: BTreeMap<String, u32> = BTreeMap::new();
    inner_one.insert("z".to_string(), 100);
    inner_one.insert("a".to_string(), 200);

    let mut inner_two: BTreeMap<String, u32> = BTreeMap::new();
    inner_two.insert("m".to_string(), 50);
    inner_two.insert("b".to_string(), 75);
    inner_two.insert("x".to_string(), 25);

    let mut original: BTreeMap<String, BTreeMap<String, u32>> = BTreeMap::new();
    original.insert("outer_b".to_string(), inner_one);
    original.insert("outer_a".to_string(), inner_two);
    original.insert("outer_c".to_string(), BTreeMap::new());

    let bytes =
        encode_to_vec(&original).expect("encode BTreeMap<String, BTreeMap<String, u32>> failed");
    let (decoded, consumed): (BTreeMap<String, BTreeMap<String, u32>>, _) =
        decode_from_slice(&bytes).expect("decode BTreeMap<String, BTreeMap<String, u32>> failed");

    assert_eq!(original, decoded);
    assert_eq!(consumed, bytes.len());

    // Outer keys must be sorted.
    let outer_keys: Vec<&str> = decoded.keys().map(String::as_str).collect();
    assert_eq!(outer_keys, vec!["outer_a", "outer_b", "outer_c"]);

    // Inner "outer_b" must be sorted.
    let inner_b_keys: Vec<&str> = decoded["outer_b"].keys().map(String::as_str).collect();
    assert_eq!(inner_b_keys, vec!["a", "z"]);
}

// ---------------------------------------------------------------------------
// 13. BTreeMap<u32, Vec<String>> roundtrip
// ---------------------------------------------------------------------------
#[test]
fn test_btreemap_u32_vec_string_roundtrip() {
    let mut original: BTreeMap<u32, Vec<String>> = BTreeMap::new();
    original.insert(1, vec!["red".to_string(), "blue".to_string()]);
    original.insert(2, vec!["green".to_string()]);
    original.insert(3, vec![]);
    original.insert(
        4,
        vec!["white".to_string(), "black".to_string(), "grey".to_string()],
    );

    let bytes = encode_to_vec(&original).expect("encode BTreeMap<u32, Vec<String>> failed");
    let (decoded, consumed): (BTreeMap<u32, Vec<String>>, _) =
        decode_from_slice(&bytes).expect("decode BTreeMap<u32, Vec<String>> failed");

    assert_eq!(original, decoded);
    assert_eq!(consumed, bytes.len());

    assert_eq!(decoded[&1], vec!["red", "blue"]);
    assert_eq!(decoded[&2], vec!["green"]);
    assert!(decoded[&3].is_empty());
}

// ---------------------------------------------------------------------------
// 14. HashMap<String, Vec<u32>> roundtrip
// ---------------------------------------------------------------------------
#[test]
fn test_hashmap_string_vec_u32_roundtrip() {
    let mut original: HashMap<String, Vec<u32>> = HashMap::new();
    original.insert("odds".to_string(), vec![1, 3, 5, 7, 9]);
    original.insert("evens".to_string(), vec![2, 4, 6, 8, 10]);
    original.insert("primes".to_string(), vec![2, 3, 5, 7, 11, 13]);
    original.insert("empty".to_string(), vec![]);

    let bytes = encode_to_vec(&original).expect("encode HashMap<String, Vec<u32>> failed");
    let (decoded, consumed): (HashMap<String, Vec<u32>>, _) =
        decode_from_slice(&bytes).expect("decode HashMap<String, Vec<u32>> failed");

    assert_eq!(original, decoded);
    assert_eq!(consumed, bytes.len());
    assert_eq!(decoded["odds"], vec![1, 3, 5, 7, 9]);
    assert_eq!(decoded["evens"], vec![2, 4, 6, 8, 10]);
    assert!(decoded["empty"].is_empty());
}

// ---------------------------------------------------------------------------
// 15. BTreeMap with 100 entries roundtrip
// ---------------------------------------------------------------------------
#[test]
fn test_btreemap_100_entries_roundtrip() {
    let original: BTreeMap<u32, u64> = (0u32..100).map(|i| (i, u64::from(i) * 1_000)).collect();

    let bytes = encode_to_vec(&original).expect("encode 100-entry BTreeMap failed");
    let (decoded, consumed): (BTreeMap<u32, u64>, _) =
        decode_from_slice(&bytes).expect("decode 100-entry BTreeMap failed");

    assert_eq!(original.len(), 100);
    assert_eq!(decoded.len(), 100);
    assert_eq!(original, decoded);
    assert_eq!(consumed, bytes.len());

    // Keys must iterate in ascending order.
    let keys: Vec<u32> = decoded.keys().copied().collect();
    let expected: Vec<u32> = (0u32..100).collect();
    assert_eq!(keys, expected);
}

// ---------------------------------------------------------------------------
// 16. HashMap with 100 entries roundtrip
// ---------------------------------------------------------------------------
#[test]
fn test_hashmap_100_entries_roundtrip() {
    let original: HashMap<u32, String> = (0u32..100)
        .map(|i| (i, format!("entry_{:03}", i)))
        .collect();

    let bytes = encode_to_vec(&original).expect("encode 100-entry HashMap failed");
    let (decoded, consumed): (HashMap<u32, String>, _) =
        decode_from_slice(&bytes).expect("decode 100-entry HashMap failed");

    assert_eq!(original.len(), 100);
    assert_eq!(decoded.len(), 100);
    assert_eq!(original, decoded);
    assert_eq!(consumed, bytes.len());

    // Spot-check a few entries.
    assert_eq!(decoded[&0], "entry_000");
    assert_eq!(decoded[&50], "entry_050");
    assert_eq!(decoded[&99], "entry_099");
}

// ---------------------------------------------------------------------------
// 17. BTreeMap sorted by key (0..50) verified by iteration
// ---------------------------------------------------------------------------
#[test]
fn test_btreemap_0_to_50_keys_verified_sorted_iteration() {
    // Insert in a pseudo-random permutation (reversed chunks).
    let mut original: BTreeMap<u32, u32> = BTreeMap::new();
    for chunk in (0u32..50).collect::<Vec<_>>().chunks(5).rev() {
        for &k in chunk {
            original.insert(k, k * 2);
        }
    }

    let bytes = encode_to_vec(&original).expect("encode BTreeMap 0..50 failed");
    let (decoded, consumed): (BTreeMap<u32, u32>, _) =
        decode_from_slice(&bytes).expect("decode BTreeMap 0..50 failed");

    assert_eq!(consumed, bytes.len());
    assert_eq!(decoded.len(), 50);

    // Iteration must be strictly ascending.
    let keys: Vec<u32> = decoded.keys().copied().collect();
    let expected: Vec<u32> = (0u32..50).collect();
    assert_eq!(
        keys, expected,
        "BTreeMap keys 0..50 must be in ascending order"
    );

    // Values must match.
    for k in 0u32..50 {
        assert_eq!(decoded[&k], k * 2, "value for key {k} incorrect");
    }
}

// ---------------------------------------------------------------------------
// 18. BTreeMap with string keys sorted alphabetically
// ---------------------------------------------------------------------------
#[test]
fn test_btreemap_string_keys_alphabetical_order() {
    let words = [
        "umbrella",
        "tangerine",
        "saxophone",
        "penguin",
        "narwhal",
        "kiwi",
        "jaguar",
        "igloo",
        "hamster",
        "giraffe",
    ];
    let original: BTreeMap<String, usize> = words
        .iter()
        .enumerate()
        .map(|(i, w)| (w.to_string(), i))
        .collect();

    let bytes = encode_to_vec(&original).expect("encode string-keyed BTreeMap failed");
    let (decoded, consumed): (BTreeMap<String, usize>, _) =
        decode_from_slice(&bytes).expect("decode string-keyed BTreeMap failed");

    assert_eq!(original, decoded);
    assert_eq!(consumed, bytes.len());

    // Iteration order must be alphabetical.
    let keys: Vec<&str> = decoded.keys().map(String::as_str).collect();
    let mut sorted = keys.clone();
    sorted.sort_unstable();
    assert_eq!(
        keys, sorted,
        "BTreeMap string keys must iterate alphabetically"
    );

    // First key must be "giraffe", last must be "umbrella".
    assert_eq!(keys[0], "giraffe");
    assert_eq!(*keys.last().expect("last key"), "umbrella");
}

// ---------------------------------------------------------------------------
// 19. Encode BTreeMap, decode as BTreeMap - same entries
// ---------------------------------------------------------------------------
#[test]
fn test_btreemap_encode_decode_same_entries() {
    let mut original: BTreeMap<u32, String> = BTreeMap::new();
    original.insert(10, "ten".to_string());
    original.insert(20, "twenty".to_string());
    original.insert(30, "thirty".to_string());

    let bytes = encode_to_vec(&original).expect("encode BTreeMap failed");
    let (decoded, consumed): (BTreeMap<u32, String>, _) =
        decode_from_slice(&bytes).expect("decode BTreeMap failed");

    // Same number of entries.
    assert_eq!(original.len(), decoded.len());
    assert_eq!(consumed, bytes.len());

    // Every key-value pair matches.
    for (k, v) in &original {
        assert_eq!(
            decoded.get(k),
            Some(v),
            "key {k} must map to '{v}' after roundtrip"
        );
    }
    // No phantom entries.
    for k in &decoded {
        assert!(original.contains_key(k.0), "phantom key {}", k.0);
    }
}

// ---------------------------------------------------------------------------
// 20. BTreeMap<(u32, u32), String> composite key roundtrip
// ---------------------------------------------------------------------------
#[test]
fn test_btreemap_tuple_composite_key_roundtrip() {
    let mut original: BTreeMap<(u32, u32), String> = BTreeMap::new();
    original.insert((0, 0), "origin".to_string());
    original.insert((0, 1), "right".to_string());
    original.insert((1, 0), "up".to_string());
    original.insert((1, 1), "diagonal".to_string());
    original.insert((255, 255), "far_corner".to_string());

    let bytes = encode_to_vec(&original).expect("encode BTreeMap tuple key failed");
    let (decoded, consumed): (BTreeMap<(u32, u32), String>, _) =
        decode_from_slice(&bytes).expect("decode BTreeMap tuple key failed");

    assert_eq!(original, decoded);
    assert_eq!(consumed, bytes.len());

    // Verify specific composite keys survived.
    assert_eq!(decoded.get(&(0, 0)), Some(&"origin".to_string()));
    assert_eq!(decoded.get(&(1, 1)), Some(&"diagonal".to_string()));
    assert_eq!(decoded.get(&(255, 255)), Some(&"far_corner".to_string()));
    assert_eq!(decoded.get(&(99, 99)), None);

    // BTreeMap orders tuples lexicographically: (0,0) < (0,1) < (1,0) < ...
    let keys: Vec<(u32, u32)> = decoded.keys().copied().collect();
    assert_eq!(keys[0], (0, 0));
    assert_eq!(keys[1], (0, 1));
    assert_eq!(keys[2], (1, 0));
}

// ---------------------------------------------------------------------------
// 21. Encode multiple maps sequentially and verify byte offsets
// ---------------------------------------------------------------------------
#[test]
fn test_encode_multiple_maps_sequentially() {
    let map_a: BTreeMap<u32, String> = [(1u32, "one".to_string()), (2, "two".to_string())]
        .into_iter()
        .collect();
    let map_b: HashMap<String, u32> = [("x".to_string(), 10u32), ("y".to_string(), 20)]
        .into_iter()
        .collect();

    let bytes_a = encode_to_vec(&map_a).expect("encode map_a failed");
    let bytes_b = encode_to_vec(&map_b).expect("encode map_b failed");

    // Concatenate and decode each segment independently.
    let mut combined = bytes_a.clone();
    combined.extend_from_slice(&bytes_b);

    let (decoded_a, consumed_a): (BTreeMap<u32, String>, _) =
        decode_from_slice(&combined).expect("decode map_a from combined failed");
    let (decoded_b, consumed_b): (HashMap<String, u32>, _) =
        decode_from_slice(&combined[consumed_a..]).expect("decode map_b from combined failed");

    assert_eq!(map_a, decoded_a);
    assert_eq!(map_b, decoded_b);

    // consumed_a + consumed_b must equal combined.len()
    assert_eq!(consumed_a + consumed_b, combined.len());
    assert_eq!(consumed_a, bytes_a.len());
    assert_eq!(consumed_b, bytes_b.len());
}

// ---------------------------------------------------------------------------
// 22. BTreeMap entry modification after roundtrip (can insert)
// ---------------------------------------------------------------------------
#[test]
fn test_btreemap_mutable_after_roundtrip() {
    let mut original: BTreeMap<u32, String> = BTreeMap::new();
    original.insert(1, "alpha".to_string());
    original.insert(2, "beta".to_string());
    original.insert(3, "gamma".to_string());

    let bytes = encode_to_vec(&original).expect("encode BTreeMap failed");
    let (mut decoded, consumed): (BTreeMap<u32, String>, _) =
        decode_from_slice(&bytes).expect("decode BTreeMap failed");

    assert_eq!(original, decoded);
    assert_eq!(consumed, bytes.len());

    // Insert a new entry into the decoded map — must work without error.
    decoded.insert(4, "delta".to_string());
    assert_eq!(decoded.len(), 4);
    assert_eq!(decoded.get(&4), Some(&"delta".to_string()));

    // Overwrite an existing entry.
    decoded.insert(1, "ALPHA_OVERWRITTEN".to_string());
    assert_eq!(decoded.get(&1), Some(&"ALPHA_OVERWRITTEN".to_string()));

    // Remove an entry.
    let removed = decoded.remove(&2);
    assert_eq!(removed, Some("beta".to_string()));
    assert_eq!(decoded.len(), 3);

    // Re-encode and decode the mutated map successfully.
    let bytes2 = encode_to_vec(&decoded).expect("re-encode mutated BTreeMap failed");
    let (final_decoded, consumed2): (BTreeMap<u32, String>, _) =
        decode_from_slice(&bytes2).expect("re-decode mutated BTreeMap failed");

    assert_eq!(decoded, final_decoded);
    assert_eq!(consumed2, bytes2.len());
    assert_eq!(final_decoded.len(), 3);

    // config is imported and referenced to silence the unused-import warning.
    let _cfg = config::standard();
}
