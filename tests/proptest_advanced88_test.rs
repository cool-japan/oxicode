//! Advanced property-based tests (set 88) — Dental practice management and
//! oral health records domain.
//!
//! 22 top-level #[test] functions, each containing exactly one proptest! block.
//! Covers tooth charting (universal numbering 1–32), periodontal probing depths,
//! radiograph exposure parameters, treatment plan cost estimates, insurance claim
//! codes (CDT), appointment scheduling slots, orthodontic bracket positions,
//! endodontic canal measurements, crown/bridge prep specifications, patient
//! consent forms, sterilization autoclave logs, and more.

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
use proptest::prelude::*;

// ── Domain types ──────────────────────────────────────────────────────────────

/// Universal tooth number (1–32 for permanent teeth).
#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct ToothCharting {
    /// Patient record identifier.
    patient_id: u64,
    /// Tooth number in universal numbering system (1–32).
    tooth_number: u8,
    /// Surface condition code: 0=sound, 1=caries, 2=restored, 3=missing, 4=implant.
    surface_condition: u8,
    /// Buccal surface involved.
    buccal: bool,
    /// Lingual surface involved.
    lingual: bool,
    /// Mesial surface involved.
    mesial: bool,
    /// Distal surface involved.
    distal: bool,
    /// Occlusal/incisal surface involved.
    occlusal: bool,
    /// Mobility grade (0–3).
    mobility_grade: u8,
}

/// Periodontal probing depth record for a single tooth (six sites).
#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct PeriodontalProbing {
    /// Patient record identifier.
    patient_id: u64,
    /// Tooth number (1–32).
    tooth_number: u8,
    /// Probing depths in mm for six sites: DB, B, MB, DL, L, ML.
    depth_db_mm: u8,
    depth_b_mm: u8,
    depth_mb_mm: u8,
    depth_dl_mm: u8,
    depth_l_mm: u8,
    depth_ml_mm: u8,
    /// Bleeding on probing detected at any site.
    bleeding_on_probing: bool,
    /// Furcation involvement grade (0–3).
    furcation_grade: u8,
    /// Recession in mm.
    recession_mm: u8,
}

/// Dental radiograph exposure parameters.
#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct RadiographExposure {
    /// Radiograph identifier.
    radiograph_id: u64,
    /// Tube voltage in kVp.
    kvp: u8,
    /// Tube current in mA.
    ma: u8,
    /// Exposure time in milliseconds.
    exposure_time_ms: u16,
    /// Film type: 0=periapical, 1=bitewing, 2=panoramic, 3=cephalometric, 4=CBCT.
    film_type: u8,
    /// Sensor size (0, 1, 2 for child/adult sizes).
    sensor_size: u8,
    /// Digital sensor used.
    is_digital: bool,
    /// Thyroid collar applied.
    thyroid_collar: bool,
    /// Lead apron applied.
    lead_apron: bool,
}

/// Treatment plan cost estimate for a procedure.
#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct TreatmentPlanCost {
    /// Plan identifier.
    plan_id: u64,
    /// Patient identifier.
    patient_id: u64,
    /// Tooth number (0 for non-tooth-specific).
    tooth_number: u8,
    /// CDT procedure code as numeric suffix (e.g. 2740 for D2740).
    cdt_code_suffix: u16,
    /// Provider fee in cents.
    provider_fee_cents: u32,
    /// Insurance estimated coverage in cents.
    insurance_coverage_cents: u32,
    /// Patient estimated copay in cents.
    patient_copay_cents: u32,
    /// Priority: 0=urgent, 1=high, 2=medium, 3=low, 4=elective.
    priority: u8,
    /// Pre-authorization required.
    pre_auth_required: bool,
}

/// Insurance claim with CDT codes.
#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct InsuranceClaim {
    /// Claim identifier.
    claim_id: u64,
    /// Patient identifier.
    patient_id: u64,
    /// Provider NPI (numeric, stored as u64).
    provider_npi: u64,
    /// CDT code suffix (numeric part after 'D').
    cdt_code_suffix: u16,
    /// Number of surfaces treated.
    surfaces_count: u8,
    /// Tooth number (1–32, 0 if non-tooth-specific).
    tooth_number: u8,
    /// Billed amount in cents.
    billed_cents: u32,
    /// Claim status: 0=submitted, 1=pending, 2=approved, 3=denied, 4=appealed.
    status: u8,
    /// Date of service as days since epoch.
    service_date_days: u32,
}

