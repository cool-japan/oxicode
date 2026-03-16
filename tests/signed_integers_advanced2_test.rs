//! Advanced signed integer encoding tests: zigzag, boundary values, wire format.

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

// 1. i8::MIN (-128) roundtrip
#[test]
fn test_i8_min_roundtrip() {
    let value: i8 = i8::MIN;
    let enc = encode_to_vec(&value).expect("encode i8::MIN");
    let (val, _): (i8, usize) = decode_from_slice(&enc).expect("decode i8::MIN");
    assert_eq!(val, i8::MIN);
}

// 2. i8::MAX (127) roundtrip
#[test]
fn test_i8_max_roundtrip() {
    let value: i8 = i8::MAX;
    let enc = encode_to_vec(&value).expect("encode i8::MAX");
    let (val, _): (i8, usize) = decode_from_slice(&enc).expect("decode i8::MAX");
    assert_eq!(val, i8::MAX);
}

// 3. i8 value 0 roundtrip
#[test]
fn test_i8_zero_roundtrip() {
    let value: i8 = 0;
    let enc = encode_to_vec(&value).expect("encode i8 zero");
    let (val, _): (i8, usize) = decode_from_slice(&enc).expect("decode i8 zero");
    assert_eq!(val, 0i8);
}

// 4. i8 value -1 roundtrip (stored as 1 byte 0xFF in fixed-width i8 encoding)
#[test]
fn test_i8_neg1_roundtrip() {
    let value: i8 = -1;
    let enc = encode_to_vec(&value).expect("encode i8 -1");
    let (val, _): (i8, usize) = decode_from_slice(&enc).expect("decode i8 -1");
    assert_eq!(val, -1i8);
    // i8 is always 1 byte (fixed-width), -1 is represented as 0xFF (two's complement)
    assert_eq!(enc.len(), 1, "i8(-1) must encode to exactly 1 byte");
    assert_eq!(enc[0], 0xFF, "i8(-1) two's complement byte must be 0xFF");
}

// 5. i16::MIN roundtrip
#[test]
fn test_i16_min_roundtrip() {
    let value: i16 = i16::MIN;
    let enc = encode_to_vec(&value).expect("encode i16::MIN");
    let (val, _): (i16, usize) = decode_from_slice(&enc).expect("decode i16::MIN");
    assert_eq!(val, i16::MIN);
}

// 6. i16::MAX roundtrip
#[test]
fn test_i16_max_roundtrip() {
    let value: i16 = i16::MAX;
    let enc = encode_to_vec(&value).expect("encode i16::MAX");
    let (val, _): (i16, usize) = decode_from_slice(&enc).expect("decode i16::MAX");
    assert_eq!(val, i16::MAX);
}

// 7. i32::MIN roundtrip
#[test]
fn test_i32_min_roundtrip() {
    let value: i32 = i32::MIN;
    let enc = encode_to_vec(&value).expect("encode i32::MIN");
    let (val, _): (i32, usize) = decode_from_slice(&enc).expect("decode i32::MIN");
    assert_eq!(val, i32::MIN);
}

// 8. i32::MAX roundtrip
#[test]
fn test_i32_max_roundtrip() {
    let value: i32 = i32::MAX;
    let enc = encode_to_vec(&value).expect("encode i32::MAX");
    let (val, _): (i32, usize) = decode_from_slice(&enc).expect("decode i32::MAX");
    assert_eq!(val, i32::MAX);
}

// 9. i64::MIN roundtrip
#[test]
fn test_i64_min_roundtrip() {
    let value: i64 = i64::MIN;
    let enc = encode_to_vec(&value).expect("encode i64::MIN");
    let (val, _): (i64, usize) = decode_from_slice(&enc).expect("decode i64::MIN");
    assert_eq!(val, i64::MIN);
}

// 10. i64::MAX roundtrip
#[test]
fn test_i64_max_roundtrip() {
    let value: i64 = i64::MAX;
    let enc = encode_to_vec(&value).expect("encode i64::MAX");
    let (val, _): (i64, usize) = decode_from_slice(&enc).expect("decode i64::MAX");
    assert_eq!(val, i64::MAX);
}

// 11. i128::MIN roundtrip
#[test]
fn test_i128_min_roundtrip() {
    let value: i128 = i128::MIN;
    let enc = encode_to_vec(&value).expect("encode i128::MIN");
    let (val, _): (i128, usize) = decode_from_slice(&enc).expect("decode i128::MIN");
    assert_eq!(val, i128::MIN);
}

// 12. i128::MAX roundtrip
#[test]
fn test_i128_max_roundtrip() {
    let value: i128 = i128::MAX;
    let enc = encode_to_vec(&value).expect("encode i128::MAX");
    let (val, _): (i128, usize) = decode_from_slice(&enc).expect("decode i128::MAX");
    assert_eq!(val, i128::MAX);
}

// 13. isize value 0 roundtrip
#[test]
fn test_isize_zero_roundtrip() {
    let value: isize = 0;
    let enc = encode_to_vec(&value).expect("encode isize zero");
    let (val, _): (isize, usize) = decode_from_slice(&enc).expect("decode isize zero");
    assert_eq!(val, 0isize);
}

