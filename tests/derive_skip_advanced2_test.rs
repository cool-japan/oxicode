//! Advanced tests for #[oxicode(skip)] and #[oxicode(default = "fn")] derive attributes.
//!
//! Covers: default type handling, field position variations, size comparisons,
//! multi-skip combinations, Option/Vec defaults, and custom default functions.

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
// Helper default functions used by `#[oxicode(default = "...")]` tests
// ---------------------------------------------------------------------------

fn default_custom_u32() -> u32 {
    42_u32
}

fn default_custom_string() -> String {
    String::from("default_tag")
}

fn default_custom_vec() -> Vec<u8> {
    vec![0xCA, 0xFE, 0xBA, 0xBE]
}

fn default_custom_bool() -> bool {
    true
}

fn default_pair_u16() -> (u16, u16) {
    (100_u16, 200_u16)
}

fn default_nested_option() -> Option<u64> {
    Some(9999_u64)
}

fn default_f64_value() -> f64 {
    3.14_f64
}

fn default_i32_negative() -> i32 {
    -1_i32
}

// ---------------------------------------------------------------------------
// Test 1: Struct with skipped Vec<u8> field (Default) — roundtrip ignores the field
// ---------------------------------------------------------------------------

#[derive(Debug, PartialEq, Encode, Decode)]
struct SkipVecField {
    id: u32,
    #[oxicode(skip)]
    cache: Vec<u8>,
    name: String,
}

#[test]
fn test_skip_vec_u8_field_default() {
    let original = SkipVecField {
        id: 1,
        cache: vec![1, 2, 3, 4, 5],
        name: "roundtrip".to_string(),
    };

    let encoded = encode_to_vec(&original).expect("encode failed");
    let (decoded, bytes_read): (SkipVecField, _) =
        decode_from_slice(&encoded).expect("decode failed");

    assert_eq!(decoded.id, 1);
    assert_eq!(decoded.name, "roundtrip");
    // Vec<u8>::default() is an empty Vec
    assert_eq!(decoded.cache, Vec::<u8>::new());
    assert_eq!(bytes_read, encoded.len());
}

// ---------------------------------------------------------------------------
// Test 2: Struct with skipped u32 field — verify 0 on decode
// ---------------------------------------------------------------------------

#[derive(Debug, PartialEq, Encode, Decode)]
struct SkipU32Field {
    label: String,
    #[oxicode(skip)]
    counter: u32,
    active: bool,
}

#[test]
fn test_skip_u32_field_is_zero_on_decode() {
    let original = SkipU32Field {
        label: "item".to_string(),
        counter: 999_999,
        active: true,
    };

    let encoded = encode_to_vec(&original).expect("encode failed");
    let (decoded, _): (SkipU32Field, _) = decode_from_slice(&encoded).expect("decode failed");

    assert_eq!(decoded.label, "item");
    assert!(decoded.active);
    // u32::default() == 0
    assert_eq!(decoded.counter, 0_u32);
}

// ---------------------------------------------------------------------------
// Test 3: Struct with skipped bool field — verify false on decode
// ---------------------------------------------------------------------------

#[derive(Debug, PartialEq, Encode, Decode)]
struct SkipBoolField {
    id: u64,
    #[oxicode(skip)]
    debug_flag: bool,
    value: i32,
}

#[test]
fn test_skip_bool_field_is_false_on_decode() {
    let original = SkipBoolField {
        id: 77,
        debug_flag: true,
        value: -5,
    };

    let encoded = encode_to_vec(&original).expect("encode failed");
    let (decoded, _): (SkipBoolField, _) = decode_from_slice(&encoded).expect("decode failed");

    assert_eq!(decoded.id, 77);
    assert_eq!(decoded.value, -5);
    // bool::default() == false
    assert!(!decoded.debug_flag);
}

// ---------------------------------------------------------------------------
// Test 4: Struct with skipped String field — verify empty string on decode
// ---------------------------------------------------------------------------

#[derive(Debug, PartialEq, Encode, Decode)]
struct SkipStringField {
    id: u32,
    payload: Vec<u8>,
    #[oxicode(skip)]
    description: String,
}

#[test]
fn test_skip_string_field_is_empty_on_decode() {
    let original = SkipStringField {
        id: 55,
        payload: vec![10, 20, 30],
        description: "this should disappear".to_string(),
    };

    let encoded = encode_to_vec(&original).expect("encode failed");
    let (decoded, _): (SkipStringField, _) = decode_from_slice(&encoded).expect("decode failed");

    assert_eq!(decoded.id, 55);
    assert_eq!(decoded.payload, vec![10, 20, 30]);
    // String::default() == ""
    assert_eq!(decoded.description, "");
}

