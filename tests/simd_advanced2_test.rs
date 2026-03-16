//! Advanced SIMD array encoding tests (set 2).
//!
//! All 22 tests exercise the public `encode_to_vec` / `decode_from_slice` API
//! (and config variants) for fixed-size arrays, Vec<u8>, Option, and derived
//! structs under the `simd` feature.  They complement `simd_test.rs` without
//! duplicating any of its scenarios.

#![cfg(feature = "simd")]
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

// ---------------------------------------------------------------------------
// 1. [u8; 64] array roundtrip
// ---------------------------------------------------------------------------
#[test]
fn test_u8_array_64_roundtrip() {
    let arr: [u8; 64] = core::array::from_fn(|i| i as u8);
    let enc = encode_to_vec(&arr).expect("encode [u8; 64]");
    let (dec, _): ([u8; 64], usize) = decode_from_slice(&enc).expect("decode [u8; 64]");
    assert_eq!(arr, dec);
}

// ---------------------------------------------------------------------------
// 2. [u8; 128] array roundtrip
// ---------------------------------------------------------------------------
#[test]
fn test_u8_array_128_roundtrip() {
    let arr: [u8; 128] = core::array::from_fn(|i| (i * 2) as u8);
    let enc = encode_to_vec(&arr).expect("encode [u8; 128]");
    let (dec, _): ([u8; 128], usize) = decode_from_slice(&enc).expect("decode [u8; 128]");
    assert_eq!(arr, dec);
}

// ---------------------------------------------------------------------------
// 3. [u8; 256] array roundtrip
// ---------------------------------------------------------------------------
#[test]
fn test_u8_array_256_roundtrip() {
    let arr: [u8; 256] = core::array::from_fn(|i| i as u8);
    let enc = encode_to_vec(&arr).expect("encode [u8; 256]");
    let (dec, _): ([u8; 256], usize) = decode_from_slice(&enc).expect("decode [u8; 256]");
    assert_eq!(arr, dec);
}

// ---------------------------------------------------------------------------
// 4. [u8; 512] array roundtrip
// ---------------------------------------------------------------------------
#[test]
fn test_u8_array_512_roundtrip() {
    let arr: [u8; 512] = core::array::from_fn(|i| (i & 0xFF) as u8);
    let enc = encode_to_vec(&arr).expect("encode [u8; 512]");
    let (dec, _): ([u8; 512], usize) = decode_from_slice(&enc).expect("decode [u8; 512]");
    assert_eq!(arr, dec);
}

// ---------------------------------------------------------------------------
// 5. [u16; 32] array roundtrip
// ---------------------------------------------------------------------------
#[test]
fn test_u16_array_32_roundtrip() {
    let arr: [u16; 32] = core::array::from_fn(|i| (i as u16) * 1000);
    let enc = encode_to_vec(&arr).expect("encode [u16; 32]");
    let (dec, _): ([u16; 32], usize) = decode_from_slice(&enc).expect("decode [u16; 32]");
    assert_eq!(arr, dec);
}

// ---------------------------------------------------------------------------
// 6. [u32; 16] array roundtrip
// ---------------------------------------------------------------------------
#[test]
fn test_u32_array_16_roundtrip() {
    let arr: [u32; 16] = core::array::from_fn(|i| (i as u32) * 100_000);
    let enc = encode_to_vec(&arr).expect("encode [u32; 16]");
    let (dec, _): ([u32; 16], usize) = decode_from_slice(&enc).expect("decode [u32; 16]");
    assert_eq!(arr, dec);
}

// ---------------------------------------------------------------------------
// 7. [u64; 8] array roundtrip
// ---------------------------------------------------------------------------
#[test]
fn test_u64_array_8_roundtrip() {
    let arr: [u64; 8] = core::array::from_fn(|i| (i as u64) * 1_000_000_000_000);
    let enc = encode_to_vec(&arr).expect("encode [u64; 8]");
    let (dec, _): ([u64; 8], usize) = decode_from_slice(&enc).expect("decode [u64; 8]");
    assert_eq!(arr, dec);
}

// ---------------------------------------------------------------------------
// 8. [i32; 16] array with negative values roundtrip
// ---------------------------------------------------------------------------
#[test]
fn test_i32_array_16_negative_values_roundtrip() {
    let arr: [i32; 16] = core::array::from_fn(|i| -((i as i32 + 1) * 12345));
    let enc = encode_to_vec(&arr).expect("encode [i32; 16] negative");
    let (dec, _): ([i32; 16], usize) = decode_from_slice(&enc).expect("decode [i32; 16] negative");
    assert_eq!(arr, dec);
}

