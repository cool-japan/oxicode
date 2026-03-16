//! Advanced tests for OxiCode serialization of enums with mixed variant types.
//!
//! Covers 22 distinct scenarios testing unit, tuple, and struct variants within
//! the same enum, including roundtrips, wire-format properties, config variants,
//! collections, nesting, and discriminant verification.

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
// Shared enum definitions
// ---------------------------------------------------------------------------

#[derive(Debug, PartialEq, Encode, Decode)]
enum Event {
    Start,
    Stop,
    Data(Vec<u8>),
    Named { id: u32, label: String },
}

#[derive(Debug, PartialEq, Encode, Decode)]
enum Shape {
    Circle { radius: f64 },
    Rectangle { w: f64, h: f64 },
    Point,
}

#[derive(Debug, PartialEq, Encode, Decode)]
enum Command {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(u8, u8, u8),
}

#[derive(Debug, PartialEq, Encode, Decode)]
enum Status {
    Ok,
    Err(String),
    Pending { reason: String, retry_in: u32 },
}

// Used for the nested enum test (test 19).
#[derive(Debug, PartialEq, Encode, Decode)]
enum Inner {
    Leaf,
    Value(u32),
}

#[derive(Debug, PartialEq, Encode, Decode)]
enum Container {
    Empty,
    Wrapped(Inner),
    Tagged { tag: String, inner: Inner },
}

// ---------------------------------------------------------------------------
// Test 1: Event::Start roundtrip
// ---------------------------------------------------------------------------

#[test]
fn test_event_start_roundtrip() {
    let original = Event::Start;
    let encoded = encode_to_vec(&original).expect("encode Event::Start");
    let (decoded, _): (Event, usize) = decode_from_slice(&encoded).expect("decode Event::Start");
    assert_eq!(original, decoded);
}

// ---------------------------------------------------------------------------
// Test 2: Event::Stop roundtrip
// ---------------------------------------------------------------------------

#[test]
fn test_event_stop_roundtrip() {
    let original = Event::Stop;
    let encoded = encode_to_vec(&original).expect("encode Event::Stop");
    let (decoded, _): (Event, usize) = decode_from_slice(&encoded).expect("decode Event::Stop");
    assert_eq!(original, decoded);
}

// ---------------------------------------------------------------------------
// Test 3: Event::Data(vec![1,2,3]) roundtrip
// ---------------------------------------------------------------------------

#[test]
fn test_event_data_roundtrip() {
    let original = Event::Data(vec![1, 2, 3]);
    let encoded = encode_to_vec(&original).expect("encode Event::Data");
    let (decoded, _): (Event, usize) = decode_from_slice(&encoded).expect("decode Event::Data");
    assert_eq!(original, decoded);
}

// ---------------------------------------------------------------------------
// Test 4: Event::Named { id: 42, label: "hello".into() } roundtrip
// ---------------------------------------------------------------------------

#[test]
fn test_event_named_roundtrip() {
    let original = Event::Named {
        id: 42,
        label: "hello".to_string(),
    };
    let encoded = encode_to_vec(&original).expect("encode Event::Named");
    let (decoded, _): (Event, usize) = decode_from_slice(&encoded).expect("decode Event::Named");
    assert_eq!(original, decoded);
}

// ---------------------------------------------------------------------------
// Test 5: Shape::Circle roundtrip
// ---------------------------------------------------------------------------

#[test]
fn test_shape_circle_roundtrip() {
    let original = Shape::Circle {
        radius: std::f64::consts::PI,
    };
    let encoded = encode_to_vec(&original).expect("encode Shape::Circle");
    let (decoded, _): (Shape, usize) = decode_from_slice(&encoded).expect("decode Shape::Circle");
    assert_eq!(original, decoded);
}

// ---------------------------------------------------------------------------
// Test 6: Shape::Rectangle roundtrip
// ---------------------------------------------------------------------------

#[test]
fn test_shape_rectangle_roundtrip() {
    let original = Shape::Rectangle { w: 10.0, h: 5.5 };
    let encoded = encode_to_vec(&original).expect("encode Shape::Rectangle");
    let (decoded, _): (Shape, usize) =
        decode_from_slice(&encoded).expect("decode Shape::Rectangle");
    assert_eq!(original, decoded);
}

// ---------------------------------------------------------------------------
// Test 7: Shape::Point roundtrip
// ---------------------------------------------------------------------------

#[test]
fn test_shape_point_roundtrip() {
    let original = Shape::Point;
    let encoded = encode_to_vec(&original).expect("encode Shape::Point");
    let (decoded, _): (Shape, usize) = decode_from_slice(&encoded).expect("decode Shape::Point");
    assert_eq!(original, decoded);
}

// ---------------------------------------------------------------------------
// Test 8: Wire bytes differ for different variants (assert_ne!)
// ---------------------------------------------------------------------------

