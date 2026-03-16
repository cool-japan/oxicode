//! Comprehensive tests for all map and set types

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
use oxicode::{Decode, Encode};
use std::collections::*;

// ─── 1. HashMap<String, u32> roundtrip ───────────────────────────────────────

#[test]
fn test_map_types_hashmap_string_u32_roundtrip() {
    let mut original: HashMap<String, u32> = HashMap::new();
    original.insert("alpha".to_string(), 1);
    original.insert("beta".to_string(), 200);
    original.insert("gamma".to_string(), 30000);

    let bytes = oxicode::encode_to_vec(&original).expect("encode failed");
    let (decoded, consumed): (HashMap<String, u32>, _) =
        oxicode::decode_from_slice(&bytes).expect("decode failed");

    assert_eq!(original, decoded);
    assert_eq!(consumed, bytes.len());
}

// ─── 2. HashMap<u32, Vec<String>> roundtrip ──────────────────────────────────

#[test]
fn test_map_types_hashmap_u32_vec_string_roundtrip() {
    let mut original: HashMap<u32, Vec<String>> = HashMap::new();
    original.insert(1, vec!["hello".to_string(), "world".to_string()]);
    original.insert(2, vec!["foo".to_string()]);
    original.insert(99, vec![]);

    let bytes = oxicode::encode_to_vec(&original).expect("encode failed");
    let (decoded, _): (HashMap<u32, Vec<String>>, _) =
        oxicode::decode_from_slice(&bytes).expect("decode failed");

    assert_eq!(original, decoded);
}

// ─── 3. BTreeMap<String, u64> roundtrip ──────────────────────────────────────

#[test]
fn test_map_types_btreemap_string_u64_roundtrip() {
    let mut original: BTreeMap<String, u64> = BTreeMap::new();
    original.insert("apple".to_string(), u64::MAX);
    original.insert("banana".to_string(), 0);
    original.insert("cherry".to_string(), 123_456_789_012_345);

    let bytes = oxicode::encode_to_vec(&original).expect("encode failed");
    let (decoded, consumed): (BTreeMap<String, u64>, _) =
        oxicode::decode_from_slice(&bytes).expect("decode failed");

    assert_eq!(original, decoded);
    assert_eq!(consumed, bytes.len());
}

// ─── 4. BTreeMap<i32, String> – negative key roundtrip ───────────────────────

#[test]
fn test_map_types_btreemap_negative_keys_roundtrip() {
    let mut original: BTreeMap<i32, String> = BTreeMap::new();
    original.insert(-1000, "minus one thousand".to_string());
    original.insert(-1, "minus one".to_string());
    original.insert(0, "zero".to_string());
    original.insert(1, "one".to_string());
    original.insert(1000, "one thousand".to_string());

    let bytes = oxicode::encode_to_vec(&original).expect("encode failed");
    let (decoded, _): (BTreeMap<i32, String>, _) =
        oxicode::decode_from_slice(&bytes).expect("decode failed");

    assert_eq!(original, decoded);

    // Verify negative keys specifically survived
    assert_eq!(decoded[&-1000], "minus one thousand");
    assert_eq!(decoded[&-1], "minus one");
}

// ─── 5. HashSet<String> roundtrip ────────────────────────────────────────────

#[test]
fn test_map_types_hashset_string_roundtrip() {
    let mut original: HashSet<String> = HashSet::new();
    original.insert("rust".to_string());
    original.insert("is".to_string());
    original.insert("awesome".to_string());

    let bytes = oxicode::encode_to_vec(&original).expect("encode failed");
    let (decoded, _): (HashSet<String>, _) =
        oxicode::decode_from_slice(&bytes).expect("decode failed");

    assert_eq!(original, decoded);
}

// ─── 6. BTreeSet<u32> roundtrip ──────────────────────────────────────────────

#[test]
fn test_map_types_btreeset_u32_roundtrip() {
    let mut original: BTreeSet<u32> = BTreeSet::new();
    original.insert(100);
    original.insert(1);
    original.insert(50);
    original.insert(200);
    original.insert(25);

    let bytes = oxicode::encode_to_vec(&original).expect("encode failed");
    let (decoded, _): (BTreeSet<u32>, _) =
        oxicode::decode_from_slice(&bytes).expect("decode failed");

    assert_eq!(original, decoded);
}

