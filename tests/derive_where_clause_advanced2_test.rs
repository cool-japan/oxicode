//! Advanced tests for OxiCode derive macro with where clauses and generic bounds

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
use oxicode::{decode_from_slice, encode_to_vec, Decode, Encode};

#[derive(Encode, Decode, Debug, PartialEq)]
struct Wrapper<T: Encode + Decode> {
    value: T,
    count: u32,
}

#[derive(Encode, Decode, Debug, PartialEq)]
struct Pair<A: Encode + Decode, B: Encode + Decode> {
    first: A,
    second: B,
}

#[derive(Encode, Decode, Debug, PartialEq)]
struct Triple<A: Encode + Decode, B: Encode + Decode, C: Encode + Decode> {
    a: A,
    b: B,
    c: C,
}

#[derive(Encode, Decode, Debug, PartialEq)]
enum Container<T: Encode + Decode> {
    Empty,
    Single(T),
    Double(T, T),
}

#[test]
fn test_wrapper_u32_roundtrip() {
    let original = Wrapper {
        value: 42u32,
        count: 1,
    };
    let enc = encode_to_vec(&original).expect("encode Wrapper<u32> failed");
    let (val, _): (Wrapper<u32>, usize) =
        decode_from_slice(&enc).expect("decode Wrapper<u32> failed");
    assert_eq!(original, val);
}

#[test]
fn test_wrapper_string_roundtrip() {
    let original = Wrapper {
        value: "hello".to_string(),
        count: 5,
    };
    let enc = encode_to_vec(&original).expect("encode Wrapper<String> failed");
    let (val, _): (Wrapper<String>, usize) =
        decode_from_slice(&enc).expect("decode Wrapper<String> failed");
    assert_eq!(original, val);
}

#[test]
fn test_wrapper_vec_u8_roundtrip() {
    let original = Wrapper {
        value: vec![1u8, 2, 3],
        count: 3,
    };
    let enc = encode_to_vec(&original).expect("encode Wrapper<Vec<u8>> failed");
    let (val, _): (Wrapper<Vec<u8>>, usize) =
        decode_from_slice(&enc).expect("decode Wrapper<Vec<u8>> failed");
    assert_eq!(original, val);
}

#[test]
fn test_wrapper_bool_roundtrip() {
    let original = Wrapper {
        value: true,
        count: 0,
    };
    let enc = encode_to_vec(&original).expect("encode Wrapper<bool> failed");
    let (val, _): (Wrapper<bool>, usize) =
        decode_from_slice(&enc).expect("decode Wrapper<bool> failed");
    assert_eq!(original, val);
}

#[test]
fn test_pair_u32_string_roundtrip() {
    let original = Pair {
        first: 1u32,
        second: "world".to_string(),
    };
    let enc = encode_to_vec(&original).expect("encode Pair<u32, String> failed");
    let (val, _): (Pair<u32, String>, usize) =
        decode_from_slice(&enc).expect("decode Pair<u32, String> failed");
    assert_eq!(original, val);
}

#[test]
fn test_pair_u64_u64_roundtrip() {
    let original = Pair {
        first: 100u64,
        second: 200u64,
    };
    let enc = encode_to_vec(&original).expect("encode Pair<u64, u64> failed");
    let (val, _): (Pair<u64, u64>, usize) =
        decode_from_slice(&enc).expect("decode Pair<u64, u64> failed");
    assert_eq!(original, val);
}

#[test]
fn test_pair_string_vec_roundtrip() {
    let original = Pair {
        first: "key".to_string(),
        second: vec![1u32, 2, 3],
    };
    let enc = encode_to_vec(&original).expect("encode Pair<String, Vec<u32>> failed");
    let (val, _): (Pair<String, Vec<u32>>, usize) =
        decode_from_slice(&enc).expect("decode Pair<String, Vec<u32>> failed");
    assert_eq!(original, val);
}

#[test]
fn test_triple_u8_u16_u32_roundtrip() {
    let original = Triple {
        a: 1u8,
        b: 2u16,
        c: 3u32,
    };
    let enc = encode_to_vec(&original).expect("encode Triple<u8, u16, u32> failed");
    let (val, _): (Triple<u8, u16, u32>, usize) =
        decode_from_slice(&enc).expect("decode Triple<u8, u16, u32> failed");
    assert_eq!(original, val);
}

#[test]
fn test_triple_strings_roundtrip() {
    let original = Triple {
        a: "alpha".to_string(),
        b: "beta".to_string(),
        c: "gamma".to_string(),
    };
    let enc = encode_to_vec(&original).expect("encode Triple<String, String, String> failed");
    let (val, _): (Triple<String, String, String>, usize) =
        decode_from_slice(&enc).expect("decode Triple<String, String, String> failed");
    assert_eq!(original, val);
}

#[test]
fn test_container_empty_roundtrip() {
    let original: Container<u32> = Container::Empty;
    let enc = encode_to_vec(&original).expect("encode Container::Empty failed");
    let (val, _): (Container<u32>, usize) =
        decode_from_slice(&enc).expect("decode Container::Empty failed");
    assert_eq!(original, val);
}

