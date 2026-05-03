//! Config-matrix benchmarks: 5 configurations × {encode, decode, byte-size}
//!
//! Compares five configurations on the same representative payload:
//!
//! | Label       | Encoding          | Endian       |
//! |-------------|-------------------|--------------|
//! | oxi_std     | varint            | little-endian|
//! | oxi_legacy  | fixed-int (1.x compat) | little-endian |
//! | oxi_be      | varint            | big-endian   |
//! | bin_std     | varint (bincode)  | little-endian|
//! | bin_legacy  | fixed-int (bincode) | little-endian |
//!
//! Run with: `cargo bench --bench config_matrix_bench`

use criterion::{criterion_group, criterion_main, BenchmarkId, Criterion, Throughput};
use oxicode::{Decode, Encode};
use std::hint::black_box;

/// A representative struct with a mix of integer and string fields.
#[derive(Clone, Encode, Decode)]
struct Payload {
    id: u64,
    count: u32,
    label: String,
    values: Vec<u32>,
    flag: bool,
}

#[derive(Clone, bincode::Encode, bincode::Decode)]
struct BinPayload {
    id: u64,
    count: u32,
    label: String,
    values: Vec<u32>,
    flag: bool,
}

fn make_payload(n: usize) -> Payload {
    Payload {
        id: 0x0102_0304_0506_0708_u64,
        count: n as u32,
        label: format!("payload-{}", n),
        values: (0..n as u32).collect(),
        flag: n % 2 == 0,
    }
}

fn make_bin_payload(n: usize) -> BinPayload {
    BinPayload {
        id: 0x0102_0304_0506_0708_u64,
        count: n as u32,
        label: format!("payload-{}", n),
        values: (0..n as u32).collect(),
        flag: n % 2 == 0,
    }
}

// ---------------------------------------------------------------------------
// Encode benchmarks
// ---------------------------------------------------------------------------

fn bench_encode(c: &mut Criterion) {
    let mut group = c.benchmark_group("config_matrix/encode");

    for n in [16usize, 128, 1024] {
        let oxi_payload = make_payload(n);
        let bin_payload = make_bin_payload(n);

        // Estimate raw bytes for throughput (values * 4 + fixed overhead)
        let byte_estimate = n as u64 * 4 + 64;
        group.throughput(Throughput::Bytes(byte_estimate));

        // oxi_std: varint, little-endian (default)
        group.bench_with_input(BenchmarkId::new("oxi_std", n), &n, |b, _| {
            b.iter(|| {
                let cfg = oxicode::config::standard();
                let bytes = oxicode::encode_to_vec_with_config(black_box(&oxi_payload), cfg)
                    .expect("oxi_std encode");
                black_box(bytes);
            });
        });

        // oxi_legacy: fixed-int, little-endian
        group.bench_with_input(BenchmarkId::new("oxi_legacy", n), &n, |b, _| {
            b.iter(|| {
                let cfg = oxicode::config::legacy();
                let bytes = oxicode::encode_to_vec_with_config(black_box(&oxi_payload), cfg)
                    .expect("oxi_legacy encode");
                black_box(bytes);
            });
        });

        // oxi_be: varint, big-endian
        group.bench_with_input(BenchmarkId::new("oxi_be", n), &n, |b, _| {
            b.iter(|| {
                let cfg = oxicode::config::standard().with_big_endian();
                let bytes = oxicode::encode_to_vec_with_config(black_box(&oxi_payload), cfg)
                    .expect("oxi_be encode");
                black_box(bytes);
            });
        });

        // bin_std: bincode standard (varint, little-endian)
        group.bench_with_input(BenchmarkId::new("bin_std", n), &n, |b, _| {
            b.iter(|| {
                let cfg = bincode::config::standard();
                let bytes = bincode::encode_to_vec(black_box(&bin_payload), black_box(cfg))
                    .expect("bin_std encode");
                black_box(bytes);
            });
        });

        // bin_legacy: bincode legacy (fixed-int, little-endian)
        group.bench_with_input(BenchmarkId::new("bin_legacy", n), &n, |b, _| {
            b.iter(|| {
                let cfg = bincode::config::legacy();
                let bytes = bincode::encode_to_vec(black_box(&bin_payload), black_box(cfg))
                    .expect("bin_legacy encode");
                black_box(bytes);
            });
        });
    }

    group.finish();
}

// ---------------------------------------------------------------------------
// Decode benchmarks
// ---------------------------------------------------------------------------

