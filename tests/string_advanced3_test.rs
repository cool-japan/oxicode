//! Advanced string / &str / char encoding edge cases — set 3.
//!
//! Focuses on varint length boundaries, Unicode multibyte sequences, config
//! variants, collection wrappers, and binary-size assertions.

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
use std::collections::BTreeMap;

use oxicode::{
    config, decode_from_slice, decode_from_slice_with_config, encode_to_vec,
    encode_to_vec_with_config,
};

// ---------------------------------------------------------------------------
// 1. Empty string roundtrip
// ---------------------------------------------------------------------------

/// Empty string encodes as a single zero byte (varint length 0, no content).
#[test]
fn test_string_empty_roundtrip() {
    let s = String::new();
    let encoded = encode_to_vec(&s).expect("encode empty String failed");
    // varint(0) = [0x00] = 1 byte, no content bytes
    assert_eq!(
        encoded.len(),
        1,
        "empty string must encode to exactly 1 byte"
    );
    assert_eq!(encoded[0], 0x00, "first byte must be 0x00 for length 0");
    let (decoded, _): (String, _) =
        decode_from_slice(&encoded).expect("decode empty String failed");
    assert_eq!(decoded, "");
}

// ---------------------------------------------------------------------------
// 2. Single ASCII char string roundtrip
// ---------------------------------------------------------------------------

/// A one-character ASCII string ("a") encodes as [1, 97] = 2 bytes.
#[test]
fn test_string_single_ascii_char_roundtrip() {
    let s = "a".to_string();
    let encoded = encode_to_vec(&s).expect("encode single-char string failed");
    // varint(1) = [0x01], then byte 0x61 ('a')
    assert_eq!(
        encoded.len(),
        2,
        "single-char ASCII string must encode to 2 bytes"
    );
    assert_eq!(encoded[0], 0x01);
    assert_eq!(encoded[1], b'a');
    let (decoded, _): (String, _) =
        decode_from_slice(&encoded).expect("decode single-char string failed");
    assert_eq!(decoded, s);
}

// ---------------------------------------------------------------------------
// 3. String with 250 bytes (varint boundary — fits in 1 length byte)
// ---------------------------------------------------------------------------

/// A 250-byte ASCII string: length byte is a single 0xFA byte; total encoded = 251 bytes.
#[test]
fn test_string_250_bytes_roundtrip() {
    let s = "x".repeat(250);
    let encoded = encode_to_vec(&s).expect("encode 250-byte string failed");
    // varint(250) = [250] = 1 byte (SINGLE_BYTE_MAX = 250), then 250 content bytes
    assert_eq!(
        encoded.len(),
        251,
        "250-byte string must encode to 251 bytes (1 length + 250 content)"
    );
    assert_eq!(encoded[0], 250, "varint(250) must be a single byte 250");
    let (decoded, _): (String, _) =
        decode_from_slice(&encoded).expect("decode 250-byte string failed");
    assert_eq!(decoded, s);
}

// ---------------------------------------------------------------------------
// 4. String with 251 bytes (varint boundary — length needs 3 bytes)
// ---------------------------------------------------------------------------

/// A 251-byte ASCII string: length is encoded as [0xFB, 0xFB, 0x00]; total encoded = 254 bytes.
#[test]
fn test_string_251_bytes_roundtrip() {
    let s = "y".repeat(251);
    let encoded = encode_to_vec(&s).expect("encode 251-byte string failed");
    // varint(251) = [0xFB, 0xFB, 0x00] = 3 bytes (marker + LE u16), then 251 content bytes
    assert_eq!(
        encoded.len(),
        254,
        "251-byte string must encode to 254 bytes (3 length + 251 content)"
    );
    assert_eq!(
        encoded[0], 0xFB,
        "first byte of 251-length varint must be 0xFB marker"
    );
    let (decoded, _): (String, _) =
        decode_from_slice(&encoded).expect("decode 251-byte string failed");
    assert_eq!(decoded, s);
}

// ---------------------------------------------------------------------------
// 5. String with emoji: "Hello 🦀" roundtrip
// ---------------------------------------------------------------------------