/// Appointment scheduling slot.
#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct AppointmentSlot {
    /// Appointment identifier.
    appointment_id: u64,
    /// Patient identifier.
    patient_id: u64,
    /// Provider identifier.
    provider_id: u32,
    /// Operatory/chair number.
    operatory: u8,
    /// Start time as minutes since midnight.
    start_minutes: u16,
    /// Duration in minutes.
    duration_minutes: u16,
    /// Appointment type: 0=recall, 1=restorative, 2=surgical, 3=endo, 4=ortho, 5=emergency.
    appointment_type: u8,
    /// Confirmed by patient.
    confirmed: bool,
    /// Date as days since epoch.
    date_days: u32,
}

/// Orthodontic bracket position for a single tooth.
#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct OrthodonticBracket {
    /// Case identifier.
    case_id: u64,
    /// Tooth number (1–32).
    tooth_number: u8,
    /// Bracket type: 0=metal, 1=ceramic, 2=lingual, 3=self-ligating.
    bracket_type: u8,
    /// Mesiodistal position offset in mm (from tooth center, ×100 for precision).
    md_offset_hundredths_mm: i16,
    /// Occlusogingival position offset in mm (×100).
    og_offset_hundredths_mm: i16,
    /// Torque prescription in degrees (×10 for precision).
    torque_tenths_deg: i16,
    /// Angulation prescription in degrees (×10).
    angulation_tenths_deg: i16,
    /// Wire slot size: 0=0.018", 1=0.022".
    slot_size: u8,
    /// Bonded successfully.
    bonded: bool,
}

/// Endodontic canal measurement for root canal therapy.
#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct EndodonticCanal {
    /// Case identifier.
    case_id: u64,
    /// Tooth number (1–32).
    tooth_number: u8,
    /// Canal name code: 0=MB, 1=DB, 2=P, 3=ML, 4=DL, 5=MB2, 6=single.
    canal_code: u8,
    /// Working length in mm (×10 for precision).
    working_length_tenths_mm: u16,
    /// Apical diameter ISO size (e.g. 25 for #25).
    apical_size_iso: u8,
    /// Taper percentage (e.g. 4 for 0.04).
    taper_pct: u8,
    /// Apex locator reading (0–100).
    apex_locator_reading: u8,
    /// Calcified canal.
    calcified: bool,
    /// Curved canal (>25 degrees).
    curved: bool,
}

/// Crown and bridge preparation specification.
#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct CrownBridgePrep {
    /// Case identifier.
    case_id: u64,
    /// Abutment tooth number.
    tooth_number: u8,
    /// Restoration type: 0=single crown, 1=bridge pontic, 2=bridge abutment,
    /// 3=onlay, 4=veneer.
    restoration_type: u8,
    /// Material: 0=PFM, 1=zirconia, 2=e.max, 3=gold, 4=composite.
    material: u8,
    /// Occlusal reduction in mm (×10).
    occlusal_reduction_tenths_mm: u8,
    /// Axial reduction in mm (×10).
    axial_reduction_tenths_mm: u8,
    /// Margin design: 0=chamfer, 1=shoulder, 2=knife-edge, 3=deep chamfer.
    margin_design: u8,
    /// Taper angle in degrees.
    taper_deg: u8,
    /// Shade guide value (Vita Classic A1–D4, mapped 0–15).
    shade_value: u8,
    /// Impression taken digitally.
    digital_impression: bool,
}

/// Patient consent form record.
#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct ConsentForm {
    /// Consent record identifier.
    consent_id: u64,
    /// Patient identifier.
    patient_id: u64,
    /// Procedure CDT code suffix.
    procedure_cdt_suffix: u16,
    /// Tooth number (0 if non-tooth-specific).
    tooth_number: u8,
    /// Risks discussed count.
    risks_discussed_count: u8,
    /// Alternatives discussed count.
    alternatives_discussed_count: u8,
    /// Consent given.
    consent_given: bool,
    /// Guardian signed (for minors).
    guardian_signed: bool,
    /// Interpreter used.
    interpreter_used: bool,
    /// Timestamp as seconds since epoch.
    timestamp_s: u64,
}

/// Sterilization autoclave cycle log entry.
#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct AutoclaveCycleLog {
    /// Cycle identifier.
    cycle_id: u64,
    /// Autoclave unit number.
    unit_number: u8,
    /// Cycle type: 0=gravity, 1=prevacuum, 2=flash, 3=liquid.
    cycle_type: u8,
    /// Temperature in degrees Celsius (×10 for precision).
    temp_tenths_c: u16,
    /// Pressure in PSI (×10).
    pressure_tenths_psi: u16,
    /// Sterilization time in seconds.
    sterilization_time_s: u16,
    /// Drying time in seconds.
    drying_time_s: u16,
    /// Biological indicator passed.
    bi_passed: bool,
    /// Chemical indicator passed.
    ci_passed: bool,
    /// Load number for the day.
    load_number: u8,
}

