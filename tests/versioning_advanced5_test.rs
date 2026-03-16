//! Struct versioning and wire format evolution tests for OxiCode.
//!
//! Covers 22 scenarios exercising manual versioning patterns, struct evolution,
//! binary layout stability, and encode/decode backward compatibility using the
//! standard encode/decode API together with `#[oxicode(skip)]`.

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
    config, decode_from_slice, decode_from_slice_with_config, encode_to_vec,
    encode_to_vec_with_config, Decode, Encode,
};
use std::collections::BTreeMap;

// ---------------------------------------------------------------------------
// Shared helper: a version tag value we will manually prepend/verify in tests.
// ---------------------------------------------------------------------------
const WIRE_VERSION_V1: u8 = 0x01;
const WIRE_VERSION_V2: u8 = 0x02;

// ── Structs used across multiple tests ──────────────────────────────────────

#[derive(Debug, PartialEq, Encode, Decode)]
struct RecordV1 {
    id: u32,
    name: String,
}

/// V2 adds an `age` field; old decoders cannot read it directly but V2 can
/// roundtrip with `age` populated.
#[derive(Debug, PartialEq, Encode, Decode)]
struct RecordV2 {
    id: u32,
    name: String,
    age: u16,
}

/// V3 extends V1 with a skipped field — the binary layout is identical to V1.
#[derive(Debug, PartialEq, Encode, Decode)]
struct RecordV3Compat {
    id: u32,
    name: String,
    #[oxicode(skip)]
    derived_score: u64,
}

// ── Scenario 1 ───────────────────────────────────────────────────────────────
// Encode V1, decode as V1 — basic roundtrip sanity.
#[test]
fn test_v1_struct_roundtrip() {
    let rec = RecordV1 {
        id: 1,
        name: "Alice".into(),
    };
    let enc = encode_to_vec(&rec).expect("encode v1");
    let (dec, consumed): (RecordV1, usize) = decode_from_slice(&enc).expect("decode v1");
    assert_eq!(rec, dec);
    assert_eq!(consumed, enc.len());
}

// ── Scenario 2 ───────────────────────────────────────────────────────────────
// V1 bytes decode into RecordV3Compat (skip field gets Default).
// Both structs share identical wire layout because `derived_score` is skipped.
#[test]
fn test_v1_bytes_decode_into_compat_struct_with_skip() {
    let v1 = RecordV1 {
        id: 42,
        name: "Bob".into(),
    };
    let enc = encode_to_vec(&v1).expect("encode v1");

    let (compat, consumed): (RecordV3Compat, usize) =
        decode_from_slice(&enc).expect("decode as RecordV3Compat");
    assert_eq!(compat.id, 42);
    assert_eq!(compat.name, "Bob");
    // Skipped field must be Default (0u64) when decoded from a V1 byte stream.
    assert_eq!(compat.derived_score, 0u64);
    assert_eq!(consumed, enc.len());
}

// ── Scenario 3 ───────────────────────────────────────────────────────────────
// RecordV3Compat encodes to the same bytes as RecordV1 (layout identity).
#[test]
fn test_skip_field_produces_same_layout_as_v1() {
    let v1 = RecordV1 {
        id: 7,
        name: "Carol".into(),
    };
    let v3 = RecordV3Compat {
        id: 7,
        name: "Carol".into(),
        derived_score: 0xDEAD_BEEF_CAFE_1234,
    };
    let enc_v1 = encode_to_vec(&v1).expect("encode v1");
    let enc_v3 = encode_to_vec(&v3).expect("encode v3");
    // Wire bytes must be identical — skip removes the field entirely.
    assert_eq!(enc_v1, enc_v3);
}

