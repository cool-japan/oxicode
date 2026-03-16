//! Advanced string encoding tests — international text and multilingual content.
//!
//! Theme: Language, LocalizedString, TranslationMap

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
use std::collections::HashMap;

#[derive(Debug, PartialEq, Encode, Decode)]
enum Language {
    English,
    Japanese,
    Chinese,
    Arabic,
    Russian,
    Spanish,
    French,
    German,
}

#[derive(Debug, PartialEq, Encode, Decode)]
struct LocalizedString {
    language: Language,
    text: String,
    rtl: bool,
}

#[derive(Debug, PartialEq, Encode, Decode)]
struct TranslationSet {
    key: String,
    translations: Vec<LocalizedString>,
}

// Test 1: Japanese text roundtrip with Language::Japanese
#[test]
fn test_japanese_localized_string_roundtrip() {
    let entry = LocalizedString {
        language: Language::Japanese,
        text: "日本語のテキストです。こんにちは世界！".to_string(),
        rtl: false,
    };
    let encoded = encode_to_vec(&entry).expect("Failed to encode Japanese LocalizedString");
    let (decoded, _): (LocalizedString, usize) =
        decode_from_slice(&encoded).expect("Failed to decode Japanese LocalizedString");
    assert_eq!(entry, decoded);
    assert_eq!(decoded.language, Language::Japanese);
    assert!(!decoded.rtl);
}

// Test 2: Chinese text roundtrip with Language::Chinese
#[test]
fn test_chinese_localized_string_roundtrip() {
    let entry = LocalizedString {
        language: Language::Chinese,
        text: "中文内容示例：你好，世界！这是一段中文文字。".to_string(),
        rtl: false,
    };
    let encoded = encode_to_vec(&entry).expect("Failed to encode Chinese LocalizedString");
    let (decoded, _): (LocalizedString, usize) =
        decode_from_slice(&encoded).expect("Failed to decode Chinese LocalizedString");
    assert_eq!(entry, decoded);
    assert_eq!(decoded.language, Language::Chinese);
}

// Test 3: Arabic RTL text roundtrip with Language::Arabic
#[test]
fn test_arabic_rtl_localized_string_roundtrip() {
    let entry = LocalizedString {
        language: Language::Arabic,
        text: "مرحبًا بالعالم! هذا نص عربي.".to_string(),
        rtl: true,
    };
    let encoded = encode_to_vec(&entry).expect("Failed to encode Arabic LocalizedString");
    let (decoded, _): (LocalizedString, usize) =
        decode_from_slice(&encoded).expect("Failed to decode Arabic LocalizedString");
    assert_eq!(entry, decoded);
    assert_eq!(decoded.language, Language::Arabic);
    assert!(decoded.rtl, "Arabic text must be marked as RTL");
}

// Test 4: Russian text roundtrip with Language::Russian
#[test]
fn test_russian_localized_string_roundtrip() {
    let entry = LocalizedString {
        language: Language::Russian,
        text: "Привет, мир! Это текст на русском языке.".to_string(),
        rtl: false,
    };
    let encoded = encode_to_vec(&entry).expect("Failed to encode Russian LocalizedString");
    let (decoded, _): (LocalizedString, usize) =
        decode_from_slice(&encoded).expect("Failed to decode Russian LocalizedString");
    assert_eq!(entry, decoded);
    assert_eq!(decoded.language, Language::Russian);
}

// Test 5: All eight Language enum variants can roundtrip independently
#[test]
fn test_all_language_variants_roundtrip() {
    let variants = vec![
        Language::English,
        Language::Japanese,
        Language::Chinese,
        Language::Arabic,
        Language::Russian,
        Language::Spanish,
        Language::French,
        Language::German,
    ];
    for lang in variants {
        let encoded = encode_to_vec(&lang).expect("Failed to encode Language variant");
        let (decoded, _): (Language, usize) =
            decode_from_slice(&encoded).expect("Failed to decode Language variant");
        assert_eq!(lang, decoded);
    }
}

