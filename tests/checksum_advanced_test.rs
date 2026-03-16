//! Advanced checksum integration tests

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
#[cfg(feature = "checksum")]
mod advanced_checksum {
    use oxicode::checksum::{
        decode_with_checksum, decode_with_checksum_config, encode_with_checksum,
        encode_with_checksum_config, verify_checksum, wrap_with_checksum, HEADER_SIZE,
    };
    use oxicode::{config, Decode, Encode};

    // A struct that deliberately has a field named after "checksum" in its
    // name – ensures no name collision with our integrity layer.
    #[derive(Debug, Clone, PartialEq, Encode, Decode)]
    struct Packet {
        seq: u32,
        data: Vec<u8>,
        checksum_field: u32,
    }

    // -----------------------------------------------------------------------
    // Test 1: struct field named "checksum_field" doesn't interfere
    // -----------------------------------------------------------------------
    #[test]
    fn test_checksum_independence_from_payload_name() {
        let p = Packet {
            seq: 42,
            data: vec![1, 2, 3],
            checksum_field: 0xDEAD_BEEF,
        };
        let wrapped = encode_with_checksum(&p).expect("encode failed");
        let (decoded, _): (Packet, _) = decode_with_checksum(&wrapped).expect("decode failed");
        assert_eq!(p, decoded);
    }

    // -----------------------------------------------------------------------
    // Test 2: single bit flip in the payload is detected
    // -----------------------------------------------------------------------
    #[test]
    fn test_checksum_detects_single_bit_flip() {
        let data = vec![0xFFu8; 1000];
        let wrapped = wrap_with_checksum(&data);

        // Flip a single bit deep in the payload
        let mut corrupted = wrapped.clone();
        corrupted[HEADER_SIZE + 500] ^= 0x01;

        let result = verify_checksum(&corrupted);
        assert!(
            matches!(result, Err(oxicode::Error::ChecksumMismatch { .. })),
            "single bit flip should be detected, got: {:?}",
            result
        );
    }

    // -----------------------------------------------------------------------
    // Test 3: corrupted version byte returns InvalidData (or any Err)
    // -----------------------------------------------------------------------
    #[test]
    fn test_checksum_wrong_version() {
        let mut wrapped = wrap_with_checksum(b"test data");
        // The version byte sits at index 3 (after the 3-byte MAGIC)
        wrapped[3] = 99;
        let result = verify_checksum(&wrapped);
        assert!(result.is_err(), "wrong version should return Err, got Ok");
    }

    // -----------------------------------------------------------------------
    // Test 4: header-only slice (payload truncated) returns UnexpectedEnd
    // -----------------------------------------------------------------------
    #[test]
    fn test_checksum_truncated_at_payload() {
        let data = vec![0u8; 100];
        let wrapped = wrap_with_checksum(&data);
        // Keep only the header bytes – the header says payload is 100 bytes,
        // but we provide none of them.
        let truncated = &wrapped[..HEADER_SIZE];
        let result = verify_checksum(truncated);
        assert!(
            matches!(result, Err(oxicode::Error::UnexpectedEnd { .. })),
            "truncated payload should give UnexpectedEnd, got: {:?}",
            result
        );
    }

    // -----------------------------------------------------------------------
    // Test 5: consumed includes the 16-byte header
    // -----------------------------------------------------------------------
    #[test]
    fn test_consumed_includes_header() {
        let value = "hello world".to_string();
        let wrapped = encode_with_checksum(&value).expect("encode failed");

        let (_, consumed) = decode_with_checksum::<String>(&wrapped).expect("decode failed");

        assert!(
            consumed >= HEADER_SIZE,
            "consumed ({}) must be at least HEADER_SIZE ({})",
            consumed,
            HEADER_SIZE
        );
        assert!(
            consumed <= wrapped.len(),
            "consumed ({}) must not exceed total wrapped len ({})",
            consumed,
            wrapped.len()
        );
    }

    // -----------------------------------------------------------------------
    // Test 6: big-endian config roundtrip
    // -----------------------------------------------------------------------
    #[test]
    fn test_encode_decode_with_big_endian_config() {
        let value = vec![1u32, 2, 3, 4, 5];
        let cfg = config::standard().with_big_endian();

        let wrapped = encode_with_checksum_config(&value, cfg).expect("encode failed");
        let (decoded, _): (Vec<u32>, _) =
            decode_with_checksum_config(&wrapped, cfg).expect("decode failed");

        assert_eq!(value, decoded);
    }

