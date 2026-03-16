//! SIMD array encoding tests.
//!
//! Tests exercise the public SIMD API through both the direct `oxicode::simd`
//! module functions and the standard `encode_to_vec` / `decode_from_slice`
//! round-trip path (which uses the standard Encode/Decode impls for arrays –
//! the SIMD feature simply changes the *internal* dispatch).

// ---------------------------------------------------------------------------
// Tests that use the simd:: API directly
// ---------------------------------------------------------------------------

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
mod simd_direct {
    use oxicode::simd::{
        decode_simd_array, detect_capability, encode_simd_array, is_simd_available,
        optimal_alignment, AlignedBuffer, AlignedVec, SimdCapability, SIMD_ALIGNMENT,
    };

    // -----------------------------------------------------------------------
    // SimdCapability API
    // -----------------------------------------------------------------------

    #[test]
    fn test_simd_capability_detection_returns_valid_variant() {
        let cap = detect_capability();
        assert!(matches!(
            cap,
            SimdCapability::Avx512
                | SimdCapability::Avx2
                | SimdCapability::Sse42
                | SimdCapability::Neon
                | SimdCapability::Scalar
        ));
    }

    #[test]
    fn test_simd_capability_is_cached() {
        let cap1 = detect_capability();
        let cap2 = detect_capability();
        assert_eq!(cap1, cap2, "detect_capability must be deterministic/cached");
    }

    #[test]
    fn test_is_simd_available_consistent_with_capability() {
        let cap = detect_capability();
        assert_eq!(
            is_simd_available(),
            cap.is_simd(),
            "is_simd_available must match capability.is_simd()"
        );
    }

    #[test]
    fn test_optimal_alignment_positive() {
        let align = optimal_alignment();
        assert!(align >= 1, "alignment must be at least 1");
        // Must be a power of two
        assert_eq!(align & (align - 1), 0, "alignment must be a power of two");
    }

    #[test]
    fn test_simd_capability_ordering() {
        assert!(SimdCapability::Scalar < SimdCapability::Sse42);
        assert!(SimdCapability::Sse42 < SimdCapability::Avx2);
        assert!(SimdCapability::Avx2 < SimdCapability::Avx512);
    }

    #[test]
    fn test_simd_capability_vector_widths() {
        assert_eq!(SimdCapability::Scalar.vector_width(), 1);
        assert_eq!(SimdCapability::Sse42.vector_width(), 16);
        assert_eq!(SimdCapability::Avx2.vector_width(), 32);
        assert_eq!(SimdCapability::Avx512.vector_width(), 64);
        assert_eq!(SimdCapability::Neon.vector_width(), 16);
    }

    #[test]
    fn test_simd_capability_lanes() {
        assert_eq!(SimdCapability::Avx2.f32_lanes(), 8);
        assert_eq!(SimdCapability::Avx2.f64_lanes(), 4);
        assert_eq!(SimdCapability::Avx2.i32_lanes(), 8);
        assert_eq!(SimdCapability::Sse42.f32_lanes(), 4);
        assert_eq!(SimdCapability::Neon.f32_lanes(), 4);
    }

    #[test]
    fn test_simd_capability_name_non_empty() {
        for cap in [
            SimdCapability::Scalar,
            SimdCapability::Sse42,
            SimdCapability::Avx2,
            SimdCapability::Avx512,
            SimdCapability::Neon,
        ] {
            assert!(!cap.name().is_empty(), "capability name must not be empty");
        }
    }

    #[test]
    fn test_scalar_is_not_simd() {
        assert!(!SimdCapability::Scalar.is_simd());
    }

    #[test]
    fn test_all_non_scalar_are_simd() {
        for cap in [
            SimdCapability::Sse42,
            SimdCapability::Avx2,
            SimdCapability::Avx512,
            SimdCapability::Neon,
        ] {
            assert!(cap.is_simd(), "{:?} should be SIMD", cap);
        }
    }

    // -----------------------------------------------------------------------
    // f32 round-trips
    // -----------------------------------------------------------------------

    #[test]
    fn test_f32_roundtrip_small() {
        let data: Vec<f32> = vec![1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0];
        let encoded = encode_simd_array(&data).expect("encode f32 failed");
        let decoded: Vec<f32> = decode_simd_array(&encoded).expect("decode f32 failed");
        assert_eq!(data, decoded);
    }

    #[test]
    fn test_f32_roundtrip_non_multiple_of_eight() {
        // 13 elements: hits both the chunked path and the remainder path
        let data: Vec<f32> = (0..13).map(|i| i as f32 * 0.5).collect();
        let encoded = encode_simd_array(&data).expect("encode f32 failed");
        let decoded: Vec<f32> = decode_simd_array(&encoded).expect("decode f32 failed");
        assert_eq!(data, decoded);
    }

    #[test]
    fn test_f32_roundtrip_large() {
        // 10 000 elements – exercises the SIMD inner loop extensively
        let data: Vec<f32> = (0..10_000).map(|i| i as f32 * 0.001).collect();
        let encoded = encode_simd_array(&data).expect("encode f32 failed");
        let decoded: Vec<f32> = decode_simd_array(&encoded).expect("decode f32 failed");
        assert_eq!(data, decoded);
    }

