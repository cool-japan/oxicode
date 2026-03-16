//! Advanced integer type encoding tests (set 2).
//!
//! Focuses on boundary values and encoding characteristics not covered
//! by existing integer_types_test.rs.

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

// ---------------------------------------------------------------------------
// 1. u8 all-values roundtrip
// ---------------------------------------------------------------------------

#[test]
fn test_u8_all_values_roundtrip() {
    for v in 0u8..=255 {
        let enc = encode_to_vec(&v).expect("u8 encode");
        let (dec, _): (u8, usize) = decode_from_slice(&enc).expect("u8 decode");
        assert_eq!(v, dec, "roundtrip failed for u8={v}");
    }
}

// ---------------------------------------------------------------------------
// 2. i8 all-values roundtrip (-128..=127)
// ---------------------------------------------------------------------------

#[test]
fn test_i8_all_values_roundtrip() {
    for raw in 0i32..=255 {
        // iterate full range via i16 cast
        let v = (raw as i16 + i8::MIN as i16) as i8;
        let enc = encode_to_vec(&v).expect("i8 encode");
        let (dec, _): (i8, usize) = decode_from_slice(&enc).expect("i8 decode");
        assert_eq!(v, dec, "roundtrip failed for i8={v}");
    }
}

// ---------------------------------------------------------------------------
// 3. u16 boundary values roundtrip
// ---------------------------------------------------------------------------

#[test]
fn test_u16_boundary_values_roundtrip() {
    let values: &[u16] = &[0, 1, 254, 255, 256, 257, 65534, 65535];
    for &v in values {
        let enc = encode_to_vec(&v).expect("u16 encode");
        let (dec, _): (u16, usize) = decode_from_slice(&enc).expect("u16 decode");
        assert_eq!(v, dec, "roundtrip failed for u16={v}");
    }
}

// ---------------------------------------------------------------------------
// 4. i16 boundary values roundtrip
// ---------------------------------------------------------------------------

#[test]
fn test_i16_boundary_values_roundtrip() {
    let values: &[i16] = &[i16::MIN, -1, 0, 1, i16::MAX];
    for &v in values {
        let enc = encode_to_vec(&v).expect("i16 encode");
        let (dec, _): (i16, usize) = decode_from_slice(&enc).expect("i16 decode");
        assert_eq!(v, dec, "roundtrip failed for i16={v}");
    }
}

// ---------------------------------------------------------------------------
// 5. u32 boundary values roundtrip (includes varint threshold boundaries)
// ---------------------------------------------------------------------------

#[test]
fn test_u32_boundary_values_roundtrip() {
    let values: &[u32] = &[0, 1, 250, 251, 252, 253, 254, u32::MAX];
    for &v in values {
        let enc = encode_to_vec(&v).expect("u32 encode");
        let (dec, _): (u32, usize) = decode_from_slice(&enc).expect("u32 decode");
        assert_eq!(v, dec, "roundtrip failed for u32={v}");
    }
}

// ---------------------------------------------------------------------------
// 6. i32 boundary values roundtrip
// ---------------------------------------------------------------------------

#[test]
fn test_i32_boundary_values_roundtrip() {
    let values: &[i32] = &[i32::MIN, -1, 0, 1, i32::MAX];
    for &v in values {
        let enc = encode_to_vec(&v).expect("i32 encode");
        let (dec, _): (i32, usize) = decode_from_slice(&enc).expect("i32 decode");
        assert_eq!(v, dec, "roundtrip failed for i32={v}");
    }
}

// ---------------------------------------------------------------------------
// 7. u64 boundary values roundtrip
// ---------------------------------------------------------------------------

#[test]
fn test_u64_boundary_values_roundtrip() {
    let values: &[u64] = &[0, 1, u64::MAX / 2, u64::MAX];
    for &v in values {
        let enc = encode_to_vec(&v).expect("u64 encode");
        let (dec, _): (u64, usize) = decode_from_slice(&enc).expect("u64 decode");
        assert_eq!(v, dec, "roundtrip failed for u64={v}");
    }
}

// ---------------------------------------------------------------------------
// 8. i64 boundary values roundtrip
// ---------------------------------------------------------------------------

