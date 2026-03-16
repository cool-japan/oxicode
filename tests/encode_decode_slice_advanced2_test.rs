//! Advanced slice encode/decode API surface tests (set 2).
//!
//! Exactly 22 top-level `#[test]` functions covering `decode_from_slice`,
//! `encode_into_slice`, `decode_from_slice_with_config`, and `encode_to_vec_with_config`.

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
    config, decode_from_slice, decode_from_slice_with_config, encode_into_slice, encode_to_vec,
    encode_to_vec_with_config, Decode, Encode,
};

// ---------------------------------------------------------------------------
// Helper types
// ---------------------------------------------------------------------------

#[derive(Debug, PartialEq, Encode, Decode)]
struct SimpleStruct {
    a: u32,
    b: u64,
}

#[derive(Debug, PartialEq, Encode, Decode)]
enum SimpleEnum {
    Alpha,
    Beta(u32),
    Gamma { x: i16, y: i16 },
}

// ---------------------------------------------------------------------------
// Test 1: decode_from_slice with exact-size buffer — consumed == buf.len()
// ---------------------------------------------------------------------------

#[test]
fn test_decode_from_slice_exact_size_buffer() {
    let encoded = encode_to_vec(&42u32).expect("encode u32");
    let (value, consumed): (u32, usize) = decode_from_slice(&encoded).expect("decode u32");
    assert_eq!(value, 42u32);
    assert_eq!(
        consumed,
        encoded.len(),
        "consumed must equal exact buffer length"
    );
}

// ---------------------------------------------------------------------------
// Test 2: decode_from_slice with trailing bytes — consumed < buf.len()
// ---------------------------------------------------------------------------

#[test]
fn test_decode_from_slice_with_trailing_bytes() {
    let mut buf = encode_to_vec(&7u32).expect("encode u32");
    let encoded_len = buf.len();
    buf.extend_from_slice(&[0xAA, 0xBB, 0xCC, 0xDD]);
    let (value, consumed): (u32, usize) = decode_from_slice(&buf).expect("decode u32");
    assert_eq!(value, 7u32);
    assert_eq!(consumed, encoded_len);
    assert!(
        consumed < buf.len(),
        "consumed must be less than total buffer"
    );
}

// ---------------------------------------------------------------------------
// Test 3: encode_into_slice — encode u32 into pre-allocated [u8; 10] buffer
// ---------------------------------------------------------------------------

#[test]
fn test_encode_into_slice_u32_preallocated_buffer() {
    let mut buf = [0u8; 10];
    let n = encode_into_slice(100u32, &mut buf, config::standard()).expect("encode into slice");
    assert!(n > 0, "must write at least one byte");
    assert!(n <= 10, "must not exceed buffer size");
    let (value, _): (u32, usize) = decode_from_slice(&buf[..n]).expect("decode from slice");
    assert_eq!(value, 100u32);
}

// ---------------------------------------------------------------------------
// Test 4: encode_into_slice — returns number of bytes written
// ---------------------------------------------------------------------------

#[test]
fn test_encode_into_slice_returns_bytes_written() {
    let mut buf = [0u8; 32];
    let n = encode_into_slice(255u8, &mut buf, config::standard()).expect("encode u8");
    // u8 always encodes as 1 byte
    assert_eq!(n, 1, "u8 must encode to exactly 1 byte");
}

// ---------------------------------------------------------------------------
// Test 5: encode_into_slice — buffer too small returns error
// ---------------------------------------------------------------------------

#[test]
fn test_encode_into_slice_buffer_too_small_returns_error() {
    // A long string will not fit into 2 bytes
    let long_str = "hello world this is a long string".to_string();
    let mut tiny_buf = [0u8; 2];
    let result = encode_into_slice(long_str, &mut tiny_buf, config::standard());
    assert!(result.is_err(), "encode into too-small buffer must fail");
}

// ---------------------------------------------------------------------------
// Test 6: encode_into_slice — exact-size buffer works
// ---------------------------------------------------------------------------

#[test]
fn test_encode_into_slice_exact_size_buffer_works() {
    // u8 encodes to 1 byte, so a 1-byte buffer is exact
    let val: u8 = 77;
    let mut buf = [0u8; 1];
    let n = encode_into_slice(val, &mut buf, config::standard()).expect("encode exact buffer");
    assert_eq!(n, 1);
    assert_eq!(buf[0], val, "single-byte value must equal raw byte");
}

// ---------------------------------------------------------------------------
// Test 7: Multiple sequential decode_from_slice from same buffer (using offset)
// ---------------------------------------------------------------------------

#[test]
fn test_multiple_sequential_decode_from_slice_with_offset() {
    let mut enc = Vec::new();
    enc.extend_from_slice(&encode_to_vec(&42u32).expect("encode 1"));
    enc.extend_from_slice(&encode_to_vec(&99u32).expect("encode 2"));
    let (first, n): (u32, usize) = decode_from_slice(&enc).expect("decode 1");
    let (second, _): (u32, usize) = decode_from_slice(&enc[n..]).expect("decode 2");
    assert_eq!(first, 42u32);
    assert_eq!(second, 99u32);
}

