//! Advanced tests for array `[T; N]` encoding in oxicode.
//!
//! Key invariant: arrays do NOT have a length prefix (length is known at compile time).
//! Only `Vec<T>` carries a varint length prefix. These tests verify that property and
//! exercise a wide range of element types, nesting patterns, and configurations.

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
use oxicode::{config, decode_from_slice, encode_to_vec, encoded_size, Decode, Encode};

// ---------------------------------------------------------------------------
// Test 1: [u8; 0] encodes to exactly 0 bytes (no length prefix)
// ---------------------------------------------------------------------------
#[test]
fn test_empty_u8_array_encodes_to_zero_bytes() {
    let arr: [u8; 0] = [];
    let encoded = encode_to_vec(&arr).expect("encode [u8; 0]");
    assert_eq!(
        encoded,
        Vec::<u8>::new(),
        "[u8; 0] must encode to empty bytes"
    );
    let size = encoded_size(&arr).expect("encoded_size [u8; 0]");
    assert_eq!(size, 0);
}

// ---------------------------------------------------------------------------
// Test 2: [u8; 1] with value 42 → single byte [42]
// ---------------------------------------------------------------------------
#[test]
fn test_single_u8_array_value_42() {
    let arr: [u8; 1] = [42];
    let encoded = encode_to_vec(&arr).expect("encode [u8; 1]");
    assert_eq!(
        encoded,
        vec![42u8],
        "[u8; 1] = [42] must be exactly one byte 0x2A"
    );

    let (decoded, consumed): ([u8; 1], usize) =
        decode_from_slice(&encoded).expect("decode [u8; 1]");
    assert_eq!(decoded, arr);
    assert_eq!(consumed, 1);
}

// ---------------------------------------------------------------------------
// Test 3: [u8; 4] = [1,2,3,4] → exact bytes [1,2,3,4] (no overhead)
// ---------------------------------------------------------------------------
#[test]
fn test_u8_array_4_exact_bytes() {
    let arr: [u8; 4] = [1, 2, 3, 4];
    let encoded = encode_to_vec(&arr).expect("encode [u8; 4]");
    assert_eq!(
        encoded,
        vec![1u8, 2, 3, 4],
        "[u8; 4] must encode as raw bytes without any prefix"
    );

    let (decoded, consumed): ([u8; 4], usize) =
        decode_from_slice(&encoded).expect("decode [u8; 4]");
    assert_eq!(decoded, arr);
    assert_eq!(consumed, 4, "must consume exactly 4 bytes");
}

// ---------------------------------------------------------------------------
// Test 4: [u8; 32] all-zeros roundtrip
// ---------------------------------------------------------------------------
#[test]
fn test_u8_array_32_all_zeros_roundtrip() {
    let arr: [u8; 32] = [0u8; 32];
    let encoded = encode_to_vec(&arr).expect("encode [u8; 32]");
    assert_eq!(encoded.len(), 32, "no length prefix for [u8; 32]");

    let (decoded, consumed): ([u8; 32], usize) =
        decode_from_slice(&encoded).expect("decode [u8; 32]");
    assert_eq!(decoded, arr);
    assert_eq!(consumed, 32);
}

// ---------------------------------------------------------------------------
// Test 5: [u8; 255] with bytes 0..=254 roundtrip
// ---------------------------------------------------------------------------
#[test]
fn test_u8_array_255_roundtrip() {
    let mut arr: [u8; 255] = [0u8; 255];
    for (i, b) in arr.iter_mut().enumerate() {
        *b = i as u8;
    }
    let encoded = encode_to_vec(&arr).expect("encode [u8; 255]");
    assert_eq!(encoded.len(), 255, "255-element u8 array must be 255 bytes");

    let (decoded, _): ([u8; 255], usize) = decode_from_slice(&encoded).expect("decode [u8; 255]");
    assert_eq!(decoded, arr);
}

