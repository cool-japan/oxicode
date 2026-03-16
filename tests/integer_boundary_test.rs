//! Integer boundary value and wire format tests for OxiCode.
//!
//! Wire format facts:
//!   u8 / i8: always written as a raw single byte (no varint, no zigzag).
//!
//! Varint encoding thresholds for u16/u32/u64 (OxiCode / bincode-compatible):
//!   0–250          → 1 byte  (value itself)
//!   251–65535      → 3 bytes [0xFB, lo, hi] (little-endian u16)
//!   65536–u32::MAX → 5 bytes [0xFC, ...LE u32...]
//!   > u32::MAX     → 9 bytes [0xFD, ...LE u64...]
//!
//! Signed integers i16/i32/i64 use zigzag mapping before varint:
//!   n >= 0 → 2*n
//!   n <  0 → 2*|n| - 1

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

// ── Varint unsigned boundary tests ──────────────────────────────────────────

#[test]
fn test_u8_min_roundtrip() {
    let original: u8 = 0;
    let encoded = encode_to_vec(&original).expect("encode u8 0 failed");
    let (decoded, consumed): (u8, _) = decode_from_slice(&encoded).expect("decode u8 0 failed");
    assert_eq!(decoded, 0u8);
    assert_eq!(consumed, encoded.len());
    assert_eq!(encoded.len(), 1, "u8(0) must encode as 1 byte");
}

#[test]
fn test_u8_127_roundtrip() {
    let original: u8 = 127;
    let encoded = encode_to_vec(&original).expect("encode u8 127 failed");
    let (decoded, consumed): (u8, _) = decode_from_slice(&encoded).expect("decode u8 127 failed");
    assert_eq!(decoded, 127u8);
    assert_eq!(consumed, encoded.len());
    assert_eq!(
        encoded.len(),
        1,
        "u8(127) must encode as 1 byte (within 0–250 range)"
    );
}

#[test]
fn test_u8_128_roundtrip() {
    let original: u8 = 128;
    let encoded = encode_to_vec(&original).expect("encode u8 128 failed");
    let (decoded, consumed): (u8, _) = decode_from_slice(&encoded).expect("decode u8 128 failed");
    assert_eq!(decoded, 128u8);
    assert_eq!(consumed, encoded.len());
    assert_eq!(
        encoded.len(),
        1,
        "u8(128) must encode as 1 byte (still within 0–250 range)"
    );
}

#[test]
fn test_u8_max_roundtrip() {
    let original: u8 = 255;
    let encoded = encode_to_vec(&original).expect("encode u8 255 failed");
    let (decoded, consumed): (u8, _) = decode_from_slice(&encoded).expect("decode u8 255 failed");
    assert_eq!(decoded, 255u8);
    assert_eq!(consumed, encoded.len());
    assert_eq!(
        encoded.len(),
        1,
        "u8(255) must encode as 1 byte (u8 is always written as a raw byte, no varint)"
    );
}

#[test]
fn test_u16_min_roundtrip() {
    let original: u16 = 0;
    let encoded = encode_to_vec(&original).expect("encode u16 0 failed");
    let (decoded, consumed): (u16, _) = decode_from_slice(&encoded).expect("decode u16 0 failed");
    assert_eq!(decoded, 0u16);
    assert_eq!(consumed, encoded.len());
    assert_eq!(encoded.len(), 1, "u16(0) must encode as 1 byte");
}

#[test]
fn test_u16_250_roundtrip() {
    let original: u16 = 250;
    let encoded = encode_to_vec(&original).expect("encode u16 250 failed");
    let (decoded, consumed): (u16, _) = decode_from_slice(&encoded).expect("decode u16 250 failed");
    assert_eq!(decoded, 250u16);
    assert_eq!(consumed, encoded.len());
    assert_eq!(
        encoded.len(),
        1,
        "u16(250) must encode as 1 byte (single-byte varint max)"
    );
}

