//! Extended SIMD encoding/decoding tests.
//!
//! 20 comprehensive tests covering large arrays, alignment, special values,
//! struct round-trips, and cross-type consistency.  All SIMD-specific tests
//! are gated with `#[cfg(feature = "simd")]`; tests that only use the
//! standard encode/decode path run unconditionally.

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
#[cfg(feature = "simd")]
mod simd_extended_tests {
    use std::f64::consts::{E, PI};

    use oxicode::simd::{decode_simd_array, encode_simd_array, optimal_alignment, AlignedVec};

    // -----------------------------------------------------------------------
    // 1. Large [u32; 1024] round-trip (reinterpreted through i32)
    // -----------------------------------------------------------------------

    /// Encodes a 1024-element u32 array by reinterpreting each element as i32
    /// bits, round-trips through the SIMD i32 path, and verifies bit-perfect
    /// reconstruction.
    #[test]
    fn test_u32_1024_roundtrip_via_i32_bits() {
        let data: [u32; 1024] = core::array::from_fn(|i| (i as u32).wrapping_mul(0x9e37_79b9));
        let as_i32: Vec<i32> = data.iter().map(|&v| v as i32).collect();
        let encoded = encode_simd_array(&as_i32).expect("encode [u32;1024] via i32 bits");
        let decoded_i32: Vec<i32> =
            decode_simd_array(&encoded).expect("decode [u32;1024] via i32 bits");
        let reconstructed: Vec<u32> = decoded_i32.iter().map(|&v| v as u32).collect();
        assert_eq!(&data[..], reconstructed.as_slice());
    }

    // -----------------------------------------------------------------------
    // 2. Large [i64; 512] round-trip
    // -----------------------------------------------------------------------

    #[test]
    fn test_i64_512_roundtrip() {
        let data: Vec<i64> = (0i64..512)
            .map(|i| i.saturating_mul(i64::MAX / 512))
            .collect();
        let encoded = encode_simd_array(&data).expect("encode [i64;512]");
        let decoded: Vec<i64> = decode_simd_array(&encoded).expect("decode [i64;512]");
        assert_eq!(data, decoded);
    }

    // -----------------------------------------------------------------------
    // 3. Large [f32; 256] round-trip using PI and E as seeds
    // -----------------------------------------------------------------------

    #[test]
    fn test_f32_256_roundtrip_pi_e_seeds() {
        let data: Vec<f32> = (0usize..256)
            .map(|i| {
                let seed = if i % 2 == 0 { PI as f32 } else { E as f32 };
                seed * (i as f32 + 1.0)
            })
            .collect();
        let encoded = encode_simd_array(&data).expect("encode [f32;256]");
        let decoded: Vec<f32> = decode_simd_array(&encoded).expect("decode [f32;256]");
        assert_eq!(data.len(), decoded.len());
        for (a, b) in data.iter().zip(decoded.iter()) {
            assert_eq!(a.to_bits(), b.to_bits(), "f32 bit mismatch at index");
        }
    }

    // -----------------------------------------------------------------------
    // 4. Large [f64; 256] round-trip
    // -----------------------------------------------------------------------

    #[test]
    fn test_f64_256_roundtrip() {
        let data: Vec<f64> = (0usize..256)
            .map(|i| PI * (i as f64) - E * (i as f64 / 2.0))
            .collect();
        let encoded = encode_simd_array(&data).expect("encode [f64;256]");
        let decoded: Vec<f64> = decode_simd_array(&encoded).expect("decode [f64;256]");
        for (a, b) in data.iter().zip(decoded.iter()) {
            assert_eq!(a.to_bits(), b.to_bits(), "f64 bit mismatch");
        }
    }

    // -----------------------------------------------------------------------
    // 5. SIMD encode of Vec<u32> with 10 000 elements (via i32 reinterpret)
    // -----------------------------------------------------------------------

    #[test]
    fn test_vec_u32_10000_encode_decode_roundtrip() {
        let data: Vec<u32> = (0u32..10_000)
            .map(|i| i.wrapping_mul(2_654_435_761))
            .collect();
        let as_i32: Vec<i32> = data.iter().map(|&v| v as i32).collect();
        let encoded = encode_simd_array(&as_i32).expect("encode Vec<u32> 10000 via i32");
        let decoded_i32: Vec<i32> =
            decode_simd_array(&encoded).expect("decode Vec<u32> 10000 via i32");
        let reconstructed: Vec<u32> = decoded_i32.iter().map(|&v| v as u32).collect();
        assert_eq!(data, reconstructed);
    }

    // -----------------------------------------------------------------------
    // 6. SIMD encode of Vec<i32> with 10 000 elements
    // -----------------------------------------------------------------------

