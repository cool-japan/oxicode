//! Tests for `#[oxicode(encode_with = "fn")]` and `#[oxicode(decode_with = "fn")]`
//! field-level attributes using free-function signatures.
//!
//! Covers 20 scenarios: custom type transformations, enums-as-integers,
//! byte manipulation, optional fields, ranges, normalization, validation,
//! nested structs, generics, alternative bool encoding, IP addresses,
//! sorted vectors, ASCII chars, mixed fields, tuple structs, and enum
//! struct variants.

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
use oxicode::{Decode, Encode};

// ---------------------------------------------------------------------------
// Test 1 helpers: u32 ↔ String
// ---------------------------------------------------------------------------

fn encode_u32_as_str<E: oxicode::enc::Encoder>(
    val: &u32,
    encoder: &mut E,
) -> Result<(), oxicode::error::Error> {
    use oxicode::enc::Encode;
    val.to_string().encode(encoder)
}

fn decode_u32_from_str<D: oxicode::de::Decoder<Context = ()>>(
    decoder: &mut D,
) -> Result<u32, oxicode::error::Error> {
    use oxicode::de::Decode;
    let s = String::decode(decoder)?;
    s.parse::<u32>()
        .map_err(|_| oxicode::error::Error::InvalidData {
            message: "u32 parse error",
        })
}

#[derive(Debug, PartialEq, Encode, Decode)]
struct U32AsString {
    #[oxicode(encode_with = "encode_u32_as_str", decode_with = "decode_u32_from_str")]
    value: u32,
}

#[test]
fn test01_u32_stored_as_string_roundtrip() {
    let original = U32AsString { value: 12345 };
    let encoded = oxicode::encode_to_vec(&original).expect("encode");
    let (decoded, n): (U32AsString, _) = oxicode::decode_from_slice(&encoded).expect("decode");
    assert_eq!(decoded, original);
    assert_eq!(n, encoded.len());
}

// ---------------------------------------------------------------------------
// Test 2 helpers: enum ↔ u8
// ---------------------------------------------------------------------------

#[derive(Debug, PartialEq, Clone, Copy)]
enum Color {
    Red,
    Green,
    Blue,
}

fn encode_color_as_u8<E: oxicode::enc::Encoder>(
    val: &Color,
    encoder: &mut E,
) -> Result<(), oxicode::error::Error> {
    use oxicode::enc::Encode;
    let byte: u8 = match val {
        Color::Red => 0,
        Color::Green => 1,
        Color::Blue => 2,
    };
    byte.encode(encoder)
}

fn decode_color_from_u8<D: oxicode::de::Decoder<Context = ()>>(
    decoder: &mut D,
) -> Result<Color, oxicode::error::Error> {
    use oxicode::de::Decode;
    match u8::decode(decoder)? {
        0 => Ok(Color::Red),
        1 => Ok(Color::Green),
        2 => Ok(Color::Blue),
        _ => Err(oxicode::error::Error::InvalidData {
            message: "unknown Color discriminant",
        }),
    }
}

#[derive(Debug, PartialEq, Encode, Decode)]
struct ColorField {
    #[oxicode(
        encode_with = "encode_color_as_u8",
        decode_with = "decode_color_from_u8"
    )]
    color: Color,
    id: u32,
}

#[test]
fn test02_enum_stored_as_u8_roundtrip() {
    for color in [Color::Red, Color::Green, Color::Blue] {
        let original = ColorField { color, id: 7 };
        let encoded = oxicode::encode_to_vec(&original).expect("encode");
        let (decoded, _): (ColorField, _) = oxicode::decode_from_slice(&encoded).expect("decode");
        assert_eq!(decoded, original);
    }
}

// ---------------------------------------------------------------------------
// Test 3 helpers: Vec<u8> with reversed bytes
// ---------------------------------------------------------------------------

fn encode_bytes_reversed<E: oxicode::enc::Encoder>(
    val: &[u8],
    encoder: &mut E,
) -> Result<(), oxicode::error::Error> {
    use oxicode::enc::Encode;
    let mut rev = val.to_vec();
    rev.reverse();
    rev.encode(encoder)
}