/// Dental hygiene recall interval and risk assessment.
#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct HygieneRecall {
    /// Patient identifier.
    patient_id: u64,
    /// Recall interval in months.
    interval_months: u8,
    /// Caries risk: 0=low, 1=moderate, 2=high.
    caries_risk: u8,
    /// Periodontal risk: 0=low, 1=moderate, 2=high.
    perio_risk: u8,
    /// Oral cancer screening performed.
    cancer_screening: bool,
    /// Fluoride varnish applied.
    fluoride_applied: bool,
    /// Sealants placed count.
    sealants_placed: u8,
    /// Plaque index score (0–100).
    plaque_index: u8,
    /// Last visit days since epoch.
    last_visit_days: u32,
}

/// Dental implant placement record.
#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct ImplantPlacement {
    /// Case identifier.
    case_id: u64,
    /// Site tooth number (position).
    site_number: u8,
    /// Implant diameter in mm (×10).
    diameter_tenths_mm: u8,
    /// Implant length in mm (×10).
    length_tenths_mm: u8,
    /// Platform type: 0=internal hex, 1=external hex, 2=conical, 3=tri-lobe.
    platform_type: u8,
    /// Insertion torque in Ncm.
    insertion_torque_ncm: u8,
    /// ISQ (implant stability quotient, 0–100).
    isq_value: u8,
    /// Bone graft used.
    bone_graft: bool,
    /// Membrane placed.
    membrane_placed: bool,
    /// Immediate provisional placed.
    immediate_provisional: bool,
}

/// TMJ (temporomandibular joint) assessment.
#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct TmjAssessment {
    /// Patient identifier.
    patient_id: u64,
    /// Maximum opening in mm.
    max_opening_mm: u8,
    /// Lateral excursion left in mm.
    lateral_left_mm: u8,
    /// Lateral excursion right in mm.
    lateral_right_mm: u8,
    /// Protrusion in mm.
    protrusion_mm: u8,
    /// Joint sounds left: 0=none, 1=click, 2=crepitus, 3=pop.
    sounds_left: u8,
    /// Joint sounds right: 0=none, 1=click, 2=crepitus, 3=pop.
    sounds_right: u8,
    /// Pain scale (0–10).
    pain_score: u8,
    /// Deviation on opening detected.
    deviation: bool,
    /// Night guard recommended.
    night_guard_recommended: bool,
}

/// Oral pathology biopsy record.
#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct OralBiopsy {
    /// Biopsy identifier.
    biopsy_id: u64,
    /// Patient identifier.
    patient_id: u64,
    /// Lesion location code: 0=tongue, 1=floor of mouth, 2=buccal mucosa,
    /// 3=palate, 4=gingiva, 5=lip, 6=other.
    location_code: u8,
    /// Lesion size in mm (greatest dimension).
    size_mm: u8,
    /// Biopsy type: 0=incisional, 1=excisional, 2=brush, 3=punch.
    biopsy_type: u8,
    /// Margin status: 0=clear, 1=close, 2=positive, 3=not applicable.
    margin_status: u8,
    /// Malignant finding.
    malignant: bool,
    /// Follow-up interval in weeks.
    followup_weeks: u8,
}

/// Fluoride treatment application record.
#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct FluorideTreatment {
    /// Record identifier.
    record_id: u64,
    /// Patient identifier.
    patient_id: u64,
    /// Fluoride type: 0=NaF varnish, 1=APF gel, 2=SnF2 rinse, 3=SDF.
    fluoride_type: u8,
    /// Concentration in ppm (stored as u16).
    concentration_ppm: u16,
    /// Application time in seconds.
    application_time_s: u16,
    /// Quadrants treated (bitmask: bit0=UR, bit1=UL, bit2=LR, bit3=LL).
    quadrants_bitmask: u8,
    /// Patient tolerated well.
    tolerated: bool,
    /// Post-treatment instructions given.
    instructions_given: bool,
}

/// Full-mouth periodontal summary across all teeth.
#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct FullMouthPerioSummary {
    /// Patient identifier.
    patient_id: u64,
    /// Total teeth present.
    teeth_present: u8,
    /// Sites with probing depth >= 4mm.
    sites_4mm_plus: u16,
    /// Sites with probing depth >= 6mm.
    sites_6mm_plus: u16,
    /// Percentage of sites with bleeding on probing (0–100).
    bop_percentage: u8,
    /// Mean clinical attachment level in tenths of mm.
    mean_cal_tenths_mm: u16,
    /// Periodontal classification: 0=health, 1=gingivitis, 2=stage I,
    /// 3=stage II, 4=stage III, 5=stage IV.
    classification: u8,
    /// Grade: 0=A(slow), 1=B(moderate), 2=C(rapid).
    grade: u8,
    /// Smoker.
    smoker: bool,
    /// Diabetic.
    diabetic: bool,
}

