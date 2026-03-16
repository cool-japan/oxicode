//! Extended edge-case tests for network address encode/decode implementations.
//!
//! Covers IpAddr::V4/V6 variants, SocketAddrV4/V6 roundtrips, Vec<IpAddr>,
//! Option<SocketAddr>, structs with SocketAddr fields, and HashMap<IpAddr, String>.

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
use oxicode::{Decode, Encode};
use std::collections::HashMap;
use std::net::*;

// ===== Helper: encode then decode, asserting roundtrip equality =====

fn roundtrip<T>(value: &T) -> T
where
    T: Encode + Decode + PartialEq + std::fmt::Debug,
{
    let encoded = oxicode::encode_to_vec(value).expect("encode failed");
    let (decoded, _): (T, _) = oxicode::decode_from_slice(&encoded).expect("decode failed");
    decoded
}

// ===== IpAddr::V4 variants =====

#[test]
fn test_net_types_extended_ipv4_unspecified() {
    let addr = IpAddr::V4(Ipv4Addr::new(0, 0, 0, 0));
    assert_eq!(addr, roundtrip(&addr));
}

#[test]
fn test_net_types_extended_ipv4_localhost() {
    let addr = IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1));
    assert_eq!(addr, roundtrip(&addr));
}

#[test]
fn test_net_types_extended_ipv4_broadcast() {
    let addr = IpAddr::V4(Ipv4Addr::new(255, 255, 255, 255));
    assert_eq!(addr, roundtrip(&addr));
}

#[test]
fn test_net_types_extended_ipv4_private() {
    let addr = IpAddr::V4(Ipv4Addr::new(10, 0, 0, 1));
    assert_eq!(addr, roundtrip(&addr));
}

#[test]
fn test_net_types_extended_ipv4_wire_format_unspecified() {
    // 0.0.0.0 must encode as discriminant 0x00 then 4 zero bytes
    let addr = IpAddr::V4(Ipv4Addr::new(0, 0, 0, 0));
    let enc = oxicode::encode_to_vec(&addr).expect("encode");
    assert_eq!(enc, &[0x00, 0x00, 0x00, 0x00, 0x00]);
}

#[test]
fn test_net_types_extended_ipv4_wire_format_broadcast() {
    // 255.255.255.255 must encode as discriminant 0x00 then [255, 255, 255, 255]
    let addr = IpAddr::V4(Ipv4Addr::new(255, 255, 255, 255));
    let enc = oxicode::encode_to_vec(&addr).expect("encode");
    assert_eq!(enc, &[0x00, 255, 255, 255, 255]);
}

// ===== IpAddr::V6 variants =====

#[test]
fn test_net_types_extended_ipv6_loopback() {
    // ::1
    let addr = IpAddr::V6(Ipv6Addr::new(0, 0, 0, 0, 0, 0, 0, 1));
    assert_eq!(addr, roundtrip(&addr));
}

#[test]
fn test_net_types_extended_ipv6_link_local() {
    // fe80::1
    let addr = IpAddr::V6(Ipv6Addr::new(0xfe80, 0, 0, 0, 0, 0, 0, 1));
    assert_eq!(addr, roundtrip(&addr));
}

#[test]
fn test_net_types_extended_ipv6_unspecified() {
    // ::
    let addr = IpAddr::V6(Ipv6Addr::new(0, 0, 0, 0, 0, 0, 0, 0));
    assert_eq!(addr, roundtrip(&addr));
}

#[test]
fn test_net_types_extended_ipv6_all_ones() {
    // ffff:ffff:ffff:ffff:ffff:ffff:ffff:ffff
    let addr = IpAddr::V6(Ipv6Addr::new(
        0xffff, 0xffff, 0xffff, 0xffff, 0xffff, 0xffff, 0xffff, 0xffff,
    ));
    assert_eq!(addr, roundtrip(&addr));
}

#[test]
fn test_net_types_extended_ipv6_documentation() {
    // 2001:db8::1 (documentation prefix)
    let addr = IpAddr::V6(Ipv6Addr::new(0x2001, 0x0db8, 0, 0, 0, 0, 0, 1));
    assert_eq!(addr, roundtrip(&addr));
}

#[test]
fn test_net_types_extended_ipv6_wire_format_length() {
    // IpAddr::V6 = 1 discriminant byte + 16 octets = 17 bytes total
    let addr = IpAddr::V6(Ipv6Addr::LOCALHOST);
    let enc = oxicode::encode_to_vec(&addr).expect("encode");
    assert_eq!(enc.len(), 17, "IpAddr::V6 must be 17 bytes");
    assert_eq!(enc[0], 1u8, "discriminant for V6 must be 1");
}

// ===== SocketAddrV4 roundtrip =====

#[test]
fn test_net_types_extended_socketaddrv4_unspecified_port_zero() {
    let addr = SocketAddrV4::new(Ipv4Addr::new(0, 0, 0, 0), 0);
    assert_eq!(addr, roundtrip(&addr));
}

