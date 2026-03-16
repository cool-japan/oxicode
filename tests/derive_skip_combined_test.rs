//! 18 tests covering combinations of `#[oxicode(skip)]` with other derive attributes.
//!
//! Covers:
//!   1.  Skip on first field of struct
//!   2.  Skip on last field of struct
//!   3.  Skip on middle field(s)
//!   4.  Skip on all fields (empty encoding, all Default on decode)
//!   5.  Skip + rename on same field (skip takes precedence)
//!   6.  Skip in tuple struct
//!   7.  Skip in enum struct variant fields
//!   8.  Multiple structs chained (A contains B, B has skipped field)
//!   9.  Schema migration: V1 encodes without field, V2 decodes with skip+default
//!   10. Skip on Vec<u8> field
//!   11. Skip on Option<String> field (restores None)
//!   12. Skip on nested struct field (restores Default)
//!   13. Struct where only non-skipped fields affect byte count
//!   14. Encode → decode cycle: verify skipped fields get default value
//!   15. Skip on u128 field
//!   16. Skip combined with seq_len attribute on other fields
//!   17. Skip on enum unit variant (variant-level skip)
//!   18. Verify encoded bytes of struct-with-skip match struct-without-that-field

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

// ── Test 1: Skip on first field ───────────────────────────────────────────────

#[derive(Debug, PartialEq, Encode, Decode)]
struct SkipFirstField {
    #[oxicode(skip)]
    first: u32,
    second: String,
    third: u64,
}

#[test]
fn test_01_skip_on_first_field() {
    let original = SkipFirstField {
        first: 0xDEAD_BEEF,
        second: "hello".to_string(),
        third: 42,
    };
    let enc = encode_to_vec(&original).expect("encode");
    let (dec, bytes_read): (SkipFirstField, _) = decode_from_slice(&enc).expect("decode");
    assert_eq!(dec.first, 0u32, "skipped first field must be Default (0)");
    assert_eq!(dec.second, "hello");
    assert_eq!(dec.third, 42);
    assert_eq!(bytes_read, enc.len());
}

// ── Test 2: Skip on last field ────────────────────────────────────────────────

#[derive(Debug, PartialEq, Encode, Decode)]
struct SkipLastField {
    alpha: u32,
    beta: String,
    #[oxicode(skip)]
    gamma: bool,
}

#[test]
fn test_02_skip_on_last_field() {
    let original = SkipLastField {
        alpha: 100,
        beta: "last-skip".to_string(),
        gamma: true,
    };
    let enc = encode_to_vec(&original).expect("encode");
    let (dec, bytes_read): (SkipLastField, _) = decode_from_slice(&enc).expect("decode");
    assert_eq!(dec.alpha, 100);
    assert_eq!(dec.beta, "last-skip");
    assert!(!dec.gamma, "skipped last field must be Default (false)");
    assert_eq!(bytes_read, enc.len());
}

// ── Test 3: Skip on middle field(s) ──────────────────────────────────────────

#[derive(Debug, PartialEq, Encode, Decode)]
struct SkipMiddleFields {
    before: u32,
    #[oxicode(skip)]
    middle_a: String,
    between: u16,
    #[oxicode(skip)]
    middle_b: u64,
    after: u32,
}

#[test]
fn test_03_skip_on_middle_fields() {
    let original = SkipMiddleFields {
        before: 111,
        middle_a: "should-not-encode".to_string(),
        between: 7,
        middle_b: 0xFFFF_FFFF_FFFF,
        after: 222,
    };
    let enc = encode_to_vec(&original).expect("encode");
    let (dec, bytes_read): (SkipMiddleFields, _) = decode_from_slice(&enc).expect("decode");
    assert_eq!(dec.before, 111);
    assert_eq!(
        dec.middle_a,
        String::new(),
        "skipped middle_a must be Default"
    );
    assert_eq!(dec.between, 7);
    assert_eq!(dec.middle_b, 0u64, "skipped middle_b must be Default");
    assert_eq!(dec.after, 222);
    assert_eq!(bytes_read, enc.len());
}

// ── Test 4: Skip on all fields ────────────────────────────────────────────────

