//! Decode implementations for primitive and standard types

use super::{read::Reader, Decode, Decoder};
use crate::{
    config::{Endianness, IntEncoding, InternalEndianConfig, InternalIntEncodingConfig},
    error::Error,
};
use core::marker::PhantomData;

// ===== Unit and PhantomData =====

impl Decode for () {
    fn decode<D: Decoder<Context = ()>>(_: &mut D) -> Result<Self, Error> {
        Ok(())
    }
}

impl<T: ?Sized> Decode for PhantomData<T> {
    fn decode<D: Decoder<Context = ()>>(_: &mut D) -> Result<Self, Error> {
        Ok(PhantomData)
    }
}

impl<'__de, T: ?Sized> crate::de::BorrowDecode<'__de> for PhantomData<T> {
    fn borrow_decode<D: crate::de::BorrowDecoder<'__de, Context = ()>>(
        _decoder: &mut D,
    ) -> Result<Self, crate::error::Error> {
        Ok(PhantomData)
    }
}

// ===== Boolean =====

impl Decode for bool {
    fn decode<D: Decoder<Context = ()>>(decoder: &mut D) -> Result<Self, Error> {
        match u8::decode(decoder)? {
            0 => Ok(false),
            1 => Ok(true),
            v => Err(Error::InvalidBooleanValue(v)),
        }
    }
}

// ===== Unsigned Integers =====

impl Decode for u8 {
    fn decode<D: Decoder<Context = ()>>(decoder: &mut D) -> Result<Self, Error> {
        let mut bytes = [0u8; 1];
        decoder.reader().read(&mut bytes)?;
        Ok(bytes[0])
    }
}

impl Decode for u16 {
    fn decode<D: Decoder<Context = ()>>(decoder: &mut D) -> Result<Self, Error> {
        match D::C::INT_ENCODING {
            IntEncoding::Variable => {
                crate::varint::varint_decode_u16(decoder.reader(), D::C::ENDIAN)
            }
            IntEncoding::Fixed => {
                let mut bytes = [0u8; 2];
                decoder.reader().read(&mut bytes)?;
                Ok(match D::C::ENDIAN {
                    Endianness::Big => u16::from_be_bytes(bytes),
                    Endianness::Little => u16::from_le_bytes(bytes),
                })
            }
        }
    }
}

impl Decode for u32 {
    fn decode<D: Decoder<Context = ()>>(decoder: &mut D) -> Result<Self, Error> {
        match D::C::INT_ENCODING {
            IntEncoding::Variable => {
                crate::varint::varint_decode_u32(decoder.reader(), D::C::ENDIAN)
            }
            IntEncoding::Fixed => {
                let mut bytes = [0u8; 4];
                decoder.reader().read(&mut bytes)?;
                Ok(match D::C::ENDIAN {
                    Endianness::Big => u32::from_be_bytes(bytes),
                    Endianness::Little => u32::from_le_bytes(bytes),
                })
            }
        }
    }
}

impl Decode for u64 {
    fn decode<D: Decoder<Context = ()>>(decoder: &mut D) -> Result<Self, Error> {
        match D::C::INT_ENCODING {
            IntEncoding::Variable => {
                crate::varint::varint_decode_u64(decoder.reader(), D::C::ENDIAN)
            }
            IntEncoding::Fixed => {
                let mut bytes = [0u8; 8];
                decoder.reader().read(&mut bytes)?;
                Ok(match D::C::ENDIAN {
                    Endianness::Big => u64::from_be_bytes(bytes),
                    Endianness::Little => u64::from_le_bytes(bytes),
                })
            }
        }
    }
}

impl Decode for u128 {
    fn decode<D: Decoder<Context = ()>>(decoder: &mut D) -> Result<Self, Error> {
        match D::C::INT_ENCODING {
            IntEncoding::Variable => {
                crate::varint::varint_decode_u128(decoder.reader(), D::C::ENDIAN)
            }
            IntEncoding::Fixed => {
                let mut bytes = [0u8; 16];
                decoder.reader().read(&mut bytes)?;
                Ok(match D::C::ENDIAN {
                    Endianness::Big => u128::from_be_bytes(bytes),
                    Endianness::Little => u128::from_le_bytes(bytes),
                })
            }
        }
    }
}

