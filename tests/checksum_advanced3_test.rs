//! Advanced checksum/CRC32 tests for OxiCode — exactly 22 top-level #[test] functions.
//!
//! Compile with: cargo test --features checksum --test checksum_advanced3_test

#![cfg(feature = "checksum")]
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
use oxicode::checksum::{
    decode_with_checksum, encode_with_checksum, verify_checksum, wrap_with_checksum, HEADER_SIZE,
};
use oxicode::{Decode, Encode};

// ---------------------------------------------------------------------------
// Shared helper type
// ---------------------------------------------------------------------------

#[derive(Debug, PartialEq, Encode, Decode)]
struct Message {
    id: u32,
    text: String,
    priority: u8,
}

// ---------------------------------------------------------------------------
// Test 1: wrap_with_checksum adds HEADER_SIZE bytes to encoded bytes
// ---------------------------------------------------------------------------
#[test]
fn test_wrap_with_checksum_adds_header_bytes() {
    let payload = b"oxicode test payload";
    let wrapped = wrap_with_checksum(payload);
    assert_eq!(
        wrapped.len(),
        payload.len() + HEADER_SIZE,
        "wrapped length must be payload + HEADER_SIZE ({} bytes)",
        HEADER_SIZE
    );
    // Magic bytes must be present at the start ("OXH")
    assert_eq!(
        &wrapped[..3],
        &[0x4F, 0x58, 0x48],
        "magic bytes must be OXH"
    );
}

// ---------------------------------------------------------------------------
// Test 2: verify_checksum succeeds on correct bytes
// ---------------------------------------------------------------------------
#[test]
fn test_verify_checksum_succeeds_on_correct_bytes() {
    let payload = b"integrity check";
    let wrapped = wrap_with_checksum(payload);
    let result = verify_checksum(&wrapped);
    assert!(
        result.is_ok(),
        "verify_checksum must succeed on correct bytes"
    );
    let recovered = result.expect("verify_checksum failed unexpectedly");
    assert_eq!(recovered, payload, "recovered payload must equal original");
}

// ---------------------------------------------------------------------------
// Test 3: verify_checksum fails on corrupted bytes (flip one byte in payload)
// ---------------------------------------------------------------------------
#[test]
fn test_verify_checksum_fails_on_corrupted_bytes() {
    let payload = b"data to corrupt";
    let mut wrapped = wrap_with_checksum(payload);
    // Flip a byte in the middle of the payload region
    let mid = HEADER_SIZE + payload.len() / 2;
    wrapped[mid] ^= 0xFF;
    let result = verify_checksum(&wrapped);
    assert!(
        result.is_err(),
        "verify_checksum must fail on corrupted bytes, got Ok"
    );
    assert!(
        matches!(result, Err(oxicode::Error::ChecksumMismatch { .. })),
        "error must be ChecksumMismatch, got: {:?}",
        result
    );
}

// ---------------------------------------------------------------------------
// Test 4: encode_with_checksum + decode_with_checksum u32 roundtrip
// ---------------------------------------------------------------------------
#[test]
fn test_encode_decode_u32_roundtrip() {
    let value: u32 = 3_141_592_653;
    let encoded = encode_with_checksum(&value).expect("encode u32 failed");
    let (decoded, consumed): (u32, _) = decode_with_checksum(&encoded).expect("decode u32 failed");
    assert_eq!(decoded, value, "decoded u32 must equal original");
    assert_eq!(
        consumed,
        encoded.len(),
        "consumed must equal total encoded length"
    );
}

// ---------------------------------------------------------------------------
// Test 5: encode_with_checksum + decode_with_checksum String roundtrip
// ---------------------------------------------------------------------------
#[test]
fn test_encode_decode_string_roundtrip() {
    let value = String::from("checksum advanced3 string test");
    let encoded = encode_with_checksum(&value).expect("encode String failed");
    let (decoded, consumed): (String, _) =
        decode_with_checksum(&encoded).expect("decode String failed");
    assert_eq!(decoded, value, "decoded String must equal original");
    assert_eq!(
        consumed,
        encoded.len(),
        "consumed must equal total encoded length"
    );
}

