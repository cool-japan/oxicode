//! Advanced tests for tuple struct (newtype and multi-field) serialization.

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
    encode_to_vec_with_config, BorrowDecode, Decode, Encode,
};

// ── Type definitions ──────────────────────────────────────────────────────────

#[derive(Encode, Decode, BorrowDecode, PartialEq, Debug)]
struct NewtypeU32(u32);

#[derive(Encode, Decode, BorrowDecode, PartialEq, Debug)]
struct NewtypeString(String);

#[derive(Encode, Decode, BorrowDecode, PartialEq, Debug)]
struct Pair(u32, u64);

#[derive(Encode, Decode, BorrowDecode, PartialEq, Debug)]
struct Triple(u8, u16, u32);

#[derive(Encode, Decode, BorrowDecode, PartialEq, Debug)]
struct NewtypeVec(Vec<u8>);

#[derive(Encode, Decode, BorrowDecode, PartialEq, Debug)]
struct Nested(NewtypeU32, NewtypeString);

// ── Test 1: NewtypeU32(0) roundtrip ──────────────────────────────────────────

#[test]
fn test_newtype_u32_zero_roundtrip() {
    let original = NewtypeU32(0);
    let encoded = encode_to_vec(&original).expect("encode NewtypeU32(0)");
    let (decoded, _): (NewtypeU32, _) = decode_from_slice(&encoded).expect("decode NewtypeU32(0)");
    assert_eq!(original, decoded);
}

// ── Test 2: NewtypeU32(u32::MAX) roundtrip ───────────────────────────────────

#[test]
fn test_newtype_u32_max_roundtrip() {
    let original = NewtypeU32(u32::MAX);
    let encoded = encode_to_vec(&original).expect("encode NewtypeU32(u32::MAX)");
    let (decoded, _): (NewtypeU32, _) =
        decode_from_slice(&encoded).expect("decode NewtypeU32(u32::MAX)");
    assert_eq!(original, decoded);
}

// ── Test 3: NewtypeU32 consumed == encoded.len() ─────────────────────────────

#[test]
fn test_newtype_u32_consumed_equals_encoded_len() {
    let original = NewtypeU32(42);
    let encoded = encode_to_vec(&original).expect("encode NewtypeU32(42)");
    let (_decoded, consumed): (NewtypeU32, _) =
        decode_from_slice(&encoded).expect("decode NewtypeU32(42)");
    assert_eq!(consumed, encoded.len());
}

// ── Test 4: NewtypeU32 encoded size same as raw u32 ──────────────────────────

#[test]
fn test_newtype_u32_encoded_size_same_as_raw_u32() {
    let value: u32 = 100;
    let raw_encoded = encode_to_vec(&value).expect("encode raw u32");
    let newtype_encoded = encode_to_vec(&NewtypeU32(value)).expect("encode NewtypeU32");
    assert_eq!(
        raw_encoded.len(),
        newtype_encoded.len(),
        "NewtypeU32 should encode identically to raw u32"
    );
}

// ── Test 5: NewtypeString("hello") roundtrip ─────────────────────────────────

#[test]
fn test_newtype_string_hello_roundtrip() {
    let original = NewtypeString("hello".to_string());
    let encoded = encode_to_vec(&original).expect("encode NewtypeString(hello)");
    let (decoded, _): (NewtypeString, _) =
        decode_from_slice(&encoded).expect("decode NewtypeString(hello)");
    assert_eq!(original, decoded);
}

// ── Test 6: NewtypeString("") empty string ───────────────────────────────────

#[test]
fn test_newtype_string_empty_roundtrip() {
    let original = NewtypeString(String::new());
    let encoded = encode_to_vec(&original).expect("encode NewtypeString(\"\")");
    let (decoded, _): (NewtypeString, _) =
        decode_from_slice(&encoded).expect("decode NewtypeString(\"\")");
    assert_eq!(original, decoded);
}

// ── Test 7: Pair(1, 2) roundtrip ─────────────────────────────────────────────

#[test]
fn test_pair_basic_roundtrip() {
    let original = Pair(1, 2);
    let encoded = encode_to_vec(&original).expect("encode Pair(1, 2)");
    let (decoded, _): (Pair, _) = decode_from_slice(&encoded).expect("decode Pair(1, 2)");
    assert_eq!(original, decoded);
}

