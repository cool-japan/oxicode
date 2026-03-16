//! Tests for newtype patterns (`struct Foo(T)`) with derive macros.
//!
//! A newtype is a tuple struct with exactly one field.
//! These tests verify encoding correctness, byte-level identity with the inner
//! type, roundtrip fidelity, and various composition patterns.

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
use std::collections::HashMap;

use oxicode::{decode_from_slice, encode_to_vec, encoded_size};

// ---------------------------------------------------------------------------
// Type definitions used across tests
// ---------------------------------------------------------------------------

#[derive(oxicode::Encode, oxicode::Decode, Debug, PartialEq, Clone)]
struct NewU32(u32);

#[derive(oxicode::Encode, oxicode::Decode, Debug, PartialEq, Eq, Hash, Clone)]
struct NewString(String);

#[derive(oxicode::Encode, oxicode::Decode, Debug, PartialEq, Clone)]
struct NewVecU8(Vec<u8>);

#[derive(oxicode::Encode, oxicode::Decode, Debug, PartialEq, Clone)]
struct NewU64(u64);

#[derive(oxicode::Encode, oxicode::Decode, Debug, PartialEq, Clone)]
struct NewF64(f64);

#[derive(oxicode::Encode, oxicode::Decode, Debug, PartialEq, Clone)]
struct NewBool(bool);

#[derive(oxicode::Encode, oxicode::Decode, Debug, PartialEq, Clone)]
struct NewOptionU32(Option<u32>);

#[derive(oxicode::Encode, oxicode::Decode, Debug, PartialEq, Clone)]
struct NewVecString(Vec<String>);

#[derive(oxicode::Encode, oxicode::Decode, Debug, PartialEq, Clone)]
struct NewHashMap(HashMap<String, u32>);

/// Double-wrapped newtype: A(B(u32))
#[derive(oxicode::Encode, oxicode::Decode, Debug, PartialEq, Clone)]
struct InnerB(u32);

#[derive(oxicode::Encode, oxicode::Decode, Debug, PartialEq, Clone)]
struct OuterA(InnerB);

/// Tuple struct with two fields (not a single-field newtype, but a tuple struct)
#[derive(oxicode::Encode, oxicode::Decode, Debug, PartialEq, Clone)]
struct Pair(u32, u64);

/// Newtype with `#[oxicode(transparent)]` — encodes as the inner field directly.
#[derive(oxicode::Encode, oxicode::Decode, Debug, PartialEq, Clone)]
#[oxicode(transparent)]
struct TransparentU32(u32);

/// Newtype enum: a single-field tuple variant.
#[derive(oxicode::Encode, oxicode::Decode, Debug, PartialEq, Clone)]
enum MyU32 {
    Value(u32),
}

/// Array newtype.
#[derive(oxicode::Encode, oxicode::Decode, Debug, PartialEq, Clone)]
struct Arr([u8; 32]);

/// Generic newtype
#[derive(oxicode::Encode, oxicode::Decode, Debug, PartialEq, Clone)]
struct Wrapper<T>(T);

/// Newtype used inside a struct
#[derive(oxicode::Encode, oxicode::Decode, Debug, PartialEq, Clone)]
struct ContainerStruct {
    id: NewU32,
    label: NewString,
}

/// Newtype used inside an enum
#[derive(oxicode::Encode, oxicode::Decode, Debug, PartialEq, Clone)]
enum ContainerEnum {
    Tagged(NewU32),
    Named { value: NewString },
}

/// Newtype with `BorrowDecode` support (wraps `&'a str` for zero-copy decoding).
/// Only `Encode` and `BorrowDecode` are derived; `Decode` is not applicable to `&str`.
#[derive(oxicode::Encode, oxicode::BorrowDecode, Debug, PartialEq, Clone)]
struct BorrowableStr<'a>(&'a str);

// ---------------------------------------------------------------------------
// Test 1: NewType(u32) encodes the same bytes as raw u32
// ---------------------------------------------------------------------------
#[test]
fn test_newtype_u32_same_bytes_as_raw() {
    let raw: u32 = 12345;
    let newtype = NewU32(raw);

    let raw_bytes = encode_to_vec(&raw).expect("encode raw u32");
    let newtype_bytes = encode_to_vec(&newtype).expect("encode NewU32");

    assert_eq!(
        raw_bytes, newtype_bytes,
        "NewU32 must encode identically to raw u32"
    );
}

// ---------------------------------------------------------------------------
// Test 2: NewType(String) encodes the same bytes as raw String
// ---------------------------------------------------------------------------
#[test]
fn test_newtype_string_same_bytes_as_raw() {
    let raw = String::from("hello oxicode");
    let newtype = NewString(raw.clone());

    let raw_bytes = encode_to_vec(&raw).expect("encode raw String");
    let newtype_bytes = encode_to_vec(&newtype).expect("encode NewString");

    assert_eq!(
        raw_bytes, newtype_bytes,
        "NewString must encode identically to raw String"
    );
}

