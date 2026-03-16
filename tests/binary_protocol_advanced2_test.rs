//! Advanced binary protocol / message encoding tests for OxiCode.

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
// Shared protocol types
// ---------------------------------------------------------------------------

#[derive(Encode, Decode, Debug, PartialEq, Clone)]
struct PacketHeader {
    version: u8,
    packet_type: u8,
    sequence: u32,
    payload_len: u32,
}

#[derive(Encode, Decode, Debug, PartialEq, Clone)]
struct DataPacket {
    header: PacketHeader,
    payload: Vec<u8>,
}

#[derive(Encode, Decode, Debug, PartialEq, Clone)]
enum Command {
    Ping { id: u32 },
    Pong { id: u32 },
    Data { seq: u32, data: Vec<u8> },
    Error { code: u16, message: String },
    Disconnect,
}

#[derive(Encode, Decode, Debug, PartialEq, Clone)]
struct Frame {
    command: Command,
    timestamp: u64,
}

// ---------------------------------------------------------------------------
// 1. test_packet_header_roundtrip
// ---------------------------------------------------------------------------

#[test]
fn test_packet_header_roundtrip() {
    let header = PacketHeader {
        version: 1,
        packet_type: 2,
        sequence: 42,
        payload_len: 256,
    };
    let enc = encode_to_vec(&header).expect("encode PacketHeader");
    let (decoded, _): (PacketHeader, usize) = decode_from_slice(&enc).expect("decode PacketHeader");
    assert_eq!(header, decoded);
}

// ---------------------------------------------------------------------------
// 2. test_data_packet_roundtrip
// ---------------------------------------------------------------------------

#[test]
fn test_data_packet_roundtrip() {
    let packet = DataPacket {
        header: PacketHeader {
            version: 2,
            packet_type: 10,
            sequence: 1234,
            payload_len: 5,
        },
        payload: vec![0xAA, 0xBB, 0xCC, 0xDD, 0xEE],
    };
    let enc = encode_to_vec(&packet).expect("encode DataPacket");
    let (decoded, _): (DataPacket, usize) = decode_from_slice(&enc).expect("decode DataPacket");
    assert_eq!(packet, decoded);
}

// ---------------------------------------------------------------------------
// 3. test_data_packet_empty_payload
// ---------------------------------------------------------------------------

#[test]
fn test_data_packet_empty_payload() {
    let packet = DataPacket {
        header: PacketHeader {
            version: 1,
            packet_type: 0,
            sequence: 0,
            payload_len: 0,
        },
        payload: vec![],
    };
    let enc = encode_to_vec(&packet).expect("encode empty-payload DataPacket");
    let (decoded, _): (DataPacket, usize) =
        decode_from_slice(&enc).expect("decode empty-payload DataPacket");
    assert_eq!(packet, decoded);
    assert!(decoded.payload.is_empty());
}

// ---------------------------------------------------------------------------
// 4. test_command_ping_roundtrip
// ---------------------------------------------------------------------------

#[test]
fn test_command_ping_roundtrip() {
    let cmd = Command::Ping { id: 1 };
    let enc = encode_to_vec(&cmd).expect("encode Ping");
    let (decoded, _): (Command, usize) = decode_from_slice(&enc).expect("decode Ping");
    assert_eq!(cmd, decoded);
}

// ---------------------------------------------------------------------------
// 5. test_command_pong_roundtrip
// ---------------------------------------------------------------------------

#[test]
fn test_command_pong_roundtrip() {
    let cmd = Command::Pong { id: 1 };
    let enc = encode_to_vec(&cmd).expect("encode Pong");
    let (decoded, _): (Command, usize) = decode_from_slice(&enc).expect("decode Pong");
    assert_eq!(cmd, decoded);
}

// ---------------------------------------------------------------------------
// 6. test_command_data_roundtrip
// ---------------------------------------------------------------------------

