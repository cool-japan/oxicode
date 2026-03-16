//! Comprehensive validation middleware tests.

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

// --- ValidationError display ---

#[test]
fn test_validation_error_display() {
    let err = ValidationError::new("username", "too short");
    let msg = format!("{}", err);
    assert!(
        msg.contains("username") && msg.contains("too short"),
        "Display must include field and message: {}",
        msg
    );
}

// --- NumericValidator standalone (always available, no alloc gate) ---

#[test]
fn test_numeric_validator_standalone_f64() {
    let validator = NumericValidator::<f64>::new().min(0.0).max(1.0);
    assert!(validator.validate(&0.5).is_ok());
    assert!(validator.validate(&0.0).is_ok());
    assert!(validator.validate(&1.0).is_ok());
    assert!(validator.validate(&-0.1).is_err());
    assert!(validator.validate(&1.1).is_err());
}

#[test]
fn test_numeric_validator_no_bounds() {
    let validator = NumericValidator::<i64>::new();
    assert!(validator.validate(&i64::MIN).is_ok());
    assert!(validator.validate(&i64::MAX).is_ok());
    assert!(validator.validate(&0).is_ok());
}

#[test]
fn test_numeric_validator_min_only() {
    let validator = NumericValidator::<i32>::new().min(0);
    assert!(validator.validate(&0).is_ok());
    assert!(validator.validate(&1_000_000).is_ok());
    assert!(validator.validate(&-1).is_err());
}

#[test]
fn test_numeric_validator_max_only() {
    let validator = NumericValidator::<u32>::new().max(100);
    assert!(validator.validate(&0).is_ok());
    assert!(validator.validate(&100).is_ok());
    assert!(validator.validate(&101).is_err());
}

#[test]
fn test_numeric_validator_range_method() {
    let validator = NumericValidator::<i16>::new().range(-10, 10);
    assert!(validator.validate(&0).is_ok());
    assert!(validator.validate(&-10).is_ok());
    assert!(validator.validate(&10).is_ok());
    assert!(validator.validate(&11).is_err());
    assert!(validator.validate(&-11).is_err());
}

// --- CollectionValidator standalone ---

#[test]
fn test_collection_validator_standalone() {
    let validator = CollectionValidator::new().max_len(5).min_len(1).non_empty();

    let short: [i32; 3] = [1, 2, 3];
    let single: [i32; 1] = [42];
    let max_ok: [i32; 5] = [0; 5];
    let too_long: [i32; 6] = [0; 6];
    let empty: [i32; 0] = [];

    assert!(validator.validate(&short).is_ok());
    assert!(validator.validate(&single).is_ok());
    assert!(validator.validate(&max_ok).is_ok());
    assert!(validator.validate(&too_long).is_err());
    assert!(validator.validate(&empty).is_err());
}

#[test]
fn test_collection_validator_validate_len() {
    let validator = CollectionValidator::new().min_len(2).max_len(4);
    assert!(validator.validate_len(2).is_ok());
    assert!(validator.validate_len(4).is_ok());
    assert!(validator.validate_len(1).is_err());
    assert!(validator.validate_len(5).is_err());
}

#[test]
fn test_collection_validator_non_empty_only() {
    let validator = CollectionValidator::new().non_empty();
    let non_empty: [u8; 1] = [0];
    let empty: [u8; 0] = [];
    assert!(validator.validate(&non_empty).is_ok());
    assert!(validator.validate(&empty).is_err());
}

// --- Alloc-gated Validator tests ---

#[cfg(feature = "alloc")]
mod alloc_tests {
    use oxicode::validation::{Constraints, ValidationConfig, Validator};

    #[test]
    fn test_validation_batch_strings() {
        let validator: Validator<String> = Validator::new()
            .constraint("text", Constraints::max_len(100))
            .constraint("text", Constraints::min_len(1))
            .constraint("text", Constraints::non_empty())
            .constraint("text", Constraints::ascii_only());

        let valid_inputs = vec![
            "hello".to_string(),
            "world123".to_string(),
            "user_name_42".to_string(),
        ];

        for input in &valid_inputs {
            assert!(
                validator.validate(input).is_ok(),
                "should be valid: {}",
                input
            );
        }

        let invalid_inputs = vec!["".to_string(), "こんにちは".to_string(), "x".repeat(101)];

        for input in &invalid_inputs {
            assert!(
                validator.validate(input).is_err(),
                "should be invalid: {}",
                input
            );
        }
    }

    #[test]
    fn test_validate_or_default_returns_value_when_valid() {
        let validator: Validator<i32> =
            Validator::new().constraint("age", Constraints::range(Some(0i32), Some(120i32)));

        assert_eq!(validator.validate_or_default(42, 0), 42);
        assert_eq!(validator.validate_or_default(0, -1), 0);
        assert_eq!(validator.validate_or_default(120, -1), 120);
    }

