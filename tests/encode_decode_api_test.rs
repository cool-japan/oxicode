//! Tests covering the full encode/decode API surface of oxicode.
//!
//! These tests verify round-trip correctness, byte-count accuracy, config variants,
//! writer/reader adapters, checksum-gated paths, and borrow-decode zero-copy behaviour.

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
#[cfg(test)]
mod encode_decode_api_tests {
    use oxicode::{
        borrow_decode_from_slice, decode_from_reader, decode_from_reader_with_config,
        decode_from_slice, decode_from_slice_with_config, decode_value, encode_copy,
        encode_into_slice, encode_to_fixed_array, encode_to_vec, encode_to_vec_with_config,
        encode_to_writer, encode_to_writer_with_config, encoded_size,
    };
    use oxicode::{Decode, Encode};
    use std::f64::consts::{E, PI};
    use std::io::Cursor;

    // -----------------------------------------------------------------------
    // Test 1: encode_to_vec basic u32 roundtrip
    // -----------------------------------------------------------------------
    #[test]
    fn encode_to_vec_basic_u32_roundtrip() {
        let bytes = encode_to_vec(&42u32).expect("encode_to_vec should succeed");
        let (value, _): (u32, _) =
            decode_from_slice(&bytes).expect("decode_from_slice should succeed");
        assert_eq!(value, 42u32);
    }

    // -----------------------------------------------------------------------
    // Test 2: decode_from_slice returns (value, consumed_bytes)
    // -----------------------------------------------------------------------
    #[test]
    fn decode_from_slice_returns_consumed_bytes() {
        let bytes = encode_to_vec(&1234u32).expect("encode");
        let (value, consumed): (u32, _) =
            decode_from_slice(&bytes).expect("decode_from_slice should return tuple");
        assert_eq!(value, 1234u32);
        assert_eq!(
            consumed,
            bytes.len(),
            "consumed should equal total encoded length"
        );
    }

    // -----------------------------------------------------------------------
    // Test 3: encode_to_vec_with_config with standard config
    // -----------------------------------------------------------------------
    #[test]
    fn encode_to_vec_with_config_standard() {
        let config = oxicode::config::standard();
        let bytes = encode_to_vec_with_config(&99u32, config).expect("encode_to_vec_with_config");
        let (value, _): (u32, _) = decode_from_slice(&bytes).expect("decode_from_slice");
        assert_eq!(value, 99u32);
    }

    // -----------------------------------------------------------------------
    // Test 4: decode_from_slice_with_config with standard config
    // -----------------------------------------------------------------------
    #[test]
    fn decode_from_slice_with_config_standard() {
        let config = oxicode::config::standard();
        let bytes = encode_to_vec_with_config(&77u32, config).expect("encode_to_vec_with_config");
        let (value, consumed): (u32, _) =
            decode_from_slice_with_config(&bytes, config).expect("decode_from_slice_with_config");
        assert_eq!(value, 77u32);
        assert_eq!(consumed, bytes.len());
    }

    // -----------------------------------------------------------------------
    // Test 5: encode_into_slice basic usage
    // -----------------------------------------------------------------------
    #[test]
    fn encode_into_slice_basic() {
        let mut buf = [0u8; 32];
        let written = encode_into_slice(42u32, &mut buf, oxicode::config::standard())
            .expect("encode_into_slice should succeed");
        assert!(written > 0, "written bytes should be > 0");
        let (value, _): (u32, _) =
            decode_from_slice(&buf[..written]).expect("decode after encode_into_slice");
        assert_eq!(value, 42u32);
    }

    // -----------------------------------------------------------------------
    // Test 6: encoded_size matches encode_to_vec len for u32
    // -----------------------------------------------------------------------
    #[test]
    fn encoded_size_matches_encode_to_vec_for_u32() {
        let size = encoded_size(&100u32).expect("encoded_size");
        let actual = encode_to_vec(&100u32).expect("encode_to_vec").len();
        assert_eq!(size, actual);
    }

    // -----------------------------------------------------------------------
    // Test 7: encoded_size matches encode_to_vec len for String
    // -----------------------------------------------------------------------
    #[test]
    fn encoded_size_matches_encode_to_vec_for_string() {
        let s = "oxicode test string".to_string();
        let size = encoded_size(&s).expect("encoded_size");
        let actual = encode_to_vec(&s).expect("encode_to_vec").len();
        assert_eq!(size, actual);
    }

