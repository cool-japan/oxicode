//! Advanced Zstd compression tests for OxiCode — Dental & Orthodontic Practice Management domain.
//!
//! Covers encode → compress → decompress → decode round-trips for types that
//! model real-world dental practice data: patient dental charts, treatment plans,
//! orthodontic bracket placements, cephalometric analyses, X-ray metadata,
//! periodontal probing depths, CDT insurance codes, appointments, lab cases,
//! aligner tray sequences, TMJ disorder assessments, whitening records,
//! pediatric milestones, sterilization logs, and patient consent forms.

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
enum ToothSurface {
    Mesial,
    Distal,
    Buccal,
    Lingual,
    Occlusal,
    Incisal,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
enum ToothCondition {
    Healthy,
    Caries,
    Filled,
    Crown,
    Missing,
    Implant,
    RootCanal,
    Fractured,
    Impacted,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
enum TreatmentType {
    Crown,
    Bridge,
    Implant,
    RootCanal,
    Extraction,
    Veneer,
    Inlay,
    Onlay,
    Denture,
    ScalingRootPlaning,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
enum BracketType {
    MetalStandard,
    CeramicClear,
    SelfLigating,
    LingualCustom,
    GoldPlated,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
enum WireType {
    NickelTitanium,
    StainlessSteel,
    BetaTitanium,
    CopperNiTi,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
enum XrayType {
    Periapical,
    Bitewing,
    Panoramic,
    Cephalometric,
    ConeBeamCt,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
enum AppointmentStatus {
    Scheduled,
    Confirmed,
    InProgress,
    Completed,
    Cancelled,
    NoShow,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
enum LabCaseStatus {
    ImpressionTaken,
    SentToLab,
    ModelFabricated,
    InProduction,
    QualityCheck,
    Shipped,
    Received,
    Seated,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
enum TmjSymptom {
    Clicking,
    Popping,
    Locking,
    Pain,
    LimitedOpening,
    Crepitus,
    Deviation,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
enum SterilizationMethod {
    SteamAutoclave,
    DryHeat,
    ChemicalVapor,
    EthyleneOxide,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
enum ConsentType {
    GeneralTreatment,
    Extraction,
    ImplantSurgery,
    OrthodonticTreatment,
    Sedation,
    Whitening,
    Radiograph,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
enum WhiteningMethod {
    InOffice,
    TakeHome,
    LaserAssisted,
    CombinedProtocol,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
enum PediatricMilestone {
    FirstToothEruption,
    PrimaryDentitionComplete,
    FirstPermanentMolar,
    MixedDentition,
    PermanentDentitionComplete,
    WisdomToothEruption,
}

/// A single tooth entry in a dental chart.
#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct ToothEntry {
    tooth_number: u8, // Universal numbering 1–32 (adult) or A–T mapped to 51–70
    condition: ToothCondition,
    surfaces_affected: Vec<ToothSurface>,
    mobility_grade: u8, // 0–3
    notes: String,
}

/// Full dental chart for a patient.
#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct DentalChart {
    patient_id: u64,
    chart_date_epoch: u64,
    teeth: Vec<ToothEntry>,
    overall_hygiene_score: u8, // 1–10
}

/// A single treatment plan item.
#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct TreatmentPlanItem {
    item_id: u32,
    tooth_number: u8,
    treatment: TreatmentType,
    cdt_code: String,
    estimated_cost_cents: u32,
    priority: u8, // 1=urgent, 5=elective
    notes: String,
}

/// Complete treatment plan.
#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct TreatmentPlan {
    plan_id: u64,
    patient_id: u64,
    created_epoch: u64,
    items: Vec<TreatmentPlanItem>,
    total_estimated_cents: u64,
    insurance_coverage_cents: u64,
}

/// Orthodontic bracket placement record.
#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct BracketPlacement {
    tooth_number: u8,
    bracket: BracketType,
    slot_size_thou: u16, // thousandths of an inch (e.g. 22 = 0.022")
    torque_deg: i8,
    angulation_deg: i8,
    offset_mm_x10: u16,
}

/// Full orthodontic appliance configuration.
#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct OrthoApplianceConfig {
    case_id: u64,
    patient_id: u64,
    wire: WireType,
    wire_diameter_thou: u16,
    brackets: Vec<BracketPlacement>,
    estimated_months: u8,
}

/// Cephalometric analysis measurement set.
#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct CephalometricAnalysis {
    analysis_id: u64,
    patient_id: u64,
    sna_angle_deg_x10: i16,
    snb_angle_deg_x10: i16,
    anb_angle_deg_x10: i16,
    fma_angle_deg_x10: i16,
    impa_angle_deg_x10: i16,
    upper_incisor_to_na_mm_x10: i16,
    lower_incisor_to_nb_mm_x10: i16,
    wits_appraisal_mm_x10: i16,
    overjet_mm_x10: u16,
    overbite_mm_x10: u16,
    notes: String,
}

/// X-ray image metadata record.
#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct XrayMetadata {
    xray_id: u64,
    patient_id: u64,
    xray_type: XrayType,
    taken_epoch: u64,
    exposure_ms: u16,
    tube_voltage_kv: u8,
    tube_current_ma: u8,
    image_width: u16,
    image_height: u16,
    file_size_bytes: u32,
    findings: String,
}

/// Periodontal probing record for one tooth (6 sites).
#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct PeriodontalProbing {
    tooth_number: u8,
    depths_mm: [u8; 6], // MB, B, DB, ML, L, DL
    recession_mm: [u8; 6],
    bleeding_on_probing: [bool; 6],
    furcation_grade: u8, // 0–3
}

/// Full periodontal chart.
#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct PeriodontalChart {
    chart_id: u64,
    patient_id: u64,
    exam_epoch: u64,
    probings: Vec<PeriodontalProbing>,
    plaque_index_pct: u8,
}

/// Insurance claim with CDT codes.
#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct InsuranceClaim {
    claim_id: u64,
    patient_id: u64,
    provider_npi: String,
    submitted_epoch: u64,
    cdt_codes: Vec<String>,
    tooth_numbers: Vec<u8>,
    total_charge_cents: u32,
    insurance_pays_cents: u32,
    patient_pays_cents: u32,
    status: String,
}

/// Appointment record.
#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct Appointment {
    appointment_id: u64,
    patient_id: u64,
    provider_id: u32,
    scheduled_epoch: u64,
    duration_minutes: u16,
    status: AppointmentStatus,
    procedure_codes: Vec<String>,
    notes: String,
}

