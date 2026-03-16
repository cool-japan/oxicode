//! Advanced tuple encoding tests for OxiCode (set 3)

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
    encode_to_vec_with_config,
};

// Test 1: (u32,) single-element tuple roundtrip
#[test]
fn test_tuple_single_u32_roundtrip() {
    let original: (u32,) = (42u32,);
    let encoded = encode_to_vec(&original).expect("encode (u32,)");
    let (decoded, _): ((u32,), _) = decode_from_slice(&encoded).expect("decode (u32,)");
    assert_eq!(original, decoded);
}

// Test 2: (u32, u32) roundtrip
#[test]
fn test_tuple_u32_u32_roundtrip() {
    let original: (u32, u32) = (1u32, 2u32);
    let encoded = encode_to_vec(&original).expect("encode (u32, u32)");
    let (decoded, _): ((u32, u32), _) = decode_from_slice(&encoded).expect("decode (u32, u32)");
    assert_eq!(original, decoded);
}

// Test 3: (u32, String) roundtrip
#[test]
fn test_tuple_u32_string_roundtrip() {
    let original: (u32, String) = (100u32, "hello".to_string());
    let encoded = encode_to_vec(&original).expect("encode (u32, String)");
    let (decoded, _): ((u32, String), _) =
        decode_from_slice(&encoded).expect("decode (u32, String)");
    assert_eq!(original, decoded);
}

// Test 4: (String, u32, bool) roundtrip
#[test]
fn test_tuple_string_u32_bool_roundtrip() {
    let original: (String, u32, bool) = ("world".to_string(), 7u32, true);
    let encoded = encode_to_vec(&original).expect("encode (String, u32, bool)");
    let (decoded, _): ((String, u32, bool), _) =
        decode_from_slice(&encoded).expect("decode (String, u32, bool)");
    assert_eq!(original, decoded);
}

// Test 5: (u8, u16, u32, u64) roundtrip
#[test]
fn test_tuple_u8_u16_u32_u64_roundtrip() {
    let original: (u8, u16, u32, u64) = (1u8, 256u16, 65536u32, 4294967296u64);
    let encoded = encode_to_vec(&original).expect("encode (u8, u16, u32, u64)");
    let (decoded, _): ((u8, u16, u32, u64), _) =
        decode_from_slice(&encoded).expect("decode (u8, u16, u32, u64)");
    assert_eq!(original, decoded);
}

// Test 6: (bool, bool, bool) roundtrip
#[test]
fn test_tuple_three_bools_roundtrip() {
    let original: (bool, bool, bool) = (true, false, true);
    let encoded = encode_to_vec(&original).expect("encode (bool, bool, bool)");
    let (decoded, _): ((bool, bool, bool), _) =
        decode_from_slice(&encoded).expect("decode (bool, bool, bool)");
    assert_eq!(original, decoded);
}

// Test 7: (f32, f64) roundtrip (bit-exact)
#[test]
fn test_tuple_f32_f64_roundtrip_bit_exact() {
    let original: (f32, f64) = (std::f32::consts::PI, std::f64::consts::E);
    let encoded = encode_to_vec(&original).expect("encode (f32, f64)");
    let (decoded, _): ((f32, f64), _) = decode_from_slice(&encoded).expect("decode (f32, f64)");
    assert_eq!(
        original.0.to_bits(),
        decoded.0.to_bits(),
        "f32 bits must match exactly"
    );
    assert_eq!(
        original.1.to_bits(),
        decoded.1.to_bits(),
        "f64 bits must match exactly"
    );
}

// Test 8: (Vec<u8>, String) roundtrip
#[test]
fn test_tuple_vec_u8_string_roundtrip() {
    let original: (Vec<u8>, String) = (vec![10u8, 20u8, 30u8], "oxicode".to_string());
    let encoded = encode_to_vec(&original).expect("encode (Vec<u8>, String)");
    let (decoded, _): ((Vec<u8>, String), _) =
        decode_from_slice(&encoded).expect("decode (Vec<u8>, String)");
    assert_eq!(original, decoded);
}

