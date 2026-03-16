//! Comprehensive tests for structs with varying field counts.
//!
//! Tests encoding correctness, roundtrip fidelity, and size verification
//! across structs ranging from 0 to 20+ fields.

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
use oxicode::{decode_from_slice, encode_to_vec, encoded_size};

// ---------------------------------------------------------------------------
// Test 1: Unit struct (0 fields) → empty bytes
// ---------------------------------------------------------------------------

#[derive(oxicode::Encode, oxicode::Decode, Debug, PartialEq)]
struct UnitStruct;

#[test]
fn test_01_unit_struct_empty_bytes() {
    let value = UnitStruct;
    let encoded = encode_to_vec(&value).expect("encode failed");
    assert_eq!(encoded, &[], "unit struct must encode to zero bytes");
    let (decoded, consumed): (UnitStruct, _) = decode_from_slice(&encoded).expect("decode failed");
    assert_eq!(decoded, value);
    assert_eq!(consumed, 0);
}

// ---------------------------------------------------------------------------
// Test 2: 1-field struct → just field bytes
// ---------------------------------------------------------------------------

#[derive(oxicode::Encode, oxicode::Decode, Debug, PartialEq)]
struct OneField {
    value: u32,
}

#[test]
fn test_02_one_field_struct() {
    let value = OneField { value: 42 };
    let encoded = encode_to_vec(&value).expect("encode failed");
    assert!(!encoded.is_empty(), "one-field struct must produce bytes");

    // Encoded size must match standalone u32 encoding
    let standalone = encode_to_vec(&42u32).expect("encode u32 failed");
    assert_eq!(
        encoded, standalone,
        "one-field struct bytes must equal standalone field bytes"
    );

    let (decoded, _): (OneField, _) = decode_from_slice(&encoded).expect("decode failed");
    assert_eq!(decoded, value);
}

// ---------------------------------------------------------------------------
// Test 3: 2-field struct
// ---------------------------------------------------------------------------

#[derive(oxicode::Encode, oxicode::Decode, Debug, PartialEq)]
struct TwoFields {
    a: u32,
    b: u64,
}

#[test]
fn test_03_two_field_struct() {
    let value = TwoFields { a: 10, b: 200 };
    let encoded = encode_to_vec(&value).expect("encode failed");

    let a_bytes = encode_to_vec(&10u32).expect("encode a");
    let b_bytes = encode_to_vec(&200u64).expect("encode b");
    let mut expected = a_bytes;
    expected.extend_from_slice(&b_bytes);
    assert_eq!(
        encoded, expected,
        "two-field struct bytes must be concatenation of field bytes"
    );

    let (decoded, _): (TwoFields, _) = decode_from_slice(&encoded).expect("decode failed");
    assert_eq!(decoded, value);
}

// ---------------------------------------------------------------------------
// Test 4: 3-field struct
// ---------------------------------------------------------------------------

#[derive(oxicode::Encode, oxicode::Decode, Debug, PartialEq)]
struct ThreeFields {
    x: i32,
    y: i32,
    z: i32,
}

#[test]
fn test_04_three_field_struct() {
    let value = ThreeFields { x: -1, y: 0, z: 1 };
    let encoded = encode_to_vec(&value).expect("encode failed");
    let (decoded, _): (ThreeFields, _) = decode_from_slice(&encoded).expect("decode failed");
    assert_eq!(decoded, value);
    assert!(
        encoded.len() >= 3,
        "three i32 fields must use at least 3 bytes"
    );
}

// ---------------------------------------------------------------------------
// Test 5: 5-field struct (mixed types)
// ---------------------------------------------------------------------------

#[derive(oxicode::Encode, oxicode::Decode, Debug, PartialEq)]
struct FiveFieldsMixed {
    count: u32,
    label: String,
    flag: bool,
    score: f64,
    tag: u8,
}

#[test]
fn test_05_five_field_mixed_types() {
    let value = FiveFieldsMixed {
        count: 99,
        label: "hello".to_string(),
        flag: true,
        score: std::f64::consts::PI,
        tag: 7,
    };
    let encoded = encode_to_vec(&value).expect("encode failed");
    let (decoded, _): (FiveFieldsMixed, _) = decode_from_slice(&encoded).expect("decode failed");
    assert_eq!(decoded, value);
}

// ---------------------------------------------------------------------------
// Test 6: 10-field struct (all u32)
// ---------------------------------------------------------------------------

#[derive(oxicode::Encode, oxicode::Decode, Debug, PartialEq)]
struct TenU32Fields {
    f0: u32,
    f1: u32,
    f2: u32,
    f3: u32,
    f4: u32,
    f5: u32,
    f6: u32,
    f7: u32,
    f8: u32,
    f9: u32,
}

