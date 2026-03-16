//! Advanced tests for competitive gaming and esports tournament management domain types.
//! 22 test functions covering enums, structs, configs, and edge cases.

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
use oxicode::{
    config, decode_from_slice, decode_from_slice_with_config, encode_to_vec,
    encode_to_vec_with_config, Decode, Encode,
};

// --- Enums ---

#[derive(Debug, PartialEq, Encode, Decode)]
enum MatchState {
    Lobby,
    Draft,
    Live,
    Paused,
    Ended,
}

#[derive(Debug, PartialEq, Encode, Decode)]
enum BracketType {
    SingleElimination,
    DoubleElimination,
    Swiss,
    RoundRobin,
    GroupStage,
}

#[derive(Debug, PartialEq, Encode, Decode)]
enum AntiCheatEvent {
    SpeedAnomaly {
        player_id: u64,
        measured_velocity: u32,
        threshold: u32,
    },
    AimbotDetection {
        player_id: u64,
        snap_angle_millideg: u32,
        confidence_pct: u8,
    },
    WallhackSuspicion {
        player_id: u64,
        los_violations: u16,
    },
    MacroDetection {
        player_id: u64,
        actions_per_sec: u32,
        pattern_hash: u64,
    },
    Clean,
}

#[derive(Debug, PartialEq, Encode, Decode)]
enum SpectatorCameraMode {
    FreeCam { x: i32, y: i32, z: i32 },
    PlayerPov { player_id: u64 },
    Overhead,
    DirectedAutomatic,
    ReplayKeyframe { tick: u64, camera_index: u16 },
}

#[derive(Debug, PartialEq, Encode, Decode)]
enum RosterChange {
    PlayerAdded {
        player_id: u64,
        role: String,
    },
    PlayerRemoved {
        player_id: u64,
        reason: String,
    },
    RoleSwap {
        player_a: u64,
        player_b: u64,
    },
    SubstitutionIn {
        out_player: u64,
        in_player: u64,
        round: u16,
    },
    CoachChange {
        old_coach: String,
        new_coach: String,
    },
}

#[derive(Debug, PartialEq, Encode, Decode)]
enum DraftAction {
    Ban { team_index: u8, hero_or_map: String },
    Pick { team_index: u8, hero_or_map: String },
    Timeout { team_index: u8, remaining_sec: u16 },
}

#[derive(Debug, PartialEq, Encode, Decode)]
enum StreamPlatform {
    Twitch { channel_id: u64, viewer_count: u32 },
    YouTube { video_id: String, concurrent: u32 },
    Custom { url: String },
}

#[derive(Debug, PartialEq, Encode, Decode)]
enum PrizeDistribution {
    EqualSplit {
        amount_cents: u64,
        num_players: u8,
    },
    PercentageBased {
        total_cents: u64,
        shares_bps: Vec<u16>,
    },
    WinnerTakeAll {
        amount_cents: u64,
    },
    Tiered {
        tiers: Vec<PrizeTier>,
    },
}

#[derive(Debug, PartialEq, Encode, Decode)]
struct PrizeTier {
    placement_start: u16,
    placement_end: u16,
    amount_cents: u64,
}

// --- Structs ---

#[derive(Debug, PartialEq, Encode, Decode)]
struct PlayerPerformance {
    player_id: u64,
    gamertag: String,
    kills: u32,
    deaths: u32,
    assists: u32,
    damage_dealt: u64,
    healing_done: u64,
    objective_time_sec: u32,
    ult_charge_pct: u8,
    headshot_pct_x100: u16,
}

#[derive(Debug, PartialEq, Encode, Decode)]
struct EconomySnapshot {
    round: u16,
    team_a_credits: u32,
    team_b_credits: u32,
    team_a_loadout_value: u32,
    team_b_loadout_value: u32,
    team_a_loss_bonus: u8,
    team_b_loss_bonus: u8,
}

#[derive(Debug, PartialEq, Encode, Decode)]
struct RatingUpdate {
    player_id: u64,
    old_elo: i32,
    new_elo: i32,
    old_mmr: i32,
    new_mmr: i32,
    confidence_pct: u8,
    games_played: u32,
}

