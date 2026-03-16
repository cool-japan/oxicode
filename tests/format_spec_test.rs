//! Binary wire format specification tests for OxiCode.
//!
//! Each test verifies the exact byte layout produced by the encoder.
//! These tests act as a living specification of the OxiCode binary format.
//!
//! Varint encoding rules (unsigned integers, little-endian):
//!   0–250         → 1 byte  [value]
//!   251–65535     → 3 bytes [0xFB, lo, hi]  (LE u16)
//!   65536–2^32-1  → 5 bytes [0xFC, b0..b3]  (LE u32)
//!   2^32–2^64-1   → 9 bytes [0xFD, b0..b7]  (LE u64)
//!   2^64–2^128-1  → 17 bytes [0xFE, b0..b15] (LE u128)
//!
//! Signed integers use zigzag encoding before the varint:
//!   zigzag(n) = (n << 1) ^ (n >> (BITS-1))
//!   e.g. -1 → 1, -64 → 127
//!
//! char encodes as raw UTF-8 bytes (1–4 bytes, no length prefix).
//! String/Vec encode as varint length prefix followed by elements.
//! Option encodes as 0x00 (None) or 0x01 followed by the encoded value (Some).
//! Fixed-int config encodes integers directly in their native byte size (LE or BE).

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
use oxicode::{config, decode_from_slice, encode_to_vec};

// ---------------------------------------------------------------------------
// Test 1: 0u8 encodes as [0x00]
// ---------------------------------------------------------------------------
#[test]
fn wire_u8_zero_is_single_zero_byte() {
    let bytes = encode_to_vec(&0u8).expect("encode 0u8");
    assert_eq!(bytes, &[0x00], "0u8 must encode as [0x00]");
    let (val, _): (u8, _) = decode_from_slice(&bytes).expect("decode 0u8");
    assert_eq!(val, 0u8);
}

// ---------------------------------------------------------------------------
// Test 2: 255u8 encodes as [0xFF]
// ---------------------------------------------------------------------------
#[test]
fn wire_u8_max_is_0xff() {
    let bytes = encode_to_vec(&255u8).expect("encode 255u8");
    assert_eq!(bytes, &[0xFF], "255u8 must encode as [0xFF]");
    let (val, _): (u8, _) = decode_from_slice(&bytes).expect("decode 255u8");
    assert_eq!(val, 255u8);
}

// ---------------------------------------------------------------------------
// Test 3: true encodes as [0x01], false encodes as [0x00]
// ---------------------------------------------------------------------------
#[test]
fn wire_bool_true_is_0x01_false_is_0x00() {
    let true_bytes = encode_to_vec(&true).expect("encode true");
    assert_eq!(true_bytes, &[0x01], "true must encode as [0x01]");

    let false_bytes = encode_to_vec(&false).expect("encode false");
    assert_eq!(false_bytes, &[0x00], "false must encode as [0x00]");

    let (t, _): (bool, _) = decode_from_slice(&true_bytes).expect("decode true");
    let (f, _): (bool, _) = decode_from_slice(&false_bytes).expect("decode false");
    assert!(t, "decoded true must be true");
    assert!(!f, "decoded false must be false");
}

// ---------------------------------------------------------------------------
// Test 4: 0u32 encodes as [0x00] (1-byte varint)
// ---------------------------------------------------------------------------
#[test]
fn wire_u32_zero_is_single_byte_varint() {
    let bytes = encode_to_vec(&0u32).expect("encode 0u32");
    assert_eq!(bytes, &[0x00], "0u32 must encode as 1-byte varint [0x00]");
    let (val, _): (u32, _) = decode_from_slice(&bytes).expect("decode 0u32");
    assert_eq!(val, 0u32);
}

// ---------------------------------------------------------------------------
// Test 5: 250u32 encodes as [0xFA] (1-byte varint, max single-byte value)
// ---------------------------------------------------------------------------
#[test]
fn wire_u32_250_is_single_byte_varint_max() {
    let bytes = encode_to_vec(&250u32).expect("encode 250u32");
    assert_eq!(
        bytes,
        &[0xFA],
        "250u32 must encode as 1-byte varint [0xFA] (SINGLE_BYTE_MAX)"
    );
    let (val, _): (u32, _) = decode_from_slice(&bytes).expect("decode 250u32");
    assert_eq!(val, 250u32);
}

