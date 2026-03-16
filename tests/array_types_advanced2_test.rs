//! Advanced tests for fixed-size array encoding in OxiCode.
//!
//! 22 tests covering a variety of element types, sizes, configurations, and edge cases.

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
// Helper struct used in test 19
// ---------------------------------------------------------------------------

#[derive(Debug, PartialEq, Encode, Decode)]
struct WithArray {
    key: [u8; 4],
    value: u64,
}

// ---------------------------------------------------------------------------
// Test 1: [u8; 4] roundtrip
// ---------------------------------------------------------------------------

#[test]
fn test_u8_array_4_roundtrip() {
    let val: [u8; 4] = [0xDE, 0xAD, 0xBE, 0xEF];
    let enc = encode_to_vec(&val).expect("encode [u8; 4]");
    let (dec, _): ([u8; 4], usize) = decode_from_slice(&enc).expect("decode [u8; 4]");
    assert_eq!(val, dec);
}

// ---------------------------------------------------------------------------
// Test 2: [u8; 16] roundtrip
// ---------------------------------------------------------------------------

#[test]
fn test_u8_array_16_roundtrip() {
    let val: [u8; 16] = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16];
    let enc = encode_to_vec(&val).expect("encode [u8; 16]");
    let (dec, _): ([u8; 16], usize) = decode_from_slice(&enc).expect("decode [u8; 16]");
    assert_eq!(val, dec);
}

// ---------------------------------------------------------------------------
// Test 3: [u32; 4] roundtrip
// ---------------------------------------------------------------------------

#[test]
fn test_u32_array_4_roundtrip() {
    let val: [u32; 4] = [100, 200, 300, 400];
    let enc = encode_to_vec(&val).expect("encode [u32; 4]");
    let (dec, _): ([u32; 4], usize) = decode_from_slice(&enc).expect("decode [u32; 4]");
    assert_eq!(val, dec);
}

// ---------------------------------------------------------------------------
// Test 4: [u64; 2] roundtrip
// ---------------------------------------------------------------------------

#[test]
fn test_u64_array_2_roundtrip() {
    let val: [u64; 2] = [u64::MAX, 0];
    let enc = encode_to_vec(&val).expect("encode [u64; 2]");
    let (dec, _): ([u64; 2], usize) = decode_from_slice(&enc).expect("decode [u64; 2]");
    assert_eq!(val, dec);
}

// ---------------------------------------------------------------------------
// Test 5: [i32; 8] with negative values roundtrip
// ---------------------------------------------------------------------------

#[test]
fn test_i32_array_8_negative_roundtrip() {
    let val: [i32; 8] = [-1, -100, -1000, i32::MIN, 0, 1, 100, i32::MAX];
    let enc = encode_to_vec(&val).expect("encode [i32; 8] negative");
    let (dec, _): ([i32; 8], usize) = decode_from_slice(&enc).expect("decode [i32; 8] negative");
    assert_eq!(val, dec);
}

// ---------------------------------------------------------------------------
// Test 6: [bool; 8] roundtrip
// ---------------------------------------------------------------------------

#[test]
fn test_bool_array_8_roundtrip() {
    let val: [bool; 8] = [true, false, true, true, false, false, true, false];
    let enc = encode_to_vec(&val).expect("encode [bool; 8]");
    let (dec, _): ([bool; 8], usize) = decode_from_slice(&enc).expect("decode [bool; 8]");
    assert_eq!(val, dec);
}

// ---------------------------------------------------------------------------
// Test 7: [u8; 0] empty array roundtrip (encodes to 0 bytes)
// ---------------------------------------------------------------------------

#[test]
fn test_u8_array_0_empty_roundtrip() {
    let val: [u8; 0] = [];
    let enc = encode_to_vec(&val).expect("encode [u8; 0]");
    assert_eq!(enc.len(), 0, "[u8; 0] must encode to exactly 0 bytes");
    let (dec, _): ([u8; 0], usize) = decode_from_slice(&enc).expect("decode [u8; 0]");
    assert_eq!(val, dec);
}

// ---------------------------------------------------------------------------
// Test 8: [u8; 256] large array roundtrip
// ---------------------------------------------------------------------------

#[test]
fn test_u8_array_256_large_roundtrip() {
    let mut val = [0u8; 256];
    for (i, b) in val.iter_mut().enumerate() {
        *b = i as u8;
    }
    let enc = encode_to_vec(&val).expect("encode [u8; 256]");
    assert_eq!(enc.len(), 256, "[u8; 256] must encode to exactly 256 bytes");
    let (dec, _): ([u8; 256], usize) = decode_from_slice(&enc).expect("decode [u8; 256]");
    assert_eq!(val, dec);
}

// ---------------------------------------------------------------------------
// Test 9: [f64; 4] roundtrip (bit-exact)
// ---------------------------------------------------------------------------

