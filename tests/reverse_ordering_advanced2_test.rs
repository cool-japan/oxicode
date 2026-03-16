//! Tests for std::cmp::Reverse<T>, std::cmp::Ordering, and std::num::Wrapping<T> encoding.

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
use std::cmp::{Ordering, Reverse};
use std::num::Wrapping;

// Struct used in test 18
#[derive(Debug, PartialEq, Encode, Decode)]
struct ReverseWithOrdering {
    rank: Reverse<u32>,
    cmp: Ordering,
}

// ===== 1. Reverse<u32> roundtrip =====

#[test]
fn test_reverse_u32_roundtrip() {
    let original = Reverse(42u32);
    let enc = encode_to_vec(&original).expect("encode Reverse<u32> failed");
    let (val, _): (Reverse<u32>, usize) =
        decode_from_slice(&enc).expect("decode Reverse<u32> failed");
    assert_eq!(original, val);
    assert_eq!(val.0, 42u32);
}

// ===== 2. Reverse<String> roundtrip =====

#[test]
fn test_reverse_string_roundtrip() {
    let original = Reverse("hello world".to_string());
    let enc = encode_to_vec(&original).expect("encode Reverse<String> failed");
    let (val, _): (Reverse<String>, usize) =
        decode_from_slice(&enc).expect("decode Reverse<String> failed");
    assert_eq!(original, val);
    assert_eq!(val.0, "hello world");
}

// ===== 3. Reverse<u32> produces same bytes as inner u32 (transparent wrapper) =====

#[test]
fn test_reverse_u32_transparent_wrapper() {
    let raw: u32 = 99u32;
    let wrapped = Reverse(99u32);
    let raw_bytes = encode_to_vec(&raw).expect("encode raw u32 failed");
    let wrapped_bytes = encode_to_vec(&wrapped).expect("encode Reverse<u32> failed");
    assert_eq!(
        raw_bytes, wrapped_bytes,
        "Reverse<u32> must encode identically to inner u32"
    );
}

// ===== 4. Vec<Reverse<u32>> roundtrip =====

#[test]
fn test_vec_reverse_u32_roundtrip() {
    let original: Vec<Reverse<u32>> = vec![Reverse(1u32), Reverse(100u32), Reverse(u32::MAX)];
    let enc = encode_to_vec(&original).expect("encode Vec<Reverse<u32>> failed");
    let (val, _): (Vec<Reverse<u32>>, usize) =
        decode_from_slice(&enc).expect("decode Vec<Reverse<u32>> failed");
    assert_eq!(original, val);
}

// ===== 5. Ordering::Less roundtrip =====

#[test]
fn test_ordering_less_roundtrip() {
    let original = Ordering::Less;
    let enc = encode_to_vec(&original).expect("encode Ordering::Less failed");
    let (val, _): (Ordering, usize) =
        decode_from_slice(&enc).expect("decode Ordering::Less failed");
    assert_eq!(original, val);
}

// ===== 6. Ordering::Equal roundtrip =====

#[test]
fn test_ordering_equal_roundtrip() {
    let original = Ordering::Equal;
    let enc = encode_to_vec(&original).expect("encode Ordering::Equal failed");
    let (val, _): (Ordering, usize) =
        decode_from_slice(&enc).expect("decode Ordering::Equal failed");
    assert_eq!(original, val);
}

// ===== 7. Ordering::Greater roundtrip =====

#[test]
fn test_ordering_greater_roundtrip() {
    let original = Ordering::Greater;
    let enc = encode_to_vec(&original).expect("encode Ordering::Greater failed");
    let (val, _): (Ordering, usize) =
        decode_from_slice(&enc).expect("decode Ordering::Greater failed");
    assert_eq!(original, val);
}

// ===== 8. Ordering::Less, Equal, Greater have distinct byte encodings =====

#[test]
fn test_ordering_variants_distinct_byte_encodings() {
    let less = encode_to_vec(&Ordering::Less).expect("encode Ordering::Less failed");
    let equal = encode_to_vec(&Ordering::Equal).expect("encode Ordering::Equal failed");
    let greater = encode_to_vec(&Ordering::Greater).expect("encode Ordering::Greater failed");
    assert_ne!(less, equal, "Less and Equal must produce different bytes");
    assert_ne!(
        equal, greater,
        "Equal and Greater must produce different bytes"
    );
    assert_ne!(
        less, greater,
        "Less and Greater must produce different bytes"
    );
}

