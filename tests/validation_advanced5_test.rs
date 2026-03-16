//! Advanced validation tests — set 5.
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
// 1. i64 range validator — values within range pass
// ---------------------------------------------------------------------------
#[test]
fn test_i64_range_constraint_within_bounds_passes() {
    let v: Validator<i64> =
        Validator::new().constraint("timestamp", Constraints::range(Some(0i64), Some(i64::MAX)));
    assert!(v.validate(&0i64).is_ok(), "zero must pass i64 [0, MAX]");
    assert!(
        v.validate(&1_000_000i64).is_ok(),
        "1_000_000 must pass i64 [0, MAX]"
    );
    assert!(
        v.validate(&i64::MAX).is_ok(),
        "i64::MAX must pass i64 [0, MAX]"
    );
}

// ---------------------------------------------------------------------------
// 2. i64 range validator — negative value below zero minimum fails
// ---------------------------------------------------------------------------
#[test]
fn test_i64_range_constraint_negative_value_fails() {
    let v: Validator<i64> =
        Validator::new().constraint("timestamp", Constraints::range(Some(0i64), Some(i64::MAX)));
    assert!(
        v.validate(&(-1i64)).is_err(),
        "-1 must fail i64 range [0, MAX]"
    );
    assert!(
        v.validate(&i64::MIN).is_err(),
        "i64::MIN must fail i64 range [0, MAX]"
    );
}

// ---------------------------------------------------------------------------
// 3. u8 range validator — valid byte values pass
// ---------------------------------------------------------------------------
#[test]
fn test_u8_range_constraint_valid_byte_passes() {
    let v: Validator<u8> =
        Validator::new().constraint("flags", Constraints::range(Some(0u8), Some(127u8)));
    assert!(v.validate(&0u8).is_ok(), "0 must pass u8 [0,127]");
    assert!(v.validate(&64u8).is_ok(), "64 must pass u8 [0,127]");
    assert!(v.validate(&127u8).is_ok(), "127 must pass u8 [0,127]");
}

// ---------------------------------------------------------------------------
// 4. u8 range validator — value above upper bound fails
// ---------------------------------------------------------------------------
#[test]
fn test_u8_range_constraint_above_upper_bound_fails() {
    let v: Validator<u8> =
        Validator::new().constraint("flags", Constraints::range(Some(0u8), Some(127u8)));
    assert!(v.validate(&128u8).is_err(), "128 must fail u8 [0,127]");
    assert!(v.validate(&255u8).is_err(), "255 must fail u8 [0,127]");
}

// ---------------------------------------------------------------------------
// 5. String ascii_only constraint — pure ASCII string passes
// ---------------------------------------------------------------------------
#[test]
fn test_string_ascii_only_constraint_pure_ascii_passes() {
    let v: Validator<String> =
        Validator::new().constraint("description", Constraints::ascii_only());
    assert!(v.validate(&"Hello, World!".to_string()).is_ok());
    assert!(v.validate(&"0123456789".to_string()).is_ok());
    assert!(v.validate(&"abcABC !@#".to_string()).is_ok());
}

// ---------------------------------------------------------------------------
// 6. String ascii_only constraint — string with non-ASCII chars fails
// ---------------------------------------------------------------------------
#[test]
fn test_string_ascii_only_constraint_non_ascii_fails() {
    let v: Validator<String> =
        Validator::new().constraint("description", Constraints::ascii_only());
    assert!(
        v.validate(&"caf\u{00E9}".to_string()).is_err(),
        "é is non-ASCII and must fail"
    );
    assert!(
        v.validate(&"日本語".to_string()).is_err(),
        "Japanese chars must fail ascii_only"
    );
    assert!(
        v.validate(&"\u{1F600}".to_string()).is_err(),
        "emoji must fail ascii_only"
    );
}

// ---------------------------------------------------------------------------
// 7. non_empty constraint — non-empty Vec passes
// ---------------------------------------------------------------------------
#[test]
fn test_non_empty_vec_constraint_passes() {
    let v: Validator<Vec<u8>> = Validator::new().constraint("data", Constraints::non_empty());
    assert!(
        v.validate(&vec![0u8]).is_ok(),
        "single-element vec must pass non_empty"
    );
    assert!(
        v.validate(&vec![1u8, 2u8, 3u8]).is_ok(),
        "three-element vec must pass non_empty"
    );
}

