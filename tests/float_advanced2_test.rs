//! Advanced float encoding/decoding tests — bit-exact roundtrips for IEEE 754 special values,
//! encoding size guarantees, endianness verification, and struct encoding with float fields.

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
// 1. f32::INFINITY roundtrip (bit-exact)
// ---------------------------------------------------------------------------

#[test]
fn test_f32_infinity_bit_exact() {
    let v = f32::INFINITY;
    let enc = encode_to_vec(&v).expect("encode f32::INFINITY");
    let (dec, _): (f32, usize) = decode_from_slice(&enc).expect("decode f32::INFINITY");
    assert_eq!(dec.to_bits(), v.to_bits(), "bit-exact float roundtrip");
    assert!(dec.is_infinite() && dec.is_sign_positive());
}

// ---------------------------------------------------------------------------
// 2. f32::NEG_INFINITY roundtrip (bit-exact)
// ---------------------------------------------------------------------------

#[test]
fn test_f32_neg_infinity_bit_exact() {
    let v = f32::NEG_INFINITY;
    let enc = encode_to_vec(&v).expect("encode f32::NEG_INFINITY");
    let (dec, _): (f32, usize) = decode_from_slice(&enc).expect("decode f32::NEG_INFINITY");
    assert_eq!(dec.to_bits(), v.to_bits(), "bit-exact float roundtrip");
    assert!(dec.is_infinite() && dec.is_sign_negative());
}

// ---------------------------------------------------------------------------
// 3. f32::NAN roundtrip (bit-exact using to_bits)
// ---------------------------------------------------------------------------

#[test]
fn test_f32_nan_bit_exact() {
    let v = f32::NAN;
    let enc = encode_to_vec(&v).expect("encode f32::NAN");
    let (dec, _): (f32, usize) = decode_from_slice(&enc).expect("decode f32::NAN");
    assert_eq!(dec.to_bits(), v.to_bits(), "bit-exact float roundtrip");
    assert!(dec.is_nan());
}

// ---------------------------------------------------------------------------
// 4. f32::MIN_POSITIVE roundtrip (bit-exact)
// ---------------------------------------------------------------------------

#[test]
fn test_f32_min_positive_bit_exact() {
    let v = f32::MIN_POSITIVE;
    let enc = encode_to_vec(&v).expect("encode f32::MIN_POSITIVE");
    let (dec, _): (f32, usize) = decode_from_slice(&enc).expect("decode f32::MIN_POSITIVE");
    assert_eq!(dec.to_bits(), v.to_bits(), "bit-exact float roundtrip");
}

// ---------------------------------------------------------------------------
// 5. f32::MAX roundtrip (bit-exact)
// ---------------------------------------------------------------------------

#[test]
fn test_f32_max_bit_exact() {
    let v = f32::MAX;
    let enc = encode_to_vec(&v).expect("encode f32::MAX");
    let (dec, _): (f32, usize) = decode_from_slice(&enc).expect("decode f32::MAX");
    assert_eq!(dec.to_bits(), v.to_bits(), "bit-exact float roundtrip");
}

// ---------------------------------------------------------------------------
// 6. f32::MIN roundtrip (bit-exact, most negative finite)
// ---------------------------------------------------------------------------

#[test]
fn test_f32_min_bit_exact() {
    let v = f32::MIN;
    let enc = encode_to_vec(&v).expect("encode f32::MIN");
    let (dec, _): (f32, usize) = decode_from_slice(&enc).expect("decode f32::MIN");
    assert_eq!(dec.to_bits(), v.to_bits(), "bit-exact float roundtrip");
    assert!(dec.is_finite() && dec.is_sign_negative());
}

// ---------------------------------------------------------------------------
// 7. f32::EPSILON roundtrip (bit-exact)
// ---------------------------------------------------------------------------

#[test]
fn test_f32_epsilon_bit_exact() {
    let v = f32::EPSILON;
    let enc = encode_to_vec(&v).expect("encode f32::EPSILON");
    let (dec, _): (f32, usize) = decode_from_slice(&enc).expect("decode f32::EPSILON");
    assert_eq!(dec.to_bits(), v.to_bits(), "bit-exact float roundtrip");
}

