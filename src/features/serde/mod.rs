//! Serde compatibility layer
//!
//! This module provides compatibility with types that implement serde's
//! `Serialize` and `Deserialize` traits but not oxicode's native traits.
//!
//! # Example
//!
//! ```ignore
//! use oxicode::serde::{encode_to_vec, decode_from_slice, Compat};
//! use serde::{Serialize, Deserialize};
//!
//! #[derive(Serialize, Deserialize, PartialEq, Debug)]
//! struct Point {
//!     x: f32,
//!     y: f32,
//! }
//!
//! let point = Point { x: 1.0, y: 2.0 };
//! let bytes = encode_to_vec(&point, oxicode::config::standard())?;
//! let (decoded, _) = decode_from_slice::<Point>(&bytes, oxicode::config::standard())?;
//! assert_eq!(point, decoded);
//! ```

mod compat;
mod de;
mod ser;

pub use compat::{BorrowCompat, Compat};

use crate::{config::Config, error::Error};

/// Encode a serde `Serialize` type to a `Vec<u8>`
///
/// # Example
///
/// ```ignore
/// let bytes = oxicode::serde::encode_to_vec(&42u32, oxicode::config::standard()).unwrap();
/// ```
#[cfg(feature = "alloc")]
pub fn encode_to_vec<T, C>(value: &T, config: C) -> Result<alloc::vec::Vec<u8>, Error>
where
    T: serde::Serialize,
    C: Config,
{
    let writer = crate::enc::VecWriter::new();
    let mut encoder = crate::enc::EncoderImpl::new(writer, config);
    let serializer = ser::Serializer::new(&mut encoder);
    value
        .serialize(serializer)
        .map_err(|e| Error::OwnedCustom { message: e.msg })?;
    Ok(encoder.into_writer().into_vec())
}

/// Encode a serde `Serialize` type into a byte slice.
///
/// Returns the number of bytes written into `dst`.
///
/// # Example
///
/// ```ignore
/// let mut buf = [0u8; 64];
/// let n = oxicode::serde::encode_into_slice(&42u32, &mut buf, oxicode::config::standard()).unwrap();
/// ```
pub fn encode_into_slice<T, C>(value: &T, dst: &mut [u8], config: C) -> Result<usize, Error>
where
    T: serde::Serialize,
    C: Config,
{
    let writer = crate::enc::SliceWriter::new(dst);
    let mut encoder = crate::enc::EncoderImpl::new(writer, config);
    let serializer = ser::Serializer::new(&mut encoder);
    value
        .serialize(serializer)
        .map_err(|e| Error::OwnedCustom { message: e.msg })?;
    Ok(encoder.into_writer().bytes_written())
}

/// Decode a serde `Deserialize` type from a byte slice (supports borrowed data).
///
/// Returns the decoded value and the number of bytes consumed.
///
/// # Example
///
/// ```ignore
/// let bytes = oxicode::serde::encode_to_vec(&"hello", oxicode::config::standard()).unwrap();
/// let (s, _): (&str, _) = oxicode::serde::decode_from_slice(&bytes, oxicode::config::standard()).unwrap();
/// ```
pub fn decode_from_slice<'a, T, C>(src: &'a [u8], config: C) -> Result<(T, usize), Error>
where
    T: serde::Deserialize<'a>,
    C: Config,
{
    let reader = crate::de::SliceReader::new(src);
    let mut decoder = crate::de::DecoderImpl::new(reader, config);
    let deserializer = de::Deserializer::new(&mut decoder);
    let value = T::deserialize(deserializer).map_err(|e| Error::OwnedCustom { message: e.msg })?;
    let bytes_read = src.len() - decoder.reader().slice.len();
    Ok((value, bytes_read))
}

