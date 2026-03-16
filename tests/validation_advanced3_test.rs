//! Advanced validation tests – third instalment (22 unique scenarios).
//!
//! All tests are top-level `#[test]` functions – no module wrapper.
//! Focus: angles NOT covered by validation_test.rs, validation_advanced_test.rs,
//! or validation_advanced2_test.rs:
//!
//! 1.  `Range` with min-only (no upper bound)
//! 2.  `Range` with max-only (no lower bound)
//! 3.  `AsciiOnly` applied to `String` (alloc impl)
//! 4.  `NonEmpty` on `&str` directly
//! 5.  `MaxLength` on `str` — exact boundary (at limit)
//! 6.  `MinLength` on `str` — exact boundary (at limit)
//! 7.  `CustomValidator::description()` accessor
//! 8.  `NumericValidator<f32>` with range method
//! 9.  `CollectionValidator::validate_len` at extreme values
//! 10. `StringValidator::default()` identical to `StringValidator::new()`
//! 11. `NumericValidator::default()` accepts all values
//! 12. `CollectionValidator::default()` accepts all lengths
//! 13. `ValidationConfig::with_checksum(false)` resets flag
//! 14. Encode → validate → decode `f32` roundtrip
//! 15. `Validator<u8>` at absolute boundaries (0 and 255)
//! 16. `validate_or_default_with` on `Vec<u8>`
//! 17. `FieldValidation` with `NonEmpty` constraint
//! 18. `StringValidator` with only `max_len` set
//! 19. `StringValidator` with only `min_len` set
//! 20. `Range::from_bounds` with open-ended (unbounded) std range
//! 21. Single `Validator<i32>` with two *different* field names
//! 22. `CollectionValidator` three constraints applied in separate instances

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
use oxicode::validation::{CollectionValidator, Constraint, Constraints, NumericValidator};

#[cfg(feature = "alloc")]
use oxicode::validation::{FieldValidation, StringValidator, ValidationConfig, Validator};

// ============================================================================
// Test 1 – Range with min-only (no upper bound)
// ============================================================================
/// A `Range` that has only a lower bound accepts any value >= min.
#[test]
fn test_range_min_only_no_upper_bound() {
    let min_only = Constraints::range(Some(100i32), None::<i32>);

    assert!(
        min_only.validate(&100).is_valid(),
        "exactly min (100) must pass"
    );
    assert!(
        min_only.validate(&i32::MAX).is_valid(),
        "i32::MAX must pass with no upper bound"
    );
    assert!(
        min_only.validate(&99).is_invalid(),
        "99 must fail when min is 100"
    );
    assert!(
        min_only.validate(&i32::MIN).is_invalid(),
        "i32::MIN must fail when min is 100"
    );
}

// ============================================================================
// Test 2 – Range with max-only (no lower bound)
// ============================================================================
/// A `Range` that has only an upper bound accepts any value <= max.
#[test]
fn test_range_max_only_no_lower_bound() {
    let max_only = Constraints::range(None::<i64>, Some(50i64));

    assert!(
        max_only.validate(&50).is_valid(),
        "exactly max (50) must pass"
    );
    assert!(
        max_only.validate(&i64::MIN).is_valid(),
        "i64::MIN must pass with no lower bound"
    );
    assert!(max_only.validate(&0).is_valid(), "0 must pass with max=50");
    assert!(
        max_only.validate(&51).is_invalid(),
        "51 must fail when max is 50"
    );
    assert!(
        max_only.validate(&i64::MAX).is_invalid(),
        "i64::MAX must fail when max is 50"
    );
}

// ============================================================================
// Test 3 – AsciiOnly on String (alloc impl path)
// ============================================================================
/// The alloc `String` impl of `AsciiOnly` is exercised directly.
#[cfg(feature = "alloc")]
#[test]
fn test_ascii_only_on_string_type() {
    let constraint = Constraints::ascii_only();

    assert!(
        constraint.validate(&"plain ASCII".to_string()).is_valid(),
        "plain ASCII String must pass"
    );
    assert!(
        constraint.validate(&String::new()).is_valid(),
        "empty String must pass AsciiOnly (vacuously)"
    );
    assert!(
        constraint.validate(&"über".to_string()).is_invalid(),
        "String with ü must fail AsciiOnly"
    );
    assert!(
        constraint.validate(&"Ångström".to_string()).is_invalid(),
        "String with Å must fail AsciiOnly"
    );
}

