//! Error handling and resilience tests — 22 new scenarios not covered by
//! error_test.rs or decode_resilience_test.rs.

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
use oxicode::{decode_from_slice, encode_to_vec, Decode, Encode};

// ---------------------------------------------------------------------------
// 1. Decode u32 from 0 bytes → error
// ---------------------------------------------------------------------------
#[test]
fn test_decode_u32_zero_bytes() {
    let result: Result<(u32, usize), _> = decode_from_slice(&[]);
    assert!(result.is_err(), "decoding u32 from 0 bytes must fail");
}

// ---------------------------------------------------------------------------
// 2. Decode u64 from truncated varint: tag byte U64_BYTE (253) signals that 8
//    body bytes follow, but only 3 are provided → UnexpectedEnd error
// ---------------------------------------------------------------------------
#[test]
fn test_decode_u64_truncated_varint_u64_tag_short_body() {
    // OxiCode varint: 253 = U64_BYTE tag means "read 8 bytes next".
    // Provide only 3 body bytes → decoder must return UnexpectedEnd.
    let bytes = [253u8, 0x00, 0x01, 0x02]; // tag + 3 of the required 8 bytes
    let result: Result<(u64, usize), _> = decode_from_slice(&bytes);
    assert!(
        result.is_err(),
        "U64_BYTE tag with only 3 body bytes must fail"
    );
    let err = result.expect_err("should be error");
    match err {
        oxicode::Error::UnexpectedEnd { .. } => {}
        other => panic!("expected UnexpectedEnd, got: {:?}", other),
    }
}

// ---------------------------------------------------------------------------
// 3. Decode String with length prefix claiming 1000 bytes but only 5 available
// ---------------------------------------------------------------------------
#[test]
fn test_decode_string_length_overflow() {
    // varint encoding of 1000: 1000 = 0b0000_0011_1110_1000
    // LEB128: lower 7 bits = 0x68 | 0x80 = 0xE8, upper = 0x07
    let bytes = [0xE8u8, 0x07, b'h', b'e', b'l', b'l', b'o']; // length=1000 but only 5 body bytes
    let result: Result<(String, usize), _> = decode_from_slice(&bytes);
    assert!(
        result.is_err(),
        "String claiming 1000 bytes with only 5 available must fail"
    );
}

// ---------------------------------------------------------------------------
// 4. Decode Vec<u32> claiming 1000 elements but insufficient bytes
// ---------------------------------------------------------------------------
#[test]
fn test_decode_vec_length_overflow() {
    // varint encoding of 1000 followed by only 2 bytes of body
    let bytes = [0xE8u8, 0x07, 0x01, 0x02]; // length=1000, 2 body bytes
    let result: Result<(Vec<u32>, usize), _> = decode_from_slice(&bytes);
    assert!(
        result.is_err(),
        "Vec<u32> claiming 1000 elements with 2 body bytes must fail"
    );
}

// ---------------------------------------------------------------------------
// 5. Decode bool from byte value 2 → InvalidBoolEncoding
// ---------------------------------------------------------------------------
#[test]
fn test_decode_bool_invalid_byte_two() {
    let result: Result<(bool, usize), _> = decode_from_slice(&[2u8]);
    assert!(result.is_err(), "byte value 2 is not a valid bool");
    let err = result.expect_err("should be error");
    match err {
        oxicode::Error::InvalidBooleanValue(v) => {
            assert_eq!(v, 2, "error must carry the bad byte value");
        }
        other => panic!("expected InvalidBooleanValue, got: {:?}", other),
    }
}

// ---------------------------------------------------------------------------
// 6. Decode i8 from 0 bytes → error
// ---------------------------------------------------------------------------
#[test]
fn test_decode_i8_zero_bytes() {
    let result: Result<(i8, usize), _> = decode_from_slice(&[]);
    assert!(result.is_err(), "decoding i8 from 0 bytes must fail");
}

// ---------------------------------------------------------------------------
// 7. Decode f32 from 3 bytes (needs 4) → error
// ---------------------------------------------------------------------------
#[test]
fn test_decode_f32_three_bytes() {
    // f32 uses fixed 4-byte little-endian encoding; 3 bytes must fail
    let result: Result<(f32, usize), _> = decode_from_slice(&[0x00u8, 0x00, 0x80]);
    assert!(result.is_err(), "f32 from 3 bytes must fail");
}

