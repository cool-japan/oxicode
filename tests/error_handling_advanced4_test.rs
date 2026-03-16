//! Advanced error-handling tests for decode error scenarios in OxiCode.
//!
//! 22 test functions covering empty input, truncation, invalid encodings,
//! limit exceeded, bad discriminants, trailing bytes, Option, and double-decode.

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
    config, decode_from_slice, decode_from_slice_with_config, encode_to_vec, Decode, Encode,
};

// ---------------------------------------------------------------------------
// Helper enum used by tests 9 and related discriminant tests
// ---------------------------------------------------------------------------

#[derive(Debug, PartialEq, Encode, Decode)]
enum TwoVariant {
    A,
    B,
}

// ---------------------------------------------------------------------------
// 1. Empty slice returns error for u32
// ---------------------------------------------------------------------------

#[test]
fn test_empty_slice_returns_error_for_u32() {
    let result: Result<(u32, usize), _> = decode_from_slice(&[]);
    assert!(result.is_err(), "empty slice should fail for u32");
}

// ---------------------------------------------------------------------------
// 2. Empty slice returns error for String
// ---------------------------------------------------------------------------

#[test]
fn test_empty_slice_returns_error_for_string() {
    let result: Result<(String, usize), _> = decode_from_slice(&[]);
    assert!(result.is_err(), "empty slice should fail for String");
}

// ---------------------------------------------------------------------------
// 3. Truncated u32 (only 3 bytes of a 4-byte varint sequence) returns error
// ---------------------------------------------------------------------------

#[test]
fn test_truncated_u32_three_bytes_returns_error() {
    // Encode a large u32 that will produce several varint bytes, then truncate.
    let val: u32 = u32::MAX;
    let mut encoded = encode_to_vec(&val).expect("encode u32::MAX");
    // Ensure there are enough bytes to truncate meaningfully.
    assert!(
        encoded.len() >= 4,
        "u32::MAX should encode to at least 4 bytes; got {}",
        encoded.len()
    );
    encoded.truncate(3);
    let result: Result<(u32, usize), _> = decode_from_slice(&encoded);
    assert!(
        result.is_err(),
        "3-byte truncated u32::MAX encoding should fail"
    );
}

// ---------------------------------------------------------------------------
// 4. Truncated String (length says 10 but only 3 bytes present) returns error
// ---------------------------------------------------------------------------

#[test]
fn test_truncated_string_length_mismatch_returns_error() {
    // Manually construct: varint length 10, then only 3 bytes of content.
    let mut bytes: Vec<u8> = Vec::new();
    bytes.push(0x0A_u8); // length prefix = 10 (single-byte varint for values < 251)
    bytes.extend_from_slice(b"abc"); // only 3 bytes of the promised 10
    let result: Result<(String, usize), _> = decode_from_slice(&bytes);
    assert!(
        result.is_err(),
        "length prefix 10 with only 3 content bytes should fail"
    );
}

// ---------------------------------------------------------------------------
// 5. Invalid UTF-8 bytes for String returns error
// ---------------------------------------------------------------------------

#[test]
fn test_invalid_utf8_bytes_for_string_returns_error() {
    // varint length prefix 4 followed by 4 bytes that are not valid UTF-8.
    let mut bytes: Vec<u8> = Vec::new();
    bytes.push(0x04_u8); // length 4
    bytes.extend_from_slice(&[0xFF, 0xFE, 0xFD, 0xFC]);
    let result: Result<(String, usize), _> = decode_from_slice(&bytes);
    assert!(
        result.is_err(),
        "invalid UTF-8 sequence should fail String decode"
    );
}

// ---------------------------------------------------------------------------
// 6. Limit exceeded: String of 100 chars with limit of 50 bytes returns error
// ---------------------------------------------------------------------------

#[test]
fn test_limit_exceeded_string_100_chars_limit_50_returns_error() {
    let val = "x".repeat(100);
    let encoded = encode_to_vec(&val).expect("encode 100-char string");
    let cfg = config::standard().with_limit::<50>();
    let result: Result<(String, usize), _> = decode_from_slice_with_config(&encoded, cfg);
    assert!(
        result.is_err(),
        "decoding 100-char string with a 50-byte limit should fail"
    );
}