    // -----------------------------------------------------------------------
    // Test 7: large struct roundtrip
    // -----------------------------------------------------------------------
    #[test]
    fn test_checksum_large_struct() {
        #[derive(Debug, PartialEq, Encode, Decode)]
        struct LargeData {
            data: Vec<u64>,
            labels: Vec<String>,
        }

        let large = LargeData {
            data: (0..10_000u64).collect(),
            labels: (0..100).map(|i| format!("label_{}", i)).collect(),
        };

        let wrapped = encode_with_checksum(&large).expect("encode failed");
        let (decoded, _): (LargeData, _) = decode_with_checksum(&wrapped).expect("decode failed");

        assert_eq!(large, decoded);
    }

    // -----------------------------------------------------------------------
    // Test 8: ChecksumMismatch error has a descriptive Display message
    // -----------------------------------------------------------------------
    #[test]
    fn test_display_checksum_mismatch_error() {
        let wrapped = wrap_with_checksum(b"original");
        let mut corrupted = wrapped.clone();
        corrupted[HEADER_SIZE] ^= 0xFF;

        let err = verify_checksum(&corrupted).expect_err("should fail");
        let msg = err.to_string();

        assert!(
            msg.contains("mismatch") || msg.contains("Checksum") || msg.contains("0x"),
            "error message should mention the mismatch clearly, got: {}",
            msg
        );
    }

    // -----------------------------------------------------------------------
    // Test 9: every byte flip in a short payload is detected
    // -----------------------------------------------------------------------
    #[test]
    fn test_all_byte_flips_detected_short_payload() {
        let data = b"short";
        let wrapped = wrap_with_checksum(data);

        for offset in 0..data.len() {
            let mut corrupted = wrapped.clone();
            corrupted[HEADER_SIZE + offset] ^= 0xFF;
            let result = verify_checksum(&corrupted);
            assert!(
                matches!(result, Err(oxicode::Error::ChecksumMismatch { .. })),
                "flip at offset {} should be detected",
                offset
            );
        }
    }

    // -----------------------------------------------------------------------
    // Test 10: raw wrap/verify preserves exact bytes
    // -----------------------------------------------------------------------
    #[test]
    fn test_wrap_verify_preserves_exact_bytes() {
        let original: Vec<u8> = (0u8..=127).collect();
        let wrapped = wrap_with_checksum(&original);
        let payload = verify_checksum(&wrapped).expect("verify failed");
        assert_eq!(payload, original.as_slice());
    }

    // -----------------------------------------------------------------------
    // Test 11: empty payload wraps and verifies successfully
    // -----------------------------------------------------------------------
    #[test]
    fn test_empty_payload_wrap_verify() {
        let wrapped = wrap_with_checksum(b"");
        assert_eq!(wrapped.len(), HEADER_SIZE);
        let payload = verify_checksum(&wrapped).expect("verify of empty payload failed");
        assert_eq!(payload, b"");
    }

    // -----------------------------------------------------------------------
    // Test 12: legacy config roundtrip
    // -----------------------------------------------------------------------
    #[test]
    fn test_encode_decode_with_legacy_config() {
        let value = 0x1234_5678u32;
        let cfg = config::legacy();

        let wrapped = encode_with_checksum_config(&value, cfg).expect("encode failed");
        let (decoded, _): (u32, _) =
            decode_with_checksum_config(&wrapped, cfg).expect("decode failed");

        assert_eq!(value, decoded);
    }

    // -----------------------------------------------------------------------
    // Test 13: corrupted magic returns InvalidData (not a panic)
    // -----------------------------------------------------------------------
    #[test]
    fn test_corrupted_magic_returns_invalid_data() {
        let mut wrapped = encode_with_checksum(&99u64).expect("encode failed");
        // Zero out the first magic byte
        wrapped[0] = 0x00;
        let result = decode_with_checksum::<u64>(&wrapped);
        assert!(result.is_err(), "corrupted magic must return Err");
    }

    // -----------------------------------------------------------------------
    // Test 14: consumed equals wrapped.len() for a clean roundtrip
    // -----------------------------------------------------------------------
    #[test]
    fn test_consumed_equals_wrapped_len() {
        let value = vec![42u8; 64];
        let wrapped = encode_with_checksum(&value).expect("encode failed");
        let (_, consumed) = decode_with_checksum::<Vec<u8>>(&wrapped).expect("decode failed");
        assert_eq!(consumed, wrapped.len());
    }
}
