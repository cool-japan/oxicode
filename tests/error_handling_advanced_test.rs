//! Advanced error handling and edge case tests for OxiCode.

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
use oxicode::{config, decode_from_slice, decode_from_slice_with_config, encode_to_vec};

// Enum used by test_invalid_enum_discriminant_fails (must be top-level)
#[derive(Debug, PartialEq, oxicode::Encode, oxicode::Decode)]
enum TwoVariants {
    A,
    B,
}

// ─── Test 1 ──────────────────────────────────────────────────────────────────

#[test]
fn test_truncated_data_fails() {
    let original = 0xDEADBEEFu32;
    let encoded = encode_to_vec(&original).expect("encode u32 failed");
    // Take only half the bytes
    let truncated = &encoded[..encoded.len() / 2];
    if !truncated.is_empty() {
        let result: Result<(u32, _), _> = decode_from_slice(truncated);
        assert!(result.is_err(), "decoding truncated data should fail");
    }
}

// ─── Test 2 ──────────────────────────────────────────────────────────────────

#[test]
fn test_empty_input_fails() {
    let result: Result<(u32, _), _> = decode_from_slice(&[]);
    assert!(result.is_err(), "decoding empty slice should fail");
}

// ─── Test 3 ──────────────────────────────────────────────────────────────────

#[test]
fn test_invalid_bool_byte_fails() {
    // 2 is not a valid bool encoding (only 0 and 1 are valid)
    let result: Result<(bool, _), _> = decode_from_slice(&[2u8]);
    assert!(result.is_err(), "byte 2 should not decode as bool");
}

// ─── Test 4 ──────────────────────────────────────────────────────────────────

#[test]
fn test_limit_exceeded_fails() {
    // The limit applies to the *allocation* count (string/vec length claims).
    // Encode a 50-char String; the decoder claims 50 bytes via claim_bytes_read.
    // A limit of 10 will be exceeded when the 50-byte string length is claimed.
    let original: String = "A".repeat(50);
    let encoded = encode_to_vec(&original).expect("encode String failed");
    let cfg = config::standard().with_limit::<10>();
    let result: Result<(String, _), _> = decode_from_slice_with_config(&encoded, cfg);
    assert!(
        result.is_err(),
        "decoding 50-char string with limit=10 should fail"
    );
}

// ─── Test 5 ──────────────────────────────────────────────────────────────────

#[test]
fn test_limit_exact_succeeds() {
    let original = 42u8;
    let encoded = encode_to_vec(&original).expect("encode u8 failed");
    let encoded_len = encoded.len();
    // A u8 with value 42 encodes to 1 byte in varint; limit=1 is exactly enough
    let cfg = config::standard().with_limit::<1>();
    let result: Result<(u8, _), _> = decode_from_slice_with_config(&encoded, cfg);
    assert!(
        result.is_ok(),
        "decoding with limit equal to encoded size ({encoded_len} byte) should succeed"
    );
    let (decoded, _) = result.expect("decode with exact limit failed");
    assert_eq!(decoded, original);
}

// ─── Test 6 ──────────────────────────────────────────────────────────────────

#[test]
fn test_wrong_type_varint_compatible() {
    // Encode a small u8 value; since OxiCode uses varint, decoding it as u32
    // should succeed when the value fits.
    let original: u8 = 7;
    let encoded = encode_to_vec(&original).expect("encode u8 failed");
    let result: Result<(u32, _), _> = decode_from_slice(&encoded);
    assert!(
        result.is_ok(),
        "small u8-encoded varint should decode successfully as u32"
    );
    let (decoded, _) = result.expect("decode u8 as u32 failed");
    assert_eq!(decoded, 7u32);
}

// ─── Test 7 ──────────────────────────────────────────────────────────────────

#[test]
fn test_extra_bytes_consumed_correctly() {
    let original = 42u32;
    let mut encoded = encode_to_vec(&original).expect("encode u32 failed");
    encoded.push(0xFF); // extra byte
    let (decoded, consumed): (u32, _) =
        decode_from_slice(&encoded).expect("decode with extra bytes failed");
    assert_eq!(decoded, 42u32);
    assert!(
        consumed < encoded.len(),
        "should not consume the extra byte"
    );
}

// ─── Test 8 ──────────────────────────────────────────────────────────────────

#[test]
fn test_string_too_long_for_limit_fails() {
    let long_string: String = "a".repeat(100);
    let encoded = encode_to_vec(&long_string).expect("encode string failed");
    // Limit of 10 bytes is too small for a 100-char string
    let cfg = config::standard().with_limit::<10>();
    let result: Result<(String, _), _> = decode_from_slice_with_config(&encoded, cfg);
    assert!(
        result.is_err(),
        "decoding 100-char string with limit=10 should fail"
    );
}