// ---------------------------------------------------------------------------
// 8. f64::INFINITY roundtrip (bit-exact)
// ---------------------------------------------------------------------------

#[test]
fn test_f64_infinity_bit_exact() {
    let v = f64::INFINITY;
    let enc = encode_to_vec(&v).expect("encode f64::INFINITY");
    let (dec, _): (f64, usize) = decode_from_slice(&enc).expect("decode f64::INFINITY");
    assert_eq!(dec.to_bits(), v.to_bits(), "bit-exact float roundtrip");
    assert!(dec.is_infinite() && dec.is_sign_positive());
}

// ---------------------------------------------------------------------------
// 9. f64::NEG_INFINITY roundtrip (bit-exact)
// ---------------------------------------------------------------------------

#[test]
fn test_f64_neg_infinity_bit_exact() {
    let v = f64::NEG_INFINITY;
    let enc = encode_to_vec(&v).expect("encode f64::NEG_INFINITY");
    let (dec, _): (f64, usize) = decode_from_slice(&enc).expect("decode f64::NEG_INFINITY");
    assert_eq!(dec.to_bits(), v.to_bits(), "bit-exact float roundtrip");
    assert!(dec.is_infinite() && dec.is_sign_negative());
}

// ---------------------------------------------------------------------------
// 10. f64::NAN roundtrip (bit-exact)
// ---------------------------------------------------------------------------

#[test]
fn test_f64_nan_bit_exact() {
    let v = f64::NAN;
    let enc = encode_to_vec(&v).expect("encode f64::NAN");
    let (dec, _): (f64, usize) = decode_from_slice(&enc).expect("decode f64::NAN");
    assert_eq!(dec.to_bits(), v.to_bits(), "bit-exact float roundtrip");
    assert!(dec.is_nan());
}

// ---------------------------------------------------------------------------
// 11. f64::MIN_POSITIVE roundtrip (bit-exact)
// ---------------------------------------------------------------------------

#[test]
fn test_f64_min_positive_bit_exact() {
    let v = f64::MIN_POSITIVE;
    let enc = encode_to_vec(&v).expect("encode f64::MIN_POSITIVE");
    let (dec, _): (f64, usize) = decode_from_slice(&enc).expect("decode f64::MIN_POSITIVE");
    assert_eq!(dec.to_bits(), v.to_bits(), "bit-exact float roundtrip");
}

// ---------------------------------------------------------------------------
// 12. f64::MAX roundtrip (bit-exact)
// ---------------------------------------------------------------------------

#[test]
fn test_f64_max_bit_exact() {
    let v = f64::MAX;
    let enc = encode_to_vec(&v).expect("encode f64::MAX");
    let (dec, _): (f64, usize) = decode_from_slice(&enc).expect("decode f64::MAX");
    assert_eq!(dec.to_bits(), v.to_bits(), "bit-exact float roundtrip");
}

// ---------------------------------------------------------------------------
// 13. f64::MIN roundtrip (bit-exact)
// ---------------------------------------------------------------------------

#[test]
fn test_f64_min_bit_exact() {
    let v = f64::MIN;
    let enc = encode_to_vec(&v).expect("encode f64::MIN");
    let (dec, _): (f64, usize) = decode_from_slice(&enc).expect("decode f64::MIN");
    assert_eq!(dec.to_bits(), v.to_bits(), "bit-exact float roundtrip");
    assert!(dec.is_finite() && dec.is_sign_negative());
}

// ---------------------------------------------------------------------------
// 14. f64::EPSILON roundtrip (bit-exact)
// ---------------------------------------------------------------------------

#[test]
fn test_f64_epsilon_bit_exact() {
    let v = f64::EPSILON;
    let enc = encode_to_vec(&v).expect("encode f64::EPSILON");
    let (dec, _): (f64, usize) = decode_from_slice(&enc).expect("decode f64::EPSILON");
    assert_eq!(dec.to_bits(), v.to_bits(), "bit-exact float roundtrip");
}

