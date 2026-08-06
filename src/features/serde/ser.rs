//! Serde Serializer implementation

use crate::enc::Encoder;
use serde::ser;

/// Error type for serde serialization
///
/// This wraps the underlying oxicode [`crate::error::Error`] without loss so that
/// callers can still recover the concrete failure cause (for example
/// [`crate::error::Error::UnexpectedEnd`] when a fixed-size output buffer is
/// exhausted) after encoding through the serde bridge. Genuine serde `custom`
/// messages that have no oxicode counterpart are carried as [`SerError::Custom`].
#[derive(Debug)]
pub enum SerError {
    /// The underlying oxicode error, preserved verbatim.
    Codec(crate::error::Error),
    /// A serde-originated message with no underlying oxicode error.
    Custom(alloc::string::String),
}

impl SerError {
    /// Create a SerError from a static string slice.
    pub(crate) fn from_static(msg: &'static str) -> Self {
        SerError::Custom(alloc::string::String::from(msg))
    }
}

impl ser::Error for SerError {
    fn custom<T: core::fmt::Display>(msg: T) -> Self {
        SerError::Custom(alloc::format!("{}", msg))
    }
}

impl core::fmt::Display for SerError {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            SerError::Codec(err) => write!(f, "{}", err),
            SerError::Custom(msg) => write!(f, "{}", msg),
        }
    }
}

impl core::error::Error for SerError {}

impl From<SerError> for crate::error::Error {
    fn from(err: SerError) -> Self {
        match err {
            SerError::Codec(inner) => inner,
            SerError::Custom(message) => crate::error::Error::OwnedCustom { message },
        }
    }
}

/// Serde serializer that wraps an oxicode Encoder
pub struct Serializer<'a, E: Encoder> {
    encoder: &'a mut E,
}

impl<'a, E: Encoder> Serializer<'a, E> {
    /// Create a new Serializer wrapping the given Encoder
    pub fn new(encoder: &'a mut E) -> Self {
        Self { encoder }
    }
}

impl<'a, E: Encoder> ser::Serializer for Serializer<'a, E> {
    type Ok = ();
    type Error = SerError;
    type SerializeSeq = SeqSerializer<'a, E>;
    type SerializeTuple = TupleSerializer<'a, E>;
    type SerializeTupleStruct = TupleSerializer<'a, E>;
    type SerializeTupleVariant = TupleVariantSerializer<'a, E>;
    type SerializeMap = MapSerializer<'a, E>;
    type SerializeStruct = StructSerializer<'a, E>;
    type SerializeStructVariant = StructVariantSerializer<'a, E>;

    fn serialize_bool(self, v: bool) -> Result<Self::Ok, Self::Error> {
        use crate::enc::Encode;
        v.encode(self.encoder).map_err(SerError::Codec)
    }

    fn serialize_i8(self, v: i8) -> Result<Self::Ok, Self::Error> {
        use crate::enc::Encode;
        v.encode(self.encoder).map_err(SerError::Codec)
    }

    fn serialize_i16(self, v: i16) -> Result<Self::Ok, Self::Error> {
        use crate::enc::Encode;
        v.encode(self.encoder).map_err(SerError::Codec)
    }

    fn serialize_i32(self, v: i32) -> Result<Self::Ok, Self::Error> {
        use crate::enc::Encode;
        v.encode(self.encoder).map_err(SerError::Codec)
    }

    fn serialize_i64(self, v: i64) -> Result<Self::Ok, Self::Error> {
        use crate::enc::Encode;
        v.encode(self.encoder).map_err(SerError::Codec)
    }

    fn serialize_i128(self, v: i128) -> Result<Self::Ok, Self::Error> {
        use crate::enc::Encode;
        v.encode(self.encoder).map_err(SerError::Codec)
    }

    fn serialize_u8(self, v: u8) -> Result<Self::Ok, Self::Error> {
        use crate::enc::Encode;
        v.encode(self.encoder).map_err(SerError::Codec)
    }

    fn serialize_u16(self, v: u16) -> Result<Self::Ok, Self::Error> {
        use crate::enc::Encode;
        v.encode(self.encoder).map_err(SerError::Codec)
    }

    fn serialize_u32(self, v: u32) -> Result<Self::Ok, Self::Error> {
        use crate::enc::Encode;
        v.encode(self.encoder).map_err(SerError::Codec)
    }

    fn serialize_u64(self, v: u64) -> Result<Self::Ok, Self::Error> {
        use crate::enc::Encode;
        v.encode(self.encoder).map_err(SerError::Codec)
    }

    fn serialize_u128(self, v: u128) -> Result<Self::Ok, Self::Error> {
        use crate::enc::Encode;
        v.encode(self.encoder).map_err(SerError::Codec)
    }

