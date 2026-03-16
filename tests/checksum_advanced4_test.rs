//! Advanced checksum/CRC32 tests for OxiCode — exactly 22 top-level #[test] functions.
//!
//! Compile with: cargo test --features checksum --test checksum_advanced4_test

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
use oxicode::{config, decode_from_slice, encode_to_vec, Decode, Encode};

// ---------------------------------------------------------------------------
// Shared helper types
// ---------------------------------------------------------------------------

#[derive(Debug, PartialEq, Encode, Decode)]
struct Coordinate {
    x: i32,
    y: i32,
    z: i32,
}

#[derive(Debug, PartialEq, Encode, Decode)]
struct Wrapper {
    inner: Coordinate,
    label: String,
}

#[derive(Debug, PartialEq, Encode, Decode)]
struct Deep {
    outer: Wrapper,
    depth: u8,
}

#[derive(Debug, PartialEq, Encode, Decode)]
enum Status {
    Active,
    Inactive,
    Pending(u32),
}

// ---------------------------------------------------------------------------
// Test 1: u8 value roundtrip with checksum
// ---------------------------------------------------------------------------
#[test]
fn test_u8_value_roundtrip_with_checksum() {
    let value: u8 = 0xAB;
    let encoded = encode_with_checksum(&value).expect("encode u8 failed");
    let (decoded, consumed): (u8, _) = decode_with_checksum(&encoded).expect("decode u8 failed");
    assert_eq!(decoded, value, "decoded u8 must equal original");
    assert_eq!(
        consumed,
        encoded.len(),
        "consumed must equal total encoded length"
    );
}

// ---------------------------------------------------------------------------
// Test 2: u16 value roundtrip with checksum
// ---------------------------------------------------------------------------
#[test]
fn test_u16_value_roundtrip_with_checksum() {
    let value: u16 = 60_000;
    let encoded = encode_with_checksum(&value).expect("encode u16 failed");
    let (decoded, consumed): (u16, _) = decode_with_checksum(&encoded).expect("decode u16 failed");
    assert_eq!(decoded, value, "decoded u16 must equal original");
    assert_eq!(
        consumed,
        encoded.len(),
        "consumed must equal total encoded length"
    );
}

// ---------------------------------------------------------------------------
// Test 3: i32 negative value roundtrip with checksum
// ---------------------------------------------------------------------------
#[test]
fn test_i32_negative_value_roundtrip_with_checksum() {
    let value: i32 = -1_234_567;
    let encoded = encode_with_checksum(&value).expect("encode negative i32 failed");
    let (decoded, consumed): (i32, _) =
        decode_with_checksum(&encoded).expect("decode negative i32 failed");
    assert_eq!(decoded, value, "decoded negative i32 must equal original");
    assert_eq!(
        consumed,
        encoded.len(),
        "consumed must equal total encoded length"
    );
}

// ---------------------------------------------------------------------------
// Test 4: f64 PI roundtrip with checksum (bit-exact)
// ---------------------------------------------------------------------------
#[test]
fn test_f64_pi_roundtrip_with_checksum_bit_exact() {
    let value: f64 = std::f64::consts::PI;
    let encoded = encode_with_checksum(&value).expect("encode f64 PI failed");
    let (decoded, consumed): (f64, _) =
        decode_with_checksum(&encoded).expect("decode f64 PI failed");
    assert_eq!(
        decoded.to_bits(),
        value.to_bits(),
        "decoded f64 PI must be bit-exact equal to original"
    );
    assert_eq!(
        consumed,
        encoded.len(),
        "consumed must equal total encoded length"
    );
}

// ---------------------------------------------------------------------------
// Test 5: Empty Vec<u8> roundtrip with checksum
// ---------------------------------------------------------------------------
#[test]
fn test_empty_vec_u8_roundtrip_with_checksum() {
    let value: Vec<u8> = Vec::new();
    let encoded = encode_with_checksum(&value).expect("encode empty Vec<u8> failed");
    let (decoded, consumed): (Vec<u8>, _) =
        decode_with_checksum(&encoded).expect("decode empty Vec<u8> failed");
    assert_eq!(decoded, value, "decoded empty Vec<u8> must equal original");
    assert!(decoded.is_empty(), "decoded vec must be empty");
    assert_eq!(
        consumed,
        encoded.len(),
        "consumed must equal total encoded length"
    );
}

