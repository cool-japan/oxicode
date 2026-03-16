//! Roundtrip and error tests for all 12 NonZero integer types.

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
use core::num::{
    NonZeroI128, NonZeroI16, NonZeroI32, NonZeroI64, NonZeroI8, NonZeroIsize, NonZeroU128,
    NonZeroU16, NonZeroU32, NonZeroU64, NonZeroU8, NonZeroUsize,
};

// ===== Roundtrip tests =====

#[test]
fn test_nonzero_u8_roundtrip() {
    let original = NonZeroU8::new(255).expect("nonzero");
    let bytes = oxicode::encode_to_vec(&original).expect("encode");
    let (decoded, _): (NonZeroU8, _) = oxicode::decode_from_slice(&bytes).expect("decode");
    assert_eq!(original, decoded);
}

#[test]
fn test_nonzero_u16_roundtrip() {
    let original = NonZeroU16::new(1000).expect("nonzero");
    let bytes = oxicode::encode_to_vec(&original).expect("encode");
    let (decoded, _): (NonZeroU16, _) = oxicode::decode_from_slice(&bytes).expect("decode");
    assert_eq!(original, decoded);
}

#[test]
fn test_nonzero_u32_roundtrip() {
    let original = NonZeroU32::new(u32::MAX).expect("nonzero");
    let bytes = oxicode::encode_to_vec(&original).expect("encode");
    let (decoded, _): (NonZeroU32, _) = oxicode::decode_from_slice(&bytes).expect("decode");
    assert_eq!(original, decoded);
}

#[test]
fn test_nonzero_u64_roundtrip() {
    let original = NonZeroU64::new(123456789).expect("nonzero");
    let bytes = oxicode::encode_to_vec(&original).expect("encode");
    let (decoded, _): (NonZeroU64, _) = oxicode::decode_from_slice(&bytes).expect("decode");
    assert_eq!(original, decoded);
}

#[test]
fn test_nonzero_u128_roundtrip() {
    let original = NonZeroU128::new(u128::MAX).expect("nonzero");
    let bytes = oxicode::encode_to_vec(&original).expect("encode");
    let (decoded, _): (NonZeroU128, _) = oxicode::decode_from_slice(&bytes).expect("decode");
    assert_eq!(original, decoded);
}

#[test]
fn test_nonzero_usize_roundtrip() {
    let original = NonZeroUsize::new(usize::MAX).expect("nonzero");
    let bytes = oxicode::encode_to_vec(&original).expect("encode");
    let (decoded, _): (NonZeroUsize, _) = oxicode::decode_from_slice(&bytes).expect("decode");
    assert_eq!(original, decoded);
}

#[test]
fn test_nonzero_i8_roundtrip() {
    let original = NonZeroI8::new(-42).expect("nonzero");
    let bytes = oxicode::encode_to_vec(&original).expect("encode");
    let (decoded, _): (NonZeroI8, _) = oxicode::decode_from_slice(&bytes).expect("decode");
    assert_eq!(original, decoded);
}

#[test]
fn test_nonzero_i16_roundtrip() {
    let original = NonZeroI16::new(-1000).expect("nonzero");
    let bytes = oxicode::encode_to_vec(&original).expect("encode");
    let (decoded, _): (NonZeroI16, _) = oxicode::decode_from_slice(&bytes).expect("decode");
    assert_eq!(original, decoded);
}

#[test]
fn test_nonzero_i32_roundtrip() {
    let original = NonZeroI32::new(i32::MIN).expect("nonzero");
    let bytes = oxicode::encode_to_vec(&original).expect("encode");
    let (decoded, _): (NonZeroI32, _) = oxicode::decode_from_slice(&bytes).expect("decode");
    assert_eq!(original, decoded);
}

#[test]
fn test_nonzero_i64_roundtrip() {
    let original = NonZeroI64::new(i64::MAX).expect("nonzero");
    let bytes = oxicode::encode_to_vec(&original).expect("encode");
    let (decoded, _): (NonZeroI64, _) = oxicode::decode_from_slice(&bytes).expect("decode");
    assert_eq!(original, decoded);
}

#[test]
fn test_nonzero_i128_roundtrip() {
    let original = NonZeroI128::new(i128::MIN).expect("nonzero");
    let bytes = oxicode::encode_to_vec(&original).expect("encode");
    let (decoded, _): (NonZeroI128, _) = oxicode::decode_from_slice(&bytes).expect("decode");
    assert_eq!(original, decoded);
}

#[test]
fn test_nonzero_isize_roundtrip() {
    let original = NonZeroIsize::new(-1).expect("nonzero");
    let bytes = oxicode::encode_to_vec(&original).expect("encode");
    let (decoded, _): (NonZeroIsize, _) = oxicode::decode_from_slice(&bytes).expect("decode");
    assert_eq!(original, decoded);
}

// ===== Zero-value error tests =====

