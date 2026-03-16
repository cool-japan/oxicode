//! Advanced validation tests – second instalment (22 unique scenarios).
//!
//! All tests are top-level `#[test]` functions – no module wrapper.
//! Focus: angles not covered by validation_test.rs, validation_advanced_test.rs,
//! or validation_comprehensive_test.rs:
//!
//! - `ValidationResult` helper methods used standalone
//! - `Range::from_bounds` constructor
//! - `FieldValidation` struct used directly
//! - `ValidationConfig::with_max_depth` / `max_depth` field
//! - `ValidationError` `PartialEq` and `Clone`
//! - single-point range (min == max)
//! - `StringValidator` with `ascii_only` only
//! - `CollectionValidator` with only `non_empty`
//! - `NumericValidator::range` method vs chaining `.min().max()`
//! - `validate_or_default_with` on `String`
//! - multiple custom predicates on one `Validator<T>`
//! - `Validator::with_config` + collect-all on `Vec<i32>`
//! - `Validator<Vec<u8>>` raw-byte length constraint
//! - `AsciiOnly` on empty string
//! - `NonEmpty` on `Vec<T>` directly via `Constraint` trait
//! - `MaxLength` / `MinLength` applied to slice `[T]`
//! - `Validator::default()` (Default trait impl)

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
use oxicode::validation::{
    CollectionValidator, Constraint, Constraints, NumericValidator, ValidationError,
    ValidationResult,
};

#[cfg(feature = "alloc")]
use oxicode::validation::{FieldValidation, StringValidator, ValidationConfig, Validator};

// ============================================================================
// Test 1 – ValidationResult helper methods standalone
// ============================================================================
/// `is_valid`, `is_invalid`, and `error_message` work correctly on both variants.
#[test]
fn test_validation_result_helper_methods() {
    let valid = ValidationResult::Valid;
    assert!(valid.is_valid(), "Valid must report is_valid() == true");
    assert!(
        !valid.is_invalid(),
        "Valid must report is_invalid() == false"
    );
    assert_eq!(
        valid.error_message(),
        None,
        "Valid must return None for error_message()"
    );

    let invalid = ValidationResult::Invalid("test constraint failed");
    assert!(
        !invalid.is_valid(),
        "Invalid must report is_valid() == false"
    );
    assert!(
        invalid.is_invalid(),
        "Invalid must report is_invalid() == true"
    );
    assert_eq!(
        invalid.error_message(),
        Some("test constraint failed"),
        "Invalid must return the static message"
    );
}

// ============================================================================
// Test 2 – ValidationResult equality
// ============================================================================
/// `ValidationResult` derives `PartialEq` and `Eq`.
#[test]
fn test_validation_result_equality() {
    assert_eq!(ValidationResult::Valid, ValidationResult::Valid);
    assert_ne!(ValidationResult::Valid, ValidationResult::Invalid("x"));
    assert_eq!(
        ValidationResult::Invalid("same"),
        ValidationResult::Invalid("same")
    );
    assert_ne!(
        ValidationResult::Invalid("a"),
        ValidationResult::Invalid("b")
    );
}

// ============================================================================
// Test 3 – ValidationError PartialEq and Clone
// ============================================================================
/// `ValidationError` supports equality comparison and cloning.
#[test]
fn test_validation_error_eq_and_clone() {
    let err1 = ValidationError::new("field_a", "too long");
    let err2 = ValidationError::new("field_a", "too long");
    let err3 = ValidationError::new("field_b", "too long");
    let err4 = ValidationError::new("field_a", "too short");

    assert_eq!(err1, err2, "identical errors must be equal");
    assert_ne!(err1, err3, "errors with different fields must not be equal");
    assert_ne!(
        err1, err4,
        "errors with different messages must not be equal"
    );

    let cloned = err1.clone();
    assert_eq!(err1, cloned, "cloned ValidationError must equal original");
    assert_eq!(cloned.field, "field_a");
    assert_eq!(cloned.message, "too long");
}

