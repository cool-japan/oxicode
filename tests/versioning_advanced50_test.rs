#![cfg(feature = "versioning")]

//! Versioning tests for OxiCode — Real Estate & PropTech domain.
//!
//! Covers property listings, comparable sales, mortgage amortization,
//! building inspections, rental yields, property tax assessments,
//! title searches, escrow accounts, zoning variances, HOA fees,
//! commercial leases, and property valuation models.

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
use oxicode::versioning::Version;
use oxicode::{decode_versioned_value, encode_versioned_value, Decode, Encode};

// ── Domain enums ────────────────────────────────────────────────────────────

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
enum PropertyType {
    SingleFamily,
    Condo,
    Townhouse,
    MultiFamily,
    Commercial,
    Industrial,
    MixedUse,
    Land,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
enum ListingStatus {
    Active,
    Pending,
    Sold,
    Withdrawn,
    Expired,
    ComingSoon,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
enum InspectionSeverity {
    Cosmetic,
    Minor,
    Moderate,
    Major,
    Critical,
    SafetyHazard,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
enum TitleDefectType {
    LienUnresolved,
    BoundaryDispute,
    EasementConflict,
    ForgedDocument,
    MissingHeir,
    UnpaidTaxes,
    None,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
enum EscrowState {
    Opened,
    EarnestMoneyDeposited,
    InspectionContingency,
    AppraisalContingency,
    LoanApproval,
    TitleCleared,
    FundsTransferred,
    Closed,
    Cancelled,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
enum ZoningClassification {
    ResidentialSingle,
    ResidentialMulti,
    CommercialRetail,
    CommercialOffice,
    Industrial,
    Agricultural,
    MixedUse,
    Overlay,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
enum LeaseType {
    TripleNet,
    GrossLease,
    ModifiedGross,
    PercentageLease,
    GroundLease,
    BondableLease,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
enum ValuationMethod {
    DiscountedCashFlow,
    CapRate,
    SalesComparison,
    CostApproach,
    GrossRentMultiplier,
    IncomeCapitalization,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
enum MortgageType {
    FixedRate,
    AdjustableRate,
    InterestOnly,
    Balloon,
    FhaInsured,
    VaGuaranteed,
    Jumbo,
}

// ── Domain structs ──────────────────────────────────────────────────────────

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct PropertyListing {
    listing_id: u64,
    mls_number: String,
    address: String,
    property_type: PropertyType,
    status: ListingStatus,
    list_price_cents: u64,
    bedrooms: u8,
    bathrooms_x10: u16,
    sqft: u32,
    lot_sqft: u32,
    year_built: u16,
    days_on_market: u16,
    hoa_monthly_cents: u32,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct ComparableSale {
    comp_id: u64,
    address: String,
    sale_price_cents: u64,
    sale_date_epoch: u64,
    sqft: u32,
    price_per_sqft_cents: u32,
    bedrooms: u8,
    bathrooms_x10: u16,
    distance_feet: u32,
    adjustment_cents: i64,
    adjusted_price_cents: u64,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct MortgageAmortizationEntry {
    payment_number: u32,
    principal_cents: u64,
    interest_cents: u64,
    total_payment_cents: u64,
    remaining_balance_cents: u64,
    cumulative_interest_cents: u64,
    cumulative_principal_cents: u64,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct MortgageSchedule {
    loan_id: u64,
    mortgage_type: MortgageType,
    original_balance_cents: u64,
    interest_rate_bps: u16,
    term_months: u16,
    entries: Vec<MortgageAmortizationEntry>,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct InspectionFinding {
    finding_id: u32,
    category: String,
    description: String,
    severity: InspectionSeverity,
    estimated_repair_cents: u64,
    photo_count: u8,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct BuildingInspectionReport {
    report_id: u64,
    property_address: String,
    inspector_name: String,
    inspection_date_epoch: u64,
    findings: Vec<InspectionFinding>,
    overall_condition_score: u8,
    recommended_reinspection: bool,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct RentalYieldAnalysis {
    property_id: u64,
    purchase_price_cents: u64,
    monthly_rent_cents: u64,
    vacancy_rate_bps: u16,
    operating_expense_ratio_bps: u16,
    gross_yield_bps: u16,
    net_yield_bps: u16,
    cap_rate_bps: u16,
    cash_on_cash_return_bps: u16,
    annual_appreciation_bps: u16,
    debt_service_coverage_x100: u16,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct PropertyTaxAssessment {
    parcel_id: String,
    tax_year: u16,
    land_value_cents: u64,
    improvement_value_cents: u64,
    total_assessed_cents: u64,
    exemptions_cents: u64,
    taxable_value_cents: u64,
    millage_rate_x1000: u32,
    annual_tax_cents: u64,
    appeal_deadline_epoch: u64,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct TitleSearchResult {
    search_id: u64,
    parcel_id: String,
    current_owner: String,
    chain_of_title_length: u16,
    defect: TitleDefectType,
    liens_outstanding_cents: u64,
    easements_count: u8,
    encumbrances: Vec<String>,
    title_insurance_premium_cents: u64,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct EscrowAccount {
    escrow_id: u64,
    buyer: String,
    seller: String,
    state: EscrowState,
    purchase_price_cents: u64,
    earnest_money_cents: u64,
    closing_costs_buyer_cents: u64,
    closing_costs_seller_cents: u64,
    prorated_taxes_cents: u64,
    remaining_balance_cents: u64,
    target_close_epoch: u64,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct ZoningVarianceApplication {
    application_id: u64,
    parcel_id: String,
    current_zoning: ZoningClassification,
    requested_zoning: ZoningClassification,
    applicant_name: String,
    proposed_use: String,
    setback_front_inches: u32,
    setback_side_inches: u32,
    max_height_inches: u32,
    lot_coverage_bps: u16,
    floor_area_ratio_x100: u16,
    public_hearing_epoch: u64,
    neighbors_notified: u16,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct HoaFeeItem {
    category: String,
    monthly_amount_cents: u32,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct HoaFeeStructure {
    hoa_id: u64,
    community_name: String,
    total_monthly_cents: u32,
    reserve_balance_cents: u64,
    special_assessment_cents: u64,
    fee_items: Vec<HoaFeeItem>,
    delinquency_rate_bps: u16,
    next_increase_epoch: u64,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct CommercialLeaseTerm {
    lease_id: u64,
    tenant_name: String,
    lease_type: LeaseType,
    sqft_leased: u32,
    base_rent_annual_cents: u64,
    cam_charges_annual_cents: u64,
    insurance_annual_cents: u64,
    tax_pass_through_annual_cents: u64,
    escalation_rate_bps: u16,
    lease_start_epoch: u64,
    lease_end_epoch: u64,
    renewal_options: u8,
    tenant_improvement_allowance_cents: u64,
    free_rent_months: u8,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct PropertyValuation {
    valuation_id: u64,
    property_address: String,
    method: ValuationMethod,
    estimated_value_cents: u64,
    discount_rate_bps: u16,
    terminal_cap_rate_bps: u16,
    noi_annual_cents: u64,
    holding_period_years: u8,
    confidence_score_bps: u16,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct ComparableSalesAnalysis {
    subject_address: String,
    subject_sqft: u32,
    comps: Vec<ComparableSale>,
    median_price_per_sqft_cents: u32,
    indicated_value_cents: u64,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct PropertyPortfolio {
    portfolio_id: u64,
    owner_name: String,
    total_units: u32,
    total_sqft: u64,
    aggregate_value_cents: u64,
    aggregate_noi_cents: u64,
    weighted_avg_cap_rate_bps: u16,
    leverage_ratio_bps: u16,
    property_ids: Vec<u64>,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct ConstructionDrawSchedule {
    draw_number: u16,
    milestone: String,
    budgeted_cents: u64,
    released_cents: u64,
    retainage_cents: u64,
    inspection_passed: bool,
    draw_date_epoch: u64,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct ConstructionLoan {
    loan_id: u64,
    project_name: String,
    total_commitment_cents: u64,
    interest_reserve_cents: u64,
    draws: Vec<ConstructionDrawSchedule>,
    maturity_epoch: u64,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct RentRollEntry {
    unit_id: String,
    tenant_name: String,
    sqft: u32,
    monthly_rent_cents: u32,
    lease_start_epoch: u64,
    lease_end_epoch: u64,
    security_deposit_cents: u32,
    is_occupied: bool,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct RentRoll {
    property_id: u64,
    property_address: String,
    entries: Vec<RentRollEntry>,
    total_monthly_income_cents: u64,
    occupancy_rate_bps: u16,
}

// ── Test 1: Property Listing Roundtrip ──────────────────────────────────────

#[test]
fn test_property_listing_single_family() {
    let listing = PropertyListing {
        listing_id: 100_001,
        mls_number: "MLS-2026-78432".into(),
        address: "742 Evergreen Terrace, Springfield, IL 62704".into(),
        property_type: PropertyType::SingleFamily,
        status: ListingStatus::Active,
        list_price_cents: 45_000_000,
        bedrooms: 4,
        bathrooms_x10: 25,
        sqft: 2_400,
        lot_sqft: 8_712,
        year_built: 1985,
        days_on_market: 12,
        hoa_monthly_cents: 0,
    };
    let ver = Version::new(1, 0, 0);
    let bytes = encode_versioned_value(&listing, ver).expect("encode property listing");
    let (decoded, version, consumed) =
        decode_versioned_value::<PropertyListing>(&bytes).expect("decode property listing");
    assert_eq!(decoded, listing);
    assert_eq!(version.major, 1);
    assert_eq!(version.minor, 0);
    assert_eq!(version.patch, 0);
    assert!(consumed > 0);
}

// ── Test 2: Comparable Sales Analysis ───────────────────────────────────────

#[test]
fn test_comparable_sales_analysis_with_adjustments() {
    let comps = vec![
        ComparableSale {
            comp_id: 1,
            address: "100 Oak St".into(),
            sale_price_cents: 42_000_000,
            sale_date_epoch: 1_700_000_000,
            sqft: 2_100,
            price_per_sqft_cents: 20_000,
            bedrooms: 3,
            bathrooms_x10: 20,
            distance_feet: 1_200,
            adjustment_cents: 1_500_000,
            adjusted_price_cents: 43_500_000,
        },
        ComparableSale {
            comp_id: 2,
            address: "205 Maple Ave".into(),
            sale_price_cents: 46_500_000,
            sale_date_epoch: 1_702_000_000,
            sqft: 2_500,
            price_per_sqft_cents: 18_600,
            bedrooms: 4,
            bathrooms_x10: 30,
            distance_feet: 2_400,
            adjustment_cents: -2_000_000,
            adjusted_price_cents: 44_500_000,
        },
    ];
    let analysis = ComparableSalesAnalysis {
        subject_address: "742 Evergreen Terrace".into(),
        subject_sqft: 2_400,
        comps,
        median_price_per_sqft_cents: 19_300,
        indicated_value_cents: 44_000_000,
    };
    let ver = Version::new(1, 2, 0);
    let bytes = encode_versioned_value(&analysis, ver).expect("encode comps analysis");
    let (decoded, version, consumed) =
        decode_versioned_value::<ComparableSalesAnalysis>(&bytes).expect("decode comps analysis");
    assert_eq!(decoded, analysis);
    assert_eq!(version.major, 1);
    assert_eq!(version.minor, 2);
    assert!(consumed > 0);
}

// ── Test 3: Mortgage Amortization Schedule ──────────────────────────────────

#[test]
fn test_mortgage_amortization_fixed_rate() {
    let entries = vec![
        MortgageAmortizationEntry {
            payment_number: 1,
            principal_cents: 45_678,
            interest_cents: 125_000,
            total_payment_cents: 170_678,
            remaining_balance_cents: 29_954_322,
            cumulative_interest_cents: 125_000,
            cumulative_principal_cents: 45_678,
        },
        MortgageAmortizationEntry {
            payment_number: 2,
            principal_cents: 45_868,
            interest_cents: 124_810,
            total_payment_cents: 170_678,
            remaining_balance_cents: 29_908_454,
            cumulative_interest_cents: 249_810,
            cumulative_principal_cents: 91_546,
        },
    ];
    let schedule = MortgageSchedule {
        loan_id: 500_001,
        mortgage_type: MortgageType::FixedRate,
        original_balance_cents: 30_000_000,
        interest_rate_bps: 500,
        term_months: 360,
        entries,
    };
    let ver = Version::new(2, 0, 0);
    let bytes = encode_versioned_value(&schedule, ver).expect("encode mortgage schedule");
    let (decoded, version, consumed) =
        decode_versioned_value::<MortgageSchedule>(&bytes).expect("decode mortgage schedule");
    assert_eq!(decoded, schedule);
    assert_eq!(version.major, 2);
    assert!(consumed > 0);
}

// ── Test 4: Building Inspection Report ──────────────────────────────────────

#[test]
fn test_building_inspection_multiple_findings() {
    let findings = vec![
        InspectionFinding {
            finding_id: 1,
            category: "Roofing".into(),
            description: "Missing shingles on north-facing slope, approx 15 sqft".into(),
            severity: InspectionSeverity::Moderate,
            estimated_repair_cents: 350_000,
            photo_count: 4,
        },
        InspectionFinding {
            finding_id: 2,
            category: "Plumbing".into(),
            description: "Galvanized supply lines showing corrosion at joints".into(),
            severity: InspectionSeverity::Major,
            estimated_repair_cents: 1_200_000,
            photo_count: 6,
        },
        InspectionFinding {
            finding_id: 3,
            category: "Electrical".into(),
            description: "Double-tapped breaker in main panel".into(),
            severity: InspectionSeverity::SafetyHazard,
            estimated_repair_cents: 80_000,
            photo_count: 2,
        },
    ];
    let report = BuildingInspectionReport {
        report_id: 900_100,
        property_address: "1600 Pennsylvania Ave NW, Washington, DC 20500".into(),
        inspector_name: "Jane Doe, ASHI Certified #12345".into(),
        inspection_date_epoch: 1_710_000_000,
        findings,
        overall_condition_score: 62,
        recommended_reinspection: true,
    };
    let ver = Version::new(1, 0, 3);
    let bytes = encode_versioned_value(&report, ver).expect("encode inspection report");
    let (decoded, version, consumed) = decode_versioned_value::<BuildingInspectionReport>(&bytes)
        .expect("decode inspection report");
    assert_eq!(decoded, report);
    assert_eq!(version.patch, 3);
    assert!(consumed > 0);
}

// ── Test 5: Rental Yield Analysis ───────────────────────────────────────────

#[test]
fn test_rental_yield_multifamily_investment() {
    let analysis = RentalYieldAnalysis {
        property_id: 200_042,
        purchase_price_cents: 120_000_000,
        monthly_rent_cents: 1_000_000,
        vacancy_rate_bps: 500,
        operating_expense_ratio_bps: 4_000,
        gross_yield_bps: 1_000,
        net_yield_bps: 570,
        cap_rate_bps: 680,
        cash_on_cash_return_bps: 850,
        annual_appreciation_bps: 350,
        debt_service_coverage_x100: 125,
    };
    let ver = Version::new(3, 1, 0);
    let bytes = encode_versioned_value(&analysis, ver).expect("encode rental yield");
    let (decoded, version, consumed) =
        decode_versioned_value::<RentalYieldAnalysis>(&bytes).expect("decode rental yield");
    assert_eq!(decoded, analysis);
    assert_eq!(version.major, 3);
    assert_eq!(version.minor, 1);
    assert!(consumed > 0);
}

// ── Test 6: Property Tax Assessment ─────────────────────────────────────────

#[test]
fn test_property_tax_assessment_with_exemptions() {
    let assessment = PropertyTaxAssessment {
        parcel_id: "07-14-300-012-0000".into(),
        tax_year: 2026,
        land_value_cents: 15_000_000,
        improvement_value_cents: 35_000_000,
        total_assessed_cents: 50_000_000,
        exemptions_cents: 7_500_000,
        taxable_value_cents: 42_500_000,
        millage_rate_x1000: 72_350,
        annual_tax_cents: 3_074_875,
        appeal_deadline_epoch: 1_720_000_000,
    };
    let ver = Version::new(1, 1, 0);
    let bytes = encode_versioned_value(&assessment, ver).expect("encode tax assessment");
    let (decoded, version, consumed) =
        decode_versioned_value::<PropertyTaxAssessment>(&bytes).expect("decode tax assessment");
    assert_eq!(decoded, assessment);
    assert_eq!(version.major, 1);
    assert_eq!(version.minor, 1);
    assert!(consumed > 0);
}

// ── Test 7: Title Search with Defects ───────────────────────────────────────

#[test]
fn test_title_search_with_lien_and_encumbrances() {
    let result = TitleSearchResult {
        search_id: 330_001,
        parcel_id: "12-34-567-890".into(),
        current_owner: "Acme Holdings LLC".into(),
        chain_of_title_length: 8,
        defect: TitleDefectType::LienUnresolved,
        liens_outstanding_cents: 12_500_000,
        easements_count: 2,
        encumbrances: vec![
            "Utility easement - 10ft along north boundary".into(),
            "Restrictive covenant - no commercial activity".into(),
        ],
        title_insurance_premium_cents: 325_000,
    };
    let ver = Version::new(2, 3, 1);
    let bytes = encode_versioned_value(&result, ver).expect("encode title search");
    let (decoded, version, consumed) =
        decode_versioned_value::<TitleSearchResult>(&bytes).expect("decode title search");
    assert_eq!(decoded, result);
    assert_eq!(version.major, 2);
    assert_eq!(version.minor, 3);
    assert_eq!(version.patch, 1);
    assert!(consumed > 0);
}

// ── Test 8: Clean Title Search ──────────────────────────────────────────────

#[test]
fn test_title_search_clean_no_defects() {
    let result = TitleSearchResult {
        search_id: 330_002,
        parcel_id: "55-66-777-888".into(),
        current_owner: "Maria Gonzalez".into(),
        chain_of_title_length: 5,
        defect: TitleDefectType::None,
        liens_outstanding_cents: 0,
        easements_count: 0,
        encumbrances: Vec::new(),
        title_insurance_premium_cents: 175_000,
    };
    let ver = Version::new(1, 0, 0);
    let bytes = encode_versioned_value(&result, ver).expect("encode clean title");
    let (decoded, version, _consumed) =
        decode_versioned_value::<TitleSearchResult>(&bytes).expect("decode clean title");
    assert_eq!(decoded, result);
    assert_eq!(version.major, 1);
}

// ── Test 9: Escrow Account Lifecycle ────────────────────────────────────────

#[test]
fn test_escrow_account_in_appraisal_contingency() {
    let escrow = EscrowAccount {
        escrow_id: 700_001,
        buyer: "John Smith".into(),
        seller: "River Properties Inc".into(),
        state: EscrowState::AppraisalContingency,
        purchase_price_cents: 55_000_000,
        earnest_money_cents: 1_650_000,
        closing_costs_buyer_cents: 1_375_000,
        closing_costs_seller_cents: 3_300_000,
        prorated_taxes_cents: 412_500,
        remaining_balance_cents: 53_350_000,
        target_close_epoch: 1_725_000_000,
    };
    let ver = Version::new(4, 0, 0);
    let bytes = encode_versioned_value(&escrow, ver).expect("encode escrow");
    let (decoded, version, consumed) =
        decode_versioned_value::<EscrowAccount>(&bytes).expect("decode escrow");
    assert_eq!(decoded, escrow);
    assert_eq!(version.major, 4);
    assert!(consumed > 0);
}

// ── Test 10: Zoning Variance Application ────────────────────────────────────

#[test]
fn test_zoning_variance_residential_to_mixed_use() {
    let app = ZoningVarianceApplication {
        application_id: 880_001,
        parcel_id: "99-01-234-567".into(),
        current_zoning: ZoningClassification::ResidentialSingle,
        requested_zoning: ZoningClassification::MixedUse,
        applicant_name: "Downtown Redevelopment Partners".into(),
        proposed_use: "Ground floor retail with 12 residential units above".into(),
        setback_front_inches: 180,
        setback_side_inches: 60,
        max_height_inches: 540,
        lot_coverage_bps: 7_500,
        floor_area_ratio_x100: 250,
        public_hearing_epoch: 1_730_000_000,
        neighbors_notified: 47,
    };
    let ver = Version::new(1, 5, 0);
    let bytes = encode_versioned_value(&app, ver).expect("encode zoning variance");
    let (decoded, version, consumed) = decode_versioned_value::<ZoningVarianceApplication>(&bytes)
        .expect("decode zoning variance");
    assert_eq!(decoded, app);
    assert_eq!(version.minor, 5);
    assert!(consumed > 0);
}

// ── Test 11: HOA Fee Structure ──────────────────────────────────────────────

#[test]
fn test_hoa_fee_structure_detailed_breakdown() {
    let items = vec![
        HoaFeeItem {
            category: "Common area maintenance".into(),
            monthly_amount_cents: 8_500,
        },
        HoaFeeItem {
            category: "Insurance master policy".into(),
            monthly_amount_cents: 4_200,
        },
        HoaFeeItem {
            category: "Reserve fund contribution".into(),
            monthly_amount_cents: 6_000,
        },
        HoaFeeItem {
            category: "Landscaping".into(),
            monthly_amount_cents: 3_500,
        },
        HoaFeeItem {
            category: "Pool and fitness center".into(),
            monthly_amount_cents: 2_800,
        },
        HoaFeeItem {
            category: "Management company fee".into(),
            monthly_amount_cents: 5_000,
        },
    ];
    let hoa = HoaFeeStructure {
        hoa_id: 440_001,
        community_name: "Sunset Ridge Estates".into(),
        total_monthly_cents: 30_000,
        reserve_balance_cents: 285_000_000,
        special_assessment_cents: 0,
        fee_items: items,
        delinquency_rate_bps: 320,
        next_increase_epoch: 1_735_000_000,
    };
    let ver = Version::new(2, 1, 4);
    let bytes = encode_versioned_value(&hoa, ver).expect("encode HOA fees");
    let (decoded, version, consumed) =
        decode_versioned_value::<HoaFeeStructure>(&bytes).expect("decode HOA fees");
    assert_eq!(decoded, hoa);
    assert_eq!(version.major, 2);
    assert_eq!(version.minor, 1);
    assert_eq!(version.patch, 4);
    assert!(consumed > 0);
}

// ── Test 12: Commercial Lease Triple Net ────────────────────────────────────

#[test]
fn test_commercial_lease_triple_net() {
    let lease = CommercialLeaseTerm {
        lease_id: 660_001,
        tenant_name: "Starbucks Corporation".into(),
        lease_type: LeaseType::TripleNet,
        sqft_leased: 1_800,
        base_rent_annual_cents: 5_400_000,
        cam_charges_annual_cents: 720_000,
        insurance_annual_cents: 180_000,
        tax_pass_through_annual_cents: 360_000,
        escalation_rate_bps: 300,
        lease_start_epoch: 1_704_067_200,
        lease_end_epoch: 1_861_920_000,
        renewal_options: 2,
        tenant_improvement_allowance_cents: 9_000_000,
        free_rent_months: 3,
    };
    let ver = Version::new(3, 0, 0);
    let bytes = encode_versioned_value(&lease, ver).expect("encode NNN lease");
    let (decoded, version, consumed) =
        decode_versioned_value::<CommercialLeaseTerm>(&bytes).expect("decode NNN lease");
    assert_eq!(decoded, lease);
    assert_eq!(version.major, 3);
    assert!(consumed > 0);
}

// ── Test 13: Commercial Lease Modified Gross ────────────────────────────────

#[test]
fn test_commercial_lease_modified_gross() {
    let lease = CommercialLeaseTerm {
        lease_id: 660_002,
        tenant_name: "Regional Law Firm LLP".into(),
        lease_type: LeaseType::ModifiedGross,
        sqft_leased: 4_500,
        base_rent_annual_cents: 15_750_000,
        cam_charges_annual_cents: 0,
        insurance_annual_cents: 0,
        tax_pass_through_annual_cents: 450_000,
        escalation_rate_bps: 250,
        lease_start_epoch: 1_709_251_200,
        lease_end_epoch: 1_898_150_400,
        renewal_options: 1,
        tenant_improvement_allowance_cents: 22_500_000,
        free_rent_months: 6,
    };
    let ver = Version::new(3, 0, 1);
    let bytes = encode_versioned_value(&lease, ver).expect("encode modified gross lease");
    let (decoded, version, consumed) =
        decode_versioned_value::<CommercialLeaseTerm>(&bytes).expect("decode modified gross lease");
    assert_eq!(decoded, lease);
    assert_eq!(version.major, 3);
    assert_eq!(version.patch, 1);
    assert!(consumed > 0);
}

// ── Test 14: DCF Property Valuation ─────────────────────────────────────────

#[test]
fn test_property_valuation_dcf_method() {
    let valuation = PropertyValuation {
        valuation_id: 770_001,
        property_address: "300 Park Avenue, New York, NY 10022".into(),
        method: ValuationMethod::DiscountedCashFlow,
        estimated_value_cents: 85_000_000_000,
        discount_rate_bps: 750,
        terminal_cap_rate_bps: 550,
        noi_annual_cents: 5_950_000_000,
        holding_period_years: 10,
        confidence_score_bps: 8_200,
    };
    let ver = Version::new(5, 0, 0);
    let bytes = encode_versioned_value(&valuation, ver).expect("encode DCF valuation");
    let (decoded, version, consumed) =
        decode_versioned_value::<PropertyValuation>(&bytes).expect("decode DCF valuation");
    assert_eq!(decoded, valuation);
    assert_eq!(version.major, 5);
    assert!(consumed > 0);
}

// ── Test 15: Cap Rate Valuation ─────────────────────────────────────────────

#[test]
fn test_property_valuation_cap_rate_method() {
    let valuation = PropertyValuation {
        valuation_id: 770_002,
        property_address: "500 Industrial Blvd, Houston, TX 77001".into(),
        method: ValuationMethod::CapRate,
        estimated_value_cents: 12_000_000_000,
        discount_rate_bps: 0,
        terminal_cap_rate_bps: 650,
        noi_annual_cents: 780_000_000,
        holding_period_years: 0,
        confidence_score_bps: 9_100,
    };
    let ver = Version::new(5, 1, 0);
    let bytes = encode_versioned_value(&valuation, ver).expect("encode cap rate valuation");
    let (decoded, version, consumed) =
        decode_versioned_value::<PropertyValuation>(&bytes).expect("decode cap rate valuation");
    assert_eq!(decoded, valuation);
    assert_eq!(version.major, 5);
    assert_eq!(version.minor, 1);
    assert!(consumed > 0);
}

// ── Test 16: Condo Listing with HOA ─────────────────────────────────────────

#[test]
fn test_listing_condo_with_high_hoa() {
    let listing = PropertyListing {
        listing_id: 100_042,
        mls_number: "MLS-2026-99001".into(),
        address: "1500 Ocean Drive Unit 42B, Miami Beach, FL 33139".into(),
        property_type: PropertyType::Condo,
        status: ListingStatus::Pending,
        list_price_cents: 275_000_000,
        bedrooms: 2,
        bathrooms_x10: 20,
        sqft: 1_200,
        lot_sqft: 0,
        year_built: 2019,
        days_on_market: 3,
        hoa_monthly_cents: 125_000,
    };
    let ver = Version::new(1, 0, 0);
    let bytes = encode_versioned_value(&listing, ver).expect("encode condo listing");
    let (decoded, version, _consumed) =
        decode_versioned_value::<PropertyListing>(&bytes).expect("decode condo listing");
    assert_eq!(decoded, listing);
    assert_eq!(version.major, 1);
}

// ── Test 17: Property Portfolio ─────────────────────────────────────────────

#[test]
fn test_property_portfolio_multi_asset() {
    let portfolio = PropertyPortfolio {
        portfolio_id: 550_001,
        owner_name: "Pacific Coast REIT".into(),
        total_units: 342,
        total_sqft: 485_000,
        aggregate_value_cents: 320_000_000_000,
        aggregate_noi_cents: 19_200_000_000,
        weighted_avg_cap_rate_bps: 600,
        leverage_ratio_bps: 5_500,
        property_ids: vec![1001, 1002, 1003, 1004, 1005, 1006, 1007],
    };
    let ver = Version::new(2, 0, 0);
    let bytes = encode_versioned_value(&portfolio, ver).expect("encode portfolio");
    let (decoded, version, consumed) =
        decode_versioned_value::<PropertyPortfolio>(&bytes).expect("decode portfolio");
    assert_eq!(decoded, portfolio);
    assert_eq!(version.major, 2);
    assert!(consumed > 0);
}

// ── Test 18: Construction Loan with Draw Schedule ───────────────────────────

#[test]
fn test_construction_loan_partial_draws() {
    let draws = vec![
        ConstructionDrawSchedule {
            draw_number: 1,
            milestone: "Foundation completion".into(),
            budgeted_cents: 500_000_000,
            released_cents: 500_000_000,
            retainage_cents: 50_000_000,
            inspection_passed: true,
            draw_date_epoch: 1_704_000_000,
        },
        ConstructionDrawSchedule {
            draw_number: 2,
            milestone: "Framing and rough-in".into(),
            budgeted_cents: 800_000_000,
            released_cents: 800_000_000,
            retainage_cents: 80_000_000,
            inspection_passed: true,
            draw_date_epoch: 1_709_000_000,
        },
        ConstructionDrawSchedule {
            draw_number: 3,
            milestone: "MEP systems rough-in".into(),
            budgeted_cents: 600_000_000,
            released_cents: 0,
            retainage_cents: 0,
            inspection_passed: false,
            draw_date_epoch: 0,
        },
    ];
    let loan = ConstructionLoan {
        loan_id: 990_001,
        project_name: "Waterfront Residences Phase II".into(),
        total_commitment_cents: 3_500_000_000,
        interest_reserve_cents: 350_000_000,
        draws,
        maturity_epoch: 1_767_225_600,
    };
    let ver = Version::new(1, 3, 0);
    let bytes = encode_versioned_value(&loan, ver).expect("encode construction loan");
    let (decoded, version, consumed) =
        decode_versioned_value::<ConstructionLoan>(&bytes).expect("decode construction loan");
    assert_eq!(decoded, loan);
    assert_eq!(version.minor, 3);
    assert!(consumed > 0);
}

// ── Test 19: Rent Roll ──────────────────────────────────────────────────────

#[test]
fn test_rent_roll_mixed_occupancy() {
    let entries = vec![
        RentRollEntry {
            unit_id: "101-A".into(),
            tenant_name: "Chen Wei".into(),
            sqft: 850,
            monthly_rent_cents: 185_000,
            lease_start_epoch: 1_700_000_000,
            lease_end_epoch: 1_731_536_000,
            security_deposit_cents: 185_000,
            is_occupied: true,
        },
        RentRollEntry {
            unit_id: "102-B".into(),
            tenant_name: String::new(),
            sqft: 950,
            monthly_rent_cents: 0,
            lease_start_epoch: 0,
            lease_end_epoch: 0,
            security_deposit_cents: 0,
            is_occupied: false,
        },
        RentRollEntry {
            unit_id: "201-A".into(),
            tenant_name: "Fatima Al-Rashid".into(),
            sqft: 1_100,
            monthly_rent_cents: 225_000,
            lease_start_epoch: 1_696_000_000,
            lease_end_epoch: 1_727_536_000,
            security_deposit_cents: 225_000,
            is_occupied: true,
        },
    ];
    let roll = RentRoll {
        property_id: 200_100,
        property_address: "4200 Lake Shore Dr, Chicago, IL 60613".into(),
        entries,
        total_monthly_income_cents: 410_000,
        occupancy_rate_bps: 6_667,
    };
    let ver = Version::new(2, 2, 0);
    let bytes = encode_versioned_value(&roll, ver).expect("encode rent roll");
    let (decoded, version, consumed) =
        decode_versioned_value::<RentRoll>(&bytes).expect("decode rent roll");
    assert_eq!(decoded, roll);
    assert_eq!(version.major, 2);
    assert_eq!(version.minor, 2);
    assert!(consumed > 0);
}

// ── Test 20: Escrow Closed State ────────────────────────────────────────────

#[test]
fn test_escrow_account_closed_successfully() {
    let escrow = EscrowAccount {
        escrow_id: 700_042,
        buyer: "Alexandra Petrov".into(),
        seller: "Heritage Home Builders".into(),
        state: EscrowState::Closed,
        purchase_price_cents: 82_500_000,
        earnest_money_cents: 2_475_000,
        closing_costs_buyer_cents: 2_062_500,
        closing_costs_seller_cents: 4_950_000,
        prorated_taxes_cents: 687_500,
        remaining_balance_cents: 0,
        target_close_epoch: 1_718_000_000,
    };
    let ver = Version::new(4, 1, 0);
    let bytes = encode_versioned_value(&escrow, ver).expect("encode closed escrow");
    let (decoded, version, consumed) =
        decode_versioned_value::<EscrowAccount>(&bytes).expect("decode closed escrow");
    assert_eq!(decoded, escrow);
    assert_eq!(decoded.remaining_balance_cents, 0);
    assert_eq!(version.major, 4);
    assert_eq!(version.minor, 1);
    assert!(consumed > 0);
}

// ── Test 21: Inspection Cosmetic Only ───────────────────────────────────────

#[test]
fn test_inspection_report_cosmetic_issues_only() {
    let findings = vec![InspectionFinding {
        finding_id: 1,
        category: "Interior paint".into(),
        description: "Scuff marks on hallway walls near entryway".into(),
        severity: InspectionSeverity::Cosmetic,
        estimated_repair_cents: 45_000,
        photo_count: 1,
    }];
    let report = BuildingInspectionReport {
        report_id: 900_200,
        property_address: "88 Willow Creek Ct, Naperville, IL 60540".into(),
        inspector_name: "Robert Kim, IL License #471-009832".into(),
        inspection_date_epoch: 1_712_000_000,
        findings,
        overall_condition_score: 94,
        recommended_reinspection: false,
    };
    let ver = Version::new(1, 0, 3);
    let bytes = encode_versioned_value(&report, ver).expect("encode cosmetic inspection");
    let (decoded, version, consumed) = decode_versioned_value::<BuildingInspectionReport>(&bytes)
        .expect("decode cosmetic inspection");
    assert_eq!(decoded, report);
    assert_eq!(decoded.overall_condition_score, 94);
    assert!(!decoded.recommended_reinspection);
    assert_eq!(version.patch, 3);
    assert!(consumed > 0);
}

// ── Test 22: GRM Valuation for Small Multifamily ────────────────────────────

#[test]
fn test_property_valuation_gross_rent_multiplier() {
    let valuation = PropertyValuation {
        valuation_id: 770_003,
        property_address: "2100 MLK Jr Blvd, Atlanta, GA 30310".into(),
        method: ValuationMethod::GrossRentMultiplier,
        estimated_value_cents: 4_800_000_00,
        discount_rate_bps: 0,
        terminal_cap_rate_bps: 0,
        noi_annual_cents: 336_000_00,
        holding_period_years: 0,
        confidence_score_bps: 7_500,
    };
    let ver = Version::new(5, 2, 0);
    let bytes = encode_versioned_value(&valuation, ver).expect("encode GRM valuation");
    let (decoded, version, consumed) =
        decode_versioned_value::<PropertyValuation>(&bytes).expect("decode GRM valuation");
    assert_eq!(decoded, valuation);
    assert_eq!(version.major, 5);
    assert_eq!(version.minor, 2);
    assert!(consumed > 0);
}
