//! Comprehensive tests for enums with varying variant counts.
//!
//! Tests cover single-variant enums through 100-variant enums, all variant
//! styles (unit, tuple, struct), various tag_type widths, recursive types,
//! and zero-copy BorrowDecode.

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
use oxicode::{BorrowDecode, Decode, Encode};

// ---------------------------------------------------------------------------
// Helper functions
// ---------------------------------------------------------------------------

fn encode_std<T: Encode>(val: &T) -> Vec<u8> {
    oxicode::encode_to_vec(val).expect("encode_std")
}

fn decode_std<T: Decode>(bytes: &[u8]) -> T {
    let (val, _) = oxicode::decode_from_slice(bytes).expect("decode_std");
    val
}

fn encode_fixed<T: Encode>(val: &T) -> Vec<u8> {
    oxicode::encode_to_vec_with_config(val, oxicode::config::legacy()).expect("encode_fixed")
}

#[allow(dead_code)]
fn decode_fixed<T: Decode>(bytes: &[u8]) -> T {
    let (val, _) = oxicode::decode_from_slice_with_config(bytes, oxicode::config::legacy())
        .expect("decode_fixed");
    val
}

// ---------------------------------------------------------------------------
// Test 1: 1-variant enum (unit) — encodes as varint 0
// ---------------------------------------------------------------------------

#[derive(Debug, PartialEq, Encode, Decode)]
enum SingleVariant {
    Only,
}

#[test]
fn test_01_single_unit_variant_encodes_as_zero() {
    let encoded = encode_fixed(&SingleVariant::Only);
    // Default tag_type is u32 — in fixed-int config that is 4 bytes.
    assert_eq!(
        encoded.len(),
        4,
        "single-variant unit enum: 4 bytes (u32 fixed)"
    );
    // Discriminant is 0 in little-endian u32.
    assert_eq!(&encoded[..4], &[0x00, 0x00, 0x00, 0x00]);
    // Roundtrip with standard (varint) config.
    let decoded: SingleVariant = decode_std(&encode_std(&SingleVariant::Only));
    assert_eq!(decoded, SingleVariant::Only);
    // Varint encoding of 0 is a single 0x00 byte.
    let std_bytes = encode_std(&SingleVariant::Only);
    assert_eq!(std_bytes.len(), 1);
    assert_eq!(std_bytes[0], 0x00);
}

// ---------------------------------------------------------------------------
// Test 2: 2-variant unit enum
// ---------------------------------------------------------------------------

#[derive(Debug, PartialEq, Encode, Decode)]
#[oxicode(tag_type = "u8")]
enum TwoVariant {
    First,
    Second,
}

#[test]
fn test_02_two_variant_unit_enum() {
    let a = encode_fixed(&TwoVariant::First);
    let b = encode_fixed(&TwoVariant::Second);
    assert_eq!(a.len(), 1);
    assert_eq!(b.len(), 1);
    assert_eq!(a[0], 0u8);
    assert_eq!(b[0], 1u8);
    assert_eq!(
        decode_std::<TwoVariant>(&encode_std(&TwoVariant::First)),
        TwoVariant::First
    );
    assert_eq!(
        decode_std::<TwoVariant>(&encode_std(&TwoVariant::Second)),
        TwoVariant::Second
    );
}

// ---------------------------------------------------------------------------
// Test 3: 3-variant enum with data
// ---------------------------------------------------------------------------

#[derive(Debug, PartialEq, Encode, Decode)]
#[oxicode(tag_type = "u8")]
enum ThreeVariantData {
    Unit,
    Tuple(u32, u32),
    Named { x: i64, label: String },
}

#[test]
fn test_03_three_variant_with_data() {
    let variants = vec![
        ThreeVariantData::Unit,
        ThreeVariantData::Tuple(100, 200),
        ThreeVariantData::Named {
            x: -42,
            label: "oxicode".into(),
        },
    ];
    for v in &variants {
        let encoded = encode_std(v);
        let decoded: ThreeVariantData = decode_std(&encoded);
        assert_eq!(*v, decoded);
    }
    // Discriminants in fixed-int mode
    assert_eq!(encode_fixed(&ThreeVariantData::Unit)[0], 0u8);
    assert_eq!(encode_fixed(&ThreeVariantData::Tuple(0, 0))[0], 1u8);
    assert_eq!(
        encode_fixed(&ThreeVariantData::Named {
            x: 0,
            label: String::new()
        })[0],
        2u8
    );
}