// ---------------------------------------------------------------------------
// Test 6: encode_with_checksum + decode_with_checksum Vec<u8> roundtrip
// ---------------------------------------------------------------------------
#[test]
fn test_encode_decode_vec_u8_roundtrip() {
    let value: Vec<u8> = vec![0u8, 1, 127, 128, 200, 255];
    let encoded = encode_with_checksum(&value).expect("encode Vec<u8> failed");
    let (decoded, consumed): (Vec<u8>, _) =
        decode_with_checksum(&encoded).expect("decode Vec<u8> failed");
    assert_eq!(decoded, value, "decoded Vec<u8> must equal original");
    assert_eq!(
        consumed,
        encoded.len(),
        "consumed must equal total encoded length"
    );
}

// ---------------------------------------------------------------------------
// Test 7: encode_with_checksum + decode_with_checksum Message struct roundtrip
// ---------------------------------------------------------------------------
#[test]
fn test_encode_decode_message_struct_roundtrip() {
    let value = Message {
        id: 42,
        text: String::from("hello from oxicode"),
        priority: 7,
    };
    let encoded = encode_with_checksum(&value).expect("encode Message failed");
    let (decoded, consumed): (Message, _) =
        decode_with_checksum(&encoded).expect("decode Message failed");
    assert_eq!(decoded, value, "decoded Message must equal original");
    assert_eq!(
        consumed,
        encoded.len(),
        "consumed must equal total encoded length"
    );
}

// ---------------------------------------------------------------------------
// Test 8: wrapped bytes are larger than unwrapped bytes by exactly HEADER_SIZE
// ---------------------------------------------------------------------------
#[test]
fn test_wrapped_bytes_larger_than_unwrapped_by_header_size() {
    let value: u32 = 999;
    let plain = oxicode::encode_to_vec(&value).expect("plain encode failed");
    let checked = encode_with_checksum(&value).expect("checksum encode failed");
    assert!(
        checked.len() > plain.len(),
        "checksum-encoded ({} bytes) must exceed plain ({} bytes)",
        checked.len(),
        plain.len()
    );
    assert_eq!(
        checked.len() - plain.len(),
        HEADER_SIZE,
        "size difference must be exactly HEADER_SIZE ({}) bytes",
        HEADER_SIZE
    );
}

// ---------------------------------------------------------------------------
// Test 9: verify_checksum on unmodified data succeeds
// ---------------------------------------------------------------------------
#[test]
fn test_verify_checksum_unmodified_data_succeeds() {
    let value: u64 = 123_456_789_012_345;
    let encoded = encode_with_checksum(&value).expect("encode u64 failed");
    let result = verify_checksum(&encoded);
    assert!(
        result.is_ok(),
        "verify_checksum must succeed on unmodified encoded data"
    );
}

// ---------------------------------------------------------------------------
// Test 10: verify_checksum on data with last byte changed fails
// ---------------------------------------------------------------------------
#[test]
fn test_verify_checksum_fails_last_byte_changed() {
    let value: u32 = 0xDEAD_BEEF;
    let mut encoded = encode_with_checksum(&value).expect("encode failed");
    let last_idx = encoded.len() - 1;
    encoded[last_idx] ^= 0x01;
    let result = verify_checksum(&encoded);
    assert!(
        result.is_err(),
        "verify_checksum must fail when last byte is changed"
    );
    assert!(
        matches!(result, Err(oxicode::Error::ChecksumMismatch { .. })),
        "error must be ChecksumMismatch, got: {:?}",
        result
    );
}

