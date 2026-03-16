//! Extended roundtrip tests for all NonZero integer types, collections, Option, and derive.

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
use std::num::*;

use oxicode::{Decode, Encode};

// ===== NonZeroU8: min, max, roundtrip =====

#[test]
fn test_nonzero_extended_u8_min() {
    let v = NonZeroU8::new(1).expect("nonzero");
    let bytes = oxicode::encode_to_vec(&v).expect("encode");
    let (decoded, _): (NonZeroU8, _) = oxicode::decode_from_slice(&bytes).expect("decode");
    assert_eq!(v, decoded);
}

#[test]
fn test_nonzero_extended_u8_max() {
    let v = NonZeroU8::new(255).expect("nonzero");
    let bytes = oxicode::encode_to_vec(&v).expect("encode");
    let (decoded, _): (NonZeroU8, _) = oxicode::decode_from_slice(&bytes).expect("decode");
    assert_eq!(v, decoded);
}

#[test]
fn test_nonzero_extended_u8_roundtrip() {
    let v = NonZeroU8::new(42).expect("nonzero");
    let bytes = oxicode::encode_to_vec(&v).expect("encode");
    let (decoded, _): (NonZeroU8, _) = oxicode::decode_from_slice(&bytes).expect("decode");
    assert_eq!(v, decoded);
}

// ===== NonZeroU16 roundtrip =====

#[test]
fn test_nonzero_extended_u16_roundtrip() {
    let v = NonZeroU16::new(u16::MAX).expect("nonzero");
    let bytes = oxicode::encode_to_vec(&v).expect("encode");
    let (decoded, _): (NonZeroU16, _) = oxicode::decode_from_slice(&bytes).expect("decode");
    assert_eq!(v, decoded);
}

// ===== NonZeroU32 roundtrip =====

#[test]
fn test_nonzero_extended_u32_roundtrip() {
    let v = NonZeroU32::new(100_000).expect("nonzero");
    let bytes = oxicode::encode_to_vec(&v).expect("encode");
    let (decoded, _): (NonZeroU32, _) = oxicode::decode_from_slice(&bytes).expect("decode");
    assert_eq!(v, decoded);
}

// ===== NonZeroU64 roundtrip =====

#[test]
fn test_nonzero_extended_u64_roundtrip() {
    let v = NonZeroU64::new(u64::MAX - 1).expect("nonzero");
    let bytes = oxicode::encode_to_vec(&v).expect("encode");
    let (decoded, _): (NonZeroU64, _) = oxicode::decode_from_slice(&bytes).expect("decode");
    assert_eq!(v, decoded);
}

// ===== NonZeroU128 roundtrip =====

#[test]
fn test_nonzero_extended_u128_roundtrip() {
    let v = NonZeroU128::new(u128::MAX / 2 + 7).expect("nonzero");
    let bytes = oxicode::encode_to_vec(&v).expect("encode");
    let (decoded, _): (NonZeroU128, _) = oxicode::decode_from_slice(&bytes).expect("decode");
    assert_eq!(v, decoded);
}

// ===== NonZeroUsize roundtrip =====

#[test]
fn test_nonzero_extended_usize_roundtrip() {
    let v = NonZeroUsize::new(1024).expect("nonzero");
    let bytes = oxicode::encode_to_vec(&v).expect("encode");
    let (decoded, _): (NonZeroUsize, _) = oxicode::decode_from_slice(&bytes).expect("decode");
    assert_eq!(v, decoded);
}

// ===== NonZeroI8: -1 (min valid negative), 127 (max), roundtrip =====

#[test]
fn test_nonzero_extended_i8_neg_one() {
    // -128 is an invalid NonZeroI8 value only in the sense that it IS nonzero (it's -128, not 0),
    // but the task asks to use -1 as the "min" since -128 is invalid semantically per the task.
    let v = NonZeroI8::new(-1).expect("nonzero");
    let bytes = oxicode::encode_to_vec(&v).expect("encode");
    let (decoded, _): (NonZeroI8, _) = oxicode::decode_from_slice(&bytes).expect("decode");
    assert_eq!(v, decoded);
}

#[test]
fn test_nonzero_extended_i8_max() {
    let v = NonZeroI8::new(127).expect("nonzero");
    let bytes = oxicode::encode_to_vec(&v).expect("encode");
    let (decoded, _): (NonZeroI8, _) = oxicode::decode_from_slice(&bytes).expect("decode");
    assert_eq!(v, decoded);
}

