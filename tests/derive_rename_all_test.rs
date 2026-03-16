//! Tests for the `#[oxicode(rename_all = "...")]` container attribute.
//!
//! OxiCode is a binary format where fields and variants are encoded positionally,
//! not by name. The `rename_all` attribute is therefore a **no-op on the wire**: it
//! is parsed, validated, and stored for diagnostic / future text-layer use, but it
//! does not change the byte layout emitted or consumed by Encode / Decode.
//!
//! Every test below confirms:
//!   1. The attribute compiles without error.
//!   2. Encode → Decode roundtrip reproduces the original value exactly.
//!   3. Where applicable, the encoded byte sequence is the same as an equivalent
//!      struct/enum without the attribute (wire-format identity).

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
use oxicode::{decode_from_slice, encode_to_vec, Decode, Encode};

// ---------------------------------------------------------------------------
// 1. Struct with rename_all = "camelCase" — fields get renamed (no-op on wire)
// ---------------------------------------------------------------------------

#[derive(Debug, PartialEq, Encode, Decode)]
#[oxicode(rename_all = "camelCase")]
struct CamelCaseStruct {
    first_name: String,
    last_name: String,
    user_age: u32,
}

#[test]
fn test_struct_rename_all_camel_case_roundtrip() {
    let original = CamelCaseStruct {
        first_name: "Alice".to_string(),
        last_name: "Wonderland".to_string(),
        user_age: 30,
    };
    let encoded = encode_to_vec(&original).expect("encode camelCase struct");
    let (decoded, bytes_read): (CamelCaseStruct, usize) =
        decode_from_slice(&encoded).expect("decode camelCase struct");
    assert_eq!(decoded, original);
    assert_eq!(bytes_read, encoded.len());
}

// ---------------------------------------------------------------------------
// 2. Struct with rename_all = "snake_case" — identity for snake_case fields
// ---------------------------------------------------------------------------

#[derive(Debug, PartialEq, Encode, Decode)]
#[oxicode(rename_all = "snake_case")]
struct SnakeCaseStruct {
    field_one: u64,
    field_two: bool,
    field_three: String,
}

#[test]
fn test_struct_rename_all_snake_case_roundtrip() {
    let original = SnakeCaseStruct {
        field_one: u64::MAX,
        field_two: true,
        field_three: "snake".to_string(),
    };
    let encoded = encode_to_vec(&original).expect("encode snake_case struct");
    let (decoded, bytes_read): (SnakeCaseStruct, usize) =
        decode_from_slice(&encoded).expect("decode snake_case struct");
    assert_eq!(decoded, original);
    assert_eq!(bytes_read, encoded.len());
}

// ---------------------------------------------------------------------------
// 3. Struct with rename_all = "PascalCase"
// ---------------------------------------------------------------------------

#[derive(Debug, PartialEq, Encode, Decode)]
#[oxicode(rename_all = "PascalCase")]
struct PascalCaseStruct {
    account_id: u32,
    display_name: String,
    is_active: bool,
}

#[test]
fn test_struct_rename_all_pascal_case_roundtrip() {
    let original = PascalCaseStruct {
        account_id: 42,
        display_name: "PascalUser".to_string(),
        is_active: true,
    };
    let encoded = encode_to_vec(&original).expect("encode PascalCase struct");
    let (decoded, bytes_read): (PascalCaseStruct, usize) =
        decode_from_slice(&encoded).expect("decode PascalCase struct");
    assert_eq!(decoded, original);
    assert_eq!(bytes_read, encoded.len());
}

// ---------------------------------------------------------------------------
// 4. Struct with rename_all = "SCREAMING_SNAKE_CASE"
// ---------------------------------------------------------------------------

#[derive(Debug, PartialEq, Encode, Decode)]
#[oxicode(rename_all = "SCREAMING_SNAKE_CASE")]
struct ScreamingSnakeCaseStruct {
    max_retry_count: u8,
    timeout_ms: u64,
    error_message: String,
}