/// Emoji characters use 4 UTF-8 bytes each; encoding must preserve them exactly.
#[test]
fn test_string_emoji_roundtrip() {
    let s = "Hello 🦀".to_string();
    let encoded = encode_to_vec(&s).expect("encode emoji string failed");
    let (decoded, _): (String, _) =
        decode_from_slice(&encoded).expect("decode emoji string failed");
    assert_eq!(decoded, s);
}

// ---------------------------------------------------------------------------
// 6. String with null bytes: "foo\0bar" roundtrip
// ---------------------------------------------------------------------------

/// Embedded null bytes (\0) must be preserved in the encoding.
#[test]
fn test_string_null_bytes_roundtrip() {
    let s = "foo\0bar".to_string();
    let encoded = encode_to_vec(&s).expect("encode null-bytes string failed");
    let (decoded, _): (String, _) =
        decode_from_slice(&encoded).expect("decode null-bytes string failed");
    assert_eq!(decoded, s);
    assert_eq!(decoded.len(), 7, "null byte must be counted in the length");
}

// ---------------------------------------------------------------------------
// 7. All printable ASCII chars (0x20..=0x7E) in one string roundtrip
// ---------------------------------------------------------------------------

/// Every printable ASCII code point in a single string must survive the round-trip.
#[test]
fn test_string_all_printable_ascii_roundtrip() {
    let s: String = (0x20u8..=0x7E).map(|b| b as char).collect();
    assert_eq!(s.len(), 95, "must have 95 printable ASCII chars");
    let encoded = encode_to_vec(&s).expect("encode printable ASCII string failed");
    let (decoded, _): (String, _) =
        decode_from_slice(&encoded).expect("decode printable ASCII string failed");
    assert_eq!(decoded, s);
}

// ---------------------------------------------------------------------------
// 8. Unicode multibyte: "日本語テスト" roundtrip
// ---------------------------------------------------------------------------

/// CJK characters are 3-byte UTF-8 sequences; round-trip must be exact.
#[test]
fn test_string_cjk_multibyte_roundtrip() {
    let s = "日本語テスト".to_string();
    let encoded = encode_to_vec(&s).expect("encode CJK string failed");
    let (decoded, _): (String, _) = decode_from_slice(&encoded).expect("decode CJK string failed");
    assert_eq!(decoded, s);
}

// ---------------------------------------------------------------------------
// 9. Vec<String> with empty, short, long strings roundtrip
// ---------------------------------------------------------------------------

/// Collections of strings of various lengths must round-trip correctly.
#[test]
fn test_vec_of_strings_roundtrip() {
    let v: Vec<String> = vec![String::new(), "short".to_string(), "x".repeat(300)];
    let encoded = encode_to_vec(&v).expect("encode Vec<String> failed");
    let (decoded, _): (Vec<String>, _) =
        decode_from_slice(&encoded).expect("decode Vec<String> failed");
    assert_eq!(decoded, v);
}

// ---------------------------------------------------------------------------
// 10. Option<String> Some and None roundtrip
// ---------------------------------------------------------------------------

/// Both Some("...") and None must encode and decode correctly.
#[test]
fn test_option_string_roundtrip() {
    let some_val: Option<String> = Some("option content".to_string());
    let enc_some = encode_to_vec(&some_val).expect("encode Some(String) failed");
    let (dec_some, _): (Option<String>, _) =
        decode_from_slice(&enc_some).expect("decode Some(String) failed");
    assert_eq!(dec_some, some_val);

    let none_val: Option<String> = None;
    let enc_none = encode_to_vec(&none_val).expect("encode None failed");
    let (dec_none, _): (Option<String>, _) =
        decode_from_slice(&enc_none).expect("decode None failed");
    assert_eq!(dec_none, none_val);
}

// ---------------------------------------------------------------------------
// 11. Nested Vec<Vec<String>> roundtrip
// ---------------------------------------------------------------------------

