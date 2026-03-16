//! Advanced checksum/CRC32 tests for OxiCode — exactly 22 top-level #[test] functions.
//!
//! Compile with: cargo test --features checksum --test checksum_advanced7_test

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
use oxicode::encode_to_vec;

// ---------------------------------------------------------------------------
// Shared helper types
// ---------------------------------------------------------------------------

#[derive(Debug, PartialEq, oxicode::Encode, oxicode::Decode)]
struct Packet {
    seq: u32,
    data: Vec<u8>,
    flags: u8,
}

#[derive(Debug, PartialEq, oxicode::Encode, oxicode::Decode)]
enum Command {
    Ping,
    Pong,
    Data(Vec<u8>),
    Connect { host: String, port: u16 },
}

// ---------------------------------------------------------------------------
// Test 1: Packet roundtrip
// ---------------------------------------------------------------------------
#[test]
fn test_packet_roundtrip() {
    let value = Packet {
        seq: 42,
        data: vec![1, 2, 3, 4, 5],
        flags: 0b0000_0011,
    };
    let encoded = encode_with_checksum(&value).expect("encode Packet failed");
    let (decoded, consumed): (Packet, _) =
        decode_with_checksum(&encoded).expect("decode Packet failed");
    assert_eq!(decoded, value, "decoded Packet must equal original");
    assert_eq!(
        consumed,
        encoded.len(),
        "consumed must equal total encoded length"
    );
}

// ---------------------------------------------------------------------------
// Test 2: Command::Ping roundtrip
// ---------------------------------------------------------------------------
#[test]
fn test_command_ping_roundtrip() {
    let value = Command::Ping;
    let encoded = encode_with_checksum(&value).expect("encode Command::Ping failed");
    let (decoded, consumed): (Command, _) =
        decode_with_checksum(&encoded).expect("decode Command::Ping failed");
    assert_eq!(decoded, value, "decoded Command::Ping must equal original");
    assert_eq!(
        consumed,
        encoded.len(),
        "consumed must equal total encoded length"
    );
}

// ---------------------------------------------------------------------------
// Test 3: Command::Data roundtrip
// ---------------------------------------------------------------------------
#[test]
fn test_command_data_roundtrip() {
    let value = Command::Data(vec![0xDE, 0xAD, 0xBE, 0xEF, 0xCA, 0xFE]);
    let encoded = encode_with_checksum(&value).expect("encode Command::Data failed");
    let (decoded, consumed): (Command, _) =
        decode_with_checksum(&encoded).expect("decode Command::Data failed");
    assert_eq!(decoded, value, "decoded Command::Data must equal original");
    assert_eq!(
        consumed,
        encoded.len(),
        "consumed must equal total encoded length"
    );
}

// ---------------------------------------------------------------------------
// Test 4: Command::Connect roundtrip
// ---------------------------------------------------------------------------
#[test]
fn test_command_connect_roundtrip() {
    let value = Command::Connect {
        host: String::from("example.com"),
        port: 8080,
    };
    let encoded = encode_with_checksum(&value).expect("encode Command::Connect failed");
    let (decoded, consumed): (Command, _) =
        decode_with_checksum(&encoded).expect("decode Command::Connect failed");
    assert_eq!(
        decoded, value,
        "decoded Command::Connect must equal original"
    );
    assert_eq!(
        consumed,
        encoded.len(),
        "consumed must equal total encoded length"
    );
}

// ---------------------------------------------------------------------------
// Test 5: Vec<Packet> roundtrip
// ---------------------------------------------------------------------------
#[test]
fn test_vec_packet_roundtrip() {
    let value: Vec<Packet> = vec![
        Packet {
            seq: 0,
            data: vec![10, 20],
            flags: 0x00,
        },
        Packet {
            seq: 1,
            data: vec![30, 40, 50],
            flags: 0x01,
        },
        Packet {
            seq: 2,
            data: Vec::new(),
            flags: 0xFF,
        },
    ];
    let encoded = encode_with_checksum(&value).expect("encode Vec<Packet> failed");
    let (decoded, consumed): (Vec<Packet>, _) =
        decode_with_checksum(&encoded).expect("decode Vec<Packet> failed");
    assert_eq!(decoded, value, "decoded Vec<Packet> must equal original");
    assert_eq!(
        consumed,
        encoded.len(),
        "consumed must equal total encoded length"
    );
}