#[derive(Debug, PartialEq, Encode, Decode)]
struct ReplayMetadata {
    replay_id: u64,
    match_id: u64,
    file_size_bytes: u64,
    duration_ticks: u64,
    tick_rate: u16,
    version_major: u8,
    version_minor: u8,
    checksum: u64,
    player_ids: Vec<u64>,
}

#[derive(Debug, PartialEq, Encode, Decode)]
struct TournamentBracket {
    tournament_id: u64,
    name: String,
    bracket_type: BracketType,
    team_count: u16,
    current_round: u16,
    total_rounds: u16,
    seeded: bool,
}

#[derive(Debug, PartialEq, Encode, Decode)]
struct MatchResult {
    match_id: u64,
    state: MatchState,
    map_name: String,
    team_a_score: u16,
    team_b_score: u16,
    duration_sec: u32,
    performances: Vec<PlayerPerformance>,
    draft_actions: Vec<DraftAction>,
    economy_history: Vec<EconomySnapshot>,
}

#[derive(Debug, PartialEq, Encode, Decode)]
struct EsportsSeason {
    season_id: u32,
    name: String,
    bracket: TournamentBracket,
    roster_changes: Vec<RosterChange>,
    anti_cheat_log: Vec<AntiCheatEvent>,
    streams: Vec<StreamPlatform>,
    prize: PrizeDistribution,
    rating_updates: Vec<RatingUpdate>,
    replay: ReplayMetadata,
}

// --- Test 1: MatchState::Lobby roundtrip ---
#[test]
fn test_match_state_lobby() {
    let val = MatchState::Lobby;
    let bytes = encode_to_vec(&val).expect("encode MatchState::Lobby");
    let (decoded, _): (MatchState, usize) =
        decode_from_slice(&bytes).expect("decode MatchState::Lobby");
    assert_eq!(val, decoded);
}

// --- Test 2: MatchState::Ended roundtrip ---
#[test]
fn test_match_state_ended() {
    let val = MatchState::Ended;
    let bytes = encode_to_vec(&val).expect("encode MatchState::Ended");
    let (decoded, _): (MatchState, usize) =
        decode_from_slice(&bytes).expect("decode MatchState::Ended");
    assert_eq!(val, decoded);
}

// --- Test 3: BracketType all variants ---
#[test]
fn test_bracket_type_all_variants() {
    let variants = vec![
        BracketType::SingleElimination,
        BracketType::DoubleElimination,
        BracketType::Swiss,
        BracketType::RoundRobin,
        BracketType::GroupStage,
    ];
    for variant in variants {
        let bytes = encode_to_vec(&variant).expect("encode BracketType variant");
        let (decoded, _): (BracketType, usize) =
            decode_from_slice(&bytes).expect("decode BracketType variant");
        assert_eq!(variant, decoded);
    }
}

// --- Test 4: AntiCheatEvent::SpeedAnomaly with tuple data ---
#[test]
fn test_anti_cheat_speed_anomaly() {
    let val = AntiCheatEvent::SpeedAnomaly {
        player_id: 88001,
        measured_velocity: 9500,
        threshold: 7200,
    };
    let bytes = encode_to_vec(&val).expect("encode SpeedAnomaly");
    let (decoded, _): (AntiCheatEvent, usize) =
        decode_from_slice(&bytes).expect("decode SpeedAnomaly");
    assert_eq!(val, decoded);
}

// --- Test 5: AntiCheatEvent::AimbotDetection ---
#[test]
fn test_anti_cheat_aimbot_detection() {
    let val = AntiCheatEvent::AimbotDetection {
        player_id: 44200,
        snap_angle_millideg: 180_000,
        confidence_pct: 92,
    };
    let bytes = encode_to_vec(&val).expect("encode AimbotDetection");
    let (decoded, _): (AntiCheatEvent, usize) =
        decode_from_slice(&bytes).expect("decode AimbotDetection");
    assert_eq!(val, decoded);
}