#[test]
fn test_container_single_roundtrip() {
    let original = Container::Single(42u32);
    let enc = encode_to_vec(&original).expect("encode Container::Single failed");
    let (val, _): (Container<u32>, usize) =
        decode_from_slice(&enc).expect("decode Container::Single failed");
    assert_eq!(original, val);
}

#[test]
fn test_container_double_roundtrip() {
    let original = Container::Double(1u32, 2u32);
    let enc = encode_to_vec(&original).expect("encode Container::Double failed");
    let (val, _): (Container<u32>, usize) =
        decode_from_slice(&enc).expect("decode Container::Double failed");
    assert_eq!(original, val);
}

#[test]
fn test_container_string_single_roundtrip() {
    let original = Container::Single("hello".to_string());
    let enc = encode_to_vec(&original).expect("encode Container::Single<String> failed");
    let (val, _): (Container<String>, usize) =
        decode_from_slice(&enc).expect("decode Container::Single<String> failed");
    assert_eq!(original, val);
}

#[test]
fn test_vec_wrapper_u32_roundtrip() {
    let original = vec![
        Wrapper {
            value: 10u32,
            count: 1,
        },
        Wrapper {
            value: 20u32,
            count: 2,
        },
    ];
    let enc = encode_to_vec(&original).expect("encode Vec<Wrapper<u32>> failed");
    let (val, _): (Vec<Wrapper<u32>>, usize) =
        decode_from_slice(&enc).expect("decode Vec<Wrapper<u32>> failed");
    assert_eq!(original, val);
}

#[test]
fn test_option_wrapper_some_roundtrip() {
    let original: Option<Wrapper<u32>> = Some(Wrapper {
        value: 99u32,
        count: 7,
    });
    let enc = encode_to_vec(&original).expect("encode Option<Wrapper<u32>> Some failed");
    let (val, _): (Option<Wrapper<u32>>, usize) =
        decode_from_slice(&enc).expect("decode Option<Wrapper<u32>> Some failed");
    assert_eq!(original, val);
}

#[test]
fn test_option_wrapper_none_roundtrip() {
    let original: Option<Wrapper<u32>> = None;
    let enc = encode_to_vec(&original).expect("encode Option<Wrapper<u32>> None failed");
    let (val, _): (Option<Wrapper<u32>>, usize) =
        decode_from_slice(&enc).expect("decode Option<Wrapper<u32>> None failed");
    assert_eq!(original, val);
}

#[test]
fn test_wrapper_consumed_equals_len() {
    let original = Wrapper {
        value: 77u32,
        count: 3,
    };
    let enc = encode_to_vec(&original).expect("encode Wrapper<u32> failed");
    let (_, consumed): (Wrapper<u32>, usize) =
        decode_from_slice(&enc).expect("decode Wrapper<u32> failed");
    assert_eq!(consumed, enc.len());
}

#[test]
fn test_pair_consumed_equals_len() {
    let original = Pair {
        first: 55u32,
        second: "data".to_string(),
    };
    let enc = encode_to_vec(&original).expect("encode Pair<u32, String> failed");
    let (_, consumed): (Pair<u32, String>, usize) =
        decode_from_slice(&enc).expect("decode Pair<u32, String> failed");
    assert_eq!(consumed, enc.len());
}

#[test]
fn test_container_empty_smaller_than_single() {
    let empty: Container<u32> = Container::Empty;
    let single = Container::Single(42u32);
    let enc_empty = encode_to_vec(&empty).expect("encode Container::Empty failed");
    let enc_single = encode_to_vec(&single).expect("encode Container::Single failed");
    assert!(enc_empty.len() < enc_single.len());
}

#[test]
fn test_wrapper_u64_max_roundtrip() {
    let original = Wrapper {
        value: u64::MAX,
        count: u32::MAX,
    };
    let enc = encode_to_vec(&original).expect("encode Wrapper<u64::MAX> failed");
    let (val, _): (Wrapper<u64>, usize) =
        decode_from_slice(&enc).expect("decode Wrapper<u64::MAX> failed");
    assert_eq!(original, val);
}

#[test]
fn test_pair_nested_wrapper_roundtrip() {
    let original = Pair {
        first: Wrapper {
            value: 100u32,
            count: 10,
        },
        second: Wrapper {
            value: "nested".to_string(),
            count: 6,
        },
    };
    let enc = encode_to_vec(&original).expect("encode Pair<Wrapper<u32>, Wrapper<String>> failed");
    let (val, _): (Pair<Wrapper<u32>, Wrapper<String>>, usize) =
        decode_from_slice(&enc).expect("decode Pair<Wrapper<u32>, Wrapper<String>> failed");
    assert_eq!(original, val);
}

#[test]
fn test_triple_all_vecs_roundtrip() {
    let original = Triple {
        a: vec![1u8, 2, 3],
        b: vec![10u16, 20],
        c: vec![100u32],
    };
    let enc = encode_to_vec(&original).expect("encode Triple<Vec<u8>, Vec<u16>, Vec<u32>> failed");
    let (val, _): (Triple<Vec<u8>, Vec<u16>, Vec<u32>>, usize) =
        decode_from_slice(&enc).expect("decode Triple<Vec<u8>, Vec<u16>, Vec<u32>> failed");
    assert_eq!(original, val);
}
