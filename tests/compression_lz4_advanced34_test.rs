//! Advanced LZ4 compression tests for the food safety management and HACCP compliance domain.
//!
//! Covers HACCP critical control points, temperature monitoring logs (cold chain),
//! allergen declarations, batch traceability records, sanitation verification schedules,
//! microbiological test results, supplier audit scores, product recall procedures,
//! shelf life studies, GMO labeling classifications, nutritional analysis panels,
//! water activity measurements, metal detector check results, pest control inspection records,
//! and related food safety management concepts.

#![cfg(feature = "compression-lz4")]
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

#[derive(Debug, PartialEq, Encode, Decode)]
enum HazardCategory {
    Biological {
        organism: String,
    },
    Chemical {
        substance: String,
        cas_number: String,
    },
    Physical {
        contaminant: String,
        size_mm: f32,
    },
    Radiological {
        isotope: String,
    },
    Allergen {
        allergen_name: String,
    },
}

#[derive(Debug, PartialEq, Encode, Decode)]
enum ControlMeasure {
    CookingTemperature {
        target_celsius: f64,
        hold_seconds: u32,
    },
    Refrigeration {
        max_celsius: f64,
    },
    MetalDetection {
        sensitivity_mm: f32,
    },
    ChemicalTreatment {
        agent: String,
        concentration_ppm: f64,
    },
    Filtration {
        pore_size_micron: f32,
    },
    PasteurizationHtst {
        temp_celsius: f64,
        hold_seconds: u32,
    },
}

#[derive(Debug, PartialEq, Encode, Decode)]
struct CriticalControlPoint {
    ccp_id: u32,
    process_step: String,
    hazard: HazardCategory,
    control_measure: ControlMeasure,
    critical_limit_description: String,
    monitoring_frequency_minutes: u32,
    corrective_action: String,
    verification_method: String,
}

#[derive(Debug, PartialEq, Encode, Decode)]
struct TemperatureReading {
    sensor_id: u32,
    timestamp_epoch: u64,
    celsius: f64,
    within_spec: bool,
}

#[derive(Debug, PartialEq, Encode, Decode)]
struct ColdChainLog {
    shipment_id: String,
    product_name: String,
    required_min_celsius: f64,
    required_max_celsius: f64,
    readings: Vec<TemperatureReading>,
    chain_broken: bool,
}

#[derive(Debug, PartialEq, Encode, Decode)]
enum AllergenPresence {
    Contains,
    MayContainTraces,
    FreeFrom,
    NotApplicable,
}

#[derive(Debug, PartialEq, Encode, Decode)]
struct AllergenDeclaration {
    product_code: String,
    gluten: AllergenPresence,
    crustaceans: AllergenPresence,
    eggs: AllergenPresence,
    fish: AllergenPresence,
    peanuts: AllergenPresence,
    soybeans: AllergenPresence,
    milk: AllergenPresence,
    tree_nuts: AllergenPresence,
    sesame: AllergenPresence,
    sulphites: AllergenPresence,
    lupin: AllergenPresence,
    molluscs: AllergenPresence,
    additional_notes: String,
}

#[derive(Debug, PartialEq, Encode, Decode)]
struct BatchTraceabilityRecord {
    batch_number: String,
    production_date_epoch: u64,
    expiry_date_epoch: u64,
    raw_material_lots: Vec<String>,
    supplier_ids: Vec<u32>,
    production_line: String,
    quantity_kg: f64,
    destination_codes: Vec<String>,
}

#[derive(Debug, PartialEq, Encode, Decode)]
enum SanitationResult {
    Pass,
    ConditionalPass { remark: String },
    Fail { deficiency: String },
}

#[derive(Debug, PartialEq, Encode, Decode)]
struct SanitationVerification {
    zone_id: u32,
    zone_name: String,
    inspection_epoch: u64,
    inspector_name: String,
    atp_rlu_reading: u32,
    acceptable_rlu_limit: u32,
    result: SanitationResult,
    surfaces_swabbed: Vec<String>,
}

#[derive(Debug, PartialEq, Encode, Decode)]
enum MicrobiologicalOutcome {
    BelowDetectionLimit,
    WithinSpec {
        cfu_per_g: f64,
    },
    ExceedsSpec {
        cfu_per_g: f64,
        action_taken: String,
    },
}

#[derive(Debug, PartialEq, Encode, Decode)]
struct MicrobiologicalTestResult {
    sample_id: String,
    product_name: String,
    organism_tested: String,
    method: String,
    outcome: MicrobiologicalOutcome,
    incubation_hours: u32,
    lab_accreditation: String,
}

#[derive(Debug, PartialEq, Encode, Decode)]
struct SupplierAuditScore {
    supplier_id: u32,
    supplier_name: String,
    audit_date_epoch: u64,
    food_safety_score: u8,
    quality_score: u8,
    documentation_score: u8,
    traceability_score: u8,
    overall_grade: String,
    non_conformances: Vec<String>,
    approved: bool,
}

#[derive(Debug, PartialEq, Encode, Decode)]
enum RecallClass {
    ClassI,
    ClassII,
    ClassIII,
}