/// Lab case tracking record.
#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct LabCase {
    case_id: u64,
    patient_id: u64,
    lab_name: String,
    tooth_numbers: Vec<u8>,
    material: String,
    shade: String,
    status: LabCaseStatus,
    sent_epoch: u64,
    expected_return_epoch: u64,
    cost_cents: u32,
}

/// Clear aligner tray sequence entry.
#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct AlignerTray {
    tray_number: u16,
    wear_start_epoch: u64,
    wear_days: u8,
    attachment_teeth: Vec<u8>,
    ipr_teeth: Vec<u8>, // inter-proximal reduction teeth
    ipr_amount_mm_x10: Vec<u8>,
    upper: bool,
}

/// Full aligner treatment sequence.
#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct AlignerSequence {
    sequence_id: u64,
    patient_id: u64,
    total_trays: u16,
    trays: Vec<AlignerTray>,
    estimated_months: u8,
}

/// TMJ disorder assessment.
#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct TmjAssessment {
    assessment_id: u64,
    patient_id: u64,
    exam_epoch: u64,
    symptoms: Vec<TmjSymptom>,
    max_opening_mm: u8,
    left_lateral_mm: u8,
    right_lateral_mm: u8,
    protrusion_mm: u8,
    pain_scale: u8, // 0–10
    joint_sounds_left: bool,
    joint_sounds_right: bool,
    notes: String,
}

/// Whitening treatment record.
#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct WhiteningRecord {
    record_id: u64,
    patient_id: u64,
    method: WhiteningMethod,
    sessions_completed: u8,
    sessions_planned: u8,
    shade_before: String,
    shade_after: String,
    peroxide_concentration_pct: u8,
    sensitivity_reported: bool,
    notes: String,
}

/// Pediatric dental development milestone record.
#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct PediatricRecord {
    record_id: u64,
    patient_id: u64,
    birth_epoch: u64,
    milestones: Vec<(PediatricMilestone, u64)>, // (milestone, observed_epoch)
    primary_teeth_present: Vec<u8>,
    permanent_teeth_present: Vec<u8>,
    fluoride_varnish_dates: Vec<u64>,
    sealant_teeth: Vec<u8>,
}

/// Infection control sterilization log entry.
#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct SterilizationLog {
    log_id: u64,
    cycle_epoch: u64,
    method: SterilizationMethod,
    load_number: u16,
    temperature_c_x10: u16,
    duration_minutes: u16,
    pressure_kpa: u16,
    biological_indicator_pass: bool,
    chemical_indicator_pass: bool,
    operator_id: u32,
    instrument_ids: Vec<u32>,
}

/// Patient consent form record.
#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct ConsentForm {
    consent_id: u64,
    patient_id: u64,
    consent_type: ConsentType,
    signed_epoch: u64,
    witness_id: u32,
    risks_acknowledged: Vec<String>,
    alternatives_discussed: Vec<String>,
    patient_questions: Vec<String>,
    revoked: bool,
}

// ---------------------------------------------------------------------------
// Helper builders
// ---------------------------------------------------------------------------

