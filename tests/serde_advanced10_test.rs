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
use std::collections::HashMap;

use serde::{Deserialize, Serialize};

// ---------------------------------------------------------------------------
// Shared test types
// ---------------------------------------------------------------------------

#[derive(Debug, PartialEq, Serialize, Deserialize)]
struct SimpleStruct {
    id: u32,
    label: String,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
enum Direction {
    North,
    South,
    East,
    West,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
struct PrimitiveFields {
    flag: bool,
    small: u8,
    medium: u32,
    large: u64,
    signed: i64,
    ratio: f64,
    text: String,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
struct WithRename {
    #[serde(rename = "uid")]
    user_id: u64,
    #[serde(rename = "nm")]
    full_name: String,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
struct WithSkip {
    kept: u32,
    #[serde(skip)]
    skipped: u32,
    also_kept: String,
}

impl Default for WithSkip {
    fn default() -> Self {
        WithSkip {
            kept: 0,
            skipped: 0,
            also_kept: String::new(),
        }
    }
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
struct Inner {
    value: i32,
    tag: String,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
struct Outer {
    name: String,
    inner: Inner,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
struct DeepLevel3 {
    z: f64,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
struct DeepLevel2 {
    y: u32,
    level3: DeepLevel3,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
struct DeepLevel1 {
    x: String,
    level2: DeepLevel2,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
enum TupleVariantEnum {
    Unit,
    Single(u64),
    Pair(String, u32),
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
struct WithVecU8 {
    name: String,
    data: Vec<u8>,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
struct SignedUnsigned {
    signed_val: i64,
    unsigned_val: u64,
}

// ---------------------------------------------------------------------------
// Test 1: Simple struct with serde derives roundtrip
// ---------------------------------------------------------------------------

#[test]
fn test_simple_struct_serde_roundtrip() {
    let original = SimpleStruct {
        id: 42,
        label: "hello".to_string(),
    };
    let enc = oxicode::serde::encode_to_vec(&original, oxicode::config::standard())
        .expect("encode SimpleStruct failed");
    let (decoded, _): (SimpleStruct, usize) =
        oxicode::serde::decode_owned_from_slice(&enc, oxicode::config::standard())
            .expect("decode SimpleStruct failed");
    assert_eq!(original, decoded);
}

// ---------------------------------------------------------------------------
// Test 2: Enum with serde derives roundtrip
// ---------------------------------------------------------------------------

#[test]
fn test_enum_serde_roundtrip() {
    for variant in [
        Direction::North,
        Direction::South,
        Direction::East,
        Direction::West,
    ] {
        let enc = oxicode::serde::encode_to_vec(&variant, oxicode::config::standard())
            .expect("encode Direction failed");
        let (decoded, _): (Direction, usize) =
            oxicode::serde::decode_owned_from_slice(&enc, oxicode::config::standard())
                .expect("decode Direction failed");
        assert_eq!(variant, decoded);
    }
}

// ---------------------------------------------------------------------------
// Test 3: Vec<T> where T has serde derives
// ---------------------------------------------------------------------------

#[test]
fn test_vec_of_serde_struct_roundtrip() {
    let items = vec![
        SimpleStruct {
            id: 1,
            label: "first".to_string(),
        },
        SimpleStruct {
            id: 2,
            label: "second".to_string(),
        },
        SimpleStruct {
            id: 3,
            label: "third".to_string(),
        },
    ];
    let enc = oxicode::serde::encode_to_vec(&items, oxicode::config::standard())
        .expect("encode Vec<SimpleStruct> failed");
    let (decoded, _): (Vec<SimpleStruct>, usize) =
        oxicode::serde::decode_owned_from_slice(&enc, oxicode::config::standard())
            .expect("decode Vec<SimpleStruct> failed");
    assert_eq!(items, decoded);
}

// ---------------------------------------------------------------------------
// Test 4: Option<T> with serde derives — both Some and None
// ---------------------------------------------------------------------------

#[test]
fn test_option_serde_roundtrip() {
    let some_val: Option<SimpleStruct> = Some(SimpleStruct {
        id: 7,
        label: "opt-some".to_string(),
    });
    let enc_some = oxicode::serde::encode_to_vec(&some_val, oxicode::config::standard())
        .expect("encode Option Some failed");
    let (decoded_some, _): (Option<SimpleStruct>, usize) =
        oxicode::serde::decode_owned_from_slice(&enc_some, oxicode::config::standard())
            .expect("decode Option Some failed");
    assert_eq!(some_val, decoded_some);

    let none_val: Option<SimpleStruct> = None;
    let enc_none = oxicode::serde::encode_to_vec(&none_val, oxicode::config::standard())
        .expect("encode Option None failed");
    let (decoded_none, _): (Option<SimpleStruct>, usize) =
        oxicode::serde::decode_owned_from_slice(&enc_none, oxicode::config::standard())
            .expect("decode Option None failed");
    assert_eq!(none_val, decoded_none);
}

// ---------------------------------------------------------------------------
// Test 5: Struct with serde rename attribute roundtrip
// ---------------------------------------------------------------------------

#[test]
fn test_serde_rename_roundtrip() {
    let original = WithRename {
        user_id: 9999,
        full_name: "Alice Wonderland".to_string(),
    };
    let enc = oxicode::serde::encode_to_vec(&original, oxicode::config::standard())
        .expect("encode WithRename failed");
    let (decoded, _): (WithRename, usize) =
        oxicode::serde::decode_owned_from_slice(&enc, oxicode::config::standard())
            .expect("decode WithRename failed");
    assert_eq!(original, decoded);
}

// ---------------------------------------------------------------------------
// Test 6: Struct with serde skip attribute roundtrip
// ---------------------------------------------------------------------------

#[test]
fn test_serde_skip_roundtrip() {
    let original = WithSkip {
        kept: 100,
        skipped: 999,
        also_kept: "still here".to_string(),
    };
    let enc = oxicode::serde::encode_to_vec(&original, oxicode::config::standard())
        .expect("encode WithSkip failed");
    let (decoded, _): (WithSkip, usize) =
        oxicode::serde::decode_owned_from_slice(&enc, oxicode::config::standard())
            .expect("decode WithSkip failed");
    // skipped field is dropped during serialization, defaults to 0 on deserialization
    assert_eq!(decoded.kept, original.kept);
    assert_eq!(decoded.also_kept, original.also_kept);
    assert_eq!(
        decoded.skipped, 0,
        "skipped field should deserialize to default (0)"
    );
}

// ---------------------------------------------------------------------------
// Test 7: Nested serde struct roundtrip
// ---------------------------------------------------------------------------

#[test]
fn test_nested_serde_struct_roundtrip() {
    let original = Outer {
        name: "wrapper".to_string(),
        inner: Inner {
            value: -42,
            tag: "inner-tag".to_string(),
        },
    };
    let enc = oxicode::serde::encode_to_vec(&original, oxicode::config::standard())
        .expect("encode Outer failed");
    let (decoded, _): (Outer, usize) =
        oxicode::serde::decode_owned_from_slice(&enc, oxicode::config::standard())
            .expect("decode Outer failed");
    assert_eq!(original, decoded);
}

// ---------------------------------------------------------------------------
// Test 8: Struct with all primitive field types roundtrip
// ---------------------------------------------------------------------------

#[test]
fn test_all_primitive_fields_roundtrip() {
    let original = PrimitiveFields {
        flag: true,
        small: 255,
        medium: 123_456,
        large: 9_876_543_210,
        signed: -9_876_543_210,
        ratio: std::f64::consts::PI,
        text: "primitives".to_string(),
    };
    let enc = oxicode::serde::encode_to_vec(&original, oxicode::config::standard())
        .expect("encode PrimitiveFields failed");
    let (decoded, _): (PrimitiveFields, usize) =
        oxicode::serde::decode_owned_from_slice(&enc, oxicode::config::standard())
            .expect("decode PrimitiveFields failed");
    assert_eq!(original, decoded);
}

// ---------------------------------------------------------------------------
// Test 9: String field with unicode content
// ---------------------------------------------------------------------------

#[test]
fn test_unicode_string_field_roundtrip() {
    let original = SimpleStruct {
        id: 1,
        label: "日本語テスト 🦀 العربية Ελληνικά".to_string(),
    };
    let enc = oxicode::serde::encode_to_vec(&original, oxicode::config::standard())
        .expect("encode unicode SimpleStruct failed");
    let (decoded, _): (SimpleStruct, usize) =
        oxicode::serde::decode_owned_from_slice(&enc, oxicode::config::standard())
            .expect("decode unicode SimpleStruct failed");
    assert_eq!(original, decoded);
}

// ---------------------------------------------------------------------------
// Test 10: HashMap with serde derive roundtrip
// ---------------------------------------------------------------------------

#[test]
fn test_hashmap_serde_roundtrip() {
    let mut map: HashMap<String, u64> = HashMap::new();
    map.insert("alpha".to_string(), 100);
    map.insert("beta".to_string(), 200);
    map.insert("gamma".to_string(), 300);
    let enc = oxicode::serde::encode_to_vec(&map, oxicode::config::standard())
        .expect("encode HashMap<String, u64> failed");
    let (decoded, _): (HashMap<String, u64>, usize) =
        oxicode::serde::decode_owned_from_slice(&enc, oxicode::config::standard())
            .expect("decode HashMap<String, u64> failed");
    assert_eq!(map, decoded);
}

// ---------------------------------------------------------------------------
// Test 11: Vec<String> roundtrip via serde
// ---------------------------------------------------------------------------

#[test]
fn test_vec_string_serde_roundtrip() {
    let original = vec![
        "one".to_string(),
        "two".to_string(),
        "three".to_string(),
        "four".to_string(),
    ];
    let enc = oxicode::serde::encode_to_vec(&original, oxicode::config::standard())
        .expect("encode Vec<String> failed");
    let (decoded, _): (Vec<String>, usize) =
        oxicode::serde::decode_owned_from_slice(&enc, oxicode::config::standard())
            .expect("decode Vec<String> failed");
    assert_eq!(original, decoded);
}

// ---------------------------------------------------------------------------
// Test 12: bool, u32, f64 fields in one struct
// ---------------------------------------------------------------------------

#[test]
fn test_bool_u32_f64_struct_roundtrip() {
    #[derive(Debug, PartialEq, Serialize, Deserialize)]
    struct Mixed {
        active: bool,
        count: u32,
        score: f64,
    }

    let original = Mixed {
        active: false,
        count: 1_000_000,
        score: -3.14159,
    };
    let enc = oxicode::serde::encode_to_vec(&original, oxicode::config::standard())
        .expect("encode Mixed failed");
    let (decoded, _): (Mixed, usize) =
        oxicode::serde::decode_owned_from_slice(&enc, oxicode::config::standard())
            .expect("decode Mixed failed");
    assert_eq!(original, decoded);
}

// ---------------------------------------------------------------------------
// Test 13: Deeply nested serde struct roundtrip
// ---------------------------------------------------------------------------

#[test]
fn test_deeply_nested_serde_struct_roundtrip() {
    let original = DeepLevel1 {
        x: "deep-test".to_string(),
        level2: DeepLevel2 {
            y: 42,
            level3: DeepLevel3 { z: 2.718281828 },
        },
    };
    let enc = oxicode::serde::encode_to_vec(&original, oxicode::config::standard())
        .expect("encode DeepLevel1 failed");
    let (decoded, _): (DeepLevel1, usize) =
        oxicode::serde::decode_owned_from_slice(&enc, oxicode::config::standard())
            .expect("decode DeepLevel1 failed");
    assert_eq!(original, decoded);
}

// ---------------------------------------------------------------------------
// Test 14: Enum with tuple variant using serde derive
// ---------------------------------------------------------------------------

#[test]
fn test_enum_tuple_variant_serde_roundtrip() {
    let variants = vec![
        TupleVariantEnum::Unit,
        TupleVariantEnum::Single(123_456_789),
        TupleVariantEnum::Pair("hello".to_string(), 99),
    ];
    for variant in variants {
        let enc = oxicode::serde::encode_to_vec(&variant, oxicode::config::standard())
            .expect("encode TupleVariantEnum failed");
        let (decoded, _): (TupleVariantEnum, usize) =
            oxicode::serde::decode_owned_from_slice(&enc, oxicode::config::standard())
                .expect("decode TupleVariantEnum failed");
        assert_eq!(variant, decoded);
    }
}

// ---------------------------------------------------------------------------
// Test 15: Struct with default serde behavior (no serde attrs)
// ---------------------------------------------------------------------------

#[test]
fn test_struct_default_serde_behavior_roundtrip() {
    #[derive(Debug, PartialEq, Serialize, Deserialize)]
    struct Plain {
        a: u8,
        b: u16,
        c: u32,
    }

    let original = Plain {
        a: 1,
        b: 256,
        c: 65_536,
    };
    let enc = oxicode::serde::encode_to_vec(&original, oxicode::config::standard())
        .expect("encode Plain failed");
    let (decoded, _): (Plain, usize) =
        oxicode::serde::decode_owned_from_slice(&enc, oxicode::config::standard())
            .expect("decode Plain failed");
    assert_eq!(original, decoded);
}

// ---------------------------------------------------------------------------
// Test 16: Compare serde encode vs oxicode native encode
// ---------------------------------------------------------------------------

#[test]
fn test_serde_vs_native_encode_comparison() {
    #[derive(Debug, PartialEq, Serialize, Deserialize, oxicode::Encode, oxicode::Decode)]
    struct Coord {
        x: u32,
        y: u32,
    }

    let point = Coord { x: 10, y: 20 };

    let serde_bytes = oxicode::serde::encode_to_vec(&point, oxicode::config::standard())
        .expect("serde encode Coord failed");
    let native_bytes = oxicode::encode_to_vec_with_config(&point, oxicode::config::standard())
        .expect("native encode Coord failed");

    // Both paths must encode the same data and decode correctly
    let (from_serde, _): (Coord, usize) =
        oxicode::serde::decode_owned_from_slice(&serde_bytes, oxicode::config::standard())
            .expect("serde decode Coord failed");
    assert_eq!(point, from_serde);
    assert_eq!(
        serde_bytes, native_bytes,
        "serde and native encode must produce identical bytes"
    );
}

// ---------------------------------------------------------------------------
// Test 17: Large collection with serde derive
// ---------------------------------------------------------------------------

#[test]
fn test_large_collection_serde_roundtrip() {
    let items: Vec<SimpleStruct> = (0..500)
        .map(|i| SimpleStruct {
            id: i,
            label: format!("item-{i}"),
        })
        .collect();
    let enc = oxicode::serde::encode_to_vec(&items, oxicode::config::standard())
        .expect("encode large Vec<SimpleStruct> failed");
    let (decoded, _): (Vec<SimpleStruct>, usize) =
        oxicode::serde::decode_owned_from_slice(&enc, oxicode::config::standard())
            .expect("decode large Vec<SimpleStruct> failed");
    assert_eq!(items.len(), decoded.len());
    assert_eq!(items, decoded);
}

// ---------------------------------------------------------------------------
// Test 18: Struct with i64 and u64 fields
// ---------------------------------------------------------------------------

#[test]
fn test_i64_u64_fields_roundtrip() {
    let cases = vec![
        SignedUnsigned {
            signed_val: i64::MIN,
            unsigned_val: 0,
        },
        SignedUnsigned {
            signed_val: i64::MAX,
            unsigned_val: u64::MAX,
        },
        SignedUnsigned {
            signed_val: -1,
            unsigned_val: 1,
        },
        SignedUnsigned {
            signed_val: 0,
            unsigned_val: 0,
        },
    ];
    for original in cases {
        let enc = oxicode::serde::encode_to_vec(&original, oxicode::config::standard())
            .expect("encode SignedUnsigned failed");
        let (decoded, _): (SignedUnsigned, usize) =
            oxicode::serde::decode_owned_from_slice(&enc, oxicode::config::standard())
                .expect("decode SignedUnsigned failed");
        assert_eq!(original, decoded);
    }
}

// ---------------------------------------------------------------------------
// Test 19: Struct with Vec<u8> field
// ---------------------------------------------------------------------------

#[test]
fn test_vec_u8_field_roundtrip() {
    let original = WithVecU8 {
        name: "binary-blob".to_string(),
        data: vec![0x00, 0xFF, 0xDE, 0xAD, 0xBE, 0xEF, 0x01, 0x80],
    };
    let enc = oxicode::serde::encode_to_vec(&original, oxicode::config::standard())
        .expect("encode WithVecU8 failed");
    let (decoded, _): (WithVecU8, usize) =
        oxicode::serde::decode_owned_from_slice(&enc, oxicode::config::standard())
            .expect("decode WithVecU8 failed");
    assert_eq!(original, decoded);
}

// ---------------------------------------------------------------------------
// Test 20: Roundtrip through serde encode/decode preserves all field values
// ---------------------------------------------------------------------------

#[test]
fn test_all_field_values_preserved_after_roundtrip() {
    let original = PrimitiveFields {
        flag: false,
        small: 128,
        medium: 0xDEAD_BEEF,
        large: 0xCAFE_BABE_1234_5678,
        signed: i64::MIN / 2,
        ratio: -0.0,
        text: "preserve-all".to_string(),
    };
    let enc = oxicode::serde::encode_to_vec(&original, oxicode::config::standard())
        .expect("encode PrimitiveFields for field-preservation test failed");
    let (decoded, _): (PrimitiveFields, usize) =
        oxicode::serde::decode_owned_from_slice(&enc, oxicode::config::standard())
            .expect("decode PrimitiveFields for field-preservation test failed");
    assert_eq!(original.flag, decoded.flag, "flag mismatch");
    assert_eq!(original.small, decoded.small, "small mismatch");
    assert_eq!(original.medium, decoded.medium, "medium mismatch");
    assert_eq!(original.large, decoded.large, "large mismatch");
    assert_eq!(original.signed, decoded.signed, "signed mismatch");
    assert_eq!(
        original.ratio.to_bits(),
        decoded.ratio.to_bits(),
        "ratio mismatch"
    );
    assert_eq!(original.text, decoded.text, "text mismatch");
}

// ---------------------------------------------------------------------------
// Test 21: consumed bytes == encoded length via serde
// ---------------------------------------------------------------------------

#[test]
fn test_consumed_bytes_equals_encoded_length() {
    let original = Outer {
        name: "consumed-test".to_string(),
        inner: Inner {
            value: 777,
            tag: "tag-abc".to_string(),
        },
    };
    let enc = oxicode::serde::encode_to_vec(&original, oxicode::config::standard())
        .expect("encode Outer for consumed-bytes test failed");
    let (_, consumed): (Outer, usize) =
        oxicode::serde::decode_owned_from_slice(&enc, oxicode::config::standard())
            .expect("decode Outer for consumed-bytes test failed");
    assert_eq!(
        consumed,
        enc.len(),
        "consumed bytes ({consumed}) must equal encoded length ({})",
        enc.len()
    );
}

// ---------------------------------------------------------------------------
// Test 22: Error on decode with truncated bytes (serde decode returns Err)
// ---------------------------------------------------------------------------

#[test]
fn test_decode_truncated_bytes_returns_error() {
    let original = PrimitiveFields {
        flag: true,
        small: 42,
        medium: 100,
        large: 200,
        signed: -300,
        ratio: 1.23,
        text: "truncation-test".to_string(),
    };
    let enc = oxicode::serde::encode_to_vec(&original, oxicode::config::standard())
        .expect("encode PrimitiveFields for truncation test failed");

    assert!(
        enc.len() > 4,
        "encoded bytes must be long enough to truncate meaningfully"
    );

    // Truncate to half the encoded length to guarantee a malformed payload
    let truncated = &enc[..enc.len() / 2];
    let result: Result<(PrimitiveFields, usize), _> =
        oxicode::serde::decode_owned_from_slice(truncated, oxicode::config::standard());
    assert!(
        result.is_err(),
        "decoding truncated bytes must return an error"
    );
}