    #[test]
    fn test_f32_roundtrip_special_values() {
        let data: Vec<f32> = vec![
            f32::MIN,
            f32::MAX,
            f32::NEG_INFINITY,
            f32::INFINITY,
            0.0,
            -0.0,
            1.0,
            -1.0,
        ];
        let encoded = encode_simd_array(&data).expect("encode f32 special failed");
        let decoded: Vec<f32> = decode_simd_array(&encoded).expect("decode f32 special failed");
        // Compare bit-by-bit to preserve NaN / signed-zero semantics
        for (a, b) in data.iter().zip(decoded.iter()) {
            assert_eq!(a.to_bits(), b.to_bits(), "f32 special value mismatch");
        }
    }

    #[test]
    fn test_f32_empty() {
        let data: Vec<f32> = vec![];
        let encoded = encode_simd_array(&data).expect("encode empty f32 failed");
        let decoded: Vec<f32> = decode_simd_array(&encoded).expect("decode empty f32 failed");
        assert_eq!(data, decoded);
    }

    // -----------------------------------------------------------------------
    // f64 round-trips
    // -----------------------------------------------------------------------

    #[test]
    fn test_f64_roundtrip_small() {
        let data: Vec<f64> = vec![1.0, 2.0, 3.0, 4.0, 5.0];
        let encoded = encode_simd_array(&data).expect("encode f64 failed");
        let decoded: Vec<f64> = decode_simd_array(&encoded).expect("decode f64 failed");
        assert_eq!(data, decoded);
    }

    #[test]
    fn test_f64_roundtrip_large() {
        let data: Vec<f64> = (0..2048).map(|i| i as f64 * 0.5).collect();
        let encoded = encode_simd_array(&data).expect("encode f64 large failed");
        let decoded: Vec<f64> = decode_simd_array(&encoded).expect("decode f64 large failed");
        assert_eq!(data, decoded);
    }

    #[test]
    fn test_f64_roundtrip_non_multiple_of_four() {
        // 7 elements: chunks=1, remainder=3
        let data: Vec<f64> = (0..7).map(|i| i as f64 * 1.23).collect();
        let encoded = encode_simd_array(&data).expect("encode f64 failed");
        let decoded: Vec<f64> = decode_simd_array(&encoded).expect("decode f64 failed");
        assert_eq!(data, decoded);
    }

    #[test]
    fn test_f64_roundtrip_special_values() {
        let data: Vec<f64> = vec![
            f64::MIN,
            f64::MAX,
            f64::NEG_INFINITY,
            f64::INFINITY,
            0.0,
            -0.0,
        ];
        let encoded = encode_simd_array(&data).expect("encode f64 special failed");
        let decoded: Vec<f64> = decode_simd_array(&encoded).expect("decode f64 special failed");
        for (a, b) in data.iter().zip(decoded.iter()) {
            assert_eq!(a.to_bits(), b.to_bits(), "f64 special value mismatch");
        }
    }

    // -----------------------------------------------------------------------
    // i32 round-trips
    // -----------------------------------------------------------------------

    #[test]
    fn test_i32_roundtrip_small() {
        let data: Vec<i32> = vec![-100, -1, 0, 1, 100, 1000, -1000, 42];
        let encoded = encode_simd_array(&data).expect("encode i32 failed");
        let decoded: Vec<i32> = decode_simd_array(&encoded).expect("decode i32 failed");
        assert_eq!(data, decoded);
    }

    #[test]
    fn test_i32_roundtrip_boundary_values() {
        let data: Vec<i32> = vec![i32::MIN, i32::MAX, 0, -1, 1, i32::MIN + 1, i32::MAX - 1];
        let encoded = encode_simd_array(&data).expect("encode i32 boundary failed");
        let decoded: Vec<i32> = decode_simd_array(&encoded).expect("decode i32 boundary failed");
        assert_eq!(data, decoded);
    }

    #[test]
    fn test_i32_roundtrip_large() {
        let data: Vec<i32> = (0..4096).map(|i| i - 2048).collect();
        let encoded = encode_simd_array(&data).expect("encode i32 large failed");
        let decoded: Vec<i32> = decode_simd_array(&encoded).expect("decode i32 large failed");
        assert_eq!(data, decoded);
    }

    #[test]
    fn test_i32_roundtrip_non_multiple_of_eight() {
        // 11 elements: chunks=1, remainder=3
        let data: Vec<i32> = (0..11).map(|i| i * -7).collect();
        let encoded = encode_simd_array(&data).expect("encode i32 failed");
        let decoded: Vec<i32> = decode_simd_array(&encoded).expect("decode i32 failed");
        assert_eq!(data, decoded);
    }

    // -----------------------------------------------------------------------
    // i64 round-trips
    // -----------------------------------------------------------------------