#[derive(Debug, PartialEq, Encode, Decode)]
struct ProductRecallProcedure {
    recall_id: String,
    product_name: String,
    affected_batches: Vec<String>,
    recall_class: RecallClass,
    reason: String,
    distribution_channels: Vec<String>,
    units_affected: u64,
    public_notification_required: bool,
    contact_authority: String,
}

#[derive(Debug, PartialEq, Encode, Decode)]
struct ShelfLifeStudy {
    study_id: String,
    product_code: String,
    storage_temp_celsius: f64,
    humidity_percent: f64,
    test_intervals_days: Vec<u32>,
    sensory_scores: Vec<f64>,
    microbial_counts: Vec<f64>,
    ph_values: Vec<f64>,
    determined_shelf_life_days: u32,
    safety_margin_days: u32,
}

#[derive(Debug, PartialEq, Encode, Decode)]
enum GmoClassification {
    NonGmo,
    GmoFree { certification_body: String },
    ContainsGmo { percentage: f64 },
    DerivedFromGmo { source_organism: String },
    Unknown,
}

#[derive(Debug, PartialEq, Encode, Decode)]
struct GmoLabelingRecord {
    product_code: String,
    classification: GmoClassification,
    testing_method: String,
    detection_threshold_percent: f64,
    certificate_number: String,
}

#[derive(Debug, PartialEq, Encode, Decode)]
struct NutrientValue {
    name: String,
    per_100g: f64,
    per_serving: f64,
    unit: String,
    daily_value_percent: f64,
}

#[derive(Debug, PartialEq, Encode, Decode)]
struct NutritionalAnalysisPanel {
    product_code: String,
    serving_size_g: f64,
    servings_per_container: f64,
    nutrients: Vec<NutrientValue>,
    analysis_lab: String,
    analysis_date_epoch: u64,
}

#[derive(Debug, PartialEq, Encode, Decode)]
struct WaterActivityMeasurement {
    sample_id: String,
    product_name: String,
    aw_value: f64,
    temperature_celsius: f64,
    instrument_model: String,
    calibration_date_epoch: u64,
    within_spec: bool,
    spec_max_aw: f64,
}

#[derive(Debug, PartialEq, Encode, Decode)]
enum MetalDetectorResult {
    Clear,
    Rejected {
        metal_type: String,
        estimated_size_mm: f32,
    },
    CalibrationCheck {
        ferrous_ok: bool,
        non_ferrous_ok: bool,
        stainless_ok: bool,
    },
}

#[derive(Debug, PartialEq, Encode, Decode)]
struct MetalDetectorCheckRecord {
    detector_id: u32,
    line_name: String,
    check_epoch: u64,
    operator_name: String,
    product_being_run: String,
    result: MetalDetectorResult,
    belt_speed_mpm: f64,
    sensitivity_fe_mm: f32,
    sensitivity_nfe_mm: f32,
    sensitivity_ss_mm: f32,
}

#[derive(Debug, PartialEq, Encode, Decode)]
enum PestType {
    Rodent { species: String },
    FlyingInsect { species: String },
    CrawlingInsect { species: String },
    StoredProductPest { species: String },
    Bird,
    Other { description: String },
}

#[derive(Debug, PartialEq, Encode, Decode)]
enum PestFinding {
    NoneDetected,
    ActivityDetected {
        pest: PestType,
        severity: u8,
        location: String,
    },
    TrapCaught {
        pest: PestType,
        trap_id: String,
    },
}

#[derive(Debug, PartialEq, Encode, Decode)]
struct PestControlInspection {
    inspection_id: u32,
    facility_name: String,
    inspection_epoch: u64,
    inspector_company: String,
    traps_checked: u32,
    bait_stations_checked: u32,
    findings: Vec<PestFinding>,
    recommendations: Vec<String>,
    next_visit_epoch: u64,
}

#[derive(Debug, PartialEq, Encode, Decode)]
struct CorrosiveCleaningAgent {
    chemical_name: String,
    concentration_percent: f64,
    contact_time_minutes: u32,
    rinse_required: bool,
}

#[derive(Debug, PartialEq, Encode, Decode)]
struct CleaningInPlaceRecord {
    cip_cycle_id: u64,
    equipment_name: String,
    pre_rinse_temp_celsius: f64,
    caustic_wash: CorrosiveCleaningAgent,
    acid_wash: CorrosiveCleaningAgent,
    final_rinse_conductivity_us: f64,
    sanitizer_concentration_ppm: f64,
    total_cycle_minutes: u32,
    passed_visual_inspection: bool,
}

#[derive(Debug, PartialEq, Encode, Decode)]
enum ForeignBodyType {
    Glass,
    Metal,
    Plastic,
    Wood,
    Stone,
    Bone,
    Insect,
    Hair,
    Other { description: String },
}

#[derive(Debug, PartialEq, Encode, Decode)]
struct ForeignBodyIncident {
    incident_id: String,
    product_code: String,
    batch_number: String,
    foreign_body: ForeignBodyType,
    size_mm: f32,
    detected_at_stage: String,
    detection_method: String,
    root_cause: String,
    corrective_actions: Vec<String>,
}