// ---------------------------------------------------------------------------
// Test 6: 251u32 encodes as [0xFB, 0xFB, 0x00] (3-byte varint: marker + LE u16)
// ---------------------------------------------------------------------------
#[test]
fn wire_u32_251_is_three_byte_varint() {
    // 251 > SINGLE_BYTE_MAX(250), fits in u16.
    // Marker 0xFB followed by 251u16 in little-endian: [0xFB, 0x00]
    let bytes = encode_to_vec(&251u32).expect("encode 251u32");
    assert_eq!(
        bytes,
        &[0xFB, 0xFB, 0x00],
        "251u32 must encode as [0xFB, 0xFB, 0x00] (marker + LE u16)"
    );
    let (val, _): (u32, _) = decode_from_slice(&bytes).expect("decode 251u32");
    assert_eq!(val, 251u32);
}

// ---------------------------------------------------------------------------
// Test 7: 65535u32 encodes as [0xFB, 0xFF, 0xFF] (3-byte varint)
// ---------------------------------------------------------------------------
#[test]
fn wire_u32_65535_is_three_byte_varint() {
    // 65535 = u16::MAX; marker 0xFB + LE u16 [0xFF, 0xFF]
    let bytes = encode_to_vec(&65535u32).expect("encode 65535u32");
    assert_eq!(
        bytes,
        &[0xFB, 0xFF, 0xFF],
        "65535u32 (u16::MAX) must encode as [0xFB, 0xFF, 0xFF]"
    );
    let (val, _): (u32, _) = decode_from_slice(&bytes).expect("decode 65535u32");
    assert_eq!(val, 65535u32);
}

// ---------------------------------------------------------------------------
// Test 8: 65536u32 encodes as [0xFC, 0x00, 0x00, 0x01, 0x00] (5-byte varint)
// ---------------------------------------------------------------------------
#[test]
fn wire_u32_65536_is_five_byte_varint() {
    // 65536 > u16::MAX; marker 0xFC + LE u32
    // 65536u32.to_le_bytes() = [0x00, 0x00, 0x01, 0x00]
    let bytes = encode_to_vec(&65536u32).expect("encode 65536u32");
    assert_eq!(
        bytes,
        &[0xFC, 0x00, 0x00, 0x01, 0x00],
        "65536u32 must encode as [0xFC, 0x00, 0x00, 0x01, 0x00]"
    );
    let (val, _): (u32, _) = decode_from_slice(&bytes).expect("decode 65536u32");
    assert_eq!(val, 65536u32);
}

// ---------------------------------------------------------------------------
// Test 9: u32::MAX encodes as 5 bytes starting with [0xFC]
// ---------------------------------------------------------------------------
#[test]
fn wire_u32_max_is_five_bytes_starting_with_fc() {
    // u32::MAX = 0xFFFFFFFF; marker 0xFC + LE u32 [0xFF, 0xFF, 0xFF, 0xFF]
    let bytes = encode_to_vec(&u32::MAX).expect("encode u32::MAX");
    assert_eq!(bytes.len(), 5, "u32::MAX must encode as exactly 5 bytes");
    assert_eq!(
        bytes[0], 0xFC,
        "first byte of u32::MAX encoding must be 0xFC"
    );
    assert_eq!(
        &bytes[1..],
        &u32::MAX.to_le_bytes(),
        "bytes[1..5] must be u32::MAX in little-endian order"
    );
    let (val, _): (u32, _) = decode_from_slice(&bytes).expect("decode u32::MAX");
    assert_eq!(val, u32::MAX);
}

// ---------------------------------------------------------------------------
// Test 10: 0u64 encodes as [0x00] (1-byte varint)
// ---------------------------------------------------------------------------
#[test]
fn wire_u64_zero_is_single_byte_varint() {
    let bytes = encode_to_vec(&0u64).expect("encode 0u64");
    assert_eq!(bytes, &[0x00], "0u64 must encode as 1-byte varint [0x00]");
    let (val, _): (u64, _) = decode_from_slice(&bytes).expect("decode 0u64");
    assert_eq!(val, 0u64);
}

// ---------------------------------------------------------------------------
// Test 11: u64::MAX encodes as 9 bytes starting with [0xFD]
// ---------------------------------------------------------------------------
#[test]
fn wire_u64_max_is_nine_bytes_starting_with_fd() {
    // u64::MAX > u32::MAX; marker 0xFD + 8 LE bytes
    let bytes = encode_to_vec(&u64::MAX).expect("encode u64::MAX");
    assert_eq!(bytes.len(), 9, "u64::MAX must encode as exactly 9 bytes");
    assert_eq!(
        bytes[0], 0xFD,
        "first byte of u64::MAX encoding must be 0xFD"
    );
    assert_eq!(
        &bytes[1..],
        &u64::MAX.to_le_bytes(),
        "bytes[1..9] must be u64::MAX in little-endian order"
    );
    let (val, _): (u64, _) = decode_from_slice(&bytes).expect("decode u64::MAX");
    assert_eq!(val, u64::MAX);
}

