//! Advanced tests for healthcare wearables and remote patient monitoring domain types.
//! 22 test functions covering complex enums, nested enums, and struct/enum compositions.

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
// Domain types
// ---------------------------------------------------------------------------

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
enum VitalSignReading {
    HeartRate {
        bpm: u16,
        confidence: u8,
    },
    SpO2 {
        percentage: u8,
        perfusion_index: u16,
    },
    BloodPressure {
        systolic: u16,
        diastolic: u16,
        mean_arterial: u16,
    },
    Temperature {
        celsius_x100: i32,
        site: TemperatureSite,
    },
    RespiratoryRate {
        breaths_per_min: u8,
        regularity: BreathRegularity,
    },
    SkinConductance {
        micro_siemens_x100: u32,
    },
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
enum TemperatureSite {
    Oral,
    Axillary,
    Tympanic,
    Temporal,
    Rectal,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
enum BreathRegularity {
    Regular,
    Irregular,
    CheyneStokes,
    Apneic { duration_secs: u16 },
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
enum EcgSegment {
    Normal {
        rr_interval_ms: u16,
        qrs_duration_ms: u16,
    },
    PrematureVentricular {
        coupling_interval_ms: u16,
    },
    PrematureAtrial {
        p_wave_morphology: PwaveMorphology,
    },
    AtrialFibrillation {
        ventricular_rate: u16,
        irregularity_index: u8,
    },
    Artifact {
        samples_affected: u32,
    },
    Pause {
        duration_ms: u32,
    },
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
enum PwaveMorphology {
    Normal,
    Inverted,
    Biphasic,
    Absent,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
enum SleepStage {
    Awake,
    NremLight {
        spindle_count: u16,
    },
    NremDeep {
        delta_power_x100: u32,
    },
    Rem {
        eye_movement_density: u8,
        atonia_index: u8,
    },
    Transition {
        from: Box<SleepStage>,
        to: Box<SleepStage>,
    },
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
enum ActivityType {
    Walking {
        steps_per_min: u16,
        stride_length_cm: u16,
    },
    Running {
        cadence: u16,
        ground_contact_ms: u16,
        vertical_oscillation_mm: u16,
    },
    Cycling {
        rpm: u16,
        power_watts: Option<u16>,
    },
    Swimming {
        stroke: SwimStroke,
        laps: u16,
        swolf: u16,
    },
    Stationary,
    Unknown {
        accelerometer_magnitude: u32,
    },
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
enum SwimStroke {
    Freestyle,
    Backstroke,
    Breaststroke,
    Butterfly,
    Mixed,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
enum FallDetectionEvent {
    FreeFall {
        duration_ms: u32,
    },
    Impact {
        g_force_x100: u32,
        direction: ImpactDirection,
    },
    PostFallImmobility {
        immobile_secs: u32,
        vitals: Option<Box<VitalSignReading>>,
    },
    FalsePositive {
        trigger_reason: String,
    },
    Confirmed {
        timestamp_epoch: u64,
        location: Option<GpsCoord>,
        responder_notified: bool,
    },
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
enum ImpactDirection {
    Forward,
    Backward,
    Lateral,
    Downward,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct GpsCoord {
    lat_x1e7: i64,
    lon_x1e7: i64,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
enum MedicationAdherenceAlert {
    Taken {
        med_id: u32,
        scheduled_epoch: u64,
        actual_epoch: u64,
    },
    Missed {
        med_id: u32,
        scheduled_epoch: u64,
        escalation: AlertEscalation,
    },
    LateButTaken {
        med_id: u32,
        delay_minutes: u32,
    },
    Skipped {
        med_id: u32,
        reason: Option<String>,
    },
    Refill {
        med_id: u32,
        remaining_doses: u8,
    },
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
enum AlertEscalation {
    Patient,
    Caregiver,
    Clinician,
    Emergency,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
enum GlucoseReading {
    InRange {
        mg_dl: u16,
        trend: GlucoseTrend,
    },
    BelowRange {
        mg_dl: u16,
        trend: GlucoseTrend,
        urgent: bool,
    },
    AboveRange {
        mg_dl: u16,
        trend: GlucoseTrend,
        urgent: bool,
    },
    CalibrationNeeded {
        hours_since_last: u16,
    },
    SensorError {
        code: u16,
        message: String,
    },
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
enum GlucoseTrend {
    RisingRapidly,
    Rising,
    Stable,
    Falling,
    FallingRapidly,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
enum StressLevel {
    Low {
        hrv_ms: u16,
    },
    Moderate {
        hrv_ms: u16,
        skin_conductance: u32,
    },
    High {
        hrv_ms: u16,
        skin_conductance: u32,
        cortisol_proxy: u8,
    },
    Acute {
        source: StressSource,
        readings: Vec<VitalSignReading>,
    },
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
enum StressSource {
    Physical,
    Cognitive,
    Emotional,
    Environmental,
    Unknown,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
enum BloodOxygenTrend {
    Stable {
        avg_spo2: u8,
        samples: u16,
    },
    Declining {
        start_spo2: u8,
        end_spo2: u8,
        duration_secs: u32,
    },
    Recovering {
        nadir_spo2: u8,
        current_spo2: u8,
    },
    Desaturation {
        events: Vec<DesatEvent>,
    },
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct DesatEvent {
    nadir_spo2: u8,
    duration_secs: u16,
    associated_apnea: bool,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
enum ArrhythmiaDetection {
    None,
    Bradycardia {
        bpm: u16,
    },
    Tachycardia {
        bpm: u16,
        sustained: bool,
    },
    AtrialFibrillation {
        burden_percent: u8,
        episodes: Vec<AfibEpisode>,
    },
    VentricularEctopy {
        pvc_per_hour: u16,
        couplets: u16,
        runs: u16,
    },
    HeartBlock {
        degree: HeartBlockDegree,
    },
    LongQt {
        qtc_ms: u16,
    },
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct AfibEpisode {
    start_epoch: u64,
    duration_secs: u32,
    avg_ventricular_rate: u16,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
enum HeartBlockDegree {
    First { pr_interval_ms: u16 },
    SecondTypeI,
    SecondTypeII,
    Third,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
enum DeviceStatus {
    Normal {
        battery_percent: u8,
        signal_strength_dbm: i8,
    },
    LowBattery {
        battery_percent: u8,
        est_hours_remaining: u16,
    },
    Disconnected {
        last_seen_epoch: u64,
        reason: DisconnectReason,
    },
    Charging {
        battery_percent: u8,
        charge_rate: u8,
    },
    FirmwareUpdate {
        version: String,
        progress_percent: u8,
    },
    Error {
        code: u16,
        description: String,
        recoverable: bool,
    },
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
enum DisconnectReason {
    OutOfRange,
    BluetoothOff,
    DevicePoweredDown,
    Interference,
    Unknown,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
enum ClinicalTrialDataPoint {
    Baseline {
        visit_id: u32,
        vitals: Vec<VitalSignReading>,
        notes: Option<String>,
    },
    FollowUp {
        visit_id: u32,
        week: u16,
        primary_endpoint: f64,
        secondary_endpoints: Vec<f64>,
    },
    AdverseEvent {
        severity: AdverseEventSeverity,
        description: String,
        related_to_treatment: Option<bool>,
    },
    Withdrawal {
        reason: String,
        visit_id: u32,
    },
    ProtocolDeviation {
        description: String,
        major: bool,
    },
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
struct PatientMonitoringSession {
    patient_id: u64,
    device: DeviceStatus,
    vitals: Vec<VitalSignReading>,
    glucose: Option<GlucoseReading>,
    ecg_segments: Vec<EcgSegment>,
    sleep: Option<SleepStage>,
    activity: Option<ActivityType>,
    arrhythmia: ArrhythmiaDetection,
    alerts: Vec<MedicationAdherenceAlert>,
}

// ---------------------------------------------------------------------------
// Tests
// ---------------------------------------------------------------------------

#[test]
fn test_vital_sign_heart_rate_roundtrip() {
    let val = VitalSignReading::HeartRate {
        bpm: 72,
        confidence: 95,
    };
    let bytes = encode_to_vec(&val).expect("encode HeartRate");
    let (decoded, _): (VitalSignReading, usize) =
        decode_from_slice(&bytes).expect("decode HeartRate");
    assert_eq!(val, decoded);
}

#[test]
fn test_vital_sign_blood_pressure_roundtrip() {
    let val = VitalSignReading::BloodPressure {
        systolic: 120,
        diastolic: 80,
        mean_arterial: 93,
    };
    let bytes = encode_to_vec(&val).expect("encode BloodPressure");
    let (decoded, _): (VitalSignReading, usize) =
        decode_from_slice(&bytes).expect("decode BloodPressure");
    assert_eq!(val, decoded);
}

#[test]
fn test_vital_sign_temperature_with_nested_site_roundtrip() {
    let val = VitalSignReading::Temperature {
        celsius_x100: 3698,
        site: TemperatureSite::Tympanic,
    };
    let bytes = encode_to_vec(&val).expect("encode Temperature");
    let (decoded, _): (VitalSignReading, usize) =
        decode_from_slice(&bytes).expect("decode Temperature");
    assert_eq!(val, decoded);
}

#[test]
fn test_respiratory_rate_with_apneic_breath_roundtrip() {
    let val = VitalSignReading::RespiratoryRate {
        breaths_per_min: 8,
        regularity: BreathRegularity::Apneic { duration_secs: 22 },
    };
    let bytes = encode_to_vec(&val).expect("encode RespiratoryRate apneic");
    let (decoded, _): (VitalSignReading, usize) =
        decode_from_slice(&bytes).expect("decode RespiratoryRate apneic");
    assert_eq!(val, decoded);
}

#[test]
fn test_ecg_atrial_fibrillation_segment_roundtrip() {
    let val = EcgSegment::AtrialFibrillation {
        ventricular_rate: 142,
        irregularity_index: 87,
    };
    let bytes = encode_to_vec(&val).expect("encode EcgSegment AFib");
    let (decoded, _): (EcgSegment, usize) =
        decode_from_slice(&bytes).expect("decode EcgSegment AFib");
    assert_eq!(val, decoded);
}

#[test]
fn test_ecg_premature_atrial_with_nested_morphology_roundtrip() {
    let val = EcgSegment::PrematureAtrial {
        p_wave_morphology: PwaveMorphology::Biphasic,
    };
    let bytes = encode_to_vec(&val).expect("encode PrematureAtrial");
    let (decoded, _): (EcgSegment, usize) =
        decode_from_slice(&bytes).expect("decode PrematureAtrial");
    assert_eq!(val, decoded);
}

#[test]
fn test_sleep_stage_transition_with_boxed_enums_roundtrip() {
    let val = SleepStage::Transition {
        from: Box::new(SleepStage::NremDeep {
            delta_power_x100: 8500,
        }),
        to: Box::new(SleepStage::Rem {
            eye_movement_density: 64,
            atonia_index: 92,
        }),
    };
    let bytes = encode_to_vec(&val).expect("encode SleepStage Transition");
    let (decoded, _): (SleepStage, usize) =
        decode_from_slice(&bytes).expect("decode SleepStage Transition");
    assert_eq!(val, decoded);
}

#[test]
fn test_activity_swimming_with_nested_stroke_roundtrip() {
    let val = ActivityType::Swimming {
        stroke: SwimStroke::Butterfly,
        laps: 20,
        swolf: 42,
    };
    let bytes = encode_to_vec(&val).expect("encode Swimming");
    let (decoded, _): (ActivityType, usize) = decode_from_slice(&bytes).expect("decode Swimming");
    assert_eq!(val, decoded);
}

#[test]
fn test_activity_cycling_with_optional_power_roundtrip() {
    let val = ActivityType::Cycling {
        rpm: 85,
        power_watts: Some(245),
    };
    let bytes = encode_to_vec(&val).expect("encode Cycling with power");
    let (decoded, _): (ActivityType, usize) =
        decode_from_slice(&bytes).expect("decode Cycling with power");
    assert_eq!(val, decoded);

    let val_none = ActivityType::Cycling {
        rpm: 60,
        power_watts: None,
    };
    let bytes2 = encode_to_vec(&val_none).expect("encode Cycling no power");
    let (decoded2, _): (ActivityType, usize) =
        decode_from_slice(&bytes2).expect("decode Cycling no power");
    assert_eq!(val_none, decoded2);
}

#[test]
fn test_fall_detection_confirmed_with_optional_gps_roundtrip() {
    let val = FallDetectionEvent::Confirmed {
        timestamp_epoch: 1_700_000_000,
        location: Some(GpsCoord {
            lat_x1e7: 408_500_000,
            lon_x1e7: -739_500_000,
        }),
        responder_notified: true,
    };
    let bytes = encode_to_vec(&val).expect("encode FallDetection Confirmed");
    let (decoded, _): (FallDetectionEvent, usize) =
        decode_from_slice(&bytes).expect("decode FallDetection Confirmed");
    assert_eq!(val, decoded);
}

#[test]
fn test_fall_detection_post_fall_with_nested_vitals_roundtrip() {
    let val = FallDetectionEvent::PostFallImmobility {
        immobile_secs: 120,
        vitals: Some(Box::new(VitalSignReading::HeartRate {
            bpm: 110,
            confidence: 60,
        })),
    };
    let bytes = encode_to_vec(&val).expect("encode PostFallImmobility");
    let (decoded, _): (FallDetectionEvent, usize) =
        decode_from_slice(&bytes).expect("decode PostFallImmobility");
    assert_eq!(val, decoded);
}

#[test]
fn test_medication_missed_with_escalation_roundtrip() {
    let val = MedicationAdherenceAlert::Missed {
        med_id: 42,
        scheduled_epoch: 1_700_100_000,
        escalation: AlertEscalation::Clinician,
    };
    let bytes = encode_to_vec(&val).expect("encode MedicationMissed");
    let (decoded, _): (MedicationAdherenceAlert, usize) =
        decode_from_slice(&bytes).expect("decode MedicationMissed");
    assert_eq!(val, decoded);
}

#[test]
fn test_medication_skipped_with_optional_reason_roundtrip() {
    let val = MedicationAdherenceAlert::Skipped {
        med_id: 7,
        reason: Some("Nausea side effect".to_string()),
    };
    let bytes = encode_to_vec(&val).expect("encode MedicationSkipped");
    let (decoded, _): (MedicationAdherenceAlert, usize) =
        decode_from_slice(&bytes).expect("decode MedicationSkipped");
    assert_eq!(val, decoded);
}

#[test]
fn test_glucose_below_range_urgent_roundtrip() {
    let val = GlucoseReading::BelowRange {
        mg_dl: 54,
        trend: GlucoseTrend::FallingRapidly,
        urgent: true,
    };
    let bytes = encode_to_vec(&val).expect("encode Glucose BelowRange");
    let (decoded, _): (GlucoseReading, usize) =
        decode_from_slice(&bytes).expect("decode Glucose BelowRange");
    assert_eq!(val, decoded);
}

#[test]
fn test_glucose_sensor_error_with_string_roundtrip() {
    let val = GlucoseReading::SensorError {
        code: 0x0E03,
        message: "Calibration drift detected".to_string(),
    };
    let bytes = encode_to_vec(&val).expect("encode Glucose SensorError");
    let (decoded, _): (GlucoseReading, usize) =
        decode_from_slice(&bytes).expect("decode Glucose SensorError");
    assert_eq!(val, decoded);
}

#[test]
fn test_stress_acute_with_vec_vitals_roundtrip() {
    let val = StressLevel::Acute {
        source: StressSource::Cognitive,
        readings: vec![
            VitalSignReading::HeartRate {
                bpm: 105,
                confidence: 88,
            },
            VitalSignReading::SkinConductance {
                micro_siemens_x100: 1250,
            },
            VitalSignReading::RespiratoryRate {
                breaths_per_min: 22,
                regularity: BreathRegularity::Irregular,
            },
        ],
    };
    let bytes = encode_to_vec(&val).expect("encode StressLevel Acute");
    let (decoded, _): (StressLevel, usize) =
        decode_from_slice(&bytes).expect("decode StressLevel Acute");
    assert_eq!(val, decoded);
}

#[test]
fn test_blood_oxygen_desaturation_with_vec_events_roundtrip() {
    let val = BloodOxygenTrend::Desaturation {
        events: vec![
            DesatEvent {
                nadir_spo2: 84,
                duration_secs: 18,
                associated_apnea: true,
            },
            DesatEvent {
                nadir_spo2: 88,
                duration_secs: 12,
                associated_apnea: false,
            },
            DesatEvent {
                nadir_spo2: 82,
                duration_secs: 25,
                associated_apnea: true,
            },
        ],
    };
    let bytes = encode_to_vec(&val).expect("encode Desaturation");
    let (decoded, _): (BloodOxygenTrend, usize) =
        decode_from_slice(&bytes).expect("decode Desaturation");
    assert_eq!(val, decoded);
}

#[test]
fn test_arrhythmia_afib_with_episode_vec_roundtrip() {
    let val = ArrhythmiaDetection::AtrialFibrillation {
        burden_percent: 23,
        episodes: vec![
            AfibEpisode {
                start_epoch: 1_700_050_000,
                duration_secs: 3600,
                avg_ventricular_rate: 138,
            },
            AfibEpisode {
                start_epoch: 1_700_080_000,
                duration_secs: 900,
                avg_ventricular_rate: 112,
            },
        ],
    };
    let bytes = encode_to_vec(&val).expect("encode AFib detection");
    let (decoded, _): (ArrhythmiaDetection, usize) =
        decode_from_slice(&bytes).expect("decode AFib detection");
    assert_eq!(val, decoded);
}

#[test]
fn test_arrhythmia_heart_block_nested_degree_roundtrip() {
    let val = ArrhythmiaDetection::HeartBlock {
        degree: HeartBlockDegree::First {
            pr_interval_ms: 240,
        },
    };
    let bytes = encode_to_vec(&val).expect("encode HeartBlock");
    let (decoded, _): (ArrhythmiaDetection, usize) =
        decode_from_slice(&bytes).expect("decode HeartBlock");
    assert_eq!(val, decoded);
}

#[test]
fn test_device_status_disconnected_with_nested_reason_roundtrip() {
    let val = DeviceStatus::Disconnected {
        last_seen_epoch: 1_700_099_000,
        reason: DisconnectReason::Interference,
    };
    let bytes = encode_to_vec(&val).expect("encode DeviceStatus Disconnected");
    let (decoded, _): (DeviceStatus, usize) =
        decode_from_slice(&bytes).expect("decode DeviceStatus Disconnected");
    assert_eq!(val, decoded);
}

#[test]
fn test_clinical_trial_baseline_with_vec_vitals_and_optional_notes_roundtrip() {
    let val = ClinicalTrialDataPoint::Baseline {
        visit_id: 1001,
        vitals: vec![
            VitalSignReading::HeartRate {
                bpm: 68,
                confidence: 99,
            },
            VitalSignReading::BloodPressure {
                systolic: 118,
                diastolic: 76,
                mean_arterial: 90,
            },
            VitalSignReading::SpO2 {
                percentage: 98,
                perfusion_index: 350,
            },
        ],
        notes: Some("Patient fasting for 12 hours".to_string()),
    };
    let bytes = encode_to_vec(&val).expect("encode ClinicalTrial Baseline");
    let (decoded, _): (ClinicalTrialDataPoint, usize) =
        decode_from_slice(&bytes).expect("decode ClinicalTrial Baseline");
    assert_eq!(val, decoded);
}

#[test]
fn test_full_patient_monitoring_session_roundtrip() {
    let val = PatientMonitoringSession {
        patient_id: 900_123,
        device: DeviceStatus::Normal {
            battery_percent: 74,
            signal_strength_dbm: -42,
        },
        vitals: vec![
            VitalSignReading::HeartRate {
                bpm: 78,
                confidence: 97,
            },
            VitalSignReading::SpO2 {
                percentage: 96,
                perfusion_index: 280,
            },
            VitalSignReading::Temperature {
                celsius_x100: 3710,
                site: TemperatureSite::Temporal,
            },
        ],
        glucose: Some(GlucoseReading::InRange {
            mg_dl: 105,
            trend: GlucoseTrend::Stable,
        }),
        ecg_segments: vec![
            EcgSegment::Normal {
                rr_interval_ms: 770,
                qrs_duration_ms: 88,
            },
            EcgSegment::Normal {
                rr_interval_ms: 785,
                qrs_duration_ms: 90,
            },
            EcgSegment::PrematureVentricular {
                coupling_interval_ms: 420,
            },
        ],
        sleep: Some(SleepStage::NremDeep {
            delta_power_x100: 7200,
        }),
        activity: None,
        arrhythmia: ArrhythmiaDetection::VentricularEctopy {
            pvc_per_hour: 12,
            couplets: 2,
            runs: 0,
        },
        alerts: vec![
            MedicationAdherenceAlert::Taken {
                med_id: 10,
                scheduled_epoch: 1_700_060_000,
                actual_epoch: 1_700_060_300,
            },
            MedicationAdherenceAlert::Refill {
                med_id: 10,
                remaining_doses: 3,
            },
        ],
    };
    let bytes = encode_to_vec(&val).expect("encode full PatientMonitoringSession");
    let (decoded, _): (PatientMonitoringSession, usize) =
        decode_from_slice(&bytes).expect("decode full PatientMonitoringSession");
    assert_eq!(val, decoded);
}
