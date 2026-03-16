//! Comprehensive roundtrip tests covering diverse type combinations in OxiCode.

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
use oxicode::{decode_from_slice, encode_to_vec, encoded_size};
use oxicode_derive::{Decode, Encode};
use std::collections::BTreeMap;

// ===== Test 1: Complex struct with all primitive types =====

#[derive(Debug, PartialEq, Encode, Decode)]
struct AllPrimitiveTypes {
    u8_val: u8,
    u16_val: u16,
    u32_val: u32,
    u64_val: u64,
    u128_val: u128,
    i8_val: i8,
    i16_val: i16,
    i32_val: i32,
    i64_val: i64,
    i128_val: i128,
    f32_val: f32,
    f64_val: f64,
    bool_val: bool,
    char_val: char,
}

#[test]
fn test_all_primitive_types_roundtrip() {
    let val = AllPrimitiveTypes {
        u8_val: 200u8,
        u16_val: 60000u16,
        u32_val: 3_000_000_000u32,
        u64_val: 9_000_000_000_000_000_000u64,
        u128_val: 340_282_366_920_938_463_463_374_607_431_768_211_455u128,
        i8_val: -100i8,
        i16_val: -30000i16,
        i32_val: -2_000_000_000i32,
        i64_val: -4_000_000_000_000_000_000i64,
        i128_val: -170_141_183_460_469_231_731_687_303_715_884_105_728i128,
        f32_val: std::f32::consts::PI,
        f64_val: std::f64::consts::PI,
        bool_val: true,
        char_val: '€',
    };
    let enc = encode_to_vec(&val).expect("encode AllPrimitiveTypes");
    let (dec, _): (AllPrimitiveTypes, _) =
        decode_from_slice(&enc).expect("decode AllPrimitiveTypes");
    assert_eq!(val, dec);
}

// ===== Test 2: Struct with String, Vec<u8>, Vec<String> fields =====

#[derive(Debug, PartialEq, Encode, Decode)]
struct StringAndVecFields {
    name: String,
    raw_bytes: Vec<u8>,
    tags: Vec<String>,
}

#[test]
fn test_string_and_vec_fields_roundtrip() {
    let val = StringAndVecFields {
        name: "OxiCode binary serialization".to_string(),
        raw_bytes: vec![0x00, 0xFF, 0x7F, 0x80, 0x01, 0xFE, 0x55, 0xAA],
        tags: vec![
            "fast".to_string(),
            "compact".to_string(),
            "pure-rust".to_string(),
            "no-unsafe".to_string(),
        ],
    };
    let enc = encode_to_vec(&val).expect("encode StringAndVecFields");
    let (dec, _): (StringAndVecFields, _) =
        decode_from_slice(&enc).expect("decode StringAndVecFields");
    assert_eq!(val, dec);
}

// ===== Test 3: Enum with 5 variants (no recursive/Box) =====

#[derive(Debug, PartialEq, Encode, Decode)]
enum MultiVariantEnum {
    Unit,
    Newtype(u64),
    Tuple(u32, u32),
    Struct { name: String, value: i32 },
    Tagged { code: u8, label: String },
}

#[test]
fn test_multi_variant_enum_roundtrip() {
    let variants: Vec<MultiVariantEnum> = vec![
        MultiVariantEnum::Unit,
        MultiVariantEnum::Newtype(9_876_543_210u64),
        MultiVariantEnum::Tuple(111_111u32, 222_222u32),
        MultiVariantEnum::Struct {
            name: "structured variant".to_string(),
            value: -42i32,
        },
        MultiVariantEnum::Tagged {
            code: 255u8,
            label: "tagged-label".to_string(),
        },
    ];
    for variant in &variants {
        let enc = encode_to_vec(variant).expect("encode MultiVariantEnum variant");
        let (dec, _): (MultiVariantEnum, _) =
            decode_from_slice(&enc).expect("decode MultiVariantEnum variant");
        assert_eq!(variant, &dec);
    }
}

// ===== Test 4: Deeply nested struct: outer { inner: { value: String, data: Vec<u32> } } =====

#[derive(Debug, PartialEq, Encode, Decode)]
struct DeepInner {
    value: String,
    data: Vec<u32>,
}