// ---------------------------------------------------------------------------
// Test 5: Struct with skipped first field
// ---------------------------------------------------------------------------

#[derive(Debug, PartialEq, Encode, Decode)]
struct SkipFirstField {
    #[oxicode(skip)]
    prefix: u32,
    name: String,
    score: u64,
}

#[test]
fn test_skip_first_field() {
    let original = SkipFirstField {
        prefix: 0xABCD_1234,
        name: "first_skip".to_string(),
        score: 42,
    };

    let encoded = encode_to_vec(&original).expect("encode failed");
    let (decoded, _): (SkipFirstField, _) = decode_from_slice(&encoded).expect("decode failed");

    // First field not encoded; restored as default
    assert_eq!(decoded.prefix, 0_u32);
    assert_eq!(decoded.name, "first_skip");
    assert_eq!(decoded.score, 42);
}

// ---------------------------------------------------------------------------
// Test 6: Struct with skipped middle field
// ---------------------------------------------------------------------------

#[derive(Debug, PartialEq, Encode, Decode)]
struct SkipMiddleField {
    first: u32,
    #[oxicode(skip)]
    middle: String,
    last: u32,
}

#[test]
fn test_skip_middle_field() {
    let original = SkipMiddleField {
        first: 11,
        middle: "middle_value".to_string(),
        last: 22,
    };

    let encoded = encode_to_vec(&original).expect("encode failed");
    let (decoded, _): (SkipMiddleField, _) = decode_from_slice(&encoded).expect("decode failed");

    assert_eq!(decoded.first, 11);
    // Middle field is skipped
    assert_eq!(decoded.middle, "");
    assert_eq!(decoded.last, 22);
}

// ---------------------------------------------------------------------------
// Test 7: Struct with skipped last field
// ---------------------------------------------------------------------------

#[derive(Debug, PartialEq, Encode, Decode)]
struct SkipLastField {
    id: u32,
    name: String,
    #[oxicode(skip)]
    trailing_data: Vec<u8>,
}

#[test]
fn test_skip_last_field() {
    let original = SkipLastField {
        id: 99,
        name: "last_skip".to_string(),
        trailing_data: vec![255, 254, 253],
    };

    let encoded = encode_to_vec(&original).expect("encode failed");
    let (decoded, _): (SkipLastField, _) = decode_from_slice(&encoded).expect("decode failed");

    assert_eq!(decoded.id, 99);
    assert_eq!(decoded.name, "last_skip");
    // Last field skipped; default is empty Vec
    assert_eq!(decoded.trailing_data, Vec::<u8>::new());
}

// ---------------------------------------------------------------------------
// Test 8: Struct with two skipped fields
// ---------------------------------------------------------------------------

#[derive(Debug, PartialEq, Encode, Decode)]
struct TwoSkippedFields {
    id: u32,
    #[oxicode(skip)]
    cache_a: Vec<u8>,
    name: String,
    #[oxicode(skip)]
    cache_b: u64,
    active: bool,
}

#[test]
fn test_two_skipped_fields() {
    let original = TwoSkippedFields {
        id: 7,
        cache_a: vec![1, 2, 3],
        name: "two_skipped".to_string(),
        cache_b: 0xFFFF_FFFF_FFFF_FFFF,
        active: true,
    };

    let encoded = encode_to_vec(&original).expect("encode failed");
    let (decoded, bytes_read): (TwoSkippedFields, _) =
        decode_from_slice(&encoded).expect("decode failed");

    assert_eq!(decoded.id, 7);
    assert_eq!(decoded.cache_a, Vec::<u8>::new());
    assert_eq!(decoded.name, "two_skipped");
    assert_eq!(decoded.cache_b, 0_u64);
    assert!(decoded.active);
    assert_eq!(bytes_read, encoded.len());
}

// ---------------------------------------------------------------------------
// Test 9: Struct with all non-id fields skipped
// ---------------------------------------------------------------------------

#[derive(Debug, PartialEq, Encode, Decode)]
struct OnlyIdEncoded {
    id: u32,
    #[oxicode(skip)]
    state_a: String,
    #[oxicode(skip)]
    state_b: Vec<u8>,
    #[oxicode(skip)]
    state_c: bool,
    #[oxicode(skip)]
    state_d: u64,
}

