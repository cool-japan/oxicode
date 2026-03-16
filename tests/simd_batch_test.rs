//! SIMD-accelerated batch encoding path tests.
//!
//! These tests exercise the standard `encode_to_vec` / `decode_from_slice` API
//! with large primitive arrays to hit SIMD-optimized code paths internally.
//! All tests verify correctness of encoded/decoded output.

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
use oxicode::{decode_from_slice, encode_to_vec};

// ---------------------------------------------------------------------------
// 1. Large Vec<u8> 1024 elements roundtrip
// ---------------------------------------------------------------------------

#[test]
fn test_simd_batch_large_vec_u8_1024_elements() {
    let original: Vec<u8> = (0u32..1024).map(|i| (i & 0xFF) as u8).collect();
    let encoded = encode_to_vec(&original).expect("encode Vec<u8> 1024 failed");
    let (decoded, consumed): (Vec<u8>, _) =
        decode_from_slice(&encoded).expect("decode Vec<u8> 1024 failed");
    assert_eq!(decoded, original);
    assert_eq!(consumed, encoded.len());
}

// ---------------------------------------------------------------------------
// 2. Large Vec<u16> 512 elements roundtrip
// ---------------------------------------------------------------------------

#[test]
fn test_simd_batch_large_vec_u16_512_elements() {
    let original: Vec<u16> = (0u16..512).collect();
    let encoded = encode_to_vec(&original).expect("encode Vec<u16> 512 failed");
    let (decoded, consumed): (Vec<u16>, _) =
        decode_from_slice(&encoded).expect("decode Vec<u16> 512 failed");
    assert_eq!(decoded, original);
    assert_eq!(consumed, encoded.len());
}

// ---------------------------------------------------------------------------
// 3. Large Vec<u32> 256 elements roundtrip
// ---------------------------------------------------------------------------

#[test]
fn test_simd_batch_large_vec_u32_256_elements() {
    let original: Vec<u32> = (0u32..256).collect();
    let encoded = encode_to_vec(&original).expect("encode Vec<u32> 256 failed");
    let (decoded, consumed): (Vec<u32>, _) =
        decode_from_slice(&encoded).expect("decode Vec<u32> 256 failed");
    assert_eq!(decoded, original);
    assert_eq!(consumed, encoded.len());
}

// ---------------------------------------------------------------------------
// 4. Large Vec<u64> 128 elements roundtrip
// ---------------------------------------------------------------------------

#[test]
fn test_simd_batch_large_vec_u64_128_elements() {
    let original: Vec<u64> = (0u64..128)
        .map(|i| i.wrapping_mul(6_364_136_223_846_793_005))
        .collect();
    let encoded = encode_to_vec(&original).expect("encode Vec<u64> 128 failed");
    let (decoded, consumed): (Vec<u64>, _) =
        decode_from_slice(&encoded).expect("decode Vec<u64> 128 failed");
    assert_eq!(decoded, original);
    assert_eq!(consumed, encoded.len());
}

// ---------------------------------------------------------------------------
// 5. Large Vec<i8> 1024 elements roundtrip
// ---------------------------------------------------------------------------

#[test]
fn test_simd_batch_large_vec_i8_1024_elements() {
    let original: Vec<i8> = (0i32..1024).map(|i| (i as i8).wrapping_mul(3)).collect();
    let encoded = encode_to_vec(&original).expect("encode Vec<i8> 1024 failed");
    let (decoded, consumed): (Vec<i8>, _) =
        decode_from_slice(&encoded).expect("decode Vec<i8> 1024 failed");
    assert_eq!(decoded, original);
    assert_eq!(consumed, encoded.len());
}

// ---------------------------------------------------------------------------
// 6. Large Vec<i32> 256 elements roundtrip
// ---------------------------------------------------------------------------

#[test]
fn test_simd_batch_large_vec_i32_256_elements() {
    let original: Vec<i32> = (0i32..256).map(|i| i - 128).collect();
    let encoded = encode_to_vec(&original).expect("encode Vec<i32> 256 failed");
    let (decoded, consumed): (Vec<i32>, _) =
        decode_from_slice(&encoded).expect("decode Vec<i32> 256 failed");
    assert_eq!(decoded, original);
    assert_eq!(consumed, encoded.len());
}

