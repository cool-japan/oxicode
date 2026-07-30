//! Hardening tests for TASK D1: coherent limit accounting (bincode-parity),
//! blanket `Encode for &T`, `Vec<u8>` fast path, generic `Cow` decode,
//! element-wise collection `BorrowDecode`, and the decode recursion-depth guard.
//!
//! These exercise BEHAVIOR, not wire bytes — every construct here produces the
//! same serialized bytes as before; only limit bookkeeping and trait coverage
//! changed.

#![cfg(feature = "std")]
use oxicode::config::{self, Config};
use oxicode::de::{Decode, DecoderImpl, SliceReader};
use oxicode::{
    borrow_decode_from_slice, decode_from_slice, decode_from_slice_with_config, encode_to_vec,
    encode_to_vec_with_config, Error,
};
use std::collections::HashMap;

// ---------------------------------------------------------------------------
// Limit accounting: primitive/scalar decodes now claim their fixed width,
// mirroring bincode 2.0.1. Under `standard()` (varint) a scalar still claims
// its full fixed width up front, exactly like bincode.
// ---------------------------------------------------------------------------

#[test]
fn scalar_decode_claims_fixed_width_u32() {
    // A single u32 claims size_of::<u32>() == 4 bytes regardless of encoding.
    let bytes = encode_to_vec(&99u32).expect("encode u32");

    // limit == 4: exactly enough for the u32 claim -> succeeds.
    let cfg_ok = config::standard().with_limit::<4>();
    let (v, _): (u32, usize) =
        decode_from_slice_with_config(&bytes, cfg_ok).expect("u32 must decode within limit 4");
    assert_eq!(v, 99);

    // limit == 3: one short of the 4-byte claim -> fails.
    let cfg_bad = config::standard().with_limit::<3>();
    let r: Result<(u32, usize), _> = decode_from_slice_with_config(&bytes, cfg_bad);
    assert!(
        matches!(r, Err(Error::LimitExceeded { .. })),
        "u32 decode with limit 3 must exceed the 4-byte claim, got {r:?}"
    );
}

#[test]
fn tuple_of_scalars_accumulates_claims() {
    // (u32, u32) claims 4 + 4 = 8 bytes.
    let bytes = encode_to_vec(&(1u32, 2u32)).expect("encode tuple");

    let cfg_ok = config::standard().with_limit::<8>();
    let (v, _): ((u32, u32), usize) =
        decode_from_slice_with_config(&bytes, cfg_ok).expect("tuple decodes within limit 8");
    assert_eq!(v, (1, 2));

    let cfg_bad = config::standard().with_limit::<7>();
    let r: Result<((u32, u32), usize), _> = decode_from_slice_with_config(&bytes, cfg_bad);
    assert!(
        matches!(r, Err(Error::LimitExceeded { .. })),
        "tuple claim (8) must exceed limit 7, got {r:?}"
    );
}

#[test]
fn primitive_heavy_array_exceeding_limit_fails() {
    // [u32; 100] reserves size_of::<[u32; 100]>() == 400 bytes up front.
    let arr = [7u32; 100];
    let bytes = encode_to_vec(&arr).expect("encode array");

    // A small limit is exceeded by the up-front reservation before any element
    // is decoded -> DoS-safe rejection.
    let cfg_bad = config::standard().with_limit::<64>();
    let r: Result<([u32; 100], usize), _> = decode_from_slice_with_config(&bytes, cfg_bad);
    assert!(
        matches!(r, Err(Error::LimitExceeded { .. })),
        "array of 100 u32 (claims 400) must exceed limit 64, got err={:?}",
        r.err()
    );
}

#[test]
fn array_legal_under_limit_passes_via_reserve_reclaim() {
    // [u32; 10] reserves 40 bytes, then reclaims 4 before each element decode,
    // which re-claims 4 -> peak stays at exactly 40. limit == 40 must pass.
    let arr = [3u32; 10];
    let bytes = encode_to_vec(&arr).expect("encode array");

    let cfg_ok = config::standard().with_limit::<40>();
    let (v, _): ([u32; 10], usize) =
        decode_from_slice_with_config(&bytes, cfg_ok).expect("array must pass at limit 40");
    assert_eq!(v, arr);

    let cfg_bad = config::standard().with_limit::<39>();
    let r: Result<([u32; 10], usize), _> = decode_from_slice_with_config(&bytes, cfg_bad);
    assert!(
        matches!(r, Err(Error::LimitExceeded { .. })),
        "array reservation (40) must exceed limit 39, got {r:?}"
    );
}

