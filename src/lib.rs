//! # OxiCode
//!
//! OxiCode is a modern binary serialization library for Rust, serving as the successor to bincode.
//!
//! It provides a compact, efficient binary encoding scheme with zero-fluff serialization.
//! The encoded size is typically equal to or smaller than the in-memory representation.
//!
//! ## Features
//!
//! - **Compact encoding**: Minimal overhead in serialized format
//! - **Fast**: Optimized for performance
//! - **Flexible**: Support for various encoding configurations
//! - **Safe**: No unwrap() policy, comprehensive error handling
//! - **Modern**: Built with latest Rust practices and patterns
//!
//! ## Example
//!
//! ```rust,ignore
//! use oxicode::{Encode, Decode};
//!
//! #[derive(Encode, Decode, PartialEq, Debug)]
//! struct Point {
//!     x: f32,
//!     y: f32,
//! }
//!
//! fn main() -> Result<(), Box<dyn std::error::Error>> {
//!     let point = Point { x: 1.0, y: 2.0 };
//!
//!     // Encode to bytes
//!     let encoded = oxicode::encode(&point)?;
//!
//!     // Decode from bytes
//!     let decoded: Point = oxicode::decode(&encoded)?;
//!
//!     assert_eq!(point, decoded);
//!     Ok(())
//! }
//! ```
//!
//! ## Relation to bincode
//!
//! OxiCode is designed as the spiritual successor to bincode, maintaining compatibility
//! with the core concepts while introducing modern improvements and best practices.

#![cfg_attr(not(feature = "std"), no_std)]
#![cfg_attr(docsrs, feature(doc_cfg))]
#![deny(missing_docs)]
#![warn(rust_2018_idioms)]

#[cfg(feature = "alloc")]
extern crate alloc;

pub mod config;
pub mod de;
pub mod display;
pub mod enc;
pub mod error;

// Re-export old names for backward compatibility during transition
#[doc(hidden)]
pub mod decode {
    pub use crate::de::*;
}
#[doc(hidden)]
pub mod encode {
    #[cfg(feature = "alloc")]
    pub use crate::enc::VecWriter;
    pub use crate::enc::{SizeWriter, SliceWriter, Writer};
}

pub(crate) mod utils;
pub(crate) mod varint;

// Features module is always present (atomic types work in no_std)
mod features;

// SIMD-optimized encoding/decoding
#[cfg(feature = "simd")]
pub mod simd;

// Compression support
#[cfg(any(
    feature = "compression-lz4",
    feature = "compression-zstd",
    feature = "compression-zstd-pure"
))]
pub mod compression;

// Schema versioning support
pub mod versioning;

// Streaming serialization support
#[cfg(feature = "alloc")]
pub mod streaming;

// Async tokio convenience module — re-exports AsyncStreamingEncoder / AsyncStreamingDecoder
// under the shorter names AsyncEncoder and AsyncDecoder.
#[cfg(feature = "async-tokio")]
pub mod async_tokio {
    //! Convenience re-exports for async tokio streaming.
    //!
    //! Provides `AsyncEncoder` and `AsyncDecoder` as shorter aliases for
    //! [`crate::streaming::AsyncStreamingEncoder`] and
    //! [`crate::streaming::AsyncStreamingDecoder`].
    pub use crate::streaming::AsyncStreamingDecoder as AsyncDecoder;
    pub use crate::streaming::AsyncStreamingEncoder as AsyncEncoder;
    pub use crate::streaming::{
        CancellableAsyncDecoder, CancellableAsyncEncoder, CancellationToken,
    };
    pub use crate::streaming::{StreamingConfig, StreamingProgress};
}

// Async IO convenience module — mirrors `async_tokio` with the `async_io` name.
// Provides `AsyncEncoder`, `AsyncDecoder`, and `StreamingConfig` for tests that
// import from `oxicode::async_io`.
#[cfg(feature = "async-tokio")]
pub mod async_io {
    //! Convenience re-exports for async IO streaming (alias of [`crate::async_tokio`]).
    //!
    //! Provides `AsyncEncoder`, `AsyncDecoder`, and `StreamingConfig` as short names
    //! for the underlying streaming types.
    pub use crate::streaming::AsyncStreamingDecoder as AsyncDecoder;
    pub use crate::streaming::AsyncStreamingEncoder as AsyncEncoder;
    pub use crate::streaming::{
        CancellableAsyncDecoder, CancellableAsyncEncoder, CancellationToken,
    };
    pub use crate::streaming::{StreamingConfig, StreamingProgress};
}

// Validation middleware
pub mod validation;

// Checksum/integrity verification
#[cfg(feature = "checksum")]
pub mod checksum;

pub use de::BorrowDecode;
#[cfg(feature = "std")]
pub use de::BufferedIoReader;
pub use de::Decode;
pub use display::EncodedBytes;
#[cfg(feature = "alloc")]
pub use display::EncodedBytesOwned;
pub use enc::Encode;
pub use error::{Error, Result};

#[cfg(feature = "derive")]
pub use oxicode_derive::{BorrowDecode, Decode, Encode};

#[cfg(feature = "serde")]
pub use features::serde;

