//! Roundtrip tests for boxed and Arc slice/str types.

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
use std::sync::Arc;

#[test]
fn test_box_slice_roundtrip() {
    let original: Box<[u32]> = vec![1, 2, 3, 4, 5].into_boxed_slice();
    let enc = encode_to_vec(&original).expect("encode");
    let (dec, _): (Box<[u32]>, _) = decode_from_slice(&enc).expect("decode");
    assert_eq!(&*original, &*dec);
}

#[test]
fn test_box_str_roundtrip() {
    let original: Box<str> = "hello world".into();
    let enc = encode_to_vec(&original).expect("encode");
    let (dec, _): (Box<str>, _) = decode_from_slice(&enc).expect("decode");
    assert_eq!(&*original, &*dec);
}

#[test]
fn test_arc_slice_roundtrip() {
    let original: Arc<[i32]> = Arc::from(vec![-1, 0, 1, 2].as_slice());
    let enc = encode_to_vec(&original).expect("encode");
    let (dec, _): (Arc<[i32]>, _) = decode_from_slice(&enc).expect("decode");
    assert_eq!(&*original, &*dec);
}

#[test]
fn test_arc_str_roundtrip() {
    let original: Arc<str> = Arc::from("rust");
    let enc = encode_to_vec(&original).expect("encode");
    let (dec, _): (Arc<str>, _) = decode_from_slice(&enc).expect("decode");
    assert_eq!(&*original, &*dec);
}

#[test]
fn test_box_slice_wire_compatible_with_vec() {
    // Box<[T]> and Vec<T> should produce identical wire bytes
    let data = vec![10u8, 20, 30];
    let vec_enc = encode_to_vec(&data).expect("encode vec");
    let boxed: Box<[u8]> = data.into_boxed_slice();
    let box_enc = encode_to_vec(&boxed).expect("encode boxed");
    assert_eq!(
        vec_enc, box_enc,
        "Box<[T]> and Vec<T> must have identical wire format"
    );
}

#[test]
fn test_box_slice_empty() {
    let original: Box<[u64]> = vec![].into_boxed_slice();
    let enc = encode_to_vec(&original).expect("encode");
    let (dec, _): (Box<[u64]>, _) = decode_from_slice(&enc).expect("decode");
    assert_eq!(original.len(), dec.len());
}

#[test]
fn test_box_str_empty() {
    let original: Box<str> = "".into();
    let enc = encode_to_vec(&original).expect("encode");
    let (dec, _): (Box<str>, _) = decode_from_slice(&enc).expect("decode");
    assert_eq!(&*original, &*dec);
}

#[test]
fn test_arc_slice_empty() {
    let original: Arc<[f64]> = Arc::from(vec![].as_slice());
    let enc = encode_to_vec(&original).expect("encode");
    let (dec, _): (Arc<[f64]>, _) = decode_from_slice(&enc).expect("decode");
    assert_eq!(original.len(), dec.len());
}

#[test]
fn test_arc_str_empty() {
    let original: Arc<str> = Arc::from("");
    let enc = encode_to_vec(&original).expect("encode");
    let (dec, _): (Arc<str>, _) = decode_from_slice(&enc).expect("decode");
    assert_eq!(&*original, &*dec);
}

#[test]
fn test_rc_slice_roundtrip() {
    use std::rc::Rc;
    let original: Rc<[u32]> = Rc::from(vec![10u32, 20, 30].as_slice());
    let enc = encode_to_vec(&original).expect("encode");
    let (dec, _): (Rc<[u32]>, _) = decode_from_slice(&enc).expect("decode");
    assert_eq!(&*original, &*dec);
}

#[test]
fn test_rc_str_roundtrip() {
    use std::rc::Rc;
    let original: Rc<str> = Rc::from("oxicode");
    let enc = encode_to_vec(&original).expect("encode");
    let (dec, _): (Rc<str>, _) = decode_from_slice(&enc).expect("decode");
    assert_eq!(&*original, &*dec);
}

#[test]
fn test_vecdeque_roundtrip() {
    use std::collections::VecDeque;
    let original: VecDeque<i32> = vec![1, 2, 3, 4, 5].into_iter().collect();
    let enc = encode_to_vec(&original).expect("encode");
    let (dec, _): (VecDeque<i32>, _) = decode_from_slice(&enc).expect("decode");
    assert_eq!(original, dec);
}

#[test]
fn test_linked_list_roundtrip() {
    use std::collections::LinkedList;
    let mut original: LinkedList<u64> = LinkedList::new();
    original.push_back(100);
    original.push_back(200);
    original.push_back(300);
    let enc = encode_to_vec(&original).expect("encode");
    let (dec, _): (LinkedList<u64>, _) = decode_from_slice(&enc).expect("decode");
    assert_eq!(original, dec);
}

#[test]
fn test_linked_list_empty() {
    use std::collections::LinkedList;
    let original: LinkedList<u8> = LinkedList::new();
    let enc = encode_to_vec(&original).expect("encode");
    let (dec, _): (LinkedList<u8>, _) = decode_from_slice(&enc).expect("decode");
    assert_eq!(original, dec);
}
