//! Advanced tests for copy-based encoding in OxiCode, covering fixed arrays,
//! tuples, structs, and bulk encoding patterns via `encode_copy` and related APIs.

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
use oxicode::{
    config, decode_from_slice, decode_from_slice_with_config, encode_to_vec,
    encode_to_vec_with_config,
};

// ============================================================================
// Test 1: [u8; 4] fixed array roundtrip via encode_copy
// ============================================================================

#[test]
fn test_copy_fixed_array_u8_4_roundtrip() {
    let original: [u8; 4] = [0xDE, 0xAD, 0xBE, 0xEF];
    let bytes = oxicode::encode_copy(original).expect("encode [u8;4]");
    let (decoded, consumed): ([u8; 4], usize) = decode_from_slice(&bytes).expect("decode [u8;4]");
    assert_eq!(decoded, original);
    assert_eq!(consumed, bytes.len());
}

// ============================================================================
// Test 2: [u32; 8] fixed array roundtrip via encode_copy
// ============================================================================

#[test]
fn test_copy_fixed_array_u32_8_roundtrip() {
    let original: [u32; 8] = [0, 1, 127, 128, 255, 256, 65535, u32::MAX];
    let bytes = oxicode::encode_copy(original).expect("encode [u32;8]");
    let (decoded, _): ([u32; 8], usize) = decode_from_slice(&bytes).expect("decode [u32;8]");
    assert_eq!(decoded, original);
}

// ============================================================================
// Test 3: [u64; 4] fixed array roundtrip via encode_copy
// ============================================================================

#[test]
fn test_copy_fixed_array_u64_4_roundtrip() {
    let original: [u64; 4] = [0, 1, u32::MAX as u64 + 1, u64::MAX];
    let bytes = oxicode::encode_copy(original).expect("encode [u64;4]");
    let (decoded, _): ([u64; 4], usize) = decode_from_slice(&bytes).expect("decode [u64;4]");
    assert_eq!(decoded, original);
}

// ============================================================================
// Test 4: [bool; 16] fixed array roundtrip via encode_copy
// ============================================================================

#[test]
fn test_copy_fixed_array_bool_16_roundtrip() {
    let original: [bool; 16] = [
        true, false, true, true, false, false, true, false, false, true, true, false, true, false,
        false, true,
    ];
    let bytes = oxicode::encode_copy(original).expect("encode [bool;16]");
    let (decoded, _): ([bool; 16], usize) = decode_from_slice(&bytes).expect("decode [bool;16]");
    assert_eq!(decoded, original);
}

// ============================================================================
// Test 5: [f64; 4] using mathematical constants, no float literals
// ============================================================================

#[test]
fn test_copy_fixed_array_f64_4_constants_roundtrip() {
    use std::f64::consts::{E, LN_2, PI, SQRT_2};
    let original: [f64; 4] = [PI, E, SQRT_2, LN_2];
    let bytes = oxicode::encode_copy(original).expect("encode [f64;4]");
    let (decoded, _): ([f64; 4], usize) = decode_from_slice(&bytes).expect("decode [f64;4]");
    for (a, b) in decoded.iter().zip(original.iter()) {
        assert_eq!(a.to_bits(), b.to_bits(), "f64 bits must match exactly");
    }
}

// ============================================================================
// Test 6: [i32; 8] with negative values roundtrip via encode_copy
// ============================================================================

#[test]
fn test_copy_fixed_array_i32_8_negatives_roundtrip() {
    let original: [i32; 8] = [i32::MIN, -65536, -256, -1, 0, 1, 256, i32::MAX];
    let bytes = oxicode::encode_copy(original).expect("encode [i32;8]");
    let (decoded, _): ([i32; 8], usize) = decode_from_slice(&bytes).expect("decode [i32;8]");
    assert_eq!(decoded, original);
}

// ============================================================================
// Test 7: [u8; 1] single element array roundtrip
// ============================================================================

#[test]
fn test_copy_fixed_array_u8_1_single_element() {
    let original: [u8; 1] = [0xFF];
    let bytes = oxicode::encode_copy(original).expect("encode [u8;1]");
    assert_eq!(bytes.len(), 1, "[u8;1] must encode to exactly 1 byte");
    let (decoded, consumed): ([u8; 1], usize) = decode_from_slice(&bytes).expect("decode [u8;1]");
    assert_eq!(decoded, original);
    assert_eq!(consumed, 1);
}

// ============================================================================
// Test 8: [u8; 255] max-small array roundtrip via encode_copy
// ============================================================================

