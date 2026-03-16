//! Comprehensive SIMD operations tests.
//!
//! 20 focused tests covering AlignedVec API surface, alignment guarantees,
//! capacity management, clone/deref semantics, and round-trips through both
//! the SIMD direct path and the standard encode/decode path with the `simd`
//! feature active.
//!
//! The entire file is gated under `#![cfg(feature = "simd")]`.

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
use oxicode::simd::{optimal_alignment, AlignedVec, SIMD_ALIGNMENT};
use oxicode::{decode_from_slice, encode_to_vec};

// ---------------------------------------------------------------------------
// 1. AlignedVec<u8> creation and basic push/len/is_empty operations
// ---------------------------------------------------------------------------

/// Verifies that a freshly created `AlignedVec<u8>` is empty, that pushing
/// bytes increments the length, and that `is_empty` toggles correctly.
#[test]
fn test_aligned_vec_u8_creation_and_basic_ops() {
    let mut av: AlignedVec<u8> = AlignedVec::new();
    assert!(av.is_empty(), "new AlignedVec must be empty");
    assert_eq!(av.len(), 0, "new AlignedVec must have len 0");

    av.push(0xDE_u8);
    av.push(0xAD_u8);
    av.push(0xBE_u8);
    av.push(0xEF_u8);

    assert!(!av.is_empty(), "AlignedVec must not be empty after pushes");
    assert_eq!(av.len(), 4, "len must equal number of pushed elements");
    assert_eq!(av.as_slice(), &[0xDE, 0xAD, 0xBE, 0xEF]);
}

// ---------------------------------------------------------------------------
// 2. AlignedVec<u32> roundtrip via encode_simd_array / decode_simd_array
// ---------------------------------------------------------------------------

/// Encodes a Vec<u32> through the SIMD array path (reinterpreted as i32),
/// then stores the encoded bytes in an `AlignedVec<u8>` and decodes from it.
#[test]
fn test_aligned_vec_u32_roundtrip_via_simd_array() {
    use oxicode::simd::{decode_simd_array, encode_simd_array};

    let data: Vec<u32> = (0u32..64).map(|i| i.wrapping_mul(0xDEAD_BEEF)).collect();
    let as_i32: Vec<i32> = data.iter().map(|&v| v as i32).collect();

    let encoded = encode_simd_array(&as_i32).expect("encode u32 via i32 SIMD");
    let mut av: AlignedVec<u8> = AlignedVec::with_capacity(encoded.len());
    av.extend(encoded.iter().copied());

    assert_eq!(
        av.len(),
        encoded.len(),
        "AlignedVec must hold all encoded bytes"
    );

    let decoded_i32: Vec<i32> =
        decode_simd_array(av.as_slice()).expect("decode u32 from AlignedVec via i32 SIMD");
    let reconstructed: Vec<u32> = decoded_i32.iter().map(|&v| v as u32).collect();
    assert_eq!(
        data, reconstructed,
        "u32 AlignedVec roundtrip must be exact"
    );
}

// ---------------------------------------------------------------------------
// 3. AlignedVec<u64> roundtrip via SIMD array path (via i64 reinterpret)
// ---------------------------------------------------------------------------

#[test]
fn test_aligned_vec_u64_roundtrip_via_simd_array() {
    use oxicode::simd::{decode_simd_array, encode_simd_array};

    let data: Vec<u64> = (0u64..32)
        .map(|i| i.wrapping_mul(0xC0FF_EE00_DEAD_BEEF))
        .collect();
    let as_i64: Vec<i64> = data.iter().map(|&v| v as i64).collect();

    let encoded = encode_simd_array(&as_i64).expect("encode u64 via i64 SIMD");
    let mut av: AlignedVec<u8> = AlignedVec::new();
    for &b in &encoded {
        av.push(b);
    }

    let decoded_i64: Vec<i64> =
        decode_simd_array(av.as_slice()).expect("decode u64 from AlignedVec via i64 SIMD");
    let reconstructed: Vec<u64> = decoded_i64.iter().map(|&v| v as u64).collect();
    assert_eq!(
        data, reconstructed,
        "u64 AlignedVec roundtrip must be bit-exact"
    );
}

