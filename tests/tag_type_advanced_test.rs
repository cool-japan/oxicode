//! Advanced tests for the `#[oxicode(tag_type = "...")]` container attribute.
//!
//! These tests focus on scenarios distinct from derive_tag_type_test.rs and
//! derive_container_advanced_test.rs:
//!   - Byte-exact discriminant verification for all four tag widths
//!   - Tuple, struct, and mixed variant encoding with u8 tag
//!   - Interaction with big-endian and fixed-int configs
//!   - Container types (Vec, Option) wrapping tagged enums
//!   - Payload types (String, Vec<u8>) inside u8-tagged enums
//!   - u16 tag for enums that need > 256 variants
//!   - Nested struct fields using tagged enums
//!   - Compactness comparison: u8 tag vs varint default

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

// ---------------------------------------------------------------------------
// Shared helper: encode with fixed-int (legacy) config — deterministic widths
// ---------------------------------------------------------------------------

fn encode_fixed<T: Encode>(val: &T) -> Vec<u8> {
    encode_to_vec_with_config(val, config::legacy()).expect("encode_fixed failed")
}

fn roundtrip<T: Encode + Decode + PartialEq + std::fmt::Debug>(val: &T) {
    let enc = encode_to_vec(val).expect("roundtrip encode failed");
    let (dec, consumed): (T, usize) = decode_from_slice(&enc).expect("roundtrip decode failed");
    assert_eq!(dec, *val, "roundtrip value mismatch");
    assert_eq!(consumed, enc.len(), "roundtrip consumed bytes mismatch");
}

// ===========================================================================
// Test 1: tag_type = "u8" unit enum — first variant encodes as exactly 1 byte
// ===========================================================================

#[derive(Debug, PartialEq, Encode, Decode)]
#[oxicode(tag_type = "u8")]
enum SmallUnit {
    Alpha,
    Beta,
    Gamma,
}

#[test]
fn test_u8_tag_first_variant_is_one_byte() {
    let bytes = encode_fixed(&SmallUnit::Alpha);
    assert_eq!(
        bytes.len(),
        1,
        "u8 tag unit variant must be 1 byte, got {:?}",
        bytes
    );
    assert_eq!(bytes[0], 0u8, "first variant discriminant must be 0");
}

// ===========================================================================
// Test 2: tag_type = "u8" — all three unit variants roundtrip correctly
// ===========================================================================

#[test]
fn test_u8_tag_all_unit_variants_roundtrip() {
    for val in [SmallUnit::Alpha, SmallUnit::Beta, SmallUnit::Gamma] {
        roundtrip(&val);
    }
}

// ===========================================================================
// Test 3: tag_type = "u8" — encoded size is exactly 1 byte for every unit variant
// ===========================================================================

#[test]
fn test_u8_tag_all_unit_variants_are_one_byte_each() {
    let a = encode_fixed(&SmallUnit::Alpha);
    let b = encode_fixed(&SmallUnit::Beta);
    let g = encode_fixed(&SmallUnit::Gamma);
    assert_eq!(a.len(), 1, "Alpha must be 1 byte");
    assert_eq!(b.len(), 1, "Beta must be 1 byte");
    assert_eq!(g.len(), 1, "Gamma must be 1 byte");
    assert_eq!(a[0], 0, "Alpha discriminant");
    assert_eq!(b[0], 1, "Beta discriminant");
    assert_eq!(g[0], 2, "Gamma discriminant");
}

// ===========================================================================
// Test 4: tag_type = "u16" — first variant encodes as exactly 2 bytes (fixed)
// ===========================================================================

#[derive(Debug, PartialEq, Encode, Decode)]
#[oxicode(tag_type = "u16")]
enum MedUnit {
    X,
    Y,
    Z,
}

#[test]
fn test_u16_tag_first_variant_is_two_bytes() {
    let bytes = encode_fixed(&MedUnit::X);
    assert_eq!(
        bytes.len(),
        2,
        "u16 tag unit variant must be 2 bytes (fixed), got {:?}",
        bytes
    );
}

// ===========================================================================
// Test 5: tag_type = "u16" — all variants roundtrip correctly
// ===========================================================================

#[test]
fn test_u16_tag_all_unit_variants_roundtrip() {
    for val in [MedUnit::X, MedUnit::Y, MedUnit::Z] {
        roundtrip(&val);
    }
}

