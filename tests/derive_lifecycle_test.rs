//! Tests for derived types with lifecycle patterns in OxiCode.
//!
//! Focuses on types with lifetime parameters, BorrowDecode, and the
//! `borrow_decode_from_slice` API.

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
use oxicode::{borrow_decode_from_slice, decode_from_slice, encode_to_vec};
use std::borrow::Cow;

// ===== Test 1: &[u8] borrow_decode from encoded Vec<u8> =====

#[test]
fn test_borrow_decode_u8_slice() {
    let original: Vec<u8> = vec![1u8, 2, 3, 4, 5];
    let encoded = encode_to_vec(&original).expect("encode Vec<u8> failed");
    let (decoded, consumed): (&[u8], _) =
        borrow_decode_from_slice(&encoded).expect("borrow_decode &[u8] failed");
    assert_eq!(decoded, &[1u8, 2, 3, 4, 5]);
    assert_eq!(consumed, encoded.len());
}

// ===== Test 2: &str borrow_decode from encoded String =====

#[test]
fn test_borrow_decode_str() {
    let original = String::from("hello world");
    let encoded = encode_to_vec(&original).expect("encode String failed");
    let (decoded, consumed): (&str, _) =
        borrow_decode_from_slice(&encoded).expect("borrow_decode &str failed");
    assert_eq!(decoded, "hello world");
    assert_eq!(consumed, encoded.len());
}

// ===== Test 3: Cow<'_, str> borrow decode comes back as Cow::Borrowed =====

#[test]
fn test_borrow_decode_cow_str_is_borrowed() {
    let original: Cow<'static, str> = Cow::Owned(String::from("test data"));
    let encoded = encode_to_vec(&original).expect("encode Cow<str> failed");
    let (decoded, _): (Cow<str>, _) =
        borrow_decode_from_slice(&encoded).expect("borrow_decode Cow<str> failed");
    assert_eq!(decoded.as_ref(), "test data");
    assert!(
        matches!(decoded, Cow::Borrowed(_)),
        "borrow_decode should give Cow::Borrowed"
    );
}

// ===== Test 4: Cow<'_, [u8]> borrow decode comes back as Cow::Borrowed =====

#[test]
fn test_borrow_decode_cow_bytes_is_borrowed() {
    let original: Cow<'static, [u8]> = Cow::Owned(vec![10u8, 20, 30, 40]);
    let encoded = encode_to_vec(&original).expect("encode Cow<[u8]> failed");
    let (decoded, _): (Cow<[u8]>, _) =
        borrow_decode_from_slice(&encoded).expect("borrow_decode Cow<[u8]> failed");
    assert_eq!(decoded.as_ref(), &[10u8, 20, 30, 40]);
    assert!(
        matches!(decoded, Cow::Borrowed(_)),
        "borrow_decode should give Cow::Borrowed for &[u8]"
    );
}

// ===== Test 5: &[u8] borrow_decode — verify it actually borrows from buffer =====

#[test]
fn test_borrow_decode_u8_slice_borrows_from_buffer() {
    let original: Vec<u8> = vec![7u8, 8, 9];
    let encoded = encode_to_vec(&original).expect("encode Vec<u8> failed");

    // decoded must live within the scope of encoded
    let (decoded, _): (&[u8], _) =
        borrow_decode_from_slice(&encoded).expect("borrow_decode &[u8] failed");

    // The decoded slice references bytes inside encoded — use both while encoded is in scope
    assert_eq!(decoded.len(), 3);
    assert_eq!(decoded[0], 7u8);
    assert_eq!(decoded[1], 8u8);
    assert_eq!(decoded[2], 9u8);

    // encoded is still alive here, proving decoded borrows from it without cloning
    let _ = &encoded;
}

// ===== Test 6: &str borrow_decode — roundtrip for unicode content =====

#[test]
fn test_borrow_decode_str_unicode() {
    let original = String::from("日本語テスト — こんにちは世界");
    let encoded = encode_to_vec(&original).expect("encode unicode String failed");
    let (decoded, consumed): (&str, _) =
        borrow_decode_from_slice(&encoded).expect("borrow_decode unicode &str failed");
    assert_eq!(decoded, "日本語テスト — こんにちは世界");
    assert_eq!(consumed, encoded.len());
}

// ===== Test 7: &[u8] borrow_decode — empty slice =====

