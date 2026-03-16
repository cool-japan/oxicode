//! Advanced validation tests — set 9: User registration / account management.
//! 22 top-level #[test] functions, no module wrapper, no #[cfg(test)].

#![cfg(feature = "validation")]
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
    CollectionValidator, Constraints, NumericValidator, ValidationError, Validator,
};
use oxicode::{decode_from_slice, encode_to_vec, Decode, Encode};

// ---------------------------------------------------------------------------
// Validate trait — defined locally since oxicode does not export it.
// ---------------------------------------------------------------------------

trait Validate {
    fn validate(&self) -> Result<(), ValidationError>;
}

// ---------------------------------------------------------------------------
// Domain types used throughout this test file
// ---------------------------------------------------------------------------

#[derive(Debug, PartialEq, Encode, Decode)]
enum AccountType {
    Free,
    Basic,
    Premium,
    Enterprise,
}

#[derive(Debug, PartialEq, Encode, Decode)]
struct UserAccount {
    id: u64,
    username: String,
    email: String,
    account_type: AccountType,
    age: u8,
    is_active: bool,
}

#[derive(Debug, PartialEq, Encode, Decode)]
struct PasswordPolicy {
    min_length: u8,
    require_uppercase: bool,
    require_digits: bool,
    require_symbols: bool,
    max_age_days: u16,
}

// ---------------------------------------------------------------------------
// Validate implementations
// ---------------------------------------------------------------------------

impl Validate for UserAccount {
    fn validate(&self) -> Result<(), ValidationError> {
        if self.username.is_empty() {
            return Err(ValidationError::new("username", "username cannot be empty"));
        }
        if !self.email.contains('@') {
            return Err(ValidationError::new(
                "email",
                "invalid email: must contain '@'",
            ));
        }
        if self.age < 13 {
            return Err(ValidationError::new("age", "must be 13 or older"));
        }
        Ok(())
    }
}

impl Validate for PasswordPolicy {
    fn validate(&self) -> Result<(), ValidationError> {
        if self.min_length < 8 {
            return Err(ValidationError::new(
                "min_length",
                "password minimum length must be at least 8",
            ));
        }
        if self.max_age_days == 0 {
            return Err(ValidationError::new(
                "max_age_days",
                "max_age_days must be greater than zero",
            ));
        }
        Ok(())
    }
}

// ---------------------------------------------------------------------------
// Helper constructors
// ---------------------------------------------------------------------------

fn valid_account() -> UserAccount {
    UserAccount {
        id: 1,
        username: "alice".to_string(),
        email: "alice@example.com".to_string(),
        account_type: AccountType::Free,
        age: 25,
        is_active: true,
    }
}

fn valid_policy() -> PasswordPolicy {
    PasswordPolicy {
        min_length: 12,
        require_uppercase: true,
        require_digits: true,
        require_symbols: false,
        max_age_days: 90,
    }
}

// ---------------------------------------------------------------------------
// 1. Valid UserAccount passes Validate
// ---------------------------------------------------------------------------
#[test]
fn test_valid_user_account_passes_validate() {
    let account = valid_account();
    assert!(
        account.validate().is_ok(),
        "a fully valid UserAccount must pass Validate"
    );
}

// ---------------------------------------------------------------------------
// 2. UserAccount with empty username fails Validate
// ---------------------------------------------------------------------------
#[test]
fn test_user_account_empty_username_fails_validate() {
    let mut account = valid_account();
    account.username = String::new();
    let result = account.validate();
    assert!(result.is_err(), "empty username must fail validation");
    let err = result.expect_err("expected ValidationError for empty username");
    assert_eq!(err.field, "username", "error field must be 'username'");
}

// ---------------------------------------------------------------------------
// 3. UserAccount with email missing '@' fails Validate
// ---------------------------------------------------------------------------
#[test]
fn test_user_account_email_without_at_fails_validate() {
    let mut account = valid_account();
    account.email = "aliceexample.com".to_string();
    let result = account.validate();
    assert!(result.is_err(), "email without '@' must fail validation");
    let err = result.expect_err("expected ValidationError for invalid email");
    assert_eq!(err.field, "email", "error field must be 'email'");
}

// ---------------------------------------------------------------------------
// 4. UserAccount with age below 13 fails Validate
// ---------------------------------------------------------------------------
#[test]
fn test_user_account_age_below_13_fails_validate() {
    let mut account = valid_account();
    account.age = 12;
    let result = account.validate();
    assert!(result.is_err(), "age 12 must fail validation (min 13)");
    let err = result.expect_err("expected ValidationError for age < 13");
    assert_eq!(err.field, "age", "error field must be 'age'");
}

