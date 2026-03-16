//! Advanced SIMD-accelerated encoding tests.
//!
//! These tests verify correctness of the encode/decode path used by the
//! `simd` feature (bulk byte copy acceleration).  All tests use the public
//! `encode_to_vec` / `decode_from_slice` API and run unconditionally — the
//! SIMD feature simply changes internal dispatch but must not alter results.

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
use oxicode::{config, decode_from_slice, encode_to_vec};

// ---------------------------------------------------------------------------
// 1. Large Vec<u8> (1024 bytes) roundtrip — SIMD bulk copy path
// ---------------------------------------------------------------------------

#[test]
fn test_large_vec_u8_1024_roundtrip() {
    let data: Vec<u8> = (0u32..1024).map(|i| (i & 0xFF) as u8).collect();
    let enc = encode_to_vec(&data).expect("encode 1024 u8");
    let (dec, _): (Vec<u8>, _) = decode_from_slice(&enc).expect("decode 1024 u8");
    assert_eq!(data, dec);
}

// ---------------------------------------------------------------------------
// 2. Large Vec<u8> (4096 bytes) roundtrip
// ---------------------------------------------------------------------------

#[test]
fn test_large_vec_u8_4096_roundtrip() {
    let data: Vec<u8> = (0u32..4096)
        .map(|i| i.wrapping_mul(1_664_525).wrapping_add(1_013_904_223) as u8)
        .collect();
    let enc = encode_to_vec(&data).expect("encode 4096 u8");
    let (dec, _): (Vec<u8>, _) = decode_from_slice(&enc).expect("decode 4096 u8");
    assert_eq!(data, dec);
}

// ---------------------------------------------------------------------------
// 3. &[u8] slice of 512 bytes encode/decode (via Vec<u8> newtype round-trip)
// ---------------------------------------------------------------------------

#[test]
fn test_slice_u8_512_roundtrip() {
    // encode_to_vec requires a Sized value; obtain one by cloning from the slice.
    let source: Vec<u8> = (0u32..512).map(|i| (i.wrapping_mul(7)) as u8).collect();
    let as_slice: &[u8] = &source;
    // Wrap as a Vec so we can encode, then compare result against original slice.
    let owned: Vec<u8> = as_slice.to_owned();
    let enc = encode_to_vec(&owned).expect("encode &[u8] 512");
    let (dec, _): (Vec<u8>, _) = decode_from_slice(&enc).expect("decode &[u8] 512");
    assert_eq!(as_slice, dec.as_slice());
}

// ---------------------------------------------------------------------------
// 4. Large Vec<u32> (256 elements) roundtrip
// ---------------------------------------------------------------------------

#[test]
fn test_large_vec_u32_256_roundtrip() {
    let data: Vec<u32> = (0u32..256).map(|i| i.wrapping_mul(0x9e37_79b9)).collect();
    let enc = encode_to_vec(&data).expect("encode Vec<u32> 256");
    let (dec, _): (Vec<u32>, _) = decode_from_slice(&enc).expect("decode Vec<u32> 256");
    assert_eq!(data, dec);
}

// ---------------------------------------------------------------------------
// 5. Large Vec<u64> (128 elements) roundtrip
// ---------------------------------------------------------------------------

#[test]
fn test_large_vec_u64_128_roundtrip() {
    let data: Vec<u64> = (0u64..128)
        .map(|i| i.wrapping_mul(6_364_136_223_846_793_005))
        .collect();
    let enc = encode_to_vec(&data).expect("encode Vec<u64> 128");
    let (dec, _): (Vec<u64>, _) = decode_from_slice(&enc).expect("decode Vec<u64> 128");
    assert_eq!(data, dec);
}

// ---------------------------------------------------------------------------
// 6. Large Vec<i32> (256 elements) roundtrip
// ---------------------------------------------------------------------------

