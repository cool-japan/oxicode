//! Decoder implementation

use super::{BorrowDecoder, BorrowReader, Decoder, Reader};
use crate::{config::Config, error::Error, utils::Sealed};

/// A Decoder that reads bytes from a given reader `R`.
///
/// This struct should rarely be used directly.
/// In most cases, prefer the `decode_*` functions in the crate root.
///
/// # Example
///
/// ```rust,ignore
/// use oxicode::de::{DecoderImpl, Decode, SliceReader};
///
/// let data = [0x05, 0x00, 0x00, 0x00];
/// let config = oxicode::config::legacy().with_little_endian();
///
/// let mut decoder = DecoderImpl::new(SliceReader::new(&data), config);
/// let value: u32 = Decode::decode(&mut decoder).unwrap();
/// assert_eq!(value, 5);
/// ```
///
/// # With Context
///
/// ```rust,ignore
/// use oxicode::de::{DecoderImpl, Decode, SliceReader};
///
/// struct MyContext { /* custom allocator, etc */ }
///
/// let data = [0x05];
/// let config = oxicode::config::standard();
/// let ctx = MyContext { };
///
/// let mut decoder = DecoderImpl::with_context(SliceReader::new(&data), config, ctx);
/// // Use decoder.context() to access MyContext
/// ```
pub struct DecoderImpl<R: Reader, C: Config, Ctx = ()> {
    reader: R,
    config: C,
    context: Ctx,
    /// Tracks how many bytes have been claimed so far (for limit enforcement).
    bytes_claimed: usize,
}

impl<R: Reader, C: Config> DecoderImpl<R, C, ()> {
    /// Create a new Decoder with unit context
    pub fn new(reader: R, config: C) -> Self {
        Self {
            reader,
            config,
            context: (),
            bytes_claimed: 0,
        }
    }
}

impl<R: Reader, C: Config, Ctx> DecoderImpl<R, C, Ctx> {
    /// Create a new Decoder with custom context
    pub fn with_context(reader: R, config: C, context: Ctx) -> Self {
        Self {
            reader,
            config,
            context,
            bytes_claimed: 0,
        }
    }

    /// Return the underlying reader
    #[inline]
    pub fn into_reader(self) -> R {
        self.reader
    }

    /// Get a reference to the underlying reader
    #[inline]
    pub fn reader(&mut self) -> &mut R {
        &mut self.reader
    }

    /// Get a reference to the context
    #[inline]
    pub fn get_context(&self) -> &Ctx {
        &self.context
    }

    /// Get a mutable reference to the context
    #[inline]
    pub fn get_context_mut(&mut self) -> &mut Ctx {
        &mut self.context
    }
}

impl<R: Reader, C: Config, Ctx> Decoder for DecoderImpl<R, C, Ctx> {
    type R = R;
    type C = C;
    type Context = Ctx;

    #[inline]
    fn reader(&mut self) -> &mut Self::R {
        &mut self.reader
    }

    #[inline]
    fn config(&self) -> &Self::C {
        &self.config
    }

    #[inline]
    fn context(&mut self) -> &mut Self::Context {
        &mut self.context
    }

    #[inline]
    fn claim_bytes_read(&mut self, n: usize) -> Result<(), Error> {
        if let Some(limit) = self.config.limit() {
            let new_total = self.bytes_claimed.saturating_add(n);
            if new_total > limit {
                return Err(Error::LimitExceeded {
                    limit: limit as u64,
                    found: new_total as u64,
                });
            }
            self.bytes_claimed = new_total;
        }
        Ok(())
    }

    #[inline]
    fn unclaim_bytes_read(&mut self, n: usize) {
        self.bytes_claimed = self.bytes_claimed.saturating_sub(n);
    }
}

impl<R: Reader, C: Config, Ctx> Sealed for DecoderImpl<R, C, Ctx> {}

/// BorrowDecoder implementation for DecoderImpl with BorrowReader
impl<'de, R, C: Config, Ctx> BorrowDecoder<'de> for DecoderImpl<R, C, Ctx>
where
    R: BorrowReader<'de>,
{
    type BR = R;

    #[inline]
    fn borrow_reader(&mut self) -> &mut Self::BR {
        &mut self.reader
    }
}
