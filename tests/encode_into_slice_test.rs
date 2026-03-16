//! Tests for encode_into_slice and related fixed-buffer encode/decode APIs.
//!
//! Note: tests for encode_to_fixed_array, decode_value, and encode_bytes are in
//! tests/utility_api_test.rs. This file focuses on encode_into_slice, the
//! slice/config decode variants, and borrow_decode_from_slice.

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
    borrow_decode_from_slice, config, decode_from_slice, decode_from_slice_with_config, Decode,
    Encode,
};

#[derive(Debug, PartialEq, Encode, Decode)]
struct Point {
    x: f32,
    y: f32,
    z: f32,
}

// ---------------------------------------------------------------------------
// encode_into_slice basic tests
// ---------------------------------------------------------------------------

#[test]
fn test_encode_into_slice_u32_roundtrip() {
    let val = 42u32;
    let mut buf = [0u8; 16];
    let n = oxicode::encode_into_slice(val, &mut buf, config::standard()).expect("encode");
    assert!(n > 0);
    assert!(n <= 16);
    let (dec, consumed): (u32, _) = decode_from_slice(&buf[..n]).expect("decode");
    assert_eq!(val, dec);
    assert_eq!(consumed, n);
}

#[test]
fn test_encode_into_slice_struct_roundtrip() {
    let p = Point {
        x: 1.0,
        y: 2.0,
        z: 3.0,
    };
    let expected = Point {
        x: 1.0,
        y: 2.0,
        z: 3.0,
    };
    let mut buf = [0u8; 64];
    let n = oxicode::encode_into_slice(p, &mut buf, config::standard()).expect("encode");
    assert!(n > 0);
    let (dec, consumed): (Point, _) = decode_from_slice(&buf[..n]).expect("decode");
    assert_eq!(expected, dec);
    assert_eq!(consumed, n);
}

#[test]
fn test_encode_into_slice_buffer_too_small() {
    // A large string cannot fit in 4 bytes
    let val = "this string is definitely longer than four bytes".to_string();
    let mut buf = [0u8; 4];
    let result = oxicode::encode_into_slice(val, &mut buf, config::standard());
    assert!(result.is_err(), "should fail when buffer is too small");
}

#[test]
fn test_encode_into_slice_exact_fit_u8() {
    // u8 encodes to exactly 1 byte
    let val: u8 = 200;
    let mut buf = [0u8; 1];
    let n = oxicode::encode_into_slice(val, &mut buf, config::standard()).expect("encode");
    assert_eq!(n, 1);
    let (dec, _): (u8, _) = decode_from_slice(&buf[..n]).expect("decode");
    assert_eq!(val, dec);
}

#[test]
fn test_encode_into_slice_bytes_written_accurate() {
    let val = 0xDEAD_BEEFu32;
    let mut buf = [0u8; 32];
    // Fill buffer with sentinel values to verify only `n` bytes are written
    buf.fill(0xFF);
    let n = oxicode::encode_into_slice(val, &mut buf, config::standard()).expect("encode");
    assert!(n > 0);
    assert!(n < 32);
    // Bytes after n should still be 0xFF (untouched)
    assert!(
        buf[n..].iter().all(|&b| b == 0xFF),
        "bytes beyond written count should be untouched"
    );
}

#[test]
fn test_encode_into_slice_vec_u8() {
    let val: Vec<u8> = vec![10, 20, 30, 40, 50];
    let expected = val.clone();
    let mut buf = [0u8; 64];
    let n = oxicode::encode_into_slice(val, &mut buf, config::standard()).expect("encode");
    assert!(n > 0);
    let (dec, consumed): (Vec<u8>, _) = decode_from_slice(&buf[..n]).expect("decode");
    assert_eq!(expected, dec);
    assert_eq!(consumed, n);
}

#[test]
fn test_encode_into_slice_sequential_values() {
    // Encode two values sequentially into the same buffer at different offsets
    let val1 = 100u32;
    let val2 = 200u64;
    let mut buf = [0u8; 64];
    let n1 = oxicode::encode_into_slice(val1, &mut buf, config::standard()).expect("encode val1");
    let n2 =
        oxicode::encode_into_slice(val2, &mut buf[n1..], config::standard()).expect("encode val2");
    assert!(n1 > 0);
    assert!(n2 > 0);

    let (dec1, _): (u32, _) = decode_from_slice(&buf[..n1]).expect("decode val1");
    let (dec2, _): (u64, _) = decode_from_slice(&buf[n1..n1 + n2]).expect("decode val2");
    assert_eq!(val1, dec1);
    assert_eq!(val2, dec2);
}

#[test]
fn test_encode_into_slice_with_fixed_int_encoding() {
    let val = 1u32;
    let cfg = config::standard().with_fixed_int_encoding();
    let mut buf = [0u8; 16];
    let n = oxicode::encode_into_slice(val, &mut buf, cfg).expect("encode");
    // fixed int encoding: u32 is always 4 bytes
    assert_eq!(n, 4);
    let (dec, _): (u32, _) = decode_from_slice_with_config(&buf[..n], cfg).expect("decode");
    assert_eq!(val, dec);
}

