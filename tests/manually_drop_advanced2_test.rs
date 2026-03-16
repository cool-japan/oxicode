//! Advanced tests for ManuallyDrop<T> encoding in OxiCode.
//! ManuallyDrop<T> is a transparent wrapper — encodes identically to T.

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
use std::mem::ManuallyDrop;

// Test 1: ManuallyDrop<u32> roundtrip value=42
#[test]
fn test_manually_drop_u32_roundtrip_42() {
    let inner: u32 = 42;
    let val = ManuallyDrop::new(inner);
    let enc = encode_to_vec(&*val).expect("encode ManuallyDrop<u32> value=42");
    let (decoded, _): (u32, usize) =
        decode_from_slice(&enc).expect("decode ManuallyDrop<u32> value=42");
    assert_eq!(*val, decoded);
}

// Test 2: ManuallyDrop<u32> roundtrip value=0
#[test]
fn test_manually_drop_u32_roundtrip_zero() {
    let inner: u32 = 0;
    let val = ManuallyDrop::new(inner);
    let enc = encode_to_vec(&*val).expect("encode ManuallyDrop<u32> value=0");
    let (decoded, _): (u32, usize) =
        decode_from_slice(&enc).expect("decode ManuallyDrop<u32> value=0");
    assert_eq!(*val, decoded);
}

// Test 3: ManuallyDrop<u32> roundtrip value=u32::MAX
#[test]
fn test_manually_drop_u32_roundtrip_max() {
    let inner: u32 = u32::MAX;
    let val = ManuallyDrop::new(inner);
    let enc = encode_to_vec(&*val).expect("encode ManuallyDrop<u32> value=u32::MAX");
    let (decoded, _): (u32, usize) =
        decode_from_slice(&enc).expect("decode ManuallyDrop<u32> value=u32::MAX");
    assert_eq!(*val, decoded);
}

// Test 4: ManuallyDrop<u64> roundtrip
#[test]
fn test_manually_drop_u64_roundtrip() {
    let inner: u64 = 1_000_000_000_000u64;
    let val = ManuallyDrop::new(inner);
    let enc = encode_to_vec(&*val).expect("encode ManuallyDrop<u64>");
    let (decoded, _): (u64, usize) = decode_from_slice(&enc).expect("decode ManuallyDrop<u64>");
    assert_eq!(*val, decoded);
}

// Test 5: ManuallyDrop<u64> roundtrip u64::MAX
#[test]
fn test_manually_drop_u64_roundtrip_max() {
    let inner: u64 = u64::MAX;
    let val = ManuallyDrop::new(inner);
    let enc = encode_to_vec(&*val).expect("encode ManuallyDrop<u64> value=u64::MAX");
    let (decoded, _): (u64, usize) =
        decode_from_slice(&enc).expect("decode ManuallyDrop<u64> value=u64::MAX");
    assert_eq!(*val, decoded);
}

// Test 6: ManuallyDrop<bool> roundtrip true
#[test]
fn test_manually_drop_bool_roundtrip_true() {
    let inner: bool = true;
    let val = ManuallyDrop::new(inner);
    let enc = encode_to_vec(&*val).expect("encode ManuallyDrop<bool> true");
    let (decoded, _): (bool, usize) =
        decode_from_slice(&enc).expect("decode ManuallyDrop<bool> true");
    assert_eq!(*val, decoded);
}

// Test 7: ManuallyDrop<bool> roundtrip false
#[test]
fn test_manually_drop_bool_roundtrip_false() {
    let inner: bool = false;
    let val = ManuallyDrop::new(inner);
    let enc = encode_to_vec(&*val).expect("encode ManuallyDrop<bool> false");
    let (decoded, _): (bool, usize) =
        decode_from_slice(&enc).expect("decode ManuallyDrop<bool> false");
    assert_eq!(*val, decoded);
}

// Test 8: ManuallyDrop<u8> wire bytes equal raw u8
#[test]
fn test_manually_drop_u8_wire_bytes_equal_raw() {
    let inner: u8 = 0xAB;
    let val = ManuallyDrop::new(inner);
    let enc_via_drop = encode_to_vec(&*val).expect("encode ManuallyDrop<u8> wire bytes");
    let enc_raw = encode_to_vec(&inner).expect("encode raw u8 wire bytes");
    assert_eq!(
        enc_via_drop, enc_raw,
        "ManuallyDrop<u8> must encode identically to raw u8"
    );
}

