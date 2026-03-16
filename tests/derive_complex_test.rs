//! Tests for complex derive patterns — 22 scenarios covering deeply nested structs,
//! large enums, generics with where clauses, PhantomData, wide-integer types, arrays,
//! nested modules, BorrowDecode with lifetimes, and more.

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
use oxicode::{decode_from_slice, encode_to_vec, BorrowDecode, Decode, Encode};
use std::marker::PhantomData;

// ---------------------------------------------------------------------------
// Test 1: Struct with all field attributes combined: skip, rename, encode_with/decode_with
// ---------------------------------------------------------------------------

mod field_transforms {
    use oxicode::{
        de::{Decode, Decoder},
        enc::{Encode, Encoder},
        Error,
    };

    #[allow(clippy::ptr_arg)]
    pub fn encode_rev<E: Encoder>(s: &String, encoder: &mut E) -> Result<(), Error> {
        let rev: String = s.chars().rev().collect();
        rev.encode(encoder)
    }

    pub fn decode_rev<D: Decoder<Context = ()>>(decoder: &mut D) -> Result<String, Error> {
        let s = String::decode(decoder)?;
        Ok(s.chars().rev().collect())
    }
}

#[derive(Debug, PartialEq, Encode, Decode)]
struct AllFieldAttrs {
    id: u32,
    #[oxicode(skip)]
    transient: u64,
    #[oxicode(rename = "displayName")]
    display_name: String,
    #[oxicode(
        encode_with = "field_transforms::encode_rev",
        decode_with = "field_transforms::decode_rev"
    )]
    token: String,
}

#[test]
fn test_all_field_attrs_combined() {
    let original = AllFieldAttrs {
        id: 42,
        transient: 9999,
        display_name: "Alice".to_string(),
        token: "hello".to_string(),
    };
    let enc = encode_to_vec(&original).expect("encode");
    let (dec, _): (AllFieldAttrs, _) = decode_from_slice(&enc).expect("decode");
    assert_eq!(dec.id, 42);
    assert_eq!(dec.transient, 0u64); // skipped → Default::default()
    assert_eq!(dec.display_name, "Alice");
    // "hello" reversed → "olleh" on wire, reversed back → "hello"
    assert_eq!(dec.token, "hello");
}

// ---------------------------------------------------------------------------
// Test 2: Deeply nested struct (5 levels deep) — roundtrip
// ---------------------------------------------------------------------------

#[derive(Debug, PartialEq, Encode, Decode)]
struct Level5 {
    value: u8,
}

#[derive(Debug, PartialEq, Encode, Decode)]
struct Level4 {
    inner: Level5,
    tag: u16,
}

#[derive(Debug, PartialEq, Encode, Decode)]
struct Level3 {
    inner: Level4,
    name: String,
}

#[derive(Debug, PartialEq, Encode, Decode)]
struct Level2 {
    inner: Level3,
    count: u32,
}

#[derive(Debug, PartialEq, Encode, Decode)]
struct Level1 {
    inner: Level2,
    active: bool,
}

#[test]
fn test_deeply_nested_5_levels_roundtrip() {
    let original = Level1 {
        active: true,
        inner: Level2 {
            count: 100,
            inner: Level3 {
                name: "deep".to_string(),
                inner: Level4 {
                    tag: 7,
                    inner: Level5 { value: 255 },
                },
            },
        },
    };
    let enc = encode_to_vec(&original).expect("encode");
    let (dec, _): (Level1, _) = decode_from_slice(&enc).expect("decode");
    assert_eq!(original, dec);
}

// ---------------------------------------------------------------------------
// Test 3: Enum with 15+ variants — roundtrip for each variant
// ---------------------------------------------------------------------------

#[derive(Debug, PartialEq, Encode, Decode)]
enum Wide15 {
    V0,
    V1(u8),
    V2(u16),
    V3(u32),
    V4(u64),
    V5(i8),
    V6(i16),
    V7(i32),
    V8(i64),
    V9(bool),
    V10(String),
    V11 { x: u32, y: u32 },
    V12(Vec<u8>),
    V13(Option<u32>),
    V14(u8, u8, u8),
    V15,
}

