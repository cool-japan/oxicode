//! Hardening tests for WP-B1: `src/features/impl_std.rs`.
//!
//! Covers:
//! - `HashMap`/`HashSet` decode claiming container memory (via
//!   `claim_container_read`) before allocating, so a huge attacker-controlled
//!   length is rejected under a configured byte limit instead of triggering an
//!   oversized allocation.
//! - `CString`/`PathBuf` (unix) decode length reads using a checked
//!   `usize::try_from` conversion and claiming memory before allocating.
//! - `OsStr`/`OsString` `Encode` erroring on non-UTF-8 content instead of
//!   silently lossy-converting it, while leaving valid-UTF-8 byte layout
//!   unchanged.

#![cfg(feature = "std")]
#![allow(clippy::useless_vec)]

use oxicode::{config, decode_from_slice_with_config, encode_to_vec_with_config};
use std::collections::{HashMap, HashSet};

// ---------------------------------------------------------------------------
// HashMap: claim_container_read must reject huge lengths under a byte limit
// ---------------------------------------------------------------------------

#[test]
fn test_hashmap_decode_huge_len_rejected_under_limit() {
    // Craft a payload that only contains a huge u64 length prefix (no element
    // bytes follow). Without claim_container_read this would attempt to
    // reserve capacity for billions of (K, V) pairs before ever reading an
    // element and consulting the limit.
    let huge_len: u64 = u64::MAX / 2;
    let payload =
        encode_to_vec_with_config(&huge_len, config::standard()).expect("encode huge len");

    let cfg = config::standard().with_limit::<64>();
    let result: Result<(HashMap<u32, u32>, usize), _> =
        decode_from_slice_with_config(&payload, cfg);

    assert!(
        result.is_err(),
        "decoding a HashMap with an enormous declared length must fail fast under a small limit, \
         not attempt to allocate memory for it"
    );
}

#[test]
fn test_hashset_decode_huge_len_rejected_under_limit() {
    let huge_len: u64 = u64::MAX / 2;
    let payload =
        encode_to_vec_with_config(&huge_len, config::standard()).expect("encode huge len");

    let cfg = config::standard().with_limit::<64>();
    let result: Result<(HashSet<u32>, usize), _> = decode_from_slice_with_config(&payload, cfg);

    assert!(
        result.is_err(),
        "decoding a HashSet with an enormous declared length must fail fast under a small limit, \
         not attempt to allocate memory for it"
    );
}

// ---------------------------------------------------------------------------
// HashMap/HashSet: normal roundtrip still works (claim doesn't break valid use)
// ---------------------------------------------------------------------------

#[test]
fn test_hashmap_roundtrip_unaffected_by_claim() {
    let mut map: HashMap<u32, String> = HashMap::new();
    map.insert(1, "one".to_string());
    map.insert(2, "two".to_string());
    map.insert(3, "three".to_string());

    let enc = encode_to_vec_with_config(&map, config::standard()).expect("encode hashmap");
    let (decoded, consumed): (HashMap<u32, String>, usize) =
        decode_from_slice_with_config(&enc, config::standard()).expect("decode hashmap");

    assert_eq!(decoded, map);
    assert_eq!(consumed, enc.len());
}

#[test]
fn test_hashset_roundtrip_unaffected_by_claim() {
    let mut set: HashSet<i64> = HashSet::new();
    set.insert(-5);
    set.insert(0);
    set.insert(42);

    let enc = encode_to_vec_with_config(&set, config::standard()).expect("encode hashset");
    let (decoded, consumed): (HashSet<i64>, usize) =
        decode_from_slice_with_config(&enc, config::standard()).expect("decode hashset");

    assert_eq!(decoded, set);
    assert_eq!(consumed, enc.len());
}

// ---------------------------------------------------------------------------
// CString: huge declared length must be rejected under a small limit, not
// preallocated (checked usize conversion + claim_bytes_read already existed,
// verify it still behaves correctly after the checked-conversion change).
// ---------------------------------------------------------------------------

#[test]
fn test_cstring_decode_huge_len_rejected_under_limit() {
    use std::ffi::CString;

    let huge_len: u64 = u64::MAX / 2;
    let payload =
        encode_to_vec_with_config(&huge_len, config::standard()).expect("encode huge len");

    let cfg = config::standard().with_limit::<64>();
    let result: Result<(CString, usize), _> = decode_from_slice_with_config(&payload, cfg);

    assert!(
        result.is_err(),
        "decoding a CString with an enormous declared length must fail fast under a small limit"
    );
}

