//! Advanced integer boundary and varint encoding edge-case tests.
//!
//! Covers all integer types at boundary values, varint encoding size invariants,
//! zigzag encoding for signed types, fixed-int and big-endian configs, mixed
//! structs, negative i64 roundtrips, and u128/i128 boundary values.

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
    encode_to_vec_with_config, Decode, Encode,
};

// ---------------------------------------------------------------------------
// 1. u8: 0, 1, 127, 128, 255 — varint size checks
//    • 0..=250  → 1 byte
//    • 255      → 3 bytes (marker byte 251 + 2-byte u16 payload)
// ---------------------------------------------------------------------------

/// u8 values in 0..=250 must each encode to exactly 1 byte.
#[test]
fn test_u8_values_0_to_250_single_byte() {
    for val in [0u8, 1u8, 127u8, 128u8, 250u8] {
        let bytes = encode_to_vec(&val).expect("encode u8");
        assert_eq!(
            bytes.len(),
            1,
            "u8={val} should encode to 1 byte (varint single-byte range)"
        );
        let (decoded, consumed): (u8, usize) = decode_from_slice(&bytes).expect("decode u8");
        assert_eq!(decoded, val, "u8={val} roundtrip");
        assert_eq!(consumed, 1, "consumed bytes for u8={val}");
    }
}

/// u8 is stored with fixed-width semantics (always 1 byte), not through the
/// multi-byte varint path.  u8=255 therefore encodes to exactly 1 byte and
/// decodes back to the original value.
#[test]
fn test_u8_value_255_one_byte_fixed_width() {
    let val: u8 = 255;
    let bytes = encode_to_vec(&val).expect("encode u8=255");
    assert_eq!(
        bytes.len(),
        1,
        "u8=255 must encode to 1 byte (u8 uses fixed-width storage, not varint)"
    );
    let (decoded, consumed): (u8, usize) = decode_from_slice(&bytes).expect("decode u8=255");
    assert_eq!(decoded, val, "u8=255 roundtrip");
    assert_eq!(consumed, 1, "consumed bytes for u8=255");
}

// ---------------------------------------------------------------------------
// 2. u16: 0, 250, 251, 65535 — boundary at 251 (3 bytes) vs ≤250 (1 byte)
// ---------------------------------------------------------------------------

/// u16 values 0 and 250 must each encode to 1 byte.
#[test]
fn test_u16_values_at_or_below_250_single_byte() {
    for val in [0u16, 250u16] {
        let bytes = encode_to_vec(&val).expect("encode u16");
        assert_eq!(bytes.len(), 1, "u16={val} should encode to 1 byte");
        let (decoded, _): (u16, usize) = decode_from_slice(&bytes).expect("decode u16");
        assert_eq!(decoded, val, "u16={val} roundtrip");
    }
}

/// u16=251 is the first value above the single-byte threshold; must encode to
/// 3 bytes (marker byte + 2-byte u16 payload).
#[test]
fn test_u16_value_251_three_bytes() {
    let val: u16 = 251;
    let bytes = encode_to_vec(&val).expect("encode u16=251");
    assert_eq!(
        bytes.len(),
        3,
        "u16=251 should encode to 3 bytes (varint prefix + 2 bytes)"
    );
    let (decoded, _): (u16, usize) = decode_from_slice(&bytes).expect("decode u16=251");
    assert_eq!(decoded, val, "u16=251 roundtrip");
}

/// u16=65535 (u16::MAX) must encode to 3 bytes.
#[test]
fn test_u16_max_three_bytes() {
    let val: u16 = u16::MAX;
    let bytes = encode_to_vec(&val).expect("encode u16::MAX");
    assert_eq!(bytes.len(), 3, "u16::MAX should encode to 3 bytes");
    let (decoded, _): (u16, usize) = decode_from_slice(&bytes).expect("decode u16::MAX");
    assert_eq!(decoded, val, "u16::MAX roundtrip");
}

// ---------------------------------------------------------------------------
// 3. u32: 0, 250, 251, 65535, 65536, u32::MAX — boundary at 65536 (5 bytes)
// ---------------------------------------------------------------------------

/// u32 values 0 and 250 encode to 1 byte.
#[test]
fn test_u32_values_0_and_250_single_byte() {
    for val in [0u32, 250u32] {
        let bytes = encode_to_vec(&val).expect("encode u32");
        assert_eq!(bytes.len(), 1, "u32={val} should encode to 1 byte");
        let (decoded, _): (u32, usize) = decode_from_slice(&bytes).expect("decode u32");
        assert_eq!(decoded, val, "u32={val} roundtrip");
    }
}

