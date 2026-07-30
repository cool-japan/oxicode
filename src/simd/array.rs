//! Array encoding and decoding for fixed-width primitive types.
//!
//! Each array is serialized as an 8-byte little-endian element count followed
//! by the elements in little-endian byte order. On little-endian targets that
//! payload is a byte image of the element slice, so encoding and decoding are
//! performed with the vectorized bulk-copy kernels in [`super::copy`] (AVX2 /
//! SSE2 on x86_64, NEON on aarch64, portable copy elsewhere). On big-endian
//! targets each element is byte-swapped individually via the scalar path.
//!
//! The serialized bytes are identical regardless of which path runs, so this
//! module never changes the wire format — it only changes throughput. These
//! functions are an explicit opt-in API; the crate's `encode_to_vec` / derive
//! machinery does not route through them automatically.

use crate::{Error, Result};

#[cfg(feature = "alloc")]
extern crate alloc;

/// Format header for encoded arrays: element count as u64 (little-endian).
const HEADER_SIZE: usize = 8;

/// Generate the four public entry points (Vec encode/decode and buffer
/// encode/decode) plus the shared endianness-aware body helpers for a
/// fixed-width primitive type.
macro_rules! impl_numeric_array_codec {
    (
        elem: $elem:ty,
        zero: $zero:expr,
        encode_vec: $encode_vec:ident,
        encode_into: $encode_into:ident,
        decode_vec: $decode_vec:ident,
        decode_into: $decode_into:ident,
        encode_body: $encode_body:ident,
        decode_body: $decode_body:ident,
    ) => {
        /// Write `data` into `dst` as little-endian bytes.
        ///
        /// `dst` must be at least `data.len() * size_of::<$elem>()` bytes; extra
        /// bytes are left untouched.
        fn $encode_body(data: &[$elem], dst: &mut [u8]) {
            const ELEM: usize = core::mem::size_of::<$elem>();

            #[cfg(target_endian = "little")]
            {
                let byte_len = data.len().saturating_mul(ELEM);
                // SAFETY: reinterpreting a `$elem` slice as bytes is always
                // valid — `u8` has alignment 1 and no invalid bit patterns, and
                // the reinterpreted length matches the source in bytes. The
                // slice is only read.
                let src = unsafe {
                    core::slice::from_raw_parts(data.as_ptr() as *const u8, byte_len)
                };
                crate::simd::copy::copy_bytes(src, dst);
            }

            #[cfg(target_endian = "big")]
            {
                for (index, value) in data.iter().enumerate() {
                    let start = index * ELEM;
                    dst[start..start + ELEM].copy_from_slice(&value.to_le_bytes());
                }
            }
        }

        /// Read little-endian bytes from `src` into `dst`.
        ///
        /// `src` must be at least `dst.len() * size_of::<$elem>()` bytes.
        fn $decode_body(src: &[u8], dst: &mut [$elem]) {
            const ELEM: usize = core::mem::size_of::<$elem>();

            #[cfg(target_endian = "little")]
            {
                let byte_len = dst.len().saturating_mul(ELEM);
                // SAFETY: `dst` holds initialized `$elem` values; reinterpreting
                // it as a mutable byte slice is sound because every bit pattern
                // is valid for `$elem` and `u8` has alignment 1. The length in
                // bytes matches `dst`.
                let out = unsafe {
                    core::slice::from_raw_parts_mut(dst.as_mut_ptr() as *mut u8, byte_len)
                };
                crate::simd::copy::copy_bytes(src, out);
            }

            #[cfg(target_endian = "big")]
            {
                for (index, value) in dst.iter_mut().enumerate() {
                    let start = index * ELEM;
                    let mut bytes = [0u8; ELEM];
                    bytes.copy_from_slice(&src[start..start + ELEM]);
                    *value = <$elem>::from_le_bytes(bytes);
                }
            }
        }

        #[doc = concat!("Encode a `", stringify!($elem), "` array into a new `Vec<u8>`.")]
        #[cfg(feature = "alloc")]
        pub fn $encode_vec(data: &[$elem]) -> Result<alloc::vec::Vec<u8>> {
            const ELEM: usize = core::mem::size_of::<$elem>();

            let byte_len = data.len().checked_mul(ELEM).ok_or(Error::InvalidData {
                message: "array length x element-size overflows usize",
            })?;
            let total = HEADER_SIZE.checked_add(byte_len).ok_or(Error::InvalidData {
                message: "array header length overflows usize",
            })?;

            let mut output = alloc::vec::Vec::with_capacity(total);
            output.extend_from_slice(&(data.len() as u64).to_le_bytes());
            output.resize(total, 0u8);
            $encode_body(data, &mut output[HEADER_SIZE..total]);
            Ok(output)
        }

        #[doc = concat!("Encode a `", stringify!($elem), "` array into `dst`, returning bytes written.")]
        pub fn $encode_into(data: &[$elem], dst: &mut [u8]) -> Result<usize> {
            const ELEM: usize = core::mem::size_of::<$elem>();

            let byte_len = data.len().checked_mul(ELEM).ok_or(Error::InvalidData {
                message: "array length x element-size overflows usize",
            })?;
            let total = HEADER_SIZE.checked_add(byte_len).ok_or(Error::InvalidData {
                message: "array header length overflows usize",
            })?;

            if dst.len() < total {
                return Err(Error::UnexpectedEnd {
                    additional: total - dst.len(),
                });
            }

            dst[..HEADER_SIZE].copy_from_slice(&(data.len() as u64).to_le_bytes());
            $encode_body(data, &mut dst[HEADER_SIZE..total]);
            Ok(total)
        }

        #[doc = concat!("Decode a `", stringify!($elem), "` array from `data` into a new `Vec`.")]
        #[cfg(feature = "alloc")]
        pub fn $decode_vec(data: &[u8]) -> Result<alloc::vec::Vec<$elem>> {
            const ELEM: usize = core::mem::size_of::<$elem>();

            if data.len() < HEADER_SIZE {
                return Err(Error::UnexpectedEnd {
                    additional: HEADER_SIZE - data.len(),
                });
            }

            let raw = u64::from_le_bytes(data[..HEADER_SIZE].try_into().map_err(|_| {
                Error::InvalidData {
                    message: "invalid header bytes",
                }
            })?);
            let count = usize::try_from(raw).map_err(|_| Error::OutsideUsizeRange(raw))?;

            let byte_len = count.checked_mul(ELEM).ok_or(Error::InvalidData {
                message: "array length x element-size overflows usize",
            })?;
            let needed = HEADER_SIZE.checked_add(byte_len).ok_or(Error::InvalidData {
                message: "array header length overflows usize",
            })?;

            if data.len() < needed {
                return Err(Error::UnexpectedEnd {
                    additional: needed - data.len(),
                });
            }

            let mut output = alloc::vec![$zero; count];
            $decode_body(&data[HEADER_SIZE..needed], &mut output);
            Ok(output)
        }

        #[doc = concat!("Decode a `", stringify!($elem), "` array from `src` into `dst`, returning elements decoded.")]
        pub fn $decode_into(src: &[u8], dst: &mut [$elem]) -> Result<usize> {
            const ELEM: usize = core::mem::size_of::<$elem>();

            if src.len() < HEADER_SIZE {
                return Err(Error::UnexpectedEnd {
                    additional: HEADER_SIZE - src.len(),
                });
            }

            let raw = u64::from_le_bytes(src[..HEADER_SIZE].try_into().map_err(|_| {
                Error::InvalidData {
                    message: "invalid header bytes",
                }
            })?);
            let count = usize::try_from(raw).map_err(|_| Error::OutsideUsizeRange(raw))?;

            if dst.len() < count {
                return Err(Error::Custom {
                    message: "destination buffer too small",
                });
            }

            let byte_len = count.checked_mul(ELEM).ok_or(Error::InvalidData {
                message: "array length x element-size overflows usize",
            })?;
            let needed = HEADER_SIZE.checked_add(byte_len).ok_or(Error::InvalidData {
                message: "array header length overflows usize",
            })?;

            if src.len() < needed {
                return Err(Error::UnexpectedEnd {
                    additional: needed - src.len(),
                });
            }

            $decode_body(&src[HEADER_SIZE..needed], &mut dst[..count]);
            Ok(count)
        }
    };
}

