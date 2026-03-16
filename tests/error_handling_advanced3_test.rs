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

#[derive(Debug, PartialEq, Encode, Decode)]
struct Data {
    id: u32,
    value: String,
}

#[derive(Debug, PartialEq, Encode, Decode)]
enum Tag {
    A,
    B,
    C,
}

// ---------------------------------------------------------------------------
// 1. Decode empty slice returns error
// ---------------------------------------------------------------------------

#[test]
fn test_decode_empty_slice_returns_error() {
    let empty: &[u8] = &[];
    let result: Result<(u32, usize), _> = decode_from_slice(empty);
    assert!(result.is_err(), "decoding empty slice must return an error");
}

// ---------------------------------------------------------------------------
// 2. Decode truncated u32 (incomplete varint) returns error
// ---------------------------------------------------------------------------

#[test]
fn test_decode_truncated_varint_returns_error() {
    // 0xFF as a single byte is a varint marker indicating more bytes follow,
    // but there are no following bytes — the stream is incomplete.
    let bad: &[u8] = &[0xFFu8];
    let result: Result<(u32, usize), _> = decode_from_slice(bad);
    assert!(
        result.is_err(),
        "incomplete varint [0xFF] must fail to decode as u32"
    );
}

// ---------------------------------------------------------------------------
// 3. Decode truncated string returns error
// ---------------------------------------------------------------------------

#[test]
fn test_decode_truncated_string_returns_error() {
    let original = String::from("hello, world");
    let mut encoded = encode_to_vec(&original).expect("encode string");
    // Remove the last byte to truncate
    encoded.pop();
    let result: Result<(String, usize), _> = decode_from_slice(&encoded);
    assert!(
        result.is_err(),
        "decoding a truncated string must return an error"
    );
}

// ---------------------------------------------------------------------------
// 4. Decode truncated Vec<u8> returns error
// ---------------------------------------------------------------------------

#[test]
fn test_decode_truncated_vec_u8_returns_error() {
    let original: Vec<u8> = vec![10, 20, 30, 40, 50];
    let mut encoded = encode_to_vec(&original).expect("encode Vec<u8>");
    encoded.pop();
    let result: Result<(Vec<u8>, usize), _> = decode_from_slice(&encoded);
    assert!(
        result.is_err(),
        "decoding a truncated Vec<u8> must return an error"
    );
}

// ---------------------------------------------------------------------------
// 5. Decode truncated struct returns error
// ---------------------------------------------------------------------------

#[test]
fn test_decode_truncated_struct_returns_error() {
    let original = Data {
        id: 42,
        value: String::from("some value here"),
    };
    let mut encoded = encode_to_vec(&original).expect("encode Data");
    encoded.pop();
    let result: Result<(Data, usize), _> = decode_from_slice(&encoded);
    assert!(
        result.is_err(),
        "decoding a truncated struct must return an error"
    );
}

// ---------------------------------------------------------------------------
// 6. Decode with wrong type may succeed or fail gracefully (no panic)
// ---------------------------------------------------------------------------

#[test]
fn test_decode_wrong_type_no_panic() {
    // Encode a u64 value and try to decode as u32.
    // The intent is to verify it does not panic regardless of success/failure.
    let val: u64 = 0xDEAD_BEEF_CAFE_BABEu64;
    let encoded = encode_to_vec(&val).expect("encode u64");
    // This should either succeed (consuming some bytes) or fail — never panic.
    let _result: Result<(u32, usize), _> = decode_from_slice(&encoded);
    // No assertion on success/failure; reaching here without panic is the test.
}

// ---------------------------------------------------------------------------
// 7. Decode with limit exceeded returns LimitExceeded error
// ---------------------------------------------------------------------------

#[test]
fn test_decode_limit_exceeded_returns_error() {
    // The limit applies to content bytes claimed for container types (String, Vec).
    // Use a small limit and a string whose content exceeds that limit.
    let cfg = config::standard().with_limit::<4>();
    // "hello world" is 11 bytes of content — well above the 4-byte limit.
    let val = String::from("hello world");
    let enc = encode_to_vec(&val).expect("encode string");
    let result: Result<(String, usize), _> = decode_from_slice_with_config(&enc, cfg);
    assert!(
        result.is_err(),
        "string content (11 bytes) should exceed the 4-byte decode limit"
    );
}

// ---------------------------------------------------------------------------
// 8. Decode beyond encoded bytes: second decode returns error
// ---------------------------------------------------------------------------

