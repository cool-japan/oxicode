//! Large-data stress tests for OxiCode encoding and decoding.
//!
//! These tests exercise correctness at scale — large Vecs, large Maps, deeply
//! nested types, sequential streaming writes, and edge-case spot-checks — all
//! designed to complete within 10 seconds each.

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
use std::collections::{BTreeMap, HashMap};

// ---------------------------------------------------------------------------
// Test 1: Vec<u8> with 1_000_000 elements (distinct value pattern)
// ---------------------------------------------------------------------------
#[test]
fn test_large_vec_u8_1m_roundtrip() {
    let original: Vec<u8> = (0u32..1_000_000)
        .map(|i| ((i * 7 + 13) % 256) as u8)
        .collect();
    assert_eq!(original.len(), 1_000_000);

    let enc = encode_to_vec(&original).expect("encode Vec<u8> 1M elements");
    let (dec, consumed): (Vec<u8>, _) =
        decode_from_slice(&enc).expect("decode Vec<u8> 1M elements");

    assert_eq!(dec.len(), 1_000_000);
    assert_eq!(original, dec);
    assert_eq!(consumed, enc.len());
    // Spot-check first, last, and middle values
    assert_eq!(dec[0], 13u8);
    assert_eq!(dec[999_999], ((999_999u32 * 7 + 13) % 256) as u8);
    assert_eq!(dec[500_000], ((500_000u32 * 7 + 13) % 256) as u8);
}

// ---------------------------------------------------------------------------
// Test 2: Vec<u32> with 100_000 elements
// ---------------------------------------------------------------------------
#[test]
fn test_large_vec_u32_100k_roundtrip() {
    let original: Vec<u32> = (0u32..100_000).map(|i| i * 7 + 3).collect();
    assert_eq!(original.len(), 100_000);

    let enc = encode_to_vec(&original).expect("encode Vec<u32> 100K elements");
    let (dec, consumed): (Vec<u32>, _) =
        decode_from_slice(&enc).expect("decode Vec<u32> 100K elements");

    assert_eq!(dec.len(), 100_000);
    assert_eq!(original, dec);
    assert_eq!(consumed, enc.len());
    assert_eq!(dec[0], 3u32);
    assert_eq!(dec[99_999], 99_999 * 7 + 3);
}

// ---------------------------------------------------------------------------
// Test 3: Vec<u64> with 100_000 elements
// ---------------------------------------------------------------------------
#[test]
fn test_large_vec_u64_100k_roundtrip() {
    let original: Vec<u64> = (0u64..100_000).map(|i| i * 1_000_003).collect();
    assert_eq!(original.len(), 100_000);

    let enc = encode_to_vec(&original).expect("encode Vec<u64> 100K elements");
    let (dec, consumed): (Vec<u64>, _) =
        decode_from_slice(&enc).expect("decode Vec<u64> 100K elements");

    assert_eq!(dec.len(), 100_000);
    assert_eq!(original, dec);
    assert_eq!(consumed, enc.len());
    assert_eq!(dec[0], 0u64);
    assert_eq!(dec[99_999], 99_999u64 * 1_000_003);
}

// ---------------------------------------------------------------------------
// Test 4: Vec<String> with 10_000 strings
// ---------------------------------------------------------------------------
#[test]
fn test_large_vec_string_10k_roundtrip() {
    let original: Vec<String> = (0u32..10_000).map(|i| format!("entry_{:06}", i)).collect();
    assert_eq!(original.len(), 10_000);

    let enc = encode_to_vec(&original).expect("encode Vec<String> 10K");
    let (dec, consumed): (Vec<String>, _) =
        decode_from_slice(&enc).expect("decode Vec<String> 10K");

    assert_eq!(dec.len(), 10_000);
    assert_eq!(original, dec);
    assert_eq!(consumed, enc.len());
    assert_eq!(dec[0], "entry_000000");
    assert_eq!(dec[9_999], "entry_009999");
    assert_eq!(dec[5_000], "entry_005000");
}

