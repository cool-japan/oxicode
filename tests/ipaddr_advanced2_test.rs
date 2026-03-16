//! Advanced IP address and socket address encoding tests for OxiCode.
//! 22 tests covering Ipv4Addr, Ipv6Addr, IpAddr, SocketAddrV4, SocketAddrV6,
//! SocketAddr, collections, Option, structs, config variants, and tuples.

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
use std::collections::BTreeMap;
use std::net::{IpAddr, Ipv4Addr, Ipv6Addr, SocketAddr, SocketAddrV4, SocketAddrV6};

use oxicode::{
    config, decode_from_slice, decode_from_slice_with_config, encode_to_vec,
    encode_to_vec_with_config, Decode, Encode,
};

// ---------------------------------------------------------------------------
// Helper struct for test 17
// ---------------------------------------------------------------------------

#[derive(Encode, Decode, PartialEq, Debug)]
struct IpEndpoint {
    ip: IpAddr,
    port: u16,
}

// ---------------------------------------------------------------------------
// 1. Ipv4Addr localhost (127.0.0.1) roundtrip
// ---------------------------------------------------------------------------

#[test]
fn test_ipv4addr_localhost_roundtrip() {
    let ip = "127.0.0.1".parse::<Ipv4Addr>().expect("parse 127.0.0.1");
    let enc = encode_to_vec(&ip).expect("encode Ipv4Addr localhost");
    let (dec, _bytes): (Ipv4Addr, usize) =
        decode_from_slice(&enc).expect("decode Ipv4Addr localhost");
    assert_eq!(ip, dec);
}

// ---------------------------------------------------------------------------
// 2. Ipv4Addr broadcast (255.255.255.255) roundtrip
// ---------------------------------------------------------------------------

#[test]
fn test_ipv4addr_broadcast_roundtrip() {
    let ip = "255.255.255.255"
        .parse::<Ipv4Addr>()
        .expect("parse 255.255.255.255");
    let enc = encode_to_vec(&ip).expect("encode Ipv4Addr broadcast");
    let (dec, _bytes): (Ipv4Addr, usize) =
        decode_from_slice(&enc).expect("decode Ipv4Addr broadcast");
    assert_eq!(ip, dec);
}

// ---------------------------------------------------------------------------
// 3. Ipv4Addr zero (0.0.0.0) roundtrip
// ---------------------------------------------------------------------------

#[test]
fn test_ipv4addr_zero_roundtrip() {
    let ip = "0.0.0.0".parse::<Ipv4Addr>().expect("parse 0.0.0.0");
    let enc = encode_to_vec(&ip).expect("encode Ipv4Addr zero");
    let (dec, _bytes): (Ipv4Addr, usize) = decode_from_slice(&enc).expect("decode Ipv4Addr zero");
    assert_eq!(ip, dec);
}

// ---------------------------------------------------------------------------
// 4. Ipv6Addr localhost (::1) roundtrip
// ---------------------------------------------------------------------------

#[test]
fn test_ipv6addr_localhost_roundtrip() {
    let ip = "::1".parse::<Ipv6Addr>().expect("parse ::1");
    let enc = encode_to_vec(&ip).expect("encode Ipv6Addr localhost");
    let (dec, _bytes): (Ipv6Addr, usize) =
        decode_from_slice(&enc).expect("decode Ipv6Addr localhost");
    assert_eq!(ip, dec);
}

// ---------------------------------------------------------------------------
// 5. Ipv6Addr zero (::) roundtrip
// ---------------------------------------------------------------------------

#[test]
fn test_ipv6addr_zero_roundtrip() {
    let ip = "::".parse::<Ipv6Addr>().expect("parse ::");
    let enc = encode_to_vec(&ip).expect("encode Ipv6Addr zero");
    let (dec, _bytes): (Ipv6Addr, usize) = decode_from_slice(&enc).expect("decode Ipv6Addr zero");
    assert_eq!(ip, dec);
}

// ---------------------------------------------------------------------------
// 6. IpAddr::V4 roundtrip
// ---------------------------------------------------------------------------

