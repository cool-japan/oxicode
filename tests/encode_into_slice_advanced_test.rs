//! Advanced tests for `encode_into_slice` and related slice APIs.

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
use oxicode::{config, decode_from_slice, encode_into_slice, encode_to_vec, Decode, Encode};

// ---------------------------------------------------------------------------
// Local types used across multiple tests
// ---------------------------------------------------------------------------

#[derive(Debug, PartialEq, Encode, Decode)]
struct Color {
    r: u8,
    g: u8,
    b: u8,
}

#[derive(Debug, PartialEq, Encode, Decode)]
enum Direction {
    North,
    South,
    East,
    West,
}

#[derive(Debug, PartialEq, Encode, Decode)]
struct Unit;

// ---------------------------------------------------------------------------
// 22 top-level tests
// ---------------------------------------------------------------------------

/// Test 1: basic u32 encode into pre-allocated buffer
#[test]
fn test_encode_into_slice_basic_u32() {
    let val: u32 = 42;
    let mut buf = [0u8; 16];
    let n = encode_into_slice(val, &mut buf, config::standard()).expect("encode basic u32");
    assert!(n > 0, "expected bytes written > 0");
    assert!(n <= 16, "expected bytes written <= buf len");
}

/// Test 2: encode_into_slice returns accurate bytes-written count
#[test]
fn test_encode_into_slice_returns_bytes_written() {
    let val: u32 = 12345;
    let mut buf = [0u8; 16];
    let n = encode_into_slice(val, &mut buf, config::standard()).expect("encode returns bytes");
    let expected = encode_to_vec(&val).expect("encode_to_vec");
    assert_eq!(
        n,
        expected.len(),
        "bytes written must equal encode_to_vec length"
    );
}

/// Test 3: String roundtrip via encode_into_slice / decode_from_slice
#[test]
fn test_encode_into_slice_string_roundtrip() {
    let val = String::from("hello, oxicode slice!");
    let expected = val.clone();
    let mut buf = [0u8; 64];
    let n = encode_into_slice(val, &mut buf, config::standard()).expect("encode string");
    let (dec, consumed): (String, _) = decode_from_slice(&buf[..n]).expect("decode string");
    assert_eq!(expected, dec);
    assert_eq!(consumed, n);
}

/// Test 4: Vec<u8> roundtrip via encode_into_slice / decode_from_slice
#[test]
fn test_encode_into_slice_vec_u8_roundtrip() {
    let val: Vec<u8> = vec![10, 20, 30, 40, 50, 60, 70, 80];
    let expected = val.clone();
    let mut buf = [0u8; 64];
    let n = encode_into_slice(val, &mut buf, config::standard()).expect("encode vec u8");
    let (dec, consumed): (Vec<u8>, _) = decode_from_slice(&buf[..n]).expect("decode vec u8");
    assert_eq!(expected, dec);
    assert_eq!(consumed, n);
}

/// Test 5: exact-size buffer succeeds
#[test]
fn test_encode_into_slice_exact_size_buffer() {
    let val: u64 = 0xCAFE_BABE_DEAD_BEEF;
    let exact_size = encode_to_vec(&val).expect("encode_to_vec").len();
    let mut buf = vec![0u8; exact_size];
    let n = encode_into_slice(val, &mut buf, config::standard())
        .expect("exact-size buffer should work");
    assert_eq!(n, exact_size);
}

/// Test 6: oversized buffer — only N bytes written, rest untouched
#[test]
fn test_encode_into_slice_oversized_buffer_only_n_written() {
    let val: u32 = 0xDEAD_BEEF;
    let mut buf = [0xFFu8; 64];
    let n = encode_into_slice(val, &mut buf, config::standard()).expect("encode oversized");
    assert!(n > 0);
    assert!(n < 64);
    assert!(
        buf[n..].iter().all(|&b| b == 0xFF),
        "bytes beyond n should remain 0xFF (untouched)"
    );
}

/// Test 7: undersized buffer returns an error
#[test]
fn test_encode_into_slice_undersized_buffer_returns_error() {
    let val = "this string is definitely longer than four bytes".to_string();
    let mut buf = [0u8; 4];
    let result = encode_into_slice(val, &mut buf, config::standard());
    assert!(result.is_err(), "undersized buffer must return Err");
}