#[test]
fn test_decode_beyond_encoded_bytes_returns_error() {
    let val: u32 = 100;
    let encoded = encode_to_vec(&val).expect("encode u32");
    // First decode consumes all bytes.
    let (_, consumed): (u32, usize) = decode_from_slice(&encoded).expect("first decode");
    // Attempt a second decode starting after all consumed bytes.
    let remainder = &encoded[consumed..];
    let result: Result<(u32, usize), _> = decode_from_slice(remainder);
    assert!(
        result.is_err(),
        "decoding past the end of the encoded buffer must return an error"
    );
}

// ---------------------------------------------------------------------------
// 9. Decode invalid bool byte (2 or higher) returns error
// ---------------------------------------------------------------------------

#[test]
fn test_decode_invalid_bool_byte_returns_error() {
    let bad = [0x02u8]; // bool only allows 0x00 or 0x01
    let result: Result<(bool, usize), _> = decode_from_slice(&bad);
    assert!(result.is_err(), "invalid bool byte 0x02 should fail");
}

// ---------------------------------------------------------------------------
// 10. Decode enum with too-large discriminant returns error
// ---------------------------------------------------------------------------

#[test]
fn test_decode_enum_too_large_discriminant_returns_error() {
    // Tag has only variants A(0), B(1), C(2). Discriminant 99 is out of range.
    let bad_bytes = encode_to_vec(&99u32).expect("encode discriminant 99");
    let result: Result<(Tag, usize), _> = decode_from_slice(&bad_bytes);
    assert!(
        result.is_err(),
        "discriminant 99 is outside Tag's valid range (0-2)"
    );
}

// ---------------------------------------------------------------------------
// 11. Double decode of same bytes: first succeeds, second fails
// ---------------------------------------------------------------------------

#[test]
fn test_double_decode_second_fails() {
    let val: u32 = 7;
    let encoded = encode_to_vec(&val).expect("encode u32");
    let (_, consumed): (u32, usize) = decode_from_slice(&encoded).expect("first decode");
    let rest = &encoded[consumed..];
    let result: Result<(u32, usize), _> = decode_from_slice(rest);
    assert!(
        result.is_err(),
        "after consuming all bytes, a second decode must fail (stream exhausted)"
    );
}

// ---------------------------------------------------------------------------
// 12. Decode with zero-length bytes returns error
// ---------------------------------------------------------------------------

#[test]
fn test_decode_zero_length_bytes_returns_error() {
    let zero: &[u8] = &[];
    let result: Result<(u64, usize), _> = decode_from_slice(zero);
    assert!(
        result.is_err(),
        "zero-length byte slice must fail to decode"
    );
}

// ---------------------------------------------------------------------------
// 13. Decode garbage bytes returns error (random bytes that aren't valid encoding)
// ---------------------------------------------------------------------------

#[test]
fn test_decode_garbage_bytes_data_struct() {
    // These bytes are highly unlikely to form a valid oxicode-encoded Data struct.
    // Data contains a String field requiring valid UTF-8, so this should fail.
    let garbage: &[u8] = &[0xDE, 0xAD, 0xBE, 0xEF, 0xFF, 0x00, 0x11, 0x22];
    let result: Result<(Data, usize), _> = decode_from_slice(garbage);
    assert!(
        result.is_err(),
        "garbage bytes must fail to decode as Data struct"
    );
}

// ---------------------------------------------------------------------------
// 14. Encode succeeds for valid data
// ---------------------------------------------------------------------------

#[test]
fn test_encode_valid_data_succeeds() {
    let data = Data {
        id: 1,
        value: String::from("test"),
    };
    let encoded = encode_to_vec(&data).expect("encode must succeed for valid Data");
    assert!(!encoded.is_empty(), "encoded output must not be empty");
}

// ---------------------------------------------------------------------------
// 15. Decode roundtrip success (sanity check)
// ---------------------------------------------------------------------------

#[test]
fn test_decode_roundtrip_success() {
    let original = Data {
        id: 999,
        value: String::from("roundtrip"),
    };
    let encoded = encode_to_vec(&original).expect("encode Data");
    let (decoded, _): (Data, usize) = decode_from_slice(&encoded).expect("decode Data");
    assert_eq!(original, decoded);
}

// ---------------------------------------------------------------------------
// 16. Limit config: u32 value within limit succeeds
// ---------------------------------------------------------------------------

#[test]
fn test_limit_config_within_limit_succeeds() {
    // u32 value 1 encodes to 1 byte; a 64-byte limit is easily satisfied.
    let cfg = config::standard().with_limit::<64>();
    let val: u32 = 1;
    let enc = encode_to_vec_with_config(&val, cfg).expect("encode u32 with config");
    let result: Result<(u32, usize), _> = decode_from_slice_with_config(&enc, cfg);
    assert!(
        result.is_ok(),
        "decoding a small u32 within a generous limit must succeed"
    );
}

// ---------------------------------------------------------------------------
// 17. Limit config: Vec<u8> content that exceeds the byte limit fails
// ---------------------------------------------------------------------------

