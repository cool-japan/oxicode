//! Advanced HashMap/HashSet serialization tests — second batch (22 tests, new angles).

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
    config,
    decode_from_slice,
    decode_from_slice_with_config,
    encode_to_vec,
    encode_to_vec_with_config,
    Decode,
    Encode, // traits needed for Registry derive
};
use std::collections::{BTreeMap, BTreeSet, HashMap, HashSet};

// ─── 1. Empty HashMap<String, u32> roundtrip ─────────────────────────────────

#[test]
fn test_ha2_empty_hashmap_string_u32() {
    let original: HashMap<String, u32> = HashMap::new();

    let bytes = encode_to_vec(&original).expect("encode empty HashMap<String,u32>");
    let (decoded, consumed): (HashMap<String, u32>, _) =
        decode_from_slice(&bytes).expect("decode empty HashMap<String,u32>");

    assert_eq!(original, decoded);
    assert!(decoded.is_empty());
    assert_eq!(consumed, bytes.len());
}

// ─── 2. Single-entry HashMap<String, u32> ────────────────────────────────────

#[test]
fn test_ha2_single_entry_hashmap_string_u32() {
    let mut original: HashMap<String, u32> = HashMap::new();
    original.insert("only".to_string(), 42);

    let bytes = encode_to_vec(&original).expect("encode single-entry HashMap");
    let (decoded, consumed): (HashMap<String, u32>, _) =
        decode_from_slice(&bytes).expect("decode single-entry HashMap");

    assert_eq!(original, decoded);
    assert_eq!(decoded.len(), 1);
    assert_eq!(decoded["only"], 42);
    assert_eq!(consumed, bytes.len());
}

// ─── 3. Multi-entry HashMap<u32, String> (5 entries) ─────────────────────────

#[test]
fn test_ha2_multi_entry_hashmap_u32_string() {
    let mut original: HashMap<u32, String> = HashMap::new();
    original.insert(1, "one".to_string());
    original.insert(2, "two".to_string());
    original.insert(3, "three".to_string());
    original.insert(100, "hundred".to_string());
    original.insert(u32::MAX, "max".to_string());

    let bytes = encode_to_vec(&original).expect("encode 5-entry HashMap<u32,String>");
    let (decoded, consumed): (HashMap<u32, String>, _) =
        decode_from_slice(&bytes).expect("decode 5-entry HashMap<u32,String>");

    assert_eq!(original, decoded);
    assert_eq!(decoded.len(), 5);
    assert_eq!(decoded[&1], "one");
    assert_eq!(decoded[&u32::MAX], "max");
    assert_eq!(consumed, bytes.len());
}

// ─── 4. HashMap<u8, u8> with all distinct values ─────────────────────────────

#[test]
fn test_ha2_hashmap_u8_u8_distinct_values() {
    let mut original: HashMap<u8, u8> = HashMap::new();
    for i in 0u8..=15 {
        original.insert(i, 255 - i);
    }

    let bytes = encode_to_vec(&original).expect("encode HashMap<u8,u8>");
    let (decoded, consumed): (HashMap<u8, u8>, _) =
        decode_from_slice(&bytes).expect("decode HashMap<u8,u8>");

    assert_eq!(original, decoded);
    assert_eq!(decoded.len(), 16);
    assert_eq!(decoded[&0], 255);
    assert_eq!(decoded[&15], 240);
    assert_eq!(consumed, bytes.len());
}

// ─── 5. HashMap<String, Vec<u32>> ────────────────────────────────────────────

#[test]
fn test_ha2_hashmap_string_vec_u32() {
    let mut original: HashMap<String, Vec<u32>> = HashMap::new();
    original.insert("primes".to_string(), vec![2, 3, 5, 7, 11, 13]);
    original.insert("squares".to_string(), vec![1, 4, 9, 16, 25]);
    original.insert("empty".to_string(), vec![]);

    let bytes = encode_to_vec(&original).expect("encode HashMap<String,Vec<u32>>");
    let (decoded, consumed): (HashMap<String, Vec<u32>>, _) =
        decode_from_slice(&bytes).expect("decode HashMap<String,Vec<u32>>");

    assert_eq!(original, decoded);
    assert_eq!(decoded["primes"].len(), 6);
    assert_eq!(decoded["squares"][0], 1);
    assert!(decoded["empty"].is_empty());
    assert_eq!(consumed, bytes.len());
}