impl_numeric_array_codec! {
    elem: f32,
    zero: 0.0f32,
    encode_vec: encode_f32_array,
    encode_into: encode_f32_array_into,
    decode_vec: decode_f32_array,
    decode_into: decode_f32_array_into,
    encode_body: encode_f32_body,
    decode_body: decode_f32_body,
}

impl_numeric_array_codec! {
    elem: f64,
    zero: 0.0f64,
    encode_vec: encode_f64_array,
    encode_into: encode_f64_array_into,
    decode_vec: decode_f64_array,
    decode_into: decode_f64_array_into,
    encode_body: encode_f64_body,
    decode_body: decode_f64_body,
}

impl_numeric_array_codec! {
    elem: i32,
    zero: 0i32,
    encode_vec: encode_i32_array,
    encode_into: encode_i32_array_into,
    decode_vec: decode_i32_array,
    decode_into: decode_i32_array_into,
    encode_body: encode_i32_body,
    decode_body: decode_i32_body,
}

impl_numeric_array_codec! {
    elem: i64,
    zero: 0i64,
    encode_vec: encode_i64_array,
    encode_into: encode_i64_array_into,
    decode_vec: decode_i64_array,
    decode_into: decode_i64_array_into,
    encode_body: encode_i64_body,
    decode_body: decode_i64_body,
}

