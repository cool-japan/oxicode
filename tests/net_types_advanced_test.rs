//! Advanced tests for network and time type encoding/decoding.
//! Covers Ipv4Addr, Ipv6Addr, SocketAddr variants, Duration, SystemTime,
//! composite struct, and Vec<SocketAddr> roundtrips.

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
use oxicode::{decode_from_slice, encode_to_vec, Decode, Encode};
use std::net::{IpAddr, Ipv4Addr, Ipv6Addr, SocketAddr, SocketAddrV4, SocketAddrV6};
use std::time::{Duration, SystemTime, UNIX_EPOCH};

// ===== Composite struct for test 19 =====

#[derive(Encode, Decode, PartialEq, Debug)]
struct NetworkRecord {
    addr: Ipv4Addr,
    timeout: Duration,
}

// ===== Ipv4Addr =====

#[test]
fn test_adv_ipv4_all_zeros_roundtrip() {
    let ip = Ipv4Addr::new(0, 0, 0, 0);
    let enc = encode_to_vec(&ip).expect("encode Ipv4Addr 0.0.0.0");
    let (dec, _): (Ipv4Addr, _) = decode_from_slice(&enc).expect("decode Ipv4Addr 0.0.0.0");
    assert_eq!(ip, dec);
}

#[test]
fn test_adv_ipv4_broadcast_roundtrip() {
    let ip = Ipv4Addr::new(255, 255, 255, 255);
    let enc = encode_to_vec(&ip).expect("encode Ipv4Addr 255.255.255.255");
    let (dec, _): (Ipv4Addr, _) = decode_from_slice(&enc).expect("decode Ipv4Addr 255.255.255.255");
    assert_eq!(ip, dec);
}

#[test]
fn test_adv_ipv4_loopback_roundtrip() {
    let ip = Ipv4Addr::new(127, 0, 0, 1);
    let enc = encode_to_vec(&ip).expect("encode Ipv4Addr 127.0.0.1");
    let (dec, _): (Ipv4Addr, _) = decode_from_slice(&enc).expect("decode Ipv4Addr 127.0.0.1");
    assert_eq!(ip, dec);
}

// ===== Ipv6Addr =====

#[test]
fn test_adv_ipv6_loopback_roundtrip() {
    let ip = Ipv6Addr::new(0, 0, 0, 0, 0, 0, 0, 1);
    let enc = encode_to_vec(&ip).expect("encode Ipv6Addr ::1");
    let (dec, _): (Ipv6Addr, _) = decode_from_slice(&enc).expect("decode Ipv6Addr ::1");
    assert_eq!(ip, dec);
}

#[test]
fn test_adv_ipv6_all_zeros_roundtrip() {
    let ip = Ipv6Addr::new(0, 0, 0, 0, 0, 0, 0, 0);
    let enc = encode_to_vec(&ip).expect("encode Ipv6Addr ::");
    let (dec, _): (Ipv6Addr, _) = decode_from_slice(&enc).expect("decode Ipv6Addr ::");
    assert_eq!(ip, dec);
}

#[test]
fn test_adv_ipv6_all_ones_roundtrip() {
    let ip = Ipv6Addr::new(
        0xffff, 0xffff, 0xffff, 0xffff, 0xffff, 0xffff, 0xffff, 0xffff,
    );
    let enc = encode_to_vec(&ip).expect("encode Ipv6Addr ffff:...");
    let (dec, _): (Ipv6Addr, _) = decode_from_slice(&enc).expect("decode Ipv6Addr ffff:...");
    assert_eq!(ip, dec);
}

// ===== SocketAddr =====

#[test]
fn test_adv_socketaddr_v4_port_zero_roundtrip() {
    let addr = SocketAddr::V4(SocketAddrV4::new(Ipv4Addr::new(10, 0, 0, 1), 0));
    let enc = encode_to_vec(&addr).expect("encode SocketAddr V4 port 0");
    let (dec, _): (SocketAddr, _) = decode_from_slice(&enc).expect("decode SocketAddr V4 port 0");
    assert_eq!(addr, dec);
}

#[test]
fn test_adv_socketaddr_v4_port_max_roundtrip() {
    let addr = SocketAddr::V4(SocketAddrV4::new(Ipv4Addr::new(10, 0, 0, 1), 65535));
    let enc = encode_to_vec(&addr).expect("encode SocketAddr V4 port 65535");
    let (dec, _): (SocketAddr, _) =
        decode_from_slice(&enc).expect("decode SocketAddr V4 port 65535");
    assert_eq!(addr, dec);
}

#[test]
fn test_adv_socketaddr_v6_port_8080_roundtrip() {
    let addr = SocketAddr::V6(SocketAddrV6::new(
        Ipv6Addr::new(0x2001, 0xdb8, 0, 0, 0, 0, 0, 1),
        8080,
        0,
        0,
    ));
    let enc = encode_to_vec(&addr).expect("encode SocketAddr V6 port 8080");
    let (dec, _): (SocketAddr, _) =
        decode_from_slice(&enc).expect("decode SocketAddr V6 port 8080");
    assert_eq!(addr, dec);
}

// ===== SocketAddrV4 / SocketAddrV6 specific =====

#[test]
fn test_adv_socketaddrv4_specific_roundtrip() {
    let addr = SocketAddrV4::new(Ipv4Addr::new(172, 16, 0, 1), 9090);
    let enc = encode_to_vec(&addr).expect("encode SocketAddrV4 172.16.0.1:9090");
    let (dec, _): (SocketAddrV4, _) =
        decode_from_slice(&enc).expect("decode SocketAddrV4 172.16.0.1:9090");
    assert_eq!(addr, dec);
}

