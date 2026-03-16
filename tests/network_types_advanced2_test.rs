//! Advanced network address type serialization tests for OxiCode (set 2).
//! Covers Ipv4Addr, Ipv6Addr, IpAddr, SocketAddr variants, Vec collections,
//! Option wrapping, consumed-bytes invariants, and fixed-int config roundtrips.

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

// ---------------------------------------------------------------------------
// 1. Ipv4Addr::LOCALHOST
// ---------------------------------------------------------------------------

#[test]
fn test_ipv4_loopback_roundtrip() {
    let ip = Ipv4Addr::LOCALHOST;
    let enc = encode_to_vec(&ip).expect("encode Ipv4Addr::LOCALHOST");
    let (dec, _): (Ipv4Addr, usize) = decode_from_slice(&enc).expect("decode Ipv4Addr::LOCALHOST");
    assert_eq!(ip, dec);
}

// ---------------------------------------------------------------------------
// 2. Ipv4Addr::BROADCAST
// ---------------------------------------------------------------------------

#[test]
fn test_ipv4_broadcast_roundtrip() {
    let ip = Ipv4Addr::BROADCAST;
    let enc = encode_to_vec(&ip).expect("encode Ipv4Addr::BROADCAST");
    let (dec, _): (Ipv4Addr, usize) = decode_from_slice(&enc).expect("decode Ipv4Addr::BROADCAST");
    assert_eq!(ip, dec);
}

// ---------------------------------------------------------------------------
// 3. Ipv4Addr::UNSPECIFIED
// ---------------------------------------------------------------------------

#[test]
fn test_ipv4_unspecified_roundtrip() {
    let ip = Ipv4Addr::UNSPECIFIED;
    let enc = encode_to_vec(&ip).expect("encode Ipv4Addr::UNSPECIFIED");
    let (dec, _): (Ipv4Addr, usize) =
        decode_from_slice(&enc).expect("decode Ipv4Addr::UNSPECIFIED");
    assert_eq!(ip, dec);
}

// ---------------------------------------------------------------------------
// 4. Ipv4Addr::new(192, 168, 1, 1)
// ---------------------------------------------------------------------------

#[test]
fn test_ipv4_custom_roundtrip() {
    let ip = Ipv4Addr::new(192, 168, 1, 1);
    let enc = encode_to_vec(&ip).expect("encode Ipv4Addr 192.168.1.1");
    let (dec, _): (Ipv4Addr, usize) = decode_from_slice(&enc).expect("decode Ipv4Addr 192.168.1.1");
    assert_eq!(ip, dec);
}

// ---------------------------------------------------------------------------
// 5. Ipv6Addr::LOCALHOST
// ---------------------------------------------------------------------------

#[test]
fn test_ipv6_loopback_roundtrip() {
    let ip = Ipv6Addr::LOCALHOST;
    let enc = encode_to_vec(&ip).expect("encode Ipv6Addr::LOCALHOST");
    let (dec, _): (Ipv6Addr, usize) = decode_from_slice(&enc).expect("decode Ipv6Addr::LOCALHOST");
    assert_eq!(ip, dec);
}

// ---------------------------------------------------------------------------
// 6. Ipv6Addr::UNSPECIFIED
// ---------------------------------------------------------------------------

#[test]
fn test_ipv6_unspecified_roundtrip() {
    let ip = Ipv6Addr::UNSPECIFIED;
    let enc = encode_to_vec(&ip).expect("encode Ipv6Addr::UNSPECIFIED");
    let (dec, _): (Ipv6Addr, usize) =
        decode_from_slice(&enc).expect("decode Ipv6Addr::UNSPECIFIED");
    assert_eq!(ip, dec);
}

// ---------------------------------------------------------------------------
// 7. Ipv6Addr custom (2001:db8::1)
// ---------------------------------------------------------------------------

#[test]
fn test_ipv6_custom_roundtrip() {
    let ip = Ipv6Addr::new(0x2001, 0xdb8, 0, 0, 0, 0, 0, 1);
    let enc = encode_to_vec(&ip).expect("encode Ipv6Addr 2001:db8::1");
    let (dec, _): (Ipv6Addr, usize) = decode_from_slice(&enc).expect("decode Ipv6Addr 2001:db8::1");
    assert_eq!(ip, dec);
}

// ---------------------------------------------------------------------------
// 8. IpAddr::V4(Ipv4Addr::LOCALHOST)
// ---------------------------------------------------------------------------

#[test]
fn test_ipaddr_v4_roundtrip() {
    let ip = IpAddr::V4(Ipv4Addr::LOCALHOST);
    let enc = encode_to_vec(&ip).expect("encode IpAddr::V4(LOCALHOST)");
    let (dec, _): (IpAddr, usize) = decode_from_slice(&enc).expect("decode IpAddr::V4(LOCALHOST)");
    assert_eq!(ip, dec);
}