// ---------------------------------------------------------------------------
// Test 6: [u32; 4] roundtrip (varint encoding)
// ---------------------------------------------------------------------------
#[test]
fn test_u32_array_4_roundtrip() {
    let arr: [u32; 4] = [0, 42, 250, 100_000];
    let encoded = encode_to_vec(&arr).expect("encode [u32; 4]");

    // With standard (varint) encoding the sizes vary per element:
    //   0       → 1 byte
    //   42      → 1 byte
    //   250     → 1 byte (250 < 251 threshold)
    //   100_000 → 3 bytes (> 0xFFFF so 3-byte varint)
    // Total = 1+1+1+3 = 6 bytes (no length prefix)
    let expected_size = encoded_size(&arr).expect("encoded_size [u32; 4]");
    assert_eq!(encoded.len(), expected_size);

    let (decoded, consumed): ([u32; 4], usize) =
        decode_from_slice(&encoded).expect("decode [u32; 4]");
    assert_eq!(decoded, arr);
    assert_eq!(consumed, encoded.len());
}

// ---------------------------------------------------------------------------
// Test 7: [u64; 8] roundtrip
// ---------------------------------------------------------------------------
#[test]
fn test_u64_array_8_roundtrip() {
    let arr: [u64; 8] = [0, 1, u64::MAX / 2, u64::MAX, 1000, 999_999, 42, 7];
    let encoded = encode_to_vec(&arr).expect("encode [u64; 8]");

    let (decoded, consumed): ([u64; 8], usize) =
        decode_from_slice(&encoded).expect("decode [u64; 8]");
    assert_eq!(decoded, arr);
    assert_eq!(consumed, encoded.len());

    // Size must match the sum of per-element varint sizes (no prefix)
    let size = encoded_size(&arr).expect("encoded_size [u64; 8]");
    assert_eq!(encoded.len(), size);
}

// ---------------------------------------------------------------------------
// Test 8: [f32; 4] roundtrip using bit-level comparison
// ---------------------------------------------------------------------------
#[test]
fn test_f32_array_4_roundtrip_bits() {
    let arr: [f32; 4] = [0.0_f32, 1.0, -1.0, f32::NAN];
    let encoded = encode_to_vec(&arr).expect("encode [f32; 4]");

    let (decoded, consumed): ([f32; 4], usize) =
        decode_from_slice(&encoded).expect("decode [f32; 4]");
    assert_eq!(consumed, encoded.len());

    // Use bit-level comparison to handle NaN correctly
    for (orig, dec) in arr.iter().zip(decoded.iter()) {
        assert_eq!(
            orig.to_bits(),
            dec.to_bits(),
            "f32 bits must be preserved: orig={orig} dec={dec}"
        );
    }
}

// ---------------------------------------------------------------------------
// Test 9: [f64; 4] roundtrip using bit-level comparison
// ---------------------------------------------------------------------------
#[test]
fn test_f64_array_4_roundtrip_bits() {
    let arr: [f64; 4] = [0.0_f64, std::f64::consts::PI, f64::INFINITY, f64::NAN];
    let encoded = encode_to_vec(&arr).expect("encode [f64; 4]");

    let (decoded, consumed): ([f64; 4], usize) =
        decode_from_slice(&encoded).expect("decode [f64; 4]");
    assert_eq!(consumed, encoded.len());

    for (orig, dec) in arr.iter().zip(decoded.iter()) {
        assert_eq!(
            orig.to_bits(),
            dec.to_bits(),
            "f64 bits must be preserved: orig={orig} dec={dec}"
        );
    }
}

// ---------------------------------------------------------------------------
// Test 10: [bool; 8] mixed roundtrip
// ---------------------------------------------------------------------------
#[test]
fn test_bool_array_8_mixed_roundtrip() {
    let arr: [bool; 8] = [true, false, true, true, false, false, true, false];
    let encoded = encode_to_vec(&arr).expect("encode [bool; 8]");

    // Each bool encodes as a single u8 (0 or 1), so 8 bytes total — no prefix
    assert_eq!(encoded.len(), 8);

    let (decoded, consumed): ([bool; 8], usize) =
        decode_from_slice(&encoded).expect("decode [bool; 8]");
    assert_eq!(decoded, arr);
    assert_eq!(consumed, 8);
}