// Test 6: RTL vs LTR flag is preserved correctly
#[test]
fn test_rtl_vs_ltr_flag_preservation() {
    let rtl_entry = LocalizedString {
        language: Language::Arabic,
        text: "العربية".to_string(),
        rtl: true,
    };
    let ltr_entry = LocalizedString {
        language: Language::English,
        text: "English".to_string(),
        rtl: false,
    };

    let enc_rtl = encode_to_vec(&rtl_entry).expect("Failed to encode RTL entry");
    let enc_ltr = encode_to_vec(&ltr_entry).expect("Failed to encode LTR entry");

    let (dec_rtl, _): (LocalizedString, usize) =
        decode_from_slice(&enc_rtl).expect("Failed to decode RTL entry");
    let (dec_ltr, _): (LocalizedString, usize) =
        decode_from_slice(&enc_ltr).expect("Failed to decode LTR entry");

    assert!(dec_rtl.rtl, "RTL flag must remain true after roundtrip");
    assert!(!dec_ltr.rtl, "LTR flag must remain false after roundtrip");
}

// Test 7: Empty text string inside LocalizedString
#[test]
fn test_empty_text_in_localized_string() {
    let entry = LocalizedString {
        language: Language::English,
        text: String::new(),
        rtl: false,
    };
    let encoded = encode_to_vec(&entry).expect("Failed to encode LocalizedString with empty text");
    let (decoded, _): (LocalizedString, usize) =
        decode_from_slice(&encoded).expect("Failed to decode LocalizedString with empty text");
    assert_eq!(entry, decoded);
    assert!(
        decoded.text.is_empty(),
        "Text must remain empty after roundtrip"
    );
}

// Test 8: Very long string (1000+ Unicode chars) inside LocalizedString
#[test]
fn test_very_long_unicode_text_in_localized_string() {
    // 100 repetitions of a 12-char Japanese phrase = 1200 Unicode chars, 3600 UTF-8 bytes
    let long_text = "日本語テキスト！！！！！！".repeat(100);
    assert!(
        long_text.chars().count() >= 1000,
        "Test requires at least 1000 chars"
    );
    let entry = LocalizedString {
        language: Language::Japanese,
        text: long_text.clone(),
        rtl: false,
    };
    let encoded = encode_to_vec(&entry).expect("Failed to encode long-text LocalizedString");
    let (decoded, _): (LocalizedString, usize) =
        decode_from_slice(&encoded).expect("Failed to decode long-text LocalizedString");
    assert_eq!(entry, decoded);
    assert_eq!(decoded.text.chars().count(), long_text.chars().count());
}

// Test 9: Mixed scripts and emoji inside a single LocalizedString
#[test]
fn test_mixed_scripts_and_emoji_text() {
    let entry = LocalizedString {
        language: Language::English,
        text: "Hello 世界 مرحبا Привет 🌍🦀🎉 Ångström Ñoño".to_string(),
        rtl: false,
    };
    let encoded = encode_to_vec(&entry).expect("Failed to encode mixed-script LocalizedString");
    let (decoded, _): (LocalizedString, usize) =
        decode_from_slice(&encoded).expect("Failed to decode mixed-script LocalizedString");
    assert_eq!(entry, decoded);
    // Verify specific scripts survive the roundtrip
    assert!(
        decoded.text.contains("世界"),
        "Chinese chars must survive roundtrip"
    );
    assert!(
        decoded.text.contains("مرحبا"),
        "Arabic chars must survive roundtrip"
    );
    assert!(
        decoded.text.contains("Привет"),
        "Russian chars must survive roundtrip"
    );
    assert!(decoded.text.contains("🌍"), "Emoji must survive roundtrip");
}

// Test 10: String with null bytes (\0) and control characters
#[test]
fn test_null_bytes_and_control_chars_in_text() {
    // Rust String can hold interior null bytes — oxicode must preserve them
    let entry = LocalizedString {
        language: Language::English,
        text: "start\0middle\0end\x01\x02\x1f".to_string(),
        rtl: false,
    };
    let encoded = encode_to_vec(&entry).expect("Failed to encode null-byte LocalizedString");
    let (decoded, _): (LocalizedString, usize) =
        decode_from_slice(&encoded).expect("Failed to decode null-byte LocalizedString");
    assert_eq!(entry, decoded);
    assert!(
        decoded.text.contains('\0'),
        "Null byte must be preserved after roundtrip"
    );
    assert!(
        decoded.text.contains('\x1f'),
        "Control char 0x1F must be preserved after roundtrip"
    );
}