/// u32 values 251 and 65535 encode to 3 bytes (marker + u16 payload).
#[test]
fn test_u32_values_251_and_65535_three_bytes() {
    for val in [251u32, 65535u32] {
        let bytes = encode_to_vec(&val).expect("encode u32");
        assert_eq!(bytes.len(), 3, "u32={val} should encode to 3 bytes");
        let (decoded, _): (u32, usize) = decode_from_slice(&bytes).expect("decode u32");
        assert_eq!(decoded, val, "u32={val} roundtrip");
    }
}

/// u32=65536 is the first value above u16::MAX; must encode to 5 bytes
/// (marker byte + 4-byte u32 payload).
#[test]
fn test_u32_value_65536_five_bytes() {
    let val: u32 = 65536;
    let bytes = encode_to_vec(&val).expect("encode u32=65536");
    assert_eq!(
        bytes.len(),
        5,
        "u32=65536 should encode to 5 bytes (marker + 4-byte u32 payload)"
    );
    let (decoded, _): (u32, usize) = decode_from_slice(&bytes).expect("decode u32=65536");
    assert_eq!(decoded, val, "u32=65536 roundtrip");
}

/// u32::MAX must encode to 5 bytes.
#[test]
fn test_u32_max_five_bytes() {
    let val: u32 = u32::MAX;
    let bytes = encode_to_vec(&val).expect("encode u32::MAX");
    assert_eq!(bytes.len(), 5, "u32::MAX should encode to 5 bytes");
    let (decoded, _): (u32, usize) = decode_from_slice(&bytes).expect("decode u32::MAX");
    assert_eq!(decoded, val, "u32::MAX roundtrip");
}

// ---------------------------------------------------------------------------
// 4. u64: 0, u64::MAX
// ---------------------------------------------------------------------------

/// u64=0 encodes to 1 byte.
#[test]
fn test_u64_zero_single_byte() {
    let val: u64 = 0;
    let bytes = encode_to_vec(&val).expect("encode u64=0");
    assert_eq!(bytes.len(), 1, "u64=0 should encode to 1 byte");
    let (decoded, _): (u64, usize) = decode_from_slice(&bytes).expect("decode u64=0");
    assert_eq!(decoded, val);
}

/// u64::MAX lies above u32::MAX and must encode to 9 bytes
/// (marker byte + 8-byte u64 payload).
#[test]
fn test_u64_max_nine_bytes() {
    let val: u64 = u64::MAX;
    let bytes = encode_to_vec(&val).expect("encode u64::MAX");
    assert_eq!(
        bytes.len(),
        9,
        "u64::MAX should encode to 9 bytes (marker + 8-byte u64 payload)"
    );
    let (decoded, _): (u64, usize) = decode_from_slice(&bytes).expect("decode u64::MAX");
    assert_eq!(decoded, val);
}

// ---------------------------------------------------------------------------
// 5. Signed integers — zigzag encoding: i8, i16, i32, i64 at MIN, -1, 0, 1, MAX
// ---------------------------------------------------------------------------

/// i8 boundary values roundtrip correctly through zigzag encoding.
#[test]
fn test_i8_zigzag_boundary_roundtrip() {
    for val in [i8::MIN, -1i8, 0i8, 1i8, i8::MAX] {
        let bytes = encode_to_vec(&val).expect("encode i8");
        let (decoded, _): (i8, usize) = decode_from_slice(&bytes).expect("decode i8");
        assert_eq!(decoded, val, "i8={val} zigzag roundtrip");
    }
}

/// i16 boundary values roundtrip correctly through zigzag encoding.
#[test]
fn test_i16_zigzag_boundary_roundtrip() {
    for val in [i16::MIN, -1i16, 0i16, 1i16, i16::MAX] {
        let bytes = encode_to_vec(&val).expect("encode i16");
        let (decoded, _): (i16, usize) = decode_from_slice(&bytes).expect("decode i16");
        assert_eq!(decoded, val, "i16={val} zigzag roundtrip");
    }
}

/// i32 boundary values roundtrip correctly through zigzag encoding.
#[test]
fn test_i32_zigzag_boundary_roundtrip() {
    for val in [i32::MIN, -1i32, 0i32, 1i32, i32::MAX] {
        let bytes = encode_to_vec(&val).expect("encode i32");
        let (decoded, _): (i32, usize) = decode_from_slice(&bytes).expect("decode i32");
        assert_eq!(decoded, val, "i32={val} zigzag roundtrip");
    }
}

