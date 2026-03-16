#![cfg(feature = "std")]
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
use oxicode::{decode_from_slice, encode_to_vec};
use std::sync::atomic::{
    AtomicBool, AtomicI16, AtomicI32, AtomicI64, AtomicI8, AtomicIsize, AtomicU16, AtomicU32,
    AtomicU64, AtomicU8, AtomicUsize, Ordering,
};

// ===== Test 1: AtomicBool true roundtrip =====

#[cfg(target_has_atomic = "8")]
#[test]
fn test_atomic_bool_true_advanced_roundtrip() {
    let val = AtomicBool::new(true);
    let bytes = encode_to_vec(&val).expect("encode AtomicBool(true)");
    let (decoded, consumed): (AtomicBool, usize) =
        decode_from_slice(&bytes).expect("decode AtomicBool(true)");
    assert!(consumed > 0, "consumed bytes must be nonzero");
    assert_eq!(
        val.load(Ordering::SeqCst),
        decoded.load(Ordering::SeqCst),
        "AtomicBool(true) roundtrip mismatch"
    );
}

// ===== Test 2: AtomicBool false roundtrip =====

#[cfg(target_has_atomic = "8")]
#[test]
fn test_atomic_bool_false_advanced_roundtrip() {
    let val = AtomicBool::new(false);
    let bytes = encode_to_vec(&val).expect("encode AtomicBool(false)");
    let (decoded, consumed): (AtomicBool, usize) =
        decode_from_slice(&bytes).expect("decode AtomicBool(false)");
    assert!(consumed > 0, "consumed bytes must be nonzero");
    assert_eq!(
        val.load(Ordering::SeqCst),
        decoded.load(Ordering::SeqCst),
        "AtomicBool(false) roundtrip mismatch"
    );
}

// ===== Test 3: AtomicU32 roundtrip value 42 =====

#[cfg(target_has_atomic = "32")]
#[test]
fn test_atomic_u32_value_42_roundtrip() {
    let val = AtomicU32::new(42u32);
    let bytes = encode_to_vec(&val).expect("encode AtomicU32(42)");
    let (decoded, consumed): (AtomicU32, usize) =
        decode_from_slice(&bytes).expect("decode AtomicU32(42)");
    assert!(consumed > 0, "consumed bytes must be nonzero");
    assert_eq!(
        val.load(Ordering::SeqCst),
        decoded.load(Ordering::SeqCst),
        "AtomicU32(42) roundtrip mismatch"
    );
}

// ===== Test 4: AtomicU32 roundtrip u32::MAX =====

#[cfg(target_has_atomic = "32")]
#[test]
fn test_atomic_u32_max_roundtrip() {
    let val = AtomicU32::new(u32::MAX);
    let bytes = encode_to_vec(&val).expect("encode AtomicU32(MAX)");
    let (decoded, consumed): (AtomicU32, usize) =
        decode_from_slice(&bytes).expect("decode AtomicU32(MAX)");
    assert!(consumed > 0, "consumed bytes must be nonzero");
    assert_eq!(
        val.load(Ordering::SeqCst),
        decoded.load(Ordering::SeqCst),
        "AtomicU32(MAX) roundtrip mismatch"
    );
}

// ===== Test 5: AtomicU64 roundtrip large value =====

#[cfg(target_has_atomic = "64")]
#[test]
fn test_atomic_u64_large_value_roundtrip() {
    let large: u64 = 9_876_543_210_123_456_789;
    let val = AtomicU64::new(large);
    let bytes = encode_to_vec(&val).expect("encode AtomicU64(large)");
    let (decoded, consumed): (AtomicU64, usize) =
        decode_from_slice(&bytes).expect("decode AtomicU64(large)");
    assert!(consumed > 0, "consumed bytes must be nonzero");
    assert_eq!(
        val.load(Ordering::SeqCst),
        decoded.load(Ordering::SeqCst),
        "AtomicU64(large) roundtrip mismatch"
    );
}

// ===== Test 6: AtomicI32 roundtrip negative value =====

