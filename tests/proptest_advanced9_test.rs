//! Advanced property-based roundtrip tests using proptest (set 9).
//!
//! Each test is a top-level #[test] function containing exactly one
//! proptest! macro block, verifying encode/decode roundtrip invariants.

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

#[test]
fn prop_u8_roundtrip() {
    proptest!(|(val: u8)| {
        let enc = encode_to_vec(&val).expect("encode u8");
        let (dec, _): (u8, usize) = decode_from_slice(&enc).expect("decode u8");
        prop_assert_eq!(val, dec);
    });
}

#[test]
fn prop_u16_roundtrip() {
    proptest!(|(val: u16)| {
        let enc = encode_to_vec(&val).expect("encode u16");
        let (dec, _): (u16, usize) = decode_from_slice(&enc).expect("decode u16");
        prop_assert_eq!(val, dec);
    });
}

#[test]
fn prop_u32_roundtrip() {
    proptest!(|(val: u32)| {
        let enc = encode_to_vec(&val).expect("encode u32");
        let (dec, _): (u32, usize) = decode_from_slice(&enc).expect("decode u32");
        prop_assert_eq!(val, dec);
    });
}

#[test]
fn prop_u64_roundtrip() {
    proptest!(|(val: u64)| {
        let enc = encode_to_vec(&val).expect("encode u64");
        let (dec, _): (u64, usize) = decode_from_slice(&enc).expect("decode u64");
        prop_assert_eq!(val, dec);
    });
}

#[test]
fn prop_u128_roundtrip() {
    proptest!(|(val: u128)| {
        let enc = encode_to_vec(&val).expect("encode u128");
        let (dec, _): (u128, usize) = decode_from_slice(&enc).expect("decode u128");
        prop_assert_eq!(val, dec);
    });
}

#[test]
fn prop_i8_roundtrip() {
    proptest!(|(val: i8)| {
        let enc = encode_to_vec(&val).expect("encode i8");
        let (dec, _): (i8, usize) = decode_from_slice(&enc).expect("decode i8");
        prop_assert_eq!(val, dec);
    });
}

#[test]
fn prop_i16_roundtrip() {
    proptest!(|(val: i16)| {
        let enc = encode_to_vec(&val).expect("encode i16");
        let (dec, _): (i16, usize) = decode_from_slice(&enc).expect("decode i16");
        prop_assert_eq!(val, dec);
    });
}

#[test]
fn prop_i32_roundtrip() {
    proptest!(|(val: i32)| {
        let enc = encode_to_vec(&val).expect("encode i32");
        let (dec, _): (i32, usize) = decode_from_slice(&enc).expect("decode i32");
        prop_assert_eq!(val, dec);
    });
}

#[test]
fn prop_i64_roundtrip() {
    proptest!(|(val: i64)| {
        let enc = encode_to_vec(&val).expect("encode i64");
        let (dec, _): (i64, usize) = decode_from_slice(&enc).expect("decode i64");
        prop_assert_eq!(val, dec);
    });
}

#[test]
fn prop_i128_roundtrip() {
    proptest!(|(val: i128)| {
        let enc = encode_to_vec(&val).expect("encode i128");
        let (dec, _): (i128, usize) = decode_from_slice(&enc).expect("decode i128");
        prop_assert_eq!(val, dec);
    });
}

#[test]
fn prop_f32_roundtrip() {
    proptest!(|(val: f32)| {
        let enc = encode_to_vec(&val).expect("encode f32");
        let (dec, _): (f32, usize) = decode_from_slice(&enc).expect("decode f32");
        prop_assert!(val.to_bits() == dec.to_bits(), "f32 roundtrip failed: {:?} != {:?}", val, dec);
    });
}

#[test]
fn prop_f64_roundtrip() {
    proptest!(|(val: f64)| {
        let enc = encode_to_vec(&val).expect("encode f64");
        let (dec, _): (f64, usize) = decode_from_slice(&enc).expect("decode f64");
        prop_assert!(val.to_bits() == dec.to_bits(), "f64 roundtrip failed: {:?} != {:?}", val, dec);
    });
}

#[test]
fn prop_bool_roundtrip() {
    proptest!(|(val: bool)| {
        let enc = encode_to_vec(&val).expect("encode bool");
        let (dec, _): (bool, usize) = decode_from_slice(&enc).expect("decode bool");
        prop_assert_eq!(val, dec);
    });
}

#[test]
fn prop_char_roundtrip() {
    proptest!(|(val: char)| {
        let enc = encode_to_vec(&val).expect("encode char");
        let (dec, _): (char, usize) = decode_from_slice(&enc).expect("decode char");
        prop_assert_eq!(val, dec);
    });
}

#[test]
fn prop_tuple_u32_u32_roundtrip() {
    proptest!(|(val: (u32, u32))| {
        let enc = encode_to_vec(&val).expect("encode (u32, u32)");
        let (dec, _): ((u32, u32), usize) = decode_from_slice(&enc).expect("decode (u32, u32)");
        prop_assert_eq!(val, dec);
    });
}

#[test]
fn prop_tuple_u32_string_roundtrip() {
    proptest!(|(val: (u32, String))| {
        let enc = encode_to_vec(&val).expect("encode (u32, String)");
        let (dec, _): ((u32, String), usize) = decode_from_slice(&enc).expect("decode (u32, String)");
        prop_assert_eq!(val, dec);
    });
}

#[test]
fn prop_vec_u8_roundtrip() {
    proptest!(|(val in proptest::collection::vec(any::<u8>(), 0..100))| {
        let enc = encode_to_vec(&val).expect("encode Vec<u8>");
        let (dec, _): (Vec<u8>, usize) = decode_from_slice(&enc).expect("decode Vec<u8>");
        prop_assert_eq!(val, dec);
    });
}

#[test]
fn prop_vec_u32_roundtrip() {
    proptest!(|(val in proptest::collection::vec(any::<u32>(), 0..50))| {
        let enc = encode_to_vec(&val).expect("encode Vec<u32>");
        let (dec, _): (Vec<u32>, usize) = decode_from_slice(&enc).expect("decode Vec<u32>");
        prop_assert_eq!(val, dec);
    });
}

#[test]
fn prop_option_u32_roundtrip() {
    proptest!(|(val: Option<u32>)| {
        let enc = encode_to_vec(&val).expect("encode Option<u32>");
        let (dec, _): (Option<u32>, usize) = decode_from_slice(&enc).expect("decode Option<u32>");
        prop_assert_eq!(val, dec);
    });
}

#[test]
fn prop_option_string_roundtrip() {
    proptest!(|(val: Option<String>)| {
        let enc = encode_to_vec(&val).expect("encode Option<String>");
        let (dec, _): (Option<String>, usize) = decode_from_slice(&enc).expect("decode Option<String>");
        prop_assert_eq!(val, dec);
    });
}

#[test]
fn prop_string_roundtrip() {
    proptest!(|(val: String)| {
        let enc = encode_to_vec(&val).expect("encode String");
        let (dec, _): (String, usize) = decode_from_slice(&enc).expect("decode String");
        prop_assert_eq!(val, dec);
    });
}

#[test]
fn prop_vec_string_roundtrip() {
    proptest!(|(val in proptest::collection::vec(any::<String>(), 0..20))| {
        let enc = encode_to_vec(&val).expect("encode Vec<String>");
        let (dec, _): (Vec<String>, usize) = decode_from_slice(&enc).expect("decode Vec<String>");
        prop_assert_eq!(val, dec);
    });
}
