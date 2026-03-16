//! Advanced tests (set 2) for `#[oxicode(rename_all = "...")]`.
//!
//! The OxiCode binary format is positional — `rename_all` is accepted, validated,
//! and stored for diagnostic/display purposes, but it does NOT alter the on-wire
//! byte layout.  Every test here verifies:
//!   1. The attribute compiles without error.
//!   2. Encode → Decode roundtrip reproduces the original value exactly.
//!   3. More complex interactions (nested types, multiple attrs, generics, enums
//!      with payloads, cross-convention byte-identity, etc.) work correctly.

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
use oxicode::{config, decode_from_slice, encode_to_vec, Decode, Encode};

// ---------------------------------------------------------------------------
// 1. rename_all = "UPPERCASE" on a struct — all field names uppercased (no-op wire)
// ---------------------------------------------------------------------------

#[derive(Debug, PartialEq, Encode, Decode)]
#[oxicode(rename_all = "UPPERCASE")]
struct UppercaseFields {
    record_id: u32,
    record_name: String,
    is_valid: bool,
}

#[test]
fn test_rename_all_uppercase_struct_roundtrip() {
    let original = UppercaseFields {
        record_id: 42,
        record_name: "test_record".to_string(),
        is_valid: true,
    };
    let encoded = encode_to_vec(&original).expect("encode UPPERCASE struct");
    let (decoded, bytes_read): (UppercaseFields, usize) =
        decode_from_slice(&encoded).expect("decode UPPERCASE struct");
    assert_eq!(decoded, original);
    assert_eq!(bytes_read, encoded.len());
}

// ---------------------------------------------------------------------------
// 2. rename_all = "UPPERCASE" — wire bytes identical to same struct without attr
// ---------------------------------------------------------------------------

#[derive(Debug, PartialEq, Encode, Decode)]
struct UppercaseBaseline {
    record_id: u32,
    record_name: String,
    is_valid: bool,
}

#[test]
fn test_rename_all_uppercase_wire_identity() {
    let baseline = UppercaseBaseline {
        record_id: 42,
        record_name: "test_record".to_string(),
        is_valid: true,
    };
    let renamed = UppercaseFields {
        record_id: 42,
        record_name: "test_record".to_string(),
        is_valid: true,
    };
    let baseline_bytes = encode_to_vec(&baseline).expect("encode baseline");
    let renamed_bytes = encode_to_vec(&renamed).expect("encode UPPERCASE renamed");
    assert_eq!(
        baseline_bytes, renamed_bytes,
        "UPPERCASE rename_all must not alter binary wire bytes"
    );
}

// ---------------------------------------------------------------------------
// 3. rename_all = "camelCase" on nested struct — outer and inner both annotated
// ---------------------------------------------------------------------------

#[derive(Debug, PartialEq, Encode, Decode)]
#[oxicode(rename_all = "camelCase")]
struct InnerCamel {
    inner_value: u32,
    inner_label: String,
}

#[derive(Debug, PartialEq, Encode, Decode)]
#[oxicode(rename_all = "camelCase")]
struct OuterCamel {
    outer_id: u64,
    nested_data: InnerCamel,
    outer_count: u16,
}

#[test]
fn test_rename_all_camel_case_nested_struct_roundtrip() {
    let original = OuterCamel {
        outer_id: 100_000,
        nested_data: InnerCamel {
            inner_value: 7,
            inner_label: "nested".to_string(),
        },
        outer_count: 3,
    };
    let encoded = encode_to_vec(&original).expect("encode nested camelCase");
    let (decoded, bytes_read): (OuterCamel, usize) =
        decode_from_slice(&encoded).expect("decode nested camelCase");
    assert_eq!(decoded, original);
    assert_eq!(bytes_read, encoded.len());
}

// ---------------------------------------------------------------------------
// 4. rename_all = "kebab-case" + multiple Vec fields
// ---------------------------------------------------------------------------

#[derive(Debug, PartialEq, Encode, Decode)]
#[oxicode(rename_all = "kebab-case")]
struct KebabMultiVec {
    raw_bytes: Vec<u8>,
    tag_ids: Vec<u32>,
    label_list: Vec<String>,
}