    #[test]
    fn test_i64_roundtrip_small() {
        let data: Vec<i64> = vec![-1000, 0, 1000, i64::MAX / 2, i64::MIN / 2];
        let encoded = encode_simd_array(&data).expect("encode i64 failed");
        let decoded: Vec<i64> = decode_simd_array(&encoded).expect("decode i64 failed");
        assert_eq!(data, decoded);
    }

    #[test]
    fn test_i64_roundtrip_boundary_values() {
        let data: Vec<i64> = vec![i64::MIN, i64::MAX, 0, -1, 1];
        let encoded = encode_simd_array(&data).expect("encode i64 boundary failed");
        let decoded: Vec<i64> = decode_simd_array(&encoded).expect("decode i64 boundary failed");
        assert_eq!(data, decoded);
    }

    #[test]
    fn test_i64_roundtrip_large() {
        let data: Vec<i64> = (0..1024i64)
            .map(|i| i.saturating_mul(i64::MAX / 1024))
            .collect();
        let encoded = encode_simd_array(&data).expect("encode i64 large failed");
        let decoded: Vec<i64> = decode_simd_array(&encoded).expect("decode i64 large failed");
        assert_eq!(data, decoded);
    }

    #[test]
    fn test_i64_roundtrip_non_multiple_of_four() {
        // 9 elements: chunks=2, remainder=1
        let data: Vec<i64> = (0..9).map(|i| i as i64 * 1_000_000_000).collect();
        let encoded = encode_simd_array(&data).expect("encode i64 failed");
        let decoded: Vec<i64> = decode_simd_array(&encoded).expect("decode i64 failed");
        assert_eq!(data, decoded);
    }

    // -----------------------------------------------------------------------
    // u8 round-trips
    // -----------------------------------------------------------------------

    #[test]
    fn test_u8_roundtrip_all_values() {
        let data: Vec<u8> = (0u8..=255).collect();
        let encoded = encode_simd_array(&data).expect("encode u8 failed");
        let decoded: Vec<u8> = decode_simd_array(&encoded).expect("decode u8 failed");
        assert_eq!(data, decoded);
    }

    #[test]
    fn test_u8_roundtrip_large_repeated() {
        // 1 MB of bytes
        let data: Vec<u8> = (0u32..1_000_000).map(|i| (i & 0xFF) as u8).collect();
        let encoded = encode_simd_array(&data).expect("encode u8 large failed");
        let decoded: Vec<u8> = decode_simd_array(&encoded).expect("decode u8 large failed");
        assert_eq!(data, decoded);
    }

    #[test]
    fn test_u8_roundtrip_empty() {
        let data: Vec<u8> = vec![];
        let encoded = encode_simd_array(&data).expect("encode u8 empty failed");
        let decoded: Vec<u8> = decode_simd_array(&encoded).expect("decode u8 empty failed");
        assert_eq!(data, decoded);
    }

    // -----------------------------------------------------------------------
    // encode_simd_into / decode_simd_into fixed-buffer variants
    // -----------------------------------------------------------------------

    #[test]
    fn test_f32_encode_into_fixed_buffer() {
        use oxicode::simd::{SimdDecodable, SimdEncodable};
        let data = [1.0f32, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0];
        // header (8 bytes) + 8 * 4 bytes = 40
        let mut buf = [0u8; 40];
        let written = f32::encode_simd_into(&data, &mut buf).expect("encode_into failed");
        assert_eq!(written, 40);

        let mut out = [0.0f32; 8];
        let count = f32::decode_simd_into(&buf[..written], &mut out).expect("decode_into failed");
        assert_eq!(count, 8);
        assert_eq!(out, data);
    }

    #[test]
    fn test_i32_encode_into_fixed_buffer() {
        use oxicode::simd::{SimdDecodable, SimdEncodable};
        let data = [-4i32, -3, -2, -1, 0, 1, 2, 3];
        // header (8 bytes) + 8 * 4 bytes = 40
        let mut buf = [0u8; 40];
        let written = i32::encode_simd_into(&data, &mut buf).expect("encode_into failed");
        assert_eq!(written, 40);

        let mut out = [0i32; 8];
        let count = i32::decode_simd_into(&buf[..written], &mut out).expect("decode_into failed");
        assert_eq!(count, 8);
        assert_eq!(out, data);
    }

    #[test]
    fn test_i64_encode_into_fixed_buffer() {
        use oxicode::simd::{SimdDecodable, SimdEncodable};
        let data = [i64::MIN, -1i64, 0, i64::MAX];
        // header (8 bytes) + 4 * 8 bytes = 40
        let mut buf = [0u8; 40];
        let written = i64::encode_simd_into(&data, &mut buf).expect("encode_into failed");
        assert_eq!(written, 40);

        let mut out = [0i64; 4];
        let count = i64::decode_simd_into(&buf[..written], &mut out).expect("decode_into failed");
        assert_eq!(count, 4);
        assert_eq!(out, data);
    }

