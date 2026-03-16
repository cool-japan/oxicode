//! Advanced property-based tests (set 76) — Sports Analytics & Performance Science.
//!
//! 22 top-level `#[test]` functions, each containing exactly one `proptest!` block.
//! Covers player tracking (GPS, speed, acceleration), biomechanical measurements,
//! match event logs, expected goals (xG), training load monitoring, injury risk,
//! team formations, pitch control, set pieces, scouting, contract valuation,
//! fan engagement, venue capacity, broadcast viewership, and referee decisions.

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

/// GPS tracking sample for a player on the pitch.
#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct PlayerTrackingData {
    player_id: u32,
    timestamp_ms: u64,
    latitude: f64,
    longitude: f64,
    speed_m_per_s: f32,
    acceleration_m_per_s2: f32,
    heart_rate_bpm: u16,
    distance_covered_m: f32,
}

/// Biomechanical measurement from a motion capture session.
#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct BiomechanicalMeasurement {
    session_id: u32,
    joint_name: String,
    flexion_angle_deg: f32,
    extension_angle_deg: f32,
    ground_reaction_force_n: f32,
    angular_velocity_deg_per_s: f32,
    moment_nm: f32,
}

/// Match event types for a football/soccer match.
#[derive(Debug, PartialEq, Clone, Encode, Decode)]
enum MatchEvent {
    Goal {
        minute: u8,
        player_id: u32,
        xg_value: f32,
        body_part: String,
    },
    Shot {
        minute: u8,
        player_id: u32,
        xg_value: f32,
        on_target: bool,
    },
    Pass {
        minute: u8,
        from_player: u32,
        to_player: u32,
        distance_m: f32,
        successful: bool,
    },
    Tackle {
        minute: u8,
        player_id: u32,
        successful: bool,
        foul_committed: bool,
    },
}

/// Expected goals model output for a single shot.
#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct ExpectedGoalsEntry {
    shot_id: u64,
    distance_to_goal_m: f32,
    angle_to_goal_deg: f32,
    defender_count: u8,
    is_header: bool,
    xg_probability: f32,
    cumulative_xg: f32,
}

/// Training load record for a single session.
#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct TrainingLoadRecord {
    athlete_id: u32,
    session_date_epoch: u64,
    rpe_score: u8,
    duration_min: u16,
    hr_zone_1_min: u16,
    hr_zone_2_min: u16,
    hr_zone_3_min: u16,
    hr_zone_4_min: u16,
    hr_zone_5_min: u16,
    trimp_score: f32,
    session_type: String,
}

/// Injury risk assessment for a player.
#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct InjuryRiskScore {
    player_id: u32,
    assessment_epoch: u64,
    acute_load: f32,
    chronic_load: f32,
    acwr_ratio: f32,
    risk_category: InjuryRiskCategory,
    sleep_quality_score: u8,
    previous_injuries: u16,
}

/// Risk category for injury assessment.
#[derive(Debug, PartialEq, Clone, Encode, Decode)]
enum InjuryRiskCategory {
    Low,
    Moderate,
    High,
    Critical,
}

/// A team formation snapshot.
#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct TeamFormation {
    team_id: u32,
    formation_label: String,
    avg_x_positions: Vec<f32>,
    avg_y_positions: Vec<f32>,
    compactness_m: f32,
    width_m: f32,
    depth_m: f32,
}

/// Pitch control model cell.
#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct PitchControlCell {
    grid_x: u16,
    grid_y: u16,
    home_control: f32,
    away_control: f32,
    dominant_player_id: u32,
    influence_radius_m: f32,
}

/// Set piece analysis record.
#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct SetPieceAnalysis {
    match_id: u64,
    set_piece_type: SetPieceType,
    delivery_zone: u8,
    attacking_players_in_box: u8,
    defending_players_in_box: u8,
    resulted_in_shot: bool,
    resulted_in_goal: bool,
    xg_from_set_piece: f32,
}

/// Type of set piece.
#[derive(Debug, PartialEq, Clone, Encode, Decode)]
enum SetPieceType {
    Corner,
    FreeKickDirect,
    FreeKickIndirect,
    ThrowIn,
    Penalty,
    GoalKick,
}

