//! Advanced tests for Wrapping<T> and Saturating<T> encoding in OxiCode (set 3).
//!
//! 22 top-level #[test] functions covering roundtrips, wire-byte identity,
//! config variants (fixed-int, big-endian), collections, Option, tuples,
//! derived structs, and wide integer types.

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
use std::num::{Saturating, Wrapping};

// ===== 1. Wrapping<u32> zero roundtrip =====

#[test]
fn test_wrapping_saturating_adv3_wrapping_u32_zero_roundtrip() {
    let original = Wrapping(0u32);
    let encoded = encode_to_vec(&original).expect("encode Wrapping<u32>(0) failed");
    let (decoded, _bytes): (Wrapping<u32>, usize) =
        decode_from_slice(&encoded).expect("decode Wrapping<u32>(0) failed");
    assert_eq!(original, decoded);
    assert_eq!(decoded.0, 0u32);
}

// ===== 2. Wrapping<u32> max value roundtrip =====

#[test]
fn test_wrapping_saturating_adv3_wrapping_u32_max_roundtrip() {
    let original = Wrapping(u32::MAX);
    let encoded = encode_to_vec(&original).expect("encode Wrapping<u32>(MAX) failed");
    let (decoded, _bytes): (Wrapping<u32>, usize) =
        decode_from_slice(&encoded).expect("decode Wrapping<u32>(MAX) failed");
    assert_eq!(original, decoded);
    assert_eq!(decoded.0, u32::MAX);
}

// ===== 3. Wrapping<u64> roundtrip =====

#[test]
fn test_wrapping_saturating_adv3_wrapping_u64_roundtrip() {
    let original = Wrapping(0x0123_4567_89AB_CDEFu64);
    let encoded = encode_to_vec(&original).expect("encode Wrapping<u64> failed");
    let (decoded, _bytes): (Wrapping<u64>, usize) =
        decode_from_slice(&encoded).expect("decode Wrapping<u64> failed");
    assert_eq!(original, decoded);
    assert_eq!(decoded.0, 0x0123_4567_89AB_CDEFu64);
}

// ===== 4. Wrapping<i32> negative value roundtrip =====

#[test]
fn test_wrapping_saturating_adv3_wrapping_i32_negative_roundtrip() {
    let original = Wrapping(-42_000i32);
    let encoded = encode_to_vec(&original).expect("encode Wrapping<i32>(-42000) failed");
    let (decoded, _bytes): (Wrapping<i32>, usize) =
        decode_from_slice(&encoded).expect("decode Wrapping<i32>(-42000) failed");
    assert_eq!(original, decoded);
    assert_eq!(decoded.0, -42_000i32);
}

// ===== 5. Wrapping<u8> same wire bytes as raw u8 =====

#[test]
fn test_wrapping_saturating_adv3_wrapping_u8_same_wire_as_raw() {
    let value = 200u8;
    let wrapped = Wrapping(value);
    let w_bytes = encode_to_vec(&wrapped).expect("encode Wrapping<u8> failed");
    let raw_bytes = encode_to_vec(&value).expect("encode raw u8 failed");
    assert_eq!(
        w_bytes, raw_bytes,
        "Wrapping<u8> must produce identical bytes to the inner u8"
    );
}

// ===== 6. Wrapping<u32> consumed == encoded length =====

#[test]
fn test_wrapping_saturating_adv3_wrapping_u32_consumed_equals_len() {
    let original = Wrapping(12_345_678u32);
    let encoded = encode_to_vec(&original).expect("encode Wrapping<u32> failed");
    let (decoded, consumed): (Wrapping<u32>, usize) =
        decode_from_slice(&encoded).expect("decode Wrapping<u32> failed");
    assert_eq!(original, decoded);
    assert_eq!(
        consumed,
        encoded.len(),
        "consumed bytes must equal encoded length"
    );
}