// ---------------------------------------------------------------------------
// Test 12: u128::MAX encodes as 17 bytes starting with [0xFE]
// ---------------------------------------------------------------------------
#[test]
fn wire_u128_max_is_seventeen_bytes_starting_with_fe() {
    // u128::MAX > u64::MAX; marker 0xFE + 16 LE bytes
    let bytes = encode_to_vec(&u128::MAX).expect("encode u128::MAX");
    assert_eq!(bytes.len(), 17, "u128::MAX must encode as exactly 17 bytes");
    assert_eq!(
        bytes[0], 0xFE,
        "first byte of u128::MAX encoding must be 0xFE"
    );
    assert_eq!(
        &bytes[1..],
        &u128::MAX.to_le_bytes(),
        "bytes[1..17] must be u128::MAX in little-endian order"
    );
    let (val, _): (u128, _) = decode_from_slice(&bytes).expect("decode u128::MAX");
    assert_eq!(val, u128::MAX);
}

// ---------------------------------------------------------------------------
// Test 13: "hello" encodes as [0x05, 0x68, 0x65, 0x6C, 0x6C, 0x6F]
// ---------------------------------------------------------------------------
#[test]
fn wire_string_hello_has_length_prefix_and_utf8_bytes() {
    // String encoding: varint length (5 fits in 1 byte) followed by raw UTF-8
    let bytes = encode_to_vec(&"hello".to_string()).expect("encode hello");
    assert_eq!(
        bytes,
        &[0x05, 0x68, 0x65, 0x6C, 0x6C, 0x6F],
        r#""hello" must encode as [0x05, 'h', 'e', 'l', 'l', 'o']"#
    );
    let (val, _): (String, _) = decode_from_slice(&bytes).expect("decode hello");
    assert_eq!(val, "hello");
}

// ---------------------------------------------------------------------------
// Test 14: empty Vec::<u8>::new() encodes as [0x00] (length varint only)
// ---------------------------------------------------------------------------
#[test]
fn wire_empty_vec_u8_is_single_zero_byte() {
    let bytes = encode_to_vec(&Vec::<u8>::new()).expect("encode empty Vec<u8>");
    assert_eq!(
        bytes,
        &[0x00],
        "empty Vec<u8> must encode as [0x00] (length=0 varint, no elements)"
    );
    let (val, _): (Vec<u8>, _) = decode_from_slice(&bytes).expect("decode empty Vec<u8>");
    assert!(val.is_empty(), "decoded Vec must be empty");
}

// ---------------------------------------------------------------------------
// Test 15: vec![1u8, 2, 3] encodes as [0x03, 0x01, 0x02, 0x03]
// ---------------------------------------------------------------------------
#[test]
fn wire_vec_u8_three_elements_has_length_prefix_and_elements() {
    // Length varint 3 followed by raw bytes 1, 2, 3
    let bytes = encode_to_vec(&vec![1u8, 2u8, 3u8]).expect("encode vec![1,2,3]");
    assert_eq!(
        bytes,
        &[0x03, 0x01, 0x02, 0x03],
        "vec![1u8,2,3] must encode as [0x03, 0x01, 0x02, 0x03]"
    );
    let (val, _): (Vec<u8>, _) = decode_from_slice(&bytes).expect("decode vec![1,2,3]");
    assert_eq!(val, vec![1u8, 2, 3]);
}

// ---------------------------------------------------------------------------
// Test 16: Option::<u32>::None encodes as [0x00]
// ---------------------------------------------------------------------------
#[test]
fn wire_option_none_is_single_zero_byte() {
    let bytes = encode_to_vec(&Option::<u32>::None).expect("encode None");
    assert_eq!(
        bytes,
        &[0x00],
        "Option::None must encode as [0x00] (discriminant 0)"
    );
    let (val, _): (Option<u32>, _) = decode_from_slice(&bytes).expect("decode None");
    assert!(val.is_none(), "decoded value must be None");
}

// ---------------------------------------------------------------------------
// Test 17: Some(42u32) encodes as [0x01, 0x2A] (Some discriminant + varint 42)
// ---------------------------------------------------------------------------
#[test]
fn wire_option_some_42_is_0x01_0x2a() {
    // Some is encoded as discriminant 1 (varint [0x01]) then the value
    // 42 as a varint = [0x2A] (42 ≤ 250, single-byte)
    let bytes = encode_to_vec(&Some(42u32)).expect("encode Some(42u32)");
    assert_eq!(
        bytes,
        &[0x01, 0x2A],
        "Some(42u32) must encode as [0x01, 0x2A]"
    );
    let (val, _): (Option<u32>, _) = decode_from_slice(&bytes).expect("decode Some(42u32)");
    assert_eq!(val, Some(42u32));
}

