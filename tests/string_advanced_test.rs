//! Advanced string encoding tests covering edge cases, Unicode categories,
//! length-prefix correctness, and struct-level derive roundtrips.

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
use oxicode::{decode_from_slice, encode_to_vec};

mod string_advanced_tests {
    use super::*;
    use oxicode::{Decode, Encode};
    use std::collections::HashMap;

    // -----------------------------------------------------------------------
    // 1. Empty string roundtrip
    // -----------------------------------------------------------------------
    #[test]
    fn test_empty_string_roundtrip() {
        let s = String::new();
        let enc = encode_to_vec(&s).expect("encode empty string");
        let (dec, consumed): (String, _) = decode_from_slice(&enc).expect("decode empty string");
        assert_eq!(s, dec, "empty string should roundtrip");
        assert_eq!(consumed, enc.len(), "all bytes should be consumed");
    }

    // -----------------------------------------------------------------------
    // 2. Single ASCII char string "a" roundtrip
    // -----------------------------------------------------------------------
    #[test]
    fn test_single_ascii_char_roundtrip() {
        let s = "a".to_string();
        let enc = encode_to_vec(&s).expect("encode single char");
        let (dec, consumed): (String, _) = decode_from_slice(&enc).expect("decode single char");
        assert_eq!(s, dec);
        assert_eq!(consumed, enc.len());
    }

    // -----------------------------------------------------------------------
    // 3. String with all printable ASCII chars
    // -----------------------------------------------------------------------
    #[test]
    fn test_all_printable_ascii_roundtrip() {
        let s: String = (0x20u8..=0x7eu8).map(|b| b as char).collect();
        assert_eq!(s.len(), 95, "there are 95 printable ASCII chars");
        let enc = encode_to_vec(&s).expect("encode printable ASCII");
        let (dec, consumed): (String, _) = decode_from_slice(&enc).expect("decode printable ASCII");
        assert_eq!(s, dec);
        assert_eq!(consumed, enc.len());
    }

    // -----------------------------------------------------------------------
    // 4. String with digits "0123456789" roundtrip
    // -----------------------------------------------------------------------
    #[test]
    fn test_digit_string_roundtrip() {
        let s = "0123456789".to_string();
        let enc = encode_to_vec(&s).expect("encode digits");
        let (dec, consumed): (String, _) = decode_from_slice(&enc).expect("decode digits");
        assert_eq!(s, dec);
        assert_eq!(consumed, enc.len());
    }

    // -----------------------------------------------------------------------
    // 5. String with spaces and tabs
    // -----------------------------------------------------------------------
    #[test]
    fn test_whitespace_string_roundtrip() {
        let s = "hello\t world\t foo  bar".to_string();
        let enc = encode_to_vec(&s).expect("encode whitespace string");
        let (dec, consumed): (String, _) =
            decode_from_slice(&enc).expect("decode whitespace string");
        assert_eq!(s, dec);
        assert_eq!(consumed, enc.len());
    }

    // -----------------------------------------------------------------------
    // 6. Unicode: Chinese characters (CJK) roundtrip
    // -----------------------------------------------------------------------
    #[test]
    fn test_cjk_chinese_roundtrip() {
        let s = "你好世界，这是一个测试。".to_string();
        let enc = encode_to_vec(&s).expect("encode CJK Chinese");
        let (dec, consumed): (String, _) = decode_from_slice(&enc).expect("decode CJK Chinese");
        assert_eq!(s, dec);
        assert_eq!(consumed, enc.len());
    }

    // -----------------------------------------------------------------------
    // 7. Unicode: Japanese hiragana roundtrip
    // -----------------------------------------------------------------------
    #[test]
    fn test_japanese_hiragana_roundtrip() {
        let s = "こんにちは、せかい！おはようございます。".to_string();
        let enc = encode_to_vec(&s).expect("encode hiragana");
        let (dec, consumed): (String, _) = decode_from_slice(&enc).expect("decode hiragana");
        assert_eq!(s, dec);
        assert_eq!(consumed, enc.len());
    }

