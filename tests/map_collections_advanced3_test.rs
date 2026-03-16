//! Advanced map/set collection tests — third batch (22 tests, new scenarios).
//! Focus: HashMap<u32,u32>, HashMap<String,*>, BTreeMap variants, HashSet,
//! BTreeSet, nested maps, Option/Vec wrappers, and a struct-level roundtrip.

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
use oxicode::{decode_from_slice, encode_to_vec, Decode, Encode};
use std::collections::{BTreeMap, BTreeSet, HashMap, HashSet};

// ─── 1. HashMap<u32, u32> with 0 entries roundtrip ───────────────────────────

#[test]
fn test_mac3_hashmap_u32_u32_empty_roundtrip() {
    let original: HashMap<u32, u32> = HashMap::new();

    let enc = encode_to_vec(&original).expect("encode empty HashMap<u32,u32>");
    let (val, _): (HashMap<u32, u32>, _) =
        decode_from_slice(&enc).expect("decode empty HashMap<u32,u32>");

    assert_eq!(original, val);
    assert!(val.is_empty());
}

// ─── 2. HashMap<u32, u32> with 1 entry roundtrip ─────────────────────────────

#[test]
fn test_mac3_hashmap_u32_u32_one_entry_roundtrip() {
    let mut original: HashMap<u32, u32> = HashMap::new();
    original.insert(7, 42);

    let enc = encode_to_vec(&original).expect("encode single-entry HashMap<u32,u32>");
    let (val, consumed): (HashMap<u32, u32>, _) =
        decode_from_slice(&enc).expect("decode single-entry HashMap<u32,u32>");

    assert_eq!(original, val);
    assert_eq!(val.len(), 1);
    assert_eq!(val[&7], 42);
    assert_eq!(consumed, enc.len());
}

// ─── 3. HashMap<u32, u32> with 100 entries roundtrip ─────────────────────────

#[test]
fn test_mac3_hashmap_u32_u32_hundred_entries_roundtrip() {
    let mut original: HashMap<u32, u32> = HashMap::with_capacity(100);
    for i in 0u32..100 {
        original.insert(i, i * i);
    }

    let enc = encode_to_vec(&original).expect("encode 100-entry HashMap<u32,u32>");
    let (val, consumed): (HashMap<u32, u32>, _) =
        decode_from_slice(&enc).expect("decode 100-entry HashMap<u32,u32>");

    assert_eq!(original, val);
    assert_eq!(val.len(), 100);
    assert_eq!(val[&0], 0);
    assert_eq!(val[&7], 49);
    assert_eq!(val[&99], 9801);
    assert_eq!(consumed, enc.len());
}

// ─── 4. HashMap<String, String> roundtrip ────────────────────────────────────

#[test]
fn test_mac3_hashmap_string_string_roundtrip() {
    let mut original: HashMap<String, String> = HashMap::new();
    original.insert("language".to_string(), "Rust".to_string());
    original.insert("library".to_string(), "oxicode".to_string());
    original.insert("format".to_string(), "binary".to_string());
    original.insert("empty_val".to_string(), String::new());

    let enc = encode_to_vec(&original).expect("encode HashMap<String,String>");
    let (val, consumed): (HashMap<String, String>, _) =
        decode_from_slice(&enc).expect("decode HashMap<String,String>");

    assert_eq!(original, val);
    assert_eq!(val["language"], "Rust");
    assert_eq!(val["library"], "oxicode");
    assert_eq!(val["empty_val"], "");
    assert_eq!(consumed, enc.len());
}

// ─── 5. HashMap<String, Vec<u8>> roundtrip ───────────────────────────────────

#[test]
fn test_mac3_hashmap_string_vec_u8_roundtrip() {
    let mut original: HashMap<String, Vec<u8>> = HashMap::new();
    original.insert("header".to_string(), vec![0xDE, 0xAD, 0xBE, 0xEF]);
    original.insert("payload".to_string(), (0u8..=127).collect());
    original.insert("empty".to_string(), vec![]);

    let enc = encode_to_vec(&original).expect("encode HashMap<String,Vec<u8>>");
    let (val, consumed): (HashMap<String, Vec<u8>>, _) =
        decode_from_slice(&enc).expect("decode HashMap<String,Vec<u8>>");

    assert_eq!(original, val);
    assert_eq!(val["header"], [0xDE, 0xAD, 0xBE, 0xEF]);
    assert_eq!(val["payload"].len(), 128);
    assert!(val["empty"].is_empty());
    assert_eq!(consumed, enc.len());
}

