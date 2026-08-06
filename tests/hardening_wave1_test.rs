//! Wave 1 hardening regression tests.
//!
//! Every test in this file pins a defect that was reachable from the public
//! decode API with untrusted input and that produced a process-level failure
//! (stack overflow, unbounded loop, or a multi-gigabyte allocation from a
//! handful of bytes) rather than a typed `Err`.
//!
//! The crafted inputs all use the varint length encoding documented in
//! `src/varint/mod.rs`: a leading `253` marker followed by eight little-endian
//! bytes is a `u64` length prefix.

use oxicode::config;
use oxicode::error::Error;

/// Nine bytes encoding a `u64` length prefix of `u64::MAX` under the default
/// (varint) configuration.
fn forged_u64_max_length() -> Vec<u8> {
    let mut bytes = vec![253u8];
    bytes.extend_from_slice(&u64::MAX.to_le_bytes());
    bytes
}

// ---------------------------------------------------------------------------
// Unbounded pre-allocation (native decode path)
// ---------------------------------------------------------------------------

/// `String::decode` used to read a `u64` length, then `vec![0u8; len]` before
/// touching the reader. Nine bytes of input therefore requested a
/// `usize::MAX`-byte allocation, aborting the process. The length must now be
/// checked against the input actually remaining.
#[test]
fn string_forged_length_is_rejected_before_allocating() {
    let bytes = forged_u64_max_length();
    let result = oxicode::decode_from_slice_with_config::<String, _>(&bytes, config::standard());
    assert!(
        matches!(result, Err(Error::UnexpectedEnd { .. })),
        "expected UnexpectedEnd, got {:?}",
        result.map(|(value, read)| (value.len(), read))
    );
}

/// Same defect on `Vec<u8>::decode`'s bulk fast path.
#[test]
fn vec_u8_forged_length_is_rejected_before_allocating() {
    let bytes = forged_u64_max_length();
    let result = oxicode::decode_from_slice_with_config::<Vec<u8>, _>(&bytes, config::standard());
    assert!(
        matches!(result, Err(Error::UnexpectedEnd { .. })),
        "expected UnexpectedEnd, got {:?}",
        result.map(|(value, read)| (value.len(), read))
    );
}

/// A forged length must not turn into a huge element reservation either.
#[test]
fn vec_of_structs_forged_length_is_rejected() {
    let bytes = forged_u64_max_length();
    let result = oxicode::decode_from_slice_with_config::<Vec<u32>, _>(&bytes, config::standard());
    assert!(result.is_err(), "forged element count must not be trusted");
}

/// Legitimate payloads of every size must still round-trip: the bound is
/// "against the remaining input", never a fixed ceiling.
#[test]
fn large_legitimate_string_still_round_trips() {
    let original = "o".repeat(300_000);
    let encoded = oxicode::encode_to_vec_with_config(&original, config::standard())
        .expect("encode must succeed");
    let (decoded, _) =
        oxicode::decode_from_slice_with_config::<String, _>(&encoded, config::standard())
            .expect("decode must succeed");
    assert_eq!(decoded, original);
}

// ---------------------------------------------------------------------------
// Unbounded pre-allocation (streaming reader, remaining input unknown)
// ---------------------------------------------------------------------------

/// A `std::io::Read` source cannot report how much input is left, so the buffer
/// is grown incrementally and a forged length fails on the first short read
/// instead of at `alloc` time.
#[test]
fn string_forged_length_over_std_read_is_rejected() {
    let mut bytes = forged_u64_max_length();
    bytes.extend_from_slice(b"hello");
    let cursor = std::io::Cursor::new(bytes);
    let result = oxicode::decode_from_std_read::<String, _, _>(cursor, config::standard());
    assert!(
        result.is_err(),
        "forged length over std::io::Read must fail"
    );
}

/// The incremental path must not corrupt or reject payloads larger than one
/// incremental step (16 KiB).
#[test]
fn large_legitimate_string_over_std_read_still_round_trips() {
    let original = "x".repeat(100_000);
    let encoded = oxicode::encode_to_vec_with_config(&original, config::standard())
        .expect("encode must succeed");
    let cursor = std::io::Cursor::new(encoded);
    let decoded = oxicode::decode_from_std_read::<String, _, _>(cursor, config::standard())
        .expect("decode must succeed");
    assert_eq!(decoded, original);
}

// ---------------------------------------------------------------------------
// Unbounded pre-allocation (derive `#[oxicode(bytes)]`)
// ---------------------------------------------------------------------------

#[derive(oxicode::Encode, oxicode::Decode, Debug, PartialEq)]
struct BytesBlob {
    #[oxicode(bytes)]
    data: Vec<u8>,
}

/// The generated `#[oxicode(bytes)]` block used to call
/// `Vec::with_capacity(__len)` with the raw wire length.
#[test]
fn derive_bytes_forged_length_is_rejected_before_allocating() {
    let bytes = forged_u64_max_length();
    let result = oxicode::decode_from_slice_with_config::<BytesBlob, _>(&bytes, config::standard());
    assert!(result.is_err(), "forged byte length must not be trusted");
}

