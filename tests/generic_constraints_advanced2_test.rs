//! Tests for generic type constraints and bounded generics in encoding/decoding.

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

// ---- Generic type definitions ----

#[derive(Debug, PartialEq, Encode, Decode)]
struct Wrapper<T: Encode + Decode> {
    value: T,
}

#[derive(Debug, PartialEq, Encode, Decode)]
struct Pair<A: Encode + Decode, B: Encode + Decode> {
    first: A,
    second: B,
}

#[derive(Debug, PartialEq, Encode, Decode)]
struct Triple<A: Encode + Decode, B: Encode + Decode, C: Encode + Decode> {
    a: A,
    b: B,
    c: C,
}

#[derive(Debug, PartialEq, Encode, Decode)]
enum Either<L: Encode + Decode, R: Encode + Decode> {
    Left(L),
    Right(R),
}

#[derive(Debug, PartialEq, Encode, Decode)]
struct Bounded<T>
where
    T: Encode + Decode + Clone,
{
    inner: T,
}

// ---- Tests ----

// 1. Generic struct Wrapper<T: Encode + Decode> with u32
#[test]
fn test_wrapper_u32_roundtrip() {
    let original = Wrapper { value: 42u32 };
    let enc = encode_to_vec(&original).expect("encode Wrapper<u32>");
    let (decoded, _): (Wrapper<u32>, usize) = decode_from_slice(&enc).expect("decode Wrapper<u32>");
    assert_eq!(original, decoded);
}

// 2. Generic struct Wrapper<T: Encode + Decode> with String
#[test]
fn test_wrapper_string_roundtrip() {
    let original = Wrapper {
        value: String::from("hello oxicode"),
    };
    let enc = encode_to_vec(&original).expect("encode Wrapper<String>");
    let (decoded, _): (Wrapper<String>, usize) =
        decode_from_slice(&enc).expect("decode Wrapper<String>");
    assert_eq!(original, decoded);
}

// 3. Generic struct Wrapper<T: Encode + Decode> with Vec<u8>
#[test]
fn test_wrapper_vec_u8_roundtrip() {
    let original = Wrapper {
        value: vec![1u8, 2, 3, 4, 5],
    };
    let enc = encode_to_vec(&original).expect("encode Wrapper<Vec<u8>>");
    let (decoded, _): (Wrapper<Vec<u8>>, usize) =
        decode_from_slice(&enc).expect("decode Wrapper<Vec<u8>>");
    assert_eq!(original, decoded);
}

// 4. Generic struct Pair<A, B> with (u32, String)
#[test]
fn test_pair_u32_string_roundtrip() {
    let original = Pair {
        first: 100u32,
        second: String::from("pair test"),
    };
    let enc = encode_to_vec(&original).expect("encode Pair<u32, String>");
    let (decoded, _): (Pair<u32, String>, usize) =
        decode_from_slice(&enc).expect("decode Pair<u32, String>");
    assert_eq!(original, decoded);
}

// 5. Generic struct Triple<A, B, C> with (u8, u16, u32)
#[test]
fn test_triple_u8_u16_u32_roundtrip() {
    let original = Triple {
        a: 1u8,
        b: 256u16,
        c: 65536u32,
    };
    let enc = encode_to_vec(&original).expect("encode Triple<u8, u16, u32>");
    let (decoded, _): (Triple<u8, u16, u32>, usize) =
        decode_from_slice(&enc).expect("decode Triple<u8, u16, u32>");
    assert_eq!(original, decoded);
}

// 6. Vec<Wrapper<u32>> roundtrip
#[test]
fn test_vec_of_wrapper_u32_roundtrip() {
    let original: Vec<Wrapper<u32>> = vec![
        Wrapper { value: 10u32 },
        Wrapper { value: 20u32 },
        Wrapper { value: 30u32 },
    ];
    let enc = encode_to_vec(&original).expect("encode Vec<Wrapper<u32>>");
    let (decoded, _): (Vec<Wrapper<u32>>, usize) =
        decode_from_slice(&enc).expect("decode Vec<Wrapper<u32>>");
    assert_eq!(original, decoded);
}