// ===== 9. Vec<Ordering> roundtrip =====

#[test]
fn test_vec_ordering_roundtrip() {
    let original: Vec<Ordering> = vec![Ordering::Less, Ordering::Equal, Ordering::Greater];
    let enc = encode_to_vec(&original).expect("encode Vec<Ordering> failed");
    let (val, _): (Vec<Ordering>, usize) =
        decode_from_slice(&enc).expect("decode Vec<Ordering> failed");
    assert_eq!(original, val);
    assert_eq!(val[0], Ordering::Less);
    assert_eq!(val[1], Ordering::Equal);
    assert_eq!(val[2], Ordering::Greater);
}

// ===== 10. Option<Ordering> Some roundtrip =====

#[test]
fn test_option_ordering_some_roundtrip() {
    let original: Option<Ordering> = Some(Ordering::Greater);
    let enc = encode_to_vec(&original).expect("encode Option<Ordering>(Some) failed");
    let (val, _): (Option<Ordering>, usize) =
        decode_from_slice(&enc).expect("decode Option<Ordering>(Some) failed");
    assert_eq!(original, val);
    assert_eq!(val, Some(Ordering::Greater));
}

// ===== 11. Wrapping<u32> roundtrip =====

#[test]
fn test_wrapping_u32_roundtrip() {
    let original = Wrapping(u32::MAX);
    let enc = encode_to_vec(&original).expect("encode Wrapping<u32> failed");
    let (val, _): (Wrapping<u32>, usize) =
        decode_from_slice(&enc).expect("decode Wrapping<u32> failed");
    assert_eq!(original, val);
    assert_eq!(val.0, u32::MAX);
}

// ===== 12. Wrapping<i32> roundtrip with negative values =====

#[test]
fn test_wrapping_i32_negative_roundtrip() {
    let original = Wrapping(-12345i32);
    let enc = encode_to_vec(&original).expect("encode Wrapping<i32> with negative value failed");
    let (val, _): (Wrapping<i32>, usize) =
        decode_from_slice(&enc).expect("decode Wrapping<i32> with negative value failed");
    assert_eq!(original, val);
    assert_eq!(val.0, -12345i32);
}

// ===== 13. Wrapping<u8> roundtrip =====

#[test]
fn test_wrapping_u8_roundtrip() {
    let original = Wrapping(255u8);
    let enc = encode_to_vec(&original).expect("encode Wrapping<u8> failed");
    let (val, _): (Wrapping<u8>, usize) =
        decode_from_slice(&enc).expect("decode Wrapping<u8> failed");
    assert_eq!(original, val);
    assert_eq!(val.0, 255u8);
}

// ===== 14. Wrapping<u32> produces same bytes as inner u32 (transparent wrapper) =====

#[test]
fn test_wrapping_u32_transparent_wrapper() {
    let raw: u32 = 7654321u32;
    let wrapped = Wrapping(7654321u32);
    let raw_bytes = encode_to_vec(&raw).expect("encode raw u32 failed");
    let wrapped_bytes = encode_to_vec(&wrapped).expect("encode Wrapping<u32> failed");
    assert_eq!(
        raw_bytes, wrapped_bytes,
        "Wrapping<u32> must encode identically to inner u32"
    );
}

// ===== 15. Vec<Wrapping<u32>> roundtrip =====

#[test]
fn test_vec_wrapping_u32_roundtrip() {
    let original: Vec<Wrapping<u32>> = vec![Wrapping(0u32), Wrapping(1u32), Wrapping(u32::MAX)];
    let enc = encode_to_vec(&original).expect("encode Vec<Wrapping<u32>> failed");
    let (val, _): (Vec<Wrapping<u32>>, usize) =
        decode_from_slice(&enc).expect("decode Vec<Wrapping<u32>> failed");
    assert_eq!(original, val);
}

// ===== 16. Option<Wrapping<u32>> Some roundtrip =====