#[test]
fn test_only_id_field_encoded() {
    let original = OnlyIdEncoded {
        id: 42,
        state_a: "foo".to_string(),
        state_b: vec![9, 8, 7],
        state_c: true,
        state_d: 123_456_789,
    };

    let encoded = encode_to_vec(&original).expect("encode failed");
    let (decoded, _): (OnlyIdEncoded, _) = decode_from_slice(&encoded).expect("decode failed");

    // Only id is encoded
    assert_eq!(decoded.id, 42);
    assert_eq!(decoded.state_a, "");
    assert_eq!(decoded.state_b, Vec::<u8>::new());
    assert!(!decoded.state_c);
    assert_eq!(decoded.state_d, 0_u64);
}

// ---------------------------------------------------------------------------
// Test 10: Skipped field not included in wire bytes (encoded size comparison)
// ---------------------------------------------------------------------------

#[derive(Debug, PartialEq, Encode, Decode)]
struct WithSkipForSizeTest {
    id: u32,
    #[oxicode(skip)]
    large_cache: Vec<u8>,
}

#[derive(Debug, PartialEq, Encode)]
struct WithoutSkipForSizeTest {
    id: u32,
    large_cache: Vec<u8>,
}

#[test]
fn test_skipped_field_not_in_wire_bytes() {
    let large_data: Vec<u8> = (0..255_u8).collect();

    let with_skip = WithSkipForSizeTest {
        id: 1,
        large_cache: large_data.clone(),
    };
    let without_skip = WithoutSkipForSizeTest {
        id: 1,
        large_cache: large_data,
    };

    let size_with_skip = encode_to_vec(&with_skip)
        .expect("encode with skip failed")
        .len();
    let size_without_skip = encode_to_vec(&without_skip)
        .expect("encode without skip failed")
        .len();

    // The version with skip must produce fewer bytes since the 255-byte Vec is absent
    assert!(
        size_with_skip < size_without_skip,
        "expected {size_with_skip} < {size_without_skip}: skip should reduce encoded size"
    );
}

// ---------------------------------------------------------------------------
// Test 11: Encode then decode — non-skipped fields are faithfully preserved
// ---------------------------------------------------------------------------

#[derive(Debug, PartialEq, Encode, Decode)]
struct NonSkippedPreserved {
    x: u32,
    y: u64,
    #[oxicode(skip)]
    temp: i32,
    label: String,
    flag: bool,
}

#[test]
fn test_non_skipped_fields_preserved_exactly() {
    let original = NonSkippedPreserved {
        x: 1_000_000,
        y: 9_999_999_999_u64,
        temp: -42,
        label: "preserved_label".to_string(),
        flag: true,
    };

    let encoded = encode_to_vec(&original).expect("encode failed");
    let (decoded, bytes_read): (NonSkippedPreserved, _) =
        decode_from_slice(&encoded).expect("decode failed");

    assert_eq!(decoded.x, 1_000_000);
    assert_eq!(decoded.y, 9_999_999_999_u64);
    assert_eq!(decoded.label, "preserved_label");
    assert!(decoded.flag);
    // Only temp is reset
    assert_eq!(decoded.temp, 0_i32);
    assert_eq!(bytes_read, encoded.len());
}

// ---------------------------------------------------------------------------
// Test 12: Skipped Vec<u8> gets empty default
// ---------------------------------------------------------------------------

#[derive(Debug, PartialEq, Encode, Decode)]
struct ExplicitVecDefault {
    version: u8,
    #[oxicode(skip)]
    buffered_bytes: Vec<u8>,
    checksum: u32,
}

#[test]
fn test_skipped_vec_u8_gets_empty_default() {
    let original = ExplicitVecDefault {
        version: 3,
        buffered_bytes: vec![0xFF; 64],
        checksum: 0xDEAD_BEEF,
    };

    let encoded = encode_to_vec(&original).expect("encode failed");
    let (decoded, _): (ExplicitVecDefault, _) = decode_from_slice(&encoded).expect("decode failed");

    assert_eq!(decoded.version, 3);
    assert_eq!(decoded.checksum, 0xDEAD_BEEF);
    // Empty Vec is the Default for Vec<u8>
    assert!(
        decoded.buffered_bytes.is_empty(),
        "expected empty Vec, got {:?}",
        decoded.buffered_bytes
    );
}

// ---------------------------------------------------------------------------
// Test 13: Skipped Option<String> gets None default
// ---------------------------------------------------------------------------

#[derive(Debug, PartialEq, Encode, Decode)]
struct SkipOptionString {
    id: u32,
    #[oxicode(skip)]
    optional_note: Option<String>,
    timestamp: u64,
}