fn decode_bytes_reversed<D: oxicode::de::Decoder<Context = ()>>(
    decoder: &mut D,
) -> Result<Vec<u8>, oxicode::error::Error> {
    use oxicode::de::Decode;
    let mut v = Vec::<u8>::decode(decoder)?;
    v.reverse();
    Ok(v)
}

#[derive(Debug, PartialEq, Encode, Decode)]
struct ReversedBytes {
    #[oxicode(
        encode_with = "encode_bytes_reversed",
        decode_with = "decode_bytes_reversed"
    )]
    data: Vec<u8>,
}

#[test]
fn test03_vec_bytes_reversed_roundtrip() {
    let original = ReversedBytes {
        data: vec![1, 2, 3, 4, 5],
    };
    let encoded = oxicode::encode_to_vec(&original).expect("encode");
    let (decoded, _): (ReversedBytes, _) = oxicode::decode_from_slice(&encoded).expect("decode");
    assert_eq!(decoded.data, vec![1, 2, 3, 4, 5]);
}

// ---------------------------------------------------------------------------
// Test 4 helpers: multiply-by-2 transformation (compression-like)
// ---------------------------------------------------------------------------

fn encode_doubled<E: oxicode::enc::Encoder>(
    val: &u32,
    encoder: &mut E,
) -> Result<(), oxicode::error::Error> {
    use oxicode::enc::Encode;
    (val.saturating_mul(2)).encode(encoder)
}

fn decode_halved<D: oxicode::de::Decoder<Context = ()>>(
    decoder: &mut D,
) -> Result<u32, oxicode::error::Error> {
    use oxicode::de::Decode;
    let v = u32::decode(decoder)?;
    Ok(v / 2)
}

#[derive(Debug, PartialEq, Encode, Decode)]
struct DoubledValue {
    #[oxicode(encode_with = "encode_doubled", decode_with = "decode_halved")]
    amount: u32,
}

#[test]
fn test04_multiply_divide_transformation_roundtrip() {
    let original = DoubledValue { amount: 100 };
    let encoded = oxicode::encode_to_vec(&original).expect("encode");
    let (decoded, _): (DoubledValue, _) = oxicode::decode_from_slice(&encoded).expect("decode");
    assert_eq!(decoded.amount, 100);
}

// ---------------------------------------------------------------------------
// Test 5 helpers: Duration stored as (u64, u32) tuple
// ---------------------------------------------------------------------------

use std::time::Duration;

fn encode_duration<E: oxicode::enc::Encoder>(
    val: &Duration,
    encoder: &mut E,
) -> Result<(), oxicode::error::Error> {
    use oxicode::enc::Encode;
    (val.as_secs(), val.subsec_nanos()).encode(encoder)
}

fn decode_duration<D: oxicode::de::Decoder<Context = ()>>(
    decoder: &mut D,
) -> Result<Duration, oxicode::error::Error> {
    use oxicode::de::Decode;
    let (secs, nanos) = <(u64, u32)>::decode(decoder)?;
    Ok(Duration::new(secs, nanos))
}

#[derive(Debug, PartialEq, Encode, Decode)]
struct WithDuration {
    #[oxicode(encode_with = "encode_duration", decode_with = "decode_duration")]
    timeout: Duration,
    label: String,
}

#[test]
fn test05_duration_stored_as_secs_nanos_roundtrip() {
    let original = WithDuration {
        timeout: Duration::new(42, 500_000_000),
        label: "deadline".into(),
    };
    let encoded = oxicode::encode_to_vec(&original).expect("encode");
    let (decoded, _): (WithDuration, _) = oxicode::decode_from_slice(&encoded).expect("decode");
    assert_eq!(decoded.timeout, Duration::new(42, 500_000_000));
    assert_eq!(decoded.label, "deadline");
}

// ---------------------------------------------------------------------------
// Test 6 helpers: chrono-like date stored as i64 days since epoch
// ---------------------------------------------------------------------------

/// Simple date representation (year, month, day) stored as i64 ordinal days.
#[derive(Debug, PartialEq, Clone, Copy)]
struct SimpleDate {
    year: i32,
    /// 1-based month.
    month: u8,
    /// 1-based day.
    day: u8,
}