// ---------------------------------------------------------------------------
// 4. AlignedVec<f32> roundtrip with bits comparison
// ---------------------------------------------------------------------------

#[test]
fn test_aligned_vec_f32_roundtrip_bits_comparison() {
    use oxicode::simd::{decode_simd_array, encode_simd_array};

    let data: Vec<f32> = (0u32..48)
        .map(|i| {
            let x = i as f32 * 0.12345_f32;
            if i % 5 == 0 {
                -x
            } else {
                x
            }
        })
        .collect();

    let encoded = encode_simd_array(&data).expect("encode f32 SIMD");
    let mut av: AlignedVec<u8> = AlignedVec::from_slice(&encoded);
    let decoded: Vec<f32> =
        decode_simd_array(av.as_slice()).expect("decode f32 from AlignedVec SIMD");

    assert_eq!(data.len(), decoded.len(), "decoded length must match");
    for (idx, (a, b)) in data.iter().zip(decoded.iter()).enumerate() {
        assert_eq!(a.to_bits(), b.to_bits(), "f32 bit mismatch at index {idx}");
    }
    // Verify the AlignedVec slot itself is mutable and can be cleared
    av.clear();
    assert!(av.is_empty(), "AlignedVec must be empty after clear");
}

// ---------------------------------------------------------------------------
// 5. AlignedVec<f64> roundtrip with bits comparison
// ---------------------------------------------------------------------------

#[test]
fn test_aligned_vec_f64_roundtrip_bits_comparison() {
    use oxicode::simd::{decode_simd_array, encode_simd_array};

    let data: Vec<f64> = (0u64..24)
        .map(|i| {
            let x = (i as f64) * core::f64::consts::LN_2;
            if i % 3 == 0 {
                -x
            } else {
                x
            }
        })
        .collect();

    let encoded = encode_simd_array(&data).expect("encode f64 SIMD");
    let av = AlignedVec::from_slice(&encoded);
    let decoded: Vec<f64> =
        decode_simd_array(av.as_slice()).expect("decode f64 from AlignedVec SIMD");

    for (idx, (a, b)) in data.iter().zip(decoded.iter()).enumerate() {
        assert_eq!(a.to_bits(), b.to_bits(), "f64 bit mismatch at index {idx}");
    }
}

// ---------------------------------------------------------------------------
// 6. AlignedVec<u8> with 4 KB of data: capacity, alignment, and content
// ---------------------------------------------------------------------------

#[test]
fn test_aligned_vec_u8_4kb_data() {
    const SIZE: usize = 4096;
    let data: Vec<u8> = (0..SIZE as u64)
        .map(|i| (i.wrapping_mul(6_364_136_223_u64) >> 16) as u8)
        .collect();

    let mut av: AlignedVec<u8> = AlignedVec::with_capacity(SIZE);
    av.extend(data.iter().copied());

    assert_eq!(av.len(), SIZE, "AlignedVec must hold all 4 KB of bytes");
    assert!(av.capacity() >= SIZE, "capacity must be at least SIZE");
    assert!(
        av.is_aligned(),
        "AlignedVec must be SIMD-aligned after extend"
    );
    assert_eq!(
        av.as_slice(),
        data.as_slice(),
        "content must match original data"
    );

    // Verify raw pointer alignment
    let addr = av.as_ptr() as usize;
    assert_eq!(
        addr % SIMD_ALIGNMENT,
        0,
        "4KB AlignedVec pointer must be {SIMD_ALIGNMENT}-byte aligned (addr={addr:#x})"
    );
}

// ---------------------------------------------------------------------------
// 7. optimal_alignment() returns a power of 2
// ---------------------------------------------------------------------------

#[test]
fn test_optimal_alignment_is_power_of_two() {
    let align = optimal_alignment();
    assert!(align >= 1, "optimal_alignment must be >= 1, got {align}");
    assert_eq!(
        align & align.wrapping_sub(1),
        0,
        "optimal_alignment must be a power of 2, got {align}"
    );
}

