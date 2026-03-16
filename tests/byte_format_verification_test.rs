//! Comprehensive byte-level format verification tests for OxiCode.
//!
//! Covers varint encoding thresholds, zigzag encoding for i64,
//! char UTF-8 encoding, and various collection/config edge cases.
//! Complements encoding_format_test.rs without duplication.
//!
//! Varint encoding rules (unsigned):
//!   0–250       → 1 byte  [value]
//!   251–65535   → 3 bytes [0xFB, lo, hi]  (LE u16)
//!   65536–2^32-1→ 5 bytes [0xFC, b0, b1, b2, b3] (LE u32)
//!   2^32+       → 9 bytes [0xFD, b0..b7] (LE u64)
//!
//! Zigzag encoding (i64 → u64):
//!   n >= 0 → 2*n
//!   n <  0 → -2*n - 1
//!
//! char encoding: UTF-8 bytes (not fixed u32).

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
use oxicode::{config, encode_to_vec, encode_to_vec_with_config};

// ---------------------------------------------------------------------------
// Test 1: u8(0) → [0x00]
// ---------------------------------------------------------------------------
#[test]
fn test_u8_zero_single_byte() {
    let bytes = encode_to_vec(&0u8).expect("encode u8 0");
    assert_eq!(bytes, &[0x00]);
}

// ---------------------------------------------------------------------------
// Test 2: u8(250) → [0xFA]  (max single-byte varint)
// ---------------------------------------------------------------------------
#[test]
fn test_u8_250_single_byte() {
    let bytes = encode_to_vec(&250u8).expect("encode u8 250");
    assert_eq!(bytes, &[0xFA]);
}

// ---------------------------------------------------------------------------
// Test 3: u8(251) → [0xFB]
//   u8 is always encoded as exactly 1 raw byte (not varint expanded)
// ---------------------------------------------------------------------------
#[test]
fn test_u8_251_single_byte() {
    let bytes = encode_to_vec(&251u8).expect("encode u8 251");
    assert_eq!(bytes, &[0xFB]);
}

// ---------------------------------------------------------------------------
// Test 4: u8(255) → [0xFF]
//   u8 is always encoded as exactly 1 raw byte
// ---------------------------------------------------------------------------
#[test]
fn test_u8_255_single_byte() {
    let bytes = encode_to_vec(&255u8).expect("encode u8 255");
    assert_eq!(bytes, &[0xFF]);
}

// ---------------------------------------------------------------------------
// Test 5: u16(256) → [0xFB, 0x00, 0x01]
//   256 = 0x0100; LE u16 = [0x00, 0x01]
// ---------------------------------------------------------------------------
#[test]
fn test_u16_256_three_byte_varint() {
    let bytes = encode_to_vec(&256u16).expect("encode u16 256");
    assert_eq!(bytes, &[0xFB, 0x00, 0x01]);
}

// ---------------------------------------------------------------------------
// Test 6: u16(1000) → [0xFB, 0xE8, 0x03]
//   1000 = 0x03E8; LE u16 = [0xE8, 0x03]
// ---------------------------------------------------------------------------
#[test]
fn test_u16_1000_three_byte_varint() {
    let bytes = encode_to_vec(&1000u16).expect("encode u16 1000");
    assert_eq!(bytes, &[0xFB, 0xE8, 0x03]);
}

// ---------------------------------------------------------------------------
// Test 7: u16(65535) → [0xFB, 0xFF, 0xFF]
//   65535 = 0xFFFF; LE u16 = [0xFF, 0xFF]
// ---------------------------------------------------------------------------
#[test]
fn test_u16_65535_three_byte_varint() {
    let bytes = encode_to_vec(&65535u16).expect("encode u16 65535");
    assert_eq!(bytes, &[0xFB, 0xFF, 0xFF]);
}

