//! Advanced generic struct serialization tests for OxiCode.
//! Tests 22 different scenarios involving generic structs and types.

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

// ---------------------------------------------------------------------------
// Struct definitions (top-level so they are available to all tests)
// ---------------------------------------------------------------------------

#[derive(Debug, PartialEq, Encode, Decode)]
struct Pair<A, B> {
    first: A,
    second: B,
}

#[derive(Debug, PartialEq, Encode, Decode)]
struct Triple<A, B, C> {
    a: A,
    b: B,
    c: C,
}

#[derive(Debug, PartialEq, Encode, Decode)]
struct Wrapper<T> {
    value: T,
}

#[derive(Debug, PartialEq, Encode, Decode)]
struct Container<T> {
    items: Vec<T>,
}

#[derive(Debug, PartialEq, Encode, Decode)]
struct KeyValue<K, V> {
    key: K,
    value: V,
}

#[derive(Debug, PartialEq, Encode, Decode)]
struct WithOption<T> {
    inner: Option<T>,
    label: String,
}

#[derive(Debug, PartialEq, Encode, Decode)]
struct Counted<T> {
    count: u32,
    item: T,
}

#[derive(Debug, PartialEq, Encode, Decode)]
struct Tagged<T> {
    tag: u8,
    payload: T,
}

#[derive(Debug, PartialEq, Encode, Decode)]
struct Buf<T> {
    data: [T; 4],
}

// ---------------------------------------------------------------------------
// Test 1 — Pair<u32, String> roundtrip
// ---------------------------------------------------------------------------

#[test]
fn test_pair_u32_string_roundtrip() {
    let original = Pair {
        first: 42u32,
        second: String::from("hello"),
    };
    let encoded = encode_to_vec(&original).expect("encode Pair<u32, String>");
    let (decoded, _): (Pair<u32, String>, usize) =
        decode_from_slice(&encoded).expect("decode Pair<u32, String>");
    assert_eq!(original, decoded);
}

// ---------------------------------------------------------------------------
// Test 2 — Pair<bool, f64> roundtrip
// ---------------------------------------------------------------------------

#[test]
fn test_pair_bool_f64_roundtrip() {
    let original = Pair {
        first: true,
        second: std::f64::consts::PI,
    };
    let encoded = encode_to_vec(&original).expect("encode Pair<bool, f64>");
    let (decoded, _): (Pair<bool, f64>, usize) =
        decode_from_slice(&encoded).expect("decode Pair<bool, f64>");
    assert_eq!(original, decoded);
}

// ---------------------------------------------------------------------------
// Test 3 — Triple<u8, u16, u32> roundtrip
// ---------------------------------------------------------------------------

#[test]
fn test_triple_u8_u16_u32_roundtrip() {
    let original = Triple {
        a: 1u8,
        b: 1000u16,
        c: 100_000u32,
    };
    let encoded = encode_to_vec(&original).expect("encode Triple<u8, u16, u32>");
    let (decoded, _): (Triple<u8, u16, u32>, usize) =
        decode_from_slice(&encoded).expect("decode Triple<u8, u16, u32>");
    assert_eq!(original, decoded);
}

// ---------------------------------------------------------------------------
// Test 4 — Wrapper<u64> roundtrip
// ---------------------------------------------------------------------------

#[test]
fn test_wrapper_u64_roundtrip() {
    let original = Wrapper { value: u64::MAX };
    let encoded = encode_to_vec(&original).expect("encode Wrapper<u64>");
    let (decoded, _): (Wrapper<u64>, usize) =
        decode_from_slice(&encoded).expect("decode Wrapper<u64>");
    assert_eq!(original, decoded);
}

// ---------------------------------------------------------------------------
// Test 5 — Wrapper<String> roundtrip
// ---------------------------------------------------------------------------

#[test]
fn test_wrapper_string_roundtrip() {
    let original = Wrapper {
        value: String::from("oxicode generic test"),
    };
    let encoded = encode_to_vec(&original).expect("encode Wrapper<String>");
    let (decoded, _): (Wrapper<String>, usize) =
        decode_from_slice(&encoded).expect("decode Wrapper<String>");
    assert_eq!(original, decoded);
}

