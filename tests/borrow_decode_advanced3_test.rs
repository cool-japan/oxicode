//! Advanced BorrowDecode tests (set 3) — 22 novel test cases.
//!
//! Covers: zero-copy &str / &[u8], Cow variants, struct with borrowed fields,
//! nested structs, wire-format equivalence, config variations, edge cases,
//! Option<&str/&[u8]>, tuples, and consumed-bytes verification.

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
#[allow(unused_imports)]
use oxicode::{decode_from_slice, encode_to_vec, BorrowDecode, Decode, Encode};
use std::borrow::Cow;

// ─── Type definitions ─────────────────────────────────────────────────────────

#[derive(Debug, PartialEq, Encode, BorrowDecode)]
struct StrStruct<'a> {
    label: &'a str,
}

#[derive(Debug, PartialEq, Encode, BorrowDecode)]
struct BytesStruct<'a> {
    payload: &'a [u8],
}

#[derive(Debug, PartialEq, Encode, BorrowDecode)]
struct NestedInner<'a> {
    tag: &'a str,
    data: &'a [u8],
}

#[derive(Debug, PartialEq, Encode, BorrowDecode)]
struct NestedOuter<'a> {
    inner: NestedInner<'a>,
    version: u32,
}

// ─── 1. &str borrow decode from slice ─────────────────────────────────────────

#[test]
fn test_borrow_decode_str_from_slice() {
    let original = "hello borrow decode";
    let encoded = encode_to_vec(&original).expect("encode &str");
    let (decoded, consumed): (&str, _) =
        oxicode::borrow_decode_from_slice(&encoded).expect("borrow_decode &str");
    assert_eq!(decoded, original);
    assert_eq!(consumed, encoded.len());
}

// ─── 2. &[u8] borrow decode from slice ────────────────────────────────────────

#[test]
fn test_borrow_decode_bytes_from_slice() {
    let original: &[u8] = &[10u8, 20, 30, 40, 50, 60];
    let encoded = encode_to_vec(&original).expect("encode &[u8]");
    let (decoded, consumed): (&[u8], _) =
        oxicode::borrow_decode_from_slice(&encoded).expect("borrow_decode &[u8]");
    assert_eq!(decoded, original);
    assert_eq!(consumed, encoded.len());
}

// ─── 3. Borrowed &str contains same content as original string ────────────────

#[test]
fn test_borrow_decode_str_content_matches() {
    let original = String::from("content equality check");
    let encoded = encode_to_vec(&original).expect("encode String for borrow check");
    let (decoded, _): (&str, _) =
        oxicode::borrow_decode_from_slice(&encoded).expect("borrow_decode &str content check");
    assert_eq!(
        decoded,
        original.as_str(),
        "decoded content must match original"
    );
    assert_eq!(decoded.len(), original.len());
}

// ─── 4. Cow<str> borrow decode produces Borrowed variant ──────────────────────

#[test]
fn test_cow_str_borrow_decode_produces_borrowed_variant() {
    let original = "cow borrowed string";
    let encoded = encode_to_vec(&original).expect("encode for Cow<str>");
    let (decoded, consumed): (Cow<str>, _) =
        oxicode::borrow_decode_from_slice(&encoded).expect("borrow_decode Cow<str>");
    assert!(
        matches!(decoded, Cow::Borrowed(_)),
        "Cow<str> must be Borrowed variant after borrow_decode_from_slice"
    );
    assert_eq!(decoded.as_ref(), original);
    assert_eq!(consumed, encoded.len());
}

// ─── 5. Cow<[u8]> borrow decode produces Borrowed variant ────────────────────

#[test]
fn test_cow_bytes_borrow_decode_produces_borrowed_variant() {
    let original: &[u8] = &[0xAA, 0xBB, 0xCC, 0xDD];
    let encoded = encode_to_vec(&original).expect("encode for Cow<[u8]>");
    let (decoded, consumed): (Cow<[u8]>, _) =
        oxicode::borrow_decode_from_slice(&encoded).expect("borrow_decode Cow<[u8]>");
    assert!(
        matches!(decoded, Cow::Borrowed(_)),
        "Cow<[u8]> must be Borrowed variant after borrow_decode_from_slice"
    );
    assert_eq!(decoded.as_ref(), original);
    assert_eq!(consumed, encoded.len());
}

// ─── 6. Multiple &str borrowed from same buffer ───────────────────────────────

