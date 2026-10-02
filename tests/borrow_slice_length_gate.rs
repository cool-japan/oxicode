//! A `BorrowReader` outside the crate that hands back fewer bytes than it was
//! asked for must make the zero-copy `&[T]` decode fail, never read past the
//! end of its buffer: `BorrowReader` is public and unsealed, so the length
//! `take_bytes` hands back is checked before any unsafe code relies on it.

#![cfg(feature = "alloc")]

use oxicode::config;
use oxicode::de::{BorrowDecode, BorrowReader, DecoderImpl, Reader, SliceReader};
use oxicode::error::Error;

/// Wraps a `SliceReader` and returns half of every requested borrow.
struct ShortReader<'a> {
    inner: SliceReader<'a>,
}

impl<'a> Reader for ShortReader<'a> {
    fn read(&mut self, bytes: &mut [u8]) -> Result<(), Error> {
        self.inner.read(bytes)
    }

    fn remaining_bytes(&self) -> Option<usize> {
        self.inner.remaining_bytes()
    }
}

impl<'a> BorrowReader<'a> for ShortReader<'a> {
    fn take_bytes(&mut self, length: usize) -> Result<&'a [u8], Error> {
        // Everything but the length gate stays as the honest reader has it:
        // the bytes come from the same buffer, so alignment is unchanged.
        self.inner.take_bytes(length / 2)
    }
}

#[test]
fn a_reader_that_returns_fewer_bytes_than_asked_is_an_error_not_an_out_of_bounds_slice() {
    let cfg = config::standard().with_fixed_int_encoding();
    let original: Vec<u32> = (1..=16).collect();
    let encoded = oxicode::encode_to_vec_with_config(&original, cfg).expect("encode");
    // Over-allocate so that even an out-of-bounds read would not fault: the
    // test must fail on the error value, not on a crash.
    let mut buffer = encoded.clone();
    buffer.extend(vec![0u8; 256]);
    let mut decoder = DecoderImpl::new(
        ShortReader {
            inner: SliceReader::new(&buffer[..encoded.len()]),
        },
        cfg,
    );
    let decoded = <&[u32] as BorrowDecode<'_>>::borrow_decode(&mut decoder);
    match decoded {
        Err(Error::InvalidData { message }) => {
            assert!(message.contains("length"), "{message}");
        }
        other => panic!("a short reader must be refused: {other:?}"),
    }
    // The honest reader still decodes the same bytes zero-copy.
    let mut honest = DecoderImpl::new(SliceReader::new(&encoded), cfg);
    let decoded = <&[u32] as BorrowDecode<'_>>::borrow_decode(&mut honest).expect("borrow");
    assert_eq!(decoded, &original[..]);
}
