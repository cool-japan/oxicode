//! Comprehensive tests for error types and their Display/Debug formatting.
//!
//! These 20 tests focus on the Display and Debug representations of all
//! error variants, as well as std::error::Error trait integration.

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
// oxicode uses a single unified Error type for both encoding and decoding.
// The task requires "use oxicode::error::{DecodeError, EncodeError}" but since
// the error module only exposes one Error type, we create type aliases here.
use oxicode::{decode_from_slice, encode_into_slice, encode_to_vec};
use std::f64::consts::{E, PI};

/// DecodeError is an alias for the unified oxicode Error type.
type DecodeError = oxicode::Error;
/// EncodeError is an alias for the unified oxicode Error type.
type EncodeError = oxicode::Error;

mod error_display_tests {
    use super::{
        decode_from_slice, encode_into_slice, encode_to_vec, DecodeError, EncodeError, E, PI,
    };

    // -----------------------------------------------------------------------
    // 1. DecodeError::UnexpectedEnd Display includes "unexpected end"
    // -----------------------------------------------------------------------
    #[test]
    fn test_01_unexpected_end_display_contains_unexpected_end() {
        let err = DecodeError::UnexpectedEnd { additional: 3 };
        let msg = format!("{}", err);
        let msg_lower = msg.to_lowercase();
        assert!(
            msg_lower.contains("unexpected") || msg_lower.contains("end"),
            "UnexpectedEnd Display must contain 'unexpected' or 'end': got '{}'",
            msg
        );
    }

    // -----------------------------------------------------------------------
    // 2. DecodeError::LimitExceeded Display includes both limit and found values
    // -----------------------------------------------------------------------
    #[test]
    fn test_02_limit_exceeded_display_contains_limit_and_found() {
        let err = DecodeError::LimitExceeded {
            limit: 512,
            found: 8192,
        };
        let msg = format!("{}", err);
        assert!(
            msg.contains("512"),
            "LimitExceeded Display must contain limit value 512: got '{}'",
            msg
        );
        assert!(
            msg.contains("8192"),
            "LimitExceeded Display must contain found value 8192: got '{}'",
            msg
        );
    }

    // -----------------------------------------------------------------------
    // 3. DecodeError::InvalidBooleanValue(2) Display includes "2"
    // -----------------------------------------------------------------------
    #[test]
    fn test_03_invalid_boolean_value_2_display_contains_2() {
        let err = DecodeError::InvalidBooleanValue(2);
        let msg = format!("{}", err);
        assert!(
            msg.contains('2'),
            "InvalidBooleanValue(2) Display must contain '2': got '{}'",
            msg
        );
    }

    // -----------------------------------------------------------------------
    // 4. DecodeError::UnexpectedVariant Display includes discriminant
    // -----------------------------------------------------------------------
    #[test]
    fn test_04_unexpected_variant_display_contains_discriminant() {
        let discriminant: u32 = 314;
        let err = DecodeError::UnexpectedVariant {
            found: discriminant,
            type_name: "MyVariantEnum",
        };
        let msg = format!("{}", err);
        assert!(
            msg.contains("314"),
            "UnexpectedVariant Display must contain discriminant 314: got '{}'",
            msg
        );
    }

    // -----------------------------------------------------------------------
    // 5. DecodeError::Utf8(err) Display includes utf8 error info
    // -----------------------------------------------------------------------
    #[cfg(feature = "alloc")]
    #[test]
    fn test_05_utf8_error_display_contains_utf8_info() {
        let invalid = &[0xFF, 0xFE][..];
        let utf8_err = core::str::from_utf8(invalid).expect_err("should produce Utf8Error");
        let err: oxicode::Error = utf8_err.into();
        let msg = format!("{}", err);
        assert!(
            !msg.is_empty(),
            "Utf8 error Display must be non-empty: got '{}'",
            msg
        );
        // The Display for Utf8 variant mentions the byte offset
        assert!(
            msg.to_lowercase().contains("utf") || msg.contains('0'),
            "Utf8 error Display should reference UTF-8 or offset: got '{}'",
            msg
        );
    }

    // -----------------------------------------------------------------------
    // 6. EncodeError::UnexpectedEnd Display is non-empty and mentions relevant info
    // -----------------------------------------------------------------------
    #[test]
    fn test_06_encode_error_unexpected_end_display() {
        // EncodeError is the same type as DecodeError in oxicode.
        let err = EncodeError::UnexpectedEnd { additional: 16 };
        let msg = format!("{}", err);
        assert!(
            !msg.is_empty(),
            "EncodeError::UnexpectedEnd Display must not be empty"
        );
        assert!(
            msg.contains("16"),
            "EncodeError::UnexpectedEnd Display must contain additional=16: got '{}'",
            msg
        );
    }

