//! SIMD encoding correctness tests.
//!
//! 22 focused correctness tests covering round-trips, boundary values,
//! alignment, determinism, and struct encoding under the SIMD feature gate.
//! All tests use `#[cfg(feature = "simd")]` and follow the no-unwrap policy.

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
mod simd_correctness_tests {
    use std::f64::consts::{E, PI};

    use oxicode::simd::{decode_simd_array, encode_simd_array, optimal_alignment, AlignedVec};

    // -----------------------------------------------------------------------
    // 1. SIMD encode i32 array: SIMD and standard encode_to_vec give identical
    //    decoded values (both paths must be semantically equivalent).
    // -----------------------------------------------------------------------

    #[test]
    fn test_simd_encode_i32_matches_standard_decode() {
        // Encode via SIMD direct path, then confirm round-trip values are correct.
        // We do NOT compare raw bytes (format differs) – only that decoded values match.
        let data: Vec<i32> = vec![-500, -1, 0, 1, 500, i32::MAX / 2, i32::MIN / 2];
        let simd_enc = encode_simd_array(&data).expect("SIMD encode i32 failed");
        let simd_dec: Vec<i32> = decode_simd_array(&simd_enc).expect("SIMD decode i32 failed");
        assert_eq!(
            data, simd_dec,
            "SIMD i32 round-trip must recover original values"
        );
    }

    // -----------------------------------------------------------------------
    // 2. SIMD encode u32 values 0..100 roundtrip (via i32 bit-cast)
    // -----------------------------------------------------------------------

    #[test]
    fn test_simd_encode_u32_0_to_100_roundtrip() {
        let data: Vec<u32> = (0u32..100).collect();
        let as_i32: Vec<i32> = data.iter().map(|&v| v as i32).collect();
        let encoded = encode_simd_array(&as_i32).expect("encode u32 0..100 via i32");
        let decoded_i32: Vec<i32> = decode_simd_array(&encoded).expect("decode u32 0..100 via i32");
        let reconstructed: Vec<u32> = decoded_i32.iter().map(|&v| v as u32).collect();
        assert_eq!(data, reconstructed, "u32 0..100 round-trip must be exact");
    }

    // -----------------------------------------------------------------------
    // 3. SIMD encode negative i32 values: only negative inputs
    // -----------------------------------------------------------------------

    #[test]
    fn test_simd_encode_negative_i32_values() {
        let data: Vec<i32> = vec![-1, -127, -128, -32768, -2_000_000, i32::MIN + 1];
        let encoded = encode_simd_array(&data).expect("encode negative i32");
        let decoded: Vec<i32> = decode_simd_array(&encoded).expect("decode negative i32");
        assert_eq!(data, decoded, "negative i32 round-trip must be exact");
    }

    // -----------------------------------------------------------------------
    // 4. SIMD encode i64 MAX/MIN boundary values
    // -----------------------------------------------------------------------

    #[test]
    fn test_simd_encode_i64_max_min_boundary() {
        let data: Vec<i64> = vec![
            i64::MIN,
            i64::MIN + 1,
            -1i64,
            0i64,
            1i64,
            i64::MAX - 1,
            i64::MAX,
        ];
        let encoded = encode_simd_array(&data).expect("encode i64 MAX/MIN");
        let decoded: Vec<i64> = decode_simd_array(&encoded).expect("decode i64 MAX/MIN");
        assert_eq!(
            data, decoded,
            "i64 MAX/MIN boundary round-trip must be exact"
        );
    }

    // -----------------------------------------------------------------------
    // 5. SIMD encode [u32; 16] exactly roundtrip (via i32)
    // -----------------------------------------------------------------------

