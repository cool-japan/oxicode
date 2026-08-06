//! Serde Deserializer implementation

use crate::de::Decoder;
use serde::de;

/// Error type for serde deserialization
///
/// This wraps the underlying oxicode [`crate::error::Error`] without loss so that
/// callers can still recover the concrete failure cause (for example
/// [`crate::error::Error::UnexpectedEnd`] for truncated input) after decoding
/// through the serde bridge. Genuine serde `custom` messages that have no
/// oxicode counterpart are carried as [`DeError::Custom`].
#[derive(Debug)]
pub enum DeError {
    /// The underlying oxicode error, preserved verbatim.
    Codec(crate::error::Error),
    /// A serde-originated message with no underlying oxicode error.
    Custom(alloc::string::String),
}

impl DeError {
    /// Create a DeError from a static string slice.
    pub(crate) fn from_static(msg: &'static str) -> Self {
        DeError::Custom(alloc::string::String::from(msg))
    }
}

impl de::Error for DeError {
    fn custom<T: core::fmt::Display>(msg: T) -> Self {
        DeError::Custom(alloc::format!("{}", msg))
    }
}

impl core::fmt::Display for DeError {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            DeError::Codec(err) => write!(f, "{}", err),
            DeError::Custom(msg) => write!(f, "{}", msg),
        }
    }
}

impl core::error::Error for DeError {}

impl From<DeError> for crate::error::Error {
    fn from(err: DeError) -> Self {
        match err {
            DeError::Codec(inner) => inner,
            DeError::Custom(message) => crate::error::Error::OwnedCustom { message },
        }
    }
}

/// Run `f` inside one level of the decoder's recursion-depth guard.
///
/// The serde bridge descends through visitor callbacks rather than through
/// `Decode` impls, so it never passed through
/// [`crate::de::decode_with_depth_guard`]. Without this guard a crafted
/// deeply-nested payload (for example `enum Tree { Leaf, Node(Box<Tree>) }`)
/// recurses roughly one stack frame per input byte and aborts the process with
/// an uncatchable stack overflow. Calls [`crate::de::Decoder::enter_recursion`]
/// before `f` and [`crate::de::Decoder::leave_recursion`] afterwards regardless
/// of the outcome, so the depth counter stays balanced on the error path.
///
/// Shared with the borrowed deserializer in [`super::de_borrowed`].
#[inline]
pub(crate) fn with_recursion_guard<D, T, F>(decoder: &mut D, f: F) -> Result<T, DeError>
where
    D: Decoder,
    F: FnOnce(&mut D) -> Result<T, DeError>,
{
    decoder.enter_recursion().map_err(DeError::Codec)?;
    let result = f(decoder);
    decoder.leave_recursion();
    result
}

/// Decode an attacker-controlled `u64` length prefix into a `usize`.
///
/// The wire format stores collection lengths as `u64`. A truncating `as usize`
/// cast (a) silently mangles the length on targets whose `usize` is narrower
/// than 64 bits and (b) can produce the `usize::MAX` value that the compound
/// deserializers previously used as an "unknown length" sentinel. Reject
/// out-of-range values with [`crate::error::Error::OutsideUsizeRange`] instead,
/// matching the native path's [`crate::de::decode_slice_len`].
#[inline]
pub(crate) fn decode_length<D: Decoder<Context = ()>>(decoder: &mut D) -> Result<usize, DeError> {
    use crate::de::Decode;
    let len = u64::decode(decoder).map_err(DeError::Codec)?;
    usize::try_from(len).map_err(|_| DeError::Codec(crate::error::Error::OutsideUsizeRange(len)))
}

/// Reserve budget for a container of `len` elements before it is decoded.
///
/// Every non-degenerate element occupies at least one byte on the wire, so a
/// container claims `len * per_element` bytes up front. Under a
/// `config.with_limit::<N>()` decoder this rejects a forged length immediately
/// instead of only discovering it incrementally (and never at all for elements
/// that claim zero bytes). Each element then [`unclaim`]s its share before it
/// is decoded, so the claim converges on the real byte count exactly the way
/// `Vec<T>::decode` does.
///
/// [`unclaim`]: crate::de::Decoder::unclaim_bytes_read
#[inline]
pub(crate) fn claim_container<D: Decoder<Context = ()>>(
    decoder: &mut D,
    len: usize,
    per_element: usize,
) -> Result<(), DeError> {
    decoder
        .claim_bytes_read(len.saturating_mul(per_element))
        .map_err(DeError::Codec)
}

