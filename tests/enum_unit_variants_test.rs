//! Tests for OxiCode enum serialization focusing on unit variants,
//! discriminants, and encoding details.

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

// ---------------------------------------------------------------------------
// Enum definitions
// ---------------------------------------------------------------------------

#[derive(Encode, Decode, PartialEq, Debug, Clone)]
enum Direction {
    North,
    South,
    East,
    West,
}

#[derive(Encode, Decode, PartialEq, Debug, Clone)]
enum Color {
    Red,
    Green,
    Blue,
}

#[derive(Encode, Decode, PartialEq, Debug, Clone)]
enum Single {
    Only,
}

#[derive(Encode, Decode, PartialEq, Debug, Clone)]
enum Mixed {
    Unit,
    WithData(u32),
    WithString(String),
}

#[derive(Encode, Decode, PartialEq, Debug, Clone)]
#[allow(clippy::enum_variant_names, dead_code)]
enum TaggedEnum {
    VarFirst,
    VarSecond,
    VarThird,
}

// ---------------------------------------------------------------------------
// Helper
// ---------------------------------------------------------------------------

fn roundtrip<T: Encode + Decode + PartialEq + std::fmt::Debug>(value: &T) -> T {
    let encoded = encode_to_vec(value).expect("encode failed");
    let (decoded, bytes_consumed) = decode_from_slice::<T>(&encoded).expect("decode failed");
    assert_eq!(
        bytes_consumed,
        encoded.len(),
        "not all bytes consumed in roundtrip"
    );
    decoded
}

// ---------------------------------------------------------------------------
// Test 1: Direction::North roundtrip
// ---------------------------------------------------------------------------

#[test]
fn direction_north_roundtrip() {
    assert_eq!(roundtrip(&Direction::North), Direction::North);
}

// ---------------------------------------------------------------------------
// Test 2: Direction::South roundtrip
// ---------------------------------------------------------------------------

#[test]
fn direction_south_roundtrip() {
    assert_eq!(roundtrip(&Direction::South), Direction::South);
}

// ---------------------------------------------------------------------------
// Test 3: Direction::East roundtrip
// ---------------------------------------------------------------------------

#[test]
fn direction_east_roundtrip() {
    assert_eq!(roundtrip(&Direction::East), Direction::East);
}

// ---------------------------------------------------------------------------
// Test 4: Direction::West roundtrip
// ---------------------------------------------------------------------------

#[test]
fn direction_west_roundtrip() {
    assert_eq!(roundtrip(&Direction::West), Direction::West);
}

// ---------------------------------------------------------------------------
// Test 5: All Direction variants encode as 1 byte (discriminants 0-3 fit in
//         single-byte varint — varint single-byte max is 250)
// ---------------------------------------------------------------------------

#[test]
fn direction_all_variants_encode_as_one_byte() {
    for variant in &[
        Direction::North,
        Direction::South,
        Direction::East,
        Direction::West,
    ] {
        let bytes = encode_to_vec(variant).expect("encode failed");
        assert_eq!(
            bytes.len(),
            1,
            "{:?} should encode as exactly 1 byte, got {}",
            variant,
            bytes.len()
        );
    }
}

// ---------------------------------------------------------------------------
// Test 6: Color::Red roundtrip
// ---------------------------------------------------------------------------

#[test]
fn color_red_roundtrip() {
    assert_eq!(roundtrip(&Color::Red), Color::Red);
}

// ---------------------------------------------------------------------------
// Test 7: Color::Green roundtrip
// ---------------------------------------------------------------------------

#[test]
fn color_green_roundtrip() {
    assert_eq!(roundtrip(&Color::Green), Color::Green);
}

// ---------------------------------------------------------------------------
// Test 8: Color::Blue roundtrip
// ---------------------------------------------------------------------------

#[test]
fn color_blue_roundtrip() {
    assert_eq!(roundtrip(&Color::Blue), Color::Blue);
}

// ---------------------------------------------------------------------------
// Test 9: Single::Only roundtrip
// ---------------------------------------------------------------------------

#[test]
fn single_only_roundtrip() {
    assert_eq!(roundtrip(&Single::Only), Single::Only);
}

// ---------------------------------------------------------------------------
// Test 10: Single::Only encodes as exactly 1 byte (discriminant 0)
// ---------------------------------------------------------------------------

#[test]
fn single_only_encodes_as_one_byte_discriminant_zero() {
    let bytes = encode_to_vec(&Single::Only).expect("encode failed");
    assert_eq!(bytes.len(), 1, "Single::Only must encode as 1 byte");
    assert_eq!(bytes[0], 0u8, "Single::Only discriminant must be 0");
}

// ---------------------------------------------------------------------------
// Test 11: Mixed::Unit roundtrip
// ---------------------------------------------------------------------------

#[test]
fn mixed_unit_roundtrip() {
    assert_eq!(roundtrip(&Mixed::Unit), Mixed::Unit);
}

// ---------------------------------------------------------------------------
// Test 12: Mixed::WithData(42) roundtrip
// ---------------------------------------------------------------------------