    #[test]
    fn test_vec_i32_10000_encode_decode_roundtrip() {
        let data: Vec<i32> = (0i32..10_000).map(|i| i * -3 + 777).collect();
        let encoded = encode_simd_array(&data).expect("encode Vec<i32> 10000");
        let decoded: Vec<i32> = decode_simd_array(&encoded).expect("decode Vec<i32> 10000");
        assert_eq!(data, decoded);
    }

    // -----------------------------------------------------------------------
    // 7. SIMD encode of Vec<u64> with 5 000 elements (via i64 reinterpret)
    // -----------------------------------------------------------------------

    #[test]
    fn test_vec_u64_5000_encode_decode_roundtrip() {
        let data: Vec<u64> = (0u64..5_000)
            .map(|i| i.wrapping_mul(6_364_136_223_846_793_005))
            .collect();
        let as_i64: Vec<i64> = data.iter().map(|&v| v as i64).collect();
        let encoded = encode_simd_array(&as_i64).expect("encode Vec<u64> 5000 via i64");
        let decoded_i64: Vec<i64> =
            decode_simd_array(&encoded).expect("decode Vec<u64> 5000 via i64");
        let reconstructed: Vec<u64> = decoded_i64.iter().map(|&v| v as u64).collect();
        assert_eq!(data, reconstructed);
    }

    // -----------------------------------------------------------------------
    // 8. SIMD encode of Vec<f64> with 5 000 elements
    // -----------------------------------------------------------------------

    #[test]
    fn test_vec_f64_5000_encode_decode_roundtrip() {
        let data: Vec<f64> = (0usize..5_000)
            .map(|i| PI * (i as f64) * 0.001 - E)
            .collect();
        let encoded = encode_simd_array(&data).expect("encode Vec<f64> 5000");
        let decoded: Vec<f64> = decode_simd_array(&encoded).expect("decode Vec<f64> 5000");
        for (idx, (a, b)) in data.iter().zip(decoded.iter()).enumerate() {
            assert_eq!(a.to_bits(), b.to_bits(), "f64 bit mismatch at index {idx}");
        }
    }

    // -----------------------------------------------------------------------
    // 9. SIMD vs standard encode: same bytes for [u32; 64] via oxicode standard
    //    path (the SIMD feature changes internal dispatch; we verify that two
    //    calls on the same data produce identical bytes each time)
    // -----------------------------------------------------------------------

    #[test]
    fn test_simd_encode_deterministic_u32_64_via_i32() {
        let data: [u32; 64] = core::array::from_fn(|i| i as u32 * 7 + 3);
        let as_i32: Vec<i32> = data.iter().map(|&v| v as i32).collect();
        let enc_a = encode_simd_array(&as_i32).expect("encode a");
        let enc_b = encode_simd_array(&as_i32).expect("encode b");
        assert_eq!(
            enc_a, enc_b,
            "two encode calls on identical [u32;64] data must produce identical bytes"
        );
    }

    // -----------------------------------------------------------------------
    // 10. SIMD vs standard encode: same bytes for Vec<u32> (determinism)
    // -----------------------------------------------------------------------

    #[test]
    fn test_simd_encode_deterministic_vec_u32() {
        let data: Vec<u32> = (0u32..512).map(|i| i * 13 + 5).collect();
        let as_i32: Vec<i32> = data.iter().map(|&v| v as i32).collect();
        let enc_a = encode_simd_array(&as_i32).expect("encode Vec<u32> a");
        let enc_b = encode_simd_array(&as_i32).expect("encode Vec<u32> b");
        assert_eq!(enc_a, enc_b, "Vec<u32> SIMD encode must be deterministic");
    }

    // -----------------------------------------------------------------------
    // 11. Encode alignment: AlignedVec<u8> can hold encoded data
    // -----------------------------------------------------------------------

    #[test]
    fn test_aligned_vec_u8_holds_encoded_data() {
        let data: Vec<f32> = (0u32..128).map(|i| i as f32 * PI as f32).collect();
        let encoded = encode_simd_array(&data).expect("encode f32 for alignment test");
        let mut av: AlignedVec<u8> = AlignedVec::new();
        av.extend(encoded.iter().copied());
        assert_eq!(av.len(), encoded.len());
        assert!(av.is_aligned(), "AlignedVec must report SIMD alignment");
        // Verify we can decode directly from the AlignedVec's slice.
        let decoded: Vec<f32> =
            decode_simd_array(av.as_slice()).expect("decode from AlignedVec slice");
        assert_eq!(data.len(), decoded.len());
        for (a, b) in data.iter().zip(decoded.iter()) {
            assert_eq!(a.to_bits(), b.to_bits(), "AlignedVec round-trip mismatch");
        }
    }

    // -----------------------------------------------------------------------
    // 12. SIMD detect: optimal_alignment() returns reasonable value (>= 1)
    // -----------------------------------------------------------------------

