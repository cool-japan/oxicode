//! Advanced config combination tests for OxiCode — 22 top-level #[test] functions.
//!
//! Covers: standard/legacy configs, fixed-int encoding, big/little endian byte order,
//! struct roundtrips, Copy config reuse, bool roundtrip, with_limit, and more.

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
use oxicode::{config, decode_from_slice_with_config, encode_to_vec_with_config, Decode, Encode};

// ---------------------------------------------------------------------------
// Shared test structs
// ---------------------------------------------------------------------------

#[derive(Debug, PartialEq, Encode, Decode)]
struct SimpleStruct {
    a: u32,
    b: u64,
}

// ---------------------------------------------------------------------------
// Test 1: standard() config basic roundtrip (u32)
// ---------------------------------------------------------------------------

#[test]
fn test_standard_config_u32_roundtrip() {
    let cfg = config::standard();
    let original: u32 = 42_000;
    let encoded = encode_to_vec_with_config(&original, cfg).expect("standard() u32 encode failed");
    let (decoded, consumed): (u32, usize) =
        decode_from_slice_with_config(&encoded, cfg).expect("standard() u32 decode failed");
    assert_eq!(decoded, original);
    assert_eq!(consumed, encoded.len());
}

// ---------------------------------------------------------------------------
// Test 2: legacy() config basic roundtrip (u32)
// ---------------------------------------------------------------------------

#[test]
fn test_legacy_config_u32_roundtrip() {
    let cfg = config::legacy();
    let original: u32 = 99_999;
    let encoded = encode_to_vec_with_config(&original, cfg).expect("legacy() u32 encode failed");
    let (decoded, consumed): (u32, usize) =
        decode_from_slice_with_config(&encoded, cfg).expect("legacy() u32 decode failed");
    assert_eq!(decoded, original);
    assert_eq!(consumed, encoded.len());
}

// ---------------------------------------------------------------------------
// Test 3: standard().with_fixed_int_encoding() — u32 is exactly 4 bytes
// ---------------------------------------------------------------------------

#[test]
fn test_standard_fixed_int_u32_is_4_bytes() {
    let cfg = config::standard().with_fixed_int_encoding();
    let original: u32 = 255;
    let encoded = encode_to_vec_with_config(&original, cfg).expect("fixed-int u32 encode failed");
    assert_eq!(encoded.len(), 4, "fixed-int u32 must be exactly 4 bytes");
    let (decoded, consumed): (u32, usize) =
        decode_from_slice_with_config(&encoded, cfg).expect("fixed-int u32 decode failed");
    assert_eq!(decoded, original);
    assert_eq!(consumed, 4);
}

// ---------------------------------------------------------------------------
// Test 4: standard().with_big_endian().with_fixed_int_encoding() — u32 bytes in big-endian order
// ---------------------------------------------------------------------------

#[test]
fn test_standard_big_endian_fixed_int_u32_byte_order() {
    let cfg = config::standard()
        .with_big_endian()
        .with_fixed_int_encoding();
    let val: u32 = 0x0102_0304;
    let encoded =
        encode_to_vec_with_config(&val, cfg).expect("big-endian fixed-int u32 encode failed");
    assert_eq!(encoded.len(), 4, "fixed-int u32 must be exactly 4 bytes");
    assert_eq!(encoded[0], 0x01, "big-endian: most significant byte first");
    assert_eq!(encoded[1], 0x02);
    assert_eq!(encoded[2], 0x03);
    assert_eq!(encoded[3], 0x04, "big-endian: least significant byte last");
    let (decoded, _): (u32, usize) = decode_from_slice_with_config(&encoded, cfg)
        .expect("big-endian fixed-int u32 decode failed");
    assert_eq!(decoded, val);
}

// ---------------------------------------------------------------------------
// Test 5: legacy().with_fixed_int_encoding() — legacy + fixed int
// ---------------------------------------------------------------------------

#[test]
fn test_legacy_with_fixed_int_encoding_roundtrip() {
    // legacy() is already fixed-int; calling with_fixed_int_encoding() is idempotent
    let cfg = config::legacy().with_fixed_int_encoding();
    let original: u32 = 0xDEAD_BEEF;
    let encoded =
        encode_to_vec_with_config(&original, cfg).expect("legacy+fixed-int u32 encode failed");
    assert_eq!(
        encoded.len(),
        4,
        "legacy+fixed-int u32 must be exactly 4 bytes"
    );
    let (decoded, consumed): (u32, usize) =
        decode_from_slice_with_config(&encoded, cfg).expect("legacy+fixed-int u32 decode failed");
    assert_eq!(decoded, original);
    assert_eq!(consumed, 4);
}