// 7. Option<Wrapper<String>> Some roundtrip
#[test]
fn test_option_wrapper_string_some_roundtrip() {
    let original: Option<Wrapper<String>> = Some(Wrapper {
        value: String::from("some value"),
    });
    let enc = encode_to_vec(&original).expect("encode Option<Wrapper<String>> Some");
    let (decoded, _): (Option<Wrapper<String>>, usize) =
        decode_from_slice(&enc).expect("decode Option<Wrapper<String>> Some");
    assert_eq!(original, decoded);
}

// 8. Option<Wrapper<String>> None roundtrip
#[test]
fn test_option_wrapper_string_none_roundtrip() {
    let original: Option<Wrapper<String>> = None;
    let enc = encode_to_vec(&original).expect("encode Option<Wrapper<String>> None");
    let (decoded, _): (Option<Wrapper<String>>, usize) =
        decode_from_slice(&enc).expect("decode Option<Wrapper<String>> None");
    assert_eq!(original, decoded);
}

// 9. Box<Wrapper<u32>> roundtrip
#[test]
fn test_box_wrapper_u32_roundtrip() {
    let original: Box<Wrapper<u32>> = Box::new(Wrapper { value: 99u32 });
    let enc = encode_to_vec(&*original).expect("encode Box<Wrapper<u32>>");
    let (decoded, _): (Wrapper<u32>, usize) =
        decode_from_slice(&enc).expect("decode Box<Wrapper<u32>>");
    assert_eq!(*original, decoded);
}

// 10. Generic enum Either<L, R> Left variant
#[test]
fn test_either_left_variant_roundtrip() {
    let original: Either<u32, String> = Either::Left(77u32);
    let enc = encode_to_vec(&original).expect("encode Either::Left");
    let (decoded, _): (Either<u32, String>, usize) =
        decode_from_slice(&enc).expect("decode Either::Left");
    assert_eq!(original, decoded);
}

// 11. Generic enum Either<L, R> Right variant
#[test]
fn test_either_right_variant_roundtrip() {
    let original: Either<u32, String> = Either::Right(String::from("right side"));
    let enc = encode_to_vec(&original).expect("encode Either::Right");
    let (decoded, _): (Either<u32, String>, usize) =
        decode_from_slice(&enc).expect("decode Either::Right");
    assert_eq!(original, decoded);
}

// 12. Nested generic Wrapper<Wrapper<u32>> roundtrip
#[test]
fn test_nested_wrapper_roundtrip() {
    let original = Wrapper {
        value: Wrapper { value: 42u32 },
    };
    let enc = encode_to_vec(&original).expect("encode Wrapper<Wrapper<u32>>");
    let (decoded, _): (Wrapper<Wrapper<u32>>, usize) =
        decode_from_slice(&enc).expect("decode Wrapper<Wrapper<u32>>");
    assert_eq!(original, decoded);
}

// 13. Vec of generic pairs roundtrip
#[test]
fn test_vec_of_pairs_roundtrip() {
    let original: Vec<Pair<u32, String>> = vec![
        Pair {
            first: 1u32,
            second: String::from("alpha"),
        },
        Pair {
            first: 2u32,
            second: String::from("beta"),
        },
        Pair {
            first: 3u32,
            second: String::from("gamma"),
        },
    ];
    let enc = encode_to_vec(&original).expect("encode Vec<Pair<u32, String>>");
    let (decoded, _): (Vec<Pair<u32, String>>, usize) =
        decode_from_slice(&enc).expect("decode Vec<Pair<u32, String>>");
    assert_eq!(original, decoded);
}

// 14. Generic struct with where clause Bounded<T> where T: Encode + Decode + Clone
#[test]
fn test_bounded_generic_where_clause_roundtrip() {
    let original = Bounded { inner: 123u64 };
    let enc = encode_to_vec(&original).expect("encode Bounded<u64>");
    let (decoded, _): (Bounded<u64>, usize) = decode_from_slice(&enc).expect("decode Bounded<u64>");
    assert_eq!(original, decoded);
}

// 15. Pair<bool, f64> roundtrip
#[test]
fn test_pair_bool_f64_roundtrip() {
    let original = Pair {
        first: true,
        second: 3.14f64,
    };
    let enc = encode_to_vec(&original).expect("encode Pair<bool, f64>");
    let (decoded, _): (Pair<bool, f64>, usize) =
        decode_from_slice(&enc).expect("decode Pair<bool, f64>");
    assert_eq!(original, decoded);
}

