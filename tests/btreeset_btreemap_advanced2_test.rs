//! Advanced BTreeSet<T> and BTreeMap<K,V> serialization tests for OxiCode.

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

// ── Test 1: BTreeSet<u32> empty roundtrip ─────────────────────────────────────

#[test]
fn test_btreeset_u32_empty_roundtrip() {
    let original: BTreeSet<u32> = BTreeSet::new();
    let encoded = encode_to_vec(&original).expect("encode empty BTreeSet<u32> failed");
    let (decoded, _consumed): (BTreeSet<u32>, usize) =
        decode_from_slice(&encoded).expect("decode empty BTreeSet<u32> failed");
    assert_eq!(decoded, original);
    assert!(decoded.is_empty());
}

// ── Test 2: BTreeSet<u32> with 3 elements roundtrip ──────────────────────────

#[test]
fn test_btreeset_u32_three_elements_roundtrip() {
    let mut original: BTreeSet<u32> = BTreeSet::new();
    original.insert(10);
    original.insert(20);
    original.insert(30);
    let encoded = encode_to_vec(&original).expect("encode BTreeSet<u32> 3 elements failed");
    let (decoded, _consumed): (BTreeSet<u32>, usize) =
        decode_from_slice(&encoded).expect("decode BTreeSet<u32> 3 elements failed");
    assert_eq!(decoded, original);
    assert_eq!(decoded.len(), 3);
}

// ── Test 3: BTreeSet<String> roundtrip ───────────────────────────────────────

#[test]
fn test_btreeset_string_roundtrip() {
    let mut original: BTreeSet<String> = BTreeSet::new();
    original.insert("alpha".to_string());
    original.insert("beta".to_string());
    original.insert("gamma".to_string());
    original.insert("delta".to_string());
    let encoded = encode_to_vec(&original).expect("encode BTreeSet<String> failed");
    let (decoded, _consumed): (BTreeSet<String>, usize) =
        decode_from_slice(&encoded).expect("decode BTreeSet<String> failed");
    assert_eq!(decoded, original);
}

// ── Test 4: BTreeSet<u32> consumed equals encoded length ─────────────────────

#[test]
fn test_btreeset_u32_consumed_equals_encoded_length() {
    let mut original: BTreeSet<u32> = BTreeSet::new();
    original.insert(1);
    original.insert(2);
    original.insert(3);
    original.insert(4);
    original.insert(5);
    let encoded = encode_to_vec(&original).expect("encode BTreeSet<u32> failed");
    let (_decoded, consumed): (BTreeSet<u32>, usize) =
        decode_from_slice(&encoded).expect("decode BTreeSet<u32> failed");
    assert_eq!(consumed, encoded.len());
}

// ── Test 5: BTreeSet<u32> with fixed-int config ───────────────────────────────

#[test]
fn test_btreeset_u32_fixed_int_config() {
    let cfg = config::standard().with_fixed_int_encoding();
    let mut original: BTreeSet<u32> = BTreeSet::new();
    original.insert(100);
    original.insert(200);
    original.insert(300);
    let encoded =
        encode_to_vec_with_config(&original, cfg).expect("encode BTreeSet<u32> fixed-int failed");
    let (decoded, consumed): (BTreeSet<u32>, usize) = decode_from_slice_with_config(&encoded, cfg)
        .expect("decode BTreeSet<u32> fixed-int failed");
    assert_eq!(decoded, original);
    assert_eq!(consumed, encoded.len());
}

// ── Test 6: BTreeMap<String, u32> empty roundtrip ────────────────────────────

#[test]
fn test_btreemap_string_u32_empty_roundtrip() {
    let original: BTreeMap<String, u32> = BTreeMap::new();
    let encoded = encode_to_vec(&original).expect("encode empty BTreeMap<String, u32> failed");
    let (decoded, _consumed): (BTreeMap<String, u32>, usize) =
        decode_from_slice(&encoded).expect("decode empty BTreeMap<String, u32> failed");
    assert_eq!(decoded, original);
    assert!(decoded.is_empty());
}

// ── Test 7: BTreeMap<String, u32> with 3 entries roundtrip ───────────────────

#[test]
fn test_btreemap_string_u32_three_entries_roundtrip() {
    let mut original: BTreeMap<String, u32> = BTreeMap::new();
    original.insert("one".to_string(), 1);
    original.insert("two".to_string(), 2);
    original.insert("three".to_string(), 3);
    let encoded = encode_to_vec(&original).expect("encode BTreeMap<String, u32> 3 entries failed");
    let (decoded, _consumed): (BTreeMap<String, u32>, usize) =
        decode_from_slice(&encoded).expect("decode BTreeMap<String, u32> 3 entries failed");
    assert_eq!(decoded, original);
    assert_eq!(decoded.len(), 3);
}