// Test 9: ManuallyDrop<u32> wire bytes equal raw u32
#[test]
fn test_manually_drop_u32_wire_bytes_equal_raw() {
    let inner: u32 = 0xDEAD_BEEF;
    let val = ManuallyDrop::new(inner);
    let enc_via_drop = encode_to_vec(&*val).expect("encode ManuallyDrop<u32> wire bytes");
    let enc_raw = encode_to_vec(&inner).expect("encode raw u32 wire bytes");
    assert_eq!(
        enc_via_drop, enc_raw,
        "ManuallyDrop<u32> must encode identically to raw u32"
    );
}

// Test 10: ManuallyDrop<u64> wire bytes equal raw u64
#[test]
fn test_manually_drop_u64_wire_bytes_equal_raw() {
    let inner: u64 = 0xCAFE_BABE_DEAD_BEEFu64;
    let val = ManuallyDrop::new(inner);
    let enc_via_drop = encode_to_vec(&*val).expect("encode ManuallyDrop<u64> wire bytes");
    let enc_raw = encode_to_vec(&inner).expect("encode raw u64 wire bytes");
    assert_eq!(
        enc_via_drop, enc_raw,
        "ManuallyDrop<u64> must encode identically to raw u64"
    );
}

// Test 11: ManuallyDrop<i32> roundtrip negative
#[test]
fn test_manually_drop_i32_roundtrip_negative() {
    let inner: i32 = -12345;
    let val = ManuallyDrop::new(inner);
    let enc = encode_to_vec(&*val).expect("encode ManuallyDrop<i32> negative");
    let (decoded, _): (i32, usize) =
        decode_from_slice(&enc).expect("decode ManuallyDrop<i32> negative");
    assert_eq!(*val, decoded);
}

// Test 12: ManuallyDrop<i64> roundtrip i64::MIN
#[test]
fn test_manually_drop_i64_roundtrip_min() {
    let inner: i64 = i64::MIN;
    let val = ManuallyDrop::new(inner);
    let enc = encode_to_vec(&*val).expect("encode ManuallyDrop<i64> i64::MIN");
    let (decoded, _): (i64, usize) =
        decode_from_slice(&enc).expect("decode ManuallyDrop<i64> i64::MIN");
    assert_eq!(*val, decoded);
}

// Test 13: ManuallyDrop<u32> consumed bytes equals encoded len
#[test]
fn test_manually_drop_u32_consumed_bytes_eq_encoded_len() {
    let inner: u32 = 999;
    let val = ManuallyDrop::new(inner);
    let enc = encode_to_vec(&*val).expect("encode ManuallyDrop<u32> consumed bytes");
    let (_, consumed): (u32, usize) =
        decode_from_slice(&enc).expect("decode ManuallyDrop<u32> consumed bytes");
    assert_eq!(
        consumed,
        enc.len(),
        "consumed bytes must equal encoded length"
    );
}

// Test 14: ManuallyDrop<u64> consumed bytes equals encoded len
#[test]
fn test_manually_drop_u64_consumed_bytes_eq_encoded_len() {
    let inner: u64 = 123_456_789_012u64;
    let val = ManuallyDrop::new(inner);
    let enc = encode_to_vec(&*val).expect("encode ManuallyDrop<u64> consumed bytes");
    let (_, consumed): (u64, usize) =
        decode_from_slice(&enc).expect("decode ManuallyDrop<u64> consumed bytes");
    assert_eq!(
        consumed,
        enc.len(),
        "consumed bytes must equal encoded length for u64"
    );
}

// Test 15: ManuallyDrop<u32> fixed int config roundtrip (4 bytes)
#[test]
fn test_manually_drop_u32_fixed_int_config_roundtrip() {
    let inner: u32 = 0x0A0B0C0D;
    let val = ManuallyDrop::new(inner);
    let cfg = config::standard().with_fixed_int_encoding();
    let enc =
        encode_to_vec_with_config(&*val, cfg).expect("encode ManuallyDrop<u32> fixed int config");
    assert_eq!(enc.len(), 4, "fixed int u32 must be exactly 4 bytes");
    let (decoded, _): (u32, usize) = decode_from_slice_with_config(&enc, cfg)
        .expect("decode ManuallyDrop<u32> fixed int config");
    assert_eq!(*val, decoded);
}

// Test 16: ManuallyDrop<u64> fixed int config (8 bytes)
#[test]
fn test_manually_drop_u64_fixed_int_config_8_bytes() {
    let inner: u64 = 0x0102030405060708u64;
    let val = ManuallyDrop::new(inner);
    let cfg = config::standard().with_fixed_int_encoding();
    let enc =
        encode_to_vec_with_config(&*val, cfg).expect("encode ManuallyDrop<u64> fixed int config");
    assert_eq!(enc.len(), 8, "fixed int u64 must be exactly 8 bytes");
    let (decoded, _): (u64, usize) = decode_from_slice_with_config(&enc, cfg)
        .expect("decode ManuallyDrop<u64> fixed int config");
    assert_eq!(*val, decoded);
}