    fn serialize_f32(self, v: f32) -> Result<Self::Ok, Self::Error> {
        use crate::enc::Encode;
        v.encode(self.encoder).map_err(SerError::Codec)
    }

    fn serialize_f64(self, v: f64) -> Result<Self::Ok, Self::Error> {
        use crate::enc::Encode;
        v.encode(self.encoder).map_err(SerError::Codec)
    }

    fn serialize_char(self, v: char) -> Result<Self::Ok, Self::Error> {
        use crate::enc::Encode;
        v.encode(self.encoder).map_err(SerError::Codec)
    }

    fn serialize_str(self, v: &str) -> Result<Self::Ok, Self::Error> {
        use crate::enc::Encode;
        v.encode(self.encoder).map_err(SerError::Codec)
    }

    fn serialize_bytes(self, v: &[u8]) -> Result<Self::Ok, Self::Error> {
        use crate::enc::Encode;
        v.encode(self.encoder).map_err(SerError::Codec)
    }

    fn serialize_none(self) -> Result<Self::Ok, Self::Error> {
        use crate::enc::Encode;
        0u8.encode(self.encoder).map_err(SerError::Codec)
    }

    fn serialize_some<T: ?Sized + ser::Serialize>(
        self,
        value: &T,
    ) -> Result<Self::Ok, Self::Error> {
        use crate::enc::Encode;
        1u8.encode(self.encoder).map_err(SerError::Codec)?;
        value.serialize(self)
    }

    fn serialize_unit(self) -> Result<Self::Ok, Self::Error> {
        Ok(())
    }

    fn serialize_unit_struct(self, _name: &'static str) -> Result<Self::Ok, Self::Error> {
        self.serialize_unit()
    }

    fn serialize_unit_variant(
        self,
        _name: &'static str,
        variant_index: u32,
        _variant: &'static str,
    ) -> Result<Self::Ok, Self::Error> {
        use crate::enc::Encode;
        variant_index.encode(self.encoder).map_err(SerError::Codec)
    }

    fn serialize_newtype_struct<T: ?Sized + ser::Serialize>(
        self,
        _name: &'static str,
        value: &T,
    ) -> Result<Self::Ok, Self::Error> {
        value.serialize(self)
    }

    fn serialize_newtype_variant<T: ?Sized + ser::Serialize>(
        self,
        _name: &'static str,
        variant_index: u32,
        _variant: &'static str,
        value: &T,
    ) -> Result<Self::Ok, Self::Error> {
        use crate::enc::Encode;
        variant_index
            .encode(self.encoder)
            .map_err(SerError::Codec)?;
        value.serialize(self)
    }

    fn serialize_seq(self, len: Option<usize>) -> Result<Self::SerializeSeq, Self::Error> {
        let len = len.ok_or_else(|| SerError::from_static("Sequence length required"))?;
        use crate::enc::Encode;
        (len as u64).encode(self.encoder).map_err(SerError::Codec)?;
        Ok(SeqSerializer {
            encoder: self.encoder,
        })
    }

    fn serialize_tuple(self, _len: usize) -> Result<Self::SerializeTuple, Self::Error> {
        Ok(TupleSerializer {
            encoder: self.encoder,
        })
    }

    fn serialize_tuple_struct(
        self,
        _name: &'static str,
        _len: usize,
    ) -> Result<Self::SerializeTupleStruct, Self::Error> {
        Ok(TupleSerializer {
            encoder: self.encoder,
        })
    }

    fn serialize_tuple_variant(
        self,
        _name: &'static str,
        variant_index: u32,
        _variant: &'static str,
        _len: usize,
    ) -> Result<Self::SerializeTupleVariant, Self::Error> {
        use crate::enc::Encode;
        variant_index
            .encode(self.encoder)
            .map_err(SerError::Codec)?;
        Ok(TupleVariantSerializer {
            encoder: self.encoder,
        })
    }

    fn serialize_map(self, len: Option<usize>) -> Result<Self::SerializeMap, Self::Error> {
        let len = len.ok_or_else(|| SerError::from_static("Map length required"))?;
        use crate::enc::Encode;
        (len as u64).encode(self.encoder).map_err(SerError::Codec)?;
        Ok(MapSerializer {
            encoder: self.encoder,
        })
    }

    fn serialize_struct(
        self,
        _name: &'static str,
        _len: usize,
    ) -> Result<Self::SerializeStruct, Self::Error> {
        Ok(StructSerializer {
            encoder: self.encoder,
        })
    }

