//! Advanced file I/O tests for OxiCode — domain: veterinary medicine and animal health

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
use oxicode::{decode_from_file, decode_from_slice, encode_to_file, encode_to_vec, Decode, Encode};
use std::env::temp_dir;

// ---------------------------------------------------------------------------
// Domain types
// ---------------------------------------------------------------------------

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
enum Species {
    Canine,
    Feline,
    Equine,
    Bovine,
    Ovine,
    Caprine,
    Porcine,
    Avian,
    Reptile,
    Lagomorph,
    Exotic { description: String },
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
enum TriageCategory {
    Resuscitation,
    Emergent,
    Urgent,
    SemiUrgent,
    NonUrgent,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
enum VaccineType {
    CoreCanine,
    NonCoreCanine,
    CoreFeline,
    NonCoreFeline,
    Rabies,
    EquineInfluenza,
    EquineTetanus,
    BovineBvd,
    BovineBrucellosis,
    AvianNewcastle,
    Custom { name: String },
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
enum ParasiteType {
    Roundworm,
    Hookworm,
    Whipworm,
    Tapeworm,
    Heartworm,
    Coccidia,
    Giardia,
    Flea,
    Tick,
    Mite,
    Lice,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
enum ZoonoticDisease {
    Rabies,
    Leptospirosis,
    Brucellosis,
    Psittacosis,
    Toxoplasmosis,
    Salmonellosis,
    Campylobacteriosis,
    Ringworm,
    AvianInfluenza,
    WestNileVirus,
    Anthrax,
    Plague,
    Tularemia,
    Lyme,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
enum SurgicalProcedureType {
    Spay,
    Neuter,
    MassRemoval,
    Orthopedic { bone: String },
    Dental,
    Ophthalmic,
    SoftTissue,
    Cardiac,
    Neurological,
    Emergency,
    Exploratory,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
enum IntakeReason {
    Stray,
    OwnerSurrender,
    Confiscated,
    Transfer,
    Born,
    Return,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
enum NutritionCategory {
    Maintenance,
    Growth,
    Senior,
    WeightLoss,
    Renal,
    Hepatic,
    GastrointestinalSupport,
    Diabetic,
    Allergenic,
    Performance,
}

// --- Structs ---

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct PatientRecord {
    patient_id: u64,
    name: String,
    species: Species,
    breed: String,
    age_months: u16,
    weight_grams: u32,
    sex_is_male: bool,
    neutered: bool,
    owner_name: String,
    microchip_id: Option<String>,
    allergies: Vec<String>,
    registration_timestamp: u64,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct VaccinationSchedule {
    schedule_id: u64,
    patient_id: u64,
    vaccine_type: VaccineType,
    administered_date_unix: u64,
    next_due_date_unix: u64,
    lot_number: String,
    manufacturer: String,
    dose_ml_x100: u16,
    route: String,
    administering_vet_id: u32,
    adverse_reaction: bool,
    notes: String,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct CbcResult {
    patient_id: u64,
    sample_timestamp: u64,
    wbc_x100: u32,
    rbc_x100: u32,
    hemoglobin_g_per_dl_x100: u32,
    hematocrit_percent_x100: u32,
    platelet_count_x1000: u32,
    neutrophil_percent_x100: u16,
    lymphocyte_percent_x100: u16,
    monocyte_percent_x100: u16,
    eosinophil_percent_x100: u16,
    basophil_percent_x100: u16,
    mcv_fl_x100: u32,
    mch_pg_x100: u32,
    mchc_g_per_dl_x100: u32,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct ChemistryPanel {
    patient_id: u64,
    sample_timestamp: u64,
    glucose_mg_per_dl_x100: u32,
    bun_mg_per_dl_x100: u32,
    creatinine_mg_per_dl_x100: u32,
    alt_u_per_l_x100: u32,
    ast_u_per_l_x100: u32,
    alp_u_per_l_x100: u32,
    total_protein_g_per_dl_x100: u32,
    albumin_g_per_dl_x100: u32,
    calcium_mg_per_dl_x100: u32,
    phosphorus_mg_per_dl_x100: u32,
    sodium_meq_per_l_x100: u32,
    potassium_meq_per_l_x100: u32,
    chloride_meq_per_l_x100: u32,
    total_bilirubin_mg_per_dl_x100: u32,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct RadiographMetadata {
    image_id: u64,
    patient_id: u64,
    timestamp: u64,
    body_region: String,
    view: String,
    kvp: u16,
    ma_s_x100: u16,
    exposure_index: u32,
    technician_id: u32,
    findings: String,
    contrast_used: bool,
    sedation_required: bool,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct SurgicalProcedureLog {
    procedure_id: u64,
    patient_id: u64,
    procedure_type: SurgicalProcedureType,
    surgeon_id: u32,
    anesthetist_id: u32,
    start_timestamp: u64,
    end_timestamp: u64,
    anesthesia_duration_minutes: u16,
    anesthetic_protocol: String,
    complications: Vec<String>,
    blood_loss_ml: u16,
    outcome: String,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct PrescriptionRecord {
    rx_id: u64,
    patient_id: u64,
    medication_name: String,
    dose_mg_x100: u32,
    route: String,
    frequency_hours: u8,
    duration_days: u16,
    prescriber_id: u32,
    dispensed_quantity: u16,
    refills_remaining: u8,
    controlled_substance: bool,
    timestamp: u64,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct MicrochipRegistration {
    chip_id: String,
    patient_id: u64,
    chip_frequency_khz: u32,
    implant_site: String,
    implant_date_unix: u64,
    implanting_vet_id: u32,
    owner_name: String,
    owner_phone: String,
    owner_address: String,
    registered_with_registry: bool,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct HerdHealthRecord {
    herd_id: u32,
    farm_name: String,
    species: Species,
    herd_size: u32,
    vaccination_compliance_percent_x100: u16,
    last_inspection_timestamp: u64,
    tb_test_status: String,
    brucellosis_free: bool,
    johnes_status: String,
    avg_daily_gain_grams: u32,
    mortality_rate_percent_x100: u16,
    treatments_this_month: Vec<String>,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct EquinePerformanceMetrics {
    horse_id: u64,
    name: String,
    breed: String,
    discipline: String,
    resting_hr_bpm: u8,
    peak_hr_bpm: u16,
    recovery_time_seconds: u16,
    lactate_threshold_mmol_per_l_x100: u32,
    stride_length_cm: u16,
    v200_m_per_s_x100: u32,
    lameness_grade: u8,
    body_condition_score_x10: u8,
    last_evaluation_timestamp: u64,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct ZoonoticSurveillanceReport {
    report_id: u64,
    disease: ZoonoticDisease,
    region_code: String,
    report_timestamp: u64,
    confirmed_cases_animal: u32,
    suspected_cases_animal: u32,
    confirmed_cases_human: u32,
    species_involved: Vec<Species>,
    containment_active: bool,
    quarantine_radius_km_x100: u32,
    lab_confirmed: bool,
    reporting_authority: String,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct NecropsyReport {
    case_id: u64,
    patient_id: u64,
    species: Species,
    age_months: u16,
    weight_grams: u32,
    date_of_death_unix: u64,
    necropsy_date_unix: u64,
    gross_findings: Vec<String>,
    histopathology_findings: Vec<String>,
    cause_of_death: String,
    manner_of_death: String,
    pathologist_id: u32,
    tissues_collected: Vec<String>,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct ShelterIntakeRecord {
    intake_id: u64,
    species: Species,
    breed_estimate: String,
    estimated_age_months: u16,
    weight_grams: u32,
    sex_is_male: bool,
    intact: bool,
    reason: IntakeReason,
    intake_timestamp: u64,
    location_found: String,
    temperament_score: u8,
    medical_notes: String,
    intake_photo_hash: Vec<u8>,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct NutritionPlan {
    plan_id: u64,
    patient_id: u64,
    category: NutritionCategory,
    daily_calories_kcal: u32,
    protein_percent_x100: u16,
    fat_percent_x100: u16,
    fiber_percent_x100: u16,
    primary_diet_name: String,
    supplement_list: Vec<String>,
    feeding_frequency_per_day: u8,
    portion_grams: u32,
    target_weight_grams: u32,
    review_date_unix: u64,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct ParasiteScreeningResult {
    sample_id: u64,
    patient_id: u64,
    sample_timestamp: u64,
    method: String,
    parasites_found: Vec<ParasiteType>,
    egg_count_per_gram: u32,
    heartworm_antigen_positive: bool,
    tick_borne_panel_positive: bool,
    treatment_recommended: String,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct EmergencyTriageEntry {
    triage_id: u64,
    patient_id: u64,
    arrival_timestamp: u64,
    category: TriageCategory,
    presenting_complaint: String,
    heart_rate_bpm: u16,
    respiratory_rate_brpm: u16,
    temperature_milli_c: u32,
    mucous_membrane_color: String,
    capillary_refill_time_ms: u16,
    pain_score: u8,
    mentation: String,
    triaged_by_id: u32,
}

// ---------------------------------------------------------------------------
// Helper
// ---------------------------------------------------------------------------

fn unique_tmp(name: &str) -> std::path::PathBuf {
    temp_dir().join(format!("oxicode_fio40_{}_{}.bin", name, std::process::id()))
}

// ---------------------------------------------------------------------------
// Tests
// ---------------------------------------------------------------------------

// 1: Patient record roundtrip via file
#[test]
fn test_vet_patient_record_file_roundtrip() {
    let path = unique_tmp("patient_record");
    let original = PatientRecord {
        patient_id: 100_001,
        name: "Bella".into(),
        species: Species::Canine,
        breed: "Golden Retriever".into(),
        age_months: 36,
        weight_grams: 29_500,
        sex_is_male: false,
        neutered: true,
        owner_name: "Maria Torres".into(),
        microchip_id: Some("985121033456789".into()),
        allergies: vec!["Penicillin".into(), "Chicken".into()],
        registration_timestamp: 1_700_000_000,
    };
    encode_to_file(&original, &path).expect("encode patient record to file");
    let decoded: PatientRecord = decode_from_file(&path).expect("decode patient record from file");
    assert_eq!(original, decoded);
    std::fs::remove_file(&path).ok();
}

// 2: Vaccination schedule via slice
#[test]
fn test_vet_vaccination_schedule_slice_roundtrip() {
    let original = VaccinationSchedule {
        schedule_id: 50_001,
        patient_id: 100_001,
        vaccine_type: VaccineType::Rabies,
        administered_date_unix: 1_700_100_000,
        next_due_date_unix: 1_731_636_000,
        lot_number: "RAB-2025-A447".into(),
        manufacturer: "Merial".into(),
        dose_ml_x100: 100,
        route: "Subcutaneous".into(),
        administering_vet_id: 8001,
        adverse_reaction: false,
        notes: "No complications".into(),
    };
    let encoded = encode_to_vec(&original).expect("encode vaccination schedule");
    let (decoded, _): (VaccinationSchedule, usize) =
        decode_from_slice(&encoded).expect("decode vaccination schedule");
    assert_eq!(original, decoded);
}

// 3: CBC results via file
#[test]
fn test_vet_cbc_result_file_roundtrip() {
    let path = unique_tmp("cbc_result");
    let original = CbcResult {
        patient_id: 100_002,
        sample_timestamp: 1_700_200_000,
        wbc_x100: 1_250,
        rbc_x100: 680,
        hemoglobin_g_per_dl_x100: 1_520,
        hematocrit_percent_x100: 4_500,
        platelet_count_x1000: 320,
        neutrophil_percent_x100: 6_800,
        lymphocyte_percent_x100: 2_200,
        monocyte_percent_x100: 500,
        eosinophil_percent_x100: 400,
        basophil_percent_x100: 100,
        mcv_fl_x100: 6_620,
        mch_pg_x100: 2_240,
        mchc_g_per_dl_x100: 3_380,
    };
    encode_to_file(&original, &path).expect("encode CBC result to file");
    let decoded: CbcResult = decode_from_file(&path).expect("decode CBC result from file");
    assert_eq!(original, decoded);
    std::fs::remove_file(&path).ok();
}

// 4: Chemistry panel via slice
#[test]
fn test_vet_chemistry_panel_slice_roundtrip() {
    let original = ChemistryPanel {
        patient_id: 100_003,
        sample_timestamp: 1_700_300_000,
        glucose_mg_per_dl_x100: 9_500,
        bun_mg_per_dl_x100: 1_800,
        creatinine_mg_per_dl_x100: 120,
        alt_u_per_l_x100: 4_500,
        ast_u_per_l_x100: 3_200,
        alp_u_per_l_x100: 8_000,
        total_protein_g_per_dl_x100: 680,
        albumin_g_per_dl_x100: 340,
        calcium_mg_per_dl_x100: 1_020,
        phosphorus_mg_per_dl_x100: 480,
        sodium_meq_per_l_x100: 14_500,
        potassium_meq_per_l_x100: 450,
        chloride_meq_per_l_x100: 10_800,
        total_bilirubin_mg_per_dl_x100: 30,
    };
    let encoded = encode_to_vec(&original).expect("encode chemistry panel");
    let (decoded, _): (ChemistryPanel, usize) =
        decode_from_slice(&encoded).expect("decode chemistry panel");
    assert_eq!(original, decoded);
}

// 5: Radiograph metadata via file
#[test]
fn test_vet_radiograph_metadata_file_roundtrip() {
    let path = unique_tmp("radiograph_meta");
    let original = RadiographMetadata {
        image_id: 200_001,
        patient_id: 100_004,
        timestamp: 1_700_400_000,
        body_region: "Thorax".into(),
        view: "Right Lateral".into(),
        kvp: 68,
        ma_s_x100: 320,
        exposure_index: 1_450,
        technician_id: 9002,
        findings: "Mild cardiomegaly with vertebral heart score of 11.2".into(),
        contrast_used: false,
        sedation_required: true,
    };
    encode_to_file(&original, &path).expect("encode radiograph metadata to file");
    let decoded: RadiographMetadata =
        decode_from_file(&path).expect("decode radiograph metadata from file");
    assert_eq!(original, decoded);
    std::fs::remove_file(&path).ok();
}

// 6: Surgical procedure log via slice
#[test]
fn test_vet_surgical_procedure_slice_roundtrip() {
    let original = SurgicalProcedureLog {
        procedure_id: 300_001,
        patient_id: 100_005,
        procedure_type: SurgicalProcedureType::Orthopedic {
            bone: "Femur".into(),
        },
        surgeon_id: 7001,
        anesthetist_id: 7002,
        start_timestamp: 1_700_500_000,
        end_timestamp: 1_700_506_600,
        anesthesia_duration_minutes: 120,
        anesthetic_protocol: "Propofol induction, Isoflurane maintenance".into(),
        complications: vec!["Minor hemorrhage at incision site".into()],
        blood_loss_ml: 45,
        outcome: "Successful TPLO repair".into(),
    };
    let encoded = encode_to_vec(&original).expect("encode surgical procedure");
    let (decoded, _): (SurgicalProcedureLog, usize) =
        decode_from_slice(&encoded).expect("decode surgical procedure");
    assert_eq!(original, decoded);
}

// 7: Prescription record via file
#[test]
fn test_vet_prescription_record_file_roundtrip() {
    let path = unique_tmp("prescription");
    let original = PrescriptionRecord {
        rx_id: 400_001,
        patient_id: 100_006,
        medication_name: "Carprofen".into(),
        dose_mg_x100: 4_400,
        route: "Oral".into(),
        frequency_hours: 12,
        duration_days: 14,
        prescriber_id: 7001,
        dispensed_quantity: 28,
        refills_remaining: 1,
        controlled_substance: false,
        timestamp: 1_700_600_000,
    };
    encode_to_file(&original, &path).expect("encode prescription to file");
    let decoded: PrescriptionRecord =
        decode_from_file(&path).expect("decode prescription from file");
    assert_eq!(original, decoded);
    std::fs::remove_file(&path).ok();
}

// 8: Microchip registration via slice
#[test]
fn test_vet_microchip_registration_slice_roundtrip() {
    let original = MicrochipRegistration {
        chip_id: "985121033456789".into(),
        patient_id: 100_007,
        chip_frequency_khz: 134_200,
        implant_site: "Left lateral neck subcutaneous".into(),
        implant_date_unix: 1_700_700_000,
        implanting_vet_id: 8002,
        owner_name: "Kenji Tanaka".into(),
        owner_phone: "+81-3-1234-5678".into(),
        owner_address: "Tokyo, Shibuya-ku, 1-2-3".into(),
        registered_with_registry: true,
    };
    let encoded = encode_to_vec(&original).expect("encode microchip registration");
    let (decoded, _): (MicrochipRegistration, usize) =
        decode_from_slice(&encoded).expect("decode microchip registration");
    assert_eq!(original, decoded);
}

// 9: Herd health record via file
#[test]
fn test_vet_herd_health_record_file_roundtrip() {
    let path = unique_tmp("herd_health");
    let original = HerdHealthRecord {
        herd_id: 5001,
        farm_name: "Green Valley Dairy".into(),
        species: Species::Bovine,
        herd_size: 250,
        vaccination_compliance_percent_x100: 9_700,
        last_inspection_timestamp: 1_700_800_000,
        tb_test_status: "Negative".into(),
        brucellosis_free: true,
        johnes_status: "Monitored - Low Risk".into(),
        avg_daily_gain_grams: 850,
        mortality_rate_percent_x100: 180,
        treatments_this_month: vec![
            "Mastitis Tx x3".into(),
            "Lameness Tx x2".into(),
            "Respiratory Tx x1".into(),
        ],
    };
    encode_to_file(&original, &path).expect("encode herd health to file");
    let decoded: HerdHealthRecord = decode_from_file(&path).expect("decode herd health from file");
    assert_eq!(original, decoded);
    std::fs::remove_file(&path).ok();
}

// 10: Equine performance metrics via slice
#[test]
fn test_vet_equine_performance_slice_roundtrip() {
    let original = EquinePerformanceMetrics {
        horse_id: 600_001,
        name: "Storm Chaser".into(),
        breed: "Thoroughbred".into(),
        discipline: "Eventing".into(),
        resting_hr_bpm: 32,
        peak_hr_bpm: 220,
        recovery_time_seconds: 180,
        lactate_threshold_mmol_per_l_x100: 400,
        stride_length_cm: 720,
        v200_m_per_s_x100: 1_050,
        lameness_grade: 0,
        body_condition_score_x10: 55,
        last_evaluation_timestamp: 1_700_900_000,
    };
    let encoded = encode_to_vec(&original).expect("encode equine performance");
    let (decoded, _): (EquinePerformanceMetrics, usize) =
        decode_from_slice(&encoded).expect("decode equine performance");
    assert_eq!(original, decoded);
}

// 11: Zoonotic surveillance report via file
#[test]
fn test_vet_zoonotic_surveillance_file_roundtrip() {
    let path = unique_tmp("zoonotic_surv");
    let original = ZoonoticSurveillanceReport {
        report_id: 700_001,
        disease: ZoonoticDisease::Rabies,
        region_code: "US-TX-031".into(),
        report_timestamp: 1_701_000_000,
        confirmed_cases_animal: 3,
        suspected_cases_animal: 7,
        confirmed_cases_human: 0,
        species_involved: vec![Species::Canine, Species::Lagomorph],
        containment_active: true,
        quarantine_radius_km_x100: 1_000,
        lab_confirmed: true,
        reporting_authority: "Texas DSHS".into(),
    };
    encode_to_file(&original, &path).expect("encode zoonotic report to file");
    let decoded: ZoonoticSurveillanceReport =
        decode_from_file(&path).expect("decode zoonotic report from file");
    assert_eq!(original, decoded);
    std::fs::remove_file(&path).ok();
}

// 12: Necropsy report via slice
#[test]
fn test_vet_necropsy_report_slice_roundtrip() {
    let original = NecropsyReport {
        case_id: 800_001,
        patient_id: 100_008,
        species: Species::Feline,
        age_months: 192,
        weight_grams: 3_200,
        date_of_death_unix: 1_701_100_000,
        necropsy_date_unix: 1_701_110_000,
        gross_findings: vec![
            "Hepatomegaly with nodular surface".into(),
            "Bilateral pleural effusion approx 80ml".into(),
            "Pale kidneys with irregular contour".into(),
        ],
        histopathology_findings: vec![
            "Hepatocellular carcinoma, well differentiated".into(),
            "Chronic interstitial nephritis".into(),
        ],
        cause_of_death: "Hepatocellular carcinoma with metastatic disease".into(),
        manner_of_death: "Euthanasia".into(),
        pathologist_id: 9501,
        tissues_collected: vec![
            "Liver".into(),
            "Kidney".into(),
            "Lung".into(),
            "Spleen".into(),
            "Heart".into(),
        ],
    };
    let encoded = encode_to_vec(&original).expect("encode necropsy report");
    let (decoded, _): (NecropsyReport, usize) =
        decode_from_slice(&encoded).expect("decode necropsy report");
    assert_eq!(original, decoded);
}

// 13: Shelter intake record via file
#[test]
fn test_vet_shelter_intake_file_roundtrip() {
    let path = unique_tmp("shelter_intake");
    let original = ShelterIntakeRecord {
        intake_id: 900_001,
        species: Species::Canine,
        breed_estimate: "Pit Bull Mix".into(),
        estimated_age_months: 24,
        weight_grams: 22_000,
        sex_is_male: true,
        intact: true,
        reason: IntakeReason::Stray,
        intake_timestamp: 1_701_200_000,
        location_found: "Intersection of Oak St and Elm Ave".into(),
        temperament_score: 3,
        medical_notes: "Mild dermatitis on ventral abdomen, otherwise healthy".into(),
        intake_photo_hash: vec![0xAB, 0xCD, 0xEF, 0x01, 0x23, 0x45, 0x67, 0x89],
    };
    encode_to_file(&original, &path).expect("encode shelter intake to file");
    let decoded: ShelterIntakeRecord =
        decode_from_file(&path).expect("decode shelter intake from file");
    assert_eq!(original, decoded);
    std::fs::remove_file(&path).ok();
}

// 14: Nutrition plan via slice
#[test]
fn test_vet_nutrition_plan_slice_roundtrip() {
    let original = NutritionPlan {
        plan_id: 1_000_001,
        patient_id: 100_009,
        category: NutritionCategory::WeightLoss,
        daily_calories_kcal: 980,
        protein_percent_x100: 3_000,
        fat_percent_x100: 1_200,
        fiber_percent_x100: 800,
        primary_diet_name: "Hill's Metabolic + Mobility".into(),
        supplement_list: vec!["Omega-3 fish oil".into(), "Glucosamine/Chondroitin".into()],
        feeding_frequency_per_day: 2,
        portion_grams: 200,
        target_weight_grams: 25_000,
        review_date_unix: 1_703_800_000,
    };
    let encoded = encode_to_vec(&original).expect("encode nutrition plan");
    let (decoded, _): (NutritionPlan, usize) =
        decode_from_slice(&encoded).expect("decode nutrition plan");
    assert_eq!(original, decoded);
}

// 15: Parasite screening via file
#[test]
fn test_vet_parasite_screening_file_roundtrip() {
    let path = unique_tmp("parasite_screen");
    let original = ParasiteScreeningResult {
        sample_id: 1_100_001,
        patient_id: 100_010,
        sample_timestamp: 1_701_300_000,
        method: "Fecal flotation with centrifugation".into(),
        parasites_found: vec![ParasiteType::Roundworm, ParasiteType::Coccidia],
        egg_count_per_gram: 450,
        heartworm_antigen_positive: false,
        tick_borne_panel_positive: false,
        treatment_recommended: "Pyrantel pamoate 5mg/kg PO, repeat in 2 weeks".into(),
    };
    encode_to_file(&original, &path).expect("encode parasite screening to file");
    let decoded: ParasiteScreeningResult =
        decode_from_file(&path).expect("decode parasite screening from file");
    assert_eq!(original, decoded);
    std::fs::remove_file(&path).ok();
}

// 16: Emergency triage via slice
#[test]
fn test_vet_emergency_triage_slice_roundtrip() {
    let original = EmergencyTriageEntry {
        triage_id: 1_200_001,
        patient_id: 100_011,
        arrival_timestamp: 1_701_400_000,
        category: TriageCategory::Emergent,
        presenting_complaint: "Hit by car, non-weight-bearing left hindlimb".into(),
        heart_rate_bpm: 160,
        respiratory_rate_brpm: 40,
        temperature_milli_c: 38_500,
        mucous_membrane_color: "Pale pink".into(),
        capillary_refill_time_ms: 2_500,
        pain_score: 8,
        mentation: "Quiet but responsive".into(),
        triaged_by_id: 9003,
    };
    let encoded = encode_to_vec(&original).expect("encode emergency triage");
    let (decoded, _): (EmergencyTriageEntry, usize) =
        decode_from_slice(&encoded).expect("decode emergency triage");
    assert_eq!(original, decoded);
}

// 17: Multiple patient records batch via file
#[test]
fn test_vet_multi_patient_batch_file_roundtrip() {
    let path = unique_tmp("multi_patient");
    let patients = vec![
        PatientRecord {
            patient_id: 200_001,
            name: "Max".into(),
            species: Species::Canine,
            breed: "German Shepherd".into(),
            age_months: 60,
            weight_grams: 38_000,
            sex_is_male: true,
            neutered: true,
            owner_name: "John Smith".into(),
            microchip_id: Some("985121044567890".into()),
            allergies: vec![],
            registration_timestamp: 1_702_000_000,
        },
        PatientRecord {
            patient_id: 200_002,
            name: "Whiskers".into(),
            species: Species::Feline,
            breed: "Domestic Shorthair".into(),
            age_months: 84,
            weight_grams: 5_400,
            sex_is_male: false,
            neutered: true,
            owner_name: "Akiko Yamamoto".into(),
            microchip_id: None,
            allergies: vec!["Metronidazole".into()],
            registration_timestamp: 1_702_010_000,
        },
        PatientRecord {
            patient_id: 200_003,
            name: "Luna".into(),
            species: Species::Lagomorph,
            breed: "Holland Lop".into(),
            age_months: 18,
            weight_grams: 1_800,
            sex_is_male: false,
            neutered: true,
            owner_name: "Emily Chen".into(),
            microchip_id: Some("956000012345678".into()),
            allergies: vec![],
            registration_timestamp: 1_702_020_000,
        },
    ];
    encode_to_file(&patients, &path).expect("encode patient batch to file");
    let decoded: Vec<PatientRecord> =
        decode_from_file(&path).expect("decode patient batch from file");
    assert_eq!(patients, decoded);
    std::fs::remove_file(&path).ok();
}

// 18: Full vaccination history via slice
#[test]
fn test_vet_vaccination_history_slice_roundtrip() {
    let history = vec![
        VaccinationSchedule {
            schedule_id: 60_001,
            patient_id: 200_001,
            vaccine_type: VaccineType::CoreCanine,
            administered_date_unix: 1_680_000_000,
            next_due_date_unix: 1_711_536_000,
            lot_number: "DA2PP-2024-B112".into(),
            manufacturer: "Zoetis".into(),
            dose_ml_x100: 100,
            route: "Subcutaneous".into(),
            administering_vet_id: 8001,
            adverse_reaction: false,
            notes: "Puppy series dose 3/3".into(),
        },
        VaccinationSchedule {
            schedule_id: 60_002,
            patient_id: 200_001,
            vaccine_type: VaccineType::Rabies,
            administered_date_unix: 1_680_100_000,
            next_due_date_unix: 1_774_708_000,
            lot_number: "RAB-2024-C998".into(),
            manufacturer: "Boehringer Ingelheim".into(),
            dose_ml_x100: 100,
            route: "Subcutaneous".into(),
            administering_vet_id: 8001,
            adverse_reaction: false,
            notes: "3-year rabies vaccine".into(),
        },
        VaccinationSchedule {
            schedule_id: 60_003,
            patient_id: 200_001,
            vaccine_type: VaccineType::NonCoreCanine,
            administered_date_unix: 1_695_000_000,
            next_due_date_unix: 1_726_536_000,
            lot_number: "LEPTO-2024-D445".into(),
            manufacturer: "Merck".into(),
            dose_ml_x100: 100,
            route: "Subcutaneous".into(),
            administering_vet_id: 8003,
            adverse_reaction: true,
            notes: "Mild facial swelling 30min post-vaccine; diphenhydramine administered".into(),
        },
    ];
    let encoded = encode_to_vec(&history).expect("encode vaccination history");
    let (decoded, _): (Vec<VaccinationSchedule>, usize) =
        decode_from_slice(&encoded).expect("decode vaccination history");
    assert_eq!(history, decoded);
}

// 19: Exotic species patient via file
#[test]
fn test_vet_exotic_species_patient_file_roundtrip() {
    let path = unique_tmp("exotic_patient");
    let original = PatientRecord {
        patient_id: 300_001,
        name: "Spike".into(),
        species: Species::Exotic {
            description: "Bearded Dragon (Pogona vitticeps)".into(),
        },
        breed: "Central Bearded Dragon".into(),
        age_months: 30,
        weight_grams: 450,
        sex_is_male: true,
        neutered: false,
        owner_name: "Carlos Rivera".into(),
        microchip_id: None,
        allergies: vec![],
        registration_timestamp: 1_703_000_000,
    };
    encode_to_file(&original, &path).expect("encode exotic patient to file");
    let decoded: PatientRecord = decode_from_file(&path).expect("decode exotic patient from file");
    assert_eq!(original, decoded);
    std::fs::remove_file(&path).ok();
}

// 20: Complex surgical log with multiple complications via slice
#[test]
fn test_vet_complex_surgery_complications_slice_roundtrip() {
    let original = SurgicalProcedureLog {
        procedure_id: 310_001,
        patient_id: 300_002,
        procedure_type: SurgicalProcedureType::Emergency,
        surgeon_id: 7003,
        anesthetist_id: 7004,
        start_timestamp: 1_703_100_000,
        end_timestamp: 1_703_114_400,
        anesthesia_duration_minutes: 240,
        anesthetic_protocol: "Fentanyl/Midazolam induction, Sevoflurane maintenance, Lidocaine CRI"
            .into(),
        complications: vec![
            "Intraoperative hypotension requiring dopamine CRI".into(),
            "Splenic torsion discovered during exploratory".into(),
            "Post-operative hypothermia".into(),
            "Coagulopathy requiring fresh frozen plasma".into(),
        ],
        blood_loss_ml: 320,
        outcome: "GDV corrected, gastropexy performed, splenectomy performed".into(),
    };
    let encoded = encode_to_vec(&original).expect("encode complex surgery");
    let (decoded, _): (SurgicalProcedureLog, usize) =
        decode_from_slice(&encoded).expect("decode complex surgery");
    assert_eq!(original, decoded);
}

// 21: Controlled substance prescription via file
#[test]
fn test_vet_controlled_substance_rx_file_roundtrip() {
    let path = unique_tmp("controlled_rx");
    let original = PrescriptionRecord {
        rx_id: 410_001,
        patient_id: 300_003,
        medication_name: "Tramadol HCl".into(),
        dose_mg_x100: 5_000,
        route: "Oral".into(),
        frequency_hours: 8,
        duration_days: 7,
        prescriber_id: 7001,
        dispensed_quantity: 21,
        refills_remaining: 0,
        controlled_substance: true,
        timestamp: 1_703_200_000,
    };
    encode_to_file(&original, &path).expect("encode controlled substance Rx to file");
    let decoded: PrescriptionRecord =
        decode_from_file(&path).expect("decode controlled substance Rx from file");
    assert_eq!(original, decoded);
    std::fs::remove_file(&path).ok();
}

// 22: Combined zoonotic surveillance with empty and populated fields via file
#[test]
fn test_vet_zoonotic_multi_disease_file_roundtrip() {
    let path = unique_tmp("zoonotic_multi");
    let reports = vec![
        ZoonoticSurveillanceReport {
            report_id: 710_001,
            disease: ZoonoticDisease::Leptospirosis,
            region_code: "JP-13".into(),
            report_timestamp: 1_703_300_000,
            confirmed_cases_animal: 12,
            suspected_cases_animal: 25,
            confirmed_cases_human: 2,
            species_involved: vec![Species::Canine, Species::Porcine, Species::Bovine],
            containment_active: true,
            quarantine_radius_km_x100: 500,
            lab_confirmed: true,
            reporting_authority: "MAFF Japan".into(),
        },
        ZoonoticSurveillanceReport {
            report_id: 710_002,
            disease: ZoonoticDisease::AvianInfluenza,
            region_code: "NL-NH".into(),
            report_timestamp: 1_703_400_000,
            confirmed_cases_animal: 0,
            suspected_cases_animal: 3,
            confirmed_cases_human: 0,
            species_involved: vec![Species::Avian],
            containment_active: false,
            quarantine_radius_km_x100: 0,
            lab_confirmed: false,
            reporting_authority: "NVWA Netherlands".into(),
        },
        ZoonoticSurveillanceReport {
            report_id: 710_003,
            disease: ZoonoticDisease::Anthrax,
            region_code: "AU-QLD".into(),
            report_timestamp: 1_703_500_000,
            confirmed_cases_animal: 1,
            suspected_cases_animal: 4,
            confirmed_cases_human: 0,
            species_involved: vec![Species::Bovine, Species::Ovine],
            containment_active: true,
            quarantine_radius_km_x100: 2_000,
            lab_confirmed: true,
            reporting_authority: "Biosecurity Queensland".into(),
        },
    ];
    encode_to_file(&reports, &path).expect("encode zoonotic multi-disease to file");
    let decoded: Vec<ZoonoticSurveillanceReport> =
        decode_from_file(&path).expect("decode zoonotic multi-disease from file");
    assert_eq!(reports, decoded);
    std::fs::remove_file(&path).ok();
}
