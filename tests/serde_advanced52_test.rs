#![cfg(feature = "serde")]
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
use oxicode::config;
use oxicode::serde::{decode_owned_from_slice, encode_to_vec};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

// ---------------------------------------------------------------------------
// Domain types — Insurance & Actuarial Science
// ---------------------------------------------------------------------------

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
enum PolicyType {
    Auto,
    Homeowners,
    Life,
    Health,
    CommercialProperty,
    ProfessionalLiability,
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
enum ClaimStatus {
    Filed,
    UnderReview,
    Approved,
    Denied,
    InLitigation,
    Settled,
    Closed,
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
struct PolicyRecord {
    policy_id: String,
    policy_type: PolicyType,
    holder_name: String,
    effective_date: String,
    expiry_date: String,
    premium_annual_cents: u64,
    coverage_limit_cents: u64,
    is_active: bool,
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
struct PremiumCalculation {
    base_premium_cents: u64,
    risk_loading_bps: u32,
    expense_loading_bps: u32,
    commission_bps: u32,
    tax_rate_bps: u32,
    discount_bps: u32,
    final_premium_cents: u64,
    calculation_date: String,
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
struct ClaimRecord {
    claim_id: String,
    policy_id: String,
    status: ClaimStatus,
    date_of_loss: String,
    date_reported: String,
    loss_amount_cents: u64,
    reserve_amount_cents: u64,
    paid_amount_cents: u64,
    adjuster_id: String,
    description: String,
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
struct ClaimsWorkflow {
    claim: ClaimRecord,
    steps_completed: Vec<String>,
    pending_documents: Vec<String>,
    approval_required: bool,
    estimated_resolution_days: u32,
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
struct MortalityTableEntry {
    age: u16,
    male_qx_per_million: u32,
    female_qx_per_million: u32,
    unisex_qx_per_million: u32,
    life_expectancy_months: u32,
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
struct MortalityTable {
    table_name: String,
    base_year: u16,
    projection_scale: String,
    entries: Vec<MortalityTableEntry>,
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
struct LossRatioAnalysis {
    line_of_business: String,
    accident_year: u16,
    earned_premium_cents: u64,
    incurred_losses_cents: u64,
    loss_ratio_bps: u32,
    combined_ratio_bps: u32,
    development_factor_bps: u32,
    ultimate_loss_cents: u64,
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
enum ReinsuranceType {
    QuotaShare,
    SurplusShare,
    ExcessOfLoss,
    StopLoss,
    CatastropheXL,
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
struct ReinsuranceTreaty {
    treaty_id: String,
    reinsurance_type: ReinsuranceType,
    ceding_company: String,
    reinsurer: String,
    retention_cents: u64,
    limit_cents: u64,
    cession_rate_bps: u32,
    effective_date: String,
    expiry_date: String,
    reinstatements: u8,
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
struct CatastropheModelOutput {
    peril: String,
    region: String,
    return_period_years: u32,
    aal_cents: u64,
    oep_100yr_cents: u64,
    oep_250yr_cents: u64,
    oep_500yr_cents: u64,
    aep_100yr_cents: u64,
    tvar_99_5_cents: u64,
    model_version: String,
    num_simulations: u32,
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
struct UnderwritingRiskScore {
    applicant_id: String,
    overall_score: u16,
    credit_score: u16,
    claims_history_score: u16,
    property_condition_score: u16,
    location_risk_score: u16,
    driving_record_score: u16,
    recommendation: String,
    factors: Vec<String>,
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
enum EndorsementAction {
    Add,
    Remove,
    Modify,
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
struct PolicyEndorsement {
    endorsement_id: String,
    policy_id: String,
    action: EndorsementAction,
    rider_name: String,
    effective_date: String,
    premium_change_cents: i64,
    description: String,
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
struct DeductibleStructure {
    policy_id: String,
    standard_deductible_cents: u64,
    wind_hail_deductible_bps: u32,
    hurricane_deductible_bps: u32,
    earthquake_deductible_bps: u32,
    flood_deductible_cents: u64,
    all_other_perils_cents: u64,
    aggregate_deductible_cents: u64,
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
struct CopayCoinsuranceSplit {
    plan_id: String,
    primary_care_copay_cents: u32,
    specialist_copay_cents: u32,
    er_copay_cents: u32,
    coinsurance_in_network_bps: u32,
    coinsurance_out_network_bps: u32,
    out_of_pocket_max_cents: u64,
    family_out_of_pocket_max_cents: u64,
    pharmacy_tier1_copay_cents: u32,
    pharmacy_tier2_copay_cents: u32,
    pharmacy_tier3_copay_cents: u32,
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
struct IbnrReserve {
    valuation_date: String,
    line_of_business: String,
    accident_year: u16,
    reported_claims_count: u32,
    ibnr_claims_count_estimate: u32,
    reported_losses_cents: u64,
    ibnr_losses_cents: u64,
    total_ultimate_cents: u64,
    development_method: String,
    confidence_level_bps: u32,
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
struct ExperienceRating {
    employer_id: String,
    rating_period: String,
    expected_losses_cents: u64,
    actual_losses_cents: u64,
    experience_mod_factor_bps: u32,
    credibility_factor_bps: u32,
    ballast_value_cents: u64,
    manual_premium_cents: u64,
    modified_premium_cents: u64,
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
struct FraudDetectionScore {
    claim_id: String,
    fraud_probability_bps: u32,
    anomaly_score: u16,
    red_flags: Vec<String>,
    network_link_count: u32,
    text_analytics_score: u16,
    geographic_anomaly: bool,
    velocity_trigger: bool,
    referral_recommended: bool,
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
struct SolvencyIiCapital {
    entity_name: String,
    reporting_date: String,
    scr_market_risk_cents: u64,
    scr_counterparty_risk_cents: u64,
    scr_underwriting_life_cents: u64,
    scr_underwriting_health_cents: u64,
    scr_underwriting_nonlife_cents: u64,
    scr_operational_risk_cents: u64,
    diversification_benefit_cents: u64,
    total_scr_cents: u64,
    eligible_own_funds_cents: u64,
    solvency_ratio_bps: u32,
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
struct PolicyPortfolio {
    portfolio_id: String,
    policies: Vec<PolicyRecord>,
    total_premium_cents: u64,
    total_exposure_cents: u64,
    average_retention_months: u32,
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
struct ActuarialAssumption {
    assumption_name: String,
    base_value_bps: u32,
    trend_bps: i32,
    volatility_bps: u32,
    confidence_interval_low_bps: u32,
    confidence_interval_high_bps: u32,
    source: String,
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
struct LossTriangleRow {
    accident_year: u16,
    development_months: Vec<u32>,
    cumulative_paid_cents: Vec<u64>,
    cumulative_incurred_cents: Vec<u64>,
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
struct LossTriangle {
    line_of_business: String,
    evaluation_date: String,
    rows: Vec<LossTriangleRow>,
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
struct RegulatoryFiling {
    filing_id: String,
    jurisdiction: String,
    filing_type: String,
    status: String,
    capital_requirements: SolvencyIiCapital,
    risk_factors: HashMap<String, u32>,
}

// ---------------------------------------------------------------------------
// Test 1: Auto insurance policy record roundtrip
// ---------------------------------------------------------------------------

#[test]
fn test_auto_policy_record_roundtrip() {
    let policy = PolicyRecord {
        policy_id: "AUTO-2026-000142".to_string(),
        policy_type: PolicyType::Auto,
        holder_name: "Tanaka Hiroshi".to_string(),
        effective_date: "2026-01-15".to_string(),
        expiry_date: "2027-01-15".to_string(),
        premium_annual_cents: 128_500,
        coverage_limit_cents: 500_000_00,
        is_active: true,
    };
    let enc = encode_to_vec(&policy, config::standard()).expect("encode auto policy failed");
    let (decoded, _): (PolicyRecord, usize) =
        decode_owned_from_slice(&enc, config::standard()).expect("decode auto policy failed");
    assert_eq!(policy, decoded);
}

// ---------------------------------------------------------------------------
// Test 2: Multiple policy types roundtrip
// ---------------------------------------------------------------------------

#[test]
fn test_multiple_policy_types_roundtrip() {
    let policies = vec![
        PolicyRecord {
            policy_id: "HOME-2026-003321".to_string(),
            policy_type: PolicyType::Homeowners,
            holder_name: "Suzuki Yuki".to_string(),
            effective_date: "2026-03-01".to_string(),
            expiry_date: "2027-03-01".to_string(),
            premium_annual_cents: 245_000,
            coverage_limit_cents: 35_000_000,
            is_active: true,
        },
        PolicyRecord {
            policy_id: "LIFE-2026-019887".to_string(),
            policy_type: PolicyType::Life,
            holder_name: "Watanabe Kenji".to_string(),
            effective_date: "2026-02-01".to_string(),
            expiry_date: "2056-02-01".to_string(),
            premium_annual_cents: 48_000,
            coverage_limit_cents: 100_000_000,
            is_active: true,
        },
        PolicyRecord {
            policy_id: "HEALTH-2026-055443".to_string(),
            policy_type: PolicyType::Health,
            holder_name: "Sato Mika".to_string(),
            effective_date: "2026-01-01".to_string(),
            expiry_date: "2026-12-31".to_string(),
            premium_annual_cents: 720_000,
            coverage_limit_cents: 0,
            is_active: false,
        },
    ];
    let enc = encode_to_vec(&policies, config::standard()).expect("encode policy list failed");
    let (decoded, _): (Vec<PolicyRecord>, usize) =
        decode_owned_from_slice(&enc, config::standard()).expect("decode policy list failed");
    assert_eq!(policies, decoded);
}

// ---------------------------------------------------------------------------
// Test 3: Premium calculation roundtrip
// ---------------------------------------------------------------------------

#[test]
fn test_premium_calculation_roundtrip() {
    let calc = PremiumCalculation {
        base_premium_cents: 100_000,
        risk_loading_bps: 1_500,
        expense_loading_bps: 2_000,
        commission_bps: 1_000,
        tax_rate_bps: 800,
        discount_bps: 500,
        final_premium_cents: 148_800,
        calculation_date: "2026-03-15".to_string(),
    };
    let enc = encode_to_vec(&calc, config::standard()).expect("encode premium calc failed");
    let (decoded, _): (PremiumCalculation, usize) =
        decode_owned_from_slice(&enc, config::standard()).expect("decode premium calc failed");
    assert_eq!(calc, decoded);
}

// ---------------------------------------------------------------------------
// Test 4: Claims processing workflow roundtrip
// ---------------------------------------------------------------------------

#[test]
fn test_claims_workflow_roundtrip() {
    let workflow = ClaimsWorkflow {
        claim: ClaimRecord {
            claim_id: "CLM-2026-112233".to_string(),
            policy_id: "HOME-2026-003321".to_string(),
            status: ClaimStatus::UnderReview,
            date_of_loss: "2026-02-20".to_string(),
            date_reported: "2026-02-21".to_string(),
            loss_amount_cents: 1_250_000,
            reserve_amount_cents: 1_500_000,
            paid_amount_cents: 0,
            adjuster_id: "ADJ-0042".to_string(),
            description: "Water damage from burst pipe in basement".to_string(),
        },
        steps_completed: vec![
            "initial_report".to_string(),
            "adjuster_assigned".to_string(),
            "site_inspection_scheduled".to_string(),
        ],
        pending_documents: vec![
            "contractor_estimate".to_string(),
            "photos_of_damage".to_string(),
        ],
        approval_required: true,
        estimated_resolution_days: 45,
    };
    let enc = encode_to_vec(&workflow, config::standard()).expect("encode claims workflow failed");
    let (decoded, _): (ClaimsWorkflow, usize) =
        decode_owned_from_slice(&enc, config::standard()).expect("decode claims workflow failed");
    assert_eq!(workflow, decoded);
}

// ---------------------------------------------------------------------------
// Test 5: Actuarial mortality table roundtrip
// ---------------------------------------------------------------------------

#[test]
fn test_mortality_table_roundtrip() {
    let table = MortalityTable {
        table_name: "CSO-2017-Ultimate".to_string(),
        base_year: 2017,
        projection_scale: "MP-2021".to_string(),
        entries: vec![
            MortalityTableEntry {
                age: 30,
                male_qx_per_million: 978,
                female_qx_per_million: 518,
                unisex_qx_per_million: 748,
                life_expectancy_months: 594,
            },
            MortalityTableEntry {
                age: 50,
                male_qx_per_million: 4_235,
                female_qx_per_million: 2_678,
                unisex_qx_per_million: 3_456,
                life_expectancy_months: 378,
            },
            MortalityTableEntry {
                age: 70,
                male_qx_per_million: 21_456,
                female_qx_per_million: 14_332,
                unisex_qx_per_million: 17_894,
                life_expectancy_months: 180,
            },
            MortalityTableEntry {
                age: 90,
                male_qx_per_million: 158_234,
                female_qx_per_million: 128_567,
                unisex_qx_per_million: 143_400,
                life_expectancy_months: 48,
            },
        ],
    };
    let enc = encode_to_vec(&table, config::standard()).expect("encode mortality table failed");
    let (decoded, _): (MortalityTable, usize) =
        decode_owned_from_slice(&enc, config::standard()).expect("decode mortality table failed");
    assert_eq!(table, decoded);
}

// ---------------------------------------------------------------------------
// Test 6: Loss ratio analysis roundtrip
// ---------------------------------------------------------------------------

#[test]
fn test_loss_ratio_analysis_roundtrip() {
    let analysis = LossRatioAnalysis {
        line_of_business: "Commercial Auto".to_string(),
        accident_year: 2025,
        earned_premium_cents: 50_000_000_00,
        incurred_losses_cents: 32_500_000_00,
        loss_ratio_bps: 6_500,
        combined_ratio_bps: 9_800,
        development_factor_bps: 10_350,
        ultimate_loss_cents: 33_637_500_00,
    };
    let enc = encode_to_vec(&analysis, config::standard()).expect("encode loss ratio failed");
    let (decoded, _): (LossRatioAnalysis, usize) =
        decode_owned_from_slice(&enc, config::standard()).expect("decode loss ratio failed");
    assert_eq!(analysis, decoded);
}

// ---------------------------------------------------------------------------
// Test 7: Reinsurance treaty roundtrip
// ---------------------------------------------------------------------------

#[test]
fn test_reinsurance_treaty_roundtrip() {
    let treaty = ReinsuranceTreaty {
        treaty_id: "RE-2026-CAT-001".to_string(),
        reinsurance_type: ReinsuranceType::CatastropheXL,
        ceding_company: "Nippon Fire & Marine".to_string(),
        reinsurer: "Swiss Re".to_string(),
        retention_cents: 500_000_000_00,
        limit_cents: 2_000_000_000_00,
        cession_rate_bps: 0,
        effective_date: "2026-04-01".to_string(),
        expiry_date: "2027-03-31".to_string(),
        reinstatements: 2,
    };
    let enc = encode_to_vec(&treaty, config::standard()).expect("encode reinsurance treaty failed");
    let (decoded, _): (ReinsuranceTreaty, usize) =
        decode_owned_from_slice(&enc, config::standard())
            .expect("decode reinsurance treaty failed");
    assert_eq!(treaty, decoded);
}

// ---------------------------------------------------------------------------
// Test 8: Multiple reinsurance types roundtrip
// ---------------------------------------------------------------------------

#[test]
fn test_reinsurance_types_roundtrip() {
    let treaties = vec![
        ReinsuranceTreaty {
            treaty_id: "RE-QS-001".to_string(),
            reinsurance_type: ReinsuranceType::QuotaShare,
            ceding_company: "ABC Insurance".to_string(),
            reinsurer: "Munich Re".to_string(),
            retention_cents: 0,
            limit_cents: 1_000_000_000_00,
            cession_rate_bps: 4_000,
            effective_date: "2026-01-01".to_string(),
            expiry_date: "2026-12-31".to_string(),
            reinstatements: 0,
        },
        ReinsuranceTreaty {
            treaty_id: "RE-XL-002".to_string(),
            reinsurance_type: ReinsuranceType::ExcessOfLoss,
            ceding_company: "ABC Insurance".to_string(),
            reinsurer: "Hannover Re".to_string(),
            retention_cents: 100_000_000_00,
            limit_cents: 400_000_000_00,
            cession_rate_bps: 0,
            effective_date: "2026-01-01".to_string(),
            expiry_date: "2026-12-31".to_string(),
            reinstatements: 1,
        },
    ];
    let enc = encode_to_vec(&treaties, config::standard()).expect("encode treaty list failed");
    let (decoded, _): (Vec<ReinsuranceTreaty>, usize) =
        decode_owned_from_slice(&enc, config::standard()).expect("decode treaty list failed");
    assert_eq!(treaties, decoded);
}

// ---------------------------------------------------------------------------
// Test 9: Catastrophe modeling output roundtrip
// ---------------------------------------------------------------------------

#[test]
fn test_catastrophe_model_output_roundtrip() {
    let cat_output = CatastropheModelOutput {
        peril: "Earthquake".to_string(),
        region: "Tokyo Metropolitan".to_string(),
        return_period_years: 250,
        aal_cents: 12_500_000_000,
        oep_100yr_cents: 85_000_000_000,
        oep_250yr_cents: 175_000_000_000,
        oep_500yr_cents: 320_000_000_000,
        aep_100yr_cents: 120_000_000_000,
        tvar_99_5_cents: 280_000_000_000,
        model_version: "RMS-RiskLink-23.1".to_string(),
        num_simulations: 100_000,
    };
    let enc =
        encode_to_vec(&cat_output, config::standard()).expect("encode cat model output failed");
    let (decoded, _): (CatastropheModelOutput, usize) =
        decode_owned_from_slice(&enc, config::standard()).expect("decode cat model output failed");
    assert_eq!(cat_output, decoded);
}

// ---------------------------------------------------------------------------
// Test 10: Underwriting risk score roundtrip
// ---------------------------------------------------------------------------

#[test]
fn test_underwriting_risk_score_roundtrip() {
    let score = UnderwritingRiskScore {
        applicant_id: "APP-2026-78901".to_string(),
        overall_score: 742,
        credit_score: 780,
        claims_history_score: 850,
        property_condition_score: 690,
        location_risk_score: 620,
        driving_record_score: 900,
        recommendation: "Standard acceptance with property inspection required".to_string(),
        factors: vec![
            "coastal_proximity_high".to_string(),
            "roof_age_over_15_years".to_string(),
            "no_prior_claims_5yr".to_string(),
            "credit_score_excellent".to_string(),
        ],
    };
    let enc = encode_to_vec(&score, config::standard()).expect("encode underwriting score failed");
    let (decoded, _): (UnderwritingRiskScore, usize) =
        decode_owned_from_slice(&enc, config::standard())
            .expect("decode underwriting score failed");
    assert_eq!(score, decoded);
}

// ---------------------------------------------------------------------------
// Test 11: Policy endorsement/rider roundtrip
// ---------------------------------------------------------------------------

#[test]
fn test_policy_endorsement_roundtrip() {
    let endorsements = vec![
        PolicyEndorsement {
            endorsement_id: "END-001".to_string(),
            policy_id: "HOME-2026-003321".to_string(),
            action: EndorsementAction::Add,
            rider_name: "Scheduled Personal Property".to_string(),
            effective_date: "2026-04-01".to_string(),
            premium_change_cents: 12_500,
            description: "Adding coverage for jewelry valued at $15,000".to_string(),
        },
        PolicyEndorsement {
            endorsement_id: "END-002".to_string(),
            policy_id: "HOME-2026-003321".to_string(),
            action: EndorsementAction::Modify,
            rider_name: "Water Backup Coverage".to_string(),
            effective_date: "2026-04-01".to_string(),
            premium_change_cents: 3_500,
            description: "Increasing water backup limit from $10k to $25k".to_string(),
        },
        PolicyEndorsement {
            endorsement_id: "END-003".to_string(),
            policy_id: "HOME-2026-003321".to_string(),
            action: EndorsementAction::Remove,
            rider_name: "Trampoline Liability".to_string(),
            effective_date: "2026-04-01".to_string(),
            premium_change_cents: -5_000,
            description: "Trampoline removed from property".to_string(),
        },
    ];
    let enc = encode_to_vec(&endorsements, config::standard()).expect("encode endorsements failed");
    let (decoded, _): (Vec<PolicyEndorsement>, usize) =
        decode_owned_from_slice(&enc, config::standard()).expect("decode endorsements failed");
    assert_eq!(endorsements, decoded);
}

// ---------------------------------------------------------------------------
// Test 12: Deductible structure roundtrip
// ---------------------------------------------------------------------------

#[test]
fn test_deductible_structure_roundtrip() {
    let deductible = DeductibleStructure {
        policy_id: "HOME-2026-003321".to_string(),
        standard_deductible_cents: 250_000,
        wind_hail_deductible_bps: 200,
        hurricane_deductible_bps: 500,
        earthquake_deductible_bps: 1_000,
        flood_deductible_cents: 500_000,
        all_other_perils_cents: 100_000,
        aggregate_deductible_cents: 1_000_000,
    };
    let enc =
        encode_to_vec(&deductible, config::standard()).expect("encode deductible structure failed");
    let (decoded, _): (DeductibleStructure, usize) =
        decode_owned_from_slice(&enc, config::standard())
            .expect("decode deductible structure failed");
    assert_eq!(deductible, decoded);
}

// ---------------------------------------------------------------------------
// Test 13: Copay/coinsurance split roundtrip
// ---------------------------------------------------------------------------

#[test]
fn test_copay_coinsurance_split_roundtrip() {
    let plan = CopayCoinsuranceSplit {
        plan_id: "HEALTH-PPO-GOLD-2026".to_string(),
        primary_care_copay_cents: 2_500,
        specialist_copay_cents: 5_000,
        er_copay_cents: 25_000,
        coinsurance_in_network_bps: 8_000,
        coinsurance_out_network_bps: 6_000,
        out_of_pocket_max_cents: 600_000,
        family_out_of_pocket_max_cents: 1_200_000,
        pharmacy_tier1_copay_cents: 1_000,
        pharmacy_tier2_copay_cents: 4_000,
        pharmacy_tier3_copay_cents: 8_000,
    };
    let enc = encode_to_vec(&plan, config::standard()).expect("encode copay plan failed");
    let (decoded, _): (CopayCoinsuranceSplit, usize) =
        decode_owned_from_slice(&enc, config::standard()).expect("decode copay plan failed");
    assert_eq!(plan, decoded);
}

// ---------------------------------------------------------------------------
// Test 14: IBNR reserves roundtrip
// ---------------------------------------------------------------------------

#[test]
fn test_ibnr_reserves_roundtrip() {
    let reserves = vec![
        IbnrReserve {
            valuation_date: "2026-03-31".to_string(),
            line_of_business: "Workers Compensation".to_string(),
            accident_year: 2025,
            reported_claims_count: 1_234,
            ibnr_claims_count_estimate: 312,
            reported_losses_cents: 45_000_000_00,
            ibnr_losses_cents: 12_800_000_00,
            total_ultimate_cents: 57_800_000_00,
            development_method: "Chain Ladder".to_string(),
            confidence_level_bps: 7_500,
        },
        IbnrReserve {
            valuation_date: "2026-03-31".to_string(),
            line_of_business: "General Liability".to_string(),
            accident_year: 2024,
            reported_claims_count: 567,
            ibnr_claims_count_estimate: 89,
            reported_losses_cents: 28_000_000_00,
            ibnr_losses_cents: 5_600_000_00,
            total_ultimate_cents: 33_600_000_00,
            development_method: "Bornhuetter-Ferguson".to_string(),
            confidence_level_bps: 8_000,
        },
    ];
    let enc = encode_to_vec(&reserves, config::standard()).expect("encode IBNR reserves failed");
    let (decoded, _): (Vec<IbnrReserve>, usize) =
        decode_owned_from_slice(&enc, config::standard()).expect("decode IBNR reserves failed");
    assert_eq!(reserves, decoded);
}

// ---------------------------------------------------------------------------
// Test 15: Experience rating factors roundtrip
// ---------------------------------------------------------------------------

#[test]
fn test_experience_rating_roundtrip() {
    let rating = ExperienceRating {
        employer_id: "EMP-2026-44556".to_string(),
        rating_period: "2024-2025".to_string(),
        expected_losses_cents: 320_000_00,
        actual_losses_cents: 195_000_00,
        experience_mod_factor_bps: 8_200,
        credibility_factor_bps: 4_500,
        ballast_value_cents: 150_000_00,
        manual_premium_cents: 480_000_00,
        modified_premium_cents: 393_600_00,
    };
    let enc = encode_to_vec(&rating, config::standard()).expect("encode experience rating failed");
    let (decoded, _): (ExperienceRating, usize) =
        decode_owned_from_slice(&enc, config::standard()).expect("decode experience rating failed");
    assert_eq!(rating, decoded);
}

// ---------------------------------------------------------------------------
// Test 16: Fraud detection score roundtrip
// ---------------------------------------------------------------------------

#[test]
fn test_fraud_detection_score_roundtrip() {
    let fraud_score = FraudDetectionScore {
        claim_id: "CLM-2026-998877".to_string(),
        fraud_probability_bps: 7_800,
        anomaly_score: 923,
        red_flags: vec![
            "claim_filed_within_60_days_of_policy_inception".to_string(),
            "prior_denied_claims_at_other_carrier".to_string(),
            "loss_amount_just_below_siu_threshold".to_string(),
            "inconsistent_witness_statements".to_string(),
        ],
        network_link_count: 3,
        text_analytics_score: 870,
        geographic_anomaly: true,
        velocity_trigger: false,
        referral_recommended: true,
    };
    let enc = encode_to_vec(&fraud_score, config::standard()).expect("encode fraud score failed");
    let (decoded, _): (FraudDetectionScore, usize) =
        decode_owned_from_slice(&enc, config::standard()).expect("decode fraud score failed");
    assert_eq!(fraud_score, decoded);
}

// ---------------------------------------------------------------------------
// Test 17: Solvency II capital requirements roundtrip
// ---------------------------------------------------------------------------

#[test]
fn test_solvency_ii_capital_roundtrip() {
    let capital = SolvencyIiCapital {
        entity_name: "Tokio Marine Holdings".to_string(),
        reporting_date: "2025-12-31".to_string(),
        scr_market_risk_cents: 800_000_000_000,
        scr_counterparty_risk_cents: 120_000_000_000,
        scr_underwriting_life_cents: 350_000_000_000,
        scr_underwriting_health_cents: 180_000_000_000,
        scr_underwriting_nonlife_cents: 420_000_000_000,
        scr_operational_risk_cents: 95_000_000_000,
        diversification_benefit_cents: 450_000_000_000,
        total_scr_cents: 1_515_000_000_000,
        eligible_own_funds_cents: 3_200_000_000_000,
        solvency_ratio_bps: 21_122,
    };
    let enc =
        encode_to_vec(&capital, config::standard()).expect("encode solvency II capital failed");
    let (decoded, _): (SolvencyIiCapital, usize) =
        decode_owned_from_slice(&enc, config::standard())
            .expect("decode solvency II capital failed");
    assert_eq!(capital, decoded);
}

// ---------------------------------------------------------------------------
// Test 18: Policy portfolio roundtrip
// ---------------------------------------------------------------------------

#[test]
fn test_policy_portfolio_roundtrip() {
    let portfolio = PolicyPortfolio {
        portfolio_id: "PF-COMMERCIAL-2026".to_string(),
        policies: vec![
            PolicyRecord {
                policy_id: "CGL-2026-001".to_string(),
                policy_type: PolicyType::CommercialProperty,
                holder_name: "Tokyo Steel Corp".to_string(),
                effective_date: "2026-01-01".to_string(),
                expiry_date: "2027-01-01".to_string(),
                premium_annual_cents: 5_400_000,
                coverage_limit_cents: 500_000_000,
                is_active: true,
            },
            PolicyRecord {
                policy_id: "PL-2026-001".to_string(),
                policy_type: PolicyType::ProfessionalLiability,
                holder_name: "Osaka Legal Partners".to_string(),
                effective_date: "2026-03-01".to_string(),
                expiry_date: "2027-03-01".to_string(),
                premium_annual_cents: 1_200_000,
                coverage_limit_cents: 100_000_000,
                is_active: true,
            },
        ],
        total_premium_cents: 6_600_000,
        total_exposure_cents: 600_000_000,
        average_retention_months: 42,
    };
    let enc = encode_to_vec(&portfolio, config::standard()).expect("encode portfolio failed");
    let (decoded, _): (PolicyPortfolio, usize) =
        decode_owned_from_slice(&enc, config::standard()).expect("decode portfolio failed");
    assert_eq!(portfolio, decoded);
}

// ---------------------------------------------------------------------------
// Test 19: Actuarial assumptions roundtrip
// ---------------------------------------------------------------------------

#[test]
fn test_actuarial_assumptions_roundtrip() {
    let assumptions = vec![
        ActuarialAssumption {
            assumption_name: "Medical trend rate".to_string(),
            base_value_bps: 700,
            trend_bps: -25,
            volatility_bps: 150,
            confidence_interval_low_bps: 550,
            confidence_interval_high_bps: 850,
            source: "SOA Health Cost Trend Survey 2025".to_string(),
        },
        ActuarialAssumption {
            assumption_name: "Lapse rate - Term Life".to_string(),
            base_value_bps: 400,
            trend_bps: 10,
            volatility_bps: 80,
            confidence_interval_low_bps: 320,
            confidence_interval_high_bps: 480,
            source: "Company experience study 2023-2025".to_string(),
        },
        ActuarialAssumption {
            assumption_name: "Investment yield".to_string(),
            base_value_bps: 425,
            trend_bps: -15,
            volatility_bps: 100,
            confidence_interval_low_bps: 325,
            confidence_interval_high_bps: 525,
            source: "Internal asset-liability model".to_string(),
        },
    ];
    let enc = encode_to_vec(&assumptions, config::standard()).expect("encode assumptions failed");
    let (decoded, _): (Vec<ActuarialAssumption>, usize) =
        decode_owned_from_slice(&enc, config::standard()).expect("decode assumptions failed");
    assert_eq!(assumptions, decoded);
}

// ---------------------------------------------------------------------------
// Test 20: Loss triangle roundtrip
// ---------------------------------------------------------------------------

#[test]
fn test_loss_triangle_roundtrip() {
    let triangle = LossTriangle {
        line_of_business: "Personal Auto Liability".to_string(),
        evaluation_date: "2026-03-31".to_string(),
        rows: vec![
            LossTriangleRow {
                accident_year: 2022,
                development_months: vec![12, 24, 36, 48],
                cumulative_paid_cents: vec![
                    10_000_000_00,
                    22_000_000_00,
                    28_000_000_00,
                    30_500_000_00,
                ],
                cumulative_incurred_cents: vec![
                    35_000_000_00,
                    33_000_000_00,
                    31_000_000_00,
                    30_800_000_00,
                ],
            },
            LossTriangleRow {
                accident_year: 2023,
                development_months: vec![12, 24, 36],
                cumulative_paid_cents: vec![11_500_000_00, 24_200_000_00, 30_100_000_00],
                cumulative_incurred_cents: vec![38_000_000_00, 35_500_000_00, 33_200_000_00],
            },
            LossTriangleRow {
                accident_year: 2024,
                development_months: vec![12, 24],
                cumulative_paid_cents: vec![12_800_000_00, 26_500_000_00],
                cumulative_incurred_cents: vec![40_000_000_00, 37_200_000_00],
            },
            LossTriangleRow {
                accident_year: 2025,
                development_months: vec![12],
                cumulative_paid_cents: vec![13_200_000_00],
                cumulative_incurred_cents: vec![42_000_000_00],
            },
        ],
    };
    let enc = encode_to_vec(&triangle, config::standard()).expect("encode loss triangle failed");
    let (decoded, _): (LossTriangle, usize) =
        decode_owned_from_slice(&enc, config::standard()).expect("decode loss triangle failed");
    assert_eq!(triangle, decoded);
}

// ---------------------------------------------------------------------------
// Test 21: Regulatory filing with HashMap roundtrip
// ---------------------------------------------------------------------------

#[test]
fn test_regulatory_filing_roundtrip() {
    let mut risk_factors = HashMap::new();
    risk_factors.insert("interest_rate_sensitivity_bps".to_string(), 350_u32);
    risk_factors.insert("equity_market_exposure_bps".to_string(), 1_200);
    risk_factors.insert("currency_risk_bps".to_string(), 280);
    risk_factors.insert("credit_spread_risk_bps".to_string(), 450);
    risk_factors.insert("longevity_risk_bps".to_string(), 620);

    let filing = RegulatoryFiling {
        filing_id: "REG-2026-SOLVII-Q1".to_string(),
        jurisdiction: "EU - EIOPA".to_string(),
        filing_type: "Quantitative Reporting Template".to_string(),
        status: "Submitted".to_string(),
        capital_requirements: SolvencyIiCapital {
            entity_name: "Pan-European Insurance SE".to_string(),
            reporting_date: "2026-03-31".to_string(),
            scr_market_risk_cents: 450_000_000_000,
            scr_counterparty_risk_cents: 75_000_000_000,
            scr_underwriting_life_cents: 220_000_000_000,
            scr_underwriting_health_cents: 95_000_000_000,
            scr_underwriting_nonlife_cents: 310_000_000_000,
            scr_operational_risk_cents: 58_000_000_000,
            diversification_benefit_cents: 280_000_000_000,
            total_scr_cents: 928_000_000_000,
            eligible_own_funds_cents: 1_850_000_000_000,
            solvency_ratio_bps: 19_935,
        },
        risk_factors,
    };
    let enc = encode_to_vec(&filing, config::standard()).expect("encode regulatory filing failed");
    let (decoded, _): (RegulatoryFiling, usize) =
        decode_owned_from_slice(&enc, config::standard()).expect("decode regulatory filing failed");
    assert_eq!(filing, decoded);
}

// ---------------------------------------------------------------------------
// Test 22: Claim status enum variants exhaustive roundtrip
// ---------------------------------------------------------------------------

#[test]
fn test_claim_status_all_variants_roundtrip() {
    let statuses = vec![
        ClaimStatus::Filed,
        ClaimStatus::UnderReview,
        ClaimStatus::Approved,
        ClaimStatus::Denied,
        ClaimStatus::InLitigation,
        ClaimStatus::Settled,
        ClaimStatus::Closed,
    ];
    let enc = encode_to_vec(&statuses, config::standard()).expect("encode claim statuses failed");
    let (decoded, _): (Vec<ClaimStatus>, usize) =
        decode_owned_from_slice(&enc, config::standard()).expect("decode claim statuses failed");
    assert_eq!(statuses, decoded);
}