impl SimpleDate {
    /// Compute approximate ordinal day number (not RFC-accurate, just for test determinism).
    fn to_day_ordinal(self) -> i64 {
        (self.year as i64) * 365 + (self.month as i64 - 1) * 30 + (self.day as i64 - 1)
    }

    fn from_day_ordinal(ord: i64) -> Self {
        let year = (ord / 365) as i32;
        let remaining = ord % 365;
        let month = (remaining / 30 + 1).min(12) as u8;
        let day = (remaining % 30 + 1).min(31) as u8;
        SimpleDate { year, month, day }
    }
}

fn encode_date_as_i64<E: oxicode::enc::Encoder>(
    val: &SimpleDate,
    encoder: &mut E,
) -> Result<(), oxicode::error::Error> {
    use oxicode::enc::Encode;
    val.to_day_ordinal().encode(encoder)
}

fn decode_date_from_i64<D: oxicode::de::Decoder<Context = ()>>(
    decoder: &mut D,
) -> Result<SimpleDate, oxicode::error::Error> {
    use oxicode::de::Decode;
    let ord = i64::decode(decoder)?;
    Ok(SimpleDate::from_day_ordinal(ord))
}

#[derive(Debug, PartialEq, Encode, Decode)]
struct EventWithDate {
    #[oxicode(
        encode_with = "encode_date_as_i64",
        decode_with = "decode_date_from_i64"
    )]
    date: SimpleDate,
    name: String,
}

#[test]
fn test06_date_stored_as_i64_ordinal_roundtrip() {
    let date = SimpleDate {
        year: 2024,
        month: 3,
        day: 15,
    };
    let original = EventWithDate {
        date,
        name: "conference".into(),
    };
    let encoded = oxicode::encode_to_vec(&original).expect("encode");
    let (decoded, _): (EventWithDate, _) = oxicode::decode_from_slice(&encoded).expect("decode");
    assert_eq!(decoded.date, date);
    assert_eq!(decoded.name, "conference");
}

// ---------------------------------------------------------------------------
// Test 7: Multiple fields using encode_with / decode_with simultaneously
// ---------------------------------------------------------------------------

#[derive(Debug, PartialEq, Encode, Decode)]
struct MultiCustomFields {
    #[oxicode(encode_with = "encode_u32_as_str", decode_with = "decode_u32_from_str")]
    id: u32,
    label: String,
    #[oxicode(encode_with = "encode_doubled", decode_with = "decode_halved")]
    count: u32,
}

#[test]
fn test07_multiple_fields_with_custom_encode_decode() {
    let original = MultiCustomFields {
        id: 99,
        label: "hello".into(),
        count: 50,
    };
    let encoded = oxicode::encode_to_vec(&original).expect("encode");
    let (decoded, _): (MultiCustomFields, _) =
        oxicode::decode_from_slice(&encoded).expect("decode");
    assert_eq!(decoded.id, 99);
    assert_eq!(decoded.label, "hello");
    assert_eq!(decoded.count, 50);
}

// ---------------------------------------------------------------------------
// Test 8 helpers: optional field stored as 0 for None
// ---------------------------------------------------------------------------

fn encode_option_u32_as_zero<E: oxicode::enc::Encoder>(
    val: &Option<u32>,
    encoder: &mut E,
) -> Result<(), oxicode::error::Error> {
    use oxicode::enc::Encode;
    val.unwrap_or(0).encode(encoder)
}

fn decode_option_u32_from_zero<D: oxicode::de::Decoder<Context = ()>>(
    decoder: &mut D,
) -> Result<Option<u32>, oxicode::error::Error> {
    use oxicode::de::Decode;
    let v = u32::decode(decoder)?;
    Ok(if v == 0 { None } else { Some(v) })
}

#[derive(Debug, PartialEq, Encode, Decode)]
struct OptionalAsZero {
    name: String,
    #[oxicode(
        encode_with = "encode_option_u32_as_zero",
        decode_with = "decode_option_u32_from_zero"
    )]
    score: Option<u32>,
}

