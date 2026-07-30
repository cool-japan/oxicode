//! Vectorized byte-copy kernels for the SIMD array codec.
//!
//! On little-endian targets the array wire format (see [`super::array`]) is a
//! plain byte image of the element slice, so encoding/decoding an array of a
//! fixed-width primitive reduces to a bulk memory copy. [`copy_bytes`] performs
//! that copy using real hardware SIMD instructions, selected at runtime:
//!
//! - **x86_64**: AVX2 (`_mm256_loadu_si256` / `_mm256_storeu_si256`) when the
//!   CPU reports AVX2 support, otherwise the SSE2 baseline
//!   (`_mm_loadu_si128` / `_mm_storeu_si128`), which is guaranteed on x86_64.
//! - **aarch64**: NEON (`vld1q_u8` / `vst1q_u8`), which is mandatory on the
//!   aarch64 base ABI.
//! - **everything else, and under Miri**: a portable `copy_from_slice`
//!   fallback.
//!
//! The copy is byte-for-byte identical to a scalar loop, so the serialized
//! output is unchanged on every architecture. Only the throughput differs.

#[cfg(all(target_arch = "x86_64", not(miri)))]
use super::detect::{detect_capability, SimdCapability};

/// Copy the leading `min(src.len(), dst.len())` bytes from `src` into `dst`
/// using the widest vector path available on this target (x86_64 build).
///
/// The two buffers must not overlap; callers pass distinct slices, so this is
/// always satisfied here.
#[cfg(all(target_arch = "x86_64", not(miri)))]
#[inline]
pub(crate) fn copy_bytes(src: &[u8], dst: &mut [u8]) {
    let len = core::cmp::min(src.len(), dst.len());
    if len == 0 {
        return;
    }
    let src_ptr = src.as_ptr();
    let dst_ptr = dst.as_mut_ptr();
    // SAFETY: `src_ptr`/`dst_ptr` are valid for `len` bytes (len is the min of
    // both slice lengths), the buffers are distinct (independent borrows), and
    // the selected kernel's target feature is satisfied: AVX2 is confirmed by
    // runtime detection, and SSE2 is part of the mandatory x86_64 baseline ABI.
    // Each kernel only reads `src` and only writes `dst`.
    unsafe {
        if detect_capability() >= SimdCapability::Avx2 {
            copy_bytes_avx2(src_ptr, dst_ptr, len);
        } else {
            copy_bytes_sse2(src_ptr, dst_ptr, len);
        }
    }
}

/// Copy the leading `min(src.len(), dst.len())` bytes from `src` into `dst`
/// using NEON (aarch64 build).
#[cfg(all(target_arch = "aarch64", not(miri)))]
#[inline]
pub(crate) fn copy_bytes(src: &[u8], dst: &mut [u8]) {
    let len = core::cmp::min(src.len(), dst.len());
    if len == 0 {
        return;
    }
    let src_ptr = src.as_ptr();
    let dst_ptr = dst.as_mut_ptr();
    // SAFETY: `src_ptr`/`dst_ptr` are valid for `len` bytes, the buffers are
    // distinct, and NEON is mandatory on the aarch64 base ABI. The kernel only
    // reads `src` and only writes `dst`.
    unsafe {
        copy_bytes_neon(src_ptr, dst_ptr, len);
    }
}

/// Copy the leading `min(src.len(), dst.len())` bytes from `src` into `dst`
/// using the portable fallback (all other targets, and under Miri).
#[cfg(not(all(any(target_arch = "x86_64", target_arch = "aarch64"), not(miri))))]
#[inline]
pub(crate) fn copy_bytes(src: &[u8], dst: &mut [u8]) {
    let len = core::cmp::min(src.len(), dst.len());
    dst[..len].copy_from_slice(&src[..len]);
}

/// AVX2 (256-bit) byte copy with a scalar tail.
///
/// # Safety
///
/// - `src` must be valid for reads of `len` bytes and `dst` valid for writes of
///   `len` bytes.
/// - `src` and `dst` must not overlap.
/// - The current CPU must support AVX2 (guaranteed by the caller's runtime
///   detection, or by the `avx2` target feature under `no_std`).
#[cfg(all(target_arch = "x86_64", not(miri)))]
#[target_feature(enable = "avx2")]
unsafe fn copy_bytes_avx2(src: *const u8, dst: *mut u8, len: usize) {
    use core::arch::x86_64::{__m256i, _mm256_loadu_si256, _mm256_storeu_si256};

    let mut offset = 0usize;
    while offset + 32 <= len {
        let chunk = _mm256_loadu_si256(src.add(offset) as *const __m256i);
        _mm256_storeu_si256(dst.add(offset) as *mut __m256i, chunk);
        offset += 32;
    }
    while offset < len {
        *dst.add(offset) = *src.add(offset);
        offset += 1;
    }
}

/// SSE2 (128-bit) byte copy with a scalar tail.
///
/// # Safety
///
/// Same contract as [`copy_bytes_avx2`]; SSE2 is part of the x86_64 baseline
/// ABI, so the feature requirement is always met on this target.
#[cfg(all(target_arch = "x86_64", not(miri)))]
#[target_feature(enable = "sse2")]
unsafe fn copy_bytes_sse2(src: *const u8, dst: *mut u8, len: usize) {
    use core::arch::x86_64::{__m128i, _mm_loadu_si128, _mm_storeu_si128};

    let mut offset = 0usize;
    while offset + 16 <= len {
        let chunk = _mm_loadu_si128(src.add(offset) as *const __m128i);
        _mm_storeu_si128(dst.add(offset) as *mut __m128i, chunk);
        offset += 16;
    }
    while offset < len {
        *dst.add(offset) = *src.add(offset);
        offset += 1;
    }
}

/// NEON (128-bit) byte copy with a scalar tail.
///
/// # Safety
///
/// - `src` must be valid for reads of `len` bytes and `dst` valid for writes of
///   `len` bytes.
/// - `src` and `dst` must not overlap.
/// - NEON must be available (mandatory on the aarch64 base ABI).
#[cfg(all(target_arch = "aarch64", not(miri)))]
#[target_feature(enable = "neon")]
unsafe fn copy_bytes_neon(src: *const u8, dst: *mut u8, len: usize) {
    use core::arch::aarch64::{vld1q_u8, vst1q_u8};

    let mut offset = 0usize;
    while offset + 16 <= len {
        let chunk = vld1q_u8(src.add(offset));
        vst1q_u8(dst.add(offset), chunk);
        offset += 16;
    }
    while offset < len {
        *dst.add(offset) = *src.add(offset);
        offset += 1;
    }
}

#[cfg(test)]
mod tests {
    extern crate alloc;

    use super::*;

    #[test]
    fn copy_bytes_matches_reference_all_lengths() {
        // Exercise lengths spanning several vector widths plus odd tails.
        for len in 0..=200usize {
            let src: alloc::vec::Vec<u8> = (0..len).map(|i| (i * 31 + 7) as u8).collect();
            let mut dst = alloc::vec![0u8; len];
            copy_bytes(&src, &mut dst);
            assert_eq!(src, dst, "mismatch at len {len}");
        }
    }

    #[test]
    fn copy_bytes_respects_shorter_length() {
        let src = [1u8, 2, 3, 4, 5];
        let mut dst = [0u8; 3];
        copy_bytes(&src, &mut dst);
        assert_eq!(dst, [1, 2, 3]);
    }
}