// ---------------------------------------------------------------------------
// Test 5: Vec<Vec<u8>> — 1_000 inner vecs of 100 bytes each
// ---------------------------------------------------------------------------
#[test]
fn test_vec_of_inner_vecs_1k_x_100bytes() {
    let original: Vec<Vec<u8>> = (0u32..1_000)
        .map(|outer| {
            (0u32..100)
                .map(|inner| ((outer * 100 + inner) % 256) as u8)
                .collect()
        })
        .collect();
    assert_eq!(original.len(), 1_000);
    assert_eq!(original[0].len(), 100);

    let total_elements: usize = original.iter().map(|v| v.len()).sum();
    assert_eq!(total_elements, 100_000);

    let enc = encode_to_vec(&original).expect("encode Vec<Vec<u8>> 1K×100");
    let (dec, consumed): (Vec<Vec<u8>>, _) =
        decode_from_slice(&enc).expect("decode Vec<Vec<u8>> 1K×100");

    assert_eq!(dec.len(), 1_000);
    assert_eq!(original, dec);
    assert_eq!(consumed, enc.len());
    assert_eq!(dec[42][7], ((42u32 * 100 + 7) % 256) as u8);
}

// ---------------------------------------------------------------------------
// Test 6: String of 1_000_000 characters roundtrip
// ---------------------------------------------------------------------------
#[test]
fn test_large_string_1m_chars_roundtrip() {
    let alphabet = "ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789_-";
    let target_len = 1_000_000usize;
    let repeats = target_len.div_ceil(alphabet.len());
    let original: String = alphabet.repeat(repeats).chars().take(target_len).collect();
    assert_eq!(original.len(), target_len);

    let enc = encode_to_vec(&original).expect("encode 1M-char string");
    let (dec, consumed): (String, _) = decode_from_slice(&enc).expect("decode 1M-char string");

    assert_eq!(dec.len(), target_len);
    assert_eq!(original, dec);
    assert_eq!(consumed, enc.len());
}

// ---------------------------------------------------------------------------
// Test 7: Large struct with 50 u64 fields roundtrip
// ---------------------------------------------------------------------------
#[derive(Debug, PartialEq, oxicode::Encode, oxicode::Decode)]
struct LargeStruct {
    f00: u64,
    f01: u64,
    f02: u64,
    f03: u64,
    f04: u64,
    f05: u64,
    f06: u64,
    f07: u64,
    f08: u64,
    f09: u64,
    f10: u64,
    f11: u64,
    f12: u64,
    f13: u64,
    f14: u64,
    f15: u64,
    f16: u64,
    f17: u64,
    f18: u64,
    f19: u64,
    f20: u64,
    f21: u64,
    f22: u64,
    f23: u64,
    f24: u64,
    f25: u64,
    f26: u64,
    f27: u64,
    f28: u64,
    f29: u64,
    f30: u64,
    f31: u64,
    f32: u64,
    f33: u64,
    f34: u64,
    f35: u64,
    f36: u64,
    f37: u64,
    f38: u64,
    f39: u64,
    f40: u64,
    f41: u64,
    f42: u64,
    f43: u64,
    f44: u64,
    f45: u64,
    f46: u64,
    f47: u64,
    f48: u64,
    f49: u64,
}