#[cfg(target_has_atomic = "32")]
#[test]
fn test_atomic_i32_negative_roundtrip() {
    let val = AtomicI32::new(-999_999i32);
    let bytes = encode_to_vec(&val).expect("encode AtomicI32(-999999)");
    let (decoded, consumed): (AtomicI32, usize) =
        decode_from_slice(&bytes).expect("decode AtomicI32(-999999)");
    assert!(consumed > 0, "consumed bytes must be nonzero");
    assert_eq!(
        val.load(Ordering::SeqCst),
        decoded.load(Ordering::SeqCst),
        "AtomicI32(-999999) roundtrip mismatch"
    );
}

// ===== Test 7: AtomicI64 roundtrip i64::MIN =====

#[cfg(target_has_atomic = "64")]
#[test]
fn test_atomic_i64_min_roundtrip() {
    let val = AtomicI64::new(i64::MIN);
    let bytes = encode_to_vec(&val).expect("encode AtomicI64(MIN)");
    let (decoded, consumed): (AtomicI64, usize) =
        decode_from_slice(&bytes).expect("decode AtomicI64(MIN)");
    assert!(consumed > 0, "consumed bytes must be nonzero");
    assert_eq!(
        val.load(Ordering::SeqCst),
        decoded.load(Ordering::SeqCst),
        "AtomicI64(MIN) roundtrip mismatch"
    );
}

// ===== Test 8: AtomicUsize roundtrip =====

#[cfg(target_has_atomic = "ptr")]
#[test]
fn test_atomic_usize_roundtrip_advanced() {
    let val = AtomicUsize::new(usize::MAX / 4);
    let bytes = encode_to_vec(&val).expect("encode AtomicUsize");
    let (decoded, consumed): (AtomicUsize, usize) =
        decode_from_slice(&bytes).expect("decode AtomicUsize");
    assert!(consumed > 0, "consumed bytes must be nonzero");
    assert_eq!(
        val.load(Ordering::SeqCst),
        decoded.load(Ordering::SeqCst),
        "AtomicUsize roundtrip mismatch"
    );
}

// ===== Test 9: AtomicIsize roundtrip negative =====

#[cfg(target_has_atomic = "ptr")]
#[test]
fn test_atomic_isize_negative_roundtrip() {
    let val = AtomicIsize::new(-1_234_567isize);
    let bytes = encode_to_vec(&val).expect("encode AtomicIsize(negative)");
    let (decoded, consumed): (AtomicIsize, usize) =
        decode_from_slice(&bytes).expect("decode AtomicIsize(negative)");
    assert!(consumed > 0, "consumed bytes must be nonzero");
    assert_eq!(
        val.load(Ordering::SeqCst),
        decoded.load(Ordering::SeqCst),
        "AtomicIsize(negative) roundtrip mismatch"
    );
}

// ===== Test 10: AtomicU8 roundtrip 255 =====

#[cfg(target_has_atomic = "8")]
#[test]
fn test_atomic_u8_max_roundtrip() {
    let val = AtomicU8::new(255u8);
    let bytes = encode_to_vec(&val).expect("encode AtomicU8(255)");
    let (decoded, consumed): (AtomicU8, usize) =
        decode_from_slice(&bytes).expect("decode AtomicU8(255)");
    assert!(consumed > 0, "consumed bytes must be nonzero");
    assert_eq!(
        val.load(Ordering::SeqCst),
        decoded.load(Ordering::SeqCst),
        "AtomicU8(255) roundtrip mismatch"
    );
}

// ===== Test 11: AtomicI8 roundtrip -128 =====

#[cfg(target_has_atomic = "8")]
#[test]
fn test_atomic_i8_min_roundtrip() {
    let val = AtomicI8::new(i8::MIN);
    let bytes = encode_to_vec(&val).expect("encode AtomicI8(MIN)");
    let (decoded, consumed): (AtomicI8, usize) =
        decode_from_slice(&bytes).expect("decode AtomicI8(MIN)");
    assert!(consumed > 0, "consumed bytes must be nonzero");
    assert_eq!(
        val.load(Ordering::SeqCst),
        decoded.load(Ordering::SeqCst),
        "AtomicI8(MIN) roundtrip mismatch"
    );
}

// ===== Test 12: AtomicU16 roundtrip 65535 =====

