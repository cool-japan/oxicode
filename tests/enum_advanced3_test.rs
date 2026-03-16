//! Advanced enum encoding scenario tests for OxiCode — 22 test functions.

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
    config, decode_from_slice, encode_to_vec, encode_to_vec_with_config, Decode, Encode,
};

// ---------------------------------------------------------------------------
// Type definitions
// ---------------------------------------------------------------------------

#[derive(Debug, PartialEq, Encode, Decode)]
enum SimpleEnum {
    A,
    B,
    C,
}

#[derive(Debug, PartialEq, Encode, Decode)]
enum DataEnum {
    Unit,
    Newtype(u32),
    Tuple(u32, String),
    Struct { x: u32, y: u32 },
}

#[derive(Debug, PartialEq, Encode, Decode)]
enum BigEnum {
    V0,
    V1,
    V2,
    V3,
    V4,
    V5,
    V6,
    V7,
    V8,
    V9,
    V10,
    V11,
    V12,
    V13,
    V14,
    V15,
}

#[derive(Debug, PartialEq, Encode, Decode)]
enum NestedEnum {
    Outer(DataEnum),
    Other(u32),
}

#[derive(Debug, PartialEq, Encode, Decode)]
#[oxicode(tag_type = "u8")]
enum TaggedEnum {
    First,
    Second,
    Third,
}

#[derive(Debug, PartialEq, Encode, Decode)]
enum CustomDisc {
    #[oxicode(variant = 100)]
    Hundred,
    Normal,
}

#[derive(Debug, PartialEq, Encode, Decode)]
enum Either<L, R> {
    Left(L),
    Right(R),
}

// ---------------------------------------------------------------------------
// Test 1: SimpleEnum::A roundtrip
// ---------------------------------------------------------------------------

#[test]
fn test_simple_enum_a_roundtrip() {
    let original = SimpleEnum::A;
    let enc = encode_to_vec(&original).expect("encode SimpleEnum::A");
    let (val, _): (SimpleEnum, usize) = decode_from_slice(&enc).expect("decode SimpleEnum::A");
    assert_eq!(original, val);
}

// ---------------------------------------------------------------------------
// Test 2: SimpleEnum::B roundtrip
// ---------------------------------------------------------------------------

#[test]
fn test_simple_enum_b_roundtrip() {
    let original = SimpleEnum::B;
    let enc = encode_to_vec(&original).expect("encode SimpleEnum::B");
    let (val, _): (SimpleEnum, usize) = decode_from_slice(&enc).expect("decode SimpleEnum::B");
    assert_eq!(original, val);
}

// ---------------------------------------------------------------------------
// Test 3: SimpleEnum::C roundtrip
// ---------------------------------------------------------------------------

#[test]
fn test_simple_enum_c_roundtrip() {
    let original = SimpleEnum::C;
    let enc = encode_to_vec(&original).expect("encode SimpleEnum::C");
    let (val, _): (SimpleEnum, usize) = decode_from_slice(&enc).expect("decode SimpleEnum::C");
    assert_eq!(original, val);
}

// ---------------------------------------------------------------------------
// Test 4: All three SimpleEnum variants produce different bytes
// ---------------------------------------------------------------------------

#[test]
fn test_simple_enum_variants_produce_different_bytes() {
    let enc_a = encode_to_vec(&SimpleEnum::A).expect("encode SimpleEnum::A");
    let enc_b = encode_to_vec(&SimpleEnum::B).expect("encode SimpleEnum::B");
    let enc_c = encode_to_vec(&SimpleEnum::C).expect("encode SimpleEnum::C");

    assert_ne!(enc_a, enc_b, "A and B must produce different bytes");
    assert_ne!(enc_b, enc_c, "B and C must produce different bytes");
    assert_ne!(enc_a, enc_c, "A and C must produce different bytes");
}

// ---------------------------------------------------------------------------
// Test 5: DataEnum::Unit roundtrip
// ---------------------------------------------------------------------------

#[test]
fn test_data_enum_unit_roundtrip() {
    let original = DataEnum::Unit;
    let enc = encode_to_vec(&original).expect("encode DataEnum::Unit");
    let (val, _): (DataEnum, usize) = decode_from_slice(&enc).expect("decode DataEnum::Unit");
    assert_eq!(original, val);
}

