//! Extended regression and correctness tests for OxiCode.
//!
//! Covers edge cases not present in regression_test.rs, decode_resilience_test.rs,
//! or the prior contents of this file. All 22 tests are new and non-duplicate.

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
use oxicode::{decode_from_slice, encode_to_vec, encoded_size, Decode, Encode};
use std::collections::BTreeMap;

// ---------------------------------------------------------------------------
// Test 1: Empty Vec<u8> decodes back to an empty Vec, not an error
// ---------------------------------------------------------------------------
#[test]
fn test_empty_vec_u8_decodes_as_empty_not_error() {
    let original: Vec<u8> = Vec::new();
    let enc = encode_to_vec(&original).expect("encode empty Vec<u8>");
    let (dec, consumed): (Vec<u8>, _) =
        decode_from_slice(&enc).expect("empty Vec<u8> must decode successfully, not error");
    assert!(dec.is_empty(), "decoded Vec<u8> must be empty");
    assert_eq!(consumed, enc.len(), "all encoded bytes must be consumed");
}

// ---------------------------------------------------------------------------
// Test 2: Empty String decodes back to an empty String, not an error
// ---------------------------------------------------------------------------
#[test]
fn test_empty_string_decodes_as_empty_not_error() {
    let original = String::new();
    let enc = encode_to_vec(&original).expect("encode empty String");
    let (dec, consumed): (String, _) =
        decode_from_slice(&enc).expect("empty String must decode successfully, not error");
    assert!(dec.is_empty(), "decoded String must be empty");
    assert_eq!(consumed, enc.len(), "all encoded bytes must be consumed");
}

// ---------------------------------------------------------------------------
// Test 3: u8(0) encodes as a single byte 0x00
// ---------------------------------------------------------------------------
#[test]
fn test_u8_zero_encodes_as_single_byte_0x00() {
    let enc = encode_to_vec(&0u8).expect("encode u8(0)");
    assert_eq!(enc.len(), 1, "u8(0) must produce exactly 1 byte");
    assert_eq!(enc[0], 0x00, "u8(0) must be encoded as 0x00");
    let (dec, _): (u8, _) = decode_from_slice(&enc).expect("decode u8(0)");
    assert_eq!(dec, 0u8, "decoded u8 must be 0");
}

// ---------------------------------------------------------------------------
// Test 4: bool false encodes as 0x00 and bool true encodes as 0x01
// (distinct from existing test which only verifies vec! values;
//  here we additionally confirm independent from u8 encoding)
// ---------------------------------------------------------------------------
#[test]
fn test_bool_false_true_byte_values_and_roundtrip() {
    let enc_false = encode_to_vec(&false).expect("encode false");
    let enc_true = encode_to_vec(&true).expect("encode true");

    // Verify the wire byte values
    assert_eq!(enc_false[0], 0x00, "false must encode as 0x00");
    assert_eq!(enc_true[0], 0x01, "true must encode as 0x01");

    // Verify decoding reconstructs the original value
    let (dec_false, _): (bool, _) = decode_from_slice(&enc_false).expect("decode false");
    let (dec_true, _): (bool, _) = decode_from_slice(&enc_true).expect("decode true");
    assert!(!dec_false, "decoded false must be false");
    assert!(dec_true, "decoded true must be true");

    // Verify bool and u8 share the same byte for 0/1 but bool is semantically distinct
    let u8_zero = encode_to_vec(&0u8).expect("encode u8(0)");
    let u8_one = encode_to_vec(&1u8).expect("encode u8(1)");
    assert_eq!(
        enc_false, u8_zero,
        "bool false and u8(0) must have the same wire encoding"
    );
    assert_eq!(
        enc_true, u8_one,
        "bool true and u8(1) must have the same wire encoding"
    );
}

// ---------------------------------------------------------------------------
// Test 5: Roundtrip of Option<Vec<String>> with nested Some values
// ---------------------------------------------------------------------------
#[test]
fn test_option_vec_string_nested_some_roundtrip() {
    let original: Option<Vec<String>> =
        Some(vec!["first".to_string(), String::new(), "last".to_string()]);
    let enc = encode_to_vec(&original).expect("encode Option<Vec<String>>");
    let (dec, consumed): (Option<Vec<String>>, _) =
        decode_from_slice(&enc).expect("decode Option<Vec<String>>");
    assert_eq!(
        original, dec,
        "Option<Vec<String>> with Some values must roundtrip"
    );
    assert_eq!(consumed, enc.len());

    // Confirm None also round-trips
    let none_val: Option<Vec<String>> = None;
    let enc_none = encode_to_vec(&none_val).expect("encode None<Vec<String>>");
    let (dec_none, _): (Option<Vec<String>>, _) =
        decode_from_slice(&enc_none).expect("decode None<Vec<String>>");
    assert_eq!(none_val, dec_none);
}

