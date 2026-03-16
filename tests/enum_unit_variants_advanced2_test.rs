//! Advanced tests for unit (fieldless) enum encoding in OxiCode.
//!
//! Covers 22 scenarios:
//!  1.  Simple 4-variant compass enum roundtrip
//!  2.  Binary unit enum (On/Off) roundtrip
//!  3.  Discriminant starts at 0
//!  4.  Large 8-variant unit enum roundtrip
//!  5.  Consumed bytes == encoded length
//!  6.  Wire size is 1 byte (varint for discriminants 0–250)
//!  7.  Big-endian config roundtrip
//!  8.  Fixed-int config roundtrip
//!  9.  Vec<UnitEnum> roundtrip
//! 10.  Option<UnitEnum> Some roundtrip
//! 11.  Option<UnitEnum> None roundtrip
//! 12.  All variants encode to distinct bytes
//! 13.  Struct containing a unit enum field roundtrip
//! 14.  Vec<(UnitEnum, String)> roundtrip
//! 15.  Re-encode decoded value yields identical bytes
//! 16.  tag_type = "u8" roundtrip
//! 17.  Color enum (R/G/B) — all variants roundtrip
//! 18.  Direction enum — 4 variants, decoded values match originals
//! 19.  Status enum — Active/Inactive/Pending roundtrip
//! 20.  Season enum — Spring/Summer/Autumn/Winter roundtrip
//! 21.  (UnitEnum, UnitEnum) tuple roundtrip
//! 22.  Default::default() (first variant) roundtrip

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
use oxicode::{config, Decode, Encode};

// ---------------------------------------------------------------------------
// Shared enums
// ---------------------------------------------------------------------------

#[derive(Debug, PartialEq, Clone, Encode, Decode, Default)]
enum Compass {
    #[default]
    North,
    South,
    East,
    West,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
enum Switch {
    On,
    Off,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode, Default)]
enum Planet {
    #[default]
    Mercury,
    Venus,
    Earth,
    Mars,
    Jupiter,
    Saturn,
    Uranus,
    Neptune,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode, Default)]
enum ColorRgb {
    #[default]
    Red,
    Green,
    Blue,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode, Default)]
enum Direction {
    #[default]
    Up,
    Down,
    Left,
    Right,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode, Default)]
enum Status {
    #[default]
    Active,
    Inactive,
    Pending,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode, Default)]