#[test]
fn test_borrow_decode_u8_slice_empty() {
    let original: Vec<u8> = vec![];
    let encoded = encode_to_vec(&original).expect("encode empty Vec<u8> failed");
    let (decoded, consumed): (&[u8], _) =
        borrow_decode_from_slice(&encoded).expect("borrow_decode empty &[u8] failed");
    assert_eq!(decoded, &[] as &[u8]);
    assert_eq!(consumed, encoded.len());
}

// ===== Test 8: &str borrow_decode — empty string =====

#[test]
fn test_borrow_decode_str_empty() {
    let original = String::from("");
    let encoded = encode_to_vec(&original).expect("encode empty String failed");
    let (decoded, consumed): (&str, _) =
        borrow_decode_from_slice(&encoded).expect("borrow_decode empty &str failed");
    assert_eq!(decoded, "");
    assert_eq!(consumed, encoded.len());
}

// ===== Test 9: Owned Vec<u8> decode (non-borrow version) for comparison =====

#[test]
fn test_decode_owned_vec_u8() {
    let original: Vec<u8> = vec![100u8, 101, 102, 103];
    let encoded = encode_to_vec(&original).expect("encode Vec<u8> failed");
    let (decoded, consumed): (Vec<u8>, _) =
        decode_from_slice(&encoded).expect("decode_from_slice Vec<u8> failed");
    assert_eq!(decoded, vec![100u8, 101, 102, 103]);
    assert_eq!(consumed, encoded.len());
}

// ===== Test 10: Owned String decode (non-borrow version) for comparison =====

#[test]
fn test_decode_owned_string() {
    let original = String::from("owned string decode");
    let encoded = encode_to_vec(&original).expect("encode String failed");
    let (decoded, consumed): (String, _) =
        decode_from_slice(&encoded).expect("decode_from_slice String failed");
    assert_eq!(decoded, "owned string decode");
    assert_eq!(consumed, encoded.len());
}

// ===== Test 11: Cow<'static, str> encode then decode_from_slice returns Owned =====

#[test]
fn test_cow_static_str_decode_is_owned() {
    let original: Cow<'static, str> = Cow::Borrowed("static borrowed string");
    let encoded = encode_to_vec(&original).expect("encode Cow<'static, str> failed");
    let (decoded, consumed): (Cow<str>, _) =
        decode_from_slice(&encoded).expect("decode_from_slice Cow<str> failed");
    assert_eq!(decoded.as_ref(), "static borrowed string");
    assert_eq!(consumed, encoded.len());
    // decode_from_slice uses Decode (not BorrowDecode), so result is always Cow::Owned
    assert!(
        matches!(decoded, Cow::Owned(_)),
        "decode_from_slice should give Cow::Owned"
    );
}

// ===== Test 12: Cow<'_, [u8]> borrow — verify Borrowed variant explicitly =====

#[test]
fn test_borrow_decode_cow_bytes_variant_check() {
    let original: Vec<u8> = vec![0xDE, 0xAD, 0xBE, 0xEF];
    let as_cow: Cow<[u8]> = Cow::Owned(original);
    let encoded = encode_to_vec(&as_cow).expect("encode Cow<[u8]> failed");
    let (decoded, _): (Cow<[u8]>, _) =
        borrow_decode_from_slice(&encoded).expect("borrow_decode Cow<[u8]> variant check failed");
    assert_eq!(&decoded[..], &[0xDE, 0xAD, 0xBE, 0xEF]);
    match decoded {
        Cow::Borrowed(slice) => {
            assert_eq!(slice, &[0xDE, 0xAD, 0xBE, 0xEF]);
        }
        Cow::Owned(_) => panic!("expected Cow::Borrowed from borrow_decode"),
    }
}

// ===== Test 13: &[u8] borrow_decode — buffer with all byte values 0–255 =====

#[test]
fn test_borrow_decode_u8_slice_all_byte_values() {
    let original: Vec<u8> = (0u8..=255u8).collect();
    let encoded = encode_to_vec(&original).expect("encode all-bytes Vec<u8> failed");
    let (decoded, consumed): (&[u8], _) =
        borrow_decode_from_slice(&encoded).expect("borrow_decode all-bytes &[u8] failed");
    let expected: Vec<u8> = (0u8..=255u8).collect();
    assert_eq!(decoded, expected.as_slice());
    assert_eq!(consumed, encoded.len());
}

