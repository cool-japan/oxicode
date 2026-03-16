//! Advanced checksum encoding tests – checksum_advanced10_test.rs

#![cfg(feature = "checksum")]
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
use oxicode::checksum::{decode_with_checksum, encode_with_checksum, HEADER_SIZE};
use oxicode::{encode_to_vec, Decode, Encode};

#[derive(Debug, PartialEq, Encode, Decode)]
struct NetworkPacket {
    seq: u32,
    src_port: u16,
    dst_port: u16,
    payload: Vec<u8>,
}

#[derive(Debug, PartialEq, Encode, Decode)]
enum PacketType {
    Data,
    Ack { seq: u32 },
    Reset,
    Fin { code: u8 },
}

// ---------------------------------------------------------------------------
// Test 1: NetworkPacket roundtrip – small payload
// ---------------------------------------------------------------------------
#[test]
fn test_network_packet_small_payload_roundtrip() {
    let pkt = NetworkPacket {
        seq: 1,
        src_port: 8080,
        dst_port: 443,
        payload: vec![0x48, 0x65, 0x6c, 0x6c, 0x6f],
    };
    let encoded = encode_with_checksum(&pkt).expect("encode NetworkPacket small failed");
    let (decoded, _): (NetworkPacket, usize) =
        decode_with_checksum(&encoded).expect("decode NetworkPacket small failed");
    assert_eq!(pkt, decoded);
}

// ---------------------------------------------------------------------------
// Test 2: NetworkPacket roundtrip – binary payload with all byte values
// ---------------------------------------------------------------------------
#[test]
fn test_network_packet_all_bytes_payload_roundtrip() {
    let pkt = NetworkPacket {
        seq: 2,
        src_port: 1024,
        dst_port: 65535,
        payload: (0u8..=255).collect(),
    };
    let encoded = encode_with_checksum(&pkt).expect("encode NetworkPacket all-bytes failed");
    let (decoded, _): (NetworkPacket, usize) =
        decode_with_checksum(&encoded).expect("decode NetworkPacket all-bytes failed");
    assert_eq!(pkt, decoded);
}

// ---------------------------------------------------------------------------
// Test 3: NetworkPacket roundtrip – zero seq and high port numbers
// ---------------------------------------------------------------------------
#[test]
fn test_network_packet_zero_seq_roundtrip() {
    let pkt = NetworkPacket {
        seq: 0,
        src_port: 65534,
        dst_port: 65535,
        payload: vec![0x00, 0xFF, 0xAA, 0x55],
    };
    let encoded = encode_with_checksum(&pkt).expect("encode NetworkPacket zero-seq failed");
    let (decoded, _): (NetworkPacket, usize) =
        decode_with_checksum(&encoded).expect("decode NetworkPacket zero-seq failed");
    assert_eq!(pkt, decoded);
}

// ---------------------------------------------------------------------------
// Test 4: NetworkPacket roundtrip – max seq u32
// ---------------------------------------------------------------------------
#[test]
fn test_network_packet_max_seq_roundtrip() {
    let pkt = NetworkPacket {
        seq: u32::MAX,
        src_port: 0,
        dst_port: 0,
        payload: vec![1, 2, 3],
    };
    let encoded = encode_with_checksum(&pkt).expect("encode NetworkPacket max-seq failed");
    let (decoded, _): (NetworkPacket, usize) =
        decode_with_checksum(&encoded).expect("decode NetworkPacket max-seq failed");
    assert_eq!(pkt, decoded);
}

// ---------------------------------------------------------------------------
// Test 5: PacketType::Data and PacketType::Reset roundtrip
// ---------------------------------------------------------------------------
#[test]
fn test_packet_type_unit_variants_roundtrip() {
    for variant in [PacketType::Data, PacketType::Reset] {
        let encoded = encode_with_checksum(&variant).expect("encode PacketType unit failed");
        let (decoded, _): (PacketType, usize) =
            decode_with_checksum(&encoded).expect("decode PacketType unit failed");
        assert_eq!(variant, decoded);
    }
}

// ---------------------------------------------------------------------------
// Test 6: PacketType::Ack and PacketType::Fin roundtrip
// ---------------------------------------------------------------------------
#[test]
fn test_packet_type_payload_variants_roundtrip() {
    let ack = PacketType::Ack { seq: 0xDEAD_BEEF };
    let fin = PacketType::Fin { code: 42 };

    let enc_ack = encode_with_checksum(&ack).expect("encode PacketType::Ack failed");
    let (dec_ack, _): (PacketType, usize) =
        decode_with_checksum(&enc_ack).expect("decode PacketType::Ack failed");
    assert_eq!(ack, dec_ack);

    let enc_fin = encode_with_checksum(&fin).expect("encode PacketType::Fin failed");
    let (dec_fin, _): (PacketType, usize) =
        decode_with_checksum(&enc_fin).expect("decode PacketType::Fin failed");
    assert_eq!(fin, dec_fin);
}

// ---------------------------------------------------------------------------
// Test 7: HEADER_SIZE is greater than zero
// ---------------------------------------------------------------------------
#[test]
fn test_header_size_greater_than_zero() {
    assert!(
        HEADER_SIZE > 0,
        "HEADER_SIZE must be > 0, but got {}",
        HEADER_SIZE
    );
}

