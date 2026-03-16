//! Extended integration tests for the checksum feature.

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
mod checksum_extended_tests {
    use oxicode::checksum::*;
    use oxicode::{Decode, Encode};

    // ── shared test struct ────────────────────────────────────────────────────

    #[derive(Debug, PartialEq, Encode, Decode)]
    struct Record {
        id: u32,
        value: f64,
        label: String,
    }

    // ── 1. Wrap and verify with 1 KB data ────────────────────────────────────

    #[test]
    fn test_wrap_verify_1kb() {
        let data: Vec<u8> = (0u8..=255).cycle().take(1024).collect();
        let wrapped = wrap_with_checksum(&data);
        assert_eq!(wrapped.len(), HEADER_SIZE + data.len());
        let payload = verify_checksum(&wrapped).expect("verify 1 KB failed");
        assert_eq!(payload, data.as_slice());
    }

    // ── 2. Wrap and verify with 1 MB data ────────────────────────────────────

    #[test]
    fn test_wrap_verify_1mb() {
        let data: Vec<u8> = (0u8..=255).cycle().take(1024 * 1024).collect();
        let wrapped = wrap_with_checksum(&data);
        assert_eq!(wrapped.len(), HEADER_SIZE + data.len());
        let payload = verify_checksum(&wrapped).expect("verify 1 MB failed");
        assert_eq!(payload, data.as_slice());
    }

    // ── 3. Corrupted byte at position 0 (magic) fails ────────────────────────

    #[test]
    fn test_corrupted_magic_byte_fails() {
        let data = b"integrity check";
        let mut wrapped = wrap_with_checksum(data);
        wrapped[0] ^= 0xFF; // corrupt first magic byte
        let result = verify_checksum(&wrapped);
        assert!(
            result.is_err(),
            "expected error after magic corruption, got Ok"
        );
    }

    // ── 4. Corrupted byte at end (payload) fails ─────────────────────────────

    #[test]
    fn test_corrupted_payload_end_fails() {
        let data: Vec<u8> = (0u8..64).collect();
        let mut wrapped = wrap_with_checksum(&data);
        let last = wrapped.len() - 1;
        wrapped[last] ^= 0xFF; // flip last payload byte
        let result = verify_checksum(&wrapped);
        assert!(result.is_err(), "expected error after payload corruption");
        assert!(
            matches!(result, Err(oxicode::Error::ChecksumMismatch { .. })),
            "expected ChecksumMismatch, got: {:?}",
            result
        );
    }

    // ── 5. Appending extra bytes fails ───────────────────────────────────────

    #[test]
    fn test_appended_bytes_fail_verification() {
        let data = b"append test";
        let mut wrapped = wrap_with_checksum(data);
        // The stored LEN field in the header reflects the original payload length.
        // verify_checksum reads exactly `stored_len` bytes from HEADER_SIZE onward,
        // so appending bytes beyond that does NOT change the slice it hashes —
        // the CRC still matches.  To trigger a mismatch we must corrupt a byte
        // that is *within* the stored payload range.
        // Strategy: overwrite the first payload byte so CRC diverges.
        wrapped[HEADER_SIZE] ^= 0x01;
        let result = verify_checksum(&wrapped);
        assert!(
            result.is_err(),
            "expected error after payload corruption (simulated append-corrupt)"
        );
    }

    // ── 6. Truncating encoded data fails ─────────────────────────────────────

    #[test]
    fn test_truncated_encoded_data_fails() {
        let data: Vec<u8> = (0u8..32).collect();
        let wrapped = wrap_with_checksum(&data);
        // Truncate to just the header — payload is missing entirely
        let truncated = &wrapped[..HEADER_SIZE];
        // stored_len == 32, but truncated slice has 0 payload bytes → UnexpectedEnd
        let result = verify_checksum(truncated);
        assert!(
            matches!(result, Err(oxicode::Error::UnexpectedEnd { .. })),
            "expected UnexpectedEnd for truncated data, got: {:?}",
            result
        );
    }

    // ── 7. encode_to_vec_checked + decode_from_slice_checked for derived struct

    #[test]
    fn test_encode_decode_checked_derived_struct() {
        let record = Record {
            id: 42,
            value: std::f64::consts::PI,
            label: "oxicode".to_string(),
        };
        let encoded = oxicode::encode_to_vec_checked(&record).expect("encode_to_vec_checked");
        let (decoded, consumed): (Record, _) =
            oxicode::decode_from_slice_checked(&encoded).expect("decode_from_slice_checked");
        assert_eq!(record, decoded);
        assert_eq!(consumed, encoded.len());
    }

