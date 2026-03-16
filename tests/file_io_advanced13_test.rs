//! Advanced file I/O tests — configuration management theme
//! Covers encode/decode to/from files using Environment, ConfigEntry, AppConfig types.

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

#[derive(Debug, PartialEq, Encode, Decode)]
enum Environment {
    Development,
    Staging,
    Production,
    Testing,
}

#[derive(Debug, PartialEq, Encode, Decode)]
struct ConfigEntry {
    key: String,
    value: String,
    is_secret: bool,
}

#[derive(Debug, PartialEq, Encode, Decode)]
struct AppConfig {
    env: Environment,
    entries: Vec<ConfigEntry>,
    version: u32,
    debug_enabled: bool,
}

// ── helpers ──────────────────────────────────────────────────────────────────

fn make_entry(key: &str, value: &str, is_secret: bool) -> ConfigEntry {
    ConfigEntry {
        key: key.to_string(),
        value: value.to_string(),
        is_secret,
    }
}

fn minimal_config(env: Environment) -> AppConfig {
    AppConfig {
        env,
        entries: vec![make_entry("host", "localhost", false)],
        version: 1,
        debug_enabled: false,
    }
}

fn tmp(name: &str) -> std::path::PathBuf {
    std::env::temp_dir().join(name)
}

// ── test 1: basic AppConfig file roundtrip ───────────────────────────────────

#[test]
fn test_app_config_file_roundtrip() {
    let val = AppConfig {
        env: Environment::Development,
        entries: vec![
            make_entry("host", "localhost", false),
            make_entry("db_password", "s3cr3t", true),
        ],
        version: 1,
        debug_enabled: true,
    };
    let path = tmp("oxicode_fileio13_test1.bin");
    encode_to_file(&val, &path).expect("encode to file");
    let decoded: AppConfig = decode_from_file(&path).expect("decode from file");
    assert_eq!(val, decoded);
    std::fs::remove_file(&path).ok();
}

// ── test 2: Environment::Development variant ────────────────────────────────

#[test]
fn test_environment_development_file_roundtrip() {
    let val = Environment::Development;
    let path = tmp("oxicode_fileio13_test2.bin");
    encode_to_file(&val, &path).expect("encode Development");
    let decoded: Environment = decode_from_file(&path).expect("decode Development");
    assert_eq!(val, decoded);
    std::fs::remove_file(&path).ok();
}

// ── test 3: Environment::Staging variant ─────────────────────────────────────

#[test]
fn test_environment_staging_file_roundtrip() {
    let val = Environment::Staging;
    let path = tmp("oxicode_fileio13_test3.bin");
    encode_to_file(&val, &path).expect("encode Staging");
    let decoded: Environment = decode_from_file(&path).expect("decode Staging");
    assert_eq!(val, decoded);
    std::fs::remove_file(&path).ok();
}

// ── test 4: Environment::Production variant ──────────────────────────────────

#[test]
fn test_environment_production_file_roundtrip() {
    let val = Environment::Production;
    let path = tmp("oxicode_fileio13_test4.bin");
    encode_to_file(&val, &path).expect("encode Production");
    let decoded: Environment = decode_from_file(&path).expect("decode Production");
    assert_eq!(val, decoded);
    std::fs::remove_file(&path).ok();
}

// ── test 5: Environment::Testing variant ────────────────────────────────────

#[test]
fn test_environment_testing_file_roundtrip() {
    let val = Environment::Testing;
    let path = tmp("oxicode_fileio13_test5.bin");
    encode_to_file(&val, &path).expect("encode Testing");
    let decoded: Environment = decode_from_file(&path).expect("decode Testing");
    assert_eq!(val, decoded);
    std::fs::remove_file(&path).ok();
}

// ── test 6: ConfigEntry file roundtrip ───────────────────────────────────────