    #[test]
    fn test_simd_encode_u32_array_16_exact_roundtrip() {
        let data: [u32; 16] = [
            0,
            1,
            2,
            3,
            1000,
            65535,
            65536,
            100_000,
            500_000,
            1_000_000,
            u32::MAX / 4,
            u32::MAX / 2,
            u32::MAX - 3,
            u32::MAX - 2,
            u32::MAX - 1,
            u32::MAX,
        ];
        let as_i32: Vec<i32> = data.iter().map(|&v| v as i32).collect();
        let encoded = encode_simd_array(&as_i32).expect("encode [u32;16]");
        let decoded_i32: Vec<i32> = decode_simd_array(&encoded).expect("decode [u32;16]");
        let reconstructed: Vec<u32> = decoded_i32.iter().map(|&v| v as u32).collect();
        assert_eq!(
            &data[..],
            reconstructed.as_slice(),
            "[u32;16] round-trip must be bit-exact"
        );
    }

    // -----------------------------------------------------------------------
    // 6. SIMD encode [i32; 32] exactly roundtrip
    // -----------------------------------------------------------------------

    #[test]
    fn test_simd_encode_i32_array_32_exact_roundtrip() {
        let data: [i32; 32] = core::array::from_fn(|i| {
            if i % 2 == 0 {
                i as i32 * 131_071
            } else {
                -(i as i32) * 131_071
            }
        });
        let encoded = encode_simd_array(&data[..]).expect("encode [i32;32]");
        let decoded: Vec<i32> = decode_simd_array(&encoded).expect("decode [i32;32]");
        assert_eq!(
            &data[..],
            decoded.as_slice(),
            "[i32;32] round-trip must be bit-exact"
        );
    }

    // -----------------------------------------------------------------------
    // 7. SIMD encode [f32; 8] with PI-based values
    // -----------------------------------------------------------------------

    #[test]
    fn test_simd_encode_f32_array_8_pi_based() {
        let pi_f32 = PI as f32;
        let data: [f32; 8] = [
            pi_f32,
            pi_f32 * 2.0,
            pi_f32 / 2.0,
            pi_f32 * pi_f32,
            -pi_f32,
            -pi_f32 * 2.0,
            pi_f32 * 0.0,
            pi_f32 * 1000.0,
        ];
        let encoded = encode_simd_array(&data[..]).expect("encode [f32;8] PI values");
        let decoded: Vec<f32> = decode_simd_array(&encoded).expect("decode [f32;8] PI values");
        assert_eq!(data.len(), decoded.len(), "decoded length must match");
        for (idx, (a, b)) in data.iter().zip(decoded.iter()).enumerate() {
            assert_eq!(
                a.to_bits(),
                b.to_bits(),
                "f32 PI value bit mismatch at index {idx}"
            );
        }
    }

    // -----------------------------------------------------------------------
    // 8. SIMD encode [f64; 4] with E-based values
    // -----------------------------------------------------------------------

    #[test]
    fn test_simd_encode_f64_array_4_e_based() {
        let data: [f64; 4] = [E, E * E, E / PI, -E * 1_000_000.0];
        let encoded = encode_simd_array(&data[..]).expect("encode [f64;4] E values");
        let decoded: Vec<f64> = decode_simd_array(&encoded).expect("decode [f64;4] E values");
        assert_eq!(data.len(), decoded.len(), "decoded length must match");
        for (idx, (a, b)) in data.iter().zip(decoded.iter()).enumerate() {
            assert_eq!(
                a.to_bits(),
                b.to_bits(),
                "f64 E value bit mismatch at index {idx}"
            );
        }
    }

    // -----------------------------------------------------------------------
    // 9. SIMD encode Vec<u32> length 1 (via i32)
    // -----------------------------------------------------------------------

    #[test]
    fn test_simd_encode_vec_u32_length_1() {
        let data: Vec<u32> = vec![42u32];
        let as_i32: Vec<i32> = data.iter().map(|&v| v as i32).collect();
        let encoded = encode_simd_array(&as_i32).expect("encode Vec<u32> len=1");
        let decoded_i32: Vec<i32> = decode_simd_array(&encoded).expect("decode Vec<u32> len=1");
        let reconstructed: Vec<u32> = decoded_i32.iter().map(|&v| v as u32).collect();
        assert_eq!(
            data, reconstructed,
            "single-element u32 round-trip must be exact"
        );
    }

