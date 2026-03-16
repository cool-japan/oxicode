//! Extended tuple tests: mixed types, nested tuples, structs, collections, and BorrowDecode.

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
use oxicode::{borrow_decode_from_slice, decode_from_slice, encode_to_vec, Decode, Encode};
use std::collections::HashMap;

// ── helper ────────────────────────────────────────────────────────────────────

fn roundtrip<T>(value: T) -> T
where
    T: Encode + Decode + PartialEq + std::fmt::Debug,
{
    let enc = encode_to_vec(&value).expect("encode");
    let (dec, _): (T, _) = decode_from_slice(&enc).expect("decode");
    assert_eq!(value, dec);
    dec
}

// ── 1. tuple1 through tuple8 with genuinely mixed types ───────────────────────

#[test]
fn test_tuple_extended_mixed_tuple1() {
    roundtrip((42u8,));
}

#[test]
fn test_tuple_extended_mixed_tuple2() {
    roundtrip((7u8, "hello oxicode".to_string()));
}

#[test]
fn test_tuple_extended_mixed_tuple3() {
    roundtrip((255u8, "world".to_string(), std::f64::consts::PI));
}

#[test]
fn test_tuple_extended_mixed_tuple4() {
    roundtrip((0u8, "rust".to_string(), std::f64::consts::E, true));
}

#[test]
fn test_tuple_extended_mixed_tuple5() {
    roundtrip((10u8, "five".to_string(), 1.0f64, false, -42i32));
}

#[test]
fn test_tuple_extended_mixed_tuple6() {
    roundtrip((1u8, "six".to_string(), 0.0f64, true, 100i32, 999u64));
}

#[test]
fn test_tuple_extended_mixed_tuple7() {
    roundtrip((
        2u8,
        "seven".to_string(),
        1.1f64,
        false,
        -1i32,
        u64::MAX,
        vec![0u8, 1u8, 2u8],
    ));
}

#[test]
fn test_tuple_extended_mixed_tuple8() {
    roundtrip((
        3u8,
        "eight".to_string(),
        9.9f64,
        true,
        i32::MIN,
        u64::MAX,
        vec![10u8, 20u8],
        Some("opt".to_string()),
    ));
}

// Tuple9 through Tuple12 — mixed types (std only has Debug/PartialEq up to 12)

#[test]
fn test_tuple_extended_mixed_tuple9() {
    roundtrip((
        1u8,
        2u16,
        3u32,
        4u64,
        -5i8,
        -6i16,
        -7i32,
        -8i64,
        "nine".to_string(),
    ));
}

#[test]
fn test_tuple_extended_mixed_tuple10() {
    roundtrip((
        0u8,
        1u16,
        2u32,
        3u64,
        -4i8,
        -5i16,
        -6i32,
        -7i64,
        "ten".to_string(),
        vec![42u8],
    ));
}

#[test]
fn test_tuple_extended_mixed_tuple11() {
    roundtrip((
        0u8,
        1u16,
        2u32,
        3u64,
        -4i8,
        -5i16,
        -6i32,
        -7i64,
        "eleven".to_string(),
        vec![1u8, 2u8],
        true,
    ));
}

#[test]
fn test_tuple_extended_mixed_tuple12() {
    roundtrip((
        0u8,
        1u16,
        2u32,
        3u64,
        -4i8,
        -5i16,
        -6i32,
        -7i64,
        "twelve".to_string(),
        vec![0u8],
        true,
        Some(99u32),
    ));
}

// Tuple13 through Tuple16 — field-by-field comparison (no PartialEq/Debug for >12)

#[test]
fn test_tuple_extended_mixed_tuple13() {
    type T13 = (
        u8,
        u16,
        u32,
        u64,
        i8,
        i16,
        i32,
        i64,
        bool,
        bool,
        u32,
        u64,
        String,
    );
    let v: T13 = (
        1,
        2,
        3,
        4,
        -5,
        -6,
        -7,
        -8,
        true,
        false,
        42,
        99,
        "thirteen".to_string(),
    );
    let enc = encode_to_vec(&v).expect("encode");
    let (dec, _): (T13, _) = decode_from_slice(&enc).expect("decode");
    assert_eq!(v.0, dec.0);
    assert_eq!(v.6, dec.6);
    assert_eq!(v.12, dec.12);
}

