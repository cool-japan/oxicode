//! Advanced string encoding edge cases — set 4.
//!
//! Covers new angles: exact byte-count assertions, varint boundary arithmetic,
//! whitespace/null/RTL/backslash/escape content, non-ASCII-only strings,
//! max Unicode codepoint, sequential decoding from a shared buffer,
//! config variants (fixed-int, big-endian), and the consumed == encoded.len()
//! invariant.

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
    encode_to_vec_with_config,
};

// ---------------------------------------------------------------------------
// 1. Empty string encodes as exactly 1 byte (varint 0)
// ---------------------------------------------------------------------------

/// An empty String must encode to a single 0x00 byte — only the varint length.
#[test]
fn test_empty_string_exactly_one_byte() {
    let s = String::new();
    let encoded = encode_to_vec(&s).expect("encode empty String");
    assert_eq!(
        encoded.len(),
        1,
        "empty string must encode to exactly 1 byte"
    );
    assert_eq!(encoded[0], 0x00, "the single byte must be 0x00 (varint 0)");
    let (decoded, consumed): (String, _) =
        decode_from_slice(&encoded).expect("decode empty String");
    assert_eq!(decoded, "");
    assert_eq!(consumed, 1);
}

// ---------------------------------------------------------------------------
// 2. 1-char ASCII string encodes as exactly 2 bytes (varint 1 + char byte)
// ---------------------------------------------------------------------------

/// A one-character ASCII string must encode to [0x01, byte] = 2 bytes.
#[test]
fn test_one_char_ascii_exactly_two_bytes() {
    let s = "Q".to_string();
    let encoded = encode_to_vec(&s).expect("encode 1-char ASCII");
    assert_eq!(encoded.len(), 2, "1-char ASCII must encode to 2 bytes");
    assert_eq!(encoded[0], 0x01, "varint(1) must be 0x01");
    assert_eq!(
        encoded[1], b'Q',
        "content byte must match ASCII value of 'Q'"
    );
    let (decoded, consumed): (String, _) =
        decode_from_slice(&encoded).expect("decode 1-char ASCII");
    assert_eq!(decoded, "Q");
    assert_eq!(consumed, 2);
}

// ---------------------------------------------------------------------------
// 3. 250-char string — varint fits in a single byte (just below multi-byte boundary)
// ---------------------------------------------------------------------------

/// A 250-byte string uses a 1-byte varint length prefix; total encoded = 251 bytes.
#[test]
fn test_250_char_string_varint_single_byte() {
    let s = "m".repeat(250);
    let encoded = encode_to_vec(&s).expect("encode 250-char string");
    assert_eq!(
        encoded.len(),
        251,
        "250-char string must encode to 251 bytes (1 varint + 250 content)"
    );
    assert_eq!(
        encoded[0], 250,
        "varint(250) must be the single byte 0xFA (250)"
    );
    let (decoded, consumed): (String, _) =
        decode_from_slice(&encoded).expect("decode 250-char string");
    assert_eq!(decoded, s);
    assert_eq!(consumed, 251);
}

// ---------------------------------------------------------------------------
// 4. 251-char string — at varint multi-byte boundary (3-byte length prefix)
// ---------------------------------------------------------------------------

/// A 251-byte string triggers the multi-byte varint; first byte = 0xFB; total = 254 bytes.
#[test]
fn test_251_char_string_varint_multibyte() {
    let s = "n".repeat(251);
    let encoded = encode_to_vec(&s).expect("encode 251-char string");
    assert_eq!(
        encoded.len(),
        254,
        "251-char string must encode to 254 bytes (3 varint + 251 content)"
    );
    assert_eq!(
        encoded[0], 0xFB,
        "first byte of multi-byte varint must be 0xFB marker"
    );
    let (decoded, consumed): (String, _) =
        decode_from_slice(&encoded).expect("decode 251-char string");
    assert_eq!(decoded, s);
    assert_eq!(consumed, 254);
}

// ---------------------------------------------------------------------------
// 5. String with all ASCII whitespace chars (space, tab, newline, vertical tab, form feed, CR)
// ---------------------------------------------------------------------------

