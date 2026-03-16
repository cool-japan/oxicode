//! Advanced tests for Bound<T> encoding in OxiCode.

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
use oxicode::{decode_from_slice, encode_to_vec, encoded_size};
use std::ops::Bound;

use oxicode_derive::{Decode, Encode};

#[derive(Debug, PartialEq, Encode, Decode)]
struct RangeBounds {
    start: Bound<u64>,
    end: Bound<u64>,
}

// 1. Bound::<u32>::Unbounded roundtrip
#[test]
fn test_bound_u32_unbounded_roundtrip() {
    let value: Bound<u32> = Bound::Unbounded;
    let encoded = encode_to_vec(&value).expect("encode Bound::Unbounded<u32>");
    let (decoded, _): (Bound<u32>, usize) =
        decode_from_slice(&encoded).expect("decode Bound::Unbounded<u32>");
    assert_eq!(value, decoded);
}

// 2. Bound::<u32>::Included(0) roundtrip
#[test]
fn test_bound_u32_included_zero_roundtrip() {
    let value: Bound<u32> = Bound::Included(0u32);
    let encoded = encode_to_vec(&value).expect("encode Bound::Included(0u32)");
    let (decoded, _): (Bound<u32>, usize) =
        decode_from_slice(&encoded).expect("decode Bound::Included(0u32)");
    assert_eq!(value, decoded);
}

// 3. Bound::<u32>::Excluded(0) roundtrip
#[test]
fn test_bound_u32_excluded_zero_roundtrip() {
    let value: Bound<u32> = Bound::Excluded(0u32);
    let encoded = encode_to_vec(&value).expect("encode Bound::Excluded(0u32)");
    let (decoded, _): (Bound<u32>, usize) =
        decode_from_slice(&encoded).expect("decode Bound::Excluded(0u32)");
    assert_eq!(value, decoded);
}

// 4. Bound::<u32>::Included(u32::MAX) roundtrip
#[test]
fn test_bound_u32_included_max_roundtrip() {
    let value: Bound<u32> = Bound::Included(u32::MAX);
    let encoded = encode_to_vec(&value).expect("encode Bound::Included(u32::MAX)");
    let (decoded, _): (Bound<u32>, usize) =
        decode_from_slice(&encoded).expect("decode Bound::Included(u32::MAX)");
    assert_eq!(value, decoded);
}

// 5. Bound::<u32>::Excluded(u32::MAX) roundtrip
#[test]
fn test_bound_u32_excluded_max_roundtrip() {
    let value: Bound<u32> = Bound::Excluded(u32::MAX);
    let encoded = encode_to_vec(&value).expect("encode Bound::Excluded(u32::MAX)");
    let (decoded, _): (Bound<u32>, usize) =
        decode_from_slice(&encoded).expect("decode Bound::Excluded(u32::MAX)");
    assert_eq!(value, decoded);
}

// 6. Bound::<i64>::Included(i64::MIN) roundtrip
#[test]
fn test_bound_i64_included_min_roundtrip() {
    let value: Bound<i64> = Bound::Included(i64::MIN);
    let encoded = encode_to_vec(&value).expect("encode Bound::Included(i64::MIN)");
    let (decoded, _): (Bound<i64>, usize) =
        decode_from_slice(&encoded).expect("decode Bound::Included(i64::MIN)");
    assert_eq!(value, decoded);
}

// 7. Bound::<i64>::Excluded(i64::MAX) roundtrip
#[test]
fn test_bound_i64_excluded_max_roundtrip() {
    let value: Bound<i64> = Bound::Excluded(i64::MAX);
    let encoded = encode_to_vec(&value).expect("encode Bound::Excluded(i64::MAX)");
    let (decoded, _): (Bound<i64>, usize) =
        decode_from_slice(&encoded).expect("decode Bound::Excluded(i64::MAX)");
    assert_eq!(value, decoded);
}

// 8. Bound::<String>::Included("alpha") roundtrip
#[test]
fn test_bound_string_included_alpha_roundtrip() {
    let value: Bound<String> = Bound::Included("alpha".to_string());
    let encoded = encode_to_vec(&value).expect("encode Bound::Included(alpha)");
    let (decoded, _): (Bound<String>, usize) =
        decode_from_slice(&encoded).expect("decode Bound::Included(alpha)");
    assert_eq!(value, decoded);
}

// 9. Bound::<String>::Excluded("omega") roundtrip
#[test]
fn test_bound_string_excluded_omega_roundtrip() {
    let value: Bound<String> = Bound::Excluded("omega".to_string());
    let encoded = encode_to_vec(&value).expect("encode Bound::Excluded(omega)");
    let (decoded, _): (Bound<String>, usize) =
        decode_from_slice(&encoded).expect("decode Bound::Excluded(omega)");
    assert_eq!(value, decoded);
}

// 10. Bound::<String>::Unbounded roundtrip
#[test]
fn test_bound_string_unbounded_roundtrip() {
    let value: Bound<String> = Bound::Unbounded;
    let encoded = encode_to_vec(&value).expect("encode Bound::Unbounded<String>");
    let (decoded, _): (Bound<String>, usize) =
        decode_from_slice(&encoded).expect("decode Bound::Unbounded<String>");
    assert_eq!(value, decoded);
}

// 11. Vec<Bound<u32>> with all three variants roundtrip
#[test]
fn test_vec_of_bounds_all_variants_roundtrip() {
    let value: Vec<Bound<u32>> = vec![
        Bound::Unbounded,
        Bound::Included(10u32),
        Bound::Excluded(20u32),
    ];
    let encoded = encode_to_vec(&value).expect("encode Vec<Bound<u32>>");
    let (decoded, _): (Vec<Bound<u32>>, usize) =
        decode_from_slice(&encoded).expect("decode Vec<Bound<u32>>");
    assert_eq!(value, decoded);
}

