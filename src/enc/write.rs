//! Encoding functionality

use crate::error::Result;

#[cfg(feature = "alloc")]
use alloc::vec::Vec;

/// Writer trait for encoding values
pub trait Writer {
    /// Write bytes to the writer
    ///
    /// This should write all bytes or return an error
    fn write(&mut self, bytes: &[u8]) -> Result<()>;
}

/// Writer implementation that writes to a `Vec<u8>`
#[cfg(feature = "alloc")]
pub struct VecWriter {
    buffer: Vec<u8>,
}

#[cfg(feature = "alloc")]
impl VecWriter {
    /// Create a new VecWriter
    pub fn new() -> Self {
        Self { buffer: Vec::new() }
    }

    /// Create a new VecWriter with the given capacity
    pub fn with_capacity(capacity: usize) -> Self {
        Self {
            buffer: Vec::with_capacity(capacity),
        }
    }

    /// Get the inner `Vec<u8>`
    pub fn into_vec(self) -> Vec<u8> {
        self.buffer
    }

    /// Get a reference to the inner buffer
    pub fn as_slice(&self) -> &[u8] {
        &self.buffer
    }
}

#[cfg(feature = "alloc")]
impl Default for VecWriter {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(feature = "alloc")]
impl Writer for VecWriter {
    fn write(&mut self, bytes: &[u8]) -> Result<()> {
        self.buffer.extend_from_slice(bytes);
        Ok(())
    }
}

/// Writer implementation that writes to a byte slice
pub struct SliceWriter<'a> {
    slice: &'a mut [u8],
    index: usize,
}

impl<'a> SliceWriter<'a> {
    /// Create a new SliceWriter
    pub fn new(slice: &'a mut [u8]) -> Self {
        Self { slice, index: 0 }
    }

    /// Get the number of bytes written
    pub fn bytes_written(&self) -> usize {
        self.index
    }
}

impl<'a> Writer for SliceWriter<'a> {
    fn write(&mut self, bytes: &[u8]) -> Result<()> {
        let len = bytes.len();
        if self.index + len > self.slice.len() {
            return Err(crate::error::Error::UnexpectedEnd { additional: len });
        }
        self.slice[self.index..self.index + len].copy_from_slice(bytes);
        self.index += len;
        Ok(())
    }
}

/// Writer implementation that wraps a `std::io::Write` implementation
///
/// This allows encoding directly to files, network streams, or any other
/// type that implements `std::io::Write`.
///
/// # Example
///
/// ```rust,ignore
/// use oxicode::enc::IoWriter;
/// use std::io::Cursor;
///
/// let mut buffer = Cursor::new(Vec::new());
/// let writer = IoWriter::new(&mut buffer);
/// // Use writer with encoder...
/// ```
#[cfg(feature = "std")]
pub struct IoWriter<W: std::io::Write> {
    writer: W,
    bytes_written: usize,
}

#[cfg(feature = "std")]
impl<W: std::io::Write> IoWriter<W> {
    /// Create a new IoWriter wrapping the given `std::io::Write` implementation
    pub fn new(writer: W) -> Self {
        Self {
            writer,
            bytes_written: 0,
        }
    }

    /// Get the number of bytes written
    pub fn bytes_written(&self) -> usize {
        self.bytes_written
    }

    /// Get a reference to the underlying writer
    pub fn inner(&self) -> &W {
        &self.writer
    }

    /// Get a mutable reference to the underlying writer
    pub fn inner_mut(&mut self) -> &mut W {
        &mut self.writer
    }

    /// Consume the IoWriter and return the underlying writer
    pub fn into_inner(self) -> W {
        self.writer
    }
}

#[cfg(feature = "std")]
impl<W: std::io::Write> Writer for IoWriter<W> {
    fn write(&mut self, bytes: &[u8]) -> Result<()> {
        self.writer
            .write_all(bytes)
            .map_err(|e| crate::error::Error::Io {
                kind: e.kind(),
                message: e.to_string(),
            })?;
        self.bytes_written += bytes.len();
        Ok(())
    }
}

/// Type alias for StdWriter (alternative name for IoWriter)
#[cfg(feature = "std")]
pub type StdWriter<W> = IoWriter<W>;

/// Writer implementation that counts bytes without writing them.
///
/// Useful for calculating the encoded size of a value without
/// actually performing the encoding to a buffer.
pub struct SizeWriter {
    bytes_written: usize,
}

impl SizeWriter {
    /// Create a new SizeWriter.
    pub fn new() -> Self {
        Self { bytes_written: 0 }
    }

    /// Get the number of bytes that would be written.
    pub fn bytes_written(&self) -> usize {
        self.bytes_written
    }
}

impl Default for SizeWriter {
    fn default() -> Self {
        Self::new()
    }
}

impl Writer for SizeWriter {
    fn write(&mut self, bytes: &[u8]) -> Result<()> {
        self.bytes_written += bytes.len();
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[cfg(feature = "alloc")]
    fn test_vec_writer() {
        let mut writer = VecWriter::new();
        writer.write(&[0x42]).expect("Failed to write");
        assert_eq!(writer.as_slice(), &[0x42]);

        writer.write(&[0x43, 0x44]).expect("Failed to write");
        assert_eq!(writer.as_slice(), &[0x42, 0x43, 0x44]);
    }

    #[test]
    fn test_size_writer() {
        let mut writer = SizeWriter::new();
        writer.write(&[0x42]).expect("Failed to write");
        assert_eq!(writer.bytes_written(), 1);

        writer.write(&[0x43, 0x44]).expect("Failed to write");
        assert_eq!(writer.bytes_written(), 3);

        writer.write(&[0xFF; 100]).expect("Failed to write");
        assert_eq!(writer.bytes_written(), 103);
    }

    #[test]
    fn test_size_writer_default() {
        let writer = SizeWriter::default();
        assert_eq!(writer.bytes_written(), 0);
    }

    #[test]
    fn test_slice_writer() {
        let mut buffer = [0u8; 10];
        {
            let mut writer = SliceWriter::new(&mut buffer);

            writer.write(&[0x42]).expect("Failed to write");
            assert_eq!(writer.bytes_written(), 1);

            writer.write(&[0x43, 0x44]).expect("Failed to write");
            assert_eq!(writer.bytes_written(), 3);

            // Fill the rest
            writer.write(&[0xFF; 7]).expect("Failed to write remaining");
            assert_eq!(writer.bytes_written(), 10);

            // Should fail - buffer full
            assert!(writer.write(&[0x00]).is_err());
        }

        // Check the buffer after writer is dropped
        assert_eq!(buffer[0], 0x42);
        assert_eq!(&buffer[..3], &[0x42, 0x43, 0x44]);
    }
}