#[test]
fn test_config_entry_file_roundtrip() {
    let val = make_entry("api_key", "abc123xyz", true);
    let path = tmp("oxicode_fileio13_test6.bin");
    encode_to_file(&val, &path).expect("encode ConfigEntry");
    let decoded: ConfigEntry = decode_from_file(&path).expect("decode ConfigEntry");
    assert_eq!(val, decoded);
    std::fs::remove_file(&path).ok();
}

// ── test 7: empty entries list ───────────────────────────────────────────────

#[test]
fn test_app_config_empty_entries() {
    let val = AppConfig {
        env: Environment::Production,
        entries: vec![],
        version: 0,
        debug_enabled: false,
    };
    let path = tmp("oxicode_fileio13_test7.bin");
    encode_to_file(&val, &path).expect("encode empty config");
    let decoded: AppConfig = decode_from_file(&path).expect("decode empty config");
    assert_eq!(val, decoded);
    std::fs::remove_file(&path).ok();
}

// ── test 8: config with many entries ─────────────────────────────────────────

#[test]
fn test_app_config_many_entries() {
    let entries: Vec<ConfigEntry> = (0..200)
        .map(|i| make_entry(&format!("key_{i}"), &format!("value_{i}"), i % 3 == 0))
        .collect();
    let val = AppConfig {
        env: Environment::Staging,
        entries,
        version: 42,
        debug_enabled: true,
    };
    let path = tmp("oxicode_fileio13_test8.bin");
    encode_to_file(&val, &path).expect("encode many entries");
    let decoded: AppConfig = decode_from_file(&path).expect("decode many entries");
    assert_eq!(val.entries.len(), decoded.entries.len());
    assert_eq!(val, decoded);
    std::fs::remove_file(&path).ok();
}

// ── test 9: large string values in entries ───────────────────────────────────

#[test]
fn test_config_entry_large_strings() {
    let val = make_entry(&"k".repeat(4096), &"v".repeat(8192), false);
    let path = tmp("oxicode_fileio13_test9.bin");
    encode_to_file(&val, &path).expect("encode large strings");
    let decoded: ConfigEntry = decode_from_file(&path).expect("decode large strings");
    assert_eq!(val.key.len(), decoded.key.len());
    assert_eq!(val.value.len(), decoded.value.len());
    assert_eq!(val, decoded);
    std::fs::remove_file(&path).ok();
}

// ── test 10: file bytes match encode_to_vec ───────────────────────────────────

#[test]
fn test_file_bytes_match_encode_to_vec() {
    let val = minimal_config(Environment::Development);
    let path = tmp("oxicode_fileio13_test10.bin");
    encode_to_file(&val, &path).expect("encode to file");
    let file_bytes = std::fs::read(&path).expect("read file bytes");
    let vec_bytes = encode_to_vec(&val).expect("encode to vec");
    assert_eq!(file_bytes, vec_bytes);
    std::fs::remove_file(&path).ok();
}

// ── test 11: decode_from_slice matches decode_from_file ───────────────────────

#[test]
fn test_decode_from_slice_matches_decode_from_file() {
    let val = minimal_config(Environment::Testing);
    let path = tmp("oxicode_fileio13_test11.bin");
    encode_to_file(&val, &path).expect("encode");
    let bytes = std::fs::read(&path).expect("read");
    let from_file: AppConfig = decode_from_file(&path).expect("decode from file");
    let (from_slice, _): (AppConfig, usize) = decode_from_slice(&bytes).expect("decode from slice");
    assert_eq!(from_file, from_slice);
    std::fs::remove_file(&path).ok();
}

// ── test 12: sequential overwrites, last value survives ──────────────────────

#[test]
fn test_sequential_overwrites_last_wins() {
    let path = tmp("oxicode_fileio13_test12.bin");
    let configs = [
        minimal_config(Environment::Development),
        minimal_config(Environment::Staging),
        minimal_config(Environment::Production),
    ];
    for cfg in &configs {
        encode_to_file(cfg, &path).expect("overwrite encode");
    }
    let decoded: AppConfig = decode_from_file(&path).expect("decode after overwrites");
    assert_eq!(decoded.env, Environment::Production);
    std::fs::remove_file(&path).ok();
}

