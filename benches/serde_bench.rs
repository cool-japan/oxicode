//! Serde-path benchmarks: oxicode::serde vs bincode::serde
//!
//! Compares the serde compatibility layer of oxicode against bincode's serde layer
//! on primitives, structs, and collections.
//!
//! Both sides use `serde::Serialize` / `serde::Deserialize` — not the native
//! `Encode`/`Decode` derive macros — so the comparison isolates the serde dispatch
//! overhead from the core encoder performance.
//!
//! Compile/run:
//! ```
//! cargo bench --bench serde_bench --features serde
//! ```
//!
//! This file is gated behind the `serde` feature in Cargo.toml
//! (`required-features = ["serde"]`).

use criterion::{criterion_group, criterion_main, BenchmarkId, Criterion, Throughput};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::hint::black_box;

// ---------------------------------------------------------------------------
// Shared data types — only serde traits, no oxicode/bincode derive
// ---------------------------------------------------------------------------

#[derive(Clone, Serialize, Deserialize, PartialEq, Debug)]
struct SerdeSmall {
    id: u32,
    value: i32,
}

#[derive(Clone, Serialize, Deserialize, PartialEq, Debug)]
struct SerdeMedium {
    id: u64,
    name: String,
    values: Vec<u32>,
    active: bool,
    score: f64,
}

fn make_medium(n: usize) -> SerdeMedium {
    SerdeMedium {
        id: n as u64 * 7,
        name: format!("serde-item-{}", n),
        values: (0..n as u32).collect(),
        active: n % 3 != 0,
        score: n as f64 * 1.23,
    }
}

// ---------------------------------------------------------------------------
// Primitives
// ---------------------------------------------------------------------------

fn bench_serde_primitives(c: &mut Criterion) {
    let mut group = c.benchmark_group("serde/primitives");

    let cfg_oxi = oxicode::config::standard();
    let cfg_bin = bincode::config::standard();

    // u64
    let value = 0x0102_0304_0506_0708_u64;
    group.throughput(Throughput::Bytes(std::mem::size_of_val(&value) as u64));

    group.bench_function("oxi_u64_encode", |b| {
        b.iter(|| {
            let bytes = oxicode::serde::encode_to_vec(black_box(&value), cfg_oxi)
                .expect("oxi serde encode u64");
            black_box(bytes);
        });
    });

    group.bench_function("bin_u64_encode", |b| {
        b.iter(|| {
            let bytes = bincode::serde::encode_to_vec(black_box(value), black_box(cfg_bin))
                .expect("bin serde encode u64");
            black_box(bytes);
        });
    });

    let oxi_bytes = oxicode::serde::encode_to_vec(&value, cfg_oxi).expect("setup");
    let bin_bytes = bincode::serde::encode_to_vec(value, cfg_bin).expect("setup");

    group.bench_function("oxi_u64_decode", |b| {
        b.iter(|| {
            let (v, _): (u64, _) =
                oxicode::serde::decode_from_slice(black_box(&oxi_bytes), cfg_oxi)
                    .expect("oxi serde decode u64");
            black_box(v);
        });
    });

    group.bench_function("bin_u64_decode", |b| {
        b.iter(|| {
            let (v, _): (u64, _) =
                bincode::serde::decode_from_slice(black_box(&bin_bytes), black_box(cfg_bin))
                    .expect("bin serde decode u64");
            black_box(v);
        });
    });

    // String
    let s = "Hello, serde benchmark! A moderately long string value.".to_string();
    group.throughput(Throughput::Bytes(s.len() as u64));

    group.bench_function("oxi_string_encode", |b| {
        b.iter(|| {
            let bytes = oxicode::serde::encode_to_vec(black_box(&s), cfg_oxi)
                .expect("oxi serde encode string");
            black_box(bytes);
        });
    });

    group.bench_function("bin_string_encode", |b| {
        b.iter(|| {
            let bytes = bincode::serde::encode_to_vec(black_box(s.clone()), black_box(cfg_bin))
                .expect("bin serde encode string");
            black_box(bytes);
        });
    });

    let oxi_str_bytes = oxicode::serde::encode_to_vec(&s, cfg_oxi).expect("setup");
    let bin_str_bytes = bincode::serde::encode_to_vec(s.clone(), cfg_bin).expect("setup");

    group.bench_function("oxi_string_decode", |b| {
        b.iter(|| {
            let (v, _): (String, _) =
                oxicode::serde::decode_from_slice(black_box(&oxi_str_bytes), cfg_oxi)
                    .expect("oxi serde decode string");
            black_box(v);
        });
    });

    group.bench_function("bin_string_decode", |b| {
        b.iter(|| {
            let (v, _): (String, _) =
                bincode::serde::decode_from_slice(black_box(&bin_str_bytes), black_box(cfg_bin))
                    .expect("bin serde decode string");
            black_box(v);
        });
    });

    group.finish();
}