// 16. Pair<Option<u32>, Vec<u8>> roundtrip
#[test]
fn test_pair_option_u32_vec_u8_roundtrip() {
    let original = Pair {
        first: Some(255u32),
        second: vec![10u8, 20, 30],
    };
    let enc = encode_to_vec(&original).expect("encode Pair<Option<u32>, Vec<u8>>");
    let (decoded, _): (Pair<Option<u32>, Vec<u8>>, usize) =
        decode_from_slice(&enc).expect("decode Pair<Option<u32>, Vec<u8>>");
    assert_eq!(original, decoded);
}

// 17. Fixed-int config with generic struct
#[test]
fn test_fixed_int_config_with_generic_struct() {
    let original = Wrapper { value: 1000u32 };
    let cfg = config::standard().with_fixed_int_encoding();
    let enc =
        encode_to_vec_with_config(&original, cfg).expect("encode Wrapper<u32> fixed_int config");
    let (decoded, _): (Wrapper<u32>, usize) =
        decode_from_slice_with_config(&enc, cfg).expect("decode Wrapper<u32> fixed_int config");
    assert_eq!(original, decoded);
}

// 18. Big-endian config with generic struct
#[test]
fn test_big_endian_config_with_generic_struct() {
    let original = Wrapper { value: 42u32 };
    let cfg = config::standard().with_big_endian();
    let enc =
        encode_to_vec_with_config(&original, cfg).expect("encode Wrapper<u32> big_endian config");
    let (decoded, _): (Wrapper<u32>, usize) =
        decode_from_slice_with_config(&enc, cfg).expect("decode Wrapper<u32> big_endian config");
    assert_eq!(original, decoded);
}

// 19. Wire size of Wrapper<u32> matches raw u32
#[test]
fn test_wire_size_wrapper_u32_matches_raw_u32() {
    let raw_value = 127u32;
    let wrapper = Wrapper { value: raw_value };

    let raw_enc = encode_to_vec(&raw_value).expect("encode raw u32");
    let wrapper_enc = encode_to_vec(&wrapper).expect("encode Wrapper<u32>");

    assert_eq!(
        raw_enc.len(),
        wrapper_enc.len(),
        "Wrapper<u32> wire size should match raw u32"
    );
}

// 20. Wire size of Pair<u8, u8> matches two raw u8s
#[test]
fn test_wire_size_pair_u8_u8_matches_two_raw_u8s() {
    let pair = Pair {
        first: 10u8,
        second: 20u8,
    };

    let a_enc = encode_to_vec(&10u8).expect("encode first u8");
    let b_enc = encode_to_vec(&20u8).expect("encode second u8");
    let pair_enc = encode_to_vec(&pair).expect("encode Pair<u8, u8>");

    assert_eq!(
        a_enc.len() + b_enc.len(),
        pair_enc.len(),
        "Pair<u8, u8> wire size should match two raw u8s"
    );
}

// 21. Vec<Either<u32, String>> mixed roundtrip
#[test]
fn test_vec_either_mixed_roundtrip() {
    let original: Vec<Either<u32, String>> = vec![
        Either::Left(1u32),
        Either::Right(String::from("hello")),
        Either::Left(99u32),
        Either::Right(String::from("world")),
        Either::Left(0u32),
    ];
    let enc = encode_to_vec(&original).expect("encode Vec<Either<u32, String>>");
    let (decoded, _): (Vec<Either<u32, String>>, usize) =
        decode_from_slice(&enc).expect("decode Vec<Either<u32, String>>");
    assert_eq!(original, decoded);
}

// 22. Wrapper<[u8; 4]> fixed array roundtrip
#[test]
fn test_wrapper_fixed_array_u8_4_roundtrip() {
    let original = Wrapper {
        value: [0xAu8, 0xBu8, 0xCu8, 0xDu8],
    };
    let enc = encode_to_vec(&original).expect("encode Wrapper<[u8; 4]>");
    let (decoded, _): (Wrapper<[u8; 4]>, usize) =
        decode_from_slice(&enc).expect("decode Wrapper<[u8; 4]>");
    assert_eq!(original, decoded);
}
