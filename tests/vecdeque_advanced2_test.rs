//! Advanced VecDeque encoding tests (set 2) for OxiCode.

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
use std::collections::VecDeque;

use oxicode::{
    config, decode_from_slice, decode_from_slice_with_config, encode_to_vec,
    encode_to_vec_with_config, Decode, Encode,
};

// ---------------------------------------------------------------------------
// Test 1: VecDeque<u32> empty roundtrip
// ---------------------------------------------------------------------------

#[test]
fn test_vecdeque_u32_empty_roundtrip() {
    let original: VecDeque<u32> = VecDeque::new();
    let bytes = encode_to_vec(&original).expect("encode empty VecDeque<u32>");
    let (decoded, _bytes): (VecDeque<u32>, usize) =
        decode_from_slice(&bytes).expect("decode empty VecDeque<u32>");
    assert_eq!(original, decoded);
    assert!(decoded.is_empty());
}

// ---------------------------------------------------------------------------
// Test 2: VecDeque<u32> single element roundtrip
// ---------------------------------------------------------------------------

#[test]
fn test_vecdeque_u32_single_element_roundtrip() {
    let mut original: VecDeque<u32> = VecDeque::new();
    original.push_back(42);
    let bytes = encode_to_vec(&original).expect("encode single-element VecDeque<u32>");
    let (decoded, _bytes): (VecDeque<u32>, usize) =
        decode_from_slice(&bytes).expect("decode single-element VecDeque<u32>");
    assert_eq!(original, decoded);
    assert_eq!(decoded.len(), 1);
    assert_eq!(decoded[0], 42);
}

// ---------------------------------------------------------------------------
// Test 3: VecDeque<u32> 10 elements roundtrip
// ---------------------------------------------------------------------------

#[test]
fn test_vecdeque_u32_ten_elements_roundtrip() {
    let original: VecDeque<u32> = (0u32..10).collect();
    let bytes = encode_to_vec(&original).expect("encode 10-element VecDeque<u32>");
    let (decoded, _bytes): (VecDeque<u32>, usize) =
        decode_from_slice(&bytes).expect("decode 10-element VecDeque<u32>");
    assert_eq!(original, decoded);
    assert_eq!(decoded.len(), 10);
}

// ---------------------------------------------------------------------------
// Test 4: VecDeque<String> roundtrip
// ---------------------------------------------------------------------------

#[test]
fn test_vecdeque_string_roundtrip() {
    let original: VecDeque<String> = vec![
        "hello".to_string(),
        "world".to_string(),
        "oxicode".to_string(),
        "unicode: \u{1F600}".to_string(),
        String::new(),
    ]
    .into_iter()
    .collect();
    let bytes = encode_to_vec(&original).expect("encode VecDeque<String>");
    let (decoded, _bytes): (VecDeque<String>, usize) =
        decode_from_slice(&bytes).expect("decode VecDeque<String>");
    assert_eq!(original, decoded);
}

// ---------------------------------------------------------------------------
// Test 5: VecDeque<Vec<u8>> roundtrip
// ---------------------------------------------------------------------------

#[test]
fn test_vecdeque_vec_u8_roundtrip() {
    let original: VecDeque<Vec<u8>> =
        vec![vec![0u8, 1, 2, 3], vec![], vec![255, 128, 64], vec![42]]
            .into_iter()
            .collect();
    let bytes = encode_to_vec(&original).expect("encode VecDeque<Vec<u8>>");
    let (decoded, _bytes): (VecDeque<Vec<u8>>, usize) =
        decode_from_slice(&bytes).expect("decode VecDeque<Vec<u8>>");
    assert_eq!(original, decoded);
}

// ---------------------------------------------------------------------------
// Test 6: VecDeque<u8> same wire bytes as Vec<u8> for same elements
// ---------------------------------------------------------------------------

#[test]
fn test_vecdeque_u8_same_wire_as_vec_u8() {
    let data = vec![1u8, 2, 3, 4, 5, 100, 200, 255];
    let vec_original: Vec<u8> = data.clone();
    let deque_original: VecDeque<u8> = data.into_iter().collect();

    let vec_bytes = encode_to_vec(&vec_original).expect("encode Vec<u8>");
    let deque_bytes = encode_to_vec(&deque_original).expect("encode VecDeque<u8>");

    assert_eq!(
        vec_bytes, deque_bytes,
        "Vec<u8> and VecDeque<u8> with same elements must produce identical bytes"
    );
}

