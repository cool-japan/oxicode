//! Roundtrip tests for extra std type implementations:
//! PathBuf, SystemTime, Range, RangeInclusive, Bound, Cell, RefCell,
//! Mutex, RwLock, Duration, IpAddr/SocketAddr families.

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
use std::cell::{Cell, RefCell};
use std::net::{IpAddr, Ipv4Addr, Ipv6Addr, SocketAddr, SocketAddrV4, SocketAddrV6};
use std::num::Wrapping;
use std::ops::Bound;
use std::path::PathBuf;
use std::sync::{Mutex, RwLock};
use std::time::{Duration, SystemTime, UNIX_EPOCH};

// ===== PathBuf =====

#[test]
fn test_pathbuf_roundtrip_absolute() {
    let path = PathBuf::from("/tmp/test.txt");
    let encoded = encode_to_vec(&path).expect("encode");
    let (decoded, _): (PathBuf, _) = decode_from_slice(&encoded).expect("decode");
    assert_eq!(path, decoded);
}

#[test]
fn test_pathbuf_roundtrip_relative() {
    let path = PathBuf::from("foo/bar/baz");
    let encoded = encode_to_vec(&path).expect("encode");
    let (decoded, _): (PathBuf, _) = decode_from_slice(&encoded).expect("decode");
    assert_eq!(path, decoded);
}

#[test]
fn test_pathbuf_roundtrip_with_spaces() {
    let path = PathBuf::from("/home/user/my documents/report.pdf");
    let encoded = encode_to_vec(&path).expect("encode");
    let (decoded, _): (PathBuf, _) = decode_from_slice(&encoded).expect("decode");
    assert_eq!(path, decoded);
}

#[test]
fn test_pathbuf_roundtrip_empty() {
    let path = PathBuf::from("");
    let encoded = encode_to_vec(&path).expect("encode");
    let (decoded, _): (PathBuf, _) = decode_from_slice(&encoded).expect("decode");
    assert_eq!(path, decoded);
}

#[test]
fn test_pathbuf_roundtrip_deeply_nested() {
    let path = PathBuf::from("/a/b/c/d/e/f/g/h/i/j.rs");
    let encoded = encode_to_vec(&path).expect("encode");
    let (decoded, _): (PathBuf, _) = decode_from_slice(&encoded).expect("decode");
    assert_eq!(path, decoded);
}

// ===== Duration =====

#[test]
fn test_duration_roundtrip_zero() {
    let dur = Duration::ZERO;
    let encoded = encode_to_vec(&dur).expect("encode");
    let (decoded, _): (Duration, _) = decode_from_slice(&encoded).expect("decode");
    assert_eq!(dur, decoded);
}

#[test]
fn test_duration_roundtrip_secs_and_nanos() {
    let dur = Duration::new(12345, 999_999_999);
    let encoded = encode_to_vec(&dur).expect("encode");
    let (decoded, _): (Duration, _) = decode_from_slice(&encoded).expect("decode");
    assert_eq!(dur, decoded);
}

#[test]
fn test_duration_roundtrip_subsecond() {
    let dur = Duration::from_nanos(123_456_789);
    let encoded = encode_to_vec(&dur).expect("encode");
    let (decoded, _): (Duration, _) = decode_from_slice(&encoded).expect("decode");
    assert_eq!(dur, decoded);
}

#[test]
fn test_duration_roundtrip_large() {
    // ~10 years in seconds
    let dur = Duration::from_secs(315_360_000);
    let encoded = encode_to_vec(&dur).expect("encode");
    let (decoded, _): (Duration, _) = decode_from_slice(&encoded).expect("decode");
    assert_eq!(dur, decoded);
}

// ===== SystemTime =====

#[test]
fn test_systemtime_roundtrip_unix_epoch() {
    let t = UNIX_EPOCH;
    let encoded = encode_to_vec(&t).expect("encode");
    let (decoded, _): (SystemTime, _) = decode_from_slice(&encoded).expect("decode");
    assert_eq!(t, decoded);
}