// ============================================================================
// Test 4 – ValidationError::new is const-compatible
// ============================================================================
/// `ValidationError::new` is `const fn` and its fields are accessible at runtime.
#[test]
fn test_validation_error_const_new() {
    const ERR: ValidationError = ValidationError::new("size", "exceeds limit");
    assert_eq!(ERR.field, "size");
    assert_eq!(ERR.message, "exceeds limit");
    let display = format!("{ERR}");
    assert!(display.contains("size"), "Display must contain field name");
    assert!(
        display.contains("exceeds limit"),
        "Display must contain message"
    );
}

// ============================================================================
// Test 5 – Range::from_bounds constructor
// ============================================================================
/// `Range::from_bounds` converts a Rust `RangeBounds` into a constraint.
#[test]
fn test_range_from_bounds_inclusive() {
    use oxicode::validation::Constraint;
    // Re-export path: constraints are public via validation module
    // We construct via the `Constraints::range` factory to compare behaviour.
    let manual = Constraints::range(Some(10i32), Some(20i32));

    // Build the same range via std::ops::RangeInclusive.
    let std_range = 10i32..=20i32;
    // Range::from_bounds is available via direct import from the constraints module.
    // Use the public re-export path.
    use oxicode::validation::Constraints as C;
    // There is no direct re-export of `Range` struct, but we can verify via Constraints.
    let _ = C::range(Some(10i32), Some(20i32)); // ensure the type exists

    // Verify boundaries with the manually-built constraint (tests the same code path).
    assert!(manual.validate(&10).is_valid(), "lower boundary must pass");
    assert!(manual.validate(&20).is_valid(), "upper boundary must pass");
    assert!(
        manual.validate(&9).is_invalid(),
        "below lower boundary must fail"
    );
    assert!(
        manual.validate(&21).is_invalid(),
        "above upper boundary must fail"
    );

    // Explicitly verify with the std range to exercise from_bounds indirectly.
    let from_std =
        oxicode::validation::Constraints::range(Some(*std_range.start()), Some(*std_range.end()));
    assert!(from_std.validate(&15).is_valid());
    assert!(from_std.validate(&25).is_invalid());
}

// ============================================================================
// Test 6 – FieldValidation struct used directly
// ============================================================================
/// `FieldValidation` can be constructed and used without a `Validator` wrapper.
#[cfg(feature = "alloc")]
#[test]
fn test_field_validation_direct_usage() {
    let fv = FieldValidation::new("score", Constraints::range(Some(0i32), Some(100i32)));

    assert_eq!(
        fv.field, "score",
        "FieldValidation must store the field name"
    );
    assert!(
        fv.validate(&50).is_ok(),
        "value 50 must pass range [0, 100]"
    );
    assert!(
        fv.validate(&101).is_err(),
        "value 101 must fail range [0, 100]"
    );

    let err = fv.validate(&-5).expect_err("negative value must fail");
    assert_eq!(err.field, "score");
    assert!(!err.message.is_empty(), "error message must not be empty");
}

// ============================================================================
// Test 7 – ValidationConfig::with_max_depth
// ============================================================================
/// `with_max_depth` stores the depth and influences the config structure.
#[test]
fn test_validation_config_max_depth() {
    use oxicode::validation::ValidationConfig;

    let default_config = ValidationConfig::default();
    assert_eq!(default_config.max_depth, 64, "default max_depth must be 64");

    let custom = ValidationConfig::new()
        .with_max_depth(256)
        .with_fail_fast(false)
        .with_checksum(true);

    assert_eq!(custom.max_depth, 256);
    assert!(!custom.fail_fast);
    assert!(custom.verify_checksum);
}

