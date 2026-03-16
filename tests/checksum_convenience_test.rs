//! Tests for the top-level `encode_to_vec_checked` / `decode_from_slice_checked`
//! convenience wrappers that delegate to `oxicode::checksum`.

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
mod checksum_convenience {
    use oxicode::{Decode, Encode};

    // -----------------------------------------------------------------------
    // Shared test type
    // -----------------------------------------------------------------------

    #[derive(Debug, Clone, PartialEq, Encode, Decode)]
    struct Data {
        id: u64,
        payload: Vec<u8>,
        label: String,
    }

    // -----------------------------------------------------------------------
    // Test 1: basic roundtrip with a derived struct
    // -----------------------------------------------------------------------

    #[test]
    fn test_roundtrip_derived_struct() {
        let original = Data {
            id: 42,
            payload: vec![1, 2, 3, 4, 5],
            label: "oxicode".to_string(),
        };
        let encoded =
            oxicode::encode_to_vec_checked(&original).expect("encode_to_vec_checked failed");
        let (decoded, consumed): (Data, usize) =
            oxicode::decode_from_slice_checked(&encoded).expect("decode_from_slice_checked failed");
        assert_eq!(original, decoded);
        assert_eq!(
            consumed,
            encoded.len(),
            "consumed must equal total wrapped length"
        );
    }

    // -----------------------------------------------------------------------
    // Test 2: roundtrip for primitive types
    // -----------------------------------------------------------------------

    #[test]
    fn test_roundtrip_u64() {
        let value: u64 = u64::MAX;
        let enc = oxicode::encode_to_vec_checked(&value).expect("encode failed");
        let (dec, _): (u64, _) = oxicode::decode_from_slice_checked(&enc).expect("decode failed");
        assert_eq!(value, dec);
    }

    #[test]
    fn test_roundtrip_string() {
        let value = "Hello, integrity checking!".to_string();
        let enc = oxicode::encode_to_vec_checked(&value).expect("encode failed");
        let (dec, _): (String, _) =
            oxicode::decode_from_slice_checked(&enc).expect("decode failed");
        assert_eq!(value, dec);
    }

    // -----------------------------------------------------------------------
    // Test 3: empty Vec<u8> payload
    // -----------------------------------------------------------------------

    #[test]
    fn test_empty_vec_payload() {
        let value: Vec<u8> = vec![];
        let enc = oxicode::encode_to_vec_checked(&value).expect("encode failed");
        let (dec, consumed): (Vec<u8>, usize) =
            oxicode::decode_from_slice_checked(&enc).expect("decode failed");
        assert_eq!(value, dec);
        assert_eq!(consumed, enc.len());
    }

    // -----------------------------------------------------------------------
    // Test 4: checksum mismatch is detected (single byte corruption)
    // -----------------------------------------------------------------------

    #[test]
    fn test_checksum_mismatch_detected() {
        use oxicode::checksum::HEADER_SIZE;

        let enc = oxicode::encode_to_vec_checked(&12345u32).expect("encode failed");
        let mut corrupted = enc.clone();
        // Flip a byte in the payload (beyond the 16-byte header)
        corrupted[HEADER_SIZE] ^= 0xFF;

        let result = oxicode::decode_from_slice_checked::<u32>(&corrupted);
        assert!(result.is_err(), "corrupted data must return Err");
        assert!(
            matches!(result, Err(oxicode::Error::ChecksumMismatch { .. })),
            "expected ChecksumMismatch, got: {:?}",
            result
        );
    }

    // -----------------------------------------------------------------------
    // Test 5: corrupted magic header returns an error (not a panic)
    // -----------------------------------------------------------------------

    #[test]
    fn test_corrupted_magic_returns_err() {
        let mut enc = oxicode::encode_to_vec_checked(&42u8).expect("encode failed");
        enc[0] = 0x00; // clobber magic byte 0
        let result = oxicode::decode_from_slice_checked::<u8>(&enc);
        assert!(result.is_err(), "corrupted magic must return Err");
    }

    // -----------------------------------------------------------------------
    // Test 6: truncated data returns UnexpectedEnd
    // -----------------------------------------------------------------------

    #[test]
    fn test_truncated_data_returns_unexpected_end() {
        let result = oxicode::decode_from_slice_checked::<u8>(&[0x4F, 0x58, 0x48]);
        assert!(
            matches!(result, Err(oxicode::Error::UnexpectedEnd { .. })),
            "expected UnexpectedEnd, got: {:?}",
            result
        );
    }

    // -----------------------------------------------------------------------
    // Test 7: nested Vec<Vec<u8>> roundtrip
    // -----------------------------------------------------------------------

    #[test]
    fn test_nested_vec_roundtrip() {
        let value: Vec<Vec<u8>> = vec![
            vec![0, 1, 2],
            vec![],
            vec![255, 128, 64],
            (0..100).collect(),
        ];
        let enc = oxicode::encode_to_vec_checked(&value).expect("encode failed");
        let (dec, _): (Vec<Vec<u8>>, _) =
            oxicode::decode_from_slice_checked(&enc).expect("decode failed");
        assert_eq!(value, dec);
    }

    // -----------------------------------------------------------------------
    // Test 8: output is longer than plain encode_to_vec (header overhead)
    // -----------------------------------------------------------------------

    #[test]
    fn test_output_longer_than_plain_encode() {
        use oxicode::checksum::HEADER_SIZE;

        let value = 100u32;
        let plain = oxicode::encode_to_vec(&value).expect("plain encode failed");
        let checked = oxicode::encode_to_vec_checked(&value).expect("checked encode failed");
        assert_eq!(
            checked.len(),
            plain.len() + HEADER_SIZE,
            "checked output must be exactly HEADER_SIZE bytes longer than plain"
        );
    }

    // -----------------------------------------------------------------------
    // Test 9: encode_to_vec_checked matches checksum::encode_with_checksum
    // -----------------------------------------------------------------------

    #[test]
    fn test_output_identical_to_checksum_module() {
        use oxicode::checksum::encode_with_checksum;

        let value = vec![1u64, 2, 3, 4, 5];
        let via_convenience = oxicode::encode_to_vec_checked(&value).expect("convenience encode");
        let via_module = encode_with_checksum(&value).expect("module encode");
        assert_eq!(
            via_convenience, via_module,
            "encode_to_vec_checked must produce identical bytes to encode_with_checksum"
        );
    }

    // -----------------------------------------------------------------------
    // Test 10: large struct roundtrip
    // -----------------------------------------------------------------------

    #[test]
    fn test_large_struct_roundtrip() {
        let original = Data {
            id: 9_999_999,
            payload: (0u8..=255).cycle().take(10_000).collect(),
            label: "a".repeat(1_000),
        };
        let enc = oxicode::encode_to_vec_checked(&original).expect("encode failed");
        let (dec, consumed): (Data, usize) =
            oxicode::decode_from_slice_checked(&enc).expect("decode failed");
        assert_eq!(original, dec);
        assert_eq!(consumed, enc.len());
    }
}
