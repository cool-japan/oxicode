//! Advanced error-handling tests for OxiCode — 22 distinct scenarios.
//!
//! Import note: `oxicode::error` exports `Error` (not `DecodeError`).
//! We alias it to `DecodeError` here to match the intended naming in the test spec.

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
use oxicode::error::Error as DecodeError;
use oxicode::{config, decode_from_slice, encode_to_vec};

// ---------------------------------------------------------------------------
// Helper types
// ---------------------------------------------------------------------------

#[derive(Debug, PartialEq, oxicode::Encode, oxicode::Decode)]
struct Simple {
    x: u32,
    y: u32,
}

#[derive(Debug, PartialEq, oxicode::Encode, oxicode::Decode)]
struct Nested {
    id: u64,
    inner: Simple,
}

#[derive(Debug, PartialEq, oxicode::Encode, oxicode::Decode)]
enum MyEnum {
    A,
    B(u32),
    C { x: u8, y: u8 },
}

// ---------------------------------------------------------------------------
// 1. Decode truncated data returns error
// ---------------------------------------------------------------------------
#[test]
fn test_01_decode_truncated_data_returns_error() {
    // Encode a u64, then chop off the last byte.
    let encoded = encode_to_vec(&0x0102030405060708u64).expect("encode");
    let truncated = &encoded[..encoded.len().saturating_sub(1)];
    let result: Result<(u64, usize), _> = decode_from_slice(truncated);
    assert!(result.is_err(), "truncated input must return an error");
}

// ---------------------------------------------------------------------------
// 2. Decode empty slice returns error
// ---------------------------------------------------------------------------
#[test]
fn test_02_decode_empty_slice_returns_error() {
    let result: Result<(u32, usize), _> = decode_from_slice(&[]);
    assert!(result.is_err(), "empty slice must return an error");
}

// ---------------------------------------------------------------------------
// 3. Decode 1 byte when more needed returns error
// ---------------------------------------------------------------------------
#[test]
fn test_03_decode_one_byte_when_more_needed() {
    // varint tag 0xFD (253) means "U64 follows — read 8 more bytes"; only 0 follow.
    let result: Result<(u64, usize), _> = decode_from_slice(&[0xFD]);
    assert!(
        result.is_err(),
        "single tag byte without body must return an error"
    );
}

// ---------------------------------------------------------------------------
// 4. Invalid bool byte (not 0 or 1) returns error
// ---------------------------------------------------------------------------
#[test]
fn test_04_invalid_bool_byte_returns_error() {
    let result: Result<(bool, usize), _> = decode_from_slice(&[2u8]);
    assert!(result.is_err(), "byte 2 is not a valid bool");
    let err = result.expect_err("must be an error");
    match err {
        DecodeError::InvalidBooleanValue(v) => {
            assert_eq!(v, 2, "error must carry the invalid byte");
        }
        other => panic!("expected InvalidBooleanValue, got: {:?}", other),
    }
}

// ---------------------------------------------------------------------------
// 5. String with invalid UTF-8 returns error
// ---------------------------------------------------------------------------
#[test]
fn test_05_string_invalid_utf8_returns_error() {
    // Encode a valid String first to learn the length-prefix format.
    let encoded = encode_to_vec(&String::from("ab")).expect("encode");
    // Replace the string body bytes with invalid UTF-8 (0xFF 0xFE).
    let mut corrupted = encoded.clone();
    let body_start = corrupted.len() - 2;
    corrupted[body_start] = 0xFF;
    corrupted[body_start + 1] = 0xFE;
    let result: Result<(String, usize), _> = decode_from_slice(&corrupted);
    assert!(result.is_err(), "invalid UTF-8 bytes must return an error");
}

