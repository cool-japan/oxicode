//! Tests for `#[oxicode(default = "fn_path")]` and `#[oxicode(skip)]` (Default::default())
//! field attributes in the derive macro.
//!
//! The `#[oxicode(default = "fn_path")]` attribute on a field means: the field is NOT
//! written to the encoded stream; when decoding, `fn_path()` is called to restore it.
//!
//! The `#[oxicode(skip)]` attribute (without a custom path) uses `Default::default()`
//! when decoding. Both forms exclude the field from the binary wire format.

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
// Module-level custom default functions
// ---------------------------------------------------------------------------

fn default_score() -> f64 {
    100.0
}

fn default_name() -> String {
    "anonymous".to_string()
}

fn default_items() -> Vec<u32> {
    vec![1, 2, 3]
}

fn default_enabled() -> bool {
    true
}

fn default_counter() -> u64 {
    42_u64
}

fn default_tag() -> u8 {
    0xFF_u8
}

fn default_array() -> [u8; 4] {
    [10, 20, 30, 40]
}

fn default_computed() -> u32 {
    // A simple "computed" value — the sum of 1..=100.
    (1_u32..=100).sum()
}

// ---------------------------------------------------------------------------
// Test 1: Struct with one skipped field using a custom default function — roundtrip
// ---------------------------------------------------------------------------

#[derive(Debug, PartialEq, Encode, Decode)]
struct WithDefaultFn {
    id: u32,
    label: String,
    #[oxicode(default = "default_name")]
    author: String,
}

#[test]
fn test_01_skip_with_default_fn_roundtrip() {
    let original = WithDefaultFn {
        id: 1,
        label: "article".to_string(),
        author: "alice".to_string(), // value is NOT encoded
    };

    let encoded = encode_to_vec(&original).expect("encode failed");
    let (decoded, bytes_read): (WithDefaultFn, _) =
        decode_from_slice(&encoded).expect("decode failed");

    assert_eq!(decoded.id, 1);
    assert_eq!(decoded.label, "article");
    // default_name() provides the restored value
    assert_eq!(decoded.author, "anonymous");
    assert_eq!(bytes_read, encoded.len());
}

// ---------------------------------------------------------------------------
// Test 2: Struct with skipped field using Default::default() — roundtrip
// ---------------------------------------------------------------------------

#[derive(Debug, PartialEq, Encode, Decode)]
struct WithSkip {
    value: u32,
    tag: String,
    #[oxicode(skip)]
    cache_key: u64,
}

#[test]
fn test_02_skip_uses_default_roundtrip() {
    let original = WithSkip {
        value: 99,
        tag: "hello".to_string(),
        cache_key: 0xDEAD_BEEF_CAFE_BABE,
    };

    let encoded = encode_to_vec(&original).expect("encode failed");
    let (decoded, bytes_read): (WithSkip, _) = decode_from_slice(&encoded).expect("decode failed");

    assert_eq!(decoded.value, 99);
    assert_eq!(decoded.tag, "hello");
    // Default::default() for u64 is 0
    assert_eq!(decoded.cache_key, 0_u64);
    assert_eq!(bytes_read, encoded.len());
}

// ---------------------------------------------------------------------------
// Test 3: Skip + default on String field (default = "String::new")
// ---------------------------------------------------------------------------

#[derive(Debug, PartialEq, Encode, Decode)]
struct StringDefault {
    id: u32,
    #[oxicode(default = "String::new")]
    notes: String,
}

#[test]
fn test_03_skip_default_string_field() {
    let original = StringDefault {
        id: 7,
        notes: "some notes".to_string(),
    };

    let encoded = encode_to_vec(&original).expect("encode failed");
    let (decoded, _): (StringDefault, _) = decode_from_slice(&encoded).expect("decode failed");

    assert_eq!(decoded.id, 7);
    // String::new() gives an empty string
    assert_eq!(decoded.notes, "");
}

// ---------------------------------------------------------------------------
// Test 4: Skip + default on Vec<u32> field (default = "Vec::new")
// ---------------------------------------------------------------------------

