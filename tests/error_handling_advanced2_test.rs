//! Advanced error handling and decode failure tests for OxiCode — set 2.
//!
//! Covers 22 distinct scenarios: empty input, truncation, limit enforcement,
//! invalid type bytes, garbage data, error display, Result discriminants,
//! struct underflow, byte corruption, and trailing-byte tolerance.

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
use oxicode::error::Error as DecodeError;
use oxicode::{
    config, decode_from_slice, decode_from_slice_with_config, encode_to_vec, Decode, Encode,
};

// ─── Test 1 ──────────────────────────────────────────────────────────────────
// Decode from empty slice returns Err (UnexpectedEnd)

#[test]
fn test_decode_empty_slice_returns_err() {
    let result: Result<(u32, usize), DecodeError> = decode_from_slice(&[]);
    assert!(
        result.is_err(),
        "decoding u32 from empty slice must return Err (UnexpectedEnd)"
    );
}

// ─── Test 2 ──────────────────────────────────────────────────────────────────
// Decode truncated u32 (only 2 bytes of a value requiring more) returns Err

#[test]
fn test_decode_truncated_u32_returns_err() {
    // u32::MAX requires multiple varint bytes; truncate to 2 bytes
    let large: u32 = u32::MAX;
    let encoded = encode_to_vec(&large).expect("encode u32::MAX failed");
    // u32::MAX in oxicode varint encodes as 5 bytes; take only the first 2
    let truncated = &encoded[..2.min(encoded.len() - 1)];
    let result: Result<(u32, usize), DecodeError> = decode_from_slice(truncated);
    assert!(
        result.is_err(),
        "decoding u32::MAX from 2-byte slice must return Err"
    );
}

// ─── Test 3 ──────────────────────────────────────────────────────────────────
// Decode truncated String (length prefix present but no payload data) returns Err

#[test]
fn test_decode_truncated_string_length_prefix_only_returns_err() {
    // Manually craft: varint(10) — claims 10 bytes but provides none
    let crafted: &[u8] = &[0x0A]; // varint 10 as string length, no data follows
    let result: Result<(String, usize), DecodeError> = decode_from_slice(crafted);
    assert!(
        result.is_err(),
        "String with length prefix 10 but no payload must return Err"
    );
}

// ─── Test 4 ──────────────────────────────────────────────────────────────────
// Decode with limit exceeded using with_limit::<8>() and large data fails

#[test]
fn test_decode_limit_exceeded_returns_err() {
    // Encode a 50-element Vec<u8>; decoder claims 50 bytes which exceeds limit of 8
    let large_vec: Vec<u8> = vec![0xBBu8; 50];
    let encoded = encode_to_vec(&large_vec).expect("encode Vec<u8> failed");
    let cfg = config::standard().with_limit::<8>();
    let result: Result<(Vec<u8>, usize), DecodeError> =
        decode_from_slice_with_config(&encoded, cfg);
    assert!(
        result.is_err(),
        "decoding 50-byte Vec with limit=8 must return Err"
    );
}

// ─── Test 5 ──────────────────────────────────────────────────────────────────
// Decode with valid but wrong type (u32 bytes decoded as bool) — may succeed or fail
// This test documents the behavior without asserting a specific outcome

#[test]
fn test_decode_u32_bytes_as_bool_does_not_panic() {
    // Encode a small u32 (0 or 1) — these happen to be valid bool encodings too
    let val: u32 = 1;
    let encoded = encode_to_vec(&val).expect("encode u32 failed");
    // Attempt decode as bool — should not panic regardless of outcome
    let result: Result<(bool, usize), DecodeError> = decode_from_slice(&encoded);
    // Byte 0x01 is a valid bool (true); decode may succeed or extra bytes make it fail
    // The important invariant: no panic
    let _ = result;
}

// ─── Test 6 ──────────────────────────────────────────────────────────────────
// Invalid bool byte (value 2) returns Err (InvalidIntegerType or similar)

#[test]
fn test_invalid_bool_byte_returns_err() {
    let bad_bytes = [2u8]; // 0 = false, 1 = true, 2+ = invalid
    let result: Result<(bool, usize), DecodeError> = decode_from_slice(&bad_bytes);
    assert!(result.is_err(), "invalid bool byte 2 should return error");
}

