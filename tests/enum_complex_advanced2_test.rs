//! Advanced complex enum encoding tests for OxiCode

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

#[derive(Debug, PartialEq, Encode, Decode)]
enum Color {
    Red,
    Green,
    Blue,
    Custom(u8, u8, u8),
}

#[derive(Debug, PartialEq, Encode, Decode)]
enum Message {
    Ping,
    Pong,
    Text(String),
    Binary(Vec<u8>),
    Close { code: u16, reason: String },
    Auth { token: String, expires: u64 },
}

#[derive(Debug, PartialEq, Encode, Decode)]
struct Packet {
    id: u32,
    message: Message,
    timestamp: u64,
}

#[derive(Debug, PartialEq, Encode, Decode)]
enum Tree {
    Leaf(i32),
    Node {
        value: i32,
        left: Box<Tree>,
        right: Box<Tree>,
    },
}

// Test 1: Color::Red roundtrip (discriminant 0)
#[test]
fn test_color_red_roundtrip() {
    let color = Color::Red;
    let encoded = encode_to_vec(&color).expect("Failed to encode Color::Red");
    let (decoded, _): (Color, usize) =
        decode_from_slice(&encoded).expect("Failed to decode Color::Red");
    assert_eq!(color, decoded);
}

// Test 2: Color::Green roundtrip (discriminant 1)
#[test]
fn test_color_green_roundtrip() {
    let color = Color::Green;
    let encoded = encode_to_vec(&color).expect("Failed to encode Color::Green");
    let (decoded, _): (Color, usize) =
        decode_from_slice(&encoded).expect("Failed to decode Color::Green");
    assert_eq!(color, decoded);
}

// Test 3: Color::Blue roundtrip (discriminant 2)
#[test]
fn test_color_blue_roundtrip() {
    let color = Color::Blue;
    let encoded = encode_to_vec(&color).expect("Failed to encode Color::Blue");
    let (decoded, _): (Color, usize) =
        decode_from_slice(&encoded).expect("Failed to decode Color::Blue");
    assert_eq!(color, decoded);
}

// Test 4: Color::Custom(255, 128, 0) roundtrip
#[test]
fn test_color_custom_roundtrip() {
    let color = Color::Custom(255, 128, 0);
    let encoded = encode_to_vec(&color).expect("Failed to encode Color::Custom");
    let (decoded, _): (Color, usize) =
        decode_from_slice(&encoded).expect("Failed to decode Color::Custom");
    assert_eq!(color, decoded);
}

// Test 5: Color unit variants encode to single byte discriminant
#[test]
fn test_color_unit_variants_single_byte_discriminant() {
    let red = Color::Red;
    let green = Color::Green;
    let blue = Color::Blue;

    let encoded_red = encode_to_vec(&red).expect("Failed to encode Color::Red");
    let encoded_green = encode_to_vec(&green).expect("Failed to encode Color::Green");
    let encoded_blue = encode_to_vec(&blue).expect("Failed to encode Color::Blue");

    assert_eq!(encoded_red.len(), 1, "Color::Red should encode to 1 byte");
    assert_eq!(
        encoded_green.len(),
        1,
        "Color::Green should encode to 1 byte"
    );
    assert_eq!(encoded_blue.len(), 1, "Color::Blue should encode to 1 byte");

    assert_ne!(encoded_red, encoded_green, "Red and Green must differ");
    assert_ne!(encoded_green, encoded_blue, "Green and Blue must differ");
    assert_ne!(encoded_red, encoded_blue, "Red and Blue must differ");
}

// Test 6: Message::Ping roundtrip
#[test]
fn test_message_ping_roundtrip() {
    let msg = Message::Ping;
    let encoded = encode_to_vec(&msg).expect("Failed to encode Message::Ping");
    let (decoded, _): (Message, usize) =
        decode_from_slice(&encoded).expect("Failed to decode Message::Ping");
    assert_eq!(msg, decoded);
}