#[derive(Debug, PartialEq, Encode, Decode)]
struct VecDefault {
    name: String,
    #[oxicode(default = "Vec::new")]
    tags: Vec<u32>,
}

#[test]
fn test_04_skip_default_vec_field() {
    let original = VecDefault {
        name: "entry".to_string(),
        tags: vec![10, 20, 30],
    };

    let encoded = encode_to_vec(&original).expect("encode failed");
    let (decoded, _): (VecDefault, _) = decode_from_slice(&encoded).expect("decode failed");

    assert_eq!(decoded.name, "entry");
    // Vec::new() gives an empty Vec
    assert!(decoded.tags.is_empty());
}

// ---------------------------------------------------------------------------
// Test 5: Skip + default on numeric field with custom default fn
// ---------------------------------------------------------------------------

#[derive(Debug, PartialEq, Encode, Decode)]
struct NumericDefault {
    x: u32,
    #[oxicode(default = "default_counter")]
    counter: u64,
    y: u32,
}

#[test]
fn test_05_skip_numeric_field_custom_default() {
    let original = NumericDefault {
        x: 10,
        counter: 999,
        y: 20,
    };

    let encoded = encode_to_vec(&original).expect("encode failed");
    let (decoded, _): (NumericDefault, _) = decode_from_slice(&encoded).expect("decode failed");

    assert_eq!(decoded.x, 10);
    // default_counter() returns 42
    assert_eq!(decoded.counter, 42_u64);
    assert_eq!(decoded.y, 20);
}

// ---------------------------------------------------------------------------
// Test 6: Skip + default on bool field
// ---------------------------------------------------------------------------

#[derive(Debug, PartialEq, Encode, Decode)]
struct BoolDefault {
    id: u32,
    #[oxicode(default = "default_enabled")]
    enabled: bool,
}

#[test]
fn test_06_skip_default_bool_field() {
    let original = BoolDefault {
        id: 5,
        enabled: false, // value is NOT encoded
    };

    let encoded = encode_to_vec(&original).expect("encode failed");
    let (decoded, _): (BoolDefault, _) = decode_from_slice(&encoded).expect("decode failed");

    assert_eq!(decoded.id, 5);
    // default_enabled() returns true
    assert!(decoded.enabled);
}

// ---------------------------------------------------------------------------
// Test 7: Skip + default on Option<String> field restoring None
// ---------------------------------------------------------------------------

#[derive(Debug, PartialEq, Encode, Decode)]
struct OptionDefault {
    id: u32,
    #[oxicode(skip)]
    metadata: Option<String>,
}

#[test]
fn test_07_skip_option_field_default_none() {
    let original = OptionDefault {
        id: 3,
        metadata: Some("rich data".to_string()),
    };

    let encoded = encode_to_vec(&original).expect("encode failed");
    let (decoded, _): (OptionDefault, _) = decode_from_slice(&encoded).expect("decode failed");

    assert_eq!(decoded.id, 3);
    // Default::default() for Option<_> is None
    assert_eq!(decoded.metadata, None);
}

// ---------------------------------------------------------------------------
// Test 8: Multiple skipped fields with different defaults
// ---------------------------------------------------------------------------

#[derive(Debug, PartialEq, Encode, Decode)]
struct MultiDefault {
    id: u32,
    #[oxicode(default = "default_name")]
    author: String,
    active: bool,
    #[oxicode(default = "default_items")]
    history: Vec<u32>,
    #[oxicode(skip)]
    revision: u32,
}

#[test]
fn test_08_multiple_defaults_different_types() {
    let original = MultiDefault {
        id: 42,
        author: "eve".to_string(),
        active: true,
        history: vec![99, 88],
        revision: 7,
    };

    let encoded = encode_to_vec(&original).expect("encode failed");
    let (decoded, _): (MultiDefault, _) = decode_from_slice(&encoded).expect("decode failed");

    assert_eq!(decoded.id, 42);
    assert_eq!(decoded.author, "anonymous"); // default_name()
    assert!(decoded.active);
    assert_eq!(decoded.history, vec![1, 2, 3]); // default_items()
    assert_eq!(decoded.revision, 0_u32); // Default::default()
}