// ---------------------------------------------------------------------------
// Test 6: Roundtrip of Vec<Option<u32>> with mix of Some and None
// ---------------------------------------------------------------------------
#[test]
fn test_vec_option_u32_mixed_some_none_roundtrip() {
    let original: Vec<Option<u32>> = vec![
        Some(0),
        None,
        Some(u32::MAX),
        None,
        Some(1),
        Some(100_000),
        None,
    ];
    let enc = encode_to_vec(&original).expect("encode Vec<Option<u32>>");
    let (dec, consumed): (Vec<Option<u32>>, _) =
        decode_from_slice(&enc).expect("decode Vec<Option<u32>>");
    assert_eq!(
        original, dec,
        "Vec<Option<u32>> mixed Some/None must roundtrip exactly"
    );
    assert_eq!(consumed, enc.len());
    // Spot-check the None entries
    assert!(dec[1].is_none());
    assert!(dec[3].is_none());
    assert!(dec[6].is_none());
}

// ---------------------------------------------------------------------------
// Test 7: Unit struct (0 fields) encodes and decodes correctly
// ---------------------------------------------------------------------------
#[derive(Debug, PartialEq, Encode, Decode)]
struct ZeroFieldUnit;

#[test]
fn test_unit_struct_zero_fields_roundtrip() {
    let original = ZeroFieldUnit;
    let enc = encode_to_vec(&original).expect("encode ZeroFieldUnit");
    let (dec, consumed): (ZeroFieldUnit, _) =
        decode_from_slice(&enc).expect("decode ZeroFieldUnit");
    assert_eq!(original, dec, "unit struct must roundtrip");
    assert_eq!(
        consumed,
        enc.len(),
        "all bytes must be consumed for unit struct"
    );
}

// ---------------------------------------------------------------------------
// Test 8: Encode of very long string (1000 chars) roundtrips without loss
// ---------------------------------------------------------------------------
#[test]
fn test_very_long_string_1000_chars_roundtrip() {
    let original: String = "A".repeat(1000);
    let enc = encode_to_vec(&original).expect("encode 1000-char string");
    let (dec, consumed): (String, _) = decode_from_slice(&enc).expect("decode 1000-char string");
    assert_eq!(original, dec, "1000-char string must roundtrip losslessly");
    assert_eq!(dec.len(), 1000, "decoded string length must be 1000");
    assert_eq!(consumed, enc.len());
}

// ---------------------------------------------------------------------------
// Test 9: Decode from truncated bytes returns Err, not panic
// ---------------------------------------------------------------------------
#[test]
fn test_decode_from_truncated_bytes_returns_err_not_panic() {
    // Use a non-trivial payload: a struct with multiple fields
    #[derive(Debug, PartialEq, Encode, Decode)]
    struct MultiField {
        a: u64,
        b: String,
        c: Vec<u8>,
    }

    let original = MultiField {
        a: 0xDEAD_BEEF_CAFE_BABE,
        b: "truncation test".to_string(),
        c: vec![1, 2, 3, 4, 5],
    };

    let enc = encode_to_vec(&original).expect("encode MultiField");
    assert!(
        enc.len() > 2,
        "encoding must have multiple bytes to truncate"
    );

    // Every prefix shorter than the full encoding must return Err
    for len in 0..enc.len() {
        let result: Result<(MultiField, _), _> = decode_from_slice(&enc[..len]);
        assert!(
            result.is_err(),
            "truncated input (len={len}) must return Err, not succeed"
        );
    }
}

