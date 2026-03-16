//! Advanced enum encoding tests for OxiCode.
//!
//! Covers complex enum patterns: mixed variants, large discriminants, nested enums,
//! recursive types, generics, custom tags, and attribute-driven behaviour.

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
use oxicode::{config, decode_from_slice, encode_to_vec, Decode, Encode};
use std::collections::HashMap;

// ---------------------------------------------------------------------------
// Top-level enum definitions
// ---------------------------------------------------------------------------

#[derive(Encode, Decode, PartialEq, Debug)]
enum SimpleUnit {
    Alpha,
    Beta,
    Gamma,
}

#[derive(Encode, Decode, PartialEq, Debug)]
enum NewtypeEnum {
    NewtypeU32(u32),
    NewtypeString(String),
}

#[derive(Encode, Decode, PartialEq, Debug)]
enum StructVariantEnum {
    Contains { x: u32, y: u32 },
    Empty,
}

#[derive(Encode, Decode, PartialEq, Debug)]
enum MixedEnum {
    Unit,
    Tuple(u32, String),
    Struct { id: u64, name: String },
}

#[derive(Encode, Decode, PartialEq, Debug)]
enum LargeDiscriminantEnum {
    Small,
    #[oxicode(variant = 255)]
    At255,
    #[oxicode(variant = 256)]
    At256,
    #[oxicode(variant = 1000)]
    AtThousand,
}

#[derive(Encode, Decode, PartialEq, Debug)]
enum Inner {
    Low,
    High,
}

#[derive(Encode, Decode, PartialEq, Debug)]
enum Outer {
    First(Inner),
    Second { nested: Inner },
}

#[derive(Encode, Decode, PartialEq, Debug)]
enum U64Payload {
    Value(u64),
    Nothing,
}

#[derive(Encode, Decode, PartialEq, Debug)]
enum StringPayload {
    Text(String),
    Empty,
}

#[derive(Encode, Decode, PartialEq, Debug)]
enum BinaryPayload {
    Data(Vec<u8>),
    NoData,
}

#[derive(Encode, Decode, PartialEq, Debug)]
enum WithSkipped {
    Active,
    #[oxicode(skip)]
    Skipped,
    Next,
}

#[derive(Encode, Decode, PartialEq, Debug)]
enum RenamedVariants {
    #[oxicode(rename = "first")]
    First,
    #[oxicode(rename = "second")]
    Second(u32),
}

#[derive(Encode, Decode, PartialEq, Debug)]
enum Expr {
    Num(i64),
    Add(Box<Expr>, Box<Expr>),
    Neg(Box<Expr>),
}

#[derive(Encode, Decode, PartialEq, Debug)]
enum ResultPayload {
    Success(Result<u32, String>),
    Pending,
}

#[derive(Encode, Decode, PartialEq, Debug)]
enum OptionPayload {
    Present(Option<u64>),
    Absent,
}

#[derive(Encode, Decode, PartialEq, Debug)]
#[oxicode(tag_type = "u8")]
enum TagU8Enum {
    A,
    B,
    C,
}

#[derive(Encode, Decode, PartialEq, Debug)]
enum GenericContainer<T> {
    Empty,
    Single(T),
    Pair(T, T),
}

// ---------------------------------------------------------------------------
// Test 1: Simple unit enum — roundtrip all 3 variants
// ---------------------------------------------------------------------------

#[test]
fn test_simple_unit_enum_roundtrip_all_variants() {
    for variant in [SimpleUnit::Alpha, SimpleUnit::Beta, SimpleUnit::Gamma] {
        let encoded = encode_to_vec(&variant).expect("encode SimpleUnit");
        let (decoded, _): (SimpleUnit, _) = decode_from_slice(&encoded).expect("decode SimpleUnit");
        assert_eq!(variant, decoded);
    }
}

// ---------------------------------------------------------------------------
// Test 2: Enum with tuple variants (NewtypeU32, NewtypeString)
// ---------------------------------------------------------------------------

