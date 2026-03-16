//! Advanced property-based roundtrip tests (set 2) using proptest.
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
    fn prop_u8_roundtrip(value: u8) {
        let encoded = encode_to_vec(&value).expect("encode u8 failed");
        let (decoded, consumed): (u8, _) =
            decode_from_slice(&encoded).expect("decode u8 failed");
        prop_assert_eq!(decoded, value);
        prop_assert_eq!(consumed, encoded.len());
    }
}

proptest! {
    #[test]
    fn prop_u16_roundtrip(value: u16) {
        let encoded = encode_to_vec(&value).expect("encode u16 failed");
        let (decoded, consumed): (u16, _) =
            decode_from_slice(&encoded).expect("decode u16 failed");
        prop_assert_eq!(decoded, value);
        prop_assert_eq!(consumed, encoded.len());
    }
}

proptest! {
    #[test]
    fn prop_u32_roundtrip(value: u32) {
        let encoded = encode_to_vec(&value).expect("encode u32 failed");
        let (decoded, consumed): (u32, _) =
            decode_from_slice(&encoded).expect("decode u32 failed");
        prop_assert_eq!(decoded, value);
        prop_assert_eq!(consumed, encoded.len());
    }
}

proptest! {
    #[test]
    fn prop_u64_roundtrip(value: u64) {
        let encoded = encode_to_vec(&value).expect("encode u64 failed");
        let (decoded, consumed): (u64, _) =
            decode_from_slice(&encoded).expect("decode u64 failed");
        prop_assert_eq!(decoded, value);
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
    fn prop_i16_roundtrip(value: i16) {
        let encoded = encode_to_vec(&value).expect("encode i16 failed");
        let (decoded, consumed): (i16, _) =
            decode_from_slice(&encoded).expect("decode i16 failed");
        prop_assert_eq!(decoded, value);
        prop_assert_eq!(consumed, encoded.len());
    }
}

proptest! {
    #[test]
    fn prop_i32_roundtrip(value: i32) {
        let encoded = encode_to_vec(&value).expect("encode i32 failed");
        let (decoded, consumed): (i32, _) =
            decode_from_slice(&encoded).expect("decode i32 failed");
        prop_assert_eq!(decoded, value);
        prop_assert_eq!(consumed, encoded.len());
    }
}

proptest! {
    #[test]
    fn prop_i64_roundtrip(value: i64) {
        let encoded = encode_to_vec(&value).expect("encode i64 failed");
        let (decoded, consumed): (i64, _) =
            decode_from_slice(&encoded).expect("decode i64 failed");
        prop_assert_eq!(decoded, value);
        prop_assert_eq!(consumed, encoded.len());
    }
}

proptest! {
    #[test]
    fn prop_bool_roundtrip(value: bool) {
        let encoded = encode_to_vec(&value).expect("encode bool failed");
        let (decoded, consumed): (bool, _) =
            decode_from_slice(&encoded).expect("decode bool failed");
        prop_assert_eq!(decoded, value);
        prop_assert_eq!(consumed, encoded.len());
    }
}

proptest! {
    #[test]
    fn prop_f32_roundtrip(value in proptest::num::f32::NORMAL) {
        let encoded = encode_to_vec(&value).expect("encode f32 failed");
        let (decoded, consumed): (f32, _) =
            decode_from_slice(&encoded).expect("decode f32 failed");
        prop_assert_eq!(decoded, value);
        prop_assert_eq!(consumed, encoded.len());
    }
}

proptest! {
    #[test]
    fn prop_f64_roundtrip(value in proptest::num::f64::NORMAL) {
        let encoded = encode_to_vec(&value).expect("encode f64 failed");
        let (decoded, consumed): (f64, _) =
            decode_from_slice(&encoded).expect("decode f64 failed");
        prop_assert_eq!(decoded, value);
        prop_assert_eq!(consumed, encoded.len());
    }
}

proptest! {
    #[test]
    fn prop_string_roundtrip(value: String) {
        let encoded = encode_to_vec(&value).expect("encode String failed");
        let (decoded, consumed): (String, _) =
            decode_from_slice(&encoded).expect("decode String failed");
        prop_assert_eq!(decoded, value);
        prop_assert_eq!(consumed, encoded.len());
    }
}

proptest! {
    #[test]
    fn prop_vec_u8_roundtrip(v in proptest::collection::vec(any::<u8>(), 0..100)) {
        let encoded = encode_to_vec(&v).expect("encode Vec<u8> failed");
        let (decoded, consumed): (Vec<u8>, _) =
            decode_from_slice(&encoded).expect("decode Vec<u8> failed");
        prop_assert_eq!(decoded, v);
        prop_assert_eq!(consumed, encoded.len());
    }
}

proptest! {
    #[test]
    fn prop_vec_u32_roundtrip(v in proptest::collection::vec(any::<u32>(), 0..50)) {
        let encoded = encode_to_vec(&v).expect("encode Vec<u32> failed");
        let (decoded, consumed): (Vec<u32>, _) =
            decode_from_slice(&encoded).expect("decode Vec<u32> failed");
        prop_assert_eq!(decoded, v);
        prop_assert_eq!(consumed, encoded.len());
    }
}

proptest! {
    #[test]
    fn prop_option_u32_roundtrip(opt: Option<u32>) {
        let encoded = encode_to_vec(&opt).expect("encode Option<u32> failed");
        let (decoded, consumed): (Option<u32>, _) =
            decode_from_slice(&encoded).expect("decode Option<u32> failed");
        prop_assert_eq!(decoded, opt);
        prop_assert_eq!(consumed, encoded.len());
    }
}

proptest! {
    #[test]
    fn prop_option_string_roundtrip(opt: Option<String>) {
        let encoded = encode_to_vec(&opt).expect("encode Option<String> failed");
        let (decoded, consumed): (Option<String>, _) =
            decode_from_slice(&encoded).expect("decode Option<String> failed");
        prop_assert_eq!(decoded, opt);
        prop_assert_eq!(consumed, encoded.len());
    }
}

proptest! {
    #[test]
    fn prop_tuple_u32_string_roundtrip(value: (u32, String)) {
        let encoded = encode_to_vec(&value).expect("encode (u32, String) failed");
        let (decoded, consumed): ((u32, String), _) =
            decode_from_slice(&encoded).expect("decode (u32, String) failed");
        prop_assert_eq!(decoded, value);
        prop_assert_eq!(consumed, encoded.len());
    }
}

proptest! {
    #[test]
    fn prop_vec_string_roundtrip(v in proptest::collection::vec(any::<String>(), 0..20)) {
        let encoded = encode_to_vec(&v).expect("encode Vec<String> failed");
        let (decoded, consumed): (Vec<String>, _) =
            decode_from_slice(&encoded).expect("decode Vec<String> failed");
        prop_assert_eq!(decoded, v);
        prop_assert_eq!(consumed, encoded.len());
    }
}

proptest! {
    #[test]
    fn prop_u128_roundtrip(value in any::<u128>()) {
        let encoded = encode_to_vec(&value).expect("encode u128 failed");
        let (decoded, consumed): (u128, _) =
            decode_from_slice(&encoded).expect("decode u128 failed");
        prop_assert_eq!(decoded, value);
        prop_assert_eq!(consumed, encoded.len());
    }
}

proptest! {
    #[test]
    fn prop_i128_roundtrip(value in any::<i128>()) {
        let encoded = encode_to_vec(&value).expect("encode i128 failed");
        let (decoded, consumed): (i128, _) =
            decode_from_slice(&encoded).expect("decode i128 failed");
        prop_assert_eq!(decoded, value);
        prop_assert_eq!(consumed, encoded.len());
    }
}

proptest! {
    #[test]
    fn prop_consumed_equals_encoded_len_u64(value: u64) {
        let encoded = encode_to_vec(&value).expect("encode u64 failed");
        let (_decoded, consumed): (u64, _) =
            decode_from_slice(&encoded).expect("decode u64 failed");
        prop_assert_eq!(consumed, encoded.len());
    }
}

proptest! {
    #[test]
    fn prop_consumed_equals_encoded_len_string(value: String) {
        let encoded = encode_to_vec(&value).expect("encode String failed");
        let (_decoded, consumed): (String, _) =
            decode_from_slice(&encoded).expect("decode String failed");
        prop_assert_eq!(consumed, encoded.len());
    }
}
