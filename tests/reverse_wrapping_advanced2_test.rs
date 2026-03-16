//! Advanced tests for Reverse<T> and Wrapping<T> serialization in OxiCode — set advanced2.

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
use std::cmp::Reverse;
use std::num::Wrapping;

// ===== 1. Reverse<u32> basic roundtrip =====

#[test]
fn test_reverse_u32_roundtrip() {
    let original = Reverse(42u32);
    let enc = encode_to_vec(&original).expect("encode Reverse(42u32) failed");
    let (val, _): (Reverse<u32>, usize) =
        decode_from_slice(&enc).expect("decode Reverse(42u32) failed");
    assert_eq!(original, val);
}

// ===== 2. Reverse<u32> zero roundtrip =====

#[test]
fn test_reverse_u32_zero_roundtrip() {
    let original = Reverse(0u32);
    let enc = encode_to_vec(&original).expect("encode Reverse(0u32) failed");
    let (val, _): (Reverse<u32>, usize) =
        decode_from_slice(&enc).expect("decode Reverse(0u32) failed");
    assert_eq!(original, val);
}

// ===== 3. Reverse<u32> MAX roundtrip =====

#[test]
fn test_reverse_u32_max_roundtrip() {
    let original = Reverse(u32::MAX);
    let enc = encode_to_vec(&original).expect("encode Reverse(u32::MAX) failed");
    let (val, _): (Reverse<u32>, usize) =
        decode_from_slice(&enc).expect("decode Reverse(u32::MAX) failed");
    assert_eq!(original, val);
    assert_eq!(val.0, u32::MAX);
}

// ===== 4. Reverse<u64> roundtrip =====

#[test]
fn test_reverse_u64_roundtrip() {
    let original = Reverse(123_456_789u64);
    let enc = encode_to_vec(&original).expect("encode Reverse(123456789u64) failed");
    let (val, _): (Reverse<u64>, usize) =
        decode_from_slice(&enc).expect("decode Reverse(123456789u64) failed");
    assert_eq!(original, val);
}

// ===== 5. Reverse<String> roundtrip =====

#[test]
fn test_reverse_string_roundtrip() {
    let original = Reverse("hello".to_string());
    let enc = encode_to_vec(&original).expect("encode Reverse(String) failed");
    let (val, _): (Reverse<String>, usize) =
        decode_from_slice(&enc).expect("decode Reverse(String) failed");
    assert_eq!(original, val);
    assert_eq!(val.0, "hello");
}

// ===== 6. Reverse<u32> encodes same bytes as inner u32 =====

#[test]
fn test_reverse_same_bytes_as_inner() {
    let raw: u32 = 42;
    let wrapped = Reverse(42u32);
    let raw_bytes = encode_to_vec(&raw).expect("encode raw u32 failed");
    let wrapped_bytes = encode_to_vec(&wrapped).expect("encode Reverse(42u32) failed");
    assert_eq!(
        raw_bytes, wrapped_bytes,
        "Reverse<u32> must encode identically to raw u32"
    );
}

// ===== 7. Reverse<u32> consumed equals encoded length =====

#[test]
fn test_reverse_consumed_equals_len() {
    let original = Reverse(999u32);
    let enc = encode_to_vec(&original).expect("encode failed");
    let (_, consumed): (Reverse<u32>, usize) = decode_from_slice(&enc).expect("decode failed");
    assert_eq!(consumed, enc.len());
}

// ===== 8. Vec<Reverse<u32>> roundtrip =====

#[test]
fn test_vec_reverse_roundtrip() {
    let original: Vec<Reverse<u32>> = vec![Reverse(10u32), Reverse(200u32), Reverse(3000u32)];
    let enc = encode_to_vec(&original).expect("encode Vec<Reverse<u32>> failed");
    let (val, _): (Vec<Reverse<u32>>, usize) =
        decode_from_slice(&enc).expect("decode Vec<Reverse<u32>> failed");
    assert_eq!(original, val);
}

// ===== 9. Option<Reverse<u32>> Some roundtrip =====

#[test]
fn test_option_reverse_some_roundtrip() {
    let original: Option<Reverse<u32>> = Some(Reverse(77u32));
    let enc = encode_to_vec(&original).expect("encode Option<Reverse<u32>>(Some) failed");
    let (val, _): (Option<Reverse<u32>>, usize) =
        decode_from_slice(&enc).expect("decode Option<Reverse<u32>>(Some) failed");
    assert_eq!(original, val);
}

// ===== 10. Option<Reverse<u32>> None roundtrip =====

#[test]
fn test_option_reverse_none_roundtrip() {
    let original: Option<Reverse<u32>> = None;
    let enc = encode_to_vec(&original).expect("encode Option<Reverse<u32>>(None) failed");
    let (val, _): (Option<Reverse<u32>>, usize) =
        decode_from_slice(&enc).expect("decode Option<Reverse<u32>>(None) failed");
    assert_eq!(original, val);
    assert!(val.is_none());
}

// ===== 11. Reverse<u32> MAX with fixed_int_encoding config =====

#[test]
fn test_reverse_fixed_int_config() {
    let original = Reverse(u32::MAX);
    let cfg = config::standard().with_fixed_int_encoding();
    let enc = encode_to_vec_with_config(&original, cfg)
        .expect("fixed_int encode Reverse(u32::MAX) failed");
    assert_eq!(enc.len(), 4, "fixed_int Reverse<u32> must be 4 bytes");
    let (val, consumed): (Reverse<u32>, usize) = decode_from_slice_with_config(&enc, cfg)
        .expect("fixed_int decode Reverse(u32::MAX) failed");
    assert_eq!(original, val);
    assert_eq!(consumed, 4);
}