fn make_tooth_entry(num: u8) -> ToothEntry {
    let condition = match num % 9 {
        0 => ToothCondition::Healthy,
        1 => ToothCondition::Caries,
        2 => ToothCondition::Filled,
        3 => ToothCondition::Crown,
        4 => ToothCondition::Missing,
        5 => ToothCondition::Implant,
        6 => ToothCondition::RootCanal,
        7 => ToothCondition::Fractured,
        _ => ToothCondition::Impacted,
    };
    let surfaces = match num % 3 {
        0 => vec![ToothSurface::Occlusal],
        1 => vec![ToothSurface::Mesial, ToothSurface::Distal],
        _ => vec![
            ToothSurface::Buccal,
            ToothSurface::Lingual,
            ToothSurface::Occlusal,
        ],
    };
    ToothEntry {
        tooth_number: num,
        condition,
        surfaces_affected: surfaces,
        mobility_grade: num % 4,
        notes: format!("Tooth #{num} clinical note"),
    }
}

fn make_dental_chart(patient_id: u64) -> DentalChart {
    DentalChart {
        patient_id,
        chart_date_epoch: 1_700_000_000 + patient_id * 86400,
        teeth: (1u8..=32).map(make_tooth_entry).collect(),
        overall_hygiene_score: ((patient_id % 10) + 1) as u8,
    }
}

fn make_treatment_plan_item(id: u32) -> TreatmentPlanItem {
    let treatment = match id % 10 {
        0 => TreatmentType::Crown,
        1 => TreatmentType::Bridge,
        2 => TreatmentType::Implant,
        3 => TreatmentType::RootCanal,
        4 => TreatmentType::Extraction,
        5 => TreatmentType::Veneer,
        6 => TreatmentType::Inlay,
        7 => TreatmentType::Onlay,
        8 => TreatmentType::Denture,
        _ => TreatmentType::ScalingRootPlaning,
    };
    let cdt = match id % 5 {
        0 => "D2740",
        1 => "D6010",
        2 => "D3330",
        3 => "D7210",
        _ => "D4341",
    };
    TreatmentPlanItem {
        item_id: id,
        tooth_number: ((id % 32) + 1) as u8,
        treatment,
        cdt_code: cdt.to_string(),
        estimated_cost_cents: 50_000 + id * 1_500,
        priority: (id % 5 + 1) as u8,
        notes: format!("Treatment item {id} notes"),
    }
}

fn make_treatment_plan(plan_id: u64) -> TreatmentPlan {
    let items: Vec<TreatmentPlanItem> = (0u32..8).map(make_treatment_plan_item).collect();
    let total: u64 = items.iter().map(|i| i.estimated_cost_cents as u64).sum();
    TreatmentPlan {
        plan_id,
        patient_id: plan_id * 10 + 1,
        created_epoch: 1_700_000_000 + plan_id * 3600,
        items,
        total_estimated_cents: total,
        insurance_coverage_cents: total * 60 / 100,
    }
}

fn make_bracket_placement(tooth: u8) -> BracketPlacement {
    let bracket = match tooth % 5 {
        0 => BracketType::MetalStandard,
        1 => BracketType::CeramicClear,
        2 => BracketType::SelfLigating,
        3 => BracketType::LingualCustom,
        _ => BracketType::GoldPlated,
    };
    BracketPlacement {
        tooth_number: tooth,
        bracket,
        slot_size_thou: 22,
        torque_deg: (tooth as i8 % 7) - 3,
        angulation_deg: (tooth as i8 % 5) - 2,
        offset_mm_x10: 20 + (tooth as u16 % 10),
    }
}

fn make_ortho_config(case_id: u64) -> OrthoApplianceConfig {
    OrthoApplianceConfig {
        case_id,
        patient_id: case_id * 10 + 2,
        wire: WireType::NickelTitanium,
        wire_diameter_thou: 16,
        brackets: (3u8..=14).map(make_bracket_placement).collect(),
        estimated_months: 18,
    }
}

fn make_cephalometric(analysis_id: u64) -> CephalometricAnalysis {
    CephalometricAnalysis {
        analysis_id,
        patient_id: analysis_id * 10 + 3,
        sna_angle_deg_x10: 820,
        snb_angle_deg_x10: 790,
        anb_angle_deg_x10: 30,
        fma_angle_deg_x10: 250,
        impa_angle_deg_x10: 950,
        upper_incisor_to_na_mm_x10: 42,
        lower_incisor_to_nb_mm_x10: 40,
        wits_appraisal_mm_x10: -10,
        overjet_mm_x10: 35,
        overbite_mm_x10: 30,
        notes: format!("Ceph analysis #{analysis_id} — Class I skeletal pattern"),
    }
}

fn make_xray_metadata(xray_id: u64) -> XrayMetadata {
    let xray_type = match xray_id % 5 {
        0 => XrayType::Periapical,
        1 => XrayType::Bitewing,
        2 => XrayType::Panoramic,
        3 => XrayType::Cephalometric,
        _ => XrayType::ConeBeamCt,
    };
    XrayMetadata {
        xray_id,
        patient_id: xray_id * 10 + 4,
        xray_type,
        taken_epoch: 1_700_000_000 + xray_id * 7200,
        exposure_ms: 160 + (xray_id % 50) as u16,
        tube_voltage_kv: 70,
        tube_current_ma: 8,
        image_width: 2048,
        image_height: 1536,
        file_size_bytes: 3_500_000 + (xray_id * 1_000) as u32,
        findings: format!("Xray #{xray_id} — no significant pathology"),
    }
}

