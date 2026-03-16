//! Extended varint encoding correctness tests
//!
//! Covers byte-level encoding verification, zigzag signed encoding,
//! decode consumed-bytes tracking, roundtrip for all boundary values,
//! little-endian byte order, and sequential encoding size composition.

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

mod varint_extended_tests {
    use super::*;

    // -----------------------------------------------------------------------
    // Test 1: Values 0..=10 each encode as 1 byte
    // -----------------------------------------------------------------------
    #[test]
    fn test_values_0_to_10_encode_as_1_byte() {
        for val in 0u64..=10 {
            let enc = encode_to_vec(&val).expect("encode should succeed");
            assert_eq!(
                enc.len(),
                1,
                "Value {} (u64) should encode as exactly 1 byte, got {} bytes",
                val,
                enc.len()
            );
        }
    }

    // -----------------------------------------------------------------------
    // Test 2: Value 100 encodes as 1 byte
    // -----------------------------------------------------------------------
    #[test]
    fn test_value_100_encodes_as_1_byte() {
        let val: u64 = 100;
        let enc = encode_to_vec(&val).expect("encode should succeed");
        assert_eq!(
            enc.len(),
            1,
            "Value 100 should encode as 1 byte, got {} bytes",
            enc.len()
        );
        assert_eq!(enc[0], 100u8, "Value 100 should encode as byte 100");
    }

    // -----------------------------------------------------------------------
    // Test 3: Value 200 encodes as 1 byte
    // -----------------------------------------------------------------------
    #[test]
    fn test_value_200_encodes_as_1_byte() {
        let val: u64 = 200;
        let enc = encode_to_vec(&val).expect("encode should succeed");
        assert_eq!(
            enc.len(),
            1,
            "Value 200 should encode as 1 byte, got {} bytes",
            enc.len()
        );
        assert_eq!(enc[0], 200u8, "Value 200 should encode as byte 200");
    }

    // -----------------------------------------------------------------------
    // Test 4: Value 250 (SINGLE_BYTE_MAX) encodes as 1 byte
    // -----------------------------------------------------------------------
    #[test]
    fn test_value_250_single_byte_max_encodes_as_1_byte() {
        // SINGLE_BYTE_MAX = 250
        let val: u64 = 250;
        let enc = encode_to_vec(&val).expect("encode should succeed");
        assert_eq!(
            enc.len(),
            1,
            "Value 250 (SINGLE_BYTE_MAX) should encode as 1 byte, got {} bytes",
            enc.len()
        );
        assert_eq!(enc[0], 250u8, "Value 250 should encode as byte 250");
    }

    // -----------------------------------------------------------------------
    // Test 5: Value 251 encodes as 3 bytes [251, 251, 0]
    // -----------------------------------------------------------------------
    #[test]
    fn test_value_251_encodes_as_3_bytes() {
        // 251 > SINGLE_BYTE_MAX, so uses U16_BYTE (251) tag + 2 LE bytes
        // 251 in LE u16 = [251, 0] = [0xFB, 0x00]
        let val: u64 = 251;
        let enc = encode_to_vec(&val).expect("encode should succeed");
        assert_eq!(
            enc.len(),
            3,
            "Value 251 should encode as 3 bytes [marker, lo, hi], got {} bytes",
            enc.len()
        );
        // tag byte = 251 (U16_BYTE), then LE bytes of 251u16 = [0xFB, 0x00]
        assert_eq!(enc[0], 251u8, "First byte should be U16_BYTE marker (251)");
        assert_eq!(enc[1], 251u8, "Second byte should be 0xFB (lo byte of 251)");
        assert_eq!(enc[2], 0u8, "Third byte should be 0x00 (hi byte of 251)");
    }