/// Draft scouting metrics for a prospect.
#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct DraftScoutingMetrics {
    prospect_id: u32,
    position: String,
    sprint_speed_percentile: f32,
    agility_score: f32,
    endurance_vo2max: f32,
    technical_rating: f32,
    tactical_awareness: f32,
    mental_resilience: f32,
    overall_grade: f32,
}

/// Contract valuation model output.
#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct ContractValuation {
    player_id: u32,
    age_years: u8,
    remaining_contract_months: u16,
    market_value_eur: u64,
    wage_weekly_eur: u32,
    performance_index: f32,
    marketability_index: f32,
    injury_discount_pct: f32,
}

/// Fan engagement metrics for a single match day.
#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct FanEngagementMetrics {
    match_id: u64,
    social_media_mentions: u64,
    app_active_users: u32,
    merchandise_sales_units: u32,
    sentiment_score: f32,
    peak_concurrent_chat_users: u32,
    hashtag_impressions: u64,
}

/// Venue capacity management snapshot.
#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct VenueCapacity {
    venue_id: u32,
    total_capacity: u32,
    tickets_sold: u32,
    attendance_actual: u32,
    vip_occupancy_pct: f32,
    concession_revenue_eur: u64,
    entry_gate_throughput_per_min: u16,
    evacuation_time_estimate_s: u32,
}

/// Broadcast viewership statistics.
#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct BroadcastViewership {
    match_id: u64,
    region_code: String,
    live_viewers: u64,
    peak_viewers: u64,
    avg_watch_time_min: f32,
    ad_impressions: u64,
    stream_bitrate_kbps: u32,
    buffering_ratio_pct: f32,
}

/// Referee decision analysis record.
#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct RefereeDecision {
    match_id: u64,
    minute: u8,
    decision_type: RefereeDecisionType,
    var_involved: bool,
    var_overturned: bool,
    correct_per_panel: bool,
    confidence_pct: f32,
}

/// Type of referee decision.
#[derive(Debug, PartialEq, Clone, Encode, Decode)]
enum RefereeDecisionType {
    FoulGiven,
    FoulNotGiven,
    YellowCard,
    RedCard,
    PenaltyAwarded,
    PenaltyNotAwarded,
    OffsideGiven,
    OffsideNotGiven,
    GoalAllowed,
    GoalDisallowed,
}

/// A batch of pitch control cells for a single frame.
#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct PitchControlFrame {
    frame_id: u64,
    timestamp_ms: u64,
    cells: Vec<PitchControlCell>,
}

/// Aggregated match statistics.
#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct MatchStatsSummary {
    match_id: u64,
    home_possession_pct: f32,
    away_possession_pct: f32,
    home_shots: u16,
    away_shots: u16,
    home_xg: f32,
    away_xg: f32,
    home_passes_completed: u16,
    away_passes_completed: u16,
    home_tackles_won: u16,
    away_tackles_won: u16,
}

// ── prop_compose! strategies ─────────────────────────────────────────────────

prop_compose! {
    fn arb_player_tracking()(
        player_id in any::<u32>(),
        timestamp_ms in any::<u64>(),
        latitude in -90.0f64..90.0,
        longitude in -180.0f64..180.0,
        speed_m_per_s in 0.0f32..12.0,
        acceleration_m_per_s2 in -6.0f32..6.0,
        heart_rate_bpm in 40u16..220,
        distance_covered_m in 0.0f32..15000.0,
    ) -> PlayerTrackingData {
        PlayerTrackingData {
            player_id, timestamp_ms, latitude, longitude,
            speed_m_per_s, acceleration_m_per_s2, heart_rate_bpm,
            distance_covered_m,
        }
    }
}

prop_compose! {
    fn arb_biomechanical()(
        session_id in any::<u32>(),
        joint_name in "(knee|hip|ankle|shoulder|elbow|wrist)",
        flexion_angle_deg in 0.0f32..180.0,
        extension_angle_deg in 0.0f32..180.0,
        ground_reaction_force_n in 0.0f32..5000.0,
        angular_velocity_deg_per_s in -500.0f32..500.0,
        moment_nm in -300.0f32..300.0,
    ) -> BiomechanicalMeasurement {
        BiomechanicalMeasurement {
            session_id, joint_name, flexion_angle_deg, extension_angle_deg,
            ground_reaction_force_n, angular_velocity_deg_per_s, moment_nm,
        }
    }
}

