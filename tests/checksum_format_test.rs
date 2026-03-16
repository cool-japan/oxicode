//! Checksum wire format and byte-level tests for OxiCode.
//!
//! Wire format: [MAGIC(3)][VERSION(1)][LEN(8 LE u64)][CRC32(4 LE u32)][PAYLOAD(N)]
//! HEADER_SIZE = 16

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
mod checksum_format_tests {
    use std::f64::consts::PI;

    use oxicode::checksum::{
        decode_with_checksum, decode_with_checksum_config, encode_with_checksum,
        encode_with_checksum_config, verify_checksum, wrap_with_checksum, HEADER_SIZE,
    };
    use oxicode::{config, Decode, Encode};

    // ── shared structs ────────────────────────────────────────────────────────

    #[derive(Debug, Clone, PartialEq, Encode, Decode)]
    struct SimpleStruct {
        id: u32,
        name: String,
    }

    #[derive(Debug, Clone, PartialEq, Encode, Decode)]
    struct FloatRecord {
        value: f64,
    }

    #[derive(Debug, Clone, PartialEq, Encode, Decode)]
    struct WrapA(u32);

    #[derive(Debug, Clone, PartialEq, Encode, Decode)]
    struct WrapB(u32);

    // ── Test 1: checksum header is prepended before payload ───────────────────

    #[test]
    fn test_checksum_bytes_appended_after_data() {
        let value = 0xDEAD_BEEFu32;
        let plain = oxicode::encode_to_vec(&value).expect("encode_to_vec failed");
        let wrapped = encode_with_checksum(&value).expect("encode_with_checksum failed");

        // The first HEADER_SIZE bytes are the header (magic + version + len + crc32)
        assert_eq!(
            &wrapped[HEADER_SIZE..],
            plain.as_slice(),
            "payload bytes after the header must equal the plain-encoded bytes"
        );
    }

    // ── Test 2: checksum adds exactly HEADER_SIZE bytes overhead ─────────────

    #[test]
    fn test_checksum_adds_exactly_header_size_bytes() {
        let value = 12345u64;
        let plain = oxicode::encode_to_vec(&value).expect("encode_to_vec failed");
        let wrapped = encode_with_checksum(&value).expect("encode_with_checksum failed");

        assert_eq!(
            wrapped.len(),
            plain.len() + HEADER_SIZE,
            "wrapped length must be plain length + HEADER_SIZE ({})",
            HEADER_SIZE
        );
    }

    // ── Test 3: different data produces different CRC32 bytes ─────────────────

    #[test]
    fn test_different_data_different_checksum_bytes() {
        let wrapped_a = encode_with_checksum(&111u64).expect("encode 111");
        let wrapped_b = encode_with_checksum(&222u64).expect("encode 222");

        // CRC32 occupies bytes 12..16 of the header
        assert_ne!(
            &wrapped_a[12..16],
            &wrapped_b[12..16],
            "different payloads must produce different CRC32 bytes at header[12..16]"
        );
    }

    // ── Test 4: same data always produces same checksum (deterministic) ────────

    #[test]
    fn test_same_data_deterministic_checksum() {
        let value = SimpleStruct {
            id: 7,
            name: "deterministic".to_string(),
        };
        let first = encode_with_checksum(&value).expect("first encode");
        let second = encode_with_checksum(&value).expect("second encode");

        assert_eq!(
            first, second,
            "same input must always produce identical byte-for-byte output"
        );
    }

    // ── Test 5: struct checksum equals wrap_with_checksum of encode_to_vec ────

    #[test]
    fn test_struct_checksum_equals_wrap_of_encode_to_vec() {
        let value = SimpleStruct {
            id: 42,
            name: "oxicode".to_string(),
        };
        let plain = oxicode::encode_to_vec(&value).expect("encode_to_vec");
        let manually_wrapped = wrap_with_checksum(&plain);
        let via_api = encode_with_checksum(&value).expect("encode_with_checksum");

        assert_eq!(
            manually_wrapped, via_api,
            "manual wrap_with_checksum(encode_to_vec(v)) must equal encode_with_checksum(v)"
        );
    }

    // ── Test 6: CRC32 of empty payload is well-defined (crc32fast::hash([]) == 0) ──

    #[test]
    fn test_crc32_empty_payload_is_defined() {
        let wrapped = wrap_with_checksum(b"");

        // crc32fast::hash(b"") == 0, stored as LE u32 → [0x00, 0x00, 0x00, 0x00]
        assert_eq!(
            &wrapped[12..16],
            &[0x00u8, 0x00, 0x00, 0x00],
            "CRC32 of empty payload must be 0 (stored as 4 LE zero bytes)"
        );

        // Verify the whole header still checks out
        let payload = verify_checksum(&wrapped).expect("empty payload must verify successfully");
        assert_eq!(payload, b"");
    }

    // ── Test 7: corrupt first byte (magic) → verify fails ────────────────────