    #[test]
    fn test_validate_or_default_returns_default_when_invalid() {
        let validator: Validator<i32> =
            Validator::new().constraint("age", Constraints::range(Some(0i32), Some(100i32)));

        assert_eq!(validator.validate_or_default(-5, 99), 99);
        assert_eq!(validator.validate_or_default(200, 99), 99);
        assert_eq!(validator.validate_or_default(-1, 0), 0);
    }

    #[test]
    fn test_validation_multiple_constraint_types_vec_string() {
        let validator: Validator<Vec<String>> = Validator::new()
            .constraint("tags", Constraints::max_len(10))
            .constraint("tags", Constraints::min_len(1))
            .constraint("tags", Constraints::non_empty());

        // Valid: 2 elements
        let valid = vec!["rust".to_string(), "fast".to_string()];
        assert!(validator.validate(&valid).is_ok());

        // Invalid: too many elements
        let too_many: Vec<String> = (0..11).map(|i| format!("tag_{}", i)).collect();
        assert!(validator.validate(&too_many).is_err());

        // Invalid: empty
        let empty: Vec<String> = vec![];
        assert!(validator.validate(&empty).is_err());
    }

    #[test]
    fn test_validation_fail_fast_stops_at_first_error() {
        let config = ValidationConfig::new().with_fail_fast(true);
        let mut validator: Validator<String> = Validator::with_config(config);
        // "hello world" (11 chars) fails max_len(5) AND passes min_len(3)
        // but add both failing constraints
        validator.add_constraint("name", Constraints::max_len(5));
        validator.add_constraint("name", Constraints::min_len(20)); // also fails (11 < 20)

        let result = validator.validate(&"hello world".to_string());
        assert!(result.is_err());
        // With fail_fast, only 1 error
        assert_eq!(
            result.err().map(|e| e.len()),
            Some(1),
            "fail_fast should stop at first error"
        );
    }

    #[test]
    fn test_validation_collect_all_errors() {
        let config = ValidationConfig::new().with_fail_fast(false);
        let mut validator: Validator<String> = Validator::with_config(config);
        // "hello world" (11 chars): fails max_len(5) AND fails min_len(20)
        validator.add_constraint("name", Constraints::max_len(5));
        validator.add_constraint("name", Constraints::min_len(20));

        let result = validator.validate(&"hello world".to_string());
        assert!(result.is_err());
        // Without fail_fast, collect all errors
        assert_eq!(
            result.err().map(|e| e.len()),
            Some(2),
            "should collect all 2 errors without fail_fast"
        );
    }

    #[test]
    fn test_validate_first_returns_first_error() {
        let validator: Validator<String> = Validator::new()
            .constraint("field", Constraints::min_len(10))
            .constraint("field", Constraints::ascii_only());

        let result = validator.validate_first(&"hi".to_string()); // too short
        assert!(result.is_err());
        let err = result.expect_err("should be error");
        assert_eq!(err.field, "field");
    }

    #[test]
    fn test_constraint_count() {
        let validator: Validator<String> = Validator::new()
            .constraint("a", Constraints::max_len(100))
            .constraint("b", Constraints::min_len(1))
            .constraint("c", Constraints::non_empty());

        assert_eq!(validator.constraint_count(), 3);
    }

    #[test]
    fn test_constraint_count_empty() {
        let validator: Validator<i32> = Validator::new();
        assert_eq!(validator.constraint_count(), 0);
    }

    #[test]
    fn test_custom_constraint_rejects_negative() {
        let validator: Validator<i32> = Validator::new().constraint(
            "positive",
            Constraints::custom(|x: &i32| *x >= 0, "must be non-negative", "non-negative"),
        );

        assert!(validator.validate(&0).is_ok());
        assert!(validator.validate(&1).is_ok());
        assert!(validator.validate(&100).is_ok());
        assert!(validator.validate(&-1).is_err());
        assert!(validator.validate(&-100).is_err());
    }

    #[test]
    fn test_string_validator_standalone() {
        use oxicode::validation::StringValidator;

        let v = StringValidator::new()
            .min_len(1)
            .max_len(20)
            .non_empty()
            .ascii_only();

        assert!(v.validate("hello").is_ok());
        assert!(v.validate("a").is_ok());
        assert!(v.validate("exactly_twenty_ch!").is_ok()); // 18 chars, ok
        assert!(v.validate("").is_err()); // empty
        assert!(v.validate("こんにちは").is_err()); // non-ASCII
        assert!(v
            .validate("this_string_is_definitely_too_long_123")
            .is_err()); // > 20
    }

    #[test]
    fn test_validator_no_constraints_always_valid() {
        let validator: Validator<String> = Validator::new();
        assert!(validator.validate(&String::new()).is_ok());
        assert!(validator.validate(&"hello world".to_string()).is_ok());
        assert!(validator.validate(&"こんにちは".to_string()).is_ok());
    }