#[test]
fn test_skipped_option_string_gets_none() {
    let original = SkipOptionString {
        id: 10,
        optional_note: Some("this note will not be encoded".to_string()),
        timestamp: 1_700_000_000,
    };

    let encoded = encode_to_vec(&original).expect("encode failed");
    let (decoded, _): (SkipOptionString, _) = decode_from_slice(&encoded).expect("decode failed");

    assert_eq!(decoded.id, 10);
    assert_eq!(decoded.timestamp, 1_700_000_000);
    // Option::default() == None
    assert_eq!(decoded.optional_note, None);
}

// ---------------------------------------------------------------------------
// Test 14: Struct with `default = "fn_path"` for skipped String field
// ---------------------------------------------------------------------------

#[derive(Debug, PartialEq, Encode, Decode)]
struct WithDefaultFnString {
    id: u32,
    #[oxicode(default = "default_custom_string")]
    tag: String,
    value: u64,
}

#[test]
fn test_default_fn_string_field_value() {
    let original = WithDefaultFnString {
        id: 20,
        tag: "runtime_tag".to_string(),
        value: 888,
    };

    let encoded = encode_to_vec(&original).expect("encode failed");
    let (decoded, _): (WithDefaultFnString, _) =
        decode_from_slice(&encoded).expect("decode failed");

    assert_eq!(decoded.id, 20);
    assert_eq!(decoded.value, 888);
    // default_custom_string() returns "default_tag"
    assert_eq!(decoded.tag, "default_tag");
}

// ---------------------------------------------------------------------------
// Test 15: `default = "fn_path"` for u32 field
// ---------------------------------------------------------------------------

#[derive(Debug, PartialEq, Encode, Decode)]
struct WithDefaultFnU32 {
    name: String,
    #[oxicode(default = "default_custom_u32")]
    priority: u32,
    active: bool,
}

#[test]
fn test_default_fn_u32_field_value() {
    let original = WithDefaultFnU32 {
        name: "task".to_string(),
        priority: 1,
        active: false,
    };

    let encoded = encode_to_vec(&original).expect("encode failed");
    let (decoded, _): (WithDefaultFnU32, _) = decode_from_slice(&encoded).expect("decode failed");

    assert_eq!(decoded.name, "task");
    assert!(!decoded.active);
    // default_custom_u32() returns 42
    assert_eq!(decoded.priority, 42_u32);
}

// ---------------------------------------------------------------------------
// Test 16: `default = "fn_path"` for Vec<u8> field returns non-empty default
// ---------------------------------------------------------------------------

#[derive(Debug, PartialEq, Encode, Decode)]
struct WithDefaultFnVec {
    id: u32,
    #[oxicode(default = "default_custom_vec")]
    magic_bytes: Vec<u8>,
    length: u32,
}

#[test]
fn test_default_fn_vec_u8_field_value() {
    let original = WithDefaultFnVec {
        id: 5,
        magic_bytes: vec![1, 2, 3],
        length: 3,
    };

    let encoded = encode_to_vec(&original).expect("encode failed");
    let (decoded, _): (WithDefaultFnVec, _) = decode_from_slice(&encoded).expect("decode failed");

    assert_eq!(decoded.id, 5);
    assert_eq!(decoded.length, 3);
    // default_custom_vec() returns [0xCA, 0xFE, 0xBA, 0xBE]
    assert_eq!(decoded.magic_bytes, vec![0xCA, 0xFE, 0xBA, 0xBE]);
}

// ---------------------------------------------------------------------------
// Test 17: `default = "fn_path"` for bool field returns true
// ---------------------------------------------------------------------------

#[derive(Debug, PartialEq, Encode, Decode)]
struct WithDefaultFnBool {
    id: u32,
    payload: Vec<u8>,
    #[oxicode(default = "default_custom_bool")]
    enabled: bool,
}

#[test]
fn test_default_fn_bool_field_value() {
    let original = WithDefaultFnBool {
        id: 8,
        payload: vec![50, 60],
        enabled: false,
    };

    let encoded = encode_to_vec(&original).expect("encode failed");
    let (decoded, _): (WithDefaultFnBool, _) = decode_from_slice(&encoded).expect("decode failed");

    assert_eq!(decoded.id, 8);
    assert_eq!(decoded.payload, vec![50, 60]);
    // default_custom_bool() returns true
    assert!(decoded.enabled);
}

// ---------------------------------------------------------------------------
// Test 18: Mixed skip and `default = "fn"` on the same struct
// ---------------------------------------------------------------------------

#[derive(Debug, PartialEq, Encode, Decode)]
struct MixedSkipAndDefault {
    id: u32,
    #[oxicode(skip)]
    transient_cache: Vec<u8>,
    #[oxicode(default = "default_custom_string")]
    category: String,
    score: u64,
}