// ---------------------------------------------------------------------------
// Test 18: fixed-int 42u32 encodes as [0x2A, 0x00, 0x00, 0x00] (LE 4 bytes)
// ---------------------------------------------------------------------------
#[test]
fn wire_fixed_int_u32_42_is_four_le_bytes() {
    let cfg = config::standard().with_fixed_int_encoding();
    let bytes =
        oxicode::encode_to_vec_with_config(&42u32, cfg).expect("encode 42u32 with fixed-int");
    assert_eq!(
        bytes,
        &[0x2A, 0x00, 0x00, 0x00],
        "fixed-int 42u32 must encode as 4 LE bytes [0x2A, 0x00, 0x00, 0x00]"
    );
    let (val, _): (u32, _) =
        oxicode::decode_from_slice_with_config(&bytes, cfg).expect("decode fixed-int 42u32");
    assert_eq!(val, 42u32);
}

// ---------------------------------------------------------------------------
// Test 19: big-endian fixed-int 42u32 encodes as [0x00, 0x00, 0x00, 0x2A]
// ---------------------------------------------------------------------------
#[test]
fn wire_big_endian_fixed_int_u32_42_is_four_be_bytes() {
    let cfg = config::standard()
        .with_big_endian()
        .with_fixed_int_encoding();
    let bytes =
        oxicode::encode_to_vec_with_config(&42u32, cfg).expect("encode 42u32 with BE fixed-int");
    assert_eq!(
        bytes,
        &[0x00, 0x00, 0x00, 0x2A],
        "BE fixed-int 42u32 must encode as [0x00, 0x00, 0x00, 0x2A]"
    );
    let (val, _): (u32, _) =
        oxicode::decode_from_slice_with_config(&bytes, cfg).expect("decode BE fixed-int 42u32");
    assert_eq!(val, 42u32);
}

// ---------------------------------------------------------------------------
// Test 20: -1i64 (zigzag) encodes as [0x01]
// ---------------------------------------------------------------------------
#[test]
fn wire_i64_minus_one_zigzag_is_0x01() {
    // Zigzag: zigzag(-1) = (-1 << 1) ^ (-1 >> 63)
    //   = 0xFFFFFFFFFFFFFFFE ^ 0xFFFFFFFFFFFFFFFF = 0x0000000000000001 = 1
    // Varint 1 ≤ 250, single byte: [0x01]
    let bytes = encode_to_vec(&(-1i64)).expect("encode -1i64");
    assert_eq!(
        bytes,
        &[0x01],
        "-1i64 zigzag encodes to unsigned 1, which must be varint [0x01]"
    );
    let (val, _): (i64, _) = decode_from_slice(&bytes).expect("decode -1i64");
    assert_eq!(val, -1i64);
}

// ---------------------------------------------------------------------------
// Test 21: -64i32 (zigzag) encodes as [0x7F]
// ---------------------------------------------------------------------------
#[test]
fn wire_i32_minus_64_zigzag_is_0x7f() {
    // Zigzag: zigzag(-64) = (-64 << 1) ^ (-64 >> 31)
    //   (-64i32 as u32).wrapping_shl(1) = 0xFFFFFF80
    //   (-64i32 >> 31) as u32            = 0xFFFFFFFF
    //   XOR                               = 0x0000007F = 127
    // Varint 127 ≤ 250, single byte: [0x7F]
    let bytes = encode_to_vec(&(-64i32)).expect("encode -64i32");
    assert_eq!(
        bytes,
        &[0x7F],
        "-64i32 zigzag encodes to unsigned 127, which must be varint [0x7F]"
    );
    let (val, _): (i32, _) = decode_from_slice(&bytes).expect("decode -64i32");
    assert_eq!(val, -64i32);
}

// ---------------------------------------------------------------------------
// Test 22: 'A' (char as UTF-8) encodes as [0x41]
// ---------------------------------------------------------------------------
#[test]
fn wire_char_ascii_a_is_utf8_byte_0x41() {
    // char encodes as raw UTF-8 bytes; 'A' = U+0041, single-byte UTF-8 = 0x41
    let bytes = encode_to_vec(&'A').expect("encode 'A'");
    assert_eq!(
        bytes,
        &[0x41],
        "'A' must encode as single UTF-8 byte [0x41]"
    );
    let (val, _): (char, _) = decode_from_slice(&bytes).expect("decode 'A'");
    assert_eq!(val, 'A');
}
