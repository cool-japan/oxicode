//! Advanced string encoding edge case tests (set 3) — 22 tests.

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

// Test 1: Empty string roundtrip
#[test]
fn test_adv3_empty_string_roundtrip() {
    let original = String::new();
    let encoded = encode_to_vec(&original).expect("encode empty string");
    let (decoded, _): (String, _) = decode_from_slice(&encoded).expect("decode empty string");
    assert_eq!(original, decoded);
}

// Test 2: ASCII string roundtrip
#[test]
fn test_adv3_ascii_string_roundtrip() {
    let original = String::from("Hello, World!");
    let encoded = encode_to_vec(&original).expect("encode ascii string");
    let (decoded, _): (String, _) = decode_from_slice(&encoded).expect("decode ascii string");
    assert_eq!(original, decoded);
}

// Test 3: String with spaces and punctuation
#[test]
fn test_adv3_string_spaces_and_punctuation_roundtrip() {
    let original = String::from("  Hello,   World!  How are you? Fine; thanks. Let's go: now!");
    let encoded = encode_to_vec(&original).expect("encode spaces/punctuation string");
    let (decoded, _): (String, _) =
        decode_from_slice(&encoded).expect("decode spaces/punctuation string");
    assert_eq!(original, decoded);
}

// Test 4: String with CJK characters (Chinese/Japanese/Korean)
#[test]
fn test_adv3_cjk_characters_roundtrip() {
    let original = String::from("中文日本語한국어 — CJK: 中日韩统一表意文字");
    let encoded = encode_to_vec(&original).expect("encode CJK string");
    let (decoded, _): (String, _) = decode_from_slice(&encoded).expect("decode CJK string");
    assert_eq!(original, decoded);
}

// Test 5: String with emoji 🦀🚀🌍
#[test]
fn test_adv3_emoji_string_roundtrip() {
    let original = String::from("Rust crab 🦀 and rocket 🚀 and earth 🌍!");
    let encoded = encode_to_vec(&original).expect("encode emoji string");
    let (decoded, _): (String, _) = decode_from_slice(&encoded).expect("decode emoji string");
    assert_eq!(original, decoded);
}

// Test 6: String with Arabic script (RTL)
#[test]
fn test_adv3_arabic_rtl_string_roundtrip() {
    let original = String::from("مرحبا بالعالم — صباح الخير — سلام دنیا");
    let encoded = encode_to_vec(&original).expect("encode Arabic RTL string");
    let (decoded, _): (String, _) = decode_from_slice(&encoded).expect("decode Arabic RTL string");
    assert_eq!(original, decoded);
}

// Test 7: String with null bytes (use "\0" in middle)
#[test]
fn test_adv3_null_bytes_in_string_roundtrip() {
    let original = String::from("before\0middle\0after");
    let encoded = encode_to_vec(&original).expect("encode null-byte string");
    let (decoded, _): (String, _) = decode_from_slice(&encoded).expect("decode null-byte string");
    assert_eq!(original, decoded);
    assert!(
        decoded.contains('\0'),
        "decoded string should contain null bytes"
    );
}

// Test 8: String with newlines/tabs
#[test]
fn test_adv3_newlines_tabs_string_roundtrip() {
    let original = String::from("line1\nline2\r\nline3\ttabbed\t\nend");
    let encoded = encode_to_vec(&original).expect("encode newlines/tabs string");
    let (decoded, _): (String, _) =
        decode_from_slice(&encoded).expect("decode newlines/tabs string");
    assert_eq!(original, decoded);
}

// Test 9: String with 1000 ASCII chars
#[test]
fn test_adv3_1000_ascii_chars_roundtrip() {
    let original: String = (0..1000).map(|i| (b'a' + (i % 26) as u8) as char).collect();
    assert_eq!(original.len(), 1000);
    let encoded = encode_to_vec(&original).expect("encode 1000-char ASCII string");
    let (decoded, _): (String, _) =
        decode_from_slice(&encoded).expect("decode 1000-char ASCII string");
    assert_eq!(original, decoded);
}