// ---------------------------------------------------------------------------
// Test 6: Vec<u32> roundtrip with checksum
// ---------------------------------------------------------------------------
#[test]
fn test_vec_u32_roundtrip_with_checksum() {
    let value: Vec<u32> = vec![0, 1, 100, 65535, u32::MAX, 42, 999_999];
    let encoded = encode_with_checksum(&value).expect("encode Vec<u32> failed");
    let (decoded, consumed): (Vec<u32>, _) =
        decode_with_checksum(&encoded).expect("decode Vec<u32> failed");
    assert_eq!(decoded, value, "decoded Vec<u32> must equal original");
    assert_eq!(
        consumed,
        encoded.len(),
        "consumed must equal total encoded length"
    );
}

// ---------------------------------------------------------------------------
// Test 7: Large Vec<u8> (2000 bytes) roundtrip with checksum
// ---------------------------------------------------------------------------
#[test]
fn test_large_vec_u8_2000_bytes_roundtrip_with_checksum() {
    let value: Vec<u8> = (0u8..=255).cycle().take(2000).collect();
    assert_eq!(value.len(), 2000, "test data must be exactly 2000 bytes");
    let encoded = encode_with_checksum(&value).expect("encode 2000-byte Vec<u8> failed");
    let (decoded, consumed): (Vec<u8>, _) =
        decode_with_checksum(&encoded).expect("decode 2000-byte Vec<u8> failed");
    assert_eq!(
        decoded, value,
        "decoded 2000-byte Vec<u8> must equal original"
    );
    assert_eq!(
        consumed,
        encoded.len(),
        "consumed must equal total encoded length"
    );
}

// ---------------------------------------------------------------------------
// Test 8: Nested struct roundtrip with checksum
// ---------------------------------------------------------------------------
#[test]
fn test_nested_struct_roundtrip_with_checksum() {
    let value = Wrapper {
        inner: Coordinate {
            x: -10,
            y: 42,
            z: 1000,
        },
        label: String::from("nested struct test"),
    };
    let encoded = encode_with_checksum(&value).expect("encode Wrapper struct failed");
    let (decoded, consumed): (Wrapper, _) =
        decode_with_checksum(&encoded).expect("decode Wrapper struct failed");
    assert_eq!(decoded, value, "decoded Wrapper struct must equal original");
    assert_eq!(
        consumed,
        encoded.len(),
        "consumed must equal total encoded length"
    );
}

// ---------------------------------------------------------------------------
// Test 9: Enum variant roundtrip with checksum
// ---------------------------------------------------------------------------
#[test]
fn test_enum_variant_roundtrip_with_checksum() {
    let value = Status::Pending(9999);
    let encoded = encode_with_checksum(&value).expect("encode Status::Pending failed");
    let (decoded, consumed): (Status, _) =
        decode_with_checksum(&encoded).expect("decode Status::Pending failed");
    assert_eq!(decoded, value, "decoded Status enum must equal original");
    assert_eq!(
        consumed,
        encoded.len(),
        "consumed must equal total encoded length"
    );
}

// ---------------------------------------------------------------------------
// Test 10: Option<String> None roundtrip with checksum
// ---------------------------------------------------------------------------
#[test]
fn test_option_string_none_roundtrip_with_checksum() {
    let value: Option<String> = None;
    let encoded = encode_with_checksum(&value).expect("encode Option<String> None failed");
    let (decoded, consumed): (Option<String>, _) =
        decode_with_checksum(&encoded).expect("decode Option<String> None failed");
    assert_eq!(
        decoded, value,
        "decoded Option<String> None must equal original"
    );
    assert!(decoded.is_none(), "decoded option must be None");
    assert_eq!(
        consumed,
        encoded.len(),
        "consumed must equal total encoded length"
    );
}

