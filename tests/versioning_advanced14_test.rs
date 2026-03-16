//! Gaming / game state versioning tests for OxiCode (set 14).
//!
//! Covers 22 scenarios exercising encode_versioned_value, decode_versioned_value,
//! Version, and three generations of GameState structs (V1/V2/V3) with all
//! CharacterClass variants, Leaderboard, Vec of versioned states, version
//! comparison, consumed bytes, version equality, empty achievements, nil guild,
//! and major/minor/patch field access.

#![cfg(feature = "versioning")]
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
    decode_from_slice, decode_versioned_value, encode_to_vec, encode_versioned_value,
    versioning::Version, Decode, Encode,
};

// ── Shared types ──────────────────────────────────────────────────────────────

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
enum CharacterClass {
    Warrior,
    Mage,
    Rogue,
    Healer,
    Ranger,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct GameStateV1 {
    player_id: u64,
    level: u32,
    score: u64,
    class: CharacterClass,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct GameStateV2 {
    player_id: u64,
    level: u32,
    score: u64,
    class: CharacterClass,
    achievements: Vec<String>,
    playtime_s: u64,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct GameStateV3 {
    player_id: u64,
    level: u32,
    score: u64,
    class: CharacterClass,
    achievements: Vec<String>,
    playtime_s: u64,
    guild: Option<String>,
    rank: u32,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct Leaderboard {
    entries: Vec<GameStateV1>,
    season: u32,
    name: String,
}

// ── Scenario 1 ────────────────────────────────────────────────────────────────
// GameStateV1 Warrior roundtrip at version 1.0.0
#[test]
fn test_game_state_v1_warrior_roundtrip() {
    let version = Version::new(1, 0, 0);
    let original = GameStateV1 {
        player_id: 100001,
        level: 42,
        score: 99500,
        class: CharacterClass::Warrior,
    };
    let encoded =
        encode_versioned_value(&original, version).expect("encode_versioned_value failed");
    let (decoded, ver, consumed): (GameStateV1, Version, usize) =
        decode_versioned_value(&encoded).expect("decode_versioned_value failed");
    assert_eq!(decoded, original);
    assert_eq!(ver, version);
    assert!(consumed > 0);
}

// ── Scenario 2 ────────────────────────────────────────────────────────────────
// GameStateV1 Mage roundtrip at version 1.1.0
#[test]
fn test_game_state_v1_mage_roundtrip() {
    let version = Version::new(1, 1, 0);
    let original = GameStateV1 {
        player_id: 200002,
        level: 88,
        score: 1_500_000,
        class: CharacterClass::Mage,
    };
    let encoded =
        encode_versioned_value(&original, version).expect("encode_versioned_value failed");
    let (decoded, ver, consumed): (GameStateV1, Version, usize) =
        decode_versioned_value(&encoded).expect("decode_versioned_value failed");
    assert_eq!(decoded, original);
    assert_eq!(ver, version);
    assert!(consumed > 0);
}

// ── Scenario 3 ────────────────────────────────────────────────────────────────
// GameStateV1 Rogue roundtrip at version 1.2.3
#[test]
fn test_game_state_v1_rogue_roundtrip() {
    let version = Version::new(1, 2, 3);
    let original = GameStateV1 {
        player_id: 300003,
        level: 55,
        score: 750_000,
        class: CharacterClass::Rogue,
    };
    let encoded =
        encode_versioned_value(&original, version).expect("encode_versioned_value failed");
    let (decoded, ver, _consumed): (GameStateV1, Version, usize) =
        decode_versioned_value(&encoded).expect("decode_versioned_value failed");
    assert_eq!(decoded.class, CharacterClass::Rogue);
    assert_eq!(decoded.player_id, 300003);
    assert_eq!(ver, version);
}

// ── Scenario 4 ────────────────────────────────────────────────────────────────
// GameStateV1 Healer roundtrip at version 1.0.1
#[test]
fn test_game_state_v1_healer_roundtrip() {
    let version = Version::new(1, 0, 1);
    let original = GameStateV1 {
        player_id: 400004,
        level: 30,
        score: 250_000,
        class: CharacterClass::Healer,
    };
    let encoded =
        encode_versioned_value(&original, version).expect("encode_versioned_value failed");
    let (decoded, ver, consumed): (GameStateV1, Version, usize) =
        decode_versioned_value(&encoded).expect("decode_versioned_value failed");
    assert_eq!(decoded, original);
    assert_eq!(ver, version);
    assert!(consumed > 0);
}

// ── Scenario 5 ────────────────────────────────────────────────────────────────
// GameStateV1 Ranger roundtrip at version 1.3.0
#[test]
fn test_game_state_v1_ranger_roundtrip() {
    let version = Version::new(1, 3, 0);
    let original = GameStateV1 {
        player_id: 500005,
        level: 72,
        score: 3_200_000,
        class: CharacterClass::Ranger,
    };
    let encoded =
        encode_versioned_value(&original, version).expect("encode_versioned_value failed");
    let (decoded, ver, consumed): (GameStateV1, Version, usize) =
        decode_versioned_value(&encoded).expect("decode_versioned_value failed");
    assert_eq!(decoded, original);
    assert_eq!(ver, version);
    assert!(consumed > 0);
}

// ── Scenario 6 ────────────────────────────────────────────────────────────────
// GameStateV2 with achievements and playtime roundtrip at version 2.0.0
#[test]
fn test_game_state_v2_with_achievements_roundtrip() {
    let version = Version::new(2, 0, 0);
    let original = GameStateV2 {
        player_id: 600006,
        level: 100,
        score: 9_999_999,
        class: CharacterClass::Warrior,
        achievements: vec![
            String::from("First Blood"),
            String::from("Dragon Slayer"),
            String::from("Castle Conqueror"),
        ],
        playtime_s: 360_000,
    };
    let encoded =
        encode_versioned_value(&original, version).expect("encode_versioned_value failed");
    let (decoded, ver, consumed): (GameStateV2, Version, usize) =
        decode_versioned_value(&encoded).expect("decode_versioned_value failed");
    assert_eq!(decoded, original);
    assert_eq!(ver, version);
    assert!(consumed > 0);
    assert_eq!(decoded.achievements.len(), 3);
}

// ── Scenario 7 ────────────────────────────────────────────────────────────────
// GameStateV2 with empty achievements roundtrip
#[test]
fn test_game_state_v2_empty_achievements_roundtrip() {
    let version = Version::new(2, 1, 0);
    let original = GameStateV2 {
        player_id: 700007,
        level: 1,
        score: 0,
        class: CharacterClass::Mage,
        achievements: vec![],
        playtime_s: 120,
    };
    let encoded =
        encode_versioned_value(&original, version).expect("encode_versioned_value failed");
    let (decoded, ver, _consumed): (GameStateV2, Version, usize) =
        decode_versioned_value(&encoded).expect("decode_versioned_value failed");
    assert_eq!(decoded, original);
    assert_eq!(ver, version);
    assert!(decoded.achievements.is_empty());
}

// ── Scenario 8 ────────────────────────────────────────────────────────────────
// GameStateV3 with guild and rank roundtrip at version 3.0.0
#[test]
fn test_game_state_v3_with_guild_roundtrip() {
    let version = Version::new(3, 0, 0);
    let original = GameStateV3 {
        player_id: 800008,
        level: 95,
        score: 8_765_432,
        class: CharacterClass::Rogue,
        achievements: vec![String::from("Shadow Master"), String::from("Night Stalker")],
        playtime_s: 720_000,
        guild: Some(String::from("Shadow Brotherhood")),
        rank: 7,
    };
    let encoded =
        encode_versioned_value(&original, version).expect("encode_versioned_value failed");
    let (decoded, ver, consumed): (GameStateV3, Version, usize) =
        decode_versioned_value(&encoded).expect("decode_versioned_value failed");
    assert_eq!(decoded, original);
    assert_eq!(ver, version);
    assert!(consumed > 0);
    assert_eq!(decoded.guild, Some(String::from("Shadow Brotherhood")));
}

// ── Scenario 9 ────────────────────────────────────────────────────────────────
// GameStateV3 with nil guild (no guild membership) roundtrip
#[test]
fn test_game_state_v3_nil_guild_roundtrip() {
    let version = Version::new(3, 0, 0);
    let original = GameStateV3 {
        player_id: 900009,
        level: 60,
        score: 4_100_000,
        class: CharacterClass::Healer,
        achievements: vec![String::from("Solo Player")],
        playtime_s: 180_000,
        guild: None,
        rank: 42,
    };
    let encoded =
        encode_versioned_value(&original, version).expect("encode_versioned_value failed");
    let (decoded, ver, _consumed): (GameStateV3, Version, usize) =
        decode_versioned_value(&encoded).expect("decode_versioned_value failed");
    assert_eq!(decoded.guild, None);
    assert_eq!(decoded.rank, 42);
    assert_eq!(ver, version);
}

// ── Scenario 10 ───────────────────────────────────────────────────────────────
// GameStateV3 Ranger with many achievements at version 3.2.1
#[test]
fn test_game_state_v3_ranger_many_achievements_roundtrip() {
    let version = Version::new(3, 2, 1);
    let original = GameStateV3 {
        player_id: 1_000_010,
        level: 99,
        score: 12_000_000,
        class: CharacterClass::Ranger,
        achievements: vec![
            String::from("Eagle Eye"),
            String::from("Forest Guardian"),
            String::from("Hundred Arrows"),
            String::from("Beast Tamer"),
            String::from("Wilderness Survivor"),
        ],
        playtime_s: 1_000_000,
        guild: Some(String::from("Rangers of the Wild")),
        rank: 1,
    };
    let encoded =
        encode_versioned_value(&original, version).expect("encode_versioned_value failed");
    let (decoded, ver, consumed): (GameStateV3, Version, usize) =
        decode_versioned_value(&encoded).expect("decode_versioned_value failed");
    assert_eq!(decoded.achievements.len(), 5);
    assert_eq!(decoded.rank, 1);
    assert_eq!(ver, version);
    assert!(consumed > 0);
}

// ── Scenario 11 ───────────────────────────────────────────────────────────────
// Version equality check: same major/minor/patch values are equal
#[test]
fn test_version_equality_check() {
    let v_a = Version::new(2, 5, 10);
    let v_b = Version::new(2, 5, 10);
    assert_eq!(v_a, v_b);
    assert!(!(v_a < v_b));
    assert!(!(v_a > v_b));
}

// ── Scenario 12 ───────────────────────────────────────────────────────────────
// Major version comparison: 1.0.0 < 2.0.0 < 3.0.0
#[test]
fn test_version_major_ordering() {
    let v1 = Version::new(1, 0, 0);
    let v2 = Version::new(2, 0, 0);
    let v3 = Version::new(3, 0, 0);
    assert!(v1 < v2);
    assert!(v2 < v3);
    assert!(v1 < v3);
    assert_ne!(v1, v2);
    assert_ne!(v2, v3);
}

// ── Scenario 13 ───────────────────────────────────────────────────────────────
// Minor version comparison within same major
#[test]
fn test_version_minor_ordering() {
    let v_low = Version::new(2, 0, 0);
    let v_mid = Version::new(2, 5, 0);
    let v_high = Version::new(2, 10, 0);
    assert!(v_low < v_mid);
    assert!(v_mid < v_high);
    assert!(v_low < v_high);
}

// ── Scenario 14 ───────────────────────────────────────────────────────────────
// Patch version comparison within same major/minor
#[test]
fn test_version_patch_ordering() {
    let v_base = Version::new(1, 0, 0);
    let v_patched = Version::new(1, 0, 7);
    assert!(v_base < v_patched);
    assert!(v_patched > v_base);
    assert_ne!(v_base, v_patched);
}

// ── Scenario 15 ───────────────────────────────────────────────────────────────
// Version field accessors (major, minor, patch) are correct after decode
#[test]
fn test_version_field_accessors_after_decode() {
    let version = Version::new(5, 11, 33);
    let original = GameStateV1 {
        player_id: 1_500_015,
        level: 77,
        score: 6_600_000,
        class: CharacterClass::Mage,
    };
    let encoded =
        encode_versioned_value(&original, version).expect("encode_versioned_value failed");
    let (_decoded, ver, _consumed): (GameStateV1, Version, usize) =
        decode_versioned_value(&encoded).expect("decode_versioned_value failed");
    assert_eq!(ver.major, 5);
    assert_eq!(ver.minor, 11);
    assert_eq!(ver.patch, 33);
}

// ── Scenario 16 ───────────────────────────────────────────────────────────────
// Vec of versioned GameStateV1 entries, each encoded independently
#[test]
fn test_vec_of_versioned_game_state_v1_entries() {
    let version = Version::new(1, 0, 0);
    let states = vec![
        GameStateV1 {
            player_id: 101,
            level: 10,
            score: 100_000,
            class: CharacterClass::Warrior,
        },
        GameStateV1 {
            player_id: 202,
            level: 25,
            score: 300_000,
            class: CharacterClass::Mage,
        },
        GameStateV1 {
            player_id: 303,
            level: 50,
            score: 800_000,
            class: CharacterClass::Ranger,
        },
    ];

    for original in &states {
        let encoded =
            encode_versioned_value(original, version).expect("encode_versioned_value failed");
        let (decoded, ver, consumed): (GameStateV1, Version, usize) =
            decode_versioned_value(&encoded).expect("decode_versioned_value failed");
        assert_eq!(&decoded, original);
        assert_eq!(ver, version);
        assert!(consumed > 0);
    }
}

// ── Scenario 17 ───────────────────────────────────────────────────────────────
// Leaderboard struct roundtrip at version 1.0.0
#[test]
fn test_leaderboard_roundtrip_v1() {
    let version = Version::new(1, 0, 0);
    let original = Leaderboard {
        entries: vec![
            GameStateV1 {
                player_id: 111,
                level: 100,
                score: 10_000_000,
                class: CharacterClass::Warrior,
            },
            GameStateV1 {
                player_id: 222,
                level: 98,
                score: 9_500_000,
                class: CharacterClass::Mage,
            },
            GameStateV1 {
                player_id: 333,
                level: 95,
                score: 9_000_000,
                class: CharacterClass::Rogue,
            },
        ],
        season: 7,
        name: String::from("Season 7 Global Rankings"),
    };
    let encoded =
        encode_versioned_value(&original, version).expect("encode_versioned_value failed");
    let (decoded, ver, consumed): (Leaderboard, Version, usize) =
        decode_versioned_value(&encoded).expect("decode_versioned_value failed");
    assert_eq!(decoded, original);
    assert_eq!(ver, version);
    assert!(consumed > 0);
    assert_eq!(decoded.entries.len(), 3);
    assert_eq!(decoded.season, 7);
}

// ── Scenario 18 ───────────────────────────────────────────────────────────────
// Leaderboard with empty entries roundtrip
#[test]
fn test_leaderboard_empty_entries_roundtrip() {
    let version = Version::new(2, 0, 0);
    let original = Leaderboard {
        entries: vec![],
        season: 1,
        name: String::from("Pre-Season Placeholder"),
    };
    let encoded =
        encode_versioned_value(&original, version).expect("encode_versioned_value failed");
    let (decoded, ver, _consumed): (Leaderboard, Version, usize) =
        decode_versioned_value(&encoded).expect("decode_versioned_value failed");
    assert_eq!(decoded, original);
    assert_eq!(ver, version);
    assert!(decoded.entries.is_empty());
}

// ── Scenario 19 ───────────────────────────────────────────────────────────────
// Same GameStateV2 data encoded at different version tags produces distinct version metadata
#[test]
fn test_same_game_state_v2_different_version_tags() {
    let v_old = Version::new(2, 0, 0);
    let v_new = Version::new(2, 3, 0);
    let state = GameStateV2 {
        player_id: 999_019,
        level: 66,
        score: 5_050_050,
        class: CharacterClass::Healer,
        achievements: vec![String::from("Lifesaver"), String::from("Miracle Worker")],
        playtime_s: 540_000,
    };

    let encoded_old =
        encode_versioned_value(&state, v_old).expect("encode_versioned_value v_old failed");
    let encoded_new =
        encode_versioned_value(&state, v_new).expect("encode_versioned_value v_new failed");

    let (decoded1, ver1, _): (GameStateV2, Version, usize) =
        decode_versioned_value(&encoded_old).expect("decode v_old failed");
    let (decoded2, ver2, _): (GameStateV2, Version, usize) =
        decode_versioned_value(&encoded_new).expect("decode v_new failed");

    assert_eq!(decoded1, state);
    assert_eq!(decoded2, state);
    assert_eq!(ver1, v_old);
    assert_eq!(ver2, v_new);
    assert_ne!(ver1, ver2);
}

// ── Scenario 20 ───────────────────────────────────────────────────────────────
// Consumed bytes is positive and does not exceed total encoded buffer length
#[test]
fn test_consumed_bytes_within_encoded_length() {
    let version = Version::new(1, 0, 0);
    let original = GameStateV1 {
        player_id: 202_020,
        level: 33,
        score: 1_234_567,
        class: CharacterClass::Rogue,
    };
    let encoded =
        encode_versioned_value(&original, version).expect("encode_versioned_value failed");
    let total_len = encoded.len();
    let (_decoded, _ver, consumed): (GameStateV1, Version, usize) =
        decode_versioned_value(&encoded).expect("decode_versioned_value failed");
    assert!(consumed > 0, "consumed must be positive");
    assert!(
        consumed <= total_len,
        "consumed ({consumed}) must not exceed total encoded length ({total_len})"
    );
}

// ── Scenario 21 ───────────────────────────────────────────────────────────────
// Version extracted from encoded GameStateV3 matches expected major/minor/patch
#[test]
fn test_version_extracted_from_game_state_v3_bytes() {
    let version = Version::new(3, 7, 15);
    let original = GameStateV3 {
        player_id: 212_121,
        level: 80,
        score: 7_777_777,
        class: CharacterClass::Warrior,
        achievements: vec![String::from("Titan Slayer")],
        playtime_s: 900_000,
        guild: Some(String::from("Iron Fist")),
        rank: 3,
    };
    let encoded =
        encode_versioned_value(&original, version).expect("encode_versioned_value failed");
    let (_decoded, ver, _consumed): (GameStateV3, Version, usize) =
        decode_versioned_value(&encoded).expect("decode_versioned_value failed");
    assert_eq!(ver.major, 3);
    assert_eq!(ver.minor, 7);
    assert_eq!(ver.patch, 15);
}

// ── Scenario 22 ───────────────────────────────────────────────────────────────
// GameStateV1 plain encode/decode baseline (no versioning wrapper) still works
#[test]
fn test_game_state_v1_plain_encode_decode_baseline() {
    let original = GameStateV1 {
        player_id: 222_222,
        level: 50,
        score: 5_000_000,
        class: CharacterClass::Ranger,
    };
    let encoded = encode_to_vec(&original).expect("encode_to_vec failed");
    let (decoded, _consumed): (GameStateV1, usize) =
        decode_from_slice(&encoded).expect("decode_from_slice failed");
    assert_eq!(decoded, original);
}