#[test]
fn test_ipaddr_v4_roundtrip() {
    let ip = "10.0.0.1".parse::<IpAddr>().expect("parse IpAddr 10.0.0.1");
    let enc = encode_to_vec(&ip).expect("encode IpAddr::V4");
    let (dec, _bytes): (IpAddr, usize) = decode_from_slice(&enc).expect("decode IpAddr::V4");
    assert_eq!(ip, dec);
}

// ---------------------------------------------------------------------------
// 7. IpAddr::V6 roundtrip
// ---------------------------------------------------------------------------

#[test]
fn test_ipaddr_v6_roundtrip() {
    let ip = "2001:db8::1".parse::<IpAddr>().expect("parse IpAddr V6");
    let enc = encode_to_vec(&ip).expect("encode IpAddr::V6");
    let (dec, _bytes): (IpAddr, usize) = decode_from_slice(&enc).expect("decode IpAddr::V6");
    assert_eq!(ip, dec);
}

// ---------------------------------------------------------------------------
// 8. SocketAddrV4 roundtrip
// ---------------------------------------------------------------------------

#[test]
fn test_socketaddrv4_roundtrip() {
    let addr = "127.0.0.1:8080"
        .parse::<SocketAddrV4>()
        .expect("parse SocketAddrV4");
    let enc = encode_to_vec(&addr).expect("encode SocketAddrV4");
    let (dec, _bytes): (SocketAddrV4, usize) =
        decode_from_slice(&enc).expect("decode SocketAddrV4");
    assert_eq!(addr, dec);
}

// ---------------------------------------------------------------------------
// 9. SocketAddrV6 roundtrip
// ---------------------------------------------------------------------------

#[test]
fn test_socketaddrv6_roundtrip() {
    let addr = SocketAddrV6::new("::1".parse::<Ipv6Addr>().expect("parse ::1"), 443, 0, 0);
    let enc = encode_to_vec(&addr).expect("encode SocketAddrV6");
    let (dec, _bytes): (SocketAddrV6, usize) =
        decode_from_slice(&enc).expect("decode SocketAddrV6");
    assert_eq!(addr, dec);
}

// ---------------------------------------------------------------------------
// 10. SocketAddr::V4 roundtrip
// ---------------------------------------------------------------------------

#[test]
fn test_socketaddr_v4_roundtrip() {
    let addr = "192.168.1.1:3000"
        .parse::<SocketAddr>()
        .expect("parse SocketAddr V4");
    let enc = encode_to_vec(&addr).expect("encode SocketAddr::V4");
    let (dec, _bytes): (SocketAddr, usize) =
        decode_from_slice(&enc).expect("decode SocketAddr::V4");
    assert_eq!(addr, dec);
}

// ---------------------------------------------------------------------------
// 11. SocketAddr::V6 roundtrip
// ---------------------------------------------------------------------------

#[test]
fn test_socketaddr_v6_roundtrip() {
    let addr = "[::1]:8443"
        .parse::<SocketAddr>()
        .expect("parse SocketAddr V6");
    let enc = encode_to_vec(&addr).expect("encode SocketAddr::V6");
    let (dec, _bytes): (SocketAddr, usize) =
        decode_from_slice(&enc).expect("decode SocketAddr::V6");
    assert_eq!(addr, dec);
}

// ---------------------------------------------------------------------------
// 12. Vec<Ipv4Addr> roundtrip
// ---------------------------------------------------------------------------

#[test]
fn test_vec_ipv4addr_roundtrip() {
    let ips: Vec<Ipv4Addr> = vec![
        "127.0.0.1".parse().expect("parse 127.0.0.1"),
        "192.168.0.1".parse().expect("parse 192.168.0.1"),
        "10.0.0.1".parse().expect("parse 10.0.0.1"),
        "0.0.0.0".parse().expect("parse 0.0.0.0"),
        "255.255.255.255".parse().expect("parse 255.255.255.255"),
    ];
    let enc = encode_to_vec(&ips).expect("encode Vec<Ipv4Addr>");
    let (dec, _bytes): (Vec<Ipv4Addr>, usize) =
        decode_from_slice(&enc).expect("decode Vec<Ipv4Addr>");
    assert_eq!(ips, dec);
}

// ---------------------------------------------------------------------------
// 13. Vec<IpAddr> roundtrip
// ---------------------------------------------------------------------------

