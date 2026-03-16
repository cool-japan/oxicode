//! Advanced property-based roundtrip tests (set 3) using proptest.
//!
//! Each proptest! block contains exactly one #[test] function.
//! Tests verify that encode → decode is a perfect roundtrip for all tested types.

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
use proptest::prelude::*;

proptest! {
    #[test]
    fn prop_isize_roundtrip(value: isize) {
        let encoded = encode_to_vec(&value).expect("encode isize failed");
        let (decoded, consumed): (isize, _) =
            decode_from_slice(&encoded).expect("decode isize failed");
        prop_assert_eq!(decoded, value);
        prop_assert_eq!(consumed, encoded.len());
    }
}

proptest! {
    #[test]
    fn prop_usize_roundtrip(value: usize) {
        let encoded = encode_to_vec(&value).expect("encode usize failed");
        let (decoded, consumed): (usize, _) =
            decode_from_slice(&encoded).expect("decode usize failed");
        prop_assert_eq!(decoded, value);
        prop_assert_eq!(consumed, encoded.len());
    }
}

proptest! {
    #[test]
    fn prop_char_roundtrip(value in proptest::char::any()) {
        let encoded = encode_to_vec(&value).expect("encode char failed");
        let (decoded, consumed): (char, _) =
            decode_from_slice(&encoded).expect("decode char failed");
        prop_assert_eq!(decoded, value);
        prop_assert_eq!(consumed, encoded.len());
    }
}

proptest! {
    #[test]
    fn prop_vec_i32_roundtrip(v in proptest::collection::vec(any::<i32>(), 0..50)) {
        let encoded = encode_to_vec(&v).expect("encode Vec<i32> failed");
        let (decoded, consumed): (Vec<i32>, _) =
            decode_from_slice(&encoded).expect("decode Vec<i32> failed");
        prop_assert_eq!(decoded, v);
        prop_assert_eq!(consumed, encoded.len());
    }
}

proptest! {
    #[test]
    fn prop_vec_bool_roundtrip(v in proptest::collection::vec(any::<bool>(), 0..20)) {
        let encoded = encode_to_vec(&v).expect("encode Vec<bool> failed");
        let (decoded, consumed): (Vec<bool>, _) =
            decode_from_slice(&encoded).expect("decode Vec<bool> failed");
        prop_assert_eq!(decoded, v);
        prop_assert_eq!(consumed, encoded.len());
    }
}

proptest! {
    #[test]
    fn prop_option_i64_roundtrip(opt: Option<i64>) {
        let encoded = encode_to_vec(&opt).expect("encode Option<i64> failed");
        let (decoded, consumed): (Option<i64>, _) =
            decode_from_slice(&encoded).expect("decode Option<i64> failed");
        prop_assert_eq!(decoded, opt);
        prop_assert_eq!(consumed, encoded.len());
    }
}

proptest! {
    #[test]
    fn prop_tuple_u8_u16_roundtrip(value: (u8, u16)) {
        let encoded = encode_to_vec(&value).expect("encode (u8, u16) failed");
        let (decoded, consumed): ((u8, u16), _) =
            decode_from_slice(&encoded).expect("decode (u8, u16) failed");
        prop_assert_eq!(decoded, value);
        prop_assert_eq!(consumed, encoded.len());
    }
}

proptest! {
    #[test]
    fn prop_tuple_i32_bool_roundtrip(value: (i32, bool)) {
        let encoded = encode_to_vec(&value).expect("encode (i32, bool) failed");
        let (decoded, consumed): ((i32, bool), _) =
            decode_from_slice(&encoded).expect("decode (i32, bool) failed");
        prop_assert_eq!(decoded, value);
        prop_assert_eq!(consumed, encoded.len());
    }
}

proptest! {
    #[test]
    fn prop_nested_option_roundtrip(opt: Option<Option<u32>>) {
        let encoded = encode_to_vec(&opt).expect("encode Option<Option<u32>> failed");
        let (decoded, consumed): (Option<Option<u32>>, _) =
            decode_from_slice(&encoded).expect("decode Option<Option<u32>> failed");
        prop_assert_eq!(decoded, opt);
        prop_assert_eq!(consumed, encoded.len());
    }
}

proptest! {
    #[test]
    fn prop_vec_option_u8_roundtrip(v in proptest::collection::vec(any::<Option<u8>>(), 0..20)) {
        let encoded = encode_to_vec(&v).expect("encode Vec<Option<u8>> failed");
        let (decoded, consumed): (Vec<Option<u8>>, _) =
            decode_from_slice(&encoded).expect("decode Vec<Option<u8>> failed");
        prop_assert_eq!(decoded, v);
        prop_assert_eq!(consumed, encoded.len());
    }
}

proptest! {
    #[test]
    fn prop_i32_consumed_eq_len(value: i32) {
        let encoded = encode_to_vec(&value).expect("encode i32 failed");
        let (decoded, consumed): (i32, _) =
            decode_from_slice(&encoded).expect("decode i32 failed");
        prop_assert_eq!(decoded, value);
        prop_assert_eq!(consumed, encoded.len());
    }
}