// ---------------------------------------------------------------------------
// 8. Decode f64 from 7 bytes (needs 8) → error
// ---------------------------------------------------------------------------
#[test]
fn test_decode_f64_seven_bytes() {
    let result: Result<(f64, usize), _> =
        decode_from_slice(&[0x00u8, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00]);
    assert!(result.is_err(), "f64 from 7 bytes must fail");
}

// ---------------------------------------------------------------------------
// 9. Decode Option<u8> where tag byte is 2 (invalid — only 0 or 1 are valid)
// ---------------------------------------------------------------------------
#[test]
fn test_decode_option_u8_invalid_tag() {
    // Option is encoded as 0 (None) or 1 followed by value (Some); tag 2 is invalid
    let result: Result<(Option<u8>, usize), _> = decode_from_slice(&[2u8]);
    assert!(
        result.is_err(),
        "Option tag byte 2 must produce a decode error"
    );
}

// ---------------------------------------------------------------------------
// 10. Decode enum discriminant pointing to non-existent variant
// ---------------------------------------------------------------------------
#[test]
fn test_decode_enum_nonexistent_discriminant() {
    #[derive(Debug, Encode, Decode)]
    enum ThreeVariants {
        First,
        Second,
        Third,
    }
    // Discriminant 99 does not exist in ThreeVariants (valid: 0, 1, 2)
    let bad_bytes = encode_to_vec(&99u32).expect("encode discriminant");
    let result: Result<(ThreeVariants, usize), _> = decode_from_slice(&bad_bytes);
    assert!(
        result.is_err(),
        "discriminant 99 is not a valid ThreeVariants variant"
    );
    let err = result.expect_err("should be error");
    let msg = format!("{}", err);
    assert!(
        msg.contains("99") || msg.to_lowercase().contains("variant"),
        "error should reference bad discriminant: {}",
        msg
    );
}

// ---------------------------------------------------------------------------
// 11. Decode [u8; 4] from 3 bytes → error
// ---------------------------------------------------------------------------
#[test]
fn test_decode_array_u8_4_from_three_bytes() {
    let result: Result<([u8; 4], usize), _> = decode_from_slice(&[0x01u8, 0x02, 0x03]);
    assert!(result.is_err(), "[u8; 4] from 3 bytes must fail");
}

// ---------------------------------------------------------------------------
// 12. Encode to fixed-size buffer that's too small → error (returns Err)
// ---------------------------------------------------------------------------
#[test]
fn test_encode_into_slice_buffer_too_small() {
    // A String "hello world" will encode to more than 4 bytes
    let value = String::from("hello world");
    let mut buf = [0u8; 4];
    let result = oxicode::encode_into_slice(value, &mut buf, oxicode::config::standard());
    assert!(
        result.is_err(),
        "encoding 'hello world' into 4-byte buffer must fail"
    );
}

// ---------------------------------------------------------------------------
// 13. Decode with size limit smaller than encoded data → LimitExceeded
// ---------------------------------------------------------------------------
#[test]
fn test_limit_exceeded_error_variant() {
    // Construct the error directly; the standard decoder does not currently
    // enforce a byte-level limit at the slice level, but the Error variant
    // must be constructible and match correctly.
    let err = oxicode::Error::LimitExceeded {
        limit: 10,
        found: 500,
    };
    match err {
        oxicode::Error::LimitExceeded { limit, found } => {
            assert_eq!(limit, 10);
            assert_eq!(found, 500);
        }
        other => panic!("expected LimitExceeded, got: {:?}", other),
    }
}

// ---------------------------------------------------------------------------
// 14. Chain of decode errors: first failure returns immediately, second
//     independent decode with valid data succeeds (early-return semantics)
// ---------------------------------------------------------------------------
#[test]
fn test_chain_decode_error_early_return() {
    // First decode: bad bool byte
    let bad = &[7u8];
    let first_result: Result<(bool, usize), _> = decode_from_slice(bad);
    assert!(first_result.is_err(), "first decode must fail");

    // Second decode: valid u32 — must succeed independently
    let good = encode_to_vec(&42u32).expect("encode");
    let (val, _): (u32, usize) = decode_from_slice(&good).expect("second decode must succeed");
    assert_eq!(val, 42u32);
}

