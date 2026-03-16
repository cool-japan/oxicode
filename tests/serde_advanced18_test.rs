#![cfg(feature = "serde")]
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
    config,
    serde::{decode_owned_from_slice, encode_to_vec},
};
use serde::{Deserialize, Serialize};

#[derive(Debug, PartialEq, Serialize, Deserialize)]
enum LogLevel {
    Trace,
    Debug,
    Info,
    Warn,
    Error,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
struct DatabaseConfig {
    host: String,
    port: u16,
    max_connections: u32,
    timeout_ms: u64,
    ssl: bool,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
struct AppConfig {
    name: String,
    version: String,
    log_level: LogLevel,
    database: DatabaseConfig,
    feature_flags: Vec<String>,
    max_retries: Option<u32>,
}

// ---- LogLevel roundtrip tests ----

#[test]
fn test_log_level_trace_roundtrip() {
    let val = LogLevel::Trace;
    let bytes = encode_to_vec(&val, config::standard()).expect("encode LogLevel::Trace");
    let (decoded, _): (LogLevel, usize) =
        decode_owned_from_slice(&bytes, config::standard()).expect("decode LogLevel::Trace");
    assert_eq!(val, decoded);
}

#[test]
fn test_log_level_debug_roundtrip() {
    let val = LogLevel::Debug;
    let bytes = encode_to_vec(&val, config::standard()).expect("encode LogLevel::Debug");
    let (decoded, _): (LogLevel, usize) =
        decode_owned_from_slice(&bytes, config::standard()).expect("decode LogLevel::Debug");
    assert_eq!(val, decoded);
}

#[test]
fn test_log_level_info_roundtrip() {
    let val = LogLevel::Info;
    let bytes = encode_to_vec(&val, config::standard()).expect("encode LogLevel::Info");
    let (decoded, _): (LogLevel, usize) =
        decode_owned_from_slice(&bytes, config::standard()).expect("decode LogLevel::Info");
    assert_eq!(val, decoded);
}

#[test]
fn test_log_level_warn_roundtrip() {
    let val = LogLevel::Warn;
    let bytes = encode_to_vec(&val, config::standard()).expect("encode LogLevel::Warn");
    let (decoded, _): (LogLevel, usize) =
        decode_owned_from_slice(&bytes, config::standard()).expect("decode LogLevel::Warn");
    assert_eq!(val, decoded);
}

#[test]
fn test_log_level_error_roundtrip() {
    let val = LogLevel::Error;
    let bytes = encode_to_vec(&val, config::standard()).expect("encode LogLevel::Error");
    let (decoded, _): (LogLevel, usize) =
        decode_owned_from_slice(&bytes, config::standard()).expect("decode LogLevel::Error");
    assert_eq!(val, decoded);
}

// ---- DatabaseConfig roundtrip tests ----

#[test]
fn test_database_config_typical_roundtrip() {
    let val = DatabaseConfig {
        host: "db.example.com".to_string(),
        port: 5432,
        max_connections: 100,
        timeout_ms: 3000,
        ssl: true,
    };
    let bytes = encode_to_vec(&val, config::standard()).expect("encode DatabaseConfig typical");
    let (decoded, _): (DatabaseConfig, usize) =
        decode_owned_from_slice(&bytes, config::standard()).expect("decode DatabaseConfig typical");
    assert_eq!(val, decoded);
}

#[test]
fn test_database_config_zero_values_roundtrip() {
    let val = DatabaseConfig {
        host: String::new(),
        port: 0,
        max_connections: 0,
        timeout_ms: 0,
        ssl: false,
    };
    let bytes = encode_to_vec(&val, config::standard()).expect("encode DatabaseConfig zero values");
    let (decoded, _): (DatabaseConfig, usize) = decode_owned_from_slice(&bytes, config::standard())
        .expect("decode DatabaseConfig zero values");
    assert_eq!(val, decoded);
}

// ---- AppConfig roundtrip tests ----

#[test]
fn test_app_config_with_none_max_retries_roundtrip() {
    let val = AppConfig {
        name: "my-service".to_string(),
        version: "1.0.0".to_string(),
        log_level: LogLevel::Info,
        database: DatabaseConfig {
            host: "localhost".to_string(),
            port: 3306,
            max_connections: 50,
            timeout_ms: 1000,
            ssl: false,
        },
        feature_flags: vec!["dark-mode".to_string(), "beta-api".to_string()],
        max_retries: None,
    };
    let bytes =
        encode_to_vec(&val, config::standard()).expect("encode AppConfig with None max_retries");
    let (decoded, _): (AppConfig, usize) = decode_owned_from_slice(&bytes, config::standard())
        .expect("decode AppConfig with None max_retries");
    assert_eq!(val, decoded);
}

#[test]
fn test_app_config_with_some_max_retries_roundtrip() {
    let val = AppConfig {
        name: "retry-service".to_string(),
        version: "2.3.1".to_string(),
        log_level: LogLevel::Warn,
        database: DatabaseConfig {
            host: "pg.internal".to_string(),
            port: 5432,
            max_connections: 200,
            timeout_ms: 5000,
            ssl: true,
        },
        feature_flags: vec!["auto-retry".to_string()],
        max_retries: Some(5),
    };
    let bytes =
        encode_to_vec(&val, config::standard()).expect("encode AppConfig with Some max_retries");
    let (decoded, _): (AppConfig, usize) = decode_owned_from_slice(&bytes, config::standard())
        .expect("decode AppConfig with Some max_retries");
    assert_eq!(val, decoded);
}

#[test]
fn test_app_config_empty_feature_flags_roundtrip() {
    let val = AppConfig {
        name: "minimal-service".to_string(),
        version: "0.1.0".to_string(),
        log_level: LogLevel::Debug,
        database: DatabaseConfig {
            host: "127.0.0.1".to_string(),
            port: 5432,
            max_connections: 10,
            timeout_ms: 500,
            ssl: false,
        },
        feature_flags: vec![],
        max_retries: None,
    };
    let bytes =
        encode_to_vec(&val, config::standard()).expect("encode AppConfig empty feature_flags");
    let (decoded, _): (AppConfig, usize) = decode_owned_from_slice(&bytes, config::standard())
        .expect("decode AppConfig empty feature_flags");
    assert_eq!(val, decoded);
}

#[test]
fn test_app_config_large_feature_flags_roundtrip() {
    let feature_flags: Vec<String> = (0..50).map(|i| format!("feature-flag-{i:03}")).collect();
    let val = AppConfig {
        name: "feature-rich-service".to_string(),
        version: "3.0.0".to_string(),
        log_level: LogLevel::Trace,
        database: DatabaseConfig {
            host: "cluster.db.internal".to_string(),
            port: 6543,
            max_connections: 1000,
            timeout_ms: 10000,
            ssl: true,
        },
        feature_flags,
        max_retries: Some(3),
    };
    let bytes =
        encode_to_vec(&val, config::standard()).expect("encode AppConfig large feature_flags");
    let (decoded, _): (AppConfig, usize) = decode_owned_from_slice(&bytes, config::standard())
        .expect("decode AppConfig large feature_flags");
    assert_eq!(val, decoded);
}

#[test]
fn test_app_config_error_log_level_roundtrip() {
    let val = AppConfig {
        name: "critical-service".to_string(),
        version: "9.9.9".to_string(),
        log_level: LogLevel::Error,
        database: DatabaseConfig {
            host: "primary.db".to_string(),
            port: 5432,
            max_connections: 500,
            timeout_ms: 2000,
            ssl: true,
        },
        feature_flags: vec!["alerts".to_string()],
        max_retries: Some(1),
    };
    let bytes =
        encode_to_vec(&val, config::standard()).expect("encode AppConfig with Error log level");
    let (decoded, _): (AppConfig, usize) = decode_owned_from_slice(&bytes, config::standard())
        .expect("decode AppConfig with Error log level");
    assert_eq!(val, decoded);
}

// ---- Vec<AppConfig> roundtrip ----

#[test]
fn test_vec_app_config_roundtrip() {
    let val = vec![
        AppConfig {
            name: "service-a".to_string(),
            version: "1.0.0".to_string(),
            log_level: LogLevel::Info,
            database: DatabaseConfig {
                host: "db-a".to_string(),
                port: 5432,
                max_connections: 20,
                timeout_ms: 1000,
                ssl: false,
            },
            feature_flags: vec!["flag-x".to_string()],
            max_retries: None,
        },
        AppConfig {
            name: "service-b".to_string(),
            version: "2.1.0".to_string(),
            log_level: LogLevel::Warn,
            database: DatabaseConfig {
                host: "db-b".to_string(),
                port: 3306,
                max_connections: 40,
                timeout_ms: 2000,
                ssl: true,
            },
            feature_flags: vec!["flag-y".to_string(), "flag-z".to_string()],
            max_retries: Some(3),
        },
    ];
    let bytes = encode_to_vec(&val, config::standard()).expect("encode Vec<AppConfig>");
    let (decoded, _): (Vec<AppConfig>, usize) =
        decode_owned_from_slice(&bytes, config::standard()).expect("decode Vec<AppConfig>");
    assert_eq!(val, decoded);
}

// ---- Bytes consumed == encoded length ----

#[test]
fn test_bytes_consumed_equals_encoded_length() {
    let val = AppConfig {
        name: "length-check-service".to_string(),
        version: "1.2.3".to_string(),
        log_level: LogLevel::Debug,
        database: DatabaseConfig {
            host: "metrics.db".to_string(),
            port: 9000,
            max_connections: 25,
            timeout_ms: 750,
            ssl: false,
        },
        feature_flags: vec!["metrics".to_string(), "tracing".to_string()],
        max_retries: Some(2),
    };
    let bytes = encode_to_vec(&val, config::standard()).expect("encode for length check");
    let (_decoded, consumed): (AppConfig, usize) =
        decode_owned_from_slice(&bytes, config::standard()).expect("decode for length check");
    assert_eq!(consumed, bytes.len());
}

// ---- Encode determinism ----

#[test]
fn test_encode_determinism() {
    let val = AppConfig {
        name: "deterministic-service".to_string(),
        version: "1.0.0".to_string(),
        log_level: LogLevel::Info,
        database: DatabaseConfig {
            host: "stable.db".to_string(),
            port: 5432,
            max_connections: 10,
            timeout_ms: 500,
            ssl: true,
        },
        feature_flags: vec!["stable".to_string()],
        max_retries: None,
    };
    let bytes1 = encode_to_vec(&val, config::standard()).expect("encode determinism first");
    let bytes2 = encode_to_vec(&val, config::standard()).expect("encode determinism second");
    assert_eq!(bytes1, bytes2, "encoding must be deterministic");
}

// ---- Big-endian config ----

#[test]
fn test_database_config_big_endian_roundtrip() {
    let cfg = config::standard().with_big_endian();
    let val = DatabaseConfig {
        host: "big-endian-host".to_string(),
        port: 8080,
        max_connections: 300,
        timeout_ms: 4000,
        ssl: true,
    };
    let bytes = encode_to_vec(&val, cfg).expect("encode DatabaseConfig big-endian");
    let (decoded, _): (DatabaseConfig, usize) =
        decode_owned_from_slice(&bytes, cfg).expect("decode DatabaseConfig big-endian");
    assert_eq!(val, decoded);
}

// ---- Fixed-int config ----

#[test]
fn test_database_config_fixed_int_roundtrip() {
    let cfg = config::standard().with_fixed_int_encoding();
    let val = DatabaseConfig {
        host: "fixed-int-host".to_string(),
        port: 1433,
        max_connections: 75,
        timeout_ms: 3500,
        ssl: false,
    };
    let bytes = encode_to_vec(&val, cfg).expect("encode DatabaseConfig fixed-int");
    let (decoded, _): (DatabaseConfig, usize) =
        decode_owned_from_slice(&bytes, cfg).expect("decode DatabaseConfig fixed-int");
    assert_eq!(val, decoded);
}

// ---- Unicode in strings ----

#[test]
fn test_app_config_unicode_strings_roundtrip() {
    let val = AppConfig {
        name: "サービス・αβγ".to_string(),
        version: "1.0.0-ñ".to_string(),
        log_level: LogLevel::Info,
        database: DatabaseConfig {
            host: "データベース.internal".to_string(),
            port: 5432,
            max_connections: 10,
            timeout_ms: 1000,
            ssl: false,
        },
        feature_flags: vec!["機能フラグ".to_string(), "флаг-β".to_string()],
        max_retries: None,
    };
    let bytes = encode_to_vec(&val, config::standard()).expect("encode AppConfig unicode");
    let (decoded, _): (AppConfig, usize) =
        decode_owned_from_slice(&bytes, config::standard()).expect("decode AppConfig unicode");
    assert_eq!(val, decoded);
}

// ---- App config with Trace log level ----

#[test]
fn test_app_config_trace_log_level_roundtrip() {
    let val = AppConfig {
        name: "trace-service".to_string(),
        version: "0.0.1".to_string(),
        log_level: LogLevel::Trace,
        database: DatabaseConfig {
            host: "trace.db".to_string(),
            port: 5432,
            max_connections: 5,
            timeout_ms: 100,
            ssl: false,
        },
        feature_flags: vec!["verbose".to_string()],
        max_retries: None,
    };
    let bytes = encode_to_vec(&val, config::standard()).expect("encode AppConfig Trace log level");
    let (decoded, _): (AppConfig, usize) = decode_owned_from_slice(&bytes, config::standard())
        .expect("decode AppConfig Trace log level");
    assert_eq!(val, decoded);
}

// ---- App config with Debug log level ----

#[test]
fn test_app_config_debug_log_level_roundtrip() {
    let val = AppConfig {
        name: "debug-service".to_string(),
        version: "0.5.0".to_string(),
        log_level: LogLevel::Debug,
        database: DatabaseConfig {
            host: "dev.db".to_string(),
            port: 5433,
            max_connections: 8,
            timeout_ms: 250,
            ssl: false,
        },
        feature_flags: vec!["debug-ui".to_string(), "hot-reload".to_string()],
        max_retries: Some(0),
    };
    let bytes = encode_to_vec(&val, config::standard()).expect("encode AppConfig Debug log level");
    let (decoded, _): (AppConfig, usize) = decode_owned_from_slice(&bytes, config::standard())
        .expect("decode AppConfig Debug log level");
    assert_eq!(val, decoded);
}

// ---- App config with big-endian config ----

#[test]
fn test_app_config_big_endian_roundtrip() {
    let cfg = config::standard().with_big_endian();
    let val = AppConfig {
        name: "big-endian-app".to_string(),
        version: "4.0.0".to_string(),
        log_level: LogLevel::Warn,
        database: DatabaseConfig {
            host: "be.db.host".to_string(),
            port: 5432,
            max_connections: 150,
            timeout_ms: 6000,
            ssl: true,
        },
        feature_flags: vec!["network-opt".to_string()],
        max_retries: Some(4),
    };
    let bytes = encode_to_vec(&val, cfg).expect("encode AppConfig big-endian");
    let (decoded, _): (AppConfig, usize) =
        decode_owned_from_slice(&bytes, cfg).expect("decode AppConfig big-endian");
    assert_eq!(val, decoded);
}

// ---- App config with all zero-value fields ----

#[test]
fn test_app_config_zero_value_fields_roundtrip() {
    let val = AppConfig {
        name: String::new(),
        version: String::new(),
        log_level: LogLevel::Trace,
        database: DatabaseConfig {
            host: String::new(),
            port: 0,
            max_connections: 0,
            timeout_ms: 0,
            ssl: false,
        },
        feature_flags: vec![],
        max_retries: None,
    };
    let bytes =
        encode_to_vec(&val, config::standard()).expect("encode AppConfig zero-value fields");
    let (decoded, _): (AppConfig, usize) = decode_owned_from_slice(&bytes, config::standard())
        .expect("decode AppConfig zero-value fields");
    assert_eq!(val, decoded);
}
