//! Encoder-based structs and traits

mod encoder;
mod impl_tuples;
mod impls;

pub mod write;

use crate::{config::Config, error::Error, utils::Sealed};

pub use self::encoder::EncoderImpl;
pub use self::write::{SizeWriter, SliceWriter, Writer};

#[cfg(feature = "alloc")]
pub use self::write::VecWriter;

#[cfg(feature = "std")]
pub use self::write::{IoWriter, StdWriter};

/// Encode trait for types that can be encoded to binary format
///
/// This trait should be implemented for all types that you want to encode.
/// It can be automatically derived using `#[derive(Encode)]` with the `derive` feature.
///
/// # Example
///
/// ```rust,ignore
/// use oxicode::Encode;
///
/// #[derive(Encode)]
/// struct Point {
///     x: f32,
///     y: f32,
/// }
/// ```
///
/// # Manual Implementation
///
/// ```rust,ignore
/// impl oxicode::Encode for Point {
///     fn encode<E: oxicode::enc::Encoder>(
///         &self,
///         encoder: &mut E,
///     ) -> Result<(), oxicode::Error> {
///         self.x.encode(encoder)?;
///         self.y.encode(encoder)?;
///         Ok(())
///     }
/// }
/// ```
pub trait Encode {
    /// Encode this value into the given encoder
    fn encode<E: Encoder>(&self, encoder: &mut E) -> Result<(), Error>;
}

/// Encoder trait for encoding values with configuration
///
/// This trait is the main interface for encoding values. It provides access to
/// both the writer and the configuration.
pub trait Encoder: Sealed {
    /// The concrete Writer type
    type W: Writer;

    /// The concrete Config type
    type C: Config;

    /// Returns a mutable reference to the writer
    fn writer(&mut self) -> &mut Self::W;

    /// Returns a reference to the configuration
    fn config(&self) -> &Self::C;
}

impl<T> Encoder for &mut T
where
    T: Encoder,
{
    type W = T::W;
    type C = T::C;

    fn writer(&mut self) -> &mut Self::W {
        T::writer(self)
    }

    fn config(&self) -> &Self::C {
        T::config(self)
    }
}

/// Encode the variant of the given option (0 for None, 1 for Some)
#[inline]
#[allow(dead_code)]
pub(crate) fn encode_option_variant<E: Encoder, T>(
    encoder: &mut E,
    value: &Option<T>,
) -> Result<(), Error> {
    match value {
        None => 0u8.encode(encoder),
        Some(_) => 1u8.encode(encoder),
    }
}

/// Encode the length of a slice/container
#[inline]
#[allow(dead_code)]
pub(crate) fn encode_slice_len<E: Encoder>(encoder: &mut E, len: usize) -> Result<(), Error> {
    (len as u64).encode(encoder)
}
