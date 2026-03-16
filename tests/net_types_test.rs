//! Tests for network and time type encode/decode implementations.

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
use core::cmp::Reverse;
use core::num::Wrapping;
use oxicode::{decode_from_slice, encode_to_vec};
use std::net::{IpAddr, Ipv4Addr, Ipv6Addr, SocketAddr, SocketAddrV4, SocketAddrV6};
use std::time::{Duration, SystemTime, UNIX_EPOCH};

// ===== Duration =====

#[test]
fn test_duration_zero_roundtrip() {
    let d = Duration::ZERO;
    let enc = encode_to_vec(&d).expect("encode Duration::ZERO");
    let (dec, _): (Duration, _) = decode_from_slice(&enc).expect("decode Duration::ZERO");
    assert_eq!(d, dec);
}

#[test]
fn test_duration_zero_wire_format() {
    // Duration::ZERO = (secs=0, nanos=0); both encode as varint 0 = single byte 0x00 each
    let d = Duration::ZERO;
    let enc = encode_to_vec(&d).expect("encode");
    assert_eq!(
        enc,
        &[0x00, 0x00],
        "Duration::ZERO should encode as 2 zero bytes"
    );
}

#[test]
fn test_duration_typical_roundtrip() {
    let d = Duration::new(3600, 500_000_000);
    let enc = encode_to_vec(&d).expect("encode");
    let (dec, _): (Duration, _) = decode_from_slice(&enc).expect("decode");
    assert_eq!(d, dec);
}

#[test]
fn test_duration_max_roundtrip() {
    let d = Duration::new(u64::MAX, 999_999_999);
    let enc = encode_to_vec(&d).expect("encode");
    let (dec, _): (Duration, _) = decode_from_slice(&enc).expect("decode");
    assert_eq!(d, dec);
}

#[test]
fn test_duration_subsec_only_roundtrip() {
    let d = Duration::from_nanos(123_456_789);
    let enc = encode_to_vec(&d).expect("encode");
    let (dec, _): (Duration, _) = decode_from_slice(&enc).expect("decode");
    assert_eq!(d, dec);
}

// ===== SystemTime =====

#[test]
fn test_systemtime_unix_epoch_roundtrip() {
    let t = UNIX_EPOCH;
    let enc = encode_to_vec(&t).expect("encode");
    let (dec, _): (SystemTime, _) = decode_from_slice(&enc).expect("decode");
    assert_eq!(t, dec);
}

#[test]
fn test_systemtime_post_epoch_roundtrip() {
    let t = UNIX_EPOCH + Duration::new(1_700_000_000, 0);
    let enc = encode_to_vec(&t).expect("encode");
    let (dec, _): (SystemTime, _) = decode_from_slice(&enc).expect("decode");
    assert_eq!(t, dec);
}

#[test]
fn test_systemtime_pre_epoch_roundtrip() {
    // Before UNIX_EPOCH: duration_since returns Err, encoded with variant 1
    let t = UNIX_EPOCH - Duration::new(86400, 0);
    let enc = encode_to_vec(&t).expect("encode");
    let (dec, _): (SystemTime, _) = decode_from_slice(&enc).expect("decode");
    assert_eq!(t, dec);
}

// ===== Ipv4Addr =====

#[test]
fn test_ipv4addr_localhost_roundtrip() {
    let ip = Ipv4Addr::LOCALHOST;
    let enc = encode_to_vec(&ip).expect("encode");
    let (dec, _): (Ipv4Addr, _) = decode_from_slice(&enc).expect("decode");
    assert_eq!(ip, dec);
}

#[test]
fn test_ipv4addr_wire_format() {
    // Ipv4Addr is encoded as 4 raw bytes (no varint), so 127.0.0.1 = [127, 0, 0, 1]
    let ip = Ipv4Addr::new(127, 0, 0, 1);
    let enc = encode_to_vec(&ip).expect("encode");
    assert_eq!(enc, &[127, 0, 0, 1], "Ipv4Addr must encode as 4 raw octets");
}

#[test]
fn test_ipv4addr_broadcast_roundtrip() {
    let ip = Ipv4Addr::BROADCAST;
    let enc = encode_to_vec(&ip).expect("encode");
    let (dec, _): (Ipv4Addr, _) = decode_from_slice(&enc).expect("decode");
    assert_eq!(ip, dec);
}

