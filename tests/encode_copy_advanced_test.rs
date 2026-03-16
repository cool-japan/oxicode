//! Comprehensive tests for the `encode_copy` API covering a wide range of `Copy` types.

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
// 1. encode_copy of u8 zero
#[test]
fn test_encode_copy_u8_zero() {
    let bytes = oxicode::encode_copy(0u8).expect("encode_copy u8 zero");
    let (dec, consumed): (u8, _) = oxicode::decode_from_slice(&bytes).expect("decode u8 zero");
    assert_eq!(dec, 0u8);
    assert_eq!(consumed, bytes.len());
}

// 2. encode_copy of u8 max
#[test]
fn test_encode_copy_u8_max() {
    let bytes = oxicode::encode_copy(u8::MAX).expect("encode_copy u8 max");
    let (dec, consumed): (u8, _) = oxicode::decode_from_slice(&bytes).expect("decode u8 max");
    assert_eq!(dec, u8::MAX);
    assert_eq!(consumed, bytes.len());
}

// 3. encode_copy of u16 values
#[test]
fn test_encode_copy_u16() {
    for val in [0u16, 1, 255, 256, u16::MAX] {
        let bytes = oxicode::encode_copy(val).expect("encode_copy u16");
        let (dec, _): (u16, _) = oxicode::decode_from_slice(&bytes).expect("decode u16");
        assert_eq!(dec, val, "u16 roundtrip failed for {val}");
    }
}

// 4. encode_copy of u32 values
#[test]
fn test_encode_copy_u32() {
    for val in [0u32, 1, 127, 128, 65535, 65536, u32::MAX] {
        let bytes = oxicode::encode_copy(val).expect("encode_copy u32");
        let (dec, _): (u32, _) = oxicode::decode_from_slice(&bytes).expect("decode u32");
        assert_eq!(dec, val, "u32 roundtrip failed for {val}");
    }
}

// 5. encode_copy of u64 values
#[test]
fn test_encode_copy_u64() {
    for val in [0u64, 1, u32::MAX as u64, u64::MAX / 2, u64::MAX] {
        let bytes = oxicode::encode_copy(val).expect("encode_copy u64");
        let (dec, _): (u64, _) = oxicode::decode_from_slice(&bytes).expect("decode u64");
        assert_eq!(dec, val, "u64 roundtrip failed for {val}");
    }
}

// 6. encode_copy of i8 values
#[test]
fn test_encode_copy_i8() {
    for val in [i8::MIN, -1i8, 0, 1, i8::MAX] {
        let bytes = oxicode::encode_copy(val).expect("encode_copy i8");
        let (dec, _): (i8, _) = oxicode::decode_from_slice(&bytes).expect("decode i8");
        assert_eq!(dec, val, "i8 roundtrip failed for {val}");
    }
}

// 7. encode_copy of i16 values
#[test]
fn test_encode_copy_i16() {
    for val in [i16::MIN, -256i16, -1, 0, 1, 256, i16::MAX] {
        let bytes = oxicode::encode_copy(val).expect("encode_copy i16");
        let (dec, _): (i16, _) = oxicode::decode_from_slice(&bytes).expect("decode i16");
        assert_eq!(dec, val, "i16 roundtrip failed for {val}");
    }
}

// 8. encode_copy of i32 values
#[test]
fn test_encode_copy_i32() {
    for val in [i32::MIN, -65536i32, -1, 0, 1, 65536, i32::MAX] {
        let bytes = oxicode::encode_copy(val).expect("encode_copy i32");
        let (dec, _): (i32, _) = oxicode::decode_from_slice(&bytes).expect("decode i32");
        assert_eq!(dec, val, "i32 roundtrip failed for {val}");
    }
}

// 9. encode_copy of i64 values
#[test]
fn test_encode_copy_i64() {
    for val in [i64::MIN, -1_000_000i64, -1, 0, 1, 1_000_000, i64::MAX] {
        let bytes = oxicode::encode_copy(val).expect("encode_copy i64");
        let (dec, _): (i64, _) = oxicode::decode_from_slice(&bytes).expect("decode i64");
        assert_eq!(dec, val, "i64 roundtrip failed for {val}");
    }
}

// 10. encode_copy of bool true and false (individual tests for each value)
#[test]
fn test_encode_copy_bool_true() {
    let bytes = oxicode::encode_copy(true).expect("encode_copy bool true");
    let (dec, _): (bool, _) = oxicode::decode_from_slice(&bytes).expect("decode bool true");
    assert!(dec);
}

