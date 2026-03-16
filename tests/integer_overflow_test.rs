//! Comprehensive tests covering integer boundary values, overflow detection,
//! and edge cases in OxiCode encoding.
//!
//! Each test is a standalone focused unit for a specific type/value, verifying:
//!   - roundtrip correctness (encode then decode yields the original value)
//!   - exact byte-consumption reported by decode_from_slice
//!   - encoded size properties where applicable
//!
//! Note: the existing `integer_types_test` and `varint_boundary_test` files cover
//! similar values via macros; these tests are deliberately individual so that a
//! failure pinpoints a single type/boundary without noise from other values.

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

// ── helpers ──────────────────────────────────────────────────────────────────

/// Encode `value`, decode it back, assert equality, and return the encoded bytes
/// so callers can make additional assertions about the wire representation.
fn roundtrip_and_return<T>(value: T) -> (T, Vec<u8>)
where
    T: oxicode::enc::Encode + oxicode::de::Decode + PartialEq + std::fmt::Debug,
{
    let enc = encode_to_vec(&value).expect("encode_to_vec must not fail");
    let (dec, consumed): (T, usize) =
        decode_from_slice(&enc).expect("decode_from_slice must not fail");
    assert_eq!(value, dec, "roundtrip mismatch");
    assert_eq!(
        consumed,
        enc.len(),
        "decode must consume exactly the encoded bytes"
    );
    (dec, enc)
}

// ── test 1: u8::MIN ──────────────────────────────────────────────────────────

#[test]
fn test_u8_min_roundtrip() {
    let (dec, enc) = roundtrip_and_return(u8::MIN);
    assert_eq!(dec, 0u8);
    // u8::MIN (0) must encode as a single byte in varint format (0-250 → 1 byte)
    assert_eq!(enc.len(), 1, "u8::MIN must encode as 1 byte");
}

// ── test 2: u8::MAX ──────────────────────────────────────────────────────────

#[test]
fn test_u8_max_roundtrip() {
    let (dec, enc) = roundtrip_and_return(u8::MAX);
    assert_eq!(dec, 255u8);
    // u8 is encoded as a raw single byte regardless of varint thresholds
    assert_eq!(
        enc.len(),
        1,
        "u8::MAX must encode as exactly 1 raw byte for u8 type"
    );
}

// ── test 3: u16::MIN ─────────────────────────────────────────────────────────

#[test]
fn test_u16_min_roundtrip() {
    let (dec, enc) = roundtrip_and_return(u16::MIN);
    assert_eq!(dec, 0u16);
    // 0 fits in the 0-250 single-byte varint range
    assert_eq!(enc.len(), 1, "u16::MIN (0) must encode as 1 byte");
}

// ── test 4: u16::MAX ─────────────────────────────────────────────────────────

#[test]
fn test_u16_max_roundtrip() {
    let (dec, enc) = roundtrip_and_return(u16::MAX);
    assert_eq!(dec, 65535u16);
    // 65535 > 250 so it needs a multi-byte varint: marker byte + 2 bytes = 3 bytes
    assert_eq!(
        enc.len(),
        3,
        "u16::MAX must encode as 3 bytes (marker + u16)"
    );
}

// ── test 5: u32::MIN ─────────────────────────────────────────────────────────

#[test]
fn test_u32_min_roundtrip() {
    let (dec, enc) = roundtrip_and_return(u32::MIN);
    assert_eq!(dec, 0u32);
    assert_eq!(enc.len(), 1, "u32::MIN (0) must encode as 1 byte");
}

// ── test 6: u32::MAX ─────────────────────────────────────────────────────────

#[test]
fn test_u32_max_roundtrip() {
    let (dec, enc) = roundtrip_and_return(u32::MAX);
    assert_eq!(dec, 4_294_967_295u32);
    // u32::MAX > u16::MAX → marker byte + 4 bytes = 5 bytes
    assert_eq!(
        enc.len(),
        5,
        "u32::MAX must encode as 5 bytes (marker + u32)"
    );
}

