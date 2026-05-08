//! `BorrowableSliceElement` marker trait for zero-copy slice borrow-decode.
//!
//! This module defines the `BorrowableSliceElement` unsafe marker trait that
//! enables `BorrowDecode<'de> for &'de [T]` for Pod-like primitive types.

use crate::config::Endianness;

/// Marker trait for types that can be borrowed as a slice from an oxicode
/// input buffer, enabling zero-copy `BorrowDecode` of `&'de [Self]`.
///
/// # Safety
///
/// Implementors must guarantee all of the following:
///
/// 1. `Self` has a fixed, well-defined memory layout that does not change
///    between encoder and decoder.
/// 2. Every byte pattern of length `core::mem::size_of::<Self>()` is a
///    valid `Self` value (no validity invariants beyond raw bytes).
/// 3. Under `IntEncoding::Fixed` and a native-endian `Endianness`, the
///    on-disk encoding of `Self` is a verbatim copy of `Self`'s in-memory
///    bytes.
/// 4. `Self: Copy + Sized + 'static` (no interior references, no Drop).
///
/// Violating any invariant causes undefined behavior in the
/// `BorrowDecode<'de> for &'de [Self]` implementation, which uses
/// `core::slice::from_raw_parts` to reinterpret the input buffer.
///
/// # Built-in implementations
///
/// `oxicode` implements this trait for the following types:
/// `u16`, `u32`, `u64`, `i16`, `i32`, `i64`, `f32`, `f64`.
///
/// `u8`, `i8`, and `str` are handled by dedicated concrete `BorrowDecode`
/// implementations — they do **not** implement `BorrowableSliceElement`
/// to avoid conflicting trait implementations.
///
/// `u128`, `i128`, `usize`, `isize`, `bool`, `char`, and composite types
/// are intentionally excluded: 128-bit alignment cannot be guaranteed
/// after oxicode's 8-byte Fixint length prefix; platform-dependent or
/// restricted-bit-pattern types violate invariant 2.
pub unsafe trait BorrowableSliceElement: Sized + Copy + 'static {
    /// Check whether the decoder's endianness is compatible with this type's
    /// in-memory representation.
    ///
    /// Returns `true` for 1-byte types (endianness is irrelevant) or when
    /// `endian` matches the host's native byte order.
    #[inline]
    fn endianness_compatible(endian: Endianness) -> bool {
        if core::mem::size_of::<Self>() <= 1 {
            return true;
        }
        #[cfg(target_endian = "little")]
        {
            endian == Endianness::Little
        }
        #[cfg(target_endian = "big")]
        {
            endian == Endianness::Big
        }
    }
}

unsafe impl BorrowableSliceElement for u16 {}
unsafe impl BorrowableSliceElement for u32 {}
unsafe impl BorrowableSliceElement for u64 {}
unsafe impl BorrowableSliceElement for i16 {}
unsafe impl BorrowableSliceElement for i32 {}
unsafe impl BorrowableSliceElement for i64 {}
unsafe impl BorrowableSliceElement for f32 {}
unsafe impl BorrowableSliceElement for f64 {}