// ---------------------------------------------------------------------------
// 7. Large Vec<f32> 256 elements roundtrip
// ---------------------------------------------------------------------------

#[test]
fn test_simd_batch_large_vec_f32_256_elements() {
    let original: Vec<f32> = (0u32..256).map(|i| i as f32 * 0.5).collect();
    let encoded = encode_to_vec(&original).expect("encode Vec<f32> 256 failed");
    let (decoded, consumed): (Vec<f32>, _) =
        decode_from_slice(&encoded).expect("decode Vec<f32> 256 failed");
    assert_eq!(decoded.len(), original.len());
    for (a, b) in original.iter().zip(decoded.iter()) {
        assert_eq!(a.to_bits(), b.to_bits(), "f32 bit mismatch");
    }
    assert_eq!(consumed, encoded.len());
}

// ---------------------------------------------------------------------------
// 8. Large Vec<f64> 128 elements roundtrip
// ---------------------------------------------------------------------------

#[test]
fn test_simd_batch_large_vec_f64_128_elements() {
    let original: Vec<f64> = (0u32..128).map(|i| i as f64 * 1.23456789).collect();
    let encoded = encode_to_vec(&original).expect("encode Vec<f64> 128 failed");
    let (decoded, consumed): (Vec<f64>, _) =
        decode_from_slice(&encoded).expect("decode Vec<f64> 128 failed");
    assert_eq!(decoded.len(), original.len());
    for (a, b) in original.iter().zip(decoded.iter()) {
        assert_eq!(a.to_bits(), b.to_bits(), "f64 bit mismatch");
    }
    assert_eq!(consumed, encoded.len());
}

// ---------------------------------------------------------------------------
// 9. Odd-size array: Vec<u32> with 7 elements (not SIMD-aligned)
// ---------------------------------------------------------------------------

#[test]
fn test_simd_batch_odd_size_vec_u32_7_elements() {
    let original: Vec<u32> = vec![1, 3, 5, 7, 9, 11, 13];
    let encoded = encode_to_vec(&original).expect("encode Vec<u32> 7 failed");
    let (decoded, consumed): (Vec<u32>, _) =
        decode_from_slice(&encoded).expect("decode Vec<u32> 7 failed");
    assert_eq!(decoded, original);
    assert_eq!(consumed, encoded.len());
}

// ---------------------------------------------------------------------------
// 10. Odd-size array: Vec<u64> with 15 elements
// ---------------------------------------------------------------------------

#[test]
fn test_simd_batch_odd_size_vec_u64_15_elements() {
    let original: Vec<u64> = (1u64..=15).map(|i| i * 1_000_000_007).collect();
    let encoded = encode_to_vec(&original).expect("encode Vec<u64> 15 failed");
    let (decoded, consumed): (Vec<u64>, _) =
        decode_from_slice(&encoded).expect("decode Vec<u64> 15 failed");
    assert_eq!(decoded, original);
    assert_eq!(consumed, encoded.len());
}

// ---------------------------------------------------------------------------
// 11. Empty array: Vec<u32> empty roundtrip
// ---------------------------------------------------------------------------

#[test]
fn test_simd_batch_empty_vec_u32_roundtrip() {
    let original: Vec<u32> = vec![];
    let encoded = encode_to_vec(&original).expect("encode empty Vec<u32> failed");
    let (decoded, consumed): (Vec<u32>, _) =
        decode_from_slice(&encoded).expect("decode empty Vec<u32> failed");
    assert_eq!(decoded, original);
    assert_eq!(consumed, encoded.len());
}

// ---------------------------------------------------------------------------
// 12. Single-element Vec<u64> roundtrip
// ---------------------------------------------------------------------------