// --- Test 6: SpectatorCameraMode::FreeCam with negative coords ---
#[test]
fn test_spectator_camera_freecam_negative() {
    let val = SpectatorCameraMode::FreeCam {
        x: -1500,
        y: 3200,
        z: -80,
    };
    let bytes = encode_to_vec(&val).expect("encode FreeCam negative coords");
    let (decoded, _): (SpectatorCameraMode, usize) =
        decode_from_slice(&bytes).expect("decode FreeCam negative coords");
    assert_eq!(val, decoded);
}

// --- Test 7: SpectatorCameraMode::ReplayKeyframe ---
#[test]
fn test_spectator_camera_replay_keyframe() {
    let val = SpectatorCameraMode::ReplayKeyframe {
        tick: 384_000,
        camera_index: 7,
    };
    let bytes = encode_to_vec(&val).expect("encode ReplayKeyframe");
    let (decoded, _): (SpectatorCameraMode, usize) =
        decode_from_slice(&bytes).expect("decode ReplayKeyframe");
    assert_eq!(val, decoded);
}

// --- Test 8: RosterChange::SubstitutionIn ---
#[test]
fn test_roster_change_substitution() {
    let val = RosterChange::SubstitutionIn {
        out_player: 10001,
        in_player: 10099,
        round: 13,
    };
    let bytes = encode_to_vec(&val).expect("encode SubstitutionIn");
    let (decoded, _): (RosterChange, usize) =
        decode_from_slice(&bytes).expect("decode SubstitutionIn");
    assert_eq!(val, decoded);
}

// --- Test 9: DraftAction ban/pick sequence ---
#[test]
fn test_draft_action_ban_pick_sequence() {
    let actions = vec![
        DraftAction::Ban {
            team_index: 0,
            hero_or_map: String::from("Widowmaker"),
        },
        DraftAction::Ban {
            team_index: 1,
            hero_or_map: String::from("Tracer"),
        },
        DraftAction::Pick {
            team_index: 0,
            hero_or_map: String::from("Reinhardt"),
        },
        DraftAction::Timeout {
            team_index: 1,
            remaining_sec: 15,
        },
        DraftAction::Pick {
            team_index: 1,
            hero_or_map: String::from("Ana"),
        },
    ];
    let bytes = encode_to_vec(&actions).expect("encode draft sequence");
    let (decoded, _): (Vec<DraftAction>, usize) =
        decode_from_slice(&bytes).expect("decode draft sequence");
    assert_eq!(actions, decoded);
    assert_eq!(decoded.len(), 5);
}

// --- Test 10: StreamPlatform::Twitch ---
#[test]
fn test_stream_platform_twitch() {
    let val = StreamPlatform::Twitch {
        channel_id: 123_456_789,
        viewer_count: 85_432,
    };
    let bytes = encode_to_vec(&val).expect("encode Twitch stream");
    let (decoded, _): (StreamPlatform, usize) =
        decode_from_slice(&bytes).expect("decode Twitch stream");
    assert_eq!(val, decoded);
}

// --- Test 11: StreamPlatform::YouTube ---
#[test]
fn test_stream_platform_youtube() {
    let val = StreamPlatform::YouTube {
        video_id: String::from("dQw4w9WgXcQ_live"),
        concurrent: 210_000,
    };
    let bytes = encode_to_vec(&val).expect("encode YouTube stream");
    let (decoded, _): (StreamPlatform, usize) =
        decode_from_slice(&bytes).expect("decode YouTube stream");
    assert_eq!(val, decoded);
}

// --- Test 12: PrizeDistribution::PercentageBased ---
#[test]
fn test_prize_distribution_percentage_based() {
    let val = PrizeDistribution::PercentageBased {
        total_cents: 1_000_000_00,
        shares_bps: vec![5000, 2500, 1250, 625, 625],
    };
    let bytes = encode_to_vec(&val).expect("encode PercentageBased prize");
    let (decoded, _): (PrizeDistribution, usize) =
        decode_from_slice(&bytes).expect("decode PercentageBased prize");
    assert_eq!(val, decoded);
}