// ---------------------------------------------------------------------------
// Test 8: u32(65536) → [0xFC, 0x00, 0x00, 0x01, 0x00]
//   65536 = 0x0001_0000; 5-byte varint: marker 0xFC, LE u32 = [0x00, 0x00, 0x01, 0x00]
// ---------------------------------------------------------------------------
#[test]
fn test_u32_65536_five_byte_varint() {
    let bytes = encode_to_vec(&65536u32).expect("encode u32 65536");
    assert_eq!(bytes, &[0xFC, 0x00, 0x00, 0x01, 0x00]);
}

// ---------------------------------------------------------------------------
// Test 9: u64(0xFFFF_FFFF + 1) → [0xFD, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00]
//   4294967296 = 0x1_0000_0000; 9-byte varint: marker 0xFD, LE u64
//   LE u64(4294967296) = [0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00]
// ---------------------------------------------------------------------------
#[test]
fn test_u64_4294967296_nine_byte_varint() {
    let val: u64 = 0xFFFF_FFFF_u64 + 1;
    let bytes = encode_to_vec(&val).expect("encode u64 4294967296");
    assert_eq!(
        bytes,
        &[0xFD, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00]
    );
}

// ---------------------------------------------------------------------------
// Test 10: i64(0) → [0x00]
//   zigzag(0) = 2*0 = 0 → varint [0x00]
// ---------------------------------------------------------------------------
#[test]
fn test_i64_zero_zigzag() {
    let bytes = encode_to_vec(&0i64).expect("encode i64 0");
    assert_eq!(bytes, &[0x00]);
}

// ---------------------------------------------------------------------------
// Test 11: i64(1) → [0x02]
//   zigzag(1) = 2*1 = 2 → varint [0x02]
// ---------------------------------------------------------------------------
#[test]
fn test_i64_pos1_zigzag() {
    let bytes = encode_to_vec(&1i64).expect("encode i64 1");
    assert_eq!(bytes, &[0x02]);
}

// ---------------------------------------------------------------------------
// Test 12: i64(-1) → [0x01]
//   zigzag(-1) = -2*(-1)-1 = 1 → varint [0x01]
// ---------------------------------------------------------------------------
#[test]
fn test_i64_neg1_zigzag() {
    let bytes = encode_to_vec(&(-1i64)).expect("encode i64 -1");
    assert_eq!(bytes, &[0x01]);
}

// ---------------------------------------------------------------------------
// Test 13: i64(63) → [0x7E]
//   zigzag(63) = 2*63 = 126 = 0x7E → single byte varint
// ---------------------------------------------------------------------------
#[test]
fn test_i64_63_zigzag_single_byte() {
    let bytes = encode_to_vec(&63i64).expect("encode i64 63");
    assert_eq!(bytes, &[0x7E]);
}

// ---------------------------------------------------------------------------
// Test 14: i64(-64) → [0x7F]
//   zigzag(-64) = -2*(-64)-1 = 127 = 0x7F → single byte varint (≤ 250)
// ---------------------------------------------------------------------------
#[test]
fn test_i64_neg64_zigzag_single_byte() {
    let bytes = encode_to_vec(&(-64i64)).expect("encode i64 -64");
    assert_eq!(bytes, &[0x7F]);
}

// ---------------------------------------------------------------------------
// Test 15: i64(125) → [0xFA]
//   zigzag(125) = 2*125 = 250 = 0xFA → single byte varint (= 250, max single byte)
// ---------------------------------------------------------------------------
#[test]
fn test_i64_125_zigzag_single_byte_max() {
    let bytes = encode_to_vec(&125i64).expect("encode i64 125");
    assert_eq!(bytes, &[0xFA]);
}