#[test]
fn test_multiple_strs_borrowed_from_same_buffer() {
    // Encode a tuple of two strings — both should borrow from the same encoded buffer
    let pair: (&str, &str) = ("first part", "second part");
    let encoded = encode_to_vec(&pair).expect("encode (&str, &str) tuple");
    let (decoded, consumed): ((&str, &str), _) =
        oxicode::borrow_decode_from_slice(&encoded).expect("borrow_decode (&str, &str)");
    assert_eq!(decoded.0, "first part");
    assert_eq!(decoded.1, "second part");
    assert_eq!(consumed, encoded.len());

    // Both decoded string pointers must lie within the encoded buffer
    let buf_start = encoded.as_ptr() as usize;
    let buf_end = buf_start + encoded.len();
    let ptr0 = decoded.0.as_ptr() as usize;
    let ptr1 = decoded.1.as_ptr() as usize;
    assert!(
        ptr0 >= buf_start && ptr0 < buf_end,
        "first &str does not point into the encoded buffer"
    );
    assert!(
        ptr1 >= buf_start && ptr1 < buf_end,
        "second &str does not point into the encoded buffer"
    );
}

// ─── 7. Struct with &'a str field — borrow decode ─────────────────────────────

#[test]
fn test_struct_with_str_field_borrow_decode() {
    let original = StrStruct {
        label: "struct str field",
    };
    let encoded = encode_to_vec(&original).expect("encode StrStruct");
    let (decoded, consumed): (StrStruct, _) =
        oxicode::borrow_decode_from_slice(&encoded).expect("borrow_decode StrStruct");
    assert_eq!(decoded.label, "struct str field");
    assert_eq!(consumed, encoded.len());
}

// ─── 8. Struct with &'a [u8] field — borrow decode ────────────────────────────

#[test]
fn test_struct_with_bytes_field_borrow_decode() {
    let data_bytes: &[u8] = &[0x01, 0x02, 0x03, 0x04, 0x05];
    let original = BytesStruct {
        payload: data_bytes,
    };
    let encoded = encode_to_vec(&original).expect("encode BytesStruct");
    let (decoded, consumed): (BytesStruct, _) =
        oxicode::borrow_decode_from_slice(&encoded).expect("borrow_decode BytesStruct");
    assert_eq!(decoded.payload, data_bytes);
    assert_eq!(consumed, encoded.len());
}

// ─── 9. Nested struct with borrowed fields ────────────────────────────────────

#[test]
fn test_nested_struct_with_borrowed_fields() {
    let inner_bytes: &[u8] = &[0xDE, 0xAD, 0xBE, 0xEF];
    let original = NestedOuter {
        inner: NestedInner {
            tag: "nested_tag",
            data: inner_bytes,
        },
        version: 7,
    };
    let encoded = encode_to_vec(&original).expect("encode NestedOuter");
    let (decoded, consumed): (NestedOuter, _) =
        oxicode::borrow_decode_from_slice(&encoded).expect("borrow_decode NestedOuter");
    assert_eq!(decoded.inner.tag, "nested_tag");
    assert_eq!(decoded.inner.data, inner_bytes);
    assert_eq!(decoded.version, 7);
    assert_eq!(consumed, encoded.len());

    // Verify zero-copy: both inner pointers must lie within the encoded buffer
    let buf_start = encoded.as_ptr() as usize;
    let buf_end = buf_start + encoded.len();
    let tag_ptr = decoded.inner.tag.as_ptr() as usize;
    let data_ptr = decoded.inner.data.as_ptr() as usize;
    assert!(
        tag_ptr >= buf_start && tag_ptr < buf_end,
        "nested tag does not point into encoded buffer"
    );
    assert!(
        data_ptr >= buf_start && data_ptr < buf_end,
        "nested data does not point into encoded buffer"
    );
}

// ─── 10. &str vs String: same wire format, different lifetime semantics ────────

#[test]
fn test_str_and_string_have_same_wire_format() {
    let text = "same wire format";
    let encoded_str = encode_to_vec(&text).expect("encode &str");
    let owned = String::from(text);
    let encoded_string = encode_to_vec(&owned).expect("encode String");
    // Wire bytes must be identical
    assert_eq!(
        encoded_str, encoded_string,
        "&str and String must produce identical wire bytes"
    );

    // Decode the &str encoding as String (owned) and verify
    let (from_str_enc, _): (String, _) =
        decode_from_slice(&encoded_str).expect("decode String from &str encoding");
    assert_eq!(from_str_enc, text);
}

// ─── 11. &[u8] vs Vec<u8>: same wire format ───────────────────────────────────

