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

// ===== 1. Ipv4Addr::LOCALHOST roundtrip (127.0.0.1) =====

#[test]
fn test_ipv4_localhost_roundtrip() {
    let addr = Ipv4Addr::LOCALHOST;
    let enc = encode_to_vec(&addr).expect("encode Ipv4Addr localhost");
    let (dec, _): (Ipv4Addr, usize) = decode_from_slice(&enc).expect("decode Ipv4Addr localhost");
    assert_eq!(addr, dec);
}

// ===== 2. Ipv4Addr::BROADCAST roundtrip (255.255.255.255) =====

#[test]
fn test_ipv4_broadcast_roundtrip() {
    let addr = Ipv4Addr::BROADCAST;
    let enc = encode_to_vec(&addr).expect("encode Ipv4Addr broadcast");
    let (dec, _): (Ipv4Addr, usize) = decode_from_slice(&enc).expect("decode Ipv4Addr broadcast");
    assert_eq!(addr, dec);
    assert_eq!(dec.octets(), [255, 255, 255, 255]);
}

// ===== 3. Ipv4Addr::new(192, 168, 1, 1) roundtrip =====

#[test]
fn test_ipv4_custom_192_168_1_1_roundtrip() {
    let addr = Ipv4Addr::new(192, 168, 1, 1);
    let enc = encode_to_vec(&addr).expect("encode Ipv4Addr 192.168.1.1");
    let (dec, _): (Ipv4Addr, usize) = decode_from_slice(&enc).expect("decode Ipv4Addr 192.168.1.1");
    assert_eq!(addr, dec);
    assert_eq!(dec.octets(), [192, 168, 1, 1]);
}

// ===== 4. Ipv4Addr encodes to 4 bytes (fixed layout) =====

#[test]
fn test_ipv4_encodes_to_4_bytes() {
    let addr = Ipv4Addr::new(10, 20, 30, 40);
    let enc = encode_to_vec(&addr).expect("encode Ipv4Addr 10.20.30.40");
    assert_eq!(enc.len(), 4, "Ipv4Addr must encode to exactly 4 bytes");
    assert_eq!(enc, &[10, 20, 30, 40]);
}

// ===== 5. Ipv6Addr::LOCALHOST roundtrip (::1) =====

#[test]
fn test_ipv6_localhost_roundtrip() {
    let addr = Ipv6Addr::LOCALHOST;
    let enc = encode_to_vec(&addr).expect("encode Ipv6Addr localhost");
    let (dec, _): (Ipv6Addr, usize) = decode_from_slice(&enc).expect("decode Ipv6Addr localhost");
    assert_eq!(addr, dec);
}

// ===== 6. Ipv6Addr::UNSPECIFIED roundtrip (::) =====

#[test]
fn test_ipv6_unspecified_roundtrip() {
    let addr = Ipv6Addr::UNSPECIFIED;
    let enc = encode_to_vec(&addr).expect("encode Ipv6Addr unspecified");
    let (dec, _): (Ipv6Addr, usize) = decode_from_slice(&enc).expect("decode Ipv6Addr unspecified");
    assert_eq!(addr, dec);
    assert_eq!(dec.octets(), [0u8; 16]);
}

// ===== 7. Ipv6Addr custom address roundtrip =====

#[test]
fn test_ipv6_custom_roundtrip() {
    let addr = Ipv6Addr::new(
        0x2001, 0x0db8, 0x85a3, 0x0000, 0x0000, 0x8a2e, 0x0370, 0x7334,
    );
    let enc = encode_to_vec(&addr).expect("encode custom Ipv6Addr");
    let (dec, _): (Ipv6Addr, usize) = decode_from_slice(&enc).expect("decode custom Ipv6Addr");
    assert_eq!(addr, dec);
}

// ===== 8. Ipv6Addr encodes to 16 bytes =====

#[test]
fn test_ipv6_encodes_to_16_bytes() {
    let addr = Ipv6Addr::LOCALHOST;
    let enc = encode_to_vec(&addr).expect("encode Ipv6Addr::LOCALHOST for length check");
    assert_eq!(enc.len(), 16, "Ipv6Addr must encode to exactly 16 bytes");
    assert_eq!(&enc[..15], &[0u8; 15]);
    assert_eq!(enc[15], 1u8);
}

// ===== 9. IpAddr::V4 variant roundtrip =====

#[test]
fn test_ipaddr_v4_roundtrip() {
    let addr = IpAddr::V4(Ipv4Addr::new(172, 16, 0, 1));
    let enc = encode_to_vec(&addr).expect("encode IpAddr::V4");
    let (dec, _): (IpAddr, usize) = decode_from_slice(&enc).expect("decode IpAddr::V4");
    assert_eq!(addr, dec);
    assert!(dec.is_ipv4(), "decoded IpAddr must be V4");
}

