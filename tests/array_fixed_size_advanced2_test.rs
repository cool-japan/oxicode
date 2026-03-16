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
    encode_to_vec_with_config, Decode, Encode,
};

// Test 1: [u8; 4] roundtrip with [1, 2, 3, 4]
#[test]
fn test_u8_4_roundtrip_basic() {
    let arr: [u8; 4] = [1, 2, 3, 4];
    let enc = encode_to_vec(&arr).expect("encode [u8;4] basic");
    let (val, _): ([u8; 4], usize) = decode_from_slice(&enc).expect("decode [u8;4] basic");
    assert_eq!(val, arr);
}

// Test 2: [u8; 4] all zeros roundtrip
#[test]
fn test_u8_4_all_zeros() {
    let arr: [u8; 4] = [0u8; 4];
    let enc = encode_to_vec(&arr).expect("encode [u8;4] zeros");
    let (val, _): ([u8; 4], usize) = decode_from_slice(&enc).expect("decode [u8;4] zeros");
    assert_eq!(val, arr);
}

// Test 3: [u8; 4] all 0xFF roundtrip
#[test]
fn test_u8_4_all_ff() {
    let arr: [u8; 4] = [0xFF; 4];
    let enc = encode_to_vec(&arr).expect("encode [u8;4] 0xFF");
    let (val, _): ([u8; 4], usize) = decode_from_slice(&enc).expect("decode [u8;4] 0xFF");
    assert_eq!(val, arr);
}

// Test 4: [u32; 3] roundtrip
#[test]
fn test_u32_3_roundtrip() {
    let arr: [u32; 3] = [100u32, 200u32, 300u32];
    let enc = encode_to_vec(&arr).expect("encode [u32;3]");
    let (val, _): ([u32; 3], usize) = decode_from_slice(&enc).expect("decode [u32;3]");
    assert_eq!(val, arr);
}

// Test 5: [u64; 2] roundtrip
#[test]
fn test_u64_2_roundtrip() {
    let arr: [u64; 2] = [u64::MAX / 2, u64::MAX];
    let enc = encode_to_vec(&arr).expect("encode [u64;2]");
    let (val, _): ([u64; 2], usize) = decode_from_slice(&enc).expect("decode [u64;2]");
    assert_eq!(val, arr);
}

// Test 6: [bool; 5] mixed true/false roundtrip
#[test]
fn test_bool_5_mixed_roundtrip() {
    let arr: [bool; 5] = [true, false, true, true, false];
    let enc = encode_to_vec(&arr).expect("encode [bool;5]");
    let (val, _): ([bool; 5], usize) = decode_from_slice(&enc).expect("decode [bool;5]");
    assert_eq!(val, arr);
}

// Test 7: [u8; 1] single element roundtrip
#[test]
fn test_u8_1_single_element() {
    let arr: [u8; 1] = [42u8];
    let enc = encode_to_vec(&arr).expect("encode [u8;1]");
    let (val, _): ([u8; 1], usize) = decode_from_slice(&enc).expect("decode [u8;1]");
    assert_eq!(val, arr);
}

// Test 8: [u8; 16] size check: encodes as exactly 16 bytes
#[test]
fn test_u8_16_exact_size() {
    let arr: [u8; 16] = [0u8; 16];
    let enc = encode_to_vec(&arr).expect("encode [u8;16]");
    assert_eq!(enc.len(), 16, "[u8; 16] must encode as exactly 16 bytes");
}

// Test 9: [u32; 4] with fixed-int config: exactly 16 bytes
#[test]
fn test_u32_4_fixed_int_config_size() {
    let arr: [u32; 4] = [1u32, 2u32, 3u32, 4u32];
    let cfg = config::standard().with_fixed_int_encoding();
    let enc = encode_to_vec_with_config(&arr, cfg).expect("encode [u32;4] fixed-int");
    assert_eq!(
        enc.len(),
        16,
        "[u32; 4] with fixed-int config must encode as exactly 16 bytes"
    );
    let (val, _): ([u32; 4], usize) =
        decode_from_slice_with_config(&enc, cfg).expect("decode [u32;4] fixed-int");
    assert_eq!(val, arr);
}