// ===== 12. Wrapping<u32> basic roundtrip =====

#[test]
fn test_wrapping_u32_roundtrip() {
    let original = Wrapping(42u32);
    let enc = encode_to_vec(&original).expect("encode Wrapping(42u32) failed");
    let (val, _): (Wrapping<u32>, usize) =
        decode_from_slice(&enc).expect("decode Wrapping(42u32) failed");
    assert_eq!(original, val);
}

// ===== 13. Wrapping<u32> zero roundtrip =====

#[test]
fn test_wrapping_u32_zero_roundtrip() {
    let original = Wrapping(0u32);
    let enc = encode_to_vec(&original).expect("encode Wrapping(0u32) failed");
    let (val, _): (Wrapping<u32>, usize) =
        decode_from_slice(&enc).expect("decode Wrapping(0u32) failed");
    assert_eq!(original, val);
    assert_eq!(val.0, 0u32);
}

// ===== 14. Wrapping<u32> MAX roundtrip =====

#[test]
fn test_wrapping_u32_max_roundtrip() {
    let original = Wrapping(u32::MAX);
    let enc = encode_to_vec(&original).expect("encode Wrapping(u32::MAX) failed");
    let (val, _): (Wrapping<u32>, usize) =
        decode_from_slice(&enc).expect("decode Wrapping(u32::MAX) failed");
    assert_eq!(original, val);
    assert_eq!(val.0, u32::MAX);
}

// ===== 15. Wrapping<u64> roundtrip =====

#[test]
fn test_wrapping_u64_roundtrip() {
    let original = Wrapping(999_999u64);
    let enc = encode_to_vec(&original).expect("encode Wrapping(999999u64) failed");
    let (val, _): (Wrapping<u64>, usize) =
        decode_from_slice(&enc).expect("decode Wrapping(999999u64) failed");
    assert_eq!(original, val);
}

// ===== 16. Wrapping<u8> 255 roundtrip =====

#[test]
fn test_wrapping_u8_roundtrip() {
    let original = Wrapping(255u8);
    let enc = encode_to_vec(&original).expect("encode Wrapping(255u8) failed");
    let (val, _): (Wrapping<u8>, usize) =
        decode_from_slice(&enc).expect("decode Wrapping(255u8) failed");
    assert_eq!(original, val);
    assert_eq!(val.0, 255u8);
}

// ===== 17. Wrapping<i32> negative roundtrip =====

#[test]
fn test_wrapping_i32_negative_roundtrip() {
    let original = Wrapping(-1i32);
    let enc = encode_to_vec(&original).expect("encode Wrapping(-1i32) failed");
    let (val, _): (Wrapping<i32>, usize) =
        decode_from_slice(&enc).expect("decode Wrapping(-1i32) failed");
    assert_eq!(original, val);
    assert_eq!(val.0, -1i32);
}

// ===== 18. Wrapping<u32> encodes same bytes as inner u32 =====

#[test]
fn test_wrapping_same_bytes_as_inner() {
    let raw: u32 = 42;
    let wrapped = Wrapping(42u32);
    let raw_bytes = encode_to_vec(&raw).expect("encode raw u32 failed");
    let wrapped_bytes = encode_to_vec(&wrapped).expect("encode Wrapping(42u32) failed");
    assert_eq!(
        raw_bytes, wrapped_bytes,
        "Wrapping<u32> must encode identically to raw u32"
    );
}

// ===== 19. Wrapping<u32> consumed equals encoded length =====

#[test]
fn test_wrapping_consumed_equals_len() {
    let original = Wrapping(500u32);
    let enc = encode_to_vec(&original).expect("encode failed");
    let (_, consumed): (Wrapping<u32>, usize) = decode_from_slice(&enc).expect("decode failed");
    assert_eq!(consumed, enc.len());
}

// ===== 20. Vec<Wrapping<u32>> roundtrip =====

#[test]
fn test_vec_wrapping_roundtrip() {
    let original: Vec<Wrapping<u32>> = vec![Wrapping(1u32), Wrapping(2u32), Wrapping(3u32)];
    let enc = encode_to_vec(&original).expect("encode Vec<Wrapping<u32>> failed");
    let (val, _): (Vec<Wrapping<u32>>, usize) =
        decode_from_slice(&enc).expect("decode Vec<Wrapping<u32>> failed");
    assert_eq!(original, val);
}

// ===== 21. Wrapping<u32> MAX with fixed_int_encoding config =====

#[test]
fn test_wrapping_fixed_int_config() {
    let original = Wrapping(u32::MAX);
    let cfg = config::standard().with_fixed_int_encoding();
    let enc = encode_to_vec_with_config(&original, cfg)
        .expect("fixed_int encode Wrapping(u32::MAX) failed");
    assert_eq!(enc.len(), 4, "fixed_int Wrapping<u32> must be 4 bytes");
    let (val, consumed): (Wrapping<u32>, usize) = decode_from_slice_with_config(&enc, cfg)
        .expect("fixed_int decode Wrapping(u32::MAX) failed");
    assert_eq!(original, val);
    assert_eq!(consumed, 4);
}

// ===== 22. Reverse<Wrapping<u32>> nested roundtrip =====

#[test]
fn test_reverse_wrapping_nested() {
    let original = Reverse(Wrapping(42u32));
    let enc = encode_to_vec(&original).expect("encode Reverse(Wrapping(42u32)) failed");
    let (val, _): (Reverse<Wrapping<u32>>, usize) =
        decode_from_slice(&enc).expect("decode Reverse(Wrapping(42u32)) failed");
    assert_eq!(original, val);
    assert_eq!(val.0 .0, 42u32);
}
