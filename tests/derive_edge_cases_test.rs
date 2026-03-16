//! Derive macro edge case tests: lifetimes, all-variant enums, large structs,
//! nested generics, unit-only enums, newtype wrappers, nested Option.

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
    borrow_decode_from_slice, decode_from_slice, encode_to_vec, BorrowDecode, Decode, Encode,
};

// ── 1. Struct with lifetime (BorrowDecode) ────────────────────────────────────

#[derive(Debug, PartialEq, Encode, BorrowDecode)]
struct WithLifetime<'a> {
    borrowed: &'a str,
    owned: u32,
}

#[test]
fn test_lifetime_struct_borrow_decode() {
    let enc = {
        // Encode a concrete instance; the encoded bytes own the string data.
        #[derive(Encode)]
        struct WithLifetimeOwned {
            borrowed: String,
            owned: u32,
        }
        encode_to_vec(&WithLifetimeOwned {
            borrowed: "hello lifetime".to_string(),
            owned: 99,
        })
        .expect("encode")
    };
    let (dec, _): (WithLifetime<'_>, _) = borrow_decode_from_slice(&enc).expect("borrow_decode");
    assert_eq!(dec.borrowed, "hello lifetime");
    assert_eq!(dec.owned, 99);
}

// ── 2. Enum with all four variant kinds ───────────────────────────────────────

#[derive(Debug, PartialEq, Encode, Decode)]
enum AllDataVariants {
    Unit,
    Newtype(u32),
    Tuple(u8, u16, u32),
    Struct { x: i64, y: i64, z: i64 },
}

#[test]
fn test_all_variant_unit_roundtrip() {
    let v = AllDataVariants::Unit;
    let enc = encode_to_vec(&v).expect("encode");
    let (dec, _): (AllDataVariants, _) = decode_from_slice(&enc).expect("decode");
    assert_eq!(v, dec);
}

#[test]
fn test_all_variant_newtype_roundtrip() {
    let v = AllDataVariants::Newtype(42);
    let enc = encode_to_vec(&v).expect("encode");
    let (dec, _): (AllDataVariants, _) = decode_from_slice(&enc).expect("decode");
    assert_eq!(v, dec);
}

#[test]
fn test_all_variant_tuple_roundtrip() {
    let v = AllDataVariants::Tuple(1, 2, 3);
    let enc = encode_to_vec(&v).expect("encode");
    let (dec, _): (AllDataVariants, _) = decode_from_slice(&enc).expect("decode");
    assert_eq!(v, dec);
}

#[test]
fn test_all_variant_struct_roundtrip() {
    let v = AllDataVariants::Struct {
        x: i64::MIN,
        y: 0,
        z: i64::MAX,
    };
    let enc = encode_to_vec(&v).expect("encode");
    let (dec, _): (AllDataVariants, _) = decode_from_slice(&enc).expect("decode");
    assert_eq!(v, dec);
}

// ── 3. Large struct covering all primitive types ───────────────────────────────

#[derive(Debug, PartialEq, Encode, Decode)]
struct LargeStruct {
    f1: u8,
    f2: u16,
    f3: u32,
    f4: u64,
    f5: i8,
    f6: i16,
    f7: i32,
    f8: i64,
    f9: f32,
    f10: f64,
    f11: bool,
    f12: char,
    f13: String,
    f14: Vec<u8>,
    f15: Option<u32>,
}

#[test]
fn test_large_struct_roundtrip() {
    let v = LargeStruct {
        f1: u8::MAX,
        f2: u16::MAX,
        f3: u32::MAX,
        f4: u64::MAX,
        f5: i8::MIN,
        f6: i16::MIN,
        f7: i32::MIN,
        f8: i64::MIN,
        f9: std::f32::consts::PI,
        f10: std::f64::consts::E,
        f11: true,
        f12: '£',
        f13: "large struct test".to_string(),
        f14: vec![0xde, 0xad, 0xbe, 0xef],
        f15: Some(0xCAFE_BABE),
    };
    let enc = encode_to_vec(&v).expect("encode");
    let (dec, _): (LargeStruct, _) = decode_from_slice(&enc).expect("decode");
    assert_eq!(v, dec);
}

#[test]
fn test_large_struct_option_none() {
    let v = LargeStruct {
        f1: 0,
        f2: 0,
        f3: 0,
        f4: 0,
        f5: 0,
        f6: 0,
        f7: 0,
        f8: 0,
        f9: 0.0,
        f10: 0.0,
        f11: false,
        f12: 'a',
        f13: String::new(),
        f14: vec![],
        f15: None,
    };
    let enc = encode_to_vec(&v).expect("encode");
    let (dec, _): (LargeStruct, _) = decode_from_slice(&enc).expect("decode");
    assert_eq!(v, dec);
}

// ── 4. Nested generic struct ───────────────────────────────────────────────────

#[derive(Debug, PartialEq, Encode, Decode)]
struct Nested<T: Encode + Decode> {
    inner: T,
    tag: u8,
}

#[test]
fn test_nested_generic_u32() {
    let v = Nested {
        inner: 0xDEAD_u32,
        tag: 7,
    };
    let enc = encode_to_vec(&v).expect("encode");
    let (dec, _): (Nested<u32>, _) = decode_from_slice(&enc).expect("decode");
    assert_eq!(v, dec);
}

#[test]
fn test_nested_generic_string() {
    let v = Nested {
        inner: "nested string".to_string(),
        tag: 42,
    };
    let enc = encode_to_vec(&v).expect("encode");
    let (dec, _): (Nested<String>, _) = decode_from_slice(&enc).expect("decode");
    assert_eq!(v, dec);
}

#[test]
fn test_nested_generic_vec() {
    let v = Nested {
        inner: vec![1u8, 2, 3, 4, 5],
        tag: 0,
    };
    let enc = encode_to_vec(&v).expect("encode");
    let (dec, _): (Nested<Vec<u8>>, _) = decode_from_slice(&enc).expect("decode");
    assert_eq!(v, dec);
}

#[test]
fn test_nested_double_wrap() {
    let v = Nested {
        inner: Nested {
            inner: 99u32,
            tag: 1,
        },
        tag: 2,
    };
    let enc = encode_to_vec(&v).expect("encode");
    let (dec, _): (Nested<Nested<u32>>, _) = decode_from_slice(&enc).expect("decode");
    assert_eq!(v, dec);
}

// ── 5. Unit-only enum (all variants are unit) ──────────────────────────────────

#[derive(Debug, PartialEq, Encode, Decode)]
enum UnitOnlyEnum {
    A,
    B,
    C,
    D,
}

#[test]
fn test_unit_only_enum_all_variants() {
    for v in [
        UnitOnlyEnum::A,
        UnitOnlyEnum::B,
        UnitOnlyEnum::C,
        UnitOnlyEnum::D,
    ] {
        let enc = encode_to_vec(&v).expect("encode");
        let (dec, _): (UnitOnlyEnum, _) = decode_from_slice(&enc).expect("decode");
        assert_eq!(v, dec);
    }
}

// ── 6. Newtype wrapper ─────────────────────────────────────────────────────────

#[derive(Debug, PartialEq, Encode, Decode)]
struct Wrapper(u64);

#[test]
fn test_newtype_wrapper_max() {
    let v = Wrapper(u64::MAX);
    let enc = encode_to_vec(&v).expect("encode");
    let (dec, _): (Wrapper, _) = decode_from_slice(&enc).expect("decode");
    assert_eq!(v, dec);
}

#[test]
fn test_newtype_wrapper_zero() {
    let v = Wrapper(0);
    let enc = encode_to_vec(&v).expect("encode");
    let (dec, _): (Wrapper, _) = decode_from_slice(&enc).expect("decode");
    assert_eq!(v, dec);
}

// ── 7. Nested Option ──────────────────────────────────────────────────────────

#[test]
fn test_option_option_some_some() {
    let v: Option<Option<u32>> = Some(Some(42));
    let enc = encode_to_vec(&v).expect("encode");
    let (dec, _): (Option<Option<u32>>, _) = decode_from_slice(&enc).expect("decode");
    assert_eq!(v, dec);
}

#[test]
fn test_option_option_some_none() {
    let v: Option<Option<u32>> = Some(None);
    let enc = encode_to_vec(&v).expect("encode");
    let (dec, _): (Option<Option<u32>>, _) = decode_from_slice(&enc).expect("decode");
    assert_eq!(v, dec);
}

#[test]
fn test_option_option_none() {
    let v: Option<Option<u32>> = None;
    let enc = encode_to_vec(&v).expect("encode");
    let (dec, _): (Option<Option<u32>>, _) = decode_from_slice(&enc).expect("decode");
    assert_eq!(v, dec);
}

// ── 8. Enum containing structs with String fields ─────────────────────────────

#[derive(Debug, PartialEq, Encode, Decode)]
enum RichEnum {
    Empty,
    Tagged { id: u64, label: String },
    Pair(String, String),
    Scalar(i64),
}

#[test]
fn test_rich_enum_empty() {
    let v = RichEnum::Empty;
    let enc = encode_to_vec(&v).expect("encode");
    let (dec, _): (RichEnum, _) = decode_from_slice(&enc).expect("decode");
    assert_eq!(v, dec);
}

#[test]
fn test_rich_enum_tagged() {
    let v = RichEnum::Tagged {
        id: 9999,
        label: "oxicode".to_string(),
    };
    let enc = encode_to_vec(&v).expect("encode");
    let (dec, _): (RichEnum, _) = decode_from_slice(&enc).expect("decode");
    assert_eq!(v, dec);
}

#[test]
fn test_rich_enum_pair() {
    let v = RichEnum::Pair("key".to_string(), "value".to_string());
    let enc = encode_to_vec(&v).expect("encode");
    let (dec, _): (RichEnum, _) = decode_from_slice(&enc).expect("decode");
    assert_eq!(v, dec);
}

#[test]
fn test_rich_enum_scalar_negative() {
    let v = RichEnum::Scalar(i64::MIN);
    let enc = encode_to_vec(&v).expect("encode");
    let (dec, _): (RichEnum, _) = decode_from_slice(&enc).expect("decode");
    assert_eq!(v, dec);
}