/// Encode a value to a `Vec<u8>` using the standard configuration.
///
/// # Examples
///
/// ```
/// let bytes = oxicode::encode_to_vec(&42u32).expect("encode failed");
/// assert!(!bytes.is_empty());
/// ```
#[cfg(feature = "alloc")]
pub fn encode_to_vec<E: Encode>(value: &E) -> Result<alloc::vec::Vec<u8>> {
    encode_to_vec_with_config(value, config::standard())
}

/// Encode a value to a `Vec<u8>` with a custom configuration.
///
/// # Examples
///
/// ```
/// let config = oxicode::config::standard().with_fixed_int_encoding();
/// let bytes = oxicode::encode_to_vec_with_config(&42u32, config).expect("encode");
/// assert_eq!(bytes.len(), 4); // u32 with fixed encoding is always 4 bytes
/// ```
#[cfg(feature = "alloc")]
pub fn encode_to_vec_with_config<E: Encode, C: config::Config>(
    value: &E,
    config: C,
) -> Result<alloc::vec::Vec<u8>> {
    let writer = enc::VecWriter::new();
    let mut encoder = enc::EncoderImpl::new(writer, config);
    value.encode(&mut encoder)?;
    Ok(encoder.into_writer().into_vec())
}

/// Calculate the encoded size of a value using the standard configuration.
///
/// This uses a [`SizeWriter`](enc::SizeWriter) internally to count bytes
/// without actually encoding to a buffer.
///
/// # Examples
///
/// ```
/// let size = oxicode::encoded_size(&100u64).expect("size failed");
/// assert!(size > 0);
/// ```
pub fn encoded_size<E: Encode>(value: &E) -> Result<usize> {
    encoded_size_with_config(value, config::standard())
}

/// Calculate the encoded size of a value with a custom configuration.
///
/// This uses a [`SizeWriter`](enc::SizeWriter) internally to count bytes
/// without actually encoding to a buffer.
pub fn encoded_size_with_config<E: Encode, C: config::Config>(
    value: &E,
    config: C,
) -> Result<usize> {
    let writer = enc::SizeWriter::new();
    let mut encoder = enc::EncoderImpl::new(writer, config);
    value.encode(&mut encoder)?;
    Ok(encoder.into_writer().bytes_written())
}

/// Encode a value into a byte slice.
///
/// Returns the number of bytes written into `dst`.
///
/// # Examples
///
/// ```
/// let mut buf = [0u8; 16];
/// let written = oxicode::encode_into_slice(42u32, &mut buf, oxicode::config::standard())
///     .expect("encode");
/// assert!(written > 0);
/// ```
pub fn encode_into_slice<E: Encode, C: config::Config>(
    value: E,
    dst: &mut [u8],
    config: C,
) -> Result<usize> {
    let writer = enc::SliceWriter::new(dst);
    let mut encoder = enc::EncoderImpl::new(writer, config);
    value.encode(&mut encoder)?;
    Ok(encoder.into_writer().bytes_written())
}

/// Encode a value into a fixed-size stack-allocated array using the standard configuration.
///
/// Returns `(array, bytes_written)`. Fails with [`Error`] if the encoded size exceeds `N` bytes.
///
/// This is ideal for no-alloc encoding on embedded systems or network packet serialization
/// into fixed-size buffers.
///
/// # Examples
///
/// ```
/// let (arr, n): ([u8; 16], _) = oxicode::encode_to_fixed_array(&42u32).expect("encode failed");
/// assert!(n > 0);
/// assert!(n <= 16);
/// ```
pub fn encode_to_fixed_array<const N: usize, E: Encode>(value: &E) -> Result<([u8; N], usize)> {
    encode_to_fixed_array_with_config(value, config::standard())
}

/// Encode a value into a fixed-size stack-allocated array with a custom configuration.
///
/// Returns `(array, bytes_written)`. Fails with [`Error`] if the encoded size exceeds `N` bytes.
pub fn encode_to_fixed_array_with_config<const N: usize, E: Encode, C: config::Config>(
    value: &E,
    config: C,
) -> Result<([u8; N], usize)> {
    let mut buf = [0u8; N];
    let writer = enc::SliceWriter::new(&mut buf);
    let mut encoder = enc::EncoderImpl::new(writer, config);
    value.encode(&mut encoder)?;
    let bytes_written = encoder.into_writer().bytes_written();
    Ok((buf, bytes_written))
}

/// Convenience: decode a value from a byte slice using the standard config,
/// discarding the consumed byte count.
///
/// This is a simpler alternative to [`decode_from_slice`] when you don't need
/// to know how many bytes were consumed.
///
/// # Examples
///
/// ```
/// let encoded = oxicode::encode_to_vec(&42u64).expect("encode failed");
/// let decoded: u64 = oxicode::decode_value(&encoded).expect("decode failed");
/// assert_eq!(decoded, 42u64);
/// ```
pub fn decode_value<D: Decode>(src: &[u8]) -> Result<D> {
    let (value, _) = decode_from_slice(src)?;
    Ok(value)
}

/// Convenience: encode a value to `Vec<u8>` using the standard config.
///
/// Alias for [`encode_to_vec`] for ergonomics.
#[cfg(feature = "alloc")]
pub fn encode_bytes<E: Encode>(value: &E) -> Result<alloc::vec::Vec<u8>> {
    encode_to_vec(value)
}

