//! Advanced display/hex tests — second batch (22 top-level test functions).
//!
//! Covers `EncodedBytes`, `EncodedBytesOwned`, `encode_to_hex`, `decode_from_hex`,
//! `hex_dump_bytes`, bool encoding, fixed-size arrays, versioned values, and more.

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
// Shared helper struct
// ---------------------------------------------------------------------------

#[derive(Debug, PartialEq, Encode, Decode)]
struct Pair {
    a: i32,
    b: i32,
}

// ---------------------------------------------------------------------------
// 1. EncodedBytes displays as space-separated hex string
// ---------------------------------------------------------------------------

#[test]
fn test_encoded_bytes_displays_as_hex_string() {
    let bytes = [0x0au8, 0x0b, 0x0c];
    let eb = oxicode::encoded_bytes(&bytes);
    let s = format!("{}", eb);
    assert_eq!(s, "0a 0b 0c");
}

// ---------------------------------------------------------------------------
// 2. encode_to_hex for a single u8 value produces valid hex
// ---------------------------------------------------------------------------

#[test]
fn test_encode_to_hex_single_u8() {
    let val: u8 = 200;
    let hex = oxicode::encode_to_hex(&val).expect("encode u8");
    assert!(
        hex.chars().all(|c| c.is_ascii_hexdigit()),
        "hex must only contain hex digits; got: {hex}"
    );
    assert!(!hex.is_empty(), "hex must not be empty");
}

// ---------------------------------------------------------------------------
// 3. encode_to_hex for a u32 value is consistent with encode_to_vec
// ---------------------------------------------------------------------------

#[test]
fn test_encode_to_hex_u32_matches_encode_to_vec() {
    let val: u32 = 0x0102_0304;
    let hex = oxicode::encode_to_hex(&val).expect("encode u32");
    let raw = oxicode::encode_to_vec(&val).expect("encode_to_vec u32");
    let expected: String = raw.iter().map(|b| format!("{:02x}", b)).collect();
    assert_eq!(hex, expected);
}

// ---------------------------------------------------------------------------
// 4. encode_to_hex for a String and verify roundtrip
// ---------------------------------------------------------------------------

#[test]
fn test_encode_to_hex_string_roundtrip() {
    let original = "oxicode display test".to_string();
    let hex = oxicode::encode_to_hex(&original).expect("encode String");
    let (decoded, _): (String, _) = oxicode::decode_from_hex(&hex).expect("decode String");
    assert_eq!(original, decoded);
}

// ---------------------------------------------------------------------------
// 5. decode_from_hex roundtrip for u32
// ---------------------------------------------------------------------------

#[test]
fn test_decode_from_hex_roundtrip_u32_value() {
    let original: u32 = 987_654_321;
    let hex = oxicode::encode_to_hex(&original).expect("encode u32");
    let (decoded, _): (u32, _) = oxicode::decode_from_hex(&hex).expect("decode u32");
    assert_eq!(original, decoded);
}

// ---------------------------------------------------------------------------
// 6. decode_from_hex roundtrip for String with special characters
// ---------------------------------------------------------------------------

#[test]
fn test_decode_from_hex_roundtrip_string_special_chars() {
    let original = "Hello\nWorld\t!".to_string();
    let hex = oxicode::encode_to_hex(&original).expect("encode special String");
    let (decoded, _): (String, _) = oxicode::decode_from_hex(&hex).expect("decode special String");
    assert_eq!(original, decoded);
}

// ---------------------------------------------------------------------------
// 7. encode_to_hex output is all lowercase (no A-F digits)
// ---------------------------------------------------------------------------

#[test]
fn test_hex_string_is_all_lowercase() {
    let val: Vec<u8> = vec![0xABu8, 0xCD, 0xEF];
    let hex = oxicode::encode_to_hex(&val).expect("encode vec");
    assert!(
        !hex.contains(|c: char| ('A'..='F').contains(&c)),
        "hex output must be all lowercase; got: {hex}"
    );
}

// ---------------------------------------------------------------------------
// 8. hex string length == 2 * byte count (via encoded_bytes LowerHex)
// ---------------------------------------------------------------------------

#[test]
fn test_hex_string_length_is_twice_byte_count() {
    let bytes: Vec<u8> = (0u8..16).collect();
    let eb = oxicode::encoded_bytes(&bytes);
    let compact_hex = format!("{:x}", eb);
    assert_eq!(
        compact_hex.len(),
        bytes.len() * 2,
        "compact hex must have length 2 * byte_count; got: {compact_hex}"
    );
}

// ---------------------------------------------------------------------------
// 9. Empty bytes display as empty string (Display format)
// ---------------------------------------------------------------------------

#[test]
fn test_empty_bytes_display_as_empty_hex() {
    let eb = oxicode::encoded_bytes(&[]);
    assert_eq!(format!("{}", eb), "");
    assert_eq!(format!("{:x}", eb), "");
    assert_eq!(format!("{:X}", eb), "");
}