// ---------------------------------------------------------------------------
// Test 6: standard() vs legacy() produce different bytes for same u64 value
// ---------------------------------------------------------------------------

#[test]
fn test_standard_vs_legacy_different_bytes_for_u64() {
    let std_cfg = config::standard();
    let leg_cfg = config::legacy();
    // Use a value > 127 so varint (standard) produces different byte count than fixed (legacy)
    let val: u64 = 1024;
    let std_bytes = encode_to_vec_with_config(&val, std_cfg).expect("standard() u64 encode failed");
    let leg_bytes = encode_to_vec_with_config(&val, leg_cfg).expect("legacy() u64 encode failed");
    // legacy = fixed-int: u64 is always 8 bytes; standard varint = compact
    assert_eq!(leg_bytes.len(), 8, "legacy u64 must be 8 bytes");
    assert_ne!(
        std_bytes, leg_bytes,
        "standard and legacy must produce different encodings for u64(1024)"
    );
}

// ---------------------------------------------------------------------------
// Test 7: standard().with_fixed_int_encoding() for u16 is exactly 2 bytes
// ---------------------------------------------------------------------------

#[test]
fn test_standard_fixed_int_u16_is_2_bytes() {
    let cfg = config::standard().with_fixed_int_encoding();
    let original: u16 = 1000;
    let encoded = encode_to_vec_with_config(&original, cfg).expect("fixed-int u16 encode failed");
    assert_eq!(encoded.len(), 2, "fixed-int u16 must be exactly 2 bytes");
    let (decoded, consumed): (u16, usize) =
        decode_from_slice_with_config(&encoded, cfg).expect("fixed-int u16 decode failed");
    assert_eq!(decoded, original);
    assert_eq!(consumed, 2);
}

// ---------------------------------------------------------------------------
// Test 8: standard().with_fixed_int_encoding() for u64 is exactly 8 bytes
// ---------------------------------------------------------------------------

#[test]
fn test_standard_fixed_int_u64_is_8_bytes() {
    let cfg = config::standard().with_fixed_int_encoding();
    let original: u64 = 1_000_000_000_000u64;
    let encoded = encode_to_vec_with_config(&original, cfg).expect("fixed-int u64 encode failed");
    assert_eq!(encoded.len(), 8, "fixed-int u64 must be exactly 8 bytes");
    let (decoded, consumed): (u64, usize) =
        decode_from_slice_with_config(&encoded, cfg).expect("fixed-int u64 decode failed");
    assert_eq!(decoded, original);
    assert_eq!(consumed, 8);
}

// ---------------------------------------------------------------------------
// Test 9: standard().with_fixed_int_encoding() for i32 is exactly 4 bytes
// ---------------------------------------------------------------------------

#[test]
fn test_standard_fixed_int_i32_is_4_bytes() {
    let cfg = config::standard().with_fixed_int_encoding();
    let original: i32 = -12345;
    let encoded = encode_to_vec_with_config(&original, cfg).expect("fixed-int i32 encode failed");
    assert_eq!(encoded.len(), 4, "fixed-int i32 must be exactly 4 bytes");
    let (decoded, consumed): (i32, usize) =
        decode_from_slice_with_config(&encoded, cfg).expect("fixed-int i32 decode failed");
    assert_eq!(decoded, original);
    assert_eq!(consumed, 4);
}

// ---------------------------------------------------------------------------
// Test 10: Big-endian: u32(0x01020304) bytes are [0x01, 0x02, 0x03, 0x04]
// ---------------------------------------------------------------------------

#[test]
fn test_big_endian_u32_explicit_bytes() {
    let cfg = config::standard()
        .with_big_endian()
        .with_fixed_int_encoding();
    let val: u32 = 0x0102_0304;
    let encoded = encode_to_vec_with_config(&val, cfg).expect("big-endian u32 encode failed");
    assert_eq!(
        encoded,
        &[0x01, 0x02, 0x03, 0x04],
        "big-endian byte order mismatch"
    );
}

// ---------------------------------------------------------------------------
// Test 11: Little-endian (default): u32(0x01020304) bytes have 0x04 first in fixed-int
// ---------------------------------------------------------------------------

#[test]
fn test_little_endian_u32_fixed_int_first_byte_is_lsb() {
    let cfg = config::standard().with_fixed_int_encoding();
    let val: u32 = 0x0102_0304;
    let encoded =
        encode_to_vec_with_config(&val, cfg).expect("little-endian fixed-int u32 encode failed");
    assert_eq!(encoded.len(), 4);
    assert_eq!(
        encoded[0], 0x04,
        "little-endian: least significant byte is first"
    );
    assert_eq!(
        encoded[3], 0x01,
        "little-endian: most significant byte is last"
    );
    let (decoded, _): (u32, usize) = decode_from_slice_with_config(&encoded, cfg)
        .expect("little-endian fixed-int u32 decode failed");
    assert_eq!(decoded, val);
}

