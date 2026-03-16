//! Advanced tests for i128 and u128 encoding in OxiCode.

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
use oxicode::{decode_from_slice, encode_to_vec, encoded_size};

#[test]
fn test_u128_zero_roundtrip_one_byte() {
    let val: u128 = 0;
    let encoded = encode_to_vec(&val).expect("Failed to encode u128(0)");
    assert_eq!(encoded.len(), 1, "u128(0) should encode to 1 byte");
    assert_eq!(encoded[0], 0x00);
    let (decoded, consumed): (u128, usize) =
        decode_from_slice(&encoded).expect("Failed to decode u128(0)");
    assert_eq!(decoded, val);
    assert_eq!(consumed, 1);
}

#[test]
fn test_u128_one_roundtrip() {
    let val: u128 = 1;
    let encoded = encode_to_vec(&val).expect("Failed to encode u128(1)");
    let (decoded, _): (u128, usize) =
        decode_from_slice(&encoded).expect("Failed to decode u128(1)");
    assert_eq!(decoded, val);
}

#[test]
fn test_u128_250_roundtrip_one_byte() {
    let val: u128 = 250;
    let encoded = encode_to_vec(&val).expect("Failed to encode u128(250)");
    assert_eq!(encoded.len(), 1, "u128(250) should encode to 1 byte");
    let (decoded, consumed): (u128, usize) =
        decode_from_slice(&encoded).expect("Failed to decode u128(250)");
    assert_eq!(decoded, val);
    assert_eq!(consumed, 1);
}

#[test]
fn test_u128_251_roundtrip_three_bytes() {
    let val: u128 = 251;
    let encoded = encode_to_vec(&val).expect("Failed to encode u128(251)");
    assert_eq!(encoded.len(), 3, "u128(251) should encode to 3 bytes");
    let (decoded, consumed): (u128, usize) =
        decode_from_slice(&encoded).expect("Failed to decode u128(251)");
    assert_eq!(decoded, val);
    assert_eq!(consumed, 3);
}

#[test]
fn test_u128_u16_max_roundtrip_three_bytes() {
    let val: u128 = u16::MAX as u128;
    let encoded = encode_to_vec(&val).expect("Failed to encode u128(u16::MAX)");
    assert_eq!(encoded.len(), 3, "u128(u16::MAX) should encode to 3 bytes");
    let (decoded, consumed): (u128, usize) =
        decode_from_slice(&encoded).expect("Failed to decode u128(u16::MAX)");
    assert_eq!(decoded, val);
    assert_eq!(consumed, 3);
}

#[test]
fn test_u128_65536_roundtrip_five_bytes() {
    let val: u128 = 65536u128;
    let encoded = encode_to_vec(&val).expect("Failed to encode u128(65536)");
    assert_eq!(encoded.len(), 5, "u128(65536) should encode to 5 bytes");
    let (decoded, consumed): (u128, usize) =
        decode_from_slice(&encoded).expect("Failed to decode u128(65536)");
    assert_eq!(decoded, val);
    assert_eq!(consumed, 5);
}

#[test]
fn test_u128_u32_max_roundtrip_five_bytes() {
    let val: u128 = u32::MAX as u128;
    let encoded = encode_to_vec(&val).expect("Failed to encode u128(u32::MAX)");
    assert_eq!(encoded.len(), 5, "u128(u32::MAX) should encode to 5 bytes");
    let (decoded, consumed): (u128, usize) =
        decode_from_slice(&encoded).expect("Failed to decode u128(u32::MAX)");
    assert_eq!(decoded, val);
    assert_eq!(consumed, 5);
}

#[test]
fn test_u128_u32_max_plus_one_roundtrip_nine_bytes() {
    let val: u128 = u32::MAX as u128 + 1;
    let encoded = encode_to_vec(&val).expect("Failed to encode u128(u32::MAX + 1)");
    assert_eq!(
        encoded.len(),
        9,
        "u128(u32::MAX + 1) should encode to 9 bytes"
    );
    let (decoded, consumed): (u128, usize) =
        decode_from_slice(&encoded).expect("Failed to decode u128(u32::MAX + 1)");
    assert_eq!(decoded, val);
    assert_eq!(consumed, 9);
}

