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

// --- Domain types: Competitive Gaming & Esports Analytics ---

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
struct PlayerKda {
    player_id: u64,
    gamertag: String,
    kills: u32,
    deaths: u32,
    assists: u32,
    kda_ratio: f64,
    actions_per_minute: u32,
    accuracy_pct: f32,
    headshot_pct: f32,
    game_id: u64,
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
struct MatchResult {
    match_id: u64,
    tournament_id: u64,
    game_title: String,
    team_a_name: String,
    team_b_name: String,
    team_a_score: u32,
    team_b_score: u32,
    map_name: String,
    duration_seconds: u32,
    is_overtime: bool,
    patch_version: String,
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
struct TeamComposition {
    team_id: u64,
    team_name: String,
    roster: Vec<String>,
    roles: Vec<String>,
    coach: String,
    substitute_count: u8,
    region: String,
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
enum DraftAction {
    Pick {
        hero: String,
        team: String,
        order: u8,
    },
    Ban {
        hero: String,
        team: String,
        phase: u8,
    },
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
struct HeroDraft {
    match_id: u64,
    actions: Vec<DraftAction>,
    draft_format: String,
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
struct GameStateSnapshot {
    tick: u64,
    elapsed_ms: u64,
    team_a_gold: u64,
    team_b_gold: u64,
    team_a_towers: u8,
    team_b_towers: u8,
    team_a_kills: u32,
    team_b_kills: u32,
    active_buffs: Vec<String>,
    next_objective_spawn_ms: u64,
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
struct TournamentBracket {
    tournament_id: u64,
    tournament_name: String,
    format: String,
    total_rounds: u8,
    current_round: u8,
    seeds: Vec<(String, u32)>,
    is_double_elimination: bool,
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
struct EloRating {
    player_id: u64,
    gamertag: String,
    elo: u32,
    mmr: i32,
    peak_elo: u32,
    rank_tier: String,
    games_played: u32,
    win_count: u32,
    loss_count: u32,
    streak: i16,
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
struct ReplayMetadata {
    replay_id: u64,
    match_id: u64,
    file_size_bytes: u64,
    duration_seconds: u32,
    client_version: String,
    server_region: String,
    tick_rate: u16,
    player_count: u8,
    checksum: String,
    recorded_at_epoch: u64,
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
struct HeatmapCoordinate {
    x: f32,
    y: f32,
    intensity: f32,
    event_type: String,
    player_id: u64,
    tick: u64,
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
struct HeatmapFrame {
    frame_id: u32,
    map_name: String,
    coordinates: Vec<HeatmapCoordinate>,
    resolution_x: u16,
    resolution_y: u16,
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
struct ObjectiveTiming {
    match_id: u64,
    objective_name: String,
    spawn_time_ms: u64,
    secured_time_ms: Option<u64>,
    securing_team: Option<String>,
    contested: bool,
    fight_duration_ms: u32,
    participants: Vec<u64>,
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
enum EconomyRoundType {
    FullBuy,
    ForceBuy,
    Eco,
    HalfBuy,
    Pistol,
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
struct EconomyRound {
    round_number: u8,
    round_type: EconomyRoundType,
    team_money: u32,
    spent: u32,
    equipment_value: u32,
    loss_bonus: u32,
    won: bool,
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
struct WeaponLoadout {
    player_id: u64,
    round_number: u8,
    primary_weapon: String,
    secondary_weapon: String,
    armor_value: u8,
    has_helmet: bool,
    utility: Vec<String>,
    total_cost: u32,
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
struct DamageBreakdown {
    source_player: u64,
    target_player: u64,
    weapon: String,
    damage_dealt: u32,
    headshot: bool,
    distance: f32,
    through_wall: bool,
    lethal: bool,
    assist_contributors: Vec<u64>,
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
struct TournamentPrizePool {
    tournament_id: u64,
    total_pool_usd: u64,
    distribution: Vec<(u8, u64)>,
    sponsor_contributions: Vec<(String, u64)>,
    crowdfunded_amount: u64,
    currency: String,
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
struct StreamingMetrics {
    stream_id: u64,
    platform: String,
    peak_viewers: u64,
    average_viewers: u64,
    total_watch_hours: f64,
    chat_messages_count: u64,
    unique_chatters: u64,
    subscriber_count: u32,
    bits_donated: u64,
    language: String,
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
struct MatchMvp {
    player_id: u64,
    gamertag: String,
    mvp_score: f64,
    combat_score: u32,
    eco_rating: f32,
    clutch_rounds_won: u8,
    first_kills: u8,
    multi_kills: Vec<u8>,
    damage_per_round: f32,
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
struct ScrimRecord {
    scrim_id: u64,
    team_a: String,
    team_b: String,
    maps_played: Vec<String>,
    scores: Vec<(u8, u8)>,
    notes: String,
    date_epoch: u64,
    vod_available: bool,
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
struct SeasonStanding {
    team_name: String,
    wins: u32,
    losses: u32,
    map_wins: u32,
    map_losses: u32,
    round_diff: i32,
    points: u32,
    playoff_seed: Option<u8>,
    is_eliminated: bool,
}

// --- Tests ---

#[test]
fn test_player_kda_roundtrip() {
    let stats = PlayerKda {
        player_id: 100_001,
        gamertag: "FragMaster99".to_string(),
        kills: 28,
        deaths: 11,
        assists: 7,
        kda_ratio: 3.18,
        actions_per_minute: 342,
        accuracy_pct: 47.3,
        headshot_pct: 31.2,
        game_id: 9_000_001,
    };
    let cfg = config::standard();
    let encoded = encode_to_vec(&stats, cfg).expect("encode PlayerKda");
    let (decoded, _): (PlayerKda, _) =
        decode_owned_from_slice(&encoded, cfg).expect("decode PlayerKda");
    assert_eq!(stats, decoded);
}

#[test]
fn test_match_result_overtime() {
    let result = MatchResult {
        match_id: 55_000,
        tournament_id: 301,
        game_title: "Valorant".to_string(),
        team_a_name: "Sentinels".to_string(),
        team_b_name: "Cloud9".to_string(),
        team_a_score: 14,
        team_b_score: 12,
        map_name: "Ascent".to_string(),
        duration_seconds: 2847,
        is_overtime: true,
        patch_version: "8.04".to_string(),
    };
    let cfg = config::standard();
    let encoded = encode_to_vec(&result, cfg).expect("encode MatchResult");
    let (decoded, _): (MatchResult, _) =
        decode_owned_from_slice(&encoded, cfg).expect("decode MatchResult");
    assert_eq!(result, decoded);
}

#[test]
fn test_team_composition_full_roster() {
    let comp = TeamComposition {
        team_id: 7001,
        team_name: "T1".to_string(),
        roster: vec![
            "Faker".to_string(),
            "Zeus".to_string(),
            "Oner".to_string(),
            "Gumayusi".to_string(),
            "Keria".to_string(),
        ],
        roles: vec![
            "Mid".to_string(),
            "Top".to_string(),
            "Jungle".to_string(),
            "ADC".to_string(),
            "Support".to_string(),
        ],
        coach: "Bengi".to_string(),
        substitute_count: 2,
        region: "LCK".to_string(),
    };
    let cfg = config::standard();
    let encoded = encode_to_vec(&comp, cfg).expect("encode TeamComposition");
    let (decoded, _): (TeamComposition, _) =
        decode_owned_from_slice(&encoded, cfg).expect("decode TeamComposition");
    assert_eq!(comp, decoded);
}

#[test]
fn test_hero_draft_picks_and_bans() {
    let draft = HeroDraft {
        match_id: 88_001,
        actions: vec![
            DraftAction::Ban {
                hero: "Azir".to_string(),
                team: "Blue".to_string(),
                phase: 1,
            },
            DraftAction::Ban {
                hero: "Orianna".to_string(),
                team: "Red".to_string(),
                phase: 1,
            },
            DraftAction::Pick {
                hero: "Jinx".to_string(),
                team: "Blue".to_string(),
                order: 1,
            },
            DraftAction::Pick {
                hero: "Nautilus".to_string(),
                team: "Red".to_string(),
                order: 2,
            },
            DraftAction::Pick {
                hero: "Vi".to_string(),
                team: "Red".to_string(),
                order: 3,
            },
            DraftAction::Ban {
                hero: "Syndra".to_string(),
                team: "Blue".to_string(),
                phase: 2,
            },
            DraftAction::Pick {
                hero: "Ahri".to_string(),
                team: "Blue".to_string(),
                order: 4,
            },
        ],
        draft_format: "Fearless".to_string(),
    };
    let cfg = config::standard();
    let encoded = encode_to_vec(&draft, cfg).expect("encode HeroDraft");
    let (decoded, _): (HeroDraft, _) =
        decode_owned_from_slice(&encoded, cfg).expect("decode HeroDraft");
    assert_eq!(draft, decoded);
}

#[test]
fn test_game_state_snapshot_midgame() {
    let snap = GameStateSnapshot {
        tick: 540_000,
        elapsed_ms: 1_080_000,
        team_a_gold: 42_350,
        team_b_gold: 38_900,
        team_a_towers: 8,
        team_b_towers: 6,
        team_a_kills: 14,
        team_b_kills: 9,
        active_buffs: vec!["Baron_TeamA".to_string(), "Elder_None".to_string()],
        next_objective_spawn_ms: 1_140_000,
    };
    let cfg = config::standard();
    let encoded = encode_to_vec(&snap, cfg).expect("encode GameStateSnapshot");
    let (decoded, _): (GameStateSnapshot, _) =
        decode_owned_from_slice(&encoded, cfg).expect("decode GameStateSnapshot");
    assert_eq!(snap, decoded);
}

#[test]
fn test_tournament_bracket_double_elim() {
    let bracket = TournamentBracket {
        tournament_id: 5001,
        tournament_name: "The International 2025".to_string(),
        format: "Double Elimination".to_string(),
        total_rounds: 6,
        current_round: 3,
        seeds: vec![
            ("Team Spirit".to_string(), 1),
            ("OG".to_string(), 2),
            ("PSG.LGD".to_string(), 3),
            ("Evil Geniuses".to_string(), 4),
            ("Tundra".to_string(), 5),
            ("Liquid".to_string(), 6),
            ("Gaimin Gladiators".to_string(), 7),
            ("BetBoom".to_string(), 8),
        ],
        is_double_elimination: true,
    };
    let cfg = config::standard();
    let encoded = encode_to_vec(&bracket, cfg).expect("encode TournamentBracket");
    let (decoded, _): (TournamentBracket, _) =
        decode_owned_from_slice(&encoded, cfg).expect("decode TournamentBracket");
    assert_eq!(bracket, decoded);
}

#[test]
fn test_elo_rating_high_rank() {
    let rating = EloRating {
        player_id: 200_042,
        gamertag: "s1mple".to_string(),
        elo: 2847,
        mmr: 9200,
        peak_elo: 2901,
        rank_tier: "Global Elite".to_string(),
        games_played: 4521,
        win_count: 2893,
        loss_count: 1628,
        streak: 7,
    };
    let cfg = config::standard();
    let encoded = encode_to_vec(&rating, cfg).expect("encode EloRating");
    let (decoded, _): (EloRating, _) =
        decode_owned_from_slice(&encoded, cfg).expect("decode EloRating");
    assert_eq!(rating, decoded);
}

#[test]
fn test_elo_rating_negative_streak() {
    let rating = EloRating {
        player_id: 200_099,
        gamertag: "TiltedGamer".to_string(),
        elo: 1240,
        mmr: 3100,
        peak_elo: 1680,
        rank_tier: "Silver II".to_string(),
        games_played: 312,
        win_count: 140,
        loss_count: 172,
        streak: -5,
    };
    let cfg = config::standard();
    let encoded = encode_to_vec(&rating, cfg).expect("encode negative streak EloRating");
    let (decoded, _): (EloRating, _) =
        decode_owned_from_slice(&encoded, cfg).expect("decode negative streak EloRating");
    assert_eq!(rating, decoded);
}

#[test]
fn test_replay_metadata_roundtrip() {
    let replay = ReplayMetadata {
        replay_id: 777_001,
        match_id: 55_000,
        file_size_bytes: 134_217_728,
        duration_seconds: 2847,
        client_version: "2.14.0-rc3".to_string(),
        server_region: "eu-west-1".to_string(),
        tick_rate: 128,
        player_count: 10,
        checksum: "a3f8c91b2d4e".to_string(),
        recorded_at_epoch: 1_735_689_600,
    };
    let cfg = config::standard();
    let encoded = encode_to_vec(&replay, cfg).expect("encode ReplayMetadata");
    let (decoded, _): (ReplayMetadata, _) =
        decode_owned_from_slice(&encoded, cfg).expect("decode ReplayMetadata");
    assert_eq!(replay, decoded);
}

#[test]
fn test_heatmap_frame_with_coordinates() {
    let frame = HeatmapFrame {
        frame_id: 420,
        map_name: "Inferno".to_string(),
        coordinates: vec![
            HeatmapCoordinate {
                x: 1024.5,
                y: 768.3,
                intensity: 0.92,
                event_type: "kill".to_string(),
                player_id: 100_001,
                tick: 32_000,
            },
            HeatmapCoordinate {
                x: 512.0,
                y: 384.7,
                intensity: 0.45,
                event_type: "death".to_string(),
                player_id: 100_002,
                tick: 32_010,
            },
            HeatmapCoordinate {
                x: 256.1,
                y: 900.0,
                intensity: 0.78,
                event_type: "flash".to_string(),
                player_id: 100_003,
                tick: 31_998,
            },
        ],
        resolution_x: 2048,
        resolution_y: 2048,
    };
    let cfg = config::standard();
    let encoded = encode_to_vec(&frame, cfg).expect("encode HeatmapFrame");
    let (decoded, _): (HeatmapFrame, _) =
        decode_owned_from_slice(&encoded, cfg).expect("decode HeatmapFrame");
    assert_eq!(frame, decoded);
}

#[test]
fn test_objective_timing_baron_secured() {
    let obj = ObjectiveTiming {
        match_id: 55_000,
        objective_name: "Baron Nashor".to_string(),
        spawn_time_ms: 1_200_000,
        secured_time_ms: Some(1_215_340),
        securing_team: Some("Blue".to_string()),
        contested: true,
        fight_duration_ms: 8_200,
        participants: vec![100_001, 100_002, 100_003, 100_004, 100_005],
    };
    let cfg = config::standard();
    let encoded = encode_to_vec(&obj, cfg).expect("encode ObjectiveTiming baron");
    let (decoded, _): (ObjectiveTiming, _) =
        decode_owned_from_slice(&encoded, cfg).expect("decode ObjectiveTiming baron");
    assert_eq!(obj, decoded);
}

#[test]
fn test_objective_timing_uncontested_dragon() {
    let obj = ObjectiveTiming {
        match_id: 55_001,
        objective_name: "Infernal Drake".to_string(),
        spawn_time_ms: 300_000,
        secured_time_ms: Some(312_000),
        securing_team: Some("Red".to_string()),
        contested: false,
        fight_duration_ms: 0,
        participants: vec![200_003, 200_004],
    };
    let cfg = config::standard();
    let encoded = encode_to_vec(&obj, cfg).expect("encode ObjectiveTiming dragon");
    let (decoded, _): (ObjectiveTiming, _) =
        decode_owned_from_slice(&encoded, cfg).expect("decode ObjectiveTiming dragon");
    assert_eq!(obj, decoded);
}

#[test]
fn test_economy_round_full_buy() {
    let round = EconomyRound {
        round_number: 16,
        round_type: EconomyRoundType::FullBuy,
        team_money: 8_400,
        spent: 6_250,
        equipment_value: 28_750,
        loss_bonus: 0,
        won: true,
    };
    let cfg = config::standard();
    let encoded = encode_to_vec(&round, cfg).expect("encode EconomyRound full buy");
    let (decoded, _): (EconomyRound, _) =
        decode_owned_from_slice(&encoded, cfg).expect("decode EconomyRound full buy");
    assert_eq!(round, decoded);
}

#[test]
fn test_economy_round_eco_save() {
    let round = EconomyRound {
        round_number: 3,
        round_type: EconomyRoundType::Eco,
        team_money: 1_900,
        spent: 300,
        equipment_value: 2_100,
        loss_bonus: 2_400,
        won: false,
    };
    let cfg = config::standard();
    let encoded = encode_to_vec(&round, cfg).expect("encode EconomyRound eco");
    let (decoded, _): (EconomyRound, _) =
        decode_owned_from_slice(&encoded, cfg).expect("decode EconomyRound eco");
    assert_eq!(round, decoded);
}

#[test]
fn test_weapon_loadout_rifle_round() {
    let loadout = WeaponLoadout {
        player_id: 100_001,
        round_number: 12,
        primary_weapon: "AK-47".to_string(),
        secondary_weapon: "Glock-18".to_string(),
        armor_value: 100,
        has_helmet: true,
        utility: vec![
            "Smoke".to_string(),
            "Flash".to_string(),
            "Flash".to_string(),
            "Molotov".to_string(),
        ],
        total_cost: 5_550,
    };
    let cfg = config::standard();
    let encoded = encode_to_vec(&loadout, cfg).expect("encode WeaponLoadout");
    let (decoded, _): (WeaponLoadout, _) =
        decode_owned_from_slice(&encoded, cfg).expect("decode WeaponLoadout");
    assert_eq!(loadout, decoded);
}

#[test]
fn test_damage_breakdown_headshot_wallbang() {
    let dmg = DamageBreakdown {
        source_player: 100_001,
        target_player: 200_003,
        weapon: "AWP".to_string(),
        damage_dealt: 115,
        headshot: true,
        distance: 42.7,
        through_wall: true,
        lethal: true,
        assist_contributors: vec![],
    };
    let cfg = config::standard();
    let encoded = encode_to_vec(&dmg, cfg).expect("encode DamageBreakdown wallbang");
    let (decoded, _): (DamageBreakdown, _) =
        decode_owned_from_slice(&encoded, cfg).expect("decode DamageBreakdown wallbang");
    assert_eq!(dmg, decoded);
}

#[test]
fn test_damage_breakdown_with_assists() {
    let dmg = DamageBreakdown {
        source_player: 100_002,
        target_player: 200_001,
        weapon: "M4A1-S".to_string(),
        damage_dealt: 91,
        headshot: false,
        distance: 18.3,
        through_wall: false,
        lethal: true,
        assist_contributors: vec![100_001, 100_005],
    };
    let cfg = config::standard();
    let encoded = encode_to_vec(&dmg, cfg).expect("encode DamageBreakdown assists");
    let (decoded, _): (DamageBreakdown, _) =
        decode_owned_from_slice(&encoded, cfg).expect("decode DamageBreakdown assists");
    assert_eq!(dmg, decoded);
}

#[test]
fn test_tournament_prize_pool_major() {
    let pool = TournamentPrizePool {
        tournament_id: 5001,
        total_pool_usd: 40_018_195,
        distribution: vec![
            (1, 18_208_300),
            (2, 5_202_400),
            (3, 2_601_200),
            (4, 2_601_200),
            (5, 1_400_600),
            (6, 1_400_600),
            (7, 1_000_400),
            (8, 1_000_400),
        ],
        sponsor_contributions: vec![
            ("Valve".to_string(), 1_600_000),
            ("CrowdFund".to_string(), 38_418_195),
        ],
        crowdfunded_amount: 38_418_195,
        currency: "USD".to_string(),
    };
    let cfg = config::standard();
    let encoded = encode_to_vec(&pool, cfg).expect("encode TournamentPrizePool");
    let (decoded, _): (TournamentPrizePool, _) =
        decode_owned_from_slice(&encoded, cfg).expect("decode TournamentPrizePool");
    assert_eq!(pool, decoded);
}

#[test]
fn test_streaming_metrics_high_viewership() {
    let metrics = StreamingMetrics {
        stream_id: 900_001,
        platform: "Twitch".to_string(),
        peak_viewers: 2_740_000,
        average_viewers: 1_890_000,
        total_watch_hours: 4_536_000.0,
        chat_messages_count: 18_500_000,
        unique_chatters: 890_000,
        subscriber_count: 42_000,
        bits_donated: 12_450_000,
        language: "en".to_string(),
    };
    let cfg = config::standard();
    let encoded = encode_to_vec(&metrics, cfg).expect("encode StreamingMetrics");
    let (decoded, _): (StreamingMetrics, _) =
        decode_owned_from_slice(&encoded, cfg).expect("decode StreamingMetrics");
    assert_eq!(metrics, decoded);
}

#[test]
fn test_match_mvp_clutch_player() {
    let mvp = MatchMvp {
        player_id: 100_001,
        gamertag: "ZywOo".to_string(),
        mvp_score: 1.47,
        combat_score: 312,
        eco_rating: 1.32,
        clutch_rounds_won: 4,
        first_kills: 8,
        multi_kills: vec![3, 4, 3, 5],
        damage_per_round: 98.7,
    };
    let cfg = config::standard();
    let encoded = encode_to_vec(&mvp, cfg).expect("encode MatchMvp");
    let (decoded, _): (MatchMvp, _) =
        decode_owned_from_slice(&encoded, cfg).expect("decode MatchMvp");
    assert_eq!(mvp, decoded);
}

#[test]
fn test_scrim_record_multi_map_series() {
    let scrim = ScrimRecord {
        scrim_id: 44_100,
        team_a: "NAVI".to_string(),
        team_b: "FaZe".to_string(),
        maps_played: vec![
            "Mirage".to_string(),
            "Ancient".to_string(),
            "Anubis".to_string(),
        ],
        scores: vec![(16, 12), (9, 16), (13, 11)],
        notes: "Strong T-side Mirage, need CT adjustments on Ancient".to_string(),
        date_epoch: 1_735_776_000,
        vod_available: true,
    };
    let cfg = config::standard();
    let encoded = encode_to_vec(&scrim, cfg).expect("encode ScrimRecord");
    let (decoded, _): (ScrimRecord, _) =
        decode_owned_from_slice(&encoded, cfg).expect("decode ScrimRecord");
    assert_eq!(scrim, decoded);
}

#[test]
fn test_season_standings_playoff_contention() {
    let standings = vec![
        SeasonStanding {
            team_name: "Gen.G".to_string(),
            wins: 14,
            losses: 2,
            map_wins: 30,
            map_losses: 8,
            round_diff: 87,
            points: 42,
            playoff_seed: Some(1),
            is_eliminated: false,
        },
        SeasonStanding {
            team_name: "Hanwha Life".to_string(),
            wins: 12,
            losses: 4,
            map_wins: 26,
            map_losses: 12,
            round_diff: 54,
            points: 36,
            playoff_seed: Some(2),
            is_eliminated: false,
        },
        SeasonStanding {
            team_name: "DRX".to_string(),
            wins: 5,
            losses: 11,
            map_wins: 14,
            map_losses: 24,
            round_diff: -38,
            points: 15,
            playoff_seed: None,
            is_eliminated: true,
        },
    ];
    let cfg = config::standard();
    let encoded = encode_to_vec(&standings, cfg).expect("encode season standings");
    let (decoded, _): (Vec<SeasonStanding>, _) =
        decode_owned_from_slice(&encoded, cfg).expect("decode season standings");
    assert_eq!(standings, decoded);
}