fn make_perio_probing(tooth: u8) -> PeriodontalProbing {
    let d = (tooth % 4) + 1;
    PeriodontalProbing {
        tooth_number: tooth,
        depths_mm: [d, d + 1, d, d + 1, d, d + 1],
        recession_mm: [0, tooth % 2, 0, tooth % 2, 0, tooth % 2],
        bleeding_on_probing: [tooth % 3 == 0, false, tooth % 2 == 0, false, true, false],
        furcation_grade: if tooth >= 14 && tooth <= 19 {
            tooth % 3
        } else {
            0
        },
    }
}

fn make_perio_chart(chart_id: u64) -> PeriodontalChart {
    PeriodontalChart {
        chart_id,
        patient_id: chart_id * 10 + 5,
        exam_epoch: 1_700_000_000 + chart_id * 86400,
        probings: (1u8..=32).map(make_perio_probing).collect(),
        plaque_index_pct: ((chart_id % 60) + 10) as u8,
    }
}

fn make_insurance_claim(claim_id: u64) -> InsuranceClaim {
    InsuranceClaim {
        claim_id,
        patient_id: claim_id * 10 + 6,
        provider_npi: format!("NPI{:010}", claim_id),
        submitted_epoch: 1_700_000_000 + claim_id * 3600,
        cdt_codes: vec![
            "D0120".to_string(),
            "D1110".to_string(),
            "D0274".to_string(),
            "D2740".to_string(),
        ],
        tooth_numbers: vec![3, 14, 19, 30],
        total_charge_cents: 185_000,
        insurance_pays_cents: 111_000,
        patient_pays_cents: 74_000,
        status: "Submitted".to_string(),
    }
}

fn make_appointment(appointment_id: u64) -> Appointment {
    let status = match appointment_id % 6 {
        0 => AppointmentStatus::Scheduled,
        1 => AppointmentStatus::Confirmed,
        2 => AppointmentStatus::InProgress,
        3 => AppointmentStatus::Completed,
        4 => AppointmentStatus::Cancelled,
        _ => AppointmentStatus::NoShow,
    };
    Appointment {
        appointment_id,
        patient_id: appointment_id * 10 + 7,
        provider_id: (appointment_id % 5 + 1) as u32,
        scheduled_epoch: 1_700_000_000 + appointment_id * 1800,
        duration_minutes: 30 + ((appointment_id % 4) * 15) as u16,
        status,
        procedure_codes: vec!["D0120".to_string(), "D1110".to_string()],
        notes: format!("Appointment #{appointment_id} recall visit"),
    }
}

fn make_lab_case(case_id: u64) -> LabCase {
    let status = match case_id % 8 {
        0 => LabCaseStatus::ImpressionTaken,
        1 => LabCaseStatus::SentToLab,
        2 => LabCaseStatus::ModelFabricated,
        3 => LabCaseStatus::InProduction,
        4 => LabCaseStatus::QualityCheck,
        5 => LabCaseStatus::Shipped,
        6 => LabCaseStatus::Received,
        _ => LabCaseStatus::Seated,
    };
    LabCase {
        case_id,
        patient_id: case_id * 10 + 8,
        lab_name: format!("DentalLab-{}", case_id % 3),
        tooth_numbers: vec![((case_id % 32) + 1) as u8],
        material: "Zirconia".to_string(),
        shade: format!("A{}", (case_id % 4) + 1),
        status,
        sent_epoch: 1_700_000_000 + case_id * 86400,
        expected_return_epoch: 1_700_000_000 + case_id * 86400 + 14 * 86400,
        cost_cents: 25_000 + (case_id * 500) as u32,
    }
}

fn make_aligner_tray(num: u16, upper: bool) -> AlignerTray {
    AlignerTray {
        tray_number: num,
        wear_start_epoch: 1_700_000_000 + (num as u64) * 14 * 86400,
        wear_days: 14,
        attachment_teeth: vec![6, 8, 11, 14, 22, 27],
        ipr_teeth: if num % 3 == 0 {
            vec![9, 10, 23, 24]
        } else {
            vec![]
        },
        ipr_amount_mm_x10: if num % 3 == 0 {
            vec![2, 2, 3, 3]
        } else {
            vec![]
        },
        upper,
    }
}

fn make_aligner_sequence(seq_id: u64) -> AlignerSequence {
    let total = 24u16;
    let trays: Vec<AlignerTray> = (1..=total)
        .map(|n| make_aligner_tray(n, n % 2 == 0))
        .collect();
    AlignerSequence {
        sequence_id: seq_id,
        patient_id: seq_id * 10 + 9,
        total_trays: total,
        trays,
        estimated_months: 12,
    }
}