#[test]
fn test_u128_u64_max_roundtrip_nine_bytes() {
    let val: u128 = u64::MAX as u128;
    let encoded = encode_to_vec(&val).expect("Failed to encode u128(u64::MAX)");
    assert_eq!(encoded.len(), 9, "u128(u64::MAX) should encode to 9 bytes");
    let (decoded, consumed): (u128, usize) =
        decode_from_slice(&encoded).expect("Failed to decode u128(u64::MAX)");
    assert_eq!(decoded, val);
    assert_eq!(consumed, 9);
}

#[test]
fn test_u128_u64_max_plus_one_roundtrip_17_bytes_fe_marker() {
    let val: u128 = u64::MAX as u128 + 1;
    let encoded = encode_to_vec(&val).expect("Failed to encode u128(u64::MAX + 1)");
    assert_eq!(
        encoded.len(),
        17,
        "u128(u64::MAX + 1) should encode to 17 bytes (0xFE marker + 16 payload)"
    );
    assert_eq!(
        encoded[0], 0xFE,
        "First byte should be 0xFE marker for u128-range values"
    );
    let (decoded, consumed): (u128, usize) =
        decode_from_slice(&encoded).expect("Failed to decode u128(u64::MAX + 1)");
    assert_eq!(decoded, val);
    assert_eq!(consumed, 17);
}

#[test]
fn test_u128_max_roundtrip_17_bytes() {
    let val: u128 = u128::MAX;
    let encoded = encode_to_vec(&val).expect("Failed to encode u128::MAX");
    assert_eq!(
        encoded.len(),
        17,
        "u128::MAX should encode to 17 bytes (0xFE marker + 16 payload)"
    );
    assert_eq!(
        encoded[0], 0xFE,
        "First byte should be 0xFE marker for u128::MAX"
    );
    let (decoded, consumed): (u128, usize) =
        decode_from_slice(&encoded).expect("Failed to decode u128::MAX");
    assert_eq!(decoded, val);
    assert_eq!(consumed, 17);
}

#[test]
fn test_i128_zero_roundtrip() {
    let val: i128 = 0;
    let encoded = encode_to_vec(&val).expect("Failed to encode i128(0)");
    let (decoded, _): (i128, usize) =
        decode_from_slice(&encoded).expect("Failed to decode i128(0)");
    assert_eq!(decoded, val);
}

#[test]
fn test_i128_positive_one_zigzag_two() {
    let val: i128 = 1;
    let encoded = encode_to_vec(&val).expect("Failed to encode i128(1)");
    // zigzag encoding: 1 -> 2, so encoding should be same as u128(2)
    let two_encoded = encode_to_vec(&2u128).expect("Failed to encode u128(2)");
    assert_eq!(
        encoded, two_encoded,
        "i128(1) zigzag encodes to 2, matching u128(2)"
    );
    let (decoded, _): (i128, usize) =
        decode_from_slice(&encoded).expect("Failed to decode i128(1)");
    assert_eq!(decoded, val);
}

#[test]
fn test_i128_negative_one_zigzag_one() {
    let val: i128 = -1;
    let encoded = encode_to_vec(&val).expect("Failed to encode i128(-1)");
    // zigzag encoding: -1 -> 1, so encoding should be same as u128(1)
    let one_encoded = encode_to_vec(&1u128).expect("Failed to encode u128(1)");
    assert_eq!(
        encoded, one_encoded,
        "i128(-1) zigzag encodes to 1, matching u128(1)"
    );
    let (decoded, _): (i128, usize) =
        decode_from_slice(&encoded).expect("Failed to decode i128(-1)");
    assert_eq!(decoded, val);
}

#[test]
fn test_i128_i64_min_roundtrip() {
    let val: i128 = i64::MIN as i128;
    let encoded = encode_to_vec(&val).expect("Failed to encode i128(i64::MIN)");
    let (decoded, _): (i128, usize) =
        decode_from_slice(&encoded).expect("Failed to decode i128(i64::MIN)");
    assert_eq!(decoded, val);
}

#[test]
fn test_i128_i64_max_roundtrip() {
    let val: i128 = i64::MAX as i128;
    let encoded = encode_to_vec(&val).expect("Failed to encode i128(i64::MAX)");
    let (decoded, _): (i128, usize) =
        decode_from_slice(&encoded).expect("Failed to decode i128(i64::MAX)");
    assert_eq!(decoded, val);
}