    #[test]
    fn test_f64_encode_into_fixed_buffer() {
        use oxicode::simd::{SimdDecodable, SimdEncodable};
        let data = [f64::NEG_INFINITY, -1.0f64, 0.0, f64::INFINITY];
        // header (8 bytes) + 4 * 8 bytes = 40
        let mut buf = [0u8; 40];
        let written = f64::encode_simd_into(&data, &mut buf).expect("encode_into failed");
        assert_eq!(written, 40);

        let mut out = [0.0f64; 4];
        let count = f64::decode_simd_into(&buf[..written], &mut out).expect("decode_into failed");
        assert_eq!(count, 4);
        for (a, b) in data.iter().zip(out.iter()) {
            assert_eq!(a.to_bits(), b.to_bits());
        }
    }

    #[test]
    fn test_u8_encode_into_fixed_buffer() {
        use oxicode::simd::{SimdDecodable, SimdEncodable};
        let data = [0u8, 1, 127, 128, 255];
        // header (8 bytes) + 5 bytes = 13
        let mut buf = [0u8; 13];
        let written = u8::encode_simd_into(&data, &mut buf).expect("encode_into failed");
        assert_eq!(written, 13);

        let mut out = [0u8; 5];
        let count = u8::decode_simd_into(&buf[..written], &mut out).expect("decode_into failed");
        assert_eq!(count, 5);
        assert_eq!(out, data);
    }

    #[test]
    fn test_encode_into_buffer_too_small_returns_error() {
        use oxicode::simd::SimdEncodable;
        let data = [1.0f32; 8];
        let mut tiny = [0u8; 1]; // far too small
        let result = f32::encode_simd_into(&data, &mut tiny);
        assert!(result.is_err(), "should fail on undersized buffer");
    }

    #[test]
    fn test_decode_into_dst_too_small_returns_error() {
        use oxicode::simd::SimdDecodable;
        let data = [1.0f32; 8];
        let encoded = encode_simd_array(&data).expect("encode failed");
        let mut dst = [0.0f32; 2]; // too small for 8 elements
        let result = f32::decode_simd_into(&encoded, &mut dst);
        assert!(result.is_err(), "should fail when dst is too small");
    }

    // -----------------------------------------------------------------------
    // AlignedVec tests
    // -----------------------------------------------------------------------

    #[test]
    fn test_aligned_vec_new_is_empty() {
        let v: AlignedVec<f32> = AlignedVec::new();
        assert_eq!(v.len(), 0);
        assert_eq!(v.capacity(), 0);
        assert!(v.is_empty());
    }

    #[test]
    fn test_aligned_vec_with_capacity_is_aligned() {
        let v: AlignedVec<f64> = AlignedVec::with_capacity(256);
        assert!(v.is_aligned(), "AlignedVec must be SIMD-aligned");
        assert!(v.capacity() >= 256);
    }

    #[test]
    fn test_aligned_vec_push_and_deref() {
        let mut v: AlignedVec<i32> = AlignedVec::new();
        for i in 0..64i32 {
            v.push(i);
        }
        assert_eq!(v.len(), 64);
        assert!(v.is_aligned());
        for (idx, &val) in v.iter().enumerate() {
            assert_eq!(val, idx as i32);
        }
    }

    #[test]
    fn test_aligned_vec_from_slice() {
        let src = [1.0f32, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0];
        let v = AlignedVec::from_slice(&src);
        assert_eq!(v.as_slice(), &src);
        assert!(v.is_aligned());
    }

    #[test]
    fn test_aligned_vec_pop() {
        let mut v: AlignedVec<u8> = AlignedVec::new();
        v.push(10);
        v.push(20);
        assert_eq!(v.pop(), Some(20));
        assert_eq!(v.pop(), Some(10));
        assert_eq!(v.pop(), None);
    }

    #[test]
    fn test_aligned_vec_clear() {
        let mut v: AlignedVec<u32> = AlignedVec::new();
        for i in 0..16u32 {
            v.push(i);
        }
        v.clear();
        assert_eq!(v.len(), 0);
        assert!(v.is_empty());
    }

    #[test]
    fn test_aligned_vec_resize_grow() {
        let mut v: AlignedVec<u32> = AlignedVec::new();
        v.resize(8, 42u32);
        assert_eq!(v.len(), 8);
        for &val in v.iter() {
            assert_eq!(val, 42);
        }
    }

    #[test]
    fn test_aligned_vec_resize_shrink() {
        let mut v: AlignedVec<u32> = AlignedVec::new();
        for i in 0..16u32 {
            v.push(i);
        }
        v.resize(4, 0);
        assert_eq!(v.len(), 4);
        assert_eq!(v.as_slice(), &[0, 1, 2, 3]);
    }

    #[test]
    fn test_aligned_vec_clone() {
        let mut v: AlignedVec<f32> = AlignedVec::new();
        for i in 0..8 {
            v.push(i as f32);
        }
        let cloned = v.clone();
        assert_eq!(v.as_slice(), cloned.as_slice());
        assert!(cloned.is_aligned());
    }

