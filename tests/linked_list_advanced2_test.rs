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
use std::collections::LinkedList;

// ── 1. LinkedList<u32> empty roundtrip ───────────────────────────────────────
#[test]
fn test_linkedlist_u32_empty_roundtrip() {
    let original: LinkedList<u32> = LinkedList::new();
    let encoded = encode_to_vec(&original).expect("encode LinkedList<u32> empty");
    let (decoded, _bytes): (LinkedList<u32>, usize) =
        decode_from_slice(&encoded).expect("decode LinkedList<u32> empty");
    assert_eq!(original, decoded);
    assert!(decoded.is_empty());
}

// ── 2. LinkedList<u32> single element roundtrip ───────────────────────────────
#[test]
fn test_linkedlist_u32_single_element_roundtrip() {
    let original: LinkedList<u32> = [42u32].iter().copied().collect();
    let encoded = encode_to_vec(&original).expect("encode LinkedList<u32> single");
    let (decoded, _bytes): (LinkedList<u32>, usize) =
        decode_from_slice(&encoded).expect("decode LinkedList<u32> single");
    assert_eq!(original, decoded);
    assert_eq!(decoded.len(), 1);
    assert_eq!(decoded.front().copied(), Some(42u32));
}

// ── 3. LinkedList<u32> 5 elements roundtrip ───────────────────────────────────
#[test]
fn test_linkedlist_u32_five_elements_roundtrip() {
    let original: LinkedList<u32> = [11u32, 22, 33, 44, 55].iter().copied().collect();
    let encoded = encode_to_vec(&original).expect("encode LinkedList<u32> five elements");
    let (decoded, _bytes): (LinkedList<u32>, usize) =
        decode_from_slice(&encoded).expect("decode LinkedList<u32> five elements");
    assert_eq!(original, decoded);
    assert_eq!(decoded.len(), 5);
}

// ── 4. LinkedList<String> roundtrip ──────────────────────────────────────────
#[test]
fn test_linkedlist_string_roundtrip() {
    let original: LinkedList<String> = ["foo", "bar", "baz", "qux"]
        .iter()
        .map(|s| s.to_string())
        .collect();
    let encoded = encode_to_vec(&original).expect("encode LinkedList<String>");
    let (decoded, _bytes): (LinkedList<String>, usize) =
        decode_from_slice(&encoded).expect("decode LinkedList<String>");
    assert_eq!(original, decoded);
}

// ── 5. LinkedList<Vec<u8>> roundtrip ─────────────────────────────────────────
#[test]
fn test_linkedlist_vec_u8_roundtrip() {
    let original: LinkedList<Vec<u8>> =
        vec![vec![0u8, 1, 2, 3], vec![], vec![255u8, 128, 64], vec![42u8]]
            .into_iter()
            .collect();
    let encoded = encode_to_vec(&original).expect("encode LinkedList<Vec<u8>>");
    let (decoded, _bytes): (LinkedList<Vec<u8>>, usize) =
        decode_from_slice(&encoded).expect("decode LinkedList<Vec<u8>>");
    assert_eq!(original, decoded);
}

// ── 6. LinkedList<u8> same wire bytes as Vec<u8> for same elements ────────────
#[test]
fn test_linkedlist_u8_same_wire_bytes_as_vec_u8() {
    let elements = [10u8, 20, 30, 40, 50];
    let vec_original: Vec<u8> = elements.to_vec();
    let list_original: LinkedList<u8> = elements.iter().copied().collect();
    let vec_encoded = encode_to_vec(&vec_original).expect("encode Vec<u8>");
    let list_encoded = encode_to_vec(&list_original).expect("encode LinkedList<u8>");
    assert_eq!(vec_encoded, list_encoded);
}

// ── 7. LinkedList<u32> consumed == encoded length ─────────────────────────────
#[test]
fn test_linkedlist_u32_consumed_equals_encoded_len() {
    let original: LinkedList<u32> = [100u32, 200, 300, 400].iter().copied().collect();
    let encoded = encode_to_vec(&original).expect("encode LinkedList<u32>");
    let (_decoded, consumed): (LinkedList<u32>, usize) =
        decode_from_slice(&encoded).expect("decode LinkedList<u32>");
    assert_eq!(consumed, encoded.len());
}

// ── 8. LinkedList<u32> element order preserved ────────────────────────────────
#[test]
fn test_linkedlist_u32_element_order_preserved() {
    let mut original: LinkedList<u32> = LinkedList::new();
    original.push_back(1);
    original.push_back(2);
    original.push_back(3);
    original.push_back(4);
    original.push_back(5);
    let encoded = encode_to_vec(&original).expect("encode LinkedList<u32> order");
    let (decoded, _bytes): (LinkedList<u32>, usize) =
        decode_from_slice(&encoded).expect("decode LinkedList<u32> order");
    let order: Vec<u32> = decoded.into_iter().collect();
    assert_eq!(order, vec![1u32, 2, 3, 4, 5]);
}