// Test 10: [u8; 0] empty array roundtrip: 0 bytes
#[test]
fn test_u8_0_empty_array() {
    let arr: [u8; 0] = [];
    let enc = encode_to_vec(&arr).expect("encode [u8;0]");
    assert_eq!(enc.len(), 0, "[u8; 0] must encode as exactly 0 bytes");
    let (val, _): ([u8; 0], usize) = decode_from_slice(&enc).expect("decode [u8;0]");
    assert_eq!(val, arr);
}

// Test 11: [i8; 4] with negative values roundtrip
#[test]
fn test_i8_4_negative_values() {
    let arr: [i8; 4] = [-1i8, -128i8, 0i8, 127i8];
    let enc = encode_to_vec(&arr).expect("encode [i8;4]");
    let (val, _): ([i8; 4], usize) = decode_from_slice(&enc).expect("decode [i8;4]");
    assert_eq!(val, arr);
}

// Test 12: [f32; 3] roundtrip (bit comparison)
#[test]
fn test_f32_3_roundtrip_bits() {
    let arr: [f32; 3] = [1.5f32, -2.5f32, f32::INFINITY];
    let enc = encode_to_vec(&arr).expect("encode [f32;3]");
    let (val, _): ([f32; 3], usize) = decode_from_slice(&enc).expect("decode [f32;3]");
    for (a, b) in arr.iter().zip(val.iter()) {
        assert_eq!(a.to_bits(), b.to_bits(), "f32 bit mismatch");
    }
}

// Test 13: [f64; 2] roundtrip (bit comparison)
#[test]
fn test_f64_2_roundtrip_bits() {
    let arr: [f64; 2] = [std::f64::consts::PI, -std::f64::consts::E];
    let enc = encode_to_vec(&arr).expect("encode [f64;2]");
    let (val, _): ([f64; 2], usize) = decode_from_slice(&enc).expect("decode [f64;2]");
    for (a, b) in arr.iter().zip(val.iter()) {
        assert_eq!(a.to_bits(), b.to_bits(), "f64 bit mismatch");
    }
}

// Test 14: [[u8; 2]; 3] nested fixed arrays roundtrip
#[test]
fn test_nested_fixed_arrays() {
    let arr: [[u8; 2]; 3] = [[1u8, 2u8], [3u8, 4u8], [5u8, 6u8]];
    let enc = encode_to_vec(&arr).expect("encode [[u8;2];3]");
    let (val, _): ([[u8; 2]; 3], usize) = decode_from_slice(&enc).expect("decode [[u8;2];3]");
    assert_eq!(val, arr);
}

// Test 15: [String; 3] array of strings roundtrip
#[test]
fn test_string_3_roundtrip() {
    let arr: [String; 3] = [
        String::from("hello"),
        String::from("world"),
        String::from("oxicode"),
    ];
    let enc = encode_to_vec(&arr).expect("encode [String;3]");
    let (val, _): ([String; 3], usize) = decode_from_slice(&enc).expect("decode [String;3]");
    assert_eq!(val, arr);
}

// Test 16: [u8; 8] matches exactly 8 bytes encoded
#[test]
fn test_u8_8_exact_size() {
    let arr: [u8; 8] = [10u8, 20u8, 30u8, 40u8, 50u8, 60u8, 70u8, 80u8];
    let enc = encode_to_vec(&arr).expect("encode [u8;8]");
    assert_eq!(enc.len(), 8, "[u8; 8] must encode as exactly 8 bytes");
    let (val, _): ([u8; 8], usize) = decode_from_slice(&enc).expect("decode [u8;8]");
    assert_eq!(val, arr);
}

