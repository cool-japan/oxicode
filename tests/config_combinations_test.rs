//! Tests for OxiCode configuration combinations and cross-config compatibility.

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

#[derive(Debug, PartialEq, Encode, Decode)]
struct TwoFields {
    x: u32,
    y: u64,
}

#[derive(Debug, PartialEq, Encode, Decode)]
#[allow(dead_code)]
struct SingleU32 {
    value: u32,
}

// Test 1: Standard config u32 roundtrip
#[test]
fn test_standard_u32_roundtrip() {
    let original: u32 = 12345;
    let encoded = encode_to_vec(&original).expect("encode failed");
    let (decoded, consumed): (u32, _) = decode_from_slice(&encoded).expect("decode failed");
    assert_eq!(original, decoded);
    assert_eq!(consumed, encoded.len());
}

// Test 2: Fixed-int config u32 — verify 4 bytes exact size
#[test]
fn test_fixed_int_u32_exact_size() {
    let cfg = config::standard().with_fixed_int_encoding();
    let encoded = encode_to_vec_with_config(&42u32, cfg).expect("encode failed");
    assert_eq!(encoded.len(), 4);
    let (decoded, _): (u32, _) =
        decode_from_slice_with_config(&encoded, cfg).expect("decode failed");
    assert_eq!(decoded, 42u32);
}

// Test 3: Big-endian config u32 roundtrip
#[test]
fn test_big_endian_u32_roundtrip() {
    let cfg = config::standard().with_big_endian();
    let original: u32 = 99999;
    let encoded = encode_to_vec_with_config(&original, cfg).expect("encode failed");
    let (decoded, consumed): (u32, _) =
        decode_from_slice_with_config(&encoded, cfg).expect("decode failed");
    assert_eq!(original, decoded);
    assert_eq!(consumed, encoded.len());
}

// Test 4: Fixed-int + roundtrip of u64 — verify 8 bytes
#[test]
fn test_fixed_int_u64_exact_size() {
    let cfg = config::standard().with_fixed_int_encoding();
    let original: u64 = 1_000_000_000_000u64;
    let encoded = encode_to_vec_with_config(&original, cfg).expect("encode failed");
    assert_eq!(encoded.len(), 8);
    let (decoded, _): (u64, _) =
        decode_from_slice_with_config(&encoded, cfg).expect("decode failed");
    assert_eq!(decoded, original);
}

// Test 5: Standard config String roundtrip
#[test]
fn test_standard_string_roundtrip() {
    let original = String::from("hello, oxicode!");
    let encoded = encode_to_vec(&original).expect("encode failed");
    let (decoded, consumed): (String, _) = decode_from_slice(&encoded).expect("decode failed");
    assert_eq!(original, decoded);
    assert_eq!(consumed, encoded.len());
}

// Test 6: Fixed-int config with String (strings use varint length prefix regardless)
#[test]
fn test_fixed_int_string_roundtrip() {
    let cfg = config::standard().with_fixed_int_encoding();
    let original = String::from("fixed int string");
    let encoded = encode_to_vec_with_config(&original, cfg).expect("encode failed");
    let (decoded, _): (String, _) =
        decode_from_slice_with_config(&encoded, cfg).expect("decode failed");
    assert_eq!(original, decoded);
}

// Test 7: Big-endian config with String
#[test]
fn test_big_endian_string_roundtrip() {
    let cfg = config::standard().with_big_endian();
    let original = String::from("big endian string");
    let encoded = encode_to_vec_with_config(&original, cfg).expect("encode failed");
    let (decoded, _): (String, _) =
        decode_from_slice_with_config(&encoded, cfg).expect("decode failed");
    assert_eq!(original, decoded);
}

// Test 8: Limit config — small data fits within limit
#[test]
fn test_limit_config_small_data_fits() {
    // Encode u8 value 42; in standard varint that is 1 byte.
    // A limit of 64 bytes is more than enough.
    let cfg = config::standard().with_limit::<64>();
    let encoded = encode_to_vec_with_config(&42u8, cfg).expect("encode failed");
    let (decoded, _): (u8, _) =
        decode_from_slice_with_config(&encoded, cfg).expect("decode failed");
    assert_eq!(decoded, 42u8);
}

