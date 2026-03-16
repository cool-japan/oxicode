//! CPU SIMD capability detection.
//!
//! This module provides runtime detection of available SIMD instruction sets.

use core::sync::atomic::{AtomicU8, Ordering};

/// Represents the SIMD capability level of the current CPU.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(u8)]
pub enum SimdCapability {
    /// No SIMD available, use scalar fallback
    Scalar = 0,
    /// SSE4.2 (128-bit, x86/x86_64)
    Sse42 = 1,
    /// AVX2 (256-bit, x86_64)
    Avx2 = 2,
    /// AVX-512 (512-bit, x86_64)
    Avx512 = 3,
    /// NEON (128-bit, ARM)
    Neon = 4,
}

impl SimdCapability {
    /// Returns the vector width in bytes for this capability.
    #[inline]
    pub const fn vector_width(self) -> usize {
        match self {
            SimdCapability::Scalar => 1,
            SimdCapability::Sse42 => 16,
            SimdCapability::Avx2 => 32,
            SimdCapability::Avx512 => 64,
            SimdCapability::Neon => 16,
        }
    }

    /// Returns true if this capability supports any SIMD instructions.
    #[inline]
    pub const fn is_simd(self) -> bool {
        !matches!(self, SimdCapability::Scalar)
    }

    /// Returns the name of this capability.
    #[inline]
    pub const fn name(self) -> &'static str {
        match self {
            SimdCapability::Scalar => "Scalar",
            SimdCapability::Sse42 => "SSE4.2",
            SimdCapability::Avx2 => "AVX2",
            SimdCapability::Avx512 => "AVX-512",
            SimdCapability::Neon => "NEON",
        }
    }

    /// Returns the number of f32 elements that can be processed in parallel.
    #[inline]
    pub const fn f32_lanes(self) -> usize {
        self.vector_width() / 4
    }

    /// Returns the number of f64 elements that can be processed in parallel.
    #[inline]
    pub const fn f64_lanes(self) -> usize {
        self.vector_width() / 8
    }

    /// Returns the number of i32 elements that can be processed in parallel.
    #[inline]
    pub const fn i32_lanes(self) -> usize {
        self.vector_width() / 4
    }
}

impl Default for SimdCapability {
    fn default() -> Self {
        detect_capability()
    }
}

// Cached capability detection result
// 0xFF = not yet detected, other values = SimdCapability as u8
static CACHED_CAPABILITY: AtomicU8 = AtomicU8::new(0xFF);

/// Detect the SIMD capability of the current CPU.
///
/// This function caches the result after the first call for efficiency.
///
/// # Example
///
/// ```rust
/// use oxicode::simd::detect_capability;
///
/// let cap = detect_capability();
/// println!("CPU supports: {} ({}-bit vectors)", cap.name(), cap.vector_width() * 8);
/// ```
#[inline]
pub fn detect_capability() -> SimdCapability {
    let cached = CACHED_CAPABILITY.load(Ordering::Relaxed);
    if cached != 0xFF {
        // SAFETY: We only store valid SimdCapability values
        return match cached {
            0 => SimdCapability::Scalar,
            1 => SimdCapability::Sse42,
            2 => SimdCapability::Avx2,
            3 => SimdCapability::Avx512,
            4 => SimdCapability::Neon,
            _ => SimdCapability::Scalar,
        };
    }

    let detected = detect_capability_impl();
    CACHED_CAPABILITY.store(detected as u8, Ordering::Relaxed);
    detected
}

/// Returns true if any SIMD capability is available.
///
/// This is a convenience function that checks if the detected capability
/// is anything other than `Scalar`.
#[inline]
pub fn is_simd_available() -> bool {
    detect_capability().is_simd()
}

/// Returns the optimal alignment for SIMD operations.
///
/// This returns the vector width of the detected SIMD capability,
/// which is the ideal alignment for memory operations.
#[inline]
pub fn optimal_alignment() -> usize {
    detect_capability().vector_width()
}

