//! Advanced integer type encoding tests for OxiCode.
//!
//! Covers varint encoding boundaries, zigzag signed encoding, fixed-int config,
//! and big-endian byte verification across all integer primitive types.

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

// ---------------------------------------------------------------------------
// Test 1: u8 value 0 roundtrip — single-byte varint (value fits in 1 byte)
// ---------------------------------------------------------------------------
#[test]
fn test_u8_zero_roundtrip() {
    let val: u8 = 0;
    let bytes = encode_to_vec(&val).expect("encode u8 0");
    assert_eq!(bytes.len(), 1, "u8(0) must encode to exactly 1 byte");
    let (decoded, consumed): (u8, usize) = decode_from_slice(&bytes).expect("decode u8 0");
    assert_eq!(decoded, val, "decoded value must equal original");
    assert_eq!(consumed, 1, "must consume 1 byte");
}

// ---------------------------------------------------------------------------
// Test 2: u8 value 250 roundtrip — last value that fits in a single-byte varint
// ---------------------------------------------------------------------------
#[test]
fn test_u8_250_roundtrip() {
    let val: u8 = 250;
    let bytes = encode_to_vec(&val).expect("encode u8 250");
    assert_eq!(bytes.len(), 1, "u8(250) must encode to exactly 1 byte");
    let (decoded, _): (u8, usize) = decode_from_slice(&bytes).expect("decode u8 250");
    assert_eq!(decoded, val);
}

// ---------------------------------------------------------------------------
// Test 3: u8 value 255 roundtrip — u8 is always 1 raw byte regardless of varint rules
// ---------------------------------------------------------------------------
#[test]
fn test_u8_255_roundtrip() {
    let val: u8 = 255;
    let bytes = encode_to_vec(&val).expect("encode u8 255");
    assert_eq!(bytes.len(), 1, "u8(255) must encode to exactly 1 byte");
    let (decoded, _): (u8, usize) = decode_from_slice(&bytes).expect("decode u8 255");
    assert_eq!(decoded, val);
}

// ---------------------------------------------------------------------------
// Test 4: u16 value 251 roundtrip — first value requiring 3-byte varint encoding
// ---------------------------------------------------------------------------
#[test]
fn test_u16_251_roundtrip() {
    let val: u16 = 251;
    let bytes = encode_to_vec(&val).expect("encode u16 251");
    // u16(251) falls in the 3-byte varint range: [0xFB, lo, hi]
    assert_eq!(bytes.len(), 3, "u16(251) must encode to exactly 3 bytes");
    let (decoded, _): (u16, usize) = decode_from_slice(&bytes).expect("decode u16 251");
    assert_eq!(decoded, val);
}

// ---------------------------------------------------------------------------
// Test 5: u16 value 65535 roundtrip — maximum u16 value
// ---------------------------------------------------------------------------
#[test]
fn test_u16_max_roundtrip() {
    let val: u16 = 65535;
    let bytes = encode_to_vec(&val).expect("encode u16::MAX");
    let (decoded, _): (u16, usize) = decode_from_slice(&bytes).expect("decode u16::MAX");
    assert_eq!(decoded, val);
}

// ---------------------------------------------------------------------------
// Test 6: u32 value 0 roundtrip
// ---------------------------------------------------------------------------
#[test]
fn test_u32_zero_roundtrip() {
    let val: u32 = 0;
    let bytes = encode_to_vec(&val).expect("encode u32 0");
    let (decoded, _): (u32, usize) = decode_from_slice(&bytes).expect("decode u32 0");
    assert_eq!(decoded, val);
}

// ---------------------------------------------------------------------------
// Test 7: u32 value u32::MAX roundtrip
// ---------------------------------------------------------------------------
#[test]
fn test_u32_max_roundtrip() {
    let val: u32 = u32::MAX;
    let bytes = encode_to_vec(&val).expect("encode u32::MAX");
    let (decoded, _): (u32, usize) = decode_from_slice(&bytes).expect("decode u32::MAX");
    assert_eq!(decoded, val);
}