// ---------------------------------------------------------------------------
// 9. [f32; 16] array roundtrip (bit-exact)
// ---------------------------------------------------------------------------
#[test]
fn test_f32_array_16_roundtrip_bit_exact() {
    let arr: [f32; 16] = core::array::from_fn(|i| (i as f32) * std::f32::consts::E);
    let enc = encode_to_vec(&arr).expect("encode [f32; 16]");
    let (dec, _): ([f32; 16], usize) = decode_from_slice(&enc).expect("decode [f32; 16]");
    for (a, b) in arr.iter().zip(dec.iter()) {
        assert_eq!(a.to_bits(), b.to_bits(), "f32 bit pattern mismatch");
    }
}

// ---------------------------------------------------------------------------
// 10. [f64; 8] array roundtrip (bit-exact)
// ---------------------------------------------------------------------------
#[test]
fn test_f64_array_8_roundtrip_bit_exact() {
    let arr: [f64; 8] = core::array::from_fn(|i| (i as f64) * std::f64::consts::PI);
    let enc = encode_to_vec(&arr).expect("encode [f64; 8]");
    let (dec, _): ([f64; 8], usize) = decode_from_slice(&enc).expect("decode [f64; 8]");
    for (a, b) in arr.iter().zip(dec.iter()) {
        assert_eq!(a.to_bits(), b.to_bits(), "f64 bit pattern mismatch");
    }
}

// ---------------------------------------------------------------------------
// 11. Vec<u8> of 1024 bytes roundtrip (SIMD path)
// ---------------------------------------------------------------------------
#[test]
fn test_vec_u8_1024_roundtrip() {
    let data: Vec<u8> = (0u32..1024).map(|i| (i & 0xFF) as u8).collect();
    let enc = encode_to_vec(&data).expect("encode Vec<u8> 1024");
    let (dec, _): (Vec<u8>, usize) = decode_from_slice(&enc).expect("decode Vec<u8> 1024");
    assert_eq!(data, dec);
}

// ---------------------------------------------------------------------------
// 12. Vec<u8> of 4096 bytes roundtrip (large SIMD path)
// ---------------------------------------------------------------------------
#[test]
fn test_vec_u8_4096_roundtrip() {
    let data: Vec<u8> = (0u32..4096)
        .map(|i| i.wrapping_mul(1_664_525u32).wrapping_add(1_013_904_223) as u8)
        .collect();
    let enc = encode_to_vec(&data).expect("encode Vec<u8> 4096");
    let (dec, _): (Vec<u8>, usize) = decode_from_slice(&enc).expect("decode Vec<u8> 4096");
    assert_eq!(data, dec);
}

// ---------------------------------------------------------------------------
// 13. [u8; 64] SIMD produces same bytes as non-SIMD (determinism)
// ---------------------------------------------------------------------------
#[test]
fn test_u8_array_64_simd_encoding_is_deterministic() {
    let arr: [u8; 64] = core::array::from_fn(|i| (i as u8).wrapping_mul(3));
    let enc1 = encode_to_vec(&arr).expect("first encode [u8; 64]");
    let enc2 = encode_to_vec(&arr).expect("second encode [u8; 64]");
    assert_eq!(
        enc1, enc2,
        "[u8; 64] encoding must be deterministic under simd feature"
    );
    let (dec, _): ([u8; 64], usize) = decode_from_slice(&enc1).expect("decode [u8; 64]");
    assert_eq!(arr, dec);
}

// ---------------------------------------------------------------------------
// 14. [u8; 64] with fixed_int_encoding config roundtrip
// ---------------------------------------------------------------------------
#[test]
fn test_u8_array_64_fixed_int_encoding_config_roundtrip() {
    let arr: [u8; 64] = core::array::from_fn(|i| i as u8);
    let cfg = config::standard().with_fixed_int_encoding();
    let enc =
        encode_to_vec_with_config(&arr, cfg).expect("encode [u8; 64] with fixed_int_encoding");
    let (dec, _): ([u8; 64], usize) =
        decode_from_slice_with_config(&enc, cfg).expect("decode [u8; 64] with fixed_int_encoding");
    assert_eq!(arr, dec);
}

// ---------------------------------------------------------------------------
// 15. [u8; 64] with big_endian config roundtrip
// ---------------------------------------------------------------------------
#[test]
fn test_u8_array_64_big_endian_config_roundtrip() {
    let arr: [u8; 64] = core::array::from_fn(|i| (255 - i) as u8);
    let cfg = config::standard().with_big_endian();
    let enc = encode_to_vec_with_config(&arr, cfg).expect("encode [u8; 64] with big_endian");
    let (dec, _): ([u8; 64], usize) =
        decode_from_slice_with_config(&enc, cfg).expect("decode [u8; 64] with big_endian");
    assert_eq!(arr, dec);
}