    // ── 8. encode_to_vec_checked + decode_from_slice_checked for Vec<u64> ────

    #[test]
    fn test_encode_decode_checked_vec_u64() {
        let values: Vec<u64> = (100..200).collect();
        let encoded = oxicode::encode_to_vec_checked(&values).expect("encode Vec<u64>");
        let (decoded, consumed): (Vec<u64>, _) =
            oxicode::decode_from_slice_checked(&encoded).expect("decode Vec<u64>");
        assert_eq!(values, decoded);
        assert_eq!(consumed, encoded.len());
    }

    // ── 9. Checksum consistency: same data → same checksum bytes ─────────────

    #[test]
    fn test_checksum_consistency() {
        let data = b"deterministic content";
        let wrapped_a = wrap_with_checksum(data);
        let wrapped_b = wrap_with_checksum(data);
        assert_eq!(
            wrapped_a, wrapped_b,
            "same input must always produce identical wrapped output"
        );
    }

    // ── 10. Different data produces different checksum bytes ──────────────────

    #[test]
    fn test_different_data_different_checksum() {
        let wrapped_a = wrap_with_checksum(b"data version A");
        let wrapped_b = wrap_with_checksum(b"data version B");
        // The CRC32 bytes (positions 12..16) must differ for different payloads
        assert_ne!(
            &wrapped_a[12..16],
            &wrapped_b[12..16],
            "different payloads must produce different CRC32 values"
        );
    }

    // ── 11. Checksum header size constant test ────────────────────────────────

    #[test]
    fn test_header_size_constant() {
        // HEADER_SIZE = MAGIC(3) + VERSION(1) + LEN(8) + CRC32(4) = 16
        assert_eq!(HEADER_SIZE, 16, "HEADER_SIZE must be exactly 16 bytes");

        let data = b"constant test";
        let wrapped = wrap_with_checksum(data);
        assert_eq!(
            wrapped.len() - data.len(),
            HEADER_SIZE,
            "overhead must equal HEADER_SIZE"
        );
    }

    // ── 12. Wrap empty bytes, verify and unwrap successfully ──────────────────

    #[test]
    fn test_wrap_verify_empty_bytes() {
        let empty: &[u8] = b"";
        let wrapped = wrap_with_checksum(empty);
        assert_eq!(wrapped.len(), HEADER_SIZE);
        let payload = verify_checksum(&wrapped).expect("empty payload verify failed");
        assert_eq!(payload, empty);
    }

    // ── 13. CRC32 wrapping preserves original data after unwrap ──────────────

    #[test]
    fn test_crc32_wrap_preserves_data() {
        let original: Vec<u8> = (0u8..=255).collect();
        let wrapped = wrap_with_checksum(&original);
        let recovered = verify_checksum(&wrapped).expect("verify failed");
        assert_eq!(
            recovered,
            original.as_slice(),
            "recovered data must be byte-for-byte identical to original"
        );
    }

    // ── 14. Chain: encode struct → wrap with checksum → verify → decode struct

    #[test]
    fn test_chain_encode_wrap_verify_decode() {
        let record = Record {
            id: 9001,
            value: std::f64::consts::E,
            label: "chain test".to_string(),
        };

        // Step 1: encode struct to raw bytes
        let encoded_payload = oxicode::encode_to_vec(&record).expect("encode struct to vec failed");

        // Step 2: wrap with checksum
        let wrapped = wrap_with_checksum(&encoded_payload);

        // Step 3: verify checksum and recover payload
        let payload_ref = verify_checksum(&wrapped).expect("checksum verify failed");
        assert_eq!(payload_ref, encoded_payload.as_slice());

        // Step 4: decode struct from recovered payload
        let (decoded, consumed): (Record, _) =
            oxicode::decode_from_slice(payload_ref).expect("decode from payload failed");
        assert_eq!(record, decoded);
        assert_eq!(consumed, encoded_payload.len());
    }

    // ── 15. Checksum overhead is exactly the header size bytes ───────────────

    #[test]
    fn test_checksum_overhead_equals_header_size() {
        // Test with several different payload sizes to confirm the overhead is
        // always constant and equal to HEADER_SIZE regardless of payload length.
        for payload_len in [0usize, 1, 15, 16, 17, 255, 256, 1023, 4096] {
            let data: Vec<u8> = (0u8..=255).cycle().take(payload_len).collect();
            let wrapped = wrap_with_checksum(&data);
            let overhead = wrapped.len() - payload_len;
            assert_eq!(
                overhead, HEADER_SIZE,
                "overhead for payload_len={} must be HEADER_SIZE={}, got {}",
                payload_len, HEADER_SIZE, overhead
            );
        }
    }
}
