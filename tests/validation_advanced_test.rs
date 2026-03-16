//! Advanced validation middleware tests – 22 unique scenarios.
//!
//! All tests are top-level `#[test]` functions (no module wrapper).
//! Tests focus on unique angles not heavily exercised by the other
//! validation test files:
//!
//! - valid struct passes / invalid struct fails
//! - custom range validator (age 0–150)
//! - multiple rules on one validator
//! - nested struct validation (separate per-field validators)
//! - Vec field validation (all elements valid / one invalid via element-level loop)
//! - Option field (Some valid / None always valid via no-constraint path)
//! - String non-empty and max-length constraints
//! - numeric min / max edge-cases
//! - encode → validate → decode roundtrip
//! - ValidationError Display format
//! - collecting multiple errors (non-fail-fast)
//! - custom error message propagation
//! - generic Validator<T> with type parameter
//! - validate_or_default fallback
//! - deeply nested struct composition
//! - builder-pattern chain returning Self
//! - after-decode check via post-decode validator

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
// ── always-available imports ────────────────────────────────────────────────
use oxicode::validation::{CollectionValidator, NumericValidator, ValidationError};

// ── alloc-gated imports ──────────────────────────────────────────────────────
#[cfg(feature = "alloc")]
use oxicode::validation::{Constraints, ValidationConfig, Validator};

// ============================================================================
// Test 1 – valid struct passes validation
// ============================================================================
/// A valid i32 value within [0, 150] must pass the age validator.
#[cfg(feature = "alloc")]
#[test]
fn test_valid_struct_passes_validation() {
    let age_validator: Validator<i32> =
        Validator::new().constraint("age", Constraints::range(Some(0i32), Some(150i32)));

    assert!(
        age_validator.validate(&30).is_ok(),
        "age 30 must pass range [0, 150]"
    );
    assert!(
        age_validator.validate(&0).is_ok(),
        "age 0 (boundary) must pass range [0, 150]"
    );
    assert!(
        age_validator.validate(&150).is_ok(),
        "age 150 (boundary) must pass range [0, 150]"
    );
}

// ============================================================================
// Test 2 – invalid struct fails validation
// ============================================================================
/// An i32 value outside [0, 150] must fail the age validator.
#[cfg(feature = "alloc")]
#[test]
fn test_invalid_struct_fails_validation() {
    let age_validator: Validator<i32> =
        Validator::new().constraint("age", Constraints::range(Some(0i32), Some(150i32)));

    assert!(
        age_validator.validate(&-1).is_err(),
        "age -1 must fail range [0, 150]"
    );
    assert!(
        age_validator.validate(&151).is_err(),
        "age 151 must fail range [0, 150]"
    );
    assert!(
        age_validator.validate(&200).is_err(),
        "age 200 must fail range [0, 150]"
    );
}

// ============================================================================
// Test 3 – custom validator checks age range [0, 150]
// ============================================================================
/// Custom `Constraints::custom` closure correctly gates the age range.
#[cfg(feature = "alloc")]
#[test]
fn test_custom_validator_age_range() {
    let validator: Validator<i32> = Validator::new().constraint(
        "age",
        Constraints::custom(
            |age: &i32| *age >= 0 && *age <= 150,
            "age must be between 0 and 150",
            "age-range-check",
        ),
    );

    assert!(validator.validate(&0).is_ok(), "age 0 passes custom range");
    assert!(
        validator.validate(&75).is_ok(),
        "age 75 passes custom range"
    );
    assert!(
        validator.validate(&150).is_ok(),
        "age 150 passes custom range"
    );
    assert!(
        validator.validate(&-1).is_err(),
        "age -1 fails custom range"
    );
    assert!(
        validator.validate(&151).is_err(),
        "age 151 fails custom range"
    );
}

// ============================================================================
// Test 4 – validator with multiple rules
// ============================================================================
/// A validator with four rules: non_empty, min_len, max_len, ascii_only.
/// Every rule must be satisfied for the value to pass.
#[cfg(feature = "alloc")]
#[test]
fn test_validator_with_multiple_rules() {
    let validator: Validator<String> = Validator::new()
        .constraint("username", Constraints::non_empty())
        .constraint("username", Constraints::min_len(3))
        .constraint("username", Constraints::max_len(20))
        .constraint("username", Constraints::ascii_only());

    assert_eq!(validator.constraint_count(), 4);

    // All rules satisfied.
    assert!(
        validator.validate(&"alice".to_string()).is_ok(),
        "'alice' must satisfy all four rules"
    );

    // Too short.
    assert!(
        validator.validate(&"ab".to_string()).is_err(),
        "'ab' violates min_len(3)"
    );

    // Non-ASCII.
    assert!(
        validator.validate(&"héllo".to_string()).is_err(),
        "non-ASCII must fail ascii_only"
    );

    // Empty.
    assert!(
        validator.validate(&String::new()).is_err(),
        "empty string fails non_empty"
    );
}