// ===== 10. IpAddr::V6 variant roundtrip =====

#[test]
fn test_ipaddr_v6_roundtrip() {
    let addr = IpAddr::V6(Ipv6Addr::LOCALHOST);
    let enc = encode_to_vec(&addr).expect("encode IpAddr::V6");
    let (dec, _): (IpAddr, usize) = decode_from_slice(&enc).expect("decode IpAddr::V6");
    assert_eq!(addr, dec);
    assert!(dec.is_ipv6(), "decoded IpAddr must be V6");
}

// ===== 11. IpAddr::V4 and IpAddr::V6 produce different discriminant bytes =====

#[test]
fn test_ipaddr_v4_v6_different_discriminant_bytes() {
    let v4 = IpAddr::V4(Ipv4Addr::new(1, 2, 3, 4));
    let v6 = IpAddr::V6(Ipv6Addr::LOCALHOST);
    let enc_v4 = encode_to_vec(&v4).expect("encode IpAddr::V4 for discriminant check");
    let enc_v6 = encode_to_vec(&v6).expect("encode IpAddr::V6 for discriminant check");
    assert_ne!(
        enc_v4[0], enc_v6[0],
        "IpAddr::V4 and IpAddr::V6 must have different discriminant bytes"
    );
    assert_eq!(enc_v4[0], 0u8, "IpAddr::V4 discriminant must be 0");
    assert_eq!(enc_v6[0], 1u8, "IpAddr::V6 discriminant must be 1");
    assert_eq!(
        enc_v4.len(),
        5,
        "IpAddr::V4 must encode as 1 discriminant + 4 octets"
    );
    assert_eq!(
        enc_v6.len(),
        17,
        "IpAddr::V6 must encode as 1 discriminant + 16 octets"
    );
}

// ===== 12. SocketAddrV4 roundtrip (addr + port) =====

#[test]
fn test_socketaddrv4_roundtrip() {
    let addr = SocketAddrV4::new(Ipv4Addr::new(192, 168, 1, 1), 8080);
    let enc = encode_to_vec(&addr).expect("encode SocketAddrV4 192.168.1.1:8080");
    let (dec, _): (SocketAddrV4, usize) =
        decode_from_slice(&enc).expect("decode SocketAddrV4 192.168.1.1:8080");
    assert_eq!(addr, dec);
    assert_eq!(dec.port(), 8080);
    assert_eq!(*dec.ip(), Ipv4Addr::new(192, 168, 1, 1));
}

// ===== 13. SocketAddrV6 roundtrip =====

#[test]
fn test_socketaddrv6_roundtrip() {
    let addr = SocketAddrV6::new(
        Ipv6Addr::new(0xfe80, 0, 0, 0, 0, 0, 0, 1),
        9000,
        0x12345678,
        42,
    );
    let enc = encode_to_vec(&addr).expect("encode SocketAddrV6");
    let (dec, _): (SocketAddrV6, usize) = decode_from_slice(&enc).expect("decode SocketAddrV6");
    assert_eq!(addr, dec);
    assert_eq!(dec.port(), 9000);
    assert_eq!(dec.flowinfo(), 0x12345678);
    assert_eq!(dec.scope_id(), 42);
}

// ===== 14. SocketAddr::V4 roundtrip =====

#[test]
fn test_socketaddr_v4_roundtrip() {
    let addr = SocketAddr::V4(SocketAddrV4::new(Ipv4Addr::new(127, 0, 0, 1), 3000));
    let enc = encode_to_vec(&addr).expect("encode SocketAddr::V4");
    let (dec, _): (SocketAddr, usize) = decode_from_slice(&enc).expect("decode SocketAddr::V4");
    assert_eq!(addr, dec);
    assert!(dec.is_ipv4(), "decoded SocketAddr must be V4");
}

// ===== 15. SocketAddr::V6 roundtrip =====

#[test]
fn test_socketaddr_v6_roundtrip() {
    let addr = SocketAddr::V6(SocketAddrV6::new(Ipv6Addr::LOCALHOST, 8443, 0, 0));
    let enc = encode_to_vec(&addr).expect("encode SocketAddr::V6");
    let (dec, _): (SocketAddr, usize) = decode_from_slice(&enc).expect("decode SocketAddr::V6");
    assert_eq!(addr, dec);
    assert!(dec.is_ipv6(), "decoded SocketAddr must be V6");
}

// ===== 16. SocketAddrV4::new(Ipv4Addr::LOCALHOST, 8080) port preserved =====

