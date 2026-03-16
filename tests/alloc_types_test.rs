//! Tests for alloc-only types (types available without std but with alloc).
//! These tests verify correct roundtrip encoding/decoding for heap-allocated
//! and alloc-based collection types.

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
use oxicode::{decode_from_slice, encode_to_vec};
use std::collections::{BTreeMap, BTreeSet};
use std::rc::Rc;

mod alloc_types_tests {
    use super::*;

    // 1. String (heap-allocated) roundtrip
    #[test]
    fn test_string_roundtrip() {
        let value = String::from("hello, oxicode!");
        let encoded = encode_to_vec(&value).expect("Failed to encode String");
        let (decoded, _): (String, _) =
            decode_from_slice(&encoded).expect("Failed to decode String");
        assert_eq!(value, decoded);
    }

    // 2. Vec<u8> roundtrip
    #[test]
    fn test_vec_u8_roundtrip() {
        let value: Vec<u8> = vec![0u8, 1, 127, 128, 255];
        let encoded = encode_to_vec(&value).expect("Failed to encode Vec<u8>");
        let (decoded, _): (Vec<u8>, _) =
            decode_from_slice(&encoded).expect("Failed to decode Vec<u8>");
        assert_eq!(value, decoded);
    }

    // 3. Vec<String> roundtrip
    #[test]
    fn test_vec_string_roundtrip() {
        let value: Vec<String> = vec![
            String::from("alpha"),
            String::from("beta"),
            String::from("gamma"),
        ];
        let encoded = encode_to_vec(&value).expect("Failed to encode Vec<String>");
        let (decoded, _): (Vec<String>, _) =
            decode_from_slice(&encoded).expect("Failed to decode Vec<String>");
        assert_eq!(value, decoded);
    }

    // 4. Box<u32> roundtrip
    #[test]
    fn test_box_u32_roundtrip() {
        let value: Box<u32> = Box::new(42u32);
        let encoded = encode_to_vec(&value).expect("Failed to encode Box<u32>");
        let (decoded, _): (Box<u32>, _) =
            decode_from_slice(&encoded).expect("Failed to decode Box<u32>");
        assert_eq!(value, decoded);
    }

    // 5. Box<str> roundtrip
    #[test]
    fn test_box_str_roundtrip() {
        let value: Box<str> = Box::from("boxed string slice");
        let encoded = encode_to_vec(&value).expect("Failed to encode Box<str>");
        let (decoded, _): (Box<str>, _) =
            decode_from_slice(&encoded).expect("Failed to decode Box<str>");
        assert_eq!(value, decoded);
    }

    // 6. Box<[u8]> roundtrip
    #[test]
    fn test_box_slice_u8_roundtrip() {
        let value: Box<[u8]> = vec![10u8, 20, 30, 40, 50].into_boxed_slice();
        let encoded = encode_to_vec(&value).expect("Failed to encode Box<[u8]>");
        let (decoded, _): (Box<[u8]>, _) =
            decode_from_slice(&encoded).expect("Failed to decode Box<[u8]>");
        assert_eq!(value, decoded);
    }

    // 7. BTreeMap<String, u64> roundtrip (alloc-only collection)
    #[test]
    fn test_btreemap_string_u64_roundtrip() {
        let mut value: BTreeMap<String, u64> = BTreeMap::new();
        value.insert(String::from("one"), 1u64);
        value.insert(String::from("two"), 2u64);
        value.insert(String::from("three"), 3u64);
        let encoded = encode_to_vec(&value).expect("Failed to encode BTreeMap<String, u64>");
        let (decoded, _): (BTreeMap<String, u64>, _) =
            decode_from_slice(&encoded).expect("Failed to decode BTreeMap<String, u64>");
        assert_eq!(value, decoded);
    }

