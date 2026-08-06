//! Borrowed (zero-copy) serde Deserializer implementation.
//!
//! This mirrors the owned [`super::de::Deserializer`] but is parameterized over a
//! [`BorrowDecoder`], which lets it hand serde's visitor slices that borrow
//! directly from the input buffer. Concretely, [`deserialize_str`] and
//! [`deserialize_bytes`] call [`serde::de::Visitor::visit_borrowed_str`] and
//! [`serde::de::Visitor::visit_borrowed_bytes`] respectively, which are the only
//! visitor entry points that serde's `&'de str` / `&'de [u8]` implementations
//! accept. The owned deserializer, by contrast, can only produce `visit_string`
//! / `visit_byte_buf`, so decoding a borrowed field through it always fails at
//! runtime.
//!
//! The wire format read here is byte-identical to the owned path: a length
//! prefix (`u64`) followed by the raw bytes, exactly as `String::decode` /
//! `Vec::<u8>::decode` and `<&str>::borrow_decode` / `<&[u8]>::borrow_decode`
//! read them.
//!
//! [`deserialize_str`]: serde::Deserializer::deserialize_str
//! [`deserialize_bytes`]: serde::Deserializer::deserialize_bytes
//! [`BorrowDecoder`]: crate::de::BorrowDecoder

use super::de::{claim_container, decode_length, with_recursion_guard, DeError};
use crate::de::BorrowDecoder;
use serde::de;

/// Serde deserializer that wraps an oxicode [`BorrowDecoder`] for zero-copy decoding.
///
/// Unlike [`super::de::Deserializer`], this emits borrowed data (`&'de str`,
/// `&'de [u8]`) whenever the visitor requests it, enabling `#[derive(Deserialize)]`
/// types with borrowed fields to decode without copying.
pub struct BorrowedDeserializer<'a, D> {
    decoder: &'a mut D,
}

impl<'a, D> BorrowedDeserializer<'a, D> {
    /// Create a new borrowed deserializer wrapping the given borrow decoder.
    pub fn new(decoder: &'a mut D) -> Self {
        Self { decoder }
    }
}

impl<'de, 'a, D: BorrowDecoder<'de, Context = ()>> de::Deserializer<'de>
    for BorrowedDeserializer<'a, D>
{
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
        use crate::de::BorrowDecode;
        let value = <&'de str>::borrow_decode(self.decoder).map_err(DeError::Codec)?;
        visitor.visit_borrowed_str(value)
    }

    fn deserialize_string<V: de::Visitor<'de>>(self, visitor: V) -> Result<V::Value, Self::Error> {
        use crate::de::Decode;
        let value = alloc::string::String::decode(self.decoder).map_err(DeError::Codec)?;
        visitor.visit_string(value)
    }

    fn deserialize_bytes<V: de::Visitor<'de>>(self, visitor: V) -> Result<V::Value, Self::Error> {
        use crate::de::BorrowDecode;
        let value = <&'de [u8]>::borrow_decode(self.decoder).map_err(DeError::Codec)?;
        visitor.visit_borrowed_bytes(value)
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
                visitor.visit_some(BorrowedDeserializer::new(decoder))
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
            visitor.visit_newtype_struct(BorrowedDeserializer::new(decoder))
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
        // `len` comes from the type's schema, not the wire.
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
    /// See [`super::de::Deserializer::is_human_readable`] for the full
    /// rationale; both deserializers must agree with the serializer.
    fn is_human_readable(&self) -> bool {
        false
    }
}

// Compound deserializers (borrowed variants).

struct SeqAccess<'a, D> {
    decoder: &'a mut D,
    /// Number of elements still to be produced; `None` when the visitor, not a
    /// wire length prefix, dictates the count. See the owned deserializer's
    /// `SeqAccess` for why the previous `usize::MAX` sentinel was unsound.
    remaining: Option<usize>,
    /// Bytes to hand back per element, matching what `claim_container`
    /// reserved. Zero when the length came from the type's schema.
    unclaim_per_element: usize,
}

