//! Advanced derive macro tests for OxiCode — complex scenarios (set 2)

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

// ---------------------------------------------------------------------------
// Shared top-level derived types used across multiple tests
// ---------------------------------------------------------------------------

#[derive(Clone, Debug, PartialEq, Encode, Decode)]
struct ManyFields {
    a: u8,
    b: u16,
    c: u32,
    d: u64,
    e: i32,
    f: f32,
    g: bool,
    h: String,
}

#[derive(Clone, Debug, PartialEq, Encode, Decode)]
enum EightVariants {
    Unit1,
    Unit2,
    Unit3,
    NewtypeU32(u32),
    NewtypeStr(String),
    Tuple2(u32, String),
    Struct2 { x: u32, y: u32 },
    MultiTuple(u8, u16, u32),
}

#[derive(Clone, Debug, PartialEq, Encode, Decode)]
struct BoxedVec {
    #[allow(clippy::vec_box)]
    items: Vec<Box<u32>>,
}

#[derive(Clone, Debug, PartialEq, Encode, Decode)]
struct OptionVecString {
    data: Option<Vec<String>>,
}

#[derive(Clone, Debug, PartialEq, Encode, Decode)]
enum WithStructVariant {
    Point { x: u32, y: u32 },
    Empty,
}

#[derive(Clone, Debug, PartialEq, Encode, Decode)]
enum WithTupleVariant {
    Pair(u32, String),
    Empty,
}

#[derive(Clone, Debug, PartialEq, Encode, Decode)]
struct Inner {
    value: u64,
    tag: String,
}

#[derive(Clone, Debug, PartialEq, Encode, Decode)]
struct Outer {
    items: Vec<Inner>,
    count: usize,
}

#[derive(Clone, Debug, PartialEq, Encode, Decode)]
struct BoolFields {
    flag_a: bool,
    flag_b: bool,
    flag_c: bool,
}

#[derive(Clone, Debug, PartialEq, Encode, Decode)]
struct AllIntegers {
    a: u8,
    b: u16,
    c: u32,
    d: u64,
    e: i8,
    f: i16,
    g: i32,
    h: i64,
}

#[derive(Clone, Debug, PartialEq, Encode, Decode)]
struct ComplexForConfig {
    name: String,
    values: Vec<u32>,
    flag: bool,
}

#[derive(Clone, Debug, PartialEq, Encode, Decode)]
struct Leaf {
    id: u32,
    label: String,
}

#[derive(Clone, Debug, PartialEq, Encode, Decode)]
struct Branch {
    leaves: Vec<Leaf>,
    name: String,
}

#[derive(Clone, Debug, PartialEq, Encode, Decode)]
struct StringAndBytes {
    text: String,
    raw: Vec<u8>,
}

#[derive(Clone, Debug, PartialEq, Encode, Decode)]
enum Discriminated {
    Unit,
    WithData(u64),
}

#[allow(dead_code)]
#[derive(Clone, Debug, Default, PartialEq, Encode, Decode)]
struct DefaultableStruct {
    count: u32,
    name: String,
    active: bool,
}

// ---------------------------------------------------------------------------
// Test 1: Struct with 8 fields of different types — roundtrip
// ---------------------------------------------------------------------------

#[test]
fn test_many_fields_roundtrip() {
    let original = ManyFields {
        a: 255,
        b: 65535,
        c: 123_456,
        d: 9_999_999_999,
        e: -42,
        f: std::f32::consts::PI,
        g: true,
        h: "hello world".to_string(),
    };
    let encoded = encode_to_vec(&original).expect("encode ManyFields");
    let (decoded, _): (ManyFields, usize) = decode_from_slice(&encoded).expect("decode ManyFields");
    assert_eq!(original, decoded);
}

// ---------------------------------------------------------------------------
// Test 2: Enum with 8 variants — roundtrip each representative variant
// ---------------------------------------------------------------------------

#[test]
fn test_eight_variants_roundtrip() {
    let variants = vec![
        EightVariants::Unit1,
        EightVariants::Unit2,
        EightVariants::Unit3,
        EightVariants::NewtypeU32(42),
        EightVariants::NewtypeStr("oxicode".to_string()),
        EightVariants::Tuple2(7, "pair".to_string()),
        EightVariants::Struct2 { x: 10, y: 20 },
        EightVariants::MultiTuple(1, 2, 3),
    ];
    for variant in &variants {
        let encoded = encode_to_vec(variant).expect("encode EightVariants");
        let (decoded, _): (EightVariants, usize) =
            decode_from_slice(&encoded).expect("decode EightVariants");
        assert_eq!(variant, &decoded);
    }
}

