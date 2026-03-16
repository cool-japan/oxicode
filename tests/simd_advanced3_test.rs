//! Advanced SIMD array encoding tests (set 3).
//!
//! All 22 tests exercise the public `encode_to_vec` / `decode_from_slice` API
//! (and config variants) for fixed-size arrays and Vec types under the `simd`
//! feature.  They complement `simd_advanced2_test.rs` with distinct scenarios
//! focused on type variety, small/large array sizes, and consumed-bytes checks.

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
    encode_to_vec_with_config,
};

// ---------------------------------------------------------------------------
// 1. [u8; 8] roundtrip
// ---------------------------------------------------------------------------
#[test]
fn test_u8_array_8_roundtrip() {
    let arr: [u8; 8] = [0, 1, 2, 3, 127, 128, 254, 255];
    let enc = encode_to_vec(&arr).expect("encode [u8; 8]");
    let (dec, _): ([u8; 8], usize) = decode_from_slice(&enc).expect("decode [u8; 8]");
    assert_eq!(arr, dec);
}

// ---------------------------------------------------------------------------
// 2. [u16; 8] roundtrip
// ---------------------------------------------------------------------------
#[test]
fn test_u16_array_8_roundtrip() {
    let arr: [u16; 8] = [0, 1, 256, 1000, 32767, 32768, 65534, 65535];
    let enc = encode_to_vec(&arr).expect("encode [u16; 8]");
    let (dec, _): ([u16; 8], usize) = decode_from_slice(&enc).expect("decode [u16; 8]");
    assert_eq!(arr, dec);
}

// ---------------------------------------------------------------------------
// 3. [u32; 8] roundtrip
// ---------------------------------------------------------------------------
#[test]
fn test_u32_array_8_roundtrip() {
    let arr: [u32; 8] = core::array::from_fn(|i| (i as u32) * 537_521);
    let enc = encode_to_vec(&arr).expect("encode [u32; 8]");
    let (dec, _): ([u32; 8], usize) = decode_from_slice(&enc).expect("decode [u32; 8]");
    assert_eq!(arr, dec);
}

// ---------------------------------------------------------------------------
// 4. [u64; 4] roundtrip
// ---------------------------------------------------------------------------
#[test]
fn test_u64_array_4_roundtrip() {
    let arr: [u64; 4] = [0, u64::MAX / 3, u64::MAX / 2, u64::MAX];
    let enc = encode_to_vec(&arr).expect("encode [u64; 4]");
    let (dec, _): ([u64; 4], usize) = decode_from_slice(&enc).expect("decode [u64; 4]");
    assert_eq!(arr, dec);
}

// ---------------------------------------------------------------------------
// 5. [i8; 16] roundtrip
// ---------------------------------------------------------------------------
#[test]
fn test_i8_array_16_roundtrip() {
    let arr: [i8; 16] = core::array::from_fn(|i| (i as i8).wrapping_sub(8));
    let enc = encode_to_vec(&arr).expect("encode [i8; 16]");
    let (dec, _): ([i8; 16], usize) = decode_from_slice(&enc).expect("decode [i8; 16]");
    assert_eq!(arr, dec);
}

// ---------------------------------------------------------------------------
// 6. [i16; 8] roundtrip
// ---------------------------------------------------------------------------
#[test]
fn test_i16_array_8_roundtrip() {
    let arr: [i16; 8] = [-32768, -1000, -1, 0, 1, 1000, 32766, 32767];
    let enc = encode_to_vec(&arr).expect("encode [i16; 8]");
    let (dec, _): ([i16; 8], usize) = decode_from_slice(&enc).expect("decode [i16; 8]");
    assert_eq!(arr, dec);
}

// ---------------------------------------------------------------------------
// 7. [i32; 8] roundtrip
// ---------------------------------------------------------------------------
#[test]
fn test_i32_array_8_roundtrip() {
    let arr: [i32; 8] = core::array::from_fn(|i| (i as i32 - 4) * 1_000_000);
    let enc = encode_to_vec(&arr).expect("encode [i32; 8]");
    let (dec, _): ([i32; 8], usize) = decode_from_slice(&enc).expect("decode [i32; 8]");
    assert_eq!(arr, dec);
}

