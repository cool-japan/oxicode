//! Advanced complex enum encoding tests — MessageType / Message / Protocol

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
// Type definitions
// ---------------------------------------------------------------------------

#[derive(Debug, PartialEq, Encode, Decode)]
enum MessageType {
    Text(String),
    Binary(Vec<u8>),
    Ping,
    Pong,
    Close {
        code: u16,
        reason: String,
    },
    Fragment {
        index: u32,
        total: u32,
        data: Vec<u8>,
    },
    Control {
        command: String,
        params: Vec<String>,
    },
    Error {
        code: u32,
        message: String,
        details: Option<String>,
    },
}

#[derive(Debug, PartialEq, Encode, Decode)]
struct Message {
    id: u64,
    msg_type: MessageType,
    timestamp_ms: u64,
    ttl_ms: Option<u32>,
}

#[derive(Debug, PartialEq, Encode, Decode)]
enum Protocol {
    Http { version: u8 },
    WebSocket,
    Grpc { service: String },
    Custom(Vec<u8>),
}

// ---------------------------------------------------------------------------
// Tests
// ---------------------------------------------------------------------------

#[test]
fn test_message_type_text_roundtrip() {
    let val = MessageType::Text(String::from("hello world"));
    let bytes = encode_to_vec(&val).expect("encode MessageType::Text");
    let (decoded, _): (MessageType, usize) =
        decode_from_slice(&bytes).expect("decode MessageType::Text");
    assert_eq!(val, decoded);
}

#[test]
fn test_message_type_binary_roundtrip() {
    let val = MessageType::Binary(vec![0x00, 0xFF, 0x42, 0xAB]);
    let bytes = encode_to_vec(&val).expect("encode MessageType::Binary");
    let (decoded, _): (MessageType, usize) =
        decode_from_slice(&bytes).expect("decode MessageType::Binary");
    assert_eq!(val, decoded);
}

#[test]
fn test_message_type_ping_roundtrip() {
    let val = MessageType::Ping;
    let bytes = encode_to_vec(&val).expect("encode MessageType::Ping");
    let (decoded, _): (MessageType, usize) =
        decode_from_slice(&bytes).expect("decode MessageType::Ping");
    assert_eq!(val, decoded);
}

#[test]
fn test_message_type_pong_roundtrip() {
    let val = MessageType::Pong;
    let bytes = encode_to_vec(&val).expect("encode MessageType::Pong");
    let (decoded, _): (MessageType, usize) =
        decode_from_slice(&bytes).expect("decode MessageType::Pong");
    assert_eq!(val, decoded);
}

#[test]
fn test_message_type_close_roundtrip() {
    let val = MessageType::Close {
        code: 1001,
        reason: String::from("going away"),
    };
    let bytes = encode_to_vec(&val).expect("encode MessageType::Close");
    let (decoded, _): (MessageType, usize) =
        decode_from_slice(&bytes).expect("decode MessageType::Close");
    assert_eq!(val, decoded);
}

#[test]
fn test_message_type_fragment_roundtrip() {
    let val = MessageType::Fragment {
        index: 3,
        total: 10,
        data: vec![0x01, 0x02, 0x03],
    };
    let bytes = encode_to_vec(&val).expect("encode MessageType::Fragment");
    let (decoded, _): (MessageType, usize) =
        decode_from_slice(&bytes).expect("decode MessageType::Fragment");
    assert_eq!(val, decoded);
}

#[test]
fn test_message_type_control_roundtrip() {
    let val = MessageType::Control {
        command: String::from("shutdown"),
        params: vec![String::from("--force"), String::from("--timeout=30")],
    };
    let bytes = encode_to_vec(&val).expect("encode MessageType::Control");
    let (decoded, _): (MessageType, usize) =
        decode_from_slice(&bytes).expect("decode MessageType::Control");
    assert_eq!(val, decoded);
}

#[test]
fn test_message_type_error_with_details_roundtrip() {
    let val = MessageType::Error {
        code: 500,
        message: String::from("internal error"),
        details: Some(String::from("stack trace here")),
    };
    let bytes = encode_to_vec(&val).expect("encode MessageType::Error with details");
    let (decoded, _): (MessageType, usize) =
        decode_from_slice(&bytes).expect("decode MessageType::Error with details");
    assert_eq!(val, decoded);
}

#[test]
fn test_message_type_error_without_details_roundtrip() {
    let val = MessageType::Error {
        code: 404,
        message: String::from("not found"),
        details: None,
    };
    let bytes = encode_to_vec(&val).expect("encode MessageType::Error no details");
    let (decoded, _): (MessageType, usize) =
        decode_from_slice(&bytes).expect("decode MessageType::Error no details");
    assert_eq!(val, decoded);
}

#[test]
fn test_message_struct_with_text_and_ttl() {
    let val = Message {
        id: 42,
        msg_type: MessageType::Text(String::from("greetings")),
        timestamp_ms: 1_700_000_000_000,
        ttl_ms: Some(5000),
    };
    let bytes = encode_to_vec(&val).expect("encode Message with ttl");
    let (decoded, _): (Message, usize) =
        decode_from_slice(&bytes).expect("decode Message with ttl");
    assert_eq!(val, decoded);
}

#[test]
fn test_message_struct_no_ttl() {
    let val = Message {
        id: 99,
        msg_type: MessageType::Ping,
        timestamp_ms: 1_000_000,
        ttl_ms: None,
    };
    let bytes = encode_to_vec(&val).expect("encode Message no ttl");
    let (decoded, _): (Message, usize) = decode_from_slice(&bytes).expect("decode Message no ttl");
    assert_eq!(val, decoded);
}