#[test]
fn test_i64_boundary_values_roundtrip() {
    let values: &[i64] = &[i64::MIN, -1, 0, 1, i64::MAX];
    for &v in values {
        let enc = encode_to_vec(&v).expect("i64 encode");
        let (dec, _): (i64, usize) = decode_from_slice(&enc).expect("i64 decode");
        assert_eq!(v, dec, "roundtrip failed for i64={v}");
    }
}

// ---------------------------------------------------------------------------
// 9. u128::MAX roundtrip
// ---------------------------------------------------------------------------

#[test]
fn test_u128_roundtrip() {
    let v: u128 = u128::MAX;
    let enc = encode_to_vec(&v).expect("u128 encode");
    let (dec, _): (u128, usize) = decode_from_slice(&enc).expect("u128 decode");
    assert_eq!(v, dec);
}

// ---------------------------------------------------------------------------
// 10. i128 MIN and MAX roundtrip
// ---------------------------------------------------------------------------

#[test]
fn test_i128_roundtrip() {
    let values: &[i128] = &[i128::MIN, i128::MAX];
    for &v in values {
        let enc = encode_to_vec(&v).expect("i128 encode");
        let (dec, _): (i128, usize) = decode_from_slice(&enc).expect("i128 decode");
        assert_eq!(v, dec, "roundtrip failed for i128={v}");
    }
}

// ---------------------------------------------------------------------------
// 11. usize zero roundtrip
// ---------------------------------------------------------------------------

#[test]
fn test_usize_zero_roundtrip() {
    let v: usize = 0;
    let enc = encode_to_vec(&v).expect("usize encode");
    let (dec, _): (usize, usize) = decode_from_slice(&enc).expect("usize decode");
    assert_eq!(v, dec);
}

// ---------------------------------------------------------------------------
// 12. usize large roundtrip (usize::MAX / 2)
// ---------------------------------------------------------------------------

#[test]
fn test_usize_large_roundtrip() {
    let v: usize = usize::MAX / 2;
    let enc = encode_to_vec(&v).expect("usize encode");
    let (dec, _): (usize, usize) = decode_from_slice(&enc).expect("usize decode");
    assert_eq!(v, dec);
}

// ---------------------------------------------------------------------------
// 13. isize MIN and MAX roundtrip
// ---------------------------------------------------------------------------

#[test]
fn test_isize_roundtrip() {
    let values: &[isize] = &[isize::MIN, isize::MAX];
    for &v in values {
        let enc = encode_to_vec(&v).expect("isize encode");
        let (dec, _): (isize, usize) = decode_from_slice(&enc).expect("isize decode");
        assert_eq!(v, dec, "roundtrip failed for isize={v}");
    }
}

// ---------------------------------------------------------------------------
// 14. u32 varint encoding size increases with value magnitude
// ---------------------------------------------------------------------------

#[test]
fn test_u32_varint_size_increases() {
    let cfg = config::standard().with_variable_int_encoding();

    // Values 0..=250 use 1 byte; 251..=65535 use 3 bytes; larger use more.
    let small = encode_to_vec_with_config(&1u32, cfg).expect("small encode");
    let medium = encode_to_vec_with_config(&300u32, cfg).expect("medium encode");
    let large = encode_to_vec_with_config(&100_000u32, cfg).expect("large encode");

    assert!(
        small.len() <= medium.len(),
        "encoding size must not decrease as value grows: small={}, medium={}",
        small.len(),
        medium.len()
    );
    assert!(
        medium.len() <= large.len(),
        "encoding size must not decrease as value grows: medium={}, large={}",
        medium.len(),
        large.len()
    );
}

// ---------------------------------------------------------------------------
// 15. i32 zigzag: -1 and 0 have the same encoded size
// ---------------------------------------------------------------------------

#[test]
fn test_i32_zigzag_positive_negative() {
    // Under zigzag encoding: 0 -> 0 (1 byte), -1 -> 1 (1 byte).
    // Both should encode to a single byte in the standard (varint) config.
    let enc_zero = encode_to_vec(&0i32).expect("encode 0");
    let enc_neg_one = encode_to_vec(&(-1i32)).expect("encode -1");

    assert_eq!(
        enc_zero.len(),
        enc_neg_one.len(),
        "0 and -1 must encode to the same number of bytes under zigzag varint; \
         0 -> {} byte(s), -1 -> {} byte(s)",
        enc_zero.len(),
        enc_neg_one.len()
    );
}

