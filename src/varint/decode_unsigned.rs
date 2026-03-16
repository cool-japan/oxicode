//! Variable-length decoding for unsigned integers

use super::{SINGLE_BYTE_MAX, U128_BYTE, U16_BYTE, U32_BYTE, U64_BYTE};
use crate::{
    config::Endianness,
    decode::Reader,
    error::{Error, IntegerType, Result},
};

/// Helper function for invalid varint discriminant errors
#[inline(never)]
#[cold]
fn invalid_varint_discriminant<T>(expected: IntegerType, found: IntegerType) -> Result<T> {
    Err(Error::InvalidIntegerType { expected, found })
}

/// Decode a u16 value from a varint
#[inline(always)]
pub fn varint_decode_u16<R: Reader>(reader: &mut R, endian: Endianness) -> Result<u16> {
    let mut bytes = [0u8; 1];
    reader.read(&mut bytes)?;
    match bytes[0] {
        byte @ 0..=SINGLE_BYTE_MAX => Ok(byte as u16),
        U16_BYTE => {
            let mut buf = [0u8; 2];
            reader.read(&mut buf)?;
            Ok(match endian {
                Endianness::Big => u16::from_be_bytes(buf),
                Endianness::Little => u16::from_le_bytes(buf),
            })
        }
        U32_BYTE => invalid_varint_discriminant(IntegerType::U16, IntegerType::U32),
        U64_BYTE => invalid_varint_discriminant(IntegerType::U16, IntegerType::U64),
        U128_BYTE => invalid_varint_discriminant(IntegerType::U16, IntegerType::U128),
        _ => invalid_varint_discriminant(IntegerType::U16, IntegerType::Reserved),
    }
}

/// Decode a u32 value from a varint
#[inline(always)]
pub fn varint_decode_u32<R: Reader>(reader: &mut R, endian: Endianness) -> Result<u32> {
    let mut bytes = [0u8; 1];
    reader.read(&mut bytes)?;
    match bytes[0] {
        byte @ 0..=SINGLE_BYTE_MAX => Ok(byte as u32),
        U16_BYTE => {
            let mut buf = [0u8; 2];
            reader.read(&mut buf)?;
            Ok(match endian {
                Endianness::Big => u16::from_be_bytes(buf) as u32,
                Endianness::Little => u16::from_le_bytes(buf) as u32,
            })
        }
        U32_BYTE => {
            let mut buf = [0u8; 4];
            reader.read(&mut buf)?;
            Ok(match endian {
                Endianness::Big => u32::from_be_bytes(buf),
                Endianness::Little => u32::from_le_bytes(buf),
            })
        }
        U64_BYTE => invalid_varint_discriminant(IntegerType::U32, IntegerType::U64),
        U128_BYTE => invalid_varint_discriminant(IntegerType::U32, IntegerType::U128),
        _ => invalid_varint_discriminant(IntegerType::U32, IntegerType::Reserved),
    }
}

/// Decode a u64 value from a varint
#[inline(always)]
pub fn varint_decode_u64<R: Reader>(reader: &mut R, endian: Endianness) -> Result<u64> {
    let mut bytes = [0u8; 1];
    reader.read(&mut bytes)?;
    match bytes[0] {
        byte @ 0..=SINGLE_BYTE_MAX => Ok(byte as u64),
        U16_BYTE => {
            let mut buf = [0u8; 2];
            reader.read(&mut buf)?;
            Ok(match endian {
                Endianness::Big => u16::from_be_bytes(buf) as u64,
                Endianness::Little => u16::from_le_bytes(buf) as u64,
            })
        }
        U32_BYTE => {
            let mut buf = [0u8; 4];
            reader.read(&mut buf)?;
            Ok(match endian {
                Endianness::Big => u32::from_be_bytes(buf) as u64,
                Endianness::Little => u32::from_le_bytes(buf) as u64,
            })
        }
        U64_BYTE => {
            let mut buf = [0u8; 8];
            reader.read(&mut buf)?;
            Ok(match endian {
                Endianness::Big => u64::from_be_bytes(buf),
                Endianness::Little => u64::from_le_bytes(buf),
            })
        }
        U128_BYTE => invalid_varint_discriminant(IntegerType::U64, IntegerType::U128),
        _ => invalid_varint_discriminant(IntegerType::U64, IntegerType::Reserved),
    }
}

/// Decode a u128 value from a varint
#[inline(always)]
pub fn varint_decode_u128<R: Reader>(reader: &mut R, endian: Endianness) -> Result<u128> {
    let mut bytes = [0u8; 1];
    reader.read(&mut bytes)?;
    match bytes[0] {
        byte @ 0..=SINGLE_BYTE_MAX => Ok(byte as u128),
        U16_BYTE => {
            let mut buf = [0u8; 2];
            reader.read(&mut buf)?;
            Ok(match endian {
                Endianness::Big => u16::from_be_bytes(buf) as u128,
                Endianness::Little => u16::from_le_bytes(buf) as u128,
            })
        }
        U32_BYTE => {
            let mut buf = [0u8; 4];
            reader.read(&mut buf)?;
            Ok(match endian {
                Endianness::Big => u32::from_be_bytes(buf) as u128,
                Endianness::Little => u32::from_le_bytes(buf) as u128,
            })
        }
        U64_BYTE => {
            let mut buf = [0u8; 8];
            reader.read(&mut buf)?;
            Ok(match endian {
                Endianness::Big => u64::from_be_bytes(buf) as u128,
                Endianness::Little => u64::from_le_bytes(buf) as u128,
            })
        }
        U128_BYTE => {
            let mut buf = [0u8; 16];
            reader.read(&mut buf)?;
            Ok(match endian {
                Endianness::Big => u128::from_be_bytes(buf),
                Endianness::Little => u128::from_le_bytes(buf),
            })
        }
        _ => invalid_varint_discriminant(IntegerType::U128, IntegerType::Reserved),
    }
}

/// Decode a usize value from a varint (encoded as u64)
#[inline(always)]
pub fn varint_decode_usize<R: Reader>(reader: &mut R, endian: Endianness) -> Result<usize> {
    let value = varint_decode_u64(reader, endian)?;

    // On 64-bit platforms usize::MAX == u64::MAX, so no overflow is possible.
    // On 32-bit (or narrower) platforms we need the bounds check.
    #[cfg(target_pointer_width = "64")]
    {
        Ok(value as usize)
    }
    #[cfg(not(target_pointer_width = "64"))]
    {
        if value > usize::MAX as u64 {
            return Err(Error::InvalidData {
                message: "usize value too large for this platform",
            });
        }
        Ok(value as usize)
    }
}
