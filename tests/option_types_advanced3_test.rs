//! Advanced Option<T> encoding edge case tests for OxiCode (set 3)

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
struct Payload {
    data: Vec<u8>,
    tag: u32,
}

#[derive(Debug, PartialEq, Encode, Decode)]
enum Category {
    Alpha,
    Beta,
    Gamma(u32),
}

// Test 1: Option<u32> None encodes to [0x00] (1 byte)
#[test]
fn test_option_u32_none_encodes_to_single_zero_byte() {
    let none: Option<u32> = None;
    let enc = encode_to_vec(&none).expect("encode None");
    assert_eq!(enc, &[0x00], "None must encode to single zero byte");
}

// Test 2: Option<u32> Some(0) encodes with discriminant 0x01
#[test]
fn test_option_u32_some_zero_has_discriminant_0x01() {
    let some_zero: Option<u32> = Some(0);
    let enc = encode_to_vec(&some_zero).expect("encode Some(0)");
    assert_eq!(enc[0], 0x01, "Some must have discriminant 0x01");
}

// Test 3: Option<u32> Some(42) roundtrip
#[test]
fn test_option_u32_some_42_roundtrip() {
    let original: Option<u32> = Some(42);
    let enc = encode_to_vec(&original).expect("encode Some(42)");
    let (decoded, _): (Option<u32>, _) = decode_from_slice(&enc).expect("decode Some(42)");
    assert_eq!(original, decoded, "Some(42) roundtrip must preserve value");
}

// Test 4: Option<String> None roundtrip
#[test]
fn test_option_string_none_roundtrip() {
    let original: Option<String> = None;
    let enc = encode_to_vec(&original).expect("encode Option<String> None");
    let (decoded, _): (Option<String>, _) =
        decode_from_slice(&enc).expect("decode Option<String> None");
    assert_eq!(original, decoded, "Option<String> None roundtrip must hold");
}

// Test 5: Option<String> Some("hello") roundtrip
#[test]
fn test_option_string_some_hello_roundtrip() {
    let original: Option<String> = Some("hello".to_string());
    let enc = encode_to_vec(&original).expect("encode Some(\"hello\")");
    let (decoded, _): (Option<String>, _) =
        decode_from_slice(&enc).expect("decode Some(\"hello\")");
    assert_eq!(
        original, decoded,
        "Option<String> Some roundtrip must preserve string"
    );
}

// Test 6: Option<Vec<u8>> None roundtrip
#[test]
fn test_option_vec_u8_none_roundtrip() {
    let original: Option<Vec<u8>> = None;
    let enc = encode_to_vec(&original).expect("encode Option<Vec<u8>> None");
    let (decoded, _): (Option<Vec<u8>>, _) =
        decode_from_slice(&enc).expect("decode Option<Vec<u8>> None");
    assert_eq!(
        original, decoded,
        "Option<Vec<u8>> None roundtrip must hold"
    );
}

// Test 7: Option<Vec<u8>> Some(vec![1,2,3]) roundtrip
#[test]
fn test_option_vec_u8_some_roundtrip() {
    let original: Option<Vec<u8>> = Some(vec![1, 2, 3]);
    let enc = encode_to_vec(&original).expect("encode Option<Vec<u8>> Some");
    let (decoded, _): (Option<Vec<u8>>, _) =
        decode_from_slice(&enc).expect("decode Option<Vec<u8>> Some");
    assert_eq!(
        original, decoded,
        "Option<Vec<u8>> Some([1,2,3]) roundtrip must preserve bytes"
    );
}

// Test 8: Option<Payload> None roundtrip
#[test]
fn test_option_payload_none_roundtrip() {
    let original: Option<Payload> = None;
    let enc = encode_to_vec(&original).expect("encode Option<Payload> None");
    let (decoded, _): (Option<Payload>, _) =
        decode_from_slice(&enc).expect("decode Option<Payload> None");
    assert_eq!(
        original, decoded,
        "Option<Payload> None roundtrip must hold"
    );
}