// Test 9: Limit config — data exceeds limit returns error
#[test]
fn test_limit_config_data_exceeds_limit_returns_error() {
    // Encode a Vec<u32> with 100 elements; this will be many bytes, exceeding limit of 4.
    let large_vec: Vec<u32> = (0u32..100).collect();
    let std_encoded = encode_to_vec(&large_vec).expect("encode failed");
    // Make sure the standard-encoded data is indeed larger than 4 bytes.
    assert!(std_encoded.len() > 4);
    // Now attempt to decode with a limit of 4 bytes — should fail.
    let cfg_limit = config::standard().with_limit::<4>();
    let result: Result<(Vec<u32>, _), _> = decode_from_slice_with_config(&std_encoded, cfg_limit);
    assert!(result.is_err());
}

// Test 10: Standard config Vec<u32> roundtrip
#[test]
fn test_standard_vec_u32_roundtrip() {
    let original: Vec<u32> = vec![1, 2, 3, 4, 5];
    let encoded = encode_to_vec(&original).expect("encode failed");
    let (decoded, consumed): (Vec<u32>, _) = decode_from_slice(&encoded).expect("decode failed");
    assert_eq!(original, decoded);
    assert_eq!(consumed, encoded.len());
}

// Test 11: Fixed-int config Vec<u32>
#[test]
fn test_fixed_int_vec_u32_roundtrip() {
    let cfg = config::standard().with_fixed_int_encoding();
    let original: Vec<u32> = vec![10, 20, 30];
    let encoded = encode_to_vec_with_config(&original, cfg).expect("encode failed");
    let (decoded, _): (Vec<u32>, _) =
        decode_from_slice_with_config(&encoded, cfg).expect("decode failed");
    assert_eq!(original, decoded);
}

// Test 12: Big-endian config Vec<u32>
#[test]
fn test_big_endian_vec_u32_roundtrip() {
    let cfg = config::standard().with_big_endian();
    let original: Vec<u32> = vec![100, 200, 300];
    let encoded = encode_to_vec_with_config(&original, cfg).expect("encode failed");
    let (decoded, _): (Vec<u32>, _) =
        decode_from_slice_with_config(&encoded, cfg).expect("decode failed");
    assert_eq!(original, decoded);
}

// Test 13: u8 is always 1 byte regardless of fixed-int vs standard config
#[test]
fn test_u8_always_one_byte() {
    let std_encoded = encode_to_vec(&255u8).expect("encode failed");
    assert_eq!(std_encoded.len(), 1);

    let cfg = config::standard().with_fixed_int_encoding();
    let fixed_encoded = encode_to_vec_with_config(&255u8, cfg).expect("encode failed");
    assert_eq!(fixed_encoded.len(), 1);
}

// Test 14: Standard vs fixed-int: encoded sizes differ for u32
#[test]
fn test_standard_vs_fixed_int_u32_sizes_differ() {
    // A small u32 (fits in 1 byte with varint) vs fixed 4 bytes.
    let val: u32 = 1;
    let std_encoded = encode_to_vec(&val).expect("encode failed");
    let fixed_cfg = config::standard().with_fixed_int_encoding();
    let fixed_encoded = encode_to_vec_with_config(&val, fixed_cfg).expect("encode failed");
    // Standard varint: value 1 -> 1 byte. Fixed: 4 bytes.
    assert_eq!(std_encoded.len(), 1);
    assert_eq!(fixed_encoded.len(), 4);
    assert!(std_encoded.len() < fixed_encoded.len());
}

// Test 15: Standard config with Option<u32> Some
#[test]
fn test_standard_option_u32_some_roundtrip() {
    let original: Option<u32> = Some(9999);
    let encoded = encode_to_vec(&original).expect("encode failed");
    let (decoded, consumed): (Option<u32>, _) = decode_from_slice(&encoded).expect("decode failed");
    assert_eq!(original, decoded);
    assert_eq!(consumed, encoded.len());
}

// Test 16: Standard config with Option<u32> None
#[test]
fn test_standard_option_u32_none_roundtrip() {
    let original: Option<u32> = None;
    let encoded = encode_to_vec(&original).expect("encode failed");
    let (decoded, consumed): (Option<u32>, _) = decode_from_slice(&encoded).expect("decode failed");
    assert_eq!(original, decoded);
    assert_eq!(consumed, encoded.len());
}

// Test 17: Fixed-int config with bool — still 1 byte
#[test]
fn test_fixed_int_bool_one_byte() {
    let cfg = config::standard().with_fixed_int_encoding();
    let true_encoded = encode_to_vec_with_config(&true, cfg).expect("encode failed");
    let false_encoded = encode_to_vec_with_config(&false, cfg).expect("encode failed");
    assert_eq!(true_encoded.len(), 1);
    assert_eq!(false_encoded.len(), 1);

    let (decoded_true, _): (bool, _) =
        decode_from_slice_with_config(&true_encoded, cfg).expect("decode failed");
    let (decoded_false, _): (bool, _) =
        decode_from_slice_with_config(&false_encoded, cfg).expect("decode failed");
    assert!(decoded_true);
    assert!(!decoded_false);
}

