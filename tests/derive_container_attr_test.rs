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
use oxicode::{Decode, Encode};
use std::marker::PhantomData;

// Test 1: #[oxicode(bound = "")] removes auto-bound for PhantomData field
#[derive(Debug, PartialEq, Encode, Decode)]
#[oxicode(bound = "")] // no bounds needed — T is only phantom
struct PhantomWrapper<T> {
    #[oxicode(skip)]
    _marker: PhantomData<T>,
    value: u32,
}

#[test]
fn test_bound_empty_for_phantom() {
    let w: PhantomWrapper<String> = PhantomWrapper {
        _marker: PhantomData,
        value: 42,
    };
    let enc = oxicode::encode_to_vec(&w).expect("encode");
    let (dec, _): (PhantomWrapper<String>, _) = oxicode::decode_from_slice(&enc).expect("decode");
    assert_eq!(w, dec);
}

// Test 2: #[oxicode(bound)] with custom bounds
#[derive(Debug, PartialEq, Encode, Decode)]
#[oxicode(bound = "T: Encode + Decode + core::fmt::Debug + PartialEq")]
struct CustomBound<T> {
    inner: T,
}

#[test]
fn test_custom_bound() {
    let v = CustomBound { inner: 99u64 };
    let enc = oxicode::encode_to_vec(&v).expect("encode");
    let (dec, _): (CustomBound<u64>, _) = oxicode::decode_from_slice(&enc).expect("decode");
    assert_eq!(v, dec);
}

// Test 3: #[oxicode(rename_all)] — accepted without error, no-op on wire
#[derive(Debug, PartialEq, Encode, Decode)]
#[oxicode(rename_all = "camelCase")]
struct CamelStruct {
    first_name: String,
    last_name: String,
}

#[test]
fn test_rename_all_noop_on_wire() {
    let s = CamelStruct {
        first_name: "Alice".into(),
        last_name: "Smith".into(),
    };
    let enc = oxicode::encode_to_vec(&s).expect("encode");
    let (dec, _): (CamelStruct, _) = oxicode::decode_from_slice(&enc).expect("decode");
    assert_eq!(s, dec);
}

// Test 4: #[oxicode(crate = "oxicode")] — works with explicit crate path
#[derive(Debug, PartialEq, Encode, Decode)]
#[oxicode(crate = "oxicode")]
struct ExplicitCrate {
    x: u32,
}

#[test]
fn test_explicit_crate_path() {
    let v = ExplicitCrate { x: 7 };
    let enc = oxicode::encode_to_vec(&v).expect("encode");
    let (dec, _): (ExplicitCrate, _) = oxicode::decode_from_slice(&enc).expect("decode");
    assert_eq!(v, dec);
}
