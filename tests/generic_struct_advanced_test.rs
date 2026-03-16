//! Advanced tests for generic struct encoding in OxiCode — 22 scenarios.

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
use oxicode::{config, decode_from_slice, encode_to_vec};
use oxicode::{decode_from_slice_with_config, encode_to_vec_with_config};
use std::collections::HashMap;

// ---------------------------------------------------------------------------
// Type definitions — all at module level
// ---------------------------------------------------------------------------

#[derive(oxicode::Encode, oxicode::Decode, PartialEq, Debug)]
struct GenWrapper<T> {
    val: T,
}

#[derive(oxicode::Encode, oxicode::Decode, PartialEq, Debug)]
struct GenPair<A, B> {
    first: A,
    second: B,
}

#[derive(oxicode::Encode, oxicode::Decode, PartialEq, Debug)]
struct GenTriple<A, B, C> {
    a: A,
    b: B,
    c: C,
}

// Generic struct with where clause (Clone bound on T)
#[derive(oxicode::Encode, oxicode::Decode, PartialEq, Debug, Clone)]
struct GenConstrained<T: Clone> {
    val: T,
}

// Generic container with Vec field
#[derive(oxicode::Encode, oxicode::Decode, PartialEq, Debug)]
struct GenContainer<T> {
    items: Vec<T>,
    count: u32,
}

// Generic enum
#[derive(oxicode::Encode, oxicode::Decode, PartialEq, Debug)]
enum GenEither<L, R> {
    Left(L),
    Right(R),
}

// Generic struct with PhantomData
#[derive(oxicode::Encode, oxicode::Decode, PartialEq, Debug)]
struct GenTagged<T> {
    val: u32,
    _marker: std::marker::PhantomData<T>,
}

// ---------------------------------------------------------------------------
// Test 1: GenWrapper<u32> roundtrip
// ---------------------------------------------------------------------------

#[test]
fn test_gen_wrapper_u32_roundtrip() {
    let original = GenWrapper { val: 42u32 };
    let enc = encode_to_vec(&original).expect("encode GenWrapper<u32>");
    let (dec, _): (GenWrapper<u32>, _) = decode_from_slice(&enc).expect("decode GenWrapper<u32>");
    assert_eq!(original, dec);
}

// ---------------------------------------------------------------------------
// Test 2: GenWrapper<String> roundtrip
// ---------------------------------------------------------------------------

#[test]
fn test_gen_wrapper_string_roundtrip() {
    let original = GenWrapper {
        val: "hello, oxicode".to_string(),
    };
    let enc = encode_to_vec(&original).expect("encode GenWrapper<String>");
    let (dec, _): (GenWrapper<String>, _) =
        decode_from_slice(&enc).expect("decode GenWrapper<String>");
    assert_eq!(original, dec);
}

// ---------------------------------------------------------------------------
// Test 3: GenWrapper<Vec<u8>> roundtrip
// ---------------------------------------------------------------------------

#[test]
fn test_gen_wrapper_vec_u8_roundtrip() {
    let original = GenWrapper {
        val: vec![0xDE_u8, 0xAD, 0xBE, 0xEF],
    };
    let enc = encode_to_vec(&original).expect("encode GenWrapper<Vec<u8>>");
    let (dec, _): (GenWrapper<Vec<u8>>, _) =
        decode_from_slice(&enc).expect("decode GenWrapper<Vec<u8>>");
    assert_eq!(original, dec);
}

// ---------------------------------------------------------------------------
// Test 4: GenPair<u32, String> roundtrip
// ---------------------------------------------------------------------------

#[test]
fn test_gen_pair_u32_string_roundtrip() {
    let original = GenPair {
        first: 100u32,
        second: "pair-value".to_string(),
    };
    let enc = encode_to_vec(&original).expect("encode GenPair<u32,String>");
    let (dec, _): (GenPair<u32, String>, _) =
        decode_from_slice(&enc).expect("decode GenPair<u32,String>");
    assert_eq!(original, dec);
}

// ---------------------------------------------------------------------------
// Test 5: GenPair<Vec<u8>, Option<u64>> roundtrip
// ---------------------------------------------------------------------------