// ─── 6. HashMap<u64, bool> ───────────────────────────────────────────────────

#[test]
fn test_ha2_hashmap_u64_bool() {
    let mut original: HashMap<u64, bool> = HashMap::new();
    original.insert(0, false);
    original.insert(1, true);
    original.insert(u64::MAX, true);
    original.insert(u64::MAX / 2, false);

    let bytes = encode_to_vec(&original).expect("encode HashMap<u64,bool>");
    let (decoded, consumed): (HashMap<u64, bool>, _) =
        decode_from_slice(&bytes).expect("decode HashMap<u64,bool>");

    assert_eq!(original, decoded);
    assert!(!decoded[&0]);
    assert!(decoded[&1]);
    assert!(decoded[&u64::MAX]);
    assert_eq!(consumed, bytes.len());
}

// ─── 7. HashSet<u32> empty ───────────────────────────────────────────────────

#[test]
fn test_ha2_empty_hashset_u32() {
    let original: HashSet<u32> = HashSet::new();

    let bytes = encode_to_vec(&original).expect("encode empty HashSet<u32>");
    let (decoded, consumed): (HashSet<u32>, _) =
        decode_from_slice(&bytes).expect("decode empty HashSet<u32>");

    assert_eq!(original, decoded);
    assert!(decoded.is_empty());
    assert_eq!(consumed, bytes.len());
}

// ─── 8. HashSet<u32> with multiple values ────────────────────────────────────

#[test]
fn test_ha2_hashset_u32_multiple_values() {
    let original: HashSet<u32> = [10, 20, 30, 40, 50, 100, 999].iter().copied().collect();

    let bytes = encode_to_vec(&original).expect("encode HashSet<u32>");
    let (decoded, consumed): (HashSet<u32>, _) =
        decode_from_slice(&bytes).expect("decode HashSet<u32>");

    assert_eq!(original, decoded);
    assert_eq!(decoded.len(), 7);
    assert!(decoded.contains(&10));
    assert!(decoded.contains(&999));
    assert!(!decoded.contains(&0));
    assert_eq!(consumed, bytes.len());
}

// ─── 9. HashSet<String> roundtrip ────────────────────────────────────────────

#[test]
fn test_ha2_hashset_string_roundtrip() {
    let original: HashSet<String> = ["rust", "oxicode", "serialization", "binary"]
        .iter()
        .map(|s| s.to_string())
        .collect();

    let bytes = encode_to_vec(&original).expect("encode HashSet<String>");
    let (decoded, consumed): (HashSet<String>, _) =
        decode_from_slice(&bytes).expect("decode HashSet<String>");

    assert_eq!(original, decoded);
    assert!(decoded.contains("rust"));
    assert!(decoded.contains("oxicode"));
    assert!(!decoded.contains("python"));
    assert_eq!(consumed, bytes.len());
}

// ─── 10. HashSet<u8> full byte range subset ──────────────────────────────────

#[test]
fn test_ha2_hashset_u8_byte_range_subset() {
    // Every even byte value 0..=254 (128 elements)
    let original: HashSet<u8> = (0u8..=127).map(|i| i * 2).collect();

    let bytes = encode_to_vec(&original).expect("encode HashSet<u8> byte range");
    let (decoded, consumed): (HashSet<u8>, _) =
        decode_from_slice(&bytes).expect("decode HashSet<u8> byte range");

    assert_eq!(original, decoded);
    assert_eq!(decoded.len(), 128);
    assert!(decoded.contains(&0));
    assert!(decoded.contains(&254));
    assert!(!decoded.contains(&1));
    assert!(!decoded.contains(&255));
    assert_eq!(consumed, bytes.len());
}

// ─── 11. Nested HashMap<String, HashMap<u32, bool>> ──────────────────────────