// ---------------------------------------------------------------------------
// 6. Limit exceeded: encode large data, decode with small limit
// ---------------------------------------------------------------------------
#[test]
fn test_06_limit_exceeded_decode_with_small_limit() {
    let big: Vec<u8> = vec![0u8; 256];
    let encoded = encode_to_vec(&big).expect("encode");
    let small_cfg = config::standard().with_limit::<4>();
    let result: Result<(Vec<u8>, usize), _> =
        oxicode::decode_from_slice_with_config(&encoded, small_cfg);
    assert!(result.is_err(), "exceeding limit must return an error");
    let err = result.expect_err("must be an error");
    match err {
        DecodeError::LimitExceeded { .. } => {}
        other => panic!("expected LimitExceeded, got: {:?}", other),
    }
}

// ---------------------------------------------------------------------------
// 7. Unexpected enum variant returns error
// ---------------------------------------------------------------------------
#[test]
fn test_07_unexpected_enum_variant_returns_error() {
    // MyEnum has variants 0, 1, 2.  Encode variant index 99 manually.
    // With varint encoding, values < 128 are direct single bytes.
    let bad_variant: &[u8] = &[99u8];
    let result: Result<(MyEnum, usize), _> = decode_from_slice(bad_variant);
    assert!(result.is_err(), "variant 99 is not valid for MyEnum");
    let err = result.expect_err("must be an error");
    // The implementation may produce either UnexpectedVariant or InvalidData.
    match &err {
        DecodeError::UnexpectedVariant { .. } | DecodeError::InvalidData { .. } => {}
        other => panic!(
            "expected UnexpectedVariant or InvalidData for bad enum variant, got: {:?}",
            other
        ),
    }
}

// ---------------------------------------------------------------------------
// 8. Decode wrong type (u32 vs u64) — different byte width causes error or mismatch
// ---------------------------------------------------------------------------
#[test]
fn test_08_decode_wrong_type_u32_as_u64() {
    // With fixed-int encoding u32 is 4 bytes, u64 is 8 bytes.
    // Encoding a u32 and decoding as u64 should either fail or produce a
    // value with a smaller byte count consumed.  We verify no panic occurs.
    let fixed_cfg = config::standard().with_fixed_int_encoding();
    let encoded = oxicode::encode_to_vec_with_config(&42u32, fixed_cfg).expect("encode");
    // With fixed encoding u32 is exactly 4 bytes; u64 needs 8 → UnexpectedEnd.
    let result: Result<(u64, usize), _> =
        oxicode::decode_from_slice_with_config(&encoded, fixed_cfg);
    assert!(
        result.is_err(),
        "decoding 4-byte u32 payload as u64 (needs 8 bytes) must fail"
    );
}

// ---------------------------------------------------------------------------
// 9. Error display is non-empty string
// ---------------------------------------------------------------------------
#[test]
fn test_09_error_display_is_non_empty() {
    let err = DecodeError::UnexpectedEnd { additional: 4 };
    let display = format!("{}", err);
    assert!(!display.is_empty(), "Display output must not be empty");
    assert!(
        display.contains('4') || display.to_lowercase().contains("end"),
        "display must mention the missing bytes or 'end': {display}"
    );
}

// ---------------------------------------------------------------------------
// 10. Error implements Debug
// ---------------------------------------------------------------------------
#[test]
fn test_10_error_implements_debug() {
    let err = DecodeError::InvalidBooleanValue(42);
    let debug = format!("{:?}", err);
    assert!(!debug.is_empty(), "Debug output must not be empty");
    assert!(
        debug.contains("42") || debug.contains("Invalid"),
        "debug must mention 42 or Invalid: {debug}"
    );
}

// ---------------------------------------------------------------------------
// 11. Truncated varint returns error
// ---------------------------------------------------------------------------
#[test]
fn test_11_truncated_varint_returns_error() {
    // 0xFD = U64_BYTE tag; the 8-byte body is missing entirely.
    let result: Result<(u64, usize), _> = decode_from_slice(&[0xFD]);
    assert!(result.is_err(), "tag without body must return an error");
    let err = result.expect_err("must be an error");
    match err {
        DecodeError::UnexpectedEnd { .. } => {}
        other => panic!("expected UnexpectedEnd, got: {:?}", other),
    }
}