#[test]
fn test08_optional_stored_as_zero_for_none() {
    let with_value = OptionalAsZero {
        name: "alice".into(),
        score: Some(42),
    };
    let encoded = oxicode::encode_to_vec(&with_value).expect("encode");
    let (decoded, _): (OptionalAsZero, _) = oxicode::decode_from_slice(&encoded).expect("decode");
    assert_eq!(decoded.score, Some(42));

    let without_value = OptionalAsZero {
        name: "bob".into(),
        score: None,
    };
    let encoded2 = oxicode::encode_to_vec(&without_value).expect("encode");
    let (decoded2, _): (OptionalAsZero, _) = oxicode::decode_from_slice(&encoded2).expect("decode");
    assert_eq!(decoded2.score, None);
}

// ---------------------------------------------------------------------------
// Test 9 helpers: Range<u32> stored as two u32s
// ---------------------------------------------------------------------------

use std::ops::Range;

fn encode_range_u32<E: oxicode::enc::Encoder>(
    val: &Range<u32>,
    encoder: &mut E,
) -> Result<(), oxicode::error::Error> {
    use oxicode::enc::Encode;
    val.start.encode(encoder)?;
    val.end.encode(encoder)
}

fn decode_range_u32<D: oxicode::de::Decoder<Context = ()>>(
    decoder: &mut D,
) -> Result<Range<u32>, oxicode::error::Error> {
    use oxicode::de::Decode;
    let start = u32::decode(decoder)?;
    let end = u32::decode(decoder)?;
    Ok(start..end)
}

#[derive(Debug, PartialEq, Encode, Decode)]
struct WithRange {
    #[oxicode(encode_with = "encode_range_u32", decode_with = "decode_range_u32")]
    span: Range<u32>,
}

#[test]
fn test09_range_stored_as_two_u32s_roundtrip() {
    let original = WithRange { span: 10..50 };
    let encoded = oxicode::encode_to_vec(&original).expect("encode");
    let (decoded, _): (WithRange, _) = oxicode::decode_from_slice(&encoded).expect("decode");
    assert_eq!(decoded.span, 10..50);
}

// ---------------------------------------------------------------------------
// Test 10 helpers: string normalization — lowercase on encode
// ---------------------------------------------------------------------------

fn encode_string_lowercase<E: oxicode::enc::Encoder>(
    val: &str,
    encoder: &mut E,
) -> Result<(), oxicode::error::Error> {
    use oxicode::enc::Encode;
    val.to_lowercase().encode(encoder)
}

fn decode_string_pass<D: oxicode::de::Decoder<Context = ()>>(
    decoder: &mut D,
) -> Result<String, oxicode::error::Error> {
    use oxicode::de::Decode;
    String::decode(decoder)
}

#[derive(Debug, PartialEq, Encode, Decode)]
struct NormalizedString {
    #[oxicode(
        encode_with = "encode_string_lowercase",
        decode_with = "decode_string_pass"
    )]
    tag: String,
}

#[test]
fn test10_string_normalized_to_lowercase_on_encode() {
    let original = NormalizedString {
        tag: "HELLO_WORLD".into(),
    };
    let encoded = oxicode::encode_to_vec(&original).expect("encode");
    let (decoded, _): (NormalizedString, _) = oxicode::decode_from_slice(&encoded).expect("decode");
    // The stored form is lowercase, so the decoded value is lowercase.
    assert_eq!(decoded.tag, "hello_world");
}

// ---------------------------------------------------------------------------
// Test 11 helpers: decode_with that validates range (panics in test context
// via .expect() — the decode fn returns Err for out-of-range values)
// ---------------------------------------------------------------------------

fn encode_u8_validated<E: oxicode::enc::Encoder>(
    val: &u8,
    encoder: &mut E,
) -> Result<(), oxicode::error::Error> {
    use oxicode::enc::Encode;
    val.encode(encoder)
}

fn decode_u8_clamped<D: oxicode::de::Decoder<Context = ()>>(
    decoder: &mut D,
) -> Result<u8, oxicode::error::Error> {
    use oxicode::de::Decode;
    let v = u8::decode(decoder)?;
    if v > 100 {
        Err(oxicode::error::Error::InvalidData {
            message: "value out of range [0,100]",
        })
    } else {
        Ok(v)
    }
}

#[derive(Debug, PartialEq, Encode, Decode)]
struct ValidatedByte {
    #[oxicode(encode_with = "encode_u8_validated", decode_with = "decode_u8_clamped")]
    percentage: u8,
}

