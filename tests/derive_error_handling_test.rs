//! Tests for error conditions in derived types.
//!
//! Covers truncated data, invalid discriminants, invalid UTF-8, smart pointer
//! fields, slice-too-small encoding, varint variant boundary, Option field
//! encodings, and discriminant-out-of-range decoding.

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
use oxicode::{config, Decode, Encode};

// ---------------------------------------------------------------------------
// 1. Decode truncated data for a derived struct
// ---------------------------------------------------------------------------

#[derive(Debug, PartialEq, Encode, Decode)]
struct TruncStruct {
    id: u64,
    label: String,
    score: f64,
}

#[test]
fn test_derive_error_handling_truncated_derived_struct() {
    let value = TruncStruct {
        id: 12345,
        label: String::from("hello world"),
        score: std::f64::consts::PI,
    };
    let encoded = oxicode::encode_to_vec(&value).expect("encode failed");
    // Truncate to half the bytes — must fail
    let truncated = &encoded[..encoded.len() / 2];
    let result: Result<(TruncStruct, usize), _> = oxicode::decode_from_slice(truncated);
    assert!(
        result.is_err(),
        "decoding truncated bytes for a derived struct must fail"
    );
}

// ---------------------------------------------------------------------------
// 2. Decode with wrong variant discriminant for derived enum
// ---------------------------------------------------------------------------

#[derive(Debug, PartialEq, Encode, Decode)]
enum ThreeVariant {
    Alpha,
    Beta,
    Gamma,
}

#[test]
fn test_derive_error_handling_wrong_variant_discriminant() {
    // Discriminant 99 does not exist in ThreeVariant (only 0, 1, 2)
    let bad_bytes = oxicode::encode_to_vec(&99u32).expect("encode discriminant");
    let result: Result<(ThreeVariant, usize), _> = oxicode::decode_from_slice(&bad_bytes);
    assert!(
        result.is_err(),
        "discriminant 99 must be rejected for ThreeVariant"
    );
}

// ---------------------------------------------------------------------------
// 3. Decode struct with invalid UTF-8 in a String field (manual byte construction)
// ---------------------------------------------------------------------------

#[derive(Debug, PartialEq, Encode, Decode)]
struct WithString {
    value: String,
}

#[test]
fn test_derive_error_handling_invalid_utf8_string_field() {
    // Manually construct bytes: the WithString struct encodes its single String field.
    // oxicode varint: length 4 as single byte 0x04, followed by 4 invalid UTF-8 bytes.
    // The struct has no outer discriminant byte; it just encodes its fields in order.
    let mut bytes: Vec<u8> = Vec::new();
    // String length prefix: 4 (single-byte varint)
    bytes.push(0x04u8);
    // 4 bytes of invalid UTF-8 (lone continuation / overlong bytes)
    bytes.extend_from_slice(&[0xFF, 0xFE, 0xFD, 0xFC]);

    let result: Result<(WithString, usize), _> = oxicode::decode_from_slice(&bytes);
    assert!(
        result.is_err(),
        "decoding invalid UTF-8 bytes as a String field must fail"
    );
}

// ---------------------------------------------------------------------------
// 4. Derive on struct with Box<T> field
// ---------------------------------------------------------------------------

#[derive(Debug, PartialEq, Encode, Decode)]
struct WithBox {
    name: String,
    count: Box<u32>,
}

#[test]
fn test_derive_error_handling_box_field_roundtrip() {
    let original = WithBox {
        name: String::from("boxed"),
        count: Box::new(42),
    };
    let encoded = oxicode::encode_to_vec(&original).expect("encode failed");
    let (decoded, _): (WithBox, _) = oxicode::decode_from_slice(&encoded).expect("decode failed");
    assert_eq!(original, decoded);
}

#[test]
fn test_derive_error_handling_box_field_truncated() {
    let original = WithBox {
        name: String::from("a longer string to guarantee truncation works"),
        count: Box::new(999),
    };
    let encoded = oxicode::encode_to_vec(&original).expect("encode failed");
    let truncated = &encoded[..encoded.len() / 2];
    let result: Result<(WithBox, usize), _> = oxicode::decode_from_slice(truncated);
    assert!(
        result.is_err(),
        "truncated bytes for Box-field struct must fail to decode"
    );
}

