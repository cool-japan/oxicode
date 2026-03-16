//! Error handling and edge case tests

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
use oxicode::{Decode, Encode};

#[test]
fn test_truncated_data() {
    let value = String::from("hello world");
    let encoded = oxicode::encode_to_vec(&value).expect("encode failed");
    let truncated = &encoded[..encoded.len() / 2];
    let result: Result<(String, usize), _> = oxicode::decode_from_slice(truncated);
    assert!(result.is_err());
}

#[test]
fn test_invalid_bool_value() {
    let bytes = [2u8];
    let result: Result<(bool, usize), _> = oxicode::decode_from_slice(&bytes);
    assert!(result.is_err());
}

#[test]
fn test_invalid_enum_variant() {
    #[derive(Debug, PartialEq, Encode, Decode)]
    enum TwoVariants {
        A,
        B,
    }
    let bytes = oxicode::encode_to_vec(&255u32).expect("encode failed");
    let result: Result<(TwoVariants, usize), _> = oxicode::decode_from_slice(&bytes);
    assert!(result.is_err());
}

#[test]
fn test_empty_input() {
    let bytes: &[u8] = &[];
    let result: Result<(u32, usize), _> = oxicode::decode_from_slice(bytes);
    assert!(result.is_err());
}

#[test]
fn test_decode_option_none() {
    let value: Option<u32> = None;
    let encoded = oxicode::encode_to_vec(&value).expect("encode failed");
    let (decoded, _): (Option<u32>, _) =
        oxicode::decode_from_slice(&encoded).expect("decode failed");
    assert_eq!(decoded, None);
}

#[test]
fn test_decode_option_some() {
    let value: Option<u32> = Some(42);
    let encoded = oxicode::encode_to_vec(&value).expect("encode failed");
    let (decoded, _): (Option<u32>, _) =
        oxicode::decode_from_slice(&encoded).expect("decode failed");
    assert_eq!(decoded, Some(42));
}

#[test]
fn test_zero_length_string() {
    let value = String::new();
    let encoded = oxicode::encode_to_vec(&value).expect("encode failed");
    let (decoded, _): (String, _) = oxicode::decode_from_slice(&encoded).expect("decode failed");
    assert_eq!(decoded, "");
}

#[test]
fn test_large_collection() {
    let value: Vec<u32> = (0..10000).collect();
    let encoded = oxicode::encode_to_vec(&value).expect("encode failed");
    let (decoded, _): (Vec<u32>, _) = oxicode::decode_from_slice(&encoded).expect("decode failed");
    assert_eq!(value, decoded);
}

#[test]
fn test_decode_error_display_unexpected_variant() {
    // Encode an invalid discriminant manually and check the error message
    let bad_bytes = oxicode::encode_to_vec(&999u32).expect("encode discriminant");

    #[derive(Debug, oxicode::Decode)]
    enum Small {
        A,
        B,
        C,
    }

    let err = oxicode::decode_from_slice::<Small>(&bad_bytes).expect_err("should fail");
    let msg = format!("{}", err);
    assert!(
        msg.contains("999") || msg.contains("variant") || msg.contains("Small"),
        "Error message should be informative: {}",
        msg
    );
}

// --- Comprehensive DecodeError variant coverage ---

/// UnexpectedEnd: decode u32 from only 1 byte using fixed-int (legacy) config
/// which requires exactly 4 bytes; 1 byte must trigger UnexpectedEnd.
#[test]
fn test_unexpected_end_of_input_fixed_int() {
    let bytes = vec![0x01u8]; // only 1 byte; fixed u32 needs 4
    let config = oxicode::config::legacy();
    let result: Result<(u32, usize), _> = oxicode::decode_from_slice_with_config(&bytes, config);
    assert!(
        result.is_err(),
        "expected UnexpectedEnd error for truncated fixed u32"
    );
    let msg = format!("{}", result.unwrap_err());
    assert!(
        msg.to_lowercase().contains("unexpected")
            || msg.to_lowercase().contains("end")
            || msg.to_lowercase().contains("bytes"),
        "error message should mention unexpected end: {}",
        msg
    );
}

/// UnexpectedEnd: empty slice for u32
#[test]
fn test_decode_empty_slice_unexpected_end() {
    let bytes: Vec<u8> = vec![];
    let result: Result<(u32, usize), _> = oxicode::decode_from_slice(&bytes);
    assert!(
        result.is_err(),
        "expected error decoding u32 from empty slice"
    );
}