/// Encode a `Copy` value into a `Vec<u8>` using default config.
/// Unlike `encode_to_vec` which takes `&E`, this takes `E` by value.
///
/// # Example
/// ```
/// let bytes = oxicode::encode_copy(42u32).expect("encode");
/// let (val, _): (u32, _) = oxicode::decode_from_slice(&bytes).expect("decode");
/// assert_eq!(val, 42u32);
/// ```
#[cfg(feature = "alloc")]
pub fn encode_copy<E: Encode + Copy>(value: E) -> Result<alloc::vec::Vec<u8>> {
    encode_to_vec(&value)
}

/// Encode an iterator of items as a length-prefixed sequence into a `Vec<u8>`.
///
/// Internally collects the iterator to determine the length (required by the wire format),
/// then encodes using the same format as `Vec<T>`. The result is byte-for-byte identical
/// to encoding a `Vec<E>` containing the same elements.
///
/// # Examples
///
/// ```
/// let items = [1u32, 2, 3, 4, 5];
/// let encoded = oxicode::encode_iter_to_vec(items.iter().copied()).expect("encode");
/// let (decoded, _): (Vec<u32>, _) = oxicode::decode_from_slice(&encoded).expect("decode");
/// assert_eq!(decoded, vec![1, 2, 3, 4, 5]);
/// ```
#[cfg(feature = "alloc")]
pub fn encode_iter_to_vec<E: Encode, I: IntoIterator<Item = E>>(
    iter: I,
) -> Result<alloc::vec::Vec<u8>> {
    encode_iter_to_vec_with_config(iter, config::standard())
}

/// Encode an iterator of items as a length-prefixed sequence into a `Vec<u8>` with a custom configuration.
///
/// See [`encode_iter_to_vec`] for details and examples.
#[cfg(feature = "alloc")]
pub fn encode_iter_to_vec_with_config<E: Encode, I: IntoIterator<Item = E>, C: config::Config>(
    iter: I,
    config: C,
) -> Result<alloc::vec::Vec<u8>> {
    let items: alloc::vec::Vec<E> = iter.into_iter().collect();
    encode_to_vec_with_config(&items, config)
}

/// Encode an exact-size iterator as a length-prefixed sequence into a `Vec<u8>`.
///
/// Unlike [`encode_iter_to_vec`], this uses the iterator's [`ExactSizeIterator::len()`]
/// to write the length prefix first, then encodes items lazily — no intermediate `Vec` allocation.
///
/// The result is byte-for-byte identical to encoding a `Vec<E>` containing the same elements.
///
/// # Examples
///
/// ```
/// let encoded = oxicode::encode_seq_to_vec(0u32..5).expect("encode");
/// let (decoded, _): (Vec<u32>, _) = oxicode::decode_from_slice(&encoded).expect("decode");
/// assert_eq!(decoded, vec![0, 1, 2, 3, 4]);
/// ```
#[cfg(feature = "alloc")]
pub fn encode_seq_to_vec<E: Encode, I: ExactSizeIterator<Item = E>>(
    iter: I,
) -> Result<alloc::vec::Vec<u8>> {
    encode_seq_to_vec_with_config(iter, config::standard())
}

/// Encode an exact-size iterator as a length-prefixed sequence into a `Vec<u8>` with a custom configuration.
///
/// See [`encode_seq_to_vec`] for details and examples.
#[cfg(feature = "alloc")]
pub fn encode_seq_to_vec_with_config<
    E: Encode,
    I: ExactSizeIterator<Item = E>,
    C: config::Config,
>(
    iter: I,
    config: C,
) -> Result<alloc::vec::Vec<u8>> {
    let len = iter.len();
    let writer = enc::VecWriter::new();
    let mut encoder = enc::EncoderImpl::new(writer, config);
    enc::encode_slice_len(&mut encoder, len)?;
    for item in iter {
        item.encode(&mut encoder)?;
    }
    Ok(encoder.into_writer().into_vec())
}

/// Encode an exact-size iterator into a pre-allocated byte slice.
///
/// Returns the number of bytes written. This is suited for no-alloc environments.
///
/// The result is byte-for-byte identical to encoding a `Vec<E>` containing the same elements.
///
/// # Examples
///
/// ```
/// let mut buf = [0u8; 64];
/// let items: Vec<u32> = vec![1, 2, 3];
/// let n = oxicode::encode_seq_into_slice(items.into_iter(), &mut buf).expect("encode");
/// let (decoded, _): (Vec<u32>, _) = oxicode::decode_from_slice(&buf[..n]).expect("decode");
/// assert_eq!(decoded, vec![1, 2, 3]);
/// ```
pub fn encode_seq_into_slice<E: Encode, I: ExactSizeIterator<Item = E>>(
    iter: I,
    dst: &mut [u8],
) -> Result<usize> {
    encode_seq_into_slice_with_config(iter, dst, config::standard())
}

/// Encode an exact-size iterator into a pre-allocated byte slice with a custom configuration.
///
/// See [`encode_seq_into_slice`] for details and examples.
pub fn encode_seq_into_slice_with_config<
    E: Encode,
    I: ExactSizeIterator<Item = E>,
    C: config::Config,
>(
    iter: I,
    dst: &mut [u8],
    config: C,
) -> Result<usize> {
    let len = iter.len();
    let writer = enc::SliceWriter::new(dst);
    let mut encoder = enc::EncoderImpl::new(writer, config);
    enc::encode_slice_len(&mut encoder, len)?;
    for item in iter {
        item.encode(&mut encoder)?;
    }
    Ok(encoder.into_writer().bytes_written())
}

