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
use oxicode::{
    config, decode_from_slice, decode_from_slice_with_config, encode_to_vec,
    encode_to_vec_with_config,
};
use std::net::{IpAddr, Ipv4Addr, Ipv6Addr, SocketAddr, SocketAddrV4, SocketAddrV6};

#[test]
fn test_ipv4_loopback_127_0_0_1_roundtrip() {
    let addr = "127.0.0.1".parse::<Ipv4Addr>().expect("parse");
    let encoded = encode_to_vec(&addr).expect("Failed to encode Ipv4Addr 127.0.0.1");
    let (decoded, _): (Ipv4Addr, _) =
        decode_from_slice(&encoded).expect("Failed to decode Ipv4Addr 127.0.0.1");
    assert_eq!(addr, decoded);
}

#[test]
fn test_ipv4_private_192_168_1_1_roundtrip() {
    let addr = "192.168.1.1".parse::<Ipv4Addr>().expect("parse");
    let encoded = encode_to_vec(&addr).expect("Failed to encode Ipv4Addr 192.168.1.1");
    let (decoded, _): (Ipv4Addr, _) =
        decode_from_slice(&encoded).expect("Failed to decode Ipv4Addr 192.168.1.1");
    assert_eq!(addr, decoded);
}

#[test]
fn test_ipv4_all_zeros_0_0_0_0_roundtrip() {
    let addr = "0.0.0.0".parse::<Ipv4Addr>().expect("parse");
    let encoded = encode_to_vec(&addr).expect("Failed to encode Ipv4Addr 0.0.0.0");
    let (decoded, _): (Ipv4Addr, _) =
        decode_from_slice(&encoded).expect("Failed to decode Ipv4Addr 0.0.0.0");
    assert_eq!(addr, decoded);
}

#[test]
fn test_ipv4_broadcast_255_255_255_255_roundtrip() {
    let addr = "255.255.255.255".parse::<Ipv4Addr>().expect("parse");
    let encoded = encode_to_vec(&addr).expect("Failed to encode Ipv4Addr 255.255.255.255");
    let (decoded, _): (Ipv4Addr, _) =
        decode_from_slice(&encoded).expect("Failed to decode Ipv4Addr 255.255.255.255");
    assert_eq!(addr, decoded);
}

#[test]
fn test_ipv6_loopback_roundtrip() {
    let addr = "::1".parse::<Ipv6Addr>().expect("parse");
    let encoded = encode_to_vec(&addr).expect("Failed to encode Ipv6Addr ::1");
    let (decoded, _): (Ipv6Addr, _) =
        decode_from_slice(&encoded).expect("Failed to decode Ipv6Addr ::1");
    assert_eq!(addr, decoded);
}

#[test]
fn test_ipv6_all_zeros_roundtrip() {
    let addr = "::".parse::<Ipv6Addr>().expect("parse");
    let encoded = encode_to_vec(&addr).expect("Failed to encode Ipv6Addr ::");
    let (decoded, _): (Ipv6Addr, _) =
        decode_from_slice(&encoded).expect("Failed to decode Ipv6Addr ::");
    assert_eq!(addr, decoded);
}

#[test]
fn test_ipv6_full_128bit_2001_db8_roundtrip() {
    let addr = "2001:db8::1".parse::<Ipv6Addr>().expect("parse");
    let encoded = encode_to_vec(&addr).expect("Failed to encode Ipv6Addr 2001:db8::1");
    let (decoded, _): (Ipv6Addr, _) =
        decode_from_slice(&encoded).expect("Failed to decode Ipv6Addr 2001:db8::1");
    assert_eq!(addr, decoded);
}

#[test]
fn test_ipaddr_v4_variant_roundtrip() {
    let addr = IpAddr::V4("10.0.0.1".parse::<Ipv4Addr>().expect("parse"));
    let encoded = encode_to_vec(&addr).expect("Failed to encode IpAddr::V4");
    let (decoded, _): (IpAddr, _) =
        decode_from_slice(&encoded).expect("Failed to decode IpAddr::V4");
    assert_eq!(addr, decoded);
}