#[test]
fn test_struct_rename_all_screaming_snake_case_roundtrip() {
    let original = ScreamingSnakeCaseStruct {
        max_retry_count: 5,
        timeout_ms: 30_000,
        error_message: "TIMEOUT".to_string(),
    };
    let encoded = encode_to_vec(&original).expect("encode SCREAMING_SNAKE_CASE struct");
    let (decoded, bytes_read): (ScreamingSnakeCaseStruct, usize) =
        decode_from_slice(&encoded).expect("decode SCREAMING_SNAKE_CASE struct");
    assert_eq!(decoded, original);
    assert_eq!(bytes_read, encoded.len());
}

// ---------------------------------------------------------------------------
// 5. Struct with rename_all = "kebab-case"
// ---------------------------------------------------------------------------

#[derive(Debug, PartialEq, Encode, Decode)]
#[oxicode(rename_all = "kebab-case")]
struct KebabCaseStruct {
    http_method: String,
    response_code: u16,
    content_type: String,
}

#[test]
fn test_struct_rename_all_kebab_case_roundtrip() {
    let original = KebabCaseStruct {
        http_method: "GET".to_string(),
        response_code: 200,
        content_type: "application/json".to_string(),
    };
    let encoded = encode_to_vec(&original).expect("encode kebab-case struct");
    let (decoded, bytes_read): (KebabCaseStruct, usize) =
        decode_from_slice(&encoded).expect("decode kebab-case struct");
    assert_eq!(decoded, original);
    assert_eq!(bytes_read, encoded.len());
}

// ---------------------------------------------------------------------------
// 6. Struct with rename_all = "lowercase"
// ---------------------------------------------------------------------------

#[derive(Debug, PartialEq, Encode, Decode)]
#[oxicode(rename_all = "lowercase")]
struct LowercaseStruct {
    node_id: u32,
    payload: Vec<u8>,
    checksum: u64,
}

#[test]
fn test_struct_rename_all_lowercase_roundtrip() {
    let original = LowercaseStruct {
        node_id: 1,
        payload: vec![0xCA, 0xFE, 0xBA, 0xBE],
        checksum: 0xDEAD_BEEF_CAFE_F00D,
    };
    let encoded = encode_to_vec(&original).expect("encode lowercase struct");
    let (decoded, bytes_read): (LowercaseStruct, usize) =
        decode_from_slice(&encoded).expect("decode lowercase struct");
    assert_eq!(decoded, original);
    assert_eq!(bytes_read, encoded.len());
}

// ---------------------------------------------------------------------------
// 7. Enum with rename_all = "camelCase" on variants
// ---------------------------------------------------------------------------

#[derive(Debug, PartialEq, Encode, Decode)]
#[oxicode(rename_all = "camelCase")]
enum CamelCaseEnum {
    ConnectToServer,
    DisconnectFromServer,
    SendMessage(String),
}

#[test]
fn test_enum_rename_all_camel_case_roundtrip() {
    let variants: Vec<CamelCaseEnum> = vec![
        CamelCaseEnum::ConnectToServer,
        CamelCaseEnum::DisconnectFromServer,
        CamelCaseEnum::SendMessage("hello".to_string()),
    ];
    for variant in variants {
        let encoded = encode_to_vec(&variant).expect("encode camelCase enum variant");
        let (decoded, bytes_read): (CamelCaseEnum, usize) =
            decode_from_slice(&encoded).expect("decode camelCase enum variant");
        assert_eq!(decoded, variant);
        assert_eq!(bytes_read, encoded.len());
    }
}

// ---------------------------------------------------------------------------
// 8. Enum with rename_all = "snake_case"
// ---------------------------------------------------------------------------

#[derive(Debug, PartialEq, Encode, Decode)]
#[oxicode(rename_all = "snake_case")]
enum SnakeCaseEnum {
    UserCreated { user_id: u32, user_name: String },
    OrderPlaced(u64),
    PaymentReceived,
}

#[test]
fn test_enum_rename_all_snake_case_roundtrip() {
    let variants: Vec<SnakeCaseEnum> = vec![
        SnakeCaseEnum::UserCreated {
            user_id: 7,
            user_name: "bob".to_string(),
        },
        SnakeCaseEnum::OrderPlaced(9_999),
        SnakeCaseEnum::PaymentReceived,
    ];
    for variant in variants {
        let encoded = encode_to_vec(&variant).expect("encode snake_case enum variant");
        let (decoded, bytes_read): (SnakeCaseEnum, usize) =
            decode_from_slice(&encoded).expect("decode snake_case enum variant");
        assert_eq!(decoded, variant);
        assert_eq!(bytes_read, encoded.len());
    }
}