#[derive(Debug, PartialEq, Encode, Decode)]
struct DeepOuter {
    label: String,
    inner: DeepInner,
}

#[test]
fn test_deeply_nested_struct_roundtrip() {
    let val = DeepOuter {
        label: "outer-container".to_string(),
        inner: DeepInner {
            value: "inner-string-value".to_string(),
            data: vec![10u32, 20u32, 30u32, 100u32, 200u32, 300u32, 1000u32],
        },
    };
    let enc = encode_to_vec(&val).expect("encode DeepOuter");
    let (dec, _): (DeepOuter, _) = decode_from_slice(&enc).expect("decode DeepOuter");
    assert_eq!(val, dec);
}

// ===== Test 5: Vec<String> with 50 elements roundtrip =====

#[test]
fn test_vec_string_50_elements_roundtrip() {
    let strings: Vec<String> = (0..50)
        .map(|i| format!("element-{:03}-suffix-{}", i, i * i))
        .collect();
    let enc = encode_to_vec(&strings).expect("encode Vec<String> 50 elements");
    let (dec, _): (Vec<String>, _) =
        decode_from_slice(&enc).expect("decode Vec<String> 50 elements");
    assert_eq!(strings, dec);
}

// ===== Test 6: BTreeMap<String, Vec<u32>> with 10 entries =====

#[test]
fn test_btreemap_string_vec_u32_10_entries_roundtrip() {
    let mut map: BTreeMap<String, Vec<u32>> = BTreeMap::new();
    for i in 0..10usize {
        let key = format!("bucket-{:02}", i);
        let values: Vec<u32> = (0..((i as u32) + 3)).map(|j| j * (i as u32 + 1)).collect();
        map.insert(key, values);
    }
    let enc = encode_to_vec(&map).expect("encode BTreeMap<String, Vec<u32>>");
    let (dec, _): (BTreeMap<String, Vec<u32>>, _) =
        decode_from_slice(&enc).expect("decode BTreeMap<String, Vec<u32>>");
    assert_eq!(map, dec);
}

// ===== Test 7: Vec<BTreeMap<String, u32>> with 5 maps =====

#[test]
fn test_vec_of_btreemaps_5_elements_roundtrip() {
    let maps: Vec<BTreeMap<String, u32>> = (0..5usize)
        .map(|i| {
            let mut m: BTreeMap<String, u32> = BTreeMap::new();
            for j in 0..((i + 3) as u32) {
                m.insert(format!("key-{}-{}", i, j), j * (i as u32 + 1) * 100);
            }
            m
        })
        .collect();
    let enc = encode_to_vec(&maps).expect("encode Vec<BTreeMap<String, u32>>");
    let (dec, _): (Vec<BTreeMap<String, u32>>, _) =
        decode_from_slice(&enc).expect("decode Vec<BTreeMap<String, u32>>");
    assert_eq!(maps, dec);
}

// ===== Test 8: Complex tuple: (String, Vec<u8>, Option<u64>, bool, u32) =====

#[test]
fn test_complex_tuple_roundtrip() {
    let tup: (String, Vec<u8>, Option<u64>, bool, u32) = (
        "tuple-string-field".to_string(),
        vec![0xCA, 0xFE, 0xBA, 0xBE, 0xDE, 0xAD],
        Some(u64::MAX / 2),
        false,
        999_999u32,
    );
    let enc = encode_to_vec(&tup).expect("encode complex tuple");
    #[allow(clippy::type_complexity)]
    let (dec, _): ((String, Vec<u8>, Option<u64>, bool, u32), _) =
        decode_from_slice(&enc).expect("decode complex tuple");
    assert_eq!(tup, dec);
}

// ===== Test 9: Option<Vec<BTreeMap<String, u32>>> Some with 3 maps =====

#[test]
fn test_option_vec_btreemap_some_roundtrip() {
    let maps: Vec<BTreeMap<String, u32>> = (0..3usize)
        .map(|i| {
            let mut m: BTreeMap<String, u32> = BTreeMap::new();
            m.insert(format!("alpha-{}", i), (i as u32) * 111);
            m.insert(format!("beta-{}", i), (i as u32) * 222);
            m.insert(format!("gamma-{}", i), (i as u32) * 333);
            m
        })
        .collect();
    let val: Option<Vec<BTreeMap<String, u32>>> = Some(maps);
    let enc = encode_to_vec(&val).expect("encode Option<Vec<BTreeMap>> Some");
    let (dec, _): (Option<Vec<BTreeMap<String, u32>>>, _) =
        decode_from_slice(&enc).expect("decode Option<Vec<BTreeMap>> Some");
    assert_eq!(val, dec);
}

