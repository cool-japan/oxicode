//! Advanced validation tests — set 7.
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
struct CustomerRecord {
    customer_id: u32,
    name: String,
    email: String,
    age: u8,
    balance_cents: i64,
}

#[derive(Debug, PartialEq, Encode, Decode)]
enum SubscriptionTier {
    Free,
    Basic { monthly_usd: u32 },
    Premium { monthly_usd: u32, max_seats: u8 },
    Enterprise(String),
}

// ---------------------------------------------------------------------------
// 1. customer_id range validator passes for valid IDs
// ---------------------------------------------------------------------------
#[test]
fn test_customer_id_range_validator_passes_for_valid_ids() {
    let v: Validator<u32> = Validator::new().constraint(
        "customer_id",
        Constraints::range(Some(1u32), Some(999_999u32)),
    );
    assert!(
        v.validate(&1u32).is_ok(),
        "customer_id 1 must pass [1, 999999]"
    );
    assert!(
        v.validate(&500_000u32).is_ok(),
        "customer_id 500000 must pass [1, 999999]"
    );
    assert!(
        v.validate(&999_999u32).is_ok(),
        "customer_id 999999 must pass [1, 999999]"
    );
}

// ---------------------------------------------------------------------------
// 2. customer_id range validator fails for zero
// ---------------------------------------------------------------------------
#[test]
fn test_customer_id_range_validator_fails_for_zero() {
    let v: Validator<u32> = Validator::new().constraint(
        "customer_id",
        Constraints::range(Some(1u32), Some(999_999u32)),
    );
    assert!(
        v.validate(&0u32).is_err(),
        "customer_id 0 must fail [1, 999999]"
    );
}

// ---------------------------------------------------------------------------
// 3. customer_id range validator fails for value above maximum
// ---------------------------------------------------------------------------
#[test]
fn test_customer_id_range_validator_fails_for_value_above_maximum() {
    let v: Validator<u32> = Validator::new().constraint(
        "customer_id",
        Constraints::range(Some(1u32), Some(999_999u32)),
    );
    assert!(
        v.validate(&1_000_000u32).is_err(),
        "customer_id 1000000 must fail [1, 999999]"
    );
    assert!(
        v.validate(&u32::MAX).is_err(),
        "u32::MAX must fail [1, 999999]"
    );
}

// ---------------------------------------------------------------------------
// 4. name min_len validator passes
// ---------------------------------------------------------------------------
#[test]
fn test_name_min_len_validator_passes() {
    let v: Validator<String> = Validator::new().constraint("name", Constraints::min_len(2));
    assert!(
        v.validate(&"Jo".to_string()).is_ok(),
        "len-2 name must pass min_len(2)"
    );
    assert!(
        v.validate(&"Alice Smith".to_string()).is_ok(),
        "longer name must pass min_len(2)"
    );
}

// ---------------------------------------------------------------------------
// 5. name min_len validator fails for empty and single-char
// ---------------------------------------------------------------------------
#[test]
fn test_name_min_len_validator_fails_for_empty_and_single_char() {
    let v: Validator<String> = Validator::new().constraint("name", Constraints::min_len(2));
    assert!(
        v.validate(&String::new()).is_err(),
        "empty name must fail min_len(2)"
    );
    assert!(
        v.validate(&"A".to_string()).is_err(),
        "single-char name must fail min_len(2)"
    );
}

// ---------------------------------------------------------------------------
// 6. name max_len validator passes and fails
// ---------------------------------------------------------------------------
#[test]
fn test_name_max_len_validator_passes_and_fails() {
    let v: Validator<String> = Validator::new().constraint("name", Constraints::max_len(64));
    let exactly_64 = "A".repeat(64);
    assert!(
        v.validate(&exactly_64).is_ok(),
        "64-char name must pass max_len(64)"
    );
    let too_long = "A".repeat(65);
    assert!(
        v.validate(&too_long).is_err(),
        "65-char name must fail max_len(64)"
    );
}

// ---------------------------------------------------------------------------
// 7. email ascii_only validator passes for plain ASCII email
// ---------------------------------------------------------------------------
#[test]
fn test_email_ascii_only_validator_passes_for_plain_ascii_email() {
    let v: Validator<String> = Validator::new().constraint("email", Constraints::ascii_only());
    assert!(
        v.validate(&"user@example.com".to_string()).is_ok(),
        "plain ASCII email must pass ascii_only"
    );
    assert!(
        v.validate(&"admin+tag@corp.org".to_string()).is_ok(),
        "email with plus must pass ascii_only"
    );
}

