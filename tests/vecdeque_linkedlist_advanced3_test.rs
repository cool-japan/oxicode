//! Advanced tests for VecDeque and LinkedList encoding in OxiCode (set 3)

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
use std::collections::{LinkedList, VecDeque};

#[derive(Debug, PartialEq, Encode, Decode)]
struct Task {
    id: u32,
    name: String,
    done: bool,
}

// Test 1: VecDeque<u32> 5 items roundtrip
#[test]
fn test_vecdeque_u32_five_items_roundtrip() {
    let mut deque: VecDeque<u32> = VecDeque::new();
    deque.push_back(10);
    deque.push_back(20);
    deque.push_back(30);
    deque.push_back(40);
    deque.push_back(50);

    let encoded = encode_to_vec(&deque).expect("Failed to encode VecDeque<u32> with 5 items");
    let (decoded, _): (VecDeque<u32>, usize) =
        decode_from_slice(&encoded).expect("Failed to decode VecDeque<u32> with 5 items");

    assert_eq!(deque, decoded);
}

// Test 2: VecDeque<u32> empty roundtrip
#[test]
fn test_vecdeque_u32_empty_roundtrip() {
    let deque: VecDeque<u32> = VecDeque::new();

    let encoded = encode_to_vec(&deque).expect("Failed to encode empty VecDeque<u32>");
    let (decoded, _): (VecDeque<u32>, usize) =
        decode_from_slice(&encoded).expect("Failed to decode empty VecDeque<u32>");

    assert_eq!(deque, decoded);
    assert!(decoded.is_empty());
}

// Test 3: VecDeque<String> roundtrip (4 items)
#[test]
fn test_vecdeque_string_four_items_roundtrip() {
    let mut deque: VecDeque<String> = VecDeque::new();
    deque.push_back("alpha".to_string());
    deque.push_back("beta".to_string());
    deque.push_back("gamma".to_string());
    deque.push_back("delta".to_string());

    let encoded = encode_to_vec(&deque).expect("Failed to encode VecDeque<String>");
    let (decoded, _): (VecDeque<String>, usize) =
        decode_from_slice(&encoded).expect("Failed to decode VecDeque<String>");

    assert_eq!(deque, decoded);
}

// Test 4: VecDeque<Task> roundtrip (3 items)
#[test]
fn test_vecdeque_task_three_items_roundtrip() {
    let mut deque: VecDeque<Task> = VecDeque::new();
    deque.push_back(Task {
        id: 1,
        name: "Design".to_string(),
        done: true,
    });
    deque.push_back(Task {
        id: 2,
        name: "Implement".to_string(),
        done: false,
    });
    deque.push_back(Task {
        id: 3,
        name: "Test".to_string(),
        done: false,
    });

    let encoded = encode_to_vec(&deque).expect("Failed to encode VecDeque<Task>");
    let (decoded, _): (VecDeque<Task>, usize) =
        decode_from_slice(&encoded).expect("Failed to decode VecDeque<Task>");

    assert_eq!(deque, decoded);
}

// Test 5: VecDeque<u32> encodes same as Vec<u32> with same elements
#[test]
fn test_vecdeque_u32_same_encoding_as_vec_u32() {
    let elements = vec![100u32, 200, 300, 400, 500];
    let vec_val: Vec<u32> = elements.clone();
    let mut deque_val: VecDeque<u32> = VecDeque::new();
    for &e in &elements {
        deque_val.push_back(e);
    }

    let vec_encoded = encode_to_vec(&vec_val).expect("Failed to encode Vec<u32>");
    let deque_encoded = encode_to_vec(&deque_val).expect("Failed to encode VecDeque<u32>");

    assert_eq!(
        vec_encoded, deque_encoded,
        "VecDeque and Vec should produce identical encodings"
    );
}

// Test 6: VecDeque<u8> with all 0..255 values roundtrip
#[test]
fn test_vecdeque_u8_all_byte_values_roundtrip() {
    let deque: VecDeque<u8> = (0u8..=255u8).collect();

    let encoded =
        encode_to_vec(&deque).expect("Failed to encode VecDeque<u8> with all byte values");
    let (decoded, _): (VecDeque<u8>, usize) =
        decode_from_slice(&encoded).expect("Failed to decode VecDeque<u8> with all byte values");

    assert_eq!(deque, decoded);
    assert_eq!(decoded.len(), 256);
}

// Test 7: VecDeque<u64> large values roundtrip (5 items)
#[test]
fn test_vecdeque_u64_large_values_roundtrip() {
    let mut deque: VecDeque<u64> = VecDeque::new();
    deque.push_back(u64::MAX);
    deque.push_back(u64::MAX / 2);
    deque.push_back(1_000_000_000_000u64);
    deque.push_back(9_999_999_999_999u64);
    deque.push_back(0u64);

    let encoded = encode_to_vec(&deque).expect("Failed to encode VecDeque<u64> large values");
    let (decoded, _): (VecDeque<u64>, usize) =
        decode_from_slice(&encoded).expect("Failed to decode VecDeque<u64> large values");

    assert_eq!(deque, decoded);
}

