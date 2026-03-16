//! Advanced checksum/CRC32 tests for OxiCode — exactly 22 top-level #[test] functions.
//!
//! Compile with: cargo test --features checksum --test checksum_advanced2_test

#![cfg(feature = "checksum")]
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
use oxicode::checksum::{decode_with_checksum, encode_with_checksum, verify_checksum, HEADER_SIZE};
use oxicode::{Decode, Encode};

// ---------------------------------------------------------------------------
// Shared helper types (defined once at top level)
// ---------------------------------------------------------------------------

#[derive(Debug, Clone, PartialEq, Encode, Decode)]
struct SimplePoint {
    x: f64,
    y: f64,
}

#[derive(Debug, Clone, PartialEq, Encode, Decode)]
struct NestedWrapper {
    label: String,
    inner: SimplePoint,
}

#[derive(Debug, Clone, PartialEq, Encode, Decode)]
enum TaggedEnum {
    Unit,
    Pair(u32, u64),
    Named { value: String },
}

// ---------------------------------------------------------------------------
// Test 1: encode u32 with checksum, decode successfully
// ---------------------------------------------------------------------------
#[test]
fn test_checksum_u32_roundtrip() {
    let value: u32 = 123_456_789;
    let encoded = encode_with_checksum(&value).expect("encode u32 failed");
    let (decoded, consumed): (u32, _) = decode_with_checksum(&encoded).expect("decode u32 failed");
    assert_eq!(decoded, value);
    assert_eq!(consumed, encoded.len());
}

// ---------------------------------------------------------------------------
// Test 2: encode String with checksum, decode successfully
// ---------------------------------------------------------------------------
#[test]
fn test_checksum_string_roundtrip() {
    let value = String::from("Hello, OxiCode checksum test!");
    let encoded = encode_with_checksum(&value).expect("encode String failed");
    let (decoded, consumed): (String, _) =
        decode_with_checksum(&encoded).expect("decode String failed");
    assert_eq!(decoded, value);
    assert_eq!(consumed, encoded.len());
}

// ---------------------------------------------------------------------------
// Test 3: encode Vec<u8> with checksum, decode successfully
// ---------------------------------------------------------------------------
#[test]
fn test_checksum_vec_u8_roundtrip() {
    let value: Vec<u8> = vec![10, 20, 30, 40, 50, 60, 70, 80];
    let encoded = encode_with_checksum(&value).expect("encode Vec<u8> failed");
    let (decoded, consumed): (Vec<u8>, _) =
        decode_with_checksum(&encoded).expect("decode Vec<u8> failed");
    assert_eq!(decoded, value);
    assert_eq!(consumed, encoded.len());
}

// ---------------------------------------------------------------------------
// Test 4: encode struct with checksum, decode successfully
// ---------------------------------------------------------------------------
#[test]
fn test_checksum_struct_roundtrip() {
    let value = SimplePoint {
        x: 3.14159,
        y: -2.71828,
    };
    let encoded = encode_with_checksum(&value).expect("encode SimplePoint failed");
    let (decoded, consumed): (SimplePoint, _) =
        decode_with_checksum(&encoded).expect("decode SimplePoint failed");
    assert_eq!(decoded, value);
    assert_eq!(consumed, encoded.len());
}

// ---------------------------------------------------------------------------
// Test 5: tampered data (flip a byte) causes decode error
// ---------------------------------------------------------------------------
#[test]
fn test_checksum_tampered_data_causes_error() {
    let value: u32 = 0xCAFE_BABE;
    let mut encoded = encode_with_checksum(&value).expect("encode failed");
    // Flip a byte in the payload region (after the 16-byte header)
    encoded[HEADER_SIZE] ^= 0xFF;
    let result = decode_with_checksum::<u32>(&encoded);
    assert!(result.is_err(), "tampered data must return Err, but got Ok");
    assert!(
        matches!(result, Err(oxicode::Error::ChecksumMismatch { .. })),
        "error must be ChecksumMismatch, got: {:?}",
        result
    );
}

// ---------------------------------------------------------------------------
// Test 6: encode bool true with checksum
// ---------------------------------------------------------------------------
#[test]
fn test_checksum_bool_true_roundtrip() {
    let value = true;
    let encoded = encode_with_checksum(&value).expect("encode bool true failed");
    let (decoded, consumed): (bool, _) =
        decode_with_checksum(&encoded).expect("decode bool true failed");
    assert!(decoded);
    assert_eq!(consumed, encoded.len());
}

// ---------------------------------------------------------------------------
// Test 7: encode bool false with checksum
// ---------------------------------------------------------------------------
#[test]
fn test_checksum_bool_false_roundtrip() {
    let value = false;
    let encoded = encode_with_checksum(&value).expect("encode bool false failed");
    let (decoded, consumed): (bool, _) =
        decode_with_checksum(&encoded).expect("decode bool false failed");
    assert!(!decoded);
    assert_eq!(consumed, encoded.len());
}

