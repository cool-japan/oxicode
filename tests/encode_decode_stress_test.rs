//! Stress and correctness tests for OxiCode encoding/decoding.
//!
//! These tests exercise a broad range of types, sizes, and structural patterns
//! to verify correctness under load, boundary conditions, and deep nesting.

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
use oxicode::{decode_from_slice, encode_to_vec, Decode, Encode};
use std::collections::{BTreeMap, HashMap};

// ---------------------------------------------------------------------------
// Test 1: Encode and decode 1000 u32 values in a Vec
// ---------------------------------------------------------------------------
#[test]
fn test_encode_decode_1000_u32_vec() {
    let data: Vec<u32> = (0u32..1000).collect();
    let enc = encode_to_vec(&data).expect("encode 1000 u32");
    let (dec, consumed): (Vec<u32>, _) = decode_from_slice(&enc).expect("decode 1000 u32");
    assert_eq!(data, dec, "round-trip of 1000 u32 values must be identical");
    assert_eq!(consumed, enc.len(), "all bytes must be consumed");
}

// ---------------------------------------------------------------------------
// Test 2: Encode and decode 1000 String values in a Vec
// ---------------------------------------------------------------------------
#[test]
fn test_encode_decode_1000_string_vec() {
    let data: Vec<String> = (0u32..1000).map(|i| format!("item_{:04}", i)).collect();
    let enc = encode_to_vec(&data).expect("encode 1000 Strings");
    let (dec, consumed): (Vec<String>, _) = decode_from_slice(&enc).expect("decode 1000 Strings");
    assert_eq!(data, dec, "round-trip of 1000 Strings must be identical");
    assert_eq!(consumed, enc.len());
}

// ---------------------------------------------------------------------------
// Test 3: Encode nested Vec<Vec<u32>> 10x10 matrix
// ---------------------------------------------------------------------------
#[test]
fn test_encode_decode_nested_vec_10x10() {
    let data: Vec<Vec<u32>> = (0u32..10)
        .map(|row| (0u32..10).map(|col| row * 10 + col).collect())
        .collect();
    let enc = encode_to_vec(&data).expect("encode 10x10 nested vec");
    let (dec, consumed): (Vec<Vec<u32>>, _) =
        decode_from_slice(&enc).expect("decode 10x10 nested vec");
    assert_eq!(data, dec, "10x10 nested vec must round-trip correctly");
    assert_eq!(consumed, enc.len());
}

// ---------------------------------------------------------------------------
// Test 4: Encode 256-element Vec<u8> with all byte values (0..=255)
// ---------------------------------------------------------------------------
#[test]
fn test_encode_decode_all_byte_values() {
    let data: Vec<u8> = (0u16..=255).map(|b| b as u8).collect();
    assert_eq!(data.len(), 256);
    let enc = encode_to_vec(&data).expect("encode all byte values");
    let (dec, consumed): (Vec<u8>, _) = decode_from_slice(&enc).expect("decode all byte values");
    assert_eq!(data, dec, "all 256 byte values must survive round-trip");
    assert_eq!(consumed, enc.len());
}

// ---------------------------------------------------------------------------
// Test 5: Encode 500-element Vec<(u32, String)> tuple vec
// ---------------------------------------------------------------------------
#[test]
fn test_encode_decode_500_tuple_vec() {
    let data: Vec<(u32, String)> = (0u32..500).map(|i| (i, format!("val_{}", i))).collect();
    let enc = encode_to_vec(&data).expect("encode 500 tuples");
    let (dec, consumed): (Vec<(u32, String)>, _) =
        decode_from_slice(&enc).expect("decode 500 tuples");
    assert_eq!(
        data, dec,
        "500 (u32, String) tuples must round-trip correctly"
    );
    assert_eq!(consumed, enc.len());
}