// ---------------------------------------------------------------------------
// 16. u8 with standard config is always 1 byte
// ---------------------------------------------------------------------------

#[test]
fn test_u8_standard_config_always_1_byte() {
    let cfg = config::standard();
    for v in [0u8, 1, 127, 128, 254, 255] {
        let enc = encode_to_vec_with_config(&v, cfg).expect("u8 standard encode");
        assert_eq!(
            enc.len(),
            1,
            "u8 must always encode to 1 byte with standard config; value={v}, got {} byte(s)",
            enc.len()
        );
        let (dec, _): (u8, usize) =
            decode_from_slice_with_config(&enc, cfg).expect("u8 standard decode");
        assert_eq!(v, dec);
    }
}

// ---------------------------------------------------------------------------
// 17. u32 with fixed_int is always 4 bytes for all values including 0 and MAX
// ---------------------------------------------------------------------------

#[test]
fn test_u32_fixed_int_always_4_bytes_all_values() {
    let cfg = config::standard().with_fixed_int_encoding();
    for v in [0u32, 1, u32::MAX] {
        let enc = encode_to_vec_with_config(&v, cfg).expect("u32 fixed encode");
        assert_eq!(
            enc.len(),
            4,
            "u32 with fixed_int must always be 4 bytes; value={v}, got {} byte(s)",
            enc.len()
        );
        let (dec, _): (u32, usize) =
            decode_from_slice_with_config(&enc, cfg).expect("u32 fixed decode");
        assert_eq!(v, dec);
    }
}

// ---------------------------------------------------------------------------
// 18. u64 with fixed_int is always 8 bytes
// ---------------------------------------------------------------------------

#[test]
fn test_u64_fixed_int_always_8_bytes() {
    let cfg = config::standard().with_fixed_int_encoding();
    for v in [0u64, u64::MAX] {
        let enc = encode_to_vec_with_config(&v, cfg).expect("u64 fixed encode");
        assert_eq!(
            enc.len(),
            8,
            "u64 with fixed_int must always be 8 bytes; value={v}, got {} byte(s)",
            enc.len()
        );
        let (dec, _): (u64, usize) =
            decode_from_slice_with_config(&enc, cfg).expect("u64 fixed decode");
        assert_eq!(v, dec);
    }
}

// ---------------------------------------------------------------------------
// 19. consumed bytes equals encoded length for u8, u32, u64, i32
// ---------------------------------------------------------------------------

#[test]
fn test_integer_consumed_equals_len() {
    // u8
    {
        let v: u8 = 42;
        let enc = encode_to_vec(&v).expect("u8 encode");
        let (_, consumed): (u8, usize) = decode_from_slice(&enc).expect("u8 decode");
        assert_eq!(
            consumed,
            enc.len(),
            "u8: consumed must equal encoded length"
        );
    }
    // u32
    {
        let v: u32 = 12345;
        let enc = encode_to_vec(&v).expect("u32 encode");
        let (_, consumed): (u32, usize) = decode_from_slice(&enc).expect("u32 decode");
        assert_eq!(
            consumed,
            enc.len(),
            "u32: consumed must equal encoded length"
        );
    }
    // u64
    {
        let v: u64 = 999_999_999;
        let enc = encode_to_vec(&v).expect("u64 encode");
        let (_, consumed): (u64, usize) = decode_from_slice(&enc).expect("u64 decode");
        assert_eq!(
            consumed,
            enc.len(),
            "u64: consumed must equal encoded length"
        );
    }
    // i32
    {
        let v: i32 = -99999;
        let enc = encode_to_vec(&v).expect("i32 encode");
        let (_, consumed): (i32, usize) = decode_from_slice(&enc).expect("i32 decode");
        assert_eq!(
            consumed,
            enc.len(),
            "i32: consumed must equal encoded length"
        );
    }
}

// ---------------------------------------------------------------------------
// 20. u16 big-endian byte order: 0x0102 -> [0x01, 0x02]
// ---------------------------------------------------------------------------

