//! Advanced tests for OxiCode `#[oxicode(rename = "...")]` and related derive attributes.
//!
//! Focuses on attribute combinations and edge cases NOT covered by:
//!   - derive_attr_test.rs  (basic rename, skip, default, variant)
//!   - derive_rename_all_test.rs  (rename_all coverage)
//!   - derive_flatten_test.rs  (basic flatten)
//!   - derive_default_value_test.rs  (default = "fn")
//!   - derive_skip_combined_test.rs  (skip combinations)
//!
//! All tests are top-level `#[test]` functions; no `#[cfg(test)]` wrappers.
//! No `unwrap()` — every fallible call uses `.expect("msg")`.

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
// Helper encode/decode modules used by encode_with / decode_with tests
// ---------------------------------------------------------------------------

mod negate_i32 {
    use oxicode::{
        de::{Decode, Decoder},
        enc::{Encode, Encoder},
        Error,
    };

    pub fn encode<E: Encoder>(value: &i32, encoder: &mut E) -> Result<(), Error> {
        value.encode(encoder)
    }

    pub fn decode<D: Decoder<Context = ()>>(decoder: &mut D) -> Result<i32, Error> {
        let v = i32::decode(decoder)?;
        Ok(-v)
    }
}

mod double_u32 {
    use oxicode::{
        de::{Decode, Decoder},
        enc::{Encode, Encoder},
        Error,
    };

    pub fn encode<E: Encoder>(value: &u32, encoder: &mut E) -> Result<(), Error> {
        (*value * 2).encode(encoder)
    }

    pub fn decode<D: Decoder<Context = ()>>(decoder: &mut D) -> Result<u32, Error> {
        let v = u32::decode(decoder)?;
        Ok(v / 2)
    }
}

// ---------------------------------------------------------------------------
// Helper default functions
// ---------------------------------------------------------------------------

fn default_flag() -> bool {
    true
}

fn default_count() -> u32 {
    77
}

fn default_label() -> String {
    "unlabeled".to_string()
}

// ---------------------------------------------------------------------------
// Test 1: `rename` + `bytes` on the same field — roundtrip
//
// The `bytes` attribute enables a bulk raw-bytes encoding path.
// The `rename` attribute is a no-op on the wire but must parse without error.
// ---------------------------------------------------------------------------

#[derive(Debug, PartialEq, Encode, Decode)]
struct RenamePlusBytesField {
    #[oxicode(rename = "rawData", bytes)]
    raw: Vec<u8>,
    id: u32,
}

#[test]
fn test_01_rename_and_bytes_attr_roundtrip() {
    let original = RenamePlusBytesField {
        raw: vec![0xCA, 0xFE, 0xBA, 0xBE],
        id: 42,
    };
    let encoded = encode_to_vec(&original).expect("encode rename+bytes");
    let (decoded, bytes_read): (RenamePlusBytesField, usize) =
        decode_from_slice(&encoded).expect("decode rename+bytes");
    assert_eq!(decoded, original);
    assert_eq!(bytes_read, encoded.len());
}

// ---------------------------------------------------------------------------
// Test 2: `rename` + `seq_len = "u8"` on the same field — roundtrip
//
// seq_len controls the wire-level length prefix type. rename is a no-op.
// Both attributes on the same field must compile and produce correct output.
// ---------------------------------------------------------------------------

#[derive(Debug, PartialEq, Encode, Decode)]
struct RenamePlusSeqLen {
    #[oxicode(rename = "itemList", seq_len = "u8")]
    items: Vec<u32>,
    name: String,
}

#[test]
fn test_02_rename_and_seq_len_roundtrip() {
    let original = RenamePlusSeqLen {
        items: vec![1, 2, 3, 4, 5],
        name: "compact".to_string(),
    };
    let encoded = encode_to_vec(&original).expect("encode rename+seq_len");
    let (decoded, bytes_read): (RenamePlusSeqLen, usize) =
        decode_from_slice(&encoded).expect("decode rename+seq_len");
    assert_eq!(decoded, original);
    assert_eq!(bytes_read, encoded.len());
}