#[test]
fn test_limit_config_exceeds_limit_fails() {
    let cfg = config::standard().with_limit::<8>();
    // A Vec<u8> with 16 bytes of content — the decoder claims 16 bytes
    // when reading the content, which exceeds the 8-byte limit.
    let val: Vec<u8> = vec![0u8; 16];
    let enc = encode_to_vec(&val).expect("encode");
    let result: Result<(Vec<u8>, usize), _> = decode_from_slice_with_config(&enc, cfg);
    assert!(result.is_err(), "should exceed limit");
}

// ---------------------------------------------------------------------------
// 18. Decode truncated Vec<String> returns error
// ---------------------------------------------------------------------------

#[test]
fn test_decode_truncated_vec_string_returns_error() {
    let original: Vec<String> = vec![
        String::from("alpha"),
        String::from("beta"),
        String::from("gamma"),
    ];
    let mut encoded = encode_to_vec(&original).expect("encode Vec<String>");
    // Remove the last 3 bytes to ensure the final string is corrupted.
    let new_len = encoded.len().saturating_sub(3);
    encoded.truncate(new_len);
    let result: Result<(Vec<String>, usize), _> = decode_from_slice(&encoded);
    assert!(
        result.is_err(),
        "decoding a truncated Vec<String> must return an error"
    );
}

// ---------------------------------------------------------------------------
// 19. Decode from empty input for Vec<u32> returns error
// ---------------------------------------------------------------------------

#[test]
fn test_decode_empty_input_vec_u32_returns_error() {
    let empty: &[u8] = &[];
    let result: Result<(Vec<u32>, usize), _> = decode_from_slice(empty);
    assert!(
        result.is_err(),
        "decoding empty bytes as Vec<u32> must return an error"
    );
}

// ---------------------------------------------------------------------------
// 20. Decode with invalid UTF-8 sequence in string returns error
// ---------------------------------------------------------------------------

#[test]
fn test_decode_invalid_utf8_string_returns_error() {
    // Manually construct: varint length 4 followed by 4 invalid UTF-8 bytes.
    let mut bytes: Vec<u8> = Vec::new();
    bytes.push(0x04u8); // length prefix: 4
    bytes.extend_from_slice(&[0xFF, 0xFE, 0xFD, 0xFC]); // invalid UTF-8
    let result: Result<(String, usize), _> = decode_from_slice(&bytes);
    assert!(
        result.is_err(),
        "invalid UTF-8 bytes must fail to decode as String"
    );
}

// ---------------------------------------------------------------------------
// 21. Decode struct with corrupted id field returns error or unexpected value
// ---------------------------------------------------------------------------

#[test]
fn test_decode_struct_corrupted_id_field() {
    let original = Data {
        id: 1,
        value: String::from("ok"),
    };
    let mut encoded = encode_to_vec(&original).expect("encode Data");
    // Overwrite the first byte to 0xFF which is an incomplete varint marker,
    // causing the id field (u32) to fail to decode.
    if !encoded.is_empty() {
        encoded[0] = 0xFF;
    }
    let result: Result<(Data, usize), _> = decode_from_slice(&encoded);
    // The corrupted first byte is an incomplete varint; decoding must fail.
    assert!(
        result.is_err(),
        "struct with corrupted id field (incomplete varint 0xFF) must fail"
    );
}

// ---------------------------------------------------------------------------
// 22. Multiple successful decodes from valid bytes
// ---------------------------------------------------------------------------

#[test]
fn test_multiple_successful_decodes_from_valid_bytes() {
    let a = Data {
        id: 1,
        value: String::from("first"),
    };
    let b = Data {
        id: 2,
        value: String::from("second"),
    };
    let c = Data {
        id: 3,
        value: String::from("third"),
    };

    let mut buffer: Vec<u8> = Vec::new();
    buffer.extend(encode_to_vec(&a).expect("encode a"));
    buffer.extend(encode_to_vec(&b).expect("encode b"));
    buffer.extend(encode_to_vec(&c).expect("encode c"));

    let (decoded_a, offset_a): (Data, usize) = decode_from_slice(&buffer).expect("decode a");
    let (decoded_b, offset_b): (Data, usize) =
        decode_from_slice(&buffer[offset_a..]).expect("decode b");
    let (decoded_c, offset_c): (Data, usize) =
        decode_from_slice(&buffer[offset_a + offset_b..]).expect("decode c");

    assert_eq!(a, decoded_a);
    assert_eq!(b, decoded_b);
    assert_eq!(c, decoded_c);
    assert_eq!(
        offset_a + offset_b + offset_c,
        buffer.len(),
        "all bytes must be consumed across three decodes"
    );
}