// 14. isize::MIN roundtrip
#[test]
fn test_isize_min_roundtrip() {
    let value: isize = isize::MIN;
    let enc = encode_to_vec(&value).expect("encode isize::MIN");
    let (val, _): (isize, usize) = decode_from_slice(&enc).expect("decode isize::MIN");
    assert_eq!(val, isize::MIN);
}

// 15. Zigzag: i32 value 0 → zigzag 0 → 1 byte
#[test]
fn test_zigzag_i32_zero_is_1_byte() {
    let value: i32 = 0;
    let enc = encode_to_vec(&value).expect("encode i32 zero zigzag");
    assert_eq!(
        enc.len(),
        1,
        "i32(0) zigzag encodes to 1 byte; got {} bytes",
        enc.len()
    );
}

// 16. Zigzag: i32 value -1 → zigzag 1 → 1 byte
#[test]
fn test_zigzag_i32_neg1_is_1_byte() {
    let value: i32 = -1;
    let enc = encode_to_vec(&value).expect("encode i32(-1) zigzag");
    assert_eq!(
        enc.len(),
        1,
        "i32(-1) zigzag(1) encodes to 1 byte; got {} bytes",
        enc.len()
    );
}

// 17. Zigzag: i32 value 1 → zigzag 2 → 1 byte
#[test]
fn test_zigzag_i32_pos1_is_1_byte() {
    let value: i32 = 1;
    let enc = encode_to_vec(&value).expect("encode i32(1) zigzag");
    assert_eq!(
        enc.len(),
        1,
        "i32(1) zigzag(2) encodes to 1 byte; got {} bytes",
        enc.len()
    );
}

// 18. Zigzag: i32 value -2 → zigzag 3 → 1 byte
#[test]
fn test_zigzag_i32_neg2_is_1_byte() {
    let value: i32 = -2;
    let enc = encode_to_vec(&value).expect("encode i32(-2) zigzag");
    assert_eq!(
        enc.len(),
        1,
        "i32(-2) zigzag(3) encodes to 1 byte; got {} bytes",
        enc.len()
    );
}

// 19. Vec<i32> with mix of positive and negative roundtrip
#[test]
fn test_vec_i32_mixed_signs_roundtrip() {
    let values: Vec<i32> = vec![i32::MIN, -1_000_000, -1, 0, 1, 1_000_000, i32::MAX, -42, 42];
    let enc = encode_to_vec(&values).expect("encode Vec<i32> mixed");
    let (decoded, _): (Vec<i32>, usize) = decode_from_slice(&enc).expect("decode Vec<i32> mixed");
    assert_eq!(decoded, values);
}

// 20. Fixed-int config: i32 always 4 bytes
#[test]
fn test_fixed_int_config_i32_always_4_bytes() {
    let cfg = config::standard().with_fixed_int_encoding();
    for value in [i32::MIN, -1i32, 0i32, 1i32, i32::MAX] {
        let enc = encode_to_vec_with_config(&value, cfg).expect("encode fixed i32");
        assert_eq!(
            enc.len(),
            4,
            "fixed-int i32({value}) must be 4 bytes; got {}",
            enc.len()
        );
        let (decoded, consumed): (i32, usize) =
            decode_from_slice_with_config(&enc, cfg).expect("decode fixed i32");
        assert_eq!(decoded, value);
        assert_eq!(consumed, 4);
    }
}

// 21. Fixed-int config: i64 always 8 bytes
#[test]
fn test_fixed_int_config_i64_always_8_bytes() {
    let cfg = config::standard().with_fixed_int_encoding();
    for value in [i64::MIN, -1i64, 0i64, 1i64, i64::MAX] {
        let enc = encode_to_vec_with_config(&value, cfg).expect("encode fixed i64");
        assert_eq!(
            enc.len(),
            8,
            "fixed-int i64({value}) must be 8 bytes; got {}",
            enc.len()
        );
        let (decoded, consumed): (i64, usize) =
            decode_from_slice_with_config(&enc, cfg).expect("decode fixed i64");
        assert_eq!(decoded, value);
        assert_eq!(consumed, 8);
    }
}

// 22. Big-endian fixed-int: i32(-1) = [0xFF, 0xFF, 0xFF, 0xFF]
#[test]
fn test_big_endian_fixed_i32_neg1_wire_format() {
    let cfg = config::standard()
        .with_big_endian()
        .with_fixed_int_encoding();
    let value: i32 = -1;
    let enc = encode_to_vec_with_config(&value, cfg).expect("encode be fixed i32(-1)");
    assert_eq!(
        enc,
        &[0xFF, 0xFF, 0xFF, 0xFF],
        "big-endian fixed i32(-1) must be [0xFF, 0xFF, 0xFF, 0xFF]"
    );
    let (decoded, _): (i32, usize) =
        decode_from_slice_with_config(&enc, cfg).expect("decode be fixed i32(-1)");
    assert_eq!(decoded, -1i32);
}