// ---------------------------------------------------------------------------
// 8. email ascii_only validator fails for non-ASCII characters
// ---------------------------------------------------------------------------
#[test]
fn test_email_ascii_only_validator_fails_for_non_ascii_characters() {
    let v: Validator<String> = Validator::new().constraint("email", Constraints::ascii_only());
    assert!(
        v.validate(&"usér@example.com".to_string()).is_err(),
        "email with accented char must fail ascii_only"
    );
}

// ---------------------------------------------------------------------------
// 9. age range validator covers boundary values 0–120
// ---------------------------------------------------------------------------
#[test]
fn test_age_range_validator_covers_boundary_values_0_to_120() {
    let v: Validator<u8> =
        Validator::new().constraint("age", Constraints::range(Some(0u8), Some(120u8)));
    assert!(v.validate(&0u8).is_ok(), "age 0 must pass [0, 120]");
    assert!(v.validate(&120u8).is_ok(), "age 120 must pass [0, 120]");
    assert!(v.validate(&121u8).is_err(), "age 121 must fail [0, 120]");
    assert!(v.validate(&255u8).is_err(), "age 255 must fail [0, 120]");
}

// ---------------------------------------------------------------------------
// 10. balance_cents custom predicate allows zero and positive
// ---------------------------------------------------------------------------
#[test]
fn test_balance_cents_custom_predicate_allows_zero_and_positive() {
    let v: Validator<i64> = Validator::new().constraint(
        "balance_cents",
        Constraints::custom(
            |b: &i64| *b >= 0,
            "balance_cents must be non-negative",
            "non-negative-balance",
        ),
    );
    assert!(
        v.validate(&0i64).is_ok(),
        "balance 0 must pass non-negative"
    );
    assert!(
        v.validate(&1i64).is_ok(),
        "balance 1 must pass non-negative"
    );
    assert!(
        v.validate(&i64::MAX).is_ok(),
        "i64::MAX must pass non-negative"
    );
    assert!(
        v.validate(&(-1i64)).is_err(),
        "balance -1 must fail non-negative"
    );
}

// ---------------------------------------------------------------------------
// 11. balance_cents custom predicate rejects deeply negative values
// ---------------------------------------------------------------------------
#[test]
fn test_balance_cents_custom_predicate_rejects_deeply_negative_values() {
    let v: Validator<i64> = Validator::new().constraint(
        "balance_cents",
        Constraints::custom(
            |b: &i64| *b >= -10_000_00, // allow up to $10 000 overdraft
            "balance_cents overdraft limit exceeded",
            "overdraft-limit",
        ),
    );
    assert!(v.validate(&(-100i64)).is_ok(), "small overdraft must pass");
    assert!(
        v.validate(&(-10_000_00i64)).is_ok(),
        "max overdraft must pass"
    );
    assert!(
        v.validate(&(-10_000_01i64)).is_err(),
        "exceeds overdraft limit must fail"
    );
    assert!(
        v.validate(&i64::MIN).is_err(),
        "i64::MIN must fail overdraft limit"
    );
}

// ---------------------------------------------------------------------------
// 12. CustomerRecord encode → decode → validate roundtrip (valid)
// ---------------------------------------------------------------------------
#[test]
fn test_customer_record_encode_decode_validate_roundtrip_valid() {
    let record = CustomerRecord {
        customer_id: 42,
        name: "Jane Doe".to_string(),
        email: "jane.doe@example.com".to_string(),
        age: 30,
        balance_cents: 150_00,
    };

    let bytes = encode_to_vec(&record).expect("encode CustomerRecord must succeed");
    let (decoded, bytes_read): (CustomerRecord, usize) =
        decode_from_slice(&bytes).expect("decode CustomerRecord must succeed");

    assert!(bytes_read > 0, "must consume at least one byte");
    assert_eq!(decoded, record, "roundtrip must preserve CustomerRecord");

    let id_v: Validator<u32> = Validator::new().constraint(
        "customer_id",
        Constraints::range(Some(1u32), Some(999_999u32)),
    );
    let name_v: Validator<String> = Validator::new()
        .constraint("name", Constraints::min_len(2))
        .constraint("name", Constraints::max_len(64));
    let email_v: Validator<String> = Validator::new()
        .constraint("email", Constraints::ascii_only())
        .constraint("email", Constraints::min_len(5))
        .constraint("email", Constraints::max_len(254));
    let age_v: Validator<u8> =
        Validator::new().constraint("age", Constraints::range(Some(0u8), Some(120u8)));

    assert!(
        id_v.validate(&decoded.customer_id).is_ok(),
        "decoded customer_id must pass"
    );
    assert!(
        name_v.validate(&decoded.name).is_ok(),
        "decoded name must pass"
    );
    assert!(
        email_v.validate(&decoded.email).is_ok(),
        "decoded email must pass"
    );
    assert!(
        age_v.validate(&decoded.age).is_ok(),
        "decoded age must pass"
    );
}

