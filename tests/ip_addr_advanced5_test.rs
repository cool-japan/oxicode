#![cfg(feature = "std")]

//! Advanced tests for IP address and network type encoding (set 5).

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
    encode_to_vec_with_config, Decode, Encode,
};
use std::net::{IpAddr, Ipv4Addr, Ipv6Addr, SocketAddr, SocketAddrV4, SocketAddrV6};

#[derive(Debug, PartialEq, Encode, Decode)]
struct ServiceEndpoint {
    host: IpAddr,
    port: u16,
    name: String,
    tls: bool,
}

// Test 1: Ipv4Addr roundtrip (192.168.1.1)
#[test]
fn test_ipv4addr_192_168_1_1_roundtrip() {
    let ip = Ipv4Addr::new(192, 168, 1, 1);
    let bytes = encode_to_vec(&ip).expect("encode Ipv4Addr 192.168.1.1");
    let (decoded, _): (Ipv4Addr, _) =
        decode_from_slice(&bytes).expect("decode Ipv4Addr 192.168.1.1");
    assert_eq!(ip, decoded);
}

// Test 2: Ipv4Addr roundtrip (0.0.0.0)
#[test]
fn test_ipv4addr_unspecified_roundtrip() {
    let ip = Ipv4Addr::new(0, 0, 0, 0);
    let bytes = encode_to_vec(&ip).expect("encode Ipv4Addr 0.0.0.0");
    let (decoded, _): (Ipv4Addr, _) = decode_from_slice(&bytes).expect("decode Ipv4Addr 0.0.0.0");
    assert_eq!(ip, decoded);
}

// Test 3: Ipv4Addr roundtrip (255.255.255.255)
#[test]
fn test_ipv4addr_broadcast_roundtrip() {
    let ip = Ipv4Addr::new(255, 255, 255, 255);
    let bytes = encode_to_vec(&ip).expect("encode Ipv4Addr 255.255.255.255");
    let (decoded, _): (Ipv4Addr, _) =
        decode_from_slice(&bytes).expect("decode Ipv4Addr 255.255.255.255");
    assert_eq!(ip, decoded);
}

// Test 4: Ipv6Addr roundtrip (::1 loopback)
#[test]
fn test_ipv6addr_loopback_roundtrip() {
    let ip = Ipv6Addr::LOCALHOST;
    let bytes = encode_to_vec(&ip).expect("encode Ipv6Addr ::1");
    let (decoded, _): (Ipv6Addr, _) = decode_from_slice(&bytes).expect("decode Ipv6Addr ::1");
    assert_eq!(ip, decoded);
}

// Test 5: Ipv6Addr roundtrip (all zeros)
#[test]
fn test_ipv6addr_all_zeros_roundtrip() {
    let ip = Ipv6Addr::new(0, 0, 0, 0, 0, 0, 0, 0);
    let bytes = encode_to_vec(&ip).expect("encode Ipv6Addr all-zeros");
    let (decoded, _): (Ipv6Addr, _) = decode_from_slice(&bytes).expect("decode Ipv6Addr all-zeros");
    assert_eq!(ip, decoded);
}

// Test 6: Ipv6Addr roundtrip (ff02::1 all-nodes multicast)
#[test]
fn test_ipv6addr_ff02_all_nodes_roundtrip() {
    let ip = Ipv6Addr::new(0xff02, 0, 0, 0, 0, 0, 0, 1);
    let bytes = encode_to_vec(&ip).expect("encode Ipv6Addr ff02::1");
    let (decoded, _): (Ipv6Addr, _) = decode_from_slice(&bytes).expect("decode Ipv6Addr ff02::1");
    assert_eq!(ip, decoded);
}

// Test 7: IpAddr::V4 roundtrip
#[test]
fn test_ipaddr_v4_roundtrip() {
    let ip = IpAddr::V4(Ipv4Addr::new(10, 20, 30, 40));
    let bytes = encode_to_vec(&ip).expect("encode IpAddr::V4");
    let (decoded, _): (IpAddr, _) = decode_from_slice(&bytes).expect("decode IpAddr::V4");
    assert_eq!(ip, decoded);
}

// Test 8: IpAddr::V6 roundtrip
#[test]
fn test_ipaddr_v6_roundtrip() {
    let ip = IpAddr::V6(Ipv6Addr::new(0x2001, 0xdb8, 0, 0, 0, 0, 0, 1));
    let bytes = encode_to_vec(&ip).expect("encode IpAddr::V6");
    let (decoded, _): (IpAddr, _) = decode_from_slice(&bytes).expect("decode IpAddr::V6");
    assert_eq!(ip, decoded);
}

