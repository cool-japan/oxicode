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

fn compress_lz4(data: &[u8]) -> Result<Vec<u8>, Box<dyn std::error::Error>> {
    compress(data, Compression::Lz4).map_err(|e| Box::new(e) as Box<dyn std::error::Error>)
}

fn decompress_lz4(data: &[u8]) -> Result<Vec<u8>, Box<dyn std::error::Error>> {
    decompress(data).map_err(|e| Box::new(e) as Box<dyn std::error::Error>)
}

// --- Domain Types ---

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
enum TriageLevel {
    Esi1Resuscitation,
    Esi2Emergent,
    Esi3Urgent,
    Esi4LessUrgent,
    Esi5NonUrgent,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct TriageRecord {
    patient_id: u64,
    esi_level: TriageLevel,
    chief_complaint: String,
    arrival_timestamp_epoch: u64,
    reassessment_minutes: u16,
    notes: String,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
enum DispatchPriority {
    Priority1LightsAndSirens,
    Priority2Expedited,
    Priority3Routine,
    Priority4Scheduled,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct AmbulanceDispatch {
    call_id: u64,
    unit_designator: String,
    priority: DispatchPriority,
    dispatch_epoch: u64,
    en_route_epoch: u64,
    on_scene_epoch: u64,
    transport_epoch: u64,
    at_hospital_epoch: u64,
    caller_location_lat: i32,
    caller_location_lon: i32,
    incident_type: String,
    crew_paramedic: String,
    crew_emt: String,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct VitalSigns {
    timestamp_epoch: u64,
    heart_rate_bpm: u16,
    systolic_bp: u16,
    diastolic_bp: u16,
    spo2_percent: u8,
    respiratory_rate: u8,
    temperature_c_x10: u16,
    gcs_eye: u8,
    gcs_verbal: u8,
    gcs_motor: u8,
    etco2_mmhg: u8,
    pain_scale_0_10: u8,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct VitalsTelemetry {
    patient_id: u64,
    device_serial: String,
    ecg_lead_ii_samples: Vec<i16>,
    spo2_waveform: Vec<u8>,
    vitals_snapshots: Vec<VitalSigns>,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
enum InjuryRegion {
    Head,
    Face,
    Chest,
    Abdomen,
    Extremity,
    External,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct TraumaAssessment {
    patient_id: u64,
    mechanism_of_injury: String,
    injuries: Vec<(InjuryRegion, u8)>,
    iss_total: u16,
    revised_trauma_score: u16,
    gcs_total: u8,
    systolic_bp: u16,
    respiratory_rate: u8,
    penetrating: bool,
    intubated: bool,
    notes: String,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
enum MedRoute {
    Intravenous,
    Intramuscular,
    Subcutaneous,
    Sublingual,
    Intraosseous,
    Endotracheal,
    Nebulized,
    Oral,
    Intranasal,
    Topical,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct MedicationAdmin {
    patient_id: u64,
    admin_epoch: u64,
    drug_name: String,
    dose_mg_x100: u32,
    route: MedRoute,
    provider_id: String,
    indication: String,
    response: String,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
enum AirwayDevice {
    Bvm,
    SupraglotticIGel,
    SupraglotticKingLt,
    EndotrachealOral,
    EndotrachealNasal,
    Cricothyrotomy,
    Cpap,
    Bipap,
    NasalCannula,
    NonRebreather,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct AirwayManagement {
    patient_id: u64,
    attempt_number: u8,
    device: AirwayDevice,
    success: bool,
    tube_size_mm_x10: u16,
    depth_cm_x10: u16,
    confirmation_method: String,
    pre_ox_spo2: u8,
    post_placement_spo2: u8,
    provider: String,
    complications: Vec<String>,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
enum CardiacRhythm {
    Asystole,
    Pea,
    VentricularFibrillation,
    PulselessVentricularTachycardia,
    Rosc,
    NormalSinus,
    AtrialFibrillation,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct ResuscitationEvent {
    elapsed_seconds: u32,
    rhythm: CardiacRhythm,
    action: String,
    shock_joules: Option<u16>,
    medication: Option<String>,
    compressor_id: String,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct CardiacArrestTimeline {
    patient_id: u64,
    witnessed: bool,
    bystander_cpr: bool,
    initial_rhythm: CardiacRhythm,
    collapse_epoch: u64,
    first_cpr_epoch: u64,
    first_defib_epoch: Option<u64>,
    rosc_epoch: Option<u64>,
    events: Vec<ResuscitationEvent>,
    total_downtime_seconds: u32,
    outcome: String,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct StrokeProtocol {
    patient_id: u64,
    activation_epoch: u64,
    last_known_well_epoch: u64,
    nihss_total: u8,
    nihss_items: Vec<u8>,
    facial_droop: bool,
    arm_drift: bool,
    speech_abnormal: bool,
    blood_glucose_mg_dl: u16,
    systolic_bp: u16,
    large_vessel_occlusion_screen_positive: bool,
    destination_facility: String,
    tpa_candidate: bool,
    thrombectomy_candidate: bool,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
enum TriageColor {
    Red,
    Yellow,
    Green,
    Black,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct MciPatient {
    triage_tag_number: u32,
    color: TriageColor,
    breathing: bool,
    respiratory_rate: u8,
    perfusion_cap_refill_seconds: u8,
    mental_status_follows_commands: bool,
    chief_complaint: String,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct MassCasualtyIncident {
    incident_id: u64,
    incident_name: String,
    declaration_epoch: u64,
    location_description: String,
    total_patients: u32,
    red_count: u32,
    yellow_count: u32,
    green_count: u32,
    black_count: u32,
    patients: Vec<MciPatient>,
    staging_area: String,
    transport_destinations: Vec<String>,
    command_structure: Vec<(String, String)>,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct SbarHandoff {
    patient_id: u64,
    situation: String,
    background: String,
    assessment: String,
    recommendation: String,
    vitals_at_handoff: VitalSigns,
    medications_given: Vec<String>,
    allergies: Vec<String>,
    receiving_provider: String,
    handoff_epoch: u64,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
enum FlightCategory {
    Scene,
    InterfacilityTransfer,
    SearchAndRescue,
    NeonatalTransport,
    OrganRetrieval,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct HemsFlightRecord {
    mission_id: u64,
    aircraft_tail_number: String,
    category: FlightCategory,
    dispatch_epoch: u64,
    liftoff_epoch: u64,
    on_scene_epoch: u64,
    depart_scene_epoch: u64,
    landing_epoch: u64,
    flight_time_minutes: u16,
    pilot: String,
    flight_nurse: String,
    flight_paramedic: String,
    landing_zone_coords_lat: i32,
    landing_zone_coords_lon: i32,
    weather_conditions: String,
    patient_weight_kg_x10: u16,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct PocLabResult {
    patient_id: u64,
    test_epoch: u64,
    glucose_mg_dl: Option<u16>,
    lactate_mmol_x10: Option<u16>,
    hemoglobin_g_dl_x10: Option<u16>,
    potassium_mmol_x100: Option<u16>,
    sodium_mmol: Option<u16>,
    troponin_ng_ml_x1000: Option<u32>,
    inr_x100: Option<u16>,
    creatinine_mg_dl_x100: Option<u16>,
    ph_x1000: Option<u16>,
    base_excess_x10: Option<i16>,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct PainAssessment {
    patient_id: u64,
    assessment_epoch: u64,
    numeric_rating_0_10: u8,
    location: String,
    quality: String,
    onset: String,
    duration_minutes: u32,
    aggravating_factors: Vec<String>,
    alleviating_factors: Vec<String>,
    radiation: Option<String>,
    intervention_given: Option<String>,
    reassessment_score: Option<u8>,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct EdBoardingMetrics {
    report_epoch: u64,
    ed_census: u32,
    total_boarding_patients: u32,
    avg_boarding_minutes: u32,
    max_boarding_minutes: u32,
    admitted_awaiting_bed: u32,
    ambulance_diversion_active: bool,
    beds_available: u32,
    beds_total: u32,
    patients_in_waiting_room: u32,
    door_to_provider_avg_minutes: u32,
    left_without_being_seen: u32,
    critical_care_occupancy_pct: u8,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
enum ResourceCategory {
    Ventilator,
    Monitor,
    DefibrillatorAed,
    InfusionPump,
    SuctionUnit,
    PortableOxygen,
    TourniquetBox,
    BurnKit,
    PediatricKit,
    HazmatSuit,
    TriageTagBox,
    Stretcher,
    WheelChair,
    SatellitePhone,
    PortableUltrasound,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct ResourceItem {
    category: ResourceCategory,
    description: String,
    quantity_available: u32,
    quantity_deployed: u32,
    last_inspected_epoch: u64,
    location: String,
    serviceable: bool,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct DisasterInventory {
    facility_name: String,
    report_epoch: u64,
    items: Vec<ResourceItem>,
    cache_locations: Vec<String>,
    expiring_medications_count: u32,
    total_mre_packs: u32,
    water_gallons: u32,
    generator_fuel_hours_remaining: u16,
    comm_plan: String,
}

// --- Tests ---

#[test]
fn test_triage_esi1_critical_patient() {
    let record = TriageRecord {
        patient_id: 900001,
        esi_level: TriageLevel::Esi1Resuscitation,
        chief_complaint: "Cardiac arrest, found down by family, CPR in progress".into(),
        arrival_timestamp_epoch: 1710500000,
        reassessment_minutes: 0,
        notes: "Immediate resuscitation bay, code team activated".into(),
    };
    let enc = encode_to_vec(&record).expect("encode triage esi1");
    let compressed = compress_lz4(&enc).expect("compress triage esi1");
    let decompressed = decompress_lz4(&compressed).expect("decompress triage esi1");
    let (decoded, _): (TriageRecord, usize) =
        decode_from_slice(&decompressed).expect("decode triage esi1");
    assert_eq!(record, decoded);
}

#[test]
fn test_multiple_triage_levels() {
    let records = vec![
        TriageRecord {
            patient_id: 900010,
            esi_level: TriageLevel::Esi1Resuscitation,
            chief_complaint: "Gunshot wound to chest, hypotensive".into(),
            arrival_timestamp_epoch: 1710500100,
            reassessment_minutes: 0,
            notes: "Trauma activation level 1".into(),
        },
        TriageRecord {
            patient_id: 900011,
            esi_level: TriageLevel::Esi2Emergent,
            chief_complaint: "Acute stroke symptoms, onset 45 min ago".into(),
            arrival_timestamp_epoch: 1710500200,
            reassessment_minutes: 15,
            notes: "Stroke alert activated, CT scanner notified".into(),
        },
        TriageRecord {
            patient_id: 900012,
            esi_level: TriageLevel::Esi3Urgent,
            chief_complaint: "Abdominal pain 8/10, vomiting x3 days".into(),
            arrival_timestamp_epoch: 1710500300,
            reassessment_minutes: 30,
            notes: "Needs 2+ resources: labs, imaging".into(),
        },
        TriageRecord {
            patient_id: 900013,
            esi_level: TriageLevel::Esi4LessUrgent,
            chief_complaint: "Laceration left forearm, controlled bleeding".into(),
            arrival_timestamp_epoch: 1710500400,
            reassessment_minutes: 60,
            notes: "1 resource expected: suture".into(),
        },
        TriageRecord {
            patient_id: 900014,
            esi_level: TriageLevel::Esi5NonUrgent,
            chief_complaint: "Medication refill request".into(),
            arrival_timestamp_epoch: 1710500500,
            reassessment_minutes: 120,
            notes: "No resources anticipated".into(),
        },
    ];
    let enc = encode_to_vec(&records).expect("encode triage vec");
    let compressed = compress_lz4(&enc).expect("compress triage vec");
    let decompressed = decompress_lz4(&compressed).expect("decompress triage vec");
    let (decoded, _): (Vec<TriageRecord>, usize) =
        decode_from_slice(&decompressed).expect("decode triage vec");
    assert_eq!(records, decoded);
}

#[test]
fn test_ambulance_dispatch_record() {
    let dispatch = AmbulanceDispatch {
        call_id: 2024031500001,
        unit_designator: "Medic-7".into(),
        priority: DispatchPriority::Priority1LightsAndSirens,
        dispatch_epoch: 1710500000,
        en_route_epoch: 1710500060,
        on_scene_epoch: 1710500420,
        transport_epoch: 1710500900,
        at_hospital_epoch: 1710501600,
        caller_location_lat: 40_712_776,
        caller_location_lon: -74_005_974,
        incident_type: "Cardiac emergency - chest pain with diaphoresis".into(),
        crew_paramedic: "P. Mitchell, NRP".into(),
        crew_emt: "E. Vasquez, EMT-B".into(),
    };
    let enc = encode_to_vec(&dispatch).expect("encode dispatch");
    let compressed = compress_lz4(&enc).expect("compress dispatch");
    let decompressed = decompress_lz4(&compressed).expect("decompress dispatch");
    let (decoded, _): (AmbulanceDispatch, usize) =
        decode_from_slice(&decompressed).expect("decode dispatch");
    assert_eq!(dispatch, decoded);
}

#[test]
fn test_vital_signs_telemetry_with_waveforms() {
    let ecg_samples: Vec<i16> = (0..500)
        .map(|i| {
            let phase = (i as f64) * 0.05;
            (phase.sin() * 200.0) as i16
        })
        .collect();
    let spo2_wave: Vec<u8> = (0..250).map(|i| (128_u16 + (i % 40)) as u8).collect();

    let telemetry = VitalsTelemetry {
        patient_id: 900020,
        device_serial: "ZOLL-X-2024-0473".into(),
        ecg_lead_ii_samples: ecg_samples,
        spo2_waveform: spo2_wave,
        vitals_snapshots: vec![
            VitalSigns {
                timestamp_epoch: 1710500000,
                heart_rate_bpm: 112,
                systolic_bp: 88,
                diastolic_bp: 54,
                spo2_percent: 91,
                respiratory_rate: 24,
                temperature_c_x10: 383,
                gcs_eye: 3,
                gcs_verbal: 4,
                gcs_motor: 5,
                etco2_mmhg: 32,
                pain_scale_0_10: 8,
            },
            VitalSigns {
                timestamp_epoch: 1710500300,
                heart_rate_bpm: 98,
                systolic_bp: 102,
                diastolic_bp: 64,
                spo2_percent: 96,
                respiratory_rate: 18,
                temperature_c_x10: 378,
                gcs_eye: 4,
                gcs_verbal: 5,
                gcs_motor: 6,
                etco2_mmhg: 38,
                pain_scale_0_10: 5,
            },
        ],
    };
    let enc = encode_to_vec(&telemetry).expect("encode telemetry");
    let compressed = compress_lz4(&enc).expect("compress telemetry");
    let decompressed = decompress_lz4(&compressed).expect("decompress telemetry");
    let (decoded, _): (VitalsTelemetry, usize) =
        decode_from_slice(&decompressed).expect("decode telemetry");
    assert_eq!(telemetry, decoded);
}

#[test]
fn test_telemetry_waveform_compression_ratio() {
    let ecg_samples: Vec<i16> = (0..2000)
        .map(|i| {
            let phase = (i as f64) * 0.02;
            (phase.sin() * 300.0) as i16
        })
        .collect();
    let spo2_wave: Vec<u8> = vec![128; 1000];

    let telemetry = VitalsTelemetry {
        patient_id: 900021,
        device_serial: "LP15-SN-98234".into(),
        ecg_lead_ii_samples: ecg_samples,
        spo2_waveform: spo2_wave,
        vitals_snapshots: vec![],
    };
    let enc = encode_to_vec(&telemetry).expect("encode telemetry waveform");
    let compressed = compress_lz4(&enc).expect("compress telemetry waveform");
    assert!(
        compressed.len() < enc.len(),
        "LZ4 compressed telemetry ({} bytes) should be smaller than uncompressed ({} bytes)",
        compressed.len(),
        enc.len()
    );
    let decompressed = decompress_lz4(&compressed).expect("decompress telemetry waveform");
    let (decoded, _): (VitalsTelemetry, usize) =
        decode_from_slice(&decompressed).expect("decode telemetry waveform");
    assert_eq!(telemetry, decoded);
}

#[test]
fn test_trauma_assessment_iss_scoring() {
    let trauma = TraumaAssessment {
        patient_id: 900030,
        mechanism_of_injury: "High-speed MVC, unrestrained driver, prolonged extrication".into(),
        injuries: vec![
            (InjuryRegion::Head, 3),
            (InjuryRegion::Chest, 4),
            (InjuryRegion::Abdomen, 2),
            (InjuryRegion::Extremity, 3),
        ],
        iss_total: 38,
        revised_trauma_score: 10,
        gcs_total: 10,
        systolic_bp: 78,
        respiratory_rate: 28,
        penetrating: false,
        intubated: true,
        notes: "Bilateral chest tubes placed, pelvic binder applied, 2U pRBCs infusing".into(),
    };
    let enc = encode_to_vec(&trauma).expect("encode trauma");
    let compressed = compress_lz4(&enc).expect("compress trauma");
    let decompressed = decompress_lz4(&compressed).expect("decompress trauma");
    let (decoded, _): (TraumaAssessment, usize) =
        decode_from_slice(&decompressed).expect("decode trauma");
    assert_eq!(trauma, decoded);
}

#[test]
fn test_medication_administration_chain() {
    let meds = vec![
        MedicationAdmin {
            patient_id: 900040,
            admin_epoch: 1710500100,
            drug_name: "Epinephrine 1:10,000".into(),
            dose_mg_x100: 100,
            route: MedRoute::Intravenous,
            provider_id: "PM-4421".into(),
            indication: "Cardiac arrest - VFib".into(),
            response: "No change in rhythm".into(),
        },
        MedicationAdmin {
            patient_id: 900040,
            admin_epoch: 1710500340,
            drug_name: "Amiodarone".into(),
            dose_mg_x100: 30000,
            route: MedRoute::Intravenous,
            provider_id: "PM-4421".into(),
            indication: "Refractory VFib after 3rd shock".into(),
            response: "Converted to PEA then ROSC after next epi".into(),
        },
        MedicationAdmin {
            patient_id: 900040,
            admin_epoch: 1710500600,
            drug_name: "Normal Saline".into(),
            dose_mg_x100: 100000,
            route: MedRoute::Intravenous,
            provider_id: "PM-4421".into(),
            indication: "Volume resuscitation post-ROSC hypotension".into(),
            response: "SBP improved 68 -> 94".into(),
        },
        MedicationAdmin {
            patient_id: 900040,
            admin_epoch: 1710500650,
            drug_name: "Fentanyl".into(),
            dose_mg_x100: 10,
            route: MedRoute::Intranasal,
            provider_id: "PM-4421".into(),
            indication: "Post-ROSC pain management, GCS 9".into(),
            response: "Pain response reduced on stimulus".into(),
        },
    ];
    let enc = encode_to_vec(&meds).expect("encode medication chain");
    let compressed = compress_lz4(&enc).expect("compress medication chain");
    let decompressed = decompress_lz4(&compressed).expect("decompress medication chain");
    let (decoded, _): (Vec<MedicationAdmin>, usize) =
        decode_from_slice(&decompressed).expect("decode medication chain");
    assert_eq!(meds, decoded);
}

#[test]
fn test_airway_management_multiple_attempts() {
    let airways = vec![
        AirwayManagement {
            patient_id: 900050,
            attempt_number: 1,
            device: AirwayDevice::EndotrachealOral,
            success: false,
            tube_size_mm_x10: 75,
            depth_cm_x10: 230,
            confirmation_method: "Direct laryngoscopy, Cormack-Lehane grade III".into(),
            pre_ox_spo2: 94,
            post_placement_spo2: 88,
            provider: "Dr. Nakamura".into(),
            complications: vec!["Desaturation to 88%".into(), "Emesis noted".into()],
        },
        AirwayManagement {
            patient_id: 900050,
            attempt_number: 2,
            device: AirwayDevice::EndotrachealOral,
            success: true,
            tube_size_mm_x10: 70,
            depth_cm_x10: 220,
            confirmation_method:
                "Video laryngoscopy, ETCO2 waveform confirmed, bilateral breath sounds".into(),
            pre_ox_spo2: 92,
            post_placement_spo2: 99,
            provider: "Dr. Nakamura".into(),
            complications: vec![],
        },
    ];
    let enc = encode_to_vec(&airways).expect("encode airway");
    let compressed = compress_lz4(&enc).expect("compress airway");
    let decompressed = decompress_lz4(&compressed).expect("decompress airway");
    let (decoded, _): (Vec<AirwayManagement>, usize) =
        decode_from_slice(&decompressed).expect("decode airway");
    assert_eq!(airways, decoded);
}

#[test]
fn test_cardiac_arrest_resuscitation_timeline() {
    let timeline = CardiacArrestTimeline {
        patient_id: 900060,
        witnessed: true,
        bystander_cpr: true,
        initial_rhythm: CardiacRhythm::VentricularFibrillation,
        collapse_epoch: 1710500000,
        first_cpr_epoch: 1710500030,
        first_defib_epoch: Some(1710500120),
        rosc_epoch: Some(1710500540),
        events: vec![
            ResuscitationEvent {
                elapsed_seconds: 0,
                rhythm: CardiacRhythm::VentricularFibrillation,
                action: "EMS arrival, AED pads applied".into(),
                shock_joules: None,
                medication: None,
                compressor_id: "Bystander".into(),
            },
            ResuscitationEvent {
                elapsed_seconds: 120,
                rhythm: CardiacRhythm::VentricularFibrillation,
                action: "Shock delivered".into(),
                shock_joules: Some(200),
                medication: None,
                compressor_id: "FF-112".into(),
            },
            ResuscitationEvent {
                elapsed_seconds: 240,
                rhythm: CardiacRhythm::VentricularFibrillation,
                action: "Shock delivered, epinephrine administered".into(),
                shock_joules: Some(200),
                medication: Some("Epinephrine 1mg IV".into()),
                compressor_id: "FF-113".into(),
            },
            ResuscitationEvent {
                elapsed_seconds: 360,
                rhythm: CardiacRhythm::VentricularFibrillation,
                action: "Shock delivered, amiodarone administered".into(),
                shock_joules: Some(200),
                medication: Some("Amiodarone 300mg IV".into()),
                compressor_id: "FF-112".into(),
            },
            ResuscitationEvent {
                elapsed_seconds: 480,
                rhythm: CardiacRhythm::Pea,
                action: "Rhythm check, PEA noted, CPR continued".into(),
                shock_joules: None,
                medication: Some("Epinephrine 1mg IV".into()),
                compressor_id: "FF-113".into(),
            },
            ResuscitationEvent {
                elapsed_seconds: 540,
                rhythm: CardiacRhythm::Rosc,
                action: "Pulse check positive, ROSC achieved".into(),
                shock_joules: None,
                medication: None,
                compressor_id: "FF-112".into(),
            },
        ],
        total_downtime_seconds: 540,
        outcome: "ROSC achieved, transported to cardiac cath lab".into(),
    };
    let enc = encode_to_vec(&timeline).expect("encode cardiac timeline");
    let compressed = compress_lz4(&enc).expect("compress cardiac timeline");
    let decompressed = decompress_lz4(&compressed).expect("decompress cardiac timeline");
    let (decoded, _): (CardiacArrestTimeline, usize) =
        decode_from_slice(&decompressed).expect("decode cardiac timeline");
    assert_eq!(timeline, decoded);
}

#[test]
fn test_stroke_protocol_activation() {
    let stroke = StrokeProtocol {
        patient_id: 900070,
        activation_epoch: 1710501000,
        last_known_well_epoch: 1710498600,
        nihss_total: 18,
        nihss_items: vec![0, 0, 2, 2, 3, 4, 0, 1, 2, 1, 2, 1, 0, 0, 0],
        facial_droop: true,
        arm_drift: true,
        speech_abnormal: true,
        blood_glucose_mg_dl: 142,
        systolic_bp: 178,
        large_vessel_occlusion_screen_positive: true,
        destination_facility: "Regional Comprehensive Stroke Center - University Hospital".into(),
        tpa_candidate: true,
        thrombectomy_candidate: true,
    };
    let enc = encode_to_vec(&stroke).expect("encode stroke");
    let compressed = compress_lz4(&enc).expect("compress stroke");
    let decompressed = decompress_lz4(&compressed).expect("decompress stroke");
    let (decoded, _): (StrokeProtocol, usize) =
        decode_from_slice(&decompressed).expect("decode stroke");
    assert_eq!(stroke, decoded);
}

#[test]
fn test_mass_casualty_incident_management() {
    let mci = MassCasualtyIncident {
        incident_id: 2024031500099,
        incident_name: "Industrial Explosion - Metro Chemical Plant".into(),
        declaration_epoch: 1710502000,
        location_description: "4200 Industrial Blvd, Sector 7, near loading dock C".into(),
        total_patients: 8,
        red_count: 2,
        yellow_count: 3,
        green_count: 2,
        black_count: 1,
        patients: vec![
            MciPatient {
                triage_tag_number: 1001,
                color: TriageColor::Red,
                breathing: true,
                respiratory_rate: 32,
                perfusion_cap_refill_seconds: 4,
                mental_status_follows_commands: false,
                chief_complaint: "Blast injury, partial thickness burns 40% TBSA, altered LOC"
                    .into(),
            },
            MciPatient {
                triage_tag_number: 1002,
                color: TriageColor::Red,
                breathing: true,
                respiratory_rate: 28,
                perfusion_cap_refill_seconds: 3,
                mental_status_follows_commands: true,
                chief_complaint: "Open femur fracture, significant hemorrhage".into(),
            },
            MciPatient {
                triage_tag_number: 1003,
                color: TriageColor::Yellow,
                breathing: true,
                respiratory_rate: 22,
                perfusion_cap_refill_seconds: 2,
                mental_status_follows_commands: true,
                chief_complaint: "Closed radius/ulna fracture, abrasions".into(),
            },
            MciPatient {
                triage_tag_number: 1004,
                color: TriageColor::Black,
                breathing: false,
                respiratory_rate: 0,
                perfusion_cap_refill_seconds: 0,
                mental_status_follows_commands: false,
                chief_complaint: "Crush injury, no signs of life after airway repositioning".into(),
            },
        ],
        staging_area: "Parking lot D, north end of facility".into(),
        transport_destinations: vec![
            "Level 1 Trauma Center - City General".into(),
            "Burn Center - University Medical".into(),
            "Community Hospital - overflow capacity".into(),
        ],
        command_structure: vec![
            ("Incident Commander".into(), "BC-Williams".into()),
            ("Triage Officer".into(), "PM-Ortiz".into()),
            ("Treatment Officer".into(), "Dr. Chen".into()),
            ("Transport Officer".into(), "LT-Reeves".into()),
        ],
    };
    let enc = encode_to_vec(&mci).expect("encode mci");
    let compressed = compress_lz4(&enc).expect("compress mci");
    let decompressed = decompress_lz4(&compressed).expect("decompress mci");
    let (decoded, _): (MassCasualtyIncident, usize) =
        decode_from_slice(&decompressed).expect("decode mci");
    assert_eq!(mci, decoded);
}

#[test]
fn test_sbar_patient_handoff() {
    let handoff = SbarHandoff {
        patient_id: 900080,
        situation: "72yo male presenting with acute STEMI, currently hemodynamically stable post-PCI notification".into(),
        background: "PMH: HTN, DM2, prior MI 2019. Medications: metoprolol, lisinopril, metformin, aspirin. Allergies: sulfa drugs".into(),
        assessment: "12-lead shows ST elevation V1-V4, reciprocal changes II/III/aVF. Troponin pending. Aspirin 324mg and heparin 5000u administered. Pain 3/10 after NTG x3".into(),
        recommendation: "Activate cath lab, continue heparin drip, serial 12-leads q15min, monitor for reperfusion dysrhythmias".into(),
        vitals_at_handoff: VitalSigns {
            timestamp_epoch: 1710503000,
            heart_rate_bpm: 76,
            systolic_bp: 132,
            diastolic_bp: 78,
            spo2_percent: 97,
            respiratory_rate: 16,
            temperature_c_x10: 369,
            gcs_eye: 4,
            gcs_verbal: 5,
            gcs_motor: 6,
            etco2_mmhg: 40,
            pain_scale_0_10: 3,
        },
        medications_given: vec![
            "Aspirin 324mg PO".into(),
            "Heparin 5000u IV bolus".into(),
            "Nitroglycerin 0.4mg SL x3".into(),
            "Morphine 4mg IV".into(),
        ],
        allergies: vec!["Sulfa drugs".into(), "Codeine".into()],
        receiving_provider: "Dr. Patel, Interventional Cardiology".into(),
        handoff_epoch: 1710503200,
    };
    let enc = encode_to_vec(&handoff).expect("encode sbar");
    let compressed = compress_lz4(&enc).expect("compress sbar");
    let decompressed = decompress_lz4(&compressed).expect("decompress sbar");
    let (decoded, _): (SbarHandoff, usize) = decode_from_slice(&decompressed).expect("decode sbar");
    assert_eq!(handoff, decoded);
}

#[test]
fn test_helicopter_ems_flight_record() {
    let flight = HemsFlightRecord {
        mission_id: 5500123,
        aircraft_tail_number: "N623LF".into(),
        category: FlightCategory::Scene,
        dispatch_epoch: 1710504000,
        liftoff_epoch: 1710504180,
        on_scene_epoch: 1710504900,
        depart_scene_epoch: 1710505500,
        landing_epoch: 1710506400,
        flight_time_minutes: 37,
        pilot: "CPT S. Rodriguez".into(),
        flight_nurse: "RN K. Thompson, CFRN".into(),
        flight_paramedic: "PM J. Walker, FP-C".into(),
        landing_zone_coords_lat: 39_952_583,
        landing_zone_coords_lon: -75_165_222,
        weather_conditions: "VFR, ceiling 4500ft, visibility 8mi, winds 180 at 12kts".into(),
        patient_weight_kg_x10: 820,
    };
    let enc = encode_to_vec(&flight).expect("encode hems");
    let compressed = compress_lz4(&enc).expect("compress hems");
    let decompressed = decompress_lz4(&compressed).expect("decompress hems");
    let (decoded, _): (HemsFlightRecord, usize) =
        decode_from_slice(&decompressed).expect("decode hems");
    assert_eq!(flight, decoded);
}

#[test]
fn test_point_of_care_lab_results() {
    let lab = PocLabResult {
        patient_id: 900090,
        test_epoch: 1710505000,
        glucose_mg_dl: Some(267),
        lactate_mmol_x10: Some(48),
        hemoglobin_g_dl_x10: Some(92),
        potassium_mmol_x100: Some(560),
        sodium_mmol: Some(138),
        troponin_ng_ml_x1000: Some(2450),
        inr_x100: Some(112),
        creatinine_mg_dl_x100: Some(180),
        ph_x1000: Some(7320),
        base_excess_x10: Some(-62),
    };
    let enc = encode_to_vec(&lab).expect("encode poc lab");
    let compressed = compress_lz4(&enc).expect("compress poc lab");
    let decompressed = decompress_lz4(&compressed).expect("decompress poc lab");
    let (decoded, _): (PocLabResult, usize) =
        decode_from_slice(&decompressed).expect("decode poc lab");
    assert_eq!(lab, decoded);
}

#[test]
fn test_pain_assessment_full_evaluation() {
    let pain = PainAssessment {
        patient_id: 900100,
        assessment_epoch: 1710506000,
        numeric_rating_0_10: 9,
        location: "Right lower quadrant abdomen".into(),
        quality: "Sharp, stabbing, constant with intermittent cramping".into(),
        onset: "Acute onset 6 hours ago, worsening progressively".into(),
        duration_minutes: 360,
        aggravating_factors: vec![
            "Movement".into(),
            "Deep breathing".into(),
            "Palpation at McBurney point".into(),
        ],
        alleviating_factors: vec!["Fetal position".into(), "Remaining still".into()],
        radiation: Some("Periumbilical initially, now localized RLQ".into()),
        intervention_given: Some("Morphine 4mg IV".into()),
        reassessment_score: Some(5),
    };
    let enc = encode_to_vec(&pain).expect("encode pain assessment");
    let compressed = compress_lz4(&enc).expect("compress pain assessment");
    let decompressed = decompress_lz4(&compressed).expect("decompress pain assessment");
    let (decoded, _): (PainAssessment, usize) =
        decode_from_slice(&decompressed).expect("decode pain assessment");
    assert_eq!(pain, decoded);
}

#[test]
fn test_ed_boarding_metrics_overcrowding() {
    let metrics = EdBoardingMetrics {
        report_epoch: 1710507000,
        ed_census: 58,
        total_boarding_patients: 14,
        avg_boarding_minutes: 342,
        max_boarding_minutes: 1080,
        admitted_awaiting_bed: 14,
        ambulance_diversion_active: true,
        beds_available: 2,
        beds_total: 45,
        patients_in_waiting_room: 23,
        door_to_provider_avg_minutes: 87,
        left_without_being_seen: 6,
        critical_care_occupancy_pct: 100,
    };
    let enc = encode_to_vec(&metrics).expect("encode ed metrics");
    let compressed = compress_lz4(&enc).expect("compress ed metrics");
    let decompressed = decompress_lz4(&compressed).expect("decompress ed metrics");
    let (decoded, _): (EdBoardingMetrics, usize) =
        decode_from_slice(&decompressed).expect("decode ed metrics");
    assert_eq!(metrics, decoded);
}

#[test]
fn test_disaster_preparedness_inventory() {
    let inventory = DisasterInventory {
        facility_name: "Regional Medical Center - Emergency Preparedness Cache".into(),
        report_epoch: 1710508000,
        items: vec![
            ResourceItem {
                category: ResourceCategory::Ventilator,
                description: "Portable transport ventilator, battery-operated".into(),
                quantity_available: 12,
                quantity_deployed: 3,
                last_inspected_epoch: 1709900000,
                location: "Storage Room B-14".into(),
                serviceable: true,
            },
            ResourceItem {
                category: ResourceCategory::DefibrillatorAed,
                description: "AED with pediatric pads, wall-mounted".into(),
                quantity_available: 25,
                quantity_deployed: 0,
                last_inspected_epoch: 1709800000,
                location: "Various public areas".into(),
                serviceable: true,
            },
            ResourceItem {
                category: ResourceCategory::TourniquetBox,
                description: "CAT Gen7 tourniquet, 10-pack".into(),
                quantity_available: 50,
                quantity_deployed: 5,
                last_inspected_epoch: 1710000000,
                location: "Trauma supply closet".into(),
                serviceable: true,
            },
            ResourceItem {
                category: ResourceCategory::PortableOxygen,
                description: "D-cylinder O2 with regulator, full".into(),
                quantity_available: 30,
                quantity_deployed: 8,
                last_inspected_epoch: 1709700000,
                location: "Ambulance bay storage".into(),
                serviceable: true,
            },
            ResourceItem {
                category: ResourceCategory::PortableUltrasound,
                description: "Handheld POCUS device, cardiac/lung/FAST capable".into(),
                quantity_available: 4,
                quantity_deployed: 2,
                last_inspected_epoch: 1710100000,
                location: "ED equipment room".into(),
                serviceable: true,
            },
        ],
        cache_locations: vec![
            "Building B, Room 14 - Primary cache".into(),
            "Ambulance bay - Secondary cache".into(),
            "Rooftop helipad storage - Tertiary cache".into(),
        ],
        expiring_medications_count: 34,
        total_mre_packs: 500,
        water_gallons: 2000,
        generator_fuel_hours_remaining: 72,
        comm_plan: "Primary: 800MHz trunked radio, Secondary: VHF simplex 155.160, Tertiary: satellite phone".into(),
    };
    let enc = encode_to_vec(&inventory).expect("encode disaster inventory");
    let compressed = compress_lz4(&enc).expect("compress disaster inventory");
    let decompressed = decompress_lz4(&compressed).expect("decompress disaster inventory");
    let (decoded, _): (DisasterInventory, usize) =
        decode_from_slice(&decompressed).expect("decode disaster inventory");
    assert_eq!(inventory, decoded);
}

#[test]
fn test_disaster_inventory_compression_ratio() {
    let large_items: Vec<ResourceItem> = (0..100)
        .map(|i| ResourceItem {
            category: match i % 5 {
                0 => ResourceCategory::Monitor,
                1 => ResourceCategory::InfusionPump,
                2 => ResourceCategory::SuctionUnit,
                3 => ResourceCategory::Stretcher,
                _ => ResourceCategory::WheelChair,
            },
            description: format!(
                "Resource item #{} from disaster cache inventory manifest",
                i
            ),
            quantity_available: 10 + (i % 20),
            quantity_deployed: i % 8,
            last_inspected_epoch: 1710000000 + (i as u64) * 3600,
            location: format!("Cache location sector {}", (i % 12) + 1),
            serviceable: i % 7 != 0,
        })
        .collect();

    let inventory = DisasterInventory {
        facility_name: "Large-scale disaster preparedness warehouse".into(),
        report_epoch: 1710509000,
        items: large_items,
        cache_locations: (0..10)
            .map(|i| format!("Regional cache site {} - distribution point", i))
            .collect(),
        expiring_medications_count: 128,
        total_mre_packs: 10000,
        water_gallons: 50000,
        generator_fuel_hours_remaining: 168,
        comm_plan: "Multi-tier communication plan with redundant systems".into(),
    };
    let enc = encode_to_vec(&inventory).expect("encode large inventory");
    let compressed = compress_lz4(&enc).expect("compress large inventory");
    assert!(
        compressed.len() < enc.len(),
        "LZ4 compressed inventory ({} bytes) should be smaller than raw ({} bytes)",
        compressed.len(),
        enc.len()
    );
    let decompressed = decompress_lz4(&compressed).expect("decompress large inventory");
    let (decoded, _): (DisasterInventory, usize) =
        decode_from_slice(&decompressed).expect("decode large inventory");
    assert_eq!(inventory, decoded);
}

#[test]
fn test_multiple_dispatch_records_compression_benefit() {
    let dispatches: Vec<AmbulanceDispatch> = (0..50)
        .map(|i| AmbulanceDispatch {
            call_id: 2024031500100 + i,
            unit_designator: format!("Medic-{}", (i % 12) + 1),
            priority: match i % 4 {
                0 => DispatchPriority::Priority1LightsAndSirens,
                1 => DispatchPriority::Priority2Expedited,
                2 => DispatchPriority::Priority3Routine,
                _ => DispatchPriority::Priority4Scheduled,
            },
            dispatch_epoch: 1710500000 + i * 300,
            en_route_epoch: 1710500060 + i * 300,
            on_scene_epoch: 1710500420 + i * 300,
            transport_epoch: 1710500900 + i * 300,
            at_hospital_epoch: 1710501600 + i * 300,
            caller_location_lat: 40_712_000 + (i as i32 * 100),
            caller_location_lon: -74_005_000 + (i as i32 * 50),
            incident_type: format!("Medical emergency type code {}", (i % 30) + 1),
            crew_paramedic: format!("Paramedic-{}", (i % 20) + 1),
            crew_emt: format!("EMT-{}", (i % 25) + 1),
        })
        .collect();

    let enc = encode_to_vec(&dispatches).expect("encode dispatch batch");
    let compressed = compress_lz4(&enc).expect("compress dispatch batch");
    assert!(
        compressed.len() < enc.len(),
        "Batch dispatches should compress: {} compressed vs {} raw",
        compressed.len(),
        enc.len()
    );
    let decompressed = decompress_lz4(&compressed).expect("decompress dispatch batch");
    let (decoded, _): (Vec<AmbulanceDispatch>, usize) =
        decode_from_slice(&decompressed).expect("decode dispatch batch");
    assert_eq!(dispatches, decoded);
}

#[test]
fn test_intraosseous_medication_route() {
    let med = MedicationAdmin {
        patient_id: 900110,
        admin_epoch: 1710510000,
        drug_name: "Epinephrine 1:10,000".into(),
        dose_mg_x100: 100,
        route: MedRoute::Intraosseous,
        provider_id: "PM-7702".into(),
        indication: "Cardiac arrest, failed IV access x2, IO placed proximal tibia".into(),
        response: "Drug delivered via IO, flush confirmed".into(),
    };
    let enc = encode_to_vec(&med).expect("encode io med");
    let compressed = compress_lz4(&enc).expect("compress io med");
    let decompressed = decompress_lz4(&compressed).expect("decompress io med");
    let (decoded, _): (MedicationAdmin, usize) =
        decode_from_slice(&decompressed).expect("decode io med");
    assert_eq!(med, decoded);
}

#[test]
fn test_neonatal_hems_transport() {
    let flight = HemsFlightRecord {
        mission_id: 5500200,
        aircraft_tail_number: "N842NE".into(),
        category: FlightCategory::NeonatalTransport,
        dispatch_epoch: 1710511000,
        liftoff_epoch: 1710511300,
        on_scene_epoch: 1710512200,
        depart_scene_epoch: 1710513000,
        landing_epoch: 1710513600,
        flight_time_minutes: 38,
        pilot: "CPT A. Novak".into(),
        flight_nurse: "RN M. Sanchez, NNP-BC".into(),
        flight_paramedic: "RT L. Davis, RRT-NPS".into(),
        landing_zone_coords_lat: 41_878_114,
        landing_zone_coords_lon: -87_629_798,
        weather_conditions:
            "MVFR, ceiling 2500ft, visibility 4mi, light rain, winds 220 at 8kts gusting 15".into(),
        patient_weight_kg_x10: 28,
    };
    let enc = encode_to_vec(&flight).expect("encode neonatal hems");
    let compressed = compress_lz4(&enc).expect("compress neonatal hems");
    let decompressed = decompress_lz4(&compressed).expect("decompress neonatal hems");
    let (decoded, _): (HemsFlightRecord, usize) =
        decode_from_slice(&decompressed).expect("decode neonatal hems");
    assert_eq!(flight, decoded);
}

#[test]
fn test_cardiac_arrest_no_rosc() {
    let timeline = CardiacArrestTimeline {
        patient_id: 900120,
        witnessed: false,
        bystander_cpr: false,
        initial_rhythm: CardiacRhythm::Asystole,
        collapse_epoch: 1710512000,
        first_cpr_epoch: 1710512600,
        first_defib_epoch: None,
        rosc_epoch: None,
        events: vec![
            ResuscitationEvent {
                elapsed_seconds: 0,
                rhythm: CardiacRhythm::Asystole,
                action: "EMS arrival, CPR initiated, monitor shows asystole in multiple leads"
                    .into(),
                shock_joules: None,
                medication: None,
                compressor_id: "PM-3301".into(),
            },
            ResuscitationEvent {
                elapsed_seconds: 120,
                rhythm: CardiacRhythm::Asystole,
                action: "IV access established, epinephrine administered".into(),
                shock_joules: None,
                medication: Some("Epinephrine 1mg IV".into()),
                compressor_id: "FF-201".into(),
            },
            ResuscitationEvent {
                elapsed_seconds: 360,
                rhythm: CardiacRhythm::Asystole,
                action: "Second epinephrine, ETCO2 remains < 10mmHg".into(),
                shock_joules: None,
                medication: Some("Epinephrine 1mg IV".into()),
                compressor_id: "PM-3301".into(),
            },
            ResuscitationEvent {
                elapsed_seconds: 1200,
                rhythm: CardiacRhythm::Asystole,
                action: "Medical command consulted, resuscitation terminated in field".into(),
                shock_joules: None,
                medication: None,
                compressor_id: "PM-3301".into(),
            },
        ],
        total_downtime_seconds: 1800,
        outcome: "Field termination of resuscitation, medical examiner notified".into(),
    };
    let enc = encode_to_vec(&timeline).expect("encode no-rosc timeline");
    let compressed = compress_lz4(&enc).expect("compress no-rosc timeline");
    let decompressed = decompress_lz4(&compressed).expect("decompress no-rosc timeline");
    let (decoded, _): (CardiacArrestTimeline, usize) =
        decode_from_slice(&decompressed).expect("decode no-rosc timeline");
    assert_eq!(timeline, decoded);
}