/// Serde deserializer that wraps an oxicode Decoder
pub struct Deserializer<'a, D: Decoder> {
    decoder: &'a mut D,
}

impl<'a, D: Decoder<Context = ()>> Deserializer<'a, D> {
    /// Create a new Deserializer wrapping the given Decoder
    pub fn new(decoder: &'a mut D) -> Self {
        Self { decoder }
    }
}

impl<'de, 'a, D: Decoder<Context = ()>> de::Deserializer<'de> for Deserializer<'a, D> {
    type Error = DeError;

    fn deserialize_any<V: de::Visitor<'de>>(self, _visitor: V) -> Result<V::Value, Self::Error> {
        Err(DeError::from_static("deserialize_any not supported"))
    }

    fn deserialize_bool<V: de::Visitor<'de>>(self, visitor: V) -> Result<V::Value, Self::Error> {
        use crate::de::Decode;
        let value = bool::decode(self.decoder).map_err(DeError::Codec)?;
        visitor.visit_bool(value)
    }

    fn deserialize_i8<V: de::Visitor<'de>>(self, visitor: V) -> Result<V::Value, Self::Error> {
        use crate::de::Decode;
        let value = i8::decode(self.decoder).map_err(DeError::Codec)?;
        visitor.visit_i8(value)
    }

    fn deserialize_i16<V: de::Visitor<'de>>(self, visitor: V) -> Result<V::Value, Self::Error> {
        use crate::de::Decode;
        let value = i16::decode(self.decoder).map_err(DeError::Codec)?;
        visitor.visit_i16(value)
    }

    fn deserialize_i32<V: de::Visitor<'de>>(self, visitor: V) -> Result<V::Value, Self::Error> {
        use crate::de::Decode;
        let value = i32::decode(self.decoder).map_err(DeError::Codec)?;
        visitor.visit_i32(value)
    }

    fn deserialize_i64<V: de::Visitor<'de>>(self, visitor: V) -> Result<V::Value, Self::Error> {
        use crate::de::Decode;
        let value = i64::decode(self.decoder).map_err(DeError::Codec)?;
        visitor.visit_i64(value)
    }

    fn deserialize_i128<V: de::Visitor<'de>>(self, visitor: V) -> Result<V::Value, Self::Error> {
        use crate::de::Decode;
        let value = i128::decode(self.decoder).map_err(DeError::Codec)?;
        visitor.visit_i128(value)
    }

    fn deserialize_u8<V: de::Visitor<'de>>(self, visitor: V) -> Result<V::Value, Self::Error> {
        use crate::de::Decode;
        let value = u8::decode(self.decoder).map_err(DeError::Codec)?;
        visitor.visit_u8(value)
    }

    fn deserialize_u16<V: de::Visitor<'de>>(self, visitor: V) -> Result<V::Value, Self::Error> {
        use crate::de::Decode;
        let value = u16::decode(self.decoder).map_err(DeError::Codec)?;
        visitor.visit_u16(value)
    }

    fn deserialize_u32<V: de::Visitor<'de>>(self, visitor: V) -> Result<V::Value, Self::Error> {
        use crate::de::Decode;
        let value = u32::decode(self.decoder).map_err(DeError::Codec)?;
        visitor.visit_u32(value)
    }

    fn deserialize_u64<V: de::Visitor<'de>>(self, visitor: V) -> Result<V::Value, Self::Error> {
        use crate::de::Decode;
        let value = u64::decode(self.decoder).map_err(DeError::Codec)?;
        visitor.visit_u64(value)
    }

    fn deserialize_u128<V: de::Visitor<'de>>(self, visitor: V) -> Result<V::Value, Self::Error> {
        use crate::de::Decode;
        let value = u128::decode(self.decoder).map_err(DeError::Codec)?;
        visitor.visit_u128(value)
    }

    fn deserialize_f32<V: de::Visitor<'de>>(self, visitor: V) -> Result<V::Value, Self::Error> {
        use crate::de::Decode;
        let value = f32::decode(self.decoder).map_err(DeError::Codec)?;
        visitor.visit_f32(value)
    }

    fn deserialize_f64<V: de::Visitor<'de>>(self, visitor: V) -> Result<V::Value, Self::Error> {
        use crate::de::Decode;
        let value = f64::decode(self.decoder).map_err(DeError::Codec)?;
        visitor.visit_f64(value)
    }

    fn deserialize_char<V: de::Visitor<'de>>(self, visitor: V) -> Result<V::Value, Self::Error> {
        use crate::de::Decode;
        let value = char::decode(self.decoder).map_err(DeError::Codec)?;
        visitor.visit_char(value)
    }

    fn deserialize_str<V: de::Visitor<'de>>(self, visitor: V) -> Result<V::Value, Self::Error> {
        self.deserialize_string(visitor)
    }

    fn deserialize_string<V: de::Visitor<'de>>(self, visitor: V) -> Result<V::Value, Self::Error> {
        use crate::de::Decode;
        let value = alloc::string::String::decode(self.decoder).map_err(DeError::Codec)?;
        visitor.visit_string(value)
    }

    fn deserialize_bytes<V: de::Visitor<'de>>(self, visitor: V) -> Result<V::Value, Self::Error> {
        self.deserialize_byte_buf(visitor)
    }

    fn deserialize_byte_buf<V: de::Visitor<'de>>(
        self,
        visitor: V,
    ) -> Result<V::Value, Self::Error> {
        use crate::de::Decode;
        let value = alloc::vec::Vec::<u8>::decode(self.decoder).map_err(DeError::Codec)?;
        visitor.visit_byte_buf(value)
    }

    fn deserialize_option<V: de::Visitor<'de>>(self, visitor: V) -> Result<V::Value, Self::Error> {
        use crate::de::Decode;
        let variant = u8::decode(self.decoder).map_err(DeError::Codec)?;
        match variant {
            0 => visitor.visit_none(),
            1 => with_recursion_guard(self.decoder, |decoder| {
                visitor.visit_some(Deserializer::new(decoder))
            }),
            _ => Err(DeError::from_static("Invalid Option variant")),
        }
    }

    fn deserialize_unit<V: de::Visitor<'de>>(self, visitor: V) -> Result<V::Value, Self::Error> {
        visitor.visit_unit()
    }

    fn deserialize_unit_struct<V: de::Visitor<'de>>(
        self,
        _name: &'static str,
        visitor: V,
    ) -> Result<V::Value, Self::Error> {
        visitor.visit_unit()
    }

    fn deserialize_newtype_struct<V: de::Visitor<'de>>(
        self,
        _name: &'static str,
        visitor: V,
    ) -> Result<V::Value, Self::Error> {
        with_recursion_guard(self.decoder, |decoder| {
            visitor.visit_newtype_struct(Deserializer::new(decoder))
        })
    }

    fn deserialize_seq<V: de::Visitor<'de>>(self, visitor: V) -> Result<V::Value, Self::Error> {
        let len = decode_length(self.decoder)?;
        claim_container(self.decoder, len, 1)?;
        with_recursion_guard(self.decoder, |decoder| {
            visitor.visit_seq(SeqAccess::from_wire(decoder, len))
        })
    }

    fn deserialize_tuple<V: de::Visitor<'de>>(
        self,
        len: usize,
        visitor: V,
    ) -> Result<V::Value, Self::Error> {
        // `len` comes from the type's schema, not the wire, so it needs no
        // length validation or container claim.
        with_recursion_guard(self.decoder, |decoder| {
            visitor.visit_seq(SeqAccess::from_schema(decoder, len))
        })
    }

    fn deserialize_tuple_struct<V: de::Visitor<'de>>(
        self,
        _name: &'static str,
        len: usize,
        visitor: V,
    ) -> Result<V::Value, Self::Error> {
        self.deserialize_tuple(len, visitor)
    }

    fn deserialize_map<V: de::Visitor<'de>>(self, visitor: V) -> Result<V::Value, Self::Error> {
        let len = decode_length(self.decoder)?;
        // Each entry carries a key and a value, so at least two bytes.
        claim_container(self.decoder, len, 2)?;
        with_recursion_guard(self.decoder, |decoder| {
            visitor.visit_map(MapAccess::new(decoder, len))
        })
    }

    fn deserialize_struct<V: de::Visitor<'de>>(
        self,
        _name: &'static str,
        _fields: &'static [&'static str],
        visitor: V,
    ) -> Result<V::Value, Self::Error> {
        with_recursion_guard(self.decoder, |decoder| {
            visitor.visit_seq(SeqAccess::unbounded(decoder))
        })
    }

    fn deserialize_enum<V: de::Visitor<'de>>(
        self,
        _name: &'static str,
        _variants: &'static [&'static str],
        visitor: V,
    ) -> Result<V::Value, Self::Error> {
        with_recursion_guard(self.decoder, |decoder| {
            visitor.visit_enum(EnumAccess::new(decoder))
        })
    }

    fn deserialize_identifier<V: de::Visitor<'de>>(
        self,
        visitor: V,
    ) -> Result<V::Value, Self::Error> {
        self.deserialize_u32(visitor)
    }

    fn deserialize_ignored_any<V: de::Visitor<'de>>(
        self,
        _visitor: V,
    ) -> Result<V::Value, Self::Error> {
        Err(DeError::from_static(
            "deserialize_ignored_any not supported",
        ))
    }

    /// The oxicode wire format is compact and binary, never self-describing.
    ///
    /// serde defaults this method to `true`. Leaving it at the default made
    /// every `Serialize`/`Deserialize` impl that branches on human-readability
    /// take the opposite branch from `bincode::serde`, so `IpAddr` decoded a
    /// varint-prefixed ASCII string instead of a one-byte tag plus raw octets
    /// (and likewise for `uuid` and `chrono`) - a silent cross-library data
    /// corruption vector. It is also internally inconsistent: the
    /// human-readable branch is entitled to call `deserialize_any`, which this
    /// deserializer rejects. `bincode` 2.0.1 returns `false` here; so do we.
    fn is_human_readable(&self) -> bool {
        false
    }
}

