//! Advanced derive macro usage tests for structs — 22 comprehensive scenarios.
//!
//! Covers: rename, skip on optional fields, default fn, bytes attribute, transparent,
//! flatten, encode_with/decode_with, all-renamed fields, skip+default combo, seq_len,
//! newtype around HashMap/BTreeMap, generic Vec<T>, const-generic array, wide tuple
//! struct, unit struct zero bytes, crate path override, long field names, underscore
//! field names, field-order encoding, nested Vec<Option<String>>, doc-comment fields.

#![allow(dead_code)]
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
use std::collections::{BTreeMap, HashMap};

// ============================================================================
// Module-level custom codec helpers for test 7
// ============================================================================

mod codec_negate_i32 {
    use oxicode::{
        de::{Decode, Decoder},
        enc::{Encode, Encoder},
        Error,
    };

    /// Encodes an `i32` as its arithmetic negation on the wire.
    pub fn encode<E: Encoder>(value: &i32, encoder: &mut E) -> Result<(), Error> {
        value.wrapping_neg().encode(encoder)
    }

    /// Decodes a negated `i32` and re-negates it to recover the original.
    pub fn decode<D: Decoder<Context = ()>>(decoder: &mut D) -> Result<i32, Error> {
        let v = i32::decode(decoder)?;
        Ok(v.wrapping_neg())
    }
}

// ============================================================================
// Test 1 — rename attribute (wire format uses new name as annotation; positional
//           encoding is unchanged, but round-trip must succeed without warnings)
// ============================================================================

#[derive(Debug, PartialEq, Encode, Decode)]
struct WithRename {
    #[oxicode(rename = "userId")]
    user_id: u64,
    #[oxicode(rename = "displayName")]
    display_name: String,
}

#[test]
fn test_rename_field_roundtrip() {
    let original = WithRename {
        user_id: 42,
        display_name: "Alice".to_string(),
    };
    let encoded = encode_to_vec(&original).expect("encode WithRename");
    let (decoded, bytes_read): (WithRename, _) =
        decode_from_slice(&encoded).expect("decode WithRename");
    assert_eq!(original, decoded);
    assert_eq!(bytes_read, encoded.len(), "all bytes consumed");

    // Verify the encoding is byte-for-byte identical to a plain struct without rename
    // (rename is a no-op on the wire — purely metadata).
    #[derive(Encode)]
    struct Plain {
        user_id: u64,
        display_name: String,
    }
    let plain = Plain {
        user_id: 42,
        display_name: "Alice".to_string(),
    };
    let plain_enc = encode_to_vec(&plain).expect("encode Plain");
    assert_eq!(
        encoded, plain_enc,
        "rename must not change wire bytes (positional encoding)"
    );
}

// ============================================================================
// Test 2 — skip on an Option<String> field (skipped field not in stream)
// ============================================================================

#[derive(Debug, PartialEq, Encode, Decode)]
struct WithSkipOptional {
    id: u32,
    #[oxicode(skip)]
    cache_hint: Option<String>,
    value: f64,
}

#[test]
fn test_skip_optional_field_absent_from_stream() {
    let original = WithSkipOptional {
        id: 7,
        cache_hint: Some("hot".to_string()),
        value: std::f64::consts::PI,
    };
    let encoded = encode_to_vec(&original).expect("encode WithSkipOptional");

    // A plain struct without cache_hint must encode to the same bytes,
    // proving cache_hint never reaches the wire.
    #[derive(Encode)]
    struct PlainNoOpt {
        id: u32,
        value: f64,
    }
    let plain = PlainNoOpt {
        id: 7,
        value: std::f64::consts::PI,
    };
    let plain_enc = encode_to_vec(&plain).expect("encode PlainNoOpt");
    assert_eq!(
        encoded, plain_enc,
        "skipped Option<String> must be absent from the binary stream"
    );

    let (decoded, _): (WithSkipOptional, _) =
        decode_from_slice(&encoded).expect("decode WithSkipOptional");
    assert_eq!(decoded.id, 7);
    assert_eq!(decoded.cache_hint, None, "skipped field decodes as None");
    assert_eq!(decoded.value, std::f64::consts::PI);
}