    #[test]
    fn test_aligned_vec_extend() {
        let mut v: AlignedVec<i64> = AlignedVec::new();
        v.extend([1i64, 2, 3, 4]);
        assert_eq!(v.as_slice(), &[1, 2, 3, 4]);
    }

    #[test]
    fn test_aligned_vec_reserve() {
        let mut v: AlignedVec<u8> = AlignedVec::new();
        v.reserve(100);
        assert!(v.capacity() >= 100);
        assert_eq!(v.len(), 0);
    }

    // -----------------------------------------------------------------------
    // AlignedBuffer (stack-allocated) tests
    // -----------------------------------------------------------------------

    #[test]
    fn test_aligned_buffer_alignment() {
        let buf: AlignedBuffer<f32, 16> = AlignedBuffer::new();
        let addr = buf.as_ptr() as usize;
        assert_eq!(
            addr % SIMD_ALIGNMENT,
            0,
            "AlignedBuffer must be {SIMD_ALIGNMENT}-byte aligned"
        );
    }

    #[test]
    fn test_aligned_buffer_push_pop() {
        let mut buf: AlignedBuffer<u32, 8> = AlignedBuffer::new();
        for i in 0..8u32 {
            buf.push(i).expect("push should succeed");
        }
        assert!(buf.is_full());
        assert!(buf.push(99).is_err(), "push to full buffer should fail");

        // Pop in reverse order
        for i in (0..8u32).rev() {
            assert_eq!(buf.pop(), Some(i));
        }
        assert!(buf.is_empty());
    }

    #[test]
    fn test_aligned_buffer_clear() {
        let mut buf: AlignedBuffer<i32, 16> = AlignedBuffer::new();
        for i in 0..10i32 {
            buf.push(i).expect("push should succeed");
        }
        buf.clear();
        assert_eq!(buf.len(), 0);
        assert!(buf.is_empty());
    }

    #[test]
    fn test_aligned_buffer_capacity() {
        let buf: AlignedBuffer<u64, 32> = AlignedBuffer::new();
        assert_eq!(buf.capacity(), 32);
    }

    #[test]
    fn test_aligned_buffer_deref_as_slice() {
        let mut buf: AlignedBuffer<f64, 4> = AlignedBuffer::new();
        buf.push(1.0).expect("push 1.0");
        buf.push(2.0).expect("push 2.0");
        let slice: &[f64] = &buf;
        assert_eq!(slice, &[1.0, 2.0]);
    }

    #[test]
    fn test_aligned_buffer_clone() {
        let mut buf: AlignedBuffer<u8, 8> = AlignedBuffer::new();
        for i in 0..5u8 {
            buf.push(i).expect("push should succeed");
        }
        let cloned = buf.clone();
        assert_eq!(buf.as_slice(), cloned.as_slice());
    }
}

// ---------------------------------------------------------------------------
// Tests via standard oxicode encode_to_vec / decode_from_slice
// (these work regardless of whether the `simd` feature is enabled, but
//  they exercise the same array code paths; with `simd` the dispatch
//  inside the SIMD module is used internally for the aligned types above)
// ---------------------------------------------------------------------------

/// Fixed-size arrays round-trip through the standard encode/decode path.
/// This always runs (no feature gate needed).
#[test]
fn test_array_u32_roundtrip_no_simd_gate() {
    let arr: [u32; 4] = [10, 20, 30, 40];
    let enc = oxicode::encode_to_vec(&arr).expect("encode");
    let (dec, _): ([u32; 4], _) = oxicode::decode_from_slice(&enc).expect("decode");
    assert_eq!(arr, dec);
}

#[test]
fn test_array_u8_256_roundtrip() {
    let arr: [u8; 256] = core::array::from_fn(|i| i as u8);
    let enc = oxicode::encode_to_vec(&arr).expect("encode u8 array");
    let (dec, _): ([u8; 256], _) = oxicode::decode_from_slice(&enc).expect("decode u8 array");
    assert_eq!(arr, dec);
}

#[test]
fn test_array_f32_8_roundtrip() {
    let arr: [f32; 8] = [1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0];
    let enc = oxicode::encode_to_vec(&arr).expect("encode f32 array");
    let (dec, _): ([f32; 8], _) = oxicode::decode_from_slice(&enc).expect("decode f32 array");
    assert_eq!(arr, dec);
}

#[test]
fn test_array_i64_4_roundtrip() {
    let arr: [i64; 4] = [-1000, 0, 1000, i64::MAX / 2];
    let enc = oxicode::encode_to_vec(&arr).expect("encode i64 array");
    let (dec, _): ([i64; 4], _) = oxicode::decode_from_slice(&enc).expect("decode i64 array");
    assert_eq!(arr, dec);
}

#[test]
fn test_array_u32_16_roundtrip() {
    let arr: [u32; 16] = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16];
    let enc = oxicode::encode_to_vec(&arr).expect("encode u32 array");
    let (dec, _): ([u32; 16], _) = oxicode::decode_from_slice(&enc).expect("decode u32 array");
    assert_eq!(arr, dec);
}

