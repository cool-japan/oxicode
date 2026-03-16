//! Advanced tests for Wrapping<T> and Reverse<T> serialization — new angles set 2.

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
use std::cmp::Reverse;
use std::num::Wrapping;

use oxicode::{
    config, decode_from_slice, decode_from_slice_with_config, encode_to_vec,
    encode_to_vec_with_config, Decode, Encode,
};

// ===== 1. Wrapping<u8> roundtrip value=0 =====

#[test]
fn test_wra2_wrapping_u8_zero() {
    let original = Wrapping(0u8);
    let encoded = encode_to_vec(&original).expect("encode failed");
    let (decoded, _): (Wrapping<u8>, usize) = decode_from_slice(&encoded).expect("decode failed");
    assert_eq!(original, decoded);
}

// ===== 2. Wrapping<u8> roundtrip value=255 =====

#[test]
fn test_wra2_wrapping_u8_max() {
    let original = Wrapping(255u8);
    let encoded = encode_to_vec(&original).expect("encode failed");
    let (decoded, _): (Wrapping<u8>, usize) = decode_from_slice(&encoded).expect("decode failed");
    assert_eq!(original, decoded);
}

// ===== 3. Wrapping<u32> roundtrip =====

#[test]
fn test_wra2_wrapping_u32_roundtrip() {
    let original = Wrapping(987_654_321u32);
    let encoded = encode_to_vec(&original).expect("encode failed");
    let (decoded, _): (Wrapping<u32>, usize) = decode_from_slice(&encoded).expect("decode failed");
    assert_eq!(original, decoded);
}

// ===== 4. Wrapping<i32> roundtrip negative =====

#[test]
fn test_wra2_wrapping_i32_negative() {
    let original = Wrapping(-42_000i32);
    let encoded = encode_to_vec(&original).expect("encode failed");
    let (decoded, _): (Wrapping<i32>, usize) = decode_from_slice(&encoded).expect("decode failed");
    assert_eq!(original, decoded);
}

// ===== 5. Wrapping<u64> roundtrip =====

#[test]
fn test_wra2_wrapping_u64_roundtrip() {
    let original = Wrapping(0xDEAD_BEEF_CAFE_BABEu64);
    let encoded = encode_to_vec(&original).expect("encode failed");
    let (decoded, _): (Wrapping<u64>, usize) = decode_from_slice(&encoded).expect("decode failed");
    assert_eq!(original, decoded);
}

// ===== 6. Wrapping<i64> roundtrip min/max =====

#[test]
fn test_wra2_wrapping_i64_min_max() {
    for val in [i64::MIN, i64::MAX, -1i64, 0i64] {
        let original = Wrapping(val);
        let encoded = encode_to_vec(&original).expect("encode failed");
        let (decoded, _): (Wrapping<i64>, usize) =
            decode_from_slice(&encoded).expect("decode failed");
        assert_eq!(original, decoded);
    }
}

// ===== 7. Vec<Wrapping<u32>> roundtrip =====

#[test]
fn test_wra2_vec_wrapping_u32_roundtrip() {
    let original: Vec<Wrapping<u32>> = vec![
        Wrapping(0u32),
        Wrapping(1u32),
        Wrapping(u32::MAX / 2),
        Wrapping(u32::MAX),
    ];
    let encoded = encode_to_vec(&original).expect("encode failed");
    let (decoded, _): (Vec<Wrapping<u32>>, usize) =
        decode_from_slice(&encoded).expect("decode failed");
    assert_eq!(original, decoded);
}

// ===== 8. Option<Wrapping<u32>> Some =====

#[test]
fn test_wra2_option_wrapping_u32_some() {
    let original: Option<Wrapping<u32>> = Some(Wrapping(77u32));
    let encoded = encode_to_vec(&original).expect("encode failed");
    let (decoded, _): (Option<Wrapping<u32>>, usize) =
        decode_from_slice(&encoded).expect("decode failed");
    assert_eq!(original, decoded);
}

// ===== 9. Option<Wrapping<u32>> None =====

#[test]
fn test_wra2_option_wrapping_u32_none() {
    let original: Option<Wrapping<u32>> = None;
    let encoded = encode_to_vec(&original).expect("encode failed");
    let (decoded, _): (Option<Wrapping<u32>>, usize) =
        decode_from_slice(&encoded).expect("decode failed");
    assert_eq!(original, decoded);
}

// ===== 10. Struct with Wrapping field =====

#[derive(Debug, PartialEq, Encode, Decode)]
struct WrappingHolder {
    counter: Wrapping<u32>,
    label: u8,
}

#[test]
fn test_wra2_struct_with_wrapping_field() {
    let original = WrappingHolder {
        counter: Wrapping(1_000_000u32),
        label: 7u8,
    };
    let encoded = encode_to_vec(&original).expect("encode failed");
    let (decoded, _): (WrappingHolder, usize) = decode_from_slice(&encoded).expect("decode failed");
    assert_eq!(original, decoded);
}

// ===== 11. Wrapping<u32> consumed == encoded.len() =====

#[test]
fn test_wra2_wrapping_u32_consumed_eq_len() {
    let original = Wrapping(500u32);
    let encoded = encode_to_vec(&original).expect("encode failed");
    let (_, consumed): (Wrapping<u32>, usize) = decode_from_slice(&encoded).expect("decode failed");
    assert_eq!(consumed, encoded.len());
}

// ===== 12. Wrapping<u8> wire size == 1 byte (same as raw u8) =====