// ---------------------------------------------------------------------------
// Test 7: VecDeque<u32> consumed == encoded length
// ---------------------------------------------------------------------------

#[test]
fn test_vecdeque_u32_consumed_equals_encoded_len() {
    let original: VecDeque<u32> = vec![10u32, 20, 30].into_iter().collect();
    let bytes = encode_to_vec(&original).expect("encode VecDeque<u32>");
    let (_decoded, consumed): (VecDeque<u32>, usize) =
        decode_from_slice(&bytes).expect("decode VecDeque<u32>");
    assert_eq!(
        consumed,
        bytes.len(),
        "consumed bytes must equal the encoded buffer length"
    );
}

// ---------------------------------------------------------------------------
// Test 8: VecDeque<u32> element order preserved
// ---------------------------------------------------------------------------

#[test]
fn test_vecdeque_u32_element_order_preserved() {
    let original: VecDeque<u32> = vec![100u32, 200, 300, 400, 500].into_iter().collect();
    let bytes = encode_to_vec(&original).expect("encode VecDeque<u32>");
    let (decoded, _bytes): (VecDeque<u32>, usize) =
        decode_from_slice(&bytes).expect("decode VecDeque<u32>");
    let decoded_vec: Vec<u32> = decoded.into_iter().collect();
    assert_eq!(decoded_vec, vec![100u32, 200, 300, 400, 500]);
}

// ---------------------------------------------------------------------------
// Test 9: VecDeque<u64> with fixed-int config roundtrip
// ---------------------------------------------------------------------------

#[test]
fn test_vecdeque_u64_fixed_int_config_roundtrip() {
    let cfg = config::standard().with_fixed_int_encoding();
    let original: VecDeque<u64> = vec![0u64, 1, u64::MAX / 2, u64::MAX].into_iter().collect();
    let bytes = encode_to_vec_with_config(&original, cfg).expect("encode VecDeque<u64> fixed-int");
    let (decoded, _bytes): (VecDeque<u64>, usize) =
        decode_from_slice_with_config(&bytes, cfg).expect("decode VecDeque<u64> fixed-int");
    assert_eq!(original, decoded);
}

// ---------------------------------------------------------------------------
// Test 10: VecDeque<u32> with big-endian config roundtrip
// ---------------------------------------------------------------------------

#[test]
fn test_vecdeque_u32_big_endian_config_roundtrip() {
    let cfg = config::standard().with_big_endian();
    let original: VecDeque<u32> = vec![1u32, 2, 3, 0xDEAD_BEEF].into_iter().collect();
    let bytes = encode_to_vec_with_config(&original, cfg).expect("encode VecDeque<u32> big-endian");
    let (decoded, _bytes): (VecDeque<u32>, usize) =
        decode_from_slice_with_config(&bytes, cfg).expect("decode VecDeque<u32> big-endian");
    assert_eq!(original, decoded);
}

// ---------------------------------------------------------------------------
// Test 11: Option<VecDeque<u32>> Some roundtrip
// ---------------------------------------------------------------------------

#[test]
fn test_option_vecdeque_u32_some_roundtrip() {
    let original: Option<VecDeque<u32>> = Some(vec![1u32, 2, 3].into_iter().collect());
    let bytes = encode_to_vec(&original).expect("encode Option<VecDeque<u32>> Some");
    let (decoded, _bytes): (Option<VecDeque<u32>>, usize) =
        decode_from_slice(&bytes).expect("decode Option<VecDeque<u32>> Some");
    assert_eq!(original, decoded);
    assert!(decoded.is_some());
}

// ---------------------------------------------------------------------------
// Test 12: Option<VecDeque<u32>> None roundtrip
// ---------------------------------------------------------------------------

#[test]
fn test_option_vecdeque_u32_none_roundtrip() {
    let original: Option<VecDeque<u32>> = None;
    let bytes = encode_to_vec(&original).expect("encode Option<VecDeque<u32>> None");
    let (decoded, _bytes): (Option<VecDeque<u32>>, usize) =
        decode_from_slice(&bytes).expect("decode Option<VecDeque<u32>> None");
    assert_eq!(original, decoded);
    assert!(decoded.is_none());
}