// ---------------------------------------------------------------------------
// 12. Truncated string data returns error
// ---------------------------------------------------------------------------
#[test]
fn test_12_truncated_string_data_returns_error() {
    // Encode "hello" then drop the last two body bytes.
    let encoded = encode_to_vec(&String::from("hello")).expect("encode");
    let truncated = &encoded[..encoded.len() - 2];
    let result: Result<(String, usize), _> = decode_from_slice(truncated);
    assert!(
        result.is_err(),
        "truncated string body must return an error"
    );
}

// ---------------------------------------------------------------------------
// 13. Truncated Vec length prefix returns error
// ---------------------------------------------------------------------------
#[test]
fn test_13_truncated_vec_length_prefix_returns_error() {
    // A Vec length uses a varint.  Send the U64_BYTE tag (0xFD) alone to
    // simulate a truncated length prefix.
    let result: Result<(Vec<u8>, usize), _> = decode_from_slice(&[0xFD]);
    assert!(
        result.is_err(),
        "tag byte without length body must return an error"
    );
}

// ---------------------------------------------------------------------------
// 14. Nested decode error propagation
// ---------------------------------------------------------------------------
#[test]
fn test_14_nested_decode_error_propagation() {
    // Encode a valid Nested, then corrupt a middle byte so the inner Simple
    // becomes undecodable.
    let val = Nested {
        id: 100,
        inner: Simple { x: 1, y: 2 },
    };
    let mut encoded = encode_to_vec(&val).expect("encode");
    // Flip the last byte to corrupt the inner struct.
    if let Some(last) = encoded.last_mut() {
        *last ^= 0xFF;
    }
    let result: Result<(Nested, usize), _> = decode_from_slice(&encoded);
    // Either the decode fails outright or the decoded value differs; the main
    // goal is no panic and the error path is exercised.
    let _ = result; // errors are acceptable; panics are not
}

// ---------------------------------------------------------------------------
// 15. Decode with zero-byte limit rejects Vec with any elements
// ---------------------------------------------------------------------------
#[test]
fn test_15_zero_byte_limit_fails_for_non_empty_vec() {
    let zero_cfg = config::standard().with_limit::<0>();
    // A Vec with 100 elements has a length prefix that already exceeds limit=0.
    let big: Vec<u8> = vec![1u8; 100];
    let encoded = encode_to_vec(&big).expect("encode");
    let result: Result<(Vec<u8>, usize), _> =
        oxicode::decode_from_slice_with_config(&encoded, zero_cfg);
    assert!(
        result.is_err(),
        "limit=0 must reject decoding a 100-element Vec"
    );
}

// ---------------------------------------------------------------------------
// 16. Config limit — large data exceeds the limit
// ---------------------------------------------------------------------------
#[test]
fn test_16_small_limit_rejects_large_data() {
    let cfg = config::standard().with_limit::<4>();
    // A Vec of 100 bytes will encode to far more than 4 bytes.
    let big: Vec<u8> = vec![0xABu8; 100];
    let big_encoded = encode_to_vec(&big).expect("encode");
    let big_result: Result<(Vec<u8>, usize), _> =
        oxicode::decode_from_slice_with_config(&big_encoded, cfg);
    assert!(big_result.is_err(), "100-element Vec must exceed limit=4");
    let err = big_result.expect_err("must be an error");
    match err {
        DecodeError::LimitExceeded { .. } => {}
        other => panic!("expected LimitExceeded, got: {:?}", other),
    }
}

// ---------------------------------------------------------------------------
// 17. Error for invalid NonZero value (value = 0)
// ---------------------------------------------------------------------------
#[test]
fn test_17_nonzero_type_is_zero_error() {
    use std::num::NonZeroU32;
    // Encode the integer 0 as a u32, then try to decode it as NonZeroU32.
    let encoded = encode_to_vec(&0u32).expect("encode");
    let result: Result<(NonZeroU32, usize), _> = decode_from_slice(&encoded);
    assert!(result.is_err(), "zero value must fail for NonZeroU32");
    let err = result.expect_err("must be an error");
    match err {
        DecodeError::NonZeroTypeIsZero { .. } => {}
        other => panic!("expected NonZeroTypeIsZero, got: {:?}", other),
    }
}

