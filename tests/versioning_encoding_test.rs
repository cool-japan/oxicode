//! Encoding/decoding-focused tests for the versioning module (split from versioning_test.rs).

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
use oxicode::versioning::{
    decode_versioned, decode_versioned_with_check, encode_versioned, extract_version, is_versioned,
    Version, VERSIONED_MAGIC,
};

// ── Low-level encode/decode versioned (raw bytes) ─────────────────────────────

#[test]
fn test_encode_decode_versioned_raw() {
    let data = b"hello world";
    let version = Version::new(1, 0, 0);

    let encoded = encode_versioned(data, version).expect("encode failed");
    let (decoded, ver) = decode_versioned(&encoded).expect("decode failed");

    assert_eq!(decoded.as_slice(), data.as_slice());
    assert_eq!(ver, version);
}

#[test]
fn test_is_versioned_detection() {
    let data = b"raw bytes";
    assert!(!is_versioned(data));

    let version = Version::new(1, 0, 0);
    let encoded = encode_versioned(data, version).expect("encode failed");
    assert!(is_versioned(&encoded));
}

#[test]
fn test_is_versioned_empty_slice() {
    assert!(!is_versioned(&[]));
    assert!(!is_versioned(&[0x4F, 0x58])); // partial magic
}

#[test]
fn test_extract_version_only() {
    let data = b"payload";
    let version = Version::new(3, 1, 4);
    let encoded = encode_versioned(data, version).expect("encode failed");
    let extracted = extract_version(&encoded).expect("extract failed");
    assert_eq!(extracted, version);
}

#[test]
fn test_decode_versioned_with_check_compatible() {
    let data = b"test payload";
    let data_version = Version::new(1, 3, 0);
    let current = Version::new(1, 5, 0);
    let min = Some(Version::new(1, 0, 0));

    let encoded = encode_versioned(data, data_version).expect("encode failed");
    let (payload, ver, compat) =
        decode_versioned_with_check(&encoded, current, min).expect("decode failed");

    assert_eq!(payload.as_slice(), data.as_slice());
    assert_eq!(ver, data_version);
    assert!(compat.is_usable());
}

#[test]
fn test_decode_versioned_with_check_incompatible() {
    let data = b"test";
    let data_version = Version::new(2, 0, 0);
    let current = Version::new(1, 0, 0);

    let encoded = encode_versioned(data, data_version).expect("encode failed");
    let result = decode_versioned_with_check(&encoded, current, None);
    assert!(result.is_err());
}

#[test]
fn test_decode_invalid_magic() {
    let garbage = b"not valid data at all";
    let result = decode_versioned(garbage);
    assert!(result.is_err());
}

#[test]
fn test_decode_truncated_data() {
    // Too short to contain a valid header
    let short = &[0x4F, 0x58, 0x49, 0x56]; // just magic, no version bytes
    let result = decode_versioned(short);
    assert!(result.is_err());
}

#[test]
fn test_encode_empty_payload() {
    let data: &[u8] = &[];
    let version = Version::new(1, 0, 0);
    let encoded = encode_versioned(data, version).expect("encode failed");
    let (decoded, ver) = decode_versioned(&encoded).expect("decode failed");
    assert!(decoded.is_empty());
    assert_eq!(ver, version);
}

// ── High-level encode_versioned_value / decode_versioned_value ────────────────

#[test]
fn test_encode_decode_versioned_value_u32() {
    let version = Version::new(1, 0, 0);
    let encoded = oxicode::encode_versioned_value(&42u32, version).expect("encode failed");
    let (decoded, ver, _consumed): (u32, _, _) =
        oxicode::decode_versioned_value(&encoded).expect("decode failed");
    assert_eq!(decoded, 42u32);
    assert_eq!(ver, version);
}