/// All six ASCII whitespace characters must be preserved after roundtrip.
#[test]
fn test_string_all_ascii_whitespace_chars() {
    // 0x09=\t, 0x0A=\n, 0x0B=\x0B (VT), 0x0C=\x0C (FF), 0x0D=\r, 0x20=space
    let s = " \t\n\x0B\x0C\r".to_string();
    assert_eq!(s.len(), 6, "six distinct ASCII whitespace bytes");
    let encoded = encode_to_vec(&s).expect("encode all-ASCII-whitespace string");
    let (decoded, consumed): (String, _) =
        decode_from_slice(&encoded).expect("decode all-ASCII-whitespace string");
    assert_eq!(
        decoded, s,
        "all ASCII whitespace chars must survive roundtrip"
    );
    assert_eq!(consumed, encoded.len());
}

// ---------------------------------------------------------------------------
// 6. String with null bytes ("\0\0\0") — Rust strings may contain null bytes
// ---------------------------------------------------------------------------

/// Three consecutive null bytes embedded in a string must be preserved.
#[test]
fn test_string_three_null_bytes() {
    let s = "\0\0\0".to_string();
    assert_eq!(s.len(), 3, "three null bytes must give byte length 3");
    let encoded = encode_to_vec(&s).expect("encode three-null-bytes string");
    // varint(3) = 1 byte, plus 3 null bytes = 4 bytes total
    assert_eq!(encoded.len(), 4, "three-null string must encode to 4 bytes");
    let (decoded, consumed): (String, _) =
        decode_from_slice(&encoded).expect("decode three-null-bytes string");
    assert_eq!(decoded, s, "null bytes must be preserved verbatim");
    assert_eq!(consumed, 4);
}

// ---------------------------------------------------------------------------
// 7. String with only spaces (100 spaces)
// ---------------------------------------------------------------------------

/// One hundred spaces must roundtrip correctly and maintain exact byte count.
#[test]
fn test_string_100_spaces_only() {
    let s = " ".repeat(100);
    let encoded = encode_to_vec(&s).expect("encode 100-spaces string");
    // varint(100) = 1 byte, plus 100 space bytes = 101 bytes total
    assert_eq!(
        encoded.len(),
        101,
        "100-space string must encode to 101 bytes"
    );
    let (decoded, consumed): (String, _) =
        decode_from_slice(&encoded).expect("decode 100-spaces string");
    assert_eq!(decoded, s);
    assert!(
        decoded.chars().all(|c| c == ' '),
        "all decoded chars must be spaces"
    );
    assert_eq!(consumed, 101);
}

// ---------------------------------------------------------------------------
// 8. String with leading and trailing whitespace preserved
// ---------------------------------------------------------------------------

/// Leading/trailing whitespace must not be trimmed or altered by encoding.
#[test]
fn test_string_leading_trailing_whitespace_preserved() {
    let s = "   hello world   \t\n".to_string();
    let encoded = encode_to_vec(&s).expect("encode leading/trailing whitespace string");
    let (decoded, consumed): (String, _) =
        decode_from_slice(&encoded).expect("decode leading/trailing whitespace string");
    assert_eq!(
        decoded, s,
        "leading and trailing whitespace must be preserved exactly"
    );
    assert!(decoded.starts_with("   "), "leading spaces must be intact");
    assert!(decoded.ends_with("\t\n"), "trailing \\t\\n must be intact");
    assert_eq!(consumed, encoded.len());
}

// ---------------------------------------------------------------------------
// 9. String with emoji sequence roundtrip
// ---------------------------------------------------------------------------

/// A sequence of diverse emoji (flags, ZWJ sequences, skin-tone modifiers) must roundtrip.
#[test]
fn test_string_emoji_sequence_roundtrip() {
    // ZWJ family sequence + country flags + other emoji
    let s = "👨‍👩‍👧‍👦🏴󠁧󠁢󠁥󠁮󠁧󠁿🎭🧬🦠🧲🌡️".to_string();
    let encoded = encode_to_vec(&s).expect("encode emoji sequence");
    let (decoded, consumed): (String, _) =
        decode_from_slice(&encoded).expect("decode emoji sequence");
    assert_eq!(decoded, s, "emoji sequence must survive roundtrip exactly");
    assert_eq!(consumed, encoded.len());
}

// ---------------------------------------------------------------------------
// 10. String with mixed CJK + ASCII
// ---------------------------------------------------------------------------

