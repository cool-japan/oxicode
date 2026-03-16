// serde_advanced6_test.rs — 22 serde integration tests for OxiCode
// All tests are gated on `#[cfg(feature = "serde")]` at the top level.
// No #[cfg(test)] module wrapper; tests are top-level items.

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
#[cfg(feature = "serde")]
use oxicode::serde::{decode_from_slice as serde_decode, encode_to_vec as serde_encode};
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

// ---------------------------------------------------------------------------
// Shared type definitions (all gated on the serde feature)
// ---------------------------------------------------------------------------

#[cfg(feature = "serde")]
#[derive(Serialize, Deserialize, Debug, PartialEq)]
struct SimpleStruct {
    name: String,
    value: u32,
}

#[cfg(feature = "serde")]
#[derive(Serialize, Deserialize, Debug, PartialEq)]
struct Inner {
    x: i32,
    y: i32,
}

#[cfg(feature = "serde")]
#[derive(Serialize, Deserialize, Debug, PartialEq)]
struct Outer {
    label: String,
    inner: Inner,
}

#[cfg(feature = "serde")]
#[derive(Serialize, Deserialize, Debug, PartialEq)]
struct MultiField {
    a: u8,
    b: u16,
    c: u32,
    d: u64,
    e: bool,
    f: String,
}

#[cfg(feature = "serde")]
#[derive(Serialize, Deserialize, Debug, PartialEq)]
struct LargeStruct {
    id: u64,
    name: String,
    scores: Vec<f64>,
    tags: Vec<String>,
    active: bool,
    metadata: std::collections::BTreeMap<String, u32>,
}

// Enum without internally-tagged representation (uses serde default = externally tagged)
#[cfg(feature = "serde")]
#[derive(Serialize, Deserialize, Debug, PartialEq)]
enum Direction {
    North,
    South,
    East,
    West,
}

// Enum with newtype variants
#[cfg(feature = "serde")]
#[derive(Serialize, Deserialize, Debug, PartialEq)]
enum Wrapper {
    Integer(u32),
    Text(String),
}

// Enum with struct variants
#[cfg(feature = "serde")]
#[derive(Serialize, Deserialize, Debug, PartialEq)]
enum Command {
    Move { dx: i32, dy: i32 },
    Fire { rounds: u32 },
}

// ---------------------------------------------------------------------------
// Test 1: Serialize/deserialize simple u32
// ---------------------------------------------------------------------------

#[cfg(feature = "serde")]
#[test]
fn test_serde6_01_u32_roundtrip() {
    let cfg = oxicode::config::standard();
    let original: u32 = 314_159;
    let bytes = serde_encode(&original, cfg).expect("encode u32");
    let (decoded, _): (u32, usize) = serde_decode(&bytes, cfg).expect("decode u32");
    assert_eq!(original, decoded);
}

// ---------------------------------------------------------------------------
// Test 2: Serialize/deserialize String
// ---------------------------------------------------------------------------

#[cfg(feature = "serde")]
#[test]
fn test_serde6_02_string_roundtrip() {
    let cfg = oxicode::config::standard();
    let original = String::from("Encode me with OxiCode serde!");
    let bytes = serde_encode(&original, cfg).expect("encode String");
    let (decoded, _): (String, usize) = serde_decode(&bytes, cfg).expect("decode String");
    assert_eq!(original, decoded);
}

// ---------------------------------------------------------------------------
// Test 3: Serialize/deserialize Vec<u32>
// ---------------------------------------------------------------------------

#[cfg(feature = "serde")]
#[test]
fn test_serde6_03_vec_u32_roundtrip() {
    let cfg = oxicode::config::standard();
    let original: Vec<u32> = vec![10, 20, 30, 40, 50, 60];
    let bytes = serde_encode(&original, cfg).expect("encode Vec<u32>");
    let (decoded, _): (Vec<u32>, usize) = serde_decode(&bytes, cfg).expect("decode Vec<u32>");
    assert_eq!(original, decoded);
}

// ---------------------------------------------------------------------------
// Test 4: Serialize/deserialize bool
// ---------------------------------------------------------------------------

#[cfg(feature = "serde")]
#[test]
fn test_serde6_04_bool_roundtrip() {
    let cfg = oxicode::config::standard();
    for value in [true, false] {
        let bytes = serde_encode(&value, cfg).expect("encode bool");
        let (decoded, _): (bool, usize) = serde_decode(&bytes, cfg).expect("decode bool");
        assert_eq!(value, decoded);
    }
}

// ---------------------------------------------------------------------------
// Test 5: Serialize/deserialize struct { name: String, value: u32 }
// ---------------------------------------------------------------------------

