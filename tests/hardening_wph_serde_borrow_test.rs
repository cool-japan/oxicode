//! WP-H hardening tests for the serde bridge.
//!
//! Covers two findings:
//!   * `api-parity-serde#0` — borrowed/zero-copy serde decoding (`&str`, `&[u8]`,
//!     `Cow`, and `#[derive(Deserialize)]` structs with borrowed fields) must work
//!     through `decode_from_slice` and `BorrowCompat`, calling
//!     `visit_borrowed_str` / `visit_borrowed_bytes`.
//!   * `api-parity-serde#9` — the serde bridge must preserve the underlying
//!     oxicode error (e.g. `Error::UnexpectedEnd`) instead of collapsing every
//!     failure into an opaque custom string.

#![cfg(feature = "serde")]

use std::borrow::Cow;

use oxicode::config::standard;
use oxicode::error::Error;
use oxicode::serde::{decode_from_slice, decode_owned_from_slice, encode_to_vec, BorrowCompat};
use serde::{Deserialize, Serialize};

/// Assert that `slice` points inside the byte range of `owner` (i.e. it is truly
/// borrowed from the input buffer rather than a freshly allocated copy).
fn is_borrowed_from(slice: &[u8], owner: &[u8]) -> bool {
    let owner_start = owner.as_ptr() as usize;
    let owner_end = owner_start + owner.len();
    let slice_start = slice.as_ptr() as usize;
    slice_start >= owner_start && slice_start < owner_end.max(owner_start + 1)
}

#[test]
fn borrowed_str_decodes_zero_copy() {
    let bytes = encode_to_vec(&"hello world", standard()).unwrap();
    let (decoded, read): (&str, usize) = decode_from_slice(&bytes, standard()).unwrap();
    assert_eq!(decoded, "hello world");
    assert_eq!(read, bytes.len());
    // The returned &str must borrow from `bytes`, not from a temporary allocation.
    assert!(
        is_borrowed_from(decoded.as_bytes(), &bytes),
        "decoded &str should borrow from the input buffer"
    );
}

#[test]
fn borrowed_bytes_decode_zero_copy() {
    let payload: &[u8] = &[1, 2, 3, 4, 5, 6, 7, 8];
    let bytes = encode_to_vec(&payload, standard()).unwrap();
    let (decoded, _read): (&[u8], usize) = decode_from_slice(&bytes, standard()).unwrap();
    assert_eq!(decoded, payload);
    assert!(
        is_borrowed_from(decoded, &bytes),
        "decoded &[u8] should borrow from the input buffer"
    );
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
struct CowHolder<'a> {
    #[serde(borrow)]
    text: Cow<'a, str>,
}

#[test]
fn borrowed_cow_str_stays_borrowed() {
    // A top-level `Cow<str>` deserializes as `Owned` per serde's own impl; the
    // zero-copy path is reached through a `#[serde(borrow)]` field, which serde
    // lowers to `visit_borrowed_str`.
    let value = CowHolder {
        text: Cow::Borrowed("cow content"),
    };
    let bytes = encode_to_vec(&value, standard()).unwrap();
    let (decoded, _read): (CowHolder, usize) = decode_from_slice(&bytes, standard()).unwrap();
    assert_eq!(decoded.text, "cow content");
    assert!(
        matches!(decoded.text, Cow::Borrowed(_)),
        "Cow<str> struct field with #[serde(borrow)] should decode as Cow::Borrowed"
    );
    assert!(
        is_borrowed_from(decoded.text.as_bytes(), &bytes),
        "borrowed Cow<str> should point into the input buffer"
    );
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
struct Borrowed<'a> {
    name: &'a str,
    // serde auto-borrows `&[u8]` fields: serialization emits a u8 sequence and
    // deserialization routes through `deserialize_bytes` (borrowing zero-copy).
    // Both directions produce/consume the identical `u64` length + raw bytes.
    blob: &'a [u8],
    count: u32,
}

#[test]
fn derived_struct_with_borrowed_fields() {
    let original = Borrowed {
        name: "oxicode",
        blob: &[9, 8, 7, 6],
        count: 42,
    };
    let bytes = encode_to_vec(&original, standard()).unwrap();
    let (decoded, read): (Borrowed, usize) = decode_from_slice(&bytes, standard()).unwrap();
    assert_eq!(decoded, original);
    assert_eq!(read, bytes.len());
    assert!(
        is_borrowed_from(decoded.name.as_bytes(), &bytes),
        "struct &str field should borrow from the input buffer"
    );
    assert!(
        is_borrowed_from(decoded.blob, &bytes),
        "struct &[u8] field should borrow from the input buffer"
    );
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
struct OwnedStruct {
    name: String,
    values: Vec<u32>,
}

#[test]
fn owned_types_still_decode_through_borrowed_entry_point() {
    // The borrowed deserializer must not regress owned decoding: `String`/`Vec`
    // fields fall back to `visit_string` / element-wise decode.
    let original = OwnedStruct {
        name: "owned".to_string(),
        values: vec![1, 2, 3],
    };
    let bytes = encode_to_vec(&original, standard()).unwrap();
    let (decoded, _read): (OwnedStruct, usize) = decode_from_slice(&bytes, standard()).unwrap();
    assert_eq!(decoded, original);

    // And the dedicated owned entry point keeps working unchanged.
    let (decoded_owned, _): (OwnedStruct, usize) =
        decode_owned_from_slice(&bytes, standard()).unwrap();
    assert_eq!(decoded_owned, original);
}

#[test]
fn borrow_compat_native_borrow_decode() {
    // BorrowCompat routes through the borrowed serde deserializer, so a wrapped
    // borrowed type must survive a native encode/borrow_decode round-trip.
    let value = BorrowCompat("borrowed via compat");
    let bytes = oxicode::encode_to_vec(&value).unwrap();
    let (decoded, _read): (BorrowCompat<&str>, usize) =
        oxicode::borrow_decode_from_slice(&bytes).unwrap();
    assert_eq!(decoded.0, "borrowed via compat");
    assert!(
        is_borrowed_from(decoded.0.as_bytes(), &bytes),
        "BorrowCompat<&str> should borrow from the input buffer"
    );
}

#[test]
fn truncated_input_preserves_unexpected_end_error() {
    // Encode a String, then truncate the payload so the body read fails.
    let bytes = encode_to_vec(&"a reasonably long string".to_string(), standard()).unwrap();
    let truncated = &bytes[..bytes.len() - 4];

    let err = decode_owned_from_slice::<String, _>(truncated, standard())
        .expect_err("truncated input must fail to decode");

    // The concrete cause must be preserved, not degraded to OwnedCustom.
    assert!(
        matches!(err, Error::UnexpectedEnd { .. }),
        "expected Error::UnexpectedEnd, got {:?}",
        err
    );
}

#[test]
fn truncated_borrowed_str_preserves_unexpected_end_error() {
    let bytes = encode_to_vec(&"borrowed truncation", standard()).unwrap();
    let truncated = &bytes[..bytes.len() - 3];

    let err = decode_from_slice::<&str, _>(truncated, standard())
        .expect_err("truncated borrowed input must fail to decode");

    assert!(
        matches!(err, Error::UnexpectedEnd { .. }),
        "expected Error::UnexpectedEnd from borrowed path, got {:?}",
        err
    );
}
