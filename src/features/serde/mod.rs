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
//!
//! # Unsupported serde attributes
//!
//! oxicode's wire format is compact, binary, and **not self-describing**
//! (field names, variant names, and type tags for `flatten`/`untagged`
//! content are never written). That is a deliberate size/speed trade-off
//! shared with `bincode::serde`, but it means a handful of common serde
//! attributes either fail at runtime instead of at compile time, or silently
//! desynchronize the field stream instead of failing at all:
//!
//! - **`#[serde(flatten)]`** — compiles, but fails at runtime. Flattening
//!   requires buffering the inner map/struct as a self-describing value to
//!   merge its keys with the outer one; oxicode's `deserialize_map`/
//!   `deserialize_any` (the calls serde's flatten machinery relies on) are
//!   not implemented for that purpose and return an error.
//! - **`#[serde(tag = "...")]` / `#[serde(tag = "...", content = "...")]`**
//!   (internally/adjacently tagged enums) and **`#[serde(untagged)]`** —
//!   compile, but fail at runtime for the same reason: both rely on
//!   `deserialize_any` to sniff the shape of the incoming data, which a
//!   non-self-describing format cannot support. Use the default
//!   (externally tagged) enum representation instead.
//! - **`#[serde(skip_serializing_if = "...")]`** — compiles and *appears* to
//!   work, but is a **silent correctness bug**, not a runtime error: the
//!   field is conditionally omitted from the byte stream on encode, but the
//!   generated `Deserialize` impl still unconditionally expects a value in
//!   that position. Every field encoded after the omitted one is read back
//!   from the wrong bytes. Only use this attribute with formats that are
//!   self-describing (JSON, etc.); it is not safe with oxicode.
//! - **`#[serde(deserialize_with = "...")]` combined with `deserialize_any`
//!   or `deserialize_ignored_any`** — anything that ultimately calls
//!   `deserialize_any`/`deserialize_ignored_any` (this includes
//!   `#[serde(other)]` on enums, and `Value`-like "capture anything" types)
//!   fails at runtime for the same non-self-describing reason as `flatten`.
//!
//! Also note: `Serializer::collect_str` is **not** overridden, so any type
//! whose `Serialize` impl calls it (typically via a `Display` bound, e.g.
//! `serde_with`'s `DisplayFromStr` or a hand-written impl doing
//! `serializer.collect_str(&self)`) serializes as a length-prefixed UTF-8
//! string rather than erroring — unlike `bincode::serde`, which explicitly
//! rejects `collect_str` as "not supported". This is usually what you want
//! (it "just works" for `Display`-based types), but it does mean such a
//! type's wire representation is a string, not whatever compact encoding an
//! explicit `Serialize` impl might otherwise choose.
//!
//! None of the above apply to plain structs/enums with normally-derived
//! `Serialize`/`Deserialize` (including `#[serde(rename = ...)]`,
//! `#[serde(default)]` on a *present* field, and standard enum variants) —
//! those work exactly as expected, byte-for-byte compatible with
//! `bincode::serde` (the serializer and both deserializers here all return
//! `false` from `is_human_readable`, matching `bincode`'s non-self-describing
//! wire format instead of serde's `true` default).

mod compat;
mod de;
mod de_borrowed;
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
    value.serialize(serializer).map_err(Error::from)?;
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
    value.serialize(serializer).map_err(Error::from)?;
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
    // Route through the borrowed deserializer so that `&str`/`&[u8]`/`Cow` fields
    // decode zero-copy against `src` (via `visit_borrowed_str`/`visit_borrowed_bytes`).
    // Owned types still decode correctly, falling back to `visit_string`/`visit_byte_buf`.
    let deserializer = de_borrowed::BorrowedDeserializer::new(&mut decoder);
    let value = T::deserialize(deserializer).map_err(Error::from)?;
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
    let value = T::deserialize(deserializer).map_err(Error::from)?;
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
    value.serialize(serializer).map_err(Error::from)?;
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
    let value = T::deserialize(deserializer).map_err(Error::from)?;
    let bytes_read = decoder.reader().inner().bytes_read();
    Ok((value, bytes_read))
}

/// Decode a serde `DeserializeOwned` type from a `std::io::Read` whose total
/// length is known.
///
/// The serde counterpart of [`crate::decode_from_std_read_limited`]. Reading an
/// arbitrary stream, the decoder cannot know how much input is left, so a
/// length-prefixed `String` / `#[serde(with = "serde_bytes")]` field can only be
/// materialized incrementally. Handing it the real length of the payload — a
/// file size, an HTTP `Content-Length`, the frame length of a length-delimited
/// protocol — turns that into an exact bound: a field claiming more bytes than
/// the stream can hold is rejected with [`crate::error::Error::UnexpectedEnd`]
/// *before* an allocation of that size is attempted, and reads past the budget
/// stop at the budget instead of running into the next message.
///
/// Returns the decoded value and the number of bytes consumed.
///
/// A budget larger than the payload never rejects valid input; it just bounds
/// less. A budget smaller than the value being decoded rejects valid input.
///
/// # Example
///
/// ```ignore
/// use std::io::Cursor;
/// let bytes = oxicode::serde::encode_to_vec(&"hi".to_string(), oxicode::config::standard()).unwrap();
/// let len = bytes.len();
/// let (v, n): (String, _) = oxicode::serde::decode_from_std_read_limited(
///     Cursor::new(bytes),
///     oxicode::config::standard(),
///     len,
/// )
/// .unwrap();
/// assert_eq!(v, "hi");
/// assert_eq!(n, len);
/// ```
#[cfg(feature = "std")]
pub fn decode_from_std_read_limited<T, R, C>(
    reader: R,
    config: C,
    max_bytes: usize,
) -> Result<(T, usize), Error>
where
    T: serde::de::DeserializeOwned,
    R: std::io::Read,
    C: Config,
{
    // Wrapping order matters: the counter sits *inside* the budgeted reader, so
    // `bytes_read` still reports what was actually pulled from the caller's
    // stream while `IoReader` enforces (and reports) the budget.
    let counting = CountingReader::new(reader);
    let io_reader = crate::de::IoReader::with_limit(counting, max_bytes);
    let mut decoder = crate::de::DecoderImpl::new(io_reader, config);
    let deserializer = de::Deserializer::new(&mut decoder);
    let value = T::deserialize(deserializer).map_err(Error::from)?;
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
    value.serialize(serializer).map_err(Error::from)?;
    Ok(encoder.into_writer().bytes_written())
}