#[test]
fn test_06_ten_field_all_u32() {
    let value = TenU32Fields {
        f0: 0,
        f1: 1,
        f2: 2,
        f3: 3,
        f4: 4,
        f5: 5,
        f6: 6,
        f7: 7,
        f8: 8,
        f9: 9,
    };
    let encoded = encode_to_vec(&value).expect("encode failed");
    let (decoded, _): (TenU32Fields, _) = decode_from_slice(&encoded).expect("decode failed");
    assert_eq!(decoded, value);
    // Each u32 0..9 encodes to 1 byte varint, so total must be 10 bytes
    assert_eq!(
        encoded.len(),
        10,
        "ten small u32 values (0-9) must encode to 10 bytes (1 varint byte each)"
    );
}

// ---------------------------------------------------------------------------
// Test 7: 15-field struct (all primitive kinds)
// ---------------------------------------------------------------------------

#[derive(oxicode::Encode, oxicode::Decode, Debug, PartialEq)]
struct FifteenFields {
    f_u8: u8,
    f_u16: u16,
    f_u32: u32,
    f_u64: u64,
    f_bool: bool,
    f_i8: i8,
    f_i16: i16,
    f_i32: i32,
    f_i64: i64,
    f_f32: f32,
    f_f64: f64,
    f_string: String,
    f_vec_u8: Vec<u8>,
    f_opt_u32: Option<u32>,
    f_char: char,
}

#[test]
fn test_07_fifteen_field_all_primitive_kinds() {
    let value = FifteenFields {
        f_u8: 255,
        f_u16: 1000,
        f_u32: 100_000,
        f_u64: 1_000_000_000,
        f_bool: false,
        f_i8: -128,
        f_i16: -1000,
        f_i32: -100_000,
        f_i64: -1_000_000_000,
        f_f32: 1.23_f32,
        f_f64: 9.87_f64,
        f_string: "oxicode".to_string(),
        f_vec_u8: vec![0xDE, 0xAD, 0xBE, 0xEF],
        f_opt_u32: Some(42),
        f_char: 'Z',
    };
    let encoded = encode_to_vec(&value).expect("encode failed");
    assert!(!encoded.is_empty());
    let (decoded, consumed): (FifteenFields, _) =
        decode_from_slice(&encoded).expect("decode failed");
    assert_eq!(decoded, value);
    assert_eq!(consumed, encoded.len());
}

// ---------------------------------------------------------------------------
// Test 8: Struct with single bool field
// ---------------------------------------------------------------------------

#[derive(oxicode::Encode, oxicode::Decode, Debug, PartialEq)]
struct SingleBool {
    flag: bool,
}

#[test]
fn test_08_single_bool_field() {
    for flag in [true, false] {
        let value = SingleBool { flag };
        let encoded = encode_to_vec(&value).expect("encode failed");
        assert_eq!(encoded.len(), 1, "bool encodes to exactly 1 byte");
        let expected_byte: u8 = if flag { 1 } else { 0 };
        assert_eq!(encoded[0], expected_byte);
        let (decoded, _): (SingleBool, _) = decode_from_slice(&encoded).expect("decode failed");
        assert_eq!(decoded, value);
    }
}

// ---------------------------------------------------------------------------
// Test 9: Struct with single String field
// ---------------------------------------------------------------------------

#[derive(oxicode::Encode, oxicode::Decode, Debug, PartialEq)]
struct SingleString {
    text: String,
}

#[test]
fn test_09_single_string_field() {
    let value = SingleString {
        text: "hello world".to_string(),
    };
    let encoded = encode_to_vec(&value).expect("encode failed");

    let standalone = encode_to_vec(&"hello world".to_string()).expect("encode string");
    assert_eq!(
        encoded, standalone,
        "single-string struct bytes must equal standalone String bytes"
    );

    let (decoded, _): (SingleString, _) = decode_from_slice(&encoded).expect("decode failed");
    assert_eq!(decoded, value);
}

// ---------------------------------------------------------------------------
// Test 10: Struct with single Vec<u8> field
// ---------------------------------------------------------------------------

#[derive(oxicode::Encode, oxicode::Decode, Debug, PartialEq)]
struct SingleVecU8 {
    data: Vec<u8>,
}