impl Decode for usize {
    fn decode<D: Decoder<Context = ()>>(decoder: &mut D) -> Result<Self, Error> {
        match D::C::INT_ENCODING {
            IntEncoding::Variable => {
                crate::varint::varint_decode_usize(decoder.reader(), D::C::ENDIAN)
            }
            IntEncoding::Fixed => {
                let mut bytes = [0u8; 8];
                decoder.reader().read(&mut bytes)?;
                let value = match D::C::ENDIAN {
                    Endianness::Big => u64::from_be_bytes(bytes),
                    Endianness::Little => u64::from_le_bytes(bytes),
                };
                Ok(value as usize)
            }
        }
    }
}

// ===== Signed Integers =====

impl Decode for i8 {
    fn decode<D: Decoder<Context = ()>>(decoder: &mut D) -> Result<Self, Error> {
        let mut bytes = [0u8; 1];
        decoder.reader().read(&mut bytes)?;
        Ok(bytes[0] as i8)
    }
}

impl Decode for i16 {
    fn decode<D: Decoder<Context = ()>>(decoder: &mut D) -> Result<Self, Error> {
        match D::C::INT_ENCODING {
            IntEncoding::Variable => {
                crate::varint::varint_decode_i16(decoder.reader(), D::C::ENDIAN)
            }
            IntEncoding::Fixed => {
                let mut bytes = [0u8; 2];
                decoder.reader().read(&mut bytes)?;
                Ok(match D::C::ENDIAN {
                    Endianness::Big => i16::from_be_bytes(bytes),
                    Endianness::Little => i16::from_le_bytes(bytes),
                })
            }
        }
    }
}

impl Decode for i32 {
    fn decode<D: Decoder<Context = ()>>(decoder: &mut D) -> Result<Self, Error> {
        match D::C::INT_ENCODING {
            IntEncoding::Variable => {
                crate::varint::varint_decode_i32(decoder.reader(), D::C::ENDIAN)
            }
            IntEncoding::Fixed => {
                let mut bytes = [0u8; 4];
                decoder.reader().read(&mut bytes)?;
                Ok(match D::C::ENDIAN {
                    Endianness::Big => i32::from_be_bytes(bytes),
                    Endianness::Little => i32::from_le_bytes(bytes),
                })
            }
        }
    }
}

impl Decode for i64 {
    fn decode<D: Decoder<Context = ()>>(decoder: &mut D) -> Result<Self, Error> {
        match D::C::INT_ENCODING {
            IntEncoding::Variable => {
                crate::varint::varint_decode_i64(decoder.reader(), D::C::ENDIAN)
            }
            IntEncoding::Fixed => {
                let mut bytes = [0u8; 8];
                decoder.reader().read(&mut bytes)?;
                Ok(match D::C::ENDIAN {
                    Endianness::Big => i64::from_be_bytes(bytes),
                    Endianness::Little => i64::from_le_bytes(bytes),
                })
            }
        }
    }
}

impl Decode for i128 {
    fn decode<D: Decoder<Context = ()>>(decoder: &mut D) -> Result<Self, Error> {
        match D::C::INT_ENCODING {
            IntEncoding::Variable => {
                crate::varint::varint_decode_i128(decoder.reader(), D::C::ENDIAN)
            }
            IntEncoding::Fixed => {
                let mut bytes = [0u8; 16];
                decoder.reader().read(&mut bytes)?;
                Ok(match D::C::ENDIAN {
                    Endianness::Big => i128::from_be_bytes(bytes),
                    Endianness::Little => i128::from_le_bytes(bytes),
                })
            }
        }
    }
}