// ===== 7. Vec<Wrapping<u32>> roundtrip =====

#[test]
fn test_wrapping_saturating_adv3_vec_wrapping_u32_roundtrip() {
    let original: Vec<Wrapping<u32>> = vec![
        Wrapping(0u32),
        Wrapping(1u32),
        Wrapping(127u32),
        Wrapping(256u32),
        Wrapping(u32::MAX),
    ];
    let encoded = encode_to_vec(&original).expect("encode Vec<Wrapping<u32>> failed");
    let (decoded, _bytes): (Vec<Wrapping<u32>>, usize) =
        decode_from_slice(&encoded).expect("decode Vec<Wrapping<u32>> failed");
    assert_eq!(original, decoded);
}

// ===== 8. Wrapping<u32> fixed-int config (4 bytes) =====

#[test]
fn test_wrapping_saturating_adv3_wrapping_u32_fixed_int_4_bytes() {
    let cfg = config::standard().with_fixed_int_encoding();
    let original = Wrapping(0xABCD_EF01u32);
    let encoded =
        encode_to_vec_with_config(&original, cfg).expect("encode Wrapping<u32> fixed_int failed");
    assert_eq!(
        encoded.len(),
        4,
        "Wrapping<u32> with fixed_int must be exactly 4 bytes"
    );
    let (decoded, consumed): (Wrapping<u32>, usize) = decode_from_slice_with_config(&encoded, cfg)
        .expect("decode Wrapping<u32> fixed_int failed");
    assert_eq!(original, decoded);
    assert_eq!(consumed, 4);
}

// ===== 9. Wrapping<u32> big-endian config roundtrip =====

#[test]
fn test_wrapping_saturating_adv3_wrapping_u32_big_endian_roundtrip() {
    let cfg = config::standard()
        .with_big_endian()
        .with_fixed_int_encoding();
    let original = Wrapping(0x0807_0605u32);
    let encoded =
        encode_to_vec_with_config(&original, cfg).expect("encode Wrapping<u32> big_endian failed");
    // Big-endian: MSB first
    assert_eq!(encoded[0], 0x08, "first byte must be MSB in big-endian");
    assert_eq!(encoded[3], 0x05, "last byte must be LSB in big-endian");
    let (decoded, _bytes): (Wrapping<u32>, usize) = decode_from_slice_with_config(&encoded, cfg)
        .expect("decode Wrapping<u32> big_endian failed");
    assert_eq!(original, decoded);
}

// ===== 10. Option<Wrapping<u32>> Some roundtrip =====

#[test]
fn test_wrapping_saturating_adv3_option_wrapping_u32_some_roundtrip() {
    let original: Option<Wrapping<u32>> = Some(Wrapping(77_777u32));
    let encoded = encode_to_vec(&original).expect("encode Option<Wrapping<u32>>(Some) failed");
    let (decoded, _bytes): (Option<Wrapping<u32>>, usize) =
        decode_from_slice(&encoded).expect("decode Option<Wrapping<u32>>(Some) failed");
    assert_eq!(original, decoded);
    assert_eq!(decoded, Some(Wrapping(77_777u32)));
}

// ===== 11. Option<Wrapping<u32>> None roundtrip =====

#[test]
fn test_wrapping_saturating_adv3_option_wrapping_u32_none_roundtrip() {
    let original: Option<Wrapping<u32>> = None;
    let encoded = encode_to_vec(&original).expect("encode Option<Wrapping<u32>>(None) failed");
    let (decoded, _bytes): (Option<Wrapping<u32>>, usize) =
        decode_from_slice(&encoded).expect("decode Option<Wrapping<u32>>(None) failed");
    assert_eq!(original, decoded);
    assert_eq!(decoded, None);
}

// ===== 12. Saturating<u32> zero roundtrip =====

