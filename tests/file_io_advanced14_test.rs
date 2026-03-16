//! File I/O advanced tests — User preferences / settings persistence
//!
//! Theme, Language, UserPreferences encode/decode to/from files.

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
use oxicode::{decode_from_file, decode_from_slice, encode_to_file, encode_to_vec, Decode, Encode};
use std::env::temp_dir;

#[derive(Debug, PartialEq, Encode, Decode)]
enum Theme {
    Light,
    Dark,
    HighContrast,
    System,
}

#[derive(Debug, PartialEq, Encode, Decode)]
enum Language {
    English,
    Japanese,
    Chinese,
    Spanish,
    French,
    German,
}

#[derive(Debug, PartialEq, Encode, Decode)]
struct NotificationSettings {
    email: bool,
    push: bool,
    sms: bool,
    frequency_hours: u8,
}

#[derive(Debug, PartialEq, Encode, Decode)]
struct UserPreferences {
    user_id: u64,
    theme: Theme,
    language: Language,
    notifications: NotificationSettings,
    font_size: u8,
    custom_tags: Vec<String>,
}

// ── test 1: basic UserPreferences file roundtrip ──────────────────────────────

#[test]
fn test_user_preferences_file_roundtrip() {
    let prefs = UserPreferences {
        user_id: 1001,
        theme: Theme::Dark,
        language: Language::English,
        notifications: NotificationSettings {
            email: true,
            push: false,
            sms: false,
            frequency_hours: 24,
        },
        font_size: 14,
        custom_tags: vec!["work".to_string(), "urgent".to_string()],
    };
    let path = temp_dir().join("oxicode_fileio14_test1.bin");
    encode_to_file(&prefs, &path).expect("encode_to_file failed");
    let decoded: UserPreferences = decode_from_file(&path).expect("decode_from_file failed");
    assert_eq!(prefs, decoded);
    std::fs::remove_file(&path).ok();
}

// ── test 2: Theme::Light roundtrip ───────────────────────────────────────────

#[test]
fn test_theme_light_roundtrip() {
    let theme = Theme::Light;
    let path = temp_dir().join("oxicode_fileio14_test2.bin");
    encode_to_file(&theme, &path).expect("encode Theme::Light");
    let decoded: Theme = decode_from_file(&path).expect("decode Theme::Light");
    assert_eq!(Theme::Light, decoded);
    std::fs::remove_file(&path).ok();
}

// ── test 3: Theme::Dark roundtrip ────────────────────────────────────────────

#[test]
fn test_theme_dark_roundtrip() {
    let theme = Theme::Dark;
    let path = temp_dir().join("oxicode_fileio14_test3.bin");
    encode_to_file(&theme, &path).expect("encode Theme::Dark");
    let decoded: Theme = decode_from_file(&path).expect("decode Theme::Dark");
    assert_eq!(Theme::Dark, decoded);
    std::fs::remove_file(&path).ok();
}

// ── test 4: Theme::HighContrast roundtrip ────────────────────────────────────

#[test]
fn test_theme_high_contrast_roundtrip() {
    let theme = Theme::HighContrast;
    let path = temp_dir().join("oxicode_fileio14_test4.bin");
    encode_to_file(&theme, &path).expect("encode Theme::HighContrast");
    let decoded: Theme = decode_from_file(&path).expect("decode Theme::HighContrast");
    assert_eq!(Theme::HighContrast, decoded);
    std::fs::remove_file(&path).ok();
}

// ── test 5: Theme::System roundtrip ──────────────────────────────────────────

#[test]
fn test_theme_system_roundtrip() {
    let theme = Theme::System;
    let path = temp_dir().join("oxicode_fileio14_test5.bin");
    encode_to_file(&theme, &path).expect("encode Theme::System");
    let decoded: Theme = decode_from_file(&path).expect("decode Theme::System");
    assert_eq!(Theme::System, decoded);
    std::fs::remove_file(&path).ok();
}

// ── test 6: Language::Japanese roundtrip ─────────────────────────────────────

#[test]
fn test_language_japanese_roundtrip() {
    let lang = Language::Japanese;
    let path = temp_dir().join("oxicode_fileio14_test6.bin");
    encode_to_file(&lang, &path).expect("encode Language::Japanese");
    let decoded: Language = decode_from_file(&path).expect("decode Language::Japanese");
    assert_eq!(Language::Japanese, decoded);
    std::fs::remove_file(&path).ok();
}