#[derive(Debug, PartialEq, Encode, Decode)]
struct EnvironmentalMonitoringSample {
    sample_id: String,
    zone: u8,
    location_description: String,
    target_organism: String,
    collection_epoch: u64,
    detected: bool,
    cfu_count: Option<f64>,
    follow_up_required: bool,
}

// ---------------------------------------------------------------------------
// Tests
// ---------------------------------------------------------------------------

#[test]
fn test_lz4_haccp_critical_control_point_roundtrip() {
    let ccp = CriticalControlPoint {
        ccp_id: 1,
        process_step: "Cooking / Pasteurization".to_string(),
        hazard: HazardCategory::Biological {
            organism: "Salmonella spp.".to_string(),
        },
        control_measure: ControlMeasure::CookingTemperature {
            target_celsius: 72.0,
            hold_seconds: 15,
        },
        critical_limit_description: "Core temperature >= 72C for >= 15 seconds".to_string(),
        monitoring_frequency_minutes: 30,
        corrective_action: "Re-cook product; hold and re-test".to_string(),
        verification_method: "Calibrated digital thermometer cross-check".to_string(),
    };

    let encoded = encode_to_vec(&ccp).expect("encode CriticalControlPoint failed");
    let compressed =
        compress(&encoded, Compression::Lz4).expect("compress CriticalControlPoint failed");
    let decompressed = decompress(&compressed).expect("decompress CriticalControlPoint failed");
    let (decoded, _): (CriticalControlPoint, usize) =
        decode_from_slice(&decompressed).expect("decode CriticalControlPoint failed");

    assert_eq!(ccp, decoded);
}

#[test]
fn test_lz4_cold_chain_log_roundtrip() {
    let log = ColdChainLog {
        shipment_id: "SHP-2026-03-0042".to_string(),
        product_name: "Fresh Atlantic Salmon Fillets".to_string(),
        required_min_celsius: -2.0,
        required_max_celsius: 4.0,
        readings: vec![
            TemperatureReading {
                sensor_id: 1,
                timestamp_epoch: 1710000000,
                celsius: 2.1,
                within_spec: true,
            },
            TemperatureReading {
                sensor_id: 1,
                timestamp_epoch: 1710003600,
                celsius: 2.3,
                within_spec: true,
            },
            TemperatureReading {
                sensor_id: 1,
                timestamp_epoch: 1710007200,
                celsius: 3.8,
                within_spec: true,
            },
            TemperatureReading {
                sensor_id: 1,
                timestamp_epoch: 1710010800,
                celsius: 5.1,
                within_spec: false,
            },
            TemperatureReading {
                sensor_id: 1,
                timestamp_epoch: 1710014400,
                celsius: 3.2,
                within_spec: true,
            },
        ],
        chain_broken: true,
    };

    let encoded = encode_to_vec(&log).expect("encode ColdChainLog failed");
    let compressed = compress(&encoded, Compression::Lz4).expect("compress ColdChainLog failed");
    let decompressed = decompress(&compressed).expect("decompress ColdChainLog failed");
    let (decoded, _): (ColdChainLog, usize) =
        decode_from_slice(&decompressed).expect("decode ColdChainLog failed");

    assert_eq!(log, decoded);
}

#[test]
fn test_lz4_allergen_declaration_roundtrip() {
    let decl = AllergenDeclaration {
        product_code: "PRD-CHOC-HAZEL-500".to_string(),
        gluten: AllergenPresence::MayContainTraces,
        crustaceans: AllergenPresence::FreeFrom,
        eggs: AllergenPresence::Contains,
        fish: AllergenPresence::FreeFrom,
        peanuts: AllergenPresence::MayContainTraces,
        soybeans: AllergenPresence::Contains,
        milk: AllergenPresence::Contains,
        tree_nuts: AllergenPresence::Contains,
        sesame: AllergenPresence::FreeFrom,
        sulphites: AllergenPresence::NotApplicable,
        lupin: AllergenPresence::FreeFrom,
        molluscs: AllergenPresence::FreeFrom,
        additional_notes: "Produced in a facility that also processes wheat and peanuts"
            .to_string(),
    };

    let encoded = encode_to_vec(&decl).expect("encode AllergenDeclaration failed");
    let compressed =
        compress(&encoded, Compression::Lz4).expect("compress AllergenDeclaration failed");
    let decompressed = decompress(&compressed).expect("decompress AllergenDeclaration failed");
    let (decoded, _): (AllergenDeclaration, usize) =
        decode_from_slice(&decompressed).expect("decode AllergenDeclaration failed");

    assert_eq!(decl, decoded);
}