// ---------------------------------------------------------------------------
// Test 8: decode_from_slice on buffer with concatenated values — decode first then second
// ---------------------------------------------------------------------------

#[test]
fn test_decode_from_slice_concatenated_values() {
    let mut enc = Vec::new();
    enc.extend_from_slice(&encode_to_vec(&1000u64).expect("encode u64"));
    enc.extend_from_slice(&encode_to_vec(&2000u64).expect("encode u64"));
    let (first, n): (u64, usize) = decode_from_slice(&enc).expect("decode first");
    let (second, _): (u64, usize) = decode_from_slice(&enc[n..]).expect("decode second");
    assert_eq!(first, 1000u64);
    assert_eq!(second, 2000u64);
}

// ---------------------------------------------------------------------------
// Test 9: encode_to_vec then decode_from_slice roundtrip for String
// ---------------------------------------------------------------------------

#[test]
fn test_encode_to_vec_decode_from_slice_string_roundtrip() {
    let original = String::from("roundtrip test string");
    let encoded = encode_to_vec(&original).expect("encode String");
    let (decoded, consumed): (String, usize) = decode_from_slice(&encoded).expect("decode String");
    assert_eq!(decoded, original);
    assert_eq!(consumed, encoded.len());
}

// ---------------------------------------------------------------------------
// Test 10: Batch encode many u32 values, then decode them all
// ---------------------------------------------------------------------------

#[test]
fn test_batch_encode_decode_many_u32_values() {
    let values: Vec<u32> = (0u32..20).collect();
    let encoded = encode_to_vec(&values).expect("encode Vec<u32>");
    let (decoded, consumed): (Vec<u32>, usize) =
        decode_from_slice(&encoded).expect("decode Vec<u32>");
    assert_eq!(decoded, values);
    assert_eq!(consumed, encoded.len());
}

// ---------------------------------------------------------------------------
// Test 11: decode_from_slice_with_config with fixed-int config
// ---------------------------------------------------------------------------

#[test]
fn test_decode_from_slice_with_config_fixed_int() {
    let cfg = config::standard().with_fixed_int_encoding();
    let encoded = encode_to_vec_with_config(&0xCAFEu32, cfg).expect("encode fixed-int");
    let (decoded, consumed): (u32, usize) =
        decode_from_slice_with_config(&encoded, cfg).expect("decode fixed-int");
    assert_eq!(decoded, 0xCAFEu32);
    assert_eq!(consumed, encoded.len());
}

// ---------------------------------------------------------------------------
// Test 12: decode_from_slice_with_config with big-endian config
// ---------------------------------------------------------------------------

#[test]
fn test_decode_from_slice_with_config_big_endian() {
    let cfg = config::standard().with_big_endian();
    let val = 0xDEADBEEFu32;
    let encoded = encode_to_vec_with_config(&val, cfg).expect("encode big-endian");
    let (decoded, consumed): (u32, usize) =
        decode_from_slice_with_config(&encoded, cfg).expect("decode big-endian");
    assert_eq!(decoded, val);
    assert_eq!(consumed, encoded.len());
}

// ---------------------------------------------------------------------------
// Test 13: encode_into_slice with fixed-int config writes 4 bytes for u32
// ---------------------------------------------------------------------------

#[test]
fn test_encode_into_slice_fixed_int_writes_4_bytes_for_u32() {
    let cfg = config::standard().with_fixed_int_encoding();
    let mut buf = [0u8; 16];
    let n = encode_into_slice(1u32, &mut buf, cfg).expect("encode fixed-int u32");
    assert_eq!(n, 4, "fixed-int u32 must always encode to 4 bytes");
}

// ---------------------------------------------------------------------------
// Test 14: encode_into_slice result bytes match encode_to_vec bytes
// ---------------------------------------------------------------------------

#[test]
fn test_encode_into_slice_bytes_match_encode_to_vec() {
    let val = 0x12345678u32;
    let vec_bytes = encode_to_vec(&val).expect("encode_to_vec");
    let mut slice_buf = [0u8; 16];
    let n = encode_into_slice(val, &mut slice_buf, config::standard()).expect("encode_into_slice");
    assert_eq!(
        &slice_buf[..n],
        vec_bytes.as_slice(),
        "slice and vec bytes must match"
    );
}

// ---------------------------------------------------------------------------
// Test 15: decode_from_slice for struct roundtrip
// ---------------------------------------------------------------------------

#[test]
fn test_decode_from_slice_struct_roundtrip() {
    let original = SimpleStruct {
        a: 42,
        b: 1_000_000,
    };
    let encoded = encode_to_vec(&original).expect("encode struct");
    let (decoded, consumed): (SimpleStruct, usize) =
        decode_from_slice(&encoded).expect("decode struct");
    assert_eq!(decoded, original);
    assert_eq!(consumed, encoded.len());
}

// ---------------------------------------------------------------------------
// Test 16: decode_from_slice for enum roundtrip
// ---------------------------------------------------------------------------

