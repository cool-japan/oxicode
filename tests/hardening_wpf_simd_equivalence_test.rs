//! WP-F: equivalence, robustness and throughput tests for the SIMD array codec.
//!
//! The SIMD path must produce byte-for-byte identical output to a plain
//! `to_le_bytes` reference on every input, and must reject malicious headers
//! (huge / overflowing element counts) with an error instead of panicking or
//! attempting a giant allocation.

#![cfg(feature = "simd")]

use oxicode::simd::{
    decode_f32_array, decode_f64_array, decode_i32_array, decode_i64_array, decode_u8_array,
    encode_f32_array, encode_f64_array, encode_i32_array, encode_i64_array, encode_u8_array,
};

const HEADER: usize = 8;

/// Tiny deterministic xorshift PRNG so tests need no external rng crate.
struct Rng(u64);

impl Rng {
    fn new(seed: u64) -> Self {
        Rng(seed | 1)
    }
    fn next_u64(&mut self) -> u64 {
        let mut x = self.0;
        x ^= x << 13;
        x ^= x >> 7;
        x ^= x << 17;
        self.0 = x;
        x
    }
    fn next_u32(&mut self) -> u32 {
        (self.next_u64() >> 32) as u32
    }
}

// ---- Reference encoders: header (len u64 LE) ++ elements as LE bytes. --------

fn ref_encode_f32(data: &[f32]) -> Vec<u8> {
    let mut out = Vec::with_capacity(HEADER + data.len() * 4);
    out.extend_from_slice(&(data.len() as u64).to_le_bytes());
    for v in data {
        out.extend_from_slice(&v.to_le_bytes());
    }
    out
}

fn ref_encode_f64(data: &[f64]) -> Vec<u8> {
    let mut out = Vec::with_capacity(HEADER + data.len() * 8);
    out.extend_from_slice(&(data.len() as u64).to_le_bytes());
    for v in data {
        out.extend_from_slice(&v.to_le_bytes());
    }
    out
}

fn ref_encode_i32(data: &[i32]) -> Vec<u8> {
    let mut out = Vec::with_capacity(HEADER + data.len() * 4);
    out.extend_from_slice(&(data.len() as u64).to_le_bytes());
    for v in data {
        out.extend_from_slice(&v.to_le_bytes());
    }
    out
}

fn ref_encode_i64(data: &[i64]) -> Vec<u8> {
    let mut out = Vec::with_capacity(HEADER + data.len() * 8);
    out.extend_from_slice(&(data.len() as u64).to_le_bytes());
    for v in data {
        out.extend_from_slice(&v.to_le_bytes());
    }
    out
}

fn ref_encode_u8(data: &[u8]) -> Vec<u8> {
    let mut out = Vec::with_capacity(HEADER + data.len());
    out.extend_from_slice(&(data.len() as u64).to_le_bytes());
    out.extend_from_slice(data);
    out
}

/// Compare f32 bit patterns exactly (so NaN payloads round-trip precisely).
fn bits_eq_f32(a: &[f32], b: &[f32]) -> bool {
    a.len() == b.len() && a.iter().zip(b).all(|(x, y)| x.to_bits() == y.to_bits())
}

fn bits_eq_f64(a: &[f64], b: &[f64]) -> bool {
    a.len() == b.len() && a.iter().zip(b).all(|(x, y)| x.to_bits() == y.to_bits())
}

// ---- Equivalence over lengths 0..=80 (covers vector widths + odd tails). ----

#[test]
fn f32_simd_matches_reference_and_roundtrips() {
    let mut rng = Rng::new(0xC0FF_EE01);
    // Edge-case pool: NaN payloads, infinities, subnormals, signed zeros, extremes.
    let edge = [
        f32::NAN,
        f32::from_bits(0x7fc0_1234),
        f32::from_bits(0xffc0_dead),
        f32::INFINITY,
        f32::NEG_INFINITY,
        f32::MIN_POSITIVE,
        f32::from_bits(1), // smallest subnormal
        -0.0,
        0.0,
        f32::MAX,
        f32::MIN,
    ];

    for len in 0..=80usize {
        let mut data: Vec<f32> = Vec::with_capacity(len);
        for i in 0..len {
            if i % 3 == 0 {
                data.push(edge[i % edge.len()]);
            } else {
                data.push(f32::from_bits(rng.next_u32()));
            }
        }

        let encoded = encode_f32_array(&data).expect("encode");
        assert_eq!(encoded, ref_encode_f32(&data), "byte mismatch at len {len}");

        let decoded = decode_f32_array(&encoded).expect("decode");
        assert!(
            bits_eq_f32(&data, &decoded),
            "roundtrip mismatch at len {len}"
        );
    }
}

