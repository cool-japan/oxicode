//! Advanced comprehensive tests for network types encode/decode.
//!
//! Covers Ipv4Addr/Ipv6Addr constants, SocketAddrV4/V6, SocketAddr variants,
//! Vec collections, Option, derived structs, HashMap, exact byte inspection,
//! and encoded_size verification.

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
use oxicode::{config, decode_from_slice, encode_to_vec, encoded_size, Decode, Encode};
use std::collections::HashMap;
use std::net::{Ipv4Addr, Ipv6Addr, SocketAddr, SocketAddrV4, SocketAddrV6};

// ===== 1. Ipv4Addr::LOCALHOST (127.0.0.1) roundtrip =====

#[test]
fn test_advanced_ipv4_localhost_roundtrip() {
    let ip = Ipv4Addr::LOCALHOST;
    let enc = encode_to_vec(&ip).expect("encode Ipv4Addr::LOCALHOST");
    let (dec, consumed): (Ipv4Addr, _) =
        decode_from_slice(&enc).expect("decode Ipv4Addr::LOCALHOST");
    assert_eq!(ip, dec);
    assert_eq!(consumed, enc.len());
}

// ===== 2. Ipv4Addr::BROADCAST (255.255.255.255) roundtrip =====

#[test]
fn test_advanced_ipv4_broadcast_roundtrip() {
    let ip = Ipv4Addr::BROADCAST;
    let enc = encode_to_vec(&ip).expect("encode Ipv4Addr::BROADCAST");
    let (dec, consumed): (Ipv4Addr, _) =
        decode_from_slice(&enc).expect("decode Ipv4Addr::BROADCAST");
    assert_eq!(ip, dec);
    assert_eq!(consumed, enc.len());
    assert_eq!(ip.octets(), [255u8, 255, 255, 255]);
}

// ===== 3. Ipv4Addr::UNSPECIFIED (0.0.0.0) roundtrip =====

#[test]
fn test_advanced_ipv4_unspecified_roundtrip() {
    let ip = Ipv4Addr::UNSPECIFIED;
    let enc = encode_to_vec(&ip).expect("encode Ipv4Addr::UNSPECIFIED");
    let (dec, consumed): (Ipv4Addr, _) =
        decode_from_slice(&enc).expect("decode Ipv4Addr::UNSPECIFIED");
    assert_eq!(ip, dec);
    assert_eq!(consumed, enc.len());
    assert_eq!(dec.octets(), [0u8, 0, 0, 0]);
}

// ===== 4. Ipv4Addr::new(192, 168, 1, 1) roundtrip =====

#[test]
fn test_advanced_ipv4_private_192_168_1_1_roundtrip() {
    let ip = Ipv4Addr::new(192, 168, 1, 1);
    let enc = encode_to_vec(&ip).expect("encode 192.168.1.1");
    let (dec, consumed): (Ipv4Addr, _) = decode_from_slice(&enc).expect("decode 192.168.1.1");
    assert_eq!(ip, dec);
    assert_eq!(consumed, enc.len());
    assert_eq!(dec.octets(), [192u8, 168, 1, 1]);
}

// ===== 5. Ipv4Addr exact bytes: encodes as exactly 4 bytes =====

#[test]
fn test_advanced_ipv4_exact_byte_count() {
    let ip = Ipv4Addr::new(10, 20, 30, 40);
    let enc = encode_to_vec(&ip).expect("encode 10.20.30.40");
    assert_eq!(
        enc.len(),
        4,
        "Ipv4Addr must encode as exactly 4 raw bytes, got {}",
        enc.len()
    );
    assert_eq!(enc.as_slice(), &[10u8, 20, 30, 40]);
}

// ===== 6. Ipv6Addr::LOCALHOST roundtrip =====

#[test]
fn test_advanced_ipv6_localhost_roundtrip() {
    let ip = Ipv6Addr::LOCALHOST;
    let enc = encode_to_vec(&ip).expect("encode Ipv6Addr::LOCALHOST");
    let (dec, consumed): (Ipv6Addr, _) =
        decode_from_slice(&enc).expect("decode Ipv6Addr::LOCALHOST");
    assert_eq!(ip, dec);
    assert_eq!(consumed, enc.len());
    // ::1 must decode back to ::1
    assert_eq!(dec.segments(), [0u16, 0, 0, 0, 0, 0, 0, 1]);
}

