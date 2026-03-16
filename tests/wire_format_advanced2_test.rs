//! Advanced wire format tests for OxiCode — exact binary layout verification.
//!
//! Each test validates the byte-level encoding of a specific type or configuration.
//! No `#[cfg(test)]` wrapper — all tests are top-level as required.

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
use oxicode::{Decode, Encode};

// ===== Derived types used across tests =====

#[derive(Debug, PartialEq, Encode, Decode)]
enum Direction {
    North,
    South,
    East,
    West,
}

// ===== Test 1: u8(0) encodes to [0x00] =====

#[test]
fn test_u8_zero_encodes_to_single_zero_byte() {
    let bytes = encode_to_vec(&0u8).expect("encode u8(0) failed");
    assert_eq!(bytes, vec![0x00u8], "u8(0) must encode to exactly [0x00]");
}

// ===== Test 2: u8(1) encodes to [0x01] =====

#[test]
fn test_u8_one_encodes_to_single_one_byte() {
    let bytes = encode_to_vec(&1u8).expect("encode u8(1) failed");
    assert_eq!(bytes, vec![0x01u8], "u8(1) must encode to exactly [0x01]");
}

// ===== Test 3: u8(127) encodes to [0x7F] =====

#[test]
fn test_u8_127_encodes_to_0x7f() {
    let bytes = encode_to_vec(&127u8).expect("encode u8(127) failed");
    assert_eq!(bytes, vec![0x7Fu8], "u8(127) must encode to exactly [0x7F]");
}

// ===== Test 4: u8(128) encodes to single byte [0x80] =====
// u8 is always written as a raw single byte, never varint

#[test]
fn test_u8_128_encodes_to_single_raw_byte() {
    let bytes = encode_to_vec(&128u8).expect("encode u8(128) failed");
    assert_eq!(bytes.len(), 1, "u8 always encodes to exactly 1 byte");
    assert_eq!(bytes, vec![0x80u8], "u8(128) must encode to [0x80]");
}

// ===== Test 5: u8(255) encodes to single byte [0xFF] =====
// u8 writes raw, not varint — no tag bytes

#[test]
fn test_u8_255_encodes_to_single_raw_byte() {
    let bytes = encode_to_vec(&255u8).expect("encode u8(255) failed");
    assert_eq!(
        bytes.len(),
        1,
        "u8(255) must be exactly 1 byte (raw, not varint)"
    );
    assert_eq!(bytes, vec![0xFFu8], "u8(255) must encode to [0xFF]");
}

// ===== Test 6: bool(false) encodes to [0x00] =====

#[test]
fn test_bool_false_encodes_to_0x00() {
    let bytes = encode_to_vec(&false).expect("encode bool(false) failed");
    assert_eq!(bytes, vec![0x00u8], "bool(false) must encode to [0x00]");
}

// ===== Test 7: bool(true) encodes to [0x01] =====

#[test]
fn test_bool_true_encodes_to_0x01() {
    let bytes = encode_to_vec(&true).expect("encode bool(true) failed");
    assert_eq!(bytes, vec![0x01u8], "bool(true) must encode to [0x01]");
}

// ===== Test 8: None::<u32> encodes to [0x00] =====

#[test]
fn test_option_none_u32_encodes_to_0x00() {
    let val: Option<u32> = None;
    let bytes = encode_to_vec(&val).expect("encode None::<u32> failed");
    assert_eq!(bytes, vec![0x00u8], "None must encode to [0x00]");
}

// ===== Test 9: Some(1u32) starts with [0x01] tag byte =====

#[test]
fn test_option_some_1u32_starts_with_0x01() {
    let val: Option<u32> = Some(1u32);
    let bytes = encode_to_vec(&val).expect("encode Some(1u32) failed");
    assert!(!bytes.is_empty(), "Some(1u32) encoding must not be empty");
    assert_eq!(
        bytes[0], 0x01u8,
        "Some variant must start with tag byte 0x01"
    );
    // Verify the total length: 1 tag byte + varint(1) for u32 = 2 bytes
    assert_eq!(
        bytes.len(),
        2,
        "Some(1u32) with varint must be 2 bytes total"
    );
    assert_eq!(bytes[1], 0x01u8, "varint(1) must be [0x01]");
}

// ===== Test 10: Empty Vec<u8> encodes to [0x00] (varint length=0) =====

#[test]
fn test_empty_vec_u8_encodes_to_single_zero() {
    let val: Vec<u8> = Vec::new();
    let bytes = encode_to_vec(&val).expect("encode empty Vec<u8> failed");
    assert_eq!(
        bytes,
        vec![0x00u8],
        "empty Vec must encode to varint length [0x00]"
    );
}

// ===== Test 11: Vec<u8> [1,2,3] encodes to [0x03, 0x01, 0x02, 0x03] =====

#[test]
fn test_vec_u8_three_elements_wire_format() {
    let val: Vec<u8> = vec![1u8, 2u8, 3u8];
    let bytes = encode_to_vec(&val).expect("encode Vec<u8>[1,2,3] failed");
    // varint(3) = [0x03], then raw bytes 0x01, 0x02, 0x03
    assert_eq!(
        bytes,
        vec![0x03u8, 0x01u8, 0x02u8, 0x03u8],
        "Vec<u8>[1,2,3] must encode to [0x03, 0x01, 0x02, 0x03]"
    );
}

// ===== Test 12: String "ab" encodes to [0x02, 0x61, 0x62] =====

#[test]
fn test_string_ab_encodes_to_length_prefix_plus_utf8() {
    let val = String::from("ab");
    let bytes = encode_to_vec(&val).expect("encode String(\"ab\") failed");
    // varint(2) = [0x02], then 'a'=0x61, 'b'=0x62
    assert_eq!(
        bytes,
        vec![0x02u8, 0x61u8, 0x62u8],
        "String(\"ab\") must encode to [0x02, 0x61, 0x62]"
    );
}

