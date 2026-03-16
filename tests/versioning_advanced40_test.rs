#![cfg(feature = "versioning")]
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
use oxicode::{decode_from_slice, encode_to_vec, Decode, Encode};
use oxicode::{decode_versioned_value, encode_versioned_value};

// ── Domain types: Pharmaceutical Manufacturing & Drug Development ────────────

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
enum ClinicalTrialPhase {
    PhaseI,
    PhaseII,
    PhaseIII,
    PhaseIV,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
enum QcTestResult {
    Pass,
    Fail,
    OutOfSpec,
    Retest,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
enum AdverseEventSeverity {
    Mild,
    Moderate,
    Severe,
    LifeThreatening,
    Fatal,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
enum DeviationCategory {
    Minor,
    Major,
    Critical,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
enum CapaStatus {
    Open,
    InvestigationPending,
    RootCauseIdentified,
    CorrectiveActionInProgress,
    Closed,
    EffectivenessVerified,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
enum ShipmentTempZone {
    Ambient,
    ColdChain2to8,
    Frozen,
    UltraCold,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
enum FdaSubmissionType {
    Ind,
    Nda,
    Anda,
    Bla,
    SupplementalNda,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
enum DrugInteractionSeverity {
    Contraindicated,
    MajorAvoid,
    Moderate,
    MinorMonitor,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct DrugCompoundRecord {
    compound_id: u64,
    generic_name: String,
    brand_name: String,
    molecular_weight_x100: u32,
    cas_number: String,
    therapeutic_class: String,
    route_of_admin: String,
    active: bool,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct ClinicalTrialRecord {
    trial_id: u64,
    nct_number: String,
    compound_id: u64,
    phase: ClinicalTrialPhase,
    enrolled_subjects: u32,
    sites_count: u16,
    primary_endpoint: String,
    start_date_epoch: u64,
    expected_completion_epoch: u64,
    blinded: bool,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct BatchManufacturingRecord {
    batch_id: u64,
    product_code: String,
    lot_number: String,
    batch_size_mg: u64,
    yield_percent_x100: u16,
    manufacturing_date_epoch: u64,
    expiry_date_epoch: u64,
    room_temp_x10: i16,
    humidity_x10: u16,
    operator_id: String,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct QualityControlTest {
    test_id: u64,
    batch_id: u64,
    test_name: String,
    specification_min_x1000: u32,
    specification_max_x1000: u32,
    actual_value_x1000: u32,
    result: QcTestResult,
    analyst_id: String,
    instrument_id: String,
    test_date_epoch: u64,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct StabilityStudyData {
    study_id: u64,
    batch_id: u64,
    condition_label: String,
    time_point_months: u16,
    assay_percent_x100: u16,
    degradant_a_x1000: u32,
    degradant_b_x1000: u32,
    moisture_x100: u16,
    dissolution_x100: u16,
    appearance_pass: bool,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct FdaSubmissionMetadata {
    submission_id: u64,
    application_number: String,
    submission_type: FdaSubmissionType,
    compound_id: u64,
    sponsor_name: String,
    submission_date_epoch: u64,
    review_division: String,
    priority_review: bool,
    orphan_drug: bool,
    fast_track: bool,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct AdverseEventReport {
    report_id: u64,
    case_number: String,
    compound_id: u64,
    severity: AdverseEventSeverity,
    event_description: String,
    onset_date_epoch: u64,
    reporter_type: String,
    outcome: String,
    serious: bool,
    causality_assessment: String,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct PharmacokineticsParams {
    pk_id: u64,
    compound_id: u64,
    subject_id: String,
    cmax_x1000: u64,
    tmax_minutes: u32,
    auc_0_inf_x1000: u64,
    half_life_minutes: u32,
    clearance_x1000: u32,
    volume_distribution_x1000: u32,
    bioavailability_x100: u16,
    dose_mg_x100: u32,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct FormulationComposition {
    formulation_id: u64,
    product_name: String,
    dosage_form: String,
    active_ingredient_mg_x100: u32,
    excipient_1: String,
    excipient_1_mg_x100: u32,
    excipient_2: String,
    excipient_2_mg_x100: u32,
    coating_type: String,
    total_weight_mg_x100: u32,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct GmpComplianceChecklist {
    checklist_id: u64,
    facility_code: String,
    inspection_date_epoch: u64,
    documentation_ok: bool,
    equipment_qualified: bool,
    personnel_trained: bool,
    env_monitoring_ok: bool,
    cleaning_validation_ok: bool,
    deviation_count: u16,
    overall_compliant: bool,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct ColdChainShippingRecord {
    shipment_id: u64,
    batch_id: u64,
    origin_site: String,
    destination_site: String,
    temp_zone: ShipmentTempZone,
    min_temp_recorded_x10: i16,
    max_temp_recorded_x10: i16,
    excursion_minutes: u32,
    departure_epoch: u64,
    arrival_epoch: u64,
    product_integrity_ok: bool,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct DrugInteractionEntry {
    interaction_id: u64,
    drug_a_name: String,
    drug_b_name: String,
    severity: DrugInteractionSeverity,
    mechanism: String,
    clinical_effect: String,
    recommendation: String,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct BioequivalenceStudy {
    study_id: u64,
    test_product_id: u64,
    reference_product_id: u64,
    subjects_enrolled: u32,
    auc_ratio_x1000: u32,
    cmax_ratio_x1000: u32,
    lower_ci_90_x1000: u32,
    upper_ci_90_x1000: u32,
    bioequivalent: bool,
    crossover_design: bool,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct LabelClaimVerification {
    verification_id: u64,
    product_code: String,
    batch_id: u64,
    claimed_strength_mg_x100: u32,
    measured_strength_mg_x100: u32,
    percent_label_claim_x100: u16,
    within_spec: bool,
    analyst_id: String,
    method_id: String,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct DeviationCapaRecord {
    deviation_id: u64,
    batch_id: u64,
    category: DeviationCategory,
    description: String,
    root_cause: String,
    capa_status: CapaStatus,
    initiated_date_epoch: u64,
    target_closure_epoch: u64,
    actual_closure_epoch: Option<u64>,
    impact_assessment: String,
}

// ── Tests ────────────────────────────────────────────────────────────────────

#[test]
fn test_drug_compound_record_roundtrip() {
    let compound = DrugCompoundRecord {
        compound_id: 10001,
        generic_name: "Atorvastatin".to_string(),
        brand_name: "Lipitor".to_string(),
        molecular_weight_x100: 55886,
        cas_number: "134523-00-5".to_string(),
        therapeutic_class: "HMG-CoA reductase inhibitor".to_string(),
        route_of_admin: "Oral".to_string(),
        active: true,
    };
    let bytes = encode_to_vec(&compound).expect("encode DrugCompoundRecord failed");
    let (decoded, consumed) =
        decode_from_slice::<DrugCompoundRecord>(&bytes).expect("decode DrugCompoundRecord failed");
    assert_eq!(compound, decoded);
    assert_eq!(consumed, bytes.len());
}

#[test]
fn test_drug_compound_versioned_v1_0_0() {
    let compound = DrugCompoundRecord {
        compound_id: 10002,
        generic_name: "Metformin".to_string(),
        brand_name: "Glucophage".to_string(),
        molecular_weight_x100: 12916,
        cas_number: "657-24-9".to_string(),
        therapeutic_class: "Biguanide antidiabetic".to_string(),
        route_of_admin: "Oral".to_string(),
        active: true,
    };
    let version = Version::new(1, 0, 0);
    let bytes = encode_versioned_value(&compound, version)
        .expect("encode versioned DrugCompoundRecord v1.0.0 failed");
    let (decoded, ver, _consumed): (DrugCompoundRecord, Version, usize) =
        decode_versioned_value::<DrugCompoundRecord>(&bytes)
            .expect("decode versioned DrugCompoundRecord v1.0.0 failed");
    assert_eq!(compound, decoded);
    assert_eq!(ver.major, 1);
    assert_eq!(ver.minor, 0);
    assert_eq!(ver.patch, 0);
}

#[test]
fn test_clinical_trial_phase_iii_roundtrip() {
    let trial = ClinicalTrialRecord {
        trial_id: 20001,
        nct_number: "NCT05123456".to_string(),
        compound_id: 10001,
        phase: ClinicalTrialPhase::PhaseIII,
        enrolled_subjects: 3200,
        sites_count: 145,
        primary_endpoint: "LDL-C reduction from baseline at week 12".to_string(),
        start_date_epoch: 1_672_531_200,
        expected_completion_epoch: 1_735_689_600,
        blinded: true,
    };
    let bytes = encode_to_vec(&trial).expect("encode ClinicalTrialRecord PhaseIII failed");
    let (decoded, consumed) = decode_from_slice::<ClinicalTrialRecord>(&bytes)
        .expect("decode ClinicalTrialRecord PhaseIII failed");
    assert_eq!(trial, decoded);
    assert_eq!(consumed, bytes.len());
}

#[test]
fn test_clinical_trial_versioned_upgrade_v1_to_v2() {
    let trial = ClinicalTrialRecord {
        trial_id: 20002,
        nct_number: "NCT06789012".to_string(),
        compound_id: 10002,
        phase: ClinicalTrialPhase::PhaseI,
        enrolled_subjects: 48,
        sites_count: 3,
        primary_endpoint: "Safety and tolerability in healthy volunteers".to_string(),
        start_date_epoch: 1_680_307_200,
        expected_completion_epoch: 1_695_168_000,
        blinded: false,
    };
    let v1 = Version::new(1, 0, 0);
    let bytes_v1 =
        encode_versioned_value(&trial, v1).expect("encode ClinicalTrialRecord v1.0.0 failed");
    let (decoded_v1, ver_v1, consumed_v1): (ClinicalTrialRecord, Version, usize) =
        decode_versioned_value::<ClinicalTrialRecord>(&bytes_v1)
            .expect("decode ClinicalTrialRecord v1.0.0 failed");
    assert_eq!(trial, decoded_v1);
    assert_eq!(ver_v1.major, 1);

    let v2 = Version::new(2, 0, 0);
    let bytes_v2 =
        encode_versioned_value(&decoded_v1, v2).expect("encode ClinicalTrialRecord v2.0.0 failed");
    let (decoded_v2, ver_v2, consumed_v2): (ClinicalTrialRecord, Version, usize) =
        decode_versioned_value::<ClinicalTrialRecord>(&bytes_v2)
            .expect("decode ClinicalTrialRecord v2.0.0 failed");
    assert_eq!(trial, decoded_v2);
    assert_eq!(ver_v2.major, 2);
    assert!(consumed_v1 > 0);
    assert!(consumed_v2 > 0);
}

#[test]
fn test_batch_manufacturing_record_roundtrip() {
    let bmr = BatchManufacturingRecord {
        batch_id: 30001,
        product_code: "ATOR-10MG-TAB".to_string(),
        lot_number: "L2025-0142".to_string(),
        batch_size_mg: 500_000_000,
        yield_percent_x100: 9845,
        manufacturing_date_epoch: 1_706_745_600,
        expiry_date_epoch: 1_769_904_000,
        room_temp_x10: 220,
        humidity_x10: 450,
        operator_id: "OP-4521".to_string(),
    };
    let bytes = encode_to_vec(&bmr).expect("encode BatchManufacturingRecord failed");
    let (decoded, consumed) = decode_from_slice::<BatchManufacturingRecord>(&bytes)
        .expect("decode BatchManufacturingRecord failed");
    assert_eq!(bmr, decoded);
    assert_eq!(consumed, bytes.len());
}

#[test]
fn test_batch_manufacturing_versioned_v3_1_0() {
    let bmr = BatchManufacturingRecord {
        batch_id: 30002,
        product_code: "MET-500MG-TAB".to_string(),
        lot_number: "L2025-0287".to_string(),
        batch_size_mg: 2_000_000_000,
        yield_percent_x100: 9712,
        manufacturing_date_epoch: 1_709_424_000,
        expiry_date_epoch: 1_772_582_400,
        room_temp_x10: 215,
        humidity_x10: 420,
        operator_id: "OP-1133".to_string(),
    };
    let version = Version::new(3, 1, 0);
    let bytes = encode_versioned_value(&bmr, version).expect("encode versioned BMR v3.1.0 failed");
    let (decoded, ver, consumed): (BatchManufacturingRecord, Version, usize) =
        decode_versioned_value::<BatchManufacturingRecord>(&bytes)
            .expect("decode versioned BMR v3.1.0 failed");
    assert_eq!(bmr, decoded);
    assert_eq!(ver.major, 3);
    assert_eq!(ver.minor, 1);
    assert_eq!(ver.patch, 0);
    assert!(consumed > 0);
}

#[test]
fn test_quality_control_pass_roundtrip() {
    let qc = QualityControlTest {
        test_id: 40001,
        batch_id: 30001,
        test_name: "Assay by HPLC".to_string(),
        specification_min_x1000: 95000,
        specification_max_x1000: 105000,
        actual_value_x1000: 99800,
        result: QcTestResult::Pass,
        analyst_id: "AN-0078".to_string(),
        instrument_id: "HPLC-007".to_string(),
        test_date_epoch: 1_706_832_000,
    };
    let bytes = encode_to_vec(&qc).expect("encode QualityControlTest Pass failed");
    let (decoded, consumed) = decode_from_slice::<QualityControlTest>(&bytes)
        .expect("decode QualityControlTest Pass failed");
    assert_eq!(qc, decoded);
    assert_eq!(consumed, bytes.len());
}

#[test]
fn test_quality_control_oos_versioned() {
    let qc = QualityControlTest {
        test_id: 40002,
        batch_id: 30002,
        test_name: "Dissolution at 45 minutes".to_string(),
        specification_min_x1000: 80000,
        specification_max_x1000: 100000,
        actual_value_x1000: 74200,
        result: QcTestResult::OutOfSpec,
        analyst_id: "AN-0112".to_string(),
        instrument_id: "DISS-003".to_string(),
        test_date_epoch: 1_709_510_400,
    };
    let version = Version::new(1, 2, 0);
    let bytes =
        encode_versioned_value(&qc, version).expect("encode versioned QC OOS v1.2.0 failed");
    let (decoded, ver, _consumed): (QualityControlTest, Version, usize) =
        decode_versioned_value::<QualityControlTest>(&bytes)
            .expect("decode versioned QC OOS v1.2.0 failed");
    assert_eq!(qc, decoded);
    assert_eq!(ver.major, 1);
    assert_eq!(ver.minor, 2);
}

#[test]
fn test_stability_study_data_roundtrip() {
    let stability = StabilityStudyData {
        study_id: 50001,
        batch_id: 30001,
        condition_label: "25C/60%RH Long-term".to_string(),
        time_point_months: 12,
        assay_percent_x100: 9920,
        degradant_a_x1000: 150,
        degradant_b_x1000: 80,
        moisture_x100: 310,
        dissolution_x100: 9550,
        appearance_pass: true,
    };
    let bytes = encode_to_vec(&stability).expect("encode StabilityStudyData failed");
    let (decoded, consumed) =
        decode_from_slice::<StabilityStudyData>(&bytes).expect("decode StabilityStudyData failed");
    assert_eq!(stability, decoded);
    assert_eq!(consumed, bytes.len());
}

#[test]
fn test_stability_study_versioned_accelerated() {
    let stability = StabilityStudyData {
        study_id: 50002,
        batch_id: 30002,
        condition_label: "40C/75%RH Accelerated".to_string(),
        time_point_months: 6,
        assay_percent_x100: 9710,
        degradant_a_x1000: 480,
        degradant_b_x1000: 320,
        moisture_x100: 520,
        dissolution_x100: 8800,
        appearance_pass: true,
    };
    let version = Version::new(2, 1, 3);
    let bytes = encode_versioned_value(&stability, version)
        .expect("encode versioned StabilityStudy v2.1.3 failed");
    let (decoded, ver, consumed): (StabilityStudyData, Version, usize) =
        decode_versioned_value::<StabilityStudyData>(&bytes)
            .expect("decode versioned StabilityStudy v2.1.3 failed");
    assert_eq!(stability, decoded);
    assert_eq!(ver.major, 2);
    assert_eq!(ver.minor, 1);
    assert_eq!(ver.patch, 3);
    assert!(consumed > 0);
}

#[test]
fn test_fda_submission_nda_roundtrip() {
    let submission = FdaSubmissionMetadata {
        submission_id: 60001,
        application_number: "NDA-215432".to_string(),
        submission_type: FdaSubmissionType::Nda,
        compound_id: 10001,
        sponsor_name: "PharmaCorp International".to_string(),
        submission_date_epoch: 1_711_929_600,
        review_division: "Division of Cardiology".to_string(),
        priority_review: false,
        orphan_drug: false,
        fast_track: false,
    };
    let bytes = encode_to_vec(&submission).expect("encode FdaSubmissionMetadata NDA failed");
    let (decoded, consumed) = decode_from_slice::<FdaSubmissionMetadata>(&bytes)
        .expect("decode FdaSubmissionMetadata NDA failed");
    assert_eq!(submission, decoded);
    assert_eq!(consumed, bytes.len());
}

#[test]
fn test_fda_submission_orphan_versioned_v1_0_0() {
    let submission = FdaSubmissionMetadata {
        submission_id: 60002,
        application_number: "BLA-761299".to_string(),
        submission_type: FdaSubmissionType::Bla,
        compound_id: 10099,
        sponsor_name: "RareDis Therapeutics".to_string(),
        submission_date_epoch: 1_714_521_600,
        review_division: "Division of Rare Diseases".to_string(),
        priority_review: true,
        orphan_drug: true,
        fast_track: true,
    };
    let version = Version::new(1, 0, 0);
    let bytes = encode_versioned_value(&submission, version)
        .expect("encode versioned FDA BLA v1.0.0 failed");
    let (decoded, ver, _consumed): (FdaSubmissionMetadata, Version, usize) =
        decode_versioned_value::<FdaSubmissionMetadata>(&bytes)
            .expect("decode versioned FDA BLA v1.0.0 failed");
    assert_eq!(submission, decoded);
    assert_eq!(ver.major, 1);
}

#[test]
fn test_adverse_event_severe_roundtrip() {
    let ae = AdverseEventReport {
        report_id: 70001,
        case_number: "AE-2025-00341".to_string(),
        compound_id: 10001,
        severity: AdverseEventSeverity::Severe,
        event_description: "Rhabdomyolysis with elevated CK levels (>10x ULN)".to_string(),
        onset_date_epoch: 1_708_300_800,
        reporter_type: "Physician".to_string(),
        outcome: "Recovered with sequelae".to_string(),
        serious: true,
        causality_assessment: "Probable".to_string(),
    };
    let bytes = encode_to_vec(&ae).expect("encode AdverseEventReport Severe failed");
    let (decoded, consumed) = decode_from_slice::<AdverseEventReport>(&bytes)
        .expect("decode AdverseEventReport Severe failed");
    assert_eq!(ae, decoded);
    assert_eq!(consumed, bytes.len());
}

#[test]
fn test_adverse_event_versioned_upgrade_v1_to_v3() {
    let ae = AdverseEventReport {
        report_id: 70002,
        case_number: "AE-2025-00892".to_string(),
        compound_id: 10002,
        severity: AdverseEventSeverity::Mild,
        event_description: "Gastrointestinal discomfort, mild nausea".to_string(),
        onset_date_epoch: 1_710_979_200,
        reporter_type: "Patient".to_string(),
        outcome: "Recovered without sequelae".to_string(),
        serious: false,
        causality_assessment: "Possible".to_string(),
    };
    let v1 = Version::new(1, 0, 0);
    let bytes_v1 = encode_versioned_value(&ae, v1).expect("encode AE v1.0.0 failed");
    let (decoded_v1, ver_v1, _consumed_v1): (AdverseEventReport, Version, usize) =
        decode_versioned_value::<AdverseEventReport>(&bytes_v1).expect("decode AE v1.0.0 failed");
    assert_eq!(ae, decoded_v1);
    assert_eq!(ver_v1.major, 1);

    let v3 = Version::new(3, 0, 0);
    let bytes_v3 = encode_versioned_value(&decoded_v1, v3).expect("encode AE v3.0.0 failed");
    let (decoded_v3, ver_v3, consumed_v3): (AdverseEventReport, Version, usize) =
        decode_versioned_value::<AdverseEventReport>(&bytes_v3).expect("decode AE v3.0.0 failed");
    assert_eq!(ae, decoded_v3);
    assert_eq!(ver_v3.major, 3);
    assert!(consumed_v3 > 0);
}

#[test]
fn test_pharmacokinetics_params_roundtrip() {
    let pk = PharmacokineticsParams {
        pk_id: 80001,
        compound_id: 10001,
        subject_id: "SUBJ-0042".to_string(),
        cmax_x1000: 54_200,
        tmax_minutes: 120,
        auc_0_inf_x1000: 485_600,
        half_life_minutes: 840,
        clearance_x1000: 625,
        volume_distribution_x1000: 381_000,
        bioavailability_x100: 1400,
        dose_mg_x100: 1000,
    };
    let bytes = encode_to_vec(&pk).expect("encode PharmacokineticsParams failed");
    let (decoded, consumed) = decode_from_slice::<PharmacokineticsParams>(&bytes)
        .expect("decode PharmacokineticsParams failed");
    assert_eq!(pk, decoded);
    assert_eq!(consumed, bytes.len());
}

#[test]
fn test_formulation_composition_versioned_v2_0_0() {
    let formulation = FormulationComposition {
        formulation_id: 90001,
        product_name: "Atorvastatin Calcium Tablets 10mg".to_string(),
        dosage_form: "Film-coated tablet".to_string(),
        active_ingredient_mg_x100: 1034,
        excipient_1: "Microcrystalline cellulose".to_string(),
        excipient_1_mg_x100: 8500,
        excipient_2: "Lactose monohydrate".to_string(),
        excipient_2_mg_x100: 5200,
        coating_type: "Opadry II White".to_string(),
        total_weight_mg_x100: 25000,
    };
    let version = Version::new(2, 0, 0);
    let bytes = encode_versioned_value(&formulation, version)
        .expect("encode versioned FormulationComposition v2.0.0 failed");
    let (decoded, ver, consumed): (FormulationComposition, Version, usize) =
        decode_versioned_value::<FormulationComposition>(&bytes)
            .expect("decode versioned FormulationComposition v2.0.0 failed");
    assert_eq!(formulation, decoded);
    assert_eq!(ver.major, 2);
    assert_eq!(ver.minor, 0);
    assert!(consumed > 0);
}

#[test]
fn test_gmp_compliance_checklist_roundtrip() {
    let checklist = GmpComplianceChecklist {
        checklist_id: 100001,
        facility_code: "FAC-US-NJ-001".to_string(),
        inspection_date_epoch: 1_712_016_000,
        documentation_ok: true,
        equipment_qualified: true,
        personnel_trained: true,
        env_monitoring_ok: true,
        cleaning_validation_ok: false,
        deviation_count: 2,
        overall_compliant: false,
    };
    let bytes = encode_to_vec(&checklist).expect("encode GmpComplianceChecklist failed");
    let (decoded, consumed) = decode_from_slice::<GmpComplianceChecklist>(&bytes)
        .expect("decode GmpComplianceChecklist failed");
    assert_eq!(checklist, decoded);
    assert_eq!(consumed, bytes.len());
}

#[test]
fn test_cold_chain_shipping_versioned_ultra_cold() {
    let shipment = ColdChainShippingRecord {
        shipment_id: 110001,
        batch_id: 30099,
        origin_site: "Kalamazoo, MI".to_string(),
        destination_site: "Brussels, Belgium".to_string(),
        temp_zone: ShipmentTempZone::UltraCold,
        min_temp_recorded_x10: -780,
        max_temp_recorded_x10: -680,
        excursion_minutes: 0,
        departure_epoch: 1_713_139_200,
        arrival_epoch: 1_713_398_400,
        product_integrity_ok: true,
    };
    let version = Version::new(1, 5, 0);
    let bytes = encode_versioned_value(&shipment, version)
        .expect("encode versioned ColdChain UltraCold v1.5.0 failed");
    let (decoded, ver, consumed): (ColdChainShippingRecord, Version, usize) =
        decode_versioned_value::<ColdChainShippingRecord>(&bytes)
            .expect("decode versioned ColdChain UltraCold v1.5.0 failed");
    assert_eq!(shipment, decoded);
    assert_eq!(ver.major, 1);
    assert_eq!(ver.minor, 5);
    assert!(consumed > 0);
}

#[test]
fn test_drug_interaction_contraindicated_roundtrip() {
    let interaction = DrugInteractionEntry {
        interaction_id: 120001,
        drug_a_name: "Simvastatin".to_string(),
        drug_b_name: "Itraconazole".to_string(),
        severity: DrugInteractionSeverity::Contraindicated,
        mechanism: "Strong CYP3A4 inhibition increases statin exposure".to_string(),
        clinical_effect: "Greatly increased risk of rhabdomyolysis".to_string(),
        recommendation: "Do not co-administer; consider alternative antifungal".to_string(),
    };
    let bytes =
        encode_to_vec(&interaction).expect("encode DrugInteractionEntry Contraindicated failed");
    let (decoded, consumed) = decode_from_slice::<DrugInteractionEntry>(&bytes)
        .expect("decode DrugInteractionEntry Contraindicated failed");
    assert_eq!(interaction, decoded);
    assert_eq!(consumed, bytes.len());
}

#[test]
fn test_bioequivalence_study_passes_roundtrip() {
    let be_study = BioequivalenceStudy {
        study_id: 130001,
        test_product_id: 10050,
        reference_product_id: 10001,
        subjects_enrolled: 36,
        auc_ratio_x1000: 1012,
        cmax_ratio_x1000: 985,
        lower_ci_90_x1000: 870,
        upper_ci_90_x1000: 1150,
        bioequivalent: true,
        crossover_design: true,
    };
    let bytes = encode_to_vec(&be_study).expect("encode BioequivalenceStudy pass failed");
    let (decoded, consumed) = decode_from_slice::<BioequivalenceStudy>(&bytes)
        .expect("decode BioequivalenceStudy pass failed");
    assert_eq!(be_study, decoded);
    assert_eq!(consumed, bytes.len());
}

#[test]
fn test_label_claim_verification_versioned_v4_0_0() {
    let label = LabelClaimVerification {
        verification_id: 140001,
        product_code: "MET-500MG-TAB".to_string(),
        batch_id: 30002,
        claimed_strength_mg_x100: 50000,
        measured_strength_mg_x100: 49750,
        percent_label_claim_x100: 9950,
        within_spec: true,
        analyst_id: "AN-0078".to_string(),
        method_id: "METH-HPLC-042".to_string(),
    };
    let version = Version::new(4, 0, 0);
    let bytes =
        encode_versioned_value(&label, version).expect("encode versioned LabelClaim v4.0.0 failed");
    let (decoded, ver, consumed): (LabelClaimVerification, Version, usize) =
        decode_versioned_value::<LabelClaimVerification>(&bytes)
            .expect("decode versioned LabelClaim v4.0.0 failed");
    assert_eq!(label, decoded);
    assert_eq!(ver.major, 4);
    assert_eq!(ver.minor, 0);
    assert_eq!(ver.patch, 0);
    assert!(consumed > 0);
}

#[test]
fn test_deviation_capa_critical_versioned_v2_3_1() {
    let deviation = DeviationCapaRecord {
        deviation_id: 150001,
        batch_id: 30001,
        category: DeviationCategory::Critical,
        description: "Temperature excursion in granulation room exceeded 30C for 45 minutes"
            .to_string(),
        root_cause: "HVAC chiller unit failure due to refrigerant leak".to_string(),
        capa_status: CapaStatus::CorrectiveActionInProgress,
        initiated_date_epoch: 1_706_918_400,
        target_closure_epoch: 1_709_510_400,
        actual_closure_epoch: None,
        impact_assessment: "Batch placed on hold pending stability evaluation".to_string(),
    };
    let version = Version::new(2, 3, 1);
    let bytes = encode_versioned_value(&deviation, version)
        .expect("encode versioned DeviationCapa v2.3.1 failed");
    let (decoded, ver, consumed): (DeviationCapaRecord, Version, usize) =
        decode_versioned_value::<DeviationCapaRecord>(&bytes)
            .expect("decode versioned DeviationCapa v2.3.1 failed");
    assert_eq!(deviation, decoded);
    assert_eq!(ver.major, 2);
    assert_eq!(ver.minor, 3);
    assert_eq!(ver.patch, 1);
    assert!(consumed > 0);
}