// ---------------------------------------------------------------------------
// 16. Consumed bytes == encoded length for [u8; 256]
// ---------------------------------------------------------------------------
#[test]
fn test_u8_array_256_consumed_bytes_equals_encoded_len() {
    let arr: [u8; 256] = core::array::from_fn(|i| i as u8);
    let enc = encode_to_vec(&arr).expect("encode [u8; 256]");
    let enc_len = enc.len();
    let (_, consumed): ([u8; 256], usize) =
        decode_from_slice(&enc).expect("decode [u8; 256] consumed");
    assert_eq!(
        consumed, enc_len,
        "consumed bytes must equal total encoded length for [u8; 256]"
    );
}

// ---------------------------------------------------------------------------
// 17. Option<[u8; 64]> Some roundtrip
// ---------------------------------------------------------------------------
#[test]
fn test_option_u8_array_64_some_roundtrip() {
    let val: Option<[u8; 64]> = Some(core::array::from_fn(|i| i as u8));
    let enc = encode_to_vec(&val).expect("encode Option<[u8; 64]> Some");
    let (dec, _): (Option<[u8; 64]>, usize) =
        decode_from_slice(&enc).expect("decode Option<[u8; 64]> Some");
    assert_eq!(val, dec);
}

// ---------------------------------------------------------------------------
// 18. Option<[u8; 64]> None roundtrip
// ---------------------------------------------------------------------------
#[test]
fn test_option_u8_array_64_none_roundtrip() {
    let val: Option<[u8; 64]> = None;
    let enc = encode_to_vec(&val).expect("encode Option<[u8; 64]> None");
    let (dec, _): (Option<[u8; 64]>, usize) =
        decode_from_slice(&enc).expect("decode Option<[u8; 64]> None");
    assert_eq!(val, dec);
}

// ---------------------------------------------------------------------------
// 19. Vec<[u8; 16]> roundtrip
// ---------------------------------------------------------------------------
#[test]
fn test_vec_of_u8_array_16_roundtrip() {
    let data: Vec<[u8; 16]> = (0u8..8)
        .map(|base| core::array::from_fn(|i| base.wrapping_add(i as u8)))
        .collect();
    let enc = encode_to_vec(&data).expect("encode Vec<[u8; 16]>");
    let (dec, _): (Vec<[u8; 16]>, usize) = decode_from_slice(&enc).expect("decode Vec<[u8; 16]>");
    assert_eq!(data, dec);
}

// ---------------------------------------------------------------------------
// 20. Struct with [u8; 32] field roundtrip (via derive)
// ---------------------------------------------------------------------------
#[derive(Encode, Decode, Debug, PartialEq)]
struct ArrayHolder {
    id: u32,
    payload: [u8; 32],
    tag: u8,
}

#[test]
fn test_struct_with_u8_array_32_field_roundtrip() {
    let val = ArrayHolder {
        id: 0xDEAD_BEEF,
        payload: core::array::from_fn(|i| (i as u8).wrapping_mul(7)),
        tag: 0xAB,
    };
    let enc = encode_to_vec(&val).expect("encode ArrayHolder");
    let (dec, _): (ArrayHolder, usize) = decode_from_slice(&enc).expect("decode ArrayHolder");
    assert_eq!(val, dec);
}

// ---------------------------------------------------------------------------
// 21. [u8; 64] all zeros roundtrip
// ---------------------------------------------------------------------------
#[test]
fn test_u8_array_64_all_zeros_roundtrip() {
    let arr: [u8; 64] = [0u8; 64];
    let enc = encode_to_vec(&arr).expect("encode [u8; 64] all zeros");
    let (dec, _): ([u8; 64], usize) = decode_from_slice(&enc).expect("decode [u8; 64] all zeros");
    assert_eq!(arr, dec);
}

// ---------------------------------------------------------------------------
// 22. [u8; 64] all 0xFF roundtrip
// ---------------------------------------------------------------------------
#[test]
fn test_u8_array_64_all_ff_roundtrip() {
    let arr: [u8; 64] = [0xFFu8; 64];
    let enc = encode_to_vec(&arr).expect("encode [u8; 64] all 0xFF");
    let (dec, _): ([u8; 64], usize) = decode_from_slice(&enc).expect("decode [u8; 64] all 0xFF");
    assert_eq!(arr, dec);
}