    #[test]
    fn test_optimal_alignment_reasonable_value() {
        let align = optimal_alignment();
        assert!(
            align >= 1,
            "optimal_alignment must be at least 1, got {align}"
        );
        assert_eq!(
            align & (align - 1),
            0,
            "optimal_alignment must be a power of two, got {align}"
        );
        // Sanity: typical SIMD alignments are 1, 4, 8, 16, 32, 64.
        assert!(
            align <= 128,
            "optimal_alignment unexpectedly large: {align}"
        );
    }

    // -----------------------------------------------------------------------
    // 13. SIMD encode of struct with array field (via standard encode path,
    //     which uses SIMD internally when feature is active)
    // -----------------------------------------------------------------------

    #[test]
    fn test_struct_with_f32_array_field_roundtrip() {
        // Use oxicode standard encode/decode, which leverages SIMD internally.
        #[derive(oxicode::Encode, oxicode::Decode, PartialEq, Debug)]
        struct Batch {
            id: u32,
            values: Vec<f32>,
        }

        let original = Batch {
            id: 42,
            values: (0u32..64).map(|i| i as f32 * PI as f32).collect(),
        };
        let encoded = oxicode::encode_to_vec(&original).expect("encode struct with array");
        let (decoded, _): (Batch, _) =
            oxicode::decode_from_slice(&encoded).expect("decode struct with array");
        assert_eq!(original, decoded);
    }

    // -----------------------------------------------------------------------
    // 14. Large [u8; 4096] round-trip
    // -----------------------------------------------------------------------

    #[test]
    fn test_u8_4096_roundtrip() {
        let data: Vec<u8> = (0u32..4096)
            .map(|i| i.wrapping_mul(1_664_525).wrapping_add(1_013_904_223) as u8)
            .collect();
        let encoded = encode_simd_array(&data).expect("encode [u8;4096]");
        let decoded: Vec<u8> = decode_simd_array(&encoded).expect("decode [u8;4096]");
        assert_eq!(data, decoded);
    }

    // -----------------------------------------------------------------------
    // 15. Large [u16; 2048] round-trip (via raw LE bytes through u8 path)
    // -----------------------------------------------------------------------

    #[test]
    fn test_u16_2048_roundtrip_via_le_bytes() {
        let data: Vec<u16> = (0u16..2048).collect();
        let raw: Vec<u8> = data.iter().flat_map(|&v| v.to_le_bytes()).collect();
        let encoded = encode_simd_array(&raw).expect("encode [u16;2048] as u8");
        let decoded_raw: Vec<u8> = decode_simd_array(&encoded).expect("decode [u16;2048] as u8");
        assert_eq!(raw, decoded_raw);
        let reconstructed: Vec<u16> = decoded_raw
            .chunks_exact(2)
            .map(|b| u16::from_le_bytes([b[0], b[1]]))
            .collect();
        assert_eq!(data, reconstructed);
    }

    // -----------------------------------------------------------------------
    // 16. SIMD encode then decode partial: first N items
    //     Encode a 512-element f32 array, then decode and verify only the
    //     first 64 elements match (the full decode gives all, we just slice).
    // -----------------------------------------------------------------------

    #[test]
    fn test_simd_encode_decode_partial_first_n_items() {
        const TOTAL: usize = 512;
        const PARTIAL: usize = 64;
        let data: Vec<f32> = (0usize..TOTAL)
            .map(|i| i as f32 * E as f32 * 0.01)
            .collect();
        let encoded = encode_simd_array(&data).expect("encode for partial decode");
        let decoded: Vec<f32> = decode_simd_array(&encoded).expect("decode full");
        assert_eq!(decoded.len(), TOTAL);
        // Check first PARTIAL items match
        for (idx, (a, b)) in data[..PARTIAL]
            .iter()
            .zip(decoded[..PARTIAL].iter())
            .enumerate()
        {
            assert_eq!(
                a.to_bits(),
                b.to_bits(),
                "partial item mismatch at index {idx}"
            );
        }
    }

    // -----------------------------------------------------------------------
    // 17. SIMD array of zeros round-trip (all types)
    // -----------------------------------------------------------------------

