//! Third batch of advanced tests for std::net type encode/decode implementations.
//! Focuses on new scenarios not covered by net_types_test.rs,
//! net_types_advanced_test.rs, or net_types_extended_test.rs.

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
use oxicode::{decode_from_slice, encode_to_vec, Decode, Encode};
use std::net::{IpAddr, Ipv4Addr, Ipv6Addr, SocketAddr, SocketAddrV4, SocketAddrV6};

// ===== Test 22 helper struct =====

#[derive(Debug, PartialEq, Encode, Decode)]
struct Endpoint {
    host: IpAddr,
    addr: SocketAddr,
    name: String,
}

// ===== Test 1: Ipv4Addr loopback 127.0.0.1 roundtrip =====

#[test]
fn test_adv3_ipv4_loopback_roundtrip() {
    let ip = Ipv4Addr::new(127, 0, 0, 1);
    let enc = encode_to_vec(&ip).expect("encode Ipv4Addr 127.0.0.1");
    let (val, _): (Ipv4Addr, _) = decode_from_slice(&enc).expect("decode Ipv4Addr 127.0.0.1");
    assert_eq!(ip, val);
}

// ===== Test 2: Ipv4Addr broadcast 255.255.255.255 roundtrip =====

#[test]
fn test_adv3_ipv4_broadcast_roundtrip() {
    let ip = Ipv4Addr::new(255, 255, 255, 255);
    let enc = encode_to_vec(&ip).expect("encode Ipv4Addr 255.255.255.255");
    let (val, _): (Ipv4Addr, _) = decode_from_slice(&enc).expect("decode Ipv4Addr 255.255.255.255");
    assert_eq!(ip, val);
}

// ===== Test 3: Ipv4Addr any 0.0.0.0 roundtrip =====

#[test]
fn test_adv3_ipv4_any_roundtrip() {
    let ip = Ipv4Addr::new(0, 0, 0, 0);
    let enc = encode_to_vec(&ip).expect("encode Ipv4Addr 0.0.0.0");
    let (val, _): (Ipv4Addr, _) = decode_from_slice(&enc).expect("decode Ipv4Addr 0.0.0.0");
    assert_eq!(ip, val);
}

// ===== Test 4: Ipv4Addr encodes as exactly 4 bytes =====

#[test]
fn test_adv3_ipv4_encodes_exactly_4_bytes() {
    let ip = Ipv4Addr::new(10, 20, 30, 40);
    let enc = encode_to_vec(&ip).expect("encode Ipv4Addr 10.20.30.40");
    assert_eq!(
        enc.len(),
        4,
        "Ipv4Addr must encode as exactly 4 raw bytes, got {}",
        enc.len()
    );
    assert_eq!(enc, &[10, 20, 30, 40], "Ipv4Addr must encode as raw octets");
}

// ===== Test 5: Ipv6Addr loopback ::1 roundtrip =====

#[test]
fn test_adv3_ipv6_loopback_roundtrip() {
    let ip = Ipv6Addr::new(0, 0, 0, 0, 0, 0, 0, 1);
    let enc = encode_to_vec(&ip).expect("encode Ipv6Addr ::1");
    let (val, _): (Ipv6Addr, _) = decode_from_slice(&enc).expect("decode Ipv6Addr ::1");
    assert_eq!(ip, val);
}

// ===== Test 6: Ipv6Addr any :: roundtrip =====

#[test]
fn test_adv3_ipv6_any_roundtrip() {
    let ip = Ipv6Addr::new(0, 0, 0, 0, 0, 0, 0, 0);
    let enc = encode_to_vec(&ip).expect("encode Ipv6Addr ::");
    let (val, _): (Ipv6Addr, _) = decode_from_slice(&enc).expect("decode Ipv6Addr ::");
    assert_eq!(ip, val);
}

// ===== Test 7: Ipv6Addr encodes as exactly 16 bytes =====

#[test]
fn test_adv3_ipv6_encodes_exactly_16_bytes() {
    let ip = Ipv6Addr::new(0x2001, 0x0db8, 0x85a3, 0, 0, 0x8a2e, 0x0370, 0x7334);
    let enc = encode_to_vec(&ip).expect("encode Ipv6Addr 2001:db8:85a3::8a2e:370:7334");
    assert_eq!(
        enc.len(),
        16,
        "Ipv6Addr must encode as exactly 16 raw bytes, got {}",
        enc.len()
    );
}

// ===== Test 8: Two different Ipv4Addrs produce different bytes =====

#[test]
fn test_adv3_two_ipv4_addrs_produce_different_bytes() {
    let ip_a = Ipv4Addr::new(192, 168, 1, 1);
    let ip_b = Ipv4Addr::new(192, 168, 1, 2);
    let enc_a = encode_to_vec(&ip_a).expect("encode ip_a");
    let enc_b = encode_to_vec(&ip_b).expect("encode ip_b");
    assert_ne!(
        enc_a, enc_b,
        "Two different Ipv4Addrs must produce different encoded bytes"
    );
}

// ===== Test 9: IpAddr::V4 roundtrip =====

