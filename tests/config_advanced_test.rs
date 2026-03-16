//! Advanced configuration tests for OxiCode.
//!
//! Covers all configuration options: standard, legacy, fixed_int, big_endian,
//! little_endian, limit, and combinations thereof.

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
    config, config::Config, decode_from_slice_with_config, encode_to_vec_with_config, Decode,
    Encode,
};

// ---------------------------------------------------------------------------
// Test 1: config::standard() default config roundtrip
// ---------------------------------------------------------------------------

#[test]
fn test_standard_config_default_roundtrip() {
    let cfg = config::standard();
    let original: (u32, String, Vec<u8>) = (42, String::from("oxicode"), vec![1, 2, 3]);
    let bytes = encode_to_vec_with_config(&original, cfg).expect("standard roundtrip encode");
    let (decoded, consumed): ((u32, String, Vec<u8>), usize) =
        decode_from_slice_with_config(&bytes, cfg).expect("standard roundtrip decode");
    assert_eq!(decoded, original);
    assert_eq!(consumed, bytes.len(), "all encoded bytes must be consumed");
}

// ---------------------------------------------------------------------------
// Test 2: config::legacy() fixed int encoding roundtrip
// ---------------------------------------------------------------------------

#[test]
fn test_legacy_config_fixed_int_roundtrip() {
    let cfg = config::legacy();
    // legacy = little-endian + fixed-int: every u32 is exactly 4 bytes
    let value: u32 = 0xDEAD_BEEF;
    let bytes = encode_to_vec_with_config(&value, cfg).expect("legacy roundtrip encode");
    assert_eq!(bytes.len(), 4, "legacy u32 must always be 4 bytes");
    let (decoded, consumed): (u32, usize) =
        decode_from_slice_with_config(&bytes, cfg).expect("legacy roundtrip decode");
    assert_eq!(decoded, value);
    assert_eq!(consumed, 4);
}

// ---------------------------------------------------------------------------
// Test 3: .with_fixed_int_encoding() — u32 always 4 bytes
// ---------------------------------------------------------------------------

#[test]
fn test_with_fixed_int_encoding_u32_always_4_bytes() {
    let cfg = config::standard().with_fixed_int_encoding();
    for value in [0u32, 1, 127, 128, 255, 300, 65535, u32::MAX] {
        let bytes = encode_to_vec_with_config(&value, cfg).expect("fixed-int u32 encode");
        assert_eq!(
            bytes.len(),
            4,
            "fixed-int u32 must always be 4 bytes; value={value}"
        );
        let (decoded, consumed): (u32, usize) =
            decode_from_slice_with_config(&bytes, cfg).expect("fixed-int u32 decode");
        assert_eq!(decoded, value);
        assert_eq!(consumed, 4);
    }
}

// ---------------------------------------------------------------------------
// Test 4: .with_big_endian() — verify byte order for u32 = 0x01020304
// ---------------------------------------------------------------------------

#[test]
fn test_with_big_endian_verify_byte_order() {
    let cfg = config::standard()
        .with_big_endian()
        .with_fixed_int_encoding();
    let val: u32 = 0x0102_0304;
    let bytes = encode_to_vec_with_config(&val, cfg).expect("big-endian encode");
    assert_eq!(
        bytes,
        &[0x01, 0x02, 0x03, 0x04],
        "big-endian u32 0x01020304 must be MSB-first"
    );
    let (decoded, _): (u32, usize) =
        decode_from_slice_with_config(&bytes, cfg).expect("big-endian decode");
    assert_eq!(decoded, val);
}

// ---------------------------------------------------------------------------
// Test 5: .with_little_endian() — default, verify same as standard for u32
// ---------------------------------------------------------------------------

#[test]
fn test_with_little_endian_same_as_standard() {
    let std_cfg = config::standard();
    let le_cfg = config::standard()
        .with_little_endian()
        .with_fixed_int_encoding();
    let fixed_std_cfg = config::standard().with_fixed_int_encoding();

    // little-endian + fixed must produce LSB-first byte order
    let val: u32 = 0x0102_0304;
    let le_bytes = encode_to_vec_with_config(&val, le_cfg).expect("le encode");
    assert_eq!(
        le_bytes,
        &[0x04, 0x03, 0x02, 0x01],
        "little-endian u32 0x01020304 must be LSB-first"
    );

    // For varint, standard and explicit little-endian must produce identical bytes
    let small: u32 = 42;
    let std_bytes = encode_to_vec_with_config(&small, std_cfg).expect("std encode");
    // varint of 42 is just [42] — standard is LE varint by default
    assert_eq!(std_bytes, &[42u8], "standard varint u32 42 must be [42]");

    // fixed-int little endian matches legacy for a simple value
    let fixed_le_bytes = encode_to_vec_with_config(&small, fixed_std_cfg).expect("fixed le encode");
    assert_eq!(
        fixed_le_bytes,
        &[42, 0, 0, 0],
        "fixed LE u32 42 must be [42, 0, 0, 0]"
    );

    let (decoded, _): (u32, usize) =
        decode_from_slice_with_config(&le_bytes, le_cfg).expect("le decode");
    assert_eq!(decoded, val);
}

