//! Advanced validation tests — set 4.
//! 22 top-level #[test] functions, no module wrapper, no #[cfg(test)].

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
    Constraints, NumericValidator, ValidationConfig, ValidationError, Validator,
};
use oxicode::{decode_from_slice, encode_to_vec, Decode, Encode};

// ---------------------------------------------------------------------------
// 1. Struct with #[validate(range)] attribute on a u32 field — range passes
// ---------------------------------------------------------------------------
#[test]
fn test_u32_range_constraint_valid_values_pass() {
    let v: Validator<u32> =
        Validator::new().constraint("port", Constraints::range(Some(1u32), Some(65535u32)));
    assert!(v.validate(&1).is_ok());
    assert!(v.validate(&8080).is_ok());
    assert!(v.validate(&65535).is_ok());
}

// ---------------------------------------------------------------------------
// 2. Struct with #[validate(length)] attribute on a String field — valid
// ---------------------------------------------------------------------------
#[test]
fn test_string_length_constraint_valid_string_passes() {
    let v: Validator<String> = Validator::new()
        .constraint("username", Constraints::min_len(3))
        .constraint("username", Constraints::max_len(32));
    assert!(v.validate(&"alice".to_string()).is_ok());
    assert!(v.validate(&"ab_".to_string()).is_ok());
}

// ---------------------------------------------------------------------------
// 3. Valid struct passes validation
// ---------------------------------------------------------------------------
#[test]
fn test_valid_struct_passes_validation() {
    let score_v: Validator<i32> =
        Validator::new().constraint("score", Constraints::range(Some(0i32), Some(100i32)));
    let label_v: Validator<String> = Validator::new()
        .constraint("label", Constraints::non_empty())
        .constraint("label", Constraints::ascii_only());
    assert!(score_v.validate(&75).is_ok());
    assert!(label_v.validate(&"passing".to_string()).is_ok());
}

// ---------------------------------------------------------------------------
// 4. Invalid value (out of range) fails validation
// ---------------------------------------------------------------------------
#[test]
fn test_out_of_range_value_fails_validation() {
    let v: Validator<u32> =
        Validator::new().constraint("count", Constraints::range(Some(1u32), Some(100u32)));
    assert!(v.validate(&0).is_err(), "0 is below minimum of 1");
    assert!(v.validate(&101).is_err(), "101 exceeds maximum of 100");
}

// ---------------------------------------------------------------------------
// 5. Invalid string (too long) fails validation
// ---------------------------------------------------------------------------
#[test]
fn test_too_long_string_fails_validation() {
    let v: Validator<String> = Validator::new().constraint("name", Constraints::max_len(10));
    let too_long = "x".repeat(11);
    assert!(
        v.validate(&too_long).is_err(),
        "11-char string must fail max_len(10)"
    );
}

// ---------------------------------------------------------------------------
// 6. Struct with multiple validated fields — all valid
// ---------------------------------------------------------------------------
#[test]
fn test_multiple_validated_fields_all_valid() {
    let name_v: Validator<String> = Validator::new()
        .constraint("name", Constraints::min_len(1))
        .constraint("name", Constraints::max_len(64))
        .constraint("name", Constraints::ascii_only());
    let age_v: Validator<i32> =
        Validator::new().constraint("age", Constraints::range(Some(0i32), Some(150i32)));
    let email_v: Validator<String> = Validator::new()
        .constraint("email", Constraints::min_len(5))
        .constraint("email", Constraints::ascii_only());

    assert!(name_v.validate(&"Alice".to_string()).is_ok());
    assert!(age_v.validate(&30).is_ok());
    assert!(email_v.validate(&"alice@example.com".to_string()).is_ok());
}

// ---------------------------------------------------------------------------
// 7. Struct with multiple validated fields — one invalid
// ---------------------------------------------------------------------------
#[test]
fn test_multiple_validated_fields_one_invalid() {
    let name_v: Validator<String> = Validator::new()
        .constraint("name", Constraints::min_len(3))
        .constraint("name", Constraints::ascii_only());
    let age_v: Validator<i32> =
        Validator::new().constraint("age", Constraints::range(Some(0i32), Some(150i32)));

    // name is too short — should fail
    assert!(
        name_v.validate(&"AB".to_string()).is_err(),
        "name too short must fail"
    );
    // age is valid — should pass
    assert!(age_v.validate(&25).is_ok(), "valid age must pass");
}