// ===== 7. Ipv6Addr::UNSPECIFIED roundtrip =====

#[test]
fn test_advanced_ipv6_unspecified_roundtrip() {
    let ip = Ipv6Addr::UNSPECIFIED;
    let enc = encode_to_vec(&ip).expect("encode Ipv6Addr::UNSPECIFIED");
    let (dec, consumed): (Ipv6Addr, _) =
        decode_from_slice(&enc).expect("decode Ipv6Addr::UNSPECIFIED");
    assert_eq!(ip, dec);
    assert_eq!(consumed, enc.len());
    assert_eq!(dec.octets(), [0u8; 16]);
}

// ===== 8. Ipv6Addr new with specific octets roundtrip =====

#[test]
fn test_advanced_ipv6_specific_segments_roundtrip() {
    // 2001:db8:85a3::8a2e:370:7334
    let ip = Ipv6Addr::new(
        0x2001, 0x0db8, 0x85a3, 0x0000, 0x0000, 0x8a2e, 0x0370, 0x7334,
    );
    let enc = encode_to_vec(&ip).expect("encode specific Ipv6Addr");
    let (dec, consumed): (Ipv6Addr, _) = decode_from_slice(&enc).expect("decode specific Ipv6Addr");
    assert_eq!(ip, dec);
    assert_eq!(consumed, enc.len());
    assert_eq!(
        dec.segments(),
        [0x2001, 0x0db8, 0x85a3, 0x0000, 0x0000, 0x8a2e, 0x0370, 0x7334]
    );
}

// ===== 9. SocketAddrV4 roundtrip =====

#[test]
fn test_advanced_socketaddrv4_roundtrip() {
    let addr = SocketAddrV4::new(Ipv4Addr::new(172, 16, 0, 1), 443);
    let enc = encode_to_vec(&addr).expect("encode SocketAddrV4");
    let (dec, consumed): (SocketAddrV4, _) = decode_from_slice(&enc).expect("decode SocketAddrV4");
    assert_eq!(addr, dec);
    assert_eq!(consumed, enc.len());
    assert_eq!(dec.port(), 443);
    assert_eq!(dec.ip(), &Ipv4Addr::new(172, 16, 0, 1));
}

// ===== 10. SocketAddrV4 with port 80 roundtrip =====

#[test]
fn test_advanced_socketaddrv4_port_80_roundtrip() {
    let addr = SocketAddrV4::new(Ipv4Addr::new(203, 0, 113, 5), 80);
    let enc = encode_to_vec(&addr).expect("encode SocketAddrV4 port 80");
    let (dec, consumed): (SocketAddrV4, _) =
        decode_from_slice(&enc).expect("decode SocketAddrV4 port 80");
    assert_eq!(addr, dec);
    assert_eq!(consumed, enc.len());
    assert_eq!(dec.port(), 80);
}

// ===== 11. SocketAddrV4 with port 65535 roundtrip =====

#[test]
fn test_advanced_socketaddrv4_port_65535_roundtrip() {
    let addr = SocketAddrV4::new(Ipv4Addr::new(198, 51, 100, 1), 65535);
    let enc = encode_to_vec(&addr).expect("encode SocketAddrV4 port 65535");
    let (dec, consumed): (SocketAddrV4, _) =
        decode_from_slice(&enc).expect("decode SocketAddrV4 port 65535");
    assert_eq!(addr, dec);
    assert_eq!(consumed, enc.len());
    assert_eq!(dec.port(), 65535u16);
}

// ===== 12. SocketAddrV6 roundtrip =====