// ---------------------------------------------------------------------------
// Test 6: Encode large string (10 KB)
// ---------------------------------------------------------------------------
#[test]
fn test_encode_decode_10kb_string() {
    // Build a 10240-byte string with deterministic content
    let chunk = "abcdefghijklmnopqrstuvwxyz0123456789ABCDEFGHIJKLMNOPQRSTUVWXYZ_-";
    let repeats = (10 * 1024_usize).div_ceil(chunk.len());
    let data: String = chunk.repeat(repeats).chars().take(10 * 1024).collect();
    assert_eq!(data.len(), 10 * 1024, "string must be exactly 10 KiB");
    let enc = encode_to_vec(&data).expect("encode 10KB string");
    let (dec, consumed): (String, _) = decode_from_slice(&enc).expect("decode 10KB string");
    assert_eq!(data, dec, "10KB string must survive round-trip");
    assert_eq!(consumed, enc.len());
}

// ---------------------------------------------------------------------------
// Test 7: Encode very deeply nested Option<Option<Option<u32>>>
// (triple nesting with all four structural combinations)
// ---------------------------------------------------------------------------
#[test]
fn test_encode_decode_triple_nested_option() {
    type TripleOpt = Option<Option<Option<u32>>>;

    let cases: Vec<TripleOpt> = vec![
        Some(Some(Some(0u32))),
        Some(Some(Some(u32::MAX))),
        Some(Some(None)),
        Some(None),
        None,
    ];

    for original in &cases {
        let enc = encode_to_vec(original).expect("encode triple Option");
        let (dec, consumed): (TripleOpt, _) =
            decode_from_slice(&enc).expect("decode triple Option");
        assert_eq!(
            original, &dec,
            "triple Option round-trip failed for {:?}",
            original
        );
        assert_eq!(consumed, enc.len());
    }
}

// ---------------------------------------------------------------------------
// Test 8: Encode Vec<Option<String>> with mix of Some/None (100 elements)
// ---------------------------------------------------------------------------
#[test]
fn test_encode_decode_vec_option_string_mixed() {
    let data: Vec<Option<String>> = (0u32..100)
        .map(|i| {
            if i % 3 == 0 {
                None
            } else {
                Some(format!("present_{}", i))
            }
        })
        .collect();
    let enc = encode_to_vec(&data).expect("encode Vec<Option<String>>");
    let (dec, consumed): (Vec<Option<String>>, _) =
        decode_from_slice(&enc).expect("decode Vec<Option<String>>");
    assert_eq!(
        data, dec,
        "Vec<Option<String>> mixed Some/None must round-trip"
    );
    assert_eq!(consumed, enc.len());
}

// ---------------------------------------------------------------------------
// Test 9: Multiple sequential encode/decode of the same value
// (10 independent round-trips of an identical struct)
// ---------------------------------------------------------------------------
#[derive(Debug, PartialEq, Encode, Decode)]
struct RepeatPayload {
    id: u64,
    label: String,
    flags: Vec<bool>,
}

#[test]
fn test_multiple_sequential_encode_decode_same_value() {
    let value = RepeatPayload {
        id: 0xDEAD_BEEF_CAFE_1234,
        label: "repeated_payload".to_string(),
        flags: vec![true, false, true, true, false],
    };

    for round in 0..10usize {
        let enc = encode_to_vec(&value).expect("encode in sequential round");
        let (dec, consumed): (RepeatPayload, _) =
            decode_from_slice(&enc).expect("decode in sequential round");
        assert_eq!(
            value, dec,
            "round-trip #{} of identical value must produce identical result",
            round
        );
        assert_eq!(
            consumed,
            enc.len(),
            "consumed bytes mismatch in round {}",
            round
        );
    }
}

// ---------------------------------------------------------------------------
// Test 10: Encode then decode then re-encode, verify bytes match
// ---------------------------------------------------------------------------
#[test]
fn test_encode_decode_reencode_bytes_match() {
    let original: Vec<u64> = (0u64..50).map(|x| x * x).collect();

    let enc1 = encode_to_vec(&original).expect("first encode");
    let (decoded, _): (Vec<u64>, _) = decode_from_slice(&enc1).expect("decode");
    let enc2 = encode_to_vec(&decoded).expect("second encode");

    assert_eq!(
        enc1, enc2,
        "re-encoding a decoded value must produce byte-for-byte identical output"
    );
}

