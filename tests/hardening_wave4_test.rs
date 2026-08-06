//! Wave 4 hardening regression tests.
//!
//! Two themes:
//!
//! 1. **Bounded IO decoding.** `Reader::remaining_bytes` gave slice-backed
//!    decoding an exact allocation bound; these tests cover the IO readers,
//!    which get the same bound once a byte budget is supplied. The important
//!    half of the coverage is not "a forged length is rejected" (easy) but
//!    "a payload whose length is *exactly* the budget still decodes" — that is
//!    what catches an off-by-one turning the bound into a false rejection.
//!
//! 2. **Context-generic derive.** `#[oxicode(context = "...")]` must produce
//!    impls usable with a non-unit decode context, while the default
//!    (`Context = ()`) derive stays byte-identical on the wire.

use oxicode::config;
use oxicode::de::{IoReader, Reader};
use oxicode::Error;
use std::io::Cursor;

/// Varint prefix for `u64::MAX` under the standard configuration: the 253
/// discriminant followed by eight little-endian `0xff` bytes.
const FORGED_U64_MAX_LEN: [u8; 9] = [253, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff];

// ---------------------------------------------------------------------------
// Reader::remaining_bytes — IO readers
// ---------------------------------------------------------------------------

#[test]
fn io_reader_without_budget_reports_unknown_remaining() {
    let reader = IoReader::new(Cursor::new(vec![1u8, 2, 3]));
    assert_eq!(reader.remaining_bytes(), None);
    assert_eq!(reader.remaining_limit(), None);
}

#[test]
fn io_reader_budget_is_exact_and_decrements_with_reads() {
    let mut reader = IoReader::with_limit(Cursor::new(vec![1u8, 2, 3, 4, 5]), 5);
    assert_eq!(reader.remaining_bytes(), Some(5));

    let mut buf = [0u8; 2];
    reader.read(&mut buf).expect("read within budget");
    assert_eq!(buf, [1, 2]);
    assert_eq!(reader.remaining_bytes(), Some(3));

    let mut buf = [0u8; 3];
    reader.read(&mut buf).expect("read exactly to the budget");
    assert_eq!(buf, [3, 4, 5]);
    assert_eq!(reader.remaining_bytes(), Some(0));
}

#[test]
fn io_reader_read_past_budget_is_typed_unexpected_end() {
    // The underlying cursor has plenty of bytes; only the budget stops us.
    let mut reader = IoReader::with_limit(Cursor::new(vec![0u8; 64]), 4);
    let mut buf = [0u8; 5];
    let err = reader.read(&mut buf).expect_err("budget must be enforced");
    assert!(
        matches!(err, Error::UnexpectedEnd { additional: 1 }),
        "expected UnexpectedEnd {{ additional: 1 }}, got {err:?}"
    );
    // The failed read consumed nothing.
    assert_eq!(reader.remaining_bytes(), Some(4));
}

#[test]
fn buffered_io_reader_budget_is_exact() {
    let mut reader = oxicode::BufferedIoReader::with_limit(Cursor::new(vec![7u8; 10]), 10);
    assert_eq!(reader.remaining_bytes(), Some(10));
    let mut buf = [0u8; 10];
    reader.read(&mut buf).expect("read exactly to the budget");
    assert_eq!(buf, [7u8; 10]);
    assert_eq!(reader.remaining_bytes(), Some(0));

    let mut buf = [0u8; 1];
    let err = reader.read(&mut buf).expect_err("budget is exhausted");
    assert!(matches!(err, Error::UnexpectedEnd { additional: 1 }));
}

#[test]
fn buffered_io_reader_eof_is_unexpected_end_not_io() {
    let mut reader = oxicode::BufferedIoReader::new(Cursor::new(vec![1u8, 2]));
    let mut buf = [0u8; 4];
    let err = reader.read(&mut buf).expect_err("cursor is too short");
    assert!(
        matches!(err, Error::UnexpectedEnd { .. }),
        "running out of input must be a decode error, got {err:?}"
    );
}

// ---------------------------------------------------------------------------
// Bounded IO decoding — the discriminating "exactly the budget" cases
// ---------------------------------------------------------------------------