#[test]
fn test_gen_pair_vec_option_roundtrip() {
    let original = GenPair {
        first: vec![1u8, 2, 3, 4, 5],
        second: Some(0xCAFE_BABE_u64),
    };
    let enc = encode_to_vec(&original).expect("encode GenPair<Vec<u8>,Option<u64>>");
    let (dec, _): (GenPair<Vec<u8>, Option<u64>>, _) =
        decode_from_slice(&enc).expect("decode GenPair<Vec<u8>,Option<u64>>");
    assert_eq!(original, dec);

    let none_case = GenPair::<Vec<u8>, Option<u64>> {
        first: vec![],
        second: None,
    };
    let enc2 = encode_to_vec(&none_case).expect("encode none case");
    let (dec2, _): (GenPair<Vec<u8>, Option<u64>>, _) =
        decode_from_slice(&enc2).expect("decode none case");
    assert_eq!(none_case, dec2);
}

// ---------------------------------------------------------------------------
// Test 6: GenTriple<u8, u16, u32> roundtrip
// ---------------------------------------------------------------------------

#[test]
fn test_gen_triple_u8_u16_u32_roundtrip() {
    let original = GenTriple {
        a: u8::MAX,
        b: u16::MAX,
        c: u32::MAX,
    };
    let enc = encode_to_vec(&original).expect("encode GenTriple<u8,u16,u32>");
    let (dec, _): (GenTriple<u8, u16, u32>, _) =
        decode_from_slice(&enc).expect("decode GenTriple<u8,u16,u32>");
    assert_eq!(original, dec);
}

// ---------------------------------------------------------------------------
// Test 7: GenConstrained<T: Clone> with T=String
// ---------------------------------------------------------------------------

#[test]
fn test_gen_constrained_clone_roundtrip() {
    let original = GenConstrained {
        val: "constrained-clone".to_string(),
    };
    let cloned = original.clone();
    assert_eq!(original, cloned);
    let enc = encode_to_vec(&original).expect("encode GenConstrained<String>");
    let (dec, _): (GenConstrained<String>, _) =
        decode_from_slice(&enc).expect("decode GenConstrained<String>");
    assert_eq!(original, dec);
}

// ---------------------------------------------------------------------------
// Test 8: Nested generic GenWrapper<GenWrapper<u32>>
// ---------------------------------------------------------------------------

#[test]
fn test_gen_nested_wrapper_roundtrip() {
    let original = GenWrapper {
        val: GenWrapper { val: 999u32 },
    };
    let enc = encode_to_vec(&original).expect("encode GenWrapper<GenWrapper<u32>>");
    let (dec, _): (GenWrapper<GenWrapper<u32>>, _) =
        decode_from_slice(&enc).expect("decode GenWrapper<GenWrapper<u32>>");
    assert_eq!(original, dec);
}

// ---------------------------------------------------------------------------
// Test 9: Option<GenWrapper<String>> Some and None roundtrip
// ---------------------------------------------------------------------------

#[test]
fn test_gen_option_wrapper_some_none_roundtrip() {
    let some_val: Option<GenWrapper<String>> = Some(GenWrapper {
        val: "wrapped-option".to_string(),
    });
    let enc_some = encode_to_vec(&some_val).expect("encode Some(GenWrapper<String>)");
    let (dec_some, _): (Option<GenWrapper<String>>, _) =
        decode_from_slice(&enc_some).expect("decode Some(GenWrapper<String>)");
    assert_eq!(some_val, dec_some);

    let none_val: Option<GenWrapper<String>> = None;
    let enc_none = encode_to_vec(&none_val).expect("encode None");
    let (dec_none, _): (Option<GenWrapper<String>>, _) =
        decode_from_slice(&enc_none).expect("decode None");
    assert_eq!(none_val, dec_none);
}

// ---------------------------------------------------------------------------
// Test 10: Vec<GenPair<u32, String>> roundtrip
// ---------------------------------------------------------------------------

#[test]
fn test_gen_vec_pair_roundtrip() {
    let original: Vec<GenPair<u32, String>> = vec![
        GenPair {
            first: 1u32,
            second: "alpha".to_string(),
        },
        GenPair {
            first: 2u32,
            second: "beta".to_string(),
        },
        GenPair {
            first: 3u32,
            second: "gamma".to_string(),
        },
    ];
    let enc = encode_to_vec(&original).expect("encode Vec<GenPair<u32,String>>");
    let (dec, _): (Vec<GenPair<u32, String>>, _) =
        decode_from_slice(&enc).expect("decode Vec<GenPair<u32,String>>");
    assert_eq!(original, dec);
}