    #[test]
    fn test_validate_or_default_with_closure_not_called_when_valid() {
        let validator: Validator<i32> =
            Validator::new().constraint("v", Constraints::range(Some(0i32), Some(100i32)));

        let mut called = false;
        let result = validator.validate_or_default_with(&50, || {
            called = true;
            -1
        });
        assert_eq!(result, 50);
        assert!(!called, "closure must not be called when value is valid");
    }

    #[test]
    fn test_validate_or_default_with_closure_called_when_invalid() {
        let validator: Validator<i32> =
            Validator::new().constraint("v", Constraints::range(Some(0i32), Some(100i32)));

        let mut called = false;
        let result = validator.validate_or_default_with(&-99, || {
            called = true;
            42
        });
        assert_eq!(result, 42);
        assert!(called, "closure must be called when value is invalid");
    }

    #[test]
    fn test_validate_or_default_no_constraints_returns_value() {
        let validator: Validator<i32> = Validator::new();
        assert_eq!(validator.validate_or_default(999, 0), 999);
        assert_eq!(validator.validate_or_default(-999, 0), -999);
    }

    #[test]
    fn test_validation_config_fail_fast_default() {
        use oxicode::validation::ValidationConfig;
        let config = ValidationConfig::new();
        assert!(config.fail_fast, "default config should be fail_fast");
    }

    #[test]
    fn test_validation_config_builder_chaining() {
        use oxicode::validation::ValidationConfig;
        let config = ValidationConfig::new()
            .with_fail_fast(false)
            .with_max_depth(128)
            .with_checksum(true);
        assert!(!config.fail_fast);
        assert_eq!(config.max_depth, 128);
        assert!(config.verify_checksum);
    }

    #[test]
    fn test_validation_error_field_and_message_accessible() {
        use oxicode::validation::ValidationError;
        let err = ValidationError::new("email", "invalid format");
        assert_eq!(err.field, "email");
        assert_eq!(err.message, "invalid format");
    }

    #[test]
    fn test_validator_with_range_i32_boundaries() {
        let validator: Validator<i32> =
            Validator::new().constraint("score", Constraints::range(Some(0i32), Some(100i32)));

        assert!(validator.validate(&0).is_ok());
        assert!(validator.validate(&100).is_ok());
        assert!(validator.validate(&50).is_ok());
        assert!(validator.validate(&-1).is_err());
        assert!(validator.validate(&101).is_err());
    }

    #[test]
    fn test_validator_ascii_boundary() {
        let validator: Validator<String> = Validator::new()
            .constraint("text", Constraints::ascii_only())
            .constraint("text", Constraints::max_len(50));

        // Pure ASCII
        assert!(validator.validate(&"Hello, World! 123".to_string()).is_ok());
        // Contains non-ASCII
        assert!(validator.validate(&"café".to_string()).is_err());
        // Chinese characters
        assert!(validator.validate(&"中文".to_string()).is_err());
    }

    #[test]
    fn test_validator_multiple_fields_same_name() {
        // Multiple constraints on same "field" name
        let validator: Validator<i32> = Validator::new()
            .constraint("value", Constraints::range(Some(-100i32), Some(100i32)))
            .constraint("value", Constraints::range(Some(-50i32), Some(50i32)));

        // Passes both
        assert!(validator.validate(&0).is_ok());
        assert!(validator.validate(&50).is_ok());
        // Fails second range (between -100..100 but outside -50..50)
        assert!(validator.validate(&75).is_err());
        assert!(validator.validate(&-75).is_err());
    }

    // --- 15 additional tests ---

    #[test]
    fn test_regex_pattern_constraint() {
        // No built-in regex/pattern constraint; use Constraints::custom to mimic
        // a pattern check that only accepts alphanumeric ASCII identifiers.
        let is_identifier = Constraints::custom(
            |s: &String| s.chars().all(|c| c.is_ascii_alphanumeric() || c == '_'),
            "must be an ASCII identifier",
            "identifier pattern",
        );
        let validator: Validator<String> = Validator::new().constraint("id", is_identifier);

        assert!(validator.validate(&"hello_world".to_string()).is_ok());
        assert!(validator.validate(&"abc123".to_string()).is_ok());
        assert!(validator.validate(&"hello world".to_string()).is_err()); // space
        assert!(validator.validate(&"hello-world".to_string()).is_err()); // hyphen
        assert!(validator.validate(&"café".to_string()).is_err()); // non-ASCII
    }