// ── test 7: u64::MIN ─────────────────────────────────────────────────────────

#[test]
fn test_u64_min_roundtrip() {
    let (dec, enc) = roundtrip_and_return(u64::MIN);
    assert_eq!(dec, 0u64);
    assert_eq!(enc.len(), 1, "u64::MIN (0) must encode as 1 byte");
}

// ── test 8: u64::MAX ─────────────────────────────────────────────────────────

#[test]
fn test_u64_max_roundtrip() {
    let (dec, enc) = roundtrip_and_return(u64::MAX);
    assert_eq!(dec, 18_446_744_073_709_551_615u64);
    // u64::MAX > u32::MAX → marker byte + 8 bytes = 9 bytes
    assert_eq!(
        enc.len(),
        9,
        "u64::MAX must encode as 9 bytes (marker + u64)"
    );
}

// ── test 9: u128::MIN ────────────────────────────────────────────────────────

#[test]
fn test_u128_min_roundtrip() {
    let (dec, enc) = roundtrip_and_return(u128::MIN);
    assert_eq!(dec, 0u128);
    assert_eq!(enc.len(), 1, "u128::MIN (0) must encode as 1 byte");
}

// ── test 10: u128::MAX ───────────────────────────────────────────────────────

#[test]
fn test_u128_max_roundtrip() {
    let value = u128::MAX;
    let enc = encode_to_vec(&value).expect("encode_to_vec must not fail");
    let (dec, consumed): (u128, usize) =
        decode_from_slice(&enc).expect("decode_from_slice must not fail");
    assert_eq!(value, dec, "u128::MAX roundtrip failed");
    assert_eq!(
        consumed,
        enc.len(),
        "consumed byte count must match encoded length"
    );
    // u128::MAX requires more than 9 bytes; exact size depends on implementation
    assert!(
        enc.len() > 9,
        "u128::MAX should require more than 9 bytes, got {}",
        enc.len()
    );
}

// ── test 11: i8::MIN ─────────────────────────────────────────────────────────

#[test]
fn test_i8_min_roundtrip() {
    let (dec, _enc) = roundtrip_and_return(i8::MIN);
    assert_eq!(dec, -128i8);
}

// ── test 12: i8::MAX ─────────────────────────────────────────────────────────

#[test]
fn test_i8_max_roundtrip() {
    let (dec, _enc) = roundtrip_and_return(i8::MAX);
    assert_eq!(dec, 127i8);
}

// ── test 13: i16::MIN ────────────────────────────────────────────────────────

#[test]
fn test_i16_min_roundtrip() {
    let (dec, _enc) = roundtrip_and_return(i16::MIN);
    assert_eq!(dec, -32768i16);
}

// ── test 14: i16::MAX ────────────────────────────────────────────────────────

#[test]
fn test_i16_max_roundtrip() {
    let (dec, _enc) = roundtrip_and_return(i16::MAX);
    assert_eq!(dec, 32767i16);
}

// ── test 15: i32::MIN ────────────────────────────────────────────────────────

#[test]
fn test_i32_min_roundtrip() {
    let (dec, _enc) = roundtrip_and_return(i32::MIN);
    assert_eq!(dec, -2_147_483_648i32);
}

// ── test 16: i32::MAX ────────────────────────────────────────────────────────

#[test]
fn test_i32_max_roundtrip() {
    let (dec, _enc) = roundtrip_and_return(i32::MAX);
    assert_eq!(dec, 2_147_483_647i32);
}

// ── test 17: i64::MIN ────────────────────────────────────────────────────────

#[test]
fn test_i64_min_roundtrip() {
    let (dec, _enc) = roundtrip_and_return(i64::MIN);
    assert_eq!(dec, -9_223_372_036_854_775_808i64);
}

// ── test 18: i64::MAX ────────────────────────────────────────────────────────