#[test]
fn test_rename_all_kebab_case_multi_vec_roundtrip() {
    let original = KebabMultiVec {
        raw_bytes: vec![1, 2, 3, 255],
        tag_ids: vec![10, 20, 30],
        label_list: vec!["alpha".to_string(), "beta".to_string()],
    };
    let encoded = encode_to_vec(&original).expect("encode kebab-case multi-Vec");
    let (decoded, bytes_read): (KebabMultiVec, usize) =
        decode_from_slice(&encoded).expect("decode kebab-case multi-Vec");
    assert_eq!(decoded, original);
    assert_eq!(bytes_read, encoded.len());
}

// ---------------------------------------------------------------------------
// 5. rename_all = "snake_case" on enum with all variant types
// ---------------------------------------------------------------------------

#[derive(Debug, PartialEq, Encode, Decode)]
#[oxicode(rename_all = "snake_case")]
enum SnakeAllVariants {
    UnitVariant,
    TupleVariant(u32, String),
    StructVariant { key_name: String, key_value: u64 },
    NestedTuple(Vec<u8>, Option<bool>),
}

#[test]
fn test_rename_all_snake_case_all_variant_types_roundtrip() {
    let cases = vec![
        SnakeAllVariants::UnitVariant,
        SnakeAllVariants::TupleVariant(99, "hello".to_string()),
        SnakeAllVariants::StructVariant {
            key_name: "config_key".to_string(),
            key_value: u64::MAX / 2,
        },
        SnakeAllVariants::NestedTuple(vec![0xAB, 0xCD], Some(false)),
    ];
    for case in cases {
        let encoded = encode_to_vec(&case).expect("encode snake_case all variants");
        let (decoded, bytes_read): (SnakeAllVariants, usize) =
            decode_from_slice(&encoded).expect("decode snake_case all variants");
        assert_eq!(decoded, case);
        assert_eq!(bytes_read, encoded.len());
    }
}

// ---------------------------------------------------------------------------
// 6. rename_all = "PascalCase" combined with individual field rename
// ---------------------------------------------------------------------------

#[derive(Debug, PartialEq, Encode, Decode)]
#[oxicode(rename_all = "PascalCase")]
struct PascalWithFieldRename {
    #[oxicode(rename = "Uid")]
    user_id: u64,
    display_name: String,
    #[oxicode(rename = "CreatedTs")]
    created_at_timestamp: u64,
}

#[test]
fn test_rename_all_pascal_combined_field_rename_roundtrip() {
    let original = PascalWithFieldRename {
        user_id: 1_000_000,
        display_name: "Pascal User".to_string(),
        created_at_timestamp: 1_700_000_000,
    };
    let encoded = encode_to_vec(&original).expect("encode PascalCase + field rename");
    let (decoded, bytes_read): (PascalWithFieldRename, usize) =
        decode_from_slice(&encoded).expect("decode PascalCase + field rename");
    assert_eq!(decoded, original);
    assert_eq!(bytes_read, encoded.len());
}

// ---------------------------------------------------------------------------
// 7. rename_all = "SCREAMING_SNAKE_CASE" on a generic struct with Vec<T>
// ---------------------------------------------------------------------------

#[derive(Debug, PartialEq, Encode, Decode)]
#[oxicode(rename_all = "SCREAMING_SNAKE_CASE")]
struct ScreamingGenericVec<T> {
    element_list: Vec<T>,
    element_count: u32,
    max_capacity: u32,
}

#[test]
fn test_rename_all_screaming_snake_generic_vec_roundtrip() {
    let original = ScreamingGenericVec::<i32> {
        element_list: vec![-1, 0, 1, 2, i32::MAX],
        element_count: 5,
        max_capacity: 100,
    };
    let encoded = encode_to_vec(&original).expect("encode SCREAMING_SNAKE generic Vec");
    let (decoded, bytes_read): (ScreamingGenericVec<i32>, usize) =
        decode_from_slice(&encoded).expect("decode SCREAMING_SNAKE generic Vec");
    assert_eq!(decoded, original);
    assert_eq!(bytes_read, encoded.len());
}

// ---------------------------------------------------------------------------
// 8. rename_all = "lowercase" on a struct with boundary numeric values
// ---------------------------------------------------------------------------

#[derive(Debug, PartialEq, Encode, Decode)]
#[oxicode(rename_all = "lowercase")]
struct LowercaseBoundaryNums {
    min_u8_val: u8,
    max_u8_val: u8,
    min_i64_val: i64,
    max_i64_val: i64,
    zero_float: f64,
}