#[test]
fn test_large_struct_50_u64_fields_roundtrip() {
    let original = LargeStruct {
        f00: 67890,
        f01: 80235,
        f02: 92580,
        f03: 104925,
        f04: 117270,
        f05: 129615,
        f06: 141960,
        f07: 154305,
        f08: 166650,
        f09: 178995,
        f10: 191340,
        f11: 203685,
        f12: 216030,
        f13: 228375,
        f14: 240720,
        f15: 253065,
        f16: 265410,
        f17: 277755,
        f18: 290100,
        f19: 302445,
        f20: 314790,
        f21: 327135,
        f22: 339480,
        f23: 351825,
        f24: 364170,
        f25: 376515,
        f26: 388860,
        f27: 401205,
        f28: 413550,
        f29: 425895,
        f30: 438240,
        f31: 450585,
        f32: 462930,
        f33: 475275,
        f34: 487620,
        f35: 499965,
        f36: 512310,
        f37: 524655,
        f38: 537000,
        f39: 549345,
        f40: 561690,
        f41: 574035,
        f42: 586380,
        f43: 598725,
        f44: 611070,
        f45: 623415,
        f46: 635760,
        f47: 648105,
        f48: 660450,
        f49: 672795,
    };

    let enc = encode_to_vec(&original).expect("encode LargeStruct 50 fields");
    let (dec, consumed): (LargeStruct, _) =
        decode_from_slice(&enc).expect("decode LargeStruct 50 fields");

    assert_eq!(original, dec);
    assert_eq!(consumed, enc.len());
    assert_eq!(dec.f00, 67890u64);
    assert_eq!(dec.f49, 672795);
}

// ---------------------------------------------------------------------------
// Test 8: Vec<u8> of 10_000 zeros — high-compression scenario
// ---------------------------------------------------------------------------
#[test]
fn test_vec_u8_10k_zeros_roundtrip() {
    let original: Vec<u8> = vec![0u8; 10_000];
    assert_eq!(original.len(), 10_000);

    let enc = encode_to_vec(&original).expect("encode 10K zeros");
    let (dec, consumed): (Vec<u8>, _) = decode_from_slice(&enc).expect("decode 10K zeros");

    assert_eq!(dec.len(), 10_000);
    assert_eq!(original, dec);
    assert_eq!(consumed, enc.len());
    assert!(
        dec.iter().all(|&b| b == 0u8),
        "all decoded bytes must be zero"
    );
}

// ---------------------------------------------------------------------------
// Test 9: Deeply nested Option<Option<Option<Option<u32>>>> — 4-level nesting
// ---------------------------------------------------------------------------
#[test]
fn test_four_level_nested_option_roundtrip() {
    type FourOpt = Option<Option<Option<Option<u32>>>>;

    let cases: &[FourOpt] = &[
        Some(Some(Some(Some(42u32)))),
        Some(Some(Some(Some(u32::MAX)))),
        Some(Some(Some(None))),
        Some(Some(None)),
        Some(None),
        None,
    ];

    for original in cases {
        let enc = encode_to_vec(original).expect("encode 4-level Option");
        let (dec, consumed): (FourOpt, _) = decode_from_slice(&enc).expect("decode 4-level Option");
        assert_eq!(original, &dec, "4-level Option mismatch for {:?}", original);
        assert_eq!(consumed, enc.len());
    }
}

// ---------------------------------------------------------------------------
// Test 10: HashMap<String, u64> with 10_000 entries — compare via BTreeMap
// ---------------------------------------------------------------------------
#[test]
fn test_hashmap_string_u64_10k_roundtrip() {
    let original: HashMap<String, u64> = (0u32..10_000)
        .map(|i| (format!("key_{:06}", i), u64::from(i) * 777))
        .collect();
    assert_eq!(original.len(), 10_000);

    let enc = encode_to_vec(&original).expect("encode HashMap<String, u64> 10K");
    let (dec, consumed): (HashMap<String, u64>, _) =
        decode_from_slice(&enc).expect("decode HashMap<String, u64> 10K");

    assert_eq!(dec.len(), 10_000);
    assert_eq!(consumed, enc.len());

    // Compare via BTreeMap to get deterministic ordering
    let original_bt: BTreeMap<String, u64> = original.into_iter().collect();
    let dec_bt: BTreeMap<String, u64> = dec.into_iter().collect();
    assert_eq!(original_bt, dec_bt);
}

