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
enum Sport {
    Soccer,
    Basketball,
    Tennis,
    Baseball,
    Rugby,
    Cricket,
    Hockey,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
enum EventOutcome {
    Goal,
    Assist,
    Save,
    Foul,
    Penalty,
    Substitution,
    Card,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct AthleteStats {
    athlete_id: u64,
    speed_cms: u32,
    endurance_pct: u8,
    strength_score: u16,
    games_played: u32,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct GameEvent {
    event_id: u64,
    sport: Sport,
    outcome: EventOutcome,
    team_id: u32,
    athlete_id: u64,
    minute: u16,
    score_impact: i8,
}

fn sport_strategy() -> impl Strategy<Value = Sport> {
    (0u8..7).prop_map(|v| match v {
        0 => Sport::Soccer,
        1 => Sport::Basketball,
        2 => Sport::Tennis,
        3 => Sport::Baseball,
        4 => Sport::Rugby,
        5 => Sport::Cricket,
        _ => Sport::Hockey,
    })
}

fn event_outcome_strategy() -> impl Strategy<Value = EventOutcome> {
    (0u8..7).prop_map(|v| match v {
        0 => EventOutcome::Goal,
        1 => EventOutcome::Assist,
        2 => EventOutcome::Save,
        3 => EventOutcome::Foul,
        4 => EventOutcome::Penalty,
        5 => EventOutcome::Substitution,
        _ => EventOutcome::Card,
    })
}

fn athlete_stats_strategy() -> impl Strategy<Value = AthleteStats> {
    (
        any::<u64>(),
        any::<u32>(),
        any::<u8>(),
        any::<u16>(),
        any::<u32>(),
    )
        .prop_map(
            |(athlete_id, speed_cms, endurance_pct, strength_score, games_played)| AthleteStats {
                athlete_id,
                speed_cms,
                endurance_pct,
                strength_score,
                games_played,
            },
        )
}

fn game_event_strategy() -> impl Strategy<Value = GameEvent> {
    (
        any::<u64>(),
        sport_strategy(),
        event_outcome_strategy(),
        any::<u32>(),
        any::<u64>(),
        any::<u16>(),
        any::<i8>(),
    )
        .prop_map(
            |(event_id, sport, outcome, team_id, athlete_id, minute, score_impact)| GameEvent {
                event_id,
                sport,
                outcome,
                team_id,
                athlete_id,
                minute,
                score_impact,
            },
        )
}

proptest! {
    #[test]
    fn test_athlete_stats_roundtrip(stats in athlete_stats_strategy()) {
        let encoded = encode_to_vec(&stats).expect("encode AthleteStats failed");
        let (decoded, _): (AthleteStats, usize) = decode_from_slice(&encoded).expect("decode AthleteStats failed");
        prop_assert_eq!(stats, decoded);
    }

    #[test]
    fn test_game_event_roundtrip(event in game_event_strategy()) {
        let encoded = encode_to_vec(&event).expect("encode GameEvent failed");
        let (decoded, _): (GameEvent, usize) = decode_from_slice(&encoded).expect("decode GameEvent failed");
        prop_assert_eq!(event, decoded);
    }

    #[test]
    fn test_athlete_stats_consumed_bytes_equals_encoded_length(stats in athlete_stats_strategy()) {
        let encoded = encode_to_vec(&stats).expect("encode AthleteStats failed");
        let (_, consumed): (AthleteStats, usize) = decode_from_slice(&encoded).expect("decode AthleteStats failed");
        prop_assert_eq!(consumed, encoded.len());
    }

    #[test]
    fn test_game_event_consumed_bytes_equals_encoded_length(event in game_event_strategy()) {
        let encoded = encode_to_vec(&event).expect("encode GameEvent failed");
        let (_, consumed): (GameEvent, usize) = decode_from_slice(&encoded).expect("decode GameEvent failed");
        prop_assert_eq!(consumed, encoded.len());
    }

    #[test]
    fn test_athlete_stats_encode_deterministic(stats in athlete_stats_strategy()) {
        let encoded_first = encode_to_vec(&stats).expect("first encode AthleteStats failed");
        let encoded_second = encode_to_vec(&stats).expect("second encode AthleteStats failed");
        prop_assert_eq!(encoded_first, encoded_second);
    }

    #[test]
    fn test_game_event_encode_deterministic(event in game_event_strategy()) {
        let encoded_first = encode_to_vec(&event).expect("first encode GameEvent failed");
        let encoded_second = encode_to_vec(&event).expect("second encode GameEvent failed");
        prop_assert_eq!(encoded_first, encoded_second);
    }

    #[test]
    fn test_vec_game_event_roundtrip(events in prop::collection::vec(game_event_strategy(), 0..8)) {
        let encoded = encode_to_vec(&events).expect("encode Vec<GameEvent> failed");
        let (decoded, _): (Vec<GameEvent>, usize) = decode_from_slice(&encoded).expect("decode Vec<GameEvent> failed");
        prop_assert_eq!(events, decoded);
    }

    #[test]
    fn test_option_athlete_stats_roundtrip(maybe_stats in prop::option::of(athlete_stats_strategy())) {
        let encoded = encode_to_vec(&maybe_stats).expect("encode Option<AthleteStats> failed");
        let (decoded, _): (Option<AthleteStats>, usize) = decode_from_slice(&encoded).expect("decode Option<AthleteStats> failed");
        prop_assert_eq!(maybe_stats, decoded);
    }

    #[test]
    fn test_sport_variant_roundtrip(index in 0u8..7) {
        let sport = match index {
            0 => Sport::Soccer,
            1 => Sport::Basketball,
            2 => Sport::Tennis,
            3 => Sport::Baseball,
            4 => Sport::Rugby,
            5 => Sport::Cricket,
            _ => Sport::Hockey,
        };
        let encoded = encode_to_vec(&sport).expect("encode Sport failed");
        let (decoded, _): (Sport, usize) = decode_from_slice(&encoded).expect("decode Sport failed");
        prop_assert_eq!(sport, decoded);
    }

    #[test]
    fn test_event_outcome_variant_roundtrip(index in 0u8..7) {
        let outcome = match index {
            0 => EventOutcome::Goal,
            1 => EventOutcome::Assist,
            2 => EventOutcome::Save,
            3 => EventOutcome::Foul,
            4 => EventOutcome::Penalty,
            5 => EventOutcome::Substitution,
            _ => EventOutcome::Card,
        };
        let encoded = encode_to_vec(&outcome).expect("encode EventOutcome failed");
        let (decoded, _): (EventOutcome, usize) = decode_from_slice(&encoded).expect("decode EventOutcome failed");
        prop_assert_eq!(outcome, decoded);
    }

    #[test]
    fn test_u8_roundtrip(val in any::<u8>()) {
        let encoded = encode_to_vec(&val).expect("encode u8 failed");
        let (decoded, _): (u8, usize) = decode_from_slice(&encoded).expect("decode u8 failed");
        prop_assert_eq!(val, decoded);
    }

    #[test]
    fn test_i32_roundtrip(val in any::<i32>()) {
        let encoded = encode_to_vec(&val).expect("encode i32 failed");
        let (decoded, _): (i32, usize) = decode_from_slice(&encoded).expect("decode i32 failed");
        prop_assert_eq!(val, decoded);
    }

    #[test]
    fn test_u64_roundtrip(val in any::<u64>()) {
        let encoded = encode_to_vec(&val).expect("encode u64 failed");
        let (decoded, _): (u64, usize) = decode_from_slice(&encoded).expect("decode u64 failed");
        prop_assert_eq!(val, decoded);
    }

    #[test]
    fn test_i64_roundtrip(val in any::<i64>()) {
        let encoded = encode_to_vec(&val).expect("encode i64 failed");
        let (decoded, _): (i64, usize) = decode_from_slice(&encoded).expect("decode i64 failed");
        prop_assert_eq!(val, decoded);
    }

    #[test]
    fn test_bool_roundtrip(val in any::<bool>()) {
        let encoded = encode_to_vec(&val).expect("encode bool failed");
        let (decoded, _): (bool, usize) = decode_from_slice(&encoded).expect("decode bool failed");
        prop_assert_eq!(val, decoded);
    }

    #[test]
    fn test_string_roundtrip(val in "\\PC*") {
        let encoded = encode_to_vec(&val).expect("encode String failed");
        let (decoded, _): (String, usize) = decode_from_slice(&encoded).expect("decode String failed");
        prop_assert_eq!(val, decoded);
    }

    #[test]
    fn test_f32_roundtrip(val in any::<f32>()) {
        let encoded = encode_to_vec(&val).expect("encode f32 failed");
        let (decoded, _): (f32, usize) = decode_from_slice(&encoded).expect("decode f32 failed");
        prop_assert_eq!(val.to_bits(), decoded.to_bits());
    }

    #[test]
    fn test_f64_roundtrip(val in any::<f64>()) {
        let encoded = encode_to_vec(&val).expect("encode f64 failed");
        let (decoded, _): (f64, usize) = decode_from_slice(&encoded).expect("decode f64 failed");
        prop_assert_eq!(val.to_bits(), decoded.to_bits());
    }

    #[test]
    fn test_vec_u8_roundtrip(val in prop::collection::vec(any::<u8>(), 0..64)) {
        let encoded = encode_to_vec(&val).expect("encode Vec<u8> failed");
        let (decoded, _): (Vec<u8>, usize) = decode_from_slice(&encoded).expect("decode Vec<u8> failed");
        prop_assert_eq!(val, decoded);
    }

    #[test]
    fn test_vec_string_roundtrip(val in prop::collection::vec("\\PC*", 0..8)) {
        let encoded = encode_to_vec(&val).expect("encode Vec<String> failed");
        let (decoded, _): (Vec<String>, usize) = decode_from_slice(&encoded).expect("decode Vec<String> failed");
        prop_assert_eq!(val, decoded);
    }

    #[test]
    fn test_option_u64_roundtrip(val in prop::option::of(any::<u64>())) {
        let encoded = encode_to_vec(&val).expect("encode Option<u64> failed");
        let (decoded, _): (Option<u64>, usize) = decode_from_slice(&encoded).expect("decode Option<u64> failed");
        prop_assert_eq!(val, decoded);
    }

    #[test]
    fn test_distinct_athlete_stats_bytes_reflect_inequality(
        stats_a in athlete_stats_strategy(),
        stats_b in athlete_stats_strategy()
    ) {
        let encoded_a = encode_to_vec(&stats_a).expect("encode AthleteStats A failed");
        let encoded_b = encode_to_vec(&stats_b).expect("encode AthleteStats B failed");
        if stats_a == stats_b {
            prop_assert_eq!(&encoded_a, &encoded_b);
        } else {
            prop_assert_ne!(&encoded_a, &encoded_b);
        }
    }

    #[test]
    fn test_zero_score_impact_event_roundtrip(
        event_id in any::<u64>(),
        sport in sport_strategy(),
        outcome in event_outcome_strategy(),
        team_id in any::<u32>(),
        athlete_id in any::<u64>(),
        minute in any::<u16>()
    ) {
        let event = GameEvent {
            event_id,
            sport,
            outcome,
            team_id,
            athlete_id,
            minute,
            score_impact: 0i8,
        };
        let encoded = encode_to_vec(&event).expect("encode zero score impact event failed");
        let (decoded, consumed): (GameEvent, usize) = decode_from_slice(&encoded).expect("decode zero score impact event failed");
        prop_assert_eq!(&event, &decoded);
        prop_assert_eq!(decoded.score_impact, 0i8);
        prop_assert_eq!(consumed, encoded.len());
    }

    #[test]
    fn test_max_speed_athlete_roundtrip(
        athlete_id in any::<u64>(),
        endurance_pct in any::<u8>(),
        strength_score in any::<u16>(),
        games_played in any::<u32>()
    ) {
        let stats = AthleteStats {
            athlete_id,
            speed_cms: u32::MAX,
            endurance_pct,
            strength_score,
            games_played,
        };
        let encoded = encode_to_vec(&stats).expect("encode max speed athlete failed");
        let (decoded, consumed): (AthleteStats, usize) = decode_from_slice(&encoded).expect("decode max speed athlete failed");
        prop_assert_eq!(&stats, &decoded);
        prop_assert_eq!(decoded.speed_cms, u32::MAX);
        prop_assert_eq!(consumed, encoded.len());
    }
}