#[test]
fn test_tuple_extended_mixed_tuple14() {
    type T14 = (
        u8,
        u16,
        u32,
        u64,
        i8,
        i16,
        i32,
        i64,
        bool,
        bool,
        u32,
        u64,
        String,
        Vec<u8>,
    );
    let v: T14 = (
        1,
        2,
        3,
        4,
        -5,
        -6,
        -7,
        -8,
        true,
        false,
        42,
        99,
        "fourteen".to_string(),
        vec![1u8, 2u8],
    );
    let enc = encode_to_vec(&v).expect("encode");
    let (dec, _): (T14, _) = decode_from_slice(&enc).expect("decode");
    assert_eq!(v.0, dec.0);
    assert_eq!(v.12, dec.12);
    assert_eq!(v.13, dec.13);
}

#[test]
fn test_tuple_extended_mixed_tuple15() {
    type T15 = (
        u8,
        u16,
        u32,
        u64,
        i8,
        i16,
        i32,
        i64,
        bool,
        bool,
        u32,
        u64,
        String,
        Vec<u8>,
        f32,
    );
    let v: T15 = (
        1,
        2,
        3,
        4,
        -5,
        -6,
        -7,
        -8,
        true,
        false,
        42,
        99,
        "fifteen".to_string(),
        vec![3u8, 4u8],
        1.5f32,
    );
    let enc = encode_to_vec(&v).expect("encode");
    let (dec, _): (T15, _) = decode_from_slice(&enc).expect("decode");
    assert_eq!(v.0, dec.0);
    assert_eq!(v.12, dec.12);
    assert_eq!(v.14, dec.14);
}

#[test]
fn test_tuple_extended_mixed_tuple16() {
    type T16 = (
        u8,
        u16,
        u32,
        u64,
        i8,
        i16,
        i32,
        i64,
        bool,
        bool,
        u32,
        u64,
        String,
        Vec<u8>,
        f32,
        f64,
    );
    let v: T16 = (
        1,
        2,
        3,
        4,
        -5,
        -6,
        -7,
        -8,
        true,
        false,
        42,
        99,
        "sixteen".to_string(),
        vec![5u8, 6u8],
        2.5f32,
        std::f64::consts::PI,
    );
    let enc = encode_to_vec(&v).expect("encode");
    let (dec, _): (T16, _) = decode_from_slice(&enc).expect("decode");
    assert_eq!(v.0, dec.0);
    assert_eq!(v.12, dec.12);
    assert_eq!(v.15, dec.15);
}

// ── 2. Nested tuples: ((u32, u32), (String, bool)) ────────────────────────────

#[test]
fn test_tuple_extended_nested_pair_of_pairs() {
    let v: ((u32, u32), (String, bool)) = ((10, 20), ("nested".to_string(), true));
    roundtrip(v);
}

#[test]
fn test_tuple_extended_nested_triple_pair() {
    // Three-level nesting: (((u8, u16), u32), u64)
    let v: (((u8, u16), u32), u64) = (((1, 2), 3), 4);
    roundtrip(v);
}

#[test]
fn test_tuple_extended_nested_option_in_tuple() {
    let v: (Option<(u32, String)>, bool) = (Some((42, "inner".to_string())), true);
    roundtrip(v);
}

// ── 3. Tuple in a struct ──────────────────────────────────────────────────────

#[derive(Debug, PartialEq, Encode, Decode)]
struct Row {
    data: (u32, String, f64),
}

#[test]
fn test_tuple_extended_struct_with_tuple_field() {
    let row = Row {
        data: (99, "oxicode".to_string(), std::f64::consts::E),
    };
    let enc = encode_to_vec(&row).expect("encode");
    let (dec, _): (Row, _) = decode_from_slice(&enc).expect("decode");
    assert_eq!(row, dec);
}

#[test]
fn test_tuple_extended_struct_tuple_field_boundaries() {
    let row = Row {
        data: (u32::MAX, String::new(), f64::NEG_INFINITY),
    };
    let enc = encode_to_vec(&row).expect("encode");
    let (dec, _): (Row, _) = decode_from_slice(&enc).expect("decode");
    assert_eq!(row, dec);
}

// ── 4. Vec of tuples: Vec<(u32, String)> with 10 elements ────────────────────

#[test]
fn test_tuple_extended_vec_of_tuples_10_elements() {
    let items: Vec<(u32, String)> = (0u32..10).map(|i| (i, format!("item_{}", i))).collect();
    roundtrip(items);
}

#[test]
fn test_tuple_extended_vec_of_tuples_empty() {
    let items: Vec<(u32, String)> = Vec::new();
    roundtrip(items);
}

