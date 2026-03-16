//! SIMD-optimized encoding and decoding for oxicode.
//!
//! This module provides hardware-accelerated encoding and decoding for numerical arrays,
//! leveraging SIMD instructions (AVX2, AVX-512, NEON, SSE4.2) when available.
//!
//! ## Features
//!
//! - **Auto-detection**: Automatically detects CPU capabilities at runtime
//! - **4-8x speedup**: Significant performance improvement for numerical data
//! - **Zero-copy alignment**: Optimized memory access patterns
//! - **Fallback**: Graceful degradation to scalar path when SIMD unavailable
//!
//! ## Example
//!
//! ```rust,ignore
//! use oxicode::simd;
//!
//! let data: Vec<f32> = vec![1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0];
//!
//! // Encode with SIMD optimization
//! let encoded = simd::encode_array(&data)?;
//!
//! // Decode with SIMD optimization
//! let decoded: Vec<f32> = simd::decode_array(&encoded)?;
//! ```

mod aligned;
mod array;
mod detect;

pub use aligned::{AlignedBuffer, AlignedVec, SIMD_ALIGNMENT};
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