fn make_tmj_assessment(assessment_id: u64) -> TmjAssessment {
    let symptoms = match assessment_id % 4 {
        0 => vec![TmjSymptom::Clicking, TmjSymptom::Pain],
        1 => vec![
            TmjSymptom::Popping,
            TmjSymptom::LimitedOpening,
            TmjSymptom::Crepitus,
        ],
        2 => vec![TmjSymptom::Locking, TmjSymptom::Deviation],
        _ => vec![
            TmjSymptom::Pain,
            TmjSymptom::Clicking,
            TmjSymptom::Popping,
            TmjSymptom::LimitedOpening,
        ],
    };
    TmjAssessment {
        assessment_id,
        patient_id: assessment_id * 10 + 10,
        exam_epoch: 1_700_000_000 + assessment_id * 86400,
        symptoms,
        max_opening_mm: 35 + (assessment_id % 15) as u8,
        left_lateral_mm: 8 + (assessment_id % 4) as u8,
        right_lateral_mm: 7 + (assessment_id % 5) as u8,
        protrusion_mm: 6 + (assessment_id % 3) as u8,
        pain_scale: (assessment_id % 11) as u8,
        joint_sounds_left: assessment_id % 2 == 0,
        joint_sounds_right: assessment_id % 3 == 0,
        notes: format!("TMJ assessment #{assessment_id} — recommend splint therapy"),
    }
}

fn make_whitening_record(record_id: u64) -> WhiteningRecord {
    let method = match record_id % 4 {
        0 => WhiteningMethod::InOffice,
        1 => WhiteningMethod::TakeHome,
        2 => WhiteningMethod::LaserAssisted,
        _ => WhiteningMethod::CombinedProtocol,
    };
    WhiteningRecord {
        record_id,
        patient_id: record_id * 10 + 11,
        method,
        sessions_completed: (record_id % 4 + 1) as u8,
        sessions_planned: 4,
        shade_before: format!("A{}", (record_id % 4) + 2),
        shade_after: format!("B{}", (record_id % 2) + 1),
        peroxide_concentration_pct: 35,
        sensitivity_reported: record_id % 3 == 0,
        notes: format!("Whitening #{record_id} — good patient compliance"),
    }
}

fn make_pediatric_record(record_id: u64) -> PediatricRecord {
    PediatricRecord {
        record_id,
        patient_id: record_id * 10 + 12,
        birth_epoch: 1_600_000_000 - record_id * 31_536_000,
        milestones: vec![
            (
                PediatricMilestone::FirstToothEruption,
                1_600_000_000 + 15_768_000,
            ),
            (
                PediatricMilestone::PrimaryDentitionComplete,
                1_600_000_000 + 78_840_000,
            ),
            (
                PediatricMilestone::FirstPermanentMolar,
                1_600_000_000 + 189_216_000,
            ),
        ],
        primary_teeth_present: (51u8..=70).collect(),
        permanent_teeth_present: vec![3, 14, 19, 30],
        fluoride_varnish_dates: vec![
            1_700_000_000,
            1_700_000_000 + 180 * 86400,
            1_700_000_000 + 360 * 86400,
        ],
        sealant_teeth: vec![3, 14, 19, 30],
    }
}

fn make_sterilization_log(log_id: u64) -> SterilizationLog {
    let method = match log_id % 4 {
        0 => SterilizationMethod::SteamAutoclave,
        1 => SterilizationMethod::DryHeat,
        2 => SterilizationMethod::ChemicalVapor,
        _ => SterilizationMethod::EthyleneOxide,
    };
    SterilizationLog {
        log_id,
        cycle_epoch: 1_700_000_000 + log_id * 3600,
        method,
        load_number: (log_id % 200 + 1) as u16,
        temperature_c_x10: 1340,
        duration_minutes: 30,
        pressure_kpa: 207,
        biological_indicator_pass: true,
        chemical_indicator_pass: true,
        operator_id: (log_id % 5 + 100) as u32,
        instrument_ids: (0u32..12)
            .map(|i| 5000 + i + (log_id as u32 * 100))
            .collect(),
    }
}

fn make_consent_form(consent_id: u64) -> ConsentForm {
    let consent_type = match consent_id % 7 {
        0 => ConsentType::GeneralTreatment,
        1 => ConsentType::Extraction,
        2 => ConsentType::ImplantSurgery,
        3 => ConsentType::OrthodonticTreatment,
        4 => ConsentType::Sedation,
        5 => ConsentType::Whitening,
        _ => ConsentType::Radiograph,
    };
    ConsentForm {
        consent_id,
        patient_id: consent_id * 10 + 13,
        consent_type,
        signed_epoch: 1_700_000_000 + consent_id * 86400,
        witness_id: (consent_id % 10 + 200) as u32,
        risks_acknowledged: vec![
            "Infection".to_string(),
            "Bleeding".to_string(),
            "Nerve damage".to_string(),
            "Treatment failure".to_string(),
        ],
        alternatives_discussed: vec![
            "No treatment".to_string(),
            "Alternative procedure".to_string(),
        ],
        patient_questions: vec![format!("Question from patient on consent #{consent_id}")],
        revoked: false,
    }
}