// ---------------------------------------------------------------------------
// Test 6 — Wrapper<Vec<u8>> roundtrip
// ---------------------------------------------------------------------------

#[test]
fn test_wrapper_vec_u8_roundtrip() {
    let original = Wrapper {
        value: vec![0u8, 1, 2, 255, 128, 64],
    };
    let encoded = encode_to_vec(&original).expect("encode Wrapper<Vec<u8>>");
    let (decoded, _): (Wrapper<Vec<u8>>, usize) =
        decode_from_slice(&encoded).expect("decode Wrapper<Vec<u8>>");
    assert_eq!(original, decoded);
}

// ---------------------------------------------------------------------------
// Test 7 — Container<u32> roundtrip
// ---------------------------------------------------------------------------

#[test]
fn test_container_u32_roundtrip() {
    let original = Container {
        items: vec![10u32, 20, 30, 40, 50],
    };
    let encoded = encode_to_vec(&original).expect("encode Container<u32>");
    let (decoded, _): (Container<u32>, usize) =
        decode_from_slice(&encoded).expect("decode Container<u32>");
    assert_eq!(original, decoded);
}

// ---------------------------------------------------------------------------
// Test 8 — Container<String> roundtrip
// ---------------------------------------------------------------------------

#[test]
fn test_container_string_roundtrip() {
    let original = Container {
        items: vec![
            String::from("alpha"),
            String::from("beta"),
            String::from("gamma"),
        ],
    };
    let encoded = encode_to_vec(&original).expect("encode Container<String>");
    let (decoded, _): (Container<String>, usize) =
        decode_from_slice(&encoded).expect("decode Container<String>");
    assert_eq!(original, decoded);
}

// ---------------------------------------------------------------------------
// Test 9 — KeyValue<String, u64> roundtrip
// ---------------------------------------------------------------------------

#[test]
fn test_keyvalue_string_u64_roundtrip() {
    let original = KeyValue {
        key: String::from("answer"),
        value: 42u64,
    };
    let encoded = encode_to_vec(&original).expect("encode KeyValue<String, u64>");
    let (decoded, _): (KeyValue<String, u64>, usize) =
        decode_from_slice(&encoded).expect("decode KeyValue<String, u64>");
    assert_eq!(original, decoded);
}

// ---------------------------------------------------------------------------
// Test 10 — Generic struct with Option<T> field roundtrip
// ---------------------------------------------------------------------------

#[test]
fn test_with_option_field_roundtrip() {
    let original_some = WithOption {
        inner: Some(99u32),
        label: String::from("present"),
    };
    let encoded_some = encode_to_vec(&original_some).expect("encode WithOption Some");
    let (decoded_some, _): (WithOption<u32>, usize) =
        decode_from_slice(&encoded_some).expect("decode WithOption Some");
    assert_eq!(original_some, decoded_some);

    let original_none: WithOption<u32> = WithOption {
        inner: None,
        label: String::from("absent"),
    };
    let encoded_none = encode_to_vec(&original_none).expect("encode WithOption None");
    let (decoded_none, _): (WithOption<u32>, usize) =
        decode_from_slice(&encoded_none).expect("decode WithOption None");
    assert_eq!(original_none, decoded_none);
}

// ---------------------------------------------------------------------------
// Test 11 — Nested generics: Pair<Wrapper<u32>, Wrapper<String>>
// ---------------------------------------------------------------------------

#[test]
fn test_pair_of_wrappers_roundtrip() {
    let original = Pair {
        first: Wrapper { value: 777u32 },
        second: Wrapper {
            value: String::from("nested"),
        },
    };
    let encoded = encode_to_vec(&original).expect("encode Pair<Wrapper<u32>, Wrapper<String>>");
    let (decoded, _): (Pair<Wrapper<u32>, Wrapper<String>>, usize) =
        decode_from_slice(&encoded).expect("decode Pair<Wrapper<u32>, Wrapper<String>>");
    assert_eq!(original, decoded);
}

// ---------------------------------------------------------------------------
// Test 12 — Vec<Pair<u32, u32>> roundtrip
// ---------------------------------------------------------------------------