// Test 9: SocketAddrV4 roundtrip
#[test]
fn test_socketaddrv4_roundtrip() {
    let addr = SocketAddrV4::new(Ipv4Addr::new(172, 16, 0, 1), 8080);
    let bytes = encode_to_vec(&addr).expect("encode SocketAddrV4");
    let (decoded, _): (SocketAddrV4, _) = decode_from_slice(&bytes).expect("decode SocketAddrV4");
    assert_eq!(addr, decoded);
}

// Test 10: SocketAddrV6 roundtrip
#[test]
fn test_socketaddrv6_roundtrip() {
    let addr = SocketAddrV6::new(Ipv6Addr::new(0xfe80, 0, 0, 0, 0, 0, 0, 1), 9443, 0, 5);
    let bytes = encode_to_vec(&addr).expect("encode SocketAddrV6");
    let (decoded, _): (SocketAddrV6, _) = decode_from_slice(&bytes).expect("decode SocketAddrV6");
    assert_eq!(addr, decoded);
}

// Test 11: SocketAddr::V4 roundtrip
#[test]
fn test_socketaddr_v4_roundtrip() {
    let addr = SocketAddr::V4(SocketAddrV4::new(Ipv4Addr::new(127, 0, 0, 1), 3000));
    let bytes = encode_to_vec(&addr).expect("encode SocketAddr::V4");
    let (decoded, _): (SocketAddr, _) = decode_from_slice(&bytes).expect("decode SocketAddr::V4");
    assert_eq!(addr, decoded);
}

// Test 12: SocketAddr::V6 roundtrip
#[test]
fn test_socketaddr_v6_roundtrip() {
    let addr = SocketAddr::V6(SocketAddrV6::new(Ipv6Addr::LOCALHOST, 8443, 0, 0));
    let bytes = encode_to_vec(&addr).expect("encode SocketAddr::V6");
    let (decoded, _): (SocketAddr, _) = decode_from_slice(&bytes).expect("decode SocketAddr::V6");
    assert_eq!(addr, decoded);
}

// Test 13: ServiceEndpoint with IpAddr::V4 roundtrip
#[test]
fn test_service_endpoint_ipv4_roundtrip() {
    let ep = ServiceEndpoint {
        host: IpAddr::V4(Ipv4Addr::new(192, 168, 1, 100)),
        port: 443,
        name: String::from("api-server"),
        tls: true,
    };
    let bytes = encode_to_vec(&ep).expect("encode ServiceEndpoint ipv4");
    let (decoded, _): (ServiceEndpoint, _) =
        decode_from_slice(&bytes).expect("decode ServiceEndpoint ipv4");
    assert_eq!(ep, decoded);
}

// Test 14: ServiceEndpoint with IpAddr::V6 roundtrip
#[test]
fn test_service_endpoint_ipv6_roundtrip() {
    let ep = ServiceEndpoint {
        host: IpAddr::V6(Ipv6Addr::new(0x2001, 0xdb8, 0xabcd, 0, 0, 0, 0, 1)),
        port: 8080,
        name: String::from("backend-v6"),
        tls: false,
    };
    let bytes = encode_to_vec(&ep).expect("encode ServiceEndpoint ipv6");
    let (decoded, _): (ServiceEndpoint, _) =
        decode_from_slice(&bytes).expect("decode ServiceEndpoint ipv6");
    assert_eq!(ep, decoded);
}

// Test 15: Vec<IpAddr> roundtrip
#[test]
fn test_vec_ipaddr_roundtrip() {
    let ips: Vec<IpAddr> = vec![
        IpAddr::V4(Ipv4Addr::new(1, 1, 1, 1)),
        IpAddr::V6(Ipv6Addr::LOCALHOST),
        IpAddr::V4(Ipv4Addr::new(8, 8, 8, 8)),
    ];
    let bytes = encode_to_vec(&ips).expect("encode Vec<IpAddr>");
    let (decoded, _): (Vec<IpAddr>, _) = decode_from_slice(&bytes).expect("decode Vec<IpAddr>");
    assert_eq!(ips, decoded);
}

// Test 16: Vec<SocketAddr> roundtrip
#[test]
fn test_vec_socketaddr_roundtrip() {
    let addrs: Vec<SocketAddr> = vec![
        SocketAddr::V4(SocketAddrV4::new(Ipv4Addr::new(127, 0, 0, 1), 80)),
        SocketAddr::V4(SocketAddrV4::new(Ipv4Addr::new(0, 0, 0, 0), 443)),
        SocketAddr::V6(SocketAddrV6::new(Ipv6Addr::LOCALHOST, 8080, 0, 0)),
    ];
    let bytes = encode_to_vec(&addrs).expect("encode Vec<SocketAddr>");
    let (decoded, _): (Vec<SocketAddr>, _) =
        decode_from_slice(&bytes).expect("decode Vec<SocketAddr>");
    assert_eq!(addrs, decoded);
}