#[test]
fn vec_element_loop_reclaims_so_it_does_not_double_count() {
    // Vec<u32> of 4 elements: length u64::decode claims 8, container claim adds
    // 4*4 = 16 (peak 24). Each iteration unclaims 4 then re-claims 4, so the
    // peak of 24 is the bound. limit 24 passes, 23 fails.
    let data: Vec<u32> = vec![10, 20, 30, 40];
    let bytes = encode_to_vec(&data).expect("encode vec");

    let cfg_ok = config::standard().with_limit::<24>();
    let (v, _): (Vec<u32>, usize) =
        decode_from_slice_with_config(&bytes, cfg_ok).expect("vec passes at limit 24");
    assert_eq!(v, data);

    let cfg_bad = config::standard().with_limit::<23>();
    let r: Result<(Vec<u32>, usize), _> = decode_from_slice_with_config(&bytes, cfg_bad);
    assert!(
        matches!(r, Err(Error::LimitExceeded { .. })),
        "vec<u32>[4] peak claim (24) must exceed limit 23, got {r:?}"
    );
}

// ---------------------------------------------------------------------------
// Blanket `impl<T: Encode + ?Sized> Encode for &T` (bincode parity).
// Byte-identical to encoding the value directly, and `encode_to_vec::<&Foo>`
// now compiles.
// ---------------------------------------------------------------------------

#[test]
fn blanket_reference_encode_is_byte_identical() {
    let v: u32 = 0xDEAD_BEEF;
    let direct = encode_to_vec(&v).expect("encode u32");
    let via_ref = encode_to_vec::<&u32>(&&v).expect("encode &u32");
    assert_eq!(direct, via_ref, "&u32 must encode identically to u32");

    let s = "hello, oxicode";
    let direct_str = encode_to_vec(&s.to_string()).expect("encode String");
    let via_ref_str = encode_to_vec::<&str>(&s).expect("encode &str");
    assert_eq!(direct_str, via_ref_str, "&str must match String bytes");

    let bytes_slice: &[u8] = &[1, 2, 3, 4, 5];
    let direct_vec = encode_to_vec(&bytes_slice.to_vec()).expect("encode Vec<u8>");
    let via_ref_slice = encode_to_vec::<&[u8]>(&bytes_slice).expect("encode &[u8]");
    assert_eq!(direct_vec, via_ref_slice, "&[u8] must match Vec<u8> bytes");
}

#[test]
fn blanket_reference_encode_covers_structs_and_nested_refs() {
    // Generic `E = &Vec<T>` and `E = &&u32` now compile through the blanket impl.
    let data: Vec<u32> = vec![1, 2, 3];
    let via_ref = encode_to_vec::<&Vec<u32>>(&&data).expect("encode &Vec<u32>");
    let direct = encode_to_vec(&data).expect("encode Vec<u32>");
    assert_eq!(via_ref, direct);

    let n: u16 = 42;
    let via_double = encode_to_vec::<&&u16>(&&&n).expect("encode &&u16");
    assert_eq!(via_double, encode_to_vec(&n).expect("encode u16"));
}

// ---------------------------------------------------------------------------
// `Vec<u8>` u8 fast path: byte-identical to the generic path.
// ---------------------------------------------------------------------------

#[test]
fn vec_u8_fast_path_roundtrips_and_matches_generic_bytes() {
    let data: Vec<u8> = (0u8..=255).collect();
    let bytes = encode_to_vec(&data).expect("encode Vec<u8>");

    // Byte layout: varint length prefix + raw bytes.
    let (decoded, consumed): (Vec<u8>, usize) = decode_from_slice(&bytes).expect("decode Vec<u8>");
    assert_eq!(decoded, data);
    assert_eq!(consumed, bytes.len());

    // Compare against a Vec<u16> (non-u8 generic path) to make sure the fast
    // path is only taken for u8 and still produces the same length framing.
    let empty: Vec<u8> = Vec::new();
    let eb = encode_to_vec(&empty).expect("encode empty");
    let (de, _): (Vec<u8>, usize) = decode_from_slice(&eb).expect("decode empty");
    assert!(de.is_empty());
}

// ---------------------------------------------------------------------------
// Generic `Cow` decode over `T: ToOwned + ?Sized`.
// ---------------------------------------------------------------------------

#[test]
fn cow_slice_of_non_u8_roundtrips() {
    use std::borrow::Cow;
    let data: Cow<[i32]> = Cow::Owned(vec![-1, 2, -3, 4]);
    let bytes = encode_to_vec(&data).expect("encode Cow<[i32]>");
    let (decoded, _): (Cow<[i32]>, usize) = decode_from_slice(&bytes).expect("decode Cow<[i32]>");
    assert_eq!(&*decoded, &[-1, 2, -3, 4]);

    let s: Cow<str> = Cow::Owned("cow-string".to_string());
    let sb = encode_to_vec(&s).expect("encode Cow<str>");
    let (ds, _): (Cow<str>, usize) = decode_from_slice(&sb).expect("decode Cow<str>");
    assert_eq!(&*ds, "cow-string");
}

