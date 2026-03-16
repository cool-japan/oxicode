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

// ── Domain types: Digital Health / Wearable Devices ──────────────────────────

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
enum HeartRhythm {
    Normal,
    Bradycardia,
    Tachycardia,
    Arrhythmia,
    AtrialFibrillation,
    VentricularFibrillation,
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
enum SleepStage {
    Awake,
    REM,
    LightSleep,
    DeepSleep,
    Unknown,
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
enum ActivityType {
    Sedentary,
    Walking,
    Running,
    Cycling,
    Swimming,
    Strength,
    Yoga,
    Custom(u16),
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
enum StressLevel {
    Low,
    Moderate,
    High,
    Critical,
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
enum FallSeverity {
    None,
    Minor,
    Moderate,
    Severe,
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
struct HeartRateSample {
    timestamp_ms: u64,
    bpm: u16,
    rr_interval_ms: u16,
    rhythm: HeartRhythm,
    signal_quality: u8,
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
struct SpO2Sample {
    timestamp_ms: u64,
    spo2_percent: u8,
    perfusion_index: f32,
    signal_quality: u8,
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
struct SleepEpoch {
    epoch_id: u32,
    start_ms: u64,
    duration_sec: u16,
    stage: SleepStage,
    heart_rate_bpm: u16,
    movement_count: u32,
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
struct SleepSession {
    session_id: u64,
    device_id: String,
    epochs: Vec<SleepEpoch>,
    total_duration_sec: u32,
    sleep_score: u8,
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
struct ActivitySegment {
    segment_id: u32,
    activity: ActivityType,
    start_ms: u64,
    duration_sec: u32,
    calories_kcal: f32,
    steps: Option<u32>,
    distance_meters: Option<f32>,
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
struct EcgLead {
    lead_id: u8,
    samples_mv: Vec<f32>,
    sample_rate_hz: u16,
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
struct EcgAnalysis {
    recording_id: u64,
    timestamp_ms: u64,
    leads: Vec<EcgLead>,
    rhythm: HeartRhythm,
    pr_interval_ms: Option<u16>,
    qrs_duration_ms: Option<u16>,
    qt_interval_ms: Option<u16>,
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
struct StressEvent {
    event_id: u64,
    timestamp_ms: u64,
    level: StressLevel,
    hrv_ms: f32,
    respiratory_rate: f32,
    skin_conductance_us: Option<f32>,
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
struct FallDetectionEvent {
    event_id: u64,
    timestamp_ms: u64,
    severity: FallSeverity,
    impact_g: f32,
    posture_before: String,
    posture_after: String,
    auto_alert_sent: bool,
    location: Option<String>,
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
struct HealthScore {
    score_id: u64,
    computed_at_ms: u64,
    overall: u8,
    cardiovascular: u8,
    sleep_quality: u8,
    activity_level: u8,
    stress_resilience: u8,
    notes: Option<String>,
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
struct WorkoutSession {
    session_id: u64,
    activity: ActivityType,
    start_ms: u64,
    end_ms: u64,
    heart_rate_samples: Vec<HeartRateSample>,
    spo2_samples: Vec<SpO2Sample>,
    peak_bpm: u16,
    avg_bpm: u16,
    calories_kcal: f32,
    completed: bool,
}

// ── helpers ───────────────────────────────────────────────────────────────────

fn make_hr_sample(ts: u64, bpm: u16, rhythm: HeartRhythm) -> HeartRateSample {
    HeartRateSample {
        timestamp_ms: ts,
        bpm,
        rr_interval_ms: 60_000 / bpm,
        rhythm,
        signal_quality: 255,
    }
}

fn make_spo2_sample(ts: u64, pct: u8) -> SpO2Sample {
    SpO2Sample {
        timestamp_ms: ts,
        spo2_percent: pct,
        perfusion_index: 2.5,
        signal_quality: 200,
    }
}

fn make_sleep_epoch(id: u32, start: u64, stage: SleepStage) -> SleepEpoch {
    SleepEpoch {
        epoch_id: id,
        start_ms: start,
        duration_sec: 30,
        stage,
        heart_rate_bpm: 58,
        movement_count: 3,
    }
}

fn make_health_score(id: u64) -> HealthScore {
    HealthScore {
        score_id: id,
        computed_at_ms: 1_700_000_000_000 + id,
        overall: 82,
        cardiovascular: 88,
        sleep_quality: 75,
        activity_level: 80,
        stress_resilience: 85,
        notes: None,
    }
}

// ── 1. HeartRateSample basic roundtrip ───────────────────────────────────────
#[test]
fn test_heart_rate_sample_basic_roundtrip() {
    let cfg = config::standard();
    let sample = make_hr_sample(1_700_000_000_000, 72, HeartRhythm::Normal);
    let bytes = encode_to_vec(&sample, cfg).expect("encode HeartRateSample");
    let (decoded, _): (HeartRateSample, usize) =
        decode_owned_from_slice(&bytes, cfg).expect("decode HeartRateSample");
    assert_eq!(sample, decoded);
}

// ── 2. HeartRhythm: all variants ──────────────────────────────────────────────
#[test]
fn test_heart_rhythm_all_variants() {
    let cfg = config::standard();
    let rhythms = [
        HeartRhythm::Normal,
        HeartRhythm::Bradycardia,
        HeartRhythm::Tachycardia,
        HeartRhythm::Arrhythmia,
        HeartRhythm::AtrialFibrillation,
        HeartRhythm::VentricularFibrillation,
    ];
    for rhythm in &rhythms {
        let bytes = encode_to_vec(rhythm, cfg).expect("encode HeartRhythm");
        let (decoded, _): (HeartRhythm, usize) =
            decode_owned_from_slice(&bytes, cfg).expect("decode HeartRhythm");
        assert_eq!(rhythm, &decoded);
    }
}

// ── 3. SleepStage: all variants ───────────────────────────────────────────────
#[test]
fn test_sleep_stage_all_variants() {
    let cfg = config::standard();
    let stages = [
        SleepStage::Awake,
        SleepStage::REM,
        SleepStage::LightSleep,
        SleepStage::DeepSleep,
        SleepStage::Unknown,
    ];
    for stage in &stages {
        let bytes = encode_to_vec(stage, cfg).expect("encode SleepStage");
        let (decoded, _): (SleepStage, usize) =
            decode_owned_from_slice(&bytes, cfg).expect("decode SleepStage");
        assert_eq!(stage, &decoded);
    }
}

// ── 4. ActivityType: unit and newtype variants ────────────────────────────────
#[test]
fn test_activity_type_all_variants() {
    let cfg = config::standard();
    let activities = vec![
        ActivityType::Sedentary,
        ActivityType::Walking,
        ActivityType::Running,
        ActivityType::Cycling,
        ActivityType::Swimming,
        ActivityType::Strength,
        ActivityType::Yoga,
        ActivityType::Custom(100),
        ActivityType::Custom(0xFFFF),
    ];
    for activity in &activities {
        let bytes = encode_to_vec(activity, cfg).expect("encode ActivityType");
        let (decoded, _): (ActivityType, usize) =
            decode_owned_from_slice(&bytes, cfg).expect("decode ActivityType");
        assert_eq!(activity, &decoded);
    }
}

// ── 5. Vec<HeartRateSample> roundtrip ────────────────────────────────────────
#[test]
fn test_vec_heart_rate_sample_roundtrip() {
    let cfg = config::standard();
    let samples = vec![
        make_hr_sample(1_700_000_000_000, 65, HeartRhythm::Normal),
        make_hr_sample(1_700_000_001_000, 68, HeartRhythm::Normal),
        make_hr_sample(1_700_000_002_000, 110, HeartRhythm::Tachycardia),
        make_hr_sample(1_700_000_003_000, 55, HeartRhythm::Bradycardia),
        make_hr_sample(1_700_000_004_000, 72, HeartRhythm::Normal),
    ];
    let bytes = encode_to_vec(&samples, cfg).expect("encode Vec<HeartRateSample>");
    let (decoded, _): (Vec<HeartRateSample>, usize) =
        decode_owned_from_slice(&bytes, cfg).expect("decode Vec<HeartRateSample>");
    assert_eq!(samples, decoded);
    assert_eq!(decoded.len(), 5);
}

// ── 6. SleepSession with multiple epochs roundtrip ───────────────────────────
#[test]
fn test_sleep_session_roundtrip() {
    let cfg = config::standard();
    let base = 1_700_000_000_000_u64;
    let epochs = vec![
        make_sleep_epoch(0, base, SleepStage::Awake),
        make_sleep_epoch(1, base + 30_000, SleepStage::LightSleep),
        make_sleep_epoch(2, base + 60_000, SleepStage::DeepSleep),
        make_sleep_epoch(3, base + 90_000, SleepStage::REM),
        make_sleep_epoch(4, base + 120_000, SleepStage::LightSleep),
        make_sleep_epoch(5, base + 150_000, SleepStage::Awake),
    ];
    let session = SleepSession {
        session_id: 42,
        device_id: "wearable-device-001".to_string(),
        epochs,
        total_duration_sec: 180,
        sleep_score: 78,
    };
    let bytes = encode_to_vec(&session, cfg).expect("encode SleepSession");
    let (decoded, _): (SleepSession, usize) =
        decode_owned_from_slice(&bytes, cfg).expect("decode SleepSession");
    assert_eq!(session, decoded);
    assert_eq!(decoded.epochs.len(), 6);
}

// ── 7. ActivitySegment with Option<u32> steps = Some ─────────────────────────
#[test]
fn test_activity_segment_with_steps() {
    let cfg = config::standard();
    let segment = ActivitySegment {
        segment_id: 1,
        activity: ActivityType::Running,
        start_ms: 1_700_000_000_000,
        duration_sec: 1800,
        calories_kcal: 350.5,
        steps: Some(4500),
        distance_meters: Some(5200.0),
    };
    let bytes = encode_to_vec(&segment, cfg).expect("encode ActivitySegment with steps");
    let (decoded, _): (ActivitySegment, usize) =
        decode_owned_from_slice(&bytes, cfg).expect("decode ActivitySegment with steps");
    assert_eq!(segment, decoded);
    assert_eq!(decoded.steps, Some(4500));
    assert_eq!(decoded.distance_meters, Some(5200.0));
}

// ── 8. ActivitySegment with Option fields = None ─────────────────────────────
#[test]
fn test_activity_segment_without_optional_fields() {
    let cfg = config::standard();
    let segment = ActivitySegment {
        segment_id: 2,
        activity: ActivityType::Swimming,
        start_ms: 1_700_000_100_000,
        duration_sec: 2400,
        calories_kcal: 420.0,
        steps: None,
        distance_meters: None,
    };
    let bytes = encode_to_vec(&segment, cfg).expect("encode ActivitySegment without options");
    let (decoded, _): (ActivitySegment, usize) =
        decode_owned_from_slice(&bytes, cfg).expect("decode ActivitySegment without options");
    assert_eq!(segment, decoded);
    assert_eq!(decoded.steps, None);
    assert_eq!(decoded.distance_meters, None);
}

// ── 9. EcgAnalysis nested struct roundtrip ───────────────────────────────────
#[test]
fn test_ecg_analysis_nested_roundtrip() {
    let cfg = config::standard();
    let leads = vec![
        EcgLead {
            lead_id: 1,
            samples_mv: vec![0.02, 0.15, 0.80, 0.15, 0.02, -0.10, 0.02],
            sample_rate_hz: 500,
        },
        EcgLead {
            lead_id: 2,
            samples_mv: vec![0.01, 0.12, 0.75, 0.12, 0.01, -0.08, 0.01],
            sample_rate_hz: 500,
        },
    ];
    let analysis = EcgAnalysis {
        recording_id: 8001,
        timestamp_ms: 1_700_000_050_000,
        leads,
        rhythm: HeartRhythm::Normal,
        pr_interval_ms: Some(160),
        qrs_duration_ms: Some(90),
        qt_interval_ms: Some(400),
    };
    let bytes = encode_to_vec(&analysis, cfg).expect("encode EcgAnalysis");
    let (decoded, _): (EcgAnalysis, usize) =
        decode_owned_from_slice(&bytes, cfg).expect("decode EcgAnalysis");
    assert_eq!(analysis, decoded);
    assert_eq!(decoded.leads.len(), 2);
    assert_eq!(decoded.leads[0].samples_mv.len(), 7);
}

// ── 10. EcgAnalysis with arrhythmia and None intervals ───────────────────────
#[test]
fn test_ecg_analysis_arrhythmia_none_intervals() {
    let cfg = config::standard();
    let analysis = EcgAnalysis {
        recording_id: 8002,
        timestamp_ms: 1_700_000_060_000,
        leads: vec![EcgLead {
            lead_id: 1,
            samples_mv: vec![0.0, 0.5, -0.1, 0.3, -0.2, 0.0],
            sample_rate_hz: 250,
        }],
        rhythm: HeartRhythm::AtrialFibrillation,
        pr_interval_ms: None,
        qrs_duration_ms: None,
        qt_interval_ms: None,
    };
    let bytes = encode_to_vec(&analysis, cfg).expect("encode EcgAnalysis arrhythmia");
    let (decoded, _): (EcgAnalysis, usize) =
        decode_owned_from_slice(&bytes, cfg).expect("decode EcgAnalysis arrhythmia");
    assert_eq!(analysis, decoded);
    assert_eq!(decoded.pr_interval_ms, None);
}

// ── 11. StressEvent with skin conductance = Some ─────────────────────────────
#[test]
fn test_stress_event_with_skin_conductance() {
    let cfg = config::standard();
    let event = StressEvent {
        event_id: 3001,
        timestamp_ms: 1_700_000_200_000,
        level: StressLevel::High,
        hrv_ms: 28.4,
        respiratory_rate: 22.0,
        skin_conductance_us: Some(7.3),
    };
    let bytes = encode_to_vec(&event, cfg).expect("encode StressEvent with conductance");
    let (decoded, _): (StressEvent, usize) =
        decode_owned_from_slice(&bytes, cfg).expect("decode StressEvent with conductance");
    assert_eq!(event, decoded);
    assert_eq!(decoded.skin_conductance_us, Some(7.3));
}

// ── 12. StressLevel: all variants ────────────────────────────────────────────
#[test]
fn test_stress_level_all_variants() {
    let cfg = config::standard();
    let levels = [
        StressLevel::Low,
        StressLevel::Moderate,
        StressLevel::High,
        StressLevel::Critical,
    ];
    for level in &levels {
        let bytes = encode_to_vec(level, cfg).expect("encode StressLevel");
        let (decoded, _): (StressLevel, usize) =
            decode_owned_from_slice(&bytes, cfg).expect("decode StressLevel");
        assert_eq!(level, &decoded);
    }
}

// ── 13. FallDetectionEvent with location = Some ──────────────────────────────
#[test]
fn test_fall_detection_event_with_location() {
    let cfg = config::standard();
    let event = FallDetectionEvent {
        event_id: 5001,
        timestamp_ms: 1_700_000_300_000,
        severity: FallSeverity::Moderate,
        impact_g: 4.7,
        posture_before: "Standing".to_string(),
        posture_after: "Lying".to_string(),
        auto_alert_sent: true,
        location: Some("Living Room".to_string()),
    };
    let bytes = encode_to_vec(&event, cfg).expect("encode FallDetectionEvent with location");
    let (decoded, _): (FallDetectionEvent, usize) =
        decode_owned_from_slice(&bytes, cfg).expect("decode FallDetectionEvent with location");
    assert_eq!(event, decoded);
    assert_eq!(decoded.location, Some("Living Room".to_string()));
}

// ── 14. FallSeverity: all variants ───────────────────────────────────────────
#[test]
fn test_fall_severity_all_variants() {
    let cfg = config::standard();
    let severities = [
        FallSeverity::None,
        FallSeverity::Minor,
        FallSeverity::Moderate,
        FallSeverity::Severe,
    ];
    for severity in &severities {
        let bytes = encode_to_vec(severity, cfg).expect("encode FallSeverity");
        let (decoded, _): (FallSeverity, usize) =
            decode_owned_from_slice(&bytes, cfg).expect("decode FallSeverity");
        assert_eq!(severity, &decoded);
    }
}

// ── 15. HealthScore with notes = Some ────────────────────────────────────────
#[test]
fn test_health_score_with_notes() {
    let cfg = config::standard();
    let score = HealthScore {
        score_id: 9001,
        computed_at_ms: 1_700_000_500_000,
        overall: 91,
        cardiovascular: 94,
        sleep_quality: 88,
        activity_level: 90,
        stress_resilience: 92,
        notes: Some("Excellent recovery post-marathon".to_string()),
    };
    let bytes = encode_to_vec(&score, cfg).expect("encode HealthScore with notes");
    let (decoded, _): (HealthScore, usize) =
        decode_owned_from_slice(&bytes, cfg).expect("decode HealthScore with notes");
    assert_eq!(score, decoded);
    assert_eq!(
        decoded.notes,
        Some("Excellent recovery post-marathon".to_string())
    );
}

// ── 16. WorkoutSession full roundtrip ────────────────────────────────────────
#[test]
fn test_workout_session_full_roundtrip() {
    let cfg = config::standard();
    let start = 1_700_000_600_000_u64;
    let session = WorkoutSession {
        session_id: 7001,
        activity: ActivityType::Cycling,
        start_ms: start,
        end_ms: start + 3_600_000,
        heart_rate_samples: vec![
            make_hr_sample(start, 80, HeartRhythm::Normal),
            make_hr_sample(start + 600_000, 140, HeartRhythm::Tachycardia),
            make_hr_sample(start + 1_800_000, 155, HeartRhythm::Tachycardia),
            make_hr_sample(start + 3_300_000, 90, HeartRhythm::Normal),
        ],
        spo2_samples: vec![
            make_spo2_sample(start, 98),
            make_spo2_sample(start + 1_800_000, 96),
            make_spo2_sample(start + 3_600_000, 97),
        ],
        peak_bpm: 165,
        avg_bpm: 135,
        calories_kcal: 820.0,
        completed: true,
    };
    let bytes = encode_to_vec(&session, cfg).expect("encode WorkoutSession");
    let (decoded, _): (WorkoutSession, usize) =
        decode_owned_from_slice(&bytes, cfg).expect("decode WorkoutSession");
    assert_eq!(session, decoded);
    assert_eq!(decoded.heart_rate_samples.len(), 4);
    assert_eq!(decoded.spo2_samples.len(), 3);
}

// ── 17. Big-endian config with SpO2Sample ────────────────────────────────────
#[test]
fn test_big_endian_spo2_sample() {
    let cfg = config::standard().with_big_endian();
    let sample = SpO2Sample {
        timestamp_ms: 0xDEAD_CAFE_BEEF_F00D,
        spo2_percent: 97,
        perfusion_index: 3.14,
        signal_quality: 180,
    };
    let bytes = encode_to_vec(&sample, cfg).expect("encode SpO2Sample big_endian");
    let (decoded, _): (SpO2Sample, usize) =
        decode_owned_from_slice(&bytes, cfg).expect("decode SpO2Sample big_endian");
    assert_eq!(sample, decoded);
}

// ── 18. Fixed-int config with HealthScore ────────────────────────────────────
#[test]
fn test_fixed_int_health_score() {
    let cfg = config::standard().with_fixed_int_encoding();
    let score = HealthScore {
        score_id: u64::MAX,
        computed_at_ms: u64::MAX - 1,
        overall: 100,
        cardiovascular: 100,
        sleep_quality: 100,
        activity_level: 100,
        stress_resilience: 100,
        notes: None,
    };
    let bytes = encode_to_vec(&score, cfg).expect("encode HealthScore fixed_int");
    let (decoded, _): (HealthScore, usize) =
        decode_owned_from_slice(&bytes, cfg).expect("decode HealthScore fixed_int");
    assert_eq!(score, decoded);
}

// ── 19. Combined big-endian + fixed-int with FallDetectionEvent ──────────────
#[test]
fn test_combined_config_fall_detection_event() {
    let cfg = config::standard()
        .with_big_endian()
        .with_fixed_int_encoding();
    let event = FallDetectionEvent {
        event_id: u64::MAX,
        timestamp_ms: u64::MAX - 100,
        severity: FallSeverity::Severe,
        impact_g: 12.5,
        posture_before: "Running".to_string(),
        posture_after: "Prone".to_string(),
        auto_alert_sent: true,
        location: None,
    };
    let bytes = encode_to_vec(&event, cfg).expect("encode FallDetectionEvent combined config");
    let (decoded, _): (FallDetectionEvent, usize) =
        decode_owned_from_slice(&bytes, cfg).expect("decode FallDetectionEvent combined config");
    assert_eq!(event, decoded);
    assert_eq!(decoded.location, None);
}

// ── 20. Vec<HealthScore> roundtrip ───────────────────────────────────────────
#[test]
fn test_vec_health_score_roundtrip() {
    let cfg = config::standard();
    let scores: Vec<HealthScore> = (1..=7).map(|i| make_health_score(i)).collect();
    let bytes = encode_to_vec(&scores, cfg).expect("encode Vec<HealthScore>");
    let (decoded, _): (Vec<HealthScore>, usize) =
        decode_owned_from_slice(&bytes, cfg).expect("decode Vec<HealthScore>");
    assert_eq!(scores, decoded);
    assert_eq!(decoded.len(), 7);
}

// ── 21. Consumed bytes equals encoded length ─────────────────────────────────
#[test]
fn test_consumed_bytes_equals_encoded_length() {
    let cfg = config::standard();
    let start = 1_700_001_000_000_u64;
    let session = WorkoutSession {
        session_id: 8888,
        activity: ActivityType::Running,
        start_ms: start,
        end_ms: start + 1_800_000,
        heart_rate_samples: vec![
            make_hr_sample(start, 75, HeartRhythm::Normal),
            make_hr_sample(start + 300_000, 150, HeartRhythm::Tachycardia),
            make_hr_sample(start + 900_000, 165, HeartRhythm::Tachycardia),
            make_hr_sample(start + 1_500_000, 120, HeartRhythm::Normal),
            make_hr_sample(start + 1_800_000, 85, HeartRhythm::Normal),
        ],
        spo2_samples: vec![
            make_spo2_sample(start, 99),
            make_spo2_sample(start + 900_000, 96),
        ],
        peak_bpm: 170,
        avg_bpm: 140,
        calories_kcal: 550.0,
        completed: true,
    };
    let bytes = encode_to_vec(&session, cfg).expect("encode for consumed-bytes check");
    let (decoded, consumed): (WorkoutSession, usize) =
        decode_owned_from_slice(&bytes, cfg).expect("decode for consumed-bytes check");
    assert_eq!(session, decoded);
    assert_eq!(
        consumed,
        bytes.len(),
        "consumed bytes must equal total encoded length"
    );
}

// ── 22. Multi-day sleep analysis via Vec<SleepSession> ───────────────────────
#[test]
fn test_multi_day_sleep_analysis() {
    let cfg = config::standard();
    let day0_base = 1_700_000_000_000_u64;
    let day1_base = day0_base + 86_400_000;
    let day2_base = day1_base + 86_400_000;

    let sessions = vec![
        SleepSession {
            session_id: 1,
            device_id: "watch-sensor-42".to_string(),
            epochs: vec![
                make_sleep_epoch(0, day0_base, SleepStage::Awake),
                make_sleep_epoch(1, day0_base + 30_000, SleepStage::LightSleep),
                make_sleep_epoch(2, day0_base + 60_000, SleepStage::DeepSleep),
                make_sleep_epoch(3, day0_base + 90_000, SleepStage::REM),
            ],
            total_duration_sec: 28800,
            sleep_score: 82,
        },
        SleepSession {
            session_id: 2,
            device_id: "watch-sensor-42".to_string(),
            epochs: vec![
                make_sleep_epoch(0, day1_base, SleepStage::LightSleep),
                make_sleep_epoch(1, day1_base + 30_000, SleepStage::DeepSleep),
                make_sleep_epoch(2, day1_base + 60_000, SleepStage::DeepSleep),
                make_sleep_epoch(3, day1_base + 90_000, SleepStage::REM),
                make_sleep_epoch(4, day1_base + 120_000, SleepStage::Awake),
            ],
            total_duration_sec: 27000,
            sleep_score: 75,
        },
        SleepSession {
            session_id: 3,
            device_id: "watch-sensor-42".to_string(),
            epochs: vec![
                make_sleep_epoch(0, day2_base, SleepStage::Awake),
                make_sleep_epoch(1, day2_base + 30_000, SleepStage::REM),
                make_sleep_epoch(2, day2_base + 60_000, SleepStage::DeepSleep),
            ],
            total_duration_sec: 25200,
            sleep_score: 70,
        },
    ];
    let bytes = encode_to_vec(&sessions, cfg).expect("encode Vec<SleepSession>");
    let (decoded, _): (Vec<SleepSession>, usize) =
        decode_owned_from_slice(&bytes, cfg).expect("decode Vec<SleepSession>");
    assert_eq!(sessions, decoded);
    assert_eq!(decoded.len(), 3);
    assert_eq!(decoded[0].epochs.len(), 4);
    assert_eq!(decoded[1].epochs.len(), 5);
    assert_eq!(decoded[2].epochs.len(), 3);
    assert_eq!(decoded[0].sleep_score, 82);
    assert_eq!(decoded[1].sleep_score, 75);
    assert_eq!(decoded[2].sleep_score, 70);
}
