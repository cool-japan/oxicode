//! Advanced wire format tests — verifies exact byte sequences produced by OxiCode.
//!
//! Wire format reference (standard config: LittleEndian + Varint):
//! - u8/i8:  1 raw byte (no varint)
//! - u16/u32/u64/usize (varint): 0–250 → 1 byte; 251–65535 → [0xFB, lo, hi] LE;
//!   65536–u32::MAX → [0xFC, b0, b1, b2, b3] LE; >u32::MAX → [0xFD, 8 bytes]
//! - i16/i32/i64/isize (varint): zigzag-encoded then unsigned varint
//!   zigzag(n) = (n << 1) ^ (n >> BITS-1)
//! - bool: 1 byte via u8 (0 = false, 1 = true)
//! - Option<T>: None → 0u8 [0x00]; Some(v) → 1u8 then v's bytes
//! - Vec<T>: u64 varint length, then each element
//! - String: u64 varint byte-length, then UTF-8 bytes

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

// ===== u8 tests =====

#[test]
fn test_wire_format_u8_zero() {
    // u8 is stored as a raw single byte — no varint
    let encoded = encode_to_vec(&0u8).expect("encode u8(0) failed");
    assert_eq!(encoded, vec![0x00u8], "u8(0) should encode as [0x00]");
    let (decoded, consumed): (u8, _) = decode_from_slice(&encoded).expect("decode u8(0) failed");
    assert_eq!(decoded, 0u8);
    assert_eq!(consumed, 1);
}

#[test]
fn test_wire_format_u8_max() {
    // u8(255) = 0xFF — raw byte regardless of SINGLE_BYTE_MAX
    let encoded = encode_to_vec(&255u8).expect("encode u8(255) failed");
    assert_eq!(encoded, vec![0xFFu8], "u8(255) should encode as [0xFF]");
    let (decoded, consumed): (u8, _) = decode_from_slice(&encoded).expect("decode u8(255) failed");
    assert_eq!(decoded, 255u8);
    assert_eq!(consumed, 1);
}

// ===== u16 varint tests =====

#[test]
fn test_wire_format_u16_zero() {
    // u16(0) via varint: 0 ≤ 250, single byte [0x00]
    let encoded = encode_to_vec(&0u16).expect("encode u16(0) failed");
    assert_eq!(
        encoded,
        vec![0x00u8],
        "u16(0) should varint-encode as [0x00]"
    );
    let (decoded, consumed): (u16, _) = decode_from_slice(&encoded).expect("decode u16(0) failed");
    assert_eq!(decoded, 0u16);
    assert_eq!(consumed, 1);
}

#[test]
fn test_wire_format_u16_250() {
    // u16(250) = 0xFA: exactly at SINGLE_BYTE_MAX, single byte [0xFA]
    let encoded = encode_to_vec(&250u16).expect("encode u16(250) failed");
    assert_eq!(
        encoded,
        vec![0xFAu8],
        "u16(250) should varint-encode as [0xFA]"
    );
    let (decoded, consumed): (u16, _) =
        decode_from_slice(&encoded).expect("decode u16(250) failed");
    assert_eq!(decoded, 250u16);
    assert_eq!(consumed, 1);
}

#[test]
fn test_wire_format_u16_251() {
    // u16(251) > SINGLE_BYTE_MAX(250): multi-byte encoding
    // Format: [U16_BYTE=0xFB, lo=0xFB, hi=0x00] (251 in little-endian u16)
    let encoded = encode_to_vec(&251u16).expect("encode u16(251) failed");
    assert_eq!(
        encoded,
        vec![0xFBu8, 0xFBu8, 0x00u8],
        "u16(251) should encode as [0xFB, 0xFB, 0x00]"
    );
    let (decoded, consumed): (u16, _) =
        decode_from_slice(&encoded).expect("decode u16(251) failed");
    assert_eq!(decoded, 251u16);
    assert_eq!(consumed, 3);
}

#[test]
fn test_wire_format_u16_max() {
    // u16(65535) = 0xFFFF: [U16_BYTE=0xFB, 0xFF, 0xFF] (LE)
    let encoded = encode_to_vec(&65535u16).expect("encode u16(65535) failed");
    assert_eq!(
        encoded,
        vec![0xFBu8, 0xFFu8, 0xFFu8],
        "u16(65535) should encode as [0xFB, 0xFF, 0xFF]"
    );
    let (decoded, consumed): (u16, _) =
        decode_from_slice(&encoded).expect("decode u16(65535) failed");
    assert_eq!(decoded, 65535u16);
    assert_eq!(consumed, 3);
}

// ===== u32 varint tests =====

#[test]
fn test_wire_format_u32_zero() {
    // u32(0) via varint: single byte [0x00]
    let encoded = encode_to_vec(&0u32).expect("encode u32(0) failed");
    assert_eq!(
        encoded,
        vec![0x00u8],
        "u32(0) should varint-encode as [0x00]"
    );
    let (decoded, consumed): (u32, _) = decode_from_slice(&encoded).expect("decode u32(0) failed");
    assert_eq!(decoded, 0u32);
    assert_eq!(consumed, 1);
}