/// UnexpectedEnd: empty slice for i64
#[test]
fn test_decode_empty_slice_i64() {
    let bytes: Vec<u8> = vec![];
    let result: Result<(i64, usize), _> = oxicode::decode_from_slice(&bytes);
    assert!(
        result.is_err(),
        "expected error decoding i64 from empty slice"
    );
}

/// UnexpectedEnd: decode Vec<u8> where length prefix promises more data than available
#[test]
fn test_unexpected_end_vec_truncated_body() {
    // varint length = 100 (two-byte varint: 0xE4 0x00 = 100 in little-endian varint)
    // but provide zero body bytes
    let mut bytes = vec![];
    bytes.push(0xE4u8); // lower 7 bits = 100 & 0x7F = 0x64, msb set → more bytes
                        // Actually 100 = 0x64: single-byte varint since 0x64 < 0x80, so just 0x64
                        // Let's use 200 = 0xC8 0x01 two-byte varint
    bytes.clear();
    bytes.push(0xC8u8); // 200 & 0x7F | 0x80 = 0xC8
    bytes.push(0x01u8); // 200 >> 7 = 1
                        // no body bytes follow
    let result: Result<(Vec<u8>, usize), _> = oxicode::decode_from_slice(&bytes);
    assert!(
        result.is_err(),
        "expected error: length prefix > available data"
    );
}

/// InvalidBooleanValue: any byte other than 0 or 1 is invalid for bool
#[test]
fn test_invalid_boolean_value_two() {
    let bytes = [2u8];
    let result: Result<(bool, usize), _> = oxicode::decode_from_slice(&bytes);
    assert!(result.is_err());
    let msg = format!("{}", result.unwrap_err());
    assert!(
        msg.contains('2') || msg.to_lowercase().contains("bool"),
        "error should mention the bad boolean byte value: {}",
        msg
    );
}

#[test]
fn test_invalid_boolean_value_255() {
    let bytes = [255u8];
    let result: Result<(bool, usize), _> = oxicode::decode_from_slice(&bytes);
    assert!(result.is_err());
    let msg = format!("{}", result.unwrap_err());
    assert!(
        msg.contains("255") || msg.to_lowercase().contains("bool"),
        "error should mention byte 255: {}",
        msg
    );
}

/// Utf8: inject a valid varint length prefix followed by invalid UTF-8 bytes
#[test]
fn test_invalid_utf8_sequence() {
    let mut bytes = vec![];
    // length = 4 as varint (single byte 0x04 since 4 < 128)
    bytes.push(0x04u8);
    // 4 bytes of invalid UTF-8 (lone continuation bytes / overlong sequences)
    bytes.extend_from_slice(&[0xFF, 0xFE, 0xFD, 0xFC]);
    let result: Result<(String, usize), _> = oxicode::decode_from_slice(&bytes);
    assert!(
        result.is_err(),
        "decoding invalid UTF-8 bytes as String should fail"
    );
    let msg = format!("{}", result.unwrap_err());
    assert!(
        msg.to_lowercase().contains("utf") || msg.to_lowercase().contains("byte"),
        "error message should reference UTF-8: {}",
        msg
    );
}

#[test]
fn test_invalid_utf8_two_byte_prefix() {
    let mut bytes = vec![];
    // length = 3 (0x03)
    bytes.push(0x03u8);
    // 0x80 is a lone continuation byte — invalid UTF-8 start
    bytes.extend_from_slice(&[0x80, 0x81, 0x82]);
    let result: Result<(String, usize), _> = oxicode::decode_from_slice(&bytes);
    assert!(
        result.is_err(),
        "lone continuation bytes must fail UTF-8 decode"
    );
}

/// NonZeroTypeIsZero: encode a zero integer and decode as NonZero type
#[test]
fn test_nonzero_u32_zero_value() {
    use core::num::NonZeroU32;
    let zero_bytes = oxicode::encode_to_vec(&0u32).expect("encode zero");
    let result: Result<(NonZeroU32, usize), _> = oxicode::decode_from_slice(&zero_bytes);
    assert!(result.is_err(), "decoding 0 as NonZeroU32 must fail");
    let msg = format!("{}", result.unwrap_err());
    assert!(
        msg.to_lowercase().contains("zero") || msg.to_lowercase().contains("nonzero"),
        "error message should mention nonzero constraint: {}",
        msg
    );
}

