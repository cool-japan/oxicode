//! Advanced error handling and edge case tests for OxiCode (set 2).

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
use oxicode::{
    config, decode_from_slice, decode_from_slice_with_config, encode_to_vec,
    encode_to_vec_with_config, Decode, Encode,
};

// ── test 1 ──────────────────────────────────────────────────────────────────

#[test]
fn test_empty_slice_decode_returns_error() {
    let result = decode_from_slice::<u32>(&[]);
    assert!(
        result.is_err(),
        "decoding empty slice should return an error"
    );
}

// ── test 2 ──────────────────────────────────────────────────────────────────
// 0xFF = 255. Varint markers are 251-254; 0xFF (255) is not a valid tag byte,
// so the decoder returns InvalidIntegerType for a u32.

#[test]
fn test_single_0xff_decode_u32_errors() {
    let result = decode_from_slice::<u32>(&[0xFF]);
    assert!(
        result.is_err(),
        "0xFF is not a valid varint tag for u32; should error"
    );
}

// ── test 3 ──────────────────────────────────────────────────────────────────
// 0xFF is still an invalid tag byte even when two bytes are present.

#[test]
fn test_truncated_varint_two_ff_bytes_errors() {
    let result = decode_from_slice::<u32>(&[0xFF, 0xFF]);
    assert!(
        result.is_err(),
        "0xFF, 0xFF is not a valid varint for u32; should error"
    );
}

// ── test 4 ──────────────────────────────────────────────────────────────────

#[test]
fn test_truncated_string_decode_returns_error() {
    let s = String::from("hello world");
    let mut encoded = encode_to_vec(&s).expect("encode string");
    // Drop the last byte to truncate the payload.
    encoded.pop();
    let result = decode_from_slice::<String>(&encoded);
    assert!(result.is_err(), "truncated string encoding should error");
}

// ── test 5 ──────────────────────────────────────────────────────────────────

#[test]
fn test_truncated_vec_u32_decode_returns_error() {
    let v: Vec<u32> = vec![1, 2, 3, 4, 5];
    let mut encoded = encode_to_vec(&v).expect("encode vec");
    // Remove last 4 bytes (one u32 element worth).
    let new_len = encoded.len().saturating_sub(4);
    encoded.truncate(new_len);
    let result = decode_from_slice::<Vec<u32>>(&encoded);
    assert!(result.is_err(), "truncated Vec<u32> should error on decode");
}

// ── test 6 ──────────────────────────────────────────────────────────────────

#[test]
fn test_extra_trailing_bytes_do_not_prevent_valid_decode() {
    let encoded = encode_to_vec(&42u32).expect("encode u32");
    let mut padded = encoded.clone();
    padded.extend_from_slice(&[0x00, 0x00, 0x00]);
    let (value, consumed): (u32, _) =
        decode_from_slice(&padded).expect("decode with trailing bytes");
    assert_eq!(value, 42u32);
    // consumed should be less than the total padded length.
    assert!(consumed < padded.len());
    assert_eq!(consumed, encoded.len());
}

// ── test 7 ──────────────────────────────────────────────────────────────────

#[test]
fn test_limit_config_decode_within_limit_succeeds() {
    let cfg = config::standard().with_limit::<128>();
    let encoded = encode_to_vec_with_config(&99u8, cfg).expect("encode u8");
    let (val, _): (u8, _) =
        decode_from_slice_with_config(&encoded, cfg).expect("decode within limit");
    assert_eq!(val, 99u8);
}

// ── test 8 ──────────────────────────────────────────────────────────────────

#[test]
fn test_limit_config_decode_exceeding_limit_returns_error() {
    // Encode a large string with no limit, then try to decode with a tiny limit.
    let s = String::from("this is a fairly long string that will exceed the tiny limit");
    let encoded = encode_to_vec(&s).expect("encode string");
    let tiny_cfg = config::standard().with_limit::<4>();
    let result = decode_from_slice_with_config::<String, _>(&encoded, tiny_cfg);
    assert!(result.is_err(), "decoding past byte limit should error");
}