prop_compose! {
    fn arb_match_event_goal()(
        minute in 0u8..120,
        player_id in any::<u32>(),
        xg_value in 0.0f32..1.0,
        body_part in "(left_foot|right_foot|head|other)",
    ) -> MatchEvent {
        MatchEvent::Goal { minute, player_id, xg_value, body_part }
    }
}

prop_compose! {
    fn arb_match_event_shot()(
        minute in 0u8..120,
        player_id in any::<u32>(),
        xg_value in 0.0f32..1.0,
        on_target in any::<bool>(),
    ) -> MatchEvent {
        MatchEvent::Shot { minute, player_id, xg_value, on_target }
    }
}

prop_compose! {
    fn arb_match_event_pass()(
        minute in 0u8..120,
        from_player in any::<u32>(),
        to_player in any::<u32>(),
        distance_m in 1.0f32..80.0,
        successful in any::<bool>(),
    ) -> MatchEvent {
        MatchEvent::Pass { minute, from_player, to_player, distance_m, successful }
    }
}

prop_compose! {
    fn arb_match_event_tackle()(
        minute in 0u8..120,
        player_id in any::<u32>(),
        successful in any::<bool>(),
        foul_committed in any::<bool>(),
    ) -> MatchEvent {
        MatchEvent::Tackle { minute, player_id, successful, foul_committed }
    }
}

fn arb_match_event() -> impl Strategy<Value = MatchEvent> {
    prop_oneof![
        arb_match_event_goal(),
        arb_match_event_shot(),
        arb_match_event_pass(),
        arb_match_event_tackle(),
    ]
}

prop_compose! {
    fn arb_xg_entry()(
        shot_id in any::<u64>(),
        distance_to_goal_m in 1.0f32..50.0,
        angle_to_goal_deg in 0.0f32..180.0,
        defender_count in 0u8..11,
        is_header in any::<bool>(),
        xg_probability in 0.0f32..1.0,
        cumulative_xg in 0.0f32..10.0,
    ) -> ExpectedGoalsEntry {
        ExpectedGoalsEntry {
            shot_id, distance_to_goal_m, angle_to_goal_deg,
            defender_count, is_header, xg_probability, cumulative_xg,
        }
    }
}

prop_compose! {
    fn arb_training_load()(
        athlete_id in any::<u32>(),
        session_date_epoch in any::<u64>(),
        rpe_score in 1u8..10,
        duration_min in 15u16..180,
        hr_zone_1_min in 0u16..60,
        hr_zone_2_min in 0u16..60,
        hr_zone_3_min in 0u16..60,
        hr_zone_4_min in 0u16..30,
        hr_zone_5_min in 0u16..15,
        trimp_score in 0.0f32..500.0,
        session_type in "(endurance|strength|speed|recovery|match)",
    ) -> TrainingLoadRecord {
        TrainingLoadRecord {
            athlete_id, session_date_epoch, rpe_score, duration_min,
            hr_zone_1_min, hr_zone_2_min, hr_zone_3_min,
            hr_zone_4_min, hr_zone_5_min, trimp_score, session_type,
        }
    }
}

fn arb_risk_category() -> impl Strategy<Value = InjuryRiskCategory> {
    prop_oneof![
        Just(InjuryRiskCategory::Low),
        Just(InjuryRiskCategory::Moderate),
        Just(InjuryRiskCategory::High),
        Just(InjuryRiskCategory::Critical),
    ]
}

prop_compose! {
    fn arb_injury_risk()(
        player_id in any::<u32>(),
        assessment_epoch in any::<u64>(),
        acute_load in 0.0f32..2000.0,
        chronic_load in 0.0f32..2000.0,
        acwr_ratio in 0.2f32..2.5,
        risk_category in arb_risk_category(),
        sleep_quality_score in 1u8..10,
        previous_injuries in 0u16..20,
    ) -> InjuryRiskScore {
        InjuryRiskScore {
            player_id, assessment_epoch, acute_load, chronic_load,
            acwr_ratio, risk_category, sleep_quality_score, previous_injuries,
        }
    }
}