// ============================================================================
// Test 3 — default = "fn_path" applied on decode
// ============================================================================

fn default_pi_string() -> String {
    format!("{:.6}", std::f64::consts::PI)
}

#[derive(Debug, PartialEq, Encode, Decode)]
struct WithDefaultFn {
    name: String,
    #[oxicode(default = "default_pi_string")]
    pi_label: String,
    count: u32,
}

#[test]
fn test_default_fn_applied_on_decode() {
    let original = WithDefaultFn {
        name: "sensor".to_string(),
        pi_label: "overridden".to_string(), // not encoded
        count: 99,
    };
    let encoded = encode_to_vec(&original).expect("encode WithDefaultFn");
    let (decoded, _): (WithDefaultFn, _) =
        decode_from_slice(&encoded).expect("decode WithDefaultFn");

    assert_eq!(decoded.name, "sensor");
    // pi_label was skipped on encode; default_pi_string() is called on decode
    assert_eq!(
        decoded.pi_label,
        format!("{:.6}", std::f64::consts::PI),
        "default fn should produce PI string"
    );
    assert_eq!(decoded.count, 99);
}

// ============================================================================
// Test 4 — #[oxicode(bytes)] on Vec<u8> field
// ============================================================================

#[derive(Debug, PartialEq, Encode, Decode)]
struct WithBytesAttr {
    tag: u8,
    #[oxicode(bytes)]
    payload: Vec<u8>,
}

#[test]
fn test_bytes_attribute_on_vec_u8() {
    let original = WithBytesAttr {
        tag: 0xAB,
        payload: vec![0x00, 0x01, 0xFE, 0xFF, 0x80, 0x7F],
    };
    let encoded = encode_to_vec(&original).expect("encode WithBytesAttr");
    let (decoded, bytes_read): (WithBytesAttr, _) =
        decode_from_slice(&encoded).expect("decode WithBytesAttr");
    assert_eq!(original, decoded);
    assert_eq!(bytes_read, encoded.len());

    // Empty payload also round-trips
    let empty = WithBytesAttr {
        tag: 0x00,
        payload: vec![],
    };
    let enc2 = encode_to_vec(&empty).expect("encode empty WithBytesAttr");
    let (dec2, _): (WithBytesAttr, _) =
        decode_from_slice(&enc2).expect("decode empty WithBytesAttr");
    assert_eq!(empty, dec2);
}

// ============================================================================
// Test 5 — #[oxicode(transparent)] newtype wrapper
// ============================================================================

#[derive(Debug, PartialEq, Encode, Decode)]
#[oxicode(transparent)]
struct Score(u32);

#[test]
fn test_transparent_newtype_identical_bytes() {
    let score = Score(1000);
    let enc_score = encode_to_vec(&score).expect("encode Score");
    let enc_raw = encode_to_vec(&1000u32).expect("encode u32");
    assert_eq!(
        enc_score, enc_raw,
        "transparent newtype must produce identical bytes to the inner type"
    );

    let (decoded, _): (Score, _) = decode_from_slice(&enc_score).expect("decode Score");
    assert_eq!(score, decoded);
}

// ============================================================================
// Test 6 — #[oxicode(flatten)] embeds inner struct fields into outer stream
// ============================================================================

#[derive(Debug, PartialEq, Encode, Decode)]
struct Coords {
    lat: f64,
    lon: f64,
}

#[derive(Debug, PartialEq, Encode, Decode)]
struct Location {
    #[oxicode(flatten)]
    position: Coords,
    altitude_m: i32,
}

