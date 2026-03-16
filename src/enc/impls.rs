//! Encode implementations for primitive and standard types

use super::{write::Writer, Encode, Encoder};
use crate::{
    config::{Endianness, IntEncoding, InternalEndianConfig, InternalIntEncodingConfig},
    error::Error,
};
use core::marker::PhantomData;

// ===== Unit and PhantomData =====

impl Encode for () {
    fn encode<E: Encoder>(&self, _: &mut E) -> Result<(), Error> {
        Ok(())
    }
}

impl<T: ?Sized> Encode for PhantomData<T> {
    fn encode<E: Encoder>(&self, _: &mut E) -> Result<(), Error> {
        Ok(())
    }
}

// ===== Boolean =====

impl Encode for bool {
    fn encode<E: Encoder>(&self, encoder: &mut E) -> Result<(), Error> {
        u8::from(*self).encode(encoder)
    }
}

// ===== Unsigned Integers =====

impl Encode for u8 {
    fn encode<E: Encoder>(&self, encoder: &mut E) -> Result<(), Error> {
        encoder.writer().write(&[*self])
    }
}

impl Encode for u16 {
    fn encode<E: Encoder>(&self, encoder: &mut E) -> Result<(), Error> {
        match E::C::INT_ENCODING {
            IntEncoding::Variable => {
                crate::varint::varint_encode_u16(encoder.writer(), E::C::ENDIAN, *self)
            }
            IntEncoding::Fixed => match E::C::ENDIAN {
                Endianness::Big => encoder.writer().write(&self.to_be_bytes()),
                Endianness::Little => encoder.writer().write(&self.to_le_bytes()),
            },
        }
    }
}

impl Encode for u32 {
    fn encode<E: Encoder>(&self, encoder: &mut E) -> Result<(), Error> {
        match E::C::INT_ENCODING {
            IntEncoding::Variable => {
                crate::varint::varint_encode_u32(encoder.writer(), E::C::ENDIAN, *self)
            }
            IntEncoding::Fixed => match E::C::ENDIAN {
                Endianness::Big => encoder.writer().write(&self.to_be_bytes()),
                Endianness::Little => encoder.writer().write(&self.to_le_bytes()),
            },
        }
    }
}

impl Encode for u64 {
    fn encode<E: Encoder>(&self, encoder: &mut E) -> Result<(), Error> {
        match E::C::INT_ENCODING {
            IntEncoding::Variable => {
                crate::varint::varint_encode_u64(encoder.writer(), E::C::ENDIAN, *self)
            }
            IntEncoding::Fixed => match E::C::ENDIAN {
                Endianness::Big => encoder.writer().write(&self.to_be_bytes()),
                Endianness::Little => encoder.writer().write(&self.to_le_bytes()),
            },
        }
    }
}

impl Encode for u128 {
    fn encode<E: Encoder>(&self, encoder: &mut E) -> Result<(), Error> {
        match E::C::INT_ENCODING {
            IntEncoding::Variable => {
                crate::varint::varint_encode_u128(encoder.writer(), E::C::ENDIAN, *self)
            }
            IntEncoding::Fixed => match E::C::ENDIAN {
                Endianness::Big => encoder.writer().write(&self.to_be_bytes()),
                Endianness::Little => encoder.writer().write(&self.to_le_bytes()),
            },
        }
    }
}

impl Encode for usize {
    fn encode<E: Encoder>(&self, encoder: &mut E) -> Result<(), Error> {
        match E::C::INT_ENCODING {
            IntEncoding::Variable => {
                crate::varint::varint_encode_usize(encoder.writer(), E::C::ENDIAN, *self)
            }
            IntEncoding::Fixed => match E::C::ENDIAN {
                Endianness::Big => encoder.writer().write(&(*self as u64).to_be_bytes()),
                Endianness::Little => encoder.writer().write(&(*self as u64).to_le_bytes()),
            },
        }
    }
}

// ===== Signed Integers =====

impl Encode for i8 {
    fn encode<E: Encoder>(&self, encoder: &mut E) -> Result<(), Error> {
        encoder.writer().write(&[*self as u8])
    }
}

impl Encode for i16 {
    fn encode<E: Encoder>(&self, encoder: &mut E) -> Result<(), Error> {
        match E::C::INT_ENCODING {
            IntEncoding::Variable => {
                crate::varint::varint_encode_i16(encoder.writer(), E::C::ENDIAN, *self)
            }
            IntEncoding::Fixed => match E::C::ENDIAN {
                Endianness::Big => encoder.writer().write(&self.to_be_bytes()),
                Endianness::Little => encoder.writer().write(&self.to_le_bytes()),
            },
        }
    }
}