// ── test 13: multiple distinct files simultaneously ───────────────────────────

#[test]
fn test_multiple_distinct_files() {
    let envs = [
        (Environment::Development, "oxicode_fileio13_test13a.bin"),
        (Environment::Staging, "oxicode_fileio13_test13b.bin"),
        (Environment::Production, "oxicode_fileio13_test13c.bin"),
        (Environment::Testing, "oxicode_fileio13_test13d.bin"),
    ];
    for (env, name) in &envs {
        let val = AppConfig {
            env: match env {
                Environment::Development => Environment::Development,
                Environment::Staging => Environment::Staging,
                Environment::Production => Environment::Production,
                Environment::Testing => Environment::Testing,
            },
            entries: vec![make_entry("label", name, false)],
            version: 1,
            debug_enabled: false,
        };
        let path = tmp(name);
        encode_to_file(&val, &path).expect("encode distinct");
        let decoded: AppConfig = decode_from_file(&path).expect("decode distinct");
        assert_eq!(val, decoded);
        std::fs::remove_file(&path).ok();
    }
}

// ── test 14: all-secrets config ──────────────────────────────────────────────

#[test]
fn test_app_config_all_secret_entries() {
    let entries: Vec<ConfigEntry> = (0..50)
        .map(|i| make_entry(&format!("secret_key_{i}"), &format!("secret_val_{i}"), true))
        .collect();
    let val = AppConfig {
        env: Environment::Production,
        entries,
        version: 99,
        debug_enabled: false,
    };
    let path = tmp("oxicode_fileio13_test14.bin");
    encode_to_file(&val, &path).expect("encode all-secrets");
    let decoded: AppConfig = decode_from_file(&path).expect("decode all-secrets");
    assert!(decoded.entries.iter().all(|e| e.is_secret));
    assert_eq!(val, decoded);
    std::fs::remove_file(&path).ok();
}

// ── test 15: no-secrets config ───────────────────────────────────────────────

#[test]
fn test_app_config_no_secret_entries() {
    let entries: Vec<ConfigEntry> = (0..50)
        .map(|i| {
            make_entry(
                &format!("public_key_{i}"),
                &format!("public_val_{i}"),
                false,
            )
        })
        .collect();
    let val = AppConfig {
        env: Environment::Development,
        entries,
        version: 1,
        debug_enabled: true,
    };
    let path = tmp("oxicode_fileio13_test15.bin");
    encode_to_file(&val, &path).expect("encode no-secrets");
    let decoded: AppConfig = decode_from_file(&path).expect("decode no-secrets");
    assert!(decoded.entries.iter().all(|e| !e.is_secret));
    assert_eq!(val, decoded);
    std::fs::remove_file(&path).ok();
}

// ── test 16: debug_enabled=true persists ─────────────────────────────────────

#[test]
fn test_app_config_debug_enabled_true() {
    let val = AppConfig {
        env: Environment::Development,
        entries: vec![make_entry("log_level", "trace", false)],
        version: 2,
        debug_enabled: true,
    };
    let path = tmp("oxicode_fileio13_test16.bin");
    encode_to_file(&val, &path).expect("encode debug=true");
    let decoded: AppConfig = decode_from_file(&path).expect("decode debug=true");
    assert!(decoded.debug_enabled);
    assert_eq!(val, decoded);
    std::fs::remove_file(&path).ok();
}

// ── test 17: debug_enabled=false persists ────────────────────────────────────