// ---------------------------------------------------------------------------
// Structs
// ---------------------------------------------------------------------------

fn bench_serde_small_struct(c: &mut Criterion) {
    let mut group = c.benchmark_group("serde/small_struct");

    let cfg_oxi = oxicode::config::standard();
    let cfg_bin = bincode::config::standard();
    let v = SerdeSmall { id: 42, value: -99 };

    let oxi_bytes = oxicode::serde::encode_to_vec(&v, cfg_oxi).expect("setup");
    let bin_bytes = bincode::serde::encode_to_vec(v.clone(), cfg_bin).expect("setup");
    group.throughput(Throughput::Bytes(oxi_bytes.len() as u64));

    group.bench_function("oxi_encode", |b| {
        b.iter(|| {
            let bytes = oxicode::serde::encode_to_vec(black_box(&v), cfg_oxi)
                .expect("oxi serde small encode");
            black_box(bytes);
        });
    });

    group.bench_function("bin_encode", |b| {
        b.iter(|| {
            let bytes = bincode::serde::encode_to_vec(black_box(v.clone()), black_box(cfg_bin))
                .expect("bin serde small encode");
            black_box(bytes);
        });
    });

    group.bench_function("oxi_decode", |b| {
        b.iter(|| {
            let (s, _): (SerdeSmall, _) =
                oxicode::serde::decode_from_slice(black_box(&oxi_bytes), cfg_oxi)
                    .expect("oxi serde small decode");
            black_box(s);
        });
    });

    group.bench_function("bin_decode", |b| {
        b.iter(|| {
            let (s, _): (SerdeSmall, _) =
                bincode::serde::decode_from_slice(black_box(&bin_bytes), black_box(cfg_bin))
                    .expect("bin serde small decode");
            black_box(s);
        });
    });

    group.finish();
}

fn bench_serde_medium_struct(c: &mut Criterion) {
    let mut group = c.benchmark_group("serde/medium_struct");

    let cfg_oxi = oxicode::config::standard();
    let cfg_bin = bincode::config::standard();

    for n in [8usize, 64, 256] {
        let v = make_medium(n);
        let oxi_bytes = oxicode::serde::encode_to_vec(&v, cfg_oxi).expect("setup");
        let bin_bytes = bincode::serde::encode_to_vec(v.clone(), cfg_bin).expect("setup");
        group.throughput(Throughput::Bytes(oxi_bytes.len() as u64));

        group.bench_with_input(BenchmarkId::new("oxi_encode", n), &n, |b, _| {
            b.iter(|| {
                let bytes = oxicode::serde::encode_to_vec(black_box(&v), cfg_oxi)
                    .expect("oxi serde medium encode");
                black_box(bytes);
            });
        });

        group.bench_with_input(BenchmarkId::new("bin_encode", n), &n, |b, _| {
            b.iter(|| {
                let bytes = bincode::serde::encode_to_vec(black_box(v.clone()), black_box(cfg_bin))
                    .expect("bin serde medium encode");
                black_box(bytes);
            });
        });

        group.bench_with_input(BenchmarkId::new("oxi_decode", n), &n, |b, _| {
            b.iter(|| {
                let (s, _): (SerdeMedium, _) =
                    oxicode::serde::decode_from_slice(black_box(&oxi_bytes), cfg_oxi)
                        .expect("oxi serde medium decode");
                black_box(s);
            });
        });

        group.bench_with_input(BenchmarkId::new("bin_decode", n), &n, |b, _| {
            b.iter(|| {
                let (s, _): (SerdeMedium, _) =
                    bincode::serde::decode_from_slice(black_box(&bin_bytes), black_box(cfg_bin))
                        .expect("bin serde medium decode");
                black_box(s);
            });
        });
    }

    group.finish();
}

