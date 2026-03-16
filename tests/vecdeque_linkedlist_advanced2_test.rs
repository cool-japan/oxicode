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
use std::collections::{LinkedList, VecDeque};

// ── 1. VecDeque<u32> empty roundtrip ─────────────────────────────────────────
#[test]
fn test_vecdeque_u32_empty_roundtrip() {
    let original: VecDeque<u32> = VecDeque::new();
    let encoded = encode_to_vec(&original).expect("encode VecDeque<u32> empty");
    let (decoded, _consumed): (VecDeque<u32>, usize) =
        decode_from_slice(&encoded).expect("decode VecDeque<u32> empty");
    assert_eq!(original, decoded);
    assert!(decoded.is_empty());
}

// ── 2. VecDeque<u32> with 5 elements roundtrip ───────────────────────────────
#[test]
fn test_vecdeque_u32_five_elements_roundtrip() {
    let original: VecDeque<u32> = [10u32, 20, 30, 40, 50].iter().copied().collect();
    let encoded = encode_to_vec(&original).expect("encode VecDeque<u32> five elements");
    let (decoded, _consumed): (VecDeque<u32>, usize) =
        decode_from_slice(&encoded).expect("decode VecDeque<u32> five elements");
    assert_eq!(original, decoded);
    assert_eq!(decoded.len(), 5);
}

// ── 3. VecDeque<String> roundtrip ────────────────────────────────────────────
#[test]
fn test_vecdeque_string_roundtrip() {
    let original: VecDeque<String> = ["alpha", "beta", "gamma"]
        .iter()
        .map(|s| s.to_string())
        .collect();
    let encoded = encode_to_vec(&original).expect("encode VecDeque<String>");
    let (decoded, _consumed): (VecDeque<String>, usize) =
        decode_from_slice(&encoded).expect("decode VecDeque<String>");
    assert_eq!(original, decoded);
}

// ── 4. VecDeque<u8> consumed equals encoded length ───────────────────────────
#[test]
fn test_vecdeque_u8_consumed_equals_encoded_len() {
    let original: VecDeque<u8> = [1u8, 2, 3, 4, 5, 6, 7, 8].iter().copied().collect();
    let encoded = encode_to_vec(&original).expect("encode VecDeque<u8>");
    let (_decoded, consumed): (VecDeque<u8>, usize) =
        decode_from_slice(&encoded).expect("decode VecDeque<u8>");
    assert_eq!(consumed, encoded.len());
}

// ── 5. VecDeque<u32> same wire bytes as Vec<u32> for same elements ────────────
#[test]
fn test_vecdeque_u32_same_wire_bytes_as_vec_u32() {
    let elements = [100u32, 200, 300];
    let vec_original: Vec<u32> = elements.to_vec();
    let deque_original: VecDeque<u32> = elements.iter().copied().collect();
    let vec_encoded = encode_to_vec(&vec_original).expect("encode Vec<u32>");
    let deque_encoded = encode_to_vec(&deque_original).expect("encode VecDeque<u32>");
    assert_eq!(vec_encoded, deque_encoded);
}

// ── 6. VecDeque<u32> with fixed-int config ───────────────────────────────────
#[test]
fn test_vecdeque_u32_fixed_int_config() {
    let cfg = config::standard().with_fixed_int_encoding();
    let original: VecDeque<u32> = [1u32, 2, 3].iter().copied().collect();
    let encoded =
        encode_to_vec_with_config(&original, cfg).expect("encode VecDeque<u32> fixed-int");
    let (decoded, consumed): (VecDeque<u32>, usize) =
        decode_from_slice_with_config(&encoded, cfg).expect("decode VecDeque<u32> fixed-int");
    assert_eq!(original, decoded);
    assert_eq!(consumed, encoded.len());
    // u32 fixed = 4 bytes each; length prefix u64 = 8 bytes; 3 elements => 8 + 3*4 = 20
    assert_eq!(encoded.len(), 20);
}

// ── 7. VecDeque<u64> with big-endian config ──────────────────────────────────
#[test]
fn test_vecdeque_u64_big_endian_config() {
    let cfg = config::standard().with_big_endian();
    let original: VecDeque<u64> = [0xDEAD_BEEF_u64, 0x0102_0304_0506_0708]
        .iter()
        .copied()
        .collect();
    let encoded =
        encode_to_vec_with_config(&original, cfg).expect("encode VecDeque<u64> big-endian");
    let (decoded, consumed): (VecDeque<u64>, usize) =
        decode_from_slice_with_config(&encoded, cfg).expect("decode VecDeque<u64> big-endian");
    assert_eq!(original, decoded);
    assert_eq!(consumed, encoded.len());
}