// ---------------------------------------------------------------------------
// 9. IpAddr::V6(Ipv6Addr::LOCALHOST)
// ---------------------------------------------------------------------------

#[test]
fn test_ipaddr_v6_roundtrip() {
    let ip = IpAddr::V6(Ipv6Addr::LOCALHOST);
    let enc = encode_to_vec(&ip).expect("encode IpAddr::V6(LOCALHOST)");
    let (dec, _): (IpAddr, usize) = decode_from_slice(&enc).expect("decode IpAddr::V6(LOCALHOST)");
    assert_eq!(ip, dec);
}

// ---------------------------------------------------------------------------
// 10. IpAddr V4 and V6 encode to different bytes
// ---------------------------------------------------------------------------

#[test]
fn test_ipaddr_v4_vs_v6_different_bytes() {
    let v4 = IpAddr::V4(Ipv4Addr::LOCALHOST);
    let v6 = IpAddr::V6(Ipv6Addr::LOCALHOST);
    let enc_v4 = encode_to_vec(&v4).expect("encode IpAddr::V4");
    let enc_v6 = encode_to_vec(&v6).expect("encode IpAddr::V6");
    assert_ne!(
        enc_v4, enc_v6,
        "V4 and V6 IpAddr must produce distinct byte representations"
    );
}

// ---------------------------------------------------------------------------
// 11. SocketAddrV4 roundtrip
// ---------------------------------------------------------------------------

#[test]
fn test_socket_addr_v4_roundtrip() {
    let addr = SocketAddrV4::new(Ipv4Addr::LOCALHOST, 8080);
    let enc = encode_to_vec(&addr).expect("encode SocketAddrV4 127.0.0.1:8080");
    let (dec, _): (SocketAddrV4, usize) =
        decode_from_slice(&enc).expect("decode SocketAddrV4 127.0.0.1:8080");
    assert_eq!(addr, dec);
}

// ---------------------------------------------------------------------------
// 12. SocketAddrV6 roundtrip
// ---------------------------------------------------------------------------

#[test]
fn test_socket_addr_v6_roundtrip() {
    let addr = SocketAddrV6::new(Ipv6Addr::LOCALHOST, 8080, 0, 0);
    let enc = encode_to_vec(&addr).expect("encode SocketAddrV6 [::1]:8080");
    let (dec, _): (SocketAddrV6, usize) =
        decode_from_slice(&enc).expect("decode SocketAddrV6 [::1]:8080");
    assert_eq!(addr, dec);
}

// ---------------------------------------------------------------------------
// 13. SocketAddr::V4 roundtrip
// ---------------------------------------------------------------------------

#[test]
fn test_socket_addr_v4_as_socketaddr_roundtrip() {
    let addr = SocketAddr::V4(SocketAddrV4::new(Ipv4Addr::LOCALHOST, 8080));
    let enc = encode_to_vec(&addr).expect("encode SocketAddr::V4(127.0.0.1:8080)");
    let (dec, _): (SocketAddr, usize) =
        decode_from_slice(&enc).expect("decode SocketAddr::V4(127.0.0.1:8080)");
    assert_eq!(addr, dec);
}

// ---------------------------------------------------------------------------
// 14. SocketAddr::V6 roundtrip
// ---------------------------------------------------------------------------

#[test]
fn test_socket_addr_v6_as_socketaddr_roundtrip() {
    let addr = SocketAddr::V6(SocketAddrV6::new(Ipv6Addr::LOCALHOST, 8080, 0, 0));
    let enc = encode_to_vec(&addr).expect("encode SocketAddr::V6([::1]:8080)");
    let (dec, _): (SocketAddr, usize) =
        decode_from_slice(&enc).expect("decode SocketAddr::V6([::1]:8080)");
    assert_eq!(addr, dec);
}

// ---------------------------------------------------------------------------
// 15. Bytes consumed equals encoded length for Ipv4Addr
// ---------------------------------------------------------------------------

#[test]
fn test_ipv4_consumed_equals_len() {
    let ip = Ipv4Addr::new(10, 20, 30, 40);
    let enc = encode_to_vec(&ip).expect("encode Ipv4Addr for consumed check");
    let (_, consumed): (Ipv4Addr, usize) =
        decode_from_slice(&enc).expect("decode Ipv4Addr for consumed check");
    assert_eq!(
        consumed,
        enc.len(),
        "consumed bytes must equal the encoded buffer length"
    );
}

// ---------------------------------------------------------------------------
// 16. Bytes consumed equals encoded length for Ipv6Addr
// ---------------------------------------------------------------------------

