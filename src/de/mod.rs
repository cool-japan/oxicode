//! Decoder-based structs and traits

mod decoder;
mod impl_tuples;
mod impls;

pub mod read;

use crate::{config::Config, error::Error, utils::Sealed};

pub use self::decoder::DecoderImpl;
pub use self::read::{Reader, SliceReader, SliceReaderBorrow};

#[cfg(feature = "std")]
pub use self::read::{BufferedIoReader, IoReader, StdReader};

/// Decode trait for types that can be decoded from binary format
///
/// This trait should be implemented for all types that you want to decode.
/// It can be automatically derived using `#[derive(Decode)]` with the `derive` feature.
///
/// The `Context` type parameter allows passing custom context (such as allocators
/// or shared state) during decoding. Most types use `()` as the context.
///
/// # Example
///
/// ```rust,ignore
/// use oxicode::Decode;
///
/// #[derive(Decode)]
/// struct Point {
///     x: f32,
///     y: f32,
/// }
/// ```
///
/// # Manual Implementation
///
/// ```rust,ignore
/// impl oxicode::Decode for Point {
///     fn decode<D: oxicode::de::Decoder>(
///         decoder: &mut D,
///     ) -> Result<Self, oxicode::Error> {
///         Ok(Point {
///             x: f32::decode(decoder)?,
///             y: f32::decode(decoder)?,
///         })
///     }
/// }
/// ```
///
/// # With Context
///
/// ```rust,ignore
/// struct MyContext {
///     arena: bumpalo::Bump,
/// }
///
/// impl<'a> oxicode::Decode<MyContext> for MyType<'a> {
///     fn decode<D: oxicode::de::Decoder<Context = MyContext>>(
///         decoder: &mut D,
///     ) -> Result<Self, oxicode::Error> {
///         let ctx = decoder.context();
///         // Use ctx.arena for allocations
///         todo!()
///     }
/// }
/// ```
pub trait Decode<Context = ()>: Sized {
    /// Decode a value from the given decoder
    fn decode<D: Decoder<Context = Context>>(decoder: &mut D) -> Result<Self, Error>;
}

/// BorrowDecode trait for zero-copy decoding of borrowed types
///
/// This trait is similar to Decode but allows decoding types that borrow from the input.
/// The lifetime `'de` is the lifetime of the borrowed data.
///
/// The `Context` type parameter allows passing custom context during decoding.
pub trait BorrowDecode<'de, Context = ()>: Sized {
    /// Decode a value from the given decoder, potentially borrowing from the input
    fn borrow_decode<D: BorrowDecoder<'de, Context = Context>>(
        decoder: &mut D,
    ) -> Result<Self, Error>;
}

/// Decoder trait for decoding values with configuration
///
/// This trait is the main interface for decoding values. It provides access to
/// the reader, configuration, and optional context.
pub trait Decoder: Sealed {
    /// The concrete Reader type
    type R: Reader;

    /// The concrete Config type
    type C: Config;

    /// The context type for this decoder
    type Context;

    /// Returns a mutable reference to the reader
    fn reader(&mut self) -> &mut Self::R;

    /// Returns a reference to the configuration
    fn config(&self) -> &Self::C;

    /// Returns a mutable reference to the context
    fn context(&mut self) -> &mut Self::Context;

    /// Claim that `n` bytes are going to be read from the reader.
    /// This is used for memory limit enforcement.
    ///
    /// Returns an error if reading `n` more bytes would exceed the limit.
    #[inline]
    fn claim_bytes_read(&mut self, _n: usize) -> Result<(), Error> {
        // Default implementation does nothing (no limit)
        Ok(())
    }

    /// Unclaim bytes that were previously claimed but not actually read.
    #[inline]
    fn unclaim_bytes_read(&mut self, _n: usize) {
        // Default implementation does nothing
    }