// ---------------------------------------------------------------------------
// Tests
// ---------------------------------------------------------------------------

/// 1. Dental chart round-trip for a single patient.
#[test]
fn test_zstd_dental_chart_roundtrip() {
    let chart = make_dental_chart(1001);
    let encoded = encode_to_vec(&chart).expect("encode DentalChart failed");
    let compressed = compress(&encoded, Compression::Zstd).expect("zstd compress failed");
    let decompressed = decompress(&compressed).expect("zstd decompress failed");
    let (decoded, _): (DentalChart, usize) =
        decode_from_slice(&decompressed).expect("decode DentalChart failed");
    assert_eq!(chart, decoded);
}

/// 2. Treatment plan round-trip.
#[test]
fn test_zstd_treatment_plan_roundtrip() {
    let plan = make_treatment_plan(42);
    let encoded = encode_to_vec(&plan).expect("encode TreatmentPlan failed");
    let compressed = compress(&encoded, Compression::Zstd).expect("zstd compress failed");
    let decompressed = decompress(&compressed).expect("zstd decompress failed");
    let (decoded, _): (TreatmentPlan, usize) =
        decode_from_slice(&decompressed).expect("decode TreatmentPlan failed");
    assert_eq!(plan, decoded);
}

/// 3. Orthodontic appliance config round-trip.
#[test]
fn test_zstd_ortho_config_roundtrip() {
    let config = make_ortho_config(7);
    let encoded = encode_to_vec(&config).expect("encode OrthoApplianceConfig failed");
    let compressed = compress(&encoded, Compression::Zstd).expect("zstd compress failed");
    let decompressed = decompress(&compressed).expect("zstd decompress failed");
    let (decoded, _): (OrthoApplianceConfig, usize) =
        decode_from_slice(&decompressed).expect("decode OrthoApplianceConfig failed");
    assert_eq!(config, decoded);
}

/// 4. Cephalometric analysis round-trip.
#[test]
fn test_zstd_cephalometric_analysis_roundtrip() {
    let analysis = make_cephalometric(99);
    let encoded = encode_to_vec(&analysis).expect("encode CephalometricAnalysis failed");
    let compressed = compress(&encoded, Compression::Zstd).expect("zstd compress failed");
    let decompressed = decompress(&compressed).expect("zstd decompress failed");
    let (decoded, _): (CephalometricAnalysis, usize) =
        decode_from_slice(&decompressed).expect("decode CephalometricAnalysis failed");
    assert_eq!(analysis, decoded);
}

/// 5. X-ray metadata round-trip.
#[test]
fn test_zstd_xray_metadata_roundtrip() {
    let xray = make_xray_metadata(55);
    let encoded = encode_to_vec(&xray).expect("encode XrayMetadata failed");
    let compressed = compress(&encoded, Compression::Zstd).expect("zstd compress failed");
    let decompressed = decompress(&compressed).expect("zstd decompress failed");
    let (decoded, _): (XrayMetadata, usize) =
        decode_from_slice(&decompressed).expect("decode XrayMetadata failed");
    assert_eq!(xray, decoded);
}

/// 6. Periodontal chart round-trip.
#[test]
fn test_zstd_periodontal_chart_roundtrip() {
    let chart = make_perio_chart(200);
    let encoded = encode_to_vec(&chart).expect("encode PeriodontalChart failed");
    let compressed = compress(&encoded, Compression::Zstd).expect("zstd compress failed");
    let decompressed = decompress(&compressed).expect("zstd decompress failed");
    let (decoded, _): (PeriodontalChart, usize) =
        decode_from_slice(&decompressed).expect("decode PeriodontalChart failed");
    assert_eq!(chart, decoded);
}

/// 7. Insurance claim with CDT codes round-trip.
#[test]
fn test_zstd_insurance_claim_roundtrip() {
    let claim = make_insurance_claim(333);
    let encoded = encode_to_vec(&claim).expect("encode InsuranceClaim failed");
    let compressed = compress(&encoded, Compression::Zstd).expect("zstd compress failed");
    let decompressed = decompress(&compressed).expect("zstd decompress failed");
    let (decoded, _): (InsuranceClaim, usize) =
        decode_from_slice(&decompressed).expect("decode InsuranceClaim failed");
    assert_eq!(claim, decoded);
}

/// 8. Appointment scheduling round-trip.
#[test]
fn test_zstd_appointment_roundtrip() {
    let appt = make_appointment(77);
    let encoded = encode_to_vec(&appt).expect("encode Appointment failed");
    let compressed = compress(&encoded, Compression::Zstd).expect("zstd compress failed");
    let decompressed = decompress(&compressed).expect("zstd decompress failed");
    let (decoded, _): (Appointment, usize) =
        decode_from_slice(&decompressed).expect("decode Appointment failed");
    assert_eq!(appt, decoded);
}