impl<'a, D> SeqAccess<'a, D> {
    /// Create a sequence whose length was read from the wire and reserved with
    /// `claim_container`.
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

impl<'de, 'a, D: BorrowDecoder<'de, Context = ()>> de::SeqAccess<'de> for SeqAccess<'a, D> {
    type Error = DeError;

    fn next_element_seed<T: de::DeserializeSeed<'de>>(
        &mut self,
        seed: T,
    ) -> Result<Option<T::Value>, Self::Error> {
        match self.remaining {
            Some(0) => return Ok(None),
            Some(ref mut remaining) => {
                *remaining -= 1;
                self.decoder.unclaim_bytes_read(self.unclaim_per_element);
            }
            None => {}
        }
        seed.deserialize(BorrowedDeserializer::new(self.decoder))
            .map(Some)
    }

    fn size_hint(&self) -> Option<usize> {
        self.remaining
    }
}

struct MapAccess<'a, D> {
    decoder: &'a mut D,
    remaining: usize,
}

impl<'a, D> MapAccess<'a, D> {
    fn new(decoder: &'a mut D, len: usize) -> Self {
        Self {
            decoder,
            remaining: len,
        }
    }
}

impl<'de, 'a, D: BorrowDecoder<'de, Context = ()>> de::MapAccess<'de> for MapAccess<'a, D> {
    type Error = DeError;

    fn next_key_seed<K: de::DeserializeSeed<'de>>(
        &mut self,
        seed: K,
    ) -> Result<Option<K::Value>, Self::Error> {
        if self.remaining == 0 {
            return Ok(None);
        }
        self.remaining -= 1;
        self.decoder.unclaim_bytes_read(1);
        seed.deserialize(BorrowedDeserializer::new(self.decoder))
            .map(Some)
    }

    fn next_value_seed<V: de::DeserializeSeed<'de>>(
        &mut self,
        seed: V,
    ) -> Result<V::Value, Self::Error> {
        self.decoder.unclaim_bytes_read(1);
        seed.deserialize(BorrowedDeserializer::new(self.decoder))
    }

    fn size_hint(&self) -> Option<usize> {
        Some(self.remaining)
    }
}

struct EnumAccess<'a, D> {
    decoder: &'a mut D,
}

impl<'a, D> EnumAccess<'a, D> {
    fn new(decoder: &'a mut D) -> Self {
        Self { decoder }
    }
}

impl<'de, 'a, D: BorrowDecoder<'de, Context = ()>> de::EnumAccess<'de> for EnumAccess<'a, D> {
    type Error = DeError;
    type Variant = VariantAccess<'a, D>;

    fn variant_seed<V: de::DeserializeSeed<'de>>(
        self,
        seed: V,
    ) -> Result<(V::Value, Self::Variant), Self::Error> {
        let variant = seed.deserialize(BorrowedDeserializer::new(self.decoder))?;
        Ok((variant, VariantAccess::new(self.decoder)))
    }
}

struct VariantAccess<'a, D> {
    decoder: &'a mut D,
}

impl<'a, D> VariantAccess<'a, D> {
    fn new(decoder: &'a mut D) -> Self {
        Self { decoder }
    }
}

impl<'de, 'a, D: BorrowDecoder<'de, Context = ()>> de::VariantAccess<'de> for VariantAccess<'a, D> {
    type Error = DeError;

    fn unit_variant(self) -> Result<(), Self::Error> {
        Ok(())
    }

    fn newtype_variant_seed<T: de::DeserializeSeed<'de>>(
        self,
        seed: T,
    ) -> Result<T::Value, Self::Error> {
        with_recursion_guard(self.decoder, |decoder| {
            seed.deserialize(BorrowedDeserializer::new(decoder))
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