// ─── Test 7 ──────────────────────────────────────────────────────────────────
// Decode from slice with only 0 bytes (a single zero byte) for a u32 returns Ok(0)
// (because varint 0x00 is a complete valid encoding for 0)

#[test]
fn test_decode_single_zero_byte_for_u32_returns_zero() {
    let bytes: &[u8] = &[0x00u8];
    let result: Result<(u32, usize), DecodeError> = decode_from_slice(bytes);
    assert!(
        result.is_ok(),
        "single zero byte must decode as u32=0 (valid varint)"
    );
    let (val, consumed) = result.expect("decode single 0x00 byte as u32");
    assert_eq!(val, 0u32, "varint 0x00 must decode to 0");
    assert_eq!(consumed, 1, "one byte consumed for varint 0");
}

// ─── Test 8 ──────────────────────────────────────────────────────────────────
// Encode success then decode success (baseline good path)

#[test]
fn test_encode_then_decode_baseline_roundtrip() {
    let original: u64 = 0xCAFE_BABE_DEAD_BEEFu64;
    let encoded = encode_to_vec(&original).expect("encode u64 failed");
    let (decoded, consumed): (u64, usize) = decode_from_slice(&encoded).expect("decode u64 failed");
    assert_eq!(decoded, original, "roundtrip must preserve u64 value");
    assert_eq!(
        consumed,
        encoded.len(),
        "all encoded bytes must be consumed"
    );
}

// ─── Test 9 ──────────────────────────────────────────────────────────────────
// Decode garbage bytes for String returns Err (UTF-8 error or unexpected end)

#[test]
fn test_decode_garbage_bytes_as_string_returns_err() {
    // varint(5) followed by 5 bytes of invalid UTF-8
    let garbage: &[u8] = &[0x05u8, 0xF8, 0xF9, 0xFA, 0xFB, 0xFC];
    let result: Result<(String, usize), DecodeError> = decode_from_slice(garbage);
    assert!(
        result.is_err(),
        "garbage bytes that violate UTF-8 must return Err when decoded as String"
    );
}

// ─── Test 10 ─────────────────────────────────────────────────────────────────
// DecodeError::UnexpectedEnd can be matched via pattern

#[test]
fn test_decode_error_unexpected_end_can_be_matched() {
    let result: Result<(u64, usize), DecodeError> = decode_from_slice(&[]);
    match result {
        Err(e) => {
            // Confirm the error formats as a non-empty string (UnexpectedEnd variant)
            let msg = format!("{e}");
            assert!(
                !msg.is_empty(),
                "DecodeError Display must produce a non-empty message"
            );
        }
        Ok(_) => panic!("expected Err for empty slice, got Ok"),
    }
}

// ─── Test 11 ─────────────────────────────────────────────────────────────────
// Decode with limit config — just barely fits: succeeds

#[test]
fn test_decode_with_limit_just_barely_fits_succeeds() {
    // A u8 value < 128 encodes as exactly 1 varint byte; limit=1 is exactly enough
    let val: u8 = 77;
    let encoded = encode_to_vec(&val).expect("encode u8 failed");
    assert_eq!(encoded.len(), 1, "u8=77 must encode to exactly 1 byte");
    let cfg = config::standard().with_limit::<1>();
    let (decoded, _): (u8, usize) =
        decode_from_slice_with_config(&encoded, cfg).expect("decode with exact limit failed");
    assert_eq!(decoded, val, "value must survive exact-limit decode");
}

// ─── Test 12 ─────────────────────────────────────────────────────────────────
// Decode with limit config — one byte over limit: fails

#[test]
fn test_decode_with_limit_one_byte_over_fails() {
    // Encode a Vec<u8> with exactly 20 elements; the decoder claims 20 bytes.
    // A limit of 19 is one short and must cause failure.
    let vec20: Vec<u8> = (0u8..20).collect();
    let encoded = encode_to_vec(&vec20).expect("encode Vec<u8>[20] failed");
    let cfg = config::standard().with_limit::<19>();
    let result: Result<(Vec<u8>, usize), DecodeError> =
        decode_from_slice_with_config(&encoded, cfg);
    assert!(
        result.is_err(),
        "limit=19 for a 20-element Vec<u8> must return Err"
    );
}