impl Encode for i32 {
    fn encode<E: Encoder>(&self, encoder: &mut E) -> Result<(), Error> {
        match E::C::INT_ENCODING {
            IntEncoding::Variable => {
                crate::varint::varint_encode_i32(encoder.writer(), E::C::ENDIAN, *self)
            }
            IntEncoding::Fixed => match E::C::ENDIAN {
                Endianness::Big => encoder.writer().write(&self.to_be_bytes()),
                Endianness::Little => encoder.writer().write(&self.to_le_bytes()),
            },
        }
    }
}

impl Encode for i64 {
    fn encode<E: Encoder>(&self, encoder: &mut E) -> Result<(), Error> {
        match E::C::INT_ENCODING {
            IntEncoding::Variable => {
                crate::varint::varint_encode_i64(encoder.writer(), E::C::ENDIAN, *self)
            }
            IntEncoding::Fixed => match E::C::ENDIAN {
                Endianness::Big => encoder.writer().write(&self.to_be_bytes()),
                Endianness::Little => encoder.writer().write(&self.to_le_bytes()),
            },
        }
    }
}

impl Encode for i128 {
    fn encode<E: Encoder>(&self, encoder: &mut E) -> Result<(), Error> {
        match E::C::INT_ENCODING {
            IntEncoding::Variable => {
                crate::varint::varint_encode_i128(encoder.writer(), E::C::ENDIAN, *self)
            }
            IntEncoding::Fixed => match E::C::ENDIAN {
                Endianness::Big => encoder.writer().write(&self.to_be_bytes()),
                Endianness::Little => encoder.writer().write(&self.to_le_bytes()),
            },
        }
    }
}

impl Encode for isize {
    fn encode<E: Encoder>(&self, encoder: &mut E) -> Result<(), Error> {
        match E::C::INT_ENCODING {
            IntEncoding::Variable => {
                crate::varint::varint_encode_isize(encoder.writer(), E::C::ENDIAN, *self)
            }
            IntEncoding::Fixed => match E::C::ENDIAN {
                Endianness::Big => encoder.writer().write(&(*self as i64).to_be_bytes()),
                Endianness::Little => encoder.writer().write(&(*self as i64).to_le_bytes()),
            },
        }
    }
}

// ===== Floating Point =====

impl Encode for f32 {
    fn encode<E: Encoder>(&self, encoder: &mut E) -> Result<(), Error> {
        match E::C::ENDIAN {
            Endianness::Big => encoder.writer().write(&self.to_be_bytes()),
            Endianness::Little => encoder.writer().write(&self.to_le_bytes()),
        }
    }
}

impl Encode for f64 {
    fn encode<E: Encoder>(&self, encoder: &mut E) -> Result<(), Error> {
        match E::C::ENDIAN {
            Endianness::Big => encoder.writer().write(&self.to_be_bytes()),
            Endianness::Little => encoder.writer().write(&self.to_le_bytes()),
        }
    }
}

// ===== Character =====

// ===== Arrays =====

impl<T: Encode, const N: usize> Encode for [T; N] {
    fn encode<E: Encoder>(&self, encoder: &mut E) -> Result<(), Error> {
        // Arrays don't encode length (compile-time known)
        for item in self.iter() {
            item.encode(encoder)?;
        }
        Ok(())
    }
}

// ===== Slices =====

impl<T: Encode> Encode for [T] {
    fn encode<E: Encoder>(&self, encoder: &mut E) -> Result<(), Error> {
        // Encode length first
        (self.len() as u64).encode(encoder)?;
        for item in self.iter() {
            item.encode(encoder)?;
        }
        Ok(())
    }
}

// ===== Option =====

impl<T: Encode> Encode for Option<T> {
    fn encode<E: Encoder>(&self, encoder: &mut E) -> Result<(), Error> {
        match self {
            None => 0u8.encode(encoder),
            Some(value) => {
                1u8.encode(encoder)?;
                value.encode(encoder)
            }
        }
    }
}

// ===== Result =====

impl<T: Encode, U: Encode> Encode for Result<T, U> {
    fn encode<E: Encoder>(&self, encoder: &mut E) -> Result<(), Error> {
        match self {
            Ok(value) => {
                0u32.encode(encoder)?;
                value.encode(encoder)
            }
            Err(err) => {
                1u32.encode(encoder)?;
                err.encode(encoder)
            }
        }
    }
}

// ===== Character =====

// ===== Cell & RefCell =====

use core::cell::{Cell, RefCell};