impl Decode for isize {
    fn decode<D: Decoder<Context = ()>>(decoder: &mut D) -> Result<Self, Error> {
        match D::C::INT_ENCODING {
            IntEncoding::Variable => {
                crate::varint::varint_decode_isize(decoder.reader(), D::C::ENDIAN)
            }
            IntEncoding::Fixed => {
                let mut bytes = [0u8; 8];
                decoder.reader().read(&mut bytes)?;
                let value = match D::C::ENDIAN {
                    Endianness::Big => i64::from_be_bytes(bytes),
                    Endianness::Little => i64::from_le_bytes(bytes),
                };
                Ok(value as isize)
            }
        }
    }
}

// ===== Floating Point =====

impl Decode for f32 {
    fn decode<D: Decoder<Context = ()>>(decoder: &mut D) -> Result<Self, Error> {
        let mut bytes = [0u8; 4];
        decoder.reader().read(&mut bytes)?;
        Ok(match D::C::ENDIAN {
            Endianness::Big => f32::from_be_bytes(bytes),
            Endianness::Little => f32::from_le_bytes(bytes),
        })
    }
}

impl Decode for f64 {
    fn decode<D: Decoder<Context = ()>>(decoder: &mut D) -> Result<Self, Error> {
        let mut bytes = [0u8; 8];
        decoder.reader().read(&mut bytes)?;
        Ok(match D::C::ENDIAN {
            Endianness::Big => f64::from_be_bytes(bytes),
            Endianness::Little => f64::from_le_bytes(bytes),
        })
    }
}

// ===== Arrays =====

impl<T: Decode, const N: usize> Decode for [T; N] {
    fn decode<D: Decoder<Context = ()>>(decoder: &mut D) -> Result<Self, Error> {
        // Arrays don't have length prefix (compile-time known)
        // Use array::try_from_fn when stabilized, for now use unsafe
        let mut result: [core::mem::MaybeUninit<T>; N] =
            unsafe { core::mem::MaybeUninit::uninit().assume_init() };

        for item in result.iter_mut() {
            item.write(T::decode(decoder)?);
        }

        // SAFETY: All elements have been initialized
        Ok(unsafe { core::mem::transmute_copy::<_, [T; N]>(&result) })
    }
}

// ===== Option =====

impl<T: Decode> Decode for Option<T> {
    fn decode<D: Decoder<Context = ()>>(decoder: &mut D) -> Result<Self, Error> {
        let variant = u8::decode(decoder)?;
        match variant {
            0 => Ok(None),
            1 => Ok(Some(T::decode(decoder)?)),
            _ => Err(Error::InvalidData {
                message: "Invalid Option variant",
            }),
        }
    }
}

// ===== Result =====

impl<T: Decode, U: Decode> Decode for Result<T, U> {
    fn decode<D: Decoder<Context = ()>>(decoder: &mut D) -> Result<Self, Error> {
        let variant = u32::decode(decoder)?;
        match variant {
            0 => Ok(Ok(T::decode(decoder)?)),
            1 => Ok(Err(U::decode(decoder)?)),
            _ => Err(Error::InvalidData {
                message: "Invalid Result variant",
            }),
        }
    }
}

// ===== Cell & RefCell =====

use core::cell::{Cell, RefCell};

impl<T: Decode + Copy> Decode for Cell<T> {
    fn decode<D: Decoder<Context = ()>>(decoder: &mut D) -> Result<Self, Error> {
        Ok(Cell::new(T::decode(decoder)?))
    }
}

impl<T: Decode> Decode for RefCell<T> {
    fn decode<D: Decoder<Context = ()>>(decoder: &mut D) -> Result<Self, Error> {
        Ok(RefCell::new(T::decode(decoder)?))
    }
}

// ===== NonZero types =====

use core::num::{
    NonZeroI128, NonZeroI16, NonZeroI32, NonZeroI64, NonZeroI8, NonZeroIsize, NonZeroU128,
    NonZeroU16, NonZeroU32, NonZeroU64, NonZeroU8, NonZeroUsize,
};