#[test]
fn test_lz4_batch_traceability_record_roundtrip() {
    let batch = BatchTraceabilityRecord {
        batch_number: "B-2026-0315-007".to_string(),
        production_date_epoch: 1710460800,
        expiry_date_epoch: 1713139200,
        raw_material_lots: vec![
            "RM-FLOUR-2026-0201".to_string(),
            "RM-SUGAR-2026-0215".to_string(),
            "RM-EGGS-2026-0314".to_string(),
        ],
        supplier_ids: vec![101, 204, 307],
        production_line: "Line-3A".to_string(),
        quantity_kg: 2500.0,
        destination_codes: vec![
            "DC-TALLINN-01".to_string(),
            "DC-RIGA-02".to_string(),
            "DC-VILNIUS-03".to_string(),
        ],
    };

    let encoded = encode_to_vec(&batch).expect("encode BatchTraceabilityRecord failed");
    let compressed =
        compress(&encoded, Compression::Lz4).expect("compress BatchTraceabilityRecord failed");
    let decompressed = decompress(&compressed).expect("decompress BatchTraceabilityRecord failed");
    let (decoded, _): (BatchTraceabilityRecord, usize) =
        decode_from_slice(&decompressed).expect("decode BatchTraceabilityRecord failed");

    assert_eq!(batch, decoded);
}

#[test]
fn test_lz4_sanitation_verification_pass_roundtrip() {
    let sv = SanitationVerification {
        zone_id: 12,
        zone_name: "Ready-to-eat packaging area".to_string(),
        inspection_epoch: 1710500000,
        inspector_name: "M. Koppel".to_string(),
        atp_rlu_reading: 85,
        acceptable_rlu_limit: 150,
        result: SanitationResult::Pass,
        surfaces_swabbed: vec![
            "Conveyor belt surface".to_string(),
            "Packaging machine hopper".to_string(),
            "Operator glove contact point".to_string(),
        ],
    };

    let encoded = encode_to_vec(&sv).expect("encode SanitationVerification failed");
    let compressed =
        compress(&encoded, Compression::Lz4).expect("compress SanitationVerification failed");
    let decompressed = decompress(&compressed).expect("decompress SanitationVerification failed");
    let (decoded, _): (SanitationVerification, usize) =
        decode_from_slice(&decompressed).expect("decode SanitationVerification failed");

    assert_eq!(sv, decoded);
}

#[test]
fn test_lz4_sanitation_verification_fail_roundtrip() {
    let sv = SanitationVerification {
        zone_id: 5,
        zone_name: "Raw meat processing floor drain".to_string(),
        inspection_epoch: 1710510000,
        inspector_name: "J. Tamm".to_string(),
        atp_rlu_reading: 420,
        acceptable_rlu_limit: 200,
        result: SanitationResult::Fail {
            deficiency: "Biofilm detected on drain grate; inadequate pre-rinse".to_string(),
        },
        surfaces_swabbed: vec![
            "Floor drain grate".to_string(),
            "Drain channel interior".to_string(),
        ],
    };

    let encoded = encode_to_vec(&sv).expect("encode SanitationVerification fail failed");
    let compressed =
        compress(&encoded, Compression::Lz4).expect("compress SanitationVerification fail failed");
    let decompressed =
        decompress(&compressed).expect("decompress SanitationVerification fail failed");
    let (decoded, _): (SanitationVerification, usize) =
        decode_from_slice(&decompressed).expect("decode SanitationVerification fail failed");

    assert_eq!(sv, decoded);
}

#[test]
fn test_lz4_microbiological_test_below_limit_roundtrip() {
    let result = MicrobiologicalTestResult {
        sample_id: "MICRO-2026-03-0088".to_string(),
        product_name: "Vanilla Ice Cream 1L".to_string(),
        organism_tested: "Listeria monocytogenes".to_string(),
        method: "ISO 11290-1:2017".to_string(),
        outcome: MicrobiologicalOutcome::BelowDetectionLimit,
        incubation_hours: 48,
        lab_accreditation: "ISO 17025 - Lab EST-0042".to_string(),
    };

    let encoded = encode_to_vec(&result).expect("encode MicrobiologicalTestResult failed");
    let compressed =
        compress(&encoded, Compression::Lz4).expect("compress MicrobiologicalTestResult failed");
    let decompressed =
        decompress(&compressed).expect("decompress MicrobiologicalTestResult failed");
    let (decoded, _): (MicrobiologicalTestResult, usize) =
        decode_from_slice(&decompressed).expect("decode MicrobiologicalTestResult failed");

    assert_eq!(result, decoded);
}

#[test]
fn test_lz4_microbiological_test_exceeds_spec_roundtrip() {
    let result = MicrobiologicalTestResult {
        sample_id: "MICRO-2026-03-0112".to_string(),
        product_name: "Minced Beef 500g".to_string(),
        organism_tested: "E. coli O157:H7".to_string(),
        method: "ISO 16654:2001".to_string(),
        outcome: MicrobiologicalOutcome::ExceedsSpec {
            cfu_per_g: 1500.0,
            action_taken: "Product held, batch quarantined, re-sampling scheduled".to_string(),
        },
        incubation_hours: 24,
        lab_accreditation: "ISO 17025 - Lab EST-0042".to_string(),
    };

    let encoded = encode_to_vec(&result).expect("encode MicrobiologicalTestResult exceeds failed");
    let compressed = compress(&encoded, Compression::Lz4)
        .expect("compress MicrobiologicalTestResult exceeds failed");
    let decompressed =
        decompress(&compressed).expect("decompress MicrobiologicalTestResult exceeds failed");
    let (decoded, _): (MicrobiologicalTestResult, usize) =
        decode_from_slice(&decompressed).expect("decode MicrobiologicalTestResult exceeds failed");

    assert_eq!(result, decoded);
}

