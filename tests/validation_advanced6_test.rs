//! Advanced validation tests — set 6.
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
    CollectionValidator, Constraints, NumericValidator, ValidationConfig, ValidationError,
    Validator,
};
use oxicode::{decode_from_slice, encode_to_vec, Decode, Encode};

// ---------------------------------------------------------------------------
// Domain types used throughout this test file
// ---------------------------------------------------------------------------

#[derive(Debug, PartialEq, Encode, Decode)]
struct ProductRecord {
    sku: String,
    price: f64,
    stock: u32,
    name: String,
}

#[derive(Debug, PartialEq, Encode, Decode)]
struct OrderLine {
    product_id: u32,
    quantity: u32,
    discount_pct: u8,
}

// ---------------------------------------------------------------------------
// 1. u32 range validator passes for value in range
// ---------------------------------------------------------------------------
#[test]
fn test_u32_range_validator_passes_for_value_in_range() {
    let v: Validator<u32> =
        Validator::new().constraint("stock", Constraints::range(Some(1u32), Some(10_000u32)));
    assert!(v.validate(&1u32).is_ok(), "1 must pass u32 [1, 10000]");
    assert!(
        v.validate(&5_000u32).is_ok(),
        "5000 must pass u32 [1, 10000]"
    );
    assert!(
        v.validate(&10_000u32).is_ok(),
        "10000 must pass u32 [1, 10000]"
    );
}

// ---------------------------------------------------------------------------
// 2. u32 range validator fails for value below minimum
// ---------------------------------------------------------------------------
#[test]
fn test_u32_range_validator_fails_for_value_below_minimum() {
    let v: Validator<u32> =
        Validator::new().constraint("stock", Constraints::range(Some(1u32), Some(10_000u32)));
    assert!(
        v.validate(&0u32).is_err(),
        "0 must fail u32 range [1, 10000]"
    );
}

// ---------------------------------------------------------------------------
// 3. u32 range validator fails for value above maximum
// ---------------------------------------------------------------------------
#[test]
fn test_u32_range_validator_fails_for_value_above_maximum() {
    let v: Validator<u32> =
        Validator::new().constraint("stock", Constraints::range(Some(1u32), Some(10_000u32)));
    assert!(
        v.validate(&10_001u32).is_err(),
        "10001 must fail u32 range [1, 10000]"
    );
    assert!(
        v.validate(&u32::MAX).is_err(),
        "u32::MAX must fail u32 range [1, 10000]"
    );
}

// ---------------------------------------------------------------------------
// 4. String min_len validator passes
// ---------------------------------------------------------------------------
#[test]
fn test_string_min_len_validator_passes() {
    let v: Validator<String> = Validator::new().constraint("name", Constraints::min_len(3));
    assert!(
        v.validate(&"abc".to_string()).is_ok(),
        "len-3 string must pass min_len(3)"
    );
    assert!(
        v.validate(&"Product X Pro".to_string()).is_ok(),
        "long string must pass min_len(3)"
    );
}

// ---------------------------------------------------------------------------
// 5. String min_len validator fails
// ---------------------------------------------------------------------------
#[test]
fn test_string_min_len_validator_fails() {
    let v: Validator<String> = Validator::new().constraint("name", Constraints::min_len(3));
    assert!(
        v.validate(&"ab".to_string()).is_err(),
        "len-2 string must fail min_len(3)"
    );
    assert!(
        v.validate(&String::new()).is_err(),
        "empty string must fail min_len(3)"
    );
}

// ---------------------------------------------------------------------------
// 6. String max_len validator passes
// ---------------------------------------------------------------------------
#[test]
fn test_string_max_len_validator_passes() {
    let v: Validator<String> = Validator::new().constraint("sku", Constraints::max_len(12));
    assert!(
        v.validate(&"SKU-001".to_string()).is_ok(),
        "7-char sku must pass max_len(12)"
    );
    assert!(
        v.validate(&"ABCDEFGHIJKL".to_string()).is_ok(),
        "exactly 12-char must pass max_len(12)"
    );
}

// ---------------------------------------------------------------------------
// 7. String max_len validator fails
// ---------------------------------------------------------------------------
#[test]
fn test_string_max_len_validator_fails() {
    let v: Validator<String> = Validator::new().constraint("sku", Constraints::max_len(12));
    assert!(
        v.validate(&"ABCDEFGHIJKLM".to_string()).is_err(),
        "13-char sku must fail max_len(12)"
    );
    assert!(
        v.validate(&"X".repeat(100)).is_err(),
        "100-char string must fail max_len(12)"
    );
}