    // -----------------------------------------------------------------------
    // 8. Unicode: Arabic text roundtrip
    // -----------------------------------------------------------------------
    #[test]
    fn test_arabic_text_roundtrip() {
        let s = "مرحبا بالعالم، هذا اختبار للترميز".to_string();
        let enc = encode_to_vec(&s).expect("encode Arabic");
        let (dec, consumed): (String, _) = decode_from_slice(&enc).expect("decode Arabic");
        assert_eq!(s, dec);
        assert_eq!(consumed, enc.len());
    }

    // -----------------------------------------------------------------------
    // 9. Unicode: Emoji roundtrip (🦀🔥✨)
    // -----------------------------------------------------------------------
    #[test]
    fn test_emoji_roundtrip() {
        let s = "🦀🔥✨🎉🚀🌈💎🦊🐉🎯".to_string();
        let enc = encode_to_vec(&s).expect("encode emoji");
        let (dec, consumed): (String, _) = decode_from_slice(&enc).expect("decode emoji");
        assert_eq!(s, dec);
        assert_eq!(consumed, enc.len());
        // Each emoji is 4 bytes in UTF-8 (except ✨ which is 3)
        assert!(s.len() > s.chars().count(), "multibyte chars");
    }

    // -----------------------------------------------------------------------
    // 10. Unicode: Mixed ASCII + emoji
    // -----------------------------------------------------------------------
    #[test]
    fn test_mixed_ascii_emoji_roundtrip() {
        let s = "Rust is 🦀, fire is 🔥, sparkle is ✨!".to_string();
        let enc = encode_to_vec(&s).expect("encode mixed ASCII+emoji");
        let (dec, consumed): (String, _) =
            decode_from_slice(&enc).expect("decode mixed ASCII+emoji");
        assert_eq!(s, dec);
        assert_eq!(consumed, enc.len());
    }

    // -----------------------------------------------------------------------
    // 11. String with 1000 'a' chars roundtrip
    // -----------------------------------------------------------------------
    #[test]
    fn test_one_thousand_chars_roundtrip() {
        let s = "a".repeat(1000);
        assert_eq!(s.len(), 1000);
        let enc = encode_to_vec(&s).expect("encode 1000 chars");
        let (dec, consumed): (String, _) = decode_from_slice(&enc).expect("decode 1000 chars");
        assert_eq!(s, dec);
        assert_eq!(consumed, enc.len());
    }

    // -----------------------------------------------------------------------
    // 12. String with 10000 'z' chars roundtrip
    // -----------------------------------------------------------------------
    #[test]
    fn test_ten_thousand_chars_roundtrip() {
        let s = "z".repeat(10_000);
        assert_eq!(s.len(), 10_000);
        let enc = encode_to_vec(&s).expect("encode 10000 chars");
        let (dec, consumed): (String, _) = decode_from_slice(&enc).expect("decode 10000 chars");
        assert_eq!(s, dec);
        assert_eq!(consumed, enc.len());
    }

    // -----------------------------------------------------------------------
    // 13. String with null bytes — valid Rust String, check it works
    // -----------------------------------------------------------------------
    #[test]
    fn test_null_byte_in_string_roundtrip() {
        // Rust String may contain null bytes (not C-string semantics).
        // OxiCode encodes length-prefixed, so nulls are transparent.
        let s = "hello\0world\0end".to_string();
        assert_eq!(s.len(), 15);
        let enc = encode_to_vec(&s).expect("encode string with null bytes");
        let (dec, consumed): (String, _) =
            decode_from_slice(&enc).expect("decode string with null bytes");
        assert_eq!(s, dec, "null bytes must survive roundtrip");
        assert_eq!(consumed, enc.len());
    }

    // -----------------------------------------------------------------------
    // 14. String with newlines and tabs
    // -----------------------------------------------------------------------
    #[test]
    fn test_newlines_and_tabs_roundtrip() {
        let s = "line1\nline2\r\nline3\ttabbed\r\nend".to_string();
        let enc = encode_to_vec(&s).expect("encode newlines/tabs");
        let (dec, consumed): (String, _) = decode_from_slice(&enc).expect("decode newlines/tabs");
        assert_eq!(s, dec);
        assert_eq!(consumed, enc.len());
    }

