//! Serde Deserializer implementation

use crate::de::Decoder;
use serde::de;

/// Error type for serde deserialization
#[derive(Debug)]
pub struct DeError {
    pub(crate) msg: alloc::string::String,
}

impl DeError {
    /// Create a DeError from a static string slice
    pub(crate) fn from_static(msg: &'static str) -> Self {
        DeError {
            msg: alloc::string::String::from(msg),
        }
    }
}

impl de::Error for DeError {
    fn custom<T: core::fmt::Display>(msg: T) -> Self {
        DeError {
            msg: alloc::format!("{}", msg),
        }
    }
}

impl core::fmt::Display for DeError {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "{}", self.msg)
    }
}

#[cfg(feature = "std")]
impl std::error::Error for DeError {}

impl From<DeError> for crate::error::Error {
    fn from(err: DeError) -> Self {
        crate::error::Error::OwnedCustom { message: err.msg }
    }
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
        let value = bool::decode(self.decoder)
            .map_err(|_| DeError::from_static("Failed to decode bool"))?;
        visitor.visit_bool(value)
    }

    fn deserialize_i8<V: de::Visitor<'de>>(self, visitor: V) -> Result<V::Value, Self::Error> {
        use crate::de::Decode;
        let value =
            i8::decode(self.decoder).map_err(|_| DeError::from_static("Failed to decode i8"))?;
        visitor.visit_i8(value)
    }

    fn deserialize_i16<V: de::Visitor<'de>>(self, visitor: V) -> Result<V::Value, Self::Error> {
        use crate::de::Decode;
        let value =
            i16::decode(self.decoder).map_err(|_| DeError::from_static("Failed to decode i16"))?;
        visitor.visit_i16(value)
    }

    fn deserialize_i32<V: de::Visitor<'de>>(self, visitor: V) -> Result<V::Value, Self::Error> {
        use crate::de::Decode;
        let value =
            i32::decode(self.decoder).map_err(|_| DeError::from_static("Failed to decode i32"))?;
        visitor.visit_i32(value)
    }

    fn deserialize_i64<V: de::Visitor<'de>>(self, visitor: V) -> Result<V::Value, Self::Error> {
        use crate::de::Decode;
        let value =
            i64::decode(self.decoder).map_err(|_| DeError::from_static("Failed to decode i64"))?;
        visitor.visit_i64(value)
    }

    fn deserialize_i128<V: de::Visitor<'de>>(self, visitor: V) -> Result<V::Value, Self::Error> {
        use crate::de::Decode;
        let value = i128::decode(self.decoder)
            .map_err(|_| DeError::from_static("Failed to decode i128"))?;
        visitor.visit_i128(value)
    }

    fn deserialize_u8<V: de::Visitor<'de>>(self, visitor: V) -> Result<V::Value, Self::Error> {
        use crate::de::Decode;
        let value =
            u8::decode(self.decoder).map_err(|_| DeError::from_static("Failed to decode u8"))?;
        visitor.visit_u8(value)
    }

    fn deserialize_u16<V: de::Visitor<'de>>(self, visitor: V) -> Result<V::Value, Self::Error> {
        use crate::de::Decode;
        let value =
            u16::decode(self.decoder).map_err(|_| DeError::from_static("Failed to decode u16"))?;
        visitor.visit_u16(value)
    }

    fn deserialize_u32<V: de::Visitor<'de>>(self, visitor: V) -> Result<V::Value, Self::Error> {
        use crate::de::Decode;
        let value =
            u32::decode(self.decoder).map_err(|_| DeError::from_static("Failed to decode u32"))?;
        visitor.visit_u32(value)
    }

    fn deserialize_u64<V: de::Visitor<'de>>(self, visitor: V) -> Result<V::Value, Self::Error> {
        use crate::de::Decode;
        let value =
            u64::decode(self.decoder).map_err(|_| DeError::from_static("Failed to decode u64"))?;
        visitor.visit_u64(value)
    }

    fn deserialize_u128<V: de::Visitor<'de>>(self, visitor: V) -> Result<V::Value, Self::Error> {
        use crate::de::Decode;
        let value = u128::decode(self.decoder)
            .map_err(|_| DeError::from_static("Failed to decode u128"))?;
        visitor.visit_u128(value)
    }

    fn deserialize_f32<V: de::Visitor<'de>>(self, visitor: V) -> Result<V::Value, Self::Error> {
        use crate::de::Decode;
        let value =
            f32::decode(self.decoder).map_err(|_| DeError::from_static("Failed to decode f32"))?;
        visitor.visit_f32(value)
    }

    fn deserialize_f64<V: de::Visitor<'de>>(self, visitor: V) -> Result<V::Value, Self::Error> {
        use crate::de::Decode;
        let value =
            f64::decode(self.decoder).map_err(|_| DeError::from_static("Failed to decode f64"))?;
        visitor.visit_f64(value)
    }

    fn deserialize_char<V: de::Visitor<'de>>(self, visitor: V) -> Result<V::Value, Self::Error> {
        use crate::de::Decode;
        let value = char::decode(self.decoder)
            .map_err(|_| DeError::from_static("Failed to decode char"))?;
        visitor.visit_char(value)
    }

    fn deserialize_str<V: de::Visitor<'de>>(self, visitor: V) -> Result<V::Value, Self::Error> {
        self.deserialize_string(visitor)
    }

    fn deserialize_string<V: de::Visitor<'de>>(self, visitor: V) -> Result<V::Value, Self::Error> {
        use crate::de::Decode;
        let value = alloc::string::String::decode(self.decoder)
            .map_err(|_| DeError::from_static("Failed to decode String"))?;
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
        let value = alloc::vec::Vec::<u8>::decode(self.decoder)
            .map_err(|_| DeError::from_static("Failed to decode bytes"))?;
        visitor.visit_byte_buf(value)
    }

    fn deserialize_option<V: de::Visitor<'de>>(self, visitor: V) -> Result<V::Value, Self::Error> {
        use crate::de::Decode;
        let variant = u8::decode(self.decoder)
            .map_err(|_| DeError::from_static("Failed to decode Option variant"))?;
        match variant {
            0 => visitor.visit_none(),
            1 => visitor.visit_some(self),
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
        visitor.visit_newtype_struct(self)
    }

    fn deserialize_seq<V: de::Visitor<'de>>(self, visitor: V) -> Result<V::Value, Self::Error> {
        use crate::de::Decode;
        let len = u64::decode(self.decoder)
            .map_err(|_| DeError::from_static("Failed to decode length"))?
            as usize;
        visitor.visit_seq(SeqAccess::new(self.decoder, len))
    }

    fn deserialize_tuple<V: de::Visitor<'de>>(
        self,
        len: usize,
        visitor: V,
    ) -> Result<V::Value, Self::Error> {
        visitor.visit_seq(SeqAccess::new(self.decoder, len))
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
        use crate::de::Decode;
        let len = u64::decode(self.decoder)
            .map_err(|_| DeError::from_static("Failed to decode length"))?
            as usize;
        visitor.visit_map(MapAccess::new(self.decoder, len))
    }

    fn deserialize_struct<V: de::Visitor<'de>>(
        self,
        _name: &'static str,
        _fields: &'static [&'static str],
        visitor: V,
    ) -> Result<V::Value, Self::Error> {
        visitor.visit_seq(SeqAccess::new(self.decoder, usize::MAX))
    }

    fn deserialize_enum<V: de::Visitor<'de>>(
        self,
        _name: &'static str,
        _variants: &'static [&'static str],
        visitor: V,
    ) -> Result<V::Value, Self::Error> {
        visitor.visit_enum(EnumAccess::new(self.decoder))
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
}