/// Interleaved CJK and ASCII characters must be preserved at the byte level.
#[test]
fn test_string_mixed_cjk_ascii_roundtrip() {
    let s = "Hello世界Rust锈迹foo日本語bar".to_string();
    let byte_len = s.len();
    assert!(byte_len > s.chars().count(), "string has multibyte chars");
    let encoded = encode_to_vec(&s).expect("encode mixed CJK+ASCII");
    let (decoded, consumed): (String, _) =
        decode_from_slice(&encoded).expect("decode mixed CJK+ASCII");
    assert_eq!(decoded, s, "mixed CJK+ASCII must survive roundtrip");
    assert_eq!(decoded.len(), byte_len, "byte length must be preserved");
    assert_eq!(consumed, encoded.len());
}

// ---------------------------------------------------------------------------
// 11. String with RTL text (Arabic + Hebrew characters)
// ---------------------------------------------------------------------------

/// Right-to-left Arabic and Hebrew text must roundtrip without corruption.
#[test]
fn test_string_rtl_arabic_hebrew_roundtrip() {
    // Arabic: "مرحبا" (marhaba = hello), Hebrew: "שלום" (shalom = peace)
    let s = "مرحبا שלום مرحبا بالعالم".to_string();
    let encoded = encode_to_vec(&s).expect("encode RTL Arabic+Hebrew");
    let (decoded, consumed): (String, _) =
        decode_from_slice(&encoded).expect("decode RTL Arabic+Hebrew");
    assert_eq!(
        decoded, s,
        "RTL Arabic and Hebrew text must survive roundtrip"
    );
    assert_eq!(consumed, encoded.len());
}

// ---------------------------------------------------------------------------
// 12. String containing backslash sequences (Windows-style paths)
// ---------------------------------------------------------------------------

/// Backslash characters in strings must not be interpreted or stripped.
#[test]
fn test_string_backslash_sequences_roundtrip() {
    let s = r"C:\Users\Ferris\Documents\rust_project\src\main.rs".to_string();
    let backslash_count = s.chars().filter(|&c| c == '\\').count();
    // "C:\Users\Ferris\Documents\rust_project\src\main.rs" has 6 backslashes
    assert!(
        backslash_count > 0,
        "path must contain at least one backslash"
    );
    let encoded = encode_to_vec(&s).expect("encode backslash-path string");
    let (decoded, consumed): (String, _) =
        decode_from_slice(&encoded).expect("decode backslash-path string");
    assert_eq!(decoded, s, "backslash path must survive roundtrip");
    assert_eq!(
        decoded.chars().filter(|&c| c == '\\').count(),
        backslash_count,
        "backslash count must be preserved after roundtrip"
    );
    assert_eq!(consumed, encoded.len());
}

// ---------------------------------------------------------------------------
// 13. String with escaped characters: "\n\t\r" (actual control chars, not escape sequences)
// ---------------------------------------------------------------------------

/// The actual byte values for newline (0x0A), tab (0x09), and CR (0x0D) must be preserved.
#[test]
fn test_string_literal_escape_chars_roundtrip() {
    let s = "\n\t\r".to_string();
    assert_eq!(s.len(), 3, "literal \\n\\t\\r must be exactly 3 bytes");
    assert_eq!(s.as_bytes()[0], 0x0A, "first byte must be 0x0A (newline)");
    assert_eq!(s.as_bytes()[1], 0x09, "second byte must be 0x09 (tab)");
    assert_eq!(s.as_bytes()[2], 0x0D, "third byte must be 0x0D (CR)");
    let encoded = encode_to_vec(&s).expect("encode \\n\\t\\r string");
    // varint(3) = 1 byte + 3 content bytes = 4 bytes
    assert_eq!(encoded.len(), 4, "\\n\\t\\r string must encode to 4 bytes");
    let (decoded, consumed): (String, _) =
        decode_from_slice(&encoded).expect("decode \\n\\t\\r string");
    assert_eq!(
        decoded, s,
        "literal escape char bytes must survive roundtrip"
    );
    assert_eq!(consumed, 4);
}

// ---------------------------------------------------------------------------
// 14. Very long string: 10,000 chars of repeated ASCII
// ---------------------------------------------------------------------------

/// A 10 000-character repeated ASCII pattern must roundtrip without any data loss.
#[test]
fn test_string_10000_repeated_ascii_roundtrip() {
    let s = "abcdefghij".repeat(1_000); // 10_000 bytes
    assert_eq!(s.len(), 10_000);
    let encoded = encode_to_vec(&s).expect("encode 10000-char repeated ASCII");
    let (decoded, consumed): (String, _) =
        decode_from_slice(&encoded).expect("decode 10000-char repeated ASCII");
    assert_eq!(decoded, s);
    assert_eq!(decoded.len(), 10_000);
    assert_eq!(consumed, encoded.len());
}