#[test]
fn test_ipv4addr_arbitrary_roundtrip() {
    let ip = Ipv4Addr::new(192, 168, 1, 100);
    let enc = encode_to_vec(&ip).expect("encode");
    let (dec, _): (Ipv4Addr, _) = decode_from_slice(&enc).expect("decode");
    assert_eq!(ip, dec);
}

// ===== Ipv6Addr =====

#[test]
fn test_ipv6addr_localhost_roundtrip() {
    let ip = Ipv6Addr::LOCALHOST;
    let enc = encode_to_vec(&ip).expect("encode");
    let (dec, _): (Ipv6Addr, _) = decode_from_slice(&enc).expect("decode");
    assert_eq!(ip, dec);
}

#[test]
fn test_ipv6addr_wire_format() {
    // Ipv6Addr is encoded as 16 raw bytes
    let ip = Ipv6Addr::LOCALHOST;
    let enc = encode_to_vec(&ip).expect("encode");
    assert_eq!(enc.len(), 16, "Ipv6Addr must encode as 16 raw octets");
    assert_eq!(&enc[..15], &[0u8; 15]);
    assert_eq!(enc[15], 1u8);
}

#[test]
fn test_ipv6addr_arbitrary_roundtrip() {
    let ip = Ipv6Addr::new(0x2001, 0x0db8, 0x85a3, 0, 0, 0x8a2e, 0x0370, 0x7334);
    let enc = encode_to_vec(&ip).expect("encode");
    let (dec, _): (Ipv6Addr, _) = decode_from_slice(&enc).expect("decode");
    assert_eq!(ip, dec);
}

// ===== IpAddr =====

#[test]
fn test_ipaddr_v4_roundtrip() {
    let ip = IpAddr::V4(Ipv4Addr::new(10, 0, 0, 1));
    let enc = encode_to_vec(&ip).expect("encode");
    let (dec, _): (IpAddr, _) = decode_from_slice(&enc).expect("decode");
    assert_eq!(ip, dec);
}

#[test]
fn test_ipaddr_v6_roundtrip() {
    let ip = IpAddr::V6(Ipv6Addr::LOCALHOST);
    let enc = encode_to_vec(&ip).expect("encode");
    let (dec, _): (IpAddr, _) = decode_from_slice(&enc).expect("decode");
    assert_eq!(ip, dec);
}

#[test]
fn test_ipaddr_v4_discriminant() {
    // IpAddr::V4 encodes with discriminant 0u8 followed by 4 octets
    let ip = IpAddr::V4(Ipv4Addr::new(1, 2, 3, 4));
    let enc = encode_to_vec(&ip).expect("encode");
    assert_eq!(enc[0], 0u8, "IpAddr::V4 discriminant must be 0");
    assert_eq!(enc.len(), 5, "IpAddr::V4 must be 1 discriminant + 4 octets");
}

#[test]
fn test_ipaddr_v6_discriminant() {
    // IpAddr::V6 encodes with discriminant 1u8 followed by 16 octets
    let ip = IpAddr::V6(Ipv6Addr::LOCALHOST);
    let enc = encode_to_vec(&ip).expect("encode");
    assert_eq!(enc[0], 1u8, "IpAddr::V6 discriminant must be 1");
    assert_eq!(
        enc.len(),
        17,
        "IpAddr::V6 must be 1 discriminant + 16 octets"
    );
}

// ===== SocketAddrV4 =====

#[test]
fn test_socketaddrv4_roundtrip() {
    let addr = SocketAddrV4::new(Ipv4Addr::new(127, 0, 0, 1), 8080);
    let enc = encode_to_vec(&addr).expect("encode");
    let (dec, _): (SocketAddrV4, _) = decode_from_slice(&enc).expect("decode");
    assert_eq!(addr, dec);
}

#[test]
fn test_socketaddrv4_zero_port_roundtrip() {
    let addr = SocketAddrV4::new(Ipv4Addr::UNSPECIFIED, 0);
    let enc = encode_to_vec(&addr).expect("encode");
    let (dec, _): (SocketAddrV4, _) = decode_from_slice(&enc).expect("decode");
    assert_eq!(addr, dec);
}

#[test]
fn test_socketaddrv4_max_port_roundtrip() {
    let addr = SocketAddrV4::new(Ipv4Addr::new(192, 168, 0, 1), 65535);
    let enc = encode_to_vec(&addr).expect("encode");
    let (dec, _): (SocketAddrV4, _) = decode_from_slice(&enc).expect("decode");
    assert_eq!(addr, dec);
}