// ---------------------------------------------------------------------------
// Test 3: Struct with Vec<Box<u32>> field — roundtrip
// ---------------------------------------------------------------------------

#[test]
fn test_vec_box_u32_roundtrip() {
    let original = BoxedVec {
        items: vec![Box::new(1u32), Box::new(2u32), Box::new(u32::MAX)],
    };
    let encoded = encode_to_vec(&original).expect("encode BoxedVec");
    let (decoded, _): (BoxedVec, usize) = decode_from_slice(&encoded).expect("decode BoxedVec");
    assert_eq!(original, decoded);
}

// ---------------------------------------------------------------------------
// Test 4: Struct with Option<Vec<String>> — Some case
// ---------------------------------------------------------------------------

#[test]
fn test_option_vec_string_some_roundtrip() {
    let original = OptionVecString {
        data: Some(vec![
            "alpha".to_string(),
            "beta".to_string(),
            "gamma".to_string(),
        ]),
    };
    let encoded = encode_to_vec(&original).expect("encode OptionVecString Some");
    let (decoded, _): (OptionVecString, usize) =
        decode_from_slice(&encoded).expect("decode OptionVecString Some");
    assert_eq!(original, decoded);
}

// ---------------------------------------------------------------------------
// Test 5: Struct with Option<Vec<String>> — None case
// ---------------------------------------------------------------------------

#[test]
fn test_option_vec_string_none_roundtrip() {
    let original = OptionVecString { data: None };
    let encoded = encode_to_vec(&original).expect("encode OptionVecString None");
    let (decoded, _): (OptionVecString, usize) =
        decode_from_slice(&encoded).expect("decode OptionVecString None");
    assert_eq!(original, decoded);
}

// ---------------------------------------------------------------------------
// Test 6: Enum with struct variant { x: u32, y: u32 } — roundtrip
// ---------------------------------------------------------------------------

#[test]
fn test_enum_struct_variant_roundtrip() {
    let original = WithStructVariant::Point { x: 100, y: 200 };
    let encoded = encode_to_vec(&original).expect("encode WithStructVariant");
    let (decoded, _): (WithStructVariant, usize) =
        decode_from_slice(&encoded).expect("decode WithStructVariant");
    assert_eq!(original, decoded);
}

// ---------------------------------------------------------------------------
// Test 7: Enum with tuple variant (u32, String) — roundtrip
// ---------------------------------------------------------------------------

#[test]
fn test_enum_tuple_variant_roundtrip() {
    let original = WithTupleVariant::Pair(99, "tuple-variant".to_string());
    let encoded = encode_to_vec(&original).expect("encode WithTupleVariant");
    let (decoded, _): (WithTupleVariant, usize) =
        decode_from_slice(&encoded).expect("decode WithTupleVariant");
    assert_eq!(original, decoded);
}

// ---------------------------------------------------------------------------
// Test 8: Deeply nested derive — struct containing Vec of another derived struct
// ---------------------------------------------------------------------------

#[test]
fn test_deeply_nested_vec_of_derived_structs() {
    let original = Outer {
        items: vec![
            Inner {
                value: 1,
                tag: "first".to_string(),
            },
            Inner {
                value: 2,
                tag: "second".to_string(),
            },
            Inner {
                value: 1_000_000,
                tag: "third".to_string(),
            },
        ],
        count: 3,
    };
    let encoded = encode_to_vec(&original).expect("encode Outer");
    let (decoded, _): (Outer, usize) = decode_from_slice(&encoded).expect("decode Outer");
    assert_eq!(original, decoded);
}

// ---------------------------------------------------------------------------
// Test 9: Derived struct with bool fields — roundtrip
// ---------------------------------------------------------------------------

#[test]
fn test_bool_fields_roundtrip() {
    let cases = [
        BoolFields {
            flag_a: false,
            flag_b: false,
            flag_c: false,
        },
        BoolFields {
            flag_a: true,
            flag_b: false,
            flag_c: true,
        },
        BoolFields {
            flag_a: true,
            flag_b: true,
            flag_c: true,
        },
    ];
    for original in &cases {
        let encoded = encode_to_vec(original).expect("encode BoolFields");
        let (decoded, _): (BoolFields, usize) =
            decode_from_slice(&encoded).expect("decode BoolFields");
        assert_eq!(original, &decoded);
    }
}

// ---------------------------------------------------------------------------
// Test 10: Derived struct with all integer types — roundtrip
// ---------------------------------------------------------------------------