/// An iterator that lazily decodes items from an encoded sequence.
///
/// Created by [`decode_iter_from_slice`] or [`decode_iter_from_slice_with_config`].
/// Items are decoded one at a time as [`Iterator::next`] is called, avoiding the need
/// to load the entire collection into memory at once. This is more memory-efficient
/// than [`decode_from_slice`] for large collections.
///
/// Each call to `next()` returns `Some(Result<T>)` until all items have been
/// yielded, at which point `None` is returned.
#[cfg(feature = "alloc")]
pub struct DecodeIter<T, D: de::Decoder> {
    decoder: D,
    remaining: u64,
    _marker: core::marker::PhantomData<T>,
}

#[cfg(feature = "alloc")]
impl<T: de::Decode, D: de::Decoder<Context = ()>> Iterator for DecodeIter<T, D> {
    type Item = Result<T>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.remaining == 0 {
            return None;
        }
        self.remaining -= 1;
        Some(T::decode(&mut self.decoder))
    }
}

/// Decode a sequence lazily, yielding items one at a time without loading all into memory.
///
/// Reads the length prefix from the encoded bytes (same wire format as `Vec<T>`), then
/// returns an iterator that decodes items on demand. More memory-efficient than
/// [`decode_from_slice`] for large collections.
///
/// Each item yielded is `Result<T>`, allowing per-item error handling.
///
/// # Examples
///
/// ```
/// let data = vec![1u32, 2, 3, 4, 5];
/// let encoded = oxicode::encode_to_vec(&data).expect("encode");
/// let sum: u32 = oxicode::decode_iter_from_slice::<u32>(&encoded)
///     .expect("init")
///     .filter_map(|r| r.ok())
///     .sum();
/// assert_eq!(sum, 15);
/// ```
#[cfg(feature = "alloc")]
pub fn decode_iter_from_slice<T: de::Decode>(
    src: &[u8],
) -> Result<DecodeIter<T, de::DecoderImpl<de::SliceReader<'_>, config::Configuration>>> {
    decode_iter_from_slice_with_config(src, config::standard())
}

/// Decode a sequence lazily with a custom configuration.
///
/// See [`decode_iter_from_slice`] for details and examples.
#[cfg(feature = "alloc")]
pub fn decode_iter_from_slice_with_config<T: de::Decode, C: config::Config>(
    src: &[u8],
    config: C,
) -> Result<DecodeIter<T, de::DecoderImpl<de::SliceReader<'_>, C>>> {
    use de::Decode as _;
    let reader = de::SliceReader::new(src);
    let mut decoder = de::DecoderImpl::new(reader, config);
    // Read the u64 length prefix — same as Vec<T> wire format
    let remaining = u64::decode(&mut decoder)?;
    Ok(DecodeIter {
        decoder,
        remaining,
        _marker: core::marker::PhantomData,
    })
}

/// Wrap a byte slice for human-readable display.
///
/// Returns an [`EncodedBytes`] wrapper that implements [`core::fmt::Display`],
/// [`core::fmt::LowerHex`], and [`core::fmt::UpperHex`].
///
/// # Examples
///
/// ```
/// let bytes = oxicode::encode_to_vec(&255u8).expect("encode failed");
/// let display = oxicode::encoded_bytes(&bytes);
/// let hex = format!("{:x}", display);
/// assert!(!hex.is_empty());
/// ```
pub fn encoded_bytes(bytes: &[u8]) -> display::EncodedBytes<'_> {
    display::EncodedBytes(bytes)
}

/// Encode a value and return it wrapped for human-readable display.
///
/// Returns an [`EncodedBytesOwned`] that implements `Display`, `LowerHex`,
/// `UpperHex`, and provides a `hex_dump()` method.
#[cfg(feature = "alloc")]
pub fn encode_to_display<E: Encode>(value: &E) -> Result<display::EncodedBytesOwned> {
    let bytes = encode_to_vec(value)?;
    Ok(display::EncodedBytesOwned(bytes))
}

/// Encode a value with a version header for forward-compatible storage.
///
/// The output contains an OXIV version header followed by the oxicode-encoded value.
/// Use [`decode_versioned_value`] to recover the value and version.
///
/// # Examples
///
/// ```
/// use oxicode::versioning::Version;
/// let version = Version::new(1, 0, 0);
/// let encoded = oxicode::encode_versioned_value(&42u32, version).expect("encode failed");
/// let (decoded, ver, _consumed): (u32, _, _) =
///     oxicode::decode_versioned_value(&encoded).expect("decode failed");
/// assert_eq!(decoded, 42u32);
/// assert_eq!(ver, version);
/// ```
#[cfg(feature = "alloc")]
pub fn encode_versioned_value<E: Encode>(
    value: &E,
    version: versioning::Version,
) -> Result<alloc::vec::Vec<u8>> {
    let payload = encode_to_vec(value)?;
    versioning::encode_versioned(&payload, version)
}