#[cfg(target_has_atomic = "16")]
#[test]
fn test_atomic_u16_max_roundtrip() {
    let val = AtomicU16::new(u16::MAX);
    let bytes = encode_to_vec(&val).expect("encode AtomicU16(MAX)");
    let (decoded, consumed): (AtomicU16, usize) =
        decode_from_slice(&bytes).expect("decode AtomicU16(MAX)");
    assert!(consumed > 0, "consumed bytes must be nonzero");
    assert_eq!(
        val.load(Ordering::SeqCst),
        decoded.load(Ordering::SeqCst),
        "AtomicU16(MAX) roundtrip mismatch"
    );
}

// ===== Test 13: AtomicI16 roundtrip -32768 =====

#[cfg(target_has_atomic = "16")]
#[test]
fn test_atomic_i16_min_roundtrip() {
    let val = AtomicI16::new(i16::MIN);
    let bytes = encode_to_vec(&val).expect("encode AtomicI16(MIN)");
    let (decoded, consumed): (AtomicI16, usize) =
        decode_from_slice(&bytes).expect("decode AtomicI16(MIN)");
    assert!(consumed > 0, "consumed bytes must be nonzero");
    assert_eq!(
        val.load(Ordering::SeqCst),
        decoded.load(Ordering::SeqCst),
        "AtomicI16(MIN) roundtrip mismatch"
    );
}

// ===== Test 14: AtomicU32 encodes same bytes as raw u32 =====

#[cfg(target_has_atomic = "32")]
#[test]
fn test_atomic_u32_same_bytes_as_u32() {
    let raw_val: u32 = 0xDEAD_BEEF;
    let atomic_val = AtomicU32::new(raw_val);
    let raw_bytes = encode_to_vec(&raw_val).expect("encode raw u32");
    let atomic_bytes = encode_to_vec(&atomic_val).expect("encode AtomicU32");
    assert_eq!(
        raw_bytes, atomic_bytes,
        "AtomicU32 and u32 should produce identical bytes"
    );
}

// ===== Test 15: AtomicI32 encodes same bytes as raw i32 =====

#[cfg(target_has_atomic = "32")]
#[test]
fn test_atomic_i32_same_bytes_as_i32() {
    let raw_val: i32 = -42_000_000i32;
    let atomic_val = AtomicI32::new(raw_val);
    let raw_bytes = encode_to_vec(&raw_val).expect("encode raw i32");
    let atomic_bytes = encode_to_vec(&atomic_val).expect("encode AtomicI32");
    assert_eq!(
        raw_bytes, atomic_bytes,
        "AtomicI32 and i32 should produce identical bytes"
    );
}

// ===== Test 16: Vec of AtomicU32 roundtrip =====

#[cfg(target_has_atomic = "32")]
#[test]
fn test_vec_of_atomic_u32_roundtrip() {
    let vals: Vec<AtomicU32> = vec![
        AtomicU32::new(0),
        AtomicU32::new(1),
        AtomicU32::new(u32::MAX / 2),
        AtomicU32::new(u32::MAX),
    ];
    let bytes = encode_to_vec(&vals).expect("encode Vec<AtomicU32>");
    let (decoded, consumed): (Vec<AtomicU32>, usize) =
        decode_from_slice(&bytes).expect("decode Vec<AtomicU32>");
    assert!(consumed > 0, "consumed bytes must be nonzero");
    assert_eq!(decoded.len(), vals.len(), "decoded length mismatch");
    for (orig, dec) in vals.iter().zip(decoded.iter()) {
        assert_eq!(
            orig.load(Ordering::SeqCst),
            dec.load(Ordering::SeqCst),
            "Vec<AtomicU32> element mismatch"
        );
    }
}

// ===== Test 17: Option<AtomicU32> Some roundtrip =====

#[cfg(target_has_atomic = "32")]
#[test]
fn test_option_atomic_u32_some_roundtrip() {
    let val: Option<AtomicU32> = Some(AtomicU32::new(12345u32));
    let bytes = encode_to_vec(&val).expect("encode Option<AtomicU32>(Some)");
    let (decoded, consumed): (Option<AtomicU32>, usize) =
        decode_from_slice(&bytes).expect("decode Option<AtomicU32>(Some)");
    assert!(consumed > 0, "consumed bytes must be nonzero");
    let inner = decoded.expect("expected Some after roundtrip");
    assert_eq!(
        inner.load(Ordering::SeqCst),
        12345u32,
        "Option<AtomicU32> Some value mismatch"
    );
}

