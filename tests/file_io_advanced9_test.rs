//! Advanced file I/O encoding tests for OxiCode - set 9

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
use oxicode::{Decode, Encode};
use std::env::temp_dir;

#[derive(Debug, PartialEq, Encode, Decode)]
struct Document {
    title: String,
    content: String,
    word_count: u32,
    tags: Vec<String>,
}

#[derive(Debug, PartialEq, Encode, Decode)]
enum DocumentStatus {
    Draft,
    Published { at: u64 },
    Archived(String),
    Deleted,
}

// ---------------------------------------------------------------------------
// Test 1: Document roundtrip via file
// ---------------------------------------------------------------------------
#[test]
fn test_adv9_document_roundtrip_file() {
    let doc = Document {
        title: "OxiCode Deep Dive".to_string(),
        content: "A thorough exploration of binary encoding.".to_string(),
        word_count: 7,
        tags: vec![
            "rust".to_string(),
            "binary".to_string(),
            "encoding".to_string(),
        ],
    };
    let path = temp_dir().join(format!("oxicode_adv9_{}_{}.bin", std::process::id(), 1));
    oxicode::encode_to_file(&doc, &path).expect("encode_to_file failed");
    let decoded: Document = oxicode::decode_from_file(&path).expect("decode_from_file failed");
    assert_eq!(doc, decoded);
    std::fs::remove_file(&path).ok();
}

// ---------------------------------------------------------------------------
// Test 2: DocumentStatus::Draft via file
// ---------------------------------------------------------------------------
#[test]
fn test_adv9_status_draft_file() {
    let status = DocumentStatus::Draft;
    let path = temp_dir().join(format!("oxicode_adv9_{}_{}.bin", std::process::id(), 2));
    oxicode::encode_to_file(&status, &path).expect("encode Draft failed");
    let decoded: DocumentStatus = oxicode::decode_from_file(&path).expect("decode Draft failed");
    assert_eq!(status, decoded);
    std::fs::remove_file(&path).ok();
}

// ---------------------------------------------------------------------------
// Test 3: DocumentStatus::Published via file
// ---------------------------------------------------------------------------
#[test]
fn test_adv9_status_published_file() {
    let status = DocumentStatus::Published {
        at: 1_700_000_000_u64,
    };
    let path = temp_dir().join(format!("oxicode_adv9_{}_{}.bin", std::process::id(), 3));
    oxicode::encode_to_file(&status, &path).expect("encode Published failed");
    let decoded: DocumentStatus =
        oxicode::decode_from_file(&path).expect("decode Published failed");
    assert_eq!(status, decoded);
    std::fs::remove_file(&path).ok();
}

// ---------------------------------------------------------------------------
// Test 4: DocumentStatus::Archived via file
// ---------------------------------------------------------------------------
#[test]
fn test_adv9_status_archived_file() {
    let status = DocumentStatus::Archived("cold_storage_2025".to_string());
    let path = temp_dir().join(format!("oxicode_adv9_{}_{}.bin", std::process::id(), 4));
    oxicode::encode_to_file(&status, &path).expect("encode Archived failed");
    let decoded: DocumentStatus = oxicode::decode_from_file(&path).expect("decode Archived failed");
    assert_eq!(status, decoded);
    std::fs::remove_file(&path).ok();
}

// ---------------------------------------------------------------------------
// Test 5: DocumentStatus::Deleted via file
// ---------------------------------------------------------------------------
#[test]
fn test_adv9_status_deleted_file() {
    let status = DocumentStatus::Deleted;
    let path = temp_dir().join(format!("oxicode_adv9_{}_{}.bin", std::process::id(), 5));
    oxicode::encode_to_file(&status, &path).expect("encode Deleted failed");
    let decoded: DocumentStatus = oxicode::decode_from_file(&path).expect("decode Deleted failed");
    assert_eq!(status, decoded);
    std::fs::remove_file(&path).ok();
}

// ---------------------------------------------------------------------------
// Test 6: Vec<Document> 3 items via file
// ---------------------------------------------------------------------------
#[test]
fn test_adv9_vec_document_file() {
    let docs = vec![
        Document {
            title: "First".to_string(),
            content: "Alpha content".to_string(),
            word_count: 2,
            tags: vec!["a".to_string()],
        },
        Document {
            title: "Second".to_string(),
            content: "Beta content".to_string(),
            word_count: 2,
            tags: vec!["b".to_string(), "c".to_string()],
        },
        Document {
            title: "Third".to_string(),
            content: "Gamma content".to_string(),
            word_count: 2,
            tags: vec![],
        },
    ];
    let path = temp_dir().join(format!("oxicode_adv9_{}_{}.bin", std::process::id(), 6));
    oxicode::encode_to_file(&docs, &path).expect("encode Vec<Document> failed");
    let decoded: Vec<Document> =
        oxicode::decode_from_file(&path).expect("decode Vec<Document> failed");
    assert_eq!(docs, decoded);
    std::fs::remove_file(&path).ok();
}