#[test]
fn test_simd_array_encode_decode_bytes_length_correct() {
    // For [u32; 4] the standard encoding writes 4 varint-encoded u32 values
    // after a length prefix.  Just confirm roundtrip gives the right values,
    // not a specific byte length (which is config-dependent).
    let arr: [u32; 4] = [0, u32::MAX / 2, u32::MAX - 1, u32::MAX];
    let enc = oxicode::encode_to_vec(&arr).expect("encode");
    assert!(!enc.is_empty());
    let (dec, _): ([u32; 4], _) = oxicode::decode_from_slice(&enc).expect("decode");
    assert_eq!(arr, dec);
}

// ---------------------------------------------------------------------------
// Large array (32K+ elements) and cross-type consistency tests
// ---------------------------------------------------------------------------

#[cfg(feature = "simd")]
mod simd_large_tests {
    use oxicode::simd::{decode_simd_array, encode_simd_array};

    #[test]
    fn test_f32_roundtrip_32k_elements() {
        let data: Vec<f32> = (0u32..32768).map(|i| i as f32).collect();
        let encoded = encode_simd_array(&data).expect("encode 32k f32");
        let decoded: Vec<f32> = decode_simd_array(&encoded).expect("decode 32k f32");
        assert_eq!(data, decoded);
    }

    #[test]
    fn test_f64_roundtrip_32k_elements() {
        let data: Vec<f64> = (0u32..32768).map(|i| i as f64 * 0.0001).collect();
        let encoded = encode_simd_array(&data).expect("encode 32k f64");
        let decoded: Vec<f64> = decode_simd_array(&encoded).expect("decode 32k f64");
        assert_eq!(data, decoded);
    }

    #[test]
    fn test_i32_roundtrip_32k_elements() {
        let data: Vec<i32> = (0u32..32768).map(|i| (i as i32) - 16384).collect();
        let encoded = encode_simd_array(&data).expect("encode 32k i32");
        let decoded: Vec<i32> = decode_simd_array(&encoded).expect("decode 32k i32");
        assert_eq!(data, decoded);
    }

    #[test]
    fn test_i64_roundtrip_32k_elements() {
        let data: Vec<i64> = (0u32..32768).map(|i| i as i64 * 1000).collect();
        let encoded = encode_simd_array(&data).expect("encode 32k i64");
        let decoded: Vec<i64> = decode_simd_array(&encoded).expect("decode 32k i64");
        assert_eq!(data, decoded);
    }

    #[test]
    fn test_u8_roundtrip_32k_elements() {
        let data: Vec<u8> = (0u32..32768).map(|i| (i & 0xFF) as u8).collect();
        let encoded = encode_simd_array(&data).expect("encode 32k u8");
        let decoded: Vec<u8> = decode_simd_array(&encoded).expect("decode 32k u8");
        assert_eq!(data, decoded);
    }

    #[test]
    fn test_simd_encode_decode_roundtrip_matches_original_f32() {
        // encode_simd_array uses its own binary format (with 8-byte header);
        // verify that the simd round-trip recovers the exact same values.
        let data_vec: Vec<f32> = (0..16).map(|i| i as f32 * 1.5).collect();
        let simd_enc = encode_simd_array(&data_vec).expect("encode simd f32");
        let decoded: Vec<f32> = decode_simd_array(&simd_enc).expect("decode simd f32");
        assert_eq!(
            data_vec, decoded,
            "simd f32 round-trip must recover original"
        );
    }

    #[test]
    fn test_simd_encode_decode_roundtrip_matches_original_u8() {
        // Same as above for u8: verify simd round-trip fidelity.
        let data_vec: Vec<u8> = (0u8..32).collect();
        let simd_enc = encode_simd_array(&data_vec).expect("encode simd u8");
        let decoded: Vec<u8> = decode_simd_array(&simd_enc).expect("decode simd u8");
        assert_eq!(
            data_vec, decoded,
            "simd u8 round-trip must recover original"
        );
    }

    #[test]
    fn test_f32_single_element_roundtrip() {
        let data: Vec<f32> = vec![std::f32::consts::PI];
        let encoded = encode_simd_array(&data).expect("encode single f32");
        let decoded: Vec<f32> = decode_simd_array(&encoded).expect("decode single f32");
        assert_eq!(data.len(), decoded.len());
        assert_eq!(data[0].to_bits(), decoded[0].to_bits());
    }

    #[test]
    fn test_i32_all_zeros_roundtrip() {
        let data: Vec<i32> = vec![0i32; 1024];
        let encoded = encode_simd_array(&data).expect("encode 1024 zeros");
        let decoded: Vec<i32> = decode_simd_array(&encoded).expect("decode 1024 zeros");
        assert_eq!(data, decoded);
    }

    #[test]
    fn test_f64_nan_roundtrip() {
        let data: Vec<f64> = vec![f64::NAN];
        let encoded = encode_simd_array(&data).expect("encode NaN");
        let decoded: Vec<f64> = decode_simd_array(&encoded).expect("decode NaN");
        assert_eq!(decoded.len(), 1);
        // Compare bit patterns to preserve NaN
        assert_eq!(
            data[0].to_bits(),
            decoded[0].to_bits(),
            "NaN bit pattern must be preserved"
        );
    }

