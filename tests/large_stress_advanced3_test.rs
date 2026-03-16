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

#[derive(Debug, PartialEq, Encode, Decode)]
struct NestedVecs {
    data: Vec<Vec<u8>>,
}

#[derive(Debug, PartialEq, Encode, Decode)]
struct LargeStruct {
    id: u64,
    name: String,
    data: Vec<u32>,
    active: bool,
    score: f64,
    tag: u8,
}

// Test 1: Encode/decode Vec<u8> of 50,000 bytes
#[test]
fn test_large_vec_u8_50000() {
    let data: Vec<u8> = (0u8..=255).cycle().take(50_000).collect();
    let enc = encode_to_vec(&data).expect("encode Vec<u8> 50000");
    let (val, _): (Vec<u8>, usize) = decode_from_slice(&enc).expect("decode Vec<u8> 50000");
    assert_eq!(data, val);
}

// Test 2: Encode/decode Vec<u32> of 10,000 elements
#[test]
fn test_large_vec_u32_10000() {
    let data: Vec<u32> = (0u32..10_000).collect();
    let enc = encode_to_vec(&data).expect("encode Vec<u32> 10000");
    let (val, _): (Vec<u32>, usize) = decode_from_slice(&enc).expect("decode Vec<u32> 10000");
    assert_eq!(data, val);
}

// Test 3: Encode/decode Vec<String> of 1,000 strings (each 20 chars)
#[test]
fn test_large_vec_string_1000() {
    let data: Vec<String> = (0u32..1_000).map(|i| format!("{:020}", i)).collect();
    let enc = encode_to_vec(&data).expect("encode Vec<String> 1000");
    let (val, _): (Vec<String>, usize) = decode_from_slice(&enc).expect("decode Vec<String> 1000");
    assert_eq!(data, val);
}

// Test 4: Encode/decode HashMap<String, u32> with 500 entries
#[test]
fn test_large_hashmap_500() {
    let mut data: HashMap<String, u32> = HashMap::new();
    for i in 0u32..500 {
        data.insert(format!("key_{:04}", i), i * 7);
    }
    let enc = encode_to_vec(&data).expect("encode HashMap 500");
    let (val, _): (HashMap<String, u32>, usize) =
        decode_from_slice(&enc).expect("decode HashMap 500");
    assert_eq!(data, val);
}

// Test 5: Encode/decode BTreeMap<u32, String> with 1000 entries
#[test]
fn test_large_btreemap_1000() {
    let mut data: BTreeMap<u32, String> = BTreeMap::new();
    for i in 0u32..1_000 {
        data.insert(i, format!("value_{}", i));
    }
    let enc = encode_to_vec(&data).expect("encode BTreeMap 1000");
    let (val, _): (BTreeMap<u32, String>, usize) =
        decode_from_slice(&enc).expect("decode BTreeMap 1000");
    assert_eq!(data, val);
}

// Test 6: Encode/decode struct with Vec<Vec<u8>> (100 inner vecs of 100 bytes)
#[test]
fn test_nested_vecs_struct_100x100() {
    let inner: Vec<Vec<u8>> = (0u8..100).map(|i| vec![i; 100]).collect();
    let data = NestedVecs { data: inner };
    let enc = encode_to_vec(&data).expect("encode NestedVecs 100x100");
    let (val, _): (NestedVecs, usize) = decode_from_slice(&enc).expect("decode NestedVecs 100x100");
    assert_eq!(data, val);
}

// Test 7: Sequential encode 1000 u32 values, decode all back
#[test]
fn test_sequential_encode_decode_1000_u32() {
    let values: Vec<u32> = (0u32..1_000).collect();
    let mut all_encoded: Vec<Vec<u8>> = Vec::new();
    for &v in &values {
        let enc = encode_to_vec(&v).expect("encode u32 sequential");
        all_encoded.push(enc);
    }
    for (i, enc) in all_encoded.iter().enumerate() {
        let (val, _): (u32, usize) = decode_from_slice(enc).expect("decode u32 sequential");
        assert_eq!(values[i], val);
    }
}

// Test 8: Encode/decode Vec<(u32, String, bool)> with 500 tuples
#[test]
fn test_large_vec_tuple_500() {
    let data: Vec<(u32, String, bool)> = (0u32..500)
        .map(|i| (i, format!("item_{}", i), i % 2 == 0))
        .collect();
    let enc = encode_to_vec(&data).expect("encode Vec<tuple> 500");
    let (val, _): (Vec<(u32, String, bool)>, usize) =
        decode_from_slice(&enc).expect("decode Vec<tuple> 500");
    assert_eq!(data, val);
}

