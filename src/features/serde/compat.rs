//! Compatibility wrappers for serde types

use crate::{
    de::{BorrowDecode, BorrowDecoder, Decode, Decoder},
    enc::{Encode, Encoder},
    error::Error,
};

/// Wrapper for types that implement serde's Serialize/Deserialize
///
/// This allows encoding/decoding types that don't implement oxicode's
/// native Encode/Decode traits.
///
/// # Example
///
/// ```ignore
/// use oxicode::serde::Compat;
/// use serde::{Serialize, Deserialize};
///
/// #[derive(Serialize, Deserialize)]
/// struct External {
///     // Some third-party type
/// }
///
/// // Wrap it to use with oxicode
/// let wrapped = Compat(external);
/// let bytes = oxicode::encode_to_vec(&wrapped)?;
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Compat<T>(pub T);

impl<T> Encode for Compat<T>
where
    T: serde::Serialize,
{
    fn encode<E: Encoder>(&self, encoder: &mut E) -> Result<(), Error> {
        let serializer = super::ser::Serializer::new(encoder);
        self.0
            .serialize(serializer)
            .map_err(|e| Error::OwnedCustom { message: e.msg })
    }
}

impl<T> Decode for Compat<T>
where
    T: serde::de::DeserializeOwned,
{
    fn decode<D: Decoder<Context = ()>>(decoder: &mut D) -> Result<Self, Error> {
        let deserializer = super::de::Deserializer::new(decoder);
        T::deserialize(deserializer)
            .map(Compat)
            .map_err(|e| Error::OwnedCustom { message: e.msg })
    }
}

/// Wrapper for borrowed types that implement serde's Deserialize
///
/// This allows zero-copy decoding of serde types.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct BorrowCompat<T>(pub T);

impl<T> Encode for BorrowCompat<T>
where
    T: serde::Serialize,
{
    fn encode<E: Encoder>(&self, encoder: &mut E) -> Result<(), Error> {
        let serializer = super::ser::Serializer::new(encoder);
        self.0
            .serialize(serializer)
            .map_err(|e| Error::OwnedCustom { message: e.msg })
    }
}

impl<'de, T> BorrowDecode<'de> for BorrowCompat<T>
where
    T: serde::Deserialize<'de>,
{
    fn borrow_decode<D: BorrowDecoder<'de, Context = ()>>(decoder: &mut D) -> Result<Self, Error> {
        let deserializer = super::de::Deserializer::new(decoder);
        T::deserialize(deserializer)
            .map(BorrowCompat)
            .map_err(|e| Error::OwnedCustom { message: e.msg })
    }
}
