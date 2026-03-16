//! Advanced property-based roundtrip tests (set 23) using proptest.
//!
//! Tests verify encode → decode is a perfect roundtrip for various types,
//! including custom derived structs, enums, configs, fixed-int sizes,
//! nested collections, and distinctness of encoded bytes.

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
use proptest::prelude::*;

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct Bounds {
    min: i32,
    max: i32,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
enum Outcome {
    Win,
    Lose,
    Draw,
    Score(u32),
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct Stats {
    games: u32,
    wins: u32,
    losses: u32,
    draws: u32,
}

// 1. Bounds roundtrip with arbitrary i32 fields
proptest! {
    #[test]
    fn prop_bounds_roundtrip(min: i32, max: i32) {
        let value = Bounds { min, max };
        let encoded = encode_to_vec(&value).expect("encode Bounds failed");
        let (decoded, _consumed): (Bounds, usize) =
            decode_from_slice(&encoded).expect("decode Bounds failed");
        prop_assert_eq!(decoded, value);
    }
}

// 2. Outcome::Win roundtrip
proptest! {
    #[test]
    fn prop_outcome_win(_unused: u8) {
        let value = Outcome::Win;
        let encoded = encode_to_vec(&value).expect("encode Outcome::Win failed");
        let (decoded, _consumed): (Outcome, usize) =
            decode_from_slice(&encoded).expect("decode Outcome::Win failed");
        prop_assert_eq!(decoded, value);
    }
}

// 3. Outcome::Lose roundtrip
proptest! {
    #[test]
    fn prop_outcome_lose(_unused: u8) {
        let value = Outcome::Lose;
        let encoded = encode_to_vec(&value).expect("encode Outcome::Lose failed");
        let (decoded, _consumed): (Outcome, usize) =
            decode_from_slice(&encoded).expect("decode Outcome::Lose failed");
        prop_assert_eq!(decoded, value);
    }
}

// 4. Outcome::Draw roundtrip
proptest! {
    #[test]
    fn prop_outcome_draw(_unused: u8) {
        let value = Outcome::Draw;
        let encoded = encode_to_vec(&value).expect("encode Outcome::Draw failed");
        let (decoded, _consumed): (Outcome, usize) =
            decode_from_slice(&encoded).expect("decode Outcome::Draw failed");
        prop_assert_eq!(decoded, value);
    }
}

// 5. Outcome::Score roundtrip with arbitrary u32
proptest! {
    #[test]
    fn prop_outcome_score(n: u32) {
        let value = Outcome::Score(n);
        let encoded = encode_to_vec(&value).expect("encode Outcome::Score failed");
        let (decoded, _consumed): (Outcome, usize) =
            decode_from_slice(&encoded).expect("decode Outcome::Score failed");
        prop_assert_eq!(decoded, value);
    }
}

// 6. Stats roundtrip with all four u32 fields
proptest! {
    #[test]
    fn prop_stats_roundtrip(games: u32, wins: u32, losses: u32, draws: u32) {
        let value = Stats { games, wins, losses, draws };
        let encoded = encode_to_vec(&value).expect("encode Stats failed");
        let (decoded, _consumed): (Stats, usize) =
            decode_from_slice(&encoded).expect("decode Stats failed");
        prop_assert_eq!(decoded, value);
    }
}

// 7. u32 big-endian + fixed_int roundtrip
proptest! {
    #[test]
    fn prop_u32_big_endian_roundtrip(value: u32) {
        let cfg = config::standard().with_big_endian().with_fixed_int_encoding();
        let encoded =
            encode_to_vec_with_config(&value, cfg).expect("encode u32 big_endian fixed_int failed");
        let (decoded, consumed): (u32, usize) =
            decode_from_slice_with_config(&encoded, cfg)
                .expect("decode u32 big_endian fixed_int failed");
        prop_assert_eq!(decoded, value);
        prop_assert_eq!(consumed, encoded.len());
    }
}

// 8. i32 big-endian + fixed_int roundtrip
proptest! {
    #[test]
    fn prop_i32_big_endian_roundtrip(value: i32) {
        let cfg = config::standard().with_big_endian().with_fixed_int_encoding();
        let encoded =
            encode_to_vec_with_config(&value, cfg).expect("encode i32 big_endian fixed_int failed");
        let (decoded, consumed): (i32, usize) =
            decode_from_slice_with_config(&encoded, cfg)
                .expect("decode i32 big_endian fixed_int failed");
        prop_assert_eq!(decoded, value);
        prop_assert_eq!(consumed, encoded.len());
    }
}

// 9. Vec<i32> roundtrip (0..50 elements)
proptest! {
    #[test]
    fn prop_vec_i32_roundtrip(v in proptest::collection::vec(any::<i32>(), 0..50)) {
        let encoded = encode_to_vec(&v).expect("encode Vec<i32> failed");
        let (decoded, consumed): (Vec<i32>, usize) =
            decode_from_slice(&encoded).expect("decode Vec<i32> failed");
        prop_assert_eq!(decoded, v);
        prop_assert_eq!(consumed, encoded.len());
    }
}

// 10. Vec<bool> roundtrip
proptest! {
    #[test]
    fn prop_vec_bool_roundtrip(v in proptest::collection::vec(any::<bool>(), 0..64)) {
        let encoded = encode_to_vec(&v).expect("encode Vec<bool> failed");
        let (decoded, consumed): (Vec<bool>, usize) =
            decode_from_slice(&encoded).expect("decode Vec<bool> failed");
        prop_assert_eq!(decoded, v);
        prop_assert_eq!(consumed, encoded.len());
    }
}

// 11. char roundtrip with arbitrary char
proptest! {
    #[test]
    fn prop_char_roundtrip(value: char) {
        let encoded = encode_to_vec(&value).expect("encode char failed");
        let (decoded, _consumed): (char, usize) =
            decode_from_slice(&encoded).expect("decode char failed");
        prop_assert_eq!(decoded, value);
    }
}

// 12. consumed bytes for Stats equals encoded length
proptest! {
    #[test]
    fn prop_consumed_bytes_stats(games: u32, wins: u32, losses: u32, draws: u32) {
        let value = Stats { games, wins, losses, draws };
        let encoded = encode_to_vec(&value).expect("encode Stats for consumed check failed");
        let (_decoded, consumed): (Stats, usize) =
            decode_from_slice(&encoded).expect("decode Stats for consumed check failed");
        prop_assert_eq!(consumed, encoded.len());
    }
}

// 13. consumed bytes for Vec<u64> equals encoded length
proptest! {
    #[test]
    fn prop_consumed_bytes_vec_u64(v in proptest::collection::vec(any::<u64>(), 0..30)) {
        let encoded = encode_to_vec(&v).expect("encode Vec<u64> for consumed check failed");
        let (_decoded, consumed): (Vec<u64>, usize) =
            decode_from_slice(&encoded).expect("decode Vec<u64> for consumed check failed");
        prop_assert_eq!(consumed, encoded.len());
    }
}

// 14. f32 roundtrip using a finite range
proptest! {
    #[test]
    fn prop_f32_roundtrip(value in -1e6f32..1e6f32) {
        let encoded = encode_to_vec(&value).expect("encode f32 failed");
        let (decoded, _consumed): (f32, usize) =
            decode_from_slice(&encoded).expect("decode f32 failed");
        prop_assert_eq!(decoded, value);
    }
}

// 15. f64 roundtrip using a finite range
proptest! {
    #[test]
    fn prop_f64_roundtrip(value in -1e12f64..1e12f64) {
        let encoded = encode_to_vec(&value).expect("encode f64 failed");
        let (decoded, _consumed): (f64, usize) =
            decode_from_slice(&encoded).expect("decode f64 failed");
        prop_assert_eq!(decoded, value);
    }
}

// 16. 4-tuple (u8, u16, u32, u64) roundtrip
proptest! {
    #[test]
    fn prop_tuple_4_roundtrip(a: u8, b: u16, c: u32, d: u64) {
        let value = (a, b, c, d);
        let encoded = encode_to_vec(&value).expect("encode (u8,u16,u32,u64) failed");
        let (decoded, consumed): ((u8, u16, u32, u64), usize) =
            decode_from_slice(&encoded).expect("decode (u8,u16,u32,u64) failed");
        prop_assert_eq!(decoded, value);
        prop_assert_eq!(consumed, encoded.len());
    }
}

// 17. Vec<Outcome> roundtrip (0..20 elements)
proptest! {
    #[test]
    fn prop_vec_outcome_roundtrip(
        v in proptest::collection::vec(
            prop_oneof![
                Just(Outcome::Win),
                Just(Outcome::Lose),
                Just(Outcome::Draw),
                any::<u32>().prop_map(Outcome::Score),
            ],
            0..20,
        )
    ) {
        let encoded = encode_to_vec(&v).expect("encode Vec<Outcome> failed");
        let (decoded, consumed): (Vec<Outcome>, usize) =
            decode_from_slice(&encoded).expect("decode Vec<Outcome> failed");
        prop_assert_eq!(decoded, v);
        prop_assert_eq!(consumed, encoded.len());
    }
}

// 18. Option<Stats> roundtrip
proptest! {
    #[test]
    fn prop_option_stats_roundtrip(games: u32, wins: u32, losses: u32, draws: u32,
                                   is_some: bool) {
        let value: Option<Stats> = if is_some {
            Some(Stats { games, wins, losses, draws })
        } else {
            None
        };
        let encoded = encode_to_vec(&value).expect("encode Option<Stats> failed");
        let (decoded, consumed): (Option<Stats>, usize) =
            decode_from_slice(&encoded).expect("decode Option<Stats> failed");
        prop_assert_eq!(decoded, value);
        prop_assert_eq!(consumed, encoded.len());
    }
}

// 19. Different Stats values produce different encoded bytes
proptest! {
    #[test]
    fn prop_different_stats_different_bytes(
        games_a: u32, wins_a: u32, losses_a: u32, draws_a: u32,
        games_b: u32, wins_b: u32, losses_b: u32, draws_b: u32,
    ) {
        let a = Stats { games: games_a, wins: wins_a, losses: losses_a, draws: draws_a };
        let b = Stats { games: games_b, wins: wins_b, losses: losses_b, draws: draws_b };
        prop_assume!(a != b);
        let encoded_a = encode_to_vec(&a).expect("encode Stats a failed");
        let encoded_b = encode_to_vec(&b).expect("encode Stats b failed");
        prop_assert_ne!(encoded_a, encoded_b);
    }
}

// 20. Vec<Bounds> roundtrip (0..15 elements)
proptest! {
    #[test]
    fn prop_vec_bounds_roundtrip(
        v in proptest::collection::vec(
            (any::<i32>(), any::<i32>()).prop_map(|(min, max)| Bounds { min, max }),
            0..15,
        )
    ) {
        let encoded = encode_to_vec(&v).expect("encode Vec<Bounds> failed");
        let (decoded, consumed): (Vec<Bounds>, usize) =
            decode_from_slice(&encoded).expect("decode Vec<Bounds> failed");
        prop_assert_eq!(decoded, v);
        prop_assert_eq!(consumed, encoded.len());
    }
}

// 21. Option<Option<i32>> nested roundtrip
proptest! {
    #[test]
    fn prop_nested_option_roundtrip(
        outer in proptest::option::of(proptest::option::of(any::<i32>()))
    ) {
        let encoded = encode_to_vec(&outer).expect("encode Option<Option<i32>> failed");
        let (decoded, consumed): (Option<Option<i32>>, usize) =
            decode_from_slice(&encoded).expect("decode Option<Option<i32>> failed");
        prop_assert_eq!(decoded, outer);
        prop_assert_eq!(consumed, encoded.len());
    }
}

// 22. String roundtrip: decoded string has same length as original
proptest! {
    #[test]
    fn prop_string_roundtrip_length_check(value: String) {
        let original_len = value.len();
        let encoded = encode_to_vec(&value).expect("encode String for length check failed");
        let (decoded, consumed): (String, usize) =
            decode_from_slice(&encoded).expect("decode String for length check failed");
        prop_assert_eq!(decoded.len(), original_len);
        prop_assert_eq!(consumed, encoded.len());
    }
}
