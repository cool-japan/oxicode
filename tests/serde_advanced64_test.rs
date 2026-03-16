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

// ---------------------------------------------------------------------------
// Domain types — Pharmaceutical Drug Development & Clinical Trial Management
// ---------------------------------------------------------------------------

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
struct MolecularDescriptor {
    compound_id: String,
    iupac_name: String,
    molecular_weight_daltons: f64,
    log_p: f64,
    hydrogen_bond_donors: u8,
    hydrogen_bond_acceptors: u8,
    rotatable_bonds: u8,
    topological_polar_surface_area: f64,
    lipinski_violations: u8,
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
enum TrialPhase {
    Preclinical,
    PhaseI,
    PhaseII,
    PhaseIII,
    PhaseIV,
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
struct ClinicalTrialRecord {
    nct_id: String,
    sponsor: String,
    phase: TrialPhase,
    indication: String,
    enrollment_target: u32,
    enrolled_count: u32,
    primary_endpoint: String,
    is_randomized: bool,
    is_double_blind: bool,
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
enum StratificationFactor {
    AgeRange { min_years: u8, max_years: u8 },
    Sex(String),
    BiomarkerPositive(String),
    PriorTherapyLines(u8),
    EcogScore(u8),
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
struct PatientCohort {
    cohort_id: String,
    trial_nct_id: String,
    arm_label: String,
    patient_count: u32,
    stratification_factors: Vec<StratificationFactor>,
    median_age_years: f32,
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
enum AdverseEventSeverity {
    Mild,
    Moderate,
    Severe,
    LifeThreatening,
    Fatal,
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
enum CausalityAssessment {
    Unrelated,
    Unlikely,
    Possible,
    Probable,
    Definite,
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
struct AdverseEventReport {
    report_id: String,
    subject_id: String,
    meddra_pt: String,
    meddra_soc: String,
    severity: AdverseEventSeverity,
    causality: CausalityAssessment,
    onset_day: u16,
    resolved: bool,
    dose_modification_required: bool,
    serious: bool,
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
struct PharmacokineticParams {
    subject_id: String,
    compound_id: String,
    dose_mg: f64,
    c_max_ng_per_ml: f64,
    t_max_hours: f64,
    auc_0_inf_ng_h_per_ml: f64,
    half_life_hours: f64,
    clearance_l_per_h: f64,
    volume_of_distribution_l: f64,
    bioavailability_pct: Option<f64>,
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
enum ExcipientFunction {
    Binder,
    Disintegrant,
    Lubricant,
    Filler,
    Coating,
    Preservative,
    Solubilizer,
    Surfactant,
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
struct FormulationComponent {
    ingredient_name: String,
    function: ExcipientFunction,
    weight_pct: f64,
    grade: String,
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
struct FormulationRecord {
    formulation_id: String,
    dosage_form: String,
    api_name: String,
    api_strength_mg: f64,
    components: Vec<FormulationComponent>,
    total_weight_mg: f64,
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
enum BatchDisposition {
    Released,
    Rejected,
    Quarantined,
    UnderInvestigation,
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
struct GmpBatchRelease {
    batch_number: String,
    product_code: String,
    manufacturing_site: String,
    batch_size_kg: f64,
    yield_pct: f64,
    assay_pct: f64,
    impurity_total_pct: f64,
    dissolution_pct_q30: f64,
    disposition: BatchDisposition,
    qa_reviewer: String,
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
enum FdaSubmissionType {
    Ind,
    Nda,
    Anda,
    Bla,
    SupplementalNda,
    TypeIIDmf,
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
enum SubmissionStatus {
    Draft,
    Submitted,
    UnderReview,
    CompleteResponseLetter,
    Approved,
    Refused,
    Withdrawn,
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
struct FdaSubmissionTracker {
    submission_id: String,
    submission_type: FdaSubmissionType,
    status: SubmissionStatus,
    applicant: String,
    drug_name: String,
    indication: String,
    pdufa_date: Option<String>,
    review_division: String,
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
enum InteractionSeverity {
    Minor,
    Moderate,
    Major,
    Contraindicated,
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
struct DrugInteractionEntry {
    drug_a: String,
    drug_b: String,
    severity: InteractionSeverity,
    mechanism: String,
    clinical_effect: String,
    management: String,
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
struct BioequivalenceResult {
    study_id: String,
    test_product: String,
    reference_product: String,
    parameter: String,
    geometric_mean_ratio_pct: f64,
    ci_lower_90_pct: f64,
    ci_upper_90_pct: f64,
    is_bioequivalent: bool,
    subject_count: u32,
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
enum StabilityCondition {
    LongTerm25C60RH,
    Intermediate30C65RH,
    Accelerated40C75RH,
    Refrigerated5C,
    Frozen,
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
struct StabilityTimepoint {
    condition: StabilityCondition,
    months: u16,
    assay_pct: f64,
    degradation_pct: f64,
    appearance_pass: bool,
    dissolution_pass: bool,
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
struct StabilityStudy {
    study_id: String,
    batch_number: String,
    product_name: String,
    timepoints: Vec<StabilityTimepoint>,
    proposed_shelf_life_months: u16,
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
struct ColdChainLogEntry {
    shipment_id: String,
    timestamp_epoch_secs: u64,
    temperature_celsius: f64,
    humidity_pct: Option<f64>,
    location_description: String,
    excursion_detected: bool,
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
struct ColdChainShipment {
    shipment_id: String,
    origin_site: String,
    destination_site: String,
    product_name: String,
    required_temp_min_c: f64,
    required_temp_max_c: f64,
    log_entries: Vec<ColdChainLogEntry>,
    chain_of_custody_intact: bool,
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
enum RegulatoryRegion {
    Fda,
    Ema,
    Pmda,
    Nmpa,
    Anvisa,
    Tga,
    HealthCanada,
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
struct RegulatoryMilestone {
    milestone_id: String,
    drug_name: String,
    region: RegulatoryRegion,
    milestone_description: String,
    target_date: String,
    completed: bool,
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
struct DoseEscalationStep {
    dose_level: u8,
    dose_mg: f64,
    subjects_enrolled: u8,
    dlts_observed: u8,
    mtd_declared: bool,
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
struct DoseEscalationStudy {
    study_id: String,
    compound_id: String,
    design: String,
    steps: Vec<DoseEscalationStep>,
    recommended_phase2_dose_mg: Option<f64>,
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
enum EfficacyEndpoint {
    ObjectiveResponseRate(f64),
    ProgressionFreeSurvivalMonths(f64),
    OverallSurvivalMonths(f64),
    DurationOfResponseMonths(f64),
    CompleteResponseRate(f64),
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
struct EfficacyAnalysis {
    trial_nct_id: String,
    arm_label: String,
    endpoint: EfficacyEndpoint,
    p_value: f64,
    is_statistically_significant: bool,
    hazard_ratio: Option<f64>,
    confidence_interval_lower: f64,
    confidence_interval_upper: f64,
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
struct DrugProductLabel {
    ndc_code: String,
    proprietary_name: String,
    nonproprietary_name: String,
    route_of_administration: String,
    dosage_form: String,
    strength: String,
    black_box_warning: bool,
    rems_required: bool,
    controlled_substance_schedule: Option<String>,
    indication_text: String,
}

// ---------------------------------------------------------------------------
// Tests
// ---------------------------------------------------------------------------

#[test]
fn test_molecular_descriptor_roundtrip() {
    let cfg = config::standard();
    let val = MolecularDescriptor {
        compound_id: "CMP-2026-00471".to_string(),
        iupac_name: "4-(3-chloro-4-fluorophenyl)-2-(piperidin-1-yl)thieno[3,2-d]pyrimidine"
            .to_string(),
        molecular_weight_daltons: 347.84,
        log_p: 3.21,
        hydrogen_bond_donors: 0,
        hydrogen_bond_acceptors: 4,
        rotatable_bonds: 2,
        topological_polar_surface_area: 59.42,
        lipinski_violations: 0,
    };
    let bytes = encode_to_vec(&val, cfg).expect("encode MolecularDescriptor");
    let (decoded, _): (MolecularDescriptor, _) =
        decode_owned_from_slice(&bytes, cfg).expect("decode MolecularDescriptor");
    assert_eq!(val, decoded);
}

#[test]
fn test_clinical_trial_record_phase_iii() {
    let cfg = config::standard();
    let val = ClinicalTrialRecord {
        nct_id: "NCT05123456".to_string(),
        sponsor: "Meridian Therapeutics Inc.".to_string(),
        phase: TrialPhase::PhaseIII,
        indication: "Non-small cell lung cancer (NSCLC)".to_string(),
        enrollment_target: 680,
        enrolled_count: 412,
        primary_endpoint: "Overall survival".to_string(),
        is_randomized: true,
        is_double_blind: true,
    };
    let bytes = encode_to_vec(&val, cfg).expect("encode ClinicalTrialRecord");
    let (decoded, _): (ClinicalTrialRecord, _) =
        decode_owned_from_slice(&bytes, cfg).expect("decode ClinicalTrialRecord");
    assert_eq!(val, decoded);
}

#[test]
fn test_patient_cohort_with_stratification() {
    let cfg = config::standard();
    let val = PatientCohort {
        cohort_id: "COH-A1".to_string(),
        trial_nct_id: "NCT05123456".to_string(),
        arm_label: "Experimental: CMP-471 + pembrolizumab".to_string(),
        patient_count: 206,
        stratification_factors: vec![
            StratificationFactor::AgeRange {
                min_years: 18,
                max_years: 75,
            },
            StratificationFactor::Sex("Male".to_string()),
            StratificationFactor::BiomarkerPositive("PD-L1 TPS >= 50%".to_string()),
            StratificationFactor::PriorTherapyLines(1),
            StratificationFactor::EcogScore(0),
        ],
        median_age_years: 62.5,
    };
    let bytes = encode_to_vec(&val, cfg).expect("encode PatientCohort");
    let (decoded, _): (PatientCohort, _) =
        decode_owned_from_slice(&bytes, cfg).expect("decode PatientCohort");
    assert_eq!(val, decoded);
}

#[test]
fn test_adverse_event_report_serious() {
    let cfg = config::standard();
    let val = AdverseEventReport {
        report_id: "AE-2026-08921".to_string(),
        subject_id: "SUBJ-0412".to_string(),
        meddra_pt: "Pneumonitis".to_string(),
        meddra_soc: "Respiratory, thoracic and mediastinal disorders".to_string(),
        severity: AdverseEventSeverity::Severe,
        causality: CausalityAssessment::Probable,
        onset_day: 84,
        resolved: false,
        dose_modification_required: true,
        serious: true,
    };
    let bytes = encode_to_vec(&val, cfg).expect("encode AdverseEventReport");
    let (decoded, _): (AdverseEventReport, _) =
        decode_owned_from_slice(&bytes, cfg).expect("decode AdverseEventReport");
    assert_eq!(val, decoded);
}

#[test]
fn test_adverse_event_mild_resolved() {
    let cfg = config::standard();
    let val = AdverseEventReport {
        report_id: "AE-2026-07100".to_string(),
        subject_id: "SUBJ-0288".to_string(),
        meddra_pt: "Headache".to_string(),
        meddra_soc: "Nervous system disorders".to_string(),
        severity: AdverseEventSeverity::Mild,
        causality: CausalityAssessment::Unlikely,
        onset_day: 7,
        resolved: true,
        dose_modification_required: false,
        serious: false,
    };
    let bytes = encode_to_vec(&val, cfg).expect("encode mild AE");
    let (decoded, _): (AdverseEventReport, _) =
        decode_owned_from_slice(&bytes, cfg).expect("decode mild AE");
    assert_eq!(val, decoded);
}

#[test]
fn test_pharmacokinetic_params_with_bioavailability() {
    let cfg = config::standard();
    let val = PharmacokineticParams {
        subject_id: "SUBJ-0101".to_string(),
        compound_id: "CMP-2026-00471".to_string(),
        dose_mg: 200.0,
        c_max_ng_per_ml: 1842.3,
        t_max_hours: 2.5,
        auc_0_inf_ng_h_per_ml: 18430.0,
        half_life_hours: 8.7,
        clearance_l_per_h: 10.85,
        volume_of_distribution_l: 136.2,
        bioavailability_pct: Some(72.4),
    };
    let bytes = encode_to_vec(&val, cfg).expect("encode PK params");
    let (decoded, _): (PharmacokineticParams, _) =
        decode_owned_from_slice(&bytes, cfg).expect("decode PK params");
    assert_eq!(val, decoded);
}

#[test]
fn test_pharmacokinetic_params_iv_no_bioavailability() {
    let cfg = config::standard();
    let val = PharmacokineticParams {
        subject_id: "SUBJ-0055".to_string(),
        compound_id: "CMP-2026-00471".to_string(),
        dose_mg: 100.0,
        c_max_ng_per_ml: 2560.0,
        t_max_hours: 0.0,
        auc_0_inf_ng_h_per_ml: 25120.0,
        half_life_hours: 8.9,
        clearance_l_per_h: 3.98,
        volume_of_distribution_l: 51.1,
        bioavailability_pct: None,
    };
    let bytes = encode_to_vec(&val, cfg).expect("encode PK IV");
    let (decoded, _): (PharmacokineticParams, _) =
        decode_owned_from_slice(&bytes, cfg).expect("decode PK IV");
    assert_eq!(val, decoded);
}

#[test]
fn test_formulation_record_tablet() {
    let cfg = config::standard();
    let val = FormulationRecord {
        formulation_id: "FORM-T-200".to_string(),
        dosage_form: "Film-coated tablet".to_string(),
        api_name: "CMP-471 mesylate".to_string(),
        api_strength_mg: 200.0,
        components: vec![
            FormulationComponent {
                ingredient_name: "Microcrystalline cellulose".to_string(),
                function: ExcipientFunction::Filler,
                weight_pct: 35.0,
                grade: "PH-102".to_string(),
            },
            FormulationComponent {
                ingredient_name: "Croscarmellose sodium".to_string(),
                function: ExcipientFunction::Disintegrant,
                weight_pct: 5.0,
                grade: "NF".to_string(),
            },
            FormulationComponent {
                ingredient_name: "Magnesium stearate".to_string(),
                function: ExcipientFunction::Lubricant,
                weight_pct: 0.5,
                grade: "NF vegetable".to_string(),
            },
            FormulationComponent {
                ingredient_name: "Opadry II White".to_string(),
                function: ExcipientFunction::Coating,
                weight_pct: 3.0,
                grade: "85F".to_string(),
            },
        ],
        total_weight_mg: 500.0,
    };
    let bytes = encode_to_vec(&val, cfg).expect("encode FormulationRecord");
    let (decoded, _): (FormulationRecord, _) =
        decode_owned_from_slice(&bytes, cfg).expect("decode FormulationRecord");
    assert_eq!(val, decoded);
}

#[test]
fn test_gmp_batch_release_approved() {
    let cfg = config::standard();
    let val = GmpBatchRelease {
        batch_number: "BN-2026-03-0042".to_string(),
        product_code: "CMP471-TAB-200".to_string(),
        manufacturing_site: "Meridian Pharma GmbH, Ludwigshafen".to_string(),
        batch_size_kg: 120.0,
        yield_pct: 98.3,
        assay_pct: 100.2,
        impurity_total_pct: 0.18,
        dissolution_pct_q30: 92.5,
        disposition: BatchDisposition::Released,
        qa_reviewer: "Dr. A. Hoffmann".to_string(),
    };
    let bytes = encode_to_vec(&val, cfg).expect("encode GMP batch released");
    let (decoded, _): (GmpBatchRelease, _) =
        decode_owned_from_slice(&bytes, cfg).expect("decode GMP batch released");
    assert_eq!(val, decoded);
}

#[test]
fn test_gmp_batch_rejected() {
    let cfg = config::standard();
    let val = GmpBatchRelease {
        batch_number: "BN-2026-02-0039".to_string(),
        product_code: "CMP471-TAB-200".to_string(),
        manufacturing_site: "Meridian Pharma GmbH, Ludwigshafen".to_string(),
        batch_size_kg: 120.0,
        yield_pct: 85.1,
        assay_pct: 94.7,
        impurity_total_pct: 1.42,
        dissolution_pct_q30: 68.3,
        disposition: BatchDisposition::Rejected,
        qa_reviewer: "Dr. K. Bauer".to_string(),
    };
    let bytes = encode_to_vec(&val, cfg).expect("encode GMP batch rejected");
    let (decoded, _): (GmpBatchRelease, _) =
        decode_owned_from_slice(&bytes, cfg).expect("decode GMP batch rejected");
    assert_eq!(val, decoded);
}

#[test]
fn test_fda_submission_nda_under_review() {
    let cfg = config::standard();
    let val = FdaSubmissionTracker {
        submission_id: "NDA-217890".to_string(),
        submission_type: FdaSubmissionType::Nda,
        status: SubmissionStatus::UnderReview,
        applicant: "Meridian Therapeutics Inc.".to_string(),
        drug_name: "MERIXIN".to_string(),
        indication: "First-line NSCLC with PD-L1 >= 50%".to_string(),
        pdufa_date: Some("2027-01-15".to_string()),
        review_division: "Division of Oncology Products 1".to_string(),
    };
    let bytes = encode_to_vec(&val, cfg).expect("encode FDA NDA");
    let (decoded, _): (FdaSubmissionTracker, _) =
        decode_owned_from_slice(&bytes, cfg).expect("decode FDA NDA");
    assert_eq!(val, decoded);
}

#[test]
fn test_fda_submission_anda_approved() {
    let cfg = config::standard();
    let val = FdaSubmissionTracker {
        submission_id: "ANDA-215432".to_string(),
        submission_type: FdaSubmissionType::Anda,
        status: SubmissionStatus::Approved,
        applicant: "GeneriCure Pharmaceuticals".to_string(),
        drug_name: "Amlodipine Besylate Tablets".to_string(),
        indication: "Hypertension".to_string(),
        pdufa_date: None,
        review_division: "Division of Cardiovascular and Renal Products".to_string(),
    };
    let bytes = encode_to_vec(&val, cfg).expect("encode FDA ANDA");
    let (decoded, _): (FdaSubmissionTracker, _) =
        decode_owned_from_slice(&bytes, cfg).expect("decode FDA ANDA");
    assert_eq!(val, decoded);
}

#[test]
fn test_drug_interaction_contraindicated() {
    let cfg = config::standard();
    let val = DrugInteractionEntry {
        drug_a: "Ketoconazole".to_string(),
        drug_b: "CMP-471".to_string(),
        severity: InteractionSeverity::Contraindicated,
        mechanism: "Strong CYP3A4 inhibition increases CMP-471 exposure ~8-fold".to_string(),
        clinical_effect: "Risk of QT prolongation and fatal arrhythmia".to_string(),
        management: "Co-administration is contraindicated; use alternative antifungal".to_string(),
    };
    let bytes = encode_to_vec(&val, cfg).expect("encode drug interaction");
    let (decoded, _): (DrugInteractionEntry, _) =
        decode_owned_from_slice(&bytes, cfg).expect("decode drug interaction");
    assert_eq!(val, decoded);
}

#[test]
fn test_bioequivalence_study_pass() {
    let cfg = config::standard();
    let val = BioequivalenceResult {
        study_id: "BE-2026-0088".to_string(),
        test_product: "Amlodipine 10mg GeneriCure".to_string(),
        reference_product: "Norvasc 10mg Pfizer".to_string(),
        parameter: "AUC(0-inf)".to_string(),
        geometric_mean_ratio_pct: 101.3,
        ci_lower_90_pct: 96.8,
        ci_upper_90_pct: 106.0,
        is_bioequivalent: true,
        subject_count: 36,
    };
    let bytes = encode_to_vec(&val, cfg).expect("encode BE result pass");
    let (decoded, _): (BioequivalenceResult, _) =
        decode_owned_from_slice(&bytes, cfg).expect("decode BE result pass");
    assert_eq!(val, decoded);
}

#[test]
fn test_bioequivalence_study_fail() {
    let cfg = config::standard();
    let val = BioequivalenceResult {
        study_id: "BE-2025-0071".to_string(),
        test_product: "Metformin 500mg TestCo".to_string(),
        reference_product: "Glucophage 500mg Merck".to_string(),
        parameter: "Cmax".to_string(),
        geometric_mean_ratio_pct: 118.7,
        ci_lower_90_pct: 108.2,
        ci_upper_90_pct: 130.4,
        is_bioequivalent: false,
        subject_count: 24,
    };
    let bytes = encode_to_vec(&val, cfg).expect("encode BE result fail");
    let (decoded, _): (BioequivalenceResult, _) =
        decode_owned_from_slice(&bytes, cfg).expect("decode BE result fail");
    assert_eq!(val, decoded);
}

#[test]
fn test_stability_study_long_term() {
    let cfg = config::standard();
    let val = StabilityStudy {
        study_id: "STAB-2025-LT-001".to_string(),
        batch_number: "BN-2025-06-0019".to_string(),
        product_name: "MERIXIN 200mg tablets".to_string(),
        timepoints: vec![
            StabilityTimepoint {
                condition: StabilityCondition::LongTerm25C60RH,
                months: 0,
                assay_pct: 100.1,
                degradation_pct: 0.0,
                appearance_pass: true,
                dissolution_pass: true,
            },
            StabilityTimepoint {
                condition: StabilityCondition::LongTerm25C60RH,
                months: 6,
                assay_pct: 99.8,
                degradation_pct: 0.12,
                appearance_pass: true,
                dissolution_pass: true,
            },
            StabilityTimepoint {
                condition: StabilityCondition::LongTerm25C60RH,
                months: 12,
                assay_pct: 99.3,
                degradation_pct: 0.31,
                appearance_pass: true,
                dissolution_pass: true,
            },
            StabilityTimepoint {
                condition: StabilityCondition::LongTerm25C60RH,
                months: 24,
                assay_pct: 98.4,
                degradation_pct: 0.68,
                appearance_pass: true,
                dissolution_pass: true,
            },
        ],
        proposed_shelf_life_months: 36,
    };
    let bytes = encode_to_vec(&val, cfg).expect("encode stability study");
    let (decoded, _): (StabilityStudy, _) =
        decode_owned_from_slice(&bytes, cfg).expect("decode stability study");
    assert_eq!(val, decoded);
}

#[test]
fn test_stability_accelerated_condition() {
    let cfg = config::standard();
    let val = StabilityTimepoint {
        condition: StabilityCondition::Accelerated40C75RH,
        months: 6,
        assay_pct: 96.1,
        degradation_pct: 2.4,
        appearance_pass: true,
        dissolution_pass: false,
    };
    let bytes = encode_to_vec(&val, cfg).expect("encode accelerated timepoint");
    let (decoded, _): (StabilityTimepoint, _) =
        decode_owned_from_slice(&bytes, cfg).expect("decode accelerated timepoint");
    assert_eq!(val, decoded);
}

#[test]
fn test_cold_chain_shipment_with_excursion() {
    let cfg = config::standard();
    let val = ColdChainShipment {
        shipment_id: "SHIP-2026-CC-0114".to_string(),
        origin_site: "Meridian Biologics, Basel".to_string(),
        destination_site: "Regional Distribution Center, Newark".to_string(),
        product_name: "Anti-PD1 mAb 100mg/4mL vial".to_string(),
        required_temp_min_c: 2.0,
        required_temp_max_c: 8.0,
        log_entries: vec![
            ColdChainLogEntry {
                shipment_id: "SHIP-2026-CC-0114".to_string(),
                timestamp_epoch_secs: 1773532800,
                temperature_celsius: 4.2,
                humidity_pct: Some(45.0),
                location_description: "Loading dock, Basel facility".to_string(),
                excursion_detected: false,
            },
            ColdChainLogEntry {
                shipment_id: "SHIP-2026-CC-0114".to_string(),
                timestamp_epoch_secs: 1773576000,
                temperature_celsius: 12.8,
                humidity_pct: Some(62.0),
                location_description: "Tarmac, Frankfurt Airport".to_string(),
                excursion_detected: true,
            },
            ColdChainLogEntry {
                shipment_id: "SHIP-2026-CC-0114".to_string(),
                timestamp_epoch_secs: 1773583200,
                temperature_celsius: 5.1,
                humidity_pct: Some(48.0),
                location_description: "Cargo hold, LH400".to_string(),
                excursion_detected: false,
            },
        ],
        chain_of_custody_intact: false,
    };
    let bytes = encode_to_vec(&val, cfg).expect("encode cold chain shipment");
    let (decoded, _): (ColdChainShipment, _) =
        decode_owned_from_slice(&bytes, cfg).expect("decode cold chain shipment");
    assert_eq!(val, decoded);
}

#[test]
fn test_regulatory_milestone_multiregion() {
    let cfg = config::standard();
    let milestones = vec![
        RegulatoryMilestone {
            milestone_id: "RM-FDA-001".to_string(),
            drug_name: "MERIXIN".to_string(),
            region: RegulatoryRegion::Fda,
            milestone_description: "NDA filing accepted".to_string(),
            target_date: "2026-07-01".to_string(),
            completed: true,
        },
        RegulatoryMilestone {
            milestone_id: "RM-EMA-001".to_string(),
            drug_name: "MERIXIN".to_string(),
            region: RegulatoryRegion::Ema,
            milestone_description: "MAA validation".to_string(),
            target_date: "2026-08-15".to_string(),
            completed: false,
        },
        RegulatoryMilestone {
            milestone_id: "RM-PMDA-001".to_string(),
            drug_name: "MERIXIN".to_string(),
            region: RegulatoryRegion::Pmda,
            milestone_description: "JNDA pre-submission meeting".to_string(),
            target_date: "2026-09-30".to_string(),
            completed: false,
        },
    ];
    let bytes = encode_to_vec(&milestones, cfg).expect("encode regulatory milestones");
    let (decoded, _): (Vec<RegulatoryMilestone>, _) =
        decode_owned_from_slice(&bytes, cfg).expect("decode regulatory milestones");
    assert_eq!(milestones, decoded);
}

#[test]
fn test_dose_escalation_study_with_mtd() {
    let cfg = config::standard();
    let val = DoseEscalationStudy {
        study_id: "DE-2025-PH1-003".to_string(),
        compound_id: "CMP-2026-00471".to_string(),
        design: "3+3 dose escalation".to_string(),
        steps: vec![
            DoseEscalationStep {
                dose_level: 1,
                dose_mg: 25.0,
                subjects_enrolled: 3,
                dlts_observed: 0,
                mtd_declared: false,
            },
            DoseEscalationStep {
                dose_level: 2,
                dose_mg: 50.0,
                subjects_enrolled: 3,
                dlts_observed: 0,
                mtd_declared: false,
            },
            DoseEscalationStep {
                dose_level: 3,
                dose_mg: 100.0,
                subjects_enrolled: 3,
                dlts_observed: 0,
                mtd_declared: false,
            },
            DoseEscalationStep {
                dose_level: 4,
                dose_mg: 200.0,
                subjects_enrolled: 6,
                dlts_observed: 1,
                mtd_declared: false,
            },
            DoseEscalationStep {
                dose_level: 5,
                dose_mg: 400.0,
                subjects_enrolled: 6,
                dlts_observed: 3,
                mtd_declared: true,
            },
        ],
        recommended_phase2_dose_mg: Some(200.0),
    };
    let bytes = encode_to_vec(&val, cfg).expect("encode dose escalation");
    let (decoded, _): (DoseEscalationStudy, _) =
        decode_owned_from_slice(&bytes, cfg).expect("decode dose escalation");
    assert_eq!(val, decoded);
}

#[test]
fn test_efficacy_analysis_significant_result() {
    let cfg = config::standard();
    let val = EfficacyAnalysis {
        trial_nct_id: "NCT05123456".to_string(),
        arm_label: "Experimental arm".to_string(),
        endpoint: EfficacyEndpoint::ProgressionFreeSurvivalMonths(12.8),
        p_value: 0.0003,
        is_statistically_significant: true,
        hazard_ratio: Some(0.58),
        confidence_interval_lower: 0.43,
        confidence_interval_upper: 0.78,
    };
    let bytes = encode_to_vec(&val, cfg).expect("encode efficacy analysis");
    let (decoded, _): (EfficacyAnalysis, _) =
        decode_owned_from_slice(&bytes, cfg).expect("decode efficacy analysis");
    assert_eq!(val, decoded);
}

#[test]
fn test_drug_product_label_with_schedule() {
    let cfg = config::standard();
    let val = DrugProductLabel {
        ndc_code: "12345-678-90".to_string(),
        proprietary_name: "MERIXIN".to_string(),
        nonproprietary_name: "CMP-471 mesylate".to_string(),
        route_of_administration: "Oral".to_string(),
        dosage_form: "Film-coated tablet".to_string(),
        strength: "200 mg".to_string(),
        black_box_warning: true,
        rems_required: false,
        controlled_substance_schedule: None,
        indication_text: "MERIXIN is a kinase inhibitor indicated for the first-line treatment \
            of adult patients with metastatic non-small cell lung cancer whose tumors express \
            PD-L1 (TPS >= 50%) as determined by an FDA-approved test, with no EGFR or ALK \
            genomic tumor aberrations."
            .to_string(),
    };
    let bytes = encode_to_vec(&val, cfg).expect("encode drug label");
    let (decoded, _): (DrugProductLabel, _) =
        decode_owned_from_slice(&bytes, cfg).expect("decode drug label");
    assert_eq!(val, decoded);
}
