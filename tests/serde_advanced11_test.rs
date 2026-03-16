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
use std::collections::HashMap;

use serde::{Deserialize, Serialize};

// ---------------------------------------------------------------------------
// Shared test types
// ---------------------------------------------------------------------------

#[derive(Debug, PartialEq, Serialize, Deserialize)]
struct LogEntry {
    level: u32,
    message: String,
    tags: Vec<String>,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
enum Severity {
    Low,
    Medium,
    High,
    Critical(String),
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
struct Report {
    entries: Vec<LogEntry>,
    summary: String,
    count: u32,
}

// ---------------------------------------------------------------------------
// Test 1: u32 primitive roundtrip via serde
// ---------------------------------------------------------------------------

#[test]
fn test_serde_u32_roundtrip() {
    let original: u32 = 4_294_967_295;
    let enc = oxicode::serde::encode_to_vec(&original, oxicode::config::standard())
        .expect("encode u32 failed");
    let (decoded, _): (u32, usize) =
        oxicode::serde::decode_owned_from_slice(&enc, oxicode::config::standard())
            .expect("decode u32 failed");
    assert_eq!(original, decoded);
}

// ---------------------------------------------------------------------------
// Test 2: String primitive roundtrip via serde
// ---------------------------------------------------------------------------

#[test]
fn test_serde_string_roundtrip() {
    let original = "oxicode serde integration test string".to_string();
    let enc = oxicode::serde::encode_to_vec(&original, oxicode::config::standard())
        .expect("encode String failed");
    let (decoded, _): (String, usize) =
        oxicode::serde::decode_owned_from_slice(&enc, oxicode::config::standard())
            .expect("decode String failed");
    assert_eq!(original, decoded);
}

// ---------------------------------------------------------------------------
// Test 3: bool primitive roundtrip via serde — both true and false
// ---------------------------------------------------------------------------

#[test]
fn test_serde_bool_roundtrip() {
    for original in [true, false] {
        let enc = oxicode::serde::encode_to_vec(&original, oxicode::config::standard())
            .expect("encode bool failed");
        let (decoded, _): (bool, usize) =
            oxicode::serde::decode_owned_from_slice(&enc, oxicode::config::standard())
                .expect("decode bool failed");
        assert_eq!(original, decoded);
    }
}

// ---------------------------------------------------------------------------
// Test 4: Vec<u8> primitive roundtrip via serde
// ---------------------------------------------------------------------------

#[test]
fn test_serde_vec_u8_roundtrip() {
    let original: Vec<u8> = vec![0x00, 0x01, 0x7F, 0x80, 0xFE, 0xFF, 0xAB, 0xCD];
    let enc = oxicode::serde::encode_to_vec(&original, oxicode::config::standard())
        .expect("encode Vec<u8> failed");
    let (decoded, _): (Vec<u8>, usize) =
        oxicode::serde::decode_owned_from_slice(&enc, oxicode::config::standard())
            .expect("decode Vec<u8> failed");
    assert_eq!(original, decoded);
}

// ---------------------------------------------------------------------------
// Test 5: f64 primitive roundtrip via serde
// ---------------------------------------------------------------------------

#[test]
fn test_serde_f64_roundtrip() {
    let cases: Vec<f64> = vec![
        0.0,
        -0.0,
        1.0,
        -1.0,
        std::f64::consts::E,
        std::f64::consts::PI,
        f64::MAX,
        f64::MIN_POSITIVE,
    ];
    for original in cases {
        let enc = oxicode::serde::encode_to_vec(&original, oxicode::config::standard())
            .expect("encode f64 failed");
        let (decoded, _): (f64, usize) =
            oxicode::serde::decode_owned_from_slice(&enc, oxicode::config::standard())
                .expect("decode f64 failed");
        assert_eq!(
            original.to_bits(),
            decoded.to_bits(),
            "f64 bits mismatch for {original}"
        );
    }
}

// ---------------------------------------------------------------------------
// Test 6: LogEntry struct roundtrip via serde
// ---------------------------------------------------------------------------

#[test]
fn test_serde_log_entry_roundtrip() {
    let original = LogEntry {
        level: 3,
        message: "application started successfully".to_string(),
        tags: vec![
            "startup".to_string(),
            "info".to_string(),
            "system".to_string(),
        ],
    };
    let enc = oxicode::serde::encode_to_vec(&original, oxicode::config::standard())
        .expect("encode LogEntry failed");
    let (decoded, _): (LogEntry, usize) =
        oxicode::serde::decode_owned_from_slice(&enc, oxicode::config::standard())
            .expect("decode LogEntry failed");
    assert_eq!(original, decoded);
}

// ---------------------------------------------------------------------------
// Test 7: LogEntry with empty tags vec roundtrip
// ---------------------------------------------------------------------------

#[test]
fn test_serde_log_entry_empty_tags_roundtrip() {
    let original = LogEntry {
        level: 0,
        message: "no tags assigned".to_string(),
        tags: vec![],
    };
    let enc = oxicode::serde::encode_to_vec(&original, oxicode::config::standard())
        .expect("encode LogEntry empty tags failed");
    let (decoded, _): (LogEntry, usize) =
        oxicode::serde::decode_owned_from_slice(&enc, oxicode::config::standard())
            .expect("decode LogEntry empty tags failed");
    assert_eq!(original, decoded);
    assert!(
        decoded.tags.is_empty(),
        "tags must remain empty after roundtrip"
    );
}

// ---------------------------------------------------------------------------
// Test 8: LogEntry with many tags roundtrip
// ---------------------------------------------------------------------------

#[test]
fn test_serde_log_entry_many_tags_roundtrip() {
    let tags: Vec<String> = (0..50).map(|i| format!("tag-{i:03}")).collect();
    let original = LogEntry {
        level: 1,
        message: "event with many classification tags".to_string(),
        tags,
    };
    let enc = oxicode::serde::encode_to_vec(&original, oxicode::config::standard())
        .expect("encode LogEntry many tags failed");
    let (decoded, _): (LogEntry, usize) =
        oxicode::serde::decode_owned_from_slice(&enc, oxicode::config::standard())
            .expect("decode LogEntry many tags failed");
    assert_eq!(original, decoded);
    assert_eq!(decoded.tags.len(), 50, "must preserve all 50 tags");
}

// ---------------------------------------------------------------------------
// Test 9: Report struct with multiple log entries roundtrip
// ---------------------------------------------------------------------------

#[test]
fn test_serde_report_roundtrip() {
    let original = Report {
        entries: vec![
            LogEntry {
                level: 1,
                message: "first entry".to_string(),
                tags: vec!["alpha".to_string()],
            },
            LogEntry {
                level: 2,
                message: "second entry".to_string(),
                tags: vec!["beta".to_string(), "gamma".to_string()],
            },
        ],
        summary: "two entries processed".to_string(),
        count: 2,
    };
    let enc = oxicode::serde::encode_to_vec(&original, oxicode::config::standard())
        .expect("encode Report failed");
    let (decoded, _): (Report, usize) =
        oxicode::serde::decode_owned_from_slice(&enc, oxicode::config::standard())
            .expect("decode Report failed");
    assert_eq!(original, decoded);
}

// ---------------------------------------------------------------------------
// Test 10: Report with empty entries roundtrip
// ---------------------------------------------------------------------------

#[test]
fn test_serde_report_empty_entries_roundtrip() {
    let original = Report {
        entries: vec![],
        summary: "nothing to report".to_string(),
        count: 0,
    };
    let enc = oxicode::serde::encode_to_vec(&original, oxicode::config::standard())
        .expect("encode empty Report failed");
    let (decoded, _): (Report, usize) =
        oxicode::serde::decode_owned_from_slice(&enc, oxicode::config::standard())
            .expect("decode empty Report failed");
    assert_eq!(original, decoded);
    assert!(
        decoded.entries.is_empty(),
        "entries must remain empty after roundtrip"
    );
    assert_eq!(decoded.count, 0, "count must be zero after roundtrip");
}

// ---------------------------------------------------------------------------
// Test 11: Severity::Low enum variant roundtrip
// ---------------------------------------------------------------------------

#[test]
fn test_serde_severity_low_roundtrip() {
    let original = Severity::Low;
    let enc = oxicode::serde::encode_to_vec(&original, oxicode::config::standard())
        .expect("encode Severity::Low failed");
    let (decoded, _): (Severity, usize) =
        oxicode::serde::decode_owned_from_slice(&enc, oxicode::config::standard())
            .expect("decode Severity::Low failed");
    assert_eq!(original, decoded);
}

// ---------------------------------------------------------------------------
// Test 12: Severity::Medium enum variant roundtrip
// ---------------------------------------------------------------------------

#[test]
fn test_serde_severity_medium_roundtrip() {
    let original = Severity::Medium;
    let enc = oxicode::serde::encode_to_vec(&original, oxicode::config::standard())
        .expect("encode Severity::Medium failed");
    let (decoded, _): (Severity, usize) =
        oxicode::serde::decode_owned_from_slice(&enc, oxicode::config::standard())
            .expect("decode Severity::Medium failed");
    assert_eq!(original, decoded);
}

// ---------------------------------------------------------------------------
// Test 13: Severity::High enum variant roundtrip
// ---------------------------------------------------------------------------

#[test]
fn test_serde_severity_high_roundtrip() {
    let original = Severity::High;
    let enc = oxicode::serde::encode_to_vec(&original, oxicode::config::standard())
        .expect("encode Severity::High failed");
    let (decoded, _): (Severity, usize) =
        oxicode::serde::decode_owned_from_slice(&enc, oxicode::config::standard())
            .expect("decode Severity::High failed");
    assert_eq!(original, decoded);
}

// ---------------------------------------------------------------------------
// Test 14: Severity::Critical(String) tuple enum variant roundtrip
// ---------------------------------------------------------------------------

#[test]
fn test_serde_severity_critical_roundtrip() {
    let original = Severity::Critical("disk usage exceeded 95%".to_string());
    let enc = oxicode::serde::encode_to_vec(&original, oxicode::config::standard())
        .expect("encode Severity::Critical failed");
    let (decoded, _): (Severity, usize) =
        oxicode::serde::decode_owned_from_slice(&enc, oxicode::config::standard())
            .expect("decode Severity::Critical failed");
    assert_eq!(original, decoded);
}

// ---------------------------------------------------------------------------
// Test 15: All Severity variants iterated roundtrip
// ---------------------------------------------------------------------------

#[test]
fn test_serde_all_severity_variants_roundtrip() {
    let variants = vec![
        Severity::Low,
        Severity::Medium,
        Severity::High,
        Severity::Critical("memory pressure critical".to_string()),
    ];
    for variant in variants {
        let enc = oxicode::serde::encode_to_vec(&variant, oxicode::config::standard())
            .expect("encode Severity variant failed");
        let (decoded, _): (Severity, usize) =
            oxicode::serde::decode_owned_from_slice(&enc, oxicode::config::standard())
                .expect("decode Severity variant failed");
        assert_eq!(variant, decoded);
    }
}

// ---------------------------------------------------------------------------
// Test 16: Vec<LogEntry> complex nested roundtrip
// ---------------------------------------------------------------------------

#[test]
fn test_serde_vec_log_entry_roundtrip() {
    let entries: Vec<LogEntry> = (0..20)
        .map(|i| LogEntry {
            level: i % 5,
            message: format!("log message number {i}"),
            tags: vec![format!("tag-a-{i}"), format!("tag-b-{i}")],
        })
        .collect();
    let enc = oxicode::serde::encode_to_vec(&entries, oxicode::config::standard())
        .expect("encode Vec<LogEntry> failed");
    let (decoded, _): (Vec<LogEntry>, usize) =
        oxicode::serde::decode_owned_from_slice(&enc, oxicode::config::standard())
            .expect("decode Vec<LogEntry> failed");
    assert_eq!(entries, decoded);
}

// ---------------------------------------------------------------------------
// Test 17: Option<Severity> Some and None roundtrip
// ---------------------------------------------------------------------------

#[test]
fn test_serde_option_severity_roundtrip() {
    let some_val: Option<Severity> =
        Some(Severity::Critical("optional critical event".to_string()));
    let enc_some = oxicode::serde::encode_to_vec(&some_val, oxicode::config::standard())
        .expect("encode Option<Severity> Some failed");
    let (decoded_some, _): (Option<Severity>, usize) =
        oxicode::serde::decode_owned_from_slice(&enc_some, oxicode::config::standard())
            .expect("decode Option<Severity> Some failed");
    assert_eq!(some_val, decoded_some);

    let none_val: Option<Severity> = None;
    let enc_none = oxicode::serde::encode_to_vec(&none_val, oxicode::config::standard())
        .expect("encode Option<Severity> None failed");
    let (decoded_none, _): (Option<Severity>, usize) =
        oxicode::serde::decode_owned_from_slice(&enc_none, oxicode::config::standard())
            .expect("decode Option<Severity> None failed");
    assert_eq!(none_val, decoded_none);
}

// ---------------------------------------------------------------------------
// Test 18: HashMap<String, LogEntry> roundtrip
// ---------------------------------------------------------------------------

#[test]
fn test_serde_hashmap_string_log_entry_roundtrip() {
    let mut map: HashMap<String, LogEntry> = HashMap::new();
    map.insert(
        "event-001".to_string(),
        LogEntry {
            level: 1,
            message: "first event".to_string(),
            tags: vec!["net".to_string()],
        },
    );
    map.insert(
        "event-002".to_string(),
        LogEntry {
            level: 3,
            message: "second event".to_string(),
            tags: vec!["db".to_string(), "slow".to_string()],
        },
    );
    map.insert(
        "event-003".to_string(),
        LogEntry {
            level: 5,
            message: "critical event".to_string(),
            tags: vec![],
        },
    );
    let enc = oxicode::serde::encode_to_vec(&map, oxicode::config::standard())
        .expect("encode HashMap<String, LogEntry> failed");
    let (decoded, _): (HashMap<String, LogEntry>, usize) =
        oxicode::serde::decode_owned_from_slice(&enc, oxicode::config::standard())
            .expect("decode HashMap<String, LogEntry> failed");
    assert_eq!(map, decoded);
}

// ---------------------------------------------------------------------------
// Test 19: Large Report with 100 LogEntry items roundtrip
// ---------------------------------------------------------------------------

#[test]
fn test_serde_large_report_roundtrip() {
    let entries: Vec<LogEntry> = (0..100)
        .map(|i| LogEntry {
            level: i % 6,
            message: format!("batch log entry index {i:04}"),
            tags: (0..3).map(|t| format!("category-{t}-entry-{i}")).collect(),
        })
        .collect();
    let original = Report {
        count: entries.len() as u32,
        summary: format!("batch report with {} entries", entries.len()),
        entries,
    };
    let enc = oxicode::serde::encode_to_vec(&original, oxicode::config::standard())
        .expect("encode large Report failed");
    let (decoded, _): (Report, usize) =
        oxicode::serde::decode_owned_from_slice(&enc, oxicode::config::standard())
            .expect("decode large Report failed");
    assert_eq!(original.count, decoded.count);
    assert_eq!(original.summary, decoded.summary);
    assert_eq!(original.entries.len(), decoded.entries.len());
    assert_eq!(original, decoded);
}

// ---------------------------------------------------------------------------
// Test 20: HashMap<String, Vec<Severity>> complex nested roundtrip
// ---------------------------------------------------------------------------

#[test]
fn test_serde_hashmap_vec_severity_roundtrip() {
    let mut map: HashMap<String, Vec<Severity>> = HashMap::new();
    map.insert(
        "host-alpha".to_string(),
        vec![Severity::Low, Severity::Medium],
    );
    map.insert(
        "host-beta".to_string(),
        vec![Severity::High, Severity::Critical("cpu spike".to_string())],
    );
    map.insert("host-gamma".to_string(), vec![]);
    let enc = oxicode::serde::encode_to_vec(&map, oxicode::config::standard())
        .expect("encode HashMap<String, Vec<Severity>> failed");
    let (decoded, _): (HashMap<String, Vec<Severity>>, usize) =
        oxicode::serde::decode_owned_from_slice(&enc, oxicode::config::standard())
            .expect("decode HashMap<String, Vec<Severity>> failed");
    assert_eq!(map, decoded);
}

// ---------------------------------------------------------------------------
// Test 21: Config fixed_int_encoding — Report roundtrip
// ---------------------------------------------------------------------------

#[test]
fn test_serde_report_fixed_int_config_roundtrip() {
    let original = Report {
        entries: vec![LogEntry {
            level: 255,
            message: "fixed int config test".to_string(),
            tags: vec!["fixed".to_string(), "int".to_string()],
        }],
        summary: "fixed int encoding variant".to_string(),
        count: 1,
    };
    let config = oxicode::config::standard().with_fixed_int_encoding();
    let enc = oxicode::serde::encode_to_vec(&original, config)
        .expect("encode Report with fixed_int_encoding failed");
    let (decoded, _): (Report, usize) = oxicode::serde::decode_owned_from_slice(&enc, config)
        .expect("decode Report with fixed_int_encoding failed");
    assert_eq!(original, decoded);
}

// ---------------------------------------------------------------------------
// Test 22: Config big_endian — LogEntry roundtrip
// ---------------------------------------------------------------------------

#[test]
fn test_serde_log_entry_big_endian_config_roundtrip() {
    let original = LogEntry {
        level: 0xDEAD_BEEF,
        message: "big endian byte order test".to_string(),
        tags: vec!["big-endian".to_string(), "network-order".to_string()],
    };
    let config = oxicode::config::standard().with_big_endian();
    let enc = oxicode::serde::encode_to_vec(&original, config)
        .expect("encode LogEntry with big_endian config failed");
    let (decoded, _): (LogEntry, usize) = oxicode::serde::decode_owned_from_slice(&enc, config)
        .expect("decode LogEntry with big_endian config failed");
    assert_eq!(original, decoded);
}