// ---------------------------------------------------------------------------
// 8. Vec<u32> max_len validator passes
// ---------------------------------------------------------------------------
#[test]
fn test_vec_u32_max_len_validator_passes() {
    let v: Validator<Vec<u32>> = Validator::new().constraint("order_ids", Constraints::max_len(5));
    let ids: Vec<u32> = vec![1, 2, 3];
    let at_max: Vec<u32> = vec![1, 2, 3, 4, 5];
    assert!(
        v.validate(&ids).is_ok(),
        "vec of len 3 must pass max_len(5)"
    );
    assert!(
        v.validate(&at_max).is_ok(),
        "vec of len 5 must pass max_len(5)"
    );
}

// ---------------------------------------------------------------------------
// 9. Vec<u32> max_len validator fails
// ---------------------------------------------------------------------------
#[test]
fn test_vec_u32_max_len_validator_fails() {
    let v: Validator<Vec<u32>> = Validator::new().constraint("order_ids", Constraints::max_len(5));
    let too_many: Vec<u32> = vec![1, 2, 3, 4, 5, 6];
    assert!(
        v.validate(&too_many).is_err(),
        "vec of len 6 must fail max_len(5)"
    );
}

// ---------------------------------------------------------------------------
// 10. Custom predicate: price must be positive (f64 > 0.0)
// ---------------------------------------------------------------------------
#[test]
fn test_custom_predicate_price_must_be_positive() {
    let v: Validator<f64> = Validator::new().constraint(
        "price",
        Constraints::custom(
            |p: &f64| *p > 0.0,
            "price must be positive",
            "positive-price",
        ),
    );
    assert!(
        v.validate(&0.01_f64).is_ok(),
        "0.01 must pass positive price"
    );
    assert!(
        v.validate(&99.99_f64).is_ok(),
        "99.99 must pass positive price"
    );
    assert!(
        v.validate(&0.0_f64).is_err(),
        "0.0 must fail positive price"
    );
    assert!(
        v.validate(&(-5.0_f64)).is_err(),
        "-5.0 must fail positive price"
    );
}

// ---------------------------------------------------------------------------
// 11. Custom predicate: sku must be alphanumeric
// ---------------------------------------------------------------------------
#[test]
fn test_custom_predicate_sku_must_be_alphanumeric() {
    let v: Validator<String> = Validator::new().constraint(
        "sku",
        Constraints::custom(
            |s: &String| s.chars().all(|c| c.is_alphanumeric() || c == '-'),
            "sku must contain only alphanumeric characters or hyphens",
            "alphanumeric-sku",
        ),
    );
    assert!(
        v.validate(&"SKU001".to_string()).is_ok(),
        "alphanumeric sku must pass"
    );
    assert!(
        v.validate(&"SKU-001".to_string()).is_ok(),
        "sku with hyphen must pass"
    );
    assert!(
        v.validate(&"SKU 001".to_string()).is_err(),
        "sku with space must fail"
    );
    assert!(
        v.validate(&"SKU@001".to_string()).is_err(),
        "sku with @ must fail"
    );
}