    #[test]
    fn test_nested_struct_validation() {
        // Validate multiple logical fields using separate Validators.
        let username_validator: Validator<String> = Validator::new()
            .constraint("username", Constraints::min_len(3))
            .constraint("username", Constraints::max_len(32))
            .constraint("username", Constraints::ascii_only());

        let email_validator: Validator<String> = Validator::new()
            .constraint("email", Constraints::min_len(5))
            .constraint("email", Constraints::max_len(254))
            .constraint("email", Constraints::ascii_only());

        let good_username = "alice".to_string();
        let bad_username = "x".to_string(); // too short
        let good_email = "alice@example.com".to_string();
        let bad_email = "".to_string(); // empty

        assert!(username_validator.validate(&good_username).is_ok());
        assert!(username_validator.validate(&bad_username).is_err());
        assert!(email_validator.validate(&good_email).is_ok());
        assert!(email_validator.validate(&bad_email).is_err());
    }

    #[test]
    fn test_validator_clone() {
        // Validator<T> does not implement Clone; construct two independent validators
        // with identical constraints and verify they behave identically.
        fn make_validator() -> Validator<i32> {
            Validator::new().constraint("n", Constraints::range(Some(0i32), Some(50i32)))
        }

        let v1 = make_validator();
        let v2 = make_validator();

        // Both accept values in range
        assert!(v1.validate(&25).is_ok());
        assert!(v2.validate(&25).is_ok());

        // Both reject out-of-range values
        assert!(v1.validate(&-1).is_err());
        assert!(v2.validate(&-1).is_err());
        assert!(v1.validate(&51).is_err());
        assert!(v2.validate(&51).is_err());
    }

    #[test]
    fn test_constraint_composition_many() {
        // Five constraints on the same field — all must pass.
        let validator: Validator<String> = Validator::new()
            .constraint("field", Constraints::non_empty())
            .constraint("field", Constraints::min_len(2))
            .constraint("field", Constraints::max_len(64))
            .constraint("field", Constraints::ascii_only())
            .constraint(
                "field",
                Constraints::custom(
                    |s: &String| !s.contains(' '),
                    "must not contain spaces",
                    "no-spaces",
                ),
            );

        assert_eq!(validator.constraint_count(), 5);
        assert!(validator.validate(&"hello_world".to_string()).is_ok());
        // Fails non_empty + min_len
        assert!(validator.validate(&"".to_string()).is_err());
        // Fails ascii_only
        assert!(validator.validate(&"héllo".to_string()).is_err());
        // Fails no-spaces
        assert!(validator.validate(&"hello world".to_string()).is_err());
    }

    #[test]
    fn test_validate_numeric_unsigned() {
        let validator: Validator<u32> =
            Validator::new().constraint("count", Constraints::range(Some(1u32), Some(100u32)));

        assert!(validator.validate(&1).is_ok());
        assert!(validator.validate(&50).is_ok());
        assert!(validator.validate(&100).is_ok());
        assert!(validator.validate(&0).is_err()); // below min
        assert!(validator.validate(&101).is_err()); // above max
    }

    #[test]
    fn test_validate_numeric_float() {
        let validator: Validator<f64> =
            Validator::new().constraint("prob", Constraints::range(Some(0.0f64), Some(1.0f64)));

        assert!(validator.validate(&0.0).is_ok());
        assert!(validator.validate(&0.5).is_ok());
        assert!(validator.validate(&1.0).is_ok());
        assert!(validator.validate(&-0.001).is_err());
        assert!(validator.validate(&1.001).is_err());
    }

    #[test]
    fn test_validate_u8_boundary() {
        let validator: Validator<u8> =
            Validator::new().constraint("byte", Constraints::range(Some(5u8), Some(200u8)));

        // Exact boundary: min=5 passes, value=4 fails
        assert!(validator.validate(&5).is_ok());
        assert!(validator.validate(&4).is_err());
        // Exact boundary: max=200 passes, value=201 fails
        assert!(validator.validate(&200).is_ok());
        assert!(validator.validate(&201).is_err());
    }

    #[test]
    fn test_validate_collection_min_max() {
        let validator: Validator<Vec<String>> = Validator::new()
            .constraint("tags", Constraints::min_len(2))
            .constraint("tags", Constraints::max_len(5));

        let ok: Vec<String> = vec!["a".into(), "b".into()];
        let ok_max: Vec<String> = (0..5).map(|i| format!("tag{}", i)).collect();
        let too_short: Vec<String> = vec!["only_one".into()];
        let too_long: Vec<String> = (0..6).map(|i| format!("tag{}", i)).collect();

        assert!(validator.validate(&ok).is_ok());
        assert!(validator.validate(&ok_max).is_ok());
        assert!(validator.validate(&too_short).is_err());
        assert!(validator.validate(&too_long).is_err());
    }

    #[test]
    fn test_fail_fast_stops_at_first_error() {
        let config = ValidationConfig::new().with_fail_fast(true);
        let mut validator: Validator<String> = Validator::with_config(config);
        // Three independently failing constraints for ""
        validator.add_constraint("f", Constraints::non_empty());
        validator.add_constraint("f", Constraints::min_len(5));
        validator.add_constraint("f", Constraints::ascii_only()); // would pass, but stops earlier

        let result = validator.validate(&"".to_string());
        assert!(result.is_err());
        assert_eq!(
            result.err().map(|e| e.len()),
            Some(1),
            "fail_fast must stop after the very first failing constraint"
        );
    }

