//! Advanced atomic integer serialization tests for OxiCode.
//!
//! Tests cover encode/decode roundtrips, wire-byte compatibility between atomic and raw integer
//! types, config variations (fixed-int, big-endian), and boundary values.

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
use std::sync::atomic::{
    AtomicBool, AtomicI16, AtomicI32, AtomicI64, AtomicI8, AtomicU16, AtomicU32, AtomicU64,
    AtomicU8, Ordering,
};

// --- Test 1: AtomicU32 encode then decode as u32 roundtrip value=42 ---

#[cfg(target_has_atomic = "32")]
#[test]
fn test_atomic_u32_decode_as_u32_value_42() {
    let original = AtomicU32::new(42);
    let enc = encode_to_vec(&original).expect("encode AtomicU32(42)");
    let (dec, _consumed): (u32, usize) = decode_from_slice(&enc).expect("decode AtomicU32 as u32");
    assert_eq!(dec, original.load(Ordering::Relaxed));
}

// --- Test 2: AtomicU32 same wire bytes as raw u32 ---

#[cfg(target_has_atomic = "32")]
#[test]
fn test_atomic_u32_same_wire_bytes_as_raw_u32() {
    let value: u32 = 12345;
    let atomic_enc = encode_to_vec(&AtomicU32::new(value)).expect("encode AtomicU32");
    let raw_enc = encode_to_vec(&value).expect("encode u32");
    assert_eq!(
        atomic_enc, raw_enc,
        "AtomicU32 and u32 must produce identical wire bytes"
    );
}

// --- Test 3: AtomicBool encode then decode as bool — true ---

#[cfg(target_has_atomic = "8")]
#[test]
fn test_atomic_bool_encode_decode_true() {
    let original = AtomicBool::new(true);
    let enc = encode_to_vec(&original).expect("encode AtomicBool(true)");
    let (dec, _consumed): (bool, usize) =
        decode_from_slice(&enc).expect("decode AtomicBool as bool (true)");
    assert!(dec, "decoded bool should be true");
}

// --- Test 4: AtomicBool encode then decode as bool — false ---

#[cfg(target_has_atomic = "8")]
#[test]
fn test_atomic_bool_encode_decode_false() {
    let original = AtomicBool::new(false);
    let enc = encode_to_vec(&original).expect("encode AtomicBool(false)");
    let (dec, _consumed): (bool, usize) =
        decode_from_slice(&enc).expect("decode AtomicBool as bool (false)");
    assert!(!dec, "decoded bool should be false");
}

// --- Test 5: AtomicU8 roundtrip value=100 ---

#[cfg(target_has_atomic = "8")]
#[test]
fn test_atomic_u8_roundtrip_100() {
    let original = AtomicU8::new(100);
    let enc = encode_to_vec(&original).expect("encode AtomicU8(100)");
    let (dec, _consumed): (u8, usize) = decode_from_slice(&enc).expect("decode AtomicU8 as u8");
    assert_eq!(dec, original.load(Ordering::Relaxed));
}

// --- Test 6: AtomicU8 roundtrip value=0 ---

#[cfg(target_has_atomic = "8")]
#[test]
fn test_atomic_u8_roundtrip_zero() {
    let original = AtomicU8::new(0);
    let enc = encode_to_vec(&original).expect("encode AtomicU8(0)");
    let (dec, _consumed): (u8, usize) = decode_from_slice(&enc).expect("decode AtomicU8(0) as u8");
    assert_eq!(dec, 0u8);
}

// --- Test 7: AtomicU8 roundtrip value=255 ---

#[cfg(target_has_atomic = "8")]
#[test]
fn test_atomic_u8_roundtrip_max() {
    let original = AtomicU8::new(255);
    let enc = encode_to_vec(&original).expect("encode AtomicU8(255)");
    let (dec, _consumed): (u8, usize) =
        decode_from_slice(&enc).expect("decode AtomicU8(255) as u8");
    assert_eq!(dec, 255u8);
}

// --- Test 8: AtomicU16 roundtrip value=1000 ---

#[cfg(target_has_atomic = "16")]
#[test]
fn test_atomic_u16_roundtrip_1000() {
    let original = AtomicU16::new(1000);
    let enc = encode_to_vec(&original).expect("encode AtomicU16(1000)");
    let (dec, _consumed): (u16, usize) = decode_from_slice(&enc).expect("decode AtomicU16 as u16");
    assert_eq!(dec, original.load(Ordering::Relaxed));
}

// --- Test 9: AtomicU64 roundtrip large value ---

