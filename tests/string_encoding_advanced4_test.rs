//! Advanced string and char encoding edge cases — multi-language text processing.

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
    encode_to_vec_with_config, Decode, Encode,
};

#[derive(Debug, PartialEq, Encode, Decode)]
enum Script {
    Latin,
    CyrillicScript,
    Arabic,
    Hebrew,
    Chinese,
    Japanese,
    Korean,
    Emoji,
    Mixed,
}

#[derive(Debug, PartialEq, Encode, Decode)]
struct TextSample {
    content: String,
    script: Script,
    byte_count: u32,
    char_count: u32,
}

// Test 1: Empty string roundtrip
#[test]
fn test_empty_string_roundtrip() {
    let s = String::new();
    let encoded = encode_to_vec(&s).expect("Failed to encode empty string");
    let (decoded, _): (String, usize) =
        decode_from_slice(&encoded).expect("Failed to decode empty string");
    assert_eq!(s, decoded);
}

// Test 2: ASCII-only string roundtrip
#[test]
fn test_ascii_only_string_roundtrip() {
    let s = "The quick brown fox jumps over the lazy dog".to_string();
    let encoded = encode_to_vec(&s).expect("Failed to encode ASCII string");
    let (decoded, _): (String, usize) =
        decode_from_slice(&encoded).expect("Failed to decode ASCII string");
    assert_eq!(s, decoded);
}

// Test 3: Latin extended characters roundtrip
#[test]
fn test_latin_extended_roundtrip() {
    let s = "Ångström résumé naïve".to_string();
    let encoded = encode_to_vec(&s).expect("Failed to encode Latin extended string");
    let (decoded, _): (String, usize) =
        decode_from_slice(&encoded).expect("Failed to decode Latin extended string");
    assert_eq!(s, decoded);
}

// Test 4: Cyrillic script roundtrip
#[test]
fn test_cyrillic_roundtrip() {
    let s = "Привет мир".to_string();
    let encoded = encode_to_vec(&s).expect("Failed to encode Cyrillic string");
    let (decoded, _): (String, usize) =
        decode_from_slice(&encoded).expect("Failed to decode Cyrillic string");
    assert_eq!(s, decoded);
}

// Test 5: Arabic script roundtrip
#[test]
fn test_arabic_roundtrip() {
    let s = "مرحبا بالعالم".to_string();
    let encoded = encode_to_vec(&s).expect("Failed to encode Arabic string");
    let (decoded, _): (String, usize) =
        decode_from_slice(&encoded).expect("Failed to decode Arabic string");
    assert_eq!(s, decoded);
}

// Test 6: Hebrew script roundtrip
#[test]
fn test_hebrew_roundtrip() {
    let s = "שלום עולם".to_string();
    let encoded = encode_to_vec(&s).expect("Failed to encode Hebrew string");
    let (decoded, _): (String, usize) =
        decode_from_slice(&encoded).expect("Failed to decode Hebrew string");
    assert_eq!(s, decoded);
}

// Test 7: Chinese simplified roundtrip
#[test]
fn test_chinese_simplified_roundtrip() {
    let s = "你好世界".to_string();
    let encoded = encode_to_vec(&s).expect("Failed to encode Chinese string");
    let (decoded, _): (String, usize) =
        decode_from_slice(&encoded).expect("Failed to decode Chinese string");
    assert_eq!(s, decoded);
}

// Test 8: Japanese hiragana/katakana roundtrip
#[test]
fn test_japanese_roundtrip() {
    let s = "こんにちは世界".to_string();
    let encoded = encode_to_vec(&s).expect("Failed to encode Japanese string");
    let (decoded, _): (String, usize) =
        decode_from_slice(&encoded).expect("Failed to decode Japanese string");
    assert_eq!(s, decoded);
}

// Test 9: Korean roundtrip
#[test]
fn test_korean_roundtrip() {
    let s = "안녕하세요".to_string();
    let encoded = encode_to_vec(&s).expect("Failed to encode Korean string");
    let (decoded, _): (String, usize) =
        decode_from_slice(&encoded).expect("Failed to decode Korean string");
    assert_eq!(s, decoded);
}

// Test 10: Emoji roundtrip
#[test]
fn test_emoji_roundtrip() {
    let s = "🎉🦀🔥💯".to_string();
    let encoded = encode_to_vec(&s).expect("Failed to encode emoji string");
    let (decoded, _): (String, usize) =
        decode_from_slice(&encoded).expect("Failed to decode emoji string");
    assert_eq!(s, decoded);
}