// ---------------------------------------------------------------------------
// Test 7: Vec<DocumentStatus> all variants via file
// ---------------------------------------------------------------------------
#[test]
fn test_adv9_vec_status_all_variants_file() {
    let statuses = vec![
        DocumentStatus::Draft,
        DocumentStatus::Published { at: 999_999 },
        DocumentStatus::Archived("archive_reason".to_string()),
        DocumentStatus::Deleted,
    ];
    let path = temp_dir().join(format!("oxicode_adv9_{}_{}.bin", std::process::id(), 7));
    oxicode::encode_to_file(&statuses, &path).expect("encode Vec<DocumentStatus> failed");
    let decoded: Vec<DocumentStatus> =
        oxicode::decode_from_file(&path).expect("decode Vec<DocumentStatus> failed");
    assert_eq!(statuses, decoded);
    std::fs::remove_file(&path).ok();
}

// ---------------------------------------------------------------------------
// Test 8: u32 via file
// ---------------------------------------------------------------------------
#[test]
fn test_adv9_u32_file() {
    let value: u32 = 0xDEAD_BEEF;
    let path = temp_dir().join(format!("oxicode_adv9_{}_{}.bin", std::process::id(), 8));
    oxicode::encode_to_file(&value, &path).expect("encode u32 failed");
    let decoded: u32 = oxicode::decode_from_file(&path).expect("decode u32 failed");
    assert_eq!(value, decoded);
    std::fs::remove_file(&path).ok();
}

// ---------------------------------------------------------------------------
// Test 9: String with unicode via file
// ---------------------------------------------------------------------------
#[test]
fn test_adv9_unicode_string_file() {
    let value = "日本語テスト: 安全なバイナリシリアライズ 🦀".to_string();
    let path = temp_dir().join(format!("oxicode_adv9_{}_{}.bin", std::process::id(), 9));
    oxicode::encode_to_file(&value, &path).expect("encode unicode string failed");
    let decoded: String = oxicode::decode_from_file(&path).expect("decode unicode string failed");
    assert_eq!(value, decoded);
    std::fs::remove_file(&path).ok();
}

// ---------------------------------------------------------------------------
// Test 10: f64 PI via file (bit-exact check)
// ---------------------------------------------------------------------------
#[test]
fn test_adv9_f64_pi_file_bit_exact() {
    let value: f64 = std::f64::consts::PI;
    let path = temp_dir().join(format!("oxicode_adv9_{}_{}.bin", std::process::id(), 10));
    oxicode::encode_to_file(&value, &path).expect("encode f64 PI failed");
    let decoded: f64 = oxicode::decode_from_file(&path).expect("decode f64 PI failed");
    assert_eq!(
        value.to_bits(),
        decoded.to_bits(),
        "f64 PI must be bit-exact"
    );
    std::fs::remove_file(&path).ok();
}

// ---------------------------------------------------------------------------
// Test 11: Option<Document> Some via file
// ---------------------------------------------------------------------------
#[test]
fn test_adv9_option_document_some_file() {
    let value: Option<Document> = Some(Document {
        title: "Optional Doc".to_string(),
        content: "This document is optional.".to_string(),
        word_count: 4,
        tags: vec!["optional".to_string()],
    });
    let path = temp_dir().join(format!("oxicode_adv9_{}_{}.bin", std::process::id(), 11));
    oxicode::encode_to_file(&value, &path).expect("encode Option<Document> Some failed");
    let decoded: Option<Document> =
        oxicode::decode_from_file(&path).expect("decode Option<Document> Some failed");
    assert_eq!(value, decoded);
    std::fs::remove_file(&path).ok();
}

// ---------------------------------------------------------------------------
// Test 12: Option<Document> None via file
// ---------------------------------------------------------------------------
#[test]
fn test_adv9_option_document_none_file() {
    let value: Option<Document> = None;
    let path = temp_dir().join(format!("oxicode_adv9_{}_{}.bin", std::process::id(), 12));
    oxicode::encode_to_file(&value, &path).expect("encode Option<Document> None failed");
    let decoded: Option<Document> =
        oxicode::decode_from_file(&path).expect("decode Option<Document> None failed");
    assert_eq!(value, decoded);
    std::fs::remove_file(&path).ok();
}