/// Decode a versioned value, returning `(value, version, bytes_consumed)`.
///
/// The input must have been produced by [`encode_versioned_value`].
///
/// # Examples
///
/// ```
/// use oxicode::versioning::Version;
/// let version = Version::new(2, 5, 0);
/// let encoded = oxicode::encode_versioned_value(&99u64, version).expect("encode failed");
/// let (decoded, ver, _consumed): (u64, _, _) =
///     oxicode::decode_versioned_value(&encoded).expect("decode failed");
/// assert_eq!(decoded, 99u64);
/// assert_eq!(ver, version);
/// ```
#[cfg(feature = "alloc")]
pub fn decode_versioned_value<D: Decode>(src: &[u8]) -> Result<(D, versioning::Version, usize)> {
    let (payload, version) = versioning::decode_versioned(src)?;
    let header_size = src.len() - payload.len();
    let (value, consumed) = decode_from_slice(&payload)?;
    Ok((value, version, header_size + consumed))
}

/// Encode `value` into any `std::io::Write` implementor using the standard configuration.
///
/// Returns the number of bytes written. This is a convenience wrapper that uses the
/// default configuration.
///
/// # Examples
///
/// ```
/// let mut buf = Vec::new();
/// let n = oxicode::encode_to_writer(&42u32, &mut buf).expect("encode");
/// assert!(n > 0);
/// assert_eq!(n, buf.len());
/// ```
#[cfg(feature = "std")]
pub fn encode_to_writer<E: Encode, W: std::io::Write>(value: &E, writer: W) -> Result<usize> {
    let io_writer = enc::IoWriter::new(writer);
    let mut encoder = enc::EncoderImpl::new(io_writer, config::standard());
    value.encode(&mut encoder)?;
    Ok(encoder.into_writer().bytes_written())
}

/// Decode a value from any `std::io::Read` implementor using the standard configuration.
///
/// Returns `(value, bytes_read)`. This is a convenience wrapper that uses the default
/// configuration and tracks the number of bytes consumed.
///
/// # Examples
///
/// ```
/// use std::io::Cursor;
/// let bytes = oxicode::encode_to_vec(&42u32).expect("encode");
/// let cursor = Cursor::new(&bytes);
/// let (decoded, n): (u32, _) = oxicode::decode_from_reader(cursor).expect("decode");
/// assert_eq!(decoded, 42u32);
/// assert_eq!(n, bytes.len());
/// ```
#[cfg(feature = "std")]
pub fn decode_from_reader<D: Decode, R: std::io::Read>(reader: R) -> Result<(D, usize)> {
    /// A counting wrapper around `std::io::Read` that tracks bytes consumed.
    struct CountingReader<R: std::io::Read> {
        inner: R,
        count: usize,
    }

    impl<R: std::io::Read> CountingReader<R> {
        fn new(inner: R) -> Self {
            Self { inner, count: 0 }
        }
    }

    impl<R: std::io::Read> de::Reader for CountingReader<R> {
        fn read(&mut self, bytes: &mut [u8]) -> Result<()> {
            self.inner.read_exact(bytes).map_err(|e| {
                if e.kind() == std::io::ErrorKind::UnexpectedEof {
                    Error::UnexpectedEnd {
                        additional: bytes.len(),
                    }
                } else {
                    Error::Io {
                        kind: e.kind(),
                        message: e.to_string(),
                    }
                }
            })?;
            self.count += bytes.len();
            Ok(())
        }
    }

    let counting = CountingReader::new(reader);
    let mut decoder = de::DecoderImpl::new(counting, config::standard());
    let value = D::decode(&mut decoder)?;
    let bytes_read = decoder.reader().count;
    Ok((value, bytes_read))
}

/// Encode `value` into any `std::io::Write` using a custom configuration.
///
/// Returns the number of bytes written. This is a convenience wrapper around
/// [`encode_into_std_write`] for ergonomics.
///
/// # Examples
///
/// ```
/// use std::io::Cursor;
/// let mut buf = Cursor::new(Vec::new());
/// let n = oxicode::encode_to_writer_with_config(&42u32, &mut buf, oxicode::config::standard())
///     .expect("encode");
/// assert!(n > 0);
/// ```
#[cfg(feature = "std")]
pub fn encode_to_writer_with_config<E: Encode, W: std::io::Write, C: config::Config>(
    value: &E,
    writer: W,
    config: C,
) -> Result<usize> {
    let io_writer = enc::IoWriter::new(writer);
    let mut encoder = enc::EncoderImpl::new(io_writer, config);
    value.encode(&mut encoder)?;
    Ok(encoder.into_writer().bytes_written())
}