#[test]
fn test_wire_format_u32_65536() {
    // u32(65536) > 65535: uses U32_BYTE prefix
    // 65536 = 0x0001_0000, LE bytes = [0x00, 0x00, 0x01, 0x00]
    // Format: [U32_BYTE=0xFC, 0x00, 0x00, 0x01, 0x00]
    let encoded = encode_to_vec(&65536u32).expect("encode u32(65536) failed");
    assert_eq!(
        encoded,
        vec![0xFCu8, 0x00u8, 0x00u8, 0x01u8, 0x00u8],
        "u32(65536) should encode as [0xFC, 0x00, 0x00, 0x01, 0x00]"
    );
    let (decoded, consumed): (u32, _) =
        decode_from_slice(&encoded).expect("decode u32(65536) failed");
    assert_eq!(decoded, 65536u32);
    assert_eq!(consumed, 5);
}

// ===== bool tests =====

#[test]
fn test_wire_format_bool_false() {
    // bool encodes as u8: false → 0u8 → [0x00]
    let encoded = encode_to_vec(&false).expect("encode bool(false) failed");
    assert_eq!(encoded, vec![0x00u8], "bool(false) should encode as [0x00]");
    let (decoded, consumed): (bool, _) =
        decode_from_slice(&encoded).expect("decode bool(false) failed");
    assert!(!decoded);
    assert_eq!(consumed, 1);
}

#[test]
fn test_wire_format_bool_true() {
    // bool encodes as u8: true → 1u8 → [0x01]
    let encoded = encode_to_vec(&true).expect("encode bool(true) failed");
    assert_eq!(encoded, vec![0x01u8], "bool(true) should encode as [0x01]");
    let (decoded, consumed): (bool, _) =
        decode_from_slice(&encoded).expect("decode bool(true) failed");
    assert!(decoded);
    assert_eq!(consumed, 1);
}

// ===== Option tests =====

#[test]
fn test_wire_format_option_none() {
    // None<u32> encodes as 0u8 → [0x00]
    let value: Option<u32> = None;
    let encoded = encode_to_vec(&value).expect("encode None failed");
    assert_eq!(encoded, vec![0x00u8], "None should encode as [0x00]");
    let (decoded, consumed): (Option<u32>, _) =
        decode_from_slice(&encoded).expect("decode None failed");
    assert_eq!(decoded, None);
    assert_eq!(consumed, 1);
}

#[test]
fn test_wire_format_option_some_zero() {
    // Some(0u32): 1u8 tag then varint(0) → [0x01, 0x00]
    let value: Option<u32> = Some(0u32);
    let encoded = encode_to_vec(&value).expect("encode Some(0u32) failed");
    assert_eq!(
        encoded,
        vec![0x01u8, 0x00u8],
        "Some(0u32) should encode as [0x01, 0x00]"
    );
    let (decoded, consumed): (Option<u32>, _) =
        decode_from_slice(&encoded).expect("decode Some(0u32) failed");
    assert_eq!(decoded, Some(0u32));
    assert_eq!(consumed, 2);
}

#[test]
fn test_wire_format_option_some_one() {
    // Some(1u32): 1u8 tag then varint(1) → [0x01, 0x01]
    let value: Option<u32> = Some(1u32);
    let encoded = encode_to_vec(&value).expect("encode Some(1u32) failed");
    assert_eq!(
        encoded,
        vec![0x01u8, 0x01u8],
        "Some(1u32) should encode as [0x01, 0x01]"
    );
    let (decoded, consumed): (Option<u32>, _) =
        decode_from_slice(&encoded).expect("decode Some(1u32) failed");
    assert_eq!(decoded, Some(1u32));
    assert_eq!(consumed, 2);
}

// ===== String test =====

#[test]
fn test_wire_format_string_hi() {
    // "hi": byte_len=2 (varint → [0x02]), then 'h'=0x68, 'i'=0x69
    let encoded = encode_to_vec(&String::from("hi")).expect("encode 'hi' failed");
    assert_eq!(
        encoded,
        vec![0x02u8, 0x68u8, 0x69u8],
        "'hi' should encode as [0x02, 0x68, 0x69]"
    );
    let (decoded, consumed): (String, _) = decode_from_slice(&encoded).expect("decode 'hi' failed");
    assert_eq!(decoded, "hi");
    assert_eq!(consumed, 3);
}

// ===== Vec<u8> tests =====

#[test]
fn test_wire_format_vec_u8_empty() {
    // Vec<u8>[] → varint(0) → [0x00]
    let value: Vec<u8> = vec![];
    let encoded = encode_to_vec(&value).expect("encode empty Vec<u8> failed");
    assert_eq!(
        encoded,
        vec![0x00u8],
        "empty Vec<u8> should encode as [0x00]"
    );
    let (decoded, consumed): (Vec<u8>, _) =
        decode_from_slice(&encoded).expect("decode empty Vec<u8> failed");
    assert_eq!(decoded, Vec::<u8>::new());
    assert_eq!(consumed, 1);
}