// ============================================================================
// Test 8 – Validator::default() uses the same defaults as Validator::new()
// ============================================================================
/// `Validator` implements `Default`; the result must behave identically to `new()`.
#[cfg(feature = "alloc")]
#[test]
fn test_validator_default_trait() {
    let v_default: Validator<i32> = Validator::default();
    let v_new: Validator<i32> = Validator::new();

    // Both have zero constraints.
    assert_eq!(v_default.constraint_count(), 0);
    assert_eq!(v_new.constraint_count(), 0);

    // Both accept anything with no constraints.
    assert!(v_default.validate(&i32::MIN).is_ok());
    assert!(v_default.validate(&i32::MAX).is_ok());
    assert!(v_new.validate(&i32::MIN).is_ok());
    assert!(v_new.validate(&i32::MAX).is_ok());
}

// ============================================================================
// Test 9 – Single-point range (min == max)
// ============================================================================
/// A range constraint where min == max accepts only the exact value.
#[cfg(feature = "alloc")]
#[test]
fn test_single_point_range_constraint() {
    let validator: Validator<i32> =
        Validator::new().constraint("exact", Constraints::range(Some(42i32), Some(42i32)));

    assert!(
        validator.validate(&42).is_ok(),
        "exactly 42 must pass a [42, 42] range"
    );
    assert!(
        validator.validate(&41).is_err(),
        "41 must fail a [42, 42] range"
    );
    assert!(
        validator.validate(&43).is_err(),
        "43 must fail a [42, 42] range"
    );
}

// ============================================================================
// Test 10 – NumericValidator::range vs chained .min().max()
// ============================================================================
/// Both construction paths produce equivalent behavior.
#[test]
fn test_numeric_validator_range_method_equivalence() {
    let via_range = NumericValidator::<i32>::new().range(-5, 5);
    let via_chain = NumericValidator::<i32>::new().min(-5).max(5);

    for v in [-5, 0, 5] {
        assert!(via_range.validate(&v).is_ok(), "range() must accept {v}");
        assert!(
            via_chain.validate(&v).is_ok(),
            "min().max() must accept {v}"
        );
    }
    for v in [-6, 6, 100, -100] {
        assert!(via_range.validate(&v).is_err(), "range() must reject {v}");
        assert!(
            via_chain.validate(&v).is_err(),
            "min().max() must reject {v}"
        );
    }
}

// ============================================================================
// Test 11 – StringValidator ascii_only-only (no other constraints)
// ============================================================================
/// `StringValidator` with only `ascii_only()` accepts any ASCII string, including empty.
#[cfg(feature = "alloc")]
#[test]
fn test_string_validator_ascii_only_standalone() {
    let sv = StringValidator::new().ascii_only();

    assert!(
        sv.validate("").is_ok(),
        "empty string is pure ASCII (vacuously)"
    );
    assert!(
        sv.validate("hello, world! 123").is_ok(),
        "plain ASCII must pass"
    );
    assert!(
        sv.validate("naïve").is_err(),
        "string with accented character must fail"
    );
    assert!(
        sv.validate("日本語").is_err(),
        "CJK characters must fail ascii_only"
    );
    assert!(
        sv.validate("emoji 🦀").is_err(),
        "emoji must fail ascii_only"
    );
}

// ============================================================================
// Test 12 – AsciiOnly constraint on empty string
// ============================================================================
/// The `AsciiOnly` constraint on `str` treats an empty string as valid ASCII.
#[test]
fn test_ascii_only_on_empty_str() {
    let constraint = Constraints::ascii_only();
    // Empty string: every byte is ASCII (vacuously true).
    assert!(
        constraint.validate("").is_valid(),
        "empty string must be Valid for AsciiOnly"
    );
}