fn bench_decode(c: &mut Criterion) {
    let mut group = c.benchmark_group("config_matrix/decode");

    for n in [16usize, 128, 1024] {
        let oxi_payload = make_payload(n);
        let bin_payload = make_bin_payload(n);

        let byte_estimate = n as u64 * 4 + 64;
        group.throughput(Throughput::Bytes(byte_estimate));

        let oxi_std_bytes =
            oxicode::encode_to_vec_with_config(&oxi_payload, oxicode::config::standard())
                .expect("oxi_std encode setup");
        let oxi_legacy_bytes =
            oxicode::encode_to_vec_with_config(&oxi_payload, oxicode::config::legacy())
                .expect("oxi_legacy encode setup");
        let oxi_be_bytes = oxicode::encode_to_vec_with_config(
            &oxi_payload,
            oxicode::config::standard().with_big_endian(),
        )
        .expect("oxi_be encode setup");
        let bin_std_bytes = bincode::encode_to_vec(&bin_payload, bincode::config::standard())
            .expect("bin_std encode setup");
        let bin_legacy_bytes = bincode::encode_to_vec(&bin_payload, bincode::config::legacy())
            .expect("bin_legacy encode setup");

        group.bench_with_input(BenchmarkId::new("oxi_std", n), &n, |b, _| {
            b.iter(|| {
                let cfg = oxicode::config::standard();
                let (p, _): (Payload, _) =
                    oxicode::decode_from_slice_with_config(black_box(&oxi_std_bytes), cfg)
                        .expect("oxi_std decode");
                black_box(p);
            });
        });

        group.bench_with_input(BenchmarkId::new("oxi_legacy", n), &n, |b, _| {
            b.iter(|| {
                let cfg = oxicode::config::legacy();
                let (p, _): (Payload, _) =
                    oxicode::decode_from_slice_with_config(black_box(&oxi_legacy_bytes), cfg)
                        .expect("oxi_legacy decode");
                black_box(p);
            });
        });

        group.bench_with_input(BenchmarkId::new("oxi_be", n), &n, |b, _| {
            b.iter(|| {
                let cfg = oxicode::config::standard().with_big_endian();
                let (p, _): (Payload, _) =
                    oxicode::decode_from_slice_with_config(black_box(&oxi_be_bytes), cfg)
                        .expect("oxi_be decode");
                black_box(p);
            });
        });

        group.bench_with_input(BenchmarkId::new("bin_std", n), &n, |b, _| {
            b.iter(|| {
                let cfg = bincode::config::standard();
                let (p, _): (BinPayload, _) =
                    bincode::decode_from_slice(black_box(&bin_std_bytes), black_box(cfg))
                        .expect("bin_std decode");
                black_box(p);
            });
        });

        group.bench_with_input(BenchmarkId::new("bin_legacy", n), &n, |b, _| {
            b.iter(|| {
                let cfg = bincode::config::legacy();
                let (p, _): (BinPayload, _) =
                    bincode::decode_from_slice(black_box(&bin_legacy_bytes), black_box(cfg))
                        .expect("bin_legacy decode");
                black_box(p);
            });
        });
    }

    group.finish();
}

// ---------------------------------------------------------------------------
// Byte-size benchmarks (one-shot: measures allocation cost + encoded size)
// ---------------------------------------------------------------------------

fn bench_byte_size(c: &mut Criterion) {
    let mut group = c.benchmark_group("config_matrix/byte_size");

    for n in [16usize, 128, 1024] {
        let oxi_payload = make_payload(n);
        let bin_payload = make_bin_payload(n);

        group.bench_with_input(BenchmarkId::new("oxi_std", n), &n, |b, _| {
            b.iter(|| {
                let bytes = oxicode::encode_to_vec_with_config(
                    black_box(&oxi_payload),
                    oxicode::config::standard(),
                )
                .expect("oxi_std byte_size");
                black_box(bytes.len());
                black_box(bytes);
            });
        });

        group.bench_with_input(BenchmarkId::new("oxi_legacy", n), &n, |b, _| {
            b.iter(|| {
                let bytes = oxicode::encode_to_vec_with_config(
                    black_box(&oxi_payload),
                    oxicode::config::legacy(),
                )
                .expect("oxi_legacy byte_size");
                black_box(bytes.len());
                black_box(bytes);
            });
        });

        group.bench_with_input(BenchmarkId::new("oxi_be", n), &n, |b, _| {
            b.iter(|| {
                let bytes = oxicode::encode_to_vec_with_config(
                    black_box(&oxi_payload),
                    oxicode::config::standard().with_big_endian(),
                )
                .expect("oxi_be byte_size");
                black_box(bytes.len());
                black_box(bytes);
            });
        });

        group.bench_with_input(BenchmarkId::new("bin_std", n), &n, |b, _| {
            b.iter(|| {
                let bytes =
                    bincode::encode_to_vec(black_box(&bin_payload), bincode::config::standard())
                        .expect("bin_std byte_size");
                black_box(bytes.len());
                black_box(bytes);
            });
        });

        group.bench_with_input(BenchmarkId::new("bin_legacy", n), &n, |b, _| {
            b.iter(|| {
                let bytes =
                    bincode::encode_to_vec(black_box(&bin_payload), bincode::config::legacy())
                        .expect("bin_legacy byte_size");
                black_box(bytes.len());
                black_box(bytes);
            });
        });
    }

    group.finish();
}

criterion_group!(benches, bench_encode, bench_decode, bench_byte_size);
criterion_main!(benches);