// ---------------------------------------------------------------------------
// Test 8: empty string with checksum roundtrip
// ---------------------------------------------------------------------------
#[test]
fn test_checksum_empty_string_roundtrip() {
    let value = String::new();
    let encoded = encode_with_checksum(&value).expect("encode empty String failed");
    let (decoded, consumed): (String, _) =
        decode_with_checksum(&encoded).expect("decode empty String failed");
    assert_eq!(decoded, value);
    assert!(
        encoded.len() >= HEADER_SIZE,
        "encoded must contain at least the header"
    );
    assert_eq!(consumed, encoded.len());
}

// ---------------------------------------------------------------------------
// Test 9: large Vec<u8> (1000 bytes) with checksum
// ---------------------------------------------------------------------------
#[test]
fn test_checksum_large_vec_u8_roundtrip() {
    let value: Vec<u8> = (0u8..=255).cycle().take(1000).collect();
    let encoded = encode_with_checksum(&value).expect("encode large Vec<u8> failed");
    let (decoded, consumed): (Vec<u8>, _) =
        decode_with_checksum(&encoded).expect("decode large Vec<u8> failed");
    assert_eq!(decoded, value);
    assert_eq!(consumed, encoded.len());
}

// ---------------------------------------------------------------------------
// Test 10: checksum encoded bytes are longer than non-checksum encoded bytes
// ---------------------------------------------------------------------------
#[test]
fn test_checksum_encoded_longer_than_plain() {
    let value: u64 = 9_999_999;
    let plain = oxicode::encode_to_vec(&value).expect("plain encode failed");
    let checked = encode_with_checksum(&value).expect("checksum encode failed");
    assert!(
        checked.len() > plain.len(),
        "checksum-encoded ({} bytes) must exceed plain ({} bytes)",
        checked.len(),
        plain.len()
    );
    assert_eq!(
        checked.len(),
        plain.len() + HEADER_SIZE,
        "difference must be exactly HEADER_SIZE ({}) bytes",
        HEADER_SIZE
    );
}

// ---------------------------------------------------------------------------
// Test 11: encode u64::MAX with checksum
// ---------------------------------------------------------------------------
#[test]
fn test_checksum_u64_max_roundtrip() {
    let value = u64::MAX;
    let encoded = encode_with_checksum(&value).expect("encode u64::MAX failed");
    let (decoded, consumed): (u64, _) =
        decode_with_checksum(&encoded).expect("decode u64::MAX failed");
    assert_eq!(decoded, value);
    assert_eq!(consumed, encoded.len());
}

// ---------------------------------------------------------------------------
// Test 12: encode i64::MIN with checksum
// ---------------------------------------------------------------------------
#[test]
fn test_checksum_i64_min_roundtrip() {
    let value = i64::MIN;
    let encoded = encode_with_checksum(&value).expect("encode i64::MIN failed");
    let (decoded, consumed): (i64, _) =
        decode_with_checksum(&encoded).expect("decode i64::MIN failed");
    assert_eq!(decoded, value);
    assert_eq!(consumed, encoded.len());
}

// ---------------------------------------------------------------------------
// Test 13: Vec<String> with checksum roundtrip
// ---------------------------------------------------------------------------
#[test]
fn test_checksum_vec_string_roundtrip() {
    let value: Vec<String> = vec![
        "alpha".to_string(),
        "beta".to_string(),
        "gamma delta".to_string(),
        String::new(),
    ];
    let encoded = encode_with_checksum(&value).expect("encode Vec<String> failed");
    let (decoded, consumed): (Vec<String>, _) =
        decode_with_checksum(&encoded).expect("decode Vec<String> failed");
    assert_eq!(decoded, value);
    assert_eq!(consumed, encoded.len());
}

// ---------------------------------------------------------------------------
// Test 14: encode/decode u8 array [1,2,3,4] with checksum
// ---------------------------------------------------------------------------
#[test]
fn test_checksum_u8_array_roundtrip() {
    let value: [u8; 4] = [1, 2, 3, 4];
    let encoded = encode_with_checksum(&value).expect("encode [u8; 4] failed");
    let (decoded, consumed): ([u8; 4], _) =
        decode_with_checksum(&encoded).expect("decode [u8; 4] failed");
    assert_eq!(decoded, value);
    assert_eq!(consumed, encoded.len());
}

// ---------------------------------------------------------------------------
// Test 15: Option<String> Some with checksum
// ---------------------------------------------------------------------------
#[test]
fn test_checksum_option_string_some_roundtrip() {
    let value: Option<String> = Some("present value".to_string());
    let encoded = encode_with_checksum(&value).expect("encode Option::Some failed");
    let (decoded, consumed): (Option<String>, _) =
        decode_with_checksum(&encoded).expect("decode Option::Some failed");
    assert_eq!(decoded, value);
    assert_eq!(consumed, encoded.len());
}