#[derive(Debug, PartialEq, Encode, Decode)]
struct AllFieldsSkipped {
    #[oxicode(skip)]
    a: u32,
    #[oxicode(skip)]
    b: String,
    #[oxicode(skip)]
    c: bool,
    #[oxicode(skip)]
    d: Vec<u8>,
}

#[test]
fn test_04_skip_all_fields_empty_encoding() {
    let original = AllFieldsSkipped {
        a: 99,
        b: "ignored".to_string(),
        c: true,
        d: vec![1, 2, 3],
    };
    let enc = encode_to_vec(&original).expect("encode");
    assert!(
        enc.is_empty(),
        "all-skipped struct must produce 0 bytes, got {}",
        enc.len()
    );
    let (dec, bytes_read): (AllFieldsSkipped, _) = decode_from_slice(&enc).expect("decode");
    assert_eq!(dec.a, 0u32);
    assert_eq!(dec.b, String::new());
    assert!(!dec.c);
    assert!(dec.d.is_empty());
    assert_eq!(bytes_read, 0);
}

// ── Test 5: Skip + rename on same field (skip takes precedence) ───────────────

#[derive(Debug, PartialEq, Encode, Decode)]
struct SkipAndRename {
    id: u32,
    #[oxicode(skip, rename = "firstName")]
    first_name: String,
    active: bool,
}

#[test]
fn test_05_skip_with_rename_skip_takes_precedence() {
    let original = SkipAndRename {
        id: 7,
        first_name: "Alice".to_string(),
        active: true,
    };
    let enc = encode_to_vec(&original).expect("encode");
    let (dec, bytes_read): (SkipAndRename, _) = decode_from_slice(&enc).expect("decode");
    assert_eq!(dec.id, 7);
    // skip takes precedence over rename — field is not in the wire format
    assert_eq!(
        dec.first_name,
        String::new(),
        "skipped+renamed field must be Default"
    );
    assert!(dec.active);
    assert_eq!(bytes_read, enc.len());
}

// ── Test 6: Skip in tuple struct ──────────────────────────────────────────────

#[derive(Debug, PartialEq, Encode, Decode)]
struct TupleWithSkip(
    u32,
    #[oxicode(skip)] u64,
    String,
    #[oxicode(skip)] bool,
    u16,
);

#[test]
fn test_06_skip_in_tuple_struct() {
    let original = TupleWithSkip(10, 0xFFFF_FFFF, "hello".to_string(), true, 55);
    let enc = encode_to_vec(&original).expect("encode");
    let (dec, bytes_read): (TupleWithSkip, _) = decode_from_slice(&enc).expect("decode");
    assert_eq!(dec.0, 10);
    assert_eq!(dec.1, 0u64, "skipped tuple field 1 must be Default");
    assert_eq!(dec.2, "hello");
    assert!(!dec.3, "skipped tuple field 3 must be Default");
    assert_eq!(dec.4, 55);
    assert_eq!(bytes_read, enc.len());
}

// ── Test 7: Skip in enum struct variant fields ────────────────────────────────

#[derive(Debug, PartialEq, Encode, Decode)]
enum CommandEvent {
    Login {
        user_id: u32,
        #[oxicode(skip)]
        session_token: u64,
        role: String,
    },
    Logout {
        user_id: u32,
    },
}

#[test]
fn test_07_skip_in_enum_struct_variant_fields() {
    let original = CommandEvent::Login {
        user_id: 42,
        session_token: 0xDEAD_CAFE_BABE,
        role: "admin".to_string(),
    };
    let enc = encode_to_vec(&original).expect("encode");
    let (dec, bytes_read): (CommandEvent, _) = decode_from_slice(&enc).expect("decode");
    match dec {
        CommandEvent::Login {
            user_id,
            session_token,
            role,
        } => {
            assert_eq!(user_id, 42);
            assert_eq!(
                session_token, 0u64,
                "skipped session_token must be Default (0)"
            );
            assert_eq!(role, "admin");
        }
        other => panic!("expected CommandEvent::Login, got {:?}", other),
    }
    assert_eq!(bytes_read, enc.len());
}

// ── Test 8: Multiple structs chained (A contains B, B has skipped field) ──────

#[derive(Debug, PartialEq, Encode, Decode)]
struct Inner8 {
    value: u32,
    #[oxicode(skip)]
    cache: u64,
}