    #[test]
    fn test_collect_all_returns_multiple_errors() {
        let config = ValidationConfig::new().with_fail_fast(false);
        let mut validator: Validator<String> = Validator::with_config(config);
        // "x" (len 1): fails max_len(0) and min_len(5) — both should be collected.
        validator.add_constraint("f", Constraints::max_len(0));
        validator.add_constraint("f", Constraints::min_len(5));

        let result = validator.validate(&"x".to_string());
        assert!(result.is_err());
        let errors = result.unwrap_err();
        assert!(
            errors.len() >= 2,
            "collect-all mode must return all errors, got {}",
            errors.len()
        );
    }

    #[test]
    fn test_string_non_empty_constraint() {
        let validator: Validator<String> =
            Validator::new().constraint("text", Constraints::non_empty());

        assert!(validator.validate(&"a".to_string()).is_ok());
        assert!(validator.validate(&" ".to_string()).is_ok()); // space is still non-empty
        assert!(validator.validate(&"".to_string()).is_err());
    }

    #[test]
    fn test_string_ascii_only_passes_ascii() {
        let validator: Validator<String> =
            Validator::new().constraint("data", Constraints::ascii_only());

        assert!(validator.validate(&"hello123".to_string()).is_ok());
        assert!(validator.validate(&"!@#$%^&*()".to_string()).is_ok());
        assert!(validator.validate(&"UPPER lower 0-9".to_string()).is_ok());
    }

    #[test]
    fn test_string_ascii_only_rejects_unicode() {
        let validator: Validator<String> =
            Validator::new().constraint("data", Constraints::ascii_only());

        assert!(validator.validate(&"héllo".to_string()).is_err()); // accented e
        assert!(validator.validate(&"日本語".to_string()).is_err());
        assert!(validator.validate(&"emoji 🦀".to_string()).is_err());
    }

    #[test]
    fn test_validator_with_config_default() {
        // Validator::new() uses ValidationConfig::default() which has fail_fast = true.
        let validator: Validator<String> = Validator::new();
        // Confirm default behaviour: adding two failing constraints yields only 1 error.
        let validator = validator
            .constraint("f", Constraints::max_len(0))
            .constraint("f", Constraints::min_len(10));

        let result = validator.validate(&"hi".to_string());
        assert!(result.is_err());
        assert_eq!(
            result.err().map(|e| e.len()),
            Some(1),
            "default config (fail_fast=true) must return exactly 1 error"
        );
    }

    #[test]
    fn test_validate_large_string_limit() {
        let validator: Validator<String> =
            Validator::new().constraint("payload", Constraints::max_len(1024));

        let exactly_1024 = "x".repeat(1024);
        let over_limit = "x".repeat(1025);

        assert!(
            validator.validate(&exactly_1024).is_ok(),
            "1024-char string must pass max_len(1024)"
        );
        assert!(
            validator.validate(&over_limit).is_err(),
            "1025-char string must fail max_len(1024)"
        );
    }

    // --- 20 additional validation middleware tests ---

    #[test]
    fn test_max_size_large_payload() {
        // 1. Validator with max_size limit for a large payload
        let validator: Validator<String> =
            Validator::new().constraint("payload", Constraints::max_len(65536));

        let exactly_65536 = "x".repeat(65536);
        let over_limit = "x".repeat(65537);

        assert!(
            validator.validate(&exactly_65536).is_ok(),
            "65536-char string must pass max_len(65536)"
        );
        assert!(
            validator.validate(&over_limit).is_err(),
            "65537-char string must fail max_len(65536)"
        );
    }

    #[test]
    fn test_version_compatibility_range_check() {
        // 2. Version compatibility range check using versioning module
        use oxicode::versioning::{
            decode_versioned_with_check, encode_versioned, CompatibilityLevel, Version,
        };

        let payload = b"schema_data";
        let data_version = Version::new(1, 5, 0);
        let current_version = Version::new(1, 8, 0);
        let min_compat = Some(Version::new(1, 0, 0));

        let encoded =
            encode_versioned(payload, data_version).expect("encode_versioned should succeed");
        let result = decode_versioned_with_check(&encoded, current_version, min_compat);
        assert!(
            result.is_ok(),
            "version 1.5.0 should be compatible with 1.8.0 (min 1.0.0)"
        );
        let (decoded_payload, ver, compat) = result.expect("decode must succeed");
        assert_eq!(decoded_payload, payload);
        assert_eq!(ver, data_version);
        assert!(
            matches!(
                compat,
                CompatibilityLevel::Compatible | CompatibilityLevel::CompatibleWithWarnings
            ),
            "compatibility level should be Compatible or CompatibleWithWarnings"
        );
    }

