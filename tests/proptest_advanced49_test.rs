//! Advanced property-based roundtrip tests using proptest (set 49).
//!
//! Domain: E-sports / competitive gaming statistics
//!
//! Each test is a top-level #[test] function inside a proptest! macro block,
//! verifying encode/decode roundtrip invariants for e-sports types.

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

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
enum GameMode {
    Ranked,
    Casual,
    Tournament,
    Practice,
    Custom,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
enum MatchOutcome {
    Victory,
    Defeat,
    Draw,
    Disconnected,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct PlayerStats {
    player_id: u64,
    kills: u32,
    deaths: u32,
    assists: u32,
    score: i64,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct GameMatch {
    match_id: u64,
    mode: GameMode,
    outcome: MatchOutcome,
    duration_s: u32,
    players: Vec<PlayerStats>,
}

fn make_game_mode(n: u8) -> GameMode {
    match n % 5 {
        0 => GameMode::Ranked,
        1 => GameMode::Casual,
        2 => GameMode::Tournament,
        3 => GameMode::Practice,
        _ => GameMode::Custom,
    }
}

fn make_match_outcome(n: u8) -> MatchOutcome {
    match n % 4 {
        0 => MatchOutcome::Victory,
        1 => MatchOutcome::Defeat,
        2 => MatchOutcome::Draw,
        _ => MatchOutcome::Disconnected,
    }
}

fn player_stats_strategy() -> impl Strategy<Value = PlayerStats> {
    (
        any::<u64>(),
        any::<u32>(),
        any::<u32>(),
        any::<u32>(),
        any::<i64>(),
    )
        .prop_map(|(player_id, kills, deaths, assists, score)| PlayerStats {
            player_id,
            kills,
            deaths,
            assists,
            score,
        })
}

fn game_match_strategy() -> impl Strategy<Value = GameMatch> {
    (
        any::<u64>(),
        any::<u8>().prop_map(make_game_mode),
        any::<u8>().prop_map(make_match_outcome),
        any::<u32>(),
        prop::collection::vec(player_stats_strategy(), 0..10),
    )
        .prop_map(|(match_id, mode, outcome, duration_s, players)| GameMatch {
            match_id,
            mode,
            outcome,
            duration_s,
            players,
        })
}

proptest! {
    #[test]
    fn prop_game_mode_roundtrip(n in 0u8..=4u8) {
        let val = make_game_mode(n);
        let enc = encode_to_vec(&val).expect("encode GameMode");
        let (dec, _): (GameMode, usize) = decode_from_slice(&enc).expect("decode GameMode");
        prop_assert_eq!(val, dec);
    }
}

proptest! {
    #[test]
    fn prop_game_mode_consumed_equals_len(n in 0u8..=4u8) {
        let val = make_game_mode(n);
        let enc = encode_to_vec(&val).expect("encode GameMode for consumed");
        let (_, consumed): (GameMode, usize) =
            decode_from_slice(&enc).expect("decode GameMode for consumed");
        prop_assert_eq!(consumed, enc.len());
    }
}

proptest! {
    #[test]
    fn prop_match_outcome_roundtrip(n in 0u8..=3u8) {
        let val = make_match_outcome(n);
        let enc = encode_to_vec(&val).expect("encode MatchOutcome");
        let (dec, _): (MatchOutcome, usize) =
            decode_from_slice(&enc).expect("decode MatchOutcome");
        prop_assert_eq!(val, dec);
    }
}

proptest! {
    #[test]
    fn prop_match_outcome_consumed_equals_len(n in 0u8..=3u8) {
        let val = make_match_outcome(n);
        let enc = encode_to_vec(&val).expect("encode MatchOutcome for consumed");
        let (_, consumed): (MatchOutcome, usize) =
            decode_from_slice(&enc).expect("decode MatchOutcome for consumed");
        prop_assert_eq!(consumed, enc.len());
    }
}

proptest! {
    #[test]
    fn prop_player_stats_roundtrip(ps in player_stats_strategy()) {
        let enc = encode_to_vec(&ps).expect("encode PlayerStats");
        let (dec, _): (PlayerStats, usize) =
            decode_from_slice(&enc).expect("decode PlayerStats");
        prop_assert_eq!(ps, dec);
    }
}

proptest! {
    #[test]
    fn prop_player_stats_consumed_equals_len(ps in player_stats_strategy()) {
        let enc = encode_to_vec(&ps).expect("encode PlayerStats for consumed");
        let (_, consumed): (PlayerStats, usize) =
            decode_from_slice(&enc).expect("decode PlayerStats for consumed");
        prop_assert_eq!(consumed, enc.len());
    }
}

proptest! {
    #[test]
    fn prop_game_match_roundtrip(gm in game_match_strategy()) {
        let enc = encode_to_vec(&gm).expect("encode GameMatch");
        let (dec, _): (GameMatch, usize) =
            decode_from_slice(&enc).expect("decode GameMatch");
        prop_assert_eq!(gm, dec);
    }
}

proptest! {
    #[test]
    fn prop_game_match_consumed_equals_len(gm in game_match_strategy()) {
        let enc = encode_to_vec(&gm).expect("encode GameMatch for consumed");
        let (_, consumed): (GameMatch, usize) =
            decode_from_slice(&enc).expect("decode GameMatch for consumed");
        prop_assert_eq!(consumed, enc.len());
    }
}

proptest! {
    #[test]
    fn prop_player_stats_deterministic_encoding(ps in player_stats_strategy()) {
        let enc1 = encode_to_vec(&ps).expect("encode PlayerStats first");
        let enc2 = encode_to_vec(&ps).expect("encode PlayerStats second");
        prop_assert_eq!(enc1, enc2);
    }
}

proptest! {
    #[test]
    fn prop_game_match_deterministic_encoding(gm in game_match_strategy()) {
        let enc1 = encode_to_vec(&gm).expect("encode GameMatch first");
        let enc2 = encode_to_vec(&gm).expect("encode GameMatch second");
        prop_assert_eq!(enc1, enc2);
    }
}

proptest! {
    #[test]
    fn prop_vec_player_stats_roundtrip(
        players in prop::collection::vec(player_stats_strategy(), 0..12)
    ) {
        let enc = encode_to_vec(&players).expect("encode Vec<PlayerStats>");
        let (dec, _): (Vec<PlayerStats>, usize) =
            decode_from_slice(&enc).expect("decode Vec<PlayerStats>");
        prop_assert_eq!(players, dec);
    }
}

proptest! {
    #[test]
    fn prop_vec_player_stats_consumed_equals_len(
        players in prop::collection::vec(player_stats_strategy(), 0..12)
    ) {
        let enc = encode_to_vec(&players).expect("encode Vec<PlayerStats> for consumed");
        let (_, consumed): (Vec<PlayerStats>, usize) =
            decode_from_slice(&enc).expect("decode Vec<PlayerStats> for consumed");
        prop_assert_eq!(consumed, enc.len());
    }
}

proptest! {
    #[test]
    fn prop_option_game_match_roundtrip(
        present: bool,
        gm in game_match_strategy(),
    ) {
        let opt: Option<GameMatch> = if present { Some(gm) } else { None };
        let enc = encode_to_vec(&opt).expect("encode Option<GameMatch>");
        let (dec, _): (Option<GameMatch>, usize) =
            decode_from_slice(&enc).expect("decode Option<GameMatch>");
        prop_assert_eq!(opt, dec);
    }
}

proptest! {
    #[test]
    fn prop_option_game_match_consumed_equals_len(
        present: bool,
        gm in game_match_strategy(),
    ) {
        let opt: Option<GameMatch> = if present { Some(gm) } else { None };
        let enc = encode_to_vec(&opt).expect("encode Option<GameMatch> for consumed");
        let (_, consumed): (Option<GameMatch>, usize) =
            decode_from_slice(&enc).expect("decode Option<GameMatch> for consumed");
        prop_assert_eq!(consumed, enc.len());
    }
}

proptest! {
    #[test]
    fn prop_player_stats_reencode_idempotency(ps in player_stats_strategy()) {
        let enc1 = encode_to_vec(&ps).expect("encode PlayerStats first");
        let (decoded, _): (PlayerStats, usize) =
            decode_from_slice(&enc1).expect("decode PlayerStats after first encode");
        let enc2 = encode_to_vec(&decoded).expect("encode PlayerStats second (re-encode)");
        prop_assert_eq!(enc1, enc2);
    }
}

proptest! {
    #[test]
    fn prop_game_match_reencode_idempotency(gm in game_match_strategy()) {
        let enc1 = encode_to_vec(&gm).expect("encode GameMatch first");
        let (decoded, _): (GameMatch, usize) =
            decode_from_slice(&enc1).expect("decode GameMatch after first encode");
        let enc2 = encode_to_vec(&decoded).expect("encode GameMatch second (re-encode)");
        prop_assert_eq!(enc1, enc2);
    }
}

proptest! {
    #[test]
    fn prop_player_stats_negative_score_roundtrip(
        player_id: u64,
        kills: u32,
        deaths: u32,
        assists: u32,
    ) {
        let ps = PlayerStats {
            player_id,
            kills,
            deaths,
            assists,
            score: i64::MIN,
        };
        let enc = encode_to_vec(&ps).expect("encode PlayerStats with i64::MIN score");
        let (dec, consumed): (PlayerStats, usize) =
            decode_from_slice(&enc).expect("decode PlayerStats with i64::MIN score");
        prop_assert_eq!(ps, dec);
        prop_assert_eq!(consumed, enc.len());
    }
}

proptest! {
    #[test]
    fn prop_game_mode_all_variants_roundtrip(n in 0u8..=4u8) {
        let val = match n {
            0 => GameMode::Ranked,
            1 => GameMode::Casual,
            2 => GameMode::Tournament,
            3 => GameMode::Practice,
            _ => GameMode::Custom,
        };
        let enc = encode_to_vec(&val).expect("encode GameMode variant");
        let (dec, consumed): (GameMode, usize) =
            decode_from_slice(&enc).expect("decode GameMode variant");
        prop_assert_eq!(val, dec);
        prop_assert_eq!(consumed, enc.len());
    }
}

proptest! {
    #[test]
    fn prop_match_outcome_all_variants_roundtrip(n in 0u8..=3u8) {
        let val = match n {
            0 => MatchOutcome::Victory,
            1 => MatchOutcome::Defeat,
            2 => MatchOutcome::Draw,
            _ => MatchOutcome::Disconnected,
        };
        let enc = encode_to_vec(&val).expect("encode MatchOutcome variant");
        let (dec, consumed): (MatchOutcome, usize) =
            decode_from_slice(&enc).expect("decode MatchOutcome variant");
        prop_assert_eq!(val, dec);
        prop_assert_eq!(consumed, enc.len());
    }
}

proptest! {
    #[test]
    fn prop_game_match_ranked_victory_roundtrip(
        match_id: u64,
        duration_s: u32,
        players in prop::collection::vec(player_stats_strategy(), 0..5),
    ) {
        let gm = GameMatch {
            match_id,
            mode: GameMode::Ranked,
            outcome: MatchOutcome::Victory,
            duration_s,
            players,
        };
        let enc = encode_to_vec(&gm).expect("encode GameMatch Ranked/Victory");
        let (dec, consumed): (GameMatch, usize) =
            decode_from_slice(&enc).expect("decode GameMatch Ranked/Victory");
        prop_assert_eq!(gm, dec);
        prop_assert_eq!(consumed, enc.len());
    }
}

proptest! {
    #[test]
    fn prop_game_match_tournament_draw_roundtrip(
        match_id: u64,
        duration_s: u32,
        players in prop::collection::vec(player_stats_strategy(), 0..5),
    ) {
        let gm = GameMatch {
            match_id,
            mode: GameMode::Tournament,
            outcome: MatchOutcome::Draw,
            duration_s,
            players,
        };
        let enc = encode_to_vec(&gm).expect("encode GameMatch Tournament/Draw");
        let (dec, consumed): (GameMatch, usize) =
            decode_from_slice(&enc).expect("decode GameMatch Tournament/Draw");
        prop_assert_eq!(gm, dec);
        prop_assert_eq!(consumed, enc.len());
    }
}

proptest! {
    #[test]
    fn prop_game_match_empty_players_roundtrip(
        match_id: u64,
        mode_n in 0u8..=4u8,
        outcome_n in 0u8..=3u8,
        duration_s: u32,
    ) {
        let gm = GameMatch {
            match_id,
            mode: make_game_mode(mode_n),
            outcome: make_match_outcome(outcome_n),
            duration_s,
            players: Vec::new(),
        };
        let enc = encode_to_vec(&gm).expect("encode GameMatch with empty players");
        let (dec, consumed): (GameMatch, usize) =
            decode_from_slice(&enc).expect("decode GameMatch with empty players");
        prop_assert_eq!(gm, dec);
        prop_assert_eq!(consumed, enc.len());
    }
}
