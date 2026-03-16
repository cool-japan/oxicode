//! Advanced tests for enum encoding edge cases in OxiCode.
//!
//! Covers discriminant/variant encoding, mixed variant types, tag_type attribute,
//! payload variants, container wrapping, config interactions, and error handling.

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
    encode_to_vec_with_config,
};
use oxicode::{Decode, Encode};

// ---- Shared types ----

#[derive(Debug, PartialEq, Encode, Decode)]
enum Direction {
    North,
    South,
    East,
    West,
}

#[derive(Debug, PartialEq, Encode, Decode)]
enum SimplePayload {
    Empty,
    Integer(i64),
    Text(String),
    Bytes(Vec<u8>),
}

#[derive(Debug, PartialEq, Encode, Decode)]
enum Command {
    Quit,
    Move { x: i32, y: i32 },
    Print(String),
    Color(u8, u8, u8),
}

#[derive(Debug, PartialEq, Encode, Decode)]
enum MixedKinds {
    Unit,
    Tuple(u32, bool),
    Struct { label: String, count: u64 },
    Bytes(Vec<u8>),
}

#[derive(Debug, PartialEq, Encode, Decode)]
enum LargePayloadEnum {
    Small(u8),
    Big { data: Vec<u8>, checksum: u64 },
}

#[derive(Debug, PartialEq, Encode, Decode)]
enum VecPayload {
    Empty,
    Data(Vec<u8>),
}

#[derive(Debug, PartialEq, Encode, Decode)]
enum StringPayload {
    None,
    Value(String),
}

#[derive(Debug, PartialEq, Encode, Decode)]
enum OptionPayload {
    Absent,
    MaybeInt(Option<i32>),
    MaybeStr(Option<String>),
}

#[derive(Debug, PartialEq, Encode, Decode)]
enum TuplePayload {
    Unit,
    Pair(i32, i32),
    Triple(u8, u16, u32),
}

// tag_type enums
#[derive(Debug, PartialEq, Encode, Decode)]
#[oxicode(tag_type = "u8")]
enum StatusU8 {
    Ok,
    Err,
    Pending,
}

#[derive(Debug, PartialEq, Encode, Decode)]
#[oxicode(tag_type = "u16")]
enum StatusU16 {
    Ok,
    Err,
    Pending,
}

#[derive(Debug, PartialEq, Encode, Decode)]
#[oxicode(tag_type = "u32")]
enum StatusU32 {
    Ok,
    Err,
    Pending,
}

// Struct containing an enum
#[derive(Debug, PartialEq, Encode, Decode)]
struct Packet {
    id: u64,
    command: Command,
}

// Nested enum
#[derive(Debug, PartialEq, Encode, Decode)]
enum OuterEvent {
    Move(Direction),
    Complex { dir: Direction, steps: u32 },
    Halt,
}

// Many-variant unit enum (8 variants)
#[derive(Debug, PartialEq, Encode, Decode)]
enum OctetVariants {
    V0,
    V1,
    V2,
    V3,
    V4,
    V5,
    V6,
    V7,
}

// C-like enum (all unit variants)
#[derive(Debug, PartialEq, Encode, Decode)]
enum Season {
    Spring,
    Summer,
    Autumn,
    Winter,
}

// ---- Tests ----

/// Test 1: Unit enum — all variants roundtrip and produce sequential 0-based discriminant bytes.
#[test]
fn test_unit_enum_all_variants() {
    for (i, dir) in [
        Direction::North,
        Direction::South,
        Direction::East,
        Direction::West,
    ]
    .iter()
    .enumerate()
    {
        let encoded = encode_to_vec(dir).expect("encode Direction failed");
        let (decoded, consumed): (Direction, _) =
            decode_from_slice(&encoded).expect("decode Direction failed");
        assert_eq!(&decoded, dir, "Direction variant {i} roundtrip mismatch");
        assert_eq!(
            consumed,
            encoded.len(),
            "Direction variant {i} consumed bytes mismatch"
        );
        assert_eq!(
            encoded[0], i as u8,
            "Direction variant {i} discriminant byte mismatch"
        );
    }
}