// Test 17: Vec<[u8; 4]> — vec of fixed arrays roundtrip
#[test]
fn test_vec_of_fixed_arrays() {
    let v: Vec<[u8; 4]> = vec![
        [1u8, 2u8, 3u8, 4u8],
        [5u8, 6u8, 7u8, 8u8],
        [9u8, 10u8, 11u8, 12u8],
    ];
    let enc = encode_to_vec(&v).expect("encode Vec<[u8;4]>");
    let (val, _): (Vec<[u8; 4]>, usize) = decode_from_slice(&enc).expect("decode Vec<[u8;4]>");
    assert_eq!(val, v);
}

// Test 18: Option<[u8; 4]> Some roundtrip
#[test]
fn test_option_fixed_array_some() {
    let opt: Option<[u8; 4]> = Some([1u8, 2u8, 3u8, 4u8]);
    let enc = encode_to_vec(&opt).expect("encode Option<[u8;4]> Some");
    let (val, _): (Option<[u8; 4]>, usize) =
        decode_from_slice(&enc).expect("decode Option<[u8;4]> Some");
    assert_eq!(val, opt);
}

// Test 19: Option<[u8; 4]> None roundtrip
#[test]
fn test_option_fixed_array_none() {
    let opt: Option<[u8; 4]> = None;
    let enc = encode_to_vec(&opt).expect("encode Option<[u8;4]> None");
    let (val, _): (Option<[u8; 4]>, usize) =
        decode_from_slice(&enc).expect("decode Option<[u8;4]> None");
    assert_eq!(val, opt);
}

// Test 20: Struct containing [u8; 16] field roundtrip
#[derive(Debug, PartialEq, Encode, Decode)]
struct KeyBlock {
    key: [u8; 16],
    id: u32,
}

#[test]
fn test_struct_with_fixed_array_field() {
    let block = KeyBlock {
        key: [
            0x00u8, 0x01u8, 0x02u8, 0x03u8, 0x04u8, 0x05u8, 0x06u8, 0x07u8, 0x08u8, 0x09u8, 0x0Au8,
            0x0Bu8, 0x0Cu8, 0x0Du8, 0x0Eu8, 0x0Fu8,
        ],
        id: 42u32,
    };
    let enc = encode_to_vec(&block).expect("encode KeyBlock");
    let (val, _): (KeyBlock, usize) = decode_from_slice(&enc).expect("decode KeyBlock");
    assert_eq!(val, block);
}

// Test 21: Big-endian + fixed-int with [u32; 2]: verify byte order
#[test]
fn test_u32_2_big_endian_fixed_int_byte_order() {
    let arr: [u32; 2] = [0x01020304u32, 0xDEADBEEFu32];
    let cfg = config::standard()
        .with_fixed_int_encoding()
        .with_big_endian();
    let enc = encode_to_vec_with_config(&arr, cfg).expect("encode [u32;2] big-endian");
    // Each u32 = 4 bytes, total = 8 bytes
    assert_eq!(
        enc.len(),
        8,
        "[u32; 2] big-endian fixed-int must be 8 bytes"
    );
    // First u32 = 0x01020304 in big-endian: [0x01, 0x02, 0x03, 0x04]
    assert_eq!(enc[0], 0x01u8, "big-endian byte 0");
    assert_eq!(enc[1], 0x02u8, "big-endian byte 1");
    assert_eq!(enc[2], 0x03u8, "big-endian byte 2");
    assert_eq!(enc[3], 0x04u8, "big-endian byte 3");
    // Decode and verify roundtrip
    let (val, _): ([u32; 2], usize) =
        decode_from_slice_with_config(&enc, cfg).expect("decode [u32;2] big-endian");
    assert_eq!(val, arr);
}

// Test 22: [u8; 256] large array roundtrip
#[test]
fn test_u8_256_large_array() {
    let mut arr = [0u8; 256];
    for (i, elem) in arr.iter_mut().enumerate() {
        *elem = (i % 256) as u8;
    }
    let enc = encode_to_vec(&arr).expect("encode [u8;256]");
    assert_eq!(enc.len(), 256, "[u8; 256] must encode as exactly 256 bytes");
    let (val, _): ([u8; 256], usize) = decode_from_slice(&enc).expect("decode [u8;256]");
    assert_eq!(val, arr);
}
