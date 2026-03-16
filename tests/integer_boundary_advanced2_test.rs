//! Advanced boundary value tests for integer types with varint encoding behavior.

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

// Test 1: u8 boundary values roundtrip
#[test]
fn test_u8_boundary_roundtrip() {
    for &val in &[0u8, 1u8, 127u8, 128u8, 255u8] {
        let enc = encode_to_vec(&val).expect("encode u8");
        let (decoded, _): (u8, usize) = decode_from_slice(&enc).expect("decode u8");
        assert_eq!(val, decoded, "u8 boundary value {} failed roundtrip", val);
    }
}

// Test 2: u16 boundary values roundtrip
#[test]
fn test_u16_boundary_roundtrip() {
    for &val in &[
        0u16, 1u16, 127u16, 128u16, 255u16, 256u16, 32767u16, 65535u16,
    ] {
        let enc = encode_to_vec(&val).expect("encode u16");
        let (decoded, _): (u16, usize) = decode_from_slice(&enc).expect("decode u16");
        assert_eq!(val, decoded, "u16 boundary value {} failed roundtrip", val);
    }
}

// Test 3: u32 boundary values roundtrip
#[test]
fn test_u32_boundary_roundtrip() {
    for &val in &[0u32, u32::MAX, (1u32 << 28) - 1, 1u32 << 28, u32::MAX] {
        let enc = encode_to_vec(&val).expect("encode u32");
        let (decoded, _): (u32, usize) = decode_from_slice(&enc).expect("decode u32");
        assert_eq!(val, decoded, "u32 boundary value {} failed roundtrip", val);
    }
}

// Test 4: u64 boundary values roundtrip
#[test]
fn test_u64_boundary_roundtrip() {
    for &val in &[0u64, 1u64, u64::MAX] {
        let enc = encode_to_vec(&val).expect("encode u64");
        let (decoded, _): (u64, usize) = decode_from_slice(&enc).expect("decode u64");
        assert_eq!(val, decoded, "u64 boundary value {} failed roundtrip", val);
    }
}

// Test 5: u128 boundary values roundtrip
#[test]
fn test_u128_boundary_roundtrip() {
    for &val in &[0u128, u128::MAX] {
        let enc = encode_to_vec(&val).expect("encode u128");
        let (decoded, _): (u128, usize) = decode_from_slice(&enc).expect("decode u128");
        assert_eq!(val, decoded, "u128 boundary value {} failed roundtrip", val);
    }
}

// Test 6: i8 boundary values roundtrip
#[test]
fn test_i8_boundary_roundtrip() {
    for &val in &[i8::MIN, -1i8, 0i8, 1i8, i8::MAX] {
        let enc = encode_to_vec(&val).expect("encode i8");
        let (decoded, _): (i8, usize) = decode_from_slice(&enc).expect("decode i8");
        assert_eq!(val, decoded, "i8 boundary value {} failed roundtrip", val);
    }
}

// Test 7: i16 boundary values roundtrip
#[test]
fn test_i16_boundary_roundtrip() {
    for &val in &[i16::MIN, -1i16, 0i16, 1i16, i16::MAX] {
        let enc = encode_to_vec(&val).expect("encode i16");
        let (decoded, _): (i16, usize) = decode_from_slice(&enc).expect("decode i16");
        assert_eq!(val, decoded, "i16 boundary value {} failed roundtrip", val);
    }
}

// Test 8: i32 boundary values roundtrip
#[test]
fn test_i32_boundary_roundtrip() {
    for &val in &[i32::MIN, -1i32, 0i32, 1i32, i32::MAX] {
        let enc = encode_to_vec(&val).expect("encode i32");
        let (decoded, _): (i32, usize) = decode_from_slice(&enc).expect("decode i32");
        assert_eq!(val, decoded, "i32 boundary value {} failed roundtrip", val);
    }
}

// Test 9: i64 boundary values roundtrip
#[test]
fn test_i64_boundary_roundtrip() {
    for &val in &[i64::MIN, i64::MAX, 0i64] {
        let enc = encode_to_vec(&val).expect("encode i64");
        let (decoded, _): (i64, usize) = decode_from_slice(&enc).expect("decode i64");
        assert_eq!(val, decoded, "i64 boundary value {} failed roundtrip", val);
    }
}

// Test 10: i128 boundary values roundtrip
#[test]
fn test_i128_boundary_roundtrip() {
    for &val in &[i128::MIN, i128::MAX, 0i128] {
        let enc = encode_to_vec(&val).expect("encode i128");
        let (decoded, _): (i128, usize) = decode_from_slice(&enc).expect("decode i128");
        assert_eq!(val, decoded, "i128 boundary value {} failed roundtrip", val);
    }
}

// Test 11: usize boundary values roundtrip (0 and 42 only — MAX may differ by platform)
#[test]
fn test_usize_boundary_roundtrip() {
    for &val in &[0usize, 42usize] {
        let enc = encode_to_vec(&val).expect("encode usize");
        let (decoded, _): (usize, usize) = decode_from_slice(&enc).expect("decode usize");
        assert_eq!(
            val, decoded,
            "usize boundary value {} failed roundtrip",
            val
        );
    }
}

// Test 12: Varint encoding — small u32 (< 128) encodes to 1 byte
#[test]
fn test_varint_small_u32_one_byte() {
    let enc = encode_to_vec(&42u32).expect("encode");
    assert_eq!(enc.len(), 1, "small u32 should be 1 byte");

    let enc0 = encode_to_vec(&0u32).expect("encode 0");
    assert_eq!(
        enc0.len(),
        1,
        "u32 value 0 should be 1 byte in varint encoding"
    );

    let enc127 = encode_to_vec(&127u32).expect("encode 127");
    assert_eq!(
        enc127.len(),
        1,
        "u32 value 127 should be 1 byte in varint encoding"
    );
}

