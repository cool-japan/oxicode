//! Advanced tests for `Wrapping<T>` and `Saturating<T>` encoding/decoding.
//!
//! Covers roundtrips, byte-level equality with raw types, collections,
//! Option wrappers, fixed-int config encoding size, and consumed byte counts.

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
use std::num::{Saturating, Wrapping};

use oxicode::{
    config, decode_from_slice, decode_from_slice_with_config, encode_to_vec,
    encode_to_vec_with_config,
};

// ===== 1. Wrapping<u32>(42) roundtrip =====

#[test]
fn test_wrapping_u32_42_roundtrip() {
    let original = Wrapping(42u32);
    let enc = encode_to_vec(&original).expect("encode Wrapping<u32>(42) failed");
    let (val, _): (Wrapping<u32>, _) =
        decode_from_slice(&enc).expect("decode Wrapping<u32>(42) failed");
    assert_eq!(val, original);
}

// ===== 2. Wrapping<u32>(0) roundtrip =====

#[test]
fn test_wrapping_u32_zero_roundtrip() {
    let original = Wrapping(0u32);
    let enc = encode_to_vec(&original).expect("encode Wrapping<u32>(0) failed");
    let (val, _): (Wrapping<u32>, _) =
        decode_from_slice(&enc).expect("decode Wrapping<u32>(0) failed");
    assert_eq!(val, original);
}

// ===== 3. Wrapping<u32>(u32::MAX) roundtrip =====

#[test]
fn test_wrapping_u32_max_roundtrip() {
    let original = Wrapping(u32::MAX);
    let enc = encode_to_vec(&original).expect("encode Wrapping<u32>(MAX) failed");
    let (val, _): (Wrapping<u32>, _) =
        decode_from_slice(&enc).expect("decode Wrapping<u32>(MAX) failed");
    assert_eq!(val, original);
}

// ===== 4. Wrapping<i32>(-1) roundtrip =====

#[test]
fn test_wrapping_i32_neg1_roundtrip() {
    let original = Wrapping(-1i32);
    let enc = encode_to_vec(&original).expect("encode Wrapping<i32>(-1) failed");
    let (val, _): (Wrapping<i32>, _) =
        decode_from_slice(&enc).expect("decode Wrapping<i32>(-1) failed");
    assert_eq!(val, original);
}

// ===== 5. Wrapping<u64>(u64::MAX) roundtrip =====

#[test]
fn test_wrapping_u64_max_roundtrip() {
    let original = Wrapping(u64::MAX);
    let enc = encode_to_vec(&original).expect("encode Wrapping<u64>(MAX) failed");
    let (val, _): (Wrapping<u64>, _) =
        decode_from_slice(&enc).expect("decode Wrapping<u64>(MAX) failed");
    assert_eq!(val, original);
}

// ===== 6. Wrapping<u8>(255) roundtrip =====

#[test]
fn test_wrapping_u8_255_roundtrip() {
    let original = Wrapping(255u8);
    let enc = encode_to_vec(&original).expect("encode Wrapping<u8>(255) failed");
    let (val, _): (Wrapping<u8>, _) =
        decode_from_slice(&enc).expect("decode Wrapping<u8>(255) failed");
    assert_eq!(val, original);
}

// ===== 7. Saturating<u32>(42) roundtrip =====

#[test]
fn test_saturating_u32_42_roundtrip() {
    let original = Saturating(42u32);
    let enc = encode_to_vec(&original).expect("encode Saturating<u32>(42) failed");
    let (val, _): (Saturating<u32>, _) =
        decode_from_slice(&enc).expect("decode Saturating<u32>(42) failed");
    assert_eq!(val, original);
}

// ===== 8. Saturating<u32>(0) roundtrip =====

#[test]
fn test_saturating_u32_zero_roundtrip() {
    let original = Saturating(0u32);
    let enc = encode_to_vec(&original).expect("encode Saturating<u32>(0) failed");
    let (val, _): (Saturating<u32>, _) =
        decode_from_slice(&enc).expect("decode Saturating<u32>(0) failed");
    assert_eq!(val, original);
}

// ===== 9. Saturating<u32>(u32::MAX) roundtrip =====

#[test]
fn test_saturating_u32_max_roundtrip() {
    let original = Saturating(u32::MAX);
    let enc = encode_to_vec(&original).expect("encode Saturating<u32>(MAX) failed");
    let (val, _): (Saturating<u32>, _) =
        decode_from_slice(&enc).expect("decode Saturating<u32>(MAX) failed");
    assert_eq!(val, original);
}