// ── test 9 ──────────────────────────────────────────────────────────────────
// An empty slice contains no bytes at all; even u32=0 needs at least 1 byte.

#[test]
fn test_zero_bytes_cannot_decode_u32_zero() {
    let result = decode_from_slice::<u32>(&[]);
    assert!(result.is_err(), "cannot decode u32 from zero bytes");
}

// ── test 10 ──────────────────────────────────────────────────────────────────

#[test]
fn test_invalid_bool_byte_0x02_returns_error() {
    let result = decode_from_slice::<bool>(&[0x02]);
    assert!(result.is_err(), "0x02 is not a valid bool byte");
}

// ── test 11 ──────────────────────────────────────────────────────────────────

#[test]
fn test_bool_false_decodes_from_0x00() {
    let (val, consumed): (bool, _) = decode_from_slice(&[0x00]).expect("decode bool false");
    assert!(!val);
    assert_eq!(consumed, 1);
}

// ── test 12 ──────────────────────────────────────────────────────────────────

#[test]
fn test_bool_true_decodes_from_0x01() {
    let (val, consumed): (bool, _) = decode_from_slice(&[0x01]).expect("decode bool true");
    assert!(val);
    assert_eq!(consumed, 1);
}

// ── test 13 ──────────────────────────────────────────────────────────────────
// Build bytes that claim length=1 and then supply 0x80, which is not valid
// UTF-8 as a lone byte.

#[test]
fn test_invalid_utf8_bytes_decode_string_returns_error() {
    // Varint length prefix 1 (single byte 0x01), then a lone 0x80 continuation byte.
    let bytes: &[u8] = &[0x01, 0x80];
    let result = decode_from_slice::<String>(bytes);
    assert!(
        result.is_err(),
        "invalid UTF-8 bytes should error on String decode"
    );
}

// ── test 14 ──────────────────────────────────────────────────────────────────

#[derive(Debug, PartialEq, Encode, Decode)]
struct SimpleStruct {
    x: u32,
    y: u64,
    z: bool,
}

#[test]
fn test_truncated_struct_decode_returns_error() {
    let val = SimpleStruct {
        x: 42,
        y: 1234567890,
        z: true,
    };
    let mut encoded = encode_to_vec(&val).expect("encode struct");
    // Chop off the last 4 bytes to create a partial payload.
    let new_len = encoded.len().saturating_sub(4);
    encoded.truncate(new_len);
    let result = decode_from_slice::<SimpleStruct>(&encoded);
    assert!(result.is_err(), "truncated struct encoding should error");
}

// ── test 15 ──────────────────────────────────────────────────────────────────

#[derive(Debug, PartialEq, Encode, Decode)]
enum TinyEnum {
    Alpha,
    Beta,
}

#[test]
fn test_nonexistent_enum_discriminant_returns_error() {
    // Encode TinyEnum::Alpha (discriminant 0), then replace its discriminant
    // byte with a large value (200) that has no matching variant.
    let encoded = encode_to_vec(&TinyEnum::Alpha).expect("encode enum");
    let mut tampered = encoded.clone();
    // Discriminant is first byte (varint, 0 → 0x00).
    tampered[0] = 200;
    let result = decode_from_slice::<TinyEnum>(&tampered);
    assert!(result.is_err(), "unknown enum discriminant should error");
}

// ── test 16 ──────────────────────────────────────────────────────────────────

#[test]
fn test_u32_max_value_roundtrip() {
    let original = u32::MAX;
    let encoded = encode_to_vec(&original).expect("encode u32::MAX");
    let (decoded, consumed): (u32, _) = decode_from_slice(&encoded).expect("decode u32::MAX");
    assert_eq!(decoded, original);
    assert_eq!(consumed, encoded.len());
}

// ── test 17 ──────────────────────────────────────────────────────────────────