// ---------------------------------------------------------------------------
// Test 11: Vec<String> roundtrip with checksum
// ---------------------------------------------------------------------------
#[test]
fn test_vec_string_roundtrip_with_checksum() {
    let value: Vec<String> = vec![
        String::from("alpha"),
        String::from("beta gamma"),
        String::new(),
        String::from("delta epsilon zeta"),
        String::from("η"),
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
// Test 12: Different data produces different checksum bytes
// ---------------------------------------------------------------------------
#[test]
fn test_different_data_produces_different_checksum_bytes() {
    let value_a: u64 = 0x0000_0000_0000_0001;
    let value_b: u64 = 0x0000_0000_0000_0002;
    let encoded_a = encode_with_checksum(&value_a).expect("encode value_a failed");
    let encoded_b = encode_with_checksum(&value_b).expect("encode value_b failed");
    assert_ne!(
        encoded_a, encoded_b,
        "different values must produce different checksummed bytes"
    );
}

// ---------------------------------------------------------------------------
// Test 13: Single bit flip in middle of data detected
// ---------------------------------------------------------------------------
#[test]
fn test_single_bit_flip_in_middle_of_data_detected() {
    let value: Vec<u8> = vec![0x11u8; 64];
    let mut encoded = encode_with_checksum(&value).expect("encode Vec<u8> 64-bytes failed");
    // Flip a single bit in the middle of the payload region
    let mid = HEADER_SIZE + 32;
    encoded[mid] ^= 0x01;
    let result = verify_checksum(&encoded);
    assert!(
        result.is_err(),
        "verify_checksum must detect a single bit flip in the middle of the data"
    );
    assert!(
        matches!(result, Err(oxicode::Error::ChecksumMismatch { .. })),
        "error must be ChecksumMismatch, got: {:?}",
        result
    );
}

// ---------------------------------------------------------------------------
// Test 14: Checksum bytes for same data are deterministic
// ---------------------------------------------------------------------------
#[test]
fn test_checksum_bytes_for_same_data_are_deterministic() {
    let value = Coordinate {
        x: 7,
        y: -3,
        z: 100,
    };
    let encoded_first = encode_with_checksum(&value).expect("first encode failed");
    let encoded_second = encode_with_checksum(&value).expect("second encode failed");
    assert_eq!(
        encoded_first, encoded_second,
        "encoding the same value twice must produce identical bytes"
    );
}

// ---------------------------------------------------------------------------
// Test 15: Truncated checksum data returns Err
// ---------------------------------------------------------------------------
#[test]
fn test_truncated_checksum_data_returns_err() {
    let value: u64 = 123_456_789_987_654_321;
    let encoded = encode_with_checksum(&value).expect("encode u64 failed");
    // Keep only the header, drop all payload bytes
    let header_only = &encoded[..HEADER_SIZE];
    let result = decode_with_checksum::<u64>(header_only);
    assert!(
        result.is_err(),
        "decode_with_checksum must return Err when payload is truncated away"
    );
}

// ---------------------------------------------------------------------------
// Test 16: Corrupted CRC32 bytes in header region returns Err
//
// Header layout: [MAGIC(3)][VERSION(1)][LEN(8 LE u64)][CRC32(4 LE u32)][PAYLOAD]
// CRC32 field lives at bytes 12..16.  Flipping bits there makes the stored
// checksum disagree with the recomputed one, so decode must return Err.
// ---------------------------------------------------------------------------
#[test]
fn test_corrupted_crc32_bytes_in_header_returns_err() {
    let value: u32 = 0xFEED_FACE;
    let mut encoded = encode_with_checksum(&value).expect("encode u32 failed");
    // Flip the first byte of the CRC32 field (offset 12) so the stored CRC
    // diverges from crc32fast::hash(payload).  The magic and length remain
    // intact so the error must be ChecksumMismatch, not InvalidData.
    encoded[12] ^= 0xFF;
    let result = decode_with_checksum::<u32>(&encoded);
    assert!(
        result.is_err(),
        "decode_with_checksum must return Err when the CRC32 header bytes are corrupted"
    );
    assert!(
        matches!(result, Err(oxicode::Error::ChecksumMismatch { .. })),
        "error must be ChecksumMismatch when CRC32 header byte is flipped, got: {:?}",
        result
    );
}

// ---------------------------------------------------------------------------
// Test 17: Three-level struct with checksum roundtrip
// ---------------------------------------------------------------------------
#[test]
fn test_three_level_struct_roundtrip_with_checksum() {
    let value = Deep {
        outer: Wrapper {
            inner: Coordinate {
                x: -500,
                y: 250,
                z: 0,
            },
            label: String::from("three-level deep"),
        },
        depth: 3,
    };
    let encoded = encode_with_checksum(&value).expect("encode Deep struct failed");
    let (decoded, consumed): (Deep, _) =
        decode_with_checksum(&encoded).expect("decode Deep struct failed");
    assert_eq!(decoded, value, "decoded Deep struct must equal original");
    assert_eq!(
        consumed,
        encoded.len(),
        "consumed must equal total encoded length"
    );
}

// ---------------------------------------------------------------------------
// Test 18: i64::MIN roundtrip with checksum
// ---------------------------------------------------------------------------
#[test]
fn test_i64_min_roundtrip_with_checksum() {
    let value = i64::MIN;
    let encoded = encode_with_checksum(&value).expect("encode i64::MIN failed");
    let (decoded, consumed): (i64, _) =
        decode_with_checksum(&encoded).expect("decode i64::MIN failed");
    assert_eq!(decoded, value, "decoded i64::MIN must equal original");
    assert_eq!(
        consumed,
        encoded.len(),
        "consumed must equal total encoded length"
    );
}

// ---------------------------------------------------------------------------
// Test 19: u64::MAX roundtrip with checksum
// ---------------------------------------------------------------------------
#[test]
fn test_u64_max_roundtrip_with_checksum() {
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
// Test 20: bool true/false roundtrip with checksum
// ---------------------------------------------------------------------------
#[test]
fn test_bool_true_false_roundtrip_with_checksum() {
    let encoded_true = encode_with_checksum(&true).expect("encode true failed");
    let (decoded_true, consumed_true): (bool, _) =
        decode_with_checksum(&encoded_true).expect("decode true failed");
    assert!(decoded_true, "decoded value must be true");
    assert_eq!(
        consumed_true,
        encoded_true.len(),
        "consumed must equal encoded_true length"
    );

    let encoded_false = encode_with_checksum(&false).expect("encode false failed");
    let (decoded_false, consumed_false): (bool, _) =
        decode_with_checksum(&encoded_false).expect("decode false failed");
    assert!(!decoded_false, "decoded value must be false");
    assert_eq!(
        consumed_false,
        encoded_false.len(),
        "consumed must equal encoded_false length"
    );

    // true and false must produce different checksummed bytes
    assert_ne!(
        encoded_true, encoded_false,
        "true and false must produce different checksummed byte sequences"
    );
}

// ---------------------------------------------------------------------------
// Test 21: Checksum overhead is constant (same extra bytes regardless of payload size)
// ---------------------------------------------------------------------------
#[test]
fn test_checksum_overhead_is_constant_regardless_of_payload_size() {
    // Measure overhead for small and large payloads using encode_to_vec / encode_with_checksum
    let small: Vec<u8> = vec![0u8; 10];
    let large: Vec<u8> = vec![0u8; 500];

    let plain_small = encode_to_vec(&small).expect("plain encode small failed");
    let plain_large = encode_to_vec(&large).expect("plain encode large failed");

    let checked_small = encode_with_checksum(&small).expect("checksum encode small failed");
    let checked_large = encode_with_checksum(&large).expect("checksum encode large failed");

    let overhead_small = checked_small.len() - plain_small.len();
    let overhead_large = checked_large.len() - plain_large.len();

    assert_eq!(
        overhead_small, HEADER_SIZE,
        "checksum overhead for small payload must be exactly HEADER_SIZE ({}) bytes, got {}",
        HEADER_SIZE, overhead_small
    );
    assert_eq!(
        overhead_large, HEADER_SIZE,
        "checksum overhead for large payload must be exactly HEADER_SIZE ({}) bytes, got {}",
        HEADER_SIZE, overhead_large
    );
    assert_eq!(
        overhead_small, overhead_large,
        "checksum overhead must be identical for all payload sizes"
    );

    // Confirm decode_from_slice works on the plain-encoded bytes (no checksum)
    let (decoded_small, _): (Vec<u8>, _) =
        decode_from_slice(&plain_small).expect("decode plain small failed");
    assert_eq!(
        decoded_small, small,
        "plain decoded small must equal original"
    );
}

// ---------------------------------------------------------------------------
// Test 22: Verify checksum on data that was not wrapped returns Err
// ---------------------------------------------------------------------------
#[test]
fn test_verify_checksum_on_unwrapped_data_returns_err() {
    // Encode a value with the standard (non-checksum) encoder
    let value: u32 = 0x1234_5678;
    let plain_bytes = encode_to_vec(&value).expect("plain encode failed");

    // These bytes have no OXH magic header — verify_checksum must reject them
    let result = verify_checksum(&plain_bytes);
    assert!(
        result.is_err(),
        "verify_checksum must return Err for data encoded without a checksum header"
    );

    // decode_with_checksum must also reject plain-encoded bytes
    let decode_result = decode_with_checksum::<u32>(&plain_bytes);
    assert!(
        decode_result.is_err(),
        "decode_with_checksum must return Err for data encoded without checksum wrapping"
    );

    // A decode_from_slice of the plain bytes must still succeed, confirming the bytes are
    // valid oxicode-encoded data — just not checksum-wrapped
    let (recovered, _): (u32, _) =
        decode_from_slice(&plain_bytes).expect("plain decode of unwrapped bytes failed");
    assert_eq!(
        recovered, value,
        "plain decode must still recover the original value from unwrapped bytes"
    );

    // Verify that wrap_with_checksum on valid raw bytes can be round-tripped separately
    let raw_payload = b"raw payload not encoded with checksum";
    let wrapped = wrap_with_checksum(raw_payload);
    let recovered_payload =
        verify_checksum(&wrapped).expect("verify_checksum of freshly wrapped raw payload failed");
    assert_eq!(
        recovered_payload, raw_payload,
        "wrap_with_checksum + verify_checksum must recover the original raw payload"
    );

    // Use config to silence unused-import lint
    let _cfg = config::standard();
}