// ============================================================================
// Test 4 – NonEmpty on &str directly
// ============================================================================
/// `NonEmpty` implements `Constraint<str>` and works directly on `&str`.
#[test]
fn test_non_empty_on_str_directly() {
    let non_empty = Constraints::non_empty();

    assert!(
        non_empty.validate("x").is_valid(),
        "single-char str must pass NonEmpty"
    );
    assert!(
        non_empty.validate("hello, world").is_valid(),
        "multi-char str must pass NonEmpty"
    );
    assert!(
        non_empty.validate("").is_invalid(),
        "empty &str must fail NonEmpty"
    );
}

// ============================================================================
// Test 5 – MaxLength on &str — exact boundary
// ============================================================================
/// At exactly the limit the `MaxLength` constraint on `str` passes.
#[test]
fn test_max_length_on_str_exact_boundary() {
    const LIMIT: usize = 8;
    let max = Constraints::max_len(LIMIT);

    let at_limit: &str = "12345678"; // 8 bytes
    let one_over: &str = "123456789"; // 9 bytes
    let under_limit: &str = "abc"; // 3 bytes

    assert!(
        max.validate(at_limit).is_valid(),
        "string of exactly max_len must pass"
    );
    assert!(
        max.validate(one_over).is_invalid(),
        "string one byte over max_len must fail"
    );
    assert!(
        max.validate(under_limit).is_valid(),
        "string under max_len must pass"
    );
}

// ============================================================================
// Test 6 – MinLength on &str — exact boundary
// ============================================================================
/// At exactly the minimum the `MinLength` constraint on `str` passes.
#[test]
fn test_min_length_on_str_exact_boundary() {
    const LIMIT: usize = 5;
    let min = Constraints::min_len(LIMIT);

    let at_limit: &str = "abcde"; // 5 bytes
    let one_under: &str = "abcd"; // 4 bytes
    let over_limit: &str = "abcdef"; // 6 bytes

    assert!(
        min.validate(at_limit).is_valid(),
        "string of exactly min_len must pass"
    );
    assert!(
        min.validate(one_under).is_invalid(),
        "string one byte under min_len must fail"
    );
    assert!(
        min.validate(over_limit).is_valid(),
        "string over min_len must pass"
    );
}

// ============================================================================
// Test 7 – CustomValidator::description() accessor
// ============================================================================
/// The `description` field of a custom validator is accessible via the
/// `Constraint::description()` method.
#[test]
fn test_custom_validator_description_accessor() {
    const DESC: &str = "power-of-two-check";
    let cv = Constraints::custom(
        |x: &u32| x.count_ones() == 1,
        "value must be a power of two",
        DESC,
    );

    assert_eq!(
        cv.description(),
        DESC,
        "description() must return the static description string"
    );
    assert!(cv.validate(&1u32).is_valid(), "1 (2^0) is a power of two");
    assert!(cv.validate(&16u32).is_valid(), "16 (2^4) is a power of two");
    assert!(cv.validate(&3u32).is_invalid(), "3 is not a power of two");
    assert!(cv.validate(&0u32).is_invalid(), "0 is not a power of two");
}

// ============================================================================
// Test 8 – NumericValidator<f32> with range method
// ============================================================================
/// `NumericValidator::range` on `f32` constrains both bounds simultaneously.
#[test]
fn test_numeric_validator_f32_range_method() {
    let nv = NumericValidator::<f32>::new().range(-1.0f32, 1.0f32);

    assert!(nv.validate(&-1.0f32).is_ok(), "-1.0 at lower boundary");
    assert!(nv.validate(&1.0f32).is_ok(), "1.0 at upper boundary");
    assert!(nv.validate(&0.0f32).is_ok(), "0.0 within range");
    assert!(nv.validate(&-1.001f32).is_err(), "-1.001 below lower");
    assert!(nv.validate(&1.001f32).is_err(), "1.001 above upper");
}