// ---------------------------------------------------------------------------
// Test 6: u32 basic type checksum roundtrip
// ---------------------------------------------------------------------------
#[test]
fn test_u32_basic_checksum_roundtrip() {
    let value: u32 = 0xDEAD_BEEF;
    let encoded = encode_with_checksum(&value).expect("encode u32 failed");
    let (decoded, consumed): (u32, _) = decode_with_checksum(&encoded).expect("decode u32 failed");
    assert_eq!(decoded, value, "decoded u32 must equal original");
    assert_eq!(
        consumed,
        encoded.len(),
        "consumed must equal total encoded length"
    );
}

// ---------------------------------------------------------------------------
// Test 7: u64 basic type checksum roundtrip
// ---------------------------------------------------------------------------
#[test]
fn test_u64_basic_checksum_roundtrip() {
    let value: u64 = u64::MAX / 7;
    let encoded = encode_with_checksum(&value).expect("encode u64 failed");
    let (decoded, consumed): (u64, _) = decode_with_checksum(&encoded).expect("decode u64 failed");
    assert_eq!(decoded, value, "decoded u64 must equal original");
    assert_eq!(
        consumed,
        encoded.len(),
        "consumed must equal total encoded length"
    );
}

// ---------------------------------------------------------------------------
// Test 8: String basic type checksum roundtrip
// ---------------------------------------------------------------------------
#[test]
fn test_string_basic_checksum_roundtrip() {
    let value = String::from("OxiCode checksum string test");
    let encoded = encode_with_checksum(&value).expect("encode String failed");
    let (decoded, consumed): (String, _) =
        decode_with_checksum(&encoded).expect("decode String failed");
    assert_eq!(decoded, value, "decoded String must equal original");
    assert_eq!(
        consumed,
        encoded.len(),
        "consumed must equal total encoded length"
    );
}

// ---------------------------------------------------------------------------
// Test 9: bool basic type checksum roundtrip
// ---------------------------------------------------------------------------
#[test]
fn test_bool_basic_checksum_roundtrip() {
    let value_true: bool = true;
    let encoded = encode_with_checksum(&value_true).expect("encode bool failed");
    let (decoded, _): (bool, _) = decode_with_checksum(&encoded).expect("decode bool failed");
    assert_eq!(decoded, value_true, "decoded bool must equal original");

    let value_false: bool = false;
    let encoded_false = encode_with_checksum(&value_false).expect("encode bool false failed");
    let (decoded_false, consumed_false): (bool, _) =
        decode_with_checksum(&encoded_false).expect("decode bool false failed");
    assert_eq!(
        decoded_false, value_false,
        "decoded bool false must equal original"
    );
    assert_eq!(
        consumed_false,
        encoded_false.len(),
        "consumed must equal total encoded length"
    );
}

// ---------------------------------------------------------------------------
// Test 10: Vec<u8> basic type checksum roundtrip
// ---------------------------------------------------------------------------
#[test]
fn test_vec_u8_basic_checksum_roundtrip() {
    let value: Vec<u8> = (0u8..=127).collect();
    let encoded = encode_with_checksum(&value).expect("encode Vec<u8> failed");
    let (decoded, consumed): (Vec<u8>, _) =
        decode_with_checksum(&encoded).expect("decode Vec<u8> failed");
    assert_eq!(decoded, value, "decoded Vec<u8> must equal original");
    assert_eq!(
        consumed,
        encoded.len(),
        "consumed must equal total encoded length"
    );
}

// ---------------------------------------------------------------------------
// Test 11: Option<Packet> Some checksum roundtrip
// ---------------------------------------------------------------------------
#[test]
fn test_option_packet_some_checksum_roundtrip() {
    let value: Option<Packet> = Some(Packet {
        seq: 7,
        data: vec![0xAB, 0xCD],
        flags: 0x05,
    });
    let encoded = encode_with_checksum(&value).expect("encode Option<Packet> Some failed");
    let (decoded, consumed): (Option<Packet>, _) =
        decode_with_checksum(&encoded).expect("decode Option<Packet> Some failed");
    assert_eq!(
        decoded, value,
        "decoded Option<Packet> Some must equal original"
    );
    assert!(decoded.is_some(), "decoded option must be Some");
    assert_eq!(
        consumed,
        encoded.len(),
        "consumed must equal total encoded length"
    );
}

