//! Integration tests for checksum feature

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
mod checksum_tests {
    use oxicode::checksum::{
        decode_with_checksum, encode_with_checksum, verify_checksum, wrap_with_checksum,
        HEADER_SIZE,
    };
    use oxicode::{Decode, Encode};

    #[derive(Debug, PartialEq, Encode, Decode)]
    struct Point {
        x: f32,
        y: f32,
    }

    #[test]
    fn test_encode_decode_roundtrip() {
        let point = Point { x: 1.5, y: 2.5 };
        let wrapped = encode_with_checksum(&point).expect("encode failed");
        let (decoded, consumed): (Point, _) =
            decode_with_checksum(&wrapped).expect("decode failed");
        assert_eq!(point, decoded);
        assert_eq!(consumed, wrapped.len());
    }

    #[test]
    fn test_encode_decode_primitives() {
        let values: Vec<u64> = (0..100).collect();
        let wrapped = encode_with_checksum(&values).expect("encode failed");
        let (decoded, _): (Vec<u64>, _) = decode_with_checksum(&wrapped).expect("decode failed");
        assert_eq!(values, decoded);
    }

    #[test]
    fn test_checksum_mismatch_error() {
        let wrapped = encode_with_checksum(&42u32).expect("encode failed");
        let mut corrupted = wrapped.clone();
        // Corrupt the payload (after the 16-byte header)
        corrupted[HEADER_SIZE] ^= 0xFF;
        let result: Result<(u32, usize), _> = decode_with_checksum(&corrupted);
        assert!(result.is_err());
        // Should be ChecksumMismatch
        if let Err(oxicode::Error::ChecksumMismatch { .. }) = result {
            // expected
        } else {
            panic!("expected ChecksumMismatch, got: {:?}", result);
        }
    }

    #[test]
    fn test_wrong_magic_error() {
        let mut wrapped = encode_with_checksum(&42u32).expect("encode failed");
        wrapped[0] = 0xFF;
        let result: Result<(u32, usize), _> = decode_with_checksum(&wrapped);
        assert!(result.is_err());
    }

    #[test]
    fn test_truncated_data_error() {
        let result = verify_checksum(&[0x4F, 0x58, 0x48]);
        assert!(matches!(result, Err(oxicode::Error::UnexpectedEnd { .. })));
    }

    #[test]
    fn test_empty_vec() {
        let value: Vec<u8> = vec![];
        let wrapped = encode_with_checksum(&value).expect("encode failed");
        let (decoded, _): (Vec<u8>, _) = decode_with_checksum(&wrapped).expect("decode failed");
        assert_eq!(value, decoded);
    }

    #[test]
    fn test_with_config() {
        use oxicode::checksum::{decode_with_checksum_config, encode_with_checksum_config};
        use oxicode::config;

        let value = 42u32;
        let wrapped = encode_with_checksum_config(&value, config::legacy()).expect("encode failed");
        let (decoded, _): (u32, _) =
            decode_with_checksum_config(&wrapped, config::legacy()).expect("decode failed");
        assert_eq!(value, decoded);
    }

    #[test]
    fn test_wrap_verify_raw() {
        let data = b"test payload";
        let wrapped = wrap_with_checksum(data);
        assert_eq!(wrapped.len(), HEADER_SIZE + data.len());
        let payload = verify_checksum(&wrapped).expect("verify failed");
        assert_eq!(payload, data);
    }

    #[test]
    fn test_consumed_bytes_correct() {
        let value = 42u32;
        let wrapped = encode_with_checksum(&value).expect("encode failed");
        let (_, consumed) = decode_with_checksum::<u32>(&wrapped).expect("decode failed");
        // consumed should equal wrapped.len() since we're decoding from the start
        assert_eq!(consumed, wrapped.len());
    }

    #[test]
    fn test_large_payload_checksum_roundtrip() {
        let data: Vec<u8> = (0u8..=255).cycle().take(50_000).collect();
        let wrapped = encode_with_checksum(&data).expect("encode");
        let (decoded, consumed): (Vec<u8>, _) = decode_with_checksum(&wrapped).expect("decode");
        assert_eq!(data, decoded);
        assert_eq!(consumed, wrapped.len());
    }

    #[test]
    fn test_checksum_string_roundtrip() {
        let s = "Hello, oxicode checksum!".to_string();
        let wrapped = encode_with_checksum(&s).expect("encode");
        let (decoded, _): (String, _) = decode_with_checksum(&wrapped).expect("decode");
        assert_eq!(s, decoded);
    }

    #[test]
    fn test_header_corruption_detection() {
        let wrapped = encode_with_checksum(&999u64).expect("encode");
        let mut corrupted = wrapped.clone();
        // Corrupt the checksum bytes (bytes 8..16 in header)
        corrupted[8] ^= 0xAB;
        corrupted[9] ^= 0xCD;
        let result: Result<(u64, usize), _> = decode_with_checksum(&corrupted);
        assert!(result.is_err());
    }
}