// ---------------------------------------------------------------------------
// 8. optimal_alignment() is at least 16
// ---------------------------------------------------------------------------

#[test]
fn test_optimal_alignment_at_least_16() {
    let align = optimal_alignment();
    assert!(
        align >= 16,
        "optimal_alignment must be >= 16 on any supported target, got {align}"
    );
}

// ---------------------------------------------------------------------------
// 9. AlignedVec alignment property: raw pointer is properly SIMD-aligned
// ---------------------------------------------------------------------------

#[test]
fn test_aligned_vec_ptr_is_simd_aligned() {
    // Test across multiple element types.
    let av_u8: AlignedVec<u8> = AlignedVec::with_capacity(128);
    let addr_u8 = av_u8.as_ptr() as usize;
    assert_eq!(
        addr_u8 % SIMD_ALIGNMENT,
        0,
        "AlignedVec<u8> pointer must be {SIMD_ALIGNMENT}-byte aligned (addr={addr_u8:#x})"
    );

    let av_f32: AlignedVec<f32> = AlignedVec::with_capacity(64);
    let addr_f32 = av_f32.as_ptr() as usize;
    assert_eq!(
        addr_f32 % SIMD_ALIGNMENT,
        0,
        "AlignedVec<f32> pointer must be {SIMD_ALIGNMENT}-byte aligned (addr={addr_f32:#x})"
    );

    let av_i64: AlignedVec<i64> = AlignedVec::with_capacity(32);
    let addr_i64 = av_i64.as_ptr() as usize;
    assert_eq!(
        addr_i64 % SIMD_ALIGNMENT,
        0,
        "AlignedVec<i64> pointer must be {SIMD_ALIGNMENT}-byte aligned (addr={addr_i64:#x})"
    );
}

// ---------------------------------------------------------------------------
// 10. AlignedVec::from_slice constructs a correctly aligned copy
// ---------------------------------------------------------------------------

#[test]
fn test_aligned_vec_from_slice_correctness() {
    let original: Vec<u32> = (0u32..100).map(|i| i * 1_000_003).collect();
    let av = AlignedVec::from_slice(&original);

    assert_eq!(av.len(), original.len(), "from_slice must preserve length");
    assert_eq!(
        av.as_slice(),
        original.as_slice(),
        "from_slice must preserve all elements"
    );
    assert!(av.is_aligned(), "from_slice result must be SIMD-aligned");

    // Verify pointer alignment directly
    let addr = av.as_ptr() as usize;
    assert_eq!(
        addr % SIMD_ALIGNMENT,
        0,
        "from_slice result pointer must be {SIMD_ALIGNMENT}-byte aligned (addr={addr:#x})"
    );
}

// ---------------------------------------------------------------------------
// 11. AlignedVec length, capacity, reserve, and pop semantics
// ---------------------------------------------------------------------------

#[test]
fn test_aligned_vec_length_capacity_reserve_pop() {
    let mut av: AlignedVec<i64> = AlignedVec::new();
    assert_eq!(av.len(), 0);
    assert_eq!(av.capacity(), 0);

    av.reserve(200);
    assert!(
        av.capacity() >= 200,
        "after reserve(200) capacity must be >= 200"
    );
    assert_eq!(av.len(), 0, "reserve must not change len");

    for v in (0i64..50).map(|i| i * i - 100) {
        av.push(v);
    }
    assert_eq!(av.len(), 50, "len must equal number of pushed elements");

    let last = av.pop().expect("pop on non-empty AlignedVec must succeed");
    assert_eq!(
        last,
        49 * 49 - 100,
        "pop must return the last pushed element"
    );
    assert_eq!(av.len(), 49, "len must decrease by 1 after pop");

    // Pop down to empty
    while av.pop().is_some() {}
    assert!(
        av.is_empty(),
        "AlignedVec must be empty after popping all elements"
    );
    assert!(
        av.pop().is_none(),
        "pop on empty AlignedVec must return None"
    );
}

// ---------------------------------------------------------------------------
// 12. Encode then decode AlignedVec<f32> matches original Vec<f32>
// ---------------------------------------------------------------------------