// ---------------------------------------------------------------------------
// Test 8: Checksum output is larger than raw encode output
// ---------------------------------------------------------------------------
#[test]
fn test_checksum_output_larger_than_raw() {
    let pkt = NetworkPacket {
        seq: 10,
        src_port: 9000,
        dst_port: 9001,
        payload: vec![0xAB, 0xCD],
    };
    let raw = encode_to_vec(&pkt).expect("raw encode failed");
    let checksummed = encode_with_checksum(&pkt).expect("checksum encode failed");
    assert!(
        checksummed.len() > raw.len(),
        "checksummed output ({}) must be larger than raw ({})",
        checksummed.len(),
        raw.len()
    );
}

// ---------------------------------------------------------------------------
// Test 9: Corrupted byte in payload returns an error
// ---------------------------------------------------------------------------
#[test]
fn test_corrupted_payload_returns_error() {
    let pkt = NetworkPacket {
        seq: 99,
        src_port: 80,
        dst_port: 8080,
        payload: vec![1, 2, 3, 4, 5],
    };
    let mut encoded = encode_with_checksum(&pkt).expect("encode for corruption test failed");
    // flip a byte inside the payload (past the HEADER_SIZE prefix)
    encoded[HEADER_SIZE] ^= 0xFF;
    let result: Result<(NetworkPacket, usize), _> = decode_with_checksum(&encoded);
    assert!(
        result.is_err(),
        "corrupted payload must produce a decode error"
    );
}

// ---------------------------------------------------------------------------
// Test 10: u32 primitive checksum roundtrip
// ---------------------------------------------------------------------------
#[test]
fn test_primitive_u32_checksum_roundtrip() {
    let value: u32 = 0xFEED_FACE;
    let encoded = encode_with_checksum(&value).expect("encode u32 failed");
    let (decoded, _): (u32, usize) = decode_with_checksum(&encoded).expect("decode u32 failed");
    assert_eq!(value, decoded);
}

// ---------------------------------------------------------------------------
// Test 11: String primitive checksum roundtrip
// ---------------------------------------------------------------------------
#[test]
fn test_primitive_string_checksum_roundtrip() {
    let value = "OxiCode network checksum test 🚀".to_string();
    let encoded = encode_with_checksum(&value).expect("encode String failed");
    let (decoded, _): (String, usize) =
        decode_with_checksum(&encoded).expect("decode String failed");
    assert_eq!(value, decoded);
}

// ---------------------------------------------------------------------------
// Test 12: bool primitive checksum roundtrip (both values)
// ---------------------------------------------------------------------------
#[test]
fn test_primitive_bool_checksum_roundtrip() {
    for val in [true, false] {
        let encoded = encode_with_checksum(&val).expect("encode bool failed");
        let (decoded, _): (bool, usize) =
            decode_with_checksum(&encoded).expect("decode bool failed");
        assert_eq!(val, decoded, "bool roundtrip failed for {val}");
    }
}

// ---------------------------------------------------------------------------
// Test 13: Vec<NetworkPacket> roundtrip with checksum
// ---------------------------------------------------------------------------
#[test]
fn test_vec_network_packet_checksum_roundtrip() {
    let packets = vec![
        NetworkPacket {
            seq: 1,
            src_port: 100,
            dst_port: 200,
            payload: vec![0x01],
        },
        NetworkPacket {
            seq: 2,
            src_port: 300,
            dst_port: 400,
            payload: vec![0x02, 0x03],
        },
        NetworkPacket {
            seq: 3,
            src_port: 500,
            dst_port: 600,
            payload: vec![],
        },
    ];
    let encoded = encode_with_checksum(&packets).expect("encode Vec<NetworkPacket> failed");
    let (decoded, _): (Vec<NetworkPacket>, usize) =
        decode_with_checksum(&encoded).expect("decode Vec<NetworkPacket> failed");
    assert_eq!(packets, decoded);
}

// ---------------------------------------------------------------------------
// Test 14: NetworkPacket with empty payload roundtrip
// ---------------------------------------------------------------------------
#[test]
fn test_network_packet_empty_payload_roundtrip() {
    let pkt = NetworkPacket {
        seq: 0,
        src_port: 0,
        dst_port: 0,
        payload: vec![],
    };
    let encoded = encode_with_checksum(&pkt).expect("encode NetworkPacket empty payload failed");
    let (decoded, _): (NetworkPacket, usize) =
        decode_with_checksum(&encoded).expect("decode NetworkPacket empty payload failed");
    assert_eq!(pkt, decoded);
}

// ---------------------------------------------------------------------------
// Test 15: Large payload NetworkPacket roundtrip
// ---------------------------------------------------------------------------
#[test]
fn test_network_packet_large_payload_roundtrip() {
    let pkt = NetworkPacket {
        seq: 1_000_000,
        src_port: 12345,
        dst_port: 54321,
        payload: (0u8..=255).cycle().take(65536).collect(),
    };
    let encoded = encode_with_checksum(&pkt).expect("encode large NetworkPacket failed");
    let (decoded, consumed): (NetworkPacket, usize) =
        decode_with_checksum(&encoded).expect("decode large NetworkPacket failed");
    assert_eq!(pkt, decoded);
    assert_eq!(
        consumed,
        encoded.len(),
        "consumed bytes must equal total encoded length for large payload"
    );
}