// ── Test 8: BTreeMap<u32, String> roundtrip ──────────────────────────────────

#[test]
fn test_btreemap_u32_string_roundtrip() {
    let mut original: BTreeMap<u32, String> = BTreeMap::new();
    original.insert(1, "first".to_string());
    original.insert(2, "second".to_string());
    original.insert(3, "third".to_string());
    original.insert(100, "hundredth".to_string());
    let encoded = encode_to_vec(&original).expect("encode BTreeMap<u32, String> failed");
    let (decoded, _consumed): (BTreeMap<u32, String>, usize) =
        decode_from_slice(&encoded).expect("decode BTreeMap<u32, String> failed");
    assert_eq!(decoded, original);
}

// ── Test 9: BTreeMap<String, Vec<u8>> roundtrip ──────────────────────────────

#[test]
fn test_btreemap_string_vec_u8_roundtrip() {
    let mut original: BTreeMap<String, Vec<u8>> = BTreeMap::new();
    original.insert("empty".to_string(), vec![]);
    original.insert("bytes".to_string(), vec![0x01, 0x02, 0x03, 0xff]);
    original.insert("single".to_string(), vec![42]);
    let encoded = encode_to_vec(&original).expect("encode BTreeMap<String, Vec<u8>> failed");
    let (decoded, _consumed): (BTreeMap<String, Vec<u8>>, usize) =
        decode_from_slice(&encoded).expect("decode BTreeMap<String, Vec<u8>> failed");
    assert_eq!(decoded, original);
}

// ── Test 10: BTreeMap<u32, u64> consumed equals encoded length ────────────────

#[test]
fn test_btreemap_u32_u64_consumed_equals_encoded_length() {
    let mut original: BTreeMap<u32, u64> = BTreeMap::new();
    original.insert(1, 100);
    original.insert(2, 200);
    original.insert(3, 300);
    let encoded = encode_to_vec(&original).expect("encode BTreeMap<u32, u64> failed");
    let (_decoded, consumed): (BTreeMap<u32, u64>, usize) =
        decode_from_slice(&encoded).expect("decode BTreeMap<u32, u64> failed");
    assert_eq!(consumed, encoded.len());
}

// ── Test 11: BTreeMap<u32, u32> with fixed-int config ────────────────────────

#[test]
fn test_btreemap_u32_u32_fixed_int_config() {
    let cfg = config::standard().with_fixed_int_encoding();
    let mut original: BTreeMap<u32, u32> = BTreeMap::new();
    original.insert(10, 100);
    original.insert(20, 200);
    original.insert(30, 300);
    let encoded = encode_to_vec_with_config(&original, cfg)
        .expect("encode BTreeMap<u32, u32> fixed-int failed");
    let (decoded, consumed): (BTreeMap<u32, u32>, usize) =
        decode_from_slice_with_config(&encoded, cfg)
            .expect("decode BTreeMap<u32, u32> fixed-int failed");
    assert_eq!(decoded, original);
    assert_eq!(consumed, encoded.len());
}

// ── Test 12: BTreeSet<i32> with negative values roundtrip ────────────────────

#[test]
fn test_btreeset_i32_negative_values_roundtrip() {
    let mut original: BTreeSet<i32> = BTreeSet::new();
    original.insert(-100);
    original.insert(-50);
    original.insert(-1);
    original.insert(0);
    original.insert(1);
    original.insert(50);
    let encoded = encode_to_vec(&original).expect("encode BTreeSet<i32> with negatives failed");
    let (decoded, _consumed): (BTreeSet<i32>, usize) =
        decode_from_slice(&encoded).expect("decode BTreeSet<i32> with negatives failed");
    assert_eq!(decoded, original);
    assert!(decoded.contains(&-100));
    assert!(decoded.contains(&0));
}

// ── Test 13: BTreeMap<String, String> roundtrip (3 entries) ──────────────────