#[test]
fn test_ha2_nested_hashmap_string_to_u32_bool() {
    let mut inner_a: HashMap<u32, bool> = HashMap::new();
    inner_a.insert(1, true);
    inner_a.insert(2, false);

    let mut inner_b: HashMap<u32, bool> = HashMap::new();
    inner_b.insert(100, true);
    inner_b.insert(200, true);
    inner_b.insert(300, false);

    let mut original: HashMap<String, HashMap<u32, bool>> = HashMap::new();
    original.insert("group_a".to_string(), inner_a);
    original.insert("group_b".to_string(), inner_b);
    original.insert("empty_group".to_string(), HashMap::new());

    let bytes = encode_to_vec(&original).expect("encode nested HashMap");
    #[allow(clippy::type_complexity)]
    let (decoded, consumed): (HashMap<String, HashMap<u32, bool>>, _) =
        decode_from_slice(&bytes).expect("decode nested HashMap");

    assert_eq!(original, decoded);
    assert!(decoded["group_a"][&1]);
    assert!(!decoded["group_a"][&2]);
    assert!(decoded["group_b"][&100]);
    assert!(!decoded["group_b"][&300]);
    assert!(decoded["empty_group"].is_empty());
    assert_eq!(consumed, bytes.len());
}

// ─── 12. HashMap<u32, Option<String>> ────────────────────────────────────────

#[test]
fn test_ha2_hashmap_u32_option_string() {
    let mut original: HashMap<u32, Option<String>> = HashMap::new();
    original.insert(1, Some("present".to_string()));
    original.insert(2, None);
    original.insert(3, Some(String::new()));
    original.insert(4, Some("another".to_string()));
    original.insert(5, None);

    let bytes = encode_to_vec(&original).expect("encode HashMap<u32,Option<String>>");
    let (decoded, consumed): (HashMap<u32, Option<String>>, _) =
        decode_from_slice(&bytes).expect("decode HashMap<u32,Option<String>>");

    assert_eq!(original, decoded);
    assert_eq!(decoded[&1], Some("present".to_string()));
    assert_eq!(decoded[&2], None);
    assert_eq!(decoded[&3], Some(String::new()));
    assert_eq!(consumed, bytes.len());
}

// ─── 13. Fixed-int config with HashMap<u32, u32> ─────────────────────────────

#[test]
fn test_ha2_hashmap_fixed_int_config() {
    let cfg = config::standard().with_fixed_int_encoding();

    let mut original: HashMap<u32, u32> = HashMap::new();
    original.insert(0, 0);
    original.insert(1, u32::MAX);
    original.insert(u32::MAX / 2, 12345);

    let bytes = encode_to_vec_with_config(&original, cfg).expect("encode with fixed-int config");
    let (decoded, consumed): (HashMap<u32, u32>, _) =
        decode_from_slice_with_config(&bytes, cfg).expect("decode with fixed-int config");

    assert_eq!(original, decoded);
    assert_eq!(decoded[&1], u32::MAX);
    assert_eq!(consumed, bytes.len());

    // 3 entries x (4 bytes key + 4 bytes value) = 24 bytes minimum (plus length prefix)
    assert!(
        bytes.len() >= 24,
        "expected at least 24 bytes for 3 fixed-int pairs"
    );
}

// ─── 14. Big-endian config with HashMap<u32, u32> ────────────────────────────

#[test]
fn test_ha2_hashmap_big_endian_config() {
    let cfg = config::standard()
        .with_big_endian()
        .with_fixed_int_encoding();

    let mut original: HashMap<u32, u32> = HashMap::new();
    original.insert(1, 0xDEAD_BEEF);
    original.insert(2, 0xCAFE_BABE);

    let bytes = encode_to_vec_with_config(&original, cfg).expect("encode big-endian");
    let (decoded, consumed): (HashMap<u32, u32>, _) =
        decode_from_slice_with_config(&bytes, cfg).expect("decode big-endian");

    assert_eq!(original, decoded);
    assert_eq!(decoded[&1], 0xDEAD_BEEF);
    assert_eq!(decoded[&2], 0xCAFE_BABE);
    assert_eq!(consumed, bytes.len());

    // Big-endian wire bytes for 0xDEADBEEF must appear verbatim
    let needle: &[u8] = &[0xDE, 0xAD, 0xBE, 0xEF];
    assert!(
        bytes.windows(4).any(|w| w == needle),
        "big-endian bytes for 0xDEADBEEF not found in encoded output"
    );
}