// Test 13: Varint encoding — u32 value 251 (first value above SINGLE_BYTE_MAX=250) encodes to 3
// bytes: tag byte (U16_BYTE=251) + 2 bytes for the u16 value
#[test]
fn test_varint_u32_251_three_bytes() {
    let enc = encode_to_vec(&251u32).expect("encode");
    assert_eq!(
        enc.len(),
        3,
        "u32 value 251 should be 3 bytes in varint encoding (tag + 2-byte u16)"
    );
}

// Test 14: Varint encoding — u32 value 65536 (first value above u16::MAX) encodes to 5 bytes:
// tag byte (U32_BYTE=252) + 4 bytes for the u32 value
#[test]
fn test_varint_u32_65536_five_bytes() {
    let enc = encode_to_vec(&65536u32).expect("encode");
    assert_eq!(
        enc.len(),
        5,
        "u32 value 65536 should be 5 bytes in varint encoding (tag + 4-byte u32)"
    );
}

// Test 15: Fixed-int config — u32 always encodes to exactly 4 bytes
#[test]
fn test_fixed_int_u32_four_bytes() {
    let cfg = config::standard().with_fixed_int_encoding();
    for &val in &[0u32, 1u32, 127u32, 128u32, 16384u32, u32::MAX] {
        let enc = encode_to_vec_with_config(&val, cfg).expect("encode u32 fixed");
        assert_eq!(
            enc.len(),
            4,
            "u32 {} with fixed-int encoding should be 4 bytes",
            val
        );
        let (decoded, _): (u32, usize) =
            decode_from_slice_with_config(&enc, cfg).expect("decode u32 fixed");
        assert_eq!(val, decoded);
    }
}

// Test 16: Fixed-int config — u64 always encodes to exactly 8 bytes
#[test]
fn test_fixed_int_u64_eight_bytes() {
    let cfg = config::standard().with_fixed_int_encoding();
    for &val in &[0u64, 1u64, 128u64, u64::MAX] {
        let enc = encode_to_vec_with_config(&val, cfg).expect("encode u64 fixed");
        assert_eq!(
            enc.len(),
            8,
            "u64 {} with fixed-int encoding should be 8 bytes",
            val
        );
        let (decoded, _): (u64, usize) =
            decode_from_slice_with_config(&enc, cfg).expect("decode u64 fixed");
        assert_eq!(val, decoded);
    }
}

// Test 17: Big-endian config — i32::MAX roundtrip
#[test]
fn test_big_endian_i32_max_roundtrip() {
    let cfg = config::standard().with_big_endian();
    let val = i32::MAX;
    let enc = encode_to_vec_with_config(&val, cfg).expect("encode i32::MAX big-endian");
    let (decoded, _): (i32, usize) =
        decode_from_slice_with_config(&enc, cfg).expect("decode i32::MAX big-endian");
    assert_eq!(val, decoded);
}

// Test 18: Big-endian config — u64::MAX roundtrip
#[test]
fn test_big_endian_u64_max_roundtrip() {
    let cfg = config::standard().with_big_endian();
    let val = u64::MAX;
    let enc = encode_to_vec_with_config(&val, cfg).expect("encode u64::MAX big-endian");
    let (decoded, _): (u64, usize) =
        decode_from_slice_with_config(&enc, cfg).expect("decode u64::MAX big-endian");
    assert_eq!(val, decoded);
}

// Test 19: Tuple of boundary values (u8::MAX, u16::MAX, u32::MAX) roundtrip
#[test]
fn test_tuple_boundary_roundtrip() {
    let val: (u8, u16, u32) = (u8::MAX, u16::MAX, u32::MAX);
    let enc = encode_to_vec(&val).expect("encode tuple");
    let (decoded, _): ((u8, u16, u32), usize) = decode_from_slice(&enc).expect("decode tuple");
    assert_eq!(val, decoded);
}

// Test 20: Vec<u64> with boundary values [0, 1, u64::MAX] roundtrip
#[test]
fn test_vec_u64_boundary_roundtrip() {
    let val: Vec<u64> = vec![0u64, 1u64, u64::MAX];
    let enc = encode_to_vec(&val).expect("encode Vec<u64>");
    let (decoded, _): (Vec<u64>, usize) = decode_from_slice(&enc).expect("decode Vec<u64>");
    assert_eq!(val, decoded);
}

// Test 21: Consumed bytes == encoded length for u128::MAX
#[test]
fn test_u128_max_consumed_bytes_equals_encoded_length() {
    let val = u128::MAX;
    let enc = encode_to_vec(&val).expect("encode u128::MAX");
    let encoded_len = enc.len();
    let (decoded, consumed): (u128, usize) = decode_from_slice(&enc).expect("decode u128::MAX");
    assert_eq!(val, decoded);
    assert_eq!(
        consumed, encoded_len,
        "consumed bytes ({}) should equal encoded length ({}) for u128::MAX",
        consumed, encoded_len
    );
}

// Test 22: Negative i64 values roundtrip correctly via zigzag encoding
#[test]
fn test_negative_i64_zigzag_roundtrip() {
    for &val in &[-1i64, -2i64, i64::MIN] {
        let enc = encode_to_vec(&val).expect("encode negative i64");
        let (decoded, _): (i64, usize) = decode_from_slice(&enc).expect("decode negative i64");
        assert_eq!(
            val, decoded,
            "negative i64 value {} failed zigzag roundtrip",
            val
        );
    }
}