#[test]
fn test_command_data_roundtrip() {
    let cmd = Command::Data {
        seq: 0,
        data: vec![1, 2, 3],
    };
    let enc = encode_to_vec(&cmd).expect("encode Command::Data");
    let (decoded, _): (Command, usize) = decode_from_slice(&enc).expect("decode Command::Data");
    assert_eq!(cmd, decoded);
}

// ---------------------------------------------------------------------------
// 7. test_command_error_roundtrip
// ---------------------------------------------------------------------------

#[test]
fn test_command_error_roundtrip() {
    let cmd = Command::Error {
        code: 404,
        message: "not found".to_string(),
    };
    let enc = encode_to_vec(&cmd).expect("encode Command::Error");
    let (decoded, _): (Command, usize) = decode_from_slice(&enc).expect("decode Command::Error");
    assert_eq!(cmd, decoded);
}

// ---------------------------------------------------------------------------
// 8. test_command_disconnect_roundtrip
// ---------------------------------------------------------------------------

#[test]
fn test_command_disconnect_roundtrip() {
    let cmd = Command::Disconnect;
    let enc = encode_to_vec(&cmd).expect("encode Disconnect");
    let (decoded, _): (Command, usize) = decode_from_slice(&enc).expect("decode Disconnect");
    assert_eq!(cmd, decoded);
}

// ---------------------------------------------------------------------------
// 9. test_frame_roundtrip
// ---------------------------------------------------------------------------

#[test]
fn test_frame_roundtrip() {
    let frame = Frame {
        command: Command::Ping { id: 99 },
        timestamp: 1_700_000_000,
    };
    let enc = encode_to_vec(&frame).expect("encode Frame (Ping)");
    let (decoded, _): (Frame, usize) = decode_from_slice(&enc).expect("decode Frame (Ping)");
    assert_eq!(frame, decoded);
}

// ---------------------------------------------------------------------------
// 10. test_frame_error_roundtrip
// ---------------------------------------------------------------------------

#[test]
fn test_frame_error_roundtrip() {
    let frame = Frame {
        command: Command::Error {
            code: 500,
            message: "internal server error".to_string(),
        },
        timestamp: 9_999_999_999,
    };
    let enc = encode_to_vec(&frame).expect("encode Frame (Error)");
    let (decoded, _): (Frame, usize) = decode_from_slice(&enc).expect("decode Frame (Error)");
    assert_eq!(frame, decoded);
}

// ---------------------------------------------------------------------------
// 11. test_vec_command_roundtrip
// ---------------------------------------------------------------------------

#[test]
fn test_vec_command_roundtrip() {
    let cmds: Vec<Command> = vec![
        Command::Ping { id: 1 },
        Command::Pong { id: 1 },
        Command::Data {
            seq: 7,
            data: vec![10, 20, 30],
        },
        Command::Error {
            code: 503,
            message: "unavailable".to_string(),
        },
        Command::Disconnect,
    ];
    let enc = encode_to_vec(&cmds).expect("encode Vec<Command>");
    let (decoded, _): (Vec<Command>, usize) = decode_from_slice(&enc).expect("decode Vec<Command>");
    assert_eq!(cmds, decoded);
}

// ---------------------------------------------------------------------------
// 12. test_vec_frame_roundtrip
// ---------------------------------------------------------------------------

#[test]
fn test_vec_frame_roundtrip() {
    let frames: Vec<Frame> = vec![
        Frame {
            command: Command::Ping { id: 1 },
            timestamp: 100,
        },
        Frame {
            command: Command::Data {
                seq: 2,
                data: vec![0xFF, 0x00],
            },
            timestamp: 200,
        },
        Frame {
            command: Command::Disconnect,
            timestamp: 300,
        },
    ];
    let enc = encode_to_vec(&frames).expect("encode Vec<Frame>");
    let (decoded, _): (Vec<Frame>, usize) = decode_from_slice(&enc).expect("decode Vec<Frame>");
    assert_eq!(frames, decoded);
}

// ---------------------------------------------------------------------------
// 13. test_packet_header_fixed_int_config
// ---------------------------------------------------------------------------