#[test]
fn test_nonzero_u8_zero_value() {
    use core::num::NonZeroU8;
    let zero_bytes = oxicode::encode_to_vec(&0u8).expect("encode zero");
    let result: Result<(NonZeroU8, usize), _> = oxicode::decode_from_slice(&zero_bytes);
    assert!(result.is_err(), "decoding 0 as NonZeroU8 must fail");
}

#[test]
fn test_nonzero_i64_zero_value() {
    use core::num::NonZeroI64;
    let zero_bytes = oxicode::encode_to_vec(&0i64).expect("encode zero");
    let result: Result<(NonZeroI64, usize), _> = oxicode::decode_from_slice(&zero_bytes);
    assert!(result.is_err(), "decoding 0 as NonZeroI64 must fail");
}

/// UnexpectedVariant: encode discriminant index out of range for enum
#[test]
fn test_unexpected_variant_out_of_range() {
    #[derive(Debug, Encode, Decode)]
    enum SmallEnum {
        A,
        B,
        C,
    }
    // Encode discriminant 100 — does not exist in SmallEnum (only 0,1,2)
    let bad_disc_bytes = oxicode::encode_to_vec(&100u32).expect("encode discriminant");
    let result: Result<(SmallEnum, usize), _> = oxicode::decode_from_slice(&bad_disc_bytes);
    assert!(
        result.is_err(),
        "variant index 100 should not be valid for SmallEnum"
    );
    let msg = format!("{}", result.unwrap_err());
    assert!(
        msg.contains("100") || msg.to_lowercase().contains("variant"),
        "error should reference bad discriminant: {}",
        msg
    );
}

#[test]
fn test_unexpected_variant_single_variant_enum() {
    #[derive(Debug, Encode, Decode)]
    enum OneVariant {
        Only,
    }
    let bad_bytes = oxicode::encode_to_vec(&1u32).expect("encode");
    let result: Result<(OneVariant, usize), _> = oxicode::decode_from_slice(&bad_bytes);
    assert!(
        result.is_err(),
        "discriminant 1 out of range for single-variant enum"
    );
}

/// LimitExceeded: constructing it directly to verify the Display format
/// (Note: the LimitExceeded variant exists for use by custom Decoder impls that
/// enforce byte limits; the standard DecoderImpl does not currently enforce limits
/// at the decode path level.)
#[test]
fn test_limit_exceeded_direct_construction() {
    let err = oxicode::Error::LimitExceeded {
        limit: 100,
        found: 10_000,
    };
    let msg = format!("{}", err);
    assert!(
        msg.contains("100") && msg.contains("10000"),
        "Display should contain both limit and found values: {}",
        msg
    );
}

#[test]
fn test_limit_exceeded_direct_construction_large() {
    let err = oxicode::Error::LimitExceeded {
        limit: 50,
        found: 5_000,
    };
    let msg = format!("{}", err);
    assert!(
        msg.to_lowercase().contains("limit") || msg.contains("50"),
        "Display should mention limit: {}",
        msg
    );
}

/// InvalidData: constructing it directly and checking Display
#[test]
fn test_error_display_invalid_data() {
    let err = oxicode::Error::InvalidData {
        message: "test invalid data message",
    };
    let msg = format!("{}", err);
    assert!(
        msg.contains("test invalid data message"),
        "Display should include the message: {}",
        msg
    );
}

/// InvalidBooleanValue: constructing it directly and checking Display
#[test]
fn test_error_display_invalid_boolean_value() {
    let err = oxicode::Error::InvalidBooleanValue(42);
    let msg = format!("{}", err);
    assert!(
        msg.contains("42"),
        "Display should include the invalid byte value: {}",
        msg
    );
}

/// UnexpectedEnd: constructing it directly and checking Display
#[test]
fn test_error_display_unexpected_end() {
    let err = oxicode::Error::UnexpectedEnd { additional: 7 };
    let msg = format!("{}", err);
    assert!(
        msg.contains('7'),
        "Display should include the additional bytes needed: {}",
        msg
    );
}

/// OutsideUsizeRange: constructing it directly and checking Display
#[test]
fn test_error_display_outside_usize_range() {
    let err = oxicode::Error::OutsideUsizeRange(u64::MAX);
    let msg = format!("{}", err);
    assert!(
        msg.contains("usize") || msg.contains("range") || msg.to_lowercase().contains("outside"),
        "Display should mention usize range: {}",
        msg
    );
}