// ---------------------------------------------------------------------------
// 8. non_empty constraint — empty Vec fails
// ---------------------------------------------------------------------------
#[test]
fn test_non_empty_vec_constraint_empty_fails() {
    let v: Validator<Vec<u8>> = Validator::new().constraint("data", Constraints::non_empty());
    let empty: Vec<u8> = vec![];
    assert!(
        v.validate(&empty).is_err(),
        "empty vec must fail non_empty constraint"
    );
}

// ---------------------------------------------------------------------------
// 9. non_empty constraint — non-empty String passes
// ---------------------------------------------------------------------------
#[test]
fn test_non_empty_string_constraint_passes() {
    let v: Validator<String> = Validator::new().constraint("label", Constraints::non_empty());
    assert!(
        v.validate(&" ".to_string()).is_ok(),
        "whitespace string must pass non_empty"
    );
    assert!(
        v.validate(&"text".to_string()).is_ok(),
        "regular string must pass non_empty"
    );
}

// ---------------------------------------------------------------------------
// 10. non_empty constraint — empty String fails
// ---------------------------------------------------------------------------
#[test]
fn test_non_empty_string_constraint_empty_fails() {
    let v: Validator<String> = Validator::new().constraint("label", Constraints::non_empty());
    assert!(
        v.validate(&String::new()).is_err(),
        "empty string must fail non_empty constraint"
    );
}

// ---------------------------------------------------------------------------
// 11. Vec min_len and max_len combined — boundary values
// ---------------------------------------------------------------------------
#[test]
fn test_vec_min_max_len_boundary_values() {
    let v: Validator<Vec<i32>> = Validator::new()
        .constraint("items", Constraints::min_len(2))
        .constraint("items", Constraints::max_len(5));

    let at_min: Vec<i32> = vec![1, 2];
    let at_max: Vec<i32> = vec![1, 2, 3, 4, 5];
    let below_min: Vec<i32> = vec![1];
    let above_max: Vec<i32> = vec![1, 2, 3, 4, 5, 6];

    assert!(
        v.validate(&at_min).is_ok(),
        "vec of len 2 must pass min_len(2)"
    );
    assert!(
        v.validate(&at_max).is_ok(),
        "vec of len 5 must pass max_len(5)"
    );
    assert!(
        v.validate(&below_min).is_err(),
        "vec of len 1 must fail min_len(2)"
    );
    assert!(
        v.validate(&above_max).is_err(),
        "vec of len 6 must fail max_len(5)"
    );
}

// ---------------------------------------------------------------------------
// 12. NumericValidator min and max chaining — valid range
// ---------------------------------------------------------------------------
#[test]
fn test_numeric_validator_min_max_chain_valid_range() {
    let v = NumericValidator::<i32>::new().min(-50).max(50);
    assert!(v.validate(&(-50)).is_ok(), "-50 must pass min(-50)");
    assert!(v.validate(&0).is_ok(), "0 must pass range [-50, 50]");
    assert!(v.validate(&50).is_ok(), "50 must pass max(50)");
}

// ---------------------------------------------------------------------------
// 13. NumericValidator — values outside min/max chain fail
// ---------------------------------------------------------------------------
#[test]
fn test_numeric_validator_min_max_chain_out_of_range_fails() {
    let v = NumericValidator::<i32>::new().min(-50).max(50);
    assert!(v.validate(&(-51)).is_err(), "-51 must fail min(-50)");
    assert!(v.validate(&51).is_err(), "51 must fail max(50)");
}

// ---------------------------------------------------------------------------
// 14. Custom validator closure — numeric predicate on i64
// ---------------------------------------------------------------------------
#[test]
fn test_custom_validator_closure_numeric_predicate() {
    let v: Validator<i64> = Validator::new().constraint(
        "even_number",
        Constraints::custom(|n: &i64| n % 2 == 0, "value must be even", "even-check"),
    );

    assert!(v.validate(&0i64).is_ok(), "0 is even, must pass");
    assert!(v.validate(&2i64).is_ok(), "2 is even, must pass");
    assert!(v.validate(&-4i64).is_ok(), "-4 is even, must pass");
    assert!(v.validate(&1i64).is_err(), "1 is odd, must fail");
    assert!(v.validate(&(-7i64)).is_err(), "-7 is odd, must fail");
}

