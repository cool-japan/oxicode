//! Advanced property-based roundtrip tests using proptest (set 5).
//!
//! Each proptest! block contains exactly one #[test] function.
//! Verifies that encoding then decoding produces the original value,
//! and that the number of bytes consumed equals the encoded length.

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
use oxicode::{
    config, decode_from_slice, decode_from_slice_with_config, encode_to_vec,
    encode_to_vec_with_config,
};
use proptest::prelude::*;

proptest! {
    #[test]
    fn prop_u8_roundtrip(value: u8) {
        let enc = encode_to_vec(&value).expect("encode u8");
        let (dec, consumed): (u8, usize) = decode_from_slice(&enc).expect("decode u8");
        prop_assert_eq!(dec, value);
        prop_assert_eq!(consumed, enc.len());
    }
}

proptest! {
    #[test]
    fn prop_u16_roundtrip(value: u16) {
        let enc = encode_to_vec(&value).expect("encode u16");
        let (dec, consumed): (u16, usize) = decode_from_slice(&enc).expect("decode u16");
        prop_assert_eq!(dec, value);
        prop_assert_eq!(consumed, enc.len());
    }
}

proptest! {
    #[test]
    fn prop_u32_roundtrip(value: u32) {
        let enc = encode_to_vec(&value).expect("encode u32");
        let (dec, consumed): (u32, usize) = decode_from_slice(&enc).expect("decode u32");
        prop_assert_eq!(dec, value);
        prop_assert_eq!(consumed, enc.len());
    }
}

proptest! {
    #[test]
    fn prop_u64_roundtrip(value: u64) {
        let enc = encode_to_vec(&value).expect("encode u64");
        let (dec, consumed): (u64, usize) = decode_from_slice(&enc).expect("decode u64");
        prop_assert_eq!(dec, value);
        prop_assert_eq!(consumed, enc.len());
    }
}

proptest! {
    #[test]
    fn prop_u128_roundtrip(value: u128) {
        let enc = encode_to_vec(&value).expect("encode u128");
        let (dec, consumed): (u128, usize) = decode_from_slice(&enc).expect("decode u128");
        prop_assert_eq!(dec, value);
        prop_assert_eq!(consumed, enc.len());
    }
}

proptest! {
    #[test]
    fn prop_i8_roundtrip(value: i8) {
        let enc = encode_to_vec(&value).expect("encode i8");
        let (dec, consumed): (i8, usize) = decode_from_slice(&enc).expect("decode i8");
        prop_assert_eq!(dec, value);
        prop_assert_eq!(consumed, enc.len());
    }
}

proptest! {
    #[test]
    fn prop_i16_roundtrip(value: i16) {
        let enc = encode_to_vec(&value).expect("encode i16");
        let (dec, consumed): (i16, usize) = decode_from_slice(&enc).expect("decode i16");
        prop_assert_eq!(dec, value);
        prop_assert_eq!(consumed, enc.len());
    }
}

proptest! {
    #[test]
    fn prop_i32_roundtrip(value: i32) {
        let enc = encode_to_vec(&value).expect("encode i32");
        let (dec, consumed): (i32, usize) = decode_from_slice(&enc).expect("decode i32");
        prop_assert_eq!(dec, value);
        prop_assert_eq!(consumed, enc.len());
    }
}

proptest! {
    #[test]
    fn prop_i64_roundtrip(value: i64) {
        let enc = encode_to_vec(&value).expect("encode i64");
        let (dec, consumed): (i64, usize) = decode_from_slice(&enc).expect("decode i64");
        prop_assert_eq!(dec, value);
        prop_assert_eq!(consumed, enc.len());
    }
}

proptest! {
    #[test]
    fn prop_bool_roundtrip(value: bool) {
        let enc = encode_to_vec(&value).expect("encode bool");
        let (dec, consumed): (bool, usize) = decode_from_slice(&enc).expect("decode bool");
        prop_assert_eq!(dec, value);
        prop_assert_eq!(consumed, enc.len());
    }
}

proptest! {
    #[test]
    fn prop_string_roundtrip(value in "[a-zA-Z0-9 ]{0,50}") {
        let enc = encode_to_vec(&value).expect("encode String");
        let (dec, consumed): (String, usize) = decode_from_slice(&enc).expect("decode String");
        prop_assert_eq!(dec, value);
        prop_assert_eq!(consumed, enc.len());
    }
}

proptest! {
    #[test]
    fn prop_vec_u8_roundtrip(value in proptest::collection::vec(0u8..=255, 0..100)) {
        let enc = encode_to_vec(&value).expect("encode Vec<u8>");
        let (dec, consumed): (Vec<u8>, usize) = decode_from_slice(&enc).expect("decode Vec<u8>");
        prop_assert_eq!(dec, value);
        prop_assert_eq!(consumed, enc.len());
    }
}