// ---------------------------------------------------------------------------
// Test 11: Vec<i64> with boundary values spread throughout
// ---------------------------------------------------------------------------
#[test]
fn test_large_vec_i64_boundary_values_roundtrip() {
    let boundary_cycle = [i64::MIN, i64::MAX, 0i64, -1i64, 1i64];
    let original: Vec<i64> = (0usize..10_000)
        .map(|i| {
            if i % 1_000 == 0 {
                boundary_cycle[(i / 1_000) % boundary_cycle.len()]
            } else {
                (i as i64) * -999 + 42
            }
        })
        .collect();
    assert_eq!(original.len(), 10_000);

    let enc = encode_to_vec(&original).expect("encode Vec<i64> boundary values");
    let (dec, consumed): (Vec<i64>, _) =
        decode_from_slice(&enc).expect("decode Vec<i64> boundary values");

    assert_eq!(dec.len(), 10_000);
    assert_eq!(original, dec);
    assert_eq!(consumed, enc.len());
    // Verify boundary values at known positions
    assert_eq!(dec[0], i64::MIN);
    assert_eq!(dec[1_000], i64::MAX);
    assert_eq!(dec[2_000], 0i64);
    assert_eq!(dec[3_000], -1i64);
    assert_eq!(dec[4_000], 1i64);
}

// ---------------------------------------------------------------------------
// Test 12: encoded_size of Vec<u8> 1M equals 1M + 5 (U32_BYTE varint header)
// ---------------------------------------------------------------------------
#[test]
fn test_encoded_size_1m_vec_u8() {
    let data: Vec<u8> = (0u32..1_000_000).map(|i| (i % 256) as u8).collect();
    let size = oxicode::encoded_size(&data).expect("encoded_size Vec<u8> 1M");
    // 1_000_000 elements > 65535, so varint uses U32_BYTE tag (1 byte) + LE u32 (4 bytes) = 5 bytes header
    assert_eq!(
        size, 1_000_005,
        "encoded size of 1M Vec<u8> must be 1_000_000 data bytes + 5 varint header bytes"
    );
}

// ---------------------------------------------------------------------------
// Test 13: Vec<bool> with 1_000_000 elements roundtrip
// ---------------------------------------------------------------------------
#[test]
fn test_large_vec_bool_1m_roundtrip() {
    let original: Vec<bool> = (0u32..1_000_000).map(|i| i % 2 == 0).collect();
    assert_eq!(original.len(), 1_000_000);

    let enc = encode_to_vec(&original).expect("encode Vec<bool> 1M");
    let (dec, consumed): (Vec<bool>, _) = decode_from_slice(&enc).expect("decode Vec<bool> 1M");

    assert_eq!(dec.len(), 1_000_000);
    assert_eq!(original, dec);
    assert_eq!(consumed, enc.len());
    assert!(dec[0]);
    assert!(!dec[1]);
    assert!(!dec[999_999]); // 999999 is odd
}

// ---------------------------------------------------------------------------
// Test 14: Sequential encode of 1000 different u64 values to cursor
// ---------------------------------------------------------------------------
#[test]
fn test_sequential_encode_1000_u64_to_cursor() {
    use std::io::Cursor;

    let values: Vec<u64> = (0u64..1000).map(|i| i * i + i + 1).collect();
    let mut cursor = Cursor::new(Vec::<u8>::new());

    for &val in &values {
        oxicode::encode_into_std_write(val, &mut cursor, config::standard())
            .expect("encode u64 to cursor");
    }

    let buf = cursor.into_inner();
    let mut offset = 0usize;
    for (idx, &expected) in values.iter().enumerate() {
        let (decoded, consumed): (u64, _) =
            decode_from_slice(&buf[offset..]).expect("decode u64 from buffer");
        assert_eq!(
            decoded, expected,
            "value at index {idx} must match after cursor round-trip"
        );
        offset += consumed;
    }
    assert_eq!(offset, buf.len(), "all bytes must be consumed");
}