// ── test 7: Language::Chinese roundtrip ──────────────────────────────────────

#[test]
fn test_language_chinese_roundtrip() {
    let lang = Language::Chinese;
    let path = temp_dir().join("oxicode_fileio14_test7.bin");
    encode_to_file(&lang, &path).expect("encode Language::Chinese");
    let decoded: Language = decode_from_file(&path).expect("decode Language::Chinese");
    assert_eq!(Language::Chinese, decoded);
    std::fs::remove_file(&path).ok();
}

// ── test 8: Language::Spanish roundtrip ──────────────────────────────────────

#[test]
fn test_language_spanish_roundtrip() {
    let lang = Language::Spanish;
    let path = temp_dir().join("oxicode_fileio14_test8.bin");
    encode_to_file(&lang, &path).expect("encode Language::Spanish");
    let decoded: Language = decode_from_file(&path).expect("decode Language::Spanish");
    assert_eq!(Language::Spanish, decoded);
    std::fs::remove_file(&path).ok();
}

// ── test 9: Language::French roundtrip ───────────────────────────────────────

#[test]
fn test_language_french_roundtrip() {
    let lang = Language::French;
    let path = temp_dir().join("oxicode_fileio14_test9.bin");
    encode_to_file(&lang, &path).expect("encode Language::French");
    let decoded: Language = decode_from_file(&path).expect("decode Language::French");
    assert_eq!(Language::French, decoded);
    std::fs::remove_file(&path).ok();
}

// ── test 10: Language::German roundtrip ──────────────────────────────────────

#[test]
fn test_language_german_roundtrip() {
    let lang = Language::German;
    let path = temp_dir().join("oxicode_fileio14_test10.bin");
    encode_to_file(&lang, &path).expect("encode Language::German");
    let decoded: Language = decode_from_file(&path).expect("decode Language::German");
    assert_eq!(Language::German, decoded);
    std::fs::remove_file(&path).ok();
}

// ── test 11: NotificationSettings file roundtrip ─────────────────────────────

#[test]
fn test_notification_settings_roundtrip() {
    let notif = NotificationSettings {
        email: true,
        push: true,
        sms: false,
        frequency_hours: 6,
    };
    let path = temp_dir().join("oxicode_fileio14_test11.bin");
    encode_to_file(&notif, &path).expect("encode NotificationSettings");
    let decoded: NotificationSettings =
        decode_from_file(&path).expect("decode NotificationSettings");
    assert_eq!(notif, decoded);
    std::fs::remove_file(&path).ok();
}

// ── test 12: file bytes match encode_to_vec output ───────────────────────────

#[test]
fn test_file_bytes_match_encode_to_vec() {
    let prefs = UserPreferences {
        user_id: 9999,
        theme: Theme::HighContrast,
        language: Language::Japanese,
        notifications: NotificationSettings {
            email: false,
            push: true,
            sms: true,
            frequency_hours: 1,
        },
        font_size: 18,
        custom_tags: vec!["accessibility".to_string()],
    };
    let path = temp_dir().join("oxicode_fileio14_test12.bin");
    encode_to_file(&prefs, &path).expect("encode_to_file");
    let file_bytes = std::fs::read(&path).expect("read file bytes");
    let vec_bytes = encode_to_vec(&prefs).expect("encode_to_vec");
    assert_eq!(file_bytes, vec_bytes);
    std::fs::remove_file(&path).ok();
}

// ── test 13: decode_from_slice matches file decode ────────────────────────────

#[test]
fn test_decode_from_slice_matches_file_decode() {
    let prefs = UserPreferences {
        user_id: 42,
        theme: Theme::Light,
        language: Language::English,
        notifications: NotificationSettings {
            email: true,
            push: false,
            sms: false,
            frequency_hours: 12,
        },
        font_size: 12,
        custom_tags: vec!["personal".to_string(), "hobby".to_string()],
    };
    let path = temp_dir().join("oxicode_fileio14_test13.bin");
    encode_to_file(&prefs, &path).expect("encode_to_file");

    let file_decoded: UserPreferences = decode_from_file(&path).expect("decode_from_file");
    let bytes = std::fs::read(&path).expect("read file");
    let (slice_decoded, _): (UserPreferences, _) =
        decode_from_slice(&bytes).expect("decode_from_slice");

    assert_eq!(file_decoded, slice_decoded);
    std::fs::remove_file(&path).ok();
}

// ── test 14: overwrite / update scenario ─────────────────────────────────────

