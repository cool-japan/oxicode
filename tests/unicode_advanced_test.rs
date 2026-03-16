//! Comprehensive Unicode encoding tests covering a wide range of scripts,
//! emoji, mixed content, collections with unicode keys, and encoding invariants.

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
use oxicode::{config, decode_from_slice, encode_to_vec, encoded_size};

mod unicode_advanced_tests {
    use super::*;
    use std::collections::BTreeMap;

    // -----------------------------------------------------------------------
    // Helper: varint overhead in bytes for a given length value
    // -----------------------------------------------------------------------
    fn varint_prefix_len(n: usize) -> usize {
        if n <= 250 {
            1
        } else if n <= 0xffff {
            3 // 1 tag byte + 2 data bytes (U16_BYTE encoding)
        } else if n <= 0xffff_ffff {
            5 // 1 tag byte + 4 data bytes (U32_BYTE encoding)
        } else {
            9 // 1 tag byte + 8 data bytes (U64_BYTE encoding)
        }
    }

    // -----------------------------------------------------------------------
    // 1. ASCII-only string "hello world" roundtrip
    // -----------------------------------------------------------------------
    #[test]
    fn test_ascii_hello_world_roundtrip() {
        let s = "hello world".to_string();
        let enc = encode_to_vec(&s).expect("encode hello world");
        let (dec, consumed): (String, _) = decode_from_slice(&enc).expect("decode hello world");
        assert_eq!(s, dec, "ascii string must roundtrip identically");
        assert_eq!(consumed, enc.len(), "all bytes must be consumed");
        // For pure ASCII, byte length == char count
        assert_eq!(s.len(), s.chars().count());
    }

    // -----------------------------------------------------------------------
    // 2. Latin extended: "café résumé naïve" roundtrip
    // -----------------------------------------------------------------------
    #[test]
    fn test_latin_extended_roundtrip() {
        let s = "café résumé naïve".to_string();
        let enc = encode_to_vec(&s).expect("encode latin extended");
        let (dec, consumed): (String, _) = decode_from_slice(&enc).expect("decode latin extended");
        assert_eq!(s, dec, "latin extended chars must roundtrip");
        assert_eq!(consumed, enc.len());
        // Accented characters are multi-byte in UTF-8
        assert!(
            s.len() > s.chars().count(),
            "accented chars are multi-byte in UTF-8"
        );
    }

    // -----------------------------------------------------------------------
    // 3. Greek alphabet: "αβγδεζηθικλμ" roundtrip
    // -----------------------------------------------------------------------
    #[test]
    fn test_greek_alphabet_roundtrip() {
        let s = "αβγδεζηθικλμ".to_string();
        let enc = encode_to_vec(&s).expect("encode Greek");
        let (dec, consumed): (String, _) = decode_from_slice(&enc).expect("decode Greek");
        assert_eq!(s, dec, "Greek characters must roundtrip");
        assert_eq!(consumed, enc.len());
        // Each Greek letter is 2 bytes in UTF-8 (U+03B1..U+03BC range)
        for ch in s.chars() {
            assert_eq!(
                ch.len_utf8(),
                2,
                "Greek char {:?} should be 2 bytes in UTF-8",
                ch
            );
        }
    }

    // -----------------------------------------------------------------------
    // 4. Cyrillic: "привет мир" roundtrip
    // -----------------------------------------------------------------------
    #[test]
    fn test_cyrillic_roundtrip() {
        let s = "привет мир".to_string();
        let enc = encode_to_vec(&s).expect("encode Cyrillic");
        let (dec, consumed): (String, _) = decode_from_slice(&enc).expect("decode Cyrillic");
        assert_eq!(s, dec, "Cyrillic must roundtrip");
        assert_eq!(consumed, enc.len());
        // Cyrillic letters: 2 bytes each; space: 1 byte
        let cyrillic_chars = s.chars().filter(|c| *c != ' ').count();
        let space_count = s.chars().filter(|c| *c == ' ').count();
        assert_eq!(s.len(), cyrillic_chars * 2 + space_count);
    }