// ---------------------------------------------------------------------------
// Test 3: `rename` + `seq_len` compactness — u8 length prefix is 1 byte
//
// Verify that a u8-prefix'd Vec<u8> with 3 elements encodes the length
// as a single raw byte (value 3), even when the field also carries `rename`.
// ---------------------------------------------------------------------------

#[derive(Debug, PartialEq, Encode, Decode)]
struct RenameSeqLenCompact {
    #[oxicode(rename = "payload", seq_len = "u8")]
    payload: Vec<u8>,
}

#[test]
fn test_03_rename_seq_len_compact_prefix_is_one_byte() {
    let original = RenameSeqLenCompact {
        payload: vec![10, 20, 30],
    };
    let encoded = encode_to_vec(&original).expect("encode rename+seq_len compact");
    // First byte must be the raw u8 length (3), not a varint-extended form.
    assert_eq!(
        encoded[0], 3u8,
        "length prefix must be raw u8 value 3, got {}",
        encoded[0]
    );
    let (decoded, _): (RenameSeqLenCompact, _) =
        decode_from_slice(&encoded).expect("decode rename+seq_len compact");
    assert_eq!(decoded, original);
}

// ---------------------------------------------------------------------------
// Test 4: `rename` + `encode_with` / `decode_with` on same field — roundtrip
//
// The custom encode/decode functions take precedence for the wire format;
// `rename` is a no-op on the wire but must not conflict with encode_with.
// ---------------------------------------------------------------------------

#[derive(Debug, PartialEq, Encode, Decode)]
struct RenamePlusEncodeWith {
    #[oxicode(
        rename = "negatedValue",
        encode_with = "negate_i32::encode",
        decode_with = "negate_i32::decode"
    )]
    value: i32,
    label: String,
}

#[test]
fn test_04_rename_and_encode_with_roundtrip() {
    let original = RenamePlusEncodeWith {
        value: 10,
        label: "encode_with_rename".to_string(),
    };
    let encoded = encode_to_vec(&original).expect("encode rename+encode_with");
    let (decoded, bytes_read): (RenamePlusEncodeWith, usize) =
        decode_from_slice(&encoded).expect("decode rename+encode_with");
    // The custom decode_with negates the value, so decoded.value = -10
    assert_eq!(decoded.value, -10);
    assert_eq!(decoded.label, original.label);
    assert_eq!(bytes_read, encoded.len());
}

// ---------------------------------------------------------------------------
// Test 5: `rename` + `encode_with` / `decode_with` — wire transform is active
//
// Confirm the encode_with doubling/halving transform works independently of
// the rename no-op.  Decoded value must be the original due to symmetric
// double→halve transform.
// ---------------------------------------------------------------------------

#[derive(Debug, PartialEq, Encode, Decode)]
struct RenameWithDoubler {
    #[oxicode(
        rename = "doubledScore",
        encode_with = "double_u32::encode",
        decode_with = "double_u32::decode"
    )]
    score: u32,
    tag: u8,
}

#[test]
fn test_05_rename_encode_with_double_halve_roundtrip() {
    let original = RenameWithDoubler { score: 50, tag: 7 };
    let encoded = encode_to_vec(&original).expect("encode rename+double");
    let (decoded, bytes_read): (RenameWithDoubler, usize) =
        decode_from_slice(&encoded).expect("decode rename+double");
    // double_u32 encodes 50 as 100, decodes 100 as 50 — symmetric roundtrip
    assert_eq!(decoded.score, 50);
    assert_eq!(decoded.tag, 7);
    assert_eq!(bytes_read, encoded.len());
}

// ---------------------------------------------------------------------------
// Test 6: `rename_all = "camelCase"` container + `seq_len = "u16"` field
//
// Container-level rename_all and field-level seq_len must coexist without error.
// ---------------------------------------------------------------------------

#[derive(Debug, PartialEq, Encode, Decode)]
#[oxicode(rename_all = "camelCase")]
struct RenameAllWithSeqLen {
    record_id: u32,
    #[oxicode(seq_len = "u16")]
    entry_list: Vec<i64>,
}