/// Custom: constructing it directly and checking Display
#[test]
fn test_error_display_custom() {
    let err = oxicode::Error::Custom {
        message: "my custom error",
    };
    let msg = format!("{}", err);
    assert_eq!(msg, "my custom error");
}

/// NonZeroTypeIsZero: constructing it directly and checking Display
#[test]
fn test_error_display_nonzero_type_is_zero() {
    let err = oxicode::Error::NonZeroTypeIsZero {
        non_zero_type: oxicode::error::IntegerType::U32,
    };
    let msg = format!("{}", err);
    assert!(
        msg.to_lowercase().contains("zero") && msg.to_lowercase().contains("u32"),
        "Display should mention type and zero: {}",
        msg
    );
}

/// UnexpectedVariant: constructing it directly and checking Display
#[test]
fn test_error_display_unexpected_variant() {
    let err = oxicode::Error::UnexpectedVariant {
        found: 99,
        type_name: "MyEnum",
    };
    let msg = format!("{}", err);
    assert!(
        msg.contains("99") && msg.contains("MyEnum"),
        "Display should contain discriminant and type name: {}",
        msg
    );
}

/// LimitExceeded: constructing it directly and checking Display
#[test]
fn test_error_display_limit_exceeded() {
    let err = oxicode::Error::LimitExceeded {
        limit: 1024,
        found: 65536,
    };
    let msg = format!("{}", err);
    assert!(
        msg.contains("1024") && msg.contains("65536"),
        "Display should contain both limit and found values: {}",
        msg
    );
}

/// InvalidCharEncoding: constructing it directly and checking Display
#[test]
fn test_error_display_invalid_char_encoding() {
    let err = oxicode::Error::InvalidCharEncoding([0xFF, 0xFE, 0xFD, 0xFC]);
    let msg = format!("{}", err);
    // Display should contain the byte values or similar
    assert!(
        !msg.is_empty(),
        "Display should produce a non-empty message for InvalidCharEncoding: {}",
        msg
    );
}

/// Checksum: construct ChecksumMismatch directly and check Display
#[cfg(feature = "checksum")]
#[test]
fn test_error_display_checksum_mismatch() {
    let err = oxicode::Error::ChecksumMismatch {
        expected: 0xDEAD_BEEF,
        found: 0xCAFE_BABE,
    };
    let msg = format!("{}", err);
    assert!(
        msg.to_lowercase().contains("checksum") || msg.contains("deadbeef"),
        "Display should mention checksum mismatch: {}",
        msg
    );
}

/// ChecksumMismatch via verify_checksum with a corrupted payload
#[cfg(feature = "checksum")]
#[test]
fn test_checksum_mismatch_corrupted_payload() {
    let original = b"hello checksum world";
    let mut wrapped = oxicode::checksum::wrap_with_checksum(original);
    // Flip the last byte to corrupt the payload/checksum
    let last_idx = wrapped.len() - 1;
    wrapped[last_idx] ^= 0xFF;
    let result = oxicode::checksum::verify_checksum(&wrapped);
    assert!(result.is_err(), "corrupted checksum should be detected");
    let msg = format!("{}", result.unwrap_err());
    assert!(
        msg.to_lowercase().contains("checksum"),
        "error message should mention checksum: {}",
        msg
    );
}

/// ChecksumMismatch: corrupt interior bytes of the payload (not the CRC footer)
#[cfg(feature = "checksum")]
#[test]
fn test_checksum_mismatch_corrupted_interior() {
    let original = b"interior corruption test data 123";
    let mut wrapped = oxicode::checksum::wrap_with_checksum(original);
    // Corrupt a byte in the middle of the payload (skip 4-byte magic header if any)
    if wrapped.len() > 8 {
        wrapped[4] ^= 0xAA;
    }
    let result = oxicode::checksum::verify_checksum(&wrapped);
    assert!(
        result.is_err(),
        "interior corruption should be detected by checksum"
    );
}

/// std-only: InvalidDuration — constructing it directly
#[cfg(feature = "std")]
#[test]
fn test_error_display_invalid_duration() {
    let err = oxicode::Error::InvalidDuration {
        secs: 42,
        nanos: 1_500_000_000, // >= 1e9, invalid
    };
    let msg = format!("{}", err);
    assert!(
        msg.to_lowercase().contains("duration") || msg.contains("42"),
        "Display should mention duration: {}",
        msg
    );
}

