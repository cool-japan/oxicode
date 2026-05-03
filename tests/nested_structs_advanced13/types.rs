// Shared domain types for nested_structs_advanced13 tests
use oxicode::{Decode, Encode};

// ---------------------------------------------------------------------------
// Domain enums
// ---------------------------------------------------------------------------

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
pub enum Species {
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
pub enum Sex {
    Male,
    Female,
    MaleNeutered,
    FemaleSpayed,
    Unknown,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
pub enum VaccinationType {
    Core,
    NonCore,
    RiskBased,
    Required,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
pub enum VaccineRoute {
    Subcutaneous,
    Intramuscular,
    Intranasal,
    Oral,
    Transdermal,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
pub enum AnesthesiaStage {
    Preinduction,
    Induction,
    Maintenance,
    Recovery,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
pub enum PainScore {
    None,
    Mild,
    Moderate,
    Severe,
    Excruciating,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
pub enum ToothCondition {
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
pub enum DosageUnit {
    MgPerKg,
    MlPerKg,
    UnitsPerKg,
    Mg,
    Ml,
    Drops,
    Tablets,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
pub enum Frequency {
    OnceDaily,
    TwiceDaily,
    ThreeTimesDaily,
    EveryOtherDay,
    Weekly,
    AsNeeded,
    SingleDose,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
pub enum LabFlag {
    Normal,
    Low,
    High,
    Critical,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
pub enum Laterality {
    Left,
    Right,
    Bilateral,
    NotApplicable,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
pub enum ImagingModality {
    Radiograph,
    Ultrasound,
    CT,
    MRI,
    Fluoroscopy,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
pub enum BoardingSize {
    Small,
    Medium,
    Large,
    ExtraLarge,
    Pasture,
    Stall,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
pub enum HerdTestType {
    Tuberculin,
    Brucellosis,
    Johnes,
    BVD,
    Lepto,
    IBR,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
pub enum SurgeryCategory {
    SoftTissue,
    Orthopedic,
    Ophthalmic,
    Dental,
    Neurologic,
    Oncologic,
    Emergency,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
pub enum NutrientCategory {
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
pub struct WeightEntry {
    pub date_epoch_days: u32,
    pub weight_grams: u64,
    pub body_condition_score: u8,
    pub notes: Option<String>,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
pub struct BreedInfo {
    pub primary_breed: String,
    pub secondary_breed: Option<String>,
    pub breed_percentage: Option<u8>,
    pub genetic_test_id: Option<String>,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
pub struct OwnerContact {
    pub name: String,
    pub phone_primary: String,
    pub phone_emergency: Option<String>,
    pub email: Option<String>,
    pub address_lines: Vec<String>,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
pub struct PatientRecord {
    pub patient_id: u64,
    pub name: String,
    pub species: Species,
    pub sex: Sex,
    pub breed: BreedInfo,
    pub date_of_birth_epoch: Option<u32>,
    pub microchip_id: Option<String>,
    pub weight_history: Vec<WeightEntry>,
    pub owners: Vec<OwnerContact>,
    pub allergies: Vec<String>,
    pub chronic_conditions: Vec<String>,
}

// ---------------------------------------------------------------------------
// Test 2: Vaccination schedule with booster tracking
// ---------------------------------------------------------------------------

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
pub struct VaccineLot {
    pub manufacturer: String,
    pub lot_number: String,
    pub expiration_epoch: u32,
    pub storage_temp_c_x10: i16,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
pub struct VaccineAdministration {
    pub date_epoch: u32,
    pub vaccine_name: String,
    pub lot: VaccineLot,
    pub route: VaccineRoute,
    pub site: String,
    pub administered_by: String,
    pub reaction_observed: Option<String>,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
pub struct BoosterSchedule {
    pub next_due_epoch: u32,
    pub interval_days: u32,
    pub is_overdue: bool,
    pub reminder_sent: bool,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
pub struct VaccinationSeries {
    pub vaccine_type: VaccinationType,
    pub disease_target: String,
    pub administrations: Vec<VaccineAdministration>,
    pub booster: Option<BoosterSchedule>,
    pub series_complete: bool,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
pub struct VaccinationRecord {
    pub patient_id: u64,
    pub series_list: Vec<VaccinationSeries>,
    pub exemptions: Vec<String>,
    pub titer_tests: Vec<TiterResult>,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
pub struct TiterResult {
    pub disease: String,
    pub date_epoch: u32,
    pub result_value: u32,
    pub adequate: bool,
    pub lab_name: String,
}

// ---------------------------------------------------------------------------
// Test 3: CBC lab panel with reference ranges
// ---------------------------------------------------------------------------

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
pub struct ReferenceRange {
    pub low_x100: i64,
    pub high_x100: i64,
    pub unit: String,
    pub species_specific: bool,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
pub struct LabAnalyte {
    pub name: String,
    pub code: String,
    pub value_x100: i64,
    pub reference: ReferenceRange,
    pub flag: LabFlag,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
pub struct CbcPanel {
    pub analytes: Vec<LabAnalyte>,
    pub morphology_notes: Option<String>,
    pub platelet_estimate: Option<String>,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
pub struct LabSubmission {
    pub accession_number: String,
    pub patient_id: u64,
    pub collected_epoch: u32,
    pub received_epoch: u32,
    pub reported_epoch: Option<u32>,
    pub collected_by: String,
    pub specimen_type: String,
    pub cbc: Option<CbcPanel>,
}

// ---------------------------------------------------------------------------
// Test 4: Chemistry panel with organ-grouped results
// ---------------------------------------------------------------------------

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
pub struct OrganPanel {
    pub organ_system: String,
    pub analytes: Vec<LabAnalyte>,
    pub clinical_significance: Option<String>,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
pub struct ChemistryReport {
    pub accession: String,
    pub fasting: bool,
    pub hemolysis_index: u8,
    pub lipemia_index: u8,
    pub panels: Vec<OrganPanel>,
    pub pathologist_comment: Option<String>,
}

// ---------------------------------------------------------------------------
// Test 5: Urinalysis with sediment findings
// ---------------------------------------------------------------------------

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
pub struct UrinalysisDipstick {
    pub ph_x10: u8,
    pub specific_gravity_x1000: u16,
    pub protein: u8,
    pub glucose: u8,
    pub ketones: u8,
    pub bilirubin: u8,
    pub blood: u8,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
pub struct SedimentFinding {
    pub element: String,
    pub quantity_per_hpf: String,
    pub significance: Option<String>,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
pub struct UrinalysisReport {
    pub collection_method: String,
    pub color: String,
    pub clarity: String,
    pub dipstick: UrinalysisDipstick,
    pub sediment: Vec<SedimentFinding>,
    pub culture_submitted: bool,
}

// ---------------------------------------------------------------------------
// Test 6: Surgical procedure log with anesthesia monitoring
// ---------------------------------------------------------------------------

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
pub struct VitalReading {
    pub elapsed_minutes: u16,
    pub heart_rate_bpm: u16,
    pub resp_rate: u16,
    pub spo2_percent: u8,
    pub etco2_mmhg: Option<u8>,
    pub systolic_bp: Option<u16>,
    pub diastolic_bp: Option<u16>,
    pub temp_c_x10: u16,
    pub anesthesia_stage: AnesthesiaStage,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
pub struct AnesthesiaDrug {
    pub drug_name: String,
    pub dose_mg_per_kg_x100: u32,
    pub route: String,
    pub time_elapsed_min: u16,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
pub struct AnesthesiaLog {
    pub protocol: String,
    pub induction_agent: String,
    pub maintenance_agent: String,
    pub drugs: Vec<AnesthesiaDrug>,
    pub vitals: Vec<VitalReading>,
    pub complications: Vec<String>,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
pub struct SurgicalProcedure {
    pub procedure_id: u64,
    pub patient_id: u64,
    pub date_epoch: u32,
    pub category: SurgeryCategory,
    pub procedure_name: String,
    pub surgeon: String,
    pub assistant: Option<String>,
    pub duration_minutes: u16,
    pub anesthesia: AnesthesiaLog,
    pub findings: String,
    pub complications: Vec<String>,
    pub post_op_pain_score: PainScore,
}

// ---------------------------------------------------------------------------
// Test 7: Prescription records with dosing regimens
// ---------------------------------------------------------------------------

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
pub struct DoseInstruction {
    pub amount_x100: u32,
    pub unit: DosageUnit,
    pub frequency: Frequency,
    pub with_food: bool,
    pub special_instructions: Option<String>,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
pub struct RefillInfo {
    pub refills_authorized: u8,
    pub refills_used: u8,
    pub last_refill_epoch: Option<u32>,
    pub pharmacy: Option<String>,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
pub struct Prescription {
    pub rx_number: u64,
    pub drug_name: String,
    pub strength: String,
    pub dose: DoseInstruction,
    pub duration_days: Option<u16>,
    pub start_epoch: u32,
    pub prescriber: String,
    pub refill: RefillInfo,
    pub warnings: Vec<String>,
    pub drug_interactions: Vec<String>,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
pub struct PrescriptionHistory {
    pub patient_id: u64,
    pub active_prescriptions: Vec<Prescription>,
    pub past_prescriptions: Vec<Prescription>,
}

// ---------------------------------------------------------------------------
// Test 8: Kennel boarding reservation
// ---------------------------------------------------------------------------

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
pub struct FeedingSchedule {
    pub time_of_day: String,
    pub food_type: String,
    pub amount: String,
    pub special_prep: Option<String>,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
pub struct MedicationDuringBoarding {
    pub drug_name: String,
    pub dose_instructions: String,
    pub frequency: Frequency,
    pub supplied_by_owner: bool,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
pub struct BoardingReservation {
    pub reservation_id: u64,
    pub patient_id: u64,
    pub check_in_epoch: u32,
    pub check_out_epoch: u32,
    pub kennel_size: BoardingSize,
    pub kennel_number: Option<String>,
    pub feeding: Vec<FeedingSchedule>,
    pub medications: Vec<MedicationDuringBoarding>,
    pub exercise_notes: Option<String>,
    pub behavioral_notes: Option<String>,
    pub emergency_contact: OwnerContact,
    pub vaccination_verified: bool,
    pub belongings: Vec<String>,
}

// ---------------------------------------------------------------------------
// Test 9: Livestock herd health program
// ---------------------------------------------------------------------------

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
pub struct HerdAnimal {
    pub ear_tag: String,
    pub rfid: Option<String>,
    pub sex: Sex,
    pub birth_date_epoch: Option<u32>,
    pub dam_tag: Option<String>,
    pub sire_tag: Option<String>,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
pub struct HerdTest {
    pub test_type: HerdTestType,
    pub date_epoch: u32,
    pub animals_tested: u32,
    pub positives: u32,
    pub inconclusive: u32,
    pub lab_report_id: Option<String>,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
pub struct TreatmentProtocol {
    pub condition: String,
    pub drug: String,
    pub withdrawal_days_meat: u16,
    pub withdrawal_days_milk: u16,
    pub dosage_instructions: String,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
pub struct HerdHealthProgram {
    pub herd_id: String,
    pub species: Species,
    pub total_head: u32,
    pub location: String,
    pub animals: Vec<HerdAnimal>,
    pub test_history: Vec<HerdTest>,
    pub protocols: Vec<TreatmentProtocol>,
    pub next_vet_visit_epoch: Option<u32>,
    pub certifications: Vec<String>,
}

// ---------------------------------------------------------------------------
// Test 10: Radiology study hierarchy (study → series → images)
// ---------------------------------------------------------------------------

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
pub struct RadiologyImage {
    pub image_uid: String,
    pub instance_number: u32,
    pub laterality: Laterality,
    pub position: String,
    pub kvp: u16,
    pub mas: u16,
    pub exposure_time_ms: u32,
    pub file_size_bytes: u64,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
pub struct RadiologySeries {
    pub series_uid: String,
    pub series_number: u32,
    pub modality: ImagingModality,
    pub body_part: String,
    pub description: String,
    pub images: Vec<RadiologyImage>,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
pub struct RadiologyFinding {
    pub location: String,
    pub description: String,
    pub severity: String,
    pub differential_diagnoses: Vec<String>,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
pub struct RadiologyStudy {
    pub study_uid: String,
    pub patient_id: u64,
    pub date_epoch: u32,
    pub referring_vet: String,
    pub radiologist: Option<String>,
    pub clinical_indication: String,
    pub series: Vec<RadiologySeries>,
    pub findings: Vec<RadiologyFinding>,
    pub impression: Option<String>,
    pub recommendations: Vec<String>,
}

// ---------------------------------------------------------------------------
// Test 11: Dental charting
// ---------------------------------------------------------------------------

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
pub struct ToothRecord {
    pub tooth_number: u16,
    pub quadrant: u8,
    pub condition: ToothCondition,
    pub pocket_depths_mm: Vec<u8>,
    pub mobility_grade: u8,
    pub notes: Option<String>,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
pub struct DentalProcedureEntry {
    pub tooth_number: u16,
    pub procedure: String,
    pub material_used: Option<String>,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
pub struct DentalChart {
    pub patient_id: u64,
    pub date_epoch: u32,
    pub veterinarian: String,
    pub dental_grade: u8,
    pub teeth: Vec<ToothRecord>,
    pub procedures_performed: Vec<DentalProcedureEntry>,
    pub scaling_performed: bool,
    pub polishing_performed: bool,
    pub fluoride_applied: bool,
    pub dental_radiographs_taken: u8,
    pub home_care_recommendations: Vec<String>,
}

// ---------------------------------------------------------------------------
// Test 12: Feline patient with multi-disease monitoring
// ---------------------------------------------------------------------------

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
pub struct BloodPressureReading {
    pub date_epoch: u32,
    pub systolic: u16,
    pub diastolic: u16,
    pub method: String,
    pub limb_used: String,
    pub readings_averaged: u8,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
pub struct ThyroidMonitoring {
    pub t4_values: Vec<(u32, u32)>,
    pub medication: Option<String>,
    pub current_dose_mg_x100: Option<u32>,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
pub struct RenalMonitoring {
    pub sdma_values: Vec<(u32, u32)>,
    pub iris_stage: u8,
    pub on_renal_diet: bool,
    pub sub_q_fluids: bool,
    pub fluid_volume_ml: Option<u16>,
    pub fluid_frequency: Option<Frequency>,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
pub struct FelineMultiDiseaseProfile {
    pub patient_id: u64,
    pub name: String,
    pub thyroid: ThyroidMonitoring,
    pub renal: RenalMonitoring,
    pub blood_pressure_history: Vec<BloodPressureReading>,
    pub current_medications: Vec<String>,
}

// ---------------------------------------------------------------------------
// Test 13: Equine lameness evaluation
// ---------------------------------------------------------------------------

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
pub struct FlexionTest {
    pub joint: String,
    pub duration_seconds: u8,
    pub lameness_before: u8,
    pub lameness_after: u8,
    pub limb: Laterality,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
pub struct NerveBlock {
    pub block_name: String,
    pub agent: String,
    pub volume_ml_x10: u16,
    pub response: String,
    pub improved_percent: u8,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
pub struct LamenessExam {
    pub patient_id: u64,
    pub date_epoch: u32,
    pub examiner: String,
    pub primary_complaint: String,
    pub grade_aaep: u8,
    pub affected_limb: Laterality,
    pub gait_observations: Vec<String>,
    pub flexion_tests: Vec<FlexionTest>,
    pub nerve_blocks: Vec<NerveBlock>,
    pub imaging_recommended: Vec<ImagingModality>,
    pub diagnosis: Option<String>,
    pub treatment_plan: Vec<String>,
}

// ---------------------------------------------------------------------------
// Test 14: Nutrition plan with nutrient analysis
// ---------------------------------------------------------------------------

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
pub struct NutrientContent {
    pub category: NutrientCategory,
    pub name: String,
    pub amount_per_kg_x100: u64,
    pub unit: String,
    pub meets_aafco: bool,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
pub struct DietComponent {
    pub food_name: String,
    pub manufacturer: String,
    pub daily_amount: String,
    pub calories_per_serving: u32,
    pub nutrients: Vec<NutrientContent>,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
pub struct NutritionPlan {
    pub patient_id: u64,
    pub target_weight_grams: u64,
    pub daily_calorie_target: u32,
    pub rer_calories: u32,
    pub activity_factor_x100: u16,
    pub diet_components: Vec<DietComponent>,
    pub supplements: Vec<String>,
    pub feeding_guidelines: Vec<String>,
    pub review_date_epoch: u32,
}

// ---------------------------------------------------------------------------
// Test 15: Exotic pet (reptile) husbandry record
// ---------------------------------------------------------------------------

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
pub struct EnclosureParams {
    pub length_cm: u16,
    pub width_cm: u16,
    pub height_cm: u16,
    pub substrate: String,
    pub basking_temp_c_x10: u16,
    pub cool_side_temp_c_x10: u16,
    pub humidity_percent: u8,
    pub uv_index: u8,
    pub light_cycle_hours: u8,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
pub struct SheddingRecord {
    pub date_epoch: u32,
    pub complete: bool,
    pub issues: Option<String>,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
pub struct ExoticPetRecord {
    pub patient_id: u64,
    pub common_name: String,
    pub scientific_name: String,
    pub species: Species,
    pub sex: Sex,
    pub length_cm: Option<u16>,
    pub weight_grams: u64,
    pub enclosure: EnclosureParams,
    pub diet_items: Vec<String>,
    pub supplement_schedule: Vec<String>,
    pub shedding_history: Vec<SheddingRecord>,
    pub parasite_history: Vec<String>,
    pub husbandry_notes: Vec<String>,
}

// ---------------------------------------------------------------------------
// Test 16: Emergency triage with treatment timeline
// ---------------------------------------------------------------------------

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
pub struct TriageAssessment {
    pub arrival_epoch: u32,
    pub presenting_complaint: String,
    pub triage_color: String,
    pub temp_c_x10: u16,
    pub heart_rate: u16,
    pub resp_rate: u16,
    pub mucous_membrane_color: String,
    pub crt_seconds_x10: u8,
    pub pain_score: PainScore,
    pub mentation: String,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
pub struct TreatmentAction {
    pub time_offset_minutes: u16,
    pub action: String,
    pub performed_by: String,
    pub details: Option<String>,
    pub outcome: Option<String>,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
pub struct EmergencyCase {
    pub case_id: u64,
    pub patient_id: u64,
    pub triage: TriageAssessment,
    pub working_diagnoses: Vec<String>,
    pub treatment_timeline: Vec<TreatmentAction>,
    pub diagnostics_ordered: Vec<String>,
    pub disposition: String,
    pub estimated_cost_yen: Option<u64>,
}

// ---------------------------------------------------------------------------
// Test 17: Ultrasound abdominal study
// ---------------------------------------------------------------------------

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
pub struct OrganMeasurement {
    pub organ: String,
    pub dimension: String,
    pub value_mm_x10: u32,
    pub normal_range_low: u32,
    pub normal_range_high: u32,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
pub struct UltrasoundOrganFinding {
    pub organ: String,
    pub echogenicity: String,
    pub architecture: String,
    pub measurements: Vec<OrganMeasurement>,
    pub abnormalities: Vec<String>,
    pub doppler_findings: Option<String>,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
pub struct AbdominalUltrasound {
    pub study_id: String,
    pub patient_id: u64,
    pub date_epoch: u32,
    pub sonographer: String,
    pub interpreter: String,
    pub prep_notes: Option<String>,
    pub organ_findings: Vec<UltrasoundOrganFinding>,
    pub free_fluid: bool,
    pub fluid_description: Option<String>,
    pub impression: String,
    pub recommendations: Vec<String>,
}

// ---------------------------------------------------------------------------
// Test 18: Avian patient with clutch and breeding records
// ---------------------------------------------------------------------------

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
pub struct EggRecord {
    pub laid_date_epoch: u32,
    pub fertile: Option<bool>,
    pub hatched: bool,
    pub hatch_date_epoch: Option<u32>,
    pub chick_band_id: Option<String>,
    pub weight_grams_x10: Option<u16>,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
pub struct ClutchRecord {
    pub clutch_number: u8,
    pub start_date_epoch: u32,
    pub nest_box_id: String,
    pub eggs: Vec<EggRecord>,
    pub incubation_method: String,
    pub notes: Option<String>,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
pub struct AvianPatient {
    pub patient_id: u64,
    pub common_name: String,
    pub scientific_name: String,
    pub band_number: Option<String>,
    pub sex: Sex,
    pub weight_grams: u32,
    pub wing_chord_mm: Option<u16>,
    pub clutch_history: Vec<ClutchRecord>,
    pub feather_condition: String,
    pub behavioral_notes: Vec<String>,
}

// ---------------------------------------------------------------------------
// Test 19: Rehabilitation/physiotherapy program
// ---------------------------------------------------------------------------

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
pub struct RangeOfMotion {
    pub joint: String,
    pub flexion_degrees: u16,
    pub extension_degrees: u16,
    pub normal_flexion: u16,
    pub normal_extension: u16,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
pub struct ExerciseProtocol {
    pub exercise_name: String,
    pub sets: u8,
    pub reps: u8,
    pub duration_seconds: Option<u16>,
    pub resistance_level: Option<String>,
    pub instructions: String,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
pub struct RehabSession {
    pub session_number: u16,
    pub date_epoch: u32,
    pub therapist: String,
    pub rom_measurements: Vec<RangeOfMotion>,
    pub exercises_performed: Vec<ExerciseProtocol>,
    pub modalities_used: Vec<String>,
    pub girth_measurements_mm: Vec<(String, u16)>,
    pub subjective_improvement: String,
    pub next_session_plan: Vec<String>,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
pub struct RehabProgram {
    pub patient_id: u64,
    pub diagnosis: String,
    pub surgery_date_epoch: Option<u32>,
    pub program_start_epoch: u32,
    pub target_goals: Vec<String>,
    pub sessions: Vec<RehabSession>,
    pub home_exercises: Vec<ExerciseProtocol>,
    pub weight_bearing_status: String,
}

// ---------------------------------------------------------------------------
// Test 20: Parasite screening and prevention protocol
// ---------------------------------------------------------------------------

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
pub struct FecalResult {
    pub date_epoch: u32,
    pub method: String,
    pub parasites_found: Vec<ParasiteIdentification>,
    pub eggs_per_gram: Option<u32>,
    pub lab_name: String,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
pub struct ParasiteIdentification {
    pub organism: String,
    pub life_stage: String,
    pub quantity: String,
    pub zoonotic_risk: bool,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
pub struct PreventiveProduct {
    pub product_name: String,
    pub active_ingredient: String,
    pub spectrum: Vec<String>,
    pub dose_for_weight_range: String,
    pub administration_route: VaccineRoute,
    pub frequency: Frequency,
    pub last_given_epoch: Option<u32>,
    pub next_due_epoch: Option<u32>,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
pub struct ParasiteScreeningRecord {
    pub patient_id: u64,
    pub heartworm_status: String,
    pub last_heartworm_test_epoch: Option<u32>,
    pub fecal_results: Vec<FecalResult>,
    pub preventives: Vec<PreventiveProduct>,
    pub environmental_risks: Vec<String>,
}

// ---------------------------------------------------------------------------
// Test 21: Multi-species clinic appointment schedule
// ---------------------------------------------------------------------------

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
pub struct AppointmentSlot {
    pub slot_epoch: u32,
    pub duration_minutes: u16,
    pub veterinarian: String,
    pub room: String,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
pub struct AppointmentPatient {
    pub patient_id: u64,
    pub patient_name: String,
    pub species: Species,
    pub owner_name: String,
    pub phone: String,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
pub struct AppointmentReason {
    pub primary_reason: String,
    pub secondary_reasons: Vec<String>,
    pub requires_sedation: bool,
    pub requires_fasting: bool,
    pub special_handling: Vec<String>,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
pub struct Appointment {
    pub appointment_id: u64,
    pub slot: AppointmentSlot,
    pub patient: AppointmentPatient,
    pub reason: AppointmentReason,
    pub confirmed: bool,
    pub checked_in: bool,
    pub no_show: bool,
    pub notes: Option<String>,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
pub struct DailySchedule {
    pub date_epoch: u32,
    pub clinic_name: String,
    pub appointments: Vec<Appointment>,
    pub blocked_slots: Vec<(u32, String)>,
    pub on_call_vet: String,
}

// ---------------------------------------------------------------------------
// Test 22: Clinical trial enrollment with multi-visit protocol
// ---------------------------------------------------------------------------

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
pub struct InclusionCriteria {
    pub criterion: String,
    pub met: bool,
    pub verification_method: String,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
pub struct AdverseEvent {
    pub date_epoch: u32,
    pub description: String,
    pub severity: String,
    pub related_to_treatment: String,
    pub action_taken: String,
    pub resolved: bool,
    pub resolution_epoch: Option<u32>,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
pub struct TrialVisit {
    pub visit_number: u8,
    pub scheduled_epoch: u32,
    pub actual_epoch: Option<u32>,
    pub procedures: Vec<String>,
    pub measurements: Vec<(String, String)>,
    pub treatment_administered: Option<String>,
    pub adverse_events: Vec<AdverseEvent>,
    pub investigator_notes: Option<String>,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
pub struct TrialProtocol {
    pub protocol_id: String,
    pub title: String,
    pub sponsor: String,
    pub phase: String,
    pub investigational_product: String,
    pub control_product: Option<String>,
    pub total_visits: u8,
    pub duration_weeks: u16,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
pub struct ClinicalTrialEnrollment {
    pub enrollment_id: String,
    pub patient_id: u64,
    pub protocol: TrialProtocol,
    pub enrollment_date_epoch: u32,
    pub randomization_group: String,
    pub blinded: bool,
    pub inclusion_criteria: Vec<InclusionCriteria>,
    pub consent_obtained: bool,
    pub consent_date_epoch: u32,
    pub visits: Vec<TrialVisit>,
    pub withdrawn: bool,
    pub withdrawal_reason: Option<String>,
}