// ── Scenario 4 ───────────────────────────────────────────────────────────────
// Manually prepend a version byte (WIRE_VERSION_V1) and read it back,
// demonstrating a custom manual versioning pattern.
#[test]
fn test_manual_version_byte_prepend_and_extract() {
    let rec = RecordV1 {
        id: 100,
        name: "Dave".into(),
    };
    let payload = encode_to_vec(&rec).expect("encode payload");
    let mut versioned: Vec<u8> = Vec::with_capacity(1 + payload.len());
    versioned.push(WIRE_VERSION_V1);
    versioned.extend_from_slice(&payload);

    // Reader side: extract version byte, then decode the rest.
    let version_tag = versioned[0];
    assert_eq!(version_tag, WIRE_VERSION_V1);
    let (decoded, consumed): (RecordV1, usize) =
        decode_from_slice(&versioned[1..]).expect("decode after version byte");
    assert_eq!(decoded, rec);
    assert_eq!(consumed, payload.len());
}

// ── Scenario 5 ───────────────────────────────────────────────────────────────
// Prepend a u32 version tag and verify it round-trips as a wrapper struct.
#[derive(Debug, PartialEq, Encode, Decode)]
struct VersionTaggedRecord {
    schema_version: u32,
    id: u32,
    name: String,
}

#[test]
fn test_u32_version_tag_in_struct_roundtrip() {
    let rec = VersionTaggedRecord {
        schema_version: 1,
        id: 55,
        name: "Eve".into(),
    };
    let enc = encode_to_vec(&rec).expect("encode version-tagged record");
    let (dec, consumed): (VersionTaggedRecord, usize) =
        decode_from_slice(&enc).expect("decode version-tagged record");
    assert_eq!(rec, dec);
    assert_eq!(consumed, enc.len());
    assert_eq!(dec.schema_version, 1u32);
}

// ── Scenario 6 ───────────────────────────────────────────────────────────────
// Binary layout: verify that `id` (u32 varint) occupies the first bytes and
// the string follows with its varint length prefix.
#[test]
fn test_binary_layout_field_order_matches_declaration() {
    let rec = RecordV1 {
        id: 1,
        name: "AB".into(),
    };
    let enc = encode_to_vec(&rec).expect("encode layout test");
    // With standard config (varint): id=1 encodes to [0x01], then
    // string length=2 → [0x02], then bytes 0x41 0x42.
    assert_eq!(enc[0], 0x01, "id=1 must be first byte 0x01");
    assert_eq!(enc[1], 0x02, "string length=2 must be next byte 0x02");
    assert_eq!(&enc[2..4], b"AB", "string bytes must follow length");
    assert_eq!(enc.len(), 4);
}

// ── Scenario 7 ───────────────────────────────────────────────────────────────
// V2 struct round-trips correctly when all fields are populated.
#[test]
fn test_v2_struct_roundtrip_all_fields() {
    let rec = RecordV2 {
        id: 999,
        name: "Frank".into(),
        age: 35,
    };
    let enc = encode_to_vec(&rec).expect("encode v2");
    let (dec, consumed): (RecordV2, usize) = decode_from_slice(&enc).expect("decode v2");
    assert_eq!(rec, dec);
    assert_eq!(consumed, enc.len());
}

// ── Scenario 8 ───────────────────────────────────────────────────────────────
// First N bytes of V2 encoding equal the first N bytes of V1 encoding
// (backward-compatible prefix property), when id and name are equal.
#[test]
fn test_v1_and_v2_share_common_prefix_bytes() {
    let v1 = RecordV1 {
        id: 5,
        name: "Grace".into(),
    };
    let v2 = RecordV2 {
        id: 5,
        name: "Grace".into(),
        age: 0,
    };
    let enc_v1 = encode_to_vec(&v1).expect("encode v1");
    let enc_v2 = encode_to_vec(&v2).expect("encode v2");
    // V1 bytes must be a prefix of V2 bytes.
    assert!(
        enc_v2.starts_with(&enc_v1),
        "V2 encoding must start with the same bytes as V1"
    );
    // V2 must be strictly longer.
    assert!(enc_v2.len() > enc_v1.len());
}

// ── Scenario 9 ───────────────────────────────────────────────────────────────
// Enum variant addition: existing variants decode without change.
#[derive(Debug, PartialEq, Encode, Decode)]
enum EventV1 {
    Created,
    Updated,
}

#[derive(Debug, PartialEq, Encode, Decode)]
enum EventV2 {
    Created,
    Updated,
    Deleted, // new in V2
}