// --- Test 13: PrizeDistribution::Tiered ---
#[test]
fn test_prize_distribution_tiered() {
    let val = PrizeDistribution::Tiered {
        tiers: vec![
            PrizeTier {
                placement_start: 1,
                placement_end: 1,
                amount_cents: 500_000_00,
            },
            PrizeTier {
                placement_start: 2,
                placement_end: 2,
                amount_cents: 250_000_00,
            },
            PrizeTier {
                placement_start: 3,
                placement_end: 4,
                amount_cents: 100_000_00,
            },
            PrizeTier {
                placement_start: 5,
                placement_end: 8,
                amount_cents: 25_000_00,
            },
        ],
    };
    let bytes = encode_to_vec(&val).expect("encode Tiered prize");
    let (decoded, _): (PrizeDistribution, usize) =
        decode_from_slice(&bytes).expect("decode Tiered prize");
    assert_eq!(val, decoded);
}

// --- Test 14: PlayerPerformance with high stats ---
#[test]
fn test_player_performance_high_stats() {
    let val = PlayerPerformance {
        player_id: 77001,
        gamertag: String::from("xN1ghtmare_OP"),
        kills: 42,
        deaths: 3,
        assists: 18,
        damage_dealt: 38_500,
        healing_done: 0,
        objective_time_sec: 245,
        ult_charge_pct: 100,
        headshot_pct_x100: 6725,
    };
    let bytes = encode_to_vec(&val).expect("encode PlayerPerformance high stats");
    let (decoded, _): (PlayerPerformance, usize) =
        decode_from_slice(&bytes).expect("decode PlayerPerformance high stats");
    assert_eq!(val, decoded);
    assert_eq!(decoded.headshot_pct_x100, 6725);
}

// --- Test 15: EconomySnapshot eco-round scenario ---
#[test]
fn test_economy_snapshot_eco_round() {
    let val = EconomySnapshot {
        round: 4,
        team_a_credits: 1_400,
        team_b_credits: 8_900,
        team_a_loadout_value: 800,
        team_b_loadout_value: 6_100,
        team_a_loss_bonus: 3,
        team_b_loss_bonus: 0,
    };
    let bytes = encode_to_vec(&val).expect("encode EconomySnapshot eco round");
    let (decoded, _): (EconomySnapshot, usize) =
        decode_from_slice(&bytes).expect("decode EconomySnapshot eco round");
    assert_eq!(val, decoded);
    assert!(decoded.team_a_credits < decoded.team_b_credits);
}

// --- Test 16: RatingUpdate with ELO drop ---
#[test]
fn test_rating_update_elo_drop() {
    let val = RatingUpdate {
        player_id: 55_123,
        old_elo: 2450,
        new_elo: 2418,
        old_mmr: 3100,
        new_mmr: 3072,
        confidence_pct: 78,
        games_played: 312,
    };
    let bytes = encode_to_vec(&val).expect("encode RatingUpdate elo drop");
    let (decoded, _): (RatingUpdate, usize) =
        decode_from_slice(&bytes).expect("decode RatingUpdate elo drop");
    assert_eq!(val, decoded);
    assert!(decoded.new_elo < decoded.old_elo);
    assert!(decoded.new_mmr < decoded.old_mmr);
}

// --- Test 17: ReplayMetadata with many player IDs ---
#[test]
fn test_replay_metadata_many_players() {
    let val = ReplayMetadata {
        replay_id: 900_100_200,
        match_id: 800_300_400,
        file_size_bytes: 256_000_000,
        duration_ticks: 1_920_000,
        tick_rate: 128,
        version_major: 2,
        version_minor: 14,
        checksum: 0xDEADBEEFCAFEBABE,
        player_ids: (1..=10).collect(),
    };
    let bytes = encode_to_vec(&val).expect("encode ReplayMetadata 10 players");
    let (decoded, _): (ReplayMetadata, usize) =
        decode_from_slice(&bytes).expect("decode ReplayMetadata 10 players");
    assert_eq!(val, decoded);
    assert_eq!(decoded.player_ids.len(), 10);
    assert_eq!(decoded.checksum, 0xDEADBEEFCAFEBABE);
}

