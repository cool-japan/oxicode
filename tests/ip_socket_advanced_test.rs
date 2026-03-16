//! Advanced tests for IP address and socket address encode/decode implementations.
//!
//! Covers Ipv4Addr, Ipv6Addr, IpAddr, SocketAddrV4, SocketAddrV6, SocketAddr roundtrips,
//! Vec collections, Option<SocketAddr>, fixed-int encoding, string-parsed addresses,
//! wire size assertions, and mixed-variant collections.

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
use std::net::{IpAddr, Ipv4Addr, Ipv6Addr, SocketAddr, SocketAddrV4, SocketAddrV6};

// ===== Test 1: Ipv4Addr::new(127, 0, 0, 1) roundtrip =====

#[test]
fn test_ipv4_localhost_roundtrip() {
    let ip = Ipv4Addr::new(127, 0, 0, 1);
    let enc = encode_to_vec(&ip).expect("encode Ipv4Addr 127.0.0.1");
    let (dec, _): (Ipv4Addr, _) = decode_from_slice(&enc).expect("decode Ipv4Addr 127.0.0.1");
    assert_eq!(ip, dec);
}

// ===== Test 2: Ipv4Addr::new(0, 0, 0, 0) roundtrip =====

#[test]
fn test_ipv4_unspecified_roundtrip() {
    let ip = Ipv4Addr::new(0, 0, 0, 0);
    let enc = encode_to_vec(&ip).expect("encode Ipv4Addr 0.0.0.0");
    let (dec, _): (Ipv4Addr, _) = decode_from_slice(&enc).expect("decode Ipv4Addr 0.0.0.0");
    assert_eq!(ip, dec);
}

// ===== Test 3: Ipv4Addr::new(255, 255, 255, 255) roundtrip =====

#[test]
fn test_ipv4_broadcast_roundtrip() {
    let ip = Ipv4Addr::new(255, 255, 255, 255);
    let enc = encode_to_vec(&ip).expect("encode Ipv4Addr broadcast");
    let (dec, _): (Ipv4Addr, _) = decode_from_slice(&enc).expect("decode Ipv4Addr broadcast");
    assert_eq!(ip, dec);
}

// ===== Test 4: Ipv4Addr::new(192, 168, 1, 1) roundtrip =====

#[test]
fn test_ipv4_private_roundtrip() {
    let ip = Ipv4Addr::new(192, 168, 1, 1);
    let enc = encode_to_vec(&ip).expect("encode Ipv4Addr 192.168.1.1");
    let (dec, _): (Ipv4Addr, _) = decode_from_slice(&enc).expect("decode Ipv4Addr 192.168.1.1");
    assert_eq!(ip, dec);
}

// ===== Test 5: Ipv6Addr::new(0, 0, 0, 0, 0, 0, 0, 1) loopback roundtrip =====

#[test]
fn test_ipv6_loopback_roundtrip() {
    let ip = Ipv6Addr::new(0, 0, 0, 0, 0, 0, 0, 1);
    let enc = encode_to_vec(&ip).expect("encode Ipv6Addr ::1");
    let (dec, _): (Ipv6Addr, _) = decode_from_slice(&enc).expect("decode Ipv6Addr ::1");
    assert_eq!(ip, dec);
}

// ===== Test 6: Ipv6Addr::new(0xfe80, ...) link-local roundtrip =====

#[test]
fn test_ipv6_link_local_roundtrip() {
    let ip = Ipv6Addr::new(0xfe80, 0, 0, 0, 0, 0, 0, 1);
    let enc = encode_to_vec(&ip).expect("encode Ipv6Addr fe80::1");
    let (dec, _): (Ipv6Addr, _) = decode_from_slice(&enc).expect("decode Ipv6Addr fe80::1");
    assert_eq!(ip, dec);
}

// ===== Test 7: Ipv6Addr::UNSPECIFIED roundtrip =====

#[test]
fn test_ipv6_unspecified_roundtrip() {
    let ip = Ipv6Addr::UNSPECIFIED;
    let enc = encode_to_vec(&ip).expect("encode Ipv6Addr UNSPECIFIED");
    let (dec, _): (Ipv6Addr, _) = decode_from_slice(&enc).expect("decode Ipv6Addr UNSPECIFIED");
    assert_eq!(ip, dec);
}

// ===== Test 8: IpAddr::V4(Ipv4Addr::new(1, 2, 3, 4)) roundtrip =====

#[test]
fn test_ipaddr_v4_roundtrip() {
    let ip = IpAddr::V4(Ipv4Addr::new(1, 2, 3, 4));
    let enc = encode_to_vec(&ip).expect("encode IpAddr::V4");
    let (dec, _): (IpAddr, _) = decode_from_slice(&enc).expect("decode IpAddr::V4");
    assert_eq!(ip, dec);
}