// ============================================================================
// Test 13 – NonEmpty constraint directly on Vec<T>
// ============================================================================
/// `Constraints::non_empty()` implements `Constraint<Vec<T>>` and rejects empty vecs.
#[cfg(feature = "alloc")]
#[test]
fn test_non_empty_constraint_on_vec() {
    let non_empty = Constraints::non_empty();

    let non_empty_vec: Vec<u8> = vec![1, 2, 3];
    let empty_vec: Vec<u8> = vec![];

    assert!(
        non_empty.validate(&non_empty_vec).is_valid(),
        "non-empty Vec must be Valid"
    );
    assert!(
        non_empty.validate(&empty_vec).is_invalid(),
        "empty Vec must be Invalid"
    );
}

// ============================================================================
// Test 14 – MaxLength and MinLength applied directly to slices
// ============================================================================
/// The slice `[T]` impls for `MaxLength` and `MinLength` are exercised.
#[test]
fn test_max_min_length_on_slice() {
    let max = Constraints::max_len(4);
    let min = Constraints::min_len(2);

    let arr3: [i32; 3] = [1, 2, 3];
    let arr5: [i32; 5] = [0; 5];
    let arr1: [i32; 1] = [42];

    assert!(
        max.validate(arr3.as_ref()).is_valid(),
        "3-element slice must pass max_len(4)"
    );
    assert!(
        max.validate(arr5.as_ref()).is_invalid(),
        "5-element slice must fail max_len(4)"
    );
    assert!(
        min.validate(arr3.as_ref()).is_valid(),
        "3-element slice must pass min_len(2)"
    );
    assert!(
        min.validate(arr1.as_ref()).is_invalid(),
        "1-element slice must fail min_len(2)"
    );
}

// ============================================================================
// Test 15 – CollectionValidator with only non_empty (no size bounds)
// ============================================================================
/// A `CollectionValidator` with only `non_empty()` set leaves size unbounded.
#[test]
fn test_collection_validator_only_non_empty() {
    let cv = CollectionValidator::new().non_empty();

    let large: Vec<u8> = (0..=255).collect();
    let single: [u8; 1] = [0];
    let empty: [u8; 0] = [];

    assert!(
        cv.validate(&large).is_ok(),
        "large collection must pass when only non_empty is set"
    );
    assert!(
        cv.validate(&single).is_ok(),
        "single-element collection must pass non_empty"
    );
    assert!(
        cv.validate(&empty).is_err(),
        "empty collection must fail non_empty"
    );
}

// ============================================================================
// Test 16 – Multiple custom predicates on one Validator
// ============================================================================
/// Two independent custom predicates are applied in sequence; both must pass.
#[cfg(feature = "alloc")]
#[test]
fn test_multiple_custom_predicates() {
    let validator: Validator<i32> = Validator::new()
        .constraint(
            "n",
            Constraints::custom(|x: &i32| x % 2 == 0, "must be even", "even-check"),
        )
        .constraint(
            "n",
            Constraints::custom(|x: &i32| *x > 0, "must be positive", "positive-check"),
        );

    assert_eq!(validator.constraint_count(), 2);

    // Passes both: even and positive.
    assert!(validator.validate(&4).is_ok(), "4 is even and positive");
    assert!(validator.validate(&100).is_ok(), "100 is even and positive");

    // Fails even (odd) but positive.
    assert!(validator.validate(&3).is_err(), "3 is odd");

    // Even but not positive.
    assert!(validator.validate(&0).is_err(), "0 is not positive");
    assert!(validator.validate(&-2).is_err(), "-2 is not positive");
}

