//! Advanced network type serialization tests — second set.
//! Covers new angles for IP/socket types: loopback/broadcast/zero addresses,
//! wire sizes, 16-byte arrays, IpAddr enum variants, SocketAddr variants,
//! Vec collections, Option, fixed-int config, big-endian config, consumed bytes,
//! and encoded size comparisons.

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

// ===== 1. Ipv4Addr loopback (127.0.0.1) roundtrip =====

#[test]
fn test_adv2_ipv4_loopback_127_0_0_1_roundtrip() {
    let ip = Ipv4Addr::new(127, 0, 0, 1);
    let enc = encode_to_vec(&ip).expect("encode Ipv4Addr 127.0.0.1");
    let (dec, _): (Ipv4Addr, _) = decode_from_slice(&enc).expect("decode Ipv4Addr 127.0.0.1");
    assert_eq!(ip, dec);
    assert!(dec.is_loopback());
}

// ===== 2. Ipv4Addr broadcast (255.255.255.255) roundtrip =====

#[test]
fn test_adv2_ipv4_broadcast_255_255_255_255_roundtrip() {
    let ip = Ipv4Addr::new(255, 255, 255, 255);
    let enc = encode_to_vec(&ip).expect("encode Ipv4Addr 255.255.255.255");
    let (dec, _): (Ipv4Addr, _) = decode_from_slice(&enc).expect("decode Ipv4Addr 255.255.255.255");
    assert_eq!(ip, dec);
    assert_eq!(dec.octets(), [255u8, 255, 255, 255]);
}

// ===== 3. Ipv4Addr all zeros (0.0.0.0) roundtrip =====

#[test]
fn test_adv2_ipv4_all_zeros_0_0_0_0_roundtrip() {
    let ip = Ipv4Addr::new(0, 0, 0, 0);
    let enc = encode_to_vec(&ip).expect("encode Ipv4Addr 0.0.0.0");
    let (dec, _): (Ipv4Addr, _) = decode_from_slice(&enc).expect("decode Ipv4Addr 0.0.0.0");
    assert_eq!(ip, dec);
    assert!(dec.is_unspecified());
}

// ===== 4. Ipv4Addr::new(192, 168, 1, 1) roundtrip =====

#[test]
fn test_adv2_ipv4_192_168_1_1_roundtrip() {
    let ip = Ipv4Addr::new(192, 168, 1, 1);
    let enc = encode_to_vec(&ip).expect("encode Ipv4Addr 192.168.1.1");
    let (dec, _): (Ipv4Addr, _) = decode_from_slice(&enc).expect("decode Ipv4Addr 192.168.1.1");
    assert_eq!(ip, dec);
    assert_eq!(dec.octets(), [192, 168, 1, 1]);
}

// ===== 5. Ipv4Addr wire size — verify exact encoded byte count =====

#[test]
fn test_adv2_ipv4_wire_size_exact_4_bytes() {
    let ip = Ipv4Addr::new(10, 20, 30, 40);
    let enc = encode_to_vec(&ip).expect("encode Ipv4Addr for wire size check");
    assert_eq!(enc.len(), 4, "Ipv4Addr must encode as exactly 4 bytes");
    // also verify the bytes are the raw octets
    assert_eq!(enc[0], 10);
    assert_eq!(enc[1], 20);
    assert_eq!(enc[2], 30);
    assert_eq!(enc[3], 40);
}

// ===== 6. Ipv6Addr::LOCALHOST roundtrip =====

#[test]
fn test_adv2_ipv6_localhost_const_roundtrip() {
    let ip = Ipv6Addr::LOCALHOST;
    let enc = encode_to_vec(&ip).expect("encode Ipv6Addr::LOCALHOST");
    let (dec, _): (Ipv6Addr, _) = decode_from_slice(&enc).expect("decode Ipv6Addr::LOCALHOST");
    assert_eq!(ip, dec);
    assert!(dec.is_loopback());
}

// ===== 7. Ipv6Addr::UNSPECIFIED roundtrip =====