/// Encodes `Vec<f32>` via SIMD, stores result in `AlignedVec<u8>`, decodes
/// back to `Vec<f32>`, and compares byte-for-byte with the original.
#[test]
fn test_encode_decode_aligned_vec_matches_vec() {
    use oxicode::simd::{decode_simd_array, encode_simd_array};

    let original: Vec<f32> = (0u32..200)
        .map(|i| {
            let t = i as f32 / 200.0;
            t * t - 0.5
        })
        .collect();

    let encoded = encode_simd_array(&original).expect("encode Vec<f32>");
    let av = AlignedVec::from_slice(&encoded);
    let decoded: Vec<f32> = decode_simd_array(av.as_slice()).expect("decode from AlignedVec");

    assert_eq!(
        original.len(),
        decoded.len(),
        "decoded length must match original"
    );
    for (idx, (a, b)) in original.iter().zip(decoded.iter()).enumerate() {
        assert_eq!(
            a.to_bits(),
            b.to_bits(),
            "f32 element mismatch at index {idx}"
        );
    }
}

// ---------------------------------------------------------------------------
// 13. AlignedVec<u8> vs Vec<u8>: SIMD encoding of same data gives identical bytes
// ---------------------------------------------------------------------------

/// Verifies that encoding the same `Vec<u8>` data via `encode_simd_array`
/// twice produces identical byte sequences, and that `AlignedVec::from_slice`
/// on the result preserves byte identity.
#[test]
fn test_aligned_vec_u8_vs_vec_u8_same_encoding() {
    use oxicode::simd::encode_simd_array;

    let data: Vec<u8> = (0u8..=255).cycle().take(512).collect();

    let enc_a = encode_simd_array(&data).expect("first encode");
    let enc_b = encode_simd_array(&data).expect("second encode");
    assert_eq!(
        enc_a, enc_b,
        "two identical encodes must produce identical bytes"
    );

    let av = AlignedVec::from_slice(&enc_a);
    assert_eq!(
        av.as_slice(),
        enc_a.as_slice(),
        "AlignedVec from_slice must match the Vec<u8> slice it was built from"
    );
}

// ---------------------------------------------------------------------------
// 14. Multiple AlignedVec roundtrips (repeated encode-store-decode cycles)
// ---------------------------------------------------------------------------

/// Performs 5 encode-store-in-AlignedVec-decode cycles on different sizes,
/// verifying that each cycle produces exact bit-level recovery.
#[test]
fn test_multiple_aligned_vec_roundtrips() {
    use oxicode::simd::{decode_simd_array, encode_simd_array};

    for n in [7usize, 15, 63, 127, 255] {
        let data: Vec<i32> = (0..n as i32)
            .map(|i| {
                if i % 2 == 0 {
                    i * 17 + 3
                } else {
                    -(i * 11 + 7)
                }
            })
            .collect();

        let encoded = encode_simd_array(&data).expect("encode i32");
        let av = AlignedVec::<u8>::from_slice(&encoded);
        let decoded: Vec<i32> =
            decode_simd_array(av.as_slice()).expect("decode i32 from AlignedVec");

        assert_eq!(data, decoded, "roundtrip mismatch for n={n}");
    }
}

// ---------------------------------------------------------------------------
// 15. AlignedVec in a struct with oxicode Encode/Decode derive
// ---------------------------------------------------------------------------

/// Encodes a struct that holds a `Vec<f32>` (standard path) while the
/// `simd` feature is active, verifying that SIMD dispatch does not interfere
/// with the standard derive-based encode/decode path.
#[test]
fn test_aligned_vec_in_struct_derive() {
    #[derive(oxicode::Encode, oxicode::Decode, PartialEq, Debug)]
    struct SensorFrame {
        sensor_id: u32,
        timestamp_ms: u64,
        readings: Vec<f32>,
        flags: u8,
    }

    let frame = SensorFrame {
        sensor_id: 0xABCD_1234,
        timestamp_ms: 1_700_000_000_000,
        readings: (0u32..32)
            .map(|i| (i as f32) * 0.001_f32 - 0.016_f32)
            .collect(),
        flags: 0b0000_1111,
    };

    let encoded = encode_to_vec(&frame).expect("encode SensorFrame");
    let (decoded, _): (SensorFrame, _) = decode_from_slice(&encoded).expect("decode SensorFrame");

    assert_eq!(frame, decoded, "SensorFrame must round-trip exactly");
}

