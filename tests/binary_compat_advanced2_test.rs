//! Binary compatibility tests for OxiCode wire format.
//!
//! These 22 tests verify that hardcoded byte sequences always decode to known values,
//! ensuring the wire format never accidentally changes between versions.

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
use oxicode::{config, decode_from_slice, encode_to_vec, Decode, Encode};

// Helper: encode with custom config
fn encode_with<E: Encode, C: config::Config>(val: &E, cfg: C) -> Vec<u8> {
    oxicode::encode_to_vec_with_config(val, cfg).expect("encode_with_config")
}

// ---------------------------------------------------------------------------
// Test 1: u8(0) → [0x00]
// ---------------------------------------------------------------------------
#[test]
fn test_compat_u8_zero() {
    let enc = encode_to_vec(&0u8).expect("encode u8 zero");
    assert_eq!(enc, vec![0x00]);
    let (val, _): (u8, usize) = decode_from_slice(&enc).expect("decode u8 zero");
    assert_eq!(val, 0u8);
}

// ---------------------------------------------------------------------------
// Test 2: u8(1) → [0x01]
// ---------------------------------------------------------------------------
#[test]
fn test_compat_u8_one() {
    let enc = encode_to_vec(&1u8).expect("encode u8 one");
    assert_eq!(enc, vec![0x01]);
    let (val, _): (u8, usize) = decode_from_slice(&enc).expect("decode u8 one");
    assert_eq!(val, 1u8);
}

// ---------------------------------------------------------------------------
// Test 3: u8(255) → [0xFF]
// ---------------------------------------------------------------------------
#[test]
fn test_compat_u8_max() {
    let enc = encode_to_vec(&255u8).expect("encode u8 max");
    assert_eq!(enc, vec![0xFF]);
    let (val, _): (u8, usize) = decode_from_slice(&enc).expect("decode u8 max");
    assert_eq!(val, 255u8);
}

// ---------------------------------------------------------------------------
// Test 4: u32(0) → [0x00] (varint zero is single byte)
// ---------------------------------------------------------------------------
#[test]
fn test_compat_u32_zero() {
    let enc = encode_to_vec(&0u32).expect("encode u32 zero");
    assert_eq!(enc, vec![0x00]);
    let (val, _): (u32, usize) = decode_from_slice(&enc).expect("decode u32 zero");
    assert_eq!(val, 0u32);
}

// ---------------------------------------------------------------------------
// Test 5: u32(250) → [0xFA]  (max 1-byte varint)
// ---------------------------------------------------------------------------
#[test]
fn test_compat_u32_250_max_single_byte_varint() {
    let enc = encode_to_vec(&250u32).expect("encode u32 250");
    assert_eq!(enc, vec![0xFA]);
    let (val, _): (u32, usize) = decode_from_slice(&enc).expect("decode u32 250");
    assert_eq!(val, 250u32);
}

// ---------------------------------------------------------------------------
// Test 6: bool(false) → [0x00]
// ---------------------------------------------------------------------------
#[test]
fn test_compat_bool_false() {
    let enc = encode_to_vec(&false).expect("encode false");
    assert_eq!(enc, vec![0x00]);
    let (val, _): (bool, usize) = decode_from_slice(&enc).expect("decode false");
    assert!(!val);
}

// ---------------------------------------------------------------------------
// Test 7: bool(true) → [0x01]
// ---------------------------------------------------------------------------
#[test]
fn test_compat_bool_true() {
    let enc = encode_to_vec(&true).expect("encode true");
    assert_eq!(enc, vec![0x01]);
    let (val, _): (bool, usize) = decode_from_slice(&enc).expect("decode true");
    assert!(val);
}

// ---------------------------------------------------------------------------
// Test 8: &str("") → [0x00]  (empty string = length 0)
// ---------------------------------------------------------------------------
#[test]
fn test_compat_empty_string() {
    let enc = encode_to_vec(&"").expect("encode empty string");
    assert_eq!(enc, vec![0x00]);
    let (val, _): (String, usize) = decode_from_slice(&enc).expect("decode empty string");
    assert_eq!(val, "");
}

