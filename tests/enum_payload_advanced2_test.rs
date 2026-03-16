//! Advanced enum payload tests for OxiCode — 22 scenarios.

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
enum Message {
    Text(String),
    Binary(Vec<u8>),
    Integer(i64),
    Float(f64),
    Empty,
}

#[derive(Debug, PartialEq, Encode, Decode)]
enum Command {
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(u8, u8, u8),
    Quit,
}

#[derive(Debug, PartialEq, Encode, Decode)]
enum Tree {
    Leaf(i32),
    Node { value: i32, count: u32 },
}

// ---------------------------------------------------------------------------
// 1. Message::Text roundtrip
// ---------------------------------------------------------------------------

#[test]
fn test_message_text_roundtrip() {
    let original = Message::Text(String::from("hello oxicode"));
    let encoded = encode_to_vec(&original).expect("encode Message::Text");
    let (decoded, _consumed): (Message, usize) =
        decode_from_slice(&encoded).expect("decode Message::Text");
    assert_eq!(original, decoded);
}

// ---------------------------------------------------------------------------
// 2. Message::Binary roundtrip
// ---------------------------------------------------------------------------

#[test]
fn test_message_binary_roundtrip() {
    let original = Message::Binary(vec![0xDE, 0xAD, 0xBE, 0xEF]);
    let encoded = encode_to_vec(&original).expect("encode Message::Binary");
    let (decoded, _consumed): (Message, usize) =
        decode_from_slice(&encoded).expect("decode Message::Binary");
    assert_eq!(original, decoded);
}

// ---------------------------------------------------------------------------
// 3. Message::Integer roundtrip
// ---------------------------------------------------------------------------

#[test]
fn test_message_integer_roundtrip() {
    let original = Message::Integer(-9_223_372_036_854_775_807_i64);
    let encoded = encode_to_vec(&original).expect("encode Message::Integer");
    let (decoded, _consumed): (Message, usize) =
        decode_from_slice(&encoded).expect("decode Message::Integer");
    assert_eq!(original, decoded);
}

// ---------------------------------------------------------------------------
// 4. Message::Float roundtrip (exact value 1.5)
// ---------------------------------------------------------------------------

#[test]
fn test_message_float_roundtrip() {
    let original = Message::Float(1.5_f64);
    let encoded = encode_to_vec(&original).expect("encode Message::Float");
    let (decoded, _consumed): (Message, usize) =
        decode_from_slice(&encoded).expect("decode Message::Float");
    assert_eq!(original, decoded);
}

// ---------------------------------------------------------------------------
// 5. Message::Empty roundtrip
// ---------------------------------------------------------------------------

#[test]
fn test_message_empty_roundtrip() {
    let original = Message::Empty;
    let encoded = encode_to_vec(&original).expect("encode Message::Empty");
    let (decoded, _consumed): (Message, usize) =
        decode_from_slice(&encoded).expect("decode Message::Empty");
    assert_eq!(original, decoded);
}

// ---------------------------------------------------------------------------
// 6. Command::Move roundtrip
// ---------------------------------------------------------------------------

#[test]
fn test_command_move_roundtrip() {
    let original = Command::Move { x: 42, y: -7 };
    let encoded = encode_to_vec(&original).expect("encode Command::Move");
    let (decoded, _consumed): (Command, usize) =
        decode_from_slice(&encoded).expect("decode Command::Move");
    assert_eq!(original, decoded);
}

// ---------------------------------------------------------------------------
// 7. Command::Write roundtrip
// ---------------------------------------------------------------------------

#[test]
fn test_command_write_roundtrip() {
    let original = Command::Write(String::from("write this payload"));
    let encoded = encode_to_vec(&original).expect("encode Command::Write");
    let (decoded, _consumed): (Command, usize) =
        decode_from_slice(&encoded).expect("decode Command::Write");
    assert_eq!(original, decoded);
}

// ---------------------------------------------------------------------------
// 8. Command::ChangeColor roundtrip
// ---------------------------------------------------------------------------

#[test]
fn test_command_change_color_roundtrip() {
    let original = Command::ChangeColor(128, 64, 32);
    let encoded = encode_to_vec(&original).expect("encode Command::ChangeColor");
    let (decoded, _consumed): (Command, usize) =
        decode_from_slice(&encoded).expect("decode Command::ChangeColor");
    assert_eq!(original, decoded);
}

// ---------------------------------------------------------------------------
// 9. Command::Quit roundtrip
// ---------------------------------------------------------------------------

