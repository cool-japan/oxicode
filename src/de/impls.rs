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
        decoder.claim_bytes_read(1)?;
        let mut bytes = [0u8; 1];
        decoder.reader().read(&mut bytes)?;
        Ok(bytes[0])
    }
}

impl Decode for u16 {
    fn decode<D: Decoder<Context = ()>>(decoder: &mut D) -> Result<Self, Error> {
        decoder.claim_bytes_read(2)?;
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
        decoder.claim_bytes_read(4)?;
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
        decoder.claim_bytes_read(8)?;
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
        decoder.claim_bytes_read(16)?;
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
        decoder.claim_bytes_read(8)?;
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
                usize::try_from(value).map_err(|_| Error::OutsideUsizeRange(value))
            }
        }
    }
}

// ===== Signed Integers =====

impl Decode for i8 {
    fn decode<D: Decoder<Context = ()>>(decoder: &mut D) -> Result<Self, Error> {
        decoder.claim_bytes_read(1)?;
        let mut bytes = [0u8; 1];
        decoder.reader().read(&mut bytes)?;
        Ok(bytes[0] as i8)
    }
}

impl Decode for i16 {
    fn decode<D: Decoder<Context = ()>>(decoder: &mut D) -> Result<Self, Error> {
        decoder.claim_bytes_read(2)?;
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
        decoder.claim_bytes_read(4)?;
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
        decoder.claim_bytes_read(8)?;
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
        decoder.claim_bytes_read(16)?;
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
        decoder.claim_bytes_read(8)?;
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
                isize::try_from(value).map_err(|_| Error::OutsideUsizeRange(value as u64))
            }
        }
    }
}

// ===== Floating Point =====

impl Decode for f32 {
    fn decode<D: Decoder<Context = ()>>(decoder: &mut D) -> Result<Self, Error> {
        decoder.claim_bytes_read(4)?;
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
        decoder.claim_bytes_read(8)?;
        let mut bytes = [0u8; 8];
        decoder.reader().read(&mut bytes)?;
        Ok(match D::C::ENDIAN {
            Endianness::Big => f64::from_be_bytes(bytes),
            Endianness::Little => f64::from_le_bytes(bytes),
        })
    }
}

// ===== Arrays =====

/// Decode `N` elements into an array using `decode_elem`, dropping any
/// already-initialized elements if a later element fails.
///
/// This mirrors bincode 2.0.1's `impl_core::collect_into_array` drop guard:
/// on an early return (element decode error) the initialized prefix is dropped
/// via `ptr::drop_in_place`, preventing a memory leak for element types with a
/// non-trivial `Drop`. On full success the guard is forgotten and the array is
/// moved out.
#[inline]
fn decode_array_with<T, const N: usize, F>(mut decode_elem: F) -> Result<[T; N], Error>
where
    F: FnMut() -> Result<T, Error>,
{
    use core::mem::MaybeUninit;

    struct Guard<'a, T, const N: usize> {
        array: &'a mut [MaybeUninit<T>; N],
        initialized: usize,
    }

    impl<T, const N: usize> Drop for Guard<'_, T, N> {
        fn drop(&mut self) {
            // SAFETY: the first `initialized` slots are guaranteed initialized,
            // and `initialized <= N`, so this slice reference is in bounds and
            // points only at live `T` values.
            for slot in self.array[..self.initialized].iter_mut() {
                unsafe { core::ptr::drop_in_place(slot.as_mut_ptr()) };
            }
        }
    }

    // SAFETY: an array of `MaybeUninit<T>` requires no initialization.
    let mut array: [MaybeUninit<T>; N] =
        unsafe { MaybeUninit::<[MaybeUninit<T>; N]>::uninit().assume_init() };
    let mut guard = Guard {
        array: &mut array,
        initialized: 0,
    };

    while guard.initialized < N {
        let value = decode_elem()?;
        guard.array[guard.initialized].write(value);
        guard.initialized += 1;
    }

    // All N elements initialized: defuse the guard so it does not drop them,
    // then move the array out.
    core::mem::forget(guard);
    // SAFETY: every slot was initialized by the loop above. `transmute_copy`
    // bit-copies the `[MaybeUninit<T>; N]` into `[T; N]`; the source is not
    // dropped (MaybeUninit has no drop glue), so no double-free occurs.
    Ok(unsafe { core::mem::transmute_copy::<_, [T; N]>(&array) })
}

