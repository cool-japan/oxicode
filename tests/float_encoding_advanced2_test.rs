//! Advanced float encoding/decoding tests — set 2.
//!
//! Covers: f32/f64 Pi roundtrip, boundary values, IEEE 754 special values,
//! zero sign preservation, fixed-int config, big-endian byte order verification,
//! mixed-value collections, struct derive, and consumed-bytes correctness.

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

// ── 1. f32 Pi roundtrip (bit-exact) ──────────────────────────────────────────

#[test]
fn test_f32_pi_roundtrip() {
    let val: f32 = std::f32::consts::PI;
    let enc = encode_to_vec(&val).expect("encode f32 pi");
    let (dec, _): (f32, usize) = decode_from_slice(&enc).expect("decode f32 pi");
    assert_eq!(
        val.to_bits(),
        dec.to_bits(),
        "f32 PI must be bit-identical after roundtrip"
    );
}

// ── 2. f64 Pi roundtrip (bit-exact) ──────────────────────────────────────────

#[test]
fn test_f64_pi_roundtrip() {
    let val: f64 = std::f64::consts::PI;
    let enc = encode_to_vec(&val).expect("encode f64 pi");
    let (dec, _): (f64, usize) = decode_from_slice(&enc).expect("decode f64 pi");
    assert_eq!(
        val.to_bits(),
        dec.to_bits(),
        "f64 PI must be bit-identical after roundtrip"
    );
}

// ── 3. f32::MAX roundtrip ─────────────────────────────────────────────────────

#[test]
fn test_f32_max_roundtrip() {
    let val = f32::MAX;
    let enc = encode_to_vec(&val).expect("encode f32::MAX");
    assert_eq!(enc.len(), 4, "f32 must encode to exactly 4 bytes");
    let (dec, _): (f32, usize) = decode_from_slice(&enc).expect("decode f32::MAX");
    assert_eq!(
        val.to_bits(),
        dec.to_bits(),
        "f32::MAX must be bit-identical after roundtrip"
    );
    assert!(dec.is_finite(), "f32::MAX must be finite");
}

// ── 4. f32::MIN roundtrip ─────────────────────────────────────────────────────

#[test]
fn test_f32_min_roundtrip() {
    let val = f32::MIN;
    let enc = encode_to_vec(&val).expect("encode f32::MIN");
    assert_eq!(enc.len(), 4, "f32 must encode to exactly 4 bytes");
    let (dec, _): (f32, usize) = decode_from_slice(&enc).expect("decode f32::MIN");
    assert_eq!(
        val.to_bits(),
        dec.to_bits(),
        "f32::MIN must be bit-identical after roundtrip"
    );
    assert!(
        dec.is_sign_negative() && dec.is_finite(),
        "f32::MIN is negative and finite"
    );
}

// ── 5. f32::INFINITY roundtrip ────────────────────────────────────────────────

#[test]
fn test_f32_infinity_roundtrip() {
    let val = f32::INFINITY;
    let enc = encode_to_vec(&val).expect("encode f32::INFINITY");
    let (dec, _): (f32, usize) = decode_from_slice(&enc).expect("decode f32::INFINITY");
    assert_eq!(
        val.to_bits(),
        dec.to_bits(),
        "f32::INFINITY must be bit-identical after roundtrip"
    );
    assert!(dec.is_infinite() && dec.is_sign_positive());
}

// ── 6. f32::NEG_INFINITY roundtrip ───────────────────────────────────────────

#[test]
fn test_f32_neg_infinity_roundtrip() {
    let val = f32::NEG_INFINITY;
    let enc = encode_to_vec(&val).expect("encode f32::NEG_INFINITY");
    let (dec, _): (f32, usize) = decode_from_slice(&enc).expect("decode f32::NEG_INFINITY");
    assert_eq!(
        val.to_bits(),
        dec.to_bits(),
        "f32::NEG_INFINITY must be bit-identical after roundtrip"
    );
    assert!(dec.is_infinite() && dec.is_sign_negative());
}

// ── 7. f32::NAN roundtrip (bit-exact via to_bits) ────────────────────────────

#[test]
fn test_f32_nan_roundtrip_bit_exact() {
    let val = f32::NAN;
    let enc = encode_to_vec(&val).expect("encode f32::NAN");
    assert_eq!(enc.len(), 4, "f32::NAN must encode to exactly 4 bytes");
    let (dec, _): (f32, usize) = decode_from_slice(&enc).expect("decode f32::NAN");
    assert!(dec.is_nan(), "decoded value must still be NaN");
    assert_eq!(
        val.to_bits(),
        dec.to_bits(),
        "f32::NAN bit pattern (including payload) must be preserved"
    );
}