#[test]
fn test_flatten_embedded_fields_binary_compatible() {
    let loc = Location {
        position: Coords {
            lat: std::f64::consts::PI / 4.0,
            lon: std::f64::consts::E,
        },
        altitude_m: 850,
    };
    let enc_loc = encode_to_vec(&loc).expect("encode Location");

    // A flat struct with the same fields in the same order must produce identical bytes.
    #[derive(Encode)]
    struct Flat {
        lat: f64,
        lon: f64,
        altitude_m: i32,
    }
    let flat = Flat {
        lat: std::f64::consts::PI / 4.0,
        lon: std::f64::consts::E,
        altitude_m: 850,
    };
    let enc_flat = encode_to_vec(&flat).expect("encode Flat");
    assert_eq!(
        enc_loc, enc_flat,
        "flatten must produce same bytes as manually inlined fields"
    );

    let (decoded, _): (Location, _) = decode_from_slice(&enc_loc).expect("decode Location");
    assert_eq!(loc, decoded);
}

// ============================================================================
// Test 7 — encode_with + decode_with custom codecs
// ============================================================================

#[derive(Debug, PartialEq, Encode, Decode)]
struct WithCustomCodec {
    label: String,
    #[oxicode(
        encode_with = "codec_negate_i32::encode",
        decode_with = "codec_negate_i32::decode"
    )]
    value: i32,
}

#[test]
fn test_encode_with_decode_with_custom_roundtrip() {
    let original = WithCustomCodec {
        label: "negate".to_string(),
        value: -42,
    };
    let encoded = encode_to_vec(&original).expect("encode WithCustomCodec");
    let (decoded, _): (WithCustomCodec, _) =
        decode_from_slice(&encoded).expect("decode WithCustomCodec");
    assert_eq!(
        original, decoded,
        "encode_with/decode_with roundtrip must recover original value"
    );

    // Verify the wire carries the negated value: encode 42 directly and compare.
    #[derive(Encode)]
    struct Plain {
        label: String,
        value: i32,
    }
    let negated_plain = Plain {
        label: "negate".to_string(),
        value: 42, // wire carries negation of -42, i.e. +42
    };
    let plain_enc = encode_to_vec(&negated_plain).expect("encode negated plain");
    assert_eq!(
        encoded, plain_enc,
        "encode_with(negate) should store the arithmetic negation of the value"
    );
}

// ============================================================================
// Test 8 — all fields have explicit #[oxicode(rename)]
// ============================================================================

#[derive(Debug, PartialEq, Encode, Decode)]
struct AllRenamed {
    #[oxicode(rename = "fieldA")]
    field_a: u8,
    #[oxicode(rename = "fieldB")]
    field_b: u16,
    #[oxicode(rename = "fieldC")]
    field_c: u32,
    #[oxicode(rename = "fieldD")]
    field_d: u64,
}

#[test]
fn test_all_fields_renamed_still_roundtrip() {
    let original = AllRenamed {
        field_a: 1,
        field_b: 2,
        field_c: 3,
        field_d: 4,
    };
    let encoded = encode_to_vec(&original).expect("encode AllRenamed");
    let (decoded, _): (AllRenamed, _) = decode_from_slice(&encoded).expect("decode AllRenamed");
    assert_eq!(original, decoded);

    // Wire format must match a plain struct (rename is purely metadata)
    #[derive(Encode)]
    struct PlainAll {
        field_a: u8,
        field_b: u16,
        field_c: u32,
        field_d: u64,
    }
    let plain_enc = encode_to_vec(&PlainAll {
        field_a: 1,
        field_b: 2,
        field_c: 3,
        field_d: 4,
    })
    .expect("encode PlainAll");
    assert_eq!(encoded, plain_enc);
}

// ============================================================================
// Test 9 — both skip and default fields in the same struct
// ============================================================================

fn default_e_value() -> f64 {
    std::f64::consts::E
}

#[derive(Debug, PartialEq, Encode, Decode)]
struct SkipAndDefault {
    id: u64,
    #[oxicode(skip)]
    transient: bool,
    name: String,
    #[oxicode(default = "default_e_value")]
    e_const: f64,
    active: bool,
}