// ---------------------------------------------------------------------------
// Test 4: 4-variant mixed enum
// ---------------------------------------------------------------------------

#[derive(Debug, PartialEq, Encode, Decode)]
#[oxicode(tag_type = "u8")]
enum FourVariantMixed {
    A,
    B(bool),
    C { value: u64 },
    D(String, Vec<u8>),
}

#[test]
fn test_04_four_variant_mixed() {
    let variants = vec![
        FourVariantMixed::A,
        FourVariantMixed::B(true),
        FourVariantMixed::C { value: u64::MAX },
        FourVariantMixed::D("hello".into(), vec![1, 2, 3]),
    ];
    for v in &variants {
        let bytes = encode_std(v);
        let decoded: FourVariantMixed = decode_std(&bytes);
        assert_eq!(*v, decoded);
    }
    // Verify discriminants
    assert_eq!(encode_fixed(&FourVariantMixed::A)[0], 0u8);
    assert_eq!(encode_fixed(&FourVariantMixed::B(false))[0], 1u8);
    assert_eq!(encode_fixed(&FourVariantMixed::C { value: 0 })[0], 2u8);
    assert_eq!(
        encode_fixed(&FourVariantMixed::D(String::new(), vec![]))[0],
        3u8
    );
}

// ---------------------------------------------------------------------------
// Test 5: 8-variant enum
// ---------------------------------------------------------------------------

#[derive(Debug, PartialEq, Encode, Decode)]
#[oxicode(tag_type = "u8")]
enum EightVariant {
    V0,
    V1,
    V2,
    V3,
    V4,
    V5,
    V6,
    V7,
}

#[test]
fn test_05_eight_variant_enum() {
    let all = [
        EightVariant::V0,
        EightVariant::V1,
        EightVariant::V2,
        EightVariant::V3,
        EightVariant::V4,
        EightVariant::V5,
        EightVariant::V6,
        EightVariant::V7,
    ];
    for (idx, v) in all.iter().enumerate() {
        let fixed = encode_fixed(v);
        assert_eq!(fixed.len(), 1);
        assert_eq!(fixed[0], idx as u8, "variant {idx} discriminant mismatch");
        let decoded: EightVariant = decode_std(&encode_std(v));
        assert_eq!(*v, decoded);
    }
}

// ---------------------------------------------------------------------------
// Test 6: 16-variant enum (all unit variants)
// ---------------------------------------------------------------------------

#[derive(Debug, PartialEq, Encode, Decode)]
#[oxicode(tag_type = "u8")]
enum SixteenVariant {
    V00,
    V01,
    V02,
    V03,
    V04,
    V05,
    V06,
    V07,
    V08,
    V09,
    V10,
    V11,
    V12,
    V13,
    V14,
    V15,
}

#[test]
fn test_06_sixteen_unit_variants() {
    let all = [
        SixteenVariant::V00,
        SixteenVariant::V01,
        SixteenVariant::V02,
        SixteenVariant::V03,
        SixteenVariant::V04,
        SixteenVariant::V05,
        SixteenVariant::V06,
        SixteenVariant::V07,
        SixteenVariant::V08,
        SixteenVariant::V09,
        SixteenVariant::V10,
        SixteenVariant::V11,
        SixteenVariant::V12,
        SixteenVariant::V13,
        SixteenVariant::V14,
        SixteenVariant::V15,
    ];
    for (idx, v) in all.iter().enumerate() {
        let fixed = encode_fixed(v);
        assert_eq!(fixed.len(), 1);
        assert_eq!(
            fixed[0], idx as u8,
            "SixteenVariant::{idx} discriminant wrong"
        );
        let decoded: SixteenVariant = decode_std(&encode_std(v));
        assert_eq!(*v, decoded);
    }
}

// ---------------------------------------------------------------------------
// Test 7: 32-variant enum
// ---------------------------------------------------------------------------