// ============================================================================
// Test 5 – nested struct validation
// ============================================================================
/// Separate validators for each logical field of a simulated struct.
#[cfg(feature = "alloc")]
#[test]
fn test_nested_struct_validation() {
    // Simulated "Profile" struct with three fields.
    struct Profile {
        name: String,
        age: i32,
        bio: String,
    }

    let name_v: Validator<String> = Validator::new()
        .constraint("name", Constraints::min_len(1))
        .constraint("name", Constraints::max_len(64))
        .constraint("name", Constraints::ascii_only());

    let age_v: Validator<i32> =
        Validator::new().constraint("age", Constraints::range(Some(0i32), Some(150i32)));

    let bio_v: Validator<String> = Validator::new()
        .constraint("bio", Constraints::max_len(512))
        .constraint("bio", Constraints::ascii_only());

    let valid_profile = Profile {
        name: "Bob".to_string(),
        age: 25,
        bio: "Rust developer".to_string(),
    };

    assert!(name_v.validate(&valid_profile.name).is_ok());
    assert!(age_v.validate(&valid_profile.age).is_ok());
    assert!(bio_v.validate(&valid_profile.bio).is_ok());

    let invalid_profile = Profile {
        name: String::new(),  // fails min_len(1)
        age: -5,              // fails range
        bio: "x".repeat(513), // fails max_len(512)
    };

    assert!(name_v.validate(&invalid_profile.name).is_err());
    assert!(age_v.validate(&invalid_profile.age).is_err());
    assert!(bio_v.validate(&invalid_profile.bio).is_err());
}

// ============================================================================
// Test 6 – Vec field validation: all elements valid
// ============================================================================
/// Each element in a Vec passes an individual i32 range validator.
#[cfg(feature = "alloc")]
#[test]
fn test_vec_field_validation_all_elements_valid() {
    let elem_v: Validator<i32> =
        Validator::new().constraint("score", Constraints::range(Some(0i32), Some(100i32)));

    let scores: Vec<i32> = vec![0, 25, 50, 75, 100];
    for score in &scores {
        assert!(
            elem_v.validate(score).is_ok(),
            "score {} must pass range [0, 100]",
            score
        );
    }
}

// ============================================================================
// Test 7 – Vec field validation: one element invalid
// ============================================================================
/// A Vec with one out-of-range element causes at least one validation failure.
#[cfg(feature = "alloc")]
#[test]
fn test_vec_field_validation_one_element_invalid() {
    let elem_v: Validator<i32> =
        Validator::new().constraint("score", Constraints::range(Some(0i32), Some(100i32)));

    let scores: Vec<i32> = vec![10, 50, 200, 80]; // 200 is invalid
    let fail_count = scores
        .iter()
        .filter(|s| elem_v.validate(s).is_err())
        .count();

    assert_eq!(fail_count, 1, "exactly one element (200) must fail");
}

// ============================================================================
// Test 8 – Option field validation: Some(valid) passes
// ============================================================================
/// When the inner value is valid, it passes regardless of the Option wrapper.
#[cfg(feature = "alloc")]
#[test]
fn test_option_field_some_valid() {
    let inner_v: Validator<i32> =
        Validator::new().constraint("count", Constraints::range(Some(1i32), Some(999i32)));

    let value: Option<i32> = Some(42);
    if let Some(ref inner) = value {
        assert!(
            inner_v.validate(inner).is_ok(),
            "Some(42) inner value must pass"
        );
    }
}

// ============================================================================
// Test 9 – Option field validation: None is treated as valid
// ============================================================================
/// When the field is None, no constraint runs and the value is considered valid.
#[cfg(feature = "alloc")]
#[test]
fn test_option_field_none_is_valid() {
    // An unconstrained validator models the "None means absent, always valid" pattern.
    let inner_v: Validator<i32> =
        Validator::new().constraint("count", Constraints::range(Some(1i32), Some(999i32)));

    let value: Option<i32> = None;
    // None → no validation runs → logically valid; verify by confirming
    // we never call validate when None.
    let ran_validation = value.as_ref().map(|v| inner_v.validate(v).is_ok());
    assert_eq!(
        ran_validation, None,
        "None field must skip validation entirely"
    );
}