// ---------------------------------------------------------------------------
// Test 3: NewType(Vec<u8>) encodes the same bytes as raw Vec<u8>
// ---------------------------------------------------------------------------
#[test]
fn test_newtype_vec_u8_same_bytes_as_raw() {
    let raw: Vec<u8> = vec![0x01, 0x02, 0x03, 0xAB, 0xCD, 0xEF];
    let newtype = NewVecU8(raw.clone());

    let raw_bytes = encode_to_vec(&raw).expect("encode raw Vec<u8>");
    let newtype_bytes = encode_to_vec(&newtype).expect("encode NewVecU8");

    assert_eq!(
        raw_bytes, newtype_bytes,
        "NewVecU8 must encode identically to raw Vec<u8>"
    );
}

// ---------------------------------------------------------------------------
// Test 4: NewType(u64) roundtrip
// ---------------------------------------------------------------------------
#[test]
fn test_newtype_u64_roundtrip() {
    let original = NewU64(u64::MAX / 3);

    let encoded = encode_to_vec(&original).expect("encode NewU64");
    let (decoded, _): (NewU64, _) = decode_from_slice(&encoded).expect("decode NewU64");

    assert_eq!(original, decoded);
}

// ---------------------------------------------------------------------------
// Test 5: NewType(f64) roundtrip — bits comparison for exact equality
// ---------------------------------------------------------------------------
#[test]
fn test_newtype_f64_roundtrip_bits() {
    let value: f64 = std::f64::consts::PI;
    let original = NewF64(value);

    let encoded = encode_to_vec(&original).expect("encode NewF64");
    let (decoded, _): (NewF64, _) = decode_from_slice(&encoded).expect("decode NewF64");

    assert_eq!(
        original.0.to_bits(),
        decoded.0.to_bits(),
        "f64 bits must be preserved exactly through encode/decode"
    );
}

// ---------------------------------------------------------------------------
// Test 6: NewType(bool) roundtrip
// ---------------------------------------------------------------------------
#[test]
fn test_newtype_bool_roundtrip() {
    for value in [true, false] {
        let original = NewBool(value);
        let encoded = encode_to_vec(&original).expect("encode NewBool");
        let (decoded, _): (NewBool, _) = decode_from_slice(&encoded).expect("decode NewBool");
        assert_eq!(original, decoded, "NewBool({}) roundtrip failed", value);
    }
}

// ---------------------------------------------------------------------------
// Test 7: NewType(Option<u32>) roundtrip
// ---------------------------------------------------------------------------
#[test]
fn test_newtype_option_u32_roundtrip() {
    let cases = [
        NewOptionU32(None),
        NewOptionU32(Some(42)),
        NewOptionU32(Some(u32::MAX)),
    ];

    for original in &cases {
        let encoded = encode_to_vec(original).expect("encode NewOptionU32");
        let (decoded, _): (NewOptionU32, _) =
            decode_from_slice(&encoded).expect("decode NewOptionU32");
        assert_eq!(original, &decoded);
    }
}

// ---------------------------------------------------------------------------
// Test 8: NewType(Vec<String>) roundtrip
// ---------------------------------------------------------------------------
#[test]
fn test_newtype_vec_string_roundtrip() {
    let original = NewVecString(vec![
        String::from("alpha"),
        String::from("beta"),
        String::from("gamma"),
    ]);

    let encoded = encode_to_vec(&original).expect("encode NewVecString");
    let (decoded, _): (NewVecString, _) = decode_from_slice(&encoded).expect("decode NewVecString");

    assert_eq!(original, decoded);
}

// ---------------------------------------------------------------------------
// Test 9: NewType(HashMap<String, u32>) roundtrip
// ---------------------------------------------------------------------------
#[test]
fn test_newtype_hashmap_roundtrip() {
    let mut map = HashMap::new();
    map.insert(String::from("one"), 1u32);
    map.insert(String::from("two"), 2u32);
    map.insert(String::from("three"), 3u32);

    let original = NewHashMap(map);
    let encoded = encode_to_vec(&original).expect("encode NewHashMap");
    let (decoded, _): (NewHashMap, _) = decode_from_slice(&encoded).expect("decode NewHashMap");

    assert_eq!(original, decoded);
}