#[test]
fn f64_simd_matches_reference_and_roundtrips() {
    let mut rng = Rng::new(0xABCD_1234);
    let edge = [
        f64::NAN,
        f64::from_bits(0x7ff8_0000_dead_beef),
        f64::INFINITY,
        f64::NEG_INFINITY,
        f64::MIN_POSITIVE,
        f64::from_bits(1),
        -0.0,
        0.0,
        f64::MAX,
        f64::MIN,
    ];
    for len in 0..=80usize {
        let mut data: Vec<f64> = Vec::with_capacity(len);
        for i in 0..len {
            if i % 4 == 0 {
                data.push(edge[i % edge.len()]);
            } else {
                data.push(f64::from_bits(rng.next_u64()));
            }
        }
        let encoded = encode_f64_array(&data).expect("encode");
        assert_eq!(encoded, ref_encode_f64(&data), "byte mismatch at len {len}");
        let decoded = decode_f64_array(&encoded).expect("decode");
        assert!(
            bits_eq_f64(&data, &decoded),
            "roundtrip mismatch at len {len}"
        );
    }
}

#[test]
fn i32_simd_matches_reference_and_roundtrips() {
    let mut rng = Rng::new(0x1357_9BDF);
    for len in 0..=80usize {
        let mut data: Vec<i32> = Vec::with_capacity(len);
        for _ in 0..len {
            data.push(rng.next_u32() as i32);
        }
        if len > 0 {
            data[0] = i32::MIN;
            data[len - 1] = i32::MAX;
        }
        let encoded = encode_i32_array(&data).expect("encode");
        assert_eq!(encoded, ref_encode_i32(&data), "byte mismatch at len {len}");
        let decoded = decode_i32_array(&encoded).expect("decode");
        assert_eq!(data, decoded, "roundtrip mismatch at len {len}");
    }
}

#[test]
fn i64_simd_matches_reference_and_roundtrips() {
    let mut rng = Rng::new(0x2468_ACE0);
    for len in 0..=80usize {
        let mut data: Vec<i64> = Vec::with_capacity(len);
        for _ in 0..len {
            data.push(rng.next_u64() as i64);
        }
        if len > 0 {
            data[0] = i64::MIN;
            data[len - 1] = i64::MAX;
        }
        let encoded = encode_i64_array(&data).expect("encode");
        assert_eq!(encoded, ref_encode_i64(&data), "byte mismatch at len {len}");
        let decoded = decode_i64_array(&encoded).expect("decode");
        assert_eq!(data, decoded, "roundtrip mismatch at len {len}");
    }
}

#[test]
fn u8_simd_matches_reference_and_roundtrips() {
    let mut rng = Rng::new(0xF00D_BEEF);
    for len in 0..=200usize {
        let data: Vec<u8> = (0..len).map(|_| rng.next_u32() as u8).collect();
        let encoded = encode_u8_array(&data).expect("encode");
        assert_eq!(encoded, ref_encode_u8(&data), "byte mismatch at len {len}");
        let decoded = decode_u8_array(&encoded).expect("decode");
        assert_eq!(data, decoded, "roundtrip mismatch at len {len}");
    }
}

// ---- Unaligned source buffers. ----------------------------------------------

#[test]
fn decode_from_unaligned_buffer() {
    let data: Vec<f64> = (0..37).map(|i| i as f64 * 1.5).collect();
    let encoded = encode_f64_array(&data).expect("encode");

    // Prepend padding bytes so the payload starts at several odd byte offsets
    // relative to an 8-byte boundary, forcing the unaligned load path.
    for pad in 1..=8usize {
        let mut buf = vec![0xAAu8; pad];
        buf.extend_from_slice(&encoded);
        let decoded = decode_f64_array(&buf[pad..]).expect("decode unaligned");
        assert!(bits_eq_f64(&data, &decoded), "unaligned mismatch pad {pad}");
    }
}

// ---- Malicious headers must error, not panic / OOM. -------------------------