/// Test 2: Tuple enum — variants with different payload types roundtrip correctly.
#[test]
fn test_tuple_enum_mixed_payloads_roundtrip() {
    let cases = [
        SimplePayload::Empty,
        SimplePayload::Integer(i64::MIN),
        SimplePayload::Text("oxicode-roundtrip".to_string()),
        SimplePayload::Bytes(vec![0xDE, 0xAD, 0xBE, 0xEF, 0x00, 0xFF]),
    ];
    for (i, val) in cases.iter().enumerate() {
        let encoded = encode_to_vec(val).expect("encode SimplePayload failed");
        let (decoded, consumed): (SimplePayload, _) =
            decode_from_slice(&encoded).expect("decode SimplePayload failed");
        assert_eq!(
            &decoded, val,
            "SimplePayload variant {i} roundtrip mismatch"
        );
        assert_eq!(
            consumed,
            encoded.len(),
            "SimplePayload variant {i} consumed bytes mismatch"
        );
    }
}

/// Test 3: Struct enum — named-field variants roundtrip correctly.
#[test]
fn test_struct_enum_named_fields_roundtrip() {
    let move_cmd = Command::Move { x: -128, y: 255 };
    let encoded = encode_to_vec(&move_cmd).expect("encode Command::Move failed");
    let (decoded, consumed): (Command, _) =
        decode_from_slice(&encoded).expect("decode Command::Move failed");
    assert_eq!(decoded, move_cmd, "Command::Move roundtrip mismatch");
    assert_eq!(
        consumed,
        encoded.len(),
        "Command::Move consumed bytes mismatch"
    );
    // Move is variant index 1 → discriminant byte 1
    assert_eq!(encoded[0], 1u8, "Command::Move discriminant must be 1");
}

/// Test 4: Mixed enum (unit + tuple + struct) — all variants roundtrip.
#[test]
fn test_mixed_enum_all_variants_roundtrip() {
    let cases = [
        MixedKinds::Unit,
        MixedKinds::Tuple(42, true),
        MixedKinds::Struct {
            label: "test-label".to_string(),
            count: u64::MAX,
        },
        MixedKinds::Bytes(vec![1, 2, 3, 4, 5]),
    ];
    for (i, val) in cases.iter().enumerate() {
        let encoded = encode_to_vec(val).expect("encode MixedKinds failed");
        let (decoded, consumed): (MixedKinds, _) =
            decode_from_slice(&encoded).expect("decode MixedKinds failed");
        assert_eq!(&decoded, val, "MixedKinds variant {i} roundtrip mismatch");
        assert_eq!(
            consumed,
            encoded.len(),
            "MixedKinds variant {i} consumed bytes mismatch"
        );
        assert_eq!(
            encoded[0], i as u8,
            "MixedKinds variant {i} discriminant byte mismatch"
        );
    }
}

/// Test 5: Enum with large payload in one variant — large Vec<u8> roundtrips correctly.
#[test]
fn test_enum_large_payload_variant_roundtrip() {
    let large_data: Vec<u8> = (0u8..=255u8).cycle().take(4096).collect();
    let val = LargePayloadEnum::Big {
        data: large_data.clone(),
        checksum: 0xDEAD_BEEF_CAFE_0001u64,
    };
    let encoded = encode_to_vec(&val).expect("encode LargePayloadEnum::Big failed");
    let (decoded, consumed): (LargePayloadEnum, _) =
        decode_from_slice(&encoded).expect("decode LargePayloadEnum::Big failed");
    assert_eq!(decoded, val, "LargePayloadEnum::Big roundtrip mismatch");
    assert_eq!(
        consumed,
        encoded.len(),
        "LargePayloadEnum::Big consumed bytes mismatch"
    );
    // Also verify the small variant
    let small = LargePayloadEnum::Small(255u8);
    let enc_small = encode_to_vec(&small).expect("encode LargePayloadEnum::Small failed");
    let (dec_small, _): (LargePayloadEnum, _) =
        decode_from_slice(&enc_small).expect("decode LargePayloadEnum::Small failed");
    assert_eq!(
        dec_small, small,
        "LargePayloadEnum::Small roundtrip mismatch"
    );
}

/// Test 6: Enum with Vec<u8> payload variant — empty and non-empty roundtrip.
#[test]
fn test_enum_vec_u8_payload_roundtrip() {
    let empty = VecPayload::Empty;
    let with_data = VecPayload::Data((0u8..=127u8).collect());
    let with_empty_vec = VecPayload::Data(vec![]);

    for val in [&empty, &with_data, &with_empty_vec] {
        let encoded = encode_to_vec(val).expect("encode VecPayload failed");
        let (decoded, consumed): (VecPayload, _) =
            decode_from_slice(&encoded).expect("decode VecPayload failed");
        assert_eq!(&decoded, val, "VecPayload roundtrip mismatch");
        assert_eq!(
            consumed,
            encoded.len(),
            "VecPayload consumed bytes mismatch"
        );
    }
}

