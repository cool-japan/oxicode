//! Roundtrip tests for atomic types, ManuallyDrop, and PhantomData.

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
use core::marker::PhantomData;
use core::mem::ManuallyDrop;

// ===== AtomicBool =====

#[cfg(target_has_atomic = "8")]
#[test]
fn test_atomic_bool_true_roundtrip() {
    use core::sync::atomic::{AtomicBool, Ordering};
    let original = AtomicBool::new(true);
    let bytes = oxicode::encode_to_vec(&original).expect("encode AtomicBool(true)");
    let (decoded, _): (AtomicBool, _) =
        oxicode::decode_from_slice(&bytes).expect("decode AtomicBool(true)");
    assert_eq!(
        original.load(Ordering::Relaxed),
        decoded.load(Ordering::Relaxed)
    );
}

#[cfg(target_has_atomic = "8")]
#[test]
fn test_atomic_bool_false_roundtrip() {
    use core::sync::atomic::{AtomicBool, Ordering};
    let original = AtomicBool::new(false);
    let bytes = oxicode::encode_to_vec(&original).expect("encode AtomicBool(false)");
    let (decoded, _): (AtomicBool, _) =
        oxicode::decode_from_slice(&bytes).expect("decode AtomicBool(false)");
    assert_eq!(
        original.load(Ordering::Relaxed),
        decoded.load(Ordering::Relaxed)
    );
}

// ===== AtomicI8 / AtomicU8 =====

#[cfg(target_has_atomic = "8")]
#[test]
fn test_atomic_i8_roundtrip() {
    use core::sync::atomic::{AtomicI8, Ordering};
    let original = AtomicI8::new(-42i8);
    let bytes = oxicode::encode_to_vec(&original).expect("encode AtomicI8");
    let (decoded, _): (AtomicI8, _) = oxicode::decode_from_slice(&bytes).expect("decode AtomicI8");
    assert_eq!(
        original.load(Ordering::Relaxed),
        decoded.load(Ordering::Relaxed)
    );
}

#[cfg(target_has_atomic = "8")]
#[test]
fn test_atomic_u8_roundtrip() {
    use core::sync::atomic::{AtomicU8, Ordering};
    let original = AtomicU8::new(200u8);
    let bytes = oxicode::encode_to_vec(&original).expect("encode AtomicU8");
    let (decoded, _): (AtomicU8, _) = oxicode::decode_from_slice(&bytes).expect("decode AtomicU8");
    assert_eq!(
        original.load(Ordering::Relaxed),
        decoded.load(Ordering::Relaxed)
    );
}

// ===== AtomicI16 / AtomicU16 =====

#[cfg(target_has_atomic = "16")]
#[test]
fn test_atomic_i16_roundtrip() {
    use core::sync::atomic::{AtomicI16, Ordering};
    let original = AtomicI16::new(-1000i16);
    let bytes = oxicode::encode_to_vec(&original).expect("encode AtomicI16");
    let (decoded, _): (AtomicI16, _) =
        oxicode::decode_from_slice(&bytes).expect("decode AtomicI16");
    assert_eq!(
        original.load(Ordering::Relaxed),
        decoded.load(Ordering::Relaxed)
    );
}

#[cfg(target_has_atomic = "16")]
#[test]
fn test_atomic_u16_roundtrip() {
    use core::sync::atomic::{AtomicU16, Ordering};
    let original = AtomicU16::new(60000u16);
    let bytes = oxicode::encode_to_vec(&original).expect("encode AtomicU16");
    let (decoded, _): (AtomicU16, _) =
        oxicode::decode_from_slice(&bytes).expect("decode AtomicU16");
    assert_eq!(
        original.load(Ordering::Relaxed),
        decoded.load(Ordering::Relaxed)
    );
}

// ===== AtomicI32 / AtomicU32 =====

#[cfg(target_has_atomic = "32")]
#[test]
fn test_atomic_i32_roundtrip() {
    use core::sync::atomic::{AtomicI32, Ordering};
    let original = AtomicI32::new(-100_000i32);
    let bytes = oxicode::encode_to_vec(&original).expect("encode AtomicI32");
    let (decoded, _): (AtomicI32, _) =
        oxicode::decode_from_slice(&bytes).expect("decode AtomicI32");
    assert_eq!(
        original.load(Ordering::Relaxed),
        decoded.load(Ordering::Relaxed)
    );
}

#[cfg(target_has_atomic = "32")]
#[test]
fn test_atomic_u32_roundtrip() {
    use core::sync::atomic::{AtomicU32, Ordering};
    let original = AtomicU32::new(42u32);
    let bytes = oxicode::encode_to_vec(&original).expect("encode AtomicU32");
    let (decoded, _): (AtomicU32, _) =
        oxicode::decode_from_slice(&bytes).expect("decode AtomicU32");
    assert_eq!(
        original.load(Ordering::Relaxed),
        decoded.load(Ordering::Relaxed)
    );
}

// ===== AtomicI64 / AtomicU64 =====

#[cfg(target_has_atomic = "64")]
#[test]
fn test_atomic_i64_roundtrip() {
    use core::sync::atomic::{AtomicI64, Ordering};
    let original = AtomicI64::new(-12345i64);
    let bytes = oxicode::encode_to_vec(&original).expect("encode AtomicI64");
    let (decoded, _): (AtomicI64, _) =
        oxicode::decode_from_slice(&bytes).expect("decode AtomicI64");
    assert_eq!(
        original.load(Ordering::Relaxed),
        decoded.load(Ordering::Relaxed)
    );
}