#[test]
fn test_enum_old_variants_encode_decode_stable() {
    let created = EventV1::Created;
    let updated = EventV1::Updated;
    let enc_c = encode_to_vec(&created).expect("encode Created");
    let enc_u = encode_to_vec(&updated).expect("encode Updated");

    // V2 variants at the same ordinal positions must produce identical bytes.
    let v2_created = EventV2::Created;
    let v2_updated = EventV2::Updated;
    let enc_v2_c = encode_to_vec(&v2_created).expect("encode V2 Created");
    let enc_v2_u = encode_to_vec(&v2_updated).expect("encode V2 Updated");

    assert_eq!(
        enc_c, enc_v2_c,
        "Created must have same encoding in V1 and V2"
    );
    assert_eq!(
        enc_u, enc_v2_u,
        "Updated must have same encoding in V1 and V2"
    );
}

// ── Scenario 10 ──────────────────────────────────────────────────────────────
// Version constant encoded as first byte: verify it survives a roundtrip
// inside a wrapper struct.
#[derive(Debug, PartialEq, Encode, Decode)]
struct VersionedFrame {
    version: u8,
    payload_len: u32,
}

#[test]
fn test_version_constant_as_first_byte_in_frame() {
    let frame = VersionedFrame {
        version: WIRE_VERSION_V2,
        payload_len: 128,
    };
    let enc = encode_to_vec(&frame).expect("encode frame");
    // version=2 is a u8 → raw single byte at offset 0.
    assert_eq!(enc[0], WIRE_VERSION_V2);
    let (dec, _): (VersionedFrame, usize) = decode_from_slice(&enc).expect("decode frame");
    assert_eq!(dec, frame);
}

// ── Scenario 11 ──────────────────────────────────────────────────────────────
// Deeply-nested versioned struct roundtrip.
#[derive(Debug, PartialEq, Encode, Decode)]
struct Address {
    city: String,
    zip: u32,
}

#[derive(Debug, PartialEq, Encode, Decode)]
struct PersonV1 {
    name: String,
    address: Address,
}

#[derive(Debug, PartialEq, Encode, Decode)]
struct OrganizationV1 {
    org_name: String,
    leader: PersonV1,
}

#[test]
fn test_deeply_nested_struct_roundtrip() {
    let org = OrganizationV1 {
        org_name: "COOLJAPAN OU".into(),
        leader: PersonV1 {
            name: "Kitasan".into(),
            address: Address {
                city: "Tallinn".into(),
                zip: 10115,
            },
        },
    };
    let enc = encode_to_vec(&org).expect("encode nested org");
    let (dec, consumed): (OrganizationV1, usize) =
        decode_from_slice(&enc).expect("decode nested org");
    assert_eq!(org, dec);
    assert_eq!(consumed, enc.len());
}

// ── Scenario 12 ──────────────────────────────────────────────────────────────
// Custom (wrong) version tag detection: data prefixed with V2 tag is rejected
// by a decoder that only accepts V1, using manual version gating.
#[test]
fn test_custom_version_tag_mismatch_is_detected() {
    let payload = encode_to_vec(&RecordV1 {
        id: 77,
        name: "Henry".into(),
    })
    .expect("encode payload");

    let mut v2_tagged: Vec<u8> = Vec::with_capacity(1 + payload.len());
    v2_tagged.push(WIRE_VERSION_V2);
    v2_tagged.extend_from_slice(&payload);

    let version_tag = v2_tagged[0];
    // Simulate a V1-only decoder that rejects non-V1 tags.
    assert_ne!(version_tag, WIRE_VERSION_V1, "V2 tag must not match V1 tag");
    // Confirm the payload itself is still valid if we skip the version byte.
    let (rec, _): (RecordV1, usize) =
        decode_from_slice(&v2_tagged[1..]).expect("raw payload still valid");
    assert_eq!(rec.id, 77);
}