// ---------------------------------------------------------------------------
// Test 16: i64(-126) → [0xFB, 0xF9, 0x00]
//   zigzag(-126) = -2*(-126)-1 = 251 → 3-byte varint [0xFB, 0xFB, 0x00]
//   Wait: zigzag(-126) = 2*126-1... let's compute carefully:
//   formula: n<0 → -2n-1 = -(2*(-126))-1 = 252-1 = 251
//   251 in LE u16 = [0xFB, 0x00]
//   So encoding: [0xFB, 0xFB, 0x00]
// ---------------------------------------------------------------------------
#[test]
fn test_i64_neg126_zigzag_three_byte() {
    let bytes = encode_to_vec(&(-126i64)).expect("encode i64 -126");
    // zigzag(-126) = -2*(-126) - 1 = 252 - 1 = 251
    // 251 > 250 → 3-byte varint: [0xFB, 251_lo, 251_hi] = [0xFB, 0xFB, 0x00]
    assert_eq!(bytes, &[0xFB, 0xFB, 0x00]);
}

// ---------------------------------------------------------------------------
// Test 17: bool(true) → [0x01], bool(false) → [0x00]
// ---------------------------------------------------------------------------
#[test]
fn test_bool_encoding() {
    let true_bytes = encode_to_vec(&true).expect("encode true");
    let false_bytes = encode_to_vec(&false).expect("encode false");
    assert_eq!(true_bytes, &[0x01]);
    assert_eq!(false_bytes, &[0x00]);
}

// ---------------------------------------------------------------------------
// Test 18: char('A') → [0x41]
//   'A' has codepoint 0x41 (65), which is < 0x80, so it encodes as 1-byte UTF-8: [0x41]
//   (NOT 4 bytes; oxicode encodes char as UTF-8, not fixed u32)
// ---------------------------------------------------------------------------
#[test]
fn test_char_ascii_utf8_encoding() {
    let bytes = encode_to_vec(&'A').expect("encode char 'A'");
    assert_eq!(bytes, &[0x41]);
}

// ---------------------------------------------------------------------------
// Test 19: String "hi" → [0x02, 0x68, 0x69]
//   varint length 2, then UTF-8 bytes: 'h'=0x68, 'i'=0x69
// ---------------------------------------------------------------------------
#[test]
fn test_string_hi_encoding() {
    let bytes = encode_to_vec(&"hi").expect("encode &str 'hi'");
    assert_eq!(bytes, &[0x02, 0x68, 0x69]);
}

// ---------------------------------------------------------------------------
// Test 20: Vec::<u8>::new() (empty) → [0x00]
//   varint length 0, no payload
// ---------------------------------------------------------------------------
#[test]
fn test_vec_u8_empty_varint_zero() {
    let v: Vec<u8> = Vec::new();
    let bytes = encode_to_vec(&v).expect("encode Vec<u8> empty");
    assert_eq!(bytes, &[0x00]);
}

// ---------------------------------------------------------------------------
// Test 21: Option::<u32>::None → [0x00], Option::<u32>::Some(42) → [0x01, 0x2A]
//   None: tag 0
//   Some(42): tag 1, then varint(42) = [0x2A]
// ---------------------------------------------------------------------------
#[test]
fn test_option_none_and_some_42() {
    let none_bytes = encode_to_vec(&Option::<u32>::None).expect("encode None");
    let some_bytes = encode_to_vec(&Some(42u32)).expect("encode Some(42)");
    assert_eq!(none_bytes, &[0x00]);
    assert_eq!(some_bytes, &[0x01, 0x2A]);
}

// ---------------------------------------------------------------------------
// Test 22: u32(1) with with_fixed_int_encoding() → [0x01, 0x00, 0x00, 0x00]
//   Fixed 4-byte LE representation, no varint compression
// ---------------------------------------------------------------------------
#[test]
fn test_fixed_int_u32_one_le() {
    let cfg = config::standard().with_fixed_int_encoding();
    let bytes = encode_to_vec_with_config(&1u32, cfg).expect("encode u32 1 fixed_int");
    assert_eq!(bytes, &[0x01, 0x00, 0x00, 0x00]);
}
