//! Extended collections, nested generics, and misc type property-based roundtrip tests using proptest
//! (split from proptest_test.rs).
//!
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
use std::collections::LinkedList;

// 2. prop_linkedlist_roundtrip - LinkedList<u32> up to 200 elements
proptest! {
    #[test]
    fn prop_linkedlist_roundtrip(
        v in proptest::collection::vec(any::<u32>(), 0..=200)
    ) {
        let list: LinkedList<u32> = v.into_iter().collect();
        let enc = oxicode::encode_to_vec(&list).expect("encode");
        let (dec, _): (LinkedList<u32>, _) = oxicode::decode_from_slice(&enc).expect("decode");
        prop_assert_eq!(list, dec);
    }
}

// 7. prop_nested_vec_string_roundtrip - Vec<Vec<String>> outer max 10, inner max 5, string max 20
proptest! {
    #[test]
    fn prop_nested_vec_string_roundtrip(
        outer in proptest::collection::vec(
            proptest::collection::vec(
                proptest::string::string_regex("[a-z]{1,20}").unwrap(),
                0..=5
            ),
            0..=10
        )
    ) {
        let enc = oxicode::encode_to_vec(&outer).expect("encode");
        let (dec, _): (Vec<Vec<String>>, _) = oxicode::decode_from_slice(&enc).expect("decode");
        prop_assert_eq!(outer, dec);
    }
}

// 8. prop_option_string_roundtrip - Option<String>
proptest! {
    #[test]
    fn prop_option_string_roundtrip(v: Option<String>) {
        let enc = encode_to_vec(&v).expect("encode");
        let (dec, _): (Option<String>, _) = decode_from_slice(&enc).expect("decode");
        prop_assert_eq!(v, dec);
    }
}

// 9. prop_option_vec_u8_roundtrip - Option<Vec<u8>> up to 1000 bytes
proptest! {
    #[test]
    fn prop_option_vec_u8_roundtrip(
        v in proptest::option::of(proptest::collection::vec(any::<u8>(), 0..=1000))
    ) {
        let enc = oxicode::encode_to_vec(&v).expect("encode");
        let (dec, _): (Option<Vec<u8>>, _) = oxicode::decode_from_slice(&enc).expect("decode");
        prop_assert_eq!(v, dec);
    }
}

// 10. prop_result_ok_u64_roundtrip - Result<u64, String> where Ok
proptest! {
    #[test]
    fn prop_result_ok_u64_roundtrip(val: u64) {
        let r: Result<u64, String> = Ok(val);
        let enc = oxicode::encode_to_vec(&r).expect("encode");
        let (dec, _): (Result<u64, String>, _) = oxicode::decode_from_slice(&enc).expect("decode");
        prop_assert_eq!(r, dec);
    }
}

// 11. prop_result_err_string_roundtrip - Result<u32, String> where Err
proptest! {
    #[test]
    fn prop_result_err_string_roundtrip(
        msg in proptest::string::string_regex("[a-zA-Z0-9 ]{0,80}").unwrap()
    ) {
        let r: Result<u32, String> = Err(msg);
        let enc = oxicode::encode_to_vec(&r).expect("encode");
        let (dec, _): (Result<u32, String>, _) = oxicode::decode_from_slice(&enc).expect("decode");
        prop_assert_eq!(r, dec);
    }
}
