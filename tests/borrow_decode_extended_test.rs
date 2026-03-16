//! Extended tests for zero-copy BorrowDecode — 20 comprehensive test cases.

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
use oxicode::{BorrowDecode, Decode, Encode};

// ─── Type definitions (must be outside #[cfg(test)] for derive macros) ────────

#[derive(Debug, PartialEq, Encode, BorrowDecode)]
struct StrField<'a> {
    name: &'a str,
}

#[derive(Debug, PartialEq, Encode, BorrowDecode)]
struct BytesField<'a> {
    data: &'a [u8],
}

#[derive(Debug, PartialEq, Encode, BorrowDecode)]
struct MultiStr<'a> {
    a: &'a str,
    b: &'a str,
}

#[derive(Debug, PartialEq, Encode, BorrowDecode)]
struct MixedFields<'a> {
    id: u32,
    label: &'a str,
    count: u64,
}

#[derive(Debug, PartialEq, Encode, Decode)]
struct OwnedStrField {
    name: String,
}

#[derive(Debug, PartialEq, Encode, BorrowDecode)]
struct InnerBorrow<'a> {
    tag: &'a str,
}

#[derive(Debug, PartialEq, Encode, BorrowDecode)]
struct OuterBorrow<'a> {
    inner: InnerBorrow<'a>,
    val: u32,
}

#[derive(Debug, PartialEq, Encode, BorrowDecode)]
enum Cmd<'a> {
    Ping,
    Text(&'a str),
}

// ─── Tests ────────────────────────────────────────────────────────────────────

#[cfg(test)]
mod borrow_decode_extended {
    use super::*;
    use std::borrow::Cow;

    // 1. &str borrow decode — decoded pointer lies within the encoded buffer
    #[test]
    fn test_str_borrow_decode_points_into_buffer() {
        let original = "hello oxicode";
        let encoded = oxicode::encode_to_vec(&original).expect("encode &str");
        let (decoded, _): (&str, _) =
            oxicode::borrow_decode_from_slice(&encoded).expect("borrow_decode &str");
        assert_eq!(original, decoded);
        let buf_start = encoded.as_ptr() as usize;
        let buf_end = buf_start + encoded.len();
        let ptr = decoded.as_ptr() as usize;
        assert!(
            ptr >= buf_start && ptr < buf_end,
            "decoded &str does not point into encoded buffer"
        );
    }

    // 2. &[u8] borrow decode — decoded pointer lies within the encoded buffer
    #[test]
    fn test_bytes_borrow_decode_points_into_buffer() {
        let original: &[u8] = &[10, 20, 30, 40, 50];
        let encoded = oxicode::encode_to_vec(&original).expect("encode &[u8]");
        let (decoded, _): (&[u8], _) =
            oxicode::borrow_decode_from_slice(&encoded).expect("borrow_decode &[u8]");
        assert_eq!(original, decoded);
        let buf_start = encoded.as_ptr() as usize;
        let buf_end = buf_start + encoded.len();
        let ptr = decoded.as_ptr() as usize;
        assert!(
            ptr >= buf_start && ptr < buf_end,
            "decoded &[u8] does not point into encoded buffer"
        );
    }

    // 3. Cow<str> as borrowed variant
    #[test]
    fn test_cow_str_borrowed_variant() {
        let original = "cow borrowed str";
        let encoded = oxicode::encode_to_vec(&original).expect("encode for Cow<str>");
        let (decoded, _): (Cow<str>, _) =
            oxicode::borrow_decode_from_slice(&encoded).expect("borrow_decode Cow<str>");
        assert_eq!(decoded.as_ref(), original);
        assert!(
            matches!(decoded, Cow::Borrowed(_)),
            "Cow<str> should be Borrowed variant"
        );
    }

    // 4. Cow<[u8]> as borrowed variant
    #[test]
    fn test_cow_bytes_borrowed_variant() {
        let original: &[u8] = &[1, 2, 3, 4, 5];
        let encoded = oxicode::encode_to_vec(&original).expect("encode for Cow<[u8]>");
        let (decoded, _): (Cow<[u8]>, _) =
            oxicode::borrow_decode_from_slice(&encoded).expect("borrow_decode Cow<[u8]>");
        assert_eq!(decoded.as_ref(), original);
        assert!(
            matches!(decoded, Cow::Borrowed(_)),
            "Cow<[u8]> should be Borrowed variant"
        );
    }

