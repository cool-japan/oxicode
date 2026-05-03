//! API surface benchmarks — covers public APIs not exercised by the other bench files.
//!
//! Groups covered:
//! - `encode_to_fixed_array` (stack allocation, no heap)
//! - `encode_seq_to_vec` (exact-size iterator, single allocation)
//! - `decode_iter_from_slice` (lazy iterator decoding)
//! - `file_io_round_trip` (encode_to_file + decode_from_file; disk-speed dependent)
//! - `streaming_write_read` (StreamingEncoder + StreamingDecoder)
//! - `encoded_size` (SizeWriter cost vs encode_to_vec().len())
//! - `checked_round_trip` (encode_to_vec_checked + decode_from_slice_checked; checksum feature)
//! - `container_breadth` (BTreeMap, HashSet, Result, nested Vec, enum variants)
//!
//! Run with: `cargo bench --bench api_surface_bench --all-features`

use criterion::{criterion_group, criterion_main, BenchmarkId, Criterion, Throughput};
use oxicode::{Decode, Encode};
use std::collections::{BTreeMap, HashSet};
use std::hint::black_box;

// ---------------------------------------------------------------------------
// Shared types
// ---------------------------------------------------------------------------

#[derive(Clone, Encode, Decode, PartialEq, Debug)]
struct Item {
    id: u32,
    value: i64,
    label: String,
}

#[derive(Clone, Encode, Decode, PartialEq, Debug)]
enum Tag {
    Alpha,
    Beta(u32),
    Gamma { x: f64, y: f64 },
}

#[derive(Clone, Encode, Decode, PartialEq, Debug)]
struct BreadthPayload {
    items: Vec<Item>,
    tags: Vec<Tag>,
    nested: Vec<Vec<u32>>,
}

fn make_item(i: u32) -> Item {
    Item {
        id: i,
        value: -(i as i64) * 17,
        label: format!("item-{:04}", i),
    }
}

fn make_breadth(n: usize) -> BreadthPayload {
    BreadthPayload {
        items: (0..n as u32).map(make_item).collect(),
        tags: (0..n)
            .map(|i| match i % 3 {
                0 => Tag::Alpha,
                1 => Tag::Beta(i as u32),
                _ => Tag::Gamma {
                    x: i as f64,
                    y: -(i as f64),
                },
            })
            .collect(),
        nested: (0..n).map(|i| (0..i as u32).collect::<Vec<_>>()).collect(),
    }
}

// ---------------------------------------------------------------------------
// encode_to_fixed_array
// ---------------------------------------------------------------------------

fn bench_encode_to_fixed_array(c: &mut Criterion) {
    let mut group = c.benchmark_group("api/encode_to_fixed_array");

    // u32: needs at most 5 bytes with varint
    group.throughput(Throughput::Bytes(4));
    group.bench_function("u32", |b| {
        let value = 0x0ABCDEF0_u32;
        b.iter(|| {
            let (arr, n): ([u8; 16], _) =
                oxicode::encode_to_fixed_array(black_box(&value)).expect("encode u32 fixed");
            black_box((arr, n));
        });
    });

    // Small struct: ~10 bytes
    let item = make_item(42);
    group.throughput(Throughput::Bytes(32));
    group.bench_function("item_struct", |b| {
        b.iter(|| {
            let (arr, n): ([u8; 64], _) =
                oxicode::encode_to_fixed_array(black_box(&item)).expect("encode item fixed");
            black_box((arr, n));
        });
    });

    // [u8; 64] payload: exactly 64 data bytes + length prefix
    let data = [0xABu8; 64];
    group.throughput(Throughput::Bytes(64));
    group.bench_function("bytes_64", |b| {
        b.iter(|| {
            let slice: &[u8] = &data;
            let (arr, n): ([u8; 80], _) =
                oxicode::encode_to_fixed_array(black_box(&slice)).expect("encode bytes fixed");
            black_box((arr, n));
        });
    });

    group.finish();
}

// ---------------------------------------------------------------------------
// encode_seq_to_vec (ExactSizeIterator path — no intermediate Vec)
// ---------------------------------------------------------------------------

