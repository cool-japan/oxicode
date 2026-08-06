//! Decoding functionality

use super::BorrowReader;
use crate::error::{Error, Result};

/// Reader trait for decoding values
pub trait Reader {
    /// Read into the provided buffer
    ///
    /// This should completely fill the buffer or return an error
    fn read(&mut self, bytes: &mut [u8]) -> Result<()>;

    /// An exact upper bound on how many bytes this reader can still produce,
    /// if it is cheaply knowable.
    ///
    /// Slice-backed readers know this exactly; streaming readers generally do
    /// not and keep the default `None`. Decoding uses it to reject a forged
    /// length prefix *before* committing an allocation of that size, which is
    /// what stops roughly nine bytes of untrusted input from requesting a
    /// multi-gigabyte buffer.
    ///
    /// Implementations must never return `Some(n)` with `n` smaller than the
    /// number of bytes a subsequent [`read`](Reader::read) would actually
    /// succeed in producing, or valid input will be rejected.
    #[inline]
    fn remaining_bytes(&self) -> Option<usize> {
        None
    }
}

/// Reader implementation that reads from a byte slice
pub struct SliceReader<'a> {
    /// The slice being read from (public for internal use)
    pub(crate) slice: &'a [u8],
}

impl<'a> SliceReader<'a> {
    /// Create a new SliceReader
    pub fn new(slice: &'a [u8]) -> Self {
        Self { slice }
    }

    /// Get the remaining bytes in the slice
    pub fn remaining(&self) -> &'a [u8] {
        self.slice
    }
}

impl<'a> Reader for SliceReader<'a> {
    fn read(&mut self, bytes: &mut [u8]) -> Result<()> {
        let len = bytes.len();
        if self.slice.len() < len {
            return Err(Error::UnexpectedEnd {
                additional: len - self.slice.len(),
            });
        }
        bytes.copy_from_slice(&self.slice[..len]);
        self.slice = &self.slice[len..];
        Ok(())
    }

    #[inline]
    fn remaining_bytes(&self) -> Option<usize> {
        Some(self.slice.len())
    }
}

impl<'a> BorrowReader<'a> for SliceReader<'a> {
    fn take_bytes(&mut self, length: usize) -> Result<&'a [u8]> {
        if self.slice.len() < length {
            return Err(Error::UnexpectedEnd {
                additional: length - self.slice.len(),
            });
        }
        let (bytes, rest) = self.slice.split_at(length);
        self.slice = rest;
        Ok(bytes)
    }

    fn peek_read(&self, n: usize) -> Option<&'a [u8]> {
        if self.slice.len() >= n {
            Some(&self.slice[..n])
        } else {
            None
        }
    }

    fn consume(&mut self, n: usize) {
        if self.slice.len() >= n {
            self.slice = &self.slice[n..];
        }
    }
}

/// Type alias for SliceReader with BorrowReader capability
/// This is exported for use in BorrowDecode implementations
pub type SliceReaderBorrow<'a> = SliceReader<'a>;

/// Reader implementation that wraps a `std::io::Read` implementation
///
/// This allows decoding directly from files, network streams, or any other
/// type that implements `std::io::Read`.
///
/// # Example
///
/// ```rust,ignore
/// use oxicode::de::IoReader;
/// use std::io::Cursor;
///
/// let data = vec![0x42, 0x43];
/// let cursor = Cursor::new(data);
/// let reader = IoReader::new(cursor);
/// // Use reader with decoder...
/// ```
#[cfg(feature = "std")]
pub struct IoReader<R: std::io::Read> {
    reader: R,
    /// Bytes this reader is still allowed to produce, when a budget was set.
    ///
    /// `None` means "unknown / unbounded", which is the only honest answer for
    /// an arbitrary stream. When `Some`, it is decremented by every successful
    /// read and enforced by [`Reader::read`], which is what makes it a sound
    /// value to hand out from [`Reader::remaining_bytes`].
    budget: Option<usize>,
}

#[cfg(feature = "std")]
impl<R: std::io::Read> IoReader<R> {
    /// Create a new IoReader wrapping the given `std::io::Read` implementation
    pub fn new(reader: R) -> Self {
        Self {
            reader,
            budget: None,
        }
    }

    /// Create an IoReader that will never produce more than `max_bytes` bytes.
    ///
    /// Use this whenever the length of the stream is known ahead of time (a
    /// file's `metadata().len()`, an HTTP `Content-Length`, a framed message
    /// size). The budget turns the reader into a bounded source: reads past it
    /// fail with [`Error::UnexpectedEnd`] instead of blocking or consuming the
    /// next message, and — more importantly — [`Reader::remaining_bytes`] can
    /// then report an exact upper bound, so a forged length prefix is rejected
    /// *before* the allocator is asked for that many bytes.
    ///
    /// Without a budget an IO reader must report `None`, and length-prefixed
    /// decoding falls back to materializing the buffer incrementally.
    pub fn with_limit(reader: R, max_bytes: usize) -> Self {
        Self {
            reader,
            budget: Some(max_bytes),
        }
    }

    /// Set (or replace) the byte budget of an already-constructed reader.
    ///
    /// `max_bytes` is the number of bytes the reader may still produce *from
    /// now on*; bytes already consumed are not counted against it. See
    /// [`IoReader::with_limit`] for the rest of the semantics.
    pub fn set_limit(&mut self, max_bytes: usize) {
        self.budget = Some(max_bytes);
    }

