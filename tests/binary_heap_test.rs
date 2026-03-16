//! Tests for BinaryHeap encode/decode roundtrips

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
use std::collections::BinaryHeap;

fn heap_to_sorted_vec<T: Ord>(heap: BinaryHeap<T>) -> Vec<T> {
    let mut v: Vec<T> = heap.into_iter().collect();
    v.sort();
    v
}

#[test]
fn test_binary_heap_u32_empty_roundtrip() {
    let original: BinaryHeap<u32> = BinaryHeap::new();
    let bytes = oxicode::encode_to_vec(&original).expect("encode failed");
    let (decoded, _): (BinaryHeap<u32>, _) =
        oxicode::decode_from_slice(&bytes).expect("decode failed");
    assert_eq!(heap_to_sorted_vec(original), heap_to_sorted_vec(decoded));
}

#[test]
fn test_binary_heap_u32_elements_roundtrip() {
    // [5,3,8,1,9] — order not preserved but elements must be the same
    let original: BinaryHeap<u32> = [5u32, 3, 8, 1, 9].iter().copied().collect();
    let bytes = oxicode::encode_to_vec(&original).expect("encode failed");
    let (decoded, _): (BinaryHeap<u32>, _) =
        oxicode::decode_from_slice(&bytes).expect("decode failed");

    assert_eq!(
        heap_to_sorted_vec(original),
        heap_to_sorted_vec(decoded),
        "element sets must be identical after roundtrip"
    );
}

#[test]
fn test_binary_heap_i64_roundtrip() {
    let elements: &[i64] = &[-100, 0, 42, -1, 9999, i64::MIN / 2, i64::MAX / 2];
    let original: BinaryHeap<i64> = elements.iter().copied().collect();
    let bytes = oxicode::encode_to_vec(&original).expect("encode failed");
    let (decoded, _): (BinaryHeap<i64>, _) =
        oxicode::decode_from_slice(&bytes).expect("decode failed");

    assert_eq!(
        heap_to_sorted_vec(original),
        heap_to_sorted_vec(decoded),
        "i64 element sets must be identical after roundtrip"
    );
}

#[test]
fn test_binary_heap_string_roundtrip() {
    let original: BinaryHeap<String> = ["banana", "apple", "cherry", "date", "elderberry"]
        .iter()
        .map(|s| s.to_string())
        .collect();
    let bytes = oxicode::encode_to_vec(&original).expect("encode failed");
    let (decoded, _): (BinaryHeap<String>, _) =
        oxicode::decode_from_slice(&bytes).expect("decode failed");

    assert_eq!(
        heap_to_sorted_vec(original),
        heap_to_sorted_vec(decoded),
        "String element sets must be identical after roundtrip"
    );
}

#[test]
fn test_binary_heap_large_u32_roundtrip() {
    let original: BinaryHeap<u32> = (0u32..100).collect();
    let bytes = oxicode::encode_to_vec(&original).expect("encode failed");
    let (decoded, _): (BinaryHeap<u32>, _) =
        oxicode::decode_from_slice(&bytes).expect("decode failed");

    assert_eq!(original.len(), decoded.len(), "lengths must match");
    assert_eq!(
        heap_to_sorted_vec(original),
        heap_to_sorted_vec(decoded),
        "100-element u32 heap must roundtrip correctly"
    );
}

#[test]
fn test_binary_heap_peek_max_after_decode() {
    // After decoding, peek() must return the maximum element
    let elements: &[u32] = &[5, 3, 8, 1, 9, 2, 7];
    let expected_max: u32 = *elements.iter().max().expect("non-empty");

    let original: BinaryHeap<u32> = elements.iter().copied().collect();
    let bytes = oxicode::encode_to_vec(&original).expect("encode failed");
    let (decoded, _): (BinaryHeap<u32>, _) =
        oxicode::decode_from_slice(&bytes).expect("decode failed");

    let peeked = decoded.peek().expect("decoded heap must not be empty");
    assert_eq!(
        *peeked, expected_max,
        "peek() after decode must return the maximum element"
    );
}