/// Test 8: buf[..n] bytes match encode_to_vec output byte-for-byte
#[test]
fn test_encode_into_slice_output_matches_encode_to_vec() {
    let val: u64 = 9_876_543_210u64;
    let expected_bytes = encode_to_vec(&val).expect("encode_to_vec");
    let mut buf = [0u8; 32];
    let n = encode_into_slice(val, &mut buf, config::standard()).expect("encode to slice");
    assert_eq!(
        &buf[..n],
        expected_bytes.as_slice(),
        "slice output must match encode_to_vec"
    );
}

/// Test 9: encode with fixed_int_encoding config — u32 always 4 bytes
#[test]
fn test_encode_into_slice_with_fixed_int_config() {
    let val: u32 = 1;
    let cfg = config::standard().with_fixed_int_encoding();
    let mut buf = [0u8; 16];
    let n = encode_into_slice(val, &mut buf, cfg).expect("encode fixed int");
    assert_eq!(
        n, 4,
        "fixed int encoding should write exactly 4 bytes for u32"
    );
    let (dec, _): (u32, _) =
        oxicode::decode_from_slice_with_config(&buf[..n], cfg).expect("decode fixed int");
    assert_eq!(val, dec);
}

/// Test 10: encode with legacy (big endian) config and roundtrip
#[test]
fn test_encode_into_slice_with_big_endian_config() {
    let val: u32 = 0x0102_0304;
    let cfg = config::legacy();
    let mut buf = [0u8; 16];
    let n = encode_into_slice(val, &mut buf, cfg).expect("encode big endian");
    assert!(n > 0);
    let (dec, _): (u32, _) =
        oxicode::decode_from_slice_with_config(&buf[..n], cfg).expect("decode big endian");
    assert_eq!(val, dec);
}

/// Test 11: struct with derive macro roundtrip via encode_into_slice
#[test]
fn test_encode_into_slice_struct_derive() {
    let val = Color {
        r: 255,
        g: 128,
        b: 0,
    };
    let expected = Color {
        r: 255,
        g: 128,
        b: 0,
    };
    let mut buf = [0u8; 32];
    let n = encode_into_slice(val, &mut buf, config::standard()).expect("encode struct");
    let (dec, consumed): (Color, _) = decode_from_slice(&buf[..n]).expect("decode struct");
    assert_eq!(expected, dec);
    assert_eq!(consumed, n);
}

/// Test 12: enum — encode all variants, decode and compare
#[test]
fn test_encode_into_slice_enum_all_variants() {
    let variants = [
        Direction::North,
        Direction::South,
        Direction::East,
        Direction::West,
    ];
    for dir in variants {
        let expected_label = format!("{dir:?}");
        let mut buf = [0u8; 16];
        let n =
            encode_into_slice(dir, &mut buf, config::standard()).expect("encode direction variant");
        let (dec, _): (Direction, _) = decode_from_slice(&buf[..n]).expect("decode direction");
        assert_eq!(format!("{dec:?}"), expected_label, "enum variant mismatch");
    }
}

/// Test 13: encode bool true and false, decode and compare
#[test]
fn test_encode_into_slice_bool_true_false() {
    for &flag in &[true, false] {
        let mut buf = [0u8; 4];
        let n = encode_into_slice(flag, &mut buf, config::standard()).expect("encode bool");
        assert!(n > 0);
        let (dec, _): (bool, _) = decode_from_slice(&buf[..n]).expect("decode bool");
        assert_eq!(flag, dec, "bool roundtrip failed for {flag}");
    }
}

/// Test 14: u8 always encodes as exactly 1 byte
#[test]
fn test_encode_into_slice_u8_always_1_byte() {
    let val: u8 = 200;
    let mut buf = [0u8; 4];
    let n = encode_into_slice(val, &mut buf, config::standard()).expect("encode u8");
    assert_eq!(n, 1, "u8 should always encode to 1 byte");
    let (dec, _): (u8, _) = decode_from_slice(&buf[..n]).expect("decode u8");
    assert_eq!(val, dec);
}

/// Test 15: large Vec<u8> with 1000 elements roundtrip
#[test]
fn test_encode_into_slice_large_vec_u8() {
    let val: Vec<u8> = (0u8..=255).cycle().take(1000).collect();
    let expected = val.clone();
    let mut buf = vec![0u8; 2048];
    let n = encode_into_slice(val, &mut buf, config::standard()).expect("encode large vec");
    let (dec, consumed): (Vec<u8>, _) = decode_from_slice(&buf[..n]).expect("decode large vec");
    assert_eq!(expected, dec);
    assert_eq!(consumed, n);
}