#[test]
fn test_rename_all_lowercase_boundary_numerics_roundtrip() {
    let original = LowercaseBoundaryNums {
        min_u8_val: u8::MIN,
        max_u8_val: u8::MAX,
        min_i64_val: i64::MIN,
        max_i64_val: i64::MAX,
        zero_float: 0.0_f64,
    };
    let encoded = encode_to_vec(&original).expect("encode lowercase boundary nums");
    let (decoded, bytes_read): (LowercaseBoundaryNums, usize) =
        decode_from_slice(&encoded).expect("decode lowercase boundary nums");
    assert_eq!(decoded, original);
    assert_eq!(bytes_read, encoded.len());
}

// ---------------------------------------------------------------------------
// 9. rename_all = "camelCase" + tag_type = "u16" on enum
// ---------------------------------------------------------------------------

#[derive(Debug, PartialEq, Encode, Decode)]
#[oxicode(rename_all = "camelCase", tag_type = "u16")]
enum CamelU16TagEnum {
    RequestStart { request_id: u32 },
    RequestEnd { request_id: u32, elapsed_ms: u64 },
    RequestError(String),
    RequestCancelled,
}

#[test]
fn test_rename_all_camel_case_u16_tag_roundtrip() {
    let cases = vec![
        CamelU16TagEnum::RequestStart { request_id: 1 },
        CamelU16TagEnum::RequestEnd {
            request_id: 1,
            elapsed_ms: 250,
        },
        CamelU16TagEnum::RequestError("timeout".to_string()),
        CamelU16TagEnum::RequestCancelled,
    ];
    for case in cases {
        let encoded = encode_to_vec(&case).expect("encode camelCase u16-tag enum");
        let (decoded, bytes_read): (CamelU16TagEnum, usize) =
            decode_from_slice(&encoded).expect("decode camelCase u16-tag enum");
        assert_eq!(decoded, case);
        assert_eq!(bytes_read, encoded.len());
    }
}

// ---------------------------------------------------------------------------
// 10. rename_all = "PascalCase" — tag_type = "u64" on large enum
// ---------------------------------------------------------------------------

#[derive(Debug, PartialEq, Encode, Decode)]
#[oxicode(rename_all = "PascalCase", tag_type = "u64")]
enum PascalU64TagEnum {
    AuthSuccess { session_token: String },
    AuthFailure { error_code: u32, error_msg: String },
    AuthExpired,
}

#[test]
fn test_rename_all_pascal_case_u64_tag_roundtrip() {
    let cases = vec![
        PascalU64TagEnum::AuthSuccess {
            session_token: "tok-abc123".to_string(),
        },
        PascalU64TagEnum::AuthFailure {
            error_code: 401,
            error_msg: "invalid credentials".to_string(),
        },
        PascalU64TagEnum::AuthExpired,
    ];
    for case in cases {
        let encoded = encode_to_vec(&case).expect("encode PascalCase u64-tag enum");
        let (decoded, bytes_read): (PascalU64TagEnum, usize) =
            decode_from_slice(&encoded).expect("decode PascalCase u64-tag enum");
        assert_eq!(decoded, case);
        assert_eq!(bytes_read, encoded.len());
    }
}

// ---------------------------------------------------------------------------
// 11. rename_all = "kebab-case" on struct containing nested enum
// ---------------------------------------------------------------------------

#[derive(Debug, PartialEq, Encode, Decode)]
#[oxicode(rename_all = "snake_case")]
enum StatusCode {
    Success,
    Pending,
    Failed,
}

#[derive(Debug, PartialEq, Encode, Decode)]
#[oxicode(rename_all = "kebab-case")]
struct KebabWithNestedEnum {
    request_id: u64,
    status_code: StatusCode,
    response_body: Option<String>,
}

#[test]
fn test_rename_all_kebab_case_nested_enum_roundtrip() {
    let cases = vec![
        KebabWithNestedEnum {
            request_id: 1,
            status_code: StatusCode::Success,
            response_body: Some("OK".to_string()),
        },
        KebabWithNestedEnum {
            request_id: 2,
            status_code: StatusCode::Pending,
            response_body: None,
        },
        KebabWithNestedEnum {
            request_id: 3,
            status_code: StatusCode::Failed,
            response_body: Some("Internal error".to_string()),
        },
    ];
    for case in cases {
        let encoded = encode_to_vec(&case).expect("encode kebab-case with nested enum");
        let (decoded, bytes_read): (KebabWithNestedEnum, usize) =
            decode_from_slice(&encoded).expect("decode kebab-case with nested enum");
        assert_eq!(decoded, case);
        assert_eq!(bytes_read, encoded.len());
    }
}