// ---------------------------------------------------------------------------
// 15. f32 encodes as exactly 4 bytes (IEEE 754 single)
// ---------------------------------------------------------------------------

#[test]
fn test_f32_encodes_as_4_bytes() {
    let values = [
        0.0f32,
        1.0f32,
        -1.0f32,
        f32::INFINITY,
        f32::NEG_INFINITY,
        f32::NAN,
        f32::MAX,
        f32::MIN,
        f32::MIN_POSITIVE,
        f32::EPSILON,
    ];
    for v in values {
        let enc = encode_to_vec(&v).expect("encode f32");
        assert_eq!(
            enc.len(),
            4,
            "f32 must encode to exactly 4 bytes (IEEE 754 single); value bits={:#010x}",
            v.to_bits()
        );
    }
}

// ---------------------------------------------------------------------------
// 16. f64 encodes as exactly 8 bytes (IEEE 754 double)
// ---------------------------------------------------------------------------

#[test]
fn test_f64_encodes_as_8_bytes() {
    let values = [
        0.0f64,
        1.0f64,
        -1.0f64,
        f64::INFINITY,
        f64::NEG_INFINITY,
        f64::NAN,
        f64::MAX,
        f64::MIN,
        f64::MIN_POSITIVE,
        f64::EPSILON,
    ];
    for v in values {
        let enc = encode_to_vec(&v).expect("encode f64");
        assert_eq!(
            enc.len(),
            8,
            "f64 must encode to exactly 8 bytes (IEEE 754 double); value bits={:#018x}",
            v.to_bits()
        );
    }
}

// ---------------------------------------------------------------------------
// 17. Fixed-int config doesn't change f32 encoding (floats always full width)
// ---------------------------------------------------------------------------

#[test]
fn test_fixed_int_config_does_not_change_f32_encoding() {
    let v = 1.25f32;
    let std_enc = encode_to_vec(&v).expect("encode f32 standard");
    let fixed_enc = encode_to_vec_with_config(&v, config::standard().with_fixed_int_encoding())
        .expect("encode f32 fixed-int");

    // Both configs must produce the same 4-byte IEEE 754 representation
    // because floats are always stored at full width regardless of int encoding.
    assert_eq!(
        std_enc, fixed_enc,
        "fixed-int config must not alter f32 encoding"
    );
    assert_eq!(
        fixed_enc.len(),
        4,
        "f32 must remain 4 bytes with fixed-int config"
    );

    // Roundtrip with fixed-int config
    let cfg = config::standard().with_fixed_int_encoding();
    let (dec, _): (f32, usize) =
        decode_from_slice_with_config(&fixed_enc, cfg).expect("decode f32 fixed-int");
    assert_eq!(dec.to_bits(), v.to_bits(), "bit-exact float roundtrip");
}

// ---------------------------------------------------------------------------
// 18. Big-endian config with f32: verify byte order matches IEEE 754 big-endian
// ---------------------------------------------------------------------------

#[test]
fn test_f32_big_endian_byte_order() {
    let cfg = config::standard().with_big_endian();
    let enc = encode_to_vec_with_config(&1.0f32, cfg).expect("encode f32 BE");
    // IEEE 754 1.0f32 = 0x3F800000, in big-endian: [0x3F, 0x80, 0x00, 0x00]
    assert_eq!(enc, vec![0x3F, 0x80, 0x00, 0x00]);

    // Verify roundtrip
    let (dec, _): (f32, usize) = decode_from_slice_with_config(&enc, cfg).expect("decode f32 BE");
    assert_eq!(dec.to_bits(), 1.0f32.to_bits(), "bit-exact float roundtrip");

    // Also verify -1.0f32 = 0xBF800000 → [0xBF, 0x80, 0x00, 0x00]
    let enc_neg = encode_to_vec_with_config(&(-1.0f32), cfg).expect("encode -1.0f32 BE");
    assert_eq!(enc_neg, vec![0xBF, 0x80, 0x00, 0x00]);
}