#[test]
fn test_adv2_ipv6_unspecified_const_roundtrip() {
    let ip = Ipv6Addr::UNSPECIFIED;
    let enc = encode_to_vec(&ip).expect("encode Ipv6Addr::UNSPECIFIED");
    let (dec, _): (Ipv6Addr, _) = decode_from_slice(&enc).expect("decode Ipv6Addr::UNSPECIFIED");
    assert_eq!(ip, dec);
    assert!(dec.is_unspecified());
    assert_eq!(enc, &[0u8; 16]);
}

// ===== 8. Ipv6Addr from 16-byte array roundtrip =====

#[test]
fn test_adv2_ipv6_from_16_byte_array_roundtrip() {
    let octets: [u8; 16] = [
        0x20, 0x01, 0x0d, 0xb8, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
        0x42,
    ];
    let ip = Ipv6Addr::from(octets);
    let enc = encode_to_vec(&ip).expect("encode Ipv6Addr from 16-byte array");
    let (dec, _): (Ipv6Addr, _) =
        decode_from_slice(&enc).expect("decode Ipv6Addr from 16-byte array");
    assert_eq!(ip, dec);
    assert_eq!(dec.octets(), octets);
    assert_eq!(enc.len(), 16);
}

// ===== 9. IpAddr::V4(Ipv4Addr::LOCALHOST) roundtrip — IpAddr is an enum =====

#[test]
fn test_adv2_ipaddr_v4_localhost_enum_roundtrip() {
    let ip = IpAddr::V4(Ipv4Addr::LOCALHOST);
    let enc = encode_to_vec(&ip).expect("encode IpAddr::V4(LOCALHOST)");
    let (dec, _): (IpAddr, _) = decode_from_slice(&enc).expect("decode IpAddr::V4(LOCALHOST)");
    assert_eq!(ip, dec);
    assert!(dec.is_ipv4());
    assert!(dec.is_loopback());
    // discriminant + 4 octets
    assert_eq!(enc.len(), 5);
}

// ===== 10. IpAddr::V6(Ipv6Addr::LOCALHOST) roundtrip =====

#[test]
fn test_adv2_ipaddr_v6_localhost_enum_roundtrip() {
    let ip = IpAddr::V6(Ipv6Addr::LOCALHOST);
    let enc = encode_to_vec(&ip).expect("encode IpAddr::V6(LOCALHOST)");
    let (dec, _): (IpAddr, _) = decode_from_slice(&enc).expect("decode IpAddr::V6(LOCALHOST)");
    assert_eq!(ip, dec);
    assert!(dec.is_ipv6());
    assert!(dec.is_loopback());
    // discriminant + 16 octets
    assert_eq!(enc.len(), 17);
}

// ===== 11. SocketAddrV4 roundtrip =====

#[test]
fn test_adv2_socketaddrv4_direct_roundtrip() {
    let addr = SocketAddrV4::new(Ipv4Addr::new(10, 0, 0, 1), 1234);
    let enc = encode_to_vec(&addr).expect("encode SocketAddrV4 10.0.0.1:1234");
    let (dec, _): (SocketAddrV4, _) =
        decode_from_slice(&enc).expect("decode SocketAddrV4 10.0.0.1:1234");
    assert_eq!(addr, dec);
    assert_eq!(dec.port(), 1234);
    assert_eq!(*dec.ip(), Ipv4Addr::new(10, 0, 0, 1));
}

// ===== 12. SocketAddrV6 roundtrip =====

#[test]
fn test_adv2_socketaddrv6_direct_roundtrip() {
    let addr = SocketAddrV6::new(
        Ipv6Addr::new(0x2001, 0x0db8, 0, 0, 0, 0, 0, 1),
        443,
        0xaabbccdd,
        5,
    );
    let enc = encode_to_vec(&addr).expect("encode SocketAddrV6 with flowinfo/scope");
    let (dec, _): (SocketAddrV6, _) =
        decode_from_slice(&enc).expect("decode SocketAddrV6 with flowinfo/scope");
    assert_eq!(addr, dec);
    assert_eq!(dec.port(), 443);
    assert_eq!(dec.flowinfo(), 0xaabbccdd);
    assert_eq!(dec.scope_id(), 5);
}