#[test]
fn test_06_rename_all_plus_seq_len_field_roundtrip() {
    let original = RenameAllWithSeqLen {
        record_id: 1024,
        entry_list: vec![-1, 0, 1, 100, i64::MAX],
    };
    let encoded = encode_to_vec(&original).expect("encode rename_all+seq_len");
    let (decoded, bytes_read): (RenameAllWithSeqLen, usize) =
        decode_from_slice(&encoded).expect("decode rename_all+seq_len");
    assert_eq!(decoded, original);
    assert_eq!(bytes_read, encoded.len());
}

// ---------------------------------------------------------------------------
// Test 7: `rename_all = "SCREAMING_SNAKE_CASE"` container + `bytes` field
// ---------------------------------------------------------------------------

#[derive(Debug, PartialEq, Encode, Decode)]
#[oxicode(rename_all = "SCREAMING_SNAKE_CASE")]
struct RenameAllPlusBytes {
    frame_number: u32,
    #[oxicode(bytes)]
    raw_frame_data: Vec<u8>,
}

#[test]
fn test_07_rename_all_screaming_snake_plus_bytes_field_roundtrip() {
    let original = RenameAllPlusBytes {
        frame_number: 5,
        raw_frame_data: (0u8..=127u8).collect(),
    };
    let encoded = encode_to_vec(&original).expect("encode rename_all SCREAMING+bytes");
    let (decoded, bytes_read): (RenameAllPlusBytes, usize) =
        decode_from_slice(&encoded).expect("decode rename_all SCREAMING+bytes");
    assert_eq!(decoded, original);
    assert_eq!(bytes_read, encoded.len());
}

// ---------------------------------------------------------------------------
// Test 8: `rename` on fields inside a generic struct — roundtrip
// ---------------------------------------------------------------------------

#[derive(Debug, PartialEq, Encode, Decode)]
struct RenamedGeneric<T> {
    #[oxicode(rename = "theValue")]
    value: T,
    #[oxicode(rename = "theCount")]
    count: u32,
}

#[test]
fn test_08_rename_on_generic_struct_string_roundtrip() {
    let original = RenamedGeneric::<String> {
        value: "generic string".to_string(),
        count: 3,
    };
    let encoded = encode_to_vec(&original).expect("encode renamed generic<string>");
    let (decoded, bytes_read): (RenamedGeneric<String>, usize) =
        decode_from_slice(&encoded).expect("decode renamed generic<string>");
    assert_eq!(decoded, original);
    assert_eq!(bytes_read, encoded.len());
}

#[test]
fn test_09_rename_on_generic_struct_vec_roundtrip() {
    let original = RenamedGeneric::<Vec<u8>> {
        value: vec![0xDE, 0xAD, 0xBE, 0xEF],
        count: 4,
    };
    let encoded = encode_to_vec(&original).expect("encode renamed generic<vec>");
    let (decoded, bytes_read): (RenamedGeneric<Vec<u8>>, usize) =
        decode_from_slice(&encoded).expect("decode renamed generic<vec>");
    assert_eq!(decoded, original);
    assert_eq!(bytes_read, encoded.len());
}

// ---------------------------------------------------------------------------
// Test 10: `rename` on fields in an enum tuple variant — roundtrip
// ---------------------------------------------------------------------------

#[derive(Debug, PartialEq, Encode, Decode)]
enum MessageWithRenamedFields {
    Text {
        #[oxicode(rename = "bodyText")]
        body: String,
        #[oxicode(rename = "timestampMs")]
        timestamp: u64,
    },
    Binary {
        #[oxicode(rename = "rawBytes")]
        data: Vec<u8>,
    },
    Empty,
}

#[test]
fn test_10_rename_in_enum_named_variants_roundtrip() {
    let variants = [
        MessageWithRenamedFields::Text {
            body: "hello world".to_string(),
            timestamp: 1_700_000_000,
        },
        MessageWithRenamedFields::Binary {
            data: vec![0xCA, 0xFE],
        },
        MessageWithRenamedFields::Empty,
    ];
    for variant in &variants {
        let encoded = encode_to_vec(variant).expect("encode enum with renamed fields");
        let (decoded, bytes_read): (MessageWithRenamedFields, usize) =
            decode_from_slice(&encoded).expect("decode enum with renamed fields");
        assert_eq!(&decoded, variant);
        assert_eq!(bytes_read, encoded.len());
    }
}