#[test]
fn test_wrapping_saturating_adv3_saturating_u32_zero_roundtrip() {
    let original = Saturating(0u32);
    let encoded = encode_to_vec(&original).expect("encode Saturating<u32>(0) failed");
    let (decoded, _bytes): (Saturating<u32>, usize) =
        decode_from_slice(&encoded).expect("decode Saturating<u32>(0) failed");
    assert_eq!(original, decoded);
    assert_eq!(decoded.0, 0u32);
}

// ===== 13. Saturating<u32> max value roundtrip =====

#[test]
fn test_wrapping_saturating_adv3_saturating_u32_max_roundtrip() {
    let original = Saturating(u32::MAX);
    let encoded = encode_to_vec(&original).expect("encode Saturating<u32>(MAX) failed");
    let (decoded, _bytes): (Saturating<u32>, usize) =
        decode_from_slice(&encoded).expect("decode Saturating<u32>(MAX) failed");
    assert_eq!(original, decoded);
    assert_eq!(decoded.0, u32::MAX);
}

// ===== 14. Saturating<u64> roundtrip =====

#[test]
fn test_wrapping_saturating_adv3_saturating_u64_roundtrip() {
    let original = Saturating(0xFEDC_BA98_7654_3210u64);
    let encoded = encode_to_vec(&original).expect("encode Saturating<u64> failed");
    let (decoded, _bytes): (Saturating<u64>, usize) =
        decode_from_slice(&encoded).expect("decode Saturating<u64> failed");
    assert_eq!(original, decoded);
    assert_eq!(decoded.0, 0xFEDC_BA98_7654_3210u64);
}

// ===== 15. Saturating<i32> negative value roundtrip =====

#[test]
fn test_wrapping_saturating_adv3_saturating_i32_negative_roundtrip() {
    let original = Saturating(-999_999i32);
    let encoded = encode_to_vec(&original).expect("encode Saturating<i32>(-999999) failed");
    let (decoded, _bytes): (Saturating<i32>, usize) =
        decode_from_slice(&encoded).expect("decode Saturating<i32>(-999999) failed");
    assert_eq!(original, decoded);
    assert_eq!(decoded.0, -999_999i32);
}

// ===== 16. Saturating<u8> same wire bytes as raw u8 =====

#[test]
fn test_wrapping_saturating_adv3_saturating_u8_same_wire_as_raw() {
    let value = 128u8;
    let saturating = Saturating(value);
    let s_bytes = encode_to_vec(&saturating).expect("encode Saturating<u8> failed");
    let raw_bytes = encode_to_vec(&value).expect("encode raw u8 failed");
    assert_eq!(
        s_bytes, raw_bytes,
        "Saturating<u8> must produce identical bytes to the inner u8"
    );
}

// ===== 17. Vec<Saturating<u32>> roundtrip =====

#[test]
fn test_wrapping_saturating_adv3_vec_saturating_u32_roundtrip() {
    let original: Vec<Saturating<u32>> = vec![
        Saturating(0u32),
        Saturating(100u32),
        Saturating(65535u32),
        Saturating(u32::MAX),
    ];
    let encoded = encode_to_vec(&original).expect("encode Vec<Saturating<u32>> failed");
    let (decoded, _bytes): (Vec<Saturating<u32>>, usize) =
        decode_from_slice(&encoded).expect("decode Vec<Saturating<u32>> failed");
    assert_eq!(original, decoded);
}

// ===== 18. Saturating<u32> fixed-int config (4 bytes) =====

#[test]
fn test_wrapping_saturating_adv3_saturating_u32_fixed_int_4_bytes() {
    let cfg = config::standard().with_fixed_int_encoding();
    let original = Saturating(0x1234_5678u32);
    let encoded =
        encode_to_vec_with_config(&original, cfg).expect("encode Saturating<u32> fixed_int failed");
    assert_eq!(
        encoded.len(),
        4,
        "Saturating<u32> with fixed_int must be exactly 4 bytes"
    );
    let (decoded, consumed): (Saturating<u32>, usize) =
        decode_from_slice_with_config(&encoded, cfg)
            .expect("decode Saturating<u32> fixed_int failed");
    assert_eq!(original, decoded);
    assert_eq!(consumed, 4);
}