fn bench_encode_seq_to_vec(c: &mut Criterion) {
    let mut group = c.benchmark_group("api/encode_seq_to_vec");

    for n in [100usize, 1_000, 10_000] {
        let items: Vec<u32> = (0..n as u32).collect();
        group.throughput(Throughput::Elements(n as u64));

        group.bench_with_input(BenchmarkId::new("seq_u32", n), &n, |b, _| {
            b.iter(|| {
                let bytes = oxicode::encode_seq_to_vec(black_box(items.iter().copied()))
                    .expect("encode_seq_to_vec u32");
                black_box(bytes);
            });
        });

        // Compare to encode_to_vec (collects internally)
        group.bench_with_input(BenchmarkId::new("vec_u32_baseline", n), &n, |b, _| {
            b.iter(|| {
                let bytes =
                    oxicode::encode_to_vec(black_box(&items)).expect("encode_to_vec u32 baseline");
                black_box(bytes);
            });
        });
    }

    group.finish();
}

// ---------------------------------------------------------------------------
// decode_iter_from_slice (lazy iterator decoding)
// ---------------------------------------------------------------------------

fn bench_decode_iter_from_slice(c: &mut Criterion) {
    let mut group = c.benchmark_group("api/decode_iter_from_slice");

    for n in [100usize, 1_000, 10_000] {
        let items: Vec<u32> = (0..n as u32).map(|i| i * 7).collect();
        let encoded = oxicode::encode_to_vec(&items).expect("encode setup");
        group.throughput(Throughput::Elements(n as u64));

        // Lazy iterator: sum without collecting
        group.bench_with_input(BenchmarkId::new("iter_sum_u32", n), &n, |b, _| {
            b.iter(|| {
                let sum: u32 = oxicode::decode_iter_from_slice::<u32>(black_box(&encoded))
                    .expect("iter init")
                    .filter_map(|r| r.ok())
                    .sum();
                black_box(sum);
            });
        });

        // Compare to decode_from_slice (eager Vec)
        group.bench_with_input(BenchmarkId::new("slice_collect_u32", n), &n, |b, _| {
            b.iter(|| {
                let (decoded, _): (Vec<u32>, _) =
                    oxicode::decode_from_slice(black_box(&encoded)).expect("decode_from_slice");
                black_box(decoded);
            });
        });
    }

    group.finish();
}

// ---------------------------------------------------------------------------
// File I/O round-trip  (disk-speed dependent — use for relative comparisons)
// ---------------------------------------------------------------------------

fn bench_file_io_round_trip(c: &mut Criterion) {
    let mut group = c.benchmark_group("api/file_io_round_trip");

    let tmp = std::env::temp_dir();

    for n in [64usize, 512, 4096] {
        let payload: Vec<u8> = (0..n as u8).cycle().take(n).collect();
        let path = tmp.join(format!("oxicode_bench_{}.bin", n));
        group.throughput(Throughput::Bytes(n as u64));

        group.bench_with_input(BenchmarkId::new("bytes", n), &n, |b, _| {
            b.iter(|| {
                oxicode::encode_to_file(black_box(&payload), &path).expect("encode_to_file");
                let decoded: Vec<u8> = oxicode::decode_from_file(&path).expect("decode_from_file");
                black_box(decoded);
            });
        });
    }

    group.finish();
}

// ---------------------------------------------------------------------------
// Streaming write + read (StreamingEncoder / StreamingDecoder)
// ---------------------------------------------------------------------------

