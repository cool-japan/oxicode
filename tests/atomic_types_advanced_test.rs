//! Advanced roundtrip and property tests for atomic types in OxiCode.

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
use std::sync::atomic::{
    AtomicBool, AtomicI32, AtomicI64, AtomicI8, AtomicIsize, AtomicU16, AtomicU32, AtomicU64,
    AtomicU8, AtomicUsize, Ordering,
};

// ---------------------------------------------------------------------------
// 1. AtomicU32::new(42) roundtrip
// ---------------------------------------------------------------------------

#[cfg(target_has_atomic = "32")]
#[test]
fn test_adv_atomic_u32_42_roundtrip() {
    let original = AtomicU32::new(42);
    let bytes = encode_to_vec(&original).expect("encode AtomicU32(42)");
    let (decoded, _): (AtomicU32, usize) = decode_from_slice(&bytes).expect("decode AtomicU32(42)");
    assert_eq!(decoded.load(Ordering::Relaxed), 42u32);
}

// ---------------------------------------------------------------------------
// 2. AtomicBool::new(true) roundtrip
// ---------------------------------------------------------------------------

#[cfg(target_has_atomic = "8")]
#[test]
fn test_adv_atomic_bool_true_roundtrip() {
    let original = AtomicBool::new(true);
    let bytes = encode_to_vec(&original).expect("encode AtomicBool(true)");
    let (decoded, _): (AtomicBool, usize) =
        decode_from_slice(&bytes).expect("decode AtomicBool(true)");
    assert!(decoded.load(Ordering::Relaxed));
}

// ---------------------------------------------------------------------------
// 3. AtomicBool::new(false) roundtrip
// ---------------------------------------------------------------------------

#[cfg(target_has_atomic = "8")]
#[test]
fn test_adv_atomic_bool_false_roundtrip() {
    let original = AtomicBool::new(false);
    let bytes = encode_to_vec(&original).expect("encode AtomicBool(false)");
    let (decoded, _): (AtomicBool, usize) =
        decode_from_slice(&bytes).expect("decode AtomicBool(false)");
    assert!(!decoded.load(Ordering::Relaxed));
}

// ---------------------------------------------------------------------------
// 4. AtomicU64::new(u64::MAX) roundtrip
// ---------------------------------------------------------------------------

#[cfg(target_has_atomic = "64")]
#[test]
fn test_adv_atomic_u64_max_roundtrip() {
    let original = AtomicU64::new(u64::MAX);
    let bytes = encode_to_vec(&original).expect("encode AtomicU64(MAX)");
    let (decoded, _): (AtomicU64, usize) =
        decode_from_slice(&bytes).expect("decode AtomicU64(MAX)");
    assert_eq!(decoded.load(Ordering::Relaxed), u64::MAX);
}

// ---------------------------------------------------------------------------
// 5. AtomicI32::new(-1000) roundtrip
// ---------------------------------------------------------------------------

#[cfg(target_has_atomic = "32")]
#[test]
fn test_adv_atomic_i32_neg1000_roundtrip() {
    let original = AtomicI32::new(-1000);
    let bytes = encode_to_vec(&original).expect("encode AtomicI32(-1000)");
    let (decoded, _): (AtomicI32, usize) =
        decode_from_slice(&bytes).expect("decode AtomicI32(-1000)");
    assert_eq!(decoded.load(Ordering::Relaxed), -1000i32);
}

// ---------------------------------------------------------------------------
// 6. AtomicU8::new(255) roundtrip
// ---------------------------------------------------------------------------

#[cfg(target_has_atomic = "8")]
#[test]
fn test_adv_atomic_u8_255_roundtrip() {
    let original = AtomicU8::new(255);
    let bytes = encode_to_vec(&original).expect("encode AtomicU8(255)");
    let (decoded, _): (AtomicU8, usize) = decode_from_slice(&bytes).expect("decode AtomicU8(255)");
    assert_eq!(decoded.load(Ordering::Relaxed), 255u8);
}

// ---------------------------------------------------------------------------
// 7. AtomicUsize::new(0) roundtrip
// ---------------------------------------------------------------------------