// ---------------------------------------------------------------------------
// Test 12: Config is Copy — can reuse same config variable for encode and decode
// ---------------------------------------------------------------------------

#[test]
fn test_config_is_copy_reuse_variable() {
    let cfg = config::standard().with_fixed_int_encoding();
    let original: u32 = 77_777;
    // cfg is Copy — we can use the same binding for both operations
    let encoded = encode_to_vec_with_config(&original, cfg).expect("copy config encode failed");
    let (decoded, _): (u32, usize) =
        decode_from_slice_with_config(&encoded, cfg).expect("copy config decode failed");
    assert_eq!(decoded, original);
}

// ---------------------------------------------------------------------------
// Test 13: standard() with Vec<u8> roundtrip
// ---------------------------------------------------------------------------

#[test]
fn test_standard_vec_u8_roundtrip() {
    let cfg = config::standard();
    let original: Vec<u8> = vec![10, 20, 30, 40, 50, 60, 70, 80, 90, 100];
    let encoded =
        encode_to_vec_with_config(&original, cfg).expect("standard Vec<u8> encode failed");
    let (decoded, consumed): (Vec<u8>, usize) =
        decode_from_slice_with_config(&encoded, cfg).expect("standard Vec<u8> decode failed");
    assert_eq!(decoded, original);
    assert_eq!(consumed, encoded.len());
}

// ---------------------------------------------------------------------------
// Test 14: standard().with_fixed_int_encoding() with struct roundtrip
// ---------------------------------------------------------------------------

#[test]
fn test_standard_fixed_int_struct_roundtrip() {
    let cfg = config::standard().with_fixed_int_encoding();
    let original = SimpleStruct {
        a: 1234,
        b: 56_789_012,
    };
    let encoded =
        encode_to_vec_with_config(&original, cfg).expect("fixed-int struct encode failed");
    // u32 = 4 bytes, u64 = 8 bytes
    assert_eq!(
        encoded.len(),
        12,
        "fixed-int struct must encode to 12 bytes"
    );
    let (decoded, consumed): (SimpleStruct, usize) =
        decode_from_slice_with_config(&encoded, cfg).expect("fixed-int struct decode failed");
    assert_eq!(decoded, original);
    assert_eq!(consumed, 12);
}

// ---------------------------------------------------------------------------
// Test 15: Big-endian config with String roundtrip
// ---------------------------------------------------------------------------

#[test]
fn test_big_endian_string_roundtrip() {
    let cfg = config::standard().with_big_endian();
    let original = String::from("big-endian oxicode test");
    let encoded =
        encode_to_vec_with_config(&original, cfg).expect("big-endian String encode failed");
    let (decoded, consumed): (String, usize) =
        decode_from_slice_with_config(&encoded, cfg).expect("big-endian String decode failed");
    assert_eq!(decoded, original);
    assert_eq!(consumed, encoded.len());
}

// ---------------------------------------------------------------------------
// Test 16: standard() vs standard().with_fixed_int_encoding() produce different bytes for u32(42)
// ---------------------------------------------------------------------------

#[test]
fn test_standard_vs_fixed_int_different_bytes_for_u32_42() {
    let std_cfg = config::standard();
    let fix_cfg = config::standard().with_fixed_int_encoding();
    let val: u32 = 42;
    let std_bytes =
        encode_to_vec_with_config(&val, std_cfg).expect("standard u32(42) encode failed");
    let fix_bytes =
        encode_to_vec_with_config(&val, fix_cfg).expect("fixed-int u32(42) encode failed");
    // varint of 42 = 1 byte; fixed-int of u32 = 4 bytes
    assert_eq!(std_bytes.len(), 1, "varint u32(42) must be 1 byte");
    assert_eq!(fix_bytes.len(), 4, "fixed-int u32(42) must be 4 bytes");
    assert_ne!(
        std_bytes, fix_bytes,
        "standard and fixed-int must produce different bytes"
    );
}

// ---------------------------------------------------------------------------
// Test 17: standard() with bool roundtrip
// ---------------------------------------------------------------------------