// ---------------------------------------------------------------------------
// Test 11: [String; 3] roundtrip
// ---------------------------------------------------------------------------
#[test]
fn test_string_array_3_roundtrip() {
    let arr: [String; 3] = [
        String::from("hello"),
        String::from(""),
        String::from("oxicode array"),
    ];
    let encoded = encode_to_vec(&arr).expect("encode [String; 3]");

    let (decoded, consumed): ([String; 3], usize) =
        decode_from_slice(&encoded).expect("decode [String; 3]");
    assert_eq!(decoded, arr);
    assert_eq!(consumed, encoded.len());

    // Each string has a length prefix (varint), but the array itself does not
    let size = encoded_size(&arr).expect("encoded_size [String; 3]");
    assert_eq!(encoded.len(), size);
}

// ---------------------------------------------------------------------------
// Test 12: [Option<u32>; 4] with mixed None/Some roundtrip
// ---------------------------------------------------------------------------
#[test]
fn test_option_u32_array_4_mixed_roundtrip() {
    let arr: [Option<u32>; 4] = [None, Some(0), Some(u32::MAX), None];
    let encoded = encode_to_vec(&arr).expect("encode [Option<u32>; 4]");

    let (decoded, consumed): ([Option<u32>; 4], usize) =
        decode_from_slice(&encoded).expect("decode [Option<u32>; 4]");
    assert_eq!(decoded, arr);
    assert_eq!(consumed, encoded.len());
}

// ---------------------------------------------------------------------------
// Test 13: [(u32, u64); 3] tuple array roundtrip
// ---------------------------------------------------------------------------
#[test]
fn test_tuple_array_3_roundtrip() {
    let arr: [(u32, u64); 3] = [(0, 0), (u32::MAX, u64::MAX), (1_000, 1_000_000_000)];
    let encoded = encode_to_vec(&arr).expect("encode [(u32, u64); 3]");

    let (decoded, consumed): ([(u32, u64); 3], usize) =
        decode_from_slice(&encoded).expect("decode [(u32, u64); 3]");
    assert_eq!(decoded, arr);
    assert_eq!(consumed, encoded.len());

    let size = encoded_size(&arr).expect("encoded_size [(u32, u64); 3]");
    assert_eq!(encoded.len(), size);
}

// ---------------------------------------------------------------------------
// Test 14: Array field in a derived struct roundtrip
// ---------------------------------------------------------------------------
#[derive(Debug, PartialEq, Encode, Decode)]
struct StructWithArray {
    id: u32,
    data: [u8; 8],
    tag: [u32; 2],
}

#[test]
fn test_array_in_struct_roundtrip() {
    let value = StructWithArray {
        id: 42,
        data: [0, 1, 2, 3, 4, 5, 6, 7],
        tag: [100, 200],
    };
    let encoded = encode_to_vec(&value).expect("encode StructWithArray");
    let (decoded, consumed): (StructWithArray, usize) =
        decode_from_slice(&encoded).expect("decode StructWithArray");
    assert_eq!(decoded, value);
    assert_eq!(consumed, encoded.len());
}

// ---------------------------------------------------------------------------
// Test 15: Array field in a derived enum roundtrip
// ---------------------------------------------------------------------------
#[derive(Debug, PartialEq, Encode, Decode)]
enum EnumWithArray {
    None,
    Fixed([u8; 4]),
    Tagged { key: [u8; 2], value: u32 },
}