// Compound deserializers

struct SeqAccess<'a, D: Decoder> {
    decoder: &'a mut D,
    remaining: usize,
}

impl<'a, D: Decoder<Context = ()>> SeqAccess<'a, D> {
    fn new(decoder: &'a mut D, len: usize) -> Self {
        Self {
            decoder,
            remaining: len,
        }
    }
}

impl<'de, 'a, D: Decoder<Context = ()>> de::SeqAccess<'de> for SeqAccess<'a, D> {
    type Error = DeError;

    fn next_element_seed<T: de::DeserializeSeed<'de>>(
        &mut self,
        seed: T,
    ) -> Result<Option<T::Value>, Self::Error> {
        if self.remaining == 0 {
            return Ok(None);
        }
        if self.remaining != usize::MAX {
            self.remaining -= 1;
        }
        seed.deserialize(Deserializer::new(self.decoder)).map(Some)
    }

    fn size_hint(&self) -> Option<usize> {
        if self.remaining == usize::MAX {
            None
        } else {
            Some(self.remaining)
        }
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
        seed.deserialize(Deserializer::new(self.decoder)).map(Some)
    }

    fn next_value_seed<V: de::DeserializeSeed<'de>>(
        &mut self,
        seed: V,
    ) -> Result<V::Value, Self::Error> {
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
        seed.deserialize(Deserializer::new(self.decoder))
    }

    fn tuple_variant<V: de::Visitor<'de>>(
        self,
        len: usize,
        visitor: V,
    ) -> Result<V::Value, Self::Error> {
        visitor.visit_seq(SeqAccess::new(self.decoder, len))
    }

    fn struct_variant<V: de::Visitor<'de>>(
        self,
        _fields: &'static [&'static str],
        visitor: V,
    ) -> Result<V::Value, Self::Error> {
        visitor.visit_seq(SeqAccess::new(self.decoder, usize::MAX))
    }
}