/// Dental lab case tracking record.
#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct LabCaseTracking {
    /// Lab case identifier.
    lab_case_id: u64,
    /// Provider identifier.
    provider_id: u32,
    /// Tooth number.
    tooth_number: u8,
    /// Restoration type code.
    restoration_code: u8,
    /// Material code.
    material_code: u8,
    /// Shade code (0–15).
    shade_code: u8,
    /// Status: 0=ordered, 1=in-progress, 2=shipped, 3=received, 4=seated.
    status: u8,
    /// Rush order.
    rush: bool,
    /// Ship date as days since epoch.
    ship_date_days: u32,
    /// Fee in cents.
    fee_cents: u32,
}

// ── Strategy generators ─────────────────────────────────────────────────────

prop_compose! {
    fn arb_tooth_charting()(
        patient_id in any::<u64>(),
        tooth_number in 1u8..=32,
        surface_condition in 0u8..=4,
        buccal in any::<bool>(),
        lingual in any::<bool>(),
        mesial in any::<bool>(),
        distal in any::<bool>(),
        occlusal in any::<bool>(),
        mobility_grade in 0u8..=3,
    ) -> ToothCharting {
        ToothCharting {
            patient_id, tooth_number, surface_condition,
            buccal, lingual, mesial, distal, occlusal, mobility_grade,
        }
    }
}

prop_compose! {
    fn arb_periodontal_probing()(
        patient_id in any::<u64>(),
        tooth_number in 1u8..=32,
        depth_db_mm in 1u8..=12,
        depth_b_mm in 1u8..=12,
        depth_mb_mm in 1u8..=12,
        depth_dl_mm in 1u8..=12,
        depth_l_mm in 1u8..=12,
        depth_ml_mm in 1u8..=12,
        bleeding_on_probing in any::<bool>(),
        furcation_grade in 0u8..=3,
        recession_mm in 0u8..=10,
    ) -> PeriodontalProbing {
        PeriodontalProbing {
            patient_id, tooth_number,
            depth_db_mm, depth_b_mm, depth_mb_mm,
            depth_dl_mm, depth_l_mm, depth_ml_mm,
            bleeding_on_probing, furcation_grade, recession_mm,
        }
    }
}

prop_compose! {
    fn arb_radiograph_exposure()(
        radiograph_id in any::<u64>(),
        kvp in 50u8..=90,
        ma in 4u8..=15,
        exposure_time_ms in 10u16..=2000,
        film_type in 0u8..=4,
        sensor_size in 0u8..=2,
        is_digital in any::<bool>(),
        thyroid_collar in any::<bool>(),
        lead_apron in any::<bool>(),
    ) -> RadiographExposure {
        RadiographExposure {
            radiograph_id, kvp, ma, exposure_time_ms,
            film_type, sensor_size, is_digital, thyroid_collar, lead_apron,
        }
    }
}

prop_compose! {
    fn arb_treatment_plan_cost()(
        plan_id in any::<u64>(),
        patient_id in any::<u64>(),
        tooth_number in 0u8..=32,
        cdt_code_suffix in 100u16..=9999,
        provider_fee_cents in 1000u32..=500_000,
        insurance_coverage_cents in 0u32..=500_000,
        patient_copay_cents in 0u32..=500_000,
        priority in 0u8..=4,
        pre_auth_required in any::<bool>(),
    ) -> TreatmentPlanCost {
        TreatmentPlanCost {
            plan_id, patient_id, tooth_number, cdt_code_suffix,
            provider_fee_cents, insurance_coverage_cents, patient_copay_cents,
            priority, pre_auth_required,
        }
    }
}

prop_compose! {
    fn arb_insurance_claim()(
        claim_id in any::<u64>(),
        patient_id in any::<u64>(),
        provider_npi in any::<u64>(),
        cdt_code_suffix in 100u16..=9999,
        surfaces_count in 1u8..=5,
        tooth_number in 0u8..=32,
        billed_cents in 1000u32..=1_000_000,
        status in 0u8..=4,
        service_date_days in 18000u32..=25000,
    ) -> InsuranceClaim {
        InsuranceClaim {
            claim_id, patient_id, provider_npi, cdt_code_suffix,
            surfaces_count, tooth_number, billed_cents, status, service_date_days,
        }
    }
}