// ---------------------------------------------------------------------------
// Test 8: u64 value u64::MAX roundtrip
// ---------------------------------------------------------------------------
#[test]
fn test_u64_max_roundtrip() {
    let val: u64 = u64::MAX;
    let bytes = encode_to_vec(&val).expect("encode u64::MAX");
    let (decoded, _): (u64, usize) = decode_from_slice(&bytes).expect("decode u64::MAX");
    assert_eq!(decoded, val);
}

// ---------------------------------------------------------------------------
// Test 9: u128 value u128::MAX roundtrip
// ---------------------------------------------------------------------------
#[test]
fn test_u128_max_roundtrip() {
    let val: u128 = u128::MAX;
    let bytes = encode_to_vec(&val).expect("encode u128::MAX");
    let (decoded, _): (u128, usize) = decode_from_slice(&bytes).expect("decode u128::MAX");
    assert_eq!(decoded, val);
}

// ---------------------------------------------------------------------------
// Test 10: i8 value -1 roundtrip — zigzag encodes -1 as 1
// ---------------------------------------------------------------------------
#[test]
fn test_i8_neg1_roundtrip() {
    let val: i8 = -1;
    let bytes = encode_to_vec(&val).expect("encode i8 -1");
    // zigzag(-1) = 1, which is a single-byte varint
    assert_eq!(bytes.len(), 1, "i8(-1) zigzag=1 must encode to 1 byte");
    let (decoded, _): (i8, usize) = decode_from_slice(&bytes).expect("decode i8 -1");
    assert_eq!(decoded, val);
}

// ---------------------------------------------------------------------------
// Test 11: i8 value -128 roundtrip — minimum i8 value
// ---------------------------------------------------------------------------
#[test]
fn test_i8_min_roundtrip() {
    let val: i8 = -128;
    let bytes = encode_to_vec(&val).expect("encode i8::MIN");
    let (decoded, _): (i8, usize) = decode_from_slice(&bytes).expect("decode i8::MIN");
    assert_eq!(decoded, val);
}

// ---------------------------------------------------------------------------
// Test 12: i16 value i16::MIN roundtrip
// ---------------------------------------------------------------------------
#[test]
fn test_i16_min_roundtrip() {
    let val: i16 = i16::MIN;
    let bytes = encode_to_vec(&val).expect("encode i16::MIN");
    let (decoded, _): (i16, usize) = decode_from_slice(&bytes).expect("decode i16::MIN");
    assert_eq!(decoded, val);
}

// ---------------------------------------------------------------------------
// Test 13: i32 value i32::MIN roundtrip
// ---------------------------------------------------------------------------
#[test]
fn test_i32_min_roundtrip() {
    let val: i32 = i32::MIN;
    let bytes = encode_to_vec(&val).expect("encode i32::MIN");
    let (decoded, _): (i32, usize) = decode_from_slice(&bytes).expect("decode i32::MIN");
    assert_eq!(decoded, val);
}

// ---------------------------------------------------------------------------
// Test 14: i32 value i32::MAX roundtrip
// ---------------------------------------------------------------------------
#[test]
fn test_i32_max_roundtrip() {
    let val: i32 = i32::MAX;
    let bytes = encode_to_vec(&val).expect("encode i32::MAX");
    let (decoded, _): (i32, usize) = decode_from_slice(&bytes).expect("decode i32::MAX");
    assert_eq!(decoded, val);
}

// ---------------------------------------------------------------------------
// Test 15: i64 value i64::MIN roundtrip
// ---------------------------------------------------------------------------
#[test]
fn test_i64_min_roundtrip() {
    let val: i64 = i64::MIN;
    let bytes = encode_to_vec(&val).expect("encode i64::MIN");
    let (decoded, _): (i64, usize) = decode_from_slice(&bytes).expect("decode i64::MIN");
    assert_eq!(decoded, val);
}

// ---------------------------------------------------------------------------
// Test 16: i64 value i64::MAX roundtrip
// ---------------------------------------------------------------------------
#[test]
fn test_i64_max_roundtrip() {
    let val: i64 = i64::MAX;
    let bytes = encode_to_vec(&val).expect("encode i64::MAX");
    let (decoded, _): (i64, usize) = decode_from_slice(&bytes).expect("decode i64::MAX");
    assert_eq!(decoded, val);
}