// ---------------------------------------------------------------------------
// Test 15: Large data with high limit config
// ---------------------------------------------------------------------------
#[test]
fn test_large_data_with_high_limit_config() {
    let original: Vec<u8> = (0u32..500_000).map(|i| ((i * 3 + 7) % 256) as u8).collect();
    assert_eq!(original.len(), 500_000);

    let cfg = config::standard().with_limit::<600_000>();

    let enc = oxicode::encode_to_vec_with_config(&original, cfg)
        .expect("encode Vec<u8> 500K with limit config");
    let (dec, consumed): (Vec<u8>, _) = oxicode::decode_from_slice_with_config(&enc, cfg)
        .expect("decode Vec<u8> 500K with limit config");

    assert_eq!(dec.len(), 500_000);
    assert_eq!(original, dec);
    assert_eq!(consumed, enc.len());
}

// ---------------------------------------------------------------------------
// Test 16: Vec<[u8; 16]> with 10_000 elements roundtrip
// ---------------------------------------------------------------------------
#[test]
fn test_vec_u8_array16_10k_roundtrip() {
    let original: Vec<[u8; 16]> = (0u32..10_000)
        .map(|i| {
            let mut arr = [0u8; 16];
            for (j, slot) in arr.iter_mut().enumerate() {
                *slot = ((i + j as u32) % 256) as u8;
            }
            arr
        })
        .collect();
    assert_eq!(original.len(), 10_000);

    let enc = encode_to_vec(&original).expect("encode Vec<[u8;16]> 10K");
    let (dec, consumed): (Vec<[u8; 16]>, _) =
        decode_from_slice(&enc).expect("decode Vec<[u8;16]> 10K");

    assert_eq!(dec.len(), 10_000);
    assert_eq!(original, dec);
    assert_eq!(consumed, enc.len());
    // Spot-check one array element
    let expected_arr: [u8; 16] = {
        let mut a = [0u8; 16];
        for (j, slot) in a.iter_mut().enumerate() {
            *slot = ((42u32 + j as u32) % 256) as u8;
        }
        a
    };
    assert_eq!(dec[42], expected_arr);
}

// ---------------------------------------------------------------------------
// Test 17: Large BTreeMap<String, Vec<u8>> with 1_000 entries
// ---------------------------------------------------------------------------
#[test]
fn test_large_btreemap_string_vec_u8_1k_roundtrip() {
    let original: BTreeMap<String, Vec<u8>> = (0u32..1_000)
        .map(|i| {
            let key = format!("bkey_{:05}", i);
            let val = vec![(i % 256) as u8; 20];
            (key, val)
        })
        .collect();
    assert_eq!(original.len(), 1_000);

    let enc = encode_to_vec(&original).expect("encode BTreeMap<String, Vec<u8>> 1K");
    let (dec, consumed): (BTreeMap<String, Vec<u8>>, _) =
        decode_from_slice(&enc).expect("decode BTreeMap<String, Vec<u8>> 1K");

    assert_eq!(dec.len(), 1_000);
    assert_eq!(original, dec);
    assert_eq!(consumed, enc.len());

    let expected_val = vec![42u8; 20]; // 42 % 256 == 42
    assert_eq!(
        dec.get("bkey_00042").map(Vec::as_slice),
        Some(expected_val.as_slice()),
        "entry bkey_00042 must hold the correct value"
    );
}

// ---------------------------------------------------------------------------
// Test 18: Encode then decode 100_000 u32 values — verify first, last, middle
// ---------------------------------------------------------------------------
#[test]
fn test_encode_decode_100k_u32_verify_first_last_middle() {
    let original: Vec<u32> = (0u32..100_000).map(|i| i * 3).collect();
    assert_eq!(original.len(), 100_000);

    let enc = encode_to_vec(&original).expect("encode Vec<u32> 100K ×3");
    let (dec, consumed): (Vec<u32>, _) = decode_from_slice(&enc).expect("decode Vec<u32> 100K ×3");

    assert_eq!(dec.len(), 100_000);
    assert_eq!(original, dec);
    assert_eq!(consumed, enc.len());
    assert_eq!(dec[0], 0u32, "first element must be 0");
    assert_eq!(dec[99_999], 99_999 * 3, "last element must be 99_999 * 3");
    assert_eq!(dec[50_000], 150_000u32, "middle element must be 150_000");
}