impl<T: Decode, const N: usize> Decode for [T; N] {
    fn decode<D: Decoder<Context = ()>>(decoder: &mut D) -> Result<Self, Error> {
        // Arrays have no length prefix (compile-time known length).
        // Reserve the full array's worth of bytes up front, then reclaim one
        // element's reservation before each element decode so the per-element
        // `claim_bytes_read` calls do not double-count (mirrors bincode 2.0.1).
        decoder.claim_bytes_read(core::mem::size_of::<[T; N]>())?;
        if unty::type_equal::<T, u8>() {
            let mut buf = [0u8; N];
            decoder.reader().read(&mut buf)?;
            // SAFETY: `unty::type_equal::<T, u8>()` proved that `T` is exactly
            // `u8`, so `[u8; N]` and `[T; N]` have identical layout. Reading
            // through the cast pointer moves the bytes out as `[T; N]`.
            let ptr = (&mut buf as *mut [u8; N]).cast::<[T; N]>();
            Ok(unsafe { ptr.read() })
        } else {
            decode_array_with::<T, N, _>(|| {
                decoder.unclaim_bytes_read(core::mem::size_of::<T>());
                T::decode(decoder)
            })
        }
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

impl<T: Decode> Decode for Cell<T> {
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

/// UTF-8 first-byte width table (a copy of `core::str::UTF8_CHAR_WIDTH`).
///
/// A width of `0` marks a byte that can never legally start a UTF-8 sequence
/// (continuation bytes `0x80..=0xBF`, the overlong-only leads `0xC0`/`0xC1`,
/// and `0xF5..=0xFF`). This mirrors bincode 2.0.1 exactly so that the same
/// malformed byte streams are rejected.
const UTF8_CHAR_WIDTH: [u8; 256] = [
    1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1,
    1, // 0x1F
    1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1,
    1, // 0x3F
    1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1,
    1, // 0x5F
    1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1,
    1, // 0x7F
    // Continuation bytes 0x80..=0xBF are invalid as a leading byte (width 0).
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, // 0x9F
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, // 0xBF
    // 0xC0/0xC1 are overlong-only leads (width 0); 0xC2..=0xDF are 2-byte leads.
    0, 0, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2,
    2, // 0xDF
    3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, // 0xEF
    // 0xF5..=0xFF are invalid (width 0).
    4, 4, 4, 4, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, // 0xFF
];

/// Return the number of bytes a UTF-8 sequence starting with `byte` occupies,
/// or `0` if `byte` is not a valid UTF-8 leading byte.
#[inline]
const fn utf8_char_width(byte: u8) -> usize {
    UTF8_CHAR_WIDTH[byte as usize] as usize
}

impl Decode for char {
    /// Decode a char from UTF-8 (bincode compatible)
    ///
    /// The first byte selects the sequence width via the internal `UTF8_CHAR_WIDTH`
    /// lookup table. A width of `0` (continuation byte, overlong lead `0xC0`/`0xC1`, or
    /// `0xF5..=0xFF`) is rejected immediately, and the assembled bytes are
    /// validated with [`core::str::from_utf8`], which rejects overlong and
    /// ill-formed sequences that a naive bit-assembly would accept.
    fn decode<D: Decoder<Context = ()>>(decoder: &mut D) -> Result<Self, Error> {
        let mut array = [0u8; 4];

        // Read the first byte to determine the sequence width.
        decoder.reader().read(&mut array[..1])?;

        let width = utf8_char_width(array[0]);
        if width == 0 {
            return Err(Error::InvalidCharEncoding(array));
        }
        // Normally we claim before reading, but the number of bytes a `char`
        // consumes varies (1..=4), so we claim the resolved width here after
        // inspecting the leading byte. This mirrors bincode 2.0.1.
        decoder.claim_bytes_read(width)?;
        if width == 1 {
            return Ok(array[0] as char);
        }

        // Read the remaining continuation bytes, then validate the whole
        // slice as UTF-8. `str::from_utf8` rejects overlong encodings and any
        // ill-formed continuation bytes.
        decoder.reader().read(&mut array[1..width])?;
        core::str::from_utf8(&array[..width])
            .ok()
            .and_then(|s| s.chars().next())
            .ok_or(Error::InvalidCharEncoding(array))
    }
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

impl<'de, T: crate::de::BorrowDecode<'de>> crate::de::BorrowDecode<'de> for core::cell::Cell<T> {
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

// ===== BorrowDecode for arrays =====

impl<'de, T, const N: usize> crate::de::BorrowDecode<'de> for [T; N]
where
    T: crate::de::BorrowDecode<'de>,
{
    fn borrow_decode<D: crate::de::BorrowDecoder<'de, Context = ()>>(
        decoder: &mut D,
    ) -> Result<Self, crate::error::Error> {
        // Arrays have no length prefix (compile-time known length).
        decoder.claim_bytes_read(core::mem::size_of::<[T; N]>())?;
        if unty::type_equal::<T, u8>() {
            let mut buf = [0u8; N];
            decoder.reader().read(&mut buf)?;
            // SAFETY: `unty::type_equal::<T, u8>()` proved that `T` is exactly
            // `u8`, so `[u8; N]` and `[T; N]` have identical layout. Reading
            // through the cast pointer moves the bytes out as `[T; N]`.
            let ptr = (&mut buf as *mut [u8; N]).cast::<[T; N]>();
            Ok(unsafe { ptr.read() })
        } else {
            decode_array_with::<T, N, _>(|| {
                decoder.unclaim_bytes_read(core::mem::size_of::<T>());
                T::borrow_decode(decoder)
            })
        }
    }
}

// ===== BorrowDecode for Result =====

impl<'de, T, U> crate::de::BorrowDecode<'de> for Result<T, U>
where
    T: crate::de::BorrowDecode<'de>,
    U: crate::de::BorrowDecode<'de>,
{
    fn borrow_decode<D: crate::de::BorrowDecoder<'de, Context = ()>>(
        decoder: &mut D,
    ) -> Result<Self, crate::error::Error> {
        let variant = u32::decode(decoder)?;
        match variant {
            0 => Ok(Ok(T::borrow_decode(decoder)?)),
            1 => Ok(Err(U::borrow_decode(decoder)?)),
            _ => Err(Error::InvalidData {
                message: "Invalid Result variant",
            }),
        }
    }
}

// ===== Allocation-free zero-copy BorrowDecode impls =====
//
// These impls perform no allocation, so they live in the core `de::impls`
// module (compiled even for `no_std` without the `alloc` feature) rather than
// in `features::impl_alloc`. That keeps zero-copy borrowing of `&str`, `&[u8]`,
// `&[i8]`, `&[T]` and `Option<T>` available in a `no_std`-without-`alloc` build,
// matching the availability of their `Decode`/`Encode` counterparts.

// ----- BorrowDecode for Option<T> -----

impl<'de, T> crate::de::BorrowDecode<'de> for Option<T>
where
    T: crate::de::BorrowDecode<'de>,
{
    fn borrow_decode<D: crate::de::BorrowDecoder<'de, Context = ()>>(
        decoder: &mut D,
    ) -> Result<Self, Error> {
        let variant = u8::decode(decoder)?;
        match variant {
            0 => Ok(None),
            1 => Ok(Some(T::borrow_decode(decoder)?)),
            _ => Err(Error::InvalidData {
                message: "Invalid Option variant",
            }),
        }
    }
}

// ----- BorrowDecode for &[u8] (zero-copy) -----

impl<'de> crate::de::BorrowDecode<'de> for &'de [u8] {
    fn borrow_decode<D: crate::de::BorrowDecoder<'de, Context = ()>>(
        decoder: &mut D,
    ) -> Result<Self, Error> {
        use crate::de::BorrowReader;
        let len = crate::de::decode_slice_len(decoder)?;
        decoder.claim_bytes_read(len)?;

        let bytes = decoder.borrow_reader().take_bytes(len)?;
        Ok(bytes)
    }
}