// ---------------------------------------------------------------------------
// 16. Empty AlignedVec roundtrip (zero-length encode/decode)
// ---------------------------------------------------------------------------

/// Encodes an empty slice via the SIMD array path and verifies that decoding
/// also yields an empty Vec.  An empty `AlignedVec` must be vacuously aligned.
#[test]
fn test_empty_aligned_vec_roundtrip() {
    use oxicode::simd::{decode_simd_array, encode_simd_array};

    let empty_f32: Vec<f32> = Vec::new();
    let encoded = encode_simd_array(&empty_f32).expect("encode empty f32 slice");
    let decoded: Vec<f32> = decode_simd_array(&encoded).expect("decode empty f32");
    assert!(decoded.is_empty(), "decoded result must also be empty");

    // An empty AlignedVec must report is_aligned() == true (vacuously).
    let av: AlignedVec<u8> = AlignedVec::new();
    assert!(
        av.is_aligned(),
        "empty AlignedVec must report is_aligned() == true"
    );
    assert!(av.is_empty());
}

// ---------------------------------------------------------------------------
// 17. AlignedVec<i32> with negative values
// ---------------------------------------------------------------------------

#[test]
fn test_aligned_vec_i32_negative_values() {
    use oxicode::simd::{decode_simd_array, encode_simd_array};

    let negatives: Vec<i32> = vec![
        i32::MIN,
        i32::MIN + 1,
        -1_000_000_000,
        -123_456_789,
        -1,
        0,
        1,
        123_456_789,
        1_000_000_000,
        i32::MAX - 1,
        i32::MAX,
    ];

    let encoded = encode_simd_array(&negatives).expect("encode i32 negatives");
    let av = AlignedVec::from_slice(&encoded);

    assert!(
        av.is_aligned(),
        "AlignedVec from negative-i32 encode must be aligned"
    );
    let decoded: Vec<i32> =
        decode_simd_array(av.as_slice()).expect("decode i32 negatives from AlignedVec");
    assert_eq!(
        negatives, decoded,
        "negative i32 AlignedVec roundtrip must be exact"
    );
}

// ---------------------------------------------------------------------------
// 18. AlignedVec<bool> roundtrip via standard encode/decode path
// ---------------------------------------------------------------------------

/// `bool` is not directly supported by `encode_simd_array`, but the standard
/// `encode_to_vec` / `decode_from_slice` path dispatches through SIMD
/// internally when the feature is active.  This test verifies that `bool`
/// encoding is unaffected.
#[test]
fn test_aligned_vec_bool_roundtrip_standard_path() {
    let data: Vec<bool> = (0..128_u32)
        .map(|i| match i % 4 {
            0 => true,
            1 => false,
            2 => true,
            _ => false,
        })
        .collect();

    let encoded = encode_to_vec(&data).expect("encode Vec<bool>");
    let (decoded, bytes_consumed): (Vec<bool>, _) =
        decode_from_slice(&encoded).expect("decode Vec<bool>");

    assert_eq!(data, decoded, "Vec<bool> must round-trip exactly");
    assert_eq!(
        bytes_consumed,
        encoded.len(),
        "all encoded bytes must be consumed"
    );
}

// ---------------------------------------------------------------------------
// 19. Large AlignedVec (10 000 elements) via SIMD array path
// ---------------------------------------------------------------------------