    #[test]
    fn test_versioned_roundtrip_v1() {
        // 3. Versioned roundtrip with version 1 data
        use oxicode::versioning::{decode_versioned, encode_versioned, Version};

        let original = b"version one payload";
        let v1 = Version::new(1, 0, 0);

        let encoded = encode_versioned(original, v1).expect("encode_versioned v1 failed");
        let (decoded, version) = decode_versioned(&encoded).expect("decode_versioned v1 failed");

        assert_eq!(decoded.as_slice(), original);
        assert_eq!(version, v1);
    }

    #[test]
    fn test_versioned_roundtrip_v2() {
        // 4. Versioned roundtrip with version 2 data
        use oxicode::versioning::{decode_versioned, encode_versioned, Version};

        let original = b"version two extended payload with more fields";
        let v2 = Version::new(2, 0, 0);

        let encoded = encode_versioned(original, v2).expect("encode_versioned v2 failed");
        let (decoded, version) = decode_versioned(&encoded).expect("decode_versioned v2 failed");

        assert_eq!(decoded.as_slice(), original);
        assert_eq!(version, v2);
    }

    #[test]
    fn test_chaining_multiple_rules() {
        // 5. Validator chaining multiple rules
        let validator: Validator<String> = Validator::new()
            .constraint("tag", Constraints::non_empty())
            .constraint("tag", Constraints::min_len(2))
            .constraint("tag", Constraints::max_len(32))
            .constraint("tag", Constraints::ascii_only())
            .constraint(
                "tag",
                Constraints::custom(
                    |s: &String| {
                        s.chars()
                            .all(|c| c.is_ascii_alphanumeric() || c == '_' || c == '-')
                    },
                    "only alphanumeric, underscore, or hyphen allowed",
                    "slug constraint",
                ),
            );

        assert_eq!(validator.constraint_count(), 5);
        assert!(validator.validate(&"rust-lang".to_string()).is_ok());
        assert!(validator.validate(&"my_tag_42".to_string()).is_ok());
        assert!(validator.validate(&"".to_string()).is_err()); // fails non_empty
        assert!(validator.validate(&"x".to_string()).is_err()); // fails min_len(2)
        assert!(validator.validate(&"has space".to_string()).is_err()); // fails slug
        assert!(validator.validate(&"café".to_string()).is_err()); // fails ascii_only
    }

    #[test]
    fn test_nested_struct_field_validators() {
        // 6. Validation of nested struct with separate field validators
        let name_validator: Validator<String> = Validator::new()
            .constraint("name", Constraints::min_len(1))
            .constraint("name", Constraints::max_len(64))
            .constraint("name", Constraints::ascii_only());

        let age_validator: Validator<i32> =
            Validator::new().constraint("age", Constraints::range(Some(0i32), Some(150i32)));

        let email_validator: Validator<String> = Validator::new()
            .constraint("email", Constraints::min_len(3))
            .constraint("email", Constraints::max_len(255))
            .constraint("email", Constraints::ascii_only())
            .constraint(
                "email",
                Constraints::custom(|s: &String| s.contains('@'), "must contain @", "email-at"),
            );

        assert!(name_validator.validate(&"Alice".to_string()).is_ok());
        assert!(name_validator.validate(&"".to_string()).is_err());
        assert!(age_validator.validate(&30).is_ok());
        assert!(age_validator.validate(&200).is_err());
        assert!(email_validator
            .validate(&"alice@example.com".to_string())
            .is_ok());
        assert!(email_validator
            .validate(&"not-an-email".to_string())
            .is_err());
    }

    #[test]
    fn test_vec_u32_length_constraint() {
        // 7. Validate a Vec<u32> with length constraint
        let validator: Validator<Vec<u32>> = Validator::new()
            .constraint("ids", Constraints::min_len(1))
            .constraint("ids", Constraints::max_len(100))
            .constraint("ids", Constraints::non_empty());

        let valid: Vec<u32> = (1..=10).collect();
        let exactly_100: Vec<u32> = (0..100).collect();
        let empty: Vec<u32> = vec![];
        let too_large: Vec<u32> = (0..101).collect();

        assert!(validator.validate(&valid).is_ok());
        assert!(validator.validate(&exactly_100).is_ok());
        assert!(validator.validate(&empty).is_err());
        assert!(validator.validate(&too_large).is_err());
    }

    #[test]
    fn test_validation_failure_error_type() {
        // 8. Validation failure returns proper error type
        let validator: Validator<String> =
            Validator::new().constraint("field", Constraints::min_len(10));

        let result = validator.validate(&"short".to_string());
        assert!(result.is_err());

        let errors = result.expect_err("must have errors");
        assert!(!errors.is_empty(), "error list must not be empty");
        let first = &errors[0];
        assert_eq!(first.field, "field");
        assert!(!first.message.is_empty(), "error message must not be empty");
    }