    #[test]
    fn test_i32_alternating_pos_neg() {
        let data: Vec<i32> = (0u32..2048)
            .map(|i| if i % 2 == 0 { i as i32 } else { -(i as i32) })
            .collect();
        let encoded = encode_simd_array(&data).expect("encode alternating");
        let decoded: Vec<i32> = decode_simd_array(&encoded).expect("decode alternating");
        assert_eq!(data, decoded);
    }

    #[test]
    fn test_u8_max_values() {
        let data: Vec<u8> = vec![0xFFu8; 256];
        let encoded = encode_simd_array(&data).expect("encode 256 max u8");
        let decoded: Vec<u8> = decode_simd_array(&encoded).expect("decode 256 max u8");
        assert_eq!(data, decoded);
    }

    // -----------------------------------------------------------------------
    // New tests: large arrays, boundary values, determinism, alignment
    // -----------------------------------------------------------------------

    /// 1. Large array of u8 (65536 elements) roundtrip with SIMD.
    #[test]
    fn test_u8_roundtrip_65536_elements() {
        let data: Vec<u8> = (0u32..65536).map(|i| (i & 0xFF) as u8).collect();
        let encoded = encode_simd_array(&data).expect("encode 65536 u8");
        let decoded: Vec<u8> = decode_simd_array(&encoded).expect("decode 65536 u8");
        assert_eq!(data, decoded);
    }

    /// 2. Large array of u16 (8192 elements) roundtrip with SIMD.
    ///
    /// `u16` is not directly supported by `encode_simd_array`; we reinterpret
    /// as raw bytes (little-endian) before encoding and reconstruct afterwards,
    /// which lets us exercise the u8 SIMD path at 16384 bytes.
    #[test]
    fn test_u16_roundtrip_8192_elements() {
        let data: Vec<u16> = (0u16..8192).collect();
        // Serialize to raw bytes (LE), round-trip through SIMD u8 path, reconstruct.
        let raw: Vec<u8> = data.iter().flat_map(|&v| v.to_le_bytes()).collect();
        let encoded = encode_simd_array(&raw).expect("encode u16 as u8");
        let decoded_raw: Vec<u8> = decode_simd_array(&encoded).expect("decode u16 as u8");
        assert_eq!(raw, decoded_raw);
        let reconstructed: Vec<u16> = decoded_raw
            .chunks_exact(2)
            .map(|b| u16::from_le_bytes([b[0], b[1]]))
            .collect();
        assert_eq!(data, reconstructed);
    }

    /// 3. Large array of i64 (4096 elements) roundtrip.
    #[test]
    fn test_i64_roundtrip_4096_elements() {
        // Use saturating arithmetic to avoid overflow in debug builds.
        let data: Vec<i64> = (0i64..4096)
            .map(|i| i.saturating_mul(i64::MAX / 4096))
            .collect();
        let encoded = encode_simd_array(&data).expect("encode 4096 i64");
        let decoded: Vec<i64> = decode_simd_array(&encoded).expect("decode 4096 i64");
        assert_eq!(data, decoded);
    }

    /// 4. Large array of f32 (8192 elements) roundtrip.
    #[test]
    fn test_f32_roundtrip_8192_elements() {
        let data: Vec<f32> = (0u32..8192).map(|i| (i as f32) * 0.123_456_7).collect();
        let encoded = encode_simd_array(&data).expect("encode 8192 f32");
        let decoded: Vec<f32> = decode_simd_array(&encoded).expect("decode 8192 f32");
        assert_eq!(data, decoded);
    }

    /// 5. Zero-length array roundtrip (all supported types).
    #[test]
    fn test_zero_length_array_roundtrip_all_types() {
        let empty_u8: Vec<u8> = vec![];
        let enc = encode_simd_array(&empty_u8).expect("encode empty u8");
        let dec: Vec<u8> = decode_simd_array(&enc).expect("decode empty u8");
        assert!(dec.is_empty());

        let empty_f32: Vec<f32> = vec![];
        let enc = encode_simd_array(&empty_f32).expect("encode empty f32");
        let dec: Vec<f32> = decode_simd_array(&enc).expect("decode empty f32");
        assert!(dec.is_empty());

        let empty_i32: Vec<i32> = vec![];
        let enc = encode_simd_array(&empty_i32).expect("encode empty i32");
        let dec: Vec<i32> = decode_simd_array(&enc).expect("decode empty i32");
        assert!(dec.is_empty());

        let empty_i64: Vec<i64> = vec![];
        let enc = encode_simd_array(&empty_i64).expect("encode empty i64");
        let dec: Vec<i64> = decode_simd_array(&enc).expect("decode empty i64");
        assert!(dec.is_empty());

        let empty_f64: Vec<f64> = vec![];
        let enc = encode_simd_array(&empty_f64).expect("encode empty f64");
        let dec: Vec<f64> = decode_simd_array(&enc).expect("decode empty f64");
        assert!(dec.is_empty());
    }