#[test]
fn test_large_aligned_vec_10000_elements() {
    use oxicode::simd::{decode_simd_array, encode_simd_array};

    const N: usize = 10_000;
    let data: Vec<f64> = (0..N)
        .map(|i| {
            let t = i as f64 / N as f64;
            core::f64::consts::TAU * t - core::f64::consts::PI
        })
        .collect();

    let encoded = encode_simd_array(&data).expect("encode 10 000 f64");
    let av = AlignedVec::from_slice(&encoded);

    assert!(av.is_aligned(), "large AlignedVec must be SIMD-aligned");
    assert!(!av.is_empty(), "large AlignedVec must not be empty");

    let decoded: Vec<f64> =
        decode_simd_array(av.as_slice()).expect("decode 10 000 f64 from AlignedVec");
    assert_eq!(decoded.len(), N, "decoded len must equal N");

    for (idx, (a, b)) in data.iter().zip(decoded.iter()).enumerate() {
        assert_eq!(a.to_bits(), b.to_bits(), "f64 bit mismatch at index {idx}");
    }
}

// ---------------------------------------------------------------------------
// 20. AlignedVec encode+decode identity for all SIMD element types
// ---------------------------------------------------------------------------

/// A comprehensive identity test: for each type supported by `encode_simd_array`
/// (u8, i32, i64, f32, f64) create an `AlignedVec<u8>` holding the encoded
/// bytes and verify that decoding from it recovers the original data exactly.
/// Each sub-test uses a different data pattern to maximise coverage.
#[test]
fn test_aligned_vec_encode_decode_identity_all_types() {
    use oxicode::simd::{decode_simd_array, encode_simd_array};

    // --- u8 ---
    let u8_data: Vec<u8> = (0u8..=255).collect();
    {
        let enc = encode_simd_array(&u8_data).expect("encode u8 identity");
        let av = AlignedVec::from_slice(&enc);
        let dec: Vec<u8> = decode_simd_array(av.as_slice()).expect("decode u8 identity");
        assert_eq!(u8_data, dec, "u8 identity roundtrip via AlignedVec");
    }

    // --- i32 ---
    let i32_data: Vec<i32> = (0i32..256)
        .map(|i| i.wrapping_mul(i32::MAX / 256).wrapping_sub(i32::MAX / 2))
        .collect();
    {
        let enc = encode_simd_array(&i32_data).expect("encode i32 identity");
        let av = AlignedVec::from_slice(&enc);
        let dec: Vec<i32> = decode_simd_array(av.as_slice()).expect("decode i32 identity");
        assert_eq!(i32_data, dec, "i32 identity roundtrip via AlignedVec");
    }

    // --- i64 ---
    let i64_data: Vec<i64> = (0i64..128)
        .map(|i| i.wrapping_mul(i64::MAX / 128).wrapping_sub(i64::MAX / 4))
        .collect();
    {
        let enc = encode_simd_array(&i64_data).expect("encode i64 identity");
        let av = AlignedVec::from_slice(&enc);
        let dec: Vec<i64> = decode_simd_array(av.as_slice()).expect("decode i64 identity");
        assert_eq!(i64_data, dec, "i64 identity roundtrip via AlignedVec");
    }

    // --- f32 (finite values only, avoiding NaN/Inf edge cases) ---
    let f32_data: Vec<f32> = (0u32..64)
        .map(|i| (i as f32 - 32.0) * core::f32::consts::FRAC_1_PI)
        .collect();
    {
        let enc = encode_simd_array(&f32_data).expect("encode f32 identity");
        let av = AlignedVec::from_slice(&enc);
        let dec: Vec<f32> = decode_simd_array(av.as_slice()).expect("decode f32 identity");
        for (idx, (a, b)) in f32_data.iter().zip(dec.iter()).enumerate() {
            assert_eq!(
                a.to_bits(),
                b.to_bits(),
                "f32 identity bit mismatch at {idx}"
            );
        }
    }

    // --- f64 ---
    let f64_data: Vec<f64> = (0u64..32)
        .map(|i| (i as f64 - 16.0) * core::f64::consts::FRAC_1_PI)
        .collect();
    {
        let enc = encode_simd_array(&f64_data).expect("encode f64 identity");
        let av = AlignedVec::from_slice(&enc);
        let dec: Vec<f64> = decode_simd_array(av.as_slice()).expect("decode f64 identity");
        for (idx, (a, b)) in f64_data.iter().zip(dec.iter()).enumerate() {
            assert_eq!(
                a.to_bits(),
                b.to_bits(),
                "f64 identity bit mismatch at {idx}"
            );
        }
    }
}