// ─── Test 13 ─────────────────────────────────────────────────────────────────
// Decode Vec with stated length longer than remaining bytes: Err

#[test]
fn test_decode_vec_stated_length_exceeds_remaining_bytes_returns_err() {
    // varint(200) claims 200 elements; only 3 bytes of payload follow
    // varint 200 is a single byte (0xC8 = 200 in raw, but oxicode uses a different encoding)
    // Use a crafted raw varint: value 200 = 0xC8 which needs multi-byte in oxicode's scheme
    // Instead, encode 200u64 and use those bytes as the length prefix for Vec<u8>
    let len_bytes = encode_to_vec(&200u64).expect("encode length 200");
    let mut crafted = len_bytes;
    crafted.extend_from_slice(&[0x01u8, 0x02u8, 0x03u8]); // only 3 actual bytes
    let result: Result<(Vec<u8>, usize), DecodeError> = decode_from_slice(&crafted);
    assert!(
        result.is_err(),
        "Vec with stated length 200 but only 3 payload bytes must return Err"
    );
}

// ─── Test 14 ─────────────────────────────────────────────────────────────────
// Truncated Vec<String> (partial entry): Err

#[test]
fn test_decode_truncated_vec_of_strings_returns_err() {
    let original: Vec<String> = vec![
        "hello".to_string(),
        "world".to_string(),
        "foo bar baz qux quux".to_string(),
    ];
    let encoded = encode_to_vec(&original).expect("encode Vec<String> failed");
    // Truncate to first quarter to cut off in the middle of an entry
    let truncated = &encoded[..encoded.len() / 4];
    let result: Result<(Vec<String>, usize), DecodeError> = decode_from_slice(truncated);
    assert!(
        result.is_err(),
        "truncated Vec<String> (quarter of bytes) must return Err"
    );
}

// ─── Test 15 ─────────────────────────────────────────────────────────────────
// Decode from single byte 0xFF for u32 — 0xFF is not a complete varint (it's a
// multi-byte marker), so decode fails with UnexpectedEnd

#[test]
fn test_decode_single_byte_0xff_for_u32_returns_err() {
    let bytes: &[u8] = &[0xFFu8];
    let result: Result<(u32, usize), DecodeError> = decode_from_slice(bytes);
    // 0xFF is a varint continuation marker needing more bytes — must fail
    assert!(
        result.is_err(),
        "single byte 0xFF is an incomplete varint and must return Err for u32"
    );
}

// ─── Test 16 ─────────────────────────────────────────────────────────────────
// Encode large struct, truncate to half, decode returns Err

#[derive(Debug, PartialEq, Encode, Decode)]
struct LargePayloadStruct {
    id: u64,
    name: String,
    tags: Vec<String>,
    score: f64,
}

#[test]
fn test_encode_large_struct_truncated_to_half_returns_err() {
    let value = LargePayloadStruct {
        id: 0xABCD_EF01_2345_6789u64,
        name: "a moderately long struct name".to_string(),
        tags: vec!["alpha".to_string(), "beta".to_string(), "gamma".to_string()],
        score: std::f64::consts::E,
    };
    let encoded = encode_to_vec(&value).expect("encode LargePayloadStruct failed");
    let half = encoded.len() / 2;
    let truncated = &encoded[..half];
    let result: Result<(LargePayloadStruct, usize), DecodeError> = decode_from_slice(truncated);
    assert!(
        result.is_err(),
        "decoding large struct truncated to half its bytes must return Err"
    );
}

// ─── Test 17 ─────────────────────────────────────────────────────────────────
// Error message contains useful information (Display impl is non-trivial)

#[test]
fn test_decode_error_display_contains_useful_info() {
    // Trigger a UTF-8 error by feeding invalid bytes for a String
    let bad: &[u8] = &[0x03u8, 0xFF, 0xFE, 0xFD]; // length=3, then 3 invalid UTF-8 bytes
    let result: Result<(String, usize), DecodeError> = decode_from_slice(bad);
    let err = result.expect_err("expected Err for invalid UTF-8 bytes");
    let display = format!("{err}");
    assert!(
        !display.is_empty(),
        "error Display implementation must produce a non-empty message; got empty string"
    );
}

