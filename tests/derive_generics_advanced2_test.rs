//! Advanced tests for derive macros with generic types

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

#[derive(Debug, PartialEq, Encode, Decode)]
struct Wrapper<T> {
    value: T,
}

#[derive(Debug, PartialEq, Encode, Decode)]
struct Pair<A, B> {
    first: A,
    second: B,
}

#[derive(Debug, PartialEq, Encode, Decode)]
struct Container<T> {
    items: Vec<T>,
    count: usize,
}

#[derive(Debug, PartialEq, Encode, Decode)]
enum Either<L, R> {
    Left(L),
    Right(R),
}

#[derive(Debug, PartialEq, Encode, Decode)]
struct Nested<T> {
    inner: Wrapper<T>,
    tag: u8,
}

#[test]
fn test_wrapper_u32_roundtrip() {
    let val = Wrapper { value: 42u32 };
    let bytes = encode_to_vec(&val).expect("encode Wrapper<u32>");
    let (decoded, _): (Wrapper<u32>, usize) =
        decode_from_slice(&bytes).expect("decode Wrapper<u32>");
    assert_eq!(val, decoded);
}

#[test]
fn test_wrapper_string_roundtrip() {
    let val = Wrapper {
        value: String::from("hello oxicode"),
    };
    let bytes = encode_to_vec(&val).expect("encode Wrapper<String>");
    let (decoded, _): (Wrapper<String>, usize) =
        decode_from_slice(&bytes).expect("decode Wrapper<String>");
    assert_eq!(val, decoded);
}

#[test]
fn test_wrapper_vec_u8_roundtrip() {
    let val = Wrapper {
        value: vec![1u8, 2, 3, 4, 5],
    };
    let bytes = encode_to_vec(&val).expect("encode Wrapper<Vec<u8>>");
    let (decoded, _): (Wrapper<Vec<u8>>, usize) =
        decode_from_slice(&bytes).expect("decode Wrapper<Vec<u8>>");
    assert_eq!(val, decoded);
}

#[test]
fn test_pair_u32_string_roundtrip() {
    let val = Pair {
        first: 100u32,
        second: String::from("pair test"),
    };
    let bytes = encode_to_vec(&val).expect("encode Pair<u32, String>");
    let (decoded, _): (Pair<u32, String>, usize) =
        decode_from_slice(&bytes).expect("decode Pair<u32, String>");
    assert_eq!(val, decoded);
}

#[test]
fn test_pair_string_vec_u8_roundtrip() {
    let val = Pair {
        first: String::from("key"),
        second: vec![10u8, 20, 30],
    };
    let bytes = encode_to_vec(&val).expect("encode Pair<String, Vec<u8>>");
    let (decoded, _): (Pair<String, Vec<u8>>, usize) =
        decode_from_slice(&bytes).expect("decode Pair<String, Vec<u8>>");
    assert_eq!(val, decoded);
}

#[test]
fn test_pair_bool_u64_roundtrip() {
    let val = Pair {
        first: true,
        second: 9_999_999_999u64,
    };
    let bytes = encode_to_vec(&val).expect("encode Pair<bool, u64>");
    let (decoded, _): (Pair<bool, u64>, usize) =
        decode_from_slice(&bytes).expect("decode Pair<bool, u64>");
    assert_eq!(val, decoded);
}

#[test]
fn test_container_u32_roundtrip() {
    let val = Container {
        items: vec![1u32, 2, 3],
        count: 3,
    };
    let bytes = encode_to_vec(&val).expect("encode Container<u32>");
    let (decoded, _): (Container<u32>, usize) =
        decode_from_slice(&bytes).expect("decode Container<u32>");
    assert_eq!(val, decoded);
}

#[test]
fn test_container_string_roundtrip() {
    let val = Container {
        items: vec![
            String::from("alpha"),
            String::from("beta"),
            String::from("gamma"),
        ],
        count: 3,
    };
    let bytes = encode_to_vec(&val).expect("encode Container<String>");
    let (decoded, _): (Container<String>, usize) =
        decode_from_slice(&bytes).expect("decode Container<String>");
    assert_eq!(val, decoded);
}

#[test]
fn test_container_vec_u8_roundtrip() {
    let val = Container {
        items: vec![vec![1u8, 2], vec![3u8, 4, 5], vec![6u8]],
        count: 3,
    };
    let bytes = encode_to_vec(&val).expect("encode Container<Vec<u8>>");
    let (decoded, _): (Container<Vec<u8>>, usize) =
        decode_from_slice(&bytes).expect("decode Container<Vec<u8>>");
    assert_eq!(val, decoded);
}

#[test]
fn test_either_left_u32_string_roundtrip() {
    let val: Either<u32, String> = Either::Left(77u32);
    let bytes = encode_to_vec(&val).expect("encode Either::Left<u32, String>");
    let (decoded, _): (Either<u32, String>, usize) =
        decode_from_slice(&bytes).expect("decode Either::Left<u32, String>");
    assert_eq!(val, decoded);
}

#[test]
fn test_either_right_u32_string_roundtrip() {
    let val: Either<u32, String> = Either::Right(String::from("right side"));
    let bytes = encode_to_vec(&val).expect("encode Either::Right<u32, String>");
    let (decoded, _): (Either<u32, String>, usize) =
        decode_from_slice(&bytes).expect("decode Either::Right<u32, String>");
    assert_eq!(val, decoded);
}