#[derive(Debug, PartialEq, Encode, Decode)]
#[oxicode(tag_type = "u8")]
enum ThirtyTwoVariant {
    V00,
    V01,
    V02,
    V03,
    V04,
    V05,
    V06,
    V07,
    V08,
    V09,
    V10,
    V11,
    V12,
    V13,
    V14,
    V15,
    V16,
    V17,
    V18,
    V19,
    V20,
    V21,
    V22,
    V23,
    V24,
    V25,
    V26,
    V27,
    V28,
    V29,
    V30,
    V31,
}

#[test]
fn test_07_thirty_two_variant_enum() {
    let samples = [
        (ThirtyTwoVariant::V00, 0u8),
        (ThirtyTwoVariant::V07, 7u8),
        (ThirtyTwoVariant::V15, 15u8),
        (ThirtyTwoVariant::V16, 16u8),
        (ThirtyTwoVariant::V31, 31u8),
    ];
    for (v, expected_disc) in &samples {
        let fixed = encode_fixed(v);
        assert_eq!(fixed.len(), 1);
        assert_eq!(fixed[0], *expected_disc);
        let decoded: ThirtyTwoVariant = decode_std(&encode_std(v));
        assert_eq!(*v, decoded);
    }
}

// ---------------------------------------------------------------------------
// Test 8: 100-variant enum — test a representative sample
// ---------------------------------------------------------------------------

macro_rules! define_hundred_variant {
    ($name:ident; $($variant:ident),+) => {
        #[derive(Debug, PartialEq, Encode, Decode)]
        #[oxicode(tag_type = "u8")]
        enum $name { $($variant),+ }
    };
}

define_hundred_variant!(HundredVariant;
    V000, V001, V002, V003, V004, V005, V006, V007, V008, V009,
    V010, V011, V012, V013, V014, V015, V016, V017, V018, V019,
    V020, V021, V022, V023, V024, V025, V026, V027, V028, V029,
    V030, V031, V032, V033, V034, V035, V036, V037, V038, V039,
    V040, V041, V042, V043, V044, V045, V046, V047, V048, V049,
    V050, V051, V052, V053, V054, V055, V056, V057, V058, V059,
    V060, V061, V062, V063, V064, V065, V066, V067, V068, V069,
    V070, V071, V072, V073, V074, V075, V076, V077, V078, V079,
    V080, V081, V082, V083, V084, V085, V086, V087, V088, V089,
    V090, V091, V092, V093, V094, V095, V096, V097, V098, V099
);

#[test]
fn test_08_hundred_variant_sample() {
    // Test first, middle, and last variants
    let first = encode_fixed(&HundredVariant::V000);
    assert_eq!(first[0], 0u8);
    assert_eq!(
        decode_std::<HundredVariant>(&encode_std(&HundredVariant::V000)),
        HundredVariant::V000
    );

    let mid = encode_fixed(&HundredVariant::V049);
    assert_eq!(mid[0], 49u8);
    assert_eq!(
        decode_std::<HundredVariant>(&encode_std(&HundredVariant::V049)),
        HundredVariant::V049
    );

    let last = encode_fixed(&HundredVariant::V099);
    assert_eq!(last[0], 99u8);
    assert_eq!(
        decode_std::<HundredVariant>(&encode_std(&HundredVariant::V099)),
        HundredVariant::V099
    );

    let near_last = encode_fixed(&HundredVariant::V098);
    assert_eq!(near_last[0], 98u8);
    assert_eq!(
        decode_std::<HundredVariant>(&encode_std(&HundredVariant::V098)),
        HundredVariant::V098,
    );
}

// ---------------------------------------------------------------------------
// Test 9: Enum with only tuple variants
// ---------------------------------------------------------------------------

#[derive(Debug, PartialEq, Encode, Decode)]
#[oxicode(tag_type = "u8")]
enum OnlyTupleVariants {
    Pair(u32, u32),
    Triple(i8, i16, i32),
    Bytes(Vec<u8>),
}

