//! Advanced tests for NonZero type encoding in OxiCode.

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
use std::num::{
    NonZeroI16, NonZeroI32, NonZeroI64, NonZeroI8, NonZeroIsize, NonZeroU16, NonZeroU32,
    NonZeroU64, NonZeroU8, NonZeroUsize,
};

// Also import 128-bit variants used below.
use std::num::{NonZeroI128, NonZeroU128};

// ---------------------------------------------------------------------------
// 1. NonZeroU8 boundary: minimum value (1)
// ---------------------------------------------------------------------------
#[test]
fn test_nonzero_u8_min() {
    let v = NonZeroU8::new(1).expect("nonzero");
    let bytes = encode_to_vec(&v).expect("encode NonZeroU8(1)");
    let (decoded, consumed): (NonZeroU8, usize) =
        decode_from_slice(&bytes).expect("decode NonZeroU8(1)");
    assert_eq!(v, decoded);
    assert_eq!(consumed, bytes.len());
}

// ---------------------------------------------------------------------------
// 2. NonZeroU8 midpoint: 127
// ---------------------------------------------------------------------------
#[test]
fn test_nonzero_u8_midpoint() {
    let v = NonZeroU8::new(127).expect("nonzero");
    let bytes = encode_to_vec(&v).expect("encode NonZeroU8(127)");
    let (decoded, _): (NonZeroU8, usize) =
        decode_from_slice(&bytes).expect("decode NonZeroU8(127)");
    assert_eq!(v, decoded);
}

// ---------------------------------------------------------------------------
// 3. NonZeroU8 maximum value (255)
// ---------------------------------------------------------------------------
#[test]
fn test_nonzero_u8_max() {
    let v = NonZeroU8::new(255).expect("nonzero");
    let bytes = encode_to_vec(&v).expect("encode NonZeroU8(255)");
    let (decoded, _): (NonZeroU8, usize) =
        decode_from_slice(&bytes).expect("decode NonZeroU8(255)");
    assert_eq!(v, decoded);
}

// ---------------------------------------------------------------------------
// 4. NonZeroU16 values: 1, 1000, 65535
// ---------------------------------------------------------------------------
#[test]
fn test_nonzero_u16_one() {
    let v = NonZeroU16::new(1).expect("nonzero");
    let bytes = encode_to_vec(&v).expect("encode NonZeroU16(1)");
    let (decoded, _): (NonZeroU16, usize) =
        decode_from_slice(&bytes).expect("decode NonZeroU16(1)");
    assert_eq!(v, decoded);
}

#[test]
fn test_nonzero_u16_boundary() {
    let v = NonZeroU16::new(1000).expect("nonzero");
    let bytes = encode_to_vec(&v).expect("encode NonZeroU16(1000)");
    let (decoded, _): (NonZeroU16, usize) =
        decode_from_slice(&bytes).expect("decode NonZeroU16(1000)");
    assert_eq!(v, decoded);
}

#[test]
fn test_nonzero_u16_max() {
    let v = NonZeroU16::new(65535).expect("nonzero");
    let bytes = encode_to_vec(&v).expect("encode NonZeroU16(65535)");
    let (decoded, _): (NonZeroU16, usize) =
        decode_from_slice(&bytes).expect("decode NonZeroU16(65535)");
    assert_eq!(v, decoded);
}

// ---------------------------------------------------------------------------
// 5. NonZeroU32 values: 1, 65536, u32::MAX
// ---------------------------------------------------------------------------
#[test]
fn test_nonzero_u32_one() {
    let v = NonZeroU32::new(1).expect("nonzero");
    let bytes = encode_to_vec(&v).expect("encode NonZeroU32(1)");
    let (decoded, _): (NonZeroU32, usize) =
        decode_from_slice(&bytes).expect("decode NonZeroU32(1)");
    assert_eq!(v, decoded);
}

#[test]
fn test_nonzero_u32_over_u16_boundary() {
    let v = NonZeroU32::new(65536).expect("nonzero");
    let bytes = encode_to_vec(&v).expect("encode NonZeroU32(65536)");
    let (decoded, _): (NonZeroU32, usize) =
        decode_from_slice(&bytes).expect("decode NonZeroU32(65536)");
    assert_eq!(v, decoded);
}

#[test]
fn test_nonzero_u32_max() {
    let v = NonZeroU32::new(u32::MAX).expect("nonzero");
    let bytes = encode_to_vec(&v).expect("encode NonZeroU32(u32::MAX)");
    let (decoded, _): (NonZeroU32, usize) =
        decode_from_slice(&bytes).expect("decode NonZeroU32(u32::MAX)");
    assert_eq!(v, decoded);
}