// Test 9: (Option<u32>, Option<String>) roundtrip with Some and None
#[test]
fn test_tuple_option_some_and_none_roundtrip() {
    let original: (Option<u32>, Option<String>) = (Some(99u32), None);
    let encoded = encode_to_vec(&original).expect("encode (Option<u32>, Option<String>)");
    let (decoded, _): ((Option<u32>, Option<String>), _) =
        decode_from_slice(&encoded).expect("decode (Option<u32>, Option<String>)");
    assert_eq!(original, decoded);

    let original2: (Option<u32>, Option<String>) = (None, Some("present".to_string()));
    let encoded2 = encode_to_vec(&original2).expect("encode (None, Some(String))");
    let (decoded2, _): ((Option<u32>, Option<String>), _) =
        decode_from_slice(&encoded2).expect("decode (None, Some(String))");
    assert_eq!(original2, decoded2);
}

// Test 10: ((u32, u32), (String, bool)) nested tuple roundtrip
#[test]
fn test_tuple_nested_roundtrip() {
    let original: ((u32, u32), (String, bool)) = ((3u32, 7u32), ("nested".to_string(), false));
    let encoded = encode_to_vec(&original).expect("encode nested tuple");
    let (decoded, _): (((u32, u32), (String, bool)), _) =
        decode_from_slice(&encoded).expect("decode nested tuple");
    assert_eq!(original, decoded);
}

// Test 11: (u32, u32) consumed bytes equals encoded len
#[test]
fn test_tuple_consumed_bytes_equals_encoded_len() {
    let original: (u32, u32) = (55u32, 66u32);
    let encoded = encode_to_vec(&original).expect("encode (u32, u32) for consumed bytes test");
    let (_, consumed): ((u32, u32), _) =
        decode_from_slice(&encoded).expect("decode (u32, u32) for consumed bytes");
    assert_eq!(
        consumed,
        encoded.len(),
        "consumed bytes must equal full encoded length"
    );
}

// Test 12: (u32, String) different from (String, u32) encoding (different type order)
#[test]
fn test_tuple_type_order_affects_encoding() {
    let t1: (u32, String) = (42u32, "abc".to_string());
    let t2: (String, u32) = ("abc".to_string(), 42u32);
    let enc1 = encode_to_vec(&t1).expect("encode (u32, String)");
    let enc2 = encode_to_vec(&t2).expect("encode (String, u32)");
    assert_ne!(
        enc1, enc2,
        "different field type ordering should produce different encodings"
    );
}

// Test 13: (u32, u32, u32, u32, u32) 5-element roundtrip
#[test]
fn test_tuple_five_u32_roundtrip() {
    let original: (u32, u32, u32, u32, u32) = (1u32, 2u32, 3u32, 4u32, 5u32);
    let encoded = encode_to_vec(&original).expect("encode 5-element tuple");
    let (decoded, _): ((u32, u32, u32, u32, u32), _) =
        decode_from_slice(&encoded).expect("decode 5-element tuple");
    assert_eq!(original, decoded);
}

// Test 14: (u32, u32) fixed int config roundtrip (8 bytes total: 4+4)
#[test]
fn test_tuple_u32_u32_fixed_int_config() {
    let cfg = config::standard().with_fixed_int_encoding();
    let original: (u32, u32) = (100u32, 200u32);
    let encoded = encode_to_vec_with_config(&original, cfg).expect("encode (u32, u32) fixed int");
    assert_eq!(
        encoded.len(),
        8,
        "fixed int encoding of (u32, u32) must be exactly 8 bytes"
    );
    let (decoded, _): ((u32, u32), _) =
        decode_from_slice_with_config(&encoded, cfg).expect("decode (u32, u32) fixed int");
    assert_eq!(original, decoded);
}

// Test 15: (i32, i64) with negative values roundtrip
#[test]
fn test_tuple_i32_i64_negative_roundtrip() {
    let original: (i32, i64) = (-42i32, -9999999999i64);
    let encoded = encode_to_vec(&original).expect("encode (i32, i64) negative");
    let (decoded, _): ((i32, i64), _) =
        decode_from_slice(&encoded).expect("decode (i32, i64) negative");
    assert_eq!(original, decoded);
}