impl<T: Encode + Copy> Encode for Cell<T> {
    fn encode<E: Encoder>(&self, encoder: &mut E) -> Result<(), Error> {
        self.get().encode(encoder)
    }
}

impl<T: Encode> Encode for RefCell<T> {
    fn encode<E: Encoder>(&self, encoder: &mut E) -> Result<(), Error> {
        let value = self.try_borrow().map_err(|_| Error::Custom {
            message: "RefCell already borrowed",
        })?;
        (*value).encode(encoder)
    }
}

// ===== NonZero types =====

use core::num::{
    NonZeroI128, NonZeroI16, NonZeroI32, NonZeroI64, NonZeroI8, NonZeroIsize, NonZeroU128,
    NonZeroU16, NonZeroU32, NonZeroU64, NonZeroU8, NonZeroUsize,
};

macro_rules! impl_encode_nonzero {
    ($nonzero:ty, $inner:ty) => {
        impl Encode for $nonzero {
            fn encode<E: Encoder>(&self, encoder: &mut E) -> Result<(), Error> {
                self.get().encode(encoder)
            }
        }
    };
}

impl_encode_nonzero!(NonZeroU8, u8);
impl_encode_nonzero!(NonZeroU16, u16);
impl_encode_nonzero!(NonZeroU32, u32);
impl_encode_nonzero!(NonZeroU64, u64);
impl_encode_nonzero!(NonZeroU128, u128);
impl_encode_nonzero!(NonZeroUsize, usize);
impl_encode_nonzero!(NonZeroI8, i8);
impl_encode_nonzero!(NonZeroI16, i16);
impl_encode_nonzero!(NonZeroI32, i32);
impl_encode_nonzero!(NonZeroI64, i64);
impl_encode_nonzero!(NonZeroI128, i128);
impl_encode_nonzero!(NonZeroIsize, isize);

// ===== Ordering =====

impl Encode for core::cmp::Ordering {
    fn encode<E: Encoder>(&self, encoder: &mut E) -> Result<(), Error> {
        let val: i8 = match self {
            core::cmp::Ordering::Less => -1,
            core::cmp::Ordering::Equal => 0,
            core::cmp::Ordering::Greater => 1,
        };
        val.encode(encoder)
    }
}

// ===== Infallible =====

impl Encode for core::convert::Infallible {
    fn encode<E: Encoder>(&self, _encoder: &mut E) -> Result<(), Error> {
        // Cannot be reached since Infallible cannot be constructed
        match *self {}
    }
}

// ===== ControlFlow =====

impl<B: Encode, C: Encode> Encode for core::ops::ControlFlow<B, C> {
    fn encode<E: Encoder>(&self, encoder: &mut E) -> Result<(), Error> {
        match self {
            core::ops::ControlFlow::Continue(c) => {
                0u32.encode(encoder)?;
                c.encode(encoder)
            }
            core::ops::ControlFlow::Break(b) => {
                1u32.encode(encoder)?;
                b.encode(encoder)
            }
        }
    }
}

// ===== Wrapping & Reverse =====

use core::cmp::Reverse;
use core::num::Wrapping;

impl<T: Encode> Encode for Wrapping<T> {
    fn encode<E: Encoder>(&self, encoder: &mut E) -> Result<(), Error> {
        self.0.encode(encoder)
    }
}

impl<T: Encode> Encode for Reverse<T> {
    fn encode<E: Encoder>(&self, encoder: &mut E) -> Result<(), Error> {
        self.0.encode(encoder)
    }
}

// ===== Saturating =====

use core::num::Saturating;

impl<T: Encode> Encode for Saturating<T> {
    fn encode<E: Encoder>(&self, encoder: &mut E) -> Result<(), Error> {
        self.0.encode(encoder)
    }
}

// ===== Range types =====

use core::ops::{Bound, Range, RangeFrom, RangeFull, RangeInclusive, RangeTo, RangeToInclusive};

impl<T: Encode> Encode for Range<T> {
    fn encode<E: Encoder>(&self, encoder: &mut E) -> Result<(), Error> {
        self.start.encode(encoder)?;
        self.end.encode(encoder)
    }
}

impl<T: Encode> Encode for RangeInclusive<T> {
    fn encode<E: Encoder>(&self, encoder: &mut E) -> Result<(), Error> {
        self.start().encode(encoder)?;
        self.end().encode(encoder)
    }
}

impl<T: Encode> Encode for Bound<T> {
    fn encode<E: Encoder>(&self, encoder: &mut E) -> Result<(), Error> {
        match self {
            Bound::Unbounded => 0u8.encode(encoder),
            Bound::Included(value) => {
                1u8.encode(encoder)?;
                value.encode(encoder)
            }
            Bound::Excluded(value) => {
                2u8.encode(encoder)?;
                value.encode(encoder)
            }
        }
    }
}