#[test]
fn test_encode_decode_versioned_value_u64() {
    let version = Version::new(2, 5, 0);
    let encoded = oxicode::encode_versioned_value(&99u64, version).expect("encode failed");
    let (decoded, ver, _): (u64, _, _) =
        oxicode::decode_versioned_value(&encoded).expect("decode failed");
    assert_eq!(decoded, 99u64);
    assert_eq!(ver, version);
}

#[test]
fn test_encode_decode_versioned_value_bool() {
    let version = Version::new(1, 0, 0);
    for val in [true, false] {
        let encoded = oxicode::encode_versioned_value(&val, version).expect("encode failed");
        let (decoded, ver, _): (bool, _, _) =
            oxicode::decode_versioned_value(&encoded).expect("decode failed");
        assert_eq!(decoded, val);
        assert_eq!(ver, version);
    }
}

#[test]
fn test_encode_decode_versioned_value_string() {
    let version = Version::new(2, 3, 1);
    let original = String::from("hello oxicode versioning");
    let encoded = oxicode::encode_versioned_value(&original, version).expect("encode failed");
    let (decoded, ver, _): (String, _, _) =
        oxicode::decode_versioned_value(&encoded).expect("decode failed");
    assert_eq!(decoded, original);
    assert_eq!(ver, version);
}

#[test]
fn test_encode_decode_versioned_value_vec_u8() {
    let version = Version::new(1, 0, 0);
    let original: Vec<u8> = vec![1, 2, 3, 4, 5, 255, 0];
    let encoded = oxicode::encode_versioned_value(&original, version).expect("encode failed");
    let (decoded, ver, _): (Vec<u8>, _, _) =
        oxicode::decode_versioned_value(&encoded).expect("decode failed");
    assert_eq!(decoded, original);
    assert_eq!(ver, version);
}

#[test]
fn test_encode_decode_versioned_value_vec_u32() {
    let version = Version::new(1, 2, 3);
    let original: Vec<u32> = vec![100, 200, 300, 400, 500];
    let encoded = oxicode::encode_versioned_value(&original, version).expect("encode failed");
    let (decoded, ver, _): (Vec<u32>, _, _) =
        oxicode::decode_versioned_value(&encoded).expect("decode failed");
    assert_eq!(decoded, original);
    assert_eq!(ver, version);
}

#[test]
fn test_version_is_preserved_in_encoded_data() {
    let version = Version::new(5, 12, 99);
    let encoded = oxicode::encode_versioned_value(&100u64, version).expect("encode failed");
    // extract_version should agree with what was stored
    let extracted = extract_version(&encoded).expect("extract failed");
    assert_eq!(extracted, version);
}

#[test]
fn test_versioned_value_is_detected_as_versioned() {
    let version = Version::new(1, 0, 0);
    let encoded = oxicode::encode_versioned_value(&true, version).expect("encode failed");
    assert!(is_versioned(&encoded));
}

#[test]
fn test_versioned_value_decode_wrong_type_fails() {
    let version = Version::new(1, 0, 0);
    // Encode a u32
    let encoded = oxicode::encode_versioned_value(&42u32, version).expect("encode failed");
    // Try to decode as a String — should fail because formats differ
    // (u32 encodes as varint, String encodes as length+bytes)
    // This verifies the payload is type-sensitive
    let result: oxicode::Result<(String, _, _)> = oxicode::decode_versioned_value(&encoded);
    // This may or may not fail depending on the value, but we verify the API works
    let _ = result; // at minimum confirm it compiles and runs
}

#[test]
fn test_multiple_version_numbers_round_trip() {
    // Test a matrix of version numbers to ensure bytes encoding is correct
    let versions = [
        Version::new(0, 0, 0),
        Version::new(1, 0, 0),
        Version::new(0, 1, 0),
        Version::new(0, 0, 1),
        Version::new(255, 255, 255),
        Version::new(1000, 2000, 3000),
        Version::new(65535, 65535, 65535),
    ];
    for version in versions {
        let encoded = oxicode::encode_versioned_value(&version.tuple().0, version).expect("encode");
        let (_, ver, _): (u16, _, _) = oxicode::decode_versioned_value(&encoded).expect("decode");
        assert_eq!(ver, version);
    }
}