use crate::error::IntegerType;

macro_rules! impl_decode_nonzero {
    ($nonzero:ty, $inner:ty, $int_type:expr) => {
        impl Decode for $nonzero {
            fn decode<D: Decoder<Context = ()>>(decoder: &mut D) -> Result<Self, Error> {
                let value = <$inner>::decode(decoder)?;
                <$nonzero>::new(value).ok_or(Error::NonZeroTypeIsZero {
                    non_zero_type: $int_type,
                })
            }
        }
    };
}

impl_decode_nonzero!(NonZeroU8, u8, IntegerType::U8);
impl_decode_nonzero!(NonZeroU16, u16, IntegerType::U16);
impl_decode_nonzero!(NonZeroU32, u32, IntegerType::U32);
impl_decode_nonzero!(NonZeroU64, u64, IntegerType::U64);
impl_decode_nonzero!(NonZeroU128, u128, IntegerType::U128);
impl_decode_nonzero!(NonZeroUsize, usize, IntegerType::Usize);
impl_decode_nonzero!(NonZeroI8, i8, IntegerType::I8);
impl_decode_nonzero!(NonZeroI16, i16, IntegerType::I16);
impl_decode_nonzero!(NonZeroI32, i32, IntegerType::I32);
impl_decode_nonzero!(NonZeroI64, i64, IntegerType::I64);
impl_decode_nonzero!(NonZeroI128, i128, IntegerType::I128);
impl_decode_nonzero!(NonZeroIsize, isize, IntegerType::Isize);

// ===== Ordering =====

impl Decode for core::cmp::Ordering {
    fn decode<D: Decoder<Context = ()>>(decoder: &mut D) -> Result<Self, Error> {
        match i8::decode(decoder)? {
            -1 => Ok(core::cmp::Ordering::Less),
            0 => Ok(core::cmp::Ordering::Equal),
            1 => Ok(core::cmp::Ordering::Greater),
            other => Err(Error::UnexpectedVariant {
                type_name: "Ordering",
                found: other as u8 as u32,
            }),
        }
    }
}

// ===== Infallible =====

impl Decode for core::convert::Infallible {
    fn decode<D: Decoder<Context = ()>>(_decoder: &mut D) -> Result<Self, Error> {
        Err(Error::InvalidData {
            message: "Infallible cannot be decoded",
        })
    }
}

// ===== ControlFlow =====

impl<B: Decode, C: Decode> Decode for core::ops::ControlFlow<B, C> {
    fn decode<D: Decoder<Context = ()>>(decoder: &mut D) -> Result<Self, Error> {
        match u32::decode(decoder)? {
            0 => Ok(core::ops::ControlFlow::Continue(C::decode(decoder)?)),
            1 => Ok(core::ops::ControlFlow::Break(B::decode(decoder)?)),
            other => Err(Error::UnexpectedVariant {
                type_name: "ControlFlow",
                found: other,
            }),
        }
    }
}

// ===== Wrapping & Reverse =====

use core::cmp::Reverse;
use core::num::Wrapping;

impl<T: Decode> Decode for Wrapping<T> {
    fn decode<D: Decoder<Context = ()>>(decoder: &mut D) -> Result<Self, Error> {
        Ok(Wrapping(T::decode(decoder)?))
    }
}

impl<T: Decode> Decode for Reverse<T> {
    fn decode<D: Decoder<Context = ()>>(decoder: &mut D) -> Result<Self, Error> {
        Ok(Reverse(T::decode(decoder)?))
    }
}

// ===== Saturating =====

use core::num::Saturating;

impl<T: Decode> Decode for Saturating<T> {
    fn decode<D: Decoder<Context = ()>>(decoder: &mut D) -> Result<Self, Error> {
        Ok(Saturating(T::decode(decoder)?))
    }
}

// ===== Range types =====

use core::ops::{Bound, Range, RangeFrom, RangeFull, RangeInclusive, RangeTo, RangeToInclusive};