/// Test 7: Enum with String payload variant — empty and non-empty strings roundtrip.
#[test]
fn test_enum_string_payload_roundtrip() {
    let none_val = StringPayload::None;
    let empty_str = StringPayload::Value(String::new());
    let long_str = StringPayload::Value("A".repeat(512));
    let unicode_str = StringPayload::Value("こんにちは世界 — OxiCode".to_string());

    for val in [&none_val, &empty_str, &long_str, &unicode_str] {
        let encoded = encode_to_vec(val).expect("encode StringPayload failed");
        let (decoded, consumed): (StringPayload, _) =
            decode_from_slice(&encoded).expect("decode StringPayload failed");
        assert_eq!(&decoded, val, "StringPayload roundtrip mismatch");
        assert_eq!(
            consumed,
            encoded.len(),
            "StringPayload consumed bytes mismatch"
        );
    }
}

/// Test 8: Enum with Option<T> payload variants — None/Some combinations roundtrip.
#[test]
fn test_enum_option_payload_roundtrip() {
    let cases = vec![
        OptionPayload::Absent,
        OptionPayload::MaybeInt(None),
        OptionPayload::MaybeInt(Some(i32::MIN)),
        OptionPayload::MaybeInt(Some(0)),
        OptionPayload::MaybeStr(None),
        OptionPayload::MaybeStr(Some("option-payload".to_string())),
    ];
    for val in &cases {
        let encoded = encode_to_vec(val).expect("encode OptionPayload failed");
        let (decoded, consumed): (OptionPayload, _) =
            decode_from_slice(&encoded).expect("decode OptionPayload failed");
        assert_eq!(&decoded, val, "OptionPayload roundtrip mismatch");
        assert_eq!(
            consumed,
            encoded.len(),
            "OptionPayload consumed bytes mismatch"
        );
    }
}

/// Test 9: Enum with tuple payload variants of different arities — all roundtrip.
#[test]
fn test_enum_tuple_payload_variants_roundtrip() {
    let cases = [
        TuplePayload::Unit,
        TuplePayload::Pair(i32::MIN, i32::MAX),
        TuplePayload::Triple(u8::MAX, u16::MAX, u32::MAX),
    ];
    for (i, val) in cases.iter().enumerate() {
        let encoded = encode_to_vec(val).expect("encode TuplePayload failed");
        let (decoded, consumed): (TuplePayload, _) =
            decode_from_slice(&encoded).expect("decode TuplePayload failed");
        assert_eq!(&decoded, val, "TuplePayload variant {i} roundtrip mismatch");
        assert_eq!(
            consumed,
            encoded.len(),
            "TuplePayload variant {i} consumed bytes mismatch"
        );
        // Discriminant at index 0
        assert_eq!(
            encoded[0], i as u8,
            "TuplePayload variant {i} discriminant mismatch"
        );
    }
}

/// Test 10: Discriminant byte values — unit enum variant 0 → [0x00], variant 1 → [0x01].
#[test]
fn test_enum_discriminant_byte_values_sequential() {
    let enc_quit = encode_to_vec(&Command::Quit).expect("encode Command::Quit failed");
    assert_eq!(
        enc_quit.len(),
        1,
        "Command::Quit (unit variant) must be 1 byte"
    );
    assert_eq!(
        enc_quit[0], 0x00u8,
        "Command::Quit discriminant must be 0x00"
    );

    let enc_move =
        encode_to_vec(&Command::Move { x: 0, y: 0 }).expect("encode Command::Move failed");
    assert_eq!(
        enc_move[0], 0x01u8,
        "Command::Move discriminant must be 0x01"
    );

    let enc_print =
        encode_to_vec(&Command::Print(String::new())).expect("encode Command::Print failed");
    assert_eq!(
        enc_print[0], 0x02u8,
        "Command::Print discriminant must be 0x02"
    );

    let enc_color = encode_to_vec(&Command::Color(0, 0, 0)).expect("encode Command::Color failed");
    assert_eq!(
        enc_color[0], 0x03u8,
        "Command::Color discriminant must be 0x03"
    );
}