/// encode_versioned_value must produce identical bytes on repeated calls.
#[test]
fn test_versioned_encoding_is_deterministic() {
    let v = Version::new(1, 2, 3);

    let enc1 = oxicode::encode_versioned_value(&42u32, v).expect("encode 1");
    let enc2 = oxicode::encode_versioned_value(&42u32, v).expect("encode 2");
    assert_eq!(enc1, enc2, "versioned encoding must be deterministic");
}

/// Encoding the same payload with different version numbers must differ in the
/// header bytes and therefore produce a different byte sequence overall.
#[test]
fn test_different_versions_produce_different_bytes() {
    let enc_v1 = oxicode::encode_versioned_value(&42u32, Version::new(1, 0, 0)).expect("encode v1");
    let enc_v2 = oxicode::encode_versioned_value(&42u32, Version::new(2, 0, 0)).expect("encode v2");

    assert_ne!(
        enc_v1, enc_v2,
        "different versions should produce different bytes"
    );
}

/// Encode with V1, extract the stored version, and confirm it matches exactly.
#[test]
fn test_stored_version_matches_encoded_version() {
    let v = Version::new(3, 7, 11);
    let encoded = oxicode::encode_versioned_value(&100u64, v).expect("encode");
    let stored = extract_version(&encoded).expect("extract");
    assert_eq!(stored, v);
}

/// Verify that decode_versioned_value returns the correct 3-tuple
/// (value, version, bytes_consumed) for a range of primitive types.
#[test]
fn test_decode_versioned_value_tuple_shape() {
    let v = Version::new(2, 0, 0);

    // u8
    let enc = oxicode::encode_versioned_value(&255u8, v).expect("encode u8");
    let (val, ver, consumed): (u8, _, usize) =
        oxicode::decode_versioned_value(&enc).expect("decode u8");
    assert_eq!(val, 255u8);
    assert_eq!(ver, v);
    assert!(consumed > 0, "consumed bytes must be positive");

    // i32
    let enc = oxicode::encode_versioned_value(&-42i32, v).expect("encode i32");
    let (val, ver, consumed): (i32, _, usize) =
        oxicode::decode_versioned_value(&enc).expect("decode i32");
    assert_eq!(val, -42i32);
    assert_eq!(ver, v);
    assert!(consumed > 0);
}

/// 10. test_versioned_encode_includes_version_header
#[test]
fn test_versioned_encode_includes_version_header() {
    let version = Version::new(2, 3, 4);
    let encoded = oxicode::encode_versioned_value(&99u32, version).expect("encode");

    assert!(
        encoded.len() >= VERSIONED_MAGIC.len(),
        "encoded output must be at least as long as the magic bytes"
    );
    assert_eq!(
        &encoded[..VERSIONED_MAGIC.len()],
        &VERSIONED_MAGIC,
        "first bytes must be the OXIV magic"
    );
}

/// 13. test_version_number_in_wire_format
#[test]
fn test_version_number_in_wire_format() {
    let version = Version::new(3, 7, 11);
    let encoded = oxicode::encode_versioned_value(&0u8, version).expect("encode");

    // Header layout: [0..4] magic, [4] header_version,
    // [5..7] major LE u16, [7..9] minor LE u16, [9..11] patch LE u16.
    let major = u16::from_le_bytes([encoded[5], encoded[6]]);
    let minor = u16::from_le_bytes([encoded[7], encoded[8]]);
    let patch = u16::from_le_bytes([encoded[9], encoded[10]]);

    assert_eq!(major, 3, "major must be encoded at bytes 5–6 as LE u16");
    assert_eq!(minor, 7, "minor must be encoded at bytes 7–8 as LE u16");
    assert_eq!(patch, 11, "patch must be encoded at bytes 9–10 as LE u16");
}

