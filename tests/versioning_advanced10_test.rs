//! Document versioning tests for OxiCode — set 10.
//!
//! Covers 22 scenarios using DocumentStatus, DocumentV1, DocumentV2, DocumentV3
//! via encode_versioned_value / decode_versioned_value and related APIs.

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
    config, decode_from_slice, decode_versioned_value, encode_to_vec, encode_versioned_value,
    versioning::Version, Decode, Encode,
};

// ── Shared types ──────────────────────────────────────────────────────────────

#[derive(Debug, PartialEq, Encode, Decode)]
enum DocumentStatus {
    Draft,
    Review,
    Approved,
    Published,
    Archived,
}

#[derive(Debug, PartialEq, Encode, Decode)]
struct DocumentV1 {
    id: u64,
    title: String,
    content: String,
}

#[derive(Debug, PartialEq, Encode, Decode)]
struct DocumentV2 {
    id: u64,
    title: String,
    content: String,
    status: DocumentStatus,
    author_id: u64,
}

#[derive(Debug, PartialEq, Encode, Decode)]
struct DocumentV3 {
    id: u64,
    title: String,
    content: String,
    status: DocumentStatus,
    author_id: u64,
    tags: Vec<String>,
    revision: u32,
}

// ── Test 1 ────────────────────────────────────────────────────────────────────
// DocumentV1 encode_versioned_value / decode_versioned_value roundtrip
#[test]
fn test_document_v1_versioned_roundtrip() {
    let version = Version::new(1, 0, 0);
    let val = DocumentV1 {
        id: 1,
        title: String::from("First Doc"),
        content: String::from("Hello world"),
    };
    let bytes = encode_versioned_value(&val, version).expect("encode v1");
    let (decoded, ver, consumed): (DocumentV1, Version, usize) =
        decode_versioned_value(&bytes).expect("decode v1");
    assert_eq!(val, decoded);
    assert_eq!(ver, version);
    assert!(consumed > 0);
}

// ── Test 2 ────────────────────────────────────────────────────────────────────
// DocumentV2 encode_versioned_value / decode_versioned_value roundtrip
#[test]
fn test_document_v2_versioned_roundtrip() {
    let version = Version::new(2, 0, 0);
    let val = DocumentV2 {
        id: 2,
        title: String::from("Second Doc"),
        content: String::from("Some content"),
        status: DocumentStatus::Review,
        author_id: 42,
    };
    let bytes = encode_versioned_value(&val, version).expect("encode v2");
    let (decoded, ver, consumed): (DocumentV2, Version, usize) =
        decode_versioned_value(&bytes).expect("decode v2");
    assert_eq!(val, decoded);
    assert_eq!(ver, version);
    assert!(consumed > 0);
}

// ── Test 3 ────────────────────────────────────────────────────────────────────
// DocumentV3 encode_versioned_value / decode_versioned_value roundtrip
#[test]
fn test_document_v3_versioned_roundtrip() {
    let version = Version::new(3, 0, 0);
    let val = DocumentV3 {
        id: 3,
        title: String::from("Third Doc"),
        content: String::from("Advanced content"),
        status: DocumentStatus::Approved,
        author_id: 99,
        tags: vec![String::from("rust"), String::from("oxicode")],
        revision: 7,
    };
    let bytes = encode_versioned_value(&val, version).expect("encode v3");
    let (decoded, ver, consumed): (DocumentV3, Version, usize) =
        decode_versioned_value(&bytes).expect("decode v3");
    assert_eq!(val, decoded);
    assert_eq!(ver, version);
    assert!(consumed > 0);
}

// ── Test 4 ────────────────────────────────────────────────────────────────────
// Version 1, 2, 3 produce distinct version values
#[test]
fn test_v1_v2_v3_produce_distinct_versions() {
    let v1 = Version::new(1, 0, 0);
    let v2 = Version::new(2, 0, 0);
    let v3 = Version::new(3, 0, 0);
    assert_ne!(v1, v2);
    assert_ne!(v2, v3);
    assert_ne!(v1, v3);
    assert!(v1 < v2);
    assert!(v2 < v3);
}