    // 8. BTreeSet<u32> roundtrip
    #[test]
    fn test_btreeset_u32_roundtrip() {
        let value: BTreeSet<u32> = [7u32, 3, 1, 4, 1, 5, 9, 2, 6].iter().copied().collect();
        let encoded = encode_to_vec(&value).expect("Failed to encode BTreeSet<u32>");
        let (decoded, _): (BTreeSet<u32>, _) =
            decode_from_slice(&encoded).expect("Failed to decode BTreeSet<u32>");
        assert_eq!(value, decoded);
    }

    // 9. Rc<u32> roundtrip
    #[test]
    fn test_rc_u32_roundtrip() {
        let value: Rc<u32> = Rc::new(99u32);
        let encoded = encode_to_vec(&value).expect("Failed to encode Rc<u32>");
        let (decoded, _): (Rc<u32>, _) =
            decode_from_slice(&encoded).expect("Failed to decode Rc<u32>");
        assert_eq!(value, decoded);
    }

    // 10. Rc<str> roundtrip
    #[test]
    fn test_rc_str_roundtrip() {
        let value: Rc<str> = Rc::from("reference counted str");
        let encoded = encode_to_vec(&value).expect("Failed to encode Rc<str>");
        let (decoded, _): (Rc<str>, _) =
            decode_from_slice(&encoded).expect("Failed to decode Rc<str>");
        assert_eq!(value, decoded);
    }

    // 11. Rc<[u8]> roundtrip
    #[test]
    fn test_rc_slice_u8_roundtrip() {
        let value: Rc<[u8]> = Rc::from(vec![11u8, 22, 33, 44].as_slice());
        let encoded = encode_to_vec(&value).expect("Failed to encode Rc<[u8]>");
        let (decoded, _): (Rc<[u8]>, _) =
            decode_from_slice(&encoded).expect("Failed to decode Rc<[u8]>");
        assert_eq!(value, decoded);
    }

    // 12. Vec<Box<str>> roundtrip
    #[test]
    fn test_vec_box_str_roundtrip() {
        let value: Vec<Box<str>> =
            vec![Box::from("first"), Box::from("second"), Box::from("third")];
        let encoded = encode_to_vec(&value).expect("Failed to encode Vec<Box<str>>");
        let (decoded, _): (Vec<Box<str>>, _) =
            decode_from_slice(&encoded).expect("Failed to decode Vec<Box<str>>");
        assert_eq!(value, decoded);
    }

    // 13. BTreeMap<u32, Vec<String>> roundtrip
    #[test]
    fn test_btreemap_u32_vec_string_roundtrip() {
        let mut value: BTreeMap<u32, Vec<String>> = BTreeMap::new();
        value.insert(1u32, vec![String::from("a"), String::from("b")]);
        value.insert(2u32, vec![String::from("c")]);
        value.insert(
            3u32,
            vec![String::from("d"), String::from("e"), String::from("f")],
        );
        let encoded = encode_to_vec(&value).expect("Failed to encode BTreeMap<u32, Vec<String>>");
        let (decoded, _): (BTreeMap<u32, Vec<String>>, _) =
            decode_from_slice(&encoded).expect("Failed to decode BTreeMap<u32, Vec<String>>");
        assert_eq!(value, decoded);
    }

    // 14. Vec<Rc<u32>> roundtrip
    #[test]
    fn test_vec_rc_u32_roundtrip() {
        let value: Vec<Rc<u32>> = vec![Rc::new(10u32), Rc::new(20u32), Rc::new(30u32)];
        let encoded = encode_to_vec(&value).expect("Failed to encode Vec<Rc<u32>>");
        let (decoded, _): (Vec<Rc<u32>>, _) =
            decode_from_slice(&encoded).expect("Failed to decode Vec<Rc<u32>>");
        assert_eq!(value, decoded);
    }

    // 15. BTreeSet<String> roundtrip
    #[test]
    fn test_btreeset_string_roundtrip() {
        let value: BTreeSet<String> = ["cherry", "apple", "banana", "date"]
            .iter()
            .map(|s| s.to_string())
            .collect();
        let encoded = encode_to_vec(&value).expect("Failed to encode BTreeSet<String>");
        let (decoded, _): (BTreeSet<String>, _) =
            decode_from_slice(&encoded).expect("Failed to decode BTreeSet<String>");
        assert_eq!(value, decoded);
    }

