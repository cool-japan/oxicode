//! Advanced property-based roundtrip tests (set 37) using proptest.
//!
//! Theme: Network packets / protocol data — PacketType, Packet, NetworkStats, Connection.
//! Each proptest! block contains exactly one #[test] function.
//! Tests verify encode → decode roundtrips, consumed bytes, determinism, boundary values,
//! nested structs, Vec types, and all PacketType variants.

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

// ---------------------------------------------------------------------------
// Domain types
// ---------------------------------------------------------------------------

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
enum PacketType {
    Data,
    Ack,
    Syn,
    SynAck,
    Fin,
    Reset,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct Packet {
    seq_num: u32,
    ack_num: u32,
    packet_type: PacketType,
    payload: Vec<u8>,
    checksum: u16,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct NetworkStats {
    packets_sent: u64,
    packets_received: u64,
    bytes_sent: u64,
    bytes_received: u64,
    error_count: u32,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct Connection {
    src_port: u16,
    dst_port: u16,
    src_ip: u32,
    dst_ip: u32,
    stats: NetworkStats,
    is_active: bool,
}

// ---------------------------------------------------------------------------
// Proptest strategies
// ---------------------------------------------------------------------------

fn arb_packet_type() -> impl Strategy<Value = PacketType> {
    prop_oneof![
        Just(PacketType::Data),
        Just(PacketType::Ack),
        Just(PacketType::Syn),
        Just(PacketType::SynAck),
        Just(PacketType::Fin),
        Just(PacketType::Reset),
    ]
}

fn arb_packet() -> impl Strategy<Value = Packet> {
    (
        any::<u32>(),
        any::<u32>(),
        arb_packet_type(),
        proptest::collection::vec(any::<u8>(), 0..256),
        any::<u16>(),
    )
        .prop_map(
            |(seq_num, ack_num, packet_type, payload, checksum)| Packet {
                seq_num,
                ack_num,
                packet_type,
                payload,
                checksum,
            },
        )
}

fn arb_network_stats() -> impl Strategy<Value = NetworkStats> {
    (
        any::<u64>(),
        any::<u64>(),
        any::<u64>(),
        any::<u64>(),
        any::<u32>(),
    )
        .prop_map(
            |(packets_sent, packets_received, bytes_sent, bytes_received, error_count)| {
                NetworkStats {
                    packets_sent,
                    packets_received,
                    bytes_sent,
                    bytes_received,
                    error_count,
                }
            },
        )
}

fn arb_connection() -> impl Strategy<Value = Connection> {
    (
        any::<u16>(),
        any::<u16>(),
        any::<u32>(),
        any::<u32>(),
        arb_network_stats(),
        any::<bool>(),
    )
        .prop_map(
            |(src_port, dst_port, src_ip, dst_ip, stats, is_active)| Connection {
                src_port,
                dst_port,
                src_ip,
                dst_ip,
                stats,
                is_active,
            },
        )
}

// ---------------------------------------------------------------------------
// Tests
// ---------------------------------------------------------------------------

// 1. PacketType roundtrip
proptest! {
    #[test]
    fn prop_packet_type_roundtrip(pkt_type in arb_packet_type()) {
        let encoded = encode_to_vec(&pkt_type).expect("encode PacketType failed");
        let (decoded, consumed): (PacketType, _) =
            decode_from_slice(&encoded).expect("decode PacketType failed");
        prop_assert_eq!(decoded, pkt_type);
        prop_assert_eq!(consumed, encoded.len());
    }
}

// 2. Packet roundtrip
proptest! {
    #[test]
    fn prop_packet_roundtrip(pkt in arb_packet()) {
        let encoded = encode_to_vec(&pkt).expect("encode Packet failed");
        let (decoded, consumed): (Packet, _) =
            decode_from_slice(&encoded).expect("decode Packet failed");
        prop_assert_eq!(decoded, pkt);
        prop_assert_eq!(consumed, encoded.len());
    }
}

// 3. NetworkStats roundtrip
proptest! {
    #[test]
    fn prop_network_stats_roundtrip(stats in arb_network_stats()) {
        let encoded = encode_to_vec(&stats).expect("encode NetworkStats failed");
        let (decoded, consumed): (NetworkStats, _) =
            decode_from_slice(&encoded).expect("decode NetworkStats failed");
        prop_assert_eq!(decoded, stats);
        prop_assert_eq!(consumed, encoded.len());
    }
}

// 4. Connection roundtrip (nested struct)
proptest! {
    #[test]
    fn prop_connection_roundtrip(conn in arb_connection()) {
        let encoded = encode_to_vec(&conn).expect("encode Connection failed");
        let (decoded, consumed): (Connection, _) =
            decode_from_slice(&encoded).expect("decode Connection failed");
        prop_assert_eq!(decoded, conn);
        prop_assert_eq!(consumed, encoded.len());
    }
}

// 5. Determinism: encoding same Packet twice yields identical bytes
proptest! {
    #[test]
    fn prop_packet_encoding_deterministic(pkt in arb_packet()) {
        let encoded1 = encode_to_vec(&pkt).expect("encode Packet (1st) failed");
        let encoded2 = encode_to_vec(&pkt).expect("encode Packet (2nd) failed");
        prop_assert_eq!(encoded1, encoded2);
    }
}

// 6. Determinism: encoding same Connection twice yields identical bytes
proptest! {
    #[test]
    fn prop_connection_encoding_deterministic(conn in arb_connection()) {
        let encoded1 = encode_to_vec(&conn).expect("encode Connection (1st) failed");
        let encoded2 = encode_to_vec(&conn).expect("encode Connection (2nd) failed");
        prop_assert_eq!(encoded1, encoded2);
    }
}

// 7. Vec<Packet> roundtrip
proptest! {
    #[test]
    fn prop_vec_packet_roundtrip(
        pkts in proptest::collection::vec(arb_packet(), 0..20)
    ) {
        let encoded = encode_to_vec(&pkts).expect("encode Vec<Packet> failed");
        let (decoded, consumed): (Vec<Packet>, _) =
            decode_from_slice(&encoded).expect("decode Vec<Packet> failed");
        prop_assert_eq!(decoded, pkts);
        prop_assert_eq!(consumed, encoded.len());
    }
}

// 8. Vec<Connection> roundtrip
proptest! {
    #[test]
    fn prop_vec_connection_roundtrip(
        conns in proptest::collection::vec(arb_connection(), 0..10)
    ) {
        let encoded = encode_to_vec(&conns).expect("encode Vec<Connection> failed");
        let (decoded, consumed): (Vec<Connection>, _) =
            decode_from_slice(&encoded).expect("decode Vec<Connection> failed");
        prop_assert_eq!(decoded, conns);
        prop_assert_eq!(consumed, encoded.len());
    }
}

// 9. All PacketType variants individually
proptest! {
    #[test]
    fn prop_all_packet_type_variants_roundtrip(_dummy: u8) {
        for variant in [
            PacketType::Data,
            PacketType::Ack,
            PacketType::Syn,
            PacketType::SynAck,
            PacketType::Fin,
            PacketType::Reset,
        ] {
            let encoded = encode_to_vec(&variant).expect("encode PacketType variant failed");
            let (decoded, consumed): (PacketType, _) =
                decode_from_slice(&encoded).expect("decode PacketType variant failed");
            prop_assert_eq!(&decoded, &variant);
            prop_assert_eq!(consumed, encoded.len());
        }
    }
}

// 10. Packet with empty payload
proptest! {
    #[test]
    fn prop_packet_empty_payload_roundtrip(
        seq_num: u32,
        ack_num: u32,
        checksum: u16,
        pkt_type in arb_packet_type()
    ) {
        let pkt = Packet {
            seq_num,
            ack_num,
            packet_type: pkt_type,
            payload: vec![],
            checksum,
        };
        let encoded = encode_to_vec(&pkt).expect("encode Packet (empty payload) failed");
        let (decoded, consumed): (Packet, _) =
            decode_from_slice(&encoded).expect("decode Packet (empty payload) failed");
        prop_assert_eq!(decoded, pkt);
        prop_assert_eq!(consumed, encoded.len());
    }
}

// 11. Packet with maximum-length payload (255 bytes)
proptest! {
    #[test]
    fn prop_packet_max_payload_roundtrip(
        seq_num: u32,
        ack_num: u32,
        checksum: u16,
        fill_byte: u8,
        pkt_type in arb_packet_type()
    ) {
        let pkt = Packet {
            seq_num,
            ack_num,
            packet_type: pkt_type,
            payload: vec![fill_byte; 255],
            checksum,
        };
        let encoded = encode_to_vec(&pkt).expect("encode Packet (max payload) failed");
        let (decoded, consumed): (Packet, _) =
            decode_from_slice(&encoded).expect("decode Packet (max payload) failed");
        prop_assert_eq!(decoded, pkt);
        prop_assert_eq!(consumed, encoded.len());
    }
}

// 12. Boundary values: u16::MAX ports in Connection
proptest! {
    #[test]
    fn prop_connection_boundary_ports(stats in arb_network_stats(), is_active: bool) {
        let conn = Connection {
            src_port: u16::MAX,
            dst_port: u16::MAX,
            src_ip: 0,
            dst_ip: 0,
            stats,
            is_active,
        };
        let encoded = encode_to_vec(&conn).expect("encode Connection (max ports) failed");
        let (decoded, consumed): (Connection, _) =
            decode_from_slice(&encoded).expect("decode Connection (max ports) failed");
        prop_assert_eq!(decoded, conn);
        prop_assert_eq!(consumed, encoded.len());
    }
}

// 13. Boundary values: u32::MAX IP addresses in Connection
proptest! {
    #[test]
    fn prop_connection_boundary_ips(
        src_port: u16,
        dst_port: u16,
        stats in arb_network_stats(),
        is_active: bool
    ) {
        let conn = Connection {
            src_port,
            dst_port,
            src_ip: u32::MAX,
            dst_ip: u32::MAX,
            stats,
            is_active,
        };
        let encoded = encode_to_vec(&conn).expect("encode Connection (max IPs) failed");
        let (decoded, consumed): (Connection, _) =
            decode_from_slice(&encoded).expect("decode Connection (max IPs) failed");
        prop_assert_eq!(decoded, conn);
        prop_assert_eq!(consumed, encoded.len());
    }
}

// 14. NetworkStats with all-zero counters
proptest! {
    #[test]
    fn prop_network_stats_zero_counters(_dummy: u8) {
        let stats = NetworkStats {
            packets_sent: 0,
            packets_received: 0,
            bytes_sent: 0,
            bytes_received: 0,
            error_count: 0,
        };
        let encoded = encode_to_vec(&stats).expect("encode NetworkStats (zeros) failed");
        let (decoded, consumed): (NetworkStats, _) =
            decode_from_slice(&encoded).expect("decode NetworkStats (zeros) failed");
        prop_assert_eq!(decoded, stats);
        prop_assert_eq!(consumed, encoded.len());
    }
}

// 15. NetworkStats with u64::MAX / u32::MAX boundary values
proptest! {
    #[test]
    fn prop_network_stats_max_boundary(_dummy: u8) {
        let stats = NetworkStats {
            packets_sent: u64::MAX,
            packets_received: u64::MAX,
            bytes_sent: u64::MAX,
            bytes_received: u64::MAX,
            error_count: u32::MAX,
        };
        let encoded = encode_to_vec(&stats).expect("encode NetworkStats (max boundary) failed");
        let (decoded, consumed): (NetworkStats, _) =
            decode_from_slice(&encoded).expect("decode NetworkStats (max boundary) failed");
        prop_assert_eq!(decoded, stats);
        prop_assert_eq!(consumed, encoded.len());
    }
}

// 16. Packet payload filled with all-0xFF bytes
proptest! {
    #[test]
    fn prop_packet_payload_all_ff(
        seq_num: u32,
        ack_num: u32,
        checksum: u16,
        len in 1usize..=128usize,
        pkt_type in arb_packet_type()
    ) {
        let pkt = Packet {
            seq_num,
            ack_num,
            packet_type: pkt_type,
            payload: vec![0xFF; len],
            checksum,
        };
        let encoded = encode_to_vec(&pkt).expect("encode Packet (all-FF payload) failed");
        let (decoded, consumed): (Packet, _) =
            decode_from_slice(&encoded).expect("decode Packet (all-FF payload) failed");
        prop_assert_eq!(decoded, pkt);
        prop_assert_eq!(consumed, encoded.len());
    }
}

// 17. Packet payload filled with alternating 0x00/0xFF bytes
proptest! {
    #[test]
    fn prop_packet_payload_alternating_bytes(
        seq_num: u32,
        ack_num: u32,
        checksum: u16,
        len in 2usize..=64usize,
        pkt_type in arb_packet_type()
    ) {
        let payload: Vec<u8> = (0..len).map(|i| if i % 2 == 0 { 0x00 } else { 0xFF }).collect();
        let pkt = Packet {
            seq_num,
            ack_num,
            packet_type: pkt_type,
            payload,
            checksum,
        };
        let encoded = encode_to_vec(&pkt).expect("encode Packet (alternating payload) failed");
        let (decoded, consumed): (Packet, _) =
            decode_from_slice(&encoded).expect("decode Packet (alternating payload) failed");
        prop_assert_eq!(decoded, pkt);
        prop_assert_eq!(consumed, encoded.len());
    }
}

// 18. Consumed bytes equal encoded length for NetworkStats
proptest! {
    #[test]
    fn prop_network_stats_consumed_eq_len(stats in arb_network_stats()) {
        let encoded = encode_to_vec(&stats).expect("encode NetworkStats failed");
        let (_decoded, consumed): (NetworkStats, _) =
            decode_from_slice(&encoded).expect("decode NetworkStats failed");
        prop_assert_eq!(consumed, encoded.len());
    }
}

// 19. Option<Packet> roundtrip (Some and None)
proptest! {
    #[test]
    fn prop_option_packet_roundtrip(
        opt in proptest::option::of(arb_packet())
    ) {
        let encoded = encode_to_vec(&opt).expect("encode Option<Packet> failed");
        let (decoded, consumed): (Option<Packet>, _) =
            decode_from_slice(&encoded).expect("decode Option<Packet> failed");
        prop_assert_eq!(decoded, opt);
        prop_assert_eq!(consumed, encoded.len());
    }
}

// 20. Vec<PacketType> roundtrip
proptest! {
    #[test]
    fn prop_vec_packet_type_roundtrip(
        types in proptest::collection::vec(arb_packet_type(), 0..30)
    ) {
        let encoded = encode_to_vec(&types).expect("encode Vec<PacketType> failed");
        let (decoded, consumed): (Vec<PacketType>, _) =
            decode_from_slice(&encoded).expect("decode Vec<PacketType> failed");
        prop_assert_eq!(decoded, types);
        prop_assert_eq!(consumed, encoded.len());
    }
}

// 21. Connection with inactive flag always false
proptest! {
    #[test]
    fn prop_connection_inactive_roundtrip(
        src_port: u16,
        dst_port: u16,
        src_ip: u32,
        dst_ip: u32,
        stats in arb_network_stats()
    ) {
        let conn = Connection {
            src_port,
            dst_port,
            src_ip,
            dst_ip,
            stats,
            is_active: false,
        };
        let encoded = encode_to_vec(&conn).expect("encode Connection (inactive) failed");
        let (decoded, consumed): (Connection, _) =
            decode_from_slice(&encoded).expect("decode Connection (inactive) failed");
        prop_assert_eq!(decoded, conn);
        prop_assert_eq!(consumed, encoded.len());
    }
}

// 22. Double encode-decode cycle for Packet (encode → decode → re-encode → re-decode)
proptest! {
    #[test]
    fn prop_packet_double_roundtrip(pkt in arb_packet()) {
        let encoded1 = encode_to_vec(&pkt).expect("encode Packet (1st) failed");
        let (decoded1, consumed1): (Packet, _) =
            decode_from_slice(&encoded1).expect("decode Packet (1st) failed");
        prop_assert_eq!(consumed1, encoded1.len());

        let encoded2 = encode_to_vec(&decoded1).expect("encode Packet (2nd) failed");
        let (decoded2, consumed2): (Packet, _) =
            decode_from_slice(&encoded2).expect("decode Packet (2nd) failed");
        prop_assert_eq!(consumed2, encoded2.len());

        prop_assert_eq!(decoded2, pkt);
        prop_assert_eq!(encoded1, encoded2);
    }
}