#[test]
fn test_enum_newtype_variants_roundtrip() {
    let v1 = NewtypeEnum::NewtypeU32(u32::MAX);
    let enc1 = encode_to_vec(&v1).expect("encode NewtypeU32");
    let (dec1, _): (NewtypeEnum, _) = decode_from_slice(&enc1).expect("decode NewtypeU32");
    assert_eq!(v1, dec1);

    let v2 = NewtypeEnum::NewtypeString("oxicode rocks".to_string());
    let enc2 = encode_to_vec(&v2).expect("encode NewtypeString");
    let (dec2, _): (NewtypeEnum, _) = decode_from_slice(&enc2).expect("decode NewtypeString");
    assert_eq!(v2, dec2);
}

// ---------------------------------------------------------------------------
// Test 3: Enum with struct variant
// ---------------------------------------------------------------------------

#[test]
fn test_enum_struct_variant_roundtrip() {
    let v1 = StructVariantEnum::Contains { x: 100, y: 200 };
    let enc1 = encode_to_vec(&v1).expect("encode Contains");
    let (dec1, _): (StructVariantEnum, _) = decode_from_slice(&enc1).expect("decode Contains");
    assert_eq!(v1, dec1);

    let v2 = StructVariantEnum::Empty;
    let enc2 = encode_to_vec(&v2).expect("encode Empty");
    let (dec2, _): (StructVariantEnum, _) = decode_from_slice(&enc2).expect("decode Empty");
    assert_eq!(v2, dec2);
}

// ---------------------------------------------------------------------------
// Test 4: Mixed enum — unit + tuple + struct variants
// ---------------------------------------------------------------------------

#[test]
fn test_mixed_enum_all_variants_roundtrip() {
    let cases = vec![
        MixedEnum::Unit,
        MixedEnum::Tuple(42, "hello".to_string()),
        MixedEnum::Struct {
            id: 9999,
            name: "oxicode".to_string(),
        },
    ];
    for variant in cases {
        let encoded = encode_to_vec(&variant).expect("encode MixedEnum");
        let (decoded, _): (MixedEnum, _) = decode_from_slice(&encoded).expect("decode MixedEnum");
        assert_eq!(variant, decoded);
    }
}

// ---------------------------------------------------------------------------
// Test 5: Enum with variant index > 250 (multi-byte varint discriminant)
// ---------------------------------------------------------------------------

#[test]
fn test_enum_with_many_variants_large_discriminant() {
    for variant in [
        LargeDiscriminantEnum::Small,
        LargeDiscriminantEnum::At255,
        LargeDiscriminantEnum::At256,
        LargeDiscriminantEnum::AtThousand,
    ] {
        let encoded = encode_to_vec(&variant).expect("encode LargeDiscriminantEnum");
        let (decoded, _): (LargeDiscriminantEnum, _) =
            decode_from_slice(&encoded).expect("decode LargeDiscriminantEnum");
        assert_eq!(variant, decoded);
    }
}

// ---------------------------------------------------------------------------
// Test 6: Nested enum in enum
// ---------------------------------------------------------------------------

#[test]
fn test_nested_enum_in_enum_roundtrip() {
    let cases = vec![
        Outer::First(Inner::Low),
        Outer::First(Inner::High),
        Outer::Second { nested: Inner::Low },
    ];
    for variant in cases {
        let encoded = encode_to_vec(&variant).expect("encode Outer");
        let (decoded, _): (Outer, _) = decode_from_slice(&encoded).expect("decode Outer");
        assert_eq!(variant, decoded);
    }
}

// ---------------------------------------------------------------------------
// Test 7: Enum in Vec roundtrip
// ---------------------------------------------------------------------------

#[test]
fn test_enum_in_vec_roundtrip() {
    let vec = vec![Inner::Low, Inner::High, Inner::Low, Inner::High];
    let encoded = encode_to_vec(&vec).expect("encode Vec<Inner>");
    let (decoded, _): (Vec<Inner>, _) = decode_from_slice(&encoded).expect("decode Vec<Inner>");
    assert_eq!(vec, decoded);
}

// ---------------------------------------------------------------------------
// Test 8: Option<MyEnum> Some/None roundtrip
// ---------------------------------------------------------------------------