#[test]
fn limited_std_read_accepts_payload_exactly_the_size_of_the_budget() {
    let value = "the quick brown fox".to_string();
    let bytes = oxicode::encode_to_vec(&value).expect("encode");
    let len = bytes.len();

    let decoded: String =
        oxicode::decode_from_std_read_limited(Cursor::new(bytes), config::standard(), len)
            .expect("a budget equal to the payload must not reject it");
    assert_eq!(decoded, value);
}

#[test]
fn limited_buffered_read_accepts_payload_exactly_the_size_of_the_budget() {
    let value: Vec<u8> = (0..=255u8).collect();
    let bytes = oxicode::encode_to_vec(&value).expect("encode");
    let len = bytes.len();

    let decoded: Vec<u8> =
        oxicode::decode_from_buffered_read_limited(Cursor::new(bytes), config::standard(), len)
            .expect("a budget equal to the payload must not reject it");
    assert_eq!(decoded, value);
}

#[test]
fn limited_buffered_read_accepts_a_budget_larger_than_the_payload() {
    let value = vec![1u32, 2, 3, 4];
    let bytes = oxicode::encode_to_vec(&value).expect("encode");
    let len = bytes.len();

    let decoded: Vec<u32> = oxicode::decode_from_buffered_read_limited(
        Cursor::new(bytes),
        config::standard(),
        len + 4096,
    )
    .expect("an over-large budget only bounds less, it never rejects");
    assert_eq!(decoded, value);
}

#[test]
fn limited_std_read_rejects_forged_length_before_allocating() {
    let err = oxicode::decode_from_std_read_limited::<String, _, _>(
        Cursor::new(FORGED_U64_MAX_LEN.to_vec()),
        config::standard(),
        FORGED_U64_MAX_LEN.len(),
    )
    .expect_err("a u64::MAX string length must be refused");
    assert!(
        matches!(err, Error::UnexpectedEnd { .. }),
        "expected UnexpectedEnd, got {err:?}"
    );
}

#[test]
fn limited_buffered_read_rejects_forged_vec_length_before_allocating() {
    let err = oxicode::decode_from_buffered_read_limited::<Vec<u8>, _>(
        Cursor::new(FORGED_U64_MAX_LEN.to_vec()),
        config::standard(),
        FORGED_U64_MAX_LEN.len(),
    )
    .expect_err("a u64::MAX byte-vector length must be refused");
    assert!(
        matches!(err, Error::UnexpectedEnd { .. }),
        "expected UnexpectedEnd, got {err:?}"
    );
}

#[test]
fn limited_read_rejects_a_length_that_only_slightly_exceeds_the_stream() {
    // 200 fits in a single varint byte, so the payload claims 200 bytes of
    // string data while the budget says only 8 remain.
    let mut input = vec![200u8];
    input.extend_from_slice(&[b'a'; 8]);
    let len = input.len();

    let err = oxicode::decode_from_std_read_limited::<String, _, _>(
        Cursor::new(input),
        config::standard(),
        len,
    )
    .expect_err("claimed length exceeds the budget");
    match err {
        Error::UnexpectedEnd { additional } => assert_eq!(additional, 200 - 8),
        other => panic!("expected UnexpectedEnd, got {other:?}"),
    }
}

#[test]
fn unbudgeted_io_decoding_still_round_trips() {
    // Regression guard for the incremental fallback: with no budget the reader
    // reports `None`, and decoding must still succeed for real input.
    let value = "x".repeat(40_000);
    let bytes = oxicode::encode_to_vec(&value).expect("encode");
    let decoded: String = oxicode::decode_from_std_read(Cursor::new(bytes), config::standard())
        .expect("unbudgeted decode of valid input");
    assert_eq!(decoded, value);
}

#[test]
fn unbudgeted_io_decoding_still_rejects_a_forged_length() {
    let err = oxicode::decode_from_std_read::<String, _, _>(
        Cursor::new(FORGED_U64_MAX_LEN.to_vec()),
        config::standard(),
    )
    .expect_err("the incremental path must fail on the first short read");
    assert!(
        matches!(err, Error::UnexpectedEnd { .. }),
        "expected UnexpectedEnd, got {err:?}"
    );
}