// ---------------------------------------------------------------------------
// Test 9: Verify skipped field is NOT present in the encoded bytes
// ---------------------------------------------------------------------------

#[derive(Debug, PartialEq, Encode, Decode)]
struct VerifySkipBytes {
    value: u32,
    #[oxicode(default = "default_counter")]
    skipped_u64: u64,
}

#[derive(Debug, Encode)]
struct NoSkipEquivalent {
    value: u32,
    skipped_u64: u64,
}

#[test]
fn test_09_skipped_field_not_in_encoded_bytes() {
    let with_skip = VerifySkipBytes {
        value: 1,
        skipped_u64: u64::MAX,
    };
    let without_skip = NoSkipEquivalent {
        value: 1,
        skipped_u64: u64::MAX,
    };

    let bytes_skip = encode_to_vec(&with_skip).expect("encode with skip");
    let bytes_no_skip = encode_to_vec(&without_skip).expect("encode without skip");

    // The version with skip must be strictly smaller because u64::MAX requires
    // 9 bytes in varint encoding, and the skipped version emits nothing for it.
    assert!(
        bytes_skip.len() < bytes_no_skip.len(),
        "expected skipped={} < no-skip={}",
        bytes_skip.len(),
        bytes_no_skip.len()
    );
}

// ---------------------------------------------------------------------------
// Test 10: Decode bytes encoded by V1 (without the field) using V2 (skip + default)
// ---------------------------------------------------------------------------

/// V1 struct: no `extra` field
#[derive(Debug, PartialEq, Encode, Decode)]
struct RecordV1 {
    id: u32,
    name: String,
}

/// V2 struct: `extra` added later, marked skip+default so V1 bytes are still decodable
#[derive(Debug, PartialEq, Encode, Decode)]
struct RecordV2 {
    id: u32,
    name: String,
    #[oxicode(default = "default_name")]
    extra: String,
}

#[test]
fn test_10_v1_bytes_decodable_by_v2() {
    let v1 = RecordV1 {
        id: 10,
        name: "legacy".to_string(),
    };

    // Encode with V1 schema (no `extra` field in stream)
    let v1_bytes = encode_to_vec(&v1).expect("v1 encode");

    // Decode using V2 schema — `extra` is not in the stream, so default is used
    let (v2_decoded, bytes_read): (RecordV2, _) =
        decode_from_slice(&v1_bytes).expect("v2 decode of v1 bytes");

    assert_eq!(v2_decoded.id, 10);
    assert_eq!(v2_decoded.name, "legacy");
    assert_eq!(v2_decoded.extra, "anonymous"); // default_name()
    assert_eq!(bytes_read, v1_bytes.len());
}

// ---------------------------------------------------------------------------
// Test 11: Skip + default on a nested struct field
// ---------------------------------------------------------------------------

#[derive(Debug, PartialEq, Encode, Decode, Default)]
struct Inner {
    x: u32,
    y: u32,
}

fn default_inner() -> Inner {
    Inner { x: 100, y: 200 }
}

#[derive(Debug, PartialEq, Encode, Decode)]
struct Outer {
    id: u32,
    #[oxicode(default = "default_inner")]
    nested: Inner,
}

#[test]
fn test_11_skip_nested_struct_field() {
    let original = Outer {
        id: 55,
        nested: Inner { x: 1, y: 2 },
    };

    let encoded = encode_to_vec(&original).expect("encode");
    let (decoded, _): (Outer, _) = decode_from_slice(&encoded).expect("decode");

    assert_eq!(decoded.id, 55);
    // default_inner() provides the restored nested struct
    assert_eq!(decoded.nested, Inner { x: 100, y: 200 });
}

// ---------------------------------------------------------------------------
// Test 12: Boundary condition — default value is deterministic and correct
// ---------------------------------------------------------------------------

fn default_boundary() -> u32 {
    u32::MAX / 2
}

#[derive(Debug, PartialEq, Encode, Decode)]
struct BoundaryDefault {
    marker: u8,
    #[oxicode(default = "default_boundary")]
    boundary_val: u32,
}

