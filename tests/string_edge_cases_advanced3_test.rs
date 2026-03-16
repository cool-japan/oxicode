//! Advanced string encoding edge cases — set 3.
//!
//! 22 top-level test functions covering empty strings, null bytes, ASCII, Unicode,
//! varint length prefixes, Vec<String>, and various roundtrip invariants.

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
#[allow(unused_imports)]
use oxicode::{decode_from_slice, encode_to_vec, Decode, Encode};

// ---------------------------------------------------------------------------
// 1. Empty string "" roundtrip
// ---------------------------------------------------------------------------

#[test]
fn test_empty_string_roundtrip() {
    let s = String::new();
    let enc = encode_to_vec(&s).expect("encode empty string");
    let (val, _): (String, usize) = decode_from_slice(&enc).expect("decode empty string");
    assert_eq!(val, "");
}

// ---------------------------------------------------------------------------
// 2. Single character "a" roundtrip
// ---------------------------------------------------------------------------

#[test]
fn test_single_char_a_roundtrip() {
    let s = "a".to_string();
    let enc = encode_to_vec(&s).expect("encode single char 'a'");
    let (val, _): (String, usize) = decode_from_slice(&enc).expect("decode single char 'a'");
    assert_eq!(val, "a");
}

// ---------------------------------------------------------------------------
// 3. All ASCII printable chars in one string roundtrip
// ---------------------------------------------------------------------------

#[test]
fn test_all_ascii_printable_roundtrip() {
    // ASCII printable: 0x20 (space) through 0x7E (~)
    let s: String = (0x20u8..=0x7Eu8).map(|b| b as char).collect();
    assert_eq!(s.len(), 95, "there are 95 printable ASCII chars");
    let enc = encode_to_vec(&s).expect("encode all printable ASCII");
    let (val, _): (String, usize) = decode_from_slice(&enc).expect("decode all printable ASCII");
    assert_eq!(val, s, "all printable ASCII must survive roundtrip");
}

// ---------------------------------------------------------------------------
// 4. String with null bytes: "hel\0lo" roundtrip
// ---------------------------------------------------------------------------

#[test]
fn test_string_with_null_byte_in_middle_roundtrip() {
    let s = "hel\0lo".to_string();
    assert_eq!(s.len(), 6, "hel\\0lo must be 6 bytes");
    let enc = encode_to_vec(&s).expect("encode string with null in middle");
    let (val, _): (String, usize) =
        decode_from_slice(&enc).expect("decode string with null in middle");
    assert_eq!(val, "hel\0lo", "null byte in middle must be preserved");
}

// ---------------------------------------------------------------------------
// 5. String with all byte values 0x01-0x7F (valid UTF-8) roundtrip
// ---------------------------------------------------------------------------

#[test]
fn test_string_all_byte_values_0x01_to_0x7f_roundtrip() {
    // 0x01..=0x7F are all valid single-byte UTF-8 codepoints
    let s: String = (0x01u8..=0x7Fu8).map(|b| b as char).collect();
    assert_eq!(s.len(), 127, "0x01..=0x7F is 127 bytes");
    let enc = encode_to_vec(&s).expect("encode 0x01..=0x7F string");
    let (val, _): (String, usize) = decode_from_slice(&enc).expect("decode 0x01..=0x7F string");
    assert_eq!(val, s, "all byte values 0x01..=0x7F must survive roundtrip");
}

// ---------------------------------------------------------------------------
// 6. Unicode BMP: "日本語テスト" roundtrip
// ---------------------------------------------------------------------------

#[test]
fn test_unicode_bmp_japanese_roundtrip() {
    let s = "日本語テスト".to_string();
    let enc = encode_to_vec(&s).expect("encode Japanese BMP string");
    let (val, _): (String, usize) = decode_from_slice(&enc).expect("decode Japanese BMP string");
    assert_eq!(
        val, "日本語テスト",
        "Japanese BMP characters must survive roundtrip"
    );
}

// ---------------------------------------------------------------------------
// 7. Unicode emoji: "🦀🎉🔥💻" roundtrip
// ---------------------------------------------------------------------------

#[test]
fn test_unicode_emoji_roundtrip() {
    let s = "🦀🎉🔥💻".to_string();
    let enc = encode_to_vec(&s).expect("encode emoji string");
    let (val, _): (String, usize) = decode_from_slice(&enc).expect("decode emoji string");
    assert_eq!(val, "🦀🎉🔥💻", "emoji must survive roundtrip");
}