#[test]
fn test_enum_15_plus_variants_roundtrip() {
    let cases: Vec<Wide15> = vec![
        Wide15::V0,
        Wide15::V1(1),
        Wide15::V2(1000),
        Wide15::V3(70_000),
        Wide15::V4(5_000_000_000),
        Wide15::V5(-100),
        Wide15::V6(-1000),
        Wide15::V7(-70_000),
        Wide15::V8(-5_000_000_000),
        Wide15::V9(true),
        Wide15::V10("variant".to_string()),
        Wide15::V11 { x: 10, y: 20 },
        Wide15::V12(vec![1, 2, 3]),
        Wide15::V13(Some(42)),
        Wide15::V14(1, 2, 3),
        Wide15::V15,
    ];
    for case in &cases {
        let enc = encode_to_vec(case).expect("encode");
        let (dec, _): (Wide15, _) = decode_from_slice(&enc).expect("decode");
        assert_eq!(case, &dec);
    }
}

// ---------------------------------------------------------------------------
// Test 4: Generic struct with where clause + derive
// ---------------------------------------------------------------------------

#[derive(Debug, PartialEq, Encode, Decode)]
struct PairWhere<A, B>
where
    A: Encode + Decode + Clone + core::fmt::Debug + PartialEq,
    B: Encode + Decode + Clone + core::fmt::Debug + PartialEq,
{
    first: A,
    second: B,
    label: String,
}

#[test]
fn test_generic_where_clause_roundtrip() {
    let original: PairWhere<u32, Vec<u8>> = PairWhere {
        first: 123,
        second: vec![10, 20, 30],
        label: "pair".to_string(),
    };
    let enc = encode_to_vec(&original).expect("encode");
    let (dec, _): (PairWhere<u32, Vec<u8>>, _) = decode_from_slice(&enc).expect("decode");
    assert_eq!(original, dec);
}

// ---------------------------------------------------------------------------
// Test 5: Struct with PhantomData<T> field — roundtrip
// ---------------------------------------------------------------------------

#[derive(Debug, PartialEq, Encode, Decode)]
struct WithPhantom<T> {
    value: u64,
    label: String,
    _marker: PhantomData<T>,
}

#[test]
fn test_phantom_data_field_roundtrip() {
    let original: WithPhantom<String> = WithPhantom {
        value: 99,
        label: "phantom".to_string(),
        _marker: PhantomData,
    };
    let enc = encode_to_vec(&original).expect("encode");
    let (dec, _): (WithPhantom<String>, _) = decode_from_slice(&enc).expect("decode");
    assert_eq!(original, dec);
}

// ---------------------------------------------------------------------------
// Test 6: Tuple struct with 8 fields — roundtrip
// ---------------------------------------------------------------------------

#[derive(Debug, PartialEq, Encode, Decode)]
struct Tuple8(u8, u16, u32, u64, i8, i16, i32, i64);

#[test]
fn test_tuple_struct_8_fields_roundtrip() {
    let original = Tuple8(1, 2, 3, 4, -1, -2, -3, -4);
    let enc = encode_to_vec(&original).expect("encode");
    let (dec, _): (Tuple8, _) = decode_from_slice(&enc).expect("decode");
    assert_eq!(original, dec);
}

// ---------------------------------------------------------------------------
// Test 7: Enum with all variant types (unit, tuple, struct) in same enum
// ---------------------------------------------------------------------------

#[derive(Debug, PartialEq, Encode, Decode)]
enum MixedVariants {
    Unit,
    TupleOne(u32),
    TupleTwo(u32, String),
    StructNamed { x: i32, y: i32, label: String },
    EmptyTuple(),
}

#[test]
fn test_enum_all_variant_types_roundtrip() {
    let cases = vec![
        MixedVariants::Unit,
        MixedVariants::TupleOne(42),
        MixedVariants::TupleTwo(7, "hello".to_string()),
        MixedVariants::StructNamed {
            x: -5,
            y: 10,
            label: "point".to_string(),
        },
        MixedVariants::EmptyTuple(),
    ];
    for case in &cases {
        let enc = encode_to_vec(case).expect("encode");
        let (dec, _): (MixedVariants, _) = decode_from_slice(&enc).expect("decode");
        assert_eq!(case, &dec);
    }
}