// ---------------------------------------------------------------------------
// Test 11: `rename` + `flatten` on the same field
//
// `flatten` causes the inner struct's fields to be encoded inline.
// `rename` is a no-op on the wire. Both must coexist without error.
// ---------------------------------------------------------------------------

#[derive(Debug, PartialEq, Encode, Decode)]
struct InnerCoords {
    x: i32,
    y: i32,
}

#[derive(Debug, PartialEq, Encode, Decode)]
struct RenameAndFlatten {
    label: String,
    #[oxicode(rename = "position", flatten)]
    coords: InnerCoords,
    z: i32,
}

#[test]
fn test_11_rename_and_flatten_combined_roundtrip() {
    let original = RenameAndFlatten {
        label: "point".to_string(),
        coords: InnerCoords { x: -5, y: 10 },
        z: 3,
    };
    let encoded = encode_to_vec(&original).expect("encode rename+flatten");
    let (decoded, bytes_read): (RenameAndFlatten, usize) =
        decode_from_slice(&encoded).expect("decode rename+flatten");
    assert_eq!(decoded, original);
    assert_eq!(bytes_read, encoded.len());
}

// ---------------------------------------------------------------------------
// Test 12: `rename` + `flatten` wire-format identity
//
// Encoding a struct with `flatten` (even when `rename` is present)
// must produce the same bytes as a flat struct with equivalent fields.
// ---------------------------------------------------------------------------

#[derive(Debug, PartialEq, Encode, Decode)]
struct FlatEquiv {
    label: String,
    x: i32,
    y: i32,
    z: i32,
}

#[test]
fn test_12_rename_flatten_wire_bytes_match_flat_struct() {
    let with_flatten = RenameAndFlatten {
        label: "pt".to_string(),
        coords: InnerCoords { x: 1, y: 2 },
        z: 3,
    };
    let flat = FlatEquiv {
        label: "pt".to_string(),
        x: 1,
        y: 2,
        z: 3,
    };
    let flatten_bytes = encode_to_vec(&with_flatten).expect("encode rename+flatten");
    let flat_bytes = encode_to_vec(&flat).expect("encode flat equiv");
    assert_eq!(
        flatten_bytes, flat_bytes,
        "rename+flatten must produce identical bytes to manually flattened struct"
    );
}

// ---------------------------------------------------------------------------
// Test 13: `rename` + `skip` on same field — skip takes precedence on wire
//
// The field must be absent from the byte stream (skip wins over rename's no-op).
// ---------------------------------------------------------------------------

#[derive(Debug, PartialEq, Encode, Decode)]
struct RenameAndSkip {
    real_data: u32,
    #[oxicode(rename = "cachedValue", skip)]
    cache: u64,
    more_data: String,
}

#[test]
fn test_13_rename_plus_skip_field_is_absent_from_wire() {
    let original = RenameAndSkip {
        real_data: 99,
        cache: u64::MAX,
        more_data: "present".to_string(),
    };
    let encoded = encode_to_vec(&original).expect("encode rename+skip");
    let (decoded, bytes_read): (RenameAndSkip, usize) =
        decode_from_slice(&encoded).expect("decode rename+skip");

    assert_eq!(decoded.real_data, 99);
    assert_eq!(decoded.cache, 0u64, "skipped field must be Default (0)");
    assert_eq!(decoded.more_data, "present");
    assert_eq!(bytes_read, encoded.len());

    // Byte count: rename+skip encodes only real_data + more_data
    // (no bytes for u64::MAX which is 9 varint bytes)
    let no_cache_struct = {
        #[derive(Encode)]
        struct Minimal {
            real_data: u32,
            more_data: String,
        }
        Minimal {
            real_data: 99,
            more_data: "present".to_string(),
        }
    };
    let minimal_bytes = encode_to_vec(&no_cache_struct).expect("encode minimal");
    assert_eq!(
        encoded, minimal_bytes,
        "rename+skip struct must encode identically to struct without that field"
    );
}