// ── Scenario 13 ──────────────────────────────────────────────────────────────
// Wire format: field ordering matches struct field declaration order.
// Encode a struct with three fields, verify bytes match expected field-order.
#[derive(Debug, PartialEq, Encode, Decode)]
struct OrderedFields {
    first: u8,
    second: u8,
    third: u8,
}

#[test]
fn test_wire_format_field_ordering_matches_declaration() {
    let val = OrderedFields {
        first: 0xAA,
        second: 0xBB,
        third: 0xCC,
    };
    let enc = encode_to_vec(&val).expect("encode ordered fields");
    // u8 is written as a raw byte, so 3 fields = 3 bytes in declaration order.
    assert_eq!(enc.len(), 3);
    assert_eq!(enc[0], 0xAA, "first field at byte 0");
    assert_eq!(enc[1], 0xBB, "second field at byte 1");
    assert_eq!(enc[2], 0xCC, "third field at byte 2");
}

// ── Scenario 14 ──────────────────────────────────────────────────────────────
// Trailing bytes: decode from a slice with extra trailing bytes succeeds and
// consumed count equals only the encoded portion.
#[test]
fn test_decode_with_trailing_bytes_consumed_is_encoded_portion_only() {
    let rec = RecordV1 {
        id: 3,
        name: "Ida".into(),
    };
    let mut enc = encode_to_vec(&rec).expect("encode for trailing test");
    let original_len = enc.len();
    // Append garbage trailing bytes.
    enc.extend_from_slice(&[0xFF, 0xFE, 0xFD]);
    let (dec, consumed): (RecordV1, usize) = decode_from_slice(&enc).expect("decode with trailing");
    assert_eq!(dec, rec);
    assert_eq!(
        consumed, original_len,
        "consumed must equal encoded portion only"
    );
}

// ── Scenario 15 ──────────────────────────────────────────────────────────────
// Large struct with many fields encodes and decodes correctly.
#[derive(Debug, PartialEq, Encode, Decode)]
struct LargeRecord {
    f00: u64,
    f01: u64,
    f02: u64,
    f03: u64,
    f04: u64,
    f05: u64,
    f06: u64,
    f07: u64,
    f08: String,
    f09: String,
    f10: bool,
    f11: bool,
}

#[test]
fn test_large_struct_many_fields_roundtrip() {
    let rec = LargeRecord {
        f00: 0,
        f01: u64::MAX,
        f02: 12345678901234,
        f03: 9999999999,
        f04: 1,
        f05: 2,
        f06: 3,
        f07: 4,
        f08: "field-eight-value".into(),
        f09: "field-nine-value".into(),
        f10: true,
        f11: false,
    };
    let enc = encode_to_vec(&rec).expect("encode large record");
    let (dec, consumed): (LargeRecord, usize) =
        decode_from_slice(&enc).expect("decode large record");
    assert_eq!(rec, dec);
    assert_eq!(consumed, enc.len());
}

// ── Scenario 16 ──────────────────────────────────────────────────────────────
// Option fields: None in V1 (absent), Some in V2 (present).
#[derive(Debug, PartialEq, Encode, Decode)]
struct RecordWithOpt {
    id: u32,
    label: Option<String>,
}

#[test]
fn test_option_field_none_and_some_roundtrip() {
    let none_rec = RecordWithOpt {
        id: 10,
        label: None,
    };
    let some_rec = RecordWithOpt {
        id: 20,
        label: Some("optlabel".into()),
    };

    let enc_none = encode_to_vec(&none_rec).expect("encode None option");
    let enc_some = encode_to_vec(&some_rec).expect("encode Some option");

    let (dec_none, c1): (RecordWithOpt, usize) =
        decode_from_slice(&enc_none).expect("decode None option");
    let (dec_some, c2): (RecordWithOpt, usize) =
        decode_from_slice(&enc_some).expect("decode Some option");

    assert_eq!(dec_none, none_rec);
    assert_eq!(dec_some, some_rec);
    assert_eq!(c1, enc_none.len());
    assert_eq!(c2, enc_some.len());
    // Option::None must encode smaller than Option::Some("optlabel").
    assert!(enc_none.len() < enc_some.len());
}

