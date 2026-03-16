//! Advanced tests for fixed-size array [T; N] encoding in OxiCode.

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
use oxicode::{config, decode_from_slice, encode_to_vec, encode_to_vec_with_config, encoded_size};

// ============================================================================
// Test 1: [u8; 0] encodes as 0 bytes (empty array)
// ============================================================================

#[test]
fn test_fixed_array_u8_0_encodes_as_zero_bytes() {
    let original: [u8; 0] = [];
    let bytes = encode_to_vec(&original).expect("encode [u8; 0]");
    assert_eq!(
        bytes.len(),
        0,
        "[u8; 0] must encode as exactly 0 bytes (no length prefix)"
    );
    let (decoded, consumed): ([u8; 0], usize) = decode_from_slice(&bytes).expect("decode [u8; 0]");
    assert_eq!(decoded, original);
    assert_eq!(consumed, 0);
}

// ============================================================================
// Test 2: [u8; 1] roundtrip, exactly 1 byte
// ============================================================================

#[test]
fn test_fixed_array_u8_1_roundtrip_exactly_1_byte() {
    let original: [u8; 1] = [0xAB];
    let bytes = encode_to_vec(&original).expect("encode [u8; 1]");
    assert_eq!(
        bytes.len(),
        1,
        "[u8; 1] must encode as exactly 1 byte (no length prefix)"
    );
    assert_eq!(bytes[0], 0xAB, "single byte must be preserved verbatim");
    let (decoded, consumed): ([u8; 1], usize) = decode_from_slice(&bytes).expect("decode [u8; 1]");
    assert_eq!(decoded, original);
    assert_eq!(consumed, 1);
}

// ============================================================================
// Test 3: [u8; 4] with values [0, 1, 254, 255] roundtrip, exactly 4 bytes
// ============================================================================

#[test]
fn test_fixed_array_u8_4_boundary_values_roundtrip() {
    let original: [u8; 4] = [0, 1, 254, 255];
    let bytes = encode_to_vec(&original).expect("encode [u8; 4]");
    assert_eq!(
        bytes.len(),
        4,
        "[u8; 4] must encode as exactly 4 bytes (no length prefix)"
    );
    assert_eq!(bytes[0], 0u8);
    assert_eq!(bytes[1], 1u8);
    assert_eq!(bytes[2], 254u8);
    assert_eq!(bytes[3], 255u8);
    let (decoded, consumed): ([u8; 4], usize) = decode_from_slice(&bytes).expect("decode [u8; 4]");
    assert_eq!(decoded, original);
    assert_eq!(consumed, 4);
}

// ============================================================================
// Test 4: [u8; 8] with all zeros roundtrip
// ============================================================================

#[test]
fn test_fixed_array_u8_8_all_zeros_roundtrip() {
    let original: [u8; 8] = [0u8; 8];
    let bytes = encode_to_vec(&original).expect("encode [u8; 8] zeros");
    assert_eq!(
        bytes.len(),
        8,
        "[u8; 8] all-zero must encode as exactly 8 bytes"
    );
    assert!(bytes.iter().all(|&b| b == 0), "all bytes must be zero");
    let (decoded, consumed): ([u8; 8], usize) =
        decode_from_slice(&bytes).expect("decode [u8; 8] zeros");
    assert_eq!(decoded, original);
    assert_eq!(consumed, 8);
}

// ============================================================================
// Test 5: [u16; 4] roundtrip - each u16 is varint encoded
// ============================================================================

#[test]
fn test_fixed_array_u16_4_roundtrip() {
    let original: [u16; 4] = [0, 127, 128, u16::MAX];
    let bytes = encode_to_vec(&original).expect("encode [u16; 4]");
    // Varint: 0 → 1 byte, 127 → 1 byte, 128 → 2 bytes, 65535 → 3 bytes
    // Total: 1 + 1 + 2 + 3 = 7 bytes (no length prefix)
    assert!(
        !bytes.is_empty(),
        "[u16; 4] encoded bytes must not be empty"
    );
    let (decoded, _): ([u16; 4], usize) = decode_from_slice(&bytes).expect("decode [u16; 4]");
    assert_eq!(decoded, original);
}

// ============================================================================
// Test 6: [u32; 3] with values [0, 65536, u32::MAX] roundtrip
// ============================================================================