// ---------------------------------------------------------------------------
// Test 8: Struct containing Box<T> (concrete) — skip dyn traits, use Box<String> etc.
// ---------------------------------------------------------------------------

#[derive(Debug, PartialEq, Encode, Decode)]
struct BoxedFields {
    name: String,
    data: Vec<u8>,
    value: Box<u64>,
}

#[test]
fn test_boxed_concrete_fields_roundtrip() {
    let original = BoxedFields {
        name: "boxed".to_string(),
        data: vec![1, 2, 3, 4, 5],
        value: Box::new(0xDEAD_BEEF),
    };
    let enc = encode_to_vec(&original).expect("encode");
    let (dec, _): (BoxedFields, _) = decode_from_slice(&enc).expect("decode");
    assert_eq!(original, dec);
}

// ---------------------------------------------------------------------------
// Test 9: Recursive-like structure using Vec for depth — roundtrip
// ---------------------------------------------------------------------------

#[derive(Debug, PartialEq, Encode, Decode)]
struct TreeNode {
    value: i32,
    children: Vec<i32>,
    depth: u8,
}

#[derive(Debug, PartialEq, Encode, Decode)]
struct Forest {
    nodes: Vec<TreeNode>,
}

#[test]
fn test_recursive_like_vec_depth_roundtrip() {
    let original = Forest {
        nodes: vec![
            TreeNode {
                value: 1,
                children: vec![2, 3],
                depth: 0,
            },
            TreeNode {
                value: 2,
                children: vec![4, 5],
                depth: 1,
            },
            TreeNode {
                value: 3,
                children: vec![6],
                depth: 1,
            },
        ],
    };
    let enc = encode_to_vec(&original).expect("encode");
    let (dec, _): (Forest, _) = decode_from_slice(&enc).expect("decode");
    assert_eq!(original, dec);
}

// ---------------------------------------------------------------------------
// Test 10: Struct with all primitive types (u8..u64, i8..i64, f32, f64, bool, char)
// ---------------------------------------------------------------------------

#[derive(Debug, PartialEq, Encode, Decode)]
struct AllPrimitives {
    a_u8: u8,
    b_u16: u16,
    c_u32: u32,
    d_u64: u64,
    e_i8: i8,
    f_i16: i16,
    g_i32: i32,
    h_i64: i64,
    i_f32: f32,
    j_f64: f64,
    k_bool: bool,
    l_char: char,
}

#[test]
fn test_all_primitive_types_roundtrip() {
    let original = AllPrimitives {
        a_u8: 0xFF,
        b_u16: 0xBEEF,
        c_u32: 0xDEAD_BEEF,
        d_u64: 0x0102_0304_0506_0708,
        e_i8: -100,
        f_i16: -1000,
        g_i32: -100_000,
        h_i64: -1_000_000_000,
        i_f32: 1.0_f32,
        j_f64: 2.0_f64,
        k_bool: true,
        l_char: 'Z',
    };
    let enc = encode_to_vec(&original).expect("encode");
    let (dec, _): (AllPrimitives, _) = decode_from_slice(&enc).expect("decode");
    assert_eq!(dec.a_u8, original.a_u8);
    assert_eq!(dec.b_u16, original.b_u16);
    assert_eq!(dec.c_u32, original.c_u32);
    assert_eq!(dec.d_u64, original.d_u64);
    assert_eq!(dec.e_i8, original.e_i8);
    assert_eq!(dec.f_i16, original.f_i16);
    assert_eq!(dec.g_i32, original.g_i32);
    assert_eq!(dec.h_i64, original.h_i64);
    // exact bit comparison for floats — no approximation
    assert_eq!(dec.i_f32.to_bits(), original.i_f32.to_bits());
    assert_eq!(dec.j_f64.to_bits(), original.j_f64.to_bits());
    assert_eq!(dec.k_bool, original.k_bool);
    assert_eq!(dec.l_char, original.l_char);
}

// ---------------------------------------------------------------------------
// Test 11: Struct with String, Vec<u8>, Option<String>, Vec<String>
// ---------------------------------------------------------------------------

#[derive(Debug, PartialEq, Encode, Decode)]
struct CommonCollections {
    name: String,
    data: Vec<u8>,
    maybe: Option<String>,
    tags: Vec<String>,
}