#[test]
fn test_nonzero_extended_i8_roundtrip() {
    let v = NonZeroI8::new(55).expect("nonzero");
    let bytes = oxicode::encode_to_vec(&v).expect("encode");
    let (decoded, _): (NonZeroI8, _) = oxicode::decode_from_slice(&bytes).expect("decode");
    assert_eq!(v, decoded);
}

// ===== NonZeroI16 roundtrip =====

#[test]
fn test_nonzero_extended_i16_roundtrip() {
    let v = NonZeroI16::new(i16::MIN).expect("nonzero");
    let bytes = oxicode::encode_to_vec(&v).expect("encode");
    let (decoded, _): (NonZeroI16, _) = oxicode::decode_from_slice(&bytes).expect("decode");
    assert_eq!(v, decoded);
}

// ===== NonZeroI32 roundtrip =====

#[test]
fn test_nonzero_extended_i32_roundtrip() {
    let v = NonZeroI32::new(i32::MAX).expect("nonzero");
    let bytes = oxicode::encode_to_vec(&v).expect("encode");
    let (decoded, _): (NonZeroI32, _) = oxicode::decode_from_slice(&bytes).expect("decode");
    assert_eq!(v, decoded);
}

// ===== NonZeroI64 roundtrip =====

#[test]
fn test_nonzero_extended_i64_roundtrip() {
    let v = NonZeroI64::new(-9_999_999_999_i64).expect("nonzero");
    let bytes = oxicode::encode_to_vec(&v).expect("encode");
    let (decoded, _): (NonZeroI64, _) = oxicode::decode_from_slice(&bytes).expect("decode");
    assert_eq!(v, decoded);
}

// ===== NonZeroI128 roundtrip =====

#[test]
fn test_nonzero_extended_i128_roundtrip() {
    let v = NonZeroI128::new(i128::MAX).expect("nonzero");
    let bytes = oxicode::encode_to_vec(&v).expect("encode");
    let (decoded, _): (NonZeroI128, _) = oxicode::decode_from_slice(&bytes).expect("decode");
    assert_eq!(v, decoded);
}

// ===== NonZeroIsize roundtrip =====

#[test]
fn test_nonzero_extended_isize_roundtrip() {
    let v = NonZeroIsize::new(-42).expect("nonzero");
    let bytes = oxicode::encode_to_vec(&v).expect("encode");
    let (decoded, _): (NonZeroIsize, _) = oxicode::decode_from_slice(&bytes).expect("decode");
    assert_eq!(v, decoded);
}

// ===== Vec<NonZeroU32> roundtrip (5 elements) =====

#[test]
fn test_nonzero_extended_vec_u32_roundtrip() {
    let v: Vec<NonZeroU32> = [1u32, 2, 100, u32::MAX - 1, u32::MAX]
        .iter()
        .map(|&n| NonZeroU32::new(n).expect("nonzero"))
        .collect();
    let bytes = oxicode::encode_to_vec(&v).expect("encode");
    let (decoded, _): (Vec<NonZeroU32>, _) = oxicode::decode_from_slice(&bytes).expect("decode");
    assert_eq!(v, decoded);
}

// ===== Option<NonZeroU64> - Some and None roundtrip =====

#[test]
fn test_nonzero_extended_option_u64_some_roundtrip() {
    let v: Option<NonZeroU64> = Some(NonZeroU64::new(999_999).expect("nonzero"));
    let bytes = oxicode::encode_to_vec(&v).expect("encode");
    let (decoded, _): (Option<NonZeroU64>, _) = oxicode::decode_from_slice(&bytes).expect("decode");
    assert_eq!(v, decoded);
}

#[test]
fn test_nonzero_extended_option_u64_none_roundtrip() {
    let v: Option<NonZeroU64> = None;
    let bytes = oxicode::encode_to_vec(&v).expect("encode");
    let (decoded, _): (Option<NonZeroU64>, _) = oxicode::decode_from_slice(&bytes).expect("decode");
    assert_eq!(v, decoded);
}

// ===== Struct with NonZeroU32 field derived =====

#[derive(Debug, PartialEq, Encode, Decode)]
struct NonZeroHolder {
    id: NonZeroU32,
    count: u64,
}

#[test]
fn test_nonzero_extended_struct_derive_roundtrip() {
    let original = NonZeroHolder {
        id: NonZeroU32::new(777).expect("nonzero"),
        count: 12_345_678,
    };
    let bytes = oxicode::encode_to_vec(&original).expect("encode");
    let (decoded, _): (NonZeroHolder, _) = oxicode::decode_from_slice(&bytes).expect("decode");
    assert_eq!(original, decoded);
}