// ─── 6. HashMap<u32, Vec<String>> roundtrip ──────────────────────────────────

#[test]
fn test_mac3_hashmap_u32_vec_string_roundtrip() {
    let mut original: HashMap<u32, Vec<String>> = HashMap::new();
    original.insert(1, vec!["alpha".to_string(), "beta".to_string()]);
    original.insert(2, vec!["gamma".to_string()]);
    original.insert(3, vec![]);
    original.insert(
        4,
        vec![
            "delta".to_string(),
            "epsilon".to_string(),
            "zeta".to_string(),
        ],
    );

    let enc = encode_to_vec(&original).expect("encode HashMap<u32,Vec<String>>");
    let (val, consumed): (HashMap<u32, Vec<String>>, _) =
        decode_from_slice(&enc).expect("decode HashMap<u32,Vec<String>>");

    assert_eq!(original, val);
    assert_eq!(val[&1].len(), 2);
    assert_eq!(val[&1][0], "alpha");
    assert_eq!(val[&2].len(), 1);
    assert!(val[&3].is_empty());
    assert_eq!(val[&4].len(), 3);
    assert_eq!(consumed, enc.len());
}

// ─── 7. BTreeMap<u32, u32> with 0 entries roundtrip ──────────────────────────

#[test]
fn test_mac3_btreemap_u32_u32_empty_roundtrip() {
    let original: BTreeMap<u32, u32> = BTreeMap::new();

    let enc = encode_to_vec(&original).expect("encode empty BTreeMap<u32,u32>");
    let (val, consumed): (BTreeMap<u32, u32>, _) =
        decode_from_slice(&enc).expect("decode empty BTreeMap<u32,u32>");

    assert_eq!(original, val);
    assert!(val.is_empty());
    assert_eq!(consumed, enc.len());
}

// ─── 8. BTreeMap<u32, u32> with 50 entries in sorted order ───────────────────

#[test]
fn test_mac3_btreemap_u32_u32_fifty_entries_sorted_roundtrip() {
    // Insert in reverse order — BTreeMap must sort them
    let mut original: BTreeMap<u32, u32> = BTreeMap::new();
    for i in (0u32..50).rev() {
        original.insert(i, i * 3 + 1);
    }

    let enc = encode_to_vec(&original).expect("encode 50-entry BTreeMap<u32,u32>");
    let (val, consumed): (BTreeMap<u32, u32>, _) =
        decode_from_slice(&enc).expect("decode 50-entry BTreeMap<u32,u32>");

    assert_eq!(original, val);
    assert_eq!(val.len(), 50);
    assert_eq!(consumed, enc.len());

    // Keys must iterate in ascending order
    let keys: Vec<u32> = val.keys().copied().collect();
    let mut expected: Vec<u32> = (0u32..50).collect();
    expected.sort();
    assert_eq!(keys, expected, "BTreeMap keys must be in ascending order");

    assert_eq!(val[&0], 1);
    assert_eq!(val[&49], 148);
}

// ─── 9. BTreeMap<String, u32> roundtrip ──────────────────────────────────────

#[test]
fn test_mac3_btreemap_string_u32_roundtrip() {
    let mut original: BTreeMap<String, u32> = BTreeMap::new();
    original.insert("cherry".to_string(), 3);
    original.insert("apple".to_string(), 1);
    original.insert("banana".to_string(), 2);
    original.insert("date".to_string(), 4);

    let enc = encode_to_vec(&original).expect("encode BTreeMap<String,u32>");
    let (val, consumed): (BTreeMap<String, u32>, _) =
        decode_from_slice(&enc).expect("decode BTreeMap<String,u32>");

    assert_eq!(original, val);
    assert_eq!(val["apple"], 1);
    assert_eq!(val["date"], 4);
    assert_eq!(consumed, enc.len());

    // Must iterate in lexicographic order
    let keys: Vec<&str> = val.keys().map(String::as_str).collect();
    assert_eq!(keys, vec!["apple", "banana", "cherry", "date"]);
}

// ─── 10. BTreeMap deterministic: same map always produces same bytes ──────────