#[test]
fn test_option_enum_some_and_none_roundtrip() {
    let some_val: Option<SimpleUnit> = Some(SimpleUnit::Beta);
    let enc_some = encode_to_vec(&some_val).expect("encode Some(SimpleUnit)");
    let (dec_some, _): (Option<SimpleUnit>, _) =
        decode_from_slice(&enc_some).expect("decode Some(SimpleUnit)");
    assert_eq!(some_val, dec_some);

    let none_val: Option<SimpleUnit> = None;
    let enc_none = encode_to_vec(&none_val).expect("encode None");
    let (dec_none, _): (Option<SimpleUnit>, _) = decode_from_slice(&enc_none).expect("decode None");
    assert_eq!(none_val, dec_none);
}

// ---------------------------------------------------------------------------
// Test 9: Enum with u64 payload roundtrip
// ---------------------------------------------------------------------------

#[test]
fn test_enum_with_u64_payload_roundtrip() {
    let v1 = U64Payload::Value(u64::MAX);
    let enc1 = encode_to_vec(&v1).expect("encode Value(u64::MAX)");
    let (dec1, _): (U64Payload, _) = decode_from_slice(&enc1).expect("decode Value(u64::MAX)");
    assert_eq!(v1, dec1);

    let v2 = U64Payload::Nothing;
    let enc2 = encode_to_vec(&v2).expect("encode Nothing");
    let (dec2, _): (U64Payload, _) = decode_from_slice(&enc2).expect("decode Nothing");
    assert_eq!(v2, dec2);
}

// ---------------------------------------------------------------------------
// Test 10: Unit variant byte size verification (varint discriminant 0 = 1 byte)
// ---------------------------------------------------------------------------

#[test]
fn test_unit_variant_encoded_size_varint() {
    // SimpleUnit::Alpha has discriminant 0, which encodes as a single varint byte.
    let encoded = encode_to_vec(&SimpleUnit::Alpha).expect("encode Alpha");
    assert_eq!(
        encoded.len(),
        1,
        "Unit variant with discriminant 0 should encode to exactly 1 byte, got {:?}",
        encoded
    );
    assert_eq!(encoded[0], 0u8, "Discriminant 0 should encode as byte 0x00");
}

// ---------------------------------------------------------------------------
// Test 11: Enum with String payload
// ---------------------------------------------------------------------------

#[test]
fn test_enum_with_string_payload_roundtrip() {
    let v1 = StringPayload::Text("hello oxicode".to_string());
    let enc1 = encode_to_vec(&v1).expect("encode Text");
    let (dec1, _): (StringPayload, _) = decode_from_slice(&enc1).expect("decode Text");
    assert_eq!(v1, dec1);

    let v2 = StringPayload::Empty;
    let enc2 = encode_to_vec(&v2).expect("encode Empty");
    let (dec2, _): (StringPayload, _) = decode_from_slice(&enc2).expect("decode Empty");
    assert_eq!(v2, dec2);
}

// ---------------------------------------------------------------------------
// Test 12: Enum with Vec<u8> payload
// ---------------------------------------------------------------------------

#[test]
fn test_enum_with_vec_u8_payload_roundtrip() {
    let v1 = BinaryPayload::Data(vec![0xDE, 0xAD, 0xBE, 0xEF]);
    let enc1 = encode_to_vec(&v1).expect("encode Data");
    let (dec1, _): (BinaryPayload, _) = decode_from_slice(&enc1).expect("decode Data");
    assert_eq!(v1, dec1);

    let v2 = BinaryPayload::NoData;
    let enc2 = encode_to_vec(&v2).expect("encode NoData");
    let (dec2, _): (BinaryPayload, _) = decode_from_slice(&enc2).expect("decode NoData");
    assert_eq!(v2, dec2);
}

// ---------------------------------------------------------------------------
// Test 13: Enum with skip attribute — skipped variant shares discriminant with next
// ---------------------------------------------------------------------------

#[test]
fn test_enum_with_skipped_variant_still_encodes() {
    // Active should roundtrip normally.
    let active = WithSkipped::Active;
    let enc_active = encode_to_vec(&active).expect("encode Active");
    let (dec_active, _): (WithSkipped, _) = decode_from_slice(&enc_active).expect("decode Active");
    assert_eq!(active, dec_active);

    // Next should roundtrip normally.
    let next = WithSkipped::Next;
    let enc_next = encode_to_vec(&next).expect("encode Next");
    let (dec_next, _): (WithSkipped, _) = decode_from_slice(&enc_next).expect("decode Next");
    assert_eq!(next, dec_next);

    // Skipped variant encodes identically to Next (shares its discriminant).
    let enc_skip = encode_to_vec(&WithSkipped::Skipped).expect("encode Skipped");
    assert_eq!(
        enc_skip, enc_next,
        "Skipped variant must encode identically to its successor (Next)"
    );
}