#[test]
fn test_adv3_ipaddr_v4_roundtrip() {
    let ip = IpAddr::V4(Ipv4Addr::new(172, 16, 254, 1));
    let enc = encode_to_vec(&ip).expect("encode IpAddr::V4 172.16.254.1");
    let (val, _): (IpAddr, _) = decode_from_slice(&enc).expect("decode IpAddr::V4 172.16.254.1");
    assert_eq!(ip, val);
}

// ===== Test 10: IpAddr::V4 and IpAddr::V6 produce different bytes (different discriminant) =====

#[test]
fn test_adv3_ipaddr_v4_v6_different_discriminant() {
    let v4 = IpAddr::V4(Ipv4Addr::new(0, 0, 0, 0));
    let v6 = IpAddr::V6(Ipv6Addr::new(0, 0, 0, 0, 0, 0, 0, 0));
    let enc_v4 = encode_to_vec(&v4).expect("encode IpAddr::V4 unspecified");
    let enc_v6 = encode_to_vec(&v6).expect("encode IpAddr::V6 unspecified");
    assert_ne!(
        enc_v4[0], enc_v6[0],
        "IpAddr::V4 and IpAddr::V6 must have different discriminant bytes"
    );
    assert_ne!(
        enc_v4, enc_v6,
        "IpAddr::V4 and IpAddr::V6 must produce different encoded bytes"
    );
}

// ===== Test 11: IpAddr::V6 roundtrip =====

#[test]
fn test_adv3_ipaddr_v6_roundtrip() {
    let ip = IpAddr::V6(Ipv6Addr::new(
        0xfe80, 0, 0, 0, 0x0202, 0xb3ff, 0xfe1e, 0x8329,
    ));
    let enc = encode_to_vec(&ip).expect("encode IpAddr::V6 fe80::202:b3ff:fe1e:8329");
    let (val, _): (IpAddr, _) =
        decode_from_slice(&enc).expect("decode IpAddr::V6 fe80::202:b3ff:fe1e:8329");
    assert_eq!(ip, val);
}

// ===== Test 12: SocketAddrV4 roundtrip (IP + port) =====

#[test]
fn test_adv3_socketaddrv4_roundtrip_ip_and_port() {
    let addr = SocketAddrV4::new(Ipv4Addr::new(10, 0, 0, 99), 8443);
    let enc = encode_to_vec(&addr).expect("encode SocketAddrV4 10.0.0.99:8443");
    let (val, _): (SocketAddrV4, _) =
        decode_from_slice(&enc).expect("decode SocketAddrV4 10.0.0.99:8443");
    assert_eq!(addr, val);
    assert_eq!(*val.ip(), Ipv4Addr::new(10, 0, 0, 99));
    assert_eq!(val.port(), 8443);
}

// ===== Test 13: SocketAddrV4 with port 0 roundtrip =====

#[test]
fn test_adv3_socketaddrv4_port_zero_roundtrip() {
    let addr = SocketAddrV4::new(Ipv4Addr::new(127, 0, 0, 1), 0);
    let enc = encode_to_vec(&addr).expect("encode SocketAddrV4 127.0.0.1:0");
    let (val, _): (SocketAddrV4, _) =
        decode_from_slice(&enc).expect("decode SocketAddrV4 127.0.0.1:0");
    assert_eq!(addr, val);
    assert_eq!(val.port(), 0);
}

// ===== Test 14: SocketAddrV4 with port 65535 roundtrip =====

#[test]
fn test_adv3_socketaddrv4_port_max_roundtrip() {
    let addr = SocketAddrV4::new(Ipv4Addr::new(0, 0, 0, 0), 65535);
    let enc = encode_to_vec(&addr).expect("encode SocketAddrV4 0.0.0.0:65535");
    let (val, _): (SocketAddrV4, _) =
        decode_from_slice(&enc).expect("decode SocketAddrV4 0.0.0.0:65535");
    assert_eq!(addr, val);
    assert_eq!(val.port(), 65535);
}

// ===== Test 15: SocketAddrV6 roundtrip =====

#[test]
fn test_adv3_socketaddrv6_roundtrip() {
    let addr = SocketAddrV6::new(
        Ipv6Addr::new(0x2001, 0x0db8, 0, 0, 0, 0, 0, 1),
        9000,
        0xabcdef01,
        5,
    );
    let enc = encode_to_vec(&addr).expect("encode SocketAddrV6 2001:db8::1:9000");
    let (val, _): (SocketAddrV6, _) =
        decode_from_slice(&enc).expect("decode SocketAddrV6 2001:db8::1:9000");
    assert_eq!(addr, val);
    assert_eq!(val.port(), 9000);
    assert_eq!(val.flowinfo(), 0xabcdef01);
    assert_eq!(val.scope_id(), 5);
}

// ===== Test 16: SocketAddr::V4 roundtrip =====

