#![cfg(feature = "std")]
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
use oxicode::{config, Decode, Encode};
use std::env::temp_dir;

#[derive(Debug, PartialEq, Encode, Decode)]
struct Message {
    id: u32,
    content: String,
    attachments: Vec<Vec<u8>>,
}

#[derive(Debug, PartialEq, Encode, Decode)]
enum Severity {
    Info,
    Warning,
    Error(String),
    Critical { code: u32, details: String },
}

#[derive(Debug, PartialEq, Encode, Decode)]
struct LogEntry {
    timestamp: u64,
    severity: Severity,
    message: String,
}

fn pid() -> u32 {
    std::process::id()
}

#[test]
fn test_message_with_attachments_roundtrip() {
    let path = temp_dir().join(format!("oxicode_adv7_msg_attach_{}.bin", pid()));
    let value = Message {
        id: 1001,
        content: "Hello, OxiCode!".to_string(),
        attachments: vec![
            vec![0u8, 1, 2, 3, 4],
            vec![255u8, 254, 253],
            b"attachment data".to_vec(),
        ],
    };
    oxicode::encode_to_file(&value, &path).expect("encode Message with attachments");
    let decoded: Message =
        oxicode::decode_from_file(&path).expect("decode Message with attachments");
    assert_eq!(value, decoded);
    std::fs::remove_file(&path).ok();
}

#[test]
fn test_severity_info_to_file() {
    let path = temp_dir().join(format!("oxicode_adv7_sev_info_{}.bin", pid()));
    let value = Severity::Info;
    oxicode::encode_to_file(&value, &path).expect("encode Severity::Info");
    let decoded: Severity = oxicode::decode_from_file(&path).expect("decode Severity::Info");
    assert_eq!(value, decoded);
    std::fs::remove_file(&path).ok();
}

#[test]
fn test_severity_warning_to_file() {
    let path = temp_dir().join(format!("oxicode_adv7_sev_warn_{}.bin", pid()));
    let value = Severity::Warning;
    oxicode::encode_to_file(&value, &path).expect("encode Severity::Warning");
    let decoded: Severity = oxicode::decode_from_file(&path).expect("decode Severity::Warning");
    assert_eq!(value, decoded);
    std::fs::remove_file(&path).ok();
}

#[test]
fn test_severity_error_to_file() {
    let path = temp_dir().join(format!("oxicode_adv7_sev_err_{}.bin", pid()));
    let value = Severity::Error("disk full".to_string());
    oxicode::encode_to_file(&value, &path).expect("encode Severity::Error");
    let decoded: Severity = oxicode::decode_from_file(&path).expect("decode Severity::Error");
    assert_eq!(value, decoded);
    std::fs::remove_file(&path).ok();
}

#[test]
fn test_severity_critical_to_file() {
    let path = temp_dir().join(format!("oxicode_adv7_sev_crit_{}.bin", pid()));
    let value = Severity::Critical {
        code: 500,
        details: "kernel panic at 0xDEADBEEF".to_string(),
    };
    oxicode::encode_to_file(&value, &path).expect("encode Severity::Critical");
    let decoded: Severity = oxicode::decode_from_file(&path).expect("decode Severity::Critical");
    assert_eq!(value, decoded);
    std::fs::remove_file(&path).ok();
}

#[test]
fn test_log_entry_roundtrip() {
    let path = temp_dir().join(format!("oxicode_adv7_logentry_{}.bin", pid()));
    let value = LogEntry {
        timestamp: 1_700_000_000u64,
        severity: Severity::Warning,
        message: "low disk space".to_string(),
    };
    oxicode::encode_to_file(&value, &path).expect("encode LogEntry");
    let decoded: LogEntry = oxicode::decode_from_file(&path).expect("decode LogEntry");
    assert_eq!(value, decoded);
    std::fs::remove_file(&path).ok();
}

#[test]
fn test_vec_log_entries_10_roundtrip() {
    let path = temp_dir().join(format!("oxicode_adv7_vec_log10_{}.bin", pid()));
    let entries: Vec<LogEntry> = (0..10)
        .map(|i| LogEntry {
            timestamp: 1_000_000u64 + i,
            severity: if i % 2 == 0 {
                Severity::Info
            } else {
                Severity::Error(format!("error #{}", i))
            },
            message: format!("log message #{}", i),
        })
        .collect();
    oxicode::encode_to_file(&entries, &path).expect("encode Vec<LogEntry>");
    let decoded: Vec<LogEntry> = oxicode::decode_from_file(&path).expect("decode Vec<LogEntry>");
    assert_eq!(entries, decoded);
    std::fs::remove_file(&path).ok();
}

#[test]
fn test_u32_max_to_file() {
    let path = temp_dir().join(format!("oxicode_adv7_u32max_{}.bin", pid()));
    let value: u32 = u32::MAX;
    oxicode::encode_to_file(&value, &path).expect("encode u32::MAX");
    let decoded: u32 = oxicode::decode_from_file(&path).expect("decode u32::MAX");
    assert_eq!(value, decoded);
    std::fs::remove_file(&path).ok();
}