/// Test 11: Enum with tag_type=u8 — discriminant is exactly 1 byte in fixed-int encoding.
#[test]
fn test_enum_tag_type_u8_size() {
    let cfg = config::legacy();
    let enc_ok = encode_to_vec_with_config(&StatusU8::Ok, cfg).expect("encode StatusU8::Ok failed");
    assert_eq!(
        enc_ok.len(),
        1,
        "tag_type=u8 discriminant must be exactly 1 byte"
    );
    assert_eq!(enc_ok[0], 0u8, "StatusU8::Ok discriminant must be 0");

    let enc_err =
        encode_to_vec_with_config(&StatusU8::Err, cfg).expect("encode StatusU8::Err failed");
    assert_eq!(enc_err.len(), 1, "tag_type=u8 Err must be exactly 1 byte");
    assert_eq!(enc_err[0], 1u8, "StatusU8::Err discriminant must be 1");

    let enc_pending = encode_to_vec_with_config(&StatusU8::Pending, cfg)
        .expect("encode StatusU8::Pending failed");
    assert_eq!(
        enc_pending.len(),
        1,
        "tag_type=u8 Pending must be exactly 1 byte"
    );
    assert_eq!(
        enc_pending[0], 2u8,
        "StatusU8::Pending discriminant must be 2"
    );
}

/// Test 12: Enum with tag_type=u16 — discriminant is exactly 2 bytes in fixed-int encoding.
#[test]
fn test_enum_tag_type_u16_size() {
    let cfg = config::legacy();
    let enc_ok =
        encode_to_vec_with_config(&StatusU16::Ok, cfg).expect("encode StatusU16::Ok failed");
    assert_eq!(
        enc_ok.len(),
        2,
        "tag_type=u16 discriminant must be exactly 2 bytes"
    );
    let disc_ok = u16::from_le_bytes([enc_ok[0], enc_ok[1]]);
    assert_eq!(
        disc_ok, 0u16,
        "StatusU16::Ok discriminant (u16 LE) must be 0"
    );

    let enc_err =
        encode_to_vec_with_config(&StatusU16::Err, cfg).expect("encode StatusU16::Err failed");
    assert_eq!(enc_err.len(), 2, "tag_type=u16 Err must be 2 bytes");
    let disc_err = u16::from_le_bytes([enc_err[0], enc_err[1]]);
    assert_eq!(disc_err, 1u16, "StatusU16::Err discriminant must be 1");
}

/// Test 13: Enum with tag_type=u32 — discriminant is exactly 4 bytes in fixed-int encoding.
#[test]
fn test_enum_tag_type_u32_size() {
    let cfg = config::legacy();
    let enc_ok =
        encode_to_vec_with_config(&StatusU32::Ok, cfg).expect("encode StatusU32::Ok failed");
    assert_eq!(
        enc_ok.len(),
        4,
        "tag_type=u32 discriminant must be exactly 4 bytes"
    );
    let disc_ok = u32::from_le_bytes([enc_ok[0], enc_ok[1], enc_ok[2], enc_ok[3]]);
    assert_eq!(
        disc_ok, 0u32,
        "StatusU32::Ok discriminant (u32 LE) must be 0"
    );

    let enc_pending = encode_to_vec_with_config(&StatusU32::Pending, cfg)
        .expect("encode StatusU32::Pending failed");
    assert_eq!(enc_pending.len(), 4, "tag_type=u32 Pending must be 4 bytes");
    let disc_pending = u32::from_le_bytes([
        enc_pending[0],
        enc_pending[1],
        enc_pending[2],
        enc_pending[3],
    ]);
    assert_eq!(
        disc_pending, 2u32,
        "StatusU32::Pending discriminant must be 2"
    );
}

/// Test 14: Vec<MyEnum> roundtrip — encoding a heterogeneous vector of enum variants.
#[test]
fn test_enum_in_vec_roundtrip() {
    let items = vec![
        SimplePayload::Empty,
        SimplePayload::Integer(-1),
        SimplePayload::Text("hello".to_string()),
        SimplePayload::Bytes(vec![0, 1, 2]),
        SimplePayload::Empty,
        SimplePayload::Integer(i64::MAX),
    ];
    let encoded = encode_to_vec(&items).expect("encode Vec<SimplePayload> failed");
    let (decoded, consumed): (Vec<SimplePayload>, _) =
        decode_from_slice(&encoded).expect("decode Vec<SimplePayload> failed");
    assert_eq!(decoded, items, "Vec<SimplePayload> roundtrip mismatch");
    assert_eq!(
        consumed,
        encoded.len(),
        "Vec<SimplePayload> consumed bytes mismatch"
    );
}