/// Decode a value from any `std::io::Read` using a custom configuration.
///
/// Returns `(value, bytes_read)`. This is a convenience wrapper that tracks
/// the number of bytes consumed from the reader.
///
/// # Examples
///
/// ```
/// use std::io::Cursor;
/// let bytes = oxicode::encode_to_vec(&42u32).expect("encode");
/// let cursor = Cursor::new(&bytes);
/// let (decoded, n): (u32, _) = oxicode::decode_from_reader_with_config(
///     cursor,
///     oxicode::config::standard(),
/// )
/// .expect("decode");
/// assert_eq!(decoded, 42u32);
/// assert_eq!(n, bytes.len());
/// ```
#[cfg(feature = "std")]
pub fn decode_from_reader_with_config<D: Decode, R: std::io::Read, C: config::Config>(
    reader: R,
    config: C,
) -> Result<(D, usize)> {
    struct CountingReader<R: std::io::Read> {
        inner: R,
        count: usize,
    }

    impl<R: std::io::Read> CountingReader<R> {
        fn new(inner: R) -> Self {
            Self { inner, count: 0 }
        }
    }

    impl<R: std::io::Read> de::Reader for CountingReader<R> {
        fn read(&mut self, bytes: &mut [u8]) -> Result<()> {
            self.inner.read_exact(bytes).map_err(|e| {
                if e.kind() == std::io::ErrorKind::UnexpectedEof {
                    Error::UnexpectedEnd {
                        additional: bytes.len(),
                    }
                } else {
                    Error::Io {
                        kind: e.kind(),
                        message: e.to_string(),
                    }
                }
            })?;
            self.count += bytes.len();
            Ok(())
        }
    }

    let counting = CountingReader::new(reader);
    let mut decoder = de::DecoderImpl::new(counting, config);
    let value = D::decode(&mut decoder)?;
    let bytes_read = decoder.reader().count;
    Ok((value, bytes_read))
}

/// Encode `value` into a `Vec<u8>` with a pre-allocated size hint.
///
/// Useful when you have an estimate of the encoded size to reduce allocations.
/// Even if `size_hint` is too small, this function will still work correctly
/// (the `Vec` grows as needed). If `size_hint` is larger than needed, the
/// `Vec` is truncated to the actual encoded size.
///
/// # Examples
///
/// ```
/// let buf = oxicode::encode_to_vec_with_size_hint(&42u32, 64).expect("encode");
/// assert!(!buf.is_empty());
/// // Result is identical to encode_to_vec
/// let expected = oxicode::encode_to_vec(&42u32).expect("encode");
/// assert_eq!(buf, expected);
/// ```
#[cfg(feature = "std")]
pub fn encode_to_vec_with_size_hint<E: Encode>(
    value: &E,
    size_hint: usize,
) -> Result<std::vec::Vec<u8>> {
    let mut buf = std::vec::Vec::with_capacity(size_hint);
    let io_writer = enc::IoWriter::new(&mut buf);
    let mut encoder = enc::EncoderImpl::new(io_writer, config::standard());
    value.encode(&mut encoder)?;
    let len = encoder.into_writer().bytes_written();
    buf.truncate(len);
    Ok(buf)
}

/// Decode a value from any `std::io::Read` using an internal 8 KiB buffer.
///
/// Significantly more efficient than [`decode_from_std_read`] for file/network
/// sources because it batches syscalls via [`BufferedIoReader`].
///
/// # Examples
///
/// ```
/// use std::io::Cursor;
///
/// let bytes = oxicode::encode_to_vec(&99u32).expect("encode failed");
/// let cursor = Cursor::new(bytes);
/// let decoded: u32 = oxicode::decode_from_buffered_read(cursor, oxicode::config::standard())
///     .expect("decode failed");
/// assert_eq!(decoded, 99u32);
/// ```
#[cfg(feature = "std")]
pub fn decode_from_buffered_read<D: Decode, R: std::io::Read>(
    src: R,
    config: impl config::Config,
) -> Result<D> {
    let reader = de::read::BufferedIoReader::new(src);
    let mut decoder = de::DecoderImpl::new(reader, config);
    D::decode(&mut decoder)
}

/// Decode a value from a byte slice using the standard configuration.
///
/// Returns the decoded value and the number of bytes consumed.
///
/// # Examples
///
/// ```
/// let bytes = oxicode::encode_to_vec(&1234u32).expect("encode failed");
/// let (value, consumed): (u32, _) = oxicode::decode_from_slice(&bytes).expect("decode failed");
/// assert_eq!(value, 1234u32);
/// assert_eq!(consumed, bytes.len());
/// ```
pub fn decode_from_slice<D: Decode>(src: &[u8]) -> Result<(D, usize)> {
    decode_from_slice_with_config(src, config::standard())
}

/// Decode a value from a byte slice with a custom configuration.
///
/// Returns the decoded value and the number of bytes consumed.
///
/// # Examples
///
/// ```
/// let config = oxicode::config::standard();
/// let bytes = oxicode::encode_to_vec_with_config(&99u32, config).expect("encode");
/// let (value, consumed): (u32, _) =
///     oxicode::decode_from_slice_with_config(&bytes, config).expect("decode");
/// assert_eq!(value, 99u32);
/// assert_eq!(consumed, bytes.len());
/// ```
pub fn decode_from_slice_with_config<D: Decode, C: config::Config>(
    src: &[u8],
    config: C,
) -> Result<(D, usize)> {
    let reader = de::SliceReader::new(src);
    let mut decoder = de::DecoderImpl::new(reader, config);
    let result = D::decode(&mut decoder)?;
    let bytes_read = src.len() - decoder.reader().slice.len();
    Ok((result, bytes_read))
}

/// Encode a value into a writer using the given configuration.
///
/// Use [`enc::SizeWriter`] to measure encoded size without allocating, or
/// [`enc::SliceWriter`] to encode into a stack buffer.
///
/// # Examples
///
/// ```
/// use oxicode::enc::SizeWriter;
/// let writer = SizeWriter::new();
/// let mut encoder_result = oxicode::encode_into_slice(42u8, &mut [0u8; 4], oxicode::config::standard())
///     .expect("encode");
/// assert_eq!(encoder_result, 1);
/// ```
pub fn encode_into_writer<E: Encode, W: enc::Writer, C: config::Config>(
    value: E,
    writer: W,
    config: C,
) -> Result<()> {
    let mut encoder = enc::EncoderImpl::new(writer, config);
    value.encode(&mut encoder)?;
    Ok(())
}