#[test]
fn test_option_wrapping_u32_some_roundtrip() {
    let original: Option<Wrapping<u32>> = Some(Wrapping(42u32));
    let enc = encode_to_vec(&original).expect("encode Option<Wrapping<u32>>(Some) failed");
    let (val, _): (Option<Wrapping<u32>>, usize) =
        decode_from_slice(&enc).expect("decode Option<Wrapping<u32>>(Some) failed");
    assert_eq!(original, val);
    assert!(val.is_some());
}

// ===== 17. Option<Wrapping<u32>> None roundtrip =====

#[test]
fn test_option_wrapping_u32_none_roundtrip() {
    let original: Option<Wrapping<u32>> = None;
    let enc = encode_to_vec(&original).expect("encode Option<Wrapping<u32>>(None) failed");
    let (val, _): (Option<Wrapping<u32>>, usize) =
        decode_from_slice(&enc).expect("decode Option<Wrapping<u32>>(None) failed");
    assert_eq!(original, val);
    assert!(val.is_none());
}

// ===== 18. Struct containing Reverse<u32> and Ordering roundtrip =====

#[test]
fn test_struct_reverse_u32_and_ordering_roundtrip() {
    let original = ReverseWithOrdering {
        rank: Reverse(5u32),
        cmp: Ordering::Less,
    };
    let enc = encode_to_vec(&original).expect("encode ReverseWithOrdering failed");
    let (val, _): (ReverseWithOrdering, usize) =
        decode_from_slice(&enc).expect("decode ReverseWithOrdering failed");
    assert_eq!(original, val);
    assert_eq!(val.rank.0, 5u32);
    assert_eq!(val.cmp, Ordering::Less);
}

// ===== 19. consumed bytes == encoded length for Ordering =====

#[test]
fn test_ordering_consumed_bytes_equals_encoded_length() {
    for &ordering in &[Ordering::Less, Ordering::Equal, Ordering::Greater] {
        let enc = encode_to_vec(&ordering).expect("encode Ordering failed");
        let (_, consumed): (Ordering, usize) =
            decode_from_slice(&enc).expect("decode Ordering failed");
        assert_eq!(
            consumed,
            enc.len(),
            "consumed bytes must equal encoded length for {:?}",
            ordering
        );
    }
}

// ===== 20. consumed bytes == encoded length for Wrapping<u64> =====

#[test]
fn test_wrapping_u64_consumed_bytes_equals_encoded_length() {
    let original = Wrapping(u64::MAX / 2);
    let enc = encode_to_vec(&original).expect("encode Wrapping<u64> failed");
    let (_, consumed): (Wrapping<u64>, usize) =
        decode_from_slice(&enc).expect("decode Wrapping<u64> failed");
    assert_eq!(
        consumed,
        enc.len(),
        "consumed bytes must equal encoded length for Wrapping<u64>"
    );
}

// ===== 21. Reverse<u32> with fixed_int_encoding config roundtrip =====

#[test]
fn test_reverse_u32_fixed_int_encoding_config_roundtrip() {
    let original = Reverse(123456u32);
    let cfg = config::standard().with_fixed_int_encoding();
    let enc = encode_to_vec_with_config(&original, cfg)
        .expect("encode Reverse<u32> with fixed_int_encoding failed");
    let (val, _): (Reverse<u32>, usize) = decode_from_slice_with_config(&enc, cfg)
        .expect("decode Reverse<u32> with fixed_int_encoding failed");
    assert_eq!(original, val);
    assert_eq!(val.0, 123456u32);
    // With fixed_int_encoding, u32 must occupy exactly 4 bytes
    assert_eq!(
        enc.len(),
        4,
        "fixed_int_encoding must produce exactly 4 bytes for u32"
    );
}

// ===== 22. Two different Wrapping<u32> values produce different encodings =====

#[test]
fn test_two_different_wrapping_u32_produce_different_encodings() {
    let a = Wrapping(0u32);
    let b = Wrapping(1u32);
    let enc_a = encode_to_vec(&a).expect("encode Wrapping(0u32) failed");
    let enc_b = encode_to_vec(&b).expect("encode Wrapping(1u32) failed");
    assert_ne!(
        enc_a, enc_b,
        "Wrapping(0u32) and Wrapping(1u32) must produce different encodings"
    );
}