#[test]
fn test_10_single_vec_u8_field() {
    let value = SingleVecU8 {
        data: vec![1, 2, 3, 4, 5],
    };
    let encoded = encode_to_vec(&value).expect("encode failed");

    let standalone = encode_to_vec(&vec![1u8, 2, 3, 4, 5]).expect("encode vec");
    assert_eq!(
        encoded, standalone,
        "single Vec<u8> struct bytes must equal standalone Vec<u8> bytes"
    );

    let (decoded, _): (SingleVecU8, _) = decode_from_slice(&encoded).expect("decode failed");
    assert_eq!(decoded, value);
}

// ---------------------------------------------------------------------------
// Test 11: Struct with single u128 field
// ---------------------------------------------------------------------------

#[derive(oxicode::Encode, oxicode::Decode, Debug, PartialEq)]
struct SingleU128 {
    big: u128,
}

#[test]
fn test_11_single_u128_field() {
    let value = SingleU128 { big: u128::MAX / 2 };
    let encoded = encode_to_vec(&value).expect("encode failed");
    let (decoded, consumed): (SingleU128, _) = decode_from_slice(&encoded).expect("decode failed");
    assert_eq!(decoded, value);
    assert_eq!(consumed, encoded.len());

    // Also verify zero
    let zero = SingleU128 { big: 0 };
    let zero_enc = encode_to_vec(&zero).expect("encode zero");
    let (zero_dec, _): (SingleU128, _) = decode_from_slice(&zero_enc).expect("decode zero");
    assert_eq!(zero_dec, zero);
}

// ---------------------------------------------------------------------------
// Test 12: Struct with Option<String> field
// ---------------------------------------------------------------------------

#[derive(oxicode::Encode, oxicode::Decode, Debug, PartialEq)]
struct SingleOptionString {
    maybe: Option<String>,
}

#[test]
fn test_12_option_string_field() {
    let some_val = SingleOptionString {
        maybe: Some("present".to_string()),
    };
    let none_val = SingleOptionString { maybe: None };

    let enc_some = encode_to_vec(&some_val).expect("encode Some");
    let enc_none = encode_to_vec(&none_val).expect("encode None");

    assert!(
        enc_some.len() > enc_none.len(),
        "Some variant must be larger than None"
    );

    let (dec_some, _): (SingleOptionString, _) = decode_from_slice(&enc_some).expect("decode Some");
    let (dec_none, _): (SingleOptionString, _) = decode_from_slice(&enc_none).expect("decode None");

    assert_eq!(dec_some, some_val);
    assert_eq!(dec_none, none_val);
}

// ---------------------------------------------------------------------------
// Test 13: Struct with Vec<String> field
// ---------------------------------------------------------------------------

#[derive(oxicode::Encode, oxicode::Decode, Debug, PartialEq)]
struct SingleVecString {
    items: Vec<String>,
}

#[test]
fn test_13_vec_string_field() {
    let value = SingleVecString {
        items: vec!["alpha".to_string(), "beta".to_string(), "gamma".to_string()],
    };
    let encoded = encode_to_vec(&value).expect("encode failed");
    let (decoded, consumed): (SingleVecString, _) =
        decode_from_slice(&encoded).expect("decode failed");
    assert_eq!(decoded, value);
    assert_eq!(consumed, encoded.len());

    // Empty vec
    let empty = SingleVecString { items: vec![] };
    let enc_empty = encode_to_vec(&empty).expect("encode empty");
    let (dec_empty, _): (SingleVecString, _) = decode_from_slice(&enc_empty).expect("decode empty");
    assert_eq!(dec_empty, empty);
}

// ---------------------------------------------------------------------------
// Test 14: Struct with nested struct field
// ---------------------------------------------------------------------------

#[derive(oxicode::Encode, oxicode::Decode, Debug, PartialEq)]
struct Inner {
    x: u32,
    y: u32,
}

#[derive(oxicode::Encode, oxicode::Decode, Debug, PartialEq)]
struct Outer {
    label: String,
    inner: Inner,
    extra: u8,
}

#[test]
fn test_14_nested_struct_field() {
    let value = Outer {
        label: "outer".to_string(),
        inner: Inner { x: 10, y: 20 },
        extra: 99,
    };
    let encoded = encode_to_vec(&value).expect("encode failed");
    let (decoded, consumed): (Outer, _) = decode_from_slice(&encoded).expect("decode failed");
    assert_eq!(decoded, value);
    assert_eq!(consumed, encoded.len());
}

// ---------------------------------------------------------------------------
// Test 15: Struct where all fields are Options
// ---------------------------------------------------------------------------

#[derive(oxicode::Encode, oxicode::Decode, Debug, PartialEq)]
struct AllOptions {
    a: Option<u8>,
    b: Option<u16>,
    c: Option<u32>,
    d: Option<u64>,
    e: Option<String>,
}

