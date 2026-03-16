//! Comprehensive tests for VecDeque and LinkedList encode/decode roundtrips.

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
use std::collections::{LinkedList, VecDeque};

use oxicode::{Decode, Encode};

// ===== Helper =====

fn roundtrip<T>(original: &T) -> T
where
    T: Encode + Decode + std::fmt::Debug + PartialEq,
{
    let bytes = oxicode::encode_to_vec(original).expect("encode failed");
    let (decoded, _): (T, _) = oxicode::decode_from_slice(&bytes).expect("decode failed");
    decoded
}

// ===== VecDeque tests =====

#[test]
fn test_vecdeque_u32_empty_roundtrip() {
    let original: VecDeque<u32> = VecDeque::new();
    let decoded = roundtrip(&original);
    assert_eq!(original, decoded);
    assert!(decoded.is_empty());
}

#[test]
fn test_vecdeque_u32_five_elements_roundtrip() {
    let original: VecDeque<u32> = vec![10u32, 20, 30, 40, 50].into_iter().collect();
    let decoded = roundtrip(&original);
    assert_eq!(original, decoded);
    assert_eq!(decoded.len(), 5);
}

#[test]
fn test_vecdeque_string_roundtrip() {
    let original: VecDeque<String> = vec![
        "alpha".to_string(),
        "beta".to_string(),
        "gamma".to_string(),
        "delta with spaces".to_string(),
        "epsilon\nnewline".to_string(),
    ]
    .into_iter()
    .collect();
    let decoded = roundtrip(&original);
    assert_eq!(original, decoded);
}

#[test]
fn test_vecdeque_nested_vec_u8_roundtrip() {
    let original: VecDeque<Vec<u8>> =
        vec![vec![0u8, 1, 2, 3], vec![], vec![255, 128, 64], vec![42]]
            .into_iter()
            .collect();
    let decoded = roundtrip(&original);
    assert_eq!(original, decoded);
    // Verify nested content explicitly
    assert_eq!(decoded[0], &[0u8, 1, 2, 3][..]);
    assert_eq!(decoded[1], &[][..]);
    assert_eq!(decoded[2], &[255u8, 128, 64][..]);
}

#[test]
fn test_vecdeque_push_front_order_preserved_roundtrip() {
    // Build a deque with elements pushed to both front and back, then verify
    // that the logical front-to-back order is preserved after encode/decode.
    let mut original: VecDeque<u32> = VecDeque::new();
    original.push_back(3);
    original.push_back(4);
    original.push_back(5);
    original.push_front(2);
    original.push_front(1);
    original.push_front(0);
    // Logical order: [0, 1, 2, 3, 4, 5]

    let decoded = roundtrip(&original);
    assert_eq!(original, decoded);

    let as_vec: Vec<u32> = decoded.into_iter().collect();
    assert_eq!(as_vec, vec![0u32, 1, 2, 3, 4, 5]);
}

// ===== LinkedList tests =====

#[test]
fn test_linkedlist_u32_empty_roundtrip() {
    let original: LinkedList<u32> = LinkedList::new();
    let decoded = roundtrip(&original);
    assert_eq!(original, decoded);
    assert!(decoded.is_empty());
}

#[test]
fn test_linkedlist_u32_five_elements_roundtrip() {
    let original: LinkedList<u32> = vec![11u32, 22, 33, 44, 55].into_iter().collect();
    let decoded = roundtrip(&original);
    assert_eq!(original, decoded);
    assert_eq!(decoded.len(), 5);
}

#[test]
fn test_linkedlist_string_roundtrip() {
    let original: LinkedList<String> = vec![
        "one".to_string(),
        "two".to_string(),
        "three".to_string(),
        "unicode: \u{1F600}".to_string(),
    ]
    .into_iter()
    .collect();
    let decoded = roundtrip(&original);
    assert_eq!(original, decoded);
}

#[test]
fn test_linkedlist_tuple_elements_roundtrip() {
    let original: LinkedList<(u32, String)> = vec![
        (1u32, "first".to_string()),
        (2, "second".to_string()),
        (3, "third".to_string()),
        (0, "zero".to_string()),
    ]
    .into_iter()
    .collect();
    let decoded = roundtrip(&original);
    assert_eq!(original, decoded);

    // Verify individual element integrity
    let items: Vec<(u32, String)> = decoded.into_iter().collect();
    assert_eq!(items[0], (1, "first".to_string()));
    assert_eq!(items[3], (0, "zero".to_string()));
}

#[test]
fn test_linkedlist_large_100_elements_roundtrip() {
    let original: LinkedList<u32> = (0u32..100).collect();
    let decoded = roundtrip(&original);
    assert_eq!(original, decoded);
    assert_eq!(decoded.len(), 100);

    // Verify ordering is preserved across the full range
    let values: Vec<u32> = decoded.into_iter().collect();
    for (i, &v) in values.iter().enumerate() {
        assert_eq!(v, i as u32, "element at index {i} should be {i}");
    }
}

// ===== VecDeque vs Vec encode size comparison =====

#[test]
fn test_vecdeque_vs_vec_encode_size_equal() {
    // VecDeque and Vec with identical elements should produce identical encoded bytes,
    // since both encode as: (len as u64) followed by each element.
    let data = vec![1u32, 2, 3, 4, 5, 100, 200, 300];
    let vec_original: Vec<u32> = data.clone();
    let deque_original: VecDeque<u32> = data.into_iter().collect();

    let vec_bytes = oxicode::encode_to_vec(&vec_original).expect("encode Vec failed");
    let deque_bytes = oxicode::encode_to_vec(&deque_original).expect("encode VecDeque failed");

    assert_eq!(
        vec_bytes.len(),
        deque_bytes.len(),
        "encoded size of Vec and VecDeque with same data must be equal"
    );
    assert_eq!(
        vec_bytes, deque_bytes,
        "encoded bytes of Vec and VecDeque with same data must be identical"
    );
}

// ===== LinkedList in a derived struct =====

#[derive(Debug, PartialEq, Encode, Decode)]
struct TaskQueue {
    name: String,
    priority: u32,
    pending: LinkedList<String>,
    tags: VecDeque<u32>,
}

#[test]
fn test_linkedlist_in_derived_struct_roundtrip() {
    let mut pending = LinkedList::new();
    pending.push_back("task-a".to_string());
    pending.push_back("task-b".to_string());
    pending.push_back("task-c".to_string());

    let mut tags: VecDeque<u32> = VecDeque::new();
    tags.push_back(10);
    tags.push_front(5);
    tags.push_back(20);

    let original = TaskQueue {
        name: "main-queue".to_string(),
        priority: 42,
        pending,
        tags,
    };

    let decoded = roundtrip(&original);
    assert_eq!(original, decoded);
    assert_eq!(decoded.pending.len(), 3);
    assert_eq!(decoded.tags.len(), 3);

    // Verify tag ordering: push_front(5) then push_back(10) push_back(20) => [5, 10, 20]
    let tags_vec: Vec<u32> = decoded.tags.into_iter().collect();
    assert_eq!(tags_vec, vec![5u32, 10, 20]);

    let pending_items: Vec<String> = decoded.pending.into_iter().collect();
    assert_eq!(pending_items[0], "task-a");
    assert_eq!(pending_items[2], "task-c");
}
