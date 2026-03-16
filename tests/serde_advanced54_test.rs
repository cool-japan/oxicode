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
use ::serde::{Deserialize, Serialize};
use oxicode::config;
use oxicode::serde::{decode_owned_from_slice, encode_to_vec};

// --- Domain types ---

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
struct PropertyListing {
    listing_id: u64,
    address: String,
    city: String,
    state: String,
    zip_code: String,
    sqft: u32,
    bedrooms: u8,
    bathrooms: f32,
    list_price_cents: u64,
    year_built: u16,
    property_type: String,
    is_active: bool,
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
struct MortgageCalculation {
    loan_id: u64,
    principal_cents: u64,
    apr_bps: u32,
    term_months: u32,
    ltv_bps: u32,
    monthly_payment_cents: u64,
    total_interest_cents: u64,
    amortization_type: String,
    is_fixed_rate: bool,
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
struct ComparableProperty {
    comp_id: u64,
    address: String,
    sold_price_cents: u64,
    sold_date: String,
    sqft: u32,
    bedrooms: u8,
    distance_meters: u32,
    adjustment_cents: i64,
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
struct ComparableMarketAnalysis {
    subject_address: String,
    analysis_date: String,
    comparables: Vec<ComparableProperty>,
    estimated_value_cents: u64,
    confidence_pct: f32,
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
struct TenantScreening {
    applicant_id: u64,
    full_name: String,
    credit_score: u16,
    income_monthly_cents: u64,
    eviction_history_count: u8,
    criminal_check_passed: bool,
    employment_verified: bool,
    references_count: u8,
    overall_score: f32,
    recommendation: String,
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
struct LeaseAgreement {
    lease_id: u64,
    tenant_name: String,
    property_address: String,
    start_date: String,
    end_date: String,
    monthly_rent_cents: u64,
    security_deposit_cents: u64,
    pet_deposit_cents: u64,
    is_month_to_month: bool,
    clauses: Vec<String>,
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
struct InspectionItem {
    category: String,
    description: String,
    severity: String,
    estimated_repair_cents: u64,
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
struct PropertyInspectionReport {
    report_id: u64,
    property_address: String,
    inspector_name: String,
    inspection_date: String,
    items: Vec<InspectionItem>,
    overall_condition: String,
    pass: bool,
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
struct HoaDue {
    period: String,
    amount_cents: u64,
    paid: bool,
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
struct HoaManagementRecord {
    hoa_id: u64,
    community_name: String,
    unit_count: u32,
    monthly_assessment_cents: u64,
    reserve_fund_cents: u64,
    dues: Vec<HoaDue>,
    board_members: Vec<String>,
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
struct TitleSearchResult {
    search_id: u64,
    property_address: String,
    current_owner: String,
    liens: Vec<String>,
    easements: Vec<String>,
    encumbrances: Vec<String>,
    title_clear: bool,
    search_date: String,
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
struct EscrowTransaction {
    escrow_id: u64,
    buyer_name: String,
    seller_name: String,
    property_address: String,
    purchase_price_cents: u64,
    earnest_money_cents: u64,
    closing_costs_cents: u64,
    status: String,
    open_date: String,
    expected_close_date: String,
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
struct ZoningClassification {
    parcel_id: String,
    zone_code: String,
    zone_description: String,
    allowed_uses: Vec<String>,
    max_building_height_ft: u32,
    max_lot_coverage_pct: f32,
    min_setback_front_ft: u32,
    min_setback_side_ft: u32,
    is_overlay_district: bool,
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
struct PropertyTaxAssessment {
    parcel_id: String,
    tax_year: u16,
    land_value_cents: u64,
    improvement_value_cents: u64,
    total_assessed_cents: u64,
    tax_rate_bps: u32,
    annual_tax_cents: u64,
    exemptions: Vec<String>,
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
struct RenovationLineItem {
    room: String,
    description: String,
    material_cost_cents: u64,
    labor_cost_cents: u64,
    timeline_days: u16,
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
struct RenovationCostEstimate {
    estimate_id: u64,
    property_address: String,
    contractor_name: String,
    line_items: Vec<RenovationLineItem>,
    total_cost_cents: u64,
    contingency_pct: f32,
    valid_until: String,
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
struct SmartHomeDevice {
    device_id: String,
    device_type: String,
    manufacturer: String,
    firmware_version: String,
    is_online: bool,
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
struct SmartHomeIotConfig {
    property_id: u64,
    hub_model: String,
    wifi_ssid: String,
    devices: Vec<SmartHomeDevice>,
    automation_rules_count: u32,
    monthly_data_usage_mb: u32,
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
struct RentalYieldCalculation {
    property_id: u64,
    address: String,
    purchase_price_cents: u64,
    monthly_rent_cents: u64,
    annual_expenses_cents: u64,
    vacancy_rate_bps: u32,
    gross_yield_bps: u32,
    net_yield_bps: u32,
    cap_rate_bps: u32,
    cash_on_cash_bps: u32,
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
struct BuildingPermitApplication {
    permit_id: u64,
    applicant_name: String,
    property_address: String,
    permit_type: String,
    project_description: String,
    estimated_cost_cents: u64,
    submission_date: String,
    status: String,
    reviewer_notes: Vec<String>,
    approved: bool,
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
struct PropertyPortfolio {
    owner_name: String,
    listings: Vec<PropertyListing>,
    total_value_cents: u64,
    total_monthly_income_cents: u64,
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
struct OpenHouseEvent {
    event_id: u64,
    listing_id: u64,
    property_address: String,
    date: String,
    start_time: String,
    end_time: String,
    agent_name: String,
    rsvp_count: u32,
    visitor_comments: Vec<String>,
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
struct PropertyInsuranceQuote {
    quote_id: u64,
    property_address: String,
    coverage_type: String,
    dwelling_coverage_cents: u64,
    liability_coverage_cents: u64,
    annual_premium_cents: u64,
    deductible_cents: u64,
    flood_zone: bool,
    earthquake_rider: bool,
}

// --- Tests ---

#[test]
fn test_property_listing_roundtrip() {
    let listing = PropertyListing {
        listing_id: 10001,
        address: "742 Evergreen Terrace".to_string(),
        city: "Springfield".to_string(),
        state: "IL".to_string(),
        zip_code: "62704".to_string(),
        sqft: 2200,
        bedrooms: 4,
        bathrooms: 2.5,
        list_price_cents: 35000000,
        year_built: 1985,
        property_type: "Single Family".to_string(),
        is_active: true,
    };
    let cfg = config::standard();
    let encoded = encode_to_vec(&listing, cfg).expect("encode property listing");
    let (decoded, _): (PropertyListing, _) =
        decode_owned_from_slice(&encoded, cfg).expect("decode property listing");
    assert_eq!(listing, decoded);
}

#[test]
fn test_mortgage_calculation_roundtrip() {
    let mortgage = MortgageCalculation {
        loan_id: 500100,
        principal_cents: 28000000,
        apr_bps: 725,
        term_months: 360,
        ltv_bps: 8000,
        monthly_payment_cents: 191012,
        total_interest_cents: 40764320,
        amortization_type: "Fixed".to_string(),
        is_fixed_rate: true,
    };
    let cfg = config::standard();
    let encoded = encode_to_vec(&mortgage, cfg).expect("encode mortgage");
    let (decoded, _): (MortgageCalculation, _) =
        decode_owned_from_slice(&encoded, cfg).expect("decode mortgage");
    assert_eq!(mortgage, decoded);
}

#[test]
fn test_comparable_market_analysis_roundtrip() {
    let cma = ComparableMarketAnalysis {
        subject_address: "100 Main St, Austin TX".to_string(),
        analysis_date: "2026-03-01".to_string(),
        comparables: vec![
            ComparableProperty {
                comp_id: 1,
                address: "102 Main St".to_string(),
                sold_price_cents: 42500000,
                sold_date: "2025-12-15".to_string(),
                sqft: 1900,
                bedrooms: 3,
                distance_meters: 120,
                adjustment_cents: -150000,
            },
            ComparableProperty {
                comp_id: 2,
                address: "210 Oak Ave".to_string(),
                sold_price_cents: 44000000,
                sold_date: "2026-01-20".to_string(),
                sqft: 2100,
                bedrooms: 4,
                distance_meters: 350,
                adjustment_cents: 200000,
            },
            ComparableProperty {
                comp_id: 3,
                address: "55 Elm Dr".to_string(),
                sold_price_cents: 39800000,
                sold_date: "2025-11-05".to_string(),
                sqft: 1750,
                bedrooms: 3,
                distance_meters: 500,
                adjustment_cents: 500000,
            },
        ],
        estimated_value_cents: 42350000,
        confidence_pct: 87.5,
    };
    let cfg = config::standard();
    let encoded = encode_to_vec(&cma, cfg).expect("encode CMA");
    let (decoded, _): (ComparableMarketAnalysis, _) =
        decode_owned_from_slice(&encoded, cfg).expect("decode CMA");
    assert_eq!(cma, decoded);
}

#[test]
fn test_tenant_screening_roundtrip() {
    let screening = TenantScreening {
        applicant_id: 88001,
        full_name: "Jane Doe".to_string(),
        credit_score: 740,
        income_monthly_cents: 650000,
        eviction_history_count: 0,
        criminal_check_passed: true,
        employment_verified: true,
        references_count: 3,
        overall_score: 92.4,
        recommendation: "Approved".to_string(),
    };
    let cfg = config::standard();
    let encoded = encode_to_vec(&screening, cfg).expect("encode tenant screening");
    let (decoded, _): (TenantScreening, _) =
        decode_owned_from_slice(&encoded, cfg).expect("decode tenant screening");
    assert_eq!(screening, decoded);
}

#[test]
fn test_lease_agreement_roundtrip() {
    let lease = LeaseAgreement {
        lease_id: 77001,
        tenant_name: "Bob Smith".to_string(),
        property_address: "500 Park Ave, Unit 12B".to_string(),
        start_date: "2026-04-01".to_string(),
        end_date: "2027-03-31".to_string(),
        monthly_rent_cents: 220000,
        security_deposit_cents: 220000,
        pet_deposit_cents: 50000,
        is_month_to_month: false,
        clauses: vec![
            "No subletting without written consent".to_string(),
            "Quiet hours 10pm-8am".to_string(),
            "Tenant responsible for lawn maintenance".to_string(),
            "Maximum 2 pets under 50 lbs".to_string(),
        ],
    };
    let cfg = config::standard();
    let encoded = encode_to_vec(&lease, cfg).expect("encode lease");
    let (decoded, _): (LeaseAgreement, _) =
        decode_owned_from_slice(&encoded, cfg).expect("decode lease");
    assert_eq!(lease, decoded);
}

#[test]
fn test_property_inspection_report_roundtrip() {
    let report = PropertyInspectionReport {
        report_id: 330001,
        property_address: "1234 Maple Ln, Denver CO".to_string(),
        inspector_name: "Mike Torres".to_string(),
        inspection_date: "2026-02-28".to_string(),
        items: vec![
            InspectionItem {
                category: "Roof".to_string(),
                description: "Missing shingles on south slope".to_string(),
                severity: "Moderate".to_string(),
                estimated_repair_cents: 350000,
            },
            InspectionItem {
                category: "Plumbing".to_string(),
                description: "Slow drain in master bathroom".to_string(),
                severity: "Minor".to_string(),
                estimated_repair_cents: 15000,
            },
            InspectionItem {
                category: "Electrical".to_string(),
                description: "GFCI outlets missing in kitchen".to_string(),
                severity: "Major".to_string(),
                estimated_repair_cents: 80000,
            },
        ],
        overall_condition: "Fair".to_string(),
        pass: false,
    };
    let cfg = config::standard();
    let encoded = encode_to_vec(&report, cfg).expect("encode inspection report");
    let (decoded, _): (PropertyInspectionReport, _) =
        decode_owned_from_slice(&encoded, cfg).expect("decode inspection report");
    assert_eq!(report, decoded);
}

#[test]
fn test_hoa_management_record_roundtrip() {
    let hoa = HoaManagementRecord {
        hoa_id: 4400,
        community_name: "Sunset Ridge Estates".to_string(),
        unit_count: 156,
        monthly_assessment_cents: 45000,
        reserve_fund_cents: 125000000,
        dues: vec![
            HoaDue {
                period: "2026-01".to_string(),
                amount_cents: 45000,
                paid: true,
            },
            HoaDue {
                period: "2026-02".to_string(),
                amount_cents: 45000,
                paid: true,
            },
            HoaDue {
                period: "2026-03".to_string(),
                amount_cents: 45000,
                paid: false,
            },
        ],
        board_members: vec![
            "Alice Johnson".to_string(),
            "Carlos Rivera".to_string(),
            "Priya Sharma".to_string(),
        ],
    };
    let cfg = config::standard();
    let encoded = encode_to_vec(&hoa, cfg).expect("encode HOA record");
    let (decoded, _): (HoaManagementRecord, _) =
        decode_owned_from_slice(&encoded, cfg).expect("decode HOA record");
    assert_eq!(hoa, decoded);
}

#[test]
fn test_title_search_result_roundtrip() {
    let title = TitleSearchResult {
        search_id: 990001,
        property_address: "88 Harbor View, Miami FL".to_string(),
        current_owner: "Oceanic Properties LLC".to_string(),
        liens: vec!["First mortgage - National Bank $320,000".to_string()],
        easements: vec![
            "Utility easement - east 10ft".to_string(),
            "Drainage easement - south boundary".to_string(),
        ],
        encumbrances: vec![],
        title_clear: false,
        search_date: "2026-03-10".to_string(),
    };
    let cfg = config::standard();
    let encoded = encode_to_vec(&title, cfg).expect("encode title search");
    let (decoded, _): (TitleSearchResult, _) =
        decode_owned_from_slice(&encoded, cfg).expect("decode title search");
    assert_eq!(title, decoded);
}

#[test]
fn test_escrow_transaction_roundtrip() {
    let escrow = EscrowTransaction {
        escrow_id: 660001,
        buyer_name: "Sarah Lee".to_string(),
        seller_name: "David Kim".to_string(),
        property_address: "321 Cedar Blvd, Portland OR".to_string(),
        purchase_price_cents: 51500000,
        earnest_money_cents: 1500000,
        closing_costs_cents: 1200000,
        status: "Pending Appraisal".to_string(),
        open_date: "2026-02-15".to_string(),
        expected_close_date: "2026-04-01".to_string(),
    };
    let cfg = config::standard();
    let encoded = encode_to_vec(&escrow, cfg).expect("encode escrow");
    let (decoded, _): (EscrowTransaction, _) =
        decode_owned_from_slice(&encoded, cfg).expect("decode escrow");
    assert_eq!(escrow, decoded);
}

#[test]
fn test_zoning_classification_roundtrip() {
    let zoning = ZoningClassification {
        parcel_id: "APN-2026-0450-012".to_string(),
        zone_code: "R-2".to_string(),
        zone_description: "Medium Density Residential".to_string(),
        allowed_uses: vec![
            "Single Family Dwelling".to_string(),
            "Duplex".to_string(),
            "Accessory Dwelling Unit".to_string(),
            "Home Occupation".to_string(),
        ],
        max_building_height_ft: 35,
        max_lot_coverage_pct: 45.0,
        min_setback_front_ft: 20,
        min_setback_side_ft: 5,
        is_overlay_district: false,
    };
    let cfg = config::standard();
    let encoded = encode_to_vec(&zoning, cfg).expect("encode zoning");
    let (decoded, _): (ZoningClassification, _) =
        decode_owned_from_slice(&encoded, cfg).expect("decode zoning");
    assert_eq!(zoning, decoded);
}

#[test]
fn test_property_tax_assessment_roundtrip() {
    let tax = PropertyTaxAssessment {
        parcel_id: "APN-2026-0450-012".to_string(),
        tax_year: 2026,
        land_value_cents: 15000000,
        improvement_value_cents: 25000000,
        total_assessed_cents: 40000000,
        tax_rate_bps: 125,
        annual_tax_cents: 500000,
        exemptions: vec![
            "Homestead Exemption".to_string(),
            "Senior Citizen Exemption".to_string(),
        ],
    };
    let cfg = config::standard();
    let encoded = encode_to_vec(&tax, cfg).expect("encode tax assessment");
    let (decoded, _): (PropertyTaxAssessment, _) =
        decode_owned_from_slice(&encoded, cfg).expect("decode tax assessment");
    assert_eq!(tax, decoded);
}

#[test]
fn test_renovation_cost_estimate_roundtrip() {
    let estimate = RenovationCostEstimate {
        estimate_id: 220001,
        property_address: "456 Birch St, Nashville TN".to_string(),
        contractor_name: "Premier Renovations Inc".to_string(),
        line_items: vec![
            RenovationLineItem {
                room: "Kitchen".to_string(),
                description: "Full gut renovation with custom cabinetry".to_string(),
                material_cost_cents: 2800000,
                labor_cost_cents: 1500000,
                timeline_days: 45,
            },
            RenovationLineItem {
                room: "Master Bath".to_string(),
                description: "Tile replacement and vanity upgrade".to_string(),
                material_cost_cents: 800000,
                labor_cost_cents: 600000,
                timeline_days: 21,
            },
            RenovationLineItem {
                room: "Exterior".to_string(),
                description: "New siding and paint".to_string(),
                material_cost_cents: 1200000,
                labor_cost_cents: 900000,
                timeline_days: 14,
            },
        ],
        total_cost_cents: 7800000,
        contingency_pct: 10.0,
        valid_until: "2026-06-15".to_string(),
    };
    let cfg = config::standard();
    let encoded = encode_to_vec(&estimate, cfg).expect("encode renovation estimate");
    let (decoded, _): (RenovationCostEstimate, _) =
        decode_owned_from_slice(&encoded, cfg).expect("decode renovation estimate");
    assert_eq!(estimate, decoded);
}

#[test]
fn test_smart_home_iot_config_roundtrip() {
    let config_data = SmartHomeIotConfig {
        property_id: 10050,
        hub_model: "SmartThings Hub v3".to_string(),
        wifi_ssid: "HomeNet-5G".to_string(),
        devices: vec![
            SmartHomeDevice {
                device_id: "THERM-001".to_string(),
                device_type: "Thermostat".to_string(),
                manufacturer: "Ecobee".to_string(),
                firmware_version: "4.8.2".to_string(),
                is_online: true,
            },
            SmartHomeDevice {
                device_id: "LOCK-001".to_string(),
                device_type: "Smart Lock".to_string(),
                manufacturer: "August".to_string(),
                firmware_version: "3.1.0".to_string(),
                is_online: true,
            },
            SmartHomeDevice {
                device_id: "CAM-001".to_string(),
                device_type: "Security Camera".to_string(),
                manufacturer: "Ring".to_string(),
                firmware_version: "2.5.7".to_string(),
                is_online: false,
            },
        ],
        automation_rules_count: 12,
        monthly_data_usage_mb: 4500,
    };
    let cfg = config::standard();
    let encoded = encode_to_vec(&config_data, cfg).expect("encode IoT config");
    let (decoded, _): (SmartHomeIotConfig, _) =
        decode_owned_from_slice(&encoded, cfg).expect("decode IoT config");
    assert_eq!(config_data, decoded);
}

#[test]
fn test_rental_yield_calculation_roundtrip() {
    let yield_calc = RentalYieldCalculation {
        property_id: 30020,
        address: "789 Pine St, Unit 4, Seattle WA".to_string(),
        purchase_price_cents: 45000000,
        monthly_rent_cents: 250000,
        annual_expenses_cents: 600000,
        vacancy_rate_bps: 500,
        gross_yield_bps: 667,
        net_yield_bps: 533,
        cap_rate_bps: 510,
        cash_on_cash_bps: 820,
    };
    let cfg = config::standard();
    let encoded = encode_to_vec(&yield_calc, cfg).expect("encode rental yield");
    let (decoded, _): (RentalYieldCalculation, _) =
        decode_owned_from_slice(&encoded, cfg).expect("decode rental yield");
    assert_eq!(yield_calc, decoded);
}

#[test]
fn test_building_permit_application_roundtrip() {
    let permit = BuildingPermitApplication {
        permit_id: 112200,
        applicant_name: "Green Build Co".to_string(),
        property_address: "900 Industrial Pkwy, Phoenix AZ".to_string(),
        permit_type: "Commercial Renovation".to_string(),
        project_description: "Convert warehouse to mixed-use retail and loft space".to_string(),
        estimated_cost_cents: 250000000,
        submission_date: "2026-01-15".to_string(),
        status: "Under Review".to_string(),
        reviewer_notes: vec![
            "Fire suppression plan required".to_string(),
            "ADA compliance documentation pending".to_string(),
        ],
        approved: false,
    };
    let cfg = config::standard();
    let encoded = encode_to_vec(&permit, cfg).expect("encode building permit");
    let (decoded, _): (BuildingPermitApplication, _) =
        decode_owned_from_slice(&encoded, cfg).expect("decode building permit");
    assert_eq!(permit, decoded);
}

#[test]
fn test_property_portfolio_roundtrip() {
    let portfolio = PropertyPortfolio {
        owner_name: "Horizon Investments LLC".to_string(),
        listings: vec![
            PropertyListing {
                listing_id: 20001,
                address: "100 First Ave".to_string(),
                city: "Austin".to_string(),
                state: "TX".to_string(),
                zip_code: "78701".to_string(),
                sqft: 1800,
                bedrooms: 3,
                bathrooms: 2.0,
                list_price_cents: 38000000,
                year_built: 2005,
                property_type: "Condo".to_string(),
                is_active: true,
            },
            PropertyListing {
                listing_id: 20002,
                address: "250 Second St".to_string(),
                city: "Austin".to_string(),
                state: "TX".to_string(),
                zip_code: "78702".to_string(),
                sqft: 3200,
                bedrooms: 5,
                bathrooms: 3.5,
                list_price_cents: 67500000,
                year_built: 2018,
                property_type: "Single Family".to_string(),
                is_active: false,
            },
        ],
        total_value_cents: 105500000,
        total_monthly_income_cents: 520000,
    };
    let cfg = config::standard();
    let encoded = encode_to_vec(&portfolio, cfg).expect("encode portfolio");
    let (decoded, _): (PropertyPortfolio, _) =
        decode_owned_from_slice(&encoded, cfg).expect("decode portfolio");
    assert_eq!(portfolio, decoded);
}

#[test]
fn test_open_house_event_roundtrip() {
    let event = OpenHouseEvent {
        event_id: 55001,
        listing_id: 10001,
        property_address: "742 Evergreen Terrace, Springfield IL".to_string(),
        date: "2026-03-22".to_string(),
        start_time: "13:00".to_string(),
        end_time: "16:00".to_string(),
        agent_name: "Lisa Chang".to_string(),
        rsvp_count: 18,
        visitor_comments: vec![
            "Beautiful backyard".to_string(),
            "Kitchen could use updating".to_string(),
            "Great school district".to_string(),
        ],
    };
    let cfg = config::standard();
    let encoded = encode_to_vec(&event, cfg).expect("encode open house");
    let (decoded, _): (OpenHouseEvent, _) =
        decode_owned_from_slice(&encoded, cfg).expect("decode open house");
    assert_eq!(event, decoded);
}

#[test]
fn test_property_insurance_quote_roundtrip() {
    let quote = PropertyInsuranceQuote {
        quote_id: 880001,
        property_address: "44 Coastal Dr, Galveston TX".to_string(),
        coverage_type: "HO-3 Special Form".to_string(),
        dwelling_coverage_cents: 50000000,
        liability_coverage_cents: 30000000,
        annual_premium_cents: 285000,
        deductible_cents: 250000,
        flood_zone: true,
        earthquake_rider: false,
    };
    let cfg = config::standard();
    let encoded = encode_to_vec(&quote, cfg).expect("encode insurance quote");
    let (decoded, _): (PropertyInsuranceQuote, _) =
        decode_owned_from_slice(&encoded, cfg).expect("decode insurance quote");
    assert_eq!(quote, decoded);
}

#[test]
fn test_empty_comparables_cma_roundtrip() {
    let cma = ComparableMarketAnalysis {
        subject_address: "1 New Construction Way".to_string(),
        analysis_date: "2026-03-15".to_string(),
        comparables: vec![],
        estimated_value_cents: 0,
        confidence_pct: 0.0,
    };
    let cfg = config::standard();
    let encoded = encode_to_vec(&cma, cfg).expect("encode empty CMA");
    let (decoded, _): (ComparableMarketAnalysis, _) =
        decode_owned_from_slice(&encoded, cfg).expect("decode empty CMA");
    assert_eq!(cma, decoded);
}

#[test]
fn test_title_search_clear_title_roundtrip() {
    let title = TitleSearchResult {
        search_id: 990050,
        property_address: "300 Clean Title Rd, Boise ID".to_string(),
        current_owner: "Perfect Record Holdings".to_string(),
        liens: vec![],
        easements: vec![],
        encumbrances: vec![],
        title_clear: true,
        search_date: "2026-03-12".to_string(),
    };
    let cfg = config::standard();
    let encoded = encode_to_vec(&title, cfg).expect("encode clear title");
    let (decoded, _): (TitleSearchResult, _) =
        decode_owned_from_slice(&encoded, cfg).expect("decode clear title");
    assert_eq!(title, decoded);
}

#[test]
fn test_month_to_month_lease_roundtrip() {
    let lease = LeaseAgreement {
        lease_id: 77050,
        tenant_name: "Freelancer Sam".to_string(),
        property_address: "Studio 5, 80 Arts District".to_string(),
        start_date: "2026-03-01".to_string(),
        end_date: "".to_string(),
        monthly_rent_cents: 145000,
        security_deposit_cents: 145000,
        pet_deposit_cents: 0,
        is_month_to_month: true,
        clauses: vec![
            "30-day notice required for termination".to_string(),
            "Rent increases with 60-day written notice".to_string(),
        ],
    };
    let cfg = config::standard();
    let encoded = encode_to_vec(&lease, cfg).expect("encode month-to-month lease");
    let (decoded, _): (LeaseAgreement, _) =
        decode_owned_from_slice(&encoded, cfg).expect("decode month-to-month lease");
    assert_eq!(lease, decoded);
}

#[test]
fn test_mixed_zoning_overlay_roundtrip() {
    let zoning = ZoningClassification {
        parcel_id: "APN-2026-0780-003".to_string(),
        zone_code: "MU-1".to_string(),
        zone_description: "Mixed Use - Neighborhood Center".to_string(),
        allowed_uses: vec![
            "Retail".to_string(),
            "Restaurant".to_string(),
            "Office".to_string(),
            "Residential above ground floor".to_string(),
            "Live-work units".to_string(),
            "Public assembly".to_string(),
        ],
        max_building_height_ft: 55,
        max_lot_coverage_pct: 80.0,
        min_setback_front_ft: 0,
        min_setback_side_ft: 0,
        is_overlay_district: true,
    };
    let cfg = config::standard();
    let encoded = encode_to_vec(&zoning, cfg).expect("encode mixed-use zoning");
    let (decoded, _): (ZoningClassification, _) =
        decode_owned_from_slice(&encoded, cfg).expect("decode mixed-use zoning");
    assert_eq!(zoning, decoded);
}