impl<T: Decode> Decode for Range<T> {
    fn decode<D: Decoder<Context = ()>>(decoder: &mut D) -> Result<Self, Error> {
        Ok(Range {
            start: T::decode(decoder)?,
            end: T::decode(decoder)?,
        })
    }
}

impl<T: Decode> Decode for RangeInclusive<T> {
    fn decode<D: Decoder<Context = ()>>(decoder: &mut D) -> Result<Self, Error> {
        let start = T::decode(decoder)?;
        let end = T::decode(decoder)?;
        Ok(RangeInclusive::new(start, end))
    }
}

impl<T: Decode> Decode for Bound<T> {
    fn decode<D: Decoder<Context = ()>>(decoder: &mut D) -> Result<Self, Error> {
        let variant = u8::decode(decoder)?;
        match variant {
            0 => Ok(Bound::Unbounded),
            1 => Ok(Bound::Included(T::decode(decoder)?)),
            2 => Ok(Bound::Excluded(T::decode(decoder)?)),
            _ => Err(Error::InvalidData {
                message: "Invalid Bound variant",
            }),
        }
    }
}

impl Decode for RangeFull {
    fn decode<D: Decoder<Context = ()>>(_decoder: &mut D) -> Result<Self, Error> {
        Ok(..)
    }
}

impl<T: Decode> Decode for RangeFrom<T> {
    fn decode<D: Decoder<Context = ()>>(decoder: &mut D) -> Result<Self, Error> {
        let start = T::decode(decoder)?;
        Ok(start..)
    }
}

impl<T: Decode> Decode for RangeTo<T> {
    fn decode<D: Decoder<Context = ()>>(decoder: &mut D) -> Result<Self, Error> {
        let end = T::decode(decoder)?;
        Ok(..end)
    }
}

impl<T: Decode> Decode for RangeToInclusive<T> {
    fn decode<D: Decoder<Context = ()>>(decoder: &mut D) -> Result<Self, Error> {
        let end = T::decode(decoder)?;
        Ok(..=end)
    }
}

// ===== Character =====

// UTF-8 decoding constants
const CONT_MASK: u8 = 0b0011_1111;

impl Decode for char {
    /// Decode a char from UTF-8 (bincode compatible)
    ///
    /// UTF-8 encoding uses variable 1-4 bytes:
    /// - 0xxxxxxx: 1 byte (ASCII)
    /// - 110xxxxx 10xxxxxx: 2 bytes
    /// - 1110xxxx 10xxxxxx 10xxxxxx: 3 bytes
    /// - 11110xxx 10xxxxxx 10xxxxxx 10xxxxxx: 4 bytes
    fn decode<D: Decoder<Context = ()>>(decoder: &mut D) -> Result<Self, Error> {
        // Read the first byte to determine the length
        let first = u8::decode(decoder)?;

        let code = if first < 0x80 {
            // 1-byte: ASCII
            first as u32
        } else if first < 0xE0 {
            // 2-byte: 110xxxxx 10xxxxxx
            let second = u8::decode(decoder)?;
            if !is_continuation_byte(second) {
                return Err(Error::InvalidCharEncoding([first, second, 0, 0]));
            }
            ((first as u32 & 0x1F) << 6) | (second as u32 & CONT_MASK as u32)
        } else if first < 0xF0 {
            // 3-byte: 1110xxxx 10xxxxxx 10xxxxxx
            let second = u8::decode(decoder)?;
            let third = u8::decode(decoder)?;
            if !is_continuation_byte(second) || !is_continuation_byte(third) {
                return Err(Error::InvalidCharEncoding([first, second, third, 0]));
            }
            ((first as u32 & 0x0F) << 12)
                | ((second as u32 & CONT_MASK as u32) << 6)
                | (third as u32 & CONT_MASK as u32)
        } else {
            // 4-byte: 11110xxx 10xxxxxx 10xxxxxx 10xxxxxx
            let second = u8::decode(decoder)?;
            let third = u8::decode(decoder)?;
            let fourth = u8::decode(decoder)?;
            if !is_continuation_byte(second)
                || !is_continuation_byte(third)
                || !is_continuation_byte(fourth)
            {
                return Err(Error::InvalidCharEncoding([first, second, third, fourth]));
            }
            ((first as u32 & 0x07) << 18)
                | ((second as u32 & CONT_MASK as u32) << 12)
                | ((third as u32 & CONT_MASK as u32) << 6)
                | (fourth as u32 & CONT_MASK as u32)
        };

        char::from_u32(code).ok_or(Error::InvalidCharEncoding([
            (code >> 24) as u8,
            (code >> 16) as u8,
            (code >> 8) as u8,
            code as u8,
        ]))
    }
}