#[test]
fn test_overwrite_preferences_update_theme() {
    let path = temp_dir().join("oxicode_fileio14_test14.bin");

    let original = UserPreferences {
        user_id: 7,
        theme: Theme::Light,
        language: Language::English,
        notifications: NotificationSettings {
            email: false,
            push: false,
            sms: false,
            frequency_hours: 0,
        },
        font_size: 14,
        custom_tags: vec![],
    };
    encode_to_file(&original, &path).expect("encode original");

    let updated = UserPreferences {
        theme: Theme::Dark,
        ..UserPreferences {
            user_id: 7,
            theme: Theme::Dark,
            language: Language::English,
            notifications: NotificationSettings {
                email: false,
                push: false,
                sms: false,
                frequency_hours: 0,
            },
            font_size: 14,
            custom_tags: vec![],
        }
    };
    encode_to_file(&updated, &path).expect("encode updated");

    let decoded: UserPreferences = decode_from_file(&path).expect("decode updated");
    assert_eq!(Theme::Dark, decoded.theme);
    assert_eq!(7, decoded.user_id);
    std::fs::remove_file(&path).ok();
}

// ── test 15: large custom_tags vector ────────────────────────────────────────

#[test]
fn test_large_custom_tags_roundtrip() {
    let tags: Vec<String> = (0..500).map(|i| format!("tag_{i:04}")).collect();
    let prefs = UserPreferences {
        user_id: 200,
        theme: Theme::System,
        language: Language::German,
        notifications: NotificationSettings {
            email: true,
            push: true,
            sms: true,
            frequency_hours: 3,
        },
        font_size: 16,
        custom_tags: tags.clone(),
    };
    let path = temp_dir().join("oxicode_fileio14_test15.bin");
    encode_to_file(&prefs, &path).expect("encode large tags");
    let decoded: UserPreferences = decode_from_file(&path).expect("decode large tags");
    assert_eq!(prefs.custom_tags.len(), decoded.custom_tags.len());
    assert_eq!(tags, decoded.custom_tags);
    std::fs::remove_file(&path).ok();
}

// ── test 16: unicode in custom_tags ──────────────────────────────────────────

#[test]
fn test_unicode_custom_tags_roundtrip() {
    let tags = vec![
        "日本語タグ".to_string(),
        "中文标签".to_string(),
        "Ñoño".to_string(),
        "Ärger".to_string(),
        "café".to_string(),
        "🎉emoji🚀".to_string(),
        "한국어태그".to_string(),
        "العربية".to_string(),
    ];
    let prefs = UserPreferences {
        user_id: 300,
        theme: Theme::Light,
        language: Language::Japanese,
        notifications: NotificationSettings {
            email: false,
            push: true,
            sms: false,
            frequency_hours: 48,
        },
        font_size: 20,
        custom_tags: tags.clone(),
    };
    let path = temp_dir().join("oxicode_fileio14_test16.bin");
    encode_to_file(&prefs, &path).expect("encode unicode tags");
    let decoded: UserPreferences = decode_from_file(&path).expect("decode unicode tags");
    assert_eq!(tags, decoded.custom_tags);
    std::fs::remove_file(&path).ok();
}

// ── test 17: multiple distinct preference files persisted in parallel ─────────

#[test]
fn test_multiple_files_parallel_persist() {
    let users: Vec<UserPreferences> = vec![
        UserPreferences {
            user_id: 1,
            theme: Theme::Light,
            language: Language::English,
            notifications: NotificationSettings {
                email: true,
                push: false,
                sms: false,
                frequency_hours: 24,
            },
            font_size: 12,
            custom_tags: vec!["alpha".to_string()],
        },
        UserPreferences {
            user_id: 2,
            theme: Theme::Dark,
            language: Language::Japanese,
            notifications: NotificationSettings {
                email: false,
                push: true,
                sms: false,
                frequency_hours: 12,
            },
            font_size: 14,
            custom_tags: vec!["beta".to_string(), "gamma".to_string()],
        },
        UserPreferences {
            user_id: 3,
            theme: Theme::HighContrast,
            language: Language::Chinese,
            notifications: NotificationSettings {
                email: true,
                push: true,
                sms: true,
                frequency_hours: 1,
            },
            font_size: 18,
            custom_tags: vec![],
        },
    ];

    let paths: Vec<_> = (17..=19)
        .map(|i| temp_dir().join(format!("oxicode_fileio14_test{i}.bin")))
        .collect();

    for (user, path) in users.iter().zip(paths.iter()) {
        encode_to_file(user, path).expect("parallel encode");
    }

    for (user, path) in users.iter().zip(paths.iter()) {
        let decoded: UserPreferences = decode_from_file(path).expect("parallel decode");
        assert_eq!(user, &decoded);
        std::fs::remove_file(path).ok();
    }
}