// ── 8. f64::MAX roundtrip ─────────────────────────────────────────────────────

#[test]
fn test_f64_max_roundtrip() {
    let val = f64::MAX;
    let enc = encode_to_vec(&val).expect("encode f64::MAX");
    assert_eq!(enc.len(), 8, "f64 must encode to exactly 8 bytes");
    let (dec, _): (f64, usize) = decode_from_slice(&enc).expect("decode f64::MAX");
    assert_eq!(
        val.to_bits(),
        dec.to_bits(),
        "f64::MAX must be bit-identical after roundtrip"
    );
    assert!(dec.is_finite(), "f64::MAX must be finite");
}

// ── 9. f64::MIN roundtrip ─────────────────────────────────────────────────────

#[test]
fn test_f64_min_roundtrip() {
    let val = f64::MIN;
    let enc = encode_to_vec(&val).expect("encode f64::MIN");
    assert_eq!(enc.len(), 8, "f64 must encode to exactly 8 bytes");
    let (dec, _): (f64, usize) = decode_from_slice(&enc).expect("decode f64::MIN");
    assert_eq!(
        val.to_bits(),
        dec.to_bits(),
        "f64::MIN must be bit-identical after roundtrip"
    );
    assert!(
        dec.is_sign_negative() && dec.is_finite(),
        "f64::MIN is negative and finite"
    );
}

// ── 10. f64::INFINITY roundtrip ───────────────────────────────────────────────

#[test]
fn test_f64_infinity_roundtrip() {
    let val = f64::INFINITY;
    let enc = encode_to_vec(&val).expect("encode f64::INFINITY");
    let (dec, _): (f64, usize) = decode_from_slice(&enc).expect("decode f64::INFINITY");
    assert_eq!(
        val.to_bits(),
        dec.to_bits(),
        "f64::INFINITY must be bit-identical after roundtrip"
    );
    assert!(dec.is_infinite() && dec.is_sign_positive());
}

// ── 11. f64::NEG_INFINITY roundtrip ──────────────────────────────────────────

#[test]
fn test_f64_neg_infinity_roundtrip() {
    let val = f64::NEG_INFINITY;
    let enc = encode_to_vec(&val).expect("encode f64::NEG_INFINITY");
    let (dec, _): (f64, usize) = decode_from_slice(&enc).expect("decode f64::NEG_INFINITY");
    assert_eq!(
        val.to_bits(),
        dec.to_bits(),
        "f64::NEG_INFINITY must be bit-identical after roundtrip"
    );
    assert!(dec.is_infinite() && dec.is_sign_negative());
}

// ── 12. f64::NAN roundtrip (bit-exact) ────────────────────────────────────────

#[test]
fn test_f64_nan_roundtrip_bit_exact() {
    let val = f64::NAN;
    let enc = encode_to_vec(&val).expect("encode f64::NAN");
    assert_eq!(enc.len(), 8, "f64::NAN must encode to exactly 8 bytes");
    let (dec, _): (f64, usize) = decode_from_slice(&enc).expect("decode f64::NAN");
    assert!(dec.is_nan(), "decoded value must still be NaN");
    assert_eq!(
        val.to_bits(),
        dec.to_bits(),
        "f64::NAN bit pattern (including payload) must be preserved"
    );
}

// ── 13. f32 positive zero (+0.0) roundtrip ───────────────────────────────────

#[test]
fn test_f32_positive_zero_roundtrip() {
    let val = 0.0f32;
    let enc = encode_to_vec(&val).expect("encode f32 +0.0");
    assert_eq!(enc.len(), 4, "f32 +0.0 must encode to exactly 4 bytes");
    // +0.0f32 in little-endian IEEE 754 is all zero bytes
    assert_eq!(
        enc,
        vec![0x00, 0x00, 0x00, 0x00],
        "+0.0f32 must encode as four zero bytes"
    );
    let (dec, _): (f32, usize) = decode_from_slice(&enc).expect("decode f32 +0.0");
    assert_eq!(
        val.to_bits(),
        dec.to_bits(),
        "f32 +0.0 bit pattern must be preserved (sign bit = 0)"
    );
    assert!(
        dec.is_sign_positive(),
        "decoded +0.0f32 must have positive sign"
    );
}

// ── 14. f64 negative zero (-0.0) roundtrip (bit-exact) ───────────────────────

