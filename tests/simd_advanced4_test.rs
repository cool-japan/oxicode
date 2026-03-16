//! SIMD-optimized array encoding tests for OxiCode.
//!
//! These tests verify that arrays of various fixed sizes round-trip correctly
//! when the `simd` feature is enabled, including numeric types, nested arrays,
//! Vec-of-array, and config-driven encoding.

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
// Test 1: [u8; 1] roundtrip
// ---------------------------------------------------------------------------
#[test]
fn test_simd_u8_array_1_roundtrip() {
    let original: [u8; 1] = [0xAB];
    let encoded = encode_to_vec(&original).expect("encode [u8; 1]");
    assert_eq!(encoded.len(), 1, "[u8; 1] must encode to exactly 1 byte");

    let (decoded, consumed): ([u8; 1], usize) =
        decode_from_slice(&encoded).expect("decode [u8; 1]");
    assert_eq!(decoded, original, "[u8; 1] roundtrip must be identical");
    assert_eq!(consumed, 1, "must consume exactly 1 byte");
}

// ---------------------------------------------------------------------------
// Test 2: [u8; 4] roundtrip with 0xDEAD_BEEF bytes
// ---------------------------------------------------------------------------
#[test]
fn test_simd_u8_array_4_deadbeef_roundtrip() {
    let original: [u8; 4] = [0xDE, 0xAD, 0xBE, 0xEF];
    let encoded = encode_to_vec(&original).expect("encode [u8; 4] deadbeef");
    assert_eq!(encoded.len(), 4, "[u8; 4] must encode to exactly 4 bytes");
    assert_eq!(
        &encoded[..],
        &[0xDE, 0xAD, 0xBE, 0xEF],
        "bytes must match verbatim"
    );

    let (decoded, consumed): ([u8; 4], usize) =
        decode_from_slice(&encoded).expect("decode [u8; 4] deadbeef");
    assert_eq!(decoded, original, "[u8; 4] roundtrip must be identical");
    assert_eq!(consumed, 4, "must consume exactly 4 bytes");
}

// ---------------------------------------------------------------------------
// Test 3: [u8; 8] roundtrip
// ---------------------------------------------------------------------------
#[test]
fn test_simd_u8_array_8_roundtrip() {
    let original: [u8; 8] = [0, 1, 2, 3, 4, 5, 6, 7];
    let encoded = encode_to_vec(&original).expect("encode [u8; 8]");
    assert_eq!(encoded.len(), 8, "[u8; 8] must encode to exactly 8 bytes");

    let (decoded, consumed): ([u8; 8], usize) =
        decode_from_slice(&encoded).expect("decode [u8; 8]");
    assert_eq!(decoded, original, "[u8; 8] roundtrip must be identical");
    assert_eq!(consumed, 8);
}

// ---------------------------------------------------------------------------
// Test 4: [u8; 16] roundtrip
// ---------------------------------------------------------------------------
#[test]
fn test_simd_u8_array_16_roundtrip() {
    let original: [u8; 16] = [
        0x00, 0x11, 0x22, 0x33, 0x44, 0x55, 0x66, 0x77, 0x88, 0x99, 0xAA, 0xBB, 0xCC, 0xDD, 0xEE,
        0xFF,
    ];
    let encoded = encode_to_vec(&original).expect("encode [u8; 16]");
    assert_eq!(
        encoded.len(),
        16,
        "[u8; 16] must encode to exactly 16 bytes"
    );

    let (decoded, consumed): ([u8; 16], usize) =
        decode_from_slice(&encoded).expect("decode [u8; 16]");
    assert_eq!(decoded, original, "[u8; 16] roundtrip must be identical");
    assert_eq!(consumed, 16);
}