#[test]
fn derive_bytes_large_payload_still_round_trips() {
    let value = BytesBlob {
        data: vec![7u8; 70_000],
    };
    let encoded = oxicode::encode_to_vec_with_config(&value, config::standard())
        .expect("encode must succeed");
    let (decoded, _) =
        oxicode::decode_from_slice_with_config::<BytesBlob, _>(&encoded, config::standard())
            .expect("decode must succeed");
    assert_eq!(decoded, value);
}

// ---------------------------------------------------------------------------
// Serde bridge
// ---------------------------------------------------------------------------

#[cfg(feature = "serde")]
mod serde_bridge {
    use super::*;
    use serde::{Deserialize, Serialize};

    /// A recursive type: one wire byte per nesting level.
    #[derive(Debug, Serialize, Deserialize)]
    enum Tree {
        Leaf,
        Node(Box<Tree>),
    }

    /// Encode `depth` nested `Tree::Node`s terminated by a `Tree::Leaf`.
    ///
    /// Both variant indices are small enough to be single varint bytes, so the
    /// payload is `depth` copies of `1` followed by a single `0`.
    fn nested_tree_bytes(depth: usize) -> Vec<u8> {
        let mut bytes = vec![1u8; depth];
        bytes.push(0u8);
        bytes
    }

    /// The serde deserializer recursed through visitor callbacks without ever
    /// entering the decoder's depth guard, so a deeply nested payload overflowed
    /// the stack and aborted the process - uncatchable, and exactly the DoS the
    /// native path was hardened against.
    #[test]
    fn owned_deserializer_bounds_recursion_depth() {
        let bytes = nested_tree_bytes(50_000);
        let result = oxicode::serde::decode_owned_from_slice::<Tree, _>(&bytes, config::standard());
        assert!(
            matches!(result, Err(Error::LimitExceeded { .. })),
            "expected LimitExceeded from the depth guard, got {:?}",
            result.map(|_| "decoded")
        );
    }

    /// The borrowed (zero-copy) deserializer had the same defect.
    #[test]
    fn borrowed_deserializer_bounds_recursion_depth() {
        let bytes = nested_tree_bytes(50_000);
        let result = oxicode::serde::decode_from_slice::<Tree, _>(&bytes, config::standard());
        assert!(
            matches!(result, Err(Error::LimitExceeded { .. })),
            "expected LimitExceeded from the depth guard, got {:?}",
            result.map(|_| "decoded")
        );
    }

    /// Nesting well inside the 128-deep default limit must still decode.
    #[test]
    fn moderate_nesting_still_decodes() {
        let bytes = nested_tree_bytes(20);
        let (value, _) =
            oxicode::serde::decode_owned_from_slice::<Tree, _>(&bytes, config::standard())
                .expect("20 levels is far inside the default 128-deep recursion limit");
        // Walk the whole chain to prove nothing was truncated.
        let mut node = &value;
        let mut levels = 0usize;
        while let Tree::Node(inner) = node {
            node = inner;
            levels += 1;
        }
        assert_eq!(levels, 20);
        assert!(matches!(node, Tree::Leaf));
    }

    /// `deserialize_seq` truncated the wire length with `as usize` and then used
    /// `usize::MAX` as an in-band "unknown length" sentinel. A wire length of
    /// exactly `u64::MAX` collided with the sentinel: the element counter never
    /// decremented and `next_element_seed` returned `Some(..)` forever, an
    /// infinite decode loop over zero-byte elements from nine bytes of input.
    ///
    /// The length is now a real count and the container is claimed up front, so
    /// a limit-configured decoder rejects it immediately instead of spinning.
    #[test]
    fn forged_seq_length_of_zero_sized_elements_terminates() {
        let bytes = forged_u64_max_length();
        let result = oxicode::serde::decode_owned_from_slice::<Vec<()>, _>(
            &bytes,
            config::standard().with_limit::<4096>(),
        );
        assert!(
            matches!(result, Err(Error::LimitExceeded { .. })),
            "expected LimitExceeded, got {:?}",
            result.map(|(value, _)| value.len())
        );
    }

    /// The same defect on the borrowed deserializer.
    #[test]
    fn forged_seq_length_of_zero_sized_elements_terminates_borrowed() {
        let bytes = forged_u64_max_length();
        let result = oxicode::serde::decode_from_slice::<Vec<()>, _>(
            &bytes,
            config::standard().with_limit::<4096>(),
        );
        assert!(
            matches!(result, Err(Error::LimitExceeded { .. })),
            "expected LimitExceeded, got {:?}",
            result.map(|(value, _)| value.len())
        );
    }