// ===========================================================================
// Test 6: tag_type = "u32" — first variant encodes as exactly 4 bytes (fixed)
// ===========================================================================

#[derive(Debug, PartialEq, Encode, Decode)]
#[oxicode(tag_type = "u32")]
enum WideUnit {
    One,
    Two,
}

#[test]
fn test_u32_tag_first_variant_is_four_bytes() {
    let bytes = encode_fixed(&WideUnit::One);
    assert_eq!(
        bytes.len(),
        4,
        "u32 tag unit variant must be 4 bytes (fixed), got {:?}",
        bytes
    );
}

// ===========================================================================
// Test 7: tag_type = "u32" — all variants roundtrip correctly
// ===========================================================================

#[test]
fn test_u32_tag_all_unit_variants_roundtrip() {
    for val in [WideUnit::One, WideUnit::Two] {
        roundtrip(&val);
    }
}

// ===========================================================================
// Test 8: tag_type = "u64" — first variant encodes as exactly 8 bytes (fixed)
// ===========================================================================

#[derive(Debug, PartialEq, Encode, Decode)]
#[oxicode(tag_type = "u64")]
enum HugeUnit {
    Sole,
    Other(u32),
}

#[test]
fn test_u64_tag_first_variant_is_eight_bytes() {
    let bytes = encode_fixed(&HugeUnit::Sole);
    assert_eq!(
        bytes.len(),
        8,
        "u64 tag unit variant must be 8 bytes (fixed), got {:?}",
        bytes
    );
}

// ===========================================================================
// Test 9: u8 tag produces smaller output than default (varint) for variant 0
//         In fixed-int mode: u8 = 1 byte vs default u32 = 4 bytes.
// ===========================================================================

#[derive(Debug, PartialEq, Encode, Decode)]
#[oxicode(tag_type = "u8")]
enum CompactTagged {
    First,
}

#[derive(Debug, PartialEq, Encode, Decode)]
enum DefaultTagged {
    First,
}

#[test]
fn test_u8_tag_more_compact_than_default_in_fixed_mode() {
    let compact = encode_fixed(&CompactTagged::First);
    let default_fixed = encode_fixed(&DefaultTagged::First);
    assert_eq!(
        compact.len(),
        1,
        "u8-tagged variant must be 1 byte in fixed mode"
    );
    assert_eq!(
        default_fixed.len(),
        4,
        "default u32-tagged variant must be 4 bytes in fixed mode"
    );
    assert!(
        compact.len() < default_fixed.len(),
        "u8 tag should be more compact than default u32 tag"
    );
}

// ===========================================================================
// Test 10: tag_type = "u8" — tuple variant (single field) roundtrip
// ===========================================================================

#[derive(Debug, PartialEq, Encode, Decode)]
#[oxicode(tag_type = "u8")]
enum U8TupleEnum {
    Nothing,
    Value(u64),
    Pair(u32, u32),
}

#[test]
fn test_u8_tag_tuple_variant_roundtrip() {
    let cases = [
        U8TupleEnum::Nothing,
        U8TupleEnum::Value(0xDEAD_BEEF_CAFE_0001),
        U8TupleEnum::Pair(111, 222),
    ];
    for val in cases {
        roundtrip(&val);
    }
}

// ===========================================================================
// Test 11: tag_type = "u8" — struct variant (named fields) roundtrip
// ===========================================================================

#[derive(Debug, PartialEq, Encode, Decode)]
#[oxicode(tag_type = "u8")]
enum U8StructEnum {
    Empty,
    Point { x: f64, y: f64 },
    Label { name: String, count: u32 },
}

#[test]
fn test_u8_tag_struct_variant_roundtrip() {
    let cases = vec![
        U8StructEnum::Empty,
        U8StructEnum::Point {
            x: std::f64::consts::PI,
            y: std::f64::consts::E,
        },
        U8StructEnum::Label {
            name: "test".to_string(),
            count: 99,
        },
    ];
    for val in cases {
        roundtrip(&val);
    }
}

// ===========================================================================
// Test 12: tag_type = "u8" — mixed variants (unit + tuple + struct) all roundtrip
// ===========================================================================

#[derive(Debug, PartialEq, Encode, Decode)]
#[oxicode(tag_type = "u8")]
enum U8MixedEnum {
    Unit,
    Tuple(i32, bool),
    Struct { key: u64, flag: bool },
}

