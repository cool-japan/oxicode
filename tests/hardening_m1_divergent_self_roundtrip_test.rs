//! Self-roundtrip stability + documented-divergence spec for the types that
//! are KNOWN to diverge from bincode 2.0.1's wire format.
//!
//! Wire-format parity work for these types is DEFERRED by user decision (see
//! repo scope guard: bincode wire-format parity is on hold as of 2026-07-17).
//! Rather than leave the divergence completely untested, this file gives it
//! an executable spec in two layers:
//!
//! 1. Self-roundtrip stability tests: lock oxicode's CURRENT encoding of each
//!    divergent type to a committed byte literal (generated once from
//!    oxicode itself) and assert encode-then-decode round-trips. These guard
//!    against *accidental* format drift in oxicode's own encoder/decoder,
//!    independent of bincode.
//! 2. `#[ignore]`d cross-tests against the live `bincode` crate (already a
//!    dev-dependency of the root package) that fail on purpose today and
//!    document exactly why, so a future engineer picking this work back up
//!    has a precise, runnable description of the gap instead of prose alone.
//!
//! Divergent types covered: `SystemTime`, `SocketAddrV6`, `IpAddr` /
//! `SocketAddr` / `Bound` under fixed-int configs, `PathBuf`, `Ordering`, and
//! `Duration`'s decode leniency.

#![cfg(feature = "std")]
use std::cmp::Ordering;
use std::net::{IpAddr, Ipv4Addr, Ipv6Addr, SocketAddr, SocketAddrV4, SocketAddrV6};
use std::ops::Bound;
use std::path::PathBuf;
use std::time::{Duration, SystemTime};

// ===========================================================================
// 1. Self-roundtrip stability (oxicode -> oxicode), locking the CURRENT format
// ===========================================================================

#[test]
fn systemtime_post_epoch_self_roundtrip_stable() {
    // The nanosecond component must stay a multiple of 100: windows'
    // SystemTime is FILETIME-backed (100 ns ticks) and would truncate a finer
    // value before the encoder ever sees it, making the committed byte
    // literal platform-dependent for no good reason.
    let value = SystemTime::UNIX_EPOCH + Duration::new(1_700_000_000, 123_456_700);
    let bytes = oxicode::encode_to_vec(&value).expect("encode failed");
    assert_eq!(
        bytes,
        &[0xfc, 0x00, 0xe2, 0xa7, 0xca, 0xfc, 0xbc, 0xcc, 0x5b, 0x07],
        "SystemTime post-epoch wire format changed unexpectedly"
    );
    let (decoded, _): (SystemTime, _) = oxicode::decode_from_slice(&bytes).expect("decode failed");
    assert_eq!(value, decoded);
}

#[test]
fn systemtime_pre_epoch_self_roundtrip_stable() {
    let value = SystemTime::UNIX_EPOCH - Duration::new(1_000, 500_000_000);
    let bytes = oxicode::encode_to_vec(&value).expect("encode failed");
    assert_eq!(
        bytes,
        &[0xfb, 0xd1, 0x07, 0xfc, 0x00, 0x65, 0xcd, 0x1d],
        "SystemTime pre-epoch wire format changed unexpectedly"
    );
    let (decoded, _): (SystemTime, _) = oxicode::decode_from_slice(&bytes).expect("decode failed");
    assert_eq!(value, decoded);
}

#[test]
fn systemtime_epoch_self_roundtrip_stable() {
    let value = SystemTime::UNIX_EPOCH;
    let bytes = oxicode::encode_to_vec(&value).expect("encode failed");
    assert_eq!(
        bytes,
        &[0x00, 0x00],
        "SystemTime epoch wire format changed unexpectedly"
    );
    let (decoded, _): (SystemTime, _) = oxicode::decode_from_slice(&bytes).expect("decode failed");
    assert_eq!(value, decoded);
}

