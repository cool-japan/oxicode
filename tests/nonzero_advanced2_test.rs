//! Advanced tests for NonZero integer type serialization in OxiCode.

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
use std::num::{
    NonZeroI128, NonZeroI16, NonZeroI32, NonZeroI64, NonZeroI8, NonZeroU128, NonZeroU16,
    NonZeroU32, NonZeroU64, NonZeroU8, NonZeroUsize,
};

// Test 1: NonZeroU32 value 1 roundtrip (minimum)
#[test]
fn test_nonzero_u32_value_one_roundtrip() {
    let val = NonZeroU32::new(1).expect("nonzero");
    let bytes = encode_to_vec(&val).expect("encode NonZeroU32(1)");
    let (decoded, _): (NonZeroU32, usize) =
        decode_from_slice(&bytes).expect("decode NonZeroU32(1)");
    assert_eq!(val, decoded);
}

// Test 2: NonZeroU32 value 42 roundtrip
#[test]
fn test_nonzero_u32_value_42_roundtrip() {
    let val = NonZeroU32::new(42).expect("nonzero");
    let bytes = encode_to_vec(&val).expect("encode NonZeroU32(42)");
    let (decoded, _): (NonZeroU32, usize) =
        decode_from_slice(&bytes).expect("decode NonZeroU32(42)");
    assert_eq!(val, decoded);
}

// Test 3: NonZeroU32 value u32::MAX roundtrip
#[test]
fn test_nonzero_u32_max_roundtrip() {
    let val = NonZeroU32::new(u32::MAX).expect("nonzero");
    let bytes = encode_to_vec(&val).expect("encode NonZeroU32(u32::MAX)");
    let (decoded, _): (NonZeroU32, usize) =
        decode_from_slice(&bytes).expect("decode NonZeroU32(u32::MAX)");
    assert_eq!(val, decoded);
}

// Test 4: NonZeroU64 large value roundtrip
#[test]
fn test_nonzero_u64_large_value_roundtrip() {
    let val = NonZeroU64::new(u64::MAX - 1).expect("nonzero");
    let bytes = encode_to_vec(&val).expect("encode NonZeroU64(u64::MAX - 1)");
    let (decoded, _): (NonZeroU64, usize) =
        decode_from_slice(&bytes).expect("decode NonZeroU64(u64::MAX - 1)");
    assert_eq!(val, decoded);
}

// Test 5: NonZeroU8 value 255 roundtrip
#[test]
fn test_nonzero_u8_value_255_roundtrip() {
    let val = NonZeroU8::new(255).expect("nonzero");
    let bytes = encode_to_vec(&val).expect("encode NonZeroU8(255)");
    let (decoded, _): (NonZeroU8, usize) =
        decode_from_slice(&bytes).expect("decode NonZeroU8(255)");
    assert_eq!(val, decoded);
}

// Test 6: NonZeroU16 value 65535 roundtrip
#[test]
fn test_nonzero_u16_value_65535_roundtrip() {
    let val = NonZeroU16::new(65535).expect("nonzero");
    let bytes = encode_to_vec(&val).expect("encode NonZeroU16(65535)");
    let (decoded, _): (NonZeroU16, usize) =
        decode_from_slice(&bytes).expect("decode NonZeroU16(65535)");
    assert_eq!(val, decoded);
}

// Test 7: NonZeroU128 large value roundtrip
#[test]
fn test_nonzero_u128_large_value_roundtrip() {
    let val = NonZeroU128::new(u128::MAX / 2 + 1).expect("nonzero");
    let bytes = encode_to_vec(&val).expect("encode NonZeroU128(large)");
    let (decoded, _): (NonZeroU128, usize) =
        decode_from_slice(&bytes).expect("decode NonZeroU128(large)");
    assert_eq!(val, decoded);
}

