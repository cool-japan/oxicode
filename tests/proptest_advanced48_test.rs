//! Advanced property-based roundtrip tests using proptest (set 48).
//!
//! Domain: Networking / packet analysis
//!
//! Each test is a top-level #[test] function inside a proptest! macro block,
//! verifying encode/decode roundtrip invariants for networking types.

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
use oxicode::{decode_from_slice, encode_to_vec, Decode, Encode};
use proptest::prelude::*;

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
enum IpVersion {
    V4,
    V6,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
enum Protocol {
    Tcp,
    Udp,
    Icmp,
    Other(u8),
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct Packet {
    src_ip: u32,
    dst_ip: u32,
    src_port: u16,
    dst_port: u16,
    protocol: Protocol,
    payload: Vec<u8>,
    ttl: u8,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct Connection {
    connection_id: u64,
    ip_version: IpVersion,
    packets: Vec<Packet>,
    bytes_transferred: u64,
}

fn make_ip_version(n: u8) -> IpVersion {
    if n % 2 == 0 {
        IpVersion::V4
    } else {
        IpVersion::V6
    }
}

fn make_protocol(n: u8) -> Protocol {
    match n % 4 {
        0 => Protocol::Tcp,
        1 => Protocol::Udp,
        2 => Protocol::Icmp,
        _ => Protocol::Other(n),
    }
}

fn packet_strategy() -> impl Strategy<Value = Packet> {
    (
        any::<u32>(),
        any::<u32>(),
        any::<u16>(),
        any::<u16>(),
        any::<u8>().prop_map(make_protocol),
        prop::collection::vec(any::<u8>(), 0..64),
        any::<u8>(),
    )
        .prop_map(
            |(src_ip, dst_ip, src_port, dst_port, protocol, payload, ttl)| Packet {
                src_ip,
                dst_ip,
                src_port,
                dst_port,
                protocol,
                payload,
                ttl,
            },
        )
}

fn connection_strategy() -> impl Strategy<Value = Connection> {
    (
        any::<u64>(),
        any::<u8>().prop_map(make_ip_version),
        prop::collection::vec(packet_strategy(), 0..8),
        any::<u64>(),
    )
        .prop_map(
            |(connection_id, ip_version, packets, bytes_transferred)| Connection {
                connection_id,
                ip_version,
                packets,
                bytes_transferred,
            },
        )
}

proptest! {
    #[test]
    fn prop_ip_version_roundtrip(n in 0u8..=1u8) {
        let val = make_ip_version(n);
        let enc = encode_to_vec(&val).expect("encode IpVersion");
        let (dec, _): (IpVersion, usize) = decode_from_slice(&enc).expect("decode IpVersion");
        prop_assert_eq!(val, dec);
    }
}

proptest! {
    #[test]
    fn prop_ip_version_consumed_equals_len(n in 0u8..=1u8) {
        let val = make_ip_version(n);
        let enc = encode_to_vec(&val).expect("encode IpVersion for consumed");
        let (_, consumed): (IpVersion, usize) =
            decode_from_slice(&enc).expect("decode IpVersion for consumed");
        prop_assert_eq!(consumed, enc.len());
    }
}

proptest! {
    #[test]
    fn prop_protocol_roundtrip(n in 0u8..=255u8) {
        let val = make_protocol(n);
        let enc = encode_to_vec(&val).expect("encode Protocol");
        let (dec, _): (Protocol, usize) = decode_from_slice(&enc).expect("decode Protocol");
        prop_assert_eq!(val, dec);
    }
}

proptest! {
    #[test]
    fn prop_protocol_all_variants_roundtrip(n in 0u8..=3u8) {
        let val = match n {
            0 => Protocol::Tcp,
            1 => Protocol::Udp,
            2 => Protocol::Icmp,
            _ => Protocol::Other(42),
        };
        let enc = encode_to_vec(&val).expect("encode Protocol variant");
        let (dec, consumed): (Protocol, usize) =
            decode_from_slice(&enc).expect("decode Protocol variant");
        prop_assert_eq!(val, dec);
        prop_assert_eq!(consumed, enc.len());
    }
}

proptest! {
    #[test]
    fn prop_protocol_other_roundtrip(tag in 0u8..=255u8) {
        let val = Protocol::Other(tag);
        let enc = encode_to_vec(&val).expect("encode Protocol::Other");
        let (dec, _): (Protocol, usize) =
            decode_from_slice(&enc).expect("decode Protocol::Other");
        prop_assert_eq!(val, dec);
    }
}

proptest! {
    #[test]
    fn prop_packet_roundtrip(pkt in packet_strategy()) {
        let enc = encode_to_vec(&pkt).expect("encode Packet");
        let (dec, _): (Packet, usize) = decode_from_slice(&enc).expect("decode Packet");
        prop_assert_eq!(pkt, dec);
    }
}

proptest! {
    #[test]
    fn prop_packet_consumed_equals_len(pkt in packet_strategy()) {
        let enc = encode_to_vec(&pkt).expect("encode Packet for consumed");
        let (_, consumed): (Packet, usize) =
            decode_from_slice(&enc).expect("decode Packet for consumed");
        prop_assert_eq!(consumed, enc.len());
    }
}

proptest! {
    #[test]
    fn prop_packet_deterministic_encoding(pkt in packet_strategy()) {
        let enc1 = encode_to_vec(&pkt).expect("encode Packet first");
        let enc2 = encode_to_vec(&pkt).expect("encode Packet second");
        prop_assert_eq!(enc1, enc2);
    }
}

proptest! {
    #[test]
    fn prop_connection_roundtrip(conn in connection_strategy()) {
        let enc = encode_to_vec(&conn).expect("encode Connection");
        let (dec, _): (Connection, usize) =
            decode_from_slice(&enc).expect("decode Connection");
        prop_assert_eq!(conn, dec);
    }
}

proptest! {
    #[test]
    fn prop_connection_consumed_equals_len(conn in connection_strategy()) {
        let enc = encode_to_vec(&conn).expect("encode Connection for consumed");
        let (_, consumed): (Connection, usize) =
            decode_from_slice(&enc).expect("decode Connection for consumed");
        prop_assert_eq!(consumed, enc.len());
    }
}

proptest! {
    #[test]
    fn prop_connection_deterministic_encoding(conn in connection_strategy()) {
        let enc1 = encode_to_vec(&conn).expect("encode Connection first");
        let enc2 = encode_to_vec(&conn).expect("encode Connection second");
        prop_assert_eq!(enc1, enc2);
    }
}

proptest! {
    #[test]
    fn prop_option_packet_roundtrip(
        present: bool,
        pkt in packet_strategy(),
    ) {
        let opt: Option<Packet> = if present { Some(pkt) } else { None };
        let enc = encode_to_vec(&opt).expect("encode Option<Packet>");
        let (dec, _): (Option<Packet>, usize) =
            decode_from_slice(&enc).expect("decode Option<Packet>");
        prop_assert_eq!(opt, dec);
    }
}

proptest! {
    #[test]
    fn prop_option_packet_consumed_equals_len(
        present: bool,
        pkt in packet_strategy(),
    ) {
        let opt: Option<Packet> = if present { Some(pkt) } else { None };
        let enc = encode_to_vec(&opt).expect("encode Option<Packet> for consumed");
        let (_, consumed): (Option<Packet>, usize) =
            decode_from_slice(&enc).expect("decode Option<Packet> for consumed");
        prop_assert_eq!(consumed, enc.len());
    }
}

proptest! {
    #[test]
    fn prop_vec_packet_roundtrip(
        pkts in prop::collection::vec(packet_strategy(), 0..8)
    ) {
        let enc = encode_to_vec(&pkts).expect("encode Vec<Packet>");
        let (dec, _): (Vec<Packet>, usize) =
            decode_from_slice(&enc).expect("decode Vec<Packet>");
        prop_assert_eq!(pkts, dec);
    }
}

proptest! {
    #[test]
    fn prop_vec_packet_consumed_equals_len(
        pkts in prop::collection::vec(packet_strategy(), 0..8)
    ) {
        let enc = encode_to_vec(&pkts).expect("encode Vec<Packet> for consumed");
        let (_, consumed): (Vec<Packet>, usize) =
            decode_from_slice(&enc).expect("decode Vec<Packet> for consumed");
        prop_assert_eq!(consumed, enc.len());
    }
}

proptest! {
    #[test]
    fn prop_packet_reencode_idempotency(pkt in packet_strategy()) {
        let enc1 = encode_to_vec(&pkt).expect("encode Packet first");
        let (decoded, _): (Packet, usize) =
            decode_from_slice(&enc1).expect("decode Packet after first encode");
        let enc2 = encode_to_vec(&decoded).expect("encode Packet second (re-encode)");
        prop_assert_eq!(enc1, enc2);
    }
}

proptest! {
    #[test]
    fn prop_connection_reencode_idempotency(conn in connection_strategy()) {
        let enc1 = encode_to_vec(&conn).expect("encode Connection first");
        let (decoded, _): (Connection, usize) =
            decode_from_slice(&enc1).expect("decode Connection after first encode");
        let enc2 = encode_to_vec(&decoded).expect("encode Connection second (re-encode)");
        prop_assert_eq!(enc1, enc2);
    }
}

proptest! {
    #[test]
    fn prop_packet_empty_payload_roundtrip(
        src_ip: u32,
        dst_ip: u32,
        src_port: u16,
        dst_port: u16,
        proto_n in 0u8..=255u8,
        ttl: u8,
    ) {
        let pkt = Packet {
            src_ip,
            dst_ip,
            src_port,
            dst_port,
            protocol: make_protocol(proto_n),
            payload: Vec::new(),
            ttl,
        };
        let enc = encode_to_vec(&pkt).expect("encode Packet with empty payload");
        let (dec, consumed): (Packet, usize) =
            decode_from_slice(&enc).expect("decode Packet with empty payload");
        prop_assert_eq!(pkt, dec);
        prop_assert_eq!(consumed, enc.len());
    }
}

proptest! {
    #[test]
    fn prop_packet_loopback_addresses_roundtrip(
        src_port: u16,
        dst_port: u16,
        proto_n in 0u8..=255u8,
        payload in prop::collection::vec(any::<u8>(), 0..32),
        ttl: u8,
    ) {
        // 127.0.0.1 in u32 big-endian = 0x7F000001
        let pkt = Packet {
            src_ip: 0x7F000001u32,
            dst_ip: 0x7F000001u32,
            src_port,
            dst_port,
            protocol: make_protocol(proto_n),
            payload,
            ttl,
        };
        let enc = encode_to_vec(&pkt).expect("encode Packet loopback");
        let (dec, _): (Packet, usize) =
            decode_from_slice(&enc).expect("decode Packet loopback");
        prop_assert_eq!(pkt, dec);
    }
}

proptest! {
    #[test]
    fn prop_connection_v4_roundtrip(
        connection_id: u64,
        pkts in prop::collection::vec(packet_strategy(), 0..5),
        bytes_transferred: u64,
    ) {
        let conn = Connection {
            connection_id,
            ip_version: IpVersion::V4,
            packets: pkts,
            bytes_transferred,
        };
        let enc = encode_to_vec(&conn).expect("encode Connection V4");
        let (dec, consumed): (Connection, usize) =
            decode_from_slice(&enc).expect("decode Connection V4");
        prop_assert_eq!(conn, dec);
        prop_assert_eq!(consumed, enc.len());
    }
}

proptest! {
    #[test]
    fn prop_connection_v6_roundtrip(
        connection_id: u64,
        pkts in prop::collection::vec(packet_strategy(), 0..5),
        bytes_transferred: u64,
    ) {
        let conn = Connection {
            connection_id,
            ip_version: IpVersion::V6,
            packets: pkts,
            bytes_transferred,
        };
        let enc = encode_to_vec(&conn).expect("encode Connection V6");
        let (dec, consumed): (Connection, usize) =
            decode_from_slice(&enc).expect("decode Connection V6");
        prop_assert_eq!(conn, dec);
        prop_assert_eq!(consumed, enc.len());
    }
}

proptest! {
    #[test]
    fn prop_protocol_other_tag_preserved(tag in 0u8..=255u8) {
        let val = Protocol::Other(tag);
        let enc = encode_to_vec(&val).expect("encode Protocol::Other for tag check");
        let (dec, _): (Protocol, usize) =
            decode_from_slice(&enc).expect("decode Protocol::Other for tag check");
        match dec {
            Protocol::Other(decoded_tag) => prop_assert_eq!(tag, decoded_tag),
            other => prop_assert!(false, "expected Protocol::Other, got {:?}", other),
        }
    }
}
