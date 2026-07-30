//! SIMD-accelerated array codec example.
//!
//! `oxicode::simd` is a small, **explicit opt-in** codec for contiguous
//! arrays of `f32`/`f64`/`i32`/`i64`/`u8`. It is *not* wired into
//! [`oxicode::encode_to_vec`], `#[derive(Encode, Decode)]`, or the `Encode`/
//! `Decode` traits — a `Vec<f64>` field in a derived struct is encoded by the
//! ordinary (non-SIMD) length-prefixed varint path, exactly as it would be
//! without the `simd` feature enabled. To get the vectorized path you must
//! call `oxicode::simd::encode_simd_array` / `decode_simd_array` (or the
//! per-type `encode_f64_array` etc.) directly, and the result uses the
//! module's own framing (an 8-byte little-endian element count followed by
//! the little-endian element bytes) — it is a different byte layout from
//! `encode_to_vec`'s output for the same `Vec<f64>` and the two are not
//! interchangeable.
//!
//! What the "SIMD" actually is: on little-endian targets each array's
//! payload is a byte image of the element slice, so encoding/decoding reduce
//! to a bulk memory copy. That copy is performed with real hardware kernels
//! (AVX2 or the SSE2 baseline on x86_64, NEON on aarch64), selected at
//! runtime via `detect_capability()`. On big-endian targets each element is
//! byte-swapped individually on a scalar fallback path. The serialized bytes
//! are identical on every architecture; only throughput differs. Because
//! the operation is a bulk copy, it is memory-bandwidth bound rather than
//! compute bound — do not expect large speedups over a well-optimized
//! scalar `to_le_bytes` loop, which the compiler can already vectorize on
//! its own in a release build.
//!
//! The `oxicode::simd` module only exists when the `simd` feature is
//! enabled, so (like `examples/async_streaming.rs`) this file gates its
//! real body behind `#[cfg(feature = "simd")]` and falls back to a short
//! notice otherwise.
//!
//! Run with: cargo run --release --example simd_arrays --features simd

#[cfg(feature = "simd")]
mod simd_demo {
    use oxicode::simd::{
        decode_simd_array, detect_capability, encode_simd_array, is_simd_available,
    };
    use oxicode::Error;
    use std::hint::black_box;
    use std::time::{Duration, Instant};

    /// Encode `data` element-by-element via `f64::to_le_bytes`, matching the
    /// framing `encode_simd_array` produces (8-byte LE count + LE element
    /// bytes), but without going through the vectorized bulk-copy kernel.
    /// Used only as a scalar baseline for the timing comparison below.
    fn encode_f64_scalar(data: &[f64]) -> Vec<u8> {
        let mut out = Vec::with_capacity(8 + data.len() * 8);
        out.extend_from_slice(&(data.len() as u64).to_le_bytes());
        for value in data {
            out.extend_from_slice(&value.to_le_bytes());
        }
        out
    }

    /// Runs `f` `iterations` times (plus one untimed warm-up call) and
    /// returns the average per-call duration and the last result.
    fn time_it<T>(
        iterations: u32,
        mut f: impl FnMut() -> Result<T, Error>,
    ) -> Result<(Duration, T), Error> {
        let warm = f()?;
        let start = Instant::now();
        let mut last = warm;
        for _ in 0..iterations {
            last = black_box(f()?);
        }
        Ok((start.elapsed() / iterations, last))
    }

    pub fn run() -> Result<(), Error> {
        println!("OxiCode SIMD Array Codec Example\n");
        println!("detected CPU capability: {:?}", detect_capability());
        println!("is_simd_available(): {}\n", is_simd_available());

        // --- 1. Round-trip via the opt-in codec ---
        println!("1. Round-trip via oxicode::simd (its own framing, not encode_to_vec):");
        let data: Vec<f64> = (0..10_000).map(|i| i as f64 * 0.5).collect();

        let encoded = encode_simd_array(&data)?;
        let decoded: Vec<f64> = decode_simd_array(&encoded)?;
        assert_eq!(data, decoded);
        println!(
            "   {} f64 values -> {} bytes (8-byte count header + 8 bytes/element), round-trip verified\n",
            data.len(),
            encoded.len()
        );

        // --- 2. Honest throughput comparison ---
        println!("2. Measured throughput: simd bulk-copy path vs a scalar to_le_bytes loop");
        println!("   (memory-bandwidth bound; run with --release for a meaningful number)\n");

        for size in [10_000usize, 100_000, 1_000_000] {
            let data: Vec<f64> = (0..size).map(|i| i as f64).collect();
            let iterations = if size >= 1_000_000 { 20 } else { 200 };

            let (simd_time, simd_bytes) = time_it(iterations, || encode_simd_array(&data))?;
            let (scalar_time, scalar_bytes) = time_it(iterations, || Ok(encode_f64_scalar(&data)))?;
            assert_eq!(simd_bytes, scalar_bytes, "framing must match byte-for-byte");

            let simd_gbps = (size * 8) as f64 / simd_time.as_secs_f64() / 1e9;
            let scalar_gbps = (size * 8) as f64 / scalar_time.as_secs_f64() / 1e9;

            println!(
                "   {:>9} elements: simd {:>7.2?} ({:>5.2} GB/s)  scalar {:>7.2?} ({:>5.2} GB/s)  ratio {:.2}x",
                size,
                simd_time,
                simd_gbps,
                scalar_time,
                scalar_gbps,
                scalar_time.as_secs_f64() / simd_time.as_secs_f64().max(f64::EPSILON),
            );
        }

        println!(
            "\n   Ratios vary by CPU, allocator, and build profile — treat the numbers above as a\n   \
             local measurement, not a portability guarantee. This crate does not ship a fixed\n   \
             \"2-4x\" (or any other) speedup claim; measure on your own target hardware."
        );

        Ok(())
    }
}

#[cfg(feature = "simd")]
fn main() -> Result<(), oxicode::Error> {
    simd_demo::run()
}

#[cfg(not(feature = "simd"))]
fn main() {
    println!("This example requires the \"simd\" feature.");
    println!("Run with: cargo run --release --example simd_arrays --features simd");
}