// ---------------------------------------------------------------------------
// 5. UserAccount with age exactly 13 passes Validate
// ---------------------------------------------------------------------------
#[test]
fn test_user_account_age_exactly_13_passes_validate() {
    let mut account = valid_account();
    account.age = 13;
    assert!(
        account.validate().is_ok(),
        "age 13 must pass validation (boundary)"
    );
}

// ---------------------------------------------------------------------------
// 6. Valid PasswordPolicy passes Validate
// ---------------------------------------------------------------------------
#[test]
fn test_valid_password_policy_passes_validate() {
    let policy = valid_policy();
    assert!(
        policy.validate().is_ok(),
        "a fully valid PasswordPolicy must pass Validate"
    );
}

// ---------------------------------------------------------------------------
// 7. PasswordPolicy with min_length < 8 fails Validate
// ---------------------------------------------------------------------------
#[test]
fn test_password_policy_min_length_below_8_fails_validate() {
    let mut policy = valid_policy();
    policy.min_length = 7;
    let result = policy.validate();
    assert!(result.is_err(), "min_length 7 must fail validation (min 8)");
    let err = result.expect_err("expected ValidationError for min_length < 8");
    assert_eq!(err.field, "min_length", "error field must be 'min_length'");
}

// ---------------------------------------------------------------------------
// 8. PasswordPolicy with max_age_days == 0 fails Validate
// ---------------------------------------------------------------------------
#[test]
fn test_password_policy_max_age_days_zero_fails_validate() {
    let mut policy = valid_policy();
    policy.max_age_days = 0;
    let result = policy.validate();
    assert!(result.is_err(), "max_age_days 0 must fail validation");
    let err = result.expect_err("expected ValidationError for max_age_days == 0");
    assert_eq!(
        err.field, "max_age_days",
        "error field must be 'max_age_days'"
    );
}

// ---------------------------------------------------------------------------
// 9. UserAccount encode → decode roundtrip preserves all fields
// ---------------------------------------------------------------------------
#[test]
fn test_user_account_encode_decode_roundtrip_preserves_all_fields() {
    let account = UserAccount {
        id: 42,
        username: "bob".to_string(),
        email: "bob@domain.org".to_string(),
        account_type: AccountType::Premium,
        age: 30,
        is_active: false,
    };

    let bytes = encode_to_vec(&account).expect("encode UserAccount must succeed");
    let (decoded, bytes_read): (UserAccount, usize) =
        decode_from_slice(&bytes).expect("decode UserAccount must succeed");

    assert!(bytes_read > 0, "must consume at least one byte");
    assert_eq!(
        decoded, account,
        "UserAccount roundtrip must preserve all fields"
    );
}

// ---------------------------------------------------------------------------
// 10. PasswordPolicy encode → decode roundtrip preserves all fields
// ---------------------------------------------------------------------------
#[test]
fn test_password_policy_encode_decode_roundtrip_preserves_all_fields() {
    let policy = PasswordPolicy {
        min_length: 16,
        require_uppercase: true,
        require_digits: true,
        require_symbols: true,
        max_age_days: 180,
    };

    let bytes = encode_to_vec(&policy).expect("encode PasswordPolicy must succeed");
    let (decoded, bytes_read): (PasswordPolicy, usize) =
        decode_from_slice(&bytes).expect("decode PasswordPolicy must succeed");

    assert!(bytes_read > 0, "must consume at least one byte");
    assert_eq!(
        decoded, policy,
        "PasswordPolicy roundtrip must preserve all fields"
    );
}

// ---------------------------------------------------------------------------
// 11. Encode then decode UserAccount, then validate passes for valid data
// ---------------------------------------------------------------------------
#[test]
fn test_encode_decode_then_validate_passes_for_valid_user_account() {
    let original = valid_account();

    let bytes = encode_to_vec(&original).expect("encode must succeed");
    let (decoded, _): (UserAccount, usize) =
        decode_from_slice(&bytes).expect("decode must succeed");

    assert!(
        decoded.validate().is_ok(),
        "decoded valid UserAccount must pass Validate"
    );
}

// ---------------------------------------------------------------------------
// 12. Encode then decode UserAccount with invalid email, then validate fails
// ---------------------------------------------------------------------------
#[test]
fn test_encode_decode_then_validate_fails_for_invalid_email() {
    let account = UserAccount {
        id: 5,
        username: "carol".to_string(),
        email: "carolNOAT.net".to_string(),
        account_type: AccountType::Basic,
        age: 22,
        is_active: true,
    };

    let bytes = encode_to_vec(&account).expect("encode must succeed");
    let (decoded, _): (UserAccount, usize) =
        decode_from_slice(&bytes).expect("decode must succeed");

    assert!(
        decoded.validate().is_err(),
        "decoded UserAccount with invalid email must fail Validate"
    );
}

