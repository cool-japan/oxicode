//! Comprehensive numeric boundary value and encoding tests.
//!
//! These tests verify correct roundtrip behaviour and encoding sizes for all
//! primitive integer and floating-point types at their extreme values, as well
//! as the exact byte-length guarantees of the oxicode varint scheme.

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

mod numeric_boundary_tests {
    use super::*;

    // -------------------------------------------------------------------------
    // 1. u8 MIN and MAX roundtrip
    // -------------------------------------------------------------------------
    #[test]
    fn test_u8_min_max_roundtrip() {
        for v in [u8::MIN, u8::MAX] {
            let enc = encode_to_vec(&v).expect("encode u8");
            let (dec, _): (u8, _) = decode_from_slice(&enc).expect("decode u8");
            assert_eq!(v, dec, "u8 roundtrip failed for {}", v);
        }
    }

    // -------------------------------------------------------------------------
    // 2. u16 MIN and MAX roundtrip
    // -------------------------------------------------------------------------
    #[test]
    fn test_u16_min_max_roundtrip() {
        for v in [u16::MIN, u16::MAX] {
            let enc = encode_to_vec(&v).expect("encode u16");
            let (dec, _): (u16, _) = decode_from_slice(&enc).expect("decode u16");
            assert_eq!(v, dec, "u16 roundtrip failed for {}", v);
        }
    }

    // -------------------------------------------------------------------------
    // 3. u32 MIN and MAX roundtrip
    // -------------------------------------------------------------------------
    #[test]
    fn test_u32_min_max_roundtrip() {
        for v in [u32::MIN, u32::MAX] {
            let enc = encode_to_vec(&v).expect("encode u32");
            let (dec, _): (u32, _) = decode_from_slice(&enc).expect("decode u32");
            assert_eq!(v, dec, "u32 roundtrip failed for {}", v);
        }
    }

    // -------------------------------------------------------------------------
    // 4. u64 MIN and MAX roundtrip
    // -------------------------------------------------------------------------
    #[test]
    fn test_u64_min_max_roundtrip() {
        for v in [u64::MIN, u64::MAX] {
            let enc = encode_to_vec(&v).expect("encode u64");
            let (dec, _): (u64, _) = decode_from_slice(&enc).expect("decode u64");
            assert_eq!(v, dec, "u64 roundtrip failed for {}", v);
        }
    }

    // -------------------------------------------------------------------------
    // 5. u128 MIN and MAX roundtrip
    // -------------------------------------------------------------------------
    #[test]
    fn test_u128_min_max_roundtrip() {
        for v in [u128::MIN, u128::MAX] {
            let enc = encode_to_vec(&v).expect("encode u128");
            let (dec, _): (u128, _) = decode_from_slice(&enc).expect("decode u128");
            assert_eq!(v, dec, "u128 roundtrip failed for {}", v);
        }
    }

    // -------------------------------------------------------------------------
    // 6. i8 MIN and MAX roundtrip
    // -------------------------------------------------------------------------
    #[test]
    fn test_i8_min_max_roundtrip() {
        for v in [i8::MIN, i8::MAX] {
            let enc = encode_to_vec(&v).expect("encode i8");
            let (dec, _): (i8, _) = decode_from_slice(&enc).expect("decode i8");
            assert_eq!(v, dec, "i8 roundtrip failed for {}", v);
        }
    }

    // -------------------------------------------------------------------------
    // 7. i16 MIN and MAX roundtrip
    // -------------------------------------------------------------------------
    #[test]
    fn test_i16_min_max_roundtrip() {
        for v in [i16::MIN, i16::MAX] {
            let enc = encode_to_vec(&v).expect("encode i16");
            let (dec, _): (i16, _) = decode_from_slice(&enc).expect("decode i16");
            assert_eq!(v, dec, "i16 roundtrip failed for {}", v);
        }
    }