#[test]
fn test_15_all_option_fields() {
    let all_some = AllOptions {
        a: Some(1),
        b: Some(2),
        c: Some(3),
        d: Some(4),
        e: Some("five".to_string()),
    };
    let all_none = AllOptions {
        a: None,
        b: None,
        c: None,
        d: None,
        e: None,
    };

    let enc_some = encode_to_vec(&all_some).expect("encode all Some");
    let enc_none = encode_to_vec(&all_none).expect("encode all None");

    assert!(
        enc_some.len() > enc_none.len(),
        "all-Some must be larger than all-None"
    );

    let (dec_some, _): (AllOptions, _) = decode_from_slice(&enc_some).expect("decode all Some");
    let (dec_none, _): (AllOptions, _) = decode_from_slice(&enc_none).expect("decode all None");

    assert_eq!(dec_some, all_some);
    assert_eq!(dec_none, all_none);
}

// ---------------------------------------------------------------------------
// Test 16: Struct with tuple fields
// ---------------------------------------------------------------------------

#[derive(oxicode::Encode, oxicode::Decode, Debug, PartialEq)]
struct TupleFields {
    pair: (u32, u32),
    triple: (u8, u16, u64),
}

#[test]
fn test_16_tuple_fields() {
    let value = TupleFields {
        pair: (100, 200),
        triple: (1, 2, 3),
    };
    let encoded = encode_to_vec(&value).expect("encode failed");
    let (decoded, consumed): (TupleFields, _) = decode_from_slice(&encoded).expect("decode failed");
    assert_eq!(decoded, value);
    assert_eq!(consumed, encoded.len());
}

// ---------------------------------------------------------------------------
// Test 17: Struct with array fields [u8; 16]
// ---------------------------------------------------------------------------

#[derive(oxicode::Encode, oxicode::Decode, Debug, PartialEq)]
struct ArrayField {
    data: [u8; 16],
    id: u32,
}

#[test]
fn test_17_array_field_u8_16() {
    let value = ArrayField {
        data: [
            0x00, 0x11, 0x22, 0x33, 0x44, 0x55, 0x66, 0x77, 0x88, 0x99, 0xAA, 0xBB, 0xCC, 0xDD,
            0xEE, 0xFF,
        ],
        id: 42,
    };
    let encoded = encode_to_vec(&value).expect("encode failed");
    let (decoded, consumed): (ArrayField, _) = decode_from_slice(&encoded).expect("decode failed");
    assert_eq!(decoded, value);
    assert_eq!(consumed, encoded.len());

    // The 16-byte array should appear literally in the encoded output
    // (arrays are not length-prefixed)
    let id_bytes = encode_to_vec(&42u32).expect("encode id");
    assert_eq!(
        encoded.len(),
        16 + id_bytes.len(),
        "array[u8;16] contributes exactly 16 bytes"
    );
}

// ---------------------------------------------------------------------------
// Test 18: Struct encoded_size verification
// ---------------------------------------------------------------------------

#[derive(oxicode::Encode, oxicode::Decode, Debug, PartialEq)]
struct SizeVerify {
    a: u8,
    b: u16,
    c: u32,
}

#[test]
fn test_18_encoded_size_verification() {
    let value = SizeVerify { a: 1, b: 2, c: 3 };

    let size = encoded_size(&value).expect("encoded_size failed");
    let actual_bytes = encode_to_vec(&value).expect("encode failed");

    assert_eq!(
        size,
        actual_bytes.len(),
        "encoded_size must match actual encoded byte length"
    );

    // Verify that each field's contribution is accounted for
    let a_size = encoded_size(&1u8).expect("size a");
    let b_size = encoded_size(&2u16).expect("size b");
    let c_size = encoded_size(&3u32).expect("size c");
    assert_eq!(
        size,
        a_size + b_size + c_size,
        "struct encoded size must equal sum of field sizes"
    );
}

// ---------------------------------------------------------------------------
// Test 19: Struct with 20 i64 fields (correctness)
// ---------------------------------------------------------------------------

#[derive(oxicode::Encode, oxicode::Decode, Debug, PartialEq)]
struct TwentyI64Fields {
    f00: i64,
    f01: i64,
    f02: i64,
    f03: i64,
    f04: i64,
    f05: i64,
    f06: i64,
    f07: i64,
    f08: i64,
    f09: i64,
    f10: i64,
    f11: i64,
    f12: i64,
    f13: i64,
    f14: i64,
    f15: i64,
    f16: i64,
    f17: i64,
    f18: i64,
    f19: i64,
}