#[test]
fn mixed_with_data_roundtrip() {
    assert_eq!(roundtrip(&Mixed::WithData(42)), Mixed::WithData(42));
}

// ---------------------------------------------------------------------------
// Test 13: Mixed::WithString("hello") roundtrip
// ---------------------------------------------------------------------------

#[test]
fn mixed_with_string_roundtrip() {
    let value = Mixed::WithString(String::from("hello"));
    assert_eq!(roundtrip(&value), value);
}

// ---------------------------------------------------------------------------
// Test 14: Vec<Direction> with all 4 variants
// ---------------------------------------------------------------------------

#[test]
fn vec_direction_all_variants_roundtrip() {
    let directions = vec![
        Direction::North,
        Direction::South,
        Direction::East,
        Direction::West,
    ];
    assert_eq!(roundtrip(&directions), directions);
}

// ---------------------------------------------------------------------------
// Test 15: Option<Direction> Some
// ---------------------------------------------------------------------------

#[test]
fn option_direction_some_roundtrip() {
    let value: Option<Direction> = Some(Direction::East);
    assert_eq!(roundtrip(&value), value);
}

// ---------------------------------------------------------------------------
// Test 16: Option<Direction> None
// ---------------------------------------------------------------------------

#[test]
fn option_direction_none_roundtrip() {
    let value: Option<Direction> = None;
    assert_eq!(roundtrip(&value), value);
}

// ---------------------------------------------------------------------------
// Test 17: Fixed-int config with Direction roundtrip
// ---------------------------------------------------------------------------

#[test]
fn direction_fixed_int_config_roundtrip() {
    let cfg = config::legacy();
    let encoded =
        encode_to_vec_with_config(&Direction::North, cfg).expect("fixed-int encode failed");
    let (decoded, consumed): (Direction, usize) =
        decode_from_slice_with_config(&encoded, cfg).expect("fixed-int decode failed");
    assert_eq!(decoded, Direction::North);
    assert_eq!(consumed, encoded.len());
}

// ---------------------------------------------------------------------------
// Test 18: Big-endian config with Direction roundtrip
// ---------------------------------------------------------------------------

#[test]
fn direction_big_endian_config_roundtrip() {
    let cfg = config::standard().with_big_endian();
    let encoded =
        encode_to_vec_with_config(&Direction::West, cfg).expect("big-endian encode failed");
    let (decoded, consumed): (Direction, usize) =
        decode_from_slice_with_config(&encoded, cfg).expect("big-endian decode failed");
    assert_eq!(decoded, Direction::West);
    assert_eq!(consumed, encoded.len());
}

// ---------------------------------------------------------------------------
// Test 19: Discriminant ordering — North=0, South=1, East=2, West=3
//          Verified by inspecting the wire byte directly.
// ---------------------------------------------------------------------------

#[test]
fn direction_discriminant_ordering_wire_bytes() {
    let north_bytes = encode_to_vec(&Direction::North).expect("encode North");
    let south_bytes = encode_to_vec(&Direction::South).expect("encode South");
    let east_bytes = encode_to_vec(&Direction::East).expect("encode East");
    let west_bytes = encode_to_vec(&Direction::West).expect("encode West");

    assert_eq!(north_bytes[0], 0u8, "North discriminant must be 0");
    assert_eq!(south_bytes[0], 1u8, "South discriminant must be 1");
    assert_eq!(east_bytes[0], 2u8, "East discriminant must be 2");
    assert_eq!(west_bytes[0], 3u8, "West discriminant must be 3");
}

// ---------------------------------------------------------------------------
// Test 20: All Direction variants consume exactly 1 byte on decode
// ---------------------------------------------------------------------------

#[test]
fn direction_all_variants_consumed_one_byte() {
    for variant in &[
        Direction::North,
        Direction::South,
        Direction::East,
        Direction::West,
    ] {
        let bytes = encode_to_vec(variant).expect("encode failed");
        let (_decoded, consumed): (Direction, usize) =
            decode_from_slice(&bytes).expect("decode failed");
        assert_eq!(
            consumed, 1,
            "{:?} decode must consume exactly 1 byte, consumed {}",
            variant, consumed
        );
    }
}

// ---------------------------------------------------------------------------
// Test 21: Vec<Mixed> with all 3 variant types roundtrip
// ---------------------------------------------------------------------------

#[test]
fn vec_mixed_all_variant_types_roundtrip() {
    let values = vec![
        Mixed::Unit,
        Mixed::WithData(100),
        Mixed::WithString(String::from("oxicode")),
    ];
    assert_eq!(roundtrip(&values), values);
}

// ---------------------------------------------------------------------------
// Test 22: Struct containing a Direction field roundtrip
// ---------------------------------------------------------------------------

#[derive(Encode, Decode, PartialEq, Debug, Clone)]
struct Waypoint {
    id: u32,
    direction: Direction,
}

#[test]
fn struct_containing_direction_field_roundtrip() {
    let wp = Waypoint {
        id: 7,
        direction: Direction::South,
    };
    assert_eq!(roundtrip(&wp), wp);
}