prop_compose! {
    fn arb_team_formation()(
        team_id in any::<u32>(),
        formation_label in "(4-3-3|4-4-2|3-5-2|4-2-3-1|5-3-2|3-4-3)",
        positions_x in proptest::collection::vec(0.0f32..105.0, 11..=11),
        positions_y in proptest::collection::vec(0.0f32..68.0, 11..=11),
        compactness_m in 10.0f32..50.0,
        width_m in 20.0f32..68.0,
        depth_m in 15.0f32..50.0,
    ) -> TeamFormation {
        TeamFormation {
            team_id, formation_label,
            avg_x_positions: positions_x,
            avg_y_positions: positions_y,
            compactness_m, width_m, depth_m,
        }
    }
}

prop_compose! {
    fn arb_pitch_control_cell()(
        grid_x in 0u16..105,
        grid_y in 0u16..68,
        home_control in 0.0f32..1.0,
        away_control in 0.0f32..1.0,
        dominant_player_id in any::<u32>(),
        influence_radius_m in 1.0f32..15.0,
    ) -> PitchControlCell {
        PitchControlCell {
            grid_x, grid_y, home_control, away_control,
            dominant_player_id, influence_radius_m,
        }
    }
}

fn arb_set_piece_type() -> impl Strategy<Value = SetPieceType> {
    prop_oneof![
        Just(SetPieceType::Corner),
        Just(SetPieceType::FreeKickDirect),
        Just(SetPieceType::FreeKickIndirect),
        Just(SetPieceType::ThrowIn),
        Just(SetPieceType::Penalty),
        Just(SetPieceType::GoalKick),
    ]
}

prop_compose! {
    fn arb_set_piece()(
        match_id in any::<u64>(),
        set_piece_type in arb_set_piece_type(),
        delivery_zone in 1u8..6,
        attacking_players_in_box in 0u8..11,
        defending_players_in_box in 0u8..11,
        resulted_in_shot in any::<bool>(),
        resulted_in_goal in any::<bool>(),
        xg_from_set_piece in 0.0f32..1.0,
    ) -> SetPieceAnalysis {
        SetPieceAnalysis {
            match_id, set_piece_type, delivery_zone,
            attacking_players_in_box, defending_players_in_box,
            resulted_in_shot, resulted_in_goal, xg_from_set_piece,
        }
    }
}

prop_compose! {
    fn arb_scouting()(
        prospect_id in any::<u32>(),
        position in "(GK|CB|FB|CM|AM|WG|ST)",
        sprint_speed_percentile in 0.0f32..100.0,
        agility_score in 0.0f32..10.0,
        endurance_vo2max in 30.0f32..80.0,
        technical_rating in 0.0f32..10.0,
        tactical_awareness in 0.0f32..10.0,
        mental_resilience in 0.0f32..10.0,
        overall_grade in 0.0f32..100.0,
    ) -> DraftScoutingMetrics {
        DraftScoutingMetrics {
            prospect_id, position, sprint_speed_percentile,
            agility_score, endurance_vo2max, technical_rating,
            tactical_awareness, mental_resilience, overall_grade,
        }
    }
}

prop_compose! {
    fn arb_contract_valuation()(
        player_id in any::<u32>(),
        age_years in 16u8..42,
        remaining_contract_months in 0u16..72,
        market_value_eur in 0u64..200_000_000,
        wage_weekly_eur in 1000u32..500_000,
        performance_index in 0.0f32..10.0,
        marketability_index in 0.0f32..10.0,
        injury_discount_pct in 0.0f32..50.0,
    ) -> ContractValuation {
        ContractValuation {
            player_id, age_years, remaining_contract_months,
            market_value_eur, wage_weekly_eur, performance_index,
            marketability_index, injury_discount_pct,
        }
    }
}

prop_compose! {
    fn arb_fan_engagement()(
        match_id in any::<u64>(),
        social_media_mentions in any::<u64>(),
        app_active_users in any::<u32>(),
        merchandise_sales_units in any::<u32>(),
        sentiment_score in -1.0f32..1.0,
        peak_concurrent_chat_users in any::<u32>(),
        hashtag_impressions in any::<u64>(),
    ) -> FanEngagementMetrics {
        FanEngagementMetrics {
            match_id, social_media_mentions, app_active_users,
            merchandise_sales_units, sentiment_score,
            peak_concurrent_chat_users, hashtag_impressions,
        }
    }
}