// ---------------------------------------------------------------------------
// Test 12: Option<Packet> None checksum roundtrip
// ---------------------------------------------------------------------------
#[test]
fn test_option_packet_none_checksum_roundtrip() {
    let value: Option<Packet> = None;
    let encoded = encode_with_checksum(&value).expect("encode Option<Packet> None failed");
    let (decoded, consumed): (Option<Packet>, _) =
        decode_with_checksum(&encoded).expect("decode Option<Packet> None failed");
    assert_eq!(
        decoded, value,
        "decoded Option<Packet> None must equal original"
    );
    assert!(decoded.is_none(), "decoded option must be None");
    assert_eq!(
        consumed,
        encoded.len(),
        "consumed must equal total encoded length"
    );
}

// ---------------------------------------------------------------------------
// Test 13: Empty Packet (zero seq, empty data, zero flags) checksum roundtrip
// ---------------------------------------------------------------------------
#[test]
fn test_empty_packet_checksum_roundtrip() {
    let value = Packet {
        seq: 0,
        data: Vec::new(),
        flags: 0,
    };
    let encoded = encode_with_checksum(&value).expect("encode empty Packet failed");
    let (decoded, consumed): (Packet, _) =
        decode_with_checksum(&encoded).expect("decode empty Packet failed");
    assert_eq!(decoded, value, "decoded empty Packet must equal original");
    assert!(decoded.data.is_empty(), "decoded Packet data must be empty");
    assert_eq!(
        consumed,
        encoded.len(),
        "consumed must equal total encoded length"
    );
}

// ---------------------------------------------------------------------------
// Test 14: Large data — Packet with 8192 bytes of payload
// ---------------------------------------------------------------------------
#[test]
fn test_large_data_packet_8192_bytes_checksum_roundtrip() {
    let large_data: Vec<u8> = (0u8..=255).cycle().take(8192).collect();
    assert_eq!(
        large_data.len(),
        8192,
        "test data must be exactly 8192 bytes"
    );
    let value = Packet {
        seq: 9999,
        data: large_data,
        flags: 0b1111_1111,
    };
    let encoded = encode_with_checksum(&value).expect("encode large Packet failed");
    let (decoded, consumed): (Packet, _) =
        decode_with_checksum(&encoded).expect("decode large Packet failed");
    assert_eq!(decoded, value, "decoded large Packet must equal original");
    assert_eq!(
        decoded.data.len(),
        8192,
        "decoded data must have 8192 bytes"
    );
    assert_eq!(
        consumed,
        encoded.len(),
        "consumed must equal total encoded length"
    );
}

// ---------------------------------------------------------------------------
// Test 15: Corrupted checksum on Packet returns error
// ---------------------------------------------------------------------------
#[test]
fn test_corrupted_checksum_packet_returns_error() {
    let value = Packet {
        seq: 100,
        data: vec![1, 2, 3],
        flags: 0x01,
    };
    let mut encoded =
        encode_with_checksum(&value).expect("encode Packet for corruption test failed");
    // Flip a byte in the payload region (after the header)
    let flip_idx = HEADER_SIZE + 1;
    encoded[flip_idx] ^= 0xFF;
    let result = decode_with_checksum::<Packet>(&encoded);
    assert!(
        result.is_err(),
        "decode_with_checksum must return Err when Packet payload byte is corrupted"
    );
}

// ---------------------------------------------------------------------------
// Test 16: Unicode String checksum roundtrip
// ---------------------------------------------------------------------------
#[test]
fn test_unicode_string_checksum_roundtrip() {
    let value = String::from("日本語テスト: 🦀 Rust + OxiCode = 最高 ✓");
    let encoded = encode_with_checksum(&value).expect("encode unicode String failed");
    let (decoded, consumed): (String, _) =
        decode_with_checksum(&encoded).expect("decode unicode String failed");
    assert_eq!(decoded, value, "decoded unicode String must equal original");
    assert_eq!(
        consumed,
        encoded.len(),
        "consumed must equal total encoded length"
    );
}

// ---------------------------------------------------------------------------
// Test 17: HEADER_SIZE check — encoded checksum size difference equals HEADER_SIZE
// ---------------------------------------------------------------------------
#[test]
fn test_header_size_equals_size_difference() {
    let value = Packet {
        seq: 1,
        data: vec![0x01, 0x02],
        flags: 0x00,
    };
    let plain = encode_to_vec(&value).expect("plain encode failed");
    let checked = encode_with_checksum(&value).expect("checksum encode failed");
    assert!(
        checked.len() > plain.len(),
        "checksummed encoding ({} bytes) must be larger than plain ({} bytes)",
        checked.len(),
        plain.len()
    );
    assert_eq!(
        checked.len() - plain.len(),
        HEADER_SIZE,
        "difference must be exactly HEADER_SIZE ({}) bytes",
        HEADER_SIZE
    );
}