// Test 11: Special Unicode code points (BMP edge cases and SMP surrogates in Rust are impossible,
// but we test high-BMP and supplementary plane chars including zero-width joiners)
#[test]
fn test_special_unicode_codepoints() {
    // U+200B ZERO WIDTH SPACE, U+200D ZERO WIDTH JOINER, U+FEFF BOM, U+1F600 GRINNING FACE
    let text = "\u{200B}\u{200D}\u{FEFF}😀\u{10FFFF}".to_string();
    let entry = LocalizedString {
        language: Language::English,
        text: text.clone(),
        rtl: false,
    };
    let encoded =
        encode_to_vec(&entry).expect("Failed to encode special-codepoints LocalizedString");
    let (decoded, _): (LocalizedString, usize) =
        decode_from_slice(&encoded).expect("Failed to decode special-codepoints LocalizedString");
    assert_eq!(entry, decoded);
    assert_eq!(decoded.text, text);
}

// Test 12: TranslationSet with multiple language translations
#[test]
fn test_translation_set_multiple_languages_roundtrip() {
    let set = TranslationSet {
        key: "greeting".to_string(),
        translations: vec![
            LocalizedString {
                language: Language::English,
                text: "Hello, World!".to_string(),
                rtl: false,
            },
            LocalizedString {
                language: Language::Japanese,
                text: "こんにちは、世界！".to_string(),
                rtl: false,
            },
            LocalizedString {
                language: Language::Arabic,
                text: "مرحبًا، أيها العالم!".to_string(),
                rtl: true,
            },
            LocalizedString {
                language: Language::Russian,
                text: "Привет, мир!".to_string(),
                rtl: false,
            },
        ],
    };
    let encoded = encode_to_vec(&set).expect("Failed to encode TranslationSet");
    let (decoded, _): (TranslationSet, usize) =
        decode_from_slice(&encoded).expect("Failed to decode TranslationSet");
    assert_eq!(set, decoded);
    assert_eq!(decoded.translations.len(), 4);
    assert!(
        decoded.translations[2].rtl,
        "Arabic translation must retain RTL flag"
    );
}

// Test 13: Vec<LocalizedString> with all eight languages
#[test]
fn test_vec_of_all_eight_languages_roundtrip() {
    let strings: Vec<LocalizedString> = vec![
        LocalizedString {
            language: Language::English,
            text: "Hello".to_string(),
            rtl: false,
        },
        LocalizedString {
            language: Language::Japanese,
            text: "日本語".to_string(),
            rtl: false,
        },
        LocalizedString {
            language: Language::Chinese,
            text: "中文".to_string(),
            rtl: false,
        },
        LocalizedString {
            language: Language::Arabic,
            text: "العربية".to_string(),
            rtl: true,
        },
        LocalizedString {
            language: Language::Russian,
            text: "Русский".to_string(),
            rtl: false,
        },
        LocalizedString {
            language: Language::Spanish,
            text: "Español".to_string(),
            rtl: false,
        },
        LocalizedString {
            language: Language::French,
            text: "Français".to_string(),
            rtl: false,
        },
        LocalizedString {
            language: Language::German,
            text: "Deutsch".to_string(),
            rtl: false,
        },
    ];
    let encoded = encode_to_vec(&strings).expect("Failed to encode Vec<LocalizedString>");
    let (decoded, _): (Vec<LocalizedString>, usize) =
        decode_from_slice(&encoded).expect("Failed to decode Vec<LocalizedString>");
    assert_eq!(strings, decoded);
    assert_eq!(decoded.len(), 8);
}

