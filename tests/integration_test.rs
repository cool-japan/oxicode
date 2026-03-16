//! End-to-end integration test simulating a real binary network protocol.
//!
//! These tests cover full round-trip serialisation/deserialisation using the
//! public oxicode API, with realistic domain types.  They complement the more
//! focused unit tests in the other test files by exercising the library as a
//! whole.

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
use oxicode::{config, decode_from_slice, encode_to_vec, Decode, Encode};

// ---------------------------------------------------------------------------
// Domain types
// ---------------------------------------------------------------------------

/// A fixed-header packet suitable for framing over a byte stream.
#[derive(Debug, PartialEq, Encode, Decode)]
struct Packet {
    version: u8,
    sequence: u32,
    payload: Vec<u8>,
    checksum: u32,
}

/// Application-layer commands sent over the protocol.
#[derive(Debug, PartialEq, Encode, Decode)]
#[oxicode(tag_type = "u8")]
enum Command {
    Ping,
    Pong,
    Data { id: u32, data: Vec<u8> },
    Error { code: u16, message: String },
}

/// A session-level message that wraps one or more commands.
#[derive(Debug, PartialEq, Encode, Decode)]
struct Session {
    session_id: u64,
    commands: Vec<Command>,
}

/// Nested address type used to exercise recursive struct encoding.
#[derive(Debug, PartialEq, Encode, Decode)]
struct Address {
    host: String,
    port: u16,
}

/// Connection metadata with nested types, options, and collections.
#[derive(Debug, PartialEq, Encode, Decode)]
struct ConnectionMeta {
    remote: Address,
    local: Address,
    tags: Vec<String>,
    timeout_ms: Option<u32>,
    tls: bool,
}

// ---------------------------------------------------------------------------
// Packet tests
// ---------------------------------------------------------------------------

#[test]
fn test_packet_roundtrip() {
    let pkt = Packet {
        version: 2,
        sequence: 42,
        payload: vec![0xde, 0xad, 0xbe, 0xef],
        checksum: 0x1234_5678,
    };

    let bytes = encode_to_vec(&pkt).expect("encode Packet");
    let (decoded, bytes_read): (Packet, _) = decode_from_slice(&bytes).expect("decode Packet");

    assert_eq!(pkt, decoded);
    assert_eq!(bytes_read, bytes.len(), "all bytes must be consumed");
}

#[test]
fn test_packet_empty_payload() {
    let pkt = Packet {
        version: 1,
        sequence: 0,
        payload: vec![],
        checksum: 0,
    };

    let bytes = encode_to_vec(&pkt).expect("encode");
    let (decoded, _): (Packet, _) = decode_from_slice(&bytes).expect("decode");
    assert_eq!(pkt, decoded);
}

#[test]
fn test_packet_large_payload() {
    let pkt = Packet {
        version: 3,
        sequence: u32::MAX,
        payload: (0u8..=255).cycle().take(4096).collect(),
        checksum: 0xffff_ffff,
    };

    let bytes = encode_to_vec(&pkt).expect("encode large packet");
    let (decoded, _): (Packet, _) = decode_from_slice(&bytes).expect("decode large packet");
    assert_eq!(pkt, decoded);
}

// ---------------------------------------------------------------------------
// Command roundtrip — all variants
// ---------------------------------------------------------------------------

#[test]
fn test_command_roundtrip_all_variants() {
    let variants: &[Command] = &[
        Command::Ping,
        Command::Pong,
        Command::Data {
            id: 1,
            data: vec![1, 2, 3, 4, 5],
        },
        Command::Data {
            id: u32::MAX,
            data: vec![],
        },
        Command::Error {
            code: 404,
            message: "Not found".into(),
        },
        Command::Error {
            code: 500,
            message: "Internal server error".into(),
        },
        Command::Error {
            code: 0,
            message: String::new(),
        },
    ];

    for cmd in variants {
        let bytes = encode_to_vec(cmd).expect("encode Command");
        let (decoded, consumed): (Command, _) = decode_from_slice(&bytes).expect("decode Command");
        assert_eq!(cmd, &decoded, "Command variant mismatch: {:?}", cmd);
        assert_eq!(consumed, bytes.len(), "unconsumed bytes for {:?}", cmd);
    }
}