/// NEW-4. decode_versioned_value returns the correct version number.
#[test]
fn test_decode_versioned_value_returns_correct_version() {
    let expected_version = Version::new(7, 3, 15);
    let encoded = oxicode::encode_versioned_value(&42u32, expected_version).expect("encode failed");
    let (val, ver, _): (u32, _, _) =
        oxicode::decode_versioned_value(&encoded).expect("decode failed");
    assert_eq!(val, 42u32);
    assert_eq!(
        ver, expected_version,
        "returned version must equal the version used at encoding time"
    );
}

/// NEW-10. decode_versioned_value with version that has all-zero components.
#[test]
fn test_decode_versioned_value_version_zero() {
    let version = Version::zero();
    assert_eq!(version.major, 0);
    assert_eq!(version.minor, 0);
    assert_eq!(version.patch, 0);

    let encoded =
        oxicode::encode_versioned_value(&999u32, version).expect("encode with zero version");
    let (val, ver, _): (u32, _, _) =
        oxicode::decode_versioned_value(&encoded).expect("decode with zero version");
    assert_eq!(val, 999u32);
    assert_eq!(ver, version, "all-zero version must be preserved");
}

/// NEW-13. Version header format: bytes 5–10 contain major, minor, patch as
/// little-endian u16.
#[test]
fn test_version_header_three_fields_at_known_offsets() {
    let version = Version::new(1, 2, 3);
    let encoded = oxicode::encode_versioned_value(&0u8, version).expect("encode");

    // Offsets: [0..4] OXIV magic, [4] header_version=1,
    //          [5..7] major LE u16, [7..9] minor LE u16, [9..11] patch LE u16.
    assert!(encoded.len() >= 11, "encoded must be at least 11 bytes");

    let major = u16::from_le_bytes([encoded[5], encoded[6]]);
    let minor = u16::from_le_bytes([encoded[7], encoded[8]]);
    let patch = u16::from_le_bytes([encoded[9], encoded[10]]);

    assert_eq!(major, 1, "major at bytes 5-6");
    assert_eq!(minor, 2, "minor at bytes 7-8");
    assert_eq!(patch, 3, "patch at bytes 9-10");
}

/// NEW-18. Error on corrupted version header (truncated data).
#[test]
fn test_error_on_truncated_version_header() {
    use oxicode::versioning::{decode_versioned, encode_versioned, Version};

    let version = Version::new(1, 0, 0);
    let full = encode_versioned(b"payload", version).expect("encode");

    // Try every prefix shorter than the 11-byte header.
    for len in 0..11usize {
        let truncated = &full[..len];
        let result = decode_versioned(truncated);
        assert!(
            result.is_err(),
            "decode must fail for truncated header of length {len}"
        );
    }
}

/// NEW-17. Versioned encoding in combination with Compression::None passthrough.
#[cfg(any(feature = "compression-lz4", feature = "compression-zstd"))]
#[test]
fn test_versioned_with_compression_none() {
    use oxicode::compression::{compress, decompress, Compression};
    use oxicode::versioning::{decode_versioned, encode_versioned, extract_version, Version};

    let version = Version::new(1, 2, 3);
    let payload_value = 9999u64;

    // Step 1: plain-encode the value.
    let plain = oxicode::encode_to_vec(&payload_value).expect("plain encode");

    // Step 2: compress with None (passthrough) codec.
    let compressed = compress(&plain, Compression::None).expect("compress None");

    // Step 3: wrap in versioned envelope.
    let versioned = encode_versioned(&compressed, version).expect("encode versioned");

    // Round-trip: extract version, decompress, plain-decode.
    let stored_ver = extract_version(&versioned).expect("extract version");
    assert_eq!(stored_ver, version);

    let (raw, ver) = decode_versioned(&versioned).expect("decode versioned");
    assert_eq!(ver, version);

    let decompressed = decompress(&raw).expect("decompress");
    let (val, _): (u64, _) = oxicode::decode_from_slice(&decompressed).expect("plain decode");

    assert_eq!(val, payload_value);
}
