//! Encoding benchmarks comparing oxicode to bincode

use criterion::{criterion_group, criterion_main, Criterion, Throughput};
use std::hint::black_box;

// Benchmark data structures
mod oxicode_types {
    use oxicode::{Decode, Encode};

    #[derive(Encode, Decode)]
    pub struct SmallStruct {
        pub id: u32,
        pub value: i32,
    }

    #[derive(Encode, Decode)]
    pub struct MediumStruct {
        pub id: u64,
        pub name: String,
        pub values: Vec<i32>,
        pub active: bool,
    }

    #[allow(dead_code)]
    #[derive(Encode, Decode)]
    pub struct LargeStruct {
        pub id: u64,
        pub title: String,
        pub description: String,
        pub tags: Vec<String>,
        pub data: Vec<u8>,
        pub metadata: Vec<(String, String)>,
    }
}

mod bincode_types {
    use bincode::{Decode, Encode};

    #[derive(Encode, Decode)]
    pub struct SmallStruct {
        pub id: u32,
        pub value: i32,
    }

    #[derive(Encode, Decode)]
    pub struct MediumStruct {
        pub id: u64,
        pub name: String,
        pub values: Vec<i32>,
        pub active: bool,
    }

    #[allow(dead_code)]
    #[derive(Encode, Decode)]
    pub struct LargeStruct {
        pub id: u64,
        pub title: String,
        pub description: String,
        pub tags: Vec<String>,
        pub data: Vec<u8>,
        pub metadata: Vec<(String, String)>,
    }
}

fn bench_primitives(c: &mut Criterion) {
    let mut group = c.benchmark_group("primitives");

    // u32 encoding
    group.bench_function("oxicode_u32", |b| {
        let value = 42u32;
        b.iter(|| {
            let bytes = oxicode::encode_to_vec(black_box(&value)).unwrap();
            black_box(bytes);
        });
    });

    group.bench_function("bincode_u32", |b| {
        let value = 42u32;
        let config = bincode::config::standard();
        b.iter(|| {
            let bytes = bincode::encode_to_vec(black_box(&value), black_box(config)).unwrap();
            black_box(bytes);
        });
    });

    // i64 encoding (varint)
    group.bench_function("oxicode_i64", |b| {
        let value = -12345i64;
        b.iter(|| {
            let bytes = oxicode::encode_to_vec(black_box(&value)).unwrap();
            black_box(bytes);
        });
    });

    group.bench_function("bincode_i64", |b| {
        let value = -12345i64;
        let config = bincode::config::standard();
        b.iter(|| {
            let bytes = bincode::encode_to_vec(black_box(&value), black_box(config)).unwrap();
            black_box(bytes);
        });
    });

    group.finish();
}

fn bench_collections(c: &mut Criterion) {
    let mut group = c.benchmark_group("collections");

    // Vec<u32>
    let vec_data = vec![1u32, 2, 3, 4, 5, 10, 20, 30, 40, 50];
    group.throughput(Throughput::Bytes((vec_data.len() * 4) as u64));

    group.bench_function("oxicode_vec", |b| {
        b.iter(|| {
            let bytes = oxicode::encode_to_vec(black_box(&vec_data)).unwrap();
            black_box(bytes);
        });
    });

    group.bench_function("bincode_vec", |b| {
        let config = bincode::config::standard();
        b.iter(|| {
            let bytes = bincode::encode_to_vec(black_box(&vec_data), black_box(config)).unwrap();
            black_box(bytes);
        });
    });

    // String
    let string_data = "Hello, OxiCode! This is a test string with some content. 🦀".to_string();
    group.throughput(Throughput::Bytes(string_data.len() as u64));

    group.bench_function("oxicode_string", |b| {
        b.iter(|| {
            let bytes = oxicode::encode_to_vec(black_box(&string_data)).unwrap();
            black_box(bytes);
        });
    });

    group.bench_function("bincode_string", |b| {
        let config = bincode::config::standard();
        b.iter(|| {
            let bytes = bincode::encode_to_vec(black_box(&string_data), black_box(config)).unwrap();
            black_box(bytes);
        });
    });

    group.finish();
}

fn bench_structs(c: &mut Criterion) {
    let mut group = c.benchmark_group("structs");

    // Small struct
    let oxi_small = oxicode_types::SmallStruct {
        id: 123,
        value: -456,
    };
    let bin_small = bincode_types::SmallStruct {
        id: 123,
        value: -456,
    };

    group.bench_function("oxicode_small_struct", |b| {
        b.iter(|| {
            let bytes = oxicode::encode_to_vec(black_box(&oxi_small)).unwrap();
            black_box(bytes);
        });
    });

    group.bench_function("bincode_small_struct", |b| {
        let config = bincode::config::standard();
        b.iter(|| {
            let bytes = bincode::encode_to_vec(black_box(&bin_small), black_box(config)).unwrap();
            black_box(bytes);
        });
    });

    // Medium struct
    let oxi_medium = oxicode_types::MediumStruct {
        id: 999,
        name: "benchmark_data".to_string(),
        values: vec![1, 2, 3, 4, 5],
        active: true,
    };
    let bin_medium = bincode_types::MediumStruct {
        id: 999,
        name: "benchmark_data".to_string(),
        values: vec![1, 2, 3, 4, 5],
        active: true,
    };

    group.bench_function("oxicode_medium_struct", |b| {
        b.iter(|| {
            let bytes = oxicode::encode_to_vec(black_box(&oxi_medium)).unwrap();
            black_box(bytes);
        });
    });

    group.bench_function("bincode_medium_struct", |b| {
        let config = bincode::config::standard();
        b.iter(|| {
            let bytes = bincode::encode_to_vec(black_box(&bin_medium), black_box(config)).unwrap();
            black_box(bytes);
        });
    });

    group.finish();
}