// ===== 10. Saturating<i32>(-1) roundtrip =====

#[test]
fn test_saturating_i32_neg1_roundtrip() {
    let original = Saturating(-1i32);
    let enc = encode_to_vec(&original).expect("encode Saturating<i32>(-1) failed");
    let (val, _): (Saturating<i32>, _) =
        decode_from_slice(&enc).expect("decode Saturating<i32>(-1) failed");
    assert_eq!(val, original);
}

// ===== 11. Saturating<u64>(u64::MAX) roundtrip =====

#[test]
fn test_saturating_u64_max_roundtrip() {
    let original = Saturating(u64::MAX);
    let enc = encode_to_vec(&original).expect("encode Saturating<u64>(MAX) failed");
    let (val, _): (Saturating<u64>, _) =
        decode_from_slice(&enc).expect("decode Saturating<u64>(MAX) failed");
    assert_eq!(val, original);
}

// ===== 12. Wrapping<u32> encodes same bytes as raw u32 =====

#[test]
fn test_wrapping_u32_bytes_equal_raw_u32() {
    let raw_value = 98765u32;
    let raw_enc = encode_to_vec(&raw_value).expect("encode raw u32 failed");
    let wrapping_enc = encode_to_vec(&Wrapping(raw_value)).expect("encode Wrapping<u32> failed");
    assert_eq!(
        raw_enc, wrapping_enc,
        "Wrapping<u32> must produce identical bytes to raw u32"
    );
}

// ===== 13. Saturating<u32> encodes same bytes as raw u32 =====

#[test]
fn test_saturating_u32_bytes_equal_raw_u32() {
    let raw_value = 11111u32;
    let raw_enc = encode_to_vec(&raw_value).expect("encode raw u32 failed");
    let saturating_enc =
        encode_to_vec(&Saturating(raw_value)).expect("encode Saturating<u32> failed");
    assert_eq!(
        raw_enc, saturating_enc,
        "Saturating<u32> must produce identical bytes to raw u32"
    );
}

// ===== 14. Wrapping<u32> and Saturating<u32> produce identical bytes for same value =====

#[test]
fn test_wrapping_and_saturating_u32_identical_bytes() {
    let value = 42u32;
    let wrapping_enc = encode_to_vec(&Wrapping(value)).expect("encode Wrapping<u32> failed");
    let saturating_enc = encode_to_vec(&Saturating(value)).expect("encode Saturating<u32> failed");
    assert_eq!(
        wrapping_enc, saturating_enc,
        "Wrapping<u32> and Saturating<u32> must produce identical bytes for the same inner value"
    );
}

// ===== 15. Vec<Wrapping<u32>> with 5 elements roundtrip =====

#[test]
fn test_vec_wrapping_u32_five_elements_roundtrip() {
    let original: Vec<Wrapping<u32>> = vec![
        Wrapping(0u32),
        Wrapping(1u32),
        Wrapping(100u32),
        Wrapping(u32::MAX / 2),
        Wrapping(u32::MAX),
    ];
    let enc = encode_to_vec(&original).expect("encode Vec<Wrapping<u32>> failed");
    let (val, _): (Vec<Wrapping<u32>>, _) =
        decode_from_slice(&enc).expect("decode Vec<Wrapping<u32>> failed");
    assert_eq!(val, original);
    assert_eq!(val.len(), 5);
}

// ===== 16. Vec<Saturating<u32>> with 5 elements roundtrip =====

#[test]
fn test_vec_saturating_u32_five_elements_roundtrip() {
    let original: Vec<Saturating<u32>> = vec![
        Saturating(0u32),
        Saturating(1u32),
        Saturating(999u32),
        Saturating(u32::MAX / 2),
        Saturating(u32::MAX),
    ];
    let enc = encode_to_vec(&original).expect("encode Vec<Saturating<u32>> failed");
    let (val, _): (Vec<Saturating<u32>>, _) =
        decode_from_slice(&enc).expect("decode Vec<Saturating<u32>> failed");
    assert_eq!(val, original);
    assert_eq!(val.len(), 5);
}

// ===== 17. Option<Wrapping<u32>> Some roundtrip =====