// ── 9. LinkedList<u64> with fixed-int config roundtrip ───────────────────────
#[test]
fn test_linkedlist_u64_fixed_int_config_roundtrip() {
    let cfg = config::standard().with_fixed_int_encoding();
    let original: LinkedList<u64> = [0u64, 1, u64::MAX / 2].iter().copied().collect();
    let encoded =
        encode_to_vec_with_config(&original, cfg).expect("encode LinkedList<u64> fixed-int");
    let (decoded, consumed): (LinkedList<u64>, usize) =
        decode_from_slice_with_config(&encoded, cfg).expect("decode LinkedList<u64> fixed-int");
    assert_eq!(original, decoded);
    assert_eq!(consumed, encoded.len());
    // u64 fixed = 8 bytes each; length prefix u64 = 8 bytes; 3 elements => 8 + 3*8 = 32
    assert_eq!(encoded.len(), 32);
}

// ── 10. LinkedList<u32> with big-endian config roundtrip ──────────────────────
#[test]
fn test_linkedlist_u32_big_endian_config_roundtrip() {
    let cfg = config::standard().with_big_endian();
    let original: LinkedList<u32> = [0xDEAD_u32, 0xBEEF, 0x1234].iter().copied().collect();
    let encoded =
        encode_to_vec_with_config(&original, cfg).expect("encode LinkedList<u32> big-endian");
    let (decoded, consumed): (LinkedList<u32>, usize) =
        decode_from_slice_with_config(&encoded, cfg).expect("decode LinkedList<u32> big-endian");
    assert_eq!(original, decoded);
    assert_eq!(consumed, encoded.len());
}

// ── 11. Option<LinkedList<u32>> Some roundtrip ────────────────────────────────
#[test]
fn test_option_linkedlist_u32_some_roundtrip() {
    let inner: LinkedList<u32> = [7u32, 8, 9].iter().copied().collect();
    let original: Option<LinkedList<u32>> = Some(inner);
    let encoded = encode_to_vec(&original).expect("encode Option<LinkedList<u32>> Some");
    let (decoded, _bytes): (Option<LinkedList<u32>>, usize) =
        decode_from_slice(&encoded).expect("decode Option<LinkedList<u32>> Some");
    assert_eq!(original, decoded);
    assert!(decoded.is_some());
}

// ── 12. Option<LinkedList<u32>> None roundtrip ────────────────────────────────
#[test]
fn test_option_linkedlist_u32_none_roundtrip() {
    let original: Option<LinkedList<u32>> = None;
    let encoded = encode_to_vec(&original).expect("encode Option<LinkedList<u32>> None");
    let (decoded, _bytes): (Option<LinkedList<u32>>, usize) =
        decode_from_slice(&encoded).expect("decode Option<LinkedList<u32>> None");
    assert_eq!(original, decoded);
    assert!(decoded.is_none());
}

// ── 13. Vec<LinkedList<u8>> roundtrip ─────────────────────────────────────────
#[test]
fn test_vec_of_linkedlist_u8_roundtrip() {
    let original: Vec<LinkedList<u8>> = vec![
        [1u8, 2, 3].iter().copied().collect(),
        LinkedList::new(),
        [100u8, 200].iter().copied().collect(),
    ];
    let encoded = encode_to_vec(&original).expect("encode Vec<LinkedList<u8>>");
    let (decoded, consumed): (Vec<LinkedList<u8>>, usize) =
        decode_from_slice(&encoded).expect("decode Vec<LinkedList<u8>>");
    assert_eq!(original, decoded);
    assert_eq!(consumed, encoded.len());
    assert_eq!(decoded.len(), 3);
    assert!(decoded[1].is_empty());
}

// ── 14. LinkedList<bool> roundtrip ────────────────────────────────────────────
#[test]
fn test_linkedlist_bool_roundtrip() {
    let original: LinkedList<bool> = [true, false, true, true, false].iter().copied().collect();
    let encoded = encode_to_vec(&original).expect("encode LinkedList<bool>");
    let (decoded, _bytes): (LinkedList<bool>, usize) =
        decode_from_slice(&encoded).expect("decode LinkedList<bool>");
    assert_eq!(original, decoded);
    let values: Vec<bool> = decoded.into_iter().collect();
    assert_eq!(values, vec![true, false, true, true, false]);
}

// ── 15. LinkedList<i32> with negative values roundtrip ───────────────────────
#[test]
fn test_linkedlist_i32_negative_values_roundtrip() {
    let original: LinkedList<i32> = [-999i32, -1, 0, 1, 999, i32::MIN, i32::MAX]
        .iter()
        .copied()
        .collect();
    let encoded = encode_to_vec(&original).expect("encode LinkedList<i32> negative");
    let (decoded, _bytes): (LinkedList<i32>, usize) =
        decode_from_slice(&encoded).expect("decode LinkedList<i32> negative");
    assert_eq!(original, decoded);
}

// ── 16. Struct { items: LinkedList<u32>, count: u32 } roundtrip ──────────────
#[derive(Debug, PartialEq, Encode, Decode)]
struct ItemCollection {
    items: LinkedList<u32>,
    count: u32,
}