// ---------------------------------------------------------------------------
// Test 17: i128 value i128::MIN roundtrip
// ---------------------------------------------------------------------------
#[test]
fn test_i128_min_roundtrip() {
    let val: i128 = i128::MIN;
    let bytes = encode_to_vec(&val).expect("encode i128::MIN");
    let (decoded, _): (i128, usize) = decode_from_slice(&bytes).expect("decode i128::MIN");
    assert_eq!(decoded, val);
}

// ---------------------------------------------------------------------------
// Test 18: usize value 0 roundtrip
// ---------------------------------------------------------------------------
#[test]
fn test_usize_zero_roundtrip() {
    let val: usize = 0;
    let bytes = encode_to_vec(&val).expect("encode usize 0");
    let (decoded, _): (usize, usize) = decode_from_slice(&bytes).expect("decode usize 0");
    assert_eq!(decoded, val);
}

// ---------------------------------------------------------------------------
// Test 19: isize value -1 roundtrip — zigzag encodes -1 as 1
// ---------------------------------------------------------------------------
#[test]
fn test_isize_neg1_roundtrip() {
    let val: isize = -1;
    let bytes = encode_to_vec(&val).expect("encode isize -1");
    let (decoded, _): (isize, usize) = decode_from_slice(&bytes).expect("decode isize -1");
    assert_eq!(decoded, val);
}

// ---------------------------------------------------------------------------
// Test 20: u32 value 42 with fixed-int config — must be exactly 4 bytes
// ---------------------------------------------------------------------------
#[test]
fn test_u32_42_fixed_int_config_4_bytes() {
    let val: u32 = 42;
    let cfg = config::standard().with_fixed_int_encoding();
    let bytes = encode_to_vec_with_config(&val, cfg).expect("encode u32 42 fixed-int");
    assert_eq!(
        bytes.len(),
        4,
        "u32 with fixed-int encoding must be exactly 4 bytes"
    );
    let (decoded, consumed): (u32, usize) =
        decode_from_slice_with_config(&bytes, cfg).expect("decode u32 42 fixed-int");
    assert_eq!(decoded, val);
    assert_eq!(consumed, 4);
}

// ---------------------------------------------------------------------------
// Test 21: u64 value 42 with fixed-int config — must be exactly 8 bytes
// ---------------------------------------------------------------------------
#[test]
fn test_u64_42_fixed_int_config_8_bytes() {
    let val: u64 = 42;
    let cfg = config::standard().with_fixed_int_encoding();
    let bytes = encode_to_vec_with_config(&val, cfg).expect("encode u64 42 fixed-int");
    assert_eq!(
        bytes.len(),
        8,
        "u64 with fixed-int encoding must be exactly 8 bytes"
    );
    let (decoded, consumed): (u64, usize) =
        decode_from_slice_with_config(&bytes, cfg).expect("decode u64 42 fixed-int");
    assert_eq!(decoded, val);
    assert_eq!(consumed, 8);
}

// ---------------------------------------------------------------------------
// Test 22: i32 value -1 with big-endian + fixed-int config — bytes are 0xFF x4
// ---------------------------------------------------------------------------
#[test]
fn test_i32_neg1_big_endian_fixed_int_bytes() {
    let val: i32 = -1;
    let cfg = config::standard()
        .with_big_endian()
        .with_fixed_int_encoding();
    let bytes = encode_to_vec_with_config(&val, cfg).expect("encode i32 -1 big-endian fixed-int");
    assert_eq!(
        bytes.len(),
        4,
        "i32 with fixed-int encoding must be exactly 4 bytes"
    );
    assert_eq!(
        bytes,
        &[0xFF, 0xFF, 0xFF, 0xFF],
        "i32(-1) big-endian fixed-int must be 0xFFFFFFFF"
    );
    let (decoded, consumed): (i32, usize) =
        decode_from_slice_with_config(&bytes, cfg).expect("decode i32 -1 big-endian fixed-int");
    assert_eq!(decoded, val);
    assert_eq!(consumed, 4);
}
