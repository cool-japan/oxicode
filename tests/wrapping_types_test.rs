//! Tests for Wrapping<T> and Reverse<T> encode/decode roundtrips.

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

use oxicode::{Decode, Encode};

// ===== Helper macro for simple roundtrip assertions =====

macro_rules! assert_roundtrip {
    ($value:expr, $ty:ty) => {{
        let encoded = oxicode::encode_to_vec(&$value).expect("encode failed");
        let (decoded, _): ($ty, _) = oxicode::decode_from_slice(&encoded).expect("decode failed");
        assert_eq!($value, decoded);
    }};
}

// ===== 1. Wrapping<u8> roundtrip (boundary values) =====

#[test]
fn test_wrapping_types_wrapping_u8_zero() {
    assert_roundtrip!(Wrapping(0u8), Wrapping<u8>);
}

#[test]
fn test_wrapping_types_wrapping_u8_max() {
    assert_roundtrip!(Wrapping(255u8), Wrapping<u8>);
}

// ===== 2. Wrapping<u32> roundtrip =====

#[test]
fn test_wrapping_types_wrapping_u32() {
    assert_roundtrip!(Wrapping(0u32), Wrapping<u32>);
    assert_roundtrip!(Wrapping(42u32), Wrapping<u32>);
    assert_roundtrip!(Wrapping(u32::MAX), Wrapping<u32>);
}

// ===== 3. Wrapping<i64> roundtrip =====

#[test]
fn test_wrapping_types_wrapping_i64() {
    assert_roundtrip!(Wrapping(0i64), Wrapping<i64>);
    assert_roundtrip!(Wrapping(i64::MIN), Wrapping<i64>);
    assert_roundtrip!(Wrapping(i64::MAX), Wrapping<i64>);
    assert_roundtrip!(Wrapping(-1i64), Wrapping<i64>);
}

// ===== 4. Wrapping<u128> roundtrip =====

#[test]
fn test_wrapping_types_wrapping_u128() {
    assert_roundtrip!(Wrapping(0u128), Wrapping<u128>);
    assert_roundtrip!(Wrapping(u128::MAX), Wrapping<u128>);
    assert_roundtrip!(
        Wrapping(0xDEAD_BEEF_CAFE_BABE_0123_4567_89AB_CDEFu128),
        Wrapping<u128>
    );
}

// ===== 5. Reverse<u32> roundtrip =====

#[test]
fn test_wrapping_types_reverse_u32() {
    assert_roundtrip!(Reverse(0u32), Reverse<u32>);
    assert_roundtrip!(Reverse(99u32), Reverse<u32>);
    assert_roundtrip!(Reverse(u32::MAX), Reverse<u32>);
}

// ===== 6. Reverse<i32> roundtrip =====

#[test]
fn test_wrapping_types_reverse_i32() {
    assert_roundtrip!(Reverse(0i32), Reverse<i32>);
    assert_roundtrip!(Reverse(i32::MIN), Reverse<i32>);
    assert_roundtrip!(Reverse(i32::MAX), Reverse<i32>);
    assert_roundtrip!(Reverse(-1i32), Reverse<i32>);
}

// ===== 7. Reverse<String> roundtrip =====

#[test]
fn test_wrapping_types_reverse_string() {
    assert_roundtrip!(Reverse(String::from("")), Reverse<String>);
    assert_roundtrip!(Reverse(String::from("hello, oxicode")), Reverse<String>);
    assert_roundtrip!(
        Reverse(String::from("unicode: こんにちは")),
        Reverse<String>
    );
}

// ===== 8. Vec<Wrapping<u32>> roundtrip =====

#[test]
fn test_wrapping_types_vec_wrapping_u32() {
    let original: Vec<Wrapping<u32>> = vec![
        Wrapping(0u32),
        Wrapping(1u32),
        Wrapping(u32::MAX),
        Wrapping(100u32),
    ];
    assert_roundtrip!(original, Vec<Wrapping<u32>>);
}

#[test]
fn test_wrapping_types_vec_wrapping_u32_empty() {
    let original: Vec<Wrapping<u32>> = vec![];
    assert_roundtrip!(original, Vec<Wrapping<u32>>);
}

// ===== 9. Struct with Wrapping field using derive =====

#[derive(Debug, PartialEq, Encode, Decode)]
struct W {
    val: Wrapping<u32>,
}

#[test]
fn test_wrapping_types_derived_struct_with_wrapping_field() {
    let original = W {
        val: Wrapping(12345u32),
    };
    assert_roundtrip!(original, W);
}

#[test]
fn test_wrapping_types_derived_struct_wrapping_field_zero() {
    let original = W {
        val: Wrapping(0u32),
    };
    assert_roundtrip!(original, W);
}

