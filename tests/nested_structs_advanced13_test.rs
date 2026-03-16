//! Advanced nested structs test #13 — Veterinary medicine and animal health management theme, 22 tests.

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

// ---------------------------------------------------------------------------
// Domain enums
// ---------------------------------------------------------------------------

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
enum Species {
    Canine,
    Feline,
    Equine,
    Bovine,
    Porcine,
    Ovine,
    Caprine,
    Avian,
    Reptile,
    Exotic(String),
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
enum Sex {
    Male,
    Female,
    MaleNeutered,
    FemaleSpayed,
    Unknown,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
enum VaccinationType {
    Core,
    NonCore,
    RiskBased,
    Required,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
enum VaccineRoute {
    Subcutaneous,
    Intramuscular,
    Intranasal,
    Oral,
    Transdermal,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
enum AnesthesiaStage {
    Preinduction,
    Induction,
    Maintenance,
    Recovery,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
enum PainScore {
    None,
    Mild,
    Moderate,
    Severe,
    Excruciating,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
enum ToothCondition {
    Healthy,
    Gingivitis,
    Periodontitis,
    Fractured,
    Resorptive,
    Missing,
    Extracted,
    Deciduous,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
enum DosageUnit {
    MgPerKg,
    MlPerKg,
    UnitsPerKg,
    Mg,
    Ml,
    Drops,
    Tablets,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
enum Frequency {
    OnceDaily,
    TwiceDaily,
    ThreeTimesDaily,
    EveryOtherDay,
    Weekly,
    AsNeeded,
    SingleDose,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
enum LabFlag {
    Normal,
    Low,
    High,
    Critical,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
enum Laterality {
    Left,
    Right,
    Bilateral,
    NotApplicable,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
enum ImagingModality {
    Radiograph,
    Ultrasound,
    CT,
    MRI,
    Fluoroscopy,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
enum BoardingSize {
    Small,
    Medium,
    Large,
    ExtraLarge,
    Pasture,
    Stall,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
enum HerdTestType {
    Tuberculin,
    Brucellosis,
    Johnes,
    BVD,
    Lepto,
    IBR,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
enum SurgeryCategory {
    SoftTissue,
    Orthopedic,
    Ophthalmic,
    Dental,
    Neurologic,
    Oncologic,
    Emergency,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
enum NutrientCategory {
    Protein,
    Fat,
    Fiber,
    Carbohydrate,
    Vitamin,
    Mineral,
    Water,
}

// ---------------------------------------------------------------------------
// Test 1: Patient record with species/breed/weight history
// ---------------------------------------------------------------------------

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct WeightEntry {
    date_epoch_days: u32,
    weight_grams: u64,
    body_condition_score: u8,
    notes: Option<String>,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct BreedInfo {
    primary_breed: String,
    secondary_breed: Option<String>,
    breed_percentage: Option<u8>,
    genetic_test_id: Option<String>,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct OwnerContact {
    name: String,
    phone_primary: String,
    phone_emergency: Option<String>,
    email: Option<String>,
    address_lines: Vec<String>,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct PatientRecord {
    patient_id: u64,
    name: String,
    species: Species,
    sex: Sex,
    breed: BreedInfo,
    date_of_birth_epoch: Option<u32>,
    microchip_id: Option<String>,
    weight_history: Vec<WeightEntry>,
    owners: Vec<OwnerContact>,
    allergies: Vec<String>,
    chronic_conditions: Vec<String>,
}

#[test]
fn test_patient_record_with_weight_history() {
    let record = PatientRecord {
        patient_id: 100234,
        name: "Bella".into(),
        species: Species::Canine,
        sex: Sex::FemaleSpayed,
        breed: BreedInfo {
            primary_breed: "Labrador Retriever".into(),
            secondary_breed: Some("Golden Retriever".into()),
            breed_percentage: Some(75),
            genetic_test_id: Some("EMB-2025-44821".into()),
        },
        date_of_birth_epoch: Some(19450),
        microchip_id: Some("985121033456789".into()),
        weight_history: vec![
            WeightEntry {
                date_epoch_days: 20100,
                weight_grams: 28500,
                body_condition_score: 5,
                notes: Some("Ideal weight".into()),
            },
            WeightEntry {
                date_epoch_days: 20200,
                weight_grams: 30200,
                body_condition_score: 6,
                notes: Some("Slight weight gain, adjust diet".into()),
            },
            WeightEntry {
                date_epoch_days: 20300,
                weight_grams: 29100,
                body_condition_score: 5,
                notes: None,
            },
        ],
        owners: vec![OwnerContact {
            name: "Tanaka Yuki".into(),
            phone_primary: "090-1234-5678".into(),
            phone_emergency: Some("080-8765-4321".into()),
            email: Some("yuki.tanaka@example.jp".into()),
            address_lines: vec!["Minato-ku Roppongi 1-2-3".into(), "Tokyo 106-0032".into()],
        }],
        allergies: vec!["Chicken protein".into(), "Amoxicillin".into()],
        chronic_conditions: vec!["Hip dysplasia".into()],
    };

    let encoded = encode_to_vec(&record).expect("encode patient record");
    let (decoded, _): (PatientRecord, _) =
        decode_from_slice(&encoded).expect("decode patient record");
    assert_eq!(record, decoded);
}

// ---------------------------------------------------------------------------
// Test 2: Vaccination schedule with booster tracking
// ---------------------------------------------------------------------------

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct VaccineLot {
    manufacturer: String,
    lot_number: String,
    expiration_epoch: u32,
    storage_temp_c_x10: i16,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct VaccineAdministration {
    date_epoch: u32,
    vaccine_name: String,
    lot: VaccineLot,
    route: VaccineRoute,
    site: String,
    administered_by: String,
    reaction_observed: Option<String>,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct BoosterSchedule {
    next_due_epoch: u32,
    interval_days: u32,
    is_overdue: bool,
    reminder_sent: bool,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct VaccinationSeries {
    vaccine_type: VaccinationType,
    disease_target: String,
    administrations: Vec<VaccineAdministration>,
    booster: Option<BoosterSchedule>,
    series_complete: bool,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct VaccinationRecord {
    patient_id: u64,
    series_list: Vec<VaccinationSeries>,
    exemptions: Vec<String>,
    titer_tests: Vec<TiterResult>,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct TiterResult {
    disease: String,
    date_epoch: u32,
    result_value: u32,
    adequate: bool,
    lab_name: String,
}

#[test]
fn test_vaccination_schedule_with_boosters() {
    let record = VaccinationRecord {
        patient_id: 100234,
        series_list: vec![
            VaccinationSeries {
                vaccine_type: VaccinationType::Core,
                disease_target: "Rabies".into(),
                administrations: vec![VaccineAdministration {
                    date_epoch: 20050,
                    vaccine_name: "Imrab 3TF".into(),
                    lot: VaccineLot {
                        manufacturer: "Boehringer Ingelheim".into(),
                        lot_number: "BI-RAB-20250101".into(),
                        expiration_epoch: 20780,
                        storage_temp_c_x10: 40,
                    },
                    route: VaccineRoute::Subcutaneous,
                    site: "Right rear leg".into(),
                    administered_by: "Dr. Sato".into(),
                    reaction_observed: None,
                }],
                booster: Some(BoosterSchedule {
                    next_due_epoch: 21145,
                    interval_days: 1095,
                    is_overdue: false,
                    reminder_sent: false,
                }),
                series_complete: true,
            },
            VaccinationSeries {
                vaccine_type: VaccinationType::Core,
                disease_target: "DHPP".into(),
                administrations: vec![
                    VaccineAdministration {
                        date_epoch: 19500,
                        vaccine_name: "Nobivac DHP".into(),
                        lot: VaccineLot {
                            manufacturer: "MSD Animal Health".into(),
                            lot_number: "MSD-DHP-001".into(),
                            expiration_epoch: 20200,
                            storage_temp_c_x10: 25,
                        },
                        route: VaccineRoute::Subcutaneous,
                        site: "Left shoulder".into(),
                        administered_by: "Dr. Kimura".into(),
                        reaction_observed: Some("Mild swelling at site".into()),
                    },
                    VaccineAdministration {
                        date_epoch: 19521,
                        vaccine_name: "Nobivac DHP".into(),
                        lot: VaccineLot {
                            manufacturer: "MSD Animal Health".into(),
                            lot_number: "MSD-DHP-002".into(),
                            expiration_epoch: 20250,
                            storage_temp_c_x10: 25,
                        },
                        route: VaccineRoute::Subcutaneous,
                        site: "Right shoulder".into(),
                        administered_by: "Dr. Kimura".into(),
                        reaction_observed: None,
                    },
                ],
                booster: Some(BoosterSchedule {
                    next_due_epoch: 20615,
                    interval_days: 365,
                    is_overdue: true,
                    reminder_sent: true,
                }),
                series_complete: true,
            },
        ],
        exemptions: vec!["Leptospirosis - prior adverse reaction".into()],
        titer_tests: vec![TiterResult {
            disease: "Distemper".into(),
            date_epoch: 20300,
            result_value: 256,
            adequate: true,
            lab_name: "VetPath Diagnostics".into(),
        }],
    };

    let encoded = encode_to_vec(&record).expect("encode vaccination record");
    let (decoded, _): (VaccinationRecord, _) =
        decode_from_slice(&encoded).expect("decode vaccination record");
    assert_eq!(record, decoded);
}

// ---------------------------------------------------------------------------
// Test 3: CBC lab panel with reference ranges
// ---------------------------------------------------------------------------

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct ReferenceRange {
    low_x100: i64,
    high_x100: i64,
    unit: String,
    species_specific: bool,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct LabAnalyte {
    name: String,
    code: String,
    value_x100: i64,
    reference: ReferenceRange,
    flag: LabFlag,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct CbcPanel {
    analytes: Vec<LabAnalyte>,
    morphology_notes: Option<String>,
    platelet_estimate: Option<String>,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct LabSubmission {
    accession_number: String,
    patient_id: u64,
    collected_epoch: u32,
    received_epoch: u32,
    reported_epoch: Option<u32>,
    collected_by: String,
    specimen_type: String,
    cbc: Option<CbcPanel>,
}

#[test]
fn test_cbc_panel_with_reference_ranges() {
    let submission = LabSubmission {
        accession_number: "LAB-2025-08812".into(),
        patient_id: 100234,
        collected_epoch: 20300,
        received_epoch: 20300,
        reported_epoch: Some(20301),
        collected_by: "Tech Nakamura".into(),
        specimen_type: "EDTA whole blood".into(),
        cbc: Some(CbcPanel {
            analytes: vec![
                LabAnalyte {
                    name: "WBC".into(),
                    code: "CBC-WBC".into(),
                    value_x100: 1250,
                    reference: ReferenceRange {
                        low_x100: 550,
                        high_x100: 1680,
                        unit: "x10^9/L".into(),
                        species_specific: true,
                    },
                    flag: LabFlag::Normal,
                },
                LabAnalyte {
                    name: "RBC".into(),
                    code: "CBC-RBC".into(),
                    value_x100: 720,
                    reference: ReferenceRange {
                        low_x100: 550,
                        high_x100: 850,
                        unit: "x10^12/L".into(),
                        species_specific: true,
                    },
                    flag: LabFlag::Normal,
                },
                LabAnalyte {
                    name: "HCT".into(),
                    code: "CBC-HCT".into(),
                    value_x100: 5200,
                    reference: ReferenceRange {
                        low_x100: 3700,
                        high_x100: 5500,
                        unit: "%".into(),
                        species_specific: true,
                    },
                    flag: LabFlag::Normal,
                },
                LabAnalyte {
                    name: "PLT".into(),
                    code: "CBC-PLT".into(),
                    value_x100: 8500,
                    reference: ReferenceRange {
                        low_x100: 17500,
                        high_x100: 50000,
                        unit: "x10^9/L".into(),
                        species_specific: true,
                    },
                    flag: LabFlag::Critical,
                },
            ],
            morphology_notes: Some(
                "Occasional target cells noted. Large platelets present.".into(),
            ),
            platelet_estimate: Some("Decreased".into()),
        }),
    };

    let encoded = encode_to_vec(&submission).expect("encode lab submission");
    let (decoded, _): (LabSubmission, _) =
        decode_from_slice(&encoded).expect("decode lab submission");
    assert_eq!(submission, decoded);
}

// ---------------------------------------------------------------------------
// Test 4: Chemistry panel with organ-grouped results
// ---------------------------------------------------------------------------

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct OrganPanel {
    organ_system: String,
    analytes: Vec<LabAnalyte>,
    clinical_significance: Option<String>,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct ChemistryReport {
    accession: String,
    fasting: bool,
    hemolysis_index: u8,
    lipemia_index: u8,
    panels: Vec<OrganPanel>,
    pathologist_comment: Option<String>,
}

#[test]
fn test_chemistry_panel_organ_grouped() {
    let report = ChemistryReport {
        accession: "LAB-2025-08813".into(),
        fasting: true,
        hemolysis_index: 0,
        lipemia_index: 1,
        panels: vec![
            OrganPanel {
                organ_system: "Hepatic".into(),
                analytes: vec![
                    LabAnalyte {
                        name: "ALT".into(),
                        code: "CHEM-ALT".into(),
                        value_x100: 4500,
                        reference: ReferenceRange {
                            low_x100: 1000,
                            high_x100: 12500,
                            unit: "U/L".into(),
                            species_specific: true,
                        },
                        flag: LabFlag::Normal,
                    },
                    LabAnalyte {
                        name: "ALP".into(),
                        code: "CHEM-ALP".into(),
                        value_x100: 28000,
                        reference: ReferenceRange {
                            low_x100: 2300,
                            high_x100: 21200,
                            unit: "U/L".into(),
                            species_specific: true,
                        },
                        flag: LabFlag::High,
                    },
                ],
                clinical_significance: Some("Elevated ALP may indicate cholestasis".into()),
            },
            OrganPanel {
                organ_system: "Renal".into(),
                analytes: vec![
                    LabAnalyte {
                        name: "BUN".into(),
                        code: "CHEM-BUN".into(),
                        value_x100: 1800,
                        reference: ReferenceRange {
                            low_x100: 700,
                            high_x100: 2700,
                            unit: "mg/dL".into(),
                            species_specific: true,
                        },
                        flag: LabFlag::Normal,
                    },
                    LabAnalyte {
                        name: "Creatinine".into(),
                        code: "CHEM-CREA".into(),
                        value_x100: 120,
                        reference: ReferenceRange {
                            low_x100: 50,
                            high_x100: 180,
                            unit: "mg/dL".into(),
                            species_specific: true,
                        },
                        flag: LabFlag::Normal,
                    },
                ],
                clinical_significance: None,
            },
        ],
        pathologist_comment: Some("Elevated ALP warrants follow-up bile acids test.".into()),
    };

    let encoded = encode_to_vec(&report).expect("encode chemistry report");
    let (decoded, _): (ChemistryReport, _) =
        decode_from_slice(&encoded).expect("decode chemistry report");
    assert_eq!(report, decoded);
}

// ---------------------------------------------------------------------------
// Test 5: Urinalysis with sediment findings
// ---------------------------------------------------------------------------

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct UrinalysisDipstick {
    ph_x10: u8,
    specific_gravity_x1000: u16,
    protein: u8,
    glucose: u8,
    ketones: u8,
    bilirubin: u8,
    blood: u8,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct SedimentFinding {
    element: String,
    quantity_per_hpf: String,
    significance: Option<String>,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct UrinalysisReport {
    collection_method: String,
    color: String,
    clarity: String,
    dipstick: UrinalysisDipstick,
    sediment: Vec<SedimentFinding>,
    culture_submitted: bool,
}

#[test]
fn test_urinalysis_with_sediment() {
    let report = UrinalysisReport {
        collection_method: "Cystocentesis".into(),
        color: "Yellow".into(),
        clarity: "Slightly turbid".into(),
        dipstick: UrinalysisDipstick {
            ph_x10: 65,
            specific_gravity_x1000: 1035,
            protein: 1,
            glucose: 0,
            ketones: 0,
            bilirubin: 0,
            blood: 2,
        },
        sediment: vec![
            SedimentFinding {
                element: "RBC".into(),
                quantity_per_hpf: "5-10".into(),
                significance: Some("Mild hematuria".into()),
            },
            SedimentFinding {
                element: "WBC".into(),
                quantity_per_hpf: "0-2".into(),
                significance: None,
            },
            SedimentFinding {
                element: "Struvite crystals".into(),
                quantity_per_hpf: "Moderate".into(),
                significance: Some("Consistent with alkaline urine".into()),
            },
        ],
        culture_submitted: true,
    };

    let encoded = encode_to_vec(&report).expect("encode urinalysis");
    let (decoded, _): (UrinalysisReport, _) =
        decode_from_slice(&encoded).expect("decode urinalysis");
    assert_eq!(report, decoded);
}

// ---------------------------------------------------------------------------
// Test 6: Surgical procedure log with anesthesia monitoring
// ---------------------------------------------------------------------------

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct VitalReading {
    elapsed_minutes: u16,
    heart_rate_bpm: u16,
    resp_rate: u16,
    spo2_percent: u8,
    etco2_mmhg: Option<u8>,
    systolic_bp: Option<u16>,
    diastolic_bp: Option<u16>,
    temp_c_x10: u16,
    anesthesia_stage: AnesthesiaStage,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct AnesthesiaDrug {
    drug_name: String,
    dose_mg_per_kg_x100: u32,
    route: String,
    time_elapsed_min: u16,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct AnesthesiaLog {
    protocol: String,
    induction_agent: String,
    maintenance_agent: String,
    drugs: Vec<AnesthesiaDrug>,
    vitals: Vec<VitalReading>,
    complications: Vec<String>,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct SurgicalProcedure {
    procedure_id: u64,
    patient_id: u64,
    date_epoch: u32,
    category: SurgeryCategory,
    procedure_name: String,
    surgeon: String,
    assistant: Option<String>,
    duration_minutes: u16,
    anesthesia: AnesthesiaLog,
    findings: String,
    complications: Vec<String>,
    post_op_pain_score: PainScore,
}

#[test]
fn test_surgical_procedure_with_anesthesia() {
    let procedure = SurgicalProcedure {
        procedure_id: 5001,
        patient_id: 100234,
        date_epoch: 20310,
        category: SurgeryCategory::Orthopedic,
        procedure_name: "Tibial Plateau Leveling Osteotomy (TPLO)".into(),
        surgeon: "Dr. Watanabe".into(),
        assistant: Some("Dr. Ito".into()),
        duration_minutes: 95,
        anesthesia: AnesthesiaLog {
            protocol: "Balanced anesthesia".into(),
            induction_agent: "Propofol".into(),
            maintenance_agent: "Isoflurane".into(),
            drugs: vec![
                AnesthesiaDrug {
                    drug_name: "Acepromazine".into(),
                    dose_mg_per_kg_x100: 2,
                    route: "IM".into(),
                    time_elapsed_min: 0,
                },
                AnesthesiaDrug {
                    drug_name: "Hydromorphone".into(),
                    dose_mg_per_kg_x100: 10,
                    route: "IM".into(),
                    time_elapsed_min: 0,
                },
                AnesthesiaDrug {
                    drug_name: "Propofol".into(),
                    dose_mg_per_kg_x100: 400,
                    route: "IV".into(),
                    time_elapsed_min: 20,
                },
            ],
            vitals: vec![
                VitalReading {
                    elapsed_minutes: 25,
                    heart_rate_bpm: 88,
                    resp_rate: 12,
                    spo2_percent: 98,
                    etco2_mmhg: Some(38),
                    systolic_bp: Some(110),
                    diastolic_bp: Some(70),
                    temp_c_x10: 384,
                    anesthesia_stage: AnesthesiaStage::Maintenance,
                },
                VitalReading {
                    elapsed_minutes: 55,
                    heart_rate_bpm: 92,
                    resp_rate: 14,
                    spo2_percent: 97,
                    etco2_mmhg: Some(40),
                    systolic_bp: Some(105),
                    diastolic_bp: Some(65),
                    temp_c_x10: 378,
                    anesthesia_stage: AnesthesiaStage::Maintenance,
                },
                VitalReading {
                    elapsed_minutes: 100,
                    heart_rate_bpm: 110,
                    resp_rate: 20,
                    spo2_percent: 99,
                    etco2_mmhg: None,
                    systolic_bp: None,
                    diastolic_bp: None,
                    temp_c_x10: 374,
                    anesthesia_stage: AnesthesiaStage::Recovery,
                },
            ],
            complications: vec![],
        },
        findings: "Complete cranial cruciate ligament rupture confirmed. Meniscal release performed. Plate angle 5 degrees.".into(),
        complications: vec![],
        post_op_pain_score: PainScore::Moderate,
    };

    let encoded = encode_to_vec(&procedure).expect("encode surgical procedure");
    let (decoded, _): (SurgicalProcedure, _) =
        decode_from_slice(&encoded).expect("decode surgical procedure");
    assert_eq!(procedure, decoded);
}

// ---------------------------------------------------------------------------
// Test 7: Prescription records with dosing regimens
// ---------------------------------------------------------------------------

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct DoseInstruction {
    amount_x100: u32,
    unit: DosageUnit,
    frequency: Frequency,
    with_food: bool,
    special_instructions: Option<String>,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct RefillInfo {
    refills_authorized: u8,
    refills_used: u8,
    last_refill_epoch: Option<u32>,
    pharmacy: Option<String>,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct Prescription {
    rx_number: u64,
    drug_name: String,
    strength: String,
    dose: DoseInstruction,
    duration_days: Option<u16>,
    start_epoch: u32,
    prescriber: String,
    refill: RefillInfo,
    warnings: Vec<String>,
    drug_interactions: Vec<String>,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct PrescriptionHistory {
    patient_id: u64,
    active_prescriptions: Vec<Prescription>,
    past_prescriptions: Vec<Prescription>,
}

#[test]
fn test_prescription_records_with_dosing() {
    let history = PrescriptionHistory {
        patient_id: 100234,
        active_prescriptions: vec![
            Prescription {
                rx_number: 900100,
                drug_name: "Carprofen".into(),
                strength: "75mg".into(),
                dose: DoseInstruction {
                    amount_x100: 220,
                    unit: DosageUnit::MgPerKg,
                    frequency: Frequency::TwiceDaily,
                    with_food: true,
                    special_instructions: Some("Monitor for GI upset".into()),
                },
                duration_days: Some(14),
                start_epoch: 20310,
                prescriber: "Dr. Watanabe".into(),
                refill: RefillInfo {
                    refills_authorized: 1,
                    refills_used: 0,
                    last_refill_epoch: None,
                    pharmacy: None,
                },
                warnings: vec!["NSAID - do not combine with corticosteroids".into()],
                drug_interactions: vec!["Aspirin".into(), "Meloxicam".into()],
            },
            Prescription {
                rx_number: 900101,
                drug_name: "Gabapentin".into(),
                strength: "100mg capsules".into(),
                dose: DoseInstruction {
                    amount_x100: 500,
                    unit: DosageUnit::MgPerKg,
                    frequency: Frequency::ThreeTimesDaily,
                    with_food: false,
                    special_instructions: Some("May cause sedation".into()),
                },
                duration_days: Some(30),
                start_epoch: 20310,
                prescriber: "Dr. Watanabe".into(),
                refill: RefillInfo {
                    refills_authorized: 2,
                    refills_used: 0,
                    last_refill_epoch: None,
                    pharmacy: Some("Pet Pharmacy Central".into()),
                },
                warnings: vec![],
                drug_interactions: vec![],
            },
        ],
        past_prescriptions: vec![Prescription {
            rx_number: 800050,
            drug_name: "Amoxicillin-Clavulanate".into(),
            strength: "250mg".into(),
            dose: DoseInstruction {
                amount_x100: 1375,
                unit: DosageUnit::MgPerKg,
                frequency: Frequency::TwiceDaily,
                with_food: true,
                special_instructions: None,
            },
            duration_days: Some(10),
            start_epoch: 20100,
            prescriber: "Dr. Sato".into(),
            refill: RefillInfo {
                refills_authorized: 0,
                refills_used: 0,
                last_refill_epoch: None,
                pharmacy: None,
            },
            warnings: vec![
                "Patient has recorded allergy to Amoxicillin - OVERRIDE by prescriber".into(),
            ],
            drug_interactions: vec![],
        }],
    };

    let encoded = encode_to_vec(&history).expect("encode prescription history");
    let (decoded, _): (PrescriptionHistory, _) =
        decode_from_slice(&encoded).expect("decode prescription history");
    assert_eq!(history, decoded);
}

// ---------------------------------------------------------------------------
// Test 8: Kennel boarding reservation
// ---------------------------------------------------------------------------

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct FeedingSchedule {
    time_of_day: String,
    food_type: String,
    amount: String,
    special_prep: Option<String>,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct MedicationDuringBoarding {
    drug_name: String,
    dose_instructions: String,
    frequency: Frequency,
    supplied_by_owner: bool,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct BoardingReservation {
    reservation_id: u64,
    patient_id: u64,
    check_in_epoch: u32,
    check_out_epoch: u32,
    kennel_size: BoardingSize,
    kennel_number: Option<String>,
    feeding: Vec<FeedingSchedule>,
    medications: Vec<MedicationDuringBoarding>,
    exercise_notes: Option<String>,
    behavioral_notes: Option<String>,
    emergency_contact: OwnerContact,
    vaccination_verified: bool,
    belongings: Vec<String>,
}

#[test]
fn test_kennel_boarding_reservation() {
    let reservation = BoardingReservation {
        reservation_id: 3001,
        patient_id: 100234,
        check_in_epoch: 20400,
        check_out_epoch: 20407,
        kennel_size: BoardingSize::Large,
        kennel_number: Some("L-14".into()),
        feeding: vec![
            FeedingSchedule {
                time_of_day: "07:00".into(),
                food_type: "Royal Canin GI Low Fat".into(),
                amount: "1.5 cups".into(),
                special_prep: Some("Add warm water, wait 5 min".into()),
            },
            FeedingSchedule {
                time_of_day: "17:00".into(),
                food_type: "Royal Canin GI Low Fat".into(),
                amount: "1.5 cups".into(),
                special_prep: Some("Add warm water, wait 5 min".into()),
            },
        ],
        medications: vec![MedicationDuringBoarding {
            drug_name: "Gabapentin 100mg".into(),
            dose_instructions: "1 capsule by mouth".into(),
            frequency: Frequency::ThreeTimesDaily,
            supplied_by_owner: true,
        }],
        exercise_notes: Some(
            "Leash walks only - post-surgical recovery. No running or jumping.".into(),
        ),
        behavioral_notes: Some("Anxious around cats. Friendly with other dogs.".into()),
        emergency_contact: OwnerContact {
            name: "Tanaka Yuki".into(),
            phone_primary: "090-1234-5678".into(),
            phone_emergency: Some("080-8765-4321".into()),
            email: Some("yuki.tanaka@example.jp".into()),
            address_lines: vec!["Minato-ku Roppongi 1-2-3".into()],
        },
        vaccination_verified: true,
        belongings: vec![
            "Blue blanket".into(),
            "Tennis ball".into(),
            "Medication bag".into(),
        ],
    };

    let encoded = encode_to_vec(&reservation).expect("encode boarding reservation");
    let (decoded, _): (BoardingReservation, _) =
        decode_from_slice(&encoded).expect("decode boarding reservation");
    assert_eq!(reservation, decoded);
}

// ---------------------------------------------------------------------------
// Test 9: Livestock herd health program
// ---------------------------------------------------------------------------

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct HerdAnimal {
    ear_tag: String,
    rfid: Option<String>,
    sex: Sex,
    birth_date_epoch: Option<u32>,
    dam_tag: Option<String>,
    sire_tag: Option<String>,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct HerdTest {
    test_type: HerdTestType,
    date_epoch: u32,
    animals_tested: u32,
    positives: u32,
    inconclusive: u32,
    lab_report_id: Option<String>,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct TreatmentProtocol {
    condition: String,
    drug: String,
    withdrawal_days_meat: u16,
    withdrawal_days_milk: u16,
    dosage_instructions: String,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct HerdHealthProgram {
    herd_id: String,
    species: Species,
    total_head: u32,
    location: String,
    animals: Vec<HerdAnimal>,
    test_history: Vec<HerdTest>,
    protocols: Vec<TreatmentProtocol>,
    next_vet_visit_epoch: Option<u32>,
    certifications: Vec<String>,
}

#[test]
fn test_livestock_herd_health_program() {
    let program = HerdHealthProgram {
        herd_id: "HERD-JP-2025-0042".into(),
        species: Species::Bovine,
        total_head: 85,
        location: "Hokkaido Dairy Farm #7".into(),
        animals: vec![
            HerdAnimal {
                ear_tag: "JP-0042-0001".into(),
                rfid: Some("840003148812345".into()),
                sex: Sex::Female,
                birth_date_epoch: Some(18900),
                dam_tag: Some("JP-0042-0089".into()),
                sire_tag: Some("AI-HOLST-9921".into()),
            },
            HerdAnimal {
                ear_tag: "JP-0042-0002".into(),
                rfid: Some("840003148812346".into()),
                sex: Sex::Female,
                birth_date_epoch: Some(19100),
                dam_tag: None,
                sire_tag: None,
            },
        ],
        test_history: vec![
            HerdTest {
                test_type: HerdTestType::Tuberculin,
                date_epoch: 20200,
                animals_tested: 85,
                positives: 0,
                inconclusive: 1,
                lab_report_id: Some("TB-20250301-042".into()),
            },
            HerdTest {
                test_type: HerdTestType::BVD,
                date_epoch: 20200,
                animals_tested: 85,
                positives: 2,
                inconclusive: 0,
                lab_report_id: Some("BVD-20250301-042".into()),
            },
        ],
        protocols: vec![TreatmentProtocol {
            condition: "Clinical mastitis".into(),
            drug: "Ceftiofur".into(),
            withdrawal_days_meat: 13,
            withdrawal_days_milk: 4,
            dosage_instructions: "1 mg/kg IM SID x 5 days".into(),
        }],
        next_vet_visit_epoch: Some(20500),
        certifications: vec!["TB-Free Status".into(), "Brucellosis-Free Status".into()],
    };

    let encoded = encode_to_vec(&program).expect("encode herd health program");
    let (decoded, _): (HerdHealthProgram, _) =
        decode_from_slice(&encoded).expect("decode herd health program");
    assert_eq!(program, decoded);
}

// ---------------------------------------------------------------------------
// Test 10: Radiology study hierarchy (study → series → images)
// ---------------------------------------------------------------------------

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct RadiologyImage {
    image_uid: String,
    instance_number: u32,
    laterality: Laterality,
    position: String,
    kvp: u16,
    mas: u16,
    exposure_time_ms: u32,
    file_size_bytes: u64,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct RadiologySeries {
    series_uid: String,
    series_number: u32,
    modality: ImagingModality,
    body_part: String,
    description: String,
    images: Vec<RadiologyImage>,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct RadiologyFinding {
    location: String,
    description: String,
    severity: String,
    differential_diagnoses: Vec<String>,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct RadiologyStudy {
    study_uid: String,
    patient_id: u64,
    date_epoch: u32,
    referring_vet: String,
    radiologist: Option<String>,
    clinical_indication: String,
    series: Vec<RadiologySeries>,
    findings: Vec<RadiologyFinding>,
    impression: Option<String>,
    recommendations: Vec<String>,
}

#[test]
fn test_radiology_study_hierarchy() {
    let study = RadiologyStudy {
        study_uid: "1.2.826.0.1.3680043.8.1055.1.20250301.1234".into(),
        patient_id: 100234,
        date_epoch: 20280,
        referring_vet: "Dr. Sato".into(),
        radiologist: Some("Dr. Yamamoto (DACVR)".into()),
        clinical_indication: "Right pelvic limb lameness, suspected CCL rupture".into(),
        series: vec![
            RadiologySeries {
                series_uid: "1.2.826.0.1.3680043.8.1055.1.20250301.1234.1".into(),
                series_number: 1,
                modality: ImagingModality::Radiograph,
                body_part: "Right stifle".into(),
                description: "Lateral view".into(),
                images: vec![RadiologyImage {
                    image_uid: "1.2.826.0.1.3680043.8.1055.1.20250301.1234.1.1".into(),
                    instance_number: 1,
                    laterality: Laterality::Right,
                    position: "Lateral".into(),
                    kvp: 55,
                    mas: 8,
                    exposure_time_ms: 20,
                    file_size_bytes: 4_200_000,
                }],
            },
            RadiologySeries {
                series_uid: "1.2.826.0.1.3680043.8.1055.1.20250301.1234.2".into(),
                series_number: 2,
                modality: ImagingModality::Radiograph,
                body_part: "Right stifle".into(),
                description: "CrCd view".into(),
                images: vec![RadiologyImage {
                    image_uid: "1.2.826.0.1.3680043.8.1055.1.20250301.1234.2.1".into(),
                    instance_number: 1,
                    laterality: Laterality::Right,
                    position: "CrCd".into(),
                    kvp: 60,
                    mas: 10,
                    exposure_time_ms: 25,
                    file_size_bytes: 4_500_000,
                }],
            },
        ],
        findings: vec![
            RadiologyFinding {
                location: "Right stifle joint".into(),
                description: "Moderate joint effusion with cranial tibial thrust".into(),
                severity: "Moderate".into(),
                differential_diagnoses: vec![
                    "Cranial cruciate ligament rupture".into(),
                    "Meniscal injury".into(),
                ],
            },
            RadiologyFinding {
                location: "Right femoral trochlea".into(),
                description: "Mild osteophyte formation on trochlear ridges".into(),
                severity: "Mild".into(),
                differential_diagnoses: vec!["Degenerative joint disease".into()],
            },
        ],
        impression: Some(
            "Findings consistent with cranial cruciate ligament rupture with secondary DJD.".into(),
        ),
        recommendations: vec![
            "Surgical consultation for TPLO".into(),
            "Follow-up radiographs in 8 weeks post-surgery".into(),
        ],
    };

    let encoded = encode_to_vec(&study).expect("encode radiology study");
    let (decoded, _): (RadiologyStudy, _) =
        decode_from_slice(&encoded).expect("decode radiology study");
    assert_eq!(study, decoded);
}

// ---------------------------------------------------------------------------
// Test 11: Dental charting
// ---------------------------------------------------------------------------

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct ToothRecord {
    tooth_number: u16,
    quadrant: u8,
    condition: ToothCondition,
    pocket_depths_mm: Vec<u8>,
    mobility_grade: u8,
    notes: Option<String>,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct DentalProcedureEntry {
    tooth_number: u16,
    procedure: String,
    material_used: Option<String>,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct DentalChart {
    patient_id: u64,
    date_epoch: u32,
    veterinarian: String,
    dental_grade: u8,
    teeth: Vec<ToothRecord>,
    procedures_performed: Vec<DentalProcedureEntry>,
    scaling_performed: bool,
    polishing_performed: bool,
    fluoride_applied: bool,
    dental_radiographs_taken: u8,
    home_care_recommendations: Vec<String>,
}

#[test]
fn test_dental_charting() {
    let chart = DentalChart {
        patient_id: 200100,
        date_epoch: 20350,
        veterinarian: "Dr. Kobayashi".into(),
        dental_grade: 3,
        teeth: vec![
            ToothRecord {
                tooth_number: 108,
                quadrant: 1,
                condition: ToothCondition::Periodontitis,
                pocket_depths_mm: vec![4, 5, 6, 4],
                mobility_grade: 2,
                notes: Some("Stage 3 periodontal disease, recommend extraction".into()),
            },
            ToothRecord {
                tooth_number: 204,
                quadrant: 2,
                condition: ToothCondition::Fractured,
                pocket_depths_mm: vec![2, 2, 3, 2],
                mobility_grade: 0,
                notes: Some("Complicated crown fracture with pulp exposure".into()),
            },
            ToothRecord {
                tooth_number: 309,
                quadrant: 3,
                condition: ToothCondition::Resorptive,
                pocket_depths_mm: vec![3, 4, 5, 3],
                mobility_grade: 1,
                notes: Some("Type 2 tooth resorption on radiograph".into()),
            },
            ToothRecord {
                tooth_number: 404,
                quadrant: 4,
                condition: ToothCondition::Healthy,
                pocket_depths_mm: vec![1, 1, 2, 1],
                mobility_grade: 0,
                notes: None,
            },
        ],
        procedures_performed: vec![
            DentalProcedureEntry {
                tooth_number: 108,
                procedure: "Surgical extraction".into(),
                material_used: Some("Absorbable suture 4-0".into()),
            },
            DentalProcedureEntry {
                tooth_number: 204,
                procedure: "Vital pulp therapy".into(),
                material_used: Some("MTA cement".into()),
            },
            DentalProcedureEntry {
                tooth_number: 309,
                procedure: "Crown amputation".into(),
                material_used: None,
            },
        ],
        scaling_performed: true,
        polishing_performed: true,
        fluoride_applied: true,
        dental_radiographs_taken: 6,
        home_care_recommendations: vec![
            "Daily tooth brushing with enzymatic paste".into(),
            "Dental chews (VOHC accepted)".into(),
            "Recheck in 6 months".into(),
        ],
    };

    let encoded = encode_to_vec(&chart).expect("encode dental chart");
    let (decoded, _): (DentalChart, _) = decode_from_slice(&encoded).expect("decode dental chart");
    assert_eq!(chart, decoded);
}

// ---------------------------------------------------------------------------
// Test 12: Feline patient with multi-disease monitoring
// ---------------------------------------------------------------------------

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct BloodPressureReading {
    date_epoch: u32,
    systolic: u16,
    diastolic: u16,
    method: String,
    limb_used: String,
    readings_averaged: u8,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct ThyroidMonitoring {
    t4_values: Vec<(u32, u32)>,
    medication: Option<String>,
    current_dose_mg_x100: Option<u32>,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct RenalMonitoring {
    sdma_values: Vec<(u32, u32)>,
    iris_stage: u8,
    on_renal_diet: bool,
    sub_q_fluids: bool,
    fluid_volume_ml: Option<u16>,
    fluid_frequency: Option<Frequency>,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct FelineMultiDiseaseProfile {
    patient_id: u64,
    name: String,
    thyroid: ThyroidMonitoring,
    renal: RenalMonitoring,
    blood_pressure_history: Vec<BloodPressureReading>,
    current_medications: Vec<String>,
}

#[test]
fn test_feline_multi_disease_monitoring() {
    let profile = FelineMultiDiseaseProfile {
        patient_id: 200200,
        name: "Mochi".into(),
        thyroid: ThyroidMonitoring {
            t4_values: vec![(20100, 680), (20200, 420), (20300, 380)],
            medication: Some("Methimazole".into()),
            current_dose_mg_x100: Some(250),
        },
        renal: RenalMonitoring {
            sdma_values: vec![(20100, 18), (20200, 22), (20300, 25)],
            iris_stage: 2,
            on_renal_diet: true,
            sub_q_fluids: true,
            fluid_volume_ml: Some(150),
            fluid_frequency: Some(Frequency::EveryOtherDay),
        },
        blood_pressure_history: vec![
            BloodPressureReading {
                date_epoch: 20200,
                systolic: 165,
                diastolic: 95,
                method: "Doppler".into(),
                limb_used: "Right forelimb".into(),
                readings_averaged: 5,
            },
            BloodPressureReading {
                date_epoch: 20300,
                systolic: 148,
                diastolic: 88,
                method: "Doppler".into(),
                limb_used: "Right forelimb".into(),
                readings_averaged: 5,
            },
        ],
        current_medications: vec![
            "Methimazole 2.5mg PO BID".into(),
            "Amlodipine 0.625mg PO SID".into(),
            "Aluminum hydroxide with meals".into(),
        ],
    };

    let encoded = encode_to_vec(&profile).expect("encode feline profile");
    let (decoded, _): (FelineMultiDiseaseProfile, _) =
        decode_from_slice(&encoded).expect("decode feline profile");
    assert_eq!(profile, decoded);
}

// ---------------------------------------------------------------------------
// Test 13: Equine lameness evaluation
// ---------------------------------------------------------------------------

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct FlexionTest {
    joint: String,
    duration_seconds: u8,
    lameness_before: u8,
    lameness_after: u8,
    limb: Laterality,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct NerveBlock {
    block_name: String,
    agent: String,
    volume_ml_x10: u16,
    response: String,
    improved_percent: u8,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct LamenessExam {
    patient_id: u64,
    date_epoch: u32,
    examiner: String,
    primary_complaint: String,
    grade_aaep: u8,
    affected_limb: Laterality,
    gait_observations: Vec<String>,
    flexion_tests: Vec<FlexionTest>,
    nerve_blocks: Vec<NerveBlock>,
    imaging_recommended: Vec<ImagingModality>,
    diagnosis: Option<String>,
    treatment_plan: Vec<String>,
}

#[test]
fn test_equine_lameness_evaluation() {
    let exam = LamenessExam {
        patient_id: 300100,
        date_epoch: 20310,
        examiner: "Dr. Hashimoto (DACVS)".into(),
        primary_complaint: "Intermittent right forelimb lameness at trot".into(),
        grade_aaep: 2,
        affected_limb: Laterality::Right,
        gait_observations: vec![
            "Head bob positive at trot on hard surface".into(),
            "No obvious lameness at walk".into(),
            "Worsens on right circle".into(),
        ],
        flexion_tests: vec![
            FlexionTest {
                joint: "Distal interphalangeal".into(),
                duration_seconds: 60,
                lameness_before: 2,
                lameness_after: 3,
                limb: Laterality::Right,
            },
            FlexionTest {
                joint: "Metacarpophalangeal".into(),
                duration_seconds: 60,
                lameness_before: 2,
                lameness_after: 2,
                limb: Laterality::Right,
            },
        ],
        nerve_blocks: vec![NerveBlock {
            block_name: "Palmar digital nerve block".into(),
            agent: "Mepivacaine 2%".into(),
            volume_ml_x10: 30,
            response: "80% improvement at trot".into(),
            improved_percent: 80,
        }],
        imaging_recommended: vec![ImagingModality::Radiograph, ImagingModality::MRI],
        diagnosis: Some("Navicular syndrome, right forelimb".into()),
        treatment_plan: vec![
            "Corrective shoeing with egg bar shoes".into(),
            "Isoxsuprine 0.6 mg/kg PO BID x 60 days".into(),
            "Follow-up lameness exam in 60 days".into(),
        ],
    };

    let encoded = encode_to_vec(&exam).expect("encode lameness exam");
    let (decoded, _): (LamenessExam, _) =
        decode_from_slice(&encoded).expect("decode lameness exam");
    assert_eq!(exam, decoded);
}

// ---------------------------------------------------------------------------
// Test 14: Nutrition plan with nutrient analysis
// ---------------------------------------------------------------------------

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct NutrientContent {
    category: NutrientCategory,
    name: String,
    amount_per_kg_x100: u64,
    unit: String,
    meets_aafco: bool,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct DietComponent {
    food_name: String,
    manufacturer: String,
    daily_amount: String,
    calories_per_serving: u32,
    nutrients: Vec<NutrientContent>,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct NutritionPlan {
    patient_id: u64,
    target_weight_grams: u64,
    daily_calorie_target: u32,
    rer_calories: u32,
    activity_factor_x100: u16,
    diet_components: Vec<DietComponent>,
    supplements: Vec<String>,
    feeding_guidelines: Vec<String>,
    review_date_epoch: u32,
}

#[test]
fn test_nutrition_plan_with_analysis() {
    let plan = NutritionPlan {
        patient_id: 100234,
        target_weight_grams: 29000,
        daily_calorie_target: 1100,
        rer_calories: 786,
        activity_factor_x100: 140,
        diet_components: vec![
            DietComponent {
                food_name: "Royal Canin Gastrointestinal Low Fat".into(),
                manufacturer: "Royal Canin".into(),
                daily_amount: "3 cups divided into 2 meals".into(),
                calories_per_serving: 266,
                nutrients: vec![
                    NutrientContent {
                        category: NutrientCategory::Protein,
                        name: "Crude Protein".into(),
                        amount_per_kg_x100: 2200,
                        unit: "g/kg".into(),
                        meets_aafco: true,
                    },
                    NutrientContent {
                        category: NutrientCategory::Fat,
                        name: "Crude Fat".into(),
                        amount_per_kg_x100: 700,
                        unit: "g/kg".into(),
                        meets_aafco: true,
                    },
                    NutrientContent {
                        category: NutrientCategory::Fiber,
                        name: "Crude Fiber".into(),
                        amount_per_kg_x100: 500,
                        unit: "g/kg".into(),
                        meets_aafco: true,
                    },
                ],
            },
            DietComponent {
                food_name: "Green beans (canned, no salt)".into(),
                manufacturer: "Generic".into(),
                daily_amount: "0.5 cup per meal".into(),
                calories_per_serving: 18,
                nutrients: vec![NutrientContent {
                    category: NutrientCategory::Fiber,
                    name: "Dietary Fiber".into(),
                    amount_per_kg_x100: 2700,
                    unit: "g/kg".into(),
                    meets_aafco: true,
                }],
            },
        ],
        supplements: vec![
            "Omega-3 fish oil 1000mg EPA+DHA daily".into(),
            "Glucosamine/Chondroitin joint supplement".into(),
        ],
        feeding_guidelines: vec![
            "Feed measured amounts only".into(),
            "No table scraps".into(),
            "Weigh weekly and record".into(),
        ],
        review_date_epoch: 20400,
    };

    let encoded = encode_to_vec(&plan).expect("encode nutrition plan");
    let (decoded, _): (NutritionPlan, _) =
        decode_from_slice(&encoded).expect("decode nutrition plan");
    assert_eq!(plan, decoded);
}

// ---------------------------------------------------------------------------
// Test 15: Exotic pet (reptile) husbandry record
// ---------------------------------------------------------------------------

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct EnclosureParams {
    length_cm: u16,
    width_cm: u16,
    height_cm: u16,
    substrate: String,
    basking_temp_c_x10: u16,
    cool_side_temp_c_x10: u16,
    humidity_percent: u8,
    uv_index: u8,
    light_cycle_hours: u8,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct SheddingRecord {
    date_epoch: u32,
    complete: bool,
    issues: Option<String>,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct ExoticPetRecord {
    patient_id: u64,
    common_name: String,
    scientific_name: String,
    species: Species,
    sex: Sex,
    length_cm: Option<u16>,
    weight_grams: u64,
    enclosure: EnclosureParams,
    diet_items: Vec<String>,
    supplement_schedule: Vec<String>,
    shedding_history: Vec<SheddingRecord>,
    parasite_history: Vec<String>,
    husbandry_notes: Vec<String>,
}

#[test]
fn test_exotic_pet_reptile_husbandry() {
    let record = ExoticPetRecord {
        patient_id: 400100,
        common_name: "Ball Python".into(),
        scientific_name: "Python regius".into(),
        species: Species::Reptile,
        sex: Sex::Female,
        length_cm: Some(130),
        weight_grams: 1800,
        enclosure: EnclosureParams {
            length_cm: 120,
            width_cm: 60,
            height_cm: 45,
            substrate: "Coconut fiber".into(),
            basking_temp_c_x10: 330,
            cool_side_temp_c_x10: 265,
            humidity_percent: 60,
            uv_index: 2,
            light_cycle_hours: 12,
        },
        diet_items: vec!["Medium rat every 14 days".into(), "Occasional chick".into()],
        supplement_schedule: vec!["Calcium dusting every other feeding".into()],
        shedding_history: vec![
            SheddingRecord {
                date_epoch: 20200,
                complete: true,
                issues: None,
            },
            SheddingRecord {
                date_epoch: 20260,
                complete: false,
                issues: Some("Retained eye caps - humidity was low".into()),
            },
            SheddingRecord {
                date_epoch: 20320,
                complete: true,
                issues: None,
            },
        ],
        parasite_history: vec!["Fecal: Snake mites treated with fipronil spray 2025-01".into()],
        husbandry_notes: vec![
            "Increase humidity to 70% during shed cycles".into(),
            "Provide two hides minimum".into(),
        ],
    };

    let encoded = encode_to_vec(&record).expect("encode exotic pet record");
    let (decoded, _): (ExoticPetRecord, _) =
        decode_from_slice(&encoded).expect("decode exotic pet record");
    assert_eq!(record, decoded);
}

// ---------------------------------------------------------------------------
// Test 16: Emergency triage with treatment timeline
// ---------------------------------------------------------------------------

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct TriageAssessment {
    arrival_epoch: u32,
    presenting_complaint: String,
    triage_color: String,
    temp_c_x10: u16,
    heart_rate: u16,
    resp_rate: u16,
    mucous_membrane_color: String,
    crt_seconds_x10: u8,
    pain_score: PainScore,
    mentation: String,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct TreatmentAction {
    time_offset_minutes: u16,
    action: String,
    performed_by: String,
    details: Option<String>,
    outcome: Option<String>,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct EmergencyCase {
    case_id: u64,
    patient_id: u64,
    triage: TriageAssessment,
    working_diagnoses: Vec<String>,
    treatment_timeline: Vec<TreatmentAction>,
    diagnostics_ordered: Vec<String>,
    disposition: String,
    estimated_cost_yen: Option<u64>,
}

#[test]
fn test_emergency_triage_timeline() {
    let case = EmergencyCase {
        case_id: 7001,
        patient_id: 100500,
        triage: TriageAssessment {
            arrival_epoch: 20315,
            presenting_complaint: "Ingested dark chocolate 2 hours ago".into(),
            triage_color: "Red - Immediate".into(),
            temp_c_x10: 395,
            heart_rate: 160,
            resp_rate: 36,
            mucous_membrane_color: "Pink, tacky".into(),
            crt_seconds_x10: 15,
            pain_score: PainScore::Mild,
            mentation: "Hyperexcitable, tremoring".into(),
        },
        working_diagnoses: vec!["Theobromine toxicosis".into(), "Caffeine toxicosis".into()],
        treatment_timeline: vec![
            TreatmentAction {
                time_offset_minutes: 0,
                action: "IV catheter placed".into(),
                performed_by: "Tech Suzuki".into(),
                details: Some("20G cephalic vein, left forelimb".into()),
                outcome: Some("Successful first attempt".into()),
            },
            TreatmentAction {
                time_offset_minutes: 5,
                action: "Apomorphine administered".into(),
                performed_by: "Dr. Honda".into(),
                details: Some("0.03 mg/kg IV".into()),
                outcome: Some("Productive emesis, chocolate material recovered".into()),
            },
            TreatmentAction {
                time_offset_minutes: 15,
                action: "Activated charcoal administered".into(),
                performed_by: "Tech Suzuki".into(),
                details: Some("2 g/kg PO via syringe".into()),
                outcome: Some("Patient accepted without aspiration".into()),
            },
            TreatmentAction {
                time_offset_minutes: 20,
                action: "IV fluid therapy initiated".into(),
                performed_by: "Tech Suzuki".into(),
                details: Some("LRS at 2x maintenance rate".into()),
                outcome: None,
            },
        ],
        diagnostics_ordered: vec![
            "Stat CBC/Chemistry".into(),
            "ECG monitoring".into(),
            "Blood pressure monitoring q30min".into(),
        ],
        disposition: "Hospitalized for 24-hour observation and IV fluids".into(),
        estimated_cost_yen: Some(180000),
    };

    let encoded = encode_to_vec(&case).expect("encode emergency case");
    let (decoded, _): (EmergencyCase, _) =
        decode_from_slice(&encoded).expect("decode emergency case");
    assert_eq!(case, decoded);
}

// ---------------------------------------------------------------------------
// Test 17: Ultrasound abdominal study
// ---------------------------------------------------------------------------

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct OrganMeasurement {
    organ: String,
    dimension: String,
    value_mm_x10: u32,
    normal_range_low: u32,
    normal_range_high: u32,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct UltrasoundOrganFinding {
    organ: String,
    echogenicity: String,
    architecture: String,
    measurements: Vec<OrganMeasurement>,
    abnormalities: Vec<String>,
    doppler_findings: Option<String>,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct AbdominalUltrasound {
    study_id: String,
    patient_id: u64,
    date_epoch: u32,
    sonographer: String,
    interpreter: String,
    prep_notes: Option<String>,
    organ_findings: Vec<UltrasoundOrganFinding>,
    free_fluid: bool,
    fluid_description: Option<String>,
    impression: String,
    recommendations: Vec<String>,
}

#[test]
fn test_ultrasound_abdominal_study() {
    let study = AbdominalUltrasound {
        study_id: "US-2025-4421".into(),
        patient_id: 200200,
        date_epoch: 20310,
        sonographer: "Dr. Morita (DACVR)".into(),
        interpreter: "Dr. Morita (DACVR)".into(),
        prep_notes: Some("12-hour fast. Patient mildly sedated with butorphanol.".into()),
        organ_findings: vec![
            UltrasoundOrganFinding {
                organ: "Liver".into(),
                echogenicity: "Mildly hyperechoic compared to falciform fat".into(),
                architecture: "Diffusely homogeneous".into(),
                measurements: vec![OrganMeasurement {
                    organ: "Liver".into(),
                    dimension: "Hepatic length".into(),
                    value_mm_x10: 520,
                    normal_range_low: 300,
                    normal_range_high: 450,
                }],
                abnormalities: vec![
                    "Hepatomegaly".into(),
                    "Diffuse hyperechogenicity suggestive of hepatic lipidosis".into(),
                ],
                doppler_findings: Some("Normal hepatic and portal venous flow".into()),
            },
            UltrasoundOrganFinding {
                organ: "Left kidney".into(),
                echogenicity: "Cortex hyperechoic".into(),
                architecture: "Preserved corticomedullary distinction".into(),
                measurements: vec![
                    OrganMeasurement {
                        organ: "Left kidney".into(),
                        dimension: "Length".into(),
                        value_mm_x10: 380,
                        normal_range_low: 300,
                        normal_range_high: 440,
                    },
                    OrganMeasurement {
                        organ: "Left kidney".into(),
                        dimension: "Cortical thickness".into(),
                        value_mm_x10: 45,
                        normal_range_low: 30,
                        normal_range_high: 60,
                    },
                ],
                abnormalities: vec![],
                doppler_findings: Some("Normal renal arterial flow, RI 0.62".into()),
            },
        ],
        free_fluid: false,
        fluid_description: None,
        impression: "Feline hepatic lipidosis. Kidneys within normal limits.".into(),
        recommendations: vec![
            "Hepatic fine needle aspirate for cytology".into(),
            "Initiate aggressive nutritional support".into(),
            "Recheck ultrasound in 4 weeks".into(),
        ],
    };

    let encoded = encode_to_vec(&study).expect("encode ultrasound study");
    let (decoded, _): (AbdominalUltrasound, _) =
        decode_from_slice(&encoded).expect("decode ultrasound study");
    assert_eq!(study, decoded);
}

// ---------------------------------------------------------------------------
// Test 18: Avian patient with clutch and breeding records
// ---------------------------------------------------------------------------

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct EggRecord {
    laid_date_epoch: u32,
    fertile: Option<bool>,
    hatched: bool,
    hatch_date_epoch: Option<u32>,
    chick_band_id: Option<String>,
    weight_grams_x10: Option<u16>,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct ClutchRecord {
    clutch_number: u8,
    start_date_epoch: u32,
    nest_box_id: String,
    eggs: Vec<EggRecord>,
    incubation_method: String,
    notes: Option<String>,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct AvianPatient {
    patient_id: u64,
    common_name: String,
    scientific_name: String,
    band_number: Option<String>,
    sex: Sex,
    weight_grams: u32,
    wing_chord_mm: Option<u16>,
    clutch_history: Vec<ClutchRecord>,
    feather_condition: String,
    behavioral_notes: Vec<String>,
}

#[test]
fn test_avian_patient_breeding_records() {
    let patient = AvianPatient {
        patient_id: 500100,
        common_name: "African Grey Parrot".into(),
        scientific_name: "Psittacus erithacus".into(),
        band_number: Some("AGP-2020-0451".into()),
        sex: Sex::Female,
        weight_grams: 420,
        wing_chord_mm: Some(245),
        clutch_history: vec![
            ClutchRecord {
                clutch_number: 1,
                start_date_epoch: 20050,
                nest_box_id: "NB-A12".into(),
                eggs: vec![
                    EggRecord {
                        laid_date_epoch: 20050,
                        fertile: Some(true),
                        hatched: true,
                        hatch_date_epoch: Some(20078),
                        chick_band_id: Some("AGP-2025-0102".into()),
                        weight_grams_x10: Some(145),
                    },
                    EggRecord {
                        laid_date_epoch: 20053,
                        fertile: Some(true),
                        hatched: true,
                        hatch_date_epoch: Some(20081),
                        chick_band_id: Some("AGP-2025-0103".into()),
                        weight_grams_x10: Some(138),
                    },
                    EggRecord {
                        laid_date_epoch: 20056,
                        fertile: Some(false),
                        hatched: false,
                        hatch_date_epoch: None,
                        chick_band_id: None,
                        weight_grams_x10: None,
                    },
                ],
                incubation_method: "Natural - parent reared".into(),
                notes: Some("Successful clutch, both chicks healthy".into()),
            },
            ClutchRecord {
                clutch_number: 2,
                start_date_epoch: 20250,
                nest_box_id: "NB-A12".into(),
                eggs: vec![EggRecord {
                    laid_date_epoch: 20250,
                    fertile: None,
                    hatched: false,
                    hatch_date_epoch: None,
                    chick_band_id: None,
                    weight_grams_x10: Some(150),
                }],
                incubation_method: "Artificial incubator".into(),
                notes: Some("Egg candled at day 10, fertility TBD".into()),
            },
        ],
        feather_condition: "Good overall, mild barbering on chest feathers".into(),
        behavioral_notes: vec![
            "Talks extensively, vocabulary ~50 words".into(),
            "Prefers female handlers".into(),
        ],
    };

    let encoded = encode_to_vec(&patient).expect("encode avian patient");
    let (decoded, _): (AvianPatient, _) =
        decode_from_slice(&encoded).expect("decode avian patient");
    assert_eq!(patient, decoded);
}

// ---------------------------------------------------------------------------
// Test 19: Rehabilitation/physiotherapy program
// ---------------------------------------------------------------------------

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct RangeOfMotion {
    joint: String,
    flexion_degrees: u16,
    extension_degrees: u16,
    normal_flexion: u16,
    normal_extension: u16,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct ExerciseProtocol {
    exercise_name: String,
    sets: u8,
    reps: u8,
    duration_seconds: Option<u16>,
    resistance_level: Option<String>,
    instructions: String,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct RehabSession {
    session_number: u16,
    date_epoch: u32,
    therapist: String,
    rom_measurements: Vec<RangeOfMotion>,
    exercises_performed: Vec<ExerciseProtocol>,
    modalities_used: Vec<String>,
    girth_measurements_mm: Vec<(String, u16)>,
    subjective_improvement: String,
    next_session_plan: Vec<String>,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct RehabProgram {
    patient_id: u64,
    diagnosis: String,
    surgery_date_epoch: Option<u32>,
    program_start_epoch: u32,
    target_goals: Vec<String>,
    sessions: Vec<RehabSession>,
    home_exercises: Vec<ExerciseProtocol>,
    weight_bearing_status: String,
}

#[test]
fn test_rehabilitation_physiotherapy_program() {
    let program = RehabProgram {
        patient_id: 100234,
        diagnosis: "Post-TPLO right stifle".into(),
        surgery_date_epoch: Some(20310),
        program_start_epoch: 20317,
        target_goals: vec![
            "Restore full range of motion".into(),
            "Rebuild quadriceps mass".into(),
            "Return to normal activity by 12 weeks".into(),
        ],
        sessions: vec![RehabSession {
            session_number: 1,
            date_epoch: 20317,
            therapist: "Rehab Tech Fujita (CCRP)".into(),
            rom_measurements: vec![RangeOfMotion {
                joint: "Right stifle".into(),
                flexion_degrees: 70,
                extension_degrees: 140,
                normal_flexion: 42,
                normal_extension: 162,
            }],
            exercises_performed: vec![
                ExerciseProtocol {
                    exercise_name: "Underwater treadmill".into(),
                    sets: 1,
                    reps: 1,
                    duration_seconds: Some(600),
                    resistance_level: Some("Water at stifle level".into()),
                    instructions: "Slow walk, 0.8 km/h".into(),
                },
                ExerciseProtocol {
                    exercise_name: "Passive range of motion".into(),
                    sets: 3,
                    reps: 15,
                    duration_seconds: None,
                    resistance_level: None,
                    instructions: "Gentle flexion/extension of stifle".into(),
                },
            ],
            modalities_used: vec![
                "Therapeutic laser Class IV, 8 J/cm2".into(),
                "Cryotherapy 10 min post-session".into(),
            ],
            girth_measurements_mm: vec![
                ("Right thigh (mid)".into(), 320),
                ("Left thigh (mid)".into(), 365),
            ],
            subjective_improvement: "Bearing more weight on right hind today".into(),
            next_session_plan: vec![
                "Increase UWTM to 12 minutes".into(),
                "Add cavaletti rails if comfortable".into(),
            ],
        }],
        home_exercises: vec![
            ExerciseProtocol {
                exercise_name: "Leash walk".into(),
                sets: 3,
                reps: 1,
                duration_seconds: Some(300),
                resistance_level: None,
                instructions: "Slow controlled walk on flat surface, 3x daily".into(),
            },
            ExerciseProtocol {
                exercise_name: "Sit-to-stand".into(),
                sets: 2,
                reps: 10,
                duration_seconds: None,
                resistance_level: None,
                instructions: "On non-slip surface, ensure symmetrical sit".into(),
            },
        ],
        weight_bearing_status: "Partial weight bearing, toe-touching at rest".into(),
    };

    let encoded = encode_to_vec(&program).expect("encode rehab program");
    let (decoded, _): (RehabProgram, _) =
        decode_from_slice(&encoded).expect("decode rehab program");
    assert_eq!(program, decoded);
}

// ---------------------------------------------------------------------------
// Test 20: Parasite screening and prevention protocol
// ---------------------------------------------------------------------------

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct FecalResult {
    date_epoch: u32,
    method: String,
    parasites_found: Vec<ParasiteIdentification>,
    eggs_per_gram: Option<u32>,
    lab_name: String,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct ParasiteIdentification {
    organism: String,
    life_stage: String,
    quantity: String,
    zoonotic_risk: bool,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct PreventiveProduct {
    product_name: String,
    active_ingredient: String,
    spectrum: Vec<String>,
    dose_for_weight_range: String,
    administration_route: VaccineRoute,
    frequency: Frequency,
    last_given_epoch: Option<u32>,
    next_due_epoch: Option<u32>,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct ParasiteScreeningRecord {
    patient_id: u64,
    heartworm_status: String,
    last_heartworm_test_epoch: Option<u32>,
    fecal_results: Vec<FecalResult>,
    preventives: Vec<PreventiveProduct>,
    environmental_risks: Vec<String>,
}

#[test]
fn test_parasite_screening_prevention() {
    let record = ParasiteScreeningRecord {
        patient_id: 100234,
        heartworm_status: "Negative".into(),
        last_heartworm_test_epoch: Some(20300),
        fecal_results: vec![FecalResult {
            date_epoch: 20300,
            method: "Centrifugal flotation with zinc sulfate".into(),
            parasites_found: vec![ParasiteIdentification {
                organism: "Giardia spp.".into(),
                life_stage: "Cysts".into(),
                quantity: "Moderate".into(),
                zoonotic_risk: true,
            }],
            eggs_per_gram: None,
            lab_name: "IDEXX Reference Lab".into(),
        }],
        preventives: vec![
            PreventiveProduct {
                product_name: "NexGard Spectra".into(),
                active_ingredient: "Afoxolaner + Milbemycin oxime".into(),
                spectrum: vec![
                    "Fleas".into(),
                    "Ticks".into(),
                    "Heartworm".into(),
                    "Roundworm".into(),
                    "Hookworm".into(),
                    "Whipworm".into(),
                ],
                dose_for_weight_range: "25.1-50 kg".into(),
                administration_route: VaccineRoute::Oral,
                frequency: Frequency::OnceDaily,
                last_given_epoch: Some(20290),
                next_due_epoch: Some(20320),
            },
            PreventiveProduct {
                product_name: "Fenbendazole".into(),
                active_ingredient: "Fenbendazole".into(),
                spectrum: vec![
                    "Giardia".into(),
                    "Roundworm".into(),
                    "Hookworm".into(),
                    "Whipworm".into(),
                    "Tapeworm (Taenia)".into(),
                ],
                dose_for_weight_range: "50 mg/kg".into(),
                administration_route: VaccineRoute::Oral,
                frequency: Frequency::OnceDaily,
                last_given_epoch: Some(20300),
                next_due_epoch: Some(20305),
            },
        ],
        environmental_risks: vec![
            "Dog park visits weekly".into(),
            "Endemic heartworm area".into(),
        ],
    };

    let encoded = encode_to_vec(&record).expect("encode parasite record");
    let (decoded, _): (ParasiteScreeningRecord, _) =
        decode_from_slice(&encoded).expect("decode parasite record");
    assert_eq!(record, decoded);
}

// ---------------------------------------------------------------------------
// Test 21: Multi-species clinic appointment schedule
// ---------------------------------------------------------------------------

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct AppointmentSlot {
    slot_epoch: u32,
    duration_minutes: u16,
    veterinarian: String,
    room: String,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct AppointmentPatient {
    patient_id: u64,
    patient_name: String,
    species: Species,
    owner_name: String,
    phone: String,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct AppointmentReason {
    primary_reason: String,
    secondary_reasons: Vec<String>,
    requires_sedation: bool,
    requires_fasting: bool,
    special_handling: Vec<String>,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct Appointment {
    appointment_id: u64,
    slot: AppointmentSlot,
    patient: AppointmentPatient,
    reason: AppointmentReason,
    confirmed: bool,
    checked_in: bool,
    no_show: bool,
    notes: Option<String>,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct DailySchedule {
    date_epoch: u32,
    clinic_name: String,
    appointments: Vec<Appointment>,
    blocked_slots: Vec<(u32, String)>,
    on_call_vet: String,
}

#[test]
fn test_multi_species_clinic_schedule() {
    let schedule = DailySchedule {
        date_epoch: 20320,
        clinic_name: "Sakura Animal Hospital".into(),
        appointments: vec![
            Appointment {
                appointment_id: 10001,
                slot: AppointmentSlot {
                    slot_epoch: 20320,
                    duration_minutes: 30,
                    veterinarian: "Dr. Sato".into(),
                    room: "Exam 1".into(),
                },
                patient: AppointmentPatient {
                    patient_id: 100234,
                    patient_name: "Bella".into(),
                    species: Species::Canine,
                    owner_name: "Tanaka Yuki".into(),
                    phone: "090-1234-5678".into(),
                },
                reason: AppointmentReason {
                    primary_reason: "Post-surgical TPLO recheck".into(),
                    secondary_reasons: vec!["Weight check".into()],
                    requires_sedation: false,
                    requires_fasting: false,
                    special_handling: vec!["Allergy to amoxicillin".into()],
                },
                confirmed: true,
                checked_in: false,
                no_show: false,
                notes: Some("Bring recent radiographs from referral".into()),
            },
            Appointment {
                appointment_id: 10002,
                slot: AppointmentSlot {
                    slot_epoch: 20320,
                    duration_minutes: 20,
                    veterinarian: "Dr. Kobayashi".into(),
                    room: "Exam 2".into(),
                },
                patient: AppointmentPatient {
                    patient_id: 200200,
                    patient_name: "Mochi".into(),
                    species: Species::Feline,
                    owner_name: "Suzuki Aiko".into(),
                    phone: "090-9876-5432".into(),
                },
                reason: AppointmentReason {
                    primary_reason: "Renal recheck + blood pressure".into(),
                    secondary_reasons: vec!["Thyroid level check".into()],
                    requires_sedation: false,
                    requires_fasting: true,
                    special_handling: vec!["Fractious - feliway in room 15 min prior".into()],
                },
                confirmed: true,
                checked_in: false,
                no_show: false,
                notes: None,
            },
            Appointment {
                appointment_id: 10003,
                slot: AppointmentSlot {
                    slot_epoch: 20320,
                    duration_minutes: 45,
                    veterinarian: "Dr. Sato".into(),
                    room: "Exotic Suite".into(),
                },
                patient: AppointmentPatient {
                    patient_id: 400100,
                    patient_name: "Nagini".into(),
                    species: Species::Reptile,
                    owner_name: "Yamada Ken".into(),
                    phone: "080-5555-1234".into(),
                },
                reason: AppointmentReason {
                    primary_reason: "Annual wellness exam".into(),
                    secondary_reasons: vec![
                        "Fecal parasite screen".into(),
                        "Husbandry review".into(),
                    ],
                    requires_sedation: false,
                    requires_fasting: false,
                    special_handling: vec![
                        "Handle with gloves - defensive striker".into(),
                        "Maintain warm room temperature".into(),
                    ],
                },
                confirmed: false,
                checked_in: false,
                no_show: false,
                notes: Some("Owner bringing enclosure photos for husbandry review".into()),
            },
        ],
        blocked_slots: vec![
            (20320, "Lunch break 12:00-13:00".into()),
            (20320, "Staff meeting 17:00-17:30".into()),
        ],
        on_call_vet: "Dr. Honda".into(),
    };

    let encoded = encode_to_vec(&schedule).expect("encode daily schedule");
    let (decoded, _): (DailySchedule, _) =
        decode_from_slice(&encoded).expect("decode daily schedule");
    assert_eq!(schedule, decoded);
}

// ---------------------------------------------------------------------------
// Test 22: Clinical trial enrollment with multi-visit protocol
// ---------------------------------------------------------------------------

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct InclusionCriteria {
    criterion: String,
    met: bool,
    verification_method: String,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct AdverseEvent {
    date_epoch: u32,
    description: String,
    severity: String,
    related_to_treatment: String,
    action_taken: String,
    resolved: bool,
    resolution_epoch: Option<u32>,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct TrialVisit {
    visit_number: u8,
    scheduled_epoch: u32,
    actual_epoch: Option<u32>,
    procedures: Vec<String>,
    measurements: Vec<(String, String)>,
    treatment_administered: Option<String>,
    adverse_events: Vec<AdverseEvent>,
    investigator_notes: Option<String>,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct TrialProtocol {
    protocol_id: String,
    title: String,
    sponsor: String,
    phase: String,
    investigational_product: String,
    control_product: Option<String>,
    total_visits: u8,
    duration_weeks: u16,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct ClinicalTrialEnrollment {
    enrollment_id: String,
    patient_id: u64,
    protocol: TrialProtocol,
    enrollment_date_epoch: u32,
    randomization_group: String,
    blinded: bool,
    inclusion_criteria: Vec<InclusionCriteria>,
    consent_obtained: bool,
    consent_date_epoch: u32,
    visits: Vec<TrialVisit>,
    withdrawn: bool,
    withdrawal_reason: Option<String>,
}

#[test]
fn test_clinical_trial_enrollment_multi_visit() {
    let enrollment = ClinicalTrialEnrollment {
        enrollment_id: "TRIAL-OA-2025-0088".into(),
        patient_id: 100234,
        protocol: TrialProtocol {
            protocol_id: "PROTO-OA-001".into(),
            title: "Efficacy of Novel Anti-NGF Monoclonal Antibody for Canine Osteoarthritis"
                .into(),
            sponsor: "VetPharma Research Inc.".into(),
            phase: "Pivotal field study".into(),
            investigational_product: "caninumab (anti-NGF mAb)".into(),
            control_product: Some("Placebo (saline)".into()),
            total_visits: 6,
            duration_weeks: 24,
        },
        enrollment_date_epoch: 20300,
        randomization_group: "Treatment".into(),
        blinded: true,
        inclusion_criteria: vec![
            InclusionCriteria {
                criterion: "Radiographic evidence of OA in at least one joint".into(),
                met: true,
                verification_method: "Radiographs reviewed by DACVR".into(),
            },
            InclusionCriteria {
                criterion: "CBPI pain score >= 3".into(),
                met: true,
                verification_method: "Owner questionnaire".into(),
            },
            InclusionCriteria {
                criterion: "No NSAIDs for 14 days prior to enrollment".into(),
                met: true,
                verification_method: "Owner report and medical record review".into(),
            },
            InclusionCriteria {
                criterion: "Body weight 10-50 kg".into(),
                met: true,
                verification_method: "Clinic scale, 29.1 kg".into(),
            },
        ],
        consent_obtained: true,
        consent_date_epoch: 20298,
        visits: vec![
            TrialVisit {
                visit_number: 1,
                scheduled_epoch: 20300,
                actual_epoch: Some(20300),
                procedures: vec![
                    "Physical exam".into(),
                    "CBPI owner assessment".into(),
                    "Gait analysis (force plate)".into(),
                    "Blood draw (CBC, chemistry, urinalysis)".into(),
                ],
                measurements: vec![
                    ("CBPI pain score".into(), "5.2".into()),
                    ("Peak vertical force (N/kg)".into(), "4.8".into()),
                    ("Weight (kg)".into(), "29.1".into()),
                ],
                treatment_administered: Some("Study drug SC injection, 0.5 mg/kg".into()),
                adverse_events: vec![],
                investigator_notes: Some("Baseline visit. Patient cooperative.".into()),
            },
            TrialVisit {
                visit_number: 2,
                scheduled_epoch: 20328,
                actual_epoch: Some(20329),
                procedures: vec![
                    "Physical exam".into(),
                    "CBPI owner assessment".into(),
                    "Gait analysis (force plate)".into(),
                ],
                measurements: vec![
                    ("CBPI pain score".into(), "3.8".into()),
                    ("Peak vertical force (N/kg)".into(), "5.6".into()),
                    ("Weight (kg)".into(), "29.3".into()),
                ],
                treatment_administered: None,
                adverse_events: vec![AdverseEvent {
                    date_epoch: 20310,
                    description: "Mild injection site swelling".into(),
                    severity: "Mild".into(),
                    related_to_treatment: "Possibly related".into(),
                    action_taken: "Cold compress applied by owner".into(),
                    resolved: true,
                    resolution_epoch: Some(20313),
                }],
                investigator_notes: Some(
                    "Improvement in pain scores and gait. Owner reports increased activity.".into(),
                ),
            },
        ],
        withdrawn: false,
        withdrawal_reason: None,
    };

    let encoded = encode_to_vec(&enrollment).expect("encode trial enrollment");
    let (decoded, _): (ClinicalTrialEnrollment, _) =
        decode_from_slice(&encoded).expect("decode trial enrollment");
    assert_eq!(enrollment, decoded);
}