// Test 14: HashMap<String, TranslationSet> roundtrip
#[test]
fn test_hashmap_string_to_translation_set_roundtrip() {
    let mut map: HashMap<String, TranslationSet> = HashMap::new();

    map.insert(
        "farewell".to_string(),
        TranslationSet {
            key: "farewell".to_string(),
            translations: vec![
                LocalizedString {
                    language: Language::English,
                    text: "Goodbye".to_string(),
                    rtl: false,
                },
                LocalizedString {
                    language: Language::Spanish,
                    text: "Adiós".to_string(),
                    rtl: false,
                },
            ],
        },
    );

    map.insert(
        "thanks".to_string(),
        TranslationSet {
            key: "thanks".to_string(),
            translations: vec![
                LocalizedString {
                    language: Language::French,
                    text: "Merci".to_string(),
                    rtl: false,
                },
                LocalizedString {
                    language: Language::German,
                    text: "Danke".to_string(),
                    rtl: false,
                },
                LocalizedString {
                    language: Language::Japanese,
                    text: "ありがとう".to_string(),
                    rtl: false,
                },
            ],
        },
    );

    let encoded = encode_to_vec(&map).expect("Failed to encode HashMap<String, TranslationSet>");
    let (decoded, _): (HashMap<String, TranslationSet>, usize) =
        decode_from_slice(&encoded).expect("Failed to decode HashMap<String, TranslationSet>");
    assert_eq!(map, decoded);
    assert_eq!(
        decoded
            .get("thanks")
            .expect("key 'thanks' must exist")
            .translations
            .len(),
        3
    );
}

// Test 15: Big-endian config roundtrip for TranslationSet
#[test]
fn test_big_endian_config_translation_set_roundtrip() {
    let set = TranslationSet {
        key: "welcome".to_string(),
        translations: vec![
            LocalizedString {
                language: Language::Chinese,
                text: "欢迎使用".to_string(),
                rtl: false,
            },
            LocalizedString {
                language: Language::Arabic,
                text: "أهلاً وسهلاً".to_string(),
                rtl: true,
            },
        ],
    };
    let cfg = config::standard().with_big_endian();
    let encoded =
        encode_to_vec_with_config(&set, cfg).expect("Failed to encode TranslationSet (BE)");
    let (decoded, _): (TranslationSet, usize) =
        decode_from_slice_with_config(&encoded, cfg).expect("Failed to decode TranslationSet (BE)");
    assert_eq!(set, decoded);
}

// Test 16: Fixed-int encoding config roundtrip for LocalizedString
#[test]
fn test_fixed_int_config_localized_string_roundtrip() {
    let entry = LocalizedString {
        language: Language::Russian,
        text: "Тест с фиксированными целыми числами".to_string(),
        rtl: false,
    };
    let cfg = config::standard().with_fixed_int_encoding();
    let encoded =
        encode_to_vec_with_config(&entry, cfg).expect("Failed to encode LocalizedString (FI)");
    let (decoded, _): (LocalizedString, usize) = decode_from_slice_with_config(&encoded, cfg)
        .expect("Failed to decode LocalizedString (FI)");
    assert_eq!(entry, decoded);
}

// Test 17: Consumed bytes equal total encoded length (verification)
#[test]
fn test_consumed_bytes_equals_encoded_length_for_localized_string() {
    let entry = LocalizedString {
        language: Language::German,
        text: "Straße München Köln".to_string(),
        rtl: false,
    };
    let encoded = encode_to_vec(&entry).expect("Failed to encode German LocalizedString");
    let total_len = encoded.len();
    let (_, consumed): (LocalizedString, usize) =
        decode_from_slice(&encoded).expect("Failed to decode German LocalizedString");
    assert_eq!(
        consumed, total_len,
        "Consumed bytes ({consumed}) must equal total encoded length ({total_len})"
    );
}

// Test 18: Consumed bytes for TranslationSet are correct
#[test]
fn test_consumed_bytes_correct_for_translation_set() {
    let set = TranslationSet {
        key: "test_key".to_string(),
        translations: vec![LocalizedString {
            language: Language::Spanish,
            text: "¡Hola, mundo! ¿Cómo estás?".to_string(),
            rtl: false,
        }],
    };
    let encoded = encode_to_vec(&set).expect("Failed to encode TranslationSet for byte check");
    let total_len = encoded.len();
    let (_, consumed): (TranslationSet, usize) =
        decode_from_slice(&encoded).expect("Failed to decode TranslationSet for byte check");
    assert_eq!(
        consumed, total_len,
        "Consumed bytes ({consumed}) must equal total encoded length ({total_len})"
    );
}