// ============================================================================
// Test 10 – String field validation: non-empty
// ============================================================================
/// `Constraints::non_empty()` rejects empty strings and accepts non-empty ones.
#[cfg(feature = "alloc")]
#[test]
fn test_string_field_non_empty() {
    let validator: Validator<String> =
        Validator::new().constraint("name", Constraints::non_empty());

    assert!(
        validator.validate(&"a".to_string()).is_ok(),
        "single char must pass non_empty"
    );
    assert!(
        validator.validate(&"  ".to_string()).is_ok(),
        "whitespace is still non-empty"
    );
    assert!(
        validator.validate(&String::new()).is_err(),
        "empty string must fail non_empty"
    );
}

// ============================================================================
// Test 11 – String field validation: max length
// ============================================================================
/// `Constraints::max_len` rejects strings exceeding the limit.
#[cfg(feature = "alloc")]
#[test]
fn test_string_field_max_length() {
    const LIMIT: usize = 32;
    let validator: Validator<String> =
        Validator::new().constraint("token", Constraints::max_len(LIMIT));

    let at_limit = "x".repeat(LIMIT);
    let over_limit = "x".repeat(LIMIT + 1);

    assert!(
        validator.validate(&at_limit).is_ok(),
        "string at exactly max_len must pass"
    );
    assert!(
        validator.validate(&over_limit).is_err(),
        "string one over max_len must fail"
    );
}

// ============================================================================
// Test 12 – Number field validation: min value
// ============================================================================
/// `NumericValidator::min` rejects values below the minimum.
#[test]
fn test_number_field_min_value() {
    let validator = NumericValidator::<i64>::new().min(0i64);

    assert!(validator.validate(&0i64).is_ok(), "0 must pass min(0)");
    assert!(
        validator.validate(&i64::MAX).is_ok(),
        "i64::MAX must pass min(0)"
    );
    assert!(validator.validate(&-1i64).is_err(), "-1 must fail min(0)");
    assert!(
        validator.validate(&i64::MIN).is_err(),
        "i64::MIN must fail min(0)"
    );
}

// ============================================================================
// Test 13 – Number field validation: max value
// ============================================================================
/// `NumericValidator::max` rejects values above the maximum.
#[test]
fn test_number_field_max_value() {
    let validator = NumericValidator::<u32>::new().max(255u32);

    assert!(validator.validate(&0u32).is_ok(), "0 must pass max(255)");
    assert!(
        validator.validate(&255u32).is_ok(),
        "255 (boundary) must pass max(255)"
    );
    assert!(
        validator.validate(&256u32).is_err(),
        "256 must fail max(255)"
    );
}

// ============================================================================
// Test 14 – Encode then validate roundtrip
// ============================================================================
/// Encode a value, validate the resulting byte vector, then decode and confirm integrity.
#[cfg(feature = "alloc")]
#[test]
fn test_encode_then_validate_roundtrip() {
    let byte_validator: Validator<Vec<u8>> = Validator::new()
        .constraint("bytes", Constraints::min_len(1))
        .constraint("bytes", Constraints::max_len(32));

    let original: u64 = 1_234_567_890u64;
    let encoded = oxicode::encode_to_vec(&original).expect("encode_to_vec must succeed");

    assert!(
        byte_validator.validate(&encoded).is_ok(),
        "encoded byte length {} must be in [1, 32]",
        encoded.len()
    );

    let (decoded, consumed): (u64, _) =
        oxicode::decode_from_slice(&encoded).expect("decode_from_slice must succeed");

    assert_eq!(decoded, original, "decoded value must equal original");
    assert_eq!(consumed, encoded.len(), "all bytes must be consumed");
}

// ============================================================================
// Test 15 – ValidationError Display format
// ============================================================================
/// The `Display` impl for `ValidationError` must include both field and message.
#[test]
fn test_validation_error_display_format() {
    let err = ValidationError::new("email", "must contain @");
    let display = format!("{err}");

    assert!(
        display.contains("email"),
        "Display output must contain field name; got: {display}"
    );
    assert!(
        display.contains("must contain @"),
        "Display output must contain error message; got: {display}"
    );
    // Confirm the canonical format produced by our impl.
    assert_eq!(
        display, "validation failed for 'email': must contain @",
        "Display format must match expected pattern"
    );
}