// ---------------------------------------------------------------------------
// Test 5: [u8; 32] roundtrip
// ---------------------------------------------------------------------------
#[test]
fn test_simd_u8_array_32_roundtrip() {
    let mut original: [u8; 32] = [0u8; 32];
    for (i, b) in original.iter_mut().enumerate() {
        *b = (i * 8) as u8;
    }
    let encoded = encode_to_vec(&original).expect("encode [u8; 32]");
    assert_eq!(
        encoded.len(),
        32,
        "[u8; 32] must encode to exactly 32 bytes"
    );

    let (decoded, consumed): ([u8; 32], usize) =
        decode_from_slice(&encoded).expect("decode [u8; 32]");
    assert_eq!(decoded, original, "[u8; 32] roundtrip must be identical");
    assert_eq!(consumed, 32);
}

// ---------------------------------------------------------------------------
// Test 6: [u8; 64] roundtrip
// ---------------------------------------------------------------------------
#[test]
fn test_simd_u8_array_64_roundtrip() {
    let mut original: [u8; 64] = [0u8; 64];
    for (i, b) in original.iter_mut().enumerate() {
        *b = (i * 4) as u8;
    }
    let encoded = encode_to_vec(&original).expect("encode [u8; 64]");
    assert_eq!(
        encoded.len(),
        64,
        "[u8; 64] must encode to exactly 64 bytes"
    );

    let (decoded, consumed): ([u8; 64], usize) =
        decode_from_slice(&encoded).expect("decode [u8; 64]");
    assert_eq!(decoded, original, "[u8; 64] roundtrip must be identical");
    assert_eq!(consumed, 64);
}

// ---------------------------------------------------------------------------
// Test 7: [u8; 128] roundtrip
// ---------------------------------------------------------------------------
#[test]
fn test_simd_u8_array_128_roundtrip() {
    let mut original: [u8; 128] = [0u8; 128];
    for (i, b) in original.iter_mut().enumerate() {
        *b = (i * 2) as u8;
    }
    let encoded = encode_to_vec(&original).expect("encode [u8; 128]");
    assert_eq!(
        encoded.len(),
        128,
        "[u8; 128] must encode to exactly 128 bytes"
    );

    let (decoded, consumed): ([u8; 128], usize) =
        decode_from_slice(&encoded).expect("decode [u8; 128]");
    assert_eq!(decoded, original, "[u8; 128] roundtrip must be identical");
    assert_eq!(consumed, 128);
}

// ---------------------------------------------------------------------------
// Test 8: [u16; 8] roundtrip
// ---------------------------------------------------------------------------
#[test]
fn test_simd_u16_array_8_roundtrip() {
    let original: [u16; 8] = [0, 1, 256, 1024, 4096, 16384, u16::MAX - 1, u16::MAX];
    let encoded = encode_to_vec(&original).expect("encode [u16; 8]");

    let (decoded, consumed): ([u16; 8], usize) =
        decode_from_slice(&encoded).expect("decode [u16; 8]");
    assert_eq!(decoded, original, "[u16; 8] roundtrip must be identical");
    assert_eq!(
        consumed,
        encoded.len(),
        "consumed must equal encoded length"
    );
}

// ---------------------------------------------------------------------------
// Test 9: [u32; 4] roundtrip
// ---------------------------------------------------------------------------
#[test]
fn test_simd_u32_array_4_roundtrip() {
    let original: [u32; 4] = [0, 42, 0x0000_FFFF, u32::MAX];
    let encoded = encode_to_vec(&original).expect("encode [u32; 4]");

    let (decoded, consumed): ([u32; 4], usize) =
        decode_from_slice(&encoded).expect("decode [u32; 4]");
    assert_eq!(decoded, original, "[u32; 4] roundtrip must be identical");
    assert_eq!(consumed, encoded.len());
}