// ---------------------------------------------------------------------------
// Test 6: .with_limit::<100>() — large data fails
// ---------------------------------------------------------------------------

#[test]
fn test_with_limit_100_large_data_fails() {
    // Encode a 50-byte string without limit to get valid bytes
    let large: String = "x".repeat(200);
    let unlimited_bytes =
        encode_to_vec_with_config(&large, config::standard()).expect("unlimited encode");
    // Now decode with limit 100 — payload exceeds limit
    let cfg = config::standard().with_limit::<100>();
    let result: Result<(String, usize), _> = decode_from_slice_with_config(&unlimited_bytes, cfg);
    assert!(
        result.is_err(),
        "decoding a 200-char string with limit 100 must fail"
    );
}

// ---------------------------------------------------------------------------
// Test 7: .with_limit::<100>() — small data succeeds
// ---------------------------------------------------------------------------

#[test]
fn test_with_limit_100_small_data_succeeds() {
    let cfg = config::standard().with_limit::<100>();
    let small: u32 = 99;
    let bytes = encode_to_vec_with_config(&small, cfg).expect("limit-100 small encode");
    assert!(
        bytes.len() <= 100,
        "encoded bytes must not exceed the limit"
    );
    let (decoded, _): (u32, usize) =
        decode_from_slice_with_config(&bytes, cfg).expect("limit-100 small decode");
    assert_eq!(decoded, small);
}

// ---------------------------------------------------------------------------
// Test 8: .with_limit::<1>() — containers with >1-byte payload fail
// ---------------------------------------------------------------------------

#[test]
fn test_with_limit_1_container_payload_fails() {
    let cfg = config::standard().with_limit::<1>();
    assert_eq!(cfg.limit(), Some(1), "limit must be Some(1)");

    // bool encodes to exactly 1 byte — should succeed at the boundary
    let bool_bytes = encode_to_vec_with_config(&true, config::standard()).expect("bool encode");
    assert_eq!(bool_bytes.len(), 1, "bool true must be 1 byte");
    let result: Result<(bool, usize), _> = decode_from_slice_with_config(&bool_bytes, cfg);
    assert!(
        result.is_ok(),
        "decoding a 1-byte bool with limit 1 must succeed"
    );

    // A String with 2 chars requires claim_bytes_read(2), which exceeds the 1-byte limit
    let two_char_str = String::from("ab");
    let str_bytes =
        encode_to_vec_with_config(&two_char_str, config::standard()).expect("string encode");
    assert!(
        str_bytes.len() > 1,
        "2-char string must encode to more than 1 byte"
    );
    let result2: Result<(String, usize), _> = decode_from_slice_with_config(&str_bytes, cfg);
    assert!(
        result2.is_err(),
        "decoding a 2-char string with limit 1 must fail (claim_bytes_read(2) > 1)"
    );
}

// ---------------------------------------------------------------------------
// Test 9: Standard config u8 is 1 byte
// ---------------------------------------------------------------------------

#[test]
fn test_standard_config_u8_is_1_byte() {
    let cfg = config::standard();
    for value in [0u8, 1, 127, 200, 255] {
        let bytes = encode_to_vec_with_config(&value, cfg).expect("u8 encode");
        assert_eq!(bytes.len(), 1, "standard u8 must be 1 byte; value={value}");
        let (decoded, consumed): (u8, usize) =
            decode_from_slice_with_config(&bytes, cfg).expect("u8 decode");
        assert_eq!(decoded, value);
        assert_eq!(consumed, 1);
    }
}

// ---------------------------------------------------------------------------
// Test 10: Standard config u64 varint encoding varies by value
// ---------------------------------------------------------------------------