// ── test 18: Language::English explicit file roundtrip ───────────────────────

#[test]
fn test_language_english_roundtrip() {
    let lang = Language::English;
    let path = temp_dir().join("oxicode_fileio14_test18.bin");
    encode_to_file(&lang, &path).expect("encode Language::English");
    let decoded: Language = decode_from_file(&path).expect("decode Language::English");
    assert_eq!(Language::English, decoded);
    std::fs::remove_file(&path).ok();
}

// ── test 19: all-false NotificationSettings roundtrip ────────────────────────

#[test]
fn test_notification_settings_all_false_roundtrip() {
    let notif = NotificationSettings {
        email: false,
        push: false,
        sms: false,
        frequency_hours: 0,
    };
    let path = temp_dir().join("oxicode_fileio14_test19.bin");
    encode_to_file(&notif, &path).expect("encode all-false NotificationSettings");
    let decoded: NotificationSettings =
        decode_from_file(&path).expect("decode all-false NotificationSettings");
    assert_eq!(notif, decoded);
    assert!(!decoded.email && !decoded.push && !decoded.sms);
    assert_eq!(0, decoded.frequency_hours);
    std::fs::remove_file(&path).ok();
}

// ── test 20: empty custom_tags roundtrip ─────────────────────────────────────

#[test]
fn test_empty_custom_tags_roundtrip() {
    let prefs = UserPreferences {
        user_id: 555,
        theme: Theme::System,
        language: Language::French,
        notifications: NotificationSettings {
            email: false,
            push: false,
            sms: false,
            frequency_hours: 0,
        },
        font_size: 11,
        custom_tags: vec![],
    };
    let path = temp_dir().join("oxicode_fileio14_test20.bin");
    encode_to_file(&prefs, &path).expect("encode empty tags");
    let decoded: UserPreferences = decode_from_file(&path).expect("decode empty tags");
    assert!(decoded.custom_tags.is_empty());
    assert_eq!(prefs, decoded);
    std::fs::remove_file(&path).ok();
}

// ── test 21: all Theme variants produce distinct byte representations ─────────

#[test]
fn test_all_theme_variants_distinct_encoding() {
    let variants = [
        Theme::Light,
        Theme::Dark,
        Theme::HighContrast,
        Theme::System,
    ];
    let mut encodings: Vec<Vec<u8>> = Vec::new();

    for (idx, variant) in variants.iter().enumerate() {
        let path = temp_dir().join(format!("oxicode_fileio14_test21_{idx}.bin"));
        encode_to_file(variant, &path).expect("encode theme variant");
        let bytes = std::fs::read(&path).expect("read theme bytes");
        encodings.push(bytes);
        std::fs::remove_file(&path).ok();
    }

    // All four encodings must be pairwise distinct
    for i in 0..encodings.len() {
        for j in (i + 1)..encodings.len() {
            assert_ne!(
                encodings[i], encodings[j],
                "Theme variants {i} and {j} must differ"
            );
        }
    }
}

// ── test 22: max font_size and all-true notifications roundtrip ───────────────

#[test]
fn test_max_font_size_all_notifications_enabled() {
    let prefs = UserPreferences {
        user_id: u64::MAX,
        theme: Theme::HighContrast,
        language: Language::Spanish,
        notifications: NotificationSettings {
            email: true,
            push: true,
            sms: true,
            frequency_hours: u8::MAX,
        },
        font_size: u8::MAX,
        custom_tags: vec![
            "max".to_string(),
            "boundary".to_string(),
            "u8::MAX".to_string(),
        ],
    };
    let path = temp_dir().join("oxicode_fileio14_test22.bin");
    encode_to_file(&prefs, &path).expect("encode max prefs");
    let decoded: UserPreferences = decode_from_file(&path).expect("decode max prefs");
    assert_eq!(u64::MAX, decoded.user_id);
    assert_eq!(u8::MAX, decoded.font_size);
    assert_eq!(u8::MAX, decoded.notifications.frequency_hours);
    assert_eq!(prefs, decoded);
    std::fs::remove_file(&path).ok();
}