#[test]
fn test_different_variants_have_different_wire_bytes() {
    let start_bytes = encode_to_vec(&Event::Start).expect("encode Event::Start");
    let stop_bytes = encode_to_vec(&Event::Stop).expect("encode Event::Stop");
    let data_bytes = encode_to_vec(&Event::Data(vec![])).expect("encode Event::Data");
    let named_bytes = encode_to_vec(&Event::Named {
        id: 0,
        label: String::new(),
    })
    .expect("encode Event::Named");

    assert_ne!(
        start_bytes, stop_bytes,
        "Start and Stop must have different wire bytes"
    );
    assert_ne!(
        start_bytes, data_bytes,
        "Start and Data must have different wire bytes"
    );
    assert_ne!(
        start_bytes, named_bytes,
        "Start and Named must have different wire bytes"
    );
    assert_ne!(
        stop_bytes, data_bytes,
        "Stop and Data must have different wire bytes"
    );
}

// ---------------------------------------------------------------------------
// Test 9: Consumed == encoded.len() for each variant
// ---------------------------------------------------------------------------

#[test]
fn test_consumed_equals_encoded_len_for_all_event_variants() {
    let variants = [
        Event::Start,
        Event::Stop,
        Event::Data(vec![0xFF, 0x00]),
        Event::Named {
            id: 7,
            label: "test".to_string(),
        },
    ];

    for variant in &variants {
        let encoded = encode_to_vec(variant).expect("encode Event variant");
        let (_, consumed): (Event, usize) =
            decode_from_slice(&encoded).expect("decode Event variant");
        assert_eq!(
            consumed,
            encoded.len(),
            "all encoded bytes must be consumed for {:?}",
            variant
        );
    }
}

// ---------------------------------------------------------------------------
// Test 10: Vec<Event> with all 4 variants roundtrip
// ---------------------------------------------------------------------------

#[test]
fn test_vec_event_all_variants_roundtrip() {
    let original = vec![
        Event::Start,
        Event::Stop,
        Event::Data(vec![10, 20, 30]),
        Event::Named {
            id: 99,
            label: "batch".to_string(),
        },
    ];
    let encoded = encode_to_vec(&original).expect("encode Vec<Event>");
    let (decoded, consumed): (Vec<Event>, usize) =
        decode_from_slice(&encoded).expect("decode Vec<Event>");
    assert_eq!(original, decoded);
    assert_eq!(consumed, encoded.len());
}

// ---------------------------------------------------------------------------
// Test 11: Option<Event> Some(Data) roundtrip
// ---------------------------------------------------------------------------

#[test]
fn test_option_event_some_data_roundtrip() {
    let original: Option<Event> = Some(Event::Data(vec![0xAB, 0xCD, 0xEF]));
    let encoded = encode_to_vec(&original).expect("encode Option<Event> Some(Data)");
    let (decoded, consumed): (Option<Event>, usize) =
        decode_from_slice(&encoded).expect("decode Option<Event> Some(Data)");
    assert_eq!(original, decoded);
    assert_eq!(consumed, encoded.len());
}

// ---------------------------------------------------------------------------
// Test 12: Option<Event> None roundtrip
// ---------------------------------------------------------------------------

#[test]
fn test_option_event_none_roundtrip() {
    let original: Option<Event> = None;
    let encoded = encode_to_vec(&original).expect("encode Option<Event> None");
    let (decoded, consumed): (Option<Event>, usize) =
        decode_from_slice(&encoded).expect("decode Option<Event> None");
    assert_eq!(original, decoded);
    assert_eq!(consumed, encoded.len());
}

// ---------------------------------------------------------------------------
// Test 13: Fixed-int config with Event::Named { id: 1, label: "x".into() }
// ---------------------------------------------------------------------------

#[test]
fn test_fixed_int_config_event_named_roundtrip() {
    let original = Event::Named {
        id: 1,
        label: "x".to_string(),
    };
    let cfg = config::legacy();
    let encoded = encode_to_vec_with_config(&original, cfg).expect("legacy encode Event::Named");
    let (decoded, consumed): (Event, usize) =
        decode_from_slice_with_config(&encoded, cfg).expect("legacy decode Event::Named");
    assert_eq!(original, decoded);
    assert_eq!(consumed, encoded.len());
}

// ---------------------------------------------------------------------------
// Test 14: Big-endian config with Shape::Circle { radius: PI }
// ---------------------------------------------------------------------------

#[test]
fn test_big_endian_config_shape_circle_roundtrip() {
    let original = Shape::Circle {
        radius: std::f64::consts::PI,
    };
    let cfg = config::standard().with_big_endian();
    let encoded =
        encode_to_vec_with_config(&original, cfg).expect("big-endian encode Shape::Circle");
    let (decoded, consumed): (Shape, usize) =
        decode_from_slice_with_config(&encoded, cfg).expect("big-endian decode Shape::Circle");
    assert_eq!(original, decoded);
    assert_eq!(consumed, encoded.len());
}

// ---------------------------------------------------------------------------
// Test 15: Command::Quit roundtrip
// ---------------------------------------------------------------------------

#[test]
fn test_command_quit_roundtrip() {
    let original = Command::Quit;
    let encoded = encode_to_vec(&original).expect("encode Command::Quit");
    let (decoded, _): (Command, usize) = decode_from_slice(&encoded).expect("decode Command::Quit");
    assert_eq!(original, decoded);
}