// ---------------------------------------------------------------------------
// 8. Mixed ASCII + Unicode: "Hello, 世界! 🌍" roundtrip
// ---------------------------------------------------------------------------

#[test]
fn test_mixed_ascii_unicode_roundtrip() {
    let s = "Hello, 世界! 🌍".to_string();
    let enc = encode_to_vec(&s).expect("encode mixed ASCII+Unicode");
    let (val, _): (String, usize) = decode_from_slice(&enc).expect("decode mixed ASCII+Unicode");
    assert_eq!(
        val, "Hello, 世界! 🌍",
        "mixed ASCII+Unicode must survive roundtrip"
    );
}

// ---------------------------------------------------------------------------
// 9. String with just whitespace: "   \t\n\r" roundtrip
// ---------------------------------------------------------------------------

#[test]
fn test_whitespace_only_string_roundtrip() {
    let s = "   \t\n\r".to_string();
    let enc = encode_to_vec(&s).expect("encode whitespace-only string");
    let (val, _): (String, usize) = decode_from_slice(&enc).expect("decode whitespace-only string");
    assert_eq!(
        val, "   \t\n\r",
        "whitespace-only string must survive roundtrip"
    );
}

// ---------------------------------------------------------------------------
// 10. Very long string (1000 'a' chars) roundtrip
// ---------------------------------------------------------------------------

#[test]
fn test_very_long_ascii_string_roundtrip() {
    let s = "a".repeat(1000);
    assert_eq!(s.len(), 1000);
    let enc = encode_to_vec(&s).expect("encode 1000-'a' string");
    let (val, _): (String, usize) = decode_from_slice(&enc).expect("decode 1000-'a' string");
    assert_eq!(val, s, "1000-char ASCII string must survive roundtrip");
    assert_eq!(val.len(), 1000);
}

// ---------------------------------------------------------------------------
// 11. Very long Unicode string (100 '中' chars) roundtrip
// ---------------------------------------------------------------------------

#[test]
fn test_very_long_unicode_string_roundtrip() {
    let s: String = std::iter::repeat('中').take(100).collect();
    assert_eq!(s.chars().count(), 100, "must have 100 Unicode codepoints");
    // '中' is 3 bytes in UTF-8
    assert_eq!(s.len(), 300, "100 × '中' must be 300 UTF-8 bytes");
    let enc = encode_to_vec(&s).expect("encode 100-'中' string");
    let (val, _): (String, usize) = decode_from_slice(&enc).expect("decode 100-'中' string");
    assert_eq!(val, s, "100-'中' Unicode string must survive roundtrip");
}

// ---------------------------------------------------------------------------
// 12. String length encoding: length is encoded as varint, not fixed size
// ---------------------------------------------------------------------------

#[test]
fn test_string_length_encoded_as_varint() {
    // A short string (len=10) should have a 1-byte varint length prefix
    let short = "a".repeat(10);
    let enc_short = encode_to_vec(&short).expect("encode short 10-char string");
    // varint(10) = 1 byte, so total = 1 + 10 = 11
    assert_eq!(
        enc_short.len(),
        11,
        "10-char string must encode to 11 bytes (1 varint + 10 content)"
    );
    assert_eq!(enc_short[0], 10, "varint(10) must be single byte 0x0A");

    // A string of length 251 crosses the 1-byte varint boundary
    let long = "b".repeat(251);
    let enc_long = encode_to_vec(&long).expect("encode 251-char string");
    // OxiCode varint: 0-250 = 1 byte; 251-65535 = 3 bytes (0xFB + LE u16)
    assert_eq!(
        enc_long.len(),
        254,
        "251-char string must encode to 254 bytes (3-byte varint + 251 content)"
    );
    assert_eq!(
        enc_long[0], 0xFB,
        "first byte of multi-byte varint for len=251 must be 0xFB marker"
    );
}

// ---------------------------------------------------------------------------
// 13. String with null byte at start: "\0hello" roundtrip
// ---------------------------------------------------------------------------

#[test]
fn test_string_null_byte_at_start_roundtrip() {
    let s = "\0hello".to_string();
    assert_eq!(s.len(), 6);
    let enc = encode_to_vec(&s).expect("encode string with leading null");
    let (val, _): (String, usize) =
        decode_from_slice(&enc).expect("decode string with leading null");
    assert_eq!(val, "\0hello", "leading null byte must be preserved");
    assert_eq!(
        val.as_bytes()[0],
        0x00,
        "first byte of decoded string must be null"
    );
}