#[test]
fn test_copy_fixed_array_u8_255_roundtrip() {
    let mut original = [0u8; 255];
    for (i, v) in original.iter_mut().enumerate() {
        *v = (i % 256) as u8;
    }
    let bytes = oxicode::encode_copy(original).expect("encode [u8;255]");
    assert_eq!(
        bytes.len(),
        255,
        "[u8;255] must encode to exactly 255 bytes"
    );
    let (decoded, consumed): ([u8; 255], usize) =
        decode_from_slice(&bytes).expect("decode [u8;255]");
    assert_eq!(decoded, original);
    assert_eq!(consumed, 255);
}

// ============================================================================
// Test 9: [u8; 0] empty array — zero bytes encoded
// ============================================================================

#[test]
fn test_copy_fixed_array_u8_0_empty_encodes_zero_bytes() {
    let original: [u8; 0] = [];
    let bytes = oxicode::encode_copy(original).expect("encode [u8;0]");
    assert_eq!(bytes.len(), 0, "[u8;0] must encode to exactly 0 bytes");
    let (decoded, consumed): ([u8; 0], usize) = decode_from_slice(&bytes).expect("decode [u8;0]");
    assert_eq!(decoded, original);
    assert_eq!(consumed, 0);
}

// ============================================================================
// Test 10: Fixed array with fixed-int encoding config
// ============================================================================

#[test]
fn test_copy_fixed_array_u32_4_with_fixed_int_config() {
    let cfg = config::standard().with_fixed_int_encoding();
    let original: [u32; 4] = [0, 1, 1000, u32::MAX];
    let bytes = encode_to_vec_with_config(&original, cfg).expect("encode [u32;4] fixed_int");
    // Each u32 is 4 bytes in fixed-int mode => total 16 bytes
    assert_eq!(bytes.len(), 16, "[u32;4] fixed_int must encode to 16 bytes");
    let (decoded, _): ([u32; 4], usize) =
        decode_from_slice_with_config(&bytes, cfg).expect("decode [u32;4] fixed_int");
    assert_eq!(decoded, original);
}

// ============================================================================
// Test 11: Fixed array with big-endian encoding config
// ============================================================================

#[test]
fn test_copy_fixed_array_u32_4_with_big_endian_config() {
    let cfg_be = config::standard()
        .with_big_endian()
        .with_fixed_int_encoding();
    let cfg_le = config::standard().with_fixed_int_encoding();
    let original: [u32; 4] = [0x01020304, 0xAABBCCDD, 1, 0];
    let be_bytes = encode_to_vec_with_config(&original, cfg_be).expect("encode [u32;4] big_endian");
    let le_bytes =
        encode_to_vec_with_config(&original, cfg_le).expect("encode [u32;4] little_endian");
    // Big-endian and little-endian bytes must differ for multi-byte values
    assert_ne!(
        be_bytes, le_bytes,
        "big-endian and little-endian must produce different bytes"
    );
    let (decoded, _): ([u32; 4], usize) =
        decode_from_slice_with_config(&be_bytes, cfg_be).expect("decode [u32;4] big_endian");
    assert_eq!(decoded, original);
}

// ============================================================================
// Test 12: Nested fixed arrays [[u8; 4]; 4] roundtrip
// ============================================================================

#[test]
fn test_copy_nested_fixed_arrays_u8_4x4_roundtrip() {
    let original: [[u8; 4]; 4] = [
        [0x00, 0x11, 0x22, 0x33],
        [0x44, 0x55, 0x66, 0x77],
        [0x88, 0x99, 0xAA, 0xBB],
        [0xCC, 0xDD, 0xEE, 0xFF],
    ];
    let bytes = oxicode::encode_copy(original).expect("encode [[u8;4];4]");
    // Inner [u8;4] has no length prefix, outer [_;4] has no length prefix => 16 bytes total
    assert_eq!(
        bytes.len(),
        16,
        "[[u8;4];4] must encode to exactly 16 bytes"
    );
    let (decoded, consumed): ([[u8; 4]; 4], usize) =
        decode_from_slice(&bytes).expect("decode [[u8;4];4]");
    assert_eq!(decoded, original);
    assert_eq!(consumed, 16);
}

// ============================================================================
// Test 13: Tuple of Copy types (u8, u16, u32, u64) roundtrip
// ============================================================================