// Test 19: TranslationSet with empty translations Vec
#[test]
fn test_translation_set_with_empty_translations_vec() {
    let set = TranslationSet {
        key: "untranslated_key".to_string(),
        translations: Vec::new(),
    };
    let encoded =
        encode_to_vec(&set).expect("Failed to encode TranslationSet with empty translations");
    let (decoded, _): (TranslationSet, usize) = decode_from_slice(&encoded)
        .expect("Failed to decode TranslationSet with empty translations");
    assert_eq!(set, decoded);
    assert!(
        decoded.translations.is_empty(),
        "Translations must remain empty"
    );
}

// Test 20: All four config combinations (varint/fixed-int × BE/LE) roundtrip correctly;
// fixed-int BE and fixed-int LE must produce different byte streams for the same string.
#[test]
fn test_all_four_endian_fixedint_config_combinations_roundtrip() {
    let entry = LocalizedString {
        language: Language::French,
        text: "L'été est magnifique — été".to_string(),
        rtl: false,
    };

    // Varint big-endian
    let cfg_be = config::standard().with_big_endian();
    let enc_be = encode_to_vec_with_config(&entry, cfg_be).expect("Failed to encode (varint BE)");
    let (dec_be, consumed_be): (LocalizedString, usize) =
        decode_from_slice_with_config(&enc_be, cfg_be).expect("Failed to decode (varint BE)");
    assert_eq!(entry, dec_be, "Roundtrip mismatch (varint BE)");
    assert_eq!(consumed_be, enc_be.len(), "Consumed mismatch (varint BE)");

    // Varint little-endian
    let cfg_le = config::standard().with_little_endian();
    let enc_le = encode_to_vec_with_config(&entry, cfg_le).expect("Failed to encode (varint LE)");
    let (dec_le, consumed_le): (LocalizedString, usize) =
        decode_from_slice_with_config(&enc_le, cfg_le).expect("Failed to decode (varint LE)");
    assert_eq!(entry, dec_le, "Roundtrip mismatch (varint LE)");
    assert_eq!(consumed_le, enc_le.len(), "Consumed mismatch (varint LE)");

    // Fixed-int big-endian
    let cfg_fi_be = config::standard()
        .with_fixed_int_encoding()
        .with_big_endian();
    let enc_fi_be =
        encode_to_vec_with_config(&entry, cfg_fi_be).expect("Failed to encode (fixed-int BE)");
    let (dec_fi_be, consumed_fi_be): (LocalizedString, usize) =
        decode_from_slice_with_config(&enc_fi_be, cfg_fi_be)
            .expect("Failed to decode (fixed-int BE)");
    assert_eq!(entry, dec_fi_be, "Roundtrip mismatch (fixed-int BE)");
    assert_eq!(
        consumed_fi_be,
        enc_fi_be.len(),
        "Consumed mismatch (fixed-int BE)"
    );

    // Fixed-int little-endian
    let cfg_fi_le = config::standard()
        .with_fixed_int_encoding()
        .with_little_endian();
    let enc_fi_le =
        encode_to_vec_with_config(&entry, cfg_fi_le).expect("Failed to encode (fixed-int LE)");
    let (dec_fi_le, consumed_fi_le): (LocalizedString, usize) =
        decode_from_slice_with_config(&enc_fi_le, cfg_fi_le)
            .expect("Failed to decode (fixed-int LE)");
    assert_eq!(entry, dec_fi_le, "Roundtrip mismatch (fixed-int LE)");
    assert_eq!(
        consumed_fi_le,
        enc_fi_le.len(),
        "Consumed mismatch (fixed-int LE)"
    );

    // Fixed-int BE and fixed-int LE must produce different byte streams for a non-trivial string
    // because the 64-bit length prefix is stored in different byte order.
    assert_ne!(
        enc_fi_be, enc_fi_le,
        "Fixed-int BE and fixed-int LE must produce different byte streams"
    );
}