// Test 8: LinkedList<u32> 5 items roundtrip
#[test]
fn test_linkedlist_u32_five_items_roundtrip() {
    let mut list: LinkedList<u32> = LinkedList::new();
    list.push_back(11);
    list.push_back(22);
    list.push_back(33);
    list.push_back(44);
    list.push_back(55);

    let encoded = encode_to_vec(&list).expect("Failed to encode LinkedList<u32> with 5 items");
    let (decoded, _): (LinkedList<u32>, usize) =
        decode_from_slice(&encoded).expect("Failed to decode LinkedList<u32> with 5 items");

    assert_eq!(list, decoded);
}

// Test 9: LinkedList<u32> empty roundtrip
#[test]
fn test_linkedlist_u32_empty_roundtrip() {
    let list: LinkedList<u32> = LinkedList::new();

    let encoded = encode_to_vec(&list).expect("Failed to encode empty LinkedList<u32>");
    let (decoded, _): (LinkedList<u32>, usize) =
        decode_from_slice(&encoded).expect("Failed to decode empty LinkedList<u32>");

    assert_eq!(list, decoded);
    assert!(decoded.is_empty());
}

// Test 10: LinkedList<String> roundtrip (4 items)
#[test]
fn test_linkedlist_string_four_items_roundtrip() {
    let mut list: LinkedList<String> = LinkedList::new();
    list.push_back("one".to_string());
    list.push_back("two".to_string());
    list.push_back("three".to_string());
    list.push_back("four".to_string());

    let encoded = encode_to_vec(&list).expect("Failed to encode LinkedList<String>");
    let (decoded, _): (LinkedList<String>, usize) =
        decode_from_slice(&encoded).expect("Failed to decode LinkedList<String>");

    assert_eq!(list, decoded);
}

// Test 11: LinkedList<Task> roundtrip (3 items)
#[test]
fn test_linkedlist_task_three_items_roundtrip() {
    let mut list: LinkedList<Task> = LinkedList::new();
    list.push_back(Task {
        id: 10,
        name: "Plan".to_string(),
        done: true,
    });
    list.push_back(Task {
        id: 20,
        name: "Execute".to_string(),
        done: false,
    });
    list.push_back(Task {
        id: 30,
        name: "Review".to_string(),
        done: true,
    });

    let encoded = encode_to_vec(&list).expect("Failed to encode LinkedList<Task>");
    let (decoded, _): (LinkedList<Task>, usize) =
        decode_from_slice(&encoded).expect("Failed to decode LinkedList<Task>");

    assert_eq!(list, decoded);
}

// Test 12: LinkedList<u32> encodes same as Vec<u32> with same elements
#[test]
fn test_linkedlist_u32_same_encoding_as_vec_u32() {
    let elements = vec![7u32, 14, 21, 28, 35];
    let vec_val: Vec<u32> = elements.clone();
    let mut list_val: LinkedList<u32> = LinkedList::new();
    for &e in &elements {
        list_val.push_back(e);
    }

    let vec_encoded = encode_to_vec(&vec_val).expect("Failed to encode Vec<u32>");
    let list_encoded = encode_to_vec(&list_val).expect("Failed to encode LinkedList<u32>");

    assert_eq!(
        vec_encoded, list_encoded,
        "LinkedList and Vec should produce identical encodings"
    );
}

// Test 13: VecDeque<Option<u32>> roundtrip (Some, None, Some)
#[test]
fn test_vecdeque_option_u32_roundtrip() {
    let mut deque: VecDeque<Option<u32>> = VecDeque::new();
    deque.push_back(Some(42));
    deque.push_back(None);
    deque.push_back(Some(99));

    let encoded = encode_to_vec(&deque).expect("Failed to encode VecDeque<Option<u32>>");
    let (decoded, _): (VecDeque<Option<u32>>, usize) =
        decode_from_slice(&encoded).expect("Failed to decode VecDeque<Option<u32>>");

    assert_eq!(deque, decoded);
}

// Test 14: LinkedList<Option<String>> roundtrip (Some, None, Some)
#[test]
fn test_linkedlist_option_string_roundtrip() {
    let mut list: LinkedList<Option<String>> = LinkedList::new();
    list.push_back(Some("hello".to_string()));
    list.push_back(None);
    list.push_back(Some("world".to_string()));

    let encoded = encode_to_vec(&list).expect("Failed to encode LinkedList<Option<String>>");
    let (decoded, _): (LinkedList<Option<String>>, usize) =
        decode_from_slice(&encoded).expect("Failed to decode LinkedList<Option<String>>");

    assert_eq!(list, decoded);
}

