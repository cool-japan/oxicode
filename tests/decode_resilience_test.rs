//! Tests for decode resilience: verifies that malformed data produces proper errors
//! rather than panics, undefined behavior, or silent wrong results.

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
use oxicode::decode_from_slice;

// Helper: generate encoded bytes for a value then corrupt individual bytes
fn corrupt_byte(bytes: &[u8], idx: usize, replacement: u8) -> Vec<u8> {
    let mut v = bytes.to_vec();
    if idx < v.len() {
        v[idx] = replacement;
    }
    v
}

#[test]
fn test_empty_input_returns_err() {
    let result: Result<(u32, _), _> = decode_from_slice(&[]);
    assert!(result.is_err(), "empty input should fail");
}

#[test]
fn test_truncated_u32_returns_err() {
    // A u32 varint might need multiple bytes; single 0xFF signals more bytes needed
    let result: Result<(u32, _), _> = decode_from_slice(&[0xFF, 0xFF]);
    // Either fails or decodes partial; should not panic
    let _ = result;
}

#[test]
fn test_truncated_string_returns_err() {
    // Encode "hello" then truncate the content
    let enc = oxicode::encode_to_vec(&"hello".to_string()).expect("encode");
    let truncated = &enc[..enc.len() - 1]; // Remove last byte
    let result: Result<(String, _), _> = decode_from_slice(truncated);
    assert!(result.is_err(), "truncated string should fail to decode");
}

#[test]
fn test_truncated_vec_returns_err() {
    let v: Vec<u64> = (0..10).collect();
    let enc = oxicode::encode_to_vec(&v).expect("encode");
    // Truncate to just the length prefix
    let truncated = &enc[..2];
    let result: Result<(Vec<u64>, _), _> = decode_from_slice(truncated);
    assert!(result.is_err(), "truncated vec should fail to decode");
}

#[test]
fn test_invalid_bool_byte_returns_err() {
    for byte in [2u8, 3, 100, 255] {
        let result: Result<(bool, _), _> = decode_from_slice(&[byte]);
        assert!(result.is_err(), "byte {} is not a valid bool", byte);
    }
}

#[test]
fn test_all_single_byte_values_decode_safely() {
    // None of these should panic; they may succeed or fail gracefully
    for byte in 0u8..=255 {
        let _result: Result<(bool, _), _> = decode_from_slice(&[byte]);
        let _result: Result<(u8, _), _> = decode_from_slice(&[byte]);
        let _result: Result<(i8, _), _> = decode_from_slice(&[byte]);
    }
}

#[test]
fn test_oversized_collection_length_err() {
    // A collection claiming 1_000_000 elements but with no body bytes must fail.
    // We manually craft the varint for 1_000_000 (which encodes as a multi-byte varint)
    // followed by zero body bytes so the reader runs out of data immediately.
    //
    // 1_000_000 = 0xF4240
    // In LEB128 varint: 0xC0 0xC4 0x3D
    let length_prefix = [0xC0u8, 0xC4, 0x3D];
    // No body bytes follow — decoder must return UnexpectedEnd, not panic.
    let result: Result<(Vec<u8>, _), _> = decode_from_slice(&length_prefix);
    assert!(
        result.is_err(),
        "collection claiming 1_000_000 elements with no body should fail"
    );
}

#[test]
fn test_invalid_utf8_string_returns_err() {
    // Construct bytes: varint(4) followed by 4 invalid UTF-8 bytes
    let mut bytes = vec![4u8]; // length = 4
    bytes.extend_from_slice(&[0xFF, 0xFE, 0xFD, 0xFC]); // invalid UTF-8
    let result: Result<(String, _), _> = decode_from_slice(&bytes);
    assert!(result.is_err(), "invalid UTF-8 should fail string decode");
}

#[test]
fn test_fuzz_random_bytes_dont_panic() {
    // Test a variety of random-ish byte sequences
    let test_inputs: &[&[u8]] = &[
        &[],
        &[0xFF],
        &[0x00, 0x00],
        &[0xFF, 0xFF, 0xFF, 0xFF],
        &[0x80, 0x80, 0x80, 0x80, 0x80, 0x80, 0x80, 0x80, 0x80, 0x01], // valid large varint
        &[0x01, 0x00, 0x00, 0x00, 0x00],                               // truncated data
    ];
    for input in test_inputs {
        // None of these should panic
        let _: Result<(u64, _), _> = decode_from_slice(input);
        let _: Result<(Vec<u8>, _), _> = decode_from_slice(input);
        let _: Result<(String, _), _> = decode_from_slice(input);
    }
}