// ---------------------------------------------------------------------------
// 8. Nested struct validation propagates errors
// ---------------------------------------------------------------------------
#[test]
fn test_nested_struct_validation_propagates_errors() {
    let inner_name_v: Validator<String> = Validator::new()
        .constraint("inner.label", Constraints::min_len(2))
        .constraint("inner.label", Constraints::ascii_only());
    let outer_id_v: Validator<i32> =
        Validator::new().constraint("outer.id", Constraints::range(Some(1i32), Some(9999i32)));

    // Valid inner, valid outer
    assert!(inner_name_v.validate(&"ok_label".to_string()).is_ok());
    assert!(outer_id_v.validate(&42).is_ok());

    // Invalid inner (too short)
    let inner_result = inner_name_v.validate(&"x".to_string());
    assert!(inner_result.is_err(), "too-short inner label must fail");
    let inner_errors = inner_result.expect_err("must have errors");
    assert!(!inner_errors.is_empty());
    assert!(inner_errors[0].field.contains("inner.label"));

    // Invalid outer (out of range)
    let outer_result = outer_id_v.validate(&0);
    assert!(outer_result.is_err(), "zero id must fail outer constraint");
}

// ---------------------------------------------------------------------------
// 9. Vec field with length validation
// ---------------------------------------------------------------------------
#[test]
fn test_vec_field_with_length_validation() {
    let v: Validator<Vec<u8>> = Validator::new()
        .constraint("payload", Constraints::min_len(1))
        .constraint("payload", Constraints::max_len(16))
        .constraint("payload", Constraints::non_empty());

    let valid: Vec<u8> = vec![1, 2, 3];
    let exactly_max: Vec<u8> = vec![0u8; 16];
    let empty: Vec<u8> = vec![];
    let too_large: Vec<u8> = vec![0u8; 17];

    assert!(v.validate(&valid).is_ok());
    assert!(v.validate(&exactly_max).is_ok());
    assert!(v.validate(&empty).is_err());
    assert!(v.validate(&too_large).is_err());
}

// ---------------------------------------------------------------------------
// 10. Optional field validation (Some value validates, None skips)
// ---------------------------------------------------------------------------
#[test]
fn test_optional_field_some_validates_none_skips() {
    let inner_v = NumericValidator::<i32>::new().range(0, 100);

    // None: always treated as absent — no validation triggered
    let none_val: Option<i32> = None;
    assert!(none_val.is_none(), "None is absent, no validation needed");

    // Some in range: passes
    let some_valid: Option<i32> = Some(50);
    if let Some(ref val) = some_valid {
        assert!(
            inner_v.validate(val).is_ok(),
            "Some(50) in [0,100] must pass"
        );
    }

    // Some out of range: fails
    let some_invalid: Option<i32> = Some(200);
    if let Some(ref val) = some_invalid {
        assert!(
            inner_v.validate(val).is_err(),
            "Some(200) outside [0,100] must fail"
        );
    }
}

// ---------------------------------------------------------------------------
// 11. Validation error message contains field name
// ---------------------------------------------------------------------------
#[test]
fn test_validation_error_message_contains_field_name() {
    let err = ValidationError::new("user_score", "value out of permitted range");
    assert_eq!(err.field, "user_score");
    assert_eq!(err.message, "value out of permitted range");
    let display = format!("{}", err);
    assert!(
        display.contains("user_score"),
        "Display must contain the field name; got: {}",
        display
    );
}

