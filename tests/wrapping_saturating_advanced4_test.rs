//! Advanced roundtrip tests for Wrapping<T> and Saturating<T> encoding in OxiCode.
//! Both types encode transparently — same bytes as their inner T.

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
use std::num::{Saturating, Wrapping};

// ===== Wrapping<T> roundtrip tests =====

#[test]
fn test_wrapping_u32_value_42_roundtrip() {
    let val = Wrapping(42u32);
    let encoded = encode_to_vec(&val).expect("Failed to encode Wrapping<u32> 42");
    let (decoded, _): (Wrapping<u32>, usize) =
        decode_from_slice(&encoded).expect("Failed to decode Wrapping<u32> 42");
    assert_eq!(val, decoded);
}

#[test]
fn test_wrapping_u32_value_zero_roundtrip() {
    let val = Wrapping(0u32);
    let encoded = encode_to_vec(&val).expect("Failed to encode Wrapping<u32> 0");
    let (decoded, _): (Wrapping<u32>, usize) =
        decode_from_slice(&encoded).expect("Failed to decode Wrapping<u32> 0");
    assert_eq!(val, decoded);
}

#[test]
fn test_wrapping_u32_max_roundtrip() {
    let val = Wrapping(u32::MAX);
    let encoded = encode_to_vec(&val).expect("Failed to encode Wrapping<u32> MAX");
    let (decoded, _): (Wrapping<u32>, usize) =
        decode_from_slice(&encoded).expect("Failed to decode Wrapping<u32> MAX");
    assert_eq!(val, decoded);
}

#[test]
fn test_wrapping_i32_negative_roundtrip() {
    let val = Wrapping(-123456i32);
    let encoded = encode_to_vec(&val).expect("Failed to encode Wrapping<i32> negative");
    let (decoded, _): (Wrapping<i32>, usize) =
        decode_from_slice(&encoded).expect("Failed to decode Wrapping<i32> negative");
    assert_eq!(val, decoded);
}

#[test]
fn test_wrapping_u64_large_value_roundtrip() {
    let val = Wrapping(u64::MAX - 1_000_000u64);
    let encoded = encode_to_vec(&val).expect("Failed to encode Wrapping<u64> large");
    let (decoded, _): (Wrapping<u64>, usize) =
        decode_from_slice(&encoded).expect("Failed to decode Wrapping<u64> large");
    assert_eq!(val, decoded);
}

#[test]
fn test_wrapping_u8_roundtrip() {
    let val = Wrapping(255u8);
    let encoded = encode_to_vec(&val).expect("Failed to encode Wrapping<u8>");
    let (decoded, _): (Wrapping<u8>, usize) =
        decode_from_slice(&encoded).expect("Failed to decode Wrapping<u8>");
    assert_eq!(val, decoded);
}

#[test]
fn test_wrapping_i64_min_roundtrip() {
    let val = Wrapping(i64::MIN);
    let encoded = encode_to_vec(&val).expect("Failed to encode Wrapping<i64> MIN");
    let (decoded, _): (Wrapping<i64>, usize) =
        decode_from_slice(&encoded).expect("Failed to decode Wrapping<i64> MIN");
    assert_eq!(val, decoded);
}

// ===== Saturating<T> roundtrip tests =====

#[test]
fn test_saturating_u32_value_42_roundtrip() {
    let val = Saturating(42u32);
    let encoded = encode_to_vec(&val).expect("Failed to encode Saturating<u32> 42");
    let (decoded, _): (Saturating<u32>, usize) =
        decode_from_slice(&encoded).expect("Failed to decode Saturating<u32> 42");
    assert_eq!(val, decoded);
}

#[test]
fn test_saturating_u32_value_zero_roundtrip() {
    let val = Saturating(0u32);
    let encoded = encode_to_vec(&val).expect("Failed to encode Saturating<u32> 0");
    let (decoded, _): (Saturating<u32>, usize) =
        decode_from_slice(&encoded).expect("Failed to decode Saturating<u32> 0");
    assert_eq!(val, decoded);
}