// ---------------------------------------------------------------------------
// 13. SubscriptionTier::Free encodes and decodes correctly
// ---------------------------------------------------------------------------
#[test]
fn test_subscription_tier_free_encodes_and_decodes_correctly() {
    let tier = SubscriptionTier::Free;
    let bytes = encode_to_vec(&tier).expect("encode Free must succeed");
    let (decoded, bytes_read): (SubscriptionTier, usize) =
        decode_from_slice(&bytes).expect("decode Free must succeed");
    assert!(bytes_read > 0, "must consume at least one byte");
    assert_eq!(
        decoded,
        SubscriptionTier::Free,
        "decoded Free tier must match"
    );
}

// ---------------------------------------------------------------------------
// 14. SubscriptionTier::Basic roundtrip with monthly_usd validation
// ---------------------------------------------------------------------------
#[test]
fn test_subscription_tier_basic_roundtrip_with_monthly_usd_validation() {
    let tier = SubscriptionTier::Basic { monthly_usd: 9 };
    let bytes = encode_to_vec(&tier).expect("encode Basic must succeed");
    let (decoded, _): (SubscriptionTier, usize) =
        decode_from_slice(&bytes).expect("decode Basic must succeed");
    assert_eq!(decoded, tier, "roundtrip must preserve Basic tier");

    if let SubscriptionTier::Basic { monthly_usd } = decoded {
        let v: Validator<u32> = Validator::new()
            .constraint("monthly_usd", Constraints::range(Some(1u32), Some(999u32)));
        assert!(
            v.validate(&monthly_usd).is_ok(),
            "Basic monthly_usd 9 must pass [1, 999]"
        );
    } else {
        panic!("decoded tier must be Basic");
    }
}

// ---------------------------------------------------------------------------
// 15. SubscriptionTier::Premium roundtrip with max_seats validation
// ---------------------------------------------------------------------------
#[test]
fn test_subscription_tier_premium_roundtrip_with_max_seats_validation() {
    let tier = SubscriptionTier::Premium {
        monthly_usd: 49,
        max_seats: 25,
    };
    let bytes = encode_to_vec(&tier).expect("encode Premium must succeed");
    let (decoded, _): (SubscriptionTier, usize) =
        decode_from_slice(&bytes).expect("decode Premium must succeed");
    assert_eq!(decoded, tier, "roundtrip must preserve Premium tier");

    if let SubscriptionTier::Premium {
        monthly_usd,
        max_seats,
    } = decoded
    {
        let price_v: Validator<u32> = Validator::new().constraint(
            "monthly_usd",
            Constraints::range(Some(1u32), Some(9_999u32)),
        );
        let seats_v: Validator<u8> =
            Validator::new().constraint("max_seats", Constraints::range(Some(1u8), Some(200u8)));
        assert!(
            price_v.validate(&monthly_usd).is_ok(),
            "Premium monthly_usd must pass"
        );
        assert!(
            seats_v.validate(&max_seats).is_ok(),
            "Premium max_seats must pass"
        );
    } else {
        panic!("decoded tier must be Premium");
    }
}

// ---------------------------------------------------------------------------
// 16. SubscriptionTier::Enterprise roundtrip with contract_id validation
// ---------------------------------------------------------------------------
#[test]
fn test_subscription_tier_enterprise_roundtrip_with_contract_id_validation() {
    let tier = SubscriptionTier::Enterprise("ENTERPRISE-2026-001".to_string());
    let bytes = encode_to_vec(&tier).expect("encode Enterprise must succeed");
    let (decoded, _): (SubscriptionTier, usize) =
        decode_from_slice(&bytes).expect("decode Enterprise must succeed");
    assert_eq!(decoded, tier, "roundtrip must preserve Enterprise tier");

    if let SubscriptionTier::Enterprise(contract_id) = decoded {
        let v: Validator<String> = Validator::new()
            .constraint("contract_id", Constraints::min_len(5))
            .constraint("contract_id", Constraints::max_len(128))
            .constraint("contract_id", Constraints::ascii_only());
        assert!(
            v.validate(&contract_id).is_ok(),
            "Enterprise contract_id must pass"
        );
    } else {
        panic!("decoded tier must be Enterprise");
    }
}