// Test 16: Vec<(u32, String)> roundtrip
#[test]
fn test_vec_of_tuples_roundtrip() {
    let original: Vec<(u32, String)> = vec![
        (1u32, "alpha".to_string()),
        (2u32, "beta".to_string()),
        (3u32, "gamma".to_string()),
    ];
    let encoded = encode_to_vec(&original).expect("encode Vec<(u32, String)>");
    let (decoded, _): (Vec<(u32, String)>, _) =
        decode_from_slice(&encoded).expect("decode Vec<(u32, String)>");
    assert_eq!(original, decoded);
}

// Test 17: Option<(u32, String)> Some roundtrip
#[test]
fn test_option_tuple_some_roundtrip() {
    let original: Option<(u32, String)> = Some((77u32, "some_value".to_string()));
    let encoded = encode_to_vec(&original).expect("encode Option<(u32, String)> Some");
    let (decoded, _): (Option<(u32, String)>, _) =
        decode_from_slice(&encoded).expect("decode Option<(u32, String)> Some");
    assert_eq!(original, decoded);
}

// Test 18: Option<(u32, String)> None roundtrip
#[test]
fn test_option_tuple_none_roundtrip() {
    let original: Option<(u32, String)> = None;
    let encoded = encode_to_vec(&original).expect("encode Option<(u32, String)> None");
    let (decoded, _): (Option<(u32, String)>, _) =
        decode_from_slice(&encoded).expect("decode Option<(u32, String)> None");
    assert_eq!(original, decoded);
}

// Test 19: (u128, u128) roundtrip
#[test]
fn test_tuple_u128_u128_roundtrip() {
    let original: (u128, u128) = (u128::MAX / 2, u128::MAX);
    let encoded = encode_to_vec(&original).expect("encode (u128, u128)");
    let (decoded, _): ((u128, u128), _) = decode_from_slice(&encoded).expect("decode (u128, u128)");
    assert_eq!(original, decoded);
}

// Test 20: ((Vec<u8>, u32), (String, bool, f64)) complex nested roundtrip
#[test]
fn test_tuple_complex_nested_roundtrip() {
    let original: ((Vec<u8>, u32), (String, bool, f64)) = (
        (vec![0xDEu8, 0xADu8, 0xBEu8, 0xEFu8], 31337u32),
        ("complex".to_string(), true, 2.718281828459045f64),
    );
    let encoded = encode_to_vec(&original).expect("encode complex nested tuple");
    let (decoded, _): (((Vec<u8>, u32), (String, bool, f64)), _) =
        decode_from_slice(&encoded).expect("decode complex nested tuple");
    assert_eq!(original.0 .0, decoded.0 .0);
    assert_eq!(original.0 .1, decoded.0 .1);
    assert_eq!(original.1 .0, decoded.1 .0);
    assert_eq!(original.1 .1, decoded.1 .1);
    assert_eq!(
        original.1 .2.to_bits(),
        decoded.1 .2.to_bits(),
        "f64 bits must match exactly"
    );
}

// Test 21: (u32, u32) tuple encodes same as two separate u32s concatenated
#[test]
fn test_tuple_encodes_as_concatenated_fields() {
    let t = (42u32, 99u32);
    let enc_tuple = encode_to_vec(&t).expect("encode tuple");
    let enc_a = encode_to_vec(&42u32).expect("encode a");
    let enc_b = encode_to_vec(&99u32).expect("encode b");
    let combined = [enc_a.as_slice(), enc_b.as_slice()].concat();
    assert_eq!(
        enc_tuple, combined,
        "tuple should encode as concatenated fields"
    );
}

// Test 22: Empty-ish: (u32,) same encoding as bare u32
#[test]
fn test_single_element_tuple_same_as_bare_u32() {
    let value: u32 = 12345u32;
    let tuple: (u32,) = (value,);
    let enc_bare = encode_to_vec(&value).expect("encode bare u32");
    let enc_tuple = encode_to_vec(&tuple).expect("encode (u32,)");
    assert_eq!(
        enc_bare, enc_tuple,
        "(u32,) must encode identically to bare u32"
    );
}