#[test]
fn test_common_collections_roundtrip() {
    let cases = vec![
        CommonCollections {
            name: "full".to_string(),
            data: vec![0xCA, 0xFE, 0xBA, 0xBE],
            maybe: Some("present".to_string()),
            tags: vec!["alpha".to_string(), "beta".to_string()],
        },
        CommonCollections {
            name: String::new(),
            data: vec![],
            maybe: None,
            tags: vec![],
        },
    ];
    for original in &cases {
        let enc = encode_to_vec(original).expect("encode");
        let (dec, _): (CommonCollections, _) = decode_from_slice(&enc).expect("decode");
        assert_eq!(original, &dec);
    }
}

// ---------------------------------------------------------------------------
// Test 12: Generic enum with 3 variants each carrying a T
// ---------------------------------------------------------------------------

#[derive(Debug, PartialEq, Encode, Decode)]
enum TriVariant<T> {
    Primary(T),
    Secondary(T),
    Tertiary(T),
}

#[test]
fn test_generic_enum_three_variants_roundtrip() {
    let str_cases: Vec<TriVariant<String>> = vec![
        TriVariant::Primary("one".to_string()),
        TriVariant::Secondary("two".to_string()),
        TriVariant::Tertiary("three".to_string()),
    ];
    for case in &str_cases {
        let enc = encode_to_vec(case).expect("encode");
        let (dec, _): (TriVariant<String>, _) = decode_from_slice(&enc).expect("decode");
        assert_eq!(case, &dec);
    }

    let num_cases: Vec<TriVariant<u32>> = vec![
        TriVariant::Primary(1),
        TriVariant::Secondary(2),
        TriVariant::Tertiary(3),
    ];
    for case in &num_cases {
        let enc = encode_to_vec(case).expect("encode");
        let (dec, _): (TriVariant<u32>, _) = decode_from_slice(&enc).expect("decode");
        assert_eq!(case, &dec);
    }
}

// ---------------------------------------------------------------------------
// Test 13: Struct with all fixed-width integers (u8..u128, i8..i128)
// ---------------------------------------------------------------------------

#[derive(Debug, PartialEq, Encode, Decode)]
struct AllWidthIntegers {
    u8_field: u8,
    u16_field: u16,
    u32_field: u32,
    u64_field: u64,
    u128_field: u128,
    i8_field: i8,
    i16_field: i16,
    i32_field: i32,
    i64_field: i64,
    i128_field: i128,
}

#[test]
fn test_all_width_integers_roundtrip() {
    let original = AllWidthIntegers {
        u8_field: u8::MAX,
        u16_field: u16::MAX,
        u32_field: u32::MAX,
        u64_field: u64::MAX,
        u128_field: u128::MAX,
        i8_field: i8::MIN,
        i16_field: i16::MIN,
        i32_field: i32::MIN,
        i64_field: i64::MIN,
        i128_field: i128::MIN,
    };
    let enc = encode_to_vec(&original).expect("encode");
    let (dec, _): (AllWidthIntegers, _) = decode_from_slice(&enc).expect("decode");
    assert_eq!(original, dec);
}

// ---------------------------------------------------------------------------
// Test 14: Struct with arrays: [u8; 4], [u32; 8], [bool; 16]
// ---------------------------------------------------------------------------

#[derive(Debug, PartialEq, Encode, Decode)]
struct WithArrays {
    bytes4: [u8; 4],
    words8: [u32; 8],
    flags16: [bool; 16],
}

#[test]
fn test_fixed_arrays_roundtrip() {
    let original = WithArrays {
        bytes4: [0xDE, 0xAD, 0xBE, 0xEF],
        words8: [1, 2, 3, 4, 5, 6, 7, 8],
        flags16: [
            true, false, true, true, false, false, true, false, true, true, false, true, false,
            true, false, true,
        ],
    };
    let enc = encode_to_vec(&original).expect("encode");
    let (dec, _): (WithArrays, _) = decode_from_slice(&enc).expect("decode");
    assert_eq!(original, dec);
}

// ---------------------------------------------------------------------------
// Test 15: Enum variant with 4+ tuple fields
// ---------------------------------------------------------------------------