/// std-only: InvalidSystemTime — constructing it directly
#[cfg(feature = "std")]
#[test]
fn test_error_display_invalid_system_time() {
    let err = oxicode::Error::InvalidSystemTime {
        duration: std::time::Duration::from_secs(100),
    };
    let msg = format!("{}", err);
    assert!(
        msg.to_lowercase().contains("systemtime") || msg.to_lowercase().contains("unix"),
        "Display should mention SystemTime or UNIX_EPOCH: {}",
        msg
    );
}

/// std-only: IO error via From<std::io::Error>
#[cfg(feature = "std")]
#[test]
fn test_error_display_io_error() {
    use std::io;
    let io_err = io::Error::new(io::ErrorKind::NotFound, "file not found");
    let err: oxicode::Error = io_err.into();
    let msg = format!("{}", err);
    assert!(
        msg.to_lowercase().contains("io") || msg.to_lowercase().contains("not found"),
        "Display should reference IO error: {}",
        msg
    );
}

/// std-only: Error trait impl — ensure std::error::Error is implemented
#[cfg(feature = "std")]
#[test]
fn test_error_implements_std_error_trait() {
    fn assert_std_error<E: std::error::Error>() {}
    assert_std_error::<oxicode::Error>();
}

/// Encode overflow: encode a large Vec<u8> into a 10-byte fixed array — must fail
#[test]
fn test_encode_to_fixed_array_overflow() {
    let data: Vec<u8> = vec![0u8; 1000];
    let result: Result<([u8; 10], usize), _> = oxicode::encode_to_fixed_array(&data);
    assert!(
        result.is_err(),
        "encoding 1000 bytes into a 10-byte array must fail"
    );
}

/// Encode into a fixed array that is just barely too small by 1 byte
#[test]
fn test_encode_to_fixed_array_off_by_one() {
    // A u64 encodes to at most 9 bytes in varint; use a value that encodes to > 4 bytes
    // u64::MAX encodes to 9 bytes in standard (varint) config
    let result: Result<([u8; 4], usize), _> = oxicode::encode_to_fixed_array(&u64::MAX);
    assert!(
        result.is_err(),
        "u64::MAX cannot fit in 4 bytes with varint encoding"
    );
}

/// Encode into a fixed array that is exactly big enough — must succeed
#[test]
fn test_encode_to_fixed_array_exact_fit() {
    // u8 value 0 encodes to 1 varint byte
    let result: Result<([u8; 1], usize), _> = oxicode::encode_to_fixed_array(&0u8);
    assert!(
        result.is_ok(),
        "u8 should fit in 1 byte: {:?}",
        result.err()
    );
    let (_, n) = result.expect("fit");
    assert_eq!(n, 1);
}

/// OwnedCustom: constructing it directly (alloc feature) and checking Display
#[cfg(feature = "alloc")]
#[test]
fn test_error_display_owned_custom() {
    let err = oxicode::Error::OwnedCustom {
        message: String::from("dynamic owned error message"),
    };
    let msg = format!("{}", err);
    assert_eq!(msg, "dynamic owned error message");
}

/// From<Utf8Error>: conversion from core::str::Utf8Error to oxicode::Error
#[test]
fn test_from_utf8_error_conversion() {
    // Produce a genuine Utf8Error by calling from_utf8 on invalid bytes
    let invalid = &[0xFF, 0xFE][..];
    let utf8_err = core::str::from_utf8(invalid).expect_err("should fail");
    let oxicode_err: oxicode::Error = utf8_err.into();
    let msg = format!("{}", oxicode_err);
    // Depending on whether alloc feature is enabled, it is either Utf8 or InvalidData
    assert!(
        !msg.is_empty(),
        "Converted error should have non-empty Display: {}",
        msg
    );
}

/// Truncated varint: each byte has the continuation bit set but there is no
/// terminating byte, so the decoder must return an error rather than panic.
#[test]
fn test_corrupted_varint_truncated() {
    // All three bytes have the MSB set (continuation), but the sequence never
    // terminates — the decoder should surface an error.
    let truncated = &[0xFFu8, 0xFF, 0xFF];
    let result: Result<(u64, usize), _> = oxicode::decode_from_slice(truncated);
    assert!(result.is_err(), "truncated varint should return error");
}

/// Bool decode must reject any byte other than 0 or 1.
#[test]
fn test_bool_decode_rejects_invalid_value() {
    let invalid = &[2u8];
    let result: Result<(bool, usize), _> = oxicode::decode_from_slice(invalid);
    assert!(result.is_err(), "byte value 2 is not a valid bool");
}

