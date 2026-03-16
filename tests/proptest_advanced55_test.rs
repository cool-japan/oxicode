//! Proptest-based tests for insurance / actuarial science domain

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
use oxicode::{decode_from_slice, encode_to_vec, Decode, Encode};
use proptest::prelude::*;

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
enum PolicyType {
    Life,
    Health,
    Auto,
    Homeowners,
    Commercial,
    Liability,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
enum ClaimStatus {
    Filed,
    UnderReview,
    Approved,
    Denied,
    Paid,
    Appealed,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct RiskProfile {
    risk_score: u16,
    age: u8,
    coverage_amount_cents: u64,
    annual_premium_cents: u32,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct InsuranceClaim {
    claim_id: u64,
    policy_id: u64,
    policy_type: PolicyType,
    status: ClaimStatus,
    amount_cents: u64,
    filed_at: u64,
}

fn policy_type_strategy() -> impl Strategy<Value = PolicyType> {
    (0u8..6).prop_map(|v| match v {
        0 => PolicyType::Life,
        1 => PolicyType::Health,
        2 => PolicyType::Auto,
        3 => PolicyType::Homeowners,
        4 => PolicyType::Commercial,
        _ => PolicyType::Liability,
    })
}

fn claim_status_strategy() -> impl Strategy<Value = ClaimStatus> {
    (0u8..6).prop_map(|v| match v {
        0 => ClaimStatus::Filed,
        1 => ClaimStatus::UnderReview,
        2 => ClaimStatus::Approved,
        3 => ClaimStatus::Denied,
        4 => ClaimStatus::Paid,
        _ => ClaimStatus::Appealed,
    })
}

fn risk_profile_strategy() -> impl Strategy<Value = RiskProfile> {
    (any::<u16>(), any::<u8>(), any::<u64>(), any::<u32>()).prop_map(
        |(risk_score, age, coverage_amount_cents, annual_premium_cents)| RiskProfile {
            risk_score,
            age,
            coverage_amount_cents,
            annual_premium_cents,
        },
    )
}

fn insurance_claim_strategy() -> impl Strategy<Value = InsuranceClaim> {
    (
        any::<u64>(),
        any::<u64>(),
        policy_type_strategy(),
        claim_status_strategy(),
        any::<u64>(),
        any::<u64>(),
    )
        .prop_map(
            |(claim_id, policy_id, policy_type, status, amount_cents, filed_at)| InsuranceClaim {
                claim_id,
                policy_id,
                policy_type,
                status,
                amount_cents,
                filed_at,
            },
        )
}

proptest! {
    #[test]
    fn test_risk_profile_roundtrip(profile in risk_profile_strategy()) {
        let encoded = encode_to_vec(&profile).expect("RiskProfile encode failed");
        let (decoded, _): (RiskProfile, usize) =
            decode_from_slice(&encoded).expect("RiskProfile decode failed");
        prop_assert_eq!(profile, decoded);
    }

    #[test]
    fn test_insurance_claim_roundtrip(claim in insurance_claim_strategy()) {
        let encoded = encode_to_vec(&claim).expect("InsuranceClaim encode failed");
        let (decoded, _): (InsuranceClaim, usize) =
            decode_from_slice(&encoded).expect("InsuranceClaim decode failed");
        prop_assert_eq!(claim, decoded);
    }

    #[test]
    fn test_risk_profile_consumed_bytes_equals_encoded_length(profile in risk_profile_strategy()) {
        let encoded = encode_to_vec(&profile).expect("RiskProfile encode failed");
        let (_, consumed): (RiskProfile, usize) =
            decode_from_slice(&encoded).expect("RiskProfile decode failed");
        prop_assert_eq!(consumed, encoded.len());
    }

    #[test]
    fn test_insurance_claim_consumed_bytes_equals_encoded_length(claim in insurance_claim_strategy()) {
        let encoded = encode_to_vec(&claim).expect("InsuranceClaim encode failed");
        let (_, consumed): (InsuranceClaim, usize) =
            decode_from_slice(&encoded).expect("InsuranceClaim decode failed");
        prop_assert_eq!(consumed, encoded.len());
    }

    #[test]
    fn test_risk_profile_encode_deterministic(profile in risk_profile_strategy()) {
        let encoded_a = encode_to_vec(&profile).expect("RiskProfile encode_a failed");
        let encoded_b = encode_to_vec(&profile).expect("RiskProfile encode_b failed");
        prop_assert_eq!(encoded_a, encoded_b);
    }

    #[test]
    fn test_insurance_claim_encode_deterministic(claim in insurance_claim_strategy()) {
        let encoded_a = encode_to_vec(&claim).expect("InsuranceClaim encode_a failed");
        let encoded_b = encode_to_vec(&claim).expect("InsuranceClaim encode_b failed");
        prop_assert_eq!(encoded_a, encoded_b);
    }

    #[test]
    fn test_vec_insurance_claims_roundtrip(claims in proptest::collection::vec(insurance_claim_strategy(), 0..8)) {
        let encoded = encode_to_vec(&claims).expect("Vec<InsuranceClaim> encode failed");
        let (decoded, _): (Vec<InsuranceClaim>, usize) =
            decode_from_slice(&encoded).expect("Vec<InsuranceClaim> decode failed");
        prop_assert_eq!(claims, decoded);
    }

    #[test]
    fn test_option_insurance_claim_roundtrip(maybe_claim in proptest::option::of(insurance_claim_strategy())) {
        let encoded = encode_to_vec(&maybe_claim).expect("Option<InsuranceClaim> encode failed");
        let (decoded, _): (Option<InsuranceClaim>, usize) =
            decode_from_slice(&encoded).expect("Option<InsuranceClaim> decode failed");
        prop_assert_eq!(maybe_claim, decoded);
    }

    #[test]
    fn test_policy_type_variant_roundtrip(variant_index in 0u8..6) {
        let policy_type = match variant_index {
            0 => PolicyType::Life,
            1 => PolicyType::Health,
            2 => PolicyType::Auto,
            3 => PolicyType::Homeowners,
            4 => PolicyType::Commercial,
            _ => PolicyType::Liability,
        };
        let encoded = encode_to_vec(&policy_type).expect("PolicyType encode failed");
        let (decoded, _): (PolicyType, usize) =
            decode_from_slice(&encoded).expect("PolicyType decode failed");
        prop_assert_eq!(policy_type, decoded);
    }

    #[test]
    fn test_claim_status_variant_roundtrip(variant_index in 0u8..6) {
        let status = match variant_index {
            0 => ClaimStatus::Filed,
            1 => ClaimStatus::UnderReview,
            2 => ClaimStatus::Approved,
            3 => ClaimStatus::Denied,
            4 => ClaimStatus::Paid,
            _ => ClaimStatus::Appealed,
        };
        let encoded = encode_to_vec(&status).expect("ClaimStatus encode failed");
        let (decoded, _): (ClaimStatus, usize) =
            decode_from_slice(&encoded).expect("ClaimStatus decode failed");
        prop_assert_eq!(status, decoded);
    }

    #[test]
    fn test_u8_roundtrip(val in any::<u8>()) {
        let encoded = encode_to_vec(&val).expect("u8 encode failed");
        let (decoded, _): (u8, usize) = decode_from_slice(&encoded).expect("u8 decode failed");
        prop_assert_eq!(val, decoded);
    }

    #[test]
    fn test_i32_roundtrip(val in any::<i32>()) {
        let encoded = encode_to_vec(&val).expect("i32 encode failed");
        let (decoded, _): (i32, usize) = decode_from_slice(&encoded).expect("i32 decode failed");
        prop_assert_eq!(val, decoded);
    }

    #[test]
    fn test_u64_roundtrip(val in any::<u64>()) {
        let encoded = encode_to_vec(&val).expect("u64 encode failed");
        let (decoded, _): (u64, usize) = decode_from_slice(&encoded).expect("u64 decode failed");
        prop_assert_eq!(val, decoded);
    }

    #[test]
    fn test_i64_roundtrip(val in any::<i64>()) {
        let encoded = encode_to_vec(&val).expect("i64 encode failed");
        let (decoded, _): (i64, usize) = decode_from_slice(&encoded).expect("i64 decode failed");
        prop_assert_eq!(val, decoded);
    }

    #[test]
    fn test_bool_roundtrip(val in any::<bool>()) {
        let encoded = encode_to_vec(&val).expect("bool encode failed");
        let (decoded, _): (bool, usize) = decode_from_slice(&encoded).expect("bool decode failed");
        prop_assert_eq!(val, decoded);
    }

    #[test]
    fn test_string_roundtrip(val in ".*") {
        let encoded = encode_to_vec(&val).expect("String encode failed");
        let (decoded, _): (String, usize) =
            decode_from_slice(&encoded).expect("String decode failed");
        prop_assert_eq!(val, decoded);
    }

    #[test]
    fn test_f32_roundtrip(val in proptest::num::f32::NORMAL | proptest::num::f32::ZERO) {
        let encoded = encode_to_vec(&val).expect("f32 encode failed");
        let (decoded, _): (f32, usize) = decode_from_slice(&encoded).expect("f32 decode failed");
        prop_assert_eq!(val.to_bits(), decoded.to_bits());
    }

    #[test]
    fn test_f64_roundtrip(val in proptest::num::f64::NORMAL | proptest::num::f64::ZERO) {
        let encoded = encode_to_vec(&val).expect("f64 encode failed");
        let (decoded, _): (f64, usize) = decode_from_slice(&encoded).expect("f64 decode failed");
        prop_assert_eq!(val.to_bits(), decoded.to_bits());
    }

    #[test]
    fn test_vec_u8_roundtrip(val in proptest::collection::vec(any::<u8>(), 0..64)) {
        let encoded = encode_to_vec(&val).expect("Vec<u8> encode failed");
        let (decoded, _): (Vec<u8>, usize) =
            decode_from_slice(&encoded).expect("Vec<u8> decode failed");
        prop_assert_eq!(val, decoded);
    }

    #[test]
    fn test_vec_string_roundtrip(val in proptest::collection::vec(".*", 0..8)) {
        let encoded = encode_to_vec(&val).expect("Vec<String> encode failed");
        let (decoded, _): (Vec<String>, usize) =
            decode_from_slice(&encoded).expect("Vec<String> decode failed");
        prop_assert_eq!(val, decoded);
    }

    #[test]
    fn test_option_u64_roundtrip(val in proptest::option::of(any::<u64>())) {
        let encoded = encode_to_vec(&val).expect("Option<u64> encode failed");
        let (decoded, _): (Option<u64>, usize) =
            decode_from_slice(&encoded).expect("Option<u64> decode failed");
        prop_assert_eq!(val, decoded);
    }

    #[test]
    fn test_distinct_insurance_claims_have_distinct_or_equal_bytes(
        claim_a in insurance_claim_strategy(),
        claim_b in insurance_claim_strategy(),
    ) {
        let encoded_a = encode_to_vec(&claim_a).expect("InsuranceClaim claim_a encode failed");
        let encoded_b = encode_to_vec(&claim_b).expect("InsuranceClaim claim_b encode failed");
        if claim_a == claim_b {
            prop_assert_eq!(&encoded_a, &encoded_b);
        } else {
            prop_assert_ne!(&encoded_a, &encoded_b);
        }
    }

    #[test]
    fn test_risk_profile_zero_values(_dummy in 0u8..1) {
        let profile = RiskProfile {
            risk_score: 0,
            age: 0,
            coverage_amount_cents: 0,
            annual_premium_cents: 0,
        };
        let encoded = encode_to_vec(&profile).expect("zero RiskProfile encode failed");
        let (decoded, consumed): (RiskProfile, usize) =
            decode_from_slice(&encoded).expect("zero RiskProfile decode failed");
        prop_assert_eq!(profile, decoded);
        prop_assert_eq!(consumed, encoded.len());
    }

    #[test]
    fn test_risk_profile_max_coverage_amount(_dummy in 0u8..1) {
        let profile = RiskProfile {
            risk_score: u16::MAX,
            age: u8::MAX,
            coverage_amount_cents: u64::MAX,
            annual_premium_cents: u32::MAX,
        };
        let encoded = encode_to_vec(&profile).expect("max RiskProfile encode failed");
        let (decoded, consumed): (RiskProfile, usize) =
            decode_from_slice(&encoded).expect("max RiskProfile decode failed");
        prop_assert_eq!(profile, decoded);
        prop_assert_eq!(consumed, encoded.len());
    }
}