#[test]
fn test_all_integer_types_roundtrip() {
    let original = AllIntegers {
        a: u8::MAX,
        b: u16::MAX,
        c: u32::MAX,
        d: u64::MAX,
        e: i8::MIN,
        f: i16::MIN,
        g: i32::MIN,
        h: i64::MIN,
    };
    let encoded = encode_to_vec(&original).expect("encode AllIntegers");
    let (decoded, _): (AllIntegers, usize) =
        decode_from_slice(&encoded).expect("decode AllIntegers");
    assert_eq!(original, decoded);
}

// ---------------------------------------------------------------------------
// Test 11: Fixed-int config with complex struct
// ---------------------------------------------------------------------------

#[test]
fn test_fixed_int_config_complex_struct() {
    let original = ComplexForConfig {
        name: "fixed-int".to_string(),
        values: vec![10, 20, 30],
        flag: false,
    };
    let cfg = config::legacy(); // fixed-int, little-endian
    let encoded = encode_to_vec_with_config(&original, cfg).expect("encode with fixed-int config");
    let (decoded, _): (ComplexForConfig, usize) =
        decode_from_slice_with_config(&encoded, cfg).expect("decode with fixed-int config");
    assert_eq!(original, decoded);
}

// ---------------------------------------------------------------------------
// Test 12: Big-endian config with complex struct
// ---------------------------------------------------------------------------

#[test]
fn test_big_endian_config_complex_struct() {
    let original = ComplexForConfig {
        name: "big-endian".to_string(),
        values: vec![0xDEAD, 0xBEEF],
        flag: true,
    };
    let cfg = config::standard().with_big_endian();
    let encoded = encode_to_vec_with_config(&original, cfg).expect("encode with big-endian config");
    let (decoded, _): (ComplexForConfig, usize) =
        decode_from_slice_with_config(&encoded, cfg).expect("decode with big-endian config");
    assert_eq!(original, decoded);
}

// ---------------------------------------------------------------------------
// Test 13: Clone derived struct and verify encode/decode consistency
// ---------------------------------------------------------------------------

#[test]
fn test_clone_and_roundtrip_consistency() {
    let original = ManyFields {
        a: 10,
        b: 200,
        c: 30_000,
        d: 4_000_000,
        e: -1,
        f: 1.0,
        g: false,
        h: "clone-test".to_string(),
    };
    let cloned = original.clone();
    assert_eq!(original, cloned);

    let encoded_orig = encode_to_vec(&original).expect("encode original");
    let encoded_clone = encode_to_vec(&cloned).expect("encode clone");
    assert_eq!(encoded_orig, encoded_clone);

    let (decoded, _): (ManyFields, usize) =
        decode_from_slice(&encoded_orig).expect("decode after clone test");
    assert_eq!(original, decoded);
}

// ---------------------------------------------------------------------------
// Test 14: Vec of complex derived structs — roundtrip
// ---------------------------------------------------------------------------

#[test]
fn test_vec_of_complex_derived_structs_roundtrip() {
    let original: Vec<Inner> = (0..5u64)
        .map(|i| Inner {
            value: i * 100,
            tag: format!("item-{i}"),
        })
        .collect();
    let encoded = encode_to_vec(&original).expect("encode Vec<Inner>");
    let (decoded, _): (Vec<Inner>, usize) = decode_from_slice(&encoded).expect("decode Vec<Inner>");
    assert_eq!(original, decoded);
}

// ---------------------------------------------------------------------------
// Test 15: Option<derived_struct> — Some roundtrip
// ---------------------------------------------------------------------------

#[test]
fn test_option_derived_struct_some_roundtrip() {
    let original: Option<Inner> = Some(Inner {
        value: 42,
        tag: "optional".to_string(),
    });
    let encoded = encode_to_vec(&original).expect("encode Option<Inner> Some");
    let (decoded, _): (Option<Inner>, usize) =
        decode_from_slice(&encoded).expect("decode Option<Inner> Some");
    assert_eq!(original, decoded);
}

// ---------------------------------------------------------------------------
// Test 16: Option<derived_struct> — None roundtrip
// ---------------------------------------------------------------------------

#[test]
fn test_option_derived_struct_none_roundtrip() {
    let original: Option<Inner> = None;
    let encoded = encode_to_vec(&original).expect("encode Option<Inner> None");
    let (decoded, _): (Option<Inner>, usize) =
        decode_from_slice(&encoded).expect("decode Option<Inner> None");
    assert_eq!(original, decoded);
}

// ---------------------------------------------------------------------------
// Test 17: Two different structs encoded independently, decoded correctly
// ---------------------------------------------------------------------------