    /// `MapAccess::next_key_seed` decremented with no sentinel check at all, so
    /// a `u64::MAX` map length gave 2^64 iterations. The up-front container
    /// claim now rejects it.
    #[test]
    fn forged_map_length_is_rejected_up_front() {
        use std::collections::BTreeMap;
        let bytes = forged_u64_max_length();
        let result = oxicode::serde::decode_owned_from_slice::<BTreeMap<u8, ()>, _>(
            &bytes,
            config::standard().with_limit::<4096>(),
        );
        assert!(
            matches!(result, Err(Error::LimitExceeded { .. })),
            "expected LimitExceeded, got {:?}",
            result.map(|(value, _)| value.len())
        );
    }

    /// Under the default (unlimited) configuration a forged length over a
    /// non-degenerate element type must still fail promptly, on the first
    /// element that runs out of input.
    #[test]
    fn forged_seq_length_fails_fast_without_a_limit() {
        let bytes = forged_u64_max_length();
        let result =
            oxicode::serde::decode_owned_from_slice::<Vec<u32>, _>(&bytes, config::standard());
        assert!(result.is_err(), "forged element count must not be trusted");
    }

    /// Sequences and maps that fit inside the configured limit must be
    /// unaffected by the new up-front claim.
    #[test]
    fn limited_config_still_accepts_legitimate_containers() {
        use std::collections::BTreeMap;

        let seq: Vec<u32> = (0..200).collect();
        let encoded = oxicode::serde::encode_to_vec(&seq, config::standard().with_limit::<4096>())
            .expect("encode must succeed");
        let (decoded, _) = oxicode::serde::decode_owned_from_slice::<Vec<u32>, _>(
            &encoded,
            config::standard().with_limit::<4096>(),
        )
        .expect("a 200-element sequence fits well inside a 4 KiB limit");
        assert_eq!(decoded, seq);

        let map: BTreeMap<u16, u16> = (0..100u16).map(|k| (k, k * 2)).collect();
        let encoded = oxicode::serde::encode_to_vec(&map, config::standard().with_limit::<4096>())
            .expect("encode must succeed");
        let (decoded, _) = oxicode::serde::decode_owned_from_slice::<BTreeMap<u16, u16>, _>(
            &encoded,
            config::standard().with_limit::<4096>(),
        )
        .expect("a 100-entry map fits well inside a 4 KiB limit");
        assert_eq!(decoded, map);
    }

    /// `is_human_readable` was never overridden, so serde defaulted it to
    /// `true` and every impl that branches on it took the opposite branch from
    /// `bincode::serde`: `127.0.0.1` went on the wire as a length-prefixed
    /// ASCII string instead of a one-byte tag plus four octets. The two
    /// libraries must now agree byte for byte.
    #[test]
    fn ip_addr_serde_bytes_match_bincode() {
        use std::net::{IpAddr, Ipv4Addr, Ipv6Addr};

        let v4 = IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1));
        let ours = oxicode::serde::encode_to_vec(&v4, config::standard()).expect("encode");
        let theirs =
            bincode::serde::encode_to_vec(v4, bincode::config::standard()).expect("bincode encode");
        assert_eq!(ours, theirs, "IPv4 serde bytes must match bincode");
        assert_eq!(ours.len(), 5, "one variant tag byte plus four octets");

        let v6 = IpAddr::V6(Ipv6Addr::LOCALHOST);
        let ours = oxicode::serde::encode_to_vec(&v6, config::standard()).expect("encode");
        let theirs =
            bincode::serde::encode_to_vec(v6, bincode::config::standard()).expect("bincode encode");
        assert_eq!(ours, theirs, "IPv6 serde bytes must match bincode");
    }

    /// The compact form must also decode back to the original value, and the
    /// deserializer must agree with the serializer about human-readability.
    #[test]
    fn ip_addr_serde_round_trips_in_compact_form() {
        use std::net::{IpAddr, Ipv4Addr};

        let addr = IpAddr::V4(Ipv4Addr::new(192, 168, 0, 42));
        let encoded = oxicode::serde::encode_to_vec(&addr, config::standard()).expect("encode");
        let (decoded, _) =
            oxicode::serde::decode_owned_from_slice::<IpAddr, _>(&encoded, config::standard())
                .expect("decode");
        assert_eq!(decoded, addr);
    }

    /// A struct still decodes through the visitor-driven (length-less) path
    /// after the `usize::MAX` sentinel was replaced by `Option<usize>`.
    #[test]
    fn structs_still_decode_without_a_wire_length() {
        #[derive(Debug, PartialEq, Serialize, Deserialize)]
        struct Config {
            name: String,
            retries: u8,
            ratio: f64,
            tags: Vec<String>,
        }

        let value = Config {
            name: "primary".into(),
            retries: 3,
            ratio: 0.25,
            tags: vec!["a".into(), "b".into()],
        };
        let encoded = oxicode::serde::encode_to_vec(&value, config::standard()).expect("encode");
        let (decoded, _) =
            oxicode::serde::decode_owned_from_slice::<Config, _>(&encoded, config::standard())
                .expect("decode");
        assert_eq!(decoded, value);
    }
}