#[cfg(feature = "serde")]
#[test]
fn test_serde6_05_simple_struct_roundtrip() {
    let cfg = oxicode::config::standard();
    let original = SimpleStruct {
        name: String::from("oxicode"),
        value: 42,
    };
    let bytes = serde_encode(&original, cfg).expect("encode SimpleStruct");
    let (decoded, _): (SimpleStruct, usize) =
        serde_decode(&bytes, cfg).expect("decode SimpleStruct");
    assert_eq!(original, decoded);
}

// ---------------------------------------------------------------------------
// Test 6: Serialize/deserialize Option<u32> Some
// ---------------------------------------------------------------------------

#[cfg(feature = "serde")]
#[test]
fn test_serde6_06_option_some_roundtrip() {
    let cfg = oxicode::config::standard();
    let original: Option<u32> = Some(255);
    let bytes = serde_encode(&original, cfg).expect("encode Option<u32> Some");
    let (decoded, _): (Option<u32>, usize) =
        serde_decode(&bytes, cfg).expect("decode Option<u32> Some");
    assert_eq!(original, decoded);
}

// ---------------------------------------------------------------------------
// Test 7: Serialize/deserialize Option<u32> None
// ---------------------------------------------------------------------------

#[cfg(feature = "serde")]
#[test]
fn test_serde6_07_option_none_roundtrip() {
    let cfg = oxicode::config::standard();
    let original: Option<u32> = None;
    let bytes = serde_encode(&original, cfg).expect("encode Option<u32> None");
    let (decoded, _): (Option<u32>, usize) =
        serde_decode(&bytes, cfg).expect("decode Option<u32> None");
    assert_eq!(original, decoded);
}

// ---------------------------------------------------------------------------
// Test 8: Serialize/deserialize enum with unit variants
// ---------------------------------------------------------------------------

#[cfg(feature = "serde")]
#[test]
fn test_serde6_08_enum_unit_variants_roundtrip() {
    let cfg = oxicode::config::standard();
    for variant in [
        Direction::North,
        Direction::South,
        Direction::East,
        Direction::West,
    ] {
        let bytes = serde_encode(&variant, cfg).expect("encode Direction");
        let (decoded, _): (Direction, usize) = serde_decode(&bytes, cfg).expect("decode Direction");
        assert_eq!(variant, decoded);
    }
}

// ---------------------------------------------------------------------------
// Test 9: Serialize/deserialize enum with newtype variants
// ---------------------------------------------------------------------------

#[cfg(feature = "serde")]
#[test]
fn test_serde6_09_enum_newtype_variants_roundtrip() {
    let cfg = oxicode::config::standard();
    let cases = [Wrapper::Integer(9999), Wrapper::Text(String::from("hello"))];
    for variant in cases {
        let bytes = serde_encode(&variant, cfg).expect("encode Wrapper");
        let (decoded, _): (Wrapper, usize) = serde_decode(&bytes, cfg).expect("decode Wrapper");
        assert_eq!(variant, decoded);
    }
}

// ---------------------------------------------------------------------------
// Test 10: Serialize/deserialize enum with struct variants
// ---------------------------------------------------------------------------

#[cfg(feature = "serde")]
#[test]
fn test_serde6_10_enum_struct_variants_roundtrip() {
    let cfg = oxicode::config::standard();
    let move_cmd = Command::Move { dx: -5, dy: 10 };
    let bytes = serde_encode(&move_cmd, cfg).expect("encode Command::Move");
    let (decoded, _): (Command, usize) = serde_decode(&bytes, cfg).expect("decode Command::Move");
    assert_eq!(move_cmd, decoded);

    let fire_cmd = Command::Fire { rounds: 3 };
    let bytes2 = serde_encode(&fire_cmd, cfg).expect("encode Command::Fire");
    let (decoded2, _): (Command, usize) = serde_decode(&bytes2, cfg).expect("decode Command::Fire");
    assert_eq!(fire_cmd, decoded2);
}

// ---------------------------------------------------------------------------
// Test 11: Serialize/deserialize Vec<String>
// ---------------------------------------------------------------------------

#[cfg(feature = "serde")]
#[test]
fn test_serde6_11_vec_string_roundtrip() {
    let cfg = oxicode::config::standard();
    let original: Vec<String> = vec![
        String::from("alpha"),
        String::from("beta"),
        String::from("gamma"),
    ];
    let bytes = serde_encode(&original, cfg).expect("encode Vec<String>");
    let (decoded, _): (Vec<String>, usize) = serde_decode(&bytes, cfg).expect("decode Vec<String>");
    assert_eq!(original, decoded);
}