impl Encode for RangeFull {
    fn encode<E: Encoder>(&self, _encoder: &mut E) -> Result<(), Error> {
        Ok(())
    }
}

impl<T: Encode> Encode for RangeFrom<T> {
    fn encode<E: Encoder>(&self, encoder: &mut E) -> Result<(), Error> {
        self.start.encode(encoder)
    }
}

impl<T: Encode> Encode for RangeTo<T> {
    fn encode<E: Encoder>(&self, encoder: &mut E) -> Result<(), Error> {
        self.end.encode(encoder)
    }
}

impl<T: Encode> Encode for RangeToInclusive<T> {
    fn encode<E: Encoder>(&self, encoder: &mut E) -> Result<(), Error> {
        self.end.encode(encoder)
    }
}

// UTF-8 encoding constants (from core::str)
const MAX_ONE_B: u32 = 0x80;
const MAX_TWO_B: u32 = 0x800;
const MAX_THREE_B: u32 = 0x10000;

const TAG_CONT: u8 = 0b1000_0000;
const TAG_TWO_B: u8 = 0b1100_0000;
const TAG_THREE_B: u8 = 0b1110_0000;
const TAG_FOUR_B: u8 = 0b1111_0000;

impl Encode for char {
    /// Encode a char as UTF-8 (bincode compatible)
    ///
    /// UTF-8 encoding uses variable 1-4 bytes:
    /// - 0x0000..0x007F: 1 byte
    /// - 0x0080..0x07FF: 2 bytes
    /// - 0x0800..0xFFFF: 3 bytes (excluding surrogates)
    /// - 0x10000..0x10FFFF: 4 bytes
    fn encode<E: Encoder>(&self, encoder: &mut E) -> Result<(), Error> {
        let code = *self as u32;

        if code < MAX_ONE_B {
            // 1-byte encoding: 0xxxxxxx
            encoder.writer().write(&[code as u8])
        } else if code < MAX_TWO_B {
            // 2-byte encoding: 110xxxxx 10xxxxxx
            encoder.writer().write(&[
                (code >> 6 & 0x1F) as u8 | TAG_TWO_B,
                (code & 0x3F) as u8 | TAG_CONT,
            ])
        } else if code < MAX_THREE_B {
            // 3-byte encoding: 1110xxxx 10xxxxxx 10xxxxxx
            encoder.writer().write(&[
                (code >> 12 & 0x0F) as u8 | TAG_THREE_B,
                (code >> 6 & 0x3F) as u8 | TAG_CONT,
                (code & 0x3F) as u8 | TAG_CONT,
            ])
        } else {
            // 4-byte encoding: 11110xxx 10xxxxxx 10xxxxxx 10xxxxxx
            encoder.writer().write(&[
                (code >> 18 & 0x07) as u8 | TAG_FOUR_B,
                (code >> 12 & 0x3F) as u8 | TAG_CONT,
                (code >> 6 & 0x3F) as u8 | TAG_CONT,
                (code & 0x3F) as u8 | TAG_CONT,
            ])
        }
    }
}

// ===== Duration =====

impl Encode for core::time::Duration {
    fn encode<E: Encoder>(&self, encoder: &mut E) -> Result<(), Error> {
        self.as_secs().encode(encoder)?;
        self.subsec_nanos().encode(encoder)
    }
}

// ===== SystemTime =====

#[cfg(feature = "std")]
impl Encode for std::time::SystemTime {
    fn encode<E: Encoder>(&self, encoder: &mut E) -> Result<(), Error> {
        // Encode as signed i64 seconds + u32 nanos relative to UNIX_EPOCH.
        // Negative seconds represent times before the epoch.
        match self.duration_since(std::time::SystemTime::UNIX_EPOCH) {
            Ok(dur) => {
                // At or after epoch: seconds fits in i64 (u64 secs ≤ i64::MAX in practice)
                let secs = dur.as_secs() as i64;
                let nanos = dur.subsec_nanos();
                secs.encode(encoder)?;
                nanos.encode(encoder)
            }
            Err(err) => {
                // Before epoch: encode as negative seconds
                let dur = err.duration();
                let secs = -(dur.as_secs() as i64) - if dur.subsec_nanos() > 0 { 1 } else { 0 };
                let nanos = if dur.subsec_nanos() > 0 {
                    1_000_000_000u32 - dur.subsec_nanos()
                } else {
                    0u32
                };
                secs.encode(encoder)?;
                nanos.encode(encoder)
            }
        }
    }
}