#[test]
fn test_i64_max_roundtrip() {
    let (dec, _enc) = roundtrip_and_return(i64::MAX);
    assert_eq!(dec, 9_223_372_036_854_775_807i64);
}

// ── test 19: i128::MIN ───────────────────────────────────────────────────────

#[test]
fn test_i128_min_roundtrip() {
    let value = i128::MIN;
    let enc = encode_to_vec(&value).expect("encode_to_vec must not fail");
    let (dec, consumed): (i128, usize) =
        decode_from_slice(&enc).expect("decode_from_slice must not fail");
    assert_eq!(value, dec, "i128::MIN roundtrip failed");
    assert_eq!(
        consumed,
        enc.len(),
        "consumed byte count must match encoded length"
    );
    // i128::MIN has a very large zigzag representation; must require multiple bytes
    assert!(
        enc.len() > 1,
        "i128::MIN should require more than 1 byte to encode"
    );
}

// ── test 20: i128::MAX ───────────────────────────────────────────────────────

#[test]
fn test_i128_max_roundtrip() {
    let value = i128::MAX;
    let enc = encode_to_vec(&value).expect("encode_to_vec must not fail");
    let (dec, consumed): (i128, usize) =
        decode_from_slice(&enc).expect("decode_from_slice must not fail");
    assert_eq!(value, dec, "i128::MAX roundtrip failed");
    assert_eq!(
        consumed,
        enc.len(),
        "consumed byte count must match encoded length"
    );
    assert!(
        enc.len() > 1,
        "i128::MAX should require more than 1 byte to encode"
    );
}

// ── test 21: usize::MAX ──────────────────────────────────────────────────────

#[test]
fn test_usize_max_roundtrip() {
    // usize is platform-dependent; on 64-bit systems this is u64::MAX value
    let value = usize::MAX;
    let enc = encode_to_vec(&value).expect("encode_to_vec must not fail");
    let (dec, consumed): (usize, usize) =
        decode_from_slice(&enc).expect("decode_from_slice must not fail");
    assert_eq!(value, dec, "usize::MAX roundtrip failed");
    assert_eq!(
        consumed,
        enc.len(),
        "consumed byte count must match encoded length"
    );
    // On any supported platform usize::MAX > 250, so it must use more than 1 byte
    assert!(
        enc.len() > 1,
        "usize::MAX must require more than 1 byte to encode (got {} bytes)",
        enc.len()
    );
}

// ── test 22: u8::MAX encodes as the single raw byte 0xFF ─────────────────────

/// Verify the raw wire format: u8 values must be encoded as a single raw byte
/// (not varint-wrapped), so u8::MAX (255 = 0xFF) must produce exactly [0xFF].
///
/// This is a fundamental guarantee of the OxiCode wire format for u8: unlike
/// larger unsigned integers that go through varint encoding (where 255 would
/// require a 3-byte varint as it exceeds the 0-250 single-byte threshold),
/// the u8 primitive type is always stored as one literal byte to preserve its
/// native width and allow zero-copy slice access.
#[test]
fn test_u8_max_encodes_as_single_byte_0xff() {
    let value: u8 = u8::MAX; // 255
    let enc = encode_to_vec(&value).expect("encode_to_vec must not fail");

    assert_eq!(
        enc.len(),
        1,
        "u8::MAX must produce exactly 1 encoded byte, got {} bytes: {:?}",
        enc.len(),
        enc
    );
    assert_eq!(
        enc[0], 0xFF,
        "u8::MAX must encode as the single raw byte 0xFF, got 0x{:02X}",
        enc[0]
    );

    // Confirm the byte decodes back correctly
    let (dec, consumed): (u8, usize) =
        decode_from_slice(&enc).expect("decode_from_slice must not fail");
    assert_eq!(dec, 255u8, "0xFF must decode back to u8::MAX");
    assert_eq!(consumed, 1, "decoding u8::MAX must consume exactly 1 byte");
}