#[test]
fn test_mac3_btreemap_deterministic_encoding() {
    let mut map_a: BTreeMap<String, u32> = BTreeMap::new();
    map_a.insert("z".to_string(), 26);
    map_a.insert("a".to_string(), 1);
    map_a.insert("m".to_string(), 13);

    // Build the same logical map via a different insertion order
    let mut map_b: BTreeMap<String, u32> = BTreeMap::new();
    map_b.insert("m".to_string(), 13);
    map_b.insert("z".to_string(), 26);
    map_b.insert("a".to_string(), 1);

    let enc_a = encode_to_vec(&map_a).expect("encode btreemap_a");
    let enc_b = encode_to_vec(&map_b).expect("encode btreemap_b");

    // BTreeMap serialises in key order, so bytes must be identical
    assert_eq!(
        enc_a, enc_b,
        "BTreeMap with same entries must produce identical bytes regardless of insertion order"
    );

    let (val, consumed): (BTreeMap<String, u32>, _) =
        decode_from_slice(&enc_a).expect("decode deterministic BTreeMap");
    assert_eq!(val, map_a);
    assert_eq!(consumed, enc_a.len());
}

// ─── 11. HashSet<u32> roundtrip ──────────────────────────────────────────────

#[test]
fn test_mac3_hashset_u32_roundtrip() {
    let original: HashSet<u32> = [0u32, 1, 2, 100, u32::MAX].iter().copied().collect();

    let enc = encode_to_vec(&original).expect("encode HashSet<u32>");
    let (val, consumed): (HashSet<u32>, _) = decode_from_slice(&enc).expect("decode HashSet<u32>");

    assert_eq!(original, val);
    assert!(val.contains(&0));
    assert!(val.contains(&u32::MAX));
    assert!(!val.contains(&42));
    assert_eq!(consumed, enc.len());
}

// ─── 12. HashSet<String> roundtrip ───────────────────────────────────────────

#[test]
fn test_mac3_hashset_string_roundtrip() {
    let original: HashSet<String> = ["oxicode", "binary", "serialization", "pure-rust"]
        .iter()
        .map(|s| s.to_string())
        .collect();

    let enc = encode_to_vec(&original).expect("encode HashSet<String>");
    let (val, consumed): (HashSet<String>, _) =
        decode_from_slice(&enc).expect("decode HashSet<String>");

    assert_eq!(original, val);
    assert!(val.contains("oxicode"));
    assert!(val.contains("pure-rust"));
    assert!(!val.contains("bincode"));
    assert_eq!(consumed, enc.len());
}

// ─── 13. BTreeSet<u32> roundtrip ─────────────────────────────────────────────

#[test]
fn test_mac3_btreeset_u32_roundtrip() {
    let original: BTreeSet<u32> = [50u32, 3, 7, 100, 1, 42].iter().copied().collect();

    let enc = encode_to_vec(&original).expect("encode BTreeSet<u32>");
    let (val, consumed): (BTreeSet<u32>, _) =
        decode_from_slice(&enc).expect("decode BTreeSet<u32>");

    assert_eq!(original, val);
    assert!(val.contains(&1));
    assert!(val.contains(&100));
    assert!(!val.contains(&0));
    assert_eq!(consumed, enc.len());
}

// ─── 14. BTreeSet<String> roundtrip (sorted order preserved) ─────────────────

#[test]
fn test_mac3_btreeset_string_sorted_roundtrip() {
    let original: BTreeSet<String> = ["mango", "apple", "cherry", "banana"]
        .iter()
        .map(|s| s.to_string())
        .collect();

    let enc = encode_to_vec(&original).expect("encode BTreeSet<String>");
    let (val, consumed): (BTreeSet<String>, _) =
        decode_from_slice(&enc).expect("decode BTreeSet<String>");

    assert_eq!(original, val);
    assert_eq!(consumed, enc.len());

    // Must iterate in lexicographic ascending order
    let items: Vec<&str> = val.iter().map(String::as_str).collect();
    assert_eq!(items, vec!["apple", "banana", "cherry", "mango"]);
}

// ─── 15. HashMap<u32, HashMap<String, u32>> nested roundtrip ─────────────────

#[test]
fn test_mac3_nested_hashmap_u32_to_hashmap_string_u32() {
    let mut inner1: HashMap<String, u32> = HashMap::new();
    inner1.insert("score".to_string(), 95);
    inner1.insert("level".to_string(), 3);

    let mut inner2: HashMap<String, u32> = HashMap::new();
    inner2.insert("score".to_string(), 72);
    inner2.insert("level".to_string(), 1);
    inner2.insert("bonus".to_string(), 10);

    let mut original: HashMap<u32, HashMap<String, u32>> = HashMap::new();
    original.insert(1001, inner1);
    original.insert(1002, inner2);
    original.insert(1003, HashMap::new());

    let enc = encode_to_vec(&original).expect("encode nested HashMap");
    #[allow(clippy::type_complexity)]
    let (val, consumed): (HashMap<u32, HashMap<String, u32>>, _) =
        decode_from_slice(&enc).expect("decode nested HashMap");

    assert_eq!(original, val);
    assert_eq!(val[&1001]["score"], 95);
    assert_eq!(val[&1001]["level"], 3);
    assert_eq!(val[&1002]["bonus"], 10);
    assert!(val[&1003].is_empty());
    assert_eq!(consumed, enc.len());
}