#[test]
fn test_vec_of_pairs_roundtrip() {
    let original: Vec<Pair<u32, u32>> = vec![
        Pair {
            first: 1,
            second: 2,
        },
        Pair {
            first: 3,
            second: 4,
        },
        Pair {
            first: 100,
            second: 200,
        },
    ];
    let encoded = encode_to_vec(&original).expect("encode Vec<Pair<u32, u32>>");
    let (decoded, _): (Vec<Pair<u32, u32>>, usize) =
        decode_from_slice(&encoded).expect("decode Vec<Pair<u32, u32>>");
    assert_eq!(original, decoded);
}

// ---------------------------------------------------------------------------
// Test 13 — Counted<String> roundtrip
// ---------------------------------------------------------------------------

#[test]
fn test_counted_string_roundtrip() {
    let original = Counted {
        count: 3u32,
        item: String::from("three items"),
    };
    let encoded = encode_to_vec(&original).expect("encode Counted<String>");
    let (decoded, _): (Counted<String>, usize) =
        decode_from_slice(&encoded).expect("decode Counted<String>");
    assert_eq!(original, decoded);
}

// ---------------------------------------------------------------------------
// Test 14 — Counted<String> size check: encoded length > 0 and count field preserved
// ---------------------------------------------------------------------------

#[test]
fn test_counted_string_size_check() {
    let original = Counted {
        count: 7u32,
        item: String::from("size check"),
    };
    let encoded = encode_to_vec(&original).expect("encode Counted<String> for size check");
    assert!(
        !encoded.is_empty(),
        "encoded bytes must be non-empty for Counted<String>"
    );
    // At minimum we expect: varint(7) + varint(len("size check")) + "size check" bytes
    // = 1 + 1 + 10 = 12 bytes minimum
    assert!(
        encoded.len() >= 12,
        "encoded Counted<String> must have at least 12 bytes, got {}",
        encoded.len()
    );
    let (decoded, _): (Counted<String>, usize) =
        decode_from_slice(&encoded).expect("decode Counted<String> for size check");
    assert_eq!(decoded.count, 7u32);
    assert_eq!(decoded.item, "size check");
}

// ---------------------------------------------------------------------------
// Test 15 — Option<Wrapper<u32>> Some roundtrip
// ---------------------------------------------------------------------------

#[test]
fn test_option_wrapper_some_roundtrip() {
    let original: Option<Wrapper<u32>> = Some(Wrapper { value: 42u32 });
    let encoded = encode_to_vec(&original).expect("encode Option<Wrapper<u32>> Some");
    let (decoded, _): (Option<Wrapper<u32>>, usize) =
        decode_from_slice(&encoded).expect("decode Option<Wrapper<u32>> Some");
    assert_eq!(original, decoded);
    assert!(decoded.is_some(), "decoded must be Some");
}

// ---------------------------------------------------------------------------
// Test 16 — Option<Wrapper<u32>> None roundtrip
// ---------------------------------------------------------------------------

#[test]
fn test_option_wrapper_none_roundtrip() {
    let original: Option<Wrapper<u32>> = None;
    let encoded = encode_to_vec(&original).expect("encode Option<Wrapper<u32>> None");
    let (decoded, _): (Option<Wrapper<u32>>, usize) =
        decode_from_slice(&encoded).expect("decode Option<Wrapper<u32>> None");
    assert_eq!(original, decoded);
    assert!(decoded.is_none(), "decoded must be None");
}

// ---------------------------------------------------------------------------
// Test 17 — Fixed-int config with Pair<u32, u64>
// ---------------------------------------------------------------------------

#[test]
fn test_pair_u32_u64_fixed_int_config() {
    let cfg = config::standard().with_fixed_int_encoding();
    let original = Pair {
        first: 0xDEAD_BEEFu32,
        second: 0x0102_0304_0506_0708u64,
    };
    let encoded =
        encode_to_vec_with_config(&original, cfg).expect("encode Pair<u32, u64> fixed-int");
    // fixed u32 = 4 bytes, fixed u64 = 8 bytes; total must be exactly 12
    assert_eq!(
        encoded.len(),
        12,
        "Pair<u32, u64> with fixed-int must encode to exactly 12 bytes"
    );
    let (decoded, consumed): (Pair<u32, u64>, usize) =
        decode_from_slice_with_config(&encoded, cfg).expect("decode Pair<u32, u64> fixed-int");
    assert_eq!(original, decoded);
    assert_eq!(consumed, 12);
}