/// Char decode must reject Unicode surrogate codepoints (0xD800–0xDFFF).
#[test]
fn test_char_decode_rejects_invalid_codepoint() {
    // 0xD800 is a UTF-16 high surrogate — not a valid Unicode scalar value.
    let invalid_char_codepoint = 0xD800u32;
    let enc = oxicode::encode_to_vec(&invalid_char_codepoint).expect("encode u32");
    let result: Result<(char, usize), _> = oxicode::decode_from_slice(&enc);
    assert!(
        result.is_err(),
        "surrogate codepoint 0xD800 must be rejected as char"
    );
}

// --- Error Display completeness: all variants produce non-empty strings ---

/// Verify that every constructible Error variant produces a non-empty Display string.
/// This test directly constructs each variant and checks the formatted output.
#[test]
fn test_all_error_variants_display_non_empty() {
    let variants: &[oxicode::Error] = &[
        oxicode::Error::UnexpectedEnd { additional: 4 },
        oxicode::Error::InvalidData {
            message: "some data problem",
        },
        oxicode::Error::InvalidIntegerType {
            expected: oxicode::error::IntegerType::U32,
            found: oxicode::error::IntegerType::I64,
        },
        oxicode::Error::InvalidBooleanValue(99),
        oxicode::Error::InvalidCharEncoding([0xF0, 0x28, 0x8C, 0xBC]),
        oxicode::Error::LimitExceeded {
            limit: 256,
            found: 512,
        },
        oxicode::Error::Custom {
            message: "custom msg",
        },
        oxicode::Error::OutsideUsizeRange(u64::MAX),
        oxicode::Error::NonZeroTypeIsZero {
            non_zero_type: oxicode::error::IntegerType::U64,
        },
        oxicode::Error::UnexpectedVariant {
            found: 42,
            type_name: "TestEnum",
        },
    ];
    for err in variants {
        let msg = format!("{}", err);
        assert!(
            !msg.is_empty(),
            "Error variant {:?} produced an empty Display string",
            err
        );
    }
}

/// Verify alloc-only variants (OwnedCustom, Utf8) produce non-empty Display strings.
#[cfg(feature = "alloc")]
#[test]
fn test_alloc_error_variants_display_non_empty() {
    let owned = oxicode::Error::OwnedCustom {
        message: String::from("owned error"),
    };
    assert!(!format!("{}", owned).is_empty());

    let invalid = &[0xFF, 0xFE][..];
    let utf8_err = core::str::from_utf8(invalid).expect_err("should fail");
    let utf8_variant: oxicode::Error = utf8_err.into();
    assert!(!format!("{}", utf8_variant).is_empty());
}

/// Verify std-only variants (Io, InvalidDuration, InvalidSystemTime) produce non-empty Display.
#[cfg(feature = "std")]
#[test]
fn test_std_error_variants_display_non_empty() {
    use std::io;

    let io_variant: oxicode::Error =
        io::Error::new(io::ErrorKind::BrokenPipe, "broken pipe").into();
    assert!(!format!("{}", io_variant).is_empty());

    let dur_variant = oxicode::Error::InvalidDuration {
        secs: 0,
        nanos: 2_000_000_000,
    };
    assert!(!format!("{}", dur_variant).is_empty());

    let st_variant = oxicode::Error::InvalidSystemTime {
        duration: std::time::Duration::from_secs(1),
    };
    assert!(!format!("{}", st_variant).is_empty());
}

/// Verify checksum variant produces non-empty Display.
#[cfg(feature = "checksum")]
#[test]
fn test_checksum_error_variant_display_non_empty() {
    let err = oxicode::Error::ChecksumMismatch {
        expected: 0x1234_5678,
        found: 0xDEAD_BEEF,
    };
    assert!(!format!("{}", err).is_empty());
}

// --- Pattern matching on error types ---

/// Verify that error variants can be matched with pattern matching.
#[test]
fn test_error_pattern_matching_unexpected_end() {
    let err = oxicode::Error::UnexpectedEnd { additional: 3 };
    match err {
        oxicode::Error::UnexpectedEnd { additional } => {
            assert_eq!(additional, 3);
        }
        other => panic!("unexpected variant: {:?}", other),
    }
}