// ===== 13. SocketAddr::V4 roundtrip =====

#[test]
fn test_adv2_socketaddr_v4_variant_roundtrip() {
    let addr = SocketAddr::V4(SocketAddrV4::new(Ipv4Addr::new(172, 16, 0, 10), 8080));
    let enc = encode_to_vec(&addr).expect("encode SocketAddr::V4");
    let (dec, _): (SocketAddr, _) = decode_from_slice(&enc).expect("decode SocketAddr::V4");
    assert_eq!(addr, dec);
    assert!(dec.is_ipv4());
    assert_eq!(dec.port(), 8080);
}

// ===== 14. SocketAddr::V6 roundtrip =====

#[test]
fn test_adv2_socketaddr_v6_variant_roundtrip() {
    let addr = SocketAddr::V6(SocketAddrV6::new(Ipv6Addr::LOCALHOST, 9090, 0, 0));
    let enc = encode_to_vec(&addr).expect("encode SocketAddr::V6");
    let (dec, _): (SocketAddr, _) = decode_from_slice(&enc).expect("decode SocketAddr::V6");
    assert_eq!(addr, dec);
    assert!(dec.is_ipv6());
    assert_eq!(dec.port(), 9090);
}

// ===== 15. Vec<Ipv4Addr> roundtrip =====

#[test]
fn test_adv2_vec_ipv4addr_roundtrip() {
    let addrs: Vec<Ipv4Addr> = vec![
        Ipv4Addr::new(192, 168, 0, 1),
        Ipv4Addr::new(10, 0, 0, 1),
        Ipv4Addr::new(172, 16, 0, 1),
        Ipv4Addr::LOCALHOST,
        Ipv4Addr::BROADCAST,
    ];
    let enc = encode_to_vec(&addrs).expect("encode Vec<Ipv4Addr>");
    let (dec, consumed): (Vec<Ipv4Addr>, _) =
        decode_from_slice(&enc).expect("decode Vec<Ipv4Addr>");
    assert_eq!(addrs, dec);
    assert_eq!(consumed, enc.len());
    assert_eq!(dec.len(), 5);
}

// ===== 16. Vec<IpAddr> with mixed V4 and V6 =====

#[test]
fn test_adv2_vec_ipaddr_mixed_v4_and_v6_roundtrip() {
    let addrs: Vec<IpAddr> = vec![
        IpAddr::V4(Ipv4Addr::new(1, 1, 1, 1)),
        IpAddr::V6(Ipv6Addr::LOCALHOST),
        IpAddr::V4(Ipv4Addr::LOCALHOST),
        IpAddr::V6(Ipv6Addr::new(0xfe80, 0, 0, 0, 0, 0, 0, 1)),
        IpAddr::V4(Ipv4Addr::new(8, 8, 8, 8)),
    ];
    let enc = encode_to_vec(&addrs).expect("encode Vec<IpAddr> mixed");
    let (dec, consumed): (Vec<IpAddr>, _) =
        decode_from_slice(&enc).expect("decode Vec<IpAddr> mixed");
    assert_eq!(addrs, dec);
    assert_eq!(consumed, enc.len());
    assert!(matches!(dec[0], IpAddr::V4(_)));
    assert!(matches!(dec[1], IpAddr::V6(_)));
}

// ===== 17. Option<IpAddr> Some =====

#[test]
fn test_adv2_option_ipaddr_some_v4_roundtrip() {
    let ip: Option<IpAddr> = Some(IpAddr::V4(Ipv4Addr::new(203, 0, 113, 1)));
    let enc = encode_to_vec(&ip).expect("encode Option<IpAddr> Some");
    let (dec, consumed): (Option<IpAddr>, _) =
        decode_from_slice(&enc).expect("decode Option<IpAddr> Some");
    assert_eq!(ip, dec);
    assert_eq!(consumed, enc.len());
    assert!(dec.is_some());
}

