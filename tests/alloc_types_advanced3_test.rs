//! Advanced tests for alloc crate types: String, Vec, Box, Rc, Arc, BTreeMap, BTreeSet, VecDeque, LinkedList.

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
use oxicode::{decode_from_slice, encode_to_vec};
use std::collections::{BTreeMap, BTreeSet, LinkedList, VecDeque};
use std::rc::Rc;
use std::sync::Arc;

// ----- String -----

#[test]
fn test_string_empty_roundtrip() {
    let original = String::new();
    let enc = encode_to_vec(&original).expect("encode empty String");
    let (val, _): (String, usize) = decode_from_slice(&enc).expect("decode empty String");
    assert_eq!(original, val);
}

#[test]
fn test_string_hello_roundtrip() {
    let original = String::from("hello");
    let enc = encode_to_vec(&original).expect("encode String");
    let (val, _): (String, usize) = decode_from_slice(&enc).expect("decode String");
    assert_eq!(original, val);
}

#[test]
fn test_string_special_chars_roundtrip() {
    let original = String::from("hello\nworld\t!\u{1F600}");
    let enc = encode_to_vec(&original).expect("encode special chars String");
    let (val, _): (String, usize) = decode_from_slice(&enc).expect("decode special chars String");
    assert_eq!(original, val);
}

#[test]
fn test_string_capacity_irrelevant() {
    let mut s1 = String::with_capacity(1024);
    s1.push_str("same content");
    let s2 = String::from("same content");
    let enc1 = encode_to_vec(&s1).expect("encode s1");
    let enc2 = encode_to_vec(&s2).expect("encode s2");
    assert_eq!(enc1, enc2, "capacity must not affect encoding");
}

// ----- Vec<T> -----

#[test]
fn test_vec_u32_empty_roundtrip() {
    let original: Vec<u32> = Vec::new();
    let enc = encode_to_vec(&original).expect("encode empty Vec<u32>");
    let (val, _): (Vec<u32>, usize) = decode_from_slice(&enc).expect("decode empty Vec<u32>");
    assert_eq!(original, val);
}

#[test]
fn test_vec_single_element_roundtrip() {
    let original: Vec<u32> = vec![42];
    let enc = encode_to_vec(&original).expect("encode single-element Vec");
    let (val, _): (Vec<u32>, usize) = decode_from_slice(&enc).expect("decode single-element Vec");
    assert_eq!(original, val);
}

#[test]
fn test_vec_100_elements_roundtrip() {
    let original: Vec<u32> = (0..100).collect();
    let enc = encode_to_vec(&original).expect("encode 100-element Vec");
    let (val, _): (Vec<u32>, usize) = decode_from_slice(&enc).expect("decode 100-element Vec");
    assert_eq!(original, val);
}

#[test]
fn test_vec_nested_roundtrip() {
    let original: Vec<Vec<u8>> = vec![vec![1, 2, 3], vec![], vec![255, 0]];
    let enc = encode_to_vec(&original).expect("encode nested Vec");
    let (val, _): (Vec<Vec<u8>>, usize) = decode_from_slice(&enc).expect("decode nested Vec");
    assert_eq!(original, val);
}

// ----- Box<T> -----

#[test]
fn test_box_u32_roundtrip() {
    let original: Box<u32> = Box::new(12345);
    let enc = encode_to_vec(&original).expect("encode Box<u32>");
    let (val, _): (Box<u32>, usize) = decode_from_slice(&enc).expect("decode Box<u32>");
    assert_eq!(original, val);
}

#[test]
fn test_box_string_roundtrip() {
    let original: Box<String> = Box::new(String::from("boxed string"));
    let enc = encode_to_vec(&original).expect("encode Box<String>");
    let (val, _): (Box<String>, usize) = decode_from_slice(&enc).expect("decode Box<String>");
    assert_eq!(original, val);
}

#[test]
fn test_box_vec_u8_roundtrip() {
    let original: Box<Vec<u8>> = Box::new(vec![10, 20, 30, 40, 50]);
    let enc = encode_to_vec(&original).expect("encode Box<Vec<u8>>");
    let (val, _): (Box<Vec<u8>>, usize) = decode_from_slice(&enc).expect("decode Box<Vec<u8>>");
    assert_eq!(original, val);
}

