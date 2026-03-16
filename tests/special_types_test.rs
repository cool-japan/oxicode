//! Tests for special types (NonZero, Range, Duration, etc.)

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
use std::{
    cell::{Cell, RefCell},
    num::{NonZeroU32, NonZeroU64, Wrapping},
    ops::{Bound, Range, RangeInclusive},
    sync::{Mutex, RwLock},
    time::{Duration, SystemTime, UNIX_EPOCH},
};

#[test]
fn test_nonzero_u32() {
    let original = NonZeroU32::new(42).expect("NonZero");
    let bytes = oxicode::encode_to_vec(&original).expect("Failed to encode");
    let (decoded, _): (NonZeroU32, _) =
        oxicode::decode_from_slice(&bytes).expect("Failed to decode");
    assert_eq!(original, decoded);
}

#[test]
fn test_nonzero_u64() {
    let original = NonZeroU64::new(999999).expect("NonZero");
    let bytes = oxicode::encode_to_vec(&original).expect("Failed to encode");
    let (decoded, _): (NonZeroU64, _) =
        oxicode::decode_from_slice(&bytes).expect("Failed to decode");
    assert_eq!(original, decoded);
}

#[test]
fn test_nonzero_decode_zero_fails() {
    // Manually encode zero as u32
    let zero_bytes = oxicode::encode_to_vec(&0u32).expect("Failed to encode");
    let result: Result<(NonZeroU32, usize), _> = oxicode::decode_from_slice(&zero_bytes);
    assert!(result.is_err(), "Should fail when decoding zero as NonZero");
}

#[test]
fn test_wrapping() {
    let original = Wrapping(u32::MAX);
    let bytes = oxicode::encode_to_vec(&original).expect("Failed to encode");
    let (decoded, _): (Wrapping<u32>, _) =
        oxicode::decode_from_slice(&bytes).expect("Failed to decode");
    assert_eq!(original, decoded);
}

#[test]
fn test_range() {
    let original = Range { start: 10, end: 20 };
    let bytes = oxicode::encode_to_vec(&original).expect("Failed to encode");
    let (decoded, _): (Range<i32>, _) =
        oxicode::decode_from_slice(&bytes).expect("Failed to decode");
    assert_eq!(original, decoded);
}

#[test]
fn test_range_inclusive() {
    let original = RangeInclusive::new(5, 15);
    let bytes = oxicode::encode_to_vec(&original).expect("Failed to encode");
    let (decoded, _): (RangeInclusive<i32>, _) =
        oxicode::decode_from_slice(&bytes).expect("Failed to decode");
    assert_eq!(original, decoded);
}

#[test]
fn test_bound_unbounded() {
    let original: Bound<u32> = Bound::Unbounded;
    let bytes = oxicode::encode_to_vec(&original).expect("Failed to encode");
    let (decoded, _): (Bound<u32>, _) =
        oxicode::decode_from_slice(&bytes).expect("Failed to decode");
    assert_eq!(original, decoded);
}

#[test]
fn test_bound_included() {
    let original: Bound<u32> = Bound::Included(42);
    let bytes = oxicode::encode_to_vec(&original).expect("Failed to encode");
    let (decoded, _): (Bound<u32>, _) =
        oxicode::decode_from_slice(&bytes).expect("Failed to decode");
    assert_eq!(original, decoded);
}

#[test]
fn test_bound_excluded() {
    let original: Bound<u32> = Bound::Excluded(99);
    let bytes = oxicode::encode_to_vec(&original).expect("Failed to encode");
    let (decoded, _): (Bound<u32>, _) =
        oxicode::decode_from_slice(&bytes).expect("Failed to decode");
    assert_eq!(original, decoded);
}

#[test]
fn test_duration() {
    let original = Duration::new(3600, 123456789);
    let bytes = oxicode::encode_to_vec(&original).expect("Failed to encode");
    let (decoded, _): (Duration, _) = oxicode::decode_from_slice(&bytes).expect("Failed to decode");
    assert_eq!(original, decoded);
}

#[test]
fn test_duration_zero() {
    let original = Duration::from_secs(0);
    let bytes = oxicode::encode_to_vec(&original).expect("Failed to encode");
    let (decoded, _): (Duration, _) = oxicode::decode_from_slice(&bytes).expect("Failed to decode");
    assert_eq!(original, decoded);
}

#[test]
fn test_duration_max() {
    let original = Duration::new(u64::MAX, 999_999_999);
    let bytes = oxicode::encode_to_vec(&original).expect("Failed to encode");
    let (decoded, _): (Duration, _) = oxicode::decode_from_slice(&bytes).expect("Failed to decode");
    assert_eq!(original, decoded);
}

#[test]
fn test_systemtime() {
    let original = UNIX_EPOCH + Duration::new(1000000, 500000000);
    let bytes = oxicode::encode_to_vec(&original).expect("Failed to encode");
    let (decoded, _): (SystemTime, _) =
        oxicode::decode_from_slice(&bytes).expect("Failed to decode");
    assert_eq!(original, decoded);
}

#[test]
fn test_systemtime_before_epoch() {
    let original = UNIX_EPOCH - Duration::new(100, 0);
    let bytes = oxicode::encode_to_vec(&original).expect("Failed to encode");
    let (decoded, _): (SystemTime, _) =
        oxicode::decode_from_slice(&bytes).expect("Failed to decode");
    assert_eq!(original, decoded);
}