// Test 7: Message::Pong roundtrip
#[test]
fn test_message_pong_roundtrip() {
    let msg = Message::Pong;
    let encoded = encode_to_vec(&msg).expect("Failed to encode Message::Pong");
    let (decoded, _): (Message, usize) =
        decode_from_slice(&encoded).expect("Failed to decode Message::Pong");
    assert_eq!(msg, decoded);
}

// Test 8: Message::Text roundtrip
#[test]
fn test_message_text_roundtrip() {
    let msg = Message::Text(String::from("Hello, OxiCode!"));
    let encoded = encode_to_vec(&msg).expect("Failed to encode Message::Text");
    let (decoded, _): (Message, usize) =
        decode_from_slice(&encoded).expect("Failed to decode Message::Text");
    assert_eq!(msg, decoded);
}

// Test 9: Message::Binary roundtrip
#[test]
fn test_message_binary_roundtrip() {
    let msg = Message::Binary(vec![0xDE, 0xAD, 0xBE, 0xEF, 0x00, 0xFF]);
    let encoded = encode_to_vec(&msg).expect("Failed to encode Message::Binary");
    let (decoded, _): (Message, usize) =
        decode_from_slice(&encoded).expect("Failed to decode Message::Binary");
    assert_eq!(msg, decoded);
}

// Test 10: Message::Close roundtrip
#[test]
fn test_message_close_roundtrip() {
    let msg = Message::Close {
        code: 1001,
        reason: String::from("Going Away"),
    };
    let encoded = encode_to_vec(&msg).expect("Failed to encode Message::Close");
    let (decoded, _): (Message, usize) =
        decode_from_slice(&encoded).expect("Failed to decode Message::Close");
    assert_eq!(msg, decoded);
}

// Test 11: Message::Auth roundtrip
#[test]
fn test_message_auth_roundtrip() {
    let msg = Message::Auth {
        token: String::from("bearer_eyJhbGciOiJIUzI1NiJ9"),
        expires: 1_700_000_000u64,
    };
    let encoded = encode_to_vec(&msg).expect("Failed to encode Message::Auth");
    let (decoded, _): (Message, usize) =
        decode_from_slice(&encoded).expect("Failed to decode Message::Auth");
    assert_eq!(msg, decoded);
}

// Test 12: Packet with Message::Text roundtrip
#[test]
fn test_packet_with_text_roundtrip() {
    let packet = Packet {
        id: 42,
        message: Message::Text(String::from("payload data")),
        timestamp: 1_700_000_100u64,
    };
    let encoded = encode_to_vec(&packet).expect("Failed to encode Packet with Text");
    let (decoded, _): (Packet, usize) =
        decode_from_slice(&encoded).expect("Failed to decode Packet with Text");
    assert_eq!(packet, decoded);
}

// Test 13: Packet with Message::Binary roundtrip
#[test]
fn test_packet_with_binary_roundtrip() {
    let packet = Packet {
        id: 99,
        message: Message::Binary(vec![1, 2, 3, 4, 5, 6, 7, 8]),
        timestamp: 1_700_000_200u64,
    };
    let encoded = encode_to_vec(&packet).expect("Failed to encode Packet with Binary");
    let (decoded, _): (Packet, usize) =
        decode_from_slice(&encoded).expect("Failed to decode Packet with Binary");
    assert_eq!(packet, decoded);
}

// Test 14: Vec<Message> all variants roundtrip
#[test]
fn test_vec_message_all_variants_roundtrip() {
    let messages = vec![
        Message::Ping,
        Message::Pong,
        Message::Text(String::from("hello")),
        Message::Binary(vec![10, 20, 30]),
        Message::Close {
            code: 1000,
            reason: String::from("Normal Closure"),
        },
        Message::Auth {
            token: String::from("token123"),
            expires: 9_999_999u64,
        },
    ];
    let encoded = encode_to_vec(&messages).expect("Failed to encode Vec<Message>");
    let (decoded, _): (Vec<Message>, usize) =
        decode_from_slice(&encoded).expect("Failed to decode Vec<Message>");
    assert_eq!(messages, decoded);
}