#[test]
fn test_fixed_array_u32_3_with_extremes_roundtrip() {
    let original: [u32; 3] = [0, 65536, u32::MAX];
    let bytes = encode_to_vec(&original).expect("encode [u32; 3]");
    assert!(
        !bytes.is_empty(),
        "[u32; 3] must produce a non-empty encoding"
    );
    let (decoded, _): ([u32; 3], usize) = decode_from_slice(&bytes).expect("decode [u32; 3]");
    assert_eq!(decoded, original);
}

// ============================================================================
// Test 7: [u64; 2] with values [0, u64::MAX] roundtrip
// ============================================================================

#[test]
fn test_fixed_array_u64_2_zero_and_max_roundtrip() {
    let original: [u64; 2] = [0, u64::MAX];
    let bytes = encode_to_vec(&original).expect("encode [u64; 2]");
    assert!(
        !bytes.is_empty(),
        "[u64; 2] must produce a non-empty encoding"
    );
    let (decoded, _): ([u64; 2], usize) = decode_from_slice(&bytes).expect("decode [u64; 2]");
    assert_eq!(decoded, original);
}

// ============================================================================
// Test 8: [i32; 4] with values [i32::MIN, -1, 0, i32::MAX] roundtrip
// ============================================================================

#[test]
fn test_fixed_array_i32_4_signed_extremes_roundtrip() {
    let original: [i32; 4] = [i32::MIN, -1, 0, i32::MAX];
    let bytes = encode_to_vec(&original).expect("encode [i32; 4]");
    assert!(
        !bytes.is_empty(),
        "[i32; 4] must produce a non-empty encoding"
    );
    let (decoded, _): ([i32; 4], usize) = decode_from_slice(&bytes).expect("decode [i32; 4]");
    assert_eq!(decoded, original);
}

// ============================================================================
// Test 9: [f32; 3] using std::f32::consts::PI for values - bit-exact comparison
// ============================================================================

#[test]
fn test_fixed_array_f32_3_bit_exact_roundtrip() {
    let original: [f32; 3] = [
        std::f32::consts::PI,
        std::f32::consts::E,
        std::f32::consts::SQRT_2,
    ];
    let bytes = encode_to_vec(&original).expect("encode [f32; 3]");
    // f32 is always 4 bytes, [f32; 3] = 12 bytes (no length prefix)
    assert_eq!(
        bytes.len(),
        12,
        "[f32; 3] must encode as exactly 12 bytes (3 × 4-byte IEEE 754)"
    );
    let (decoded, consumed): ([f32; 3], usize) =
        decode_from_slice(&bytes).expect("decode [f32; 3]");
    assert_eq!(consumed, 12);
    assert_eq!(
        decoded[0].to_bits(),
        original[0].to_bits(),
        "f32 PI must be bit-exact after roundtrip"
    );
    assert_eq!(
        decoded[1].to_bits(),
        original[1].to_bits(),
        "f32 E must be bit-exact after roundtrip"
    );
    assert_eq!(
        decoded[2].to_bits(),
        original[2].to_bits(),
        "f32 SQRT_2 must be bit-exact after roundtrip"
    );
}

// ============================================================================
// Test 10: [f64; 2] using std::f64::consts::PI and E - bit-exact comparison
// ============================================================================

#[test]
fn test_fixed_array_f64_2_bit_exact_roundtrip() {
    let original: [f64; 2] = [std::f64::consts::PI, std::f64::consts::E];
    let bytes = encode_to_vec(&original).expect("encode [f64; 2]");
    // f64 is always 8 bytes, [f64; 2] = 16 bytes (no length prefix)
    assert_eq!(
        bytes.len(),
        16,
        "[f64; 2] must encode as exactly 16 bytes (2 × 8-byte IEEE 754)"
    );
    let (decoded, consumed): ([f64; 2], usize) =
        decode_from_slice(&bytes).expect("decode [f64; 2]");
    assert_eq!(consumed, 16);
    assert_eq!(
        decoded[0].to_bits(),
        original[0].to_bits(),
        "f64 PI must be bit-exact after roundtrip"
    );
    assert_eq!(
        decoded[1].to_bits(),
        original[1].to_bits(),
        "f64 E must be bit-exact after roundtrip"
    );
}

// ============================================================================
// Test 11: [bool; 5] roundtrip
// ============================================================================

