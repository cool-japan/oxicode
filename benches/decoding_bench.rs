//! Decoding benchmarks comparing oxicode to bincode

use criterion::{criterion_group, criterion_main, Criterion, Throughput};
use std::collections::HashMap;
use std::hint::black_box;

fn bench_primitive_decode(c: &mut Criterion) {
    let mut group = c.benchmark_group("primitive_decode");

    // u32 decoding
    let value = 42u32;
    let oxi_bytes = oxicode::encode_to_vec(&value).unwrap();
    let bin_bytes = bincode::encode_to_vec(value, bincode::config::standard()).unwrap();

    group.bench_function("oxicode_u32_decode", |b| {
        b.iter(|| {
            let (decoded, _): (u32, _) = oxicode::decode_from_slice(black_box(&oxi_bytes)).unwrap();
            black_box(decoded);
        });
    });

    group.bench_function("bincode_u32_decode", |b| {
        let config = bincode::config::standard();
        b.iter(|| {
            let (decoded, _): (u32, _) =
                bincode::decode_from_slice(black_box(&bin_bytes), black_box(config)).unwrap();
            black_box(decoded);
        });
    });

    // i64 decoding (zigzag)
    let value = -12345i64;
    let oxi_bytes = oxicode::encode_to_vec(&value).unwrap();
    let bin_bytes = bincode::encode_to_vec(value, bincode::config::standard()).unwrap();

    group.bench_function("oxicode_i64_decode", |b| {
        b.iter(|| {
            let (decoded, _): (i64, _) = oxicode::decode_from_slice(black_box(&oxi_bytes)).unwrap();
            black_box(decoded);
        });
    });

    group.bench_function("bincode_i64_decode", |b| {
        let config = bincode::config::standard();
        b.iter(|| {
            let (decoded, _): (i64, _) =
                bincode::decode_from_slice(black_box(&bin_bytes), black_box(config)).unwrap();
            black_box(decoded);
        });
    });

    group.finish();
}

fn bench_string_decode(c: &mut Criterion) {
    let mut group = c.benchmark_group("string_decode");

    let value = "Hello, OxiCode! This is a test string with some content. 🦀".to_string();
    group.throughput(Throughput::Bytes(value.len() as u64));

    let oxi_bytes = oxicode::encode_to_vec(&value).unwrap();
    let bin_bytes = bincode::encode_to_vec(&value, bincode::config::standard()).unwrap();

    // Regular decode (with allocation)
    group.bench_function("oxicode_string_decode", |b| {
        b.iter(|| {
            let (decoded, _): (String, _) =
                oxicode::decode_from_slice(black_box(&oxi_bytes)).unwrap();
            black_box(decoded);
        });
    });

    group.bench_function("bincode_string_decode", |b| {
        let config = bincode::config::standard();
        b.iter(|| {
            let (decoded, _): (String, _) =
                bincode::decode_from_slice(black_box(&bin_bytes), black_box(config)).unwrap();
            black_box(decoded);
        });
    });

    // Zero-copy decode (no allocation)
    group.bench_function("oxicode_str_borrow_decode", |b| {
        b.iter(|| {
            let (decoded, _): (&str, _) =
                oxicode::borrow_decode_from_slice(black_box(&oxi_bytes)).unwrap();
            black_box(decoded);
        });
    });

    group.bench_function("bincode_str_borrow_decode", |b| {
        let config = bincode::config::standard();
        b.iter(|| {
            let (decoded, _): (&str, _) =
                bincode::borrow_decode_from_slice(black_box(&bin_bytes), black_box(config))
                    .unwrap();
            black_box(decoded);
        });
    });

    group.finish();
}

fn bench_vec_decode(c: &mut Criterion) {
    let mut group = c.benchmark_group("vec_decode");

    let value = vec![1u32; 1000];
    group.throughput(Throughput::Bytes((value.len() * 4) as u64));

    let oxi_bytes = oxicode::encode_to_vec(&value).unwrap();
    let bin_bytes = bincode::encode_to_vec(&value, bincode::config::standard()).unwrap();

    group.bench_function("oxicode_vec_decode", |b| {
        b.iter(|| {
            let (decoded, _): (Vec<u32>, _) =
                oxicode::decode_from_slice(black_box(&oxi_bytes)).unwrap();
            black_box(decoded);
        });
    });

    group.bench_function("bincode_vec_decode", |b| {
        let config = bincode::config::standard();
        b.iter(|| {
            let (decoded, _): (Vec<u32>, _) =
                bincode::decode_from_slice(black_box(&bin_bytes), black_box(config)).unwrap();
            black_box(decoded);
        });
    });

    group.finish();
}