#[derive(Debug, PartialEq, Encode, Decode)]
struct Outer8 {
    label: String,
    inner: Inner8,
    count: u32,
}

#[test]
fn test_08_chained_structs_with_skipped_nested_field() {
    let original = Outer8 {
        label: "outer".to_string(),
        inner: Inner8 {
            value: 77,
            cache: 0xCAFE_BABE_DEAD,
        },
        count: 3,
    };
    let enc = encode_to_vec(&original).expect("encode");
    let (dec, bytes_read): (Outer8, _) = decode_from_slice(&enc).expect("decode");
    assert_eq!(dec.label, "outer");
    assert_eq!(dec.inner.value, 77);
    assert_eq!(
        dec.inner.cache, 0u64,
        "nested skipped field must be Default"
    );
    assert_eq!(dec.count, 3);
    assert_eq!(bytes_read, enc.len());
}

// ── Test 9: Schema migration (V1 encodes without field, V2 decodes with skip) ─

#[derive(Debug, PartialEq, Encode, Decode)]
struct RecordSchemaV1 {
    id: u32,
    name: String,
}

fn default_status() -> String {
    "active".to_string()
}

#[derive(Debug, PartialEq, Encode, Decode)]
struct RecordSchemaV2 {
    id: u32,
    name: String,
    #[oxicode(default = "default_status")]
    status: String,
}

#[test]
fn test_09_schema_migration_v1_bytes_decoded_by_v2() {
    let v1 = RecordSchemaV1 {
        id: 10,
        name: "legacy".to_string(),
    };
    let v1_bytes = encode_to_vec(&v1).expect("v1 encode");

    // V2 decodes V1 bytes: `status` was not in the stream, so default_status() is used.
    let (v2, bytes_read): (RecordSchemaV2, _) =
        decode_from_slice(&v1_bytes).expect("decode v1 bytes with v2 schema");
    assert_eq!(v2.id, 10);
    assert_eq!(v2.name, "legacy");
    assert_eq!(
        v2.status, "active",
        "status must come from default_status()"
    );
    assert_eq!(bytes_read, v1_bytes.len());
}

// ── Test 10: Skip on Vec<u8> field ────────────────────────────────────────────

#[derive(Debug, PartialEq, Encode, Decode)]
struct SkipVecU8 {
    id: u32,
    #[oxicode(skip)]
    blob: Vec<u8>,
    tag: String,
}

#[test]
fn test_10_skip_on_vec_u8_field() {
    let original = SkipVecU8 {
        id: 5,
        blob: vec![0xDE, 0xAD, 0xBE, 0xEF, 0x00, 0xFF],
        tag: "binary".to_string(),
    };
    let enc = encode_to_vec(&original).expect("encode");
    let (dec, bytes_read): (SkipVecU8, _) = decode_from_slice(&enc).expect("decode");
    assert_eq!(dec.id, 5);
    assert!(
        dec.blob.is_empty(),
        "skipped Vec<u8> must be Default (empty vec)"
    );
    assert_eq!(dec.tag, "binary");
    assert_eq!(bytes_read, enc.len());
}

// ── Test 11: Skip on Option<String> field (restores None) ────────────────────

#[derive(Debug, PartialEq, Encode, Decode)]
struct SkipOptionString {
    id: u32,
    #[oxicode(skip)]
    metadata: Option<String>,
    value: u64,
}

#[test]
fn test_11_skip_on_option_string_restores_none() {
    let original = SkipOptionString {
        id: 3,
        metadata: Some("rich data".to_string()),
        value: 9999,
    };
    let enc = encode_to_vec(&original).expect("encode");
    let (dec, bytes_read): (SkipOptionString, _) = decode_from_slice(&enc).expect("decode");
    assert_eq!(dec.id, 3);
    assert_eq!(
        dec.metadata, None,
        "skipped Option<String> must be Default (None)"
    );
    assert_eq!(dec.value, 9999);
    assert_eq!(bytes_read, enc.len());
}

// ── Test 12: Skip on nested struct field (restores Default) ───────────────────

#[derive(Debug, PartialEq, Encode, Decode, Default)]
struct NestedInner {
    x: u32,
    y: u32,
}

#[derive(Debug, PartialEq, Encode, Decode)]
struct SkipNestedStruct {
    id: u32,
    #[oxicode(skip)]
    inner: NestedInner,
    label: String,
}