/// Test 16: encode_into_slice then decode_from_slice explicit roundtrip
#[test]
fn test_encode_into_slice_then_decode_from_slice() {
    let val: i32 = -99_999;
    let mut buf = [0u8; 16];
    let n = encode_into_slice(val, &mut buf, config::standard()).expect("encode i32");
    let slice = &buf[..n];
    let (dec, consumed): (i32, _) = decode_from_slice(slice).expect("decode i32 from slice");
    assert_eq!(val, dec);
    assert_eq!(consumed, n);
}

/// Test 17: Option<u64> Some and None variants
#[test]
fn test_encode_into_slice_option_some_none() {
    let some_val: Option<u64> = Some(0xFFFF_FFFF_FFFF_FFFFu64);
    let none_val: Option<u64> = None;

    let mut buf = [0u8; 16];

    let n = encode_into_slice(some_val, &mut buf, config::standard()).expect("encode Some");
    let (dec_some, _): (Option<u64>, _) = decode_from_slice(&buf[..n]).expect("decode Some option");
    assert_eq!(some_val, dec_some);

    let n = encode_into_slice(none_val, &mut buf, config::standard()).expect("encode None");
    let (dec_none, _): (Option<u64>, _) = decode_from_slice(&buf[..n]).expect("decode None option");
    assert_eq!(none_val, dec_none);
}

/// Test 18: i128 encode/decode roundtrip
#[test]
fn test_encode_into_slice_i128() {
    let val: i128 = i128::MIN / 2 + 12345678901234567890i128;
    let mut buf = [0u8; 32];
    let n = encode_into_slice(val, &mut buf, config::standard()).expect("encode i128");
    let (dec, consumed): (i128, _) = decode_from_slice(&buf[..n]).expect("decode i128");
    assert_eq!(val, dec);
    assert_eq!(consumed, n);
}

/// Test 19: sequential encode at different offsets in the same buffer
#[test]
fn test_encode_into_slice_sequential_at_different_offsets() {
    let val1: u32 = 111;
    let val2: u64 = 222_222_222_222u64;
    let mut buf = [0u8; 64];

    let n1 = encode_into_slice(val1, &mut buf, config::standard()).expect("encode val1");
    let n2 =
        encode_into_slice(val2, &mut buf[n1..], config::standard()).expect("encode val2 at offset");
    assert!(n1 > 0);
    assert!(n2 > 0);

    let (dec1, _): (u32, _) = decode_from_slice(&buf[..n1]).expect("decode val1");
    let (dec2, _): (u64, _) = decode_from_slice(&buf[n1..n1 + n2]).expect("decode val2");
    assert_eq!(val1, dec1);
    assert_eq!(val2, dec2);
}

/// Test 20: encode empty Vec<u8>, decode back as empty vec
#[test]
fn test_encode_into_slice_empty_vec() {
    let val: Vec<u8> = Vec::new();
    let expected: Vec<u8> = Vec::new();
    let mut buf = [0u8; 16];
    let n = encode_into_slice(val, &mut buf, config::standard()).expect("encode empty vec");
    assert!(n > 0, "empty vec still needs at least a length prefix byte");
    let (dec, consumed): (Vec<u8>, _) = decode_from_slice(&buf[..n]).expect("decode empty vec");
    assert_eq!(expected, dec);
    assert_eq!(consumed, n);
}

/// Test 21: encode Unit struct (zero data fields), decode back
#[test]
fn test_encode_into_slice_unit_struct() {
    let val = Unit;
    let mut buf = [0u8; 8];
    let n = encode_into_slice(val, &mut buf, config::standard()).expect("encode Unit");
    // Unit struct encodes to 0 bytes (no fields)
    let (dec, _): (Unit, _) = decode_from_slice(&buf[..n]).expect("decode Unit");
    assert_eq!(Unit, dec);
}

/// Test 22: bytes_written equals encode_to_vec().len() for multiple types
#[test]
fn test_encode_into_slice_bytes_written_equals_encode_to_vec_len() {
    fn check<T: Encode + Clone>(val: T) {
        let expected_len = encode_to_vec(&val).expect("encode_to_vec").len();
        let mut buf = vec![0u8; expected_len + 64];
        let n = encode_into_slice(val, &mut buf, config::standard()).expect("encode_into_slice");
        assert_eq!(
            n, expected_len,
            "bytes_written must equal encode_to_vec length"
        );
    }

    check(0u8);
    check(255u8);
    check(0u32);
    check(u32::MAX);
    check(0i64);
    check(i64::MIN);
    check(false);
    check(true);
    check(String::from("test string for size check"));
    check(vec![1u8, 2, 3, 4, 5]);
}