// ---------------------------------------------------------------------------
// 7. Limit exceeded: Vec<u8> of 200 items with limit of 100 returns error
// ---------------------------------------------------------------------------

#[test]
fn test_limit_exceeded_vec_u8_200_items_limit_100_returns_error() {
    let val: Vec<u8> = vec![0xAB_u8; 200];
    let encoded = encode_to_vec(&val).expect("encode Vec<u8> 200");
    let cfg = config::standard().with_limit::<100>();
    let result: Result<(Vec<u8>, usize), _> = decode_from_slice_with_config(&encoded, cfg);
    assert!(
        result.is_err(),
        "decoding Vec<u8> of 200 items with a 100-byte limit should fail"
    );
}

// ---------------------------------------------------------------------------
// 8. Invalid bool value (byte value 2) returns error
// ---------------------------------------------------------------------------

#[test]
fn test_invalid_bool_byte_2_returns_error() {
    let bad_bytes = [2u8];
    let result = decode_from_slice::<bool>(&bad_bytes);
    assert!(result.is_err(), "bool byte 2 should be an error");
}

// ---------------------------------------------------------------------------
// 9. Unknown enum variant discriminant returns error
// ---------------------------------------------------------------------------

#[test]
fn test_unknown_enum_variant_discriminant_returns_error() {
    // TwoVariant only has discriminants 0 (A) and 1 (B). Discriminant 5 is invalid.
    let bad_bytes = encode_to_vec(&5u32).expect("encode discriminant 5");
    let result = decode_from_slice::<TwoVariant>(&bad_bytes);
    assert!(
        result.is_err(),
        "discriminant 5 is not valid for TwoVariant"
    );
}

// ---------------------------------------------------------------------------
// 10. Successful u32 decode returns Ok variant
// ---------------------------------------------------------------------------

#[test]
fn test_successful_u32_decode_returns_ok() {
    let val: u32 = 42;
    let encoded = encode_to_vec(&val).expect("encode u32");
    let result: Result<(u32, usize), _> = decode_from_slice(&encoded);
    assert!(result.is_ok(), "valid u32 decode should return Ok");
    let (decoded, _) = result.expect("unwrap valid decode");
    assert_eq!(decoded, val);
}

// ---------------------------------------------------------------------------
// 11. Successful String decode returns Ok variant
// ---------------------------------------------------------------------------

#[test]
fn test_successful_string_decode_returns_ok() {
    let val = String::from("hello oxicode");
    let encoded = encode_to_vec(&val).expect("encode String");
    let result: Result<(String, usize), _> = decode_from_slice(&encoded);
    assert!(result.is_ok(), "valid String decode should return Ok");
    let (decoded, _) = result.expect("unwrap valid String decode");
    assert_eq!(decoded, val);
}

// ---------------------------------------------------------------------------
// 12. DecodeError can be formatted/displayed
// ---------------------------------------------------------------------------

#[test]
fn test_decode_error_can_be_formatted_and_displayed() {
    let result: Result<(u32, usize), _> = decode_from_slice(&[]);
    let err = result.expect_err("empty slice should be an error");
    let display_str = format!("{}", err);
    assert!(
        !display_str.is_empty(),
        "Display output for DecodeError must be non-empty; got: {:?}",
        display_str
    );
    let debug_str = format!("{:?}", err);
    assert!(
        !debug_str.is_empty(),
        "Debug output for DecodeError must be non-empty; got: {:?}",
        debug_str
    );
}

// ---------------------------------------------------------------------------
// 13. Zero-length decode: unit type () encodes to 0 bytes and consumed == 0
// ---------------------------------------------------------------------------

#[test]
fn test_zero_length_decode_unit_type_consumes_zero_bytes() {
    let val: () = ();
    let encoded = encode_to_vec(&val).expect("encode unit");
    // Unit encodes to 0 bytes.
    assert_eq!(encoded.len(), 0, "unit type must encode to 0 bytes");
    let (decoded, consumed): ((), usize) = decode_from_slice(&encoded).expect("decode unit");
    assert_eq!(decoded, val);
    assert_eq!(
        consumed, 0,
        "decoding unit from empty slice must consume 0 bytes"
    );
}