    // -------------------------------------------------------------------------
    // 8. i32 MIN and MAX roundtrip
    // -------------------------------------------------------------------------
    #[test]
    fn test_i32_min_max_roundtrip() {
        for v in [i32::MIN, i32::MAX] {
            let enc = encode_to_vec(&v).expect("encode i32");
            let (dec, _): (i32, _) = decode_from_slice(&enc).expect("decode i32");
            assert_eq!(v, dec, "i32 roundtrip failed for {}", v);
        }
    }

    // -------------------------------------------------------------------------
    // 9. i64 MIN and MAX roundtrip
    // -------------------------------------------------------------------------
    #[test]
    fn test_i64_min_max_roundtrip() {
        for v in [i64::MIN, i64::MAX] {
            let enc = encode_to_vec(&v).expect("encode i64");
            let (dec, _): (i64, _) = decode_from_slice(&enc).expect("decode i64");
            assert_eq!(v, dec, "i64 roundtrip failed for {}", v);
        }
    }

    // -------------------------------------------------------------------------
    // 10. i128 MIN and MAX roundtrip
    // -------------------------------------------------------------------------
    #[test]
    fn test_i128_min_max_roundtrip() {
        for v in [i128::MIN, i128::MAX] {
            let enc = encode_to_vec(&v).expect("encode i128");
            let (dec, _): (i128, _) = decode_from_slice(&enc).expect("decode i128");
            assert_eq!(v, dec, "i128 roundtrip failed for {}", v);
        }
    }

    // -------------------------------------------------------------------------
    // 11. usize::MAX roundtrip
    //     oxicode encodes usize as u64; on 64-bit systems usize::MAX == u64::MAX
    // -------------------------------------------------------------------------
    #[test]
    fn test_usize_max_roundtrip() {
        let v: usize = usize::MAX;
        let enc = encode_to_vec(&v).expect("encode usize::MAX");
        let (dec, _): (usize, _) = decode_from_slice(&enc).expect("decode usize::MAX");
        assert_eq!(v, dec, "usize::MAX roundtrip failed");
    }

    // -------------------------------------------------------------------------
    // 12. f32 INFINITY, NEG_INFINITY, NAN roundtrip
    //     NaN comparison must use bit equality since NaN != NaN by IEEE 754.
    // -------------------------------------------------------------------------
    #[test]
    fn test_f32_special_values_roundtrip() {
        // INFINITY
        let enc = encode_to_vec(&f32::INFINITY).expect("encode f32::INFINITY");
        let (dec, _): (f32, _) = decode_from_slice(&enc).expect("decode f32::INFINITY");
        assert_eq!(dec, f32::INFINITY, "f32::INFINITY roundtrip failed");

        // NEG_INFINITY
        let enc = encode_to_vec(&f32::NEG_INFINITY).expect("encode f32::NEG_INFINITY");
        let (dec, _): (f32, _) = decode_from_slice(&enc).expect("decode f32::NEG_INFINITY");
        assert_eq!(dec, f32::NEG_INFINITY, "f32::NEG_INFINITY roundtrip failed");

        // NAN – compare as bits because NaN != NaN
        let nan = f32::NAN;
        let enc = encode_to_vec(&nan).expect("encode f32::NAN");
        let (dec, _): (f32, _) = decode_from_slice(&enc).expect("decode f32::NAN");
        assert_eq!(
            nan.to_bits(),
            dec.to_bits(),
            "f32::NAN bit pattern not preserved"
        );
    }