// ---------------------------------------------------------------------------
// 10. Single byte [0xFF] displays as "ff" in Display and LowerHex
// ---------------------------------------------------------------------------

#[test]
fn test_single_byte_ff_displays_as_ff() {
    let eb = oxicode::encoded_bytes(&[0xFFu8]);
    assert_eq!(format!("{}", eb), "ff");
    assert_eq!(format!("{:x}", eb), "ff");
}

// ---------------------------------------------------------------------------
// 11. Known value [0x01, 0x02] LowerHex displays as "0102"
// ---------------------------------------------------------------------------

#[test]
fn test_known_bytes_0102_lower_hex_is_0102() {
    let eb = oxicode::encoded_bytes(&[0x01u8, 0x02]);
    assert_eq!(format!("{:x}", eb), "0102");
}

// ---------------------------------------------------------------------------
// 12. Hex encode then decode preserves value (Pair struct)
// ---------------------------------------------------------------------------

#[test]
fn test_hex_encode_then_decode_preserves_pair_struct() {
    let original = Pair { a: -42, b: 9999 };
    let hex = oxicode::encode_to_hex(&original).expect("encode Pair");
    let (decoded, _): (Pair, _) = oxicode::decode_from_hex(&hex).expect("decode Pair");
    assert_eq!(original, decoded);
}

// ---------------------------------------------------------------------------
// 13. Invalid hex string returns Err on decode
// ---------------------------------------------------------------------------

#[test]
fn test_invalid_hex_string_returns_error_on_decode() {
    let bad_hex = "not_a_hex_string!!";
    let result: Result<(u32, _), _> = oxicode::decode_from_hex(bad_hex);
    assert!(
        result.is_err(),
        "decoding invalid hex should return Err; input: {bad_hex}"
    );
}

// ---------------------------------------------------------------------------
// 14. Uppercase hex input decodes correctly (case insensitive)
// ---------------------------------------------------------------------------

#[test]
fn test_uppercase_hex_decodes_correctly() {
    let val: u64 = 0xDEAD_BEEF_CAFE_1234;
    let lower_hex = oxicode::encode_to_hex(&val).expect("encode u64");
    let upper_hex = lower_hex.to_uppercase();
    let (from_lower, _): (u64, _) =
        oxicode::decode_from_hex(&lower_hex).expect("decode from lower hex");
    let (from_upper, _): (u64, _) =
        oxicode::decode_from_hex(&upper_hex).expect("decode from upper hex");
    assert_eq!(from_lower, from_upper);
    assert_eq!(val, from_lower);
}

// ---------------------------------------------------------------------------
// 15. Hex encode and decode for Vec<u8> (longer slice)
// ---------------------------------------------------------------------------

#[test]
fn test_hex_encode_decode_for_vec_u8_longer_slice() {
    let original: Vec<u8> = (0u8..=255).collect();
    let hex = oxicode::encode_to_hex(&original).expect("encode full-byte Vec<u8>");
    let (decoded, _): (Vec<u8>, _) =
        oxicode::decode_from_hex(&hex).expect("decode full-byte Vec<u8>");
    assert_eq!(original, decoded);
}

// ---------------------------------------------------------------------------
// 16. EncodedBytesOwned struct roundtrip via encode_to_display
// ---------------------------------------------------------------------------

#[test]
fn test_encoded_bytes_owned_struct_roundtrip() {
    let val = Pair { a: 1, b: 2 };
    let owned = oxicode::encode_to_display(&val).expect("encode_to_display Pair");
    let raw = oxicode::encode_to_vec(&val).expect("encode_to_vec Pair");
    // as_bytes must match encode_to_vec output
    assert_eq!(owned.as_bytes(), raw.as_slice());
    // LowerHex must match the expected compact hex
    let expected_hex: String = raw.iter().map(|b| format!("{:02x}", b)).collect();
    assert_eq!(format!("{:x}", owned), expected_hex);
}

// ---------------------------------------------------------------------------
// 17. Display formatting for various sizes produces correct byte counts
// ---------------------------------------------------------------------------

#[test]
fn test_display_formatting_for_various_sizes() {
    for len in [0usize, 1, 7, 15, 16, 17, 31, 32] {
        let bytes: Vec<u8> = (0u8..).take(len).collect();
        let eb = oxicode::encoded_bytes(&bytes);
        let display = format!("{}", eb);
        if len == 0 {
            assert!(
                display.is_empty(),
                "empty bytes must give empty Display; len={len}"
            );
        } else {
            // Space-separated: len tokens, each 2 chars, separated by spaces
            let tokens: Vec<&str> = display.split(' ').collect();
            assert_eq!(
                tokens.len(),
                len,
                "Display must have {len} space-separated tokens; got {}: '{display}'",
                tokens.len()
            );
            for token in &tokens {
                assert_eq!(
                    token.len(),
                    2,
                    "each hex token must be 2 chars; got '{token}'"
                );
                assert!(
                    token.chars().all(|c| c.is_ascii_hexdigit()),
                    "each token must be hex digits; got '{token}'"
                );
            }
        }
    }
}