// ---------------------------------------------------------------------------
// Test 10: Extra bytes after a value are reported via the consumed-byte count
// ---------------------------------------------------------------------------
#[test]
fn test_extra_bytes_after_decode_reported_as_remaining() {
    let value = 77u64;
    let mut enc = encode_to_vec(&value).expect("encode u64");
    let value_byte_count = enc.len();

    // Append known trailing garbage
    let trailer: &[u8] = &[0xCA, 0xFE, 0xBA, 0xBE, 0xDE, 0xAD];
    enc.extend_from_slice(trailer);

    let (dec, consumed): (u64, _) = decode_from_slice(&enc).expect("decode with trailing bytes");
    assert_eq!(dec, value, "decoded value must match");
    assert_eq!(
        consumed, value_byte_count,
        "consumed byte count must equal only the value's encoded size"
    );
    let remaining = enc.len() - consumed;
    assert_eq!(
        remaining,
        trailer.len(),
        "remaining bytes must equal the trailer length"
    );
    assert_eq!(
        &enc[consumed..],
        trailer,
        "the unconsumed tail must equal the trailer"
    );
}

// ---------------------------------------------------------------------------
// Test 11: Roundtrip of struct containing all zero integer/float/bool values
// ---------------------------------------------------------------------------
#[derive(Debug, PartialEq, Encode, Decode)]
struct AllZeroComplex {
    a: u8,
    b: u32,
    c: u64,
    d: i32,
    e: i64,
    f: f32,
    g: f64,
    h: bool,
}

#[test]
fn test_struct_all_zero_values_roundtrip() {
    let original = AllZeroComplex {
        a: 0,
        b: 0,
        c: 0,
        d: 0,
        e: 0,
        f: 0.0,
        g: 0.0,
        h: false,
    };
    let enc = encode_to_vec(&original).expect("encode all-zero struct");
    let (dec, consumed): (AllZeroComplex, _) =
        decode_from_slice(&enc).expect("decode all-zero struct");
    assert_eq!(original, dec, "all-zero struct must roundtrip");
    assert_eq!(consumed, enc.len());
}

// ---------------------------------------------------------------------------
// Test 12: Roundtrip of struct containing all maximum integer values
// ---------------------------------------------------------------------------
#[derive(Debug, PartialEq, Encode, Decode)]
struct AllMaxIntegers {
    a: u8,
    b: u16,
    c: u32,
    d: u64,
    e: i8,
    f: i16,
    g: i32,
    h: i64,
}

#[test]
fn test_struct_all_max_values_roundtrip() {
    let original = AllMaxIntegers {
        a: u8::MAX,
        b: u16::MAX,
        c: u32::MAX,
        d: u64::MAX,
        e: i8::MAX,
        f: i16::MAX,
        g: i32::MAX,
        h: i64::MAX,
    };
    let enc = encode_to_vec(&original).expect("encode all-max struct");
    let (dec, consumed): (AllMaxIntegers, _) =
        decode_from_slice(&enc).expect("decode all-max struct");
    assert_eq!(original, dec, "all-max struct must roundtrip");
    assert_eq!(consumed, enc.len());
}

// ---------------------------------------------------------------------------
// Test 13: Two distinct values always produce distinct byte sequences
// ---------------------------------------------------------------------------
#[test]
fn test_two_different_values_produce_different_byte_sequences() {
    // Integers
    let enc_a = encode_to_vec(&10u32).expect("encode 10u32");
    let enc_b = encode_to_vec(&11u32).expect("encode 11u32");
    assert_ne!(enc_a, enc_b, "10u32 and 11u32 must encode differently");

    // Strings
    let enc_s1 = encode_to_vec(&"abc".to_string()).expect("encode abc");
    let enc_s2 = encode_to_vec(&"abd".to_string()).expect("encode abd");
    assert_ne!(
        enc_s1, enc_s2,
        "\"abc\" and \"abd\" must encode differently"
    );

    // Vecs
    let enc_v1 = encode_to_vec(&vec![1u8, 2, 3]).expect("encode vec [1,2,3]");
    let enc_v2 = encode_to_vec(&vec![1u8, 2, 4]).expect("encode vec [1,2,4]");
    assert_ne!(
        enc_v1, enc_v2,
        "[1,2,3] and [1,2,4] must encode differently"
    );

    // Options: None vs Some
    let enc_none = encode_to_vec(&(None::<u32>)).expect("encode None");
    let enc_some = encode_to_vec(&Some(0u32)).expect("encode Some(0)");
    assert_ne!(
        enc_none, enc_some,
        "None and Some(0) must encode differently"
    );
}