// Compound deserializers

struct SeqAccess<'a, D: Decoder> {
    decoder: &'a mut D,
    /// Number of elements still to be produced.
    ///
    /// `None` means "driven by the visitor, not by a wire length" (struct
    /// fields and struct variants). This used to be encoded as the in-band
    /// sentinel `usize::MAX`, which collided with a wire length of `u64::MAX`:
    /// the counter then never decremented and `next_element_seed` returned
    /// `Some(..)` forever, an infinite decode loop from nine bytes of input.
    remaining: Option<usize>,
    /// Bytes to hand back per element, matching what [`claim_container`]
    /// reserved for this sequence. Zero for sequences whose length came from
    /// the type's schema rather than the wire, since those reserve nothing.
    unclaim_per_element: usize,
}

impl<'a, D: Decoder<Context = ()>> SeqAccess<'a, D> {
    /// Create a sequence whose length was read from the wire and reserved with
    /// [`claim_container`].
    fn from_wire(decoder: &'a mut D, len: usize) -> Self {
        Self {
            decoder,
            remaining: Some(len),
            unclaim_per_element: 1,
        }
    }

    /// Create a sequence whose length comes from the type's schema (tuples,
    /// tuple variants). Nothing was claimed up front, so nothing is unclaimed.
    fn from_schema(decoder: &'a mut D, len: usize) -> Self {
        Self {
            decoder,
            remaining: Some(len),
            unclaim_per_element: 0,
        }
    }