#[test]
fn test_adv3_socketaddr_v4_roundtrip() {
    let addr = SocketAddr::V4(SocketAddrV4::new(Ipv4Addr::new(127, 0, 0, 1), 4444));
    let enc = encode_to_vec(&addr).expect("encode SocketAddr::V4 127.0.0.1:4444");
    let (val, _): (SocketAddr, _) =
        decode_from_slice(&enc).expect("decode SocketAddr::V4 127.0.0.1:4444");
    assert_eq!(addr, val);
}

// ===== Test 17: SocketAddr::V6 roundtrip =====

#[test]
fn test_adv3_socketaddr_v6_roundtrip() {
    let addr = SocketAddr::V6(SocketAddrV6::new(
        Ipv6Addr::new(0, 0, 0, 0, 0, 0, 0, 1),
        7777,
        0,
        0,
    ));
    let enc = encode_to_vec(&addr).expect("encode SocketAddr::V6 [::1]:7777");
    let (val, _): (SocketAddr, _) =
        decode_from_slice(&enc).expect("decode SocketAddr::V6 [::1]:7777");
    assert_eq!(addr, val);
}

// ===== Test 18: Vec<IpAddr> roundtrip (mixed V4 and V6) =====

#[test]
fn test_adv3_vec_ipaddr_mixed_roundtrip() {
    let addrs: Vec<IpAddr> = vec![
        IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)),
        IpAddr::V6(Ipv6Addr::new(0, 0, 0, 0, 0, 0, 0, 1)),
        IpAddr::V4(Ipv4Addr::new(255, 255, 255, 255)),
        IpAddr::V6(Ipv6Addr::new(0xfe80, 0, 0, 0, 0, 0, 0, 1)),
        IpAddr::V4(Ipv4Addr::new(0, 0, 0, 0)),
    ];
    let enc = encode_to_vec(&addrs).expect("encode Vec<IpAddr> mixed");
    let (val, _): (Vec<IpAddr>, _) = decode_from_slice(&enc).expect("decode Vec<IpAddr> mixed");
    assert_eq!(addrs, val);
    assert_eq!(val.len(), 5);
}

// ===== Test 19: Vec<SocketAddr> roundtrip =====

#[test]
fn test_adv3_vec_socketaddr_roundtrip() {
    let addrs: Vec<SocketAddr> = vec![
        SocketAddr::V4(SocketAddrV4::new(Ipv4Addr::new(127, 0, 0, 1), 80)),
        SocketAddr::V4(SocketAddrV4::new(Ipv4Addr::new(10, 0, 0, 1), 443)),
        SocketAddr::V6(SocketAddrV6::new(
            Ipv6Addr::new(0, 0, 0, 0, 0, 0, 0, 1),
            8080,
            0,
            0,
        )),
        SocketAddr::V6(SocketAddrV6::new(
            Ipv6Addr::new(0x2001, 0x0db8, 0, 0, 0, 0, 0, 2),
            9443,
            0,
            0,
        )),
    ];
    let enc = encode_to_vec(&addrs).expect("encode Vec<SocketAddr>");
    let (val, _): (Vec<SocketAddr>, _) = decode_from_slice(&enc).expect("decode Vec<SocketAddr>");
    assert_eq!(addrs, val);
    assert_eq!(val.len(), 4);
}

// ===== Test 20: Option<IpAddr> Some roundtrip =====

#[test]
fn test_adv3_option_ipaddr_some_roundtrip() {
    let opt: Option<IpAddr> = Some(IpAddr::V4(Ipv4Addr::new(192, 0, 2, 1)));
    let enc = encode_to_vec(&opt).expect("encode Option<IpAddr> Some");
    let (val, _): (Option<IpAddr>, _) =
        decode_from_slice(&enc).expect("decode Option<IpAddr> Some");
    assert_eq!(opt, val);
    assert!(val.is_some(), "decoded Option<IpAddr> must be Some");
}

// ===== Test 21: Option<SocketAddr> None roundtrip =====

#[test]
fn test_adv3_option_socketaddr_none_roundtrip() {
    let opt: Option<SocketAddr> = None;
    let enc = encode_to_vec(&opt).expect("encode Option<SocketAddr> None");
    let (val, _): (Option<SocketAddr>, _) =
        decode_from_slice(&enc).expect("decode Option<SocketAddr> None");
    assert_eq!(opt, val);
    assert!(val.is_none(), "decoded Option<SocketAddr> must be None");
}

// ===== Test 22: Struct containing IpAddr and SocketAddr fields roundtrip =====

#[test]
fn test_adv3_struct_endpoint_roundtrip() {
    let endpoint = Endpoint {
        host: IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)),
        addr: SocketAddr::V4(SocketAddrV4::new(Ipv4Addr::new(0, 0, 0, 0), 8080)),
        name: "my-service".to_string(),
    };
    let enc = encode_to_vec(&endpoint).expect("encode Endpoint");
    let (val, _): (Endpoint, _) = decode_from_slice(&enc).expect("decode Endpoint");
    assert_eq!(endpoint, val);
    assert_eq!(val.name, "my-service");
    assert_eq!(val.host, IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)));
    assert_eq!(
        val.addr,
        SocketAddr::V4(SocketAddrV4::new(Ipv4Addr::new(0, 0, 0, 0), 8080))
    );
}