    /// 6. Array with boundary values (u8::MIN and u8::MAX mixed) roundtrip.
    #[test]
    fn test_u8_boundary_values_mixed_roundtrip() {
        // Interleave 0 and 255 to stress boundary handling in SIMD lanes.
        let data: Vec<u8> = (0u32..512)
            .map(|i| if i % 2 == 0 { u8::MIN } else { u8::MAX })
            .collect();
        let encoded = encode_simd_array(&data).expect("encode boundary u8");
        let decoded: Vec<u8> = decode_simd_array(&encoded).expect("decode boundary u8");
        assert_eq!(data, decoded);
    }

    /// 7. Two independent `encode_simd_array` calls on the same data produce
    ///    byte-identical output (determinism across calls).
    #[test]
    fn test_encode_simd_array_determinism_same_bytes() {
        let data: Vec<f32> = (0u32..256).map(|i| i as f32 * 1.5).collect();
        let enc1 = encode_simd_array(&data).expect("encode first call");
        let enc2 = encode_simd_array(&data).expect("encode second call");
        assert_eq!(
            enc1, enc2,
            "repeated encode_simd_array calls must produce identical bytes"
        );
        // Verify roundtrip too.
        let dec: Vec<f32> = decode_simd_array(&enc1).expect("decode determinism test");
        assert_eq!(data, dec);
    }

    /// 8. Multiple encode calls with the same data produce identical bytes
    ///    (extended determinism: 5 independent calls, all types).
    #[test]
    fn test_multiple_encode_calls_determinism_all_types() {
        // f32
        let f32_data: Vec<f32> = (0u16..128).map(|i| i as f32).collect();
        let f32_enc: Vec<Vec<u8>> = (0..5)
            .map(|_| encode_simd_array(&f32_data).expect("encode f32"))
            .collect();
        for w in f32_enc.windows(2) {
            assert_eq!(w[0], w[1], "f32 encode non-deterministic");
        }

        // i64
        let i64_data: Vec<i64> = (0i64..128).map(|i| i * -999).collect();
        let i64_enc: Vec<Vec<u8>> = (0..5)
            .map(|_| encode_simd_array(&i64_data).expect("encode i64"))
            .collect();
        for w in i64_enc.windows(2) {
            assert_eq!(w[0], w[1], "i64 encode non-deterministic");
        }

        // u8
        let u8_data: Vec<u8> = (0u8..=255).collect();
        let u8_enc: Vec<Vec<u8>> = (0..5)
            .map(|_| encode_simd_array(&u8_data).expect("encode u8"))
            .collect();
        for w in u8_enc.windows(2) {
            assert_eq!(w[0], w[1], "u8 encode non-deterministic");
        }
    }

    /// 9. Array of i32 negative values roundtrip.
    #[test]
    fn test_i32_all_negative_values_roundtrip() {
        // Use the full range of negative i32 sampled at regular intervals.
        let data: Vec<i32> = (1u32..=1024)
            .map(|i| -(i as i32) * 2097152) // spread across negative half
            .collect();
        let encoded = encode_simd_array(&data).expect("encode negative i32");
        let decoded: Vec<i32> = decode_simd_array(&encoded).expect("decode negative i32");
        assert_eq!(data, decoded);
    }

    /// 10. Very large array (262144 u8 elements) roundtrip — exercises
    ///     the full SIMD inner loop many times and validates correctness.
    #[test]
    fn test_u8_roundtrip_262144_elements() {
        // Use a mixing constant that fits in u32 (LCG-style: 1664525 * 2^32 / 2^32 style)
        let data: Vec<u8> = (0u32..262144)
            .map(|i| i.wrapping_mul(1_664_525).wrapping_add(1_013_904_223) as u8)
            .collect();
        let encoded = encode_simd_array(&data).expect("encode 262144 u8");
        let decoded: Vec<u8> = decode_simd_array(&encoded).expect("decode 262144 u8");
        assert_eq!(data, decoded);
    }

    /// 11. `optimal_alignment()` returns a valid alignment value (power of 2,
    ///     at least 1, and consistent with SIMD_ALIGNMENT).
    #[test]
    fn test_optimal_alignment_is_power_of_two_and_consistent() {
        use oxicode::simd::{optimal_alignment, SIMD_ALIGNMENT};
        let align = optimal_alignment();
        assert!(align >= 1, "alignment must be at least 1");
        assert_eq!(
            align & (align - 1),
            0,
            "optimal_alignment must be a power of two, got {align}"
        );
        // SIMD_ALIGNMENT is the compile-time constant; optimal_alignment() must
        // be compatible (either equal or a multiple / factor of it).
        assert!(
            align <= SIMD_ALIGNMENT || SIMD_ALIGNMENT % align == 0 || align % SIMD_ALIGNMENT == 0,
            "optimal_alignment ({align}) must be compatible with SIMD_ALIGNMENT ({SIMD_ALIGNMENT})"
        );
    }
}