    // -----------------------------------------------------------------------
    // 5. Chinese simplified: "你好世界" roundtrip
    // -----------------------------------------------------------------------
    #[test]
    fn test_chinese_simplified_roundtrip() {
        let s = "你好世界".to_string();
        let enc = encode_to_vec(&s).expect("encode Chinese simplified");
        let (dec, consumed): (String, _) =
            decode_from_slice(&enc).expect("decode Chinese simplified");
        assert_eq!(s, dec, "Chinese simplified must roundtrip");
        assert_eq!(consumed, enc.len());
        // CJK Unified Ideographs are 3 bytes in UTF-8
        assert_eq!(s.len(), 4 * 3, "4 CJK chars × 3 bytes = 12 bytes");
        assert_eq!(s.chars().count(), 4);
    }

    // -----------------------------------------------------------------------
    // 6. Japanese hiragana: "ひらがな" roundtrip
    // -----------------------------------------------------------------------
    #[test]
    fn test_japanese_hiragana_short_roundtrip() {
        let s = "ひらがな".to_string();
        let enc = encode_to_vec(&s).expect("encode hiragana");
        let (dec, consumed): (String, _) = decode_from_slice(&enc).expect("decode hiragana");
        assert_eq!(s, dec, "hiragana must roundtrip");
        assert_eq!(consumed, enc.len());
        // Hiragana: U+3041-U+3096, each 3 bytes in UTF-8
        assert_eq!(s.len(), 4 * 3, "4 hiragana × 3 bytes = 12 bytes");
        assert_eq!(s.chars().count(), 4);
    }

    // -----------------------------------------------------------------------
    // 7. Japanese katakana: "カタカナ" roundtrip
    // -----------------------------------------------------------------------
    #[test]
    fn test_japanese_katakana_roundtrip() {
        let s = "カタカナ".to_string();
        let enc = encode_to_vec(&s).expect("encode katakana");
        let (dec, consumed): (String, _) = decode_from_slice(&enc).expect("decode katakana");
        assert_eq!(s, dec, "katakana must roundtrip");
        assert_eq!(consumed, enc.len());
        // Katakana: U+30A0-U+30FF, each 3 bytes in UTF-8
        assert_eq!(s.len(), 4 * 3, "4 katakana × 3 bytes = 12 bytes");
        assert_eq!(s.chars().count(), 4);
    }

    // -----------------------------------------------------------------------
    // 8. Japanese kanji: "漢字" roundtrip
    // -----------------------------------------------------------------------
    #[test]
    fn test_japanese_kanji_roundtrip() {
        let s = "漢字".to_string();
        let enc = encode_to_vec(&s).expect("encode kanji");
        let (dec, consumed): (String, _) = decode_from_slice(&enc).expect("decode kanji");
        assert_eq!(s, dec, "kanji must roundtrip");
        assert_eq!(consumed, enc.len());
        // 漢 = U+6F22, 字 = U+5B57, both 3-byte UTF-8
        assert_eq!(s.len(), 6, "2 kanji × 3 bytes = 6 bytes");
        assert_eq!(s.chars().count(), 2);
    }

    // -----------------------------------------------------------------------
    // 9. Korean: "안녕하세요" roundtrip
    // -----------------------------------------------------------------------
    #[test]
    fn test_korean_roundtrip() {
        let s = "안녕하세요".to_string();
        let enc = encode_to_vec(&s).expect("encode Korean");
        let (dec, consumed): (String, _) = decode_from_slice(&enc).expect("decode Korean");
        assert_eq!(s, dec, "Korean Hangul must roundtrip");
        assert_eq!(consumed, enc.len());
        // Korean Hangul syllables: U+AC00-U+D7A3, each 3 bytes in UTF-8
        assert_eq!(s.len(), 5 * 3, "5 Hangul syllables × 3 bytes = 15 bytes");
        assert_eq!(s.chars().count(), 5);
    }