// ---------------------------------------------------------------------------
// 8. [i64; 4] roundtrip
// ---------------------------------------------------------------------------
#[test]
fn test_i64_array_4_roundtrip() {
    let arr: [i64; 4] = [i64::MIN, -1, 0, i64::MAX];
    let enc = encode_to_vec(&arr).expect("encode [i64; 4]");
    let (dec, _): ([i64; 4], usize) = decode_from_slice(&enc).expect("decode [i64; 4]");
    assert_eq!(arr, dec);
}

// ---------------------------------------------------------------------------
// 9. [f32; 8] roundtrip (bit-exact, including special values)
// ---------------------------------------------------------------------------
#[test]
fn test_f32_array_8_roundtrip_bit_exact() {
    let val: [f32; 8] = [
        1.0,
        -1.0,
        f32::INFINITY,
        f32::NEG_INFINITY,
        0.0,
        -0.0,
        1.5,
        -1.5,
    ];
    let enc = encode_to_vec(&val).expect("encode [f32; 8]");
    let (decoded, _): ([f32; 8], usize) = decode_from_slice(&enc).expect("decode [f32; 8]");
    for (a, b) in val.iter().zip(decoded.iter()) {
        assert_eq!(a.to_bits(), b.to_bits(), "f32 must be bit-identical");
    }
}

// ---------------------------------------------------------------------------
// 10. [f64; 4] roundtrip (bit-exact, including special values)
// ---------------------------------------------------------------------------
#[test]
fn test_f64_array_4_roundtrip_bit_exact() {
    let val: [f64; 4] = [f64::INFINITY, f64::NEG_INFINITY, 0.0, -0.0];
    let enc = encode_to_vec(&val).expect("encode [f64; 4]");
    let (decoded, _): ([f64; 4], usize) = decode_from_slice(&enc).expect("decode [f64; 4]");
    for (a, b) in val.iter().zip(decoded.iter()) {
        assert_eq!(a.to_bits(), b.to_bits(), "f64 must be bit-identical");
    }
}

// ---------------------------------------------------------------------------
// 11. [u8; 32] roundtrip
// ---------------------------------------------------------------------------
#[test]
fn test_u8_array_32_roundtrip() {
    let arr: [u8; 32] = core::array::from_fn(|i| (i as u8).wrapping_mul(8));
    let enc = encode_to_vec(&arr).expect("encode [u8; 32]");
    let (dec, _): ([u8; 32], usize) = decode_from_slice(&enc).expect("decode [u8; 32]");
    assert_eq!(arr, dec);
}

// ---------------------------------------------------------------------------
// 12. [u8; 64] roundtrip
// ---------------------------------------------------------------------------
#[test]
fn test_u8_array_64_roundtrip() {
    let arr: [u8; 64] = core::array::from_fn(|i| (i as u8).wrapping_mul(3));
    let enc = encode_to_vec(&arr).expect("encode [u8; 64]");
    let (dec, _): ([u8; 64], usize) = decode_from_slice(&enc).expect("decode [u8; 64]");
    assert_eq!(arr, dec);
}

// ---------------------------------------------------------------------------
// 13. [u8; 128] roundtrip
// ---------------------------------------------------------------------------
#[test]
fn test_u8_array_128_roundtrip() {
    let arr: [u8; 128] = core::array::from_fn(|i| (i as u8).wrapping_mul(2).wrapping_add(1));
    let enc = encode_to_vec(&arr).expect("encode [u8; 128]");
    let (dec, _): ([u8; 128], usize) = decode_from_slice(&enc).expect("decode [u8; 128]");
    assert_eq!(arr, dec);
}

// ---------------------------------------------------------------------------
// 14. Vec<u8> 1024 bytes roundtrip
// ---------------------------------------------------------------------------
#[test]
fn test_vec_u8_1024_roundtrip() {
    let data: Vec<u8> = (0u32..1024)
        .map(|i| i.wrapping_mul(6_700_417).wrapping_add(17) as u8)
        .collect();
    let enc = encode_to_vec(&data).expect("encode Vec<u8> 1024");
    let (dec, _): (Vec<u8>, usize) = decode_from_slice(&enc).expect("decode Vec<u8> 1024");
    assert_eq!(data, dec);
}

// ---------------------------------------------------------------------------
// 15. Vec<u32> 256 elements roundtrip
// ---------------------------------------------------------------------------
#[test]
fn test_vec_u32_256_roundtrip() {
    let data: Vec<u32> = (0u32..256).map(|i| i.wrapping_mul(16_777_619)).collect();
    let enc = encode_to_vec(&data).expect("encode Vec<u32> 256");
    let (dec, _): (Vec<u32>, usize) = decode_from_slice(&enc).expect("decode Vec<u32> 256");
    assert_eq!(data, dec);
}