#[test]
fn test_packet_header_fixed_int_config() {
    let header = PacketHeader {
        version: 3,
        packet_type: 7,
        sequence: 65_535,
        payload_len: 1_024,
    };
    let cfg = config::standard().with_fixed_int_encoding();
    let enc = encode_to_vec_with_config(&header, cfg).expect("encode with fixed_int");
    let (decoded, consumed): (PacketHeader, usize) =
        decode_from_slice_with_config(&enc, cfg).expect("decode with fixed_int");
    assert_eq!(header, decoded);
    assert_eq!(consumed, enc.len());
}

// ---------------------------------------------------------------------------
// 14. test_packet_header_big_endian_config
// ---------------------------------------------------------------------------

#[test]
fn test_packet_header_big_endian_config() {
    let header = PacketHeader {
        version: 1,
        packet_type: 2,
        sequence: 0x0102_0304,
        payload_len: 0xDEAD_BEEF,
    };
    let cfg = config::standard()
        .with_big_endian()
        .with_fixed_int_encoding();
    let enc = encode_to_vec_with_config(&header, cfg).expect("encode big-endian");
    let (decoded, consumed): (PacketHeader, usize) =
        decode_from_slice_with_config(&enc, cfg).expect("decode big-endian");
    assert_eq!(header, decoded);
    assert_eq!(consumed, enc.len());
}

// ---------------------------------------------------------------------------
// 15. test_data_packet_consumed_equals_len
// ---------------------------------------------------------------------------

#[test]
fn test_data_packet_consumed_equals_len() {
    let packet = DataPacket {
        header: PacketHeader {
            version: 1,
            packet_type: 3,
            sequence: 88,
            payload_len: 4,
        },
        payload: vec![0x01, 0x02, 0x03, 0x04],
    };
    let enc = encode_to_vec(&packet).expect("encode DataPacket (consumed check)");
    let (_, consumed): (DataPacket, usize) =
        decode_from_slice(&enc).expect("decode DataPacket (consumed check)");
    assert_eq!(consumed, enc.len());
}

// ---------------------------------------------------------------------------
// 16. test_command_size_ordering
// ---------------------------------------------------------------------------

#[test]
fn test_command_size_ordering() {
    let disconnect_enc = encode_to_vec(&Command::Disconnect).expect("encode Disconnect");
    let ping_enc = encode_to_vec(&Command::Ping { id: 1 }).expect("encode Ping");
    let large_data_enc = encode_to_vec(&Command::Data {
        seq: 0,
        data: vec![0u8; 100],
    })
    .expect("encode large Data");

    // Disconnect (unit variant) < Ping (one u32 field) < Data with 100 bytes
    assert!(
        disconnect_enc.len() < ping_enc.len(),
        "Disconnect must be smaller than Ping"
    );
    assert!(
        ping_enc.len() < large_data_enc.len(),
        "Ping must be smaller than Data(100 bytes)"
    );
}

// ---------------------------------------------------------------------------
// 17. test_frame_consumed_equals_len
// ---------------------------------------------------------------------------

#[test]
fn test_frame_consumed_equals_len() {
    let frame = Frame {
        command: Command::Pong { id: 42 },
        timestamp: 1_234_567_890,
    };
    let enc = encode_to_vec(&frame).expect("encode Frame (consumed check)");
    let (_, consumed): (Frame, usize) =
        decode_from_slice(&enc).expect("decode Frame (consumed check)");
    assert_eq!(consumed, enc.len());
}

// ---------------------------------------------------------------------------
// 18. test_packet_header_field_preservation
// ---------------------------------------------------------------------------

#[test]
fn test_packet_header_field_preservation() {
    let header = PacketHeader {
        version: 255,
        packet_type: 128,
        sequence: 0xFFFF_FFFF,
        payload_len: 0x0000_0001,
    };
    let enc = encode_to_vec(&header).expect("encode header (field preservation)");
    let (decoded, _): (PacketHeader, usize) =
        decode_from_slice(&enc).expect("decode header (field preservation)");
    assert_eq!(decoded.version, 255);
    assert_eq!(decoded.packet_type, 128);
    assert_eq!(decoded.sequence, 0xFFFF_FFFF);
    assert_eq!(decoded.payload_len, 0x0000_0001);
}

