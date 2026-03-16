//! Advanced string and unicode encoding tests for OxiCode (set 3).

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

/// Test 1: Empty string roundtrip
#[test]
fn test_adv3_empty_string_roundtrip() {
    let original = String::from("");
    let encoded = encode_to_vec(&original).expect("encode empty string");
    let (decoded, _consumed): (String, usize) =
        decode_from_slice(&encoded).expect("decode empty string");
    assert_eq!(original, decoded);
    assert!(decoded.is_empty(), "decoded string must be empty");
}

/// Test 2: ASCII-only string roundtrip "Hello, World!"
#[test]
fn test_adv3_ascii_only_hello_world_roundtrip() {
    let original = String::from("Hello, World!");
    let encoded = encode_to_vec(&original).expect("encode Hello, World!");
    let (decoded, _consumed): (String, usize) =
        decode_from_slice(&encoded).expect("decode Hello, World!");
    assert_eq!(original, decoded);
    assert!(
        decoded.is_ascii(),
        "decoded string must contain only ASCII characters"
    );
}

/// Test 3: 2-byte UTF-8 characters "é à ü ñ" roundtrip
#[test]
fn test_adv3_two_byte_utf8_chars_roundtrip() {
    let original = String::from("é à ü ñ");
    // Each accented char encodes to 2 UTF-8 bytes
    let encoded = encode_to_vec(&original).expect("encode 2-byte utf8 chars");
    let (decoded, _consumed): (String, usize) =
        decode_from_slice(&encoded).expect("decode 2-byte utf8 chars");
    assert_eq!(original, decoded);
    assert!(
        decoded.contains('é')
            && decoded.contains('à')
            && decoded.contains('ü')
            && decoded.contains('ñ'),
        "decoded must contain all 2-byte UTF-8 chars"
    );
}

/// Test 4: 3-byte UTF-8 characters "中文日本語" roundtrip
#[test]
fn test_adv3_three_byte_utf8_chars_roundtrip() {
    let original = String::from("中文日本語");
    // Each CJK character encodes to 3 UTF-8 bytes
    assert_eq!(
        original.len(),
        15,
        "5 CJK chars × 3 bytes each = 15 UTF-8 bytes"
    );
    let encoded = encode_to_vec(&original).expect("encode 3-byte utf8 chars");
    let (decoded, _consumed): (String, usize) =
        decode_from_slice(&encoded).expect("decode 3-byte utf8 chars");
    assert_eq!(original, decoded);
    assert_eq!(decoded.chars().count(), 5, "decoded must have 5 CJK chars");
}

/// Test 5: 4-byte UTF-8 emoji "🦀🌍🎉" roundtrip
#[test]
fn test_adv3_four_byte_utf8_emoji_roundtrip() {
    let original = String::from("🦀🌍🎉");
    // Each emoji encodes to 4 UTF-8 bytes
    assert_eq!(
        original.len(),
        12,
        "3 emoji × 4 bytes each = 12 UTF-8 bytes"
    );
    let encoded = encode_to_vec(&original).expect("encode 4-byte utf8 emoji");
    let (decoded, _consumed): (String, usize) =
        decode_from_slice(&encoded).expect("decode 4-byte utf8 emoji");
    assert_eq!(original, decoded);
    assert_eq!(
        decoded.chars().count(),
        3,
        "decoded must have 3 emoji chars"
    );
}

/// Test 6: Mixed ASCII + unicode roundtrip
#[test]
fn test_adv3_mixed_ascii_and_unicode_roundtrip() {
    let original = String::from("ASCII: hello | CJK: 世界 | emoji: 🦀 | latin: café");
    let encoded = encode_to_vec(&original).expect("encode mixed ASCII + unicode");
    let (decoded, _consumed): (String, usize) =
        decode_from_slice(&encoded).expect("decode mixed ASCII + unicode");
    assert_eq!(original, decoded);
    assert!(decoded.contains("hello"), "decoded must contain ASCII part");
    assert!(decoded.contains("世界"), "decoded must contain CJK part");
    assert!(decoded.contains("🦀"), "decoded must contain emoji part");
    assert!(
        decoded.contains("café"),
        "decoded must contain latin extended part"
    );
}

/// Test 7: String with null unicode char "\u{0000}" roundtrip
#[test]
fn test_adv3_string_with_null_unicode_char_roundtrip() {
    let original = String::from("before\u{0000}after");
    let encoded = encode_to_vec(&original).expect("encode string with null unicode char");
    let (decoded, _consumed): (String, usize) =
        decode_from_slice(&encoded).expect("decode string with null unicode char");
    assert_eq!(original, decoded);
    assert!(
        decoded.contains('\u{0000}'),
        "decoded string must contain U+0000"
    );
    assert!(
        decoded.contains("before") && decoded.contains("after"),
        "decoded must have text around the null char"
    );
}