// ---------------------------------------------------------------------------
// Protocol simulation
// ---------------------------------------------------------------------------

#[test]
fn test_protocol_simulation() {
    // Simulate sending a series of commands over a byte channel.
    let commands = vec![
        Command::Ping,
        Command::Data {
            id: 1,
            data: vec![1, 2, 3],
        },
        Command::Error {
            code: 404,
            message: "Not found".into(),
        },
        Command::Pong,
        Command::Data {
            id: 100,
            data: (0u8..128).collect(),
        },
    ];

    for cmd in &commands {
        let bytes = encode_to_vec(cmd).expect("encode");
        let (decoded, _): (Command, _) = decode_from_slice(&bytes).expect("decode");
        assert_eq!(cmd, &decoded);
    }
}

/// Simulate encoding several commands into a single contiguous buffer and
/// reading them back one-by-one using the consumed-bytes cursor.
#[test]
fn test_protocol_framing_stream() {
    let outbound = vec![
        Command::Ping,
        Command::Data {
            id: 7,
            data: vec![0xff, 0x00],
        },
        Command::Pong,
        Command::Error {
            code: 503,
            message: "service unavailable".into(),
        },
    ];

    // Encode all commands into one buffer
    let mut buffer: Vec<u8> = Vec::new();
    for cmd in &outbound {
        let frame = encode_to_vec(cmd).expect("encode frame");
        buffer.extend_from_slice(&frame);
    }

    // Decode them back sequentially
    let mut cursor = 0usize;
    let mut decoded_commands: Vec<Command> = Vec::new();
    while cursor < buffer.len() {
        let (cmd, n): (Command, _) = decode_from_slice(&buffer[cursor..]).expect("decode frame");
        cursor += n;
        decoded_commands.push(cmd);
    }

    assert_eq!(outbound, decoded_commands);
}

// ---------------------------------------------------------------------------
// encoded_size consistency
// ---------------------------------------------------------------------------

#[test]
fn test_encoded_size_consistency() {
    let commands: &[Command] = &[
        Command::Ping,
        Command::Pong,
        Command::Data {
            id: 42,
            data: vec![1, 2, 3],
        },
        Command::Error {
            code: 400,
            message: "bad request".into(),
        },
    ];

    for cmd in commands {
        let size = oxicode::encoded_size(cmd).expect("encoded_size");
        let bytes = encode_to_vec(cmd).expect("encode");
        assert_eq!(
            size,
            bytes.len(),
            "encoded_size mismatch for {:?}: predicted={}, actual={}",
            cmd,
            size,
            bytes.len()
        );
    }

    // Also verify for Packet
    let pkt = Packet {
        version: 1,
        sequence: 99,
        payload: vec![42, 43, 44],
        checksum: 0xdead,
    };
    let size = oxicode::encoded_size(&pkt).expect("encoded_size Packet");
    let bytes = encode_to_vec(&pkt).expect("encode Packet");
    assert_eq!(size, bytes.len());
}

// ---------------------------------------------------------------------------
// Session (nested Vec<Command>)
// ---------------------------------------------------------------------------

#[test]
fn test_session_roundtrip() {
    let session = Session {
        session_id: 0x0102_0304_0506_0708,
        commands: vec![
            Command::Ping,
            Command::Data {
                id: 1,
                data: vec![10, 20, 30],
            },
            Command::Pong,
        ],
    };

    let bytes = encode_to_vec(&session).expect("encode Session");
    let (decoded, _): (Session, _) = decode_from_slice(&bytes).expect("decode Session");
    assert_eq!(session, decoded);
}

#[test]
fn test_empty_session_roundtrip() {
    let session = Session {
        session_id: 0,
        commands: vec![],
    };

    let bytes = encode_to_vec(&session).expect("encode empty Session");
    let (decoded, _): (Session, _) = decode_from_slice(&bytes).expect("decode empty Session");
    assert_eq!(session, decoded);
}

// ---------------------------------------------------------------------------
// ConnectionMeta — nested types, Option, Vec<String>
// ---------------------------------------------------------------------------