// ---------------------------------------------------------------------------
// Test 13: Vec<VecDeque<u8>> roundtrip
// ---------------------------------------------------------------------------

#[test]
fn test_vec_of_vecdeque_u8_roundtrip() {
    let original: Vec<VecDeque<u8>> = vec![
        vec![1u8, 2, 3].into_iter().collect(),
        VecDeque::new(),
        vec![255u8, 0, 128].into_iter().collect(),
    ];
    let bytes = encode_to_vec(&original).expect("encode Vec<VecDeque<u8>>");
    let (decoded, _bytes): (Vec<VecDeque<u8>>, usize) =
        decode_from_slice(&bytes).expect("decode Vec<VecDeque<u8>>");
    assert_eq!(original, decoded);
}

// ---------------------------------------------------------------------------
// Test 14: VecDeque with elements pushed from both front and back roundtrip
// ---------------------------------------------------------------------------

#[test]
fn test_vecdeque_push_front_and_back_roundtrip() {
    let mut original: VecDeque<u32> = VecDeque::new();
    original.push_back(3);
    original.push_back(4);
    original.push_back(5);
    original.push_front(2);
    original.push_front(1);
    original.push_front(0);
    // Logical order: [0, 1, 2, 3, 4, 5]

    let bytes = encode_to_vec(&original).expect("encode push-front-and-back VecDeque");
    let (decoded, _bytes): (VecDeque<u32>, usize) =
        decode_from_slice(&bytes).expect("decode push-front-and-back VecDeque");
    assert_eq!(original, decoded);

    let as_vec: Vec<u32> = decoded.into_iter().collect();
    assert_eq!(as_vec, vec![0u32, 1, 2, 3, 4, 5]);
}

// ---------------------------------------------------------------------------
// Test 15: VecDeque<bool> roundtrip
// ---------------------------------------------------------------------------

#[test]
fn test_vecdeque_bool_roundtrip() {
    let original: VecDeque<bool> = vec![true, false, true, true, false].into_iter().collect();
    let bytes = encode_to_vec(&original).expect("encode VecDeque<bool>");
    let (decoded, _bytes): (VecDeque<bool>, usize) =
        decode_from_slice(&bytes).expect("decode VecDeque<bool>");
    assert_eq!(original, decoded);
}

// ---------------------------------------------------------------------------
// Test 16: VecDeque<i32> with negative values roundtrip
// ---------------------------------------------------------------------------

#[test]
fn test_vecdeque_i32_negative_values_roundtrip() {
    let original: VecDeque<i32> = vec![-100i32, -1, 0, 1, i32::MIN, i32::MAX]
        .into_iter()
        .collect();
    let bytes = encode_to_vec(&original).expect("encode VecDeque<i32> negative");
    let (decoded, _bytes): (VecDeque<i32>, usize) =
        decode_from_slice(&bytes).expect("decode VecDeque<i32> negative");
    assert_eq!(original, decoded);
}

// ---------------------------------------------------------------------------
// Test 17: VecDeque<f64> — encode bit patterns for exact comparison
// ---------------------------------------------------------------------------

#[test]
fn test_vecdeque_f64_via_bit_patterns_roundtrip() {
    let floats: Vec<f64> = vec![
        0.0_f64,
        1.0,
        -1.0,
        3.14159,
        f64::INFINITY,
        f64::NEG_INFINITY,
    ];
    let original: VecDeque<u64> = floats.iter().map(|f| f.to_bits()).collect();
    let bytes = encode_to_vec(&original).expect("encode VecDeque<u64> of f64 bits");
    let (decoded, _bytes): (VecDeque<u64>, usize) =
        decode_from_slice(&bytes).expect("decode VecDeque<u64> of f64 bits");
    assert_eq!(original, decoded);
    // Verify the bit patterns round-trip to the same f64 values
    let recovered: Vec<f64> = decoded.iter().map(|&b| f64::from_bits(b)).collect();
    for (orig, rec) in floats.iter().zip(recovered.iter()) {
        assert_eq!(orig.to_bits(), rec.to_bits(), "f64 bit pattern mismatch");
    }
}