#[test]
fn test_string_with_unicode_to_file() {
    let path = temp_dir().join(format!("oxicode_adv7_unicode_str_{}.bin", pid()));
    let value = "Hello \u{4e16}\u{754c} \u{1F600} Caf\u{e9} \u{0391}\u{03b2}\u{03b3}".to_string();
    oxicode::encode_to_file(&value, &path).expect("encode unicode String");
    let decoded: String = oxicode::decode_from_file(&path).expect("decode unicode String");
    assert_eq!(value, decoded);
    std::fs::remove_file(&path).ok();
}

#[test]
fn test_vec_u8_50000_bytes_roundtrip() {
    let path = temp_dir().join(format!("oxicode_adv7_vecu8_50k_{}.bin", pid()));
    let value: Vec<u8> = (0u8..=255).cycle().take(50_000).collect();
    oxicode::encode_to_file(&value, &path).expect("encode Vec<u8> 50000 bytes");
    let decoded: Vec<u8> = oxicode::decode_from_file(&path).expect("decode Vec<u8> 50000 bytes");
    assert_eq!(value, decoded);
    std::fs::remove_file(&path).ok();
}

#[test]
fn test_option_log_entry_some_to_file() {
    let path = temp_dir().join(format!("oxicode_adv7_opt_log_some_{}.bin", pid()));
    let value: Option<LogEntry> = Some(LogEntry {
        timestamp: 9_999_999u64,
        severity: Severity::Critical {
            code: 42,
            details: "critical situation".to_string(),
        },
        message: "system overload".to_string(),
    });
    oxicode::encode_to_file(&value, &path).expect("encode Option<LogEntry> Some");
    let decoded: Option<LogEntry> =
        oxicode::decode_from_file(&path).expect("decode Option<LogEntry> Some");
    assert_eq!(value, decoded);
    std::fs::remove_file(&path).ok();
}

#[test]
fn test_option_message_none_to_file() {
    let path = temp_dir().join(format!("oxicode_adv7_opt_msg_none_{}.bin", pid()));
    let value: Option<Message> = None;
    oxicode::encode_to_file(&value, &path).expect("encode Option<Message> None");
    let decoded: Option<Message> =
        oxicode::decode_from_file(&path).expect("decode Option<Message> None");
    assert_eq!(value, decoded);
    std::fs::remove_file(&path).ok();
}

#[test]
fn test_fixed_int_config_with_log_entry() {
    let path = temp_dir().join(format!("oxicode_adv7_fixedint_log_{}.bin", pid()));
    let cfg = config::standard().with_fixed_int_encoding();
    let value = LogEntry {
        timestamp: 1_234_567_890u64,
        severity: Severity::Error("timeout".to_string()),
        message: "request timed out after 30s".to_string(),
    };
    oxicode::encode_to_file_with_config(&value, &path, cfg)
        .expect("encode LogEntry with fixed int config");
    let decoded: LogEntry = oxicode::decode_from_file_with_config(&path, cfg)
        .expect("decode LogEntry with fixed int config");
    assert_eq!(value, decoded);
    std::fs::remove_file(&path).ok();
}

#[test]
fn test_big_endian_config_with_u32() {
    let path = temp_dir().join(format!("oxicode_adv7_bigendian_u32_{}.bin", pid()));
    let cfg = config::standard()
        .with_big_endian()
        .with_fixed_int_encoding();
    let value: u32 = 0xDEAD_BEEF;
    oxicode::encode_to_file_with_config(&value, &path, cfg)
        .expect("encode u32 with big endian config");
    let decoded: u32 = oxicode::decode_from_file_with_config(&path, cfg)
        .expect("decode u32 with big endian config");
    assert_eq!(value, decoded);
    // Verify big-endian byte order in file
    let raw = std::fs::read(&path).expect("read raw bytes");
    assert_eq!(raw, vec![0xDE, 0xAD, 0xBE, 0xEF]);
    std::fs::remove_file(&path).ok();
}

#[test]
fn test_sequential_write_5_then_read_all_5() {
    let paths: Vec<std::path::PathBuf> = (0..5)
        .map(|i| temp_dir().join(format!("oxicode_adv7_seq5_{}_{}.bin", pid(), i)))
        .collect();

    let entries: Vec<LogEntry> = (0..5)
        .map(|i| LogEntry {
            timestamp: i as u64 * 100,
            severity: Severity::Info,
            message: format!("sequential entry {}", i),
        })
        .collect();

    for (entry, path) in entries.iter().zip(paths.iter()) {
        oxicode::encode_to_file(entry, path).expect("sequential encode");
    }

    let decoded_entries: Vec<LogEntry> = paths
        .iter()
        .map(|p| oxicode::decode_from_file::<LogEntry>(p).expect("sequential decode"))
        .collect();

    assert_eq!(entries, decoded_entries);

    for path in &paths {
        std::fs::remove_file(path).ok();
    }
}