// ===== Test 10: Option<Vec<BTreeMap<String, u32>>> None =====

#[test]
fn test_option_vec_btreemap_none_roundtrip() {
    let val: Option<Vec<BTreeMap<String, u32>>> = None;
    let enc = encode_to_vec(&val).expect("encode Option<Vec<BTreeMap>> None");
    let (dec, _): (Option<Vec<BTreeMap<String, u32>>>, _) =
        decode_from_slice(&enc).expect("decode Option<Vec<BTreeMap>> None");
    assert_eq!(val, dec);
}

// ===== Test 11: Vec<Option<String>> with mix of Some/None =====

#[test]
fn test_vec_option_string_mixed_roundtrip() {
    let val: Vec<Option<String>> = vec![
        Some("first".to_string()),
        None,
        Some("third".to_string()),
        None,
        None,
        Some("sixth".to_string()),
        Some("seventh-with-unicode-こんにちは".to_string()),
        None,
        Some(String::new()),
        Some("last".to_string()),
    ];
    let enc = encode_to_vec(&val).expect("encode Vec<Option<String>>");
    let (dec, _): (Vec<Option<String>>, _) =
        decode_from_slice(&enc).expect("decode Vec<Option<String>>");
    assert_eq!(val, dec);
}

// ===== Test 12: BTreeMap<u32, String> with 100 entries =====

#[test]
fn test_btreemap_u32_string_100_entries_roundtrip() {
    let map: BTreeMap<u32, String> = (0..100u32)
        .map(|i| {
            (
                i * 7 + 3,
                format!("value-for-key-{:04}-extra-{}", i, i % 13),
            )
        })
        .collect();
    let enc = encode_to_vec(&map).expect("encode BTreeMap<u32, String> 100 entries");
    let (dec, _): (BTreeMap<u32, String>, _) =
        decode_from_slice(&enc).expect("decode BTreeMap<u32, String> 100 entries");
    assert_eq!(map, dec);
}

// ===== Test 13: Struct with Option fields - all Some =====

#[derive(Debug, PartialEq, Encode, Decode)]
struct OptionFields {
    maybe_name: Option<String>,
    maybe_count: Option<u64>,
    maybe_ratio: Option<f64>,
    maybe_flag: Option<bool>,
    maybe_bytes: Option<Vec<u8>>,
}

#[test]
fn test_struct_option_fields_all_some_roundtrip() {
    let val = OptionFields {
        maybe_name: Some("present-name".to_string()),
        maybe_count: Some(42_000_000u64),
        maybe_ratio: Some(std::f64::consts::E),
        maybe_flag: Some(true),
        maybe_bytes: Some(vec![0x11u8, 0x22u8, 0x33u8, 0x44u8]),
    };
    let enc = encode_to_vec(&val).expect("encode OptionFields all Some");
    let (dec, _): (OptionFields, _) =
        decode_from_slice(&enc).expect("decode OptionFields all Some");
    assert_eq!(val, dec);
}

// ===== Test 14: Struct with Option fields - all None =====

#[test]
fn test_struct_option_fields_all_none_roundtrip() {
    let val = OptionFields {
        maybe_name: None,
        maybe_count: None,
        maybe_ratio: None,
        maybe_flag: None,
        maybe_bytes: None,
    };
    let enc = encode_to_vec(&val).expect("encode OptionFields all None");
    let (dec, _): (OptionFields, _) =
        decode_from_slice(&enc).expect("decode OptionFields all None");
    assert_eq!(val, dec);
}

// ===== Test 15: Vec of derived structs roundtrip (5 elements) =====

#[derive(Debug, PartialEq, Encode, Decode)]
struct DerivedItem {
    id: u32,
    name: String,
    score: f64,
    active: bool,
    tags: Vec<String>,
}