// ============================================================================
// Test 16 – Multiple validation errors collected (non-fail-fast)
// ============================================================================
/// With `fail_fast = false` every failing constraint is collected into the error vec.
#[cfg(feature = "alloc")]
#[test]
fn test_multiple_validation_errors_collected() {
    let config = ValidationConfig::new().with_fail_fast(false);
    let mut validator: Validator<String> = Validator::with_config(config);
    // "x" (len=1):  fails max_len(0)  AND  fails min_len(10)  →  2 errors.
    validator.add_constraint("field", Constraints::max_len(0));
    validator.add_constraint("field", Constraints::min_len(10));

    let errors = validator
        .validate(&"x".to_string())
        .expect_err("both constraints must fail");

    assert!(
        errors.len() >= 2,
        "collect-all mode must return at least 2 errors; got {}",
        errors.len()
    );
}

// ============================================================================
// Test 17 – Validator with custom error message
// ============================================================================
/// A custom constraint propagates its exact static error message.
#[cfg(feature = "alloc")]
#[test]
fn test_validator_custom_error_message() {
    const CUSTOM_MSG: &str = "value must be divisible by three";

    let validator: Validator<i32> = Validator::new().constraint(
        "divisor",
        Constraints::custom(|x: &i32| x % 3 == 0, CUSTOM_MSG, "div-by-3"),
    );

    // Valid multiples.
    assert!(validator.validate(&0).is_ok(), "0 is divisible by 3");
    assert!(validator.validate(&9).is_ok(), "9 is divisible by 3");
    assert!(validator.validate(&-12).is_ok(), "-12 is divisible by 3");

    // Invalid: the message must match exactly.
    let errors = validator
        .validate(&1i32)
        .expect_err("1 must fail the custom constraint");

    assert_eq!(errors.len(), 1);
    assert_eq!(
        errors[0].message, CUSTOM_MSG,
        "error message must be '{CUSTOM_MSG}'"
    );
    assert_eq!(errors[0].field, "divisor");
}

// ============================================================================
// Test 18 – Validator on generic struct (type-parameterised)
// ============================================================================
/// `Validator<T>` works identically for different numeric types via a helper function.
#[cfg(feature = "alloc")]
#[test]
fn test_validator_on_generic_struct() {
    // Helper that creates a range-constrained validator for any PartialOrd + Clone + 'static type.
    fn make_range_validator<T>(min: T, max: T) -> Validator<T>
    where
        T: PartialOrd + Clone + Send + Sync + 'static,
    {
        Validator::new().constraint("value", Constraints::range(Some(min), Some(max)))
    }

    let i32_v = make_range_validator(0i32, 100i32);
    assert!(i32_v.validate(&50i32).is_ok());
    assert!(i32_v.validate(&-1i32).is_err());

    let f64_v = make_range_validator(0.0f64, 1.0f64);
    assert!(f64_v.validate(&0.5f64).is_ok());
    assert!(f64_v.validate(&2.0f64).is_err());

    let u8_v = make_range_validator(10u8, 20u8);
    assert!(u8_v.validate(&15u8).is_ok());
    assert!(u8_v.validate(&5u8).is_err());
}

// ============================================================================
// Test 19 – validate_or_default fallback behaviour
// ============================================================================
/// Valid values are returned unchanged; invalid values yield the provided default.
#[cfg(feature = "alloc")]
#[test]
fn test_validate_or_default_fallback() {
    let validator: Validator<i32> =
        Validator::new().constraint("score", Constraints::range(Some(0i32), Some(100i32)));

    // Valid → return value as-is.
    assert_eq!(
        validator.validate_or_default(75, -1),
        75,
        "valid value must be returned unchanged"
    );
    assert_eq!(
        validator.validate_or_default(0, -1),
        0,
        "lower boundary must be returned unchanged"
    );
    assert_eq!(
        validator.validate_or_default(100, -1),
        100,
        "upper boundary must be returned unchanged"
    );

    // Invalid → return default.
    assert_eq!(
        validator.validate_or_default(-5, 0),
        0,
        "below-min value must yield default"
    );
    assert_eq!(
        validator.validate_or_default(999, 50),
        50,
        "above-max value must yield default"
    );
}