// ---------------------------------------------------------------------------
// File decoding picks up the file size as a real end-of-stream bound
// ---------------------------------------------------------------------------

#[test]
fn decode_from_file_bounds_allocation_by_the_file_size() {
    let mut path = std::env::temp_dir();
    path.push(format!(
        "oxicode_wave4_forged_len_{}_{:?}.bin",
        std::process::id(),
        std::thread::current().id()
    ));
    std::fs::write(&path, FORGED_U64_MAX_LEN).expect("write temp file");

    let result = oxicode::decode_from_file::<String>(&path);
    let _ = std::fs::remove_file(&path);

    let err = result.expect_err("a 9-byte file cannot hold a u64::MAX string");
    assert!(
        matches!(err, Error::UnexpectedEnd { .. }),
        "expected UnexpectedEnd, got {err:?}"
    );
}

#[test]
fn decode_from_file_still_round_trips_real_data() {
    let mut path = std::env::temp_dir();
    path.push(format!(
        "oxicode_wave4_roundtrip_{}_{:?}.bin",
        std::process::id(),
        std::thread::current().id()
    ));

    let value = (1u64..=64).collect::<Vec<u64>>();
    oxicode::encode_to_file(&value, &path).expect("encode to file");
    let decoded = oxicode::decode_from_file::<Vec<u64>>(&path);
    let _ = std::fs::remove_file(&path);

    assert_eq!(decoded.expect("decode from file"), value);
}

// ---------------------------------------------------------------------------
// A loose bound must not become a loose allocation
// ---------------------------------------------------------------------------

/// A reader that claims an enormous amount of input is left and records the
/// largest buffer decoding ever asks it to fill.
///
/// `Reader::remaining_bytes` is only ever an *upper* bound, so a caller that
/// passes a deliberately generous budget must not be able to turn a forged
/// length prefix into one allocation of that size. This reader makes the peak
/// request directly observable.
struct BoastfulReader {
    data: std::io::Cursor<Vec<u8>>,
    largest_read: std::rc::Rc<std::cell::Cell<usize>>,
}

impl Reader for BoastfulReader {
    fn read(&mut self, bytes: &mut [u8]) -> Result<(), Error> {
        if bytes.len() > self.largest_read.get() {
            self.largest_read.set(bytes.len());
        }
        use std::io::Read as _;
        self.data
            .read_exact(bytes)
            .map_err(|_| Error::UnexpectedEnd {
                additional: bytes.len(),
            })
    }

    fn remaining_bytes(&self) -> Option<usize> {
        // "There is a terabyte left, trust me."
        Some(1 << 40)
    }
}

#[test]
fn a_generous_bound_does_not_become_a_generous_allocation() {
    let largest_read = std::rc::Rc::new(std::cell::Cell::new(0usize));

    // A 1 GiB string length prefix (varint u64) followed by nothing.
    let mut input = vec![253u8];
    input.extend_from_slice(&(1u64 << 30).to_le_bytes());

    let reader = BoastfulReader {
        data: std::io::Cursor::new(input),
        largest_read: std::rc::Rc::clone(&largest_read),
    };

    let err = oxicode::decode_from_de_reader::<String, _, _>(reader, config::standard())
        .expect_err("there is no payload behind the 1 GiB length");
    assert!(
        matches!(err, Error::UnexpectedEnd { .. }),
        "expected UnexpectedEnd, got {err:?}"
    );

    // The decode must never have asked for the whole gigabyte in one step.
    let peak = largest_read.get();
    assert!(
        peak <= 16 * 1024 * 1024,
        "a forged 1 GiB length committed a {peak}-byte buffer in one step"
    );
}

#[test]
fn payloads_larger_than_the_eager_allocation_ceiling_still_round_trip() {
    // Exercises the multi-step materialization path: 17 MiB is above the
    // single-allocation ceiling, so the buffer is built in several steps and
    // must still come back byte-identical.
    let payload: Vec<u8> = (0..17 * 1024 * 1024u32).map(|i| (i % 251) as u8).collect();
    let bytes = oxicode::encode_to_vec(&payload).expect("encode");

    let (decoded, read): (Vec<u8>, usize) = oxicode::decode_from_slice(&bytes).expect("decode");
    assert_eq!(read, bytes.len());
    assert_eq!(decoded, payload);
}