// ---------------------------------------------------------------------------
// 14. String with null byte at end: "hello\0" roundtrip
// ---------------------------------------------------------------------------

#[test]
fn test_string_null_byte_at_end_roundtrip() {
    let s = "hello\0".to_string();
    assert_eq!(s.len(), 6);
    let enc = encode_to_vec(&s).expect("encode string with trailing null");
    let (val, _): (String, usize) =
        decode_from_slice(&enc).expect("decode string with trailing null");
    assert_eq!(val, "hello\0", "trailing null byte must be preserved");
    assert_eq!(
        val.as_bytes()[5],
        0x00,
        "last byte of decoded string must be null"
    );
}

// ---------------------------------------------------------------------------
// 15. Two different strings encode differently
// ---------------------------------------------------------------------------

#[test]
fn test_two_different_strings_encode_differently() {
    let s1 = "hello".to_string();
    let s2 = "world".to_string();
    let enc1 = encode_to_vec(&s1).expect("encode 'hello'");
    let enc2 = encode_to_vec(&s2).expect("encode 'world'");
    assert_ne!(
        enc1, enc2,
        "distinct strings must produce distinct encoded bytes"
    );
}

// ---------------------------------------------------------------------------
// 16. String length 250 (varint boundary) roundtrip
// ---------------------------------------------------------------------------

#[test]
fn test_string_length_250_varint_boundary_roundtrip() {
    let s = "x".repeat(250);
    let enc = encode_to_vec(&s).expect("encode 250-char string");
    // varint(250) = 1 byte (250 <= 250), total = 1 + 250 = 251
    assert_eq!(
        enc.len(),
        251,
        "250-char string must encode to 251 bytes (1-byte varint + 250 content)"
    );
    assert_eq!(enc[0], 250, "varint(250) must be the single byte 0xFA");
    let (val, consumed): (String, usize) = decode_from_slice(&enc).expect("decode 250-char string");
    assert_eq!(val, s, "250-char string must roundtrip correctly");
    assert_eq!(consumed, 251);
}

// ---------------------------------------------------------------------------
// 17. String length 251 (crosses 1-byte varint boundary) roundtrip
// ---------------------------------------------------------------------------

#[test]
fn test_string_length_251_crosses_varint_boundary_roundtrip() {
    let s = "y".repeat(251);
    let enc = encode_to_vec(&s).expect("encode 251-char string");
    // varint(251) requires 3 bytes (0xFB + LE u16), total = 3 + 251 = 254
    assert_eq!(
        enc.len(),
        254,
        "251-char string must encode to 254 bytes (3-byte varint + 251 content)"
    );
    assert_eq!(
        enc[0], 0xFB,
        "first byte must be 0xFB multi-byte varint marker"
    );
    let (val, consumed): (String, usize) = decode_from_slice(&enc).expect("decode 251-char string");
    assert_eq!(val, s, "251-char string must roundtrip correctly");
    assert_eq!(consumed, 254);
}

// ---------------------------------------------------------------------------
// 18. String "hello" and String::from("hello") produce same bytes
// ---------------------------------------------------------------------------

#[test]
fn test_string_literal_and_string_from_produce_same_bytes() {
    let s1 = "hello".to_string();
    let s2 = String::from("hello");
    let enc1 = encode_to_vec(&s1).expect("encode string literal");
    let enc2 = encode_to_vec(&s2).expect("encode String::from");
    assert_eq!(
        enc1, enc2,
        "\"hello\".to_string() and String::from(\"hello\") must produce identical encoded bytes"
    );
}

// ---------------------------------------------------------------------------
// 19. Encoded string bytes: first N bytes encode the UTF-8 length
// ---------------------------------------------------------------------------