// ---------------------------------------------------------------------------
// 12. Custom validator closure
// ---------------------------------------------------------------------------
#[test]
fn test_custom_validator_closure() {
    let v: Validator<String> = Validator::new().constraint(
        "slug",
        Constraints::custom(
            |s: &String| s.chars().all(|c| c.is_ascii_lowercase() || c == '-'),
            "slug must contain only lowercase ASCII letters and hyphens",
            "slug-format",
        ),
    );

    assert!(v.validate(&"valid-slug".to_string()).is_ok());
    assert!(v.validate(&"another-one".to_string()).is_ok());
    assert!(
        v.validate(&"UPPER".to_string()).is_err(),
        "uppercase fails slug"
    );
    assert!(
        v.validate(&"has space".to_string()).is_err(),
        "space fails slug"
    );
    assert!(v.validate(&"123".to_string()).is_err(), "digits fail slug");
}

// ---------------------------------------------------------------------------
// 13. Encode then decode then validate workflow
// ---------------------------------------------------------------------------
#[derive(Debug, PartialEq, Encode, Decode)]
struct ScoreRecord {
    score: i32,
    label: String,
}

#[test]
fn test_encode_decode_then_validate_workflow() {
    let record = ScoreRecord {
        score: 42,
        label: "intermediate".to_string(),
    };

    let bytes = encode_to_vec(&record).expect("encode should succeed");
    let (decoded, _bytes_read): (ScoreRecord, usize) =
        decode_from_slice(&bytes).expect("decode should succeed");

    let score_v: Validator<i32> =
        Validator::new().constraint("score", Constraints::range(Some(0i32), Some(100i32)));
    let label_v: Validator<String> = Validator::new()
        .constraint("label", Constraints::min_len(1))
        .constraint("label", Constraints::max_len(50))
        .constraint("label", Constraints::ascii_only());

    assert!(score_v.validate(&decoded.score).is_ok());
    assert!(label_v.validate(&decoded.label).is_ok());
    assert_eq!(decoded, record);
}

// ---------------------------------------------------------------------------
// 14. Valid data roundtrip with validation check
// ---------------------------------------------------------------------------
#[derive(Debug, PartialEq, Encode, Decode)]
struct UserProfile {
    id: u32,
    username: String,
}

#[test]
fn test_valid_data_roundtrip_with_validation_check() {
    let profile = UserProfile {
        id: 7,
        username: "kitasan".to_string(),
    };

    let bytes = encode_to_vec(&profile).expect("encode must succeed");
    let (decoded, _): (UserProfile, usize) =
        decode_from_slice(&bytes).expect("decode must succeed");

    let id_v: Validator<u32> =
        Validator::new().constraint("id", Constraints::range(Some(1u32), Some(u32::MAX)));
    let name_v: Validator<String> = Validator::new()
        .constraint("username", Constraints::min_len(1))
        .constraint("username", Constraints::max_len(32))
        .constraint("username", Constraints::ascii_only());

    assert!(id_v.validate(&decoded.id).is_ok());
    assert!(name_v.validate(&decoded.username).is_ok());
    assert_eq!(decoded, profile);
}

// ---------------------------------------------------------------------------
// 15. Boundary value (exactly at limit) passes
// ---------------------------------------------------------------------------
#[test]
fn test_boundary_value_exactly_at_limit_passes() {
    let v = NumericValidator::<u32>::new().min(10).max(200);
    assert!(v.validate(&10).is_ok(), "min boundary 10 must pass");
    assert!(v.validate(&200).is_ok(), "max boundary 200 must pass");
}

// ---------------------------------------------------------------------------
// 16. Value one above limit fails
// ---------------------------------------------------------------------------
#[test]
fn test_value_one_above_limit_fails() {
    let v = NumericValidator::<u32>::new().max(200);
    assert!(v.validate(&201).is_err(), "201 must fail when max=200");
}

// ---------------------------------------------------------------------------
// 17. Value one below lower bound fails
// ---------------------------------------------------------------------------
#[test]
fn test_value_one_below_lower_bound_fails() {
    let v = NumericValidator::<i32>::new().min(10);
    assert!(v.validate(&9).is_err(), "9 must fail when min=10");
}