#[test]
fn test_command_quit_roundtrip() {
    let original = Command::Quit;
    let encoded = encode_to_vec(&original).expect("encode Command::Quit");
    let (decoded, _consumed): (Command, usize) =
        decode_from_slice(&encoded).expect("decode Command::Quit");
    assert_eq!(original, decoded);
}

// ---------------------------------------------------------------------------
// 10. Tree::Leaf roundtrip
// ---------------------------------------------------------------------------

#[test]
fn test_tree_leaf_roundtrip() {
    let original = Tree::Leaf(-100);
    let encoded = encode_to_vec(&original).expect("encode Tree::Leaf");
    let (decoded, _consumed): (Tree, usize) =
        decode_from_slice(&encoded).expect("decode Tree::Leaf");
    assert_eq!(original, decoded);
}

// ---------------------------------------------------------------------------
// 11. Tree::Node roundtrip
// ---------------------------------------------------------------------------

#[test]
fn test_tree_node_roundtrip() {
    let original = Tree::Node {
        value: 42,
        count: 7,
    };
    let encoded = encode_to_vec(&original).expect("encode Tree::Node");
    let (decoded, _consumed): (Tree, usize) =
        decode_from_slice(&encoded).expect("decode Tree::Node");
    assert_eq!(original, decoded);
}

// ---------------------------------------------------------------------------
// 12. Vec<Message> with all variants roundtrip
// ---------------------------------------------------------------------------

#[test]
fn test_vec_message_all_variants_roundtrip() {
    let original: Vec<Message> = vec![
        Message::Text(String::from("alpha")),
        Message::Binary(vec![1, 2, 3]),
        Message::Integer(99),
        Message::Float(1.5_f64),
        Message::Empty,
    ];
    let encoded = encode_to_vec(&original).expect("encode Vec<Message>");
    let (decoded, _consumed): (Vec<Message>, usize) =
        decode_from_slice(&encoded).expect("decode Vec<Message>");
    assert_eq!(original, decoded);
}

// ---------------------------------------------------------------------------
// 13. Vec<Command> with all variants roundtrip
// ---------------------------------------------------------------------------

#[test]
fn test_vec_command_all_variants_roundtrip() {
    let original: Vec<Command> = vec![
        Command::Move { x: 0, y: 0 },
        Command::Write(String::from("cmd")),
        Command::ChangeColor(10, 20, 30),
        Command::Quit,
    ];
    let encoded = encode_to_vec(&original).expect("encode Vec<Command>");
    let (decoded, _consumed): (Vec<Command>, usize) =
        decode_from_slice(&encoded).expect("decode Vec<Command>");
    assert_eq!(original, decoded);
}

// ---------------------------------------------------------------------------
// 14. Option<Message> Some(Text) roundtrip
// ---------------------------------------------------------------------------

#[test]
fn test_option_message_some_text_roundtrip() {
    let original: Option<Message> = Some(Message::Text(String::from("optional text")));
    let encoded = encode_to_vec(&original).expect("encode Option<Message> Some");
    let (decoded, _consumed): (Option<Message>, usize) =
        decode_from_slice(&encoded).expect("decode Option<Message> Some");
    assert_eq!(original, decoded);
}

// ---------------------------------------------------------------------------
// 15. Option<Command> None roundtrip
// ---------------------------------------------------------------------------

#[test]
fn test_option_command_none_roundtrip() {
    let original: Option<Command> = None;
    let encoded = encode_to_vec(&original).expect("encode Option<Command> None");
    let (decoded, _consumed): (Option<Command>, usize) =
        decode_from_slice(&encoded).expect("decode Option<Command> None");
    assert_eq!(original, decoded);
}

// ---------------------------------------------------------------------------
// 16. Message::Text empty string roundtrip
// ---------------------------------------------------------------------------

#[test]
fn test_message_text_empty_string_roundtrip() {
    let original = Message::Text(String::new());
    let encoded = encode_to_vec(&original).expect("encode Message::Text empty");
    let (decoded, _consumed): (Message, usize) =
        decode_from_slice(&encoded).expect("decode Message::Text empty");
    assert_eq!(original, decoded);
    if let Message::Text(ref s) = decoded {
        assert!(s.is_empty(), "inner string must be empty");
    }
}

// ---------------------------------------------------------------------------
// 17. Message::Binary empty vec roundtrip
// ---------------------------------------------------------------------------

#[test]
fn test_message_binary_empty_vec_roundtrip() {
    let original = Message::Binary(Vec::new());
    let encoded = encode_to_vec(&original).expect("encode Message::Binary empty");
    let (decoded, _consumed): (Message, usize) =
        decode_from_slice(&encoded).expect("decode Message::Binary empty");
    assert_eq!(original, decoded);
    if let Message::Binary(ref b) = decoded {
        assert!(b.is_empty(), "inner vec must be empty");
    }
}