#[test]
fn socket_addr_v6_nonzero_flowinfo_scope_id_self_roundtrip_stable() {
    let value = SocketAddrV6::new(Ipv6Addr::new(0x2001, 0xdb8, 0, 0, 0, 0, 0, 1), 8080, 42, 7);
    let bytes = oxicode::encode_to_vec(&value).expect("encode failed");
    assert_eq!(
        bytes,
        &[
            0x20, 0x01, 0x0d, 0xb8, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
            0x00, 0x01, 0xfb, 0x90, 0x1f, 0x2a, 0x07
        ],
        "SocketAddrV6 wire format changed unexpectedly"
    );
    let (decoded, _): (SocketAddrV6, _) =
        oxicode::decode_from_slice(&bytes).expect("decode failed");
    assert_eq!(value, decoded);
    assert_eq!(decoded.flowinfo(), 42);
    assert_eq!(decoded.scope_id(), 7);
}

#[test]
fn ip_addr_v4_standard_and_fixed_int_self_roundtrip_stable() {
    let value = IpAddr::V4(Ipv4Addr::new(192, 168, 1, 1));

    let standard_bytes = oxicode::encode_to_vec(&value).expect("encode failed");
    assert_eq!(standard_bytes, &[0x00, 0xc0, 0xa8, 0x01, 0x01]);

    let fixed_config = oxicode::config::standard().with_fixed_int_encoding();
    let fixed_bytes =
        oxicode::encode_to_vec_with_config(&value, fixed_config).expect("encode failed");
    assert_eq!(fixed_bytes, &[0x00, 0xc0, 0xa8, 0x01, 0x01]);

    let (decoded, _): (IpAddr, _) =
        oxicode::decode_from_slice_with_config(&fixed_bytes, fixed_config).expect("decode failed");
    assert_eq!(value, decoded);
}

#[test]
fn socket_addr_v4_fixed_int_self_roundtrip_stable() {
    let value = SocketAddr::V4(SocketAddrV4::new(Ipv4Addr::new(10, 0, 0, 1), 443));

    let standard_bytes = oxicode::encode_to_vec(&value).expect("encode failed");
    assert_eq!(
        standard_bytes,
        &[0x00, 0x0a, 0x00, 0x00, 0x01, 0xfb, 0xbb, 0x01]
    );

    let fixed_config = oxicode::config::standard().with_fixed_int_encoding();
    let fixed_bytes =
        oxicode::encode_to_vec_with_config(&value, fixed_config).expect("encode failed");
    assert_eq!(fixed_bytes, &[0x00, 0x0a, 0x00, 0x00, 0x01, 0xbb, 0x01]);

    let (decoded, _): (SocketAddr, _) =
        oxicode::decode_from_slice_with_config(&fixed_bytes, fixed_config).expect("decode failed");
    assert_eq!(value, decoded);
}

#[test]
fn bound_included_fixed_int_self_roundtrip_stable() {
    let value: Bound<u32> = Bound::Included(7);

    let standard_bytes = oxicode::encode_to_vec(&value).expect("encode failed");
    assert_eq!(standard_bytes, &[0x01, 0x07]);

    let fixed_config = oxicode::config::standard().with_fixed_int_encoding();
    let fixed_bytes =
        oxicode::encode_to_vec_with_config(&value, fixed_config).expect("encode failed");
    assert_eq!(fixed_bytes, &[0x01, 0x07, 0x00, 0x00, 0x00]);

    let (decoded, _): (Bound<u32>, _) =
        oxicode::decode_from_slice_with_config(&fixed_bytes, fixed_config).expect("decode failed");
    assert_eq!(value, decoded);
}

#[test]
fn pathbuf_self_roundtrip_stable() {
    let value = PathBuf::from("/tmp/example/path.txt");
    let bytes = oxicode::encode_to_vec(&value).expect("encode failed");
    assert_eq!(
        bytes,
        &[
            0x15, 0x2f, 0x74, 0x6d, 0x70, 0x2f, 0x65, 0x78, 0x61, 0x6d, 0x70, 0x6c, 0x65, 0x2f,
            0x70, 0x61, 0x74, 0x68, 0x2e, 0x74, 0x78, 0x74
        ],
        "PathBuf wire format changed unexpectedly"
    );
    let (decoded, _): (PathBuf, _) = oxicode::decode_from_slice(&bytes).expect("decode failed");
    assert_eq!(value, decoded);
}