// ---------------------------------------------------------------------------
// Test 10: [u32; 8] roundtrip
// ---------------------------------------------------------------------------
#[test]
fn test_simd_u32_array_8_roundtrip() {
    let original: [u32; 8] = [0, 1, 100, 1_000, 10_000, 100_000, 1_000_000, u32::MAX];
    let encoded = encode_to_vec(&original).expect("encode [u32; 8]");

    let (decoded, consumed): ([u32; 8], usize) =
        decode_from_slice(&encoded).expect("decode [u32; 8]");
    assert_eq!(decoded, original, "[u32; 8] roundtrip must be identical");
    assert_eq!(consumed, encoded.len());
}

// ---------------------------------------------------------------------------
// Test 11: [u64; 4] roundtrip
// ---------------------------------------------------------------------------
#[test]
fn test_simd_u64_array_4_roundtrip() {
    let original: [u64; 4] = [0, u64::MAX / 4, u64::MAX / 2, u64::MAX];
    let encoded = encode_to_vec(&original).expect("encode [u64; 4]");

    let (decoded, consumed): ([u64; 4], usize) =
        decode_from_slice(&encoded).expect("decode [u64; 4]");
    assert_eq!(decoded, original, "[u64; 4] roundtrip must be identical");
    assert_eq!(consumed, encoded.len());
}

// ---------------------------------------------------------------------------
// Test 12: [f32; 8] roundtrip with bit-exact comparison
// ---------------------------------------------------------------------------
#[test]
fn test_simd_f32_array_8_bit_exact_roundtrip() {
    let original: [f32; 8] = [1.5f32, 2.5, 3.5, 4.5, 5.5, 6.5, 7.5, 8.5];
    let encoded = encode_to_vec(&original).expect("encode [f32; 8]");

    let (decoded, consumed): ([f32; 8], usize) =
        decode_from_slice(&encoded).expect("decode [f32; 8]");
    assert_eq!(
        consumed,
        encoded.len(),
        "consumed must equal encoded length"
    );

    for (a, b) in original.iter().zip(decoded.iter()) {
        assert_eq!(a.to_bits(), b.to_bits(), "f32 must be bit-exact");
    }
}

// ---------------------------------------------------------------------------
// Test 13: [f64; 4] roundtrip with bit-exact comparison
// ---------------------------------------------------------------------------
#[test]
fn test_simd_f64_array_4_bit_exact_roundtrip() {
    let original: [f64; 4] = [
        1.123_456_789_f64,
        std::f64::consts::PI,
        f64::INFINITY,
        -0.0_f64,
    ];
    let encoded = encode_to_vec(&original).expect("encode [f64; 4]");

    let (decoded, consumed): ([f64; 4], usize) =
        decode_from_slice(&encoded).expect("decode [f64; 4]");
    assert_eq!(
        consumed,
        encoded.len(),
        "consumed must equal encoded length"
    );

    for (a, b) in original.iter().zip(decoded.iter()) {
        assert_eq!(a.to_bits(), b.to_bits(), "f64 must be bit-exact");
    }
}

// ---------------------------------------------------------------------------
// Test 14: [i32; 8] roundtrip with negative values
// ---------------------------------------------------------------------------
#[test]
fn test_simd_i32_array_8_negative_values_roundtrip() {
    let original: [i32; 8] = [-1, -100, -10_000, -1_000_000, 0, 1, 100, i32::MIN];
    let encoded = encode_to_vec(&original).expect("encode [i32; 8]");

    let (decoded, consumed): ([i32; 8], usize) =
        decode_from_slice(&encoded).expect("decode [i32; 8]");
    assert_eq!(
        decoded, original,
        "[i32; 8] with negatives must roundtrip identically"
    );
    assert_eq!(consumed, encoded.len());
}

// ---------------------------------------------------------------------------
// Test 15: [i64; 4] roundtrip with i64::MIN/MAX
// ---------------------------------------------------------------------------
#[test]
fn test_simd_i64_array_4_min_max_roundtrip() {
    let original: [i64; 4] = [i64::MIN, -1, 1, i64::MAX];
    let encoded = encode_to_vec(&original).expect("encode [i64; 4]");

    let (decoded, consumed): ([i64; 4], usize) =
        decode_from_slice(&encoded).expect("decode [i64; 4]");
    assert_eq!(
        decoded, original,
        "[i64; 4] with MIN/MAX must roundtrip identically"
    );
    assert_eq!(consumed, encoded.len());
}

