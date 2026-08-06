//! Hardening tests for TASK B2: char decode validation, `[T; N]` leak-safety
//! and u8 bulk fast path, `BorrowDecode` for arrays/`Result`, and the relaxed
//! `Cell<T>` decode bound.
//!
//! These tests avoid the derive macros on purpose so the file compiles and runs
//! independently of the derive crate.

#![cfg(all(feature = "alloc", feature = "simd"))]
use core::cell::Cell;
use core::sync::atomic::{AtomicUsize, Ordering};

use oxicode::de::{Decode, Decoder};
use oxicode::error::Error;

// ===== char decode: reject overlong / invalid UTF-8 =====

#[test]
fn char_decode_rejects_overlong_two_byte_c0_80() {
    // [0xC0, 0x80] is the overlong encoding of U+0000; bincode rejects it.
    let bytes = [0xC0u8, 0x80];
    let result: Result<(char, usize), _> = oxicode::decode_from_slice(&bytes);
    assert!(
        matches!(result, Err(Error::InvalidCharEncoding(_))),
        "expected InvalidCharEncoding, got {:?}",
        result
    );
}

#[test]
fn char_decode_rejects_overlong_two_byte_c1_81() {
    // [0xC1, 0x81] is the overlong encoding of 'A'.
    let bytes = [0xC1u8, 0x81];
    let result: Result<(char, usize), _> = oxicode::decode_from_slice(&bytes);
    assert!(matches!(result, Err(Error::InvalidCharEncoding(_))));
}

#[test]
fn char_decode_rejects_lone_continuation_start_byte() {
    // 0x80 is a continuation byte and can never start a sequence (width 0).
    let bytes = [0x80u8, 0x80];
    let result: Result<(char, usize), _> = oxicode::decode_from_slice(&bytes);
    assert!(matches!(result, Err(Error::InvalidCharEncoding(_))));
}

#[test]
fn char_decode_rejects_overlong_four_byte() {
    // [0xF0, 0x80, 0x80, 0x80] is an overlong encoding of U+0000.
    let bytes = [0xF0u8, 0x80, 0x80, 0x80];
    let result: Result<(char, usize), _> = oxicode::decode_from_slice(&bytes);
    assert!(matches!(result, Err(Error::InvalidCharEncoding(_))));
}

#[test]
fn char_decode_accepts_valid_ascii_and_multibyte() {
    // Valid chars must still round-trip byte-identically.
    for c in ['A', '\0', 'z', 'é', '€', '𝄞'] {
        let bytes = oxicode::encode_to_vec(&c).expect("encode char");
        let (decoded, consumed): (char, usize) =
            oxicode::decode_from_slice(&bytes).expect("decode char");
        assert_eq!(decoded, c);
        assert_eq!(consumed, bytes.len());
    }
}

// ===== [T; N] decode: leak safety on mid-array error =====

static DROP_COUNT: AtomicUsize = AtomicUsize::new(0);

/// `DROP_COUNT` is process-global, so the two tests that assert on it must not
/// run concurrently — otherwise one test's four drops land inside the other
/// test's window and the assertion sees a count it never produced.
static DROP_COUNT_LOCK: std::sync::Mutex<()> = std::sync::Mutex::new(());