// =============================================================================
// u8 Array Encoding/Decoding (raw byte copy with header)
// =============================================================================

/// Encode a `u8` array (a raw byte copy prefixed with an element-count header).
#[cfg(feature = "alloc")]
pub fn encode_u8_array(data: &[u8]) -> Result<alloc::vec::Vec<u8>> {
    let total = HEADER_SIZE
        .checked_add(data.len())
        .ok_or(Error::InvalidData {
            message: "array header length overflows usize",
        })?;
    let mut output = alloc::vec::Vec::with_capacity(total);
    output.extend_from_slice(&(data.len() as u64).to_le_bytes());
    output.extend_from_slice(data);
    Ok(output)
}

/// Encode a `u8` array into `dst`, returning bytes written.
pub fn encode_u8_array_into(data: &[u8], dst: &mut [u8]) -> Result<usize> {
    let total = HEADER_SIZE
        .checked_add(data.len())
        .ok_or(Error::InvalidData {
            message: "array header length overflows usize",
        })?;

    if dst.len() < total {
        return Err(Error::UnexpectedEnd {
            additional: total - dst.len(),
        });
    }

    dst[..HEADER_SIZE].copy_from_slice(&(data.len() as u64).to_le_bytes());
    crate::simd::copy::copy_bytes(data, &mut dst[HEADER_SIZE..total]);
    Ok(total)
}

/// Decode a `u8` array from `data` into a new `Vec`.
#[cfg(feature = "alloc")]
pub fn decode_u8_array(data: &[u8]) -> Result<alloc::vec::Vec<u8>> {
    if data.len() < HEADER_SIZE {
        return Err(Error::UnexpectedEnd {
            additional: HEADER_SIZE - data.len(),
        });
    }

    let raw =
        u64::from_le_bytes(
            data[..HEADER_SIZE]
                .try_into()
                .map_err(|_| Error::InvalidData {
                    message: "invalid header bytes",
                })?,
        );
    let count = usize::try_from(raw).map_err(|_| Error::OutsideUsizeRange(raw))?;

    let needed = HEADER_SIZE.checked_add(count).ok_or(Error::InvalidData {
        message: "array header length overflows usize",
    })?;

    if data.len() < needed {
        return Err(Error::UnexpectedEnd {
            additional: needed - data.len(),
        });
    }

    Ok(data[HEADER_SIZE..needed].to_vec())
}