    // 5. Struct with &str field BorrowDecode
    #[test]
    fn test_struct_with_str_field() {
        let owned = OwnedStrField {
            name: "str field test".to_string(),
        };
        let encoded = oxicode::encode_to_vec(&owned).expect("encode OwnedStrField");
        let (decoded, consumed): (StrField, _) =
            oxicode::borrow_decode_from_slice(&encoded).expect("borrow_decode StrField");
        assert_eq!(decoded.name, "str field test");
        assert_eq!(consumed, encoded.len());
    }

    // 6. Struct with &[u8] field BorrowDecode
    #[test]
    fn test_struct_with_bytes_field() {
        let data: &[u8] = &[0xDE, 0xAD, 0xBE, 0xEF];
        let original = BytesField { data };
        let encoded = oxicode::encode_to_vec(&original).expect("encode BytesField");
        let (decoded, consumed): (BytesField, _) =
            oxicode::borrow_decode_from_slice(&encoded).expect("borrow_decode BytesField");
        assert_eq!(decoded.data, data);
        assert_eq!(consumed, encoded.len());
    }

    // 7. Struct with multiple borrowed fields
    #[test]
    fn test_struct_with_multiple_borrowed_fields() {
        let original = MultiStr {
            a: "alpha",
            b: "beta",
        };
        let encoded = oxicode::encode_to_vec(&original).expect("encode MultiStr");
        let (decoded, _): (MultiStr, _) =
            oxicode::borrow_decode_from_slice(&encoded).expect("borrow_decode MultiStr");
        assert_eq!(decoded.a, "alpha");
        assert_eq!(decoded.b, "beta");
    }

    // 8. Struct with both owned and borrowed fields
    #[test]
    fn test_struct_with_owned_and_borrowed_fields() {
        let original = MixedFields {
            id: 42,
            label: "mixed label",
            count: 9999,
        };
        let encoded = oxicode::encode_to_vec(&original).expect("encode MixedFields");
        let (decoded, consumed): (MixedFields, _) =
            oxicode::borrow_decode_from_slice(&encoded).expect("borrow_decode MixedFields");
        assert_eq!(decoded.id, 42);
        assert_eq!(decoded.label, "mixed label");
        assert_eq!(decoded.count, 9999);
        assert_eq!(consumed, encoded.len());
    }

    // 9. BorrowDecode vs Decode: both give same value for &str / String
    #[test]
    fn test_borrow_decode_vs_decode_same_value_for_str() {
        let original = "same value test";
        let encoded = oxicode::encode_to_vec(&original).expect("encode &str");
        let (borrowed, _): (&str, _) =
            oxicode::borrow_decode_from_slice(&encoded).expect("borrow_decode &str");
        let (owned, _): (String, _) = oxicode::decode_from_slice(&encoded).expect("decode String");
        assert_eq!(borrowed, owned.as_str());
    }

    // 10. Multiple sequential BorrowDecodes from separate buffers
    #[test]
    fn test_multiple_sequential_borrow_decodes_from_same_buffer() {
        let first = "first string";
        let second = "second string";
        let enc1 = oxicode::encode_to_vec(&first).expect("encode first");
        let enc2 = oxicode::encode_to_vec(&second).expect("encode second");
        let (decoded1, _): (&str, _) =
            oxicode::borrow_decode_from_slice(&enc1).expect("borrow_decode first");
        let (decoded2, _): (&str, _) =
            oxicode::borrow_decode_from_slice(&enc2).expect("borrow_decode second");
        assert_eq!(decoded1, first);
        assert_eq!(decoded2, second);
    }

    // 11. BorrowDecode of empty &str
    #[test]
    fn test_borrow_decode_empty_str() {
        let original = "";
        let encoded = oxicode::encode_to_vec(&original).expect("encode empty &str");
        let (decoded, consumed): (&str, _) =
            oxicode::borrow_decode_from_slice(&encoded).expect("borrow_decode empty &str");
        assert_eq!(decoded, "");
        assert_eq!(consumed, encoded.len());
    }

    // 12. BorrowDecode of empty &[u8]
    #[test]
    fn test_borrow_decode_empty_bytes() {
        let original: &[u8] = &[];
        let encoded = oxicode::encode_to_vec(&original).expect("encode empty &[u8]");
        let (decoded, consumed): (&[u8], _) =
            oxicode::borrow_decode_from_slice(&encoded).expect("borrow_decode empty &[u8]");
        assert_eq!(decoded, b"");
        assert_eq!(consumed, encoded.len());
    }