// ---------------------------------------------------------------------------
// 13. All AccountType variants encode → decode correctly
// ---------------------------------------------------------------------------
#[test]
fn test_all_account_type_variants_encode_decode_correctly() {
    let variants = [
        AccountType::Free,
        AccountType::Basic,
        AccountType::Premium,
        AccountType::Enterprise,
    ];

    for variant in variants {
        let bytes = encode_to_vec(&variant).expect("encode AccountType must succeed");
        let (decoded, bytes_read): (AccountType, usize) =
            decode_from_slice(&bytes).expect("decode AccountType must succeed");
        assert!(
            bytes_read > 0,
            "must consume at least one byte per AccountType variant"
        );
        assert_eq!(
            decoded, variant,
            "AccountType roundtrip must match original"
        );
    }
}

// ---------------------------------------------------------------------------
// 14. UserAccount with Enterprise account type passes Validate and roundtrips
// ---------------------------------------------------------------------------
#[test]
fn test_user_account_enterprise_type_passes_validate_and_roundtrips() {
    let account = UserAccount {
        id: 999,
        username: "corp_user".to_string(),
        email: "admin@corp.example".to_string(),
        account_type: AccountType::Enterprise,
        age: 40,
        is_active: true,
    };

    assert!(
        account.validate().is_ok(),
        "Enterprise account must pass Validate"
    );

    let bytes = encode_to_vec(&account).expect("encode Enterprise UserAccount must succeed");
    let (decoded, bytes_read): (UserAccount, usize) =
        decode_from_slice(&bytes).expect("decode Enterprise UserAccount must succeed");

    assert!(bytes_read > 0, "must consume at least one byte");
    assert_eq!(
        decoded, account,
        "Enterprise UserAccount roundtrip must match original"
    );
}

// ---------------------------------------------------------------------------
// 15. ValidationError new() correctly stores field and message
// ---------------------------------------------------------------------------
#[test]
fn test_validation_error_new_stores_field_and_message() {
    let err = ValidationError::new("username", "username cannot be empty");
    assert_eq!(
        err.field, "username",
        "ValidationError field must be 'username'"
    );
    assert_eq!(
        err.message, "username cannot be empty",
        "ValidationError message must match"
    );
}

// ---------------------------------------------------------------------------
// 16. ValidationError Display includes field name
// ---------------------------------------------------------------------------
#[test]
fn test_validation_error_display_includes_field_name() {
    let err = ValidationError::new("email", "invalid email: must contain '@'");
    let display = format!("{}", err);
    assert!(
        display.contains("email"),
        "ValidationError Display must contain field name 'email'"
    );
    assert!(
        display.contains("invalid email"),
        "ValidationError Display must contain message text"
    );
}

// ---------------------------------------------------------------------------
// 17. Validator<u8> age range accepts boundary values 13 and 120
// ---------------------------------------------------------------------------
#[test]
fn test_validator_u8_age_range_accepts_boundary_values_13_and_120() {
    let v: Validator<u8> =
        Validator::new().constraint("age", Constraints::range(Some(13u8), Some(120u8)));

    assert!(v.validate(&13u8).is_ok(), "age 13 must pass [13, 120]");
    assert!(v.validate(&120u8).is_ok(), "age 120 must pass [13, 120]");
    assert!(v.validate(&12u8).is_err(), "age 12 must fail [13, 120]");
    assert!(v.validate(&121u8).is_err(), "age 121 must fail [13, 120]");
}

// ---------------------------------------------------------------------------
// 18. Validator<String> username non_empty and max_len enforced
// ---------------------------------------------------------------------------
#[test]
fn test_validator_string_username_non_empty_and_max_len_enforced() {
    let v: Validator<String> = Validator::new()
        .constraint("username", Constraints::non_empty())
        .constraint("username", Constraints::max_len(64));

    assert!(
        v.validate(&"alice".to_string()).is_ok(),
        "non-empty short username must pass"
    );
    assert!(
        v.validate(&String::new()).is_err(),
        "empty username must fail non_empty"
    );
    let too_long = "x".repeat(65);
    assert!(
        v.validate(&too_long).is_err(),
        "65-char username must fail max_len(64)"
    );
}

// ---------------------------------------------------------------------------
// 19. Validator<u8> min_length for PasswordPolicy: boundary at 8
// ---------------------------------------------------------------------------
#[test]
fn test_validator_u8_min_length_boundary_at_8() {
    let v: Validator<u8> =
        Validator::new().constraint("min_length", Constraints::range(Some(8u8), Some(255u8)));

    assert!(v.validate(&8u8).is_ok(), "min_length 8 must pass [8, 255]");
    assert!(
        v.validate(&255u8).is_ok(),
        "min_length 255 must pass [8, 255]"
    );
    assert!(v.validate(&7u8).is_err(), "min_length 7 must fail [8, 255]");
    assert!(v.validate(&0u8).is_err(), "min_length 0 must fail [8, 255]");
}