prop_compose! {
    fn arb_venue_capacity()(
        venue_id in any::<u32>(),
        total_capacity in 5000u32..100_000,
        tickets_sold in 0u32..100_000,
        attendance_actual in 0u32..100_000,
        vip_occupancy_pct in 0.0f32..100.0,
        concession_revenue_eur in 0u64..5_000_000,
        entry_gate_throughput_per_min in 50u16..500,
        evacuation_time_estimate_s in 120u32..1800,
    ) -> VenueCapacity {
        VenueCapacity {
            venue_id, total_capacity, tickets_sold, attendance_actual,
            vip_occupancy_pct, concession_revenue_eur,
            entry_gate_throughput_per_min, evacuation_time_estimate_s,
        }
    }
}

prop_compose! {
    fn arb_broadcast_viewership()(
        match_id in any::<u64>(),
        region_code in "(US|GB|DE|FR|ES|BR|JP|KR|AU|IN)",
        live_viewers in any::<u64>(),
        peak_viewers in any::<u64>(),
        avg_watch_time_min in 0.0f32..120.0,
        ad_impressions in any::<u64>(),
        stream_bitrate_kbps in 500u32..20_000,
        buffering_ratio_pct in 0.0f32..10.0,
    ) -> BroadcastViewership {
        BroadcastViewership {
            match_id, region_code, live_viewers, peak_viewers,
            avg_watch_time_min, ad_impressions, stream_bitrate_kbps,
            buffering_ratio_pct,
        }
    }
}

fn arb_decision_type() -> impl Strategy<Value = RefereeDecisionType> {
    prop_oneof![
        Just(RefereeDecisionType::FoulGiven),
        Just(RefereeDecisionType::FoulNotGiven),
        Just(RefereeDecisionType::YellowCard),
        Just(RefereeDecisionType::RedCard),
        Just(RefereeDecisionType::PenaltyAwarded),
        Just(RefereeDecisionType::PenaltyNotAwarded),
        Just(RefereeDecisionType::OffsideGiven),
        Just(RefereeDecisionType::OffsideNotGiven),
        Just(RefereeDecisionType::GoalAllowed),
        Just(RefereeDecisionType::GoalDisallowed),
    ]
}

prop_compose! {
    fn arb_referee_decision()(
        match_id in any::<u64>(),
        minute in 0u8..120,
        decision_type in arb_decision_type(),
        var_involved in any::<bool>(),
        var_overturned in any::<bool>(),
        correct_per_panel in any::<bool>(),
        confidence_pct in 0.0f32..100.0,
    ) -> RefereeDecision {
        RefereeDecision {
            match_id, minute, decision_type, var_involved,
            var_overturned, correct_per_panel, confidence_pct,
        }
    }
}

prop_compose! {
    fn arb_pitch_control_frame()(
        frame_id in any::<u64>(),
        timestamp_ms in any::<u64>(),
        cells in proptest::collection::vec(arb_pitch_control_cell(), 1..8),
    ) -> PitchControlFrame {
        PitchControlFrame { frame_id, timestamp_ms, cells }
    }
}

prop_compose! {
    fn arb_match_stats_summary()(
        match_id in any::<u64>(),
        home_possession_pct in 20.0f32..80.0,
        away_possession_pct in 20.0f32..80.0,
        home_shots in 0u16..40,
        away_shots in 0u16..40,
        home_xg in 0.0f32..6.0,
        away_xg in 0.0f32..6.0,
        home_passes_completed in 100u16..900,
        away_passes_completed in 100u16..900,
        home_tackles_won in 5u16..40,
        away_tackles_won in 5u16..40,
    ) -> MatchStatsSummary {
        MatchStatsSummary {
            match_id, home_possession_pct, away_possession_pct,
            home_shots, away_shots, home_xg, away_xg,
            home_passes_completed, away_passes_completed,
            home_tackles_won, away_tackles_won,
        }
    }
}

// ── Tests ────────────────────────────────────────────────────────────────────