#[test]
fn test_advanced_socketaddrv6_roundtrip() {
    let addr = SocketAddrV6::new(
        Ipv6Addr::new(0x2001, 0x0db8, 0, 0, 0, 0, 0, 0x0001),
        8443,
        0xaabbccdd,
        99,
    );
    let enc = encode_to_vec(&addr).expect("encode SocketAddrV6");
    let (dec, consumed): (SocketAddrV6, _) = decode_from_slice(&enc).expect("decode SocketAddrV6");
    assert_eq!(addr, dec);
    assert_eq!(consumed, enc.len());
    assert_eq!(dec.port(), 8443);
    assert_eq!(dec.flowinfo(), 0xaabbccdd);
    assert_eq!(dec.scope_id(), 99);
}

// ===== 13. SocketAddr::V4 variant roundtrip =====

#[test]
fn test_advanced_socketaddr_v4_variant_roundtrip() {
    let addr = SocketAddr::V4(SocketAddrV4::new(Ipv4Addr::new(100, 64, 0, 1), 9090));
    let enc = encode_to_vec(&addr).expect("encode SocketAddr::V4");
    let (dec, consumed): (SocketAddr, _) = decode_from_slice(&enc).expect("decode SocketAddr::V4");
    assert_eq!(addr, dec);
    assert_eq!(consumed, enc.len());
    assert!(matches!(dec, SocketAddr::V4(_)));
    assert_eq!(dec.port(), 9090);
}

// ===== 14. SocketAddr::V6 variant roundtrip =====

#[test]
fn test_advanced_socketaddr_v6_variant_roundtrip() {
    let addr = SocketAddr::V6(SocketAddrV6::new(
        Ipv6Addr::new(0xfd00, 0, 0, 0, 0, 0, 0, 1),
        5353,
        0,
        0,
    ));
    let enc = encode_to_vec(&addr).expect("encode SocketAddr::V6");
    let (dec, consumed): (SocketAddr, _) = decode_from_slice(&enc).expect("decode SocketAddr::V6");
    assert_eq!(addr, dec);
    assert_eq!(consumed, enc.len());
    assert!(matches!(dec, SocketAddr::V6(_)));
    assert_eq!(dec.port(), 5353);
}

// ===== 15. Vec<Ipv4Addr> with 100 addresses roundtrip =====

#[test]
fn test_advanced_vec_ipv4_100_addresses_roundtrip() {
    let addrs: Vec<Ipv4Addr> = (0u8..100)
        .map(|i| Ipv4Addr::new(10, 0, i / 10, i % 10))
        .collect();
    assert_eq!(addrs.len(), 100);
    let enc = encode_to_vec(&addrs).expect("encode Vec<Ipv4Addr> x100");
    let (dec, consumed): (Vec<Ipv4Addr>, _) =
        decode_from_slice(&enc).expect("decode Vec<Ipv4Addr> x100");
    assert_eq!(addrs, dec);
    assert_eq!(consumed, enc.len());
    assert_eq!(dec.len(), 100);
    // Spot-check first and last
    assert_eq!(dec[0], Ipv4Addr::new(10, 0, 0, 0));
    assert_eq!(dec[99], Ipv4Addr::new(10, 0, 9, 9));
}

// ===== 16. Vec<SocketAddr> mixed v4/v6 roundtrip =====

#[test]
fn test_advanced_vec_socketaddr_mixed_roundtrip() {
    let addrs: Vec<SocketAddr> = vec![
        SocketAddr::V4(SocketAddrV4::new(Ipv4Addr::LOCALHOST, 80)),
        SocketAddr::V6(SocketAddrV6::new(Ipv6Addr::LOCALHOST, 443, 0, 0)),
        SocketAddr::V4(SocketAddrV4::new(Ipv4Addr::new(10, 0, 0, 1), 8080)),
        SocketAddr::V6(SocketAddrV6::new(
            Ipv6Addr::new(0xfe80, 0, 0, 0, 0, 0, 0, 1),
            9000,
            0,
            1,
        )),
        SocketAddr::V4(SocketAddrV4::new(Ipv4Addr::UNSPECIFIED, 0)),
    ];
    let enc = encode_to_vec(&addrs).expect("encode Vec<SocketAddr> mixed");
    let (dec, consumed): (Vec<SocketAddr>, _) =
        decode_from_slice(&enc).expect("decode Vec<SocketAddr> mixed");
    assert_eq!(addrs, dec);
    assert_eq!(consumed, enc.len());
    assert_eq!(dec.len(), 5);
    assert!(matches!(dec[0], SocketAddr::V4(_)));
    assert!(matches!(dec[1], SocketAddr::V6(_)));
}