#[test]
fn test_saturating_u32_max_roundtrip() {
    let val = Saturating(u32::MAX);
    let encoded = encode_to_vec(&val).expect("Failed to encode Saturating<u32> MAX");
    let (decoded, _): (Saturating<u32>, usize) =
        decode_from_slice(&encoded).expect("Failed to decode Saturating<u32> MAX");
    assert_eq!(val, decoded);
}

#[test]
fn test_saturating_i32_negative_roundtrip() {
    let val = Saturating(-987654i32);
    let encoded = encode_to_vec(&val).expect("Failed to encode Saturating<i32> negative");
    let (decoded, _): (Saturating<i32>, usize) =
        decode_from_slice(&encoded).expect("Failed to decode Saturating<i32> negative");
    assert_eq!(val, decoded);
}

#[test]
fn test_saturating_u64_large_value_roundtrip() {
    let val = Saturating(9_999_999_999_999u64);
    let encoded = encode_to_vec(&val).expect("Failed to encode Saturating<u64> large");
    let (decoded, _): (Saturating<u64>, usize) =
        decode_from_slice(&encoded).expect("Failed to decode Saturating<u64> large");
    assert_eq!(val, decoded);
}

// ===== Transparent encoding — same bytes as raw inner T =====

#[test]
fn test_wrapping_u32_encodes_same_bytes_as_raw_u32() {
    let inner = 77u32;
    let raw_bytes = encode_to_vec(&inner).expect("Failed to encode raw u32");
    let wrapped_bytes = encode_to_vec(&Wrapping(inner)).expect("Failed to encode Wrapping<u32>");
    assert_eq!(
        raw_bytes, wrapped_bytes,
        "Wrapping<u32> should produce identical bytes to raw u32"
    );
}

#[test]
fn test_saturating_u32_encodes_same_bytes_as_raw_u32() {
    let inner = 77u32;
    let raw_bytes = encode_to_vec(&inner).expect("Failed to encode raw u32");
    let saturating_bytes =
        encode_to_vec(&Saturating(inner)).expect("Failed to encode Saturating<u32>");
    assert_eq!(
        raw_bytes, saturating_bytes,
        "Saturating<u32> should produce identical bytes to raw u32"
    );
}

#[test]
fn test_wrapping_and_saturating_produce_same_bytes_for_same_value() {
    let inner = 12345u32;
    let wrapping_bytes = encode_to_vec(&Wrapping(inner)).expect("Failed to encode Wrapping<u32>");
    let saturating_bytes =
        encode_to_vec(&Saturating(inner)).expect("Failed to encode Saturating<u32>");
    assert_eq!(
        wrapping_bytes, saturating_bytes,
        "Wrapping and Saturating should produce identical bytes for the same inner value"
    );
}

// ===== Collection types =====

#[test]
fn test_vec_of_wrapping_u32_roundtrip() {
    let val: Vec<Wrapping<u32>> = vec![
        Wrapping(0u32),
        Wrapping(1u32),
        Wrapping(100u32),
        Wrapping(u32::MAX - 1),
        Wrapping(u32::MAX),
    ];
    let encoded = encode_to_vec(&val).expect("Failed to encode Vec<Wrapping<u32>>");
    let (decoded, _): (Vec<Wrapping<u32>>, usize) =
        decode_from_slice(&encoded).expect("Failed to decode Vec<Wrapping<u32>>");
    assert_eq!(val, decoded);
}

#[test]
fn test_vec_of_saturating_u32_roundtrip() {
    let val: Vec<Saturating<u32>> = vec![
        Saturating(0u32),
        Saturating(1u32),
        Saturating(100u32),
        Saturating(u32::MAX - 1),
        Saturating(u32::MAX),
    ];
    let encoded = encode_to_vec(&val).expect("Failed to encode Vec<Saturating<u32>>");
    let (decoded, _): (Vec<Saturating<u32>>, usize) =
        decode_from_slice(&encoded).expect("Failed to decode Vec<Saturating<u32>>");
    assert_eq!(val, decoded);
}