#[test]
fn test_ipv6_consumed_equals_len() {
    let ip = Ipv6Addr::new(0xfe80, 0, 0, 0, 0, 0, 0, 1);
    let enc = encode_to_vec(&ip).expect("encode Ipv6Addr for consumed check");
    let (_, consumed): (Ipv6Addr, usize) =
        decode_from_slice(&enc).expect("decode Ipv6Addr for consumed check");
    assert_eq!(
        consumed,
        enc.len(),
        "consumed bytes must equal the encoded buffer length"
    );
}

// ---------------------------------------------------------------------------
// 17. Bytes consumed equals encoded length for SocketAddr
// ---------------------------------------------------------------------------

#[test]
fn test_socketaddr_consumed_equals_len() {
    let addr = SocketAddr::V4(SocketAddrV4::new(Ipv4Addr::new(172, 16, 0, 1), 443));
    let enc = encode_to_vec(&addr).expect("encode SocketAddr for consumed check");
    let (_, consumed): (SocketAddr, usize) =
        decode_from_slice(&enc).expect("decode SocketAddr for consumed check");
    assert_eq!(
        consumed,
        enc.len(),
        "consumed bytes must equal the encoded buffer length"
    );
}

// ---------------------------------------------------------------------------
// 18. Vec<Ipv4Addr> with 3 addresses
// ---------------------------------------------------------------------------

#[test]
fn test_vec_ipv4_roundtrip() {
    let addrs: Vec<Ipv4Addr> = vec![
        Ipv4Addr::LOCALHOST,
        Ipv4Addr::BROADCAST,
        Ipv4Addr::new(192, 168, 0, 1),
    ];
    let enc = encode_to_vec(&addrs).expect("encode Vec<Ipv4Addr>");
    let (dec, _): (Vec<Ipv4Addr>, usize) = decode_from_slice(&enc).expect("decode Vec<Ipv4Addr>");
    assert_eq!(addrs, dec);
}

// ---------------------------------------------------------------------------
// 19. Vec<IpAddr> with V4 and V6 mixed
// ---------------------------------------------------------------------------

#[test]
fn test_vec_ipaddr_roundtrip() {
    let addrs: Vec<IpAddr> = vec![
        IpAddr::V4(Ipv4Addr::LOCALHOST),
        IpAddr::V6(Ipv6Addr::LOCALHOST),
        IpAddr::V4(Ipv4Addr::new(8, 8, 8, 8)),
        IpAddr::V6(Ipv6Addr::new(0x2001, 0x4860, 0x4860, 0, 0, 0, 0, 0x8888)),
    ];
    let enc = encode_to_vec(&addrs).expect("encode Vec<IpAddr>");
    let (dec, _): (Vec<IpAddr>, usize) = decode_from_slice(&enc).expect("decode Vec<IpAddr>");
    assert_eq!(addrs, dec);
}

// ---------------------------------------------------------------------------
// 20. Option<IpAddr> Some roundtrip
// ---------------------------------------------------------------------------

#[test]
fn test_option_ipaddr_some_roundtrip() {
    let opt: Option<IpAddr> = Some(IpAddr::V4(Ipv4Addr::new(10, 0, 0, 1)));
    let enc = encode_to_vec(&opt).expect("encode Option<IpAddr> Some");
    let (dec, _): (Option<IpAddr>, usize) =
        decode_from_slice(&enc).expect("decode Option<IpAddr> Some");
    assert_eq!(opt, dec);
}

// ---------------------------------------------------------------------------
// 21. Option<IpAddr> None roundtrip
// ---------------------------------------------------------------------------

#[test]
fn test_option_ipaddr_none_roundtrip() {
    let opt: Option<IpAddr> = None;
    let enc = encode_to_vec(&opt).expect("encode Option<IpAddr> None");
    let (dec, _): (Option<IpAddr>, usize) =
        decode_from_slice(&enc).expect("decode Option<IpAddr> None");
    assert_eq!(opt, dec);
}

// ---------------------------------------------------------------------------
// 22. Ipv4Addr with fixed_int_encoding config roundtrip
// ---------------------------------------------------------------------------

#[test]
fn test_ipv4_fixed_int_config_roundtrip() {
    let ip = Ipv4Addr::new(192, 168, 1, 100);
    let cfg = config::standard().with_fixed_int_encoding();
    let enc = encode_to_vec_with_config(&ip, cfg).expect("encode Ipv4Addr with fixed_int config");
    let (dec, _): (Ipv4Addr, usize) =
        decode_from_slice_with_config(&enc, cfg).expect("decode Ipv4Addr with fixed_int config");
    assert_eq!(ip, dec);
}