#[test]
fn test_lz4_supplier_audit_score_roundtrip() {
    let audit = SupplierAuditScore {
        supplier_id: 5501,
        supplier_name: "Nordic Dairy Cooperative".to_string(),
        audit_date_epoch: 1710288000,
        food_safety_score: 92,
        quality_score: 88,
        documentation_score: 95,
        traceability_score: 90,
        overall_grade: "A".to_string(),
        non_conformances: vec!["Minor: Pest control log missing one entry in January".to_string()],
        approved: true,
    };

    let encoded = encode_to_vec(&audit).expect("encode SupplierAuditScore failed");
    let compressed =
        compress(&encoded, Compression::Lz4).expect("compress SupplierAuditScore failed");
    let decompressed = decompress(&compressed).expect("decompress SupplierAuditScore failed");
    let (decoded, _): (SupplierAuditScore, usize) =
        decode_from_slice(&decompressed).expect("decode SupplierAuditScore failed");

    assert_eq!(audit, decoded);
}

#[test]
fn test_lz4_product_recall_procedure_roundtrip() {
    let recall = ProductRecallProcedure {
        recall_id: "RCL-2026-EE-0003".to_string(),
        product_name: "Organic Peanut Butter 350g".to_string(),
        affected_batches: vec![
            "B-2026-0210-001".to_string(),
            "B-2026-0210-002".to_string(),
            "B-2026-0211-001".to_string(),
        ],
        recall_class: RecallClass::ClassI,
        reason: "Undeclared milk allergen due to cross-contamination on shared production line"
            .to_string(),
        distribution_channels: vec![
            "Retail - Prisma stores".to_string(),
            "Retail - Selver stores".to_string(),
            "E-commerce - Barbora".to_string(),
        ],
        units_affected: 12_400,
        public_notification_required: true,
        contact_authority: "Estonian Veterinary and Food Board (VTA)".to_string(),
    };

    let encoded = encode_to_vec(&recall).expect("encode ProductRecallProcedure failed");
    let compressed =
        compress(&encoded, Compression::Lz4).expect("compress ProductRecallProcedure failed");
    let decompressed = decompress(&compressed).expect("decompress ProductRecallProcedure failed");
    let (decoded, _): (ProductRecallProcedure, usize) =
        decode_from_slice(&decompressed).expect("decode ProductRecallProcedure failed");

    assert_eq!(recall, decoded);
}

#[test]
fn test_lz4_shelf_life_study_roundtrip() {
    let study = ShelfLifeStudy {
        study_id: "SLS-2026-042".to_string(),
        product_code: "PRD-YOGURT-STRAW-200".to_string(),
        storage_temp_celsius: 4.0,
        humidity_percent: 75.0,
        test_intervals_days: vec![0, 7, 14, 21, 28, 35],
        sensory_scores: vec![9.2, 8.8, 8.1, 7.4, 6.5, 4.9],
        microbial_counts: vec![10.0, 50.0, 200.0, 800.0, 3500.0, 15000.0],
        ph_values: vec![4.2, 4.2, 4.1, 4.0, 3.9, 3.7],
        determined_shelf_life_days: 28,
        safety_margin_days: 7,
    };

    let encoded = encode_to_vec(&study).expect("encode ShelfLifeStudy failed");
    let compressed = compress(&encoded, Compression::Lz4).expect("compress ShelfLifeStudy failed");
    let decompressed = decompress(&compressed).expect("decompress ShelfLifeStudy failed");
    let (decoded, _): (ShelfLifeStudy, usize) =
        decode_from_slice(&decompressed).expect("decode ShelfLifeStudy failed");

    assert_eq!(study, decoded);
}

#[test]
fn test_lz4_gmo_labeling_record_roundtrip() {
    let record = GmoLabelingRecord {
        product_code: "PRD-SOY-MILK-1L".to_string(),
        classification: GmoClassification::GmoFree {
            certification_body: "Non-GMO Project Verified".to_string(),
        },
        testing_method: "Real-time PCR screening for CaMV 35S and NOS terminator".to_string(),
        detection_threshold_percent: 0.9,
        certificate_number: "NGP-EU-2026-88421".to_string(),
    };

    let encoded = encode_to_vec(&record).expect("encode GmoLabelingRecord failed");
    let compressed =
        compress(&encoded, Compression::Lz4).expect("compress GmoLabelingRecord failed");
    let decompressed = decompress(&compressed).expect("decompress GmoLabelingRecord failed");
    let (decoded, _): (GmoLabelingRecord, usize) =
        decode_from_slice(&decompressed).expect("decode GmoLabelingRecord failed");

    assert_eq!(record, decoded);
}