prop_compose! {
    fn arb_appointment_slot()(
        appointment_id in any::<u64>(),
        patient_id in any::<u64>(),
        provider_id in any::<u32>(),
        operatory in 1u8..=12,
        start_minutes in 0u16..=1440,
        duration_minutes in 15u16..=180,
        appointment_type in 0u8..=5,
        confirmed in any::<bool>(),
        date_days in 18000u32..=25000,
    ) -> AppointmentSlot {
        AppointmentSlot {
            appointment_id, patient_id, provider_id, operatory,
            start_minutes, duration_minutes, appointment_type, confirmed, date_days,
        }
    }
}

prop_compose! {
    fn arb_orthodontic_bracket()(
        case_id in any::<u64>(),
        tooth_number in 1u8..=32,
        bracket_type in 0u8..=3,
        md_offset_hundredths_mm in -200i16..=200,
        og_offset_hundredths_mm in -200i16..=200,
        torque_tenths_deg in -250i16..=250,
        angulation_tenths_deg in -150i16..=150,
        slot_size in 0u8..=1,
        bonded in any::<bool>(),
    ) -> OrthodonticBracket {
        OrthodonticBracket {
            case_id, tooth_number, bracket_type,
            md_offset_hundredths_mm, og_offset_hundredths_mm,
            torque_tenths_deg, angulation_tenths_deg, slot_size, bonded,
        }
    }
}

prop_compose! {
    fn arb_endodontic_canal()(
        case_id in any::<u64>(),
        tooth_number in 1u8..=32,
        canal_code in 0u8..=6,
        working_length_tenths_mm in 80u16..=300,
        apical_size_iso in 15u8..=80,
        taper_pct in 2u8..=10,
        apex_locator_reading in 0u8..=100,
        calcified in any::<bool>(),
        curved in any::<bool>(),
    ) -> EndodonticCanal {
        EndodonticCanal {
            case_id, tooth_number, canal_code,
            working_length_tenths_mm, apical_size_iso, taper_pct,
            apex_locator_reading, calcified, curved,
        }
    }
}

prop_compose! {
    fn arb_crown_bridge_prep()(
        case_id in any::<u64>(),
        tooth_number in 1u8..=32,
        restoration_type in 0u8..=4,
        material in 0u8..=4,
        occlusal_reduction_tenths_mm in 10u8..=25,
        axial_reduction_tenths_mm in 8u8..=20,
        margin_design in 0u8..=3,
        taper_deg in 4u8..=12,
        shade_value in 0u8..=15,
        digital_impression in any::<bool>(),
    ) -> CrownBridgePrep {
        CrownBridgePrep {
            case_id, tooth_number, restoration_type, material,
            occlusal_reduction_tenths_mm, axial_reduction_tenths_mm,
            margin_design, taper_deg, shade_value, digital_impression,
        }
    }
}

prop_compose! {
    fn arb_consent_form()(
        consent_id in any::<u64>(),
        patient_id in any::<u64>(),
        procedure_cdt_suffix in 100u16..=9999,
        tooth_number in 0u8..=32,
        risks_discussed_count in 1u8..=20,
        alternatives_discussed_count in 1u8..=10,
        consent_given in any::<bool>(),
        guardian_signed in any::<bool>(),
        interpreter_used in any::<bool>(),
        timestamp_s in any::<u64>(),
    ) -> ConsentForm {
        ConsentForm {
            consent_id, patient_id, procedure_cdt_suffix, tooth_number,
            risks_discussed_count, alternatives_discussed_count,
            consent_given, guardian_signed, interpreter_used, timestamp_s,
        }
    }
}

prop_compose! {
    fn arb_autoclave_cycle_log()(
        cycle_id in any::<u64>(),
        unit_number in 1u8..=5,
        cycle_type in 0u8..=3,
        temp_tenths_c in 1210u16..=1370,
        pressure_tenths_psi in 150u16..=350,
        sterilization_time_s in 180u16..=1800,
        drying_time_s in 120u16..=1800,
        bi_passed in any::<bool>(),
        ci_passed in any::<bool>(),
        load_number in 1u8..=20,
    ) -> AutoclaveCycleLog {
        AutoclaveCycleLog {
            cycle_id, unit_number, cycle_type,
            temp_tenths_c, pressure_tenths_psi,
            sterilization_time_s, drying_time_s,
            bi_passed, ci_passed, load_number,
        }
    }
}

prop_compose! {
    fn arb_hygiene_recall()(
        patient_id in any::<u64>(),
        interval_months in 3u8..=12,
        caries_risk in 0u8..=2,
        perio_risk in 0u8..=2,
        cancer_screening in any::<bool>(),
        fluoride_applied in any::<bool>(),
        sealants_placed in 0u8..=8,
        plaque_index in 0u8..=100,
        last_visit_days in 18000u32..=25000,
    ) -> HygieneRecall {
        HygieneRecall {
            patient_id, interval_months, caries_risk, perio_risk,
            cancer_screening, fluoride_applied, sealants_placed,
            plaque_index, last_visit_days,
        }
    }
}