#[test]
fn test_vec_of_derived_structs_roundtrip() {
    let items: Vec<DerivedItem> = (0..5u32)
        .map(|i| DerivedItem {
            id: i,
            name: format!("item-name-{}", i),
            score: std::f64::consts::PI * (i as f64 + 1.0),
            active: i % 2 == 0,
            tags: (0..3u32).map(|t| format!("tag-{}-{}", i, t)).collect(),
        })
        .collect();
    let enc = encode_to_vec(&items).expect("encode Vec<DerivedItem>");
    let (dec, _): (Vec<DerivedItem>, _) = decode_from_slice(&enc).expect("decode Vec<DerivedItem>");
    assert_eq!(items, dec);
}

// ===== Test 16: Enum with all 5 variants covered =====

#[derive(Debug, PartialEq, Encode, Decode)]
enum Status {
    Active,
    Inactive,
    Pending(u32),
    Error { code: u32, msg: String },
    Reserved,
}

#[test]
fn test_enum_all_variants_roundtrip() {
    let cases: Vec<Status> = vec![
        Status::Active,
        Status::Inactive,
        Status::Pending(404u32),
        Status::Error {
            code: 500u32,
            msg: "internal server error occurred during processing".to_string(),
        },
        Status::Reserved,
    ];
    for case in &cases {
        let enc = encode_to_vec(case).expect("encode Status variant");
        let (dec, _): (Status, _) = decode_from_slice(&enc).expect("decode Status variant");
        assert_eq!(case, &dec);
    }
}

// ===== Test 17: Struct with Vec<Struct> nested field =====

#[derive(Debug, PartialEq, Encode, Decode)]
struct NestedChild {
    index: u32,
    payload: Vec<u8>,
    label: String,
}

#[derive(Debug, PartialEq, Encode, Decode)]
struct NestedParent {
    id: u64,
    children: Vec<NestedChild>,
    description: String,
}

#[test]
fn test_struct_with_vec_of_structs_roundtrip() {
    let val = NestedParent {
        id: 0xDEAD_BEEF_CAFE_1234u64,
        children: vec![
            NestedChild {
                index: 0u32,
                payload: vec![0x01u8, 0x02u8, 0x03u8],
                label: "child-zero".to_string(),
            },
            NestedChild {
                index: 1u32,
                payload: vec![0xAAu8, 0xBBu8, 0xCCu8, 0xDDu8],
                label: "child-one".to_string(),
            },
            NestedChild {
                index: 2u32,
                payload: (0u8..16u8).collect(),
                label: "child-two-with-longer-label".to_string(),
            },
        ],
        description: "parent struct containing vector of child structs".to_string(),
    };
    let enc = encode_to_vec(&val).expect("encode NestedParent");
    let (dec, _): (NestedParent, _) = decode_from_slice(&enc).expect("decode NestedParent");
    assert_eq!(val, dec);
}

// ===== Test 18: encoded_size matches encode_to_vec.len() for complex struct =====

#[derive(Debug, PartialEq, Encode, Decode)]
struct ComplexSizeStruct {
    id: u64,
    names: Vec<String>,
    mapping: BTreeMap<String, u32>,
    flags: Vec<bool>,
    ratio: f64,
}

#[test]
fn test_encoded_size_matches_encoded_bytes_len() {
    let mut mapping: BTreeMap<String, u32> = BTreeMap::new();
    mapping.insert("alpha".to_string(), 100u32);
    mapping.insert("beta".to_string(), 200u32);
    mapping.insert("gamma".to_string(), 300u32);
    mapping.insert("delta".to_string(), 400u32);

    let val = ComplexSizeStruct {
        id: 123_456_789u64,
        names: vec![
            "name-one".to_string(),
            "name-two".to_string(),
            "name-three".to_string(),
        ],
        mapping,
        flags: vec![true, false, true, true, false],
        ratio: std::f64::consts::E,
    };

    let predicted_size = encoded_size(&val).expect("encoded_size for ComplexSizeStruct");
    let actual_bytes = encode_to_vec(&val).expect("encode ComplexSizeStruct");
    assert_eq!(
        predicted_size,
        actual_bytes.len(),
        "encoded_size prediction must match actual encoded byte length"
    );
}

// ===== Test 19: Re-encode after decode produces identical bytes =====

#[derive(Debug, PartialEq, Encode, Decode)]
struct ReEncodeStruct {
    version: u16,
    data: Vec<u8>,
    label: String,
    count: u64,
}