#[test]
fn test_socketaddrv4_localhost_port_8080_preserved() {
    let addr = SocketAddrV4::new(Ipv4Addr::LOCALHOST, 8080);
    let enc = encode_to_vec(&addr).expect("encode SocketAddrV4 localhost:8080");
    let (dec, _): (SocketAddrV4, usize) =
        decode_from_slice(&enc).expect("decode SocketAddrV4 localhost:8080");
    assert_eq!(addr, dec);
    assert_eq!(
        dec.port(),
        8080,
        "port 8080 must be preserved after roundtrip"
    );
    assert_eq!(*dec.ip(), Ipv4Addr::LOCALHOST);
}

// ===== 17. Vec<Ipv4Addr> roundtrip =====

#[test]
fn test_vec_ipv4addr_roundtrip() {
    let addrs: Vec<Ipv4Addr> = vec![
        Ipv4Addr::LOCALHOST,
        Ipv4Addr::BROADCAST,
        Ipv4Addr::new(192, 168, 1, 1),
        Ipv4Addr::new(10, 0, 0, 1),
        Ipv4Addr::UNSPECIFIED,
    ];
    let enc = encode_to_vec(&addrs).expect("encode Vec<Ipv4Addr>");
    let (dec, _): (Vec<Ipv4Addr>, usize) = decode_from_slice(&enc).expect("decode Vec<Ipv4Addr>");
    assert_eq!(addrs, dec);
    assert_eq!(dec.len(), 5);
}

// ===== 18. Option<IpAddr> Some roundtrip =====

#[test]
fn test_option_ipaddr_some_roundtrip() {
    let opt: Option<IpAddr> = Some(IpAddr::V4(Ipv4Addr::new(203, 0, 113, 42)));
    let enc = encode_to_vec(&opt).expect("encode Option<IpAddr> Some");
    let (dec, _): (Option<IpAddr>, usize) =
        decode_from_slice(&enc).expect("decode Option<IpAddr> Some");
    assert_eq!(opt, dec);
    assert!(dec.is_some(), "decoded Option<IpAddr> must be Some");
}

// ===== 19. Option<SocketAddr> None roundtrip =====

#[test]
fn test_option_socketaddr_none_roundtrip() {
    let opt: Option<SocketAddr> = None;
    let enc = encode_to_vec(&opt).expect("encode Option<SocketAddr> None");
    let (dec, _): (Option<SocketAddr>, usize) =
        decode_from_slice(&enc).expect("decode Option<SocketAddr> None");
    assert_eq!(opt, dec);
    assert!(dec.is_none(), "decoded Option<SocketAddr> must be None");
}

// ===== 20. Struct with Ipv4Addr and u16 port fields roundtrip =====

#[derive(Debug, PartialEq, Encode, Decode)]
struct Endpoint {
    ip: Ipv4Addr,
    port: u16,
}

#[test]
fn test_struct_with_ipv4addr_and_port_roundtrip() {
    let ep = Endpoint {
        ip: Ipv4Addr::new(10, 0, 0, 1),
        port: 4040,
    };
    let enc = encode_to_vec(&ep).expect("encode Endpoint struct");
    let (dec, _): (Endpoint, usize) = decode_from_slice(&enc).expect("decode Endpoint struct");
    assert_eq!(ep, dec);
    assert_eq!(dec.ip, Ipv4Addr::new(10, 0, 0, 1));
    assert_eq!(dec.port, 4040);
}

// ===== 21. consumed bytes == encoded length for IpAddr =====

#[test]
fn test_ipaddr_consumed_equals_encoded_length() {
    let addr = IpAddr::V4(Ipv4Addr::new(198, 51, 100, 7));
    let enc = encode_to_vec(&addr).expect("encode IpAddr for consumed check");
    let (dec, consumed): (IpAddr, usize) =
        decode_from_slice(&enc).expect("decode IpAddr for consumed check");
    assert_eq!(addr, dec);
    assert_eq!(
        consumed,
        enc.len(),
        "consumed bytes must equal encoded length for IpAddr"
    );
}

// ===== 22. Two different Ipv4Addr produce different encodings =====

#[test]
fn test_two_different_ipv4_addrs_produce_different_encodings() {
    let ip_a = Ipv4Addr::new(1, 2, 3, 4);
    let ip_b = Ipv4Addr::new(5, 6, 7, 8);
    let enc_a = encode_to_vec(&ip_a).expect("encode ip_a 1.2.3.4");
    let enc_b = encode_to_vec(&ip_b).expect("encode ip_b 5.6.7.8");
    assert_ne!(
        enc_a, enc_b,
        "1.2.3.4 and 5.6.7.8 must produce different encodings"
    );
    let (dec_a, _): (Ipv4Addr, usize) = decode_from_slice(&enc_a).expect("decode ip_a");
    let (dec_b, _): (Ipv4Addr, usize) = decode_from_slice(&enc_b).expect("decode ip_b");
    assert_eq!(ip_a, dec_a);
    assert_eq!(ip_b, dec_b);
}