// ---------------------------------------------------------------------------
// Test 13: Empty Document (empty content, zero words, no tags) via file
// ---------------------------------------------------------------------------
#[test]
fn test_adv9_empty_document_file() {
    let doc = Document {
        title: String::new(),
        content: String::new(),
        word_count: 0,
        tags: vec![],
    };
    let path = temp_dir().join(format!("oxicode_adv9_{}_{}.bin", std::process::id(), 13));
    oxicode::encode_to_file(&doc, &path).expect("encode empty Document failed");
    let decoded: Document = oxicode::decode_from_file(&path).expect("decode empty Document failed");
    assert_eq!(doc, decoded);
    std::fs::remove_file(&path).ok();
}

// ---------------------------------------------------------------------------
// Test 14: Document with 100 tags via file
// ---------------------------------------------------------------------------
#[test]
fn test_adv9_document_100_tags_file() {
    let tags: Vec<String> = (0..100).map(|i| format!("tag_{:03}", i)).collect();
    let doc = Document {
        title: "Tagged Document".to_string(),
        content: "A document with exactly one hundred tags.".to_string(),
        word_count: 7,
        tags,
    };
    let path = temp_dir().join(format!("oxicode_adv9_{}_{}.bin", std::process::id(), 14));
    oxicode::encode_to_file(&doc, &path).expect("encode 100-tag Document failed");
    let decoded: Document =
        oxicode::decode_from_file(&path).expect("decode 100-tag Document failed");
    assert_eq!(doc.tags.len(), 100);
    assert_eq!(doc, decoded);
    std::fs::remove_file(&path).ok();
}

// ---------------------------------------------------------------------------
// Test 15: i64::MIN via file
// ---------------------------------------------------------------------------
#[test]
fn test_adv9_i64_min_file() {
    let value: i64 = i64::MIN;
    let path = temp_dir().join(format!("oxicode_adv9_{}_{}.bin", std::process::id(), 15));
    oxicode::encode_to_file(&value, &path).expect("encode i64::MIN failed");
    let decoded: i64 = oxicode::decode_from_file(&path).expect("decode i64::MIN failed");
    assert_eq!(value, decoded);
    std::fs::remove_file(&path).ok();
}

// ---------------------------------------------------------------------------
// Test 16: u128 via file
// ---------------------------------------------------------------------------
#[test]
fn test_adv9_u128_file() {
    let value: u128 = u128::MAX / 3 * 2 + 7;
    let path = temp_dir().join(format!("oxicode_adv9_{}_{}.bin", std::process::id(), 16));
    oxicode::encode_to_file(&value, &path).expect("encode u128 failed");
    let decoded: u128 = oxicode::decode_from_file(&path).expect("decode u128 failed");
    assert_eq!(value, decoded);
    std::fs::remove_file(&path).ok();
}

// ---------------------------------------------------------------------------
// Test 17: Fixed-int config with Document
// ---------------------------------------------------------------------------
#[test]
fn test_adv9_fixed_int_config_document_file() {
    let config = oxicode::config::standard().with_fixed_int_encoding();
    let doc = Document {
        title: "Fixed Int Config".to_string(),
        content: "Testing fixed integer encoding configuration.".to_string(),
        word_count: 5,
        tags: vec!["fixint".to_string(), "config".to_string()],
    };
    let path = temp_dir().join(format!("oxicode_adv9_{}_{}.bin", std::process::id(), 17));
    oxicode::encode_to_file_with_config(&doc, &path, config)
        .expect("encode with fixed-int config failed");
    let decoded: Document = oxicode::decode_from_file_with_config(&path, config)
        .expect("decode with fixed-int config failed");
    assert_eq!(doc, decoded);
    std::fs::remove_file(&path).ok();
}

// ---------------------------------------------------------------------------
// Test 18: Big-endian config with u32 (verify raw bytes)
// ---------------------------------------------------------------------------
#[test]
fn test_adv9_big_endian_config_u32_raw_bytes() {
    let config = oxicode::config::standard()
        .with_big_endian()
        .with_fixed_int_encoding();
    let value: u32 = 0x0102_0304_u32;
    let path = temp_dir().join(format!("oxicode_adv9_{}_{}.bin", std::process::id(), 18));
    oxicode::encode_to_file_with_config(&value, &path, config)
        .expect("encode big-endian u32 failed");
    let raw = std::fs::read(&path).expect("read raw bytes failed");
    // With big-endian fixed-int, u32 = 0x01020304 must appear as [0x01, 0x02, 0x03, 0x04]
    assert_eq!(
        raw,
        &[0x01, 0x02, 0x03, 0x04],
        "big-endian bytes must match"
    );
    let decoded: u32 =
        oxicode::decode_from_file_with_config(&path, config).expect("decode big-endian u32 failed");
    assert_eq!(value, decoded);
    std::fs::remove_file(&path).ok();
}