// Test 11: Mixed scripts roundtrip
#[test]
fn test_mixed_scripts_roundtrip() {
    let s = "Hello 世界 مرحبا 🌍".to_string();
    let encoded = encode_to_vec(&s).expect("Failed to encode mixed scripts string");
    let (decoded, _): (String, usize) =
        decode_from_slice(&encoded).expect("Failed to decode mixed scripts string");
    assert_eq!(s, decoded);
}

// Test 12: String with control characters (tab, newline, carriage return)
#[test]
fn test_control_characters_roundtrip() {
    let s = "line1\tcolumn2\nline2\r\nline3".to_string();
    let encoded = encode_to_vec(&s).expect("Failed to encode control character string");
    let (decoded, _): (String, usize) =
        decode_from_slice(&encoded).expect("Failed to decode control character string");
    assert_eq!(s, decoded);
    // Verify control chars are preserved exactly
    assert!(decoded.contains('\t'));
    assert!(decoded.contains('\n'));
    assert!(decoded.contains('\r'));
}

// Test 13: Very long string (1000 chars) roundtrip
#[test]
fn test_very_long_string_roundtrip() {
    let s = "あいうえおかきくけこ".repeat(100); // 10 chars * 100 = 1000 chars
    assert_eq!(s.chars().count(), 1000);
    let encoded = encode_to_vec(&s).expect("Failed to encode long string");
    let (decoded, _): (String, usize) =
        decode_from_slice(&encoded).expect("Failed to decode long string");
    assert_eq!(s, decoded);
}

// Test 14: String with only whitespace and newlines roundtrip
#[test]
fn test_whitespace_only_string_roundtrip() {
    let s = "   \t\t  \n\n\r\n   \t   ".to_string();
    let encoded = encode_to_vec(&s).expect("Failed to encode whitespace string");
    let (decoded, _): (String, usize) =
        decode_from_slice(&encoded).expect("Failed to decode whitespace string");
    assert_eq!(s, decoded);
}

// Test 15: TextSample with Latin script roundtrip
#[test]
fn test_text_sample_latin_roundtrip() {
    let content = "Ångström résumé naïve café".to_string();
    let byte_count = content.len() as u32;
    let char_count = content.chars().count() as u32;
    let sample = TextSample {
        content,
        script: Script::Latin,
        byte_count,
        char_count,
    };
    let encoded = encode_to_vec(&sample).expect("Failed to encode Latin TextSample");
    let (decoded, _): (TextSample, usize) =
        decode_from_slice(&encoded).expect("Failed to decode Latin TextSample");
    assert_eq!(sample, decoded);
    assert_eq!(decoded.script, Script::Latin);
}

// Test 16: TextSample with Japanese script roundtrip
#[test]
fn test_text_sample_japanese_roundtrip() {
    let content = "こんにちは世界、日本語のテキストです。".to_string();
    let byte_count = content.len() as u32;
    let char_count = content.chars().count() as u32;
    let sample = TextSample {
        content,
        script: Script::Japanese,
        byte_count,
        char_count,
    };
    let encoded = encode_to_vec(&sample).expect("Failed to encode Japanese TextSample");
    let (decoded, _): (TextSample, usize) =
        decode_from_slice(&encoded).expect("Failed to decode Japanese TextSample");
    assert_eq!(sample, decoded);
    assert_eq!(decoded.script, Script::Japanese);
    // Japanese chars are 3 bytes each in UTF-8
    assert!(decoded.byte_count > decoded.char_count);
}

// Test 17: TextSample with Emoji script roundtrip
#[test]
fn test_text_sample_emoji_roundtrip() {
    // All characters here are 4-byte UTF-8 codepoints (supplementary plane)
    let content = "🎉🦀🔥💯🌍🚀🎊🏆".to_string();
    let char_count = content.chars().count() as u32;
    // Each character is exactly 4 bytes in UTF-8
    for c in content.chars() {
        assert_eq!(
            c.len_utf8(),
            4,
            "Expected all emoji to be 4-byte UTF-8, got {:?}",
            c
        );
    }
    let byte_count = content.len() as u32;
    let sample = TextSample {
        content,
        script: Script::Emoji,
        byte_count,
        char_count,
    };
    let encoded = encode_to_vec(&sample).expect("Failed to encode Emoji TextSample");
    let (decoded, _): (TextSample, usize) =
        decode_from_slice(&encoded).expect("Failed to decode Emoji TextSample");
    assert_eq!(sample, decoded);
    assert_eq!(decoded.script, Script::Emoji);
    // All emoji are 4 bytes each in UTF-8, so byte_count == char_count * 4
    assert_eq!(decoded.byte_count, decoded.char_count * 4);
}