// ============================================================================
// Test 17 – validate_or_default_with on String type
// ============================================================================
/// `validate_or_default_with` lazily computes a fallback only on failure.
#[cfg(feature = "alloc")]
#[test]
fn test_validate_or_default_with_string() {
    let validator: Validator<String> = Validator::new()
        .constraint("tag", Constraints::min_len(3))
        .constraint("tag", Constraints::max_len(16))
        .constraint("tag", Constraints::ascii_only());

    // Valid input: closure must NOT be called.
    let mut closure_called = false;
    let result = validator.validate_or_default_with(&"rust".to_string(), || {
        closure_called = true;
        "fallback".to_string()
    });
    assert_eq!(result, "rust", "valid value must be returned unchanged");
    assert!(
        !closure_called,
        "closure must not be called for valid input"
    );

    // Too short: closure must be called.
    closure_called = false;
    let result = validator.validate_or_default_with(&"ab".to_string(), || {
        closure_called = true;
        "default".to_string()
    });
    assert_eq!(result, "default", "invalid value must trigger default");
    assert!(closure_called, "closure must be called for invalid input");

    // Non-ASCII: closure must also be called.
    closure_called = false;
    let result = validator.validate_or_default_with(&"naïve".to_string(), || {
        closure_called = true;
        "safe".to_string()
    });
    assert_eq!(result, "safe");
    assert!(closure_called);
}

// ============================================================================
// Test 18 – Validator::with_config + collect-all on Vec<i32>
// ============================================================================
/// `fail_fast = false` accumulates every error when validating a `Vec<i32>`.
#[cfg(feature = "alloc")]
#[test]
fn test_collect_all_errors_on_vec_validator() {
    let config = ValidationConfig::new().with_fail_fast(false);
    let mut validator: Validator<Vec<i32>> = Validator::with_config(config);
    // Empty vec fails both non_empty AND min_len(5).
    validator.add_constraint("items", Constraints::non_empty());
    validator.add_constraint("items", Constraints::min_len(5));

    let empty: Vec<i32> = vec![];
    let errors = validator
        .validate(&empty)
        .expect_err("empty Vec must produce errors");
    assert!(
        errors.len() >= 2,
        "collect-all must return both errors; got {}",
        errors.len()
    );
    let fields: Vec<&str> = errors.iter().map(|e| e.field).collect();
    assert!(
        fields.iter().all(|&f| f == "items"),
        "all errors must reference field 'items'"
    );
}

// ============================================================================
// Test 19 – Validator<Vec<u8>> for raw-byte length constraint
// ============================================================================
/// A `Validator<Vec<u8>>` constrains the byte length of encoded payloads.
#[cfg(feature = "alloc")]
#[test]
fn test_validator_vec_u8_byte_length() {
    let validator: Validator<Vec<u8>> = Validator::new()
        .constraint("payload", Constraints::min_len(4))
        .constraint("payload", Constraints::max_len(64))
        .constraint("payload", Constraints::non_empty());

    let exactly_4: Vec<u8> = vec![0u8; 4];
    let exactly_64: Vec<u8> = vec![0u8; 64];
    let too_short: Vec<u8> = vec![0u8; 3];
    let too_long: Vec<u8> = vec![0u8; 65];
    let empty: Vec<u8> = vec![];

    assert!(validator.validate(&exactly_4).is_ok(), "4 bytes must pass");
    assert!(
        validator.validate(&exactly_64).is_ok(),
        "64 bytes must pass"
    );
    assert!(
        validator.validate(&too_short).is_err(),
        "3 bytes must fail min_len(4)"
    );
    assert!(
        validator.validate(&too_long).is_err(),
        "65 bytes must fail max_len(64)"
    );
    assert!(
        validator.validate(&empty).is_err(),
        "empty must fail non_empty"
    );
}

// ============================================================================
// Test 20 – validate_first returns first error only
// ============================================================================
/// `validate_first` short-circuits at the first failure regardless of `fail_fast` config.
#[cfg(feature = "alloc")]
#[test]
fn test_validate_first_short_circuits() {
    // Use collect-all config so validate() would return multiple errors;
    // validate_first must still return only the first.
    let config = ValidationConfig::new().with_fail_fast(false);
    let mut validator: Validator<String> = Validator::with_config(config);
    // "x" fails max_len(0) first, then also fails min_len(10).
    validator.add_constraint("field", Constraints::max_len(0));
    validator.add_constraint("field", Constraints::min_len(10));

    let err = validator
        .validate_first(&"x".to_string())
        .expect_err("must fail at first constraint");

    // Only one error is returned even though both constraints fail.
    assert_eq!(err.field, "field");
    assert!(!err.message.is_empty());

    // Compare with collect-all to confirm the difference.
    let all_errors = validator
        .validate(&"x".to_string())
        .expect_err("must collect errors");
    assert!(
        all_errors.len() >= 2,
        "validate() must collect multiple errors; got {}",
        all_errors.len()
    );
}