// ---------------------------------------------------------------------------
// 9. Enum with rename_all = "PascalCase"
// ---------------------------------------------------------------------------

#[derive(Debug, PartialEq, Encode, Decode)]
#[oxicode(rename_all = "PascalCase")]
#[allow(clippy::enum_variant_names)]
enum PascalCaseEnum {
    NetworkError,
    TimeoutError,
    ValidationError { field: String, reason: String },
}

#[test]
fn test_enum_rename_all_pascal_case_roundtrip() {
    let variants: Vec<PascalCaseEnum> = vec![
        PascalCaseEnum::NetworkError,
        PascalCaseEnum::TimeoutError,
        PascalCaseEnum::ValidationError {
            field: "email".to_string(),
            reason: "invalid format".to_string(),
        },
    ];
    for variant in variants {
        let encoded = encode_to_vec(&variant).expect("encode PascalCase enum variant");
        let (decoded, bytes_read): (PascalCaseEnum, usize) =
            decode_from_slice(&encoded).expect("decode PascalCase enum variant");
        assert_eq!(decoded, variant);
        assert_eq!(bytes_read, encoded.len());
    }
}

// ---------------------------------------------------------------------------
// 10. Combined rename_all + individual field rename
//     Individual #[oxicode(rename = "...")] takes semantic precedence;
//     both are no-ops on the binary wire, but both must parse without error.
// ---------------------------------------------------------------------------

#[derive(Debug, PartialEq, Encode, Decode)]
#[oxicode(rename_all = "camelCase")]
struct CombinedRenameStruct {
    #[oxicode(rename = "userId")]
    user_id: u32,
    display_name: String,
    #[oxicode(rename = "ts")]
    created_at: u64,
}

#[test]
fn test_combined_rename_all_and_field_rename_roundtrip() {
    let original = CombinedRenameStruct {
        user_id: 100,
        display_name: "Combined".to_string(),
        created_at: 1_700_000_000,
    };
    let encoded = encode_to_vec(&original).expect("encode combined rename struct");
    let (decoded, bytes_read): (CombinedRenameStruct, usize) =
        decode_from_slice(&encoded).expect("decode combined rename struct");
    assert_eq!(decoded, original);
    assert_eq!(bytes_read, encoded.len());
}

// ---------------------------------------------------------------------------
// 11. Roundtrip — rename_all does not change byte content vs. plain struct
// ---------------------------------------------------------------------------

#[derive(Debug, PartialEq, Encode, Decode)]
struct BaselineStruct {
    count: u32,
    label: String,
}

#[derive(Debug, PartialEq, Encode, Decode)]
#[oxicode(rename_all = "camelCase")]
struct RenamedStruct {
    count: u32,
    label: String,
}

#[test]
fn test_rename_all_does_not_change_wire_bytes() {
    let baseline = BaselineStruct {
        count: 7,
        label: "hello".to_string(),
    };
    let renamed = RenamedStruct {
        count: 7,
        label: "hello".to_string(),
    };
    let baseline_bytes = encode_to_vec(&baseline).expect("encode baseline");
    let renamed_bytes = encode_to_vec(&renamed).expect("encode renamed");
    assert_eq!(
        baseline_bytes, renamed_bytes,
        "rename_all must not alter the binary wire bytes"
    );
}

// ---------------------------------------------------------------------------
// 12. Roundtrip with numeric field values — camelCase
// ---------------------------------------------------------------------------

#[derive(Debug, PartialEq, Encode, Decode)]
#[oxicode(rename_all = "camelCase")]
struct NumericPayload {
    frame_index: u32,
    byte_offset: u64,
    sample_rate: u32,
}

#[test]
fn test_rename_all_camel_case_numeric_roundtrip() {
    let original = NumericPayload {
        frame_index: 1_024,
        byte_offset: 0xFFFF_FFFF_0000_0001,
        sample_rate: 44_100,
    };
    let encoded = encode_to_vec(&original).expect("encode numeric camelCase");
    let (decoded, bytes_read): (NumericPayload, usize) =
        decode_from_slice(&encoded).expect("decode numeric camelCase");
    assert_eq!(decoded, original);
    assert_eq!(bytes_read, encoded.len());
}