#[test]
fn test_standard_config_u64_varint_varies_by_value() {
    let cfg = config::standard();

    // Small values (< 251): 1 byte
    let small_bytes = encode_to_vec_with_config(&42u64, cfg).expect("varint small encode");
    assert_eq!(small_bytes.len(), 1, "varint u64 42 must be 1 byte");

    // 300 encodes as 3 bytes: marker 251 + LE u16
    let mid_bytes = encode_to_vec_with_config(&300u64, cfg).expect("varint mid encode");
    assert_eq!(mid_bytes.len(), 3, "varint u64 300 must be 3 bytes");
    assert_eq!(mid_bytes, &[251, 44, 1], "varint 300 must be [251, 44, 1]");

    // u64::MAX encodes as 9 bytes: marker 255 + raw u64
    let max_bytes = encode_to_vec_with_config(&u64::MAX, cfg).expect("varint max encode");
    assert_eq!(max_bytes.len(), 9, "varint u64::MAX must be 9 bytes");

    // Roundtrip all three
    let (dec_small, _): (u64, usize) =
        decode_from_slice_with_config(&small_bytes, cfg).expect("varint small decode");
    let (dec_mid, _): (u64, usize) =
        decode_from_slice_with_config(&mid_bytes, cfg).expect("varint mid decode");
    let (dec_max, _): (u64, usize) =
        decode_from_slice_with_config(&max_bytes, cfg).expect("varint max decode");
    assert_eq!(dec_small, 42u64);
    assert_eq!(dec_mid, 300u64);
    assert_eq!(dec_max, u64::MAX);
}

// ---------------------------------------------------------------------------
// Test 11: Fixed int u8 is still 1 byte
// ---------------------------------------------------------------------------

#[test]
fn test_fixed_int_u8_is_still_1_byte() {
    let cfg = config::standard().with_fixed_int_encoding();
    for value in [0u8, 1, 127, 200, 255] {
        let bytes = encode_to_vec_with_config(&value, cfg).expect("fixed u8 encode");
        assert_eq!(bytes.len(), 1, "fixed-int u8 must be 1 byte; value={value}");
        let (decoded, consumed): (u8, usize) =
            decode_from_slice_with_config(&bytes, cfg).expect("fixed u8 decode");
        assert_eq!(decoded, value);
        assert_eq!(consumed, 1);
    }
}

// ---------------------------------------------------------------------------
// Test 12: Fixed int u16 is always 2 bytes
// ---------------------------------------------------------------------------

#[test]
fn test_fixed_int_u16_is_always_2_bytes() {
    let cfg = config::standard().with_fixed_int_encoding();
    for value in [0u16, 1, 255, 256, 1000, u16::MAX] {
        let bytes = encode_to_vec_with_config(&value, cfg).expect("fixed u16 encode");
        assert_eq!(
            bytes.len(),
            2,
            "fixed-int u16 must always be 2 bytes; value={value}"
        );
        let (decoded, consumed): (u16, usize) =
            decode_from_slice_with_config(&bytes, cfg).expect("fixed u16 decode");
        assert_eq!(decoded, value);
        assert_eq!(consumed, 2);
    }
}

// ---------------------------------------------------------------------------
// Test 13: Fixed int u32 is always 4 bytes
// ---------------------------------------------------------------------------

#[test]
fn test_fixed_int_u32_is_always_4_bytes() {
    let cfg = config::standard().with_fixed_int_encoding();
    for value in [0u32, 1, 255, 300, 65535, u32::MAX] {
        let bytes = encode_to_vec_with_config(&value, cfg).expect("fixed u32 encode");
        assert_eq!(
            bytes.len(),
            4,
            "fixed-int u32 must always be 4 bytes; value={value}"
        );
        let (decoded, consumed): (u32, usize) =
            decode_from_slice_with_config(&bytes, cfg).expect("fixed u32 decode");
        assert_eq!(decoded, value);
        assert_eq!(consumed, 4);
    }
}

// ---------------------------------------------------------------------------
// Test 14: Fixed int u64 is always 8 bytes
// ---------------------------------------------------------------------------

#[test]
fn test_fixed_int_u64_is_always_8_bytes() {
    let cfg = config::standard().with_fixed_int_encoding();
    for value in [0u64, 1, 255, 300, u32::MAX as u64, u64::MAX] {
        let bytes = encode_to_vec_with_config(&value, cfg).expect("fixed u64 encode");
        assert_eq!(
            bytes.len(),
            8,
            "fixed-int u64 must always be 8 bytes; value={value}"
        );
        let (decoded, consumed): (u64, usize) =
            decode_from_slice_with_config(&bytes, cfg).expect("fixed u64 decode");
        assert_eq!(decoded, value);
        assert_eq!(consumed, 8);
    }
}