// ===== 17. Option<SocketAddr> Some/None roundtrip =====

#[test]
fn test_advanced_option_socketaddr_some_and_none_roundtrip() {
    // None case
    let none_val: Option<SocketAddr> = None;
    let enc_none = encode_to_vec(&none_val).expect("encode None");
    let (dec_none, _): (Option<SocketAddr>, _) = decode_from_slice(&enc_none).expect("decode None");
    assert_eq!(none_val, dec_none);

    // Some(V4) case
    let some_v4: Option<SocketAddr> = Some(SocketAddr::V4(SocketAddrV4::new(
        Ipv4Addr::new(192, 168, 100, 200),
        1234,
    )));
    let enc_some = encode_to_vec(&some_v4).expect("encode Some(V4)");
    let (dec_some, _): (Option<SocketAddr>, _) =
        decode_from_slice(&enc_some).expect("decode Some(V4)");
    assert_eq!(some_v4, dec_some);

    // Some(V6) case
    let some_v6: Option<SocketAddr> = Some(SocketAddr::V6(SocketAddrV6::new(
        Ipv6Addr::new(0x2607, 0xf8b0, 0x4004, 0x0c17, 0, 0, 0, 0x200e),
        443,
        0,
        0,
    )));
    let enc_some_v6 = encode_to_vec(&some_v6).expect("encode Some(V6)");
    let (dec_some_v6, _): (Option<SocketAddr>, _) =
        decode_from_slice(&enc_some_v6).expect("decode Some(V6)");
    assert_eq!(some_v6, dec_some_v6);
}

// ===== 18. Struct with Ipv4Addr field (derive) roundtrip =====

#[derive(Debug, PartialEq, Encode, Decode)]
struct NetworkInterface {
    address: Ipv4Addr,
    netmask: Ipv4Addr,
    gateway: Ipv4Addr,
    mtu: u16,
}

#[test]
fn test_advanced_struct_with_ipv4addr_derive_roundtrip() {
    let iface = NetworkInterface {
        address: Ipv4Addr::new(192, 168, 1, 100),
        netmask: Ipv4Addr::new(255, 255, 255, 0),
        gateway: Ipv4Addr::new(192, 168, 1, 1),
        mtu: 1500,
    };
    let enc = encode_to_vec(&iface).expect("encode NetworkInterface");
    let (dec, consumed): (NetworkInterface, _) =
        decode_from_slice(&enc).expect("decode NetworkInterface");
    assert_eq!(iface, dec);
    assert_eq!(consumed, enc.len());
    assert_eq!(dec.address, Ipv4Addr::new(192, 168, 1, 100));
    assert_eq!(dec.netmask, Ipv4Addr::new(255, 255, 255, 0));
    assert_eq!(dec.gateway, Ipv4Addr::new(192, 168, 1, 1));
    assert_eq!(dec.mtu, 1500u16);
}

// ===== 19. HashMap<String, SocketAddr> roundtrip =====

#[test]
fn test_advanced_hashmap_string_socketaddr_roundtrip() {
    let mut map: HashMap<String, SocketAddr> = HashMap::new();
    map.insert(
        "web".to_string(),
        SocketAddr::V4(SocketAddrV4::new(Ipv4Addr::new(93, 184, 216, 34), 80)),
    );
    map.insert(
        "api".to_string(),
        SocketAddr::V4(SocketAddrV4::new(Ipv4Addr::new(93, 184, 216, 34), 443)),
    );
    map.insert(
        "ipv6-web".to_string(),
        SocketAddr::V6(SocketAddrV6::new(
            Ipv6Addr::new(0x2606, 0x2800, 0x220, 0x1, 0x248, 0x1893, 0x25c8, 0x1946),
            80,
            0,
            0,
        )),
    );
    map.insert(
        "local".to_string(),
        SocketAddr::V4(SocketAddrV4::new(Ipv4Addr::LOCALHOST, 8080)),
    );

    let enc = encode_to_vec(&map).expect("encode HashMap<String, SocketAddr>");
    let (dec, consumed): (HashMap<String, SocketAddr>, _) =
        decode_from_slice(&enc).expect("decode HashMap<String, SocketAddr>");
    assert_eq!(map, dec);
    assert_eq!(consumed, enc.len());
    assert_eq!(dec.len(), 4);
    assert_eq!(dec["local"].port(), 8080);
}