#[test]
fn test_f64_array_4_bit_exact_roundtrip() {
    let val: [f64; 4] = [0.0_f64, 1.5_f64, -1.5_f64, f64::NAN];
    let enc = encode_to_vec(&val).expect("encode [f64; 4]");
    let (dec, _): ([f64; 4], usize) = decode_from_slice(&enc).expect("decode [f64; 4]");
    for (a, b) in val.iter().zip(dec.iter()) {
        assert_eq!(
            a.to_bits(),
            b.to_bits(),
            "f64 bit pattern must be preserved: {:?} vs {:?}",
            a,
            b
        );
    }
}

// ---------------------------------------------------------------------------
// Test 10: [[u8; 4]; 4] nested array roundtrip
// ---------------------------------------------------------------------------

#[test]
fn test_nested_u8_array_4x4_roundtrip() {
    let val: [[u8; 4]; 4] = [
        [0x01, 0x02, 0x03, 0x04],
        [0x05, 0x06, 0x07, 0x08],
        [0x09, 0x0A, 0x0B, 0x0C],
        [0x0D, 0x0E, 0x0F, 0x10],
    ];
    let enc = encode_to_vec(&val).expect("encode [[u8; 4]; 4]");
    assert_eq!(
        enc.len(),
        16,
        "[[u8; 4]; 4] must encode to exactly 16 bytes with no length prefixes"
    );
    let (dec, _): ([[u8; 4]; 4], usize) = decode_from_slice(&enc).expect("decode [[u8; 4]; 4]");
    assert_eq!(val, dec);
}

// ---------------------------------------------------------------------------
// Test 11: [u8; 4] with fixed_int_encoding config roundtrip
// ---------------------------------------------------------------------------

#[test]
fn test_u8_array_4_fixed_int_config_roundtrip() {
    let val: [u8; 4] = [0xAA, 0xBB, 0xCC, 0xDD];
    let cfg = config::standard().with_fixed_int_encoding();
    let enc = encode_to_vec_with_config(&val, cfg).expect("encode [u8; 4] fixed_int");
    let (dec, _): ([u8; 4], usize) =
        decode_from_slice_with_config(&enc, cfg).expect("decode [u8; 4] fixed_int");
    assert_eq!(val, dec);
}

// ---------------------------------------------------------------------------
// Test 12: [u32; 4] with big_endian + fixed_int_encoding config roundtrip
// ---------------------------------------------------------------------------

#[test]
fn test_u32_array_4_big_endian_fixed_int_roundtrip() {
    let val: [u32; 4] = [0x01020304, 0x05060708, 0xDEADBEEF, 0xCAFEBABE];
    let cfg = config::standard()
        .with_big_endian()
        .with_fixed_int_encoding();
    let enc = encode_to_vec_with_config(&val, cfg).expect("encode [u32; 4] BE+fixed");
    assert_eq!(
        enc.len(),
        16,
        "[u32; 4] with BE+fixed must be exactly 16 bytes"
    );
    let (dec, _): ([u32; 4], usize) =
        decode_from_slice_with_config(&enc, cfg).expect("decode [u32; 4] BE+fixed");
    assert_eq!(val, dec);
}

// ---------------------------------------------------------------------------
// Test 13: [u8; 4] encodes to exactly 4 bytes
// ---------------------------------------------------------------------------

#[test]
fn test_u8_array_4_exact_byte_length() {
    let val: [u8; 4] = [1, 2, 3, 4];
    let enc = encode_to_vec(&val).expect("encode [u8; 4]");
    assert_eq!(
        enc.len(),
        4,
        "[u8; 4] must encode to exactly 4 bytes (no length prefix)"
    );
}

// ---------------------------------------------------------------------------
// Test 14: [u32; 4] with standard config — each u32 varint-encoded
// ---------------------------------------------------------------------------

#[test]
fn test_u32_array_4_standard_varint_encoding() {
    // With varint encoding, small values use fewer bytes.
    // Values 0..=127 each use 1 byte as varint in oxicode standard.
    let val: [u32; 4] = [1, 2, 3, 4];
    let enc = encode_to_vec(&val).expect("encode [u32; 4] standard varint");
    // Each u32 value 1..=4 encodes as 1 byte varint, so total must be < 16
    assert!(
        enc.len() < 16,
        "[u32; 4] standard config with small values must be less than 16 bytes (varint); got {}",
        enc.len()
    );
    let (dec, _): ([u32; 4], usize) =
        decode_from_slice(&enc).expect("decode [u32; 4] standard varint");
    assert_eq!(val, dec);
}

// ---------------------------------------------------------------------------
// Test 15: [u8; 4] with big_endian config byte order check
// ---------------------------------------------------------------------------

#[test]
fn test_u8_array_4_big_endian_byte_order() {
    // For u8 arrays, endianness has no effect — bytes are always the same.
    let val: [u8; 4] = [0x01, 0x02, 0x03, 0x04];
    let cfg = config::standard().with_big_endian();
    let enc = encode_to_vec_with_config(&val, cfg).expect("encode [u8; 4] BE");
    assert_eq!(enc.len(), 4, "[u8; 4] BE must be exactly 4 bytes");
    assert_eq!(enc[0], 0x01, "first byte must be 0x01");
    assert_eq!(enc[1], 0x02, "second byte must be 0x02");
    assert_eq!(enc[2], 0x03, "third byte must be 0x03");
    assert_eq!(enc[3], 0x04, "fourth byte must be 0x04");
    let (dec, _): ([u8; 4], usize) =
        decode_from_slice_with_config(&enc, cfg).expect("decode [u8; 4] BE");
    assert_eq!(val, dec);
}