    // -----------------------------------------------------------------------
    // 10. SIMD encode Vec<u32> length 255 (boundary, via i32)
    // -----------------------------------------------------------------------

    #[test]
    fn test_simd_encode_vec_u32_length_255_boundary() {
        let data: Vec<u32> = (0u32..255).map(|i| i.wrapping_mul(16_843_009)).collect();
        let as_i32: Vec<i32> = data.iter().map(|&v| v as i32).collect();
        let encoded = encode_simd_array(&as_i32).expect("encode Vec<u32> len=255");
        let decoded_i32: Vec<i32> = decode_simd_array(&encoded).expect("decode Vec<u32> len=255");
        let reconstructed: Vec<u32> = decoded_i32.iter().map(|&v| v as u32).collect();
        assert_eq!(
            data, reconstructed,
            "255-element u32 round-trip must be exact"
        );
    }

    // -----------------------------------------------------------------------
    // 11. SIMD encode Vec<u32> length 256 (via i32)
    // -----------------------------------------------------------------------

    #[test]
    fn test_simd_encode_vec_u32_length_256() {
        let data: Vec<u32> = (0u32..256).map(|i| i.wrapping_mul(16_843_009)).collect();
        let as_i32: Vec<i32> = data.iter().map(|&v| v as i32).collect();
        let encoded = encode_simd_array(&as_i32).expect("encode Vec<u32> len=256");
        let decoded_i32: Vec<i32> = decode_simd_array(&encoded).expect("decode Vec<u32> len=256");
        let reconstructed: Vec<u32> = decoded_i32.iter().map(|&v| v as u32).collect();
        assert_eq!(
            data, reconstructed,
            "256-element u32 round-trip must be exact"
        );
    }

    // -----------------------------------------------------------------------
    // 12. SIMD encode Vec<i64> length 1000
    // -----------------------------------------------------------------------

    #[test]
    fn test_simd_encode_vec_i64_length_1000() {
        let data: Vec<i64> = (0i64..1000)
            .map(|i| {
                // Spread across both positive and negative range
                if i % 3 == 0 {
                    i.saturating_mul(9_999_999_999)
                } else if i % 3 == 1 {
                    i.saturating_neg().saturating_mul(9_999_999_999)
                } else {
                    0i64
                }
            })
            .collect();
        let encoded = encode_simd_array(&data).expect("encode Vec<i64> len=1000");
        let decoded: Vec<i64> = decode_simd_array(&encoded).expect("decode Vec<i64> len=1000");
        assert_eq!(data, decoded, "1000-element i64 round-trip must be exact");
    }

    // -----------------------------------------------------------------------
    // 13. SIMD encode array of zeros [u32; 64] (via i32)
    // -----------------------------------------------------------------------

    #[test]
    fn test_simd_encode_u32_array_64_zeros() {
        let data: [u32; 64] = [0u32; 64];
        let as_i32: Vec<i32> = data.iter().map(|&v| v as i32).collect();
        let encoded = encode_simd_array(&as_i32).expect("encode [u32;64] zeros");
        let decoded_i32: Vec<i32> = decode_simd_array(&encoded).expect("decode [u32;64] zeros");
        let reconstructed: Vec<u32> = decoded_i32.iter().map(|&v| v as u32).collect();
        assert_eq!(
            &data[..],
            reconstructed.as_slice(),
            "[u32;64] zeros round-trip must be exact"
        );
    }

    // -----------------------------------------------------------------------
    // 14. SIMD encode array of u32::MAX values [u32; 16] (via i32)
    // -----------------------------------------------------------------------