#[test]
fn test_box_encoding_transparent() {
    let inner: u32 = 99;
    let boxed: Box<u32> = Box::new(inner);
    let enc_inner = encode_to_vec(&inner).expect("encode raw u32");
    let enc_boxed = encode_to_vec(&boxed).expect("encode Box<u32>");
    assert_eq!(enc_inner, enc_boxed, "Box<T> must encode identically to T");
}

// ----- Rc<T> -----

#[test]
fn test_rc_u32_roundtrip() {
    let original: Rc<u32> = Rc::new(777);
    let enc = encode_to_vec(&original).expect("encode Rc<u32>");
    let (val, _): (Rc<u32>, usize) = decode_from_slice(&enc).expect("decode Rc<u32>");
    assert_eq!(*original, *val);
}

#[test]
fn test_rc_string_roundtrip() {
    let original: Rc<String> = Rc::new(String::from("rc string"));
    let enc = encode_to_vec(&original).expect("encode Rc<String>");
    let (val, _): (Rc<String>, usize) = decode_from_slice(&enc).expect("decode Rc<String>");
    assert_eq!(*original, *val);
}

#[test]
fn test_rc_and_box_same_bytes() {
    let inner = 42u32;
    let rc: Rc<u32> = Rc::new(inner);
    let boxed: Box<u32> = Box::new(inner);
    let enc_rc = encode_to_vec(&rc).expect("encode Rc<u32>");
    let enc_box = encode_to_vec(&boxed).expect("encode Box<u32>");
    assert_eq!(enc_rc, enc_box, "Rc<T> and Box<T> must encode identically");
}

// ----- Arc<T> -----

#[test]
fn test_arc_u32_roundtrip() {
    let original: Arc<u32> = Arc::new(9999);
    let enc = encode_to_vec(&original).expect("encode Arc<u32>");
    let (val, _): (Arc<u32>, usize) = decode_from_slice(&enc).expect("decode Arc<u32>");
    assert_eq!(*original, *val);
}

#[test]
fn test_arc_string_roundtrip() {
    let original: Arc<String> = Arc::new(String::from("arc string"));
    let enc = encode_to_vec(&original).expect("encode Arc<String>");
    let (val, _): (Arc<String>, usize) = decode_from_slice(&enc).expect("decode Arc<String>");
    assert_eq!(*original, *val);
}

#[test]
fn test_arc_and_rc_same_bytes() {
    let inner = 123u32;
    let arc: Arc<u32> = Arc::new(inner);
    let rc: Rc<u32> = Rc::new(inner);
    let enc_arc = encode_to_vec(&arc).expect("encode Arc<u32>");
    let enc_rc = encode_to_vec(&rc).expect("encode Rc<u32>");
    assert_eq!(enc_arc, enc_rc, "Arc<T> and Rc<T> must encode identically");
}

// ----- BTreeMap / BTreeSet -----

#[test]
fn test_btreemap_u32_roundtrip() {
    let mut original: BTreeMap<u32, u32> = BTreeMap::new();
    for i in 0..5 {
        original.insert(i, i * 10);
    }
    let enc = encode_to_vec(&original).expect("encode BTreeMap");
    let (val, _): (BTreeMap<u32, u32>, usize) = decode_from_slice(&enc).expect("decode BTreeMap");
    assert_eq!(original, val);
}

#[test]
fn test_btreeset_u32_roundtrip() {
    let original: BTreeSet<u32> = [10, 20, 30, 40, 50].iter().cloned().collect();
    let enc = encode_to_vec(&original).expect("encode BTreeSet");
    let (val, _): (BTreeSet<u32>, usize) = decode_from_slice(&enc).expect("decode BTreeSet");
    assert_eq!(original, val);
}

// ----- VecDeque / LinkedList -----

#[test]
fn test_vecdeque_u32_roundtrip() {
    let original: VecDeque<u32> = [1, 2, 3, 4, 5].iter().cloned().collect();
    let enc = encode_to_vec(&original).expect("encode VecDeque");
    let (val, _): (VecDeque<u32>, usize) = decode_from_slice(&enc).expect("decode VecDeque");
    assert_eq!(original, val);
}

#[test]
fn test_linkedlist_u32_roundtrip() {
    let original: LinkedList<u32> = [11, 22, 33, 44, 55].iter().cloned().collect();
    let enc = encode_to_vec(&original).expect("encode LinkedList");
    let (val, _): (LinkedList<u32>, usize) = decode_from_slice(&enc).expect("decode LinkedList");
    assert_eq!(original, val);
}