// ---------------------------------------------------------------------------
// Test 19: Vec<(u32, u64, bool)> with 10_000 tuples roundtrip
// ---------------------------------------------------------------------------
#[test]
fn test_vec_of_tuples_10k_roundtrip() {
    let original: Vec<(u32, u64, bool)> = (0u32..10_000)
        .map(|i| (i, u64::from(i) * 1_000_007, i % 3 == 0))
        .collect();
    assert_eq!(original.len(), 10_000);

    let enc = encode_to_vec(&original).expect("encode Vec<(u32,u64,bool)> 10K");
    let (dec, consumed): (Vec<(u32, u64, bool)>, _) =
        decode_from_slice(&enc).expect("decode Vec<(u32,u64,bool)> 10K");

    assert_eq!(dec.len(), 10_000);
    assert_eq!(original, dec);
    assert_eq!(consumed, enc.len());
    assert_eq!(dec[5_000], (5_000u32, 5_000u64 * 1_000_007, 5_000 % 3 == 0));
}

// ---------------------------------------------------------------------------
// Test 20: Very large u128 values in vec
// ---------------------------------------------------------------------------
#[test]
fn test_large_u128_values_roundtrip() {
    let original: Vec<u128> = vec![0u128, 1u128, u64::MAX as u128, u128::MAX];
    assert_eq!(original.len(), 4);

    let enc = encode_to_vec(&original).expect("encode Vec<u128> boundary values");
    let (dec, consumed): (Vec<u128>, _) =
        decode_from_slice(&enc).expect("decode Vec<u128> boundary values");

    assert_eq!(dec.len(), 4);
    assert_eq!(dec[0], 0u128);
    assert_eq!(dec[1], 1u128);
    assert_eq!(dec[2], u64::MAX as u128);
    assert_eq!(dec[3], u128::MAX);
    assert_eq!(consumed, enc.len());
}

// ---------------------------------------------------------------------------
// Test 21: Large data round-trip correctness: encode 500KB, verify edges
// ---------------------------------------------------------------------------
#[test]
fn test_500kb_data_roundtrip_verify_edges() {
    let target_len = 512_000usize;
    let original: Vec<u8> = (0u32..512_000)
        .map(|i| ((i * 251 + 37) % 256) as u8)
        .collect();
    assert_eq!(original.len(), target_len);

    let enc = encode_to_vec(&original).expect("encode 512KB Vec<u8>");
    let (dec, consumed): (Vec<u8>, _) = decode_from_slice(&enc).expect("decode 512KB Vec<u8>");

    assert_eq!(dec.len(), target_len);
    assert_eq!(consumed, enc.len());

    // Verify first 10 bytes
    assert_eq!(&dec[..10], &original[..10], "first 10 bytes must match");
    // Verify last 10 bytes
    assert_eq!(
        &dec[target_len - 10..],
        &original[target_len - 10..],
        "last 10 bytes must match"
    );
}

// ---------------------------------------------------------------------------
// Test 22: Multiple large encodings: 10 × 10_000 u32 values, compare all
// ---------------------------------------------------------------------------
#[test]
fn test_multiple_large_encodings_compare_all() {
    for k in 0u32..10 {
        let original: Vec<u32> = (0u32..10_000).map(|i| i + k * 10_000).collect();
        assert_eq!(original.len(), 10_000);

        let enc = encode_to_vec(&original).expect("encode large Vec<u32> in multi-batch");
        let (dec, consumed): (Vec<u32>, _) =
            decode_from_slice(&enc).expect("decode large Vec<u32> in multi-batch");

        assert_eq!(
            dec.len(),
            10_000,
            "batch {k}: decoded length must be 10_000"
        );
        assert_eq!(
            original, dec,
            "batch {k}: decoded values must match original"
        );
        assert_eq!(consumed, enc.len(), "batch {k}: all bytes must be consumed");
    }
}
