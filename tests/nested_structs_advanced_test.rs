//! Advanced tests for deeply nested struct encoding in OxiCode.
//! 22 tests covering nesting, attributes, generics, collections, and configs.

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
use oxicode::{config, decode_from_slice, encode_to_vec};
use oxicode::{decode_from_slice_with_config, encode_to_vec_with_config};
use std::collections::HashMap;

// ---------------------------------------------------------------------------
// Test 1: Struct with 1 field (u32) roundtrip
// ---------------------------------------------------------------------------

#[derive(oxicode::Encode, oxicode::Decode, PartialEq, Debug)]
struct SingleField {
    val: u32,
}

#[test]
fn test_single_field_struct_roundtrip() {
    let original = SingleField { val: 42 };
    let encoded = encode_to_vec(&original).expect("encode failed");
    let (decoded, _): (SingleField, _) = decode_from_slice(&encoded).expect("decode failed");
    assert_eq!(original, decoded);
}

// ---------------------------------------------------------------------------
// Test 2: Struct with 5 primitive fields roundtrip
// ---------------------------------------------------------------------------

#[derive(oxicode::Encode, oxicode::Decode, PartialEq, Debug)]
struct FivePrimitives {
    a: u8,
    b: u16,
    c: u32,
    d: u64,
    e: bool,
}

#[test]
fn test_five_primitive_fields_roundtrip() {
    let original = FivePrimitives {
        a: 255,
        b: 1000,
        c: 100_000,
        d: 1_000_000_000,
        e: true,
    };
    let encoded = encode_to_vec(&original).expect("encode failed");
    let (decoded, _): (FivePrimitives, _) = decode_from_slice(&encoded).expect("decode failed");
    assert_eq!(original, decoded);
}

// ---------------------------------------------------------------------------
// Test 3: Struct with string fields roundtrip
// ---------------------------------------------------------------------------

#[derive(oxicode::Encode, oxicode::Decode, PartialEq, Debug)]
struct StringFields {
    name: String,
    label: String,
    tag: String,
}

#[test]
fn test_string_fields_roundtrip() {
    let original = StringFields {
        name: "Alice".to_string(),
        label: "primary".to_string(),
        tag: "rust".to_string(),
    };
    let encoded = encode_to_vec(&original).expect("encode failed");
    let (decoded, _): (StringFields, _) = decode_from_slice(&encoded).expect("decode failed");
    assert_eq!(original, decoded);
}

// ---------------------------------------------------------------------------
// Test 4: Struct containing another struct roundtrip
// ---------------------------------------------------------------------------

#[derive(oxicode::Encode, oxicode::Decode, PartialEq, Debug)]
struct InnerT4 {
    x: i32,
}

#[derive(oxicode::Encode, oxicode::Decode, PartialEq, Debug)]
struct OuterT4 {
    inner: InnerT4,
    id: u32,
}

#[test]
fn test_struct_containing_struct_roundtrip() {
    let original = OuterT4 {
        inner: InnerT4 { x: -42 },
        id: 7,
    };
    let encoded = encode_to_vec(&original).expect("encode failed");
    let (decoded, _): (OuterT4, _) = decode_from_slice(&encoded).expect("decode failed");
    assert_eq!(original, decoded);
}

// ---------------------------------------------------------------------------
// Test 5: 3-level deep nesting roundtrip
// ---------------------------------------------------------------------------

#[derive(oxicode::Encode, oxicode::Decode, PartialEq, Debug)]
struct Level3T5 {
    val: u64,
}

#[derive(oxicode::Encode, oxicode::Decode, PartialEq, Debug)]
struct Level2T5 {
    inner: Level3T5,
    name: String,
}

#[derive(oxicode::Encode, oxicode::Decode, PartialEq, Debug)]
struct Level1T5 {
    inner: Level2T5,
    count: u32,
}

#[test]
fn test_three_level_deep_nesting_roundtrip() {
    let original = Level1T5 {
        count: 99,
        inner: Level2T5 {
            name: "deep".to_string(),
            inner: Level3T5 {
                val: 0xDEAD_BEEF_CAFE_BABE,
            },
        },
    };
    let encoded = encode_to_vec(&original).expect("encode failed");
    let (decoded, _): (Level1T5, _) = decode_from_slice(&encoded).expect("decode failed");
    assert_eq!(original, decoded);
}