#[test]
fn test_i128_min_roundtrip() {
    let val: i128 = i128::MIN;
    let encoded = encode_to_vec(&val).expect("Failed to encode i128::MIN");
    let (decoded, _): (i128, usize) =
        decode_from_slice(&encoded).expect("Failed to decode i128::MIN");
    assert_eq!(decoded, val);
}

#[test]
fn test_i128_max_roundtrip() {
    let val: i128 = i128::MAX;
    let encoded = encode_to_vec(&val).expect("Failed to encode i128::MAX");
    let (decoded, _): (i128, usize) =
        decode_from_slice(&encoded).expect("Failed to decode i128::MAX");
    assert_eq!(decoded, val);
}

#[test]
fn test_vec_u128_roundtrip() {
    let val: Vec<u128> = vec![
        0,
        1,
        250,
        251,
        u16::MAX as u128,
        65536u128,
        u32::MAX as u128,
        u32::MAX as u128 + 1,
        u64::MAX as u128,
        u64::MAX as u128 + 1,
        u128::MAX,
    ];
    let encoded = encode_to_vec(&val).expect("Failed to encode Vec<u128>");
    let (decoded, _): (Vec<u128>, usize) =
        decode_from_slice(&encoded).expect("Failed to decode Vec<u128>");
    assert_eq!(decoded, val);
}

#[test]
fn test_vec_i128_roundtrip() {
    let val: Vec<i128> = vec![
        0,
        1,
        -1,
        i64::MIN as i128,
        i64::MAX as i128,
        i128::MIN,
        i128::MAX,
        -42,
        42,
        i128::MIN / 2,
        i128::MAX / 2,
    ];
    let encoded = encode_to_vec(&val).expect("Failed to encode Vec<i128>");
    let (decoded, _): (Vec<i128>, usize) =
        decode_from_slice(&encoded).expect("Failed to decode Vec<i128>");
    assert_eq!(decoded, val);
}

#[test]
fn test_encoded_size_u128_matches_encode_to_vec_len() {
    let test_values: &[u128] = &[
        0,
        1,
        250,
        251,
        u16::MAX as u128,
        65536u128,
        u32::MAX as u128,
        u32::MAX as u128 + 1,
        u64::MAX as u128,
        u64::MAX as u128 + 1,
        u128::MAX,
    ];
    for &val in test_values {
        let predicted = encoded_size(&val).expect("encoded_size failed for u128");
        let actual = encode_to_vec(&val)
            .expect("encode_to_vec failed for u128")
            .len();
        assert_eq!(
            predicted, actual,
            "encoded_size({}) = {} but encode_to_vec produced {} bytes",
            val, predicted, actual
        );
    }
}

#[test]
fn test_option_u128_some_and_none_roundtrip() {
    // Test Some(u128::MAX)
    let some_val: Option<u128> = Some(u128::MAX);
    let encoded_some = encode_to_vec(&some_val).expect("Failed to encode Some(u128::MAX)");
    let (decoded_some, _): (Option<u128>, usize) =
        decode_from_slice(&encoded_some).expect("Failed to decode Some(u128::MAX)");
    assert_eq!(decoded_some, some_val);

    // Test None
    let none_val: Option<u128> = None;
    let encoded_none = encode_to_vec(&none_val).expect("Failed to encode None::<u128>");
    let (decoded_none, _): (Option<u128>, usize) =
        decode_from_slice(&encoded_none).expect("Failed to decode None::<u128>");
    assert_eq!(decoded_none, none_val);

    // Ensure Some and None have different encodings
    assert_ne!(
        encoded_some, encoded_none,
        "Some(u128::MAX) and None should have different encodings"
    );

    // Test Some(0)
    let some_zero: Option<u128> = Some(0);
    let encoded_some_zero = encode_to_vec(&some_zero).expect("Failed to encode Some(0u128)");
    let (decoded_some_zero, _): (Option<u128>, usize) =
        decode_from_slice(&encoded_some_zero).expect("Failed to decode Some(0u128)");
    assert_eq!(decoded_some_zero, some_zero);
    assert_ne!(
        encoded_some_zero, encoded_none,
        "Some(0u128) and None should have different encodings"
    );
}