// ---------------------------------------------------------------------------
// Test 14: `rename` + `default = "fn_path"` on same field
//
// default = "fn" excludes the field from the stream (like skip) and calls the
// function on decode. rename must not interfere with this mechanism.
// ---------------------------------------------------------------------------

#[derive(Debug, PartialEq, Encode, Decode)]
struct RenameAndDefault {
    id: u32,
    #[oxicode(rename = "userFlag", default = "default_flag")]
    flag: bool,
    name: String,
}

#[test]
fn test_14_rename_plus_default_fn_uses_default_on_decode() {
    let original = RenameAndDefault {
        id: 5,
        flag: false, // NOT encoded; default_flag() returns true
        name: "test".to_string(),
    };
    let encoded = encode_to_vec(&original).expect("encode rename+default");
    let (decoded, bytes_read): (RenameAndDefault, usize) =
        decode_from_slice(&encoded).expect("decode rename+default");

    assert_eq!(decoded.id, 5);
    assert!(decoded.flag, "default_flag() must return true");
    assert_eq!(decoded.name, "test");
    assert_eq!(bytes_read, encoded.len());
}

// ---------------------------------------------------------------------------
// Test 15: `rename_all = "UPPERCASE"` (rarely-exercised convention) — roundtrip
// ---------------------------------------------------------------------------

#[derive(Debug, PartialEq, Encode, Decode)]
#[oxicode(rename_all = "UPPERCASE")]
struct UppercaseRenameAll {
    node_id: u32,
    cluster_name: String,
    is_leader: bool,
}

#[test]
fn test_15_rename_all_uppercase_roundtrip() {
    let original = UppercaseRenameAll {
        node_id: 7,
        cluster_name: "alpha".to_string(),
        is_leader: true,
    };
    let encoded = encode_to_vec(&original).expect("encode rename_all UPPERCASE");
    let (decoded, bytes_read): (UppercaseRenameAll, usize) =
        decode_from_slice(&encoded).expect("decode rename_all UPPERCASE");
    assert_eq!(decoded, original);
    assert_eq!(bytes_read, encoded.len());
}

// ---------------------------------------------------------------------------
// Test 16: `rename_all` + `default = "fn_path"` interaction
//
// Container-level rename_all is a no-op; field-level default = "fn" must still
// exclude the field from the wire and restore via the named function.
// ---------------------------------------------------------------------------

#[derive(Debug, PartialEq, Encode, Decode)]
#[oxicode(rename_all = "camelCase")]
struct RenameAllPlusDefaultFn {
    item_count: u32,
    #[oxicode(default = "default_count")]
    retry_count: u32,
    item_label: String,
}

#[test]
fn test_16_rename_all_plus_default_fn_roundtrip() {
    let original = RenameAllPlusDefaultFn {
        item_count: 10,
        retry_count: 0, // NOT encoded; default_count() returns 77
        item_label: "batch".to_string(),
    };
    let encoded = encode_to_vec(&original).expect("encode rename_all+default_fn");
    let (decoded, bytes_read): (RenameAllPlusDefaultFn, usize) =
        decode_from_slice(&encoded).expect("decode rename_all+default_fn");

    assert_eq!(decoded.item_count, 10);
    assert_eq!(decoded.retry_count, 77, "default_count() must return 77");
    assert_eq!(decoded.item_label, "batch");
    assert_eq!(bytes_read, encoded.len());
}

// ---------------------------------------------------------------------------
// Test 17: `rename_all = "kebab-case"` on enum + `variant = N` custom tag
//
// Both container rename_all and variant-level custom tags must coexist.
// The variant tag controls the discriminant; rename_all is a no-op.
// ---------------------------------------------------------------------------

#[derive(Debug, PartialEq, Encode, Decode)]
#[oxicode(rename_all = "kebab-case")]
enum KebabWithCustomTags {
    #[oxicode(variant = 0x10)]
    StartProcess,
    #[oxicode(variant = 0x20)]
    StopProcess,
    #[oxicode(variant = 0x30)]
    ReloadConfig { wait_ms: u32 },
}