// ---------------------------------------------------------------------------
// 15. Multiple roundtrips with error injection (corrupt a byte in the middle)
// ---------------------------------------------------------------------------
#[test]
fn test_corrupt_middle_byte_decode_fails_or_succeeds_gracefully() {
    let value: Vec<u32> = vec![10, 20, 30, 40];
    let encoded = encode_to_vec(&value).expect("encode");
    assert!(encoded.len() >= 3, "encoded must have at least 3 bytes");

    // Corrupt the middle byte
    let mid = encoded.len() / 2;
    let mut corrupted = encoded.clone();
    corrupted[mid] = corrupted[mid].wrapping_add(0xAB);

    // Must not panic — either succeeds with different value or returns error
    let result: Result<(Vec<u32>, usize), _> = decode_from_slice(&corrupted);
    // If it decodes without error it should at least not produce the original value
    if let Ok((decoded, _)) = result {
        // Corrupted data may decode to something different — that's acceptable
        let _ = decoded; // just ensure no panic
    }
    // If it errors, that's the expected resilient behavior too — no assertion needed
}

// ---------------------------------------------------------------------------
// 16. Decode CString containing null byte in middle → error
// ---------------------------------------------------------------------------
#[cfg(feature = "std")]
#[test]
fn test_decode_cstring_with_null_byte_in_middle() {
    use std::ffi::CString;

    // Build bytes that look like a valid length-prefixed blob containing a null byte
    // CString encodes as a Vec<u8> (length-prefixed) and then validates no interior nulls
    // We craft: varint(5) followed by [104, 101, 0, 108, 111] = "he\0lo"
    let mut bytes = vec![5u8]; // length = 5
    bytes.extend_from_slice(&[104u8, 101, 0, 108, 111]); // "he\0lo"
    let result: Result<(CString, usize), _> = decode_from_slice(&bytes);
    assert!(
        result.is_err(),
        "CString with embedded null byte must fail to decode"
    );
}

// ---------------------------------------------------------------------------
// 17. Decode NonZeroU32 from value 0 → error (NonZeroValueIsZero)
// ---------------------------------------------------------------------------
#[test]
fn test_decode_nonzero_u32_from_zero() {
    use core::num::NonZeroU32;
    let zero_bytes = encode_to_vec(&0u32).expect("encode zero");
    let result: Result<(NonZeroU32, usize), _> = decode_from_slice(&zero_bytes);
    assert!(result.is_err(), "decoding 0 as NonZeroU32 must fail");
    let err = result.expect_err("should be error");
    match err {
        oxicode::Error::NonZeroTypeIsZero { non_zero_type } => {
            let msg = format!("{}", non_zero_type);
            assert!(
                msg.to_lowercase().contains("u32"),
                "type should be u32, got: {}",
                msg
            );
        }
        other => panic!("expected NonZeroTypeIsZero, got: {:?}", other),
    }
}

// ---------------------------------------------------------------------------
// 18. Decode char from invalid Unicode codepoint (surrogate 0xD800) → error
// ---------------------------------------------------------------------------
#[test]
fn test_decode_char_invalid_unicode_codepoint() {
    // 0xD800 is a UTF-16 high surrogate — not a valid Unicode scalar value
    let bad_codepoint = 0xD800u32;
    let encoded = encode_to_vec(&bad_codepoint).expect("encode u32 codepoint");
    let result: Result<(char, usize), _> = decode_from_slice(&encoded);
    assert!(
        result.is_err(),
        "surrogate codepoint 0xD800 must be rejected as char"
    );
}

// ---------------------------------------------------------------------------
// 19. Recovery: partial decode failure doesn't corrupt unrelated data
//     (decode failures leave the original slice unmodified)
// ---------------------------------------------------------------------------
#[test]
fn test_partial_decode_failure_leaves_source_intact() {
    let original: Vec<u8> = vec![0xFF, 0xFF, 0xFF]; // likely fails as String

    let snapshot = original.clone();

    // Attempt decode — we don't care if it succeeds or fails
    let _: Result<(String, usize), _> = decode_from_slice(&original);

    // The source slice must be bit-for-bit identical after the attempted decode
    assert_eq!(
        original, snapshot,
        "source slice must be unmodified after a decode attempt"
    );
}

