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
    encode_to_vec_with_config, Decode, Encode,
};
use proptest::prelude::*;
use std::collections::{BTreeMap, BTreeSet};

#[derive(Encode, Decode, Debug, PartialEq, Clone)]
struct PropPoint {
    x: i32,
    y: i32,
}

#[derive(Encode, Decode, Debug, PartialEq, Clone)]
enum PropColor {
    Red,
    Green,
    Blue,
    Custom(u8, u8, u8),
}

#[test]
fn test_prop_u8_fixed_int_roundtrip() {
    proptest!(|(value: u8)| {
        let cfg = config::standard().with_fixed_int_encoding();
        let enc = encode_to_vec_with_config(&value, cfg).expect("encode");
        let (dec, consumed): (u8, usize) =
            decode_from_slice_with_config(&enc, cfg).expect("decode");
        assert_eq!(value, dec);
        assert_eq!(consumed, enc.len());
    });
}

#[test]
fn test_prop_u16_fixed_int_roundtrip() {
    proptest!(|(value: u16)| {
        let cfg = config::standard().with_fixed_int_encoding();
        let enc = encode_to_vec_with_config(&value, cfg).expect("encode");
        let (dec, consumed): (u16, usize) =
            decode_from_slice_with_config(&enc, cfg).expect("decode");
        assert_eq!(value, dec);
        assert_eq!(consumed, enc.len());
    });
}

#[test]
fn test_prop_u32_big_endian_roundtrip() {
    proptest!(|(value: u32)| {
        let cfg = config::standard().with_big_endian().with_fixed_int_encoding();
        let enc = encode_to_vec_with_config(&value, cfg).expect("encode");
        let (dec, consumed): (u32, usize) =
            decode_from_slice_with_config(&enc, cfg).expect("decode");
        assert_eq!(value, dec);
        assert_eq!(consumed, enc.len());
    });
}

#[test]
fn test_prop_i32_big_endian_roundtrip() {
    proptest!(|(value: i32)| {
        let cfg = config::standard().with_big_endian().with_fixed_int_encoding();
        let enc = encode_to_vec_with_config(&value, cfg).expect("encode");
        let (dec, consumed): (i32, usize) =
            decode_from_slice_with_config(&enc, cfg).expect("decode");
        assert_eq!(value, dec);
        assert_eq!(consumed, enc.len());
    });
}

#[test]
fn test_prop_vec_u8_roundtrip() {
    proptest!(|(value: Vec<u8>)| {
        let enc = encode_to_vec(&value).expect("encode");
        let (dec, consumed): (Vec<u8>, usize) = decode_from_slice(&enc).expect("decode");
        assert_eq!(value, dec);
        assert_eq!(consumed, enc.len());
    });
}

#[test]
fn test_prop_vec_u8_fixed_int_roundtrip() {
    proptest!(|(value: Vec<u8>)| {
        let cfg = config::standard().with_fixed_int_encoding();
        let enc = encode_to_vec_with_config(&value, cfg).expect("encode");
        let (dec, consumed): (Vec<u8>, usize) =
            decode_from_slice_with_config(&enc, cfg).expect("decode");
        assert_eq!(value, dec);
        assert_eq!(consumed, enc.len());
    });
}

#[test]
fn test_prop_btreeset_u32_roundtrip() {
    proptest!(|(value in prop::collection::btree_set(any::<u32>(), 0..10))| {
        let enc = encode_to_vec(&value).expect("encode");
        let (dec, consumed): (BTreeSet<u32>, usize) = decode_from_slice(&enc).expect("decode");
        assert_eq!(value, dec);
        assert_eq!(consumed, enc.len());
    });
}

#[test]
fn test_prop_btreemap_u32_u32_roundtrip() {
    proptest!(|(value in prop::collection::btree_map(any::<u32>(), any::<u32>(), 0..10))| {
        let enc = encode_to_vec(&value).expect("encode");
        let (dec, consumed): (BTreeMap<u32, u32>, usize) = decode_from_slice(&enc).expect("decode");
        assert_eq!(value, dec);
        assert_eq!(consumed, enc.len());
    });
}

#[test]
fn test_prop_option_string_roundtrip() {
    proptest!(|(value: Option<String>)| {
        let enc = encode_to_vec(&value).expect("encode");
        let (dec, consumed): (Option<String>, usize) = decode_from_slice(&enc).expect("decode");
        assert_eq!(value, dec);
        assert_eq!(consumed, enc.len());
    });
}

#[test]
fn test_prop_option_u32_roundtrip() {
    proptest!(|(value: Option<u32>)| {
        let enc = encode_to_vec(&value).expect("encode");
        let (dec, consumed): (Option<u32>, usize) = decode_from_slice(&enc).expect("decode");
        assert_eq!(value, dec);
        assert_eq!(consumed, enc.len());
    });
}