#[test]
fn test_decode_consumes_correct_byte_count_from_multi_value_slice() {
    // Encode two separate values and concatenate them.
    let enc_a = encode_to_vec(&10u32).expect("encode a");
    let enc_b = encode_to_vec(&20u64).expect("encode b");
    let mut combined = enc_a.clone();
    combined.extend_from_slice(&enc_b);

    let (val_a, consumed_a): (u32, _) = decode_from_slice(&combined).expect("decode first value");
    assert_eq!(val_a, 10u32);
    assert_eq!(consumed_a, enc_a.len());

    // Remaining bytes should decode the second value.
    let (val_b, consumed_b): (u64, _) =
        decode_from_slice(&combined[consumed_a..]).expect("decode second value");
    assert_eq!(val_b, 20u64);
    assert_eq!(consumed_b, enc_b.len());
}

// ── test 18 ──────────────────────────────────────────────────────────────────

#[test]
fn test_multiple_sequential_decodes_accumulate_offsets() {
    let values: Vec<u32> = vec![1, 2, 3, 4, 5];
    let mut buffer: Vec<u8> = Vec::new();
    for &v in &values {
        let chunk = encode_to_vec(&v).expect("encode element");
        buffer.extend_from_slice(&chunk);
    }

    let mut offset = 0usize;
    for &expected in &values {
        let (val, consumed): (u32, _) =
            decode_from_slice(&buffer[offset..]).expect("sequential decode");
        assert_eq!(val, expected);
        offset += consumed;
    }
    // All bytes consumed.
    assert_eq!(offset, buffer.len());
}

// ── test 19 ──────────────────────────────────────────────────────────────────
// Craft a sequence with a very large length prefix but without enough bytes.
// Use a tiny byte-limit config so it errors before trying to allocate.

#[test]
fn test_large_sequence_length_with_small_limit_returns_error() {
    // Varint-encode a huge length: 0xFD (U64_BYTE = 253) + 8 LE bytes for u64::MAX/2.
    // This claims to be a Vec of billions of u32 elements.
    let huge_len: u64 = 0x0001_0000_0000; // 4 billion
    let len_bytes = huge_len.to_le_bytes();
    let mut fake_payload: Vec<u8> = vec![253]; // U64_BYTE
    fake_payload.extend_from_slice(&len_bytes);
    // No actual element data follows.

    let tiny_cfg = config::standard().with_limit::<16>();
    let result = decode_from_slice_with_config::<Vec<u32>, _>(&fake_payload, tiny_cfg);
    assert!(
        result.is_err(),
        "huge sequence with tiny limit should error"
    );
}

// ── test 20 ──────────────────────────────────────────────────────────────────

#[test]
fn test_empty_vec_encodes_to_single_byte_and_roundtrips() {
    let v: Vec<u32> = vec![];
    let encoded = encode_to_vec(&v).expect("encode empty vec");
    // Varint 0 = single byte 0x00.
    assert_eq!(
        encoded.len(),
        1,
        "empty Vec should encode to exactly 1 byte"
    );
    assert_eq!(encoded[0], 0x00);
    let (decoded, consumed): (Vec<u32>, _) = decode_from_slice(&encoded).expect("decode empty vec");
    assert!(decoded.is_empty());
    assert_eq!(consumed, 1);
}

// ── test 21 ──────────────────────────────────────────────────────────────────

#[test]
fn test_none_option_encodes_as_single_byte_0x00() {
    let val: Option<u32> = None;
    let encoded = encode_to_vec(&val).expect("encode None");
    assert_eq!(encoded.len(), 1, "None should encode to exactly 1 byte");
    assert_eq!(encoded[0], 0x00, "None discriminant must be 0x00");
}

// ── test 22 ──────────────────────────────────────────────────────────────────

#[test]
fn test_some_u32_max_first_byte_is_some_discriminant() {
    let val: Option<u32> = Some(u32::MAX);
    let encoded = encode_to_vec(&val).expect("encode Some(u32::MAX)");
    assert!(
        !encoded.is_empty(),
        "Some(u32::MAX) encoding must not be empty"
    );
    // The first byte is the Some discriminant (1).
    assert_eq!(encoded[0], 0x01, "Some variant discriminant must be 0x01");
    // Verify the roundtrip as well.
    let (decoded, _): (Option<u32>, _) =
        decode_from_slice(&encoded).expect("decode Some(u32::MAX)");
    assert_eq!(decoded, Some(u32::MAX));
}
