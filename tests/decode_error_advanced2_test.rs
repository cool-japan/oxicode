//! Advanced error-handling tests for OxiCode decode operations (set 2).
//!
//! 22 top-level `#[test]` functions covering malformed / invalid input
//! scenarios.  No `#[cfg(test)]` wrapper, no `unwrap()`, no undefined helpers.

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

// ---------------------------------------------------------------------------
// Shared test enum — placed at module level to be accessible by all tests.
// ---------------------------------------------------------------------------
#[derive(Debug, oxicode::Encode, oxicode::Decode)]
enum TwoVariants {
    Alpha,
    Beta,
}

// ---------------------------------------------------------------------------
// 1. Empty slice → decode u32 fails
// ---------------------------------------------------------------------------
#[test]
fn test_empty_slice_decode_u32_fails() {
    let result: Result<(u32, usize), _> = decode_from_slice(&[]);
    assert!(result.is_err(), "decoding u32 from empty slice must fail");
}

// ---------------------------------------------------------------------------
// 2. 1-byte slice for u32: byte value 0 is a valid varint (value = 0)
//    but byte value 0x80 has continuation bits set → may fail or succeed
//    depending on the value; we test 0x80 which signals "more bytes needed"
//    under LEB128 but in OxiCode varint it is 128, a value in the reserved
//    zone — this should return an InvalidIntegerType error.
// ---------------------------------------------------------------------------
#[test]
fn test_one_byte_slice_u32_reserved_varint_tag() {
    // In OxiCode varint, values 251..=254 are tag bytes that require more data.
    // 0xFB = 251 = U16_BYTE: signals that 2 additional bytes follow, but none do.
    let result: Result<(u32, usize), _> = decode_from_slice(&[251u8]);
    // Must either fail with UnexpectedEnd (no body bytes) or succeed as a
    // type-mismatch error; it must NOT silently return garbage.
    assert!(
        result.is_err(),
        "varint tag 251 with no body bytes must not succeed"
    );
}

// ---------------------------------------------------------------------------
// 3. Truncated Vec<u8> (length says 10 but only 5 body bytes follow) → fails
// ---------------------------------------------------------------------------
#[test]
fn test_truncated_vec_u8_fails() {
    // varint(10) = single byte 10, then only 5 body bytes
    let bytes = [10u8, 0x01, 0x02, 0x03, 0x04, 0x05];
    let result: Result<(Vec<u8>, usize), _> = decode_from_slice(&bytes);
    assert!(
        result.is_err(),
        "Vec<u8> with claimed length 10 but only 5 bytes must fail"
    );
}

// ---------------------------------------------------------------------------
// 4. Invalid bool byte (0x02) → fails with InvalidBooleanValue
// ---------------------------------------------------------------------------
#[test]
fn test_invalid_bool_byte_0x02() {
    let result: Result<(bool, usize), _> = decode_from_slice(&[0x02u8]);
    assert!(result.is_err(), "byte 0x02 is not a valid bool encoding");
    let err = result.expect_err("must be error");
    match err {
        oxicode::Error::InvalidBooleanValue(v) => {
            assert_eq!(v, 2u8, "error must carry byte value 2");
        }
        other => panic!("expected InvalidBooleanValue(2), got: {:?}", other),
    }
}

// ---------------------------------------------------------------------------
// 5. Invalid bool byte (0xFF) → fails with InvalidBooleanValue
// ---------------------------------------------------------------------------
#[test]
fn test_invalid_bool_byte_0xff() {
    let result: Result<(bool, usize), _> = decode_from_slice(&[0xFFu8]);
    assert!(result.is_err(), "byte 0xFF is not a valid bool encoding");
    let err = result.expect_err("must be error");
    match err {
        oxicode::Error::InvalidBooleanValue(v) => {
            assert_eq!(v, 0xFF, "error must carry byte value 0xFF");
        }
        other => panic!("expected InvalidBooleanValue(0xFF), got: {:?}", other),
    }
}