// ---------------------------------------------------------------------------
// Test 14: Large discriminant — varint multi-byte encoding
// ---------------------------------------------------------------------------

#[test]
fn test_large_discriminant_three_byte_varint_encoding() {
    // Discriminant 0 (Small) → varint 1 byte
    let enc_small = encode_to_vec(&LargeDiscriminantEnum::Small).expect("encode Small");
    assert_eq!(enc_small.len(), 1, "discriminant 0 must be 1 varint byte");

    // Discriminant 256 → varint 2 bytes (0x80 0x02 in LEB128 unsigned style)
    let enc_256 = encode_to_vec(&LargeDiscriminantEnum::At256).expect("encode At256");
    assert!(
        enc_256.len() >= 2,
        "discriminant 256 must need at least 2 varint bytes, got {:?}",
        enc_256
    );

    // Discriminant 1000 → also multi-byte
    let enc_1000 = encode_to_vec(&LargeDiscriminantEnum::AtThousand).expect("encode AtThousand");
    assert!(
        enc_1000.len() >= 2,
        "discriminant 1000 must need at least 2 varint bytes, got {:?}",
        enc_1000
    );
}

// ---------------------------------------------------------------------------
// Test 15: Enum variant with rename attribute roundtrip
// ---------------------------------------------------------------------------

#[test]
fn test_enum_variant_with_rename_attribute_roundtrip() {
    let v1 = RenamedVariants::First;
    let enc1 = encode_to_vec(&v1).expect("encode First");
    let (dec1, _): (RenamedVariants, _) = decode_from_slice(&enc1).expect("decode First");
    assert_eq!(v1, dec1);

    let v2 = RenamedVariants::Second(99);
    let enc2 = encode_to_vec(&v2).expect("encode Second");
    let (dec2, _): (RenamedVariants, _) = decode_from_slice(&enc2).expect("decode Second");
    assert_eq!(v2, dec2);
}

// ---------------------------------------------------------------------------
// Test 16: Recursive Expr tree roundtrip
// ---------------------------------------------------------------------------

#[test]
fn test_recursive_expr_tree_roundtrip() {
    // Build: Add(Num(3), Neg(Num(5)))
    let tree = Expr::Add(
        Box::new(Expr::Num(3)),
        Box::new(Expr::Neg(Box::new(Expr::Num(5)))),
    );
    let encoded = encode_to_vec(&tree).expect("encode Expr tree");
    let (decoded, _): (Expr, _) = decode_from_slice(&encoded).expect("decode Expr tree");
    assert_eq!(tree, decoded);
}

// ---------------------------------------------------------------------------
// Test 17: Enum with Result payload roundtrip
// ---------------------------------------------------------------------------

#[test]
fn test_enum_with_result_payload_roundtrip() {
    let cases = vec![
        ResultPayload::Success(Ok(42u32)),
        ResultPayload::Success(Err("error message".to_string())),
        ResultPayload::Pending,
    ];
    for variant in cases {
        let encoded = encode_to_vec(&variant).expect("encode ResultPayload");
        let (decoded, _): (ResultPayload, _) =
            decode_from_slice(&encoded).expect("decode ResultPayload");
        assert_eq!(variant, decoded);
    }
}

// ---------------------------------------------------------------------------
// Test 18: Enum with Option payload roundtrip
// ---------------------------------------------------------------------------

#[test]
fn test_enum_with_option_payload_roundtrip() {
    let cases = vec![
        OptionPayload::Present(Some(999u64)),
        OptionPayload::Present(None),
        OptionPayload::Absent,
    ];
    for variant in cases {
        let encoded = encode_to_vec(&variant).expect("encode OptionPayload");
        let (decoded, _): (OptionPayload, _) =
            decode_from_slice(&encoded).expect("decode OptionPayload");
        assert_eq!(variant, decoded);
    }
}

// ---------------------------------------------------------------------------
// Test 19: Vec<MyEnum> with all variants roundtrip
// ---------------------------------------------------------------------------