// ---------------------------------------------------------------------------
// 12. rename_all = "UPPERCASE" on enum with explicit variant discriminants
// ---------------------------------------------------------------------------

#[derive(Debug, PartialEq, Encode, Decode)]
#[oxicode(rename_all = "UPPERCASE", tag_type = "u8")]
enum UppercaseExplicitVariants {
    #[oxicode(variant = 10)]
    AlphaEvent,
    #[oxicode(variant = 20)]
    BetaEvent(u32),
    #[oxicode(variant = 30)]
    GammaEvent { payload_size: u64 },
}

#[test]
fn test_rename_all_uppercase_explicit_variant_tags_roundtrip() {
    let cases = vec![
        UppercaseExplicitVariants::AlphaEvent,
        UppercaseExplicitVariants::BetaEvent(9_999),
        UppercaseExplicitVariants::GammaEvent { payload_size: 1024 },
    ];
    for case in cases {
        let encoded = encode_to_vec(&case).expect("encode UPPERCASE explicit variants");
        let (decoded, bytes_read): (UppercaseExplicitVariants, usize) =
            decode_from_slice(&encoded).expect("decode UPPERCASE explicit variants");
        assert_eq!(decoded, case);
        assert_eq!(bytes_read, encoded.len());
    }
}

// ---------------------------------------------------------------------------
// 13. rename_all = "camelCase" on two-layer generic struct
// ---------------------------------------------------------------------------

#[derive(Debug, PartialEq, Encode, Decode)]
#[oxicode(rename_all = "camelCase")]
struct CamelPage<T> {
    page_index: u32,
    page_size: u32,
    total_records: u64,
    page_items: Vec<T>,
}

#[test]
fn test_rename_all_camel_case_generic_page_roundtrip() {
    let original = CamelPage::<String> {
        page_index: 2,
        page_size: 10,
        total_records: 47,
        page_items: vec!["item1".to_string(), "item2".to_string()],
    };
    let encoded = encode_to_vec(&original).expect("encode camelCase generic page");
    let (decoded, bytes_read): (CamelPage<String>, usize) =
        decode_from_slice(&encoded).expect("decode camelCase generic page");
    assert_eq!(decoded, original);
    assert_eq!(bytes_read, encoded.len());
}

// ---------------------------------------------------------------------------
// 14. rename_all = "snake_case" + legacy config — wire format unaffected
// ---------------------------------------------------------------------------

#[derive(Debug, PartialEq, Encode, Decode)]
#[oxicode(rename_all = "snake_case")]
struct SnakeLegacy {
    record_value: u32,
    record_label: String,
}

#[test]
fn test_rename_all_snake_case_legacy_config_roundtrip() {
    let original = SnakeLegacy {
        record_value: 255,
        record_label: "legacy_label".to_string(),
    };
    let cfg = config::legacy();
    let encoded =
        oxicode::encode_to_vec_with_config(&original, cfg).expect("encode snake_case legacy");
    let (decoded, bytes_read): (SnakeLegacy, usize) =
        oxicode::decode_from_slice_with_config(&encoded, cfg).expect("decode snake_case legacy");
    assert_eq!(decoded, original);
    assert_eq!(bytes_read, encoded.len());
}

// ---------------------------------------------------------------------------
// 15. rename_all = "SCREAMING_SNAKE_CASE" + skip + default_value
// ---------------------------------------------------------------------------

#[derive(Debug, PartialEq, Encode, Decode)]
#[oxicode(rename_all = "SCREAMING_SNAKE_CASE")]
struct ScreamingWithSkip {
    node_id: u32,
    node_name: String,
    #[oxicode(skip, default_value = "0u64")]
    cache_timestamp: u64,
    node_score: i32,
}