#[test]
fn test_09_only_tuple_variants() {
    let vals = vec![
        OnlyTupleVariants::Pair(1, 2),
        OnlyTupleVariants::Triple(-1, -2, -3),
        OnlyTupleVariants::Bytes(vec![0xDE, 0xAD, 0xBE, 0xEF]),
    ];
    for v in &vals {
        let encoded = encode_std(v);
        let decoded: OnlyTupleVariants = decode_std(&encoded);
        assert_eq!(*v, decoded);
    }
    assert_eq!(encode_fixed(&OnlyTupleVariants::Pair(0, 0))[0], 0u8);
    assert_eq!(encode_fixed(&OnlyTupleVariants::Triple(0, 0, 0))[0], 1u8);
    assert_eq!(encode_fixed(&OnlyTupleVariants::Bytes(vec![]))[0], 2u8);
}

// ---------------------------------------------------------------------------
// Test 10: Enum with only struct variants
// ---------------------------------------------------------------------------

#[derive(Debug, PartialEq, Encode, Decode)]
#[oxicode(tag_type = "u8")]
enum OnlyStructVariants {
    Point { x: f32, y: f32 },
    Rect { x: i32, y: i32, w: u32, h: u32 },
    Person { name: String, age: u8 },
}

#[test]
fn test_10_only_struct_variants() {
    let vals = vec![
        OnlyStructVariants::Point { x: 1.5, y: -2.5 },
        OnlyStructVariants::Rect {
            x: -10,
            y: 5,
            w: 100,
            h: 200,
        },
        OnlyStructVariants::Person {
            name: "Alice".into(),
            age: 30,
        },
    ];
    for v in &vals {
        let encoded = encode_std(v);
        let decoded: OnlyStructVariants = decode_std(&encoded);
        assert_eq!(*v, decoded);
    }
    assert_eq!(
        encode_fixed(&OnlyStructVariants::Point { x: 0.0, y: 0.0 })[0],
        0u8
    );
    assert_eq!(
        encode_fixed(&OnlyStructVariants::Rect {
            x: 0,
            y: 0,
            w: 0,
            h: 0
        })[0],
        1u8
    );
    assert_eq!(
        encode_fixed(&OnlyStructVariants::Person {
            name: String::new(),
            age: 0
        })[0],
        2u8
    );
}

// ---------------------------------------------------------------------------
// Test 11: Enum with unit + tuple + struct mix
// ---------------------------------------------------------------------------

#[derive(Debug, PartialEq, Encode, Decode)]
#[oxicode(tag_type = "u8")]
enum MixedStyleEnum {
    Empty,
    Flag(bool),
    Count(u64),
    Coord { lat: f64, lon: f64 },
    Named { id: u32, tag: String },
}

#[test]
fn test_11_unit_tuple_struct_mix() {
    let vals = vec![
        MixedStyleEnum::Empty,
        MixedStyleEnum::Flag(false),
        MixedStyleEnum::Count(u64::MAX),
        MixedStyleEnum::Coord {
            lat: 51.5074,
            lon: -0.1278,
        },
        MixedStyleEnum::Named {
            id: 999,
            tag: "london".into(),
        },
    ];
    for v in &vals {
        let encoded = encode_std(v);
        let decoded: MixedStyleEnum = decode_std(&encoded);
        assert_eq!(*v, decoded);
    }
}

// ---------------------------------------------------------------------------
// Test 12: Enum with large tuple data
// ---------------------------------------------------------------------------

#[derive(Debug, PartialEq, Encode, Decode)]
#[oxicode(tag_type = "u8")]
enum LargeTupleData {
    Small(u8),
    Medium(u32, u32, u32, u32),
    Large(Vec<u64>),
    Mixed(String, Vec<u32>, bool, u64),
}

#[test]
fn test_12_large_tuple_data() {
    let large_vec: Vec<u64> = (0..1000).map(|i| i * i).collect();
    let vals = vec![
        LargeTupleData::Small(255),
        LargeTupleData::Medium(u32::MAX, 0, 1234567890, 42),
        LargeTupleData::Large(large_vec.clone()),
        LargeTupleData::Mixed("a".repeat(500), (0u32..100).collect(), true, u64::MAX / 2),
    ];
    for v in &vals {
        let encoded = encode_std(v);
        let decoded: LargeTupleData = decode_std(&encoded);
        assert_eq!(*v, decoded);
    }
    // Size sanity: large variant must be bigger than small variant
    let small_size = encode_std(&LargeTupleData::Small(0)).len();
    let large_size = encode_std(&LargeTupleData::Large(large_vec)).len();
    assert!(large_size > small_size);
}