#[test]
fn test_ipaddr_v6_variant_roundtrip() {
    let addr = IpAddr::V6("fe80::1".parse::<Ipv6Addr>().expect("parse"));
    let encoded = encode_to_vec(&addr).expect("Failed to encode IpAddr::V6");
    let (decoded, _): (IpAddr, _) =
        decode_from_slice(&encoded).expect("Failed to decode IpAddr::V6");
    assert_eq!(addr, decoded);
}

#[test]
fn test_socketaddr_v4_port_8080_roundtrip() {
    let addr: SocketAddr = "127.0.0.1:8080".parse().expect("parse");
    let encoded = encode_to_vec(&addr).expect("Failed to encode SocketAddr V4 port 8080");
    let (decoded, _): (SocketAddr, _) =
        decode_from_slice(&encoded).expect("Failed to decode SocketAddr V4 port 8080");
    assert_eq!(addr, decoded);
}

#[test]
fn test_socketaddr_v4_port_0_roundtrip() {
    let addr: SocketAddr = "0.0.0.0:0".parse().expect("parse");
    let encoded = encode_to_vec(&addr).expect("Failed to encode SocketAddr V4 port 0");
    let (decoded, _): (SocketAddr, _) =
        decode_from_slice(&encoded).expect("Failed to decode SocketAddr V4 port 0");
    assert_eq!(addr, decoded);
}

#[test]
fn test_socketaddr_v6_port_443_roundtrip() {
    let addr: SocketAddr = "[::1]:443".parse().expect("parse");
    let encoded = encode_to_vec(&addr).expect("Failed to encode SocketAddr V6 port 443");
    let (decoded, _): (SocketAddr, _) =
        decode_from_slice(&encoded).expect("Failed to decode SocketAddr V6 port 443");
    assert_eq!(addr, decoded);
}

#[test]
fn test_socketaddrv4_roundtrip() {
    let addr = SocketAddrV4::new("192.168.1.100".parse::<Ipv4Addr>().expect("parse"), 9090);
    let encoded = encode_to_vec(&addr).expect("Failed to encode SocketAddrV4");
    let (decoded, _): (SocketAddrV4, _) =
        decode_from_slice(&encoded).expect("Failed to decode SocketAddrV4");
    assert_eq!(addr, decoded);
}

#[test]
fn test_socketaddrv6_roundtrip() {
    let ip = "2001:db8::1".parse::<Ipv6Addr>().expect("parse");
    let addr = SocketAddrV6::new(ip, 8443, 0, 0);
    let encoded = encode_to_vec(&addr).expect("Failed to encode SocketAddrV6");
    let (decoded, _): (SocketAddrV6, _) =
        decode_from_slice(&encoded).expect("Failed to decode SocketAddrV6");
    assert_eq!(addr, decoded);
}

#[test]
fn test_vec_ipv4addr_four_items_roundtrip() {
    let addrs = vec![
        "1.2.3.4".parse::<Ipv4Addr>().expect("parse"),
        "10.0.0.1".parse::<Ipv4Addr>().expect("parse"),
        "172.16.254.1".parse::<Ipv4Addr>().expect("parse"),
        "192.168.0.255".parse::<Ipv4Addr>().expect("parse"),
    ];
    let encoded = encode_to_vec(&addrs).expect("Failed to encode Vec<Ipv4Addr>");
    let (decoded, _): (Vec<Ipv4Addr>, _) =
        decode_from_slice(&encoded).expect("Failed to decode Vec<Ipv4Addr>");
    assert_eq!(addrs, decoded);
}

