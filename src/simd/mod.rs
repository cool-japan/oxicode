//! Opt-in vectorized array encoding and decoding for oxicode.
//!
//! This module is an **explicit, opt-in** codec for contiguous arrays of a few
//! fixed-width primitive types (`f32`, `f64`, `i32`, `i64`, `u8`). It is *not*
//! wired into [`crate::encode_to_vec`], the derive macro, or the `Encode` /
//! `Decode` traits — callers must invoke these functions directly. It has its
//! own self-describing framing (an 8-byte little-endian element count followed
//! by the little-endian element bytes) that is independent of the crate's main
//! wire format.
//!
//! ## What the "SIMD" actually is
//!
//! Each array's payload is, on little-endian targets, a byte image of the
//! element slice. Encoding and decoding therefore reduce to a bulk memory copy,
//! which this module performs with real hardware SIMD kernels (see the
//! internal `copy` module): AVX2 or the SSE2 baseline on x86_64, NEON on aarch64, and a
//! portable `copy_from_slice` fallback everywhere else (and under Miri). On
//! big-endian targets each element is byte-swapped individually on the scalar
//! path. The serialized bytes are identical on every architecture — only
//! throughput differs.
//!
//! Under `std`, the widest usable instruction set is detected at runtime via
//! [`detect_capability`]. Under `no_std` (the `simd` feature without `std`),
//! runtime feature detection is unavailable, so dispatch falls back to
//! compile-time `target_feature` selection.
//!
//! ## Example
//!
//! ```rust,ignore
//! use oxicode::simd;
//!
//! let data: Vec<f32> = vec![1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0];
//! let encoded = simd::encode_simd_array(&data)?;
//! let decoded: Vec<f32> = simd::decode_simd_array(&encoded)?;
//! ```

mod aligned;
mod array;
mod copy;
mod detect;

#[cfg(feature = "alloc")]
pub use aligned::AlignedVec;
pub use aligned::{AlignedBuffer, SIMD_ALIGNMENT};
#[cfg(feature = "alloc")]
pub use array::{
    decode_f32_array, decode_f64_array, decode_i32_array, decode_i64_array, decode_u8_array,
    encode_f32_array, encode_f64_array, encode_i32_array, encode_i64_array, encode_u8_array,
};
pub use detect::{detect_capability, is_simd_available, optimal_alignment, SimdCapability};

use crate::Result;

/// Encode a slice of primitives using SIMD-optimized path when available.
///
/// This is the main entry point for SIMD-accelerated encoding. It automatically
/// dispatches to the appropriate SIMD implementation based on detected CPU capabilities.
///
/// # Type Support
///
/// Currently supports:
/// - `f32`, `f64` - Floating point types
/// - `i32`, `i64` - Signed integers
/// - `u8` - Byte arrays
///
/// # Example
///
/// ```rust,ignore
/// use oxicode::simd::encode_simd_array;
///
/// let floats: &[f32] = &[1.0, 2.0, 3.0, 4.0];
/// let encoded = encode_simd_array(floats)?;
/// ```
#[cfg(feature = "alloc")]
pub fn encode_simd_array<T: SimdEncodable>(data: &[T]) -> Result<alloc::vec::Vec<u8>> {
    T::encode_simd(data)
}

/// Decode a byte slice into a Vec of primitives using SIMD-optimized path when available.
///
/// This is the main entry point for SIMD-accelerated decoding.
///
/// # Type Support
///
/// Currently supports:
/// - `f32`, `f64` - Floating point types
/// - `i32`, `i64` - Signed integers
/// - `u8` - Byte arrays
///
/// # Example
///
/// ```rust,ignore
/// use oxicode::simd::decode_simd_array;
///
/// let encoded: &[u8] = &[...];
/// let floats: Vec<f32> = decode_simd_array(encoded)?;
/// ```
#[cfg(feature = "alloc")]
pub fn decode_simd_array<T: SimdDecodable>(data: &[u8]) -> Result<alloc::vec::Vec<T>> {
    T::decode_simd(data)
}

/// Trait for types that can be encoded using SIMD optimization.
pub trait SimdEncodable: Sized + Copy {
    /// Encode a slice of this type using SIMD when available.
    #[cfg(feature = "alloc")]
    fn encode_simd(data: &[Self]) -> Result<alloc::vec::Vec<u8>>;

    /// Encode into an existing buffer, returning bytes written.
    fn encode_simd_into(data: &[Self], dst: &mut [u8]) -> Result<usize>;
}

/// Trait for types that can be decoded using SIMD optimization.
pub trait SimdDecodable: Sized + Copy {
    /// Decode a slice into a Vec of this type using SIMD when available.
    #[cfg(feature = "alloc")]
    fn decode_simd(data: &[u8]) -> Result<alloc::vec::Vec<Self>>;

    /// Decode into an existing buffer, returning items decoded.
    fn decode_simd_into(src: &[u8], dst: &mut [Self]) -> Result<usize>;
}

#[cfg(feature = "alloc")]
extern crate alloc;

// Implement SimdEncodable for f32
impl SimdEncodable for f32 {
    #[cfg(feature = "alloc")]
    fn encode_simd(data: &[Self]) -> Result<alloc::vec::Vec<u8>> {
        encode_f32_array(data)
    }

    fn encode_simd_into(data: &[Self], dst: &mut [u8]) -> Result<usize> {
        array::encode_f32_array_into(data, dst)
    }
}

