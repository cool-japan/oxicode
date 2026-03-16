//! Advanced generic struct encoding tests for OxiCode

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
use std::collections::BTreeMap;

#[derive(Encode, Decode, Debug, PartialEq, Clone)]
struct Container<T> {
    value: T,
    count: u32,
}

#[derive(Encode, Decode, Debug, PartialEq, Clone)]
struct Pair<A, B> {
    first: A,
    second: B,
}

#[derive(Encode, Decode, Debug, PartialEq, Clone)]
struct Triple<A, B, C> {
    a: A,
    b: B,
    c: C,
}

#[derive(Encode, Decode, Debug, PartialEq, Clone)]
struct Wrapper<T>(T);

#[derive(Encode, Decode, Debug, PartialEq, Clone)]
enum Either<L, R> {
    Left(L),
    Right(R),
}

#[test]
fn test_container_u32_roundtrip() {
    let original = Container {
        value: 42u32,
        count: 7,
    };
    let encoded = encode_to_vec(&original).expect("encode failed");
    let (decoded, _bytes): (Container<u32>, usize) =
        decode_from_slice(&encoded).expect("decode failed");
    assert_eq!(original, decoded);
}

#[test]
fn test_container_string_roundtrip() {
    let original = Container {
        value: "hello oxicode".to_string(),
        count: 3,
    };
    let encoded = encode_to_vec(&original).expect("encode failed");
    let (decoded, _bytes): (Container<String>, usize) =
        decode_from_slice(&encoded).expect("decode failed");
    assert_eq!(original, decoded);
}

#[test]
fn test_container_vec_u8_roundtrip() {
    let original = Container {
        value: vec![1u8, 2, 3, 4, 5],
        count: 5,
    };
    let encoded = encode_to_vec(&original).expect("encode failed");
    let (decoded, _bytes): (Container<Vec<u8>>, usize) =
        decode_from_slice(&encoded).expect("decode failed");
    assert_eq!(original, decoded);
}

#[test]
fn test_pair_u32_u32_roundtrip() {
    let original = Pair {
        first: 100u32,
        second: 200u32,
    };
    let encoded = encode_to_vec(&original).expect("encode failed");
    let (decoded, _bytes): (Pair<u32, u32>, usize) =
        decode_from_slice(&encoded).expect("decode failed");
    assert_eq!(original, decoded);
}

#[test]
fn test_pair_string_vec_u8_roundtrip() {
    let original = Pair {
        first: "key".to_string(),
        second: vec![10u8, 20, 30],
    };
    let encoded = encode_to_vec(&original).expect("encode failed");
    let (decoded, _bytes): (Pair<String, Vec<u8>>, usize) =
        decode_from_slice(&encoded).expect("decode failed");
    assert_eq!(original, decoded);
}

#[test]
fn test_triple_u32_string_bool_roundtrip() {
    let original = Triple {
        a: 99u32,
        b: "triple".to_string(),
        c: true,
    };
    let encoded = encode_to_vec(&original).expect("encode failed");
    let (decoded, _bytes): (Triple<u32, String, bool>, usize) =
        decode_from_slice(&encoded).expect("decode failed");
    assert_eq!(original, decoded);
}

#[test]
fn test_wrapper_u32_roundtrip() {
    let original = Wrapper(12345u32);
    let encoded = encode_to_vec(&original).expect("encode failed");
    let (decoded, _bytes): (Wrapper<u32>, usize) =
        decode_from_slice(&encoded).expect("decode failed");
    assert_eq!(original, decoded);
}

#[test]
fn test_wrapper_string_roundtrip() {
    let original = Wrapper("wrapped string".to_string());
    let encoded = encode_to_vec(&original).expect("encode failed");
    let (decoded, _bytes): (Wrapper<String>, usize) =
        decode_from_slice(&encoded).expect("decode failed");
    assert_eq!(original, decoded);
}

#[test]
fn test_either_left_roundtrip() {
    let original: Either<u32, String> = Either::Left(42u32);
    let encoded = encode_to_vec(&original).expect("encode failed");
    let (decoded, _bytes): (Either<u32, String>, usize) =
        decode_from_slice(&encoded).expect("decode failed");
    assert_eq!(original, decoded);
}

#[test]
fn test_either_right_roundtrip() {
    let original: Either<u32, String> = Either::Right("right value".to_string());
    let encoded = encode_to_vec(&original).expect("encode failed");
    let (decoded, _bytes): (Either<u32, String>, usize) =
        decode_from_slice(&encoded).expect("decode failed");
    assert_eq!(original, decoded);
}

#[test]
fn test_vec_container_u32_roundtrip() {
    let original = vec![
        Container {
            value: 1u32,
            count: 1,
        },
        Container {
            value: 2u32,
            count: 2,
        },
        Container {
            value: 3u32,
            count: 3,
        },
    ];
    let encoded = encode_to_vec(&original).expect("encode failed");
    let (decoded, _bytes): (Vec<Container<u32>>, usize) =
        decode_from_slice(&encoded).expect("decode failed");
    assert_eq!(original, decoded);
}