prop_compose! {
    fn arb_implant_placement()(
        case_id in any::<u64>(),
        site_number in 1u8..=32,
        diameter_tenths_mm in 30u8..=60,
        length_tenths_mm in 60u8..=160,
        platform_type in 0u8..=3,
        insertion_torque_ncm in 10u8..=80,
        isq_value in 40u8..=85,
        bone_graft in any::<bool>(),
        membrane_placed in any::<bool>(),
        immediate_provisional in any::<bool>(),
    ) -> ImplantPlacement {
        ImplantPlacement {
            case_id, site_number, diameter_tenths_mm, length_tenths_mm,
            platform_type, insertion_torque_ncm, isq_value,
            bone_graft, membrane_placed, immediate_provisional,
        }
    }
}

prop_compose! {
    fn arb_tmj_assessment()(
        patient_id in any::<u64>(),
        max_opening_mm in 15u8..=65,
        lateral_left_mm in 0u8..=15,
        lateral_right_mm in 0u8..=15,
        protrusion_mm in 0u8..=12,
        sounds_left in 0u8..=3,
        sounds_right in 0u8..=3,
        pain_score in 0u8..=10,
        deviation in any::<bool>(),
        night_guard_recommended in any::<bool>(),
    ) -> TmjAssessment {
        TmjAssessment {
            patient_id, max_opening_mm, lateral_left_mm, lateral_right_mm,
            protrusion_mm, sounds_left, sounds_right, pain_score,
            deviation, night_guard_recommended,
        }
    }
}

prop_compose! {
    fn arb_oral_biopsy()(
        biopsy_id in any::<u64>(),
        patient_id in any::<u64>(),
        location_code in 0u8..=6,
        size_mm in 1u8..=50,
        biopsy_type in 0u8..=3,
        margin_status in 0u8..=3,
        malignant in any::<bool>(),
        followup_weeks in 1u8..=52,
    ) -> OralBiopsy {
        OralBiopsy {
            biopsy_id, patient_id, location_code, size_mm,
            biopsy_type, margin_status, malignant, followup_weeks,
        }
    }
}

prop_compose! {
    fn arb_fluoride_treatment()(
        record_id in any::<u64>(),
        patient_id in any::<u64>(),
        fluoride_type in 0u8..=3,
        concentration_ppm in 900u16..=44800,
        application_time_s in 30u16..=300,
        quadrants_bitmask in 0u8..=15,
        tolerated in any::<bool>(),
        instructions_given in any::<bool>(),
    ) -> FluorideTreatment {
        FluorideTreatment {
            record_id, patient_id, fluoride_type, concentration_ppm,
            application_time_s, quadrants_bitmask, tolerated, instructions_given,
        }
    }
}

prop_compose! {
    fn arb_full_mouth_perio_summary()(
        patient_id in any::<u64>(),
        teeth_present in 0u8..=32,
        sites_4mm_plus in 0u16..=192,
        sites_6mm_plus in 0u16..=192,
        bop_percentage in 0u8..=100,
        mean_cal_tenths_mm in 0u16..=120,
        classification in 0u8..=5,
        grade in 0u8..=2,
        smoker in any::<bool>(),
        diabetic in any::<bool>(),
    ) -> FullMouthPerioSummary {
        FullMouthPerioSummary {
            patient_id, teeth_present, sites_4mm_plus, sites_6mm_plus,
            bop_percentage, mean_cal_tenths_mm, classification, grade,
            smoker, diabetic,
        }
    }
}

prop_compose! {
    fn arb_lab_case_tracking()(
        lab_case_id in any::<u64>(),
        provider_id in any::<u32>(),
        tooth_number in 1u8..=32,
        restoration_code in 0u8..=10,
        material_code in 0u8..=6,
        shade_code in 0u8..=15,
        status in 0u8..=4,
        rush in any::<bool>(),
        ship_date_days in 18000u32..=25000,
        fee_cents in 5000u32..=200_000,
    ) -> LabCaseTracking {
        LabCaseTracking {
            lab_case_id, provider_id, tooth_number,
            restoration_code, material_code, shade_code, status,
            rush, ship_date_days, fee_cents,
        }
    }
}

// ── Tests ───────────────────────────────────────────────────────────────────

#[test]
fn test_tooth_charting_roundtrip() {
    proptest!(|(val in arb_tooth_charting())| {
        let encoded = encode_to_vec(&val).expect("encode ToothCharting failed");
        let (decoded, _) = decode_from_slice::<ToothCharting>(&encoded)
            .expect("decode ToothCharting failed");
        prop_assert_eq!(val, decoded);
    });
}