    /// Create a sequence whose length is dictated by the visitor rather than by
    /// a wire-encoded length prefix.
    fn unbounded(decoder: &'a mut D) -> Self {
        Self {
            decoder,
            remaining: None,
            unclaim_per_element: 0,
        }
    }
}

impl<'de, 'a, D: Decoder<Context = ()>> de::SeqAccess<'de> for SeqAccess<'a, D> {
    type Error = DeError;

    fn next_element_seed<T: de::DeserializeSeed<'de>>(
        &mut self,
        seed: T,
    ) -> Result<Option<T::Value>, Self::Error> {
        match self.remaining {
            Some(0) => return Ok(None),
            Some(ref mut remaining) => {
                *remaining -= 1;
                // Hand this element's reservation back before decoding it, so
                // the element's own claims are not double counted.
                self.decoder.unclaim_bytes_read(self.unclaim_per_element);
            }
            None => {}
        }
        seed.deserialize(Deserializer::new(self.decoder)).map(Some)
    }

    fn size_hint(&self) -> Option<usize> {
        self.remaining
    }
}

struct MapAccess<'a, D: Decoder> {
    decoder: &'a mut D,
    remaining: usize,
}

impl<'a, D: Decoder<Context = ()>> MapAccess<'a, D> {
    fn new(decoder: &'a mut D, len: usize) -> Self {
        Self {
            decoder,
            remaining: len,
        }
    }
}