    // -----------------------------------------------------------------------
    // 10. Arabic (RTL): "مرحبا بالعالم" roundtrip
    // -----------------------------------------------------------------------
    #[test]
    fn test_arabic_rtl_roundtrip() {
        // "مرحبا بالعالم" = "Hello World" in Arabic (RTL)
        let s = "مرحبا بالعالم".to_string();
        let enc = encode_to_vec(&s).expect("encode Arabic RTL");
        let (dec, consumed): (String, _) = decode_from_slice(&enc).expect("decode Arabic RTL");
        assert_eq!(s, dec, "Arabic RTL string must roundtrip");
        assert_eq!(consumed, enc.len());
        // Arabic chars are 2 bytes in UTF-8 (U+0600-U+06FF range)
        let char_count = s.chars().count();
        assert!(char_count > 0);
        assert!(
            s.len() > char_count,
            "Arabic is multi-byte: byte_len={} > char_count={}",
            s.len(),
            char_count
        );
    }

    // -----------------------------------------------------------------------
    // 11. Hebrew (RTL): "שלום עולם" roundtrip
    // -----------------------------------------------------------------------
    #[test]
    fn test_hebrew_rtl_roundtrip() {
        // "שלום עולם" = "Hello World" in Hebrew (RTL)
        let s = "שלום עולם".to_string();
        let enc = encode_to_vec(&s).expect("encode Hebrew RTL");
        let (dec, consumed): (String, _) = decode_from_slice(&enc).expect("decode Hebrew RTL");
        assert_eq!(s, dec, "Hebrew RTL string must roundtrip");
        assert_eq!(consumed, enc.len());
        // Hebrew letters: U+05D0-U+05EA, each 2 bytes in UTF-8; space: 1 byte
        let hebrew_chars = s.chars().filter(|c| *c != ' ').count();
        let space_count = s.chars().filter(|c| *c == ' ').count();
        assert_eq!(
            s.len(),
            hebrew_chars * 2 + space_count,
            "Hebrew chars are 2 bytes each"
        );
    }

    // -----------------------------------------------------------------------
    // 12. Emoji: "😀🎉🌍🚀💻" roundtrip
    // -----------------------------------------------------------------------
    #[test]
    fn test_emoji_five_roundtrip() {
        let s = "😀🎉🌍🚀💻".to_string();
        let enc = encode_to_vec(&s).expect("encode emoji");
        let (dec, consumed): (String, _) = decode_from_slice(&enc).expect("decode emoji");
        assert_eq!(s, dec, "emoji string must roundtrip");
        assert_eq!(consumed, enc.len());
        // Each of these emoji is a single code point encoded as 4 bytes in UTF-8
        assert_eq!(s.chars().count(), 5, "5 emoji characters");
        assert_eq!(s.len(), 5 * 4, "5 emoji × 4 bytes each = 20 bytes");
        for ch in s.chars() {
            assert_eq!(
                ch.len_utf8(),
                4,
                "emoji {:?} should be 4 bytes in UTF-8",
                ch
            );
        }
    }

    // -----------------------------------------------------------------------
    // 13. Mixed emoji+text: "Hello 🌍!" roundtrip
    // -----------------------------------------------------------------------
    #[test]
    fn test_mixed_emoji_text_roundtrip() {
        let s = "Hello 🌍!".to_string();
        let enc = encode_to_vec(&s).expect("encode mixed emoji+text");
        let (dec, consumed): (String, _) =
            decode_from_slice(&enc).expect("decode mixed emoji+text");
        assert_eq!(s, dec, "mixed emoji+text must roundtrip");
        assert_eq!(consumed, enc.len());
        // "Hello " = 6 bytes, 🌍 = 4 bytes, "!" = 1 byte => total 11 bytes
        assert_eq!(s.len(), 11, "byte length: 6 ASCII + 4 emoji + 1 ASCII = 11");
        assert_eq!(s.chars().count(), 8, "char count: 6 + 1 emoji + 1 = 8");
    }

    // -----------------------------------------------------------------------
    // 14. NUL character "\x00" in string roundtrip
    // -----------------------------------------------------------------------
    #[test]
    fn test_nul_char_roundtrip() {
        // A string consisting solely of a NUL byte. Rust strings allow this;
        // OxiCode's length-prefix encoding is not C-string termination based.
        let s = "\x00".to_string();
        assert_eq!(s.len(), 1, "NUL char is 1 byte in UTF-8");
        assert_eq!(s.chars().count(), 1);
        let enc = encode_to_vec(&s).expect("encode NUL char string");
        let (dec, consumed): (String, _) = decode_from_slice(&enc).expect("decode NUL char string");
        assert_eq!(
            s, dec,
            "NUL-only string must roundtrip via length-prefix encoding"
        );
        assert_eq!(consumed, enc.len());
        // Encoded: 1 byte varint prefix (value=1) + 1 byte data
        assert_eq!(
            enc.len(),
            2,
            "NUL string: 1-byte varint prefix + 1 data byte"
        );
        assert_eq!(enc[0], 1, "varint prefix encodes length=1");
        assert_eq!(enc[1], 0x00, "data byte is the NUL character");
    }