// ============================================================================
// Test 9 – CollectionValidator::validate_len at extreme values
// ============================================================================
/// `validate_len` handles 0, `usize::MAX`, and values at exact boundaries.
#[test]
fn test_collection_validator_validate_len_extremes() {
    let cv = CollectionValidator::new().min_len(1).max_len(100);

    assert!(cv.validate_len(0).is_err(), "length 0 must fail min_len(1)");
    assert!(cv.validate_len(1).is_ok(), "length 1 must pass");
    assert!(cv.validate_len(100).is_ok(), "length 100 must pass");
    assert!(
        cv.validate_len(101).is_err(),
        "length 101 must fail max_len(100)"
    );
    // non_empty is not set, so 0 only fails min_len, not the separate non_empty check
    let cv_ne = CollectionValidator::new().non_empty();
    assert!(
        cv_ne.validate_len(0).is_err(),
        "non_empty must reject length 0"
    );
    assert!(
        cv_ne.validate_len(1).is_ok(),
        "non_empty must accept length 1"
    );
    // usize::MAX only reachable check: ensure it passes when no max is set
    // (We can't actually create a slice of that size, but validate_len accepts the usize directly)
    let unbounded = CollectionValidator::new().min_len(1);
    assert!(
        unbounded.validate_len(usize::MAX).is_ok(),
        "usize::MAX must pass when only min_len(1) is set"
    );
}

// ============================================================================
// Test 10 – StringValidator::default() identical to StringValidator::new()
// ============================================================================
/// Both construction paths produce a validator that accepts any string.
#[cfg(feature = "alloc")]
#[test]
fn test_string_validator_default_equals_new() {
    let sv_default = StringValidator::default();
    let sv_new = StringValidator::new();

    // Neither has any constraints; both accept everything.
    for s in &["", "hello", "日本語", "🦀", "x".repeat(1000).as_str()] {
        assert!(
            sv_default.validate(s).is_ok(),
            "default StringValidator must accept '{}'",
            s
        );
        assert!(
            sv_new.validate(s).is_ok(),
            "new StringValidator must accept '{}'",
            s
        );
    }
}

// ============================================================================
// Test 11 – NumericValidator::default() accepts all values
// ============================================================================
/// A `NumericValidator::default()` with no bounds accepts every value.
#[test]
fn test_numeric_validator_default_accepts_all() {
    let nv: NumericValidator<i32> = NumericValidator::default();

    assert!(nv.validate(&i32::MIN).is_ok(), "i32::MIN must pass");
    assert!(nv.validate(&0).is_ok(), "0 must pass");
    assert!(nv.validate(&i32::MAX).is_ok(), "i32::MAX must pass");
}

// ============================================================================
// Test 12 – CollectionValidator::default() accepts all lengths
// ============================================================================
/// `CollectionValidator::default()` applies no constraints whatsoever.
#[test]
fn test_collection_validator_default_accepts_all() {
    let cv = CollectionValidator::default();

    assert!(cv.validate_len(0).is_ok(), "length 0 must pass");
    assert!(cv.validate_len(1_000_000).is_ok(), "large length must pass");

    let empty: [u8; 0] = [];
    let large: [u8; 255] = [0u8; 255];
    assert!(
        cv.validate(&empty).is_ok(),
        "empty slice must pass default CollectionValidator"
    );
    assert!(
        cv.validate(&large).is_ok(),
        "255-byte slice must pass default CollectionValidator"
    );
}

// ============================================================================
// Test 13 – ValidationConfig::with_checksum(false) resets flag
// ============================================================================
/// Calling `with_checksum(true)` then `with_checksum(false)` yields false.
#[test]
fn test_validation_config_reset_checksum() {
    use oxicode::validation::ValidationConfig;

    let config = ValidationConfig::new()
        .with_checksum(true)
        .with_checksum(false); // reset

    assert!(
        !config.verify_checksum,
        "verify_checksum must be false after reset"
    );

    let config2 = ValidationConfig::new().with_checksum(true);
    assert!(config2.verify_checksum, "verify_checksum must be true");
}