// ─── Test 9 ──────────────────────────────────────────────────────────────────

#[test]
fn test_invalid_utf8_string_fails() {
    // Construct: varint(3) + [0xFF, 0xFF, 0xFF] (invalid UTF-8)
    let bad_bytes = vec![3u8, 0xFF, 0xFF, 0xFF];
    let result: Result<(String, _), _> = decode_from_slice(&bad_bytes);
    assert!(
        result.is_err(),
        "invalid UTF-8 should fail to decode as String"
    );
}

// ─── Test 10 ─────────────────────────────────────────────────────────────────

#[test]
fn test_vec_too_long_for_limit_fails() {
    let big_vec: Vec<u32> = (0u32..100).collect();
    let encoded = encode_to_vec(&big_vec).expect("encode Vec<u32> failed");
    // Limit of 8 bytes is too small for 100 u32 values
    let cfg = config::standard().with_limit::<8>();
    let result: Result<(Vec<u32>, _), _> = decode_from_slice_with_config(&encoded, cfg);
    assert!(
        result.is_err(),
        "decoding Vec<u32> with 100 elements and limit=8 should fail"
    );
}

// ─── Test 11 ─────────────────────────────────────────────────────────────────

#[test]
fn test_nested_vec_tight_limit_fails() {
    let nested: Vec<Vec<u8>> = vec![vec![1, 2, 3], vec![4, 5, 6], vec![7, 8, 9]];
    let encoded = encode_to_vec(&nested).expect("encode nested Vec failed");
    // Use a limit that is far too small for the nested structure
    let cfg = config::standard().with_limit::<4>();
    let result: Result<(Vec<Vec<u8>>, _), _> = decode_from_slice_with_config(&encoded, cfg);
    assert!(
        result.is_err(),
        "decoding nested Vec<Vec<u8>> with limit=4 should fail"
    );
}

// ─── Test 12 ─────────────────────────────────────────────────────────────────

#[test]
fn test_unit_struct_encode_succeeds() {
    #[derive(Debug, PartialEq, oxicode::Encode, oxicode::Decode)]
    struct ZeroSized;

    let unit = ZeroSized;
    let encoded = encode_to_vec(&unit).expect("encode unit struct failed");
    // Unit struct should encode without error; just verify it produces bytes or empty
    let _ = encoded;
}

// ─── Test 13 ─────────────────────────────────────────────────────────────────

#[test]
fn test_unit_struct_decode_succeeds() {
    #[derive(Debug, PartialEq, oxicode::Encode, oxicode::Decode)]
    struct ZeroSized;

    let unit = ZeroSized;
    let encoded = encode_to_vec(&unit).expect("encode unit struct failed");
    let (decoded, _): (ZeroSized, _) =
        decode_from_slice(&encoded).expect("decode unit struct failed");
    assert_eq!(unit, decoded);
}

// ─── Test 14 ─────────────────────────────────────────────────────────────────

#[test]
fn test_multiple_decodes_from_concatenated_encodings() {
    let a = 10u32;
    let b = 20u32;
    let c = 30u32;

    let mut concatenated = encode_to_vec(&a).expect("encode a failed");
    concatenated.extend(encode_to_vec(&b).expect("encode b failed"));
    concatenated.extend(encode_to_vec(&c).expect("encode c failed"));

    let (decoded_a, n1): (u32, _) = decode_from_slice(&concatenated).expect("decode a failed");
    let (decoded_b, n2): (u32, _) =
        decode_from_slice(&concatenated[n1..]).expect("decode b failed");
    let (decoded_c, _): (u32, _) =
        decode_from_slice(&concatenated[n1 + n2..]).expect("decode c failed");

    assert_eq!(decoded_a, a);
    assert_eq!(decoded_b, b);
    assert_eq!(decoded_c, c);
}

// ─── Test 15 ─────────────────────────────────────────────────────────────────

#[test]
fn test_roundtrip_with_limit_equal_to_encoded_size() {
    let original = 99u8;
    let encoded = encode_to_vec(&original).expect("encode u8 failed");
    // A u8 with value 99 encodes to 1 byte; limit=1 should be exactly enough
    let cfg = config::standard().with_limit::<1>();
    let (decoded, _): (u8, _) =
        decode_from_slice_with_config(&encoded, cfg).expect("decode with exact limit failed");
    assert_eq!(decoded, original);
}

// ─── Test 16 ─────────────────────────────────────────────────────────────────