// ---------------------------------------------------------------------------
// Test 16: Command::Move roundtrip
// ---------------------------------------------------------------------------

#[test]
fn test_command_move_roundtrip() {
    let original = Command::Move { x: -5, y: 100 };
    let encoded = encode_to_vec(&original).expect("encode Command::Move");
    let (decoded, _): (Command, usize) = decode_from_slice(&encoded).expect("decode Command::Move");
    assert_eq!(original, decoded);
}

// ---------------------------------------------------------------------------
// Test 17: Command::Write roundtrip
// ---------------------------------------------------------------------------

#[test]
fn test_command_write_roundtrip() {
    let original = Command::Write("hello world".to_string());
    let encoded = encode_to_vec(&original).expect("encode Command::Write");
    let (decoded, _): (Command, usize) =
        decode_from_slice(&encoded).expect("decode Command::Write");
    assert_eq!(original, decoded);
}

// ---------------------------------------------------------------------------
// Test 18: Command::ChangeColor roundtrip
// ---------------------------------------------------------------------------

#[test]
fn test_command_change_color_roundtrip() {
    let original = Command::ChangeColor(255, 128, 0);
    let encoded = encode_to_vec(&original).expect("encode Command::ChangeColor");
    let (decoded, _): (Command, usize) =
        decode_from_slice(&encoded).expect("decode Command::ChangeColor");
    assert_eq!(original, decoded);
}

// ---------------------------------------------------------------------------
// Test 19: Nested: enum containing another enum as a field
// ---------------------------------------------------------------------------

#[test]
fn test_nested_enum_as_field_roundtrip() {
    let cases = [
        Container::Empty,
        Container::Wrapped(Inner::Leaf),
        Container::Wrapped(Inner::Value(42)),
        Container::Tagged {
            tag: "important".to_string(),
            inner: Inner::Value(99),
        },
    ];
    for original in &cases {
        let encoded = encode_to_vec(original).expect("encode Container variant");
        let (decoded, consumed): (Container, usize) =
            decode_from_slice(&encoded).expect("decode Container variant");
        assert_eq!(original, &decoded);
        assert_eq!(consumed, encoded.len());
    }
}

// ---------------------------------------------------------------------------
// Test 20: Vec<Command> with all variants roundtrip
// ---------------------------------------------------------------------------

#[test]
fn test_vec_command_all_variants_roundtrip() {
    let original = vec![
        Command::Quit,
        Command::Move { x: 10, y: -3 },
        Command::Write("oxicode".to_string()),
        Command::ChangeColor(0, 255, 0),
        Command::Move { x: 0, y: 0 },
        Command::Write(String::new()),
    ];
    let encoded = encode_to_vec(&original).expect("encode Vec<Command>");
    let (decoded, consumed): (Vec<Command>, usize) =
        decode_from_slice(&encoded).expect("decode Vec<Command>");
    assert_eq!(original, decoded);
    assert_eq!(consumed, encoded.len());
}

// ---------------------------------------------------------------------------
// Test 21: Discriminant byte check: first variant has discriminant 0
// ---------------------------------------------------------------------------

#[test]
fn test_first_variant_has_discriminant_zero() {
    // For unit variants, the entire encoding is the discriminant (varint).
    // Discriminant 0 always encodes as a single byte 0x00.
    let quit_bytes = encode_to_vec(&Command::Quit).expect("encode Command::Quit");
    assert_eq!(
        quit_bytes.len(),
        1,
        "Command::Quit (first variant) must encode to exactly 1 byte"
    );
    assert_eq!(quit_bytes[0], 0u8, "Command::Quit discriminant must be 0");

    let start_bytes = encode_to_vec(&Event::Start).expect("encode Event::Start");
    assert_eq!(
        start_bytes.len(),
        1,
        "Event::Start (first variant) must encode to exactly 1 byte"
    );
    assert_eq!(start_bytes[0], 0u8, "Event::Start discriminant must be 0");

    let circle_bytes = encode_to_vec(&Shape::Circle { radius: 1.0 }).expect("encode Shape::Circle");
    assert_eq!(
        circle_bytes[0], 0u8,
        "Shape::Circle (first variant) discriminant must be 0"
    );
}

// ---------------------------------------------------------------------------
// Test 22: Status enum — all 3 variants roundtrip
// ---------------------------------------------------------------------------

#[test]
fn test_status_all_variants_roundtrip() {
    let cases = [
        Status::Ok,
        Status::Err("something went wrong".to_string()),
        Status::Pending {
            reason: "waiting for upstream".to_string(),
            retry_in: 30,
        },
    ];

    for original in &cases {
        let encoded = encode_to_vec(original).expect("encode Status variant");
        let (decoded, consumed): (Status, usize) =
            decode_from_slice(&encoded).expect("decode Status variant");
        assert_eq!(original, &decoded);
        assert_eq!(
            consumed,
            encoded.len(),
            "all bytes must be consumed for {:?}",
            original
        );
    }
}