// ── Test 5 ────────────────────────────────────────────────────────────────────
// Version is preserved in encoded bytes for DocumentV1
#[test]
fn test_version_preserved_in_encoded_bytes_v1() {
    let version = Version::new(1, 2, 3);
    let val = DocumentV1 {
        id: 10,
        title: String::from("Preserved"),
        content: String::from("check version"),
    };
    let bytes = encode_versioned_value(&val, version).expect("encode");
    let (_decoded, ver, _consumed): (DocumentV1, Version, usize) =
        decode_versioned_value(&bytes).expect("decode");
    assert_eq!(ver.major, 1);
    assert_eq!(ver.minor, 2);
    assert_eq!(ver.patch, 3);
}

// ── Test 6 ────────────────────────────────────────────────────────────────────
// DocumentStatus::Draft variant roundtrip via versioned encoding
#[test]
fn test_document_status_draft_variant() {
    let version = Version::new(2, 0, 0);
    let val = DocumentV2 {
        id: 100,
        title: String::from("Draft Doc"),
        content: String::from("..."),
        status: DocumentStatus::Draft,
        author_id: 1,
    };
    let bytes = encode_versioned_value(&val, version).expect("encode draft");
    let (decoded, _ver, _consumed): (DocumentV2, Version, usize) =
        decode_versioned_value(&bytes).expect("decode draft");
    assert_eq!(decoded.status, DocumentStatus::Draft);
}

// ── Test 7 ────────────────────────────────────────────────────────────────────
// DocumentStatus::Published variant roundtrip
#[test]
fn test_document_status_published_variant() {
    let version = Version::new(2, 0, 0);
    let val = DocumentV2 {
        id: 200,
        title: String::from("Published Doc"),
        content: String::from("Live content"),
        status: DocumentStatus::Published,
        author_id: 5,
    };
    let bytes = encode_versioned_value(&val, version).expect("encode published");
    let (decoded, _ver, _consumed): (DocumentV2, Version, usize) =
        decode_versioned_value(&bytes).expect("decode published");
    assert_eq!(decoded.status, DocumentStatus::Published);
}

// ── Test 8 ────────────────────────────────────────────────────────────────────
// DocumentStatus::Archived variant roundtrip via DocumentV3
#[test]
fn test_document_status_archived_variant_v3() {
    let version = Version::new(3, 0, 0);
    let val = DocumentV3 {
        id: 300,
        title: String::from("Archived Doc"),
        content: String::from("Old content"),
        status: DocumentStatus::Archived,
        author_id: 7,
        tags: vec![String::from("archive")],
        revision: 1,
    };
    let bytes = encode_versioned_value(&val, version).expect("encode archived");
    let (decoded, _ver, _consumed): (DocumentV3, Version, usize) =
        decode_versioned_value(&bytes).expect("decode archived");
    assert_eq!(decoded.status, DocumentStatus::Archived);
}

// ── Test 9 ────────────────────────────────────────────────────────────────────
// DocumentStatus::Approved variant roundtrip
#[test]
fn test_document_status_approved_variant() {
    let version = Version::new(2, 1, 0);
    let val = DocumentV2 {
        id: 400,
        title: String::from("Approved Doc"),
        content: String::from("Approved"),
        status: DocumentStatus::Approved,
        author_id: 3,
    };
    let bytes = encode_versioned_value(&val, version).expect("encode approved");
    let (decoded, ver, _consumed): (DocumentV2, Version, usize) =
        decode_versioned_value(&bytes).expect("decode approved");
    assert_eq!(decoded.status, DocumentStatus::Approved);
    assert_eq!(ver, Version::new(2, 1, 0));
}

