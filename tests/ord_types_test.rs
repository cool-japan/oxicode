//! Tests for std::cmp::Ordering and std::ops::ControlFlow encode/decode.

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
use std::cmp::Ordering;

#[test]
fn test_ordering_less_roundtrip() {
    let enc = encode_to_vec(&Ordering::Less).expect("encode");
    let (dec, _): (Ordering, _) = decode_from_slice(&enc).expect("decode");
    assert_eq!(Ordering::Less, dec);
}

#[test]
fn test_ordering_equal_roundtrip() {
    let enc = encode_to_vec(&Ordering::Equal).expect("encode");
    let (dec, _): (Ordering, _) = decode_from_slice(&enc).expect("decode");
    assert_eq!(Ordering::Equal, dec);
}

#[test]
fn test_ordering_greater_roundtrip() {
    let enc = encode_to_vec(&Ordering::Greater).expect("encode");
    let (dec, _): (Ordering, _) = decode_from_slice(&enc).expect("decode");
    assert_eq!(Ordering::Greater, dec);
}

#[test]
fn test_all_orderings_distinct_encoding() {
    let less = encode_to_vec(&Ordering::Less).expect("encode");
    let equal = encode_to_vec(&Ordering::Equal).expect("encode");
    let greater = encode_to_vec(&Ordering::Greater).expect("encode");

    assert_ne!(less, equal);
    assert_ne!(equal, greater);
    assert_ne!(less, greater);
}

// ControlFlow
#[test]
fn test_control_flow_continue_roundtrip() {
    use std::ops::ControlFlow;
    let cf: ControlFlow<(), i32> = ControlFlow::Continue(42);
    let enc = encode_to_vec(&cf).expect("encode");
    let (dec, _): (ControlFlow<(), i32>, _) = decode_from_slice(&enc).expect("decode");
    assert_eq!(cf, dec);
}

#[test]
fn test_control_flow_break_roundtrip() {
    use std::ops::ControlFlow;
    let cf: ControlFlow<String, ()> = ControlFlow::Break("stop".to_string());
    let enc = encode_to_vec(&cf).expect("encode");
    let (dec, _): (ControlFlow<String, ()>, _) = decode_from_slice(&enc).expect("decode");
    assert_eq!(cf, dec);
}