// Test 8: NonZeroUsize value roundtrip
#[test]
fn test_nonzero_usize_value_roundtrip() {
    let val = NonZeroUsize::new(12345678).expect("nonzero");
    let bytes = encode_to_vec(&val).expect("encode NonZeroUsize(12345678)");
    let (decoded, _): (NonZeroUsize, usize) =
        decode_from_slice(&bytes).expect("decode NonZeroUsize(12345678)");
    assert_eq!(val, decoded);
}

// Test 9: NonZeroI32 positive roundtrip
#[test]
fn test_nonzero_i32_positive_roundtrip() {
    let val = NonZeroI32::new(99999).expect("nonzero");
    let bytes = encode_to_vec(&val).expect("encode NonZeroI32(99999)");
    let (decoded, _): (NonZeroI32, usize) =
        decode_from_slice(&bytes).expect("decode NonZeroI32(99999)");
    assert_eq!(val, decoded);
}

// Test 10: NonZeroI32 negative roundtrip (e.g. -1)
#[test]
fn test_nonzero_i32_negative_roundtrip() {
    let val = NonZeroI32::new(-1).expect("nonzero");
    let bytes = encode_to_vec(&val).expect("encode NonZeroI32(-1)");
    let (decoded, _): (NonZeroI32, usize) =
        decode_from_slice(&bytes).expect("decode NonZeroI32(-1)");
    assert_eq!(val, decoded);
}

// Test 11: NonZeroI64 i64::MIN + 1 roundtrip
#[test]
fn test_nonzero_i64_min_plus_one_roundtrip() {
    let val = NonZeroI64::new(i64::MIN + 1).expect("nonzero");
    let bytes = encode_to_vec(&val).expect("encode NonZeroI64(i64::MIN + 1)");
    let (decoded, _): (NonZeroI64, usize) =
        decode_from_slice(&bytes).expect("decode NonZeroI64(i64::MIN + 1)");
    assert_eq!(val, decoded);
}

// Test 12: NonZeroI8 roundtrip
#[test]
fn test_nonzero_i8_roundtrip() {
    let val = NonZeroI8::new(-42).expect("nonzero");
    let bytes = encode_to_vec(&val).expect("encode NonZeroI8(-42)");
    let (decoded, _): (NonZeroI8, usize) =
        decode_from_slice(&bytes).expect("decode NonZeroI8(-42)");
    assert_eq!(val, decoded);
}

// Test 13: NonZeroI16 roundtrip
#[test]
fn test_nonzero_i16_roundtrip() {
    let val = NonZeroI16::new(-1000).expect("nonzero");
    let bytes = encode_to_vec(&val).expect("encode NonZeroI16(-1000)");
    let (decoded, _): (NonZeroI16, usize) =
        decode_from_slice(&bytes).expect("decode NonZeroI16(-1000)");
    assert_eq!(val, decoded);
}

// Test 14: NonZeroI128 roundtrip
#[test]
fn test_nonzero_i128_roundtrip() {
    let val = NonZeroI128::new(i128::MAX - 99).expect("nonzero");
    let bytes = encode_to_vec(&val).expect("encode NonZeroI128(i128::MAX - 99)");
    let (decoded, _): (NonZeroI128, usize) =
        decode_from_slice(&bytes).expect("decode NonZeroI128(i128::MAX - 99)");
    assert_eq!(val, decoded);
}

// Test 15: Vec<NonZeroU32> roundtrip (5 items)
#[test]
fn test_vec_nonzero_u32_five_items_roundtrip() {
    let val: Vec<NonZeroU32> = vec![
        NonZeroU32::new(1).expect("nonzero"),
        NonZeroU32::new(10).expect("nonzero"),
        NonZeroU32::new(100).expect("nonzero"),
        NonZeroU32::new(1000).expect("nonzero"),
        NonZeroU32::new(10000).expect("nonzero"),
    ];
    let bytes = encode_to_vec(&val).expect("encode Vec<NonZeroU32> 5 items");
    let (decoded, _): (Vec<NonZeroU32>, usize) =
        decode_from_slice(&bytes).expect("decode Vec<NonZeroU32> 5 items");
    assert_eq!(val, decoded);
}