#[test]
fn test_roundtrip_with_limit_one_less_than_encoded_size_fails() {
    // The limit applies to the allocation byte count (claim_bytes_read).
    // Encode a Vec<u8> with exactly 20 bytes; the decoder claims 20 bytes.
    // A limit of 19 is one less than the payload size and should fail.
    let original: Vec<u8> = (0u8..20).collect();
    let encoded = encode_to_vec(&original).expect("encode Vec<u8> failed");
    let cfg = config::standard().with_limit::<19>();
    let result: Result<(Vec<u8>, _), _> = decode_from_slice_with_config(&encoded, cfg);
    assert!(
        result.is_err(),
        "limit 19 for a 20-byte Vec<u8> payload should fail"
    );
}

// ─── Test 17 ─────────────────────────────────────────────────────────────────

#[test]
fn test_corrupted_first_byte_fails() {
    let original = 0xDEADBEEFu32;
    let mut encoded = encode_to_vec(&original).expect("encode u32 failed");
    // Corrupt the first byte by flipping all bits
    encoded[0] ^= 0xFF;
    let result: Result<(u32, _), _> = decode_from_slice(&encoded);
    // Corrupted varint may return an error or a different value; either is acceptable.
    // The important invariant: it must NOT silently return the original value.
    if let Ok((decoded, _)) = result {
        assert_ne!(
            decoded, original,
            "corrupted first byte must not decode to original value"
        );
    }
}

// ─── Test 18 ─────────────────────────────────────────────────────────────────

#[test]
fn test_corrupted_length_byte_for_vec_fails() {
    let original: Vec<u8> = vec![1, 2, 3, 4, 5];
    let mut encoded = encode_to_vec(&original).expect("encode Vec<u8> failed");
    // The first byte encodes the length; set it to 0x7F to claim a very large length
    encoded[0] = 0x7F;
    let result: Result<(Vec<u8>, _), _> = decode_from_slice(&encoded);
    // Claiming a length (127) that far exceeds the available data must fail
    assert!(
        result.is_err(),
        "corrupted length byte claiming 127 elements should cause decode failure"
    );
}

// ─── Test 19 ─────────────────────────────────────────────────────────────────

#[test]
fn test_invalid_enum_discriminant_fails() {
    // Variant 2 doesn't exist in TwoVariants (only 0 and 1 are valid)
    let bad = vec![2u8]; // varint(2) = discriminant 2
    let result: Result<(TwoVariants, _), _> = decode_from_slice(&bad);
    assert!(
        result.is_err(),
        "discriminant 2 should fail for TwoVariants"
    );
}

// ─── Test 20 ─────────────────────────────────────────────────────────────────

#[test]
fn test_decode_error_display_is_non_empty() {
    let result: Result<(u32, _), _> = decode_from_slice(&[]);
    let err = result.expect_err("expected error for empty input");
    let msg = format!("{err}");
    assert!(
        !msg.is_empty(),
        "error Display must produce a non-empty string"
    );
}

// ─── Test 21 ─────────────────────────────────────────────────────────────────

#[test]
fn test_deeply_nested_option_roundtrip() {
    let value: Option<Option<Option<u32>>> = Some(Some(Some(42)));
    let encoded = encode_to_vec(&value).expect("encode nested Option failed");
    let (decoded, _): (Option<Option<Option<u32>>>, _) =
        decode_from_slice(&encoded).expect("decode nested Option failed");
    assert_eq!(decoded, value);
}

// ─── Test 22 ─────────────────────────────────────────────────────────────────

#[test]
fn test_option_of_result_roundtrip() {
    // Some(Ok(...))
    let v1: Option<Result<u32, String>> = Some(Ok(1234));
    let enc1 = encode_to_vec(&v1).expect("encode Some(Ok) failed");
    let (dec1, _): (Option<Result<u32, String>>, _) =
        decode_from_slice(&enc1).expect("decode Some(Ok) failed");
    assert_eq!(dec1, v1);

    // Some(Err(...))
    let v2: Option<Result<u32, String>> = Some(Err("oops".to_string()));
    let enc2 = encode_to_vec(&v2).expect("encode Some(Err) failed");
    let (dec2, _): (Option<Result<u32, String>>, _) =
        decode_from_slice(&enc2).expect("decode Some(Err) failed");
    assert_eq!(dec2, v2);

    // None
    let v3: Option<Result<u32, String>> = None;
    let enc3 = encode_to_vec(&v3).expect("encode None failed");
    let (dec3, _): (Option<Result<u32, String>>, _) =
        decode_from_slice(&enc3).expect("decode None failed");
    assert_eq!(dec3, v3);
}
