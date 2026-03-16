//! Tests for string/bytes encoding edge cases.

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

#[test]
fn test_empty_string() {
    let s = String::new();
    let enc = encode_to_vec(&s).expect("encode");
    let (dec, _): (String, _) = decode_from_slice(&enc).expect("decode");
    assert_eq!(s, dec);
}

#[test]
fn test_unicode_string() {
    let s = "Hello, 世界! 🦀 αβγ ñ ü".to_string();
    let enc = encode_to_vec(&s).expect("encode");
    let (dec, _): (String, _) = decode_from_slice(&enc).expect("decode");
    assert_eq!(s, dec);
}

#[test]
fn test_long_string() {
    let s = "x".repeat(65536);
    let enc = encode_to_vec(&s).expect("encode");
    let (dec, _): (String, _) = decode_from_slice(&enc).expect("decode");
    assert_eq!(s, dec);
}

#[test]
fn test_string_with_null_bytes() {
    let s = "hello\0world\0".to_string();
    let enc = encode_to_vec(&s).expect("encode");
    let (dec, _): (String, _) = decode_from_slice(&enc).expect("decode");
    assert_eq!(s, dec);
}

#[test]
fn test_str_ref_roundtrip() {
    let s: &str = "hello, world";
    let enc = encode_to_vec(&s).expect("encode");
    // &str encodes the same as String
    let enc2 = encode_to_vec(&s.to_string()).expect("encode String");
    assert_eq!(enc, enc2, "str and String should encode identically");
}

#[test]
fn test_string_decode_from_str_bytes() {
    // Encoding a &str and decoding as String should work
    let s = "oxicode is great!";
    let enc = encode_to_vec(&s).expect("encode");
    let (dec, _): (String, _) = decode_from_slice(&enc).expect("decode as String");
    assert_eq!(s, dec.as_str());
}

#[test]
fn test_bytes_vec_u8_roundtrip() {
    let data: Vec<u8> = (0u8..=255).collect();
    let enc = encode_to_vec(&data).expect("encode");
    let (dec, _): (Vec<u8>, _) = decode_from_slice(&enc).expect("decode");
    assert_eq!(data, dec);
}

#[test]
fn test_bytes_field_compact() {
    // A type using #[oxicode(bytes)] should encode Vec<u8> compactly
    use oxicode::{Decode, Encode};

    #[derive(Debug, PartialEq, Encode, Decode)]
    struct WithBytes {
        #[oxicode(bytes)]
        data: Vec<u8>,
    }

    let v = WithBytes {
        data: vec![1, 2, 3, 4, 5],
    };
    let enc = encode_to_vec(&v).expect("encode");
    let (dec, _): (WithBytes, _) = decode_from_slice(&enc).expect("decode");
    assert_eq!(v, dec);
}