// ---------------------------------------------------------------------------
// Test 11: Large BTreeMap<u64, String> with 100 entries
// ---------------------------------------------------------------------------
#[test]
fn test_encode_decode_btreemap_u64_string_100() {
    let data: BTreeMap<u64, String> = (0u64..100)
        .map(|i| (i * 1_000_003, format!("entry_{:06}", i)))
        .collect();
    let enc = encode_to_vec(&data).expect("encode BTreeMap<u64, String>");
    let (dec, consumed): (BTreeMap<u64, String>, _) =
        decode_from_slice(&enc).expect("decode BTreeMap<u64, String>");
    assert_eq!(
        data, dec,
        "BTreeMap<u64, String> with 100 entries must round-trip"
    );
    assert_eq!(consumed, enc.len());
}

// ---------------------------------------------------------------------------
// Test 12: Large HashMap<String, Vec<u32>> with 50 entries
// ---------------------------------------------------------------------------
#[test]
fn test_encode_decode_hashmap_string_vec_u32_50() {
    let mut data: HashMap<String, Vec<u32>> = HashMap::new();
    for i in 0u32..50 {
        let key = format!("key_{:03}", i);
        let val: Vec<u32> = (0..i).collect();
        data.insert(key, val);
    }
    let enc = encode_to_vec(&data).expect("encode HashMap<String, Vec<u32>>");
    let (dec, consumed): (HashMap<String, Vec<u32>>, _) =
        decode_from_slice(&enc).expect("decode HashMap<String, Vec<u32>>");
    assert_eq!(
        data, dec,
        "HashMap with 50 entries must round-trip correctly"
    );
    assert_eq!(consumed, enc.len());
}

// ---------------------------------------------------------------------------
// Test 13: Vec<bool> with 1000 alternating true/false
// ---------------------------------------------------------------------------
#[test]
fn test_encode_decode_vec_bool_1000_alternating() {
    let data: Vec<bool> = (0u32..1000).map(|i| i % 2 == 0).collect();
    let enc = encode_to_vec(&data).expect("encode Vec<bool> 1000 alternating");
    let (dec, consumed): (Vec<bool>, _) =
        decode_from_slice(&enc).expect("decode Vec<bool> 1000 alternating");
    assert_eq!(data, dec, "1000 alternating bool values must round-trip");
    assert_eq!(consumed, enc.len());
}

// ---------------------------------------------------------------------------
// Test 14: Large tuple (u8, u16, u32, u64, i8, i16, i32, i64, bool, String)
// ---------------------------------------------------------------------------
#[test]
fn test_encode_decode_large_primitive_tuple() {
    type BigTuple = (u8, u16, u32, u64, i8, i16, i32, i64, bool, String);

    let data: BigTuple = (
        u8::MAX,
        u16::MAX,
        u32::MAX,
        u64::MAX,
        i8::MIN,
        i16::MIN,
        i32::MIN,
        i64::MIN,
        true,
        "boundary_values_tuple".to_string(),
    );

    let enc = encode_to_vec(&data).expect("encode big primitive tuple");
    let (dec, consumed): (BigTuple, _) =
        decode_from_slice(&enc).expect("decode big primitive tuple");
    assert_eq!(
        data, dec,
        "large primitive tuple must round-trip with boundary values"
    );
    assert_eq!(consumed, enc.len());
}

// ---------------------------------------------------------------------------
// Test 15: Encode String with all ASCII printable characters (0x20..=0x7E)
// ---------------------------------------------------------------------------
#[test]
fn test_encode_decode_string_all_ascii_printable() {
    let data: String = (0x20u8..=0x7E).map(|c| c as char).collect();
    assert_eq!(
        data.len(),
        0x7E - 0x20 + 1,
        "must contain every printable ASCII char"
    );
    let enc = encode_to_vec(&data).expect("encode all printable ASCII string");
    let (dec, consumed): (String, _) =
        decode_from_slice(&enc).expect("decode all printable ASCII string");
    assert_eq!(
        data, dec,
        "String with all ASCII printable chars must round-trip"
    );
    assert_eq!(consumed, enc.len());
}