// ---------------------------------------------------------------------------
// Test 18 — Big-endian config with Wrapper<u32>
// ---------------------------------------------------------------------------

#[test]
fn test_wrapper_u32_big_endian_config() {
    let cfg = config::standard()
        .with_big_endian()
        .with_fixed_int_encoding();
    let original = Wrapper {
        value: 0x0102_0304u32,
    };
    let encoded =
        encode_to_vec_with_config(&original, cfg).expect("encode Wrapper<u32> big-endian");
    // big-endian fixed u32 = [0x01, 0x02, 0x03, 0x04]
    assert_eq!(
        encoded,
        &[0x01, 0x02, 0x03, 0x04],
        "Wrapper<u32> big-endian fixed must be MSB-first bytes"
    );
    let (decoded, _): (Wrapper<u32>, usize) =
        decode_from_slice_with_config(&encoded, cfg).expect("decode Wrapper<u32> big-endian");
    assert_eq!(original, decoded);
}

// ---------------------------------------------------------------------------
// Test 19 — Tagged<Vec<u8>> roundtrip
// ---------------------------------------------------------------------------

#[test]
fn test_tagged_vec_u8_roundtrip() {
    let original = Tagged {
        tag: 0xABu8,
        payload: vec![10u8, 20, 30, 40, 50],
    };
    let encoded = encode_to_vec(&original).expect("encode Tagged<Vec<u8>>");
    let (decoded, _): (Tagged<Vec<u8>>, usize) =
        decode_from_slice(&encoded).expect("decode Tagged<Vec<u8>>");
    assert_eq!(original, decoded);
}

// ---------------------------------------------------------------------------
// Test 20 — consumed == encoded.len() for generic struct
// ---------------------------------------------------------------------------

#[test]
fn test_consumed_equals_encoded_len_for_generic_struct() {
    let original = Wrapper {
        value: vec![1u32, 2, 3, 4, 5, 6, 7, 8],
    };
    let encoded = encode_to_vec(&original).expect("encode Wrapper<Vec<u32>>");
    let (decoded, consumed): (Wrapper<Vec<u32>>, usize) =
        decode_from_slice(&encoded).expect("decode Wrapper<Vec<u32>>");
    assert_eq!(original, decoded);
    assert_eq!(
        consumed,
        encoded.len(),
        "consumed bytes must equal encoded length for Wrapper<Vec<u32>>"
    );
}

// ---------------------------------------------------------------------------
// Test 21 — Buf<u8> (generic struct with [T; 4] array field) roundtrip
// ---------------------------------------------------------------------------

#[test]
fn test_buf_u8_array_roundtrip() {
    let original = Buf {
        data: [0xAAu8, 0xBB, 0xCC, 0xDD],
    };
    let encoded = encode_to_vec(&original).expect("encode Buf<u8>");
    let (decoded, _): (Buf<u8>, usize) = decode_from_slice(&encoded).expect("decode Buf<u8>");
    assert_eq!(original, decoded);
}

// ---------------------------------------------------------------------------
// Test 22 — Deeply nested Pair<Pair<u8, u16>, Pair<u32, u64>>
// ---------------------------------------------------------------------------

type InnerSmall = Pair<u8, u16>;
type InnerLarge = Pair<u32, u64>;
type DeepPair = Pair<InnerSmall, InnerLarge>;

#[test]
fn test_deeply_nested_pair_roundtrip() {
    let original: DeepPair = Pair {
        first: Pair {
            first: 0x01u8,
            second: 0x0203u16,
        },
        second: Pair {
            first: 0x0405_0607u32,
            second: 0x0809_0A0B_0C0D_0E0Fu64,
        },
    };
    let encoded = encode_to_vec(&original).expect("encode Pair<Pair<u8, u16>, Pair<u32, u64>>");
    let (decoded, consumed): (DeepPair, usize) =
        decode_from_slice(&encoded).expect("decode Pair<Pair<u8, u16>, Pair<u32, u64>>");
    assert_eq!(original, decoded);
    assert_eq!(
        consumed,
        encoded.len(),
        "consumed must equal encoded length for deeply nested Pair"
    );
}