// ---------------------------------------------------------------------------
// 13. Roundtrip with Vec field — SCREAMING_SNAKE_CASE
// ---------------------------------------------------------------------------

#[derive(Debug, PartialEq, Encode, Decode)]
#[oxicode(rename_all = "SCREAMING_SNAKE_CASE")]
struct ScreamingWithVec {
    chunk_data: Vec<u8>,
    chunk_index: u32,
}

#[test]
fn test_rename_all_screaming_snake_vec_roundtrip() {
    let original = ScreamingWithVec {
        chunk_data: (0u8..=255u8).collect(),
        chunk_index: 3,
    };
    let encoded = encode_to_vec(&original).expect("encode SCREAMING_SNAKE_CASE vec");
    let (decoded, bytes_read): (ScreamingWithVec, usize) =
        decode_from_slice(&encoded).expect("decode SCREAMING_SNAKE_CASE vec");
    assert_eq!(decoded, original);
    assert_eq!(bytes_read, encoded.len());
}

// ---------------------------------------------------------------------------
// 14. Roundtrip with Option fields — kebab-case, Some values
// ---------------------------------------------------------------------------

#[derive(Debug, PartialEq, Encode, Decode)]
#[oxicode(rename_all = "kebab-case")]
struct KebabWithOption {
    primary_key: u64,
    optional_label: Option<String>,
    fallback_value: Option<u32>,
}

#[test]
fn test_rename_all_kebab_case_option_some_roundtrip() {
    let original = KebabWithOption {
        primary_key: 9_001,
        optional_label: Some("kebab-label".to_string()),
        fallback_value: Some(42),
    };
    let encoded = encode_to_vec(&original).expect("encode kebab-case Option::Some");
    let (decoded, bytes_read): (KebabWithOption, usize) =
        decode_from_slice(&encoded).expect("decode kebab-case Option::Some");
    assert_eq!(decoded, original);
    assert_eq!(bytes_read, encoded.len());
}

// ---------------------------------------------------------------------------
// 15. Roundtrip with Option fields — kebab-case, None values
// ---------------------------------------------------------------------------

#[test]
fn test_rename_all_kebab_case_option_none_roundtrip() {
    let original = KebabWithOption {
        primary_key: 0,
        optional_label: None,
        fallback_value: None,
    };
    let encoded = encode_to_vec(&original).expect("encode kebab-case Option::None");
    let (decoded, bytes_read): (KebabWithOption, usize) =
        decode_from_slice(&encoded).expect("decode kebab-case Option::None");
    assert_eq!(decoded, original);
    assert_eq!(bytes_read, encoded.len());
}

// ---------------------------------------------------------------------------
// 16. Enum with rename_all = "SCREAMING_SNAKE_CASE" and named/tuple variants
// ---------------------------------------------------------------------------

#[derive(Debug, PartialEq, Encode, Decode)]
#[oxicode(rename_all = "SCREAMING_SNAKE_CASE")]
enum ScreamingEnum {
    ConnectionLost,
    DataReceived(Vec<u8>),
    HeartbeatTimeout { elapsed_ms: u64 },
}

#[test]
fn test_enum_rename_all_screaming_snake_roundtrip() {
    let variants: Vec<ScreamingEnum> = vec![
        ScreamingEnum::ConnectionLost,
        ScreamingEnum::DataReceived(vec![0xDE, 0xAD, 0xBE, 0xEF]),
        ScreamingEnum::HeartbeatTimeout { elapsed_ms: 5_000 },
    ];
    for variant in variants {
        let encoded = encode_to_vec(&variant).expect("encode SCREAMING_SNAKE_CASE enum");
        let (decoded, bytes_read): (ScreamingEnum, usize) =
            decode_from_slice(&encoded).expect("decode SCREAMING_SNAKE_CASE enum");
        assert_eq!(decoded, variant);
        assert_eq!(bytes_read, encoded.len());
    }
}

// ---------------------------------------------------------------------------
// 17. Enum with rename_all = "kebab-case"
// ---------------------------------------------------------------------------