/// Test 15: Option<MyEnum> roundtrip — None, Some(unit), Some(data) all roundtrip.
#[test]
fn test_enum_in_option_roundtrip() {
    let none_val: Option<Command> = None;
    let some_quit: Option<Command> = Some(Command::Quit);
    let some_move: Option<Command> = Some(Command::Move { x: 10, y: -20 });
    let some_print: Option<Command> = Some(Command::Print("opt-test".to_string()));

    for val in [&none_val, &some_quit, &some_move, &some_print] {
        let encoded = encode_to_vec(val).expect("encode Option<Command> failed");
        let (decoded, consumed): (Option<Command>, _) =
            decode_from_slice(&encoded).expect("decode Option<Command> failed");
        assert_eq!(&decoded, val, "Option<Command> roundtrip mismatch");
        assert_eq!(
            consumed,
            encoded.len(),
            "Option<Command> consumed bytes mismatch"
        );
    }
}

/// Test 16: Enum as struct field — the surrounding struct roundtrips correctly.
#[test]
fn test_enum_as_struct_field_roundtrip() {
    let cases = vec![
        Packet {
            id: 1,
            command: Command::Quit,
        },
        Packet {
            id: 42,
            command: Command::Move { x: 100, y: -100 },
        },
        Packet {
            id: u64::MAX,
            command: Command::Print("struct-field-test".to_string()),
        },
        Packet {
            id: 0,
            command: Command::Color(255, 128, 0),
        },
    ];
    for val in &cases {
        let encoded = encode_to_vec(val).expect("encode Packet failed");
        let (decoded, consumed): (Packet, _) =
            decode_from_slice(&encoded).expect("decode Packet failed");
        assert_eq!(&decoded, val, "Packet roundtrip mismatch");
        assert_eq!(consumed, encoded.len(), "Packet consumed bytes mismatch");
    }
}

/// Test 17: Nested enum — enum variant containing another enum roundtrips with correct layout.
#[test]
fn test_nested_enum_roundtrip() {
    let cases = vec![
        OuterEvent::Move(Direction::North),
        OuterEvent::Move(Direction::West),
        OuterEvent::Complex {
            dir: Direction::East,
            steps: 99,
        },
        OuterEvent::Halt,
    ];
    for val in &cases {
        let encoded = encode_to_vec(val).expect("encode OuterEvent failed");
        let (decoded, consumed): (OuterEvent, _) =
            decode_from_slice(&encoded).expect("decode OuterEvent failed");
        assert_eq!(&decoded, val, "OuterEvent roundtrip mismatch");
        assert_eq!(
            consumed,
            encoded.len(),
            "OuterEvent consumed bytes mismatch"
        );
    }

    // Binary layout check: OuterEvent::Move(Direction::East) → [0, 2]
    let enc_east = encode_to_vec(&OuterEvent::Move(Direction::East))
        .expect("encode OuterEvent::Move(East) failed");
    assert_eq!(
        enc_east[0], 0u8,
        "OuterEvent::Move outer discriminant must be 0"
    );
    assert_eq!(
        enc_east[1], 2u8,
        "Direction::East inner discriminant must be 2"
    );
}

/// Test 18: Enum with big-endian config — roundtrip succeeds under big-endian byte order.
#[test]
fn test_enum_big_endian_config_roundtrip() {
    let cfg = config::standard().with_big_endian();
    let cases = vec![
        SimplePayload::Empty,
        SimplePayload::Integer(-9876543210i64),
        SimplePayload::Text("big-endian-test".to_string()),
        SimplePayload::Bytes(vec![0xFF, 0x00, 0xAB, 0xCD]),
    ];
    for val in &cases {
        let encoded =
            encode_to_vec_with_config(val, cfg).expect("encode SimplePayload big-endian failed");
        let (decoded, consumed): (SimplePayload, _) = decode_from_slice_with_config(&encoded, cfg)
            .expect("decode SimplePayload big-endian failed");
        assert_eq!(&decoded, val, "SimplePayload big-endian roundtrip mismatch");
        assert_eq!(
            consumed,
            encoded.len(),
            "SimplePayload big-endian consumed bytes mismatch"
        );
    }
}

