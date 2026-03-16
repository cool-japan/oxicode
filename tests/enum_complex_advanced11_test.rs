//! Tests for Telecommunications / Network Protocol — advanced enum roundtrip coverage.
//!
//! Domain types model a simplified network packet processing pipeline:
//! protocol versioning, message flags, packet headers, payloads, and connection states.

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
// Domain types
// ---------------------------------------------------------------------------

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
enum ProtocolVersion {
    V1,
    V2,
    V3,
    V4,
    Custom { major: u8, minor: u8 },
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
enum MessageFlag {
    Syn,
    Ack,
    Fin,
    Rst,
    Psh,
    Urg,
    Ece,
    Cwr,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct PacketHeader {
    version: ProtocolVersion,
    flags: Vec<MessageFlag>,
    src_port: u16,
    dst_port: u16,
    seq_num: u32,
    ack_num: u32,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
enum PayloadType {
    Text(String),
    Binary(Vec<u8>),
    Json(String),
    Compressed { algorithm: String, data: Vec<u8> },
    Empty,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct NetworkPacket {
    header: PacketHeader,
    payload: PayloadType,
    checksum: u32,
    ttl: u8,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
enum ConnectionState {
    Connecting,
    Connected { session_id: u64 },
    Disconnecting { reason: String },
    Disconnected,
    Error { code: u32, message: String },
}

// ---------------------------------------------------------------------------
// Helpers
// ---------------------------------------------------------------------------

fn make_header(
    version: ProtocolVersion,
    flags: Vec<MessageFlag>,
    src_port: u16,
    dst_port: u16,
    seq_num: u32,
    ack_num: u32,
) -> PacketHeader {
    PacketHeader {
        version,
        flags,
        src_port,
        dst_port,
        seq_num,
        ack_num,
    }
}

fn make_packet(
    header: PacketHeader,
    payload: PayloadType,
    checksum: u32,
    ttl: u8,
) -> NetworkPacket {
    NetworkPacket {
        header,
        payload,
        checksum,
        ttl,
    }
}

// ---------------------------------------------------------------------------
// Test 1: ProtocolVersion — all variants roundtrip
// ---------------------------------------------------------------------------

#[test]
fn test_protocol_version_all_variants_roundtrip() {
    let variants = vec![
        ProtocolVersion::V1,
        ProtocolVersion::V2,
        ProtocolVersion::V3,
        ProtocolVersion::V4,
        ProtocolVersion::Custom { major: 5, minor: 2 },
    ];

    for variant in &variants {
        let bytes = encode_to_vec(variant).expect("encode ProtocolVersion");
        let (decoded, consumed): (ProtocolVersion, usize) =
            decode_from_slice(&bytes).expect("decode ProtocolVersion");
        assert_eq!(variant, &decoded);
        assert_eq!(
            consumed,
            bytes.len(),
            "consumed must equal encoded length for ProtocolVersion"
        );
    }
}

// ---------------------------------------------------------------------------
// Test 2: ProtocolVersion — discriminant uniqueness
// ---------------------------------------------------------------------------

#[test]
fn test_protocol_version_discriminant_uniqueness() {
    let variants = vec![
        ProtocolVersion::V1,
        ProtocolVersion::V2,
        ProtocolVersion::V3,
        ProtocolVersion::V4,
        ProtocolVersion::Custom { major: 0, minor: 0 },
    ];

    let encodings: Vec<Vec<u8>> = variants
        .iter()
        .map(|v| encode_to_vec(v).expect("encode ProtocolVersion for uniqueness"))
        .collect();

    for i in 0..encodings.len() {
        for j in (i + 1)..encodings.len() {
            assert_ne!(
                encodings[i], encodings[j],
                "ProtocolVersion variants {i} and {j} must yield distinct encodings"
            );
        }
    }
}

// ---------------------------------------------------------------------------
// Test 3: ProtocolVersion::Custom — field values preserved
// ---------------------------------------------------------------------------

#[test]
fn test_protocol_version_custom_fields_roundtrip() {
    let cases = vec![
        ProtocolVersion::Custom { major: 0, minor: 0 },
        ProtocolVersion::Custom {
            major: 255,
            minor: 255,
        },
        ProtocolVersion::Custom {
            major: 10,
            minor: 7,
        },
        ProtocolVersion::Custom {
            major: 128,
            minor: 64,
        },
    ];

    for val in &cases {
        let bytes = encode_to_vec(val).expect("encode Custom ProtocolVersion");
        let (decoded, consumed): (ProtocolVersion, usize) =
            decode_from_slice(&bytes).expect("decode Custom ProtocolVersion");
        assert_eq!(val, &decoded);
        assert_eq!(
            consumed,
            bytes.len(),
            "consumed must equal full encoded length"
        );
    }
}

// ---------------------------------------------------------------------------
// Test 4: MessageFlag — all 8 variants roundtrip
// ---------------------------------------------------------------------------

#[test]
fn test_message_flag_all_variants_roundtrip() {
    let flags = vec![
        MessageFlag::Syn,
        MessageFlag::Ack,
        MessageFlag::Fin,
        MessageFlag::Rst,
        MessageFlag::Psh,
        MessageFlag::Urg,
        MessageFlag::Ece,
        MessageFlag::Cwr,
    ];

    for flag in &flags {
        let bytes = encode_to_vec(flag).expect("encode MessageFlag");
        let (decoded, consumed): (MessageFlag, usize) =
            decode_from_slice(&bytes).expect("decode MessageFlag");
        assert_eq!(flag, &decoded);
        assert_eq!(
            consumed,
            bytes.len(),
            "consumed must equal encoded length for MessageFlag"
        );
    }
}

// ---------------------------------------------------------------------------
// Test 5: MessageFlag — all 8 variants produce distinct encodings
// ---------------------------------------------------------------------------

#[test]
fn test_message_flag_discriminant_uniqueness() {
    let flags = vec![
        MessageFlag::Syn,
        MessageFlag::Ack,
        MessageFlag::Fin,
        MessageFlag::Rst,
        MessageFlag::Psh,
        MessageFlag::Urg,
        MessageFlag::Ece,
        MessageFlag::Cwr,
    ];

    let encodings: Vec<Vec<u8>> = flags
        .iter()
        .map(|f| encode_to_vec(f).expect("encode MessageFlag for uniqueness"))
        .collect();

    for i in 0..encodings.len() {
        for j in (i + 1)..encodings.len() {
            assert_ne!(
                encodings[i], encodings[j],
                "MessageFlag variants {i} and {j} must yield distinct encodings"
            );
        }
    }
}

// ---------------------------------------------------------------------------
// Test 6: PacketHeader — basic SYN packet roundtrip
// ---------------------------------------------------------------------------

#[test]
fn test_packet_header_syn_roundtrip() {
    let header = make_header(ProtocolVersion::V2, vec![MessageFlag::Syn], 12345, 80, 0, 0);
    let bytes = encode_to_vec(&header).expect("encode PacketHeader SYN");
    let (decoded, consumed): (PacketHeader, usize) =
        decode_from_slice(&bytes).expect("decode PacketHeader SYN");
    assert_eq!(header, decoded);
    assert_eq!(
        consumed,
        bytes.len(),
        "consumed must equal encoded length for PacketHeader"
    );
}

// ---------------------------------------------------------------------------
// Test 7: PacketHeader — SYN+ACK with Custom version, extreme port values
// ---------------------------------------------------------------------------

#[test]
fn test_packet_header_syn_ack_custom_version_roundtrip() {
    let header = make_header(
        ProtocolVersion::Custom { major: 3, minor: 1 },
        vec![MessageFlag::Syn, MessageFlag::Ack],
        u16::MAX,
        443,
        1_000_000,
        999_999,
    );
    let bytes = encode_to_vec(&header).expect("encode SYN-ACK PacketHeader");
    let (decoded, consumed): (PacketHeader, usize) =
        decode_from_slice(&bytes).expect("decode SYN-ACK PacketHeader");
    assert_eq!(header, decoded);
    assert_eq!(consumed, bytes.len(), "consumed must match full encoding");
}

// ---------------------------------------------------------------------------
// Test 8: PacketHeader — all flags simultaneously
// ---------------------------------------------------------------------------

#[test]
fn test_packet_header_all_flags_roundtrip() {
    let header = make_header(
        ProtocolVersion::V4,
        vec![
            MessageFlag::Syn,
            MessageFlag::Ack,
            MessageFlag::Fin,
            MessageFlag::Rst,
            MessageFlag::Psh,
            MessageFlag::Urg,
            MessageFlag::Ece,
            MessageFlag::Cwr,
        ],
        8080,
        9090,
        u32::MAX,
        u32::MAX - 1,
    );
    let bytes = encode_to_vec(&header).expect("encode all-flags PacketHeader");
    let (decoded, consumed): (PacketHeader, usize) =
        decode_from_slice(&bytes).expect("decode all-flags PacketHeader");
    assert_eq!(header, decoded);
    assert_eq!(
        consumed,
        bytes.len(),
        "consumed must match all-flags PacketHeader encoding"
    );
}

// ---------------------------------------------------------------------------
// Test 9: PayloadType — all variants roundtrip
// ---------------------------------------------------------------------------

#[test]
fn test_payload_type_all_variants_roundtrip() {
    let payloads = vec![
        PayloadType::Empty,
        PayloadType::Text("Hello, network!".to_string()),
        PayloadType::Binary(vec![0xDE, 0xAD, 0xBE, 0xEF, 0xCA, 0xFE]),
        PayloadType::Json(r#"{"type":"ping","timestamp":1706000000}"#.to_string()),
        PayloadType::Compressed {
            algorithm: "lz4".to_string(),
            data: vec![0x04, 0x22, 0x4D, 0x18, 0x64, 0x40],
        },
    ];

    for payload in &payloads {
        let bytes = encode_to_vec(payload).expect("encode PayloadType");
        let (decoded, consumed): (PayloadType, usize) =
            decode_from_slice(&bytes).expect("decode PayloadType");
        assert_eq!(payload, &decoded);
        assert_eq!(
            consumed,
            bytes.len(),
            "consumed must equal encoded length for PayloadType"
        );
    }
}

// ---------------------------------------------------------------------------
// Test 10: NetworkPacket — full SYN packet with text payload
// ---------------------------------------------------------------------------

#[test]
fn test_network_packet_syn_text_payload_roundtrip() {
    let header = make_header(
        ProtocolVersion::V1,
        vec![MessageFlag::Syn],
        54321,
        80,
        100,
        0,
    );
    let packet = make_packet(
        header,
        PayloadType::Text("GET / HTTP/1.1\r\nHost: example.com\r\n\r\n".to_string()),
        0xABCD_EF01,
        64,
    );
    let bytes = encode_to_vec(&packet).expect("encode NetworkPacket SYN-text");
    let (decoded, consumed): (NetworkPacket, usize) =
        decode_from_slice(&bytes).expect("decode NetworkPacket SYN-text");
    assert_eq!(packet, decoded);
    assert_eq!(
        consumed,
        bytes.len(),
        "consumed must equal full packet encoding length"
    );
}

// ---------------------------------------------------------------------------
// Test 11: NetworkPacket — binary payload with large seq_num
// ---------------------------------------------------------------------------

#[test]
fn test_network_packet_binary_payload_roundtrip() {
    let header = make_header(
        ProtocolVersion::V3,
        vec![MessageFlag::Psh, MessageFlag::Ack],
        40000,
        22,
        2_147_483_648,
        2_147_483_647,
    );
    let packet = make_packet(
        header,
        PayloadType::Binary((0u8..=127).collect()),
        0xFFFF_FFFF,
        128,
    );
    let bytes = encode_to_vec(&packet).expect("encode binary NetworkPacket");
    let (decoded, consumed): (NetworkPacket, usize) =
        decode_from_slice(&bytes).expect("decode binary NetworkPacket");
    assert_eq!(packet, decoded);
    assert_eq!(
        consumed,
        bytes.len(),
        "consumed must equal full binary packet encoding"
    );
}

// ---------------------------------------------------------------------------
// Test 12: NetworkPacket — JSON payload roundtrip
// ---------------------------------------------------------------------------

#[test]
fn test_network_packet_json_payload_roundtrip() {
    let json_body =
        r#"{"protocol":"oxicode","version":2,"session":"a1b2c3d4","command":"handshake"}"#;
    let header = make_header(
        ProtocolVersion::V2,
        vec![MessageFlag::Ack],
        9001,
        9002,
        500,
        501,
    );
    let packet = make_packet(
        header,
        PayloadType::Json(json_body.to_string()),
        0x1234_5678,
        32,
    );
    let bytes = encode_to_vec(&packet).expect("encode JSON NetworkPacket");
    let (decoded, consumed): (NetworkPacket, usize) =
        decode_from_slice(&bytes).expect("decode JSON NetworkPacket");
    assert_eq!(packet, decoded);
    assert_eq!(
        consumed,
        bytes.len(),
        "consumed must match JSON packet encoding"
    );
}

// ---------------------------------------------------------------------------
// Test 13: NetworkPacket — Compressed payload roundtrip
// ---------------------------------------------------------------------------

#[test]
fn test_network_packet_compressed_payload_roundtrip() {
    let compressed_data: Vec<u8> = (0u8..=255).cycle().take(512).collect();
    let header = make_header(
        ProtocolVersion::V4,
        vec![MessageFlag::Psh],
        7777,
        8888,
        300_000,
        299_999,
    );
    let packet = make_packet(
        header,
        PayloadType::Compressed {
            algorithm: "zstd".to_string(),
            data: compressed_data,
        },
        0xDEAD_BEEF,
        48,
    );
    let bytes = encode_to_vec(&packet).expect("encode compressed NetworkPacket");
    let (decoded, consumed): (NetworkPacket, usize) =
        decode_from_slice(&bytes).expect("decode compressed NetworkPacket");
    assert_eq!(packet, decoded);
    assert_eq!(
        consumed,
        bytes.len(),
        "consumed must match compressed packet encoding"
    );
}

// ---------------------------------------------------------------------------
// Test 14: NetworkPacket — Empty payload (FIN teardown)
// ---------------------------------------------------------------------------

#[test]
fn test_network_packet_empty_payload_fin_roundtrip() {
    let header = make_header(
        ProtocolVersion::V1,
        vec![MessageFlag::Fin, MessageFlag::Ack],
        60001,
        443,
        9_999_999,
        10_000_000,
    );
    let packet = make_packet(header, PayloadType::Empty, 0x0000_0000, 1);
    let bytes = encode_to_vec(&packet).expect("encode FIN NetworkPacket");
    let (decoded, consumed): (NetworkPacket, usize) =
        decode_from_slice(&bytes).expect("decode FIN NetworkPacket");
    assert_eq!(packet, decoded);
    assert_eq!(
        consumed,
        bytes.len(),
        "consumed must equal FIN packet encoding length"
    );
}

// ---------------------------------------------------------------------------
// Test 15: ConnectionState — all variants roundtrip
// ---------------------------------------------------------------------------

#[test]
fn test_connection_state_all_variants_roundtrip() {
    let states = vec![
        ConnectionState::Connecting,
        ConnectionState::Connected {
            session_id: 0x0102_0304_0506_0708,
        },
        ConnectionState::Disconnecting {
            reason: "user initiated teardown".to_string(),
        },
        ConnectionState::Disconnected,
        ConnectionState::Error {
            code: 503,
            message: "Service Unavailable — upstream timeout exceeded 30s".to_string(),
        },
    ];

    for state in &states {
        let bytes = encode_to_vec(state).expect("encode ConnectionState");
        let (decoded, consumed): (ConnectionState, usize) =
            decode_from_slice(&bytes).expect("decode ConnectionState");
        assert_eq!(state, &decoded);
        assert_eq!(
            consumed,
            bytes.len(),
            "consumed must equal encoded length for ConnectionState"
        );
    }
}

// ---------------------------------------------------------------------------
// Test 16: ConnectionState — discriminant uniqueness
// ---------------------------------------------------------------------------

#[test]
fn test_connection_state_discriminant_uniqueness() {
    let states = vec![
        ConnectionState::Connecting,
        ConnectionState::Connected { session_id: 1 },
        ConnectionState::Disconnecting {
            reason: "x".to_string(),
        },
        ConnectionState::Disconnected,
        ConnectionState::Error {
            code: 1,
            message: "e".to_string(),
        },
    ];

    let encodings: Vec<Vec<u8>> = states
        .iter()
        .map(|s| encode_to_vec(s).expect("encode ConnectionState for uniqueness"))
        .collect();

    for i in 0..encodings.len() {
        for j in (i + 1)..encodings.len() {
            assert_ne!(
                encodings[i], encodings[j],
                "ConnectionState variants {i} and {j} must yield distinct encodings"
            );
        }
    }
}

// ---------------------------------------------------------------------------
// Test 17: Vec<NetworkPacket> — mixed batch roundtrip
// ---------------------------------------------------------------------------

#[test]
fn test_vec_network_packets_roundtrip() {
    let packets: Vec<NetworkPacket> = vec![
        make_packet(
            make_header(ProtocolVersion::V1, vec![MessageFlag::Syn], 1001, 80, 0, 0),
            PayloadType::Empty,
            0x0000_0001,
            64,
        ),
        make_packet(
            make_header(
                ProtocolVersion::V2,
                vec![MessageFlag::Syn, MessageFlag::Ack],
                80,
                1001,
                0,
                1,
            ),
            PayloadType::Text("HTTP/1.1 200 OK".to_string()),
            0x0000_0002,
            63,
        ),
        make_packet(
            make_header(
                ProtocolVersion::V4,
                vec![MessageFlag::Psh, MessageFlag::Ack],
                1001,
                80,
                1,
                1,
            ),
            PayloadType::Binary(b"PING".to_vec()),
            0x0000_0003,
            62,
        ),
        make_packet(
            make_header(
                ProtocolVersion::V3,
                vec![MessageFlag::Fin, MessageFlag::Ack],
                1001,
                80,
                2,
                2,
            ),
            PayloadType::Empty,
            0x0000_0004,
            61,
        ),
    ];

    let bytes = encode_to_vec(&packets).expect("encode Vec<NetworkPacket>");
    let (decoded, consumed): (Vec<NetworkPacket>, usize) =
        decode_from_slice(&bytes).expect("decode Vec<NetworkPacket>");
    assert_eq!(packets, decoded);
    assert_eq!(
        consumed,
        bytes.len(),
        "consumed must equal Vec<NetworkPacket> encoding length"
    );
}

// ---------------------------------------------------------------------------
// Test 18: Big-endian config — NetworkPacket roundtrip
// ---------------------------------------------------------------------------

#[test]
fn test_network_packet_big_endian_config_roundtrip() {
    let cfg = config::standard().with_big_endian();
    let header = make_header(
        ProtocolVersion::V2,
        vec![MessageFlag::Ack],
        443,
        55555,
        77777,
        77778,
    );
    let packet = make_packet(
        header,
        PayloadType::Text("status: connected".to_string()),
        0xCAFE_BABE,
        60,
    );
    let bytes = encode_to_vec_with_config(&packet, cfg).expect("encode big-endian NetworkPacket");
    let (decoded, consumed): (NetworkPacket, usize) =
        decode_from_slice_with_config(&bytes, cfg).expect("decode big-endian NetworkPacket");
    assert_eq!(packet, decoded);
    assert_eq!(
        consumed,
        bytes.len(),
        "consumed must equal big-endian encoding length"
    );
}

// ---------------------------------------------------------------------------
// Test 19: Fixed-int config — NetworkPacket roundtrip
// ---------------------------------------------------------------------------

#[test]
fn test_network_packet_fixed_int_config_roundtrip() {
    let cfg = config::standard().with_fixed_int_encoding();
    let header = make_header(
        ProtocolVersion::V3,
        vec![MessageFlag::Rst],
        0,
        0,
        u32::MAX,
        0,
    );
    let packet = make_packet(header, PayloadType::Empty, 0xBEEF_CAFE, 255);
    let bytes = encode_to_vec_with_config(&packet, cfg).expect("encode fixed-int NetworkPacket");
    let (decoded, consumed): (NetworkPacket, usize) =
        decode_from_slice_with_config(&bytes, cfg).expect("decode fixed-int NetworkPacket");
    assert_eq!(packet, decoded);
    assert_eq!(
        consumed,
        bytes.len(),
        "consumed must equal fixed-int encoding length"
    );
}

// ---------------------------------------------------------------------------
// Test 20: Big-endian + fixed-int config — ConnectionState::Connected roundtrip
// ---------------------------------------------------------------------------

#[test]
fn test_connection_state_big_endian_fixed_int_roundtrip() {
    let cfg = config::standard()
        .with_big_endian()
        .with_fixed_int_encoding();
    let state = ConnectionState::Connected {
        session_id: u64::MAX,
    };
    let bytes =
        encode_to_vec_with_config(&state, cfg).expect("encode ConnectionState big-endian+fixed");
    let (decoded, consumed): (ConnectionState, usize) = decode_from_slice_with_config(&bytes, cfg)
        .expect("decode ConnectionState big-endian+fixed");
    assert_eq!(state, decoded);
    assert_eq!(
        consumed,
        bytes.len(),
        "consumed must equal big-endian+fixed encoding length"
    );
}

// ---------------------------------------------------------------------------
// Test 21: PayloadType::Compressed discriminant differs from other PayloadType variants
// ---------------------------------------------------------------------------

#[test]
fn test_payload_type_compressed_discriminant_uniqueness() {
    let payloads: Vec<PayloadType> = vec![
        PayloadType::Empty,
        PayloadType::Text("x".to_string()),
        PayloadType::Binary(vec![0x00]),
        PayloadType::Json("{}".to_string()),
        PayloadType::Compressed {
            algorithm: "lz4".to_string(),
            data: vec![0x00],
        },
    ];

    let encodings: Vec<Vec<u8>> = payloads
        .iter()
        .map(|p| encode_to_vec(p).expect("encode PayloadType for uniqueness"))
        .collect();

    for i in 0..encodings.len() {
        for j in (i + 1)..encodings.len() {
            assert_ne!(
                encodings[i], encodings[j],
                "PayloadType variants {i} and {j} must yield distinct encodings"
            );
        }
    }
}

// ---------------------------------------------------------------------------
// Test 22: Consumed bytes accuracy across multiple sequential decode calls
// ---------------------------------------------------------------------------

#[test]
fn test_consumed_bytes_accuracy_sequential_decodes() {
    // Encode three independent values into one buffer and verify consumed counts.
    let state1 = ConnectionState::Connecting;
    let state2 = ConnectionState::Connected {
        session_id: 42_000_000_000,
    };
    let state3 = ConnectionState::Error {
        code: 404,
        message: "Not Found".to_string(),
    };

    let mut buffer: Vec<u8> = Vec::new();
    buffer.extend(encode_to_vec(&state1).expect("encode state1"));
    buffer.extend(encode_to_vec(&state2).expect("encode state2"));
    buffer.extend(encode_to_vec(&state3).expect("encode state3"));

    let (decoded1, consumed1): (ConnectionState, usize) =
        decode_from_slice(&buffer).expect("decode state1");
    assert_eq!(state1, decoded1);

    let (decoded2, consumed2): (ConnectionState, usize) =
        decode_from_slice(&buffer[consumed1..]).expect("decode state2");
    assert_eq!(state2, decoded2);

    let (decoded3, consumed3): (ConnectionState, usize) =
        decode_from_slice(&buffer[consumed1 + consumed2..]).expect("decode state3");
    assert_eq!(state3, decoded3);

    assert_eq!(
        consumed1 + consumed2 + consumed3,
        buffer.len(),
        "sum of consumed bytes must equal total buffer length"
    );
}