    // -----------------------------------------------------------------------
    // Test 6: Value 252 encodes as 3 bytes [251, 252, 0]
    // -----------------------------------------------------------------------
    #[test]
    fn test_value_252_encodes_as_3_bytes() {
        // 252 > SINGLE_BYTE_MAX, uses U16_BYTE (251) tag + 2 LE bytes of 252
        // 252 in LE u16 = [0xFC, 0x00]
        let val: u64 = 252;
        let enc = encode_to_vec(&val).expect("encode should succeed");
        assert_eq!(
            enc.len(),
            3,
            "Value 252 should encode as 3 bytes, got {} bytes",
            enc.len()
        );
        assert_eq!(enc[0], 251u8, "First byte should be U16_BYTE marker (251)");
        assert_eq!(enc[1], 252u8, "Second byte should be 0xFC (lo byte of 252)");
        assert_eq!(enc[2], 0u8, "Third byte should be 0x00 (hi byte of 252)");
    }

    // -----------------------------------------------------------------------
    // Test 7: Value 1000 encodes as 3 bytes
    // -----------------------------------------------------------------------
    #[test]
    fn test_value_1000_encodes_as_3_bytes() {
        // 1000 is in range [251, 65535], so encodes as 3 bytes
        let val: u64 = 1000;
        let enc = encode_to_vec(&val).expect("encode should succeed");
        assert_eq!(
            enc.len(),
            3,
            "Value 1000 should encode as 3 bytes (U16_BYTE + 2 LE bytes), got {} bytes",
            enc.len()
        );
        assert_eq!(enc[0], 251u8, "First byte should be U16_BYTE marker (251)");
        // 1000 in LE u16 = [0xE8, 0x03]
        assert_eq!(
            enc[1], 0xE8u8,
            "Second byte should be 0xE8 (lo byte of 1000)"
        );
        assert_eq!(
            enc[2], 0x03u8,
            "Third byte should be 0x03 (hi byte of 1000)"
        );
    }

    // -----------------------------------------------------------------------
    // Test 8: Value 65535 (u16::MAX) encodes as 3 bytes
    // -----------------------------------------------------------------------
    #[test]
    fn test_value_u16_max_encodes_as_3_bytes() {
        let val: u64 = u16::MAX as u64;
        let enc = encode_to_vec(&val).expect("encode should succeed");
        assert_eq!(
            enc.len(),
            3,
            "u16::MAX (65535) should encode as 3 bytes, got {} bytes",
            enc.len()
        );
        assert_eq!(enc[0], 251u8, "First byte should be U16_BYTE marker (251)");
        // 65535 in LE u16 = [0xFF, 0xFF]
        assert_eq!(enc[1], 0xFFu8, "Second byte should be 0xFF");
        assert_eq!(enc[2], 0xFFu8, "Third byte should be 0xFF");
    }

    // -----------------------------------------------------------------------
    // Test 9: Value 65536 encodes as 5 bytes
    // -----------------------------------------------------------------------
    #[test]
    fn test_value_65536_encodes_as_5_bytes() {
        // 65536 > u16::MAX, so uses U32_BYTE (252) tag + 4 LE bytes
        let val: u64 = 65536;
        let enc = encode_to_vec(&val).expect("encode should succeed");
        assert_eq!(
            enc.len(),
            5,
            "Value 65536 should encode as 5 bytes, got {} bytes",
            enc.len()
        );
        assert_eq!(enc[0], 252u8, "First byte should be U32_BYTE marker (252)");
        // 65536 in LE u32 = [0x00, 0x00, 0x01, 0x00]
        assert_eq!(enc[1], 0x00u8, "LE u32 byte 0 of 65536");
        assert_eq!(enc[2], 0x00u8, "LE u32 byte 1 of 65536");
        assert_eq!(enc[3], 0x01u8, "LE u32 byte 2 of 65536");
        assert_eq!(enc[4], 0x00u8, "LE u32 byte 3 of 65536");
    }

    // -----------------------------------------------------------------------
    // Test 10: Value 65537 encodes as 5 bytes
    // -----------------------------------------------------------------------
    #[test]
    fn test_value_65537_encodes_as_5_bytes() {
        let val: u64 = 65537;
        let enc = encode_to_vec(&val).expect("encode should succeed");
        assert_eq!(
            enc.len(),
            5,
            "Value 65537 should encode as 5 bytes, got {} bytes",
            enc.len()
        );
        assert_eq!(enc[0], 252u8, "First byte should be U32_BYTE marker (252)");
        // 65537 = 0x00010001, LE = [0x01, 0x00, 0x01, 0x00]
        assert_eq!(enc[1], 0x01u8, "LE u32 byte 0 of 65537");
        assert_eq!(enc[2], 0x00u8, "LE u32 byte 1 of 65537");
        assert_eq!(enc[3], 0x01u8, "LE u32 byte 2 of 65537");
        assert_eq!(enc[4], 0x00u8, "LE u32 byte 3 of 65537");
    }