#[allow(clippy::type_complexity)]
/// Nested collections of strings must survive the round-trip.
#[test]
fn test_nested_vec_of_strings_roundtrip() {
    let v: Vec<Vec<String>> = vec![
        vec!["alpha".to_string(), "beta".to_string()],
        vec![],
        vec!["γ".to_string(), "δ".to_string(), "🦀".to_string()],
    ];
    let encoded = encode_to_vec(&v).expect("encode Vec<Vec<String>> failed");
    let (decoded, _): (Vec<Vec<String>>, _) =
        decode_from_slice(&encoded).expect("decode Vec<Vec<String>> failed");
    assert_eq!(decoded, v);
}

// ---------------------------------------------------------------------------
// 12. String with newlines and tabs roundtrip
// ---------------------------------------------------------------------------

/// Control characters such as \n, \r, and \t must be preserved.
#[test]
fn test_string_whitespace_control_chars_roundtrip() {
    let s = "line1\nline2\r\ntabbed\there".to_string();
    let encoded = encode_to_vec(&s).expect("encode control-char string failed");
    let (decoded, _): (String, _) =
        decode_from_slice(&encoded).expect("decode control-char string failed");
    assert_eq!(decoded, s);
}

// ---------------------------------------------------------------------------
// 13. Very long string (10_000 chars) roundtrip
// ---------------------------------------------------------------------------

/// A 10 000-character string must encode and decode without data loss.
#[test]
fn test_string_10000_chars_roundtrip() {
    let s = "abcdefghij".repeat(1_000); // 10 000 ASCII bytes
    assert_eq!(s.len(), 10_000);
    let encoded = encode_to_vec(&s).expect("encode 10000-char string failed");
    let (decoded, _): (String, _) =
        decode_from_slice(&encoded).expect("decode 10000-char string failed");
    assert_eq!(decoded, s);
}

// ---------------------------------------------------------------------------
// 14. String byte size: empty = [0] = 1 byte
// ---------------------------------------------------------------------------

/// Verify the exact wire representation of the empty string.
#[test]
fn test_string_empty_byte_size() {
    let s = String::new();
    let encoded = encode_to_vec(&s).expect("encode empty String failed");
    // varint(0) = [0x00] = 1 byte, no content
    assert_eq!(encoded.len(), 1);
    assert_eq!(encoded[0], 0x00);
    let (decoded, _): (String, _) =
        decode_from_slice(&encoded).expect("decode empty String failed");
    assert_eq!(decoded, "");
}

// ---------------------------------------------------------------------------
// 15. String byte size: "hello" = 1 (len) + 5 (content) = 6 bytes
// ---------------------------------------------------------------------------

/// Verify the exact wire representation of "hello".
#[test]
fn test_string_hello_byte_size() {
    let s = "hello".to_string();
    let encoded = encode_to_vec(&s).expect("encode 'hello' failed");
    // varint(5) = [0x05], then b"hello"
    assert_eq!(encoded.len(), 6, "'hello' must encode to 6 bytes");
    assert_eq!(encoded[0], 5, "length prefix must be 5");
    assert_eq!(&encoded[1..], b"hello", "content bytes must match");
    let (decoded, _): (String, _) = decode_from_slice(&encoded).expect("decode 'hello' failed");
    assert_eq!(decoded, s);
}

// ---------------------------------------------------------------------------
// 16. String with big-endian config roundtrip
// ---------------------------------------------------------------------------

/// String encoding is config-agnostic for endianness (only length varint is affected).
#[test]
fn test_string_big_endian_config_roundtrip() {
    let s = "big endian test 🌏".to_string();
    let cfg = config::standard().with_big_endian();
    let encoded = encode_to_vec_with_config(&s, cfg).expect("encode big-endian string failed");
    let (decoded, _): (String, _) =
        decode_from_slice_with_config(&encoded, cfg).expect("decode big-endian string failed");
    assert_eq!(decoded, s);
}

// ---------------------------------------------------------------------------
// 17. String with fixed-int config roundtrip
// ---------------------------------------------------------------------------

/// With fixed-int encoding the string length uses a fixed-width integer prefix.
#[test]
fn test_string_fixed_int_config_roundtrip() {
    let s = "fixed int encoding test".to_string();
    let cfg = config::standard().with_fixed_int_encoding();
    let encoded = encode_to_vec_with_config(&s, cfg).expect("encode fixed-int string failed");
    let (decoded, _): (String, _) =
        decode_from_slice_with_config(&encoded, cfg).expect("decode fixed-int string failed");
    assert_eq!(decoded, s);
}

