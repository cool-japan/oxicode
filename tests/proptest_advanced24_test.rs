//! Advanced property-based roundtrip tests (set 24) using proptest.
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
struct Range2D {
    x_min: i32,
    x_max: i32,
    y_min: i32,
    y_max: i32,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
enum Direction {
    North,
    South,
    East,
    West,
    Custom(i32, i32),
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct Path {
    waypoints: Vec<(i32, i32)>,
    closed: bool,
}

// 1. Range2D roundtrip with arbitrary i32 fields
proptest! {
    #[test]
    fn prop_range2d_roundtrip(x_min: i32, x_max: i32, y_min: i32, y_max: i32) {
        let value = Range2D { x_min, x_max, y_min, y_max };
        let encoded = encode_to_vec(&value).expect("encode Range2D failed");
        let (decoded, _consumed): (Range2D, usize) =
            decode_from_slice(&encoded).expect("decode Range2D failed");
        prop_assert_eq!(decoded, value);
    }
}

// 2. Direction::North roundtrip
proptest! {
    #[test]
    fn prop_direction_north(_unused: u8) {
        let value = Direction::North;
        let encoded = encode_to_vec(&value).expect("encode Direction::North failed");
        let (decoded, _consumed): (Direction, usize) =
            decode_from_slice(&encoded).expect("decode Direction::North failed");
        prop_assert_eq!(decoded, value);
    }
}

// 3. Direction::South roundtrip
proptest! {
    #[test]
    fn prop_direction_south(_unused: u8) {
        let value = Direction::South;
        let encoded = encode_to_vec(&value).expect("encode Direction::South failed");
        let (decoded, _consumed): (Direction, usize) =
            decode_from_slice(&encoded).expect("decode Direction::South failed");
        prop_assert_eq!(decoded, value);
    }
}

// 4. Direction::East roundtrip
proptest! {
    #[test]
    fn prop_direction_east(_unused: u8) {
        let value = Direction::East;
        let encoded = encode_to_vec(&value).expect("encode Direction::East failed");
        let (decoded, _consumed): (Direction, usize) =
            decode_from_slice(&encoded).expect("decode Direction::East failed");
        prop_assert_eq!(decoded, value);
    }
}

// 5. Direction::West roundtrip
proptest! {
    #[test]
    fn prop_direction_west(_unused: u8) {
        let value = Direction::West;
        let encoded = encode_to_vec(&value).expect("encode Direction::West failed");
        let (decoded, _consumed): (Direction, usize) =
            decode_from_slice(&encoded).expect("decode Direction::West failed");
        prop_assert_eq!(decoded, value);
    }
}

// 6. Direction::Custom roundtrip with two arbitrary i32 values
proptest! {
    #[test]
    fn prop_direction_custom(dx: i32, dy: i32) {
        let value = Direction::Custom(dx, dy);
        let encoded = encode_to_vec(&value).expect("encode Direction::Custom failed");
        let (decoded, _consumed): (Direction, usize) =
            decode_from_slice(&encoded).expect("decode Direction::Custom failed");
        prop_assert_eq!(decoded, value);
    }
}

// 7. Path roundtrip with 0..10 waypoints each (i32, i32)
proptest! {
    #[test]
    fn prop_path_roundtrip(
        waypoints in proptest::collection::vec(
            (any::<i32>(), any::<i32>()),
            0..10,
        ),
        closed: bool,
    ) {
        let value = Path { waypoints, closed };
        let encoded = encode_to_vec(&value).expect("encode Path failed");
        let (decoded, _consumed): (Path, usize) =
            decode_from_slice(&encoded).expect("decode Path failed");
        prop_assert_eq!(decoded, value);
    }
}

// 8. Path closed flag roundtrip
proptest! {
    #[test]
    fn prop_path_closed_flag(closed: bool) {
        let value = Path { waypoints: vec![], closed };
        let encoded = encode_to_vec(&value).expect("encode Path closed_flag failed");
        let (decoded, _consumed): (Path, usize) =
            decode_from_slice(&encoded).expect("decode Path closed_flag failed");
        prop_assert_eq!(decoded.closed, closed);
    }
}

// 9. u32 encode/decode identity
proptest! {
    #[test]
    fn prop_u32_encode_decode_identity(value: u32) {
        let encoded = encode_to_vec(&value).expect("encode u32 failed");
        let (decoded, consumed): (u32, usize) =
            decode_from_slice(&encoded).expect("decode u32 failed");
        prop_assert_eq!(decoded, value);
        prop_assert_eq!(consumed, encoded.len());
    }
}

// 10. String no data loss: decoded equals original
proptest! {
    #[test]
    fn prop_string_no_data_loss(value: String) {
        let encoded = encode_to_vec(&value).expect("encode String failed");
        let (decoded, _consumed): (String, usize) =
            decode_from_slice(&encoded).expect("decode String failed");
        prop_assert_eq!(decoded, value);
    }
}

// 11. Vec<Direction> roundtrip (0..15 elements)
proptest! {
    #[test]
    fn prop_vec_direction_roundtrip(
        v in proptest::collection::vec(
            prop_oneof![
                Just(Direction::North),
                Just(Direction::South),
                Just(Direction::East),
                Just(Direction::West),
                (any::<i32>(), any::<i32>()).prop_map(|(dx, dy)| Direction::Custom(dx, dy)),
            ],
            0..15,
        )
    ) {
        let encoded = encode_to_vec(&v).expect("encode Vec<Direction> failed");
        let (decoded, consumed): (Vec<Direction>, usize) =
            decode_from_slice(&encoded).expect("decode Vec<Direction> failed");
        prop_assert_eq!(decoded, v);
        prop_assert_eq!(consumed, encoded.len());
    }
}

// 12. Consumed bytes for Range2D equals encoded length
proptest! {
    #[test]
    fn prop_consumed_bytes_range2d(x_min: i32, x_max: i32, y_min: i32, y_max: i32) {
        let value = Range2D { x_min, x_max, y_min, y_max };
        let encoded = encode_to_vec(&value).expect("encode Range2D for consumed check failed");
        let (_decoded, consumed): (Range2D, usize) =
            decode_from_slice(&encoded).expect("decode Range2D for consumed check failed");
        prop_assert_eq!(consumed, encoded.len());
    }
}

// 13. Option<Direction> roundtrip using Just(Direction::North) or Custom
proptest! {
    #[test]
    fn prop_option_direction(is_some: bool, dx: i32, dy: i32) {
        let value: Option<Direction> = if is_some {
            Some(Direction::Custom(dx, dy))
        } else {
            None
        };
        let encoded = encode_to_vec(&value).expect("encode Option<Direction> failed");
        let (decoded, consumed): (Option<Direction>, usize) =
            decode_from_slice(&encoded).expect("decode Option<Direction> failed");
        prop_assert_eq!(decoded, value);
        prop_assert_eq!(consumed, encoded.len());
    }
}

// 14. i32 big-endian + fixed_int roundtrip
proptest! {
    #[test]
    fn prop_big_endian_i32_roundtrip(value: i32) {
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

// 15. fixed_int u64 encodes to exactly 8 bytes
proptest! {
    #[test]
    fn prop_fixed_int_u64_8_bytes(value: u64) {
        let cfg = config::standard().with_fixed_int_encoding();
        let encoded =
            encode_to_vec_with_config(&value, cfg).expect("encode u64 fixed_int failed");
        prop_assert_eq!(encoded.len(), 8usize);
        let (decoded, _consumed): (u64, usize) =
            decode_from_slice_with_config(&encoded, cfg)
                .expect("decode u64 fixed_int failed");
        prop_assert_eq!(decoded, value);
    }
}

// 16. Tuple (i32, i32) roundtrip
proptest! {
    #[test]
    fn prop_tuple_i32_i32_roundtrip(a: i32, b: i32) {
        let value = (a, b);
        let encoded = encode_to_vec(&value).expect("encode (i32, i32) failed");
        let (decoded, consumed): ((i32, i32), usize) =
            decode_from_slice(&encoded).expect("decode (i32, i32) failed");
        prop_assert_eq!(decoded, value);
        prop_assert_eq!(consumed, encoded.len());
    }
}

// 17. Vec<Path> roundtrip (0..5 paths)
proptest! {
    #[test]
    fn prop_vec_path_roundtrip(
        v in proptest::collection::vec(
            (
                proptest::collection::vec((any::<i32>(), any::<i32>()), 0..5),
                any::<bool>(),
            ).prop_map(|(waypoints, closed)| Path { waypoints, closed }),
            0..5,
        )
    ) {
        let encoded = encode_to_vec(&v).expect("encode Vec<Path> failed");
        let (decoded, consumed): (Vec<Path>, usize) =
            decode_from_slice(&encoded).expect("decode Vec<Path> failed");
        prop_assert_eq!(decoded, v);
        prop_assert_eq!(consumed, encoded.len());
    }
}

// 18. Different Range2D values produce different encoded bytes
proptest! {
    #[test]
    fn prop_range2d_different_values_different_bytes(
        x_min_a: i32, x_max_a: i32, y_min_a: i32, y_max_a: i32,
        x_min_b: i32, x_max_b: i32, y_min_b: i32, y_max_b: i32,
    ) {
        let a = Range2D { x_min: x_min_a, x_max: x_max_a, y_min: y_min_a, y_max: y_max_a };
        let b = Range2D { x_min: x_min_b, x_max: x_max_b, y_min: y_min_b, y_max: y_max_b };
        prop_assume!(a != b);
        let encoded_a = encode_to_vec(&a).expect("encode Range2D a failed");
        let encoded_b = encode_to_vec(&b).expect("encode Range2D b failed");
        prop_assert_ne!(encoded_a, encoded_b);
    }
}

// 19. char roundtrip with any arbitrary char
proptest! {
    #[test]
    fn prop_char_roundtrip(value: char) {
        let encoded = encode_to_vec(&value).expect("encode char failed");
        let (decoded, _consumed): (char, usize) =
            decode_from_slice(&encoded).expect("decode char failed");
        prop_assert_eq!(decoded, value);
    }
}

// 20. Vec<u8> roundtrip (0..100 bytes)
proptest! {
    #[test]
    fn prop_vec_u8_roundtrip(v in proptest::collection::vec(any::<u8>(), 0..100)) {
        let encoded = encode_to_vec(&v).expect("encode Vec<u8> failed");
        let (decoded, consumed): (Vec<u8>, usize) =
            decode_from_slice(&encoded).expect("decode Vec<u8> failed");
        prop_assert_eq!(decoded, v);
        prop_assert_eq!(consumed, encoded.len());
    }
}

// 21. i64 roundtrip
proptest! {
    #[test]
    fn prop_i64_roundtrip(value: i64) {
        let encoded = encode_to_vec(&value).expect("encode i64 failed");
        let (decoded, consumed): (i64, usize) =
            decode_from_slice(&encoded).expect("decode i64 failed");
        prop_assert_eq!(decoded, value);
        prop_assert_eq!(consumed, encoded.len());
    }
}

// 22. Consumed bytes for Path equals encoded length
proptest! {
    #[test]
    fn prop_consumed_bytes_path(
        waypoints in proptest::collection::vec(
            (any::<i32>(), any::<i32>()),
            0..10,
        ),
        closed: bool,
    ) {
        let value = Path { waypoints, closed };
        let encoded = encode_to_vec(&value).expect("encode Path for consumed check failed");
        let (_decoded, consumed): (Path, usize) =
            decode_from_slice(&encoded).expect("decode Path for consumed check failed");
        prop_assert_eq!(consumed, encoded.len());
    }
}