// ---------------------------------------------------------------------------
// Test 6: Struct with Vec<SubStruct> field
// ---------------------------------------------------------------------------

#[derive(oxicode::Encode, oxicode::Decode, PartialEq, Debug)]
struct ItemT6 {
    id: u32,
    name: String,
}

#[derive(oxicode::Encode, oxicode::Decode, PartialEq, Debug)]
struct ItemListT6 {
    items: Vec<ItemT6>,
}

#[test]
fn test_struct_with_vec_of_sub_struct() {
    let original = ItemListT6 {
        items: vec![
            ItemT6 {
                id: 1,
                name: "alpha".to_string(),
            },
            ItemT6 {
                id: 2,
                name: "beta".to_string(),
            },
            ItemT6 {
                id: 3,
                name: "gamma".to_string(),
            },
        ],
    };
    let encoded = encode_to_vec(&original).expect("encode failed");
    let (decoded, _): (ItemListT6, _) = decode_from_slice(&encoded).expect("decode failed");
    assert_eq!(original, decoded);
}

// ---------------------------------------------------------------------------
// Test 7: Struct with Option<SubStruct> Some/None
// ---------------------------------------------------------------------------

#[derive(oxicode::Encode, oxicode::Decode, PartialEq, Debug)]
struct InnerT7 {
    score: u32,
}

#[derive(oxicode::Encode, oxicode::Decode, PartialEq, Debug)]
struct ContainerT7 {
    maybe: Option<InnerT7>,
}

#[test]
fn test_struct_with_option_sub_struct_some_and_none() {
    let with_some = ContainerT7 {
        maybe: Some(InnerT7 { score: 100 }),
    };
    let encoded = encode_to_vec(&with_some).expect("encode Some failed");
    let (decoded, _): (ContainerT7, _) = decode_from_slice(&encoded).expect("decode Some failed");
    assert_eq!(with_some, decoded);

    let with_none = ContainerT7 { maybe: None };
    let encoded2 = encode_to_vec(&with_none).expect("encode None failed");
    let (decoded2, _): (ContainerT7, _) = decode_from_slice(&encoded2).expect("decode None failed");
    assert_eq!(with_none, decoded2);
}

// ---------------------------------------------------------------------------
// Test 8: Struct with HashMap<String, SubStruct>
// ---------------------------------------------------------------------------

#[derive(oxicode::Encode, oxicode::Decode, PartialEq, Debug)]
struct EntryT8 {
    score: u32,
}

#[derive(oxicode::Encode, oxicode::Decode, PartialEq, Debug)]
struct MapHolderT8 {
    map: HashMap<String, EntryT8>,
}

#[test]
fn test_struct_with_hashmap_string_to_sub_struct() {
    let mut map = HashMap::new();
    map.insert("alice".to_string(), EntryT8 { score: 90 });
    map.insert("bob".to_string(), EntryT8 { score: 75 });
    let original = MapHolderT8 { map };
    let encoded = encode_to_vec(&original).expect("encode failed");
    let (decoded, _): (MapHolderT8, _) = decode_from_slice(&encoded).expect("decode failed");
    assert_eq!(original, decoded);
}

// ---------------------------------------------------------------------------
// Test 9: Struct with multiple Vec fields
// ---------------------------------------------------------------------------

#[derive(oxicode::Encode, oxicode::Decode, PartialEq, Debug)]
struct MultiVecT9 {
    ids: Vec<u32>,
    names: Vec<String>,
    flags: Vec<bool>,
}

#[test]
fn test_struct_with_multiple_vec_fields() {
    let original = MultiVecT9 {
        ids: vec![1, 2, 3, 4, 5],
        names: vec!["x".to_string(), "y".to_string()],
        flags: vec![true, false, true],
    };
    let encoded = encode_to_vec(&original).expect("encode failed");
    let (decoded, _): (MultiVecT9, _) = decode_from_slice(&encoded).expect("decode failed");
    assert_eq!(original, decoded);
}

// ---------------------------------------------------------------------------
// Test 10: Struct with bool flags and counts
// ---------------------------------------------------------------------------