#[test]
fn ordering_self_roundtrip_stable() {
    let cases = [
        (Ordering::Less, [0xffu8].as_slice()),
        (Ordering::Equal, [0x00].as_slice()),
        (Ordering::Greater, [0x01].as_slice()),
    ];
    for (value, expected) in cases {
        let bytes = oxicode::encode_to_vec(&value).expect("encode failed");
        assert_eq!(bytes, expected, "Ordering::{value:?} wire format changed");
        let (decoded, _): (Ordering, _) =
            oxicode::decode_from_slice(&bytes).expect("decode failed");
        assert_eq!(value, decoded);
    }
}

/// Duration's decode is intentionally STRICTER than bincode's: bincode
/// accepts `subsec_nanos >= 1_000_000_000` (as long as it doesn't overflow
/// `secs`) and normalizes it via `Duration::new`, while oxicode rejects it
/// outright as `InvalidData`. This is allowed under the repo scope guard
/// (rejecting invalid input is always permitted) and is a deliberate
/// hardening choice, not a bug — this test documents and locks that
/// behavior rather than "fixing" it into leniency.
#[test]
fn duration_rejects_overflowing_subsec_nanos() {
    // secs=5 (varint) followed by nanos=1_000_000_000 (u32 varint): a
    // technically-invalid-but-decodable-by-bincode Duration payload.
    let raw: &[u8] = &[0x05, 0xfc, 0x00, 0xca, 0x9a, 0x3b];
    let result: oxicode::Result<(Duration, usize)> = oxicode::decode_from_slice(raw);
    assert!(
        result.is_err(),
        "oxicode should reject Duration payloads with subsec_nanos >= 1_000_000_000"
    );
}

// ===========================================================================
// 2. `#[ignore]`d cross-tests against the live `bincode` crate, documenting
//    exactly why they fail today. Un-ignoring any of these (without also
//    changing the corresponding `src/` impl) is expected to fail.
// ===========================================================================

#[test]
#[ignore = "known divergence: oxicode encodes SystemTime as signed-i64-zigzag \
            seconds + u32 nanos relative to UNIX_EPOCH (allowing pre-epoch \
            times); bincode 2.0.1 encodes the underlying Duration (unsigned \
            varint secs + u32 nanos) and REJECTS pre-epoch SystemTimes at \
            encode time (duration_since fails). Byte layout differs even for \
            post-epoch times because a zigzag-varint i64 and a plain-varint \
            u64 diverge above the single-byte boundary. See src/enc/impls.rs \
            and src/de/impls.rs 'SystemTime' sections. Deferred by scope guard."]
fn cross_lib_systemtime_diverges_from_bincode() {
    let value = SystemTime::UNIX_EPOCH + Duration::new(1_700_000_000, 123_456_789);
    let oxi_bytes = oxicode::encode_to_vec(&value).expect("oxicode encode failed");
    let bin_bytes =
        bincode::encode_to_vec(value, bincode::config::standard()).expect("bincode encode failed");
    assert_eq!(
        oxi_bytes, bin_bytes,
        "expected to diverge (see #[ignore] reason)"
    );
}

#[test]
#[ignore = "known divergence: oxicode's SocketAddrV6 encodes ip + port + \
            flowinfo (u32) + scope_id (u32); bincode 2.0.1 encodes only ip + \
            port and always DECODES flowinfo/scope_id as 0, silently \
            discarding them on the encode side. Byte length differs by 8 \
            bytes regardless of value. See src/features/impl_std.rs \
            'SocketAddrV6' section. Deferred by scope guard."]
fn cross_lib_socket_addr_v6_diverges_from_bincode() {
    let value = SocketAddrV6::new(Ipv6Addr::new(0x2001, 0xdb8, 0, 0, 0, 0, 0, 1), 8080, 42, 7);
    let oxi_bytes = oxicode::encode_to_vec(&value).expect("oxicode encode failed");
    let bin_bytes =
        bincode::encode_to_vec(value, bincode::config::standard()).expect("bincode encode failed");
    assert_eq!(
        oxi_bytes, bin_bytes,
        "expected to diverge (see #[ignore] reason)"
    );
}