#[test]
fn test_large_vec_i32_256_roundtrip() {
    let data: Vec<i32> = (0i32..256).map(|i| i * -1031 + 500_000).collect();
    let enc = encode_to_vec(&data).expect("encode Vec<i32> 256");
    let (dec, _): (Vec<i32>, _) = decode_from_slice(&enc).expect("decode Vec<i32> 256");
    assert_eq!(data, dec);
}

// ---------------------------------------------------------------------------
// 7. Large Vec<f64> (128 elements) roundtrip — computed values, no literals
// ---------------------------------------------------------------------------

#[test]
fn test_large_vec_f64_128_roundtrip() {
    let pi = std::f64::consts::PI;
    let e = std::f64::consts::E;
    let data: Vec<f64> = (0usize..128)
        .map(|i| pi * (i as f64) - e * (i as f64 / 2.0))
        .collect();
    let enc = encode_to_vec(&data).expect("encode Vec<f64> 128");
    let (dec, _): (Vec<f64>, _) = decode_from_slice(&enc).expect("decode Vec<f64> 128");
    assert_eq!(data.len(), dec.len());
    for (idx, (a, b)) in data.iter().zip(dec.iter()).enumerate() {
        assert_eq!(a.to_bits(), b.to_bits(), "f64 bit mismatch at index {idx}");
    }
}

// ---------------------------------------------------------------------------
// 8. Large Vec<bool> (1000 elements) roundtrip
// ---------------------------------------------------------------------------

#[test]
fn test_large_vec_bool_1000_roundtrip() {
    let data: Vec<bool> = (0u32..1000).map(|i| i % 3 != 0).collect();
    let enc = encode_to_vec(&data).expect("encode Vec<bool> 1000");
    let (dec, _): (Vec<bool>, _) = decode_from_slice(&enc).expect("decode Vec<bool> 1000");
    assert_eq!(data, dec);
}

// ---------------------------------------------------------------------------
// 9. Large Vec<String> (100 strings) roundtrip
// ---------------------------------------------------------------------------

#[test]
fn test_large_vec_string_100_roundtrip() {
    let data: Vec<String> = (0u32..100)
        .map(|i| format!("simd_test_entry_{:04}_{}", i, "x".repeat((i % 20) as usize)))
        .collect();
    let enc = encode_to_vec(&data).expect("encode Vec<String> 100");
    let (dec, _): (Vec<String>, _) = decode_from_slice(&enc).expect("decode Vec<String> 100");
    assert_eq!(data, dec);
}

// ---------------------------------------------------------------------------
// 10. Vec<u8> at SIMD boundary sizes: 15, 16, 17, 31, 32, 33 bytes
// ---------------------------------------------------------------------------

#[test]
fn test_vec_u8_simd_boundary_sizes() {
    for size in [15usize, 16, 17, 31, 32, 33] {
        let data: Vec<u8> = (0..size).map(|i| (i * 13 + 7) as u8).collect();
        let enc = encode_to_vec(&data).expect("encode boundary size");
        let (dec, _): (Vec<u8>, _) = decode_from_slice(&enc).expect("decode boundary size");
        assert_eq!(data, dec, "boundary size {size} roundtrip mismatch");
    }
}

// ---------------------------------------------------------------------------
// 11. Large Vec<u8> equality: encode then decode matches original
// ---------------------------------------------------------------------------

#[test]
fn test_large_vec_u8_encode_decode_equality() {
    let data: Vec<u8> = (0u32..2048).map(|i| (i.wrapping_mul(31)) as u8).collect();
    let enc = encode_to_vec(&data).expect("encode 2048 u8 for equality");
    let (dec, _): (Vec<u8>, _) = decode_from_slice(&enc).expect("decode 2048 u8 for equality");
    assert_eq!(data, dec, "decoded data must equal original");
    assert_eq!(data.len(), dec.len(), "decoded length must match");
}

// ---------------------------------------------------------------------------
// 12. SIMD + fixed int encoding: large Vec<u32>
// ---------------------------------------------------------------------------

