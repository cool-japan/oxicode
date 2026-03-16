//! Tests for smart pointer types: Box<T>, Rc<T>, Arc<T>.

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

#[test]
fn test_box_u32_roundtrip() {
    let b: Box<u32> = Box::new(42);
    let enc = encode_to_vec(&b).expect("encode");
    let (dec, _): (Box<u32>, _) = decode_from_slice(&enc).expect("decode");
    assert_eq!(b, dec);
}

#[test]
fn test_box_string_roundtrip() {
    let b: Box<String> = Box::new("hello".to_string());
    let enc = encode_to_vec(&b).expect("encode");
    let (dec, _): (Box<String>, _) = decode_from_slice(&enc).expect("decode");
    assert_eq!(b, dec);
}

#[test]
fn test_box_same_as_inner() {
    let val = vec![1u32, 2, 3];
    let boxed: Box<Vec<u32>> = Box::new(val.clone());

    let enc_boxed = encode_to_vec(&boxed).expect("encode boxed");
    let enc_raw = encode_to_vec(&val).expect("encode raw");
    assert_eq!(enc_boxed, enc_raw, "Box<T> should encode identically to T");
}

#[test]
fn test_rc_u64_roundtrip() {
    use std::rc::Rc;
    let r: Rc<u64> = Rc::new(999);
    let enc = encode_to_vec(&r).expect("encode");
    let (dec, _): (Rc<u64>, _) = decode_from_slice(&enc).expect("decode");
    assert_eq!(*r, *dec);
}

#[test]
fn test_arc_string_roundtrip() {
    use std::sync::Arc;
    let a: Arc<String> = Arc::new("arc_string".to_string());
    let enc = encode_to_vec(&a).expect("encode");
    let (dec, _): (Arc<String>, _) = decode_from_slice(&enc).expect("decode");
    assert_eq!(*a, *dec);
}

#[test]
fn test_nested_box_roundtrip() {
    let b: Box<Vec<Box<u32>>> = Box::new(vec![Box::new(1), Box::new(2), Box::new(3)]);
    let enc = encode_to_vec(&b).expect("encode");
    let (dec, _): (Box<Vec<Box<u32>>>, _) = decode_from_slice(&enc).expect("decode");
    assert_eq!(b, dec);
}