#[test]
fn decode_rejects_overflowing_counts_without_panic() {
    // count = u64::MAX: count * size_of overflows usize -> checked error.
    let max = (u64::MAX).to_le_bytes();
    assert!(decode_f32_array(&max).is_err());
    assert!(decode_f64_array(&max).is_err());
    assert!(decode_i32_array(&max).is_err());
    assert!(decode_i64_array(&max).is_err());
    assert!(decode_u8_array(&max).is_err());

    // count = 2^62: for f32, count*4 wraps to 0 under a naive multiply.
    let wrap = (1u64 << 62).to_le_bytes();
    assert!(decode_f32_array(&wrap).is_err());

    // count = 2^61: for f64, count*8 wraps to 0 under a naive multiply.
    let wrap8 = (1u64 << 61).to_le_bytes();
    assert!(decode_f64_array(&wrap8).is_err());

    // Plausible-but-truncated: header claims 1000 elements, no payload.
    let mut short = (1000u64).to_le_bytes().to_vec();
    short.truncate(8);
    assert!(decode_f32_array(&short).is_err());
}

#[test]
fn decode_rejects_short_header() {
    assert!(decode_f32_array(&[0u8; 3]).is_err());
    assert!(decode_u8_array(&[]).is_err());
}

// ---- Honest throughput measurement (prints; never fails). -------------------

#[test]
fn throughput_report_f32() {
    use std::time::Instant;

    let n = 4_000_000usize;
    let data: Vec<f32> = (0..n).map(|i| i as f32 * 0.5).collect();
    let bytes = (n * 4) as f64;

    // Warm up detection + allocator.
    let _ = encode_f32_array(&data).expect("encode");

    let iters = 20;

    // SIMD (public) path.
    let start = Instant::now();
    let mut sink = 0usize;
    for _ in 0..iters {
        let encoded = encode_f32_array(&data).expect("encode");
        sink ^= encoded.len();
    }
    let simd_secs = start.elapsed().as_secs_f64();

    // Naive per-element reference (what the fabricated code effectively did).
    let start = Instant::now();
    for _ in 0..iters {
        let mut out = Vec::with_capacity(HEADER + n * 4);
        out.extend_from_slice(&(n as u64).to_le_bytes());
        for v in &data {
            out.extend_from_slice(&v.to_le_bytes());
        }
        sink ^= out.len();
    }
    let scalar_secs = start.elapsed().as_secs_f64();

    let simd_gbps = bytes * iters as f64 / simd_secs / 1e9;
    let scalar_gbps = bytes * iters as f64 / scalar_secs / 1e9;
    println!(
        "f32 Vec encode: SIMD/bulk = {simd_gbps:.2} GB/s, naive per-element = {scalar_gbps:.2} GB/s (speedup {:.2}x) [sink={sink}]",
        simd_gbps / scalar_gbps
    );

    // Allocation-free `_into` path isolates the vector-copy kernel from
    // allocation/zero-init cost. `black_box` on the buffer defeats dead-store
    // elimination so both sides are measured fairly.
    use oxicode::simd::SimdEncodable;
    use std::hint::black_box;
    let mut buf = vec![0u8; HEADER + n * 4];

    let start = Instant::now();
    for _ in 0..iters {
        let w = <f32 as SimdEncodable>::encode_simd_into(black_box(&data), black_box(&mut buf))
            .expect("encode_into");
        sink ^= w;
        black_box(&buf);
    }
    let simd_into_secs = start.elapsed().as_secs_f64();

    let start = Instant::now();
    for _ in 0..iters {
        let buf = black_box(&mut buf);
        buf[..HEADER].copy_from_slice(&(n as u64).to_le_bytes());
        let mut off = HEADER;
        for v in black_box(&data) {
            buf[off..off + 4].copy_from_slice(&v.to_le_bytes());
            off += 4;
        }
        sink ^= off;
        black_box(&buf);
    }
    let scalar_into_secs = start.elapsed().as_secs_f64();

    let simd_into_gbps = bytes * iters as f64 / simd_into_secs / 1e9;
    let scalar_into_gbps = bytes * iters as f64 / scalar_into_secs / 1e9;
    println!(
        "f32 into-buffer: SIMD/bulk = {simd_into_gbps:.2} GB/s, naive per-element = {scalar_into_gbps:.2} GB/s (speedup {:.2}x) [sink={sink}]",
        simd_into_gbps / scalar_into_gbps
    );
}