impl SimdDecodable for f32 {
    #[cfg(feature = "alloc")]
    fn decode_simd(data: &[u8]) -> Result<alloc::vec::Vec<Self>> {
        decode_f32_array(data)
    }

    fn decode_simd_into(src: &[u8], dst: &mut [Self]) -> Result<usize> {
        array::decode_f32_array_into(src, dst)
    }
}

// Implement SimdEncodable for f64
impl SimdEncodable for f64 {
    #[cfg(feature = "alloc")]
    fn encode_simd(data: &[Self]) -> Result<alloc::vec::Vec<u8>> {
        encode_f64_array(data)
    }

    fn encode_simd_into(data: &[Self], dst: &mut [u8]) -> Result<usize> {
        array::encode_f64_array_into(data, dst)
    }
}

impl SimdDecodable for f64 {
    #[cfg(feature = "alloc")]
    fn decode_simd(data: &[u8]) -> Result<alloc::vec::Vec<Self>> {
        decode_f64_array(data)
    }

    fn decode_simd_into(src: &[u8], dst: &mut [Self]) -> Result<usize> {
        array::decode_f64_array_into(src, dst)
    }
}

// Implement SimdEncodable for i32
impl SimdEncodable for i32 {
    #[cfg(feature = "alloc")]
    fn encode_simd(data: &[Self]) -> Result<alloc::vec::Vec<u8>> {
        encode_i32_array(data)
    }

    fn encode_simd_into(data: &[Self], dst: &mut [u8]) -> Result<usize> {
        array::encode_i32_array_into(data, dst)
    }
}

impl SimdDecodable for i32 {
    #[cfg(feature = "alloc")]
    fn decode_simd(data: &[u8]) -> Result<alloc::vec::Vec<Self>> {
        decode_i32_array(data)
    }

    fn decode_simd_into(src: &[u8], dst: &mut [Self]) -> Result<usize> {
        array::decode_i32_array_into(src, dst)
    }
}

// Implement SimdEncodable for i64
impl SimdEncodable for i64 {
    #[cfg(feature = "alloc")]
    fn encode_simd(data: &[Self]) -> Result<alloc::vec::Vec<u8>> {
        encode_i64_array(data)
    }

    fn encode_simd_into(data: &[Self], dst: &mut [u8]) -> Result<usize> {
        array::encode_i64_array_into(data, dst)
    }
}

impl SimdDecodable for i64 {
    #[cfg(feature = "alloc")]
    fn decode_simd(data: &[u8]) -> Result<alloc::vec::Vec<Self>> {
        decode_i64_array(data)
    }

    fn decode_simd_into(src: &[u8], dst: &mut [Self]) -> Result<usize> {
        array::decode_i64_array_into(src, dst)
    }
}

// Implement SimdEncodable for u8
impl SimdEncodable for u8 {
    #[cfg(feature = "alloc")]
    fn encode_simd(data: &[Self]) -> Result<alloc::vec::Vec<u8>> {
        encode_u8_array(data)
    }

    fn encode_simd_into(data: &[Self], dst: &mut [u8]) -> Result<usize> {
        array::encode_u8_array_into(data, dst)
    }
}

impl SimdDecodable for u8 {
    #[cfg(feature = "alloc")]
    fn decode_simd(data: &[u8]) -> Result<alloc::vec::Vec<Self>> {
        decode_u8_array(data)
    }

    fn decode_simd_into(src: &[u8], dst: &mut [Self]) -> Result<usize> {
        array::decode_u8_array_into(src, dst)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_simd_detection() {
        let cap = detect_capability();
        // Should always succeed, even if returning Scalar
        assert!(matches!(
            cap,
            SimdCapability::Avx512
                | SimdCapability::Avx2
                | SimdCapability::Sse42
                | SimdCapability::Neon
                | SimdCapability::Scalar
        ));
    }

    #[test]
    fn test_is_simd_available() {
        // This test just ensures the function works
        let _ = is_simd_available();
    }

    #[cfg(feature = "alloc")]
    #[test]
    fn test_f32_roundtrip() {
        let data: alloc::vec::Vec<f32> = alloc::vec![1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0];
        let encoded = encode_simd_array(&data).expect("encode failed");
        let decoded: alloc::vec::Vec<f32> = decode_simd_array(&encoded).expect("decode failed");
        assert_eq!(data, decoded);
    }

    #[cfg(feature = "alloc")]
    #[test]
    fn test_i32_roundtrip() {
        let data: alloc::vec::Vec<i32> = alloc::vec![-100, -1, 0, 1, 100, 1000, -1000, 42];
        let encoded = encode_simd_array(&data).expect("encode failed");
        let decoded: alloc::vec::Vec<i32> = decode_simd_array(&encoded).expect("decode failed");
        assert_eq!(data, decoded);
    }

    #[cfg(feature = "alloc")]
    #[test]
    fn test_large_array() {
        // Test with a larger array to ensure SIMD path is exercised
        let data: alloc::vec::Vec<f64> = (0..1024).map(|i| i as f64 * 0.5).collect();
        let encoded = encode_simd_array(&data).expect("encode failed");
        let decoded: alloc::vec::Vec<f64> = decode_simd_array(&encoded).expect("decode failed");
        assert_eq!(data, decoded);
    }
}