// ---------------------------------------------------------------------------
// Test 14: Two identical values produce identical byte sequences
// ---------------------------------------------------------------------------
#[test]
fn test_two_identical_values_produce_identical_byte_sequences() {
    let enc1 = encode_to_vec(&99_999u64).expect("encode first 99999");
    let enc2 = encode_to_vec(&99_999u64).expect("encode second 99999");
    assert_eq!(enc1, enc2, "same u64 value must always encode identically");

    let s = "deterministic encoding".to_string();
    let enc_s1 = encode_to_vec(&s).expect("encode string first time");
    let enc_s2 = encode_to_vec(&s).expect("encode string second time");
    assert_eq!(enc_s1, enc_s2, "same String must always encode identically");

    let v: Vec<u32> = vec![0, 1, 2, 3, u32::MAX];
    let enc_v1 = encode_to_vec(&v).expect("encode vec first time");
    let enc_v2 = encode_to_vec(&v).expect("encode vec second time");
    assert_eq!(enc_v1, enc_v2, "same Vec must always encode identically");
}

// ---------------------------------------------------------------------------
// Test 15: Roundtrip of tuple (u8, u16, u32, u64) with boundary values
// ---------------------------------------------------------------------------
#[test]
fn test_tuple_u8_u16_u32_u64_boundary_roundtrip() {
    // All-zero tuple
    let zero_tuple: (u8, u16, u32, u64) = (0, 0, 0, 0);
    let enc_zero = encode_to_vec(&zero_tuple).expect("encode zero tuple");
    let (dec_zero, consumed_zero): ((u8, u16, u32, u64), _) =
        decode_from_slice(&enc_zero).expect("decode zero tuple");
    assert_eq!(zero_tuple, dec_zero, "zero tuple must roundtrip");
    assert_eq!(consumed_zero, enc_zero.len());

    // All-max tuple
    let max_tuple: (u8, u16, u32, u64) = (u8::MAX, u16::MAX, u32::MAX, u64::MAX);
    let enc_max = encode_to_vec(&max_tuple).expect("encode max tuple");
    let (dec_max, consumed_max): ((u8, u16, u32, u64), _) =
        decode_from_slice(&enc_max).expect("decode max tuple");
    assert_eq!(max_tuple, dec_max, "max tuple must roundtrip");
    assert_eq!(consumed_max, enc_max.len());

    // Mixed tuple
    let mixed_tuple: (u8, u16, u32, u64) = (1, 256, 65536, u64::MAX / 2);
    let enc_mixed = encode_to_vec(&mixed_tuple).expect("encode mixed tuple");
    let (dec_mixed, consumed_mixed): ((u8, u16, u32, u64), _) =
        decode_from_slice(&enc_mixed).expect("decode mixed tuple");
    assert_eq!(mixed_tuple, dec_mixed, "mixed tuple must roundtrip");
    assert_eq!(consumed_mixed, enc_mixed.len());

    // All three encodings must be distinct
    assert_ne!(enc_zero, enc_max, "zero and max tuples must differ");
    assert_ne!(enc_zero, enc_mixed, "zero and mixed tuples must differ");
    assert_ne!(enc_max, enc_mixed, "max and mixed tuples must differ");
}

// ---------------------------------------------------------------------------
// Test 16: Enum with 4 variants roundtrips including discriminant boundaries
// ---------------------------------------------------------------------------
#[derive(Debug, PartialEq, Encode, Decode)]
enum FourVariantEnum {
    First,
    Second,
    Third,
    Fourth,
}

#[test]
fn test_enum_four_variants_discriminant_boundaries_roundtrip() {
    let all = [
        FourVariantEnum::First,
        FourVariantEnum::Second,
        FourVariantEnum::Third,
        FourVariantEnum::Fourth,
    ];

    let encodings: Vec<Vec<u8>> = all
        .iter()
        .map(|v| encode_to_vec(v).expect("encode enum variant"))
        .collect();

    // Each variant decodes back correctly
    for (i, enc) in encodings.iter().enumerate() {
        let (dec, consumed): (FourVariantEnum, _) =
            decode_from_slice(enc).expect("decode enum variant");
        assert_eq!(&all[i], &dec, "variant {i} must roundtrip correctly");
        assert_eq!(consumed, enc.len(), "variant {i} consumed wrong byte count");
    }

    // All four encodings must be pairwise distinct
    for i in 0..encodings.len() {
        for j in (i + 1)..encodings.len() {
            assert_ne!(
                encodings[i], encodings[j],
                "enum variants {i} and {j} must produce distinct encodings"
            );
        }
    }
}