#[test]
fn test_two_structs_encoded_independently() {
    let struct_a = BoolFields {
        flag_a: true,
        flag_b: false,
        flag_c: true,
    };
    let struct_b = AllIntegers {
        a: 1,
        b: 2,
        c: 3,
        d: 4,
        e: -1,
        f: -2,
        g: -3,
        h: -4,
    };

    let encoded_a = encode_to_vec(&struct_a).expect("encode struct_a");
    let encoded_b = encode_to_vec(&struct_b).expect("encode struct_b");

    // Buffers must not be equal — they encode different types
    assert_ne!(encoded_a, encoded_b);

    let (decoded_a, _): (BoolFields, usize) =
        decode_from_slice(&encoded_a).expect("decode struct_a");
    let (decoded_b, _): (AllIntegers, usize) =
        decode_from_slice(&encoded_b).expect("decode struct_b");

    assert_eq!(struct_a, decoded_a);
    assert_eq!(struct_b, decoded_b);
}

// ---------------------------------------------------------------------------
// Test 18: Struct inside Vec inside struct — roundtrip
// ---------------------------------------------------------------------------

#[test]
fn test_struct_inside_vec_inside_struct_roundtrip() {
    let original = Branch {
        leaves: vec![
            Leaf {
                id: 1,
                label: "oak".to_string(),
            },
            Leaf {
                id: 2,
                label: "maple".to_string(),
            },
            Leaf {
                id: 3,
                label: "pine".to_string(),
            },
        ],
        name: "forest".to_string(),
    };
    let encoded = encode_to_vec(&original).expect("encode Branch");
    let (decoded, _): (Branch, usize) = decode_from_slice(&encoded).expect("decode Branch");
    assert_eq!(original, decoded);
}

// ---------------------------------------------------------------------------
// Test 19: consumed == encoded.len() for complex struct
// ---------------------------------------------------------------------------

#[test]
fn test_consumed_equals_encoded_len() {
    let original = Outer {
        items: vec![
            Inner {
                value: 7,
                tag: "a".to_string(),
            },
            Inner {
                value: 8,
                tag: "b".to_string(),
            },
        ],
        count: 2,
    };
    let encoded = encode_to_vec(&original).expect("encode for consumed check");
    let (_decoded, consumed): (Outer, usize) =
        decode_from_slice(&encoded).expect("decode for consumed check");
    assert_eq!(
        consumed,
        encoded.len(),
        "consumed bytes must equal encoded length"
    );
}

// ---------------------------------------------------------------------------
// Test 20: Struct with String and Vec<u8> fields — roundtrip
// ---------------------------------------------------------------------------

#[test]
fn test_string_and_bytes_roundtrip() {
    let original = StringAndBytes {
        text: "binary + text".to_string(),
        raw: vec![0x00, 0xFF, 0xAB, 0xCD, 0x12, 0x34],
    };
    let encoded = encode_to_vec(&original).expect("encode StringAndBytes");
    let (decoded, _): (StringAndBytes, usize) =
        decode_from_slice(&encoded).expect("decode StringAndBytes");
    assert_eq!(original, decoded);
}

// ---------------------------------------------------------------------------
// Test 21: Enum discriminant bytes differ for unit vs newtype variant
// ---------------------------------------------------------------------------

#[test]
fn test_enum_discriminant_differs_unit_vs_newtype() {
    let unit_val = Discriminated::Unit;
    let newtype_val = Discriminated::WithData(999);

    let encoded_unit = encode_to_vec(&unit_val).expect("encode Discriminated::Unit");
    let encoded_newtype = encode_to_vec(&newtype_val).expect("encode Discriminated::WithData");

    // The first byte(s) represent the discriminant; the two encodings must not be identical
    assert_ne!(
        encoded_unit, encoded_newtype,
        "unit and newtype variants must produce different byte sequences"
    );

    // Also verify each decodes back correctly
    let (decoded_unit, _): (Discriminated, usize) =
        decode_from_slice(&encoded_unit).expect("decode Discriminated::Unit");
    let (decoded_newtype, _): (Discriminated, usize) =
        decode_from_slice(&encoded_newtype).expect("decode Discriminated::WithData");

    assert_eq!(unit_val, decoded_unit);
    assert_eq!(newtype_val, decoded_newtype);
}

// ---------------------------------------------------------------------------
// Test 22: Default-derived struct — zero-value roundtrip
// ---------------------------------------------------------------------------

#[test]
fn test_default_struct_zero_value_roundtrip() {
    let original = DefaultableStruct::default();
    assert_eq!(original.count, 0);
    assert_eq!(original.name, "");
    assert!(!original.active);

    let encoded = encode_to_vec(&original).expect("encode DefaultableStruct");
    let (decoded, consumed): (DefaultableStruct, usize) =
        decode_from_slice(&encoded).expect("decode DefaultableStruct");

    assert_eq!(original, decoded);
    assert_eq!(consumed, encoded.len());
}