/// Check if a byte is a UTF-8 continuation byte (10xxxxxx)
#[inline]
const fn is_continuation_byte(byte: u8) -> bool {
    (byte & 0b1100_0000) == 0b1000_0000
}

// ===== Duration =====

impl Decode for core::time::Duration {
    fn decode<D: Decoder<Context = ()>>(decoder: &mut D) -> Result<Self, Error> {
        let secs = u64::decode(decoder)?;
        let nanos = u32::decode(decoder)?;
        if nanos >= 1_000_000_000 {
            return Err(Error::InvalidData {
                message: "Duration subsec_nanos out of range (must be < 1_000_000_000)",
            });
        }
        Ok(core::time::Duration::new(secs, nanos))
    }
}

// ===== SystemTime =====

#[cfg(feature = "std")]
impl Decode for std::time::SystemTime {
    fn decode<D: Decoder<Context = ()>>(decoder: &mut D) -> Result<Self, Error> {
        let secs = i64::decode(decoder)?;
        let nanos = u32::decode(decoder)?;
        if nanos >= 1_000_000_000 {
            return Err(Error::InvalidData {
                message: "SystemTime subsec_nanos out of range (must be < 1_000_000_000)",
            });
        }
        let epoch = std::time::SystemTime::UNIX_EPOCH;
        if secs >= 0 {
            let dur = std::time::Duration::new(secs as u64, nanos);
            epoch.checked_add(dur).ok_or(Error::InvalidData {
                message: "SystemTime value overflow (too far in the future)",
            })
        } else {
            // Negative secs: time is before epoch.
            // secs is negative, so we need |secs| seconds before epoch minus nanos adjustment.
            let abs_secs = (-(secs + 1)) as u64;
            let remaining_nanos = if nanos == 0 {
                0u32
            } else {
                1_000_000_000u32 - nanos
            };
            // Go back (abs_secs + 1) seconds, then forward (1_000_000_000 - nanos) nanos
            let back = std::time::Duration::new(abs_secs + 1, 0);
            let fwd = std::time::Duration::new(0, remaining_nanos);
            let t = epoch.checked_sub(back).ok_or(Error::InvalidData {
                message: "SystemTime value underflow (too far in the past)",
            })?;
            t.checked_add(fwd).ok_or(Error::InvalidData {
                message: "SystemTime value overflow after nanos adjustment",
            })
        }
    }
}

// ===== BorrowDecode impls for primitives (delegate to Decode) =====
// These allow the BorrowDecode derive macro to work on structs containing
// owned types like u32, u64, bool, f32, etc.

crate::impl_borrow_decode!(());
crate::impl_borrow_decode!(bool);
crate::impl_borrow_decode!(u8);
crate::impl_borrow_decode!(u16);
crate::impl_borrow_decode!(u32);
crate::impl_borrow_decode!(u64);
crate::impl_borrow_decode!(u128);
crate::impl_borrow_decode!(usize);
crate::impl_borrow_decode!(i8);
crate::impl_borrow_decode!(i16);
crate::impl_borrow_decode!(i32);
crate::impl_borrow_decode!(i64);
crate::impl_borrow_decode!(i128);
crate::impl_borrow_decode!(isize);
crate::impl_borrow_decode!(f32);
crate::impl_borrow_decode!(f64);
crate::impl_borrow_decode!(char);
crate::impl_borrow_decode!(core::cmp::Ordering);
crate::impl_borrow_decode!(core::convert::Infallible);