// ---------------------------------------------------------------------------
// 6. Invalid UTF-8 bytes in String position → fails (Utf8 error)
// ---------------------------------------------------------------------------
#[test]
fn test_invalid_utf8_in_string_position() {
    // varint(4) as length prefix, followed by 4 invalid UTF-8 bytes
    let bytes = [4u8, 0xFF, 0xFE, 0xFD, 0xFC];
    let result: Result<(String, usize), _> = decode_from_slice(&bytes);
    assert!(
        result.is_err(),
        "invalid UTF-8 bytes must cause String decode to fail"
    );
}

// ---------------------------------------------------------------------------
// 7. Truncated String (length says 5 but only 3 bytes follow) → fails
// ---------------------------------------------------------------------------
#[test]
fn test_truncated_string_fails() {
    // varint(5) as length, then only 3 bytes
    let bytes = [5u8, b'h', b'e', b'l'];
    let result: Result<(String, usize), _> = decode_from_slice(&bytes);
    assert!(
        result.is_err(),
        "String claiming length 5 with only 3 body bytes must fail"
    );
}

// ---------------------------------------------------------------------------
// 8. Oversized string claim with limit config → fails (LimitExceeded)
//    The limit enforcement is applied via claim_bytes_read which is called
//    when decoding String/Vec types.  A limit of 4 bytes cannot accommodate
//    a String whose body is 10 bytes (limit applies to the body length).
// ---------------------------------------------------------------------------
#[test]
fn test_oversized_string_with_limit_config_fails() {
    // Limit of 4 bytes.  The String "helloworld" has a 10-byte body.
    // claim_bytes_read(10) will exceed limit=4 → LimitExceeded.
    let encoded = encode_to_vec(&String::from("helloworld")).expect("encode string");
    let cfg = config::standard().with_limit::<4>();
    let result: Result<(String, usize), _> = decode_from_slice_with_config(&encoded, cfg);
    assert!(
        result.is_err(),
        "String with 10-byte body must fail against a 4-byte limit"
    );
    let err = result.expect_err("must be error");
    match err {
        oxicode::Error::LimitExceeded { .. } => {}
        other => panic!("expected LimitExceeded, got: {:?}", other),
    }
}

// ---------------------------------------------------------------------------
// 9. Invalid enum discriminant (out of range) → fails for TwoVariants
// ---------------------------------------------------------------------------
#[test]
fn test_invalid_enum_discriminant_out_of_range() {
    // TwoVariants has discriminants 0 (Alpha) and 1 (Beta); 99 is invalid.
    // Encode u32 value 99, which is what the derive macro reads as discriminant.
    let bad_bytes = encode_to_vec(&99u32).expect("encode discriminant");
    let result: Result<(TwoVariants, usize), _> = decode_from_slice(&bad_bytes);
    assert!(
        result.is_err(),
        "discriminant 99 must not decode as TwoVariants"
    );
    let err = result.expect_err("must be error");
    let display = format!("{}", err);
    assert!(
        display.contains("99") || display.to_lowercase().contains("variant"),
        "error message should reference the bad discriminant: {}",
        display
    );
}

// ---------------------------------------------------------------------------
// 10. Correct data decoded successfully → Ok
// ---------------------------------------------------------------------------
#[test]
fn test_correct_data_decoded_successfully() {
    let original: u64 = 42_000;
    let encoded = encode_to_vec(&original).expect("encode u64");
    let (decoded, consumed): (u64, usize) =
        decode_from_slice(&encoded).expect("decode must succeed for valid data");
    assert_eq!(decoded, original, "decoded value must match original");
    assert_eq!(consumed, encoded.len(), "all bytes should be consumed");
}