#[test]
fn test_wrapping_types_derived_struct_wrapping_field_max() {
    let original = W {
        val: Wrapping(u32::MAX),
    };
    assert_roundtrip!(original, W);
}

// ===== 10. Reverse in a Vec: Vec<Reverse<u64>> roundtrip =====

#[test]
fn test_wrapping_types_vec_reverse_u64() {
    let original: Vec<Reverse<u64>> = vec![
        Reverse(0u64),
        Reverse(u64::MAX),
        Reverse(42u64),
        Reverse(1_000_000u64),
    ];
    assert_roundtrip!(original, Vec<Reverse<u64>>);
}

#[test]
fn test_wrapping_types_vec_reverse_u64_empty() {
    let original: Vec<Reverse<u64>> = vec![];
    assert_roundtrip!(original, Vec<Reverse<u64>>);
}

// ===== 11. Wrapping arithmetic preserved after encode/decode =====

#[test]
fn test_wrapping_types_wrapping_add_preserved() {
    // Compute a wrapping addition, encode the result, decode, verify value.
    let a = Wrapping(u32::MAX);
    let b = Wrapping(1u32);
    let sum = a + b; // wraps to 0

    assert_eq!(sum, Wrapping(0u32));

    let encoded = oxicode::encode_to_vec(&sum).expect("encode failed");
    let (decoded, _): (Wrapping<u32>, _) =
        oxicode::decode_from_slice(&encoded).expect("decode failed");

    assert_eq!(decoded, Wrapping(0u32));
    // Verify the wrapped inner value is semantically preserved
    assert_eq!(decoded.0, 0u32);
}

#[test]
fn test_wrapping_types_wrapping_sub_preserved() {
    let a = Wrapping(0u32);
    let b = Wrapping(1u32);
    let diff = a - b; // wraps to u32::MAX

    assert_eq!(diff, Wrapping(u32::MAX));

    let encoded = oxicode::encode_to_vec(&diff).expect("encode failed");
    let (decoded, _): (Wrapping<u32>, _) =
        oxicode::decode_from_slice(&encoded).expect("decode failed");

    assert_eq!(decoded, Wrapping(u32::MAX));
    assert_eq!(decoded.0, u32::MAX);
}

#[test]
fn test_wrapping_types_wrapping_mul_then_roundtrip() {
    // 200u8 * 2 = 400, wrapped to 400 % 256 = 144
    let a = Wrapping(200u8);
    let b = Wrapping(2u8);
    let product = a * b;

    assert_eq!(product.0, 144u8);

    let encoded = oxicode::encode_to_vec(&product).expect("encode failed");
    let (decoded, _): (Wrapping<u8>, _) =
        oxicode::decode_from_slice(&encoded).expect("decode failed");

    assert_eq!(decoded.0, 144u8);

    // Confirm further arithmetic continues to wrap correctly after decode
    let further = decoded + Wrapping(112u8); // 144 + 112 = 256 = 0
    assert_eq!(further.0, 0u8);
}

// ===== 12. BorrowDecode for Wrapping<u64> =====

#[test]
fn test_wrapping_types_borrow_decode_wrapping_u64() {
    // Wrapping<u64> has a BorrowDecode impl that delegates to its inner type.
    // Since u64 implements BorrowDecode (via impl_borrow_decode! macro),
    // the BorrowDecode impl for Wrapping<T> in de/impls.rs is exercised here.
    let original = Wrapping(0xCAFE_BABE_DEAD_BEEFu64);

    let encoded = oxicode::encode_to_vec(&original).expect("encode failed");
    let (decoded, _): (Wrapping<u64>, _) =
        oxicode::borrow_decode_from_slice(&encoded).expect("borrow_decode failed");

    assert_eq!(original, decoded);
}

#[test]
fn test_wrapping_types_borrow_decode_wrapping_u64_zero() {
    let original = Wrapping(0u64);

    let encoded = oxicode::encode_to_vec(&original).expect("encode failed");
    let (decoded, _): (Wrapping<u64>, _) =
        oxicode::borrow_decode_from_slice(&encoded).expect("borrow_decode failed");

    assert_eq!(original, decoded);
}

#[test]
fn test_wrapping_types_borrow_decode_wrapping_u64_max() {
    let original = Wrapping(u64::MAX);

    let encoded = oxicode::encode_to_vec(&original).expect("encode failed");
    let (decoded, _): (Wrapping<u64>, _) =
        oxicode::borrow_decode_from_slice(&encoded).expect("borrow_decode failed");

    assert_eq!(original, decoded);
}