#[test]
fn test_lz4_nutritional_analysis_panel_roundtrip() {
    let panel = NutritionalAnalysisPanel {
        product_code: "PRD-GRANOLA-BAR-40".to_string(),
        serving_size_g: 40.0,
        servings_per_container: 6.0,
        nutrients: vec![
            NutrientValue {
                name: "Energy (kcal)".to_string(),
                per_100g: 452.0,
                per_serving: 180.8,
                unit: "kcal".to_string(),
                daily_value_percent: 9.0,
            },
            NutrientValue {
                name: "Total Fat".to_string(),
                per_100g: 18.5,
                per_serving: 7.4,
                unit: "g".to_string(),
                daily_value_percent: 11.4,
            },
            NutrientValue {
                name: "Saturated Fat".to_string(),
                per_100g: 5.2,
                per_serving: 2.08,
                unit: "g".to_string(),
                daily_value_percent: 10.4,
            },
            NutrientValue {
                name: "Carbohydrate".to_string(),
                per_100g: 62.0,
                per_serving: 24.8,
                unit: "g".to_string(),
                daily_value_percent: 9.5,
            },
            NutrientValue {
                name: "Sugars".to_string(),
                per_100g: 28.0,
                per_serving: 11.2,
                unit: "g".to_string(),
                daily_value_percent: 0.0,
            },
            NutrientValue {
                name: "Dietary Fibre".to_string(),
                per_100g: 6.3,
                per_serving: 2.52,
                unit: "g".to_string(),
                daily_value_percent: 10.1,
            },
            NutrientValue {
                name: "Protein".to_string(),
                per_100g: 8.1,
                per_serving: 3.24,
                unit: "g".to_string(),
                daily_value_percent: 6.5,
            },
            NutrientValue {
                name: "Salt".to_string(),
                per_100g: 0.35,
                per_serving: 0.14,
                unit: "g".to_string(),
                daily_value_percent: 2.3,
            },
        ],
        analysis_lab: "Eurofins Scientific Estonia".to_string(),
        analysis_date_epoch: 1710374400,
    };

    let encoded = encode_to_vec(&panel).expect("encode NutritionalAnalysisPanel failed");
    let compressed =
        compress(&encoded, Compression::Lz4).expect("compress NutritionalAnalysisPanel failed");
    let decompressed = decompress(&compressed).expect("decompress NutritionalAnalysisPanel failed");
    let (decoded, _): (NutritionalAnalysisPanel, usize) =
        decode_from_slice(&decompressed).expect("decode NutritionalAnalysisPanel failed");

    assert_eq!(panel, decoded);
}

#[test]
fn test_lz4_water_activity_measurement_roundtrip() {
    let measurement = WaterActivityMeasurement {
        sample_id: "AW-2026-03-0055".to_string(),
        product_name: "Beef Jerky Original 80g".to_string(),
        aw_value: 0.82,
        temperature_celsius: 25.0,
        instrument_model: "Rotronic HygroLab C1".to_string(),
        calibration_date_epoch: 1709683200,
        within_spec: true,
        spec_max_aw: 0.85,
    };

    let encoded = encode_to_vec(&measurement).expect("encode WaterActivityMeasurement failed");
    let compressed =
        compress(&encoded, Compression::Lz4).expect("compress WaterActivityMeasurement failed");
    let decompressed = decompress(&compressed).expect("decompress WaterActivityMeasurement failed");
    let (decoded, _): (WaterActivityMeasurement, usize) =
        decode_from_slice(&decompressed).expect("decode WaterActivityMeasurement failed");

    assert_eq!(measurement, decoded);
}

#[test]
fn test_lz4_metal_detector_clear_roundtrip() {
    let check = MetalDetectorCheckRecord {
        detector_id: 3,
        line_name: "Cereal Packing Line 2".to_string(),
        check_epoch: 1710520000,
        operator_name: "R. Lepp".to_string(),
        product_being_run: "Whole Grain Oat Flakes 500g".to_string(),
        result: MetalDetectorResult::Clear,
        belt_speed_mpm: 45.0,
        sensitivity_fe_mm: 1.5,
        sensitivity_nfe_mm: 2.0,
        sensitivity_ss_mm: 2.5,
    };

    let encoded = encode_to_vec(&check).expect("encode MetalDetectorCheckRecord clear failed");
    let compressed = compress(&encoded, Compression::Lz4)
        .expect("compress MetalDetectorCheckRecord clear failed");
    let decompressed =
        decompress(&compressed).expect("decompress MetalDetectorCheckRecord clear failed");
    let (decoded, _): (MetalDetectorCheckRecord, usize) =
        decode_from_slice(&decompressed).expect("decode MetalDetectorCheckRecord clear failed");

    assert_eq!(check, decoded);
}