#[test]
fn test_cstring_roundtrip() {
    use std::ffi::CString;

    let value = CString::new("hello, oxicode").unwrap();
    let enc = encode_to_vec_with_config(&value, config::standard()).expect("encode cstring");
    let (decoded, consumed): (CString, usize) =
        decode_from_slice_with_config(&enc, config::standard()).expect("decode cstring");

    assert_eq!(decoded, value);
    assert_eq!(consumed, enc.len());
}

// ---------------------------------------------------------------------------
// PathBuf (unix): huge declared length must be rejected under a small limit.
// ---------------------------------------------------------------------------

#[cfg(unix)]
#[test]
fn test_pathbuf_decode_huge_len_rejected_under_limit() {
    use std::path::PathBuf;

    let huge_len: u64 = u64::MAX / 2;
    let payload =
        encode_to_vec_with_config(&huge_len, config::standard()).expect("encode huge len");

    let cfg = config::standard().with_limit::<64>();
    let result: Result<(PathBuf, usize), _> = decode_from_slice_with_config(&payload, cfg);

    assert!(
        result.is_err(),
        "decoding a PathBuf with an enormous declared length must fail fast under a small limit"
    );
}

#[cfg(unix)]
#[test]
fn test_pathbuf_roundtrip_unix() {
    use std::path::PathBuf;

    let value = PathBuf::from("/tmp/some/oxicode/test/path.bin");
    let enc = encode_to_vec_with_config(&value, config::standard()).expect("encode pathbuf");
    let (decoded, consumed): (PathBuf, usize) =
        decode_from_slice_with_config(&enc, config::standard()).expect("decode pathbuf");

    assert_eq!(decoded, value);
    assert_eq!(consumed, enc.len());
}

// ---------------------------------------------------------------------------
// OsStr/OsString: valid UTF-8 roundtrips with byte layout identical to a
// plain `String` encode (scope guard: no wire-format change for valid input).
// ---------------------------------------------------------------------------

#[cfg(not(target_family = "wasm"))]
#[test]
fn test_osstring_roundtrip_valid_utf8() {
    use std::ffi::OsString;

    let value = OsString::from("hello world, oxicode");
    let enc = encode_to_vec_with_config(&value, config::standard()).expect("encode osstring");
    let (decoded, consumed): (OsString, usize) =
        decode_from_slice_with_config(&enc, config::standard()).expect("decode osstring");

    assert_eq!(decoded, value);
    assert_eq!(consumed, enc.len());

    // Byte layout for valid UTF-8 content must be identical to encoding the
    // equivalent String directly (scope guard: no wire-format change for
    // valid input).
    let as_string = value.to_str().unwrap().to_string();
    let string_enc =
        encode_to_vec_with_config(&as_string, config::standard()).expect("encode string");
    assert_eq!(
        enc, string_enc,
        "OsString encode must be byte-identical to the equivalent String encode for valid UTF-8"
    );
}

#[cfg(unix)]
#[test]
fn test_osstr_encode_errors_on_non_utf8_instead_of_lossy_converting() {
    use oxicode::enc::{EncoderImpl, VecWriter};
    use oxicode::Encode;
    use std::ffi::OsStr;
    use std::os::unix::ffi::OsStrExt;

    // 0x9f is not valid UTF-8 on its own (a lone continuation-style byte in
    // this position is invalid), so this OsStr is not valid UTF-8.
    let invalid_bytes: &[u8] = &[0x66, 0x6f, 0x6f, 0xff, 0x9f, 0x62, 0x61, 0x72];
    let os_str = OsStr::from_bytes(invalid_bytes);
    assert!(
        os_str.to_str().is_none(),
        "test fixture must actually be invalid UTF-8"
    );

    // Directly exercise Encode::encode to ensure it returns an error rather
    // than silently substituting U+FFFD replacement characters.
    let writer = VecWriter::new();
    let mut encoder = EncoderImpl::new(writer, config::standard());
    let result = os_str.encode(&mut encoder);

    assert!(
        result.is_err(),
        "encoding a non-UTF-8 OsStr must return an error, not silently lossy-convert the data"
    );
}