#[test]
fn test_fixed_array_bool_5_roundtrip() {
    let original: [bool; 5] = [true, false, true, true, false];
    let bytes = encode_to_vec(&original).expect("encode [bool; 5]");
    // bool is always 1 byte, [bool; 5] = 5 bytes
    assert_eq!(bytes.len(), 5, "[bool; 5] must encode as exactly 5 bytes");
    let (decoded, consumed): ([bool; 5], usize) =
        decode_from_slice(&bytes).expect("decode [bool; 5]");
    assert_eq!(consumed, 5);
    assert_eq!(decoded, original);
}

// ============================================================================
// Test 12: [[u8; 4]; 3] nested array roundtrip
// ============================================================================

#[test]
fn test_fixed_array_nested_u8_4_3_roundtrip() {
    let original: [[u8; 4]; 3] = [
        [0x01, 0x02, 0x03, 0x04],
        [0xAA, 0xBB, 0xCC, 0xDD],
        [0xFF, 0xFE, 0xFD, 0xFC],
    ];
    let bytes = encode_to_vec(&original).expect("encode [[u8; 4]; 3]");
    // [u8; 4] encodes as 4 bytes (no length prefix), [[u8; 4]; 3] = 12 bytes
    assert_eq!(
        bytes.len(),
        12,
        "[[u8; 4]; 3] must encode as exactly 12 bytes (no length prefixes at any level)"
    );
    let (decoded, consumed): ([[u8; 4]; 3], usize) =
        decode_from_slice(&bytes).expect("decode [[u8; 4]; 3]");
    assert_eq!(consumed, 12);
    assert_eq!(decoded, original);
}

// ============================================================================
// Test 13: [u8; 256] - all byte values 0..=255 roundtrip
// ============================================================================

#[test]
fn test_fixed_array_u8_256_all_byte_values_roundtrip() {
    let mut original = [0u8; 256];
    for (i, slot) in original.iter_mut().enumerate() {
        *slot = i as u8;
    }
    let bytes = encode_to_vec(&original).expect("encode [u8; 256]");
    assert_eq!(
        bytes.len(),
        256,
        "[u8; 256] must encode as exactly 256 bytes (no length prefix)"
    );
    // Verify raw bytes match directly since u8 encodes verbatim
    for (i, &byte) in bytes.iter().enumerate() {
        assert_eq!(
            byte, i as u8,
            "byte at index {i} must equal its index value"
        );
    }
    let (decoded, consumed): ([u8; 256], usize) =
        decode_from_slice(&bytes).expect("decode [u8; 256]");
    assert_eq!(consumed, 256);
    assert_eq!(decoded, original);
}

// ============================================================================
// Test 14: encoded_size([u8; 10]) == 10 (no length prefix)
// ============================================================================

#[test]
fn test_encoded_size_u8_10_is_10() {
    let value: [u8; 10] = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
    let size = encoded_size(&value).expect("encoded_size [u8; 10]");
    assert_eq!(
        size, 10,
        "encoded_size([u8; 10]) must be exactly 10 — no length prefix for fixed arrays"
    );
    // Verify consistency with actual encoding
    let bytes = encode_to_vec(&value).expect("encode [u8; 10]");
    assert_eq!(
        size,
        bytes.len(),
        "encoded_size must match actual encoded byte length"
    );
}

// ============================================================================
// Test 15: encoded_size([u32; 4]) with fixed_int_encoding = 16 bytes
// ============================================================================

#[test]
fn test_encoded_size_u32_4_with_fixed_int_encoding_is_16() {
    let value: [u32; 4] = [1, 2, 3, 4];
    let cfg = config::standard().with_fixed_int_encoding();
    let bytes = encode_to_vec_with_config(&value, cfg).expect("encode [u32; 4] fixed");
    // u32 with fixed encoding = 4 bytes each, [u32; 4] = 16 bytes
    assert_eq!(
        bytes.len(),
        16,
        "[u32; 4] with fixed_int_encoding must encode as exactly 16 bytes"
    );
}

// ============================================================================
// Test 16: [u8; 4] with fixed_int_encoding config - still 4 bytes (u8 always 1 byte)
// ============================================================================