// ---------------------------------------------------------------------------
// Test 16: Option<String> None with checksum
// ---------------------------------------------------------------------------
#[test]
fn test_checksum_option_string_none_roundtrip() {
    let value: Option<String> = None;
    let encoded = encode_with_checksum(&value).expect("encode Option::None failed");
    let (decoded, consumed): (Option<String>, _) =
        decode_with_checksum(&encoded).expect("decode Option::None failed");
    assert_eq!(decoded, value);
    assert_eq!(consumed, encoded.len());
}

// ---------------------------------------------------------------------------
// Test 17: nested struct with checksum
// ---------------------------------------------------------------------------
#[test]
fn test_checksum_nested_struct_roundtrip() {
    let value = NestedWrapper {
        label: "origin point".to_string(),
        inner: SimplePoint { x: 0.0, y: 0.0 },
    };
    let encoded = encode_with_checksum(&value).expect("encode NestedWrapper failed");
    let (decoded, consumed): (NestedWrapper, _) =
        decode_with_checksum(&encoded).expect("decode NestedWrapper failed");
    assert_eq!(decoded, value);
    assert_eq!(consumed, encoded.len());
}

// ---------------------------------------------------------------------------
// Test 18: enum with tuple variant with checksum
// ---------------------------------------------------------------------------
#[test]
fn test_checksum_enum_tuple_variant_roundtrip() {
    let value = TaggedEnum::Pair(42, u64::MAX / 2);
    let encoded = encode_with_checksum(&value).expect("encode TaggedEnum::Pair failed");
    let (decoded, consumed): (TaggedEnum, _) =
        decode_with_checksum(&encoded).expect("decode TaggedEnum::Pair failed");
    assert_eq!(decoded, value);
    assert_eq!(consumed, encoded.len());
}

// ---------------------------------------------------------------------------
// Test 19: f64 PI with checksum (bit-exact roundtrip)
// ---------------------------------------------------------------------------
#[test]
fn test_checksum_f64_pi_bit_exact_roundtrip() {
    let value = std::f64::consts::PI;
    let encoded = encode_with_checksum(&value).expect("encode f64 PI failed");
    let (decoded, consumed): (f64, _) =
        decode_with_checksum(&encoded).expect("decode f64 PI failed");
    // Bit-exact comparison via u64 representation
    assert_eq!(
        decoded.to_bits(),
        value.to_bits(),
        "f64 PI must survive a bit-exact roundtrip"
    );
    assert_eq!(consumed, encoded.len());
}

// ---------------------------------------------------------------------------
// Test 20: two consecutive checksum-encoded values in same buffer
// ---------------------------------------------------------------------------
#[test]
fn test_checksum_two_consecutive_values_in_same_buffer() {
    let first: u32 = 111;
    let second: u64 = 222_222_222_222;

    let mut buf = encode_with_checksum(&first).expect("encode first value failed");
    let second_bytes = encode_with_checksum(&second).expect("encode second value failed");
    buf.extend_from_slice(&second_bytes);

    // Decode first value from the start
    let (decoded_first, consumed_first): (u32, _) =
        decode_with_checksum(&buf).expect("decode first value failed");
    assert_eq!(decoded_first, first);

    // Decode second value starting at the byte offset returned for the first
    let remainder = &buf[consumed_first..];
    let (decoded_second, consumed_second): (u64, _) =
        decode_with_checksum(remainder).expect("decode second value failed");
    assert_eq!(decoded_second, second);
    assert_eq!(consumed_first + consumed_second, buf.len());
}

// ---------------------------------------------------------------------------
// Test 21: checksum bytes differ from non-checksum bytes for same value
// ---------------------------------------------------------------------------
#[test]
fn test_checksum_bytes_differ_from_plain_bytes() {
    let value: u32 = 42;
    let plain = oxicode::encode_to_vec(&value).expect("plain encode failed");
    let checked = encode_with_checksum(&value).expect("checksum encode failed");

    // The two buffers must not be byte-equal (checked has a 16-byte header)
    assert_ne!(
        plain.as_slice(),
        checked.as_slice(),
        "checksum-encoded bytes must differ from plain-encoded bytes"
    );
    // The payload embedded inside the checked buffer must equal the plain encoding
    let payload = verify_checksum(&checked).expect("verify_checksum failed on valid data");
    assert_eq!(
        payload,
        plain.as_slice(),
        "payload inside checksum buffer must equal plain encoding"
    );
}

// ---------------------------------------------------------------------------
// Test 22: u128 with checksum roundtrip
// ---------------------------------------------------------------------------
#[test]
fn test_checksum_u128_roundtrip() {
    let value: u128 = u128::MAX / 3;
    let encoded = encode_with_checksum(&value).expect("encode u128 failed");
    let (decoded, consumed): (u128, _) =
        decode_with_checksum(&encoded).expect("decode u128 failed");
    assert_eq!(decoded, value);
    assert_eq!(consumed, encoded.len());
}