// ---------------------------------------------------------------------------
// Test 6: DataEnum::Newtype(42) roundtrip
// ---------------------------------------------------------------------------

#[test]
fn test_data_enum_newtype_roundtrip() {
    let original = DataEnum::Newtype(42);
    let enc = encode_to_vec(&original).expect("encode DataEnum::Newtype(42)");
    let (val, _): (DataEnum, usize) =
        decode_from_slice(&enc).expect("decode DataEnum::Newtype(42)");
    assert_eq!(original, val);
}

// ---------------------------------------------------------------------------
// Test 7: DataEnum::Tuple(1, "hello") roundtrip
// ---------------------------------------------------------------------------

#[test]
fn test_data_enum_tuple_roundtrip() {
    let original = DataEnum::Tuple(1, "hello".to_string());
    let enc = encode_to_vec(&original).expect("encode DataEnum::Tuple");
    let (val, _): (DataEnum, usize) = decode_from_slice(&enc).expect("decode DataEnum::Tuple");
    assert_eq!(original, val);
}

// ---------------------------------------------------------------------------
// Test 8: DataEnum::Struct{x:5, y:10} roundtrip
// ---------------------------------------------------------------------------

#[test]
fn test_data_enum_struct_roundtrip() {
    let original = DataEnum::Struct { x: 5, y: 10 };
    let enc = encode_to_vec(&original).expect("encode DataEnum::Struct");
    let (val, _): (DataEnum, usize) = decode_from_slice(&enc).expect("decode DataEnum::Struct");
    assert_eq!(original, val);
}

// ---------------------------------------------------------------------------
// Test 9: All four DataEnum variants produce different bytes
// ---------------------------------------------------------------------------

#[test]
fn test_data_enum_all_variants_different_bytes() {
    let enc_unit = encode_to_vec(&DataEnum::Unit).expect("encode DataEnum::Unit");
    let enc_newtype = encode_to_vec(&DataEnum::Newtype(42)).expect("encode DataEnum::Newtype");
    let enc_tuple =
        encode_to_vec(&DataEnum::Tuple(1, "hello".to_string())).expect("encode DataEnum::Tuple");
    let enc_struct =
        encode_to_vec(&DataEnum::Struct { x: 5, y: 10 }).expect("encode DataEnum::Struct");

    assert_ne!(enc_unit, enc_newtype, "Unit and Newtype must differ");
    assert_ne!(enc_unit, enc_tuple, "Unit and Tuple must differ");
    assert_ne!(enc_unit, enc_struct, "Unit and Struct must differ");
    assert_ne!(enc_newtype, enc_tuple, "Newtype and Tuple must differ");
    assert_ne!(enc_newtype, enc_struct, "Newtype and Struct must differ");
    assert_ne!(enc_tuple, enc_struct, "Tuple and Struct must differ");
}

// ---------------------------------------------------------------------------
// Test 10: BigEnum::V0 roundtrip
// ---------------------------------------------------------------------------

#[test]
fn test_big_enum_v0_roundtrip() {
    let original = BigEnum::V0;
    let enc = encode_to_vec(&original).expect("encode BigEnum::V0");
    let (val, _): (BigEnum, usize) = decode_from_slice(&enc).expect("decode BigEnum::V0");
    assert_eq!(original, val);
}

// ---------------------------------------------------------------------------
// Test 11: BigEnum::V15 roundtrip (last variant)
// ---------------------------------------------------------------------------

#[test]
fn test_big_enum_v15_roundtrip() {
    let original = BigEnum::V15;
    let enc = encode_to_vec(&original).expect("encode BigEnum::V15");
    let (val, _): (BigEnum, usize) = decode_from_slice(&enc).expect("decode BigEnum::V15");
    assert_eq!(original, val);
}

// ---------------------------------------------------------------------------
// Test 12: Vec<DataEnum> with all variants roundtrip
// ---------------------------------------------------------------------------

#[test]
fn test_vec_data_enum_all_variants_roundtrip() {
    let original = vec![
        DataEnum::Unit,
        DataEnum::Newtype(42),
        DataEnum::Tuple(1, "hello".to_string()),
        DataEnum::Struct { x: 5, y: 10 },
    ];
    let enc = encode_to_vec(&original).expect("encode Vec<DataEnum>");
    let (val, _): (Vec<DataEnum>, usize) = decode_from_slice(&enc).expect("decode Vec<DataEnum>");
    assert_eq!(original, val);
}