// ---------------------------------------------------------------------------
// Collections
// ---------------------------------------------------------------------------

fn bench_serde_vec(c: &mut Criterion) {
    let mut group = c.benchmark_group("serde/vec_u32");

    let cfg_oxi = oxicode::config::standard();
    let cfg_bin = bincode::config::standard();

    for n in [100usize, 1000, 10_000] {
        let v: Vec<u32> = (0..n as u32).collect();
        group.throughput(Throughput::Bytes(v.len() as u64 * 4));

        let oxi_bytes = oxicode::serde::encode_to_vec(&v, cfg_oxi).expect("setup");
        let bin_bytes = bincode::serde::encode_to_vec(v.clone(), cfg_bin).expect("setup");

        group.bench_with_input(BenchmarkId::new("oxi_encode", n), &n, |b, _| {
            b.iter(|| {
                let bytes = oxicode::serde::encode_to_vec(black_box(&v), cfg_oxi)
                    .expect("oxi serde vec encode");
                black_box(bytes);
            });
        });

        group.bench_with_input(BenchmarkId::new("bin_encode", n), &n, |b, _| {
            b.iter(|| {
                let bytes = bincode::serde::encode_to_vec(black_box(v.clone()), black_box(cfg_bin))
                    .expect("bin serde vec encode");
                black_box(bytes);
            });
        });

        group.bench_with_input(BenchmarkId::new("oxi_decode", n), &n, |b, _| {
            b.iter(|| {
                let (decoded, _): (Vec<u32>, _) =
                    oxicode::serde::decode_from_slice(black_box(&oxi_bytes), cfg_oxi)
                        .expect("oxi serde vec decode");
                black_box(decoded);
            });
        });

        group.bench_with_input(BenchmarkId::new("bin_decode", n), &n, |b, _| {
            b.iter(|| {
                let (decoded, _): (Vec<u32>, _) =
                    bincode::serde::decode_from_slice(black_box(&bin_bytes), black_box(cfg_bin))
                        .expect("bin serde vec decode");
                black_box(decoded);
            });
        });
    }

    group.finish();
}

fn bench_serde_hashmap(c: &mut Criterion) {
    let mut group = c.benchmark_group("serde/hashmap_str_u64");

    let cfg_oxi = oxicode::config::standard();
    let cfg_bin = bincode::config::standard();

    for n in [10usize, 100, 500] {
        let map: HashMap<String, u64> = (0..n as u64)
            .map(|i| (format!("key_{:04}", i), i * 13))
            .collect();
        group.throughput(Throughput::Bytes(map.len() as u64 * 16));

        let oxi_bytes = oxicode::serde::encode_to_vec(&map, cfg_oxi).expect("setup");
        let bin_bytes = bincode::serde::encode_to_vec(map.clone(), cfg_bin).expect("setup");

        group.bench_with_input(BenchmarkId::new("oxi_encode", n), &n, |b, _| {
            b.iter(|| {
                let bytes = oxicode::serde::encode_to_vec(black_box(&map), cfg_oxi)
                    .expect("oxi serde map encode");
                black_box(bytes);
            });
        });

        group.bench_with_input(BenchmarkId::new("bin_encode", n), &n, |b, _| {
            b.iter(|| {
                let bytes =
                    bincode::serde::encode_to_vec(black_box(map.clone()), black_box(cfg_bin))
                        .expect("bin serde map encode");
                black_box(bytes);
            });
        });

        group.bench_with_input(BenchmarkId::new("oxi_decode", n), &n, |b, _| {
            b.iter(|| {
                let (m, _): (HashMap<String, u64>, _) =
                    oxicode::serde::decode_from_slice(black_box(&oxi_bytes), cfg_oxi)
                        .expect("oxi serde map decode");
                black_box(m);
            });
        });

        group.bench_with_input(BenchmarkId::new("bin_decode", n), &n, |b, _| {
            b.iter(|| {
                let (m, _): (HashMap<String, u64>, _) =
                    bincode::serde::decode_from_slice(black_box(&bin_bytes), black_box(cfg_bin))
                        .expect("bin serde map decode");
                black_box(m);
            });
        });
    }

    group.finish();
}

criterion_group!(
    benches,
    bench_serde_primitives,
    bench_serde_small_struct,
    bench_serde_medium_struct,
    bench_serde_vec,
    bench_serde_hashmap
);
criterion_main!(benches);