// --- Test 18: TournamentBracket swiss ---
#[test]
fn test_tournament_bracket_swiss() {
    let val = TournamentBracket {
        tournament_id: 2026_001,
        name: String::from("BLAST Premier Spring 2026"),
        bracket_type: BracketType::Swiss,
        team_count: 16,
        current_round: 3,
        total_rounds: 5,
        seeded: true,
    };
    let bytes = encode_to_vec(&val).expect("encode TournamentBracket swiss");
    let (decoded, _): (TournamentBracket, usize) =
        decode_from_slice(&bytes).expect("decode TournamentBracket swiss");
    assert_eq!(val, decoded);
    assert!(decoded.seeded);
}

// --- Test 19: Full MatchResult with draft, economy, and performances ---
#[test]
fn test_full_match_result_roundtrip() {
    let val = MatchResult {
        match_id: 6_000_001,
        state: MatchState::Ended,
        map_name: String::from("Inferno"),
        team_a_score: 16,
        team_b_score: 14,
        duration_sec: 2_870,
        performances: vec![
            PlayerPerformance {
                player_id: 1,
                gamertag: String::from("FragMaster"),
                kills: 28,
                deaths: 19,
                assists: 4,
                damage_dealt: 22_400,
                healing_done: 0,
                objective_time_sec: 0,
                ult_charge_pct: 0,
                headshot_pct_x100: 5120,
            },
            PlayerPerformance {
                player_id: 2,
                gamertag: String::from("SupportGod"),
                kills: 6,
                deaths: 18,
                assists: 22,
                damage_dealt: 4_800,
                healing_done: 15_600,
                objective_time_sec: 180,
                ult_charge_pct: 45,
                headshot_pct_x100: 1200,
            },
        ],
        draft_actions: vec![
            DraftAction::Ban {
                team_index: 0,
                hero_or_map: String::from("Mirage"),
            },
            DraftAction::Ban {
                team_index: 1,
                hero_or_map: String::from("Nuke"),
            },
            DraftAction::Pick {
                team_index: 0,
                hero_or_map: String::from("Inferno"),
            },
        ],
        economy_history: vec![
            EconomySnapshot {
                round: 1,
                team_a_credits: 800,
                team_b_credits: 800,
                team_a_loadout_value: 650,
                team_b_loadout_value: 650,
                team_a_loss_bonus: 0,
                team_b_loss_bonus: 0,
            },
            EconomySnapshot {
                round: 2,
                team_a_credits: 3_400,
                team_b_credits: 1_900,
                team_a_loadout_value: 4_750,
                team_b_loadout_value: 1_500,
                team_a_loss_bonus: 0,
                team_b_loss_bonus: 1,
            },
        ],
    };
    let bytes = encode_to_vec(&val).expect("encode full MatchResult");
    let (decoded, _): (MatchResult, usize) =
        decode_from_slice(&bytes).expect("decode full MatchResult");
    assert_eq!(val, decoded);
    assert_eq!(decoded.performances.len(), 2);
    assert_eq!(decoded.draft_actions.len(), 3);
    assert_eq!(decoded.economy_history.len(), 2);
}

// --- Test 20: Big endian config with RatingUpdate ---
#[test]
fn test_big_endian_rating_update() {
    let val = RatingUpdate {
        player_id: 101_202,
        old_elo: 1800,
        new_elo: 1835,
        old_mmr: 2600,
        new_mmr: 2645,
        confidence_pct: 90,
        games_played: 150,
    };
    let cfg = config::standard().with_big_endian();
    let bytes = encode_to_vec_with_config(&val, cfg).expect("encode big endian RatingUpdate");
    let (decoded, _): (RatingUpdate, usize) =
        decode_from_slice_with_config(&bytes, cfg).expect("decode big endian RatingUpdate");
    assert_eq!(val, decoded);
}

// --- Test 21: Fixed int config with EconomySnapshot ---
#[test]
fn test_fixed_int_economy_snapshot() {
    let val = EconomySnapshot {
        round: 15,
        team_a_credits: 16_000,
        team_b_credits: 4_200,
        team_a_loadout_value: 28_350,
        team_b_loadout_value: 3_900,
        team_a_loss_bonus: 0,
        team_b_loss_bonus: 4,
    };
    let cfg = config::standard().with_fixed_int_encoding();
    let bytes = encode_to_vec_with_config(&val, cfg).expect("encode fixed int EconomySnapshot");
    let (decoded, _): (EconomySnapshot, usize) =
        decode_from_slice_with_config(&bytes, cfg).expect("decode fixed int EconomySnapshot");
    assert_eq!(val, decoded);
}