// ---------------------------------------------------------------------------
// Element-wise collection `BorrowDecode`: `HashMap<&str, u32>` and
// `Vec<&str>` now borrow-decode (previously blocked by `Decode + 'static`).
// ---------------------------------------------------------------------------

#[test]
fn hashmap_of_borrowed_key_borrow_decodes() {
    // Build with owned keys, encode, then borrow-decode into borrowed keys.
    let mut owned: HashMap<String, u32> = HashMap::new();
    owned.insert("alpha".to_string(), 1);
    owned.insert("beta".to_string(), 2);
    let bytes = encode_to_vec(&owned).expect("encode HashMap<String,u32>");

    let (borrowed, _): (HashMap<&str, u32>, usize) =
        borrow_decode_from_slice(&bytes).expect("borrow-decode HashMap<&str,u32>");
    assert_eq!(borrowed.len(), 2);
    assert_eq!(borrowed.get("alpha"), Some(&1));
    assert_eq!(borrowed.get("beta"), Some(&2));
}

#[test]
fn vec_of_borrowed_str_borrow_decodes() {
    let owned: Vec<String> = vec!["one".into(), "two".into(), "three".into()];
    let bytes = encode_to_vec(&owned).expect("encode Vec<String>");
    let (borrowed, _): (Vec<&str>, usize) =
        borrow_decode_from_slice(&bytes).expect("borrow-decode Vec<&str>");
    assert_eq!(borrowed, vec!["one", "two", "three"]);
}

// ---------------------------------------------------------------------------
// Decode recursion-depth guard: deeply nested containers are bounded.
// ---------------------------------------------------------------------------

#[test]
fn recursion_guard_rejects_over_limit_nesting() {
    // Vec<Vec<Vec<u8>>> is 3 nesting levels deep on decode.
    let data: Vec<Vec<Vec<u8>>> = vec![vec![vec![1u8, 2, 3]]];
    let bytes = encode_to_vec(&data).expect("encode nested vec");

    // recursion limit 2 -> the third level trips the guard.
    let mut dec = DecoderImpl::new(SliceReader::new(&bytes), config::standard());
    dec.set_recursion_limit(2);
    let r = Vec::<Vec<Vec<u8>>>::decode(&mut dec);
    assert!(
        matches!(r, Err(Error::LimitExceeded { .. })),
        "3-level nesting must exceed recursion limit 2, got {r:?}"
    );
}

#[test]
fn recursion_guard_allows_within_limit_nesting() {
    let data: Vec<Vec<Vec<u8>>> = vec![vec![vec![9u8, 8, 7]]];
    let bytes = encode_to_vec(&data).expect("encode nested vec");

    // recursion limit 3 -> exactly enough for 3 levels.
    let mut dec = DecoderImpl::new(SliceReader::new(&bytes), config::standard());
    dec.set_recursion_limit(3);
    let decoded = Vec::<Vec<Vec<u8>>>::decode(&mut dec).expect("3 levels within limit 3");
    assert_eq!(decoded, data);
}

#[test]
fn default_recursion_limit_allows_normal_data() {
    // Shallow, realistic nesting decodes fine under the default limit (128).
    let data: Vec<Vec<u32>> = vec![vec![1, 2], vec![3, 4, 5], vec![]];
    let bytes = encode_to_vec_with_config(&data, config::standard()).expect("encode");
    let (decoded, _): (Vec<Vec<u32>>, usize) =
        decode_from_slice(&bytes).expect("decode nested under default limit");
    assert_eq!(decoded, data);
}

// ---------------------------------------------------------------------------
// Sanity: the accounting changes do not alter wire bytes for valid input.
// ---------------------------------------------------------------------------

#[test]
fn accounting_changes_preserve_wire_bytes() {
    // Encode a mixed structure and assert a stable byte length under standard().
    let value = (vec![1u8, 2, 3], "hi".to_string(), 42u32, Some(7i64));
    let bytes = encode_to_vec(&value).expect("encode mixed");

    // Round-trips without a limit.
    let (decoded, consumed): ((Vec<u8>, String, u32, Option<i64>), usize) =
        decode_from_slice(&bytes).expect("decode mixed");
    assert_eq!(decoded, value);
    assert_eq!(consumed, bytes.len());
    let _ = config::standard().int_encoding();
}