#[test]
fn test_12_skip_on_nested_struct_restores_default() {
    let original = SkipNestedStruct {
        id: 55,
        inner: NestedInner { x: 100, y: 200 },
        label: "outer".to_string(),
    };
    let enc = encode_to_vec(&original).expect("encode");
    let (dec, bytes_read): (SkipNestedStruct, _) = decode_from_slice(&enc).expect("decode");
    assert_eq!(dec.id, 55);
    // Default::default() for NestedInner gives x=0, y=0
    assert_eq!(
        dec.inner,
        NestedInner { x: 0, y: 0 },
        "skipped nested struct must be Default"
    );
    assert_eq!(dec.label, "outer");
    assert_eq!(bytes_read, enc.len());
}

// ── Test 13: Struct where only non-skipped fields affect byte count ───────────

#[derive(Encode)]
struct OnlyRealFields13 {
    real_a: u32,
    real_c: String,
}

#[derive(Debug, PartialEq, Encode, Decode)]
struct SkipManyForByteCount {
    real_a: u32,
    #[oxicode(skip)]
    skip_b: u32,
    real_c: String,
    #[oxicode(skip)]
    skip_d: bool,
    #[oxicode(skip)]
    skip_e: u64,
}

#[test]
fn test_13_only_non_skipped_fields_affect_byte_count() {
    let v = SkipManyForByteCount {
        real_a: 1,
        skip_b: 999,
        real_c: "kept".to_string(),
        skip_d: true,
        skip_e: 0xFFFF_FFFF,
    };
    let enc = encode_to_vec(&v).expect("encode SkipManyForByteCount");
    let expected_enc = encode_to_vec(&OnlyRealFields13 {
        real_a: 1,
        real_c: "kept".to_string(),
    })
    .expect("encode OnlyRealFields13");
    assert_eq!(
        enc, expected_enc,
        "only real fields should appear in the binary wire format"
    );
}

// ── Test 14: Encode → decode cycle: skipped fields get default value ──────────

#[derive(Debug, PartialEq, Encode, Decode)]
struct EncodeDecodeCycle {
    count: u32,
    #[oxicode(skip)]
    temp_cache: u64,
    label: String,
    #[oxicode(skip)]
    flags: u8,
}

#[test]
fn test_14_encode_decode_cycle_skipped_fields_get_default() {
    let original = EncodeDecodeCycle {
        count: 42,
        temp_cache: 0xDEAD_BEEF_DEAD_BEEF,
        label: "cycle".to_string(),
        flags: 0xFF,
    };
    let enc = encode_to_vec(&original).expect("encode");
    let (dec, bytes_read): (EncodeDecodeCycle, _) = decode_from_slice(&enc).expect("decode");
    // Non-skipped fields are preserved
    assert_eq!(dec.count, 42);
    assert_eq!(dec.label, "cycle");
    // Skipped fields must be Default::default()
    assert_eq!(dec.temp_cache, 0u64, "skipped temp_cache must be 0");
    assert_eq!(dec.flags, 0u8, "skipped flags must be 0");
    assert_eq!(bytes_read, enc.len());
}

// ── Test 15: Skip on u128 field ───────────────────────────────────────────────

#[derive(Debug, PartialEq, Encode, Decode)]
struct SkipU128Field {
    id: u32,
    #[oxicode(skip)]
    big_number: u128,
    name: String,
}

#[test]
fn test_15_skip_on_u128_field() {
    let original = SkipU128Field {
        id: 9,
        big_number: u128::MAX,
        name: "large".to_string(),
    };
    let enc = encode_to_vec(&original).expect("encode");
    let (dec, bytes_read): (SkipU128Field, _) = decode_from_slice(&enc).expect("decode");
    assert_eq!(dec.id, 9);
    assert_eq!(dec.big_number, 0u128, "skipped u128 must be Default (0)");
    assert_eq!(dec.name, "large");
    assert_eq!(bytes_read, enc.len());
}

// ── Test 16: Skip combined with seq_len attribute on other fields ─────────────

#[derive(Debug, PartialEq, Encode, Decode)]
struct SkipWithSeqLenOtherField {
    #[oxicode(seq_len = "u16")]
    items: Vec<u32>,
    #[oxicode(skip)]
    transient: u64,
    label: String,
}