// ─── 15. HashMap with 100 entries — consumed == encoded.len() ────────────────

#[test]
fn test_ha2_hashmap_100_entries_consumed_equals_len() {
    let mut original: HashMap<u32, u64> = HashMap::with_capacity(100);
    for i in 0u32..100 {
        original.insert(i, u64::from(i) * 1_000_000_007);
    }

    let bytes = encode_to_vec(&original).expect("encode 100-entry HashMap");
    let (decoded, consumed): (HashMap<u32, u64>, _) =
        decode_from_slice(&bytes).expect("decode 100-entry HashMap");

    assert_eq!(original, decoded);
    assert_eq!(decoded.len(), 100);
    assert_eq!(
        consumed,
        bytes.len(),
        "consumed must equal total encoded length"
    );

    assert_eq!(decoded[&0], 0);
    assert_eq!(decoded[&1], 1_000_000_007);
    assert_eq!(decoded[&99], 99 * 1_000_000_007);
}

// ─── 16. BTreeMap<String, u32> — deterministic ordering ──────────────────────

#[test]
fn test_ha2_btreemap_string_u32_deterministic_order() {
    let mut original: BTreeMap<String, u32> = BTreeMap::new();
    // Insert out of lexicographic order
    original.insert("zebra".to_string(), 26);
    original.insert("apple".to_string(), 1);
    original.insert("mango".to_string(), 13);
    original.insert("banana".to_string(), 2);

    let bytes = encode_to_vec(&original).expect("encode BTreeMap<String,u32>");
    let (decoded, consumed): (BTreeMap<String, u32>, _) =
        decode_from_slice(&bytes).expect("decode BTreeMap<String,u32>");

    assert_eq!(original, decoded);
    assert_eq!(consumed, bytes.len());

    // BTreeMap must iterate in ascending lexicographic order
    let keys: Vec<&str> = decoded.keys().map(String::as_str).collect();
    assert_eq!(keys, vec!["apple", "banana", "mango", "zebra"]);
}

// ─── 17. BTreeSet<i32> roundtrip ─────────────────────────────────────────────

#[test]
fn test_ha2_btreeset_i32_roundtrip() {
    let original: BTreeSet<i32> = [-100, -1, 0, 1, 42, 127, i32::MIN, i32::MAX]
        .iter()
        .copied()
        .collect();

    let bytes = encode_to_vec(&original).expect("encode BTreeSet<i32>");
    let (decoded, consumed): (BTreeSet<i32>, _) =
        decode_from_slice(&bytes).expect("decode BTreeSet<i32>");

    assert_eq!(original, decoded);
    assert_eq!(consumed, bytes.len());

    // Ascending order check
    let items: Vec<i32> = decoded.iter().copied().collect();
    let mut sorted = items.clone();
    sorted.sort();
    assert_eq!(items, sorted);

    assert!(decoded.contains(&i32::MIN));
    assert!(decoded.contains(&i32::MAX));
}

// ─── 18. Empty HashSet<String> ───────────────────────────────────────────────

#[test]
fn test_ha2_empty_hashset_string() {
    let original: HashSet<String> = HashSet::new();

    let bytes = encode_to_vec(&original).expect("encode empty HashSet<String>");
    let (decoded, consumed): (HashSet<String>, _) =
        decode_from_slice(&bytes).expect("decode empty HashSet<String>");

    assert_eq!(original, decoded);
    assert!(decoded.is_empty());
    assert_eq!(consumed, bytes.len());
}

// ─── 19. HashMap<(u8, u8), u32> tuple keys ───────────────────────────────────