    // -----------------------------------------------------------------------
    // Test 8: encoded_size matches encode_to_vec len for Vec<u64>
    // -----------------------------------------------------------------------
    #[test]
    fn encoded_size_matches_encode_to_vec_for_vec_u64() {
        let v: Vec<u64> = vec![1, 2, 3, 4, 5];
        let size = encoded_size(&v).expect("encoded_size");
        let actual = encode_to_vec(&v).expect("encode_to_vec").len();
        assert_eq!(size, actual);
    }

    // -----------------------------------------------------------------------
    // Test 9: encode_copy basic usage
    // -----------------------------------------------------------------------
    #[test]
    fn encode_copy_basic() {
        let bytes = encode_copy(42u32).expect("encode_copy should succeed");
        let (value, _): (u32, _) = decode_from_slice(&bytes).expect("decode after encode_copy");
        assert_eq!(value, 42u32);
    }

    // -----------------------------------------------------------------------
    // Test 10: decode_value basic usage
    // -----------------------------------------------------------------------
    #[test]
    fn decode_value_basic() {
        let encoded = encode_to_vec(&123u64).expect("encode_to_vec");
        let value: u64 = decode_value(&encoded).expect("decode_value should succeed");
        assert_eq!(value, 123u64);
    }

    // -----------------------------------------------------------------------
    // Test 11: encode_to_fixed_array basic usage
    // -----------------------------------------------------------------------
    #[test]
    fn encode_to_fixed_array_basic() {
        let (arr, n): ([u8; 16], _) =
            encode_to_fixed_array(&42u32).expect("encode_to_fixed_array should succeed");
        assert!(n > 0, "written bytes should be > 0");
        assert!(n <= 16, "written bytes should fit in array");
        let (value, _): (u32, _) =
            decode_from_slice(&arr[..n]).expect("decode after encode_to_fixed_array");
        assert_eq!(value, 42u32);
    }

    // -----------------------------------------------------------------------
    // Test 12: encode_to_vec_checked with limit that fits
    // -----------------------------------------------------------------------
    #[cfg(feature = "checksum")]
    #[test]
    fn encode_to_vec_checked_with_limit_that_fits() {
        let bytes =
            oxicode::encode_to_vec_checked(&42u32).expect("encode_to_vec_checked should succeed");
        assert!(!bytes.is_empty(), "encoded bytes should not be empty");
    }

    // -----------------------------------------------------------------------
    // Test 13: decode_from_slice_checked with correct bytes
    // -----------------------------------------------------------------------
    #[cfg(feature = "checksum")]
    #[test]
    fn decode_from_slice_checked_with_correct_bytes() {
        let bytes =
            oxicode::encode_to_vec_checked(&42u32).expect("encode_to_vec_checked should succeed");
        let (value, _): (u32, _) = oxicode::decode_from_slice_checked(&bytes)
            .expect("decode_from_slice_checked should succeed");
        assert_eq!(value, 42u32);
    }

    // -----------------------------------------------------------------------
    // Test 14: encode_to_vec_checked with limit too small -> error
    // -----------------------------------------------------------------------
    #[cfg(feature = "checksum")]
    #[test]
    fn encode_to_vec_checked_with_limit_too_small() {
        let bytes =
            oxicode::encode_to_vec_checked(&42u32).expect("encode_to_vec_checked should succeed");
        // Truncate to only 4 bytes — header alone is 16 bytes, so this must fail
        let truncated = &bytes[..4.min(bytes.len())];
        let result: oxicode::Result<(u32, usize)> = oxicode::decode_from_slice_checked(truncated);
        assert!(result.is_err(), "decode from truncated bytes should fail");
    }

    // -----------------------------------------------------------------------
    // Test 15: decode_from_slice_checked with corrupted bytes -> error
    // -----------------------------------------------------------------------
    #[cfg(feature = "checksum")]
    #[test]
    fn decode_from_slice_checked_with_corrupted_bytes() {
        let mut bytes =
            oxicode::encode_to_vec_checked(&42u32).expect("encode_to_vec_checked should succeed");
        // Flip a byte in the middle of the payload area
        let mid = bytes.len() / 2;
        bytes[mid] ^= 0xFF;
        let result: oxicode::Result<(u32, usize)> = oxicode::decode_from_slice_checked(&bytes);
        assert!(result.is_err(), "decode from corrupted bytes should fail");
    }