// ---------------------------------------------------------------------------
// Test 12: Serialize/deserialize tuple (u32, u64)
// ---------------------------------------------------------------------------

#[cfg(feature = "serde")]
#[test]
fn test_serde6_12_tuple_u32_u64_roundtrip() {
    let cfg = oxicode::config::standard();
    let original: (u32, u64) = (100_u32, 999_999_999_999_u64);
    let bytes = serde_encode(&original, cfg).expect("encode (u32, u64)");
    let (decoded, _): ((u32, u64), usize) = serde_decode(&bytes, cfg).expect("decode (u32, u64)");
    assert_eq!(original, decoded);
}

// ---------------------------------------------------------------------------
// Test 13: Consumed bytes equal encoded length
// ---------------------------------------------------------------------------

#[cfg(feature = "serde")]
#[test]
fn test_serde6_13_consumed_equals_encoded_len() {
    let cfg = oxicode::config::standard();
    let original = SimpleStruct {
        name: String::from("checklen"),
        value: 77,
    };
    let bytes = serde_encode(&original, cfg).expect("encode for length check");
    let (_, consumed): (SimpleStruct, usize) =
        serde_decode(&bytes, cfg).expect("decode for length check");
    assert_eq!(
        consumed,
        bytes.len(),
        "consumed must equal encoded byte length"
    );
}

// ---------------------------------------------------------------------------
// Test 14: Struct with multiple fields roundtrip
// ---------------------------------------------------------------------------

#[cfg(feature = "serde")]
#[test]
fn test_serde6_14_multi_field_struct_roundtrip() {
    let cfg = oxicode::config::standard();
    let original = MultiField {
        a: 255,
        b: 65535,
        c: 1_000_000,
        d: u64::MAX / 2,
        e: true,
        f: String::from("multi-field test"),
    };
    let bytes = serde_encode(&original, cfg).expect("encode MultiField");
    let (decoded, _): (MultiField, usize) = serde_decode(&bytes, cfg).expect("decode MultiField");
    assert_eq!(original, decoded);
}

// ---------------------------------------------------------------------------
// Test 15: Nested structs roundtrip
// ---------------------------------------------------------------------------

#[cfg(feature = "serde")]
#[test]
fn test_serde6_15_nested_structs_roundtrip() {
    let cfg = oxicode::config::standard();
    let original = Outer {
        label: String::from("origin"),
        inner: Inner { x: -100, y: 200 },
    };
    let bytes = serde_encode(&original, cfg).expect("encode Outer");
    let (decoded, _): (Outer, usize) = serde_decode(&bytes, cfg).expect("decode Outer");
    assert_eq!(original, decoded);
}

// ---------------------------------------------------------------------------
// Test 16: Vec<Struct> roundtrip
// ---------------------------------------------------------------------------

#[cfg(feature = "serde")]
#[test]
fn test_serde6_16_vec_struct_roundtrip() {
    let cfg = oxicode::config::standard();
    let original = vec![
        SimpleStruct {
            name: String::from("first"),
            value: 1,
        },
        SimpleStruct {
            name: String::from("second"),
            value: 2,
        },
        SimpleStruct {
            name: String::from("third"),
            value: 3,
        },
    ];
    let bytes = serde_encode(&original, cfg).expect("encode Vec<SimpleStruct>");
    let (decoded, _): (Vec<SimpleStruct>, usize) =
        serde_decode(&bytes, cfg).expect("decode Vec<SimpleStruct>");
    assert_eq!(original, decoded);
}

// ---------------------------------------------------------------------------
// Test 17: Option<Struct> roundtrip (Some and None)
// ---------------------------------------------------------------------------

#[cfg(feature = "serde")]
#[test]
fn test_serde6_17_option_struct_roundtrip() {
    let cfg = oxicode::config::standard();

    let some_val: Option<Inner> = Some(Inner { x: 7, y: -3 });
    let bytes_some = serde_encode(&some_val, cfg).expect("encode Option<Inner> Some");
    let (decoded_some, _): (Option<Inner>, usize) =
        serde_decode(&bytes_some, cfg).expect("decode Option<Inner> Some");
    assert_eq!(some_val, decoded_some);

    let none_val: Option<Inner> = None;
    let bytes_none = serde_encode(&none_val, cfg).expect("encode Option<Inner> None");
    let (decoded_none, _): (Option<Inner>, usize) =
        serde_decode(&bytes_none, cfg).expect("decode Option<Inner> None");
    assert_eq!(none_val, decoded_none);
}

// ---------------------------------------------------------------------------
// Test 18: BTreeMap<String, u32> roundtrip via serde
// ---------------------------------------------------------------------------