#[test]
fn test_encode_copy_bool_false() {
    let bytes = oxicode::encode_copy(false).expect("encode_copy bool false");
    let (dec, _): (bool, _) = oxicode::decode_from_slice(&bytes).expect("decode bool false");
    assert!(!dec);
}

// 11. encode_copy of f32 values
#[test]
fn test_encode_copy_f32() {
    for val in [
        0.0f32,
        1.0,
        -1.0,
        f32::MIN_POSITIVE,
        f32::MAX,
        std::f32::consts::PI,
    ] {
        let bytes = oxicode::encode_copy(val).expect("encode_copy f32");
        let (dec, _): (f32, _) = oxicode::decode_from_slice(&bytes).expect("decode f32");
        assert_eq!(
            dec.to_bits(),
            val.to_bits(),
            "f32 roundtrip failed for {val}"
        );
    }
}

// 12. encode_copy of f64 values (non-PI to avoid duplication with copy_encode_test.rs)
#[test]
fn test_encode_copy_f64_values() {
    for val in [
        0.0f64,
        1.0,
        -1.0,
        f64::MIN_POSITIVE,
        f64::MAX,
        f64::NEG_INFINITY,
    ] {
        let bytes = oxicode::encode_copy(val).expect("encode_copy f64");
        let (dec, _): (f64, _) = oxicode::decode_from_slice(&bytes).expect("decode f64");
        assert_eq!(
            dec.to_bits(),
            val.to_bits(),
            "f64 roundtrip failed for {val}"
        );
    }
}

// 13. encode_copy matches encode_to_vec for same type (using u32 to avoid duplication with u64 test)
#[test]
fn test_encode_copy_matches_encode_to_vec_u32() {
    let val = 987654321u32;
    let enc_copy = oxicode::encode_copy(val).expect("encode_copy");
    let enc_ref = oxicode::encode_to_vec(&val).expect("encode_to_vec");
    assert_eq!(
        enc_copy, enc_ref,
        "encode_copy and encode_to_vec must produce identical bytes"
    );
}

// 14. encode_copy of char values
#[test]
fn test_encode_copy_char() {
    for val in ['\0', 'A', 'z', '0', '\u{00FF}', '\u{1F600}'] {
        let bytes = oxicode::encode_copy(val).expect("encode_copy char");
        let (dec, _): (char, _) = oxicode::decode_from_slice(&bytes).expect("decode char");
        assert_eq!(dec, val, "char roundtrip failed for U+{:04X}", val as u32);
    }
}

// 15. encode_copy of u128 values
#[test]
fn test_encode_copy_u128() {
    for val in [0u128, 1, u64::MAX as u128, u128::MAX / 2, u128::MAX] {
        let bytes = oxicode::encode_copy(val).expect("encode_copy u128");
        let (dec, _): (u128, _) = oxicode::decode_from_slice(&bytes).expect("decode u128");
        assert_eq!(dec, val, "u128 roundtrip failed for {val}");
    }
}

// 16. encode_copy of i128 values
#[test]
fn test_encode_copy_i128() {
    for val in [i128::MIN, -1i128, 0, 1, i128::MAX / 2, i128::MAX] {
        let bytes = oxicode::encode_copy(val).expect("encode_copy i128");
        let (dec, _): (i128, _) = oxicode::decode_from_slice(&bytes).expect("decode i128");
        assert_eq!(dec, val, "i128 roundtrip failed for {val}");
    }
}

// 17. encode_copy of usize value
#[test]
fn test_encode_copy_usize() {
    for val in [0usize, 1, 255, 65535, usize::MAX / 2] {
        let bytes = oxicode::encode_copy(val).expect("encode_copy usize");
        let (dec, _): (usize, _) = oxicode::decode_from_slice(&bytes).expect("decode usize");
        assert_eq!(dec, val, "usize roundtrip failed for {val}");
    }
}

// 18. encode_copy with standard config — result matches encode_to_vec_with_config
#[test]
fn test_encode_copy_with_standard_config() {
    let val = 0xDEADBEEFu32;
    let config = oxicode::config::standard();
    let enc_copy = oxicode::encode_copy(val).expect("encode_copy");
    let enc_cfg =
        oxicode::encode_to_vec_with_config(&val, config).expect("encode_to_vec_with_config");
    assert_eq!(
        enc_copy, enc_cfg,
        "encode_copy must match encode_to_vec_with_config(standard)"
    );
}