// ===== BorrowDecode impls for NonZero types =====

crate::impl_borrow_decode!(NonZeroU8);
crate::impl_borrow_decode!(NonZeroU16);
crate::impl_borrow_decode!(NonZeroU32);
crate::impl_borrow_decode!(NonZeroU64);
crate::impl_borrow_decode!(NonZeroU128);
crate::impl_borrow_decode!(NonZeroUsize);
crate::impl_borrow_decode!(NonZeroI8);
crate::impl_borrow_decode!(NonZeroI16);
crate::impl_borrow_decode!(NonZeroI32);
crate::impl_borrow_decode!(NonZeroI64);
crate::impl_borrow_decode!(NonZeroI128);
crate::impl_borrow_decode!(NonZeroIsize);

// ===== BorrowDecode for Duration =====

crate::impl_borrow_decode!(core::time::Duration);

// ===== BorrowDecode for SystemTime =====

#[cfg(feature = "std")]
impl<'de> crate::de::BorrowDecode<'de> for std::time::SystemTime {
    fn borrow_decode<D: crate::de::BorrowDecoder<'de, Context = ()>>(
        decoder: &mut D,
    ) -> Result<Self, crate::error::Error> {
        <std::time::SystemTime as crate::de::Decode>::decode(decoder)
    }
}

// ===== BorrowDecode for Wrapping & Reverse =====

impl<'de, T: crate::de::BorrowDecode<'de>> crate::de::BorrowDecode<'de> for Wrapping<T> {
    fn borrow_decode<D: crate::de::BorrowDecoder<'de, Context = ()>>(
        decoder: &mut D,
    ) -> Result<Self, crate::error::Error> {
        Ok(Wrapping(T::borrow_decode(decoder)?))
    }
}

impl<'de, T: crate::de::BorrowDecode<'de>> crate::de::BorrowDecode<'de> for Reverse<T> {
    fn borrow_decode<D: crate::de::BorrowDecoder<'de, Context = ()>>(
        decoder: &mut D,
    ) -> Result<Self, crate::error::Error> {
        Ok(Reverse(T::borrow_decode(decoder)?))
    }
}

// ===== BorrowDecode for Saturating =====

impl<'de, T: crate::de::BorrowDecode<'de>> crate::de::BorrowDecode<'de> for Saturating<T> {
    fn borrow_decode<D: crate::de::BorrowDecoder<'de, Context = ()>>(
        decoder: &mut D,
    ) -> Result<Self, crate::error::Error> {
        Ok(Saturating(T::borrow_decode(decoder)?))
    }
}

// ===== BorrowDecode for Range types =====

impl<'de, T: crate::de::BorrowDecode<'de>> crate::de::BorrowDecode<'de> for core::ops::Range<T> {
    fn borrow_decode<D: crate::de::BorrowDecoder<'de, Context = ()>>(
        decoder: &mut D,
    ) -> Result<Self, crate::error::Error> {
        Ok(core::ops::Range {
            start: T::borrow_decode(decoder)?,
            end: T::borrow_decode(decoder)?,
        })
    }
}

impl<'de, T: crate::de::BorrowDecode<'de>> crate::de::BorrowDecode<'de>
    for core::ops::RangeInclusive<T>
{
    fn borrow_decode<D: crate::de::BorrowDecoder<'de, Context = ()>>(
        decoder: &mut D,
    ) -> Result<Self, crate::error::Error> {
        let start = T::borrow_decode(decoder)?;
        let end = T::borrow_decode(decoder)?;
        Ok(core::ops::RangeInclusive::new(start, end))
    }
}