#[test]
fn test_vec_ipaddr_mixed_v4_v6_four_items_roundtrip() {
    let addrs = vec![
        IpAddr::V4("192.168.1.1".parse::<Ipv4Addr>().expect("parse")),
        IpAddr::V6("fe80::1".parse::<Ipv6Addr>().expect("parse")),
        IpAddr::V4("10.0.0.2".parse::<Ipv4Addr>().expect("parse")),
        IpAddr::V6("::1".parse::<Ipv6Addr>().expect("parse")),
    ];
    let encoded = encode_to_vec(&addrs).expect("Failed to encode Vec<IpAddr> mixed");
    let (decoded, _): (Vec<IpAddr>, _) =
        decode_from_slice(&encoded).expect("Failed to decode Vec<IpAddr> mixed");
    assert_eq!(addrs, decoded);
}

#[test]
fn test_vec_socketaddr_three_items_roundtrip() {
    let addrs = vec![
        "127.0.0.1:80".parse::<SocketAddr>().expect("parse"),
        "192.168.0.10:8080".parse::<SocketAddr>().expect("parse"),
        "[::1]:443".parse::<SocketAddr>().expect("parse"),
    ];
    let encoded = encode_to_vec(&addrs).expect("Failed to encode Vec<SocketAddr>");
    let (decoded, _): (Vec<SocketAddr>, _) =
        decode_from_slice(&encoded).expect("Failed to decode Vec<SocketAddr>");
    assert_eq!(addrs, decoded);
}

#[test]
fn test_option_ipaddr_some_roundtrip() {
    let addr: Option<IpAddr> = Some(IpAddr::V4(
        "10.10.10.10".parse::<Ipv4Addr>().expect("parse"),
    ));
    let encoded = encode_to_vec(&addr).expect("Failed to encode Option<IpAddr> Some");
    let (decoded, _): (Option<IpAddr>, _) =
        decode_from_slice(&encoded).expect("Failed to decode Option<IpAddr> Some");
    assert_eq!(addr, decoded);
}

#[test]
fn test_option_ipaddr_none_roundtrip() {
    let addr: Option<IpAddr> = None;
    let encoded = encode_to_vec(&addr).expect("Failed to encode Option<IpAddr> None");
    let (decoded, _): (Option<IpAddr>, _) =
        decode_from_slice(&encoded).expect("Failed to decode Option<IpAddr> None");
    assert_eq!(addr, decoded);
}

#[test]
fn test_ipaddr_v4_and_v6_produce_different_encodings() {
    let v4 = IpAddr::V4("127.0.0.1".parse::<Ipv4Addr>().expect("parse"));
    let v6 = IpAddr::V6("::1".parse::<Ipv6Addr>().expect("parse"));
    let encoded_v4 = encode_to_vec(&v4).expect("Failed to encode IpAddr::V4 for discriminant test");
    let encoded_v6 = encode_to_vec(&v6).expect("Failed to encode IpAddr::V6 for discriminant test");
    assert_ne!(
        encoded_v4, encoded_v6,
        "IpAddr V4 and V6 must have different encodings due to different discriminants"
    );
    assert_ne!(
        encoded_v4[0], encoded_v6[0],
        "First byte (discriminant) must differ between V4 and V6 variants"
    );
}

#[test]
fn test_consumed_bytes_equals_encoded_length_for_socketaddr() {
    let addr: SocketAddr = "192.168.1.50:5000".parse().expect("parse");
    let encoded =
        encode_to_vec(&addr).expect("Failed to encode SocketAddr for consumed bytes test");
    let (_, consumed): (SocketAddr, _) =
        decode_from_slice(&encoded).expect("Failed to decode SocketAddr for consumed bytes test");
    assert_eq!(
        consumed,
        encoded.len(),
        "Consumed bytes must equal the full encoded length for SocketAddr"
    );
}

#[test]
fn test_ipv4addr_with_fixed_int_config_roundtrip() {
    let cfg = config::standard().with_fixed_int_encoding();
    let addr = "172.20.30.40".parse::<Ipv4Addr>().expect("parse");
    let encoded = encode_to_vec_with_config(&addr, cfg)
        .expect("Failed to encode Ipv4Addr with fixed-int config");
    let (decoded, _): (Ipv4Addr, _) = decode_from_slice_with_config(&encoded, cfg)
        .expect("Failed to decode Ipv4Addr with fixed-int config");
    assert_eq!(addr, decoded);
}