// ============================================================================
// Test 20 – Validation of deeply nested struct
// ============================================================================
/// Three levels of "nesting" validated via independent validators at each level.
#[cfg(feature = "alloc")]
#[test]
fn test_validation_deeply_nested_struct() {
    // Level 1: address line.
    let line_v: Validator<String> = Validator::new()
        .constraint("line", Constraints::min_len(1))
        .constraint("line", Constraints::max_len(100))
        .constraint("line", Constraints::ascii_only());

    // Level 2: postcode.
    let postcode_v: Validator<String> = Validator::new()
        .constraint("postcode", Constraints::min_len(3))
        .constraint("postcode", Constraints::max_len(10))
        .constraint("postcode", Constraints::ascii_only());

    // Level 3: country code (ISO 3166-1 alpha-2 style).
    let country_v: Validator<String> = Validator::new().constraint(
        "country",
        Constraints::custom(
            |s: &String| s.len() == 2 && s.chars().all(|c| c.is_ascii_uppercase()),
            "must be a 2-letter uppercase country code",
            "country-code",
        ),
    );

    let line = "123 Ferris Street".to_string();
    let postcode = "12345".to_string();
    let country = "US".to_string();

    assert!(
        line_v.validate(&line).is_ok(),
        "valid address line must pass"
    );
    assert!(
        postcode_v.validate(&postcode).is_ok(),
        "valid postcode must pass"
    );
    assert!(
        country_v.validate(&country).is_ok(),
        "valid country code must pass"
    );

    // Invalid deeply nested field.
    let bad_country = "united_states".to_string();
    assert!(
        country_v.validate(&bad_country).is_err(),
        "long country string must fail"
    );
}

// ============================================================================
// Test 21 – Validator::new() with builder pattern
// ============================================================================
/// Builder-pattern chaining returns `Self`, allowing fluent construction.
#[cfg(feature = "alloc")]
#[test]
fn test_validator_new_builder_pattern() {
    // Chain: new() → constraint() → constraint() → constraint()
    let validator: Validator<String> = Validator::new()
        .constraint("tag", Constraints::non_empty())
        .constraint("tag", Constraints::min_len(2))
        .constraint("tag", Constraints::max_len(32))
        .constraint("tag", Constraints::ascii_only())
        .constraint(
            "tag",
            Constraints::custom(
                |s: &String| !s.contains(' '),
                "must not contain spaces",
                "no-spaces",
            ),
        );

    assert_eq!(
        validator.constraint_count(),
        5,
        "builder chain must register all 5 constraints"
    );

    // All constraints satisfied.
    assert!(validator.validate(&"rust-lang".to_string()).is_ok());
    assert!(validator.validate(&"my_tag_42".to_string()).is_ok());

    // Fails non_empty + min_len.
    assert!(validator.validate(&String::new()).is_err());
    // Fails no-spaces.
    assert!(validator.validate(&"has space".to_string()).is_err());
    // Fails ascii_only.
    assert!(validator.validate(&"café".to_string()).is_err());
}

// ============================================================================
// Test 22 – Validation with after-decode check
// ============================================================================
/// Encode a value, decode it, then validate the decoded value to confirm integrity.
#[cfg(feature = "alloc")]
#[test]
fn test_validation_with_after_decode_check() {
    // Encode an i32 and then validate it after decoding.
    let post_decode_v: Validator<i32> =
        Validator::new().constraint("value", Constraints::range(Some(1i32), Some(1_000i32)));

    let original: i32 = 42;
    let encoded = oxicode::encode_to_vec(&original).expect("encode_to_vec must succeed");

    let (decoded, _): (i32, _) =
        oxicode::decode_from_slice(&encoded).expect("decode_from_slice must succeed");

    assert_eq!(decoded, original, "decoded value must equal original");
    assert!(
        post_decode_v.validate(&decoded).is_ok(),
        "decoded value {decoded} must pass the post-decode validator"
    );

    // Confirm that a tampered (out-of-range) value fails post-decode validation.
    let tampered: i32 = 9999;
    assert!(
        post_decode_v.validate(&tampered).is_err(),
        "tampered value {tampered} must fail post-decode validation"
    );

    // Also verify CollectionValidator on the raw bytes.
    let byte_cv = CollectionValidator::new()
        .min_len(1)
        .max_len(16)
        .non_empty();

    assert!(
        byte_cv.validate(&encoded).is_ok(),
        "encoded byte slice length {} must satisfy collection constraints",
        encoded.len()
    );
}