#[test]
fn test_large_vec_u32_fixed_int_encoding() {
    use oxicode::{decode_from_slice_with_config, encode_to_vec_with_config};
    let data: Vec<u32> = (0u32..256).map(|i| i * 1000 + 42).collect();
    let cfg = config::legacy(); // fixed-width integer encoding
    let enc = encode_to_vec_with_config(&data, cfg).expect("encode fixed-int Vec<u32>");
    let (dec, _): (Vec<u32>, _) =
        decode_from_slice_with_config(&enc, cfg).expect("decode fixed-int Vec<u32>");
    assert_eq!(data, dec);
}

// ---------------------------------------------------------------------------
// 13. SIMD + big endian: large Vec<u32>
// ---------------------------------------------------------------------------

#[test]
fn test_large_vec_u32_big_endian_encoding() {
    use oxicode::{decode_from_slice_with_config, encode_to_vec_with_config};
    let data: Vec<u32> = (0u32..256).map(|i| i.wrapping_mul(0xDEAD_BEEF)).collect();
    let cfg = config::standard().with_big_endian();
    let enc = encode_to_vec_with_config(&data, cfg).expect("encode big-endian Vec<u32>");
    let (dec, _): (Vec<u32>, _) =
        decode_from_slice_with_config(&enc, cfg).expect("decode big-endian Vec<u32>");
    assert_eq!(data, dec);
}

// ---------------------------------------------------------------------------
// 14. String of 1000 chars roundtrip
// ---------------------------------------------------------------------------

#[test]
fn test_string_1000_chars_roundtrip() {
    let data: String = (0u32..1000)
        .map(|i| char::from(b'a' + (i % 26) as u8))
        .collect();
    assert_eq!(data.len(), 1000);
    let enc = encode_to_vec(&data).expect("encode 1000-char string");
    let (dec, _): (String, _) = decode_from_slice(&enc).expect("decode 1000-char string");
    assert_eq!(data, dec);
}

// ---------------------------------------------------------------------------
// 15. Vec<Vec<u8>> with multiple large inner vecs
// ---------------------------------------------------------------------------

#[test]
fn test_vec_of_vec_u8_large_inner_roundtrip() {
    let data: Vec<Vec<u8>> = (0u32..8)
        .map(|row| (0u32..256).map(|col| (row * 37 + col) as u8).collect())
        .collect();
    let enc = encode_to_vec(&data).expect("encode Vec<Vec<u8>>");
    let (dec, _): (Vec<Vec<u8>>, _) = decode_from_slice(&enc).expect("decode Vec<Vec<u8>>");
    assert_eq!(data, dec);
}

// ---------------------------------------------------------------------------
// 16. Large nested struct roundtrip
// ---------------------------------------------------------------------------

#[test]
fn test_large_nested_struct_roundtrip() {
    #[derive(oxicode::Encode, oxicode::Decode, PartialEq, Debug)]
    struct Inner {
        values: Vec<u32>,
        tag: u64,
    }

    #[derive(oxicode::Encode, oxicode::Decode, PartialEq, Debug)]
    struct Outer {
        label: String,
        items: Vec<Inner>,
        flags: Vec<bool>,
    }

    let original = Outer {
        label: "simd_nested_struct_test".to_owned(),
        items: (0u32..16)
            .map(|i| Inner {
                values: (0u32..64).map(|j| i * 64 + j).collect(),
                tag: u64::from(i) * 1_000_000,
            })
            .collect(),
        flags: (0u32..64).map(|i| i % 2 == 0).collect(),
    };

    let enc = encode_to_vec(&original).expect("encode nested struct");
    let (dec, _): (Outer, _) = decode_from_slice(&enc).expect("decode nested struct");
    assert_eq!(original, dec);
}

// ---------------------------------------------------------------------------
// 17. Encode/decode of 10 000-byte Vec<u8>
// ---------------------------------------------------------------------------

#[test]
fn test_vec_u8_10000_bytes_roundtrip() {
    let data: Vec<u8> = (0u32..10_000)
        .map(|i| i.wrapping_mul(1_664_525).wrapping_add(1_013_904_223) as u8)
        .collect();
    let enc = encode_to_vec(&data).expect("encode 10000 u8");
    let (dec, _): (Vec<u8>, _) = decode_from_slice(&enc).expect("decode 10000 u8");
    assert_eq!(data, dec);
}

