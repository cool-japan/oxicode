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
fn test_ipv4_loopback_roundtrip() {
    let addr = Ipv4Addr::new(127, 0, 0, 1);
    let encoded = encode_to_vec(&addr).expect("Failed to encode Ipv4Addr 127.0.0.1");
    let (decoded, _): (Ipv4Addr, _) =
        decode_from_slice(&encoded).expect("Failed to decode Ipv4Addr 127.0.0.1");
    assert_eq!(addr, decoded);
}

#[test]
fn test_ipv4_all_zeros_roundtrip() {
    let addr = Ipv4Addr::new(0, 0, 0, 0);
    let encoded = encode_to_vec(&addr).expect("Failed to encode Ipv4Addr 0.0.0.0");
    let (decoded, _): (Ipv4Addr, _) =
        decode_from_slice(&encoded).expect("Failed to decode Ipv4Addr 0.0.0.0");
    assert_eq!(addr, decoded);
}

#[test]
fn test_ipv4_broadcast_roundtrip() {
    let addr = Ipv4Addr::new(255, 255, 255, 255);
    let encoded = encode_to_vec(&addr).expect("Failed to encode Ipv4Addr 255.255.255.255");
    let (decoded, _): (Ipv4Addr, _) =
        decode_from_slice(&encoded).expect("Failed to decode Ipv4Addr 255.255.255.255");
    assert_eq!(addr, decoded);
}

#[test]
fn test_ipv4_private_roundtrip() {
    let addr = Ipv4Addr::new(192, 168, 1, 100);
    let encoded = encode_to_vec(&addr).expect("Failed to encode Ipv4Addr 192.168.1.100");
    let (decoded, _): (Ipv4Addr, _) =
        decode_from_slice(&encoded).expect("Failed to decode Ipv4Addr 192.168.1.100");
    assert_eq!(addr, decoded);
}

#[test]
fn test_ipv6_loopback_roundtrip() {
    let addr = Ipv6Addr::new(0, 0, 0, 0, 0, 0, 0, 1);
    let encoded = encode_to_vec(&addr).expect("Failed to encode Ipv6Addr ::1");
    let (decoded, _): (Ipv6Addr, _) =
        decode_from_slice(&encoded).expect("Failed to decode Ipv6Addr ::1");
    assert_eq!(addr, decoded);
}

#[test]
fn test_ipv6_all_zeros_roundtrip() {
    let addr = Ipv6Addr::new(0, 0, 0, 0, 0, 0, 0, 0);
    let encoded = encode_to_vec(&addr).expect("Failed to encode Ipv6Addr ::");
    let (decoded, _): (Ipv6Addr, _) =
        decode_from_slice(&encoded).expect("Failed to decode Ipv6Addr ::");
    assert_eq!(addr, decoded);
}

#[test]
fn test_ipv6_documentation_roundtrip() {
    let addr = Ipv6Addr::new(0x2001, 0x0db8, 0, 0, 0, 0, 0, 1);
    let encoded = encode_to_vec(&addr).expect("Failed to encode Ipv6Addr 2001:db8::1");
    let (decoded, _): (Ipv6Addr, _) =
        decode_from_slice(&encoded).expect("Failed to decode Ipv6Addr 2001:db8::1");
    assert_eq!(addr, decoded);
}

#[test]
fn test_ipaddr_v4_roundtrip() {
    let addr = IpAddr::V4(Ipv4Addr::new(10, 0, 0, 1));
    let encoded = encode_to_vec(&addr).expect("Failed to encode IpAddr::V4");
    let (decoded, _): (IpAddr, _) =
        decode_from_slice(&encoded).expect("Failed to decode IpAddr::V4");
    assert_eq!(addr, decoded);
}

#[test]
fn test_ipaddr_v6_roundtrip() {
    let addr = IpAddr::V6(Ipv6Addr::new(0xfe80, 0, 0, 0, 0, 0, 0, 1));
    let encoded = encode_to_vec(&addr).expect("Failed to encode IpAddr::V6");
    let (decoded, _): (IpAddr, _) =
        decode_from_slice(&encoded).expect("Failed to decode IpAddr::V6");
    assert_eq!(addr, decoded);
}

#[test]
fn test_socketaddrv4_loopback_8080_roundtrip() {
    let addr = SocketAddrV4::new(Ipv4Addr::new(127, 0, 0, 1), 8080);
    let encoded = encode_to_vec(&addr).expect("Failed to encode SocketAddrV4 127.0.0.1:8080");
    let (decoded, _): (SocketAddrV4, _) =
        decode_from_slice(&encoded).expect("Failed to decode SocketAddrV4 127.0.0.1:8080");
    assert_eq!(addr, decoded);
}

#[test]
fn test_socketaddrv4_unspecified_443_roundtrip() {
    let addr = SocketAddrV4::new(Ipv4Addr::new(0, 0, 0, 0), 443);
    let encoded = encode_to_vec(&addr).expect("Failed to encode SocketAddrV4 0.0.0.0:443");
    let (decoded, _): (SocketAddrV4, _) =
        decode_from_slice(&encoded).expect("Failed to decode SocketAddrV4 0.0.0.0:443");
    assert_eq!(addr, decoded);
}

#[test]
fn test_socketaddrv6_loopback_9000_roundtrip() {
    let addr = SocketAddrV6::new(Ipv6Addr::new(0, 0, 0, 0, 0, 0, 0, 1), 9000, 0, 0);
    let encoded = encode_to_vec(&addr).expect("Failed to encode SocketAddrV6 ::1:9000");
    let (decoded, _): (SocketAddrV6, _) =
        decode_from_slice(&encoded).expect("Failed to decode SocketAddrV6 ::1:9000");
    assert_eq!(addr, decoded);
}