impl<'de, 'a, D: Decoder<Context = ()>> de::MapAccess<'de> for MapAccess<'a, D> {
    type Error = DeError;

    fn next_key_seed<K: de::DeserializeSeed<'de>>(
        &mut self,
        seed: K,
    ) -> Result<Option<K::Value>, Self::Error> {
        if self.remaining == 0 {
            return Ok(None);
        }
        self.remaining -= 1;
        // Release the key half of this entry's two-byte reservation.
        self.decoder.unclaim_bytes_read(1);
        seed.deserialize(Deserializer::new(self.decoder)).map(Some)
    }

    fn next_value_seed<V: de::DeserializeSeed<'de>>(
        &mut self,
        seed: V,
    ) -> Result<V::Value, Self::Error> {
        // Release the value half of this entry's two-byte reservation.
        self.decoder.unclaim_bytes_read(1);
        seed.deserialize(Deserializer::new(self.decoder))
    }

    fn size_hint(&self) -> Option<usize> {
        Some(self.remaining)
    }
}

struct EnumAccess<'a, D: Decoder> {
    decoder: &'a mut D,
}

impl<'a, D: Decoder<Context = ()>> EnumAccess<'a, D> {
    fn new(decoder: &'a mut D) -> Self {
        Self { decoder }
    }
}

impl<'de, 'a, D: Decoder<Context = ()>> de::EnumAccess<'de> for EnumAccess<'a, D> {
    type Error = DeError;
    type Variant = VariantAccess<'a, D>;

    fn variant_seed<V: de::DeserializeSeed<'de>>(
        self,
        seed: V,
    ) -> Result<(V::Value, Self::Variant), Self::Error> {
        let variant = seed.deserialize(Deserializer::new(self.decoder))?;
        Ok((variant, VariantAccess::new(self.decoder)))
    }
}

struct VariantAccess<'a, D: Decoder> {
    decoder: &'a mut D,
}

impl<'a, D: Decoder<Context = ()>> VariantAccess<'a, D> {
    fn new(decoder: &'a mut D) -> Self {
        Self { decoder }
    }
}

impl<'de, 'a, D: Decoder<Context = ()>> de::VariantAccess<'de> for VariantAccess<'a, D> {
    type Error = DeError;

    fn unit_variant(self) -> Result<(), Self::Error> {
        Ok(())
    }

    fn newtype_variant_seed<T: de::DeserializeSeed<'de>>(
        self,
        seed: T,
    ) -> Result<T::Value, Self::Error> {
        with_recursion_guard(self.decoder, |decoder| {
            seed.deserialize(Deserializer::new(decoder))
        })
    }

    fn tuple_variant<V: de::Visitor<'de>>(
        self,
        len: usize,
        visitor: V,
    ) -> Result<V::Value, Self::Error> {
        with_recursion_guard(self.decoder, |decoder| {
            visitor.visit_seq(SeqAccess::from_schema(decoder, len))
        })
    }

    fn struct_variant<V: de::Visitor<'de>>(
        self,
        _fields: &'static [&'static str],
        visitor: V,
    ) -> Result<V::Value, Self::Error> {
        with_recursion_guard(self.decoder, |decoder| {
            visitor.visit_seq(SeqAccess::unbounded(decoder))
        })
    }
}