#[test]
fn test_standard_bool_roundtrip() {
    let cfg = config::standard();
    for &val in &[true, false] {
        let encoded = encode_to_vec_with_config(&val, cfg).expect("standard bool encode failed");
        assert_eq!(encoded.len(), 1, "bool must encode to exactly 1 byte");
        let (decoded, consumed): (bool, usize) =
            decode_from_slice_with_config(&encoded, cfg).expect("standard bool decode failed");
        assert_eq!(decoded, val);
        assert_eq!(consumed, 1);
    }
}

// ---------------------------------------------------------------------------
// Test 18: Config with limit — standard().with_limit::<100>() decode String within limit
// ---------------------------------------------------------------------------

#[test]
fn test_limit_config_small_string_within_limit() {
    // Encode a 10-byte content string without a limit config
    let std_cfg = config::standard();
    let small = String::from("0123456789"); // exactly 10 ASCII bytes of content
    let encoded = encode_to_vec_with_config(&small, std_cfg).expect("small string encode failed");

    // Decode with limit::<100>() — the string content is 10 bytes, well within 100
    let lim_cfg = config::standard().with_limit::<100>();
    let (decoded, _): (String, usize) = decode_from_slice_with_config(&encoded, lim_cfg)
        .expect("decode small string within limit::<100> must succeed");
    assert_eq!(decoded, small);
}

// ---------------------------------------------------------------------------
// Test 19: Config with limit — exceed limit returns error for large String
// ---------------------------------------------------------------------------

#[test]
fn test_limit_config_large_string_exceeds_limit() {
    // Encode a 200-byte content string without a limit config
    let std_cfg = config::standard();
    let large = "x".repeat(200);
    let encoded = encode_to_vec_with_config(&large, std_cfg).expect("large string encode failed");

    // Decode with limit::<50>() — the string content is 200 bytes, exceeds 50
    let lim_cfg = config::standard().with_limit::<50>();
    let result: Result<(String, usize), _> = decode_from_slice_with_config(&encoded, lim_cfg);
    assert!(
        result.is_err(),
        "decode 200-byte string with limit::<50> must return an error"
    );
}

// ---------------------------------------------------------------------------
// Test 20: legacy() with struct roundtrip
// ---------------------------------------------------------------------------

#[test]
fn test_legacy_struct_roundtrip() {
    let cfg = config::legacy();
    let original = SimpleStruct { a: 987, b: 654_321 };
    let encoded = encode_to_vec_with_config(&original, cfg).expect("legacy struct encode failed");
    // legacy = fixed-int: u32=4 bytes, u64=8 bytes => 12 bytes total
    assert_eq!(encoded.len(), 12, "legacy struct must encode to 12 bytes");
    let (decoded, consumed): (SimpleStruct, usize) =
        decode_from_slice_with_config(&encoded, cfg).expect("legacy struct decode failed");
    assert_eq!(decoded, original);
    assert_eq!(consumed, 12);
}

// ---------------------------------------------------------------------------
// Test 21: standard().with_big_endian() (variable int) with u64(1000)
// ---------------------------------------------------------------------------

#[test]
fn test_standard_big_endian_varint_u64_1000() {
    let cfg = config::standard().with_big_endian();
    let val: u64 = 1000;
    let encoded =
        encode_to_vec_with_config(&val, cfg).expect("big-endian varint u64(1000) encode failed");
    // varint of 1000: 1000 >= 128 so it needs more than 1 byte
    assert!(
        encoded.len() > 1,
        "varint u64(1000) should need more than 1 byte"
    );
    let (decoded, consumed): (u64, usize) = decode_from_slice_with_config(&encoded, cfg)
        .expect("big-endian varint u64(1000) decode failed");
    assert_eq!(decoded, val);
    assert_eq!(consumed, encoded.len());
}

// ---------------------------------------------------------------------------
// Test 22: fixed-int + big-endian: u16(0x0102) is exactly [0x01, 0x02]
// ---------------------------------------------------------------------------

#[test]
fn test_fixed_int_big_endian_u16_exact_bytes() {
    let cfg = config::standard()
        .with_fixed_int_encoding()
        .with_big_endian();
    let val: u16 = 0x0102;
    let encoded =
        encode_to_vec_with_config(&val, cfg).expect("fixed-int big-endian u16 encode failed");
    assert_eq!(encoded.len(), 2, "fixed-int u16 must be exactly 2 bytes");
    assert_eq!(encoded[0], 0x01, "big-endian u16: high byte first");
    assert_eq!(encoded[1], 0x02, "big-endian u16: low byte second");
    let (decoded, consumed): (u16, usize) = decode_from_slice_with_config(&encoded, cfg)
        .expect("fixed-int big-endian u16 decode failed");
    assert_eq!(decoded, val);
    assert_eq!(consumed, 2);
}