// ---------------------------------------------------------------------------
// Test 9: &str("a") → [0x01, 0x61]  (length 1, ASCII 'a')
// ---------------------------------------------------------------------------
#[test]
fn test_compat_string_a() {
    let enc = encode_to_vec(&"a").expect("encode string a");
    assert_eq!(enc, vec![0x01, 0x61]);
    let (val, _): (String, usize) = decode_from_slice(&enc).expect("decode string a");
    assert_eq!(val, "a");
}

// ---------------------------------------------------------------------------
// Test 10: Option::<u32>::None → [0x00]
// ---------------------------------------------------------------------------
#[test]
fn test_compat_option_none() {
    let enc = encode_to_vec(&None::<u32>).expect("encode None");
    assert_eq!(enc, vec![0x00]);
    let (val, _): (Option<u32>, usize) = decode_from_slice(&enc).expect("decode None");
    assert_eq!(val, None);
}

// ---------------------------------------------------------------------------
// Test 11: Some(0u8) → [0x01, 0x00]
// ---------------------------------------------------------------------------
#[test]
fn test_compat_option_some_zero_u8() {
    let enc = encode_to_vec(&Some(0u8)).expect("encode Some(0u8)");
    assert_eq!(enc, vec![0x01, 0x00]);
    let (val, _): (Option<u8>, usize) = decode_from_slice(&enc).expect("decode Some(0u8)");
    assert_eq!(val, Some(0u8));
}

// ---------------------------------------------------------------------------
// Test 12: vec![0u8, 1u8, 2u8] starts with [0x03] (length prefix = 3)
// ---------------------------------------------------------------------------
#[test]
fn test_compat_vec_length_prefix() {
    let enc = encode_to_vec(&vec![0u8, 1u8, 2u8]).expect("encode vec");
    // Length 3 encoded as varint = [0x03]
    assert_eq!(enc[0], 0x03);
    // Full wire: [0x03, 0x00, 0x01, 0x02]
    assert_eq!(enc, vec![0x03, 0x00, 0x01, 0x02]);
    let (val, _): (Vec<u8>, usize) = decode_from_slice(&enc).expect("decode vec");
    assert_eq!(val, vec![0u8, 1u8, 2u8]);
}

// ---------------------------------------------------------------------------
// Test 13: Unit struct encodes to [] (0 bytes)
// ---------------------------------------------------------------------------
#[derive(Encode, Decode, PartialEq, Debug)]
struct UnitStruct;

#[test]
fn test_compat_unit_struct_zero_bytes() {
    let enc = encode_to_vec(&UnitStruct).expect("encode UnitStruct");
    assert_eq!(enc, Vec::<u8>::new());
    let (val, consumed): (UnitStruct, usize) = decode_from_slice(&enc).expect("decode UnitStruct");
    assert_eq!(val, UnitStruct);
    assert_eq!(consumed, 0);
}

// ---------------------------------------------------------------------------
// Test 14: Fixed-int LE config: u32(1) → [0x01, 0x00, 0x00, 0x00]
// ---------------------------------------------------------------------------
#[test]
fn test_compat_fixed_int_le_u32_one() {
    let cfg = config::standard().with_fixed_int_encoding();
    let enc = encode_with(&1u32, cfg);
    assert_eq!(enc, vec![0x01, 0x00, 0x00, 0x00]);
    let (val, _): (u32, usize) =
        oxicode::decode_from_slice_with_config(&enc, cfg).expect("decode fixed int le u32");
    assert_eq!(val, 1u32);
}

// ---------------------------------------------------------------------------
// Test 15: Big-endian fixed-int: u32(1) → [0x00, 0x00, 0x00, 0x01]
// ---------------------------------------------------------------------------
#[test]
fn test_compat_fixed_int_be_u32_one() {
    let cfg = config::standard()
        .with_fixed_int_encoding()
        .with_big_endian();
    let enc = encode_with(&1u32, cfg);
    assert_eq!(enc, vec![0x00, 0x00, 0x00, 0x01]);
    let (val, _): (u32, usize) =
        oxicode::decode_from_slice_with_config(&enc, cfg).expect("decode fixed int be u32");
    assert_eq!(val, 1u32);
}