// ── Scenario 17 ──────────────────────────────────────────────────────────────
// Different configs (varint vs fixed-int) produce different byte sequences
// for the same value, demonstrating config wire incompatibility.
#[test]
fn test_different_configs_produce_different_byte_sequences() {
    let value: u32 = 300;
    let varint_cfg = config::standard(); // varint by default
    let fixed_cfg = config::standard().with_fixed_int_encoding();

    let enc_varint = encode_to_vec_with_config(&value, varint_cfg).expect("encode varint");
    let enc_fixed = encode_to_vec_with_config(&value, fixed_cfg).expect("encode fixed");

    assert_ne!(
        enc_varint, enc_fixed,
        "varint and fixed-int configs must produce different bytes for u32=300"
    );
    // Fixed u32 is always 4 bytes; varint 300 must be strictly fewer than 4 bytes.
    assert_eq!(enc_fixed.len(), 4, "fixed u32 must be 4 bytes");
    assert!(
        enc_varint.len() < 4,
        "varint 300 must encode in fewer than 4 bytes (got {})",
        enc_varint.len()
    );
}

// ── Scenario 18 ──────────────────────────────────────────────────────────────
// u32 versioned wrapper struct: two structs with the same outer shape but
// different schema_version values produce different bytes.
#[derive(Debug, PartialEq, Encode, Decode)]
struct VersionedWrapperV1 {
    schema_version: u32,
    data: u64,
}

#[derive(Debug, PartialEq, Encode, Decode)]
struct VersionedWrapperV2 {
    schema_version: u32,
    data: u64,
}

#[test]
fn test_versioned_wrapper_different_version_tag_produces_different_bytes() {
    let w1 = VersionedWrapperV1 {
        schema_version: 1,
        data: 0xABCD,
    };
    let w2 = VersionedWrapperV2 {
        schema_version: 2,
        data: 0xABCD,
    };
    let enc1 = encode_to_vec(&w1).expect("encode wrapper v1");
    let enc2 = encode_to_vec(&w2).expect("encode wrapper v2");
    assert_ne!(
        enc1, enc2,
        "different version tags must yield different bytes"
    );
    // Confirm both decode back correctly.
    let (d1, _): (VersionedWrapperV1, usize) = decode_from_slice(&enc1).expect("decode wrapper v1");
    let (d2, _): (VersionedWrapperV2, usize) = decode_from_slice(&enc2).expect("decode wrapper v2");
    assert_eq!(d1.schema_version, 1u32);
    assert_eq!(d2.schema_version, 2u32);
    assert_eq!(d1.data, d2.data);
}

// ── Scenario 19 ──────────────────────────────────────────────────────────────
// Struct with String and Vec fields: verify their encoded offsets.
#[derive(Debug, PartialEq, Encode, Decode)]
struct StringVecRecord {
    tag: u8,
    label: String,
    items: Vec<u8>,
}

#[test]
fn test_string_and_vec_fields_at_correct_offsets() {
    let rec = StringVecRecord {
        tag: 0x05,
        label: "XY".into(), // length=2 → varint [0x02], then [0x58, 0x59]
        items: vec![0x0A, 0x0B, 0x0C], // length=3 → varint [0x03], then 3 bytes
    };
    let enc = encode_to_vec(&rec).expect("encode string-vec record");
    // byte 0: tag = 0x05
    assert_eq!(enc[0], 0x05);
    // byte 1: label length = 2
    assert_eq!(enc[1], 0x02);
    // bytes 2..4: "XY"
    assert_eq!(&enc[2..4], b"XY");
    // byte 4: items length = 3
    assert_eq!(enc[4], 0x03);
    // bytes 5..8: items
    assert_eq!(&enc[5..8], &[0x0A, 0x0B, 0x0C]);
    assert_eq!(enc.len(), 8);
}