#[test]
fn test_bytes_slice_and_vec_have_same_wire_format() {
    let bytes: &[u8] = &[1u8, 2, 3, 4, 5];
    let encoded_slice = encode_to_vec(&bytes).expect("encode &[u8]");
    let owned_vec: Vec<u8> = bytes.to_vec();
    let encoded_vec = encode_to_vec(&owned_vec).expect("encode Vec<u8>");
    assert_eq!(
        encoded_slice, encoded_vec,
        "&[u8] and Vec<u8> must produce identical wire bytes"
    );

    // Decode the slice encoding as Vec<u8>
    let (from_slice_enc, _): (Vec<u8>, _) =
        decode_from_slice(&encoded_slice).expect("decode Vec<u8> from &[u8] encoding");
    assert_eq!(from_slice_enc.as_slice(), bytes);
}

// ─── 12. BorrowDecode with standard config ────────────────────────────────────

#[test]
fn test_borrow_decode_with_standard_config() {
    let cfg = oxicode::config::standard();
    let original = "standard config borrow";
    let encoded =
        oxicode::encode_to_vec_with_config(&original, cfg).expect("encode with standard config");
    let (decoded, consumed): (&str, _) =
        oxicode::borrow_decode_from_slice_with_config(&encoded, cfg)
            .expect("borrow_decode with standard config");
    assert_eq!(decoded, original);
    assert_eq!(consumed, encoded.len());
}

// ─── 13. BorrowDecode with fixed-int config ───────────────────────────────────

#[test]
fn test_borrow_decode_with_fixed_int_config() {
    let cfg = oxicode::config::standard().with_fixed_int_encoding();
    let original = "fixed int config borrow";
    let encoded =
        oxicode::encode_to_vec_with_config(&original, cfg).expect("encode with fixed_int config");
    let (decoded, consumed): (&str, _) =
        oxicode::borrow_decode_from_slice_with_config(&encoded, cfg)
            .expect("borrow_decode with fixed_int config");
    assert_eq!(decoded, original);
    assert_eq!(consumed, encoded.len());
}

// ─── 14. Empty &str borrow decode ─────────────────────────────────────────────

#[test]
fn test_borrow_decode_empty_str() {
    let original = "";
    let encoded = encode_to_vec(&original).expect("encode empty &str");
    let (decoded, consumed): (&str, _) =
        oxicode::borrow_decode_from_slice(&encoded).expect("borrow_decode empty &str");
    assert_eq!(decoded, "");
    assert_eq!(decoded.len(), 0);
    assert_eq!(consumed, encoded.len());
}

// ─── 15. Empty &[u8] borrow decode ────────────────────────────────────────────

#[test]
fn test_borrow_decode_empty_bytes() {
    let original: &[u8] = &[];
    let encoded = encode_to_vec(&original).expect("encode empty &[u8]");
    let (decoded, consumed): (&[u8], _) =
        oxicode::borrow_decode_from_slice(&encoded).expect("borrow_decode empty &[u8]");
    assert_eq!(decoded, original);
    assert_eq!(decoded.len(), 0);
    assert_eq!(consumed, encoded.len());
}

// ─── 16. Long string borrow decode (100 chars) ────────────────────────────────

#[test]
fn test_borrow_decode_long_string_100_chars() {
    let original: String = "abcdefghij".repeat(10); // 100 chars
    assert_eq!(original.len(), 100);
    let encoded = encode_to_vec(&original).expect("encode 100-char string");
    let (decoded, consumed): (&str, _) =
        oxicode::borrow_decode_from_slice(&encoded).expect("borrow_decode 100-char string");
    assert_eq!(decoded, original.as_str());
    assert_eq!(decoded.len(), 100);
    assert_eq!(consumed, encoded.len());
}

// ─── 17. Unicode string borrow decode ─────────────────────────────────────────

#[test]
fn test_borrow_decode_unicode_string() {
    let original = "日本語テスト 🦀🌍 Ñoño Ça va?";
    let encoded = encode_to_vec(&original).expect("encode unicode string");
    let (decoded, consumed): (&str, _) =
        oxicode::borrow_decode_from_slice(&encoded).expect("borrow_decode unicode string");
    assert_eq!(decoded, original);
    assert_eq!(consumed, encoded.len());
    // Pointer must lie within the encoded buffer
    let buf_start = encoded.as_ptr() as usize;
    let buf_end = buf_start + encoded.len();
    let str_ptr = decoded.as_ptr() as usize;
    assert!(
        str_ptr >= buf_start && str_ptr < buf_end,
        "unicode &str does not point into encoded buffer"
    );
}