// ---------------------------------------------------------------------------
// 19. Vec<f64> with PI, E, sqrt(2), phi roundtrip (bit-exact)
// ---------------------------------------------------------------------------

#[test]
fn test_vec_f64_mathematical_constants_roundtrip() {
    use std::f64::consts::{E, FRAC_1_SQRT_2, PI, SQRT_2};
    // phi (golden ratio) is not in std::f64::consts, compute manually.
    let phi: f64 = (1.0 + 5.0f64.sqrt()) / 2.0;
    let values: Vec<f64> = vec![PI, E, SQRT_2, FRAC_1_SQRT_2, phi];

    let enc = encode_to_vec(&values).expect("encode Vec<f64> constants");
    let (dec, _): (Vec<f64>, usize) = decode_from_slice(&enc).expect("decode Vec<f64> constants");

    assert_eq!(dec.len(), values.len(), "decoded Vec length must match");
    for (orig, decoded_val) in values.iter().zip(dec.iter()) {
        assert_eq!(
            decoded_val.to_bits(),
            orig.to_bits(),
            "bit-exact float roundtrip for value {}",
            orig
        );
    }
}

// ---------------------------------------------------------------------------
// 20. f32(0.0) and f32(-0.0) both roundtrip, may have different bits
// ---------------------------------------------------------------------------

#[test]
fn test_f32_positive_and_negative_zero_roundtrip() {
    let pos_zero = 0.0f32;
    let neg_zero = -0.0f32;

    // Both should roundtrip with their original bit pattern preserved.
    let enc_pos = encode_to_vec(&pos_zero).expect("encode 0.0f32");
    let (dec_pos, _): (f32, usize) = decode_from_slice(&enc_pos).expect("decode 0.0f32");
    assert_eq!(
        dec_pos.to_bits(),
        pos_zero.to_bits(),
        "bit-exact roundtrip for +0.0f32"
    );

    let enc_neg = encode_to_vec(&neg_zero).expect("encode -0.0f32");
    let (dec_neg, _): (f32, usize) = decode_from_slice(&enc_neg).expect("decode -0.0f32");
    assert_eq!(
        dec_neg.to_bits(),
        neg_zero.to_bits(),
        "bit-exact roundtrip for -0.0f32"
    );

    // +0.0 and -0.0 compare as equal by IEEE 754 arithmetic equality
    // but have distinct bit patterns.
    assert_eq!(pos_zero, neg_zero, "+0.0 == -0.0 by IEEE 754");
    assert_ne!(
        pos_zero.to_bits(),
        neg_zero.to_bits(),
        "+0.0 and -0.0 have different bit patterns"
    );
    assert_ne!(
        enc_pos, enc_neg,
        "+0.0f32 and -0.0f32 must produce different encodings"
    );
}

// ---------------------------------------------------------------------------
// 21. All 22 special f64 values: positive/negative NaN, inf, 0, subnormal roundtrip
// ---------------------------------------------------------------------------