// ── Test 8: Pair(u32::MAX, u64::MAX) roundtrip ───────────────────────────────

#[test]
fn test_pair_max_values_roundtrip() {
    let original = Pair(u32::MAX, u64::MAX);
    let encoded = encode_to_vec(&original).expect("encode Pair(u32::MAX, u64::MAX)");
    let (decoded, _): (Pair, _) =
        decode_from_slice(&encoded).expect("decode Pair(u32::MAX, u64::MAX)");
    assert_eq!(original, decoded);
}

// ── Test 9: Triple(1, 2, 3) roundtrip ────────────────────────────────────────

#[test]
fn test_triple_basic_roundtrip() {
    let original = Triple(1, 2, 3);
    let encoded = encode_to_vec(&original).expect("encode Triple(1, 2, 3)");
    let (decoded, _): (Triple, _) = decode_from_slice(&encoded).expect("decode Triple(1, 2, 3)");
    assert_eq!(original, decoded);
}

// ── Test 10: Vec<NewtypeU32> roundtrip ───────────────────────────────────────

#[test]
fn test_vec_of_newtype_u32_roundtrip() {
    let original = vec![NewtypeU32(10), NewtypeU32(20), NewtypeU32(30)];
    let encoded = encode_to_vec(&original).expect("encode Vec<NewtypeU32>");
    let (decoded, _): (Vec<NewtypeU32>, _) =
        decode_from_slice(&encoded).expect("decode Vec<NewtypeU32>");
    assert_eq!(original, decoded);
}

// ── Test 11: Option<NewtypeU32> Some ─────────────────────────────────────────

#[test]
fn test_option_newtype_u32_some_roundtrip() {
    let original: Option<NewtypeU32> = Some(NewtypeU32(99));
    let encoded = encode_to_vec(&original).expect("encode Option<NewtypeU32> Some");
    let (decoded, _): (Option<NewtypeU32>, _) =
        decode_from_slice(&encoded).expect("decode Option<NewtypeU32> Some");
    assert_eq!(original, decoded);
}

// ── Test 12: Option<NewtypeU32> None ─────────────────────────────────────────

#[test]
fn test_option_newtype_u32_none_roundtrip() {
    let original: Option<NewtypeU32> = None;
    let encoded = encode_to_vec(&original).expect("encode Option<NewtypeU32> None");
    let (decoded, _): (Option<NewtypeU32>, _) =
        decode_from_slice(&encoded).expect("decode Option<NewtypeU32> None");
    assert_eq!(original, decoded);
}

// ── Test 13: NewtypeVec(vec![1,2,3]) roundtrip ───────────────────────────────

#[test]
fn test_newtype_vec_basic_roundtrip() {
    let original = NewtypeVec(vec![1u8, 2, 3]);
    let encoded = encode_to_vec(&original).expect("encode NewtypeVec([1,2,3])");
    let (decoded, _): (NewtypeVec, _) =
        decode_from_slice(&encoded).expect("decode NewtypeVec([1,2,3])");
    assert_eq!(original, decoded);
}

// ── Test 14: NewtypeVec(vec![]) empty ────────────────────────────────────────

#[test]
fn test_newtype_vec_empty_roundtrip() {
    let original = NewtypeVec(vec![]);
    let encoded = encode_to_vec(&original).expect("encode NewtypeVec([])");
    let (decoded, _): (NewtypeVec, _) = decode_from_slice(&encoded).expect("decode NewtypeVec([])");
    assert_eq!(original, decoded);
}

// ── Test 15: Nested roundtrip ─────────────────────────────────────────────────

#[test]
fn test_nested_roundtrip() {
    let original = Nested(NewtypeU32(7), NewtypeString("world".to_string()));
    let encoded = encode_to_vec(&original).expect("encode Nested");
    let (decoded, _): (Nested, _) = decode_from_slice(&encoded).expect("decode Nested");
    assert_eq!(original, decoded);
}

// ── Test 16: Fixed-int config with NewtypeU32 ────────────────────────────────

