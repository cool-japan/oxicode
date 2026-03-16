//! Advanced roundtrip tests for Cow<str> and Cow<[u8]> encoding/decoding.
//!
//! Note: decode_from_slice always returns Cow::Owned variants because allocation
//! is required. For Borrowed tests we verify that encoding Borrowed produces
//! identical bytes to encoding Owned.

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
use std::borrow::Cow;
use std::collections::BTreeMap;
use std::f64::consts::{E, PI};

use oxicode::{decode_from_slice, encode_to_vec, Decode, Encode};

mod cow_advanced_tests {
    use super::*;

    // ── Test 1: Cow::Borrowed(&str) roundtrip ────────────────────────────────

    #[test]
    fn test_cow_str_borrowed_roundtrip() {
        let original: Cow<'_, str> = Cow::Borrowed("borrowed str slice");
        let encoded = encode_to_vec(&original).expect("encode Cow::Borrowed(&str) failed");
        let (decoded, consumed): (Cow<'static, str>, usize) =
            decode_from_slice(&encoded).expect("decode Cow::Borrowed(&str) failed");
        assert_eq!(original.as_ref(), decoded.as_ref());
        assert_eq!(consumed, encoded.len());
        // Decoded is always Owned after a heap-allocating decode
        assert!(matches!(decoded, Cow::Owned(_)));
    }

    // ── Test 2: Cow::Owned(String) roundtrip ────────────────────────────────

    #[test]
    fn test_cow_str_owned_string_roundtrip() {
        let original: Cow<'_, str> = Cow::Owned("owned String value".to_string());
        let encoded = encode_to_vec(&original).expect("encode Cow::Owned(String) failed");
        let (decoded, consumed): (Cow<'static, str>, usize) =
            decode_from_slice(&encoded).expect("decode Cow::Owned(String) failed");
        assert_eq!(original.as_ref(), decoded.as_ref());
        assert_eq!(consumed, encoded.len());
        assert!(matches!(decoded, Cow::Owned(_)));
    }

    // ── Test 3: Cow<str> borrowed and owned produce same bytes ───────────────

    #[test]
    fn test_cow_str_borrowed_and_owned_produce_same_bytes() {
        let content = "identical wire format";
        let borrowed: Cow<'_, str> = Cow::Borrowed(content);
        let owned: Cow<'_, str> = Cow::Owned(content.to_string());
        let enc_borrowed = encode_to_vec(&borrowed).expect("encode borrowed failed");
        let enc_owned = encode_to_vec(&owned).expect("encode owned failed");
        assert_eq!(
            enc_borrowed, enc_owned,
            "Borrowed and Owned Cow<str> must produce identical encoding"
        );
    }

    // ── Test 4: Cow<[u8]> borrowed roundtrip ────────────────────────────────

    #[test]
    fn test_cow_bytes_borrowed_roundtrip() {
        let data: &[u8] = &[10u8, 20, 30, 40, 50];
        let original: Cow<'_, [u8]> = Cow::Borrowed(data);
        let encoded = encode_to_vec(&original).expect("encode Cow::Borrowed(&[u8]) failed");
        let (decoded, consumed): (Cow<'static, [u8]>, usize) =
            decode_from_slice(&encoded).expect("decode Cow::Borrowed(&[u8]) failed");
        assert_eq!(original.as_ref(), decoded.as_ref());
        assert_eq!(consumed, encoded.len());
        assert!(matches!(decoded, Cow::Owned(_)));
    }

    // ── Test 5: Cow<[u8]> owned roundtrip ────────────────────────────────────

    #[test]
    fn test_cow_bytes_owned_roundtrip() {
        let original: Cow<'_, [u8]> = Cow::Owned(vec![0xCAu8, 0xFE, 0xBA, 0xBE]);
        let encoded = encode_to_vec(&original).expect("encode Cow::Owned(Vec<u8>) failed");
        let (decoded, consumed): (Cow<'static, [u8]>, usize) =
            decode_from_slice(&encoded).expect("decode Cow::Owned(Vec<u8>) failed");
        assert_eq!(original.as_ref(), decoded.as_ref());
        assert_eq!(consumed, encoded.len());
        assert!(matches!(decoded, Cow::Owned(_)));
    }

    // ── Test 6: Cow<str> with empty string ───────────────────────────────────

    #[test]
    fn test_cow_str_empty_string_roundtrip() {
        let original: Cow<'_, str> = Cow::Owned(String::new());
        let encoded = encode_to_vec(&original).expect("encode empty Cow<str> failed");
        let (decoded, consumed): (Cow<'static, str>, usize) =
            decode_from_slice(&encoded).expect("decode empty Cow<str> failed");
        assert_eq!("", decoded.as_ref());
        assert_eq!(consumed, encoded.len());
    }

    // ── Test 7: Cow<[u8]> with empty slice ───────────────────────────────────

    #[test]
    fn test_cow_bytes_empty_slice_roundtrip() {
        let original: Cow<'_, [u8]> = Cow::Owned(Vec::new());
        let encoded = encode_to_vec(&original).expect("encode empty Cow<[u8]> failed");
        let (decoded, consumed): (Cow<'static, [u8]>, usize) =
            decode_from_slice(&encoded).expect("decode empty Cow<[u8]> failed");
        assert_eq!(0usize, decoded.len());
        assert_eq!(consumed, encoded.len());
    }

    // ── Test 8: Cow<str> with unicode content ────────────────────────────────

    #[test]
    fn test_cow_str_unicode_content_roundtrip() {
        let unicode = "こんにちは世界🌏αβγδεζηθ";
        let original: Cow<'_, str> = Cow::Owned(unicode.to_string());
        let encoded = encode_to_vec(&original).expect("encode unicode Cow<str> failed");
        let (decoded, consumed): (Cow<'static, str>, usize) =
            decode_from_slice(&encoded).expect("decode unicode Cow<str> failed");
        assert_eq!(unicode, decoded.as_ref());
        assert_eq!(consumed, encoded.len());
    }

    // ── Test 9: Cow<[u8]> with all byte values 0..=255 ───────────────────────

    #[test]
    fn test_cow_bytes_all_byte_values_roundtrip() {
        let all_bytes: Vec<u8> = (0u8..=255).collect();
        let original: Cow<'_, [u8]> = Cow::Owned(all_bytes.clone());
        let encoded = encode_to_vec(&original).expect("encode all-bytes Cow<[u8]> failed");
        let (decoded, consumed): (Cow<'static, [u8]>, usize) =
            decode_from_slice(&encoded).expect("decode all-bytes Cow<[u8]> failed");
        assert_eq!(all_bytes.as_slice(), decoded.as_ref());
        assert_eq!(consumed, encoded.len());
    }

    // ── Test 10: Vec<Cow<str>> roundtrip (all owned) ─────────────────────────

    #[test]
    fn test_vec_cow_str_all_owned_roundtrip() {
        let original: Vec<Cow<'static, str>> = vec![
            Cow::Owned("alpha".to_string()),
            Cow::Owned("beta".to_string()),
            Cow::Owned("gamma".to_string()),
            Cow::Owned("delta".to_string()),
        ];
        let encoded = encode_to_vec(&original).expect("encode Vec<Cow<str>> failed");
        let (decoded, consumed): (Vec<Cow<'static, str>>, usize) =
            decode_from_slice(&encoded).expect("decode Vec<Cow<str>> failed");
        assert_eq!(original.len(), decoded.len());
        for (a, b) in original.iter().zip(decoded.iter()) {
            assert_eq!(a.as_ref(), b.as_ref());
        }
        assert_eq!(consumed, encoded.len());
    }

    // ── Test 11: Vec<Cow<[u8]>> roundtrip ────────────────────────────────────

    #[test]
    fn test_vec_cow_bytes_roundtrip() {
        let original: Vec<Cow<'static, [u8]>> = vec![
            Cow::Owned(vec![1u8, 2, 3]),
            Cow::Owned(vec![4u8, 5, 6]),
            Cow::Owned(vec![7u8, 8, 9]),
        ];
        let encoded = encode_to_vec(&original).expect("encode Vec<Cow<[u8]>> failed");
        let (decoded, consumed): (Vec<Cow<'static, [u8]>>, usize) =
            decode_from_slice(&encoded).expect("decode Vec<Cow<[u8]>> failed");
        assert_eq!(original.len(), decoded.len());
        for (a, b) in original.iter().zip(decoded.iter()) {
            assert_eq!(a.as_ref(), b.as_ref());
        }
        assert_eq!(consumed, encoded.len());
    }

    // ── Test 12: Option<Cow<str>> Some roundtrip ─────────────────────────────

    #[test]
    fn test_option_cow_str_some_roundtrip() {
        let original: Option<Cow<'static, str>> = Some(Cow::Owned("some value".to_string()));
        let encoded = encode_to_vec(&original).expect("encode Option<Cow<str>> Some failed");
        let (decoded, consumed): (Option<Cow<'static, str>>, usize) =
            decode_from_slice(&encoded).expect("decode Option<Cow<str>> Some failed");
        assert!(decoded.is_some());
        assert_eq!(
            original.as_deref().expect("original is Some"),
            decoded.as_deref().expect("decoded is Some")
        );
        assert_eq!(consumed, encoded.len());
    }

    // ── Test 13: Option<Cow<str>> None roundtrip ─────────────────────────────

    #[test]
    fn test_option_cow_str_none_roundtrip() {
        let original: Option<Cow<'static, str>> = None;
        let encoded = encode_to_vec(&original).expect("encode Option<Cow<str>> None failed");
        let (decoded, consumed): (Option<Cow<'static, str>>, usize) =
            decode_from_slice(&encoded).expect("decode Option<Cow<str>> None failed");
        assert!(decoded.is_none());
        assert_eq!(consumed, encoded.len());
    }

    // ── Test 14: Struct with Cow<str> field derive roundtrip ─────────────────

    #[derive(Debug, PartialEq, Encode, Decode)]
    struct CowStrRecord {
        label: String,
        value: Cow<'static, str>,
        count: u32,
    }

    #[test]
    fn test_struct_with_cow_str_field_derive_roundtrip() {
        let original = CowStrRecord {
            label: "record-label".to_string(),
            value: Cow::Owned("record-value".to_string()),
            count: 42,
        };
        let encoded = encode_to_vec(&original).expect("encode CowStrRecord failed");
        let (decoded, consumed): (CowStrRecord, usize) =
            decode_from_slice(&encoded).expect("decode CowStrRecord failed");
        assert_eq!(original, decoded);
        assert_eq!(consumed, encoded.len());
    }

    // ── Test 15: Struct with Cow<[u8]> field derive roundtrip ────────────────

    #[derive(Debug, PartialEq, Encode, Decode)]
    struct CowBytesRecord {
        id: u64,
        payload: Cow<'static, [u8]>,
    }

    #[test]
    fn test_struct_with_cow_bytes_field_derive_roundtrip() {
        let original = CowBytesRecord {
            id: 9_999_999_999u64,
            payload: Cow::Owned(vec![0xFFu8, 0x00, 0xAB, 0xCD, 0xEF]),
        };
        let encoded = encode_to_vec(&original).expect("encode CowBytesRecord failed");
        let (decoded, consumed): (CowBytesRecord, usize) =
            decode_from_slice(&encoded).expect("decode CowBytesRecord failed");
        assert_eq!(original, decoded);
        assert_eq!(consumed, encoded.len());
    }

    // ── Test 16: Cow<str> produces same bytes as String for same content ──────

    #[test]
    fn test_cow_str_same_bytes_as_string() {
        let content = "wire format parity check";
        let cow: Cow<'_, str> = Cow::Owned(content.to_string());
        let string = content.to_string();
        let enc_cow = encode_to_vec(&cow).expect("encode Cow<str> failed");
        let enc_string = encode_to_vec(&string).expect("encode String failed");
        assert_eq!(
            enc_cow, enc_string,
            "Cow<str> and String must encode identically"
        );
    }

    // ── Test 17: Cow<[u8]> produces same bytes as Vec<u8> for same content ───

    #[test]
    fn test_cow_bytes_same_bytes_as_vec_u8() {
        let data = vec![0xDEu8, 0xAD, 0xBE, 0xEF, 0x01, 0x23, 0x45, 0x67];
        let cow: Cow<'_, [u8]> = Cow::Owned(data.clone());
        let enc_cow = encode_to_vec(&cow).expect("encode Cow<[u8]> failed");
        let enc_vec = encode_to_vec(&data).expect("encode Vec<u8> failed");
        assert_eq!(
            enc_cow, enc_vec,
            "Cow<[u8]> and Vec<u8> must encode identically"
        );
    }

    // ── Test 18: BTreeMap<String, Cow<str>> roundtrip ────────────────────────

    #[test]
    fn test_btreemap_string_cow_str_roundtrip() {
        let mut original: BTreeMap<String, Cow<'static, str>> = BTreeMap::new();
        original.insert("key_a".to_string(), Cow::Owned("value_a".to_string()));
        original.insert("key_b".to_string(), Cow::Owned("value_b".to_string()));
        original.insert("key_c".to_string(), Cow::Owned("value_c".to_string()));

        let encoded = encode_to_vec(&original).expect("encode BTreeMap<String, Cow<str>> failed");
        let (decoded, consumed): (BTreeMap<String, Cow<'static, str>>, usize) =
            decode_from_slice(&encoded).expect("decode BTreeMap<String, Cow<str>> failed");

        assert_eq!(original.len(), decoded.len());
        for (k, v) in &original {
            let dv = decoded.get(k).expect("key missing in decoded map");
            assert_eq!(v.as_ref(), dv.as_ref());
        }
        assert_eq!(consumed, encoded.len());
    }

    // ── Test 19: Long Cow<str> (1000 chars) roundtrip ────────────────────────

    #[test]
    fn test_cow_str_long_1000_chars_roundtrip() {
        let long_str = "x".repeat(1000);
        let original: Cow<'_, str> = Cow::Owned(long_str.clone());
        let encoded = encode_to_vec(&original).expect("encode long Cow<str> failed");
        let (decoded, consumed): (Cow<'static, str>, usize) =
            decode_from_slice(&encoded).expect("decode long Cow<str> failed");
        assert_eq!(long_str.as_str(), decoded.as_ref());
        assert_eq!(1000, decoded.len());
        assert_eq!(consumed, encoded.len());
    }

    // ── Test 20: Cow<str> with PI-derived string content ─────────────────────

    #[test]
    fn test_cow_str_pi_derived_content_roundtrip() {
        // Build a string from PI and E to satisfy the requirement for
        // std::f64::consts::PI and E usage.
        let pi_str = format!("pi={:.15} e={:.15} product={:.15}", PI, E, PI * E);
        let original: Cow<'_, str> = Cow::Owned(pi_str.clone());
        let encoded = encode_to_vec(&original).expect("encode PI-derived Cow<str> failed");
        let (decoded, consumed): (Cow<'static, str>, usize) =
            decode_from_slice(&encoded).expect("decode PI-derived Cow<str> failed");
        assert_eq!(pi_str.as_str(), decoded.as_ref());
        assert_eq!(consumed, encoded.len());
        // Sanity check: the decoded string must contain a recognisable PI prefix
        assert!(
            decoded.as_ref().starts_with("pi=3.14159"),
            "decoded string should start with pi=3.14159, got: {}",
            decoded
        );
    }
}