#[test]
fn test_f64_22_special_values_roundtrip() {
    // Construct a diverse set of special and boundary f64 values.
    // Subnormal: the smallest positive subnormal is f64::from_bits(1).
    let subnormal_min = f64::from_bits(1u64);
    // A custom NaN with a specific payload to test payload preservation.
    let nan_payload: f64 = f64::from_bits(0x7FF8_0000_0000_DEAD_u64);

    let specials: &[f64] = &[
        f64::INFINITY,
        f64::NEG_INFINITY,
        f64::NAN,
        nan_payload,
        0.0f64,
        -0.0f64,
        subnormal_min,
        -subnormal_min,
        f64::MIN_POSITIVE,
        -f64::MIN_POSITIVE,
        f64::MAX,
        f64::MIN,
        f64::EPSILON,
        -f64::EPSILON,
        1.0f64,
        -1.0f64,
        // Smallest normal that rounds differently on some platforms
        f64::from_bits(0x0010_0000_0000_0000_u64),
        // Largest subnormal
        f64::from_bits(0x000F_FFFF_FFFF_FFFF_u64),
        // Quiet NaN with all payload bits set
        f64::from_bits(0x7FFF_FFFF_FFFF_FFFF_u64),
        // Signaling NaN representation (may be quieted by hardware)
        f64::from_bits(0x7FF0_0000_0000_0001_u64),
        // A value near 1.0 ULP above 1.0
        f64::from_bits(1.0f64.to_bits() + 1),
        // A value near 1.0 ULP below 1.0
        f64::from_bits(1.0f64.to_bits() - 1),
    ];

    assert_eq!(specials.len(), 22, "must have exactly 22 special values");

    for &v in specials {
        let enc = encode_to_vec(&v).expect("encode special f64");
        assert_eq!(enc.len(), 8, "f64 must encode to exactly 8 bytes");
        let (dec, _): (f64, usize) = decode_from_slice(&enc).expect("decode special f64");
        assert_eq!(
            dec.to_bits(),
            v.to_bits(),
            "bit-exact float roundtrip for bits={:#018x}",
            v.to_bits()
        );
    }
}

// ---------------------------------------------------------------------------
// 22. Struct with f32 and f64 fields roundtrip
// ---------------------------------------------------------------------------

use oxicode::{Decode, Encode};

#[derive(Debug, PartialEq, Encode, Decode)]
struct FloatRecord {
    id: u32,
    x: f32,
    y: f32,
    z: f64,
    magnitude: f64,
}

#[test]
fn test_struct_with_f32_and_f64_fields_roundtrip() {
    let original = FloatRecord {
        id: 42,
        x: 1.5f32,
        y: -3.14f32,
        z: std::f64::consts::PI,
        magnitude: f64::INFINITY,
    };

    let enc = encode_to_vec(&original).expect("encode FloatRecord");
    let (dec, consumed): (FloatRecord, usize) =
        decode_from_slice(&enc).expect("decode FloatRecord");

    assert_eq!(consumed, enc.len(), "all bytes must be consumed");
    assert_eq!(dec.id, original.id, "id field must roundtrip");
    assert_eq!(
        dec.x.to_bits(),
        original.x.to_bits(),
        "bit-exact f32 x field roundtrip"
    );
    assert_eq!(
        dec.y.to_bits(),
        original.y.to_bits(),
        "bit-exact f32 y field roundtrip"
    );
    assert_eq!(
        dec.z.to_bits(),
        original.z.to_bits(),
        "bit-exact f64 z field roundtrip"
    );
    assert_eq!(
        dec.magnitude.to_bits(),
        original.magnitude.to_bits(),
        "bit-exact f64 magnitude field roundtrip"
    );

    // Also verify with NaN fields
    let nan_record = FloatRecord {
        id: 99,
        x: f32::NAN,
        y: f32::MIN,
        z: f64::NAN,
        magnitude: f64::MIN_POSITIVE,
    };

    let enc_nan = encode_to_vec(&nan_record).expect("encode FloatRecord with NaN");
    let (dec_nan, _): (FloatRecord, usize) =
        decode_from_slice(&enc_nan).expect("decode FloatRecord with NaN");

    assert_eq!(dec_nan.id, nan_record.id);
    assert_eq!(
        dec_nan.x.to_bits(),
        nan_record.x.to_bits(),
        "bit-exact f32 NaN roundtrip"
    );
    assert_eq!(
        dec_nan.y.to_bits(),
        nan_record.y.to_bits(),
        "bit-exact f32 MIN roundtrip"
    );
    assert_eq!(
        dec_nan.z.to_bits(),
        nan_record.z.to_bits(),
        "bit-exact f64 NaN roundtrip"
    );
    assert_eq!(
        dec_nan.magnitude.to_bits(),
        nan_record.magnitude.to_bits(),
        "bit-exact f64 MIN_POSITIVE roundtrip"
    );
}