// ---------------------------------------------------------------------------
// 15. Custom validator closure — Vec predicate (all elements positive)
// ---------------------------------------------------------------------------
#[test]
fn test_custom_validator_closure_vec_all_positive() {
    let v: Validator<Vec<i32>> = Validator::new().constraint(
        "positive_list",
        Constraints::custom(
            |list: &Vec<i32>| list.iter().all(|&x| x > 0),
            "all elements must be positive",
            "all-positive",
        ),
    );

    assert!(
        v.validate(&vec![1, 2, 3]).is_ok(),
        "all-positive vec must pass"
    );
    assert!(
        v.validate(&vec![0, 1, 2]).is_err(),
        "vec containing 0 must fail"
    );
    assert!(
        v.validate(&vec![1, -1, 2]).is_err(),
        "vec containing negative must fail"
    );
}

// ---------------------------------------------------------------------------
// 16. ValidationError fields and Display formatting
// ---------------------------------------------------------------------------
#[test]
fn test_validation_error_fields_and_display_formatting() {
    let err = ValidationError::new("connection_timeout_ms", "must be between 100 and 30000");
    assert_eq!(err.field, "connection_timeout_ms");
    assert_eq!(err.message, "must be between 100 and 30000");
    let display = format!("{}", err);
    assert!(
        display.contains("connection_timeout_ms"),
        "Display must contain field name; got: {}",
        display
    );
    assert!(
        display.contains("must be between"),
        "Display must contain part of message; got: {}",
        display
    );
}

// ---------------------------------------------------------------------------
// 17. fail_fast=true stops at first error
// ---------------------------------------------------------------------------
#[test]
fn test_fail_fast_true_stops_at_first_error() {
    let config = ValidationConfig::new().with_fail_fast(true);
    let mut v: Validator<String> = Validator::with_config(config);
    // "" (len 0) fails both max_len(0) — wait, use constraints that both fail on "abc":
    // min_len(10) fails because len=3 < 10
    // max_len(1)  fails because len=3 > 1
    // With fail_fast we expect exactly 1 error reported.
    v.add_constraint("f", Constraints::min_len(10));
    v.add_constraint("f", Constraints::max_len(1));

    let result = v.validate(&"abc".to_string());
    assert!(result.is_err(), "both constraints fail, must return Err");
    let errors = result.expect_err("must have validation errors");
    assert_eq!(
        errors.len(),
        1,
        "fail_fast=true must produce exactly 1 error, got {}",
        errors.len()
    );
}

// ---------------------------------------------------------------------------
// 18. fail_fast=false collects all errors
// ---------------------------------------------------------------------------
#[test]
fn test_fail_fast_false_collects_all_errors() {
    let config = ValidationConfig::new().with_fail_fast(false);
    let mut v: Validator<String> = Validator::with_config(config);
    v.add_constraint("g", Constraints::min_len(10));
    v.add_constraint("g", Constraints::max_len(1));

    let result = v.validate(&"abc".to_string());
    assert!(result.is_err(), "both constraints fail, must return Err");
    let errors = result.expect_err("must have all validation errors");
    assert!(
        errors.len() >= 2,
        "fail_fast=false must collect at least 2 errors, got {}",
        errors.len()
    );
}

// ---------------------------------------------------------------------------
// 19. Encode → decode → validate workflow with u16 field
// ---------------------------------------------------------------------------
#[derive(Debug, PartialEq, Encode, Decode)]
struct PortRecord {
    port: u16,
    service: String,
}

#[test]
fn test_encode_decode_validate_u16_port_record() {
    let record = PortRecord {
        port: 8443,
        service: "https-alt".to_string(),
    };

    let bytes = encode_to_vec(&record).expect("encode PortRecord must succeed");
    let (decoded, bytes_read): (PortRecord, usize) =
        decode_from_slice(&bytes).expect("decode PortRecord must succeed");

    assert!(bytes_read > 0, "must consume bytes");
    assert_eq!(decoded, record, "roundtrip must preserve PortRecord");

    let port_v: Validator<u16> =
        Validator::new().constraint("port", Constraints::range(Some(1u16), Some(65535u16)));
    let svc_v: Validator<String> = Validator::new()
        .constraint("service", Constraints::min_len(1))
        .constraint("service", Constraints::max_len(64))
        .constraint("service", Constraints::ascii_only());

    assert!(
        port_v.validate(&decoded.port).is_ok(),
        "port must pass validation"
    );
    assert!(
        svc_v.validate(&decoded.service).is_ok(),
        "service must pass validation"
    );
}