// ─── 16. BTreeMap<String, BTreeSet<u32>> nested roundtrip ────────────────────

#[test]
fn test_mac3_nested_btreemap_string_to_btreeset_u32() {
    let mut original: BTreeMap<String, BTreeSet<u32>> = BTreeMap::new();
    original.insert(
        "primes".to_string(),
        [2u32, 3, 5, 7, 11].iter().copied().collect(),
    );
    original.insert(
        "evens".to_string(),
        [0u32, 2, 4, 6, 8, 10].iter().copied().collect(),
    );
    original.insert("empty".to_string(), BTreeSet::new());

    let enc = encode_to_vec(&original).expect("encode BTreeMap<String,BTreeSet<u32>>");
    let (val, consumed): (BTreeMap<String, BTreeSet<u32>>, _) =
        decode_from_slice(&enc).expect("decode BTreeMap<String,BTreeSet<u32>>");

    assert_eq!(original, val);
    assert!(val["primes"].contains(&5));
    assert!(!val["primes"].contains(&4));
    assert!(val["evens"].contains(&10));
    assert!(val["empty"].is_empty());
    assert_eq!(consumed, enc.len());

    // BTreeSet within must also be sorted
    let prime_items: Vec<u32> = val["primes"].iter().copied().collect();
    assert_eq!(prime_items, vec![2, 3, 5, 7, 11]);
}

// ─── 17. HashMap re-decoded has all keys and values ──────────────────────────

#[test]
fn test_mac3_hashmap_all_keys_and_values_present_after_decode() {
    let entries: Vec<(String, u64)> = (0u64..20)
        .map(|i| (format!("key_{:02}", i), i * 1_000_000_007))
        .collect();

    let mut original: HashMap<String, u64> = HashMap::new();
    for (k, v) in &entries {
        original.insert(k.clone(), *v);
    }

    let enc = encode_to_vec(&original).expect("encode HashMap for all-keys check");
    let (val, _): (HashMap<String, u64>, _) =
        decode_from_slice(&enc).expect("decode HashMap for all-keys check");

    assert_eq!(val.len(), entries.len());

    // Every original key must map to its expected value
    for (k, v) in &entries {
        let decoded_v = val.get(k).expect("key must be present after decode");
        assert_eq!(*decoded_v, *v, "value mismatch for key {}", k);
    }

    // No extra keys
    for k in val.keys() {
        assert!(
            original.contains_key(k.as_str()),
            "unexpected key in decoded HashMap: {}",
            k
        );
    }
}

// ─── 18. BTreeSet decoded in same order as original ──────────────────────────

#[test]
fn test_mac3_btreeset_decoded_in_same_order_as_original() {
    let values = [99u32, 1, 50, 7, 200, 3, 42, 0];
    let original: BTreeSet<u32> = values.iter().copied().collect();

    let enc = encode_to_vec(&original).expect("encode BTreeSet order check");
    let (val, consumed): (BTreeSet<u32>, _) =
        decode_from_slice(&enc).expect("decode BTreeSet order check");

    assert_eq!(original, val);
    assert_eq!(consumed, enc.len());

    // Both the original and decoded BTreeSets must produce the same sorted sequence
    let orig_items: Vec<u32> = original.iter().copied().collect();
    let val_items: Vec<u32> = val.iter().copied().collect();
    assert_eq!(
        orig_items, val_items,
        "BTreeSet iteration order must match after decode"
    );

    // Confirm the sequence is actually sorted ascending
    let mut sorted = val_items.clone();
    sorted.sort();
    assert_eq!(val_items, sorted);
}

// ─── 19. HashMap consumed bytes == encoded length ────────────────────────────

#[test]
fn test_mac3_hashmap_consumed_bytes_equals_encoded_length() {
    let mut original: HashMap<u32, u32> = HashMap::new();
    for i in 0u32..25 {
        original.insert(i, u32::MAX - i);
    }

    let enc = encode_to_vec(&original).expect("encode HashMap consumed-bytes check");
    let (val, consumed): (HashMap<u32, u32>, _) =
        decode_from_slice(&enc).expect("decode HashMap consumed-bytes check");

    assert_eq!(original, val);
    assert_eq!(
        consumed,
        enc.len(),
        "consumed bytes must equal the total encoded length"
    );
}