#[cfg(target_has_atomic = "64")]
#[test]
fn test_atomic_u64_roundtrip_large_value() {
    let large: u64 = 9_999_999_999_999_999_u64;
    let original = AtomicU64::new(large);
    let enc = encode_to_vec(&original).expect("encode AtomicU64 large");
    let (dec, _consumed): (u64, usize) =
        decode_from_slice(&enc).expect("decode AtomicU64 large as u64");
    assert_eq!(dec, large);
}

// --- Test 10: AtomicI32 roundtrip negative value ---

#[cfg(target_has_atomic = "32")]
#[test]
fn test_atomic_i32_roundtrip_negative() {
    let original = AtomicI32::new(-987654);
    let enc = encode_to_vec(&original).expect("encode AtomicI32 negative");
    let (dec, _consumed): (i32, usize) = decode_from_slice(&enc).expect("decode AtomicI32 as i32");
    assert_eq!(dec, original.load(Ordering::Relaxed));
}

// --- Test 11: AtomicI64 roundtrip large negative ---

#[cfg(target_has_atomic = "64")]
#[test]
fn test_atomic_i64_roundtrip_large_negative() {
    let original = AtomicI64::new(i64::MIN + 1);
    let enc = encode_to_vec(&original).expect("encode AtomicI64 large negative");
    let (dec, _consumed): (i64, usize) = decode_from_slice(&enc).expect("decode AtomicI64 as i64");
    assert_eq!(dec, original.load(Ordering::Relaxed));
}

// --- Test 12: AtomicI8 roundtrip value=-50 ---

#[cfg(target_has_atomic = "8")]
#[test]
fn test_atomic_i8_roundtrip_neg50() {
    let original = AtomicI8::new(-50);
    let enc = encode_to_vec(&original).expect("encode AtomicI8(-50)");
    let (dec, _consumed): (i8, usize) = decode_from_slice(&enc).expect("decode AtomicI8 as i8");
    assert_eq!(dec, -50i8);
}

// --- Test 13: AtomicI16 roundtrip value=32767 ---

#[cfg(target_has_atomic = "16")]
#[test]
fn test_atomic_i16_roundtrip_32767() {
    let original = AtomicI16::new(32767);
    let enc = encode_to_vec(&original).expect("encode AtomicI16(32767)");
    let (dec, _consumed): (i16, usize) = decode_from_slice(&enc).expect("decode AtomicI16 as i16");
    assert_eq!(dec, 32767i16);
}

// --- Test 14: AtomicU32 consumed equals encoded length ---

#[cfg(target_has_atomic = "32")]
#[test]
fn test_atomic_u32_consumed_equals_encoded_length() {
    let original = AtomicU32::new(7777);
    let enc = encode_to_vec(&original).expect("encode AtomicU32 for consumed check");
    let (_dec, consumed): (u32, usize) =
        decode_from_slice(&enc).expect("decode AtomicU32 consumed");
    assert_eq!(
        consumed,
        enc.len(),
        "consumed bytes must equal total encoded length"
    );
}

// --- Test 15: AtomicU32 with fixed-int config — 4 bytes ---

#[cfg(target_has_atomic = "32")]
#[test]
fn test_atomic_u32_fixed_int_config_4_bytes() {
    let original = AtomicU32::new(1);
    let cfg = config::standard().with_fixed_int_encoding();
    let enc = encode_to_vec_with_config(&original, cfg).expect("encode AtomicU32 fixed-int");
    assert_eq!(enc.len(), 4, "fixed-int u32 must encode to exactly 4 bytes");
    let (dec, _consumed): (u32, usize) =
        decode_from_slice_with_config(&enc, cfg).expect("decode AtomicU32 fixed-int");
    assert_eq!(dec, original.load(Ordering::Relaxed));
}

// --- Test 16: AtomicU32 with big-endian config byte order ---

#[cfg(target_has_atomic = "32")]
#[test]
fn test_atomic_u32_big_endian_config() {
    let value: u32 = 0x01020304;
    let original = AtomicU32::new(value);
    let cfg = config::standard()
        .with_big_endian()
        .with_fixed_int_encoding();
    let enc = encode_to_vec_with_config(&original, cfg).expect("encode AtomicU32 big-endian");
    // big-endian fixed: most significant byte first
    assert_eq!(enc[0], 0x01, "first byte should be MSB in big-endian");
    assert_eq!(enc[1], 0x02);
    assert_eq!(enc[2], 0x03);
    assert_eq!(enc[3], 0x04);
    let (dec, _consumed): (u32, usize) =
        decode_from_slice_with_config(&enc, cfg).expect("decode AtomicU32 big-endian");
    assert_eq!(dec, value);
}