proptest! {
    #[test]
    fn prop_vec_u32_roundtrip(value in proptest::collection::vec(any::<u32>(), 0..50)) {
        let enc = encode_to_vec(&value).expect("encode Vec<u32>");
        let (dec, consumed): (Vec<u32>, usize) = decode_from_slice(&enc).expect("decode Vec<u32>");
        prop_assert_eq!(dec, value);
        prop_assert_eq!(consumed, enc.len());
    }
}

proptest! {
    #[test]
    fn prop_vec_string_roundtrip(value in proptest::collection::vec("[a-z]{0,10}", 0..10)) {
        let enc = encode_to_vec(&value).expect("encode Vec<String>");
        let (dec, consumed): (Vec<String>, usize) = decode_from_slice(&enc).expect("decode Vec<String>");
        prop_assert_eq!(dec, value);
        prop_assert_eq!(consumed, enc.len());
    }
}

proptest! {
    #[test]
    fn prop_option_u32_roundtrip(value: Option<u32>) {
        let enc = encode_to_vec(&value).expect("encode Option<u32>");
        let (dec, consumed): (Option<u32>, usize) = decode_from_slice(&enc).expect("decode Option<u32>");
        prop_assert_eq!(dec, value);
        prop_assert_eq!(consumed, enc.len());
    }
}

proptest! {
    #[test]
    fn prop_option_string_roundtrip(value in proptest::option::of("[a-zA-Z0-9]{0,30}")) {
        let enc = encode_to_vec(&value).expect("encode Option<String>");
        let (dec, consumed): (Option<String>, usize) = decode_from_slice(&enc).expect("decode Option<String>");
        prop_assert_eq!(dec, value);
        prop_assert_eq!(consumed, enc.len());
    }
}

proptest! {
    #[test]
    fn prop_tuple_u32_u64_roundtrip(value: (u32, u64)) {
        let enc = encode_to_vec(&value).expect("encode (u32, u64)");
        let (dec, consumed): ((u32, u64), usize) = decode_from_slice(&enc).expect("decode (u32, u64)");
        prop_assert_eq!(dec, value);
        prop_assert_eq!(consumed, enc.len());
    }
}

proptest! {
    #[test]
    fn prop_tuple_u8_bool_u32_roundtrip(value: (u8, bool, u32)) {
        let enc = encode_to_vec(&value).expect("encode (u8, bool, u32)");
        let (dec, consumed): ((u8, bool, u32), usize) = decode_from_slice(&enc).expect("decode (u8, bool, u32)");
        prop_assert_eq!(dec, value);
        prop_assert_eq!(consumed, enc.len());
    }
}

proptest! {
    #[test]
    fn prop_u32_fixed_int_config_roundtrip(value: u32) {
        let cfg = config::standard().with_fixed_int_encoding();
        let enc = encode_to_vec_with_config(&value, cfg).expect("encode u32 fixed-int");
        let (dec, consumed): (u32, usize) =
            decode_from_slice_with_config(&enc, cfg).expect("decode u32 fixed-int");
        prop_assert_eq!(dec, value);
        prop_assert_eq!(consumed, enc.len());
    }
}

proptest! {
    #[test]
    fn prop_u64_fixed_int_config_roundtrip(value: u64) {
        let cfg = config::standard().with_fixed_int_encoding();
        let enc = encode_to_vec_with_config(&value, cfg).expect("encode u64 fixed-int");
        let (dec, consumed): (u64, usize) =
            decode_from_slice_with_config(&enc, cfg).expect("decode u64 fixed-int");
        prop_assert_eq!(dec, value);
        prop_assert_eq!(consumed, enc.len());
    }
}

proptest! {
    #[test]
    fn prop_i32_consumed_equals_encoded_len(value: i32) {
        let enc = encode_to_vec(&value).expect("encode i32 for consumed check");
        let (_dec, consumed): (i32, usize) =
            decode_from_slice(&enc).expect("decode i32 for consumed check");
        prop_assert_eq!(consumed, enc.len());
    }
}

proptest! {
    #[test]
    fn prop_vec_bool_roundtrip(value in proptest::collection::vec(any::<bool>(), 0..100)) {
        let enc = encode_to_vec(&value).expect("encode Vec<bool>");
        let (dec, consumed): (Vec<bool>, usize) = decode_from_slice(&enc).expect("decode Vec<bool>");
        prop_assert_eq!(dec, value);
        prop_assert_eq!(consumed, enc.len());
    }
}