#[test]
fn test_fixed_array_u8_4_fixed_int_encoding_still_4_bytes() {
    let original: [u8; 4] = [10, 20, 30, 40];
    let cfg = config::standard().with_fixed_int_encoding();
    let bytes = encode_to_vec_with_config(&original, cfg).expect("encode [u8; 4] fixed config");
    // u8 is always 1 byte regardless of int encoding mode
    assert_eq!(
        bytes.len(),
        4,
        "[u8; 4] with fixed_int_encoding must still be exactly 4 bytes — u8 has no varint form"
    );
    let (decoded, consumed): ([u8; 4], usize) =
        oxicode::decode_from_slice_with_config(&bytes, cfg).expect("decode [u8; 4] fixed config");
    assert_eq!(consumed, 4);
    assert_eq!(decoded, original);
}

// ============================================================================
// Test 17: [u32; 2] varint vs fixed: varint for [1, 1] is 2 bytes; fixed is 8 bytes
// ============================================================================

#[test]
fn test_fixed_array_u32_2_varint_vs_fixed_int_encoding_size_difference() {
    let value: [u32; 2] = [1, 1];

    let varint_bytes = encode_to_vec(&value).expect("encode [u32; 2] varint");
    // u32 value 1 in varint = 1 byte each → [u32; 2] = 2 bytes
    assert_eq!(
        varint_bytes.len(),
        2,
        "[u32; 2] = [1, 1] in varint encoding must be 2 bytes (1 byte per element)"
    );

    let fixed_cfg = config::standard().with_fixed_int_encoding();
    let fixed_bytes = encode_to_vec_with_config(&value, fixed_cfg).expect("encode [u32; 2] fixed");
    // u32 fixed = 4 bytes each → [u32; 2] = 8 bytes
    assert_eq!(
        fixed_bytes.len(),
        8,
        "[u32; 2] = [1, 1] in fixed-int encoding must be 8 bytes (4 bytes per element)"
    );

    // Both must decode back to the same value
    let (decoded_varint, _): ([u32; 2], usize) =
        decode_from_slice(&varint_bytes).expect("decode [u32; 2] varint");
    assert_eq!(decoded_varint, value);

    let (decoded_fixed, _): ([u32; 2], usize) =
        oxicode::decode_from_slice_with_config(&fixed_bytes, fixed_cfg)
            .expect("decode [u32; 2] fixed");
    assert_eq!(decoded_fixed, value);
}

// ============================================================================
// Test 18: Vec<[u8; 4]> roundtrip (vec of fixed arrays)
// ============================================================================

#[test]
fn test_vec_of_fixed_arrays_u8_4_roundtrip() {
    let original: Vec<[u8; 4]> = vec![
        [0x01, 0x02, 0x03, 0x04],
        [0xAA, 0xBB, 0xCC, 0xDD],
        [0x00, 0xFF, 0x00, 0xFF],
    ];
    let bytes = encode_to_vec(&original).expect("encode Vec<[u8; 4]>");
    // Vec<T> has a length prefix (varint). Each [u8; 4] element = 4 bytes.
    // Total = varint(3) + 3 × 4 = 1 + 12 = 13 bytes
    assert_eq!(
        bytes.len(),
        13,
        "Vec<[u8; 4]> with 3 elements must encode as 13 bytes (1-byte varint len + 12 data bytes)"
    );
    let (decoded, _): (Vec<[u8; 4]>, usize) =
        decode_from_slice(&bytes).expect("decode Vec<[u8; 4]>");
    assert_eq!(decoded, original);
}

// ============================================================================
// Test 19: Option<[u32; 3]> Some and None roundtrip
// ============================================================================

#[test]
fn test_option_of_fixed_array_u32_3_some_and_none_roundtrip() {
    // Test Some variant
    let some_val: Option<[u32; 3]> = Some([100, 200, 300]);
    let bytes_some = encode_to_vec(&some_val).expect("encode Option<[u32; 3]> Some");
    let (decoded_some, _): (Option<[u32; 3]>, usize) =
        decode_from_slice(&bytes_some).expect("decode Option<[u32; 3]> Some");
    assert_eq!(decoded_some, some_val);

    // Test None variant
    let none_val: Option<[u32; 3]> = None;
    let bytes_none = encode_to_vec(&none_val).expect("encode Option<[u32; 3]> None");
    let (decoded_none, _): (Option<[u32; 3]>, usize) =
        decode_from_slice(&bytes_none).expect("decode Option<[u32; 3]> None");
    assert_eq!(decoded_none, none_val);

    // None must encode as fewer bytes than Some
    assert!(
        bytes_none.len() < bytes_some.len(),
        "None must encode as fewer bytes than Some([u32; 3])"
    );
}