// ---------------------------------------------------------------------------
// 20. Encode → decode → validate workflow — invalid decoded data detected
// ---------------------------------------------------------------------------
#[derive(Debug, PartialEq, Encode, Decode)]
struct BoundedCounter {
    count: i32,
}

#[test]
fn test_encode_decode_invalid_value_detected_by_validator() {
    // Encode a value that deliberately violates the constraint we'll apply.
    let record = BoundedCounter { count: -99 };

    let bytes = encode_to_vec(&record).expect("encode BoundedCounter must succeed");
    let (decoded, _): (BoundedCounter, usize) =
        decode_from_slice(&bytes).expect("decode BoundedCounter must succeed");

    assert_eq!(decoded.count, -99, "decoded count must be -99");

    // Constraint: count must be in [0, 1000].
    let v: Validator<i32> =
        Validator::new().constraint("count", Constraints::range(Some(0i32), Some(1000i32)));

    assert!(
        v.validate(&decoded.count).is_err(),
        "decoded value -99 must fail [0, 1000] range constraint"
    );
}

// ---------------------------------------------------------------------------
// 21. Multiple independent validators — all valid fields succeed together
// ---------------------------------------------------------------------------
#[derive(Debug, PartialEq, Encode, Decode)]
struct MetricEntry {
    sensor_id: u32,
    reading: f32,
    unit: String,
}

#[test]
fn test_multiple_independent_validators_all_valid_fields_succeed() {
    let entry = MetricEntry {
        sensor_id: 42,
        reading: 98.6_f32,
        unit: "Fahrenheit".to_string(),
    };

    let bytes = encode_to_vec(&entry).expect("encode MetricEntry must succeed");
    let (decoded, _): (MetricEntry, usize) =
        decode_from_slice(&bytes).expect("decode MetricEntry must succeed");

    let id_v: Validator<u32> =
        Validator::new().constraint("sensor_id", Constraints::range(Some(1u32), Some(9999u32)));
    let unit_v: Validator<String> = Validator::new()
        .constraint("unit", Constraints::non_empty())
        .constraint("unit", Constraints::min_len(1))
        .constraint("unit", Constraints::max_len(32))
        .constraint("unit", Constraints::ascii_only());

    assert!(
        id_v.validate(&decoded.sensor_id).is_ok(),
        "sensor_id must pass"
    );
    assert!(unit_v.validate(&decoded.unit).is_ok(), "unit must pass");
    assert_eq!(decoded, entry, "roundtrip must preserve MetricEntry");
}

// ---------------------------------------------------------------------------
// 22. Nested path field names in errors are preserved correctly
// ---------------------------------------------------------------------------
#[test]
fn test_nested_path_field_names_in_errors_preserved() {
    let config = ValidationConfig::new().with_fail_fast(false);
    let mut v1: Validator<u32> = Validator::with_config(config.clone());
    v1.add_constraint(
        "outer.inner.count",
        Constraints::range(Some(1u32), Some(10u32)),
    );

    let v2: Validator<String> = Validator::new()
        .constraint("outer.inner.label", Constraints::min_len(3))
        .constraint("outer.inner.label", Constraints::ascii_only());

    // Valid values
    assert!(
        v1.validate(&5u32).is_ok(),
        "5 in [1,10] must pass outer.inner.count"
    );
    assert!(
        v2.validate(&"ok_value".to_string()).is_ok(),
        "ok_value must pass outer.inner.label"
    );

    // Invalid count — verify error carries the full nested path
    let count_result = v1.validate(&0u32);
    assert!(count_result.is_err(), "0 must fail outer.inner.count range");
    let count_errors = count_result.expect_err("must have count errors");
    assert!(
        !count_errors.is_empty(),
        "must have at least one count error"
    );
    assert!(
        count_errors[0].field.contains("outer.inner.count"),
        "error field must contain 'outer.inner.count'; got '{}'",
        count_errors[0].field
    );

    // Invalid label — too short
    let label_result = v2.validate(&"ab".to_string());
    assert!(label_result.is_err(), "two-char label must fail min_len(3)");
    let label_errors = label_result.expect_err("must have label errors");
    assert!(
        !label_errors.is_empty(),
        "must have at least one label error"
    );
    assert!(
        label_errors[0].field.contains("outer.inner.label"),
        "error field must contain 'outer.inner.label'; got '{}'",
        label_errors[0].field
    );
}