// --- Test 22: Full EsportsSeason aggregate struct ---
#[test]
fn test_full_esports_season_roundtrip() {
    let val = EsportsSeason {
        season_id: 2026,
        name: String::from("Champions League Season 12"),
        bracket: TournamentBracket {
            tournament_id: 120_001,
            name: String::from("CL S12 Playoffs"),
            bracket_type: BracketType::DoubleElimination,
            team_count: 8,
            current_round: 4,
            total_rounds: 6,
            seeded: true,
        },
        roster_changes: vec![
            RosterChange::PlayerAdded {
                player_id: 5001,
                role: String::from("flex_dps"),
            },
            RosterChange::RoleSwap {
                player_a: 5002,
                player_b: 5003,
            },
            RosterChange::CoachChange {
                old_coach: String::from("OldSchool"),
                new_coach: String::from("Stratego"),
            },
        ],
        anti_cheat_log: vec![
            AntiCheatEvent::Clean,
            AntiCheatEvent::WallhackSuspicion {
                player_id: 9999,
                los_violations: 3,
            },
            AntiCheatEvent::MacroDetection {
                player_id: 8888,
                actions_per_sec: 42,
                pattern_hash: 0xABCDEF0123456789,
            },
        ],
        streams: vec![
            StreamPlatform::Twitch {
                channel_id: 55_000_001,
                viewer_count: 142_000,
            },
            StreamPlatform::YouTube {
                video_id: String::from("abc123XYZ_live"),
                concurrent: 89_000,
            },
            StreamPlatform::Custom {
                url: String::from("https://esports.example.com/live/cl-s12"),
            },
        ],
        prize: PrizeDistribution::Tiered {
            tiers: vec![
                PrizeTier {
                    placement_start: 1,
                    placement_end: 1,
                    amount_cents: 2_000_000_00,
                },
                PrizeTier {
                    placement_start: 2,
                    placement_end: 2,
                    amount_cents: 800_000_00,
                },
                PrizeTier {
                    placement_start: 3,
                    placement_end: 4,
                    amount_cents: 300_000_00,
                },
                PrizeTier {
                    placement_start: 5,
                    placement_end: 8,
                    amount_cents: 100_000_00,
                },
            ],
        },
        rating_updates: vec![
            RatingUpdate {
                player_id: 5001,
                old_elo: 2100,
                new_elo: 2180,
                old_mmr: 3000,
                new_mmr: 3090,
                confidence_pct: 85,
                games_played: 47,
            },
            RatingUpdate {
                player_id: 5002,
                old_elo: 2350,
                new_elo: 2340,
                old_mmr: 3400,
                new_mmr: 3385,
                confidence_pct: 92,
                games_played: 203,
            },
        ],
        replay: ReplayMetadata {
            replay_id: 7_000_001,
            match_id: 6_000_001,
            file_size_bytes: 512_000_000,
            duration_ticks: 3_840_000,
            tick_rate: 128,
            version_major: 3,
            version_minor: 1,
            checksum: 0x1234_5678_9ABC_DEF0,
            player_ids: vec![5001, 5002, 5003, 5004, 5005, 6001, 6002, 6003, 6004, 6005],
        },
    };
    let bytes = encode_to_vec(&val).expect("encode full EsportsSeason");
    let (decoded, _): (EsportsSeason, usize) =
        decode_from_slice(&bytes).expect("decode full EsportsSeason");
    assert_eq!(val, decoded);
    assert_eq!(decoded.roster_changes.len(), 3);
    assert_eq!(decoded.anti_cheat_log.len(), 3);
    assert_eq!(decoded.streams.len(), 3);
    assert_eq!(decoded.rating_updates.len(), 2);
    assert_eq!(decoded.replay.player_ids.len(), 10);
}