    // -----------------------------------------------------------------------
    // 15. String with backslashes
    // -----------------------------------------------------------------------
    #[test]
    fn test_backslashes_roundtrip() {
        let s = r"C:\Users\test\path\to\file.txt".to_string();
        let enc = encode_to_vec(&s).expect("encode backslashes");
        let (dec, consumed): (String, _) = decode_from_slice(&enc).expect("decode backslashes");
        assert_eq!(s, dec);
        assert_eq!(consumed, enc.len());
    }

    // -----------------------------------------------------------------------
    // 16. String with quotes and special chars
    // -----------------------------------------------------------------------
    #[test]
    fn test_quotes_and_special_chars_roundtrip() {
        let s = r#"He said "hello" and she said 'world' & <tag>data</tag>"#.to_string();
        let enc = encode_to_vec(&s).expect("encode quotes/special");
        let (dec, consumed): (String, _) = decode_from_slice(&enc).expect("decode quotes/special");
        assert_eq!(s, dec);
        assert_eq!(consumed, enc.len());
    }

    // -----------------------------------------------------------------------
    // 17. Multiple strings in Vec<String> roundtrip
    // -----------------------------------------------------------------------
    #[test]
    fn test_vec_of_strings_roundtrip() {
        let v: Vec<String> = vec![
            "alpha".to_string(),
            "beta".to_string(),
            "gamma delta".to_string(),
            "".to_string(),
            "🦀 rust".to_string(),
            "日本語".to_string(),
        ];
        let enc = encode_to_vec(&v).expect("encode Vec<String>");
        let (dec, consumed): (Vec<String>, _) =
            decode_from_slice(&enc).expect("decode Vec<String>");
        assert_eq!(v, dec);
        assert_eq!(consumed, enc.len());
    }

    // -----------------------------------------------------------------------
    // 18. String as HashMap key roundtrip
    // -----------------------------------------------------------------------
    #[test]
    fn test_string_as_hashmap_key_roundtrip() {
        let mut map: HashMap<String, u64> = HashMap::new();
        map.insert("one".to_string(), 1);
        map.insert("two".to_string(), 2);
        map.insert("three".to_string(), 3);
        map.insert("unicode_key_🦀".to_string(), 42);

        let enc = encode_to_vec(&map).expect("encode HashMap<String,u64>");
        let (dec, consumed): (HashMap<String, u64>, _) =
            decode_from_slice(&enc).expect("decode HashMap<String,u64>");
        assert_eq!(map.len(), dec.len(), "map sizes must match");
        for (k, v) in &map {
            assert_eq!(dec.get(k), Some(v), "key '{}' must survive roundtrip", k);
        }
        assert_eq!(consumed, enc.len());
    }

    // -----------------------------------------------------------------------
    // 19. String length prefix is correct varint (len bytes + data)
    // -----------------------------------------------------------------------
    #[test]
    fn test_string_length_prefix_varint_correctness() {
        // OxiCode uses a custom varint scheme where values 0..=250 fit in
        // a single byte. Values 251..=65535 use a 3-byte encoding:
        // 1 marker byte + 2 data bytes.
        //
        // So for strings with byte length <= 250, the length prefix is 1 byte.
        // For strings with byte length 251..=65535, the prefix is 3 bytes.

        // "hello" = 5 bytes, prefix = 1 byte, total = 6
        let s5 = "hello".to_string();
        let enc5 = encode_to_vec(&s5).expect("encode 5-byte string");
        assert_eq!(
            enc5.len(),
            1 + s5.len(),
            "5-byte string: 1-byte varint prefix + 5 data bytes"
        );

        // 250-byte string: prefix still 1 byte (250 == SINGLE_BYTE_MAX), total = 251
        let s250 = "x".repeat(250);
        let enc250 = encode_to_vec(&s250).expect("encode 250-byte string");
        assert_eq!(
            enc250.len(),
            1 + 250,
            "250-byte string: 1-byte varint prefix (250 == SINGLE_BYTE_MAX) + 250 data bytes"
        );

        // 251-byte string: prefix uses 3-byte marker encoding, total = 254
        let s251 = "y".repeat(251);
        let enc251 = encode_to_vec(&s251).expect("encode 251-byte string");
        assert_eq!(
            enc251.len(),
            3 + 251,
            "251-byte string: 3-byte varint prefix (marker + 2 length bytes) + 251 data bytes"
        );
    }