#[test]
fn test_simd_batch_single_element_vec_u64() {
    let original: Vec<u64> = vec![u64::MAX / 3];
    let encoded = encode_to_vec(&original).expect("encode single-element Vec<u64> failed");
    let (decoded, consumed): (Vec<u64>, _) =
        decode_from_slice(&encoded).expect("decode single-element Vec<u64> failed");
    assert_eq!(decoded, original);
    assert_eq!(consumed, encoded.len());
}

// ---------------------------------------------------------------------------
// 13. Very large Vec<u8> 65536 elements roundtrip (verify consumed == len)
// ---------------------------------------------------------------------------

#[test]
fn test_simd_batch_very_large_vec_u8_65536_elements() {
    let original: Vec<u8> = (0u32..65536)
        .map(|i| i.wrapping_mul(1_664_525).wrapping_add(1_013_904_223) as u8)
        .collect();
    let encoded = encode_to_vec(&original).expect("encode Vec<u8> 65536 failed");
    let (decoded, consumed): (Vec<u8>, _) =
        decode_from_slice(&encoded).expect("decode Vec<u8> 65536 failed");
    assert_eq!(decoded, original);
    assert_eq!(consumed, encoded.len());
}

// ---------------------------------------------------------------------------
// 14. Vec<u8> with all byte values 0-255 repeated 4 times (1024 bytes)
// ---------------------------------------------------------------------------

#[test]
fn test_simd_batch_vec_u8_all_values_repeated_4x() {
    let original: Vec<u8> = (0u32..1024).map(|i| (i & 0xFF) as u8).collect();
    let encoded = encode_to_vec(&original).expect("encode Vec<u8> all values x4 failed");
    let (decoded, consumed): (Vec<u8>, _) =
        decode_from_slice(&encoded).expect("decode Vec<u8> all values x4 failed");
    assert_eq!(decoded, original);
    assert_eq!(consumed, encoded.len());
}

// ---------------------------------------------------------------------------
// 15. Vec<u32> with sequential values 0..256
// ---------------------------------------------------------------------------

#[test]
fn test_simd_batch_vec_u32_sequential_0_to_255() {
    let original: Vec<u32> = (0u32..256).collect();
    let encoded = encode_to_vec(&original).expect("encode Vec<u32> sequential failed");
    let (decoded, consumed): (Vec<u32>, _) =
        decode_from_slice(&encoded).expect("decode Vec<u32> sequential failed");
    assert_eq!(decoded, original);
    assert_eq!(consumed, encoded.len());
}

// ---------------------------------------------------------------------------
// 16. Vec<i64> with min/max/zero values mixed
// ---------------------------------------------------------------------------

#[test]
fn test_simd_batch_vec_i64_min_max_zero_mixed() {
    let original: Vec<i64> = (0usize..256)
        .map(|i| match i % 4 {
            0 => i64::MIN,
            1 => i64::MAX,
            2 => 0i64,
            _ => -(i as i64) * 1_000_000,
        })
        .collect();
    let encoded = encode_to_vec(&original).expect("encode Vec<i64> mixed failed");
    let (decoded, consumed): (Vec<i64>, _) =
        decode_from_slice(&encoded).expect("decode Vec<i64> mixed failed");
    assert_eq!(decoded, original);
    assert_eq!(consumed, encoded.len());
}

// ---------------------------------------------------------------------------
// 17. Consecutive encode/decode cycles (encode result, decode, re-encode, compare)
// ---------------------------------------------------------------------------

#[test]
fn test_simd_batch_consecutive_encode_decode_cycles() {
    let original: Vec<u32> = (0u32..128).map(|i| i.wrapping_mul(2_654_435_761)).collect();

    let encoded_first = encode_to_vec(&original).expect("first encode failed");
    let (decoded_first, _): (Vec<u32>, _) =
        decode_from_slice(&encoded_first).expect("first decode failed");
    assert_eq!(decoded_first, original);

    let encoded_second = encode_to_vec(&decoded_first).expect("second encode failed");
    let (decoded_second, consumed): (Vec<u32>, _) =
        decode_from_slice(&encoded_second).expect("second decode failed");
    assert_eq!(decoded_second, original);
    assert_eq!(
        encoded_first, encoded_second,
        "re-encode must produce identical bytes"
    );
    assert_eq!(consumed, encoded_second.len());
}