// ---------------------------------------------------------------------------
// 18. Vec<u16> with 512 elements roundtrip
// ---------------------------------------------------------------------------

#[test]
fn test_vec_u16_512_roundtrip() {
    let data: Vec<u16> = (0u16..512).collect();
    let enc = encode_to_vec(&data).expect("encode Vec<u16> 512");
    let (dec, _): (Vec<u16>, _) = decode_from_slice(&enc).expect("decode Vec<u16> 512");
    assert_eq!(data, dec);
}

// ---------------------------------------------------------------------------
// 19. Vec<i64> with 256 elements roundtrip
// ---------------------------------------------------------------------------

#[test]
fn test_vec_i64_256_roundtrip() {
    let data: Vec<i64> = (0i64..256)
        .map(|i| i.saturating_mul(i64::MAX / 256))
        .collect();
    let enc = encode_to_vec(&data).expect("encode Vec<i64> 256");
    let (dec, _): (Vec<i64>, _) = decode_from_slice(&enc).expect("decode Vec<i64> 256");
    assert_eq!(data, dec);
}

// ---------------------------------------------------------------------------
// 20. Mixed large data: struct with multiple large fields
// ---------------------------------------------------------------------------

#[test]
fn test_mixed_large_struct_multiple_fields_roundtrip() {
    #[derive(oxicode::Encode, oxicode::Decode, PartialEq, Debug)]
    struct LargePayload {
        bytes: Vec<u8>,
        words: Vec<u32>,
        longs: Vec<i64>,
        name: String,
        count: u64,
    }

    let original = LargePayload {
        bytes: (0u32..512).map(|i| (i * 7 + 3) as u8).collect(),
        words: (0u32..256).map(|i| i.wrapping_mul(0xCAFE_BABE)).collect(),
        longs: (0i64..128)
            .map(|i| i.saturating_mul(i64::MAX / 128))
            .collect(),
        name: "mixed_large_payload_simd_test".to_owned(),
        count: 0xDEAD_BEEF_CAFE_BABE,
    };

    let enc = encode_to_vec(&original).expect("encode LargePayload");
    let (dec, _): (LargePayload, _) = decode_from_slice(&enc).expect("decode LargePayload");
    assert_eq!(original, dec);
}

// ---------------------------------------------------------------------------
// 21. Large Vec with limit config (set limit to 100_000 bytes)
// ---------------------------------------------------------------------------

#[test]
fn test_large_vec_u8_with_limit_config_roundtrip() {
    use oxicode::{decode_from_slice_with_config, encode_to_vec_with_config};
    // 1024 bytes well within 100_000 byte limit
    let data: Vec<u8> = (0u32..1024).map(|i| (i * 3 + 11) as u8).collect();
    let cfg = config::standard().with_limit::<100_000>();
    let enc = encode_to_vec_with_config(&data, cfg).expect("encode with limit");
    let (dec, _): (Vec<u8>, _) =
        decode_from_slice_with_config(&enc, cfg).expect("decode with limit");
    assert_eq!(data, dec);
}

// ---------------------------------------------------------------------------
// 22. Encode large data, verify decoded length matches
// ---------------------------------------------------------------------------

#[test]
fn test_encode_large_data_decoded_length_matches() {
    const N: usize = 2048;
    let data: Vec<u8> = (0u32..N as u32)
        .map(|i| (i.wrapping_mul(97)) as u8)
        .collect();
    let enc = encode_to_vec(&data).expect("encode large data for length check");
    let (dec, consumed): (Vec<u8>, _) =
        decode_from_slice(&enc).expect("decode large data for length check");
    assert_eq!(
        dec.len(),
        N,
        "decoded Vec length must equal original length"
    );
    assert_eq!(consumed, enc.len(), "all encoded bytes must be consumed");
}