#[test]
fn test_mixed_skip_and_default_fn() {
    let original = MixedSkipAndDefault {
        id: 30,
        transient_cache: vec![7, 8, 9],
        category: "original_category".to_string(),
        score: 1_500,
    };

    let encoded = encode_to_vec(&original).expect("encode failed");
    let (decoded, _): (MixedSkipAndDefault, _) =
        decode_from_slice(&encoded).expect("decode failed");

    assert_eq!(decoded.id, 30);
    assert_eq!(decoded.score, 1_500);
    // #[oxicode(skip)] → Vec::default()
    assert_eq!(decoded.transient_cache, Vec::<u8>::new());
    // #[oxicode(default = "default_custom_string")] → "default_tag"
    assert_eq!(decoded.category, "default_tag");
}

// ---------------------------------------------------------------------------
// Test 19: `default = "fn_path"` with tuple type
// ---------------------------------------------------------------------------

#[derive(Debug, PartialEq, Encode, Decode)]
struct WithTupleDefault {
    id: u32,
    #[oxicode(default = "default_pair_u16")]
    coordinates: (u16, u16),
    label: String,
}

#[test]
fn test_default_fn_tuple_type() {
    let original = WithTupleDefault {
        id: 3,
        coordinates: (10, 20),
        label: "origin".to_string(),
    };

    let encoded = encode_to_vec(&original).expect("encode failed");
    let (decoded, _): (WithTupleDefault, _) = decode_from_slice(&encoded).expect("decode failed");

    assert_eq!(decoded.id, 3);
    assert_eq!(decoded.label, "origin");
    // default_pair_u16() returns (100, 200)
    assert_eq!(decoded.coordinates, (100_u16, 200_u16));
}

// ---------------------------------------------------------------------------
// Test 20: `default = "fn_path"` for Option<u64> returning Some(...)
// ---------------------------------------------------------------------------

#[derive(Debug, PartialEq, Encode, Decode)]
struct WithOptionDefault {
    id: u32,
    #[oxicode(default = "default_nested_option")]
    max_value: Option<u64>,
    name: String,
}

#[test]
fn test_default_fn_option_u64_some() {
    let original = WithOptionDefault {
        id: 15,
        max_value: Some(0),
        name: "opt_test".to_string(),
    };

    let encoded = encode_to_vec(&original).expect("encode failed");
    let (decoded, _): (WithOptionDefault, _) = decode_from_slice(&encoded).expect("decode failed");

    assert_eq!(decoded.id, 15);
    assert_eq!(decoded.name, "opt_test");
    // default_nested_option() returns Some(9999)
    assert_eq!(decoded.max_value, Some(9_999_u64));
}

// ---------------------------------------------------------------------------
// Test 21: `default = "fn_path"` for f64 field
// ---------------------------------------------------------------------------

#[derive(Debug, PartialEq, Encode, Decode)]
struct WithF64Default {
    id: u32,
    label: String,
    #[oxicode(default = "default_f64_value")]
    ratio: f64,
}

#[test]
fn test_default_fn_f64_field_value() {
    let original = WithF64Default {
        id: 99,
        label: "f64_test".to_string(),
        ratio: 2.718_281_828,
    };

    let encoded = encode_to_vec(&original).expect("encode failed");
    let (decoded, _): (WithF64Default, _) = decode_from_slice(&encoded).expect("decode failed");

    assert_eq!(decoded.id, 99);
    assert_eq!(decoded.label, "f64_test");
    // default_f64_value() returns 3.14
    assert!((decoded.ratio - 3.14_f64).abs() < f64::EPSILON);
}

// ---------------------------------------------------------------------------
// Test 22: `default = "fn_path"` for negative i32 field
// ---------------------------------------------------------------------------

#[derive(Debug, PartialEq, Encode, Decode)]
struct WithNegativeI32Default {
    id: u32,
    #[oxicode(default = "default_i32_negative")]
    offset: i32,
    description: String,
}

#[test]
fn test_default_fn_negative_i32_field() {
    let original = WithNegativeI32Default {
        id: 200,
        offset: 500,
        description: "negative_default_test".to_string(),
    };

    let encoded = encode_to_vec(&original).expect("encode failed");
    let (decoded, bytes_read): (WithNegativeI32Default, _) =
        decode_from_slice(&encoded).expect("decode failed");

    assert_eq!(decoded.id, 200);
    assert_eq!(decoded.description, "negative_default_test");
    // default_i32_negative() returns -1
    assert_eq!(decoded.offset, -1_i32);
    assert_eq!(bytes_read, encoded.len());
}
