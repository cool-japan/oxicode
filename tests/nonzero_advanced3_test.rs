//! Advanced roundtrip tests for NonZero integer types (set 3).

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
use oxicode::{
    config, decode_from_slice, decode_from_slice_with_config, encode_to_vec,
    encode_to_vec_with_config,
};
use std::num::{NonZeroI128, NonZeroI16, NonZeroI32, NonZeroI64, NonZeroI8, NonZeroIsize};
use std::num::{NonZeroU128, NonZeroU16, NonZeroU32, NonZeroU64, NonZeroU8, NonZeroUsize};

// Test 1: NonZeroU8 value=1 roundtrip
#[test]
fn test_nonzero_advanced3_u8_value_one_roundtrip() {
    let val = NonZeroU8::new(1).expect("NonZeroU8::new(1) must succeed");
    let bytes = encode_to_vec(&val).expect("encode NonZeroU8(1)");
    let (decoded, _): (NonZeroU8, usize) = decode_from_slice(&bytes).expect("decode NonZeroU8(1)");
    assert_eq!(val, decoded);
}

// Test 2: NonZeroU8 value=255 (max) roundtrip
#[test]
fn test_nonzero_advanced3_u8_value_255_roundtrip() {
    let val = NonZeroU8::new(255).expect("NonZeroU8::new(255) must succeed");
    let bytes = encode_to_vec(&val).expect("encode NonZeroU8(255)");
    let (decoded, _): (NonZeroU8, usize) =
        decode_from_slice(&bytes).expect("decode NonZeroU8(255)");
    assert_eq!(val, decoded);
}

// Test 3: NonZeroU16 value=1 roundtrip
#[test]
fn test_nonzero_advanced3_u16_value_one_roundtrip() {
    let val = NonZeroU16::new(1).expect("NonZeroU16::new(1) must succeed");
    let bytes = encode_to_vec(&val).expect("encode NonZeroU16(1)");
    let (decoded, _): (NonZeroU16, usize) =
        decode_from_slice(&bytes).expect("decode NonZeroU16(1)");
    assert_eq!(val, decoded);
}

// Test 4: NonZeroU16 max roundtrip
#[test]
fn test_nonzero_advanced3_u16_max_roundtrip() {
    let val = NonZeroU16::new(u16::MAX).expect("NonZeroU16::new(u16::MAX) must succeed");
    let bytes = encode_to_vec(&val).expect("encode NonZeroU16(u16::MAX)");
    let (decoded, _): (NonZeroU16, usize) =
        decode_from_slice(&bytes).expect("decode NonZeroU16(u16::MAX)");
    assert_eq!(val, decoded);
}

// Test 5: NonZeroU32 value=42 roundtrip
#[test]
fn test_nonzero_advanced3_u32_value_42_roundtrip() {
    let val = NonZeroU32::new(42).expect("NonZeroU32::new(42) must succeed");
    let bytes = encode_to_vec(&val).expect("encode NonZeroU32(42)");
    let (decoded, _): (NonZeroU32, usize) =
        decode_from_slice(&bytes).expect("decode NonZeroU32(42)");
    assert_eq!(val, decoded);
}

// Test 6: NonZeroU32 max roundtrip
#[test]
fn test_nonzero_advanced3_u32_max_roundtrip() {
    let val = NonZeroU32::new(u32::MAX).expect("NonZeroU32::new(u32::MAX) must succeed");
    let bytes = encode_to_vec(&val).expect("encode NonZeroU32(u32::MAX)");
    let (decoded, _): (NonZeroU32, usize) =
        decode_from_slice(&bytes).expect("decode NonZeroU32(u32::MAX)");
    assert_eq!(val, decoded);
}

// Test 7: NonZeroU64 roundtrip
#[test]
fn test_nonzero_advanced3_u64_roundtrip() {
    let val = NonZeroU64::new(u64::MAX).expect("NonZeroU64::new(u64::MAX) must succeed");
    let bytes = encode_to_vec(&val).expect("encode NonZeroU64(u64::MAX)");
    let (decoded, _): (NonZeroU64, usize) =
        decode_from_slice(&bytes).expect("decode NonZeroU64(u64::MAX)");
    assert_eq!(val, decoded);
}