#[derive(Debug, PartialEq, Encode, Decode)]
#[oxicode(rename_all = "kebab-case")]
#[allow(clippy::enum_variant_names)]
enum KebabEnum {
    StartProcess,
    StopProcess,
    RestartProcess { grace_period_ms: u32 },
}

#[test]
fn test_enum_rename_all_kebab_case_roundtrip() {
    let variants: Vec<KebabEnum> = vec![
        KebabEnum::StartProcess,
        KebabEnum::StopProcess,
        KebabEnum::RestartProcess {
            grace_period_ms: 1_000,
        },
    ];
    for variant in variants {
        let encoded = encode_to_vec(&variant).expect("encode kebab-case enum");
        let (decoded, bytes_read): (KebabEnum, usize) =
            decode_from_slice(&encoded).expect("decode kebab-case enum");
        assert_eq!(decoded, variant);
        assert_eq!(bytes_read, encoded.len());
    }
}

// ---------------------------------------------------------------------------
// 18. rename_all combined with tag_type — attribute interaction roundtrip
// ---------------------------------------------------------------------------

#[derive(Debug, PartialEq, Encode, Decode)]
#[oxicode(rename_all = "camelCase", tag_type = "u8")]
#[allow(clippy::enum_variant_names)]
enum CompactCamelEnum {
    FirstVariant,
    SecondVariant(u32),
    ThirdVariant { value: String },
}

#[test]
fn test_rename_all_combined_with_tag_type_roundtrip() {
    let variants: Vec<CompactCamelEnum> = vec![
        CompactCamelEnum::FirstVariant,
        CompactCamelEnum::SecondVariant(255),
        CompactCamelEnum::ThirdVariant {
            value: "combined".to_string(),
        },
    ];
    for variant in variants {
        let encoded = encode_to_vec(&variant).expect("encode rename_all + tag_type");
        let (decoded, bytes_read): (CompactCamelEnum, usize) =
            decode_from_slice(&encoded).expect("decode rename_all + tag_type");
        assert_eq!(decoded, variant);
        assert_eq!(bytes_read, encoded.len());
    }
}

// ---------------------------------------------------------------------------
// 19. rename_all on a generic struct — PascalCase
// ---------------------------------------------------------------------------

#[derive(Debug, PartialEq, Encode, Decode)]
#[oxicode(rename_all = "PascalCase")]
struct PascalGeneric<T> {
    inner_value: T,
    item_count: u32,
}

#[test]
fn test_rename_all_pascal_case_generic_roundtrip() {
    let original = PascalGeneric::<String> {
        inner_value: "generic_value".to_string(),
        item_count: 10,
    };
    let encoded = encode_to_vec(&original).expect("encode PascalCase generic");
    let (decoded, bytes_read): (PascalGeneric<String>, usize) =
        decode_from_slice(&encoded).expect("decode PascalCase generic");
    assert_eq!(decoded, original);
    assert_eq!(bytes_read, encoded.len());
}

// ---------------------------------------------------------------------------
// 20. Full combination: rename_all + individual rename + skip — roundtrip
// ---------------------------------------------------------------------------

#[derive(Debug, PartialEq, Encode, Decode)]
#[oxicode(rename_all = "SCREAMING_SNAKE_CASE")]
struct FullCombinationStruct {
    #[oxicode(rename = "ID")]
    record_id: u32,
    record_name: String,
    #[oxicode(skip)]
    cache_hit: bool, // not encoded; decodes as Default (false)
    record_score: i64,
}

#[test]
fn test_rename_all_full_combination_roundtrip() {
    let original = FullCombinationStruct {
        record_id: 999,
        record_name: "full-combo".to_string(),
        cache_hit: true, // this value is NOT encoded
        record_score: -42,
    };
    let encoded = encode_to_vec(&original).expect("encode full combination");
    let (decoded, bytes_read): (FullCombinationStruct, usize) =
        decode_from_slice(&encoded).expect("decode full combination");
    assert_eq!(decoded.record_id, original.record_id);
    assert_eq!(decoded.record_name, original.record_name);
    assert!(
        !decoded.cache_hit,
        "skipped field must decode as Default (false)"
    );
    assert_eq!(decoded.record_score, original.record_score);
    assert_eq!(bytes_read, encoded.len());
}