struct DropCounter(#[allow(dead_code)] u8);

impl Drop for DropCounter {
    fn drop(&mut self) {
        DROP_COUNT.fetch_add(1, Ordering::SeqCst);
    }
}

impl Decode for DropCounter {
    fn decode<D: Decoder<Context = ()>>(decoder: &mut D) -> Result<Self, Error> {
        Ok(DropCounter(u8::decode(decoder)?))
    }
}

#[test]
fn array_decode_drops_initialized_prefix_on_error() {
    let _guard = DROP_COUNT_LOCK.lock().unwrap_or_else(|e| e.into_inner());
    DROP_COUNT.store(0, Ordering::SeqCst);

    // Feed only 2 bytes for a [DropCounter; 4]: elements 0 and 1 decode, then
    // element 2 fails with UnexpectedEnd. The drop guard must drop the two
    // already-initialized elements.
    let bytes = [7u8, 9];
    let result: Result<([DropCounter; 4], usize), _> = oxicode::decode_from_slice(&bytes);
    assert!(result.is_err(), "decode should fail on truncated input");

    assert_eq!(
        DROP_COUNT.load(Ordering::SeqCst),
        2,
        "exactly the two initialized elements must be dropped (no leak, no double-free)"
    );
}

#[test]
fn array_decode_full_success_does_not_overdrop() {
    let _guard = DROP_COUNT_LOCK.lock().unwrap_or_else(|e| e.into_inner());
    DROP_COUNT.store(0, Ordering::SeqCst);
    {
        let bytes = [1u8, 2, 3, 4];
        let (arr, consumed): ([DropCounter; 4], usize) =
            oxicode::decode_from_slice(&bytes).expect("decode full array");
        assert_eq!(consumed, 4);
        // Nothing dropped while the array is alive.
        assert_eq!(DROP_COUNT.load(Ordering::SeqCst), 0);
        drop(arr);
    }
    // Exactly 4 drops after the array goes out of scope.
    assert_eq!(DROP_COUNT.load(Ordering::SeqCst), 4);
}

// ===== u8 bulk fast path: byte-identical output =====

#[test]
fn u8_array_fast_path_bytes_identical() {
    // [u8; 4] has no length prefix, so it must encode to exactly its 4 bytes.
    let arr: [u8; 4] = [10, 20, 30, 40];
    let bytes = oxicode::encode_to_vec(&arr).expect("encode [u8; 4]");
    assert_eq!(bytes, vec![10, 20, 30, 40]);

    let (decoded, consumed): ([u8; 4], usize) =
        oxicode::decode_from_slice(&bytes).expect("decode [u8; 4]");
    assert_eq!(decoded, arr);
    assert_eq!(consumed, 4);
}

#[test]
fn non_u8_array_roundtrip_matches_per_element_path() {
    // The non-u8 path (per-element) must still round-trip identically.
    let arr: [u16; 4] = [1, 2, 3, 4];
    let bytes = oxicode::encode_to_vec(&arr).expect("encode [u16; 4]");
    let (decoded, _consumed): ([u16; 4], usize) =
        oxicode::decode_from_slice(&bytes).expect("decode [u16; 4]");
    assert_eq!(decoded, arr);
}

#[test]
fn u8_slice_fast_path_roundtrip() {
    let data: Vec<u8> = (0..=255u8).collect();
    let bytes = oxicode::encode_to_vec(&data).expect("encode Vec<u8>");
    let (decoded, _consumed): (Vec<u8>, usize) =
        oxicode::decode_from_slice(&bytes).expect("decode Vec<u8>");
    assert_eq!(decoded, data);
}

// ===== Cell<T> decode without the Copy bound =====

#[test]
fn cell_of_non_copy_type_decodes() {
    // Cell<String> is not Copy; decoding it must now work (Copy dropped).
    let s = String::from("hello world");
    let bytes = oxicode::encode_to_vec(&s).expect("encode String");
    let (cell, _consumed): (Cell<String>, usize) =
        oxicode::decode_from_slice(&bytes).expect("decode Cell<String>");
    assert_eq!(cell.into_inner(), "hello world");
}

// ===== BorrowDecode for [T; N] and Result<T, U> =====

#[test]
fn borrow_decode_array() {
    let arr: [u32; 3] = [111, 222, 333];
    let bytes = oxicode::encode_to_vec(&arr).expect("encode [u32; 3]");
    let (decoded, _consumed): ([u32; 3], usize) =
        oxicode::borrow_decode_from_slice(&bytes).expect("borrow_decode [u32; 3]");
    assert_eq!(decoded, arr);
}

#[test]
fn borrow_decode_u8_array_fast_path() {
    let arr: [u8; 5] = [9, 8, 7, 6, 5];
    let bytes = oxicode::encode_to_vec(&arr).expect("encode [u8; 5]");
    let (decoded, _consumed): ([u8; 5], usize) =
        oxicode::borrow_decode_from_slice(&bytes).expect("borrow_decode [u8; 5]");
    assert_eq!(decoded, arr);
}

#[test]
fn borrow_decode_result_ok_and_err() {
    let ok: Result<u32, u16> = Ok(42);
    let bytes = oxicode::encode_to_vec(&ok).expect("encode Ok");
    let (decoded, _consumed): (Result<u32, u16>, usize) =
        oxicode::borrow_decode_from_slice(&bytes).expect("borrow_decode Ok");
    assert_eq!(decoded, ok);

    let err: Result<u32, u16> = Err(7);
    let bytes = oxicode::encode_to_vec(&err).expect("encode Err");
    let (decoded, _consumed): (Result<u32, u16>, usize) =
        oxicode::borrow_decode_from_slice(&bytes).expect("borrow_decode Err");
    assert_eq!(decoded, err);
}