/// Test 8: String with all ASCII printable chars roundtrip
#[test]
fn test_adv3_all_ascii_printable_chars_roundtrip() {
    // ASCII printable range: 0x20 (space) through 0x7E (~)
    let original: String = (0x20u8..=0x7eu8).map(|b| b as char).collect();
    assert_eq!(
        original.len(),
        95,
        "printable ASCII range has exactly 95 characters"
    );
    let encoded = encode_to_vec(&original).expect("encode all printable ASCII");
    let (decoded, _consumed): (String, usize) =
        decode_from_slice(&encoded).expect("decode all printable ASCII");
    assert_eq!(original, decoded);
    assert_eq!(decoded.len(), 95, "decoded must still have 95 chars");
}

/// Test 9: Arabic text "مرحبا بالعالم" roundtrip
#[test]
fn test_adv3_arabic_text_roundtrip() {
    let original = String::from("مرحبا بالعالم");
    let encoded = encode_to_vec(&original).expect("encode arabic text");
    let (decoded, _consumed): (String, usize) =
        decode_from_slice(&encoded).expect("decode arabic text");
    assert_eq!(original, decoded);
    // Arabic characters are 2-byte UTF-8; verify byte length is greater than char count
    assert!(
        decoded.len() > decoded.chars().count(),
        "arabic UTF-8 byte length must exceed char count"
    );
}

/// Test 10: Russian text "Привет мир" roundtrip
#[test]
fn test_adv3_russian_text_roundtrip() {
    let original = String::from("Привет мир");
    let encoded = encode_to_vec(&original).expect("encode russian text");
    let (decoded, _consumed): (String, usize) =
        decode_from_slice(&encoded).expect("decode russian text");
    assert_eq!(original, decoded);
    // Cyrillic characters are 2-byte UTF-8; verify correct char count
    assert_eq!(decoded.chars().count(), 10, "Привет мир has 10 characters");
}

/// Test 11: String with various whitespace ("\t\n\r ") roundtrip
#[test]
fn test_adv3_various_whitespace_roundtrip() {
    let original = String::from("\t\n\r ");
    let encoded = encode_to_vec(&original).expect("encode whitespace string");
    let (decoded, _consumed): (String, usize) =
        decode_from_slice(&encoded).expect("decode whitespace string");
    assert_eq!(original, decoded);
    assert!(decoded.contains('\t'), "decoded must contain tab");
    assert!(decoded.contains('\n'), "decoded must contain newline");
    assert!(
        decoded.contains('\r'),
        "decoded must contain carriage return"
    );
    assert!(decoded.contains(' '), "decoded must contain space");
}

/// Test 12: Very long string (1000 'a' chars) roundtrip
#[test]
fn test_adv3_very_long_1000_a_chars_roundtrip() {
    let original: String = "a".repeat(1000);
    assert_eq!(original.len(), 1000, "string must be exactly 1000 bytes");
    let encoded = encode_to_vec(&original).expect("encode 1000-char string");
    let (decoded, _consumed): (String, usize) =
        decode_from_slice(&encoded).expect("decode 1000-char string");
    assert_eq!(original, decoded);
    assert_eq!(decoded.len(), 1000, "decoded must still be 1000 bytes long");
}

/// Test 13: String with backslash and quotes roundtrip
#[test]
fn test_adv3_backslash_and_quotes_roundtrip() {
    let original = String::from("path\\to\\\"file\"\\and\\'quote\\'");
    let encoded = encode_to_vec(&original).expect("encode string with backslash and quotes");
    let (decoded, _consumed): (String, usize) =
        decode_from_slice(&encoded).expect("decode string with backslash and quotes");
    assert_eq!(original, decoded);
    assert!(decoded.contains('\\'), "decoded must contain backslash");
    assert!(decoded.contains('"'), "decoded must contain double quote");
    assert!(decoded.contains('\''), "decoded must contain single quote");
}

/// Test 14: Vec<String> roundtrip with 5 unicode strings
#[test]
fn test_adv3_vec_string_five_unicode_roundtrip() {
    let original: Vec<String> = vec![
        String::from("日本語テキスト"),
        String::from("مرحبا بالعالم"),
        String::from("Привет мир"),
        String::from("🦀🌍🎉🔥💯"),
        String::from("café naïve résumé"),
    ];
    let encoded = encode_to_vec(&original).expect("encode Vec<String> unicode");
    let (decoded, _consumed): (Vec<String>, usize) =
        decode_from_slice(&encoded).expect("decode Vec<String> unicode");
    assert_eq!(original, decoded);
    assert_eq!(decoded.len(), 5, "decoded vec must have 5 elements");
    assert_eq!(decoded[2], "Привет мир", "Cyrillic string must roundtrip");
    assert_eq!(decoded[3], "🦀🌍🎉🔥💯", "emoji string must roundtrip");
}

