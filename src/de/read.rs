//! Decoding functionality

use super::BorrowReader;
use crate::error::{Error, Result};

/// Reader trait for decoding values
pub trait Reader {
    /// Read into the provided buffer
    ///
    /// This should completely fill the buffer or return an error
    fn read(&mut self, bytes: &mut [u8]) -> Result<()>;
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
}

#[cfg(feature = "std")]
impl<R: std::io::Read> IoReader<R> {
    /// Create a new IoReader wrapping the given `std::io::Read` implementation
    pub fn new(reader: R) -> Self {
        Self { reader }
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
        })
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
}

#[cfg(feature = "std")]
impl<R: std::io::Read> BufferedIoReader<R> {
    /// Create with default 8 KiB internal buffer.
    pub fn new(reader: R) -> Self {
        Self {
            inner: std::io::BufReader::new(reader),
        }
    }

    /// Create with a custom buffer capacity in bytes.
    pub fn with_capacity(capacity: usize, reader: R) -> Self {
        Self {
            inner: std::io::BufReader::with_capacity(capacity, reader),
        }
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
        self.inner
            .read_exact(bytes)
            .map_err(|e| crate::error::Error::Io {
                kind: e.kind(),
                message: e.to_string(),
            })
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
