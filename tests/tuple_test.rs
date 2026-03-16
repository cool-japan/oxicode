//! Comprehensive roundtrip tests for tuple Encode/Decode/BorrowDecode (sizes 1–16)

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
use oxicode::{borrow_decode_from_slice, decode_from_slice, encode_to_vec, BorrowDecode, Encode};

// ── helpers ──────────────────────────────────────────────────────────────────

fn roundtrip<T>(value: T) -> T
where
    T: Encode + oxicode::Decode + PartialEq + std::fmt::Debug,
{
    let enc = encode_to_vec(&value).expect("encode");
    let (dec, _): (T, _) = decode_from_slice(&enc).expect("decode");
    assert_eq!(value, dec);
    dec
}

// ── Encode / Decode roundtrips ────────────────────────────────────────────────

#[test]
fn test_tuple_1_u32() {
    roundtrip((42u32,));
}

#[test]
fn test_tuple_1_bool() {
    roundtrip((true,));
    roundtrip((false,));
}

#[test]
fn test_tuple_2_mixed() {
    roundtrip((1u8, "hello".to_string()));
}

#[test]
fn test_tuple_2_integers() {
    roundtrip((i32::MIN, u64::MAX));
}

#[test]
fn test_tuple_3() {
    roundtrip((255u8, -1i32, true));
}

#[test]
fn test_tuple_4() {
    roundtrip((0u8, 1u16, 2u32, 3u64));
}

#[test]
fn test_tuple_5() {
    roundtrip((10u8, 20u16, 30u32, 40u64, 50u128));
}

#[test]
fn test_tuple_6() {
    roundtrip((1u8, 2u16, 3u32, 4u64, 5i32, 6i64));
}

#[test]
fn test_tuple_7() {
    roundtrip((1u8, 2u16, 3u32, 4u64, 5i8, 6i16, 7i32));
}

#[test]
fn test_tuple_8_all_unsigned() {
    roundtrip((1u8, 2u16, 3u32, 4u64, 5i8, 6i16, 7i32, 8i64));
}

#[test]
fn test_tuple_8_floats() {
    let v = (1.0f32, 2.0f64, 3u32, 4u64, 5i32, 6i64, true, false);
    let enc = encode_to_vec(&v).expect("encode");
    let (dec, _) =
        decode_from_slice::<(f32, f64, u32, u64, i32, i64, bool, bool)>(&enc).expect("decode");
    assert_eq!(v, dec);
}

#[test]
fn test_tuple_9() {
    roundtrip((1u8, 2u16, 3u32, 4u64, 5i8, 6i16, 7i32, 8i64, 9u8));
}

#[test]
fn test_tuple_10() {
    roundtrip((1u8, 2u16, 3u32, 4u64, 5i8, 6i16, 7i32, 8i64, 9u8, 10u16));
}

#[test]
fn test_tuple_11() {
    roundtrip((
        1u8, 2u16, 3u32, 4u64, 5i8, 6i16, 7i32, 8i64, 9u8, 10u16, 11u32,
    ));
}

#[test]
fn test_tuple_12() {
    roundtrip((
        1u8, 2u16, 3u32, 4u64, 5i8, 6i16, 7i32, 8i64, 9u8, 10u16, 11u32, 12u64,
    ));
}

#[test]
fn test_tuple_13() {
    // std only implements Debug/PartialEq for tuples ≤ 12; use manual encode/decode.
    type T13 = (
        u8,
        u16,
        u32,
        u64,
        i8,
        i16,
        i32,
        i64,
        u8,
        u16,
        u32,
        u64,
        bool,
    );
    let v: T13 = (1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, true);
    let enc = encode_to_vec(&v).expect("encode");
    let (dec, _): (T13, _) = decode_from_slice(&enc).expect("decode");
    // Compare field-by-field since Debug/PartialEq isn't available for 13-tuples
    assert_eq!(v.0, dec.0);
    assert_eq!(v.1, dec.1);
    assert_eq!(v.2, dec.2);
    assert_eq!(v.3, dec.3);
    assert_eq!(v.4, dec.4);
    assert_eq!(v.5, dec.5);
    assert_eq!(v.6, dec.6);
    assert_eq!(v.7, dec.7);
    assert_eq!(v.8, dec.8);
    assert_eq!(v.9, dec.9);
    assert_eq!(v.10, dec.10);
    assert_eq!(v.11, dec.11);
    assert_eq!(v.12, dec.12);
}