#[test]
fn test_app_config_debug_enabled_false() {
    let val = AppConfig {
        env: Environment::Production,
        entries: vec![make_entry("log_level", "error", false)],
        version: 10,
        debug_enabled: false,
    };
    let path = tmp("oxicode_fileio13_test17.bin");
    encode_to_file(&val, &path).expect("encode debug=false");
    let decoded: AppConfig = decode_from_file(&path).expect("decode debug=false");
    assert!(!decoded.debug_enabled);
    assert_eq!(val, decoded);
    std::fs::remove_file(&path).ok();
}

// ── test 18: high version number roundtrip ───────────────────────────────────

#[test]
fn test_app_config_high_version_number() {
    let val = AppConfig {
        env: Environment::Production,
        entries: vec![make_entry("migration", "complete", false)],
        version: u32::MAX,
        debug_enabled: false,
    };
    let path = tmp("oxicode_fileio13_test18.bin");
    encode_to_file(&val, &path).expect("encode high version");
    let decoded: AppConfig = decode_from_file(&path).expect("decode high version");
    assert_eq!(decoded.version, u32::MAX);
    assert_eq!(val, decoded);
    std::fs::remove_file(&path).ok();
}

// ── test 19: config entry with empty key and value ────────────────────────────

#[test]
fn test_config_entry_empty_key_and_value() {
    let val = make_entry("", "", false);
    let path = tmp("oxicode_fileio13_test19.bin");
    encode_to_file(&val, &path).expect("encode empty key/value");
    let decoded: ConfigEntry = decode_from_file(&path).expect("decode empty key/value");
    assert_eq!(val, decoded);
    assert!(decoded.key.is_empty());
    assert!(decoded.value.is_empty());
    std::fs::remove_file(&path).ok();
}

// ── test 20: app config with unicode strings ──────────────────────────────────

#[test]
fn test_app_config_unicode_strings() {
    let val = AppConfig {
        env: Environment::Staging,
        entries: vec![
            make_entry("ключ", "значение", false),
            make_entry("鍵", "値", true),
            make_entry("مفتاح", "قيمة", false),
            make_entry("🔑", "🎉", true),
        ],
        version: 7,
        debug_enabled: false,
    };
    let path = tmp("oxicode_fileio13_test20.bin");
    encode_to_file(&val, &path).expect("encode unicode");
    let decoded: AppConfig = decode_from_file(&path).expect("decode unicode");
    assert_eq!(val.entries.len(), decoded.entries.len());
    assert_eq!(val, decoded);
    std::fs::remove_file(&path).ok();
}

// ── test 21: repeated read from same file stays consistent ────────────────────

#[test]
fn test_repeated_reads_from_same_file() {
    let val = AppConfig {
        env: Environment::Testing,
        entries: vec![
            make_entry("retry_count", "3", false),
            make_entry("timeout_ms", "5000", false),
        ],
        version: 5,
        debug_enabled: true,
    };
    let path = tmp("oxicode_fileio13_test21.bin");
    encode_to_file(&val, &path).expect("encode for repeated reads");
    for _ in 0..10 {
        let decoded: AppConfig = decode_from_file(&path).expect("repeated decode");
        assert_eq!(val, decoded);
    }
    std::fs::remove_file(&path).ok();
}

// ── test 22: large AppConfig file roundtrip (stress) ─────────────────────────

#[test]
fn test_large_app_config_file_roundtrip() {
    let entries: Vec<ConfigEntry> = (0..1_000)
        .map(|i| {
            make_entry(
                &format!("param_{i:05}"),
                &"x".repeat(256 + (i % 512)),
                i % 7 == 0,
            )
        })
        .collect();
    let val = AppConfig {
        env: Environment::Production,
        entries,
        version: 1_000,
        debug_enabled: false,
    };
    let path = tmp("oxicode_fileio13_test22.bin");
    encode_to_file(&val, &path).expect("encode large config");
    let decoded: AppConfig = decode_from_file(&path).expect("decode large config");
    assert_eq!(val.entries.len(), decoded.entries.len());
    assert_eq!(val.version, decoded.version);
    assert_eq!(val, decoded);
    std::fs::remove_file(&path).ok();
}