/// Decode an owned serde `DeserializeOwned` type from a byte slice.
///
/// Returns the decoded value and the number of bytes consumed.
///
/// # Example
///
/// ```ignore
/// let bytes = oxicode::serde::encode_to_vec(&42u32, oxicode::config::standard()).unwrap();
/// let (v, _): (u32, _) = oxicode::serde::decode_owned_from_slice(&bytes, oxicode::config::standard()).unwrap();
/// assert_eq!(v, 42u32);
/// ```
pub fn decode_owned_from_slice<T, C>(src: &[u8], config: C) -> Result<(T, usize), Error>
where
    T: serde::de::DeserializeOwned,
    C: Config,
{
    let reader = crate::de::SliceReader::new(src);
    let mut decoder = crate::de::DecoderImpl::new(reader, config);
    let deserializer = de::Deserializer::new(&mut decoder);
    let value = T::deserialize(deserializer).map_err(|e| Error::OwnedCustom { message: e.msg })?;
    let bytes_read = src.len() - decoder.reader().slice.len();
    Ok((value, bytes_read))
}

/// Encode a serde `Serialize` type into a `std::io::Write`.
///
/// Returns the number of bytes written.
///
/// # Example
///
/// ```ignore
/// use std::io::Cursor;
/// let mut cursor = Cursor::new(Vec::new());
/// let n = oxicode::serde::encode_into_std_write(&42u32, &mut cursor, oxicode::config::standard()).unwrap();
/// assert!(n > 0);
/// ```
#[cfg(feature = "std")]
pub fn encode_into_std_write<T, W, C>(value: &T, writer: W, config: C) -> Result<usize, Error>
where
    T: serde::Serialize,
    W: std::io::Write,
    C: Config,
{
    let io_writer = crate::enc::IoWriter::new(writer);
    let mut encoder = crate::enc::EncoderImpl::new(io_writer, config);
    let serializer = ser::Serializer::new(&mut encoder);
    value
        .serialize(serializer)
        .map_err(|e| Error::OwnedCustom { message: e.msg })?;
    Ok(encoder.into_writer().bytes_written())
}

/// A `std::io::Read` wrapper that counts bytes consumed.
///
/// Used internally by `decode_from_std_read` to track the number of bytes read.
#[cfg(feature = "std")]
struct CountingReader<R: std::io::Read> {
    inner: R,
    bytes_read: usize,
}

#[cfg(feature = "std")]
impl<R: std::io::Read> CountingReader<R> {
    fn new(inner: R) -> Self {
        Self {
            inner,
            bytes_read: 0,
        }
    }

    fn bytes_read(&self) -> usize {
        self.bytes_read
    }
}

#[cfg(feature = "std")]
impl<R: std::io::Read> std::io::Read for CountingReader<R> {
    fn read(&mut self, buf: &mut [u8]) -> std::io::Result<usize> {
        let n = self.inner.read(buf)?;
        self.bytes_read += n;
        Ok(n)
    }
}

/// Decode a serde `DeserializeOwned` type from a `std::io::Read`.
///
/// Returns the decoded value and the number of bytes consumed.
///
/// # Example
///
/// ```ignore
/// use std::io::Cursor;
/// let bytes = oxicode::serde::encode_to_vec(&42u32, oxicode::config::standard()).unwrap();
/// let cursor = Cursor::new(bytes);
/// let (v, n): (u32, _) = oxicode::serde::decode_from_std_read(cursor, oxicode::config::standard()).unwrap();
/// assert_eq!(v, 42u32);
/// assert!(n > 0);
/// ```
#[cfg(feature = "std")]
pub fn decode_from_std_read<T, R, C>(reader: R, config: C) -> Result<(T, usize), Error>
where
    T: serde::de::DeserializeOwned,
    R: std::io::Read,
    C: Config,
{
    let counting = CountingReader::new(reader);
    let io_reader = crate::de::IoReader::new(counting);
    let mut decoder = crate::de::DecoderImpl::new(io_reader, config);
    let deserializer = de::Deserializer::new(&mut decoder);
    let value = T::deserialize(deserializer).map_err(|e| Error::OwnedCustom { message: e.msg })?;
    let bytes_read = decoder.reader().inner().bytes_read();
    Ok((value, bytes_read))
}

/// Convenience: encode a serde `Serialize` type to `Vec<u8>` using the standard configuration.
///
/// Equivalent to `encode_to_vec(value, oxicode::config::standard())`.
///
/// # Example
///
/// ```ignore
/// let bytes = oxicode::serde::encode_serde(&42u32).unwrap();
/// ```
#[cfg(feature = "alloc")]
pub fn encode_serde<T>(value: &T) -> Result<alloc::vec::Vec<u8>, Error>
where
    T: serde::Serialize,
{
    encode_to_vec(value, crate::config::standard())
}