#[test]
fn test_array_in_enum_roundtrip() {
    let variants = [
        EnumWithArray::None,
        EnumWithArray::Fixed([0xDE, 0xAD, 0xBE, 0xEF]),
        EnumWithArray::Tagged {
            key: [0x01, 0x02],
            value: 999,
        },
    ];
    for variant in &variants {
        let encoded = encode_to_vec(variant).expect("encode EnumWithArray");
        let (decoded, consumed): (EnumWithArray, usize) =
            decode_from_slice(&encoded).expect("decode EnumWithArray");
        assert_eq!(&decoded, variant);
        assert_eq!(consumed, encoded.len());
    }
}

// ---------------------------------------------------------------------------
// Test 16: Nested array [[u8; 4]; 4] roundtrip
// ---------------------------------------------------------------------------
#[test]
fn test_nested_array_4x4_roundtrip() {
    let arr: [[u8; 4]; 4] = [
        [0x00, 0x01, 0x02, 0x03],
        [0x10, 0x11, 0x12, 0x13],
        [0x20, 0x21, 0x22, 0x23],
        [0x30, 0x31, 0x32, 0x33],
    ];
    let encoded = encode_to_vec(&arr).expect("encode [[u8; 4]; 4]");
    // 4 * 4 = 16 bytes, no length prefix at any level
    assert_eq!(encoded.len(), 16);

    let (decoded, consumed): ([[u8; 4]; 4], usize) =
        decode_from_slice(&encoded).expect("decode [[u8; 4]; 4]");
    assert_eq!(decoded, arr);
    assert_eq!(consumed, 16);
}

// ---------------------------------------------------------------------------
// Test 17: encoded_size([u8; 32]) == 32 (no length prefix for arrays)
// ---------------------------------------------------------------------------
#[test]
fn test_encoded_size_u8_array_32_no_prefix() {
    let arr: [u8; 32] = [0u8; 32];
    let size = encoded_size(&arr).expect("encoded_size [u8; 32]");
    assert_eq!(
        size, 32,
        "[u8; 32] encoded_size must be exactly 32 (no length prefix)"
    );

    // Compare with Vec<u8> of same data: Vec adds a varint length prefix
    let v: Vec<u8> = vec![0u8; 32];
    let vec_size = encoded_size(&v).expect("encoded_size Vec<u8>");
    assert!(
        vec_size > size,
        "Vec<u8> must be larger than [u8; N] due to length prefix: vec={vec_size} arr={size}"
    );
}

// ---------------------------------------------------------------------------
// Test 18: encoded_size([u32; 8]) == sum of varint sizes (no length prefix)
// ---------------------------------------------------------------------------
#[test]
fn test_encoded_size_u32_array_8_equals_sum_of_varints() {
    // All values < 251: each encodes as 1 byte with standard varint
    let arr: [u32; 8] = [0, 1, 2, 3, 4, 5, 10, 250];
    let size = encoded_size(&arr).expect("encoded_size [u32; 8]");
    // Each of these 8 values fits in a single varint byte (< 251)
    assert_eq!(
        size, 8,
        "8 u32 values all < 251 must each be 1 varint byte, total 8"
    );

    // No length prefix: confirm array size < Vec of same data
    let v: Vec<u32> = arr.to_vec();
    let vec_size = encoded_size(&v).expect("encoded_size Vec<u32>");
    assert!(
        vec_size > size,
        "Vec<u32> with prefix must be larger than array: vec={vec_size} arr={size}"
    );
}

// ---------------------------------------------------------------------------
// Test 19: [u8; 16] with MIN/MAX values roundtrip
// ---------------------------------------------------------------------------
#[test]
fn test_u8_array_min_max_values() {
    let arr: [u8; 16] = [
        u8::MIN,
        u8::MAX,
        u8::MIN,
        u8::MAX,
        0,
        255,
        1,
        254,
        128,
        127,
        0,
        255,
        64,
        192,
        32,
        224,
    ];
    let encoded = encode_to_vec(&arr).expect("encode [u8; 16] min/max");
    assert_eq!(encoded.len(), 16);
    assert_eq!(encoded[0], u8::MIN);
    assert_eq!(encoded[1], u8::MAX);

    let (decoded, consumed): ([u8; 16], usize) =
        decode_from_slice(&encoded).expect("decode [u8; 16] min/max");
    assert_eq!(decoded, arr);
    assert_eq!(consumed, 16);
}