// ---------------------------------------------------------------------------
// Test 13: Option<DataEnum> Some(Unit) roundtrip
// ---------------------------------------------------------------------------

#[test]
fn test_option_data_enum_some_roundtrip() {
    let original: Option<DataEnum> = Some(DataEnum::Unit);
    let enc = encode_to_vec(&original).expect("encode Option<DataEnum> Some(Unit)");
    let (val, _): (Option<DataEnum>, usize) =
        decode_from_slice(&enc).expect("decode Option<DataEnum> Some(Unit)");
    assert_eq!(original, val);
}

// ---------------------------------------------------------------------------
// Test 14: Option<DataEnum> None roundtrip
// ---------------------------------------------------------------------------

#[test]
fn test_option_data_enum_none_roundtrip() {
    let original: Option<DataEnum> = None;
    let enc = encode_to_vec(&original).expect("encode Option<DataEnum> None");
    let (val, _): (Option<DataEnum>, usize) =
        decode_from_slice(&enc).expect("decode Option<DataEnum> None");
    assert_eq!(original, val);
}

// ---------------------------------------------------------------------------
// Test 15: NestedEnum::Outer(DataEnum::Newtype(99)) roundtrip
// ---------------------------------------------------------------------------

#[test]
fn test_nested_enum_outer_newtype_roundtrip() {
    let original = NestedEnum::Outer(DataEnum::Newtype(99));
    let enc = encode_to_vec(&original).expect("encode NestedEnum::Outer(Newtype(99))");
    let (val, _): (NestedEnum, usize) =
        decode_from_slice(&enc).expect("decode NestedEnum::Outer(Newtype(99))");
    assert_eq!(original, val);
}

// ---------------------------------------------------------------------------
// Test 16: NestedEnum::Other(42) roundtrip
// ---------------------------------------------------------------------------

#[test]
fn test_nested_enum_other_roundtrip() {
    let original = NestedEnum::Other(42);
    let enc = encode_to_vec(&original).expect("encode NestedEnum::Other(42)");
    let (val, _): (NestedEnum, usize) =
        decode_from_slice(&enc).expect("decode NestedEnum::Other(42)");
    assert_eq!(original, val);
}

// ---------------------------------------------------------------------------
// Test 17: Discriminant check: SimpleEnum::A has discriminant 0
// ---------------------------------------------------------------------------

#[test]
fn test_simple_enum_a_discriminant_is_zero() {
    let enc = encode_to_vec(&SimpleEnum::A).expect("encode SimpleEnum::A");
    // First byte is the discriminant in the default varint encoding.
    assert!(!enc.is_empty(), "encoded bytes must not be empty");
    assert_eq!(enc[0], 0u8, "SimpleEnum::A discriminant must be 0");
}

// ---------------------------------------------------------------------------
// Test 18: custom tag_type with u8: #[oxicode(tag_type = "u8")] on enum
// ---------------------------------------------------------------------------

#[test]
fn test_tagged_enum_u8_tag_type() {
    // With u8 tag_type and legacy (fixed-int) config, discriminant is exactly 1 byte.
    let enc_first =
        encode_to_vec_with_config(&TaggedEnum::First, config::legacy()).expect("encode First");
    let enc_second =
        encode_to_vec_with_config(&TaggedEnum::Second, config::legacy()).expect("encode Second");
    let enc_third =
        encode_to_vec_with_config(&TaggedEnum::Third, config::legacy()).expect("encode Third");

    assert_eq!(
        enc_first.len(),
        1,
        "u8 tag First must be 1 byte; got {:?}",
        enc_first
    );
    assert_eq!(
        enc_second.len(),
        1,
        "u8 tag Second must be 1 byte; got {:?}",
        enc_second
    );
    assert_eq!(
        enc_third.len(),
        1,
        "u8 tag Third must be 1 byte; got {:?}",
        enc_third
    );

    assert_eq!(enc_first[0], 0u8, "First discriminant must be 0");
    assert_eq!(enc_second[0], 1u8, "Second discriminant must be 1");
    assert_eq!(enc_third[0], 2u8, "Third discriminant must be 2");

    // Roundtrip
    let (val_first, _): (TaggedEnum, usize) =
        decode_from_slice(&enc_first).expect("decode TaggedEnum::First");
    assert_eq!(val_first, TaggedEnum::First);

    let (val_second, _): (TaggedEnum, usize) =
        decode_from_slice(&enc_second).expect("decode TaggedEnum::Second");
    assert_eq!(val_second, TaggedEnum::Second);
}