// Test 17: Option<IpAddr> Some roundtrip
#[test]
fn test_option_ipaddr_some_roundtrip() {
    let opt: Option<IpAddr> = Some(IpAddr::V4(Ipv4Addr::new(10, 0, 0, 1)));
    let bytes = encode_to_vec(&opt).expect("encode Option<IpAddr> Some");
    let (decoded, _): (Option<IpAddr>, _) =
        decode_from_slice(&bytes).expect("decode Option<IpAddr> Some");
    assert_eq!(opt, decoded);
}

// Test 18: Option<SocketAddr> None roundtrip
#[test]
fn test_option_socketaddr_none_roundtrip() {
    let opt: Option<SocketAddr> = None;
    let bytes = encode_to_vec(&opt).expect("encode Option<SocketAddr> None");
    let (decoded, _): (Option<SocketAddr>, _) =
        decode_from_slice(&bytes).expect("decode Option<SocketAddr> None");
    assert_eq!(opt, decoded);
}

// Test 19: Ipv4Addr encodes as 4 bytes with fixed-int config
#[test]
fn test_ipv4addr_fixed_int_config_encodes_as_4_bytes() {
    let ip = Ipv4Addr::new(192, 168, 1, 1);
    let cfg = config::standard().with_fixed_int_encoding();
    let bytes = encode_to_vec_with_config(&ip, cfg).expect("encode Ipv4Addr with fixed-int config");
    assert_eq!(
        bytes.len(),
        4,
        "Ipv4Addr must always encode as exactly 4 bytes"
    );
    let (decoded, _): (Ipv4Addr, _) =
        decode_from_slice_with_config(&bytes, cfg).expect("decode Ipv4Addr with fixed-int config");
    assert_eq!(ip, decoded);
}

// Test 20: Consumed bytes equals encoded length for ServiceEndpoint
#[test]
fn test_service_endpoint_consumed_bytes_equals_encoded_length() {
    let ep = ServiceEndpoint {
        host: IpAddr::V4(Ipv4Addr::new(203, 0, 113, 42)),
        port: 7070,
        name: String::from("metrics"),
        tls: true,
    };
    let bytes = encode_to_vec(&ep).expect("encode ServiceEndpoint for consumed check");
    let (_decoded, consumed): (ServiceEndpoint, _) =
        decode_from_slice(&bytes).expect("decode ServiceEndpoint for consumed check");
    assert_eq!(
        consumed,
        bytes.len(),
        "consumed bytes must equal total encoded length"
    );
}

// Test 21: IpAddr::V4 and IpAddr::V6 produce different encodings
#[test]
fn test_ipaddr_v4_and_v6_produce_different_encodings() {
    let v4 = IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1));
    let v6 = IpAddr::V6(Ipv6Addr::LOCALHOST);
    let bytes_v4 = encode_to_vec(&v4).expect("encode IpAddr::V4 for diff check");
    let bytes_v6 = encode_to_vec(&v6).expect("encode IpAddr::V6 for diff check");
    assert_ne!(
        bytes_v4, bytes_v6,
        "IpAddr::V4 and IpAddr::V6 must produce distinct byte encodings"
    );
    // V4 encodes as 1 discriminant byte + 4 octets = 5 bytes
    assert_eq!(bytes_v4.len(), 5, "IpAddr::V4 must be 5 bytes");
    // V6 encodes as 1 discriminant byte + 16 octets = 17 bytes
    assert_eq!(bytes_v6.len(), 17, "IpAddr::V6 must be 17 bytes");
}

// Test 22: Vec<ServiceEndpoint> roundtrip (3 items)
#[test]
fn test_vec_service_endpoint_three_items_roundtrip() {
    let endpoints: Vec<ServiceEndpoint> = vec![
        ServiceEndpoint {
            host: IpAddr::V4(Ipv4Addr::new(10, 0, 0, 1)),
            port: 80,
            name: String::from("web"),
            tls: false,
        },
        ServiceEndpoint {
            host: IpAddr::V6(Ipv6Addr::new(0x2001, 0xdb8, 0, 0, 0, 0, 0, 2)),
            port: 443,
            name: String::from("secure-web"),
            tls: true,
        },
        ServiceEndpoint {
            host: IpAddr::V4(Ipv4Addr::new(192, 168, 100, 5)),
            port: 5432,
            name: String::from("database"),
            tls: true,
        },
    ];
    let bytes = encode_to_vec(&endpoints).expect("encode Vec<ServiceEndpoint>");
    let (decoded, _): (Vec<ServiceEndpoint>, _) =
        decode_from_slice(&bytes).expect("decode Vec<ServiceEndpoint>");
    assert_eq!(endpoints, decoded);
}