// ---------------------------------------------------------------------------
// Test 13: Enum with tag_type = "u8" (0-255 range)
// ---------------------------------------------------------------------------

#[derive(Debug, PartialEq, Encode, Decode)]
#[oxicode(tag_type = "u8")]
enum TagU8Enum {
    Alpha,
    Beta(u32),
    Gamma { val: String },
}

#[test]
fn test_13_tag_type_u8() {
    let fixed_alpha = encode_fixed(&TagU8Enum::Alpha);
    assert_eq!(fixed_alpha.len(), 1, "u8 tag: 1 byte discriminant");
    assert_eq!(fixed_alpha[0], 0u8);

    let fixed_beta = encode_fixed(&TagU8Enum::Beta(0));
    assert_eq!(
        fixed_beta.len(),
        1 + 4,
        "u8 tag + u32 payload = 5 bytes fixed"
    );
    assert_eq!(fixed_beta[0], 1u8);

    let fixed_gamma = encode_fixed(&TagU8Enum::Gamma { val: String::new() });
    // tag (1) + string length as u64 (8) + 0 chars = 9 bytes
    assert_eq!(fixed_gamma.len(), 9, "u8 tag + empty string fixed encoding");
    assert_eq!(fixed_gamma[0], 2u8);

    for v in [
        TagU8Enum::Alpha,
        TagU8Enum::Beta(42),
        TagU8Enum::Gamma { val: "hi".into() },
    ] {
        let decoded: TagU8Enum = decode_std(&encode_std(&v));
        assert_eq!(v, decoded);
    }
}

// ---------------------------------------------------------------------------
// Test 14: Enum with tag_type = "u16"
// ---------------------------------------------------------------------------

#[derive(Debug, PartialEq, Encode, Decode)]
#[oxicode(tag_type = "u16")]
enum TagU16Enum {
    Zero,
    One(i32),
    Two { a: u8, b: u8 },
}

#[test]
fn test_14_tag_type_u16() {
    let fixed_zero = encode_fixed(&TagU16Enum::Zero);
    assert_eq!(fixed_zero.len(), 2, "u16 tag: 2 bytes discriminant");
    assert_eq!(&fixed_zero[..2], &[0x00, 0x00]);

    let fixed_one = encode_fixed(&TagU16Enum::One(0));
    assert_eq!(
        fixed_one.len(),
        2 + 4,
        "u16 tag + i32 payload = 6 bytes fixed"
    );
    assert_eq!(&fixed_one[..2], &[0x01, 0x00]); // little-endian 1

    for v in [
        TagU16Enum::Zero,
        TagU16Enum::One(-99),
        TagU16Enum::Two { a: 1, b: 2 },
    ] {
        let decoded: TagU16Enum = decode_std(&encode_std(&v));
        assert_eq!(v, decoded);
    }
}

// ---------------------------------------------------------------------------
// Test 15: Enum with tag_type = "u32"
// ---------------------------------------------------------------------------

#[derive(Debug, PartialEq, Encode, Decode)]
#[oxicode(tag_type = "u32")]
enum TagU32Enum {
    Alpha,
    Beta { data: Vec<u8> },
}

#[test]
fn test_15_tag_type_u32() {
    let fixed_alpha = encode_fixed(&TagU32Enum::Alpha);
    assert_eq!(fixed_alpha.len(), 4, "u32 tag: 4 bytes discriminant");
    assert_eq!(&fixed_alpha[..4], &[0x00, 0x00, 0x00, 0x00]);

    let fixed_beta = encode_fixed(&TagU32Enum::Beta { data: vec![] });
    // tag (4) + seq length as u64 (8) + 0 bytes = 12 bytes
    assert_eq!(fixed_beta.len(), 12);
    assert_eq!(&fixed_beta[..4], &[0x01, 0x00, 0x00, 0x00]); // little-endian 1

    for v in [
        TagU32Enum::Alpha,
        TagU32Enum::Beta {
            data: vec![0xFF, 0x00, 0xAB],
        },
    ] {
        let decoded: TagU32Enum = decode_std(&encode_std(&v));
        assert_eq!(v, decoded);
    }
}

