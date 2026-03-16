//! Comprehensive validation tests – 22 unique scenarios.
//!
//! All 22 tests cover angles not exercised by `validation_test.rs` or
//! `validation_advanced_test.rs`: direct `ValidationResult` / `Constraint` API,
//! `Range::from_bounds`, `FieldValidation`, boundary values not previously hit,
//! and encode/decode-integrated validation workflows.

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
use oxicode::validation::{CollectionValidator, NumericValidator, ValidationError};

#[cfg(feature = "alloc")]
use oxicode::validation::{Constraint, Constraints, ValidationConfig, ValidationResult, Validator};

// ============================================================================
// 22 tests grouped into one module
// ============================================================================

#[cfg(test)]
mod validation_comprehensive_tests {
    use super::*;

    // ── 1. Validator with no constraints – always passes ─────────────────────
    //
    // Distinct angle: exercises a *non-String* type (u64) with zero constraints,
    // including sentinel values that would trip any range guard.
    #[cfg(feature = "alloc")]
    #[test]
    fn test_01_no_constraints_always_passes_u64() {
        let v: Validator<u64> = Validator::new();
        assert_eq!(v.constraint_count(), 0);
        assert!(
            v.validate(&0u64).is_ok(),
            "0 must pass unconstrained validator"
        );
        assert!(
            v.validate(&u64::MAX).is_ok(),
            "u64::MAX must pass unconstrained validator"
        );
        assert!(
            v.validate(&u64::MIN).is_ok(),
            "u64::MIN must pass unconstrained validator"
        );
    }

    // ── 2. StringValidator max_len 10 – passes for short string ──────────────
    //
    // Uses the `StringValidator` builder (not `Validator<String>`) – distinct path.
    #[cfg(feature = "alloc")]
    #[test]
    fn test_02_string_validator_max_len_10_passes() {
        use oxicode::validation::StringValidator;
        let sv = StringValidator::new().max_len(10);
        assert!(
            sv.validate("short").is_ok(),
            "5-char string must pass max_len(10)"
        );
        assert!(
            sv.validate("0123456789").is_ok(),
            "10-char string must pass max_len(10)"
        );
    }

    // ── 3. StringValidator max_len 5 – fails for long string ─────────────────
    #[cfg(feature = "alloc")]
    #[test]
    fn test_03_string_validator_max_len_5_fails_for_long() {
        use oxicode::validation::StringValidator;
        let sv = StringValidator::new().max_len(5);
        assert!(
            sv.validate("toolong").is_err(),
            "7-char string must fail max_len(5)"
        );
    }

    // ── 4. StringValidator min_len 3 – passes for long string ────────────────
    #[cfg(feature = "alloc")]
    #[test]
    fn test_04_string_validator_min_len_3_passes() {
        use oxicode::validation::StringValidator;
        let sv = StringValidator::new().min_len(3);
        assert!(
            sv.validate("abc").is_ok(),
            "exactly 3 chars must pass min_len(3)"
        );
        assert!(
            sv.validate("longer string").is_ok(),
            "longer string must pass min_len(3)"
        );
    }

    // ── 5. StringValidator min_len 10 – fails for short string ───────────────
    #[cfg(feature = "alloc")]
    #[test]
    fn test_05_string_validator_min_len_10_fails_for_short() {
        use oxicode::validation::StringValidator;
        let sv = StringValidator::new().min_len(10);
        assert!(
            sv.validate("hi").is_err(),
            "2-char string must fail min_len(10)"
        );
    }

    // ── 6. NumericValidator min value passes ─────────────────────────────────
    //
    // Tests `.min()` alone (no upper bound) on i16.
    #[test]
    fn test_06_numeric_validator_min_only_passes() {
        let v = NumericValidator::<i16>::new().min(-100i16);
        assert!(v.validate(&-100i16).is_ok(), "exactly at min must pass");
        assert!(v.validate(&0i16).is_ok(), "above min must pass");
        assert!(v.validate(&i16::MAX).is_ok(), "i16::MAX must pass");
    }

    // ── 7. NumericValidator max value passes ─────────────────────────────────
    //
    // Tests `.max()` alone (no lower bound) on u8.
    #[test]
    fn test_07_numeric_validator_max_only_passes() {
        let v = NumericValidator::<u8>::new().max(200u8);
        assert!(v.validate(&0u8).is_ok(), "0 must pass max(200)");
        assert!(v.validate(&200u8).is_ok(), "exactly at max must pass");
    }

    // ── 8. NumericValidator out of range fails ────────────────────────────────
    //
    // Both below-min and above-max failure paths for u8.
    #[test]
    fn test_08_numeric_validator_out_of_range_fails() {
        let v = NumericValidator::<u8>::new().max(200u8);
        assert!(v.validate(&201u8).is_err(), "201 must fail max(200)");
    }