#[test]
fn test11_decode_with_validation_ok() {
    let original = ValidatedByte { percentage: 75 };
    let encoded = oxicode::encode_to_vec(&original).expect("encode");
    let (decoded, _): (ValidatedByte, _) = oxicode::decode_from_slice(&encoded).expect("decode");
    assert_eq!(decoded.percentage, 75);
}

#[test]
fn test11b_decode_with_validation_error() {
    // Manually craft bytes for ValidatedByte { percentage: 200 } — a single byte 200.
    // oxicode encodes u8 as a single byte in default config.
    let bytes = oxicode::encode_to_vec(&200u8).expect("encode");
    let result: Result<(ValidatedByte, _), _> = oxicode::decode_from_slice(&bytes);
    assert!(result.is_err(), "should fail validation for value > 100");
}

// ---------------------------------------------------------------------------
// Test 12: Nested struct where inner struct uses encode_with
// ---------------------------------------------------------------------------

#[derive(Debug, PartialEq, Encode, Decode)]
struct Inner {
    #[oxicode(encode_with = "encode_u32_as_str", decode_with = "decode_u32_from_str")]
    code: u32,
}

#[derive(Debug, PartialEq, Encode, Decode)]
struct Outer {
    inner: Inner,
    description: String,
}

#[test]
fn test12_nested_struct_with_inner_encode_with() {
    let original = Outer {
        inner: Inner { code: 42 },
        description: "nested".into(),
    };
    let encoded = oxicode::encode_to_vec(&original).expect("encode");
    let (decoded, _): (Outer, _) = oxicode::decode_from_slice(&encoded).expect("decode");
    assert_eq!(decoded.inner.code, 42);
    assert_eq!(decoded.description, "nested");
}

// ---------------------------------------------------------------------------
// Test 13: Generic struct with encode_with on a concrete field type
// ---------------------------------------------------------------------------

fn encode_string_len_u32<E: oxicode::enc::Encoder>(
    val: &str,
    encoder: &mut E,
) -> Result<(), oxicode::error::Error> {
    use oxicode::enc::Encode;
    (val.len() as u32).encode(encoder)
}

fn decode_string_from_len<D: oxicode::de::Decoder<Context = ()>>(
    decoder: &mut D,
) -> Result<String, oxicode::error::Error> {
    use oxicode::de::Decode;
    let len = u32::decode(decoder)?;
    Ok("_".repeat(len as usize))
}

#[derive(Debug, PartialEq, Encode, Decode)]
struct GenericContainer<T: Encode + Decode> {
    payload: T,
    #[oxicode(
        encode_with = "encode_string_len_u32",
        decode_with = "decode_string_from_len"
    )]
    metadata: String,
}

#[test]
fn test13_generic_struct_with_encode_with_on_concrete_field() {
    let original = GenericContainer::<u64> {
        payload: 9999,
        metadata: "hello".into(),
    };
    let encoded = oxicode::encode_to_vec(&original).expect("encode");
    let (decoded, _): (GenericContainer<u64>, _) =
        oxicode::decode_from_slice(&encoded).expect("decode");
    assert_eq!(decoded.payload, 9999);
    // metadata was encoded as length=5 then decoded as "_____"
    assert_eq!(decoded.metadata, "_____");
}

// ---------------------------------------------------------------------------
// Test 14 helpers: bool stored as u8 0/1 (alternative to default bool encoding)
// ---------------------------------------------------------------------------

fn encode_bool_as_u8<E: oxicode::enc::Encoder>(
    val: &bool,
    encoder: &mut E,
) -> Result<(), oxicode::error::Error> {
    use oxicode::enc::Encode;
    (*val as u8).encode(encoder)
}

fn decode_bool_from_u8<D: oxicode::de::Decoder<Context = ()>>(
    decoder: &mut D,
) -> Result<bool, oxicode::error::Error> {
    use oxicode::de::Decode;
    let v = u8::decode(decoder)?;
    match v {
        0 => Ok(false),
        1 => Ok(true),
        _ => Err(oxicode::error::Error::InvalidData {
            message: "expected 0 or 1 for bool",
        }),
    }
}