// Test 8: NonZeroU128 roundtrip
#[test]
fn test_nonzero_advanced3_u128_roundtrip() {
    let val = NonZeroU128::new(u128::MAX).expect("NonZeroU128::new(u128::MAX) must succeed");
    let bytes = encode_to_vec(&val).expect("encode NonZeroU128(u128::MAX)");
    let (decoded, _): (NonZeroU128, usize) =
        decode_from_slice(&bytes).expect("decode NonZeroU128(u128::MAX)");
    assert_eq!(val, decoded);
}

// Test 9: NonZeroUsize roundtrip
#[test]
fn test_nonzero_advanced3_usize_roundtrip() {
    let val = NonZeroUsize::new(usize::MAX).expect("NonZeroUsize::new(usize::MAX) must succeed");
    let bytes = encode_to_vec(&val).expect("encode NonZeroUsize(usize::MAX)");
    let (decoded, _): (NonZeroUsize, usize) =
        decode_from_slice(&bytes).expect("decode NonZeroUsize(usize::MAX)");
    assert_eq!(val, decoded);
}

// Test 10: NonZeroI8 value=1 roundtrip
#[test]
fn test_nonzero_advanced3_i8_value_one_roundtrip() {
    let val = NonZeroI8::new(1).expect("NonZeroI8::new(1) must succeed");
    let bytes = encode_to_vec(&val).expect("encode NonZeroI8(1)");
    let (decoded, _): (NonZeroI8, usize) = decode_from_slice(&bytes).expect("decode NonZeroI8(1)");
    assert_eq!(val, decoded);
}

// Test 11: NonZeroI8 value=-1 roundtrip
#[test]
fn test_nonzero_advanced3_i8_value_neg_one_roundtrip() {
    let val = NonZeroI8::new(-1).expect("NonZeroI8::new(-1) must succeed");
    let bytes = encode_to_vec(&val).expect("encode NonZeroI8(-1)");
    let (decoded, _): (NonZeroI8, usize) = decode_from_slice(&bytes).expect("decode NonZeroI8(-1)");
    assert_eq!(val, decoded);
}

// Test 12: NonZeroI16 positive roundtrip
#[test]
fn test_nonzero_advanced3_i16_positive_roundtrip() {
    let val = NonZeroI16::new(i16::MAX).expect("NonZeroI16::new(i16::MAX) must succeed");
    let bytes = encode_to_vec(&val).expect("encode NonZeroI16(i16::MAX)");
    let (decoded, _): (NonZeroI16, usize) =
        decode_from_slice(&bytes).expect("decode NonZeroI16(i16::MAX)");
    assert_eq!(val, decoded);
}

// Test 13: NonZeroI32 negative roundtrip
#[test]
fn test_nonzero_advanced3_i32_negative_roundtrip() {
    let val = NonZeroI32::new(-999999).expect("NonZeroI32::new(-999999) must succeed");
    let bytes = encode_to_vec(&val).expect("encode NonZeroI32(-999999)");
    let (decoded, _): (NonZeroI32, usize) =
        decode_from_slice(&bytes).expect("decode NonZeroI32(-999999)");
    assert_eq!(val, decoded);
}

// Test 14: NonZeroI64 max roundtrip
#[test]
fn test_nonzero_advanced3_i64_max_roundtrip() {
    let val = NonZeroI64::new(i64::MAX).expect("NonZeroI64::new(i64::MAX) must succeed");
    let bytes = encode_to_vec(&val).expect("encode NonZeroI64(i64::MAX)");
    let (decoded, _): (NonZeroI64, usize) =
        decode_from_slice(&bytes).expect("decode NonZeroI64(i64::MAX)");
    assert_eq!(val, decoded);
}

// Test 15: NonZeroI128 min roundtrip
#[test]
fn test_nonzero_advanced3_i128_min_roundtrip() {
    // i128::MIN is a valid NonZeroI128 since it is not zero
    let val = NonZeroI128::new(i128::MIN).expect("NonZeroI128::new(i128::MIN) must succeed");
    let bytes = encode_to_vec(&val).expect("encode NonZeroI128(i128::MIN)");
    let (decoded, _): (NonZeroI128, usize) =
        decode_from_slice(&bytes).expect("decode NonZeroI128(i128::MIN)");
    assert_eq!(val, decoded);
}