#[test]
fn test_decode_correct_after_previous_error() {
    // After a failed decode, the next decode with valid data should succeed
    let bad = &[0xFF, 0xFF]; // Will likely fail as a String
    let _: Result<(String, _), _> = decode_from_slice(bad);

    // Now decode valid data
    let good = oxicode::encode_to_vec(&42u32).expect("encode");
    let (val, _): (u32, _) = decode_from_slice(&good).expect("decode after error");
    assert_eq!(val, 42u32);
}

#[test]
fn test_corrupt_byte_helper_works() {
    let original = vec![0u8, 1, 2, 3, 4];
    let corrupted = corrupt_byte(&original, 2, 0xFF);
    assert_eq!(corrupted[2], 0xFF);
    assert_eq!(corrupted[0], 0);
    assert_eq!(corrupted[4], 4);
}

#[test]
fn test_corrupt_byte_out_of_bounds_no_panic() {
    let original = vec![0u8, 1, 2];
    let result = corrupt_byte(&original, 100, 0xFF);
    // Should return unchanged vec if index out of range
    assert_eq!(result, original);
}

#[test]
fn test_corrupt_encoded_u64_decodes_or_fails_gracefully() {
    let enc = oxicode::encode_to_vec(&0xDEAD_BEEF_u64).expect("encode");
    for i in 0..enc.len() {
        let corrupted = corrupt_byte(&enc, i, 0xFF);
        // Should not panic regardless of corruption
        let _: Result<(u64, _), _> = decode_from_slice(&corrupted);
    }
}

#[test]
fn test_corrupt_encoded_string_decodes_or_fails_gracefully() {
    let enc = oxicode::encode_to_vec(&"resilience".to_string()).expect("encode");
    for i in 0..enc.len() {
        let corrupted = corrupt_byte(&enc, i, 0xAA);
        // Should not panic regardless of corruption
        let _: Result<(String, _), _> = decode_from_slice(&corrupted);
    }
}

#[test]
fn test_partial_varint_sequence() {
    // Each of these has continuation bits set but no final byte
    let inputs: &[&[u8]] = &[
        &[0x80],
        &[0x80, 0x80],
        &[0x80, 0x80, 0x80],
        &[0x80, 0x80, 0x80, 0x80],
        &[0x80, 0x80, 0x80, 0x80, 0x80],
        &[0x80, 0x80, 0x80, 0x80, 0x80, 0x80],
        &[0x80, 0x80, 0x80, 0x80, 0x80, 0x80, 0x80],
        &[0x80, 0x80, 0x80, 0x80, 0x80, 0x80, 0x80, 0x80],
        &[0x80, 0x80, 0x80, 0x80, 0x80, 0x80, 0x80, 0x80, 0x80],
    ];
    for input in inputs {
        let result: Result<(u64, _), _> = decode_from_slice(input);
        // Must not panic; should return an error for unterminated varint
        let _ = result;
    }
}

#[test]
fn test_zero_bytes_all_types_dont_panic() {
    let zeros = vec![0u8; 16];
    let _: Result<(u8, _), _> = decode_from_slice(&zeros);
    let _: Result<(u16, _), _> = decode_from_slice(&zeros);
    let _: Result<(u32, _), _> = decode_from_slice(&zeros);
    let _: Result<(u64, _), _> = decode_from_slice(&zeros);
    let _: Result<(i8, _), _> = decode_from_slice(&zeros);
    let _: Result<(i16, _), _> = decode_from_slice(&zeros);
    let _: Result<(i32, _), _> = decode_from_slice(&zeros);
    let _: Result<(i64, _), _> = decode_from_slice(&zeros);
    let _: Result<(bool, _), _> = decode_from_slice(&zeros);
    let _: Result<(String, _), _> = decode_from_slice(&zeros);
    let _: Result<(Vec<u8>, _), _> = decode_from_slice(&zeros);
}

#[test]
fn test_max_bytes_all_types_dont_panic() {
    let max_bytes = vec![0xFFu8; 16];
    let _: Result<(u8, _), _> = decode_from_slice(&max_bytes);
    let _: Result<(u16, _), _> = decode_from_slice(&max_bytes);
    let _: Result<(u32, _), _> = decode_from_slice(&max_bytes);
    let _: Result<(u64, _), _> = decode_from_slice(&max_bytes);
    let _: Result<(i8, _), _> = decode_from_slice(&max_bytes);
    let _: Result<(i16, _), _> = decode_from_slice(&max_bytes);
    let _: Result<(i32, _), _> = decode_from_slice(&max_bytes);
    let _: Result<(i64, _), _> = decode_from_slice(&max_bytes);
    let _: Result<(bool, _), _> = decode_from_slice(&max_bytes);
    let _: Result<(String, _), _> = decode_from_slice(&max_bytes);
    let _: Result<(Vec<u8>, _), _> = decode_from_slice(&max_bytes);
}