// ===== Test 9: IpAddr::V6(Ipv6Addr::new(0x2001, 0xdb8, ...)) roundtrip =====

#[test]
fn test_ipaddr_v6_documentation_roundtrip() {
    let ip = IpAddr::V6(Ipv6Addr::new(0x2001, 0x0db8, 0, 0, 0, 0, 0, 1));
    let enc = encode_to_vec(&ip).expect("encode IpAddr::V6 documentation");
    let (dec, _): (IpAddr, _) = decode_from_slice(&enc).expect("decode IpAddr::V6 documentation");
    assert_eq!(ip, dec);
}

// ===== Test 10: SocketAddrV4::new(127.0.0.1, 8080) roundtrip =====

#[test]
fn test_socketaddrv4_localhost_8080_roundtrip() {
    let addr = SocketAddrV4::new(Ipv4Addr::new(127, 0, 0, 1), 8080);
    let enc = encode_to_vec(&addr).expect("encode SocketAddrV4 127.0.0.1:8080");
    let (dec, _): (SocketAddrV4, _) =
        decode_from_slice(&enc).expect("decode SocketAddrV4 127.0.0.1:8080");
    assert_eq!(addr, dec);
}

// ===== Test 11: SocketAddrV4::new(0.0.0.0, 0) roundtrip =====

#[test]
fn test_socketaddrv4_unspecified_zero_port_roundtrip() {
    let addr = SocketAddrV4::new(Ipv4Addr::new(0, 0, 0, 0), 0);
    let enc = encode_to_vec(&addr).expect("encode SocketAddrV4 0.0.0.0:0");
    let (dec, _): (SocketAddrV4, _) =
        decode_from_slice(&enc).expect("decode SocketAddrV4 0.0.0.0:0");
    assert_eq!(addr, dec);
}

// ===== Test 12: SocketAddrV4::new(255.255.255.255, 65535) roundtrip =====

#[test]
fn test_socketaddrv4_broadcast_maxport_roundtrip() {
    let addr = SocketAddrV4::new(Ipv4Addr::new(255, 255, 255, 255), 65535);
    let enc = encode_to_vec(&addr).expect("encode SocketAddrV4 255.255.255.255:65535");
    let (dec, _): (SocketAddrV4, _) =
        decode_from_slice(&enc).expect("decode SocketAddrV4 255.255.255.255:65535");
    assert_eq!(addr, dec);
}

// ===== Test 13: SocketAddrV6::new(::1, 443, 0, 0) roundtrip =====

#[test]
fn test_socketaddrv6_loopback_443_roundtrip() {
    let addr = SocketAddrV6::new(Ipv6Addr::new(0, 0, 0, 0, 0, 0, 0, 1), 443, 0, 0);
    let enc = encode_to_vec(&addr).expect("encode SocketAddrV6 [::1]:443");
    let (dec, _): (SocketAddrV6, _) =
        decode_from_slice(&enc).expect("decode SocketAddrV6 [::1]:443");
    assert_eq!(addr, dec);
}

// ===== Test 14: SocketAddr::V4(10.0.0.1:3000) roundtrip =====

#[test]
fn test_socketaddr_v4_3000_roundtrip() {
    let addr = SocketAddr::V4(SocketAddrV4::new(Ipv4Addr::new(10, 0, 0, 1), 3000));
    let enc = encode_to_vec(&addr).expect("encode SocketAddr::V4 10.0.0.1:3000");
    let (dec, _): (SocketAddr, _) =
        decode_from_slice(&enc).expect("decode SocketAddr::V4 10.0.0.1:3000");
    assert_eq!(addr, dec);
}

// ===== Test 15: SocketAddr::V6([::1]:80) roundtrip =====

#[test]
fn test_socketaddr_v6_loopback_80_roundtrip() {
    let addr = SocketAddr::V6(SocketAddrV6::new(
        Ipv6Addr::new(0, 0, 0, 0, 0, 0, 0, 1),
        80,
        0,
        0,
    ));
    let enc = encode_to_vec(&addr).expect("encode SocketAddr::V6 [::1]:80");
    let (dec, _): (SocketAddr, _) =
        decode_from_slice(&enc).expect("decode SocketAddr::V6 [::1]:80");
    assert_eq!(addr, dec);
}

// ===== Test 16: Vec<Ipv4Addr> with 3 addresses roundtrip =====

#[test]
fn test_vec_ipv4addr_roundtrip() {
    let addrs = vec![
        Ipv4Addr::new(10, 0, 0, 1),
        Ipv4Addr::new(172, 16, 0, 1),
        Ipv4Addr::new(192, 168, 1, 1),
    ];
    let enc = encode_to_vec(&addrs).expect("encode Vec<Ipv4Addr>");
    let (dec, _): (Vec<Ipv4Addr>, _) = decode_from_slice(&enc).expect("decode Vec<Ipv4Addr>");
    assert_eq!(addrs, dec);
}