    /// Claim that a container with `len` elements of type `T` is going to be read.
    /// This checks that `len * size_of::<T>()` bytes can be read.
    #[inline]
    fn claim_container_read<T>(&mut self, len: usize) -> Result<(), Error> {
        let size = core::mem::size_of::<T>().saturating_mul(len);
        self.claim_bytes_read(size)
    }
}

/// BorrowDecoder trait for zero-copy decoding
///
/// This extends Decoder with the ability to borrow bytes directly from the input.
pub trait BorrowDecoder<'de>: Decoder {
    /// The concrete BorrowReader type
    type BR: BorrowReader<'de>;

    /// Returns a mutable reference to the borrow reader
    fn borrow_reader(&mut self) -> &mut Self::BR;
}

/// BorrowReader trait for zero-copy reading
///
/// This extends Reader with the ability to take bytes directly from the input
/// without copying.
pub trait BorrowReader<'de>: Reader {
    /// Take `length` bytes from the reader without copying.
    /// Returns a slice that borrows from the original input.
    fn take_bytes(&mut self, length: usize) -> Result<&'de [u8], Error>;

    /// Peek at the next bytes without consuming them.
    #[inline]
    fn peek_read(&self, _n: usize) -> Option<&'de [u8]> {
        None
    }

    /// Consume `n` bytes that were previously peeked.
    #[inline]
    fn consume(&mut self, _n: usize) {
        // Default implementation does nothing
    }
}

impl<T> Decoder for &mut T
where
    T: Decoder,
{
    type R = T::R;
    type C = T::C;
    type Context = T::Context;

    fn reader(&mut self) -> &mut Self::R {
        T::reader(self)
    }

    fn config(&self) -> &Self::C {
        T::config(self)
    }

    fn context(&mut self) -> &mut Self::Context {
        T::context(self)
    }

    fn claim_bytes_read(&mut self, n: usize) -> Result<(), Error> {
        T::claim_bytes_read(self, n)
    }

    fn unclaim_bytes_read(&mut self, n: usize) {
        T::unclaim_bytes_read(self, n)
    }
}

impl<'de, T> BorrowDecoder<'de> for &mut T
where
    T: BorrowDecoder<'de>,
{
    type BR = T::BR;

    fn borrow_reader(&mut self) -> &mut Self::BR {
        T::borrow_reader(self)
    }
}

/// Decode the variant of an option (0 for None, 1 for Some)
#[inline]
#[allow(dead_code)]
pub(crate) fn decode_option_variant<D: Decoder<Context = ()>>(
    decoder: &mut D,
) -> Result<Option<()>, Error> {
    let variant = u8::decode(decoder)?;
    match variant {
        0 => Ok(None),
        1 => Ok(Some(())),
        _v => Err(Error::InvalidData {
            message: "Invalid Option variant",
        }),
    }
}

/// Decode the length of a slice/container
#[inline]
#[allow(dead_code)]
pub(crate) fn decode_slice_len<D: Decoder<Context = ()>>(decoder: &mut D) -> Result<usize, Error> {
    u64::decode(decoder).map(|len| len as usize)
}

/// Helper macro to implement BorrowDecode for types that implement Decode.
/// This is useful for types that don't need to borrow from the input.
#[macro_export]
macro_rules! impl_borrow_decode {
    ($ty:ty) => {
        impl<'de> $crate::de::BorrowDecode<'de> for $ty {
            fn borrow_decode<D: $crate::de::BorrowDecoder<'de, Context = ()>>(
                decoder: &mut D,
            ) -> Result<Self, $crate::error::Error> {
                <$ty as $crate::de::Decode>::decode(decoder)
            }
        }
    };
    ($ty:ty, $ctx:ty) => {
        impl<'de> $crate::de::BorrowDecode<'de, $ctx> for $ty {
            fn borrow_decode<D: $crate::de::BorrowDecoder<'de, Context = $ctx>>(
                decoder: &mut D,
            ) -> Result<Self, $crate::error::Error> {
                <$ty as $crate::de::Decode<$ctx>>::decode(decoder)
            }
        }
    };
}