fn bench_duration(c: &mut Criterion) {
    let dur = std::time::Duration::new(3600, 123_456_789);
    let enc = oxicode::encode_to_vec(&dur).expect("encode");

    c.bench_function("encode_duration", |b| {
        b.iter(|| {
            let bytes = oxicode::encode_to_vec(black_box(&dur)).expect("encode");
            black_box(bytes);
        });
    });

    c.bench_function("decode_duration", |b| {
        b.iter(|| {
            let (d, _): (std::time::Duration, _) =
                oxicode::decode_from_slice(black_box(&enc)).expect("decode");
            black_box(d);
        });
    });
}

fn bench_ipaddr(c: &mut Criterion) {
    use std::net::{IpAddr, Ipv4Addr, Ipv6Addr};

    let ipv4: IpAddr = IpAddr::V4(Ipv4Addr::new(192, 168, 1, 100));
    let ipv6: IpAddr = IpAddr::V6(Ipv6Addr::new(
        0x2001, 0x0db8, 0x85a3, 0, 0, 0x8a2e, 0x0370, 0x7334,
    ));

    let enc_ipv4 = oxicode::encode_to_vec(&ipv4).expect("encode");
    let enc_ipv6 = oxicode::encode_to_vec(&ipv6).expect("encode");

    let mut group = c.benchmark_group("ipaddr");

    group.bench_function("encode_ipv4", |b| {
        b.iter(|| {
            let bytes = oxicode::encode_to_vec(black_box(&ipv4)).expect("encode");
            black_box(bytes);
        });
    });

    group.bench_function("decode_ipv4", |b| {
        b.iter(|| {
            let (d, _): (IpAddr, _) =
                oxicode::decode_from_slice(black_box(&enc_ipv4)).expect("decode");
            black_box(d);
        });
    });

    group.bench_function("encode_ipv6", |b| {
        b.iter(|| {
            let bytes = oxicode::encode_to_vec(black_box(&ipv6)).expect("encode");
            black_box(bytes);
        });
    });

    group.bench_function("decode_ipv6", |b| {
        b.iter(|| {
            let (d, _): (IpAddr, _) =
                oxicode::decode_from_slice(black_box(&enc_ipv6)).expect("decode");
            black_box(d);
        });
    });

    group.finish();
}

fn bench_range(c: &mut Criterion) {
    use std::ops::Range;

    let range: Range<u32> = 1_000..999_999;
    let enc = oxicode::encode_to_vec(&range).expect("encode");

    c.bench_function("encode_range_u32", |b| {
        b.iter(|| {
            let bytes = oxicode::encode_to_vec(black_box(&range)).expect("encode");
            black_box(bytes);
        });
    });

    c.bench_function("decode_range_u32", |b| {
        b.iter(|| {
            let (r, _): (Range<u32>, _) =
                oxicode::decode_from_slice(black_box(&enc)).expect("decode");
            black_box(r);
        });
    });
}

fn bench_option_vec(c: &mut Criterion) {
    let some_payload: Option<Vec<u8>> = Some(vec![0xDEu8; 64]);
    let none_payload: Option<Vec<u8>> = None;

    let enc_some = oxicode::encode_to_vec(&some_payload).expect("encode some");
    let enc_none = oxicode::encode_to_vec(&none_payload).expect("encode none");

    let mut group = c.benchmark_group("option_vec_u8");
    group.throughput(Throughput::Bytes(64));

    group.bench_function("encode_some", |b| {
        b.iter(|| {
            let bytes = oxicode::encode_to_vec(black_box(&some_payload)).expect("encode");
            black_box(bytes);
        });
    });

    group.bench_function("decode_some", |b| {
        b.iter(|| {
            let (v, _): (Option<Vec<u8>>, _) =
                oxicode::decode_from_slice(black_box(&enc_some)).expect("decode");
            black_box(v);
        });
    });

    group.bench_function("encode_none", |b| {
        b.iter(|| {
            let bytes = oxicode::encode_to_vec(black_box(&none_payload)).expect("encode");
            black_box(bytes);
        });
    });

    group.bench_function("decode_none", |b| {
        b.iter(|| {
            let (v, _): (Option<Vec<u8>>, _) =
                oxicode::decode_from_slice(black_box(&enc_none)).expect("decode");
            black_box(v);
        });
    });

    group.finish();
}

criterion_group!(
    benches,
    bench_primitives,
    bench_collections,
    bench_structs,
    bench_duration,
    bench_ipaddr,
    bench_range,
    bench_option_vec
);
criterion_main!(benches);