#[test]
fn test_periodontal_probing_roundtrip() {
    proptest!(|(val in arb_periodontal_probing())| {
        let encoded = encode_to_vec(&val).expect("encode PeriodontalProbing failed");
        let (decoded, _) = decode_from_slice::<PeriodontalProbing>(&encoded)
            .expect("decode PeriodontalProbing failed");
        prop_assert_eq!(val, decoded);
    });
}

#[test]
fn test_radiograph_exposure_roundtrip() {
    proptest!(|(val in arb_radiograph_exposure())| {
        let encoded = encode_to_vec(&val).expect("encode RadiographExposure failed");
        let (decoded, _) = decode_from_slice::<RadiographExposure>(&encoded)
            .expect("decode RadiographExposure failed");
        prop_assert_eq!(val, decoded);
    });
}

#[test]
fn test_treatment_plan_cost_roundtrip() {
    proptest!(|(val in arb_treatment_plan_cost())| {
        let encoded = encode_to_vec(&val).expect("encode TreatmentPlanCost failed");
        let (decoded, _) = decode_from_slice::<TreatmentPlanCost>(&encoded)
            .expect("decode TreatmentPlanCost failed");
        prop_assert_eq!(val, decoded);
    });
}

#[test]
fn test_insurance_claim_roundtrip() {
    proptest!(|(val in arb_insurance_claim())| {
        let encoded = encode_to_vec(&val).expect("encode InsuranceClaim failed");
        let (decoded, _) = decode_from_slice::<InsuranceClaim>(&encoded)
            .expect("decode InsuranceClaim failed");
        prop_assert_eq!(val, decoded);
    });
}

#[test]
fn test_appointment_slot_roundtrip() {
    proptest!(|(val in arb_appointment_slot())| {
        let encoded = encode_to_vec(&val).expect("encode AppointmentSlot failed");
        let (decoded, _) = decode_from_slice::<AppointmentSlot>(&encoded)
            .expect("decode AppointmentSlot failed");
        prop_assert_eq!(val, decoded);
    });
}

#[test]
fn test_orthodontic_bracket_roundtrip() {
    proptest!(|(val in arb_orthodontic_bracket())| {
        let encoded = encode_to_vec(&val).expect("encode OrthodonticBracket failed");
        let (decoded, _) = decode_from_slice::<OrthodonticBracket>(&encoded)
            .expect("decode OrthodonticBracket failed");
        prop_assert_eq!(val, decoded);
    });
}

#[test]
fn test_endodontic_canal_roundtrip() {
    proptest!(|(val in arb_endodontic_canal())| {
        let encoded = encode_to_vec(&val).expect("encode EndodonticCanal failed");
        let (decoded, _) = decode_from_slice::<EndodonticCanal>(&encoded)
            .expect("decode EndodonticCanal failed");
        prop_assert_eq!(val, decoded);
    });
}

#[test]
fn test_crown_bridge_prep_roundtrip() {
    proptest!(|(val in arb_crown_bridge_prep())| {
        let encoded = encode_to_vec(&val).expect("encode CrownBridgePrep failed");
        let (decoded, _) = decode_from_slice::<CrownBridgePrep>(&encoded)
            .expect("decode CrownBridgePrep failed");
        prop_assert_eq!(val, decoded);
    });
}

#[test]
fn test_consent_form_roundtrip() {
    proptest!(|(val in arb_consent_form())| {
        let encoded = encode_to_vec(&val).expect("encode ConsentForm failed");
        let (decoded, _) = decode_from_slice::<ConsentForm>(&encoded)
            .expect("decode ConsentForm failed");
        prop_assert_eq!(val, decoded);
    });
}

#[test]
fn test_autoclave_cycle_log_roundtrip() {
    proptest!(|(val in arb_autoclave_cycle_log())| {
        let encoded = encode_to_vec(&val).expect("encode AutoclaveCycleLog failed");
        let (decoded, _) = decode_from_slice::<AutoclaveCycleLog>(&encoded)
            .expect("decode AutoclaveCycleLog failed");
        prop_assert_eq!(val, decoded);
    });
}

#[test]
fn test_hygiene_recall_roundtrip() {
    proptest!(|(val in arb_hygiene_recall())| {
        let encoded = encode_to_vec(&val).expect("encode HygieneRecall failed");
        let (decoded, _) = decode_from_slice::<HygieneRecall>(&encoded)
            .expect("decode HygieneRecall failed");
        prop_assert_eq!(val, decoded);
    });
}