// ===== 18. Option<IpAddr> None =====

#[test]
fn test_adv2_option_ipaddr_none_roundtrip() {
    let ip: Option<IpAddr> = None;
    let enc = encode_to_vec(&ip).expect("encode Option<IpAddr> None");
    let (dec, consumed): (Option<IpAddr>, _) =
        decode_from_slice(&enc).expect("decode Option<IpAddr> None");
    assert_eq!(ip, dec);
    assert_eq!(consumed, enc.len());
    assert!(dec.is_none());
    // None must encode as a single byte (the None discriminant)
    assert_eq!(
        enc,
        &[0x00u8],
        "Option::None must encode as single 0x00 byte"
    );
}

// ===== 19. Fixed-int config with SocketAddrV4 =====

#[test]
fn test_adv2_socketaddrv4_with_legacy_fixed_int_config() {
    let addr = SocketAddrV4::new(Ipv4Addr::new(127, 0, 0, 1), 8888);
    let enc = encode_to_vec_with_config(&addr, config::legacy())
        .expect("encode SocketAddrV4 with legacy config");
    let (dec, consumed): (SocketAddrV4, _) = decode_from_slice_with_config(&enc, config::legacy())
        .expect("decode SocketAddrV4 with legacy config");
    assert_eq!(addr, dec);
    assert_eq!(consumed, enc.len());
    assert_eq!(dec.port(), 8888);
}

// ===== 20. Big-endian config with Ipv4Addr =====

#[test]
fn test_adv2_ipv4addr_with_big_endian_config() {
    let ip = Ipv4Addr::new(192, 0, 2, 1);
    let cfg = config::standard().with_big_endian();
    let enc = encode_to_vec_with_config(&ip, cfg).expect("encode Ipv4Addr with big-endian config");
    let (dec, consumed): (Ipv4Addr, _) =
        decode_from_slice_with_config(&enc, cfg).expect("decode Ipv4Addr with big-endian config");
    assert_eq!(ip, dec);
    assert_eq!(consumed, enc.len());
    assert_eq!(dec.octets(), [192, 0, 2, 1]);
}

// ===== 21. Ipv4Addr consumed == encoded.len() =====

#[test]
fn test_adv2_ipv4addr_consumed_equals_encoded_len() {
    let ip = Ipv4Addr::new(198, 51, 100, 42);
    let enc = encode_to_vec(&ip).expect("encode Ipv4Addr for consumed check");
    let (_, consumed): (Ipv4Addr, usize) =
        decode_from_slice(&enc).expect("decode Ipv4Addr for consumed check");
    assert_eq!(
        consumed,
        enc.len(),
        "bytes consumed must equal encoded length for Ipv4Addr"
    );
    assert_eq!(consumed, 4);
}

// ===== 22. IpAddr::V4 vs IpAddr::V6 — different encoded sizes =====

#[test]
fn test_adv2_ipaddr_v4_smaller_encoded_size_than_v6() {
    let v4 = IpAddr::V4(Ipv4Addr::LOCALHOST);
    let v6 = IpAddr::V6(Ipv6Addr::LOCALHOST);

    let enc_v4 = encode_to_vec(&v4).expect("encode IpAddr::V4 for size comparison");
    let enc_v6 = encode_to_vec(&v6).expect("encode IpAddr::V6 for size comparison");

    // IpAddr::V4 = 1 discriminant byte + 4 octets = 5 bytes
    // IpAddr::V6 = 1 discriminant byte + 16 octets = 17 bytes
    assert_eq!(enc_v4.len(), 5, "IpAddr::V4 must encode as 5 bytes");
    assert_eq!(enc_v6.len(), 17, "IpAddr::V6 must encode as 17 bytes");
    assert!(
        enc_v4.len() < enc_v6.len(),
        "IpAddr::V4 ({} bytes) must encode smaller than IpAddr::V6 ({} bytes)",
        enc_v4.len(),
        enc_v6.len()
    );
}
