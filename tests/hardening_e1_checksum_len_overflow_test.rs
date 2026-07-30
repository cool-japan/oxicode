//! Adversarial tests for WP-E1: `verify_checksum` must never panic on a
//! forged / malicious `LEN` field, regardless of value. It must always
//! return `Err(..)` instead of overflowing arithmetic or slicing out of
//! bounds.
//!
//! Reference: TODO.md WP-E findings (checksum integrity & DoS). The wire
//! format itself (`[MAGIC(3)][VERSION(1)][LEN(8 LE)][CRC32(4 LE)][PAYLOAD]`)
//! is unchanged; only the validation of the untrusted LEN field is hardened.

#![cfg(feature = "checksum")]

use oxicode::checksum::{verify_checksum, wrap_with_checksum, HEADER_SIZE, MAGIC};

/// Build a forged checksummed header with an arbitrary LEN field and
/// arbitrary trailing payload bytes, bypassing `wrap_with_checksum` so we
/// can inject values that the honest encoder would never produce.
fn forge_header(len_field: u64, version: u8, trailing: &[u8]) -> Vec<u8> {
    let mut buf = Vec::with_capacity(HEADER_SIZE + trailing.len());
    buf.extend_from_slice(&MAGIC);
    buf.push(version);
    buf.extend_from_slice(&len_field.to_le_bytes());
    buf.extend_from_slice(&0u32.to_le_bytes()); // CRC32 placeholder, doesn't matter
    buf.extend_from_slice(trailing);
    buf
}

#[test]
fn forged_len_near_usize_max_does_not_panic() {
    let data = forge_header(u64::MAX, 1, b"abcd");
    let result = verify_checksum(&data);
    assert!(result.is_err(), "expected Err, got {result:?}");
}

#[test]
fn forged_len_exactly_usize_max_minus_header_does_not_panic() {
    // len_field chosen so that HEADER_SIZE + stored_len would wrap to a
    // small value on 64-bit targets (the classic "wraps to 5" repro from
    // the TODO.md finding).
    let len_field = u64::MAX - 10;
    let data = forge_header(len_field, 1, b"abcd");
    let result = verify_checksum(&data);
    assert!(result.is_err(), "expected Err, got {result:?}");
}

#[test]
fn forged_len_larger_than_buffer_returns_unexpected_end() {
    let data = forge_header(1_000_000, 1, b"short");
    let result = verify_checksum(&data);
    assert!(
        matches!(result, Err(oxicode::Error::UnexpectedEnd { .. })),
        "expected UnexpectedEnd, got {result:?}"
    );
}

#[test]
fn forged_len_wraps_when_added_to_header_size_does_not_panic() {
    // stored_len = usize::MAX - HEADER_SIZE + 1 would make
    // HEADER_SIZE + stored_len wrap to 0 under unchecked arithmetic.
    let stored_len = (usize::MAX - HEADER_SIZE + 1) as u64;
    let data = forge_header(stored_len, 1, b"xyz1");
    let result = verify_checksum(&data);
    assert!(result.is_err(), "expected Err, got {result:?}");
}

#[test]
fn truncated_buffer_shorter_than_header_returns_err() {
    // Only 5 bytes total, far short of HEADER_SIZE (16).
    let data = [0x4Fu8, 0x58, 0x48, 1, 0];
    let result = verify_checksum(&data);
    assert!(
        matches!(result, Err(oxicode::Error::UnexpectedEnd { .. })),
        "expected UnexpectedEnd, got {result:?}"
    );
}

#[test]
fn truncated_buffer_header_only_no_payload_returns_err_or_ok_for_zero_len() {
    // Exactly HEADER_SIZE bytes with LEN=0 is technically valid (empty
    // payload); verify it returns Ok with an empty slice, never panics.
    let data = forge_header(0, 1, &[]);
    let result = verify_checksum(&data);
    assert!(
        result.is_ok(),
        "expected Ok for zero-length payload, got {result:?}"
    );
    assert_eq!(result.unwrap().len(), 0);
}

#[test]
fn truncated_buffer_missing_declared_payload_bytes_returns_err() {
    // Declares LEN=100 but only supplies 4 trailing bytes.
    let data = forge_header(100, 1, b"abcd");
    let result = verify_checksum(&data);
    assert!(
        matches!(result, Err(oxicode::Error::UnexpectedEnd { .. })),
        "expected UnexpectedEnd, got {result:?}"
    );
}

#[test]
fn empty_input_returns_err_never_panics() {
    let data: [u8; 0] = [];
    let result = verify_checksum(&data);
    assert!(
        matches!(result, Err(oxicode::Error::UnexpectedEnd { .. })),
        "expected UnexpectedEnd, got {result:?}"
    );
}

#[test]
fn single_byte_input_returns_err_never_panics() {
    let data = [0x4Fu8];
    let result = verify_checksum(&data);
    assert!(
        matches!(result, Err(oxicode::Error::UnexpectedEnd { .. })),
        "expected UnexpectedEnd, got {result:?}"
    );
}

#[test]
fn header_size_boundary_minus_one_returns_err() {
    let data = vec![0u8; HEADER_SIZE - 1];
    let result = verify_checksum(&data);
    assert!(
        matches!(result, Err(oxicode::Error::UnexpectedEnd { .. })),
        "expected UnexpectedEnd, got {result:?}"
    );
}

#[test]
fn all_possible_len_field_byte_patterns_never_panic() {
    // Sweep a broad set of adversarial LEN field bit patterns to ensure
    // none of them trigger a panic, only well-formed Err results.
    let candidates: &[u64] = &[
        0,
        1,
        u64::MAX,
        u64::MAX - 1,
        u64::MAX / 2,
        (usize::MAX as u64).wrapping_add(1),
        usize::MAX as u64,
        (usize::MAX as u64) - (HEADER_SIZE as u64) + 1,
        1u64 << 63,
        1u64 << 32,
        (1u64 << 32) - 1,
    ];

    for &len_field in candidates {
        let data = forge_header(len_field, 1, b"payload_bytes_here");
        // Must never panic; result may be Ok or Err depending on whether
        // the forged LEN happens to match the trailing bytes length, but
        // since CRC is a placeholder 0, a "false positive" Ok is extremely
        // unlikely; either way, no panic is the property under test.
        let _ = verify_checksum(&data);
    }
}

#[test]
fn legitimate_roundtrip_still_works_after_hardening() {
    let payload = b"hardening should not affect valid roundtrips";
    let wrapped = wrap_with_checksum(payload);
    let result = verify_checksum(&wrapped).expect("valid input must still verify");
    assert_eq!(result, payload);
}

#[test]
fn legitimate_large_roundtrip_still_works_after_hardening() {
    let payload: Vec<u8> = (0u8..=255).cycle().take(50_000).collect();
    let wrapped = wrap_with_checksum(&payload);
    let result = verify_checksum(&wrapped).expect("valid large input must still verify");
    assert_eq!(result, payload.as_slice());
}