// ===== Test 14: Option<Vec<u8>> decode (owned comparison — Option<&[u8]> not directly supported) =====

#[test]
fn test_decode_option_vec_u8() {
    let original: Option<Vec<u8>> = Some(vec![1u8, 2, 3]);
    let encoded = encode_to_vec(&original).expect("encode Option<Vec<u8>> failed");
    #[allow(clippy::type_complexity)]
    let (decoded, consumed): (Option<Vec<u8>>, _) =
        decode_from_slice(&encoded).expect("decode Option<Vec<u8>> failed");
    assert_eq!(decoded, Some(vec![1u8, 2, 3]));
    assert_eq!(consumed, encoded.len());

    let original_none: Option<Vec<u8>> = None;
    let encoded_none = encode_to_vec(&original_none).expect("encode None Vec<u8> failed");
    let (decoded_none, _): (Option<Vec<u8>>, _) =
        decode_from_slice(&encoded_none).expect("decode None Vec<u8> failed");
    assert_eq!(decoded_none, None);
}

// ===== Test 15: &str borrow_decode — string with null bytes embedded =====

#[test]
fn test_borrow_decode_str_with_null_bytes() {
    // Null bytes are valid in Rust strings (and valid UTF-8)
    let original = String::from("hello\0world\0end");
    let encoded = encode_to_vec(&original).expect("encode null-byte String failed");
    let (decoded, consumed): (&str, _) =
        borrow_decode_from_slice(&encoded).expect("borrow_decode null-byte &str failed");
    assert_eq!(decoded, "hello\0world\0end");
    assert_eq!(decoded.len(), 15);
    assert_eq!(consumed, encoded.len());
}

// ===== Test 16: &[u8] borrow_decode byte size check (same as Vec<u8> encoded length) =====

#[test]
fn test_borrow_decode_u8_slice_encoded_length_matches() {
    let original: Vec<u8> = vec![1u8, 2, 3, 4, 5, 6, 7, 8];
    let encoded_from_vec = encode_to_vec(&original).expect("encode Vec<u8> failed");
    let encoded_from_slice = encode_to_vec(&original.as_slice()).expect("encode &[u8] failed");

    // Both Vec<u8> and &[u8] encode to the same wire format
    assert_eq!(encoded_from_vec, encoded_from_slice);

    let (decoded, consumed): (&[u8], _) =
        borrow_decode_from_slice(&encoded_from_vec).expect("borrow_decode &[u8] size check failed");
    assert_eq!(decoded.len(), 8);
    assert_eq!(consumed, encoded_from_vec.len());
}

// ===== Test 17: borrow_decode of &[i8] slice (reinterpreted raw bytes) =====

#[test]
fn test_borrow_decode_i8_slice() {
    // Encode as Vec<u8> (same wire format); decode as &[i8] via zero-copy reinterpretation
    let original: Vec<u8> = vec![0u8, 127, 128, 255];
    let encoded = encode_to_vec(&original).expect("encode Vec<u8> for i8 test failed");
    let (decoded, consumed): (&[i8], _) =
        borrow_decode_from_slice(&encoded).expect("borrow_decode &[i8] failed");
    // Bit-pattern reinterpretation: 128u8 → -128i8, 255u8 → -1i8
    assert_eq!(decoded[0], 0i8);
    assert_eq!(decoded[1], 127i8);
    assert_eq!(decoded[2], -128i8);
    assert_eq!(decoded[3], -1i8);
    assert_eq!(consumed, encoded.len());
}

// ===== Test 18: Struct with Vec<u8> field decoded normally (owned) =====

#[test]
fn test_decode_struct_with_vec_u8_field() {
    use oxicode::{Decode, Encode};

    #[derive(Debug, PartialEq, Encode, Decode)]
    struct Payload {
        id: u32,
        data: Vec<u8>,
    }

    let original = Payload {
        id: 42,
        data: vec![0xCA, 0xFE, 0xBA, 0xBE],
    };
    let encoded = encode_to_vec(&original).expect("encode Payload failed");
    let (decoded, consumed): (Payload, _) =
        decode_from_slice(&encoded).expect("decode Payload failed");
    assert_eq!(decoded, original);
    assert_eq!(consumed, encoded.len());
}