// ---------------------------------------------------------------------------
// 18. String decode with tight limit fails
// ---------------------------------------------------------------------------

/// Decoding a string with a byte limit smaller than the payload must return an error.
#[test]
fn test_string_decode_tight_limit_fails() {
    // Encode a reasonably large string without any limit.
    let s = "z".repeat(100);
    let unlimited_bytes =
        encode_to_vec_with_config(&s, config::standard()).expect("unlimited encode failed");

    // Now attempt to decode those bytes with a limit far smaller than the payload.
    let small_cfg = config::standard().with_limit::<4>();
    let result: Result<(String, _), _> = decode_from_slice_with_config(&unlimited_bytes, small_cfg);
    assert!(
        result.is_err(),
        "decoding a 100-byte string with a 4-byte limit must fail"
    );
}

// ---------------------------------------------------------------------------
// 19. (String, String, String) tuple roundtrip
// ---------------------------------------------------------------------------

#[allow(clippy::type_complexity)]
/// A 3-tuple of strings must round-trip correctly.
#[test]
fn test_string_triple_tuple_roundtrip() {
    let t = (
        "first".to_string(),
        "second".to_string(),
        "third".to_string(),
    );
    let encoded = encode_to_vec(&t).expect("encode (String, String, String) failed");
    let (decoded, _): ((String, String, String), _) =
        decode_from_slice(&encoded).expect("decode (String, String, String) failed");
    assert_eq!(decoded, t);
}

// ---------------------------------------------------------------------------
// 20. BTreeMap<String, String> roundtrip
// ---------------------------------------------------------------------------

/// A BTreeMap with string keys and values must round-trip correctly.
#[test]
fn test_btreemap_string_string_roundtrip() {
    let mut map = BTreeMap::new();
    map.insert("key_alpha".to_string(), "value_one".to_string());
    map.insert("key_beta".to_string(), "value_two".to_string());
    map.insert("key_gamma".to_string(), "value_three".to_string());
    map.insert(String::new(), "empty key".to_string());

    let encoded = encode_to_vec(&map).expect("encode BTreeMap<String, String> failed");
    let (decoded, _): (BTreeMap<String, String>, _) =
        decode_from_slice(&encoded).expect("decode BTreeMap<String, String> failed");
    assert_eq!(decoded, map);
}

// ---------------------------------------------------------------------------
// 21. String containing only whitespace roundtrip
// ---------------------------------------------------------------------------

/// A string of spaces, tabs, and newlines only must survive the round-trip.
#[test]
fn test_string_only_whitespace_roundtrip() {
    let s = "   \t\t\n\n\r\n   ".to_string();
    let encoded = encode_to_vec(&s).expect("encode whitespace-only string failed");
    let (decoded, _): (String, _) =
        decode_from_slice(&encoded).expect("decode whitespace-only string failed");
    assert_eq!(decoded, s);
}

// ---------------------------------------------------------------------------
// 22. String with all 4-byte UTF-8 chars (Mathematical Fraktur) roundtrip
// ---------------------------------------------------------------------------

/// Characters outside the BMP that encode as 4 UTF-8 bytes must survive the round-trip.
#[test]
fn test_string_four_byte_utf8_roundtrip() {
    // U+1D573..U+1D576 are 4-byte Mathematical Fraktur capital letters
    let s = "𝕳𝖊𝖑𝖑𝖔".to_string();
    // Each glyph is 4 UTF-8 bytes; 5 glyphs = 20 bytes of content.
    assert_eq!(s.len(), 20, "each Fraktur char is 4 UTF-8 bytes");
    let encoded = encode_to_vec(&s).expect("encode 4-byte UTF-8 string failed");
    let (decoded, _): (String, _) =
        decode_from_slice(&encoded).expect("decode 4-byte UTF-8 string failed");
    assert_eq!(decoded, s);
    assert_eq!(decoded.chars().count(), 5, "must preserve 5 codepoints");
}
