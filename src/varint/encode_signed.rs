//! Variable-length encoding for signed integers using zigzag encoding
//!
//! Zigzag encoding maps signed integers to unsigned integers in a way that
//! small absolute values result in small encoded values:
//! - 0 -> 0, -1 -> 1, 1 -> 2, -2 -> 3, 2 -> 4, etc.
//!
//! Branchless formula: `(val << 1) ^ (val >> (BITS - 1))`
//! The arithmetic right shift fills all bits with the sign bit (0 for
//! non-negative, all-ones for negative), so XOR produces zigzag without
//! any conditional branch.

use super::{varint_encode_u128, varint_encode_u16, varint_encode_u32, varint_encode_u64};
use crate::{config::Endianness, encode::Writer, error::Result};

/// Encode an i16 value as a varint using zigzag encoding
#[inline(always)]
pub fn varint_encode_i16<W: Writer>(writer: &mut W, endian: Endianness, val: i16) -> Result<()> {
    // Branchless zigzag: (val << 1) ^ (val >> 15)
    let zigzag = ((val as u16).wrapping_shl(1)) ^ ((val >> 15) as u16);
    varint_encode_u16(writer, endian, zigzag)
}

/// Encode an i32 value as a varint using zigzag encoding
#[inline(always)]
pub fn varint_encode_i32<W: Writer>(writer: &mut W, endian: Endianness, val: i32) -> Result<()> {
    // Branchless zigzag: (val << 1) ^ (val >> 31)
    let zigzag = ((val as u32).wrapping_shl(1)) ^ ((val >> 31) as u32);
    varint_encode_u32(writer, endian, zigzag)
}

/// Encode an i64 value as a varint using zigzag encoding
#[inline(always)]
pub fn varint_encode_i64<W: Writer>(writer: &mut W, endian: Endianness, val: i64) -> Result<()> {
    // Branchless zigzag: (val << 1) ^ (val >> 63)
    let zigzag = ((val as u64).wrapping_shl(1)) ^ ((val >> 63) as u64);
    varint_encode_u64(writer, endian, zigzag)
}

/// Encode an i128 value as a varint using zigzag encoding
#[inline(always)]
pub fn varint_encode_i128<W: Writer>(writer: &mut W, endian: Endianness, val: i128) -> Result<()> {
    // Branchless zigzag: (val << 1) ^ (val >> 127)
    let zigzag = ((val as u128).wrapping_shl(1)) ^ ((val >> 127) as u128);
    varint_encode_u128(writer, endian, zigzag)
}

/// Encode an isize value as a varint (encoded as i64)
#[inline(always)]
pub fn varint_encode_isize<W: Writer>(
    writer: &mut W,
    endian: Endianness,
    val: isize,
) -> Result<()> {
    varint_encode_i64(writer, endian, val as i64)
}