enum Season {
    #[default]
    Spring,
    Summer,
    Autumn,
    Winter,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
#[oxicode(tag_type = "u8")]
enum TagU8Enum {
    Alpha,
    Beta,
    Gamma,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct WithEnum {
    id: u32,
    direction: Direction,
}

// ---------------------------------------------------------------------------
// Helper
// ---------------------------------------------------------------------------

fn encode_bytes<T: Encode>(val: &T) -> Vec<u8> {
    oxicode::encode_to_vec(val).expect("encode failed")
}

fn roundtrip_val<T: Encode + Decode + PartialEq + std::fmt::Debug>(val: T) -> T {
    let encoded = encode_bytes(&val);
    let (decoded, _bytes): (T, usize) =
        oxicode::decode_from_slice(&encoded).expect("decode failed");
    decoded
}

// ---------------------------------------------------------------------------
// Test 1: Simple 4-variant compass enum roundtrip (North/South/East/West)
// ---------------------------------------------------------------------------

#[test]
fn unit_enum_adv2_compass_roundtrip() {
    assert_eq!(roundtrip_val(Compass::North), Compass::North);
    assert_eq!(roundtrip_val(Compass::South), Compass::South);
    assert_eq!(roundtrip_val(Compass::East), Compass::East);
    assert_eq!(roundtrip_val(Compass::West), Compass::West);
}

// ---------------------------------------------------------------------------
// Test 2: Binary unit enum (On/Off) roundtrip
// ---------------------------------------------------------------------------

#[test]
fn unit_enum_adv2_switch_on_off_roundtrip() {
    assert_eq!(roundtrip_val(Switch::On), Switch::On);
    assert_eq!(roundtrip_val(Switch::Off), Switch::Off);
}

// ---------------------------------------------------------------------------
// Test 3: Unit enum discriminant starts at 0
// ---------------------------------------------------------------------------

#[test]
fn unit_enum_adv2_discriminant_starts_at_zero() {
    let bytes = encode_bytes(&Compass::North);
    assert_eq!(bytes.len(), 1, "North should encode as a single byte");
    assert_eq!(bytes[0], 0u8, "first variant discriminant must be 0");
}

// ---------------------------------------------------------------------------
// Test 4: Large 8-variant unit enum roundtrip
// ---------------------------------------------------------------------------

#[test]
fn unit_enum_adv2_large_eight_variant_roundtrip() {
    let variants = [
        Planet::Mercury,
        Planet::Venus,
        Planet::Earth,
        Planet::Mars,
        Planet::Jupiter,
        Planet::Saturn,
        Planet::Uranus,
        Planet::Neptune,
    ];
    for variant in variants {
        assert_eq!(roundtrip_val(variant.clone()), variant);
    }
}

// ---------------------------------------------------------------------------
// Test 5: Consumed bytes equal encoded length
// ---------------------------------------------------------------------------

#[test]
fn unit_enum_adv2_consumed_equals_encoded_len() {
    for variant in [Status::Active, Status::Inactive, Status::Pending] {
        let encoded = encode_bytes(&variant);
        let (_val, consumed): (Status, usize) =
            oxicode::decode_from_slice(&encoded).expect("decode failed");
        assert_eq!(
            consumed,
            encoded.len(),
            "consumed should equal encoded length for {:?}",
            variant
        );
    }
}

// ---------------------------------------------------------------------------
// Test 6: Unit enum wire size is 1 byte (varint for discriminants 0–250)
// ---------------------------------------------------------------------------

#[test]
fn unit_enum_adv2_wire_size_one_byte() {
    for (i, variant) in [
        Season::Spring,
        Season::Summer,
        Season::Autumn,
        Season::Winter,
    ]
    .iter()
    .enumerate()
    {
        let bytes = encode_bytes(variant);
        assert_eq!(
            bytes.len(),
            1,
            "{:?} (discriminant {}) should be exactly 1 byte",
            variant,
            i
        );
    }
}

// ---------------------------------------------------------------------------
// Test 7: Big-endian config roundtrip
// ---------------------------------------------------------------------------

#[test]
fn unit_enum_adv2_big_endian_config_roundtrip() {
    let cfg = config::standard().with_big_endian();
    for variant in [Compass::North, Compass::South, Compass::East, Compass::West] {
        let encoded =
            oxicode::encode_to_vec_with_config(&variant, cfg).expect("encode with big_endian");
        let (decoded, _bytes): (Compass, usize) =
            oxicode::decode_from_slice_with_config(&encoded, cfg).expect("decode with big_endian");
        assert_eq!(decoded, variant);
    }
}

// ---------------------------------------------------------------------------
// Test 8: Fixed-int config roundtrip
// ---------------------------------------------------------------------------

#[test]
fn unit_enum_adv2_fixed_int_config_roundtrip() {
    let cfg = config::standard().with_fixed_int_encoding();
    for variant in [
        Direction::Up,
        Direction::Down,
        Direction::Left,
        Direction::Right,
    ] {
        let encoded =
            oxicode::encode_to_vec_with_config(&variant, cfg).expect("encode with fixed_int");
        let (decoded, _bytes): (Direction, usize) =
            oxicode::decode_from_slice_with_config(&encoded, cfg).expect("decode with fixed_int");
        assert_eq!(decoded, variant);
    }
}

// ---------------------------------------------------------------------------
// Test 9: Vec<UnitEnum> roundtrip
// ---------------------------------------------------------------------------

#[test]
fn unit_enum_adv2_vec_roundtrip() {
    let list = vec![
        ColorRgb::Red,
        ColorRgb::Green,
        ColorRgb::Blue,
        ColorRgb::Red,
        ColorRgb::Blue,
    ];
    let encoded = encode_bytes(&list);
    let (decoded, _bytes): (Vec<ColorRgb>, usize) =
        oxicode::decode_from_slice(&encoded).expect("decode Vec<ColorRgb>");
    assert_eq!(decoded, list);
}

// ---------------------------------------------------------------------------
// Test 10: Option<UnitEnum> Some roundtrip
// ---------------------------------------------------------------------------

#[test]
fn unit_enum_adv2_option_some_roundtrip() {
    let val: Option<Switch> = Some(Switch::On);
    let encoded = encode_bytes(&val);
    let (decoded, _bytes): (Option<Switch>, usize) =
        oxicode::decode_from_slice(&encoded).expect("decode Option<Switch> Some");
    assert_eq!(decoded, val);
}

// ---------------------------------------------------------------------------
// Test 11: Option<UnitEnum> None roundtrip
// ---------------------------------------------------------------------------

#[test]
fn unit_enum_adv2_option_none_roundtrip() {
    let val: Option<Switch> = None;
    let encoded = encode_bytes(&val);
    let (decoded, _bytes): (Option<Switch>, usize) =
        oxicode::decode_from_slice(&encoded).expect("decode Option<Switch> None");
    assert_eq!(decoded, val);
}

// ---------------------------------------------------------------------------
// Test 12: All unit variants encode to distinct byte sequences
// ---------------------------------------------------------------------------

#[test]
fn unit_enum_adv2_all_variants_distinct_bytes() {
    let encodings: Vec<Vec<u8>> = [
        Season::Spring,
        Season::Summer,
        Season::Autumn,
        Season::Winter,
    ]
    .iter()
    .map(encode_bytes)
    .collect();

    for i in 0..encodings.len() {
        for j in (i + 1)..encodings.len() {
            assert_ne!(
                encodings[i], encodings[j],
                "variant {} and {} must produce distinct encodings",
                i, j
            );
        }
    }
}

// ---------------------------------------------------------------------------
// Test 13: Struct with unit enum field roundtrip
// ---------------------------------------------------------------------------

#[test]
fn unit_enum_adv2_struct_with_enum_field_roundtrip() {
    let val = WithEnum {
        id: 42,
        direction: Direction::Left,
    };
    let encoded = encode_bytes(&val);
    let (decoded, _bytes): (WithEnum, usize) =
        oxicode::decode_from_slice(&encoded).expect("decode WithEnum");
    assert_eq!(decoded, val);
}

// ---------------------------------------------------------------------------
// Test 14: Vec<(UnitEnum, String)> roundtrip
// ---------------------------------------------------------------------------

#[test]
fn unit_enum_adv2_vec_pairs_roundtrip() {
    let pairs: Vec<(Status, String)> = vec![
        (Status::Active, "active entry".into()),
        (Status::Inactive, "inactive entry".into()),
        (Status::Pending, "pending entry".into()),
    ];
    let encoded = encode_bytes(&pairs);
    let (decoded, _bytes): (Vec<(Status, String)>, usize) =
        oxicode::decode_from_slice(&encoded).expect("decode Vec<(Status, String)>");
    assert_eq!(decoded, pairs);
}

// ---------------------------------------------------------------------------
// Test 15: Re-encode decoded value yields identical bytes
// ---------------------------------------------------------------------------

#[test]
fn unit_enum_adv2_reencode_decoded_same_bytes() {
    let original = Compass::East;
    let encoded_first = encode_bytes(&original);
    let (decoded, _bytes): (Compass, usize) =
        oxicode::decode_from_slice(&encoded_first).expect("first decode");
    let encoded_second = encode_bytes(&decoded);
    assert_eq!(
        encoded_first, encoded_second,
        "re-encoding decoded value must produce identical bytes"
    );
}

// ---------------------------------------------------------------------------
// Test 16: tag_type = "u8" roundtrip
// ---------------------------------------------------------------------------

#[test]
fn unit_enum_adv2_tag_type_u8_roundtrip() {
    for variant in [TagU8Enum::Alpha, TagU8Enum::Beta, TagU8Enum::Gamma] {
        let encoded = encode_bytes(&variant);
        let (decoded, _bytes): (TagU8Enum, usize) =
            oxicode::decode_from_slice(&encoded).expect("decode TagU8Enum");
        assert_eq!(decoded, variant);
    }
}

// ---------------------------------------------------------------------------
// Test 17: Color enum (R/G/B) — 3 variants, all roundtrip
// ---------------------------------------------------------------------------

#[test]
fn unit_enum_adv2_color_rgb_all_roundtrip() {
    assert_eq!(roundtrip_val(ColorRgb::Red), ColorRgb::Red);
    assert_eq!(roundtrip_val(ColorRgb::Green), ColorRgb::Green);
    assert_eq!(roundtrip_val(ColorRgb::Blue), ColorRgb::Blue);
}

// ---------------------------------------------------------------------------
// Test 18: Direction enum — 4 variants, decoded values match originals
// ---------------------------------------------------------------------------

#[test]
fn unit_enum_adv2_direction_decoded_matches_original() {
    let variants = [
        Direction::Up,
        Direction::Down,
        Direction::Left,
        Direction::Right,
    ];
    for variant in variants {
        let encoded = encode_bytes(&variant);
        let (decoded, _bytes): (Direction, usize) =
            oxicode::decode_from_slice(&encoded).expect("decode Direction");
        assert_eq!(
            decoded, variant,
            "decoded Direction must match original {:?}",
            variant
        );
    }
}

// ---------------------------------------------------------------------------
// Test 19: Status enum — Active/Inactive/Pending roundtrip
// ---------------------------------------------------------------------------

#[test]
fn unit_enum_adv2_status_roundtrip() {
    assert_eq!(roundtrip_val(Status::Active), Status::Active);
    assert_eq!(roundtrip_val(Status::Inactive), Status::Inactive);
    assert_eq!(roundtrip_val(Status::Pending), Status::Pending);
}

// ---------------------------------------------------------------------------
// Test 20: Season enum — Spring/Summer/Autumn/Winter roundtrip
// ---------------------------------------------------------------------------

#[test]
fn unit_enum_adv2_season_all_roundtrip() {
    assert_eq!(roundtrip_val(Season::Spring), Season::Spring);
    assert_eq!(roundtrip_val(Season::Summer), Season::Summer);
    assert_eq!(roundtrip_val(Season::Autumn), Season::Autumn);
    assert_eq!(roundtrip_val(Season::Winter), Season::Winter);
}

// ---------------------------------------------------------------------------
// Test 21: (UnitEnum, UnitEnum) tuple roundtrip
// ---------------------------------------------------------------------------

#[test]
fn unit_enum_adv2_tuple_roundtrip() {
    let val: (Compass, Season) = (Compass::West, Season::Winter);
    let encoded = encode_bytes(&val);
    let (decoded, _bytes): ((Compass, Season), usize) =
        oxicode::decode_from_slice(&encoded).expect("decode (Compass, Season)");
    assert_eq!(decoded, val);
}

// ---------------------------------------------------------------------------
// Test 22: Default::default() (first variant) roundtrip
// ---------------------------------------------------------------------------

#[test]
fn unit_enum_adv2_default_roundtrip() {
    let val: Compass = Default::default();
    let encoded = encode_bytes(&val);
    let (decoded, _bytes): (Compass, usize) =
        oxicode::decode_from_slice(&encoded).expect("decode Compass default");
    assert_eq!(decoded, val, "Default::default() must roundtrip correctly");
    // The first variant should have discriminant 0.
    assert_eq!(
        encoded[0], 0u8,
        "default (first) variant must encode with discriminant 0"
    );
}