// ---------------------------------------------------------------------------
// Test 16: Discriminant byte verification (first byte = variant index)
// ---------------------------------------------------------------------------

#[derive(Debug, PartialEq, Encode, Decode)]
#[oxicode(tag_type = "u8")]
enum DiscriminantVerify {
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

#[test]
fn test_16_discriminant_byte_verification() {
    // In fixed-int encoding the first byte must equal the variant index exactly.
    let pairs: [(DiscriminantVerify, u8); 16] = [
        (DiscriminantVerify::V0, 0),
        (DiscriminantVerify::V1, 1),
        (DiscriminantVerify::V2, 2),
        (DiscriminantVerify::V3, 3),
        (DiscriminantVerify::V4, 4),
        (DiscriminantVerify::V5, 5),
        (DiscriminantVerify::V6, 6),
        (DiscriminantVerify::V7, 7),
        (DiscriminantVerify::V8, 8),
        (DiscriminantVerify::V9, 9),
        (DiscriminantVerify::V10, 10),
        (DiscriminantVerify::V11, 11),
        (DiscriminantVerify::V12, 12),
        (DiscriminantVerify::V13, 13),
        (DiscriminantVerify::V14, 14),
        (DiscriminantVerify::V15, 15),
    ];
    for (v, expected) in &pairs {
        let bytes = encode_fixed(v);
        assert_eq!(
            bytes[0], *expected,
            "expected discriminant {} for variant index {}",
            expected, expected
        );
        // Also confirm the standard-config first byte equals discriminant (varint of small int = 1 byte)
        let std_bytes = encode_std(v);
        assert_eq!(
            std_bytes[0], *expected,
            "varint of small discriminant must be 1 byte"
        );
    }
}

// ---------------------------------------------------------------------------
// Test 17: Enum with recursive data (Box<T>)
// ---------------------------------------------------------------------------

#[derive(Debug, PartialEq, Encode, Decode)]
#[oxicode(tag_type = "u8")]
enum Tree {
    Leaf(i64),
    Node { left: Box<Tree>, right: Box<Tree> },
}

#[test]
fn test_17_recursive_enum_with_box() {
    let tree = Tree::Node {
        left: Box::new(Tree::Node {
            left: Box::new(Tree::Leaf(1)),
            right: Box::new(Tree::Leaf(2)),
        }),
        right: Box::new(Tree::Node {
            left: Box::new(Tree::Leaf(3)),
            right: Box::new(Tree::Leaf(4)),
        }),
    };
    let encoded = encode_std(&tree);
    let decoded: Tree = decode_std(&encoded);
    assert_eq!(tree, decoded);

    // Leaf roundtrip
    let leaf = Tree::Leaf(-9999);
    let encoded_leaf = encode_std(&leaf);
    let decoded_leaf: Tree = decode_std(&encoded_leaf);
    assert_eq!(leaf, decoded_leaf);
}

// ---------------------------------------------------------------------------
// Test 18: Enum with Vec<String> payload
// ---------------------------------------------------------------------------

#[derive(Debug, PartialEq, Encode, Decode)]
#[oxicode(tag_type = "u8")]
enum VecStringPayload {
    Empty,
    Tags(Vec<String>),
    Metadata {
        keys: Vec<String>,
        values: Vec<String>,
    },
}

#[test]
fn test_18_vec_string_payload() {
    let vals = vec![
        VecStringPayload::Empty,
        VecStringPayload::Tags(vec!["rust".into(), "oxicode".into(), "encode".into()]),
        VecStringPayload::Metadata {
            keys: vec!["name".into(), "version".into()],
            values: vec!["oxicode".into(), "0.2.0".into()],
        },
    ];
    for v in &vals {
        let encoded = encode_std(v);
        let decoded: VecStringPayload = decode_std(&encoded);
        assert_eq!(*v, decoded);
    }
    // Empty vec roundtrip
    let empty_tags = VecStringPayload::Tags(vec![]);
    let decoded: VecStringPayload = decode_std(&encode_std(&empty_tags));
    assert_eq!(empty_tags, decoded);
}

// ---------------------------------------------------------------------------
// Test 19: Enum with Option payload
// ---------------------------------------------------------------------------

#[derive(Debug, PartialEq, Encode, Decode)]
#[oxicode(tag_type = "u8")]
enum OptionPayload {
    Nothing,
    MaybeInt(Option<i32>),
    MaybeStr { label: Option<String> },
    Both { a: Option<u64>, b: Option<bool> },
}

#[test]
fn test_19_option_payload() {
    let vals = vec![
        OptionPayload::Nothing,
        OptionPayload::MaybeInt(None),
        OptionPayload::MaybeInt(Some(42)),
        OptionPayload::MaybeStr { label: None },
        OptionPayload::MaybeStr {
            label: Some("present".into()),
        },
        OptionPayload::Both {
            a: Some(u64::MAX),
            b: Some(false),
        },
        OptionPayload::Both { a: None, b: None },
    ];
    for v in &vals {
        let encoded = encode_std(v);
        let decoded: OptionPayload = decode_std(&encoded);
        assert_eq!(*v, decoded);
    }
}

// ---------------------------------------------------------------------------
// Test 20: encoded_size for each enum variant
// ---------------------------------------------------------------------------

#[derive(Debug, PartialEq, Encode, Decode)]
#[oxicode(tag_type = "u8")]
enum SizedVariants {
    Unit,
    WithU8(u8),
    WithU64(u64),
    WithString(String),
}

#[test]
fn test_20_encoded_size_per_variant() {
    // Unit: varint discriminant (1 byte) + no payload = 1 byte
    let size_unit = oxicode::encoded_size(&SizedVariants::Unit).expect("size unit");
    assert_eq!(size_unit, 1, "Unit variant: tag only = 1 byte");

    // WithU8: 1 (tag) + 1 (u8 varint) = 2 bytes
    let size_u8 = oxicode::encoded_size(&SizedVariants::WithU8(0)).expect("size u8");
    assert_eq!(size_u8, 2, "WithU8: tag(1) + u8(1) = 2");

    // WithU64(0): 1 (tag) + 1 (varint 0) = 2 bytes
    let size_u64_zero = oxicode::encoded_size(&SizedVariants::WithU64(0)).expect("size u64 zero");
    assert_eq!(size_u64_zero, 2, "WithU64(0): tag(1) + varint_0(1) = 2");

    // WithU64(u64::MAX) needs more bytes for varint
    let size_u64_max =
        oxicode::encoded_size(&SizedVariants::WithU64(u64::MAX)).expect("size u64 max");
    assert!(
        size_u64_max > size_u64_zero,
        "large u64 needs more bytes than 0"
    );

    // WithString(""): 1 (tag) + 1 (varint len=0) = 2 bytes
    let size_empty_str =
        oxicode::encoded_size(&SizedVariants::WithString(String::new())).expect("size empty str");
    assert_eq!(
        size_empty_str, 2,
        "WithString(empty): tag(1) + len_varint(1) = 2"
    );

    // WithString("hello"): 1 (tag) + 1 (varint len=5) + 5 (chars) = 7 bytes
    let size_hello =
        oxicode::encoded_size(&SizedVariants::WithString("hello".into())).expect("size hello");
    assert_eq!(
        size_hello, 7,
        "WithString(\"hello\"): tag(1) + len(1) + 5 chars = 7"
    );

    // Verify encode_to_vec lengths match encoded_size
    let actual_unit = encode_std(&SizedVariants::Unit).len();
    assert_eq!(actual_unit, size_unit);
    let actual_hello = encode_std(&SizedVariants::WithString("hello".into())).len();
    assert_eq!(actual_hello, size_hello);
}

// ---------------------------------------------------------------------------
// Test 21: Sequential encode/decode of 100 deterministic variants
// ---------------------------------------------------------------------------

#[derive(Debug, PartialEq, Encode, Decode)]
#[oxicode(tag_type = "u8")]
enum StressVariant {
    Alpha(u64),
    Beta(i32),
    Gamma { name: String, val: u32 },
    Delta(Vec<u8>),
    Epsilon,
}

#[test]
fn test_21_sequential_encode_decode_100_variants() {
    // Build 100 deterministic (pseudo-random-style) variants using a simple LCG sequence
    let mut state: u64 = 0xDEAD_BEEF_CAFE_1234;
    let lcg_next = |s: &mut u64| -> u64 {
        *s = s
            .wrapping_mul(6364136223846793005)
            .wrapping_add(1442695040888963407);
        *s
    };

    let mut variants: Vec<StressVariant> = Vec::with_capacity(100);
    for _ in 0..100 {
        let selector = lcg_next(&mut state) % 5;
        let v = match selector {
            0 => StressVariant::Alpha(lcg_next(&mut state)),
            1 => StressVariant::Beta((lcg_next(&mut state) as i32).wrapping_abs()),
            2 => StressVariant::Gamma {
                name: format!("item_{}", lcg_next(&mut state) % 1000),
                val: (lcg_next(&mut state) % (u32::MAX as u64)) as u32,
            },
            3 => {
                let len = (lcg_next(&mut state) % 32) as usize;
                let bytes: Vec<u8> = (0..len)
                    .map(|_| (lcg_next(&mut state) % 256) as u8)
                    .collect();
                StressVariant::Delta(bytes)
            }
            _ => StressVariant::Epsilon,
        };
        variants.push(v);
    }

    for v in &variants {
        let encoded = encode_std(v);
        let decoded: StressVariant = decode_std(&encoded);
        assert_eq!(*v, decoded, "roundtrip failed for variant {:?}", v);
    }

    // Also verify encode-all-at-once as a vec roundtrips
    let encoded_vec = encode_std(&variants);
    let decoded_vec: Vec<StressVariant> = decode_std(&encoded_vec);
    assert_eq!(variants, decoded_vec);
}

// ---------------------------------------------------------------------------
// Test 22: BorrowDecode for enum with &str variant
// ---------------------------------------------------------------------------

#[derive(Debug, PartialEq, Encode, BorrowDecode)]
#[oxicode(tag_type = "u8")]
enum BorrowEnum<'a> {
    Empty,
    Str(&'a str),
    Bytes(&'a [u8]),
    OwnedInt(u64),
}

#[test]
fn test_22_borrow_decode_enum_with_str_variant() {
    // Encode using standard Encode, then borrow-decode (zero-copy) for &str / &[u8]
    let variants_to_test: [(BorrowEnum<'_>, &str); 4] = [
        (BorrowEnum::Empty, "Empty"),
        (BorrowEnum::Str("zero-copy string"), "Str"),
        (BorrowEnum::Bytes(b"raw bytes"), "Bytes"),
        (BorrowEnum::OwnedInt(0xCAFE_BABE), "OwnedInt"),
    ];

    for (variant, label) in &variants_to_test {
        let encoded = encode_std(variant);
        let (decoded, consumed) = oxicode::borrow_decode_from_slice::<BorrowEnum<'_>>(&encoded)
            .unwrap_or_else(|e| panic!("borrow_decode failed for {}: {:?}", label, e));
        assert_eq!(*variant, decoded, "borrow_decode mismatch for {}", label);
        assert_eq!(
            consumed,
            encoded.len(),
            "consumed bytes mismatch for {}",
            label
        );
    }

    // Zero-copy pointer check: the decoded &str must point into the encoded buffer.
    let original_str = "hello from zero copy";
    let v = BorrowEnum::Str(original_str);
    let encoded = encode_std(&v);
    let (decoded, _) =
        oxicode::borrow_decode_from_slice::<BorrowEnum<'_>>(&encoded).expect("borrow_decode");
    if let BorrowEnum::Str(s) = decoded {
        assert_eq!(s, original_str);
        // The pointer should reference memory inside `encoded` (not a copy)
        let enc_start = encoded.as_ptr() as usize;
        let enc_end = enc_start + encoded.len();
        let str_ptr = s.as_ptr() as usize;
        assert!(
            str_ptr >= enc_start && str_ptr < enc_end,
            "decoded &str should point into the encoded buffer (zero-copy)"
        );
    } else {
        panic!("expected BorrowEnum::Str, got {:?}", decoded);
    }
}
