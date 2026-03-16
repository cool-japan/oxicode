//! Advanced property-based roundtrip tests using proptest (set 6).
//!
//! Each #[test] function contains exactly one proptest! block.
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

#[test]
fn test_prop_u8_roundtrip() {
    proptest!(|(value: u8)| {
        let enc = encode_to_vec(&value).expect("encode");
        let (dec, consumed): (u8, usize) = decode_from_slice(&enc).expect("decode");
        prop_assert_eq!(value, dec);
        prop_assert_eq!(consumed, enc.len());
    });
}

#[test]
fn test_prop_u16_roundtrip() {
    proptest!(|(value: u16)| {
        let enc = encode_to_vec(&value).expect("encode");
        let (dec, consumed): (u16, usize) = decode_from_slice(&enc).expect("decode");
        prop_assert_eq!(value, dec);
        prop_assert_eq!(consumed, enc.len());
    });
}

#[test]
fn test_prop_u32_roundtrip() {
    proptest!(|(value: u32)| {
        let enc = encode_to_vec(&value).expect("encode");
        let (dec, consumed): (u32, usize) = decode_from_slice(&enc).expect("decode");
        prop_assert_eq!(value, dec);
        prop_assert_eq!(consumed, enc.len());
    });
}

#[test]
fn test_prop_u64_roundtrip() {
    proptest!(|(value: u64)| {
        let enc = encode_to_vec(&value).expect("encode");
        let (dec, consumed): (u64, usize) = decode_from_slice(&enc).expect("decode");
        prop_assert_eq!(value, dec);
        prop_assert_eq!(consumed, enc.len());
    });
}

#[test]
fn test_prop_u128_roundtrip() {
    proptest!(|(value: u128)| {
        let enc = encode_to_vec(&value).expect("encode");
        let (dec, consumed): (u128, usize) = decode_from_slice(&enc).expect("decode");
        prop_assert_eq!(value, dec);
        prop_assert_eq!(consumed, enc.len());
    });
}

#[test]
fn test_prop_i8_roundtrip() {
    proptest!(|(value: i8)| {
        let enc = encode_to_vec(&value).expect("encode");
        let (dec, consumed): (i8, usize) = decode_from_slice(&enc).expect("decode");
        prop_assert_eq!(value, dec);
        prop_assert_eq!(consumed, enc.len());
    });
}

#[test]
fn test_prop_i16_roundtrip() {
    proptest!(|(value: i16)| {
        let enc = encode_to_vec(&value).expect("encode");
        let (dec, consumed): (i16, usize) = decode_from_slice(&enc).expect("decode");
        prop_assert_eq!(value, dec);
        prop_assert_eq!(consumed, enc.len());
    });
}

#[test]
fn test_prop_i32_roundtrip() {
    proptest!(|(value: i32)| {
        let enc = encode_to_vec(&value).expect("encode");
        let (dec, consumed): (i32, usize) = decode_from_slice(&enc).expect("decode");
        prop_assert_eq!(value, dec);
        prop_assert_eq!(consumed, enc.len());
    });
}

#[test]
fn test_prop_i64_roundtrip() {
    proptest!(|(value: i64)| {
        let enc = encode_to_vec(&value).expect("encode");
        let (dec, consumed): (i64, usize) = decode_from_slice(&enc).expect("decode");
        prop_assert_eq!(value, dec);
        prop_assert_eq!(consumed, enc.len());
    });
}

#[test]
fn test_prop_i128_roundtrip() {
    proptest!(|(value: i128)| {
        let enc = encode_to_vec(&value).expect("encode");
        let (dec, consumed): (i128, usize) = decode_from_slice(&enc).expect("decode");
        prop_assert_eq!(value, dec);
        prop_assert_eq!(consumed, enc.len());
    });
}

#[test]
fn test_prop_bool_roundtrip() {
    proptest!(|(value: bool)| {
        let enc = encode_to_vec(&value).expect("encode");
        let (dec, consumed): (bool, usize) = decode_from_slice(&enc).expect("decode");
        prop_assert_eq!(value, dec);
        prop_assert_eq!(consumed, enc.len());
    });
}

#[test]
fn test_prop_char_roundtrip() {
    proptest!(|(value: char)| {
        let enc = encode_to_vec(&value).expect("encode");
        let (dec, consumed): (char, usize) = decode_from_slice(&enc).expect("decode");
        prop_assert_eq!(value, dec);
        prop_assert_eq!(consumed, enc.len());
    });
}