    #[test]
    fn test_corrupt_first_byte_verify_fails() {
        let wrapped = encode_with_checksum(&9999u32).expect("encode u32");
        let mut corrupted = wrapped.clone();
        corrupted[0] ^= 0xFF; // corrupt first magic byte

        let result = verify_checksum(&corrupted);
        assert!(
            result.is_err(),
            "corrupted magic (byte 0) must cause verify to fail"
        );
    }

    // ── Test 8: corrupt last CRC32 header byte → verify fails ─────────────────

    #[test]
    fn test_corrupt_last_checksum_byte_fails() {
        let wrapped = encode_with_checksum(&0x12345678u32).expect("encode u32");
        let mut corrupted = wrapped.clone();
        // CRC32 field occupies bytes 12..16; the last byte of CRC32 is at index 15
        corrupted[15] ^= 0xFF;

        let result = verify_checksum(&corrupted);
        assert!(
            result.is_err(),
            "corrupting the last CRC32 header byte (index 15) must cause verify to fail"
        );
    }

    // ── Test 9: corrupt middle byte of payload → ChecksumMismatch ────────────

    #[test]
    fn test_corrupt_middle_byte_fails() {
        // Use a value whose encoded payload is long enough to have a "middle"
        let value = SimpleStruct {
            id: 100,
            name: "middle byte corruption test".to_string(),
        };
        let wrapped = encode_with_checksum(&value).expect("encode struct");

        // Compute middle offset in payload region
        let payload_len = wrapped.len() - HEADER_SIZE;
        assert!(
            payload_len >= 3,
            "payload must be at least 3 bytes to have a middle"
        );
        let middle = HEADER_SIZE + payload_len / 2;

        let mut corrupted = wrapped.clone();
        corrupted[middle] ^= 0xAA;

        let result = verify_checksum(&corrupted);
        assert!(
            matches!(result, Err(oxicode::Error::ChecksumMismatch { .. })),
            "corrupting a middle payload byte must return ChecksumMismatch, got: {:?}",
            result
        );
    }

    // ── Test 10: extra bytes at end → consumed < total length ─────────────────

    #[test]
    fn test_extra_bytes_at_end_not_consumed() {
        let value = 55u32;
        let wrapped = encode_with_checksum(&value).expect("encode u32");

        // Append extra garbage bytes
        let mut with_extra = wrapped.clone();
        with_extra.extend_from_slice(&[0xFFu8, 0xFE, 0xFD]);

        // decode_with_checksum reads exactly HEADER_SIZE + stored_len bytes;
        // extra bytes are not consumed
        let (decoded, consumed) = decode_with_checksum::<u32>(&with_extra)
            .expect("decode must succeed even with trailing bytes");
        assert_eq!(decoded, value, "decoded value must match original");
        assert!(
            consumed < with_extra.len(),
            "consumed ({}) must be less than total length ({}) when extra bytes trail",
            consumed,
            with_extra.len()
        );
        assert_eq!(
            consumed,
            wrapped.len(),
            "consumed must equal the original wrapped length"
        );
    }

    // ── Test 11: truncated by 1 byte → UnexpectedEnd ──────────────────────────

    #[test]
    fn test_truncated_by_one_byte_fails() {
        let data = b"ten bytes!"; // exactly 10 bytes
        let wrapped = wrap_with_checksum(data);

        // Remove the last byte — the payload is now 1 byte short
        let truncated = &wrapped[..wrapped.len() - 1];

        let result = verify_checksum(truncated);
        assert!(
            matches!(result, Err(oxicode::Error::UnexpectedEnd { .. })),
            "truncating by 1 byte must return UnexpectedEnd, got: {:?}",
            result
        );
    }

    // ── Test 12: two newtype structs with same value → same checksum bytes ─────

    #[test]
    fn test_two_structs_same_encoding_same_checksum() {
        // WrapA(42) and WrapB(42) encode to the same payload bytes (just a u32).
        // Therefore their CRC32 fields must be identical.
        let a = encode_with_checksum(&WrapA(42)).expect("encode WrapA");
        let b = encode_with_checksum(&WrapB(42)).expect("encode WrapB");

        // Confirm the payloads are identical
        let payload_a = &a[HEADER_SIZE..];
        let payload_b = &b[HEADER_SIZE..];
        assert_eq!(
            payload_a, payload_b,
            "WrapA(42) and WrapB(42) must encode to identical payload bytes"
        );

        // And thus the CRC32 header bytes must be identical
        assert_eq!(
            &a[12..16],
            &b[12..16],
            "identical payloads must produce identical CRC32 bytes"
        );
    }

    // ── Test 13: encode_with_checksum is longer than encode_to_vec by HEADER_SIZE

    #[test]
    fn test_checksum_version_longer_than_plain() {
        let value = vec![1u8, 2, 3, 4, 5];
        let plain = oxicode::encode_to_vec(&value).expect("encode_to_vec");
        let wrapped = encode_with_checksum(&value).expect("encode_with_checksum");

        assert!(
            wrapped.len() > plain.len(),
            "checksum-wrapped output must be longer than plain-encoded output"
        );
        assert_eq!(
            wrapped.len() - plain.len(),
            HEADER_SIZE,
            "the difference must be exactly HEADER_SIZE ({})",
            HEADER_SIZE
        );
    }

