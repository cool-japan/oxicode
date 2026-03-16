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

// ---------------------------------------------------------------------------
// Domain types: Gaming / Esports Platform
// ---------------------------------------------------------------------------

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
struct PlayerStats {
    player_id: u64,
    username: String,
    kills: u32,
    deaths: u32,
    assists: u32,
    headshot_pct: f32,
    win_rate: f64,
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
struct MatchResult {
    match_id: u64,
    map_name: String,
    duration_secs: u32,
    team_a_score: u32,
    team_b_score: u32,
    mvp_player_id: u64,
    ranked: bool,
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
enum BracketOutcome {
    Win { winner_id: u64 },
    Loss { loser_id: u64 },
    Draw,
    Forfeit { forfeiting_team: String },
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
struct TournamentBracket {
    tournament_id: u32,
    name: String,
    rounds: Vec<Vec<BracketOutcome>>,
    prize_pool_usd: u64,
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
struct LeaderboardEntry {
    rank: u32,
    player_id: u64,
    display_name: String,
    mmr: i32,
    games_played: u32,
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
struct ItemInventory {
    player_id: u64,
    items: Vec<InventoryItem>,
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
struct InventoryItem {
    item_id: u32,
    name: String,
    rarity: ItemRarity,
    quantity: u16,
    tradeable: bool,
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
enum ItemRarity {
    Common,
    Uncommon,
    Rare,
    Epic,
    Legendary,
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
struct SkillRating {
    player_id: u64,
    elo: i32,
    mmr: i32,
    peak_mmr: i32,
    rank_tier: String,
    confidence_interval: f32,
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
struct ReplayFrame {
    frame_index: u32,
    timestamp_ms: u64,
    player_positions: Vec<(u64, f32, f32, f32)>,
    events: Vec<String>,
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
struct TeamComposition {
    team_id: u32,
    team_name: String,
    roster: Vec<u64>,
    coach_id: Option<u64>,
    region: String,
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
struct AbilityCooldown {
    player_id: u64,
    ability_name: String,
    base_cooldown_ms: u32,
    remaining_ms: u32,
    charges: u8,
    max_charges: u8,
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
struct DamageEvent {
    event_id: u64,
    attacker_id: u64,
    victim_id: u64,
    weapon: String,
    damage: f32,
    is_critical: bool,
    timestamp_ms: u64,
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
struct SpectatorData {
    match_id: u64,
    observer_id: u64,
    focused_player_id: Option<u64>,
    free_cam: bool,
    replay_speed: f32,
    bookmarks: Vec<u64>,
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
struct PrizeDistribution {
    tournament_id: u32,
    currency: String,
    placements: Vec<PrizePlacement>,
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
struct PrizePlacement {
    place: u32,
    team_id: u32,
    amount_cents: u64,
}

// ---------------------------------------------------------------------------
// Tests — exactly 22
// ---------------------------------------------------------------------------

#[test]
fn test_player_stats_standard_roundtrip() {
    let cfg = config::standard();
    let val = PlayerStats {
        player_id: 10_001,
        username: "ShadowAce".to_string(),
        kills: 342,
        deaths: 210,
        assists: 88,
        headshot_pct: 0.47,
        win_rate: 0.623,
    };
    let bytes = encode_to_vec(&val, cfg).expect("encode PlayerStats");
    let (decoded, _): (PlayerStats, _) =
        decode_owned_from_slice(&bytes, cfg).expect("decode PlayerStats");
    assert_eq!(val, decoded);
}

#[test]
fn test_match_result_fixed_int_roundtrip() {
    let cfg = config::standard().with_fixed_int_encoding();
    let val = MatchResult {
        match_id: 99_999_999,
        map_name: "Dust2".to_string(),
        duration_secs: 2700,
        team_a_score: 16,
        team_b_score: 12,
        mvp_player_id: 20_042,
        ranked: true,
    };
    let bytes = encode_to_vec(&val, cfg).expect("encode MatchResult fixed_int");
    let (decoded, _): (MatchResult, _) =
        decode_owned_from_slice(&bytes, cfg).expect("decode MatchResult fixed_int");
    assert_eq!(val, decoded);
}

#[test]
fn test_bracket_outcome_win_big_endian() {
    let cfg = config::standard().with_big_endian();
    let val = BracketOutcome::Win { winner_id: 7_777 };
    let bytes = encode_to_vec(&val, cfg).expect("encode BracketOutcome::Win big_endian");
    let (decoded, _): (BracketOutcome, _) =
        decode_owned_from_slice(&bytes, cfg).expect("decode BracketOutcome::Win big_endian");
    assert_eq!(val, decoded);
}

#[test]
fn test_bracket_outcome_forfeit_standard() {
    let cfg = config::standard();
    let val = BracketOutcome::Forfeit {
        forfeiting_team: "TeamGhost".to_string(),
    };
    let bytes = encode_to_vec(&val, cfg).expect("encode BracketOutcome::Forfeit");
    let (decoded, _): (BracketOutcome, _) =
        decode_owned_from_slice(&bytes, cfg).expect("decode BracketOutcome::Forfeit");
    assert_eq!(val, decoded);
}

#[test]
fn test_tournament_bracket_nested_rounds() {
    let cfg = config::standard();
    let val = TournamentBracket {
        tournament_id: 501,
        name: "World Championship 2025".to_string(),
        rounds: vec![
            vec![
                BracketOutcome::Win { winner_id: 1 },
                BracketOutcome::Loss { loser_id: 2 },
            ],
            vec![BracketOutcome::Draw],
            vec![BracketOutcome::Win { winner_id: 3 }],
        ],
        prize_pool_usd: 1_000_000,
    };
    let bytes = encode_to_vec(&val, cfg).expect("encode TournamentBracket");
    let (decoded, _): (TournamentBracket, _) =
        decode_owned_from_slice(&bytes, cfg).expect("decode TournamentBracket");
    assert_eq!(val, decoded);
}

#[test]
fn test_leaderboard_top10_fixed_int() {
    let cfg = config::standard().with_fixed_int_encoding();
    let entries: Vec<LeaderboardEntry> = (1u32..=10)
        .map(|rank| LeaderboardEntry {
            rank,
            player_id: rank as u64 * 1000,
            display_name: format!("Player_{}", rank),
            mmr: 3000 - (rank as i32 * 50),
            games_played: 500 + rank * 10,
        })
        .collect();
    let bytes = encode_to_vec(&entries, cfg).expect("encode leaderboard top10");
    let (decoded, _): (Vec<LeaderboardEntry>, _) =
        decode_owned_from_slice(&bytes, cfg).expect("decode leaderboard top10");
    assert_eq!(entries, decoded);
}

#[test]
fn test_item_inventory_legendary_item() {
    let cfg = config::standard();
    let val = ItemInventory {
        player_id: 888,
        items: vec![
            InventoryItem {
                item_id: 9001,
                name: "Dragon Blade".to_string(),
                rarity: ItemRarity::Legendary,
                quantity: 1,
                tradeable: false,
            },
            InventoryItem {
                item_id: 100,
                name: "Basic Potion".to_string(),
                rarity: ItemRarity::Common,
                quantity: 99,
                tradeable: true,
            },
        ],
    };
    let bytes = encode_to_vec(&val, cfg).expect("encode ItemInventory");
    let (decoded, _): (ItemInventory, _) =
        decode_owned_from_slice(&bytes, cfg).expect("decode ItemInventory");
    assert_eq!(val, decoded);
}

#[test]
fn test_item_rarity_all_variants_big_endian() {
    let cfg = config::standard().with_big_endian();
    let variants = vec![
        ItemRarity::Common,
        ItemRarity::Uncommon,
        ItemRarity::Rare,
        ItemRarity::Epic,
        ItemRarity::Legendary,
    ];
    let bytes = encode_to_vec(&variants, cfg).expect("encode ItemRarity variants big_endian");
    let (decoded, _): (Vec<ItemRarity>, _) =
        decode_owned_from_slice(&bytes, cfg).expect("decode ItemRarity variants big_endian");
    assert_eq!(variants, decoded);
}

#[test]
fn test_skill_rating_negative_elo() {
    let cfg = config::standard();
    let val = SkillRating {
        player_id: 4242,
        elo: -150,
        mmr: 800,
        peak_mmr: 1500,
        rank_tier: "Bronze II".to_string(),
        confidence_interval: 150.5,
    };
    let bytes = encode_to_vec(&val, cfg).expect("encode SkillRating negative elo");
    let (decoded, _): (SkillRating, _) =
        decode_owned_from_slice(&bytes, cfg).expect("decode SkillRating negative elo");
    assert_eq!(val, decoded);
}

#[test]
fn test_skill_rating_grandmaster_fixed_int() {
    let cfg = config::standard().with_fixed_int_encoding();
    let val = SkillRating {
        player_id: 1,
        elo: 2800,
        mmr: 9000,
        peak_mmr: 9500,
        rank_tier: "Grandmaster".to_string(),
        confidence_interval: 30.0,
    };
    let bytes = encode_to_vec(&val, cfg).expect("encode SkillRating grandmaster");
    let (decoded, _): (SkillRating, _) =
        decode_owned_from_slice(&bytes, cfg).expect("decode SkillRating grandmaster");
    assert_eq!(val, decoded);
}

#[test]
fn test_replay_frame_multi_player_positions() {
    let cfg = config::standard();
    let val = ReplayFrame {
        frame_index: 1200,
        timestamp_ms: 40_000,
        player_positions: vec![
            (1001, 12.5, -3.0, 0.0),
            (1002, 50.1, 22.9, 5.0),
            (1003, -10.0, 8.8, 0.0),
        ],
        events: vec!["kill:1001->1002".to_string(), "bomb_plant:1003".to_string()],
    };
    let bytes = encode_to_vec(&val, cfg).expect("encode ReplayFrame");
    let (decoded, _): (ReplayFrame, _) =
        decode_owned_from_slice(&bytes, cfg).expect("decode ReplayFrame");
    assert_eq!(val, decoded);
}

#[test]
fn test_replay_frame_empty_events_big_endian() {
    let cfg = config::standard().with_big_endian();
    let val = ReplayFrame {
        frame_index: 0,
        timestamp_ms: 0,
        player_positions: vec![],
        events: vec![],
    };
    let bytes = encode_to_vec(&val, cfg).expect("encode ReplayFrame empty big_endian");
    let (decoded, _): (ReplayFrame, _) =
        decode_owned_from_slice(&bytes, cfg).expect("decode ReplayFrame empty big_endian");
    assert_eq!(val, decoded);
}

#[test]
fn test_team_composition_with_coach() {
    let cfg = config::standard();
    let val = TeamComposition {
        team_id: 77,
        team_name: "NovaSurge".to_string(),
        roster: vec![101, 102, 103, 104, 105],
        coach_id: Some(999),
        region: "EU".to_string(),
    };
    let bytes = encode_to_vec(&val, cfg).expect("encode TeamComposition with coach");
    let (decoded, _): (TeamComposition, _) =
        decode_owned_from_slice(&bytes, cfg).expect("decode TeamComposition with coach");
    assert_eq!(val, decoded);
}

#[test]
fn test_team_composition_no_coach_fixed_int() {
    let cfg = config::standard().with_fixed_int_encoding();
    let val = TeamComposition {
        team_id: 12,
        team_name: "SilentStorm".to_string(),
        roster: vec![200, 201, 202, 203, 204],
        coach_id: None,
        region: "NA".to_string(),
    };
    let bytes = encode_to_vec(&val, cfg).expect("encode TeamComposition no coach fixed_int");
    let (decoded, _): (TeamComposition, _) =
        decode_owned_from_slice(&bytes, cfg).expect("decode TeamComposition no coach fixed_int");
    assert_eq!(val, decoded);
}

#[test]
fn test_ability_cooldown_zero_remaining() {
    let cfg = config::standard();
    let val = AbilityCooldown {
        player_id: 55,
        ability_name: "Flash Dash".to_string(),
        base_cooldown_ms: 8000,
        remaining_ms: 0,
        charges: 2,
        max_charges: 2,
    };
    let bytes = encode_to_vec(&val, cfg).expect("encode AbilityCooldown zero remaining");
    let (decoded, _): (AbilityCooldown, _) =
        decode_owned_from_slice(&bytes, cfg).expect("decode AbilityCooldown zero remaining");
    assert_eq!(val, decoded);
}

#[test]
fn test_ability_cooldown_on_cooldown_big_endian() {
    let cfg = config::standard().with_big_endian();
    let val = AbilityCooldown {
        player_id: 66,
        ability_name: "Gravity Pull".to_string(),
        base_cooldown_ms: 15_000,
        remaining_ms: 7_342,
        charges: 0,
        max_charges: 1,
    };
    let bytes = encode_to_vec(&val, cfg).expect("encode AbilityCooldown on_cooldown big_endian");
    let (decoded, _): (AbilityCooldown, _) = decode_owned_from_slice(&bytes, cfg)
        .expect("decode AbilityCooldown on_cooldown big_endian");
    assert_eq!(val, decoded);
}

#[test]
fn test_damage_event_critical_hit() {
    let cfg = config::standard();
    let val = DamageEvent {
        event_id: 1_000_001,
        attacker_id: 300,
        victim_id: 301,
        weapon: "AWP".to_string(),
        damage: 415.0,
        is_critical: true,
        timestamp_ms: 123_456,
    };
    let bytes = encode_to_vec(&val, cfg).expect("encode DamageEvent critical");
    let (decoded, _): (DamageEvent, _) =
        decode_owned_from_slice(&bytes, cfg).expect("decode DamageEvent critical");
    assert_eq!(val, decoded);
}

#[test]
fn test_damage_event_sequence_fixed_int() {
    let cfg = config::standard().with_fixed_int_encoding();
    let events: Vec<DamageEvent> = (0u64..5)
        .map(|i| DamageEvent {
            event_id: i,
            attacker_id: 400 + i,
            victim_id: 500 + i,
            weapon: "Rifle".to_string(),
            damage: 30.0 + i as f32,
            is_critical: i % 2 == 0,
            timestamp_ms: i * 100,
        })
        .collect();
    let bytes = encode_to_vec(&events, cfg).expect("encode DamageEvent sequence fixed_int");
    let (decoded, _): (Vec<DamageEvent>, _) =
        decode_owned_from_slice(&bytes, cfg).expect("decode DamageEvent sequence fixed_int");
    assert_eq!(events, decoded);
}

#[test]
fn test_spectator_data_free_cam_with_bookmarks() {
    let cfg = config::standard();
    let val = SpectatorData {
        match_id: 77_001,
        observer_id: 9_999,
        focused_player_id: None,
        free_cam: true,
        replay_speed: 2.0,
        bookmarks: vec![0, 1500, 3000, 7200],
    };
    let bytes = encode_to_vec(&val, cfg).expect("encode SpectatorData free_cam");
    let (decoded, _): (SpectatorData, _) =
        decode_owned_from_slice(&bytes, cfg).expect("decode SpectatorData free_cam");
    assert_eq!(val, decoded);
}

#[test]
fn test_spectator_data_focused_player_big_endian() {
    let cfg = config::standard().with_big_endian();
    let val = SpectatorData {
        match_id: 77_002,
        observer_id: 8_888,
        focused_player_id: Some(1234),
        free_cam: false,
        replay_speed: 1.0,
        bookmarks: vec![],
    };
    let bytes = encode_to_vec(&val, cfg).expect("encode SpectatorData focused big_endian");
    let (decoded, _): (SpectatorData, _) =
        decode_owned_from_slice(&bytes, cfg).expect("decode SpectatorData focused big_endian");
    assert_eq!(val, decoded);
}

#[test]
fn test_prize_distribution_top8() {
    let cfg = config::standard();
    let placements = vec![
        PrizePlacement {
            place: 1,
            team_id: 1,
            amount_cents: 50_000_000,
        },
        PrizePlacement {
            place: 2,
            team_id: 2,
            amount_cents: 25_000_000,
        },
        PrizePlacement {
            place: 3,
            team_id: 3,
            amount_cents: 12_500_000,
        },
        PrizePlacement {
            place: 4,
            team_id: 4,
            amount_cents: 7_500_000,
        },
        PrizePlacement {
            place: 5,
            team_id: 5,
            amount_cents: 2_500_000,
        },
        PrizePlacement {
            place: 5,
            team_id: 6,
            amount_cents: 2_500_000,
        },
        PrizePlacement {
            place: 7,
            team_id: 7,
            amount_cents: 1_000_000,
        },
        PrizePlacement {
            place: 7,
            team_id: 8,
            amount_cents: 1_000_000,
        },
    ];
    let val = PrizeDistribution {
        tournament_id: 42,
        currency: "USD".to_string(),
        placements,
    };
    let bytes = encode_to_vec(&val, cfg).expect("encode PrizeDistribution top8");
    let (decoded, _): (PrizeDistribution, _) =
        decode_owned_from_slice(&bytes, cfg).expect("decode PrizeDistribution top8");
    assert_eq!(val, decoded);
}

#[test]
fn test_prize_distribution_consumed_bytes_match() {
    let cfg = config::standard().with_fixed_int_encoding();
    let val = PrizeDistribution {
        tournament_id: 1,
        currency: "EUR".to_string(),
        placements: vec![PrizePlacement {
            place: 1,
            team_id: 10,
            amount_cents: 100_000,
        }],
    };
    let bytes = encode_to_vec(&val, cfg).expect("encode PrizeDistribution for byte count");
    let (decoded, consumed): (PrizeDistribution, _) =
        decode_owned_from_slice(&bytes, cfg).expect("decode PrizeDistribution for byte count");
    assert_eq!(val, decoded);
    assert_eq!(consumed, bytes.len());
}