// ── 8. VecDeque<i32> with negative values roundtrip ──────────────────────────
#[test]
fn test_vecdeque_i32_negative_values_roundtrip() {
    let original: VecDeque<i32> = [-1i32, -100, -32768, 0, 42].iter().copied().collect();
    let encoded = encode_to_vec(&original).expect("encode VecDeque<i32> negative");
    let (decoded, _consumed): (VecDeque<i32>, usize) =
        decode_from_slice(&encoded).expect("decode VecDeque<i32> negative");
    assert_eq!(original, decoded);
}

// ── 9. VecDeque<Vec<u8>> nested roundtrip ────────────────────────────────────
#[test]
fn test_vecdeque_nested_vec_u8_roundtrip() {
    let original: VecDeque<Vec<u8>> = vec![vec![0u8, 1, 2], vec![], vec![255u8, 128, 64, 32]]
        .into_iter()
        .collect();
    let encoded = encode_to_vec(&original).expect("encode VecDeque<Vec<u8>>");
    let (decoded, _consumed): (VecDeque<Vec<u8>>, usize) =
        decode_from_slice(&encoded).expect("decode VecDeque<Vec<u8>>");
    assert_eq!(original, decoded);
}

// ── 10. Option<VecDeque<u32>> Some roundtrip ─────────────────────────────────
#[test]
fn test_option_vecdeque_u32_some_roundtrip() {
    let inner: VecDeque<u32> = [7u32, 8, 9].iter().copied().collect();
    let original: Option<VecDeque<u32>> = Some(inner);
    let encoded = encode_to_vec(&original).expect("encode Option<VecDeque<u32>> Some");
    let (decoded, _consumed): (Option<VecDeque<u32>>, usize) =
        decode_from_slice(&encoded).expect("decode Option<VecDeque<u32>> Some");
    assert_eq!(original, decoded);
}

// ── 11. Option<VecDeque<u32>> None roundtrip ─────────────────────────────────
#[test]
fn test_option_vecdeque_u32_none_roundtrip() {
    let original: Option<VecDeque<u32>> = None;
    let encoded = encode_to_vec(&original).expect("encode Option<VecDeque<u32>> None");
    let (decoded, _consumed): (Option<VecDeque<u32>>, usize) =
        decode_from_slice(&encoded).expect("decode Option<VecDeque<u32>> None");
    assert_eq!(original, decoded);
    assert!(decoded.is_none());
}

// ── 12. LinkedList<u32> empty roundtrip ──────────────────────────────────────
#[test]
fn test_linkedlist_u32_empty_roundtrip() {
    let original: LinkedList<u32> = LinkedList::new();
    let encoded = encode_to_vec(&original).expect("encode LinkedList<u32> empty");
    let (decoded, _consumed): (LinkedList<u32>, usize) =
        decode_from_slice(&encoded).expect("decode LinkedList<u32> empty");
    assert_eq!(original, decoded);
    assert!(decoded.is_empty());
}

// ── 13. LinkedList<u32> with 5 elements roundtrip ────────────────────────────
#[test]
fn test_linkedlist_u32_five_elements_roundtrip() {
    let original: LinkedList<u32> = [11u32, 22, 33, 44, 55].iter().copied().collect();
    let encoded = encode_to_vec(&original).expect("encode LinkedList<u32> five elements");
    let (decoded, _consumed): (LinkedList<u32>, usize) =
        decode_from_slice(&encoded).expect("decode LinkedList<u32> five elements");
    assert_eq!(original, decoded);
    assert_eq!(decoded.len(), 5);
}

// ── 14. LinkedList<String> roundtrip ─────────────────────────────────────────
#[test]
fn test_linkedlist_string_roundtrip() {
    let original: LinkedList<String> = ["foo", "bar", "baz"]
        .iter()
        .map(|s| s.to_string())
        .collect();
    let encoded = encode_to_vec(&original).expect("encode LinkedList<String>");
    let (decoded, _consumed): (LinkedList<String>, usize) =
        decode_from_slice(&encoded).expect("decode LinkedList<String>");
    assert_eq!(original, decoded);
}

// ── 15. LinkedList<u8> consumed equals encoded length ────────────────────────
#[test]
fn test_linkedlist_u8_consumed_equals_encoded_len() {
    let original: LinkedList<u8> = [10u8, 20, 30, 40].iter().copied().collect();
    let encoded = encode_to_vec(&original).expect("encode LinkedList<u8>");
    let (_decoded, consumed): (LinkedList<u8>, usize) =
        decode_from_slice(&encoded).expect("decode LinkedList<u8>");
    assert_eq!(consumed, encoded.len());
}

// ── 16. LinkedList<u32> same wire bytes as Vec<u32> for same elements ─────────
#[test]
fn test_linkedlist_u32_same_wire_bytes_as_vec_u32() {
    let elements = [5u32, 10, 15, 20];
    let vec_original: Vec<u32> = elements.to_vec();
    let list_original: LinkedList<u32> = elements.iter().copied().collect();
    let vec_encoded = encode_to_vec(&vec_original).expect("encode Vec<u32>");
    let list_encoded = encode_to_vec(&list_original).expect("encode LinkedList<u32>");
    assert_eq!(vec_encoded, list_encoded);
}