    // -----------------------------------------------------------------------
    // Test 11: Value 4294967295 (u32::MAX) encodes as 5 bytes
    // -----------------------------------------------------------------------
    #[test]
    fn test_value_u32_max_encodes_as_5_bytes() {
        let val: u64 = u32::MAX as u64;
        let enc = encode_to_vec(&val).expect("encode should succeed");
        assert_eq!(
            enc.len(),
            5,
            "u32::MAX should encode as 5 bytes, got {} bytes",
            enc.len()
        );
        assert_eq!(enc[0], 252u8, "First byte should be U32_BYTE marker (252)");
        // u32::MAX = 0xFFFFFFFF, LE = [0xFF, 0xFF, 0xFF, 0xFF]
        assert_eq!(enc[1], 0xFFu8, "LE u32 byte 0 of u32::MAX");
        assert_eq!(enc[2], 0xFFu8, "LE u32 byte 1 of u32::MAX");
        assert_eq!(enc[3], 0xFFu8, "LE u32 byte 2 of u32::MAX");
        assert_eq!(enc[4], 0xFFu8, "LE u32 byte 3 of u32::MAX");
    }

    // -----------------------------------------------------------------------
    // Test 12: Value 4294967296 (u32::MAX + 1) encodes as 9 bytes
    // -----------------------------------------------------------------------
    #[test]
    fn test_value_u32_max_plus_1_encodes_as_9_bytes() {
        let val: u64 = u32::MAX as u64 + 1;
        let enc = encode_to_vec(&val).expect("encode should succeed");
        assert_eq!(
            enc.len(),
            9,
            "u32::MAX + 1 should encode as 9 bytes, got {} bytes",
            enc.len()
        );
        assert_eq!(enc[0], 253u8, "First byte should be U64_BYTE marker (253)");
        // 4294967296 = 0x100000000, LE u64 = [0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00]
        assert_eq!(enc[1], 0x00u8, "LE u64 byte 0");
        assert_eq!(enc[2], 0x00u8, "LE u64 byte 1");
        assert_eq!(enc[3], 0x00u8, "LE u64 byte 2");
        assert_eq!(enc[4], 0x00u8, "LE u64 byte 3");
        assert_eq!(enc[5], 0x01u8, "LE u64 byte 4");
        assert_eq!(enc[6], 0x00u8, "LE u64 byte 5");
        assert_eq!(enc[7], 0x00u8, "LE u64 byte 6");
        assert_eq!(enc[8], 0x00u8, "LE u64 byte 7");
    }

    // -----------------------------------------------------------------------
    // Test 13: Value u64::MAX encodes as 9 bytes
    // -----------------------------------------------------------------------
    #[test]
    fn test_value_u64_max_encodes_as_9_bytes() {
        let val: u64 = u64::MAX;
        let enc = encode_to_vec(&val).expect("encode should succeed");
        assert_eq!(
            enc.len(),
            9,
            "u64::MAX should encode as 9 bytes, got {} bytes",
            enc.len()
        );
        assert_eq!(enc[0], 253u8, "First byte should be U64_BYTE marker (253)");
        // u64::MAX = all 0xFF bytes in LE
        assert_eq!(
            &enc[1..9],
            &[0xFFu8; 8],
            "Bytes 1..9 of u64::MAX encoding should all be 0xFF"
        );
    }