// ---------------------------------------------------------------------------
// Test 16: [[u8; 4]; 4] nested array roundtrip
// ---------------------------------------------------------------------------
#[test]
fn test_simd_nested_u8_array_4x4_roundtrip() {
    let original: [[u8; 4]; 4] = [
        [0x01, 0x02, 0x03, 0x04],
        [0xDE, 0xAD, 0xBE, 0xEF],
        [0xFF, 0x00, 0xFF, 0x00],
        [0x10, 0x20, 0x30, 0x40],
    ];
    let encoded = encode_to_vec(&original).expect("encode [[u8; 4]; 4]");
    assert_eq!(
        encoded.len(),
        16,
        "[[u8; 4]; 4] must encode to exactly 16 bytes"
    );

    let (decoded, consumed): ([[u8; 4]; 4], usize) =
        decode_from_slice(&encoded).expect("decode [[u8; 4]; 4]");
    assert_eq!(
        decoded, original,
        "[[u8; 4]; 4] roundtrip must be identical"
    );
    assert_eq!(consumed, 16);
}

// ---------------------------------------------------------------------------
// Test 17: Vec<[u8; 16]> roundtrip with 5 items
// ---------------------------------------------------------------------------
#[test]
fn test_simd_vec_of_u8_array_16_roundtrip() {
    let original: Vec<[u8; 16]> = vec![
        [0u8; 16],
        [1u8; 16],
        [0xFF; 16],
        [
            0xAA, 0xBB, 0xCC, 0xDD, 0xEE, 0xFF, 0x00, 0x11, 0x22, 0x33, 0x44, 0x55, 0x66, 0x77,
            0x88, 0x99,
        ],
        [
            0x10, 0x20, 0x30, 0x40, 0x50, 0x60, 0x70, 0x80, 0x90, 0xA0, 0xB0, 0xC0, 0xD0, 0xE0,
            0xF0, 0x01,
        ],
    ];
    let encoded = encode_to_vec(&original).expect("encode Vec<[u8; 16]>");

    let (decoded, consumed): (Vec<[u8; 16]>, usize) =
        decode_from_slice(&encoded).expect("decode Vec<[u8; 16]>");
    assert_eq!(
        decoded, original,
        "Vec<[u8; 16]> roundtrip must be identical"
    );
    assert_eq!(
        consumed,
        encoded.len(),
        "consumed must equal encoded length"
    );
    assert_eq!(decoded.len(), 5, "decoded vec must have 5 items");
}

// ---------------------------------------------------------------------------
// Test 18: Fixed-int config with [u32; 4] (exactly 16 bytes)
// ---------------------------------------------------------------------------
#[test]
fn test_simd_u32_array_4_fixed_int_config_exactly_16_bytes() {
    let original: [u32; 4] = [0x0000_0001, 0x0000_FFFF, 0x00FF_FFFF, 0xFFFF_FFFF];
    let cfg = config::standard().with_fixed_int_encoding();
    let encoded =
        encode_to_vec_with_config(&original, cfg).expect("encode [u32; 4] with fixed-int config");

    assert_eq!(
        encoded.len(),
        16,
        "[u32; 4] with fixed-int encoding must be exactly 16 bytes (4 * 4)"
    );

    let (decoded, consumed): ([u32; 4], usize) = decode_from_slice_with_config(&encoded, cfg)
        .expect("decode [u32; 4] with fixed-int config");
    assert_eq!(
        decoded, original,
        "[u32; 4] fixed-int roundtrip must be identical"
    );
    assert_eq!(consumed, 16);
}