/// Convenience: decode a serde `DeserializeOwned` type from a byte slice using the standard configuration.
///
/// Returns only the value, discarding the bytes-consumed count.
///
/// # Example
///
/// ```ignore
/// let bytes = oxicode::serde::encode_serde(&42u32).unwrap();
/// let v: u32 = oxicode::serde::decode_serde(&bytes).unwrap();
/// assert_eq!(v, 42);
/// ```
#[cfg(feature = "alloc")]
pub fn decode_serde<T>(bytes: &[u8]) -> Result<T, Error>
where
    T: serde::de::DeserializeOwned,
{
    decode_owned_from_slice(bytes, crate::config::standard()).map(|(v, _)| v)
}

/// Encode a serde `Serialize` type to `Vec<u8>` with a custom configuration.
///
/// # Example
///
/// ```ignore
/// let cfg = oxicode::config::standard().with_fixed_int_encoding();
/// let bytes = oxicode::serde::encode_serde_with_config(&42u32, cfg).unwrap();
/// ```
#[cfg(feature = "alloc")]
pub fn encode_serde_with_config<T, C>(value: &T, config: C) -> Result<alloc::vec::Vec<u8>, Error>
where
    T: serde::Serialize,
    C: Config,
{
    encode_to_vec(value, config)
}

/// Decode a serde `DeserializeOwned` type from a byte slice with a custom configuration.
///
/// Returns only the value, discarding the bytes-consumed count.
///
/// # Example
///
/// ```ignore
/// let cfg = oxicode::config::standard();
/// let bytes = oxicode::serde::encode_serde_with_config(&42u32, cfg).unwrap();
/// let v: u32 = oxicode::serde::decode_serde_with_config(&bytes, cfg).unwrap();
/// assert_eq!(v, 42);
/// ```
#[cfg(feature = "alloc")]
pub fn decode_serde_with_config<T, C>(bytes: &[u8], config: C) -> Result<T, Error>
where
    T: serde::de::DeserializeOwned,
    C: Config,
{
    decode_owned_from_slice(bytes, config).map(|(v, _)| v)
}

/// Encode a serde `Serialize` type to a file using the standard configuration.
///
/// Creates or overwrites the file at the given path.
///
/// # Example
///
/// ```ignore
/// oxicode::serde::encode_serde_to_file(&42u32, "/tmp/test.bin").unwrap();
/// ```
#[cfg(feature = "std")]
pub fn encode_serde_to_file<T>(value: &T, path: impl AsRef<std::path::Path>) -> Result<(), Error>
where
    T: serde::Serialize,
{
    let file = std::fs::File::create(path)?;
    encode_into_std_write(value, file, crate::config::standard())?;
    Ok(())
}

/// Decode a serde `DeserializeOwned` type from a file using the standard configuration.
///
/// # Example
///
/// ```ignore
/// let v: u32 = oxicode::serde::decode_serde_from_file("/tmp/test.bin").unwrap();
/// ```
#[cfg(feature = "std")]
pub fn decode_serde_from_file<T>(path: impl AsRef<std::path::Path>) -> Result<T, Error>
where
    T: serde::de::DeserializeOwned,
{
    let file = std::fs::File::open(path)?;
    decode_from_std_read(file, crate::config::standard()).map(|(v, _)| v)
}

/// Compute the encoded size of a serde `Serialize` type without allocating.
///
/// Uses a `SizeWriter` internally to count bytes without writing to any buffer.
///
/// # Example
///
/// ```ignore
/// let size = oxicode::serde::encoded_serde_size(&42u32).unwrap();
/// assert!(size > 0);
/// ```
pub fn encoded_serde_size<T>(value: &T) -> Result<usize, Error>
where
    T: serde::Serialize,
{
    let writer = crate::enc::SizeWriter::new();
    let mut encoder = crate::enc::EncoderImpl::new(writer, crate::config::standard());
    let serializer = ser::Serializer::new(&mut encoder);
    value
        .serialize(serializer)
        .map_err(|e| Error::OwnedCustom { message: e.msg })?;
    Ok(encoder.into_writer().bytes_written())
}