#[test]
fn test_u16_251_roundtrip() {
    let original: u16 = 251;
    let encoded = encode_to_vec(&original).expect("encode u16 251 failed");
    let (decoded, consumed): (u16, _) = decode_from_slice(&encoded).expect("decode u16 251 failed");
    assert_eq!(decoded, 251u16);
    assert_eq!(consumed, encoded.len());
    assert_eq!(
        encoded.len(),
        3,
        "u16(251) must encode as 3 bytes (crosses varint threshold)"
    );
}

#[test]
fn test_u16_max_roundtrip() {
    let original: u16 = 65535;
    let encoded = encode_to_vec(&original).expect("encode u16 65535 failed");
    let (decoded, consumed): (u16, _) =
        decode_from_slice(&encoded).expect("decode u16 65535 failed");
    assert_eq!(decoded, 65535u16);
    assert_eq!(consumed, encoded.len());
    assert_eq!(
        encoded.len(),
        3,
        "u16::MAX must encode as 3 bytes (fits in 3-byte varint range)"
    );
}

#[test]
fn test_u32_min_roundtrip() {
    let original: u32 = 0;
    let encoded = encode_to_vec(&original).expect("encode u32 0 failed");
    let (decoded, consumed): (u32, _) = decode_from_slice(&encoded).expect("decode u32 0 failed");
    assert_eq!(decoded, 0u32);
    assert_eq!(consumed, encoded.len());
}

#[test]
fn test_u32_max_roundtrip() {
    let original: u32 = u32::MAX;
    let encoded = encode_to_vec(&original).expect("encode u32::MAX failed");
    let (decoded, consumed): (u32, _) =
        decode_from_slice(&encoded).expect("decode u32::MAX failed");
    assert_eq!(decoded, u32::MAX);
    assert_eq!(consumed, encoded.len());
    assert_eq!(
        encoded.len(),
        5,
        "u32::MAX must encode as 5 bytes (exceeds u16 range)"
    );
}

#[test]
fn test_u64_min_roundtrip() {
    let original: u64 = 0;
    let encoded = encode_to_vec(&original).expect("encode u64 0 failed");
    let (decoded, consumed): (u64, _) = decode_from_slice(&encoded).expect("decode u64 0 failed");
    assert_eq!(decoded, 0u64);
    assert_eq!(consumed, encoded.len());
}

#[test]
fn test_u64_max_roundtrip() {
    let original: u64 = u64::MAX;
    let encoded = encode_to_vec(&original).expect("encode u64::MAX failed");
    let (decoded, consumed): (u64, _) =
        decode_from_slice(&encoded).expect("decode u64::MAX failed");
    assert_eq!(decoded, u64::MAX);
    assert_eq!(consumed, encoded.len());
    assert_eq!(
        encoded.len(),
        9,
        "u64::MAX must encode as 9 bytes (exceeds u32 range)"
    );
}

// ── Varint signed boundary tests (zigzag encoding) ───────────────────────────

#[test]
fn test_i8_min_roundtrip() {
    // zigzag(i8::MIN = -128) = 2*128 - 1 = 255 > 250 → 3-byte varint
    let original: i8 = i8::MIN;
    let encoded = encode_to_vec(&original).expect("encode i8::MIN failed");
    let (decoded, consumed): (i8, _) = decode_from_slice(&encoded).expect("decode i8::MIN failed");
    assert_eq!(decoded, i8::MIN);
    assert_eq!(consumed, encoded.len());
    assert_eq!(
        encoded.len(),
        1,
        "i8::MIN must encode as 1 byte (i8 is always written as a raw byte, no varint)"
    );
}

#[test]
fn test_i8_max_roundtrip() {
    // zigzag(i8::MAX = 127) = 2*127 = 254 > 250 → 3-byte varint
    let original: i8 = i8::MAX;
    let encoded = encode_to_vec(&original).expect("encode i8::MAX failed");
    let (decoded, consumed): (i8, _) = decode_from_slice(&encoded).expect("decode i8::MAX failed");
    assert_eq!(decoded, i8::MAX);
    assert_eq!(consumed, encoded.len());
    assert_eq!(
        encoded.len(),
        1,
        "i8::MAX must encode as 1 byte (i8 is always written as a raw byte, no varint)"
    );
}