#[derive(Debug, PartialEq, Encode, Decode)]
struct BoolAsU8 {
    name: String,
    #[oxicode(encode_with = "encode_bool_as_u8", decode_with = "decode_bool_from_u8")]
    flag: bool,
}

#[test]
fn test14_bool_stored_as_u8_roundtrip() {
    for flag in [true, false] {
        let original = BoolAsU8 {
            name: "test".into(),
            flag,
        };
        let encoded = oxicode::encode_to_vec(&original).expect("encode");
        let (decoded, _): (BoolAsU8, _) = oxicode::decode_from_slice(&encoded).expect("decode");
        assert_eq!(decoded.flag, flag);
    }
}

// ---------------------------------------------------------------------------
// Test 15 helpers: IP address as [u8; 4] ↔ u32
// ---------------------------------------------------------------------------

fn encode_ip_as_u32<E: oxicode::enc::Encoder>(
    val: &[u8; 4],
    encoder: &mut E,
) -> Result<(), oxicode::error::Error> {
    use oxicode::enc::Encode;
    let n = u32::from_be_bytes(*val);
    n.encode(encoder)
}

fn decode_ip_from_u32<D: oxicode::de::Decoder<Context = ()>>(
    decoder: &mut D,
) -> Result<[u8; 4], oxicode::error::Error> {
    use oxicode::de::Decode;
    let n = u32::decode(decoder)?;
    Ok(n.to_be_bytes())
}

#[derive(Debug, PartialEq, Encode, Decode)]
struct IpRecord {
    #[oxicode(encode_with = "encode_ip_as_u32", decode_with = "decode_ip_from_u32")]
    addr: [u8; 4],
    port: u16,
}

#[test]
fn test15_ip_address_as_u32_roundtrip() {
    let original = IpRecord {
        addr: [192, 168, 1, 100],
        port: 8080,
    };
    let encoded = oxicode::encode_to_vec(&original).expect("encode");
    let (decoded, _): (IpRecord, _) = oxicode::decode_from_slice(&encoded).expect("decode");
    assert_eq!(decoded.addr, [192, 168, 1, 100]);
    assert_eq!(decoded.port, 8080);
}

// ---------------------------------------------------------------------------
// Test 16 helpers: Vec<u32> sorted on encode
// ---------------------------------------------------------------------------

fn encode_vec_sorted<E: oxicode::enc::Encoder>(
    val: &[u32],
    encoder: &mut E,
) -> Result<(), oxicode::error::Error> {
    use oxicode::enc::Encode;
    let mut sorted = val.to_vec();
    sorted.sort_unstable();
    sorted.encode(encoder)
}

fn decode_vec_passthrough<D: oxicode::de::Decoder<Context = ()>>(
    decoder: &mut D,
) -> Result<Vec<u32>, oxicode::error::Error> {
    use oxicode::de::Decode;
    Vec::<u32>::decode(decoder)
}

#[derive(Debug, PartialEq, Encode, Decode)]
struct SortedList {
    #[oxicode(
        encode_with = "encode_vec_sorted",
        decode_with = "decode_vec_passthrough"
    )]
    items: Vec<u32>,
}

#[test]
fn test16_vec_sorted_on_encode_produces_sorted_output() {
    let original = SortedList {
        items: vec![5, 3, 1, 4, 2],
    };
    let encoded = oxicode::encode_to_vec(&original).expect("encode");
    let (decoded, _): (SortedList, _) = oxicode::decode_from_slice(&encoded).expect("decode");
    assert_eq!(decoded.items, vec![1, 2, 3, 4, 5]);
}

// ---------------------------------------------------------------------------
// Test 17 helpers: char stored as u8 (ASCII only)
// ---------------------------------------------------------------------------

fn encode_char_as_u8<E: oxicode::enc::Encoder>(
    val: &char,
    encoder: &mut E,
) -> Result<(), oxicode::error::Error> {
    use oxicode::enc::Encode;
    if !val.is_ascii() {
        return Err(oxicode::error::Error::InvalidData {
            message: "char is not ASCII",
        });
    }
    (*val as u8).encode(encoder)
}