// ---------------------------------------------------------------------------
// Test 17: BTreeMap with 0 entries roundtrips correctly
// ---------------------------------------------------------------------------
#[test]
fn test_btreemap_zero_entries_roundtrip() {
    let original: BTreeMap<u32, String> = BTreeMap::new();
    let enc = encode_to_vec(&original).expect("encode empty BTreeMap");
    let (dec, consumed): (BTreeMap<u32, String>, _) =
        decode_from_slice(&enc).expect("decode empty BTreeMap");
    assert_eq!(original, dec, "empty BTreeMap must roundtrip");
    assert_eq!(consumed, enc.len());
    assert!(dec.is_empty(), "decoded BTreeMap must have 0 entries");
}

// ---------------------------------------------------------------------------
// Test 18: BTreeMap with 1 entry roundtrips and preserves key-value pair
// ---------------------------------------------------------------------------
#[test]
fn test_btreemap_single_entry_roundtrip() {
    let mut original: BTreeMap<String, u64> = BTreeMap::new();
    original.insert("oxicode".to_string(), 20260314u64);

    let enc = encode_to_vec(&original).expect("encode single-entry BTreeMap");
    let (dec, consumed): (BTreeMap<String, u64>, _) =
        decode_from_slice(&enc).expect("decode single-entry BTreeMap");
    assert_eq!(original, dec, "single-entry BTreeMap must roundtrip");
    assert_eq!(consumed, enc.len());
    assert_eq!(
        dec.get("oxicode"),
        Some(&20260314u64),
        "key-value pair must be preserved"
    );
}

// ---------------------------------------------------------------------------
// Test 19: Vec<Vec<u8>> with empty inner vecs roundtrips correctly
// ---------------------------------------------------------------------------
#[test]
fn test_vec_of_vec_u8_with_empty_inner_vecs_roundtrip() {
    let original: Vec<Vec<u8>> = vec![vec![], vec![10, 20, 30], vec![], vec![255], vec![]];
    let enc = encode_to_vec(&original).expect("encode Vec<Vec<u8>> with empties");
    let (dec, consumed): (Vec<Vec<u8>>, _) =
        decode_from_slice(&enc).expect("decode Vec<Vec<u8>> with empties");
    assert_eq!(
        original, dec,
        "Vec<Vec<u8>> with empty inner vecs must roundtrip"
    );
    assert_eq!(consumed, enc.len());
    // Confirm the empty inner vecs are preserved
    assert!(dec[0].is_empty(), "first inner vec must remain empty");
    assert!(dec[2].is_empty(), "third inner vec must remain empty");
    assert!(dec[4].is_empty(), "fifth inner vec must remain empty");
    assert_eq!(
        dec[1],
        vec![10u8, 20, 30],
        "non-empty inner vec must be preserved"
    );
}

// ---------------------------------------------------------------------------
// Test 20: Option<Option<u32>> — all four combinations roundtrip and differ
// ---------------------------------------------------------------------------
#[test]
fn test_option_of_option_u32_all_combinations_roundtrip() {
    let cases: Vec<Option<Option<u32>>> =
        vec![None, Some(None), Some(Some(0)), Some(Some(u32::MAX))];

    let encodings: Vec<Vec<u8>> = cases
        .iter()
        .map(|c| encode_to_vec(c).expect("encode Option<Option<u32>>"))
        .collect();

    // Each case decodes back to itself
    for (i, enc) in encodings.iter().enumerate() {
        let (dec, consumed): (Option<Option<u32>>, _) =
            decode_from_slice(enc).expect("decode Option<Option<u32>>");
        assert_eq!(
            cases[i], dec,
            "Option<Option<u32>> case {i} ({:?}) must roundtrip",
            cases[i]
        );
        assert_eq!(consumed, enc.len(), "case {i} must consume all bytes");
    }

    // All four encodings must be pairwise distinct
    for i in 0..encodings.len() {
        for j in (i + 1)..encodings.len() {
            assert_ne!(
                encodings[i], encodings[j],
                "Option<Option<u32>> case {i} and {j} must encode differently"
            );
        }
    }
}

// ---------------------------------------------------------------------------
// Test 21: encoded_size matches encode_to_vec length for complex nested types
// ---------------------------------------------------------------------------
#[derive(Debug, PartialEq, Encode, Decode)]
struct NestedComplex {
    id: u64,
    label: String,
    tags: Vec<String>,
    score: Option<f64>,
    active: bool,
}