#[test]
fn test_u16_big_endian_byte_order() {
    let cfg = config::standard()
        .with_big_endian()
        .with_fixed_int_encoding();
    let v: u16 = 0x0102;
    let enc = encode_to_vec_with_config(&v, cfg).expect("u16 be encode");
    assert_eq!(
        enc.as_slice(),
        &[0x01, 0x02],
        "big-endian u16=0x0102 must serialize as [0x01, 0x02]"
    );
    let (dec, _): (u16, usize) = decode_from_slice_with_config(&enc, cfg).expect("u16 be decode");
    assert_eq!(dec, v);
}

// ---------------------------------------------------------------------------
// 21. i32 big-endian byte order: -1 -> [0xFF, 0xFF, 0xFF, 0xFF]
// ---------------------------------------------------------------------------

#[test]
fn test_i32_big_endian_byte_order() {
    let cfg = config::standard()
        .with_big_endian()
        .with_fixed_int_encoding();
    let v: i32 = -1;
    let enc = encode_to_vec_with_config(&v, cfg).expect("i32 be encode");
    assert_eq!(
        enc.as_slice(),
        &[0xFF, 0xFF, 0xFF, 0xFF],
        "big-endian i32=-1 must serialize as [0xFF, 0xFF, 0xFF, 0xFF]"
    );
    let (dec, _): (i32, usize) = decode_from_slice_with_config(&enc, cfg).expect("i32 be decode");
    assert_eq!(dec, v);
}

// ---------------------------------------------------------------------------
// 22. All integer types at zero roundtrip
// ---------------------------------------------------------------------------

#[test]
fn test_all_integer_types_zero_roundtrip() {
    // u8
    {
        let v: u8 = 0;
        let enc = encode_to_vec(&v).expect("u8 zero encode");
        let (dec, _): (u8, usize) = decode_from_slice(&enc).expect("u8 zero decode");
        assert_eq!(dec, v);
    }
    // u16
    {
        let v: u16 = 0;
        let enc = encode_to_vec(&v).expect("u16 zero encode");
        let (dec, _): (u16, usize) = decode_from_slice(&enc).expect("u16 zero decode");
        assert_eq!(dec, v);
    }
    // u32
    {
        let v: u32 = 0;
        let enc = encode_to_vec(&v).expect("u32 zero encode");
        let (dec, _): (u32, usize) = decode_from_slice(&enc).expect("u32 zero decode");
        assert_eq!(dec, v);
    }
    // u64
    {
        let v: u64 = 0;
        let enc = encode_to_vec(&v).expect("u64 zero encode");
        let (dec, _): (u64, usize) = decode_from_slice(&enc).expect("u64 zero decode");
        assert_eq!(dec, v);
    }
    // u128
    {
        let v: u128 = 0;
        let enc = encode_to_vec(&v).expect("u128 zero encode");
        let (dec, _): (u128, usize) = decode_from_slice(&enc).expect("u128 zero decode");
        assert_eq!(dec, v);
    }
    // i8
    {
        let v: i8 = 0;
        let enc = encode_to_vec(&v).expect("i8 zero encode");
        let (dec, _): (i8, usize) = decode_from_slice(&enc).expect("i8 zero decode");
        assert_eq!(dec, v);
    }
    // i16
    {
        let v: i16 = 0;
        let enc = encode_to_vec(&v).expect("i16 zero encode");
        let (dec, _): (i16, usize) = decode_from_slice(&enc).expect("i16 zero decode");
        assert_eq!(dec, v);
    }
    // i32
    {
        let v: i32 = 0;
        let enc = encode_to_vec(&v).expect("i32 zero encode");
        let (dec, _): (i32, usize) = decode_from_slice(&enc).expect("i32 zero decode");
        assert_eq!(dec, v);
    }
    // i64
    {
        let v: i64 = 0;
        let enc = encode_to_vec(&v).expect("i64 zero encode");
        let (dec, _): (i64, usize) = decode_from_slice(&enc).expect("i64 zero decode");
        assert_eq!(dec, v);
    }
    // i128
    {
        let v: i128 = 0;
        let enc = encode_to_vec(&v).expect("i128 zero encode");
        let (dec, _): (i128, usize) = decode_from_slice(&enc).expect("i128 zero decode");
        assert_eq!(dec, v);
    }
}