// ---------------------------------------------------------------------------
// 5. Derive on struct with Rc<T> field (#[cfg(feature = "alloc")])
// ---------------------------------------------------------------------------

#[cfg(feature = "alloc")]
#[derive(Debug, PartialEq, Encode, Decode)]
struct WithRc {
    data: std::rc::Rc<u64>,
    label: std::rc::Rc<String>,
}

#[cfg(feature = "alloc")]
#[test]
fn test_derive_error_handling_rc_field_roundtrip() {
    let original = WithRc {
        data: std::rc::Rc::new(0xDEAD_BEEF_CAFE_BABEu64),
        label: std::rc::Rc::new(String::from("rc label")),
    };
    let encoded = oxicode::encode_to_vec(&original).expect("encode failed");
    let (decoded, _): (WithRc, _) = oxicode::decode_from_slice(&encoded).expect("decode failed");
    assert_eq!(original, decoded);
}

#[cfg(feature = "alloc")]
#[test]
fn test_derive_error_handling_rc_field_truncated() {
    let original = WithRc {
        data: std::rc::Rc::new(12345678u64),
        label: std::rc::Rc::new(String::from("a label long enough to cause truncation")),
    };
    let encoded = oxicode::encode_to_vec(&original).expect("encode failed");
    let truncated = &encoded[..encoded.len() / 2];
    let result: Result<(WithRc, usize), _> = oxicode::decode_from_slice(truncated);
    assert!(
        result.is_err(),
        "truncated bytes for Rc-field struct must fail to decode"
    );
}

// ---------------------------------------------------------------------------
// 6. Encode derived struct to slice where slice is too small (verify Err)
// ---------------------------------------------------------------------------

#[derive(Debug, PartialEq, Encode, Decode)]
struct LargeStruct {
    payload: Vec<u8>,
    description: String,
}

#[test]
fn test_derive_error_handling_encode_to_slice_too_small() {
    let value = LargeStruct {
        payload: vec![0u8; 256],
        description: String::from("this is a fairly long description string"),
    };
    // 4-byte buffer is far too small for this struct
    let mut buf = [0u8; 4];
    let result = oxicode::encode_into_slice(value, &mut buf, config::standard());
    assert!(
        result.is_err(),
        "encoding a large struct into a 4-byte slice must fail"
    );
}

// ---------------------------------------------------------------------------
// 7. Decode derived enum with too many variants (test varint boundary at 251)
// ---------------------------------------------------------------------------

// oxicode varint: values 0-250 encode as 1 single byte.
// Value 251 encodes as 3 bytes: marker byte 0xFB (251) + 2 bytes for u16 little-endian.
// An enum with only 3 variants and discriminant 251 must fail.

#[derive(Debug, PartialEq, Encode, Decode)]
enum SmallEnum {
    First,
    Second,
    Third,
}

#[test]
fn test_derive_error_handling_varint_boundary_251() {
    // Encode the value 251u32 as varint. In oxicode's varint this produces:
    // [0xFB, 0xFB, 0x00] (marker 251, then u16 little-endian 251 = [0xFB, 0x00])
    let bad_bytes = oxicode::encode_to_vec(&251u32).expect("encode discriminant 251");
    let result: Result<(SmallEnum, usize), _> = oxicode::decode_from_slice(&bad_bytes);
    assert!(
        result.is_err(),
        "discriminant 251 is outside SmallEnum's range (0-2) and must fail"
    );
}

#[test]
fn test_derive_error_handling_varint_boundary_250_is_invalid() {
    // 250 is still a valid single-byte varint but out of range for SmallEnum
    let bad_bytes = oxicode::encode_to_vec(&250u32).expect("encode discriminant 250");
    let result: Result<(SmallEnum, usize), _> = oxicode::decode_from_slice(&bad_bytes);
    assert!(
        result.is_err(),
        "discriminant 250 is outside SmallEnum's range (0-2) and must fail"
    );
}