// ---------------------------------------------------------------------------
// 18. Command::ChangeColor max values (255, 255, 255) roundtrip
// ---------------------------------------------------------------------------

#[test]
fn test_command_change_color_max_values_roundtrip() {
    let original = Command::ChangeColor(255, 255, 255);
    let encoded = encode_to_vec(&original).expect("encode Command::ChangeColor max");
    let (decoded, _consumed): (Command, usize) =
        decode_from_slice(&encoded).expect("decode Command::ChangeColor max");
    assert_eq!(original, decoded);
}

// ---------------------------------------------------------------------------
// 19. Message consumed equals encoded length
// ---------------------------------------------------------------------------

#[test]
fn test_message_consumed_equals_encoded_length() {
    let original = Message::Text(String::from("consume me"));
    let encoded = encode_to_vec(&original).expect("encode for consumed check");
    let (_decoded, consumed): (Message, usize) =
        decode_from_slice(&encoded).expect("decode for consumed check");
    assert_eq!(
        consumed,
        encoded.len(),
        "consumed bytes must equal total encoded length"
    );
}

// ---------------------------------------------------------------------------
// 20. Command with fixed-int (legacy) config
// ---------------------------------------------------------------------------

#[test]
fn test_command_with_fixed_int_config() {
    let original = Command::Move { x: 10, y: -20 };
    let cfg = config::legacy();
    let encoded =
        encode_to_vec_with_config(&original, cfg).expect("encode Command::Move with legacy config");
    let (decoded, consumed): (Command, usize) = decode_from_slice_with_config(&encoded, cfg)
        .expect("decode Command::Move with legacy config");
    assert_eq!(original, decoded);
    assert_eq!(consumed, encoded.len());
}

// ---------------------------------------------------------------------------
// 21. Vec<Tree> mixed Leaf/Node roundtrip
// ---------------------------------------------------------------------------

#[test]
fn test_vec_tree_mixed_roundtrip() {
    let original: Vec<Tree> = vec![
        Tree::Leaf(1),
        Tree::Node { value: 2, count: 3 },
        Tree::Leaf(-5),
        Tree::Node {
            value: 100,
            count: 200,
        },
        Tree::Leaf(0),
    ];
    let encoded = encode_to_vec(&original).expect("encode Vec<Tree> mixed");
    let (decoded, _consumed): (Vec<Tree>, usize) =
        decode_from_slice(&encoded).expect("decode Vec<Tree> mixed");
    assert_eq!(original, decoded);
}

// ---------------------------------------------------------------------------
// 22. Different enum variants produce different encoded bytes
// ---------------------------------------------------------------------------

#[test]
fn test_different_variants_produce_different_bytes() {
    let text_bytes = encode_to_vec(&Message::Text(String::from("x"))).expect("encode Text variant");
    let binary_bytes = encode_to_vec(&Message::Binary(vec![0x78])).expect("encode Binary variant");
    let integer_bytes = encode_to_vec(&Message::Integer(0)).expect("encode Integer variant");
    let float_bytes = encode_to_vec(&Message::Float(0.0_f64)).expect("encode Float variant");
    let empty_bytes = encode_to_vec(&Message::Empty).expect("encode Empty variant");

    // Each variant must start with a distinct discriminant byte.
    assert_ne!(
        text_bytes[0], binary_bytes[0],
        "Text and Binary must have different discriminants"
    );
    assert_ne!(
        text_bytes[0], integer_bytes[0],
        "Text and Integer must have different discriminants"
    );
    assert_ne!(
        text_bytes[0], float_bytes[0],
        "Text and Float must have different discriminants"
    );
    assert_ne!(
        text_bytes[0], empty_bytes[0],
        "Text and Empty must have different discriminants"
    );
    assert_ne!(
        binary_bytes[0], integer_bytes[0],
        "Binary and Integer must have different discriminants"
    );
    assert_ne!(
        binary_bytes[0], float_bytes[0],
        "Binary and Float must have different discriminants"
    );
    assert_ne!(
        binary_bytes[0], empty_bytes[0],
        "Binary and Empty must have different discriminants"
    );
    assert_ne!(
        integer_bytes[0], float_bytes[0],
        "Integer and Float must have different discriminants"
    );
    assert_ne!(
        integer_bytes[0], empty_bytes[0],
        "Integer and Empty must have different discriminants"
    );
    assert_ne!(
        float_bytes[0], empty_bytes[0],
        "Float and Empty must have different discriminants"
    );
}