// ---------------------------------------------------------------------------
// Test 19: Big-endian config with [u32; 4]
// ---------------------------------------------------------------------------
#[test]
fn test_simd_u32_array_4_big_endian_config_roundtrip() {
    let original: [u32; 4] = [0x0102_0304, 0xDEAD_BEEF, 0x0000_0000, 0xFFFF_FFFF];
    let be_cfg = config::standard()
        .with_big_endian()
        .with_fixed_int_encoding();
    let encoded = encode_to_vec_with_config(&original, be_cfg).expect("encode [u32; 4] big-endian");

    assert_eq!(encoded.len(), 16, "big-endian [u32; 4] must be 16 bytes");
    // Verify big-endian byte order for the first element 0x01020304
    assert_eq!(
        &encoded[0..4],
        &[0x01, 0x02, 0x03, 0x04],
        "first element must be big-endian"
    );
    // Verify second element 0xDEADBEEF
    assert_eq!(
        &encoded[4..8],
        &[0xDE, 0xAD, 0xBE, 0xEF],
        "second element must be big-endian"
    );

    let (decoded, consumed): ([u32; 4], usize) =
        decode_from_slice_with_config(&encoded, be_cfg).expect("decode [u32; 4] big-endian");
    assert_eq!(
        decoded, original,
        "big-endian [u32; 4] roundtrip must be identical"
    );
    assert_eq!(consumed, 16);

    // Confirm big-endian and little-endian produce different bytes (for non-symmetric value)
    let le_cfg = config::standard().with_fixed_int_encoding();
    let le_encoded =
        encode_to_vec_with_config(&original, le_cfg).expect("encode [u32; 4] little-endian");
    assert_ne!(
        encoded, le_encoded,
        "big-endian and little-endian must differ"
    );
}

// ---------------------------------------------------------------------------
// Test 20: Consumed bytes equals encoded length
// ---------------------------------------------------------------------------
#[test]
fn test_simd_consumed_bytes_equals_encoded_length() {
    let original: [u64; 4] = [1, 22, 333, 4444];
    let encoded = encode_to_vec(&original).expect("encode [u64; 4] for consumed check");

    let (_, consumed): ([u64; 4], usize) =
        decode_from_slice(&encoded).expect("decode [u64; 4] for consumed check");
    assert_eq!(
        consumed,
        encoded.len(),
        "consumed bytes must equal the total encoded length when decoding the full buffer"
    );
}

// ---------------------------------------------------------------------------
// Test 21: Same array value encodes to same bytes both times
// ---------------------------------------------------------------------------
#[test]
fn test_simd_same_array_encodes_deterministically() {
    let original: [u32; 8] = [0, 1, 2, 3, 4, 5, 6, 7];
    let encoded_first = encode_to_vec(&original).expect("encode [u32; 8] first time");
    let encoded_second = encode_to_vec(&original).expect("encode [u32; 8] second time");

    assert_eq!(
        encoded_first, encoded_second,
        "encoding the same array twice must produce identical bytes"
    );
}

// ---------------------------------------------------------------------------
// Test 22: [u8; 256] large array roundtrip
// ---------------------------------------------------------------------------
#[test]
fn test_simd_u8_array_256_roundtrip() {
    let mut original: [u8; 256] = [0u8; 256];
    for (i, b) in original.iter_mut().enumerate() {
        *b = i as u8;
    }
    let encoded = encode_to_vec(&original).expect("encode [u8; 256]");
    assert_eq!(
        encoded.len(),
        256,
        "[u8; 256] must encode to exactly 256 bytes"
    );

    let (decoded, consumed): ([u8; 256], usize) =
        decode_from_slice(&encoded).expect("decode [u8; 256]");
    assert_eq!(decoded, original, "[u8; 256] roundtrip must be identical");
    assert_eq!(consumed, 256, "must consume exactly 256 bytes");
}