// Platform-specific detection implementation
#[cfg(all(target_arch = "x86_64", target_feature = "sse2"))]
fn detect_capability_impl() -> SimdCapability {
    // On x86_64, SSE2 is always available
    // Check for higher capabilities

    #[cfg(target_feature = "avx512f")]
    {
        // If compiled with AVX-512 support, it's available
        return SimdCapability::Avx512;
    }

    #[cfg(not(target_feature = "avx512f"))]
    {
        // Runtime detection using std::arch
        if is_x86_feature_detected!("avx512f") {
            return SimdCapability::Avx512;
        }

        if is_x86_feature_detected!("avx2") {
            return SimdCapability::Avx2;
        }

        if is_x86_feature_detected!("sse4.2") {
            return SimdCapability::Sse42;
        }

        // SSE2 is always available on x86_64, but we require SSE4.2 minimum
        SimdCapability::Scalar
    }
}

#[cfg(all(target_arch = "x86", target_feature = "sse2"))]
fn detect_capability_impl() -> SimdCapability {
    // On 32-bit x86, check for SSE4.2 and AVX2
    if is_x86_feature_detected!("avx2") {
        return SimdCapability::Avx2;
    }

    if is_x86_feature_detected!("sse4.2") {
        return SimdCapability::Sse42;
    }

    SimdCapability::Scalar
}

#[cfg(all(target_arch = "aarch64", target_feature = "neon"))]
fn detect_capability_impl() -> SimdCapability {
    // NEON is mandatory on AArch64
    SimdCapability::Neon
}

#[cfg(all(target_arch = "arm", target_feature = "neon"))]
fn detect_capability_impl() -> SimdCapability {
    // Check if NEON is actually available at runtime
    #[cfg(target_os = "linux")]
    {
        // On Linux, we can check /proc/cpuinfo or use getauxval
        // For simplicity, if compiled with neon feature, assume it's available
        SimdCapability::Neon
    }

    #[cfg(not(target_os = "linux"))]
    {
        // On other platforms, assume NEON if compiled with feature
        SimdCapability::Neon
    }
}

// Fallback for platforms without SIMD or with unknown SIMD support
#[cfg(not(any(
    all(target_arch = "x86_64", target_feature = "sse2"),
    all(target_arch = "x86", target_feature = "sse2"),
    all(target_arch = "aarch64", target_feature = "neon"),
    all(target_arch = "arm", target_feature = "neon"),
)))]
fn detect_capability_impl() -> SimdCapability {
    SimdCapability::Scalar
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_detect_capability() {
        let cap = detect_capability();
        println!("Detected SIMD capability: {:?}", cap);
        println!("Vector width: {} bytes", cap.vector_width());
        println!("f32 lanes: {}", cap.f32_lanes());
        println!("f64 lanes: {}", cap.f64_lanes());
    }

    #[test]
    fn test_cached_detection() {
        // Call twice to test caching
        let cap1 = detect_capability();
        let cap2 = detect_capability();
        assert_eq!(cap1, cap2);
    }

    #[test]
    fn test_simd_capability_ordering() {
        // Verify capability ordering makes sense
        assert!(SimdCapability::Scalar < SimdCapability::Sse42);
        assert!(SimdCapability::Sse42 < SimdCapability::Avx2);
        assert!(SimdCapability::Avx2 < SimdCapability::Avx512);
    }

    #[test]
    fn test_vector_widths() {
        assert_eq!(SimdCapability::Scalar.vector_width(), 1);
        assert_eq!(SimdCapability::Sse42.vector_width(), 16);
        assert_eq!(SimdCapability::Avx2.vector_width(), 32);
        assert_eq!(SimdCapability::Avx512.vector_width(), 64);
        assert_eq!(SimdCapability::Neon.vector_width(), 16);
    }

    #[test]
    fn test_is_simd() {
        assert!(!SimdCapability::Scalar.is_simd());
        assert!(SimdCapability::Sse42.is_simd());
        assert!(SimdCapability::Avx2.is_simd());
        assert!(SimdCapability::Avx512.is_simd());
        assert!(SimdCapability::Neon.is_simd());
    }
}