// Test 9: Option<Payload> Some roundtrip
#[test]
fn test_option_payload_some_roundtrip() {
    let original: Option<Payload> = Some(Payload {
        data: vec![10, 20, 30],
        tag: 99,
    });
    let enc = encode_to_vec(&original).expect("encode Option<Payload> Some");
    let (decoded, _): (Option<Payload>, _) =
        decode_from_slice(&enc).expect("decode Option<Payload> Some");
    assert_eq!(
        original, decoded,
        "Option<Payload> Some roundtrip must preserve struct fields"
    );
}

// Test 10: Option<Category> None roundtrip
#[test]
fn test_option_category_none_roundtrip() {
    let original: Option<Category> = None;
    let enc = encode_to_vec(&original).expect("encode Option<Category> None");
    let (decoded, _): (Option<Category>, _) =
        decode_from_slice(&enc).expect("decode Option<Category> None");
    assert_eq!(
        original, decoded,
        "Option<Category> None roundtrip must hold"
    );
}

// Test 11: Option<Category> Some(Alpha) roundtrip
#[test]
fn test_option_category_some_alpha_roundtrip() {
    let original: Option<Category> = Some(Category::Alpha);
    let enc = encode_to_vec(&original).expect("encode Option<Category> Some(Alpha)");
    let (decoded, _): (Option<Category>, _) =
        decode_from_slice(&enc).expect("decode Option<Category> Some(Alpha)");
    assert_eq!(
        original, decoded,
        "Option<Category> Some(Alpha) roundtrip must hold"
    );
}

// Test 12: Option<Category> Some(Gamma(99)) roundtrip
#[test]
fn test_option_category_some_gamma_roundtrip() {
    let original: Option<Category> = Some(Category::Gamma(99));
    let enc = encode_to_vec(&original).expect("encode Option<Category> Some(Gamma(99))");
    let (decoded, _): (Option<Category>, _) =
        decode_from_slice(&enc).expect("decode Option<Category> Some(Gamma(99))");
    assert_eq!(
        original, decoded,
        "Option<Category> Some(Gamma(99)) roundtrip must preserve variant and value"
    );
}

// Test 13: Option<Option<u32>> None roundtrip
#[test]
fn test_option_option_u32_none_roundtrip() {
    let original: Option<Option<u32>> = None;
    let enc = encode_to_vec(&original).expect("encode Option<Option<u32>> None");
    let (decoded, _): (Option<Option<u32>>, _) =
        decode_from_slice(&enc).expect("decode Option<Option<u32>> None");
    assert_eq!(original, decoded, "Nested None roundtrip must hold");
}

// Test 14: Option<Option<u32>> Some(None) roundtrip
#[test]
fn test_option_option_u32_some_none_roundtrip() {
    let original: Option<Option<u32>> = Some(None);
    let enc = encode_to_vec(&original).expect("encode Option<Option<u32>> Some(None)");
    let (decoded, _): (Option<Option<u32>>, _) =
        decode_from_slice(&enc).expect("decode Option<Option<u32>> Some(None)");
    assert_eq!(original, decoded, "Some(None) nested roundtrip must hold");
}

// Test 15: Option<Option<u32>> Some(Some(42)) roundtrip
#[test]
fn test_option_option_u32_some_some_42_roundtrip() {
    let original: Option<Option<u32>> = Some(Some(42));
    let enc = encode_to_vec(&original).expect("encode Option<Option<u32>> Some(Some(42))");
    let (decoded, _): (Option<Option<u32>>, _) =
        decode_from_slice(&enc).expect("decode Option<Option<u32>> Some(Some(42))");
    assert_eq!(
        original, decoded,
        "Some(Some(42)) nested roundtrip must preserve inner value"
    );
}