/// Decode a `u8` array from `src` into `dst`, returning bytes decoded.
pub fn decode_u8_array_into(src: &[u8], dst: &mut [u8]) -> Result<usize> {
    if src.len() < HEADER_SIZE {
        return Err(Error::UnexpectedEnd {
            additional: HEADER_SIZE - src.len(),
        });
    }

    let raw =
        u64::from_le_bytes(
            src[..HEADER_SIZE]
                .try_into()
                .map_err(|_| Error::InvalidData {
                    message: "invalid header bytes",
                })?,
        );
    let count = usize::try_from(raw).map_err(|_| Error::OutsideUsizeRange(raw))?;

    if dst.len() < count {
        return Err(Error::Custom {
            message: "destination buffer too small",
        });
    }

    let needed = HEADER_SIZE.checked_add(count).ok_or(Error::InvalidData {
        message: "array header length overflows usize",
    })?;

    if src.len() < needed {
        return Err(Error::UnexpectedEnd {
            additional: needed - src.len(),
        });
    }

    crate::simd::copy::copy_bytes(&src[HEADER_SIZE..needed], &mut dst[..count]);
    Ok(count)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[cfg(feature = "alloc")]
    #[test]
    fn test_f32_roundtrip() {
        let data = alloc::vec![1.0f32, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0, 10.0];
        let encoded = encode_f32_array(&data).expect("encode failed");
        let decoded = decode_f32_array(&encoded).expect("decode failed");
        assert_eq!(data, decoded);
    }

    #[cfg(feature = "alloc")]
    #[test]
    fn test_f64_roundtrip() {
        let data = alloc::vec![1.0f64, 2.0, 3.0, 4.0, 5.0];
        let encoded = encode_f64_array(&data).expect("encode failed");
        let decoded = decode_f64_array(&encoded).expect("decode failed");
        assert_eq!(data, decoded);
    }

    #[cfg(feature = "alloc")]
    #[test]
    fn test_i32_roundtrip() {
        let data = alloc::vec![-100i32, -1, 0, 1, 100, 1000, -1000, 42, 99, 123];
        let encoded = encode_i32_array(&data).expect("encode failed");
        let decoded = decode_i32_array(&encoded).expect("decode failed");
        assert_eq!(data, decoded);
    }

    #[cfg(feature = "alloc")]
    #[test]
    fn test_i64_roundtrip() {
        let data = alloc::vec![-1000i64, 0, 1000, i64::MIN, i64::MAX];
        let encoded = encode_i64_array(&data).expect("encode failed");
        let decoded = decode_i64_array(&encoded).expect("decode failed");
        assert_eq!(data, decoded);
    }

    #[cfg(feature = "alloc")]
    #[test]
    fn test_u8_roundtrip() {
        let data = alloc::vec![0u8, 1, 2, 3, 255, 128, 64, 32];
        let encoded = encode_u8_array(&data).expect("encode failed");
        let decoded = decode_u8_array(&encoded).expect("decode failed");
        assert_eq!(data, decoded);
    }

    #[cfg(feature = "alloc")]
    #[test]
    fn test_empty_array() {
        let data: alloc::vec::Vec<f32> = alloc::vec![];
        let encoded = encode_f32_array(&data).expect("encode failed");
        let decoded = decode_f32_array(&encoded).expect("decode failed");
        assert_eq!(data, decoded);
    }

    #[cfg(feature = "alloc")]
    #[test]
    fn test_large_array() {
        let data: alloc::vec::Vec<f32> = (0..10000).map(|i| i as f32 * 0.01).collect();
        let encoded = encode_f32_array(&data).expect("encode failed");
        let decoded = decode_f32_array(&encoded).expect("decode failed");
        assert_eq!(data, decoded);
    }

    #[test]
    fn test_into_buffer_f32() {
        let data = [1.0f32, 2.0, 3.0, 4.0];
        let mut buffer = [0u8; 100];
        let written = encode_f32_array_into(&data, &mut buffer).expect("encode failed");
        assert_eq!(written, HEADER_SIZE + 16); // 8 + 4 * 4

        let mut output = [0.0f32; 4];
        let count = decode_f32_array_into(&buffer[..written], &mut output).expect("decode failed");
        assert_eq!(count, 4);
        assert_eq!(output, data);
    }

    #[cfg(feature = "alloc")]
    #[test]
    fn test_decode_rejects_overflowing_count() {
        // A count near usize::MAX must be rejected by the checked multiply,
        // not trigger an unchecked-arithmetic panic or a huge allocation.
        let mut header = [0u8; 8];
        header.copy_from_slice(&(u64::MAX).to_le_bytes());
        assert!(decode_f32_array(&header).is_err());
        assert!(decode_f64_array(&header).is_err());
        assert!(decode_i32_array(&header).is_err());
        assert!(decode_i64_array(&header).is_err());
        assert!(decode_u8_array(&header).is_err());
    }

    #[cfg(feature = "alloc")]
    #[test]
    fn test_decode_rejects_power_of_two_wrap() {
        // count = 2^62 makes count * size_of::<f32>() wrap to 0 under a naive
        // (unchecked) multiply; the checked path must reject it.
        let mut header = [0u8; 8];
        header.copy_from_slice(&(1u64 << 62).to_le_bytes());
        assert!(decode_f32_array(&header).is_err());
    }
}