#[test]
fn test_socketaddr_v4_roundtrip() {
    let addr = SocketAddr::V4(SocketAddrV4::new(Ipv4Addr::new(172, 16, 0, 1), 3000));
    let encoded = encode_to_vec(&addr).expect("Failed to encode SocketAddr::V4");
    let (decoded, _): (SocketAddr, _) =
        decode_from_slice(&encoded).expect("Failed to decode SocketAddr::V4");
    assert_eq!(addr, decoded);
}

#[test]
fn test_socketaddr_v6_roundtrip() {
    let addr = SocketAddr::V6(SocketAddrV6::new(
        Ipv6Addr::new(0x2001, 0x0db8, 0, 0, 0, 0, 0, 0x0001),
        443,
        0,
        0,
    ));
    let encoded = encode_to_vec(&addr).expect("Failed to encode SocketAddr::V6");
    let (decoded, _): (SocketAddr, _) =
        decode_from_slice(&encoded).expect("Failed to decode SocketAddr::V6");
    assert_eq!(addr, decoded);
}

#[test]
fn test_vec_ipv4addr_roundtrip() {
    let addrs = vec![
        Ipv4Addr::new(1, 2, 3, 4),
        Ipv4Addr::new(10, 0, 0, 1),
        Ipv4Addr::new(172, 16, 254, 1),
        Ipv4Addr::new(192, 168, 0, 255),
        Ipv4Addr::new(255, 255, 255, 0),
    ];
    let encoded = encode_to_vec(&addrs).expect("Failed to encode Vec<Ipv4Addr>");
    let (decoded, _): (Vec<Ipv4Addr>, _) =
        decode_from_slice(&encoded).expect("Failed to decode Vec<Ipv4Addr>");
    assert_eq!(addrs, decoded);
}

#[test]
fn test_vec_ipaddr_mixed_roundtrip() {
    let addrs = vec![
        IpAddr::V4(Ipv4Addr::new(192, 168, 1, 1)),
        IpAddr::V6(Ipv6Addr::new(0xfe80, 0, 0, 0, 0, 0, 0, 0x0001)),
        IpAddr::V4(Ipv4Addr::new(10, 0, 0, 2)),
        IpAddr::V6(Ipv6Addr::new(0, 0, 0, 0, 0, 0, 0, 1)),
    ];
    let encoded = encode_to_vec(&addrs).expect("Failed to encode Vec<IpAddr> mixed");
    let (decoded, _): (Vec<IpAddr>, _) =
        decode_from_slice(&encoded).expect("Failed to decode Vec<IpAddr> mixed");
    assert_eq!(addrs, decoded);
}

#[test]
fn test_option_socketaddr_some_roundtrip() {
    let addr: Option<SocketAddr> = Some(SocketAddr::V4(SocketAddrV4::new(
        Ipv4Addr::new(127, 0, 0, 1),
        8443,
    )));
    let encoded = encode_to_vec(&addr).expect("Failed to encode Option<SocketAddr> Some");
    let (decoded, _): (Option<SocketAddr>, _) =
        decode_from_slice(&encoded).expect("Failed to decode Option<SocketAddr> Some");
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
fn test_ipv4addr_consumed_bytes_equals_encoded_len() {
    let addr = Ipv4Addr::new(10, 20, 30, 40);
    let encoded = encode_to_vec(&addr).expect("Failed to encode Ipv4Addr for byte length check");
    let (_, consumed): (Ipv4Addr, _) =
        decode_from_slice(&encoded).expect("Failed to decode Ipv4Addr for byte length check");
    assert_eq!(consumed, encoded.len());
}

#[test]
fn test_two_different_ipv4addrs_produce_different_encodings() {
    let addr_a = Ipv4Addr::new(1, 1, 1, 1);
    let addr_b = Ipv4Addr::new(8, 8, 8, 8);
    let encoded_a = encode_to_vec(&addr_a).expect("Failed to encode Ipv4Addr 1.1.1.1");
    let encoded_b = encode_to_vec(&addr_b).expect("Failed to encode Ipv4Addr 8.8.8.8");
    assert_ne!(encoded_a, encoded_b);
}

#[test]
fn test_ipv4addr_fixed_int_config_roundtrip() {
    let cfg = config::standard().with_fixed_int_encoding();
    let addr = Ipv4Addr::new(100, 200, 10, 50);
    let encoded = encode_to_vec_with_config(&addr, cfg)
        .expect("Failed to encode Ipv4Addr with fixed int config");
    let (decoded, _): (Ipv4Addr, _) = decode_from_slice_with_config(&encoded, cfg)
        .expect("Failed to decode Ipv4Addr with fixed int config");
    assert_eq!(addr, decoded);
}

#[test]
fn test_vec_socketaddrv4_roundtrip() {
    let addrs = vec![
        SocketAddrV4::new(Ipv4Addr::new(127, 0, 0, 1), 80),
        SocketAddrV4::new(Ipv4Addr::new(192, 168, 0, 10), 8080),
        SocketAddrV4::new(Ipv4Addr::new(10, 0, 0, 5), 443),
    ];
    let encoded = encode_to_vec(&addrs).expect("Failed to encode Vec<SocketAddrV4>");
    let (decoded, _): (Vec<SocketAddrV4>, _) =
        decode_from_slice(&encoded).expect("Failed to decode Vec<SocketAddrV4>");
    assert_eq!(addrs, decoded);
}
