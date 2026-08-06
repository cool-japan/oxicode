//! Cross-library parity for the serde types whose `Serialize`/`Deserialize`
//! impls branch on `Serializer::is_human_readable()` / `Deserializer::
//! is_human_readable()` — the standard-library IP and socket-address types.
//!
//! serde defaults `is_human_readable` to `true`; a non-self-describing binary
//! format must override it to `false`. When oxicode's serde bridge left the
//! method at its `true` default, every one of these types silently took the
//! *opposite* branch from `bincode::serde` — e.g. `127.0.0.1` went on the wire
//! as a varint-prefixed ASCII string instead of a one-byte variant tag plus
//! four raw octets — a size regression and a silent cross-library
//! incompatibility that the existing serde corpus (struct/enum/map/Option/i128)
//! could not catch because none of those types consult `is_human_readable`.
//!
//! These tests are the regression net for that override: they assert byte-for-
//! byte equality with `bincode::serde` and full cross-decoding in both
//! directions. If the override is ever dropped again, every case here fails.

use std::net::{IpAddr, Ipv4Addr, Ipv6Addr, SocketAddr, SocketAddrV4, SocketAddrV6};

/// Assert that `value` encodes byte-identically through both serde bridges and
/// that each library can decode the other's output back to `value`.
fn assert_serde_parity<T>(value: &T)
where
    T: serde::Serialize + serde::de::DeserializeOwned + PartialEq + std::fmt::Debug,
{
    let oxi_bytes = oxicode::serde::encode_to_vec(value, oxicode::config::standard())
        .expect("oxicode serde encode failed");
    let bin_bytes = bincode::serde::encode_to_vec(value, bincode::config::standard())
        .expect("bincode serde encode failed");

    assert_eq!(
        oxi_bytes, bin_bytes,
        "serde-bridge encoding of {value:?} must be byte-identical to bincode \
         (is_human_readable must be false, i.e. the compact binary branch)"
    );

    let (oxi_from_bin, _): (T, usize) =
        oxicode::serde::decode_from_slice(&bin_bytes, oxicode::config::standard())
            .expect("oxicode serde decode of bincode bytes failed");
    assert_eq!(
        *value, oxi_from_bin,
        "oxicode failed to decode bincode bytes"
    );

    let (bin_from_oxi, _): (T, usize) =
        bincode::serde::decode_from_slice(&oxi_bytes, bincode::config::standard())
            .expect("bincode serde decode of oxicode bytes failed");
    assert_eq!(
        *value, bin_from_oxi,
        "bincode failed to decode oxicode bytes"
    );
}

#[test]
fn serde_ipv4addr_binary_branch_parity() {
    for addr in [
        Ipv4Addr::new(127, 0, 0, 1),
        Ipv4Addr::UNSPECIFIED,
        Ipv4Addr::BROADCAST,
        Ipv4Addr::new(192, 168, 1, 254),
    ] {
        assert_serde_parity(&addr);
        // The compact binary form is exactly four raw octets — never the
        // 9+ byte ASCII string a human-readable serializer would emit.
        let bytes =
            oxicode::serde::encode_to_vec(&addr, oxicode::config::standard()).expect("encode ipv4");
        assert_eq!(
            bytes,
            addr.octets().to_vec(),
            "Ipv4Addr must be 4 raw octets"
        );
    }
}

#[test]
fn serde_ipv6addr_binary_branch_parity() {
    for addr in [
        Ipv6Addr::LOCALHOST,
        Ipv6Addr::UNSPECIFIED,
        Ipv6Addr::new(0x2001, 0xdb8, 0, 0, 0, 0, 0, 1),
    ] {
        assert_serde_parity(&addr);
        let bytes =
            oxicode::serde::encode_to_vec(&addr, oxicode::config::standard()).expect("encode ipv6");
        assert_eq!(
            bytes,
            addr.octets().to_vec(),
            "Ipv6Addr must be 16 raw octets"
        );
    }
}

#[test]
fn serde_ipaddr_enum_branch_parity() {
    for addr in [
        IpAddr::V4(Ipv4Addr::new(10, 0, 0, 42)),
        IpAddr::V6(Ipv6Addr::LOCALHOST),
    ] {
        assert_serde_parity(&addr);
    }
}

#[test]
fn serde_socketaddr_parity() {
    let v4 = SocketAddr::V4(SocketAddrV4::new(Ipv4Addr::new(203, 0, 113, 5), 8080));
    let v6 = SocketAddr::V6(SocketAddrV6::new(Ipv6Addr::LOCALHOST, 443, 0, 0));
    assert_serde_parity(&v4);
    assert_serde_parity(&v6);
}

#[test]
fn serde_vec_of_ipaddr_parity() {
    // A collection of human-readable-sensitive elements, to catch any drift
    // that only appears through the seq path.
    let addrs: Vec<IpAddr> = vec![
        IpAddr::V4(Ipv4Addr::new(1, 1, 1, 1)),
        IpAddr::V6(Ipv6Addr::new(0xfe80, 0, 0, 0, 0, 0, 0, 1)),
        IpAddr::V4(Ipv4Addr::LOCALHOST),
    ];
    assert_serde_parity(&addrs);
}

#[test]
fn serde_option_ipaddr_parity() {
    let some: Option<IpAddr> = Some(IpAddr::V4(Ipv4Addr::new(8, 8, 8, 8)));
    let none: Option<IpAddr> = None;
    assert_serde_parity(&some);
    assert_serde_parity(&none);
}
