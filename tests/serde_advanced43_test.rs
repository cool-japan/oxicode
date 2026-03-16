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

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
enum ConsultationStatus {
    Scheduled,
    InProgress,
    Completed,
    Cancelled,
    NoShow,
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
enum Severity {
    Mild,
    Moderate,
    Severe,
    Critical,
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
enum TrialPhase {
    Phase1,
    Phase2,
    Phase3,
    Phase4,
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
enum ConsentStatus {
    Pending,
    Granted,
    Withdrawn,
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
struct VitalSign {
    patient_id: u64,
    timestamp: u64,
    heart_rate_bpm: u16,
    systolic_bp: u16,
    diastolic_bp: u16,
    spo2_pct: u8,
    temp_c_x100: u16,
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
struct Consultation {
    consult_id: u64,
    patient_id: u64,
    provider_id: u64,
    status: ConsultationStatus,
    start_time: u64,
    duration_min: Option<u16>,
    diagnosis_codes: Vec<String>,
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
struct Prescription {
    rx_id: u64,
    patient_id: u64,
    drug_code: String,
    dosage_mg: u32,
    frequency_daily: u8,
    duration_days: u16,
    issued_at: u64,
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
struct TrialEnrollment {
    enrollment_id: u64,
    trial_id: u32,
    patient_id: u64,
    phase: TrialPhase,
    consent: ConsentStatus,
    enrolled_date: u64,
    arm_id: u8,
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
struct AdverseEvent {
    event_id: u64,
    trial_id: u32,
    patient_id: u64,
    severity: Severity,
    onset_date: u64,
    resolved_date: Option<u64>,
    description: String,
}

// --- VitalSign tests ---

#[test]
fn test_vital_sign_standard_roundtrip() {
    let vital = VitalSign {
        patient_id: 1001,
        timestamp: 1_700_000_000,
        heart_rate_bpm: 72,
        systolic_bp: 120,
        diastolic_bp: 80,
        spo2_pct: 98,
        temp_c_x100: 3700,
    };
    let cfg = config::standard();
    let encoded = encode_to_vec(&vital, cfg).expect("encode VitalSign standard");
    let (decoded, consumed): (VitalSign, usize) =
        decode_owned_from_slice(&encoded, cfg).expect("decode VitalSign standard");
    assert_eq!(vital, decoded);
    assert_eq!(consumed, encoded.len());
}

#[test]
fn test_vital_sign_big_endian_roundtrip() {
    let vital = VitalSign {
        patient_id: 2002,
        timestamp: 1_700_001_000,
        heart_rate_bpm: 88,
        systolic_bp: 140,
        diastolic_bp: 90,
        spo2_pct: 95,
        temp_c_x100: 3850,
    };
    let cfg = config::standard().with_big_endian();
    let encoded = encode_to_vec(&vital, cfg).expect("encode VitalSign big endian");
    let (decoded, consumed): (VitalSign, usize) =
        decode_owned_from_slice(&encoded, cfg).expect("decode VitalSign big endian");
    assert_eq!(vital, decoded);
    assert_eq!(consumed, encoded.len());
}

#[test]
fn test_vital_sign_fixed_int_roundtrip() {
    let vital = VitalSign {
        patient_id: 3003,
        timestamp: 1_700_002_000,
        heart_rate_bpm: 60,
        systolic_bp: 110,
        diastolic_bp: 70,
        spo2_pct: 99,
        temp_c_x100: 3680,
    };
    let cfg = config::standard().with_fixed_int_encoding();
    let encoded = encode_to_vec(&vital, cfg).expect("encode VitalSign fixed int");
    let (decoded, consumed): (VitalSign, usize) =
        decode_owned_from_slice(&encoded, cfg).expect("decode VitalSign fixed int");
    assert_eq!(vital, decoded);
    assert_eq!(consumed, encoded.len());
}

// --- Consultation tests ---

#[test]
fn test_consultation_completed_with_duration() {
    let consult = Consultation {
        consult_id: 5001,
        patient_id: 1001,
        provider_id: 9001,
        status: ConsultationStatus::Completed,
        start_time: 1_700_010_000,
        duration_min: Some(45),
        diagnosis_codes: vec!["Z00.00".to_string(), "I10".to_string()],
    };
    let cfg = config::standard();
    let encoded = encode_to_vec(&consult, cfg).expect("encode Consultation completed");
    let (decoded, consumed): (Consultation, usize) =
        decode_owned_from_slice(&encoded, cfg).expect("decode Consultation completed");
    assert_eq!(consult, decoded);
    assert_eq!(consumed, encoded.len());
}

#[test]
fn test_consultation_scheduled_no_duration() {
    let consult = Consultation {
        consult_id: 5002,
        patient_id: 2002,
        provider_id: 9002,
        status: ConsultationStatus::Scheduled,
        start_time: 1_700_020_000,
        duration_min: None,
        diagnosis_codes: vec![],
    };
    let cfg = config::standard();
    let encoded = encode_to_vec(&consult, cfg).expect("encode Consultation scheduled");
    let (decoded, consumed): (Consultation, usize) =
        decode_owned_from_slice(&encoded, cfg).expect("decode Consultation scheduled");
    assert_eq!(consult, decoded);
    assert_eq!(consumed, encoded.len());
    assert_eq!(decoded.duration_min, None);
    assert!(decoded.diagnosis_codes.is_empty());
}

#[test]
fn test_consultation_inprogress_big_endian() {
    let consult = Consultation {
        consult_id: 5003,
        patient_id: 3003,
        provider_id: 9003,
        status: ConsultationStatus::InProgress,
        start_time: 1_700_030_000,
        duration_min: None,
        diagnosis_codes: vec!["J06.9".to_string()],
    };
    let cfg = config::standard().with_big_endian();
    let encoded = encode_to_vec(&consult, cfg).expect("encode Consultation in-progress big endian");
    let (decoded, consumed): (Consultation, usize) =
        decode_owned_from_slice(&encoded, cfg).expect("decode Consultation in-progress big endian");
    assert_eq!(consult, decoded);
    assert_eq!(consumed, encoded.len());
}

#[test]
fn test_consultation_noshow_fixed_int() {
    let consult = Consultation {
        consult_id: 5004,
        patient_id: 4004,
        provider_id: 9004,
        status: ConsultationStatus::NoShow,
        start_time: 1_700_040_000,
        duration_min: None,
        diagnosis_codes: vec![],
    };
    let cfg = config::standard().with_fixed_int_encoding();
    let encoded = encode_to_vec(&consult, cfg).expect("encode Consultation no-show fixed int");
    let (decoded, consumed): (Consultation, usize) =
        decode_owned_from_slice(&encoded, cfg).expect("decode Consultation no-show fixed int");
    assert_eq!(consult, decoded);
    assert_eq!(consumed, encoded.len());
    assert_eq!(decoded.status, ConsultationStatus::NoShow);
}

#[test]
fn test_consultation_cancelled_multiple_diagnosis() {
    let consult = Consultation {
        consult_id: 5005,
        patient_id: 5005,
        provider_id: 9005,
        status: ConsultationStatus::Cancelled,
        start_time: 1_700_050_000,
        duration_min: Some(0),
        diagnosis_codes: vec![
            "E11.9".to_string(),
            "I25.10".to_string(),
            "N18.3".to_string(),
        ],
    };
    let cfg = config::standard();
    let encoded = encode_to_vec(&consult, cfg).expect("encode Consultation cancelled multi-diag");
    let (decoded, consumed): (Consultation, usize) =
        decode_owned_from_slice(&encoded, cfg).expect("decode Consultation cancelled multi-diag");
    assert_eq!(consult, decoded);
    assert_eq!(consumed, encoded.len());
    assert_eq!(decoded.diagnosis_codes.len(), 3);
}

// --- Prescription tests ---

#[test]
fn test_prescription_standard_roundtrip() {
    let rx = Prescription {
        rx_id: 8001,
        patient_id: 1001,
        drug_code: "MET500".to_string(),
        dosage_mg: 500,
        frequency_daily: 2,
        duration_days: 30,
        issued_at: 1_700_060_000,
    };
    let cfg = config::standard();
    let encoded = encode_to_vec(&rx, cfg).expect("encode Prescription standard");
    let (decoded, consumed): (Prescription, usize) =
        decode_owned_from_slice(&encoded, cfg).expect("decode Prescription standard");
    assert_eq!(rx, decoded);
    assert_eq!(consumed, encoded.len());
}

#[test]
fn test_prescription_big_endian_roundtrip() {
    let rx = Prescription {
        rx_id: 8002,
        patient_id: 2002,
        drug_code: "AMO250".to_string(),
        dosage_mg: 250,
        frequency_daily: 3,
        duration_days: 7,
        issued_at: 1_700_070_000,
    };
    let cfg = config::standard().with_big_endian();
    let encoded = encode_to_vec(&rx, cfg).expect("encode Prescription big endian");
    let (decoded, consumed): (Prescription, usize) =
        decode_owned_from_slice(&encoded, cfg).expect("decode Prescription big endian");
    assert_eq!(rx, decoded);
    assert_eq!(consumed, encoded.len());
}

#[test]
fn test_prescription_fixed_int_roundtrip() {
    let rx = Prescription {
        rx_id: 8003,
        patient_id: 3003,
        drug_code: "LIS10".to_string(),
        dosage_mg: 10,
        frequency_daily: 1,
        duration_days: 90,
        issued_at: 1_700_080_000,
    };
    let cfg = config::standard().with_fixed_int_encoding();
    let encoded = encode_to_vec(&rx, cfg).expect("encode Prescription fixed int");
    let (decoded, consumed): (Prescription, usize) =
        decode_owned_from_slice(&encoded, cfg).expect("decode Prescription fixed int");
    assert_eq!(rx, decoded);
    assert_eq!(consumed, encoded.len());
    assert_eq!(decoded.drug_code, "LIS10");
}

// --- TrialEnrollment tests ---

#[test]
fn test_trial_enrollment_phase2_granted() {
    let enrollment = TrialEnrollment {
        enrollment_id: 7001,
        trial_id: 101,
        patient_id: 1001,
        phase: TrialPhase::Phase2,
        consent: ConsentStatus::Granted,
        enrolled_date: 1_700_090_000,
        arm_id: 1,
    };
    let cfg = config::standard();
    let encoded = encode_to_vec(&enrollment, cfg).expect("encode TrialEnrollment phase2 granted");
    let (decoded, consumed): (TrialEnrollment, usize) =
        decode_owned_from_slice(&encoded, cfg).expect("decode TrialEnrollment phase2 granted");
    assert_eq!(enrollment, decoded);
    assert_eq!(consumed, encoded.len());
    assert_eq!(decoded.phase, TrialPhase::Phase2);
    assert_eq!(decoded.consent, ConsentStatus::Granted);
}

#[test]
fn test_trial_enrollment_phase3_withdrawn_big_endian() {
    let enrollment = TrialEnrollment {
        enrollment_id: 7002,
        trial_id: 202,
        patient_id: 2002,
        phase: TrialPhase::Phase3,
        consent: ConsentStatus::Withdrawn,
        enrolled_date: 1_700_100_000,
        arm_id: 2,
    };
    let cfg = config::standard().with_big_endian();
    let encoded = encode_to_vec(&enrollment, cfg)
        .expect("encode TrialEnrollment phase3 withdrawn big endian");
    let (decoded, consumed): (TrialEnrollment, usize) = decode_owned_from_slice(&encoded, cfg)
        .expect("decode TrialEnrollment phase3 withdrawn big endian");
    assert_eq!(enrollment, decoded);
    assert_eq!(consumed, encoded.len());
}

#[test]
fn test_trial_enrollment_phase1_pending_fixed_int() {
    let enrollment = TrialEnrollment {
        enrollment_id: 7003,
        trial_id: 303,
        patient_id: 3003,
        phase: TrialPhase::Phase1,
        consent: ConsentStatus::Pending,
        enrolled_date: 1_700_110_000,
        arm_id: 0,
    };
    let cfg = config::standard().with_fixed_int_encoding();
    let encoded =
        encode_to_vec(&enrollment, cfg).expect("encode TrialEnrollment phase1 pending fixed int");
    let (decoded, consumed): (TrialEnrollment, usize) = decode_owned_from_slice(&encoded, cfg)
        .expect("decode TrialEnrollment phase1 pending fixed int");
    assert_eq!(enrollment, decoded);
    assert_eq!(consumed, encoded.len());
    assert_eq!(decoded.consent, ConsentStatus::Pending);
}

#[test]
fn test_trial_enrollment_phase4_standard() {
    let enrollment = TrialEnrollment {
        enrollment_id: 7004,
        trial_id: 404,
        patient_id: 4004,
        phase: TrialPhase::Phase4,
        consent: ConsentStatus::Granted,
        enrolled_date: 1_700_120_000,
        arm_id: 3,
    };
    let cfg = config::standard();
    let encoded = encode_to_vec(&enrollment, cfg).expect("encode TrialEnrollment phase4 standard");
    let (decoded, consumed): (TrialEnrollment, usize) =
        decode_owned_from_slice(&encoded, cfg).expect("decode TrialEnrollment phase4 standard");
    assert_eq!(enrollment, decoded);
    assert_eq!(consumed, encoded.len());
    assert_eq!(decoded.phase, TrialPhase::Phase4);
}

// --- AdverseEvent tests ---

#[test]
fn test_adverse_event_resolved_standard() {
    let event = AdverseEvent {
        event_id: 6001,
        trial_id: 101,
        patient_id: 1001,
        severity: Severity::Mild,
        onset_date: 1_700_130_000,
        resolved_date: Some(1_700_216_400),
        description: "Mild nausea following first dose administration.".to_string(),
    };
    let cfg = config::standard();
    let encoded = encode_to_vec(&event, cfg).expect("encode AdverseEvent resolved standard");
    let (decoded, consumed): (AdverseEvent, usize) =
        decode_owned_from_slice(&encoded, cfg).expect("decode AdverseEvent resolved standard");
    assert_eq!(event, decoded);
    assert_eq!(consumed, encoded.len());
    assert!(decoded.resolved_date.is_some());
}

#[test]
fn test_adverse_event_unresolved_big_endian() {
    let event = AdverseEvent {
        event_id: 6002,
        trial_id: 202,
        patient_id: 2002,
        severity: Severity::Severe,
        onset_date: 1_700_140_000,
        resolved_date: None,
        description: "Severe allergic reaction, patient hospitalized.".to_string(),
    };
    let cfg = config::standard().with_big_endian();
    let encoded = encode_to_vec(&event, cfg).expect("encode AdverseEvent unresolved big endian");
    let (decoded, consumed): (AdverseEvent, usize) =
        decode_owned_from_slice(&encoded, cfg).expect("decode AdverseEvent unresolved big endian");
    assert_eq!(event, decoded);
    assert_eq!(consumed, encoded.len());
    assert_eq!(decoded.resolved_date, None);
    assert_eq!(decoded.severity, Severity::Severe);
}

#[test]
fn test_adverse_event_critical_fixed_int() {
    let event = AdverseEvent {
        event_id: 6003,
        trial_id: 303,
        patient_id: 3003,
        severity: Severity::Critical,
        onset_date: 1_700_150_000,
        resolved_date: None,
        description: "Critical cardiac event detected during monitoring.".to_string(),
    };
    let cfg = config::standard().with_fixed_int_encoding();
    let encoded = encode_to_vec(&event, cfg).expect("encode AdverseEvent critical fixed int");
    let (decoded, consumed): (AdverseEvent, usize) =
        decode_owned_from_slice(&encoded, cfg).expect("decode AdverseEvent critical fixed int");
    assert_eq!(event, decoded);
    assert_eq!(consumed, encoded.len());
    assert_eq!(decoded.severity, Severity::Critical);
}

#[test]
fn test_adverse_event_moderate_resolved_fixed_int() {
    let event = AdverseEvent {
        event_id: 6004,
        trial_id: 404,
        patient_id: 4004,
        severity: Severity::Moderate,
        onset_date: 1_700_160_000,
        resolved_date: Some(1_700_332_800),
        description: "Moderate headache and dizziness post-infusion.".to_string(),
    };
    let cfg = config::standard().with_fixed_int_encoding();
    let encoded =
        encode_to_vec(&event, cfg).expect("encode AdverseEvent moderate resolved fixed int");
    let (decoded, consumed): (AdverseEvent, usize) = decode_owned_from_slice(&encoded, cfg)
        .expect("decode AdverseEvent moderate resolved fixed int");
    assert_eq!(event, decoded);
    assert_eq!(consumed, encoded.len());
    assert!(decoded.resolved_date.is_some());
}

// --- Vec roundtrip tests ---

#[test]
fn test_vec_vital_signs_roundtrip() {
    let vitals = vec![
        VitalSign {
            patient_id: 100,
            timestamp: 1_700_170_000,
            heart_rate_bpm: 65,
            systolic_bp: 118,
            diastolic_bp: 75,
            spo2_pct: 99,
            temp_c_x100: 3690,
        },
        VitalSign {
            patient_id: 101,
            timestamp: 1_700_170_060,
            heart_rate_bpm: 90,
            systolic_bp: 150,
            diastolic_bp: 95,
            spo2_pct: 93,
            temp_c_x100: 3820,
        },
        VitalSign {
            patient_id: 102,
            timestamp: 1_700_170_120,
            heart_rate_bpm: 55,
            systolic_bp: 105,
            diastolic_bp: 65,
            spo2_pct: 97,
            temp_c_x100: 3660,
        },
    ];
    let cfg = config::standard();
    let encoded = encode_to_vec(&vitals, cfg).expect("encode Vec<VitalSign>");
    let (decoded, consumed): (Vec<VitalSign>, usize) =
        decode_owned_from_slice(&encoded, cfg).expect("decode Vec<VitalSign>");
    assert_eq!(vitals, decoded);
    assert_eq!(consumed, encoded.len());
    assert_eq!(decoded.len(), 3);
}

#[test]
fn test_vec_prescriptions_big_endian_roundtrip() {
    let prescriptions = vec![
        Prescription {
            rx_id: 9001,
            patient_id: 200,
            drug_code: "ASP81".to_string(),
            dosage_mg: 81,
            frequency_daily: 1,
            duration_days: 365,
            issued_at: 1_700_180_000,
        },
        Prescription {
            rx_id: 9002,
            patient_id: 201,
            drug_code: "SIM20".to_string(),
            dosage_mg: 20,
            frequency_daily: 1,
            duration_days: 180,
            issued_at: 1_700_180_300,
        },
    ];
    let cfg = config::standard().with_big_endian();
    let encoded = encode_to_vec(&prescriptions, cfg).expect("encode Vec<Prescription> big endian");
    let (decoded, consumed): (Vec<Prescription>, usize) =
        decode_owned_from_slice(&encoded, cfg).expect("decode Vec<Prescription> big endian");
    assert_eq!(prescriptions, decoded);
    assert_eq!(consumed, encoded.len());
    assert_eq!(decoded.len(), 2);
}

#[test]
fn test_vec_trial_enrollments_fixed_int_roundtrip() {
    let enrollments = vec![
        TrialEnrollment {
            enrollment_id: 10001,
            trial_id: 501,
            patient_id: 300,
            phase: TrialPhase::Phase2,
            consent: ConsentStatus::Granted,
            enrolled_date: 1_700_190_000,
            arm_id: 0,
        },
        TrialEnrollment {
            enrollment_id: 10002,
            trial_id: 501,
            patient_id: 301,
            phase: TrialPhase::Phase2,
            consent: ConsentStatus::Pending,
            enrolled_date: 1_700_190_600,
            arm_id: 1,
        },
        TrialEnrollment {
            enrollment_id: 10003,
            trial_id: 501,
            patient_id: 302,
            phase: TrialPhase::Phase2,
            consent: ConsentStatus::Withdrawn,
            enrolled_date: 1_700_191_200,
            arm_id: 0,
        },
    ];
    let cfg = config::standard().with_fixed_int_encoding();
    let encoded = encode_to_vec(&enrollments, cfg).expect("encode Vec<TrialEnrollment> fixed int");
    let (decoded, consumed): (Vec<TrialEnrollment>, usize) =
        decode_owned_from_slice(&encoded, cfg).expect("decode Vec<TrialEnrollment> fixed int");
    assert_eq!(enrollments, decoded);
    assert_eq!(consumed, encoded.len());
    assert_eq!(decoded.len(), 3);
}