#[test]
fn test_wra2_wrapping_u8_wire_size_is_one_byte() {
    let original = Wrapping(42u8);
    let wrapping_bytes = encode_to_vec(&original).expect("encode Wrapping<u8> failed");

    let raw: u8 = 42u8;
    let raw_bytes = encode_to_vec(&raw).expect("encode u8 failed");

    assert_eq!(
        wrapping_bytes.len(),
        1,
        "Wrapping<u8> must encode to 1 byte"
    );
    assert_eq!(wrapping_bytes.len(), raw_bytes.len());
    assert_eq!(wrapping_bytes, raw_bytes);
}

// ===== 13. Reverse<u32> roundtrip =====

#[test]
fn test_wra2_reverse_u32_roundtrip() {
    let original = Reverse(123_456u32);
    let encoded = encode_to_vec(&original).expect("encode failed");
    let (decoded, _): (Reverse<u32>, usize) = decode_from_slice(&encoded).expect("decode failed");
    assert_eq!(original, decoded);
}

// ===== 14. Reverse<i32> roundtrip =====

#[test]
fn test_wra2_reverse_i32_roundtrip() {
    let original = Reverse(-9_999i32);
    let encoded = encode_to_vec(&original).expect("encode failed");
    let (decoded, _): (Reverse<i32>, usize) = decode_from_slice(&encoded).expect("decode failed");
    assert_eq!(original, decoded);
}

// ===== 15. Reverse<u64> roundtrip =====

#[test]
fn test_wra2_reverse_u64_roundtrip() {
    let original = Reverse(u64::MAX / 3);
    let encoded = encode_to_vec(&original).expect("encode failed");
    let (decoded, _): (Reverse<u64>, usize) = decode_from_slice(&encoded).expect("decode failed");
    assert_eq!(original, decoded);
}

// ===== 16. Reverse<String> roundtrip =====

#[test]
fn test_wra2_reverse_string_roundtrip() {
    let original = Reverse(String::from("oxicode-wrapping-test"));
    let encoded = encode_to_vec(&original).expect("encode failed");
    let (decoded, _): (Reverse<String>, usize) =
        decode_from_slice(&encoded).expect("decode failed");
    assert_eq!(original, decoded);
}

// ===== 17. Vec<Reverse<u32>> roundtrip =====

#[test]
fn test_wra2_vec_reverse_u32_roundtrip() {
    let original: Vec<Reverse<u32>> = vec![Reverse(0u32), Reverse(100u32), Reverse(u32::MAX)];
    let encoded = encode_to_vec(&original).expect("encode failed");
    let (decoded, _): (Vec<Reverse<u32>>, usize) =
        decode_from_slice(&encoded).expect("decode failed");
    assert_eq!(original, decoded);
}

// ===== 18. Option<Reverse<u32>> Some =====

#[test]
fn test_wra2_option_reverse_u32_some() {
    let original: Option<Reverse<u32>> = Some(Reverse(42u32));
    let encoded = encode_to_vec(&original).expect("encode failed");
    let (decoded, _): (Option<Reverse<u32>>, usize) =
        decode_from_slice(&encoded).expect("decode failed");
    assert_eq!(original, decoded);
}

// ===== 19. Struct with Reverse field =====

#[derive(Debug, PartialEq, Encode, Decode)]
struct ReverseHolder {
    priority: Reverse<u32>,
    tag: u8,
}

#[test]
fn test_wra2_struct_with_reverse_field() {
    let original = ReverseHolder {
        priority: Reverse(255u32),
        tag: 3u8,
    };
    let encoded = encode_to_vec(&original).expect("encode failed");
    let (decoded, _): (ReverseHolder, usize) = decode_from_slice(&encoded).expect("decode failed");
    assert_eq!(original, decoded);
}

// ===== 20. Reverse<u32> consumed == encoded.len() =====

#[test]
fn test_wra2_reverse_u32_consumed_eq_len() {
    let original = Reverse(999u32);
    let encoded = encode_to_vec(&original).expect("encode failed");
    let (_, consumed): (Reverse<u32>, usize) = decode_from_slice(&encoded).expect("decode failed");
    assert_eq!(consumed, encoded.len());
}

// ===== 21. Fixed-int config with Wrapping<u32> =====

#[test]
fn test_wra2_wrapping_u32_fixed_int_config() {
    let original = Wrapping(12345u32);
    let cfg = config::standard().with_fixed_int_encoding();
    let encoded = encode_to_vec_with_config(&original, cfg).expect("fixed-int encode failed");
    // Fixed-int always uses 4 bytes for u32
    assert_eq!(encoded.len(), 4, "fixed-int Wrapping<u32> must be 4 bytes");
    let (decoded, consumed): (Wrapping<u32>, usize) =
        decode_from_slice_with_config(&encoded, cfg).expect("fixed-int decode failed");
    assert_eq!(original, decoded);
    assert_eq!(consumed, encoded.len());
}

// ===== 22. Big-endian config with Wrapping<u32> =====

#[test]
fn test_wra2_wrapping_u32_big_endian_config() {
    let original = Wrapping(0x0102_0304u32);
    let cfg = config::standard()
        .with_fixed_int_encoding()
        .with_big_endian();
    let encoded = encode_to_vec_with_config(&original, cfg).expect("big-endian encode failed");
    assert_eq!(
        encoded.len(),
        4,
        "big-endian fixed-int Wrapping<u32> must be 4 bytes"
    );
    // Big-endian byte order: most-significant byte first
    assert_eq!(encoded[0], 0x01u8);
    assert_eq!(encoded[1], 0x02u8);
    assert_eq!(encoded[2], 0x03u8);
    assert_eq!(encoded[3], 0x04u8);
    let (decoded, consumed): (Wrapping<u32>, usize) =
        decode_from_slice_with_config(&encoded, cfg).expect("big-endian decode failed");
    assert_eq!(original, decoded);
    assert_eq!(consumed, encoded.len());
}
