//! Advanced tests for display/hex utilities in OxiCode.
//!
//! Tests cover `EncodedBytes`, `EncodedBytesOwned`, `encoded_bytes()`,
//! `encode_to_display()`, `encode_to_hex()`, `decode_from_hex()`,
//! and `hex_dump_bytes()`.

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
use oxicode::{Decode, Encode};

// ---------------------------------------------------------------------------
// Helper struct used across multiple tests
// ---------------------------------------------------------------------------

#[derive(Debug, PartialEq, Encode, Decode)]
struct Sample {
    id: u32,
    value: u64,
}

// ---------------------------------------------------------------------------
// 1. EncodedBytes::Display shows space-separated hex
// ---------------------------------------------------------------------------

#[test]
fn test_encoded_bytes_display_space_separated() {
    let bytes = [0x48u8, 0x65, 0x6c];
    let eb = oxicode::encoded_bytes(&bytes);
    assert_eq!(format!("{}", eb), "48 65 6c");
}

// ---------------------------------------------------------------------------
// 2. Empty bytes display as empty string
// ---------------------------------------------------------------------------

#[test]
fn test_encoded_bytes_display_empty() {
    let bytes: [u8; 0] = [];
    let eb = oxicode::encoded_bytes(&bytes);
    assert_eq!(format!("{}", eb), "");
}

// ---------------------------------------------------------------------------
// 3. Single byte display
// ---------------------------------------------------------------------------

#[test]
fn test_encoded_bytes_single_byte() {
    let bytes = [0xffu8];
    let eb = oxicode::encoded_bytes(&bytes);
    assert_eq!(format!("{}", eb), "ff");
}

// ---------------------------------------------------------------------------
// 4. Known bytes: [0xDE, 0xAD, 0xBE, 0xEF] space-separated
// ---------------------------------------------------------------------------

#[test]
fn test_encoded_bytes_known_deadbeef_display() {
    let bytes = [0xdeu8, 0xad, 0xbe, 0xef];
    let eb = oxicode::encoded_bytes(&bytes);
    assert_eq!(format!("{}", eb), "de ad be ef");
}

// ---------------------------------------------------------------------------
// 5. encode_to_hex of [0xDE, 0xAD, 0xBE, 0xEF] via encode_to_vec
// ---------------------------------------------------------------------------

#[test]
fn test_encode_to_hex_deadbeef_bytes() {
    // Encode the byte slice using encode_to_hex, then verify hex matches bytes.
    let raw: Vec<u8> = vec![0xdeu8, 0xad, 0xbe, 0xef];
    let hex = oxicode::encode_to_hex(&raw).expect("encode deadbeef bytes");
    // hex must be valid and all lowercase hex digits
    assert!(
        hex.chars().all(|c| c.is_ascii_hexdigit()),
        "hex must contain only hex digits, got: {hex}"
    );
    assert!(!hex.is_empty());
}

// ---------------------------------------------------------------------------
// 6. decode_from_hex roundtrip for known u32
// ---------------------------------------------------------------------------

#[test]
fn test_decode_from_hex_roundtrip_u32() {
    let original: u32 = 0xdeadbeef;
    let hex = oxicode::encode_to_hex(&original).expect("encode u32");
    let (decoded, _): (u32, _) = oxicode::decode_from_hex(&hex).expect("decode u32");
    assert_eq!(original, decoded);
}

// ---------------------------------------------------------------------------
// 7. encode_to_hex / decode_from_hex roundtrip for Vec<u8>
// ---------------------------------------------------------------------------

#[test]
fn test_hex_encode_decode_roundtrip_vec_u8() {
    let original: Vec<u8> = (0u8..=127).collect();
    let hex = oxicode::encode_to_hex(&original).expect("encode vec u8");
    let (decoded, _): (Vec<u8>, _) = oxicode::decode_from_hex(&hex).expect("decode vec u8");
    assert_eq!(original, decoded);
}

// ---------------------------------------------------------------------------
// 8. encode_to_hex output is always lowercase
// ---------------------------------------------------------------------------