// ---------------------------------------------------------------------------
// Test 18: Struct { items: VecDeque<u32>, name: String } roundtrip
// ---------------------------------------------------------------------------

#[derive(Debug, PartialEq, Encode, Decode)]
struct NamedQueue {
    items: VecDeque<u32>,
    name: String,
}

#[test]
fn test_struct_with_vecdeque_field_roundtrip() {
    let original = NamedQueue {
        items: vec![10u32, 20, 30, 40].into_iter().collect(),
        name: "my-queue".to_string(),
    };
    let bytes = encode_to_vec(&original).expect("encode NamedQueue");
    let (decoded, _bytes): (NamedQueue, usize) =
        decode_from_slice(&bytes).expect("decode NamedQueue");
    assert_eq!(original, decoded);
    assert_eq!(decoded.name, "my-queue");
    assert_eq!(decoded.items.len(), 4);
}

// ---------------------------------------------------------------------------
// Test 19: Large VecDeque (100 elements) roundtrip
// ---------------------------------------------------------------------------

#[test]
fn test_vecdeque_large_100_elements_roundtrip() {
    let original: VecDeque<u32> = (0u32..100).collect();
    let bytes = encode_to_vec(&original).expect("encode 100-element VecDeque<u32>");
    let (decoded, _bytes): (VecDeque<u32>, usize) =
        decode_from_slice(&bytes).expect("decode 100-element VecDeque<u32>");
    assert_eq!(original, decoded);
    assert_eq!(decoded.len(), 100);
    let as_vec: Vec<u32> = decoded.into_iter().collect();
    for (i, &v) in as_vec.iter().enumerate() {
        assert_eq!(v, i as u32, "element at index {i} should equal {i}");
    }
}

// ---------------------------------------------------------------------------
// Test 20: VecDeque<(u32, String)> tuple elements roundtrip
// ---------------------------------------------------------------------------

#[test]
fn test_vecdeque_tuple_elements_roundtrip() {
    let original: VecDeque<(u32, String)> = vec![
        (1u32, "one".to_string()),
        (2, "two".to_string()),
        (3, "three".to_string()),
    ]
    .into_iter()
    .collect();
    let bytes = encode_to_vec(&original).expect("encode VecDeque<(u32, String)>");
    let (decoded, _bytes): (VecDeque<(u32, String)>, usize) =
        decode_from_slice(&bytes).expect("decode VecDeque<(u32, String)>");
    assert_eq!(original, decoded);
    let items: Vec<(u32, String)> = decoded.into_iter().collect();
    assert_eq!(items[0], (1, "one".to_string()));
    assert_eq!(items[2], (3, "three".to_string()));
}

// ---------------------------------------------------------------------------
// Test 21: VecDeque<Option<u32>> roundtrip
// ---------------------------------------------------------------------------

#[test]
fn test_vecdeque_option_u32_roundtrip() {
    let original: VecDeque<Option<u32>> = vec![Some(1u32), None, Some(42), None, Some(u32::MAX)]
        .into_iter()
        .collect();
    let bytes = encode_to_vec(&original).expect("encode VecDeque<Option<u32>>");
    let (decoded, _bytes): (VecDeque<Option<u32>>, usize) =
        decode_from_slice(&bytes).expect("decode VecDeque<Option<u32>>");
    assert_eq!(original, decoded);
    assert_eq!(decoded[1], None);
    assert_eq!(decoded[2], Some(42));
}

// ---------------------------------------------------------------------------
// Test 22: Re-encode decoded VecDeque gives same bytes
// ---------------------------------------------------------------------------

#[test]
fn test_vecdeque_reencode_gives_same_bytes() {
    let original: VecDeque<u32> = vec![7u32, 14, 21, 28, 35].into_iter().collect();
    let bytes1 = encode_to_vec(&original).expect("first encode VecDeque<u32>");
    let (decoded, _bytes): (VecDeque<u32>, usize) =
        decode_from_slice(&bytes1).expect("decode VecDeque<u32>");
    let bytes2 = encode_to_vec(&decoded).expect("re-encode decoded VecDeque<u32>");
    assert_eq!(
        bytes1, bytes2,
        "re-encoding a decoded VecDeque must produce identical bytes"
    );
}