// ===== 19. Struct with Wrapping<u32> and Saturating<u64> fields roundtrip =====

#[derive(Debug, PartialEq, Encode, Decode)]
struct MixedWrappers {
    w: Wrapping<u32>,
    s: Saturating<u64>,
}

#[test]
fn test_wrapping_saturating_adv3_struct_mixed_wrappers_roundtrip() {
    let original = MixedWrappers {
        w: Wrapping(0xDEAD_BEEFu32),
        s: Saturating(0xCAFE_BABE_1234_5678u64),
    };
    let encoded = encode_to_vec(&original).expect("encode MixedWrappers failed");
    let (decoded, _bytes): (MixedWrappers, usize) =
        decode_from_slice(&encoded).expect("decode MixedWrappers failed");
    assert_eq!(original, decoded);
    assert_eq!(decoded.w, Wrapping(0xDEAD_BEEFu32));
    assert_eq!(decoded.s, Saturating(0xCAFE_BABE_1234_5678u64));
}

// ===== 20. Wrapping<u32> and Saturating<u32> same wire bytes for same value =====

#[test]
fn test_wrapping_saturating_adv3_wrapping_and_saturating_same_wire_bytes() {
    let value = 0x5A5A_A5A5u32;
    let w_bytes = encode_to_vec(&Wrapping(value)).expect("encode Wrapping<u32> failed");
    let s_bytes = encode_to_vec(&Saturating(value)).expect("encode Saturating<u32> failed");
    let raw_bytes = encode_to_vec(&value).expect("encode raw u32 failed");
    assert_eq!(
        w_bytes, s_bytes,
        "Wrapping<u32> and Saturating<u32> must produce identical bytes for the same inner value"
    );
    assert_eq!(
        w_bytes, raw_bytes,
        "both wrapper types must match raw u32 encoding"
    );
}

// ===== 21. (Wrapping<u32>, Saturating<u32>) tuple roundtrip =====

#[test]
fn test_wrapping_saturating_adv3_tuple_wrapping_saturating_roundtrip() {
    let original: (Wrapping<u32>, Saturating<u32>) = (Wrapping(111u32), Saturating(222u32));
    let encoded = encode_to_vec(&original).expect("encode (Wrapping<u32>, Saturating<u32>) failed");
    let (decoded, _bytes): ((Wrapping<u32>, Saturating<u32>), usize) =
        decode_from_slice(&encoded).expect("decode (Wrapping<u32>, Saturating<u32>) failed");
    assert_eq!(original, decoded);
    assert_eq!(decoded.0, Wrapping(111u32));
    assert_eq!(decoded.1, Saturating(222u32));
}

// ===== 22. Wrapping<i128> roundtrip =====

#[test]
fn test_wrapping_saturating_adv3_wrapping_i128_roundtrip() {
    let original = Wrapping(i128::MIN);
    let encoded = encode_to_vec(&original).expect("encode Wrapping<i128>(MIN) failed");
    let (decoded, _bytes): (Wrapping<i128>, usize) =
        decode_from_slice(&encoded).expect("decode Wrapping<i128>(MIN) failed");
    assert_eq!(original, decoded);
    assert_eq!(decoded.0, i128::MIN);

    let original_max = Wrapping(i128::MAX);
    let encoded_max = encode_to_vec(&original_max).expect("encode Wrapping<i128>(MAX) failed");
    let (decoded_max, _bytes2): (Wrapping<i128>, usize) =
        decode_from_slice(&encoded_max).expect("decode Wrapping<i128>(MAX) failed");
    assert_eq!(original_max, decoded_max);
    assert_eq!(decoded_max.0, i128::MAX);
}