// ---------------------------------------------------------------------------
// 16. Vec<f32> 128 elements roundtrip (bit-exact)
// ---------------------------------------------------------------------------
#[test]
fn test_vec_f32_128_roundtrip_bit_exact() {
    let data: Vec<f32> = (0u32..128)
        .map(|i| (i as f32) * std::f32::consts::LN_2)
        .collect();
    let enc = encode_to_vec(&data).expect("encode Vec<f32> 128");
    let (dec, _): (Vec<f32>, usize) = decode_from_slice(&enc).expect("decode Vec<f32> 128");
    for (a, b) in data.iter().zip(dec.iter()) {
        assert_eq!(
            a.to_bits(),
            b.to_bits(),
            "Vec<f32> element must be bit-identical"
        );
    }
}

// ---------------------------------------------------------------------------
// 17. [u32; 16] with fixed_int_encoding config roundtrip
// ---------------------------------------------------------------------------
#[test]
fn test_u32_array_16_fixed_int_config_roundtrip() {
    let arr: [u32; 16] = core::array::from_fn(|i| (i as u32) * 65537);
    let cfg = config::standard().with_fixed_int_encoding();
    let enc = encode_to_vec_with_config(&arr, cfg).expect("encode [u32; 16] fixed_int");
    let (dec, _): ([u32; 16], usize) =
        decode_from_slice_with_config(&enc, cfg).expect("decode [u32; 16] fixed_int");
    assert_eq!(arr, dec);
}

// ---------------------------------------------------------------------------
// 18. [u8; 4] roundtrip (small array)
// ---------------------------------------------------------------------------
#[test]
fn test_u8_array_4_roundtrip_small() {
    let arr: [u8; 4] = [0, 42, 127, 255];
    let enc = encode_to_vec(&arr).expect("encode [u8; 4]");
    let (dec, _): ([u8; 4], usize) = decode_from_slice(&enc).expect("decode [u8; 4]");
    assert_eq!(arr, dec);
}

// ---------------------------------------------------------------------------
// 19. Vec<u64> 64 elements roundtrip
// ---------------------------------------------------------------------------
#[test]
fn test_vec_u64_64_roundtrip() {
    let data: Vec<u64> = (0u64..64)
        .map(|i| i.wrapping_mul(1_000_000_007).wrapping_add(999_999_937))
        .collect();
    let enc = encode_to_vec(&data).expect("encode Vec<u64> 64");
    let (dec, _): (Vec<u64>, usize) = decode_from_slice(&enc).expect("decode Vec<u64> 64");
    assert_eq!(data, dec);
}

// ---------------------------------------------------------------------------
// 20. [i32; 32] roundtrip
// ---------------------------------------------------------------------------
#[test]
fn test_i32_array_32_roundtrip() {
    let arr: [i32; 32] = core::array::from_fn(|i| (i as i32 - 16) * 77_777);
    let enc = encode_to_vec(&arr).expect("encode [i32; 32]");
    let (dec, _): ([i32; 32], usize) = decode_from_slice(&enc).expect("decode [i32; 32]");
    assert_eq!(arr, dec);
}

// ---------------------------------------------------------------------------
// 21. [u8; 256] roundtrip
// ---------------------------------------------------------------------------
#[test]
fn test_u8_array_256_roundtrip() {
    let arr: [u8; 256] = core::array::from_fn(|i| i as u8);
    let enc = encode_to_vec(&arr).expect("encode [u8; 256]");
    let (dec, _): ([u8; 256], usize) = decode_from_slice(&enc).expect("decode [u8; 256]");
    assert_eq!(arr, dec);
}

// ---------------------------------------------------------------------------
// 22. [u32; 64] consumed bytes equals encoded len
// ---------------------------------------------------------------------------
#[test]
fn test_u32_array_64_consumed_bytes_equals_encoded_len() {
    let arr: [u32; 64] = core::array::from_fn(|i| (i as u32).wrapping_mul(2_654_435_761));
    let enc = encode_to_vec(&arr).expect("encode [u32; 64]");
    let enc_len = enc.len();
    let (_, consumed): ([u32; 64], usize) =
        decode_from_slice(&enc).expect("decode [u32; 64] consumed");
    assert_eq!(
        consumed, enc_len,
        "consumed bytes must equal total encoded length for [u32; 64]"
    );
}