    // -----------------------------------------------------------------------
    // Test 16: borrow_decode_from_slice for &str
    // -----------------------------------------------------------------------
    #[test]
    fn borrow_decode_from_slice_for_str() {
        let data = String::from("hello borrow");
        let encoded = encode_to_vec(&data).expect("encode_to_vec");
        let (decoded, _): (&str, _) =
            borrow_decode_from_slice(&encoded).expect("borrow_decode_from_slice for &str");
        assert_eq!(decoded, "hello borrow");
    }

    // -----------------------------------------------------------------------
    // Test 17: encode_to_writer to Cursor<Vec<u8>>
    // -----------------------------------------------------------------------
    #[test]
    fn encode_to_writer_to_cursor() {
        let mut cursor = Cursor::new(Vec::<u8>::new());
        let n = encode_to_writer(&42u32, &mut cursor).expect("encode_to_writer");
        assert!(n > 0, "bytes written should be > 0");
        let inner = cursor.into_inner();
        let (value, _): (u32, _) =
            decode_from_slice(&inner).expect("decode after encode_to_writer");
        assert_eq!(value, 42u32);
    }

    // -----------------------------------------------------------------------
    // Test 18: decode_from_reader from Cursor<Vec<u8>>
    // -----------------------------------------------------------------------
    #[test]
    fn decode_from_reader_from_cursor() {
        let bytes = encode_to_vec(&42u32).expect("encode_to_vec");
        let expected_len = bytes.len();
        let cursor = Cursor::new(bytes);
        let (value, n): (u32, _) =
            decode_from_reader(cursor).expect("decode_from_reader should succeed");
        assert_eq!(value, 42u32);
        assert_eq!(n, expected_len, "bytes_read should equal encoded length");
    }

    // -----------------------------------------------------------------------
    // Test 19: encode_to_writer_with_config to Cursor
    // -----------------------------------------------------------------------
    #[test]
    fn encode_to_writer_with_config_to_cursor() {
        let mut cursor = Cursor::new(Vec::<u8>::new());
        let n = encode_to_writer_with_config(&42u32, &mut cursor, oxicode::config::standard())
            .expect("encode_to_writer_with_config");
        assert!(n > 0, "bytes written should be > 0");
        let inner = cursor.into_inner();
        let (value, _): (u32, _) =
            decode_from_slice(&inner).expect("decode after encode_to_writer_with_config");
        assert_eq!(value, 42u32);
    }

    // -----------------------------------------------------------------------
    // Test 20: decode_from_reader_with_config from Cursor
    // -----------------------------------------------------------------------
    #[test]
    fn decode_from_reader_with_config_from_cursor() {
        let bytes = encode_to_vec(&42u32).expect("encode_to_vec");
        let cursor = Cursor::new(bytes);
        let (value, _): (u32, _) =
            decode_from_reader_with_config(cursor, oxicode::config::standard())
                .expect("decode_from_reader_with_config should succeed");
        assert_eq!(value, 42u32);
    }

    // -----------------------------------------------------------------------
    // Test 21: encode_to_vec_with_size_hint basic usage
    // -----------------------------------------------------------------------
    #[test]
    fn encode_to_vec_with_size_hint_basic() {
        let buf = oxicode::encode_to_vec_with_size_hint(&42u32, 64)
            .expect("encode_to_vec_with_size_hint should succeed");
        assert!(!buf.is_empty(), "encoded buffer should not be empty");
        let expected = encode_to_vec(&42u32).expect("encode_to_vec");
        assert_eq!(buf, expected, "size-hinted encode must match encode_to_vec");
    }

    // -----------------------------------------------------------------------
    // Test 22: API consistency — same struct encodes identically via
    //          encode_to_vec and encode_into_slice
    // -----------------------------------------------------------------------
    #[derive(Encode, Decode, PartialEq, Debug)]
    struct ApiTestPoint {
        x: f64,
        y: f64,
    }

    #[test]
    fn api_consistency_struct_encode_to_vec_vs_encode_into_slice() {
        let vec_bytes =
            encode_to_vec(&ApiTestPoint { x: PI, y: E }).expect("encode_to_vec for ApiTestPoint");

        let mut buf = [0u8; 64];
        let slice_written = encode_into_slice(
            ApiTestPoint { x: PI, y: E },
            &mut buf,
            oxicode::config::standard(),
        )
        .expect("encode_into_slice for ApiTestPoint");

        assert_eq!(
            vec_bytes.as_slice(),
            &buf[..slice_written],
            "encode_to_vec and encode_into_slice must produce identical bytes"
        );
    }
}
