//! Roundtrip tests for Cow<str> and Cow<[u8]> encode/decode/borrow_decode

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

// ===== Cow<str> tests =====
// Note: decode_from_slice always produces Cow::Owned, so we use Cow<'static, str>
// as the return type annotation (the 'static lifetime is satisfied by Owned).

#[test]
fn test_cow_str_owned_roundtrip() {
    let original: Cow<'_, str> = Cow::Owned("hello oxicode".to_string());
    let encoded = encode_to_vec(&original).expect("encode failed");
    let (decoded, consumed): (Cow<'static, str>, usize) =
        decode_from_slice(&encoded).expect("decode failed");
    assert_eq!(original.as_ref(), decoded.as_ref());
    assert_eq!(consumed, encoded.len());
}

#[test]
fn test_cow_str_empty_roundtrip() {
    let original: Cow<'_, str> = Cow::Owned(String::new());
    let encoded = encode_to_vec(&original).expect("encode failed");
    let (decoded, consumed): (Cow<'static, str>, usize) =
        decode_from_slice(&encoded).expect("decode failed");
    assert_eq!(original.as_ref(), decoded.as_ref());
    assert_eq!(consumed, encoded.len());
}

#[test]
fn test_cow_str_unicode_roundtrip() {
    let original: Cow<'_, str> = Cow::Owned("日本語テスト🦀".to_string());
    let encoded = encode_to_vec(&original).expect("encode failed");
    let (decoded, consumed): (Cow<'static, str>, usize) =
        decode_from_slice(&encoded).expect("decode failed");
    assert_eq!(original.as_ref(), decoded.as_ref());
    assert_eq!(consumed, encoded.len());
}

#[test]
fn test_cow_str_borrow_decode_yields_borrowed() {
    let original: Cow<'_, str> = Cow::Borrowed("zero copy str");
    let encoded = encode_to_vec(&original).expect("encode failed");
    // borrow_decode_from_slice should return Cow::Borrowed pointing into `encoded`
    let (decoded, consumed): (Cow<'_, str>, usize) =
        borrow_decode_from_slice(&encoded).expect("borrow_decode failed");
    assert_eq!(original.as_ref(), decoded.as_ref());
    assert_eq!(consumed, encoded.len());
    // The decoded Cow must be Borrowed (zero-copy)
    assert!(matches!(decoded, Cow::Borrowed(_)));
}

#[test]
fn test_cow_str_long_string_roundtrip() {
    let s = "a".repeat(1024);
    let original: Cow<'_, str> = Cow::Owned(s);
    let encoded = encode_to_vec(&original).expect("encode failed");
    let (decoded, consumed): (Cow<'static, str>, usize) =
        decode_from_slice(&encoded).expect("decode failed");
    assert_eq!(original.as_ref(), decoded.as_ref());
    assert_eq!(consumed, encoded.len());
}

// ===== Cow<[u8]> tests =====
// Same pattern: decode_from_slice returns Cow<'static, [u8]> (always Owned).

#[test]
fn test_cow_bytes_owned_roundtrip() {
    let original: Cow<'_, [u8]> = Cow::Owned(vec![1u8, 2, 3, 4, 5]);
    let encoded = encode_to_vec(&original).expect("encode failed");
    let (decoded, consumed): (Cow<'static, [u8]>, usize) =
        decode_from_slice(&encoded).expect("decode failed");
    assert_eq!(original.as_ref(), decoded.as_ref());
    assert_eq!(consumed, encoded.len());
}

#[test]
fn test_cow_bytes_empty_roundtrip() {
    let original: Cow<'_, [u8]> = Cow::Owned(Vec::new());
    let encoded = encode_to_vec(&original).expect("encode failed");
    let (decoded, consumed): (Cow<'static, [u8]>, usize) =
        decode_from_slice(&encoded).expect("decode failed");
    assert_eq!(original.as_ref(), decoded.as_ref());
    assert_eq!(consumed, encoded.len());
}

#[test]
fn test_cow_bytes_large_roundtrip() {
    let data: Vec<u8> = (0u8..=255).cycle().take(1000).collect();
    let original: Cow<'_, [u8]> = Cow::Owned(data);
    let encoded = encode_to_vec(&original).expect("encode failed");
    let (decoded, consumed): (Cow<'static, [u8]>, usize) =
        decode_from_slice(&encoded).expect("decode failed");
    assert_eq!(original.as_ref(), decoded.as_ref());
    assert_eq!(consumed, encoded.len());
}

#[test]
fn test_cow_bytes_borrow_decode_yields_borrowed() {
    let original: Cow<'_, [u8]> = Cow::Borrowed(&[10u8, 20, 30, 40]);
    let encoded = encode_to_vec(&original).expect("encode failed");
    // borrow_decode_from_slice should return Cow::Borrowed pointing into `encoded`
    let (decoded, consumed): (Cow<'_, [u8]>, usize) =
        borrow_decode_from_slice(&encoded).expect("borrow_decode failed");
    assert_eq!(original.as_ref(), decoded.as_ref());
    assert_eq!(consumed, encoded.len());
    // The decoded Cow must be Borrowed (zero-copy)
    assert!(matches!(decoded, Cow::Borrowed(_)));
}

#[test]
fn test_cow_bytes_all_byte_values_roundtrip() {
    let data: Vec<u8> = (0u8..=255).collect();
    let original: Cow<'_, [u8]> = Cow::Owned(data);
    let encoded = encode_to_vec(&original).expect("encode failed");
    let (decoded, _consumed): (Cow<'static, [u8]>, usize) =
        decode_from_slice(&encoded).expect("decode failed");
    assert_eq!(original.as_ref(), decoded.as_ref());
}

// ===== Cow encode/decode consistency with owned String and Vec<u8> =====

#[test]
fn test_cow_str_and_string_same_encoding() {
    let s = "consistency check";
    let cow: Cow<'_, str> = Cow::Borrowed(s);
    let string = s.to_string();
    let cow_enc = encode_to_vec(&cow).expect("encode cow");
    let str_enc = encode_to_vec(&string).expect("encode string");
    // Cow<str> and String must produce identical wire format
    assert_eq!(cow_enc, str_enc);
}

#[test]
fn test_cow_bytes_and_vec_same_encoding() {
    let data = vec![0xDEu8, 0xAD, 0xBE, 0xEF];
    let cow: Cow<'_, [u8]> = Cow::Borrowed(&data);
    let vec_enc = encode_to_vec(&data).expect("encode vec");
    let cow_enc = encode_to_vec(&cow).expect("encode cow");
    // Cow<[u8]> and Vec<u8> must produce identical wire format
    assert_eq!(cow_enc, vec_enc);
}