    // -----------------------------------------------------------------------
    // 15. All ASCII 32-126 roundtrip (verifying byte-exact encoding)
    // -----------------------------------------------------------------------
    #[test]
    fn test_all_ascii_32_to_126_encoding_invariant() {
        let s: String = (32u8..=126u8).map(|b| b as char).collect();
        assert_eq!(s.len(), 95, "95 printable ASCII chars");
        assert_eq!(s.chars().count(), 95, "all single-byte in UTF-8");
        let enc = encode_to_vec(&s).expect("encode all printable ASCII");
        let (dec, consumed): (String, _) =
            decode_from_slice(&enc).expect("decode all printable ASCII");
        assert_eq!(s, dec, "all printable ASCII must roundtrip");
        assert_eq!(consumed, enc.len());
        // 95 bytes of data, 1-byte varint prefix (95 <= 250), total = 96
        assert_eq!(
            enc.len(),
            96,
            "95 ASCII bytes + 1-byte varint prefix = 96 total"
        );
        assert_eq!(enc[0], 95, "varint prefix encodes length=95");
        // Verify data bytes are the exact ASCII values in order
        for (i, b) in (32u8..=126u8).enumerate() {
            assert_eq!(
                enc[1 + i],
                b,
                "byte at position {} should be ASCII {}",
                1 + i,
                b
            );
        }
    }

    // -----------------------------------------------------------------------
    // 16. 4-byte UTF-8 chars: "𝕳𝖊𝖑𝖑𝖔" roundtrip (Mathematical Fraktur Capital H-O)
    // -----------------------------------------------------------------------
    #[test]
    fn test_four_byte_utf8_math_fraktur_roundtrip() {
        // Different chars from existing test: 𝕳=U+1D573 𝖊=U+1D58A 𝖑=U+1D591 𝖑=U+1D591 𝖔=U+1D594
        let s = "𝕳𝖊𝖑𝖑𝖔".to_string();
        let enc = encode_to_vec(&s).expect("encode Mathematical Fraktur");
        let (dec, consumed): (String, _) =
            decode_from_slice(&enc).expect("decode Mathematical Fraktur");
        assert_eq!(s, dec, "Mathematical Fraktur must roundtrip");
        assert_eq!(consumed, enc.len());
        assert_eq!(s.chars().count(), 5, "5 fraktur characters");
        // Verify all are 4-byte code points
        for ch in s.chars() {
            assert_eq!(
                ch.len_utf8(),
                4,
                "fraktur char {:?} must be 4 bytes in UTF-8",
                ch
            );
        }
        assert_eq!(s.len(), 5 * 4, "5 chars × 4 bytes = 20 bytes");
    }

    // -----------------------------------------------------------------------
    // 17. String with all Chinese numbers: "一二三四五六七八九十" roundtrip
    // -----------------------------------------------------------------------
    #[test]
    fn test_chinese_numbers_roundtrip() {
        let s = "一二三四五六七八九十".to_string();
        let enc = encode_to_vec(&s).expect("encode Chinese numbers");
        let (dec, consumed): (String, _) = decode_from_slice(&enc).expect("decode Chinese numbers");
        assert_eq!(s, dec, "Chinese numbers must roundtrip");
        assert_eq!(consumed, enc.len());
        assert_eq!(s.chars().count(), 10, "10 Chinese digit characters");
        // CJK Unified Ideographs: each 3 bytes in UTF-8
        assert_eq!(s.len(), 10 * 3, "10 CJK chars × 3 bytes = 30 bytes");
    }