    // ── Test 14: checksum with fixed_int config ───────────────────────────────

    #[test]
    fn test_checksum_with_fixed_int_config() {
        let value = 0xABCD_EF01u32;
        let cfg = config::standard().with_fixed_int_encoding();

        let wrapped =
            encode_with_checksum_config(&value, cfg).expect("encode with fixed_int config");
        let (decoded, consumed): (u32, _) =
            decode_with_checksum_config(&wrapped, cfg).expect("decode with fixed_int config");

        assert_eq!(
            value, decoded,
            "fixed_int config roundtrip must preserve value"
        );
        assert_eq!(
            consumed,
            wrapped.len(),
            "consumed must equal wrapped length"
        );
    }

    // ── Test 15: checksum with big_endian config ──────────────────────────────

    #[test]
    fn test_checksum_with_big_endian_config() {
        let value = 0x1122_3344u32;
        let cfg = config::standard().with_big_endian();

        let wrapped =
            encode_with_checksum_config(&value, cfg).expect("encode with big_endian config");
        let (decoded, consumed): (u32, _) =
            decode_with_checksum_config(&wrapped, cfg).expect("decode with big_endian config");

        assert_eq!(
            value, decoded,
            "big_endian config roundtrip must preserve value"
        );
        assert_eq!(
            consumed,
            wrapped.len(),
            "consumed must equal wrapped length"
        );
    }

    // ── Test 16: checksum of Vec<u8> of all zeros ─────────────────────────────

    #[test]
    fn test_checksum_vec_u8_zeros() {
        let zeros: Vec<u8> = vec![0u8; 256];
        let wrapped = encode_with_checksum(&zeros).expect("encode Vec<u8> zeros");
        let (decoded, consumed): (Vec<u8>, _) =
            decode_with_checksum(&wrapped).expect("decode Vec<u8> zeros");

        assert_eq!(zeros, decoded, "all-zero Vec<u8> must survive roundtrip");
        assert_eq!(
            consumed,
            wrapped.len(),
            "consumed must equal wrapped length"
        );
    }

    // ── Test 17: checksum of Vec<u8> of all 255s ──────────────────────────────

    #[test]
    fn test_checksum_vec_u8_all_255() {
        let all_ff: Vec<u8> = vec![0xFFu8; 256];
        let wrapped = encode_with_checksum(&all_ff).expect("encode Vec<u8> all-0xFF");
        let (decoded, consumed): (Vec<u8>, _) =
            decode_with_checksum(&wrapped).expect("decode Vec<u8> all-0xFF");

        assert_eq!(all_ff, decoded, "all-0xFF Vec<u8> must survive roundtrip");
        assert_eq!(
            consumed,
            wrapped.len(),
            "consumed must equal wrapped length"
        );
    }

    // ── Test 18: sequential encode/verify 20 items — all pass ─────────────────

    #[test]
    fn test_sequential_encode_verify_20_items() {
        const N: u32 = 20;

        let blobs: Vec<Vec<u8>> = (0..N)
            .map(|i| encode_with_checksum(&i).expect("sequential encode"))
            .collect();

        for (i, blob) in blobs.iter().enumerate() {
            let (decoded, consumed): (u32, _) =
                decode_with_checksum(blob).expect("sequential decode");
            assert_eq!(
                decoded, i as u32,
                "sequential item {} must decode to correct value",
                i
            );
            assert_eq!(
                consumed,
                blob.len(),
                "consumed must equal blob length at item {}",
                i
            );
        }
    }

    // ── Test 19: checksum of struct with f64 PI field ─────────────────────────

    #[test]
    fn test_checksum_struct_with_f64_pi() {
        let record = FloatRecord { value: PI };
        let wrapped = encode_with_checksum(&record).expect("encode FloatRecord with PI");
        let (decoded, consumed): (FloatRecord, _) =
            decode_with_checksum(&wrapped).expect("decode FloatRecord with PI");

        assert_eq!(
            decoded.value, PI,
            "f64 PI must survive checksum roundtrip exactly (bit-for-bit)"
        );
        assert_eq!(
            consumed,
            wrapped.len(),
            "consumed must equal wrapped length"
        );
    }

    // ── Test 20: re-encode after checksum decode → same bytes ─────────────────

    #[test]
    fn test_reencode_after_checksum_decode_same_bytes() {
        let original = SimpleStruct {
            id: 99,
            name: "re-encode test".to_string(),
        };
        let first_wrapped = encode_with_checksum(&original).expect("first encode");

        // Decode from the first wrapped bytes
        let (decoded, _): (SimpleStruct, _) =
            decode_with_checksum(&first_wrapped).expect("decode after first encode");

        // Re-encode the decoded value
        let second_wrapped = encode_with_checksum(&decoded).expect("re-encode after decode");

        assert_eq!(
            first_wrapped, second_wrapped,
            "re-encoding a decoded value must produce byte-for-byte identical output"
        );
    }
}