    #[test]
    fn test_validate_or_default_schema_migration() {
        // 9. validate_or_default with schema migration (clamp to valid range)
        let validator: Validator<i32> =
            Validator::new().constraint("score", Constraints::range(Some(0i32), Some(100i32)));

        // Old data might have score -1 (invalid), migrated default is 0.
        assert_eq!(validator.validate_or_default(-1, 0), 0);
        // Old data with 255 (invalid), migrated default is 100.
        assert_eq!(validator.validate_or_default(255, 100), 100);
        // Valid data is preserved.
        assert_eq!(validator.validate_or_default(75, 0), 75);
    }

    #[test]
    fn test_custom_validation_function() {
        // 10. Validator with custom validation function
        let validator: Validator<String> = Validator::new().constraint(
            "hex_color",
            Constraints::custom(
                |s: &String| {
                    s.starts_with('#')
                        && s.len() == 7
                        && s[1..].chars().all(|c| c.is_ascii_hexdigit())
                },
                "must be a valid #RRGGBB hex color",
                "hex-color",
            ),
        );

        assert!(validator.validate(&"#ff0000".to_string()).is_ok());
        assert!(validator.validate(&"#000000".to_string()).is_ok());
        assert!(validator.validate(&"#ffffff".to_string()).is_ok());
        assert!(validator.validate(&"ff0000".to_string()).is_err()); // missing #
        assert!(validator.validate(&"#gg0000".to_string()).is_err()); // invalid hex
        assert!(validator.validate(&"#fff".to_string()).is_err()); // too short
    }

    #[test]
    fn test_string_length_bounds() {
        // 11. Validate String length bounds
        let validator: Validator<String> = Validator::new()
            .constraint("bio", Constraints::min_len(10))
            .constraint("bio", Constraints::max_len(500));

        let exactly_10 = "a".repeat(10);
        let exactly_500 = "z".repeat(500);
        let too_short = "tiny".to_string();
        let too_long = "x".repeat(501);

        assert!(validator.validate(&exactly_10).is_ok());
        assert!(validator.validate(&exactly_500).is_ok());
        assert!(validator.validate(&too_short).is_err());
        assert!(validator.validate(&too_long).is_err());
    }

    #[test]
    fn test_numeric_bounds_i64() {
        // 12. Validate numeric bounds (min/max value check)
        let validator: Validator<i64> = Validator::new().constraint(
            "timestamp",
            Constraints::range(Some(0i64), Some(9_999_999_999i64)),
        );

        assert!(validator.validate(&0i64).is_ok());
        assert!(validator.validate(&1_000_000_000i64).is_ok());
        assert!(validator.validate(&9_999_999_999i64).is_ok());
        assert!(validator.validate(&-1i64).is_err());
        assert!(validator.validate(&10_000_000_000i64).is_err());
    }

    #[test]
    fn test_batch_validation_multiple_items() {
        // 13. Batch validation of multiple items
        let validator: Validator<i32> =
            Validator::new().constraint("item", Constraints::range(Some(0i32), Some(99i32)));

        let items: Vec<i32> = vec![0, 1, 50, 99];
        let invalid_items: Vec<i32> = vec![-1, 100, 200];

        let valid_results: Vec<bool> = items
            .iter()
            .map(|v| validator.validate(v).is_ok())
            .collect();
        assert!(
            valid_results.iter().all(|&ok| ok),
            "all valid items should pass"
        );

        let invalid_results: Vec<bool> = invalid_items
            .iter()
            .map(|v| validator.validate(v).is_err())
            .collect();
        assert!(
            invalid_results.iter().all(|&err| err),
            "all invalid items should fail"
        );
    }

    #[test]
    fn test_validation_passes_for_empty_unconstrained() {
        // 14. Validation passes for empty data (no constraints)
        let str_validator: Validator<String> = Validator::new();
        assert!(str_validator.validate(&String::new()).is_ok());

        let vec_validator: Validator<Vec<u8>> = Validator::new();
        assert!(vec_validator.validate(&Vec::new()).is_ok());

        let int_validator: Validator<i32> = Validator::new();
        assert!(int_validator.validate(&0).is_ok());
        assert!(int_validator.validate(&i32::MIN).is_ok());
        assert!(int_validator.validate(&i32::MAX).is_ok());
    }

    #[test]
    fn test_version_range_too_old_rejected() {
        // 15. Version range validation (too old version rejected)
        use oxicode::versioning::{decode_versioned_with_check, encode_versioned, Version};

        let payload = b"old_format_data";
        let old_version = Version::new(1, 0, 0);
        let current_version = Version::new(3, 0, 0);
        // min_compat is 2.0.0, so 1.0.0 should be incompatible
        let min_compat = Some(Version::new(2, 0, 0));

        let encoded =
            encode_versioned(payload, old_version).expect("encode_versioned should succeed");
        let result = decode_versioned_with_check(&encoded, current_version, min_compat);

        assert!(
            result.is_err(),
            "version 1.0.0 should be rejected when min_compat is 2.0.0"
        );
    }