// ============================================================================
// Test 14 – Encode → validate → decode f32 roundtrip
// ============================================================================
/// An `f32` value is encoded, the byte buffer passes the length validator,
/// and the decoded value matches the original.
#[cfg(feature = "alloc")]
#[test]
fn test_encode_validate_decode_f32_roundtrip() {
    let byte_validator: Validator<Vec<u8>> = Validator::new()
        .constraint("buf", Constraints::min_len(1))
        .constraint("buf", Constraints::max_len(32));

    let original: f32 = std::f32::consts::PI;
    let encoded = oxicode::encode_to_vec(&original).expect("encode_to_vec f32 must succeed");

    assert!(
        byte_validator.validate(&encoded).is_ok(),
        "encoded f32 byte length {} must be in [1, 32]",
        encoded.len()
    );

    let (decoded, consumed): (f32, _) =
        oxicode::decode_from_slice(&encoded).expect("decode_from_slice f32 must succeed");

    assert_eq!(
        decoded.to_bits(),
        original.to_bits(),
        "decoded f32 bits must match original"
    );
    assert_eq!(
        consumed,
        encoded.len(),
        "all bytes must be consumed for f32"
    );
}

// ============================================================================
// Test 15 – Validator<u8> at absolute boundaries (0 and 255)
// ============================================================================
/// A `Validator<u8>` with range [0, 255] accepts every possible u8 value.
#[cfg(feature = "alloc")]
#[test]
fn test_validator_u8_full_range() {
    let validator: Validator<u8> =
        Validator::new().constraint("byte", Constraints::range(Some(0u8), Some(255u8)));

    // All u8 values must pass.
    for v in [0u8, 1, 127, 128, 254, 255] {
        assert!(
            validator.validate(&v).is_ok(),
            "u8 value {} must pass range [0, 255]",
            v
        );
    }

    // Narrow the range: values outside [10, 200] must fail.
    let narrow: Validator<u8> =
        Validator::new().constraint("byte", Constraints::range(Some(10u8), Some(200u8)));

    assert!(
        narrow.validate(&0u8).is_err(),
        "0 must fail range [10, 200]"
    );
    assert!(
        narrow.validate(&9u8).is_err(),
        "9 must fail range [10, 200]"
    );
    assert!(
        narrow.validate(&10u8).is_ok(),
        "10 must pass range [10, 200]"
    );
    assert!(
        narrow.validate(&200u8).is_ok(),
        "200 must pass range [10, 200]"
    );
    assert!(
        narrow.validate(&201u8).is_err(),
        "201 must fail range [10, 200]"
    );
    assert!(
        narrow.validate(&255u8).is_err(),
        "255 must fail range [10, 200]"
    );
}

// ============================================================================
// Test 16 – validate_or_default_with on Vec<u8>
// ============================================================================
/// `validate_or_default_with` on `Vec<u8>` returns the value when valid and
/// calls the closure lazily only on failure.
#[cfg(feature = "alloc")]
#[test]
fn test_validate_or_default_with_vec_u8() {
    let validator: Validator<Vec<u8>> = Validator::new()
        .constraint("data", Constraints::non_empty())
        .constraint("data", Constraints::min_len(2))
        .constraint("data", Constraints::max_len(8));

    let valid: Vec<u8> = vec![1, 2, 3, 4];
    let mut closure_called = false;

    let result = validator.validate_or_default_with(&valid, || {
        closure_called = true;
        vec![0u8]
    });
    assert_eq!(result, valid, "valid Vec<u8> must be returned unchanged");
    assert!(
        !closure_called,
        "closure must not be called for valid input"
    );

    // Single byte: fails min_len(2).
    let too_short: Vec<u8> = vec![42u8];
    closure_called = false;
    let fallback = validator.validate_or_default_with(&too_short, || {
        closure_called = true;
        vec![0u8; 4]
    });
    assert_eq!(
        fallback,
        vec![0u8; 4],
        "fallback must be returned for invalid input"
    );
    assert!(closure_called, "closure must be called for invalid input");
}