// ---------------------------------------------------------------------------
// Test 19: custom variant: #[oxicode(variant = 100)] on a specific variant
// ---------------------------------------------------------------------------

#[test]
fn test_custom_disc_hundred_variant() {
    let enc_hundred = encode_to_vec(&CustomDisc::Hundred).expect("encode CustomDisc::Hundred");

    // The variant attribute sets the discriminant to 100.
    // Varint encoding: values 0..=250 encode as a single byte.
    // 100 fits in one byte.
    assert!(!enc_hundred.is_empty(), "encoded bytes must not be empty");
    assert_eq!(enc_hundred[0], 100u8, "Hundred discriminant must be 100");

    // Roundtrip
    let (val, _): (CustomDisc, usize) =
        decode_from_slice(&enc_hundred).expect("decode CustomDisc::Hundred");
    assert_eq!(val, CustomDisc::Hundred);

    // Normal variant roundtrip
    let enc_normal = encode_to_vec(&CustomDisc::Normal).expect("encode CustomDisc::Normal");
    let (val_normal, _): (CustomDisc, usize) =
        decode_from_slice(&enc_normal).expect("decode CustomDisc::Normal");
    assert_eq!(val_normal, CustomDisc::Normal);
}

// ---------------------------------------------------------------------------
// Test 20: Enum with generic type parameter roundtrip
// ---------------------------------------------------------------------------

#[test]
fn test_either_generic_enum_roundtrip() {
    let left: Either<u32, String> = Either::Left(42);
    let enc_left = encode_to_vec(&left).expect("encode Either::Left(42)");
    let (val_left, _): (Either<u32, String>, usize) =
        decode_from_slice(&enc_left).expect("decode Either::Left(42)");
    assert_eq!(left, val_left);

    let right: Either<u32, String> = Either::Right("hello".to_string());
    let enc_right = encode_to_vec(&right).expect("encode Either::Right(\"hello\")");
    let (val_right, _): (Either<u32, String>, usize) =
        decode_from_slice(&enc_right).expect("decode Either::Right(\"hello\")");
    assert_eq!(right, val_right);

    // Left and Right must produce different encodings
    assert_ne!(
        enc_left, enc_right,
        "Left and Right must produce different bytes"
    );
}

// ---------------------------------------------------------------------------
// Test 21: Vec<SimpleEnum> with 10 elements roundtrip
// ---------------------------------------------------------------------------

#[test]
fn test_vec_simple_enum_ten_elements_roundtrip() {
    let original = vec![
        SimpleEnum::A,
        SimpleEnum::B,
        SimpleEnum::C,
        SimpleEnum::A,
        SimpleEnum::C,
        SimpleEnum::B,
        SimpleEnum::A,
        SimpleEnum::A,
        SimpleEnum::B,
        SimpleEnum::C,
    ];
    assert_eq!(original.len(), 10, "must have exactly 10 elements");

    let enc = encode_to_vec(&original).expect("encode Vec<SimpleEnum>");
    let (val, _): (Vec<SimpleEnum>, usize) =
        decode_from_slice(&enc).expect("decode Vec<SimpleEnum>");
    assert_eq!(original, val);
}

// ---------------------------------------------------------------------------
// Test 22: DataEnum consumed bytes == encoded length for all variants
// ---------------------------------------------------------------------------

#[test]
fn test_data_enum_consumed_bytes_equals_encoded_length() {
    let variants = vec![
        DataEnum::Unit,
        DataEnum::Newtype(42),
        DataEnum::Tuple(1, "hello".to_string()),
        DataEnum::Struct { x: 5, y: 10 },
    ];

    for variant in &variants {
        let enc = encode_to_vec(variant).expect("encode DataEnum variant");
        let (_, consumed): (DataEnum, usize) =
            decode_from_slice(&enc).expect("decode DataEnum variant");
        assert_eq!(
            consumed,
            enc.len(),
            "consumed bytes ({consumed}) must equal encoded length ({}) for {:?}",
            enc.len(),
            variant
        );
    }
}