// ---------------------------------------------------------------------------
// 15. String with max Unicode scalar '\u{10FFFF}' repeated 5 times
// ---------------------------------------------------------------------------

/// The maximum valid Unicode scalar value (U+10FFFF) encodes as 4 UTF-8 bytes each.
#[test]
fn test_string_max_unicode_scalar_repeated() {
    let max_char = '\u{10FFFF}';
    assert_eq!(max_char.len_utf8(), 4, "U+10FFFF must be 4 UTF-8 bytes");
    let s: String = std::iter::repeat(max_char).take(5).collect();
    assert_eq!(s.len(), 20, "5 × U+10FFFF must be 20 UTF-8 bytes");
    let encoded = encode_to_vec(&s).expect("encode max-Unicode-scalar string");
    // varint(20) = 1 byte, plus 20 content bytes = 21 bytes total
    assert_eq!(encoded.len(), 21, "5×U+10FFFF must encode to 21 bytes");
    let (decoded, consumed): (String, _) =
        decode_from_slice(&encoded).expect("decode max-Unicode-scalar string");
    assert_eq!(decoded, s, "max Unicode scalar must survive roundtrip");
    assert_eq!(decoded.chars().count(), 5, "must decode as 5 codepoints");
    assert_eq!(consumed, 21);
}

// ---------------------------------------------------------------------------
// 16. String equality after roundtrip (not just byte equality)
// ---------------------------------------------------------------------------

/// After roundtrip the decoded String must compare equal using PartialEq, not just byte-by-byte.
#[test]
fn test_string_equality_after_roundtrip_not_just_bytes() {
    let s = "Ångström café naïve résumé".to_string();
    let encoded = encode_to_vec(&s).expect("encode accented Latin string");
    let (decoded, consumed): (String, _) =
        decode_from_slice(&encoded).expect("decode accented Latin string");
    // Test PartialEq (String == String), not pointer equality
    assert_eq!(decoded, s, "decoded String must be equal via PartialEq");
    assert!(
        !std::ptr::eq(decoded.as_ptr(), s.as_ptr()),
        "must be a distinct allocation"
    );
    assert_eq!(decoded.len(), s.len(), "byte lengths must match");
    assert_eq!(consumed, encoded.len());
}

// ---------------------------------------------------------------------------
// 17. Fixed-int config with String (string length still uses varint in oxicode)
// ---------------------------------------------------------------------------

/// Under fixed-int encoding config the String still roundtrips correctly.
#[test]
fn test_string_fixed_int_config_roundtrip() {
    let s = "fixed-int config string test 🦀".to_string();
    let cfg = config::standard().with_fixed_int_encoding();
    let encoded = encode_to_vec_with_config(&s, cfg).expect("encode fixed-int string");
    let (decoded, consumed): (String, _) =
        decode_from_slice_with_config(&encoded, cfg).expect("decode fixed-int string");
    assert_eq!(
        decoded, s,
        "fixed-int config must not corrupt string content"
    );
    assert_eq!(consumed, encoded.len());
}

// ---------------------------------------------------------------------------
// 18. Big-endian config with String
// ---------------------------------------------------------------------------

/// Under big-endian config the String must still roundtrip correctly.
#[test]
fn test_string_big_endian_config_roundtrip() {
    let s = "big-endian config string テスト".to_string();
    let cfg = config::standard().with_big_endian();
    let encoded = encode_to_vec_with_config(&s, cfg).expect("encode big-endian string");
    let (decoded, consumed): (String, _) =
        decode_from_slice_with_config(&encoded, cfg).expect("decode big-endian string");
    assert_eq!(
        decoded, s,
        "big-endian config must not corrupt string content"
    );
    assert_eq!(consumed, encoded.len());
}

// ---------------------------------------------------------------------------
// 19. consumed == encoded.len() invariant for diverse strings
// ---------------------------------------------------------------------------