#[test]
fn test_skip_and_default_combined() {
    let original = SkipAndDefault {
        id: 1234,
        transient: true, // skipped; not in stream
        name: "test".to_string(),
        e_const: 0.0, // not encoded; default_e_value() applied on decode
        active: true,
    };
    let encoded = encode_to_vec(&original).expect("encode SkipAndDefault");
    let (decoded, _): (SkipAndDefault, _) =
        decode_from_slice(&encoded).expect("decode SkipAndDefault");

    assert_eq!(decoded.id, 1234);
    assert!(!decoded.transient, "skipped bool defaults to false");
    assert_eq!(decoded.name, "test");
    assert_eq!(
        decoded.e_const,
        std::f64::consts::E,
        "default fn must supply Euler's number"
    );
    assert!(decoded.active);
}

// ============================================================================
// Test 10 — seq_len = "u32" on a Vec<u64> field
// ============================================================================

#[derive(Debug, PartialEq, Encode, Decode)]
struct WithSeqLen {
    label: String,
    #[oxicode(seq_len = "u32")]
    readings: Vec<u64>,
}

#[test]
fn test_seq_len_u32_on_vec_u64() {
    let original = WithSeqLen {
        label: "sensor-data".to_string(),
        readings: vec![100, 200, 300, 400, 500],
    };
    let encoded = encode_to_vec(&original).expect("encode WithSeqLen");
    let (decoded, bytes_read): (WithSeqLen, _) =
        decode_from_slice(&encoded).expect("decode WithSeqLen");
    assert_eq!(original, decoded);
    assert_eq!(bytes_read, encoded.len());

    // Empty vec also round-trips
    let empty = WithSeqLen {
        label: "empty".to_string(),
        readings: vec![],
    };
    let enc2 = encode_to_vec(&empty).expect("encode empty WithSeqLen");
    let (dec2, _): (WithSeqLen, _) = decode_from_slice(&enc2).expect("decode empty WithSeqLen");
    assert_eq!(empty, dec2);
}

// ============================================================================
// Test 11 — newtype struct around HashMap<String, u64>
// ============================================================================

#[derive(Debug, PartialEq, Encode, Decode)]
struct NewtypeHashMap(HashMap<String, u64>);

#[test]
fn test_newtype_around_hashmap() {
    let mut inner: HashMap<String, u64> = HashMap::new();
    inner.insert("alpha".to_string(), 1);
    inner.insert("beta".to_string(), 2);
    inner.insert("gamma".to_string(), 3);

    let original = NewtypeHashMap(inner);
    let encoded = encode_to_vec(&original).expect("encode NewtypeHashMap");
    let (decoded, _): (NewtypeHashMap, _) =
        decode_from_slice(&encoded).expect("decode NewtypeHashMap");
    assert_eq!(original.0, decoded.0);
}

// ============================================================================
// Test 12 — newtype struct around BTreeMap<u32, String>
// ============================================================================

#[derive(Debug, PartialEq, Encode, Decode)]
struct NewtypeBTreeMap(BTreeMap<u32, String>);

#[test]
fn test_newtype_around_btreemap() {
    let mut inner: BTreeMap<u32, String> = BTreeMap::new();
    inner.insert(1, "one".to_string());
    inner.insert(2, "two".to_string());
    inner.insert(10, "ten".to_string());

    let original = NewtypeBTreeMap(inner);
    let encoded = encode_to_vec(&original).expect("encode NewtypeBTreeMap");
    let (decoded, _): (NewtypeBTreeMap, _) =
        decode_from_slice(&encoded).expect("decode NewtypeBTreeMap");
    assert_eq!(original, decoded);
}

// ============================================================================
// Test 13 — struct with generic field Vec<T> where T is u32
// ============================================================================

#[derive(Debug, PartialEq, Encode, Decode)]
struct GenericVec<T> {
    items: Vec<T>,
    count: usize,
}