#[test]
fn test_17_rename_all_kebab_plus_custom_variant_tags_roundtrip() {
    let variants = [
        KebabWithCustomTags::StartProcess,
        KebabWithCustomTags::StopProcess,
        KebabWithCustomTags::ReloadConfig { wait_ms: 500 },
    ];
    for variant in &variants {
        let encoded = encode_to_vec(variant).expect("encode rename_all kebab + custom variant tag");
        let (decoded, bytes_read): (KebabWithCustomTags, usize) =
            decode_from_slice(&encoded).expect("decode rename_all kebab + custom variant tag");
        assert_eq!(&decoded, variant);
        assert_eq!(bytes_read, encoded.len());
    }
}

// ---------------------------------------------------------------------------
// Test 18: `rename_all = "PascalCase"` + `tag_type = "u8"` — compact enum
// ---------------------------------------------------------------------------

#[derive(Debug, PartialEq, Encode, Decode)]
#[oxicode(rename_all = "PascalCase", tag_type = "u8")]
#[allow(clippy::enum_variant_names)]
enum CompactPascalEnum {
    AlphaState,
    BetaState(u32),
    GammaState { code: u16, message: String },
}

#[test]
fn test_18_rename_all_pascal_plus_tag_type_u8_roundtrip() {
    let variants = [
        CompactPascalEnum::AlphaState,
        CompactPascalEnum::BetaState(255),
        CompactPascalEnum::GammaState {
            code: 404,
            message: "not found".to_string(),
        },
    ];
    for variant in &variants {
        let encoded = encode_to_vec(variant).expect("encode rename_all PascalCase + tag_type u8");
        let (decoded, bytes_read): (CompactPascalEnum, usize) =
            decode_from_slice(&encoded).expect("decode rename_all PascalCase + tag_type u8");
        assert_eq!(&decoded, variant);
        assert_eq!(bytes_read, encoded.len());
    }
}

// ---------------------------------------------------------------------------
// Test 19: `rename` with special characters in the name string — roundtrip
//
// rename = "..." accepts any string. Hyphens, digits, dots, and mixed case
// must all be accepted without compile error and produce a correct roundtrip.
// ---------------------------------------------------------------------------

#[derive(Debug, PartialEq, Encode, Decode)]
struct RenameSpecialChars {
    #[oxicode(rename = "field-with-hyphens")]
    field_a: u32,
    #[oxicode(rename = "field.with.dots")]
    field_b: String,
    #[oxicode(rename = "field123")]
    field_c: bool,
    #[oxicode(rename = "MixedCase_With_Underscores")]
    field_d: u64,
}

#[test]
fn test_19_rename_with_special_characters_roundtrip() {
    let original = RenameSpecialChars {
        field_a: 1,
        field_b: "dots.in.name".to_string(),
        field_c: true,
        field_d: u64::MAX / 2,
    };
    let encoded = encode_to_vec(&original).expect("encode rename special chars");
    let (decoded, bytes_read): (RenameSpecialChars, usize) =
        decode_from_slice(&encoded).expect("decode rename special chars");
    assert_eq!(decoded, original);
    assert_eq!(bytes_read, encoded.len());
}

// ---------------------------------------------------------------------------
// Test 20: `rename` does NOT change wire bytes (wire-format identity check)
//
// Encoding a struct with `rename` on its fields must produce identical bytes
// to an equivalent struct without `rename`.
// ---------------------------------------------------------------------------

#[derive(Debug, PartialEq, Encode, Decode)]
struct WithRenames {
    #[oxicode(rename = "alpha")]
    a: u32,
    #[oxicode(rename = "beta")]
    b: String,
    #[oxicode(rename = "gamma")]
    c: bool,
}

#[derive(Debug, PartialEq, Encode, Decode)]
struct WithoutRenames {
    a: u32,
    b: String,
    c: bool,
}

#[test]
fn test_20_rename_does_not_alter_wire_bytes() {
    let with_renames = WithRenames {
        a: 42,
        b: "binary".to_string(),
        c: true,
    };
    let without_renames = WithoutRenames {
        a: 42,
        b: "binary".to_string(),
        c: true,
    };
    let renamed_bytes = encode_to_vec(&with_renames).expect("encode with renames");
    let plain_bytes = encode_to_vec(&without_renames).expect("encode without renames");
    assert_eq!(
        renamed_bytes, plain_bytes,
        "rename must not alter the binary wire bytes"
    );
}