    // -------------------------------------------------------------------------
    // 13. f64 INFINITY, NEG_INFINITY, NAN roundtrip
    // -------------------------------------------------------------------------
    #[test]
    fn test_f64_special_values_roundtrip() {
        // INFINITY
        let enc = encode_to_vec(&f64::INFINITY).expect("encode f64::INFINITY");
        let (dec, _): (f64, _) = decode_from_slice(&enc).expect("decode f64::INFINITY");
        assert_eq!(dec, f64::INFINITY, "f64::INFINITY roundtrip failed");

        // NEG_INFINITY
        let enc = encode_to_vec(&f64::NEG_INFINITY).expect("encode f64::NEG_INFINITY");
        let (dec, _): (f64, _) = decode_from_slice(&enc).expect("decode f64::NEG_INFINITY");
        assert_eq!(dec, f64::NEG_INFINITY, "f64::NEG_INFINITY roundtrip failed");

        // NAN – compare as bits
        let nan = f64::NAN;
        let enc = encode_to_vec(&nan).expect("encode f64::NAN");
        let (dec, _): (f64, _) = decode_from_slice(&enc).expect("decode f64::NAN");
        assert_eq!(
            nan.to_bits(),
            dec.to_bits(),
            "f64::NAN bit pattern not preserved"
        );
    }

    // -------------------------------------------------------------------------
    // 14. f32 MIN and MAX roundtrip
    // -------------------------------------------------------------------------
    #[test]
    fn test_f32_min_max_roundtrip() {
        for v in [f32::MIN, f32::MAX] {
            let enc = encode_to_vec(&v).expect("encode f32");
            let (dec, _): (f32, _) = decode_from_slice(&enc).expect("decode f32");
            assert_eq!(
                v.to_bits(),
                dec.to_bits(),
                "f32 bit pattern not preserved for {}",
                v
            );
        }
    }

    // -------------------------------------------------------------------------
    // 15. f64 MIN and MAX roundtrip
    // -------------------------------------------------------------------------
    #[test]
    fn test_f64_min_max_roundtrip() {
        for v in [f64::MIN, f64::MAX] {
            let enc = encode_to_vec(&v).expect("encode f64");
            let (dec, _): (f64, _) = decode_from_slice(&enc).expect("decode f64");
            assert_eq!(
                v.to_bits(),
                dec.to_bits(),
                "f64 bit pattern not preserved for {}",
                v
            );
        }
    }

    // -------------------------------------------------------------------------
    // 16. f32::EPSILON roundtrip
    //     Also verifies that the mathematical constant PI roundtrips correctly
    //     as f32 (cast from std::f64::consts::PI).
    // -------------------------------------------------------------------------
    #[test]
    fn test_f32_epsilon_roundtrip() {
        // f32::EPSILON is the smallest difference between 1.0 and the next
        // representable f32 value.
        let v = f32::EPSILON;
        let enc = encode_to_vec(&v).expect("encode f32::EPSILON");
        let (dec, _): (f32, _) = decode_from_slice(&enc).expect("decode f32::EPSILON");
        assert_eq!(
            v.to_bits(),
            dec.to_bits(),
            "f32::EPSILON bit pattern not preserved"
        );

        // PI as f32 for an additional well-known irrational constant check.
        let pi_f32 = std::f64::consts::PI as f32;
        let enc = encode_to_vec(&pi_f32).expect("encode PI as f32");
        let (dec_pi, _): (f32, _) = decode_from_slice(&enc).expect("decode PI as f32");
        assert_eq!(
            pi_f32.to_bits(),
            dec_pi.to_bits(),
            "PI (f32) bit pattern not preserved"
        );
    }

    // -------------------------------------------------------------------------
    // 17. f64::EPSILON roundtrip
    //     Also verifies that Euler's number E roundtrips correctly as f64.
    // -------------------------------------------------------------------------
    #[test]
    fn test_f64_epsilon_roundtrip() {
        let v = f64::EPSILON;
        let enc = encode_to_vec(&v).expect("encode f64::EPSILON");
        let (dec, _): (f64, _) = decode_from_slice(&enc).expect("decode f64::EPSILON");
        assert_eq!(
            v.to_bits(),
            dec.to_bits(),
            "f64::EPSILON bit pattern not preserved"
        );

        // Euler's number E as an additional well-known irrational constant check.
        let e = std::f64::consts::E;
        let enc = encode_to_vec(&e).expect("encode E");
        let (dec_e, _): (f64, _) = decode_from_slice(&enc).expect("decode E");
        assert_eq!(
            e.to_bits(),
            dec_e.to_bits(),
            "E (f64) bit pattern not preserved"
        );
    }