// ===== SocketAddrV6 =====

#[test]
fn test_socketaddrv6_roundtrip() {
    let addr = SocketAddrV6::new(Ipv6Addr::LOCALHOST, 443, 0, 0);
    let enc = encode_to_vec(&addr).expect("encode");
    let (dec, _): (SocketAddrV6, _) = decode_from_slice(&enc).expect("decode");
    assert_eq!(addr, dec);
}

#[test]
fn test_socketaddrv6_with_flowinfo_scope_roundtrip() {
    let addr = SocketAddrV6::new(
        Ipv6Addr::new(0xfe80, 0, 0, 0, 0, 0, 0, 1),
        9000,
        0x12345678,
        42,
    );
    let enc = encode_to_vec(&addr).expect("encode");
    let (dec, _): (SocketAddrV6, _) = decode_from_slice(&enc).expect("decode");
    assert_eq!(addr, dec);
}

// ===== SocketAddr =====

#[test]
fn test_socketaddr_v4_roundtrip() {
    let addr = SocketAddr::V4(SocketAddrV4::new(Ipv4Addr::new(127, 0, 0, 1), 3000));
    let enc = encode_to_vec(&addr).expect("encode");
    let (dec, _): (SocketAddr, _) = decode_from_slice(&enc).expect("decode");
    assert_eq!(addr, dec);
}

#[test]
fn test_socketaddr_v6_roundtrip() {
    let addr = SocketAddr::V6(SocketAddrV6::new(Ipv6Addr::LOCALHOST, 8443, 0, 0));
    let enc = encode_to_vec(&addr).expect("encode");
    let (dec, _): (SocketAddr, _) = decode_from_slice(&enc).expect("decode");
    assert_eq!(addr, dec);
}

#[test]
fn test_socketaddr_v4_discriminant() {
    let addr = SocketAddr::V4(SocketAddrV4::new(Ipv4Addr::new(1, 2, 3, 4), 80));
    let enc = encode_to_vec(&addr).expect("encode");
    assert_eq!(enc[0], 0u8, "SocketAddr::V4 discriminant must be 0");
}

#[test]
fn test_socketaddr_v6_discriminant() {
    let addr = SocketAddr::V6(SocketAddrV6::new(Ipv6Addr::LOCALHOST, 80, 0, 0));
    let enc = encode_to_vec(&addr).expect("encode");
    assert_eq!(enc[0], 1u8, "SocketAddr::V6 discriminant must be 1");
}

// ===== Wrapping<T> =====

#[test]
fn test_wrapping_u32_roundtrip() {
    let w = Wrapping(42u32);
    let enc = encode_to_vec(&w).expect("encode");
    let (dec, _): (Wrapping<u32>, _) = decode_from_slice(&enc).expect("decode");
    assert_eq!(w, dec);
}

#[test]
fn test_wrapping_u32_max_roundtrip() {
    let w = Wrapping(u32::MAX);
    let enc = encode_to_vec(&w).expect("encode");
    let (dec, _): (Wrapping<u32>, _) = decode_from_slice(&enc).expect("decode");
    assert_eq!(w, dec);
}

#[test]
fn test_wrapping_i64_negative_roundtrip() {
    let w = Wrapping(-1i64);
    let enc = encode_to_vec(&w).expect("encode");
    let (dec, _): (Wrapping<i64>, _) = decode_from_slice(&enc).expect("decode");
    assert_eq!(w, dec);
}

#[test]
fn test_wrapping_zero_roundtrip() {
    let w = Wrapping(0u64);
    let enc = encode_to_vec(&w).expect("encode");
    let (dec, _): (Wrapping<u64>, _) = decode_from_slice(&enc).expect("decode");
    assert_eq!(w, dec);
}

// ===== Reverse<T> =====

#[test]
fn test_reverse_u32_roundtrip() {
    let r = Reverse(100u32);
    let enc = encode_to_vec(&r).expect("encode");
    let (dec, _): (Reverse<u32>, _) = decode_from_slice(&enc).expect("decode");
    assert_eq!(r, dec);
}

#[test]
fn test_reverse_string_roundtrip() {
    let r = Reverse("hello".to_string());
    let enc = encode_to_vec(&r).expect("encode");
    let (dec, _): (Reverse<String>, _) = decode_from_slice(&enc).expect("decode");
    assert_eq!(r, dec);
}