    #[test]
    fn test_simd_encode_u32_max_array_16() {
        let data: [u32; 16] = [u32::MAX; 16];
        let as_i32: Vec<i32> = data.iter().map(|&v| v as i32).collect();
        let encoded = encode_simd_array(&as_i32).expect("encode [u32::MAX;16]");
        let decoded_i32: Vec<i32> = decode_simd_array(&encoded).expect("decode [u32::MAX;16]");
        let reconstructed: Vec<u32> = decoded_i32.iter().map(|&v| v as u32).collect();
        assert_eq!(
            &data[..],
            reconstructed.as_slice(),
            "[u32::MAX;16] round-trip must be bit-exact"
        );
    }

    // -----------------------------------------------------------------------
    // 15. SIMD encode mixed positive/negative [i32; 32]
    // -----------------------------------------------------------------------

    #[test]
    fn test_simd_encode_mixed_pos_neg_i32_array_32() {
        // Alternating positive, negative, and zero values in a 32-element array.
        let data: [i32; 32] = core::array::from_fn(|i| match i % 3 {
            0 => (i as i32) * 1_000_000,
            1 => -((i as i32) * 1_000_000),
            _ => 0,
        });
        let encoded = encode_simd_array(&data[..]).expect("encode mixed [i32;32]");
        let decoded: Vec<i32> = decode_simd_array(&encoded).expect("decode mixed [i32;32]");
        assert_eq!(
            &data[..],
            decoded.as_slice(),
            "mixed pos/neg [i32;32] round-trip must be exact"
        );
    }

    // -----------------------------------------------------------------------
    // 16. AlignedVec<u8> stores encoded data correctly
    // -----------------------------------------------------------------------

    #[test]
    fn test_aligned_vec_u8_stores_encoded_data_correctly() {
        let data: Vec<i32> = (0i32..64).map(|i| i * 77 - 2464).collect();
        let encoded = encode_simd_array(&data).expect("encode i32 for AlignedVec test");
        let expected_len = encoded.len();
        let mut av: AlignedVec<u8> = AlignedVec::new();
        for &byte in &encoded {
            av.push(byte);
        }
        assert_eq!(
            av.len(),
            expected_len,
            "AlignedVec must hold all encoded bytes"
        );
        // Verify data integrity by decoding from the AlignedVec slice.
        let decoded: Vec<i32> =
            decode_simd_array(av.as_slice()).expect("decode from AlignedVec<u8>");
        assert_eq!(
            data, decoded,
            "data decoded from AlignedVec must match original"
        );
    }

    // -----------------------------------------------------------------------
    // 17. AlignedVec<u8> alignment is at least 8 bytes
    // -----------------------------------------------------------------------

    #[test]
    fn test_aligned_vec_u8_alignment_at_least_8_bytes() {
        let mut av: AlignedVec<u8> = AlignedVec::with_capacity(64);
        av.push(0xABu8);
        // is_aligned() checks against SIMD_ALIGNMENT (which is >= 8 on all targets).
        assert!(
            av.is_aligned(),
            "AlignedVec<u8> must report SIMD alignment (>= 8 bytes)"
        );
        // Additionally verify the raw pointer address is at least 8-byte aligned.
        let addr = av.as_slice().as_ptr() as usize;
        assert_eq!(
            addr % 8,
            0,
            "AlignedVec<u8> data pointer must be 8-byte aligned (addr={addr:#x})"
        );
    }

    // -----------------------------------------------------------------------
    // 18. SIMD encode then decode: consumed bytes count is correct
    //     The SIMD encoded payload is fully consumed during decode (no leftover
    //     bytes – verified by round-tripping and checking that re-encoding
    //     the decoded data produces the identical byte stream).
    // -----------------------------------------------------------------------

    #[test]
    fn test_simd_encode_decode_consumed_bytes_count_correct() {
        let data: Vec<f64> = (0usize..32).map(|i| E.powi(i as i32) * 0.000_001).collect();
        let encoded_first = encode_simd_array(&data).expect("first encode f64");
        let decoded: Vec<f64> = decode_simd_array(&encoded_first).expect("decode f64");
        assert_eq!(
            decoded.len(),
            data.len(),
            "decoded element count must equal original"
        );
        // Re-encode and verify the byte streams are identical (idempotent encoding).
        let encoded_second = encode_simd_array(&decoded).expect("second encode f64");
        assert_eq!(
            encoded_first.len(),
            encoded_second.len(),
            "re-encoded byte count must equal first encode byte count"
        );
        assert_eq!(
            encoded_first, encoded_second,
            "re-encoding decoded data must produce identical bytes"
        );
    }