// ============================================================================
// Test 17 – FieldValidation with NonEmpty constraint
// ============================================================================
/// `FieldValidation::new` wraps `NonEmpty` and validates via `Ok`/`Err`.
#[cfg(feature = "alloc")]
#[test]
fn test_field_validation_with_non_empty_constraint() {
    let fv = FieldValidation::<String>::new("tag", Constraints::non_empty());

    assert_eq!(fv.field, "tag", "FieldValidation must store the field name");
    assert!(
        fv.validate(&"rust".to_string()).is_ok(),
        "non-empty value must pass"
    );
    assert!(
        fv.validate(&String::new()).is_err(),
        "empty String must fail"
    );

    let err = fv
        .validate(&String::new())
        .expect_err("empty String must produce an error");
    assert_eq!(err.field, "tag");
    assert!(!err.message.is_empty(), "error message must not be empty");
}

// ============================================================================
// Test 18 – StringValidator with only max_len set
// ============================================================================
/// A `StringValidator` with only `max_len` allows empty strings and rejects
/// strings exceeding the limit.
#[cfg(feature = "alloc")]
#[test]
fn test_string_validator_max_len_only() {
    let sv = StringValidator::new().max_len(10);

    assert!(
        sv.validate("").is_ok(),
        "empty string passes when only max_len is set"
    );
    assert!(
        sv.validate("hello").is_ok(),
        "5-char string passes max_len(10)"
    );
    assert!(
        sv.validate("1234567890").is_ok(),
        "10-char string passes max_len(10)"
    );
    assert!(
        sv.validate("12345678901").is_err(),
        "11-char string fails max_len(10)"
    );
    assert!(
        sv.validate("日本語").is_ok(),
        "Unicode passes when no ascii_only is set"
    );
}

// ============================================================================
// Test 19 – StringValidator with only min_len set
// ============================================================================
/// A `StringValidator` with only `min_len` accepts anything of sufficient length.
#[cfg(feature = "alloc")]
#[test]
fn test_string_validator_min_len_only() {
    let sv = StringValidator::new().min_len(3);

    assert!(
        sv.validate("abc").is_ok(),
        "3-char string passes min_len(3)"
    );
    assert!(
        sv.validate("longer string here").is_ok(),
        "long string passes min_len(3)"
    );
    assert!(sv.validate("ab").is_err(), "2-char string fails min_len(3)");
    assert!(sv.validate("").is_err(), "empty string fails min_len(3)");
    // Non-ASCII is fine because ascii_only is not set
    assert!(
        sv.validate("こんにちは").is_ok(),
        "Japanese passes when ascii_only not set"
    );
}

// ============================================================================
// Test 20 – Range::from_bounds with open-ended (unbounded) std range
// ============================================================================
/// `Range::from_bounds` called with `..` (unbounded) yields a constraint that
/// accepts all values (both min and max are None).
#[test]
fn test_range_from_bounds_unbounded() {
    use oxicode::validation::constraints::Range;

    // `..` is RangeFull which is Unbounded on both sides.
    let unbounded: Range<i32> = Range::from_bounds(&..);

    // With both min and max as None, every value must pass.
    assert!(
        unbounded.validate(&i32::MIN).is_valid(),
        "i32::MIN must pass unbounded range"
    );
    assert!(
        unbounded.validate(&0).is_valid(),
        "0 must pass unbounded range"
    );
    assert!(
        unbounded.validate(&i32::MAX).is_valid(),
        "i32::MAX must pass unbounded range"
    );

    // Also verify with a half-open included range (x..=y becomes Some(x)..Some(y)).
    let half_open: Range<i32> = Range::from_bounds(&(5i32..=20i32));
    assert!(half_open.validate(&5).is_valid(), "start bound must pass");
    assert!(half_open.validate(&20).is_valid(), "end bound must pass");
    assert!(half_open.validate(&4).is_invalid(), "below start must fail");
    // Note: Excluded end bounds map to None per implementation, so above 20 would pass
    // for an exclusive range. We tested inclusive so 20 passes and 21 is constrained.
    assert!(
        half_open.validate(&21).is_invalid(),
        "above end bound must fail"
    );
}