/// 9. Lab case tracking round-trip.
#[test]
fn test_zstd_lab_case_roundtrip() {
    let case = make_lab_case(15);
    let encoded = encode_to_vec(&case).expect("encode LabCase failed");
    let compressed = compress(&encoded, Compression::Zstd).expect("zstd compress failed");
    let decompressed = decompress(&compressed).expect("zstd decompress failed");
    let (decoded, _): (LabCase, usize) =
        decode_from_slice(&decompressed).expect("decode LabCase failed");
    assert_eq!(case, decoded);
}

/// 10. Aligner tray sequence round-trip.
#[test]
fn test_zstd_aligner_sequence_roundtrip() {
    let seq = make_aligner_sequence(5);
    let encoded = encode_to_vec(&seq).expect("encode AlignerSequence failed");
    let compressed = compress(&encoded, Compression::Zstd).expect("zstd compress failed");
    let decompressed = decompress(&compressed).expect("zstd decompress failed");
    let (decoded, _): (AlignerSequence, usize) =
        decode_from_slice(&decompressed).expect("decode AlignerSequence failed");
    assert_eq!(seq, decoded);
}

/// 11. TMJ disorder assessment round-trip.
#[test]
fn test_zstd_tmj_assessment_roundtrip() {
    let tmj = make_tmj_assessment(88);
    let encoded = encode_to_vec(&tmj).expect("encode TmjAssessment failed");
    let compressed = compress(&encoded, Compression::Zstd).expect("zstd compress failed");
    let decompressed = decompress(&compressed).expect("zstd decompress failed");
    let (decoded, _): (TmjAssessment, usize) =
        decode_from_slice(&decompressed).expect("decode TmjAssessment failed");
    assert_eq!(tmj, decoded);
}

/// 12. Whitening treatment record round-trip.
#[test]
fn test_zstd_whitening_record_roundtrip() {
    let rec = make_whitening_record(12);
    let encoded = encode_to_vec(&rec).expect("encode WhiteningRecord failed");
    let compressed = compress(&encoded, Compression::Zstd).expect("zstd compress failed");
    let decompressed = decompress(&compressed).expect("zstd decompress failed");
    let (decoded, _): (WhiteningRecord, usize) =
        decode_from_slice(&decompressed).expect("decode WhiteningRecord failed");
    assert_eq!(rec, decoded);
}

/// 13. Pediatric development milestone record round-trip.
#[test]
fn test_zstd_pediatric_record_roundtrip() {
    let rec = make_pediatric_record(3);
    let encoded = encode_to_vec(&rec).expect("encode PediatricRecord failed");
    let compressed = compress(&encoded, Compression::Zstd).expect("zstd compress failed");
    let decompressed = decompress(&compressed).expect("zstd decompress failed");
    let (decoded, _): (PediatricRecord, usize) =
        decode_from_slice(&decompressed).expect("decode PediatricRecord failed");
    assert_eq!(rec, decoded);
}

/// 14. Sterilization log round-trip.
#[test]
fn test_zstd_sterilization_log_roundtrip() {
    let log = make_sterilization_log(501);
    let encoded = encode_to_vec(&log).expect("encode SterilizationLog failed");
    let compressed = compress(&encoded, Compression::Zstd).expect("zstd compress failed");
    let decompressed = decompress(&compressed).expect("zstd decompress failed");
    let (decoded, _): (SterilizationLog, usize) =
        decode_from_slice(&decompressed).expect("decode SterilizationLog failed");
    assert_eq!(log, decoded);
}

/// 15. Patient consent form round-trip.
#[test]
fn test_zstd_consent_form_roundtrip() {
    let form = make_consent_form(9);
    let encoded = encode_to_vec(&form).expect("encode ConsentForm failed");
    let compressed = compress(&encoded, Compression::Zstd).expect("zstd compress failed");
    let decompressed = decompress(&compressed).expect("zstd decompress failed");
    let (decoded, _): (ConsentForm, usize) =
        decode_from_slice(&decompressed).expect("decode ConsentForm failed");
    assert_eq!(form, decoded);
}

/// 16. Batch of dental charts — compression ratio check.
#[test]
fn test_zstd_dental_charts_batch_compression_ratio() {
    let charts: Vec<DentalChart> = (1u64..=50).map(make_dental_chart).collect();
    let encoded = encode_to_vec(&charts).expect("encode batch DentalCharts failed");
    let compressed = compress(&encoded, Compression::Zstd).expect("zstd compress batch failed");
    assert!(
        compressed.len() < encoded.len(),
        "compressed ({}) should be smaller than encoded ({})",
        compressed.len(),
        encoded.len()
    );
    let decompressed = decompress(&compressed).expect("zstd decompress batch failed");
    let (decoded, _): (Vec<DentalChart>, usize) =
        decode_from_slice(&decompressed).expect("decode batch DentalCharts failed");
    assert_eq!(charts, decoded);
}