#[test]
fn test_net_types_extended_socketaddrv4_localhost_http() {
    let addr = SocketAddrV4::new(Ipv4Addr::new(127, 0, 0, 1), 80);
    assert_eq!(addr, roundtrip(&addr));
}

#[test]
fn test_net_types_extended_socketaddrv4_private_high_port() {
    let addr = SocketAddrV4::new(Ipv4Addr::new(192, 168, 0, 100), 49152);
    assert_eq!(addr, roundtrip(&addr));
}

#[test]
fn test_net_types_extended_socketaddrv4_broadcast_max_port() {
    let addr = SocketAddrV4::new(Ipv4Addr::new(255, 255, 255, 255), 65535);
    assert_eq!(addr, roundtrip(&addr));
}

// ===== SocketAddrV6 roundtrip =====

#[test]
fn test_net_types_extended_socketaddrv6_loopback_https() {
    let addr = SocketAddrV6::new(Ipv6Addr::new(0, 0, 0, 0, 0, 0, 0, 1), 443, 0, 0);
    assert_eq!(addr, roundtrip(&addr));
}

#[test]
fn test_net_types_extended_socketaddrv6_link_local_with_scope() {
    // fe80::1 with scope_id = 7 (typical interface index)
    let addr = SocketAddrV6::new(Ipv6Addr::new(0xfe80, 0, 0, 0, 0, 0, 0, 1), 8080, 0, 7);
    assert_eq!(addr, roundtrip(&addr));
}

#[test]
fn test_net_types_extended_socketaddrv6_all_fields_nonzero() {
    let addr = SocketAddrV6::new(
        Ipv6Addr::new(0x2001, 0x0db8, 0, 0, 0, 0, 0, 1),
        9000,
        0xdeadbeef,
        42,
    );
    assert_eq!(addr, roundtrip(&addr));
}

// ===== Vec<IpAddr> roundtrip =====

#[test]
fn test_net_types_extended_vec_ipaddr_empty() {
    let addrs: Vec<IpAddr> = vec![];
    assert_eq!(addrs, roundtrip(&addrs));
}

#[test]
fn test_net_types_extended_vec_ipaddr_mixed_v4_v6() {
    let addrs: Vec<IpAddr> = vec![
        IpAddr::V4(Ipv4Addr::new(0, 0, 0, 0)),
        IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)),
        IpAddr::V4(Ipv4Addr::new(255, 255, 255, 255)),
        IpAddr::V4(Ipv4Addr::new(10, 0, 0, 1)),
        IpAddr::V6(Ipv6Addr::new(0, 0, 0, 0, 0, 0, 0, 1)),
        IpAddr::V6(Ipv6Addr::new(0xfe80, 0, 0, 0, 0, 0, 0, 1)),
    ];
    assert_eq!(addrs, roundtrip(&addrs));
}

#[test]
fn test_net_types_extended_vec_ipaddr_only_v4() {
    let addrs: Vec<IpAddr> = vec![
        IpAddr::V4(Ipv4Addr::new(10, 0, 0, 1)),
        IpAddr::V4(Ipv4Addr::new(172, 16, 0, 1)),
        IpAddr::V4(Ipv4Addr::new(192, 168, 1, 1)),
    ];
    assert_eq!(addrs, roundtrip(&addrs));
}

#[test]
fn test_net_types_extended_vec_ipaddr_only_v6() {
    let addrs: Vec<IpAddr> = vec![
        IpAddr::V6(Ipv6Addr::LOCALHOST),
        IpAddr::V6(Ipv6Addr::new(0xfe80, 0, 0, 0, 0, 0, 0, 1)),
        IpAddr::V6(Ipv6Addr::new(
            0xffff, 0xffff, 0xffff, 0xffff, 0xffff, 0xffff, 0xffff, 0xffff,
        )),
    ];
    assert_eq!(addrs, roundtrip(&addrs));
}

// ===== Option<SocketAddr> roundtrip =====

#[test]
fn test_net_types_extended_option_socketaddr_none() {
    let addr: Option<SocketAddr> = None;
    assert_eq!(addr, roundtrip(&addr));
}

#[test]
fn test_net_types_extended_option_socketaddr_some_v4() {
    let addr: Option<SocketAddr> = Some(SocketAddr::V4(SocketAddrV4::new(
        Ipv4Addr::new(127, 0, 0, 1),
        8080,
    )));
    assert_eq!(addr, roundtrip(&addr));
}

#[test]
fn test_net_types_extended_option_socketaddr_some_v6() {
    let addr: Option<SocketAddr> = Some(SocketAddr::V6(SocketAddrV6::new(
        Ipv6Addr::LOCALHOST,
        443,
        0,
        0,
    )));
    assert_eq!(addr, roundtrip(&addr));
}