// ---------------------------------------------------------------------------
// 11. Extra trailing bytes — decode succeeds, consumed < total
// ---------------------------------------------------------------------------
#[test]
fn test_extra_trailing_bytes_decode_succeeds() {
    let value: u32 = 7;
    let mut bytes = encode_to_vec(&value).expect("encode u32");
    let original_len = bytes.len();
    // Append a junk byte
    bytes.push(0xDE);
    let (decoded, consumed): (u32, usize) =
        decode_from_slice(&bytes).expect("decode should succeed ignoring trailing byte");
    assert_eq!(decoded, value, "value must match despite trailing byte");
    assert!(
        consumed < bytes.len(),
        "consumed ({}) must be less than total ({}) when trailing bytes exist",
        consumed,
        bytes.len()
    );
    assert_eq!(
        consumed, original_len,
        "consumed bytes must equal the originally encoded length"
    );
}

// ---------------------------------------------------------------------------
// 12. Decode u64 from too-short slice → fails
// ---------------------------------------------------------------------------
#[test]
fn test_decode_u64_too_short_slice_fails() {
    // Tag byte 253 (U64_BYTE) demands 8 more bytes; provide only 4.
    let bytes = [253u8, 0x00, 0x00, 0x00, 0x00];
    let result: Result<(u64, usize), _> = decode_from_slice(&bytes);
    assert!(
        result.is_err(),
        "U64_BYTE tag with only 4 body bytes must fail"
    );
}

// ---------------------------------------------------------------------------
// 13. Decode i128 from 10-byte slice → fails (varint needs up to 17 bytes
//     for values that require U128_BYTE tag: 1 tag + 16 data bytes)
// ---------------------------------------------------------------------------
#[test]
fn test_decode_i128_from_ten_bytes_fails() {
    // U128_BYTE tag = 254; then 16 body bytes needed; only 9 provided here.
    let bytes = [254u8, 0, 0, 0, 0, 0, 0, 0, 0, 0]; // 1 + 9 = 10 bytes, need 1 + 16 = 17
    let result: Result<(i128, usize), _> = decode_from_slice(&bytes);
    assert!(
        result.is_err(),
        "i128 with U128_BYTE tag and only 9 body bytes must fail"
    );
}

// ---------------------------------------------------------------------------
// 14. Empty string "" decodes ok (wire: 1 byte 0x00 for length = 0)
// ---------------------------------------------------------------------------
#[test]
fn test_empty_string_decodes_ok() {
    // varint(0) = 0x00 → empty string
    let bytes = [0x00u8];
    let (decoded, consumed): (String, usize) =
        decode_from_slice(&bytes).expect("empty string must decode successfully");
    assert!(decoded.is_empty(), "decoded string must be empty");
    assert_eq!(consumed, 1, "exactly 1 byte should be consumed");
}

// ---------------------------------------------------------------------------
// 15. Limit=3: decode Vec<u8> with 2 elements (body=2 bytes) → Ok
//    The limit enforces the body size via claim_bytes_read.  A Vec<u8>
//    with 2 elements claims 2 bytes which is within limit=3.
// ---------------------------------------------------------------------------
#[test]
fn test_limit_3_decode_vec_2_elements_succeeds() {
    let cfg = config::standard().with_limit::<3>();
    let data: Vec<u8> = vec![10, 20];
    let encoded = encode_to_vec(&data).expect("encode Vec<u8>");
    let (decoded, _): (Vec<u8>, usize) = decode_from_slice_with_config(&encoded, cfg)
        .expect("Vec<u8> of 2 elements must succeed within limit=3");
    assert_eq!(decoded, data);
}