#[test]
fn test_12_boundary_default_value() {
    let original = BoundaryDefault {
        marker: 1,
        boundary_val: 0, // not encoded
    };

    let encoded = encode_to_vec(&original).expect("encode");
    let (decoded, _): (BoundaryDefault, _) = decode_from_slice(&encoded).expect("decode");

    assert_eq!(decoded.marker, 1);
    assert_eq!(decoded.boundary_val, u32::MAX / 2);
}

// ---------------------------------------------------------------------------
// Test 13: Struct where some fields are skipped and some are fully encoded
// ---------------------------------------------------------------------------

#[derive(Debug, PartialEq, Encode, Decode)]
struct MixedFields {
    a: u32, // encoded
    #[oxicode(skip)]
    b: u32, // skipped — Default
    c: String, // encoded
    #[oxicode(default = "default_tag")]
    d: u8, // skipped — custom default
    e: bool, // encoded
}

#[test]
fn test_13_mixed_encoded_and_skipped_fields() {
    let original = MixedFields {
        a: 111,
        b: 222,
        c: "payload".to_string(),
        d: 0x00,
        e: true,
    };

    let encoded = encode_to_vec(&original).expect("encode");
    let (decoded, _): (MixedFields, _) = decode_from_slice(&encoded).expect("decode");

    // Encoded fields preserved
    assert_eq!(decoded.a, 111);
    assert_eq!(decoded.c, "payload");
    assert!(decoded.e);
    // Skipped fields use defaults
    assert_eq!(decoded.b, 0_u32);
    assert_eq!(decoded.d, 0xFF_u8); // default_tag()
}

// ---------------------------------------------------------------------------
// Test 14: Roundtrip — encode skips the field, decode restores default
// ---------------------------------------------------------------------------

#[derive(Debug, PartialEq, Encode, Decode)]
struct RoundtripDefault {
    count: u32,
    #[oxicode(default = "default_items")]
    items: Vec<u32>,
}

#[test]
fn test_14_roundtrip_encode_skips_decode_restores() {
    let original = RoundtripDefault {
        count: 3,
        items: vec![7, 8, 9], // not encoded
    };

    let encoded = encode_to_vec(&original).expect("encode");
    let (decoded, bytes_read): (RoundtripDefault, _) = decode_from_slice(&encoded).expect("decode");

    assert_eq!(decoded.count, 3);
    // Items were not in the byte stream; default_items() restores [1, 2, 3]
    assert_eq!(decoded.items, vec![1, 2, 3]);
    assert_eq!(bytes_read, encoded.len());
}

// ---------------------------------------------------------------------------
// Test 15: Custom default fn that returns a computed value
// ---------------------------------------------------------------------------

#[derive(Debug, PartialEq, Encode, Decode)]
struct ComputedDefault {
    label: String,
    #[oxicode(default = "default_computed")]
    checksum: u32,
}

#[test]
fn test_15_custom_default_computed_value() {
    let original = ComputedDefault {
        label: "doc".to_string(),
        checksum: 0, // not encoded
    };

    let encoded = encode_to_vec(&original).expect("encode");
    let (decoded, _): (ComputedDefault, _) = decode_from_slice(&encoded).expect("decode");

    assert_eq!(decoded.label, "doc");
    // default_computed() = sum(1..=100) = 5050
    assert_eq!(decoded.checksum, 5050_u32);
}

// ---------------------------------------------------------------------------
// Test 16: Skip + default in an enum variant's fields
// ---------------------------------------------------------------------------

#[derive(Debug, PartialEq, Encode, Decode)]
enum Event {
    Created {
        id: u32,
        #[oxicode(default = "default_name")]
        author: String,
    },
    Deleted(u32),
}

#[test]
fn test_16_skip_default_in_enum_variant_fields() {
    let original = Event::Created {
        id: 9,
        author: "carol".to_string(), // not encoded
    };

    let encoded = encode_to_vec(&original).expect("encode");
    let (decoded, _): (Event, _) = decode_from_slice(&encoded).expect("decode");

    match decoded {
        Event::Created { id, author } => {
            assert_eq!(id, 9);
            assert_eq!(author, "anonymous"); // default_name()
        }
        other => panic!("expected Event::Created, got {:?}", other),
    }
}