#[test]
fn test_option_pair_some_roundtrip() {
    let original: Option<Pair<u32, u32>> = Some(Pair {
        first: 10u32,
        second: 20u32,
    });
    let encoded = encode_to_vec(&original).expect("encode failed");
    let (decoded, _bytes): (Option<Pair<u32, u32>>, usize) =
        decode_from_slice(&encoded).expect("decode failed");
    assert_eq!(original, decoded);
}

#[test]
fn test_container_u32_fixed_int_config_roundtrip() {
    let original = Container {
        value: 255u32,
        count: 8,
    };
    let cfg = config::standard().with_fixed_int_encoding();
    let encoded = encode_to_vec_with_config(&original, cfg).expect("encode failed");
    let (decoded, _bytes): (Container<u32>, usize) =
        decode_from_slice_with_config(&encoded, cfg).expect("decode failed");
    assert_eq!(original, decoded);
}

#[test]
fn test_pair_u32_u64_big_endian_config_roundtrip() {
    let original = Pair {
        first: 1000u32,
        second: 9_999_999_999u64,
    };
    let cfg = config::standard().with_big_endian();
    let encoded = encode_to_vec_with_config(&original, cfg).expect("encode failed");
    let (decoded, _bytes): (Pair<u32, u64>, usize) =
        decode_from_slice_with_config(&encoded, cfg).expect("decode failed");
    assert_eq!(original, decoded);
}

#[test]
fn test_nested_container_container_u32_roundtrip() {
    let original = Container {
        value: Container {
            value: 77u32,
            count: 2,
        },
        count: 1,
    };
    let encoded = encode_to_vec(&original).expect("encode failed");
    let (decoded, _bytes): (Container<Container<u32>>, usize) =
        decode_from_slice(&encoded).expect("decode failed");
    assert_eq!(original, decoded);
}

#[test]
fn test_container_option_string_roundtrip() {
    let original = Container {
        value: Some("optional string".to_string()),
        count: 4,
    };
    let encoded = encode_to_vec(&original).expect("encode failed");
    let (decoded, _bytes): (Container<Option<String>>, usize) =
        decode_from_slice(&encoded).expect("decode failed");
    assert_eq!(original, decoded);
}

#[test]
fn test_pair_vec_u8_vec_u32_roundtrip() {
    let original = Pair {
        first: vec![0u8, 1, 2, 3],
        second: vec![100u32, 200, 300],
    };
    let encoded = encode_to_vec(&original).expect("encode failed");
    let (decoded, _bytes): (Pair<Vec<u8>, Vec<u32>>, usize) =
        decode_from_slice(&encoded).expect("decode failed");
    assert_eq!(original, decoded);
}

#[test]
fn test_container_u32_consumed_equals_encoded_length() {
    let original = Container {
        value: 42u32,
        count: 1,
    };
    let encoded = encode_to_vec(&original).expect("encode failed");
    let (_decoded, consumed): (Container<u32>, usize) =
        decode_from_slice(&encoded).expect("decode failed");
    assert_eq!(consumed, encoded.len());
}

#[test]
fn test_container_bool_roundtrip() {
    let original_true = Container {
        value: true,
        count: 1,
    };
    let original_false = Container {
        value: false,
        count: 0,
    };

    let encoded_true = encode_to_vec(&original_true).expect("encode true failed");
    let (decoded_true, _bytes): (Container<bool>, usize) =
        decode_from_slice(&encoded_true).expect("decode true failed");
    assert_eq!(original_true, decoded_true);

    let encoded_false = encode_to_vec(&original_false).expect("encode false failed");
    let (decoded_false, _bytes): (Container<bool>, usize) =
        decode_from_slice(&encoded_false).expect("decode false failed");
    assert_eq!(original_false, decoded_false);
}

#[test]
fn test_triple_u64_u64_u64_roundtrip() {
    let original = Triple {
        a: u64::MAX,
        b: u64::MIN,
        c: 123_456_789u64,
    };
    let encoded = encode_to_vec(&original).expect("encode failed");
    let (decoded, _bytes): (Triple<u64, u64, u64>, usize) =
        decode_from_slice(&encoded).expect("decode failed");
    assert_eq!(original, decoded);
}

#[test]
fn test_either_left_big_vec_roundtrip() {
    let big_vec: Vec<u8> = (0u8..=255).cycle().take(1000).collect();
    let original: Either<Vec<u8>, String> = Either::Left(big_vec);
    let cfg = config::standard().with_big_endian();
    let encoded = encode_to_vec_with_config(&original, cfg).expect("encode failed");
    let (decoded, _bytes): (Either<Vec<u8>, String>, usize) =
        decode_from_slice_with_config(&encoded, cfg).expect("decode failed");
    assert_eq!(original, decoded);
}

#[test]
fn test_container_btreemap_u32_string_roundtrip() {
    let mut map = BTreeMap::new();
    map.insert(1u32, "one".to_string());
    map.insert(2u32, "two".to_string());
    map.insert(3u32, "three".to_string());
    let original = Container {
        value: map,
        count: 3,
    };
    let encoded = encode_to_vec(&original).expect("encode failed");
    let (decoded, _bytes): (Container<BTreeMap<u32, String>>, usize) =
        decode_from_slice(&encoded).expect("decode failed");
    assert_eq!(original, decoded);
}