#[test]
fn test_generic_vec_field_u32() {
    let original: GenericVec<u32> = GenericVec {
        items: vec![10, 20, 30, 40, 50],
        count: 5,
    };
    let encoded = encode_to_vec(&original).expect("encode GenericVec<u32>");
    let (decoded, _): (GenericVec<u32>, _) =
        decode_from_slice(&encoded).expect("decode GenericVec<u32>");
    assert_eq!(original, decoded);

    // Also test with an empty vec
    let empty: GenericVec<u32> = GenericVec {
        items: vec![],
        count: 0,
    };
    let enc2 = encode_to_vec(&empty).expect("encode GenericVec<u32> empty");
    let (dec2, _): (GenericVec<u32>, _) =
        decode_from_slice(&enc2).expect("decode GenericVec<u32> empty");
    assert_eq!(empty, dec2);
}

// ============================================================================
// Test 14 — struct with a fixed-size array field [u8; 16]
// ============================================================================

#[derive(Debug, PartialEq, Encode, Decode)]
struct WithFixedArray {
    version: u32,
    uuid_bytes: [u8; 16],
}

#[test]
fn test_fixed_size_array_field() {
    let original = WithFixedArray {
        version: 1,
        uuid_bytes: [
            0x6b, 0xa7, 0xb8, 0x10, 0x9d, 0xad, 0x11, 0xd1, 0x80, 0xb4, 0x00, 0xc0, 0x4f, 0xd4,
            0x30, 0xc8,
        ],
    };
    let encoded = encode_to_vec(&original).expect("encode WithFixedArray");
    let (decoded, bytes_read): (WithFixedArray, _) =
        decode_from_slice(&encoded).expect("decode WithFixedArray");
    assert_eq!(original, decoded);
    assert_eq!(bytes_read, encoded.len());

    // Zero-filled array also round-trips
    let zeroed = WithFixedArray {
        version: 0,
        uuid_bytes: [0u8; 16],
    };
    let enc2 = encode_to_vec(&zeroed).expect("encode zeroed WithFixedArray");
    let (dec2, _): (WithFixedArray, _) =
        decode_from_slice(&enc2).expect("decode zeroed WithFixedArray");
    assert_eq!(zeroed, dec2);
}

// ============================================================================
// Test 15 — tuple struct with 5 mixed-type fields
// ============================================================================

#[derive(Debug, PartialEq, Encode, Decode)]
struct FiveField(u8, i32, f64, String, bool);

#[test]
fn test_tuple_struct_five_mixed_fields() {
    let original = FiveField(
        255,
        -100_000,
        std::f64::consts::E,
        "mixed-tuple".to_string(),
        true,
    );
    let encoded = encode_to_vec(&original).expect("encode FiveField");
    let (decoded, bytes_read): (FiveField, _) =
        decode_from_slice(&encoded).expect("decode FiveField");
    assert_eq!(original, decoded);
    assert_eq!(bytes_read, encoded.len());

    // Boundary values
    let boundary = FiveField(0, i32::MIN, f64::MAX, String::new(), false);
    let enc2 = encode_to_vec(&boundary).expect("encode boundary FiveField");
    let (dec2, _): (FiveField, _) = decode_from_slice(&enc2).expect("decode boundary FiveField");
    assert_eq!(boundary, dec2);
}

// ============================================================================
// Test 16 — unit struct encodes to exactly zero bytes
// ============================================================================

#[derive(Debug, PartialEq, Encode, Decode)]
struct Sentinel;

#[test]
fn test_unit_struct_is_zero_bytes() {
    let sentinel = Sentinel;
    let encoded = encode_to_vec(&sentinel).expect("encode Sentinel");
    assert_eq!(
        encoded.len(),
        0,
        "unit struct must encode to exactly 0 bytes"
    );

    let (decoded, bytes_consumed): (Sentinel, _) =
        decode_from_slice(&encoded).expect("decode Sentinel");
    assert_eq!(sentinel, decoded);
    assert_eq!(bytes_consumed, 0, "unit struct decode consumes 0 bytes");
}