    // -----------------------------------------------------------------------
    // 20. Very long string with repeated pattern encodes/decodes correctly
    // -----------------------------------------------------------------------
    #[test]
    fn test_long_repeated_pattern_roundtrip() {
        // Use PI and E digits to create a non-trivial pattern
        let pi_str = std::f64::consts::PI.to_string();
        let e_str = std::f64::consts::E.to_string();
        let pattern = format!("{}:{}", pi_str, e_str);
        // Repeat to ~50 000 bytes
        let repeat_count = 50_000 / pattern.len() + 1;
        let s = pattern.repeat(repeat_count);
        assert!(s.len() >= 50_000);

        let enc = encode_to_vec(&s).expect("encode long repeated pattern");
        let (dec, consumed): (String, _) =
            decode_from_slice(&enc).expect("decode long repeated pattern");
        assert_eq!(s, dec);
        assert_eq!(consumed, enc.len());
    }

    // -----------------------------------------------------------------------
    // 21. String containing valid UTF-8 4-byte sequences (𝕳𝖊𝖑𝖑𝖔 — Mathematical Fraktur)
    // -----------------------------------------------------------------------
    #[test]
    fn test_four_byte_utf8_sequences_roundtrip() {
        // These are U+1D573 range Mathematical Fraktur letters, each 4 bytes in UTF-8.
        let s = "𝕳𝖊𝖑𝖑𝖔 𝖂𝖔𝖗𝖑𝖉".to_string();
        // Verify we truly have 4-byte chars
        for ch in s.chars().filter(|c| *c != ' ') {
            assert_eq!(ch.len_utf8(), 4, "char {:?} should be 4 bytes in UTF-8", ch);
        }
        let enc = encode_to_vec(&s).expect("encode 4-byte UTF-8 sequences");
        let (dec, consumed): (String, _) =
            decode_from_slice(&enc).expect("decode 4-byte UTF-8 sequences");
        assert_eq!(s, dec);
        assert_eq!(consumed, enc.len());
    }

    // -----------------------------------------------------------------------
    // 22. Struct with 5 string fields derive roundtrip
    // -----------------------------------------------------------------------
    #[derive(Debug, PartialEq, Encode, Decode)]
    struct FiveStringFields {
        first_name: String,
        last_name: String,
        email: String,
        bio: String,
        motto: String,
    }

    #[test]
    fn test_struct_five_string_fields_derive_roundtrip() {
        let record = FiveStringFields {
            first_name: "Ferris".to_string(),
            last_name: "Rustacean".to_string(),
            email: "ferris@rust-lang.org".to_string(),
            bio: "The official Rust mascot 🦀. Loves memory safety and zero-cost abstractions."
                .to_string(),
            motto: "Fearless concurrency and blazing-fast performance!".to_string(),
        };

        let enc = encode_to_vec(&record).expect("encode FiveStringFields");
        let (dec, consumed): (FiveStringFields, _) =
            decode_from_slice(&enc).expect("decode FiveStringFields");

        assert_eq!(record, dec);
        assert_eq!(consumed, enc.len());
        assert_eq!(dec.first_name, "Ferris");
        assert_eq!(dec.last_name, "Rustacean");
        assert_eq!(dec.email, "ferris@rust-lang.org");
        assert!(dec.bio.contains("🦀"));
        assert!(dec.motto.contains("concurrency"));
    }
}