// ── Test 10 ───────────────────────────────────────────────────────────────────
// Consumed bytes check: consumed equals total encoded length for DocumentV1
#[test]
fn test_consumed_bytes_equals_total_for_document_v1() {
    let version = Version::new(1, 0, 0);
    let val = DocumentV1 {
        id: 9,
        title: String::from("Consumed check"),
        content: String::from("bytes"),
    };
    let bytes = encode_versioned_value(&val, version).expect("encode consumed check");
    let (_decoded, _ver, consumed): (DocumentV1, Version, usize) =
        decode_versioned_value(&bytes).expect("decode consumed check");
    // consumed reports total bytes consumed (header + payload)
    assert_eq!(consumed, bytes.len());
}

// ── Test 11 ───────────────────────────────────────────────────────────────────
// Consumed bytes check for DocumentV3 with multiple tags
#[test]
fn test_consumed_bytes_for_document_v3_with_tags() {
    let version = Version::new(3, 0, 0);
    let val = DocumentV3 {
        id: 42,
        title: String::from("Tagged"),
        content: String::from("Multi-tag content"),
        status: DocumentStatus::Review,
        author_id: 11,
        tags: vec![
            String::from("alpha"),
            String::from("beta"),
            String::from("gamma"),
        ],
        revision: 3,
    };
    let bytes = encode_versioned_value(&val, version).expect("encode v3 tags");
    let (_decoded, _ver, consumed): (DocumentV3, Version, usize) =
        decode_versioned_value(&bytes).expect("decode v3 tags");
    // consumed reports total bytes consumed (header + payload)
    assert_eq!(consumed, bytes.len());
}

// ── Test 12 ───────────────────────────────────────────────────────────────────
// V1 and V2 produce different byte sizes (V2 has more fields)
#[test]
fn test_v1_and_v2_produce_different_byte_sizes() {
    let v1_doc = DocumentV1 {
        id: 1,
        title: String::from("Title"),
        content: String::from("Content"),
    };
    let v2_doc = DocumentV2 {
        id: 1,
        title: String::from("Title"),
        content: String::from("Content"),
        status: DocumentStatus::Draft,
        author_id: 0,
    };
    let v1_bytes = encode_to_vec(&v1_doc).expect("encode v1_doc");
    let v2_bytes = encode_to_vec(&v2_doc).expect("encode v2_doc");
    // V2 has two extra fields: status (enum) + author_id (u64)
    assert!(v2_bytes.len() > v1_bytes.len());
}

// ── Test 13 ───────────────────────────────────────────────────────────────────
// Vec of DocumentV1 versioned roundtrip
#[test]
fn test_vec_of_document_v1_versioned_roundtrip() {
    let version = Version::new(1, 0, 0);
    let docs = vec![
        DocumentV1 {
            id: 1,
            title: String::from("Doc A"),
            content: String::from("A"),
        },
        DocumentV1 {
            id: 2,
            title: String::from("Doc B"),
            content: String::from("B"),
        },
        DocumentV1 {
            id: 3,
            title: String::from("Doc C"),
            content: String::from("C"),
        },
    ];
    let bytes = encode_versioned_value(&docs, version).expect("encode vec v1");
    let (decoded, ver, consumed): (Vec<DocumentV1>, Version, usize) =
        decode_versioned_value(&bytes).expect("decode vec v1");
    assert_eq!(docs, decoded);
    assert_eq!(ver, version);
    assert!(consumed > 0);
}