// ---------------------------------------------------------------------------
// Test 10: Double newtype OuterA(InnerB(u32)) roundtrip
// ---------------------------------------------------------------------------
#[test]
fn test_double_newtype_roundtrip() {
    let original = OuterA(InnerB(999));

    let encoded = encode_to_vec(&original).expect("encode OuterA");
    let (decoded, _): (OuterA, _) = decode_from_slice(&encoded).expect("decode OuterA");

    assert_eq!(original, decoded);

    // Also verify byte identity with raw u32 (transparent composition)
    let raw_bytes = encode_to_vec(&999u32).expect("encode raw u32");
    assert_eq!(
        raw_bytes, encoded,
        "OuterA(InnerB(u32)) must encode identically to raw u32"
    );
}

// ---------------------------------------------------------------------------
// Test 11: Tuple struct with two fields Pair(u32, u64)
// ---------------------------------------------------------------------------
#[test]
fn test_tuple_two_fields_roundtrip() {
    let original = Pair(0xDEAD_BEEF, 0x0123_4567_89AB_CDEF);

    let encoded = encode_to_vec(&original).expect("encode Pair");
    let (decoded, _): (Pair, _) = decode_from_slice(&encoded).expect("decode Pair");

    assert_eq!(original, decoded);
}

// ---------------------------------------------------------------------------
// Test 12: #[oxicode(transparent)] encodes same bytes as plain newtype
// ---------------------------------------------------------------------------
#[test]
fn test_transparent_same_bytes_as_plain_newtype() {
    let plain = NewU32(42);
    let transparent = TransparentU32(42);

    let plain_bytes = encode_to_vec(&plain).expect("encode NewU32");
    let transparent_bytes = encode_to_vec(&transparent).expect("encode TransparentU32");

    // Both must equal encoding of raw u32
    let raw_bytes = encode_to_vec(&42u32).expect("encode raw u32");
    assert_eq!(plain_bytes, raw_bytes, "NewU32 vs raw u32");
    assert_eq!(transparent_bytes, raw_bytes, "TransparentU32 vs raw u32");

    // And roundtrip the transparent variant
    let (decoded, _): (TransparentU32, _) =
        decode_from_slice(&transparent_bytes).expect("decode TransparentU32");
    assert_eq!(transparent, decoded);
}

// ---------------------------------------------------------------------------
// Test 13: Newtype enum MyU32::Value(u32) roundtrip
// ---------------------------------------------------------------------------
#[test]
fn test_newtype_enum_roundtrip() {
    let original = MyU32::Value(7777);

    let encoded = encode_to_vec(&original).expect("encode MyU32");
    let (decoded, _): (MyU32, _) = decode_from_slice(&encoded).expect("decode MyU32");

    assert_eq!(original, decoded);
}

// ---------------------------------------------------------------------------
// Test 14: Array newtype Arr([u8; 32]) roundtrip
// ---------------------------------------------------------------------------
#[test]
fn test_array_newtype_roundtrip() {
    let mut data = [0u8; 32];
    for (i, byte) in data.iter_mut().enumerate() {
        *byte = (i as u8).wrapping_mul(7);
    }
    let original = Arr(data);

    let encoded = encode_to_vec(&original).expect("encode Arr");
    let (decoded, _): (Arr, _) = decode_from_slice(&encoded).expect("decode Arr");

    assert_eq!(original, decoded);
}

// ---------------------------------------------------------------------------
// Test 15: Generic newtype Wrapper<u32> roundtrip
// ---------------------------------------------------------------------------
#[test]
fn test_generic_newtype_u32_roundtrip() {
    let original: Wrapper<u32> = Wrapper(0xCAFE_BABE);

    let encoded = encode_to_vec(&original).expect("encode Wrapper<u32>");
    let (decoded, _): (Wrapper<u32>, _) = decode_from_slice(&encoded).expect("decode Wrapper<u32>");

    assert_eq!(original, decoded);

    // Byte identity with raw u32
    let raw_bytes = encode_to_vec(&0xCAFE_BABEu32).expect("encode raw");
    assert_eq!(
        raw_bytes, encoded,
        "Wrapper<u32> must encode identically to raw u32"
    );
}

// ---------------------------------------------------------------------------
// Test 16: Generic newtype Wrapper<String> roundtrip
// ---------------------------------------------------------------------------
#[test]
fn test_generic_newtype_string_roundtrip() {
    let original: Wrapper<String> = Wrapper(String::from("generic string wrapper"));

    let encoded = encode_to_vec(&original).expect("encode Wrapper<String>");
    let (decoded, _): (Wrapper<String>, _) =
        decode_from_slice(&encoded).expect("decode Wrapper<String>");

    assert_eq!(original, decoded);

    // Byte identity with raw String
    let raw_bytes =
        encode_to_vec(&String::from("generic string wrapper")).expect("encode raw String");
    assert_eq!(
        raw_bytes, encoded,
        "Wrapper<String> must encode identically to raw String"
    );
}