// Test 15: Tree::Leaf roundtrip
#[test]
fn test_tree_leaf_roundtrip() {
    let tree = Tree::Leaf(42);
    let encoded = encode_to_vec(&tree).expect("Failed to encode Tree::Leaf");
    let (decoded, _): (Tree, usize) =
        decode_from_slice(&encoded).expect("Failed to decode Tree::Leaf");
    assert_eq!(tree, decoded);
}

// Test 16: Tree::Node with two leaves roundtrip
#[test]
fn test_tree_node_with_leaves_roundtrip() {
    let tree = Tree::Node {
        value: 1,
        left: Box::new(Tree::Leaf(2)),
        right: Box::new(Tree::Leaf(3)),
    };
    let encoded = encode_to_vec(&tree).expect("Failed to encode Tree::Node with leaves");
    let (decoded, _): (Tree, usize) =
        decode_from_slice(&encoded).expect("Failed to decode Tree::Node with leaves");
    assert_eq!(tree, decoded);
}

// Test 17: Tree::Node deeply nested roundtrip (depth 3)
#[test]
fn test_tree_node_deeply_nested_roundtrip() {
    let tree = Tree::Node {
        value: 10,
        left: Box::new(Tree::Node {
            value: 5,
            left: Box::new(Tree::Leaf(2)),
            right: Box::new(Tree::Leaf(7)),
        }),
        right: Box::new(Tree::Node {
            value: 15,
            left: Box::new(Tree::Leaf(12)),
            right: Box::new(Tree::Leaf(20)),
        }),
    };
    let encoded = encode_to_vec(&tree).expect("Failed to encode deeply nested Tree");
    let (decoded, _): (Tree, usize) =
        decode_from_slice(&encoded).expect("Failed to decode deeply nested Tree");
    assert_eq!(tree, decoded);
}

// Test 18: Option<Message> Some::Text roundtrip
#[test]
fn test_option_message_some_text_roundtrip() {
    let opt: Option<Message> = Some(Message::Text(String::from("optional text")));
    let encoded = encode_to_vec(&opt).expect("Failed to encode Option<Message> Some");
    let (decoded, _): (Option<Message>, usize) =
        decode_from_slice(&encoded).expect("Failed to decode Option<Message> Some");
    assert_eq!(opt, decoded);
}

// Test 19: Option<Message> None roundtrip
#[test]
fn test_option_message_none_roundtrip() {
    let opt: Option<Message> = None;
    let encoded = encode_to_vec(&opt).expect("Failed to encode Option<Message> None");
    let (decoded, _): (Option<Message>, usize) =
        decode_from_slice(&encoded).expect("Failed to decode Option<Message> None");
    assert_eq!(opt, decoded);
}

// Test 20: Message::Ping and Message::Pong encode differently
#[test]
fn test_message_ping_pong_encode_differently() {
    let ping = Message::Ping;
    let pong = Message::Pong;

    let encoded_ping = encode_to_vec(&ping).expect("Failed to encode Message::Ping");
    let encoded_pong = encode_to_vec(&pong).expect("Failed to encode Message::Pong");

    assert_ne!(
        encoded_ping, encoded_pong,
        "Message::Ping and Message::Pong must encode to different bytes"
    );
}

// Test 21: Vec<Color> all four variants roundtrip
#[test]
fn test_vec_color_all_variants_roundtrip() {
    let colors = vec![
        Color::Red,
        Color::Green,
        Color::Blue,
        Color::Custom(100, 150, 200),
    ];
    let encoded = encode_to_vec(&colors).expect("Failed to encode Vec<Color>");
    let (decoded, _): (Vec<Color>, usize) =
        decode_from_slice(&encoded).expect("Failed to decode Vec<Color>");
    assert_eq!(colors, decoded);
}

// Test 22: Color fixed int config roundtrip
#[test]
fn test_color_fixed_int_config_roundtrip() {
    let cfg = config::standard().with_fixed_int_encoding();
    let color = Color::Custom(64, 128, 192);
    let encoded = encode_to_vec_with_config(&color, cfg)
        .expect("Failed to encode Color with fixed int config");
    let (decoded, _): (Color, usize) = decode_from_slice_with_config(&encoded, cfg)
        .expect("Failed to decode Color with fixed int config");
    assert_eq!(color, decoded);
}