// ---------------------------------------------------------------------------
// 17. ValidationConfig fail_fast stops at first error on CustomerRecord fields
// ---------------------------------------------------------------------------
#[test]
fn test_validation_config_fail_fast_stops_at_first_error_on_customer_record_fields() {
    let config = ValidationConfig::new().with_fail_fast(true);
    let mut v: Validator<String> = Validator::with_config(config);
    // "" fails both non_empty AND min_len(3)
    v.add_constraint("name", Constraints::non_empty());
    v.add_constraint("name", Constraints::min_len(3));

    let result = v.validate(&String::new());
    assert!(result.is_err(), "empty string must fail");
    let errors = result.expect_err("must produce errors");
    assert_eq!(
        errors.len(),
        1,
        "fail_fast=true must stop at first error, got {}",
        errors.len()
    );
}

// ---------------------------------------------------------------------------
// 18. ValidationConfig collect-all gathers multiple errors for email
// ---------------------------------------------------------------------------
#[test]
fn test_validation_config_collect_all_gathers_multiple_errors_for_email() {
    let config = ValidationConfig::new().with_fail_fast(false);
    let mut v: Validator<String> = Validator::with_config(config);
    // "x" fails min_len(5) and also fails max_len(0) — two distinct errors
    v.add_constraint("email", Constraints::min_len(5));
    v.add_constraint("email", Constraints::max_len(0));

    let result = v.validate(&"x".to_string());
    assert!(result.is_err(), "email 'x' must fail both constraints");
    let errors = result.expect_err("must collect all errors");
    assert!(
        errors.len() >= 2,
        "fail_fast=false must collect at least 2 errors, got {}",
        errors.len()
    );
}

// ---------------------------------------------------------------------------
// 19. ValidationError for customer_id carries correct field name
// ---------------------------------------------------------------------------
#[test]
fn test_validation_error_for_customer_id_carries_correct_field_name() {
    let err = ValidationError::new("customer.customer_id", "customer_id must be positive");
    assert_eq!(
        err.field, "customer.customer_id",
        "field must be 'customer.customer_id', got '{}'",
        err.field
    );
    assert_eq!(
        err.message, "customer_id must be positive",
        "message must match, got '{}'",
        err.message
    );
}

// ---------------------------------------------------------------------------
// 20. ValidationError Display for balance_cents
// ---------------------------------------------------------------------------
#[test]
fn test_validation_error_display_for_balance_cents() {
    let err = ValidationError::new(
        "customer.balance_cents",
        "balance_cents must be non-negative",
    );
    let display = format!("{}", err);
    assert!(
        display.contains("customer.balance_cents"),
        "Display must contain field name; got: {}",
        display
    );
    assert!(
        display.contains("balance_cents must be non-negative"),
        "Display must contain message; got: {}",
        display
    );
}

// ---------------------------------------------------------------------------
// 21. NumericValidator chained min/max for monthly_usd
// ---------------------------------------------------------------------------
#[test]
fn test_numeric_validator_chained_min_max_for_monthly_usd() {
    let v = NumericValidator::<u32>::new().min(1u32).max(9_999u32);

    assert!(v.validate(&1u32).is_ok(), "monthly_usd 1 must pass min(1)");
    assert!(
        v.validate(&9_999u32).is_ok(),
        "monthly_usd 9999 must pass max(9999)"
    );
    assert!(v.validate(&0u32).is_err(), "monthly_usd 0 must fail min(1)");
    assert!(
        v.validate(&10_000u32).is_err(),
        "monthly_usd 10000 must fail max(9999)"
    );
}

// ---------------------------------------------------------------------------
// 22. CollectionValidator on Vec<CustomerRecord> checks collection bounds
// ---------------------------------------------------------------------------
#[test]
fn test_collection_validator_on_vec_customer_record_checks_collection_bounds() {
    let records: Vec<CustomerRecord> = vec![
        CustomerRecord {
            customer_id: 1,
            name: "Alice".to_string(),
            email: "alice@example.com".to_string(),
            age: 28,
            balance_cents: 500_00,
        },
        CustomerRecord {
            customer_id: 2,
            name: "Bob".to_string(),
            email: "bob@example.com".to_string(),
            age: 35,
            balance_cents: 0,
        },
    ];

    let coll_v = CollectionValidator::new().min_len(1).max_len(100);
    assert!(
        coll_v.validate(&records).is_ok(),
        "2-element CustomerRecord vec must pass min_len(1)/max_len(100)"
    );

    // Empty vec must fail min_len(1)
    let empty: Vec<CustomerRecord> = vec![];
    assert!(
        coll_v.validate(&empty).is_err(),
        "empty CustomerRecord vec must fail min_len(1)"
    );

    // Additionally validate each record's age field
    let age_v: Validator<u8> =
        Validator::new().constraint("age", Constraints::range(Some(0u8), Some(120u8)));
    for rec in &records {
        assert!(
            age_v.validate(&rec.age).is_ok(),
            "age {} must pass [0, 120]",
            rec.age
        );
    }
}