// ---------------------------------------------------------------------------
// 18. Vec<f32> with special values (0.0, 1.0, -1.0, f32::MAX, f32::MIN_POSITIVE)
// ---------------------------------------------------------------------------

#[test]
fn test_simd_batch_vec_f32_special_values() {
    // Repeat the special values enough times to exercise SIMD lane width
    let base = [0.0f32, 1.0, -1.0, f32::MAX, f32::MIN_POSITIVE];
    let original: Vec<f32> = base.iter().copied().cycle().take(128).collect();
    let encoded = encode_to_vec(&original).expect("encode Vec<f32> special values failed");
    let (decoded, consumed): (Vec<f32>, _) =
        decode_from_slice(&encoded).expect("decode Vec<f32> special values failed");
    assert_eq!(decoded.len(), original.len());
    for (a, b) in original.iter().zip(decoded.iter()) {
        assert_eq!(a.to_bits(), b.to_bits(), "f32 special value bit mismatch");
    }
    assert_eq!(consumed, encoded.len());
}

// ---------------------------------------------------------------------------
// 19. Large [u8; 256] fixed array roundtrip
// ---------------------------------------------------------------------------

#[test]
fn test_simd_batch_fixed_array_u8_256_roundtrip() {
    let original: [u8; 256] = core::array::from_fn(|i| i as u8);
    let encoded = encode_to_vec(&original).expect("encode [u8; 256] failed");
    let (decoded, consumed): ([u8; 256], _) =
        decode_from_slice(&encoded).expect("decode [u8; 256] failed");
    assert_eq!(decoded, original);
    assert_eq!(consumed, encoded.len());
}

// ---------------------------------------------------------------------------
// 20. Large [u32; 64] fixed array roundtrip
// ---------------------------------------------------------------------------

#[test]
fn test_simd_batch_fixed_array_u32_64_roundtrip() {
    let original: [u32; 64] = core::array::from_fn(|i| (i as u32) * 7 + 3);
    let encoded = encode_to_vec(&original).expect("encode [u32; 64] failed");
    let (decoded, consumed): ([u32; 64], _) =
        decode_from_slice(&encoded).expect("decode [u32; 64] failed");
    assert_eq!(decoded, original);
    assert_eq!(consumed, encoded.len());
}

// ---------------------------------------------------------------------------
// 21. Vec<u32> with fixed-int encoding config (each element 4 bytes)
// ---------------------------------------------------------------------------

#[test]
fn test_simd_batch_vec_u32_fixed_int_config_roundtrip() {
    use oxicode::{config, decode_from_slice_with_config, encode_to_vec_with_config};

    let original: Vec<u32> = (0u32..256).map(|i| i.wrapping_mul(999_983)).collect();
    let cfg = config::legacy();
    let encoded =
        encode_to_vec_with_config(&original, cfg).expect("encode Vec<u32> fixed-int config failed");
    let (decoded, consumed): (Vec<u32>, _) = decode_from_slice_with_config(&encoded, cfg)
        .expect("decode Vec<u32> fixed-int config failed");
    assert_eq!(decoded, original);
    assert_eq!(consumed, encoded.len());
}

// ---------------------------------------------------------------------------
// 22. Vec<u64> with big-endian config roundtrip
// ---------------------------------------------------------------------------

#[test]
fn test_simd_batch_vec_u64_big_endian_config_roundtrip() {
    use oxicode::{config, decode_from_slice_with_config, encode_to_vec_with_config};

    let original: Vec<u64> = (0u64..128)
        .map(|i| i.wrapping_mul(9_999_999_999_999_999_937))
        .collect();
    let cfg = config::standard().with_big_endian();
    let encoded =
        encode_to_vec_with_config(&original, cfg).expect("encode Vec<u64> big-endian failed");
    let (decoded, consumed): (Vec<u64>, _) =
        decode_from_slice_with_config(&encoded, cfg).expect("decode Vec<u64> big-endian failed");
    assert_eq!(decoded, original);
    assert_eq!(consumed, encoded.len());
}