#[test]
fn test_u8_tag_mixed_variants_all_roundtrip() {
    let cases = vec![
        U8MixedEnum::Unit,
        U8MixedEnum::Tuple(-42, true),
        U8MixedEnum::Struct {
            key: 0xFFFF_FFFF,
            flag: false,
        },
    ];
    for val in cases {
        roundtrip(&val);
    }
}

// ===========================================================================
// Test 13: tag_type = "u8" with big-endian config — roundtrip succeeds
// ===========================================================================

#[derive(Debug, PartialEq, Encode, Decode)]
#[oxicode(tag_type = "u8")]
enum U8BigEndianEnum {
    First,
    Second { val: u32 },
}

#[test]
fn test_u8_tag_big_endian_config_roundtrip() {
    let cfg = config::standard().with_big_endian();
    let cases = vec![
        U8BigEndianEnum::First,
        U8BigEndianEnum::Second { val: 0x0102_0304 },
    ];
    for val in &cases {
        let enc = encode_to_vec_with_config(val, cfg).expect("encode big-endian u8 tag");
        let (dec, consumed): (U8BigEndianEnum, usize) =
            decode_from_slice_with_config(&enc, cfg).expect("decode big-endian u8 tag");
        assert_eq!(dec, *val, "big-endian roundtrip mismatch");
        assert_eq!(consumed, enc.len(), "big-endian consumed bytes mismatch");
    }
}

// ===========================================================================
// Test 14: tag_type = "u8" with fixed-int encoding config — 1 byte discriminant
// ===========================================================================

#[derive(Debug, PartialEq, Encode, Decode)]
#[oxicode(tag_type = "u8")]
enum U8FixedIntEnum {
    A,
    B(u32),
}

#[test]
fn test_u8_tag_fixed_int_encoding_discriminant_size() {
    let cfg = config::standard().with_fixed_int_encoding();
    // With fixed-int encoding, u8 discriminant for unit variant A = 1 byte.
    let enc = encode_to_vec_with_config(&U8FixedIntEnum::A, cfg)
        .expect("encode fixed-int u8 tag unit variant");
    assert_eq!(
        enc.len(),
        1,
        "u8 tag with fixed-int encoding: unit variant must be 1 byte, got {:?}",
        enc
    );
    // Roundtrip B variant as well.
    let enc_b = encode_to_vec_with_config(&U8FixedIntEnum::B(42), cfg)
        .expect("encode fixed-int u8 tag tuple variant");
    let (dec_b, _): (U8FixedIntEnum, usize) =
        decode_from_slice_with_config(&enc_b, cfg).expect("decode fixed-int u8 tag");
    assert_eq!(
        dec_b,
        U8FixedIntEnum::B(42),
        "fixed-int roundtrip mismatch for B"
    );
}

// ===========================================================================
// Test 15: tag_type = "u16" big-endian — verify 2 bytes in big-endian byte order
// ===========================================================================

#[derive(Debug, PartialEq, Encode, Decode)]
#[oxicode(tag_type = "u16")]
enum U16BigEndianEnum {
    First,
    Second,
}

#[test]
fn test_u16_tag_big_endian_byte_order() {
    // With big-endian + fixed-int: u16 discriminant for variant 0 → [0x00, 0x00]
    let cfg = config::standard()
        .with_big_endian()
        .with_fixed_int_encoding();
    let enc =
        encode_to_vec_with_config(&U16BigEndianEnum::First, cfg).expect("encode big-endian u16");
    assert_eq!(
        enc.len(),
        2,
        "u16 tag (big-endian fixed) must be 2 bytes, got {:?}",
        enc
    );
    assert_eq!(enc[0], 0x00, "big-endian u16 discriminant high byte");
    assert_eq!(enc[1], 0x00, "big-endian u16 discriminant low byte");

    // Variant 1 (Second) → [0x00, 0x01]
    let enc_second = encode_to_vec_with_config(&U16BigEndianEnum::Second, cfg)
        .expect("encode big-endian u16 second");
    assert_eq!(enc_second.len(), 2, "u16 Second must be 2 bytes");
    assert_eq!(enc_second[0], 0x00, "big-endian u16 Second high byte");
    assert_eq!(enc_second[1], 0x01, "big-endian u16 Second low byte");

    // Roundtrip both variants
    for val in [U16BigEndianEnum::First, U16BigEndianEnum::Second] {
        let e = encode_to_vec_with_config(&val, cfg).expect("encode u16 be roundtrip");
        let (d, _): (U16BigEndianEnum, usize) =
            decode_from_slice_with_config(&e, cfg).expect("decode u16 be roundtrip");
        assert_eq!(d, val);
    }
}