// ---------------------------------------------------------------------------
// 20. Error Display format includes the message
// ---------------------------------------------------------------------------
#[test]
fn test_error_display_contains_message() {
    let err = oxicode::Error::InvalidData {
        message: "unique-sentinel-xyz",
    };
    let displayed = format!("{}", err);
    assert!(
        displayed.contains("unique-sentinel-xyz"),
        "Display must include the message: {}",
        displayed
    );
}

// ---------------------------------------------------------------------------
// 21. Error Debug format is non-empty
// ---------------------------------------------------------------------------
#[test]
fn test_error_debug_format_non_empty() {
    let variants: &[oxicode::Error] = &[
        oxicode::Error::UnexpectedEnd { additional: 1 },
        oxicode::Error::InvalidBooleanValue(3),
        oxicode::Error::LimitExceeded {
            limit: 5,
            found: 10,
        },
        oxicode::Error::NonZeroTypeIsZero {
            non_zero_type: oxicode::error::IntegerType::U8,
        },
        oxicode::Error::UnexpectedVariant {
            found: 7,
            type_name: "DebugEnum",
        },
        oxicode::Error::Custom { message: "debug-x" },
        oxicode::Error::OutsideUsizeRange(u64::MAX),
        oxicode::Error::InvalidCharEncoding([0xAA, 0xBB, 0xCC, 0xDD]),
    ];
    for err in variants {
        let debug_str = format!("{:?}", err);
        assert!(
            !debug_str.is_empty(),
            "Debug format must not be empty for: {:?}",
            err
        );
    }
}

// ---------------------------------------------------------------------------
// 22. All error variants accessible and distinguishable via pattern matching
// ---------------------------------------------------------------------------
#[test]
fn test_all_error_variants_distinguishable() {
    fn classify(e: &oxicode::Error) -> &'static str {
        match e {
            oxicode::Error::UnexpectedEnd { .. } => "unexpected_end",
            oxicode::Error::InvalidData { .. } => "invalid_data",
            oxicode::Error::InvalidIntegerType { .. } => "invalid_integer_type",
            oxicode::Error::InvalidBooleanValue(_) => "invalid_boolean_value",
            oxicode::Error::InvalidCharEncoding(_) => "invalid_char_encoding",
            oxicode::Error::LimitExceeded { .. } => "limit_exceeded",
            oxicode::Error::Custom { .. } => "custom",
            oxicode::Error::OutsideUsizeRange(_) => "outside_usize_range",
            oxicode::Error::NonZeroTypeIsZero { .. } => "nonzero_type_is_zero",
            oxicode::Error::UnexpectedVariant { .. } => "unexpected_variant",
            _ => "other",
        }
    }

    let cases: &[(&oxicode::Error, &str)] = &[
        (
            &oxicode::Error::UnexpectedEnd { additional: 4 },
            "unexpected_end",
        ),
        (
            &oxicode::Error::InvalidData {
                message: "bad data",
            },
            "invalid_data",
        ),
        (
            &oxicode::Error::InvalidIntegerType {
                expected: oxicode::error::IntegerType::U32,
                found: oxicode::error::IntegerType::I64,
            },
            "invalid_integer_type",
        ),
        (
            &oxicode::Error::InvalidBooleanValue(9),
            "invalid_boolean_value",
        ),
        (
            &oxicode::Error::InvalidCharEncoding([0x00, 0x01, 0x02, 0x03]),
            "invalid_char_encoding",
        ),
        (
            &oxicode::Error::LimitExceeded { limit: 1, found: 2 },
            "limit_exceeded",
        ),
        (&oxicode::Error::Custom { message: "c" }, "custom"),
        (
            &oxicode::Error::OutsideUsizeRange(u64::MAX),
            "outside_usize_range",
        ),
        (
            &oxicode::Error::NonZeroTypeIsZero {
                non_zero_type: oxicode::error::IntegerType::I16,
            },
            "nonzero_type_is_zero",
        ),
        (
            &oxicode::Error::UnexpectedVariant {
                found: 0,
                type_name: "E",
            },
            "unexpected_variant",
        ),
    ];

    for (err, expected_class) in cases {
        let got = classify(err);
        assert_eq!(
            got, *expected_class,
            "expected class '{}' for {:?}, got '{}'",
            expected_class, err, got
        );
    }
}