// ---------------------------------------------------------------------------
// Test 20: Vec<[u8; 4]> roundtrip (vector of arrays)
// ---------------------------------------------------------------------------
#[test]
fn test_vec_of_u8_array_4_roundtrip() {
    let value: Vec<[u8; 4]> = vec![
        [0x01, 0x02, 0x03, 0x04],
        [0xFF, 0xFE, 0xFD, 0xFC],
        [0x00, 0x00, 0x00, 0x00],
    ];
    let encoded = encode_to_vec(&value).expect("encode Vec<[u8; 4]>");

    // Vec has a length prefix (varint for 3 elements = 1 byte), then 3 * 4 = 12 bytes
    // Total: 1 + 12 = 13 bytes
    let size = encoded_size(&value).expect("encoded_size Vec<[u8; 4]>");
    assert_eq!(encoded.len(), size);

    let (decoded, consumed): (Vec<[u8; 4]>, usize) =
        decode_from_slice(&encoded).expect("decode Vec<[u8; 4]>");
    assert_eq!(decoded, value);
    assert_eq!(consumed, encoded.len());
}

// ---------------------------------------------------------------------------
// Test 21: [Vec<u8>; 3] roundtrip (array of vectors)
// ---------------------------------------------------------------------------
#[test]
fn test_array_of_vec_u8_roundtrip() {
    let value: [Vec<u8>; 3] = [vec![1, 2, 3], vec![], vec![255, 0, 128, 64]];
    let encoded = encode_to_vec(&value).expect("encode [Vec<u8>; 3]");

    // The outer array has NO length prefix.
    // Each inner Vec<u8> DOES have a varint length prefix.
    let size = encoded_size(&value).expect("encoded_size [Vec<u8>; 3]");
    assert_eq!(encoded.len(), size);

    let (decoded, consumed): ([Vec<u8>; 3], usize) =
        decode_from_slice(&encoded).expect("decode [Vec<u8>; 3]");
    assert_eq!(decoded, value);
    assert_eq!(consumed, encoded.len());
}

// ---------------------------------------------------------------------------
// Test 22: Array with big_endian config roundtrip
// ---------------------------------------------------------------------------
#[test]
fn test_array_big_endian_config_roundtrip() {
    let arr: [u32; 4] = [0x0102_0304, 0xDEAD_BEEF, 0, u32::MAX];
    let big_endian_cfg = config::standard()
        .with_big_endian()
        .with_fixed_int_encoding();

    let encoded = oxicode::encode_to_vec_with_config(&arr, big_endian_cfg)
        .expect("encode [u32; 4] big-endian");

    // With fixed-int big-endian: each u32 is 4 bytes, total = 16, no prefix
    assert_eq!(
        encoded.len(),
        16,
        "[u32; 4] with fixed-int must be 16 bytes"
    );

    // Verify byte order: first element 0x01020304 in big-endian
    assert_eq!(&encoded[0..4], &[0x01, 0x02, 0x03, 0x04]);
    // Second element 0xDEADBEEF in big-endian
    assert_eq!(&encoded[4..8], &[0xDE, 0xAD, 0xBE, 0xEF]);

    let (decoded, consumed): ([u32; 4], usize) =
        oxicode::decode_from_slice_with_config(&encoded, big_endian_cfg)
            .expect("decode [u32; 4] big-endian");
    assert_eq!(decoded, arr);
    assert_eq!(consumed, 16);

    // Confirm big-endian encoding differs from little-endian (standard)
    let little_endian_cfg = config::standard().with_fixed_int_encoding();
    let le_encoded = oxicode::encode_to_vec_with_config(&arr, little_endian_cfg)
        .expect("encode [u32; 4] little-endian");
    assert_ne!(
        encoded, le_encoded,
        "big-endian and little-endian encodings must differ"
    );
}