// ---------------------------------------------------------------------------
// 18. Error returned from decode is distinct per case
// ---------------------------------------------------------------------------
#[test]
fn test_18_errors_are_distinct_per_case() {
    let bool_err: DecodeError = decode_from_slice::<bool>(&[5u8]).expect_err("must error");
    let end_err: DecodeError = decode_from_slice::<u64>(&[]).expect_err("must error");
    assert_ne!(bool_err, end_err, "different error cases must not be equal");
}

// ---------------------------------------------------------------------------
// 19. Decode corrupted struct (modify middle bytes)
// ---------------------------------------------------------------------------
#[test]
fn test_19_corrupted_struct_middle_bytes() {
    let val = Simple { x: 1000, y: 2000 };
    let mut encoded = encode_to_vec(&val).expect("encode");
    let mid = encoded.len() / 2;
    encoded[mid] ^= 0xAA;
    // Either decodes to a different value or returns an error — no panics.
    let _result: Result<(Simple, usize), _> = decode_from_slice(&encoded);
}

// ---------------------------------------------------------------------------
// 20. Multiple sequential decodes, error on third
// ---------------------------------------------------------------------------
#[test]
fn test_20_sequential_decodes_error_on_third() {
    let a = encode_to_vec(&1u32).expect("encode a");
    let b = encode_to_vec(&2u32).expect("encode b");
    // Third "slice" is intentionally empty — cannot decode a u32 from nothing.
    let bad: Vec<u8> = vec![];

    let (va, _) = decode_from_slice::<u32>(&a).expect("decode a");
    let (vb, _) = decode_from_slice::<u32>(&b).expect("decode b");
    let result_c: Result<(u32, usize), _> = decode_from_slice(&bad);

    assert_eq!(va, 1u32);
    assert_eq!(vb, 2u32);
    assert!(result_c.is_err(), "third decode on empty slice must fail");
}

// ---------------------------------------------------------------------------
// 21. Decode error pattern matching works
// ---------------------------------------------------------------------------
#[test]
fn test_21_error_pattern_matching_works() {
    let limit_err = DecodeError::LimitExceeded {
        limit: 10,
        found: 100,
    };
    let matched = match &limit_err {
        DecodeError::LimitExceeded { limit, found } => *found > *limit,
        _ => false,
    };
    assert!(matched, "pattern match on LimitExceeded must succeed");

    let variant_err = DecodeError::UnexpectedVariant {
        found: 42,
        type_name: "Foo",
    };
    let matched_variant = match &variant_err {
        DecodeError::UnexpectedVariant { found, .. } => *found == 42,
        _ => false,
    };
    assert!(
        matched_variant,
        "pattern match on UnexpectedVariant must succeed"
    );
}

// ---------------------------------------------------------------------------
// 22. Decoding with too-large length prefix (limit exceeded)
// ---------------------------------------------------------------------------
#[test]
fn test_22_too_large_length_prefix_exceeds_limit() {
    // Encode a Vec of 1000 u8 bytes (large payload).
    let large: Vec<u8> = vec![0u8; 1000];
    let encoded = encode_to_vec(&large).expect("encode");

    // Decode with a tight limit of 8 bytes.
    let tight_cfg = config::standard().with_limit::<8>();
    let result: Result<(Vec<u8>, usize), _> =
        oxicode::decode_from_slice_with_config(&encoded, tight_cfg);
    assert!(
        result.is_err(),
        "1000-byte Vec with 8-byte limit must return an error"
    );
    let err = result.expect_err("must be an error");
    match err {
        DecodeError::LimitExceeded { .. } => {}
        other => panic!("expected LimitExceeded, got: {:?}", other),
    }
}