#[test]
fn test_struct_with_linkedlist_roundtrip() {
    let items: LinkedList<u32> = [10u32, 20, 30, 40].iter().copied().collect();
    let original = ItemCollection {
        count: items.len() as u32,
        items,
    };
    let encoded = encode_to_vec(&original).expect("encode ItemCollection");
    let (decoded, _bytes): (ItemCollection, usize) =
        decode_from_slice(&encoded).expect("decode ItemCollection");
    assert_eq!(original, decoded);
    assert_eq!(decoded.count, 4);
    assert_eq!(decoded.items.len(), 4);
}

// ── 17. Large LinkedList (50 elements) roundtrip ─────────────────────────────
#[test]
fn test_linkedlist_large_50_elements_roundtrip() {
    let original: LinkedList<u32> = (0u32..50).collect();
    let encoded = encode_to_vec(&original).expect("encode large LinkedList<u32>");
    let (decoded, consumed): (LinkedList<u32>, usize) =
        decode_from_slice(&encoded).expect("decode large LinkedList<u32>");
    assert_eq!(original, decoded);
    assert_eq!(decoded.len(), 50);
    assert_eq!(consumed, encoded.len());
    let values: Vec<u32> = decoded.into_iter().collect();
    for (i, &v) in values.iter().enumerate() {
        assert_eq!(v, i as u32, "element at index {i} should be {i}");
    }
}

// ── 18. LinkedList<(u32, String)> tuple elements roundtrip ───────────────────
#[test]
fn test_linkedlist_tuple_elements_roundtrip() {
    let original: LinkedList<(u32, String)> = vec![
        (1u32, "first".to_string()),
        (2, "second".to_string()),
        (3, "third".to_string()),
    ]
    .into_iter()
    .collect();
    let encoded = encode_to_vec(&original).expect("encode LinkedList<(u32, String)>");
    let (decoded, _bytes): (LinkedList<(u32, String)>, usize) =
        decode_from_slice(&encoded).expect("decode LinkedList<(u32, String)>");
    assert_eq!(original, decoded);
    let items: Vec<(u32, String)> = decoded.into_iter().collect();
    assert_eq!(items[0], (1u32, "first".to_string()));
    assert_eq!(items[2], (3u32, "third".to_string()));
}

// ── 19. LinkedList<Option<u32>> roundtrip ─────────────────────────────────────
#[test]
fn test_linkedlist_option_u32_roundtrip() {
    let original: LinkedList<Option<u32>> = [Some(1u32), None, Some(3), None, Some(5)]
        .iter()
        .copied()
        .collect();
    let encoded = encode_to_vec(&original).expect("encode LinkedList<Option<u32>>");
    let (decoded, _bytes): (LinkedList<Option<u32>>, usize) =
        decode_from_slice(&encoded).expect("decode LinkedList<Option<u32>>");
    assert_eq!(original, decoded);
    let values: Vec<Option<u32>> = decoded.into_iter().collect();
    assert_eq!(values[1], None);
    assert_eq!(values[2], Some(3u32));
}

// ── 20. Re-encode decoded LinkedList gives same bytes ─────────────────────────
#[test]
fn test_linkedlist_reencode_gives_same_bytes() {
    let original: LinkedList<u32> = [5u32, 10, 15, 20, 25].iter().copied().collect();
    let first_encoded = encode_to_vec(&original).expect("first encode LinkedList<u32>");
    let (decoded, _bytes): (LinkedList<u32>, usize) =
        decode_from_slice(&first_encoded).expect("decode LinkedList<u32>");
    let second_encoded = encode_to_vec(&decoded).expect("second encode LinkedList<u32>");
    assert_eq!(first_encoded, second_encoded);
}

// ── 21. LinkedList<u32> length preserved after roundtrip ──────────────────────
#[test]
fn test_linkedlist_u32_length_preserved_after_roundtrip() {
    let original: LinkedList<u32> = (1u32..=12).collect();
    let original_len = original.len();
    let encoded = encode_to_vec(&original).expect("encode LinkedList<u32> length");
    let (decoded, _bytes): (LinkedList<u32>, usize) =
        decode_from_slice(&encoded).expect("decode LinkedList<u32> length");
    assert_eq!(decoded.len(), original_len);
    assert_eq!(decoded.len(), 12);
}

// ── 22. LinkedList front/back elements preserved after roundtrip ──────────────
#[test]
fn test_linkedlist_front_back_elements_preserved() {
    let original: LinkedList<u32> = [111u32, 222, 333, 444, 555].iter().copied().collect();
    let expected_front = *original.front().expect("front element should exist");
    let expected_back = *original.back().expect("back element should exist");
    let encoded = encode_to_vec(&original).expect("encode LinkedList<u32> front/back");
    let (decoded, _bytes): (LinkedList<u32>, usize) =
        decode_from_slice(&encoded).expect("decode LinkedList<u32> front/back");
    assert_eq!(
        decoded.front().copied(),
        Some(expected_front),
        "front element must match"
    );
    assert_eq!(
        decoded.back().copied(),
        Some(expected_back),
        "back element must match"
    );
    assert_eq!(expected_front, 111u32);
    assert_eq!(expected_back, 555u32);
}