// Test 10: String with 1000 Unicode chars (mix of multi-byte)
#[test]
fn test_adv3_1000_unicode_chars_roundtrip() {
    let chars: Vec<char> = vec!['A', 'é', '中', 'ñ', '€', 'α', 'β', '日', '한', '™'];
    let original: String = (0..1000).map(|i| chars[i % chars.len()]).collect();
    assert_eq!(original.chars().count(), 1000);
    let encoded = encode_to_vec(&original).expect("encode 1000-unicode-char string");
    let (decoded, _): (String, _) =
        decode_from_slice(&encoded).expect("decode 1000-unicode-char string");
    assert_eq!(original, decoded);
}

// Test 11: String encoding starts with length prefix (check first bytes)
#[test]
fn test_adv3_string_encoding_has_length_prefix() {
    let original = String::from("Hello");
    let encoded = encode_to_vec(&original).expect("encode string for prefix check");
    // With standard (varint) config, lengths < 128 fit in one byte value equal to the length.
    assert!(!encoded.is_empty(), "encoded bytes should not be empty");
    assert_eq!(
        encoded[0], 5,
        "first byte should be the varint-encoded length 5"
    );
    assert_eq!(&encoded[1..], b"Hello");
}

// Test 12: String consumed bytes equals encoded length
#[test]
fn test_adv3_consumed_bytes_equals_encoded_length() {
    let original = String::from("oxicode string length check");
    let encoded = encode_to_vec(&original).expect("encode string for consumed check");
    let total_len = encoded.len();
    let (decoded, consumed): (String, _) =
        decode_from_slice(&encoded).expect("decode string for consumed check");
    assert_eq!(original, decoded);
    assert_eq!(
        consumed, total_len,
        "consumed bytes should equal total encoded length"
    );
}

// Test 13: Vec<String> roundtrip (10 strings)
#[test]
fn test_adv3_vec_string_10_roundtrip() {
    let original: Vec<String> = (0..10)
        .map(|i| format!("string number {} with some content", i))
        .collect();
    let encoded = encode_to_vec(&original).expect("encode Vec<String> 10");
    let (decoded, _): (Vec<String>, _) =
        decode_from_slice(&encoded).expect("decode Vec<String> 10");
    assert_eq!(original, decoded);
    assert_eq!(decoded.len(), 10);
}

// Test 14: Vec<String> with empty strings mixed in
#[test]
fn test_adv3_vec_string_with_empty_strings_roundtrip() {
    let original: Vec<String> = vec![
        String::from("first"),
        String::new(),
        String::from("third"),
        String::new(),
        String::new(),
        String::from("sixth"),
    ];
    let encoded = encode_to_vec(&original).expect("encode Vec<String> with empties");
    let (decoded, _): (Vec<String>, _) =
        decode_from_slice(&encoded).expect("decode Vec<String> with empties");
    assert_eq!(original, decoded);
    assert_eq!(decoded[1], "");
    assert_eq!(decoded[3], "");
    assert_eq!(decoded[4], "");
}

// Test 15: Option<String> Some roundtrip
#[test]
fn test_adv3_option_string_some_roundtrip() {
    let original: Option<String> = Some(String::from("optional content here"));
    let encoded = encode_to_vec(&original).expect("encode Option<String> Some");
    let (decoded, _): (Option<String>, _) =
        decode_from_slice(&encoded).expect("decode Option<String> Some");
    assert_eq!(original, decoded);
    assert!(decoded.is_some());
}

// Test 16: Option<String> None roundtrip
#[test]
fn test_adv3_option_string_none_roundtrip() {
    let original: Option<String> = None;
    let encoded = encode_to_vec(&original).expect("encode Option<String> None");
    let (decoded, _): (Option<String>, _) =
        decode_from_slice(&encoded).expect("decode Option<String> None");
    assert_eq!(original, decoded);
    assert!(decoded.is_none());
}