#[test]
fn test_error_pattern_matching_invalid_boolean() {
    let err = oxicode::Error::InvalidBooleanValue(7);
    match err {
        oxicode::Error::InvalidBooleanValue(v) => {
            assert_eq!(v, 7);
        }
        other => panic!("unexpected variant: {:?}", other),
    }
}

#[test]
fn test_error_pattern_matching_unexpected_variant() {
    let err = oxicode::Error::UnexpectedVariant {
        found: 55,
        type_name: "SomeEnum",
    };
    match err {
        oxicode::Error::UnexpectedVariant { found, type_name } => {
            assert_eq!(found, 55);
            assert_eq!(type_name, "SomeEnum");
        }
        other => panic!("unexpected variant: {:?}", other),
    }
}

#[test]
fn test_error_pattern_matching_limit_exceeded() {
    let err = oxicode::Error::LimitExceeded {
        limit: 100,
        found: 200,
    };
    match err {
        oxicode::Error::LimitExceeded { limit, found } => {
            assert_eq!(limit, 100);
            assert_eq!(found, 200);
        }
        other => panic!("unexpected variant: {:?}", other),
    }
}

#[test]
fn test_error_pattern_matching_nonzero_is_zero() {
    let err = oxicode::Error::NonZeroTypeIsZero {
        non_zero_type: oxicode::error::IntegerType::I32,
    };
    match err {
        oxicode::Error::NonZeroTypeIsZero { non_zero_type } => {
            assert_eq!(non_zero_type, oxicode::error::IntegerType::I32);
        }
        other => panic!("unexpected variant: {:?}", other),
    }
}

#[test]
fn test_error_pattern_matching_custom() {
    let err = oxicode::Error::Custom {
        message: "pattern test",
    };
    match err {
        oxicode::Error::Custom { message } => {
            assert_eq!(message, "pattern test");
        }
        other => panic!("unexpected variant: {:?}", other),
    }
}

/// Verify that decode errors can be matched after a real decode failure.
#[test]
fn test_decode_error_matched_via_pattern() {
    let bytes = [2u8]; // invalid bool
    let result: Result<(bool, usize), _> = oxicode::decode_from_slice(&bytes);
    let err = result.expect_err("expected decode error");
    match err {
        oxicode::Error::InvalidBooleanValue(v) => {
            assert_eq!(v, 2, "InvalidBooleanValue should carry the byte value");
        }
        other => panic!("expected InvalidBooleanValue, got: {:?}", other),
    }
}

/// Verify UnexpectedEnd can be matched after a real decode failure on empty input.
#[test]
fn test_decode_unexpected_end_matched_via_pattern() {
    let result: Result<(u64, usize), _> = oxicode::decode_from_slice(&[]);
    let err = result.expect_err("expected decode error on empty slice");
    match err {
        oxicode::Error::UnexpectedEnd { .. } => {}
        other => panic!("expected UnexpectedEnd, got: {:?}", other),
    }
}

/// Verify that multiple error types are distinguishable at the match site.
#[test]
fn test_error_variants_are_distinct_in_match() {
    fn classify(err: &oxicode::Error) -> &'static str {
        match err {
            oxicode::Error::UnexpectedEnd { .. } => "unexpected_end",
            oxicode::Error::InvalidBooleanValue(_) => "invalid_bool",
            oxicode::Error::UnexpectedVariant { .. } => "unexpected_variant",
            oxicode::Error::LimitExceeded { .. } => "limit_exceeded",
            oxicode::Error::NonZeroTypeIsZero { .. } => "nonzero_is_zero",
            oxicode::Error::Custom { .. } => "custom",
            oxicode::Error::InvalidData { .. } => "invalid_data",
            oxicode::Error::InvalidIntegerType { .. } => "invalid_int_type",
            oxicode::Error::InvalidCharEncoding(_) => "invalid_char",
            oxicode::Error::OutsideUsizeRange(_) => "outside_usize",
            _ => "other",
        }
    }

    assert_eq!(
        classify(&oxicode::Error::UnexpectedEnd { additional: 1 }),
        "unexpected_end"
    );
    assert_eq!(
        classify(&oxicode::Error::InvalidBooleanValue(5)),
        "invalid_bool"
    );
    assert_eq!(
        classify(&oxicode::Error::LimitExceeded { limit: 1, found: 2 }),
        "limit_exceeded"
    );
    assert_eq!(classify(&oxicode::Error::Custom { message: "x" }), "custom");
}