#[test]
fn test_wire_format_vec_u8_three_elements() {
    // Vec<u8>[1,2,3]: varint(3) = [0x03], then raw bytes [0x01, 0x02, 0x03]
    let value: Vec<u8> = vec![1u8, 2u8, 3u8];
    let encoded = encode_to_vec(&value).expect("encode Vec<u8>[1,2,3] failed");
    assert_eq!(
        encoded,
        vec![0x03u8, 0x01u8, 0x02u8, 0x03u8],
        "Vec<u8>[1,2,3] should encode as [0x03, 0x01, 0x02, 0x03]"
    );
    let (decoded, consumed): (Vec<u8>, _) =
        decode_from_slice(&encoded).expect("decode Vec<u8>[1,2,3] failed");
    assert_eq!(decoded, vec![1u8, 2u8, 3u8]);
    assert_eq!(consumed, 4);
}

// ===== i8 tests =====

#[test]
fn test_wire_format_i8_neg_one() {
    // i8(-1) stored as raw byte: -1i8 as u8 = 255 = 0xFF
    let encoded = encode_to_vec(&(-1i8)).expect("encode i8(-1) failed");
    assert_eq!(
        encoded,
        vec![0xFFu8],
        "i8(-1) should encode as raw byte [0xFF] (two's complement)"
    );
    let (decoded, consumed): (i8, _) = decode_from_slice(&encoded).expect("decode i8(-1) failed");
    assert_eq!(decoded, -1i8);
    assert_eq!(consumed, 1);
}

#[test]
fn test_wire_format_i8_max() {
    // i8(127) = 0x7F raw byte
    let encoded = encode_to_vec(&127i8).expect("encode i8(127) failed");
    assert_eq!(encoded, vec![0x7Fu8], "i8(127) should encode as [0x7F]");
    let (decoded, consumed): (i8, _) = decode_from_slice(&encoded).expect("decode i8(127) failed");
    assert_eq!(decoded, 127i8);
    assert_eq!(consumed, 1);
}

#[test]
fn test_wire_format_i8_min() {
    // i8(-128) = -128i8 as u8 = 0x80 raw byte
    let encoded = encode_to_vec(&(-128i8)).expect("encode i8(-128) failed");
    assert_eq!(encoded, vec![0x80u8], "i8(-128) should encode as [0x80]");
    let (decoded, consumed): (i8, _) = decode_from_slice(&encoded).expect("decode i8(-128) failed");
    assert_eq!(decoded, -128i8);
    assert_eq!(consumed, 1);
}

// ===== i16/i32 zigzag+varint tests =====

#[test]
fn test_wire_format_i16_neg_one() {
    // i16(-1): zigzag = ((-1i16 as u16).wrapping_shl(1)) ^ ((-1i16 >> 15) as u16)
    //        = 0xFFFE ^ 0xFFFF = 1
    // varint(1) = [0x01]
    let encoded = encode_to_vec(&(-1i16)).expect("encode i16(-1) failed");
    assert_eq!(
        encoded,
        vec![0x01u8],
        "i16(-1) zigzag=1 should varint-encode as [0x01]"
    );
    let (decoded, consumed): (i16, _) = decode_from_slice(&encoded).expect("decode i16(-1) failed");
    assert_eq!(decoded, -1i16);
    assert_eq!(consumed, 1);
}

#[test]
fn test_wire_format_i32_pos_one() {
    // i32(1): zigzag = ((1u32).wrapping_shl(1)) ^ ((1i32 >> 31) as u32)
    //       = 2 ^ 0 = 2
    // varint(2) = [0x02]
    let encoded = encode_to_vec(&1i32).expect("encode i32(1) failed");
    assert_eq!(
        encoded,
        vec![0x02u8],
        "i32(1) zigzag=2 should varint-encode as [0x02]"
    );
    let (decoded, consumed): (i32, _) = decode_from_slice(&encoded).expect("decode i32(1) failed");
    assert_eq!(decoded, 1i32);
    assert_eq!(consumed, 1);
}

#[test]
fn test_wire_format_i32_neg_one() {
    // i32(-1): zigzag = ((-1i32 as u32).wrapping_shl(1)) ^ ((-1i32 >> 31) as u32)
    //        = 0xFFFFFFFE ^ 0xFFFFFFFF = 1
    // varint(1) = [0x01]
    let encoded = encode_to_vec(&(-1i32)).expect("encode i32(-1) failed");
    assert_eq!(
        encoded,
        vec![0x01u8],
        "i32(-1) zigzag=1 should varint-encode as [0x01]"
    );
    let (decoded, consumed): (i32, _) = decode_from_slice(&encoded).expect("decode i32(-1) failed");
    assert_eq!(decoded, -1i32);
    assert_eq!(consumed, 1);
}