// Test 16: NonZeroIsize roundtrip
#[test]
fn test_nonzero_advanced3_isize_roundtrip() {
    let val = NonZeroIsize::new(-1).expect("NonZeroIsize::new(-1) must succeed");
    let bytes = encode_to_vec(&val).expect("encode NonZeroIsize(-1)");
    let (decoded, _): (NonZeroIsize, usize) =
        decode_from_slice(&bytes).expect("decode NonZeroIsize(-1)");
    assert_eq!(val, decoded);
}

// Test 17: NonZeroU32 same wire bytes as raw u32 with same value
#[test]
fn test_nonzero_advanced3_u32_same_wire_bytes_as_raw_u32() {
    let raw_val: u32 = 12345;
    let nonzero_val = NonZeroU32::new(raw_val).expect("NonZeroU32::new(12345) must succeed");
    let raw_bytes = encode_to_vec(&raw_val).expect("encode raw u32(12345)");
    let nonzero_bytes = encode_to_vec(&nonzero_val).expect("encode NonZeroU32(12345)");
    assert_eq!(
        raw_bytes, nonzero_bytes,
        "NonZeroU32 and u32 with the same value must produce identical wire bytes"
    );
}

// Test 18: Vec<NonZeroU32> roundtrip (5 items)
#[test]
fn test_nonzero_advanced3_vec_nonzero_u32_five_items_roundtrip() {
    let val: Vec<NonZeroU32> = vec![
        NonZeroU32::new(1).expect("NonZeroU32::new(1)"),
        NonZeroU32::new(2).expect("NonZeroU32::new(2)"),
        NonZeroU32::new(100).expect("NonZeroU32::new(100)"),
        NonZeroU32::new(65535).expect("NonZeroU32::new(65535)"),
        NonZeroU32::new(u32::MAX).expect("NonZeroU32::new(u32::MAX)"),
    ];
    let bytes = encode_to_vec(&val).expect("encode Vec<NonZeroU32> 5 items");
    let (decoded, _): (Vec<NonZeroU32>, usize) =
        decode_from_slice(&bytes).expect("decode Vec<NonZeroU32> 5 items");
    assert_eq!(val, decoded);
}

// Test 19: Option<NonZeroU32> Some roundtrip
#[test]
fn test_nonzero_advanced3_option_nonzero_u32_some_roundtrip() {
    let val: Option<NonZeroU32> = Some(NonZeroU32::new(777).expect("NonZeroU32::new(777)"));
    let bytes = encode_to_vec(&val).expect("encode Option<NonZeroU32>::Some(777)");
    let (decoded, _): (Option<NonZeroU32>, usize) =
        decode_from_slice(&bytes).expect("decode Option<NonZeroU32>::Some(777)");
    assert_eq!(val, decoded);
}

// Test 20: Option<NonZeroU32> None roundtrip
#[test]
fn test_nonzero_advanced3_option_nonzero_u32_none_roundtrip() {
    let val: Option<NonZeroU32> = None;
    let bytes = encode_to_vec(&val).expect("encode Option<NonZeroU32>::None");
    let (decoded, _): (Option<NonZeroU32>, usize) =
        decode_from_slice(&bytes).expect("decode Option<NonZeroU32>::None");
    assert_eq!(val, decoded);
}

// Test 21: NonZeroU32 with fixed-int config
#[test]
fn test_nonzero_advanced3_u32_with_fixed_int_config() {
    let val = NonZeroU32::new(42).expect("NonZeroU32::new(42) must succeed");
    let cfg = config::standard().with_fixed_int_encoding();
    let bytes =
        encode_to_vec_with_config(&val, cfg).expect("encode NonZeroU32(42) with fixed-int config");
    let (decoded, _): (NonZeroU32, usize) = decode_from_slice_with_config(&bytes, cfg)
        .expect("decode NonZeroU32(42) with fixed-int config");
    assert_eq!(val, decoded);
}

// Test 22: NonZeroU64 consumed bytes equals encoded length
#[test]
fn test_nonzero_advanced3_u64_consumed_bytes_equals_encoded_length() {
    let val = NonZeroU64::new(987654321).expect("NonZeroU64::new(987654321) must succeed");
    let bytes = encode_to_vec(&val).expect("encode NonZeroU64(987654321)");
    let (_decoded, consumed): (NonZeroU64, usize) =
        decode_from_slice(&bytes).expect("decode NonZeroU64(987654321) for length check");
    assert_eq!(
        consumed,
        bytes.len(),
        "consumed bytes must equal the full encoded length"
    );
}