#[test]
fn test_connection_meta_roundtrip() {
    let meta = ConnectionMeta {
        remote: Address {
            host: "192.168.1.1".into(),
            port: 8080,
        },
        local: Address {
            host: "127.0.0.1".into(),
            port: 0,
        },
        tags: vec!["production".into(), "region:eu-west".into()],
        timeout_ms: Some(5000),
        tls: true,
    };

    let bytes = encode_to_vec(&meta).expect("encode ConnectionMeta");
    let (decoded, _): (ConnectionMeta, _) =
        decode_from_slice(&bytes).expect("decode ConnectionMeta");
    assert_eq!(meta, decoded);
}

#[test]
fn test_connection_meta_no_timeout() {
    let meta = ConnectionMeta {
        remote: Address {
            host: "10.0.0.1".into(),
            port: 443,
        },
        local: Address {
            host: "0.0.0.0".into(),
            port: 54321,
        },
        tags: vec![],
        timeout_ms: None,
        tls: false,
    };

    let bytes = encode_to_vec(&meta).expect("encode ConnectionMeta no timeout");
    let (decoded, _): (ConnectionMeta, _) =
        decode_from_slice(&bytes).expect("decode ConnectionMeta no timeout");
    assert_eq!(meta, decoded);
}

// ---------------------------------------------------------------------------
// Configuration variants (standard vs legacy)
// ---------------------------------------------------------------------------

#[test]
fn test_packet_standard_and_legacy_configs() {
    let pkt = Packet {
        version: 1,
        sequence: 1000,
        payload: vec![0xaa, 0xbb],
        checksum: 0x1234,
    };

    // Standard config (varint)
    let std_bytes =
        oxicode::encode_to_vec_with_config(&pkt, config::standard()).expect("encode standard");
    let (std_decoded, _): (Packet, _) =
        oxicode::decode_from_slice_with_config(&std_bytes, config::standard())
            .expect("decode standard");
    assert_eq!(pkt, std_decoded);

    // Legacy config (fixed-int, bincode 1.x compatible)
    let leg_bytes =
        oxicode::encode_to_vec_with_config(&pkt, config::legacy()).expect("encode legacy");
    let (leg_decoded, _): (Packet, _) =
        oxicode::decode_from_slice_with_config(&leg_bytes, config::legacy())
            .expect("decode legacy");
    assert_eq!(pkt, leg_decoded);

    // The two configs must produce different byte sequences (layout differs)
    assert_ne!(
        std_bytes, leg_bytes,
        "standard and legacy configs must produce different byte layouts"
    );
}

// ---------------------------------------------------------------------------
// Error cases — truncated / corrupt data
// ---------------------------------------------------------------------------

#[test]
fn test_decode_truncated_data_returns_error() {
    let cmd = Command::Data {
        id: 999,
        data: vec![1, 2, 3, 4, 5, 6, 7, 8],
    };
    let bytes = encode_to_vec(&cmd).expect("encode");

    // Attempt to decode from a truncated buffer — must return an Err, not panic.
    let truncated = &bytes[..bytes.len() / 2];
    let result: Result<(Command, _), _> = decode_from_slice(truncated);
    assert!(
        result.is_err(),
        "decoding truncated data should return an error"
    );
}

#[test]
fn test_decode_empty_slice_returns_error() {
    let result: Result<(Packet, _), _> = decode_from_slice(&[]);
    assert!(
        result.is_err(),
        "decoding empty slice should return an error"
    );
}

// ---------------------------------------------------------------------------
// Idempotency — re-encoding decoded values yields identical bytes
// ---------------------------------------------------------------------------

#[test]
fn test_encode_decode_encode_idempotency() {
    let original = Session {
        session_id: 42,
        commands: vec![
            Command::Ping,
            Command::Error {
                code: 200,
                message: "OK".into(),
            },
        ],
    };

    let bytes1 = encode_to_vec(&original).expect("encode pass 1");
    let (decoded, _): (Session, _) = decode_from_slice(&bytes1).expect("decode");
    let bytes2 = encode_to_vec(&decoded).expect("encode pass 2");

    assert_eq!(
        bytes1, bytes2,
        "re-encoding a decoded value must yield identical bytes"
    );
}