#[test]
fn test_player_tracking_data_roundtrip() {
    proptest!(|(data in arb_player_tracking())| {
        let encoded = encode_to_vec(&data).expect("encode player tracking");
        let (decoded, _): (PlayerTrackingData, _) =
            decode_from_slice(&encoded).expect("decode player tracking");
        prop_assert_eq!(data, decoded);
    });
}

#[test]
fn test_biomechanical_measurement_roundtrip() {
    proptest!(|(data in arb_biomechanical())| {
        let encoded = encode_to_vec(&data).expect("encode biomechanical");
        let (decoded, _): (BiomechanicalMeasurement, _) =
            decode_from_slice(&encoded).expect("decode biomechanical");
        prop_assert_eq!(data, decoded);
    });
}

#[test]
fn test_match_event_roundtrip() {
    proptest!(|(event in arb_match_event())| {
        let encoded = encode_to_vec(&event).expect("encode match event");
        let (decoded, _): (MatchEvent, _) =
            decode_from_slice(&encoded).expect("decode match event");
        prop_assert_eq!(event, decoded);
    });
}

#[test]
fn test_expected_goals_entry_roundtrip() {
    proptest!(|(entry in arb_xg_entry())| {
        let encoded = encode_to_vec(&entry).expect("encode xG entry");
        let (decoded, _): (ExpectedGoalsEntry, _) =
            decode_from_slice(&encoded).expect("decode xG entry");
        prop_assert_eq!(entry, decoded);
    });
}

#[test]
fn test_training_load_record_roundtrip() {
    proptest!(|(record in arb_training_load())| {
        let encoded = encode_to_vec(&record).expect("encode training load");
        let (decoded, _): (TrainingLoadRecord, _) =
            decode_from_slice(&encoded).expect("decode training load");
        prop_assert_eq!(record, decoded);
    });
}

#[test]
fn test_injury_risk_score_roundtrip() {
    proptest!(|(score in arb_injury_risk())| {
        let encoded = encode_to_vec(&score).expect("encode injury risk");
        let (decoded, _): (InjuryRiskScore, _) =
            decode_from_slice(&encoded).expect("decode injury risk");
        prop_assert_eq!(score, decoded);
    });
}

#[test]
fn test_team_formation_roundtrip() {
    proptest!(|(formation in arb_team_formation())| {
        let encoded = encode_to_vec(&formation).expect("encode team formation");
        let (decoded, _): (TeamFormation, _) =
            decode_from_slice(&encoded).expect("decode team formation");
        prop_assert_eq!(formation, decoded);
    });
}

#[test]
fn test_pitch_control_cell_roundtrip() {
    proptest!(|(cell in arb_pitch_control_cell())| {
        let encoded = encode_to_vec(&cell).expect("encode pitch control cell");
        let (decoded, _): (PitchControlCell, _) =
            decode_from_slice(&encoded).expect("decode pitch control cell");
        prop_assert_eq!(cell, decoded);
    });
}

#[test]
fn test_set_piece_analysis_roundtrip() {
    proptest!(|(sp in arb_set_piece())| {
        let encoded = encode_to_vec(&sp).expect("encode set piece");
        let (decoded, _): (SetPieceAnalysis, _) =
            decode_from_slice(&encoded).expect("decode set piece");
        prop_assert_eq!(sp, decoded);
    });
}

#[test]
fn test_draft_scouting_metrics_roundtrip() {
    proptest!(|(metrics in arb_scouting())| {
        let encoded = encode_to_vec(&metrics).expect("encode scouting");
        let (decoded, _): (DraftScoutingMetrics, _) =
            decode_from_slice(&encoded).expect("decode scouting");
        prop_assert_eq!(metrics, decoded);
    });
}

#[test]
fn test_contract_valuation_roundtrip() {
    proptest!(|(val in arb_contract_valuation())| {
        let encoded = encode_to_vec(&val).expect("encode contract valuation");
        let (decoded, _): (ContractValuation, _) =
            decode_from_slice(&encoded).expect("decode contract valuation");
        prop_assert_eq!(val, decoded);
    });
}

#[test]
fn test_fan_engagement_metrics_roundtrip() {
    proptest!(|(metrics in arb_fan_engagement())| {
        let encoded = encode_to_vec(&metrics).expect("encode fan engagement");
        let (decoded, _): (FanEngagementMetrics, _) =
            decode_from_slice(&encoded).expect("decode fan engagement");
        prop_assert_eq!(metrics, decoded);
    });
}