#[test]
fn test_btreemap_string_string_roundtrip() {
    let mut original: BTreeMap<String, String> = BTreeMap::new();
    original.insert("key_a".to_string(), "value_a".to_string());
    original.insert("key_b".to_string(), "value_b".to_string());
    original.insert("key_c".to_string(), "value_c".to_string());
    let encoded = encode_to_vec(&original).expect("encode BTreeMap<String, String> failed");
    let (decoded, _consumed): (BTreeMap<String, String>, usize) =
        decode_from_slice(&encoded).expect("decode BTreeMap<String, String> failed");
    assert_eq!(decoded, original);
    assert_eq!(decoded.get("key_b"), Some(&"value_b".to_string()));
}

// ── Test 14: BTreeSet<u8> full byte range subset roundtrip ───────────────────

#[test]
fn test_btreeset_u8_byte_range_subset_roundtrip() {
    let original: BTreeSet<u8> = (0u8..=127).collect();
    let encoded = encode_to_vec(&original).expect("encode BTreeSet<u8> byte range subset failed");
    let (decoded, consumed): (BTreeSet<u8>, usize) =
        decode_from_slice(&encoded).expect("decode BTreeSet<u8> byte range subset failed");
    assert_eq!(decoded, original);
    assert_eq!(decoded.len(), 128);
    assert_eq!(consumed, encoded.len());
}

// ── Test 15: BTreeMap<u8, u8> multiple entries roundtrip ─────────────────────

#[test]
fn test_btreemap_u8_u8_multiple_entries_roundtrip() {
    let mut original: BTreeMap<u8, u8> = BTreeMap::new();
    for i in 0u8..16 {
        original.insert(i, i * 2);
    }
    let encoded = encode_to_vec(&original).expect("encode BTreeMap<u8, u8> failed");
    let (decoded, consumed): (BTreeMap<u8, u8>, usize) =
        decode_from_slice(&encoded).expect("decode BTreeMap<u8, u8> failed");
    assert_eq!(decoded, original);
    assert_eq!(decoded.len(), 16);
    assert_eq!(consumed, encoded.len());
}

// ── Test 16: BTreeSet<u32> wire length matches BTreeVec equivalent ────────────

#[test]
fn test_btreeset_u32_wire_length_matches_vec_equivalent() {
    let mut set: BTreeSet<u32> = BTreeSet::new();
    set.insert(10);
    set.insert(20);
    set.insert(30);

    // Collect set elements in sorted order (BTreeSet iterates in order)
    let vec_equiv: Vec<u32> = set.iter().copied().collect();

    let set_encoded = encode_to_vec(&set).expect("encode BTreeSet<u32> failed");
    let vec_encoded = encode_to_vec(&vec_equiv).expect("encode Vec<u32> equivalent failed");

    // Both use the same length-prefixed wire format, so byte lengths must match
    assert_eq!(
        set_encoded.len(),
        vec_encoded.len(),
        "BTreeSet and sorted Vec must produce identical wire lengths"
    );
    assert_eq!(
        set_encoded, vec_encoded,
        "BTreeSet and sorted Vec must be byte-for-byte identical on the wire"
    );
}

// ── Test 17: BTreeMap<u32, Option<String>> roundtrip ─────────────────────────

#[test]
fn test_btreemap_u32_option_string_roundtrip() {
    let mut original: BTreeMap<u32, Option<String>> = BTreeMap::new();
    original.insert(1, Some("present".to_string()));
    original.insert(2, None);
    original.insert(3, Some("also present".to_string()));
    original.insert(4, None);
    let encoded = encode_to_vec(&original).expect("encode BTreeMap<u32, Option<String>> failed");
    let (decoded, consumed): (BTreeMap<u32, Option<String>>, usize) =
        decode_from_slice(&encoded).expect("decode BTreeMap<u32, Option<String>> failed");
    assert_eq!(decoded, original);
    assert_eq!(decoded.get(&2), Some(&None));
    assert_eq!(consumed, encoded.len());
}

// ── Test 18: BTreeSet<u64> large values roundtrip ────────────────────────────

#[test]
fn test_btreeset_u64_large_values_roundtrip() {
    let mut original: BTreeSet<u64> = BTreeSet::new();
    original.insert(u64::MAX);
    original.insert(u64::MAX - 1);
    original.insert(u64::MAX / 2);
    original.insert(1_000_000_000_000u64);
    original.insert(0u64);
    let encoded = encode_to_vec(&original).expect("encode BTreeSet<u64> large values failed");
    let (decoded, consumed): (BTreeSet<u64>, usize) =
        decode_from_slice(&encoded).expect("decode BTreeSet<u64> large values failed");
    assert_eq!(decoded, original);
    assert!(decoded.contains(&u64::MAX));
    assert!(decoded.contains(&0u64));
    assert_eq!(consumed, encoded.len());
}