fn decode_char_from_u8<D: oxicode::de::Decoder<Context = ()>>(
    decoder: &mut D,
) -> Result<char, oxicode::error::Error> {
    use oxicode::de::Decode;
    let b = u8::decode(decoder)?;
    if b.is_ascii() {
        Ok(b as char)
    } else {
        Err(oxicode::error::Error::InvalidData {
            message: "byte is not ASCII",
        })
    }
}

#[derive(Debug, PartialEq, Encode, Decode)]
struct AsciiChar {
    #[oxicode(encode_with = "encode_char_as_u8", decode_with = "decode_char_from_u8")]
    letter: char,
    index: u32,
}

#[test]
fn test17_char_stored_as_u8_ascii_roundtrip() {
    let original = AsciiChar {
        letter: 'Z',
        index: 25,
    };
    let encoded = oxicode::encode_to_vec(&original).expect("encode");
    let (decoded, _): (AsciiChar, _) = oxicode::decode_from_slice(&encoded).expect("decode");
    assert_eq!(decoded.letter, 'Z');
    assert_eq!(decoded.index, 25);
}

// ---------------------------------------------------------------------------
// Test 18: Multiple fields — some with encode_with, some without
// ---------------------------------------------------------------------------

#[derive(Debug, PartialEq, Encode, Decode)]
struct MixedFields {
    normal_str: String,
    #[oxicode(encode_with = "encode_u32_as_str", decode_with = "decode_u32_from_str")]
    custom_id: u32,
    normal_u64: u64,
    #[oxicode(encode_with = "encode_bool_as_u8", decode_with = "decode_bool_from_u8")]
    custom_flag: bool,
}

#[test]
fn test18_mixed_custom_and_plain_fields_roundtrip() {
    let original = MixedFields {
        normal_str: "world".into(),
        custom_id: 999,
        normal_u64: u64::MAX,
        custom_flag: true,
    };
    let encoded = oxicode::encode_to_vec(&original).expect("encode");
    let (decoded, _): (MixedFields, _) = oxicode::decode_from_slice(&encoded).expect("decode");
    assert_eq!(decoded.normal_str, "world");
    assert_eq!(decoded.custom_id, 999);
    assert_eq!(decoded.normal_u64, u64::MAX);
    assert!(decoded.custom_flag);
}

// ---------------------------------------------------------------------------
// Test 19: Tuple struct field with encode_with
// ---------------------------------------------------------------------------

#[derive(Debug, PartialEq, Encode, Decode)]
struct TupleCustom(
    String,
    #[oxicode(
        encode_with = "encode_color_as_u8",
        decode_with = "decode_color_from_u8"
    )]
    Color,
    u16,
);

#[test]
fn test19_tuple_struct_field_with_encode_with_roundtrip() {
    let original = TupleCustom("label".into(), Color::Green, 512);
    let encoded = oxicode::encode_to_vec(&original).expect("encode");
    let (decoded, _): (TupleCustom, _) = oxicode::decode_from_slice(&encoded).expect("decode");
    assert_eq!(decoded.0, "label");
    assert_eq!(decoded.1, Color::Green);
    assert_eq!(decoded.2, 512);
}

// ---------------------------------------------------------------------------
// Test 20: Enum with struct variant using encode_with on a field
// ---------------------------------------------------------------------------

#[derive(Debug, PartialEq, Encode, Decode)]
enum NetworkEvent {
    Connect {
        #[oxicode(encode_with = "encode_ip_as_u32", decode_with = "decode_ip_from_u32")]
        remote_addr: [u8; 4],
        port: u16,
    },
    Disconnect {
        reason: String,
    },
    Ping,
}

#[test]
fn test20_enum_struct_variant_field_with_encode_with_roundtrip() {
    let events = [
        NetworkEvent::Connect {
            remote_addr: [10, 0, 0, 1],
            port: 443,
        },
        NetworkEvent::Disconnect {
            reason: "timeout".into(),
        },
        NetworkEvent::Ping,
    ];
    for event in &events {
        let encoded = oxicode::encode_to_vec(event).expect("encode");
        let (decoded, n): (NetworkEvent, _) = oxicode::decode_from_slice(&encoded).expect("decode");
        assert_eq!(&decoded, event);
        assert_eq!(n, encoded.len());
    }
}
