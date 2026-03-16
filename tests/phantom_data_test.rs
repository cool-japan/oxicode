//! Tests for PhantomData<T> and zero-sized types in OxiCode.
//!
//! PhantomData<T> encodes as zero bytes (it carries no data).
//! Unit type () also encodes as zero bytes.

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
    decode_from_slice, decode_from_slice_with_config, encode_to_vec, encode_to_vec_with_config,
    encoded_size,
};
use std::marker::PhantomData;

// Struct with a PhantomData field alongside real data (test 6).
#[derive(oxicode::Encode, oxicode::Decode, PartialEq, Debug)]
struct Typed<T> {
    value: u32,
    _phantom: std::marker::PhantomData<T>,
}

// Struct with ONLY a PhantomData field (test 19).
#[derive(oxicode::Encode, oxicode::Decode, PartialEq, Debug)]
struct PhantomOnly {
    _marker: std::marker::PhantomData<u32>,
}

// ── Test 1 ───────────────────────────────────────────────────────────────────

#[test]
fn test_phantom_u32_encodes_to_zero_bytes() {
    let bytes = encode_to_vec(&PhantomData::<u32>).expect("encode PhantomData::<u32>");
    assert!(
        bytes.is_empty(),
        "PhantomData<u32> must encode to 0 bytes, got {}",
        bytes.len()
    );
}

// ── Test 2 ───────────────────────────────────────────────────────────────────

#[test]
fn test_phantom_string_roundtrip() {
    let original: PhantomData<String> = PhantomData;
    let bytes = encode_to_vec(&original).expect("encode PhantomData::<String>");
    let (decoded, _): (PhantomData<String>, _) =
        decode_from_slice(&bytes).expect("decode PhantomData::<String>");
    assert_eq!(original, decoded);
}

// ── Test 3 ───────────────────────────────────────────────────────────────────

#[test]
fn test_phantom_vec_u8_roundtrip() {
    let original: PhantomData<Vec<u8>> = PhantomData;
    let bytes = encode_to_vec(&original).expect("encode PhantomData::<Vec<u8>>");
    let (decoded, _): (PhantomData<Vec<u8>>, _) =
        decode_from_slice(&bytes).expect("decode PhantomData::<Vec<u8>>");
    assert_eq!(original, decoded);
}

// ── Test 4 ───────────────────────────────────────────────────────────────────

#[test]
fn test_tuple_u32_phantom_string_size() {
    let tuple_bytes =
        encode_to_vec(&(42u32, PhantomData::<String>)).expect("encode (u32, PhantomData)");
    let u32_bytes = encode_to_vec(&42u32).expect("encode u32");
    assert_eq!(
        tuple_bytes.len(),
        u32_bytes.len(),
        "PhantomData adds 0 bytes to encoded size"
    );
}

// ── Test 5 ───────────────────────────────────────────────────────────────────

#[test]
fn test_tuple_phantom_at_start() {
    let tuple_bytes =
        encode_to_vec(&(PhantomData::<u8>, 100u64)).expect("encode (PhantomData, u64)");
    let u64_bytes = encode_to_vec(&100u64).expect("encode u64");
    assert_eq!(
        tuple_bytes.len(),
        u64_bytes.len(),
        "PhantomData at tuple start adds 0 bytes"
    );
}

// ── Test 6 ───────────────────────────────────────────────────────────────────

#[test]
fn test_struct_with_phantom_field() {
    let original: Typed<String> = Typed {
        value: 42,
        _phantom: PhantomData,
    };
    let bytes = encode_to_vec(&original).expect("encode Typed<String>");
    let (decoded, _): (Typed<String>, _) = decode_from_slice(&bytes).expect("decode Typed<String>");
    assert_eq!(original, decoded);
    // Encoded size must equal that of a bare u32 (42 → 1 byte varint)
    let u32_bytes = encode_to_vec(&42u32).expect("encode u32");
    assert_eq!(bytes.len(), u32_bytes.len());
}

// ── Test 7 ───────────────────────────────────────────────────────────────────

#[test]
fn test_encoded_size_phantom_is_zero() {
    let size = encoded_size(&PhantomData::<u32>).expect("encoded_size PhantomData");
    assert_eq!(size, 0, "encoded_size of PhantomData must be 0");
}

// ── Test 8 ───────────────────────────────────────────────────────────────────

#[test]
fn test_option_phantom_some() {
    let value: Option<PhantomData<u32>> = Some(PhantomData);
    let bytes = encode_to_vec(&value).expect("encode Some(PhantomData)");
    // Option discriminant is u8 (1 byte) + 0 bytes for PhantomData = 1 byte total
    assert_eq!(bytes.len(), 1, "Some(PhantomData) must be 1 byte");
    assert_eq!(bytes[0], 1u8, "Some discriminant must be 1");
}

// ── Test 9 ───────────────────────────────────────────────────────────────────

#[test]
fn test_option_phantom_none() {
    let value: Option<PhantomData<u32>> = None;
    let bytes = encode_to_vec(&value).expect("encode None::<PhantomData<u32>>");
    assert_eq!(bytes.len(), 1, "None must be 1 byte");
    assert_eq!(bytes[0], 0u8, "None discriminant must be 0");
}

// ── Test 10 ──────────────────────────────────────────────────────────────────

#[test]
fn test_vec_phantom_empty() {
    let value: Vec<PhantomData<u32>> = Vec::new();
    let bytes = encode_to_vec(&value).expect("encode empty Vec<PhantomData>");
    // Vec length is u64 varint; varint(0) = 1 byte
    assert_eq!(
        bytes.len(),
        1,
        "empty Vec<PhantomData> must be 1 byte (length prefix)"
    );
    assert_eq!(bytes[0], 0u8, "length varint for 0 elements must be 0x00");
}