// ---------------------------------------------------------------------------
// 6. NonZeroU64 values: 1, u32::MAX as u64 + 1, u64::MAX
// ---------------------------------------------------------------------------
#[test]
fn test_nonzero_u64_one() {
    let v = NonZeroU64::new(1).expect("nonzero");
    let bytes = encode_to_vec(&v).expect("encode NonZeroU64(1)");
    let (decoded, _): (NonZeroU64, usize) =
        decode_from_slice(&bytes).expect("decode NonZeroU64(1)");
    assert_eq!(v, decoded);
}

#[test]
fn test_nonzero_u64_over_u32_boundary() {
    let val = u32::MAX as u64 + 1;
    let v = NonZeroU64::new(val).expect("nonzero");
    let bytes = encode_to_vec(&v).expect("encode NonZeroU64(u32::MAX+1)");
    let (decoded, _): (NonZeroU64, usize) =
        decode_from_slice(&bytes).expect("decode NonZeroU64(u32::MAX+1)");
    assert_eq!(v, decoded);
}

#[test]
fn test_nonzero_u64_max() {
    let v = NonZeroU64::new(u64::MAX).expect("nonzero");
    let bytes = encode_to_vec(&v).expect("encode NonZeroU64(u64::MAX)");
    let (decoded, _): (NonZeroU64, usize) =
        decode_from_slice(&bytes).expect("decode NonZeroU64(u64::MAX)");
    assert_eq!(v, decoded);
}

// ---------------------------------------------------------------------------
// 7. NonZeroI8: i8::MIN (-128) and i8::MAX (127)
//    Note: i8::MIN is -128, which is nonzero.
// ---------------------------------------------------------------------------
#[test]
fn test_nonzero_i8_min() {
    // i8::MIN = -128, which is nonzero (only 0 is invalid for NonZero types)
    let v = NonZeroI8::new(i8::MIN).expect("nonzero i8::MIN");
    let bytes = encode_to_vec(&v).expect("encode NonZeroI8(i8::MIN)");
    let (decoded, _): (NonZeroI8, usize) =
        decode_from_slice(&bytes).expect("decode NonZeroI8(i8::MIN)");
    assert_eq!(v, decoded);
}

#[test]
fn test_nonzero_i8_max() {
    let v = NonZeroI8::new(i8::MAX).expect("nonzero i8::MAX");
    let bytes = encode_to_vec(&v).expect("encode NonZeroI8(i8::MAX)");
    let (decoded, _): (NonZeroI8, usize) =
        decode_from_slice(&bytes).expect("decode NonZeroI8(i8::MAX)");
    assert_eq!(v, decoded);
}

// ---------------------------------------------------------------------------
// 8. NonZeroI16 roundtrip: i16::MIN and i16::MAX
// ---------------------------------------------------------------------------
#[test]
fn test_nonzero_i16_boundary() {
    let neg = NonZeroI16::new(i16::MIN).expect("nonzero i16::MIN");
    let pos = NonZeroI16::new(i16::MAX).expect("nonzero i16::MAX");

    let neg_bytes = encode_to_vec(&neg).expect("encode NonZeroI16(i16::MIN)");
    let (neg_dec, _): (NonZeroI16, usize) =
        decode_from_slice(&neg_bytes).expect("decode NonZeroI16(i16::MIN)");
    assert_eq!(neg, neg_dec);

    let pos_bytes = encode_to_vec(&pos).expect("encode NonZeroI16(i16::MAX)");
    let (pos_dec, _): (NonZeroI16, usize) =
        decode_from_slice(&pos_bytes).expect("decode NonZeroI16(i16::MAX)");
    assert_eq!(pos, pos_dec);
}

// ---------------------------------------------------------------------------
// 9. NonZeroI32 roundtrip: i32::MIN and i32::MAX
// ---------------------------------------------------------------------------
#[test]
fn test_nonzero_i32_boundary() {
    let neg = NonZeroI32::new(i32::MIN).expect("nonzero i32::MIN");
    let pos = NonZeroI32::new(i32::MAX).expect("nonzero i32::MAX");

    let neg_bytes = encode_to_vec(&neg).expect("encode NonZeroI32(i32::MIN)");
    let (neg_dec, _): (NonZeroI32, usize) =
        decode_from_slice(&neg_bytes).expect("decode NonZeroI32(i32::MIN)");
    assert_eq!(neg, neg_dec);

    let pos_bytes = encode_to_vec(&pos).expect("encode NonZeroI32(i32::MAX)");
    let (pos_dec, _): (NonZeroI32, usize) =
        decode_from_slice(&pos_bytes).expect("decode NonZeroI32(i32::MAX)");
    assert_eq!(pos, pos_dec);
}