// Test 17: ManuallyDrop<u128> roundtrip
#[test]
fn test_manually_drop_u128_roundtrip() {
    let inner: u128 = 0xDEAD_BEEF_CAFE_BABE_0102_0304_0506_0708u128;
    let val = ManuallyDrop::new(inner);
    let enc = encode_to_vec(&*val).expect("encode ManuallyDrop<u128>");
    let (decoded, _): (u128, usize) = decode_from_slice(&enc).expect("decode ManuallyDrop<u128>");
    assert_eq!(*val, decoded);
}

// Test 18: Vec decode after ManuallyDrop<Vec<u8>> — get inner by borrow
#[test]
fn test_manually_drop_vec_u8_roundtrip_via_borrow() {
    let inner: Vec<u8> = vec![0x01, 0x02, 0x03, 0xFF, 0xAB];
    let val = ManuallyDrop::new(inner);
    // Borrow the inner Vec<u8> via Deref (single deref to get &Vec<u8>, not &[u8])
    let enc = encode_to_vec(&*val).expect("encode ManuallyDrop<Vec<u8>> inner");
    let (decoded, _): (Vec<u8>, usize) =
        decode_from_slice(&enc).expect("decode ManuallyDrop<Vec<u8>> inner");
    assert_eq!(
        &*val as &Vec<u8>, &decoded,
        "decoded Vec<u8> must match original inner"
    );
}

// Test 19: ManuallyDrop<u32> big-endian fixed int config roundtrip (value 0x01020304)
#[test]
fn test_manually_drop_u32_big_endian_fixed_int_roundtrip() {
    let inner: u32 = 0x01020304u32;
    let val = ManuallyDrop::new(inner);
    let cfg = config::standard()
        .with_fixed_int_encoding()
        .with_big_endian();
    let enc =
        encode_to_vec_with_config(&*val, cfg).expect("encode ManuallyDrop<u32> big-endian fixed");
    assert_eq!(
        enc.len(),
        4,
        "big-endian fixed int u32 must be exactly 4 bytes"
    );
    assert_eq!(enc[0], 0x01, "big-endian: most significant byte first");
    assert_eq!(enc[1], 0x02);
    assert_eq!(enc[2], 0x03);
    assert_eq!(enc[3], 0x04);
    let (decoded, _): (u32, usize) = decode_from_slice_with_config(&enc, cfg)
        .expect("decode ManuallyDrop<u32> big-endian fixed");
    assert_eq!(*val, decoded);
}

// Test 20: ManuallyDrop<f64> roundtrip (bit-exact via to_bits())
#[test]
fn test_manually_drop_f64_roundtrip_bit_exact() {
    let inner: f64 = std::f64::consts::PI;
    let val = ManuallyDrop::new(inner);
    let enc = encode_to_vec(&*val).expect("encode ManuallyDrop<f64>");
    let (decoded, _): (f64, usize) = decode_from_slice(&enc).expect("decode ManuallyDrop<f64>");
    assert_eq!(
        (*val).to_bits(),
        decoded.to_bits(),
        "ManuallyDrop<f64> roundtrip must be bit-exact"
    );
}

// Test 21: Multiple ManuallyDrop<u32> values produce different encodings for different values
#[test]
fn test_manually_drop_u32_different_values_different_encodings() {
    let val_a = ManuallyDrop::new(1u32);
    let val_b = ManuallyDrop::new(2u32);
    let val_c = ManuallyDrop::new(1000u32);
    let enc_a = encode_to_vec(&*val_a).expect("encode ManuallyDrop<u32> value=1");
    let enc_b = encode_to_vec(&*val_b).expect("encode ManuallyDrop<u32> value=2");
    let enc_c = encode_to_vec(&*val_c).expect("encode ManuallyDrop<u32> value=1000");
    assert_ne!(enc_a, enc_b, "encodings of 1 and 2 must differ");
    assert_ne!(enc_a, enc_c, "encodings of 1 and 1000 must differ");
    assert_ne!(enc_b, enc_c, "encodings of 2 and 1000 must differ");
}

// Test 22: ManuallyDrop<u16> roundtrip
#[test]
fn test_manually_drop_u16_roundtrip() {
    let inner: u16 = 0xBEEF;
    let val = ManuallyDrop::new(inner);
    let enc = encode_to_vec(&*val).expect("encode ManuallyDrop<u16>");
    let (decoded, _): (u16, usize) = decode_from_slice(&enc).expect("decode ManuallyDrop<u16>");
    assert_eq!(*val, decoded);
}
