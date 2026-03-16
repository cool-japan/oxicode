//! Tests for the `#[oxicode(seq_len = "...")]` field attribute.

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
use oxicode::{Decode, Encode};

#[derive(Debug, PartialEq, Encode, Decode)]
struct Compact {
    #[oxicode(seq_len = "u8")] // at most 255 elements
    tags: Vec<String>,
    #[oxicode(seq_len = "u16")] // at most 65535 elements
    values: Vec<u32>,
}

#[test]
fn test_seq_len_u8_roundtrip() {
    let v = Compact {
        tags: vec!["rust".into(), "fast".into()],
        values: vec![1, 2, 3, 4, 5],
    };
    let enc = oxicode::encode_to_vec(&v).expect("encode");
    let (dec, _): (Compact, _) = oxicode::decode_from_slice(&enc).expect("decode");
    assert_eq!(v, dec);
}

#[test]
fn test_seq_len_u8_compact() {
    // A Vec<u8> with 3 elements and a u8 length prefix should occupy 4 bytes total:
    // 1 byte for the u8 length (= 3) followed by 3 bytes for the element values.
    #[derive(Debug, PartialEq, Encode, Decode)]
    struct S {
        #[oxicode(seq_len = "u8")]
        items: Vec<u8>,
    }
    let s = S {
        items: vec![10, 20, 30],
    };
    let enc = oxicode::encode_to_vec(&s).expect("encode");
    // First byte must be the raw u8 length = 3 (not a varint-extended form)
    assert_eq!(enc[0], 3u8, "length prefix should be u8 value 3");
    let (dec, _): (S, _) = oxicode::decode_from_slice(&enc).expect("decode");
    assert_eq!(s, dec);
}

#[test]
fn test_seq_len_empty_vec() {
    #[derive(Debug, PartialEq, Encode, Decode)]
    struct S {
        #[oxicode(seq_len = "u8")]
        items: Vec<u32>,
    }
    let s = S { items: vec![] };
    let enc = oxicode::encode_to_vec(&s).expect("encode");
    let (dec, _): (S, _) = oxicode::decode_from_slice(&enc).expect("decode");
    assert_eq!(s, dec);
}

#[test]
fn test_seq_len_u16_roundtrip() {
    #[derive(Debug, PartialEq, Encode, Decode)]
    struct S {
        #[oxicode(seq_len = "u16")]
        data: Vec<i64>,
    }
    let s = S {
        data: (0_i64..100).collect(),
    };
    let enc = oxicode::encode_to_vec(&s).expect("encode");
    let (dec, _): (S, _) = oxicode::decode_from_slice(&enc).expect("decode");
    assert_eq!(s, dec);
}

#[test]
fn test_seq_len_u32_roundtrip() {
    #[derive(Debug, PartialEq, Encode, Decode)]
    struct S {
        #[oxicode(seq_len = "u32")]
        data: Vec<u8>,
    }
    let s = S {
        data: vec![0xDE, 0xAD, 0xBE, 0xEF],
    };
    let enc = oxicode::encode_to_vec(&s).expect("encode");
    let (dec, _): (S, _) = oxicode::decode_from_slice(&enc).expect("decode");
    assert_eq!(s, dec);
}

#[test]
fn test_seq_len_mixed_with_normal_fields() {
    #[derive(Debug, PartialEq, Encode, Decode)]
    struct Mixed {
        id: u32,
        #[oxicode(seq_len = "u8")]
        tags: Vec<String>,
        score: f64,
    }
    let m = Mixed {
        id: 42,
        tags: vec!["alpha".into(), "beta".into(), "gamma".into()],
        score: 9.87,
    };
    let enc = oxicode::encode_to_vec(&m).expect("encode");
    let (dec, _): (Mixed, _) = oxicode::decode_from_slice(&enc).expect("decode");
    assert_eq!(m, dec);
}