// ===== Test 17: Vec<IpAddr> with mixed V4/V6 roundtrip =====

#[test]
fn test_vec_ipaddr_mixed_roundtrip() {
    let addrs: Vec<IpAddr> = vec![
        IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)),
        IpAddr::V6(Ipv6Addr::new(0, 0, 0, 0, 0, 0, 0, 1)),
        IpAddr::V4(Ipv4Addr::new(8, 8, 8, 8)),
    ];
    let enc = encode_to_vec(&addrs).expect("encode Vec<IpAddr> mixed");
    let (dec, _): (Vec<IpAddr>, _) = decode_from_slice(&enc).expect("decode Vec<IpAddr> mixed");
    assert_eq!(addrs, dec);
}

// ===== Test 18: Option<SocketAddr> Some/None roundtrip =====

#[test]
fn test_option_socketaddr_some_none_roundtrip() {
    let some_addr: Option<SocketAddr> = Some(SocketAddr::V4(SocketAddrV4::new(
        Ipv4Addr::new(127, 0, 0, 1),
        9090,
    )));
    let enc_some = encode_to_vec(&some_addr).expect("encode Option<SocketAddr> Some");
    let (dec_some, _): (Option<SocketAddr>, _) =
        decode_from_slice(&enc_some).expect("decode Option<SocketAddr> Some");
    assert_eq!(some_addr, dec_some);

    let none_addr: Option<SocketAddr> = None;
    let enc_none = encode_to_vec(&none_addr).expect("encode Option<SocketAddr> None");
    let (dec_none, _): (Option<SocketAddr>, _) =
        decode_from_slice(&enc_none).expect("decode Option<SocketAddr> None");
    assert_eq!(none_addr, dec_none);
}

// ===== Test 19: Ipv4Addr with fixed-int encoding =====

#[test]
fn test_ipv4addr_fixed_int_encoding_roundtrip() {
    use oxicode::{decode_from_slice_with_config, encode_to_vec_with_config};
    let ip = Ipv4Addr::new(192, 168, 0, 1);
    let cfg = config::standard().with_fixed_int_encoding();
    let enc = encode_to_vec_with_config(&ip, cfg).expect("encode Ipv4Addr fixed_int");
    // Ipv4Addr encodes as raw 4 bytes regardless of int encoding mode
    assert_eq!(enc.len(), 4, "Ipv4Addr must encode as 4 raw bytes");
    assert_eq!(enc, &[192, 168, 0, 1], "Ipv4Addr fixed_int octets mismatch");
    let (dec, _): (Ipv4Addr, _) =
        decode_from_slice_with_config(&enc, cfg).expect("decode Ipv4Addr fixed_int");
    assert_eq!(ip, dec);
}

// ===== Test 20: Ipv4Addr parsed from string =====

#[test]
fn test_ipv4addr_parsed_from_string_roundtrip() {
    let ip: Ipv4Addr = "192.168.0.1".parse().expect("parse Ipv4Addr from string");
    let enc = encode_to_vec(&ip).expect("encode parsed Ipv4Addr");
    let (dec, _): (Ipv4Addr, _) = decode_from_slice(&enc).expect("decode parsed Ipv4Addr");
    assert_eq!(ip, dec);
    assert_eq!(dec.octets(), [192, 168, 0, 1]);
}

// ===== Test 21: Ipv6Addr encoded size is 16 bytes =====

#[test]
fn test_ipv6addr_encoded_size_is_16_bytes() {
    let ip = Ipv6Addr::new(0x2001, 0x0db8, 0x85a3, 0, 0, 0x8a2e, 0x0370, 0x7334);
    let enc = encode_to_vec(&ip).expect("encode Ipv6Addr for size check");
    assert_eq!(
        enc.len(),
        16,
        "Ipv6Addr must encode as exactly 16 raw bytes, got {}",
        enc.len()
    );
}

// ===== Test 22: Vec<SocketAddr> with 2 socket addresses roundtrip =====

#[test]
fn test_vec_socketaddr_roundtrip() {
    let addrs: Vec<SocketAddr> = vec![
        SocketAddr::V4(SocketAddrV4::new(Ipv4Addr::new(127, 0, 0, 1), 8080)),
        SocketAddr::V6(SocketAddrV6::new(
            Ipv6Addr::new(0, 0, 0, 0, 0, 0, 0, 1),
            443,
            0,
            0,
        )),
    ];
    let enc = encode_to_vec(&addrs).expect("encode Vec<SocketAddr>");
    let (dec, _): (Vec<SocketAddr>, _) = decode_from_slice(&enc).expect("decode Vec<SocketAddr>");
    assert_eq!(addrs, dec);
}