#[test]
fn test_lz4_metal_detector_rejected_roundtrip() {
    let check = MetalDetectorCheckRecord {
        detector_id: 3,
        line_name: "Cereal Packing Line 2".to_string(),
        check_epoch: 1710521800,
        operator_name: "R. Lepp".to_string(),
        product_being_run: "Whole Grain Oat Flakes 500g".to_string(),
        result: MetalDetectorResult::Rejected {
            metal_type: "Ferrous".to_string(),
            estimated_size_mm: 3.2,
        },
        belt_speed_mpm: 45.0,
        sensitivity_fe_mm: 1.5,
        sensitivity_nfe_mm: 2.0,
        sensitivity_ss_mm: 2.5,
    };

    let encoded = encode_to_vec(&check).expect("encode MetalDetectorCheckRecord rejected failed");
    let compressed = compress(&encoded, Compression::Lz4)
        .expect("compress MetalDetectorCheckRecord rejected failed");
    let decompressed =
        decompress(&compressed).expect("decompress MetalDetectorCheckRecord rejected failed");
    let (decoded, _): (MetalDetectorCheckRecord, usize) =
        decode_from_slice(&decompressed).expect("decode MetalDetectorCheckRecord rejected failed");

    assert_eq!(check, decoded);
}

#[test]
fn test_lz4_pest_control_no_findings_roundtrip() {
    let inspection = PestControlInspection {
        inspection_id: 441,
        facility_name: "Tallinn Fresh Produce Warehouse".to_string(),
        inspection_epoch: 1710460800,
        inspector_company: "Anticimex Estonia".to_string(),
        traps_checked: 48,
        bait_stations_checked: 24,
        findings: vec![PestFinding::NoneDetected],
        recommendations: vec![
            "Seal gap under loading dock door 3".to_string(),
            "Replace worn door brush strip on cold store entrance".to_string(),
        ],
        next_visit_epoch: 1712880000,
    };

    let encoded = encode_to_vec(&inspection).expect("encode PestControlInspection failed");
    let compressed =
        compress(&encoded, Compression::Lz4).expect("compress PestControlInspection failed");
    let decompressed = decompress(&compressed).expect("decompress PestControlInspection failed");
    let (decoded, _): (PestControlInspection, usize) =
        decode_from_slice(&decompressed).expect("decode PestControlInspection failed");

    assert_eq!(inspection, decoded);
}

#[test]
fn test_lz4_pest_control_with_findings_roundtrip() {
    let inspection = PestControlInspection {
        inspection_id: 442,
        facility_name: "Tartu Bakery Production Hall".to_string(),
        inspection_epoch: 1710547200,
        inspector_company: "Anticimex Estonia".to_string(),
        traps_checked: 32,
        bait_stations_checked: 16,
        findings: vec![
            PestFinding::TrapCaught {
                pest: PestType::StoredProductPest {
                    species: "Plodia interpunctella (Indian meal moth)".to_string(),
                },
                trap_id: "PHT-B2-07".to_string(),
            },
            PestFinding::ActivityDetected {
                pest: PestType::CrawlingInsect {
                    species: "Blattella germanica (German cockroach)".to_string(),
                },
                severity: 2,
                location: "Behind flour silo base plate".to_string(),
            },
        ],
        recommendations: vec![
            "Deep clean flour silo area and apply gel bait treatment".to_string(),
            "Install additional pheromone traps near raw material intake".to_string(),
            "Increase monitoring frequency to weekly for next 4 weeks".to_string(),
        ],
        next_visit_epoch: 1711152000,
    };

    let encoded = encode_to_vec(&inspection).expect("encode PestControlInspection findings failed");
    let compressed = compress(&encoded, Compression::Lz4)
        .expect("compress PestControlInspection findings failed");
    let decompressed =
        decompress(&compressed).expect("decompress PestControlInspection findings failed");
    let (decoded, _): (PestControlInspection, usize) =
        decode_from_slice(&decompressed).expect("decode PestControlInspection findings failed");

    assert_eq!(inspection, decoded);
}

#[test]
fn test_lz4_cip_cleaning_record_roundtrip() {
    let cip = CleaningInPlaceRecord {
        cip_cycle_id: 98_201,
        equipment_name: "UHT Heat Exchanger Unit 2".to_string(),
        pre_rinse_temp_celsius: 50.0,
        caustic_wash: CorrosiveCleaningAgent {
            chemical_name: "Sodium Hydroxide (NaOH)".to_string(),
            concentration_percent: 1.5,
            contact_time_minutes: 20,
            rinse_required: true,
        },
        acid_wash: CorrosiveCleaningAgent {
            chemical_name: "Nitric Acid (HNO3)".to_string(),
            concentration_percent: 0.8,
            contact_time_minutes: 15,
            rinse_required: true,
        },
        final_rinse_conductivity_us: 12.5,
        sanitizer_concentration_ppm: 200.0,
        total_cycle_minutes: 75,
        passed_visual_inspection: true,
    };

    let encoded = encode_to_vec(&cip).expect("encode CleaningInPlaceRecord failed");
    let compressed =
        compress(&encoded, Compression::Lz4).expect("compress CleaningInPlaceRecord failed");
    let decompressed = decompress(&compressed).expect("decompress CleaningInPlaceRecord failed");
    let (decoded, _): (CleaningInPlaceRecord, usize) =
        decode_from_slice(&decompressed).expect("decode CleaningInPlaceRecord failed");

    assert_eq!(cip, decoded);
}