#[test]
fn test_tuple_extended_vec_of_3_tuples() {
    let items: Vec<(u8, String, bool)> = vec![
        (1, "a".to_string(), true),
        (2, "b".to_string(), false),
        (3, "c".to_string(), true),
    ];
    roundtrip(items);
}

// ── 5. Option<(u32, String)> roundtrip ───────────────────────────────────────

#[test]
fn test_tuple_extended_option_some_tuple() {
    let v: Option<(u32, String)> = Some((42, "some".to_string()));
    roundtrip(v);
}

#[test]
fn test_tuple_extended_option_none_tuple() {
    let v: Option<(u32, String)> = None;
    roundtrip(v);
}

#[test]
fn test_tuple_extended_option_nested_tuple() {
    let v: Option<(u32, Option<String>)> = Some((7, Some("inner".to_string())));
    roundtrip(v);
}

// ── 6. HashMap<String, (u32, u64)> roundtrip ─────────────────────────────────

#[test]
fn test_tuple_extended_hashmap_string_to_pair() {
    let mut map: HashMap<String, (u32, u64)> = HashMap::new();
    map.insert("alpha".to_string(), (1, 100));
    map.insert("beta".to_string(), (2, 200));
    map.insert("gamma".to_string(), (3, u64::MAX));

    let enc = encode_to_vec(&map).expect("encode");
    let (dec, _): (HashMap<String, (u32, u64)>, _) = decode_from_slice(&enc).expect("decode");

    assert_eq!(map.len(), dec.len());
    for (key, val) in &map {
        assert_eq!(dec.get(key), Some(val));
    }
}

#[test]
fn test_tuple_extended_hashmap_empty_tuple_values() {
    let map: HashMap<String, (u32, u64)> = HashMap::new();
    let enc = encode_to_vec(&map).expect("encode");
    let (dec, _): (HashMap<String, (u32, u64)>, _) = decode_from_slice(&enc).expect("decode");
    assert!(dec.is_empty());
}

// ── 7. Tuple struct vs tuple encoding compatibility ───────────────────────────
//
// A named tuple struct `Pair(u32, u64)` and a plain tuple `(u32, u64)` share
// the same wire format — verify round-trip byte equality.

#[derive(Debug, PartialEq, Encode, Decode)]
struct Pair(u32, u64);

#[test]
fn test_tuple_extended_tuple_struct_wire_compat_with_plain_tuple() {
    let pair = Pair(42, 99);
    let plain: (u32, u64) = (42, 99);

    let enc_struct = encode_to_vec(&pair).expect("encode struct");
    let enc_plain = encode_to_vec(&plain).expect("encode plain");

    // Both should produce identical bytes
    assert_eq!(
        enc_struct, enc_plain,
        "tuple struct and plain tuple must share wire format"
    );

    // Decode struct bytes as plain tuple
    let (decoded_plain, _): ((u32, u64), _) =
        decode_from_slice(&enc_struct).expect("decode plain from struct bytes");
    assert_eq!(decoded_plain, (42u32, 99u64));

    // Decode plain bytes as struct
    let (decoded_struct, _): (Pair, _) =
        decode_from_slice(&enc_plain).expect("decode struct from plain bytes");
    assert_eq!(decoded_struct, Pair(42, 99));
}

#[test]
fn test_tuple_extended_tuple_struct_roundtrip() {
    let p = Pair(u32::MAX, u64::MAX);
    let enc = encode_to_vec(&p).expect("encode");
    let (dec, _): (Pair, _) = decode_from_slice(&enc).expect("decode");
    assert_eq!(p, dec);
}

// ── 8. Large tuple: all primitive types ──────────────────────────────────────
// (u8, u16, u32, u64, u128, i8, i16, i32, i64, i128, f32, f64, bool, char, String, Vec<u8>)