// ── 17. LinkedList<u32> with fixed-int config ────────────────────────────────
#[test]
fn test_linkedlist_u32_fixed_int_config() {
    let cfg = config::standard().with_fixed_int_encoding();
    let original: LinkedList<u32> = [9u32, 18, 27].iter().copied().collect();
    let encoded =
        encode_to_vec_with_config(&original, cfg).expect("encode LinkedList<u32> fixed-int");
    let (decoded, consumed): (LinkedList<u32>, usize) =
        decode_from_slice_with_config(&encoded, cfg).expect("decode LinkedList<u32> fixed-int");
    assert_eq!(original, decoded);
    assert_eq!(consumed, encoded.len());
    // u32 fixed = 4 bytes each; length prefix u64 = 8 bytes; 3 elements => 8 + 3*4 = 20
    assert_eq!(encoded.len(), 20);
}

// ── 18. LinkedList<u64> with big-endian config ───────────────────────────────
#[test]
fn test_linkedlist_u64_big_endian_config() {
    let cfg = config::standard().with_big_endian();
    let original: LinkedList<u64> = [0x0011_2233_u64, 0xAABB_CCDD_EEFF_0011]
        .iter()
        .copied()
        .collect();
    let encoded =
        encode_to_vec_with_config(&original, cfg).expect("encode LinkedList<u64> big-endian");
    let (decoded, consumed): (LinkedList<u64>, usize) =
        decode_from_slice_with_config(&encoded, cfg).expect("decode LinkedList<u64> big-endian");
    assert_eq!(original, decoded);
    assert_eq!(consumed, encoded.len());
}

// ── 19. LinkedList<i32> with negative values roundtrip ───────────────────────
#[test]
fn test_linkedlist_i32_negative_values_roundtrip() {
    let original: LinkedList<i32> = [-999i32, -1, 0, 1, 999].iter().copied().collect();
    let encoded = encode_to_vec(&original).expect("encode LinkedList<i32> negative");
    let (decoded, _consumed): (LinkedList<i32>, usize) =
        decode_from_slice(&encoded).expect("decode LinkedList<i32> negative");
    assert_eq!(original, decoded);
}

// ── 20. Vec<VecDeque<u32>> roundtrip ─────────────────────────────────────────
#[test]
fn test_vec_of_vecdeque_u32_roundtrip() {
    let original: Vec<VecDeque<u32>> = vec![
        [1u32, 2, 3].iter().copied().collect(),
        VecDeque::new(),
        [100u32, 200].iter().copied().collect(),
    ];
    let encoded = encode_to_vec(&original).expect("encode Vec<VecDeque<u32>>");
    let (decoded, consumed): (Vec<VecDeque<u32>>, usize) =
        decode_from_slice(&encoded).expect("decode Vec<VecDeque<u32>>");
    assert_eq!(original, decoded);
    assert_eq!(consumed, encoded.len());
    assert_eq!(decoded.len(), 3);
}

// ── 21. VecDeque<u32> push_front then roundtrip preserves order ──────────────
#[test]
fn test_vecdeque_u32_push_front_preserves_order() {
    let mut original: VecDeque<u32> = VecDeque::new();
    // Push front in reverse to produce [1, 2, 3, 4, 5] order
    original.push_front(5);
    original.push_front(4);
    original.push_front(3);
    original.push_front(2);
    original.push_front(1);

    let expected: VecDeque<u32> = [1u32, 2, 3, 4, 5].iter().copied().collect();
    assert_eq!(original, expected);

    let encoded = encode_to_vec(&original).expect("encode VecDeque<u32> push_front");
    let (decoded, consumed): (VecDeque<u32>, usize) =
        decode_from_slice(&encoded).expect("decode VecDeque<u32> push_front");
    assert_eq!(decoded, expected);
    assert_eq!(consumed, encoded.len());
}

// ── 22. LinkedList<u32> push_back then roundtrip preserves order ──────────────
#[test]
fn test_linkedlist_u32_push_back_preserves_order() {
    let mut original: LinkedList<u32> = LinkedList::new();
    original.push_back(10);
    original.push_back(20);
    original.push_back(30);
    original.push_back(40);
    original.push_back(50);

    let encoded = encode_to_vec(&original).expect("encode LinkedList<u32> push_back");
    let (decoded, consumed): (LinkedList<u32>, usize) =
        decode_from_slice(&encoded).expect("decode LinkedList<u32> push_back");
    assert_eq!(original, decoded);
    assert_eq!(consumed, encoded.len());

    let decoded_vec: Vec<u32> = decoded.into_iter().collect();
    assert_eq!(decoded_vec, vec![10u32, 20, 30, 40, 50]);
}