    // -----------------------------------------------------------------------
    // 18. Long unicode string: 1000 CJK chars roundtrip
    // -----------------------------------------------------------------------
    #[test]
    fn test_long_cjk_string_roundtrip() {
        // Build 1000 CJK chars by cycling through a set of representative ideographs
        let base_chars = ['中', '文', '字', '符', '串', '编', '码', '测', '试', '集'];
        let s: String = (0..1000)
            .map(|i| base_chars[i % base_chars.len()])
            .collect();
        assert_eq!(s.chars().count(), 1000);
        // Each CJK char is 3 bytes => 3000 bytes total
        assert_eq!(s.len(), 3000);

        let enc = encode_to_vec(&s).expect("encode long CJK string");
        let (dec, consumed): (String, _) = decode_from_slice(&enc).expect("decode long CJK string");
        assert_eq!(s, dec, "long CJK string must roundtrip");
        assert_eq!(consumed, enc.len());

        // 3000 bytes > 250 but <= 65535: varint prefix is 3 bytes (tag + 2 data bytes)
        let prefix_len = varint_prefix_len(s.len());
        assert_eq!(prefix_len, 3, "3000-byte string uses 3-byte varint prefix");
        assert_eq!(enc.len(), prefix_len + s.len());
    }

    // -----------------------------------------------------------------------
    // 19. Vec<String> with mixed languages roundtrip
    // -----------------------------------------------------------------------
    #[test]
    fn test_vec_string_mixed_languages_roundtrip() {
        let v: Vec<String> = vec![
            "English".to_string(),
            "中文".to_string(),
            "日本語".to_string(),
            "한국어".to_string(),
            "العربية".to_string(),
            "עברית".to_string(),
            "Ελληνικά".to_string(),
            "Русский".to_string(),
            "😀🎉🌍".to_string(),
            "café naïve résumé".to_string(),
        ];
        let enc = encode_to_vec(&v).expect("encode Vec<String> mixed languages");
        let (dec, consumed): (Vec<String>, _) =
            decode_from_slice(&enc).expect("decode Vec<String> mixed languages");
        assert_eq!(v, dec, "Vec<String> with mixed languages must roundtrip");
        assert_eq!(consumed, enc.len());
        assert_eq!(dec.len(), 10, "all 10 strings must be present");
        // Spot-check a few entries
        assert_eq!(dec[0], "English");
        assert_eq!(dec[1], "中文");
        assert_eq!(dec[8], "😀🎉🌍");
    }

    // -----------------------------------------------------------------------
    // 20. BTreeMap<String, String> with unicode keys and values roundtrip
    // -----------------------------------------------------------------------
    #[test]
    fn test_btreemap_unicode_keys_values_roundtrip() {
        let mut map: BTreeMap<String, String> = BTreeMap::new();
        map.insert("english_key".to_string(), "English value".to_string());
        map.insert("中文键".to_string(), "中文值".to_string());
        map.insert("日本語キー".to_string(), "日本語の値".to_string());
        map.insert("한국어_키".to_string(), "한국어 값".to_string());
        map.insert("emoji_🔑".to_string(), "value_💎".to_string());
        map.insert("مفتاح".to_string(), "قيمة".to_string());

        let enc = encode_to_vec(&map).expect("encode BTreeMap unicode");
        let (dec, consumed): (BTreeMap<String, String>, _) =
            decode_from_slice(&enc).expect("decode BTreeMap unicode");
        assert_eq!(map.len(), dec.len(), "map entry count must match");
        assert_eq!(consumed, enc.len());

        for (k, v) in &map {
            assert_eq!(
                dec.get(k),
                Some(v),
                "unicode key '{}' and its value must survive roundtrip",
                k
            );
        }
        // BTreeMap ordering is deterministic by key; verify iteration order matches
        let orig_keys: Vec<&str> = map.keys().map(|s| s.as_str()).collect();
        let dec_keys: Vec<&str> = dec.keys().map(|s| s.as_str()).collect();
        assert_eq!(
            orig_keys, dec_keys,
            "BTreeMap keys must maintain sorted order"
        );
    }