// ---------------------------------------------------------------------------
// Test 11: verify_checksum on data with first payload byte changed fails
// ---------------------------------------------------------------------------
#[test]
fn test_verify_checksum_fails_first_payload_byte_changed() {
    let value: u32 = 0xCAFE_BABE;
    let mut encoded = encode_with_checksum(&value).expect("encode failed");
    // First byte of payload is at HEADER_SIZE offset
    encoded[HEADER_SIZE] ^= 0x01;
    let result = verify_checksum(&encoded);
    assert!(
        result.is_err(),
        "verify_checksum must fail when first payload byte is changed"
    );
    assert!(
        matches!(result, Err(oxicode::Error::ChecksumMismatch { .. })),
        "error must be ChecksumMismatch, got: {:?}",
        result
    );
}

// ---------------------------------------------------------------------------
// Test 12: two identical values produce identical checksummed bytes
// ---------------------------------------------------------------------------
#[test]
fn test_identical_values_produce_identical_checksummed_bytes() {
    let value_a: u32 = 77777;
    let value_b: u32 = 77777;
    let encoded_a = encode_with_checksum(&value_a).expect("encode value_a failed");
    let encoded_b = encode_with_checksum(&value_b).expect("encode value_b failed");
    assert_eq!(
        encoded_a, encoded_b,
        "identical values must produce identical checksummed bytes"
    );
}

// ---------------------------------------------------------------------------
// Test 13: two different values produce different checksummed bytes
// ---------------------------------------------------------------------------
#[test]
fn test_different_values_produce_different_checksummed_bytes() {
    let value_a: u32 = 1;
    let value_b: u32 = 2;
    let encoded_a = encode_with_checksum(&value_a).expect("encode value_a failed");
    let encoded_b = encode_with_checksum(&value_b).expect("encode value_b failed");
    assert_ne!(
        encoded_a, encoded_b,
        "different values must produce different checksummed bytes"
    );
}

// ---------------------------------------------------------------------------
// Test 14: decode_with_checksum on truncated data returns Err
// ---------------------------------------------------------------------------
#[test]
fn test_decode_with_checksum_truncated_data_returns_err() {
    let value: u64 = 9_876_543_210;
    let encoded = encode_with_checksum(&value).expect("encode u64 failed");
    // Truncate to roughly half
    let truncated = &encoded[..encoded.len() / 2];
    let result = decode_with_checksum::<u64>(truncated);
    assert!(
        result.is_err(),
        "decode_with_checksum must return Err on truncated data"
    );
}

// ---------------------------------------------------------------------------
// Test 15: decode_with_checksum on empty data returns Err
// ---------------------------------------------------------------------------
#[test]
fn test_decode_with_checksum_empty_data_returns_err() {
    let result = decode_with_checksum::<u32>(&[]);
    assert!(
        result.is_err(),
        "decode_with_checksum must return Err on empty data"
    );
    assert!(
        matches!(result, Err(oxicode::Error::UnexpectedEnd { .. })),
        "error must be UnexpectedEnd, got: {:?}",
        result
    );
}

// ---------------------------------------------------------------------------
// Test 16: encode_with_checksum + decode_with_checksum bool true roundtrip
// ---------------------------------------------------------------------------
#[test]
fn test_encode_decode_bool_true_roundtrip() {
    let value = true;
    let encoded = encode_with_checksum(&value).expect("encode bool true failed");
    let (decoded, consumed): (bool, _) =
        decode_with_checksum(&encoded).expect("decode bool true failed");
    assert!(decoded, "decoded bool must be true");
    assert_eq!(
        consumed,
        encoded.len(),
        "consumed must equal total encoded length"
    );
}

// ---------------------------------------------------------------------------
// Test 17: encode_with_checksum + decode_with_checksum bool false roundtrip
// ---------------------------------------------------------------------------
#[test]
fn test_encode_decode_bool_false_roundtrip() {
    let value = false;
    let encoded = encode_with_checksum(&value).expect("encode bool false failed");
    let (decoded, consumed): (bool, _) =
        decode_with_checksum(&encoded).expect("decode bool false failed");
    assert!(!decoded, "decoded bool must be false");
    assert_eq!(
        consumed,
        encoded.len(),
        "consumed must equal total encoded length"
    );
}