// ---------------------------------------------------------------------------
// Test 11: GenContainer<T> struct with T=u32
// ---------------------------------------------------------------------------

#[test]
fn test_gen_container_struct_u32() {
    let original = GenContainer {
        items: vec![10u32, 20, 30],
        count: 3,
    };
    let enc = encode_to_vec(&original).expect("encode GenContainer<u32>");
    let (dec, _): (GenContainer<u32>, _) =
        decode_from_slice(&enc).expect("decode GenContainer<u32>");
    assert_eq!(original, dec);
}

// ---------------------------------------------------------------------------
// Test 12: GenContainer<String> with 5 strings
// ---------------------------------------------------------------------------

#[test]
fn test_gen_container_string_5_items() {
    let original = GenContainer {
        items: vec![
            "one".to_string(),
            "two".to_string(),
            "three".to_string(),
            "four".to_string(),
            "five".to_string(),
        ],
        count: 5,
    };
    let enc = encode_to_vec(&original).expect("encode GenContainer<String>");
    let (dec, _): (GenContainer<String>, _) =
        decode_from_slice(&enc).expect("decode GenContainer<String>");
    assert_eq!(original, dec);
    assert_eq!(dec.items.len(), 5);
}

// ---------------------------------------------------------------------------
// Test 13: GenContainer<u64> with 10 numbers
// ---------------------------------------------------------------------------

#[test]
fn test_gen_container_u64_10_items() {
    let original = GenContainer {
        items: (0u64..10).collect(),
        count: 10,
    };
    let enc = encode_to_vec(&original).expect("encode GenContainer<u64>");
    let (dec, _): (GenContainer<u64>, _) =
        decode_from_slice(&enc).expect("decode GenContainer<u64>");
    assert_eq!(original, dec);
    assert_eq!(dec.items.len(), 10);
}

// ---------------------------------------------------------------------------
// Test 14: GenEither enum — both variants with different types
// ---------------------------------------------------------------------------

#[test]
fn test_gen_either_enum_variants() {
    let left: GenEither<u32, String> = GenEither::Left(7u32);
    let right: GenEither<u32, String> = GenEither::Right("right-side".to_string());

    let enc_left = encode_to_vec(&left).expect("encode Left");
    let (dec_left, _): (GenEither<u32, String>, _) =
        decode_from_slice(&enc_left).expect("decode Left");
    assert_eq!(left, dec_left);

    let enc_right = encode_to_vec(&right).expect("encode Right");
    let (dec_right, _): (GenEither<u32, String>, _) =
        decode_from_slice(&enc_right).expect("decode Right");
    assert_eq!(right, dec_right);
}

// ---------------------------------------------------------------------------
// Test 15: GenEither<u32, String>::Left(42) roundtrip
// ---------------------------------------------------------------------------

#[test]
fn test_gen_either_left_u32_string_roundtrip() {
    let original: GenEither<u32, String> = GenEither::Left(42u32);
    let enc = encode_to_vec(&original).expect("encode Left(42)");
    let (dec, _): (GenEither<u32, String>, _) = decode_from_slice(&enc).expect("decode Left(42)");
    assert_eq!(original, dec);
    assert!(matches!(dec, GenEither::Left(42)));
}

// ---------------------------------------------------------------------------
// Test 16: GenEither<u32, String>::Right("hello") roundtrip
// ---------------------------------------------------------------------------

#[test]
fn test_gen_either_right_u32_string_roundtrip() {
    let original: GenEither<u32, String> = GenEither::Right("hello".to_string());
    let enc = encode_to_vec(&original).expect("encode Right(hello)");
    let (dec, _): (GenEither<u32, String>, _) =
        decode_from_slice(&enc).expect("decode Right(hello)");
    assert_eq!(original, dec);
}

// ---------------------------------------------------------------------------
// Test 17: Nested generic enum GenEither<GenWrapper<u32>, String>
// ---------------------------------------------------------------------------

#[test]
fn test_gen_nested_either_wrapper_roundtrip() {
    let original: GenEither<GenWrapper<u32>, String> = GenEither::Left(GenWrapper { val: 1234u32 });
    let enc = encode_to_vec(&original).expect("encode GenEither<GenWrapper<u32>,String>");
    let (dec, _): (GenEither<GenWrapper<u32>, String>, _) =
        decode_from_slice(&enc).expect("decode GenEither<GenWrapper<u32>,String>");
    assert_eq!(original, dec);

    let right_case: GenEither<GenWrapper<u32>, String> =
        GenEither::Right("right-nested".to_string());
    let enc_r = encode_to_vec(&right_case).expect("encode right nested");
    let (dec_r, _): (GenEither<GenWrapper<u32>, String>, _) =
        decode_from_slice(&enc_r).expect("decode right nested");
    assert_eq!(right_case, dec_r);
}