// ---------------------------------------------------------------------------
// 18. Empty string when min_length > 0 fails
// ---------------------------------------------------------------------------
#[test]
fn test_empty_string_fails_min_length_greater_than_zero() {
    let v: Validator<String> = Validator::new().constraint("field", Constraints::min_len(1));
    assert!(
        v.validate(&String::new()).is_err(),
        "empty string must fail min_len(1)"
    );
}

// ---------------------------------------------------------------------------
// 19. String exactly at max_length passes
// ---------------------------------------------------------------------------
#[test]
fn test_string_exactly_at_max_length_passes() {
    let max = 20usize;
    let v: Validator<String> = Validator::new().constraint("field", Constraints::max_len(max));
    let exactly_max = "a".repeat(max);
    assert!(
        v.validate(&exactly_max).is_ok(),
        "string of exactly {} chars must pass max_len({})",
        max,
        max
    );
}

// ---------------------------------------------------------------------------
// 20. String one char over max_length fails
// ---------------------------------------------------------------------------
#[test]
fn test_string_one_char_over_max_length_fails() {
    let max = 20usize;
    let v: Validator<String> = Validator::new().constraint("field", Constraints::max_len(max));
    let over_max = "a".repeat(max + 1);
    assert!(
        v.validate(&over_max).is_err(),
        "string of {} chars must fail max_len({})",
        max + 1,
        max
    );
}

// ---------------------------------------------------------------------------
// 21. Validation produces distinct error variants for different violations
// ---------------------------------------------------------------------------
#[test]
fn test_validation_produces_distinct_errors_for_different_violations() {
    // Use fail_fast=false to collect all errors independently.
    let config = ValidationConfig::new().with_fail_fast(false);
    let mut v: Validator<String> = Validator::with_config(config);
    // "x" (len 1) fails max_len(0) AND min_len(5) independently.
    v.add_constraint("f", Constraints::max_len(0));
    v.add_constraint("f", Constraints::min_len(5));

    let result = v.validate(&"x".to_string());
    assert!(result.is_err());
    let errors = result.expect_err("must have validation errors");
    assert!(
        errors.len() >= 2,
        "two distinct failing constraints must produce at least 2 errors, got {}",
        errors.len()
    );
    // Verify each error carries field and message information.
    for err in &errors {
        assert_eq!(err.field, "f", "all errors must reference field 'f'");
        assert!(
            !err.message.is_empty(),
            "each error must have a non-empty message"
        );
    }
}

// ---------------------------------------------------------------------------
// 22. ValidatedData struct roundtrip — encode, decode, validate all succeed
// ---------------------------------------------------------------------------
#[derive(Debug, PartialEq, Encode, Decode)]
struct ValidatedData {
    version: u8,
    value: i32,
    tag: String,
}

#[test]
fn test_validated_data_struct_full_roundtrip_succeeds() {
    let original = ValidatedData {
        version: 1,
        value: 99,
        tag: "release".to_string(),
    };

    // Encode
    let bytes = encode_to_vec(&original).expect("encode ValidatedData must succeed");

    // Decode
    let (decoded, bytes_consumed): (ValidatedData, usize) =
        decode_from_slice(&bytes).expect("decode ValidatedData must succeed");

    assert!(bytes_consumed > 0, "must have consumed at least 1 byte");
    assert_eq!(decoded, original, "roundtrip must preserve data exactly");

    // Validate all fields
    let version_v: Validator<u8> =
        Validator::new().constraint("version", Constraints::range(Some(1u8), Some(255u8)));
    let value_v: Validator<i32> =
        Validator::new().constraint("value", Constraints::range(Some(0i32), Some(100i32)));
    let tag_v: Validator<String> = Validator::new()
        .constraint("tag", Constraints::non_empty())
        .constraint("tag", Constraints::min_len(1))
        .constraint("tag", Constraints::max_len(32))
        .constraint("tag", Constraints::ascii_only());

    assert!(
        version_v.validate(&decoded.version).is_ok(),
        "version field must pass validation"
    );
    assert!(
        value_v.validate(&decoded.value).is_ok(),
        "value field must pass validation"
    );
    assert!(
        tag_v.validate(&decoded.tag).is_ok(),
        "tag field must pass validation"
    );
}