#[test]
fn test_nonzero_u8_zero_is_error() {
    let zero_bytes = oxicode::encode_to_vec(&0u8).expect("encode");
    let result: Result<(NonZeroU8, usize), _> = oxicode::decode_from_slice(&zero_bytes);
    assert!(result.is_err(), "decoding zero as NonZeroU8 must fail");
}

#[test]
fn test_nonzero_u16_zero_is_error() {
    let zero_bytes = oxicode::encode_to_vec(&0u16).expect("encode");
    let result: Result<(NonZeroU16, usize), _> = oxicode::decode_from_slice(&zero_bytes);
    assert!(result.is_err(), "decoding zero as NonZeroU16 must fail");
}

#[test]
fn test_nonzero_u32_zero_is_error() {
    let zero_bytes = oxicode::encode_to_vec(&0u32).expect("encode");
    let result: Result<(NonZeroU32, usize), _> = oxicode::decode_from_slice(&zero_bytes);
    assert!(result.is_err(), "decoding zero as NonZeroU32 must fail");
}

#[test]
fn test_nonzero_u64_zero_is_error() {
    let zero_bytes = oxicode::encode_to_vec(&0u64).expect("encode");
    let result: Result<(NonZeroU64, usize), _> = oxicode::decode_from_slice(&zero_bytes);
    assert!(result.is_err(), "decoding zero as NonZeroU64 must fail");
}

#[test]
fn test_nonzero_u128_zero_is_error() {
    let zero_bytes = oxicode::encode_to_vec(&0u128).expect("encode");
    let result: Result<(NonZeroU128, usize), _> = oxicode::decode_from_slice(&zero_bytes);
    assert!(result.is_err(), "decoding zero as NonZeroU128 must fail");
}

#[test]
fn test_nonzero_usize_zero_is_error() {
    let zero_bytes = oxicode::encode_to_vec(&0usize).expect("encode");
    let result: Result<(NonZeroUsize, usize), _> = oxicode::decode_from_slice(&zero_bytes);
    assert!(result.is_err(), "decoding zero as NonZeroUsize must fail");
}

#[test]
fn test_nonzero_i8_zero_is_error() {
    let zero_bytes = oxicode::encode_to_vec(&0i8).expect("encode");
    let result: Result<(NonZeroI8, usize), _> = oxicode::decode_from_slice(&zero_bytes);
    assert!(result.is_err(), "decoding zero as NonZeroI8 must fail");
}

#[test]
fn test_nonzero_i16_zero_is_error() {
    let zero_bytes = oxicode::encode_to_vec(&0i16).expect("encode");
    let result: Result<(NonZeroI16, usize), _> = oxicode::decode_from_slice(&zero_bytes);
    assert!(result.is_err(), "decoding zero as NonZeroI16 must fail");
}

#[test]
fn test_nonzero_i32_zero_is_error() {
    let zero_bytes = oxicode::encode_to_vec(&0i32).expect("encode");
    let result: Result<(NonZeroI32, usize), _> = oxicode::decode_from_slice(&zero_bytes);
    assert!(result.is_err(), "decoding zero as NonZeroI32 must fail");
}

#[test]
fn test_nonzero_i64_zero_is_error() {
    let zero_bytes = oxicode::encode_to_vec(&0i64).expect("encode");
    let result: Result<(NonZeroI64, usize), _> = oxicode::decode_from_slice(&zero_bytes);
    assert!(result.is_err(), "decoding zero as NonZeroI64 must fail");
}

#[test]
fn test_nonzero_i128_zero_is_error() {
    let zero_bytes = oxicode::encode_to_vec(&0i128).expect("encode");
    let result: Result<(NonZeroI128, usize), _> = oxicode::decode_from_slice(&zero_bytes);
    assert!(result.is_err(), "decoding zero as NonZeroI128 must fail");
}

#[test]
fn test_nonzero_isize_zero_is_error() {
    let zero_bytes = oxicode::encode_to_vec(&0isize).expect("encode");
    let result: Result<(NonZeroIsize, usize), _> = oxicode::decode_from_slice(&zero_bytes);
    assert!(result.is_err(), "decoding zero as NonZeroIsize must fail");
}

// ===== Error variant check =====

#[test]
fn test_nonzero_u32_zero_error_variant() {
    use oxicode::error::{Error, IntegerType};
    let zero_bytes = oxicode::encode_to_vec(&0u32).expect("encode");
    let err =
        oxicode::decode_from_slice::<NonZeroU32>(&zero_bytes).expect_err("should be an error");
    assert_eq!(
        err,
        Error::NonZeroTypeIsZero {
            non_zero_type: IntegerType::U32,
        }
    );
}

#[test]
fn test_nonzero_i64_zero_error_variant() {
    use oxicode::error::{Error, IntegerType};
    let zero_bytes = oxicode::encode_to_vec(&0i64).expect("encode");
    let err =
        oxicode::decode_from_slice::<NonZeroI64>(&zero_bytes).expect_err("should be an error");
    assert_eq!(
        err,
        Error::NonZeroTypeIsZero {
            non_zero_type: IntegerType::I64,
        }
    );
}