// ─── 20. Option<HashMap<String, u32>> Some roundtrip ─────────────────────────

#[test]
fn test_mac3_option_hashmap_string_u32_some_roundtrip() {
    let mut inner: HashMap<String, u32> = HashMap::new();
    inner.insert("alpha".to_string(), 1);
    inner.insert("beta".to_string(), 2);

    let original: Option<HashMap<String, u32>> = Some(inner.clone());

    let enc = encode_to_vec(&original).expect("encode Option<HashMap<String,u32>>");
    let (val, consumed): (Option<HashMap<String, u32>>, _) =
        decode_from_slice(&enc).expect("decode Option<HashMap<String,u32>>");

    assert_eq!(original, val);
    let inner_val = val.expect("Option must be Some after decode");
    assert_eq!(inner_val["alpha"], 1);
    assert_eq!(inner_val["beta"], 2);
    assert_eq!(consumed, enc.len());

    // Also verify None encodes/decodes correctly
    let none_orig: Option<HashMap<String, u32>> = None;
    let enc_none = encode_to_vec(&none_orig).expect("encode None");
    let (val_none, consumed_none): (Option<HashMap<String, u32>>, _) =
        decode_from_slice(&enc_none).expect("decode None");
    assert_eq!(none_orig, val_none);
    assert_eq!(consumed_none, enc_none.len());
}

// ─── 21. Vec<BTreeMap<u32, String>> roundtrip ────────────────────────────────

#[test]
fn test_mac3_vec_of_btreemaps_roundtrip() {
    let mut map0: BTreeMap<u32, String> = BTreeMap::new();
    map0.insert(1, "first".to_string());
    map0.insert(2, "second".to_string());

    let map1: BTreeMap<u32, String> = BTreeMap::new(); // empty map in vector

    let mut map2: BTreeMap<u32, String> = BTreeMap::new();
    map2.insert(10, "ten".to_string());
    map2.insert(20, "twenty".to_string());
    map2.insert(30, "thirty".to_string());

    let original: Vec<BTreeMap<u32, String>> = vec![map0.clone(), map1, map2.clone()];

    let enc = encode_to_vec(&original).expect("encode Vec<BTreeMap<u32,String>>");
    let (val, consumed): (Vec<BTreeMap<u32, String>>, _) =
        decode_from_slice(&enc).expect("decode Vec<BTreeMap<u32,String>>");

    assert_eq!(original, val);
    assert_eq!(val.len(), 3);
    assert_eq!(val[0][&1], "first");
    assert!(val[1].is_empty());
    assert_eq!(val[2][&30], "thirty");
    assert_eq!(consumed, enc.len());
}

// ─── 22. Struct containing HashMap and BTreeSet fields roundtrip ──────────────

#[derive(Debug, PartialEq, Encode, Decode)]
struct MapHolder {
    labels: std::collections::HashMap<String, u32>,
    tags: std::collections::BTreeSet<String>,
}

#[test]
fn test_mac3_struct_with_hashmap_and_btreeset_fields_roundtrip() {
    let mut labels: HashMap<String, u32> = HashMap::new();
    labels.insert("version".to_string(), 2);
    labels.insert("priority".to_string(), 10);
    labels.insert("retry".to_string(), 3);

    let tags: BTreeSet<String> = ["stable", "production", "reviewed"]
        .iter()
        .map(|s| s.to_string())
        .collect();

    let original = MapHolder {
        labels: labels.clone(),
        tags: tags.clone(),
    };

    let enc = encode_to_vec(&original).expect("encode MapHolder struct");
    let (val, consumed): (MapHolder, _) = decode_from_slice(&enc).expect("decode MapHolder struct");

    assert_eq!(original, val);
    assert_eq!(val.labels["version"], 2);
    assert_eq!(val.labels["priority"], 10);
    assert_eq!(val.labels["retry"], 3);
    assert!(val.tags.contains("stable"));
    assert!(val.tags.contains("production"));
    assert!(!val.tags.contains("deprecated"));
    assert_eq!(consumed, enc.len());

    // BTreeSet inside struct must still be sorted
    let tag_items: Vec<&str> = val.tags.iter().map(String::as_str).collect();
    assert_eq!(tag_items, vec!["production", "reviewed", "stable"]);
}