#[derive(Debug, PartialEq, Encode, Decode)]
enum BigTupleEnum {
    Quad(u32, u32, u32, u32),
    Quint(u8, u16, u32, u64, i32),
    Single(String),
}

#[test]
fn test_enum_variant_4_plus_tuple_fields_roundtrip() {
    let cases = vec![
        BigTupleEnum::Quad(10, 20, 30, 40),
        BigTupleEnum::Quint(1, 2, 3, 4, -5),
        BigTupleEnum::Single("solo".to_string()),
    ];
    for case in &cases {
        let enc = encode_to_vec(case).expect("encode");
        let (dec, _): (BigTupleEnum, _) = decode_from_slice(&enc).expect("decode");
        assert_eq!(case, &dec);
    }
}

// ---------------------------------------------------------------------------
// Test 16: Derive on unit struct within generic context
// ---------------------------------------------------------------------------

#[derive(Debug, PartialEq, Encode, Decode)]
struct TaggedUnit<T> {
    _marker: PhantomData<T>,
    label: String,
}

#[test]
fn test_unit_struct_in_generic_context() {
    let original: TaggedUnit<u64> = TaggedUnit {
        _marker: PhantomData,
        label: "generic-unit".to_string(),
    };
    let enc = encode_to_vec(&original).expect("encode");
    let (dec, _): (TaggedUnit<u64>, _) = decode_from_slice(&enc).expect("decode");
    assert_eq!(original, dec);
}

// ---------------------------------------------------------------------------
// Test 17: Multiple derives on same type in nested modules
// ---------------------------------------------------------------------------

mod outer_mod {
    use oxicode::{decode_from_slice, encode_to_vec, Decode, Encode};

    pub mod inner_mod {
        use oxicode::{decode_from_slice, encode_to_vec, Decode, Encode};

        #[derive(Debug, PartialEq, Encode, Decode)]
        pub struct InnerMsg {
            pub payload: u32,
            pub text: String,
        }

        pub fn roundtrip(msg: &InnerMsg) -> InnerMsg {
            let enc = encode_to_vec(msg).expect("encode inner");
            let (dec, _): (InnerMsg, _) = decode_from_slice(&enc).expect("decode inner");
            dec
        }
    }

    #[derive(Debug, PartialEq, Encode, Decode)]
    pub struct OuterMsg {
        pub id: u64,
        pub inner: inner_mod::InnerMsg,
    }

    pub fn roundtrip(msg: &OuterMsg) -> OuterMsg {
        let enc = encode_to_vec(msg).expect("encode outer");
        let (dec, _): (OuterMsg, _) = decode_from_slice(&enc).expect("decode outer");
        dec
    }
}

#[test]
fn test_multiple_derives_nested_modules() {
    let inner = outer_mod::inner_mod::InnerMsg {
        payload: 77,
        text: "inner".to_string(),
    };
    let inner_rt = outer_mod::inner_mod::roundtrip(&inner);
    assert_eq!(inner, inner_rt);

    let outer = outer_mod::OuterMsg {
        id: 1001,
        inner: outer_mod::inner_mod::InnerMsg {
            payload: 99,
            text: "nested".to_string(),
        },
    };
    let outer_rt = outer_mod::roundtrip(&outer);
    assert_eq!(outer, outer_rt);
}

// ---------------------------------------------------------------------------
// Test 18: Struct with very long field name (50+ chars)
// ---------------------------------------------------------------------------

#[derive(Debug, PartialEq, Encode, Decode)]
struct LongFieldNames {
    this_is_a_very_long_field_name_that_exceeds_fifty_characters_total: u32,
    another_extremely_verbose_field_name_that_is_also_very_long_indeed: String,
}

#[test]
fn test_long_field_names_roundtrip() {
    let original = LongFieldNames {
        this_is_a_very_long_field_name_that_exceeds_fifty_characters_total: 12345,
        another_extremely_verbose_field_name_that_is_also_very_long_indeed: "verbose".to_string(),
    };
    let enc = encode_to_vec(&original).expect("encode");
    let (dec, _): (LongFieldNames, _) = decode_from_slice(&enc).expect("decode");
    assert_eq!(original, dec);
}

// ---------------------------------------------------------------------------
// Test 19: Struct with single-char field names (a, b, c …)
// ---------------------------------------------------------------------------