    #[test]
    fn test_version_range_too_new_rejected() {
        // 16. Version range validation (too new version rejected)
        use oxicode::versioning::{decode_versioned_with_check, encode_versioned, Version};

        let payload = b"future_format_data";
        let future_version = Version::new(5, 0, 0);
        let current_version = Version::new(1, 0, 0);

        let encoded =
            encode_versioned(payload, future_version).expect("encode_versioned should succeed");
        let result = decode_versioned_with_check(&encoded, current_version, None);

        assert!(
            result.is_err(),
            "version 5.0.0 should be rejected when current is 1.0.0"
        );
    }

    #[test]
    fn test_validator_with_checksum_config() {
        // 17. Validator with checksum enabled via config
        let config = ValidationConfig::new()
            .with_checksum(true)
            .with_fail_fast(false);

        assert!(config.verify_checksum, "verify_checksum should be true");
        assert!(!config.fail_fast, "fail_fast should be false");

        let validator: Validator<i32> = Validator::with_config(config)
            .constraint("value", Constraints::range(Some(0i32), Some(1000i32)));

        assert!(validator.validate(&500).is_ok());
        assert!(validator.validate(&-1).is_err());
    }

    #[test]
    fn test_schema_evolution_v1_bytes_pass_v2_validator() {
        // 18. Schema evolution: V1 validates, V2 extends, V1 bytes pass V2 validator
        use oxicode::versioning::{
            decode_versioned, decode_versioned_with_check, encode_versioned, CompatibilityLevel,
            Version,
        };

        let v1_payload = b"field_a:hello";
        let v1 = Version::new(1, 0, 0);
        let v2 = Version::new(1, 1, 0); // minor bump = backwards-compatible
        let min_compat = Some(Version::new(1, 0, 0));

        // Encode as V1
        let v1_encoded = encode_versioned(v1_payload, v1).expect("v1 encode failed");

        // A V2 decoder should accept V1 bytes (same major version, higher minor)
        let result = decode_versioned_with_check(&v1_encoded, v2, min_compat);
        assert!(
            result.is_ok(),
            "V2 decoder must accept V1 bytes for same major"
        );

        let (payload, detected_ver, compat) = result.expect("decode must succeed");
        assert_eq!(payload.as_slice(), v1_payload);
        assert_eq!(detected_ver, v1);
        assert!(
            matches!(
                compat,
                CompatibilityLevel::Compatible | CompatibilityLevel::CompatibleWithWarnings
            ),
            "compat must be Compatible or CompatibleWithWarnings, got {:?}",
            compat
        );

        // Also verify plain decode works
        let (plain_payload, plain_ver) =
            decode_versioned(&v1_encoded).expect("plain decode failed");
        assert_eq!(plain_payload.as_slice(), v1_payload);
        assert_eq!(plain_ver, v1);
    }

    #[test]
    fn test_validator_checksum_verification_config() {
        // 19. Validator with checksum verification enabled
        let config_with_checksum = ValidationConfig::new()
            .with_checksum(true)
            .with_fail_fast(true)
            .with_max_depth(32);

        assert!(config_with_checksum.verify_checksum);
        assert!(config_with_checksum.fail_fast);
        assert_eq!(config_with_checksum.max_depth, 32);

        let validator: Validator<String> = Validator::with_config(config_with_checksum)
            .constraint("data", Constraints::non_empty())
            .constraint("data", Constraints::max_len(256))
            .constraint("data", Constraints::ascii_only());

        assert!(validator
            .validate(&"checksum_test_value".to_string())
            .is_ok());
        assert!(validator.validate(&String::new()).is_err()); // fails non_empty
        assert!(validator.validate(&"日本語".to_string()).is_err()); // fails ascii_only
        assert_eq!(validator.constraint_count(), 3);
    }

    #[test]
    fn test_roundtrip_encode_validate_decode_preserves_data() {
        // 20. Roundtrip encode -> validate -> decode preserves data
        use oxicode::versioning::{decode_versioned, encode_versioned, Version};

        let version_validator: Validator<Vec<u8>> = Validator::new()
            .constraint("payload", Constraints::min_len(1))
            .constraint("payload", Constraints::max_len(1024));

        // Simulate a payload (raw bytes representing structured data)
        let original_data: Vec<u8> = (0u8..=127u8).collect();
        let version = Version::new(1, 0, 0);

        // Encode with version header
        let encoded = encode_versioned(&original_data, version).expect("encode_versioned failed");

        // Decode to get payload bytes
        let (payload, ver) = decode_versioned(&encoded).expect("decode_versioned failed");

        // Validate the payload
        assert!(
            version_validator.validate(&payload).is_ok(),
            "decoded payload must pass length constraints"
        );

        // Assert round-trip integrity
        assert_eq!(payload, original_data);
        assert_eq!(ver, version);
    }
}