// ── Test 14 ───────────────────────────────────────────────────────────────────
// Vec of DocumentV3 versioned roundtrip
#[test]
fn test_vec_of_document_v3_versioned_roundtrip() {
    let version = Version::new(3, 1, 0);
    let docs = vec![
        DocumentV3 {
            id: 10,
            title: String::from("Item 1"),
            content: String::from("Content 1"),
            status: DocumentStatus::Draft,
            author_id: 1,
            tags: vec![String::from("new")],
            revision: 0,
        },
        DocumentV3 {
            id: 20,
            title: String::from("Item 2"),
            content: String::from("Content 2"),
            status: DocumentStatus::Published,
            author_id: 2,
            tags: vec![String::from("pub"), String::from("live")],
            revision: 5,
        },
    ];
    let bytes = encode_versioned_value(&docs, version).expect("encode vec v3");
    let (decoded, ver, _consumed): (Vec<DocumentV3>, Version, usize) =
        decode_versioned_value(&bytes).expect("decode vec v3");
    assert_eq!(docs, decoded);
    assert_eq!(ver, version);
}

// ── Test 15 ───────────────────────────────────────────────────────────────────
// Big-endian config encodes DocumentV1 correctly
#[test]
fn test_big_endian_config_document_v1_roundtrip() {
    let cfg = config::standard().with_big_endian();
    let val = DocumentV1 {
        id: 0xDEADBEEF_CAFEBABE,
        title: String::from("Big Endian"),
        content: String::from("BE content"),
    };
    let bytes = oxicode::encode_to_vec_with_config(&val, cfg).expect("encode big endian v1");
    let (decoded, _consumed): (DocumentV1, usize) =
        oxicode::decode_from_slice_with_config(&bytes, cfg).expect("decode big endian v1");
    assert_eq!(val, decoded);
}

// ── Test 16 ───────────────────────────────────────────────────────────────────
// Fixed-int config encodes DocumentV2 with fixed-size integers
#[test]
fn test_fixed_int_config_document_v2_roundtrip() {
    let cfg = config::standard().with_fixed_int_encoding();
    let val = DocumentV2 {
        id: 255,
        title: String::from("Fixed Int"),
        content: String::from("fixed"),
        status: DocumentStatus::Approved,
        author_id: 1024,
    };
    let bytes = oxicode::encode_to_vec_with_config(&val, cfg).expect("encode fixed int v2");
    let (decoded, _consumed): (DocumentV2, usize) =
        oxicode::decode_from_slice_with_config(&bytes, cfg).expect("decode fixed int v2");
    assert_eq!(val, decoded);
    // id (u64=8) + author_id (u64=8) = 16 bytes of fixed-int fields
    // title + content are length-prefixed strings, so total > 16
    assert!(bytes.len() >= 16);
}

// ── Test 17 ───────────────────────────────────────────────────────────────────
// Fixed-int config vs standard config produce different byte lengths for DocumentV3
#[test]
fn test_fixed_int_vs_standard_config_different_lengths() {
    let std_cfg = config::standard();
    let fix_cfg = config::standard().with_fixed_int_encoding();
    let val = DocumentV3 {
        id: 1,
        title: String::from("Config Compare"),
        content: String::from("Compare"),
        status: DocumentStatus::Review,
        author_id: 1,
        tags: vec![String::from("cmp")],
        revision: 1,
    };
    let std_bytes = oxicode::encode_to_vec_with_config(&val, std_cfg).expect("encode std");
    let fix_bytes = oxicode::encode_to_vec_with_config(&val, fix_cfg).expect("encode fix");
    // Fixed encoding uses more bytes for small integers (no varint compression)
    assert_ne!(std_bytes, fix_bytes);
}

// ── Test 18 ───────────────────────────────────────────────────────────────────
// encode_versioned_value on DocumentV1 with minor+patch version preserved
#[test]
fn test_minor_and_patch_version_fields_preserved() {
    let version = Version::new(1, 5, 9);
    let val = DocumentV1 {
        id: 7,
        title: String::from("Minor Patch"),
        content: String::from("v1.5.9"),
    };
    let bytes = encode_versioned_value(&val, version).expect("encode 1.5.9");
    let (_decoded, ver, _consumed): (DocumentV1, Version, usize) =
        decode_versioned_value(&bytes).expect("decode 1.5.9");
    assert_eq!(ver.major, 1);
    assert_eq!(ver.minor, 5);
    assert_eq!(ver.patch, 9);
    assert_eq!(ver, version);
}