// ---------------------------------------------------------------------------
// Test 18: Size comparison — different Packets yield different encoded sizes
// ---------------------------------------------------------------------------
#[test]
fn test_size_comparison_different_packets() {
    let small = Packet {
        seq: 0,
        data: vec![1],
        flags: 0,
    };
    let large = Packet {
        seq: 0,
        data: vec![1; 100],
        flags: 0,
    };
    let encoded_small = encode_with_checksum(&small).expect("encode small Packet failed");
    let encoded_large = encode_with_checksum(&large).expect("encode large Packet failed");
    assert!(
        encoded_large.len() > encoded_small.len(),
        "larger Packet payload must produce larger checksummed encoding"
    );
}

// ---------------------------------------------------------------------------
// Test 19: Determinism — encoding the same Packet twice gives identical bytes
// ---------------------------------------------------------------------------
#[test]
fn test_determinism_same_packet_twice() {
    let value = Packet {
        seq: 55,
        data: vec![0xAA, 0xBB, 0xCC],
        flags: 0x07,
    };
    let first = encode_with_checksum(&value).expect("first encode failed");
    let second = encode_with_checksum(&value).expect("second encode failed");
    assert_eq!(
        first, second,
        "encoding the same Packet twice must produce identical checksummed bytes"
    );
}

// ---------------------------------------------------------------------------
// Test 20: Multiple Packets sequential — encode and decode independently
// ---------------------------------------------------------------------------
#[test]
fn test_multiple_packets_sequential_encode_decode() {
    let packets = [
        Packet {
            seq: 10,
            data: vec![0x01],
            flags: 0x01,
        },
        Packet {
            seq: 11,
            data: vec![0x02, 0x03],
            flags: 0x02,
        },
        Packet {
            seq: 12,
            data: vec![0x04, 0x05, 0x06],
            flags: 0x03,
        },
    ];

    for original in &packets {
        let encoded = encode_with_checksum(original).expect("encode sequential Packet failed");
        let (decoded, consumed): (Packet, _) =
            decode_with_checksum(&encoded).expect("decode sequential Packet failed");
        assert_eq!(
            &decoded, original,
            "decoded sequential Packet must equal original"
        );
        assert_eq!(
            consumed,
            encoded.len(),
            "consumed must equal total encoded length"
        );
    }
}

// ---------------------------------------------------------------------------
// Test 21: Consumed bytes matches encoded length for Command::Pong
// ---------------------------------------------------------------------------
#[test]
fn test_consumed_bytes_matches_encoded_length_pong() {
    let value = Command::Pong;
    let encoded = encode_with_checksum(&value).expect("encode Command::Pong failed");
    let (decoded, consumed): (Command, _) =
        decode_with_checksum(&encoded).expect("decode Command::Pong failed");
    assert_eq!(decoded, value, "decoded Command::Pong must equal original");
    assert_eq!(
        consumed,
        encoded.len(),
        "consumed ({}) must exactly equal encoded length ({})",
        consumed,
        encoded.len()
    );
}

// ---------------------------------------------------------------------------
// Test 22: Vec<Command> all variants roundtrip and checksum of same data twice
// ---------------------------------------------------------------------------
#[test]
fn test_vec_command_all_variants_and_determinism() {
    let value: Vec<Command> = vec![
        Command::Ping,
        Command::Pong,
        Command::Data(vec![0x11, 0x22, 0x33]),
        Command::Connect {
            host: String::from("localhost"),
            port: 9000,
        },
        Command::Ping,
        Command::Data(Vec::new()),
        Command::Connect {
            host: String::from("192.168.0.1"),
            port: 443,
        },
    ];
    let encoded_first = encode_with_checksum(&value).expect("first encode Vec<Command> failed");
    let encoded_second = encode_with_checksum(&value).expect("second encode Vec<Command> failed");
    assert_eq!(
        encoded_first, encoded_second,
        "checksum of same Vec<Command> data twice must give identical bytes"
    );
    let (decoded, consumed): (Vec<Command>, _) =
        decode_with_checksum(&encoded_first).expect("decode Vec<Command> failed");
    assert_eq!(decoded, value, "decoded Vec<Command> must equal original");
    assert_eq!(
        consumed,
        encoded_first.len(),
        "consumed must equal total encoded length"
    );
}