#[test]
fn test_copy_tuple_u8_u16_u32_u64_roundtrip() {
    let original: (u8, u16, u32, u64) = (0xFF, 0xFFFF, u32::MAX, u64::MAX);
    let bytes = oxicode::encode_copy(original).expect("encode tuple (u8,u16,u32,u64)");
    let (decoded, _): ((u8, u16, u32, u64), usize) =
        decode_from_slice(&bytes).expect("decode tuple (u8,u16,u32,u64)");
    assert_eq!(decoded, original);
}

// ============================================================================
// Test 14: Struct with only Copy fields
// ============================================================================

#[derive(oxicode::Encode, oxicode::Decode, Debug, PartialEq, Clone, Copy)]
struct CopyStruct {
    x: u32,
    y: i32,
    flag: bool,
    val: u8,
}

#[test]
fn test_copy_struct_only_copy_fields_roundtrip() {
    let original = CopyStruct {
        x: 12345,
        y: -9876,
        flag: true,
        val: 0xAB,
    };
    let bytes = oxicode::encode_copy(original).expect("encode CopyStruct");
    let (decoded, consumed): (CopyStruct, usize) =
        decode_from_slice(&bytes).expect("decode CopyStruct");
    assert_eq!(decoded, original);
    assert_eq!(consumed, bytes.len());
}

// ============================================================================
// Test 15: Array of tuples [(u8, u32); 4] roundtrip via encode_copy
// ============================================================================

#[test]
fn test_copy_array_of_tuples_u8_u32_roundtrip() {
    let original: [(u8, u32); 4] = [(0, 0), (1, 100), (127, 50000), (255, u32::MAX)];
    let bytes = oxicode::encode_copy(original).expect("encode [(u8,u32);4]");
    let (decoded, _): ([(u8, u32); 4], usize) =
        decode_from_slice(&bytes).expect("decode [(u8,u32);4]");
    assert_eq!(decoded, original);
}

// ============================================================================
// Test 16: [u32; 4] encoded size = 4 * 4 bytes with fixed-int config
// ============================================================================

#[test]
fn test_copy_fixed_array_u32_4_size_verification_fixed_int() {
    let cfg = config::standard().with_fixed_int_encoding();
    let original: [u32; 4] = [1, 2, 3, 4];
    let bytes = encode_to_vec_with_config(&original, cfg).expect("encode [u32;4] for size check");
    // Each u32 = 4 bytes, 4 elements => 16 bytes, no length prefix for fixed arrays
    assert_eq!(
        bytes.len(),
        16,
        "[u32;4] with fixed_int must be exactly 16 bytes"
    );
}

// ============================================================================
// Test 17: Copy primitives: u8, u16, u32, u64, i8, i16, i32, i64 roundtrip
// ============================================================================

#[test]
fn test_copy_primitives_all_integer_types_roundtrip() {
    // u8
    {
        let v = u8::MAX;
        let b = oxicode::encode_copy(v).expect("encode u8");
        let (d, _): (u8, _) = decode_from_slice(&b).expect("decode u8");
        assert_eq!(d, v);
    }
    // u16
    {
        let v = u16::MAX;
        let b = oxicode::encode_copy(v).expect("encode u16");
        let (d, _): (u16, _) = decode_from_slice(&b).expect("decode u16");
        assert_eq!(d, v);
    }
    // u32
    {
        let v = u32::MAX;
        let b = oxicode::encode_copy(v).expect("encode u32");
        let (d, _): (u32, _) = decode_from_slice(&b).expect("decode u32");
        assert_eq!(d, v);
    }
    // u64
    {
        let v = u64::MAX;
        let b = oxicode::encode_copy(v).expect("encode u64");
        let (d, _): (u64, _) = decode_from_slice(&b).expect("decode u64");
        assert_eq!(d, v);
    }
    // i8
    {
        let v = i8::MIN;
        let b = oxicode::encode_copy(v).expect("encode i8");
        let (d, _): (i8, _) = decode_from_slice(&b).expect("decode i8");
        assert_eq!(d, v);
    }
    // i16
    {
        let v = i16::MIN;
        let b = oxicode::encode_copy(v).expect("encode i16");
        let (d, _): (i16, _) = decode_from_slice(&b).expect("decode i16");
        assert_eq!(d, v);
    }
    // i32
    {
        let v = i32::MIN;
        let b = oxicode::encode_copy(v).expect("encode i32");
        let (d, _): (i32, _) = decode_from_slice(&b).expect("decode i32");
        assert_eq!(d, v);
    }
    // i64
    {
        let v = i64::MIN;
        let b = oxicode::encode_copy(v).expect("encode i64");
        let (d, _): (i64, _) = decode_from_slice(&b).expect("decode i64");
        assert_eq!(d, v);
    }
}