#[test]
fn test_net_types_extended_option_socketaddr_none_wire_format() {
    // None encodes as single byte 0x00
    let addr: Option<SocketAddr> = None;
    let enc = oxicode::encode_to_vec(&addr).expect("encode");
    assert_eq!(enc, &[0x00], "Option::None must encode as single 0x00");
}

// ===== Struct with SocketAddr field =====

#[derive(Encode, Decode, PartialEq, Debug)]
struct ServerConfig {
    bind_addr: SocketAddr,
    max_connections: u32,
}

#[test]
fn test_net_types_extended_struct_with_socketaddr_v4() {
    let cfg = ServerConfig {
        bind_addr: SocketAddr::V4(SocketAddrV4::new(Ipv4Addr::new(0, 0, 0, 0), 8080)),
        max_connections: 1024,
    };
    assert_eq!(cfg, roundtrip(&cfg));
}

#[test]
fn test_net_types_extended_struct_with_socketaddr_v6() {
    let cfg = ServerConfig {
        bind_addr: SocketAddr::V6(SocketAddrV6::new(Ipv6Addr::LOCALHOST, 443, 0, 0)),
        max_connections: 512,
    };
    assert_eq!(cfg, roundtrip(&cfg));
}

#[derive(Encode, Decode, PartialEq, Debug)]
struct PeerInfo {
    addr: SocketAddr,
    label: String,
    active: bool,
}

#[test]
fn test_net_types_extended_struct_peer_info_v4() {
    let peer = PeerInfo {
        addr: SocketAddr::V4(SocketAddrV4::new(Ipv4Addr::new(192, 168, 1, 50), 3000)),
        label: "edge-node".to_string(),
        active: true,
    };
    assert_eq!(peer, roundtrip(&peer));
}

#[test]
fn test_net_types_extended_struct_peer_info_v6_link_local() {
    let peer = PeerInfo {
        addr: SocketAddr::V6(SocketAddrV6::new(
            Ipv6Addr::new(0xfe80, 0, 0, 0, 0, 0, 0, 1),
            9999,
            0,
            3,
        )),
        label: "link-local-peer".to_string(),
        active: false,
    };
    assert_eq!(peer, roundtrip(&peer));
}

// ===== HashMap<IpAddr, String> roundtrip =====

#[test]
fn test_net_types_extended_hashmap_ipaddr_string_empty() {
    let map: HashMap<IpAddr, String> = HashMap::new();
    assert_eq!(map, roundtrip(&map));
}

#[test]
fn test_net_types_extended_hashmap_ipaddr_string_v4_entries() {
    let mut map: HashMap<IpAddr, String> = HashMap::new();
    map.insert(
        IpAddr::V4(Ipv4Addr::new(10, 0, 0, 1)),
        "gateway".to_string(),
    );
    map.insert(IpAddr::V4(Ipv4Addr::new(10, 0, 0, 2)), "dns".to_string());
    map.insert(
        IpAddr::V4(Ipv4Addr::new(192, 168, 1, 1)),
        "router".to_string(),
    );
    assert_eq!(map, roundtrip(&map));
}

#[test]
fn test_net_types_extended_hashmap_ipaddr_string_v6_entries() {
    let mut map: HashMap<IpAddr, String> = HashMap::new();
    map.insert(IpAddr::V6(Ipv6Addr::LOCALHOST), "loopback".to_string());
    map.insert(
        IpAddr::V6(Ipv6Addr::new(0xfe80, 0, 0, 0, 0, 0, 0, 1)),
        "link-local".to_string(),
    );
    assert_eq!(map, roundtrip(&map));
}

#[test]
fn test_net_types_extended_hashmap_ipaddr_string_mixed_variants() {
    let mut map: HashMap<IpAddr, String> = HashMap::new();
    map.insert(
        IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)),
        "v4-loopback".to_string(),
    );
    map.insert(
        IpAddr::V4(Ipv4Addr::new(0, 0, 0, 0)),
        "v4-unspecified".to_string(),
    );
    map.insert(IpAddr::V6(Ipv6Addr::LOCALHOST), "v6-loopback".to_string());
    map.insert(
        IpAddr::V6(Ipv6Addr::new(0x2001, 0x0db8, 0, 0, 0, 0, 0, 1)),
        "documentation".to_string(),
    );
    assert_eq!(map, roundtrip(&map));
}

#[test]
fn test_net_types_extended_hashmap_ipaddr_string_single_entry() {
    let mut map: HashMap<IpAddr, String> = HashMap::new();
    map.insert(
        IpAddr::V4(Ipv4Addr::new(255, 255, 255, 255)),
        "broadcast".to_string(),
    );
    let decoded = roundtrip(&map);
    assert_eq!(decoded.len(), 1);
    assert_eq!(
        decoded.get(&IpAddr::V4(Ipv4Addr::new(255, 255, 255, 255))),
        Some(&"broadcast".to_string())
    );
}
