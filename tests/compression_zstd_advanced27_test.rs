//! Advanced Zstd compression tests for OxiCode — Insurance Underwriting &
//! Actuarial Analysis domain.
//!
//! Covers encode → compress → decompress → decode round-trips for types that
//! model real-world insurance data: policy underwriting criteria, claims
//! processing workflows, actuarial mortality/morbidity tables, premium
//! calculation factors, reinsurance treaty structures, loss reserve triangles,
//! catastrophe model outputs, policyholder demographics, fraud detection scores,
//! IBNR estimates, Solvency II capital requirements, and more.

#![cfg(feature = "compression-zstd")]
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
use oxicode::compression::{compress, decompress, Compression};
use oxicode::{decode_from_slice, encode_to_vec, Decode, Encode};

// ---------------------------------------------------------------------------
// Domain types
// ---------------------------------------------------------------------------

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
enum RiskClass {
    Preferred,
    Standard,
    Substandard,
    Declined,
    Deferred,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
enum ClaimType {
    Death,
    TotalPermanentDisability,
    CriticalIllness,
    Hospitalization,
    Accident,
    PropertyDamage,
    Liability,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
enum ClaimDecision {
    Approved { payout_cents: u64 },
    PartialApproval { payout_cents: u64, reason: String },
    Denied { reason: String },
    UnderInvestigation,
    Referred { to_department: String },
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
enum ReinsuranceType {
    QuotaShare,
    SurplusShare,
    ExcessOfLoss,
    StopLoss,
    CatastropheXol,
    FacultativeProportional,
    FacultativeNonProportional,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
enum CatastrophePeril {
    Hurricane,
    Earthquake,
    Flood,
    Wildfire,
    Tornado,
    Hailstorm,
    Tsunami,
    VolcanicEruption,
    WinterStorm,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
enum Gender {
    Male,
    Female,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
enum SmokingStatus {
    NonSmoker,
    Smoker,
    FormerSmokerOver12Months,
    FormerSmokerUnder12Months,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
enum SolvencyIiRiskModule {
    MarketRisk,
    CounterpartyDefault,
    LifeUnderwriting,
    HealthUnderwriting,
    NonLifeUnderwriting,
    OperationalRisk,
    IntangibleAsset,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
enum FraudIndicator {
    Clean,
    LowSuspicion,
    ModerateSuspicion,
    HighSuspicion,
    Confirmed,
}

// ---------------------------------------------------------------------------
// Structs
// ---------------------------------------------------------------------------

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct UnderwritingCriteria {
    policy_id: u64,
    applicant_age: u8,
    gender: Gender,
    smoking_status: SmokingStatus,
    bmi_x100: u32,
    risk_class: RiskClass,
    medical_conditions: Vec<String>,
    family_history_flags: Vec<String>,
    occupation_code: u16,
    hazardous_activities: Vec<String>,
    sum_assured_cents: u64,
    loading_bps: u32,
    exclusions: Vec<String>,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct ClaimsWorkflowStep {
    step_order: u8,
    step_name: String,
    responsible_department: String,
    sla_hours: u32,
    is_automated: bool,
    required_documents: Vec<String>,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct ClaimsWorkflow {
    claim_id: u64,
    claim_type: ClaimType,
    steps: Vec<ClaimsWorkflowStep>,
    decision: ClaimDecision,
    total_elapsed_hours: u32,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct MortalityTableRow {
    age: u8,
    qx_per_million: u32,
    lx: u64,
    dx: u64,
    ex_x100: u32,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct MortalityTable {
    table_name: String,
    gender: Gender,
    smoking_status: SmokingStatus,
    base_year: u16,
    rows: Vec<MortalityTableRow>,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct MorbidityEntry {
    condition_code: String,
    incidence_per_100k: u32,
    average_duration_days: u32,
    disability_weight_bps: u16,
    age_band_start: u8,
    age_band_end: u8,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct PremiumFactor {
    factor_name: String,
    base_rate_bps: u32,
    age_adjustment_bps: i32,
    gender_adjustment_bps: i32,
    smoking_adjustment_bps: i32,
    occupation_adjustment_bps: i32,
    territory_adjustment_bps: i32,
    volume_discount_bps: i32,
    final_rate_bps: u32,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct ReinsuranceTreaty {
    treaty_id: u64,
    cedant_name: String,
    reinsurer_name: String,
    treaty_type: ReinsuranceType,
    retention_cents: u64,
    limit_cents: u64,
    cession_pct_bps: u32,
    commission_pct_bps: u32,
    inception_year: u16,
    expiry_year: u16,
    covered_lines: Vec<String>,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct LossTriangleCell {
    origin_year: u16,
    development_year: u16,
    cumulative_paid_cents: u64,
    cumulative_incurred_cents: u64,
    case_reserves_cents: u64,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct LossReserveTriangle {
    line_of_business: String,
    currency: String,
    cells: Vec<LossTriangleCell>,
    selected_ult_cents: u64,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct CatastropheModelOutput {
    model_name: String,
    peril: CatastrophePeril,
    region_code: String,
    return_period_years: u32,
    gross_loss_cents: u64,
    net_loss_cents: u64,
    insured_loss_cents: u64,
    event_count: u32,
    affected_policies: u32,
    average_loss_per_policy_cents: u64,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct PolicyholderDemographics {
    policyholder_id: u64,
    age: u8,
    gender: Gender,
    marital_status: String,
    dependents: u8,
    annual_income_cents: u64,
    occupation_category: String,
    region: String,
    risk_score: u16,
    tenure_months: u32,
    policies_held: Vec<String>,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct FraudDetectionResult {
    claim_id: u64,
    overall_score: u16,
    indicator: FraudIndicator,
    rule_triggers: Vec<String>,
    network_anomaly_score: u16,
    velocity_score: u16,
    geographic_anomaly: bool,
    duplicate_claim_flag: bool,
    provider_risk_score: u16,
    recommendation: String,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct IbnrEstimate {
    valuation_date_yyyymmdd: u32,
    line_of_business: String,
    method_name: String,
    paid_to_date_cents: u64,
    case_reserves_cents: u64,
    ibnr_cents: u64,
    ultimate_loss_cents: u64,
    development_factor: u32,
    confidence_level_bps: u16,
    low_estimate_cents: u64,
    high_estimate_cents: u64,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct SolvencyIiCapital {
    reporting_date_yyyymmdd: u32,
    risk_module: SolvencyIiRiskModule,
    gross_scr_cents: u64,
    diversification_benefit_cents: u64,
    net_scr_cents: u64,
    loss_absorbing_capacity_cents: u64,
    eligible_own_funds_cents: u64,
    solvency_ratio_bps: u32,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct ExperienceStudy {
    study_name: String,
    observation_period_start: u32,
    observation_period_end: u32,
    age_band_start: u8,
    age_band_end: u8,
    gender: Gender,
    expected_claims: u32,
    actual_claims: u32,
    ae_ratio_bps: u32,
    credibility_factor_bps: u16,
    exposures: u64,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct CommutationValuation {
    treaty_id: u64,
    valuation_date_yyyymmdd: u32,
    outstanding_reserves_cents: u64,
    discount_rate_bps: u32,
    present_value_cents: u64,
    risk_margin_cents: u64,
    settlement_amount_cents: u64,
    currency: String,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct PolicyLapsePrediction {
    policy_id: u64,
    months_in_force: u32,
    premium_frequency: String,
    payment_history_flags: Vec<bool>,
    lapse_probability_bps: u32,
    persistency_bonus_eligible: bool,
    surrender_value_cents: u64,
    competing_offer_detected: bool,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct AggregateExposure {
    territory_code: String,
    line_of_business: String,
    total_sum_insured_cents: u64,
    policy_count: u32,
    avg_sum_insured_cents: u64,
    max_single_risk_cents: u64,
    pml_250yr_cents: u64,
    pml_500yr_cents: u64,
    peril_breakdown: Vec<(String, u64)>,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct PricingAssumption {
    product_code: String,
    assumption_type: String,
    best_estimate_bps: u32,
    prudential_margin_bps: u32,
    final_assumption_bps: u32,
    source: String,
    review_date_yyyymmdd: u32,
    sensitivity_low_bps: u32,
    sensitivity_high_bps: u32,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct PooledRiskSegment {
    segment_id: u32,
    description: String,
    member_count: u32,
    total_premium_cents: u64,
    total_claims_cents: u64,
    loss_ratio_bps: u32,
    expense_ratio_bps: u32,
    combined_ratio_bps: u32,
    trend_factor_bps: u32,
    credibility_bps: u16,
}

// ---------------------------------------------------------------------------
// Tests (22 total)
// ---------------------------------------------------------------------------

/// 1. Underwriting criteria for a life insurance applicant.
#[test]
fn test_zstd_underwriting_criteria_roundtrip() {
    let val = UnderwritingCriteria {
        policy_id: 100_001,
        applicant_age: 42,
        gender: Gender::Male,
        smoking_status: SmokingStatus::FormerSmokerOver12Months,
        bmi_x100: 2650,
        risk_class: RiskClass::Standard,
        medical_conditions: vec!["Hypertension (controlled)".into(), "Mild asthma".into()],
        family_history_flags: vec!["Father – MI at age 58".into()],
        occupation_code: 312,
        hazardous_activities: vec!["Recreational scuba diving".into()],
        sum_assured_cents: 500_000_00,
        loading_bps: 75,
        exclusions: vec!["Aviation exclusion (private pilot)".into()],
    };
    let encoded = encode_to_vec(&val).expect("encode UnderwritingCriteria");
    let compressed = compress(&encoded, Compression::Zstd).expect("compress UnderwritingCriteria");
    let decompressed = decompress(&compressed).expect("decompress UnderwritingCriteria");
    let (decoded, _): (UnderwritingCriteria, usize) =
        decode_from_slice(&decompressed).expect("decode UnderwritingCriteria");
    assert_eq!(val, decoded);
}

/// 2. Claims processing workflow with multiple steps.
#[test]
fn test_zstd_claims_workflow_roundtrip() {
    let val = ClaimsWorkflow {
        claim_id: 77_042,
        claim_type: ClaimType::CriticalIllness,
        steps: vec![
            ClaimsWorkflowStep {
                step_order: 1,
                step_name: "Initial Notification".into(),
                responsible_department: "Call Centre".into(),
                sla_hours: 2,
                is_automated: true,
                required_documents: vec!["Claim form".into()],
            },
            ClaimsWorkflowStep {
                step_order: 2,
                step_name: "Medical Evidence Review".into(),
                responsible_department: "Medical Underwriting".into(),
                sla_hours: 72,
                is_automated: false,
                required_documents: vec![
                    "Attending physician statement".into(),
                    "Pathology report".into(),
                ],
            },
            ClaimsWorkflowStep {
                step_order: 3,
                step_name: "Benefit Adjudication".into(),
                responsible_department: "Claims Adjudication".into(),
                sla_hours: 48,
                is_automated: false,
                required_documents: vec![],
            },
        ],
        decision: ClaimDecision::Approved {
            payout_cents: 250_000_00,
        },
        total_elapsed_hours: 96,
    };
    let encoded = encode_to_vec(&val).expect("encode ClaimsWorkflow");
    let compressed = compress(&encoded, Compression::Zstd).expect("compress ClaimsWorkflow");
    let decompressed = decompress(&compressed).expect("decompress ClaimsWorkflow");
    let (decoded, _): (ClaimsWorkflow, usize) =
        decode_from_slice(&decompressed).expect("decode ClaimsWorkflow");
    assert_eq!(val, decoded);
}

/// 3. Actuarial mortality table (abbreviated).
#[test]
fn test_zstd_mortality_table_roundtrip() {
    let rows: Vec<MortalityTableRow> = (20u8..=80)
        .map(|age| {
            let qx = match age {
                20..=29 => 400 + u32::from(age) * 5,
                30..=49 => 600 + u32::from(age) * 15,
                50..=69 => 2_000 + u32::from(age) * 80,
                _ => 8_000 + u32::from(age) * 300,
            };
            MortalityTableRow {
                age,
                qx_per_million: qx,
                lx: 1_000_000 - u64::from(age) * 5_000,
                dx: u64::from(qx),
                ex_x100: (8500 - u32::from(age) * 80).max(200),
            }
        })
        .collect();
    let val = MortalityTable {
        table_name: "CSO2017-ANB".into(),
        gender: Gender::Female,
        smoking_status: SmokingStatus::NonSmoker,
        base_year: 2017,
        rows,
    };
    let encoded = encode_to_vec(&val).expect("encode MortalityTable");
    let compressed = compress(&encoded, Compression::Zstd).expect("compress MortalityTable");
    let decompressed = decompress(&compressed).expect("decompress MortalityTable");
    let (decoded, _): (MortalityTable, usize) =
        decode_from_slice(&decompressed).expect("decode MortalityTable");
    assert_eq!(val, decoded);
}

/// 4. Morbidity entries for various conditions.
#[test]
fn test_zstd_morbidity_entries_roundtrip() {
    let val: Vec<MorbidityEntry> = vec![
        MorbidityEntry {
            condition_code: "I25.1".into(),
            incidence_per_100k: 320,
            average_duration_days: 90,
            disability_weight_bps: 1500,
            age_band_start: 50,
            age_band_end: 59,
        },
        MorbidityEntry {
            condition_code: "C50.9".into(),
            incidence_per_100k: 125,
            average_duration_days: 365,
            disability_weight_bps: 4200,
            age_band_start: 40,
            age_band_end: 49,
        },
        MorbidityEntry {
            condition_code: "M54.5".into(),
            incidence_per_100k: 2800,
            average_duration_days: 21,
            disability_weight_bps: 800,
            age_band_start: 30,
            age_band_end: 39,
        },
    ];
    let encoded = encode_to_vec(&val).expect("encode Vec<MorbidityEntry>");
    let compressed = compress(&encoded, Compression::Zstd).expect("compress Vec<MorbidityEntry>");
    let decompressed = decompress(&compressed).expect("decompress Vec<MorbidityEntry>");
    let (decoded, _): (Vec<MorbidityEntry>, usize) =
        decode_from_slice(&decompressed).expect("decode Vec<MorbidityEntry>");
    assert_eq!(val, decoded);
}

/// 5. Premium calculation factors for a term life product.
#[test]
fn test_zstd_premium_factor_roundtrip() {
    let val = PremiumFactor {
        factor_name: "Term Life 20yr Male NS Age 35".into(),
        base_rate_bps: 120,
        age_adjustment_bps: 25,
        gender_adjustment_bps: 15,
        smoking_adjustment_bps: 0,
        occupation_adjustment_bps: -5,
        territory_adjustment_bps: 10,
        volume_discount_bps: -8,
        final_rate_bps: 157,
    };
    let encoded = encode_to_vec(&val).expect("encode PremiumFactor");
    let compressed = compress(&encoded, Compression::Zstd).expect("compress PremiumFactor");
    let decompressed = decompress(&compressed).expect("decompress PremiumFactor");
    let (decoded, _): (PremiumFactor, usize) =
        decode_from_slice(&decompressed).expect("decode PremiumFactor");
    assert_eq!(val, decoded);
}

/// 6. Reinsurance treaty — quota share arrangement.
#[test]
fn test_zstd_reinsurance_treaty_quota_share_roundtrip() {
    let val = ReinsuranceTreaty {
        treaty_id: 2024_001,
        cedant_name: "Acme Life Insurance Co.".into(),
        reinsurer_name: "Global Reinsurance Ltd.".into(),
        treaty_type: ReinsuranceType::QuotaShare,
        retention_cents: 0,
        limit_cents: 10_000_000_00,
        cession_pct_bps: 4000,
        commission_pct_bps: 3200,
        inception_year: 2024,
        expiry_year: 2027,
        covered_lines: vec![
            "Term Life".into(),
            "Whole Life".into(),
            "Critical Illness".into(),
        ],
    };
    let encoded = encode_to_vec(&val).expect("encode ReinsuranceTreaty QS");
    let compressed = compress(&encoded, Compression::Zstd).expect("compress ReinsuranceTreaty QS");
    let decompressed = decompress(&compressed).expect("decompress ReinsuranceTreaty QS");
    let (decoded, _): (ReinsuranceTreaty, usize) =
        decode_from_slice(&decompressed).expect("decode ReinsuranceTreaty QS");
    assert_eq!(val, decoded);
}

/// 7. Reinsurance treaty — excess of loss (cat XOL).
#[test]
fn test_zstd_reinsurance_treaty_cat_xol_roundtrip() {
    let val = ReinsuranceTreaty {
        treaty_id: 2024_015,
        cedant_name: "Pacific Property Insurers Inc.".into(),
        reinsurer_name: "Atlantic Re".into(),
        treaty_type: ReinsuranceType::CatastropheXol,
        retention_cents: 50_000_000_00,
        limit_cents: 200_000_000_00,
        cession_pct_bps: 10000,
        commission_pct_bps: 0,
        inception_year: 2025,
        expiry_year: 2026,
        covered_lines: vec!["Commercial Property".into(), "Homeowners".into()],
    };
    let encoded = encode_to_vec(&val).expect("encode ReinsuranceTreaty CatXOL");
    let compressed =
        compress(&encoded, Compression::Zstd).expect("compress ReinsuranceTreaty CatXOL");
    let decompressed = decompress(&compressed).expect("decompress ReinsuranceTreaty CatXOL");
    let (decoded, _): (ReinsuranceTreaty, usize) =
        decode_from_slice(&decompressed).expect("decode ReinsuranceTreaty CatXOL");
    assert_eq!(val, decoded);
}

/// 8. Loss reserve triangle with 10 origin years × multiple dev years.
#[test]
fn test_zstd_loss_reserve_triangle_roundtrip() {
    let cells: Vec<LossTriangleCell> = (2015u16..=2024)
        .flat_map(|origin| {
            (0u16..=(2024 - origin)).map(move |dev| LossTriangleCell {
                origin_year: origin,
                development_year: dev,
                cumulative_paid_cents: u64::from(origin - 2014) * 1_000_000 * u64::from(dev + 1),
                cumulative_incurred_cents: u64::from(origin - 2014)
                    * 1_200_000
                    * u64::from(dev + 1),
                case_reserves_cents: u64::from(origin - 2014) * 200_000 / u64::from(dev + 1),
            })
        })
        .collect();
    let val = LossReserveTriangle {
        line_of_business: "Workers Compensation".into(),
        currency: "USD".into(),
        cells,
        selected_ult_cents: 85_000_000_00,
    };
    let encoded = encode_to_vec(&val).expect("encode LossReserveTriangle");
    let compressed = compress(&encoded, Compression::Zstd).expect("compress LossReserveTriangle");
    let decompressed = decompress(&compressed).expect("decompress LossReserveTriangle");
    let (decoded, _): (LossReserveTriangle, usize) =
        decode_from_slice(&decompressed).expect("decode LossReserveTriangle");
    assert_eq!(val, decoded);
}

/// 9. Catastrophe model output for a hurricane event.
#[test]
fn test_zstd_catastrophe_model_hurricane_roundtrip() {
    let val = CatastropheModelOutput {
        model_name: "AIR Touchstone v10".into(),
        peril: CatastrophePeril::Hurricane,
        region_code: "US-FL".into(),
        return_period_years: 250,
        gross_loss_cents: 5_000_000_000_00,
        net_loss_cents: 1_200_000_000_00,
        insured_loss_cents: 3_500_000_000_00,
        event_count: 1,
        affected_policies: 42_000,
        average_loss_per_policy_cents: 83_333_33,
    };
    let encoded = encode_to_vec(&val).expect("encode CatModelOutput hurricane");
    let compressed =
        compress(&encoded, Compression::Zstd).expect("compress CatModelOutput hurricane");
    let decompressed = decompress(&compressed).expect("decompress CatModelOutput hurricane");
    let (decoded, _): (CatastropheModelOutput, usize) =
        decode_from_slice(&decompressed).expect("decode CatModelOutput hurricane");
    assert_eq!(val, decoded);
}

/// 10. Catastrophe model output for an earthquake scenario.
#[test]
fn test_zstd_catastrophe_model_earthquake_roundtrip() {
    let val = CatastropheModelOutput {
        model_name: "RMS RiskLink v21".into(),
        peril: CatastrophePeril::Earthquake,
        region_code: "JP-KANTO".into(),
        return_period_years: 500,
        gross_loss_cents: 12_000_000_000_00,
        net_loss_cents: 3_000_000_000_00,
        insured_loss_cents: 8_000_000_000_00,
        event_count: 1,
        affected_policies: 180_000,
        average_loss_per_policy_cents: 44_444_44,
    };
    let encoded = encode_to_vec(&val).expect("encode CatModelOutput earthquake");
    let compressed =
        compress(&encoded, Compression::Zstd).expect("compress CatModelOutput earthquake");
    let decompressed = decompress(&compressed).expect("decompress CatModelOutput earthquake");
    let (decoded, _): (CatastropheModelOutput, usize) =
        decode_from_slice(&decompressed).expect("decode CatModelOutput earthquake");
    assert_eq!(val, decoded);
}

/// 11. Policyholder demographics record.
#[test]
fn test_zstd_policyholder_demographics_roundtrip() {
    let val = PolicyholderDemographics {
        policyholder_id: 9_876_543,
        age: 37,
        gender: Gender::Female,
        marital_status: "Married".into(),
        dependents: 2,
        annual_income_cents: 85_000_00,
        occupation_category: "Professional — Engineering".into(),
        region: "US-CA-90210".into(),
        risk_score: 720,
        tenure_months: 48,
        policies_held: vec![
            "TL-2020-1234".into(),
            "CI-2021-5678".into(),
            "HO-2019-9012".into(),
        ],
    };
    let encoded = encode_to_vec(&val).expect("encode PolicyholderDemographics");
    let compressed =
        compress(&encoded, Compression::Zstd).expect("compress PolicyholderDemographics");
    let decompressed = decompress(&compressed).expect("decompress PolicyholderDemographics");
    let (decoded, _): (PolicyholderDemographics, usize) =
        decode_from_slice(&decompressed).expect("decode PolicyholderDemographics");
    assert_eq!(val, decoded);
}

/// 12. Fraud detection result — high suspicion.
#[test]
fn test_zstd_fraud_detection_high_suspicion_roundtrip() {
    let val = FraudDetectionResult {
        claim_id: 55_001,
        overall_score: 870,
        indicator: FraudIndicator::HighSuspicion,
        rule_triggers: vec![
            "Policy inception < 90 days".into(),
            "Multiple prior claims at same address".into(),
            "Provider on watchlist".into(),
        ],
        network_anomaly_score: 920,
        velocity_score: 780,
        geographic_anomaly: true,
        duplicate_claim_flag: false,
        provider_risk_score: 850,
        recommendation: "Refer to Special Investigations Unit".into(),
    };
    let encoded = encode_to_vec(&val).expect("encode FraudDetectionResult high");
    let compressed =
        compress(&encoded, Compression::Zstd).expect("compress FraudDetectionResult high");
    let decompressed = decompress(&compressed).expect("decompress FraudDetectionResult high");
    let (decoded, _): (FraudDetectionResult, usize) =
        decode_from_slice(&decompressed).expect("decode FraudDetectionResult high");
    assert_eq!(val, decoded);
}

/// 13. Fraud detection result — clean.
#[test]
fn test_zstd_fraud_detection_clean_roundtrip() {
    let val = FraudDetectionResult {
        claim_id: 55_200,
        overall_score: 120,
        indicator: FraudIndicator::Clean,
        rule_triggers: vec![],
        network_anomaly_score: 80,
        velocity_score: 50,
        geographic_anomaly: false,
        duplicate_claim_flag: false,
        provider_risk_score: 100,
        recommendation: "Auto-approve".into(),
    };
    let encoded = encode_to_vec(&val).expect("encode FraudDetectionResult clean");
    let compressed =
        compress(&encoded, Compression::Zstd).expect("compress FraudDetectionResult clean");
    let decompressed = decompress(&compressed).expect("decompress FraudDetectionResult clean");
    let (decoded, _): (FraudDetectionResult, usize) =
        decode_from_slice(&decompressed).expect("decode FraudDetectionResult clean");
    assert_eq!(val, decoded);
}

/// 14. IBNR estimate using Bornhuetter-Ferguson method.
#[test]
fn test_zstd_ibnr_estimate_roundtrip() {
    let val = IbnrEstimate {
        valuation_date_yyyymmdd: 20251231,
        line_of_business: "General Liability".into(),
        method_name: "Bornhuetter-Ferguson".into(),
        paid_to_date_cents: 45_000_000_00,
        case_reserves_cents: 12_000_000_00,
        ibnr_cents: 18_500_000_00,
        ultimate_loss_cents: 75_500_000_00,
        development_factor: 12_500,
        confidence_level_bps: 7500,
        low_estimate_cents: 14_000_000_00,
        high_estimate_cents: 24_000_000_00,
    };
    let encoded = encode_to_vec(&val).expect("encode IbnrEstimate");
    let compressed = compress(&encoded, Compression::Zstd).expect("compress IbnrEstimate");
    let decompressed = decompress(&compressed).expect("decompress IbnrEstimate");
    let (decoded, _): (IbnrEstimate, usize) =
        decode_from_slice(&decompressed).expect("decode IbnrEstimate");
    assert_eq!(val, decoded);
}

/// 15. Solvency II capital — market risk module.
#[test]
fn test_zstd_solvency_ii_market_risk_roundtrip() {
    let val = SolvencyIiCapital {
        reporting_date_yyyymmdd: 20251231,
        risk_module: SolvencyIiRiskModule::MarketRisk,
        gross_scr_cents: 320_000_000_00,
        diversification_benefit_cents: 48_000_000_00,
        net_scr_cents: 272_000_000_00,
        loss_absorbing_capacity_cents: 35_000_000_00,
        eligible_own_funds_cents: 600_000_000_00,
        solvency_ratio_bps: 22_060,
    };
    let encoded = encode_to_vec(&val).expect("encode SolvencyIiCapital market");
    let compressed =
        compress(&encoded, Compression::Zstd).expect("compress SolvencyIiCapital market");
    let decompressed = decompress(&compressed).expect("decompress SolvencyIiCapital market");
    let (decoded, _): (SolvencyIiCapital, usize) =
        decode_from_slice(&decompressed).expect("decode SolvencyIiCapital market");
    assert_eq!(val, decoded);
}

/// 16. Solvency II capital — life underwriting risk module.
#[test]
fn test_zstd_solvency_ii_life_uw_roundtrip() {
    let val = SolvencyIiCapital {
        reporting_date_yyyymmdd: 20251231,
        risk_module: SolvencyIiRiskModule::LifeUnderwriting,
        gross_scr_cents: 180_000_000_00,
        diversification_benefit_cents: 22_000_000_00,
        net_scr_cents: 158_000_000_00,
        loss_absorbing_capacity_cents: 20_000_000_00,
        eligible_own_funds_cents: 600_000_000_00,
        solvency_ratio_bps: 37_975,
    };
    let encoded = encode_to_vec(&val).expect("encode SolvencyIiCapital life");
    let compressed =
        compress(&encoded, Compression::Zstd).expect("compress SolvencyIiCapital life");
    let decompressed = decompress(&compressed).expect("decompress SolvencyIiCapital life");
    let (decoded, _): (SolvencyIiCapital, usize) =
        decode_from_slice(&decompressed).expect("decode SolvencyIiCapital life");
    assert_eq!(val, decoded);
}

/// 17. Experience study — actual vs expected mortality analysis.
#[test]
fn test_zstd_experience_study_roundtrip() {
    let val = ExperienceStudy {
        study_name: "2020-2024 Mortality Experience".into(),
        observation_period_start: 20200101,
        observation_period_end: 20241231,
        age_band_start: 45,
        age_band_end: 54,
        gender: Gender::Male,
        expected_claims: 1_250,
        actual_claims: 1_087,
        ae_ratio_bps: 8_696,
        credibility_factor_bps: 9_200,
        exposures: 312_000,
    };
    let encoded = encode_to_vec(&val).expect("encode ExperienceStudy");
    let compressed = compress(&encoded, Compression::Zstd).expect("compress ExperienceStudy");
    let decompressed = decompress(&compressed).expect("decompress ExperienceStudy");
    let (decoded, _): (ExperienceStudy, usize) =
        decode_from_slice(&decompressed).expect("decode ExperienceStudy");
    assert_eq!(val, decoded);
}

/// 18. Commutation valuation for a reinsurance treaty settlement.
#[test]
fn test_zstd_commutation_valuation_roundtrip() {
    let val = CommutationValuation {
        treaty_id: 2018_003,
        valuation_date_yyyymmdd: 20251001,
        outstanding_reserves_cents: 15_000_000_00,
        discount_rate_bps: 350,
        present_value_cents: 14_200_000_00,
        risk_margin_cents: 1_100_000_00,
        settlement_amount_cents: 15_300_000_00,
        currency: "EUR".into(),
    };
    let encoded = encode_to_vec(&val).expect("encode CommutationValuation");
    let compressed = compress(&encoded, Compression::Zstd).expect("compress CommutationValuation");
    let decompressed = decompress(&compressed).expect("decompress CommutationValuation");
    let (decoded, _): (CommutationValuation, usize) =
        decode_from_slice(&decompressed).expect("decode CommutationValuation");
    assert_eq!(val, decoded);
}

/// 19. Policy lapse prediction with payment history flags.
#[test]
fn test_zstd_policy_lapse_prediction_roundtrip() {
    let val = PolicyLapsePrediction {
        policy_id: 443_210,
        months_in_force: 36,
        premium_frequency: "Monthly".into(),
        payment_history_flags: vec![
            true, true, true, true, true, true, true, true, true, true, true, true, true, true,
            true, true, true, true, true, true, true, true, true, true, true, true, false, false,
            true, false, false, false, false, false, true, false,
        ],
        lapse_probability_bps: 6_800,
        persistency_bonus_eligible: false,
        surrender_value_cents: 3_200_00,
        competing_offer_detected: true,
    };
    let encoded = encode_to_vec(&val).expect("encode PolicyLapsePrediction");
    let compressed = compress(&encoded, Compression::Zstd).expect("compress PolicyLapsePrediction");
    let decompressed = decompress(&compressed).expect("decompress PolicyLapsePrediction");
    let (decoded, _): (PolicyLapsePrediction, usize) =
        decode_from_slice(&decompressed).expect("decode PolicyLapsePrediction");
    assert_eq!(val, decoded);
}

/// 20. Aggregate exposure summary with peril breakdown.
#[test]
fn test_zstd_aggregate_exposure_roundtrip() {
    let val = AggregateExposure {
        territory_code: "US-FL-MIAMI-DADE".into(),
        line_of_business: "Homeowners".into(),
        total_sum_insured_cents: 25_000_000_000_00,
        policy_count: 85_000,
        avg_sum_insured_cents: 294_117_65,
        max_single_risk_cents: 15_000_000_00,
        pml_250yr_cents: 4_500_000_000_00,
        pml_500yr_cents: 7_200_000_000_00,
        peril_breakdown: vec![
            ("Hurricane Wind".into(), 3_800_000_000_00),
            ("Storm Surge".into(), 1_500_000_000_00),
            ("Inland Flood".into(), 800_000_000_00),
            ("Tornado".into(), 200_000_000_00),
        ],
    };
    let encoded = encode_to_vec(&val).expect("encode AggregateExposure");
    let compressed = compress(&encoded, Compression::Zstd).expect("compress AggregateExposure");
    let decompressed = decompress(&compressed).expect("decompress AggregateExposure");
    let (decoded, _): (AggregateExposure, usize) =
        decode_from_slice(&decompressed).expect("decode AggregateExposure");
    assert_eq!(val, decoded);
}

/// 21. Pricing assumptions for a whole life product.
#[test]
fn test_zstd_pricing_assumptions_roundtrip() {
    let val: Vec<PricingAssumption> = vec![
        PricingAssumption {
            product_code: "WL-2025".into(),
            assumption_type: "Mortality".into(),
            best_estimate_bps: 85,
            prudential_margin_bps: 10,
            final_assumption_bps: 95,
            source: "2020-2024 Experience Study".into(),
            review_date_yyyymmdd: 20250601,
            sensitivity_low_bps: 75,
            sensitivity_high_bps: 110,
        },
        PricingAssumption {
            product_code: "WL-2025".into(),
            assumption_type: "Lapse".into(),
            best_estimate_bps: 500,
            prudential_margin_bps: 50,
            final_assumption_bps: 550,
            source: "Industry benchmark".into(),
            review_date_yyyymmdd: 20250601,
            sensitivity_low_bps: 400,
            sensitivity_high_bps: 700,
        },
        PricingAssumption {
            product_code: "WL-2025".into(),
            assumption_type: "Expense Inflation".into(),
            best_estimate_bps: 250,
            prudential_margin_bps: 25,
            final_assumption_bps: 275,
            source: "CPI forecast".into(),
            review_date_yyyymmdd: 20250601,
            sensitivity_low_bps: 200,
            sensitivity_high_bps: 350,
        },
    ];
    let encoded = encode_to_vec(&val).expect("encode Vec<PricingAssumption>");
    let compressed =
        compress(&encoded, Compression::Zstd).expect("compress Vec<PricingAssumption>");
    let decompressed = decompress(&compressed).expect("decompress Vec<PricingAssumption>");
    let (decoded, _): (Vec<PricingAssumption>, usize) =
        decode_from_slice(&decompressed).expect("decode Vec<PricingAssumption>");
    assert_eq!(val, decoded);
}

/// 22. Pooled risk segment — auto insurance combined ratio analysis.
#[test]
fn test_zstd_pooled_risk_segment_roundtrip() {
    let val: Vec<PooledRiskSegment> = vec![
        PooledRiskSegment {
            segment_id: 1,
            description: "Young drivers 18-25, urban".into(),
            member_count: 12_500,
            total_premium_cents: 45_000_000_00,
            total_claims_cents: 38_000_000_00,
            loss_ratio_bps: 8_444,
            expense_ratio_bps: 2_800,
            combined_ratio_bps: 11_244,
            trend_factor_bps: 10_350,
            credibility_bps: 8_500,
        },
        PooledRiskSegment {
            segment_id: 2,
            description: "Mid-age drivers 35-55, suburban".into(),
            member_count: 45_000,
            total_premium_cents: 120_000_000_00,
            total_claims_cents: 72_000_000_00,
            loss_ratio_bps: 6_000,
            expense_ratio_bps: 2_500,
            combined_ratio_bps: 8_500,
            trend_factor_bps: 10_150,
            credibility_bps: 9_800,
        },
        PooledRiskSegment {
            segment_id: 3,
            description: "Senior drivers 65+, rural".into(),
            member_count: 8_000,
            total_premium_cents: 32_000_000_00,
            total_claims_cents: 24_000_000_00,
            loss_ratio_bps: 7_500,
            expense_ratio_bps: 3_200,
            combined_ratio_bps: 10_700,
            trend_factor_bps: 10_250,
            credibility_bps: 7_200,
        },
    ];
    let encoded = encode_to_vec(&val).expect("encode Vec<PooledRiskSegment>");
    let compressed =
        compress(&encoded, Compression::Zstd).expect("compress Vec<PooledRiskSegment>");
    let decompressed = decompress(&compressed).expect("decompress Vec<PooledRiskSegment>");
    let (decoded, _): (Vec<PooledRiskSegment>, usize) =
        decode_from_slice(&decompressed).expect("decode Vec<PooledRiskSegment>");
    assert_eq!(val, decoded);
}