// ============================================================================
// Test 18: [u16; 16] roundtrip via encode_copy
// ============================================================================

#[test]
fn test_copy_fixed_array_u16_16_roundtrip() {
    let mut original = [0u16; 16];
    for (i, v) in original.iter_mut().enumerate() {
        *v = (i as u16) * 1000;
    }
    let bytes = oxicode::encode_copy(original).expect("encode [u16;16]");
    let (decoded, _): ([u16; 16], usize) = decode_from_slice(&bytes).expect("decode [u16;16]");
    assert_eq!(decoded, original);
}

// ============================================================================
// Test 19: [i64; 8] roundtrip via encode_copy
// ============================================================================

#[test]
fn test_copy_fixed_array_i64_8_roundtrip() {
    let original: [i64; 8] = [
        i64::MIN,
        i64::MIN / 2,
        -1,
        0,
        1,
        i64::MAX / 2,
        i64::MAX - 1,
        i64::MAX,
    ];
    let bytes = oxicode::encode_copy(original).expect("encode [i64;8]");
    let (decoded, _): ([i64; 8], usize) = decode_from_slice(&bytes).expect("decode [i64;8]");
    assert_eq!(decoded, original);
}

// ============================================================================
// Test 20: Option<[u8; 4]> Some and None roundtrip
// ============================================================================

#[test]
fn test_copy_option_fixed_array_some_none_roundtrip() {
    // Some variant
    let some_val: Option<[u8; 4]> = Some([0x01, 0x02, 0x03, 0x04]);
    let bytes_some = encode_to_vec(&some_val).expect("encode Option Some [u8;4]");
    let (decoded_some, _): (Option<[u8; 4]>, usize) =
        decode_from_slice(&bytes_some).expect("decode Option Some [u8;4]");
    assert_eq!(decoded_some, some_val);

    // None variant
    let none_val: Option<[u8; 4]> = None;
    let bytes_none = encode_to_vec(&none_val).expect("encode Option None [u8;4]");
    let (decoded_none, _): (Option<[u8; 4]>, usize) =
        decode_from_slice(&bytes_none).expect("decode Option None [u8;4]");
    assert_eq!(decoded_none, none_val);

    // Some and None bytes must differ
    assert_ne!(
        bytes_some, bytes_none,
        "Some and None must encode differently"
    );
}

// ============================================================================
// Test 21: Vec<[u8; 4]> roundtrip
// ============================================================================

#[test]
fn test_copy_vec_of_fixed_arrays_roundtrip() {
    let original: Vec<[u8; 4]> = vec![
        [0x00, 0x01, 0x02, 0x03],
        [0xFF, 0xFE, 0xFD, 0xFC],
        [0x10, 0x20, 0x30, 0x40],
        [0xAA, 0xBB, 0xCC, 0xDD],
    ];
    let bytes = encode_to_vec(&original).expect("encode Vec<[u8;4]>");
    let (decoded, consumed): (Vec<[u8; 4]>, usize) =
        decode_from_slice(&bytes).expect("decode Vec<[u8;4]>");
    assert_eq!(decoded, original);
    assert_eq!(consumed, bytes.len());
}

// ============================================================================
// Test 22: [u32; N] encoded size matches N * fixed_size_of_u32 (fixed_int config)
// ============================================================================

#[test]
fn test_copy_fixed_array_u32_n_encoded_size_formula() {
    let cfg = config::standard().with_fixed_int_encoding();
    // u32 in fixed-int mode is always 4 bytes
    const U32_FIXED_BYTES: usize = 4;

    let arr4: [u32; 4] = [10, 20, 30, 40];
    let b4 = encode_to_vec_with_config(&arr4, cfg).expect("encode [u32;4]");
    assert_eq!(b4.len(), 4 * U32_FIXED_BYTES, "[u32;4] must be 16 bytes");

    let arr8: [u32; 8] = [1, 2, 3, 4, 5, 6, 7, 8];
    let b8 = encode_to_vec_with_config(&arr8, cfg).expect("encode [u32;8]");
    assert_eq!(b8.len(), 8 * U32_FIXED_BYTES, "[u32;8] must be 32 bytes");

    let arr16: [u32; 16] = [0u32; 16];
    let b16 = encode_to_vec_with_config(&arr16, cfg).expect("encode [u32;16]");
    assert_eq!(b16.len(), 16 * U32_FIXED_BYTES, "[u32;16] must be 64 bytes");
}