/// For any string, the number of bytes consumed by decode must equal the encoded length.
#[test]
fn test_string_consumed_equals_encoded_len_invariant() {
    let cases: &[&str] = &[
        "",
        "a",
        "hello",
        "日本語",
        "🦀🔥",
        "\0\0",
        &"x".repeat(250),
        &"y".repeat(251),
        &"z".repeat(300),
    ];
    for &s in cases {
        let encoded = encode_to_vec(&s.to_string())
            .unwrap_or_else(|e| panic!("encode {:?} failed: {}", s, e));
        let (_decoded, consumed): (String, _) =
            decode_from_slice(&encoded).unwrap_or_else(|e| panic!("decode {:?} failed: {}", s, e));
        assert_eq!(
            consumed,
            encoded.len(),
            "consumed ({}) must equal encoded.len() ({}) for {:?}",
            consumed,
            encoded.len(),
            s
        );
    }
}

// ---------------------------------------------------------------------------
// 20. 252-char string (within varint 3-byte range, well above 251 boundary)
// ---------------------------------------------------------------------------

/// A 252-byte string stays in the 3-byte varint range; total encoded = 255 bytes.
#[test]
fn test_252_char_string_varint_range() {
    let s = "p".repeat(252);
    let encoded = encode_to_vec(&s).expect("encode 252-char string");
    // varint(252) uses 3-byte encoding: 0xFB marker + LE u16(252) = [0xFB, 0xFC, 0x00]
    assert_eq!(
        encoded.len(),
        255,
        "252-char string must encode to 255 bytes (3 varint + 252 content)"
    );
    assert_eq!(
        encoded[0], 0xFB,
        "first byte must be 0xFB multi-byte varint marker"
    );
    let (decoded, consumed): (String, _) =
        decode_from_slice(&encoded).expect("decode 252-char string");
    assert_eq!(decoded, s);
    assert_eq!(consumed, 255);
}

// ---------------------------------------------------------------------------
// 21. String with only non-ASCII (all 4-byte Unicode chars)
// ---------------------------------------------------------------------------

/// A string composed exclusively of 4-byte UTF-8 codepoints must roundtrip correctly.
#[test]
fn test_string_only_four_byte_unicode_roundtrip() {
    // U+1F600..=U+1F609 are smiley emoji, each 4 bytes in UTF-8
    let s: String = ('\u{1F600}'..='\u{1F609}').collect(); // 10 emoji = 40 bytes
    assert_eq!(s.chars().count(), 10, "must have 10 codepoints");
    assert_eq!(s.len(), 40, "10 × 4-byte chars = 40 UTF-8 bytes");
    assert!(!s.is_ascii(), "string must be non-ASCII only");
    let encoded = encode_to_vec(&s).expect("encode all-4-byte-Unicode string");
    // varint(40) = 1 byte + 40 content bytes = 41 bytes
    assert_eq!(
        encoded.len(),
        41,
        "all-4-byte-unicode string must encode to 41 bytes"
    );
    let (decoded, consumed): (String, _) =
        decode_from_slice(&encoded).expect("decode all-4-byte-Unicode string");
    assert_eq!(
        decoded, s,
        "4-byte-only Unicode string must survive roundtrip"
    );
    assert_eq!(decoded.chars().count(), 10);
    assert_eq!(consumed, 41);
}

// ---------------------------------------------------------------------------
// 22. Multiple strings encoded and decoded sequentially from same buffer
// ---------------------------------------------------------------------------

/// Sequential encode-then-decode of four strings from a single concatenated buffer.
#[test]
fn test_multiple_strings_sequential_from_same_buffer() {
    let strings: &[&str] = &[
        "alpha",
        "",
        "γεια σου κόσμε", // Greek "hello world"
        "🌍🌎🌏",
    ];

    // Encode all strings end-to-end into one buffer
    let mut buf = Vec::new();
    for s in strings {
        let part = encode_to_vec(&s.to_string())
            .unwrap_or_else(|e| panic!("encode {:?} failed: {}", s, e));
        buf.extend_from_slice(&part);
    }

    // Sequentially decode from the shared buffer using consumed offsets
    let mut offset = 0usize;
    for &expected in strings {
        let (decoded, consumed): (String, _) = decode_from_slice(&buf[offset..])
            .unwrap_or_else(|e| panic!("decode at offset {} failed: {}", offset, e));
        assert_eq!(
            decoded, expected,
            "decoded string at offset {} must match {:?}",
            offset, expected
        );
        offset += consumed;
    }
    assert_eq!(
        offset,
        buf.len(),
        "total consumed bytes must equal full buffer length"
    );
}