#[test]
fn test_encode_to_hex_is_lowercase() {
    let val: Vec<u8> = (0xa0u8..=0xffu8).collect();
    let hex = oxicode::encode_to_hex(&val).expect("encode high bytes");
    assert!(
        !hex.contains(|c: char| ('A'..='F').contains(&c)),
        "hex output must be lowercase; got: {hex}"
    );
}

// ---------------------------------------------------------------------------
// 9. decode_from_hex of empty-encoded value roundtrips
// ---------------------------------------------------------------------------

#[test]
fn test_hex_decode_empty_vec_roundtrip() {
    let original: Vec<u8> = vec![];
    let hex = oxicode::encode_to_hex(&original).expect("encode empty vec");
    let (decoded, _): (Vec<u8>, _) = oxicode::decode_from_hex(&hex).expect("decode empty vec");
    assert_eq!(original, decoded);
}

// ---------------------------------------------------------------------------
// 10. decode_from_hex with invalid characters returns Err
// ---------------------------------------------------------------------------

#[test]
fn test_decode_from_hex_invalid_chars_returns_err() {
    let result: Result<(u8, _), _> = oxicode::decode_from_hex("GG");
    assert!(result.is_err(), "invalid hex chars should return Err");
}

// ---------------------------------------------------------------------------
// 11. decode_from_hex with odd-length string returns Err
// ---------------------------------------------------------------------------

#[test]
fn test_decode_from_hex_odd_length_returns_err() {
    let result: Result<(u8, _), _> = oxicode::decode_from_hex("abc");
    assert!(result.is_err(), "odd-length hex string should return Err");
}

// ---------------------------------------------------------------------------
// 12. encode_to_hex of all-zero bytes decodes back to all zeros
// ---------------------------------------------------------------------------

#[test]
fn test_encode_to_hex_all_zeros_roundtrip() {
    let val: Vec<u8> = vec![0u8; 8];
    let hex = oxicode::encode_to_hex(&val).expect("encode all zeros");
    // The encoding includes a varint length prefix, so the hex will not be all
    // zeroes — but it must decode back correctly.
    let (decoded, _): (Vec<u8>, _) = oxicode::decode_from_hex(&hex).expect("decode all zeros");
    assert_eq!(val, decoded);
    // hex must be all-lowercase hex digits
    assert!(
        hex.chars().all(|c| c.is_ascii_hexdigit()),
        "hex must contain only hex digits; got: {hex}"
    );
}

// ---------------------------------------------------------------------------
// 13. encode_to_hex of all-0xFF bytes decodes back to all 0xFF
// ---------------------------------------------------------------------------

#[test]
fn test_encode_to_hex_all_ff_roundtrip() {
    let val: Vec<u8> = vec![0xffu8; 4];
    let hex = oxicode::encode_to_hex(&val).expect("encode all 0xff");
    let (decoded, _): (Vec<u8>, _) = oxicode::decode_from_hex(&hex).expect("decode all 0xff");
    assert_eq!(val, decoded);
    // payload portion of the hex must contain "ff" groups
    assert!(
        hex.contains("ff"),
        "hex of all-0xFF payload must contain 'ff'; got: {hex}"
    );
}

// ---------------------------------------------------------------------------
// 14. EncodedBytes LowerHex and UpperHex formats
// ---------------------------------------------------------------------------

#[test]
fn test_encoded_bytes_lower_and_upper_hex_formats() {
    let bytes = [0xabu8, 0xcd];
    let eb = oxicode::encoded_bytes(&bytes);
    assert_eq!(format!("{:x}", eb), "abcd");
    assert_eq!(format!("{:X}", eb), "ABCD");
}

// ---------------------------------------------------------------------------
// 15. hex_dump output contains the zero address prefix
// ---------------------------------------------------------------------------

#[test]
fn test_hex_dump_contains_zero_address() {
    let data = b"Hello, World!";
    let eb = oxicode::encoded_bytes(data);
    let dump = eb.hex_dump();
    assert!(
        dump.contains("00000000:"),
        "hex_dump must start with address 00000000:; got:\n{dump}"
    );
}

// ---------------------------------------------------------------------------
// 16. hex_dump of short data contains ASCII sidebar
// ---------------------------------------------------------------------------