fn bench_streaming_write_read(c: &mut Criterion) {
    let mut group = c.benchmark_group("api/streaming_write_read");

    for n in [64usize, 256, 1024] {
        let items: Vec<Item> = (0..n as u32).map(make_item).collect();
        let byte_estimate = n as u64 * 24; // rough per-item overhead
        group.throughput(Throughput::Bytes(byte_estimate));

        group.bench_with_input(BenchmarkId::new("encode_items", n), &n, |b, _| {
            b.iter(|| {
                let buf: Vec<u8> = Vec::new();
                let mut encoder = oxicode::streaming::StreamingEncoder::new(buf);
                for item in black_box(&items) {
                    encoder.write_item(item).expect("streaming write");
                }
                let buf = encoder.finish().expect("streaming finish");
                black_box(buf);
            });
        });

        // Pre-encode for decode bench
        let mut pre_buf: Vec<u8> = Vec::new();
        {
            let mut enc = oxicode::streaming::StreamingEncoder::new(&mut pre_buf);
            for item in &items {
                enc.write_item(item).expect("setup streaming write");
            }
            enc.finish().expect("setup streaming finish");
        }

        group.bench_with_input(BenchmarkId::new("decode_items", n), &n, |b, _| {
            b.iter(|| {
                let cursor = std::io::Cursor::new(black_box(&pre_buf));
                let mut decoder = oxicode::streaming::StreamingDecoder::new(cursor);
                let mut count = 0usize;
                while let Some(item) = decoder.read_item::<Item>().expect("streaming read") {
                    black_box(item);
                    count += 1;
                }
                black_box(count);
            });
        });
    }

    group.finish();
}

// ---------------------------------------------------------------------------
// encoded_size vs encode_to_vec().len()
// ---------------------------------------------------------------------------

fn bench_encoded_size(c: &mut Criterion) {
    let mut group = c.benchmark_group("api/encoded_size");

    let item = make_item(99);
    let large: Vec<u32> = (0..1000_u32).collect();

    // Single struct
    group.throughput(Throughput::Bytes(32));

    group.bench_function("item_struct_encoded_size", |b| {
        b.iter(|| {
            let sz = oxicode::encoded_size(black_box(&item)).expect("encoded_size item");
            black_box(sz);
        });
    });

    group.bench_function("item_struct_vec_len_baseline", |b| {
        b.iter(|| {
            let sz = oxicode::encode_to_vec(black_box(&item))
                .expect("encode_to_vec item")
                .len();
            black_box(sz);
        });
    });

    // Large Vec<u32>
    group.throughput(Throughput::Bytes(large.len() as u64 * 4));

    group.bench_function("vec_u32_encoded_size", |b| {
        b.iter(|| {
            let sz = oxicode::encoded_size(black_box(&large)).expect("encoded_size vec");
            black_box(sz);
        });
    });

    group.bench_function("vec_u32_vec_len_baseline", |b| {
        b.iter(|| {
            let sz = oxicode::encode_to_vec(black_box(&large))
                .expect("encode_to_vec vec")
                .len();
            black_box(sz);
        });
    });

    group.finish();
}

// ---------------------------------------------------------------------------
// Checksum round-trip  (only registered when feature = "checksum")
// ---------------------------------------------------------------------------

#[cfg(feature = "checksum")]
fn bench_checked_round_trip(c: &mut Criterion) {
    let mut group = c.benchmark_group("api/checked_round_trip");

    for n in [64usize, 512, 4096] {
        let payload: Vec<u8> = (0..n as u8).cycle().take(n).collect();
        group.throughput(Throughput::Bytes(n as u64));

        group.bench_with_input(BenchmarkId::new("encode_checked", n), &n, |b, _| {
            b.iter(|| {
                let bytes =
                    oxicode::encode_to_vec_checked(black_box(&payload)).expect("encode_checked");
                black_box(bytes);
            });
        });

        let checked_bytes = oxicode::encode_to_vec_checked(&payload).expect("setup encode_checked");

        group.bench_with_input(BenchmarkId::new("decode_checked", n), &n, |b, _| {
            b.iter(|| {
                let (decoded, _): (Vec<u8>, _) =
                    oxicode::decode_from_slice_checked(black_box(&checked_bytes))
                        .expect("decode_checked");
                black_box(decoded);
            });
        });
    }

    group.finish();
}

// ---------------------------------------------------------------------------
// Container breadth: BTreeMap, HashSet, Result, nested Vec, enum
// ---------------------------------------------------------------------------

