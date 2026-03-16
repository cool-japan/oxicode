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

#[test]
fn test_u8_array_empty_roundtrip() {
    let val: [u8; 0] = [];
    let enc = encode_to_vec(&val).expect("encode [u8; 0]");
    let (dec, _): ([u8; 0], usize) = decode_from_slice(&enc).expect("decode [u8; 0]");
    assert_eq!(val, dec);
}

#[test]
fn test_u8_array_single_roundtrip() {
    let val: [u8; 1] = [42];
    let enc = encode_to_vec(&val).expect("encode [u8; 1]");
    let (dec, _): ([u8; 1], usize) = decode_from_slice(&enc).expect("decode [u8; 1]");
    assert_eq!(val, dec);
}

#[test]
fn test_u8_array_4_deadbeef_roundtrip() {
    let val: [u8; 4] = [0xDE, 0xAD, 0xBE, 0xEF];
    let enc = encode_to_vec(&val).expect("encode [u8; 4] deadbeef");
    let (dec, _): ([u8; 4], usize) = decode_from_slice(&enc).expect("decode [u8; 4] deadbeef");
    assert_eq!(val, dec);
}

#[test]
fn test_u8_array_16_roundtrip() {
    let val: [u8; 16] = [0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15];
    let enc = encode_to_vec(&val).expect("encode [u8; 16]");
    let (dec, _): ([u8; 16], usize) = decode_from_slice(&enc).expect("decode [u8; 16]");
    assert_eq!(val, dec);
}

#[test]
fn test_u8_array_32_roundtrip() {
    let val: [u8; 32] = [
        0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24,
        25, 26, 27, 28, 29, 30, 31,
    ];
    let enc = encode_to_vec(&val).expect("encode [u8; 32]");
    let (dec, _): ([u8; 32], usize) = decode_from_slice(&enc).expect("decode [u8; 32]");
    assert_eq!(val, dec);
}

#[test]
fn test_u16_array_4_roundtrip() {
    let val: [u16; 4] = [1000, 2000, 3000, 4000];
    let enc = encode_to_vec(&val).expect("encode [u16; 4]");
    let (dec, _): ([u16; 4], usize) = decode_from_slice(&enc).expect("decode [u16; 4]");
    assert_eq!(val, dec);
}

#[test]
fn test_u32_array_4_roundtrip() {
    let val: [u32; 4] = [100_000, 200_000, 300_000, 400_000];
    let enc = encode_to_vec(&val).expect("encode [u32; 4]");
    let (dec, _): ([u32; 4], usize) = decode_from_slice(&enc).expect("decode [u32; 4]");
    assert_eq!(val, dec);
}

#[test]
fn test_u64_array_4_roundtrip() {
    let val: [u64; 4] = [u64::MAX / 4, u64::MAX / 3, u64::MAX / 2, u64::MAX];
    let enc = encode_to_vec(&val).expect("encode [u64; 4]");
    let (dec, _): ([u64; 4], usize) = decode_from_slice(&enc).expect("decode [u64; 4]");
    assert_eq!(val, dec);
}

#[test]
fn test_i32_array_8_negative_roundtrip() {
    let val: [i32; 8] = [-1, -100, -1000, -10000, 0, 1, 100, 1000];
    let enc = encode_to_vec(&val).expect("encode [i32; 8]");
    let (dec, _): ([i32; 8], usize) = decode_from_slice(&enc).expect("decode [i32; 8]");
    assert_eq!(val, dec);
}

#[test]
fn test_f32_array_4_bit_exact_roundtrip() {
    let val: [f32; 4] = [1.0_f32, -2.5_f32, std::f32::consts::PI, f32::EPSILON];
    let enc = encode_to_vec(&val).expect("encode [f32; 4]");
    let (dec, _): ([f32; 4], usize) = decode_from_slice(&enc).expect("decode [f32; 4]");
    for (a, b) in val.iter().zip(dec.iter()) {
        assert_eq!(a.to_bits(), b.to_bits(), "f32 bit-exact mismatch");
    }
}

#[test]
fn test_f64_array_2_bit_exact_roundtrip() {
    let val: [f64; 2] = [std::f64::consts::E, -std::f64::consts::PI];
    let enc = encode_to_vec(&val).expect("encode [f64; 2]");
    let (dec, _): ([f64; 2], usize) = decode_from_slice(&enc).expect("decode [f64; 2]");
    for (a, b) in val.iter().zip(dec.iter()) {
        assert_eq!(a.to_bits(), b.to_bits(), "f64 bit-exact mismatch");
    }
}

#[test]
fn test_bool_array_8_roundtrip() {
    let val: [bool; 8] = [true, false, true, true, false, false, true, false];
    let enc = encode_to_vec(&val).expect("encode [bool; 8]");
    let (dec, _): ([bool; 8], usize) = decode_from_slice(&enc).expect("decode [bool; 8]");
    assert_eq!(val, dec);
}