// ---------------------------------------------------------------------------
// Test 17: Vec of newtypes roundtrip
// ---------------------------------------------------------------------------
#[test]
fn test_vec_of_newtypes_roundtrip() {
    let original: Vec<NewU32> = vec![NewU32(1), NewU32(2), NewU32(3), NewU32(u32::MAX)];

    let encoded = encode_to_vec(&original).expect("encode Vec<NewU32>");
    let (decoded, _): (Vec<NewU32>, _) = decode_from_slice(&encoded).expect("decode Vec<NewU32>");

    assert_eq!(original, decoded);

    // Byte identity with Vec<u32>
    let raw: Vec<u32> = vec![1, 2, 3, u32::MAX];
    let raw_bytes = encode_to_vec(&raw).expect("encode Vec<u32>");
    assert_eq!(
        raw_bytes, encoded,
        "Vec<NewU32> must encode identically to Vec<u32>"
    );
}

// ---------------------------------------------------------------------------
// Test 18: HashMap of newtypes roundtrip
// ---------------------------------------------------------------------------
#[test]
fn test_hashmap_of_newtypes_roundtrip() {
    let mut original: HashMap<NewString, NewU32> = HashMap::new();
    original.insert(NewString(String::from("alpha")), NewU32(1));
    original.insert(NewString(String::from("beta")), NewU32(2));

    let encoded = encode_to_vec(&original).expect("encode HashMap<NewString, NewU32>");
    let (decoded, _): (HashMap<NewString, NewU32>, _) =
        decode_from_slice(&encoded).expect("decode HashMap<NewString, NewU32>");

    assert_eq!(original, decoded);
}

// ---------------------------------------------------------------------------
// Test 19: Newtype inside a struct roundtrip
// ---------------------------------------------------------------------------
#[test]
fn test_newtype_inside_struct_roundtrip() {
    let original = ContainerStruct {
        id: NewU32(100),
        label: NewString(String::from("container")),
    };

    let encoded = encode_to_vec(&original).expect("encode ContainerStruct");
    let (decoded, _): (ContainerStruct, _) =
        decode_from_slice(&encoded).expect("decode ContainerStruct");

    assert_eq!(original, decoded);
}

// ---------------------------------------------------------------------------
// Test 20: Newtype inside an enum roundtrip
// ---------------------------------------------------------------------------
#[test]
fn test_newtype_inside_enum_roundtrip() {
    let cases = [
        ContainerEnum::Tagged(NewU32(55)),
        ContainerEnum::Named {
            value: NewString(String::from("named variant")),
        },
    ];

    for original in &cases {
        let encoded = encode_to_vec(original).expect("encode ContainerEnum");
        let (decoded, _): (ContainerEnum, _) =
            decode_from_slice(&encoded).expect("decode ContainerEnum");
        assert_eq!(original, &decoded);
    }
}

// ---------------------------------------------------------------------------
// Test 21: BorrowDecode for newtype containing &str (zero-copy)
// ---------------------------------------------------------------------------
#[test]
fn test_borrow_decode_newtype_str() {
    // Encode using a String (owned), then borrow-decode back as &str newtype
    let original_str = String::from("zero-copy borrow decode");
    let original = NewString(original_str.clone());
    let encoded = encode_to_vec(&original).expect("encode NewString for borrow test");

    // BorrowableStr wraps &str; decode from the encoded bytes zero-copy
    let (decoded, consumed): (BorrowableStr<'_>, _) =
        oxicode::borrow_decode_from_slice(&encoded).expect("borrow_decode BorrowableStr");

    assert_eq!(decoded.0, original_str.as_str());
    assert_eq!(consumed, encoded.len());
}

// ---------------------------------------------------------------------------
// Test 22: encoded_size for newtype matches inner type size
// ---------------------------------------------------------------------------
#[test]
fn test_encoded_size_newtype_matches_inner() {
    let inner_u32: u32 = 42;
    let newtype_u32 = NewU32(inner_u32);
    assert_eq!(
        encoded_size(&inner_u32).expect("size raw u32"),
        encoded_size(&newtype_u32).expect("size NewU32"),
        "encoded_size of NewU32 must equal encoded_size of raw u32"
    );

    let inner_str = String::from("hello");
    let newtype_str = NewString(inner_str.clone());
    assert_eq!(
        encoded_size(&inner_str).expect("size raw String"),
        encoded_size(&newtype_str).expect("size NewString"),
        "encoded_size of NewString must equal encoded_size of raw String"
    );

    // Transparent newtype must also match
    let transparent = TransparentU32(100);
    let raw_100: u32 = 100;
    assert_eq!(
        encoded_size(&raw_100).expect("size raw u32 100"),
        encoded_size(&transparent).expect("size TransparentU32"),
        "encoded_size of TransparentU32 must equal encoded_size of raw u32"
    );
}