    #[test]
    fn test_simd_array_zeros_roundtrip_all_types() {
        // f32 zeros
        let zeros_f32 = vec![0.0f32; 256];
        let enc = encode_simd_array(&zeros_f32).expect("encode f32 zeros");
        let dec: Vec<f32> = decode_simd_array(&enc).expect("decode f32 zeros");
        assert_eq!(zeros_f32, dec);

        // f64 zeros
        let zeros_f64 = vec![0.0f64; 128];
        let enc = encode_simd_array(&zeros_f64).expect("encode f64 zeros");
        let dec: Vec<f64> = decode_simd_array(&enc).expect("decode f64 zeros");
        assert_eq!(zeros_f64, dec);

        // i32 zeros
        let zeros_i32 = vec![0i32; 1024];
        let enc = encode_simd_array(&zeros_i32).expect("encode i32 zeros");
        let dec: Vec<i32> = decode_simd_array(&enc).expect("decode i32 zeros");
        assert_eq!(zeros_i32, dec);

        // i64 zeros
        let zeros_i64 = vec![0i64; 512];
        let enc = encode_simd_array(&zeros_i64).expect("encode i64 zeros");
        let dec: Vec<i64> = decode_simd_array(&enc).expect("decode i64 zeros");
        assert_eq!(zeros_i64, dec);

        // u8 zeros
        let zeros_u8 = vec![0u8; 4096];
        let enc = encode_simd_array(&zeros_u8).expect("encode u8 zeros");
        let dec: Vec<u8> = decode_simd_array(&enc).expect("decode u8 zeros");
        assert_eq!(zeros_u8, dec);
    }

    // -----------------------------------------------------------------------
    // 18. SIMD array with max values round-trip
    // -----------------------------------------------------------------------

    #[test]
    fn test_simd_array_max_values_roundtrip() {
        // f32 max
        let maxf32 = vec![f32::MAX; 128];
        let enc = encode_simd_array(&maxf32).expect("encode f32 MAX");
        let dec: Vec<f32> = decode_simd_array(&enc).expect("decode f32 MAX");
        for (a, b) in maxf32.iter().zip(dec.iter()) {
            assert_eq!(a.to_bits(), b.to_bits(), "f32 MAX mismatch");
        }

        // f64 max
        let maxf64 = vec![f64::MAX; 64];
        let enc = encode_simd_array(&maxf64).expect("encode f64 MAX");
        let dec: Vec<f64> = decode_simd_array(&enc).expect("decode f64 MAX");
        for (a, b) in maxf64.iter().zip(dec.iter()) {
            assert_eq!(a.to_bits(), b.to_bits(), "f64 MAX mismatch");
        }

        // i32 max/min
        let maxi32: Vec<i32> = (0..512)
            .map(|i| if i % 2 == 0 { i32::MAX } else { i32::MIN })
            .collect();
        let enc = encode_simd_array(&maxi32).expect("encode i32 MAX/MIN");
        let dec: Vec<i32> = decode_simd_array(&enc).expect("decode i32 MAX/MIN");
        assert_eq!(maxi32, dec);

        // u8 max (all 0xFF)
        let maxu8 = vec![0xFFu8; 512];
        let enc = encode_simd_array(&maxu8).expect("encode u8 MAX");
        let dec: Vec<u8> = decode_simd_array(&enc).expect("decode u8 MAX");
        assert_eq!(maxu8, dec);
    }

    // -----------------------------------------------------------------------
    // 19. SIMD encode nested Vec<Vec<u32>> via standard path
    // -----------------------------------------------------------------------

    #[test]
    fn test_nested_vec_vec_u32_roundtrip_standard_path() {
        // The `simd` feature affects internal dispatch; Vec<Vec<u32>> uses the
        // standard encode/decode path, which benefits from it automatically.
        let nested: Vec<Vec<u32>> = (0u32..16)
            .map(|row| (0u32..32).map(|col| row * 100 + col).collect())
            .collect();
        let encoded = oxicode::encode_to_vec(&nested).expect("encode nested Vec<Vec<u32>>");
        let (decoded, _): (Vec<Vec<u32>>, _) =
            oxicode::decode_from_slice(&encoded).expect("decode nested Vec<Vec<u32>>");
        assert_eq!(nested, decoded);
    }

    // -----------------------------------------------------------------------
    // 20. Encode performance: SIMD large array completes without panic
    //     (65 536 f32 elements — equivalent to a 256 KB numerical buffer)
    // -----------------------------------------------------------------------

    #[test]
    fn test_simd_large_array_completes_without_panic() {
        let data: Vec<f32> = (0u32..65_536)
            .map(|i| (i as f32) * PI as f32 * 0.000_01)
            .collect();
        let encoded = encode_simd_array(&data).expect("encode 65536 f32 must not fail");
        assert!(!encoded.is_empty(), "encoded output must not be empty");
        let decoded: Vec<f32> =
            decode_simd_array(&encoded).expect("decode 65536 f32 must not fail");
        assert_eq!(decoded.len(), data.len(), "decoded length must match");
        // Spot-check a few values to confirm correctness.
        for idx in [0usize, 1, 255, 1023, 32767, 65535] {
            assert_eq!(
                data[idx].to_bits(),
                decoded[idx].to_bits(),
                "spot-check mismatch at index {idx}"
            );
        }
    }
}