// ===========================================================================
// Test 16: tag_type = "u8" enum inside Vec<T> — all elements roundtrip
// ===========================================================================

#[derive(Debug, PartialEq, Encode, Decode)]
#[oxicode(tag_type = "u8")]
enum U8VecItem {
    None,
    Num(i32),
    Text(String),
}

#[test]
fn test_u8_tag_enum_in_vec_roundtrip() {
    let items = vec![
        U8VecItem::None,
        U8VecItem::Num(42),
        U8VecItem::Text("hello".to_string()),
        U8VecItem::Num(-1),
        U8VecItem::None,
    ];
    let enc = encode_to_vec(&items).expect("encode Vec<U8VecItem>");
    let (dec, consumed): (Vec<U8VecItem>, usize) =
        decode_from_slice(&enc).expect("decode Vec<U8VecItem>");
    assert_eq!(dec, items, "Vec<U8VecItem> roundtrip mismatch");
    assert_eq!(
        consumed,
        enc.len(),
        "Vec<U8VecItem> consumed bytes mismatch"
    );
}

// ===========================================================================
// Test 17: tag_type = "u8" enum wrapped in Option<T> — Some and None roundtrip
// ===========================================================================

#[derive(Debug, PartialEq, Encode, Decode)]
#[oxicode(tag_type = "u8")]
enum U8OptionItem {
    Zero,
    One(u8),
}

#[test]
fn test_u8_tag_enum_in_option_roundtrip() {
    let some_val: Option<U8OptionItem> = Some(U8OptionItem::One(255));
    let none_val: Option<U8OptionItem> = None;

    roundtrip(&some_val);
    roundtrip(&none_val);

    // Verify Some(Zero) also works
    let some_zero: Option<U8OptionItem> = Some(U8OptionItem::Zero);
    roundtrip(&some_zero);
}

// ===========================================================================
// Test 18: tag_type = "u8" enum with String payload — large string roundtrip
// ===========================================================================

#[derive(Debug, PartialEq, Encode, Decode)]
#[oxicode(tag_type = "u8")]
enum U8StringPayload {
    Empty,
    Short(String),
    Long { content: String, repetitions: u32 },
}

#[test]
fn test_u8_tag_enum_with_string_payload_roundtrip() {
    let long_str = "abcdefghij".repeat(100);
    let cases = vec![
        U8StringPayload::Empty,
        U8StringPayload::Short("hello, world".to_string()),
        U8StringPayload::Long {
            content: long_str.clone(),
            repetitions: 100,
        },
    ];
    for val in cases {
        roundtrip(&val);
    }
}

// ===========================================================================
// Test 19: tag_type = "u8" enum with Vec<u8> payload — binary data roundtrip
// ===========================================================================

#[derive(Debug, PartialEq, Encode, Decode)]
#[oxicode(tag_type = "u8")]
enum U8BytesPayload {
    NoData,
    Bytes(Vec<u8>),
    TwoBuffers(Vec<u8>, Vec<u8>),
}

#[test]
fn test_u8_tag_enum_with_vec_u8_payload_roundtrip() {
    let cases = vec![
        U8BytesPayload::NoData,
        U8BytesPayload::Bytes(vec![0xDE, 0xAD, 0xBE, 0xEF]),
        U8BytesPayload::Bytes((0u8..=255u8).collect()),
        U8BytesPayload::TwoBuffers(vec![1, 2, 3], vec![4, 5, 6]),
    ];
    for val in cases {
        roundtrip(&val);
    }
}

// ===========================================================================
// Test 20: tag_type = "u16" supports 256+ variant indices — basic roundtrip
//          (u8 would overflow; u16 can represent variant index >= 256)
// ===========================================================================

// We define a u16-tagged enum with explicitly-assigned discriminants above 255
// to demonstrate that u16 handles the range that u8 cannot.
#[derive(Debug, PartialEq, Encode, Decode)]
#[oxicode(tag_type = "u16")]
enum U16WideEnum {
    #[oxicode(variant = 0)]
    Low,
    #[oxicode(variant = 255)]
    MaxU8,
    #[oxicode(variant = 256)]
    JustAboveU8,
    #[oxicode(variant = 1000)]
    High,
}