#[derive(Debug, PartialEq, Encode, Decode)]
struct SingleCharFields {
    a: u8,
    b: u16,
    c: u32,
    d: u64,
    e: bool,
    f: char,
    g: String,
}

#[test]
fn test_single_char_field_names_roundtrip() {
    let original = SingleCharFields {
        a: 1,
        b: 2,
        c: 3,
        d: 4,
        e: true,
        f: 'x',
        g: "g_val".to_string(),
    };
    let enc = encode_to_vec(&original).expect("encode");
    let (dec, _): (SingleCharFields, _) = decode_from_slice(&enc).expect("decode");
    assert_eq!(original, dec);
}

// ---------------------------------------------------------------------------
// Test 20: Enum with variants named using common type keywords
// ---------------------------------------------------------------------------

#[allow(clippy::enum_variant_names)]
#[derive(Debug, PartialEq, Encode, Decode)]
enum KeywordVariants {
    Value(u32),
    Error(String),
    Option(bool),
    Result { code: i32, msg: String },
    Ok,
    None,
}

#[test]
fn test_enum_keyword_named_variants_roundtrip() {
    let cases = vec![
        KeywordVariants::Value(100),
        KeywordVariants::Error("oops".to_string()),
        KeywordVariants::Option(false),
        KeywordVariants::Result {
            code: -1,
            msg: "fail".to_string(),
        },
        KeywordVariants::Ok,
        KeywordVariants::None,
    ];
    for case in &cases {
        let enc = encode_to_vec(case).expect("encode");
        let (dec, _): (KeywordVariants, _) = decode_from_slice(&enc).expect("decode");
        assert_eq!(case, &dec);
    }
}

// ---------------------------------------------------------------------------
// Test 21: Struct implementing Encode+Decode used as field in another struct with derive
// ---------------------------------------------------------------------------

#[derive(Debug, PartialEq, Encode, Decode)]
struct Inner21 {
    x: i32,
    y: i32,
}

#[derive(Debug, PartialEq, Encode, Decode)]
struct Outer21 {
    origin: Inner21,
    scale: f32,
    label: String,
}

#[derive(Debug, PartialEq, Encode, Decode)]
struct Container21 {
    items: Vec<Outer21>,
    count: usize,
}

#[test]
fn test_nested_derived_struct_as_field_roundtrip() {
    let original = Container21 {
        count: 2,
        items: vec![
            Outer21 {
                origin: Inner21 { x: 0, y: 0 },
                scale: 1.0_f32,
                label: "first".to_string(),
            },
            Outer21 {
                origin: Inner21 { x: -5, y: 10 },
                scale: 2.0_f32,
                label: "second".to_string(),
            },
        ],
    };
    let enc = encode_to_vec(&original).expect("encode");
    let (dec, _): (Container21, _) = decode_from_slice(&enc).expect("decode");
    assert_eq!(original.count, dec.count);
    assert_eq!(original.items.len(), dec.items.len());
    for (a, b) in original.items.iter().zip(dec.items.iter()) {
        assert_eq!(a.origin, b.origin);
        assert_eq!(a.scale.to_bits(), b.scale.to_bits());
        assert_eq!(a.label, b.label);
    }
}

// ---------------------------------------------------------------------------
// Test 22: BorrowDecode derived struct with lifetime parameter
// ---------------------------------------------------------------------------

#[derive(Debug, PartialEq, Encode, BorrowDecode)]
struct BorrowWithLifetime<'a> {
    raw: &'a [u8],
    text: &'a str,
    version: u32,
}

#[test]
fn test_borrow_decode_derived_with_lifetime() {
    let original = BorrowWithLifetime {
        raw: b"binary-data",
        text: "zero-copy-text",
        version: 3,
    };
    let enc = encode_to_vec(&original).expect("encode");
    let (dec, consumed): (BorrowWithLifetime<'_>, _) =
        oxicode::borrow_decode_from_slice(&enc).expect("borrow decode");
    assert_eq!(dec.raw, b"binary-data");
    assert_eq!(dec.text, "zero-copy-text");
    assert_eq!(dec.version, 3);
    assert_eq!(consumed, enc.len());
}