// ---------------------------------------------------------------------------
// 19. test_command_different_variants_different_bytes
// ---------------------------------------------------------------------------

#[test]
fn test_command_different_variants_different_bytes() {
    let ping = encode_to_vec(&Command::Ping { id: 0 }).expect("encode Ping");
    let pong = encode_to_vec(&Command::Pong { id: 0 }).expect("encode Pong");
    let data = encode_to_vec(&Command::Data {
        seq: 0,
        data: vec![],
    })
    .expect("encode Data");
    let error = encode_to_vec(&Command::Error {
        code: 0,
        message: String::new(),
    })
    .expect("encode Error");
    let disconnect = encode_to_vec(&Command::Disconnect).expect("encode Disconnect");

    // All five variants must produce distinct byte sequences
    assert_ne!(ping, pong);
    assert_ne!(ping, data);
    assert_ne!(ping, error);
    assert_ne!(ping, disconnect);
    assert_ne!(pong, data);
    assert_ne!(pong, error);
    assert_ne!(pong, disconnect);
    assert_ne!(data, error);
    assert_ne!(data, disconnect);
    assert_ne!(error, disconnect);
}

// ---------------------------------------------------------------------------
// 20. test_data_packet_large_payload
// ---------------------------------------------------------------------------

#[test]
fn test_data_packet_large_payload() {
    let payload: Vec<u8> = (0u16..1000).map(|i| (i % 256) as u8).collect();
    let packet = DataPacket {
        header: PacketHeader {
            version: 1,
            packet_type: 5,
            sequence: 9999,
            payload_len: 1000,
        },
        payload: payload.clone(),
    };
    let enc = encode_to_vec(&packet).expect("encode large DataPacket");
    let (decoded, consumed): (DataPacket, usize) =
        decode_from_slice(&enc).expect("decode large DataPacket");
    assert_eq!(decoded.payload.len(), 1000);
    assert_eq!(decoded.payload, payload);
    assert_eq!(consumed, enc.len());
}

// ---------------------------------------------------------------------------
// 21. test_frame_timestamp_preserved
// ---------------------------------------------------------------------------

#[test]
fn test_frame_timestamp_preserved() {
    let frame = Frame {
        command: Command::Disconnect,
        timestamp: u64::MAX,
    };
    let enc = encode_to_vec(&frame).expect("encode Frame (u64::MAX timestamp)");
    let (decoded, _): (Frame, usize) =
        decode_from_slice(&enc).expect("decode Frame (u64::MAX timestamp)");
    assert_eq!(decoded.timestamp, u64::MAX);
}

// ---------------------------------------------------------------------------
// 22. test_multiple_packets_sequential_decode
// ---------------------------------------------------------------------------

#[test]
fn test_multiple_packets_sequential_decode() {
    let cmd1 = Command::Ping { id: 10 };
    let cmd2 = Command::Error {
        code: 404,
        message: "not found".to_string(),
    };

    let enc1 = encode_to_vec(&cmd1).expect("encode cmd1");
    let enc2 = encode_to_vec(&cmd2).expect("encode cmd2");

    // Concatenate both encodings into a single buffer
    let mut concat = Vec::with_capacity(enc1.len() + enc2.len());
    concat.extend_from_slice(&enc1);
    concat.extend_from_slice(&enc2);

    // Decode first command from concatenated slice
    let (decoded1, consumed1): (Command, usize) =
        decode_from_slice(&concat).expect("decode cmd1 from concat");
    assert_eq!(decoded1, cmd1);
    assert_eq!(consumed1, enc1.len());

    // Decode second command from remainder
    let (decoded2, consumed2): (Command, usize) =
        decode_from_slice(&concat[consumed1..]).expect("decode cmd2 from concat remainder");
    assert_eq!(decoded2, cmd2);
    assert_eq!(consumed2, enc2.len());
}