#[test]
fn test_u8_array_4_big_endian_fixed_int_roundtrip() {
    let val: [u8; 4] = [0x01, 0x02, 0x03, 0x04];
    let cfg = config::standard()
        .with_big_endian()
        .with_fixed_int_encoding();
    let enc = encode_to_vec_with_config(&val, cfg).expect("encode [u8; 4] big-endian fixed");
    let (dec, _): ([u8; 4], usize) =
        decode_from_slice_with_config(&enc, cfg).expect("decode [u8; 4] big-endian fixed");
    assert_eq!(val, dec);
}

#[test]
fn test_u32_array_4_fixed_int_size_check() {
    let val: [u32; 4] = [1, 2, 3, 4];
    let cfg = config::standard().with_fixed_int_encoding();
    let enc = encode_to_vec_with_config(&val, cfg).expect("encode [u32; 4] fixed int");
    // Each u32 = 4 bytes, so total = 16 bytes
    assert_eq!(enc.len(), 16, "4x u32 fixed should be 16 bytes");
    let (dec, _): ([u32; 4], usize) =
        decode_from_slice_with_config(&enc, cfg).expect("decode [u32; 4] fixed int");
    assert_eq!(val, dec);
}

#[test]
fn test_u8_array_4_consumed_bytes_equals_encoded_len() {
    let val: [u8; 4] = [10, 20, 30, 40];
    let enc = encode_to_vec(&val).expect("encode [u8; 4] consumed");
    let (_dec, consumed): ([u8; 4], usize) =
        decode_from_slice(&enc).expect("decode [u8; 4] consumed");
    assert_eq!(
        consumed,
        enc.len(),
        "consumed bytes should equal encoded length"
    );
}

#[test]
fn test_u8_array_4_and_8_produce_different_lengths() {
    let val4: [u8; 4] = [1, 2, 3, 4];
    let val8: [u8; 8] = [1, 2, 3, 4, 5, 6, 7, 8];
    let enc4 = encode_to_vec(&val4).expect("encode [u8; 4]");
    let enc8 = encode_to_vec(&val8).expect("encode [u8; 8]");
    assert_ne!(
        enc4.len(),
        enc8.len(),
        "[u8; 4] and [u8; 8] should have different encoded lengths"
    );
}

#[test]
fn test_u8_array_4_different_values_produce_different_bytes() {
    let val_a: [u8; 4] = [1, 2, 3, 4];
    let val_b: [u8; 4] = [5, 6, 7, 8];
    let enc_a = encode_to_vec(&val_a).expect("encode val_a");
    let enc_b = encode_to_vec(&val_b).expect("encode val_b");
    assert_ne!(
        enc_a, enc_b,
        "different [u8; 4] values should produce different encodings"
    );
}

#[test]
fn test_nested_u8_array_4x4_roundtrip() {
    let val: [[u8; 4]; 4] = [
        [0x01, 0x02, 0x03, 0x04],
        [0x05, 0x06, 0x07, 0x08],
        [0x09, 0x0A, 0x0B, 0x0C],
        [0x0D, 0x0E, 0x0F, 0x10],
    ];
    let enc = encode_to_vec(&val).expect("encode [[u8; 4]; 4]");
    let (dec, _): ([[u8; 4]; 4], usize) = decode_from_slice(&enc).expect("decode [[u8; 4]; 4]");
    assert_eq!(val, dec);
}

#[test]
fn test_u8_array_64_roundtrip() {
    let val: [u8; 64] = {
        let mut arr = [0u8; 64];
        for (i, x) in arr.iter_mut().enumerate() {
            *x = (i * 4 % 256) as u8;
        }
        arr
    };
    let enc = encode_to_vec(&val).expect("encode [u8; 64]");
    let (dec, _): ([u8; 64], usize) = decode_from_slice(&enc).expect("decode [u8; 64]");
    assert_eq!(val, dec);
}

#[test]
fn test_i64_array_8_min_max_roundtrip() {
    let val: [i64; 8] = [
        i64::MIN,
        i64::MIN / 2,
        -1,
        0,
        1,
        i64::MAX / 2,
        i64::MAX - 1,
        i64::MAX,
    ];
    let enc = encode_to_vec(&val).expect("encode [i64; 8] min/max");
    let (dec, _): ([i64; 8], usize) = decode_from_slice(&enc).expect("decode [i64; 8] min/max");
    assert_eq!(val, dec);
}

#[test]
fn test_vec_of_u8_array_4_roundtrip() {
    let val: Vec<[u8; 4]> = (0u8..10)
        .map(|i| [i * 4, i * 4 + 1, i * 4 + 2, i * 4 + 3])
        .collect();
    let enc = encode_to_vec(&val).expect("encode Vec<[u8; 4]>");
    let (dec, _): (Vec<[u8; 4]>, usize) = decode_from_slice(&enc).expect("decode Vec<[u8; 4]>");
    assert_eq!(val, dec);
}

#[test]
fn test_u128_array_2_roundtrip() {
    let val: [u128; 2] = [u128::MAX, u128::MAX / 2];
    let enc = encode_to_vec(&val).expect("encode [u128; 2]");
    let (dec, _): ([u128; 2], usize) = decode_from_slice(&enc).expect("decode [u128; 2]");
    assert_eq!(val, dec);
}