fn bench_decode_large_string(c: &mut Criterion) {
    let mut group = c.benchmark_group("decode_large_string");

    let value: String = "A".repeat(10 * 1024); // 10KB string
    let oxi_bytes = oxicode::encode_to_vec(&value).unwrap();
    let bin_bytes = bincode::encode_to_vec(&value, bincode::config::standard()).unwrap();

    group.throughput(Throughput::Bytes(value.len() as u64));

    group.bench_function("oxicode_large_string_decode", |b| {
        b.iter(|| {
            let (decoded, _): (String, _) =
                oxicode::decode_from_slice(black_box(&oxi_bytes)).unwrap();
            black_box(decoded);
        });
    });

    group.bench_function("bincode_large_string_decode", |b| {
        let config = bincode::config::standard();
        b.iter(|| {
            let (decoded, _): (String, _) =
                bincode::decode_from_slice(black_box(&bin_bytes), black_box(config)).unwrap();
            black_box(decoded);
        });
    });

    group.finish();
}

mod decode_bench_types {
    use oxicode::{Decode, Encode};

    #[derive(Clone, Encode, Decode)]
    pub struct MultiFieldOxi {
        pub id: u64,
        pub name: String,
        pub score: f64,
        pub active: bool,
        pub tags: Vec<String>,
    }
}

mod decode_bench_bincode_types {
    use bincode::{Decode, Encode};

    #[derive(Clone, Encode, Decode)]
    pub struct MultiFieldBin {
        pub id: u64,
        pub name: String,
        pub score: f64,
        pub active: bool,
        pub tags: Vec<String>,
    }
}

fn bench_decode_struct(c: &mut Criterion) {
    use decode_bench_bincode_types::MultiFieldBin;
    use decode_bench_types::MultiFieldOxi;

    let mut group = c.benchmark_group("decode_struct");

    let oxi_val = MultiFieldOxi {
        id: 42,
        name: "benchmark_item".to_string(),
        score: 99.5,
        active: true,
        tags: vec!["alpha".to_string(), "beta".to_string(), "gamma".to_string()],
    };
    let bin_val = MultiFieldBin {
        id: 42,
        name: "benchmark_item".to_string(),
        score: 99.5,
        active: true,
        tags: vec!["alpha".to_string(), "beta".to_string(), "gamma".to_string()],
    };

    let oxi_bytes = oxicode::encode_to_vec(&oxi_val).unwrap();
    let bin_bytes = bincode::encode_to_vec(&bin_val, bincode::config::standard()).unwrap();

    group.bench_function("oxicode_struct_decode", |b| {
        b.iter(|| {
            let (decoded, _): (MultiFieldOxi, _) =
                oxicode::decode_from_slice(black_box(&oxi_bytes)).unwrap();
            black_box(decoded);
        });
    });

    group.bench_function("bincode_struct_decode", |b| {
        let config = bincode::config::standard();
        b.iter(|| {
            let (decoded, _): (MultiFieldBin, _) =
                bincode::decode_from_slice(black_box(&bin_bytes), black_box(config)).unwrap();
            black_box(decoded);
        });
    });

    group.finish();
}

fn bench_decode_vec_u64(c: &mut Criterion) {
    let mut group = c.benchmark_group("decode_vec_u64");

    let value: Vec<u64> = (0..1000).map(|i| i as u64 * 12345).collect();
    group.throughput(Throughput::Bytes((value.len() * 8) as u64));

    let oxi_bytes = oxicode::encode_to_vec(&value).unwrap();
    let bin_bytes = bincode::encode_to_vec(&value, bincode::config::standard()).unwrap();

    group.bench_function("oxicode_vec_u64_decode", |b| {
        b.iter(|| {
            let (decoded, _): (Vec<u64>, _) =
                oxicode::decode_from_slice(black_box(&oxi_bytes)).unwrap();
            black_box(decoded);
        });
    });

    group.bench_function("bincode_vec_u64_decode", |b| {
        let config = bincode::config::standard();
        b.iter(|| {
            let (decoded, _): (Vec<u64>, _) =
                bincode::decode_from_slice(black_box(&bin_bytes), black_box(config)).unwrap();
            black_box(decoded);
        });
    });

    group.finish();
}

fn bench_decode_hashmap(c: &mut Criterion) {
    let mut group = c.benchmark_group("decode_hashmap");

    let value: HashMap<String, u64> = (0..100u64)
        .map(|i| (format!("key_{:03}", i), i * 7))
        .collect();

    let oxi_bytes = oxicode::encode_to_vec(&value).unwrap();
    let bin_bytes = bincode::encode_to_vec(&value, bincode::config::standard()).unwrap();

    group.bench_function("oxicode_hashmap_decode", |b| {
        b.iter(|| {
            let (decoded, _): (HashMap<String, u64>, _) =
                oxicode::decode_from_slice(black_box(&oxi_bytes)).unwrap();
            black_box(decoded);
        });
    });

    group.bench_function("bincode_hashmap_decode", |b| {
        let config = bincode::config::standard();
        b.iter(|| {
            let (decoded, _): (HashMap<String, u64>, _) =
                bincode::decode_from_slice(black_box(&bin_bytes), black_box(config)).unwrap();
            black_box(decoded);
        });
    });

    group.finish();
}

criterion_group!(
    benches,
    bench_primitive_decode,
    bench_string_decode,
    bench_vec_decode,
    bench_decode_large_string,
    bench_decode_struct,
    bench_decode_vec_u64,
    bench_decode_hashmap
);
criterion_main!(benches);