#[test]
fn test_encoded_string_length_prefix_matches_utf8_byte_count() {
    // For a short string, the first byte is the varint of the UTF-8 byte length
    let s = "hello".to_string();
    assert_eq!(s.len(), 5, "'hello' is 5 UTF-8 bytes");
    let enc = encode_to_vec(&s).expect("encode 'hello'");
    // varint(5) = 0x05
    assert_eq!(enc[0], 5, "first encoded byte must be varint(5) = 0x05");
    // The remaining bytes must be the raw UTF-8 content
    assert_eq!(
        &enc[1..],
        b"hello",
        "content bytes must match raw UTF-8 of 'hello'"
    );

    // Also test with a Unicode string where byte length != char count
    let u = "日本".to_string();
    // '日' = 3 bytes, '本' = 3 bytes => total 6 UTF-8 bytes
    assert_eq!(u.len(), 6, "'日本' is 6 UTF-8 bytes");
    let enc_u = encode_to_vec(&u).expect("encode '日本'");
    assert_eq!(
        enc_u[0], 6,
        "first encoded byte for '日本' must be varint(6) = 0x06"
    );
    assert_eq!(
        &enc_u[1..],
        u.as_bytes(),
        "content bytes must match raw UTF-8 of '日本'"
    );
}

// ---------------------------------------------------------------------------
// 20. Consumed bytes == encoded length for various strings
// ---------------------------------------------------------------------------

#[test]
fn test_consumed_bytes_equals_encoded_length_for_various_strings() {
    let cases: &[&str] = &[
        "",
        "z",
        "hello",
        "🦀",
        "日本語",
        "\0\0\0",
        &"a".repeat(50),
        &"b".repeat(250),
        &"c".repeat(251),
    ];
    for &s in cases {
        let owned = s.to_string();
        let enc = encode_to_vec(&owned).unwrap_or_else(|e| panic!("encode {:?} failed: {}", s, e));
        let (_val, consumed): (String, usize) =
            decode_from_slice(&enc).unwrap_or_else(|e| panic!("decode {:?} failed: {}", s, e));
        assert_eq!(
            consumed,
            enc.len(),
            "consumed ({}) must equal encoded length ({}) for {:?}",
            consumed,
            enc.len(),
            s
        );
    }
}

// ---------------------------------------------------------------------------
// 21. Vec<String> with empty and non-empty strings roundtrip
// ---------------------------------------------------------------------------

#[test]
fn test_vec_string_with_empty_and_nonempty_roundtrip() {
    let v: Vec<String> = vec![
        "".to_string(),
        "hello".to_string(),
        "".to_string(),
        "世界".to_string(),
        "🔥".to_string(),
        "".to_string(),
    ];
    let enc = encode_to_vec(&v).expect("encode Vec<String>");
    let (val, consumed): (Vec<String>, usize) =
        decode_from_slice(&enc).expect("decode Vec<String>");
    assert_eq!(
        val, v,
        "Vec<String> with empty and non-empty strings must roundtrip"
    );
    assert_eq!(val.len(), 6, "decoded Vec must have 6 elements");
    assert_eq!(val[0], "", "first element must be empty string");
    assert_eq!(val[1], "hello", "second element must be 'hello'");
    assert_eq!(val[2], "", "third element must be empty string");
    assert_eq!(consumed, enc.len(), "consumed must equal encoded length");
}

// ---------------------------------------------------------------------------
// 22. String with mixed multi-byte characters at different positions
// ---------------------------------------------------------------------------

#[test]
fn test_string_mixed_multibyte_at_different_positions_roundtrip() {
    // Mix of: 1-byte ASCII, 2-byte Latin ext, 3-byte CJK, 4-byte emoji
    // 'A' = 1 byte, 'é' = 2 bytes, '中' = 3 bytes, '🦀' = 4 bytes
    let s = "A\u{00E9}\u{4E2D}\u{1F980}B\u{00E9}\u{4E2D}\u{1F980}C".to_string();
    // byte lengths: 1+2+3+4 + 1+2+3+4 + 1 = 21 bytes
    let expected_byte_len = 1 + 2 + 3 + 4 + 1 + 2 + 3 + 4 + 1;
    assert_eq!(
        s.len(),
        expected_byte_len,
        "mixed multibyte string byte length mismatch"
    );
    assert_eq!(s.chars().count(), 9, "must have 9 codepoints");
    let enc = encode_to_vec(&s).expect("encode mixed multibyte string");
    let (val, consumed): (String, usize) =
        decode_from_slice(&enc).expect("decode mixed multibyte string");
    assert_eq!(val, s, "mixed multibyte characters must survive roundtrip");
    assert_eq!(
        val.len(),
        expected_byte_len,
        "byte length must be preserved after roundtrip"
    );
    assert_eq!(val.chars().count(), 9, "codepoint count must be preserved");
    assert_eq!(consumed, enc.len(), "consumed must equal encoded length");
}