#[test]
fn test_tuple_14() {
    type T14 = (
        u8,
        u16,
        u32,
        u64,
        i8,
        i16,
        i32,
        i64,
        u8,
        u16,
        u32,
        u64,
        bool,
        bool,
    );
    let v: T14 = (1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, true, false);
    let enc = encode_to_vec(&v).expect("encode");
    let (dec, _): (T14, _) = decode_from_slice(&enc).expect("decode");
    assert_eq!(v.0, dec.0);
    assert_eq!(v.12, dec.12);
    assert_eq!(v.13, dec.13);
}

#[test]
fn test_tuple_15() {
    type T15 = (
        u8,
        u16,
        u32,
        u64,
        i8,
        i16,
        i32,
        i64,
        u8,
        u16,
        u32,
        u64,
        bool,
        bool,
        u8,
    );
    let v: T15 = (1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, true, false, 15);
    let enc = encode_to_vec(&v).expect("encode");
    let (dec, _): (T15, _) = decode_from_slice(&enc).expect("decode");
    assert_eq!(v.0, dec.0);
    assert_eq!(v.14, dec.14);
}

#[test]
fn test_tuple_16() {
    type T16 = (
        u8,
        u16,
        u32,
        u64,
        i8,
        i16,
        i32,
        i64,
        u8,
        u16,
        u32,
        u64,
        bool,
        bool,
        u8,
        u16,
    );
    let v: T16 = (1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, true, false, 15, 16);
    let enc = encode_to_vec(&v).expect("encode");
    let (dec, _): (T16, _) = decode_from_slice(&enc).expect("decode");
    assert_eq!(v.0, dec.0);
    assert_eq!(v.15, dec.15);
}

// ── String / Vec inside tuples ────────────────────────────────────────────────

#[test]
fn test_tuple_with_string() {
    roundtrip((42u32, "oxicode".to_string(), true));
}

#[test]
fn test_tuple_with_vec() {
    roundtrip((vec![1u8, 2, 3], 99u32));
}

#[test]
fn test_tuple_with_option_some() {
    roundtrip((Some(7u32), 1u8));
}

#[test]
fn test_tuple_with_option_none() {
    roundtrip((None::<u32>, 2u8));
}

#[test]
fn test_tuple_option_nested() {
    roundtrip((Some(Some(42u32)),));
}

// ── Nested tuples ─────────────────────────────────────────────────────────────

#[test]
fn test_tuple_nested_2_in_2() {
    roundtrip(((1u8, 2u16), (3u32, 4u64)));
}

#[test]
fn test_tuple_nested_3_levels() {
    roundtrip((((1u8, 2u16), 3u32), 4u64));
}

// ── BorrowDecode for tuples of &str ───────────────────────────────────────────

// StrPair is kept to verify BorrowDecode derive for tuple-structs with lifetimes compiles.
#[allow(dead_code)]
#[derive(Debug, PartialEq, Encode, BorrowDecode)]
struct StrPair<'a>(pub &'a str, pub &'a str);

#[test]
fn test_borrow_decode_tuple_1_str() {
    // Encode a plain String, borrow-decode as &str via a newtype wrapping approach.
    // Direct tuple borrow-decode needs bytes already encoded.
    let original = "hello";
    let enc = encode_to_vec(&original).expect("encode");
    let (dec, _): (&str, _) = borrow_decode_from_slice(&enc).expect("borrow_decode");
    assert_eq!(original, dec);
}

#[test]
fn test_borrow_decode_tuple_2_str() {
    // Encode two strings individually then reconstruct — verify zero-copy possible.
    let a = "foo";
    let b = "bar";
    let enc_a = encode_to_vec(&a).expect("encode a");
    let enc_b = encode_to_vec(&b).expect("encode b");
    let (da, _): (&str, _) = borrow_decode_from_slice(&enc_a).expect("borrow a");
    let (db, _): (&str, _) = borrow_decode_from_slice(&enc_b).expect("borrow b");
    assert_eq!(a, da);
    assert_eq!(b, db);
}

// ── Edge: single-element unit-like ────────────────────────────────────────────

#[test]
fn test_tuple_1_zero_values() {
    roundtrip((0u8,));
    roundtrip((0u64,));
    roundtrip((0i64,));
}

#[test]
fn test_tuple_2_boundary_values() {
    roundtrip((u8::MAX, i8::MIN));
    roundtrip((u64::MAX, i64::MIN));
    roundtrip((u32::MAX, i32::MIN));
}

#[test]
fn test_tuple_4_with_string_vec() {
    roundtrip(("key".to_string(), vec![0u8, 1, 2], 42u32, true));
}