// Test 16: Vec<Option<u32>> mixed roundtrip [None, Some(1), None, Some(2)]
#[test]
fn test_vec_option_u32_mixed_roundtrip() {
    let original: Vec<Option<u32>> = vec![None, Some(1), None, Some(2)];
    let enc = encode_to_vec(&original).expect("encode Vec<Option<u32>> mixed");
    let (decoded, _): (Vec<Option<u32>>, _) =
        decode_from_slice(&enc).expect("decode Vec<Option<u32>> mixed");
    assert_eq!(
        original, decoded,
        "Vec<Option<u32>> mixed roundtrip must preserve all elements"
    );
}

// Test 17: Option<u32> consumed bytes equals encoded len
#[test]
fn test_option_u32_consumed_bytes_equals_encoded_len() {
    let original: Option<u32> = Some(255);
    let enc = encode_to_vec(&original).expect("encode for consumed bytes test");
    let (_, consumed): (Option<u32>, _) =
        decode_from_slice(&enc).expect("decode for consumed bytes test");
    assert_eq!(
        consumed,
        enc.len(),
        "consumed bytes must equal total encoded length"
    );
}

// Test 18: Option<u32> None encoded size is exactly 1 byte
#[test]
fn test_option_u32_none_encoded_size_is_one_byte() {
    let none: Option<u32> = None;
    let enc = encode_to_vec(&none).expect("encode None for size check");
    assert_eq!(enc.len(), 1, "Option::None must encode to exactly 1 byte");
}

// Test 19: Option<u32> Some vs None produce different encodings
#[test]
fn test_option_u32_some_vs_none_produce_different_encodings() {
    let none: Option<u32> = None;
    let some: Option<u32> = Some(0);
    let enc_none = encode_to_vec(&none).expect("encode None for diff test");
    let enc_some = encode_to_vec(&some).expect("encode Some(0) for diff test");
    assert_ne!(
        enc_none, enc_some,
        "None and Some(0) must produce distinct byte sequences"
    );
}

// Test 20: Option<u32> fixed int config roundtrip
#[test]
fn test_option_u32_fixed_int_config_roundtrip() {
    let original: Option<u32> = Some(1234567);
    let cfg = config::standard().with_fixed_int_encoding();
    let enc = encode_to_vec_with_config(&original, cfg).expect("encode with fixed int config");
    let (decoded, _): (Option<u32>, _) =
        decode_from_slice_with_config(&enc, cfg).expect("decode with fixed int config");
    assert_eq!(
        original, decoded,
        "Option<u32> with fixed int config roundtrip must preserve value"
    );
}

// Test 21: Option<Vec<String>> Some with unicode strings roundtrip
#[test]
fn test_option_vec_string_some_unicode_roundtrip() {
    let original: Option<Vec<String>> = Some(vec![
        "こんにちは".to_string(),
        "мир".to_string(),
        "🦀".to_string(),
        "café".to_string(),
    ]);
    let enc = encode_to_vec(&original).expect("encode Option<Vec<String>> unicode");
    let (decoded, _): (Option<Vec<String>>, _) =
        decode_from_slice(&enc).expect("decode Option<Vec<String>> unicode");
    assert_eq!(
        original, decoded,
        "Option<Vec<String>> with unicode roundtrip must preserve all characters"
    );
}

// Test 22: Vec<Option<Payload>> with 5 mixed entries roundtrip
#[test]
fn test_vec_option_payload_mixed_five_entries_roundtrip() {
    let original: Vec<Option<Payload>> = vec![
        Some(Payload {
            data: vec![1, 2],
            tag: 10,
        }),
        None,
        Some(Payload {
            data: vec![],
            tag: 0,
        }),
        None,
        Some(Payload {
            data: vec![255, 128, 64, 32, 16],
            tag: 9999,
        }),
    ];
    let enc = encode_to_vec(&original).expect("encode Vec<Option<Payload>> mixed 5");
    let (decoded, _): (Vec<Option<Payload>>, _) =
        decode_from_slice(&enc).expect("decode Vec<Option<Payload>> mixed 5");
    assert_eq!(
        original, decoded,
        "Vec<Option<Payload>> with 5 mixed entries roundtrip must preserve all elements"
    );
}