// 12. (Bound<u32>, Bound<u32>) tuple as a range pair roundtrip
#[test]
fn test_bound_u32_tuple_range_pair_roundtrip() {
    let value: (Bound<u32>, Bound<u32>) = (Bound::Included(5u32), Bound::Excluded(100u32));
    let encoded = encode_to_vec(&value).expect("encode (Bound<u32>, Bound<u32>)");
    let (decoded, _): ((Bound<u32>, Bound<u32>), usize) =
        decode_from_slice(&encoded).expect("decode (Bound<u32>, Bound<u32>)");
    assert_eq!(value, decoded);
}

// 13. Option<Bound<u64>> Some(Included) roundtrip
#[test]
fn test_option_bound_u64_some_included_roundtrip() {
    let value: Option<Bound<u64>> = Some(Bound::Included(42u64));
    let encoded = encode_to_vec(&value).expect("encode Some(Bound::Included(42u64))");
    let (decoded, _): (Option<Bound<u64>>, usize) =
        decode_from_slice(&encoded).expect("decode Some(Bound::Included(42u64))");
    assert_eq!(value, decoded);
}

// 14. Option<Bound<u64>> None roundtrip
#[test]
fn test_option_bound_u64_none_roundtrip() {
    let value: Option<Bound<u64>> = None;
    let encoded = encode_to_vec(&value).expect("encode None::<Bound<u64>>");
    let (decoded, _): (Option<Bound<u64>>, usize) =
        decode_from_slice(&encoded).expect("decode None::<Bound<u64>>");
    assert_eq!(value, decoded);
}

// 15. Bound<u8> Unbounded encodes as 1 byte (discriminant only, no payload)
#[test]
fn test_bound_u8_unbounded_encodes_one_byte() {
    let value: Bound<u8> = Bound::Unbounded;
    let encoded = encode_to_vec(&value).expect("encode Bound::Unbounded<u8>");
    assert_eq!(
        encoded.len(),
        1,
        "Bound::Unbounded should encode as exactly 1 byte (discriminant only)"
    );
}

// 16. Bound<u8> Included(42) encodes as 2 bytes (discriminant + value)
#[test]
fn test_bound_u8_included_encodes_two_bytes() {
    let value: Bound<u8> = Bound::Included(42u8);
    let encoded = encode_to_vec(&value).expect("encode Bound::Included(42u8)");
    assert_eq!(
        encoded.len(),
        2,
        "Bound::Included(u8) should encode as exactly 2 bytes (discriminant + payload)"
    );
}

// 17. Bound<u8> Excluded(42) encodes as 2 bytes
#[test]
fn test_bound_u8_excluded_encodes_two_bytes() {
    let value: Bound<u8> = Bound::Excluded(42u8);
    let encoded = encode_to_vec(&value).expect("encode Bound::Excluded(42u8)");
    assert_eq!(
        encoded.len(),
        2,
        "Bound::Excluded(u8) should encode as exactly 2 bytes (discriminant + payload)"
    );
}

// 18. Unbounded discriminant byte value verification (should be tag 0)
#[test]
fn test_unbounded_discriminant_is_zero() {
    let value: Bound<u8> = Bound::Unbounded;
    let encoded =
        encode_to_vec(&value).expect("encode Bound::Unbounded<u8> for discriminant check");
    assert_eq!(
        encoded[0], 0u8,
        "Bound::Unbounded discriminant byte should be 0"
    );
}

// 19. Included discriminant byte value verification (should be tag 1)
#[test]
fn test_included_discriminant_is_one() {
    let value: Bound<u8> = Bound::Included(99u8);
    let encoded =
        encode_to_vec(&value).expect("encode Bound::Included(99u8) for discriminant check");
    assert_eq!(
        encoded[0], 1u8,
        "Bound::Included discriminant byte should be 1"
    );
}

// 20. Excluded discriminant byte value verification (should be tag 2)
#[test]
fn test_excluded_discriminant_is_two() {
    let value: Bound<u8> = Bound::Excluded(77u8);
    let encoded =
        encode_to_vec(&value).expect("encode Bound::Excluded(77u8) for discriminant check");
    assert_eq!(
        encoded[0], 2u8,
        "Bound::Excluded discriminant byte should be 2"
    );
}

// 21. encoded_size consistency: encoded_size(Bound::Unbounded::<u32>) == encode_to_vec().len()
#[test]
fn test_encoded_size_consistency_unbounded_u32() {
    let value: Bound<u32> = Bound::Unbounded;
    let computed_size = encoded_size(&value).expect("encoded_size for Bound::Unbounded<u32>");
    let actual_bytes = encode_to_vec(&value).expect("encode_to_vec for Bound::Unbounded<u32>");
    assert_eq!(
        computed_size,
        actual_bytes.len(),
        "encoded_size should match actual encoded byte length for Bound::Unbounded<u32>"
    );
}

// 22. Struct with two Bound fields roundtrip (using derive)
#[test]
fn test_struct_with_two_bound_fields_roundtrip() {
    let value = RangeBounds {
        start: Bound::Included(100u64),
        end: Bound::Excluded(200u64),
    };
    let encoded = encode_to_vec(&value).expect("encode RangeBounds struct");
    let (decoded, _): (RangeBounds, usize) =
        decode_from_slice(&encoded).expect("decode RangeBounds struct");
    assert_eq!(value, decoded);
}