// ===== Test 18: Option<AtomicU32> None roundtrip =====

#[cfg(target_has_atomic = "32")]
#[test]
fn test_option_atomic_u32_none_roundtrip() {
    let val: Option<AtomicU32> = None;
    let bytes = encode_to_vec(&val).expect("encode Option<AtomicU32>(None)");
    let (decoded, consumed): (Option<AtomicU32>, usize) =
        decode_from_slice(&bytes).expect("decode Option<AtomicU32>(None)");
    assert!(consumed > 0, "consumed bytes must be nonzero");
    assert!(decoded.is_none(), "expected None after roundtrip");
}

// ===== Test 19: AtomicBool with fixed-int config =====

#[cfg(target_has_atomic = "8")]
#[test]
fn test_atomic_bool_fixed_int_config_roundtrip() {
    use oxicode::{config, decode_from_slice_with_config, encode_to_vec_with_config};
    let cfg = config::standard().with_fixed_int_encoding();
    let val = AtomicBool::new(true);
    let bytes =
        encode_to_vec_with_config(&val, cfg).expect("encode AtomicBool with fixed-int config");
    let (decoded, consumed): (AtomicBool, usize) = decode_from_slice_with_config(&bytes, cfg)
        .expect("decode AtomicBool with fixed-int config");
    assert!(consumed > 0, "consumed bytes must be nonzero");
    assert_eq!(
        val.load(Ordering::SeqCst),
        decoded.load(Ordering::SeqCst),
        "AtomicBool fixed-int config roundtrip mismatch"
    );
}

// ===== Test 20: AtomicU64 with fixed-int config =====

#[cfg(target_has_atomic = "64")]
#[test]
fn test_atomic_u64_fixed_int_config_roundtrip() {
    use oxicode::{config, decode_from_slice_with_config, encode_to_vec_with_config};
    let cfg = config::standard().with_fixed_int_encoding();
    let val = AtomicU64::new(u64::MAX);
    let bytes =
        encode_to_vec_with_config(&val, cfg).expect("encode AtomicU64(MAX) with fixed-int config");
    // With fixed-int encoding, u64 always takes 8 bytes
    assert_eq!(
        bytes.len(),
        8,
        "AtomicU64 fixed-int encoding should be 8 bytes"
    );
    let (decoded, consumed): (AtomicU64, usize) = decode_from_slice_with_config(&bytes, cfg)
        .expect("decode AtomicU64(MAX) with fixed-int config");
    assert_eq!(consumed, 8, "consumed bytes should be 8 for fixed-int u64");
    assert_eq!(
        val.load(Ordering::SeqCst),
        decoded.load(Ordering::SeqCst),
        "AtomicU64(MAX) fixed-int config roundtrip mismatch"
    );
}

// ===== Test 21: Encoded size matches between AtomicU32 and u32 =====

#[cfg(target_has_atomic = "32")]
#[test]
fn test_atomic_u32_encoded_size_matches_u32() {
    let raw_val: u32 = 100u32;
    let atomic_val = AtomicU32::new(raw_val);
    let raw_size = oxicode::encoded_size(&raw_val).expect("encoded_size for u32");
    let atomic_size = oxicode::encoded_size(&atomic_val).expect("encoded_size for AtomicU32");
    assert_eq!(
        raw_size, atomic_size,
        "encoded size of AtomicU32 should match u32 for the same value"
    );
}

// ===== Test 22: AtomicU32 zero roundtrip =====

#[cfg(target_has_atomic = "32")]
#[test]
fn test_atomic_u32_zero_roundtrip() {
    let val = AtomicU32::new(0u32);
    let bytes = encode_to_vec(&val).expect("encode AtomicU32(0)");
    let (decoded, consumed): (AtomicU32, usize) =
        decode_from_slice(&bytes).expect("decode AtomicU32(0)");
    assert!(consumed > 0, "consumed bytes must be nonzero");
    assert_eq!(
        val.load(Ordering::SeqCst),
        decoded.load(Ordering::SeqCst),
        "AtomicU32(0) roundtrip mismatch"
    );
}