#[test]
fn test_vec_ipaddr_roundtrip() {
    let ips: Vec<IpAddr> = vec![
        "127.0.0.1".parse().expect("parse IpAddr v4"),
        "::1".parse().expect("parse IpAddr v6"),
        "10.0.0.2".parse().expect("parse IpAddr v4 2"),
        "2001:db8::2".parse().expect("parse IpAddr v6 2"),
    ];
    let enc = encode_to_vec(&ips).expect("encode Vec<IpAddr>");
    let (dec, _bytes): (Vec<IpAddr>, usize) = decode_from_slice(&enc).expect("decode Vec<IpAddr>");
    assert_eq!(ips, dec);
}

// ---------------------------------------------------------------------------
// 14. Option<SocketAddr> Some roundtrip
// ---------------------------------------------------------------------------

#[test]
fn test_option_socketaddr_some_roundtrip() {
    let addr: Option<SocketAddr> = Some(
        "127.0.0.1:9000"
            .parse()
            .expect("parse SocketAddr for Option Some"),
    );
    let enc = encode_to_vec(&addr).expect("encode Option<SocketAddr> Some");
    let (dec, _bytes): (Option<SocketAddr>, usize) =
        decode_from_slice(&enc).expect("decode Option<SocketAddr> Some");
    assert_eq!(addr, dec);
}

// ---------------------------------------------------------------------------
// 15. Option<SocketAddr> None roundtrip
// ---------------------------------------------------------------------------

#[test]
fn test_option_socketaddr_none_roundtrip() {
    let addr: Option<SocketAddr> = None;
    let enc = encode_to_vec(&addr).expect("encode Option<SocketAddr> None");
    let (dec, _bytes): (Option<SocketAddr>, usize) =
        decode_from_slice(&enc).expect("decode Option<SocketAddr> None");
    assert_eq!(addr, dec);
}

// ---------------------------------------------------------------------------
// 16. Ipv4Addr encoded as 4 bytes with fixed-int config
// ---------------------------------------------------------------------------

#[test]
fn test_ipv4addr_fixed_int_config_4_bytes() {
    let ip = "127.0.0.1".parse::<Ipv4Addr>().expect("parse 127.0.0.1");
    let cfg = config::standard().with_fixed_int_encoding();
    let enc = encode_to_vec_with_config(&ip, cfg).expect("encode Ipv4Addr fixed-int");
    // Ipv4Addr is always 4 raw bytes regardless of int encoding config
    assert_eq!(enc.len(), 4, "Ipv4Addr must encode as exactly 4 bytes");
    assert_eq!(enc, &[127, 0, 0, 1], "Ipv4Addr bytes must be raw octets");
    let (dec, _bytes): (Ipv4Addr, usize) =
        decode_from_slice_with_config(&enc, cfg).expect("decode Ipv4Addr fixed-int");
    assert_eq!(ip, dec);
}

// ---------------------------------------------------------------------------
// 17. Struct { ip: IpAddr, port: u16 } roundtrip
// ---------------------------------------------------------------------------

#[test]
fn test_struct_ipaddr_port_roundtrip() {
    let endpoint = IpEndpoint {
        ip: "192.168.1.100"
            .parse::<IpAddr>()
            .expect("parse IpAddr for struct"),
        port: 8080,
    };
    let enc = encode_to_vec(&endpoint).expect("encode IpEndpoint");
    let (dec, _bytes): (IpEndpoint, usize) = decode_from_slice(&enc).expect("decode IpEndpoint");
    assert_eq!(endpoint, dec);
}

// ---------------------------------------------------------------------------
// 18. Vec<SocketAddr> roundtrip
// ---------------------------------------------------------------------------

#[test]
fn test_vec_socketaddr_roundtrip() {
    let addrs: Vec<SocketAddr> = vec![
        "127.0.0.1:80".parse().expect("parse :80"),
        "10.0.0.1:443".parse().expect("parse :443"),
        "[::1]:8080".parse().expect("parse ipv6 :8080"),
        "[2001:db8::1]:9443".parse().expect("parse ipv6 :9443"),
    ];
    let enc = encode_to_vec(&addrs).expect("encode Vec<SocketAddr>");
    let (dec, _bytes): (Vec<SocketAddr>, usize) =
        decode_from_slice(&enc).expect("decode Vec<SocketAddr>");
    assert_eq!(addrs, dec);
}