#[test]
fn test_systemtime_roundtrip_after_epoch() {
    let t = UNIX_EPOCH + Duration::from_secs(1_000_000_000);
    let encoded = encode_to_vec(&t).expect("encode");
    let (decoded, _): (SystemTime, _) = decode_from_slice(&encoded).expect("decode");
    assert_eq!(t, decoded);
}

#[test]
fn test_systemtime_roundtrip_before_epoch() {
    // SystemTime can represent times before UNIX_EPOCH
    let t = UNIX_EPOCH - Duration::from_secs(3600);
    let encoded = encode_to_vec(&t).expect("encode");
    let (decoded, _): (SystemTime, _) = decode_from_slice(&encoded).expect("decode");
    assert_eq!(t, decoded);
}

#[test]
fn test_systemtime_roundtrip_now() {
    let t = SystemTime::now();
    let encoded = encode_to_vec(&t).expect("encode");
    let (decoded, _): (SystemTime, _) = decode_from_slice(&encoded).expect("decode");
    assert_eq!(t, decoded);
}

#[test]
fn test_systemtime_roundtrip_with_nanos() {
    // A time with subsecond precision
    let t = UNIX_EPOCH + Duration::new(1_700_000_000, 123_456_789);
    let encoded = encode_to_vec(&t).expect("encode");
    let (decoded, _): (SystemTime, _) = decode_from_slice(&encoded).expect("decode");
    assert_eq!(t, decoded);
}

// ===== Range<T> =====

#[test]
fn test_range_u32_roundtrip() {
    let r = 0u32..100u32;
    let encoded = encode_to_vec(&r).expect("encode");
    let (decoded, _): (std::ops::Range<u32>, _) = decode_from_slice(&encoded).expect("decode");
    assert_eq!(r, decoded);
}

#[test]
fn test_range_i64_roundtrip() {
    let r = -50i64..50i64;
    let encoded = encode_to_vec(&r).expect("encode");
    let (decoded, _): (std::ops::Range<i64>, _) = decode_from_slice(&encoded).expect("decode");
    assert_eq!(r, decoded);
}

#[test]
fn test_range_empty_roundtrip() {
    let r = 42u32..42u32;
    let encoded = encode_to_vec(&r).expect("encode");
    let (decoded, _): (std::ops::Range<u32>, _) = decode_from_slice(&encoded).expect("decode");
    assert_eq!(r, decoded);
}

#[test]
fn test_range_string_roundtrip() {
    let r = "apple".to_string().."mango".to_string();
    let encoded = encode_to_vec(&r).expect("encode");
    let (decoded, _): (std::ops::Range<String>, _) = decode_from_slice(&encoded).expect("decode");
    assert_eq!(r, decoded);
}

// ===== RangeInclusive<T> =====

#[test]
fn test_range_inclusive_u32_roundtrip() {
    let r = 0u32..=255u32;
    let encoded = encode_to_vec(&r).expect("encode");
    let (decoded, _): (std::ops::RangeInclusive<u32>, _) =
        decode_from_slice(&encoded).expect("decode");
    assert_eq!(r, decoded);
}

#[test]
fn test_range_inclusive_f64_roundtrip() {
    let r = 0.0f64..=1.0f64;
    let encoded = encode_to_vec(&r).expect("encode");
    let (decoded, _): (std::ops::RangeInclusive<f64>, _) =
        decode_from_slice(&encoded).expect("decode");
    assert_eq!(r, decoded);
}

#[test]
fn test_range_inclusive_single_element() {
    let r = 7u64..=7u64;
    let encoded = encode_to_vec(&r).expect("encode");
    let (decoded, _): (std::ops::RangeInclusive<u64>, _) =
        decode_from_slice(&encoded).expect("decode");
    assert_eq!(r, decoded);
}

// ===== Bound<T> =====

#[test]
fn test_bound_unbounded_roundtrip() {
    let b: Bound<u32> = Bound::Unbounded;
    let encoded = encode_to_vec(&b).expect("encode");
    let (decoded, _): (Bound<u32>, _) = decode_from_slice(&encoded).expect("decode");
    assert_eq!(b, decoded);
}