#[test]
fn test_19_twenty_i64_fields_correctness() {
    let value = TwentyI64Fields {
        f00: i64::MIN,
        f01: -1_000_000_000_000,
        f02: -1_000_000_000,
        f03: -1_000_000,
        f04: -1000,
        f05: -1,
        f06: 0,
        f07: 1,
        f08: 1000,
        f09: 1_000_000,
        f10: 1_000_000_000,
        f11: 1_000_000_000_000,
        f12: i64::MAX,
        f13: 42,
        f14: -42,
        f15: 127,
        f16: -127,
        f17: 256,
        f18: -256,
        f19: 0,
    };
    let encoded = encode_to_vec(&value).expect("encode failed");
    let (decoded, consumed): (TwentyI64Fields, _) =
        decode_from_slice(&encoded).expect("decode failed");
    assert_eq!(decoded, value);
    assert_eq!(consumed, encoded.len());
}

// ---------------------------------------------------------------------------
// Test 20: Struct roundtrip with all zeros
// ---------------------------------------------------------------------------

#[derive(oxicode::Encode, oxicode::Decode, Debug, PartialEq)]
struct AllZeros {
    a: u8,
    b: u16,
    c: u32,
    d: u64,
    e: i8,
    f: i16,
    g: i32,
    h: i64,
    i: f32,
    j: f64,
}

#[test]
fn test_20_roundtrip_all_zeros() {
    let value = AllZeros {
        a: 0,
        b: 0,
        c: 0,
        d: 0,
        e: 0,
        f: 0,
        g: 0,
        h: 0,
        i: 0.0,
        j: 0.0,
    };
    let encoded = encode_to_vec(&value).expect("encode failed");
    let (decoded, consumed): (AllZeros, _) = decode_from_slice(&encoded).expect("decode failed");
    assert_eq!(decoded, value);
    assert_eq!(consumed, encoded.len());

    // All numeric zeros encode compactly with varint
    let size = encoded_size(&value).expect("size");
    assert_eq!(size, encoded.len());
}

// ---------------------------------------------------------------------------
// Test 21: Struct roundtrip with max values
// ---------------------------------------------------------------------------

#[derive(oxicode::Encode, oxicode::Decode, Debug, PartialEq)]
struct MaxValues {
    a: u8,
    b: u16,
    c: u32,
    d: u64,
    e: i8,
    f: i16,
    g: i32,
    h: i64,
}

#[test]
fn test_21_roundtrip_max_values() {
    let value = MaxValues {
        a: u8::MAX,
        b: u16::MAX,
        c: u32::MAX,
        d: u64::MAX,
        e: i8::MAX,
        f: i16::MAX,
        g: i32::MAX,
        h: i64::MAX,
    };
    let encoded = encode_to_vec(&value).expect("encode failed");
    let (decoded, consumed): (MaxValues, _) = decode_from_slice(&encoded).expect("decode failed");
    assert_eq!(decoded, value);
    assert_eq!(consumed, encoded.len());

    let size = encoded_size(&value).expect("size");
    assert_eq!(size, encoded.len());
}

// ---------------------------------------------------------------------------
// Test 22: Sequential encoding of 1000 structs
// ---------------------------------------------------------------------------

#[derive(oxicode::Encode, oxicode::Decode, Debug, PartialEq)]
struct SmallRecord {
    id: u32,
    value: u64,
    tag: u8,
}

#[test]
fn test_22_sequential_encoding_1000_structs() {
    let records: Vec<SmallRecord> = (0u32..1000)
        .map(|i| SmallRecord {
            id: i,
            value: u64::from(i) * 1000,
            tag: (i % 256) as u8,
        })
        .collect();

    // Encode all records independently and verify each roundtrips correctly
    let mut all_bytes: Vec<Vec<u8>> = Vec::with_capacity(1000);
    for record in &records {
        let encoded = encode_to_vec(record).expect("encode record");
        all_bytes.push(encoded);
    }

    for (i, (record, bytes)) in records.iter().zip(all_bytes.iter()).enumerate() {
        let (decoded, consumed): (SmallRecord, _) =
            decode_from_slice(bytes).expect("decode record");
        assert_eq!(decoded, *record, "record {i} must roundtrip correctly");
        assert_eq!(
            consumed,
            bytes.len(),
            "record {i} must consume exactly all bytes"
        );
    }

    // Also verify encoded_size matches for each record
    for record in &records {
        let size = encoded_size(record).expect("size");
        let bytes = encode_to_vec(record).expect("encode for size check");
        assert_eq!(
            size,
            bytes.len(),
            "encoded_size must match actual bytes for id={}",
            record.id
        );
    }
}