// ============================================================================
// Test 17 — struct with #[oxicode(crate = "oxicode")] path override
// ============================================================================

#[derive(Debug, PartialEq, Encode, Decode)]
#[oxicode(crate = "oxicode")]
struct WithCratePath {
    x: u32,
    y: u32,
}

#[test]
fn test_crate_path_attr_struct_roundtrip() {
    let original = WithCratePath { x: 100, y: 200 };
    let encoded = encode_to_vec(&original).expect("encode WithCratePath");
    let (decoded, bytes_read): (WithCratePath, _) =
        decode_from_slice(&encoded).expect("decode WithCratePath");
    assert_eq!(original, decoded);
    assert_eq!(bytes_read, encoded.len());

    // Must produce identical bytes to a plain struct (crate path doesn't change wire format)
    #[derive(Encode)]
    struct PlainXY {
        x: u32,
        y: u32,
    }
    let plain_enc = encode_to_vec(&PlainXY { x: 100, y: 200 }).expect("encode PlainXY");
    assert_eq!(
        encoded, plain_enc,
        "crate path override must not change wire bytes"
    );
}

// ============================================================================
// Test 18 — struct with very long field names (50 characters each)
// ============================================================================

#[derive(Debug, PartialEq, Encode, Decode)]
struct LongFieldNames {
    this_is_a_very_long_field_name_with_fifty_chars_aa: u32,
    this_is_a_very_long_field_name_with_fifty_chars_bb: u64,
    this_is_a_very_long_field_name_with_fifty_chars_cc: String,
}

#[test]
fn test_struct_with_long_field_names() {
    let original = LongFieldNames {
        this_is_a_very_long_field_name_with_fifty_chars_aa: u32::MAX,
        this_is_a_very_long_field_name_with_fifty_chars_bb: u64::MAX,
        this_is_a_very_long_field_name_with_fifty_chars_cc: "long-names".to_string(),
    };
    let encoded = encode_to_vec(&original).expect("encode LongFieldNames");
    let (decoded, _): (LongFieldNames, _) =
        decode_from_slice(&encoded).expect("decode LongFieldNames");
    assert_eq!(original, decoded);
}

// ============================================================================
// Test 19 — struct with field name starting with underscore
// ============================================================================

#[derive(Debug, PartialEq, Encode, Decode)]
struct WithUnderscoreField {
    /// Public field
    pub_value: u32,
    /// Field name starting with underscore (private-by-convention)
    _private_counter: u64,
    /// Another underscore field
    _internal_flag: bool,
}

#[test]
fn test_struct_with_underscore_field_names() {
    let original = WithUnderscoreField {
        pub_value: 42,
        _private_counter: 9999,
        _internal_flag: true,
    };
    let encoded = encode_to_vec(&original).expect("encode WithUnderscoreField");
    let (decoded, bytes_read): (WithUnderscoreField, _) =
        decode_from_slice(&encoded).expect("decode WithUnderscoreField");
    assert_eq!(original, decoded);
    assert_eq!(bytes_read, encoded.len());
}

// ============================================================================
// Test 20 — field order determines encoding order
// ============================================================================

#[derive(Debug, PartialEq, Encode, Decode)]
struct FieldOrderABC {
    a: u8,
    b: u16,
    c: u32,
}

#[derive(Debug, PartialEq, Encode, Decode)]
struct FieldOrderCBA {
    c: u32,
    b: u16,
    a: u8,
}