#[test]
fn test_venue_capacity_roundtrip() {
    proptest!(|(venue in arb_venue_capacity())| {
        let encoded = encode_to_vec(&venue).expect("encode venue capacity");
        let (decoded, _): (VenueCapacity, _) =
            decode_from_slice(&encoded).expect("decode venue capacity");
        prop_assert_eq!(venue, decoded);
    });
}

#[test]
fn test_broadcast_viewership_roundtrip() {
    proptest!(|(bv in arb_broadcast_viewership())| {
        let encoded = encode_to_vec(&bv).expect("encode broadcast viewership");
        let (decoded, _): (BroadcastViewership, _) =
            decode_from_slice(&encoded).expect("decode broadcast viewership");
        prop_assert_eq!(bv, decoded);
    });
}

#[test]
fn test_referee_decision_roundtrip() {
    proptest!(|(decision in arb_referee_decision())| {
        let encoded = encode_to_vec(&decision).expect("encode referee decision");
        let (decoded, _): (RefereeDecision, _) =
            decode_from_slice(&encoded).expect("decode referee decision");
        prop_assert_eq!(decision, decoded);
    });
}

#[test]
fn test_pitch_control_frame_roundtrip() {
    proptest!(|(frame in arb_pitch_control_frame())| {
        let encoded = encode_to_vec(&frame).expect("encode pitch control frame");
        let (decoded, _): (PitchControlFrame, _) =
            decode_from_slice(&encoded).expect("decode pitch control frame");
        prop_assert_eq!(frame, decoded);
    });
}

#[test]
fn test_match_stats_summary_roundtrip() {
    proptest!(|(stats in arb_match_stats_summary())| {
        let encoded = encode_to_vec(&stats).expect("encode match stats");
        let (decoded, _): (MatchStatsSummary, _) =
            decode_from_slice(&encoded).expect("decode match stats");
        prop_assert_eq!(stats, decoded);
    });
}

#[test]
fn test_vec_of_match_events_roundtrip() {
    proptest!(|(events in proptest::collection::vec(arb_match_event(), 0..20))| {
        let encoded = encode_to_vec(&events).expect("encode event vec");
        let (decoded, _): (Vec<MatchEvent>, _) =
            decode_from_slice(&encoded).expect("decode event vec");
        prop_assert_eq!(events, decoded);
    });
}

#[test]
fn test_vec_of_xg_entries_roundtrip() {
    proptest!(|(entries in proptest::collection::vec(arb_xg_entry(), 0..15))| {
        let encoded = encode_to_vec(&entries).expect("encode xG vec");
        let (decoded, _): (Vec<ExpectedGoalsEntry>, _) =
            decode_from_slice(&encoded).expect("decode xG vec");
        prop_assert_eq!(entries, decoded);
    });
}

#[test]
fn test_vec_of_training_loads_roundtrip() {
    proptest!(|(records in proptest::collection::vec(arb_training_load(), 0..10))| {
        let encoded = encode_to_vec(&records).expect("encode training load vec");
        let (decoded, _): (Vec<TrainingLoadRecord>, _) =
            decode_from_slice(&encoded).expect("decode training load vec");
        prop_assert_eq!(records, decoded);
    });
}

#[test]
fn test_vec_of_referee_decisions_roundtrip() {
    proptest!(|(decisions in proptest::collection::vec(arb_referee_decision(), 0..30))| {
        let encoded = encode_to_vec(&decisions).expect("encode referee vec");
        let (decoded, _): (Vec<RefereeDecision>, _) =
            decode_from_slice(&encoded).expect("decode referee vec");
        prop_assert_eq!(decisions, decoded);
    });
}

#[test]
fn test_tuple_scouting_and_contract_roundtrip() {
    proptest!(|(
        scout in arb_scouting(),
        contract in arb_contract_valuation(),
    )| {
        let pair = (scout.clone(), contract.clone());
        let encoded = encode_to_vec(&pair).expect("encode scouting-contract tuple");
        let (decoded, _): ((DraftScoutingMetrics, ContractValuation), _) =
            decode_from_slice(&encoded).expect("decode scouting-contract tuple");
        prop_assert_eq!(pair, decoded);
    });
}