// ---------------------------------------------------------------------------
// 8. Struct with all Option fields where all are None: verify small encoding
// ---------------------------------------------------------------------------

#[derive(Debug, PartialEq, Encode, Decode)]
struct AllOptions {
    a: Option<u32>,
    b: Option<String>,
    c: Option<Vec<u8>>,
    d: Option<u64>,
}

#[test]
fn test_derive_error_handling_all_options_none_small_encoding() {
    let value = AllOptions {
        a: None,
        b: None,
        c: None,
        d: None,
    };
    let encoded = oxicode::encode_to_vec(&value).expect("encode failed");
    // Each None encodes as a single 0x00 byte, so 4 fields = 4 bytes
    assert_eq!(
        encoded.len(),
        4,
        "all-None struct should encode to exactly 4 bytes (one 0x00 per Option), got {:?}",
        encoded
    );
    // Verify roundtrip
    let (decoded, _): (AllOptions, _) = oxicode::decode_from_slice(&encoded).expect("decode");
    assert_eq!(value, decoded);
}

// ---------------------------------------------------------------------------
// 9. Struct with all Option fields where all are Some: verify larger encoding
// ---------------------------------------------------------------------------

#[test]
fn test_derive_error_handling_all_options_some_larger_encoding() {
    let all_none = AllOptions {
        a: None,
        b: None,
        c: None,
        d: None,
    };
    let all_some = AllOptions {
        a: Some(42),
        b: Some(String::from("hello")),
        c: Some(vec![1, 2, 3]),
        d: Some(u64::MAX),
    };

    let encoded_none = oxicode::encode_to_vec(&all_none).expect("encode none");
    let encoded_some = oxicode::encode_to_vec(&all_some).expect("encode some");

    assert!(
        encoded_some.len() > encoded_none.len(),
        "all-Some encoding ({} bytes) must be larger than all-None encoding ({} bytes)",
        encoded_some.len(),
        encoded_none.len()
    );

    // Verify roundtrip for all-Some
    let (decoded, _): (AllOptions, _) = oxicode::decode_from_slice(&encoded_some).expect("decode");
    assert_eq!(all_some, decoded);
}

// ---------------------------------------------------------------------------
// 10. Enum discriminant out of range: write 255u8 to single byte then
//     try decode 3-variant enum
// ---------------------------------------------------------------------------

#[test]
fn test_derive_error_handling_discriminant_255_single_byte() {
    // Write a single byte 0xFF (255) directly — this is a valid single-byte varint
    // for value 255 under some encodings, but let's confirm by using encode_to_vec
    // of a u8 value 255 (which encodes as a single 0xFF byte in standard config).
    // Then attempt to decode as ThreeVariant (only discriminants 0, 1, 2 are valid).
    let bytes: Vec<u8> = vec![0xFFu8];
    // In oxicode standard varint: 0xFF = 255 > 250, so it is a 3-byte marker encoding.
    // A bare [0xFF] byte is an incomplete 3-byte varint (missing 2 more bytes) so it
    // should error with UnexpectedEnd rather than UnexpectedVariant.
    // Either way the result must be Err.
    let result: Result<(ThreeVariant, usize), _> = oxicode::decode_from_slice(&bytes);
    assert!(
        result.is_err(),
        "byte 0xFF alone must fail when decoding a 3-variant enum"
    );
}

#[test]
fn test_derive_error_handling_discriminant_out_of_range_via_encoded_u8() {
    // Encode 255 as a u32 (produces full varint bytes) and try to decode as ThreeVariant.
    let bad_bytes = oxicode::encode_to_vec(&255u32).expect("encode 255");
    let result: Result<(ThreeVariant, usize), _> = oxicode::decode_from_slice(&bad_bytes);
    assert!(
        result.is_err(),
        "discriminant 255 must be rejected for ThreeVariant (variants 0-2 only)"
    );
}