// ----- BorrowDecode for &str (zero-copy) -----

impl<'de> crate::de::BorrowDecode<'de> for &'de str {
    fn borrow_decode<D: crate::de::BorrowDecoder<'de, Context = ()>>(
        decoder: &mut D,
    ) -> Result<Self, Error> {
        use crate::de::BorrowReader;
        let len = crate::de::decode_slice_len(decoder)?;
        decoder.claim_bytes_read(len)?;

        let bytes = decoder.borrow_reader().take_bytes(len)?;

        // Validate UTF-8. Use the `From<Utf8Error>` conversion (not the
        // alloc-gated `Error::Utf8` variant directly) so this core impl also
        // compiles for `no_std` without the `alloc` feature.
        core::str::from_utf8(bytes).map_err(Error::from)
    }
}

// ----- BorrowDecode for &[i8] (zero-copy signed bytes) -----

/// Zero-copy decode for `&[i8]`.
///
/// Since `i8` and `u8` have identical bit patterns, we can reinterpret
/// the `&[u8]` taken from the input buffer as `&[i8]` using a safe transmute.
/// This is valid because:
/// 1. `i8` has the same size and alignment as `u8`
/// 2. All bit patterns are valid for `i8`
impl<'de> crate::de::BorrowDecode<'de> for &'de [i8] {
    fn borrow_decode<D: crate::de::BorrowDecoder<'de, Context = ()>>(
        decoder: &mut D,
    ) -> Result<Self, Error> {
        use crate::de::BorrowReader;
        let len = crate::de::decode_slice_len(decoder)?;
        decoder.claim_bytes_read(len)?;

        let bytes: &'de [u8] = decoder.borrow_reader().take_bytes(len)?;

        // SAFETY: &[u8] and &[i8] have the same size, alignment, and all bit
        // patterns are valid. The lifetime 'de is preserved correctly.
        let signed_bytes: &'de [i8] =
            unsafe { core::slice::from_raw_parts(bytes.as_ptr() as *const i8, bytes.len()) };
        Ok(signed_bytes)
    }
}