// ── Scenario 20 ──────────────────────────────────────────────────────────────
// Encoded length equals the sum of individual field lengths.
#[test]
fn test_encode_length_equals_sum_of_field_lengths() {
    let id: u32 = 7;
    let flag: bool = true;
    let name = "LenCheck".to_string();

    let enc_id = encode_to_vec(&id).expect("encode id");
    let enc_flag = encode_to_vec(&flag).expect("encode flag");
    let enc_name = encode_to_vec(&name).expect("encode name");

    #[derive(Debug, PartialEq, Encode, Decode)]
    struct LenCheckRecord {
        id: u32,
        flag: bool,
        name: String,
    }

    let rec = LenCheckRecord {
        id,
        flag,
        name: name.clone(),
    };
    let enc_whole = encode_to_vec(&rec).expect("encode whole record");
    assert_eq!(
        enc_whole.len(),
        enc_id.len() + enc_flag.len() + enc_name.len(),
        "whole struct length must equal sum of field lengths"
    );
}

// ── Scenario 21 ──────────────────────────────────────────────────────────────
// BTreeMap with string keys version compatibility: two versions of a config
// struct embed a BTreeMap and roundtrip independently.
#[derive(Debug, PartialEq, Encode, Decode)]
struct ConfigMapV1 {
    version: u8,
    settings: BTreeMap<String, String>,
}

#[derive(Debug, PartialEq, Encode, Decode)]
struct ConfigMapV2 {
    version: u8,
    settings: BTreeMap<String, String>,
    #[oxicode(skip)]
    cached_checksum: u32,
}

#[test]
fn test_btreemap_string_keys_version_compat_roundtrip() {
    let mut settings_v1 = BTreeMap::new();
    settings_v1.insert("timeout".into(), "30".into());
    settings_v1.insert("retries".into(), "3".into());

    let cfg_v1 = ConfigMapV1 {
        version: 1,
        settings: settings_v1.clone(),
    };
    let enc_v1 = encode_to_vec(&cfg_v1).expect("encode ConfigMapV1");

    // Decode V1 bytes as ConfigMapV2 (skip field gets default = 0).
    let (dec_v2, consumed): (ConfigMapV2, usize) =
        decode_from_slice(&enc_v1).expect("decode V1 bytes as ConfigMapV2");
    assert_eq!(dec_v2.version, 1u8);
    assert_eq!(dec_v2.settings, settings_v1);
    assert_eq!(dec_v2.cached_checksum, 0u32, "skipped field defaults to 0");
    assert_eq!(consumed, enc_v1.len());
}

// ── Scenario 22 ──────────────────────────────────────────────────────────────
// Roundtrip that verifies consumed bytes == total encoded bytes for a complex struct.
#[derive(Debug, PartialEq, Encode, Decode)]
struct ComplexRecord {
    id: u64,
    tags: Vec<String>,
    score: f64,
    active: bool,
    meta: BTreeMap<String, u32>,
}

#[test]
fn test_complex_struct_consumed_equals_total_encoded_bytes() {
    let mut meta = BTreeMap::new();
    meta.insert("priority".into(), 10u32);
    meta.insert("weight".into(), 42u32);

    let rec = ComplexRecord {
        id: 9876543210,
        tags: vec!["alpha".into(), "beta".into(), "gamma".into()],
        score: std::f64::consts::PI,
        active: true,
        meta,
    };

    let enc = encode_to_vec(&rec).expect("encode complex record");
    let (dec, consumed): (ComplexRecord, usize) =
        decode_from_slice(&enc).expect("decode complex record");

    assert_eq!(rec, dec);
    assert_eq!(
        consumed,
        enc.len(),
        "consumed bytes must equal total encoded bytes"
    );

    // Also verify with fixed-int config produces a larger buffer (all ints fixed-width).
    let fixed_cfg = config::standard().with_fixed_int_encoding();
    let enc_fixed = encode_to_vec_with_config(&rec, fixed_cfg).expect("encode fixed-int");
    let (dec_fixed, consumed_fixed): (ComplexRecord, usize) =
        decode_from_slice_with_config(&enc_fixed, fixed_cfg).expect("decode fixed-int");
    assert_eq!(rec, dec_fixed);
    assert_eq!(consumed_fixed, enc_fixed.len());
    // Fixed-int encoding must be >= varint encoding for this struct.
    assert!(enc_fixed.len() >= enc.len());
}
