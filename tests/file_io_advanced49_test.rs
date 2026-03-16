#![cfg(feature = "std")]
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

// ── Sports Analytics Domain Types ───────────────────────────────────────────

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct GpsCoordinate {
    latitude: f64,
    longitude: f64,
    altitude_m: f32,
    timestamp_ms: u64,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct AccelerometerReading {
    x: f32,
    y: f32,
    z: f32,
    magnitude: f32,
    timestamp_ms: u64,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct PlayerBiomechanics {
    player_id: u32,
    name: String,
    gps_trace: Vec<GpsCoordinate>,
    accelerometer: Vec<AccelerometerReading>,
    total_distance_m: f64,
    top_speed_kmh: f32,
    sprint_count: u16,
    high_intensity_distance_m: f64,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
enum MatchEventKind {
    Goal,
    Assist,
    YellowCard,
    RedCard,
    Substitution,
    Foul,
    Corner,
    Offside,
    PenaltyAwarded,
    FreeKick,
    ThrowIn,
    GoalKick,
    VarReview,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct MatchEvent {
    minute: u8,
    second: u8,
    added_time_sec: u16,
    kind: MatchEventKind,
    player_id: u32,
    team_id: u16,
    pitch_x: f32,
    pitch_y: f32,
    description: String,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct MatchTimeline {
    match_id: u64,
    home_team: String,
    away_team: String,
    events: Vec<MatchEvent>,
    home_possession_pct: f32,
    away_possession_pct: f32,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct ExpectedGoalsModel {
    shot_id: u64,
    player_id: u32,
    xg_value: f64,
    distance_to_goal_m: f32,
    angle_degrees: f32,
    body_part: String,
    situation: String,
    is_headed: bool,
    is_penalty: bool,
    defenders_in_cone: u8,
    goalkeeper_position_x: f32,
    goalkeeper_position_y: f32,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct PitchControlCell {
    x: f32,
    y: f32,
    home_control: f32,
    away_control: f32,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct PitchControlMap {
    timestamp_ms: u64,
    grid_width: u16,
    grid_height: u16,
    cells: Vec<PitchControlCell>,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct PassingEdge {
    from_player_id: u32,
    to_player_id: u32,
    pass_count: u16,
    total_distance_m: f32,
    completion_rate: f32,
    progressive_count: u16,
    avg_pass_speed_kmh: f32,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct PassingNetworkGraph {
    team_id: u16,
    match_id: u64,
    formation: String,
    edges: Vec<PassingEdge>,
    total_passes: u32,
    pass_accuracy_pct: f32,
    avg_pass_length_m: f32,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
enum InjuryRiskLevel {
    Low,
    Moderate,
    Elevated,
    High,
    Critical,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct MuscleGroupLoad {
    name: String,
    asymmetry_ratio: f32,
    fatigue_index: f32,
    strain_score: f32,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct InjuryRiskAssessment {
    player_id: u32,
    assessment_date: String,
    overall_risk: InjuryRiskLevel,
    risk_score: f64,
    muscle_groups: Vec<MuscleGroupLoad>,
    cumulative_load_7d: f64,
    cumulative_load_28d: f64,
    acute_chronic_ratio: f32,
    previous_injury_factor: f32,
    sleep_quality_score: f32,
    recommendations: Vec<String>,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct ContractValuation {
    player_id: u32,
    player_name: String,
    current_market_value_eur: u64,
    projected_value_1y_eur: u64,
    projected_value_3y_eur: u64,
    age: u8,
    contract_years_remaining: f32,
    wage_weekly_eur: u32,
    performance_index: f64,
    marketability_score: f32,
    comparable_transfers: Vec<u64>,
    replacement_cost_eur: u64,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct ScoutingAttribute {
    name: String,
    raw_score: f32,
    percentile: f32,
    trend: i8,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct DraftScoutingReport {
    prospect_id: u32,
    prospect_name: String,
    position: String,
    age: u8,
    height_cm: u16,
    weight_kg: f32,
    preferred_foot: String,
    attributes: Vec<ScoutingAttribute>,
    overall_grade: f64,
    ceiling_grade: f64,
    floor_grade: f64,
    comparison_player: String,
    notes: String,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct TrainingSession {
    session_id: u64,
    date: String,
    duration_min: u16,
    session_type: String,
    total_distance_m: f64,
    high_speed_running_m: f64,
    sprint_distance_m: f64,
    accelerations: u16,
    decelerations: u16,
    player_load: f64,
    session_rpe: u8,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct TrainingLoadManagement {
    player_id: u32,
    week_number: u8,
    sessions: Vec<TrainingSession>,
    weekly_load: f64,
    monotony: f32,
    strain: f64,
    fitness_level: f64,
    fatigue_level: f64,
    readiness_score: f32,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct HeartRateZone {
    zone_id: u8,
    zone_name: String,
    lower_bpm: u16,
    upper_bpm: u16,
    time_in_zone_sec: u32,
    percentage_of_session: f32,
    calories_burned: f32,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct HeartRateDistribution {
    player_id: u32,
    session_date: String,
    resting_hr: u16,
    max_hr_recorded: u16,
    avg_hr: u16,
    hrv_rmssd: f32,
    zones: Vec<HeartRateZone>,
    recovery_time_min: u16,
    trimp_score: f64,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct SprintProfile {
    sprint_id: u32,
    start_time_ms: u64,
    end_time_ms: u64,
    distance_m: f32,
    max_speed_kmh: f32,
    avg_acceleration: f32,
    max_acceleration: f32,
    mechanical_load: f32,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct TacticalZoneOccupancy {
    zone_name: String,
    time_in_zone_sec: u32,
    entries: u16,
    avg_speed_in_zone_kmh: f32,
    events_in_zone: u16,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct PlayerHeatmap {
    player_id: u32,
    match_id: u64,
    resolution: u16,
    zones: Vec<TacticalZoneOccupancy>,
    centroid_x: f32,
    centroid_y: f32,
    avg_position_x: f32,
    avg_position_y: f32,
    distance_covered_m: f64,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct SetPieceAnalysis {
    set_piece_type: String,
    total_count: u16,
    goals_scored: u16,
    shots_generated: u16,
    xg_total: f64,
    delivery_accuracy_pct: f32,
    avg_players_in_box: f32,
    first_contact_win_pct: f32,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct TeamSetPieces {
    team_id: u16,
    season: String,
    corners: SetPieceAnalysis,
    free_kicks: SetPieceAnalysis,
    throw_ins: SetPieceAnalysis,
    penalties: SetPieceAnalysis,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct PressureEvent {
    timestamp_ms: u64,
    presser_id: u32,
    target_id: u32,
    start_x: f32,
    start_y: f32,
    end_x: f32,
    end_y: f32,
    duration_ms: u16,
    successful: bool,
    caused_turnover: bool,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct PressingIntensityProfile {
    team_id: u16,
    match_id: u64,
    ppda: f32,
    high_press_count: u16,
    mid_press_count: u16,
    low_press_count: u16,
    press_events: Vec<PressureEvent>,
    press_success_rate: f32,
    avg_press_duration_ms: u16,
    regains_in_final_third: u16,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct DefensiveAction {
    action_type: String,
    count: u16,
    success_rate: f32,
    avg_x_position: f32,
    avg_y_position: f32,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct DefensiveProfile {
    player_id: u32,
    match_id: u64,
    actions: Vec<DefensiveAction>,
    duels_won: u16,
    duels_lost: u16,
    aerial_duels_won: u16,
    aerial_duels_lost: u16,
    blocks: u16,
    interceptions: u16,
    clearances: u16,
    defensive_line_height_m: f32,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct GoalkeeperMetrics {
    player_id: u32,
    shots_faced: u16,
    saves: u16,
    goals_conceded: u16,
    xg_against: f64,
    post_shot_xg: f64,
    goals_prevented: f64,
    distribution_accuracy_pct: f32,
    avg_action_height_m: f32,
    sweeper_actions: u16,
    penalty_save_rate: f32,
    high_claims: u16,
    punches: u16,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct ProgressiveCarry {
    carrier_id: u32,
    start_x: f32,
    start_y: f32,
    end_x: f32,
    end_y: f32,
    distance_m: f32,
    speed_kmh: f32,
    defenders_beaten: u8,
    entered_final_third: bool,
    entered_box: bool,
    ended_with_shot: bool,
}

// ── Test 1: GPS tracking biomechanics roundtrip (file) ──────────────────────

#[test]
fn test_player_biomechanics_gps_tracking() {
    let path = temp_dir().join("oxicode_test_sports_biomechanics_gps.bin");
    let bio = PlayerBiomechanics {
        player_id: 7,
        name: "Cristiano Ronaldo".into(),
        gps_trace: vec![
            GpsCoordinate {
                latitude: 40.4530,
                longitude: -3.6883,
                altitude_m: 650.2,
                timestamp_ms: 0,
            },
            GpsCoordinate {
                latitude: 40.4531,
                longitude: -3.6884,
                altitude_m: 650.3,
                timestamp_ms: 100,
            },
            GpsCoordinate {
                latitude: 40.4535,
                longitude: -3.6880,
                altitude_m: 650.1,
                timestamp_ms: 200,
            },
        ],
        accelerometer: vec![
            AccelerometerReading {
                x: 0.12,
                y: 9.78,
                z: 0.05,
                magnitude: 9.79,
                timestamp_ms: 0,
            },
            AccelerometerReading {
                x: 2.5,
                y: 8.1,
                z: 1.2,
                magnitude: 8.67,
                timestamp_ms: 100,
            },
        ],
        total_distance_m: 10842.5,
        top_speed_kmh: 33.8,
        sprint_count: 24,
        high_intensity_distance_m: 2150.3,
    };
    encode_to_file(&bio, &path).expect("encode biomechanics to file");
    let decoded: PlayerBiomechanics =
        decode_from_file(&path).expect("decode biomechanics from file");
    assert_eq!(bio, decoded);
    std::fs::remove_file(&path).expect("cleanup biomechanics file");
}

// ── Test 2: Match event timeline roundtrip (vec) ────────────────────────────

#[test]
fn test_match_event_timeline_roundtrip() {
    let timeline = MatchTimeline {
        match_id: 2024_001_523,
        home_team: "FC Barcelona".into(),
        away_team: "Real Madrid".into(),
        events: vec![
            MatchEvent {
                minute: 23,
                second: 14,
                added_time_sec: 0,
                kind: MatchEventKind::Goal,
                player_id: 10,
                team_id: 1,
                pitch_x: 92.3,
                pitch_y: 34.1,
                description: "Left foot curler from edge of box".into(),
            },
            MatchEvent {
                minute: 23,
                second: 14,
                added_time_sec: 0,
                kind: MatchEventKind::Assist,
                player_id: 8,
                team_id: 1,
                pitch_x: 78.0,
                pitch_y: 50.0,
                description: "Through ball splitting defence".into(),
            },
            MatchEvent {
                minute: 45,
                second: 0,
                added_time_sec: 120,
                kind: MatchEventKind::YellowCard,
                player_id: 4,
                team_id: 2,
                pitch_x: 55.0,
                pitch_y: 40.0,
                description: "Late tackle on counterattack".into(),
            },
            MatchEvent {
                minute: 67,
                second: 30,
                added_time_sec: 0,
                kind: MatchEventKind::Substitution,
                player_id: 11,
                team_id: 1,
                pitch_x: 50.0,
                pitch_y: 0.0,
                description: "Tactical change to 3-5-2".into(),
            },
            MatchEvent {
                minute: 88,
                second: 52,
                added_time_sec: 0,
                kind: MatchEventKind::VarReview,
                player_id: 9,
                team_id: 2,
                pitch_x: 95.0,
                pitch_y: 33.0,
                description: "Offside check on equalizer".into(),
            },
        ],
        home_possession_pct: 62.4,
        away_possession_pct: 37.6,
    };
    let bytes = encode_to_vec(&timeline).expect("encode timeline to vec");
    let (decoded, _): (MatchTimeline, _) =
        decode_from_slice(&bytes).expect("decode timeline from slice");
    assert_eq!(timeline, decoded);
}

// ── Test 3: Expected goals model roundtrip (file) ───────────────────────────

#[test]
fn test_expected_goals_model_file() {
    let path = temp_dir().join("oxicode_test_sports_xg_model.bin");
    let shots = vec![
        ExpectedGoalsModel {
            shot_id: 1,
            player_id: 9,
            xg_value: 0.76,
            distance_to_goal_m: 8.2,
            angle_degrees: 24.5,
            body_part: "right_foot".into(),
            situation: "open_play".into(),
            is_headed: false,
            is_penalty: false,
            defenders_in_cone: 1,
            goalkeeper_position_x: 0.5,
            goalkeeper_position_y: 34.0,
        },
        ExpectedGoalsModel {
            shot_id: 2,
            player_id: 11,
            xg_value: 0.12,
            distance_to_goal_m: 22.0,
            angle_degrees: 8.3,
            body_part: "left_foot".into(),
            situation: "free_kick".into(),
            is_headed: false,
            is_penalty: false,
            defenders_in_cone: 4,
            goalkeeper_position_x: 1.0,
            goalkeeper_position_y: 34.0,
        },
        ExpectedGoalsModel {
            shot_id: 3,
            player_id: 7,
            xg_value: 0.93,
            distance_to_goal_m: 5.0,
            angle_degrees: 38.0,
            body_part: "head".into(),
            situation: "corner".into(),
            is_headed: true,
            is_penalty: false,
            defenders_in_cone: 0,
            goalkeeper_position_x: 3.0,
            goalkeeper_position_y: 36.0,
        },
    ];
    encode_to_file(&shots, &path).expect("encode xG shots to file");
    let decoded: Vec<ExpectedGoalsModel> =
        decode_from_file(&path).expect("decode xG shots from file");
    assert_eq!(shots, decoded);
    std::fs::remove_file(&path).expect("cleanup xG file");
}

// ── Test 4: Pitch control map roundtrip (vec) ───────────────────────────────

#[test]
fn test_pitch_control_map_roundtrip() {
    let mut cells = Vec::new();
    for row in 0..5 {
        for col in 0..8 {
            let home = 0.3 + (row as f32) * 0.05 + (col as f32) * 0.02;
            cells.push(PitchControlCell {
                x: col as f32 * 13.125,
                y: row as f32 * 13.6,
                home_control: home.min(1.0),
                away_control: (1.0 - home).max(0.0),
            });
        }
    }
    let pcm = PitchControlMap {
        timestamp_ms: 1_500_000,
        grid_width: 8,
        grid_height: 5,
        cells,
    };
    let bytes = encode_to_vec(&pcm).expect("encode pitch control map");
    let (decoded, _): (PitchControlMap, _) =
        decode_from_slice(&bytes).expect("decode pitch control map");
    assert_eq!(pcm, decoded);
}

// ── Test 5: Passing network graph roundtrip (file) ──────────────────────────

#[test]
fn test_passing_network_graph_file() {
    let path = temp_dir().join("oxicode_test_sports_passing_network.bin");
    let graph = PassingNetworkGraph {
        team_id: 42,
        match_id: 9900123,
        formation: "4-3-3".into(),
        edges: vec![
            PassingEdge {
                from_player_id: 6,
                to_player_id: 8,
                pass_count: 34,
                total_distance_m: 450.0,
                completion_rate: 0.91,
                progressive_count: 8,
                avg_pass_speed_kmh: 52.0,
            },
            PassingEdge {
                from_player_id: 8,
                to_player_id: 10,
                pass_count: 28,
                total_distance_m: 620.0,
                completion_rate: 0.85,
                progressive_count: 12,
                avg_pass_speed_kmh: 58.0,
            },
            PassingEdge {
                from_player_id: 4,
                to_player_id: 6,
                pass_count: 41,
                total_distance_m: 310.0,
                completion_rate: 0.95,
                progressive_count: 5,
                avg_pass_speed_kmh: 45.0,
            },
            PassingEdge {
                from_player_id: 10,
                to_player_id: 9,
                pass_count: 15,
                total_distance_m: 280.0,
                completion_rate: 0.73,
                progressive_count: 10,
                avg_pass_speed_kmh: 65.0,
            },
        ],
        total_passes: 547,
        pass_accuracy_pct: 88.3,
        avg_pass_length_m: 14.7,
    };
    encode_to_file(&graph, &path).expect("encode passing network to file");
    let decoded: PassingNetworkGraph =
        decode_from_file(&path).expect("decode passing network from file");
    assert_eq!(graph, decoded);
    std::fs::remove_file(&path).expect("cleanup passing network file");
}

// ── Test 6: Injury risk assessment roundtrip (vec) ──────────────────────────

#[test]
fn test_injury_risk_assessment_roundtrip() {
    let assessment = InjuryRiskAssessment {
        player_id: 15,
        assessment_date: "2024-11-15".into(),
        overall_risk: InjuryRiskLevel::Elevated,
        risk_score: 0.67,
        muscle_groups: vec![
            MuscleGroupLoad {
                name: "left_hamstring".into(),
                asymmetry_ratio: 1.18,
                fatigue_index: 0.72,
                strain_score: 0.65,
            },
            MuscleGroupLoad {
                name: "right_hamstring".into(),
                asymmetry_ratio: 0.85,
                fatigue_index: 0.55,
                strain_score: 0.42,
            },
            MuscleGroupLoad {
                name: "left_quadriceps".into(),
                asymmetry_ratio: 1.02,
                fatigue_index: 0.45,
                strain_score: 0.38,
            },
            MuscleGroupLoad {
                name: "right_quadriceps".into(),
                asymmetry_ratio: 0.98,
                fatigue_index: 0.48,
                strain_score: 0.40,
            },
            MuscleGroupLoad {
                name: "left_calf".into(),
                asymmetry_ratio: 1.05,
                fatigue_index: 0.60,
                strain_score: 0.52,
            },
        ],
        cumulative_load_7d: 2450.0,
        cumulative_load_28d: 8920.0,
        acute_chronic_ratio: 1.35,
        previous_injury_factor: 1.2,
        sleep_quality_score: 6.8,
        recommendations: vec![
            "Reduce high-intensity training by 20%".into(),
            "Focus on left hamstring rehabilitation exercises".into(),
            "Add extra recovery session between match days".into(),
        ],
    };
    let bytes = encode_to_vec(&assessment).expect("encode injury risk assessment");
    let (decoded, _): (InjuryRiskAssessment, _) =
        decode_from_slice(&bytes).expect("decode injury risk assessment");
    assert_eq!(assessment, decoded);
}

// ── Test 7: Contract valuation roundtrip (file) ─────────────────────────────

#[test]
fn test_contract_valuation_file() {
    let path = temp_dir().join("oxicode_test_sports_contract_val.bin");
    let valuation = ContractValuation {
        player_id: 10,
        player_name: "Kylian Mbappe".into(),
        current_market_value_eur: 180_000_000,
        projected_value_1y_eur: 195_000_000,
        projected_value_3y_eur: 160_000_000,
        age: 25,
        contract_years_remaining: 3.5,
        wage_weekly_eur: 450_000,
        performance_index: 92.7,
        marketability_score: 9.5,
        comparable_transfers: vec![222_000_000, 180_000_000, 145_000_000, 130_000_000],
        replacement_cost_eur: 200_000_000,
    };
    encode_to_file(&valuation, &path).expect("encode contract valuation to file");
    let decoded: ContractValuation =
        decode_from_file(&path).expect("decode contract valuation from file");
    assert_eq!(valuation, decoded);
    std::fs::remove_file(&path).expect("cleanup contract valuation file");
}

// ── Test 8: Draft scouting report roundtrip (vec) ───────────────────────────

#[test]
fn test_draft_scouting_report_roundtrip() {
    let report = DraftScoutingReport {
        prospect_id: 5501,
        prospect_name: "Lamine Yamal Jr.".into(),
        position: "Right Winger".into(),
        age: 17,
        height_cm: 178,
        weight_kg: 68.5,
        preferred_foot: "left".into(),
        attributes: vec![
            ScoutingAttribute { name: "pace".into(), raw_score: 88.0, percentile: 96.5, trend: 1 },
            ScoutingAttribute { name: "dribbling".into(), raw_score: 91.0, percentile: 98.2, trend: 1 },
            ScoutingAttribute { name: "passing".into(), raw_score: 78.0, percentile: 82.0, trend: 1 },
            ScoutingAttribute { name: "shooting".into(), raw_score: 72.0, percentile: 75.0, trend: 1 },
            ScoutingAttribute { name: "defending".into(), raw_score: 35.0, percentile: 22.0, trend: 0 },
            ScoutingAttribute { name: "physicality".into(), raw_score: 55.0, percentile: 40.0, trend: 1 },
            ScoutingAttribute { name: "vision".into(), raw_score: 85.0, percentile: 93.0, trend: 1 },
        ],
        overall_grade: 87.5,
        ceiling_grade: 95.0,
        floor_grade: 78.0,
        comparison_player: "Lionel Messi".into(),
        notes: "Exceptional close control and acceleration. Needs to add muscle mass for physical duels. Decision-making in final third already elite for age.".into(),
    };
    let bytes = encode_to_vec(&report).expect("encode scouting report");
    let (decoded, _): (DraftScoutingReport, _) =
        decode_from_slice(&bytes).expect("decode scouting report");
    assert_eq!(report, decoded);
}

// ── Test 9: Training load management roundtrip (file) ───────────────────────

#[test]
fn test_training_load_management_file() {
    let path = temp_dir().join("oxicode_test_sports_training_load.bin");
    let load = TrainingLoadManagement {
        player_id: 22,
        week_number: 38,
        sessions: vec![
            TrainingSession {
                session_id: 1001,
                date: "2024-11-11".into(),
                duration_min: 90,
                session_type: "tactical".into(),
                total_distance_m: 6200.0,
                high_speed_running_m: 450.0,
                sprint_distance_m: 120.0,
                accelerations: 35,
                decelerations: 32,
                player_load: 420.5,
                session_rpe: 6,
            },
            TrainingSession {
                session_id: 1002,
                date: "2024-11-12".into(),
                duration_min: 75,
                session_type: "recovery".into(),
                total_distance_m: 4100.0,
                high_speed_running_m: 80.0,
                sprint_distance_m: 0.0,
                accelerations: 12,
                decelerations: 10,
                player_load: 210.0,
                session_rpe: 3,
            },
            TrainingSession {
                session_id: 1003,
                date: "2024-11-13".into(),
                duration_min: 105,
                session_type: "match_prep".into(),
                total_distance_m: 7800.0,
                high_speed_running_m: 680.0,
                sprint_distance_m: 250.0,
                accelerations: 48,
                decelerations: 45,
                player_load: 560.0,
                session_rpe: 7,
            },
        ],
        weekly_load: 1190.5,
        monotony: 1.42,
        strain: 1690.5,
        fitness_level: 72.3,
        fatigue_level: 38.5,
        readiness_score: 7.2,
    };
    encode_to_file(&load, &path).expect("encode training load to file");
    let decoded: TrainingLoadManagement =
        decode_from_file(&path).expect("decode training load from file");
    assert_eq!(load, decoded);
    std::fs::remove_file(&path).expect("cleanup training load file");
}

// ── Test 10: Heart rate zone distribution roundtrip (vec) ───────────────────

#[test]
fn test_heart_rate_distribution_roundtrip() {
    let hr = HeartRateDistribution {
        player_id: 8,
        session_date: "2024-11-14".into(),
        resting_hr: 48,
        max_hr_recorded: 192,
        avg_hr: 156,
        hrv_rmssd: 42.5,
        zones: vec![
            HeartRateZone {
                zone_id: 1,
                zone_name: "recovery".into(),
                lower_bpm: 96,
                upper_bpm: 115,
                time_in_zone_sec: 420,
                percentage_of_session: 7.8,
                calories_burned: 35.0,
            },
            HeartRateZone {
                zone_id: 2,
                zone_name: "aerobic".into(),
                lower_bpm: 116,
                upper_bpm: 134,
                time_in_zone_sec: 1080,
                percentage_of_session: 20.0,
                calories_burned: 120.0,
            },
            HeartRateZone {
                zone_id: 3,
                zone_name: "tempo".into(),
                lower_bpm: 135,
                upper_bpm: 153,
                time_in_zone_sec: 1620,
                percentage_of_session: 30.0,
                calories_burned: 210.0,
            },
            HeartRateZone {
                zone_id: 4,
                zone_name: "threshold".into(),
                lower_bpm: 154,
                upper_bpm: 172,
                time_in_zone_sec: 1350,
                percentage_of_session: 25.0,
                calories_burned: 250.0,
            },
            HeartRateZone {
                zone_id: 5,
                zone_name: "max_effort".into(),
                lower_bpm: 173,
                upper_bpm: 192,
                time_in_zone_sec: 930,
                percentage_of_session: 17.2,
                calories_burned: 195.0,
            },
        ],
        recovery_time_min: 48,
        trimp_score: 285.0,
    };
    let bytes = encode_to_vec(&hr).expect("encode heart rate distribution");
    let (decoded, _): (HeartRateDistribution, _) =
        decode_from_slice(&bytes).expect("decode heart rate distribution");
    assert_eq!(hr, decoded);
}

// ── Test 11: Sprint profile batch roundtrip (file) ──────────────────────────

#[test]
fn test_sprint_profile_batch_file() {
    let path = temp_dir().join("oxicode_test_sports_sprint_profiles.bin");
    let sprints: Vec<SprintProfile> = (0..12)
        .map(|i| SprintProfile {
            sprint_id: i,
            start_time_ms: i as u64 * 300_000 + 10_000,
            end_time_ms: i as u64 * 300_000 + 10_000 + 2500 + (i as u64 % 3) * 500,
            distance_m: 25.0 + (i as f32) * 2.5,
            max_speed_kmh: 30.0 + (i as f32) * 0.4,
            avg_acceleration: 3.2 + (i as f32) * 0.1,
            max_acceleration: 5.5 + (i as f32) * 0.15,
            mechanical_load: 180.0 + (i as f32) * 12.0,
        })
        .collect();
    encode_to_file(&sprints, &path).expect("encode sprint profiles to file");
    let decoded: Vec<SprintProfile> =
        decode_from_file(&path).expect("decode sprint profiles from file");
    assert_eq!(sprints, decoded);
    std::fs::remove_file(&path).expect("cleanup sprint profiles file");
}

// ── Test 12: Player heatmap roundtrip (vec) ─────────────────────────────────

#[test]
fn test_player_heatmap_roundtrip() {
    let heatmap = PlayerHeatmap {
        player_id: 6,
        match_id: 20241115001,
        resolution: 10,
        zones: vec![
            TacticalZoneOccupancy {
                zone_name: "defensive_third_left".into(),
                time_in_zone_sec: 420,
                entries: 15,
                avg_speed_in_zone_kmh: 8.2,
                events_in_zone: 12,
            },
            TacticalZoneOccupancy {
                zone_name: "defensive_third_center".into(),
                time_in_zone_sec: 890,
                entries: 28,
                avg_speed_in_zone_kmh: 7.5,
                events_in_zone: 22,
            },
            TacticalZoneOccupancy {
                zone_name: "middle_third_left".into(),
                time_in_zone_sec: 1200,
                entries: 42,
                avg_speed_in_zone_kmh: 10.3,
                events_in_zone: 35,
            },
            TacticalZoneOccupancy {
                zone_name: "middle_third_center".into(),
                time_in_zone_sec: 1650,
                entries: 55,
                avg_speed_in_zone_kmh: 9.8,
                events_in_zone: 48,
            },
            TacticalZoneOccupancy {
                zone_name: "attacking_third_left".into(),
                time_in_zone_sec: 540,
                entries: 18,
                avg_speed_in_zone_kmh: 12.5,
                events_in_zone: 15,
            },
            TacticalZoneOccupancy {
                zone_name: "attacking_third_center".into(),
                time_in_zone_sec: 300,
                entries: 10,
                avg_speed_in_zone_kmh: 14.0,
                events_in_zone: 8,
            },
        ],
        centroid_x: 42.5,
        centroid_y: 35.0,
        avg_position_x: 40.8,
        avg_position_y: 33.2,
        distance_covered_m: 11250.0,
    };
    let bytes = encode_to_vec(&heatmap).expect("encode player heatmap");
    let (decoded, _): (PlayerHeatmap, _) =
        decode_from_slice(&bytes).expect("decode player heatmap");
    assert_eq!(heatmap, decoded);
}

// ── Test 13: Team set pieces analysis roundtrip (file) ──────────────────────

#[test]
fn test_team_set_pieces_file() {
    let path = temp_dir().join("oxicode_test_sports_set_pieces.bin");
    let set_pieces = TeamSetPieces {
        team_id: 7,
        season: "2024-2025".into(),
        corners: SetPieceAnalysis {
            set_piece_type: "corner".into(),
            total_count: 142,
            goals_scored: 8,
            shots_generated: 38,
            xg_total: 6.42,
            delivery_accuracy_pct: 32.5,
            avg_players_in_box: 6.2,
            first_contact_win_pct: 41.0,
        },
        free_kicks: SetPieceAnalysis {
            set_piece_type: "direct_free_kick".into(),
            total_count: 55,
            goals_scored: 3,
            shots_generated: 22,
            xg_total: 2.85,
            delivery_accuracy_pct: 45.0,
            avg_players_in_box: 4.1,
            first_contact_win_pct: 38.0,
        },
        throw_ins: SetPieceAnalysis {
            set_piece_type: "throw_in_attacking".into(),
            total_count: 320,
            goals_scored: 1,
            shots_generated: 12,
            xg_total: 0.95,
            delivery_accuracy_pct: 78.0,
            avg_players_in_box: 2.5,
            first_contact_win_pct: 55.0,
        },
        penalties: SetPieceAnalysis {
            set_piece_type: "penalty".into(),
            total_count: 8,
            goals_scored: 6,
            shots_generated: 8,
            xg_total: 6.08,
            delivery_accuracy_pct: 100.0,
            avg_players_in_box: 1.0,
            first_contact_win_pct: 100.0,
        },
    };
    encode_to_file(&set_pieces, &path).expect("encode set pieces to file");
    let decoded: TeamSetPieces = decode_from_file(&path).expect("decode set pieces from file");
    assert_eq!(set_pieces, decoded);
    std::fs::remove_file(&path).expect("cleanup set pieces file");
}

// ── Test 14: Pressing intensity profile roundtrip (vec) ─────────────────────

#[test]
fn test_pressing_intensity_roundtrip() {
    let pressing = PressingIntensityProfile {
        team_id: 3,
        match_id: 88990012,
        ppda: 7.8,
        high_press_count: 42,
        mid_press_count: 65,
        low_press_count: 28,
        press_events: vec![
            PressureEvent {
                timestamp_ms: 120_000,
                presser_id: 9,
                target_id: 4,
                start_x: 85.0,
                start_y: 40.0,
                end_x: 82.0,
                end_y: 38.0,
                duration_ms: 3200,
                successful: true,
                caused_turnover: true,
            },
            PressureEvent {
                timestamp_ms: 540_000,
                presser_id: 7,
                target_id: 6,
                start_x: 72.0,
                start_y: 55.0,
                end_x: 70.0,
                end_y: 52.0,
                duration_ms: 2800,
                successful: false,
                caused_turnover: false,
            },
            PressureEvent {
                timestamp_ms: 2_700_000,
                presser_id: 11,
                target_id: 3,
                start_x: 90.0,
                start_y: 20.0,
                end_x: 88.0,
                end_y: 22.0,
                duration_ms: 1500,
                successful: true,
                caused_turnover: true,
            },
        ],
        press_success_rate: 0.42,
        avg_press_duration_ms: 2500,
        regains_in_final_third: 18,
    };
    let bytes = encode_to_vec(&pressing).expect("encode pressing intensity");
    let (decoded, _): (PressingIntensityProfile, _) =
        decode_from_slice(&bytes).expect("decode pressing intensity");
    assert_eq!(pressing, decoded);
}

// ── Test 15: Defensive profile roundtrip (file) ─────────────────────────────

#[test]
fn test_defensive_profile_file() {
    let path = temp_dir().join("oxicode_test_sports_defensive.bin");
    let defense = DefensiveProfile {
        player_id: 4,
        match_id: 77001234,
        actions: vec![
            DefensiveAction {
                action_type: "tackle".into(),
                count: 5,
                success_rate: 0.80,
                avg_x_position: 35.0,
                avg_y_position: 34.0,
            },
            DefensiveAction {
                action_type: "interception".into(),
                count: 3,
                success_rate: 1.0,
                avg_x_position: 40.0,
                avg_y_position: 30.0,
            },
            DefensiveAction {
                action_type: "block".into(),
                count: 2,
                success_rate: 1.0,
                avg_x_position: 20.0,
                avg_y_position: 34.0,
            },
            DefensiveAction {
                action_type: "clearance".into(),
                count: 7,
                success_rate: 0.86,
                avg_x_position: 15.0,
                avg_y_position: 34.0,
            },
        ],
        duels_won: 12,
        duels_lost: 4,
        aerial_duels_won: 6,
        aerial_duels_lost: 2,
        blocks: 2,
        interceptions: 3,
        clearances: 7,
        defensive_line_height_m: 38.5,
    };
    encode_to_file(&defense, &path).expect("encode defensive profile to file");
    let decoded: DefensiveProfile =
        decode_from_file(&path).expect("decode defensive profile from file");
    assert_eq!(defense, decoded);
    std::fs::remove_file(&path).expect("cleanup defensive profile file");
}

// ── Test 16: Goalkeeper metrics roundtrip (vec) ─────────────────────────────

#[test]
fn test_goalkeeper_metrics_roundtrip() {
    let gk = GoalkeeperMetrics {
        player_id: 1,
        shots_faced: 5,
        saves: 3,
        goals_conceded: 2,
        xg_against: 2.85,
        post_shot_xg: 2.10,
        goals_prevented: 0.85,
        distribution_accuracy_pct: 72.5,
        avg_action_height_m: 1.35,
        sweeper_actions: 4,
        penalty_save_rate: 0.33,
        high_claims: 3,
        punches: 1,
    };
    let bytes = encode_to_vec(&gk).expect("encode goalkeeper metrics");
    let (decoded, _): (GoalkeeperMetrics, _) =
        decode_from_slice(&bytes).expect("decode goalkeeper metrics");
    assert_eq!(gk, decoded);
}

// ── Test 17: Progressive carries batch roundtrip (file) ─────────────────────

#[test]
fn test_progressive_carries_file() {
    let path = temp_dir().join("oxicode_test_sports_prog_carries.bin");
    let carries = vec![
        ProgressiveCarry {
            carrier_id: 10,
            start_x: 45.0,
            start_y: 34.0,
            end_x: 70.0,
            end_y: 30.0,
            distance_m: 26.5,
            speed_kmh: 22.0,
            defenders_beaten: 2,
            entered_final_third: true,
            entered_box: false,
            ended_with_shot: false,
        },
        ProgressiveCarry {
            carrier_id: 10,
            start_x: 55.0,
            start_y: 50.0,
            end_x: 88.0,
            end_y: 42.0,
            distance_m: 34.2,
            speed_kmh: 28.5,
            defenders_beaten: 3,
            entered_final_third: true,
            entered_box: true,
            ended_with_shot: true,
        },
        ProgressiveCarry {
            carrier_id: 7,
            start_x: 60.0,
            start_y: 10.0,
            end_x: 78.0,
            end_y: 25.0,
            distance_m: 22.0,
            speed_kmh: 30.2,
            defenders_beaten: 1,
            entered_final_third: true,
            entered_box: false,
            ended_with_shot: false,
        },
    ];
    encode_to_file(&carries, &path).expect("encode progressive carries to file");
    let decoded: Vec<ProgressiveCarry> =
        decode_from_file(&path).expect("decode progressive carries from file");
    assert_eq!(carries, decoded);
    std::fs::remove_file(&path).expect("cleanup progressive carries file");
}

// ── Test 18: Multiple injury risk levels enum roundtrip (vec) ───────────────

#[test]
fn test_injury_risk_level_all_variants() {
    let levels = vec![
        InjuryRiskLevel::Low,
        InjuryRiskLevel::Moderate,
        InjuryRiskLevel::Elevated,
        InjuryRiskLevel::High,
        InjuryRiskLevel::Critical,
    ];
    let bytes = encode_to_vec(&levels).expect("encode injury risk levels");
    let (decoded, _): (Vec<InjuryRiskLevel>, _) =
        decode_from_slice(&bytes).expect("decode injury risk levels");
    assert_eq!(levels, decoded);
}

// ── Test 19: Match event kind all variants roundtrip (file) ─────────────────

#[test]
fn test_match_event_kind_all_variants_file() {
    let path = temp_dir().join("oxicode_test_sports_event_kinds.bin");
    let kinds = vec![
        MatchEventKind::Goal,
        MatchEventKind::Assist,
        MatchEventKind::YellowCard,
        MatchEventKind::RedCard,
        MatchEventKind::Substitution,
        MatchEventKind::Foul,
        MatchEventKind::Corner,
        MatchEventKind::Offside,
        MatchEventKind::PenaltyAwarded,
        MatchEventKind::FreeKick,
        MatchEventKind::ThrowIn,
        MatchEventKind::GoalKick,
        MatchEventKind::VarReview,
    ];
    encode_to_file(&kinds, &path).expect("encode all match event kinds to file");
    let decoded: Vec<MatchEventKind> =
        decode_from_file(&path).expect("decode all match event kinds from file");
    assert_eq!(kinds, decoded);
    std::fs::remove_file(&path).expect("cleanup event kinds file");
}

// ── Test 20: Combined multi-player scouting reports roundtrip (vec) ─────────

#[test]
fn test_multi_player_scouting_batch_roundtrip() {
    let reports: Vec<DraftScoutingReport> = (0..5)
        .map(|i| DraftScoutingReport {
            prospect_id: 6000 + i,
            prospect_name: format!("Prospect_{}", i),
            position: match i % 4 {
                0 => "Striker".into(),
                1 => "Midfielder".into(),
                2 => "Defender".into(),
                _ => "Goalkeeper".into(),
            },
            age: 18 + (i as u8 % 4),
            height_cm: 175 + (i as u16 % 15),
            weight_kg: 70.0 + (i as f32) * 2.5,
            preferred_foot: if i % 2 == 0 {
                "right".into()
            } else {
                "left".into()
            },
            attributes: vec![
                ScoutingAttribute {
                    name: "technique".into(),
                    raw_score: 70.0 + (i as f32) * 3.0,
                    percentile: 60.0 + (i as f32) * 5.0,
                    trend: if i % 3 == 0 { 1 } else { 0 },
                },
                ScoutingAttribute {
                    name: "work_rate".into(),
                    raw_score: 75.0 + (i as f32) * 2.0,
                    percentile: 65.0 + (i as f32) * 4.0,
                    trend: 1,
                },
            ],
            overall_grade: 72.0 + (i as f64) * 3.5,
            ceiling_grade: 85.0 + (i as f64) * 2.0,
            floor_grade: 65.0 + (i as f64) * 2.5,
            comparison_player: format!("Legend_{}", i),
            notes: format!("Prospect {} shows good potential in key areas", i),
        })
        .collect();
    let bytes = encode_to_vec(&reports).expect("encode multi-player scouting reports");
    let (decoded, _): (Vec<DraftScoutingReport>, _) =
        decode_from_slice(&bytes).expect("decode multi-player scouting reports");
    assert_eq!(reports, decoded);
}

// ── Test 21: Full season training load history roundtrip (file) ──────────────

#[test]
fn test_full_season_training_load_file() {
    let path = temp_dir().join("oxicode_test_sports_season_load.bin");
    let weekly_loads: Vec<TrainingLoadManagement> = (1..=4)
        .map(|week| TrainingLoadManagement {
            player_id: 14,
            week_number: week,
            sessions: (0..3)
                .map(|s| TrainingSession {
                    session_id: (week as u64) * 100 + s,
                    date: format!(
                        "2024-{:02}-{:02}",
                        8 + (week / 5),
                        (week * 3 + s as u8) % 28 + 1
                    ),
                    duration_min: 60 + (s as u16) * 15,
                    session_type: match s % 3 {
                        0 => "strength".into(),
                        1 => "tactical".into(),
                        _ => "recovery".into(),
                    },
                    total_distance_m: 4000.0 + (s as f64) * 1500.0,
                    high_speed_running_m: 200.0 + (s as f64) * 150.0,
                    sprint_distance_m: 50.0 + (s as f64) * 80.0,
                    accelerations: 20 + (s as u16) * 10,
                    decelerations: 18 + (s as u16) * 9,
                    player_load: 250.0 + (s as f64) * 100.0,
                    session_rpe: 4 + (s as u8 % 4),
                })
                .collect(),
            weekly_load: 750.0 + (week as f64) * 50.0,
            monotony: 1.1 + (week as f32) * 0.05,
            strain: 825.0 + (week as f64) * 60.0,
            fitness_level: 60.0 + (week as f64) * 2.5,
            fatigue_level: 30.0 + (week as f64) * 1.5,
            readiness_score: 7.0 + (week as f32) * 0.2,
        })
        .collect();
    encode_to_file(&weekly_loads, &path).expect("encode season training load to file");
    let decoded: Vec<TrainingLoadManagement> =
        decode_from_file(&path).expect("decode season training load from file");
    assert_eq!(weekly_loads, decoded);
    std::fs::remove_file(&path).expect("cleanup season training load file");
}

// ── Test 22: Encoded size verification for compact binary format ────────────

#[test]
fn test_encoded_size_compact_verification() {
    let simple_event = MatchEvent {
        minute: 90,
        second: 0,
        added_time_sec: 180,
        kind: MatchEventKind::Goal,
        player_id: 9,
        team_id: 1,
        pitch_x: 94.0,
        pitch_y: 34.0,
        description: "Injury time winner".into(),
    };
    let bytes = encode_to_vec(&simple_event).expect("encode match event for size check");
    let (decoded, consumed): (MatchEvent, usize) =
        decode_from_slice(&bytes).expect("decode match event for size check");
    assert_eq!(simple_event, decoded);
    assert_eq!(
        consumed,
        bytes.len(),
        "consumed bytes should equal encoded length"
    );
    assert!(
        bytes.len() < 200,
        "compact binary should be under 200 bytes for a single event"
    );

    let empty_timeline = MatchTimeline {
        match_id: 0,
        home_team: String::new(),
        away_team: String::new(),
        events: Vec::new(),
        home_possession_pct: 0.0,
        away_possession_pct: 0.0,
    };
    let empty_bytes = encode_to_vec(&empty_timeline).expect("encode empty timeline");
    let (decoded_empty, _): (MatchTimeline, _) =
        decode_from_slice(&empty_bytes).expect("decode empty timeline");
    assert_eq!(empty_timeline, decoded_empty);
    assert!(
        empty_bytes.len() < 50,
        "empty timeline should encode very compactly"
    );
}