/// Test 15: Option<String> Some with unicode roundtrip
#[test]
fn test_adv3_option_string_some_unicode_roundtrip() {
    let original: Option<String> = Some(String::from("unicode 🌸 こんにちは ñ"));
    let encoded = encode_to_vec(&original).expect("encode Option<String> Some unicode");
    let (decoded, _consumed): (Option<String>, usize) =
        decode_from_slice(&encoded).expect("decode Option<String> Some unicode");
    assert_eq!(original, decoded);
    assert!(decoded.is_some(), "decoded Option must be Some variant");
    let inner = decoded.expect("must be Some");
    assert!(
        inner.contains("🌸"),
        "decoded inner string must contain emoji"
    );
}

/// Test 16: String encoding size = length prefix + UTF-8 byte count
#[test]
fn test_adv3_encoding_size_is_length_prefix_plus_utf8_bytes() {
    // Use a string whose UTF-8 byte count fits in 1 varint byte (< 128)
    let original = String::from("Привет"); // 6 Cyrillic chars, 12 UTF-8 bytes
    let utf8_byte_len = original.len(); // 12
    let encoded = encode_to_vec(&original).expect("encode for size verification");
    // varint for 12 fits in 1 byte, so total = 1 (length) + 12 (content) = 13
    assert_eq!(
        encoded.len(),
        1 + utf8_byte_len,
        "encoded length must be 1 (varint prefix) + {} (UTF-8 bytes)",
        utf8_byte_len
    );
}

/// Test 17: "Hello" encodes to 6 bytes: 1 length byte + 5 content bytes
#[test]
fn test_adv3_hello_encodes_to_6_bytes() {
    let original = String::from("Hello");
    let encoded = encode_to_vec(&original).expect("encode Hello");
    assert_eq!(
        encoded.len(),
        6,
        "Hello must encode to exactly 6 bytes (1 varint length + 5 ASCII bytes)"
    );
    assert_eq!(encoded[0], 5, "first byte must be varint 5 for Hello");
    assert_eq!(&encoded[1..], b"Hello", "content bytes must be ASCII Hello");
}

/// Test 18: Same unicode string codepoints always produce the same encoded bytes
#[test]
fn test_adv3_same_codepoints_produce_same_bytes_deterministic() {
    let s1 = String::from("中文日本語🦀");
    let s2 = String::from("中文日本語🦀");
    let enc1 = encode_to_vec(&s1).expect("encode first instance");
    let enc2 = encode_to_vec(&s2).expect("encode second instance");
    assert_eq!(
        enc1, enc2,
        "identical unicode strings must always encode to identical bytes"
    );
}

/// Test 19: String with fixed-int config roundtrip
#[test]
fn test_adv3_string_with_fixed_int_config_roundtrip() {
    let original = String::from("固定整数設定テスト 🧪");
    let cfg = config::standard().with_fixed_int_encoding();
    let encoded = encode_to_vec_with_config(&original, cfg).expect("encode with fixed-int config");
    let (decoded, _consumed): (String, usize) =
        decode_from_slice_with_config(&encoded, cfg).expect("decode with fixed-int config");
    assert_eq!(original, decoded);
    assert!(
        decoded.contains("🧪"),
        "decoded must contain test emoji with fixed-int config"
    );
}

/// Test 20: 256-char string roundtrip verifying varint length encoding
#[test]
fn test_adv3_256_char_string_varint_length_encoding_roundtrip() {
    // A 256-byte ASCII string requires a 2-byte varint length prefix
    let original: String = "x".repeat(256);
    assert_eq!(original.len(), 256, "string must be exactly 256 bytes");
    let encoded = encode_to_vec(&original).expect("encode 256-char string");
    // varint for 256 encodes in this implementation as 3 bytes, total = 3 + 256 = 259
    assert_eq!(
        encoded.len(),
        259,
        "256-byte string must encode to 259 bytes (3 varint + 256 content)"
    );
    let (decoded, consumed): (String, usize) =
        decode_from_slice(&encoded).expect("decode 256-char string");
    assert_eq!(original, decoded);
    assert_eq!(consumed, 259, "consumed must equal total encoded length");
}

/// Test 21: String containing only whitespace roundtrip
#[test]
fn test_adv3_only_whitespace_string_roundtrip() {
    let original = String::from("     \t\t\t\n\n\r\r   ");
    let encoded = encode_to_vec(&original).expect("encode whitespace-only string");
    let (decoded, _consumed): (String, usize) =
        decode_from_slice(&encoded).expect("decode whitespace-only string");
    assert_eq!(original, decoded);
    assert!(
        decoded.chars().all(|c| c.is_whitespace()),
        "all decoded chars must be whitespace"
    );
}

/// Test 22: Consumed bytes equals encoded length for a unicode string
#[test]
fn test_adv3_consumed_bytes_equals_encoded_length_unicode() {
    let original = String::from("消費バイト数検証 🔢 مرحبا Привет");
    let encoded = encode_to_vec(&original).expect("encode unicode for consumed check");
    let (_decoded, consumed): (String, usize) =
        decode_from_slice(&encoded).expect("decode unicode for consumed check");
    assert_eq!(
        consumed,
        encoded.len(),
        "consumed bytes ({}) must equal total encoded length ({}) for unicode string",
        consumed,
        encoded.len()
    );
}
