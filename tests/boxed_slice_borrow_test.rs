//! Tests for Box<[T]>, Box<str>, Arc<[T]>, Arc<str> and their BorrowDecode implementations.

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
use std::sync::Arc;

// ===== Test 1: Box<str> roundtrip (empty) =====

#[test]
fn test_box_str_empty_roundtrip() {
    let original: Box<str> = "".into();
    let enc = encode_to_vec(&original).expect("encode Box<str> empty");
    let (dec, _): (Box<str>, _) = decode_from_slice(&enc).expect("decode Box<str> empty");
    assert_eq!(&*original, &*dec);
    assert!(dec.is_empty());
}

// ===== Test 2: Box<str> roundtrip (non-empty) =====

#[test]
fn test_box_str_nonempty_roundtrip() {
    let original: Box<str> = "hello, oxicode!".into();
    let enc = encode_to_vec(&original).expect("encode Box<str> non-empty");
    let (dec, _): (Box<str>, _) = decode_from_slice(&enc).expect("decode Box<str> non-empty");
    assert_eq!(&*original, &*dec);
}

// ===== Test 3: Box<[u8]> roundtrip (empty) =====

#[test]
fn test_box_u8_slice_empty_roundtrip() {
    let original: Box<[u8]> = vec![].into_boxed_slice();
    let enc = encode_to_vec(&original).expect("encode Box<[u8]> empty");
    let (dec, _): (Box<[u8]>, _) = decode_from_slice(&enc).expect("decode Box<[u8]> empty");
    assert_eq!(original.len(), dec.len());
    assert!(dec.is_empty());
}

// ===== Test 4: Box<[u8]> roundtrip (with data) =====

#[test]
fn test_box_u8_slice_data_roundtrip() {
    let original: Box<[u8]> = vec![0u8, 42, 127, 200, 255].into_boxed_slice();
    let enc = encode_to_vec(&original).expect("encode Box<[u8]> data");
    let (dec, _): (Box<[u8]>, _) = decode_from_slice(&enc).expect("decode Box<[u8]> data");
    assert_eq!(&*original, &*dec);
}

// ===== Test 5: Box<[u32]> roundtrip =====

#[test]
fn test_box_u32_slice_roundtrip() {
    let original: Box<[u32]> = vec![1u32, 2, 3, 4, 5, 1000, u32::MAX].into_boxed_slice();
    let enc = encode_to_vec(&original).expect("encode Box<[u32]>");
    let (dec, _): (Box<[u32]>, _) = decode_from_slice(&enc).expect("decode Box<[u32]>");
    assert_eq!(&*original, &*dec);
}

// ===== Test 6: Box<[String]> roundtrip =====

#[test]
fn test_box_string_slice_roundtrip() {
    let original: Box<[String]> = vec![
        String::from("alpha"),
        String::from("beta"),
        String::from("gamma"),
    ]
    .into_boxed_slice();
    let enc = encode_to_vec(&original).expect("encode Box<[String]>");
    let (dec, _): (Box<[String]>, _) = decode_from_slice(&enc).expect("decode Box<[String]>");
    assert_eq!(&*original, &*dec);
}

// ===== Test 7: Arc<str> roundtrip (empty) =====

#[test]
fn test_arc_str_empty_roundtrip() {
    let original: Arc<str> = Arc::from("");
    let enc = encode_to_vec(&original).expect("encode Arc<str> empty");
    let (dec, _): (Arc<str>, _) = decode_from_slice(&enc).expect("decode Arc<str> empty");
    assert_eq!(&*original, &*dec);
    assert!(dec.is_empty());
}

// ===== Test 8: Arc<str> roundtrip (non-empty) =====

#[test]
fn test_arc_str_nonempty_roundtrip() {
    let original: Arc<str> = Arc::from("arc string slice roundtrip");
    let enc = encode_to_vec(&original).expect("encode Arc<str> non-empty");
    let (dec, _): (Arc<str>, _) = decode_from_slice(&enc).expect("decode Arc<str> non-empty");
    assert_eq!(&*original, &*dec);
}

// ===== Test 9: Arc<[u8]> roundtrip (empty) =====

#[test]
fn test_arc_u8_slice_empty_roundtrip() {
    let original: Arc<[u8]> = Arc::from(vec![].as_slice());
    let enc = encode_to_vec(&original).expect("encode Arc<[u8]> empty");
    let (dec, _): (Arc<[u8]>, _) = decode_from_slice(&enc).expect("decode Arc<[u8]> empty");
    assert_eq!(original.len(), dec.len());
    assert!(dec.is_empty());
}

// ===== Test 10: Arc<[u8]> roundtrip (with data) =====

#[test]
fn test_arc_u8_slice_data_roundtrip() {
    let original: Arc<[u8]> = Arc::from(vec![10u8, 20, 30, 40, 50].as_slice());
    let enc = encode_to_vec(&original).expect("encode Arc<[u8]> data");
    let (dec, _): (Arc<[u8]>, _) = decode_from_slice(&enc).expect("decode Arc<[u8]> data");
    assert_eq!(&*original, &*dec);
}

// ===== Test 11: Arc<[u32]> roundtrip =====

#[test]
fn test_arc_u32_slice_roundtrip() {
    let original: Arc<[u32]> = Arc::from(vec![100u32, 200, 300, u32::MAX, 0].as_slice());
    let enc = encode_to_vec(&original).expect("encode Arc<[u32]>");
    let (dec, _): (Arc<[u32]>, _) = decode_from_slice(&enc).expect("decode Arc<[u32]>");
    assert_eq!(&*original, &*dec);
}