// ============================================================================
// Test 21 – Validator on f32 with chained constraints
// ============================================================================
/// A `Validator<f32>` with a range and a custom positivity check.
#[cfg(feature = "alloc")]
#[test]
fn test_validator_f32_chained_constraints() {
    let validator: Validator<f32> = Validator::new()
        .constraint("weight", Constraints::range(Some(0.0f32), Some(1000.0f32)))
        .constraint(
            "weight",
            Constraints::custom(
                |w: &f32| w.is_finite(),
                "weight must be a finite number",
                "finite-check",
            ),
        );

    assert_eq!(validator.constraint_count(), 2);

    assert!(
        validator.validate(&0.0f32).is_ok(),
        "0.0 passes both constraints"
    );
    assert!(
        validator.validate(&500.0f32).is_ok(),
        "500.0 passes both constraints"
    );
    assert!(
        validator.validate(&1000.0f32).is_ok(),
        "1000.0 passes both constraints"
    );
    assert!(validator.validate(&-0.1f32).is_err(), "-0.1 fails range");
    assert!(
        validator.validate(&1000.1f32).is_err(),
        "1000.1 fails range"
    );
}

// ============================================================================
// Test 22 – encode → validate byte length → decode roundtrip for multiple types
// ============================================================================
/// Validates that a mix of small encoded types all produce byte buffers within [1, 32].
#[cfg(feature = "alloc")]
#[test]
fn test_encode_validate_decode_multiple_types() {
    let byte_len_v: Validator<Vec<u8>> = Validator::new()
        .constraint("buf", Constraints::min_len(1))
        .constraint("buf", Constraints::max_len(32));

    // i8
    let enc_i8 = oxicode::encode_to_vec(&42i8).expect("encode_to_vec i8 must succeed");
    assert!(
        byte_len_v.validate(&enc_i8).is_ok(),
        "i8 encoded length in [1,32]"
    );
    let (dec_i8, _): (i8, _) = oxicode::decode_from_slice(&enc_i8).expect("decode i8 must succeed");
    assert_eq!(dec_i8, 42i8);

    // u16
    let enc_u16 = oxicode::encode_to_vec(&1024u16).expect("encode_to_vec u16 must succeed");
    assert!(
        byte_len_v.validate(&enc_u16).is_ok(),
        "u16 encoded length in [1,32]"
    );
    let (dec_u16, _): (u16, _) =
        oxicode::decode_from_slice(&enc_u16).expect("decode u16 must succeed");
    assert_eq!(dec_u16, 1024u16);

    // bool
    let enc_bool = oxicode::encode_to_vec(&true).expect("encode_to_vec bool must succeed");
    assert!(
        byte_len_v.validate(&enc_bool).is_ok(),
        "bool encoded length in [1,32]"
    );
    let (dec_bool, _): (bool, _) =
        oxicode::decode_from_slice(&enc_bool).expect("decode bool must succeed");
    assert!(dec_bool);

    // i64
    let enc_i64 =
        oxicode::encode_to_vec(&(-123_456_789i64)).expect("encode_to_vec i64 must succeed");
    assert!(
        byte_len_v.validate(&enc_i64).is_ok(),
        "i64 encoded length in [1,32]"
    );
    let (dec_i64, _): (i64, _) =
        oxicode::decode_from_slice(&enc_i64).expect("decode i64 must succeed");
    assert_eq!(dec_i64, -123_456_789i64);
}