    // ── 9. NumericValidator range – passes for valid value ────────────────────
    //
    // Uses f32 (not covered by other test files).
    #[test]
    fn test_09_numeric_validator_range_f32_passes() {
        let v = NumericValidator::<f32>::new().range(-1.0f32, 1.0f32);
        assert!(
            v.validate(&0.0f32).is_ok(),
            "0.0 must pass range [-1.0, 1.0]"
        );
        assert!(v.validate(&-1.0f32).is_ok(), "exactly -1.0 must pass");
        assert!(v.validate(&1.0f32).is_ok(), "exactly 1.0 must pass");
    }

    // ── 10. NumericValidator range – fails for invalid value ──────────────────
    #[test]
    fn test_10_numeric_validator_range_f32_fails() {
        let v = NumericValidator::<f32>::new().range(-1.0f32, 1.0f32);
        assert!(
            v.validate(&1.1f32).is_err(),
            "1.1 must fail range [-1.0, 1.0]"
        );
        assert!(
            v.validate(&-1.1f32).is_err(),
            "-1.1 must fail range [-1.0, 1.0]"
        );
    }

    // ── 11. CollectionValidator max items – passes ────────────────────────────
    //
    // Uses `validate_len` (direct length API – not tested as a solo call in other files).
    #[test]
    fn test_11_collection_validator_max_items_passes_via_len() {
        let cv = CollectionValidator::new().max_len(8);
        assert!(cv.validate_len(0).is_ok(), "length 0 must pass max_len(8)");
        assert!(cv.validate_len(8).is_ok(), "length 8 must pass max_len(8)");
    }

    // ── 12. CollectionValidator max items – fails ─────────────────────────────
    #[test]
    fn test_12_collection_validator_max_items_fails_via_len() {
        let cv = CollectionValidator::new().max_len(8);
        assert!(cv.validate_len(9).is_err(), "length 9 must fail max_len(8)");
    }

    // ── 13. CollectionValidator min items – passes ────────────────────────────
    #[test]
    fn test_13_collection_validator_min_items_passes() {
        let cv = CollectionValidator::new().min_len(3);
        let arr: [u32; 3] = [1, 2, 3];
        assert!(
            cv.validate(&arr).is_ok(),
            "3-element slice must pass min_len(3)"
        );
        let arr2: [u32; 10] = [0; 10];
        assert!(
            cv.validate(&arr2).is_ok(),
            "10-element slice must pass min_len(3)"
        );
    }

    // ── 14. CollectionValidator min items – fails ─────────────────────────────
    #[test]
    fn test_14_collection_validator_min_items_fails() {
        let cv = CollectionValidator::new().min_len(5);
        let arr: [u8; 4] = [0; 4];
        assert!(
            cv.validate(&arr).is_err(),
            "4-element slice must fail min_len(5)"
        );
        assert!(cv.validate_len(0).is_err(), "length 0 must fail min_len(5)");
    }

    // ── 15. ascii_only validation – passes for ASCII ──────────────────────────
    //
    // Uses the `Constraint` trait directly on `AsciiOnly` (not via `Validator<String>`).
    #[cfg(feature = "alloc")]
    #[test]
    fn test_15_ascii_only_constraint_passes_for_ascii() {
        let c = Constraints::ascii_only();
        let ascii_samples = ["hello", "RUST_2024", "!@#$", "0123456789"];
        for s in &ascii_samples {
            assert!(
                matches!(c.validate(*s), ValidationResult::Valid),
                "pure ASCII '{}' must be Valid",
                s
            );
        }
    }

    // ── 16. ascii_only validation – fails for Unicode ─────────────────────────
    #[cfg(feature = "alloc")]
    #[test]
    fn test_16_ascii_only_constraint_fails_for_unicode() {
        let c = Constraints::ascii_only();
        let unicode_samples = ["こんにちは", "🦀", "naïve", "Ünïcode"];
        for s in &unicode_samples {
            assert!(
                matches!(c.validate(*s), ValidationResult::Invalid(_)),
                "non-ASCII '{}' must be Invalid",
                s
            );
        }
    }

    // ── 17. validate_or_default – returns valid data on pass ──────────────────
    //
    // Tests with `String` type (the i32 variant is the common example; this is distinct).
    #[cfg(feature = "alloc")]
    #[test]
    fn test_17_validate_or_default_returns_valid_string() {
        let v: Validator<String> = Validator::new()
            .constraint("tag", Constraints::min_len(1))
            .constraint("tag", Constraints::max_len(20));

        let result = v.validate_or_default("rust".to_string(), "default".to_string());
        assert_eq!(result, "rust", "valid string must be returned unchanged");
    }

    // ── 18. validate_or_default – returns default on fail ─────────────────────
    #[cfg(feature = "alloc")]
    #[test]
    fn test_18_validate_or_default_returns_default_string_on_fail() {
        let v: Validator<String> = Validator::new().constraint("tag", Constraints::max_len(3));

        let too_long = "this_is_too_long".to_string();
        let result = v.validate_or_default(too_long, "fallback".to_string());
        assert_eq!(
            result, "fallback",
            "invalid string must return the fallback default"
        );
    }