#[cfg(target_has_atomic = "ptr")]
#[test]
fn test_adv_atomic_usize_zero_roundtrip() {
    let original = AtomicUsize::new(0);
    let bytes = encode_to_vec(&original).expect("encode AtomicUsize(0)");
    let (decoded, _): (AtomicUsize, usize) =
        decode_from_slice(&bytes).expect("decode AtomicUsize(0)");
    assert_eq!(decoded.load(Ordering::Relaxed), 0usize);
}

// ---------------------------------------------------------------------------
// 8. AtomicIsize::new(isize::MIN) roundtrip
// ---------------------------------------------------------------------------

#[cfg(target_has_atomic = "ptr")]
#[test]
fn test_adv_atomic_isize_min_roundtrip() {
    let original = AtomicIsize::new(isize::MIN);
    let bytes = encode_to_vec(&original).expect("encode AtomicIsize(MIN)");
    let (decoded, _): (AtomicIsize, usize) =
        decode_from_slice(&bytes).expect("decode AtomicIsize(MIN)");
    assert_eq!(decoded.load(Ordering::Relaxed), isize::MIN);
}

// ---------------------------------------------------------------------------
// 9. AtomicI64::new(i64::MIN) roundtrip
// ---------------------------------------------------------------------------

#[cfg(target_has_atomic = "64")]
#[test]
fn test_adv_atomic_i64_min_roundtrip() {
    let original = AtomicI64::new(i64::MIN);
    let bytes = encode_to_vec(&original).expect("encode AtomicI64(MIN)");
    let (decoded, _): (AtomicI64, usize) =
        decode_from_slice(&bytes).expect("decode AtomicI64(MIN)");
    assert_eq!(decoded.load(Ordering::Relaxed), i64::MIN);
}

// ---------------------------------------------------------------------------
// 10. AtomicU16::new(65535) roundtrip
// ---------------------------------------------------------------------------

#[cfg(target_has_atomic = "16")]
#[test]
fn test_adv_atomic_u16_max_roundtrip() {
    let original = AtomicU16::new(65535);
    let bytes = encode_to_vec(&original).expect("encode AtomicU16(65535)");
    let (decoded, _): (AtomicU16, usize) =
        decode_from_slice(&bytes).expect("decode AtomicU16(65535)");
    assert_eq!(decoded.load(Ordering::Relaxed), 65535u16);
}

// ---------------------------------------------------------------------------
// 11. AtomicU32 encodes same bytes as plain u32
// ---------------------------------------------------------------------------

#[cfg(target_has_atomic = "32")]
#[test]
fn test_adv_atomic_u32_same_bytes_as_u32() {
    let n: u32 = 999_999;
    let atomic_bytes = encode_to_vec(&AtomicU32::new(n)).expect("encode AtomicU32");
    let plain_bytes = encode_to_vec(&n).expect("encode u32");
    assert_eq!(
        atomic_bytes, plain_bytes,
        "AtomicU32 must encode identically to u32"
    );
}

// ---------------------------------------------------------------------------
// 12. Vec<AtomicU32> with 3 elements roundtrip
// ---------------------------------------------------------------------------

#[cfg(target_has_atomic = "32")]
#[test]
fn test_adv_vec_atomic_u32_roundtrip() {
    let original: Vec<AtomicU32> = vec![AtomicU32::new(1), AtomicU32::new(2), AtomicU32::new(3)];
    let bytes = encode_to_vec(&original).expect("encode Vec<AtomicU32>");
    let (decoded, _): (Vec<AtomicU32>, usize) =
        decode_from_slice(&bytes).expect("decode Vec<AtomicU32>");
    let expected = [1u32, 2, 3];
    for (i, atom) in decoded.iter().enumerate() {
        assert_eq!(atom.load(Ordering::Relaxed), expected[i]);
    }
}

// ---------------------------------------------------------------------------
// 13. AtomicBool with fixed int encoding
// ---------------------------------------------------------------------------

#[cfg(target_has_atomic = "8")]
#[test]
fn test_adv_atomic_bool_fixed_int_encoding() {
    use oxicode::{decode_from_slice_with_config, encode_to_vec_with_config};
    let cfg = config::standard().with_fixed_int_encoding();
    let original = AtomicBool::new(true);
    let bytes = encode_to_vec_with_config(&original, cfg).expect("encode AtomicBool fixed-int");
    let (decoded, _): (AtomicBool, usize) =
        decode_from_slice_with_config(&bytes, cfg).expect("decode AtomicBool fixed-int");
    assert!(decoded.load(Ordering::Relaxed));
}