// ---------------------------------------------------------------------------
// Test 18: encode_with_checksum + decode_with_checksum u64::MAX roundtrip
// ---------------------------------------------------------------------------
#[test]
fn test_encode_decode_u64_max_roundtrip() {
    let value = u64::MAX;
    let encoded = encode_with_checksum(&value).expect("encode u64::MAX failed");
    let (decoded, consumed): (u64, _) =
        decode_with_checksum(&encoded).expect("decode u64::MAX failed");
    assert_eq!(decoded, value, "decoded u64::MAX must equal original");
    assert_eq!(
        consumed,
        encoded.len(),
        "consumed must equal total encoded length"
    );
}

// ---------------------------------------------------------------------------
// Test 19: encode_with_checksum + decode_with_checksum Vec<String> roundtrip
// ---------------------------------------------------------------------------
#[test]
fn test_encode_decode_vec_string_roundtrip() {
    let value: Vec<String> = vec![
        "first".to_string(),
        "second".to_string(),
        String::new(),
        "fourth with spaces".to_string(),
    ];
    let encoded = encode_with_checksum(&value).expect("encode Vec<String> failed");
    let (decoded, consumed): (Vec<String>, _) =
        decode_with_checksum(&encoded).expect("decode Vec<String> failed");
    assert_eq!(decoded, value, "decoded Vec<String> must equal original");
    assert_eq!(
        consumed,
        encoded.len(),
        "consumed must equal total encoded length"
    );
}

// ---------------------------------------------------------------------------
// Test 20: encode_with_checksum + decode_with_checksum Option<String> Some roundtrip
// ---------------------------------------------------------------------------
#[test]
fn test_encode_decode_option_string_some_roundtrip() {
    let value: Option<String> = Some("option some value".to_string());
    let encoded = encode_with_checksum(&value).expect("encode Option<String> Some failed");
    let (decoded, consumed): (Option<String>, _) =
        decode_with_checksum(&encoded).expect("decode Option<String> Some failed");
    assert_eq!(
        decoded, value,
        "decoded Option<String> Some must equal original"
    );
    assert_eq!(
        consumed,
        encoded.len(),
        "consumed must equal total encoded length"
    );
}

// ---------------------------------------------------------------------------
// Test 21: encode_with_checksum + decode_with_checksum Option<u32> None roundtrip
// ---------------------------------------------------------------------------
#[test]
fn test_encode_decode_option_u32_none_roundtrip() {
    let value: Option<u32> = None;
    let encoded = encode_with_checksum(&value).expect("encode Option<u32> None failed");
    let (decoded, consumed): (Option<u32>, _) =
        decode_with_checksum(&encoded).expect("decode Option<u32> None failed");
    assert_eq!(
        decoded, value,
        "decoded Option<u32> None must equal original"
    );
    assert_eq!(
        consumed,
        encoded.len(),
        "consumed must equal total encoded length"
    );
}

// ---------------------------------------------------------------------------
// Test 22: large data (Vec<u8> with 1000 bytes) encode_with_checksum roundtrip
// ---------------------------------------------------------------------------
#[test]
fn test_encode_decode_large_vec_u8_1000_bytes_roundtrip() {
    let value: Vec<u8> = (0u8..=255).cycle().take(1000).collect();
    assert_eq!(value.len(), 1000, "test data must be exactly 1000 bytes");
    let encoded = encode_with_checksum(&value).expect("encode large Vec<u8> failed");
    let (decoded, consumed): (Vec<u8>, _) =
        decode_with_checksum(&encoded).expect("decode large Vec<u8> failed");
    assert_eq!(decoded, value, "decoded large Vec<u8> must equal original");
    assert_eq!(
        consumed,
        encoded.len(),
        "consumed must equal total encoded length"
    );
    // Sanity: encoded is larger than raw data (header + length prefix overhead)
    assert!(
        encoded.len() > 1000,
        "encoded size ({}) must exceed raw payload size (1000)",
        encoded.len()
    );
}