// ===== Option<Wrapping<T>> and Option<Saturating<T>> =====

#[test]
fn test_option_wrapping_u32_some_roundtrip() {
    let val: Option<Wrapping<u32>> = Some(Wrapping(42u32));
    let encoded = encode_to_vec(&val).expect("Failed to encode Option<Wrapping<u32>> Some");
    let (decoded, _): (Option<Wrapping<u32>>, usize) =
        decode_from_slice(&encoded).expect("Failed to decode Option<Wrapping<u32>> Some");
    assert_eq!(val, decoded);
}

#[test]
fn test_option_saturating_u32_none_roundtrip() {
    let val: Option<Saturating<u32>> = None;
    let encoded = encode_to_vec(&val).expect("Failed to encode Option<Saturating<u32>> None");
    let (decoded, _): (Option<Saturating<u32>>, usize) =
        decode_from_slice(&encoded).expect("Failed to decode Option<Saturating<u32>> None");
    assert_eq!(val, decoded);
}

// ===== Fixed-int config tests =====

#[test]
fn test_wrapping_u32_with_fixed_int_config() {
    let val = Wrapping(9999u32);
    let cfg = config::standard().with_fixed_int_encoding();
    let encoded = encode_to_vec_with_config(&val, cfg)
        .expect("Failed to encode Wrapping<u32> with fixed-int config");
    // With fixed-int encoding, u32 always occupies exactly 4 bytes
    assert_eq!(
        encoded.len(),
        4,
        "Wrapping<u32> with fixed-int config should encode to 4 bytes"
    );
    let (decoded, _): (Wrapping<u32>, usize) = decode_from_slice_with_config(&encoded, cfg)
        .expect("Failed to decode Wrapping<u32> with fixed-int config");
    assert_eq!(val, decoded);
}

#[test]
fn test_saturating_u64_with_fixed_int_config() {
    let val = Saturating(u64::MAX / 2);
    let cfg = config::standard().with_fixed_int_encoding();
    let encoded = encode_to_vec_with_config(&val, cfg)
        .expect("Failed to encode Saturating<u64> with fixed-int config");
    // With fixed-int encoding, u64 always occupies exactly 8 bytes
    assert_eq!(
        encoded.len(),
        8,
        "Saturating<u64> with fixed-int config should encode to 8 bytes"
    );
    let (decoded, _): (Saturating<u64>, usize) = decode_from_slice_with_config(&encoded, cfg)
        .expect("Failed to decode Saturating<u64> with fixed-int config");
    assert_eq!(val, decoded);
}

// ===== Consumed bytes verification =====

#[test]
fn test_consumed_bytes_equals_encoded_length() {
    let wrapping_val = Wrapping(42u32);
    let saturating_val = Saturating(42u32);

    let wrapping_bytes = encode_to_vec(&wrapping_val)
        .expect("Failed to encode Wrapping<u32> for consumed-bytes check");
    let saturating_bytes = encode_to_vec(&saturating_val)
        .expect("Failed to encode Saturating<u32> for consumed-bytes check");

    let (_, wrapping_consumed): (Wrapping<u32>, usize) = decode_from_slice(&wrapping_bytes)
        .expect("Failed to decode Wrapping<u32> for consumed-bytes check");
    let (_, saturating_consumed): (Saturating<u32>, usize) = decode_from_slice(&saturating_bytes)
        .expect("Failed to decode Saturating<u32> for consumed-bytes check");

    assert_eq!(
        wrapping_consumed,
        wrapping_bytes.len(),
        "Consumed bytes for Wrapping<u32> should equal encoded length"
    );
    assert_eq!(
        saturating_consumed,
        saturating_bytes.len(),
        "Consumed bytes for Saturating<u32> should equal encoded length"
    );
}