    // -----------------------------------------------------------------------
    // Test 14: Signed zigzag i64(0) → zigzag u64(0) → 1 byte
    // -----------------------------------------------------------------------
    #[test]
    fn test_zigzag_i64_zero_encodes_as_1_byte() {
        // zigzag(0) = (0 << 1) ^ (0 >> 63) = 0
        let val: i64 = 0;
        let enc = encode_to_vec(&val).expect("encode should succeed");
        assert_eq!(
            enc.len(),
            1,
            "i64(0) zigzag-encodes to 0 which should be 1 byte, got {} bytes",
            enc.len()
        );
        assert_eq!(enc[0], 0u8, "i64(0) should encode as byte 0");
    }

    // -----------------------------------------------------------------------
    // Test 15: Signed zigzag i64(-1) → zigzag u64(1) → 1 byte
    // -----------------------------------------------------------------------
    #[test]
    fn test_zigzag_i64_neg1_encodes_as_1_byte() {
        // zigzag(-1) = ((-1u64).wrapping_shl(1)) ^ ((-1i64 >> 63) as u64)
        //            = 0xFFFFFFFFFFFFFFFE ^ 0xFFFFFFFFFFFFFFFF = 1
        let val: i64 = -1;
        let enc = encode_to_vec(&val).expect("encode should succeed");
        assert_eq!(
            enc.len(),
            1,
            "i64(-1) zigzag-encodes to 1 which should be 1 byte, got {} bytes",
            enc.len()
        );
        assert_eq!(enc[0], 1u8, "i64(-1) should encode as byte 1 (zigzag=1)");
    }

    // -----------------------------------------------------------------------
    // Test 16: Signed zigzag i64(1) → zigzag u64(2) → 1 byte
    // -----------------------------------------------------------------------
    #[test]
    fn test_zigzag_i64_pos1_encodes_as_1_byte() {
        // zigzag(1) = (1u64.wrapping_shl(1)) ^ (0) = 2
        let val: i64 = 1;
        let enc = encode_to_vec(&val).expect("encode should succeed");
        assert_eq!(
            enc.len(),
            1,
            "i64(1) zigzag-encodes to 2 which should be 1 byte, got {} bytes",
            enc.len()
        );
        assert_eq!(enc[0], 2u8, "i64(1) should encode as byte 2 (zigzag=2)");
    }

    // -----------------------------------------------------------------------
    // Test 17: Signed zigzag i64(-64) → zigzag u64(127) → 1 byte
    // -----------------------------------------------------------------------
    #[test]
    fn test_zigzag_i64_neg64_encodes_as_1_byte() {
        // zigzag(-64): -64 as u64 = 0xFFFFFFFFFFFFFFC0
        // wrapping_shl(1) = 0xFFFFFFFFFFFFFF80
        // (-64 >> 63) as u64 = 0xFFFFFFFFFFFFFFFF
        // XOR = 0x7F = 127 → 1 byte (127 <= 250)
        let val: i64 = -64;
        let enc = encode_to_vec(&val).expect("encode should succeed");
        assert_eq!(
            enc.len(),
            1,
            "i64(-64) zigzag-encodes to 127 which should be 1 byte, got {} bytes",
            enc.len()
        );
        assert_eq!(
            enc[0], 127u8,
            "i64(-64) should encode as byte 127 (zigzag=127)"
        );
    }

    // -----------------------------------------------------------------------
    // Test 18: Signed zigzag i64(-65) → zigzag u64(129) → 1 byte
    //
    // Note: 129 <= SINGLE_BYTE_MAX (250), so this IS a single byte encoding.
    // -----------------------------------------------------------------------
    #[test]
    fn test_zigzag_i64_neg65_encodes_as_1_byte() {
        // zigzag(-65): -65 as u64 = 0xFFFFFFFFFFFFFFBF
        // wrapping_shl(1) = 0xFFFFFFFFFFFFFF7E
        // (-65 >> 63) as u64 = 0xFFFFFFFFFFFFFFFF
        // XOR = 0x81 = 129 → 1 byte (129 <= 250 = SINGLE_BYTE_MAX)
        let val: i64 = -65;
        let enc = encode_to_vec(&val).expect("encode should succeed");
        assert_eq!(
            enc.len(),
            1,
            "i64(-65) zigzag-encodes to 129 (still <= SINGLE_BYTE_MAX=250), got {} bytes",
            enc.len()
        );
        assert_eq!(
            enc[0], 129u8,
            "i64(-65) should encode as byte 129 (zigzag=129)"
        );
    }