// ---------------------------------------------------------------------------
// Test 19: Sequential writes: 3 Documents, read back 3
// ---------------------------------------------------------------------------
#[test]
fn test_adv9_sequential_write_read_three_documents() {
    let base_path_str = format!("oxicode_adv9_{}_{}", std::process::id(), 19);
    let docs = [
        Document {
            title: "Doc A".to_string(),
            content: "Content A".to_string(),
            word_count: 2,
            tags: vec!["x".to_string()],
        },
        Document {
            title: "Doc B".to_string(),
            content: "Content B".to_string(),
            word_count: 2,
            tags: vec!["y".to_string()],
        },
        Document {
            title: "Doc C".to_string(),
            content: "Content C".to_string(),
            word_count: 2,
            tags: vec!["z".to_string()],
        },
    ];

    // Write each doc to a separate file, then read back
    for (i, doc) in docs.iter().enumerate() {
        let path = temp_dir().join(format!("{}_{}.bin", base_path_str, i));
        oxicode::encode_to_file(doc, &path).expect("encode sequential doc failed");
    }
    for (i, doc) in docs.iter().enumerate() {
        let path = temp_dir().join(format!("{}_{}.bin", base_path_str, i));
        let decoded: Document =
            oxicode::decode_from_file(&path).expect("decode sequential doc failed");
        assert_eq!(*doc, decoded);
        std::fs::remove_file(&path).ok();
    }
}

// ---------------------------------------------------------------------------
// Test 20: Overwrite: second write replaces first
// ---------------------------------------------------------------------------
#[test]
fn test_adv9_overwrite_second_replaces_first() {
    let path = temp_dir().join(format!("oxicode_adv9_{}_{}.bin", std::process::id(), 20));
    let first = Document {
        title: "First Version".to_string(),
        content: "This will be overwritten.".to_string(),
        word_count: 4,
        tags: vec!["old".to_string()],
    };
    let second = Document {
        title: "Second Version".to_string(),
        content: "This is the replacement.".to_string(),
        word_count: 4,
        tags: vec!["new".to_string(), "overwrite".to_string()],
    };
    oxicode::encode_to_file(&first, &path).expect("encode first version failed");
    oxicode::encode_to_file(&second, &path).expect("encode second version failed");
    let decoded: Document =
        oxicode::decode_from_file(&path).expect("decode after overwrite failed");
    assert_eq!(second, decoded, "second write must replace first");
    assert_ne!(first, decoded, "first write must not be present");
    std::fs::remove_file(&path).ok();
}

// ---------------------------------------------------------------------------
// Test 21: Non-existent path returns error
// ---------------------------------------------------------------------------
#[test]
fn test_adv9_nonexistent_path_returns_error() {
    let path = temp_dir().join(format!(
        "oxicode_adv9_{}_{}_nonexistent_xyz.bin",
        std::process::id(),
        21
    ));
    // Ensure the file definitely does not exist
    std::fs::remove_file(&path).ok();
    let result = oxicode::decode_from_file::<Document>(&path);
    assert!(
        result.is_err(),
        "decoding from non-existent path must return an error"
    );
}

// ---------------------------------------------------------------------------
// Test 22: File bytes match encode_to_vec output
// ---------------------------------------------------------------------------
#[test]
fn test_adv9_file_bytes_match_encode_to_vec() {
    let doc = Document {
        title: "Byte Verification".to_string(),
        content: "File bytes must match encode_to_vec output exactly.".to_string(),
        word_count: 8,
        tags: vec!["verify".to_string(), "bytes".to_string()],
    };
    let path = temp_dir().join(format!("oxicode_adv9_{}_{}.bin", std::process::id(), 22));
    oxicode::encode_to_file(&doc, &path).expect("encode_to_file for byte check failed");
    let file_bytes = std::fs::read(&path).expect("read file bytes failed");
    let vec_bytes = oxicode::encode_to_vec(&doc).expect("encode_to_vec for comparison failed");
    assert_eq!(
        file_bytes, vec_bytes,
        "file bytes must be identical to encode_to_vec output"
    );
    std::fs::remove_file(&path).ok();
}