#[test]
fn test_cell() {
    let original = Cell::new(42u32);
    let bytes = oxicode::encode_to_vec(&original).expect("Failed to encode");
    let (decoded, _): (Cell<u32>, _) =
        oxicode::decode_from_slice(&bytes).expect("Failed to decode");
    assert_eq!(original.get(), decoded.get());
}

#[test]
fn test_refcell() {
    let original = RefCell::new(vec![1, 2, 3]);
    let bytes = oxicode::encode_to_vec(&original).expect("Failed to encode");
    let (decoded, _): (RefCell<Vec<i32>>, _) =
        oxicode::decode_from_slice(&bytes).expect("Failed to decode");
    assert_eq!(*original.borrow(), *decoded.borrow());
}

#[test]
fn test_mutex() {
    let original = Mutex::new(String::from("locked data"));
    let bytes = oxicode::encode_to_vec(&original).expect("Failed to encode");
    let (decoded, _): (Mutex<String>, _) =
        oxicode::decode_from_slice(&bytes).expect("Failed to decode");
    assert_eq!(
        *original.lock().expect("Lock"),
        *decoded.lock().expect("Lock")
    );
}

#[test]
fn test_rwlock() {
    let original = RwLock::new(99u64);
    let bytes = oxicode::encode_to_vec(&original).expect("Failed to encode");
    let (decoded, _): (RwLock<u64>, _) =
        oxicode::decode_from_slice(&bytes).expect("Failed to decode");
    assert_eq!(
        *original.read().expect("Read"),
        *decoded.read().expect("Read")
    );
}

#[test]
#[cfg(target_has_atomic = "ptr")]
fn test_atomic_usize() {
    use std::sync::atomic::{AtomicUsize, Ordering};
    let original = AtomicUsize::new(12345);
    let bytes = oxicode::encode_to_vec(&original).expect("Failed to encode");
    let (decoded, _): (AtomicUsize, _) =
        oxicode::decode_from_slice(&bytes).expect("Failed to decode");
    assert_eq!(
        original.load(Ordering::Relaxed),
        decoded.load(Ordering::Relaxed)
    );
}

#[test]
#[cfg(target_has_atomic = "8")]
fn test_atomic_bool() {
    use std::sync::atomic::{AtomicBool, Ordering};
    let original = AtomicBool::new(true);
    let bytes = oxicode::encode_to_vec(&original).expect("Failed to encode");
    let (decoded, _): (AtomicBool, _) =
        oxicode::decode_from_slice(&bytes).expect("Failed to decode");
    assert_eq!(
        original.load(Ordering::Relaxed),
        decoded.load(Ordering::Relaxed)
    );
}

#[test]
fn test_ipaddr_v4() {
    use std::net::IpAddr;
    let original = "192.168.1.1".parse::<IpAddr>().expect("Parse IP");
    let bytes = oxicode::encode_to_vec(&original).expect("Failed to encode");
    let (decoded, _): (IpAddr, _) = oxicode::decode_from_slice(&bytes).expect("Failed to decode");
    assert_eq!(original, decoded);
}

#[test]
fn test_ipaddr_v6() {
    use std::net::IpAddr;
    let original = "::1".parse::<IpAddr>().expect("Parse IP");
    let bytes = oxicode::encode_to_vec(&original).expect("Failed to encode");
    let (decoded, _): (IpAddr, _) = oxicode::decode_from_slice(&bytes).expect("Failed to decode");
    assert_eq!(original, decoded);
}

#[test]
fn test_socketaddr() {
    use std::net::SocketAddr;
    let original = "127.0.0.1:8080".parse::<SocketAddr>().expect("Parse");
    let bytes = oxicode::encode_to_vec(&original).expect("Failed to encode");
    let (decoded, _): (SocketAddr, _) =
        oxicode::decode_from_slice(&bytes).expect("Failed to decode");
    assert_eq!(original, decoded);
}

#[test]
fn test_path() {
    use std::path::PathBuf;
    let original = PathBuf::from("/usr/local/bin/oxicode");
    let bytes = oxicode::encode_to_vec(&original).expect("Failed to encode");
    let (decoded, _): (PathBuf, _) = oxicode::decode_from_slice(&bytes).expect("Failed to decode");
    assert_eq!(original, decoded);
}

#[test]
fn test_cstring() {
    use std::ffi::CString;
    let original = CString::new("hello world").expect("CString");
    let bytes = oxicode::encode_to_vec(&original).expect("Failed to encode");
    let (decoded, _): (CString, _) = oxicode::decode_from_slice(&bytes).expect("Failed to decode");
    assert_eq!(original, decoded);
}

#[test]
#[cfg(not(target_family = "wasm"))]
fn test_osstring_roundtrip() {
    use std::ffi::OsString;
    let original = OsString::from("hello/world");
    let enc = oxicode::encode_to_vec(&original).expect("encode");
    let (dec, _): (OsString, _) = oxicode::decode_from_slice(&enc).expect("decode");
    assert_eq!(original, dec);
}

#[test]
#[cfg(not(target_family = "wasm"))]
fn test_osstring_empty() {
    use std::ffi::OsString;
    let original = OsString::from("");
    let enc = oxicode::encode_to_vec(&original).expect("encode");
    let (dec, _): (OsString, _) = oxicode::decode_from_slice(&enc).expect("decode");
    assert_eq!(original, dec);
}