// ---------------------------------------------------------------------------
// 14. AtomicU32 with big endian config
// ---------------------------------------------------------------------------

#[cfg(target_has_atomic = "32")]
#[test]
fn test_adv_atomic_u32_big_endian() {
    use oxicode::{decode_from_slice_with_config, encode_to_vec_with_config};
    let cfg = config::standard().with_big_endian();
    let value = 0xDEAD_BEEFu32;
    let original = AtomicU32::new(value);
    let bytes = encode_to_vec_with_config(&original, cfg).expect("encode AtomicU32 big-endian");
    let (decoded, _): (AtomicU32, usize) =
        decode_from_slice_with_config(&bytes, cfg).expect("decode AtomicU32 big-endian");
    assert_eq!(decoded.load(Ordering::Relaxed), value);
}

// ---------------------------------------------------------------------------
// 15. AtomicI8::new(i8::MIN) and AtomicI8::new(i8::MAX) roundtrip
// ---------------------------------------------------------------------------

#[cfg(target_has_atomic = "8")]
#[test]
fn test_adv_atomic_i8_boundary_roundtrip() {
    let min_orig = AtomicI8::new(i8::MIN);
    let min_bytes = encode_to_vec(&min_orig).expect("encode AtomicI8(MIN)");
    let (min_dec, _): (AtomicI8, usize) =
        decode_from_slice(&min_bytes).expect("decode AtomicI8(MIN)");
    assert_eq!(min_dec.load(Ordering::Relaxed), i8::MIN);

    let max_orig = AtomicI8::new(i8::MAX);
    let max_bytes = encode_to_vec(&max_orig).expect("encode AtomicI8(MAX)");
    let (max_dec, _): (AtomicI8, usize) =
        decode_from_slice(&max_bytes).expect("decode AtomicI8(MAX)");
    assert_eq!(max_dec.load(Ordering::Relaxed), i8::MAX);
}

// ---------------------------------------------------------------------------
// 16. AtomicU8::new(0) and AtomicU8::new(255) roundtrip
// ---------------------------------------------------------------------------

#[cfg(target_has_atomic = "8")]
#[test]
fn test_adv_atomic_u8_boundary_roundtrip() {
    let zero_orig = AtomicU8::new(0);
    let zero_bytes = encode_to_vec(&zero_orig).expect("encode AtomicU8(0)");
    let (zero_dec, _): (AtomicU8, usize) =
        decode_from_slice(&zero_bytes).expect("decode AtomicU8(0)");
    assert_eq!(zero_dec.load(Ordering::Relaxed), 0u8);

    let max_orig = AtomicU8::new(255);
    let max_bytes = encode_to_vec(&max_orig).expect("encode AtomicU8(255)");
    let (max_dec, _): (AtomicU8, usize) =
        decode_from_slice(&max_bytes).expect("decode AtomicU8(255)");
    assert_eq!(max_dec.load(Ordering::Relaxed), 255u8);
}

// ---------------------------------------------------------------------------
// 17. AtomicU64::new(u64::MAX) roundtrip (legacy config variant)
// ---------------------------------------------------------------------------

#[cfg(target_has_atomic = "64")]
#[test]
fn test_adv_atomic_u64_max_legacy_roundtrip() {
    use oxicode::{decode_from_slice_with_config, encode_to_vec_with_config};
    let cfg = config::legacy();
    let original = AtomicU64::new(u64::MAX);
    let bytes = encode_to_vec_with_config(&original, cfg).expect("encode AtomicU64(MAX) legacy");
    let (decoded, _): (AtomicU64, usize) =
        decode_from_slice_with_config(&bytes, cfg).expect("decode AtomicU64(MAX) legacy");
    assert_eq!(decoded.load(Ordering::Relaxed), u64::MAX);
}

// ---------------------------------------------------------------------------
// 18. AtomicI64::new(i64::MIN) roundtrip (fixed-int config variant)
// ---------------------------------------------------------------------------

