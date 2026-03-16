//! Advanced tests (set 2) for std::ops::Bound<T> serialization in OxiCode.

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
use std::ops::Bound;

// 1. Bound::Included(42u32) roundtrip
#[test]
fn test_bound_included_u32_roundtrip() {
    let value: Bound<u32> = Bound::Included(42u32);
    let enc = encode_to_vec(&value).expect("encode Bound::Included(42u32)");
    let (val, _): (Bound<u32>, usize) =
        decode_from_slice(&enc).expect("decode Bound::Included(42u32)");
    assert_eq!(val, value);
}

// 2. Bound::Excluded(42u32) roundtrip
#[test]
fn test_bound_excluded_u32_roundtrip() {
    let value: Bound<u32> = Bound::Excluded(42u32);
    let enc = encode_to_vec(&value).expect("encode Bound::Excluded(42u32)");
    let (val, _): (Bound<u32>, usize) =
        decode_from_slice(&enc).expect("decode Bound::Excluded(42u32)");
    assert_eq!(val, value);
}

// 3. Bound::<u32>::Unbounded roundtrip
#[test]
fn test_bound_unbounded_roundtrip() {
    let value: Bound<u32> = Bound::Unbounded;
    let enc = encode_to_vec(&value).expect("encode Bound::Unbounded<u32>");
    let (val, _): (Bound<u32>, usize) =
        decode_from_slice(&enc).expect("decode Bound::Unbounded<u32>");
    assert_eq!(val, value);
}

// 4. Bound::Included(0u32) roundtrip
#[test]
fn test_bound_included_zero_roundtrip() {
    let value: Bound<u32> = Bound::Included(0u32);
    let enc = encode_to_vec(&value).expect("encode Bound::Included(0u32)");
    let (val, _): (Bound<u32>, usize) =
        decode_from_slice(&enc).expect("decode Bound::Included(0u32)");
    assert_eq!(val, value);
}

// 5. Bound::Included(u32::MAX) roundtrip
#[test]
fn test_bound_included_max_roundtrip() {
    let value: Bound<u32> = Bound::Included(u32::MAX);
    let enc = encode_to_vec(&value).expect("encode Bound::Included(u32::MAX)");
    let (val, _): (Bound<u32>, usize) =
        decode_from_slice(&enc).expect("decode Bound::Included(u32::MAX)");
    assert_eq!(val, value);
}

// 6. Bound::Excluded(0u32) roundtrip
#[test]
fn test_bound_excluded_zero_roundtrip() {
    let value: Bound<u32> = Bound::Excluded(0u32);
    let enc = encode_to_vec(&value).expect("encode Bound::Excluded(0u32)");
    let (val, _): (Bound<u32>, usize) =
        decode_from_slice(&enc).expect("decode Bound::Excluded(0u32)");
    assert_eq!(val, value);
}

// 7. Bound::Included("hello".to_string()) roundtrip
#[test]
fn test_bound_included_string_roundtrip() {
    let value: Bound<String> = Bound::Included("hello".to_string());
    let enc = encode_to_vec(&value).expect("encode Bound::Included(hello)");
    let (val, _): (Bound<String>, usize) =
        decode_from_slice(&enc).expect("decode Bound::Included(hello)");
    assert_eq!(val, value);
}

// 8. Bound::Excluded("world".to_string()) roundtrip
#[test]
fn test_bound_excluded_string_roundtrip() {
    let value: Bound<String> = Bound::Excluded("world".to_string());
    let enc = encode_to_vec(&value).expect("encode Bound::Excluded(world)");
    let (val, _): (Bound<String>, usize) =
        decode_from_slice(&enc).expect("decode Bound::Excluded(world)");
    assert_eq!(val, value);
}

// 9. Bound::<String>::Unbounded roundtrip
#[test]
fn test_bound_unbounded_string_roundtrip() {
    let value: Bound<String> = Bound::Unbounded;
    let enc = encode_to_vec(&value).expect("encode Bound::Unbounded<String>");
    let (val, _): (Bound<String>, usize) =
        decode_from_slice(&enc).expect("decode Bound::Unbounded<String>");
    assert_eq!(val, value);
}

// 10. Included, Excluded, Unbounded all produce different byte sequences
#[test]
fn test_bound_variants_different_bytes() {
    let inc: Bound<u32> = Bound::Included(42u32);
    let exc: Bound<u32> = Bound::Excluded(42u32);
    let unb: Bound<u32> = Bound::Unbounded;

    let enc_inc = encode_to_vec(&inc).expect("encode Included");
    let enc_exc = encode_to_vec(&exc).expect("encode Excluded");
    let enc_unb = encode_to_vec(&unb).expect("encode Unbounded");

    assert_ne!(enc_inc, enc_exc, "Included and Excluded must differ");
    assert_ne!(enc_inc, enc_unb, "Included and Unbounded must differ");
    assert_ne!(enc_exc, enc_unb, "Excluded and Unbounded must differ");
}

// 11. Bytes consumed equals encoded length
#[test]
fn test_bound_consumed_equals_len() {
    let value: Bound<u32> = Bound::Included(999u32);
    let enc = encode_to_vec(&value).expect("encode Bound::Included(999u32)");
    let (_, consumed): (Bound<u32>, usize) =
        decode_from_slice(&enc).expect("decode Bound::Included(999u32)");
    assert_eq!(
        consumed,
        enc.len(),
        "consumed bytes must equal encoded length"
    );
}