#[test]
fn test_adv_socketaddrv6_specific_roundtrip() {
    let addr = SocketAddrV6::new(Ipv6Addr::new(0xfe80, 0, 0, 0, 0, 0, 0, 2), 5353, 0, 1);
    let enc = encode_to_vec(&addr).expect("encode SocketAddrV6 fe80::2:5353");
    let (dec, _): (SocketAddrV6, _) =
        decode_from_slice(&enc).expect("decode SocketAddrV6 fe80::2:5353");
    assert_eq!(addr, dec);
}

// ===== IpAddr enum =====

#[test]
fn test_adv_ipaddr_v4_roundtrip() {
    let ip = IpAddr::V4(Ipv4Addr::new(203, 0, 113, 1));
    let enc = encode_to_vec(&ip).expect("encode IpAddr::V4 203.0.113.1");
    let (dec, _): (IpAddr, _) = decode_from_slice(&enc).expect("decode IpAddr::V4 203.0.113.1");
    assert_eq!(ip, dec);
}

#[test]
fn test_adv_ipaddr_v6_roundtrip() {
    let ip = IpAddr::V6(Ipv6Addr::new(0x2001, 0xdb8, 0, 0, 0, 0, 0, 2));
    let enc = encode_to_vec(&ip).expect("encode IpAddr::V6 2001:db8::2");
    let (dec, _): (IpAddr, _) = decode_from_slice(&enc).expect("decode IpAddr::V6 2001:db8::2");
    assert_eq!(ip, dec);
}

// ===== Duration =====

#[test]
fn test_adv_duration_zero_roundtrip() {
    let dur = Duration::from_secs(0);
    let enc = encode_to_vec(&dur).expect("encode Duration zero");
    let (dec, _): (Duration, _) = decode_from_slice(&enc).expect("decode Duration zero");
    assert_eq!(dur, dec);
    assert_eq!(dec.as_secs(), 0);
    assert_eq!(dec.subsec_nanos(), 0);
}

#[test]
fn test_adv_duration_max_roundtrip() {
    let dur = Duration::new(u64::MAX, 999_999_999);
    let enc = encode_to_vec(&dur).expect("encode Duration::MAX");
    let (dec, _): (Duration, _) = decode_from_slice(&enc).expect("decode Duration::MAX");
    assert_eq!(dur, dec);
}

#[test]
fn test_adv_duration_from_nanos_one_second() {
    let dur_nanos = Duration::from_nanos(1_000_000_000);
    let dur_secs = Duration::from_secs(1);
    assert_eq!(
        dur_nanos, dur_secs,
        "1_000_000_000 nanos must equal 1 second"
    );
    let enc_nanos = encode_to_vec(&dur_nanos).expect("encode dur_nanos");
    let enc_secs = encode_to_vec(&dur_secs).expect("encode dur_secs");
    assert_eq!(
        enc_nanos, enc_secs,
        "encoded bytes of 1_000_000_000 nanos and 1 second must be identical"
    );
    let (dec, _): (Duration, _) =
        decode_from_slice(&enc_nanos).expect("decode dur from nanos encoding");
    assert_eq!(dec, Duration::from_secs(1));
}

// ===== SystemTime =====

#[test]
fn test_adv_systemtime_unix_epoch_roundtrip() {
    let t = UNIX_EPOCH;
    let enc = encode_to_vec(&t).expect("encode UNIX_EPOCH");
    let (dec, _): (SystemTime, _) = decode_from_slice(&enc).expect("decode UNIX_EPOCH");
    assert_eq!(t, dec);
}

#[test]
fn test_adv_systemtime_now_roundtrip() {
    let t = SystemTime::now();
    let enc = encode_to_vec(&t).expect("encode SystemTime::now()");
    let (dec, _): (SystemTime, _) = decode_from_slice(&enc).expect("decode SystemTime::now()");
    assert_eq!(t, dec);
}

// ===== Composite struct with Ipv4Addr + Duration =====

#[test]
fn test_adv_struct_ipv4_duration_roundtrip() {
    let record = NetworkRecord {
        addr: Ipv4Addr::new(192, 168, 42, 10),
        timeout: Duration::from_millis(5000),
    };
    let enc = encode_to_vec(&record).expect("encode NetworkRecord");
    let (dec, _): (NetworkRecord, _) = decode_from_slice(&enc).expect("decode NetworkRecord");
    assert_eq!(record, dec);
}

// ===== Vec<SocketAddr> =====

#[test]
fn test_adv_vec_socketaddr_roundtrip() {
    let addrs: Vec<SocketAddr> = vec![
        SocketAddr::V4(SocketAddrV4::new(Ipv4Addr::new(127, 0, 0, 1), 80)),
        SocketAddr::V4(SocketAddrV4::new(Ipv4Addr::new(192, 168, 1, 1), 443)),
        SocketAddr::V6(SocketAddrV6::new(
            Ipv6Addr::new(0, 0, 0, 0, 0, 0, 0, 1),
            8080,
            0,
            0,
        )),
        SocketAddr::V6(SocketAddrV6::new(
            Ipv6Addr::new(0x2001, 0xdb8, 0, 0, 0, 0, 0, 1),
            9443,
            0,
            0,
        )),
    ];
    let enc = encode_to_vec(&addrs).expect("encode Vec<SocketAddr>");
    let (dec, _): (Vec<SocketAddr>, _) = decode_from_slice(&enc).expect("decode Vec<SocketAddr>");
    assert_eq!(addrs, dec);
}