// ---------------------------------------------------------------------------
// The serde bridge's IO path gets the same bound
// ---------------------------------------------------------------------------
//
// `String` / byte fields decoded through the serde deserializer go through the
// same `String::decode` / `Vec::<u8>::decode` impls as the native path, so the
// remaining-input bound applies to them too — but only if the reader has one.
// `oxicode::serde::decode_from_std_read` has no way to know the stream length;
// `decode_from_std_read_limited` is the budgeted counterpart.
#[cfg(feature = "serde")]
mod serde_bounded_io {
    use super::FORGED_U64_MAX_LEN;
    use oxicode::config;
    use oxicode::Error;
    use std::io::Cursor;

    #[test]
    fn budgeted_serde_read_rejects_a_forged_length_prefix() {
        let err = oxicode::serde::decode_from_std_read_limited::<String, _, _>(
            Cursor::new(FORGED_U64_MAX_LEN.to_vec()),
            config::standard(),
            FORGED_U64_MAX_LEN.len(),
        )
        .expect_err("a 16 EiB string cannot fit a 9-byte stream");
        assert!(
            matches!(err, Error::UnexpectedEnd { .. }),
            "expected UnexpectedEnd, got {err:?}"
        );
    }

    #[test]
    fn budgeted_serde_read_accepts_a_payload_exactly_the_size_of_the_budget() {
        // The off-by-one case: the bound must reject one byte *more* than the
        // stream holds, never the stream itself.
        let value = "border case".to_string();
        let bytes = oxicode::serde::encode_to_vec(&value, config::standard()).expect("encode");
        let len = bytes.len();

        let (decoded, read): (String, usize) = oxicode::serde::decode_from_std_read_limited(
            Cursor::new(bytes),
            config::standard(),
            len,
        )
        .expect("a payload the exact size of the budget must decode");
        assert_eq!(decoded, value);
        assert_eq!(read, len);
    }

    #[test]
    fn a_budget_one_byte_short_is_a_typed_rejection_not_a_panic() {
        let value = vec![7u8; 64];
        let bytes = oxicode::serde::encode_to_vec(&value, config::standard()).expect("encode");
        let len = bytes.len();

        let err = oxicode::serde::decode_from_std_read_limited::<Vec<u8>, _, _>(
            Cursor::new(bytes),
            config::standard(),
            len - 1,
        )
        .expect_err("the budget is one byte short of the payload");
        assert!(
            matches!(err, Error::UnexpectedEnd { .. }),
            "expected UnexpectedEnd, got {err:?}"
        );
    }

    #[test]
    fn a_budget_stops_at_the_message_boundary_in_a_shared_stream() {
        // Two messages back to back. Decoding the first under its own budget
        // must not consume any of the second, so the same stream can be read
        // again for the next frame.
        let first = "one".to_string();
        let second = "two".to_string();
        let first_bytes =
            oxicode::serde::encode_to_vec(&first, config::standard()).expect("encode");
        let second_bytes =
            oxicode::serde::encode_to_vec(&second, config::standard()).expect("encode");

        let mut stream = Vec::new();
        stream.extend_from_slice(&first_bytes);
        stream.extend_from_slice(&second_bytes);
        let mut cursor = Cursor::new(stream);

        let (decoded_first, read_first): (String, usize) =
            oxicode::serde::decode_from_std_read_limited(
                &mut cursor,
                config::standard(),
                first_bytes.len(),
            )
            .expect("first frame");
        assert_eq!(decoded_first, first);
        assert_eq!(read_first, first_bytes.len());

        let (decoded_second, read_second): (String, usize) =
            oxicode::serde::decode_from_std_read_limited(
                &mut cursor,
                config::standard(),
                second_bytes.len(),
            )
            .expect("second frame reads from where the first stopped");
        assert_eq!(decoded_second, second);
        assert_eq!(read_second, second_bytes.len());
    }
}