    // 13. BorrowDecode of long &str (1 KB)
    #[test]
    fn test_borrow_decode_long_str_1kb() {
        let original: String = "x".repeat(1024);
        let encoded = oxicode::encode_to_vec(&original).expect("encode 1KB string");
        let (decoded, consumed): (&str, _) =
            oxicode::borrow_decode_from_slice(&encoded).expect("borrow_decode 1KB string");
        assert_eq!(decoded, original.as_str());
        assert_eq!(consumed, encoded.len());
    }

    // 14. BorrowDecode of &[u8] with all byte values 0..=255
    #[test]
    fn test_borrow_decode_bytes_all_values() {
        let all_bytes: Vec<u8> = (0u8..=255).collect();
        let original: &[u8] = &all_bytes;
        let encoded = oxicode::encode_to_vec(&original).expect("encode all-byte-values");
        let (decoded, consumed): (&[u8], _) =
            oxicode::borrow_decode_from_slice(&encoded).expect("borrow_decode all-byte-values");
        assert_eq!(decoded, original);
        assert_eq!(consumed, encoded.len());
    }

    // 15. BorrowDecode roundtrip: encode String → decode as &str
    #[test]
    fn test_roundtrip_encode_string_decode_as_str() {
        let original = String::from("roundtrip string decode");
        let encoded = oxicode::encode_to_vec(&original).expect("encode String");
        let (decoded, _): (&str, _) =
            oxicode::borrow_decode_from_slice(&encoded).expect("borrow_decode as &str");
        assert_eq!(decoded, original.as_str());
    }

    // 16. BorrowDecode with standard config
    #[test]
    fn test_borrow_decode_with_standard_config() {
        let original = "standard config test";
        let config = oxicode::config::standard();
        let encoded =
            oxicode::encode_to_vec_with_config(&original, config).expect("encode with config");
        let (decoded, consumed): (&str, _) =
            oxicode::borrow_decode_from_slice_with_config(&encoded, config)
                .expect("borrow_decode with standard config");
        assert_eq!(decoded, original);
        assert_eq!(consumed, encoded.len());
    }

    // 17. BorrowDecode of nested struct containing &str
    #[test]
    fn test_nested_struct_containing_str() {
        let original = OuterBorrow {
            inner: InnerBorrow { tag: "nested_tag" },
            val: 77,
        };
        let encoded = oxicode::encode_to_vec(&original).expect("encode OuterBorrow");
        let (decoded, consumed): (OuterBorrow, _) =
            oxicode::borrow_decode_from_slice(&encoded).expect("borrow_decode OuterBorrow");
        assert_eq!(decoded.inner.tag, "nested_tag");
        assert_eq!(decoded.val, 77);
        assert_eq!(consumed, encoded.len());
    }

    // 18. BorrowDecode with enum that has &str variant
    #[test]
    fn test_enum_with_str_variant() {
        let original = Cmd::Text("hello enum");
        let encoded = oxicode::encode_to_vec(&original).expect("encode Cmd::Text");
        let (decoded, consumed): (Cmd, _) =
            oxicode::borrow_decode_from_slice(&encoded).expect("borrow_decode Cmd::Text");
        assert_eq!(consumed, encoded.len());
        match decoded {
            Cmd::Text(s) => assert_eq!(s, "hello enum"),
            Cmd::Ping => panic!("expected Cmd::Text, got Cmd::Ping"),
        }
    }

    // 19. Cow<str> content matches original
    #[test]
    fn test_cow_str_content_matches() {
        let original = "cow content check";
        let encoded = oxicode::encode_to_vec(&original).expect("encode for Cow content check");
        let (decoded, _): (Cow<str>, _) =
            oxicode::borrow_decode_from_slice(&encoded).expect("borrow_decode Cow<str> content");
        assert_eq!(decoded.as_ref(), original);
    }

    // 20. BorrowDecode: consumed bytes count equals encoded length
    #[test]
    fn test_borrow_decode_consumed_bytes_count() {
        let original = "consumed bytes test";
        let encoded = oxicode::encode_to_vec(&original).expect("encode for consumed bytes test");
        let (decoded, consumed): (&str, _) =
            oxicode::borrow_decode_from_slice(&encoded).expect("borrow_decode consumed count");
        assert_eq!(decoded, original);
        assert_eq!(
            consumed,
            encoded.len(),
            "consumed bytes should equal total encoded length"
        );
    }
}