#[test]
fn test_bound_included_roundtrip() {
    let b: Bound<u32> = Bound::Included(42);
    let encoded = encode_to_vec(&b).expect("encode");
    let (decoded, _): (Bound<u32>, _) = decode_from_slice(&encoded).expect("decode");
    assert_eq!(b, decoded);
}

#[test]
fn test_bound_excluded_roundtrip() {
    let b: Bound<u32> = Bound::Excluded(99);
    let encoded = encode_to_vec(&b).expect("encode");
    let (decoded, _): (Bound<u32>, _) = decode_from_slice(&encoded).expect("decode");
    assert_eq!(b, decoded);
}

#[test]
fn test_bound_included_string_roundtrip() {
    let b: Bound<String> = Bound::Included("hello".to_string());
    let encoded = encode_to_vec(&b).expect("encode");
    let (decoded, _): (Bound<String>, _) = decode_from_slice(&encoded).expect("decode");
    assert_eq!(b, decoded);
}

#[test]
fn test_bound_wire_format_variants() {
    // Verify the wire format discriminants: 0=Unbounded, 1=Included, 2=Excluded
    let unbounded_enc = encode_to_vec(&Bound::<u8>::Unbounded).expect("encode");
    assert_eq!(unbounded_enc[0], 0, "Unbounded should have discriminant 0");

    let included_enc = encode_to_vec(&Bound::Included(0u8)).expect("encode");
    assert_eq!(included_enc[0], 1, "Included should have discriminant 1");

    let excluded_enc = encode_to_vec(&Bound::Excluded(0u8)).expect("encode");
    assert_eq!(excluded_enc[0], 2, "Excluded should have discriminant 2");
}

// ===== Cell<T> =====

#[test]
fn test_cell_u32_roundtrip() {
    let cell = Cell::new(99u32);
    let encoded = encode_to_vec(&cell).expect("encode");
    let (decoded, _): (Cell<u32>, _) = decode_from_slice(&encoded).expect("decode");
    assert_eq!(cell.get(), decoded.get());
}

#[test]
fn test_cell_bool_roundtrip() {
    for val in [true, false] {
        let cell = Cell::new(val);
        let encoded = encode_to_vec(&cell).expect("encode");
        let (decoded, _): (Cell<bool>, _) = decode_from_slice(&encoded).expect("decode");
        assert_eq!(cell.get(), decoded.get());
    }
}

#[test]
fn test_cell_i64_roundtrip() {
    let cell = Cell::new(-987654321i64);
    let encoded = encode_to_vec(&cell).expect("encode");
    let (decoded, _): (Cell<i64>, _) = decode_from_slice(&encoded).expect("decode");
    assert_eq!(cell.get(), decoded.get());
}

// ===== RefCell<T> =====

#[test]
fn test_refcell_string_roundtrip() {
    let rc = RefCell::new("hello world".to_string());
    let encoded = encode_to_vec(&rc).expect("encode");
    let (decoded, _): (RefCell<String>, _) = decode_from_slice(&encoded).expect("decode");
    assert_eq!(*rc.borrow(), *decoded.borrow());
}

#[test]
fn test_refcell_vec_roundtrip() {
    let rc = RefCell::new(vec![1u32, 2, 3, 4, 5]);
    let encoded = encode_to_vec(&rc).expect("encode");
    let (decoded, _): (RefCell<Vec<u32>>, _) = decode_from_slice(&encoded).expect("decode");
    assert_eq!(*rc.borrow(), *decoded.borrow());
}

#[test]
fn test_refcell_nested_roundtrip() {
    let rc: RefCell<Option<u64>> = RefCell::new(Some(42));
    let encoded = encode_to_vec(&rc).expect("encode");
    let (decoded, _): (RefCell<Option<u64>>, _) = decode_from_slice(&encoded).expect("decode");
    assert_eq!(*rc.borrow(), *decoded.borrow());
}

// ===== Mutex<T> =====