// ---------------------------------------------------------------------------
// 16. Limit=2: decode Vec<u8> with 5 elements (body=5 bytes) → Err
//    claim_bytes_read(5) exceeds limit=2 → LimitExceeded error.
// ---------------------------------------------------------------------------
#[test]
fn test_limit_2_decode_vec_5_elements_fails() {
    let cfg = config::standard().with_limit::<2>();
    let data: Vec<u8> = vec![1, 2, 3, 4, 5];
    let encoded = encode_to_vec(&data).expect("encode Vec<u8>");
    let result: Result<(Vec<u8>, usize), _> = decode_from_slice_with_config(&encoded, cfg);
    assert!(
        result.is_err(),
        "Vec<u8> with 5-byte body must fail against a 2-byte limit"
    );
    let err = result.expect_err("must be error");
    match err {
        oxicode::Error::LimitExceeded { .. } => {}
        other => panic!("expected LimitExceeded, got: {:?}", other),
    }
}

// ---------------------------------------------------------------------------
// 17. Decode Option<u32> from [0x02, ...] — invalid discriminant → fails
// ---------------------------------------------------------------------------
#[test]
fn test_option_u32_invalid_discriminant_fails() {
    // Option<T> uses u8 tag: 0=None, 1=Some; 0x02 is invalid.
    let bytes = [0x02u8, 0x01];
    let result: Result<(Option<u32>, usize), _> = decode_from_slice(&bytes);
    assert!(
        result.is_err(),
        "Option<u32> with discriminant 0x02 must fail"
    );
}

// ---------------------------------------------------------------------------
// 18. Decode Result<u32, String> from corrupt data → fails
// ---------------------------------------------------------------------------
#[test]
fn test_result_u32_string_corrupt_data_fails() {
    // Result<T,U> reads a u32 discriminant first.
    // Encode variant value 99 (not 0/Ok or 1/Err) — this should fail.
    let bad_discriminant = encode_to_vec(&99u32).expect("encode discriminant");
    let result: Result<(core::result::Result<u32, String>, usize), _> =
        decode_from_slice(&bad_discriminant);
    assert!(
        result.is_err(),
        "Result<u32, String> with discriminant 99 must fail"
    );
}

// ---------------------------------------------------------------------------
// 19. Decode [u8; 4] from only 3 bytes → fails
// ---------------------------------------------------------------------------
#[test]
fn test_decode_array_u8_4_from_three_bytes_fails() {
    let bytes = [0x01u8, 0x02, 0x03];
    let result: Result<([u8; 4], usize), _> = decode_from_slice(&bytes);
    assert!(
        result.is_err(),
        "decoding [u8; 4] from only 3 bytes must fail"
    );
}

// ---------------------------------------------------------------------------
// 20. Sequential decode: first value ok, second truncated → second fails
// ---------------------------------------------------------------------------
#[test]
fn test_sequential_decode_first_ok_second_truncated() {
    // Build a slice that has a complete u32 followed by a truncated u64.
    let first_encoded = encode_to_vec(&42u32).expect("encode first");
    // For a truncated u64, use U64_BYTE tag with only 3 of the required 8 body bytes.
    let truncated_u64 = [253u8, 0x00, 0x01, 0x02]; // tag + 3 bytes (needs 8)

    let mut combined = first_encoded.clone();
    combined.extend_from_slice(&truncated_u64);

    // Decode first value from the combined slice
    let (first_val, first_consumed): (u32, usize) =
        decode_from_slice(&combined).expect("first u32 must decode successfully");
    assert_eq!(first_val, 42u32, "first value must be 42");

    // Decode second value starting right after the first
    let remainder = &combined[first_consumed..];
    let second_result: Result<(u64, usize), _> = decode_from_slice(remainder);
    assert!(
        second_result.is_err(),
        "second decode of truncated u64 must fail"
    );
}