// 12. Vec<Bound<u32>> with all 3 variants roundtrip
#[test]
fn test_vec_bound_roundtrip() {
    let value: Vec<Bound<u32>> = vec![
        Bound::Unbounded,
        Bound::Included(10u32),
        Bound::Excluded(20u32),
    ];
    let enc = encode_to_vec(&value).expect("encode Vec<Bound<u32>>");
    let (val, _): (Vec<Bound<u32>>, usize) =
        decode_from_slice(&enc).expect("decode Vec<Bound<u32>>");
    assert_eq!(val, value);
}

// 13. Option<Bound<u32>> Some(Included(5)) roundtrip
#[test]
fn test_option_bound_some_roundtrip() {
    let value: Option<Bound<u32>> = Some(Bound::Included(5u32));
    let enc = encode_to_vec(&value).expect("encode Some(Bound::Included(5))");
    let (val, _): (Option<Bound<u32>>, usize) =
        decode_from_slice(&enc).expect("decode Some(Bound::Included(5))");
    assert_eq!(val, value);
}

// 14. Option<Bound<u32>> None roundtrip
#[test]
fn test_option_bound_none_roundtrip() {
    let value: Option<Bound<u32>> = None;
    let enc = encode_to_vec(&value).expect("encode None::<Option<Bound<u32>>>");
    let (val, _): (Option<Bound<u32>>, usize) =
        decode_from_slice(&enc).expect("decode None::<Option<Bound<u32>>>");
    assert_eq!(val, value);
}

// 15. Bound::Included(u64::MAX) roundtrip
#[test]
fn test_bound_u64_roundtrip() {
    let value: Bound<u64> = Bound::Included(u64::MAX);
    let enc = encode_to_vec(&value).expect("encode Bound::Included(u64::MAX)");
    let (val, _): (Bound<u64>, usize) =
        decode_from_slice(&enc).expect("decode Bound::Included(u64::MAX)");
    assert_eq!(val, value);
}

// 16. Bound::Included(u32::MAX) with fixed_int config roundtrip
#[test]
fn test_bound_fixed_int_config_roundtrip() {
    let cfg = config::standard().with_fixed_int_encoding();
    let value: Bound<u32> = Bound::Included(u32::MAX);
    let enc =
        encode_to_vec_with_config(&value, cfg).expect("encode Bound::Included(u32::MAX) fixed_int");
    let (val, _): (Bound<u32>, usize) = decode_from_slice_with_config(&enc, cfg)
        .expect("decode Bound::Included(u32::MAX) fixed_int");
    assert_eq!(val, value);
}

// 17. Bound with big_endian config roundtrip
#[test]
fn test_bound_big_endian_config_roundtrip() {
    let cfg = config::standard().with_big_endian();
    let value: Bound<u32> = Bound::Excluded(12345u32);
    let enc =
        encode_to_vec_with_config(&value, cfg).expect("encode Bound::Excluded(12345) big_endian");
    let (val, _): (Bound<u32>, usize) =
        decode_from_slice_with_config(&enc, cfg).expect("decode Bound::Excluded(12345) big_endian");
    assert_eq!(val, value);
}

// 18. Unbounded should produce smaller (or equal) encoding than Included (no payload)
#[test]
fn test_bound_unbounded_is_smallest() {
    let unb: Bound<u32> = Bound::Unbounded;
    let inc: Bound<u32> = Bound::Included(42u32);

    let enc_unb = encode_to_vec(&unb).expect("encode Unbounded");
    let enc_inc = encode_to_vec(&inc).expect("encode Included(42)");

    assert!(
        enc_unb.len() < enc_inc.len(),
        "Unbounded ({} bytes) should be smaller than Included ({} bytes)",
        enc_unb.len(),
        enc_inc.len()
    );
}

// 19. (Bound<u32>, Bound<u32>) as range bounds roundtrip — (Included(0), Excluded(100))
#[test]
fn test_bound_pair_roundtrip() {
    let value: (Bound<u32>, Bound<u32>) = (Bound::Included(0u32), Bound::Excluded(100u32));
    let enc = encode_to_vec(&value).expect("encode (Bound<u32>, Bound<u32>)");
    let (val, _): ((Bound<u32>, Bound<u32>), usize) =
        decode_from_slice(&enc).expect("decode (Bound<u32>, Bound<u32>)");
    assert_eq!(val, value);
}

// 20. Vec<Bound<String>> with 3 elements roundtrip
#[test]
fn test_vec_bound_string_roundtrip() {
    let value: Vec<Bound<String>> = vec![
        Bound::Included("apple".to_string()),
        Bound::Excluded("mango".to_string()),
        Bound::Unbounded,
    ];
    let enc = encode_to_vec(&value).expect("encode Vec<Bound<String>>");
    let (val, _): (Vec<Bound<String>>, usize) =
        decode_from_slice(&enc).expect("decode Vec<Bound<String>>");
    assert_eq!(val, value);
}

// 21. Bound::Included(-100i64) roundtrip
#[test]
fn test_bound_i64_negative_roundtrip() {
    let value: Bound<i64> = Bound::Included(-100i64);
    let enc = encode_to_vec(&value).expect("encode Bound::Included(-100i64)");
    let (val, _): (Bound<i64>, usize) =
        decode_from_slice(&enc).expect("decode Bound::Included(-100i64)");
    assert_eq!(val, value);
}

// 22. Re-encode decoded Bound produces same bytes (consistency)
#[test]
fn test_bound_reencode_consistency() {
    let original: Bound<u32> = Bound::Included(777u32);
    let enc1 = encode_to_vec(&original).expect("encode original Bound");
    let (decoded, _): (Bound<u32>, usize) = decode_from_slice(&enc1).expect("decode Bound");
    let enc2 = encode_to_vec(&decoded).expect("re-encode decoded Bound");
    assert_eq!(
        enc1, enc2,
        "re-encoding a decoded Bound must produce identical bytes"
    );
}