// Test 21: Emoji-only string inside LocalizedString preserves byte length
#[test]
fn test_emoji_only_string_byte_length_preservation() {
    // Each emoji in this list is exactly 4 bytes in UTF-8
    let emoji_text = "🎉🌍🚀🦀💯🏆🔥🎊".to_string();
    let original_char_count = emoji_text.chars().count();
    let original_byte_len = emoji_text.len();

    // Verify all are 4-byte UTF-8 before encoding
    for ch in emoji_text.chars() {
        assert_eq!(
            ch.len_utf8(),
            4,
            "Expected 4-byte emoji, got {:?} ({} bytes)",
            ch,
            ch.len_utf8()
        );
    }

    let entry = LocalizedString {
        language: Language::English,
        text: emoji_text,
        rtl: false,
    };
    let encoded = encode_to_vec(&entry).expect("Failed to encode emoji-only LocalizedString");
    let (decoded, _): (LocalizedString, usize) =
        decode_from_slice(&encoded).expect("Failed to decode emoji-only LocalizedString");
    assert_eq!(entry, decoded);
    assert_eq!(decoded.text.chars().count(), original_char_count);
    assert_eq!(decoded.text.len(), original_byte_len);
    // Each char is 4 bytes
    assert_eq!(decoded.text.len(), original_char_count * 4);
}

// Test 22: Vec of TranslationSets with deep nesting, varied scripts, and multiple configs
#[test]
fn test_vec_of_translation_sets_deep_nesting_multiple_configs() {
    let sets = vec![
        TranslationSet {
            key: "app_name".to_string(),
            translations: vec![
                LocalizedString {
                    language: Language::English,
                    text: "Application Name".to_string(),
                    rtl: false,
                },
                LocalizedString {
                    language: Language::Japanese,
                    text: "アプリケーション名".to_string(),
                    rtl: false,
                },
                LocalizedString {
                    language: Language::Arabic,
                    text: "اسم التطبيق".to_string(),
                    rtl: true,
                },
            ],
        },
        TranslationSet {
            key: "error_message".to_string(),
            translations: vec![
                LocalizedString {
                    language: Language::Chinese,
                    text: "发生了错误，请重试。".to_string(),
                    rtl: false,
                },
                LocalizedString {
                    language: Language::Russian,
                    text: "Произошла ошибка. Попробуйте снова.".to_string(),
                    rtl: false,
                },
                LocalizedString {
                    language: Language::German,
                    text: "Ein Fehler ist aufgetreten. Bitte erneut versuchen.".to_string(),
                    rtl: false,
                },
                LocalizedString {
                    language: Language::Spanish,
                    text: "Se produjo un error. Por favor, inténtelo de nuevo.".to_string(),
                    rtl: false,
                },
                LocalizedString {
                    language: Language::French,
                    text: "Une erreur s'est produite. Veuillez réessayer.".to_string(),
                    rtl: false,
                },
            ],
        },
    ];

    // Default config roundtrip
    let encoded_default =
        encode_to_vec(&sets).expect("Failed to encode Vec<TranslationSet> (default)");
    let (decoded_default, consumed): (Vec<TranslationSet>, usize) =
        decode_from_slice(&encoded_default)
            .expect("Failed to decode Vec<TranslationSet> (default)");
    assert_eq!(sets, decoded_default);
    assert_eq!(consumed, encoded_default.len());

    // Big-endian config roundtrip
    let cfg_be = config::standard().with_big_endian();
    let encoded_be = encode_to_vec_with_config(&sets, cfg_be)
        .expect("Failed to encode Vec<TranslationSet> (BE)");
    let (decoded_be, _): (Vec<TranslationSet>, usize) =
        decode_from_slice_with_config(&encoded_be, cfg_be)
            .expect("Failed to decode Vec<TranslationSet> (BE)");
    assert_eq!(sets, decoded_be);

    // Verify structure integrity
    assert_eq!(decoded_default.len(), 2);
    assert_eq!(
        decoded_default[0].translations.len(),
        3,
        "app_name must have 3 translations"
    );
    assert_eq!(
        decoded_default[1].translations.len(),
        5,
        "error_message must have 5 translations"
    );
    // Verify RTL flag on Arabic entry survived
    assert!(
        decoded_default[0].translations[2].rtl,
        "Arabic translation in app_name must retain RTL flag"
    );
}