// ---------------------------------------------------------------------------
// Test 16: Vec<[u8; 4]> roundtrip
// ---------------------------------------------------------------------------

#[test]
fn test_vec_of_u8_array_4_roundtrip() {
    let val: Vec<[u8; 4]> = vec![
        [0x01, 0x02, 0x03, 0x04],
        [0xAA, 0xBB, 0xCC, 0xDD],
        [0xFF, 0x00, 0xFF, 0x00],
    ];
    let enc = encode_to_vec(&val).expect("encode Vec<[u8; 4]>");
    let (dec, _): (Vec<[u8; 4]>, usize) = decode_from_slice(&enc).expect("decode Vec<[u8; 4]>");
    assert_eq!(val, dec);
}

// ---------------------------------------------------------------------------
// Test 17: Option<[u8; 8]> Some roundtrip
// ---------------------------------------------------------------------------

#[test]
fn test_option_u8_array_8_some_roundtrip() {
    let val: Option<[u8; 8]> = Some([10, 20, 30, 40, 50, 60, 70, 80]);
    let enc = encode_to_vec(&val).expect("encode Option<[u8; 8]> Some");
    let (dec, _): (Option<[u8; 8]>, usize) =
        decode_from_slice(&enc).expect("decode Option<[u8; 8]> Some");
    assert_eq!(val, dec);
}

// ---------------------------------------------------------------------------
// Test 18: Option<[u8; 8]> None roundtrip
// ---------------------------------------------------------------------------

#[test]
fn test_option_u8_array_8_none_roundtrip() {
    let val: Option<[u8; 8]> = None;
    let enc = encode_to_vec(&val).expect("encode Option<[u8; 8]> None");
    let (dec, _): (Option<[u8; 8]>, usize) =
        decode_from_slice(&enc).expect("decode Option<[u8; 8]> None");
    assert_eq!(val, dec);
}

// ---------------------------------------------------------------------------
// Test 19: Struct with [u8; 4] field roundtrip
// ---------------------------------------------------------------------------

#[test]
fn test_struct_with_u8_array_field_roundtrip() {
    let val = WithArray {
        key: [0xDE, 0xAD, 0xBE, 0xEF],
        value: 0xCAFE_BABE_DEAD_BEEF,
    };
    let enc = encode_to_vec(&val).expect("encode WithArray");
    let (dec, _): (WithArray, usize) = decode_from_slice(&enc).expect("decode WithArray");
    assert_eq!(val, dec);
}

// ---------------------------------------------------------------------------
// Test 20: consumed bytes == encoded length for [u64; 8]
// ---------------------------------------------------------------------------

#[test]
fn test_u64_array_8_consumed_equals_encoded_length() {
    let val: [u64; 8] = [0, 1, 2, 3, u64::MAX, u64::MAX - 1, 100, 999_999];
    let enc = encode_to_vec(&val).expect("encode [u64; 8]");
    let (_, consumed): ([u64; 8], usize) = decode_from_slice(&enc).expect("decode [u64; 8]");
    assert_eq!(
        consumed,
        enc.len(),
        "consumed bytes must equal total encoded length"
    );
}

// ---------------------------------------------------------------------------
// Test 21: Two different [u8; 4] values produce different encodings
// ---------------------------------------------------------------------------

#[test]
fn test_u8_array_4_different_values_different_encodings() {
    let val_a: [u8; 4] = [0x01, 0x02, 0x03, 0x04];
    let val_b: [u8; 4] = [0x04, 0x03, 0x02, 0x01];
    let enc_a = encode_to_vec(&val_a).expect("encode [u8; 4] val_a");
    let enc_b = encode_to_vec(&val_b).expect("encode [u8; 4] val_b");
    assert_ne!(
        enc_a, enc_b,
        "different [u8; 4] values must produce different encodings"
    );
}

// ---------------------------------------------------------------------------
// Test 22: [u8; 32] roundtrip (common hash/key size)
// ---------------------------------------------------------------------------

#[test]
fn test_u8_array_32_hash_key_roundtrip() {
    let val: [u8; 32] = [
        0x00, 0x11, 0x22, 0x33, 0x44, 0x55, 0x66, 0x77, 0x88, 0x99, 0xAA, 0xBB, 0xCC, 0xDD, 0xEE,
        0xFF, 0xFF, 0xEE, 0xDD, 0xCC, 0xBB, 0xAA, 0x99, 0x88, 0x77, 0x66, 0x55, 0x44, 0x33, 0x22,
        0x11, 0x00,
    ];
    let enc = encode_to_vec(&val).expect("encode [u8; 32] hash key");
    assert_eq!(enc.len(), 32, "[u8; 32] must encode to exactly 32 bytes");
    let (dec, _): ([u8; 32], usize) = decode_from_slice(&enc).expect("decode [u8; 32] hash key");
    assert_eq!(val, dec);
}