    // -----------------------------------------------------------------------
    // Test 19: Decode varint: consumed bytes matches encoding size
    // -----------------------------------------------------------------------
    #[test]
    fn test_decode_consumed_bytes_matches_encoding_size() {
        // Test that the number of bytes consumed by decode equals encode output length
        let test_cases: &[u64] = &[
            0,
            1,
            100,
            250,
            251,
            1000,
            65535,
            65536,
            u32::MAX as u64,
            u64::MAX,
        ];

        for &val in test_cases {
            let enc = encode_to_vec(&val).expect("encode should succeed");
            let encoded_len = enc.len();

            // Append extra bytes to ensure decode stops at the right boundary
            let mut padded = enc.clone();
            padded.extend_from_slice(&[0xAAu8, 0xBBu8, 0xCCu8]);

            let (decoded_val, consumed): (u64, usize) =
                decode_from_slice(&padded).expect("decode should succeed");

            assert_eq!(
                decoded_val, val,
                "Decoded value should match original for val={}",
                val
            );
            assert_eq!(
                consumed, encoded_len,
                "Consumed bytes ({}) should match encoded length ({}) for val={}",
                consumed, encoded_len, val
            );
        }
    }

    // -----------------------------------------------------------------------
    // Test 20: Varint roundtrip for all boundaries
    // -----------------------------------------------------------------------
    #[test]
    fn test_varint_roundtrip_all_boundaries() {
        use std::f64::consts::{E, PI};

        let boundary_u64: &[u64] = &[0, 250, 251, 65535, 65536, u32::MAX as u64, u64::MAX];
        for &val in boundary_u64 {
            let enc = encode_to_vec(&val).expect("encode u64 boundary");
            let (dec, _): (u64, usize) = decode_from_slice(&enc).expect("decode u64 boundary");
            assert_eq!(val, dec, "u64 roundtrip failed for boundary value {}", val);
        }

        // Also verify f64 constants roundtrip correctly
        let enc_pi = encode_to_vec(&PI).expect("encode PI");
        let (dec_pi, _): (f64, usize) = decode_from_slice(&enc_pi).expect("decode PI");
        assert_eq!(
            PI, dec_pi,
            "f64 PI should roundtrip exactly (IEEE 754 fixed-width encoding)"
        );

        let enc_e = encode_to_vec(&E).expect("encode E");
        let (dec_e, _): (f64, usize) = decode_from_slice(&enc_e).expect("decode E");
        assert_eq!(
            E, dec_e,
            "f64 E should roundtrip exactly (IEEE 754 fixed-width encoding)"
        );
    }