/// Encode a value into a `std::io::Write` using the given configuration.
///
/// Returns the number of bytes written.
///
/// # Examples
///
/// ```
/// use std::io::Cursor;
/// let mut buf = Cursor::new(Vec::new());
/// let written = oxicode::encode_into_std_write(42u8, &mut buf, oxicode::config::standard())
///     .expect("encode");
/// assert_eq!(written, 1);
/// ```
#[cfg(feature = "std")]
pub fn encode_into_std_write<E: Encode, W: std::io::Write, C: config::Config>(
    value: E,
    writer: W,
    config: C,
) -> Result<usize> {
    let io_writer = enc::IoWriter::new(writer);
    let mut encoder = enc::EncoderImpl::new(io_writer, config);
    value.encode(&mut encoder)?;
    Ok(encoder.into_writer().bytes_written())
}

/// Decode a value from an oxicode [`de::Reader`] using the given configuration.
///
/// This is the low-level reader-based API. For byte slices prefer [`decode_from_slice`];
/// for `std::io::Read` prefer [`decode_from_std_read`] or the convenience [`decode_from_reader`].
///
/// # Examples
///
/// ```
/// let bytes = oxicode::encode_to_vec(&42u32).expect("encode");
/// let reader = oxicode::de::SliceReader::new(&bytes);
/// let value: u32 =
///     oxicode::decode_from_de_reader(reader, oxicode::config::standard()).expect("decode");
/// assert_eq!(value, 42u32);
/// ```
pub fn decode_from_de_reader<D: Decode, R: de::Reader, C: config::Config>(
    reader: R,
    config: C,
) -> Result<D> {
    let mut decoder = de::DecoderImpl::new(reader, config);
    D::decode(&mut decoder)
}

/// Decode a value from a `std::io::Read` using the given configuration.
///
/// # Examples
///
/// ```
/// use std::io::Cursor;
/// let bytes = oxicode::encode_to_vec(&42u32).expect("encode");
/// let cursor = Cursor::new(bytes);
/// let value: u32 =
///     oxicode::decode_from_std_read(cursor, oxicode::config::standard()).expect("decode");
/// assert_eq!(value, 42u32);
/// ```
#[cfg(feature = "std")]
pub fn decode_from_std_read<D: Decode, R: std::io::Read, C: config::Config>(
    reader: R,
    config: C,
) -> Result<D> {
    let io_reader = de::IoReader::new(reader);
    let mut decoder = de::DecoderImpl::new(io_reader, config);
    D::decode(&mut decoder)
}

/// Encode `value` to a lowercase hex string using default config.
///
/// The hex string is a compact, human-readable representation of the
/// oxicode-encoded bytes. It can be round-tripped via [`decode_from_hex`].
///
/// # Examples
///
/// ```
/// let hex = oxicode::encode_to_hex(&42u8).expect("encode");
/// assert!(hex.chars().all(|c| c.is_ascii_hexdigit()));
/// let (val, _): (u8, _) = oxicode::decode_from_hex(&hex).expect("decode");
/// assert_eq!(val, 42u8);
/// ```
#[cfg(feature = "std")]
pub fn encode_to_hex<E: Encode>(value: &E) -> Result<String> {
    let bytes = encode_to_vec(value)?;
    Ok(bytes
        .iter()
        .fold(String::with_capacity(bytes.len() * 2), |mut s, b| {
            use std::fmt::Write as _;
            let _ = write!(s, "{:02x}", b);
            s
        }))
}

/// Decode a value from a hex string (lowercase or uppercase) using default config.
///
/// Returns `Err` if the hex string contains non-hex characters, has an odd length,
/// or the decoded bytes cannot be deserialized into `D`.
///
/// # Examples
///
/// ```
/// let hex = oxicode::encode_to_hex(&12345u32).expect("encode");
/// let (val, _): (u32, _) = oxicode::decode_from_hex(&hex).expect("decode");
/// assert_eq!(val, 12345u32);
/// ```
#[cfg(feature = "std")]
pub fn decode_from_hex<D: Decode>(hex: &str) -> Result<(D, usize)> {
    if hex.len() % 2 != 0 {
        return Err(Error::InvalidData {
            message: "hex string has odd length",
        });
    }
    let bytes = (0..hex.len())
        .step_by(2)
        .map(|i| {
            hex.get(i..i + 2)
                .and_then(|s| u8::from_str_radix(s, 16).ok())
        })
        .collect::<Option<Vec<u8>>>()
        .ok_or(Error::InvalidData {
            message: "invalid hex string: contains non-hex characters",
        })?;
    decode_from_slice(&bytes)
}

/// Encode a value to a file using the standard configuration
///
/// Creates or overwrites the file at the given path.
#[cfg(feature = "std")]
pub fn encode_to_file<E: Encode>(value: &E, path: impl AsRef<std::path::Path>) -> Result<()> {
    encode_to_file_with_config(value, path, config::standard())
}