#[test]
fn test_f64_negative_zero_roundtrip_bit_exact() {
    let val = -0.0f64;
    let enc = encode_to_vec(&val).expect("encode f64 -0.0");
    assert_eq!(enc.len(), 8, "f64 -0.0 must encode to exactly 8 bytes");
    let (dec, _): (f64, usize) = decode_from_slice(&enc).expect("decode f64 -0.0");
    assert_eq!(
        val.to_bits(),
        dec.to_bits(),
        "f64 -0.0 bit pattern (sign bit = 1) must be preserved after roundtrip"
    );
    assert!(
        dec.is_sign_negative(),
        "decoded -0.0f64 must have negative sign"
    );
    // Arithmetic equality: -0.0 == +0.0 per IEEE 754
    assert_eq!(
        dec, 0.0f64,
        "-0.0f64 must be arithmetically equal to +0.0f64"
    );
    // But bit patterns differ
    assert_ne!(
        (-0.0f64).to_bits(),
        (0.0f64).to_bits(),
        "-0.0f64 and +0.0f64 must have different bit patterns"
    );
}

// ── 15. f32 with fixed_int_encoding config (still 4 bytes) ───────────────────

#[test]
fn test_f32_fixed_int_encoding_config_stays_4_bytes() {
    let val = std::f32::consts::E;
    let cfg = config::standard().with_fixed_int_encoding();

    let enc = encode_to_vec_with_config(&val, cfg).expect("encode f32 with fixed_int_encoding");
    assert_eq!(
        enc.len(),
        4,
        "f32 must remain 4 bytes even with fixed_int_encoding config"
    );

    // Compare against default config — floats must always be stored at full width
    let enc_std = encode_to_vec(&val).expect("encode f32 standard config");
    assert_eq!(
        enc, enc_std,
        "fixed_int_encoding must not alter the raw IEEE 754 byte representation of f32"
    );

    let (dec, consumed): (f32, usize) =
        decode_from_slice_with_config(&enc, cfg).expect("decode f32 with fixed_int_encoding");
    assert_eq!(consumed, 4, "exactly 4 bytes must be consumed for f32");
    assert_eq!(
        val.to_bits(),
        dec.to_bits(),
        "f32 must be bit-identical after fixed_int_encoding roundtrip"
    );
}

// ── 16. f64 with fixed_int_encoding config (still 8 bytes) ───────────────────

#[test]
fn test_f64_fixed_int_encoding_config_stays_8_bytes() {
    let val = std::f64::consts::LN_2;
    let cfg = config::standard().with_fixed_int_encoding();

    let enc = encode_to_vec_with_config(&val, cfg).expect("encode f64 with fixed_int_encoding");
    assert_eq!(
        enc.len(),
        8,
        "f64 must remain 8 bytes even with fixed_int_encoding config"
    );

    let enc_std = encode_to_vec(&val).expect("encode f64 standard config");
    assert_eq!(
        enc, enc_std,
        "fixed_int_encoding must not alter the raw IEEE 754 byte representation of f64"
    );

    let (dec, consumed): (f64, usize) =
        decode_from_slice_with_config(&enc, cfg).expect("decode f64 with fixed_int_encoding");
    assert_eq!(consumed, 8, "exactly 8 bytes must be consumed for f64");
    assert_eq!(
        val.to_bits(),
        dec.to_bits(),
        "f64 must be bit-identical after fixed_int_encoding roundtrip"
    );
}

// ── 17. f32 big_endian config: check byte order ───────────────────────────────

#[test]
fn test_f32_big_endian_config_byte_order() {
    let cfg = config::standard().with_big_endian();

    // Use a value with a well-known IEEE 754 big-endian representation.
    // 2.0f32 = 0x40000000 → BE bytes: [0x40, 0x00, 0x00, 0x00]
    let val = 2.0f32;
    let enc = encode_to_vec_with_config(&val, cfg).expect("encode f32 2.0 big-endian");
    assert_eq!(enc.len(), 4, "f32 must be 4 bytes in big-endian config");
    let expected_be = val.to_bits().to_be_bytes();
    assert_eq!(
        enc.as_slice(),
        expected_be.as_slice(),
        "encoded bytes must match big-endian IEEE 754 layout for f32 2.0"
    );
    // Verify it differs from little-endian
    let enc_le = encode_to_vec(&val).expect("encode f32 2.0 little-endian");
    let expected_le = val.to_bits().to_le_bytes();
    assert_eq!(
        enc_le.as_slice(),
        expected_le.as_slice(),
        "standard config is little-endian"
    );
    // Big-endian and little-endian must differ for 2.0f32 (bytes are not palindromic)
    assert_ne!(
        enc, enc_le,
        "big-endian and little-endian encodings of 2.0f32 must differ"
    );

    // Full roundtrip
    let (dec, _): (f32, usize) =
        decode_from_slice_with_config(&enc, cfg).expect("decode f32 2.0 big-endian");
    assert_eq!(
        dec.to_bits(),
        val.to_bits(),
        "f32 must be bit-identical after big-endian roundtrip"
    );
}