#[test]
fn test_rename_all_screaming_snake_with_skip_default_roundtrip() {
    let original = ScreamingWithSkip {
        node_id: 7,
        node_name: "primary".to_string(),
        cache_timestamp: 999_999, // not encoded
        node_score: -10,
    };
    let encoded = encode_to_vec(&original).expect("encode SCREAMING_SNAKE + skip");
    let (decoded, bytes_read): (ScreamingWithSkip, usize) =
        decode_from_slice(&encoded).expect("decode SCREAMING_SNAKE + skip");
    assert_eq!(decoded.node_id, original.node_id);
    assert_eq!(decoded.node_name, original.node_name);
    assert_eq!(
        decoded.cache_timestamp, 0,
        "skipped field must restore to default_value = 0"
    );
    assert_eq!(decoded.node_score, original.node_score);
    assert_eq!(bytes_read, encoded.len());
}

// ---------------------------------------------------------------------------
// 16. rename_all = "camelCase" + bound = "" for PhantomData struct
// ---------------------------------------------------------------------------

use std::marker::PhantomData;

#[derive(Debug, PartialEq, Encode, Decode)]
#[oxicode(rename_all = "camelCase", bound = "")]
struct CamelPhantom<T> {
    payload_size: u32,
    #[oxicode(skip)]
    phantom_marker: PhantomData<T>,
}

#[test]
fn test_rename_all_camel_case_with_bound_phantom_roundtrip() {
    let original: CamelPhantom<Vec<u8>> = CamelPhantom {
        payload_size: 512,
        phantom_marker: PhantomData,
    };
    let encoded = encode_to_vec(&original).expect("encode camelCase + bound phantom");
    let (decoded, bytes_read): (CamelPhantom<Vec<u8>>, usize) =
        decode_from_slice(&encoded).expect("decode camelCase + bound phantom");
    assert_eq!(decoded, original);
    assert_eq!(bytes_read, encoded.len());
}

// ---------------------------------------------------------------------------
// 17. rename_all does not affect cross-decode: bytes from one convention
//     decode correctly under a different convention on same field layout
// ---------------------------------------------------------------------------

#[derive(Debug, PartialEq, Encode, Decode)]
#[oxicode(rename_all = "kebab-case")]
struct SourceConvention {
    field_alpha: u32,
    field_beta: u64,
}

#[derive(Debug, PartialEq, Encode, Decode)]
#[oxicode(rename_all = "SCREAMING_SNAKE_CASE")]
struct TargetConvention {
    field_alpha: u32,
    field_beta: u64,
}

#[test]
fn test_rename_all_cross_convention_binary_compatibility() {
    let source = SourceConvention {
        field_alpha: 123,
        field_beta: 456_789,
    };
    let encoded = encode_to_vec(&source).expect("encode source convention");
    // Binary layout is the same — should decode into a differently-named type
    let (decoded, bytes_read): (TargetConvention, usize) =
        decode_from_slice(&encoded).expect("cross-decode target convention");
    assert_eq!(decoded.field_alpha, source.field_alpha);
    assert_eq!(decoded.field_beta, source.field_beta);
    assert_eq!(bytes_read, encoded.len());
}

// ---------------------------------------------------------------------------
// 18. rename_all = "lowercase" on enum — all variant types, empty vec payload
// ---------------------------------------------------------------------------

#[derive(Debug, PartialEq, Encode, Decode)]
#[oxicode(rename_all = "lowercase")]
enum LowercaseEventEnum {
    TaskStarted { task_id: u32 },
    TaskFinished { task_id: u32, duration_ms: u64 },
    TaskPayload(Vec<u8>),
    TaskAborted,
}

#[test]
fn test_rename_all_lowercase_enum_empty_payload_roundtrip() {
    let empty_payload = LowercaseEventEnum::TaskPayload(vec![]);
    let encoded = encode_to_vec(&empty_payload).expect("encode lowercase empty payload");
    let (decoded, bytes_read): (LowercaseEventEnum, usize) =
        decode_from_slice(&encoded).expect("decode lowercase empty payload");
    assert_eq!(decoded, empty_payload);
    assert_eq!(bytes_read, encoded.len());
}

// ---------------------------------------------------------------------------
// 19. rename_all = "camelCase" on multi-generic struct
// ---------------------------------------------------------------------------

#[derive(Debug, PartialEq, Encode, Decode)]
#[oxicode(rename_all = "camelCase")]
struct CamelMultiGeneric<K, V> {
    entry_key: K,
    entry_value: V,
    entry_version: u32,
}