#[test]
fn test_mutex_u32_roundtrip() {
    let m = Mutex::new(42u32);
    let encoded = encode_to_vec(&m).expect("encode");
    let (decoded, _): (Mutex<u32>, _) = decode_from_slice(&encoded).expect("decode");
    let orig = *m.lock().expect("lock");
    let dec = *decoded.lock().expect("lock");
    assert_eq!(orig, dec);
}

#[test]
fn test_mutex_string_roundtrip() {
    let m = Mutex::new("secure data".to_string());
    let encoded = encode_to_vec(&m).expect("encode");
    let (decoded, _): (Mutex<String>, _) = decode_from_slice(&encoded).expect("decode");
    let orig = m.lock().expect("lock").clone();
    let dec = decoded.lock().expect("lock").clone();
    assert_eq!(orig, dec);
}

// ===== RwLock<T> =====

#[test]
fn test_rwlock_u64_roundtrip() {
    let lock = RwLock::new(999u64);
    let encoded = encode_to_vec(&lock).expect("encode");
    let (decoded, _): (RwLock<u64>, _) = decode_from_slice(&encoded).expect("decode");
    let orig = *lock.read().expect("read");
    let dec = *decoded.read().expect("read");
    assert_eq!(orig, dec);
}

#[test]
fn test_rwlock_vec_roundtrip() {
    let lock = RwLock::new(vec![10u8, 20, 30]);
    let encoded = encode_to_vec(&lock).expect("encode");
    let (decoded, _): (RwLock<Vec<u8>>, _) = decode_from_slice(&encoded).expect("decode");
    let orig = lock.read().expect("read").clone();
    let dec = decoded.read().expect("read").clone();
    assert_eq!(orig, dec);
}

// ===== IpAddr =====

#[test]
fn test_ipv4_roundtrip() {
    let addr = Ipv4Addr::new(192, 168, 1, 1);
    let encoded = encode_to_vec(&addr).expect("encode");
    let (decoded, _): (Ipv4Addr, _) = decode_from_slice(&encoded).expect("decode");
    assert_eq!(addr, decoded);
}

#[test]
fn test_ipv6_roundtrip() {
    let addr = Ipv6Addr::new(0x2001, 0x0db8, 0x85a3, 0, 0, 0x8a2e, 0x0370, 0x7334);
    let encoded = encode_to_vec(&addr).expect("encode");
    let (decoded, _): (Ipv6Addr, _) = decode_from_slice(&encoded).expect("decode");
    assert_eq!(addr, decoded);
}

#[test]
fn test_ipv6_localhost_roundtrip() {
    let addr = Ipv6Addr::LOCALHOST;
    let encoded = encode_to_vec(&addr).expect("encode");
    let (decoded, _): (Ipv6Addr, _) = decode_from_slice(&encoded).expect("decode");
    assert_eq!(addr, decoded);
}

#[test]
fn test_ipaddr_v4_roundtrip() {
    let addr = IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1));
    let encoded = encode_to_vec(&addr).expect("encode");
    let (decoded, _): (IpAddr, _) = decode_from_slice(&encoded).expect("decode");
    assert_eq!(addr, decoded);
}

#[test]
fn test_ipaddr_v6_roundtrip() {
    let addr = IpAddr::V6(Ipv6Addr::LOCALHOST);
    let encoded = encode_to_vec(&addr).expect("encode");
    let (decoded, _): (IpAddr, _) = decode_from_slice(&encoded).expect("decode");
    assert_eq!(addr, decoded);
}

#[test]
fn test_socketaddrv4_roundtrip() {
    let addr = SocketAddrV4::new(Ipv4Addr::new(10, 0, 0, 1), 8080);
    let encoded = encode_to_vec(&addr).expect("encode");
    let (decoded, _): (SocketAddrV4, _) = decode_from_slice(&encoded).expect("decode");
    assert_eq!(addr, decoded);
}

