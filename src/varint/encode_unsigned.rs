//! Variable-length encoding for unsigned integers

use super::{SINGLE_BYTE_MAX, U128_BYTE, U16_BYTE, U32_BYTE, U64_BYTE};
use crate::{config::Endianness, encode::Writer, error::Result};

/// Encode a u16 value as a varint
#[inline(always)]
pub fn varint_encode_u16<W: Writer>(writer: &mut W, endian: Endianness, val: u16) -> Result<()> {
    if val <= SINGLE_BYTE_MAX as u16 {
        writer.write(&[val as u8])
    } else {
        let val_bytes = match endian {
            Endianness::Big => val.to_be_bytes(),
            Endianness::Little => val.to_le_bytes(),
        };
        let buf = [U16_BYTE, val_bytes[0], val_bytes[1]];
        writer.write(&buf)
    }
}

/// Encode a u32 value as a varint
#[inline(always)]
pub fn varint_encode_u32<W: Writer>(writer: &mut W, endian: Endianness, val: u32) -> Result<()> {
    if val <= SINGLE_BYTE_MAX as u32 {
        writer.write(&[val as u8])
    } else if val <= u16::MAX as u32 {
        let val_bytes = match endian {
            Endianness::Big => (val as u16).to_be_bytes(),
            Endianness::Little => (val as u16).to_le_bytes(),
        };
        let buf = [U16_BYTE, val_bytes[0], val_bytes[1]];
        writer.write(&buf)
    } else {
        let val_bytes = match endian {
            Endianness::Big => val.to_be_bytes(),
            Endianness::Little => val.to_le_bytes(),
        };
        let buf = [
            U32_BYTE,
            val_bytes[0],
            val_bytes[1],
            val_bytes[2],
            val_bytes[3],
        ];
        writer.write(&buf)
    }
}

/// Encode a u64 value as a varint
#[inline(always)]
pub fn varint_encode_u64<W: Writer>(writer: &mut W, endian: Endianness, val: u64) -> Result<()> {
    if val <= SINGLE_BYTE_MAX as u64 {
        writer.write(&[val as u8])
    } else if val <= u16::MAX as u64 {
        let val_bytes = match endian {
            Endianness::Big => (val as u16).to_be_bytes(),
            Endianness::Little => (val as u16).to_le_bytes(),
        };
        let buf = [U16_BYTE, val_bytes[0], val_bytes[1]];
        writer.write(&buf)
    } else if val <= u32::MAX as u64 {
        let val_bytes = match endian {
            Endianness::Big => (val as u32).to_be_bytes(),
            Endianness::Little => (val as u32).to_le_bytes(),
        };
        let buf = [
            U32_BYTE,
            val_bytes[0],
            val_bytes[1],
            val_bytes[2],
            val_bytes[3],
        ];
        writer.write(&buf)
    } else {
        let val_bytes = match endian {
            Endianness::Big => val.to_be_bytes(),
            Endianness::Little => val.to_le_bytes(),
        };
        let buf = [
            U64_BYTE,
            val_bytes[0],
            val_bytes[1],
            val_bytes[2],
            val_bytes[3],
            val_bytes[4],
            val_bytes[5],
            val_bytes[6],
            val_bytes[7],
        ];
        writer.write(&buf)
    }
}

/// Encode a u128 value as a varint
#[inline(always)]
pub fn varint_encode_u128<W: Writer>(writer: &mut W, endian: Endianness, val: u128) -> Result<()> {
    if val <= SINGLE_BYTE_MAX as u128 {
        writer.write(&[val as u8])
    } else if val <= u16::MAX as u128 {
        let val_bytes = match endian {
            Endianness::Big => (val as u16).to_be_bytes(),
            Endianness::Little => (val as u16).to_le_bytes(),
        };
        let buf = [U16_BYTE, val_bytes[0], val_bytes[1]];
        writer.write(&buf)
    } else if val <= u32::MAX as u128 {
        let val_bytes = match endian {
            Endianness::Big => (val as u32).to_be_bytes(),
            Endianness::Little => (val as u32).to_le_bytes(),
        };
        let buf = [
            U32_BYTE,
            val_bytes[0],
            val_bytes[1],
            val_bytes[2],
            val_bytes[3],
        ];
        writer.write(&buf)
    } else if val <= u64::MAX as u128 {
        let val_bytes = match endian {
            Endianness::Big => (val as u64).to_be_bytes(),
            Endianness::Little => (val as u64).to_le_bytes(),
        };
        let buf = [
            U64_BYTE,
            val_bytes[0],
            val_bytes[1],
            val_bytes[2],
            val_bytes[3],
            val_bytes[4],
            val_bytes[5],
            val_bytes[6],
            val_bytes[7],
        ];
        writer.write(&buf)
    } else {
        let val_bytes = match endian {
            Endianness::Big => val.to_be_bytes(),
            Endianness::Little => val.to_le_bytes(),
        };
        let buf = [
            U128_BYTE,
            val_bytes[0],
            val_bytes[1],
            val_bytes[2],
            val_bytes[3],
            val_bytes[4],
            val_bytes[5],
            val_bytes[6],
            val_bytes[7],
            val_bytes[8],
            val_bytes[9],
            val_bytes[10],
            val_bytes[11],
            val_bytes[12],
            val_bytes[13],
            val_bytes[14],
            val_bytes[15],
        ];
        writer.write(&buf)
    }
}

/// Encode a usize value as a varint (encoded as u64)
#[inline(always)]
pub fn varint_encode_usize<W: Writer>(
    writer: &mut W,
    endian: Endianness,
    val: usize,
) -> Result<()> {
    varint_encode_u64(writer, endian, val as u64)
}