#[test]
fn test_rename_all_camel_case_multi_generic_roundtrip() {
    let original = CamelMultiGeneric::<u64, String> {
        entry_key: 9_999_999_999,
        entry_value: "complex_value".to_string(),
        entry_version: 3,
    };
    let encoded = encode_to_vec(&original).expect("encode camelCase multi-generic");
    let (decoded, bytes_read): (CamelMultiGeneric<u64, String>, usize) =
        decode_from_slice(&encoded).expect("decode camelCase multi-generic");
    assert_eq!(decoded, original);
    assert_eq!(bytes_read, encoded.len());
}

// ---------------------------------------------------------------------------
// 20. rename_all = "PascalCase" on struct with unit, tuple, and named fields
//     in a single enum — verify each arm independently
// ---------------------------------------------------------------------------

#[derive(Debug, PartialEq, Encode, Decode)]
#[oxicode(rename_all = "PascalCase")]
enum PascalMixedEnum {
    EmptySlot,
    SingleValue(i64),
    RichRecord { slot_index: u32, slot_data: Vec<u8> },
}

#[test]
fn test_rename_all_pascal_case_mixed_enum_each_arm() {
    let unit_case = PascalMixedEnum::EmptySlot;
    let enc_u = encode_to_vec(&unit_case).expect("encode EmptySlot");
    let (dec_u, _): (PascalMixedEnum, usize) = decode_from_slice(&enc_u).expect("decode EmptySlot");
    assert_eq!(dec_u, unit_case);

    let tuple_case = PascalMixedEnum::SingleValue(i64::MIN);
    let enc_t = encode_to_vec(&tuple_case).expect("encode SingleValue");
    let (dec_t, _): (PascalMixedEnum, usize) =
        decode_from_slice(&enc_t).expect("decode SingleValue");
    assert_eq!(dec_t, tuple_case);

    let struct_case = PascalMixedEnum::RichRecord {
        slot_index: 0,
        slot_data: vec![0xFF; 16],
    };
    let enc_s = encode_to_vec(&struct_case).expect("encode RichRecord");
    let (dec_s, _): (PascalMixedEnum, usize) =
        decode_from_slice(&enc_s).expect("decode RichRecord");
    assert_eq!(dec_s, struct_case);
}

// ---------------------------------------------------------------------------
// 21. rename_all = "UPPERCASE" + individual variant rename on enum
// ---------------------------------------------------------------------------

#[derive(Debug, PartialEq, Encode, Decode)]
#[oxicode(rename_all = "UPPERCASE")]
enum UppercaseVariantRename {
    #[oxicode(rename = "INIT_EVT")]
    InitEvent,
    ProcessEvent {
        sequence_num: u32,
    },
    #[oxicode(rename = "DONE_EVT")]
    DoneEvent(u64),
}

#[test]
fn test_rename_all_uppercase_variant_rename_roundtrip() {
    let cases = vec![
        UppercaseVariantRename::InitEvent,
        UppercaseVariantRename::ProcessEvent { sequence_num: 42 },
        UppercaseVariantRename::DoneEvent(123_456_789),
    ];
    for case in cases {
        let encoded = encode_to_vec(&case).expect("encode UPPERCASE + variant rename");
        let (decoded, bytes_read): (UppercaseVariantRename, usize) =
            decode_from_slice(&encoded).expect("decode UPPERCASE + variant rename");
        assert_eq!(decoded, case);
        assert_eq!(bytes_read, encoded.len());
    }
}

// ---------------------------------------------------------------------------
// 22. rename_all = "snake_case" on struct with deeply nested Vec<Option<T>>
// ---------------------------------------------------------------------------

#[derive(Debug, PartialEq, Encode, Decode)]
#[oxicode(rename_all = "snake_case")]
struct SnakeDeepNested {
    matrix_rows: Vec<Vec<u32>>,
    optional_tags: Vec<Option<String>>,
    root_value: Option<u64>,
}

#[test]
fn test_rename_all_snake_case_deep_nested_roundtrip() {
    let original = SnakeDeepNested {
        matrix_rows: vec![vec![1, 2, 3], vec![], vec![9, 8]],
        optional_tags: vec![
            Some("present".to_string()),
            None,
            Some("also_present".to_string()),
        ],
        root_value: Some(u64::MAX),
    };
    let encoded = encode_to_vec(&original).expect("encode snake_case deep nested");
    let (decoded, bytes_read): (SnakeDeepNested, usize) =
        decode_from_slice(&encoded).expect("decode snake_case deep nested");
    assert_eq!(decoded, original);
    assert_eq!(bytes_read, encoded.len());
}