#[test]
fn test_socketaddrv6_roundtrip() {
    let addr = SocketAddrV6::new(Ipv6Addr::LOCALHOST, 443, 0, 0);
    let encoded = encode_to_vec(&addr).expect("encode");
    let (decoded, _): (SocketAddrV6, _) = decode_from_slice(&encoded).expect("decode");
    assert_eq!(addr, decoded);
}

#[test]
fn test_socketaddr_v4_roundtrip() {
    let addr = SocketAddr::V4(SocketAddrV4::new(Ipv4Addr::new(127, 0, 0, 1), 9000));
    let encoded = encode_to_vec(&addr).expect("encode");
    let (decoded, _): (SocketAddr, _) = decode_from_slice(&encoded).expect("decode");
    assert_eq!(addr, decoded);
}

#[test]
fn test_socketaddr_v6_roundtrip() {
    let addr = SocketAddr::V6(SocketAddrV6::new(Ipv6Addr::LOCALHOST, 9001, 0, 0));
    let encoded = encode_to_vec(&addr).expect("encode");
    let (decoded, _): (SocketAddr, _) = decode_from_slice(&encoded).expect("decode");
    assert_eq!(addr, decoded);
}

// ===== Wrapping<T> =====

#[test]
fn test_wrapping_u32_roundtrip() {
    let w = Wrapping(u32::MAX);
    let encoded = encode_to_vec(&w).expect("encode");
    let (decoded, _): (Wrapping<u32>, _) = decode_from_slice(&encoded).expect("decode");
    assert_eq!(w, decoded);
}

#[test]
fn test_wrapping_i64_roundtrip() {
    let w = Wrapping(i64::MIN);
    let encoded = encode_to_vec(&w).expect("encode");
    let (decoded, _): (Wrapping<i64>, _) = decode_from_slice(&encoded).expect("decode");
    assert_eq!(w, decoded);
}

// ===== std::cmp::Reverse<T> =====

#[test]
fn test_reverse_u32_roundtrip() {
    use std::cmp::Reverse;
    let r = Reverse(42u32);
    let encoded = encode_to_vec(&r).expect("encode");
    let (decoded, _): (Reverse<u32>, _) = decode_from_slice(&encoded).expect("decode");
    assert_eq!(r, decoded);
}

#[test]
fn test_reverse_string_roundtrip() {
    use std::cmp::Reverse;
    let r = Reverse("zebra".to_string());
    let encoded = encode_to_vec(&r).expect("encode");
    let (decoded, _): (Reverse<String>, _) = decode_from_slice(&encoded).expect("decode");
    assert_eq!(r, decoded);
}

// ===== Cross-type compound tests =====

#[test]
fn test_option_pathbuf_roundtrip() {
    let opt: Option<PathBuf> = Some(PathBuf::from("/etc/hosts"));
    let encoded = encode_to_vec(&opt).expect("encode");
    let (decoded, _): (Option<PathBuf>, _) = decode_from_slice(&encoded).expect("decode");
    assert_eq!(opt, decoded);
}

#[test]
fn test_vec_of_ranges_roundtrip() {
    let ranges: Vec<std::ops::Range<u32>> = vec![0..10, 20..30, 100..200];
    let encoded = encode_to_vec(&ranges).expect("encode");
    let (decoded, _): (Vec<std::ops::Range<u32>>, _) = decode_from_slice(&encoded).expect("decode");
    assert_eq!(ranges, decoded);
}

#[test]
fn test_option_systemtime_none_roundtrip() {
    let opt: Option<SystemTime> = None;
    let encoded = encode_to_vec(&opt).expect("encode");
    let (decoded, _): (Option<SystemTime>, _) = decode_from_slice(&encoded).expect("decode");
    assert_eq!(opt, decoded);
}

#[test]
fn test_option_systemtime_some_roundtrip() {
    let opt: Option<SystemTime> = Some(UNIX_EPOCH + Duration::from_secs(500_000));
    let encoded = encode_to_vec(&opt).expect("encode");
    let (decoded, _): (Option<SystemTime>, _) = decode_from_slice(&encoded).expect("decode");
    assert_eq!(opt, decoded);
}