// ── Test 19: BTreeMap<String, BTreeSet<u32>> nested roundtrip ────────────────

#[test]
fn test_btreemap_string_btreeset_u32_nested_roundtrip() {
    let mut original: BTreeMap<String, BTreeSet<u32>> = BTreeMap::new();

    let mut odds: BTreeSet<u32> = BTreeSet::new();
    odds.insert(1);
    odds.insert(3);
    odds.insert(5);
    odds.insert(7);

    let mut evens: BTreeSet<u32> = BTreeSet::new();
    evens.insert(2);
    evens.insert(4);
    evens.insert(6);
    evens.insert(8);

    let empty: BTreeSet<u32> = BTreeSet::new();

    original.insert("odds".to_string(), odds);
    original.insert("evens".to_string(), evens);
    original.insert("empty".to_string(), empty);

    let encoded = encode_to_vec(&original).expect("encode BTreeMap<String, BTreeSet<u32>> failed");
    let (decoded, consumed): (BTreeMap<String, BTreeSet<u32>>, usize) =
        decode_from_slice(&encoded).expect("decode BTreeMap<String, BTreeSet<u32>> failed");
    assert_eq!(decoded, original);
    assert_eq!(consumed, encoded.len());
    assert!(decoded
        .get("empty")
        .expect("empty key must exist")
        .is_empty());
}

// ── Test 20: BTreeSet<u32> insert-then-roundtrip preserves ordering ───────────

#[test]
fn test_btreeset_u32_insert_then_roundtrip_preserves_ordering() {
    let mut original: BTreeSet<u32> = BTreeSet::new();
    // Insert in reverse order to verify BTreeSet sorts automatically
    for i in (0u32..10).rev() {
        original.insert(i * 10);
    }
    let encoded = encode_to_vec(&original).expect("encode BTreeSet<u32> ordering test failed");
    let (decoded, _consumed): (BTreeSet<u32>, usize) =
        decode_from_slice(&encoded).expect("decode BTreeSet<u32> ordering test failed");

    let original_ordered: Vec<u32> = original.iter().copied().collect();
    let decoded_ordered: Vec<u32> = decoded.iter().copied().collect();

    assert_eq!(decoded_ordered, original_ordered);
    // Verify elements are in ascending order
    let is_sorted = decoded_ordered.windows(2).all(|w| w[0] < w[1]);
    assert!(
        is_sorted,
        "decoded BTreeSet<u32> elements must be in ascending order"
    );
}

// ── Test 21: BTreeMap<u32, Vec<u32>> roundtrip with vector values ─────────────

#[test]
fn test_btreemap_u32_vec_u32_roundtrip() {
    let mut original: BTreeMap<u32, Vec<u32>> = BTreeMap::new();
    original.insert(1, vec![]);
    original.insert(2, vec![10, 20, 30]);
    original.insert(3, vec![100, 200, 300, 400, 500]);
    original.insert(4, vec![1]);
    let encoded = encode_to_vec(&original).expect("encode BTreeMap<u32, Vec<u32>> failed");
    let (decoded, consumed): (BTreeMap<u32, Vec<u32>>, usize) =
        decode_from_slice(&encoded).expect("decode BTreeMap<u32, Vec<u32>> failed");
    assert_eq!(decoded, original);
    assert_eq!(consumed, encoded.len());
    assert!(decoded.get(&1).expect("key 1 must exist").is_empty());
    assert_eq!(decoded.get(&3).expect("key 3 must exist").len(), 5);
}

// ── Test 22: BTreeMap<String, u32> with big-endian config ────────────────────

#[test]
fn test_btreemap_string_u32_big_endian_config() {
    let cfg = config::standard().with_big_endian();
    let mut original: BTreeMap<String, u32> = BTreeMap::new();
    original.insert("alpha".to_string(), 1);
    original.insert("beta".to_string(), 2);
    original.insert("gamma".to_string(), 3);
    let encoded = encode_to_vec_with_config(&original, cfg)
        .expect("encode BTreeMap<String, u32> big-endian failed");
    let (decoded, consumed): (BTreeMap<String, u32>, usize) =
        decode_from_slice_with_config(&encoded, cfg)
            .expect("decode BTreeMap<String, u32> big-endian failed");
    assert_eq!(decoded, original);
    assert_eq!(consumed, encoded.len());
    assert_eq!(decoded.get("beta"), Some(&2u32));
}