// ─── 7. Empty HashMap roundtrip ──────────────────────────────────────────────

#[test]
fn test_map_types_empty_hashmap_roundtrip() {
    let original: HashMap<String, u32> = HashMap::new();

    let bytes = oxicode::encode_to_vec(&original).expect("encode failed");
    let (decoded, consumed): (HashMap<String, u32>, _) =
        oxicode::decode_from_slice(&bytes).expect("decode failed");

    assert_eq!(original, decoded);
    assert!(decoded.is_empty());
    assert_eq!(consumed, bytes.len());
}

// ─── 8. Large HashMap (1 000 entries) ────────────────────────────────────────

#[test]
fn test_map_types_large_hashmap_roundtrip() {
    let mut original: HashMap<String, u32> = HashMap::with_capacity(1000);
    for i in 0u32..1000 {
        original.insert(format!("key_{:04}", i), i * 7);
    }

    let bytes = oxicode::encode_to_vec(&original).expect("encode failed");
    let (decoded, consumed): (HashMap<String, u32>, _) =
        oxicode::decode_from_slice(&bytes).expect("decode failed");

    assert_eq!(original, decoded);
    assert_eq!(decoded.len(), 1000);
    assert_eq!(consumed, bytes.len());

    // Spot-check a handful of entries
    assert_eq!(decoded["key_0000"], 0);
    assert_eq!(decoded["key_0007"], 49);
    assert_eq!(decoded["key_0999"], 6993);
}

// ─── 9. Nested: HashMap<String, HashMap<u32, bool>> ──────────────────────────

#[test]
fn test_map_types_nested_hashmap_roundtrip() {
    let mut inner_a: HashMap<u32, bool> = HashMap::new();
    inner_a.insert(1, true);
    inner_a.insert(2, false);
    inner_a.insert(3, true);

    let mut inner_b: HashMap<u32, bool> = HashMap::new();
    inner_b.insert(10, false);
    inner_b.insert(20, true);

    let mut original: HashMap<String, HashMap<u32, bool>> = HashMap::new();
    original.insert("flags_a".to_string(), inner_a);
    original.insert("flags_b".to_string(), inner_b);
    original.insert("flags_empty".to_string(), HashMap::new());

    let bytes = oxicode::encode_to_vec(&original).expect("encode failed");
    let (decoded, _): (HashMap<String, HashMap<u32, bool>>, _) =
        oxicode::decode_from_slice(&bytes).expect("decode failed");

    assert_eq!(original, decoded);
    assert!(decoded["flags_a"][&1]);
    assert!(!decoded["flags_a"][&2]);
    assert!(!decoded["flags_b"][&10]);
    assert!(decoded["flags_empty"].is_empty());
}

// ─── 10. HashMap with big-endian config ──────────────────────────────────────

#[test]
fn test_map_types_hashmap_big_endian_config() {
    let config = oxicode::config::standard()
        .with_big_endian()
        .with_fixed_int_encoding();

    let mut original: HashMap<String, u32> = HashMap::new();
    original.insert("one".to_string(), 1);
    original.insert("two".to_string(), 2);
    original.insert("three".to_string(), 3);

    let bytes =
        oxicode::encode_to_vec_with_config(&original, config).expect("encode with config failed");
    let (decoded, consumed): (HashMap<String, u32>, _) =
        oxicode::decode_from_slice_with_config(&bytes, config).expect("decode with config failed");

    assert_eq!(original, decoded);
    assert_eq!(consumed, bytes.len());

    // Encoding with standard (little-endian) config must produce different bytes
    let bytes_le = oxicode::encode_to_vec(&original).expect("encode le failed");
    // The bytes will differ when u32 values are encoded with fixed big-endian vs. varint little-endian
    assert_ne!(
        bytes, bytes_le,
        "big-endian fixed and little-endian varint encodings should differ"
    );
}

// ─── 11. HashMap insertion-order independence for decode ─────────────────────