// ---------------------------------------------------------------------------
// Test 16: Large Vec<f64> with 500 values (including edge cases)
// ---------------------------------------------------------------------------
#[test]
fn test_encode_decode_vec_f64_500() {
    let mut data: Vec<f64> = (0u32..496)
        .map(|i| {
            let x = i as f64;
            x * x * 0.001 - x * 0.5 + 1.234_567_89
        })
        .collect();
    // Append notable edge-case f64 values
    data.push(f64::MAX);
    data.push(f64::MIN_POSITIVE);
    data.push(0.0_f64);
    data.push(-0.0_f64);
    assert_eq!(data.len(), 500);

    let enc = encode_to_vec(&data).expect("encode Vec<f64> 500");
    let (dec, consumed): (Vec<f64>, _) = decode_from_slice(&enc).expect("decode Vec<f64> 500");

    assert_eq!(data.len(), dec.len(), "decoded length must match");
    for (idx, (a, b)) in data.iter().zip(dec.iter()).enumerate() {
        assert_eq!(
            a.to_bits(),
            b.to_bits(),
            "f64 value at index {} must be bit-for-bit identical after round-trip",
            idx
        );
    }
    assert_eq!(consumed, enc.len());
}

// ---------------------------------------------------------------------------
// Test 17: Deeply nested struct (5 levels deep)
// ---------------------------------------------------------------------------
#[derive(Debug, PartialEq, Encode, Decode)]
struct Level5 {
    value: u32,
    tag: String,
}

#[derive(Debug, PartialEq, Encode, Decode)]
struct Level4 {
    inner: Level5,
    extra: u16,
}

#[derive(Debug, PartialEq, Encode, Decode)]
struct Level3 {
    inner: Level4,
    name: String,
}

#[derive(Debug, PartialEq, Encode, Decode)]
struct Level2 {
    inner: Level3,
    items: Vec<u8>,
}

#[derive(Debug, PartialEq, Encode, Decode)]
struct Level1 {
    inner: Level2,
    active: bool,
}

#[test]
fn test_encode_decode_deeply_nested_struct_5_levels() {
    let data = Level1 {
        active: true,
        inner: Level2 {
            items: vec![10, 20, 30, 40, 50],
            inner: Level3 {
                name: "level3_name".to_string(),
                inner: Level4 {
                    extra: 0xBEEF,
                    inner: Level5 {
                        value: 0xDEAD_CAFE,
                        tag: "deepest".to_string(),
                    },
                },
            },
        },
    };

    let enc = encode_to_vec(&data).expect("encode 5-level nested struct");
    let (dec, consumed): (Level1, _) =
        decode_from_slice(&enc).expect("decode 5-level nested struct");
    assert_eq!(
        data, dec,
        "5-level deeply nested struct must round-trip correctly"
    );
    assert_eq!(consumed, enc.len());
}

// ---------------------------------------------------------------------------
// Test 18: Vec<BTreeMap<u32, Vec<String>>> complex collection (10 entries)
// ---------------------------------------------------------------------------
#[test]
fn test_encode_decode_vec_btreemap_vec_string() {
    let data: Vec<BTreeMap<u32, Vec<String>>> = (0u32..10)
        .map(|outer| {
            (0u32..5)
                .map(|inner| {
                    let strings: Vec<String> = (0u32..3)
                        .map(|k| format!("o{}_i{}_k{}", outer, inner, k))
                        .collect();
                    (inner * 100 + outer, strings)
                })
                .collect()
        })
        .collect();

    let enc = encode_to_vec(&data).expect("encode Vec<BTreeMap<u32, Vec<String>>>");
    let (dec, consumed): (Vec<BTreeMap<u32, Vec<String>>>, _) =
        decode_from_slice(&enc).expect("decode Vec<BTreeMap<u32, Vec<String>>>");
    assert_eq!(
        data, dec,
        "complex Vec<BTreeMap<u32, Vec<String>>> must round-trip correctly"
    );
    assert_eq!(consumed, enc.len());
}