// --- Test 17: Multiple AtomicU32 values encode independently ---

#[cfg(target_has_atomic = "32")]
#[test]
fn test_multiple_atomic_u32_encode_independently() {
    let a = AtomicU32::new(111);
    let b = AtomicU32::new(222);
    let c = AtomicU32::new(333);
    let enc_a = encode_to_vec(&a).expect("encode AtomicU32 a");
    let enc_b = encode_to_vec(&b).expect("encode AtomicU32 b");
    let enc_c = encode_to_vec(&c).expect("encode AtomicU32 c");
    let (dec_a, _): (u32, usize) = decode_from_slice(&enc_a).expect("decode a");
    let (dec_b, _): (u32, usize) = decode_from_slice(&enc_b).expect("decode b");
    let (dec_c, _): (u32, usize) = decode_from_slice(&enc_c).expect("decode c");
    assert_eq!(dec_a, 111u32);
    assert_eq!(dec_b, 222u32);
    assert_eq!(dec_c, 333u32);
    // Buffers are distinct
    assert_ne!(enc_a, enc_b);
    assert_ne!(enc_b, enc_c);
}

// --- Test 18: AtomicU64 with fixed-int config — 8 bytes ---

#[cfg(target_has_atomic = "64")]
#[test]
fn test_atomic_u64_fixed_int_config_8_bytes() {
    let original = AtomicU64::new(42);
    let cfg = config::standard().with_fixed_int_encoding();
    let enc = encode_to_vec_with_config(&original, cfg).expect("encode AtomicU64 fixed-int");
    assert_eq!(enc.len(), 8, "fixed-int u64 must encode to exactly 8 bytes");
    let (dec, _consumed): (u64, usize) =
        decode_from_slice_with_config(&enc, cfg).expect("decode AtomicU64 fixed-int");
    assert_eq!(dec, original.load(Ordering::Relaxed));
}

// --- Test 19: AtomicI32 with fixed-int config — 4 bytes ---

#[cfg(target_has_atomic = "32")]
#[test]
fn test_atomic_i32_fixed_int_config_4_bytes() {
    let original = AtomicI32::new(-1);
    let cfg = config::standard().with_fixed_int_encoding();
    let enc = encode_to_vec_with_config(&original, cfg).expect("encode AtomicI32 fixed-int");
    assert_eq!(enc.len(), 4, "fixed-int i32 must encode to exactly 4 bytes");
    let (dec, _consumed): (i32, usize) =
        decode_from_slice_with_config(&enc, cfg).expect("decode AtomicI32 fixed-int");
    assert_eq!(dec, -1i32);
}

// --- Test 20: AtomicBool true and false have distinct wire bytes ---

#[cfg(target_has_atomic = "8")]
#[test]
fn test_atomic_bool_true_false_distinct_wire_bytes() {
    let enc_true = encode_to_vec(&AtomicBool::new(true)).expect("encode AtomicBool(true)");
    let enc_false = encode_to_vec(&AtomicBool::new(false)).expect("encode AtomicBool(false)");
    assert!(
        !enc_true.is_empty(),
        "AtomicBool(true) encoding must not be empty"
    );
    assert!(
        !enc_false.is_empty(),
        "AtomicBool(false) encoding must not be empty"
    );
    assert_ne!(
        enc_true, enc_false,
        "true and false must have distinct wire representations"
    );
}

// --- Test 21: AtomicU32 zero roundtrip ---

#[cfg(target_has_atomic = "32")]
#[test]
fn test_atomic_u32_zero_roundtrip() {
    let original = AtomicU32::new(0);
    let enc = encode_to_vec(&original).expect("encode AtomicU32(0)");
    let (dec, _consumed): (u32, usize) =
        decode_from_slice(&enc).expect("decode AtomicU32(0) as u32");
    assert_eq!(dec, 0u32);
}

// --- Test 22: AtomicU32 max value roundtrip (u32::MAX) ---

#[cfg(target_has_atomic = "32")]
#[test]
fn test_atomic_u32_max_value_roundtrip() {
    let original = AtomicU32::new(u32::MAX);
    let enc = encode_to_vec(&original).expect("encode AtomicU32(u32::MAX)");
    let (dec, _consumed): (u32, usize) =
        decode_from_slice(&enc).expect("decode AtomicU32(MAX) as u32");
    assert_eq!(dec, u32::MAX);
}