// ---------------------------------------------------------------------------
// 10. NonZeroI64 roundtrip: i64::MIN and i64::MAX
// ---------------------------------------------------------------------------
#[test]
fn test_nonzero_i64_boundary() {
    let neg = NonZeroI64::new(i64::MIN).expect("nonzero i64::MIN");
    let pos = NonZeroI64::new(i64::MAX).expect("nonzero i64::MAX");

    let neg_bytes = encode_to_vec(&neg).expect("encode NonZeroI64(i64::MIN)");
    let (neg_dec, _): (NonZeroI64, usize) =
        decode_from_slice(&neg_bytes).expect("decode NonZeroI64(i64::MIN)");
    assert_eq!(neg, neg_dec);

    let pos_bytes = encode_to_vec(&pos).expect("encode NonZeroI64(i64::MAX)");
    let (pos_dec, _): (NonZeroI64, usize) =
        decode_from_slice(&pos_bytes).expect("decode NonZeroI64(i64::MAX)");
    assert_eq!(pos, pos_dec);
}

// ---------------------------------------------------------------------------
// 11. NonZeroUsize and NonZeroIsize roundtrip
// ---------------------------------------------------------------------------
#[test]
fn test_nonzero_usize_roundtrip() {
    // Use a mid-range value portable across 32/64-bit targets.
    let v = NonZeroUsize::new(0x0001_FFFF).expect("nonzero usize");
    let bytes = encode_to_vec(&v).expect("encode NonZeroUsize");
    let (decoded, _): (NonZeroUsize, usize) =
        decode_from_slice(&bytes).expect("decode NonZeroUsize");
    assert_eq!(v, decoded);
}

#[test]
fn test_nonzero_isize_roundtrip() {
    let v = NonZeroIsize::new(-100_000).expect("nonzero isize");
    let bytes = encode_to_vec(&v).expect("encode NonZeroIsize");
    let (decoded, _): (NonZeroIsize, usize) =
        decode_from_slice(&bytes).expect("decode NonZeroIsize");
    assert_eq!(v, decoded);
}

// ---------------------------------------------------------------------------
// 12. Vec<NonZeroU32> roundtrip
// ---------------------------------------------------------------------------
#[test]
fn test_nonzero_vec_u32_roundtrip() {
    let values: Vec<NonZeroU32> = [1u32, 251, 65535, 65536, u32::MAX]
        .iter()
        .map(|&n| NonZeroU32::new(n).expect("nonzero"))
        .collect();
    let bytes = encode_to_vec(&values).expect("encode Vec<NonZeroU32>");
    let (decoded, _): (Vec<NonZeroU32>, usize) =
        decode_from_slice(&bytes).expect("decode Vec<NonZeroU32>");
    assert_eq!(values, decoded);
}

// ---------------------------------------------------------------------------
// 13. Option<NonZeroU64> Some and None roundtrip
// ---------------------------------------------------------------------------
#[test]
fn test_nonzero_option_u64_some_roundtrip() {
    let v: Option<NonZeroU64> = Some(NonZeroU64::new(12_345_678_901_u64).expect("nonzero"));
    let bytes = encode_to_vec(&v).expect("encode Option<NonZeroU64> Some");
    let (decoded, _): (Option<NonZeroU64>, usize) =
        decode_from_slice(&bytes).expect("decode Option<NonZeroU64> Some");
    assert_eq!(v, decoded);
}

#[test]
fn test_nonzero_option_u64_none_roundtrip() {
    let v: Option<NonZeroU64> = None;
    let bytes = encode_to_vec(&v).expect("encode Option<NonZeroU64> None");
    let (decoded, _): (Option<NonZeroU64>, usize) =
        decode_from_slice(&bytes).expect("decode Option<NonZeroU64> None");
    assert_eq!(v, decoded);
}

// ---------------------------------------------------------------------------
// 14. Byte size verification: NonZeroU8
//     u8 is always encoded as exactly 1 raw byte (not varint-expanded).
//     So NonZeroU8(1) = 1 byte, NonZeroU8(255) = 1 byte.
// ---------------------------------------------------------------------------
#[test]
fn test_nonzero_u8_byte_size_one() {
    let v = NonZeroU8::new(1).expect("nonzero");
    let bytes = encode_to_vec(&v).expect("encode NonZeroU8(1)");
    assert_eq!(
        bytes.len(),
        1,
        "NonZeroU8(1) must encode to exactly 1 byte, got {}",
        bytes.len()
    );
}