#[test]
fn test_prop_string_roundtrip() {
    proptest!(|(value in "[a-z]{0,20}")| {
        let enc = encode_to_vec(&value).expect("encode");
        let (dec, consumed): (String, usize) = decode_from_slice(&enc).expect("decode");
        prop_assert_eq!(value, dec);
        prop_assert_eq!(consumed, enc.len());
    });
}

#[test]
fn test_prop_vec_u32_roundtrip() {
    proptest!(|(value in proptest::collection::vec(any::<u32>(), 0..50))| {
        let enc = encode_to_vec(&value).expect("encode");
        let (dec, consumed): (Vec<u32>, usize) = decode_from_slice(&enc).expect("decode");
        prop_assert_eq!(value, dec);
        prop_assert_eq!(consumed, enc.len());
    });
}

#[test]
fn test_prop_option_u64_roundtrip() {
    proptest!(|(value: Option<u64>)| {
        let enc = encode_to_vec(&value).expect("encode");
        let (dec, consumed): (Option<u64>, usize) = decode_from_slice(&enc).expect("decode");
        prop_assert_eq!(value, dec);
        prop_assert_eq!(consumed, enc.len());
    });
}

#[test]
fn test_prop_tuple_u32_u64_roundtrip() {
    proptest!(|(value: (u32, u64))| {
        let enc = encode_to_vec(&value).expect("encode");
        let (dec, consumed): ((u32, u64), usize) = decode_from_slice(&enc).expect("decode");
        prop_assert_eq!(value, dec);
        prop_assert_eq!(consumed, enc.len());
    });
}

#[test]
fn test_prop_tuple_u8_bool_roundtrip() {
    proptest!(|(value: (u8, bool))| {
        let enc = encode_to_vec(&value).expect("encode");
        let (dec, consumed): ((u8, bool), usize) = decode_from_slice(&enc).expect("decode");
        prop_assert_eq!(value, dec);
        prop_assert_eq!(consumed, enc.len());
    });
}

#[test]
fn test_prop_vec_string_roundtrip() {
    proptest!(|(value in proptest::collection::vec(any::<String>(), 0..10))| {
        let enc = encode_to_vec(&value).expect("encode");
        let (dec, consumed): (Vec<String>, usize) = decode_from_slice(&enc).expect("decode");
        prop_assert_eq!(value, dec);
        prop_assert_eq!(consumed, enc.len());
    });
}

#[test]
fn test_prop_u32_fixed_int_roundtrip() {
    let cfg = config::standard().with_fixed_int_encoding();
    proptest!(|(value: u32)| {
        let enc = encode_to_vec_with_config(&value, cfg).expect("encode");
        let (dec, consumed): (u32, usize) = decode_from_slice_with_config(&enc, cfg).expect("decode");
        prop_assert_eq!(value, dec);
        prop_assert_eq!(consumed, enc.len());
    });
}

#[test]
fn test_prop_i64_fixed_int_roundtrip() {
    let cfg = config::standard().with_fixed_int_encoding();
    proptest!(|(value: i64)| {
        let enc = encode_to_vec_with_config(&value, cfg).expect("encode");
        let (dec, consumed): (i64, usize) = decode_from_slice_with_config(&enc, cfg).expect("decode");
        prop_assert_eq!(value, dec);
        prop_assert_eq!(consumed, enc.len());
    });
}

#[test]
fn test_prop_option_bool_roundtrip() {
    proptest!(|(value: Option<bool>)| {
        let enc = encode_to_vec(&value).expect("encode");
        let (dec, consumed): (Option<bool>, usize) = decode_from_slice(&enc).expect("decode");
        prop_assert_eq!(value, dec);
        prop_assert_eq!(consumed, enc.len());
    });
}

#[test]
fn test_prop_vec_i32_roundtrip() {
    proptest!(|(value in proptest::collection::vec(any::<i32>(), 0..50))| {
        let enc = encode_to_vec(&value).expect("encode");
        let (dec, consumed): (Vec<i32>, usize) = decode_from_slice(&enc).expect("decode");
        prop_assert_eq!(value, dec);
        prop_assert_eq!(consumed, enc.len());
    });
}