// Test 9: Large struct with all field types and Vec<u32> of 1000 elements
#[test]
fn test_large_struct_all_fields() {
    let data = LargeStruct {
        id: u64::MAX / 2,
        name: String::from("large_struct_test_name"),
        data: (0u32..1_000).collect(),
        active: true,
        score: 3.141592653589793,
        tag: 42,
    };
    let enc = encode_to_vec(&data).expect("encode LargeStruct");
    let (val, _): (LargeStruct, usize) = decode_from_slice(&enc).expect("decode LargeStruct");
    assert_eq!(data, val);
}

// Test 10: Nested Vec<Vec<Vec<u8>>>: 10x10x100 bytes
#[test]
fn test_nested_vec_vec_vec_u8_10x10x100() {
    let data: Vec<Vec<Vec<u8>>> = (0u8..10)
        .map(|i| (0u8..10).map(|j| vec![i.wrapping_add(j); 100]).collect())
        .collect();
    let enc = encode_to_vec(&data).expect("encode Vec<Vec<Vec<u8>>> 10x10x100");
    let (val, _): (Vec<Vec<Vec<u8>>>, usize) =
        decode_from_slice(&enc).expect("decode Vec<Vec<Vec<u8>>> 10x10x100");
    assert_eq!(data, val);
}

// Test 11: Vec<u8> of exactly 65535 bytes roundtrip
#[test]
fn test_vec_u8_exactly_65535() {
    let data: Vec<u8> = (0u8..=255).cycle().take(65535).collect();
    let enc = encode_to_vec(&data).expect("encode Vec<u8> 65535");
    let (val, _): (Vec<u8>, usize) = decode_from_slice(&enc).expect("decode Vec<u8> 65535");
    assert_eq!(data, val);
}

// Test 12: Vec<u8> of 100,000 bytes roundtrip
#[test]
fn test_vec_u8_100000() {
    let data: Vec<u8> = (0u8..=255).cycle().take(100_000).collect();
    let enc = encode_to_vec(&data).expect("encode Vec<u8> 100000");
    let (val, _): (Vec<u8>, usize) = decode_from_slice(&enc).expect("decode Vec<u8> 100000");
    assert_eq!(data, val);
}

// Test 13: String of 10,000 'a' chars roundtrip
#[test]
fn test_string_10000_ascii() {
    let data: String = "a".repeat(10_000);
    let enc = encode_to_vec(&data).expect("encode String 10000 ascii");
    let (val, _): (String, usize) = decode_from_slice(&enc).expect("decode String 10000 ascii");
    assert_eq!(data, val);
}

// Test 14: String of 5,000 '中' chars (multi-byte) roundtrip
#[test]
fn test_string_5000_multibyte() {
    let data: String = "中".repeat(5_000);
    let enc = encode_to_vec(&data).expect("encode String 5000 multibyte");
    let (val, _): (String, usize) = decode_from_slice(&enc).expect("decode String 5000 multibyte");
    assert_eq!(data, val);
}

// Test 15: Vec<u64> of 5,000 elements roundtrip
#[test]
fn test_vec_u64_5000() {
    let data: Vec<u64> = (0u64..5_000).map(|i| i * 1_000_000_007).collect();
    let enc = encode_to_vec(&data).expect("encode Vec<u64> 5000");
    let (val, _): (Vec<u64>, usize) = decode_from_slice(&enc).expect("decode Vec<u64> 5000");
    assert_eq!(data, val);
}

// Test 16: Vec<i64> with alternating min/max values, 2000 elements
#[test]
fn test_vec_i64_alternating_minmax_2000() {
    let data: Vec<i64> = (0..2_000)
        .map(|i| if i % 2 == 0 { i64::MIN } else { i64::MAX })
        .collect();
    let enc = encode_to_vec(&data).expect("encode Vec<i64> alternating minmax");
    let (val, _): (Vec<i64>, usize) =
        decode_from_slice(&enc).expect("decode Vec<i64> alternating minmax");
    assert_eq!(data, val);
}