// ---------------------------------------------------------------------------
// Test 15: Fixed int + big endian u32 roundtrip with exact byte verification
// ---------------------------------------------------------------------------

#[test]
fn test_fixed_int_big_endian_u32_roundtrip() {
    let cfg = config::standard()
        .with_fixed_int_encoding()
        .with_big_endian();
    // 300 = 0x0000_012C → big-endian bytes = [0x00, 0x00, 0x01, 0x2C]
    let value: u32 = 300;
    let bytes = encode_to_vec_with_config(&value, cfg).expect("fixed BE u32 encode");
    assert_eq!(bytes.len(), 4, "fixed BE u32 must be 4 bytes");
    assert_eq!(
        bytes,
        &[0x00, 0x00, 0x01, 0x2C],
        "fixed big-endian u32 300 must be [0x00, 0x00, 0x01, 0x2C]"
    );
    let (decoded, consumed): (u32, usize) =
        decode_from_slice_with_config(&bytes, cfg).expect("fixed BE u32 decode");
    assert_eq!(decoded, value);
    assert_eq!(consumed, 4);
}

// ---------------------------------------------------------------------------
// Test 16: Config chain: standard().with_fixed_int_encoding().with_big_endian()
// ---------------------------------------------------------------------------

#[test]
fn test_config_chain_fixed_int_then_big_endian() {
    let cfg = config::standard()
        .with_fixed_int_encoding()
        .with_big_endian();

    use oxicode::config::{Endianness, IntEncoding};
    assert_eq!(
        cfg.endianness(),
        Endianness::Big,
        "chained config must be big-endian"
    );
    assert_eq!(
        cfg.int_encoding(),
        IntEncoding::Fixed,
        "chained config must be fixed-int"
    );
    assert_eq!(cfg.limit(), None, "chained config must have no limit");

    // Verify encoding of 0x0102_0304_0506_0708 as 8 BE bytes
    let val: u64 = 0x0102_0304_0506_0708;
    let bytes = encode_to_vec_with_config(&val, cfg).expect("chain u64 encode");
    assert_eq!(
        bytes,
        &[0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08],
        "chained BE fixed u64 must be MSB-first"
    );
    let (decoded, _): (u64, usize) =
        decode_from_slice_with_config(&bytes, cfg).expect("chain u64 decode");
    assert_eq!(decoded, val);
}

// ---------------------------------------------------------------------------
// Test 17: Config doesn't affect string encoding (always length-prefixed UTF-8)
// ---------------------------------------------------------------------------

#[test]
fn test_config_does_not_affect_string_encoding_content() {
    let s = String::from("hello");

    // Encode with standard (varint) and fixed-int configs
    let std_bytes = encode_to_vec_with_config(&s, config::standard()).expect("std string encode");
    let fixed_bytes = encode_to_vec_with_config(&s, config::standard().with_fixed_int_encoding())
        .expect("fixed string encode");

    // Both must decode back to the same string
    let (dec_std, _): (String, usize) =
        decode_from_slice_with_config(&std_bytes, config::standard()).expect("std string decode");
    let (dec_fixed, _): (String, usize) =
        decode_from_slice_with_config(&fixed_bytes, config::standard().with_fixed_int_encoding())
            .expect("fixed string decode");
    assert_eq!(dec_std, s);
    assert_eq!(dec_fixed, s);

    // The UTF-8 content bytes (b"hello") must appear in both encodings
    assert!(
        std_bytes.windows(5).any(|w| w == b"hello"),
        "UTF-8 content must be present in standard-encoded string"
    );
    assert!(
        fixed_bytes.windows(5).any(|w| w == b"hello"),
        "UTF-8 content must be present in fixed-encoded string"
    );
}

// ---------------------------------------------------------------------------
// Test 18: Config doesn't affect bool encoding (always 1 byte: 0 or 1)
// ---------------------------------------------------------------------------