// ===== Test 13: String "" encodes to [0x00] =====

#[test]
fn test_empty_string_encodes_to_single_zero() {
    let val = String::new();
    let bytes = encode_to_vec(&val).expect("encode empty String failed");
    assert_eq!(
        bytes,
        vec![0x00u8],
        "empty String must encode to [0x00] (varint length 0)"
    );
}

// ===== Test 14: Unit enum first variant encodes to [0x00] =====

#[test]
fn test_unit_enum_first_variant_encodes_to_0x00() {
    let val = Direction::North;
    let bytes = encode_to_vec(&val).expect("encode Direction::North failed");
    // Enum discriminant is u32 varint; 0 => [0x00]
    assert_eq!(
        bytes,
        vec![0x00u8],
        "first unit enum variant must encode to [0x00]"
    );
}

// ===== Test 15: Unit enum second variant encodes to [0x01] =====

#[test]
fn test_unit_enum_second_variant_encodes_to_0x01() {
    let val = Direction::South;
    let bytes = encode_to_vec(&val).expect("encode Direction::South failed");
    // Discriminant 1 as u32 varint => [0x01]
    assert_eq!(
        bytes,
        vec![0x01u8],
        "second unit enum variant must encode to [0x01]"
    );
}

// ===== Test 16: u32(0) with fixed-int encodes to [0x00, 0x00, 0x00, 0x00] =====

#[test]
fn test_u32_zero_fixed_int_encodes_to_four_zero_bytes() {
    let cfg = config::standard().with_fixed_int_encoding();
    let bytes = encode_to_vec_with_config(&0u32, cfg).expect("encode u32(0) fixed failed");
    assert_eq!(
        bytes,
        vec![0x00u8, 0x00u8, 0x00u8, 0x00u8],
        "u32(0) with fixed encoding must be [0x00, 0x00, 0x00, 0x00]"
    );
}

// ===== Test 17: u32(1) fixed-int LE encodes to [0x01, 0x00, 0x00, 0x00] =====

#[test]
fn test_u32_one_fixed_int_little_endian() {
    let cfg = config::standard()
        .with_fixed_int_encoding()
        .with_little_endian();
    let bytes = encode_to_vec_with_config(&1u32, cfg).expect("encode u32(1) fixed LE failed");
    assert_eq!(
        bytes,
        vec![0x01u8, 0x00u8, 0x00u8, 0x00u8],
        "u32(1) fixed LE must be [0x01, 0x00, 0x00, 0x00]"
    );
}

// ===== Test 18: u32(256) fixed-int LE encodes to [0x00, 0x01, 0x00, 0x00] =====

#[test]
fn test_u32_256_fixed_int_little_endian() {
    let cfg = config::standard()
        .with_fixed_int_encoding()
        .with_little_endian();
    let bytes = encode_to_vec_with_config(&256u32, cfg).expect("encode u32(256) fixed LE failed");
    assert_eq!(
        bytes,
        vec![0x00u8, 0x01u8, 0x00u8, 0x00u8],
        "u32(256) fixed LE must be [0x00, 0x01, 0x00, 0x00]"
    );
}

// ===== Test 19: u32(1) fixed-int BE encodes to [0x00, 0x00, 0x00, 0x01] =====

#[test]
fn test_u32_one_fixed_int_big_endian() {
    let cfg = config::standard()
        .with_fixed_int_encoding()
        .with_big_endian();
    let bytes = encode_to_vec_with_config(&1u32, cfg).expect("encode u32(1) fixed BE failed");
    assert_eq!(
        bytes,
        vec![0x00u8, 0x00u8, 0x00u8, 0x01u8],
        "u32(1) fixed BE must be [0x00, 0x00, 0x00, 0x01]"
    );
}

// ===== Test 20: (u8, u8) tuple (1, 2) encodes to [0x01, 0x02] =====

#[test]
fn test_u8_tuple_encodes_as_concatenated_raw_bytes() {
    let val: (u8, u8) = (1u8, 2u8);
    let bytes = encode_to_vec(&val).expect("encode (u8, u8) failed");
    // Tuples encode as field concatenation; u8 is always raw 1 byte
    assert_eq!(
        bytes,
        vec![0x01u8, 0x02u8],
        "(u8, u8) = (1, 2) must encode to [0x01, 0x02]"
    );
}

// ===== Test 21: Round-trip decode from known bytes =====

#[test]
fn test_roundtrip_decode_from_known_bytes_matches() {
    // Encode a known value and verify decoding gives back the original
    let original: u32 = 12345u32;
    let encoded = encode_to_vec(&original).expect("encode u32(12345) failed");
    let (decoded, consumed): (u32, usize) =
        decode_from_slice(&encoded).expect("decode u32(12345) failed");
    assert_eq!(decoded, original, "decoded value must match original");
    assert_eq!(consumed, encoded.len(), "must consume all encoded bytes");
}

// ===== Test 22: Decode known byte sequence yields expected value =====

#[test]
fn test_decode_known_bytes_for_varint_u32_yields_correct_value() {
    // varint encoding: value 42 (<=250) encodes as [0x2A]
    // For u32 in standard (varint) config: [0x2A] decodes to 42
    let raw_bytes: &[u8] = &[0x2Au8];
    let (val, consumed): (u32, usize) =
        decode_from_slice_with_config(raw_bytes, config::standard())
            .expect("decode varint u32 from [0x2A] failed");
    assert_eq!(val, 42u32, "varint byte 0x2A must decode to u32(42)");
    assert_eq!(
        consumed, 1usize,
        "single varint byte must consume exactly 1 byte"
    );
}