// Test 17: Vec<f64> of 2000 elements roundtrip (bit-exact)
#[test]
fn test_vec_f64_2000_bit_exact() {
    let data: Vec<f64> = (0..2_000)
        .map(|i| f64::from_bits(0x3FF0_0000_0000_0000u64.wrapping_add(i as u64 * 1_234_567)))
        .collect();
    let enc = encode_to_vec(&data).expect("encode Vec<f64> 2000");
    let (val, _): (Vec<f64>, usize) = decode_from_slice(&enc).expect("decode Vec<f64> 2000");
    assert_eq!(data.len(), val.len());
    for (a, b) in data.iter().zip(val.iter()) {
        assert_eq!(a.to_bits(), b.to_bits(), "f64 bit mismatch");
    }
}

// Test 18: HashMap<String, Vec<u8>> with 100 keys, each value 100 bytes
#[test]
fn test_hashmap_string_vec_u8_100x100() {
    let mut data: HashMap<String, Vec<u8>> = HashMap::new();
    for i in 0u8..100 {
        data.insert(format!("key_{:03}", i), vec![i; 100]);
    }
    let enc = encode_to_vec(&data).expect("encode HashMap<String, Vec<u8>> 100x100");
    let (val, _): (HashMap<String, Vec<u8>>, usize) =
        decode_from_slice(&enc).expect("decode HashMap<String, Vec<u8>> 100x100");
    assert_eq!(data, val);
}

// Test 19: Encoding 10 large structs sequentially, verify bytes match encode_to_vec
#[test]
fn test_sequential_large_struct_encode_consistency() {
    let structs: Vec<LargeStruct> = (0u64..10)
        .map(|i| LargeStruct {
            id: i,
            name: format!("struct_{}", i),
            data: (0u32..100).map(|x| x + i as u32).collect(),
            active: i % 2 == 0,
            score: i as f64 * 1.1,
            tag: (i % 256) as u8,
        })
        .collect();

    for s in &structs {
        let enc1 = encode_to_vec(s).expect("encode large struct first");
        let enc2 = encode_to_vec(s).expect("encode large struct second");
        assert_eq!(enc1, enc2, "sequential encode must be deterministic");
        let (val, _): (LargeStruct, usize) = decode_from_slice(&enc1).expect("decode large struct");
        assert_eq!(s.id, val.id);
        assert_eq!(s.name, val.name);
        assert_eq!(s.data, val.data);
    }
}

// Test 20: Vec<Option<String>> of 1000 elements, half Some half None
#[test]
fn test_vec_option_string_1000_half_none() {
    let data: Vec<Option<String>> = (0u32..1_000)
        .map(|i| {
            if i % 2 == 0 {
                Some(format!("val_{}", i))
            } else {
                None
            }
        })
        .collect();
    let enc = encode_to_vec(&data).expect("encode Vec<Option<String>> 1000");
    let (val, _): (Vec<Option<String>>, usize) =
        decode_from_slice(&enc).expect("decode Vec<Option<String>> 1000");
    assert_eq!(data, val);
}

// Test 21: Vec<u8> encoded then decode_from_slice consumed == encoded length (for large data)
#[test]
fn test_large_vec_u8_consumed_bytes_equals_encoded_len() {
    let data: Vec<u8> = (0u8..=255).cycle().take(20_000).collect();
    let enc = encode_to_vec(&data).expect("encode Vec<u8> 20000");
    let enc_len = enc.len();
    let (val, consumed): (Vec<u8>, usize) =
        decode_from_slice(&enc).expect("decode Vec<u8> 20000 consumed");
    assert_eq!(data, val);
    assert_eq!(
        consumed, enc_len,
        "consumed bytes must equal encoded length"
    );
}

// Test 22: Two large Vec<u8> (each 10000 bytes) encoded sequentially, both decoded correctly
#[test]
fn test_two_large_vec_u8_sequential_decode() {
    let data1: Vec<u8> = (0u8..=255).cycle().take(10_000).collect();
    let data2: Vec<u8> = (0u8..=255).rev().cycle().take(10_000).collect();

    let enc1 = encode_to_vec(&data1).expect("encode Vec<u8> data1");
    let enc2 = encode_to_vec(&data2).expect("encode Vec<u8> data2");

    // Concatenate both encoded buffers
    let mut combined = enc1.clone();
    combined.extend_from_slice(&enc2);

    // Decode first from combined
    let (val1, consumed1): (Vec<u8>, usize) =
        decode_from_slice(&combined).expect("decode Vec<u8> data1 from combined");
    assert_eq!(data1, val1);

    // Decode second starting after first consumed bytes
    let (val2, _): (Vec<u8>, usize) =
        decode_from_slice(&combined[consumed1..]).expect("decode Vec<u8> data2 from combined");
    assert_eq!(data2, val2);
}