#[test]
fn test_file_size_matches_encode_to_vec_length() {
    let path = temp_dir().join(format!("oxicode_adv7_file_size_{}.bin", pid()));
    let value = LogEntry {
        timestamp: 8_888_888u64,
        severity: Severity::Warning,
        message: "disk usage at 85%".to_string(),
    };
    oxicode::encode_to_file(&value, &path).expect("encode for file size check");
    let file_metadata = std::fs::metadata(&path).expect("read metadata");
    let vec_bytes = oxicode::encode_to_vec(&value).expect("encode to vec for size check");
    assert_eq!(file_metadata.len() as usize, vec_bytes.len());
    std::fs::remove_file(&path).ok();
}

#[test]
fn test_write_then_overwrite_second_value_wins() {
    let path = temp_dir().join(format!("oxicode_adv7_overwrite_{}.bin", pid()));
    let first = LogEntry {
        timestamp: 111u64,
        severity: Severity::Info,
        message: "first write".to_string(),
    };
    let second = LogEntry {
        timestamp: 999u64,
        severity: Severity::Error("overwritten".to_string()),
        message: "second write wins".to_string(),
    };
    oxicode::encode_to_file(&first, &path).expect("first encode");
    oxicode::encode_to_file(&second, &path).expect("second (overwrite) encode");
    let decoded: LogEntry = oxicode::decode_from_file(&path).expect("decode after overwrite");
    assert_eq!(second, decoded);
    assert_ne!(first, decoded);
    std::fs::remove_file(&path).ok();
}

#[test]
fn test_decode_from_nonexistent_file_returns_error() {
    let path = temp_dir().join(format!(
        "oxicode_adv7_nonexistent_{}_{}.bin",
        pid(),
        u64::MAX
    ));
    let result = oxicode::decode_from_file::<LogEntry>(&path);
    assert!(
        result.is_err(),
        "expected error decoding from nonexistent file"
    );
}

#[test]
fn test_vec_severity_all_variants_to_file() {
    let path = temp_dir().join(format!("oxicode_adv7_all_sev_{}.bin", pid()));
    let value: Vec<Severity> = vec![
        Severity::Info,
        Severity::Warning,
        Severity::Error("something went wrong".to_string()),
        Severity::Critical {
            code: 1,
            details: "total failure".to_string(),
        },
    ];
    oxicode::encode_to_file(&value, &path).expect("encode Vec<Severity> all variants");
    let decoded: Vec<Severity> =
        oxicode::decode_from_file(&path).expect("decode Vec<Severity> all variants");
    assert_eq!(value, decoded);
    std::fs::remove_file(&path).ok();
}

#[test]
fn test_message_with_empty_attachments_to_file() {
    let path = temp_dir().join(format!("oxicode_adv7_msg_empty_attach_{}.bin", pid()));
    let value = Message {
        id: 0,
        content: "no attachments here".to_string(),
        attachments: vec![],
    };
    oxicode::encode_to_file(&value, &path).expect("encode Message with empty attachments");
    let decoded: Message =
        oxicode::decode_from_file(&path).expect("decode Message with empty attachments");
    assert_eq!(value, decoded);
    std::fs::remove_file(&path).ok();
}

#[test]
fn test_message_with_10_attachments_each_256_bytes() {
    let path = temp_dir().join(format!("oxicode_adv7_msg_10attach_{}.bin", pid()));
    let attachment: Vec<u8> = (0u8..=255u8).collect();
    let value = Message {
        id: 42,
        content: "ten attachments".to_string(),
        attachments: std::iter::repeat(attachment).take(10).collect(),
    };
    assert_eq!(value.attachments.len(), 10);
    assert!(value.attachments.iter().all(|a| a.len() == 256));
    oxicode::encode_to_file(&value, &path)
        .expect("encode Message with 10 attachments each 256 bytes");
    let decoded: Message = oxicode::decode_from_file(&path)
        .expect("decode Message with 10 attachments each 256 bytes");
    assert_eq!(value, decoded);
    std::fs::remove_file(&path).ok();
}

#[test]
fn test_large_log_entry_1000_char_message() {
    let path = temp_dir().join(format!("oxicode_adv7_large_log_{}.bin", pid()));
    let large_message = "X".repeat(1000);
    let value = LogEntry {
        timestamp: u64::MAX / 2,
        severity: Severity::Critical {
            code: 0xFFFF_FFFF,
            details: "Y".repeat(500),
        },
        message: large_message,
    };
    assert_eq!(value.message.len(), 1000);
    oxicode::encode_to_file(&value, &path).expect("encode large LogEntry");
    let decoded: LogEntry = oxicode::decode_from_file(&path).expect("decode large LogEntry");
    assert_eq!(value, decoded);
    std::fs::remove_file(&path).ok();
}