#[test]
fn test_implant_placement_roundtrip() {
    proptest!(|(val in arb_implant_placement())| {
        let encoded = encode_to_vec(&val).expect("encode ImplantPlacement failed");
        let (decoded, _) = decode_from_slice::<ImplantPlacement>(&encoded)
            .expect("decode ImplantPlacement failed");
        prop_assert_eq!(val, decoded);
    });
}

#[test]
fn test_tmj_assessment_roundtrip() {
    proptest!(|(val in arb_tmj_assessment())| {
        let encoded = encode_to_vec(&val).expect("encode TmjAssessment failed");
        let (decoded, _) = decode_from_slice::<TmjAssessment>(&encoded)
            .expect("decode TmjAssessment failed");
        prop_assert_eq!(val, decoded);
    });
}

#[test]
fn test_oral_biopsy_roundtrip() {
    proptest!(|(val in arb_oral_biopsy())| {
        let encoded = encode_to_vec(&val).expect("encode OralBiopsy failed");
        let (decoded, _) = decode_from_slice::<OralBiopsy>(&encoded)
            .expect("decode OralBiopsy failed");
        prop_assert_eq!(val, decoded);
    });
}

#[test]
fn test_fluoride_treatment_roundtrip() {
    proptest!(|(val in arb_fluoride_treatment())| {
        let encoded = encode_to_vec(&val).expect("encode FluorideTreatment failed");
        let (decoded, _) = decode_from_slice::<FluorideTreatment>(&encoded)
            .expect("decode FluorideTreatment failed");
        prop_assert_eq!(val, decoded);
    });
}

#[test]
fn test_full_mouth_perio_summary_roundtrip() {
    proptest!(|(val in arb_full_mouth_perio_summary())| {
        let encoded = encode_to_vec(&val).expect("encode FullMouthPerioSummary failed");
        let (decoded, _) = decode_from_slice::<FullMouthPerioSummary>(&encoded)
            .expect("decode FullMouthPerioSummary failed");
        prop_assert_eq!(val, decoded);
    });
}

#[test]
fn test_lab_case_tracking_roundtrip() {
    proptest!(|(val in arb_lab_case_tracking())| {
        let encoded = encode_to_vec(&val).expect("encode LabCaseTracking failed");
        let (decoded, _) = decode_from_slice::<LabCaseTracking>(&encoded)
            .expect("decode LabCaseTracking failed");
        prop_assert_eq!(val, decoded);
    });
}

#[test]
fn test_tooth_charting_vec_roundtrip() {
    proptest!(|(vals in proptest::collection::vec(arb_tooth_charting(), 0..16))| {
        let encoded = encode_to_vec(&vals).expect("encode Vec<ToothCharting> failed");
        let (decoded, _) = decode_from_slice::<Vec<ToothCharting>>(&encoded)
            .expect("decode Vec<ToothCharting> failed");
        prop_assert_eq!(vals, decoded);
    });
}

#[test]
fn test_periodontal_probing_vec_roundtrip() {
    proptest!(|(vals in proptest::collection::vec(arb_periodontal_probing(), 0..16))| {
        let encoded = encode_to_vec(&vals).expect("encode Vec<PeriodontalProbing> failed");
        let (decoded, _) = decode_from_slice::<Vec<PeriodontalProbing>>(&encoded)
            .expect("decode Vec<PeriodontalProbing> failed");
        prop_assert_eq!(vals, decoded);
    });
}

#[test]
fn test_treatment_and_claim_pair_roundtrip() {
    proptest!(|(plan in arb_treatment_plan_cost(), claim in arb_insurance_claim())| {
        let pair = (plan.clone(), claim.clone());
        let encoded = encode_to_vec(&pair).expect("encode (TreatmentPlanCost, InsuranceClaim) failed");
        let (decoded, _) = decode_from_slice::<(TreatmentPlanCost, InsuranceClaim)>(&encoded)
            .expect("decode (TreatmentPlanCost, InsuranceClaim) failed");
        prop_assert_eq!(pair, decoded);
    });
}

#[test]
fn test_optional_implant_with_biopsy_roundtrip() {
    proptest!(|(
        implant in proptest::option::of(arb_implant_placement()),
        biopsy in proptest::option::of(arb_oral_biopsy()),
    )| {
        let pair = (implant.clone(), biopsy.clone());
        let encoded = encode_to_vec(&pair)
            .expect("encode (Option<ImplantPlacement>, Option<OralBiopsy>) failed");
        let (decoded, _) = decode_from_slice::<(
            Option<ImplantPlacement>,
            Option<OralBiopsy>,
        )>(&encoded)
            .expect("decode (Option<ImplantPlacement>, Option<OralBiopsy>) failed");
        prop_assert_eq!(pair, decoded);
    });
}