#[test]
fn test_decode_from_slice_enum_roundtrip() {
    let variants = [
        SimpleEnum::Alpha,
        SimpleEnum::Beta(99),
        SimpleEnum::Gamma { x: -10, y: 20 },
    ];
    for variant in &variants {
        let encoded = encode_to_vec(variant).expect("encode enum");
        let (decoded, consumed): (SimpleEnum, usize) =
            decode_from_slice(&encoded).expect("decode enum");
        assert_eq!(&decoded, variant);
        assert_eq!(consumed, encoded.len());
    }
}

// ---------------------------------------------------------------------------
// Test 17: Slice with zero bytes — decode returns error
// ---------------------------------------------------------------------------

#[test]
fn test_decode_from_slice_zero_bytes_returns_error() {
    let empty: &[u8] = &[];
    let result: oxicode::Result<(u32, usize)> = decode_from_slice(empty);
    assert!(result.is_err(), "decoding from empty slice must fail");
}

// ---------------------------------------------------------------------------
// Test 18: Slice with one zero byte — decode as u8 returns 0, 1 consumed
// ---------------------------------------------------------------------------

#[test]
fn test_decode_from_slice_one_zero_byte_as_u8() {
    let buf: &[u8] = &[0u8];
    let (value, consumed): (u8, usize) = decode_from_slice(buf).expect("decode single zero byte");
    assert_eq!(value, 0u8);
    assert_eq!(consumed, 1);
}

// ---------------------------------------------------------------------------
// Test 19: encode_into_slice writes to start of buffer (offset 0)
// ---------------------------------------------------------------------------

#[test]
fn test_encode_into_slice_writes_at_offset_zero() {
    let val = 42u32;
    let mut buf = [0xFFu8; 16];
    let n = encode_into_slice(val, &mut buf, config::standard()).expect("encode");
    // The first `n` bytes should have been overwritten (not 0xFF for all positions)
    // Verify by decoding the front
    let (decoded, _): (u32, usize) = decode_from_slice(&buf[..n]).expect("decode front");
    assert_eq!(decoded, val, "value at offset 0 must decode correctly");
    // Bytes after n remain untouched
    assert!(
        buf[n..].iter().all(|&b| b == 0xFF),
        "bytes after written region must be untouched"
    );
}

// ---------------------------------------------------------------------------
// Test 20: encode_into_slice + decode_from_slice produces same value
// ---------------------------------------------------------------------------

#[test]
fn test_encode_into_slice_then_decode_from_slice_roundtrip() {
    let original: u64 = 0xDEAD_BEEF_CAFE_BABEu64;
    let mut buf = [0u8; 16];
    let n = encode_into_slice(original, &mut buf, config::standard()).expect("encode");
    let (decoded, consumed): (u64, usize) = decode_from_slice(&buf[..n]).expect("decode");
    assert_eq!(decoded, original);
    assert_eq!(consumed, n);
}

// ---------------------------------------------------------------------------
// Test 21: Multiple encode_into_slice calls to non-overlapping buffer regions
// ---------------------------------------------------------------------------

#[test]
fn test_multiple_encode_into_slice_non_overlapping_regions() {
    let val_a = 111u32;
    let val_b = 222u32;
    let val_c = 333u32;
    let mut buf = [0u8; 64];

    let n_a = encode_into_slice(val_a, &mut buf, config::standard()).expect("encode a");
    let n_b = encode_into_slice(val_b, &mut buf[n_a..], config::standard()).expect("encode b");
    let n_c =
        encode_into_slice(val_c, &mut buf[n_a + n_b..], config::standard()).expect("encode c");

    let (dec_a, _): (u32, usize) = decode_from_slice(&buf[..n_a]).expect("decode a");
    let (dec_b, _): (u32, usize) = decode_from_slice(&buf[n_a..n_a + n_b]).expect("decode b");
    let (dec_c, _): (u32, usize) =
        decode_from_slice(&buf[n_a + n_b..n_a + n_b + n_c]).expect("decode c");

    assert_eq!(dec_a, val_a);
    assert_eq!(dec_b, val_b);
    assert_eq!(dec_c, val_c);
}

// ---------------------------------------------------------------------------
// Test 22: decode_from_slice consumed count is correct for Vec<u8>
// ---------------------------------------------------------------------------

#[test]
fn test_decode_from_slice_consumed_count_for_vec_u8() {
    let data: Vec<u8> = vec![10, 20, 30, 40, 50, 60, 70, 80];
    let encoded = encode_to_vec(&data).expect("encode Vec<u8>");
    let expected_consumed = encoded.len();
    // Append trailing bytes
    let mut buf = encoded.clone();
    buf.extend_from_slice(&[0xDE, 0xAD]);
    let (decoded, consumed): (Vec<u8>, usize) = decode_from_slice(&buf).expect("decode Vec<u8>");
    assert_eq!(decoded, data);
    assert_eq!(
        consumed, expected_consumed,
        "consumed must equal encoded length, not total buffer"
    );
}