    /// Bytes still allowed by the budget, or `None` when no budget was set.
    pub fn remaining_limit(&self) -> Option<usize> {
        self.budget
    }

    /// Get a reference to the underlying reader
    pub fn inner(&self) -> &R {
        &self.reader
    }

    /// Get a mutable reference to the underlying reader
    pub fn inner_mut(&mut self) -> &mut R {
        &mut self.reader
    }

    /// Consume the IoReader and return the underlying reader
    pub fn into_inner(self) -> R {
        self.reader
    }
}

#[cfg(feature = "std")]
impl<R: std::io::Read> Reader for IoReader<R> {
    fn read(&mut self, bytes: &mut [u8]) -> Result<()> {
        if let Some(budget) = self.budget {
            if bytes.len() > budget {
                return Err(Error::UnexpectedEnd {
                    additional: bytes.len() - budget,
                });
            }
        }
        self.reader.read_exact(bytes).map_err(|e| {
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
        if let Some(budget) = self.budget.as_mut() {
            *budget -= bytes.len();
        }
        Ok(())
    }

    #[inline]
    fn remaining_bytes(&self) -> Option<usize> {
        self.budget
    }
}

/// Type alias for StdReader (alternative name for IoReader)
#[cfg(feature = "std")]
pub type StdReader<R> = IoReader<R>;

/// A buffered reader wrapping any `std::io::Read` source.
///
/// Uses `std::io::BufReader` internally to batch syscalls, dramatically
/// improving throughput when decoding from files or network sockets.
#[cfg(feature = "std")]
pub struct BufferedIoReader<R: std::io::Read> {
    inner: std::io::BufReader<R>,
    /// Bytes this reader is still allowed to produce; see
    /// [`BufferedIoReader::with_limit`].
    budget: Option<usize>,
}

#[cfg(feature = "std")]
impl<R: std::io::Read> BufferedIoReader<R> {
    /// Create with default 8 KiB internal buffer.
    pub fn new(reader: R) -> Self {
        Self {
            inner: std::io::BufReader::new(reader),
            budget: None,
        }
    }

    /// Create with a custom buffer capacity in bytes.
    pub fn with_capacity(capacity: usize, reader: R) -> Self {
        Self {
            inner: std::io::BufReader::with_capacity(capacity, reader),
            budget: None,
        }
    }

    /// Create a buffered reader that will never produce more than `max_bytes`
    /// bytes, and reports that budget from [`Reader::remaining_bytes`].
    ///
    /// # Why not the internal buffer length?
    ///
    /// It would be tempting to report `BufReader::buffer().len()` when no
    /// budget is set. That is *unsound* for this trait: `remaining_bytes` is
    /// consumed as an upper bound, and the buffered length is a lower bound —
    /// a 4 KiB buffer in front of a 4 GiB file would cause every legitimate
    /// value longer than the current buffer contents to be rejected with
    /// [`Error::UnexpectedEnd`]. Only a real end-of-stream bound may be
    /// reported here, which is what `max_bytes` supplies.
    pub fn with_limit(reader: R, max_bytes: usize) -> Self {
        Self {
            inner: std::io::BufReader::new(reader),
            budget: Some(max_bytes),
        }
    }

    /// Set (or replace) the byte budget of an already-constructed reader.
    ///
    /// `max_bytes` counts from now on; bytes already consumed do not count
    /// against it.
    pub fn set_limit(&mut self, max_bytes: usize) {
        self.budget = Some(max_bytes);
    }

    /// Bytes still allowed by the budget, or `None` when no budget was set.
    pub fn remaining_limit(&self) -> Option<usize> {
        self.budget
    }

    /// Get a reference to the underlying reader.
    pub fn inner(&self) -> &R {
        self.inner.get_ref()
    }

    /// Get a mutable reference to the underlying reader.
    pub fn inner_mut(&mut self) -> &mut R {
        self.inner.get_mut()
    }

    /// Consume the BufferedIoReader and return the underlying reader.
    pub fn into_inner(self) -> R {
        self.inner.into_inner()
    }
}

#[cfg(feature = "std")]
impl<R: std::io::Read> Reader for BufferedIoReader<R> {
    fn read(&mut self, bytes: &mut [u8]) -> crate::error::Result<()> {
        use std::io::Read as _;
        if let Some(budget) = self.budget {
            if bytes.len() > budget {
                return Err(Error::UnexpectedEnd {
                    additional: bytes.len() - budget,
                });
            }
        }
        self.inner.read_exact(bytes).map_err(|e| {
            // Running out of input is a decode-level condition, not an IO
            // failure; report it with the same typed error the slice and
            // unbuffered IO readers use.
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
        if let Some(budget) = self.budget.as_mut() {
            *budget -= bytes.len();
        }
        Ok(())
    }

    #[inline]
    fn remaining_bytes(&self) -> Option<usize> {
        self.budget
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_slice_reader() {
        let data = [0x42, 0x43, 0x44, 0x45];
        let mut reader = SliceReader::new(&data);

        let mut buf = [0u8; 1];
        reader.read(&mut buf).expect("Failed to read");
        assert_eq!(buf[0], 0x42);
        assert_eq!(reader.remaining().len(), 3);

        let mut buf = [0u8; 2];
        reader.read(&mut buf).expect("Failed to read");
        assert_eq!(buf, [0x43, 0x44]);
        assert_eq!(reader.remaining().len(), 1);

        let mut buf = [0u8; 2];
        assert!(reader.read(&mut buf).is_err()); // Not enough bytes
    }
}