// ---------------------------------------------------------------------------
// Test 16: Consumed bytes equals total encoded length
// ---------------------------------------------------------------------------
#[test]
fn test_consumed_bytes_equals_encoded_length() {
    let pkt = NetworkPacket {
        seq: 77,
        src_port: 4040,
        dst_port: 5050,
        payload: vec![0xDE, 0xAD, 0xBE, 0xEF],
    };
    let encoded = encode_with_checksum(&pkt).expect("encode for consumed-bytes test failed");
    let (_, consumed): (NetworkPacket, usize) =
        decode_with_checksum(&encoded).expect("decode for consumed-bytes test failed");
    assert_eq!(
        consumed,
        encoded.len(),
        "consumed ({consumed}) must equal encoded length ({})",
        encoded.len()
    );
}

// ---------------------------------------------------------------------------
// Test 17: Option<NetworkPacket> Some roundtrip
// ---------------------------------------------------------------------------
#[test]
fn test_option_some_network_packet_roundtrip() {
    let value: Option<NetworkPacket> = Some(NetworkPacket {
        seq: 42,
        src_port: 7777,
        dst_port: 8888,
        payload: vec![0x11, 0x22, 0x33],
    });
    let encoded = encode_with_checksum(&value).expect("encode Option<NetworkPacket> Some failed");
    let (decoded, _): (Option<NetworkPacket>, usize) =
        decode_with_checksum(&encoded).expect("decode Option<NetworkPacket> Some failed");
    assert_eq!(value, decoded);
}

// ---------------------------------------------------------------------------
// Test 18: Option<NetworkPacket> None roundtrip
// ---------------------------------------------------------------------------
#[test]
fn test_option_none_network_packet_roundtrip() {
    let value: Option<NetworkPacket> = None;
    let encoded = encode_with_checksum(&value).expect("encode Option<NetworkPacket> None failed");
    let (decoded, _): (Option<NetworkPacket>, usize) =
        decode_with_checksum(&encoded).expect("decode Option<NetworkPacket> None failed");
    assert_eq!(value, decoded);
}

// ---------------------------------------------------------------------------
// Test 19: Same data encoded twice produces identical checksum bytes (deterministic)
// ---------------------------------------------------------------------------
#[test]
fn test_checksum_encoding_is_deterministic() {
    let pkt = NetworkPacket {
        seq: 123,
        src_port: 2000,
        dst_port: 3000,
        payload: vec![0xAA, 0xBB, 0xCC],
    };
    let first = encode_with_checksum(&pkt).expect("first encode failed");
    let second = encode_with_checksum(&pkt).expect("second encode failed");
    assert_eq!(
        first, second,
        "encoding the same NetworkPacket twice must produce identical bytes"
    );
}

// ---------------------------------------------------------------------------
// Test 20: Different data produces different checksum bytes
// ---------------------------------------------------------------------------
#[test]
fn test_different_data_produces_different_checksum() {
    let pkt_a = NetworkPacket {
        seq: 1,
        src_port: 100,
        dst_port: 200,
        payload: vec![0x01],
    };
    let pkt_b = NetworkPacket {
        seq: 2,
        src_port: 100,
        dst_port: 200,
        payload: vec![0x02],
    };
    let enc_a = encode_with_checksum(&pkt_a).expect("encode pkt_a failed");
    let enc_b = encode_with_checksum(&pkt_b).expect("encode pkt_b failed");
    assert_ne!(
        enc_a, enc_b,
        "different NetworkPackets must produce different encoded bytes"
    );
}

// ---------------------------------------------------------------------------
// Test 21: Vec<PacketType> with all variants roundtrip
// ---------------------------------------------------------------------------
#[test]
fn test_vec_packet_type_all_variants_roundtrip() {
    let variants = vec![
        PacketType::Data,
        PacketType::Ack { seq: 1 },
        PacketType::Reset,
        PacketType::Fin { code: 0 },
        PacketType::Ack { seq: u32::MAX },
        PacketType::Fin { code: 255 },
    ];
    let encoded = encode_with_checksum(&variants).expect("encode Vec<PacketType> failed");
    let (decoded, _): (Vec<PacketType>, usize) =
        decode_with_checksum(&encoded).expect("decode Vec<PacketType> failed");
    assert_eq!(variants, decoded);
}

// ---------------------------------------------------------------------------
// Test 22: u128 max value checksum roundtrip
// ---------------------------------------------------------------------------
#[test]
fn test_u128_max_checksum_roundtrip() {
    let value: u128 = u128::MAX;
    let encoded = encode_with_checksum(&value).expect("encode u128::MAX failed");
    let (decoded, _): (u128, usize) =
        decode_with_checksum(&encoded).expect("decode u128::MAX failed");
    assert_eq!(value, decoded);
}