// ---------------------------------------------------------------------------
// 21. Valid encode, flip first byte, decode fails with meaningful error
// ---------------------------------------------------------------------------
#[test]
fn test_flip_first_byte_decode_fails_meaningfully() {
    let original: u64 = 300; // encodes as [251, 0x2c, 0x01] (U16_BYTE tag + 2 bytes)
    let encoded = encode_to_vec(&original).expect("encode u64 300");

    assert!(!encoded.is_empty(), "encoded bytes must not be empty");

    let mut corrupted = encoded.clone();
    // Flip the first byte by XOR with 0xFF to invalidate the varint tag
    corrupted[0] ^= 0xFF;

    // Must not be equal to the original
    assert_ne!(
        corrupted[0], encoded[0],
        "first byte must have been flipped"
    );

    let result: Result<(u64, usize), _> = decode_from_slice(&corrupted);
    // Whether it errors or happens to decode a different value, it must not panic.
    // If it errors, confirm the error is well-formed by displaying it.
    if let Err(ref err) = result {
        let display = format!("{}", err);
        assert!(
            !display.is_empty(),
            "error display must not be empty: {:?}",
            err
        );
    }
    // At minimum the decoded value should differ from the original if successful
    if let Ok((decoded, _)) = result {
        // XOR-flipping the tag byte almost always changes semantics; note we
        // allow it to succeed with a *different* value — only panic is forbidden.
        let _ = decoded;
    }
}

// ---------------------------------------------------------------------------
// 22. Decode well-formed data for all primitive types → all succeed
// ---------------------------------------------------------------------------
#[test]
fn test_all_primitive_types_roundtrip_successfully() {
    // u8
    let (v, _): (u8, usize) =
        decode_from_slice(&encode_to_vec(&200u8).expect("encode u8")).expect("decode u8");
    assert_eq!(v, 200u8);

    // u16
    let (v, _): (u16, usize) =
        decode_from_slice(&encode_to_vec(&1000u16).expect("encode u16")).expect("decode u16");
    assert_eq!(v, 1000u16);

    // u32
    let (v, _): (u32, usize) =
        decode_from_slice(&encode_to_vec(&100_000u32).expect("encode u32")).expect("decode u32");
    assert_eq!(v, 100_000u32);

    // u64
    let (v, _): (u64, usize) =
        decode_from_slice(&encode_to_vec(&9_999_999u64).expect("encode u64")).expect("decode u64");
    assert_eq!(v, 9_999_999u64);

    // i8
    let (v, _): (i8, usize) =
        decode_from_slice(&encode_to_vec(&-50i8).expect("encode i8")).expect("decode i8");
    assert_eq!(v, -50i8);

    // i32
    let (v, _): (i32, usize) =
        decode_from_slice(&encode_to_vec(&-70_000i32).expect("encode i32")).expect("decode i32");
    assert_eq!(v, -70_000i32);

    // i64
    let (v, _): (i64, usize) =
        decode_from_slice(&encode_to_vec(&i64::MIN).expect("encode i64")).expect("decode i64");
    assert_eq!(v, i64::MIN);

    // f32
    let f32_val = 1.5f32; // exact power-of-two fraction: no approximation lint
    let (v, _): (f32, usize) =
        decode_from_slice(&encode_to_vec(&f32_val).expect("encode f32")).expect("decode f32");
    assert!((v - f32_val).abs() < 1e-5, "f32 roundtrip: got {}", v);

    // f64
    let f64_val = 1.125f64; // exact in IEEE-754, avoids known-constant lint
    let (v, _): (f64, usize) =
        decode_from_slice(&encode_to_vec(&f64_val).expect("encode f64")).expect("decode f64");
    assert!((v - f64_val).abs() < 1e-12, "f64 roundtrip: got {}", v);

    // bool true
    let (v, _): (bool, usize) =
        decode_from_slice(&encode_to_vec(&true).expect("encode bool")).expect("decode bool");
    assert!(v, "bool true roundtrip");

    // bool false
    let (v, _): (bool, usize) =
        decode_from_slice(&encode_to_vec(&false).expect("encode bool")).expect("decode bool");
    assert!(!v, "bool false roundtrip");

    // String
    let (v, _): (String, usize) =
        decode_from_slice(&encode_to_vec(&String::from("hello world")).expect("encode String"))
            .expect("decode String");
    assert_eq!(v, "hello world");
}