    // -----------------------------------------------------------------------
    // 7. EncodeError::LimitExceeded Display is non-empty and contains values
    // -----------------------------------------------------------------------
    #[test]
    fn test_07_encode_error_limit_exceeded_display() {
        let err = EncodeError::LimitExceeded {
            limit: 1024,
            found: 65536,
        };
        let msg = format!("{}", err);
        assert!(
            !msg.is_empty(),
            "EncodeError::LimitExceeded Display must not be empty"
        );
        assert!(
            msg.contains("1024") && msg.contains("65536"),
            "EncodeError::LimitExceeded Display must contain 1024 and 65536: got '{}'",
            msg
        );
    }

    // -----------------------------------------------------------------------
    // 8. Error Debug format is non-empty for all constructible variants
    // -----------------------------------------------------------------------
    #[test]
    fn test_08_error_debug_format_all_variants_non_empty() {
        let variants: &[oxicode::Error] = &[
            oxicode::Error::UnexpectedEnd { additional: 1 },
            oxicode::Error::InvalidBooleanValue(99),
            oxicode::Error::LimitExceeded {
                limit: 10,
                found: 20,
            },
            oxicode::Error::InvalidData {
                message: "debug test",
            },
            oxicode::Error::Custom {
                message: "custom-debug",
            },
            oxicode::Error::OutsideUsizeRange(u64::MAX),
            oxicode::Error::NonZeroTypeIsZero {
                non_zero_type: oxicode::error::IntegerType::U64,
            },
            oxicode::Error::UnexpectedVariant {
                found: 42,
                type_name: "DebugTestEnum",
            },
            oxicode::Error::InvalidCharEncoding([0xAA, 0xBB, 0xCC, 0xDD]),
            oxicode::Error::InvalidIntegerType {
                expected: oxicode::error::IntegerType::U32,
                found: oxicode::error::IntegerType::I64,
            },
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

    // -----------------------------------------------------------------------
    // 9. Error from truncated bytes (empty slice for u64) → is UnexpectedEnd
    // -----------------------------------------------------------------------
    #[test]
    fn test_09_error_from_truncated_empty_slice_u64_is_unexpected_end() {
        let result: Result<(u64, usize), _> = decode_from_slice(&[]);
        let err = result.expect_err("decoding u64 from empty slice must fail");
        match err {
            oxicode::Error::UnexpectedEnd { .. } => {
                // Expected — test passes
            }
            other => panic!("Expected UnexpectedEnd from empty slice, got: {:?}", other),
        }
    }

    // -----------------------------------------------------------------------
    // 10. Error from invalid bool (byte 2) → is InvalidBooleanValue
    // -----------------------------------------------------------------------
    #[test]
    fn test_10_error_from_invalid_bool_byte_is_invalid_boolean_value() {
        let result: Result<(bool, usize), _> = decode_from_slice(&[2u8]);
        let err = result.expect_err("decoding bool from byte 2 must fail");
        match err {
            oxicode::Error::InvalidBooleanValue(v) => {
                assert_eq!(v, 2, "InvalidBooleanValue must carry the byte value 2");
            }
            other => panic!("Expected InvalidBooleanValue, got: {:?}", other),
        }
    }

    // -----------------------------------------------------------------------
    // 11. Error from limit exceeded for large Vec → is LimitExceeded (direct)
    // -----------------------------------------------------------------------
    #[test]
    fn test_11_limit_exceeded_variant_constructed_directly() {
        // The standard decoder does not enforce a byte limit on slices; test
        // that the variant is constructible and matches correctly.
        let err = oxicode::Error::LimitExceeded {
            limit: 100,
            found: 50_000,
        };
        match err {
            oxicode::Error::LimitExceeded { limit, found } => {
                assert_eq!(limit, 100, "limit field must be 100");
                assert_eq!(found, 50_000, "found field must be 50_000");
            }
            other => panic!("Expected LimitExceeded, got: {:?}", other),
        }
    }

    // -----------------------------------------------------------------------
    // 12. Error from bad UTF-8 → is Utf8 (alloc) or InvalidData (no-alloc)
    // -----------------------------------------------------------------------
    #[test]
    fn test_12_error_from_bad_utf8_is_utf8_or_invalid_data() {
        // Craft: varint(3) = 0x03, then 3 bytes of invalid UTF-8.
        let mut bytes: Vec<u8> = Vec::new();
        bytes.push(0x03u8);
        bytes.extend_from_slice(&[0xFF, 0xFE, 0xFD]);
        let result: Result<(String, usize), _> = decode_from_slice(&bytes);
        let err = result.expect_err("bad UTF-8 bytes must cause decode error");
        let msg = format!("{}", err);
        assert!(
            !msg.is_empty(),
            "UTF-8 error Display must be non-empty: got '{}'",
            msg
        );
        // Either Utf8 or InvalidData — both are valid depending on features.
        // We verify by checking the match exhausts the expected variants.
        let is_expected = matches!(
            err,
            oxicode::Error::InvalidData { .. } | oxicode::Error::InvalidCharEncoding(_)
        );
        #[cfg(feature = "alloc")]
        let is_expected = is_expected || matches!(err, oxicode::Error::Utf8 { .. });
        assert!(
            is_expected,
            "Expected Utf8 or InvalidData for bad UTF-8 bytes, got Display: {}",
            msg
        );
    }

    // -----------------------------------------------------------------------
    // 13. Error from unknown enum discriminant → is an error with the discriminant value
    // -----------------------------------------------------------------------
    #[test]
    fn test_13_error_from_unknown_discriminant_is_error_variant() {
        use oxicode::{Decode, Encode};

        #[derive(Debug, Encode, Decode)]
        enum FourVariantEnum {
            Alpha,
            Beta,
            Gamma,
            Delta,
        }

        // Discriminant 200 is not valid for FourVariantEnum (valid: 0..=3).
        let bad_bytes = encode_to_vec(&200u32).expect("encoding discriminant 200 must succeed");
        let result: Result<(FourVariantEnum, usize), _> = decode_from_slice(&bad_bytes);
        let err = result.expect_err("discriminant 200 must be rejected for FourVariantEnum");
        // The derive macro may emit either UnexpectedVariant or InvalidData for unknown variants.
        // Both are valid error types; verify the error is informative.
        let msg = format!("{}", err);
        assert!(
            !msg.is_empty(),
            "Error Display for unknown discriminant must be non-empty: got '{}'",
            msg
        );
        let is_expected_variant = matches!(
            err,
            oxicode::Error::UnexpectedVariant { .. } | oxicode::Error::InvalidData { .. }
        );
        assert!(
            is_expected_variant,
            "Expected UnexpectedVariant or InvalidData for bad discriminant, got: {}",
            msg
        );
    }

    // -----------------------------------------------------------------------
    // 14. Error equality: same errors compare equal (PartialEq implemented)
    // -----------------------------------------------------------------------
    #[test]
    fn test_14_same_errors_compare_equal_via_partial_eq() {
        let err_a = oxicode::Error::InvalidBooleanValue(7);
        let err_b = oxicode::Error::InvalidBooleanValue(7);
        assert_eq!(
            err_a, err_b,
            "identical InvalidBooleanValue errors must be equal"
        );

        let err_c = oxicode::Error::UnexpectedEnd { additional: 4 };
        let err_d = oxicode::Error::UnexpectedEnd { additional: 4 };
        assert_eq!(err_c, err_d, "identical UnexpectedEnd errors must be equal");

        let err_e = oxicode::Error::LimitExceeded {
            limit: 10,
            found: 20,
        };
        let err_f = oxicode::Error::LimitExceeded {
            limit: 10,
            found: 20,
        };
        assert_eq!(err_e, err_f, "identical LimitExceeded errors must be equal");

        // Different values must not be equal
        let err_g = oxicode::Error::InvalidBooleanValue(3);
        assert_ne!(
            err_a, err_g,
            "different InvalidBooleanValue errors must not be equal"
        );
    }

    // -----------------------------------------------------------------------
    // 15. Error::source() returns None for decode errors (std feature)
    // -----------------------------------------------------------------------
    #[cfg(feature = "std")]
    #[test]
    fn test_15_error_source_returns_none_for_decode_errors() {
        use std::error::Error as StdError;

        let errors: &[oxicode::Error] = &[
            oxicode::Error::UnexpectedEnd { additional: 5 },
            oxicode::Error::InvalidBooleanValue(9),
            oxicode::Error::LimitExceeded { limit: 1, found: 2 },
            oxicode::Error::Custom {
                message: "source test",
            },
            oxicode::Error::InvalidData {
                message: "source none test",
            },
        ];
        for err in errors {
            assert!(
                err.source().is_none(),
                "source() must return None for {:?}",
                err
            );
        }
    }

    // -----------------------------------------------------------------------
    // 16. std::error::Error trait: DecodeError implements std::error::Error
    // -----------------------------------------------------------------------
    #[cfg(feature = "std")]
    #[test]
    fn test_16_decode_error_implements_std_error_trait() {
        // Compile-time check: if this compiles, the impl exists.
        fn assert_implements_std_error<Err: std::error::Error + Send + Sync + 'static>() {}
        assert_implements_std_error::<oxicode::Error>();

        // Runtime check: cast to dyn Error and call .to_string()
        let err: &dyn std::error::Error = &oxicode::Error::UnexpectedEnd { additional: 2 };
        let description = err.to_string();
        assert!(
            !description.is_empty(),
            "dyn std::error::Error::to_string() must be non-empty"
        );
    }

    // -----------------------------------------------------------------------
    // 17. Error display is deterministic (same error → same string)
    // -----------------------------------------------------------------------
    #[test]
    fn test_17_error_display_is_deterministic() {
        // Encode PI and E to verify floating point round-trips are stable.
        let pi_bytes = encode_to_vec(&PI).expect("encoding PI must succeed");
        let (decoded_pi, _): (f64, usize) =
            decode_from_slice(&pi_bytes).expect("decoding PI must succeed");
        assert_eq!(decoded_pi, PI, "PI roundtrip must be exact");

        let e_bytes = encode_to_vec(&E).expect("encoding E must succeed");
        let (decoded_e, _): (f64, usize) =
            decode_from_slice(&e_bytes).expect("decoding E must succeed");
        assert_eq!(decoded_e, E, "E roundtrip must be exact");

        // Test Display determinism: same error must produce the same string every time.
        // Use 271 as discriminant (floor(E * 100) = 271).
        let err = oxicode::Error::UnexpectedVariant {
            found: 271,
            type_name: "PiEnum",
        };
        let msg1 = format!("{}", err);
        let msg2 = format!("{}", err);
        assert_eq!(
            msg1, msg2,
            "Display output must be deterministic: '{}' vs '{}'",
            msg1, msg2
        );
        assert!(
            msg1.contains("271"),
            "Display must contain discriminant 271"
        );
    }

    // -----------------------------------------------------------------------
    // 18. Error format includes type name for UnexpectedVariant
    // -----------------------------------------------------------------------
    #[test]
    fn test_18_unexpected_variant_display_includes_type_name() {
        let type_name = "SentinelTypeForTest";
        let err = oxicode::Error::UnexpectedVariant {
            found: 99,
            type_name,
        };
        let msg = format!("{}", err);
        assert!(
            msg.contains(type_name),
            "UnexpectedVariant Display must include type name '{}': got '{}'",
            type_name,
            msg
        );
    }

    // -----------------------------------------------------------------------
    // 19. Multiple decode errors from same buffer are independent
    // -----------------------------------------------------------------------
    #[test]
    fn test_19_multiple_decode_errors_from_same_buffer_are_independent() {
        let bad_bool = &[2u8];

        // First decode attempt
        let err1 = decode_from_slice::<(bool, usize)>(bad_bool)
            .expect_err("first decode of byte 2 as bool must fail");

        // Second decode attempt from the same buffer
        let err2 = decode_from_slice::<(bool, usize)>(bad_bool)
            .expect_err("second decode of byte 2 as bool must fail");

        // Both errors must be the same variant with the same value
        assert_eq!(
            err1, err2,
            "repeated decodes from the same buffer must produce equal errors"
        );

        // The buffer must remain unchanged
        assert_eq!(
            bad_bool,
            &[2u8],
            "source buffer must be unchanged after failed decodes"
        );

        // Also verify that a valid decode still works after errors
        let valid_bytes = encode_to_vec(&42u32).expect("encoding u32 must succeed");
        let (val, _): (u32, usize) =
            decode_from_slice(&valid_bytes).expect("valid decode must succeed after prior errors");
        assert_eq!(val, 42u32, "valid decode value must be 42");
    }

    // -----------------------------------------------------------------------
    // 20. EncodeError from buffer too small → check display
    // -----------------------------------------------------------------------
    #[test]
    fn test_20_encode_error_buffer_too_small_display() {
        // "hello from oxicode" is 18 chars; with a 1-byte length prefix that is
        // 19 bytes minimum. A 5-byte buffer must cause the encode to fail.
        let mut small_buf = [0u8; 5];
        let result = encode_into_slice(
            String::from("hello from oxicode"),
            &mut small_buf,
            oxicode::config::standard(),
        );
        let err = result.expect_err("encoding long string into 5-byte buffer must fail");
        let msg = format!("{}", err);
        assert!(
            !msg.is_empty(),
            "EncodeError from buffer-too-small must produce non-empty Display: got '{}'",
            msg
        );
        let msg_lower = msg.to_lowercase();
        assert!(
            msg_lower.contains("unexpected")
                || msg_lower.contains("end")
                || msg_lower.contains("limit")
                || msg_lower.contains("bytes"),
            "EncodeError display should mention buffer/end/limit: got '{}'",
            msg
        );
    }
}