// ---------------------------------------------------------------------------
// Test 21: `rename_all` + individual `rename` + `skip` three-way combination
//
// Tests the full combination of container rename_all, individual field rename,
// and skip all on the same struct — all must coexist without error and produce
// a correct encode/decode cycle.
// ---------------------------------------------------------------------------

#[derive(Debug, PartialEq, Encode, Decode)]
#[oxicode(rename_all = "snake_case")]
struct TripleCombo {
    #[oxicode(rename = "primaryId")]
    primary_id: u32,
    display_name: String,
    #[oxicode(rename = "cachedAt", skip)]
    cached_at: u64, // not encoded; restores as Default (0)
    active: bool,
    #[oxicode(rename = "retryCount", default = "default_count")]
    retry_count: u32, // not encoded; restores via default_count()
}

#[test]
fn test_21_rename_all_plus_field_rename_plus_skip_combination() {
    let original = TripleCombo {
        primary_id: 101,
        display_name: "triple".to_string(),
        cached_at: 9_999_999_999,
        active: true,
        retry_count: 500, // will NOT be encoded
    };
    let encoded = encode_to_vec(&original).expect("encode triple combo");
    let (decoded, bytes_read): (TripleCombo, usize) =
        decode_from_slice(&encoded).expect("decode triple combo");

    assert_eq!(decoded.primary_id, 101);
    assert_eq!(decoded.display_name, "triple");
    assert_eq!(decoded.cached_at, 0u64, "skipped field must be Default (0)");
    assert!(decoded.active);
    assert_eq!(decoded.retry_count, 77, "default_count() must return 77");
    assert_eq!(bytes_read, encoded.len());
}

// ---------------------------------------------------------------------------
// Test 22: `rename` on enum variant + struct fields inside that variant
//
// Combines variant-level rename with field-level rename inside the variant.
// Also includes a skip + default field to verify all attributes coexist.
// ---------------------------------------------------------------------------

#[derive(Debug, PartialEq, Encode, Decode)]
enum ComplexRenameEnum {
    #[oxicode(rename = "userEvent")]
    UserEvent {
        #[oxicode(rename = "userId")]
        user_id: u32,
        #[oxicode(rename = "eventName")]
        event_name: String,
        #[oxicode(rename = "internalTag", default = "default_label")]
        internal_tag: String, // not encoded; restores via default_label()
    },
    #[oxicode(rename = "systemEvent")]
    SystemEvent(u64),
    #[oxicode(rename = "noOp")]
    NoOp,
}

#[test]
fn test_22_rename_on_enum_variant_and_fields_combined_roundtrip() {
    let user_event = ComplexRenameEnum::UserEvent {
        user_id: 42,
        event_name: "login".to_string(),
        internal_tag: "ignored value".to_string(), // NOT encoded
    };
    let system_event = ComplexRenameEnum::SystemEvent(0xDEAD_BEEF_CAFE_F00D);
    let no_op = ComplexRenameEnum::NoOp;

    for variant in &[user_event, system_event, no_op] {
        let encoded = encode_to_vec(variant).expect("encode complex rename enum");
        let (decoded, bytes_read): (ComplexRenameEnum, usize) =
            decode_from_slice(&encoded).expect("decode complex rename enum");

        // For UserEvent, verify the default_label() is applied to internal_tag.
        if let ComplexRenameEnum::UserEvent {
            ref user_id,
            ref event_name,
            ref internal_tag,
        } = decoded
        {
            assert_eq!(*user_id, 42);
            assert_eq!(event_name.as_str(), "login");
            assert_eq!(
                internal_tag.as_str(),
                "unlabeled",
                "default_label() must return 'unlabeled'"
            );
        } else {
            // For other variants just verify structural equality
            assert_eq!(&decoded, variant);
        }

        assert_eq!(bytes_read, encoded.len());
    }
}