#[test]
fn test_reencode_after_decode_produces_identical_bytes() {
    let original = ReEncodeStruct {
        version: 2u16,
        data: (0u8..32u8).collect(),
        label: "re-encode-verification-test".to_string(),
        count: 1_234_567_890u64,
    };
    let first_enc = encode_to_vec(&original).expect("first encode");
    let (decoded, _): (ReEncodeStruct, _) =
        decode_from_slice(&first_enc).expect("decode for re-encode test");
    assert_eq!(original, decoded);

    let second_enc = encode_to_vec(&decoded).expect("second encode (re-encode)");
    assert_eq!(
        first_enc, second_enc,
        "re-encoding decoded value must produce byte-identical output"
    );
}

// ===== Test 20: Struct with [u8; 32] hash field roundtrip =====

#[derive(Debug, PartialEq, Encode, Decode)]
struct HashRecord {
    id: u64,
    hash: [u8; 32],
    description: String,
    verified: bool,
}

#[test]
fn test_struct_with_fixed_array_hash_roundtrip() {
    let val = HashRecord {
        id: 9_999_999_999u64,
        hash: [
            0x01, 0x23, 0x45, 0x67, 0x89, 0xAB, 0xCD, 0xEF, 0xFE, 0xDC, 0xBA, 0x98, 0x76, 0x54,
            0x32, 0x10, 0xDE, 0xAD, 0xBE, 0xEF, 0xCA, 0xFE, 0xBA, 0xBE, 0x00, 0x11, 0x22, 0x33,
            0x44, 0x55, 0x66, 0x77,
        ],
        description: "sha256-like hash field in fixed-size array".to_string(),
        verified: true,
    };
    let enc = encode_to_vec(&val).expect("encode HashRecord");
    let (dec, _): (HashRecord, _) = decode_from_slice(&enc).expect("decode HashRecord");
    assert_eq!(val, dec);
}

// ===== Test 21: Large data: Vec<u64> with 10000 elements =====

#[test]
fn test_large_vec_u64_10000_elements_roundtrip() {
    let data: Vec<u64> = (0..10000u64)
        .map(|i| {
            // Mix of small, medium and large values to exercise varint encoding thoroughly
            match i % 4 {
                0 => i,
                1 => i * 257,
                2 => i * 65537,
                _ => i * 4_294_967_311,
            }
        })
        .collect();
    let enc = encode_to_vec(&data).expect("encode large Vec<u64>");
    let (dec, _): (Vec<u64>, _) = decode_from_slice(&enc).expect("decode large Vec<u64>");
    assert_eq!(data, dec);
}

// ===== Test 22: Boundary integers in struct: u8::MAX, u16::MAX, u32::MAX, u64::MAX, i64::MIN, i64::MAX =====

#[derive(Debug, PartialEq, Encode, Decode)]
struct BoundaryIntegers {
    u8_max: u8,
    u16_max: u16,
    u32_max: u32,
    u64_max: u64,
    i8_min: i8,
    i8_max: i8,
    i16_min: i16,
    i16_max: i16,
    i32_min: i32,
    i32_max: i32,
    i64_min: i64,
    i64_max: i64,
    u8_min: u8,
    u16_min: u16,
    u32_min: u32,
    u64_min: u64,
}

#[test]
fn test_boundary_integers_roundtrip() {
    let val = BoundaryIntegers {
        u8_max: u8::MAX,
        u16_max: u16::MAX,
        u32_max: u32::MAX,
        u64_max: u64::MAX,
        i8_min: i8::MIN,
        i8_max: i8::MAX,
        i16_min: i16::MIN,
        i16_max: i16::MAX,
        i32_min: i32::MIN,
        i32_max: i32::MAX,
        i64_min: i64::MIN,
        i64_max: i64::MAX,
        u8_min: u8::MIN,
        u16_min: u16::MIN,
        u32_min: u32::MIN,
        u64_min: u64::MIN,
    };
    let enc = encode_to_vec(&val).expect("encode BoundaryIntegers");
    let (dec, _): (BoundaryIntegers, _) = decode_from_slice(&enc).expect("decode BoundaryIntegers");
    assert_eq!(val, dec);
}
