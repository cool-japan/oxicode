//! Tests for standard library type implementations: Ordering, Infallible, ControlFlow

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
use std::convert::Infallible;
use std::ops::ControlFlow;

#[test]
fn test_ordering_roundtrip() {
    for ord in [Ordering::Less, Ordering::Equal, Ordering::Greater] {
        let enc = encode_to_vec(&ord).expect("encode");
        let (dec, _): (Ordering, _) = decode_from_slice(&enc).expect("decode");
        assert_eq!(ord, dec);
    }
}

#[test]
fn test_ordering_wire_format() {
    // i8 is always a single byte (not varint-encoded).
    let enc = encode_to_vec(&Ordering::Less).expect("encode");
    assert_eq!(enc[0], 0xFF_u8, "Less should encode as -1i8 = 0xFF");
    let enc = encode_to_vec(&Ordering::Equal).expect("encode");
    assert_eq!(enc[0], 0x00, "Equal should encode as 0i8 = 0x00");
    let enc = encode_to_vec(&Ordering::Greater).expect("encode");
    assert_eq!(enc[0], 0x01, "Greater should encode as 1i8 = 0x01");
}

#[test]
fn test_infallible_in_result_ok() {
    // We can only test encode/decode via Result<T, Infallible> since Infallible can't be constructed
    let ok: Result<u32, Infallible> = Ok(42);
    let enc = encode_to_vec(&ok).expect("encode");
    let (dec, _): (Result<u32, Infallible>, _) = decode_from_slice(&enc).expect("decode");
    assert_eq!(ok, dec);
}

#[test]
fn test_infallible_decode_always_errors() {
    // Any byte input to Decode for Infallible must fail
    let result: Result<(Infallible, usize), _> = decode_from_slice(&[0u8]);
    assert!(
        result.is_err(),
        "Infallible decode must always return an error"
    );
}

#[test]
fn test_control_flow_continue_roundtrip() {
    let cf: ControlFlow<String, u32> = ControlFlow::Continue(42);
    let enc = encode_to_vec(&cf).expect("encode");
    let (dec, _): (ControlFlow<String, u32>, _) = decode_from_slice(&enc).expect("decode");
    assert_eq!(cf, dec);
}

#[test]
fn test_control_flow_break_roundtrip() {
    let cf: ControlFlow<String, u32> = ControlFlow::Break("stop".to_string());
    let enc = encode_to_vec(&cf).expect("encode");
    let (dec, _): (ControlFlow<String, u32>, _) = decode_from_slice(&enc).expect("decode");
    assert_eq!(cf, dec);
}

#[test]
fn test_control_flow_unit_variants() {
    let cf_c: ControlFlow<(), ()> = ControlFlow::Continue(());
    let enc = encode_to_vec(&cf_c).expect("encode");
    let (dec, _): (ControlFlow<(), ()>, _) = decode_from_slice(&enc).expect("decode");
    assert_eq!(cf_c, dec);

    let cf_b: ControlFlow<(), ()> = ControlFlow::Break(());
    let enc = encode_to_vec(&cf_b).expect("encode");
    let (dec, _): (ControlFlow<(), ()>, _) = decode_from_slice(&enc).expect("decode");
    assert_eq!(cf_b, dec);
}