#[test]
fn test_nonzero_u8_byte_size_max() {
    let v = NonZeroU8::new(255).expect("nonzero");
    let bytes = encode_to_vec(&v).expect("encode NonZeroU8(255)");
    assert_eq!(
        bytes.len(),
        1,
        "NonZeroU8(255) must encode to exactly 1 byte, got {}",
        bytes.len()
    );
}

// ---------------------------------------------------------------------------
// 15. Byte size verification: NonZeroU16(256)
//     256 > 250, so varint encoding: 3 bytes [0xFB marker + 2-byte LE u16].
// ---------------------------------------------------------------------------
#[test]
fn test_nonzero_u16_256_byte_size() {
    let v = NonZeroU16::new(256).expect("nonzero");
    let bytes = encode_to_vec(&v).expect("encode NonZeroU16(256)");
    assert_eq!(
        bytes.len(),
        3,
        "NonZeroU16(256) must encode to exactly 3 bytes (varint 0xFB prefix), got {}",
        bytes.len()
    );
    // Also verify the 0xFB marker prefix and LE bytes for 256 = 0x0100.
    assert_eq!(bytes[0], 0xFB, "varint marker for 256 must be 0xFB");
    assert_eq!(&bytes[1..], &[0x00, 0x01], "LE u16(256) = [0x00, 0x01]");
}

// ---------------------------------------------------------------------------
// 16. Byte size verification: NonZeroU32(65536)
//     65536 > 65535, so varint encoding: 5 bytes [0xFC marker + 4-byte LE u32].
// ---------------------------------------------------------------------------
#[test]
fn test_nonzero_u32_65536_byte_size() {
    let v = NonZeroU32::new(65536).expect("nonzero");
    let bytes = encode_to_vec(&v).expect("encode NonZeroU32(65536)");
    assert_eq!(
        bytes.len(),
        5,
        "NonZeroU32(65536) must encode to exactly 5 bytes, got {}",
        bytes.len()
    );
    // 0xFC is the 5-byte varint marker; LE u32(65536) = [0x00, 0x00, 0x01, 0x00]
    assert_eq!(bytes[0], 0xFC, "varint marker for 65536 must be 0xFC");
    assert_eq!(
        &bytes[1..],
        &[0x00, 0x00, 0x01, 0x00],
        "LE u32(65536) = [0x00, 0x00, 0x01, 0x00]"
    );
}

// ---------------------------------------------------------------------------
// 17. Struct with NonZero field - derive-based roundtrip
// ---------------------------------------------------------------------------
use oxicode::{Decode, Encode};

#[derive(Debug, PartialEq, Encode, Decode)]
struct TaskRecord {
    id: NonZeroU32,
    priority: NonZeroU16,
    tag: u64,
}

#[test]
fn test_nonzero_struct_field_roundtrip() {
    let original = TaskRecord {
        id: NonZeroU32::new(42_000).expect("nonzero id"),
        priority: NonZeroU16::new(10).expect("nonzero priority"),
        tag: 0xDEAD_BEEF_CAFE_0001_u64,
    };
    let bytes = encode_to_vec(&original).expect("encode TaskRecord");
    let (decoded, consumed): (TaskRecord, usize) =
        decode_from_slice(&bytes).expect("decode TaskRecord");
    assert_eq!(original, decoded);
    assert_eq!(consumed, bytes.len());
}

// ---------------------------------------------------------------------------
// 18. Struct with multiple NonZero fields and Option<NonZeroU64>
// ---------------------------------------------------------------------------
#[derive(Debug, PartialEq, Encode, Decode)]
struct OptionalNonZeroRecord {
    required: NonZeroU32,
    optional_count: Option<NonZeroU64>,
    label: NonZeroI32,
}

#[test]
fn test_nonzero_struct_with_option_some() {
    let original = OptionalNonZeroRecord {
        required: NonZeroU32::new(1).expect("nonzero required"),
        optional_count: Some(NonZeroU64::new(u64::MAX).expect("nonzero count")),
        label: NonZeroI32::new(-999).expect("nonzero label"),
    };
    let bytes = encode_to_vec(&original).expect("encode OptionalNonZeroRecord Some");
    let (decoded, _): (OptionalNonZeroRecord, usize) =
        decode_from_slice(&bytes).expect("decode OptionalNonZeroRecord Some");
    assert_eq!(original, decoded);
}