/// i64 boundary values roundtrip correctly through zigzag encoding.
#[test]
fn test_i64_zigzag_boundary_roundtrip() {
    for val in [i64::MIN, -1i64, 0i64, 1i64, i64::MAX] {
        let bytes = encode_to_vec(&val).expect("encode i64");
        let (decoded, _): (i64, usize) = decode_from_slice(&bytes).expect("decode i64");
        assert_eq!(decoded, val, "i64={val} zigzag roundtrip");
    }
}

// ---------------------------------------------------------------------------
// 6. Config: fixed_int encoding — all integers use fixed-width, no varint
// ---------------------------------------------------------------------------

/// With `with_fixed_int_encoding()`, u8/u16/u32/u64 must use fixed widths
/// (1, 2, 4, 8 bytes respectively) regardless of value.
#[test]
fn test_fixed_int_encoding_fixed_widths() {
    let cfg = config::standard().with_fixed_int_encoding();

    // u8: always 1 byte
    for val in [0u8, 1u8, 127u8, 255u8] {
        let bytes = encode_to_vec_with_config(&val, cfg).expect("encode u8 fixed");
        assert_eq!(bytes.len(), 1, "u8={val} fixed-int: must be 1 byte");
        let (decoded, _): (u8, usize) =
            decode_from_slice_with_config(&bytes, cfg).expect("decode u8 fixed");
        assert_eq!(decoded, val);
    }

    // u16: always 2 bytes
    for val in [0u16, 251u16, u16::MAX] {
        let bytes = encode_to_vec_with_config(&val, cfg).expect("encode u16 fixed");
        assert_eq!(bytes.len(), 2, "u16={val} fixed-int: must be 2 bytes");
        let (decoded, _): (u16, usize) =
            decode_from_slice_with_config(&bytes, cfg).expect("decode u16 fixed");
        assert_eq!(decoded, val);
    }

    // u32: always 4 bytes
    for val in [0u32, 251u32, 65536u32, u32::MAX] {
        let bytes = encode_to_vec_with_config(&val, cfg).expect("encode u32 fixed");
        assert_eq!(bytes.len(), 4, "u32={val} fixed-int: must be 4 bytes");
        let (decoded, _): (u32, usize) =
            decode_from_slice_with_config(&bytes, cfg).expect("decode u32 fixed");
        assert_eq!(decoded, val);
    }

    // u64: always 8 bytes
    for val in [0u64, 251u64, u64::MAX] {
        let bytes = encode_to_vec_with_config(&val, cfg).expect("encode u64 fixed");
        assert_eq!(bytes.len(), 8, "u64={val} fixed-int: must be 8 bytes");
        let (decoded, _): (u64, usize) =
            decode_from_slice_with_config(&bytes, cfg).expect("decode u64 fixed");
        assert_eq!(decoded, val);
    }
}

// ---------------------------------------------------------------------------
// 7. Config: big_endian — roundtrips for boundary values
// ---------------------------------------------------------------------------

/// With `with_big_endian()`, all integer boundary values roundtrip correctly.
#[test]
fn test_big_endian_integer_boundary_roundtrip() {
    let cfg = config::standard().with_big_endian();

    let bytes = encode_to_vec_with_config(&u8::MAX, cfg).expect("encode u8 be");
    let (v, _): (u8, usize) = decode_from_slice_with_config(&bytes, cfg).expect("decode u8 be");
    assert_eq!(v, u8::MAX);

    let bytes = encode_to_vec_with_config(&u16::MAX, cfg).expect("encode u16 be");
    let (v, _): (u16, usize) = decode_from_slice_with_config(&bytes, cfg).expect("decode u16 be");
    assert_eq!(v, u16::MAX);

    let bytes = encode_to_vec_with_config(&u32::MAX, cfg).expect("encode u32 be");
    let (v, _): (u32, usize) = decode_from_slice_with_config(&bytes, cfg).expect("decode u32 be");
    assert_eq!(v, u32::MAX);

    let bytes = encode_to_vec_with_config(&u64::MAX, cfg).expect("encode u64 be");
    let (v, _): (u64, usize) = decode_from_slice_with_config(&bytes, cfg).expect("decode u64 be");
    assert_eq!(v, u64::MAX);

    let bytes = encode_to_vec_with_config(&i64::MIN, cfg).expect("encode i64 be");
    let (v, _): (i64, usize) = decode_from_slice_with_config(&bytes, cfg).expect("decode i64 be");
    assert_eq!(v, i64::MIN);
}