#[test]
#[ignore = "known divergence, FIXED-INT CONFIG ONLY: oxicode tags IpAddr's \
            V4/V6 variant with a raw u8 (1 byte in every config); bincode \
            2.0.1 tags it with a u32 run through the configured int \
            encoding, so under with_fixed_int_encoding() bincode spends 4 \
            bytes on the tag versus oxicode's 1. (Under the default varint \
            config the two happen to match, since a varint-encoded 0 or 1 is \
            also a single byte -- see the passing, non-ignored coverage for \
            that case.) See src/features/impl_std.rs 'IpAddr' section. \
            Deferred by scope guard."]
fn cross_lib_ip_addr_diverges_from_bincode_under_fixed_int() {
    let value = IpAddr::V4(Ipv4Addr::new(192, 168, 1, 1));
    let fixed_config = oxicode::config::standard().with_fixed_int_encoding();
    let oxi_bytes =
        oxicode::encode_to_vec_with_config(&value, fixed_config).expect("oxicode encode failed");
    let bin_bytes =
        bincode::encode_to_vec(value, bincode::config::standard().with_fixed_int_encoding())
            .expect("bincode encode failed");
    assert_eq!(
        oxi_bytes, bin_bytes,
        "expected to diverge (see #[ignore] reason)"
    );
}

#[test]
#[ignore = "known divergence, FIXED-INT CONFIG ONLY: same u8-vs-u32 variant \
            tag divergence as IpAddr (see above), inherited by SocketAddr's \
            V4/V6 tag. See src/features/impl_std.rs 'SocketAddr' section. \
            Deferred by scope guard."]
fn cross_lib_socket_addr_diverges_from_bincode_under_fixed_int() {
    let value = SocketAddr::V4(SocketAddrV4::new(Ipv4Addr::new(10, 0, 0, 1), 443));
    let fixed_config = oxicode::config::standard().with_fixed_int_encoding();
    let oxi_bytes =
        oxicode::encode_to_vec_with_config(&value, fixed_config).expect("oxicode encode failed");
    let bin_bytes =
        bincode::encode_to_vec(value, bincode::config::standard().with_fixed_int_encoding())
            .expect("bincode encode failed");
    assert_eq!(
        oxi_bytes, bin_bytes,
        "expected to diverge (see #[ignore] reason)"
    );
}

#[test]
#[ignore = "known divergence, FIXED-INT CONFIG ONLY: same u8-vs-u32 variant \
            tag divergence as IpAddr/SocketAddr, but for core::ops::Bound's \
            Unbounded/Included/Excluded tag. See src/enc/impls.rs and \
            src/de/impls.rs 'Bound' sections. Deferred by scope guard."]
fn cross_lib_bound_diverges_from_bincode_under_fixed_int() {
    let value: Bound<u32> = Bound::Included(7);
    let fixed_config = oxicode::config::standard().with_fixed_int_encoding();
    let oxi_bytes =
        oxicode::encode_to_vec_with_config(&value, fixed_config).expect("oxicode encode failed");
    let bin_bytes =
        bincode::encode_to_vec(value, bincode::config::standard().with_fixed_int_encoding())
            .expect("bincode encode failed");
    assert_eq!(
        oxi_bytes, bin_bytes,
        "expected to diverge (see #[ignore] reason)"
    );
}

#[test]
#[ignore = "known divergence: bincode 2.0.1's `Encode for &Path` requires the \
            path to be valid UTF-8 (`Path::to_str()`) and returns \
            EncodeError::InvalidPathCharacters for anything else. oxicode's \
            Path/PathBuf instead encode the platform's raw OsStr bytes \
            (unix) or UTF-16 code units (windows) unconditionally, so it \
            succeeds even for non-UTF-8 paths where bincode cannot encode at \
            all. (For paths that ARE valid UTF-8 on a unix host, the two \
            actually agree byte-for-byte today -- including under \
            with_fixed_int_encoding(), since both end up using a u64 length \
            prefix followed by the same UTF-8 bytes; see the non-ignored \
            pathbuf_self_roundtrip_stable test above for that passing case. \
            This test isolates the part that is unconditionally divergent: \
            non-UTF-8 paths.) See src/features/impl_std.rs 'Path & PathBuf' \
            section. Deferred by scope guard."]