#[cfg(feature = "serde")]
#[test]
fn test_serde6_18_btreemap_roundtrip() {
    use std::collections::BTreeMap;

    let cfg = oxicode::config::standard();
    let mut original: BTreeMap<String, u32> = BTreeMap::new();
    original.insert(String::from("apple"), 1);
    original.insert(String::from("banana"), 2);
    original.insert(String::from("cherry"), 3);

    let bytes = serde_encode(&original, cfg).expect("encode BTreeMap");
    let (decoded, _): (BTreeMap<String, u32>, usize) =
        serde_decode(&bytes, cfg).expect("decode BTreeMap");
    assert_eq!(original, decoded);
}

// ---------------------------------------------------------------------------
// Test 19: Serde-encoded bytes for two structurally different types with the
//          same field values are identical (confirming the binary format is
//          field-name-agnostic — only values are encoded).
// ---------------------------------------------------------------------------

#[cfg(feature = "serde")]
#[test]
fn test_serde6_19_field_name_agnostic_encoding() {
    // Two structs with different field names but the same field types and values
    // must produce identical serde bytes, because oxicode's serde layer only
    // encodes field values, not field names.
    #[derive(Serialize, Debug)]
    struct VersionA {
        alpha: u32,
        beta: u64,
    }

    #[derive(Serialize, Debug)]
    struct VersionB {
        first_value: u32,
        second_value: u64,
    }

    let cfg = oxicode::config::standard();
    let a = VersionA {
        alpha: 77,
        beta: 1234,
    };
    let b = VersionB {
        first_value: 77,
        second_value: 1234,
    };

    let bytes_a = serde_encode(&a, cfg).expect("encode VersionA");
    let bytes_b = serde_encode(&b, cfg).expect("encode VersionB");

    assert_eq!(
        bytes_a, bytes_b,
        "structs with same field values but different field names must encode identically"
    );
}

// ---------------------------------------------------------------------------
// Test 20: Serde None vs Some produce different byte sequences
// ---------------------------------------------------------------------------

#[cfg(feature = "serde")]
#[test]
fn test_serde6_20_none_vs_some_bytes_differ() {
    let cfg = oxicode::config::standard();
    let some_val: Option<u32> = Some(0);
    let none_val: Option<u32> = None;

    let bytes_some = serde_encode(&some_val, cfg).expect("encode Some(0)");
    let bytes_none = serde_encode(&none_val, cfg).expect("encode None");

    assert_ne!(
        bytes_some, bytes_none,
        "Some and None must encode to different bytes"
    );
}

// ---------------------------------------------------------------------------
// Test 21: Re-serializing a deserialized value produces identical bytes
// ---------------------------------------------------------------------------

#[cfg(feature = "serde")]
#[test]
fn test_serde6_21_reserialize_gives_same_bytes() {
    let cfg = oxicode::config::standard();
    let original = LargeStruct {
        id: 42,
        name: String::from("reserialize"),
        scores: vec![1.0, 2.5, 3.75],
        tags: vec![String::from("a"), String::from("b")],
        active: true,
        metadata: {
            let mut m = std::collections::BTreeMap::new();
            m.insert(String::from("k"), 99);
            m
        },
    };

    let bytes1 = serde_encode(&original, cfg).expect("first encode");
    let (decoded, _): (LargeStruct, usize) = serde_decode(&bytes1, cfg).expect("first decode");
    let bytes2 = serde_encode(&decoded, cfg).expect("second encode");

    assert_eq!(
        bytes1, bytes2,
        "re-serializing deserialized value must produce identical bytes"
    );
}

// ---------------------------------------------------------------------------
// Test 22: Large struct roundtrip
// ---------------------------------------------------------------------------

#[cfg(feature = "serde")]
#[test]
fn test_serde6_22_large_struct_roundtrip() {
    use std::collections::BTreeMap;

    let cfg = oxicode::config::standard();
    let mut metadata = BTreeMap::new();
    for i in 0_u32..20 {
        metadata.insert(format!("key_{i}"), i * i);
    }

    let original = LargeStruct {
        id: u64::MAX,
        name: String::from("large_struct_roundtrip_test"),
        scores: (0..50).map(|i| f64::from(i) * 0.1).collect(),
        tags: (0..30).map(|i| format!("tag_{i}")).collect(),
        active: false,
        metadata,
    };

    let bytes = serde_encode(&original, cfg).expect("encode LargeStruct");
    let (decoded, consumed): (LargeStruct, usize) =
        serde_decode(&bytes, cfg).expect("decode LargeStruct");

    assert_eq!(original, decoded);
    assert_eq!(
        consumed,
        bytes.len(),
        "consumed must equal total encoded size"
    );
}