// ── 18. f64 big_endian config: check byte order ───────────────────────────────

#[test]
fn test_f64_big_endian_config_byte_order() {
    let cfg = config::standard().with_big_endian();

    // 1.0f64 = 0x3FF0000000000000 → BE bytes: [0x3F, 0xF0, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00]
    let val = 1.0f64;
    let enc = encode_to_vec_with_config(&val, cfg).expect("encode f64 1.0 big-endian");
    assert_eq!(enc.len(), 8, "f64 must be 8 bytes in big-endian config");
    let expected_be = val.to_bits().to_be_bytes();
    assert_eq!(
        enc.as_slice(),
        expected_be.as_slice(),
        "encoded bytes must match big-endian IEEE 754 layout for f64 1.0"
    );
    assert_eq!(
        enc,
        vec![0x3F, 0xF0, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00],
        "f64 1.0 big-endian must be [0x3F, 0xF0, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00]"
    );

    // Full roundtrip
    let (dec, _): (f64, usize) =
        decode_from_slice_with_config(&enc, cfg).expect("decode f64 1.0 big-endian");
    assert_eq!(
        dec.to_bits(),
        val.to_bits(),
        "f64 must be bit-identical after big-endian roundtrip"
    );
}

// ── 19. Vec<f32> mixed values roundtrip ──────────────────────────────────────

#[test]
fn test_vec_f32_mixed_values_roundtrip() {
    // Subnormal: smallest positive subnormal f32 has bits = 1
    let subnormal = f32::from_bits(1u32);
    let vals: Vec<f32> = vec![
        f32::INFINITY,
        f32::NEG_INFINITY,
        f32::NAN,
        f32::MAX,
        f32::MIN,
        f32::MIN_POSITIVE,
        subnormal,
        0.0f32,
        -0.0f32,
        std::f32::consts::PI,
        std::f32::consts::E,
        std::f32::consts::SQRT_2,
        f32::EPSILON,
        -f32::EPSILON,
        1.0f32,
        -1.0f32,
    ];

    let enc = encode_to_vec(&vals).expect("encode Vec<f32> mixed");
    let (dec, consumed): (Vec<f32>, usize) =
        decode_from_slice(&enc).expect("decode Vec<f32> mixed");

    assert_eq!(consumed, enc.len(), "all encoded bytes must be consumed");
    assert_eq!(
        dec.len(),
        vals.len(),
        "decoded Vec<f32> length must match original"
    );

    for (i, (orig, got)) in vals.iter().zip(dec.iter()).enumerate() {
        assert_eq!(
            orig.to_bits(),
            got.to_bits(),
            "Vec<f32> element {} bit pattern mismatch: orig bits={:#010x}, got bits={:#010x}",
            i,
            orig.to_bits(),
            got.to_bits()
        );
    }
}

// ── 20. Vec<f64> mixed values roundtrip ──────────────────────────────────────

#[test]
fn test_vec_f64_mixed_values_roundtrip() {
    // NaN with a custom payload to verify payload preservation
    let custom_nan = f64::from_bits(0x7FF8_CAFE_BABE_0000_u64);
    // Subnormal: smallest positive subnormal f64
    let subnormal = f64::from_bits(1u64);
    // Largest subnormal f64
    let largest_subnormal = f64::from_bits(0x000F_FFFF_FFFF_FFFF_u64);

    let vals: Vec<f64> = vec![
        f64::INFINITY,
        f64::NEG_INFINITY,
        f64::NAN,
        custom_nan,
        f64::MAX,
        f64::MIN,
        f64::MIN_POSITIVE,
        subnormal,
        largest_subnormal,
        0.0f64,
        -0.0f64,
        std::f64::consts::PI,
        std::f64::consts::E,
        std::f64::consts::SQRT_2,
        std::f64::consts::LN_2,
        f64::EPSILON,
        -f64::EPSILON,
        1.0f64,
        -1.0f64,
    ];

    let enc = encode_to_vec(&vals).expect("encode Vec<f64> mixed");
    let (dec, consumed): (Vec<f64>, usize) =
        decode_from_slice(&enc).expect("decode Vec<f64> mixed");

    assert_eq!(consumed, enc.len(), "all encoded bytes must be consumed");
    assert_eq!(
        dec.len(),
        vals.len(),
        "decoded Vec<f64> length must match original"
    );

    for (i, (orig, got)) in vals.iter().zip(dec.iter()).enumerate() {
        assert_eq!(
            orig.to_bits(),
            got.to_bits(),
            "Vec<f64> element {} bit pattern mismatch: orig bits={:#018x}, got bits={:#018x}",
            i,
            orig.to_bits(),
            got.to_bits()
        );
    }
}