// ---------------------------------------------------------------------------
// Test 16: i8(-1) encodes as [0xFF] (raw byte, no zigzag)
// ---------------------------------------------------------------------------
#[test]
fn test_compat_i8_neg1_raw_byte() {
    let enc = encode_to_vec(&(-1i8)).expect("encode i8 -1");
    assert_eq!(enc, vec![0xFF]);
    let (val, _): (i8, usize) = decode_from_slice(&enc).expect("decode i8 -1");
    assert_eq!(val, -1i8);
}

// ---------------------------------------------------------------------------
// Test 17: decode_from_slice::<u8>(&[0x42]) == 66 decimal
// ---------------------------------------------------------------------------
#[test]
fn test_compat_decode_u8_0x42() {
    let (val, _): (u8, usize) = decode_from_slice(&[0x42]).expect("decode 0x42 as u8");
    assert_eq!(val, 0x42u8);
    assert_eq!(val, 66u8);
}

// ---------------------------------------------------------------------------
// Test 18: decode_from_slice::<bool>(&[0x00]) == false
// ---------------------------------------------------------------------------
#[test]
fn test_compat_decode_bool_false_from_bytes() {
    let (val, _): (bool, usize) = decode_from_slice(&[0x00]).expect("decode bool false");
    assert!(!val);
}

// ---------------------------------------------------------------------------
// Test 19: decode_from_slice::<bool>(&[0x01]) == true
// ---------------------------------------------------------------------------
#[test]
fn test_compat_decode_bool_true_from_bytes() {
    let (val, _): (bool, usize) = decode_from_slice(&[0x01]).expect("decode bool true");
    assert!(val);
}

// ---------------------------------------------------------------------------
// Test 20: [0x05, 0x68, 0x65, 0x6C, 0x6C, 0x6F] decodes to "hello"
// ---------------------------------------------------------------------------
#[test]
fn test_compat_decode_hello_bytes() {
    let bytes = [0x05u8, 0x68, 0x65, 0x6C, 0x6C, 0x6F];
    let (val, _): (String, usize) = decode_from_slice(&bytes).expect("decode hello");
    assert_eq!(val, "hello");
    // Also verify round-trip consistency
    let enc = encode_to_vec(&"hello").expect("encode hello");
    assert_eq!(enc, bytes.to_vec());
}

// ---------------------------------------------------------------------------
// Test 21: legacy config (fixed int) encodes u32(1) differently from standard (varint)
// ---------------------------------------------------------------------------
#[test]
fn test_compat_legacy_vs_standard_config_differ() {
    let standard_enc = encode_to_vec(&1u32).expect("encode standard");
    let legacy_cfg = config::legacy();
    let legacy_enc = encode_with(&1u32, legacy_cfg);
    // standard (varint): [0x01] — 1 byte
    assert_eq!(standard_enc, vec![0x01]);
    // legacy (fixed int, LE): [0x01, 0x00, 0x00, 0x00] — 4 bytes
    assert_eq!(legacy_enc, vec![0x01, 0x00, 0x00, 0x00]);
    assert_ne!(standard_enc, legacy_enc);
}

// ---------------------------------------------------------------------------
// Test 22: varint zero [0x00] decodes as u8=0, u16=0, u32=0, u64=0
// ---------------------------------------------------------------------------
#[test]
fn test_compat_varint_zero_decodes_all_integer_types() {
    let zero_byte = [0x00u8];

    let (v8, _): (u8, usize) = decode_from_slice(&zero_byte).expect("decode u8 zero");
    assert_eq!(v8, 0u8);

    let (v16, _): (u16, usize) = decode_from_slice(&zero_byte).expect("decode u16 zero");
    assert_eq!(v16, 0u16);

    let (v32, _): (u32, usize) = decode_from_slice(&zero_byte).expect("decode u32 zero");
    assert_eq!(v32, 0u32);

    let (v64, _): (u64, usize) = decode_from_slice(&zero_byte).expect("decode u64 zero");
    assert_eq!(v64, 0u64);
}