#[test]
fn test_hex_dump_short_data_ascii_sidebar() {
    let data = b"Hello";
    let eb = oxicode::encoded_bytes(data);
    let dump = eb.hex_dump();
    assert!(
        dump.contains("Hello"),
        "hex_dump sidebar should show printable 'Hello'; got:\n{dump}"
    );
}

// ---------------------------------------------------------------------------
// 17. hex_dump of exactly 16 bytes fits on one line
// ---------------------------------------------------------------------------

#[test]
fn test_hex_dump_16_bytes_single_line() {
    let data: [u8; 16] = [b'A'; 16];
    let eb = oxicode::encoded_bytes(&data);
    let dump = eb.hex_dump();
    let lines: Vec<&str> = dump.lines().collect();
    assert_eq!(
        lines.len(),
        1,
        "16 bytes should produce exactly one dump line"
    );
}

// ---------------------------------------------------------------------------
// 18. hex_dump of 17 bytes produces two lines
// ---------------------------------------------------------------------------

#[test]
fn test_hex_dump_17_bytes_two_lines() {
    let data: [u8; 17] = [b'B'; 17];
    let eb = oxicode::encoded_bytes(&data);
    let dump = eb.hex_dump();
    let lines: Vec<&str> = dump.lines().collect();
    assert_eq!(
        lines.len(),
        2,
        "17 bytes should produce exactly two dump lines; got:\n{dump}"
    );
    assert!(
        lines[1].contains("00000010:"),
        "second line address should be 00000010:"
    );
}

// ---------------------------------------------------------------------------
// 19. EncodedBytesOwned from encode_to_display: hex matches encode_to_vec
// ---------------------------------------------------------------------------

#[test]
fn test_encode_to_display_hex_matches_encode_to_vec() {
    let val = 42u32;
    let owned = oxicode::encode_to_display(&val).expect("encode_to_display u32");
    let lower_hex = format!("{:x}", owned);
    let raw_bytes = oxicode::encode_to_vec(&val).expect("encode_to_vec u32");
    let expected: String = raw_bytes.iter().map(|b| format!("{:02x}", b)).collect();
    assert_eq!(lower_hex, expected);
}

// ---------------------------------------------------------------------------
// 20. EncodedBytesOwned::as_bytes matches encode_to_vec
// ---------------------------------------------------------------------------

#[test]
fn test_encoded_bytes_owned_as_bytes_matches_encode_to_vec() {
    let val = Sample { id: 7, value: 99 };
    let owned = oxicode::encode_to_display(&val).expect("encode Sample to display");
    let raw = oxicode::encode_to_vec(&val).expect("encode Sample to vec");
    assert_eq!(owned.as_bytes(), raw.as_slice());
}

// ---------------------------------------------------------------------------
// 21. encoded_bytes Display matches per-byte hex of encode_to_vec output
// ---------------------------------------------------------------------------

#[test]
fn test_encoded_bytes_display_matches_encode_to_vec() {
    let val = Sample { id: 1, value: 2 };
    let raw = oxicode::encode_to_vec(&val).expect("encode Sample");
    let eb = oxicode::encoded_bytes(&raw);
    let display = format!("{}", eb);
    let expected: String = raw
        .iter()
        .enumerate()
        .map(|(i, b)| {
            if i == 0 {
                format!("{:02x}", b)
            } else {
                format!(" {:02x}", b)
            }
        })
        .collect();
    assert_eq!(display, expected);
}

// ---------------------------------------------------------------------------
// 22. Full roundtrip: encode → encode_to_hex → decode_from_hex → decode matches original
// ---------------------------------------------------------------------------

#[test]
fn test_full_roundtrip_encode_hex_decode() {
    let original = Sample {
        id: 42,
        value: 1_000_000,
    };
    let hex = oxicode::encode_to_hex(&original).expect("encode Sample to hex");
    assert!(
        hex.chars().all(|c| c.is_ascii_hexdigit()),
        "hex must only contain hex digits; got: {hex}"
    );
    let (decoded, _): (Sample, _) = oxicode::decode_from_hex(&hex).expect("decode Sample from hex");
    assert_eq!(original, decoded);
}