// ---------------------------------------------------------------------------
// 8. Struct with mixed integer types
// ---------------------------------------------------------------------------

#[derive(Debug, PartialEq, Encode, Decode)]
struct MixedIntStruct {
    a: u8,
    b: i8,
    c: u16,
    d: i16,
    e: u32,
    f: i32,
    g: u64,
    h: i64,
}

/// A struct containing every integer width at boundary values roundtrips correctly.
#[test]
fn test_mixed_int_struct_boundary_roundtrip() {
    let original = MixedIntStruct {
        a: u8::MAX,
        b: i8::MIN,
        c: u16::MAX,
        d: i16::MIN,
        e: u32::MAX,
        f: i32::MIN,
        g: u64::MAX,
        h: i64::MIN,
    };
    let bytes = encode_to_vec(&original).expect("encode MixedIntStruct");
    let (decoded, _): (MixedIntStruct, usize) =
        decode_from_slice(&bytes).expect("decode MixedIntStruct");
    assert_eq!(decoded, original, "MixedIntStruct boundary roundtrip");
}

// ---------------------------------------------------------------------------
// 9. Negative i64 values — comprehensive zigzag roundtrip
// ---------------------------------------------------------------------------

/// Negative i64 values including -1, -2, i64::MIN, and large negatives all
/// roundtrip correctly through zigzag encoding.
#[test]
fn test_negative_i64_comprehensive_zigzag_roundtrip() {
    for val in [
        -1i64,
        -2i64,
        -127i64,
        -128i64,
        -32768i64,
        -2_147_483_648i64,
        i64::MIN,
    ] {
        let bytes = encode_to_vec(&val).expect("encode negative i64");
        let (decoded, _): (i64, usize) = decode_from_slice(&bytes).expect("decode negative i64");
        assert_eq!(decoded, val, "negative i64={val} zigzag roundtrip");
    }
}

// ---------------------------------------------------------------------------
// 10. u128 and i128 boundary values
// ---------------------------------------------------------------------------

/// u128 boundary values (0, 1, u128::MAX) encode and decode correctly.
#[test]
fn test_u128_boundary_roundtrip() {
    for val in [0u128, 1u128, u64::MAX as u128, u128::MAX] {
        let bytes = encode_to_vec(&val).expect("encode u128");
        let (decoded, consumed): (u128, usize) = decode_from_slice(&bytes).expect("decode u128");
        assert_eq!(decoded, val, "u128={val} roundtrip");
        assert_eq!(
            consumed,
            bytes.len(),
            "consumed bytes must equal encoded length for u128={val}"
        );
    }
}

/// i128 boundary values (i128::MIN, -1, 0, 1, i128::MAX) encode and decode correctly
/// through zigzag encoding.
#[test]
fn test_i128_boundary_roundtrip() {
    for val in [i128::MIN, -1i128, 0i128, 1i128, i128::MAX] {
        let bytes = encode_to_vec(&val).expect("encode i128");
        let (decoded, consumed): (i128, usize) = decode_from_slice(&bytes).expect("decode i128");
        assert_eq!(decoded, val, "i128={val} roundtrip");
        assert_eq!(
            consumed,
            bytes.len(),
            "consumed bytes must equal encoded length for i128={val}"
        );
    }
}

// ---------------------------------------------------------------------------
// Additional targeted edge-case tests
// ---------------------------------------------------------------------------

/// Fixed-int config must NOT change correctness for signed integers.
/// i32::MIN and i32::MAX roundtrip under fixed-int encoding.
#[test]
fn test_fixed_int_signed_boundary_roundtrip() {
    let cfg = config::standard().with_fixed_int_encoding();

    for val in [i32::MIN, -1i32, 0i32, 1i32, i32::MAX] {
        let bytes = encode_to_vec_with_config(&val, cfg).expect("encode i32 fixed");
        assert_eq!(
            bytes.len(),
            4,
            "i32={val} with fixed-int must be exactly 4 bytes"
        );
        let (decoded, _): (i32, usize) =
            decode_from_slice_with_config(&bytes, cfg).expect("decode i32 fixed");
        assert_eq!(decoded, val, "i32={val} fixed-int roundtrip");
    }
}