// Test 17: String with fixed-int config roundtrip
#[test]
fn test_adv3_string_fixed_int_config_roundtrip() {
    let original = String::from("fixed-int config test string");
    let fixed_cfg = config::standard().with_fixed_int_encoding();
    let encoded =
        encode_to_vec_with_config(&original, fixed_cfg).expect("encode string fixed-int config");
    let (decoded, _): (String, _) =
        decode_from_slice_with_config(&encoded, fixed_cfg).expect("decode string fixed-int config");
    assert_eq!(original, decoded);
}

// Test 18: Very long string (5000 chars)
#[test]
fn test_adv3_very_long_string_5000_chars_roundtrip() {
    let original: String = "abcdefghijklmnopqrstuvwxyz0123456789"
        .chars()
        .cycle()
        .take(5000)
        .collect();
    assert_eq!(original.len(), 5000);
    let encoded = encode_to_vec(&original).expect("encode 5000-char string");
    let (decoded, _): (String, _) = decode_from_slice(&encoded).expect("decode 5000-char string");
    assert_eq!(original, decoded);
}

// Test 19: String with all printable ASCII chars (32-126)
#[test]
fn test_adv3_all_printable_ascii_roundtrip() {
    let original: String = (32u8..=126u8).map(|b| b as char).collect();
    assert_eq!(original.len(), 95, "should have 95 printable ASCII chars");
    let encoded = encode_to_vec(&original).expect("encode printable ASCII string");
    let (decoded, _): (String, _) =
        decode_from_slice(&encoded).expect("decode printable ASCII string");
    assert_eq!(original, decoded);
}

// Test 20: String with 4-byte UTF-8 sequences (supplementary planes)
#[test]
fn test_adv3_4_byte_utf8_sequences_roundtrip() {
    // Supplementary plane characters: U+1F600, U+1D11E, U+10000, U+1F3B5, U+1F98A
    let original = String::from("😀 𝄞 𐀀 🎵 🦊 💯 🐉 🌺 🎨 🚀");
    for ch in original.chars().filter(|c| !c.is_whitespace()) {
        assert!(
            ch as u32 > 0xFFFF,
            "char U+{:04X} should be in supplementary plane",
            ch as u32
        );
    }
    let encoded = encode_to_vec(&original).expect("encode 4-byte UTF-8 string");
    let (decoded, _): (String, _) =
        decode_from_slice(&encoded).expect("decode 4-byte UTF-8 string");
    assert_eq!(original, decoded);
}

// Test 21: Different strings produce different encodings
#[test]
fn test_adv3_different_strings_produce_different_encodings() {
    let s1 = String::from("alpha");
    let s2 = String::from("beta");
    let s3 = String::from("Alpha");
    let s4 = String::new();

    let enc1 = encode_to_vec(&s1).expect("encode s1");
    let enc2 = encode_to_vec(&s2).expect("encode s2");
    let enc3 = encode_to_vec(&s3).expect("encode s3");
    let enc4 = encode_to_vec(&s4).expect("encode s4");

    assert_ne!(enc1, enc2, "alpha and beta should encode differently");
    assert_ne!(enc1, enc3, "alpha and Alpha should encode differently");
    assert_ne!(enc1, enc4, "alpha and empty should encode differently");
    assert_ne!(enc2, enc3, "beta and Alpha should encode differently");
    assert_ne!(enc2, enc4, "beta and empty should encode differently");
    assert_ne!(enc3, enc4, "Alpha and empty should encode differently");
}

// Test 22: String with mathematical symbols (∑∫∂π)
#[test]
fn test_adv3_mathematical_symbols_roundtrip() {
    let original = String::from("∑∫∂π — ∀x∈ℝ, ∃y: y=√x² — ≤ ≥ ≠ ∞ ∝ ∇ ∈ ∉ ⊂ ⊃ ∪ ∩");
    let encoded = encode_to_vec(&original).expect("encode mathematical symbols string");
    let (decoded, _): (String, _) =
        decode_from_slice(&encoded).expect("decode mathematical symbols string");
    assert_eq!(original, decoded);
    assert!(decoded.contains('∑'));
    assert!(decoded.contains('∫'));
    assert!(decoded.contains('∂'));
    assert!(decoded.contains('π'));
}