/// Encode a value to a file with a custom configuration
///
/// Creates or overwrites the file at the given path.
#[cfg(feature = "std")]
pub fn encode_to_file_with_config<E: Encode, C: config::Config>(
    value: &E,
    path: impl AsRef<std::path::Path>,
    config: C,
) -> Result<()> {
    let file = std::fs::File::create(path)?;
    let io_writer = enc::IoWriter::new(file);
    let mut encoder = enc::EncoderImpl::new(io_writer, config);
    value.encode(&mut encoder)?;
    Ok(())
}

/// Decode a value from a file using the standard configuration
#[cfg(feature = "std")]
pub fn decode_from_file<D: Decode>(path: impl AsRef<std::path::Path>) -> Result<D> {
    decode_from_file_with_config(path, config::standard())
}

/// Decode a value from a file with a custom configuration
#[cfg(feature = "std")]
pub fn decode_from_file_with_config<D: Decode, C: config::Config>(
    path: impl AsRef<std::path::Path>,
    config: C,
) -> Result<D> {
    let file = std::fs::File::open(path)?;
    decode_from_std_read(file, config)
}

/// Decode a value from a byte slice with custom context
pub fn decode_from_slice_with_context<Ctx, D, C: config::Config>(
    src: &[u8],
    config: C,
    context: Ctx,
) -> Result<(D, usize)>
where
    D: de::Decode<Ctx>,
{
    let reader = de::SliceReader::new(src);
    let mut decoder = de::DecoderImpl::with_context(reader, config, context);
    let result = D::decode(&mut decoder)?;
    let bytes_read = src.len() - decoder.reader().slice.len();
    Ok((result, bytes_read))
}

/// Borrow decode a value from a byte slice (zero-copy) using the standard configuration.
///
/// Unlike [`decode_from_slice`], the returned value may hold references into `src`.
/// Use with types that implement [`BorrowDecode`] (e.g., `&str`, `&[u8]`).
///
/// # Examples
///
/// ```
/// // Encode a String and borrow-decode back as &str (zero-copy)
/// let data = String::from("hello world");
/// let encoded = oxicode::encode_to_vec(&data).expect("encode");
/// let (decoded, _): (&str, _) =
///     oxicode::borrow_decode_from_slice(&encoded).expect("decode");
/// assert_eq!(decoded, data.as_str());
/// ```
pub fn borrow_decode_from_slice<'a, D>(src: &'a [u8]) -> Result<(D, usize)>
where
    D: de::BorrowDecode<'a>,
{
    borrow_decode_from_slice_with_config(src, config::standard())
}

/// Encode a value to a `Vec<u8>` with a CRC32 checksum header appended, using default config.
///
/// The output is a self-describing integrity-checked buffer that embeds a 16-byte header
/// containing a magic marker, format version, payload length, and CRC32 checksum.
/// Use [`decode_from_slice_checked`] to decode and verify.
///
/// This is a thin convenience wrapper around [`checksum::encode_with_checksum`].
///
/// # Examples
///
/// ```
/// #[cfg(feature = "checksum")]
/// {
///     let bytes = oxicode::encode_to_vec_checked(&42u32).expect("encode failed");
///     let (val, _): (u32, _) = oxicode::decode_from_slice_checked(&bytes).expect("decode failed");
///     assert_eq!(val, 42u32);
/// }
/// ```
#[cfg(all(feature = "checksum", feature = "alloc"))]
pub fn encode_to_vec_checked<E: Encode>(value: &E) -> Result<alloc::vec::Vec<u8>> {
    checksum::encode_with_checksum(value)
}

/// Decode a value from bytes that were encoded with [`encode_to_vec_checked`].
///
/// Verifies the embedded CRC32 checksum before decoding.  Returns
/// `Err(Error::ChecksumMismatch { .. })` if the data has been corrupted or tampered with.
/// Returns `(value, bytes_consumed)` on success, where `bytes_consumed` includes the 16-byte
/// integrity header.
///
/// This is a thin convenience wrapper around [`checksum::decode_with_checksum`].
///
/// # Examples
///
/// ```
/// #[cfg(feature = "checksum")]
/// {
///     let encoded = oxicode::encode_to_vec_checked(&99u64).expect("encode failed");
///     let (val, consumed): (u64, _) =
///         oxicode::decode_from_slice_checked(&encoded).expect("decode failed");
///     assert_eq!(val, 99u64);
///     assert_eq!(consumed, encoded.len());
/// }
/// ```
#[cfg(all(feature = "checksum", feature = "alloc"))]
pub fn decode_from_slice_checked<D: Decode>(src: &[u8]) -> Result<(D, usize)> {
    checksum::decode_with_checksum(src)
}

/// Borrow decode a value from a byte slice (zero-copy) with custom configuration
pub fn borrow_decode_from_slice_with_config<'a, D, C: config::Config>(
    src: &'a [u8],
    config: C,
) -> Result<(D, usize)>
where
    D: de::BorrowDecode<'a>,
{
    let reader = de::SliceReader::new(src);
    let mut decoder = de::DecoderImpl::new(reader, config);
    let result = D::borrow_decode(&mut decoder)?;
    let bytes_read = src.len() - decoder.reader().slice.len();
    Ok((result, bytes_read))
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_basic_encoding() {
        // Basic test placeholder
        assert_eq!(2 + 2, 4);
    }
}