#[test]
fn test_config_does_not_affect_bool_encoding() {
    let configs_produce_1_byte = |b: bool| {
        let std_bytes = encode_to_vec_with_config(&b, config::standard()).expect("std bool encode");
        let fixed_bytes =
            encode_to_vec_with_config(&b, config::standard().with_fixed_int_encoding())
                .expect("fixed bool encode");
        let be_bytes = encode_to_vec_with_config(
            &b,
            config::standard()
                .with_big_endian()
                .with_fixed_int_encoding(),
        )
        .expect("BE bool encode");

        assert_eq!(
            std_bytes.len(),
            1,
            "bool {b} must always be 1 byte (standard)"
        );
        assert_eq!(
            fixed_bytes.len(),
            1,
            "bool {b} must always be 1 byte (fixed)"
        );
        assert_eq!(
            be_bytes.len(),
            1,
            "bool {b} must always be 1 byte (big-endian)"
        );

        let expected = if b { 1u8 } else { 0u8 };
        assert_eq!(std_bytes[0], expected);
        assert_eq!(fixed_bytes[0], expected);
        assert_eq!(be_bytes[0], expected);
    };

    configs_produce_1_byte(false);
    configs_produce_1_byte(true);
}

// ---------------------------------------------------------------------------
// Test 19: Limit config with Vec<u8> — limit exceeded fails
// ---------------------------------------------------------------------------

#[test]
fn test_limit_config_with_vec_u8_limit_exceeded_fails() {
    // Build a 50-element Vec, encode without limit to get valid bytes
    let data: Vec<u8> = (0u8..50).collect();
    let unlimited_bytes =
        encode_to_vec_with_config(&data, config::standard()).expect("unlimited encode");

    // Decode with a 4-byte limit — must fail
    let cfg = config::standard().with_limit::<4>();
    let result: Result<(Vec<u8>, usize), _> = decode_from_slice_with_config(&unlimited_bytes, cfg);
    assert!(
        result.is_err(),
        "decoding a 50-element Vec<u8> with a 4-byte limit must fail"
    );
}

// ---------------------------------------------------------------------------
// Test 20: Standard config with all primitive types roundtrip
// ---------------------------------------------------------------------------

#[test]
fn test_standard_config_all_primitive_types_roundtrip() {
    let cfg = config::standard();

    // u8
    let v_u8: u8 = 200;
    let b = encode_to_vec_with_config(&v_u8, cfg).expect("u8 encode");
    let (d, _): (u8, usize) = decode_from_slice_with_config(&b, cfg).expect("u8 decode");
    assert_eq!(d, v_u8);

    // u16
    let v_u16: u16 = 1000;
    let b = encode_to_vec_with_config(&v_u16, cfg).expect("u16 encode");
    let (d, _): (u16, usize) = decode_from_slice_with_config(&b, cfg).expect("u16 decode");
    assert_eq!(d, v_u16);

    // u32
    let v_u32: u32 = 100_000;
    let b = encode_to_vec_with_config(&v_u32, cfg).expect("u32 encode");
    let (d, _): (u32, usize) = decode_from_slice_with_config(&b, cfg).expect("u32 decode");
    assert_eq!(d, v_u32);

    // u64
    let v_u64: u64 = 5_000_000_000;
    let b = encode_to_vec_with_config(&v_u64, cfg).expect("u64 encode");
    let (d, _): (u64, usize) = decode_from_slice_with_config(&b, cfg).expect("u64 decode");
    assert_eq!(d, v_u64);

    // i8
    let v_i8: i8 = -100;
    let b = encode_to_vec_with_config(&v_i8, cfg).expect("i8 encode");
    let (d, _): (i8, usize) = decode_from_slice_with_config(&b, cfg).expect("i8 decode");
    assert_eq!(d, v_i8);

    // i16
    let v_i16: i16 = -1000;
    let b = encode_to_vec_with_config(&v_i16, cfg).expect("i16 encode");
    let (d, _): (i16, usize) = decode_from_slice_with_config(&b, cfg).expect("i16 decode");
    assert_eq!(d, v_i16);

    // i32
    let v_i32: i32 = -100_000;
    let b = encode_to_vec_with_config(&v_i32, cfg).expect("i32 encode");
    let (d, _): (i32, usize) = decode_from_slice_with_config(&b, cfg).expect("i32 decode");
    assert_eq!(d, v_i32);

    // i64
    let v_i64: i64 = -5_000_000_000;
    let b = encode_to_vec_with_config(&v_i64, cfg).expect("i64 encode");
    let (d, _): (i64, usize) = decode_from_slice_with_config(&b, cfg).expect("i64 decode");
    assert_eq!(d, v_i64);

    // f32 — use a value that is not a clippy-flagged approximation of a known constant
    let v_f32: f32 = 1.23456_f32;
    let b = encode_to_vec_with_config(&v_f32, cfg).expect("f32 encode");
    let (d, _): (f32, usize) = decode_from_slice_with_config(&b, cfg).expect("f32 decode");
    assert!((d - v_f32).abs() < f32::EPSILON, "f32 roundtrip mismatch");

    // f64 — use a value that is not a clippy-flagged approximation of a known constant
    let v_f64: f64 = 1.234_567_890_123_f64;
    let b = encode_to_vec_with_config(&v_f64, cfg).expect("f64 encode");
    let (d, _): (f64, usize) = decode_from_slice_with_config(&b, cfg).expect("f64 decode");
    assert!((d - v_f64).abs() < f64::EPSILON, "f64 roundtrip mismatch");

    // bool
    let v_bool = true;
    let b = encode_to_vec_with_config(&v_bool, cfg).expect("bool encode");
    let (d, _): (bool, usize) = decode_from_slice_with_config(&b, cfg).expect("bool decode");
    assert_eq!(d, v_bool);

    // char
    let v_char: char = 'Z';
    let b = encode_to_vec_with_config(&v_char, cfg).expect("char encode");
    let (d, _): (char, usize) = decode_from_slice_with_config(&b, cfg).expect("char decode");
    assert_eq!(d, v_char);
}