#[test]
fn test_vec_with_all_enum_variants_roundtrip() {
    let vec = vec![
        MixedEnum::Unit,
        MixedEnum::Tuple(1, "first".to_string()),
        MixedEnum::Struct {
            id: 42,
            name: "struct_variant".to_string(),
        },
        MixedEnum::Unit,
        MixedEnum::Tuple(2, "second".to_string()),
    ];
    let encoded = encode_to_vec(&vec).expect("encode Vec<MixedEnum>");
    let (decoded, _): (Vec<MixedEnum>, _) =
        decode_from_slice(&encoded).expect("decode Vec<MixedEnum>");
    assert_eq!(vec, decoded);
}

// ---------------------------------------------------------------------------
// Test 20: Enum in HashMap value roundtrip
// ---------------------------------------------------------------------------

#[test]
fn test_enum_in_hashmap_value_roundtrip() {
    let mut map = HashMap::new();
    map.insert("alpha".to_string(), SimpleUnit::Alpha);
    map.insert("beta".to_string(), SimpleUnit::Beta);
    map.insert("gamma".to_string(), SimpleUnit::Gamma);

    let encoded = encode_to_vec(&map).expect("encode HashMap<String, SimpleUnit>");
    let (decoded, _): (HashMap<String, SimpleUnit>, _) =
        decode_from_slice(&encoded).expect("decode HashMap<String, SimpleUnit>");
    assert_eq!(map, decoded);
}

// ---------------------------------------------------------------------------
// Test 21: Enum with tag_type attribute (custom discriminant width)
// ---------------------------------------------------------------------------

#[test]
fn test_enum_with_tag_type_attribute_roundtrip() {
    // Standard (varint) config roundtrip
    for variant in [TagU8Enum::A, TagU8Enum::B, TagU8Enum::C] {
        let encoded = encode_to_vec(&variant).expect("encode TagU8Enum");
        let (decoded, _): (TagU8Enum, _) = decode_from_slice(&encoded).expect("decode TagU8Enum");
        assert_eq!(variant, decoded);
    }

    // Fixed-int (legacy) config: with tag_type="u8" the discriminant is exactly 1 byte
    let legacy = config::legacy();
    let enc_a = oxicode::encode_to_vec_with_config(&TagU8Enum::A, legacy).expect("encode A fixed");
    assert_eq!(
        enc_a.len(),
        1,
        "u8 tag_type with fixed-int config must be 1 byte, got {:?}",
        enc_a
    );
    assert_eq!(enc_a[0], 0u8);

    let enc_b = oxicode::encode_to_vec_with_config(&TagU8Enum::B, legacy).expect("encode B fixed");
    assert_eq!(enc_b.len(), 1);
    assert_eq!(enc_b[0], 1u8);
}

// ---------------------------------------------------------------------------
// Test 22: Generic enum Container<T> roundtrip
// ---------------------------------------------------------------------------

#[test]
fn test_generic_enum_container_roundtrip() {
    // T = u32
    let empty: GenericContainer<u32> = GenericContainer::Empty;
    let enc_empty = encode_to_vec(&empty).expect("encode Empty<u32>");
    let (dec_empty, _): (GenericContainer<u32>, _) =
        decode_from_slice(&enc_empty).expect("decode Empty<u32>");
    assert_eq!(empty, dec_empty);

    let single = GenericContainer::Single(42u32);
    let enc_single = encode_to_vec(&single).expect("encode Single(42u32)");
    let (dec_single, _): (GenericContainer<u32>, _) =
        decode_from_slice(&enc_single).expect("decode Single(42u32)");
    assert_eq!(single, dec_single);

    let pair = GenericContainer::Pair(1u32, 2u32);
    let enc_pair = encode_to_vec(&pair).expect("encode Pair(1, 2)");
    let (dec_pair, _): (GenericContainer<u32>, _) =
        decode_from_slice(&enc_pair).expect("decode Pair(1, 2)");
    assert_eq!(pair, dec_pair);

    // T = String
    let single_str = GenericContainer::Single("hello".to_string());
    let enc_str = encode_to_vec(&single_str).expect("encode Single(String)");
    let (dec_str, _): (GenericContainer<String>, _) =
        decode_from_slice(&enc_str).expect("decode Single(String)");
    assert_eq!(single_str, dec_str);
}