// Test 15: Vec<VecDeque<u32>> roundtrip (3 inner deques)
#[test]
fn test_vec_of_vecdeque_u32_roundtrip() {
    let inner1: VecDeque<u32> = vec![1u32, 2, 3].into_iter().collect();
    let inner2: VecDeque<u32> = vec![4u32, 5].into_iter().collect();
    let inner3: VecDeque<u32> = vec![6u32, 7, 8, 9].into_iter().collect();
    let outer: Vec<VecDeque<u32>> = vec![inner1, inner2, inner3];

    let encoded = encode_to_vec(&outer).expect("Failed to encode Vec<VecDeque<u32>>");
    let (decoded, _): (Vec<VecDeque<u32>>, usize) =
        decode_from_slice(&encoded).expect("Failed to decode Vec<VecDeque<u32>>");

    assert_eq!(outer, decoded);
}

// Test 16: VecDeque with fixed-int config
#[test]
fn test_vecdeque_with_fixed_int_config() {
    let mut deque: VecDeque<u32> = VecDeque::new();
    deque.push_back(1u32);
    deque.push_back(2u32);
    deque.push_back(3u32);

    let cfg = config::standard().with_fixed_int_encoding();
    let encoded = encode_to_vec_with_config(&deque, cfg)
        .expect("Failed to encode VecDeque<u32> with fixed-int config");
    let (decoded, _): (VecDeque<u32>, usize) = decode_from_slice_with_config(&encoded, cfg)
        .expect("Failed to decode VecDeque<u32> with fixed-int config");

    assert_eq!(deque, decoded);
}

// Test 17: LinkedList with fixed-int config
#[test]
fn test_linkedlist_with_fixed_int_config() {
    let mut list: LinkedList<u32> = LinkedList::new();
    list.push_back(100u32);
    list.push_back(200u32);
    list.push_back(300u32);

    let cfg = config::standard().with_fixed_int_encoding();
    let encoded = encode_to_vec_with_config(&list, cfg)
        .expect("Failed to encode LinkedList<u32> with fixed-int config");
    let (decoded, _): (LinkedList<u32>, usize) = decode_from_slice_with_config(&encoded, cfg)
        .expect("Failed to decode LinkedList<u32> with fixed-int config");

    assert_eq!(list, decoded);
}

// Test 18: Option<VecDeque<u32>> Some roundtrip
#[test]
fn test_option_vecdeque_u32_some_roundtrip() {
    let inner: VecDeque<u32> = vec![5u32, 10, 15].into_iter().collect();
    let val: Option<VecDeque<u32>> = Some(inner);

    let encoded = encode_to_vec(&val).expect("Failed to encode Option<VecDeque<u32>> Some");
    let (decoded, _): (Option<VecDeque<u32>>, usize) =
        decode_from_slice(&encoded).expect("Failed to decode Option<VecDeque<u32>> Some");

    assert_eq!(val, decoded);
    assert!(decoded.is_some());
}

// Test 19: Option<LinkedList<u32>> None roundtrip
#[test]
fn test_option_linkedlist_u32_none_roundtrip() {
    let val: Option<LinkedList<u32>> = None;

    let encoded = encode_to_vec(&val).expect("Failed to encode Option<LinkedList<u32>> None");
    let (decoded, _): (Option<LinkedList<u32>>, usize) =
        decode_from_slice(&encoded).expect("Failed to decode Option<LinkedList<u32>> None");

    assert_eq!(val, decoded);
    assert!(decoded.is_none());
}

// Test 20: VecDeque consumed bytes equals encoded length
#[test]
fn test_vecdeque_consumed_bytes_equals_encoded_length() {
    let mut deque: VecDeque<u32> = VecDeque::new();
    deque.push_back(1u32);
    deque.push_back(2u32);
    deque.push_back(3u32);
    deque.push_back(4u32);

    let encoded =
        encode_to_vec(&deque).expect("Failed to encode VecDeque<u32> for byte count check");
    let (_decoded, consumed): (VecDeque<u32>, usize) =
        decode_from_slice(&encoded).expect("Failed to decode VecDeque<u32> for byte count check");

    assert_eq!(
        consumed,
        encoded.len(),
        "Consumed bytes should equal total encoded length"
    );
}

// Test 21: Large VecDeque (100 u32 items) roundtrip
#[test]
fn test_vecdeque_u32_large_100_items_roundtrip() {
    let deque: VecDeque<u32> = (0u32..100).collect();

    let encoded =
        encode_to_vec(&deque).expect("Failed to encode large VecDeque<u32> with 100 items");
    let (decoded, _): (VecDeque<u32>, usize) =
        decode_from_slice(&encoded).expect("Failed to decode large VecDeque<u32> with 100 items");

    assert_eq!(deque, decoded);
    assert_eq!(decoded.len(), 100);
}

// Test 22: Large LinkedList (50 String items) roundtrip
#[test]
fn test_linkedlist_string_large_50_items_roundtrip() {
    let list: LinkedList<String> = (0u32..50).map(|i| format!("item_{:03}", i)).collect();

    let encoded =
        encode_to_vec(&list).expect("Failed to encode large LinkedList<String> with 50 items");
    let (decoded, _): (LinkedList<String>, usize) = decode_from_slice(&encoded)
        .expect("Failed to decode large LinkedList<String> with 50 items");

    assert_eq!(list, decoded);
    assert_eq!(decoded.len(), 50);
}