// ===== Test 12: Option<Box<str>> roundtrip =====

#[test]
fn test_option_box_str_roundtrip() {
    // Some variant
    let original_some: Option<Box<str>> = Some("optional boxed str".into());
    let enc_some = encode_to_vec(&original_some).expect("encode Option<Box<str>> Some");
    let (dec_some, _): (Option<Box<str>>, _) =
        decode_from_slice(&enc_some).expect("decode Option<Box<str>> Some");
    assert_eq!(original_some.as_deref(), dec_some.as_deref());

    // None variant
    let original_none: Option<Box<str>> = None;
    let enc_none = encode_to_vec(&original_none).expect("encode Option<Box<str>> None");
    let (dec_none, _): (Option<Box<str>>, _) =
        decode_from_slice(&enc_none).expect("decode Option<Box<str>> None");
    assert!(dec_none.is_none());
}

// ===== Test 13: Option<Arc<str>> roundtrip =====

#[test]
fn test_option_arc_str_roundtrip() {
    // Some variant
    let original_some: Option<Arc<str>> = Some(Arc::from("optional arc str"));
    let enc_some = encode_to_vec(&original_some).expect("encode Option<Arc<str>> Some");
    let (dec_some, _): (Option<Arc<str>>, _) =
        decode_from_slice(&enc_some).expect("decode Option<Arc<str>> Some");
    assert_eq!(original_some.as_deref(), dec_some.as_deref());

    // None variant
    let original_none: Option<Arc<str>> = None;
    let enc_none = encode_to_vec(&original_none).expect("encode Option<Arc<str>> None");
    let (dec_none, _): (Option<Arc<str>>, _) =
        decode_from_slice(&enc_none).expect("decode Option<Arc<str>> None");
    assert!(dec_none.is_none());
}

// ===== Test 14: Vec<Box<str>> roundtrip =====

#[test]
fn test_vec_box_str_roundtrip() {
    let original: Vec<Box<str>> = vec![
        "first".into(),
        "second".into(),
        "".into(),
        "fourth with spaces".into(),
    ];
    let enc = encode_to_vec(&original).expect("encode Vec<Box<str>>");
    let (dec, _): (Vec<Box<str>>, _) = decode_from_slice(&enc).expect("decode Vec<Box<str>>");
    assert_eq!(original.len(), dec.len());
    for (a, b) in original.iter().zip(dec.iter()) {
        assert_eq!(&**a, &**b);
    }
}

// ===== Test 15: Vec<Arc<str>> roundtrip =====

#[test]
fn test_vec_arc_str_roundtrip() {
    let original: Vec<Arc<str>> = vec![
        Arc::from("arc one"),
        Arc::from("arc two"),
        Arc::from(""),
        Arc::from("arc four"),
    ];
    let enc = encode_to_vec(&original).expect("encode Vec<Arc<str>>");
    let (dec, _): (Vec<Arc<str>>, _) = decode_from_slice(&enc).expect("decode Vec<Arc<str>>");
    assert_eq!(original.len(), dec.len());
    for (a, b) in original.iter().zip(dec.iter()) {
        assert_eq!(&**a, &**b);
    }
}

// ===== Test 16: Struct with Box<str> and Arc<[u8]> fields + derive =====

#[derive(Debug, PartialEq, Encode, Decode)]
struct BorrowableRecord {
    label: Box<str>,
    data: Arc<[u8]>,
    count: u32,
}

#[test]
fn test_struct_with_box_str_and_arc_slice_derive() {
    let original = BorrowableRecord {
        label: "record-label".into(),
        data: Arc::from(vec![0xde_u8, 0xad, 0xbe, 0xef].as_slice()),
        count: 42,
    };
    let enc = encode_to_vec(&original).expect("encode BorrowableRecord");
    let (dec, _): (BorrowableRecord, _) = decode_from_slice(&enc).expect("decode BorrowableRecord");
    assert_eq!(original, dec);
    assert_eq!(&*original.label, &*dec.label);
    assert_eq!(&*original.data, &*dec.data);
    assert_eq!(original.count, dec.count);
}

// ===== Test 17: Verify Box<str> and String produce same bytes for same content =====

#[test]
fn test_box_str_and_string_same_wire_format() {
    let content = "wire format compatibility";

    let string_val = String::from(content);
    let box_str_val: Box<str> = content.into();

    let string_enc = encode_to_vec(&string_val).expect("encode String");
    let box_str_enc = encode_to_vec(&box_str_val).expect("encode Box<str>");

    assert_eq!(
        string_enc, box_str_enc,
        "Box<str> and String must produce identical wire bytes for the same content"
    );
}

// ===== Test 18: Verify Arc<[u8]> and Vec<u8> produce same bytes for same content =====

#[test]
fn test_arc_u8_slice_and_vec_u8_same_wire_format() {
    let content = vec![1u8, 2, 3, 100, 200, 255];

    let vec_val: Vec<u8> = content.clone();
    let arc_val: Arc<[u8]> = Arc::from(content.as_slice());

    let vec_enc = encode_to_vec(&vec_val).expect("encode Vec<u8>");
    let arc_enc = encode_to_vec(&arc_val).expect("encode Arc<[u8]>");

    assert_eq!(
        vec_enc, arc_enc,
        "Arc<[u8]> and Vec<u8> must produce identical wire bytes for the same content"
    );
}
