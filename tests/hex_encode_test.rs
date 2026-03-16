//! Tests for encode_to_hex and decode_from_hex.

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
struct Point {
    x: u32,
    y: u32,
}

#[test]
fn test_encode_to_hex_u8() {
    let hex = oxicode::encode_to_hex(&1u8).expect("encode");
    assert!(!hex.is_empty());
    // Hex string contains only valid hex chars
    assert!(hex.chars().all(|c| c.is_ascii_hexdigit()));
}

#[test]
fn test_hex_roundtrip_u32() {
    let val = 12345u32;
    let hex = oxicode::encode_to_hex(&val).expect("encode");
    let (dec, _): (u32, _) = oxicode::decode_from_hex(&hex).expect("decode");
    assert_eq!(val, dec);
}

#[test]
fn test_hex_roundtrip_string() {
    let val = "hello oxicode".to_string();
    let hex = oxicode::encode_to_hex(&val).expect("encode");
    let (dec, _): (String, _) = oxicode::decode_from_hex(&hex).expect("decode");
    assert_eq!(val, dec);
}

#[test]
fn test_hex_roundtrip_struct() {
    let p = Point { x: 10, y: 20 };
    let hex = oxicode::encode_to_hex(&p).expect("encode");
    let (dec, _): (Point, _) = oxicode::decode_from_hex(&hex).expect("decode");
    assert_eq!(p, dec);
}

#[test]
fn test_hex_matches_encode_to_vec() {
    let val = vec![1u8, 2, 3, 4, 5];
    let hex = oxicode::encode_to_hex(&val).expect("encode hex");
    let bytes = oxicode::encode_to_vec(&val).expect("encode bytes");
    let expected_hex: String = bytes.iter().map(|b| format!("{:02x}", b)).collect();
    assert_eq!(hex, expected_hex);
}

#[test]
fn test_decode_from_hex_invalid_returns_err() {
    let result: Result<(u8, _), _> = oxicode::decode_from_hex("ZZ"); // invalid hex
    assert!(result.is_err());
}

#[test]
fn test_hex_roundtrip_empty_vec() {
    let val: Vec<u8> = vec![];
    let hex = oxicode::encode_to_hex(&val).expect("encode");
    let (dec, _): (Vec<u8>, _) = oxicode::decode_from_hex(&hex).expect("decode");
    assert_eq!(val, dec);
}

#[test]
fn test_hex_lowercase_output() {
    // Verify output is lowercase hex (no uppercase A-F)
    let val = 255u8;
    let hex = oxicode::encode_to_hex(&val).expect("encode");
    assert!(!hex.contains(|c: char| c.is_ascii_uppercase()));
}

#[test]
fn test_decode_from_hex_uppercase_input() {
    // decode_from_hex should accept uppercase hex strings as well
    let lower = oxicode::encode_to_hex(&255u8).expect("encode");
    let upper = lower.to_uppercase();
    let (val_lower, _): (u8, _) = oxicode::decode_from_hex(&lower).expect("decode lower");
    let (val_upper, _): (u8, _) = oxicode::decode_from_hex(&upper).expect("decode upper");
    assert_eq!(val_lower, val_upper);
}

#[test]
fn test_decode_from_hex_odd_length_returns_err() {
    let result: Result<(u8, _), _> = oxicode::decode_from_hex("abc"); // odd length
    assert!(result.is_err());
}

// ---------------------------------------------------------------------------
// Extended tests: i128 / u128 and Option<String>
// ---------------------------------------------------------------------------

#[test]
fn test_hex_roundtrip_u128_max() {
    let val: u128 = u128::MAX;
    let hex = oxicode::encode_to_hex(&val).expect("encode u128::MAX");
    let (dec, _): (u128, _) = oxicode::decode_from_hex(&hex).expect("decode u128::MAX");
    assert_eq!(val, dec);
}

#[test]
fn test_hex_roundtrip_u128_zero() {
    let val: u128 = 0u128;
    let hex = oxicode::encode_to_hex(&val).expect("encode u128 zero");
    let (dec, _): (u128, _) = oxicode::decode_from_hex(&hex).expect("decode u128 zero");
    assert_eq!(val, dec);
}

#[test]
fn test_hex_roundtrip_i128_extremes() {
    for val in [i128::MIN, -1i128, 0i128, 1i128, i128::MAX] {
        let hex = oxicode::encode_to_hex(&val).expect("encode i128");
        let (dec, _): (i128, _) = oxicode::decode_from_hex(&hex).expect("decode i128");
        assert_eq!(val, dec, "i128 roundtrip failed for {}", val);
    }
}

#[test]
fn test_hex_roundtrip_option_string_some() {
    let val: Option<String> = Some("oxicode hex option".to_string());
    let hex = oxicode::encode_to_hex(&val).expect("encode Option<String> Some");
    let (dec, _): (Option<String>, _) =
        oxicode::decode_from_hex(&hex).expect("decode Option<String> Some");
    assert_eq!(val, dec);
}

#[test]
fn test_hex_roundtrip_option_string_none() {
    let val: Option<String> = None;
    let hex = oxicode::encode_to_hex(&val).expect("encode Option<String> None");
    let (dec, _): (Option<String>, _) =
        oxicode::decode_from_hex(&hex).expect("decode Option<String> None");
    assert_eq!(val, dec);
}

#[test]
fn test_hex_always_lowercase_for_high_bytes() {
    // Encode a value whose bytes include values in 0xa0..=0xff to ensure
    // the hex representation is always lowercase (no A-F digits).
    let val: Vec<u8> = (0xa0u8..=0xff).collect();
    let hex = oxicode::encode_to_hex(&val).expect("encode high bytes");
    assert!(
        !hex.contains(|c: char| ('A'..='F').contains(&c)),
        "hex output must be lowercase, got: {}",
        hex
    );
    // Verify round-trip as well
    let (dec, _): (Vec<u8>, _) = oxicode::decode_from_hex(&hex).expect("decode high bytes");
    assert_eq!(val, dec);
}