#[test]
fn test_map_types_hashmap_insertion_order_independence() {
    // Build two HashMaps with the same key-value pairs but different insertion orders.
    let mut map_a: HashMap<String, u32> = HashMap::new();
    map_a.insert("x".to_string(), 10);
    map_a.insert("y".to_string(), 20);
    map_a.insert("z".to_string(), 30);

    let mut map_b: HashMap<String, u32> = HashMap::new();
    map_b.insert("z".to_string(), 30);
    map_b.insert("x".to_string(), 10);
    map_b.insert("y".to_string(), 20);

    let bytes_a = oxicode::encode_to_vec(&map_a).expect("encode a failed");
    let bytes_b = oxicode::encode_to_vec(&map_b).expect("encode b failed");

    let (decoded_a, _): (HashMap<String, u32>, _) =
        oxicode::decode_from_slice(&bytes_a).expect("decode a failed");
    let (decoded_b, _): (HashMap<String, u32>, _) =
        oxicode::decode_from_slice(&bytes_b).expect("decode b failed");

    // Both must decode to logically equivalent maps regardless of wire order
    assert_eq!(decoded_a, decoded_b);
    assert_eq!(decoded_a["x"], 10);
    assert_eq!(decoded_a["y"], 20);
    assert_eq!(decoded_a["z"], 30);
}

// ─── 12. BTreeMap sorted order preservation ──────────────────────────────────

#[test]
fn test_map_types_btreemap_sorted_order_preservation() {
    let mut original: BTreeMap<u32, String> = BTreeMap::new();
    // Insert in reverse order to confirm BTreeMap sorts on its own
    for i in (0u32..10).rev() {
        original.insert(i, format!("val_{}", i));
    }

    let bytes = oxicode::encode_to_vec(&original).expect("encode failed");
    let (decoded, _): (BTreeMap<u32, String>, _) =
        oxicode::decode_from_slice(&bytes).expect("decode failed");

    assert_eq!(original, decoded);

    // Verify the decoded BTreeMap iterates in ascending key order
    let keys: Vec<u32> = decoded.keys().copied().collect();
    let mut sorted = keys.clone();
    sorted.sort();
    assert_eq!(keys, sorted, "BTreeMap keys must be in ascending order");

    // Verify values match their keys
    for (k, v) in &decoded {
        assert_eq!(*v, format!("val_{}", k));
    }
}

// ─── 13. HashSet membership after roundtrip ──────────────────────────────────

#[test]
fn test_map_types_hashset_membership_after_roundtrip() {
    let members = ["alpha", "beta", "gamma", "delta", "epsilon"];
    let original: HashSet<String> = members.iter().map(|s| s.to_string()).collect();

    let bytes = oxicode::encode_to_vec(&original).expect("encode failed");
    let (decoded, _): (HashSet<String>, _) =
        oxicode::decode_from_slice(&bytes).expect("decode failed");

    assert_eq!(decoded.len(), members.len());

    // Every original member must be present
    for member in &members {
        assert!(
            decoded.contains(*member),
            "decoded HashSet missing member: {}",
            member
        );
    }

    // No phantom members
    let not_members = ["zeta", "eta", "theta"];
    for non_member in &not_members {
        assert!(
            !decoded.contains(*non_member),
            "decoded HashSet unexpectedly contains: {}",
            non_member
        );
    }
}

// ─── 14. BTreeSet sorted membership after roundtrip ─────────────────────────

#[test]
fn test_map_types_btreeset_sorted_membership_after_roundtrip() {
    let values: Vec<u32> = vec![42, 7, 100, 3, 55, 1];
    let original: BTreeSet<u32> = values.iter().copied().collect();

    let bytes = oxicode::encode_to_vec(&original).expect("encode failed");
    let (decoded, _): (BTreeSet<u32>, _) =
        oxicode::decode_from_slice(&bytes).expect("decode failed");

    assert_eq!(decoded.len(), original.len());

    // All original values must be present
    for v in &values {
        assert!(decoded.contains(v), "decoded BTreeSet missing value: {}", v);
    }

    // BTreeSet must iterate in ascending order
    let items: Vec<u32> = decoded.iter().copied().collect();
    let mut sorted = items.clone();
    sorted.sort();
    assert_eq!(items, sorted, "BTreeSet must iterate in ascending order");

    // First and last elements
    assert_eq!(*decoded.iter().next().expect("first element"), 1);
    assert_eq!(*decoded.iter().next_back().expect("last element"), 100);
}

// ─── Compile-time check: traits are in scope ─────────────────────────────────

fn _assert_encode_decode_in_scope<T: Encode + Decode>() {}

fn _assert_traits() {
    _assert_encode_decode_in_scope::<u32>();
    _assert_encode_decode_in_scope::<String>();
}