// Test 16: Option<NonZeroU32> Some roundtrip
#[test]
fn test_option_nonzero_u32_some_roundtrip() {
    let val: Option<NonZeroU32> = Some(NonZeroU32::new(42).expect("nonzero"));
    let bytes = encode_to_vec(&val).expect("encode Option<NonZeroU32>::Some");
    let (decoded, _): (Option<NonZeroU32>, usize) =
        decode_from_slice(&bytes).expect("decode Option<NonZeroU32>::Some");
    assert_eq!(val, decoded);
}

// Test 17: Option<NonZeroU32> None roundtrip
#[test]
fn test_option_nonzero_u32_none_roundtrip() {
    let val: Option<NonZeroU32> = None;
    let bytes = encode_to_vec(&val).expect("encode Option<NonZeroU32>::None");
    let (decoded, _): (Option<NonZeroU32>, usize) =
        decode_from_slice(&bytes).expect("decode Option<NonZeroU32>::None");
    assert_eq!(val, decoded);
}

// Test 18: NonZeroU32 with fixed-int config
#[test]
fn test_nonzero_u32_with_fixed_int_config() {
    let val = NonZeroU32::new(42).expect("nonzero");
    let cfg = config::standard().with_fixed_int_encoding();
    let bytes = encode_to_vec_with_config(&val, cfg).expect("encode NonZeroU32 fixed-int");
    let (decoded, _): (NonZeroU32, usize) =
        decode_from_slice_with_config(&bytes, cfg).expect("decode NonZeroU32 fixed-int");
    assert_eq!(val, decoded);
}

// Test 19: NonZeroU64 consumed bytes equals encoded length
#[test]
fn test_nonzero_u64_consumed_bytes_equals_encoded_length() {
    let val = NonZeroU64::new(987654321).expect("nonzero");
    let bytes = encode_to_vec(&val).expect("encode NonZeroU64 for length check");
    let (_decoded, consumed): (NonZeroU64, usize) =
        decode_from_slice(&bytes).expect("decode NonZeroU64 for length check");
    assert_eq!(
        consumed,
        bytes.len(),
        "consumed bytes must equal the full encoded length"
    );
}

// Test 20: Two NonZeroU32 with same value produce same bytes
#[test]
fn test_nonzero_u32_same_value_same_bytes() {
    let val_a = NonZeroU32::new(777).expect("nonzero");
    let val_b = NonZeroU32::new(777).expect("nonzero");
    let bytes_a = encode_to_vec(&val_a).expect("encode val_a");
    let bytes_b = encode_to_vec(&val_b).expect("encode val_b");
    assert_eq!(
        bytes_a, bytes_b,
        "same NonZeroU32 value must produce identical encodings"
    );
}

// Test 21: Different NonZeroU32 values produce different bytes
#[test]
fn test_nonzero_u32_different_values_different_bytes() {
    let val_a = NonZeroU32::new(1).expect("nonzero");
    let val_b = NonZeroU32::new(2).expect("nonzero");
    let bytes_a = encode_to_vec(&val_a).expect("encode val_a");
    let bytes_b = encode_to_vec(&val_b).expect("encode val_b");
    assert_ne!(
        bytes_a, bytes_b,
        "different NonZeroU32 values must produce different encodings"
    );
}

// Test 22: NonZeroI32 with big-endian fixed-int config
#[test]
fn test_nonzero_i32_with_big_endian_fixed_int_config() {
    let val = NonZeroI32::new(-12345).expect("nonzero");
    let cfg = config::standard()
        .with_fixed_int_encoding()
        .with_big_endian();
    let bytes =
        encode_to_vec_with_config(&val, cfg).expect("encode NonZeroI32 big-endian fixed-int");
    let (decoded, _): (NonZeroI32, usize) =
        decode_from_slice_with_config(&bytes, cfg).expect("decode NonZeroI32 big-endian fixed-int");
    assert_eq!(val, decoded);
}