#[test]
fn test_u16_tag_supports_discriminants_above_255() {
    let cases = [
        U16WideEnum::Low,
        U16WideEnum::MaxU8,
        U16WideEnum::JustAboveU8,
        U16WideEnum::High,
    ];
    for val in cases {
        roundtrip(&val);
    }

    // In fixed-int mode, verify JustAboveU8 (discriminant 256) encodes as [0x00, 0x01]
    // in little-endian (legacy config).
    let enc = encode_fixed(&U16WideEnum::JustAboveU8);
    assert_eq!(enc.len(), 2, "u16 tag must be 2 bytes in fixed mode");
    // little-endian 256 = [0x00, 0x01]
    assert_eq!(enc[0], 0x00, "little-endian low byte of 256");
    assert_eq!(enc[1], 0x01, "little-endian high byte of 256");
}

// ===========================================================================
// Test 21: tag_type = "u8" nested as a struct field — struct roundtrip
// ===========================================================================

#[derive(Debug, PartialEq, Encode, Decode)]
#[oxicode(tag_type = "u8")]
enum StatusCode {
    Ok,
    Err(u32),
    Pending { id: u64 },
}

#[derive(Debug, PartialEq, Encode, Decode)]
struct Response {
    request_id: u64,
    status: StatusCode,
    payload: Vec<u8>,
}

#[test]
fn test_u8_tag_enum_nested_in_struct_roundtrip() {
    let cases = vec![
        Response {
            request_id: 1,
            status: StatusCode::Ok,
            payload: vec![1, 2, 3],
        },
        Response {
            request_id: 2,
            status: StatusCode::Err(404),
            payload: vec![],
        },
        Response {
            request_id: 3,
            status: StatusCode::Pending { id: 0xABCD_EF01 },
            payload: (0..32).collect(),
        },
    ];
    for val in cases {
        roundtrip(&val);
    }
}

// ===========================================================================
// Test 22: u8 tag is strictly smaller than u32 tag in fixed-int mode,
//          and even smaller than varint encoding when discriminant = 0
// ===========================================================================

#[derive(Debug, PartialEq, Encode, Decode)]
#[oxicode(tag_type = "u8")]
enum U8CompactEnum {
    First,
}

#[derive(Debug, PartialEq, Encode, Decode)]
#[oxicode(tag_type = "u32")]
enum U32CompactEnum {
    First,
}

#[test]
fn test_u8_tag_compactness_vs_u32_and_varint() {
    // Fixed-int mode comparison
    let u8_fixed = encode_fixed(&U8CompactEnum::First);
    let u32_fixed = encode_fixed(&U32CompactEnum::First);
    assert_eq!(u8_fixed.len(), 1, "u8 tag fixed must be 1 byte");
    assert_eq!(u32_fixed.len(), 4, "u32 tag fixed must be 4 bytes");
    assert!(
        u8_fixed.len() < u32_fixed.len(),
        "u8 tag must be more compact than u32 tag"
    );

    // Standard (varint) mode: variant index 0 encodes as 1 varint byte regardless,
    // but the tag_type still affects what type is used internally.
    // Both should roundtrip correctly and produce valid (small) output.
    let u8_varint = encode_to_vec(&U8CompactEnum::First).expect("encode u8 varint");
    let u32_varint = encode_to_vec(&U32CompactEnum::First).expect("encode u32 varint");
    assert!(
        !u8_varint.is_empty(),
        "u8 varint encoding must be non-empty"
    );
    assert!(
        !u32_varint.is_empty(),
        "u32 varint encoding must be non-empty"
    );

    // Both must roundtrip
    let (dec_u8, _): (U8CompactEnum, usize) =
        decode_from_slice(&u8_varint).expect("decode u8 varint");
    assert_eq!(dec_u8, U8CompactEnum::First);
    let (dec_u32, _): (U32CompactEnum, usize) =
        decode_from_slice(&u32_varint).expect("decode u32 varint");
    assert_eq!(dec_u32, U32CompactEnum::First);

    // u8 varint output must be no larger than u32 varint output for small indices
    assert!(
        u8_varint.len() <= u32_varint.len(),
        "u8 tag varint output ({} bytes) should be <= u32 tag varint output ({} bytes) for index 0",
        u8_varint.len(),
        u32_varint.len()
    );
}