// ---------------------------------------------------------------------------
// Test 21: Fixed int config with struct roundtrip
// ---------------------------------------------------------------------------

#[derive(Debug, PartialEq, Encode, Decode)]
struct SampleStruct {
    id: u32,
    value: i64,
    flag: bool,
}

#[test]
fn test_fixed_int_config_struct_roundtrip() {
    let cfg = config::standard().with_fixed_int_encoding();
    let original = SampleStruct {
        id: 0xABCD_1234,
        value: -987_654_321,
        flag: true,
    };
    let bytes = encode_to_vec_with_config(&original, cfg).expect("struct fixed encode");
    // u32 (4) + i64 (8) + bool (1) = 13 bytes
    assert_eq!(
        bytes.len(),
        13,
        "SampleStruct with fixed-int must encode to 13 bytes"
    );
    let (decoded, consumed): (SampleStruct, usize) =
        decode_from_slice_with_config(&bytes, cfg).expect("struct fixed decode");
    assert_eq!(decoded, original);
    assert_eq!(consumed, 13);
}

// ---------------------------------------------------------------------------
// Test 22: Config comparison: standard vs fixed_int encoded sizes for u64
// ---------------------------------------------------------------------------

#[test]
fn test_config_comparison_standard_vs_fixed_int_u64_sizes() {
    let std_cfg = config::standard();
    let fixed_cfg = config::standard().with_fixed_int_encoding();

    // For a small u64 (< 251), standard (varint) is smaller than fixed-int
    let small: u64 = 1;
    let std_small = encode_to_vec_with_config(&small, std_cfg).expect("std small encode");
    let fixed_small = encode_to_vec_with_config(&small, fixed_cfg).expect("fixed small encode");
    assert_eq!(std_small.len(), 1, "varint u64 1 must be 1 byte");
    assert_eq!(fixed_small.len(), 8, "fixed u64 1 must be 8 bytes");
    assert!(
        std_small.len() < fixed_small.len(),
        "standard (varint) must be smaller than fixed for small u64"
    );

    // For u64::MAX, varint is larger (9 bytes) than fixed-int (8 bytes)
    let large: u64 = u64::MAX;
    let std_large = encode_to_vec_with_config(&large, std_cfg).expect("std large encode");
    let fixed_large = encode_to_vec_with_config(&large, fixed_cfg).expect("fixed large encode");
    assert_eq!(std_large.len(), 9, "varint u64::MAX must be 9 bytes");
    assert_eq!(fixed_large.len(), 8, "fixed u64::MAX must be 8 bytes");
    assert!(
        std_large.len() > fixed_large.len(),
        "standard (varint) must be larger than fixed for u64::MAX"
    );

    // Both roundtrip correctly
    let (dec_std, _): (u64, usize) =
        decode_from_slice_with_config(&std_large, std_cfg).expect("std large decode");
    let (dec_fixed, _): (u64, usize) =
        decode_from_slice_with_config(&fixed_large, fixed_cfg).expect("fixed large decode");
    assert_eq!(dec_std, large);
    assert_eq!(dec_fixed, large);
}