// 19. encode_copy of u32 with fixed int encoding config — size is exactly 4 bytes
#[test]
fn test_encode_copy_fixed_int_encoding() {
    let config = oxicode::config::standard().with_fixed_int_encoding();
    // encode_copy uses standard config internally; use encode_to_vec_with_config for fixed-int variant
    let val = 1u32;
    let bytes = oxicode::encode_to_vec_with_config(&val, config).expect("encode fixed_int");
    assert_eq!(
        bytes.len(),
        4,
        "u32 with fixed encoding must be exactly 4 bytes"
    );
    let (dec, _): (u32, _) =
        oxicode::decode_from_slice_with_config(&bytes, config).expect("decode fixed_int");
    assert_eq!(dec, val);
    // Verify encode_copy (standard config) differs in byte count for small u32
    let bytes_std = oxicode::encode_copy(val).expect("encode_copy standard");
    // varint encoding of 1u32 should be shorter than fixed 4 bytes
    assert!(bytes_std.len() <= bytes.len());
}

// 20. encode_copy of [u8; 4] array
#[test]
fn test_encode_copy_u8_array_4() {
    let val: [u8; 4] = [0xDE, 0xAD, 0xBE, 0xEF];
    let bytes = oxicode::encode_copy(val).expect("encode_copy [u8;4]");
    let (dec, consumed): ([u8; 4], _) = oxicode::decode_from_slice(&bytes).expect("decode [u8;4]");
    assert_eq!(dec, val);
    assert_eq!(consumed, bytes.len());
}

// 21. encode_copy followed by decode_from_slice roundtrip for multiple types
#[test]
fn test_encode_copy_roundtrip_various_types() {
    // u8
    {
        let val = 99u8;
        let bytes = oxicode::encode_copy(val).expect("encode u8");
        let (dec, _): (u8, _) = oxicode::decode_from_slice(&bytes).expect("decode u8");
        assert_eq!(dec, val);
    }
    // i32
    {
        let val = -42_000i32;
        let bytes = oxicode::encode_copy(val).expect("encode i32");
        let (dec, _): (i32, _) = oxicode::decode_from_slice(&bytes).expect("decode i32");
        assert_eq!(dec, val);
    }
    // f32
    {
        let val = -0.0f32;
        let bytes = oxicode::encode_copy(val).expect("encode f32");
        let (dec, _): (f32, _) = oxicode::decode_from_slice(&bytes).expect("decode f32");
        assert_eq!(dec.to_bits(), val.to_bits());
    }
    // bool
    {
        let val = true;
        let bytes = oxicode::encode_copy(val).expect("encode bool");
        let (dec, _): (bool, _) = oxicode::decode_from_slice(&bytes).expect("decode bool");
        assert_eq!(dec, val);
    }
    // char
    {
        let val = '\u{263A}';
        let bytes = oxicode::encode_copy(val).expect("encode char");
        let (dec, _): (char, _) = oxicode::decode_from_slice(&bytes).expect("decode char");
        assert_eq!(dec, val);
    }
}

// 22. Multiple encode_copy calls produce identical results for same input
#[test]
fn test_encode_copy_multiple_calls_are_deterministic() {
    let values_u32 = [0u32, 1, 127, 128, 255, 256, 65535, u32::MAX];
    for val in values_u32 {
        let bytes_a = oxicode::encode_copy(val).expect("encode_copy first call");
        let bytes_b = oxicode::encode_copy(val).expect("encode_copy second call");
        let bytes_c = oxicode::encode_copy(val).expect("encode_copy third call");
        assert_eq!(
            bytes_a, bytes_b,
            "encode_copy must be deterministic (a==b) for {val}"
        );
        assert_eq!(
            bytes_b, bytes_c,
            "encode_copy must be deterministic (b==c) for {val}"
        );
    }

    let values_i64 = [i64::MIN, -1i64, 0, 1, i64::MAX];
    for val in values_i64 {
        let bytes_a = oxicode::encode_copy(val).expect("encode_copy i64 first call");
        let bytes_b = oxicode::encode_copy(val).expect("encode_copy i64 second call");
        assert_eq!(
            bytes_a, bytes_b,
            "encode_copy must be deterministic for i64 {val}"
        );
    }
}
