//! CPU SIMD capability detection.
//!
//! This module provides runtime detection of available SIMD instruction sets.

use core::sync::atomic::{AtomicU8, Ordering};

/// Represents the SIMD capability level of the current CPU.
///
/// # `Ord`/`PartialOrd` are only meaningful *within* a target architecture
///
/// The derived ordering is a single linear scale (`Scalar < Sse42 < Avx2 <
/// Avx512 < Neon`) so that it has a well-defined total order at all, but the
/// x86/x86_64 tiers (`Scalar`, `Sse42`, `Avx2`, `Avx512`) and the ARM tier
/// (`Neon`) can never both be *detected* on the same build — `detect_capability`
/// only ever returns `Neon` on `aarch64`/`arm`, and only ever returns one of
/// the other four on `x86`/`x86_64`. Comparing across those two families is
/// therefore never meaningful, even though the derive makes it compile:
/// `SimdCapability::Neon > SimdCapability::Avx512` is `true` numerically, but
/// does **not** mean "NEON is a stronger capability than AVX-512". Do not
/// write comparisons like `detect_capability() >= SimdCapability::Avx2`
/// (the idiom used in `src/simd/copy.rs`) unless the surrounding code is
/// already gated to a single architecture family with `#[cfg(target_arch =
/// ...)]` — otherwise the comparison can silently take the wrong branch if
/// this enum's variants are ever reordered, or (more subtly) it invites a
/// reader to assume a meaningful cross-family ranking that does not exist.
/// Prefer `matches!(cap, SimdCapability::Avx2 | SimdCapability::Avx512)`
/// for any comparison not already confined to one architecture.
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
    /// NEON (128-bit, ARM). Numerically the highest tier (see the `Ord`
    /// caveat on this type's own docs above) but **not** comparable in
    /// strength to the x86/x86_64 tiers below it — it is a different
    /// architecture's capability, never detected alongside them.
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

// Platform-specific detection implementation.
//
// Two dispatch strategies are used depending on the build:
//
//  * With `std`, `is_x86_feature_detected!` (a std-only macro) probes the CPU
//    at runtime, so a binary built for the x86_64 baseline still lights up AVX2
//    on capable hardware.
//  * Without `std` the macro is unavailable, so detection falls back to
//    compile-time `cfg!(target_feature = ...)`, reflecting exactly the
//    instruction sets the compiler was told it may emit.

#[cfg(all(target_arch = "x86_64", feature = "std"))]
fn detect_capability_impl() -> SimdCapability {
    // Runtime detection via std::arch. SSE2 is part of the x86_64 baseline, but
    // this enum's SIMD tiers start at SSE4.2, so a plain-SSE2 CPU reports
    // Scalar (the vectorized copy path still uses the SSE2 baseline directly).
    if is_x86_feature_detected!("avx512f") {
        return SimdCapability::Avx512;
    }
    if is_x86_feature_detected!("avx2") {
        return SimdCapability::Avx2;
    }
    if is_x86_feature_detected!("sse4.2") {
        return SimdCapability::Sse42;
    }
    SimdCapability::Scalar
}

#[cfg(all(target_arch = "x86_64", not(feature = "std")))]
fn detect_capability_impl() -> SimdCapability {
    // Compile-time detection (no runtime feature probing in no_std).
    if cfg!(target_feature = "avx512f") {
        SimdCapability::Avx512
    } else if cfg!(target_feature = "avx2") {
        SimdCapability::Avx2
    } else if cfg!(target_feature = "sse4.2") {
        SimdCapability::Sse42
    } else {
        SimdCapability::Scalar
    }
}

#[cfg(all(target_arch = "x86", feature = "std"))]
fn detect_capability_impl() -> SimdCapability {
    if is_x86_feature_detected!("avx2") {
        return SimdCapability::Avx2;
    }
    if is_x86_feature_detected!("sse4.2") {
        return SimdCapability::Sse42;
    }
    SimdCapability::Scalar
}

#[cfg(all(target_arch = "x86", not(feature = "std")))]
fn detect_capability_impl() -> SimdCapability {
    if cfg!(target_feature = "avx2") {
        SimdCapability::Avx2
    } else if cfg!(target_feature = "sse4.2") {
        SimdCapability::Sse42
    } else {
        SimdCapability::Scalar
    }
}

#[cfg(all(target_arch = "aarch64", target_feature = "neon"))]
fn detect_capability_impl() -> SimdCapability {
    // NEON is mandatory on the aarch64 base ABI.
    SimdCapability::Neon
}

#[cfg(all(target_arch = "arm", target_feature = "neon"))]
fn detect_capability_impl() -> SimdCapability {
    // 32-bit ARM: if the compiler was told NEON is available, assume it.
    SimdCapability::Neon
}

// Fallback for platforms without SIMD or with unknown SIMD support.
#[cfg(not(any(
    target_arch = "x86_64",
    target_arch = "x86",
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
        // Verify capability ordering makes sense *within* the x86/x86_64 tiers.
        assert!(SimdCapability::Scalar < SimdCapability::Sse42);
        assert!(SimdCapability::Sse42 < SimdCapability::Avx2);
        assert!(SimdCapability::Avx2 < SimdCapability::Avx512);
    }

    /// Pins the documented cross-architecture-family caveat on `SimdCapability`'s
    /// `Ord` impl: `Neon` sorts numerically above every x86/x86_64 tier,
    /// including `Avx512`, even though the two are never detected on the same
    /// build and are not meaningfully comparable. This is intentional (see the
    /// type-level doc), not a bug — the test exists so that if a future change
    /// reorders the enum to "fix" this, it fails loudly here instead of
    /// silently changing the meaning of any existing `>=`-style comparison.
    #[test]
    fn test_neon_sorts_above_x86_tiers_but_is_not_comparable_in_strength() {
        assert!(SimdCapability::Neon > SimdCapability::Avx512);
        assert!(SimdCapability::Neon > SimdCapability::Scalar);
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