#[test]
fn test_prop_vec_option_u32_roundtrip() {
    proptest!(|(value in prop::collection::vec(any::<Option<u32>>(), 0..5))| {
        let enc = encode_to_vec(&value).expect("encode");
        let (dec, consumed): (Vec<Option<u32>>, usize) = decode_from_slice(&enc).expect("decode");
        assert_eq!(value, dec);
        assert_eq!(consumed, enc.len());
    });
}

#[test]
fn test_prop_tuple_3_roundtrip() {
    proptest!(|(value: (u8, u16, u32))| {
        let enc = encode_to_vec(&value).expect("encode");
        let (dec, consumed): ((u8, u16, u32), usize) = decode_from_slice(&enc).expect("decode");
        assert_eq!(value, dec);
        assert_eq!(consumed, enc.len());
    });
}

#[test]
fn test_prop_tuple_4_roundtrip() {
    proptest!(|(value: (u32, u64, bool, String))| {
        let enc = encode_to_vec(&value).expect("encode");
        let (dec, consumed): ((u32, u64, bool, String), usize) =
            decode_from_slice(&enc).expect("decode");
        assert_eq!(value, dec);
        assert_eq!(consumed, enc.len());
    });
}

#[test]
fn test_prop_prop_point_roundtrip() {
    proptest!(|(value in (any::<i32>(), any::<i32>()).prop_map(|(x, y)| PropPoint { x, y }))| {
        let enc = encode_to_vec(&value).expect("encode");
        let (dec, consumed): (PropPoint, usize) = decode_from_slice(&enc).expect("decode");
        assert_eq!(value, dec);
        assert_eq!(consumed, enc.len());
    });
}

#[test]
fn test_prop_prop_color_red() {
    proptest!(|(_dummy: u8)| {
        let value = PropColor::Red;
        let enc = encode_to_vec(&value).expect("encode");
        let (dec, consumed): (PropColor, usize) = decode_from_slice(&enc).expect("decode");
        assert_eq!(value, dec);
        assert_eq!(consumed, enc.len());
    });
}

#[test]
fn test_prop_encoded_size_matches_vec_len_u32() {
    proptest!(|(value: u32)| {
        let size = oxicode::encoded_size(&value).expect("encoded_size");
        let enc = encode_to_vec(&value).expect("encode");
        assert_eq!(size, enc.len());
    });
}

#[test]
fn test_prop_encoded_size_matches_vec_len_u64() {
    proptest!(|(value: u64)| {
        let size = oxicode::encoded_size(&value).expect("encoded_size");
        let enc = encode_to_vec(&value).expect("encode");
        assert_eq!(size, enc.len());
    });
}

#[test]
fn test_prop_encode_decode_idempotent_u64() {
    proptest!(|(value: u64)| {
        let enc1 = encode_to_vec(&value).expect("encode1");
        let (dec, _): (u64, usize) = decode_from_slice(&enc1).expect("decode");
        let enc2 = encode_to_vec(&dec).expect("encode2");
        assert_eq!(enc1, enc2);
    });
}

#[test]
fn test_prop_encode_decode_idempotent_string() {
    proptest!(|(value: String)| {
        let enc1 = encode_to_vec(&value).expect("encode1");
        let (dec, _): (String, usize) = decode_from_slice(&enc1).expect("decode");
        let enc2 = encode_to_vec(&dec).expect("encode2");
        assert_eq!(enc1, enc2);
    });
}

#[test]
fn test_prop_vec_bool_roundtrip() {
    proptest!(|(value: Vec<bool>)| {
        let enc = encode_to_vec(&value).expect("encode");
        let (dec, consumed): (Vec<bool>, usize) = decode_from_slice(&enc).expect("decode");
        assert_eq!(value, dec);
        assert_eq!(consumed, enc.len());
    });
}

#[test]
fn test_prop_vec_i64_roundtrip() {
    proptest!(|(value: Vec<i64>)| {
        let enc = encode_to_vec(&value).expect("encode");
        let (dec, consumed): (Vec<i64>, usize) = decode_from_slice(&enc).expect("decode");
        assert_eq!(value, dec);
        assert_eq!(consumed, enc.len());
    });
}

#[test]
fn test_prop_btreemap_string_u64_roundtrip() {
    proptest!(|(value in prop::collection::btree_map(any::<String>(), any::<u64>(), 0..10))| {
        let enc = encode_to_vec(&value).expect("encode");
        let (dec, consumed): (BTreeMap<String, u64>, usize) = decode_from_slice(&enc).expect("decode");
        assert_eq!(value, dec);
        assert_eq!(consumed, enc.len());
    });
}