// Test 18: Vec<TextSample> with 5 different scripts roundtrip
#[test]
fn test_vec_text_sample_multiple_scripts_roundtrip() {
    let samples: Vec<TextSample> = vec![
        {
            let c = "Hello world".to_string();
            let b = c.len() as u32;
            let n = c.chars().count() as u32;
            TextSample {
                content: c,
                script: Script::Latin,
                byte_count: b,
                char_count: n,
            }
        },
        {
            let c = "Привет мир".to_string();
            let b = c.len() as u32;
            let n = c.chars().count() as u32;
            TextSample {
                content: c,
                script: Script::CyrillicScript,
                byte_count: b,
                char_count: n,
            }
        },
        {
            let c = "你好世界".to_string();
            let b = c.len() as u32;
            let n = c.chars().count() as u32;
            TextSample {
                content: c,
                script: Script::Chinese,
                byte_count: b,
                char_count: n,
            }
        },
        {
            let c = "안녕하세요".to_string();
            let b = c.len() as u32;
            let n = c.chars().count() as u32;
            TextSample {
                content: c,
                script: Script::Korean,
                byte_count: b,
                char_count: n,
            }
        },
        {
            let c = "🎉🦀🔥💯🌍".to_string();
            let b = c.len() as u32;
            let n = c.chars().count() as u32;
            TextSample {
                content: c,
                script: Script::Emoji,
                byte_count: b,
                char_count: n,
            }
        },
    ];
    let encoded = encode_to_vec(&samples).expect("Failed to encode Vec<TextSample>");
    let (decoded, _): (Vec<TextSample>, usize) =
        decode_from_slice(&encoded).expect("Failed to decode Vec<TextSample>");
    assert_eq!(samples, decoded);
    assert_eq!(decoded.len(), 5);
}

// Test 19: Big-endian config TextSample roundtrip
#[test]
fn test_big_endian_config_text_sample_roundtrip() {
    let content = "مرحبا بالعالم".to_string();
    let byte_count = content.len() as u32;
    let char_count = content.chars().count() as u32;
    let sample = TextSample {
        content,
        script: Script::Arabic,
        byte_count,
        char_count,
    };
    let cfg = config::standard().with_big_endian();
    let encoded =
        encode_to_vec_with_config(&sample, cfg).expect("Failed to encode Arabic TextSample (BE)");
    let (decoded, _): (TextSample, usize) = decode_from_slice_with_config(&encoded, cfg)
        .expect("Failed to decode Arabic TextSample (BE)");
    assert_eq!(sample, decoded);
}

// Test 20: Fixed-int config TextSample roundtrip
#[test]
fn test_fixed_int_config_text_sample_roundtrip() {
    let content = "שלום עולם".to_string();
    let byte_count = content.len() as u32;
    let char_count = content.chars().count() as u32;
    let sample = TextSample {
        content,
        script: Script::Hebrew,
        byte_count,
        char_count,
    };
    let cfg = config::standard().with_fixed_int_encoding();
    let encoded =
        encode_to_vec_with_config(&sample, cfg).expect("Failed to encode Hebrew TextSample (FI)");
    let (decoded, _): (TextSample, usize) = decode_from_slice_with_config(&encoded, cfg)
        .expect("Failed to decode Hebrew TextSample (FI)");
    assert_eq!(sample, decoded);
}

// Test 21: Consumed bytes == total encoded bytes for ASCII string
#[test]
fn test_consumed_bytes_equals_total_encoded_for_ascii() {
    let s = "hello world".to_string();
    let encoded = encode_to_vec(&s).expect("Failed to encode ASCII for consumed bytes test");
    let total_len = encoded.len();
    let (_, consumed): (String, usize) =
        decode_from_slice(&encoded).expect("Failed to decode ASCII for consumed bytes test");
    assert_eq!(
        consumed, total_len,
        "Consumed bytes ({consumed}) should equal total encoded length ({total_len})"
    );
}

// Test 22: Unicode string — char count != byte count (UTF-8 multibyte verification)
#[test]
fn test_unicode_char_count_differs_from_byte_count() {
    // Each CJK character is 3 bytes in UTF-8; emojis are 4 bytes
    let s = "你好🦀".to_string();
    let char_count = s.chars().count(); // 3
    let byte_count = s.len(); // 3+3+4 = 10

    assert_ne!(
        char_count, byte_count,
        "char count and byte count must differ for Unicode"
    );
    assert_eq!(char_count, 3);
    assert_eq!(byte_count, 10);

    let encoded = encode_to_vec(&s).expect("Failed to encode Unicode string");
    let (decoded, _): (String, usize) =
        decode_from_slice(&encoded).expect("Failed to decode Unicode string");

    assert_eq!(s, decoded);
    // Confirm the decoded string preserves the multi-byte character boundary semantics
    assert_eq!(decoded.chars().count(), 3);
    assert_eq!(decoded.len(), 10);
}