// ---------------------------------------------------------------------------
// 19. IpAddr::V4 vs V6 different wire bytes for different variants
// ---------------------------------------------------------------------------

#[test]
fn test_ipaddr_v4_v6_different_wire_bytes() {
    let v4 = IpAddr::V4("1.2.3.4".parse().expect("parse v4 for wire"));
    let v6 = IpAddr::V6("::1".parse().expect("parse v6 for wire"));

    let enc_v4 = encode_to_vec(&v4).expect("encode IpAddr::V4 for wire test");
    let enc_v6 = encode_to_vec(&v6).expect("encode IpAddr::V6 for wire test");

    // V4 discriminant is 0, V6 discriminant is 1
    assert_eq!(enc_v4[0], 0u8, "IpAddr::V4 discriminant must be 0");
    assert_eq!(enc_v6[0], 1u8, "IpAddr::V6 discriminant must be 1");

    // V4 = 1 discriminant + 4 octets = 5 bytes; V6 = 1 discriminant + 16 octets = 17 bytes
    assert_eq!(enc_v4.len(), 5, "IpAddr::V4 wire size must be 5 bytes");
    assert_eq!(enc_v6.len(), 17, "IpAddr::V6 wire size must be 17 bytes");

    assert_ne!(
        enc_v4, enc_v6,
        "V4 and V6 must produce different wire bytes"
    );
}

// ---------------------------------------------------------------------------
// 20. SocketAddrV4 big-endian config roundtrip
// ---------------------------------------------------------------------------

#[test]
fn test_socketaddrv4_big_endian_config_roundtrip() {
    let addr = SocketAddrV4::new(
        "172.16.0.1".parse::<Ipv4Addr>().expect("parse 172.16.0.1"),
        9090,
    );
    let cfg = config::standard().with_big_endian();
    let enc = encode_to_vec_with_config(&addr, cfg).expect("encode SocketAddrV4 big-endian");
    let (dec, _bytes): (SocketAddrV4, usize) =
        decode_from_slice_with_config(&enc, cfg).expect("decode SocketAddrV4 big-endian");
    assert_eq!(addr, dec);
}

// ---------------------------------------------------------------------------
// 21. BTreeMap<u8, Ipv4Addr> roundtrip
// ---------------------------------------------------------------------------

#[test]
fn test_btreemap_u8_ipv4addr_roundtrip() {
    let mut map: BTreeMap<u8, Ipv4Addr> = BTreeMap::new();
    map.insert(1, "10.0.0.1".parse().expect("parse 10.0.0.1"));
    map.insert(2, "172.16.0.1".parse().expect("parse 172.16.0.1"));
    map.insert(3, "192.168.1.1".parse().expect("parse 192.168.1.1"));
    map.insert(
        255,
        "255.255.255.255".parse().expect("parse 255.255.255.255"),
    );

    let enc = encode_to_vec(&map).expect("encode BTreeMap<u8, Ipv4Addr>");
    let (dec, _bytes): (BTreeMap<u8, Ipv4Addr>, usize) =
        decode_from_slice(&enc).expect("decode BTreeMap<u8, Ipv4Addr>");
    assert_eq!(map, dec);
}

// ---------------------------------------------------------------------------
// 22. (Ipv4Addr, Ipv6Addr) tuple roundtrip
// ---------------------------------------------------------------------------

#[test]
fn test_tuple_ipv4_ipv6_roundtrip() {
    let tuple: (Ipv4Addr, Ipv6Addr) = (
        "192.0.2.1".parse().expect("parse Ipv4 for tuple"),
        "2001:db8::ff00:42:8329"
            .parse()
            .expect("parse Ipv6 for tuple"),
    );
    let enc = encode_to_vec(&tuple).expect("encode (Ipv4Addr, Ipv6Addr)");
    let (dec, _bytes): ((Ipv4Addr, Ipv6Addr), usize) =
        decode_from_slice(&enc).expect("decode (Ipv4Addr, Ipv6Addr)");
    assert_eq!(tuple, dec);
    // Sanity: encoded length must be exactly 4 + 16 = 20 bytes
    assert_eq!(
        enc.len(),
        20,
        "(Ipv4Addr, Ipv6Addr) must encode as 4 + 16 = 20 bytes"
    );
}