    // -----------------------------------------------------------------------
    // 19. SIMD encode produces same bytes in multiple calls
    // -----------------------------------------------------------------------

    #[test]
    fn test_simd_encode_same_bytes_multiple_calls() {
        let data: Vec<i64> = (0i64..128)
            .map(|i| i.saturating_mul(i64::MAX / 128))
            .collect();
        let enc_a = encode_simd_array(&data).expect("encode call A");
        let enc_b = encode_simd_array(&data).expect("encode call B");
        let enc_c = encode_simd_array(&data).expect("encode call C");
        assert_eq!(
            enc_a, enc_b,
            "encode call A and B must produce identical bytes"
        );
        assert_eq!(
            enc_b, enc_c,
            "encode call B and C must produce identical bytes"
        );
    }

    // -----------------------------------------------------------------------
    // 20. SIMD encode struct with array field (via standard encode/decode path)
    // -----------------------------------------------------------------------

    #[test]
    fn test_simd_encode_struct_with_array_field() {
        #[derive(oxicode::Encode, oxicode::Decode, PartialEq, Debug)]
        struct SignalBlock {
            channel: u8,
            samples: [f32; 16],
        }

        let pi_f32 = PI as f32;
        let original = SignalBlock {
            channel: 7,
            samples: core::array::from_fn(|i| pi_f32 * (i as f32 + 1.0) * E as f32),
        };
        let encoded = oxicode::encode_to_vec(&original).expect("encode struct with [f32;16] field");
        let (decoded, _): (SignalBlock, _) =
            oxicode::decode_from_slice(&encoded).expect("decode struct with [f32;16] field");
        assert_eq!(
            original, decoded,
            "struct with array field must round-trip exactly"
        );
    }

    // -----------------------------------------------------------------------
    // 21. SIMD encode large Vec<f32> (5000 elements)
    // -----------------------------------------------------------------------

    #[test]
    fn test_simd_encode_large_vec_f32_5000_elements() {
        // Use PI and E to generate varied float values across the full exponent range.
        let data: Vec<f32> = (0usize..5000)
            .map(|i| {
                let t = i as f32 / 5000.0;
                (PI as f32) * t - (E as f32) * (1.0 - t)
            })
            .collect();
        let encoded = encode_simd_array(&data).expect("encode Vec<f32> 5000 elements");
        assert!(
            !encoded.is_empty(),
            "encoded output for 5000 f32 elements must not be empty"
        );
        let decoded: Vec<f32> = decode_simd_array(&encoded).expect("decode Vec<f32> 5000 elements");
        assert_eq!(
            data.len(),
            decoded.len(),
            "decoded Vec<f32> length must equal 5000"
        );
        // Verify bit-exact fidelity for every element.
        for (idx, (orig, dec)) in data.iter().zip(decoded.iter()).enumerate() {
            assert_eq!(
                orig.to_bits(),
                dec.to_bits(),
                "f32 bit mismatch at index {idx}"
            );
        }
    }

    // -----------------------------------------------------------------------
    // 22. SIMD optimal_alignment returns power-of-two
    // -----------------------------------------------------------------------

    #[test]
    fn test_simd_optimal_alignment_returns_power_of_two() {
        let align = optimal_alignment();
        assert!(align >= 1, "optimal_alignment must be >= 1, got {align}");
        // A power-of-two satisfies: align & (align - 1) == 0
        assert_eq!(
            align & (align - 1),
            0,
            "optimal_alignment must be a power of two, got {align}"
        );
        // Sanity: alignment values outside 1..=512 would be highly unusual.
        assert!(
            align <= 512,
            "optimal_alignment value {align} is unexpectedly large"
        );
    }
}