// ---------------------------------------------------------------------------
// decode_from_slice_with_config tests
// ---------------------------------------------------------------------------

#[test]
fn test_decode_from_slice_with_config_standard() {
    let val = 0x01020304u32;
    let std_cfg = config::standard();
    let enc = oxicode::encode_to_vec_with_config(&val, std_cfg).expect("encode");
    let (dec, consumed): (u32, _) = decode_from_slice_with_config(&enc, std_cfg).expect("decode");
    assert_eq!(val, dec);
    assert_eq!(consumed, enc.len());
}

#[test]
fn test_decode_from_slice_with_config_legacy() {
    let val = 0x01020304u32;
    let legacy = config::legacy();
    let enc = oxicode::encode_to_vec_with_config(&val, legacy).expect("encode");
    let (dec, consumed): (u32, _) = decode_from_slice_with_config(&enc, legacy).expect("decode");
    assert_eq!(val, dec);
    assert_eq!(consumed, enc.len());
}

#[test]
fn test_decode_from_slice_with_config_vec_standard() {
    let val: Vec<u8> = vec![1, 2, 3, 4, 5, 6, 7, 8];
    let std_cfg = config::standard();
    let enc = oxicode::encode_to_vec_with_config(&val, std_cfg).expect("encode");
    let (dec, consumed): (Vec<u8>, _) =
        decode_from_slice_with_config(&enc, std_cfg).expect("decode");
    assert_eq!(val, dec);
    assert_eq!(consumed, enc.len());
}

#[test]
fn test_decode_from_slice_partial_buffer() {
    // Encode a u32 then append extra bytes; consumed should equal only the encoded bytes
    let val = 99u32;
    let mut enc = oxicode::encode_to_vec(&val).expect("encode");
    let actual_len = enc.len();
    enc.extend_from_slice(&[0xAA, 0xBB, 0xCC]); // trailing junk
    let (dec, consumed): (u32, _) = decode_from_slice(&enc).expect("decode");
    assert_eq!(val, dec);
    assert_eq!(
        consumed, actual_len,
        "consumed should equal encoded bytes, not total buffer"
    );
}

// ---------------------------------------------------------------------------
// borrow_decode_from_slice tests
// ---------------------------------------------------------------------------

#[test]
fn test_borrow_decode_from_slice_str() {
    // Encode a String, borrow-decode back as &str (zero-copy)
    let data = String::from("hello borrow decode");
    let enc = oxicode::encode_to_vec(&data).expect("encode");
    let (dec, consumed): (&str, _) = borrow_decode_from_slice(&enc).expect("borrow_decode");
    assert_eq!(dec, data.as_str());
    assert_eq!(consumed, enc.len());
}

#[test]
fn test_borrow_decode_from_slice_bytes() {
    // Encode a Vec<u8>, borrow-decode back as &[u8]
    let data: Vec<u8> = vec![0xDE, 0xAD, 0xBE, 0xEF];
    let enc = oxicode::encode_to_vec(&data).expect("encode");
    let (dec, _): (&[u8], _) = borrow_decode_from_slice(&enc).expect("borrow_decode");
    assert_eq!(dec, data.as_slice());
}

#[test]
fn test_borrow_decode_from_slice_string_owned() {
    // String owns its data so borrow_decode works for owned String too
    let val = String::from("owned string via borrow_decode");
    let enc = oxicode::encode_to_vec(&val).expect("encode");
    let (dec, consumed): (String, _) = borrow_decode_from_slice(&enc).expect("borrow_decode");
    assert_eq!(val, dec);
    assert_eq!(consumed, enc.len());
}

// ---------------------------------------------------------------------------
// encode_to_fixed_array_with_config tests (not duplicating utility_api_test.rs)
// ---------------------------------------------------------------------------

#[test]
fn test_encode_to_fixed_array_with_legacy_config() {
    let val = 0xABCDu32;
    let legacy = config::legacy();
    let (arr, n): ([u8; 16], _) =
        oxicode::encode_to_fixed_array_with_config(&val, legacy).expect("encode");
    let (dec, _): (u32, _) = decode_from_slice_with_config(&arr[..n], legacy).expect("decode");
    assert_eq!(val, dec);
}

#[test]
fn test_encode_to_fixed_array_with_fixed_int_encoding() {
    let val = 1u32;
    let cfg = config::standard().with_fixed_int_encoding();
    let (arr, n): ([u8; 16], _) =
        oxicode::encode_to_fixed_array_with_config(&val, cfg).expect("encode");
    // fixed int encoding: u32 is always 4 bytes
    assert_eq!(n, 4);
    let (dec, _): (u32, _) = decode_from_slice_with_config(&arr[..n], cfg).expect("decode");
    assert_eq!(val, dec);
}