/// Test 19: Enum with fixed-int encoding config — roundtrip succeeds with Fixint encoding.
#[test]
fn test_enum_fixed_int_config_roundtrip() {
    let cfg = config::standard().with_fixed_int_encoding();
    let cases = vec![
        Command::Quit,
        Command::Move {
            x: i32::MIN,
            y: i32::MAX,
        },
        Command::Print("fixed-int-config".to_string()),
        Command::Color(0, 127, 255),
    ];
    for val in &cases {
        let encoded = encode_to_vec_with_config(val, cfg).expect("encode Command fixed-int failed");
        let (decoded, consumed): (Command, _) =
            decode_from_slice_with_config(&encoded, cfg).expect("decode Command fixed-int failed");
        assert_eq!(&decoded, val, "Command fixed-int roundtrip mismatch");
        assert_eq!(
            consumed,
            encoded.len(),
            "Command fixed-int consumed bytes mismatch"
        );
    }
}

/// Test 20: Invalid discriminant decode failure — decoding a byte sequence with out-of-range
/// discriminant must return an error rather than panic.
#[test]
fn test_enum_invalid_discriminant_decode_failure() {
    // Direction has 4 variants (discriminants 0..=3). A byte of 0xFF is out of range.
    // The varint 0xFF encodes as a multi-byte varint sequence; to keep it simple we
    // use a well-formed varint byte 0x10 (= 16), which is still out of range for Direction.
    let invalid_bytes = [0x10u8]; // varint value 16, no valid Direction variant
    let result: Result<(Direction, usize), _> = decode_from_slice(&invalid_bytes);
    assert!(
        result.is_err(),
        "Decoding a Direction from discriminant 16 must fail, not return a value"
    );
}

/// Test 21: Enum with many unit variants (8) — all variants roundtrip and encode sequentially.
#[test]
fn test_enum_many_unit_variants_roundtrip() {
    let variants = [
        OctetVariants::V0,
        OctetVariants::V1,
        OctetVariants::V2,
        OctetVariants::V3,
        OctetVariants::V4,
        OctetVariants::V5,
        OctetVariants::V6,
        OctetVariants::V7,
    ];
    for (expected_disc, val) in variants.iter().enumerate() {
        let encoded = encode_to_vec(val).expect("encode OctetVariants failed");
        // All discriminants 0..7 fit in a single varint byte
        assert_eq!(
            encoded.len(),
            1,
            "OctetVariants V{expected_disc} must be 1 byte"
        );
        assert_eq!(
            encoded[0], expected_disc as u8,
            "OctetVariants V{expected_disc} discriminant mismatch"
        );
        let (decoded, consumed): (OctetVariants, _) =
            decode_from_slice(&encoded).expect("decode OctetVariants failed");
        assert_eq!(
            &decoded, val,
            "OctetVariants V{expected_disc} roundtrip mismatch"
        );
        assert_eq!(
            consumed, 1,
            "OctetVariants V{expected_disc} consumed bytes must be 1"
        );
    }
}

/// Test 22: C-like enum (all unit variants, sequential discriminants) — full roundtrip
/// and wire-format verification confirming 0-based sequential discriminant bytes.
#[test]
fn test_c_like_enum_sequential_discriminants_and_roundtrip() {
    let variants = [
        Season::Spring,
        Season::Summer,
        Season::Autumn,
        Season::Winter,
    ];
    for (expected_disc, val) in variants.iter().enumerate() {
        let encoded = encode_to_vec(val).expect("encode Season failed");
        assert_eq!(
            encoded.len(),
            1,
            "Season::{val:?} must encode to exactly 1 byte"
        );
        assert_eq!(
            encoded[0], expected_disc as u8,
            "Season::{val:?} discriminant must be {expected_disc}"
        );
        let (decoded, consumed): (Season, _) =
            decode_from_slice(&encoded).expect("decode Season failed");
        assert_eq!(&decoded, val, "Season::{val:?} roundtrip mismatch");
        assert_eq!(consumed, 1, "Season::{val:?} must consume exactly 1 byte");
    }

    // Extra: verify that encoding Spring twice and collecting into a Vec still roundtrips
    let repeated = vec![
        Season::Spring,
        Season::Winter,
        Season::Summer,
        Season::Spring,
    ];
    let enc_vec = encode_to_vec(&repeated).expect("encode Vec<Season> failed");
    let (dec_vec, consumed_vec): (Vec<Season>, _) =
        decode_from_slice(&enc_vec).expect("decode Vec<Season> failed");
    assert_eq!(dec_vec, repeated, "Vec<Season> roundtrip mismatch");
    assert_eq!(
        consumed_vec,
        enc_vec.len(),
        "Vec<Season> consumed bytes mismatch"
    );
}