// ---------------------------------------------------------------------------
// 14. Overly large length prefix for Vec returns error (large value, small limit)
// ---------------------------------------------------------------------------

#[test]
fn test_overly_large_length_prefix_for_vec_returns_error() {
    // Encode a Vec<u8> with a huge claimed length but use a tight limit so the
    // decoder rejects the length claim before attempting any allocation.
    // We write a varint length of 1_000_000 followed by no content bytes.
    // With a limit of 512 bytes the length claim itself already exceeds the limit,
    // so the decode must fail with LimitExceeded (or UnexpectedEnd if the limit
    // check happens after content read — either is acceptable).
    let huge_len: u64 = 1_000_000_u64;
    let length_bytes = encode_to_vec(&huge_len).expect("encode huge length prefix");
    let cfg = config::standard().with_limit::<512>();
    let result: Result<(Vec<u8>, usize), _> = decode_from_slice_with_config(&length_bytes, cfg);
    assert!(
        result.is_err(),
        "claimed Vec length of 1_000_000 with a 512-byte limit and no content must fail"
    );
}

// ---------------------------------------------------------------------------
// 15. Overly large length prefix for String returns error
// ---------------------------------------------------------------------------

#[test]
fn test_overly_large_length_prefix_for_string_returns_error() {
    // Same approach as test 14 but decoded as String.
    let huge_len: u64 = 1_000_000_u64;
    let length_bytes = encode_to_vec(&huge_len).expect("encode huge string length prefix");
    let cfg = config::standard().with_limit::<512>();
    let result: Result<(String, usize), _> = decode_from_slice_with_config(&length_bytes, cfg);
    assert!(
        result.is_err(),
        "claimed String length of 1_000_000 with a 512-byte limit and no content must fail"
    );
}

// ---------------------------------------------------------------------------
// 16. Wrong type decode: u8 bytes decoded as u64 — check no panic, handle truncation
// ---------------------------------------------------------------------------

#[test]
fn test_wrong_type_decode_u8_as_u64_no_panic_handles_truncation() {
    // Encode a single u8 (1 byte on wire) and try to decode as u64.
    // The u64 varint decoder may succeed (the single byte is a valid 1-byte varint)
    // or fail, but must never panic.
    let val: u8 = 0x7F;
    let encoded = encode_to_vec(&val).expect("encode u8");
    let result: Result<(u64, usize), _> = decode_from_slice(&encoded);
    // We only require no panic.  If it succeeds, the decoded u64 value must be
    // logically consistent with the single-byte varint semantics.
    match result {
        Ok((decoded, consumed)) => {
            assert_eq!(
                consumed,
                encoded.len(),
                "consumed bytes must match encoded length"
            );
            // The single-byte varint 0x7F represents 127.
            assert_eq!(
                decoded, 0x7F_u64,
                "decoded u64 from single-byte varint 0x7F should be 127"
            );
        }
        Err(_) => {
            // Also acceptable — some configs may reject this.
        }
    }
}

// ---------------------------------------------------------------------------
// 17. Decode with tight limit — just enough bytes passes
// ---------------------------------------------------------------------------

#[test]
fn test_decode_tight_limit_just_enough_bytes_passes() {
    // Encode a Vec<u8> of exactly 8 bytes of content.
    // Use a limit of 8, which is the exact content length.
    let val: Vec<u8> = vec![1u8, 2, 3, 4, 5, 6, 7, 8];
    let encoded = encode_to_vec(&val).expect("encode Vec<u8> 8");
    let cfg = config::standard().with_limit::<8>();
    let result: Result<(Vec<u8>, usize), _> = decode_from_slice_with_config(&encoded, cfg);
    assert!(
        result.is_ok(),
        "Vec<u8> of exactly 8 bytes with a limit of 8 should succeed; got: {:?}",
        result.err()
    );
    let (decoded, _) = result.expect("decode tight limit 8");
    assert_eq!(decoded, val);
}

// ---------------------------------------------------------------------------
// 18. Decode with tight limit — one byte too few fails
// ---------------------------------------------------------------------------