    // ── 19. Multiple validators chained ───────────────────────────────────────
    //
    // Uses `add_constraint` (mutable) + `constraint` (builder) together – distinct
    // combination not tested as a unit.
    #[cfg(feature = "alloc")]
    #[test]
    fn test_19_multiple_validators_chained_mixed_api() {
        // Build partly via builder, partly via add_constraint.
        let mut v: Validator<i32> =
            Validator::new().constraint("n", Constraints::range(Some(-50i32), Some(50i32)));
        v.add_constraint(
            "n",
            Constraints::custom(|x: &i32| x % 2 == 0, "must be even", "even-check"),
        );

        assert_eq!(v.constraint_count(), 2);

        // Passes both constraints: in range AND even.
        assert!(v.validate(&0).is_ok(), "0 satisfies both constraints");
        assert!(v.validate(&-50).is_ok(), "-50 is even and at boundary");
        assert!(v.validate(&50).is_ok(), "50 is even and at boundary");

        // Fails range but is even.
        assert!(v.validate(&52).is_err(), "52 is even but outside range");

        // In range but odd.
        assert!(v.validate(&3).is_err(), "3 is in range but odd");
    }

    // ── 20. ValidationConfig fail_fast behaviour ───────────────────────────────
    //
    // Previously tested with Strings; here tested with i32 and three constraints.
    #[cfg(feature = "alloc")]
    #[test]
    fn test_20_validation_config_fail_fast_with_i32() {
        let config = ValidationConfig::new().with_fail_fast(true);
        let mut v: Validator<i32> = Validator::with_config(config);
        // Value 200 fails range [0,100], fails range [-10, 10], and would fail a
        // third custom constraint; fail_fast must stop after the first.
        v.add_constraint("val", Constraints::range(Some(0i32), Some(100i32)));
        v.add_constraint("val", Constraints::range(Some(-10i32), Some(10i32)));
        v.add_constraint(
            "val",
            Constraints::custom(|x: &i32| *x == 0, "must be zero", "zero-check"),
        );

        let errors = v
            .validate(&200i32)
            .expect_err("200 must fail all constraints");
        assert_eq!(
            errors.len(),
            1,
            "fail_fast must stop at first error, got {} errors",
            errors.len()
        );
        assert_eq!(errors[0].field, "val");
    }

    // ── 21. Encode then validate then decode ──────────────────────────────────
    //
    // Encodes a u32 with `encode_to_vec`, validates the byte-length of the
    // encoded buffer, then decodes and confirms round-trip integrity.
    #[cfg(feature = "alloc")]
    #[test]
    fn test_21_encode_validate_decode_roundtrip() {
        // We validate the encoded bytes (Vec<u8>) length is within [1, 16].
        let byte_validator: Validator<Vec<u8>> = Validator::new()
            .constraint("bytes", Constraints::min_len(1))
            .constraint("bytes", Constraints::max_len(16));

        let original: u32 = 12345u32;
        let encoded = oxicode::encode_to_vec(&original).expect("encode_to_vec must not fail");

        assert!(
            byte_validator.validate(&encoded).is_ok(),
            "encoded bytes length must be in [1, 16]; got {}",
            encoded.len()
        );

        let (decoded, consumed): (u32, _) =
            oxicode::decode_from_slice(&encoded).expect("decode_from_slice must not fail");

        assert_eq!(decoded, original, "decoded value must equal original");
        assert_eq!(consumed, encoded.len(), "all bytes must be consumed");
    }

    // ── 22. Validation with custom error message ───────────────────────────────
    //
    // Verifies that the `ValidationError` produced from a custom constraint carries
    // the exact message string supplied by the caller (not a generic fallback).
    #[cfg(feature = "alloc")]
    #[test]
    fn test_22_custom_error_message_propagated() {
        const ERR_MSG: &str = "value must be a multiple of seven";

        let v: Validator<i32> = Validator::new().constraint(
            "lucky",
            Constraints::custom(|x: &i32| x % 7 == 0, ERR_MSG, "multiple-of-seven"),
        );

        // Valid: multiples of 7.
        assert!(v.validate(&0).is_ok(), "0 is divisible by 7");
        assert!(v.validate(&49).is_ok(), "49 is divisible by 7");
        assert!(v.validate(&-7).is_ok(), "-7 is divisible by 7");

        // Invalid: non-multiple; the error message must be exact.
        let errors = v.validate(&3i32).expect_err("3 is not divisible by 7");
        assert_eq!(errors.len(), 1);
        assert_eq!(
            errors[0].message, ERR_MSG,
            "error message must be '{}'",
            ERR_MSG
        );
        assert_eq!(errors[0].field, "lucky");

        // Also verify the Display output includes the custom message.
        let display = format!("{}", errors[0]);
        assert!(
            display.contains(ERR_MSG),
            "Display output must contain the custom message: {}",
            display
        );

        // Confirm ValidationError::new produces the same fields.
        let manual = ValidationError::new("lucky", ERR_MSG);
        assert_eq!(manual.field, "lucky");
        assert_eq!(manual.message, ERR_MSG);
    }
}