fn bench_btreemap(c: &mut Criterion) {
    let mut group = c.benchmark_group("api/container/btreemap");

    for n in [10usize, 100, 500] {
        let map: BTreeMap<String, u64> = (0..n as u64)
            .map(|i| (format!("key-{:04}", i), i * 11))
            .collect();
        group.throughput(Throughput::Bytes(map.len() as u64 * 16));

        let encoded = oxicode::encode_to_vec(&map).expect("setup btreemap encode");

        group.bench_with_input(BenchmarkId::new("encode", n), &n, |b, _| {
            b.iter(|| {
                let bytes = oxicode::encode_to_vec(black_box(&map)).expect("btreemap encode");
                black_box(bytes);
            });
        });

        group.bench_with_input(BenchmarkId::new("decode", n), &n, |b, _| {
            b.iter(|| {
                let (m, _): (BTreeMap<String, u64>, _) =
                    oxicode::decode_from_slice(black_box(&encoded)).expect("btreemap decode");
                black_box(m);
            });
        });
    }

    group.finish();
}

fn bench_hashset(c: &mut Criterion) {
    let mut group = c.benchmark_group("api/container/hashset");

    for n in [10usize, 100, 500] {
        let set: HashSet<u64> = (0..n as u64).map(|i| i * 13).collect();
        group.throughput(Throughput::Bytes(set.len() as u64 * 8));

        let encoded = oxicode::encode_to_vec(&set).expect("setup hashset encode");

        group.bench_with_input(BenchmarkId::new("encode", n), &n, |b, _| {
            b.iter(|| {
                let bytes = oxicode::encode_to_vec(black_box(&set)).expect("hashset encode");
                black_box(bytes);
            });
        });

        group.bench_with_input(BenchmarkId::new("decode", n), &n, |b, _| {
            b.iter(|| {
                let (s, _): (HashSet<u64>, _) =
                    oxicode::decode_from_slice(black_box(&encoded)).expect("hashset decode");
                black_box(s);
            });
        });
    }

    group.finish();
}

fn bench_result_type(c: &mut Criterion) {
    let mut group = c.benchmark_group("api/container/result");

    let ok_val: Result<u64, String> = Ok(0xDEAD_BEEF_u64);
    let err_val: Result<u64, String> = Err("something went wrong".to_string());

    let ok_bytes = oxicode::encode_to_vec(&ok_val).expect("setup result ok");
    let err_bytes = oxicode::encode_to_vec(&err_val).expect("setup result err");
    group.throughput(Throughput::Bytes(16));

    group.bench_function("encode_ok", |b| {
        b.iter(|| {
            let bytes = oxicode::encode_to_vec(black_box(&ok_val)).expect("encode result ok");
            black_box(bytes);
        });
    });

    group.bench_function("encode_err", |b| {
        b.iter(|| {
            let bytes = oxicode::encode_to_vec(black_box(&err_val)).expect("encode result err");
            black_box(bytes);
        });
    });

    group.bench_function("decode_ok", |b| {
        b.iter(|| {
            let (v, _): (Result<u64, String>, _) =
                oxicode::decode_from_slice(black_box(&ok_bytes)).expect("decode result ok");
            let _ = black_box(v);
        });
    });

    group.bench_function("decode_err", |b| {
        b.iter(|| {
            let (v, _): (Result<u64, String>, _) =
                oxicode::decode_from_slice(black_box(&err_bytes)).expect("decode result err");
            let _ = black_box(v);
        });
    });

    group.finish();
}

fn bench_enum_variants(c: &mut Criterion) {
    let mut group = c.benchmark_group("api/container/enum");

    let alpha = Tag::Alpha;
    let beta = Tag::Beta(0xABCDEF_u32);
    let gamma = Tag::Gamma { x: 1.5, y: -2.5 };

    let alpha_bytes = oxicode::encode_to_vec(&alpha).expect("setup alpha");
    let beta_bytes = oxicode::encode_to_vec(&beta).expect("setup beta");
    let gamma_bytes = oxicode::encode_to_vec(&gamma).expect("setup gamma");

    group.bench_function("encode_unit", |b| {
        b.iter(|| {
            let bytes = oxicode::encode_to_vec(black_box(&alpha)).expect("encode alpha");
            black_box(bytes);
        });
    });

    group.bench_function("encode_tuple", |b| {
        b.iter(|| {
            let bytes = oxicode::encode_to_vec(black_box(&beta)).expect("encode beta");
            black_box(bytes);
        });
    });

    group.bench_function("encode_struct", |b| {
        b.iter(|| {
            let bytes = oxicode::encode_to_vec(black_box(&gamma)).expect("encode gamma");
            black_box(bytes);
        });
    });

    group.bench_function("decode_unit", |b| {
        b.iter(|| {
            let (v, _): (Tag, _) =
                oxicode::decode_from_slice(black_box(&alpha_bytes)).expect("decode alpha");
            black_box(v);
        });
    });

    group.bench_function("decode_tuple", |b| {
        b.iter(|| {
            let (v, _): (Tag, _) =
                oxicode::decode_from_slice(black_box(&beta_bytes)).expect("decode beta");
            black_box(v);
        });
    });

    group.bench_function("decode_struct", |b| {
        b.iter(|| {
            let (v, _): (Tag, _) =
                oxicode::decode_from_slice(black_box(&gamma_bytes)).expect("decode gamma");
            black_box(v);
        });
    });

    group.finish();
}