    // -------------------------------------------------------------------------
    // 18. Varint boundary: value 250 encodes as exactly 1 byte
    //     oxicode varint: 0–250 → single byte (SINGLE_BYTE_MAX = 250)
    // -------------------------------------------------------------------------
    #[test]
    fn test_varint_250_encodes_as_1_byte() {
        let v: u64 = 250;
        let enc = encode_to_vec(&v).expect("encode 250");
        assert_eq!(
            enc.len(),
            1,
            "value 250 must encode as 1 byte, got {} bytes",
            enc.len()
        );
        let (dec, _): (u64, _) = decode_from_slice(&enc).expect("decode 250");
        assert_eq!(v, dec);
    }

    // -------------------------------------------------------------------------
    // 19. Varint boundary: value 251 encodes as exactly 3 bytes
    //     251 is the first value above SINGLE_BYTE_MAX; encoded as
    //     marker byte (251) + 2 little-endian bytes for the u16 value.
    // -------------------------------------------------------------------------
    #[test]
    fn test_varint_251_encodes_as_3_bytes() {
        let v: u64 = 251;
        let enc = encode_to_vec(&v).expect("encode 251");
        assert_eq!(
            enc.len(),
            3,
            "value 251 must encode as 3 bytes (marker + u16), got {} bytes",
            enc.len()
        );
        let (dec, _): (u64, _) = decode_from_slice(&enc).expect("decode 251");
        assert_eq!(v, dec);
    }

    // -------------------------------------------------------------------------
    // 20. Varint boundary: value 65535 (u16::MAX) encodes as exactly 3 bytes
    //     All values in [251, 65535] fit within the 3-byte varint range.
    // -------------------------------------------------------------------------
    #[test]
    fn test_varint_65535_encodes_as_3_bytes() {
        let v: u64 = 65535;
        let enc = encode_to_vec(&v).expect("encode 65535");
        assert_eq!(
            enc.len(),
            3,
            "value 65535 must encode as 3 bytes (marker + u16), got {} bytes",
            enc.len()
        );
        let (dec, _): (u64, _) = decode_from_slice(&enc).expect("decode 65535");
        assert_eq!(v, dec);
    }

    // -------------------------------------------------------------------------
    // 21. Varint boundary: value 65536 (u16::MAX + 1) encodes as exactly 5 bytes
    //     First value that exceeds the 3-byte range; encoded as
    //     marker byte (252) + 4 little-endian bytes for the u32 value.
    // -------------------------------------------------------------------------
    #[test]
    fn test_varint_65536_encodes_as_5_bytes() {
        let v: u64 = 65536;
        let enc = encode_to_vec(&v).expect("encode 65536");
        assert_eq!(
            enc.len(),
            5,
            "value 65536 must encode as 5 bytes (marker + u32), got {} bytes",
            enc.len()
        );
        let (dec, _): (u64, _) = decode_from_slice(&enc).expect("decode 65536");
        assert_eq!(v, dec);
    }

    // -------------------------------------------------------------------------
    // 22. Varint boundary: value 4294967295 (u32::MAX) encodes as exactly 5 bytes
    //     All values in [65536, 4294967295] fit within the 5-byte varint range.
    // -------------------------------------------------------------------------
    #[test]
    fn test_varint_u32_max_encodes_as_5_bytes() {
        let v: u64 = 4_294_967_295; // u32::MAX
        let enc = encode_to_vec(&v).expect("encode u32::MAX");
        assert_eq!(
            enc.len(),
            5,
            "value u32::MAX must encode as 5 bytes (marker + u32), got {} bytes",
            enc.len()
        );
        let (dec, _): (u64, _) = decode_from_slice(&enc).expect("decode u32::MAX");
        assert_eq!(v, dec);
    }
}