#[test]
fn test_nonzero_struct_with_option_none() {
    let original = OptionalNonZeroRecord {
        required: NonZeroU32::new(u32::MAX).expect("nonzero required"),
        optional_count: None,
        label: NonZeroI32::new(i32::MAX).expect("nonzero label"),
    };
    let bytes = encode_to_vec(&original).expect("encode OptionalNonZeroRecord None");
    let (decoded, _): (OptionalNonZeroRecord, usize) =
        decode_from_slice(&bytes).expect("decode OptionalNonZeroRecord None");
    assert_eq!(original, decoded);
}

// ---------------------------------------------------------------------------
// 19. NonZeroU128 large value roundtrip
// ---------------------------------------------------------------------------
#[test]
fn test_nonzero_u128_large_value() {
    // Use a 128-bit value that doesn't fit in u64 to exercise wide-integer encoding.
    let big: u128 = (u64::MAX as u128) + 1;
    let v = NonZeroU128::new(big).expect("nonzero u128");
    let bytes = encode_to_vec(&v).expect("encode NonZeroU128");
    let (decoded, _): (NonZeroU128, usize) = decode_from_slice(&bytes).expect("decode NonZeroU128");
    assert_eq!(v, decoded);
}

// ---------------------------------------------------------------------------
// 20. NonZeroI128 min/max roundtrip
// ---------------------------------------------------------------------------
#[test]
fn test_nonzero_i128_boundary() {
    let neg = NonZeroI128::new(i128::MIN).expect("nonzero i128::MIN");
    let pos = NonZeroI128::new(i128::MAX).expect("nonzero i128::MAX");

    let neg_bytes = encode_to_vec(&neg).expect("encode NonZeroI128(i128::MIN)");
    let (neg_dec, _): (NonZeroI128, usize) =
        decode_from_slice(&neg_bytes).expect("decode NonZeroI128(i128::MIN)");
    assert_eq!(neg, neg_dec);

    let pos_bytes = encode_to_vec(&pos).expect("encode NonZeroI128(i128::MAX)");
    let (pos_dec, _): (NonZeroI128, usize) =
        decode_from_slice(&pos_bytes).expect("decode NonZeroI128(i128::MAX)");
    assert_eq!(pos, pos_dec);
}

// ---------------------------------------------------------------------------
// 21. Zero-value decode must fail: NonZeroU8, NonZeroI32, NonZeroU64
//     These tests confirm the invariant is enforced at decode time.
// ---------------------------------------------------------------------------
#[test]
fn test_nonzero_u8_zero_decode_fails() {
    let zero_bytes = encode_to_vec(&0u8).expect("encode 0u8");
    let result: Result<(NonZeroU8, usize), _> = decode_from_slice(&zero_bytes);
    assert!(
        result.is_err(),
        "Decoding zero-byte representation as NonZeroU8 must return an error"
    );
}

#[test]
fn test_nonzero_i32_zero_decode_fails() {
    let zero_bytes = encode_to_vec(&0i32).expect("encode 0i32");
    let result: Result<(NonZeroI32, usize), _> = decode_from_slice(&zero_bytes);
    assert!(
        result.is_err(),
        "Decoding zero-byte representation as NonZeroI32 must return an error"
    );
}

#[test]
fn test_nonzero_u64_zero_decode_fails() {
    let zero_bytes = encode_to_vec(&0u64).expect("encode 0u64");
    let result: Result<(NonZeroU64, usize), _> = decode_from_slice(&zero_bytes);
    assert!(
        result.is_err(),
        "Decoding zero-byte representation as NonZeroU64 must return an error"
    );
}

// ---------------------------------------------------------------------------
// 22. Vec<NonZeroU8> roundtrip with mixed boundary values
// ---------------------------------------------------------------------------
#[test]
fn test_nonzero_vec_u8_mixed_values() {
    let values: Vec<NonZeroU8> = [1u8, 127, 128, 250, 251, 255]
        .iter()
        .map(|&n| NonZeroU8::new(n).expect("nonzero"))
        .collect();
    let bytes = encode_to_vec(&values).expect("encode Vec<NonZeroU8>");
    let (decoded, consumed): (Vec<NonZeroU8>, usize) =
        decode_from_slice(&bytes).expect("decode Vec<NonZeroU8>");
    assert_eq!(values, decoded);
    assert_eq!(consumed, bytes.len());
}