/// 17. Batch of treatment plans — compression saves space.
#[test]
fn test_zstd_treatment_plans_batch_compression() {
    let plans: Vec<TreatmentPlan> = (1u64..=30).map(make_treatment_plan).collect();
    let encoded = encode_to_vec(&plans).expect("encode batch TreatmentPlans failed");
    let compressed = compress(&encoded, Compression::Zstd).expect("zstd compress failed");
    assert!(
        compressed.len() < encoded.len(),
        "compressed ({}) should be smaller than encoded ({})",
        compressed.len(),
        encoded.len()
    );
    let decompressed = decompress(&compressed).expect("zstd decompress failed");
    let (decoded, _): (Vec<TreatmentPlan>, usize) =
        decode_from_slice(&decompressed).expect("decode batch TreatmentPlans failed");
    assert_eq!(plans, decoded);
}

/// 18. Mixed appointment schedule — batch round-trip.
#[test]
fn test_zstd_appointment_schedule_batch() {
    let appointments: Vec<Appointment> = (1u64..=100).map(make_appointment).collect();
    let encoded = encode_to_vec(&appointments).expect("encode batch Appointments failed");
    let compressed = compress(&encoded, Compression::Zstd).expect("zstd compress failed");
    assert!(
        compressed.len() < encoded.len(),
        "compressed ({}) should be smaller than encoded ({})",
        compressed.len(),
        encoded.len()
    );
    let decompressed = decompress(&compressed).expect("zstd decompress failed");
    let (decoded, _): (Vec<Appointment>, usize) =
        decode_from_slice(&decompressed).expect("decode batch Appointments failed");
    assert_eq!(appointments, decoded);
}

/// 19. Full practice day — multiple sterilization logs.
#[test]
fn test_zstd_sterilization_logs_batch() {
    let logs: Vec<SterilizationLog> = (1u64..=40).map(make_sterilization_log).collect();
    let encoded = encode_to_vec(&logs).expect("encode batch SterilizationLogs failed");
    let compressed = compress(&encoded, Compression::Zstd).expect("zstd compress failed");
    assert!(
        compressed.len() < encoded.len(),
        "compressed ({}) should be smaller than encoded ({})",
        compressed.len(),
        encoded.len()
    );
    let decompressed = decompress(&compressed).expect("zstd decompress failed");
    let (decoded, _): (Vec<SterilizationLog>, usize) =
        decode_from_slice(&decompressed).expect("decode batch SterilizationLogs failed");
    assert_eq!(logs, decoded);
}

/// 20. Comprehensive patient record — all dental data combined.
#[test]
fn test_zstd_comprehensive_patient_record() {
    let record = (
        make_dental_chart(500),
        make_treatment_plan(500),
        make_perio_chart(500),
        make_insurance_claim(500),
        make_consent_form(500),
    );
    let encoded = encode_to_vec(&record).expect("encode comprehensive record failed");
    let compressed = compress(&encoded, Compression::Zstd).expect("zstd compress failed");
    let decompressed = decompress(&compressed).expect("zstd decompress failed");
    let (decoded, _): (
        (
            DentalChart,
            TreatmentPlan,
            PeriodontalChart,
            InsuranceClaim,
            ConsentForm,
        ),
        usize,
    ) = decode_from_slice(&decompressed).expect("decode comprehensive record failed");
    assert_eq!(record, decoded);
}

/// 21. Ortho + aligner combined — large payload compression check.
#[test]
fn test_zstd_ortho_aligner_combined_compression() {
    let configs: Vec<OrthoApplianceConfig> = (1u64..=20).map(make_ortho_config).collect();
    let sequences: Vec<AlignerSequence> = (1u64..=10).map(make_aligner_sequence).collect();
    let combined = (configs.clone(), sequences.clone());
    let encoded = encode_to_vec(&combined).expect("encode ortho+aligner failed");
    let compressed = compress(&encoded, Compression::Zstd).expect("zstd compress failed");
    assert!(
        compressed.len() < encoded.len(),
        "compressed ({}) should be smaller than encoded ({})",
        compressed.len(),
        encoded.len()
    );
    let decompressed = decompress(&compressed).expect("zstd decompress failed");
    let (decoded, _): ((Vec<OrthoApplianceConfig>, Vec<AlignerSequence>), usize) =
        decode_from_slice(&decompressed).expect("decode ortho+aligner failed");
    assert_eq!((configs, sequences), decoded);
}

/// 22. Full practice X-ray archive — batch compression ratio.
#[test]
fn test_zstd_xray_archive_batch_compression() {
    let xrays: Vec<XrayMetadata> = (1u64..=80).map(make_xray_metadata).collect();
    let encoded = encode_to_vec(&xrays).expect("encode batch XrayMetadata failed");
    let compressed = compress(&encoded, Compression::Zstd).expect("zstd compress failed");
    assert!(
        compressed.len() < encoded.len(),
        "compressed ({}) should be smaller than encoded ({})",
        compressed.len(),
        encoded.len()
    );
    let decompressed = decompress(&compressed).expect("zstd decompress failed");
    let (decoded, _): (Vec<XrayMetadata>, usize) =
        decode_from_slice(&decompressed).expect("decode batch XrayMetadata failed");
    assert_eq!(xrays, decoded);
}