// Test 18: Big-endian u32: verify byte order is big-endian in encoded bytes
#[test]
fn test_big_endian_u32_byte_order() {
    let cfg = config::standard()
        .with_big_endian()
        .with_fixed_int_encoding();
    // Use a value whose big-endian representation is unambiguous: 0x01020304
    let val: u32 = 0x01020304u32;
    let encoded = encode_to_vec_with_config(&val, cfg).expect("encode failed");
    assert_eq!(encoded.len(), 4);
    assert_eq!(encoded[0], 0x01);
    assert_eq!(encoded[1], 0x02);
    assert_eq!(encoded[2], 0x03);
    assert_eq!(encoded[3], 0x04);
}

// Test 19: Little-endian (standard) u32: verify byte order is little-endian
#[test]
fn test_little_endian_u32_byte_order() {
    let cfg = config::standard().with_fixed_int_encoding();
    // Use the same value as test 18: 0x01020304
    let val: u32 = 0x01020304u32;
    let encoded = encode_to_vec_with_config(&val, cfg).expect("encode failed");
    assert_eq!(encoded.len(), 4);
    // Little-endian: least significant byte first
    assert_eq!(encoded[0], 0x04);
    assert_eq!(encoded[1], 0x03);
    assert_eq!(encoded[2], 0x02);
    assert_eq!(encoded[3], 0x01);
}

// Test 20: Standard config with struct containing multiple fields
#[test]
fn test_standard_struct_roundtrip() {
    let original = TwoFields {
        x: 42,
        y: 1_000_000,
    };
    let encoded = encode_to_vec(&original).expect("encode failed");
    let (decoded, consumed): (TwoFields, _) = decode_from_slice(&encoded).expect("decode failed");
    assert_eq!(original, decoded);
    assert_eq!(consumed, encoded.len());
}

// Test 21: Fixed-int config with struct
#[test]
fn test_fixed_int_struct_roundtrip() {
    let cfg = config::standard().with_fixed_int_encoding();
    let original = TwoFields {
        x: 7,
        y: 999_999_999,
    };
    let encoded = encode_to_vec_with_config(&original, cfg).expect("encode failed");
    // With fixed-int: u32 = 4 bytes, u64 = 8 bytes -> total 12 bytes
    assert_eq!(encoded.len(), 12);
    let (decoded, _): (TwoFields, _) =
        decode_from_slice_with_config(&encoded, cfg).expect("decode failed");
    assert_eq!(original, decoded);
}

// Test 22: Limit config just at boundary
// A string of N bytes claims exactly N bytes via claim_bytes_read.
// Encoding a 1-byte string "a" produces 2 bytes (1-byte varint length + 1 byte content).
// Decoding it with limit::<1>() claims 1 byte of content — succeeds.
// Decoding a 2-byte string "ab" with limit::<1>() claims 2 bytes — fails.
#[test]
fn test_limit_config_boundary() {
    // Part A: "a" has 1 byte of content; limit::<1>() must succeed.
    let one_char_encoded = encode_to_vec(&String::from("a")).expect("encode failed");
    // Encoding: 1-byte varint(1) + 1 byte 'a' = 2 bytes total.
    assert_eq!(
        one_char_encoded.len(),
        2,
        "1-char string should encode to 2 bytes"
    );

    let cfg_limit_1 = config::standard().with_limit::<1>();
    let result_ok: Result<(String, _), _> =
        decode_from_slice_with_config(&one_char_encoded, cfg_limit_1);
    assert!(
        result_ok.is_ok(),
        "decode with limit::<1> should succeed for 1-byte content string"
    );
    let (val, _) = result_ok.expect("decode succeeded");
    assert_eq!(val, "a");

    // Part B: "ab" has 2 bytes of content; limit::<1>() must fail because claim_bytes_read(2) > 1.
    let two_char_encoded = encode_to_vec(&String::from("ab")).expect("encode failed");
    // Encoding: 1-byte varint(2) + 2 bytes "ab" = 3 bytes total.
    assert_eq!(
        two_char_encoded.len(),
        3,
        "2-char string should encode to 3 bytes"
    );

    let result_err: Result<(String, _), _> =
        decode_from_slice_with_config(&two_char_encoded, cfg_limit_1);
    assert!(
        result_err.is_err(),
        "decode with limit::<1> should fail for 2-byte content string"
    );
}