#[test]
fn test_tuple_extended_large_all_primitives() {
    // 16 elements — std doesn't implement PartialEq/Debug for 16-tuples
    // so compare field-by-field.
    type T16 = (
        u8,
        u16,
        u32,
        u64,
        u128,
        i8,
        i16,
        i32,
        i64,
        i128,
        f32,
        f64,
        bool,
        char,
        String,
        Vec<u8>,
    );

    let v: T16 = (
        u8::MAX,
        u16::MAX,
        u32::MAX,
        u64::MAX,
        u128::MAX,
        i8::MIN,
        i16::MIN,
        i32::MIN,
        i64::MIN,
        i128::MIN,
        1.0f32,
        -2.5f64,
        true,
        'Z',
        "primitives".to_string(),
        vec![0u8, 128u8, 255u8],
    );

    let enc = encode_to_vec(&v).expect("encode");
    let (dec, _): (T16, _) = decode_from_slice(&enc).expect("decode");

    assert_eq!(v.0, dec.0); // u8
    assert_eq!(v.1, dec.1); // u16
    assert_eq!(v.2, dec.2); // u32
    assert_eq!(v.3, dec.3); // u64
    assert_eq!(v.4, dec.4); // u128
    assert_eq!(v.5, dec.5); // i8
    assert_eq!(v.6, dec.6); // i16
    assert_eq!(v.7, dec.7); // i32
    assert_eq!(v.8, dec.8); // i64
    assert_eq!(v.9, dec.9); // i128
    assert_eq!(v.10, dec.10); // f32
    assert_eq!(v.11, dec.11); // f64
    assert_eq!(v.12, dec.12); // bool
    assert_eq!(v.13, dec.13); // char
    assert_eq!(v.14, dec.14); // String
    assert_eq!(v.15, dec.15); // Vec<u8>
}

// ── 9. Unit tuple () roundtrip ────────────────────────────────────────────────

#[test]
fn test_tuple_extended_unit_tuple_roundtrip() {
    let v: () = ();
    let enc = encode_to_vec(&v).expect("encode");
    let (_dec, consumed): ((), _) = decode_from_slice(&enc).expect("decode");
    // Unit type encodes to zero bytes
    assert_eq!(consumed, 0);
    assert!(enc.is_empty());
}

#[test]
fn test_tuple_extended_unit_in_option() {
    // Option<()> should work just like any other Option
    let some_unit: Option<()> = Some(());
    let none_unit: Option<()> = None;
    roundtrip(some_unit);
    roundtrip(none_unit);
}

// ── 10. BorrowDecode for (&'de str, u32) ─────────────────────────────────────

#[test]
fn test_tuple_extended_borrow_decode_str_u32_pair() {
    // Encode as owned (String, u32) then borrow-decode as (&str, u32).
    // The borrow_decode impl for (T0, T1) requires both T0 and T1 to be BorrowDecode.
    // u32 implements BorrowDecode (delegates to Decode), and &str implements BorrowDecode.
    let original_str = "borrow_me";
    let original_num: u32 = 12345;

    // Encode the two values as a 2-tuple
    let enc = encode_to_vec(&(original_str.to_string(), original_num)).expect("encode");

    // Borrow-decode as (&str, u32) — zero-copy on the string
    let (dec_str, dec_num): (&str, u32) = borrow_decode_from_slice::<(&str, u32)>(&enc)
        .map(|(v, _)| v)
        .expect("borrow_decode");

    assert_eq!(dec_str, original_str);
    assert_eq!(dec_num, original_num);
}

#[test]
fn test_tuple_extended_borrow_decode_str_only() {
    let original = "zero_copy_string";
    let enc = encode_to_vec(&original).expect("encode");
    let (dec, _): (&str, _) = borrow_decode_from_slice(&enc).expect("borrow_decode");
    assert_eq!(dec, original);
}

// ── 11. Additional edge cases ─────────────────────────────────────────────────

#[test]
fn test_tuple_extended_tuple_with_empty_string() {
    roundtrip((0u32, String::new(), false));
}

#[test]
fn test_tuple_extended_tuple_with_empty_vec() {
    roundtrip((u64::MAX, Vec::<u8>::new()));
}

#[test]
fn test_tuple_extended_tuple_with_unicode_string() {
    let v = (42u32, "Hello, 世界! 🌏".to_string(), true);
    roundtrip(v);
}

#[test]
fn test_tuple_extended_tuple_option_chain() {
    // Nested Option in a tuple
    let v: (Option<Option<u32>>, String) = (Some(Some(255)), "chain".to_string());
    roundtrip(v);
    let none_outer: (Option<Option<u32>>, String) = (None, "none".to_string());
    roundtrip(none_outer);
    let some_none: (Option<Option<u32>>, String) = (Some(None), "inner_none".to_string());
    roundtrip(some_none);
}

#[test]
fn test_tuple_extended_vec_of_tuples_large() {
    let items: Vec<(u32, String)> = (0u32..100)
        .map(|i| (i * 17, format!("element_{:04}", i)))
        .collect();
    let enc = encode_to_vec(&items).expect("encode");
    let (dec, _): (Vec<(u32, String)>, _) = decode_from_slice(&enc).expect("decode");
    assert_eq!(items, dec);
}