#[test]
fn test_ha2_hashmap_tuple_keys() {
    let mut original: HashMap<(u8, u8), u32> = HashMap::new();
    original.insert((0, 0), 0);
    original.insert((1, 2), 12);
    original.insert((255, 255), u32::MAX);
    original.insert((10, 20), 1020);

    let bytes = encode_to_vec(&original).expect("encode HashMap<(u8,u8),u32>");
    let (decoded, consumed): (HashMap<(u8, u8), u32>, _) =
        decode_from_slice(&bytes).expect("decode HashMap<(u8,u8),u32>");

    assert_eq!(original, decoded);
    assert_eq!(decoded[&(0, 0)], 0);
    assert_eq!(decoded[&(1, 2)], 12);
    assert_eq!(decoded[&(255, 255)], u32::MAX);
    assert_eq!(consumed, bytes.len());
}

// ─── 20. Struct with HashMap field ───────────────────────────────────────────

#[derive(Debug, PartialEq, Encode, Decode)]
struct Registry {
    name: String,
    entries: HashMap<u32, String>,
    version: u8,
}

#[test]
fn test_ha2_struct_with_hashmap_field() {
    let mut entries: HashMap<u32, String> = HashMap::new();
    entries.insert(1, "first".to_string());
    entries.insert(2, "second".to_string());
    entries.insert(99, "ninety-nine".to_string());

    let original = Registry {
        name: "test-registry".to_string(),
        entries,
        version: 3,
    };

    let bytes = encode_to_vec(&original).expect("encode Registry struct");
    let (decoded, consumed): (Registry, _) =
        decode_from_slice(&bytes).expect("decode Registry struct");

    assert_eq!(original, decoded);
    assert_eq!(decoded.name, "test-registry");
    assert_eq!(decoded.version, 3);
    assert_eq!(decoded.entries[&1], "first");
    assert_eq!(decoded.entries[&99], "ninety-nine");
    assert_eq!(consumed, bytes.len());
}

// ─── 21. HashMap<u32, Vec<u8>> — binary data values ─────────────────────────

#[test]
fn test_ha2_hashmap_u32_vec_u8_binary_data() {
    let mut original: HashMap<u32, Vec<u8>> = HashMap::new();
    original.insert(1, vec![0x00, 0xFF, 0xDE, 0xAD, 0xBE, 0xEF]);
    original.insert(2, (0u8..=255).collect());
    original.insert(3, vec![]);
    original.insert(4, vec![0x42]);

    let bytes = encode_to_vec(&original).expect("encode HashMap<u32,Vec<u8>>");
    let (decoded, consumed): (HashMap<u32, Vec<u8>>, _) =
        decode_from_slice(&bytes).expect("decode HashMap<u32,Vec<u8>>");

    assert_eq!(original, decoded);
    assert_eq!(decoded[&1], [0x00, 0xFF, 0xDE, 0xAD, 0xBE, 0xEF]);
    assert_eq!(decoded[&2].len(), 256);
    assert_eq!(decoded[&2][0], 0);
    assert_eq!(decoded[&2][255], 255);
    assert!(decoded[&3].is_empty());
    assert_eq!(decoded[&4], [0x42]);
    assert_eq!(consumed, bytes.len());
}

// ─── 22. HashSet<bool> (at most 2 elements: true, false) ─────────────────────

#[test]
fn test_ha2_hashset_bool_both_values() {
    let mut original: HashSet<bool> = HashSet::new();
    original.insert(true);
    original.insert(false);

    let bytes = encode_to_vec(&original).expect("encode HashSet<bool>");
    let (decoded, consumed): (HashSet<bool>, _) =
        decode_from_slice(&bytes).expect("decode HashSet<bool>");

    assert_eq!(original, decoded);
    assert_eq!(decoded.len(), 2);
    assert!(decoded.contains(&true));
    assert!(decoded.contains(&false));
    assert_eq!(consumed, bytes.len());

    // Verify a single-element HashSet<bool> also roundtrips correctly
    let only_true: HashSet<bool> = [true].iter().copied().collect();
    let bytes2 = encode_to_vec(&only_true).expect("encode HashSet<bool> single");
    let (decoded2, consumed2): (HashSet<bool>, _) =
        decode_from_slice(&bytes2).expect("decode HashSet<bool> single");

    assert_eq!(only_true, decoded2);
    assert_eq!(decoded2.len(), 1);
    assert!(decoded2.contains(&true));
    assert!(!decoded2.contains(&false));
    assert_eq!(consumed2, bytes2.len());
}