#[test]
fn test_i8_neg1_roundtrip() {
    // zigzag(-1) = 2*1 - 1 = 1 → 1-byte varint
    let original: i8 = -1;
    let encoded = encode_to_vec(&original).expect("encode i8(-1) failed");
    let (decoded, consumed): (i8, _) = decode_from_slice(&encoded).expect("decode i8(-1) failed");
    assert_eq!(decoded, -1i8);
    assert_eq!(consumed, encoded.len());
    assert_eq!(encoded.len(), 1, "i8(-1) zigzag→1 must encode as 1 byte");
}

#[test]
fn test_i16_min_roundtrip() {
    let original: i16 = i16::MIN;
    let encoded = encode_to_vec(&original).expect("encode i16::MIN failed");
    let (decoded, consumed): (i16, _) =
        decode_from_slice(&encoded).expect("decode i16::MIN failed");
    assert_eq!(decoded, i16::MIN);
    assert_eq!(consumed, encoded.len());
}

#[test]
fn test_i32_min_roundtrip() {
    let original: i32 = i32::MIN;
    let encoded = encode_to_vec(&original).expect("encode i32::MIN failed");
    let (decoded, consumed): (i32, _) =
        decode_from_slice(&encoded).expect("decode i32::MIN failed");
    assert_eq!(decoded, i32::MIN);
    assert_eq!(consumed, encoded.len());
}

#[test]
fn test_i32_max_roundtrip() {
    let original: i32 = i32::MAX;
    let encoded = encode_to_vec(&original).expect("encode i32::MAX failed");
    let (decoded, consumed): (i32, _) =
        decode_from_slice(&encoded).expect("decode i32::MAX failed");
    assert_eq!(decoded, i32::MAX);
    assert_eq!(consumed, encoded.len());
}

#[test]
fn test_i64_min_roundtrip() {
    let original: i64 = i64::MIN;
    let encoded = encode_to_vec(&original).expect("encode i64::MIN failed");
    let (decoded, consumed): (i64, _) =
        decode_from_slice(&encoded).expect("decode i64::MIN failed");
    assert_eq!(decoded, i64::MIN);
    assert_eq!(consumed, encoded.len());
}

#[test]
fn test_i64_max_roundtrip() {
    let original: i64 = i64::MAX;
    let encoded = encode_to_vec(&original).expect("encode i64::MAX failed");
    let (decoded, consumed): (i64, _) =
        decode_from_slice(&encoded).expect("decode i64::MAX failed");
    assert_eq!(decoded, i64::MAX);
    assert_eq!(consumed, encoded.len());
}

// ── Fixed-int encoding tests (exact byte-width, no varint) ───────────────────

#[test]
fn test_u32_fixed_int_encoding_size() {
    let cfg = config::standard().with_fixed_int_encoding();
    let encoded = encode_to_vec_with_config(&42u32, cfg).expect("encode u32 fixed-int failed");
    assert_eq!(
        encoded.len(),
        4,
        "u32 with fixed-int encoding must be exactly 4 bytes"
    );
    let (decoded, _): (u32, _) =
        decode_from_slice_with_config(&encoded, cfg).expect("decode u32 fixed-int failed");
    assert_eq!(decoded, 42u32);
}

#[test]
fn test_i64_neg1_fixed_int_encoding_size() {
    let cfg = config::standard().with_fixed_int_encoding();
    let encoded =
        encode_to_vec_with_config(&(-1i64), cfg).expect("encode i64(-1) fixed-int failed");
    assert_eq!(
        encoded.len(),
        8,
        "i64 with fixed-int encoding must be exactly 8 bytes"
    );
    let (decoded, _): (i64, _) =
        decode_from_slice_with_config(&encoded, cfg).expect("decode i64(-1) fixed-int failed");
    assert_eq!(decoded, -1i64);
}