impl<'de, T: crate::de::BorrowDecode<'de>> crate::de::BorrowDecode<'de> for core::ops::Bound<T> {
    fn borrow_decode<D: crate::de::BorrowDecoder<'de, Context = ()>>(
        decoder: &mut D,
    ) -> Result<Self, crate::error::Error> {
        let variant = u8::decode(decoder)?;
        match variant {
            0 => Ok(core::ops::Bound::Unbounded),
            1 => Ok(core::ops::Bound::Included(T::borrow_decode(decoder)?)),
            2 => Ok(core::ops::Bound::Excluded(T::borrow_decode(decoder)?)),
            _ => Err(crate::error::Error::InvalidData {
                message: "Invalid Bound variant",
            }),
        }
    }
}

impl<'de> crate::de::BorrowDecode<'de> for RangeFull {
    fn borrow_decode<D: crate::de::BorrowDecoder<'de, Context = ()>>(
        decoder: &mut D,
    ) -> Result<Self, crate::error::Error> {
        Decode::decode(decoder)
    }
}

impl<'de, T: crate::de::BorrowDecode<'de>> crate::de::BorrowDecode<'de> for RangeFrom<T> {
    fn borrow_decode<D: crate::de::BorrowDecoder<'de, Context = ()>>(
        decoder: &mut D,
    ) -> Result<Self, crate::error::Error> {
        let start = T::borrow_decode(decoder)?;
        Ok(start..)
    }
}

impl<'de, T: crate::de::BorrowDecode<'de>> crate::de::BorrowDecode<'de> for RangeTo<T> {
    fn borrow_decode<D: crate::de::BorrowDecoder<'de, Context = ()>>(
        decoder: &mut D,
    ) -> Result<Self, crate::error::Error> {
        let end = T::borrow_decode(decoder)?;
        Ok(..end)
    }
}

impl<'de, T: crate::de::BorrowDecode<'de>> crate::de::BorrowDecode<'de> for RangeToInclusive<T> {
    fn borrow_decode<D: crate::de::BorrowDecoder<'de, Context = ()>>(
        decoder: &mut D,
    ) -> Result<Self, crate::error::Error> {
        let end = T::borrow_decode(decoder)?;
        Ok(..=end)
    }
}

// ===== BorrowDecode for Cell & RefCell =====

impl<'de, T: crate::de::BorrowDecode<'de> + Copy> crate::de::BorrowDecode<'de>
    for core::cell::Cell<T>
{
    fn borrow_decode<D: crate::de::BorrowDecoder<'de, Context = ()>>(
        decoder: &mut D,
    ) -> Result<Self, crate::error::Error> {
        Ok(core::cell::Cell::new(T::borrow_decode(decoder)?))
    }
}

impl<'de, T: crate::de::BorrowDecode<'de>> crate::de::BorrowDecode<'de> for core::cell::RefCell<T> {
    fn borrow_decode<D: crate::de::BorrowDecoder<'de, Context = ()>>(
        decoder: &mut D,
    ) -> Result<Self, crate::error::Error> {
        Ok(core::cell::RefCell::new(T::borrow_decode(decoder)?))
    }
}

// ===== BorrowDecode for ControlFlow =====

impl<'__de, B, C> crate::de::BorrowDecode<'__de> for core::ops::ControlFlow<B, C>
where
    B: crate::de::BorrowDecode<'__de>,
    C: crate::de::BorrowDecode<'__de>,
{
    fn borrow_decode<D: crate::de::BorrowDecoder<'__de, Context = ()>>(
        decoder: &mut D,
    ) -> Result<Self, crate::error::Error> {
        let discriminant = u32::decode(decoder)?;
        match discriminant {
            0 => Ok(core::ops::ControlFlow::Continue(C::borrow_decode(decoder)?)),
            1 => Ok(core::ops::ControlFlow::Break(B::borrow_decode(decoder)?)),
            other => Err(crate::error::Error::UnexpectedVariant {
                type_name: "ControlFlow",
                found: other,
            }),
        }
    }
}
