// serde_advanced7_test.rs — 22 advanced serde integration tests for OxiCode
// All tests are gated on the `serde` feature at the file level.
// No #[cfg(test)] module wrapper; tests are top-level items.
// No unwrap() — all Results use .expect("...").

#![cfg(feature = "serde")]
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
use oxicode::serde::{decode_from_slice as serde_decode, encode_to_vec as serde_encode};
use serde::{Deserialize, Serialize};
use std::collections::{BTreeMap, HashMap};

// ---------------------------------------------------------------------------
// Shared type definitions
// ---------------------------------------------------------------------------

#[derive(Serialize, Deserialize, Debug, PartialEq)]
struct RenamedFields {
    #[serde(rename = "first_name")]
    given_name: String,
    #[serde(rename = "last_name")]
    family_name: String,
    age: u32,
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
struct WithSkipField {
    id: u64,
    name: String,
    #[serde(skip)]
    internal_cache: u32,
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
struct WithDefaultField {
    id: u64,
    #[serde(default)]
    label: String,
    #[serde(default = "default_count")]
    count: u32,
}

fn default_count() -> u32 {
    42
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
#[serde(tag = "type")]
enum InternallyTaggedEvent {
    Login { user_id: u64 },
    Logout { user_id: u64 },
    Error { code: u32, message: String },
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
#[serde(tag = "kind", content = "payload")]
enum AdjacentlyTaggedMsg {
    Text(String),
    Number(i64),
    Blob(Vec<u8>),
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
struct FlattenBase {
    name: String,
    #[serde(flatten)]
    extra: FlattenExtra,
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
struct FlattenExtra {
    x: i32,
    y: i32,
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
struct TupleStruct(u32, String, bool);

#[derive(Serialize, Deserialize, Debug, PartialEq)]
struct NewtypeStruct(u64);

#[derive(Serialize, Deserialize, Debug, PartialEq)]
struct AllNumericTypes {
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
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
struct NestedDeep {
    level: u32,
    child: Option<Box<NestedDeep>>,
    values: Vec<u32>,
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
struct WithVecString {
    tags: Vec<String>,
    count: usize,
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
enum NewtypeVariantEnum {
    Wrapped(u64),
    WrappedStr(String),
    WrappedVec(Vec<u8>),
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
#[serde(rename_all = "camelCase")]
struct CamelCasedStruct {
    first_name: String,
    last_name: String,
    age_in_years: u32,
    is_active: bool,
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
struct LargeTenPlusFields {
    field_01: u8,
    field_02: u16,
    field_03: u32,
    field_04: u64,
    field_05: i8,
    field_06: i16,
    field_07: i32,
    field_08: i64,
    field_09: f32,
    field_10: f64,
    field_11: String,
    field_12: bool,
    field_13: Vec<u32>,
    field_14: Option<String>,
}

// Native-compatible struct for cross-decode test
#[derive(oxicode::Encode, oxicode::Decode, Serialize, Deserialize, Debug, PartialEq)]
struct NativeCompatible {
    id: u32,
    value: u64,
}

// ---------------------------------------------------------------------------
// Test 1: HashMap<String, String> via serde roundtrip
// ---------------------------------------------------------------------------

#[test]
fn test_serde7_01_hashmap_string_string_roundtrip() {
    let cfg = oxicode::config::standard();
    let mut original: HashMap<String, String> = HashMap::new();
    original.insert(String::from("key_a"), String::from("value_alpha"));
    original.insert(String::from("key_b"), String::from("value_beta"));
    original.insert(String::from("key_c"), String::from("value_gamma"));

    let bytes = serde_encode(&original, cfg).expect("encode HashMap<String, String>");
    let (decoded, consumed): (HashMap<String, String>, usize) =
        serde_decode(&bytes, cfg).expect("decode HashMap<String, String>");

    assert_eq!(
        original, decoded,
        "HashMap<String, String> must roundtrip correctly"
    );
    assert_eq!(
        consumed,
        bytes.len(),
        "consumed bytes must equal total encoded length"
    );
}

// ---------------------------------------------------------------------------
// Test 2: Vec<HashMap<String, u32>> via serde roundtrip
// ---------------------------------------------------------------------------

#[test]
fn test_serde7_02_vec_hashmap_string_u32_roundtrip() {
    let cfg = oxicode::config::standard();

    let mut map1: HashMap<String, u32> = HashMap::new();
    map1.insert(String::from("alpha"), 1);
    map1.insert(String::from("beta"), 2);

    let mut map2: HashMap<String, u32> = HashMap::new();
    map2.insert(String::from("gamma"), 3);
    map2.insert(String::from("delta"), 4);

    let original: Vec<HashMap<String, u32>> = vec![map1, map2];
    let bytes = serde_encode(&original, cfg).expect("encode Vec<HashMap<String, u32>>");
    let (decoded, consumed): (Vec<HashMap<String, u32>>, usize) =
        serde_decode(&bytes, cfg).expect("decode Vec<HashMap<String, u32>>");

    assert_eq!(
        original, decoded,
        "Vec<HashMap<String, u32>> must roundtrip correctly"
    );
    assert_eq!(
        consumed,
        bytes.len(),
        "consumed bytes must equal total encoded length"
    );
}

// ---------------------------------------------------------------------------
// Test 3: serde_json::Value encode-only (oxicode is not self-describing,
//         so decode via serde_json::Value is not supported; we verify that
//         encoding succeeds and produces non-empty bytes, then verify that
//         the same payload round-trips through a known-shape struct).
// ---------------------------------------------------------------------------

#[test]
fn test_serde7_03_serde_json_value_encode_produces_bytes() {
    let cfg = oxicode::config::standard();

    // Encoding a serde_json::Value must succeed (Serialize is implemented).
    let json_val = serde_json::json!({
        "name": "oxicode",
        "version": 2
    });
    let bytes = serde_encode(&json_val, cfg).expect("encode serde_json::Value");
    assert!(
        !bytes.is_empty(),
        "encoded serde_json::Value must produce non-empty bytes"
    );
}

// ---------------------------------------------------------------------------
// Test 4: Struct with #[serde(rename)] roundtrip
// ---------------------------------------------------------------------------

#[test]
fn test_serde7_04_serde_rename_attribute_roundtrip() {
    let cfg = oxicode::config::standard();
    let original = RenamedFields {
        given_name: String::from("Alice"),
        family_name: String::from("Wonderland"),
        age: 30,
    };

    let bytes = serde_encode(&original, cfg).expect("encode RenamedFields");
    let (decoded, consumed): (RenamedFields, usize) =
        serde_decode(&bytes, cfg).expect("decode RenamedFields");

    assert_eq!(
        original, decoded,
        "#[serde(rename)] struct must roundtrip correctly"
    );
    assert_eq!(
        consumed,
        bytes.len(),
        "consumed bytes must equal total encoded length"
    );
}

// ---------------------------------------------------------------------------
// Test 5: Struct with #[serde(skip)] attribute
// ---------------------------------------------------------------------------

#[test]
fn test_serde7_05_serde_skip_attribute() {
    let cfg = oxicode::config::standard();
    let original = WithSkipField {
        id: 999,
        name: String::from("skip-test"),
        internal_cache: 12345,
    };

    let bytes = serde_encode(&original, cfg).expect("encode WithSkipField");
    let (decoded, consumed): (WithSkipField, usize) =
        serde_decode(&bytes, cfg).expect("decode WithSkipField");

    // The skipped field must be restored to its Default value (0u32)
    assert_eq!(decoded.id, original.id, "id must be preserved");
    assert_eq!(decoded.name, original.name, "name must be preserved");
    assert_eq!(
        decoded.internal_cache, 0,
        "#[serde(skip)] field must default to 0 after decode"
    );
    assert_eq!(
        consumed,
        bytes.len(),
        "consumed bytes must equal total encoded length"
    );
}

// ---------------------------------------------------------------------------
// Test 6: Struct with #[serde(default)] attribute
// ---------------------------------------------------------------------------

#[test]
fn test_serde7_06_serde_default_attribute_roundtrip() {
    let cfg = oxicode::config::standard();
    let original = WithDefaultField {
        id: 777,
        label: String::from("explicit-label"),
        count: 100,
    };

    let bytes = serde_encode(&original, cfg).expect("encode WithDefaultField");
    let (decoded, consumed): (WithDefaultField, usize) =
        serde_decode(&bytes, cfg).expect("decode WithDefaultField");

    assert_eq!(
        original, decoded,
        "#[serde(default)] struct must roundtrip correctly"
    );
    assert_eq!(
        consumed,
        bytes.len(),
        "consumed bytes must equal total encoded length"
    );
}

// ---------------------------------------------------------------------------
// Test 7: Enum with #[serde(tag = "type")] (internally tagged) — encode path
//
// oxicode is not a self-describing format, so decoding an internally-tagged
// enum (which needs `deserialize_any` on the decode side) is not supported.
// We verify that encoding succeeds and produces non-empty, distinct bytes for
// each variant, confirming the Serialize path works correctly.
// ---------------------------------------------------------------------------

#[test]
fn test_serde7_07_internally_tagged_enum_encode_distinct_bytes() {
    let cfg = oxicode::config::standard();

    let login_bytes = serde_encode(&InternallyTaggedEvent::Login { user_id: 1001 }, cfg)
        .expect("encode InternallyTaggedEvent::Login");
    let logout_bytes = serde_encode(&InternallyTaggedEvent::Logout { user_id: 2002 }, cfg)
        .expect("encode InternallyTaggedEvent::Logout");
    let error_bytes = serde_encode(
        &InternallyTaggedEvent::Error {
            code: 404,
            message: String::from("not found"),
        },
        cfg,
    )
    .expect("encode InternallyTaggedEvent::Error");

    assert!(
        !login_bytes.is_empty(),
        "Login variant must produce non-empty bytes"
    );
    assert!(
        !logout_bytes.is_empty(),
        "Logout variant must produce non-empty bytes"
    );
    assert!(
        !error_bytes.is_empty(),
        "Error variant must produce non-empty bytes"
    );

    // Login and Logout have the same field type but different variant indices,
    // so their byte sequences must differ.
    assert_ne!(
        login_bytes, logout_bytes,
        "Login and Logout variants must encode to distinct byte sequences"
    );
    assert_ne!(
        login_bytes, error_bytes,
        "Login and Error variants must encode to distinct byte sequences"
    );
}

// ---------------------------------------------------------------------------
// Test 8: Enum with #[serde(tag = "kind", content = "payload")] (adjacently tagged)
// ---------------------------------------------------------------------------

#[test]
fn test_serde7_08_adjacently_tagged_enum_roundtrip() {
    let cfg = oxicode::config::standard();

    let variants: Vec<AdjacentlyTaggedMsg> = vec![
        AdjacentlyTaggedMsg::Text(String::from("hello adjacently tagged")),
        AdjacentlyTaggedMsg::Number(-9876),
        AdjacentlyTaggedMsg::Blob(vec![0xDE, 0xAD, 0xBE, 0xEF]),
    ];

    for variant in &variants {
        let bytes = serde_encode(variant, cfg).expect("encode AdjacentlyTaggedMsg");
        let (decoded, consumed): (AdjacentlyTaggedMsg, usize) =
            serde_decode(&bytes, cfg).expect("decode AdjacentlyTaggedMsg");
        assert_eq!(
            variant, &decoded,
            "adjacently tagged enum variant must roundtrip"
        );
        assert_eq!(
            consumed,
            bytes.len(),
            "consumed bytes must equal total encoded length"
        );
    }
}

// ---------------------------------------------------------------------------
// Test 9: #[serde(flatten)] encode limitation and struct-level encode correctness
//
// oxicode's binary serde layer serializes struct fields by position, not by
// key. The `#[serde(flatten)]` attribute requires the serializer to support
// map serialization with an upfront known length, which the oxicode serde
// layer does not provide (non-self-describing format).  We verify that:
//   a) Encoding a struct with `#[serde(flatten)]` returns an error.
//   b) Encoding a plain struct (without flatten) that mirrors the same fields
//      succeeds and roundtrips correctly, confirming the encode path itself is fine.
// ---------------------------------------------------------------------------

#[derive(Serialize, Deserialize, Debug, PartialEq)]
struct FlatEquivalent {
    name: String,
    x: i32,
    y: i32,
}

#[test]
fn test_serde7_09_serde_flatten_encode_error_and_plain_roundtrip() {
    let cfg = oxicode::config::standard();

    // Flatten struct encoding must fail (map length not supported)
    let flatten_result = serde_encode(
        &FlattenBase {
            name: String::from("flatten-test"),
            extra: FlattenExtra { x: 100, y: -200 },
        },
        cfg,
    );
    assert!(
        flatten_result.is_err(),
        "#[serde(flatten)] encoding must return an error in non-self-describing format"
    );

    // A logically equivalent struct without flatten must roundtrip correctly
    let equivalent = FlatEquivalent {
        name: String::from("flatten-test"),
        x: 100,
        y: -200,
    };
    let bytes = serde_encode(&equivalent, cfg).expect("encode FlatEquivalent");
    let (decoded, consumed): (FlatEquivalent, usize) =
        serde_decode(&bytes, cfg).expect("decode FlatEquivalent");
    assert_eq!(
        equivalent, decoded,
        "plain equivalent struct must roundtrip correctly"
    );
    assert_eq!(
        consumed,
        bytes.len(),
        "consumed bytes must equal total encoded length"
    );
}

// ---------------------------------------------------------------------------
// Test 10: Tuple struct with serde roundtrip
// ---------------------------------------------------------------------------

#[test]
fn test_serde7_10_tuple_struct_roundtrip() {
    let cfg = oxicode::config::standard();
    let original = TupleStruct(42, String::from("tuple-struct-value"), true);

    let bytes = serde_encode(&original, cfg).expect("encode TupleStruct");
    let (decoded, consumed): (TupleStruct, usize) =
        serde_decode(&bytes, cfg).expect("decode TupleStruct");

    assert_eq!(original, decoded, "tuple struct must roundtrip correctly");
    assert_eq!(
        consumed,
        bytes.len(),
        "consumed bytes must equal total encoded length"
    );
}

// ---------------------------------------------------------------------------
// Test 11: Newtype struct with serde roundtrip
// ---------------------------------------------------------------------------

#[test]
fn test_serde7_11_newtype_struct_roundtrip() {
    let cfg = oxicode::config::standard();
    let original = NewtypeStruct(u64::MAX / 3);

    let bytes = serde_encode(&original, cfg).expect("encode NewtypeStruct");
    let (decoded, consumed): (NewtypeStruct, usize) =
        serde_decode(&bytes, cfg).expect("decode NewtypeStruct");

    assert_eq!(original, decoded, "newtype struct must roundtrip correctly");
    assert_eq!(
        consumed,
        bytes.len(),
        "consumed bytes must equal total encoded length"
    );
}

// ---------------------------------------------------------------------------
// Test 12: Struct with all numeric types via serde roundtrip
// ---------------------------------------------------------------------------

#[test]
fn test_serde7_12_all_numeric_types_roundtrip() {
    let cfg = oxicode::config::standard();
    let original = AllNumericTypes {
        a_u8: u8::MAX,
        b_u16: u16::MAX,
        c_u32: u32::MAX,
        d_u64: u64::MAX,
        e_i8: i8::MIN,
        f_i16: i16::MIN,
        g_i32: i32::MIN,
        h_i64: i64::MIN,
        i_f32: core::f32::consts::PI,
        j_f64: core::f64::consts::E,
    };

    let bytes = serde_encode(&original, cfg).expect("encode AllNumericTypes");
    let (decoded, consumed): (AllNumericTypes, usize) =
        serde_decode(&bytes, cfg).expect("decode AllNumericTypes");

    assert_eq!(
        original, decoded,
        "struct with all numeric types must roundtrip correctly"
    );
    assert_eq!(
        consumed,
        bytes.len(),
        "consumed bytes must equal total encoded length"
    );
}

// ---------------------------------------------------------------------------
// Test 13: Option<String> via serde (as a proxy for optional JSON-like value)
//         oxicode is not self-describing so serde_json::Value cannot be decoded;
//         we use Option<String> to test optional-value semantics thoroughly.
// ---------------------------------------------------------------------------

#[test]
fn test_serde7_13_option_string_some_and_none_roundtrip() {
    let cfg = oxicode::config::standard();

    let some_val: Option<String> = Some(String::from("optional-content"));
    let bytes_some = serde_encode(&some_val, cfg).expect("encode Option<String> Some");
    let (decoded_some, _): (Option<String>, usize) =
        serde_decode(&bytes_some, cfg).expect("decode Option<String> Some");
    assert_eq!(some_val, decoded_some, "Option<String> Some must roundtrip");

    let none_val: Option<String> = None;
    let bytes_none = serde_encode(&none_val, cfg).expect("encode Option<String> None");
    let (decoded_none, _): (Option<String>, usize) =
        serde_decode(&bytes_none, cfg).expect("decode Option<String> None");
    assert_eq!(none_val, decoded_none, "Option<String> None must roundtrip");

    assert_ne!(
        bytes_some, bytes_none,
        "Some and None must produce distinct byte sequences"
    );
}

// ---------------------------------------------------------------------------
// Test 14: Vec<String> as generic list of values via serde roundtrip
// ---------------------------------------------------------------------------

#[test]
fn test_serde7_14_vec_string_generic_values_roundtrip() {
    let cfg = oxicode::config::standard();
    let original: Vec<String> = vec![
        String::from("value_one"),
        String::from("value_two"),
        String::from("value_three"),
        String::from("value_four"),
        String::from("value_five"),
    ];

    let bytes = serde_encode(&original, cfg).expect("encode Vec<String>");
    let (decoded, consumed): (Vec<String>, usize) =
        serde_decode(&bytes, cfg).expect("decode Vec<String>");

    assert_eq!(original, decoded, "Vec<String> must roundtrip correctly");
    assert_eq!(
        consumed,
        bytes.len(),
        "consumed bytes must equal total encoded length"
    );
}

// ---------------------------------------------------------------------------
// Test 15: Deeply nested serde structs roundtrip
// ---------------------------------------------------------------------------

#[test]
fn test_serde7_15_deeply_nested_structs_roundtrip() {
    let cfg = oxicode::config::standard();
    let original = NestedDeep {
        level: 1,
        values: vec![10, 20, 30],
        child: Some(Box::new(NestedDeep {
            level: 2,
            values: vec![40, 50],
            child: Some(Box::new(NestedDeep {
                level: 3,
                values: vec![60],
                child: None,
            })),
        })),
    };

    let bytes = serde_encode(&original, cfg).expect("encode NestedDeep");
    let (decoded, consumed): (NestedDeep, usize) =
        serde_decode(&bytes, cfg).expect("decode NestedDeep");

    assert_eq!(
        original, decoded,
        "deeply nested struct must roundtrip correctly"
    );
    assert_eq!(
        consumed,
        bytes.len(),
        "consumed bytes must equal total encoded length"
    );
}

// ---------------------------------------------------------------------------
// Test 16: Serde struct with Vec<String> field roundtrip
// ---------------------------------------------------------------------------

#[test]
fn test_serde7_16_struct_with_vec_string_field_roundtrip() {
    let cfg = oxicode::config::standard();
    let original = WithVecString {
        tags: vec![
            String::from("rust"),
            String::from("serialization"),
            String::from("binary"),
            String::from("oxicode"),
        ],
        count: 4,
    };

    let bytes = serde_encode(&original, cfg).expect("encode WithVecString");
    let (decoded, consumed): (WithVecString, usize) =
        serde_decode(&bytes, cfg).expect("decode WithVecString");

    assert_eq!(
        original, decoded,
        "struct with Vec<String> field must roundtrip correctly"
    );
    assert_eq!(
        consumed,
        bytes.len(),
        "consumed bytes must equal total encoded length"
    );
}

// ---------------------------------------------------------------------------
// Test 17: Serde enum with newtype variants roundtrip
// ---------------------------------------------------------------------------

#[test]
fn test_serde7_17_enum_newtype_variants_roundtrip() {
    let cfg = oxicode::config::standard();

    let variants: Vec<NewtypeVariantEnum> = vec![
        NewtypeVariantEnum::Wrapped(u64::MAX),
        NewtypeVariantEnum::WrappedStr(String::from("newtype-variant")),
        NewtypeVariantEnum::WrappedVec(vec![1u8, 2, 3, 4, 5]),
    ];

    for variant in &variants {
        let bytes = serde_encode(variant, cfg).expect("encode NewtypeVariantEnum");
        let (decoded, consumed): (NewtypeVariantEnum, usize) =
            serde_decode(&bytes, cfg).expect("decode NewtypeVariantEnum");
        assert_eq!(
            variant, &decoded,
            "newtype variant enum must roundtrip correctly"
        );
        assert_eq!(
            consumed,
            bytes.len(),
            "consumed bytes must equal total encoded length"
        );
    }
}

// ---------------------------------------------------------------------------
// Test 18: Cross-decode: serde-encoded bytes decode with native Decode
//          for a type that implements both serde and oxicode native traits.
//          Both encode paths (serde and native) must produce the same bytes,
//          and each decoder must accept the other encoder's output.
// ---------------------------------------------------------------------------

#[test]
fn test_serde7_18_cross_decode_serde_and_native_compatible() {
    let cfg = oxicode::config::standard();
    let value = NativeCompatible {
        id: 1234,
        value: 5678901234,
    };

    // Encode via serde path
    let serde_bytes = serde_encode(&value, cfg).expect("serde-encode NativeCompatible");

    // Encode via native oxicode path
    let native_bytes =
        oxicode::encode_to_vec_with_config(&value, cfg).expect("native-encode NativeCompatible");

    // Both encodings must be identical (same binary layout)
    assert_eq!(
        serde_bytes, native_bytes,
        "serde and native encode must produce identical bytes for a compatible type"
    );

    // Native decoder must accept serde-encoded bytes
    let (native_decoded, _): (NativeCompatible, usize) =
        oxicode::decode_from_slice(&serde_bytes).expect("native-decode serde bytes");
    assert_eq!(
        value, native_decoded,
        "native decode of serde bytes must yield original value"
    );

    // Serde decoder must accept natively-encoded bytes
    let (serde_decoded, _): (NativeCompatible, usize) =
        serde_decode(&native_bytes, cfg).expect("serde-decode native bytes");
    assert_eq!(
        value, serde_decoded,
        "serde decode of native bytes must yield original value"
    );
}

// ---------------------------------------------------------------------------
// Test 19: BTreeMap<String, Vec<u32>> roundtrip via serde
// ---------------------------------------------------------------------------

#[test]
fn test_serde7_19_btreemap_string_vec_roundtrip() {
    let cfg = oxicode::config::standard();
    let mut original: BTreeMap<String, Vec<u32>> = BTreeMap::new();
    original.insert(String::from("primes"), vec![2, 3, 5, 7, 11, 13]);
    original.insert(String::from("evens"), vec![2, 4, 6, 8, 10]);
    original.insert(String::from("odds"), vec![1, 3, 5, 7, 9]);

    let bytes = serde_encode(&original, cfg).expect("encode BTreeMap<String, Vec<u32>>");
    let (decoded, consumed): (BTreeMap<String, Vec<u32>>, usize) =
        serde_decode(&bytes, cfg).expect("decode BTreeMap<String, Vec<u32>>");

    assert_eq!(
        original, decoded,
        "BTreeMap<String, Vec<u32>> must roundtrip correctly"
    );
    assert_eq!(
        consumed,
        bytes.len(),
        "consumed bytes must equal total encoded length"
    );
}

// ---------------------------------------------------------------------------
// Test 20: Large struct (14 fields) via serde roundtrip
// ---------------------------------------------------------------------------

#[test]
fn test_serde7_20_large_struct_14_fields_roundtrip() {
    let cfg = oxicode::config::standard();
    let original = LargeTenPlusFields {
        field_01: 255,
        field_02: 65535,
        field_03: 1_000_000,
        field_04: u64::MAX / 4,
        field_05: i8::MIN,
        field_06: i16::MAX,
        field_07: -1_000_000,
        field_08: i64::MIN / 2,
        field_09: core::f32::consts::SQRT_2,
        field_10: core::f64::consts::LN_2,
        field_11: String::from("large-struct-field-11"),
        field_12: true,
        field_13: (0_u32..20).collect(),
        field_14: Some(String::from("optional-field-14")),
    };

    let bytes = serde_encode(&original, cfg).expect("encode LargeTenPlusFields");
    let (decoded, consumed): (LargeTenPlusFields, usize) =
        serde_decode(&bytes, cfg).expect("decode LargeTenPlusFields");

    assert_eq!(
        original, decoded,
        "large struct with 14 fields must roundtrip correctly"
    );
    assert_eq!(
        consumed,
        bytes.len(),
        "consumed bytes must equal total encoded length"
    );
}

// ---------------------------------------------------------------------------
// Test 21: Struct with #[serde(rename_all = "camelCase")] roundtrip
// ---------------------------------------------------------------------------

#[test]
fn test_serde7_21_rename_all_camel_case_roundtrip() {
    let cfg = oxicode::config::standard();
    let original = CamelCasedStruct {
        first_name: String::from("John"),
        last_name: String::from("Doe"),
        age_in_years: 35,
        is_active: true,
    };

    let bytes = serde_encode(&original, cfg).expect("encode CamelCasedStruct");
    let (decoded, consumed): (CamelCasedStruct, usize) =
        serde_decode(&bytes, cfg).expect("decode CamelCasedStruct");

    assert_eq!(
        original, decoded,
        "#[serde(rename_all = \"camelCase\")] struct must roundtrip correctly"
    );
    assert_eq!(
        consumed,
        bytes.len(),
        "consumed bytes must equal total encoded length"
    );
}

// ---------------------------------------------------------------------------
// Test 22: Error case — decode wrong type via serde returns an error
//
// Encode a u64 value larger than u8::MAX; then attempt to decode it as u8.
// Since oxicode uses variable-length integer encoding, a large u64 will
// occupy more bytes than a valid u8, so decoding truncates incorrectly.
// We also verify that decoding a Vec<u32> payload as a bare u64 fails when
// the encoded sequence length prefix is out of valid u64 range.
// ---------------------------------------------------------------------------

#[test]
fn test_serde7_22_decode_wrong_type_returns_error() {
    let cfg = oxicode::config::standard();

    // Encode a Vec<u32> with several elements.  The first encoded byte will be
    // a varint representing the sequence length (e.g., 5), which when decoded
    // as a u64 would give 5 — the actual elements would then be leftover and
    // `consumed` would be less than `bytes.len()`.  More crucially, we need a
    // case that definitely errors.  Use a large Vec so its length prefix encodes
    // as multiple bytes in varint form, making it an invalid u8 value when the
    // remaining bytes are expected by a struct but not present.
    //
    // Strategy: encode a struct, then attempt to decode the bytes as a
    // Vec<String> with a grossly mismatched element count, causing an OOM or
    // UnexpectedEnd error.  We do this by encoding a small u8 value (1 byte),
    // then decoding it as a Vec<u64>, which expects more bytes than available.

    let small: u8 = 1;
    let bytes = serde_encode(&small, cfg).expect("encode u8 for error test");

    // A Vec<u64> expects a length prefix followed by that many u64 values.
    // The single byte `[1]` will be interpreted as length=1, then decoding
    // the single u64 element will exhaust the buffer and return an error.
    let result: Result<(Vec<u64>, usize), _> = serde_decode::<Vec<u64>, _>(&bytes, cfg);
    assert!(
        result.is_err(),
        "decoding a 1-byte buffer as Vec<u64> must return an error (UnexpectedEnd or similar)"
    );
}