// ── 21. Struct containing f32 and f64 fields roundtrip ───────────────────────

#[derive(Debug, Encode, Decode)]
struct MixedFloatPoint {
    index: u32,
    x: f32,
    y: f32,
    z: f64,
    weight: f64,
    flags: u8,
}

#[test]
fn test_struct_mixed_f32_f64_fields_roundtrip() {
    let original = MixedFloatPoint {
        index: 7,
        x: std::f32::consts::PI,
        y: f32::NEG_INFINITY,
        z: std::f64::consts::E,
        weight: f64::NAN,
        flags: 0b1010_1010,
    };

    let enc = encode_to_vec(&original).expect("encode MixedFloatPoint");
    // Expected size: u32 (varint, value 7 → 1 byte) + f32 (4) + f32 (4) + f64 (8) + f64 (8) + u8 (1) = 26 bytes
    // (varint for u32 value 7 is 1 byte in standard config)
    assert!(enc.len() > 0, "encoded MixedFloatPoint must be non-empty");

    let (dec, consumed): (MixedFloatPoint, usize) =
        decode_from_slice(&enc).expect("decode MixedFloatPoint");

    assert_eq!(
        consumed,
        enc.len(),
        "all encoded bytes must be consumed for MixedFloatPoint"
    );
    assert_eq!(dec.index, original.index, "index field must roundtrip");
    assert_eq!(
        dec.x.to_bits(),
        original.x.to_bits(),
        "x (f32 PI) must be bit-identical after roundtrip"
    );
    assert_eq!(
        dec.y.to_bits(),
        original.y.to_bits(),
        "y (f32 NEG_INFINITY) must be bit-identical after roundtrip"
    );
    assert_eq!(
        dec.z.to_bits(),
        original.z.to_bits(),
        "z (f64 E) must be bit-identical after roundtrip"
    );
    assert!(dec.weight.is_nan(), "weight field must decode as NaN");
    assert_eq!(
        dec.weight.to_bits(),
        original.weight.to_bits(),
        "weight (f64 NAN) bit pattern must be preserved"
    );
    assert_eq!(dec.flags, original.flags, "flags field must roundtrip");
}

// ── 22. Consumed bytes == encoded length for f64 ─────────────────────────────

#[test]
fn test_f64_consumed_bytes_equals_encoded_length() {
    // Verify this invariant holds for a representative set of f64 values,
    // including specials, boundary values, and mathematical constants.
    let test_values: &[f64] = &[
        f64::INFINITY,
        f64::NEG_INFINITY,
        f64::NAN,
        f64::MAX,
        f64::MIN,
        f64::MIN_POSITIVE,
        0.0f64,
        -0.0f64,
        1.0f64,
        -1.0f64,
        std::f64::consts::PI,
        std::f64::consts::E,
        f64::EPSILON,
        // Subnormal
        f64::from_bits(0x0000_0000_0000_0001_u64),
        // Quiet NaN with arbitrary payload
        f64::from_bits(0x7FF8_DEAD_BEEF_1234_u64),
    ];

    for &val in test_values {
        let enc = encode_to_vec(&val).expect("encode f64 for consumed-bytes test");
        assert_eq!(
            enc.len(),
            8,
            "f64 must always encode to exactly 8 bytes; bits={:#018x}",
            val.to_bits()
        );

        let (dec, consumed): (f64, usize) =
            decode_from_slice(&enc).expect("decode f64 for consumed-bytes test");

        assert_eq!(
            consumed,
            enc.len(),
            "consumed bytes ({}) must equal encoded length ({}) for f64 bits={:#018x}",
            consumed,
            enc.len(),
            val.to_bits()
        );
        assert_eq!(
            dec.to_bits(),
            val.to_bits(),
            "decoded f64 must be bit-identical to original; bits={:#018x}",
            val.to_bits()
        );
    }
}