fn bench_nested_vec(c: &mut Criterion) {
    let mut group = c.benchmark_group("api/container/nested_vec");

    for n in [8usize, 32, 128] {
        let nested: Vec<Vec<u32>> = (0..n).map(|i| (0..i as u32).collect()).collect();
        let byte_estimate = (n * n / 2) as u64 * 4;
        group.throughput(Throughput::Bytes(byte_estimate));

        let encoded = oxicode::encode_to_vec(&nested).expect("setup nested vec");

        group.bench_with_input(BenchmarkId::new("encode", n), &n, |b, _| {
            b.iter(|| {
                let bytes = oxicode::encode_to_vec(black_box(&nested)).expect("nested vec encode");
                black_box(bytes);
            });
        });

        group.bench_with_input(BenchmarkId::new("decode", n), &n, |b, _| {
            b.iter(|| {
                let (v, _): (Vec<Vec<u32>>, _) =
                    oxicode::decode_from_slice(black_box(&encoded)).expect("nested vec decode");
                black_box(v);
            });
        });
    }

    group.finish();
}

fn bench_breadth_payload(c: &mut Criterion) {
    let mut group = c.benchmark_group("api/container/breadth_payload");

    for n in [8usize, 32, 128] {
        let payload = make_breadth(n);
        let encoded = oxicode::encode_to_vec(&payload).expect("setup breadth");
        group.throughput(Throughput::Bytes(encoded.len() as u64));

        group.bench_with_input(BenchmarkId::new("encode", n), &n, |b, _| {
            b.iter(|| {
                let bytes = oxicode::encode_to_vec(black_box(&payload)).expect("breadth encode");
                black_box(bytes);
            });
        });

        group.bench_with_input(BenchmarkId::new("decode", n), &n, |b, _| {
            b.iter(|| {
                let (p, _): (BreadthPayload, _) =
                    oxicode::decode_from_slice(black_box(&encoded)).expect("breadth decode");
                black_box(p);
            });
        });
    }

    group.finish();
}

// ---------------------------------------------------------------------------
// Criterion group registration
// ---------------------------------------------------------------------------

// With checksum feature: register the checked_round_trip bench
#[cfg(feature = "checksum")]
criterion_group!(
    benches,
    bench_encode_to_fixed_array,
    bench_encode_seq_to_vec,
    bench_decode_iter_from_slice,
    bench_file_io_round_trip,
    bench_streaming_write_read,
    bench_encoded_size,
    bench_checked_round_trip,
    bench_btreemap,
    bench_hashset,
    bench_result_type,
    bench_enum_variants,
    bench_nested_vec,
    bench_breadth_payload,
);

// Without checksum feature: same list minus checked_round_trip
#[cfg(not(feature = "checksum"))]
criterion_group!(
    benches,
    bench_encode_to_fixed_array,
    bench_encode_seq_to_vec,
    bench_decode_iter_from_slice,
    bench_file_io_round_trip,
    bench_streaming_write_read,
    bench_encoded_size,
    bench_btreemap,
    bench_hashset,
    bench_result_type,
    bench_enum_variants,
    bench_nested_vec,
    bench_breadth_payload,
);

criterion_main!(benches);