#[derive(oxicode::Encode, oxicode::Decode, PartialEq, Debug)]
struct BoolFlagsT10 {
    is_active: bool,
    is_admin: bool,
    is_verified: bool,
    login_count: u32,
    post_count: u32,
}

#[test]
fn test_struct_with_bool_flags_and_counts() {
    let original = BoolFlagsT10 {
        is_active: true,
        is_admin: false,
        is_verified: true,
        login_count: 42,
        post_count: 7,
    };
    let encoded = encode_to_vec(&original).expect("encode failed");
    let (decoded, _): (BoolFlagsT10, _) = decode_from_slice(&encoded).expect("decode failed");
    assert_eq!(original, decoded);
}

// ---------------------------------------------------------------------------
// Test 11: Tree-like struct with Box<u64>
// ---------------------------------------------------------------------------

#[derive(oxicode::Encode, oxicode::Decode, PartialEq, Debug)]
struct NodeT11 {
    value: u32,
    child: Box<u64>,
}

#[test]
fn test_tree_like_struct_with_box() {
    let original = NodeT11 {
        value: 123,
        child: Box::new(0xCAFEBABE),
    };
    let encoded = encode_to_vec(&original).expect("encode failed");
    let (decoded, _): (NodeT11, _) = decode_from_slice(&encoded).expect("decode failed");
    assert_eq!(original, decoded);
}

// ---------------------------------------------------------------------------
// Test 12: Struct with all primitive types
// ---------------------------------------------------------------------------