// ---------------------------------------------------------------------------
// 20. NumericValidator for max_age_days rejects 0 and accepts positive values
// ---------------------------------------------------------------------------
#[test]
fn test_numeric_validator_max_age_days_rejects_zero_accepts_positive() {
    let v = NumericValidator::<u16>::new().min(1u16);

    assert!(v.validate(&1u16).is_ok(), "max_age_days 1 must pass min(1)");
    assert!(
        v.validate(&365u16).is_ok(),
        "max_age_days 365 must pass min(1)"
    );
    assert!(
        v.validate(&0u16).is_err(),
        "max_age_days 0 must fail min(1)"
    );
}

// ---------------------------------------------------------------------------
// 21. Vec<UserAccount> encode → decode roundtrip
// ---------------------------------------------------------------------------
#[test]
fn test_vec_user_account_encode_decode_roundtrip() {
    let accounts: Vec<UserAccount> = vec![
        UserAccount {
            id: 1,
            username: "alice".to_string(),
            email: "alice@example.com".to_string(),
            account_type: AccountType::Free,
            age: 20,
            is_active: true,
        },
        UserAccount {
            id: 2,
            username: "bob".to_string(),
            email: "bob@example.com".to_string(),
            account_type: AccountType::Basic,
            age: 35,
            is_active: true,
        },
        UserAccount {
            id: 3,
            username: "carol".to_string(),
            email: "carol@corp.example".to_string(),
            account_type: AccountType::Enterprise,
            age: 50,
            is_active: false,
        },
    ];

    let bytes = encode_to_vec(&accounts).expect("encode Vec<UserAccount> must succeed");
    let (decoded, bytes_read): (Vec<UserAccount>, usize) =
        decode_from_slice(&bytes).expect("decode Vec<UserAccount> must succeed");

    assert!(bytes_read > 0, "must consume at least one byte");
    assert_eq!(decoded.len(), 3, "decoded Vec must have 3 elements");
    assert_eq!(
        decoded, accounts,
        "Vec<UserAccount> roundtrip must preserve all accounts"
    );
}

// ---------------------------------------------------------------------------
// 22. CollectionValidator on Vec<UserAccount> and per-account Validate
// ---------------------------------------------------------------------------
#[test]
fn test_collection_validator_on_vec_user_account_and_per_account_validate() {
    let accounts: Vec<UserAccount> = vec![
        UserAccount {
            id: 10,
            username: "dave".to_string(),
            email: "dave@example.com".to_string(),
            account_type: AccountType::Premium,
            age: 28,
            is_active: true,
        },
        UserAccount {
            id: 11,
            username: "eve".to_string(),
            email: "eve@example.com".to_string(),
            account_type: AccountType::Basic,
            age: 17,
            is_active: true,
        },
    ];

    let coll_v = CollectionValidator::new()
        .min_len(1)
        .max_len(1000)
        .non_empty();
    assert!(
        coll_v.validate(&accounts).is_ok(),
        "2-element Vec<UserAccount> must pass collection constraints"
    );

    let empty: Vec<UserAccount> = vec![];
    assert!(
        coll_v.validate(&empty).is_err(),
        "empty Vec<UserAccount> must fail non_empty constraint"
    );

    // All accounts in the list are valid — confirm each passes Validate
    for account in &accounts {
        assert!(
            account.validate().is_ok(),
            "account with id {} must pass Validate",
            account.id
        );
    }

    // Construct a deliberately invalid account and confirm Validate rejects it
    let underage = UserAccount {
        id: 99,
        username: "young".to_string(),
        email: "young@example.com".to_string(),
        account_type: AccountType::Free,
        age: 10,
        is_active: false,
    };
    assert!(
        underage.validate().is_err(),
        "underage account (age 10) must fail Validate"
    );

    // Verify ValidationError equality for identical errors
    let err_a = ValidationError::new("age", "must be 13 or older");
    let err_b = ValidationError::new("age", "must be 13 or older");
    assert_eq!(err_a, err_b, "identical ValidationErrors must be equal");

    // Verify bytes_read == encoded length for one account
    let account_to_encode = &accounts[0];
    let bytes = encode_to_vec(account_to_encode).expect("encode single UserAccount must succeed");
    let (_decoded_account, bytes_read): (UserAccount, usize) =
        decode_from_slice(&bytes).expect("decode single UserAccount must succeed");
    assert_eq!(
        bytes_read,
        bytes.len(),
        "bytes_read ({}) must equal encoded length ({})",
        bytes_read,
        bytes.len()
    );
}