// ---------------------------------------------------------------------------
// Test 19: 1000 independent encode/decode cycles — all must succeed
// ---------------------------------------------------------------------------
#[test]
fn test_1000_independent_encode_decode_cycles() {
    for i in 0u32..1000 {
        let value: (u32, String, bool) = (i, format!("cycle_{}", i), i % 2 == 0);
        let enc = encode_to_vec(&value).expect("encode in independent cycle");
        let (dec, consumed): ((u32, String, bool), _) =
            decode_from_slice(&enc).expect("decode in independent cycle");
        assert_eq!(
            value, dec,
            "independent encode/decode cycle {} must produce identical result",
            i
        );
        assert_eq!(
            consumed,
            enc.len(),
            "cycle {}: consumed bytes must match encoded len",
            i
        );
    }
}

// ---------------------------------------------------------------------------
// Test 20: Large binary data Vec<u8> with 4096 deterministic bytes
// ---------------------------------------------------------------------------
#[test]
fn test_encode_decode_4096_deterministic_binary() {
    // Generate a deterministic pseudo-random sequence without using rand:
    // simple LCG: x_{n+1} = (a * x_n + c) mod m
    let mut state: u64 = 0x123456789ABCDEF0;
    let data: Vec<u8> = (0..4096)
        .map(|_| {
            state = state
                .wrapping_mul(6_364_136_223_846_793_005)
                .wrapping_add(1_442_695_040_888_963_407);
            (state >> 56) as u8
        })
        .collect();
    assert_eq!(data.len(), 4096);

    let enc = encode_to_vec(&data).expect("encode 4096 deterministic bytes");
    let (dec, consumed): (Vec<u8>, _) =
        decode_from_slice(&enc).expect("decode 4096 deterministic bytes");
    assert_eq!(
        data, dec,
        "4096 deterministic binary bytes must round-trip exactly"
    );
    assert_eq!(consumed, enc.len());
}

// ---------------------------------------------------------------------------
// Test 21: Encode Vec with 251 elements (varint length threshold boundary)
// ---------------------------------------------------------------------------
#[test]
fn test_encode_decode_vec_251_elements_varint_boundary() {
    // 251 is the threshold used by many varint schemes (0..=250 fit in one byte).
    // This test verifies that a length of exactly 251 is encoded and decoded correctly.
    let data: Vec<u32> = (0u32..251).collect();
    assert_eq!(data.len(), 251);

    let enc = encode_to_vec(&data).expect("encode 251-element vec");
    let (dec, consumed): (Vec<u32>, _) = decode_from_slice(&enc).expect("decode 251-element vec");
    assert_eq!(
        data, dec,
        "Vec with 251 elements must round-trip at varint boundary"
    );
    assert_eq!(consumed, enc.len());
}

// ---------------------------------------------------------------------------
// Test 22: Encode Vec with 252 elements (just past 251 boundary)
// ---------------------------------------------------------------------------
#[test]
fn test_encode_decode_vec_252_elements_past_varint_boundary() {
    // 252 is the first length that may require a wider varint representation.
    // This test verifies correct behaviour immediately past the boundary.
    let data: Vec<u32> = (0u32..252).collect();
    assert_eq!(data.len(), 252);

    let enc = encode_to_vec(&data).expect("encode 252-element vec");
    let (dec, consumed): (Vec<u32>, _) = decode_from_slice(&enc).expect("decode 252-element vec");
    assert_eq!(
        data, dec,
        "Vec with 252 elements must round-trip past varint boundary"
    );
    assert_eq!(consumed, enc.len());

    // Cross-check: the 251-element and 252-element encodings must differ
    // (the length prefix bytes must differ due to the boundary)
    let data_251: Vec<u32> = (0u32..251).collect();
    let enc_251 = encode_to_vec(&data_251).expect("encode 251-element vec for comparison");
    assert_ne!(
        &enc[..enc_251.len().min(enc.len())],
        &enc_251[..enc_251.len().min(enc.len())],
        "encodings of 251 and 252 element vecs must differ somewhere in the length prefix region"
    );
}