// ===== Test 19: Multiple borrow_decodes from the same buffer =====

#[test]
fn test_multiple_borrow_decodes_from_same_buffer() {
    // Encode two values into a combined buffer and borrow-decode both
    let str_val = String::from("alpha");
    let bytes_val: Vec<u8> = vec![1u8, 2, 3];

    let mut combined = encode_to_vec(&str_val).expect("encode str part failed");
    let bytes_part = encode_to_vec(&bytes_val).expect("encode bytes part failed");
    combined.extend_from_slice(&bytes_part);

    // Borrow-decode &str from the first portion
    let (decoded_str, str_consumed): (&str, _) =
        borrow_decode_from_slice(&combined).expect("borrow_decode &str from combined failed");
    assert_eq!(decoded_str, "alpha");

    // Borrow-decode &[u8] from the remaining portion
    let (decoded_bytes, bytes_consumed): (&[u8], _) =
        borrow_decode_from_slice(&combined[str_consumed..])
            .expect("borrow_decode &[u8] from combined tail failed");
    assert_eq!(decoded_bytes, &[1u8, 2, 3]);

    // Verify that str_consumed + bytes_consumed equals total combined length
    assert_eq!(str_consumed + bytes_consumed, combined.len());

    // Both decoded values must remain usable while combined is alive
    let _ = &combined;
    assert_eq!(decoded_str.len(), 5);
    assert_eq!(decoded_bytes.len(), 3);
}

// ===== Test 20: borrow_decode of large &[u8] (1000 bytes) =====

#[test]
fn test_borrow_decode_u8_slice_large() {
    let original: Vec<u8> = (0u8..=255u8).cycle().take(1000).collect();
    let encoded = encode_to_vec(&original).expect("encode large Vec<u8> failed");
    let (decoded, consumed): (&[u8], _) =
        borrow_decode_from_slice(&encoded).expect("borrow_decode large &[u8] failed");
    assert_eq!(decoded.len(), 1000);
    assert_eq!(&decoded[..5], &[0u8, 1, 2, 3, 4]);
    assert_eq!(&decoded[255..260], &[255u8, 0, 1, 2, 3]);
    assert_eq!(consumed, encoded.len());
}

// ===== Test 21: Cow<'_, str> borrow vs owned — same content, different ownership =====

#[test]
fn test_cow_str_borrow_vs_owned_same_content() {
    let content = String::from("the quick brown fox");
    let encoded = encode_to_vec(&content).expect("encode String for cow comparison failed");

    // borrow_decode_from_slice gives Cow::Borrowed
    let (borrow_decoded, _): (Cow<str>, _) =
        borrow_decode_from_slice(&encoded).expect("borrow_decode Cow<str> failed");
    assert_eq!(borrow_decoded.as_ref(), "the quick brown fox");
    assert!(
        matches!(borrow_decoded, Cow::Borrowed(_)),
        "borrow_decode_from_slice should yield Cow::Borrowed"
    );

    // decode_from_slice gives Cow::Owned
    let (owned_decoded, _): (Cow<str>, _) =
        decode_from_slice(&encoded).expect("decode_from_slice Cow<str> failed");
    assert_eq!(owned_decoded.as_ref(), "the quick brown fox");
    assert!(
        matches!(owned_decoded, Cow::Owned(_)),
        "decode_from_slice should yield Cow::Owned"
    );

    // Both have the same string content
    assert_eq!(borrow_decoded.as_ref(), owned_decoded.as_ref());
}

// ===== Test 22: &str borrow_decode — ASCII-only string =====

#[test]
fn test_borrow_decode_str_ascii_only() {
    let original = String::from("Hello, World! 0123456789 ABCDEFGHIJKLMNOPQRSTUVWXYZ");
    let encoded = encode_to_vec(&original).expect("encode ASCII String failed");
    let (decoded, consumed): (&str, _) =
        borrow_decode_from_slice(&encoded).expect("borrow_decode ASCII &str failed");
    assert_eq!(
        decoded,
        "Hello, World! 0123456789 ABCDEFGHIJKLMNOPQRSTUVWXYZ"
    );
    assert!(decoded.is_ascii(), "decoded string should be purely ASCII");
    assert_eq!(consumed, encoded.len());

    // For pure ASCII: byte length == char count
    assert_eq!(decoded.len(), decoded.chars().count());
}