// ---------------------------------------------------------------------------
// Test 17: V1 bytes (without field) decodable by V2 (skip + default)
// ---------------------------------------------------------------------------

/// Older format: only carries `score`
#[derive(Debug, PartialEq, Encode, Decode)]
struct PlayerV1 {
    score: u32,
}

/// Newer format: `nickname` added, backward-compatible via skip + default
#[derive(Debug, PartialEq, Encode, Decode)]
struct PlayerV2 {
    score: u32,
    #[oxicode(default = "default_name")]
    nickname: String,
}

#[test]
fn test_17_v1_schema_bytes_decoded_by_v2_schema() {
    let player_v1 = PlayerV1 { score: 9001 };
    let v1_bytes = encode_to_vec(&player_v1).expect("v1 encode");

    let (player_v2, _): (PlayerV2, _) =
        decode_from_slice(&v1_bytes).expect("decode v1 bytes with v2 schema");

    assert_eq!(player_v2.score, 9001);
    assert_eq!(player_v2.nickname, "anonymous"); // default_name()
}

// ---------------------------------------------------------------------------
// Test 18: Skip + default with u64 field
// ---------------------------------------------------------------------------

#[derive(Debug, PartialEq, Encode, Decode)]
struct U64Default {
    seq: u32,
    #[oxicode(default = "default_counter")]
    timestamp: u64,
}

#[test]
fn test_18_skip_default_u64_field() {
    let original = U64Default {
        seq: 1,
        timestamp: 9_999_999_999_u64, // not encoded
    };

    let encoded = encode_to_vec(&original).expect("encode");
    let (decoded, _): (U64Default, _) = decode_from_slice(&encoded).expect("decode");

    assert_eq!(decoded.seq, 1);
    // default_counter() returns 42
    assert_eq!(decoded.timestamp, 42_u64);
}

// ---------------------------------------------------------------------------
// Test 19: Skip + default with array field
// ---------------------------------------------------------------------------

#[derive(Debug, PartialEq, Encode, Decode)]
struct ArrayDefault {
    id: u32,
    #[oxicode(default = "default_array")]
    header: [u8; 4],
}

#[test]
fn test_19_skip_default_array_field() {
    let original = ArrayDefault {
        id: 77,
        header: [0xFF; 4], // not encoded
    };

    let encoded = encode_to_vec(&original).expect("encode");
    let (decoded, _): (ArrayDefault, _) = decode_from_slice(&encoded).expect("decode");

    assert_eq!(decoded.id, 77);
    // default_array() returns [10, 20, 30, 40]
    assert_eq!(decoded.header, [10_u8, 20, 30, 40]);
}

// ---------------------------------------------------------------------------
// Test 20: Skip + default with generic type parameter
// ---------------------------------------------------------------------------

fn default_zero_u32() -> u32 {
    0_u32
}

#[derive(Debug, PartialEq, Encode, Decode)]
struct GenericDefault<T>
where
    T: oxicode::Encode + oxicode::Decode + std::fmt::Debug + PartialEq,
{
    payload: T,
    #[oxicode(default = "default_zero_u32")]
    version: u32,
}

#[test]
fn test_20_skip_default_generic_type_parameter() {
    let original: GenericDefault<String> = GenericDefault {
        payload: "generic_payload".to_string(),
        version: 100, // not encoded
    };

    let encoded = encode_to_vec(&original).expect("encode");
    let (decoded, bytes_read): (GenericDefault<String>, _) =
        decode_from_slice(&encoded).expect("decode");

    assert_eq!(decoded.payload, "generic_payload");
    // default_zero_u32() returns 0
    assert_eq!(decoded.version, 0_u32);
    assert_eq!(bytes_read, encoded.len());
}

// ---------------------------------------------------------------------------
// Silence unused-function warnings for default_score and default_array
// which are used only via string paths in attributes, not directly in code.
// ---------------------------------------------------------------------------

#[allow(dead_code)]
fn _use_fns() {
    let _ = default_score();
    let _ = default_array();
}