#[test]
fn test_newtype_u32_fixed_int_config_roundtrip() {
    let cfg = config::standard().with_fixed_int_encoding();
    let original = NewtypeU32(12345);
    let encoded = encode_to_vec_with_config(&original, cfg).expect("encode NewtypeU32 fixed-int");
    let (decoded, consumed): (NewtypeU32, _) =
        decode_from_slice_with_config(&encoded, cfg).expect("decode NewtypeU32 fixed-int");
    assert_eq!(original, decoded);
    assert_eq!(consumed, encoded.len());
    // Fixed-int encoding: u32 always uses 4 bytes
    assert_eq!(encoded.len(), 4, "fixed-int u32 must be 4 bytes");
}

// ── Test 17: Big-endian config with NewtypeU32 ───────────────────────────────

#[test]
fn test_newtype_u32_big_endian_config_roundtrip() {
    let cfg = config::standard()
        .with_big_endian()
        .with_fixed_int_encoding();
    let original = NewtypeU32(0x0102_0304);
    let encoded = encode_to_vec_with_config(&original, cfg).expect("encode NewtypeU32 big-endian");
    let (decoded, consumed): (NewtypeU32, _) =
        decode_from_slice_with_config(&encoded, cfg).expect("decode NewtypeU32 big-endian");
    assert_eq!(original, decoded);
    assert_eq!(consumed, encoded.len());
    // Verify big-endian byte order: 0x0102_0304 → [0x01, 0x02, 0x03, 0x04]
    assert_eq!(encoded[0], 0x01);
    assert_eq!(encoded[1], 0x02);
    assert_eq!(encoded[2], 0x03);
    assert_eq!(encoded[3], 0x04);
}

// ── Test 18: [NewtypeU32; 3] array roundtrip ─────────────────────────────────

#[test]
fn test_newtype_u32_array_roundtrip() {
    let original = [NewtypeU32(1), NewtypeU32(2), NewtypeU32(3)];
    let encoded = encode_to_vec(&original).expect("encode [NewtypeU32; 3]");
    let (decoded, consumed): ([NewtypeU32; 3], _) =
        decode_from_slice(&encoded).expect("decode [NewtypeU32; 3]");
    assert_eq!(original, decoded);
    assert_eq!(consumed, encoded.len());
}

// ── Test 19: Pair consumed == encoded.len() ──────────────────────────────────

#[test]
fn test_pair_consumed_equals_encoded_len() {
    let original = Pair(555, 999_999);
    let encoded = encode_to_vec(&original).expect("encode Pair(555, 999_999)");
    let (_decoded, consumed): (Pair, _) =
        decode_from_slice(&encoded).expect("decode Pair(555, 999_999)");
    assert_eq!(consumed, encoded.len());
}

// ── Test 20: Triple consumed == encoded.len() ────────────────────────────────

#[test]
fn test_triple_consumed_equals_encoded_len() {
    let original = Triple(255, 1000, 70000);
    let encoded = encode_to_vec(&original).expect("encode Triple(255, 1000, 70000)");
    let (_decoded, consumed): (Triple, _) =
        decode_from_slice(&encoded).expect("decode Triple(255, 1000, 70000)");
    assert_eq!(consumed, encoded.len());
}

// ── Test 21: Vec<Pair> roundtrip ─────────────────────────────────────────────

#[test]
fn test_vec_of_pair_roundtrip() {
    let original = vec![Pair(0, 0), Pair(1, 2), Pair(u32::MAX, u64::MAX)];
    let encoded = encode_to_vec(&original).expect("encode Vec<Pair>");
    let (decoded, consumed): (Vec<Pair>, _) =
        decode_from_slice(&encoded).expect("decode Vec<Pair>");
    assert_eq!(original, decoded);
    assert_eq!(consumed, encoded.len());
}

// ── Test 22: Option<Nested> Some value ───────────────────────────────────────

#[test]
fn test_option_nested_some_roundtrip() {
    let original: Option<Nested> =
        Some(Nested(NewtypeU32(42), NewtypeString("oxicode".to_string())));
    let encoded = encode_to_vec(&original).expect("encode Option<Nested> Some");
    let (decoded, consumed): (Option<Nested>, _) =
        decode_from_slice(&encoded).expect("decode Option<Nested> Some");
    assert_eq!(original, decoded);
    assert_eq!(consumed, encoded.len());
}