    // 16. Box<Vec<u32>> roundtrip (clippy::box_collection suppressed intentionally)
    #[test]
    #[allow(clippy::box_collection)]
    fn test_box_vec_u32_roundtrip() {
        let value: Box<Vec<u32>> = Box::new(vec![100u32, 200, 300, 400]);
        let encoded = encode_to_vec(&value).expect("Failed to encode Box<Vec<u32>>");
        let (decoded, _): (Box<Vec<u32>>, _) =
            decode_from_slice(&encoded).expect("Failed to decode Box<Vec<u32>>");
        assert_eq!(value, decoded);
    }

    // 17. Vec<BTreeMap<String, u32>> roundtrip
    #[test]
    fn test_vec_btreemap_roundtrip() {
        let mut map1: BTreeMap<String, u32> = BTreeMap::new();
        map1.insert(String::from("x"), 10u32);
        map1.insert(String::from("y"), 20u32);

        let mut map2: BTreeMap<String, u32> = BTreeMap::new();
        map2.insert(String::from("a"), 1u32);

        let value: Vec<BTreeMap<String, u32>> = vec![map1, map2];
        let encoded = encode_to_vec(&value).expect("Failed to encode Vec<BTreeMap<String, u32>>");
        let (decoded, _): (Vec<BTreeMap<String, u32>>, _) =
            decode_from_slice(&encoded).expect("Failed to decode Vec<BTreeMap<String, u32>>");
        assert_eq!(value, decoded);
    }

    // 18. Rc<Vec<u8>> roundtrip
    #[test]
    fn test_rc_vec_u8_roundtrip() {
        let value: Rc<Vec<u8>> = Rc::new(vec![9u8, 8, 7, 6, 5]);
        let encoded = encode_to_vec(&value).expect("Failed to encode Rc<Vec<u8>>");
        let (decoded, _): (Rc<Vec<u8>>, _) =
            decode_from_slice(&encoded).expect("Failed to decode Rc<Vec<u8>>");
        assert_eq!(value, decoded);
    }

    // 19. Option<Box<str>> roundtrip
    #[test]
    fn test_option_box_str_roundtrip() {
        let value_some: Option<Box<str>> = Some(Box::from("optional boxed str"));
        let encoded_some = encode_to_vec(&value_some).expect("Failed to encode Some(Box<str>)");
        let (decoded_some, _): (Option<Box<str>>, _) =
            decode_from_slice(&encoded_some).expect("Failed to decode Some(Box<str>)");
        assert_eq!(value_some, decoded_some);

        let value_none: Option<Box<str>> = None;
        let encoded_none = encode_to_vec(&value_none).expect("Failed to encode None::<Box<str>>");
        let (decoded_none, _): (Option<Box<str>>, _) =
            decode_from_slice(&encoded_none).expect("Failed to decode None::<Box<str>>");
        assert_eq!(value_none, decoded_none);
    }

    // 20. BTreeMap<String, BTreeSet<u32>> roundtrip
    #[test]
    fn test_btreemap_string_btreeset_roundtrip() {
        let mut value: BTreeMap<String, BTreeSet<u32>> = BTreeMap::new();
        value.insert(
            String::from("primes"),
            [2u32, 3, 5, 7, 11].iter().copied().collect(),
        );
        value.insert(
            String::from("squares"),
            [1u32, 4, 9, 16, 25].iter().copied().collect(),
        );
        value.insert(String::from("empty"), BTreeSet::new());
        let encoded =
            encode_to_vec(&value).expect("Failed to encode BTreeMap<String, BTreeSet<u32>>");
        let (decoded, _): (BTreeMap<String, BTreeSet<u32>>, _) =
            decode_from_slice(&encoded).expect("Failed to decode BTreeMap<String, BTreeSet<u32>>");
        assert_eq!(value, decoded);
    }
}