// ===== 20. Ipv4Addr bytes: [192, 168, 1, 1] exact =====

#[test]
fn test_advanced_ipv4_192_168_1_1_exact_bytes() {
    let ip = Ipv4Addr::new(192, 168, 1, 1);
    let enc = encode_to_vec(&ip).expect("encode 192.168.1.1 for byte check");
    // Ipv4Addr must encode as 4 raw octets in network order
    assert_eq!(
        enc.as_slice(),
        &[192u8, 168, 1, 1],
        "Ipv4Addr 192.168.1.1 must encode as bytes [192, 168, 1, 1]"
    );
}

// ===== 21. encoded_size for Ipv4Addr == 4 =====

#[test]
fn test_advanced_encoded_size_ipv4addr_is_4() {
    let ip = Ipv4Addr::new(1, 2, 3, 4);
    let size = encoded_size(&ip).expect("encoded_size Ipv4Addr");
    assert_eq!(
        size, 4,
        "encoded_size of Ipv4Addr must be exactly 4, got {}",
        size
    );

    // Verify it is the same regardless of the specific address
    let size_localhost = encoded_size(&Ipv4Addr::LOCALHOST).expect("encoded_size LOCALHOST");
    let size_broadcast = encoded_size(&Ipv4Addr::BROADCAST).expect("encoded_size BROADCAST");
    let size_unspecified = encoded_size(&Ipv4Addr::UNSPECIFIED).expect("encoded_size UNSPECIFIED");
    assert_eq!(size_localhost, 4);
    assert_eq!(size_broadcast, 4);
    assert_eq!(size_unspecified, 4);
}

// ===== 22. encoded_size for Ipv6Addr == 16 =====

#[test]
fn test_advanced_encoded_size_ipv6addr_is_16() {
    let ip = Ipv6Addr::new(0x2001, 0x0db8, 0, 0, 0, 0, 0, 1);
    let size = encoded_size(&ip).expect("encoded_size Ipv6Addr");
    assert_eq!(
        size, 16,
        "encoded_size of Ipv6Addr must be exactly 16, got {}",
        size
    );

    // Verify it is consistent across all Ipv6Addr values
    let size_localhost = encoded_size(&Ipv6Addr::LOCALHOST).expect("encoded_size LOCALHOST");
    let size_unspecified = encoded_size(&Ipv6Addr::UNSPECIFIED).expect("encoded_size UNSPECIFIED");
    let size_all_ones = encoded_size(&Ipv6Addr::new(
        0xffff, 0xffff, 0xffff, 0xffff, 0xffff, 0xffff, 0xffff, 0xffff,
    ))
    .expect("encoded_size all-ones");
    assert_eq!(size_localhost, 16);
    assert_eq!(size_unspecified, 16);
    assert_eq!(size_all_ones, 16);
}

// ===== Bonus: verify config module is accessible (used in test context) =====
// This ensures the `config` import does not emit an unused-import warning.

#[test]
fn test_advanced_config_standard_accessible() {
    let ip = Ipv4Addr::new(1, 1, 1, 1);
    let enc = oxicode::encode_to_vec_with_config(&ip, config::standard())
        .expect("encode with explicit config");
    let (dec, _): (Ipv4Addr, _) = oxicode::decode_from_slice_with_config(&enc, config::standard())
        .expect("decode with explicit config");
    assert_eq!(ip, dec);
}