    // -----------------------------------------------------------------------
    // Test 21: Varint is little-endian within multi-byte encoding
    // -----------------------------------------------------------------------
    #[test]
    fn test_varint_multi_byte_is_little_endian() {
        // 3-byte: value 1000 = 0x03E8 → LE bytes = [0xE8, 0x03]
        let val_3: u64 = 1000;
        let enc_3 = encode_to_vec(&val_3).expect("encode 1000");
        assert_eq!(enc_3.len(), 3, "1000 should be 3 bytes");
        assert_eq!(enc_3[0], 251u8, "Tag byte for U16 range");
        assert_eq!(enc_3[1], 0xE8u8, "LE lo byte of 1000");
        assert_eq!(enc_3[2], 0x03u8, "LE hi byte of 1000");
        // Verify: reconstruct value from LE bytes
        let reconstructed_3 = (enc_3[1] as u64) | ((enc_3[2] as u64) << 8);
        assert_eq!(reconstructed_3, val_3, "LE reconstruction of 1000");

        // 5-byte: value 0x01020304 = 16909060 → LE u32 = [0x04, 0x03, 0x02, 0x01]
        let val_5: u64 = 0x01020304u64;
        let enc_5 = encode_to_vec(&val_5).expect("encode 0x01020304");
        assert_eq!(enc_5.len(), 5, "0x01020304 should be 5 bytes");
        assert_eq!(enc_5[0], 252u8, "Tag byte for U32 range");
        assert_eq!(enc_5[1], 0x04u8, "LE byte 0 of 0x01020304");
        assert_eq!(enc_5[2], 0x03u8, "LE byte 1 of 0x01020304");
        assert_eq!(enc_5[3], 0x02u8, "LE byte 2 of 0x01020304");
        assert_eq!(enc_5[4], 0x01u8, "LE byte 3 of 0x01020304");
        // Verify: reconstruct value from LE bytes
        let reconstructed_5 = (enc_5[1] as u64)
            | ((enc_5[2] as u64) << 8)
            | ((enc_5[3] as u64) << 16)
            | ((enc_5[4] as u64) << 24);
        assert_eq!(reconstructed_5, val_5, "LE reconstruction of 0x01020304");

        // 9-byte: value 0x0102030405060708 → LE u64 bytes
        let val_9: u64 = 0x0102030405060708u64;
        let enc_9 = encode_to_vec(&val_9).expect("encode 0x0102030405060708");
        assert_eq!(enc_9.len(), 9, "0x0102030405060708 should be 9 bytes");
        assert_eq!(enc_9[0], 253u8, "Tag byte for U64 range");
        assert_eq!(enc_9[1], 0x08u8, "LE byte 0 of 0x0102030405060708");
        assert_eq!(enc_9[2], 0x07u8, "LE byte 1");
        assert_eq!(enc_9[3], 0x06u8, "LE byte 2");
        assert_eq!(enc_9[4], 0x05u8, "LE byte 3");
        assert_eq!(enc_9[5], 0x04u8, "LE byte 4");
        assert_eq!(enc_9[6], 0x03u8, "LE byte 5");
        assert_eq!(enc_9[7], 0x02u8, "LE byte 6");
        assert_eq!(enc_9[8], 0x01u8, "LE byte 7");
    }

    // -----------------------------------------------------------------------
    // Test 22: Sequential varint values in Vec: total size is sum of individual sizes
    // -----------------------------------------------------------------------
    #[test]
    fn test_sequential_varint_vec_total_size() {
        // The encoding of Vec<u64> is: varint(len) + each element encoded as varint.
        // We verify that the payload size (after the length prefix) equals sum of individual sizes.

        let values: Vec<u64> = vec![
            0,
            100,
            250,
            251,
            1000,
            65535,
            65536,
            u32::MAX as u64,
            u64::MAX,
        ];

        // Compute expected individual encoding sizes
        let individual_sizes: Vec<usize> = values
            .iter()
            .map(|v| encode_to_vec(v).expect("encode element").len())
            .collect();
        let expected_payload: usize = individual_sizes.iter().sum();

        // Encode the full vec
        let vec_enc = encode_to_vec(&values).expect("encode Vec");

        // The length prefix is also a varint. values.len() = 9, which encodes as 1 byte.
        let len_prefix_size = encode_to_vec(&(values.len() as u64))
            .expect("encode length prefix")
            .len();

        let total_expected = len_prefix_size + expected_payload;

        assert_eq!(
            vec_enc.len(),
            total_expected,
            "Vec encoding size ({}) should equal length_prefix ({}) + sum_of_elements ({})",
            vec_enc.len(),
            len_prefix_size,
            expected_payload
        );

        // Additionally verify the expected individual sizes match known encoding rules
        assert_eq!(individual_sizes[0], 1, "0 should be 1 byte");
        assert_eq!(individual_sizes[1], 1, "100 should be 1 byte");
        assert_eq!(individual_sizes[2], 1, "250 should be 1 byte");
        assert_eq!(individual_sizes[3], 3, "251 should be 3 bytes");
        assert_eq!(individual_sizes[4], 3, "1000 should be 3 bytes");
        assert_eq!(individual_sizes[5], 3, "65535 should be 3 bytes");
        assert_eq!(individual_sizes[6], 5, "65536 should be 5 bytes");
        assert_eq!(individual_sizes[7], 5, "u32::MAX should be 5 bytes");
        assert_eq!(individual_sizes[8], 9, "u64::MAX should be 9 bytes");
    }
}