proptest! {
    #[test]
    fn prop_string_consumed_eq_len(value: String) {
        let encoded = encode_to_vec(&value).expect("encode String failed");
        let (decoded, consumed): (String, _) =
            decode_from_slice(&encoded).expect("decode String failed");
        prop_assert_eq!(decoded, value);
        prop_assert_eq!(consumed, encoded.len());
    }
}

proptest! {
    #[test]
    fn prop_vec_u64_consumed_eq_len(v in proptest::collection::vec(any::<u64>(), 0..30)) {
        let encoded = encode_to_vec(&v).expect("encode Vec<u64> failed");
        let (decoded, consumed): (Vec<u64>, _) =
            decode_from_slice(&encoded).expect("decode Vec<u64> failed");
        prop_assert_eq!(decoded, v);
        prop_assert_eq!(consumed, encoded.len());
    }
}

proptest! {
    #[test]
    fn prop_tuple3_roundtrip(value: (u8, u32, u64)) {
        let encoded = encode_to_vec(&value).expect("encode (u8, u32, u64) failed");
        let (decoded, consumed): ((u8, u32, u64), _) =
            decode_from_slice(&encoded).expect("decode (u8, u32, u64) failed");
        prop_assert_eq!(decoded, value);
        prop_assert_eq!(consumed, encoded.len());
    }
}

proptest! {
    #[test]
    fn prop_tuple4_roundtrip(value: (bool, i32, u64, String)) {
        let encoded = encode_to_vec(&value).expect("encode (bool, i32, u64, String) failed");
        let (decoded, consumed): ((bool, i32, u64, String), _) =
            decode_from_slice(&encoded).expect("decode (bool, i32, u64, String) failed");
        prop_assert_eq!(decoded, value);
        prop_assert_eq!(consumed, encoded.len());
    }
}

proptest! {
    #[test]
    fn prop_vec_string_consumed_eq_len(v in proptest::collection::vec(any::<String>(), 0..10)) {
        let encoded = encode_to_vec(&v).expect("encode Vec<String> failed");
        let (decoded, consumed): (Vec<String>, _) =
            decode_from_slice(&encoded).expect("decode Vec<String> failed");
        prop_assert_eq!(decoded, v);
        prop_assert_eq!(consumed, encoded.len());
    }
}

proptest! {
    #[test]
    fn prop_i8_roundtrip(value: i8) {
        let encoded = encode_to_vec(&value).expect("encode i8 failed");
        let (decoded, consumed): (i8, _) =
            decode_from_slice(&encoded).expect("decode i8 failed");
        prop_assert_eq!(decoded, value);
        prop_assert_eq!(consumed, encoded.len());
    }
}

proptest! {
    #[test]
    fn prop_u128_range_roundtrip(value in 0u128..u32::MAX as u128) {
        let encoded = encode_to_vec(&value).expect("encode u128 range failed");
        let (decoded, consumed): (u128, _) =
            decode_from_slice(&encoded).expect("decode u128 range failed");
        prop_assert_eq!(decoded, value);
        prop_assert_eq!(consumed, encoded.len());
    }
}

proptest! {
    #[test]
    fn prop_option_vec_u8_roundtrip(
        opt in proptest::option::of(proptest::collection::vec(any::<u8>(), 0..20))
    ) {
        let encoded = encode_to_vec(&opt).expect("encode Option<Vec<u8>> failed");
        let (decoded, consumed): (Option<Vec<u8>>, _) =
            decode_from_slice(&encoded).expect("decode Option<Vec<u8>> failed");
        prop_assert_eq!(decoded, opt);
        prop_assert_eq!(consumed, encoded.len());
    }
}

proptest! {
    #[test]
    fn prop_string_unicode_roundtrip(value: String) {
        let encoded = encode_to_vec(&value).expect("encode unicode String failed");
        let (decoded, consumed): (String, _) =
            decode_from_slice(&encoded).expect("decode unicode String failed");
        prop_assert_eq!(decoded, value);
        prop_assert_eq!(consumed, encoded.len());
    }
}

proptest! {
    #[test]
    fn prop_vec_of_pairs_roundtrip(
        v in proptest::collection::vec((any::<u32>(), any::<String>()), 0..10)
    ) {
        let encoded = encode_to_vec(&v).expect("encode Vec<(u32, String)> failed");
        let (decoded, consumed): (Vec<(u32, String)>, _) =
            decode_from_slice(&encoded).expect("decode Vec<(u32, String)> failed");
        prop_assert_eq!(decoded, v);
        prop_assert_eq!(consumed, encoded.len());
    }
}

proptest! {
    #[test]
    fn prop_deeply_nested_roundtrip(
        opt in proptest::option::of(proptest::option::of(proptest::option::of(any::<bool>())))
    ) {
        let encoded = encode_to_vec(&opt).expect("encode Option<Option<Option<bool>>> failed");
        let (decoded, consumed): (Option<Option<Option<bool>>>, _) =
            decode_from_slice(&encoded).expect("decode Option<Option<Option<bool>>> failed");
        prop_assert_eq!(decoded, opt);
        prop_assert_eq!(consumed, encoded.len());
    }
}