// ─── Test 18 ─────────────────────────────────────────────────────────────────
// Decode of Result<u32, String> with invalid discriminant (> 1) returns Err

#[test]
fn test_decode_result_invalid_discriminant_returns_err() {
    // Result<u32, String> encodes discriminant: 0 = Ok, 1 = Err.
    // Discriminant 2 is invalid and must fail.
    let bad_discriminant = encode_to_vec(&2u32).expect("encode discriminant 2");
    let result: Result<(Result<u32, String>, usize), DecodeError> =
        decode_from_slice(&bad_discriminant);
    assert!(
        result.is_err(),
        "discriminant 2 must return Err when decoding Result<u32, String>"
    );
}

// ─── Test 19 ─────────────────────────────────────────────────────────────────
// Decode struct with fewer bytes than needed returns Err

#[derive(Debug, PartialEq, Encode, Decode)]
struct ThreeFieldStruct {
    a: u64,
    b: u64,
    c: u64,
}

#[test]
fn test_decode_struct_with_insufficient_bytes_returns_err() {
    // A ThreeFieldStruct needs 3 × (1–9 bytes each) at minimum; give only 2 bytes
    let too_few: &[u8] = &[0x01u8, 0x02u8];
    let result: Result<(ThreeFieldStruct, usize), DecodeError> = decode_from_slice(too_few);
    assert!(
        result.is_err(),
        "2 bytes is not enough for ThreeFieldStruct (needs at least 3 bytes for 3 fields)"
    );
}

// ─── Test 20 ─────────────────────────────────────────────────────────────────
// Two-step: valid encode, corrupt one byte, decode returns Err or wrong value

#[test]
fn test_corrupt_one_byte_decode_returns_err_or_different_value() {
    let original: u32 = 0xDEAD_BEEFu32;
    let mut encoded = encode_to_vec(&original).expect("encode u32 failed");
    // Flip the high bit of the first byte — corrupts the varint
    encoded[0] ^= 0x80;
    let result: Result<(u32, usize), DecodeError> = decode_from_slice(&encoded);
    match result {
        Err(_) => { /* corruption caused a decode error — expected */ }
        Ok((decoded, _)) => {
            assert_ne!(
                decoded, original,
                "corrupted varint must not silently decode to the original value"
            );
        }
    }
}

// ─── Test 21 ─────────────────────────────────────────────────────────────────
// Bytes slice &[u8] decode with exactly right number of bytes succeeds

#[test]
fn test_decode_bytes_slice_exact_bytes_succeeds() {
    let original: Vec<u8> = vec![0xAA, 0xBB, 0xCC, 0xDD, 0xEE];
    let encoded = encode_to_vec(&original).expect("encode Vec<u8> failed");
    // Decode from exactly the encoded bytes — must succeed with the original value
    let (decoded, consumed): (Vec<u8>, usize) =
        decode_from_slice(&encoded).expect("decode Vec<u8> from exact bytes failed");
    assert_eq!(decoded, original, "decoded value must match original");
    assert_eq!(
        consumed,
        encoded.len(),
        "all encoded bytes must be consumed"
    );
}

// ─── Test 22 ─────────────────────────────────────────────────────────────────
// Bytes slice &[u8] decode with one extra trailing byte succeeds and consumed < total

#[test]
fn test_decode_bytes_slice_with_trailing_byte_succeeds_and_consumed_less_than_total() {
    let original: Vec<u8> = vec![0x11, 0x22, 0x33];
    let mut buffer = encode_to_vec(&original).expect("encode Vec<u8> failed");
    let original_len = buffer.len();
    buffer.push(0xFFu8); // append one extra trailing byte
    let (decoded, consumed): (Vec<u8>, usize) =
        decode_from_slice(&buffer).expect("decode Vec<u8> with trailing byte failed");
    assert_eq!(
        decoded, original,
        "value must be preserved despite trailing byte"
    );
    assert_eq!(
        consumed, original_len,
        "consumed must equal the original encoded length, not include the trailing byte"
    );
    assert!(
        consumed < buffer.len(),
        "consumed ({consumed}) must be strictly less than total buffer length ({})",
        buffer.len()
    );
}