    fn serialize_struct_variant(
        self,
        _name: &'static str,
        variant_index: u32,
        _variant: &'static str,
        _len: usize,
    ) -> Result<Self::SerializeStructVariant, Self::Error> {
        use crate::enc::Encode;
        variant_index
            .encode(self.encoder)
            .map_err(SerError::Codec)?;
        Ok(StructVariantSerializer {
            encoder: self.encoder,
        })
    }

    /// The oxicode wire format is compact and binary, never self-describing.
    ///
    /// serde defaults this method to `true`. Leaving it at the default made
    /// every `Serialize` impl that branches on human-readability take the
    /// opposite branch from `bincode::serde`: `127.0.0.1` went on the wire as a
    /// varint length plus ten ASCII bytes instead of a one-byte tag plus four
    /// octets, and `uuid` as a 36-character string instead of 16 bytes. That is
    /// both a size regression and a silent cross-library incompatibility.
    /// `bincode` 2.0.1 returns `false` here; so do we.
    fn is_human_readable(&self) -> bool {
        false
    }
}

// Compound serializers

/// Serializer for sequences
pub struct SeqSerializer<'a, E: Encoder> {
    encoder: &'a mut E,
}

impl<'a, E: Encoder> ser::SerializeSeq for SeqSerializer<'a, E> {
    type Ok = ();
    type Error = SerError;

    fn serialize_element<T: ?Sized + ser::Serialize>(
        &mut self,
        value: &T,
    ) -> Result<(), Self::Error> {
        value.serialize(Serializer::new(self.encoder))
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        Ok(())
    }
}

/// Serializer for tuples
pub struct TupleSerializer<'a, E: Encoder> {
    encoder: &'a mut E,
}

impl<'a, E: Encoder> ser::SerializeTuple for TupleSerializer<'a, E> {
    type Ok = ();
    type Error = SerError;

    fn serialize_element<T: ?Sized + ser::Serialize>(
        &mut self,
        value: &T,
    ) -> Result<(), Self::Error> {
        value.serialize(Serializer::new(self.encoder))
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        Ok(())
    }
}

impl<'a, E: Encoder> ser::SerializeTupleStruct for TupleSerializer<'a, E> {
    type Ok = ();
    type Error = SerError;

    fn serialize_field<T: ?Sized + ser::Serialize>(
        &mut self,
        value: &T,
    ) -> Result<(), Self::Error> {
        value.serialize(Serializer::new(self.encoder))
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        Ok(())
    }
}

/// Serializer for tuple variants
pub struct TupleVariantSerializer<'a, E: Encoder> {
    encoder: &'a mut E,
}

impl<'a, E: Encoder> ser::SerializeTupleVariant for TupleVariantSerializer<'a, E> {
    type Ok = ();
    type Error = SerError;

    fn serialize_field<T: ?Sized + ser::Serialize>(
        &mut self,
        value: &T,
    ) -> Result<(), Self::Error> {
        value.serialize(Serializer::new(self.encoder))
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        Ok(())
    }
}

/// Serializer for maps
pub struct MapSerializer<'a, E: Encoder> {
    encoder: &'a mut E,
}

impl<'a, E: Encoder> ser::SerializeMap for MapSerializer<'a, E> {
    type Ok = ();
    type Error = SerError;

    fn serialize_key<T: ?Sized + ser::Serialize>(&mut self, key: &T) -> Result<(), Self::Error> {
        key.serialize(Serializer::new(self.encoder))
    }

    fn serialize_value<T: ?Sized + ser::Serialize>(
        &mut self,
        value: &T,
    ) -> Result<(), Self::Error> {
        value.serialize(Serializer::new(self.encoder))
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        Ok(())
    }
}

/// Serializer for structs
pub struct StructSerializer<'a, E: Encoder> {
    encoder: &'a mut E,
}

impl<'a, E: Encoder> ser::SerializeStruct for StructSerializer<'a, E> {
    type Ok = ();
    type Error = SerError;

    fn serialize_field<T: ?Sized + ser::Serialize>(
        &mut self,
        _key: &'static str,
        value: &T,
    ) -> Result<(), Self::Error> {
        value.serialize(Serializer::new(self.encoder))
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        Ok(())
    }
}

/// Serializer for struct variants
pub struct StructVariantSerializer<'a, E: Encoder> {
    encoder: &'a mut E,
}

impl<'a, E: Encoder> ser::SerializeStructVariant for StructVariantSerializer<'a, E> {
    type Ok = ();
    type Error = SerError;

    fn serialize_field<T: ?Sized + ser::Serialize>(
        &mut self,
        _key: &'static str,
        value: &T,
    ) -> Result<(), Self::Error> {
        value.serialize(Serializer::new(self.encoder))
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        Ok(())
    }
}