#[test]
fn test_16_skip_combined_with_seq_len_on_other_fields() {
    let original = SkipWithSeqLenOtherField {
        items: vec![10, 20, 30, 40, 50],
        transient: 999_999,
        label: "seq-skip".to_string(),
    };
    let enc = encode_to_vec(&original).expect("encode");
    let (dec, bytes_read): (SkipWithSeqLenOtherField, _) = decode_from_slice(&enc).expect("decode");
    assert_eq!(dec.items, vec![10u32, 20, 30, 40, 50]);
    assert_eq!(dec.transient, 0u64, "skipped transient must be Default (0)");
    assert_eq!(dec.label, "seq-skip");
    assert_eq!(bytes_read, enc.len());
}

// ── Test 17: Skip on enum unit variant (variant-level skip) ───────────────────
//
// `#[oxicode(skip)]` on a variant causes it to share the discriminant of its
// nearest non-skipped successor.  Encoding the skipped variant produces the
// same bytes as encoding the successor, and decoding always yields the successor.

#[derive(Debug, PartialEq, Encode, Decode)]
enum ActionKind {
    Start,
    #[oxicode(skip)]
    Deprecated,
    Stop,
}

#[test]
fn test_17_skip_on_enum_unit_variant() {
    let bytes_deprecated = encode_to_vec(&ActionKind::Deprecated).expect("encode Deprecated");
    let bytes_stop = encode_to_vec(&ActionKind::Stop).expect("encode Stop");

    // Skipped variant must encode as its nearest non-skipped successor (Stop).
    assert_eq!(
        bytes_deprecated, bytes_stop,
        "encoding the skipped variant must produce the same bytes as its successor"
    );

    // Decoding the bytes yields the successor, not the skipped variant.
    let (decoded, bytes_read): (ActionKind, _) =
        decode_from_slice(&bytes_deprecated).expect("decode");
    assert_eq!(
        decoded,
        ActionKind::Stop,
        "decoding bytes of the skipped variant must yield its successor"
    );
    assert_eq!(bytes_read, bytes_deprecated.len());

    // Non-skipped variants still round-trip normally.
    let enc_start = encode_to_vec(&ActionKind::Start).expect("encode Start");
    let (dec_start, _): (ActionKind, _) = decode_from_slice(&enc_start).expect("decode Start");
    assert_eq!(dec_start, ActionKind::Start);

    let enc_stop = encode_to_vec(&ActionKind::Stop).expect("encode Stop");
    let (dec_stop, _): (ActionKind, _) = decode_from_slice(&enc_stop).expect("decode Stop");
    assert_eq!(dec_stop, ActionKind::Stop);
}

// ── Test 18: Encoded bytes of struct-with-skip match struct-without-that-field ─

#[derive(Debug, PartialEq, Encode, Decode)]
struct WithSkipField18 {
    x: u32,
    #[oxicode(skip)]
    skipped: u64,
    y: String,
}

#[derive(Debug, PartialEq, Encode, Decode)]
struct WithoutSkipField18 {
    x: u32,
    y: String,
}

#[test]
fn test_18_encoded_bytes_match_struct_without_skipped_field() {
    let with_skip = WithSkipField18 {
        x: 123,
        skipped: u64::MAX, // this value must NOT appear in the wire format
        y: "compare".to_string(),
    };
    let without_skip = WithoutSkipField18 {
        x: 123,
        y: "compare".to_string(),
    };

    let enc_with = encode_to_vec(&with_skip).expect("encode with skip");
    let enc_without = encode_to_vec(&without_skip).expect("encode without skip");

    assert_eq!(
        enc_with, enc_without,
        "struct-with-skip wire bytes must equal struct-without-that-field wire bytes"
    );

    // Decode back through the skip-carrying type to verify round-trip correctness.
    let (dec, bytes_read): (WithSkipField18, _) = decode_from_slice(&enc_with).expect("decode");
    assert_eq!(dec.x, 123);
    assert_eq!(
        dec.skipped, 0u64,
        "skipped field must be Default after decode"
    );
    assert_eq!(dec.y, "compare");
    assert_eq!(bytes_read, enc_with.len());
}