// ─── 18. Option<&str> borrow decode Some ──────────────────────────────────────

#[test]
fn test_option_str_borrow_decode_some() {
    let original: Option<&str> = Some("option some value");
    let encoded = encode_to_vec(&original).expect("encode Option<&str> Some");
    let (decoded, consumed): (Option<&str>, _) =
        oxicode::borrow_decode_from_slice(&encoded).expect("borrow_decode Option<&str> Some");
    assert_eq!(decoded, Some("option some value"));
    assert_eq!(consumed, encoded.len());
}

// ─── 19. Option<&[u8]> borrow decode None ─────────────────────────────────────

#[test]
fn test_option_bytes_borrow_decode_none() {
    let original: Option<&[u8]> = None;
    let encoded = encode_to_vec(&original).expect("encode Option<&[u8]> None");
    let (decoded, consumed): (Option<&[u8]>, _) =
        oxicode::borrow_decode_from_slice(&encoded).expect("borrow_decode Option<&[u8]> None");
    assert_eq!(decoded, None);
    assert_eq!(consumed, encoded.len());
}

// ─── 20. Tuple (&str, &[u8]) borrow decode ────────────────────────────────────

#[test]
fn test_tuple_str_bytes_borrow_decode() {
    let str_part = "tuple str part";
    let bytes_part: &[u8] = &[0x10, 0x20, 0x30, 0x40];
    let original: (&str, &[u8]) = (str_part, bytes_part);
    let encoded = encode_to_vec(&original).expect("encode (&str, &[u8]) tuple");
    let (decoded, consumed): ((&str, &[u8]), _) =
        oxicode::borrow_decode_from_slice(&encoded).expect("borrow_decode (&str, &[u8]) tuple");
    assert_eq!(decoded.0, str_part);
    assert_eq!(decoded.1, bytes_part);
    assert_eq!(consumed, encoded.len());
}

// ─── 21. Vec<u8> of borrowed bytes vs owned — same decoded content ─────────────

#[test]
fn test_vec_u8_borrowed_vs_owned_same_content() {
    let data: &[u8] = &[5u8, 10, 15, 20, 25, 30];
    // Encode via borrowed slice
    let encoded_borrow = encode_to_vec(&data).expect("encode &[u8]");
    // Decode as owned Vec<u8>
    let (owned_decoded, owned_consumed): (Vec<u8>, _) =
        decode_from_slice(&encoded_borrow).expect("decode Vec<u8> from &[u8] encoding");
    // Decode as borrowed &[u8]
    let (borrow_decoded, borrow_consumed): (&[u8], _) =
        oxicode::borrow_decode_from_slice(&encoded_borrow).expect("borrow_decode &[u8]");
    // Both must decode to the same byte sequence
    assert_eq!(owned_decoded.as_slice(), data);
    assert_eq!(borrow_decoded, data);
    assert_eq!(
        owned_consumed, borrow_consumed,
        "consumed bytes must be the same"
    );
}

// ─── 22. BorrowDecode consumed bytes == encoded length ────────────────────────

#[test]
fn test_borrow_decode_consumed_equals_encoded_length() {
    // Test several types to confirm consumed always equals the full encoded length
    let str_val = "consumed bytes test";
    let enc_str = encode_to_vec(&str_val).expect("encode &str for consumed check");
    let (_, str_consumed): (&str, _) =
        oxicode::borrow_decode_from_slice(&enc_str).expect("borrow_decode &str consumed check");
    assert_eq!(
        str_consumed,
        enc_str.len(),
        "&str consumed must equal encoded length"
    );

    let bytes_val: &[u8] = &[0xAA, 0xBB, 0xCC];
    let enc_bytes = encode_to_vec(&bytes_val).expect("encode &[u8] for consumed check");
    let (_, bytes_consumed): (&[u8], _) =
        oxicode::borrow_decode_from_slice(&enc_bytes).expect("borrow_decode &[u8] consumed check");
    assert_eq!(
        bytes_consumed,
        enc_bytes.len(),
        "&[u8] consumed must equal encoded length"
    );

    let cow_val: Cow<str> = Cow::Borrowed("cow consumed check");
    let enc_cow = encode_to_vec(&cow_val).expect("encode Cow<str> for consumed check");
    let (_, cow_consumed): (Cow<str>, _) =
        oxicode::borrow_decode_from_slice(&enc_cow).expect("borrow_decode Cow<str> consumed check");
    assert_eq!(
        cow_consumed,
        enc_cow.len(),
        "Cow<str> consumed must equal encoded length"
    );
}