#[cfg(target_has_atomic = "64")]
#[test]
fn test_adv_atomic_i64_min_fixed_int_roundtrip() {
    use oxicode::{decode_from_slice_with_config, encode_to_vec_with_config};
    let cfg = config::standard().with_fixed_int_encoding();
    let original = AtomicI64::new(i64::MIN);
    let bytes = encode_to_vec_with_config(&original, cfg).expect("encode AtomicI64(MIN) fixed");
    let (decoded, _): (AtomicI64, usize) =
        decode_from_slice_with_config(&bytes, cfg).expect("decode AtomicI64(MIN) fixed");
    assert_eq!(decoded.load(Ordering::Relaxed), i64::MIN);
}

// ---------------------------------------------------------------------------
// 19. (AtomicU32, AtomicBool) tuple roundtrip
// ---------------------------------------------------------------------------

#[cfg(all(target_has_atomic = "32", target_has_atomic = "8"))]
#[test]
fn test_adv_tuple_atomic_u32_bool_roundtrip() {
    let original = (AtomicU32::new(12345), AtomicBool::new(true));
    let bytes = encode_to_vec(&original).expect("encode (AtomicU32, AtomicBool)");
    let (decoded, _): ((AtomicU32, AtomicBool), usize) =
        decode_from_slice(&bytes).expect("decode (AtomicU32, AtomicBool)");
    assert_eq!(decoded.0.load(Ordering::Relaxed), 12345u32);
    assert!(decoded.1.load(Ordering::Relaxed));
}

// ---------------------------------------------------------------------------
// 20. Option<AtomicU64> Some/None roundtrip
// ---------------------------------------------------------------------------

#[cfg(target_has_atomic = "64")]
#[test]
fn test_adv_option_atomic_u64_roundtrip() {
    // Some variant
    let some_val: Option<AtomicU64> = Some(AtomicU64::new(42));
    let some_bytes = encode_to_vec(&some_val).expect("encode Option<AtomicU64>(Some)");
    let (some_dec, _): (Option<AtomicU64>, usize) =
        decode_from_slice(&some_bytes).expect("decode Option<AtomicU64>(Some)");
    assert_eq!(
        some_dec.expect("expected Some").load(Ordering::Relaxed),
        42u64
    );

    // None variant
    let none_val: Option<AtomicU64> = None;
    let none_bytes = encode_to_vec(&none_val).expect("encode Option<AtomicU64>(None)");
    let (none_dec, _): (Option<AtomicU64>, usize) =
        decode_from_slice(&none_bytes).expect("decode Option<AtomicU64>(None)");
    assert!(none_dec.is_none());
}

// ---------------------------------------------------------------------------
// 21. AtomicU32 with config::standard().with_limit::<1000>()
// ---------------------------------------------------------------------------

#[cfg(target_has_atomic = "32")]
#[test]
fn test_adv_atomic_u32_with_limit_config() {
    use oxicode::{decode_from_slice_with_config, encode_to_vec_with_config};
    let cfg = config::standard().with_limit::<1000>();
    let value = 777u32;
    let original = AtomicU32::new(value);
    let bytes = encode_to_vec_with_config(&original, cfg).expect("encode AtomicU32 with limit");
    let (decoded, _): (AtomicU32, usize) =
        decode_from_slice_with_config(&bytes, cfg).expect("decode AtomicU32 with limit");
    assert_eq!(decoded.load(Ordering::Relaxed), value);
}

// ---------------------------------------------------------------------------
// 22. AtomicUsize encode size equals usize encode size
// ---------------------------------------------------------------------------

#[cfg(target_has_atomic = "ptr")]
#[test]
fn test_adv_atomic_usize_encode_size_equals_usize() {
    let n: usize = 12345;
    let atomic_bytes = encode_to_vec(&AtomicUsize::new(n)).expect("encode AtomicUsize");
    let plain_bytes = encode_to_vec(&n).expect("encode usize");
    assert_eq!(
        atomic_bytes.len(),
        plain_bytes.len(),
        "AtomicUsize must encode to the same number of bytes as usize"
    );
    assert_eq!(
        atomic_bytes, plain_bytes,
        "AtomicUsize must encode identically to usize"
    );
}