#[test]
fn test_message_struct_with_binary_type() {
    let val = Message {
        id: 7,
        msg_type: MessageType::Binary(vec![0xDE, 0xAD, 0xBE, 0xEF]),
        timestamp_ms: 9_999_999,
        ttl_ms: Some(1000),
    };
    let bytes = encode_to_vec(&val).expect("encode Message binary");
    let (decoded, _): (Message, usize) = decode_from_slice(&bytes).expect("decode Message binary");
    assert_eq!(val, decoded);
}

#[test]
fn test_message_struct_with_fragment_type() {
    let val = Message {
        id: 1024,
        msg_type: MessageType::Fragment {
            index: 0,
            total: 5,
            data: vec![0xAA, 0xBB],
        },
        timestamp_ms: 2_000_000_000,
        ttl_ms: None,
    };
    let bytes = encode_to_vec(&val).expect("encode Message fragment");
    let (decoded, _): (Message, usize) =
        decode_from_slice(&bytes).expect("decode Message fragment");
    assert_eq!(val, decoded);
}

#[test]
fn test_protocol_http_roundtrip() {
    let val = Protocol::Http { version: 2 };
    let bytes = encode_to_vec(&val).expect("encode Protocol::Http");
    let (decoded, _): (Protocol, usize) = decode_from_slice(&bytes).expect("decode Protocol::Http");
    assert_eq!(val, decoded);
}

#[test]
fn test_protocol_websocket_roundtrip() {
    let val = Protocol::WebSocket;
    let bytes = encode_to_vec(&val).expect("encode Protocol::WebSocket");
    let (decoded, _): (Protocol, usize) =
        decode_from_slice(&bytes).expect("decode Protocol::WebSocket");
    assert_eq!(val, decoded);
}

#[test]
fn test_protocol_grpc_roundtrip() {
    let val = Protocol::Grpc {
        service: String::from("com.example.UserService"),
    };
    let bytes = encode_to_vec(&val).expect("encode Protocol::Grpc");
    let (decoded, _): (Protocol, usize) = decode_from_slice(&bytes).expect("decode Protocol::Grpc");
    assert_eq!(val, decoded);
}

#[test]
fn test_protocol_custom_roundtrip() {
    let val = Protocol::Custom(vec![0x01, 0x02, 0x03, 0x04, 0x05]);
    let bytes = encode_to_vec(&val).expect("encode Protocol::Custom");
    let (decoded, _): (Protocol, usize) =
        decode_from_slice(&bytes).expect("decode Protocol::Custom");
    assert_eq!(val, decoded);
}

#[test]
fn test_discriminant_ping_differs_from_pong() {
    let ping_bytes = encode_to_vec(&MessageType::Ping).expect("encode Ping");
    let pong_bytes = encode_to_vec(&MessageType::Pong).expect("encode Pong");
    assert_ne!(
        ping_bytes, pong_bytes,
        "Ping and Pong must have distinct discriminants"
    );
}

#[test]
fn test_discriminant_websocket_differs_from_http() {
    let ws_bytes = encode_to_vec(&Protocol::WebSocket).expect("encode WebSocket");
    let http_bytes = encode_to_vec(&Protocol::Http { version: 1 }).expect("encode Http v1");
    assert_ne!(
        ws_bytes, http_bytes,
        "WebSocket and Http must have distinct discriminants"
    );
}

#[test]
fn test_consumed_bytes_nonempty_text() {
    let val = MessageType::Text(String::from("abc"));
    let bytes = encode_to_vec(&val).expect("encode Text abc");
    let (_, consumed): (MessageType, usize) = decode_from_slice(&bytes).expect("decode Text abc");
    assert!(consumed > 0, "consumed bytes must be positive");
    assert_eq!(
        consumed,
        bytes.len(),
        "all encoded bytes should be consumed"
    );
}

#[test]
fn test_config_big_endian_message_roundtrip() {
    let val = Message {
        id: 12345,
        msg_type: MessageType::Close {
            code: 1000,
            reason: String::from("normal closure"),
        },
        timestamp_ms: 1_700_123_456_789,
        ttl_ms: Some(60_000),
    };
    let cfg = config::standard().with_big_endian();
    let bytes = encode_to_vec_with_config(&val, cfg).expect("encode Message big_endian");
    let (decoded, _): (Message, usize) =
        decode_from_slice_with_config(&bytes, cfg).expect("decode Message big_endian");
    assert_eq!(val, decoded);
}

#[test]
fn test_config_fixed_int_protocol_roundtrip() {
    let val = Protocol::Http { version: 1 };
    let cfg = config::standard().with_fixed_int_encoding();
    let bytes = encode_to_vec_with_config(&val, cfg).expect("encode Protocol fixed_int");
    let (decoded, _): (Protocol, usize) =
        decode_from_slice_with_config(&bytes, cfg).expect("decode Protocol fixed_int");
    assert_eq!(val, decoded);
}

#[test]
fn test_vec_of_messages_roundtrip() {
    let messages = vec![
        Message {
            id: 1,
            msg_type: MessageType::Ping,
            timestamp_ms: 100,
            ttl_ms: None,
        },
        Message {
            id: 2,
            msg_type: MessageType::Text(String::from("hello")),
            timestamp_ms: 200,
            ttl_ms: Some(300),
        },
        Message {
            id: 3,
            msg_type: MessageType::Error {
                code: 503,
                message: String::from("unavailable"),
                details: None,
            },
            timestamp_ms: 300,
            ttl_ms: None,
        },
    ];
    let bytes = encode_to_vec(&messages).expect("encode Vec<Message>");
    let (decoded, consumed): (Vec<Message>, usize) =
        decode_from_slice(&bytes).expect("decode Vec<Message>");
    assert_eq!(messages, decoded);
    assert_eq!(consumed, bytes.len(), "all bytes consumed for Vec<Message>");
}