#[test]
fn test_option_wrapping_u32_some_roundtrip() {
    let original: Option<Wrapping<u32>> = Some(Wrapping(77u32));
    let enc = encode_to_vec(&original).expect("encode Option<Wrapping<u32>> Some failed");
    let (val, _): (Option<Wrapping<u32>>, _) =
        decode_from_slice(&enc).expect("decode Option<Wrapping<u32>> Some failed");
    assert_eq!(val, original);
    assert!(val.is_some());
    assert_eq!(val.expect("expected Some"), Wrapping(77u32));
}

// ===== 18. Option<Saturating<u32>> None roundtrip =====

#[test]
fn test_option_saturating_u32_none_roundtrip() {
    let original: Option<Saturating<u32>> = None;
    let enc = encode_to_vec(&original).expect("encode Option<Saturating<u32>> None failed");
    let (val, _): (Option<Saturating<u32>>, _) =
        decode_from_slice(&enc).expect("decode Option<Saturating<u32>> None failed");
    assert_eq!(val, original);
    assert!(val.is_none());
}

// ===== 19. Wrapping<u8> representing bool-like values 0 and 1 roundtrip =====

#[test]
fn test_wrapping_u8_bool_like_zero_and_one_roundtrip() {
    // bool is not directly supported by Wrapping arithmetic,
    // so we test Wrapping<u8> with the canonical bool-like values 0 and 1.
    let zero = Wrapping(0u8);
    let one = Wrapping(1u8);

    let enc_zero = encode_to_vec(&zero).expect("encode Wrapping<u8>(0) failed");
    let (val_zero, _): (Wrapping<u8>, _) =
        decode_from_slice(&enc_zero).expect("decode Wrapping<u8>(0) failed");
    assert_eq!(val_zero, zero);
    assert_eq!(val_zero.0, 0u8);

    let enc_one = encode_to_vec(&one).expect("encode Wrapping<u8>(1) failed");
    let (val_one, _): (Wrapping<u8>, _) =
        decode_from_slice(&enc_one).expect("decode Wrapping<u8>(1) failed");
    assert_eq!(val_one, one);
    assert_eq!(val_one.0, 1u8);
}

// ===== 20. Fixed-int config with Wrapping<u32> — 4 bytes =====

#[test]
fn test_wrapping_u32_fixed_int_config_four_bytes() {
    let cfg = config::standard().with_fixed_int_encoding();
    let value = Wrapping(123u32);
    let enc = encode_to_vec_with_config(&value, cfg)
        .expect("encode Wrapping<u32> with fixed-int config failed");
    // u32 with fixed encoding is always 4 bytes
    assert_eq!(
        enc.len(),
        4,
        "fixed-int encoded Wrapping<u32> must be exactly 4 bytes"
    );
    let (val, _): (Wrapping<u32>, _) = decode_from_slice_with_config(&enc, cfg)
        .expect("decode Wrapping<u32> with fixed-int config failed");
    assert_eq!(val, value);
}

// ===== 21. Fixed-int config with Saturating<u64> — 8 bytes =====

#[test]
fn test_saturating_u64_fixed_int_config_eight_bytes() {
    let cfg = config::standard().with_fixed_int_encoding();
    let value = Saturating(0xDEAD_BEEF_CAFE_BABEu64);
    let enc = encode_to_vec_with_config(&value, cfg)
        .expect("encode Saturating<u64> with fixed-int config failed");
    // u64 with fixed encoding is always 8 bytes
    assert_eq!(
        enc.len(),
        8,
        "fixed-int encoded Saturating<u64> must be exactly 8 bytes"
    );
    let (val, _): (Saturating<u64>, _) = decode_from_slice_with_config(&enc, cfg)
        .expect("decode Saturating<u64> with fixed-int config failed");
    assert_eq!(val, value);
}

// ===== 22. Consumed bytes equals encoded length for Wrapping<u64> =====

#[test]
fn test_wrapping_u64_consumed_bytes_equals_encoded_length() {
    let value = Wrapping(0x0102_0304_0506_0708u64);
    let enc = encode_to_vec(&value).expect("encode Wrapping<u64> failed");
    let (val, consumed): (Wrapping<u64>, _) =
        decode_from_slice(&enc).expect("decode Wrapping<u64> failed");
    assert_eq!(val, value);
    assert_eq!(
        consumed,
        enc.len(),
        "consumed bytes must equal the full encoded length"
    );
}