#[test]
fn test_decode_tight_limit_one_byte_too_few_fails() {
    // Encode a Vec<u8> of 9 bytes of content but use a limit of 8.
    let val: Vec<u8> = vec![1u8, 2, 3, 4, 5, 6, 7, 8, 9];
    let encoded = encode_to_vec(&val).expect("encode Vec<u8> 9");
    let cfg = config::standard().with_limit::<8>();
    let result: Result<(Vec<u8>, usize), _> = decode_from_slice_with_config(&encoded, cfg);
    assert!(
        result.is_err(),
        "Vec<u8> of 9 bytes with a limit of 8 should fail"
    );
}

// ---------------------------------------------------------------------------
// 19. Trailing bytes after valid decode: consumed != total but no error
// ---------------------------------------------------------------------------

#[test]
fn test_trailing_bytes_after_valid_decode_consumed_less_than_total() {
    let val: u32 = 99;
    let mut encoded = encode_to_vec(&val).expect("encode u32");
    let original_len = encoded.len();
    // Append some extra bytes.
    encoded.extend_from_slice(&[0xAA, 0xBB, 0xCC]);
    let result: Result<(u32, usize), _> = decode_from_slice(&encoded);
    assert!(result.is_ok(), "valid prefix should decode without error");
    let (decoded, consumed) = result.expect("decode with trailing bytes");
    assert_eq!(decoded, val);
    assert_eq!(
        consumed, original_len,
        "consumed must equal the original encoded length, not the total with trailing bytes"
    );
    assert!(
        consumed < encoded.len(),
        "trailing bytes must remain unconsumed (consumed={} < total={})",
        consumed,
        encoded.len()
    );
}

// ---------------------------------------------------------------------------
// 20. Vec<u8> with length 0 decodes to empty vec — no error
// ---------------------------------------------------------------------------

#[test]
fn test_vec_u8_length_zero_decodes_to_empty_vec() {
    let val: Vec<u8> = Vec::new();
    let encoded = encode_to_vec(&val).expect("encode empty Vec<u8>");
    let result: Result<(Vec<u8>, usize), _> = decode_from_slice(&encoded);
    assert!(result.is_ok(), "empty Vec<u8> should decode without error");
    let (decoded, _) = result.expect("decode empty Vec<u8>");
    assert!(decoded.is_empty(), "decoded Vec<u8> must be empty");
}

// ---------------------------------------------------------------------------
// 21. Option None (byte 0) decodes as None — no error
// ---------------------------------------------------------------------------

#[test]
fn test_option_none_byte_zero_decodes_as_none() {
    let val: Option<u32> = None;
    let encoded = encode_to_vec(&val).expect("encode Option None");
    // None encodes as a single 0x00 byte.
    assert_eq!(encoded, vec![0x00_u8], "None should encode as [0x00]");
    let result: Result<(Option<u32>, usize), _> = decode_from_slice(&encoded);
    assert!(
        result.is_ok(),
        "Option None byte 0 should decode without error"
    );
    let (decoded, consumed) = result.expect("decode Option None");
    assert_eq!(decoded, None);
    assert_eq!(consumed, 1, "decoding None must consume exactly 1 byte");
}

// ---------------------------------------------------------------------------
// 22. Double-decode: use consumed bytes to decode second value from same slice
// ---------------------------------------------------------------------------

#[test]
fn test_double_decode_use_consumed_bytes_to_decode_second_value() {
    // Encode two distinct u32 values into a single contiguous buffer.
    let first: u32 = 1111;
    let second: u32 = 2222;
    let mut buffer: Vec<u8> = Vec::new();
    buffer.extend(encode_to_vec(&first).expect("encode first"));
    let split = buffer.len(); // boundary between first and second
    buffer.extend(encode_to_vec(&second).expect("encode second"));

    // Decode the first value and verify consumed == split.
    let (decoded_first, consumed_first): (u32, usize) =
        decode_from_slice(&buffer).expect("decode first value");
    assert_eq!(decoded_first, first, "first decoded value must match");
    assert_eq!(
        consumed_first, split,
        "first decode must consume exactly the first segment"
    );

    // Decode the second value from the remainder.
    let (decoded_second, consumed_second): (u32, usize) =
        decode_from_slice(&buffer[consumed_first..]).expect("decode second value");
    assert_eq!(decoded_second, second, "second decoded value must match");
    assert_eq!(
        consumed_first + consumed_second,
        buffer.len(),
        "both decodes together must consume all bytes"
    );
}