#[test]
fn test_lz4_foreign_body_incident_roundtrip() {
    let incident = ForeignBodyIncident {
        incident_id: "FBI-2026-0047".to_string(),
        product_code: "PRD-TOMATO-SOUP-400".to_string(),
        batch_number: "B-2026-0312-003".to_string(),
        foreign_body: ForeignBodyType::Glass,
        size_mm: 4.5,
        detected_at_stage: "Post-fill X-ray inspection".to_string(),
        detection_method: "X-ray detection system (Mettler Toledo)".to_string(),
        root_cause: "Chipped jar rim from upstream filling nozzle misalignment".to_string(),
        corrective_actions: vec![
            "Quarantine affected batch".to_string(),
            "Realign filling nozzle and verify with test runs".to_string(),
            "100% X-ray re-inspection of all jars from last 2 hours".to_string(),
            "Replace worn nozzle guide rails".to_string(),
        ],
    };

    let encoded = encode_to_vec(&incident).expect("encode ForeignBodyIncident failed");
    let compressed =
        compress(&encoded, Compression::Lz4).expect("compress ForeignBodyIncident failed");
    let decompressed = decompress(&compressed).expect("decompress ForeignBodyIncident failed");
    let (decoded, _): (ForeignBodyIncident, usize) =
        decode_from_slice(&decompressed).expect("decode ForeignBodyIncident failed");

    assert_eq!(incident, decoded);
}

#[test]
fn test_lz4_environmental_monitoring_roundtrip() {
    let sample = EnvironmentalMonitoringSample {
        sample_id: "ENV-2026-03-Z1-014".to_string(),
        zone: 1,
        location_description: "Slicer blade housing interior, RTE deli line".to_string(),
        target_organism: "Listeria monocytogenes".to_string(),
        collection_epoch: 1710504000,
        detected: false,
        cfu_count: None,
        follow_up_required: false,
    };

    let encoded = encode_to_vec(&sample).expect("encode EnvironmentalMonitoringSample failed");
    let compressed = compress(&encoded, Compression::Lz4)
        .expect("compress EnvironmentalMonitoringSample failed");
    let decompressed =
        decompress(&compressed).expect("decompress EnvironmentalMonitoringSample failed");
    let (decoded, _): (EnvironmentalMonitoringSample, usize) =
        decode_from_slice(&decompressed).expect("decode EnvironmentalMonitoringSample failed");

    assert_eq!(sample, decoded);
}

#[test]
fn test_lz4_haccp_chemical_hazard_with_metal_calibration_roundtrip() {
    let ccp = CriticalControlPoint {
        ccp_id: 7,
        process_step: "Metal Detection Post-Packaging".to_string(),
        hazard: HazardCategory::Physical {
            contaminant: "Metal fragments from worn blade".to_string(),
            size_mm: 2.0,
        },
        control_measure: ControlMeasure::MetalDetection {
            sensitivity_mm: 1.5,
        },
        critical_limit_description:
            "Reject any package with Fe >= 1.5mm, NFe >= 2.0mm, SS >= 2.5mm".to_string(),
        monitoring_frequency_minutes: 60,
        corrective_action: "Quarantine product since last good check; re-inspect 100%".to_string(),
        verification_method: "Pass certified test pieces at start, mid-shift, and end of shift"
            .to_string(),
    };

    let calibration_check = MetalDetectorCheckRecord {
        detector_id: 5,
        line_name: "Snack Bar Wrapping Line A".to_string(),
        check_epoch: 1710530000,
        operator_name: "K. Mets".to_string(),
        product_being_run: "Protein Bar Chocolate 60g".to_string(),
        result: MetalDetectorResult::CalibrationCheck {
            ferrous_ok: true,
            non_ferrous_ok: true,
            stainless_ok: true,
        },
        belt_speed_mpm: 60.0,
        sensitivity_fe_mm: 1.5,
        sensitivity_nfe_mm: 2.0,
        sensitivity_ss_mm: 2.5,
    };

    let encoded_ccp = encode_to_vec(&ccp).expect("encode CCP physical hazard failed");
    let compressed_ccp =
        compress(&encoded_ccp, Compression::Lz4).expect("compress CCP physical hazard failed");
    let decompressed_ccp =
        decompress(&compressed_ccp).expect("decompress CCP physical hazard failed");
    let (decoded_ccp, _): (CriticalControlPoint, usize) =
        decode_from_slice(&decompressed_ccp).expect("decode CCP physical hazard failed");
    assert_eq!(ccp, decoded_ccp);

    let encoded_cal = encode_to_vec(&calibration_check).expect("encode calibration check failed");
    let compressed_cal =
        compress(&encoded_cal, Compression::Lz4).expect("compress calibration check failed");
    let decompressed_cal =
        decompress(&compressed_cal).expect("decompress calibration check failed");
    let (decoded_cal, _): (MetalDetectorCheckRecord, usize) =
        decode_from_slice(&decompressed_cal).expect("decode calibration check failed");
    assert_eq!(calibration_check, decoded_cal);
}