// ---------------------------------------------------------------------------
// 18. Debug output of EncodedBytesOwned includes hex bytes via Display
// ---------------------------------------------------------------------------

#[test]
fn test_encoded_bytes_owned_lower_hex_contains_hex_digits() {
    let val: u32 = 0xABCD_EF01;
    let owned = oxicode::encode_to_display(&val).expect("encode_to_display u32");
    let hex_str = format!("{:x}", owned);
    assert!(
        !hex_str.is_empty(),
        "LowerHex of EncodedBytesOwned must not be empty"
    );
    assert!(
        hex_str.chars().all(|c| c.is_ascii_hexdigit()),
        "LowerHex output must only contain hex digits; got: {hex_str}"
    );
    assert!(
        !hex_str.contains(|c: char| ('A'..='F').contains(&c)),
        "LowerHex must be lowercase; got: {hex_str}"
    );
}

// ---------------------------------------------------------------------------
// 19. Hex of bool true encodes to a hex string that decodes back to true
// ---------------------------------------------------------------------------

#[test]
fn test_hex_of_bool_true_roundtrip() {
    let val: bool = true;
    let hex = oxicode::encode_to_hex(&val).expect("encode bool true");
    let (decoded, _): (bool, _) = oxicode::decode_from_hex(&hex).expect("decode bool true");
    assert!(decoded, "decoded bool must be true");
    // The encoded byte for true should be 0x01
    let raw = oxicode::encode_to_vec(&val).expect("encode_to_vec bool true");
    assert_eq!(raw, vec![0x01u8], "bool true encodes as single byte 0x01");
}

// ---------------------------------------------------------------------------
// 20. Hex of bool false encodes to a hex string that decodes back to false
// ---------------------------------------------------------------------------

#[test]
fn test_hex_of_bool_false_roundtrip() {
    let val: bool = false;
    let hex = oxicode::encode_to_hex(&val).expect("encode bool false");
    let (decoded, _): (bool, _) = oxicode::decode_from_hex(&hex).expect("decode bool false");
    assert!(!decoded, "decoded bool must be false");
    // The encoded byte for false should be 0x00
    let raw = oxicode::encode_to_vec(&val).expect("encode_to_vec bool false");
    assert_eq!(raw, vec![0x00u8], "bool false encodes as single byte 0x00");
}

// ---------------------------------------------------------------------------
// 21. Hex of u32(0) — encoded bytes start with 0x00 (varint zero)
// ---------------------------------------------------------------------------

#[test]
fn test_hex_of_u32_zero_first_byte_is_zero() {
    let val: u32 = 0;
    let raw = oxicode::encode_to_vec(&val).expect("encode_to_vec u32 zero");
    // Under standard config the varint for 0 is a single 0x00 byte
    assert_eq!(raw, vec![0x00u8], "u32(0) must encode as single byte 0x00");
    let hex = oxicode::encode_to_hex(&val).expect("encode_to_hex u32 zero");
    assert_eq!(hex, "00", "u32(0) encode_to_hex must be '00'");
}

// ---------------------------------------------------------------------------
// 22. hex_dump format: shows address, hex groups, and ASCII sidebar
// ---------------------------------------------------------------------------

#[test]
fn test_hex_dump_format_shows_address_hex_and_ascii() {
    let data = b"The quick brown fox!";
    let eb = oxicode::encoded_bytes(data);
    let dump = eb.hex_dump();

    // Must have the zero address
    assert!(
        dump.contains("00000000:"),
        "hex_dump must start with '00000000:'; got:\n{dump}"
    );

    // ASCII sidebar must contain printable characters from the input
    assert!(
        dump.contains("The"),
        "hex_dump ASCII sidebar must contain 'The'; got:\n{dump}"
    );
    assert!(
        dump.contains("fox"),
        "hex_dump ASCII sidebar must contain 'fox'; got:\n{dump}"
    );

    // The hex part should have bytes in groups of 2 (pairs of hex chars)
    let first_line = dump
        .lines()
        .next()
        .expect("dump must have at least one line");
    assert!(
        first_line.contains(' '),
        "hex_dump line must contain spaces between byte groups; got: {first_line}"
    );

    // 20 bytes fit on two lines since each line holds 16
    let line_count = dump.lines().count();
    assert_eq!(
        line_count, 2,
        "20 bytes should produce 2 hex dump lines; got {line_count}:\n{dump}"
    );

    // Second line address must be 00000010 (16 in hex)
    assert!(
        dump.contains("00000010:"),
        "second dump line must have address '00000010:'; got:\n{dump}"
    );
}