// ============================================================================
// Test 21 – Single Validator<i32> with two different field names
// ============================================================================
/// Two constraints with *different* field names in a single validator are both
/// applied; errors identify which field failed.
#[cfg(feature = "alloc")]
#[test]
fn test_validator_two_different_field_names() {
    // "lower_bound" constraint: value must be >= 0.
    // "upper_bound" constraint: value must be <= 100.
    let config = ValidationConfig::new().with_fail_fast(false);
    let mut validator: Validator<i32> = Validator::with_config(config);
    validator.add_constraint("lower_bound", Constraints::range(Some(0i32), None::<i32>));
    validator.add_constraint("upper_bound", Constraints::range(None::<i32>, Some(100i32)));

    // Valid: within both bounds.
    assert!(
        validator.validate(&50).is_ok(),
        "50 passes both constraints"
    );
    assert!(
        validator.validate(&0).is_ok(),
        "0 passes lower_bound constraint"
    );
    assert!(
        validator.validate(&100).is_ok(),
        "100 passes upper_bound constraint"
    );

    // Fails lower_bound only (negative).
    let errors_neg = validator
        .validate(&-1)
        .expect_err("-1 must fail lower_bound");
    assert_eq!(errors_neg.len(), 1, "only lower_bound must fail for -1");
    assert_eq!(
        errors_neg[0].field, "lower_bound",
        "error field must be 'lower_bound'"
    );

    // Fails upper_bound only (above 100).
    let errors_pos = validator
        .validate(&101)
        .expect_err("101 must fail upper_bound");
    assert_eq!(errors_pos.len(), 1, "only upper_bound must fail for 101");
    assert_eq!(
        errors_pos[0].field, "upper_bound",
        "error field must be 'upper_bound'"
    );
}

// ============================================================================
// Test 22 – CollectionValidator three constraints applied in separate instances
// ============================================================================
/// Building three separate `CollectionValidator` instances (each with one
/// constraint) and combining them manually is equivalent to a single instance
/// with all three constraints.
#[test]
fn test_collection_validator_three_separate_instances() {
    // Combined instance.
    let combined = CollectionValidator::new().min_len(2).max_len(6).non_empty();

    // Three separate instances.
    let only_min = CollectionValidator::new().min_len(2);
    let only_max = CollectionValidator::new().max_len(6);
    let only_ne = CollectionValidator::new().non_empty();

    let samples: &[&[i32]] = &[
        &[1, 2],             // len 2 – passes all
        &[1, 2, 3, 4, 5, 6], // len 6 – passes all
        &[],                 // len 0 – fails all three
        &[99],               // len 1 – fails min_len(2) and non_empty(with non_empty=true)
        &[0; 7],             // len 7 – fails max_len(6)
    ];

    for slice in samples {
        let combined_result = combined.validate(slice);
        // Each separate instance must agree with combined on the specific constraint it checks.
        let min_result = only_min.validate(slice);
        let max_result = only_max.validate(slice);
        let ne_result = only_ne.validate(slice);

        // If combined passes, all three individual constraints must also pass.
        if combined_result.is_ok() {
            assert!(
                min_result.is_ok(),
                "min_len must pass for slice of len {}",
                slice.len()
            );
            assert!(
                max_result.is_ok(),
                "max_len must pass for slice of len {}",
                slice.len()
            );
            assert!(
                ne_result.is_ok(),
                "non_empty must pass for slice of len {}",
                slice.len()
            );
        }
        // If combined fails, at least one individual constraint must also fail.
        if combined_result.is_err() {
            let any_fails = min_result.is_err() || max_result.is_err() || ne_result.is_err();
            assert!(
                any_fails,
                "at least one individual constraint must fail for slice of len {}",
                slice.len()
            );
        }
    }

    // Explicit spot-checks.
    assert!(
        combined.validate(&[1i32, 2i32]).is_ok(),
        "[1,2] passes combined"
    );
    assert!(
        combined.validate::<[i32; 0]>(&[]).is_err(),
        "[] fails combined"
    );
    assert!(
        combined.validate(&[0i32; 7]).is_err(),
        "[0;7] fails combined max_len(6)"
    );
    assert!(
        combined.validate(&[1i32]).is_err(),
        "[1] fails combined min_len(2)"
    );
}