#[derive(oxicode::Encode, oxicode::Decode, PartialEq, Debug)]
struct AllPrimitivesT12 {
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
fn test_struct_with_all_primitive_types() {
    let original = AllPrimitivesT12 {
        a_u8: 0xFF,
        b_u16: 0xBEEF,
        c_u32: 0xDEAD_BEEF,
        d_u64: 0x0102_0304_0506_0708,
        e_i8: -100,
        f_i16: -1000,
        g_i32: -100_000,
        h_i64: -1_000_000_000,
        i_f32: 1.5_f32,
        j_f64: 1234.5678_f64,
        k_bool: true,
        l_char: 'Z',
    };
    let encoded = encode_to_vec(&original).expect("encode failed");
    let (decoded, _): (AllPrimitivesT12, _) = decode_from_slice(&encoded).expect("decode failed");
    assert_eq!(decoded.a_u8, original.a_u8);
    assert_eq!(decoded.b_u16, original.b_u16);
    assert_eq!(decoded.c_u32, original.c_u32);
    assert_eq!(decoded.d_u64, original.d_u64);
    assert_eq!(decoded.e_i8, original.e_i8);
    assert_eq!(decoded.f_i16, original.f_i16);
    assert_eq!(decoded.g_i32, original.g_i32);
    assert_eq!(decoded.h_i64, original.h_i64);
    assert_eq!(decoded.i_f32.to_bits(), original.i_f32.to_bits());
    assert_eq!(decoded.j_f64.to_bits(), original.j_f64.to_bits());
    assert_eq!(decoded.k_bool, original.k_bool);
    assert_eq!(decoded.l_char, original.l_char);
}

// ---------------------------------------------------------------------------
// Test 13: Struct with mixed Option<T> fields (some Some, some None)
// ---------------------------------------------------------------------------

#[derive(oxicode::Encode, oxicode::Decode, PartialEq, Debug)]
struct MixedOptionsT13 {
    name: Option<String>,
    count: Option<u32>,
    active: Option<bool>,
    score: Option<f64>,
}

#[test]
fn test_struct_with_mixed_option_fields() {
    let original = MixedOptionsT13 {
        name: Some("test".to_string()),
        count: None,
        active: Some(true),
        score: None,
    };
    let encoded = encode_to_vec(&original).expect("encode failed");
    let (decoded, _): (MixedOptionsT13, _) = decode_from_slice(&encoded).expect("decode failed");
    assert_eq!(original, decoded);
}

// ---------------------------------------------------------------------------
// Test 14: Encode size verification
// ---------------------------------------------------------------------------

#[derive(oxicode::Encode, oxicode::Decode, PartialEq, Debug)]
struct SizeCheckT14 {
    id: u32,
    name: String,
    count: u64,
}

#[test]
fn test_encode_size_verification() {
    let original = SizeCheckT14 {
        id: 1,
        name: "hello".to_string(),
        count: 42,
    };
    let encoded = encode_to_vec(&original).expect("encode failed");
    // Encoded bytes should be non-empty and within a reasonable bound
    assert!(!encoded.is_empty(), "encoded length must be > 0");
    assert!(encoded.len() < 1024, "encoded length should be reasonable");
    // Verify roundtrip
    let (decoded, consumed): (SizeCheckT14, _) =
        decode_from_slice(&encoded).expect("decode failed");
    assert_eq!(original, decoded);
    assert_eq!(consumed, encoded.len());
}

// ---------------------------------------------------------------------------
// Test 15: Struct with fixed int encoding
// ---------------------------------------------------------------------------

#[derive(oxicode::Encode, oxicode::Decode, PartialEq, Debug)]
struct FixedIntT15 {
    a: u32,
    b: u64,
}

#[test]
fn test_struct_with_fixed_int_encoding() {
    let cfg = config::standard().with_fixed_int_encoding();
    let original = FixedIntT15 { a: 1000, b: 99_999 };
    let encoded = encode_to_vec_with_config(&original, cfg).expect("encode with fixed int failed");
    // u32 is 4 bytes + u64 is 8 bytes = 12 bytes with fixed encoding
    assert_eq!(encoded.len(), 12, "fixed int: u32(4) + u64(8) = 12 bytes");
    let (decoded, _): (FixedIntT15, _) =
        decode_from_slice_with_config(&encoded, cfg).expect("decode with fixed int failed");
    assert_eq!(original, decoded);
}

// ---------------------------------------------------------------------------
// Test 16: Struct with big endian config
// ---------------------------------------------------------------------------

#[derive(oxicode::Encode, oxicode::Decode, PartialEq, Debug)]
struct BigEndianT16 {
    value: u32,
    label: String,
}

#[test]
fn test_struct_with_big_endian_config() {
    let cfg = config::standard().with_big_endian();
    let original = BigEndianT16 {
        value: 0xDEAD_BEEF,
        label: "be".to_string(),
    };
    let encoded = encode_to_vec_with_config(&original, cfg).expect("encode big endian failed");
    let (decoded, _): (BigEndianT16, _) =
        decode_from_slice_with_config(&encoded, cfg).expect("decode big endian failed");
    assert_eq!(original, decoded);
}

// ---------------------------------------------------------------------------
// Test 17: Struct with bytes attribute (#[oxicode(bytes)] on Vec<u8> field)
// ---------------------------------------------------------------------------

#[derive(oxicode::Encode, oxicode::Decode, PartialEq, Debug)]
struct BlobHolderT17 {
    #[oxicode(bytes)]
    data: Vec<u8>,
    id: u32,
}

#[test]
fn test_struct_with_bytes_attribute() {
    let original = BlobHolderT17 {
        data: vec![0xDE, 0xAD, 0xBE, 0xEF, 0xCA, 0xFE],
        id: 1,
    };
    let encoded = encode_to_vec(&original).expect("encode failed");
    let (decoded, _): (BlobHolderT17, _) = decode_from_slice(&encoded).expect("decode failed");
    assert_eq!(original, decoded);
}

// ---------------------------------------------------------------------------
// Test 18: Struct with skip attribute (#[oxicode(skip)] on a field)
// ---------------------------------------------------------------------------

#[derive(oxicode::Encode, oxicode::Decode, PartialEq, Debug)]
struct WithSkipT18 {
    id: u32,
    #[oxicode(skip)]
    cached: u64,
    name: String,
}

#[test]
fn test_struct_with_skip_attribute() {
    let original = WithSkipT18 {
        id: 77,
        cached: 0xDEAD_BEEF_CAFE_BABE,
        name: "skipme".to_string(),
    };
    let encoded = encode_to_vec(&original).expect("encode failed");
    let (decoded, _): (WithSkipT18, _) = decode_from_slice(&encoded).expect("decode failed");
    assert_eq!(decoded.id, 77);
    assert_eq!(decoded.cached, 0u64, "skipped field must be Default (0)");
    assert_eq!(decoded.name, "skipme");
}

// ---------------------------------------------------------------------------
// Test 19: Struct with rename attribute (#[oxicode(rename = "x")] on field)
// ---------------------------------------------------------------------------

#[derive(oxicode::Encode, oxicode::Decode, PartialEq, Debug)]
struct WithRenameT19 {
    #[oxicode(rename = "firstName")]
    first_name: String,
    age: u32,
}

#[test]
fn test_struct_with_rename_attribute() {
    let original = WithRenameT19 {
        first_name: "Carol".to_string(),
        age: 30,
    };
    let encoded = encode_to_vec(&original).expect("encode failed");
    let (decoded, _): (WithRenameT19, _) = decode_from_slice(&encoded).expect("decode failed");
    assert_eq!(original, decoded);
}

// ---------------------------------------------------------------------------
// Test 20: Generic struct `Wrapper<T> { value: T, tag: u32 }` roundtrip
// ---------------------------------------------------------------------------

#[derive(oxicode::Encode, oxicode::Decode, PartialEq, Debug)]
struct WrapperT20<T> {
    value: T,
    tag: u32,
}

#[test]
fn test_generic_wrapper_struct_roundtrip() {
    let str_wrapper = WrapperT20 {
        value: "hello".to_string(),
        tag: 1,
    };
    let encoded = encode_to_vec(&str_wrapper).expect("encode String wrapper failed");
    let (decoded, _): (WrapperT20<String>, _) =
        decode_from_slice(&encoded).expect("decode String wrapper failed");
    assert_eq!(str_wrapper, decoded);

    let u64_wrapper = WrapperT20 {
        value: 0xCAFE_BABEu64,
        tag: 2,
    };
    let encoded2 = encode_to_vec(&u64_wrapper).expect("encode u64 wrapper failed");
    let (decoded2, _): (WrapperT20<u64>, _) =
        decode_from_slice(&encoded2).expect("decode u64 wrapper failed");
    assert_eq!(u64_wrapper, decoded2);
}

// ---------------------------------------------------------------------------
// Test 21: Vec<MyStruct> with 5 elements roundtrip
// ---------------------------------------------------------------------------

#[derive(oxicode::Encode, oxicode::Decode, PartialEq, Debug)]
struct MyStructT21 {
    id: u32,
    name: String,
}

#[test]
fn test_vec_of_my_struct_five_elements() {
    let original: Vec<MyStructT21> = vec![
        MyStructT21 {
            id: 1,
            name: "one".to_string(),
        },
        MyStructT21 {
            id: 2,
            name: "two".to_string(),
        },
        MyStructT21 {
            id: 3,
            name: "three".to_string(),
        },
        MyStructT21 {
            id: 4,
            name: "four".to_string(),
        },
        MyStructT21 {
            id: 5,
            name: "five".to_string(),
        },
    ];
    let encoded = encode_to_vec(&original).expect("encode failed");
    let (decoded, _): (Vec<MyStructT21>, _) = decode_from_slice(&encoded).expect("decode failed");
    assert_eq!(original, decoded);
}

// ---------------------------------------------------------------------------
// Test 22: Struct containing enums and structs mixed
// ---------------------------------------------------------------------------

#[derive(oxicode::Encode, oxicode::Decode, PartialEq, Debug)]
enum StatusT22 {
    Active,
    Inactive,
    Custom(u32),
}

#[derive(oxicode::Encode, oxicode::Decode, PartialEq, Debug)]
struct InfoStructT22 {
    label: String,
}

#[derive(oxicode::Encode, oxicode::Decode, PartialEq, Debug)]
struct RecordT22 {
    id: u32,
    status: StatusT22,
    info: InfoStructT22,
}

#[test]
fn test_struct_with_enum_and_struct_mixed() {
    let cases = vec![
        RecordT22 {
            id: 1,
            status: StatusT22::Active,
            info: InfoStructT22 {
                label: "active record".to_string(),
            },
        },
        RecordT22 {
            id: 2,
            status: StatusT22::Inactive,
            info: InfoStructT22 {
                label: "inactive record".to_string(),
            },
        },
        RecordT22 {
            id: 3,
            status: StatusT22::Custom(42),
            info: InfoStructT22 {
                label: "custom record".to_string(),
            },
        },
    ];
    for original in &cases {
        let encoded = encode_to_vec(original).expect("encode failed");
        let (decoded, _): (RecordT22, _) = decode_from_slice(&encoded).expect("decode failed");
        assert_eq!(original, &decoded);
    }
}