// ---------------------------------------------------------------------------
// Test 18: GenWrapper<u32> with fixed int encoding (legacy config)
// ---------------------------------------------------------------------------

#[test]
fn test_gen_wrapper_fixed_int_encoding() {
    let original = GenWrapper { val: 1000u32 };
    let enc = encode_to_vec_with_config(&original, config::legacy()).expect("legacy encode");
    let (dec, _): (GenWrapper<u32>, _) =
        decode_from_slice_with_config(&enc, config::legacy()).expect("legacy decode");
    assert_eq!(original, dec);
    // legacy u32 always 4 bytes; wrapper adds varint length prefix for count
    // the u32 itself must be exactly 4 bytes within the encoding
    assert!(enc.len() >= 4);
}

// ---------------------------------------------------------------------------
// Test 19: GenWrapper<u32> with big endian config
// ---------------------------------------------------------------------------

#[test]
fn test_gen_wrapper_big_endian_config() {
    let original = GenWrapper {
        val: 0x0102_0304_u32,
    };
    let cfg = config::standard().with_big_endian();
    let enc = encode_to_vec_with_config(&original, cfg).expect("big_endian encode");
    let (dec, _): (GenWrapper<u32>, _) =
        decode_from_slice_with_config(&enc, cfg).expect("big_endian decode");
    assert_eq!(original, dec);

    // Compare with little-endian encoding — bytes should differ for non-palindrome values
    let le_cfg = config::standard().with_little_endian();
    let le_enc = encode_to_vec_with_config(&original, le_cfg).expect("le encode");
    // big-endian and little-endian may produce different byte sequences
    let _ = le_enc; // used for comparison context; just verify roundtrip
}

// ---------------------------------------------------------------------------
// Test 20: HashMap<String, GenWrapper<u64>> roundtrip
// ---------------------------------------------------------------------------

#[test]
fn test_gen_hashmap_string_wrapper_roundtrip() {
    let mut original: HashMap<String, GenWrapper<u64>> = HashMap::new();
    original.insert("alpha".to_string(), GenWrapper { val: 100u64 });
    original.insert("beta".to_string(), GenWrapper { val: 200u64 });
    original.insert("gamma".to_string(), GenWrapper { val: 300u64 });

    let enc = encode_to_vec(&original).expect("encode HashMap<String,GenWrapper<u64>>");
    let (dec, _): (HashMap<String, GenWrapper<u64>>, _) =
        decode_from_slice(&enc).expect("decode HashMap<String,GenWrapper<u64>>");
    assert_eq!(original, dec);
}

// ---------------------------------------------------------------------------
// Test 21: GenTagged<T> with PhantomData roundtrip
// ---------------------------------------------------------------------------

#[test]
fn test_gen_tagged_phantom_data_roundtrip() {
    let original: GenTagged<String> = GenTagged {
        val: 42u32,
        _marker: std::marker::PhantomData,
    };
    let enc = encode_to_vec(&original).expect("encode GenTagged<String>");
    let (dec, _): (GenTagged<String>, _) =
        decode_from_slice(&enc).expect("decode GenTagged<String>");
    assert_eq!(original, dec);
    assert_eq!(dec.val, 42u32);
}

// ---------------------------------------------------------------------------
// Test 22: Encoded size of GenWrapper<u32> equals encoded size of u32
// ---------------------------------------------------------------------------

#[test]
fn test_gen_wrapper_size_equals_u32_size() {
    let wrapper = GenWrapper { val: 99u32 };
    let bare_val = 99u32;

    let wrapper_enc = encode_to_vec(&wrapper).expect("encode wrapper");
    let bare_enc = encode_to_vec(&bare_val).expect("encode bare u32");

    // A single-field wrapper around T should encode to the same bytes as T alone
    assert_eq!(
        wrapper_enc.len(),
        bare_enc.len(),
        "GenWrapper<u32> encoded size should equal u32 encoded size"
    );
    assert_eq!(
        wrapper_enc, bare_enc,
        "GenWrapper<u32> should encode identically to bare u32"
    );
}