// ---------------------------------------------------------------------------
// 12. ValidationConfig fail_fast stops at first error
// ---------------------------------------------------------------------------
#[test]
fn test_validation_config_fail_fast_stops_at_first_error() {
    let config = ValidationConfig::new().with_fail_fast(true);
    let mut v: Validator<String> = Validator::with_config(config);
    // "X" (len 1) fails min_len(5) AND fails ascii_only? No, it is ascii.
    // Use min_len(5) and max_len(0) so that "X" fails both.
    v.add_constraint("name", Constraints::min_len(5));
    v.add_constraint("name", Constraints::max_len(0));

    let result = v.validate(&"X".to_string());
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
// 13. ValidationConfig collect-all gathers multiple errors
// ---------------------------------------------------------------------------
#[test]
fn test_validation_config_collect_all_gathers_multiple_errors() {
    let config = ValidationConfig::new().with_fail_fast(false);
    let mut v: Validator<String> = Validator::with_config(config);
    // "X" (len 1) fails min_len(5) and also fails max_len(0)
    v.add_constraint("name", Constraints::min_len(5));
    v.add_constraint("name", Constraints::max_len(0));

    let result = v.validate(&"X".to_string());
    assert!(result.is_err(), "both constraints fail, must return Err");
    let errors = result.expect_err("must have all validation errors");
    assert!(
        errors.len() >= 2,
        "fail_fast=false must collect at least 2 errors, got {}",
        errors.len()
    );
}

// ---------------------------------------------------------------------------
// 14. ValidationError has field_name set correctly
// ---------------------------------------------------------------------------
#[test]
fn test_validation_error_has_field_name_set_correctly() {
    let err = ValidationError::new("product.sku", "sku must not be empty");
    assert_eq!(
        err.field, "product.sku",
        "field must be 'product.sku', got '{}'",
        err.field
    );
}

// ---------------------------------------------------------------------------
// 15. ValidationError has error message
// ---------------------------------------------------------------------------
#[test]
fn test_validation_error_has_error_message() {
    let err = ValidationError::new("order.quantity", "quantity must be at least 1");
    assert_eq!(
        err.message, "quantity must be at least 1",
        "message must match, got '{}'",
        err.message
    );
    let display = format!("{}", err);
    assert!(
        display.contains("order.quantity"),
        "Display must contain field name; got: {}",
        display
    );
    assert!(
        display.contains("quantity must be at least 1"),
        "Display must contain message; got: {}",
        display
    );
}

// ---------------------------------------------------------------------------
// 16. ProductRecord encode → decode → validate roundtrip (valid)
// ---------------------------------------------------------------------------
#[test]
fn test_product_record_encode_decode_validate_roundtrip_valid() {
    let record = ProductRecord {
        sku: "PROD-001".to_string(),
        price: 29.99,
        stock: 150,
        name: "Widget Pro".to_string(),
    };

    let bytes = encode_to_vec(&record).expect("encode ProductRecord must succeed");
    let (decoded, bytes_read): (ProductRecord, usize) =
        decode_from_slice(&bytes).expect("decode ProductRecord must succeed");

    assert!(bytes_read > 0, "must consume at least one byte");
    assert_eq!(decoded, record, "roundtrip must preserve ProductRecord");

    let sku_v: Validator<String> = Validator::new()
        .constraint("sku", Constraints::min_len(1))
        .constraint("sku", Constraints::max_len(20))
        .constraint("sku", Constraints::ascii_only());
    let price_v: Validator<f64> = Validator::new().constraint(
        "price",
        Constraints::custom(
            |p: &f64| *p > 0.0,
            "price must be positive",
            "positive-price",
        ),
    );
    let stock_v: Validator<u32> =
        Validator::new().constraint("stock", Constraints::range(Some(0u32), Some(1_000_000u32)));
    let name_v: Validator<String> = Validator::new()
        .constraint("name", Constraints::non_empty())
        .constraint("name", Constraints::min_len(1))
        .constraint("name", Constraints::max_len(100));

    assert!(
        sku_v.validate(&decoded.sku).is_ok(),
        "decoded sku must pass validation"
    );
    assert!(
        price_v.validate(&decoded.price).is_ok(),
        "decoded price must pass validation"
    );
    assert!(
        stock_v.validate(&decoded.stock).is_ok(),
        "decoded stock must pass validation"
    );
    assert!(
        name_v.validate(&decoded.name).is_ok(),
        "decoded name must pass validation"
    );
}

// ---------------------------------------------------------------------------
// 17. OrderLine quantity range validation
// ---------------------------------------------------------------------------
#[test]
fn test_order_line_quantity_range_validation() {
    let v: Validator<u32> =
        Validator::new().constraint("quantity", Constraints::range(Some(1u32), Some(9_999u32)));

    assert!(v.validate(&1u32).is_ok(), "quantity 1 must pass [1, 9999]");
    assert!(
        v.validate(&500u32).is_ok(),
        "quantity 500 must pass [1, 9999]"
    );
    assert!(
        v.validate(&9_999u32).is_ok(),
        "quantity 9999 must pass [1, 9999]"
    );
    assert!(v.validate(&0u32).is_err(), "quantity 0 must fail [1, 9999]");
    assert!(
        v.validate(&10_000u32).is_err(),
        "quantity 10000 must fail [1, 9999]"
    );
}

// ---------------------------------------------------------------------------
// 18. OrderLine discount_pct must be 0–100
// ---------------------------------------------------------------------------
#[test]
fn test_order_line_discount_pct_must_be_0_to_100() {
    let v: Validator<u8> =
        Validator::new().constraint("discount_pct", Constraints::range(Some(0u8), Some(100u8)));

    assert!(v.validate(&0u8).is_ok(), "discount 0 must pass [0, 100]");
    assert!(v.validate(&50u8).is_ok(), "discount 50 must pass [0, 100]");
    assert!(
        v.validate(&100u8).is_ok(),
        "discount 100 must pass [0, 100]"
    );
    assert!(
        v.validate(&101u8).is_err(),
        "discount 101 must fail [0, 100]"
    );
    assert!(
        v.validate(&255u8).is_err(),
        "discount 255 must fail [0, 100]"
    );
}

// ---------------------------------------------------------------------------
// 19. Vec<OrderLine> each item validated via CollectionValidator
// ---------------------------------------------------------------------------
#[test]
fn test_vec_order_line_each_item_validated_via_collection_validator() {
    let lines: Vec<OrderLine> = vec![
        OrderLine {
            product_id: 1,
            quantity: 3,
            discount_pct: 10,
        },
        OrderLine {
            product_id: 2,
            quantity: 1,
            discount_pct: 0,
        },
        OrderLine {
            product_id: 3,
            quantity: 50,
            discount_pct: 25,
        },
    ];
    let coll_v = CollectionValidator::new().min_len(1).max_len(50);
    assert!(
        coll_v.validate(&lines).is_ok(),
        "3-element OrderLine vec must pass min_len(1)/max_len(50)"
    );

    let discount_v: Validator<u8> =
        Validator::new().constraint("discount_pct", Constraints::range(Some(0u8), Some(100u8)));
    let quantity_v: Validator<u32> =
        Validator::new().constraint("quantity", Constraints::range(Some(1u32), Some(9_999u32)));

    for line in &lines {
        assert!(
            discount_v.validate(&line.discount_pct).is_ok(),
            "discount_pct {} must pass [0, 100]",
            line.discount_pct
        );
        assert!(
            quantity_v.validate(&line.quantity).is_ok(),
            "quantity {} must pass [1, 9999]",
            line.quantity
        );
    }
}

// ---------------------------------------------------------------------------
// 20. String non_empty validator for name
// ---------------------------------------------------------------------------
#[test]
fn test_string_non_empty_validator_for_name() {
    let v: Validator<String> = Validator::new().constraint("name", Constraints::non_empty());

    assert!(
        v.validate(&"Widget".to_string()).is_ok(),
        "non-empty name must pass"
    );
    assert!(
        v.validate(&" ".to_string()).is_ok(),
        "whitespace name must pass non_empty"
    );
    assert!(
        v.validate(&String::new()).is_err(),
        "empty name must fail non_empty"
    );
}

// ---------------------------------------------------------------------------
// 21. NumericValidator chained min/max
// ---------------------------------------------------------------------------
#[test]
fn test_numeric_validator_chained_min_max() {
    let v = NumericValidator::<u32>::new().min(10u32).max(500u32);

    assert!(v.validate(&10u32).is_ok(), "10 must pass min(10)");
    assert!(v.validate(&250u32).is_ok(), "250 must pass [10, 500]");
    assert!(v.validate(&500u32).is_ok(), "500 must pass max(500)");
    assert!(v.validate(&9u32).is_err(), "9 must fail min(10)");
    assert!(v.validate(&501u32).is_err(), "501 must fail max(500)");
}

// ---------------------------------------------------------------------------
// 22. FieldValidator (FieldValidation) with nested path name
//     e.g. "order.discount_pct"
// ---------------------------------------------------------------------------
#[test]
fn test_field_validation_with_nested_path_name() {
    use oxicode::validation::FieldValidation;

    let fv: FieldValidation<u8> = FieldValidation::new(
        "order.discount_pct",
        Constraints::range(Some(0u8), Some(100u8)),
    );

    assert_eq!(
        fv.field, "order.discount_pct",
        "field path must be preserved"
    );

    // Valid discount
    assert!(
        fv.validate(&50u8).is_ok(),
        "50 must pass order.discount_pct [0, 100]"
    );

    // Invalid discount — verify error carries the nested path
    let result = fv.validate(&200u8);
    assert!(result.is_err(), "200 must fail order.discount_pct [0, 100]");
    let err = result.expect_err("must produce a ValidationError for 200");
    assert_eq!(
        err.field, "order.discount_pct",
        "error field must be 'order.discount_pct', got '{}'",
        err.field
    );
    assert!(
        !err.message.is_empty(),
        "error message must not be empty; got '{}'",
        err.message
    );
}