#[test]
fn test_field_order_determines_encoding_order() {
    // Use fixed-int legacy config so widths are deterministic
    let config = oxicode::config::legacy();

    let abc = FieldOrderABC { a: 1, b: 2, c: 3 };
    let cba = FieldOrderCBA { c: 3, b: 2, a: 1 };

    let enc_abc = oxicode::encode_to_vec_with_config(&abc, config).expect("encode ABC");
    let enc_cba = oxicode::encode_to_vec_with_config(&cba, config).expect("encode CBA");

    // Both encode to 7 bytes (1+2+4) but in different orders so they must differ
    assert_eq!(enc_abc.len(), enc_cba.len(), "same total size");
    assert_ne!(
        enc_abc, enc_cba,
        "different field orders must produce different byte streams"
    );

    // Each still round-trips correctly with its own type
    let (dec_abc, _): (FieldOrderABC, _) =
        oxicode::decode_from_slice_with_config(&enc_abc, config).expect("decode ABC");
    let (dec_cba, _): (FieldOrderCBA, _) =
        oxicode::decode_from_slice_with_config(&enc_cba, config).expect("decode CBA");
    assert_eq!(abc, dec_abc);
    assert_eq!(cba, dec_cba);
}

// ============================================================================
// Test 21 — struct with Vec<Option<String>> field
// ============================================================================

#[derive(Debug, PartialEq, Encode, Decode)]
struct WithNestedOptionVec {
    id: u32,
    entries: Vec<Option<String>>,
}

#[test]
fn test_nested_vec_option_string_roundtrip() {
    let original = WithNestedOptionVec {
        id: 5,
        entries: vec![
            Some("first".to_string()),
            None,
            Some("third".to_string()),
            None,
            Some(String::new()),
        ],
    };
    let encoded = encode_to_vec(&original).expect("encode WithNestedOptionVec");
    let (decoded, bytes_read): (WithNestedOptionVec, _) =
        decode_from_slice(&encoded).expect("decode WithNestedOptionVec");
    assert_eq!(original, decoded);
    assert_eq!(bytes_read, encoded.len());

    // All None variant
    let all_none = WithNestedOptionVec {
        id: 0,
        entries: vec![None, None, None],
    };
    let enc2 = encode_to_vec(&all_none).expect("encode all-None");
    let (dec2, _): (WithNestedOptionVec, _) = decode_from_slice(&enc2).expect("decode all-None");
    assert_eq!(all_none, dec2);
}

// ============================================================================
// Test 22 — struct with doc comments on fields (docs don't affect encoding)
// ============================================================================

#[derive(Debug, PartialEq, Encode, Decode)]
struct WithDocComments {
    /// The primary identifier for this record.
    /// Must be unique within the dataset.
    id: u64,
    /// Human-readable name, UTF-8 encoded.
    ///
    /// # Constraints
    /// - Maximum 255 bytes after UTF-8 encoding
    name: String,
    /// Mathematical constant π used as a reference value.
    ///
    /// Stored as IEEE 754 double-precision float.
    reference_pi: f64,
    /// Euler's number e for exponential growth calculations.
    reference_e: f64,
    /// Whether this record is currently active.
    enabled: bool,
}

#[test]
fn test_doc_comments_do_not_affect_encoding() {
    let original = WithDocComments {
        id: 0xDEAD_BEEF_CAFE_1234,
        name: "documented-struct".to_string(),
        reference_pi: std::f64::consts::PI,
        reference_e: std::f64::consts::E,
        enabled: true,
    };
    let encoded = encode_to_vec(&original).expect("encode WithDocComments");
    let (decoded, bytes_read): (WithDocComments, _) =
        decode_from_slice(&encoded).expect("decode WithDocComments");
    assert_eq!(original, decoded);
    assert_eq!(bytes_read, encoded.len());

    // Wire format must match a plain struct without doc comments
    #[derive(Encode)]
    struct PlainEquivalent {
        id: u64,
        name: String,
        reference_pi: f64,
        reference_e: f64,
        enabled: bool,
    }
    let plain_enc = encode_to_vec(&PlainEquivalent {
        id: 0xDEAD_BEEF_CAFE_1234,
        name: "documented-struct".to_string(),
        reference_pi: std::f64::consts::PI,
        reference_e: std::f64::consts::E,
        enabled: true,
    })
    .expect("encode PlainEquivalent");
    assert_eq!(
        encoded, plain_enc,
        "doc comments must not change the binary wire format"
    );
}