// ── Test 11 ──────────────────────────────────────────────────────────────────

#[test]
fn test_vec_phantom_three_elements() {
    let value: Vec<PhantomData<u32>> = vec![PhantomData; 3];
    let bytes = encode_to_vec(&value).expect("encode Vec<PhantomData> with 3 elements");
    // varint(3) = 1 byte, each element = 0 bytes → total = 1 byte
    assert_eq!(
        bytes.len(),
        1,
        "Vec of 3 PhantomData elements must be 1 byte (just the length prefix)"
    );
    assert_eq!(bytes[0], 3u8, "length varint for 3 elements must be 0x03");
}

// ── Test 12 ──────────────────────────────────────────────────────────────────

#[test]
fn test_phantom_with_fixed_int_encoding() {
    let cfg = oxicode::config::standard().with_fixed_int_encoding();
    let bytes =
        encode_to_vec_with_config(&PhantomData::<u32>, cfg).expect("encode PhantomData fixed");
    assert!(
        bytes.is_empty(),
        "PhantomData must still encode to 0 bytes with fixed-int config"
    );
}

// ── Test 13 ──────────────────────────────────────────────────────────────────

#[test]
fn test_phantom_with_big_endian_config() {
    let cfg = oxicode::config::standard().with_big_endian();
    let bytes =
        encode_to_vec_with_config(&PhantomData::<u32>, cfg).expect("encode PhantomData big-endian");
    assert!(
        bytes.is_empty(),
        "PhantomData must still encode to 0 bytes with big-endian config"
    );
}

// ── Test 14 ──────────────────────────────────────────────────────────────────

#[test]
fn test_triple_phantom_encodes_zero_bytes() {
    let value = (PhantomData::<u8>, PhantomData::<u16>, PhantomData::<u32>);
    let bytes = encode_to_vec(&value).expect("encode triple PhantomData tuple");
    assert!(
        bytes.is_empty(),
        "Tuple of three PhantomData must encode to 0 bytes"
    );
}

// ── Test 15 ──────────────────────────────────────────────────────────────────

#[test]
fn test_box_phantom_roundtrip() {
    let original: Box<PhantomData<u32>> = Box::new(PhantomData);
    let bytes = encode_to_vec(&original).expect("encode Box<PhantomData>");
    let (decoded, _): (Box<PhantomData<u32>>, _) =
        decode_from_slice(&bytes).expect("decode Box<PhantomData>");
    assert_eq!(original, decoded);
}

// ── Test 16 ──────────────────────────────────────────────────────────────────

#[test]
fn test_unit_type_zero_bytes() {
    let bytes = encode_to_vec(&()).expect("encode ()");
    assert!(bytes.is_empty(), "Unit type () must encode to 0 bytes");
}

// ── Test 17 ──────────────────────────────────────────────────────────────────

#[test]
fn test_two_units_zero_bytes() {
    let bytes = encode_to_vec(&((), ())).expect("encode ((), ())");
    assert!(bytes.is_empty(), "((), ()) must encode to 0 bytes");
}

// ── Test 18 ──────────────────────────────────────────────────────────────────

#[test]
fn test_u32_unit_tuple_same_size_as_u32() {
    let tuple_bytes = encode_to_vec(&(42u32, ())).expect("encode (u32, ())");
    let u32_bytes = encode_to_vec(&42u32).expect("encode u32");
    assert_eq!(
        tuple_bytes.len(),
        u32_bytes.len(),
        "Adding () to a tuple must not change encoded size"
    );
}

// ── Test 19 ──────────────────────────────────────────────────────────────────

#[test]
fn test_phantom_only_struct_zero_bytes() {
    let value = PhantomOnly {
        _marker: PhantomData,
    };
    let size = encoded_size(&value).expect("encoded_size PhantomOnly");
    assert_eq!(
        size, 0,
        "Struct with only a PhantomData field must encode to 0 bytes"
    );
}

// ── Test 20 ──────────────────────────────────────────────────────────────────

#[test]
fn test_fixed_array_phantoms_zero_bytes() {
    // Fixed-size arrays have NO length prefix (compile-time known size).
    let value: [PhantomData<u8>; 4] = [PhantomData; 4];
    let bytes = encode_to_vec(&value).expect("encode [PhantomData; 4]");
    assert!(
        bytes.is_empty(),
        "[PhantomData<u8>; 4] must encode to 0 bytes (no length prefix, no element bytes)"
    );
}

// ── Test 21 ──────────────────────────────────────────────────────────────────

#[test]
fn test_phantom_decode_consumes_zero_bytes() {
    // Decoding PhantomData from an empty slice must succeed and consume 0 bytes.
    let (decoded, bytes_consumed): (PhantomData<u32>, _) =
        decode_from_slice(&[]).expect("decode PhantomData from empty slice");
    assert_eq!(decoded, PhantomData::<u32>);
    assert_eq!(bytes_consumed, 0, "PhantomData decode must consume 0 bytes");
}

// ── Test 22 ──────────────────────────────────────────────────────────────────

#[test]
fn test_result_ok_phantom_roundtrip() {
    let original: Result<PhantomData<u32>, String> = Ok(PhantomData);
    let bytes = encode_to_vec(&original).expect("encode Ok(PhantomData)");
    // Result Ok discriminant is u32 varint-encoded; varint(0) = 1 byte.
    // PhantomData adds 0 bytes → total = 1 byte.
    assert_eq!(bytes.len(), 1, "Ok(PhantomData) must encode to 1 byte");
    // Roundtrip
    let cfg = oxicode::config::standard();
    let (decoded, _): (Result<PhantomData<u32>, String>, _) =
        decode_from_slice_with_config(&bytes, cfg).expect("decode Ok(PhantomData)");
    assert_eq!(original, decoded);
}
