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

// ── Domain types: Fitness center & gym management ───────────────────────────

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
enum MembershipType {
    Basic,
    Premium,
    VIP,
    Student,
    Senior,
    Corporate,
    FamilyPlan,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct FitnessGoal {
    description: String,
    target_value: f64,
    current_value: f64,
    unit: String,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct MemberProfile {
    member_id: u64,
    name: String,
    membership: MembershipType,
    join_year: u16,
    join_month: u8,
    join_day: u8,
    goals: Vec<FitnessGoal>,
    emergency_contact: String,
    active: bool,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct ExerciseSet {
    set_number: u8,
    reps: u16,
    weight_kg: f32,
    rest_seconds: u16,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct Exercise {
    name: String,
    sets: Vec<ExerciseSet>,
    notes: String,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct WorkoutSession {
    session_id: u64,
    member_id: u64,
    date_epoch: u64,
    duration_minutes: u16,
    exercises: Vec<Exercise>,
    calories_burned: u32,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct BodyComposition {
    member_id: u64,
    date_epoch: u64,
    weight_kg: f64,
    height_cm: f64,
    bmi: f64,
    body_fat_pct: f64,
    lean_mass_kg: f64,
    waist_cm: f64,
    chest_cm: f64,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
enum ClassType {
    Yoga,
    Spinning,
    Hiit,
    CrossFit,
    Pilates,
    Zumba,
    Boxing,
    Swimming,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct ClassSchedule {
    class_id: u32,
    class_type: ClassType,
    instructor: String,
    day_of_week: u8,
    start_hour: u8,
    start_minute: u8,
    duration_minutes: u16,
    max_capacity: u16,
    enrolled_member_ids: Vec<u64>,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct PersonalTrainerAssignment {
    trainer_id: u32,
    trainer_name: String,
    member_id: u64,
    sessions_per_week: u8,
    specializations: Vec<String>,
    hourly_rate_cents: u32,
    start_epoch: u64,
    end_epoch: u64,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
enum EquipmentStatus {
    Operational,
    NeedsMaintenance,
    UnderRepair,
    OutOfService,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct EquipmentMaintenanceRecord {
    equipment_id: u32,
    equipment_name: String,
    status: EquipmentStatus,
    last_inspection_epoch: u64,
    next_inspection_epoch: u64,
    maintenance_notes: Vec<String>,
    cost_cents: u32,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
enum HeartRateZone {
    Rest,
    WarmUp,
    FatBurn,
    Cardio,
    Peak,
    Anaerobic,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct HeartRateTracking {
    member_id: u64,
    session_id: u64,
    samples: Vec<(u32, u8)>,
    zone_durations_sec: Vec<(HeartRateZone, u32)>,
    avg_bpm: u8,
    max_bpm: u8,
    resting_bpm: u8,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct NutritionMacros {
    member_id: u64,
    date_epoch: u64,
    calories: u32,
    protein_g: f32,
    carbs_g: f32,
    fat_g: f32,
    fiber_g: f32,
    water_ml: u32,
    meals: Vec<String>,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
enum BillingStatus {
    Active,
    PastDue,
    Suspended,
    Cancelled,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct MembershipBillingCycle {
    member_id: u64,
    cycle_start_epoch: u64,
    cycle_end_epoch: u64,
    amount_cents: u32,
    status: BillingStatus,
    payment_method: String,
    auto_renew: bool,
    discount_pct: u8,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct CheckInOut {
    member_id: u64,
    check_in_epoch: u64,
    check_out_epoch: u64,
    facility_zone: String,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct FacilityUtilizationSlot {
    hour: u8,
    day_of_week: u8,
    occupancy_count: u32,
    capacity: u32,
    zone: String,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct FacilityHeatmap {
    facility_name: String,
    slots: Vec<FacilityUtilizationSlot>,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct LeaderboardEntry {
    member_id: u64,
    member_name: String,
    score: u64,
    rank: u32,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct GroupChallenge {
    challenge_id: u32,
    name: String,
    description: String,
    start_epoch: u64,
    end_epoch: u64,
    leaderboard: Vec<LeaderboardEntry>,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
enum InjurySeverity {
    Minor,
    Moderate,
    Severe,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct InjuryIncidentReport {
    incident_id: u32,
    member_id: u64,
    date_epoch: u64,
    location: String,
    description: String,
    severity: InjurySeverity,
    equipment_involved: Option<String>,
    witness_names: Vec<String>,
    follow_up_required: bool,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
enum AmenityType {
    Sauna,
    SteamRoom,
    Pool,
    HotTub,
    ColdPlunge,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct AmenityUsageLog {
    member_id: u64,
    amenity: AmenityType,
    start_epoch: u64,
    duration_minutes: u16,
    temperature_c: Option<f32>,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct ReferralRecord {
    referrer_member_id: u64,
    referred_name: String,
    referred_email: String,
    referral_epoch: u64,
    signup_epoch: Option<u64>,
    reward_cents: u32,
    redeemed: bool,
}

// ── Tests ───────────────────────────────────────────────────────────────────

#[test]
fn test_member_profile_roundtrip() {
    let profile = MemberProfile {
        member_id: 100_001,
        name: "Alice Johnson".to_string(),
        membership: MembershipType::Premium,
        join_year: 2024,
        join_month: 3,
        join_day: 15,
        goals: vec![
            FitnessGoal {
                description: "Lose weight".to_string(),
                target_value: 70.0,
                current_value: 78.5,
                unit: "kg".to_string(),
            },
            FitnessGoal {
                description: "Run 5K under 25 min".to_string(),
                target_value: 25.0,
                current_value: 28.3,
                unit: "minutes".to_string(),
            },
        ],
        emergency_contact: "+1-555-0199".to_string(),
        active: true,
    };
    let encoded = encode_to_vec(&profile).expect("encode member profile");
    let compressed = compress_lz4(&encoded).expect("compress member profile");
    let decompressed = decompress_lz4(&compressed).expect("decompress member profile");
    let (decoded, _): (MemberProfile, usize) =
        decode_from_slice(&decompressed).expect("decode member profile");
    assert_eq!(profile, decoded);
}

#[test]
fn test_workout_session_roundtrip() {
    let session = WorkoutSession {
        session_id: 550_001,
        member_id: 100_001,
        date_epoch: 1_710_000_000,
        duration_minutes: 75,
        exercises: vec![
            Exercise {
                name: "Barbell Bench Press".to_string(),
                sets: vec![
                    ExerciseSet {
                        set_number: 1,
                        reps: 12,
                        weight_kg: 60.0,
                        rest_seconds: 90,
                    },
                    ExerciseSet {
                        set_number: 2,
                        reps: 10,
                        weight_kg: 70.0,
                        rest_seconds: 90,
                    },
                    ExerciseSet {
                        set_number: 3,
                        reps: 8,
                        weight_kg: 80.0,
                        rest_seconds: 120,
                    },
                ],
                notes: "Good form on all sets".to_string(),
            },
            Exercise {
                name: "Incline Dumbbell Fly".to_string(),
                sets: vec![
                    ExerciseSet {
                        set_number: 1,
                        reps: 15,
                        weight_kg: 12.0,
                        rest_seconds: 60,
                    },
                    ExerciseSet {
                        set_number: 2,
                        reps: 12,
                        weight_kg: 14.0,
                        rest_seconds: 60,
                    },
                ],
                notes: String::new(),
            },
        ],
        calories_burned: 520,
    };
    let encoded = encode_to_vec(&session).expect("encode workout session");
    let compressed = compress_lz4(&encoded).expect("compress workout session");
    let decompressed = decompress_lz4(&compressed).expect("decompress workout session");
    let (decoded, _): (WorkoutSession, usize) =
        decode_from_slice(&decompressed).expect("decode workout session");
    assert_eq!(session, decoded);
}

#[test]
fn test_body_composition_roundtrip() {
    let comp = BodyComposition {
        member_id: 100_002,
        date_epoch: 1_710_100_000,
        weight_kg: 82.3,
        height_cm: 178.0,
        bmi: 25.97,
        body_fat_pct: 18.4,
        lean_mass_kg: 67.15,
        waist_cm: 86.0,
        chest_cm: 102.5,
    };
    let encoded = encode_to_vec(&comp).expect("encode body composition");
    let compressed = compress_lz4(&encoded).expect("compress body composition");
    let decompressed = decompress_lz4(&compressed).expect("decompress body composition");
    let (decoded, _): (BodyComposition, usize) =
        decode_from_slice(&decompressed).expect("decode body composition");
    assert_eq!(comp, decoded);
}

#[test]
fn test_class_schedule_roundtrip() {
    let schedule = ClassSchedule {
        class_id: 301,
        class_type: ClassType::Hiit,
        instructor: "Coach Martinez".to_string(),
        day_of_week: 2,
        start_hour: 18,
        start_minute: 30,
        duration_minutes: 45,
        max_capacity: 25,
        enrolled_member_ids: vec![100_001, 100_005, 100_012, 100_034, 100_089],
    };
    let encoded = encode_to_vec(&schedule).expect("encode class schedule");
    let compressed = compress_lz4(&encoded).expect("compress class schedule");
    let decompressed = decompress_lz4(&compressed).expect("decompress class schedule");
    let (decoded, _): (ClassSchedule, usize) =
        decode_from_slice(&decompressed).expect("decode class schedule");
    assert_eq!(schedule, decoded);
}

#[test]
fn test_personal_trainer_assignment_roundtrip() {
    let assignment = PersonalTrainerAssignment {
        trainer_id: 50,
        trainer_name: "Derek Powell".to_string(),
        member_id: 100_042,
        sessions_per_week: 3,
        specializations: vec![
            "Strength Training".to_string(),
            "Olympic Lifting".to_string(),
            "Mobility".to_string(),
        ],
        hourly_rate_cents: 8500,
        start_epoch: 1_709_000_000,
        end_epoch: 1_717_000_000,
    };
    let encoded = encode_to_vec(&assignment).expect("encode trainer assignment");
    let compressed = compress_lz4(&encoded).expect("compress trainer assignment");
    let decompressed = decompress_lz4(&compressed).expect("decompress trainer assignment");
    let (decoded, _): (PersonalTrainerAssignment, usize) =
        decode_from_slice(&decompressed).expect("decode trainer assignment");
    assert_eq!(assignment, decoded);
}

#[test]
fn test_equipment_maintenance_roundtrip() {
    let record = EquipmentMaintenanceRecord {
        equipment_id: 2001,
        equipment_name: "Treadmill #7 - Life Fitness".to_string(),
        status: EquipmentStatus::NeedsMaintenance,
        last_inspection_epoch: 1_708_500_000,
        next_inspection_epoch: 1_711_000_000,
        maintenance_notes: vec![
            "Belt showing wear on left side".to_string(),
            "Motor noise at speeds above 12 km/h".to_string(),
            "Display flickering intermittently".to_string(),
        ],
        cost_cents: 45_000,
    };
    let encoded = encode_to_vec(&record).expect("encode equipment record");
    let compressed = compress_lz4(&encoded).expect("compress equipment record");
    let decompressed = decompress_lz4(&compressed).expect("decompress equipment record");
    let (decoded, _): (EquipmentMaintenanceRecord, usize) =
        decode_from_slice(&decompressed).expect("decode equipment record");
    assert_eq!(record, decoded);
}

#[test]
fn test_heart_rate_tracking_roundtrip() {
    let hr = HeartRateTracking {
        member_id: 100_010,
        session_id: 550_200,
        samples: vec![
            (0, 72),
            (30, 95),
            (60, 120),
            (90, 145),
            (120, 162),
            (150, 170),
            (180, 155),
            (210, 140),
            (240, 110),
            (270, 85),
        ],
        zone_durations_sec: vec![
            (HeartRateZone::WarmUp, 60),
            (HeartRateZone::FatBurn, 90),
            (HeartRateZone::Cardio, 120),
            (HeartRateZone::Peak, 30),
        ],
        avg_bpm: 132,
        max_bpm: 170,
        resting_bpm: 62,
    };
    let encoded = encode_to_vec(&hr).expect("encode heart rate tracking");
    let compressed = compress_lz4(&encoded).expect("compress heart rate tracking");
    let decompressed = decompress_lz4(&compressed).expect("decompress heart rate tracking");
    let (decoded, _): (HeartRateTracking, usize) =
        decode_from_slice(&decompressed).expect("decode heart rate tracking");
    assert_eq!(hr, decoded);
}

#[test]
fn test_nutrition_macros_roundtrip() {
    let macros = NutritionMacros {
        member_id: 100_020,
        date_epoch: 1_710_200_000,
        calories: 2350,
        protein_g: 180.5,
        carbs_g: 220.0,
        fat_g: 72.3,
        fiber_g: 35.0,
        water_ml: 3200,
        meals: vec![
            "Oatmeal with berries and whey".to_string(),
            "Grilled chicken salad".to_string(),
            "Protein shake post-workout".to_string(),
            "Salmon with sweet potato and broccoli".to_string(),
            "Greek yogurt with almonds".to_string(),
        ],
    };
    let encoded = encode_to_vec(&macros).expect("encode nutrition macros");
    let compressed = compress_lz4(&encoded).expect("compress nutrition macros");
    let decompressed = decompress_lz4(&compressed).expect("decompress nutrition macros");
    let (decoded, _): (NutritionMacros, usize) =
        decode_from_slice(&decompressed).expect("decode nutrition macros");
    assert_eq!(macros, decoded);
}

#[test]
fn test_billing_cycle_roundtrip() {
    let billing = MembershipBillingCycle {
        member_id: 100_055,
        cycle_start_epoch: 1_709_000_000,
        cycle_end_epoch: 1_711_600_000,
        amount_cents: 5999,
        status: BillingStatus::Active,
        payment_method: "Visa ending 4242".to_string(),
        auto_renew: true,
        discount_pct: 10,
    };
    let encoded = encode_to_vec(&billing).expect("encode billing cycle");
    let compressed = compress_lz4(&encoded).expect("compress billing cycle");
    let decompressed = decompress_lz4(&compressed).expect("decompress billing cycle");
    let (decoded, _): (MembershipBillingCycle, usize) =
        decode_from_slice(&decompressed).expect("decode billing cycle");
    assert_eq!(billing, decoded);
}

#[test]
fn test_check_in_out_roundtrip() {
    let visit = CheckInOut {
        member_id: 100_077,
        check_in_epoch: 1_710_300_000,
        check_out_epoch: 1_710_305_400,
        facility_zone: "Weight Room Floor 2".to_string(),
    };
    let encoded = encode_to_vec(&visit).expect("encode check-in/out");
    let compressed = compress_lz4(&encoded).expect("compress check-in/out");
    let decompressed = decompress_lz4(&compressed).expect("decompress check-in/out");
    let (decoded, _): (CheckInOut, usize) =
        decode_from_slice(&decompressed).expect("decode check-in/out");
    assert_eq!(visit, decoded);
}

#[test]
fn test_facility_heatmap_roundtrip() {
    let mut slots = Vec::new();
    for day in 0u8..7 {
        for hour in 6u8..22 {
            slots.push(FacilityUtilizationSlot {
                hour,
                day_of_week: day,
                occupancy_count: (hour as u32 * 3 + day as u32 * 7) % 50,
                capacity: 80,
                zone: "Main Gym".to_string(),
            });
        }
    }
    let heatmap = FacilityHeatmap {
        facility_name: "Downtown Fitness Center".to_string(),
        slots,
    };
    let encoded = encode_to_vec(&heatmap).expect("encode facility heatmap");
    let compressed = compress_lz4(&encoded).expect("compress facility heatmap");
    let decompressed = decompress_lz4(&compressed).expect("decompress facility heatmap");
    let (decoded, _): (FacilityHeatmap, usize) =
        decode_from_slice(&decompressed).expect("decode facility heatmap");
    assert_eq!(heatmap, decoded);
}

#[test]
fn test_group_challenge_leaderboard_roundtrip() {
    let challenge = GroupChallenge {
        challenge_id: 9001,
        name: "March Madness Step Challenge".to_string(),
        description: "Walk or run the most steps in March to win prizes".to_string(),
        start_epoch: 1_709_251_200,
        end_epoch: 1_711_929_600,
        leaderboard: vec![
            LeaderboardEntry {
                member_id: 100_003,
                member_name: "Sarah Kim".to_string(),
                score: 485_320,
                rank: 1,
            },
            LeaderboardEntry {
                member_id: 100_018,
                member_name: "Jake Torres".to_string(),
                score: 462_100,
                rank: 2,
            },
            LeaderboardEntry {
                member_id: 100_042,
                member_name: "Mia Chen".to_string(),
                score: 451_800,
                rank: 3,
            },
        ],
    };
    let encoded = encode_to_vec(&challenge).expect("encode group challenge");
    let compressed = compress_lz4(&encoded).expect("compress group challenge");
    let decompressed = decompress_lz4(&compressed).expect("decompress group challenge");
    let (decoded, _): (GroupChallenge, usize) =
        decode_from_slice(&decompressed).expect("decode group challenge");
    assert_eq!(challenge, decoded);
}

#[test]
fn test_injury_incident_report_roundtrip() {
    let report = InjuryIncidentReport {
        incident_id: 7042,
        member_id: 100_088,
        date_epoch: 1_710_400_000,
        location: "Free weights area near squat rack #3".to_string(),
        description: "Member dropped barbell on left foot during deadlift attempt".to_string(),
        severity: InjurySeverity::Moderate,
        equipment_involved: Some("Olympic barbell 20kg".to_string()),
        witness_names: vec!["Coach Rivera".to_string(), "Tom Nguyen".to_string()],
        follow_up_required: true,
    };
    let encoded = encode_to_vec(&report).expect("encode injury report");
    let compressed = compress_lz4(&encoded).expect("compress injury report");
    let decompressed = decompress_lz4(&compressed).expect("decompress injury report");
    let (decoded, _): (InjuryIncidentReport, usize) =
        decode_from_slice(&decompressed).expect("decode injury report");
    assert_eq!(report, decoded);
}

#[test]
fn test_amenity_usage_log_roundtrip() {
    let logs: Vec<AmenityUsageLog> = vec![
        AmenityUsageLog {
            member_id: 100_033,
            amenity: AmenityType::Sauna,
            start_epoch: 1_710_500_000,
            duration_minutes: 20,
            temperature_c: Some(85.0),
        },
        AmenityUsageLog {
            member_id: 100_033,
            amenity: AmenityType::Pool,
            start_epoch: 1_710_501_500,
            duration_minutes: 45,
            temperature_c: Some(28.0),
        },
        AmenityUsageLog {
            member_id: 100_033,
            amenity: AmenityType::ColdPlunge,
            start_epoch: 1_710_504_200,
            duration_minutes: 5,
            temperature_c: Some(4.5),
        },
    ];
    let encoded = encode_to_vec(&logs).expect("encode amenity usage logs");
    let compressed = compress_lz4(&encoded).expect("compress amenity usage logs");
    let decompressed = decompress_lz4(&compressed).expect("decompress amenity usage logs");
    let (decoded, _): (Vec<AmenityUsageLog>, usize) =
        decode_from_slice(&decompressed).expect("decode amenity usage logs");
    assert_eq!(logs, decoded);
}

#[test]
fn test_referral_program_roundtrip() {
    let referrals: Vec<ReferralRecord> = vec![
        ReferralRecord {
            referrer_member_id: 100_001,
            referred_name: "Bob Smith".to_string(),
            referred_email: "bob.smith@example.com".to_string(),
            referral_epoch: 1_709_800_000,
            signup_epoch: Some(1_709_900_000),
            reward_cents: 2500,
            redeemed: true,
        },
        ReferralRecord {
            referrer_member_id: 100_001,
            referred_name: "Carol Davis".to_string(),
            referred_email: "carol.d@example.com".to_string(),
            referral_epoch: 1_710_100_000,
            signup_epoch: None,
            reward_cents: 0,
            redeemed: false,
        },
    ];
    let encoded = encode_to_vec(&referrals).expect("encode referral records");
    let compressed = compress_lz4(&encoded).expect("compress referral records");
    let decompressed = decompress_lz4(&compressed).expect("decompress referral records");
    let (decoded, _): (Vec<ReferralRecord>, usize) =
        decode_from_slice(&decompressed).expect("decode referral records");
    assert_eq!(referrals, decoded);
}

#[test]
fn test_heatmap_compressed_smaller_than_uncompressed() {
    let mut slots = Vec::new();
    for day in 0u8..7 {
        for hour in 5u8..23 {
            slots.push(FacilityUtilizationSlot {
                hour,
                day_of_week: day,
                occupancy_count: 42,
                capacity: 100,
                zone: "Cardio Zone".to_string(),
            });
        }
    }
    let heatmap = FacilityHeatmap {
        facility_name: "Eastside Gym".to_string(),
        slots,
    };
    let encoded = encode_to_vec(&heatmap).expect("encode heatmap for size check");
    let compressed = compress_lz4(&encoded).expect("compress heatmap for size check");
    assert!(
        compressed.len() < encoded.len(),
        "compressed size {} should be smaller than uncompressed size {}",
        compressed.len(),
        encoded.len()
    );
}

#[test]
fn test_repeated_workout_sessions_compressed_smaller() {
    let template_exercise = Exercise {
        name: "Lat Pulldown".to_string(),
        sets: vec![
            ExerciseSet {
                set_number: 1,
                reps: 12,
                weight_kg: 50.0,
                rest_seconds: 60,
            },
            ExerciseSet {
                set_number: 2,
                reps: 10,
                weight_kg: 55.0,
                rest_seconds: 60,
            },
            ExerciseSet {
                set_number: 3,
                reps: 8,
                weight_kg: 60.0,
                rest_seconds: 90,
            },
        ],
        notes: "Focus on mind-muscle connection".to_string(),
    };
    let sessions: Vec<WorkoutSession> = (0..30)
        .map(|i| WorkoutSession {
            session_id: 600_000 + i,
            member_id: 100_001,
            date_epoch: 1_709_000_000 + i * 86400,
            duration_minutes: 60,
            exercises: vec![template_exercise.clone()],
            calories_burned: 380,
        })
        .collect();
    let encoded = encode_to_vec(&sessions).expect("encode repeated sessions");
    let compressed = compress_lz4(&encoded).expect("compress repeated sessions");
    assert!(
        compressed.len() < encoded.len(),
        "compressed {} should be smaller than uncompressed {}",
        compressed.len(),
        encoded.len()
    );
}

#[test]
fn test_multiple_membership_types_roundtrip() {
    let types = vec![
        MembershipType::Basic,
        MembershipType::Premium,
        MembershipType::VIP,
        MembershipType::Student,
        MembershipType::Senior,
        MembershipType::Corporate,
        MembershipType::FamilyPlan,
    ];
    let encoded = encode_to_vec(&types).expect("encode membership types");
    let compressed = compress_lz4(&encoded).expect("compress membership types");
    let decompressed = decompress_lz4(&compressed).expect("decompress membership types");
    let (decoded, _): (Vec<MembershipType>, usize) =
        decode_from_slice(&decompressed).expect("decode membership types");
    assert_eq!(types, decoded);
}

#[test]
fn test_billing_batch_compressed_smaller() {
    let billings: Vec<MembershipBillingCycle> = (0..100)
        .map(|i| MembershipBillingCycle {
            member_id: 100_000 + i,
            cycle_start_epoch: 1_709_000_000,
            cycle_end_epoch: 1_711_600_000,
            amount_cents: 4999,
            status: BillingStatus::Active,
            payment_method: "Auto-pay bank transfer".to_string(),
            auto_renew: true,
            discount_pct: 0,
        })
        .collect();
    let encoded = encode_to_vec(&billings).expect("encode billing batch");
    let compressed = compress_lz4(&encoded).expect("compress billing batch");
    assert!(
        compressed.len() < encoded.len(),
        "compressed billing batch {} should be smaller than uncompressed {}",
        compressed.len(),
        encoded.len()
    );
}

#[test]
fn test_class_schedule_all_types_roundtrip() {
    let schedules = vec![
        ClassSchedule {
            class_id: 401,
            class_type: ClassType::Yoga,
            instructor: "Priya Sharma".to_string(),
            day_of_week: 1,
            start_hour: 7,
            start_minute: 0,
            duration_minutes: 60,
            max_capacity: 20,
            enrolled_member_ids: vec![100_010, 100_011, 100_012],
        },
        ClassSchedule {
            class_id: 402,
            class_type: ClassType::Spinning,
            instructor: "Marcus Lee".to_string(),
            day_of_week: 2,
            start_hour: 6,
            start_minute: 30,
            duration_minutes: 45,
            max_capacity: 30,
            enrolled_member_ids: vec![100_020, 100_021],
        },
        ClassSchedule {
            class_id: 403,
            class_type: ClassType::CrossFit,
            instructor: "Nadia Okonkwo".to_string(),
            day_of_week: 3,
            start_hour: 17,
            start_minute: 0,
            duration_minutes: 60,
            max_capacity: 15,
            enrolled_member_ids: vec![100_030, 100_031, 100_032, 100_033],
        },
        ClassSchedule {
            class_id: 404,
            class_type: ClassType::Boxing,
            instructor: "Tony Reeves".to_string(),
            day_of_week: 4,
            start_hour: 19,
            start_minute: 0,
            duration_minutes: 50,
            max_capacity: 12,
            enrolled_member_ids: vec![100_040],
        },
    ];
    let encoded = encode_to_vec(&schedules).expect("encode class schedules");
    let compressed = compress_lz4(&encoded).expect("compress class schedules");
    let decompressed = decompress_lz4(&compressed).expect("decompress class schedules");
    let (decoded, _): (Vec<ClassSchedule>, usize) =
        decode_from_slice(&decompressed).expect("decode class schedules");
    assert_eq!(schedules, decoded);
}

#[test]
fn test_injury_report_no_equipment_roundtrip() {
    let report = InjuryIncidentReport {
        incident_id: 7100,
        member_id: 100_099,
        date_epoch: 1_710_600_000,
        location: "Locker room corridor".to_string(),
        description: "Slipped on wet floor near showers".to_string(),
        severity: InjurySeverity::Minor,
        equipment_involved: None,
        witness_names: Vec::new(),
        follow_up_required: false,
    };
    let encoded = encode_to_vec(&report).expect("encode injury report without equipment");
    let compressed = compress_lz4(&encoded).expect("compress injury report without equipment");
    let decompressed =
        decompress_lz4(&compressed).expect("decompress injury report without equipment");
    let (decoded, _): (InjuryIncidentReport, usize) =
        decode_from_slice(&decompressed).expect("decode injury report without equipment");
    assert_eq!(report, decoded);
}

#[test]
fn test_full_day_check_ins_compressed_smaller() {
    let check_ins: Vec<CheckInOut> = (0..200)
        .map(|i| CheckInOut {
            member_id: 100_000 + (i % 80),
            check_in_epoch: 1_710_300_000 + i * 300,
            check_out_epoch: 1_710_300_000 + i * 300 + 3600,
            facility_zone: "Main Floor".to_string(),
        })
        .collect();
    let encoded = encode_to_vec(&check_ins).expect("encode daily check-ins");
    let compressed = compress_lz4(&encoded).expect("compress daily check-ins");
    assert!(
        compressed.len() < encoded.len(),
        "compressed check-ins {} should be smaller than uncompressed {}",
        compressed.len(),
        encoded.len()
    );
    let decompressed = decompress_lz4(&compressed).expect("decompress daily check-ins");
    let (decoded, _): (Vec<CheckInOut>, usize) =
        decode_from_slice(&decompressed).expect("decode daily check-ins");
    assert_eq!(check_ins, decoded);
}