#[test]
fn test_either_left_vec_u8_u64_roundtrip() {
    let val: Either<Vec<u8>, u64> = Either::Left(vec![0xde, 0xad, 0xbe, 0xef]);
    let bytes = encode_to_vec(&val).expect("encode Either::Left<Vec<u8>, u64>");
    let (decoded, _): (Either<Vec<u8>, u64>, usize) =
        decode_from_slice(&bytes).expect("decode Either::Left<Vec<u8>, u64>");
    assert_eq!(val, decoded);
}

#[test]
fn test_nested_u32_roundtrip() {
    let val = Nested {
        inner: Wrapper { value: 255u32 },
        tag: 7u8,
    };
    let bytes = encode_to_vec(&val).expect("encode Nested<u32>");
    let (decoded, _): (Nested<u32>, usize) = decode_from_slice(&bytes).expect("decode Nested<u32>");
    assert_eq!(val, decoded);
}

#[test]
fn test_nested_string_roundtrip() {
    let val = Nested {
        inner: Wrapper {
            value: String::from("nested string"),
        },
        tag: 42u8,
    };
    let bytes = encode_to_vec(&val).expect("encode Nested<String>");
    let (decoded, _): (Nested<String>, usize) =
        decode_from_slice(&bytes).expect("decode Nested<String>");
    assert_eq!(val, decoded);
}

#[test]
fn test_wrapper_of_wrapper_u32_roundtrip() {
    let val = Wrapper {
        value: Wrapper { value: 1234u32 },
    };
    let bytes = encode_to_vec(&val).expect("encode Wrapper<Wrapper<u32>>");
    let (decoded, _): (Wrapper<Wrapper<u32>>, usize) =
        decode_from_slice(&bytes).expect("decode Wrapper<Wrapper<u32>>");
    assert_eq!(val, decoded);
}

#[test]
fn test_pair_of_wrappers_roundtrip() {
    let val = Pair {
        first: Wrapper { value: 99u32 },
        second: Wrapper {
            value: String::from("wrapped string"),
        },
    };
    let bytes = encode_to_vec(&val).expect("encode Pair<Wrapper<u32>, Wrapper<String>>");
    let (decoded, _): (Pair<Wrapper<u32>, Wrapper<String>>, usize) =
        decode_from_slice(&bytes).expect("decode Pair<Wrapper<u32>, Wrapper<String>>");
    assert_eq!(val, decoded);
}

#[test]
fn test_container_of_pairs_roundtrip() {
    let val = Container {
        items: vec![
            Pair {
                first: 1u32,
                second: String::from("one"),
            },
            Pair {
                first: 2u32,
                second: String::from("two"),
            },
            Pair {
                first: 3u32,
                second: String::from("three"),
            },
        ],
        count: 3,
    };
    let bytes = encode_to_vec(&val).expect("encode Container<Pair<u32, String>>");
    let (decoded, _): (Container<Pair<u32, String>>, usize) =
        decode_from_slice(&bytes).expect("decode Container<Pair<u32, String>>");
    assert_eq!(val, decoded);
}

#[test]
fn test_wrapper_u32_with_fixed_int_config_roundtrip() {
    let val = Wrapper { value: 500u32 };
    let cfg = config::standard().with_fixed_int_encoding();
    let bytes =
        encode_to_vec_with_config(&val, cfg).expect("encode Wrapper<u32> with fixed_int config");
    let (decoded, _): (Wrapper<u32>, usize) = decode_from_slice_with_config(&bytes, cfg)
        .expect("decode Wrapper<u32> with fixed_int config");
    assert_eq!(val, decoded);
}

#[test]
fn test_consumed_bytes_equals_encoded_length_pair_u32_string() {
    let val = Pair {
        first: 42u32,
        second: String::from("length check"),
    };
    let bytes = encode_to_vec(&val).expect("encode Pair<u32, String> for length check");
    let (_, consumed): (Pair<u32, String>, usize) =
        decode_from_slice(&bytes).expect("decode Pair<u32, String> for length check");
    assert_eq!(
        consumed,
        bytes.len(),
        "consumed bytes must equal encoded length"
    );
}

#[test]
fn test_vec_of_wrappers_u32_roundtrip() {
    let val: Vec<Wrapper<u32>> = vec![
        Wrapper { value: 10u32 },
        Wrapper { value: 20u32 },
        Wrapper { value: 30u32 },
    ];
    let bytes = encode_to_vec(&val).expect("encode Vec<Wrapper<u32>>");
    let (decoded, _): (Vec<Wrapper<u32>>, usize) =
        decode_from_slice(&bytes).expect("decode Vec<Wrapper<u32>>");
    assert_eq!(val, decoded);
}

#[test]
fn test_option_pair_u32_string_some_roundtrip() {
    let val: Option<Pair<u32, String>> = Some(Pair {
        first: 7u32,
        second: String::from("some value"),
    });
    let bytes = encode_to_vec(&val).expect("encode Option<Pair<u32, String>> Some");
    let (decoded, _): (Option<Pair<u32, String>>, usize) =
        decode_from_slice(&bytes).expect("decode Option<Pair<u32, String>> Some");
    assert_eq!(val, decoded);
}

#[test]
fn test_option_pair_u32_string_none_roundtrip() {
    let val: Option<Pair<u32, String>> = None;
    let bytes = encode_to_vec(&val).expect("encode Option<Pair<u32, String>> None");
    let (decoded, _): (Option<Pair<u32, String>>, usize) =
        decode_from_slice(&bytes).expect("decode Option<Pair<u32, String>> None");
    assert_eq!(val, decoded);
}