#[test]
fn test_encoded_size_matches_encode_to_vec_length_for_complex_types() {
    let values = vec![
        NestedComplex {
            id: 0,
            label: String::new(),
            tags: vec![],
            score: None,
            active: false,
        },
        NestedComplex {
            id: u64::MAX,
            label: "maximum label with many characters for thorough size testing".to_string(),
            tags: vec![
                "tag-a".to_string(),
                "tag-b".to_string(),
                "tag-c".to_string(),
            ],
            score: Some(std::f64::consts::PI),
            active: true,
        },
        NestedComplex {
            id: 42,
            label: "mid".to_string(),
            tags: vec![String::new(), "x".to_string()],
            score: Some(0.0),
            active: false,
        },
    ];

    for value in &values {
        let size = encoded_size(value).expect("encoded_size must succeed for NestedComplex");
        let bytes = encode_to_vec(value).expect("encode_to_vec must succeed for NestedComplex");
        assert_eq!(
            size,
            bytes.len(),
            "encoded_size ({size}) must exactly match encode_to_vec length ({}) for {:?}",
            bytes.len(),
            value
        );
    }

    // Also verify for primitive types
    for n in [0u64, 1, 127, 128, 255, u64::MAX] {
        let size = encoded_size(&n).expect("encoded_size u64");
        let bytes = encode_to_vec(&n).expect("encode_to_vec u64");
        assert_eq!(
            size,
            bytes.len(),
            "encoded_size({n}) = {size} must match encode_to_vec length {}",
            bytes.len()
        );
    }
}

// ---------------------------------------------------------------------------
// Test 22: Re-encoding after decoding produces byte-for-byte identical output
// ---------------------------------------------------------------------------
#[test]
fn test_reencode_after_decode_produces_identical_bytes() {
    // u64
    let orig_u64: u64 = 0x0102_0304_0506_0708;
    let enc1 = encode_to_vec(&orig_u64).expect("first encode u64");
    let (dec_u64, _): (u64, _) = decode_from_slice(&enc1).expect("decode u64");
    let enc2 = encode_to_vec(&dec_u64).expect("re-encode u64");
    assert_eq!(
        enc1, enc2,
        "u64 re-encoding must be byte-for-byte identical"
    );

    // String
    let orig_str = "oxicode stability".to_string();
    let enc_s1 = encode_to_vec(&orig_str).expect("first encode String");
    let (dec_str, _): (String, _) = decode_from_slice(&enc_s1).expect("decode String");
    let enc_s2 = encode_to_vec(&dec_str).expect("re-encode String");
    assert_eq!(
        enc_s1, enc_s2,
        "String re-encoding must be byte-for-byte identical"
    );

    // Vec<u32>
    let orig_vec: Vec<u32> = vec![0, 127, 128, 255, u32::MAX / 3, u32::MAX];
    let enc_v1 = encode_to_vec(&orig_vec).expect("first encode Vec<u32>");
    let (dec_vec, _): (Vec<u32>, _) = decode_from_slice(&enc_v1).expect("decode Vec<u32>");
    let enc_v2 = encode_to_vec(&dec_vec).expect("re-encode Vec<u32>");
    assert_eq!(
        enc_v1, enc_v2,
        "Vec<u32> re-encoding must be byte-for-byte identical"
    );

    // BTreeMap
    let mut orig_map: BTreeMap<u32, u64> = BTreeMap::new();
    orig_map.insert(1u32, 100u64);
    orig_map.insert(2u32, 200u64);
    orig_map.insert(1000u32, u64::MAX);
    let enc_m1 = encode_to_vec(&orig_map).expect("first encode BTreeMap");
    let (dec_map, _): (BTreeMap<u32, u64>, _) =
        decode_from_slice(&enc_m1).expect("decode BTreeMap");
    let enc_m2 = encode_to_vec(&dec_map).expect("re-encode BTreeMap");
    assert_eq!(
        enc_m1, enc_m2,
        "BTreeMap re-encoding must be byte-for-byte identical"
    );

    // Option<Vec<String>>
    let orig_opt: Option<Vec<String>> = Some(vec!["re-encode".to_string(), "test".to_string()]);
    let enc_o1 = encode_to_vec(&orig_opt).expect("first encode Option<Vec<String>>");
    let (dec_opt, _): (Option<Vec<String>>, _) =
        decode_from_slice(&enc_o1).expect("decode Option<Vec<String>>");
    let enc_o2 = encode_to_vec(&dec_opt).expect("re-encode Option<Vec<String>>");
    assert_eq!(
        enc_o1, enc_o2,
        "Option<Vec<String>> re-encoding must be byte-for-byte identical"
    );
}
