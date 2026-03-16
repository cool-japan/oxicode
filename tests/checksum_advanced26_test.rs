//! Advanced checksum tests for OxiCode — food safety and traceability theme.
//!
//! Compile with: cargo test --features checksum --test checksum_advanced26_test
//!
//! Exactly 22 `#[test]` functions.

#![cfg(feature = "checksum")]
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
use oxicode::checksum::{decode_with_checksum, encode_with_checksum, HEADER_SIZE};
use oxicode::{Decode, Encode};

// ---------------------------------------------------------------------------
// Domain types — food safety & traceability
// ---------------------------------------------------------------------------

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
enum HazardCategory {
    Biological,
    Chemical,
    Physical,
    Allergen,
    Radiological,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
enum CcpDecision {
    WithinLimits,
    CorrectiveActionRequired,
    DeviationRecorded,
    LineShutdown,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct HaccpCriticalControlPoint {
    ccp_id: String,
    process_step: String,
    hazard: HazardCategory,
    critical_limit_celsius: f64,
    measured_value: f64,
    decision: CcpDecision,
    monitoring_frequency_secs: u32,
    responsible_person: String,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct FarmToForkRecord {
    lot_number: String,
    farm_id: String,
    harvest_date: String,
    transport_vehicle: String,
    receiving_facility: String,
    processing_timestamp: u64,
    distribution_center: String,
    retail_store_id: String,
    gps_lat: f64,
    gps_lon: f64,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
enum AllergenPresence {
    Contains,
    MayContainTraces,
    FreeFrom,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct AllergenDeclaration {
    product_name: String,
    gluten: AllergenPresence,
    dairy: AllergenPresence,
    nuts: AllergenPresence,
    soy: AllergenPresence,
    eggs: AllergenPresence,
    fish: AllergenPresence,
    shellfish: AllergenPresence,
    sesame: AllergenPresence,
    regulatory_code: String,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct ColdChainTemperatureLog {
    sensor_serial: String,
    readings_celsius: Vec<f64>,
    timestamps_epoch: Vec<u64>,
    min_threshold: f64,
    max_threshold: f64,
    alert_triggered: bool,
    compartment_id: u16,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
enum PathogenType {
    Salmonella,
    EColi,
    Listeria,
    Campylobacter,
    Norovirus,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
enum TestOutcome {
    NotDetected,
    DetectedBelowLimit,
    DetectedAboveLimit,
    Inconclusive,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct PathogenTestResult {
    sample_id: String,
    pathogen: PathogenType,
    outcome: TestOutcome,
    cfu_per_gram: f64,
    lab_accreditation: String,
    analyst_id: String,
    method_reference: String,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct NutritionalPanel {
    serving_size_grams: f64,
    calories_kcal: f64,
    total_fat_g: f64,
    saturated_fat_g: f64,
    trans_fat_g: f64,
    cholesterol_mg: f64,
    sodium_mg: f64,
    total_carbs_g: f64,
    dietary_fiber_g: f64,
    sugars_g: f64,
    protein_g: f64,
    vitamin_d_mcg: f64,
    calcium_mg: f64,
    iron_mg: f64,
    potassium_mg: f64,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct IngredientSourcingRecord {
    ingredient_name: String,
    supplier_id: String,
    country_of_origin: String,
    organic_certified: bool,
    gmo_free: bool,
    arrival_date: String,
    certificate_numbers: Vec<String>,
    unit_cost_cents: u64,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
enum SanitationResult {
    Pass,
    Fail,
    RequiresRetest,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct SanitationLog {
    line_id: String,
    sanitizer_chemical: String,
    concentration_ppm: f64,
    contact_time_secs: u32,
    surface_atp_rlu: u32,
    result: SanitationResult,
    verified_by: String,
    timestamp_epoch: u64,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
enum RecallClass {
    ClassI,
    ClassII,
    ClassIII,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct ProductRecallNotice {
    recall_number: String,
    product_description: String,
    reason: String,
    recall_class: RecallClass,
    distribution_states: Vec<String>,
    affected_lot_numbers: Vec<String>,
    units_affected: u64,
    contact_phone: String,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct Gs1BarcodeData {
    gtin: String,
    batch_lot: String,
    production_date: String,
    expiration_date: String,
    serial_number: String,
    net_weight_kg: f64,
    country_code: u16,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct BatchExpirationTracker {
    batch_id: String,
    product_name: String,
    production_epoch: u64,
    best_before_epoch: u64,
    use_by_epoch: u64,
    remaining_units: u32,
    warehouse_zone: String,
    fifo_position: u16,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct CertificationRecord {
    cert_type: String,
    certifying_body: String,
    certificate_number: String,
    issue_date: String,
    expiry_date: String,
    scope_description: String,
    audit_score: f64,
    non_conformities: u16,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct FoodContactMaterialCompliance {
    material_type: String,
    regulation_reference: String,
    migration_limit_mg_per_kg: f64,
    actual_migration: f64,
    compliant: bool,
    test_lab: String,
    report_number: String,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct WaterQualityTest {
    source_id: String,
    ph_value: f64,
    turbidity_ntu: f64,
    total_coliforms_per_100ml: u32,
    ecoli_per_100ml: u32,
    chlorine_residual_ppm: f64,
    lead_ppb: f64,
    arsenic_ppb: f64,
    compliant: bool,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
enum PestType {
    Rodent,
    Insect,
    Bird,
    Other,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct PestControlInspection {
    facility_zone: String,
    inspector_id: String,
    pest_type_found: Vec<PestType>,
    traps_checked: u16,
    traps_active: u16,
    evidence_found: bool,
    corrective_actions: Vec<String>,
    next_inspection_epoch: u64,
}

// ---------------------------------------------------------------------------
// Test 1: HACCP critical control point roundtrip
// ---------------------------------------------------------------------------
#[test]
fn test_haccp_ccp_roundtrip() {
    let ccp = HaccpCriticalControlPoint {
        ccp_id: "CCP-001-PASTEURIZE".into(),
        process_step: "Milk pasteurization".into(),
        hazard: HazardCategory::Biological,
        critical_limit_celsius: 72.0,
        measured_value: 73.5,
        decision: CcpDecision::WithinLimits,
        monitoring_frequency_secs: 15,
        responsible_person: "J. Tanaka".into(),
    };
    let encoded = encode_with_checksum(&ccp).expect("encode HACCP CCP failed");
    let (decoded, consumed): (HaccpCriticalControlPoint, _) =
        decode_with_checksum(&encoded).expect("decode HACCP CCP failed");
    assert_eq!(decoded, ccp);
    assert_eq!(consumed, encoded.len());
}

// ---------------------------------------------------------------------------
// Test 2: Farm-to-fork traceability roundtrip
// ---------------------------------------------------------------------------
#[test]
fn test_farm_to_fork_roundtrip() {
    let record = FarmToForkRecord {
        lot_number: "LOT-2026-03-15-A".into(),
        farm_id: "FARM-JP-0421".into(),
        harvest_date: "2026-03-10".into(),
        transport_vehicle: "TRK-8832".into(),
        receiving_facility: "PLANT-OSAKA-02".into(),
        processing_timestamp: 1_741_900_800,
        distribution_center: "DC-KOBE-01".into(),
        retail_store_id: "STORE-7711".into(),
        gps_lat: 34.6937,
        gps_lon: 135.5023,
    };
    let encoded = encode_with_checksum(&record).expect("encode farm-to-fork failed");
    let (decoded, consumed): (FarmToForkRecord, _) =
        decode_with_checksum(&encoded).expect("decode farm-to-fork failed");
    assert_eq!(decoded, record);
    assert_eq!(consumed, encoded.len());
}

// ---------------------------------------------------------------------------
// Test 3: Allergen declaration roundtrip
// ---------------------------------------------------------------------------
#[test]
fn test_allergen_declaration_roundtrip() {
    let decl = AllergenDeclaration {
        product_name: "Premium Dark Chocolate Bar 85%".into(),
        gluten: AllergenPresence::FreeFrom,
        dairy: AllergenPresence::Contains,
        nuts: AllergenPresence::MayContainTraces,
        soy: AllergenPresence::Contains,
        eggs: AllergenPresence::FreeFrom,
        fish: AllergenPresence::FreeFrom,
        shellfish: AllergenPresence::FreeFrom,
        sesame: AllergenPresence::FreeFrom,
        regulatory_code: "EU-1169/2011".into(),
    };
    let encoded = encode_with_checksum(&decl).expect("encode allergen decl failed");
    let (decoded, consumed): (AllergenDeclaration, _) =
        decode_with_checksum(&encoded).expect("decode allergen decl failed");
    assert_eq!(decoded, decl);
    assert_eq!(consumed, encoded.len());
}

// ---------------------------------------------------------------------------
// Test 4: Cold chain temperature log roundtrip
// ---------------------------------------------------------------------------
#[test]
fn test_cold_chain_temperature_log_roundtrip() {
    let log = ColdChainTemperatureLog {
        sensor_serial: "TEMP-SN-44920".into(),
        readings_celsius: vec![-18.2, -18.0, -17.8, -18.1, -19.0, -18.5, -17.5, -18.3],
        timestamps_epoch: vec![
            1_741_900_000,
            1_741_900_300,
            1_741_900_600,
            1_741_900_900,
            1_741_901_200,
            1_741_901_500,
            1_741_901_800,
            1_741_902_100,
        ],
        min_threshold: -22.0,
        max_threshold: -15.0,
        alert_triggered: false,
        compartment_id: 3,
    };
    let encoded = encode_with_checksum(&log).expect("encode cold chain log failed");
    let (decoded, consumed): (ColdChainTemperatureLog, _) =
        decode_with_checksum(&encoded).expect("decode cold chain log failed");
    assert_eq!(decoded, log);
    assert_eq!(consumed, encoded.len());
}

// ---------------------------------------------------------------------------
// Test 5: Pathogen test result — Salmonella not detected
// ---------------------------------------------------------------------------
#[test]
fn test_pathogen_salmonella_not_detected_roundtrip() {
    let result = PathogenTestResult {
        sample_id: "SAM-2026-0315-001".into(),
        pathogen: PathogenType::Salmonella,
        outcome: TestOutcome::NotDetected,
        cfu_per_gram: 0.0,
        lab_accreditation: "ISO-17025-JP-0044".into(),
        analyst_id: "ANALYST-K-112".into(),
        method_reference: "ISO 6579-1:2017".into(),
    };
    let encoded = encode_with_checksum(&result).expect("encode salmonella test failed");
    let (decoded, consumed): (PathogenTestResult, _) =
        decode_with_checksum(&encoded).expect("decode salmonella test failed");
    assert_eq!(decoded, result);
    assert_eq!(consumed, encoded.len());
}

// ---------------------------------------------------------------------------
// Test 6: Pathogen test result — E. coli detected above limit
// ---------------------------------------------------------------------------
#[test]
fn test_pathogen_ecoli_above_limit_roundtrip() {
    let result = PathogenTestResult {
        sample_id: "SAM-2026-0315-002".into(),
        pathogen: PathogenType::EColi,
        outcome: TestOutcome::DetectedAboveLimit,
        cfu_per_gram: 1500.0,
        lab_accreditation: "ISO-17025-JP-0044".into(),
        analyst_id: "ANALYST-M-207".into(),
        method_reference: "ISO 16649-2:2001".into(),
    };
    let encoded = encode_with_checksum(&result).expect("encode ecoli test failed");
    let (decoded, consumed): (PathogenTestResult, _) =
        decode_with_checksum(&encoded).expect("decode ecoli test failed");
    assert_eq!(decoded, result);
    assert_eq!(consumed, encoded.len());
}

// ---------------------------------------------------------------------------
// Test 7: Pathogen test result — Listeria inconclusive
// ---------------------------------------------------------------------------
#[test]
fn test_pathogen_listeria_inconclusive_roundtrip() {
    let result = PathogenTestResult {
        sample_id: "SAM-2026-0315-003".into(),
        pathogen: PathogenType::Listeria,
        outcome: TestOutcome::Inconclusive,
        cfu_per_gram: 10.0,
        lab_accreditation: "ISO-17025-JP-0044".into(),
        analyst_id: "ANALYST-S-019".into(),
        method_reference: "ISO 11290-1:2017".into(),
    };
    let encoded = encode_with_checksum(&result).expect("encode listeria test failed");
    let (decoded, consumed): (PathogenTestResult, _) =
        decode_with_checksum(&encoded).expect("decode listeria test failed");
    assert_eq!(decoded, result);
    assert_eq!(consumed, encoded.len());
}

// ---------------------------------------------------------------------------
// Test 8: Nutritional analysis panel roundtrip
// ---------------------------------------------------------------------------
#[test]
fn test_nutritional_panel_roundtrip() {
    let panel = NutritionalPanel {
        serving_size_grams: 240.0,
        calories_kcal: 150.0,
        total_fat_g: 8.0,
        saturated_fat_g: 1.0,
        trans_fat_g: 0.0,
        cholesterol_mg: 0.0,
        sodium_mg: 160.0,
        total_carbs_g: 17.0,
        dietary_fiber_g: 4.0,
        sugars_g: 1.0,
        protein_g: 3.0,
        vitamin_d_mcg: 2.0,
        calcium_mg: 260.0,
        iron_mg: 6.0,
        potassium_mg: 235.0,
    };
    let encoded = encode_with_checksum(&panel).expect("encode nutritional panel failed");
    let (decoded, consumed): (NutritionalPanel, _) =
        decode_with_checksum(&encoded).expect("decode nutritional panel failed");
    assert_eq!(decoded, panel);
    assert_eq!(consumed, encoded.len());
}

// ---------------------------------------------------------------------------
// Test 9: Ingredient sourcing record roundtrip
// ---------------------------------------------------------------------------
#[test]
fn test_ingredient_sourcing_roundtrip() {
    let record = IngredientSourcingRecord {
        ingredient_name: "Organic Matcha Powder".into(),
        supplier_id: "SUP-JP-UJI-0012".into(),
        country_of_origin: "Japan".into(),
        organic_certified: true,
        gmo_free: true,
        arrival_date: "2026-03-12".into(),
        certificate_numbers: vec!["JAS-ORG-2026-00441".into(), "USDA-NOP-2026-11203".into()],
        unit_cost_cents: 4850,
    };
    let encoded = encode_with_checksum(&record).expect("encode ingredient sourcing failed");
    let (decoded, consumed): (IngredientSourcingRecord, _) =
        decode_with_checksum(&encoded).expect("decode ingredient sourcing failed");
    assert_eq!(decoded, record);
    assert_eq!(consumed, encoded.len());
}

// ---------------------------------------------------------------------------
// Test 10: Production line sanitation log roundtrip
// ---------------------------------------------------------------------------
#[test]
fn test_sanitation_log_roundtrip() {
    let log = SanitationLog {
        line_id: "LINE-04-FILL".into(),
        sanitizer_chemical: "Peracetic acid".into(),
        concentration_ppm: 200.0,
        contact_time_secs: 600,
        surface_atp_rlu: 25,
        result: SanitationResult::Pass,
        verified_by: "QA-LEAD-Suzuki".into(),
        timestamp_epoch: 1_741_910_400,
    };
    let encoded = encode_with_checksum(&log).expect("encode sanitation log failed");
    let (decoded, consumed): (SanitationLog, _) =
        decode_with_checksum(&encoded).expect("decode sanitation log failed");
    assert_eq!(decoded, log);
    assert_eq!(consumed, encoded.len());
}

// ---------------------------------------------------------------------------
// Test 11: Product recall notice roundtrip
// ---------------------------------------------------------------------------
#[test]
fn test_product_recall_notice_roundtrip() {
    let recall = ProductRecallNotice {
        recall_number: "F-2026-0315-0001".into(),
        product_description: "Frozen Edamame 500g".into(),
        reason: "Undeclared soy allergen on English-language label".into(),
        recall_class: RecallClass::ClassI,
        distribution_states: vec!["California".into(), "Oregon".into(), "Washington".into()],
        affected_lot_numbers: vec![
            "LOT-2026-02-28-A".into(),
            "LOT-2026-02-28-B".into(),
            "LOT-2026-03-01-A".into(),
        ],
        units_affected: 48_000,
        contact_phone: "+1-800-555-0199".into(),
    };
    let encoded = encode_with_checksum(&recall).expect("encode recall notice failed");
    let (decoded, consumed): (ProductRecallNotice, _) =
        decode_with_checksum(&encoded).expect("decode recall notice failed");
    assert_eq!(decoded, recall);
    assert_eq!(consumed, encoded.len());
}

// ---------------------------------------------------------------------------
// Test 12: GS1 barcode data roundtrip
// ---------------------------------------------------------------------------
#[test]
fn test_gs1_barcode_data_roundtrip() {
    let barcode = Gs1BarcodeData {
        gtin: "04912345678904".into(),
        batch_lot: "LOT-2026-0315".into(),
        production_date: "2026-03-15".into(),
        expiration_date: "2027-03-15".into(),
        serial_number: "SN-00000012345".into(),
        net_weight_kg: 0.500,
        country_code: 49,
    };
    let encoded = encode_with_checksum(&barcode).expect("encode GS1 barcode failed");
    let (decoded, consumed): (Gs1BarcodeData, _) =
        decode_with_checksum(&encoded).expect("decode GS1 barcode failed");
    assert_eq!(decoded, barcode);
    assert_eq!(consumed, encoded.len());
}

// ---------------------------------------------------------------------------
// Test 13: Batch expiration tracking roundtrip
// ---------------------------------------------------------------------------
#[test]
fn test_batch_expiration_tracking_roundtrip() {
    let tracker = BatchExpirationTracker {
        batch_id: "BATCH-2026-0310-RYE".into(),
        product_name: "Artisan Rye Bread 750g".into(),
        production_epoch: 1_741_564_800,
        best_before_epoch: 1_742_169_600,
        use_by_epoch: 1_742_428_800,
        remaining_units: 320,
        warehouse_zone: "ZONE-C-AMBIENT".into(),
        fifo_position: 7,
    };
    let encoded = encode_with_checksum(&tracker).expect("encode batch tracker failed");
    let (decoded, consumed): (BatchExpirationTracker, _) =
        decode_with_checksum(&encoded).expect("decode batch tracker failed");
    assert_eq!(decoded, tracker);
    assert_eq!(consumed, encoded.len());
}

// ---------------------------------------------------------------------------
// Test 14: Organic certification roundtrip
// ---------------------------------------------------------------------------
#[test]
fn test_organic_certification_roundtrip() {
    let cert = CertificationRecord {
        cert_type: "Organic".into(),
        certifying_body: "JAS (Japanese Agricultural Standard)".into(),
        certificate_number: "JAS-ORG-2025-08812".into(),
        issue_date: "2025-09-01".into(),
        expiry_date: "2026-08-31".into(),
        scope_description: "Green tea cultivation, processing, and packaging".into(),
        audit_score: 96.5,
        non_conformities: 1,
    };
    let encoded = encode_with_checksum(&cert).expect("encode organic cert failed");
    let (decoded, consumed): (CertificationRecord, _) =
        decode_with_checksum(&encoded).expect("decode organic cert failed");
    assert_eq!(decoded, cert);
    assert_eq!(consumed, encoded.len());
}

// ---------------------------------------------------------------------------
// Test 15: Halal certification roundtrip
// ---------------------------------------------------------------------------
#[test]
fn test_halal_certification_roundtrip() {
    let cert = CertificationRecord {
        cert_type: "Halal".into(),
        certifying_body: "JAKIM (Malaysia)".into(),
        certificate_number: "JAKIM-HAL-2025-33091".into(),
        issue_date: "2025-06-15".into(),
        expiry_date: "2027-06-14".into(),
        scope_description: "Frozen seafood products - shrimp and squid".into(),
        audit_score: 99.0,
        non_conformities: 0,
    };
    let encoded = encode_with_checksum(&cert).expect("encode halal cert failed");
    let (decoded, consumed): (CertificationRecord, _) =
        decode_with_checksum(&encoded).expect("decode halal cert failed");
    assert_eq!(decoded, cert);
    assert_eq!(consumed, encoded.len());
}

// ---------------------------------------------------------------------------
// Test 16: Food contact material compliance roundtrip
// ---------------------------------------------------------------------------
#[test]
fn test_food_contact_material_compliance_roundtrip() {
    let fcm = FoodContactMaterialCompliance {
        material_type: "Polypropylene food container".into(),
        regulation_reference: "EU 10/2011 (FCM Plastics)".into(),
        migration_limit_mg_per_kg: 10.0,
        actual_migration: 2.3,
        compliant: true,
        test_lab: "SGS Tokyo Laboratory".into(),
        report_number: "SGS-FCM-2026-04421".into(),
    };
    let encoded = encode_with_checksum(&fcm).expect("encode FCM compliance failed");
    let (decoded, consumed): (FoodContactMaterialCompliance, _) =
        decode_with_checksum(&encoded).expect("decode FCM compliance failed");
    assert_eq!(decoded, fcm);
    assert_eq!(consumed, encoded.len());
}

// ---------------------------------------------------------------------------
// Test 17: Water quality testing roundtrip
// ---------------------------------------------------------------------------
#[test]
fn test_water_quality_roundtrip() {
    let water = WaterQualityTest {
        source_id: "WELL-FACILITY-MAIN".into(),
        ph_value: 7.2,
        turbidity_ntu: 0.3,
        total_coliforms_per_100ml: 0,
        ecoli_per_100ml: 0,
        chlorine_residual_ppm: 0.5,
        lead_ppb: 1.2,
        arsenic_ppb: 0.8,
        compliant: true,
    };
    let encoded = encode_with_checksum(&water).expect("encode water quality failed");
    let (decoded, consumed): (WaterQualityTest, _) =
        decode_with_checksum(&encoded).expect("decode water quality failed");
    assert_eq!(decoded, water);
    assert_eq!(consumed, encoded.len());
}

// ---------------------------------------------------------------------------
// Test 18: Pest control inspection roundtrip
// ---------------------------------------------------------------------------
#[test]
fn test_pest_control_inspection_roundtrip() {
    let inspection = PestControlInspection {
        facility_zone: "Dry storage warehouse B".into(),
        inspector_id: "PCO-JP-0088".into(),
        pest_type_found: vec![PestType::Insect],
        traps_checked: 24,
        traps_active: 22,
        evidence_found: true,
        corrective_actions: vec![
            "Replaced 2 damaged bait stations".into(),
            "Sealed gap under loading dock door #3".into(),
            "Scheduled follow-up in 7 days".into(),
        ],
        next_inspection_epoch: 1_742_505_600,
    };
    let encoded = encode_with_checksum(&inspection).expect("encode pest control failed");
    let (decoded, consumed): (PestControlInspection, _) =
        decode_with_checksum(&encoded).expect("decode pest control failed");
    assert_eq!(decoded, inspection);
    assert_eq!(consumed, encoded.len());
}

// ---------------------------------------------------------------------------
// Test 19: Corruption detection — flip byte in HACCP payload
// ---------------------------------------------------------------------------
#[test]
fn test_corruption_haccp_payload_flip() {
    let ccp = HaccpCriticalControlPoint {
        ccp_id: "CCP-003-METAL-DET".into(),
        process_step: "Metal detection after packaging".into(),
        hazard: HazardCategory::Physical,
        critical_limit_celsius: 0.0,
        measured_value: 0.0,
        decision: CcpDecision::WithinLimits,
        monitoring_frequency_secs: 1,
        responsible_person: "K. Yamamoto".into(),
    };
    let mut encoded = encode_with_checksum(&ccp).expect("encode HACCP CCP failed");
    // Flip a byte in the payload region
    let flip_idx = HEADER_SIZE + 2;
    if flip_idx < encoded.len() {
        encoded[flip_idx] ^= 0xFF;
    }
    let result = decode_with_checksum::<HaccpCriticalControlPoint>(&encoded);
    assert!(
        result.is_err(),
        "checksum must detect corruption in HACCP payload"
    );
}

// ---------------------------------------------------------------------------
// Test 20: Corruption detection — flip byte in allergen declaration
// ---------------------------------------------------------------------------
#[test]
fn test_corruption_allergen_declaration_flip() {
    let decl = AllergenDeclaration {
        product_name: "Soba Noodles 300g".into(),
        gluten: AllergenPresence::Contains,
        dairy: AllergenPresence::FreeFrom,
        nuts: AllergenPresence::Contains,
        soy: AllergenPresence::Contains,
        eggs: AllergenPresence::FreeFrom,
        fish: AllergenPresence::FreeFrom,
        shellfish: AllergenPresence::MayContainTraces,
        sesame: AllergenPresence::Contains,
        regulatory_code: "CODEX-STAN-1-1985".into(),
    };
    let mut encoded = encode_with_checksum(&decl).expect("encode allergen failed");
    // Flip the last byte of payload
    let last_idx = encoded.len() - 1;
    encoded[last_idx] ^= 0xAA;
    let result = decode_with_checksum::<AllergenDeclaration>(&encoded);
    assert!(
        result.is_err(),
        "checksum must detect corruption in allergen payload"
    );
}

// ---------------------------------------------------------------------------
// Test 21: Corruption detection — flip byte in recall notice
// ---------------------------------------------------------------------------
#[test]
fn test_corruption_recall_notice_flip() {
    let recall = ProductRecallNotice {
        recall_number: "F-2026-0315-0002".into(),
        product_description: "Frozen Gyoza 24-pack".into(),
        reason: "Foreign material (plastic fragment) found".into(),
        recall_class: RecallClass::ClassII,
        distribution_states: vec!["Tokyo".into(), "Osaka".into()],
        affected_lot_numbers: vec!["LOT-2026-03-05-C".into()],
        units_affected: 12_000,
        contact_phone: "+81-120-555-0100".into(),
    };
    let mut encoded = encode_with_checksum(&recall).expect("encode recall failed");
    // Flip a byte in the middle of payload
    let mid_idx = HEADER_SIZE + (encoded.len() - HEADER_SIZE) / 2;
    if mid_idx < encoded.len() {
        encoded[mid_idx] ^= 0x42;
    }
    let result = decode_with_checksum::<ProductRecallNotice>(&encoded);
    assert!(
        result.is_err(),
        "checksum must detect corruption in recall payload"
    );
}

// ---------------------------------------------------------------------------
// Test 22: Corruption detection — flip byte in cold chain log
// ---------------------------------------------------------------------------
#[test]
fn test_corruption_cold_chain_flip() {
    let log = ColdChainTemperatureLog {
        sensor_serial: "TEMP-SN-99012".into(),
        readings_celsius: vec![-20.0, -19.5, -19.8, -20.1],
        timestamps_epoch: vec![1_741_900_000, 1_741_900_300, 1_741_900_600, 1_741_900_900],
        min_threshold: -25.0,
        max_threshold: -15.0,
        alert_triggered: false,
        compartment_id: 1,
    };
    let mut encoded = encode_with_checksum(&log).expect("encode cold chain failed");
    // Flip first payload byte
    let flip_idx = HEADER_SIZE;
    if flip_idx < encoded.len() {
        encoded[flip_idx] ^= 0xFF;
    }
    let result = decode_with_checksum::<ColdChainTemperatureLog>(&encoded);
    assert!(
        result.is_err(),
        "checksum must detect corruption in cold chain payload"
    );
}