// ----- BorrowDecode for &[T] where T: BorrowableSliceElement (zero-copy Pod) -----

/// Zero-copy borrow-decode for `&'de [T]` where `T` is a Pod-like primitive.
///
/// Reinterprets a contiguous byte slice from the input buffer as `&'de [T]`
/// without copying. Three runtime guards ensure soundness:
///
/// 1. **Encoding gate** — `IntEncoding::Fixed` only; Varint compresses
///    elements and breaks the in-memory layout identity.
/// 2. **Endianness gate** — decoder endianness must match the host's native
///    byte order for any `T` wider than one byte.
/// 3. **Alignment gate** — `take_bytes`'s result must be aligned to
///    `align_of::<T>()`.
///
/// All gates produce `Error::InvalidData` with a descriptive message on
/// failure, making the error actionable (switch to `Vec<T>`, fix config, etc.).
impl<'de, T> crate::de::BorrowDecode<'de> for &'de [T]
where
    T: crate::de::BorrowableSliceElement,
{
    fn borrow_decode<D: crate::de::BorrowDecoder<'de, Context = ()>>(
        decoder: &mut D,
    ) -> Result<Self, Error> {
        use crate::config::Config;
        use crate::de::BorrowReader;

        let len = crate::de::decode_slice_len(decoder)?;
        decoder.claim_container_read::<T>(len)?;

        // Encoding gate: only Fixint produces verbatim in-memory layout per element.
        let cfg = decoder.config();
        if cfg.int_encoding() != IntEncoding::Fixed {
            return Err(Error::InvalidData {
                message: "borrow-decode of &[T] requires IntEncoding::Fixed; \
                          varint-encoded payloads do not match in-memory layout",
            });
        }

        // Endianness gate: the buffer bytes must match the host's native byte order.
        if !T::endianness_compatible(cfg.endianness()) {
            return Err(Error::InvalidData {
                message: "borrow-decode of &[T] requires native endianness; \
                          use Vec<T> for cross-endian decoding",
            });
        }

        let elem_size = core::mem::size_of::<T>();
        let byte_count = len.checked_mul(elem_size).ok_or(Error::InvalidData {
            message: "borrow-decode of &[T]: length × element-size overflows usize",
        })?;

        let bytes = decoder.borrow_reader().take_bytes(byte_count)?;

        // Alignment gate: pointer must satisfy align_of::<T>().
        let align = core::mem::align_of::<T>();
        if (bytes.as_ptr() as usize) % align != 0 {
            return Err(Error::InvalidData {
                message: "borrow-decode of &[T]: buffer is not aligned to align_of::<T>(); \
                          use Vec<T> instead",
            });
        }

        // SAFETY:
        // - `bytes` is `&'de [u8]` with length exactly `len * size_of::<T>()`.
        // - `bytes.as_ptr()` is aligned to `align_of::<T>()` (checked above).
        // - Encoding is Fixed and endianness is native (checked above), so
        //   the bytes are a verbatim copy of `[T; len]` in host memory.
        // - `T: BorrowableSliceElement` guarantees: `T: Copy + Sized + 'static`,
        //   all bit patterns are valid, layout is fixed and known.
        // - Lifetime `'de` of `bytes` is correctly propagated.
        let slice: &'de [T] =
            unsafe { core::slice::from_raw_parts(bytes.as_ptr() as *const T, len) };
        Ok(slice)
    }
}