// ── Test 19 ───────────────────────────────────────────────────────────────────
// All DocumentStatus variants can be encoded and decoded individually
#[test]
fn test_all_document_status_variants_encode_decode() {
    let statuses = [
        DocumentStatus::Draft,
        DocumentStatus::Review,
        DocumentStatus::Approved,
        DocumentStatus::Published,
        DocumentStatus::Archived,
    ];
    for status in statuses {
        let bytes = encode_to_vec(&status).expect("encode status");
        let (decoded, _consumed): (DocumentStatus, usize) =
            decode_from_slice(&bytes).expect("decode status");
        assert_eq!(status, decoded);
    }
}

// ── Test 20 ───────────────────────────────────────────────────────────────────
// DocumentV3 with empty tags and revision=0 roundtrip via versioned encoding
#[test]
fn test_document_v3_empty_tags_zero_revision() {
    let version = Version::new(3, 0, 0);
    let val = DocumentV3 {
        id: 0,
        title: String::from("Empty Tags"),
        content: String::from("No tags here"),
        status: DocumentStatus::Draft,
        author_id: 0,
        tags: vec![],
        revision: 0,
    };
    let bytes = encode_versioned_value(&val, version).expect("encode empty tags");
    let (decoded, ver, consumed): (DocumentV3, Version, usize) =
        decode_versioned_value(&bytes).expect("decode empty tags");
    assert_eq!(val, decoded);
    assert_eq!(ver, version);
    assert!(consumed > 0);
    assert!(decoded.tags.is_empty());
    assert_eq!(decoded.revision, 0);
}

// ── Test 21 ───────────────────────────────────────────────────────────────────
// Version tuple() accessor returns correct (major, minor, patch)
#[test]
fn test_version_tuple_accessor_correct() {
    let version = Version::new(2, 4, 8);
    let val = DocumentV2 {
        id: 50,
        title: String::from("Tuple Test"),
        content: String::from("checking tuple"),
        status: DocumentStatus::Published,
        author_id: 77,
    };
    let bytes = encode_versioned_value(&val, version).expect("encode tuple test");
    let (_decoded, ver, _consumed): (DocumentV2, Version, usize) =
        decode_versioned_value(&bytes).expect("decode tuple test");
    assert_eq!(ver.tuple(), (2, 4, 8));
}

// ── Test 22 ───────────────────────────────────────────────────────────────────
// DocumentV1, V2, V3 all produce different total encoded sizes (versioned)
#[test]
fn test_v1_v2_v3_versioned_produce_different_total_sizes() {
    let title = String::from("Same Title");
    let content = String::from("Same content here");
    let v1 = DocumentV1 {
        id: 1,
        title: title.clone(),
        content: content.clone(),
    };
    let v2 = DocumentV2 {
        id: 1,
        title: title.clone(),
        content: content.clone(),
        status: DocumentStatus::Draft,
        author_id: 0,
    };
    let v3 = DocumentV3 {
        id: 1,
        title: title.clone(),
        content: content.clone(),
        status: DocumentStatus::Draft,
        author_id: 0,
        tags: vec![],
        revision: 0,
    };
    let b1 = encode_versioned_value(&v1, Version::new(1, 0, 0)).expect("encode v1");
    let b2 = encode_versioned_value(&v2, Version::new(2, 0, 0)).expect("encode v2");
    let b3 = encode_versioned_value(&v3, Version::new(3, 0, 0)).expect("encode v3");
    // Each schema version has more fields, so encoded sizes grow
    assert!(b1.len() < b2.len(), "v2 should be larger than v1");
    assert!(b2.len() < b3.len(), "v3 should be larger than v2");
}