    // -----------------------------------------------------------------------
    // 21. String byte length vs char count verification across scripts
    // -----------------------------------------------------------------------
    #[test]
    fn test_byte_length_vs_char_count_across_scripts() {
        // Each tuple: (string, expected_byte_len, expected_char_count)
        let cases: &[(&str, usize, usize)] = &[
            ("abc", 3, 3),     // pure ASCII: byte_len == char_count
            ("αβγ", 6, 3),     // 2-byte Greek: byte_len = 2×char_count
            ("你好世", 9, 3),  // 3-byte CJK: byte_len = 3×char_count
            ("😀🎉🌍", 12, 3), // 4-byte emoji: byte_len = 4×char_count
            ("a中😀", 8, 3),   // mixed: 1 + 3 + 4 = 8 bytes, 3 chars
            ("café", 5, 4),    // é is 2 bytes: 3 + 2 = 5 bytes, 4 chars
        ];

        for (s, expected_byte_len, expected_char_count) in cases {
            let s = s.to_string();
            assert_eq!(s.len(), *expected_byte_len, "byte_len mismatch for '{}'", s);
            assert_eq!(
                s.chars().count(),
                *expected_char_count,
                "char_count mismatch for '{}'",
                s
            );

            // Verify roundtrip correctness for each case
            let enc = encode_to_vec(&s).expect("encode mixed script");
            let (dec, consumed): (String, _) =
                decode_from_slice(&enc).expect("decode mixed script");
            assert_eq!(s, dec, "script '{}' must roundtrip", s);
            assert_eq!(consumed, enc.len());
        }
    }

    // -----------------------------------------------------------------------
    // 22. encode_to_vec length = varint_prefix(utf8_bytes) + utf8_byte_count
    // -----------------------------------------------------------------------
    #[test]
    fn test_encoded_length_equals_varint_prefix_plus_utf8_bytes() {
        // Verify the invariant: encode_to_vec(s).len() == varint_prefix_len(s.len()) + s.len()
        // for strings of various UTF-8 byte lengths, including boundary cases.

        let test_strings: &[&str] = &[
            "",                // 0 bytes
            "x",               // 1 byte
            "中",              // 3 bytes (3-byte CJK)
            "😀",              // 4 bytes (4-byte emoji)
            &"a".repeat(249),  // 249 bytes: still 1-byte prefix
            &"a".repeat(250),  // 250 bytes: exactly at SINGLE_BYTE_MAX (1-byte prefix)
            &"a".repeat(251),  // 251 bytes: crosses to 3-byte prefix
            &"a".repeat(500),  // 500 bytes: 3-byte prefix
            &"αβ".repeat(100), // 200 chars × 2 bytes = 400 bytes: 1-byte prefix (<=250? No: 400 > 250, 3-byte)
            &"中".repeat(83),  // 83 chars × 3 bytes = 249 bytes: 1-byte prefix
            &"中".repeat(84),  // 84 chars × 3 bytes = 252 bytes: 3-byte prefix
        ];

        for s_ref in test_strings {
            let s = s_ref.to_string();
            let utf8_len = s.len();
            let expected_prefix = varint_prefix_len(utf8_len);
            let expected_total = expected_prefix + utf8_len;

            let enc = encode_to_vec(&s).expect("encode for length invariant test");
            assert_eq!(
                enc.len(),
                expected_total,
                "string of {} UTF-8 bytes: expected {} bytes encoded ({}+{}), got {}",
                utf8_len,
                expected_total,
                expected_prefix,
                utf8_len,
                enc.len()
            );

            // Also verify encoded_size() matches encode_to_vec().len()
            let esz = encoded_size(&s).expect("encoded_size");
            assert_eq!(
                esz,
                enc.len(),
                "encoded_size must match encode_to_vec length for string of {} bytes",
                utf8_len
            );

            // And verify the config::standard() path is consistent
            let enc2 = oxicode::encode_to_vec_with_config(&s, config::standard())
                .expect("encode_to_vec_with_config");
            assert_eq!(
                enc, enc2,
                "standard() config must produce identical bytes for string of {} bytes",
                utf8_len
            );

            // Roundtrip check
            let (dec, consumed): (String, _) =
                decode_from_slice(&enc).expect("decode for length invariant test");
            assert_eq!(s, dec, "length-invariant string must roundtrip");
            assert_eq!(consumed, enc.len());
        }
    }
}