#[cfg(target_has_atomic = "64")]
#[test]
fn test_atomic_u64_roundtrip() {
    use core::sync::atomic::{AtomicU64, Ordering};
    let original = AtomicU64::new(u64::MAX / 2);
    let bytes = oxicode::encode_to_vec(&original).expect("encode AtomicU64");
    let (decoded, _): (AtomicU64, _) =
        oxicode::decode_from_slice(&bytes).expect("decode AtomicU64");
    assert_eq!(
        original.load(Ordering::Relaxed),
        decoded.load(Ordering::Relaxed)
    );
}

// ===== AtomicIsize / AtomicUsize =====

#[cfg(target_has_atomic = "ptr")]
#[test]
fn test_atomic_isize_roundtrip() {
    use core::sync::atomic::{AtomicIsize, Ordering};
    let original = AtomicIsize::new(-9999isize);
    let bytes = oxicode::encode_to_vec(&original).expect("encode AtomicIsize");
    let (decoded, _): (AtomicIsize, _) =
        oxicode::decode_from_slice(&bytes).expect("decode AtomicIsize");
    assert_eq!(
        original.load(Ordering::Relaxed),
        decoded.load(Ordering::Relaxed)
    );
}

#[cfg(target_has_atomic = "ptr")]
#[test]
fn test_atomic_usize_roundtrip() {
    use core::sync::atomic::{AtomicUsize, Ordering};
    let original = AtomicUsize::new(999usize);
    let bytes = oxicode::encode_to_vec(&original).expect("encode AtomicUsize");
    let (decoded, _): (AtomicUsize, _) =
        oxicode::decode_from_slice(&bytes).expect("decode AtomicUsize");
    assert_eq!(
        original.load(Ordering::Relaxed),
        decoded.load(Ordering::Relaxed)
    );
}

// ===== ManuallyDrop =====

#[test]
fn test_manually_drop_u32_roundtrip() {
    let original = ManuallyDrop::new(42u32);
    let bytes = oxicode::encode_to_vec(&original).expect("encode ManuallyDrop<u32>");
    let (decoded, _): (ManuallyDrop<u32>, _) =
        oxicode::decode_from_slice(&bytes).expect("decode ManuallyDrop<u32>");
    assert_eq!(*original, *decoded);
}

#[test]
fn test_manually_drop_i64_roundtrip() {
    let original = ManuallyDrop::new(-9876543210i64);
    let bytes = oxicode::encode_to_vec(&original).expect("encode ManuallyDrop<i64>");
    let (decoded, _): (ManuallyDrop<i64>, _) =
        oxicode::decode_from_slice(&bytes).expect("decode ManuallyDrop<i64>");
    assert_eq!(*original, *decoded);
}

#[test]
fn test_manually_drop_bool_roundtrip() {
    let original = ManuallyDrop::new(true);
    let bytes = oxicode::encode_to_vec(&original).expect("encode ManuallyDrop<bool>");
    let (decoded, _): (ManuallyDrop<bool>, _) =
        oxicode::decode_from_slice(&bytes).expect("decode ManuallyDrop<bool>");
    assert_eq!(*original, *decoded);
}

#[test]
fn test_manually_drop_string_roundtrip() {
    let original = ManuallyDrop::new(String::from("hello atomics"));
    let bytes = oxicode::encode_to_vec(&original).expect("encode ManuallyDrop<String>");
    let (decoded, _): (ManuallyDrop<String>, _) =
        oxicode::decode_from_slice(&bytes).expect("decode ManuallyDrop<String>");
    assert_eq!(*original, *decoded);
}

// ===== PhantomData =====

#[test]
fn test_phantom_data_sized_roundtrip() {
    let original: PhantomData<u64> = PhantomData;
    let bytes = oxicode::encode_to_vec(&original).expect("encode PhantomData<u64>");
    assert_eq!(bytes.len(), 0, "PhantomData should encode to zero bytes");
    let (decoded, _): (PhantomData<u64>, _) =
        oxicode::decode_from_slice(&bytes).expect("decode PhantomData<u64>");
    // PhantomData is a ZST; just verify it round-trips
    let _: PhantomData<u64> = decoded;
}

#[test]
fn test_phantom_data_unsized_roundtrip() {
    let original: PhantomData<str> = PhantomData;
    let bytes = oxicode::encode_to_vec(&original).expect("encode PhantomData<str>");
    assert_eq!(
        bytes.len(),
        0,
        "PhantomData<str> should encode to zero bytes"
    );
    let (decoded, _): (PhantomData<str>, _) =
        oxicode::decode_from_slice(&bytes).expect("decode PhantomData<str>");
    let _: PhantomData<str> = decoded;
}

#[test]
fn test_phantom_data_slice_roundtrip() {
    let original: PhantomData<[u8]> = PhantomData;
    let bytes = oxicode::encode_to_vec(&original).expect("encode PhantomData<[u8]>");
    assert_eq!(
        bytes.len(),
        0,
        "PhantomData<[u8]> should encode to zero bytes"
    );
    let (decoded, _): (PhantomData<[u8]>, _) =
        oxicode::decode_from_slice(&bytes).expect("decode PhantomData<[u8]>");
    let _: PhantomData<[u8]> = decoded;
}