// ============================================================================
// Test 20: Struct with a [u8; 16] field (like a UUID) roundtrip
// ============================================================================

use oxicode_derive::{Decode, Encode};

#[derive(Debug, PartialEq, Encode, Decode)]
struct UuidRecord {
    id: [u8; 16],
    label: String,
}

#[test]
fn test_struct_with_uuid_field_roundtrip() {
    let original = UuidRecord {
        // A UUID-like byte array: 16 bytes, no length prefix
        id: [
            0x6b, 0xa7, 0xb8, 0x10, 0x9d, 0xad, 0x11, 0xd1, 0x80, 0xb4, 0x00, 0xc0, 0x4f, 0xd4,
            0x30, 0xc8,
        ],
        label: String::from("oxicode-uuid-test"),
    };
    let bytes = encode_to_vec(&original).expect("encode UuidRecord");
    let (decoded, _): (UuidRecord, usize) = decode_from_slice(&bytes).expect("decode UuidRecord");
    assert_eq!(decoded, original);

    // Verify that the [u8; 16] field occupies exactly 16 bytes in the output
    // (no length prefix). The total size = 16 (id) + varint(len("oxicode-uuid-test")) + 17 bytes.
    let label_len_varint = 1usize; // 17 < 128, so varint fits in 1 byte
    let label_bytes = original.label.len();
    let expected_total = 16 + label_len_varint + label_bytes;
    assert_eq!(
        bytes.len(),
        expected_total,
        "UuidRecord encoding must be id(16) + varint_len(1) + label_bytes(17)"
    );
}

// ============================================================================
// Test 21: [u8; 32] roundtrip (sha256 sized)
// ============================================================================

#[test]
fn test_fixed_array_u8_32_sha256_sized_roundtrip() {
    // Simulate a SHA-256 digest (32 bytes)
    let original: [u8; 32] = [
        0xe3, 0xb0, 0xc4, 0x42, 0x98, 0xfc, 0x1c, 0x14, 0x9a, 0xfb, 0xf4, 0xc8, 0x99, 0x6f, 0xb9,
        0x24, 0x27, 0xae, 0x41, 0xe4, 0x64, 0x9b, 0x93, 0x4c, 0xa4, 0x95, 0x99, 0x1b, 0x78, 0x52,
        0xb8, 0x55,
    ];
    let bytes = encode_to_vec(&original).expect("encode [u8; 32]");
    assert_eq!(
        bytes.len(),
        32,
        "[u8; 32] must encode as exactly 32 bytes (no length prefix — fixed-size array)"
    );
    // Wire bytes must be verbatim copy of the source array
    assert_eq!(
        bytes.as_slice(),
        original.as_slice(),
        "[u8; 32] wire bytes must be identical to source data"
    );
    let (decoded, consumed): ([u8; 32], usize) =
        decode_from_slice(&bytes).expect("decode [u8; 32]");
    assert_eq!(consumed, 32);
    assert_eq!(decoded, original);
}

// ============================================================================
// Test 22: Tuple ([u8; 4], String) roundtrip
// ============================================================================

#[test]
fn test_tuple_fixed_array_and_string_roundtrip() {
    let original: ([u8; 4], String) = ([0xDE, 0xAD, 0xBE, 0xEF], String::from("oxicode"));
    let bytes = encode_to_vec(&original).expect("encode ([u8; 4], String)");
    // [u8; 4] = 4 bytes, String = varint(7) + 7 bytes = 1 + 7 = 8 bytes
    // Total = 4 + 8 = 12 bytes
    let expected_size = 4 + 1 + original.1.len();
    assert_eq!(
        bytes.len(),
        expected_size,
        "([u8; 4], String) tuple must encode as {} bytes",
        expected_size
    );
    let (decoded, _): (([u8; 4], String), usize) =
        decode_from_slice(&bytes).expect("decode ([u8; 4], String)");
    assert_eq!(decoded.0, original.0);
    assert_eq!(decoded.1, original.1);
}