#[cfg(unix)]
fn cross_lib_pathbuf_diverges_from_bincode_on_invalid_utf8() {
    use std::ffi::OsStr;
    use std::os::unix::ffi::OsStrExt;

    // 0xFF is never a valid standalone (or leading) UTF-8 byte.
    let invalid_utf8_bytes: &[u8] = &[0x2f, 0x74, 0x6d, 0x70, 0x2f, 0xff, 0xfe];
    let value = PathBuf::from(OsStr::from_bytes(invalid_utf8_bytes));

    let oxi_bytes = oxicode::encode_to_vec(&value);
    assert!(
        oxi_bytes.is_ok(),
        "oxicode is expected to encode non-UTF-8 paths successfully"
    );

    let bin_bytes = bincode::encode_to_vec(value.as_path(), bincode::config::standard());
    assert!(
        bin_bytes.is_ok(),
        "expected bincode to also succeed here (see #[ignore] reason: it \
         currently returns EncodeError::InvalidPathCharacters instead)"
    );
}

#[test]
#[ignore = "known non-parity: bincode 2.0.1 ships NO Encode/Decode impl for \
            core::cmp::Ordering at all (grepped the bincode-2.0.1 sources: \
            zero matches outside core::sync::atomic::Ordering, which is an \
            unrelated type). oxicode's Ordering support is therefore a pure \
            EXTENSION beyond bincode's coverage, not a byte-format mismatch \
            -- there is nothing to make byte-identical because bincode has \
            no encoding to compare against. This test intentionally does not \
            compile against a real bincode call (there is none to make); it \
            exists purely to keep this fact documented and discoverable next \
            to the other divergence tests. Deferred by scope guard (nothing \
            to defer, but tracked for completeness)."]
fn cross_lib_ordering_has_no_bincode_counterpart() {
    // Deliberately fails when un-ignored: there is no `bincode::Encode` for
    // `core::cmp::Ordering`, so this assertion stands in for "not
    // applicable" rather than a real byte comparison.
    panic!(
        "bincode 2.0.1 has no Encode/Decode impl for core::cmp::Ordering; \
         there is no cross-library byte comparison to make"
    );
}

#[test]
#[ignore = "known leniency divergence: bincode 2.0.1's Duration::decode \
            ACCEPTS subsec_nanos >= 1_000_000_000 (as long as secs + \
            nanos/1e9 doesn't overflow u64) and normalizes via \
            Duration::new, which itself carries the nanosecond overflow into \
            extra seconds. oxicode's Duration::decode REJECTS any \
            subsec_nanos >= 1_000_000_000 outright as invalid data. This is \
            an intentional hardening choice (rejecting malformed input is \
            explicitly permitted by the repo scope guard) rather than a bug, \
            so it is not going to be relaxed to match bincode's leniency; \
            this test documents the gap. See \
            duration_rejects_overflowing_subsec_nanos above for the \
            (non-ignored) assertion of oxicode's actual behavior."]
fn cross_lib_duration_leniency_diverges_from_bincode() {
    // secs=5, nanos=1_000_000_000 (invalid: nanos must be < 1_000_000_000).
    let raw: &[u8] = &[0x05, 0xfc, 0x00, 0xca, 0x9a, 0x3b];
    let oxi_result: oxicode::Result<(Duration, usize)> = oxicode::decode_from_slice(raw);
    let bin_result: Result<(Duration, usize), _> =
        bincode::decode_from_slice(raw, bincode::config::standard());
    assert!(oxi_result.is_err(), "oxicode should reject this payload");
    assert!(
        bin_result.is_ok(),
        "expected bincode to leniently accept this payload (see #[ignore] reason)"
    );
}
