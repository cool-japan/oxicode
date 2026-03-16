//! Comprehensive comparison benchmarks: oxicode vs bincode vs rkyv vs postcard vs borsh
//!
//! This benchmark compares oxicode against other popular binary serialization libraries:
//! - bincode: Binary encoding, similar design to oxicode
//! - rkyv: Zero-copy deserialization
//! - postcard: Embedded-friendly serialization
//! - borsh: Borsh binary serialization (used in Solana/NEAR)
//!
//! Run with: cargo bench --bench comparison_bench

use criterion::{criterion_group, criterion_main, BenchmarkId, Criterion, Throughput};
use std::hint::black_box;

// Test data structures
#[derive(Clone)]
struct BenchmarkData {
    primitives: Vec<u64>,
    strings: Vec<String>,
    nested: Vec<NestedData>,
}

#[derive(Clone)]
struct NestedData {
    id: u64,
    name: String,
    values: Vec<f64>,
}

impl BenchmarkData {
    fn new(size: usize) -> Self {
        Self {
            primitives: (0..size as u64).collect(),
            strings: (0..size).map(|i| format!("String value {}", i)).collect(),
            nested: (0..size)
                .map(|i| NestedData {
                    id: i as u64,
                    name: format!("Nested {}", i),
                    values: vec![i as f64; 10],
                })
                .collect(),
        }
    }
}

// OxiCode types
mod oxicode_types {
    use oxicode::{Decode, Encode};

    #[derive(Clone, Encode, Decode)]
    pub struct BenchmarkData {
        pub primitives: Vec<u64>,
        pub strings: Vec<String>,
        pub nested: Vec<NestedData>,
    }

    #[derive(Clone, Encode, Decode)]
    pub struct NestedData {
        pub id: u64,
        pub name: String,
        pub values: Vec<f64>,
    }

    impl From<&super::BenchmarkData> for BenchmarkData {
        fn from(data: &super::BenchmarkData) -> Self {
            Self {
                primitives: data.primitives.clone(),
                strings: data.strings.clone(),
                nested: data
                    .nested
                    .iter()
                    .map(|n| NestedData {
                        id: n.id,
                        name: n.name.clone(),
                        values: n.values.clone(),
                    })
                    .collect(),
            }
        }
    }
}

// Bincode types
mod bincode_types {
    use bincode::{Decode, Encode};

    #[derive(Clone, Encode, Decode)]
    pub struct BenchmarkData {
        pub primitives: Vec<u64>,
        pub strings: Vec<String>,
        pub nested: Vec<NestedData>,
    }

    #[derive(Clone, Encode, Decode)]
    pub struct NestedData {
        pub id: u64,
        pub name: String,
        pub values: Vec<f64>,
    }

    impl From<&super::BenchmarkData> for BenchmarkData {
        fn from(data: &super::BenchmarkData) -> Self {
            Self {
                primitives: data.primitives.clone(),
                strings: data.strings.clone(),
                nested: data
                    .nested
                    .iter()
                    .map(|n| NestedData {
                        id: n.id,
                        name: n.name.clone(),
                        values: n.values.clone(),
                    })
                    .collect(),
            }
        }
    }
}

fn bench_encoding_comparison(c: &mut Criterion) {
    let mut group = c.benchmark_group("encoding_comparison");

    for size in [100, 1000, 10000] {
        let data = BenchmarkData::new(size);
        let oxi_data = oxicode_types::BenchmarkData::from(&data);
        let bin_data = bincode_types::BenchmarkData::from(&data);

        // Estimate throughput
        let estimated_bytes = size * (8 + 20 + 80); // rough estimate

        group.throughput(Throughput::Bytes(estimated_bytes as u64));

        // OxiCode
        group.bench_with_input(BenchmarkId::new("oxicode", size), &oxi_data, |b, data| {
            b.iter(|| {
                let bytes = oxicode::encode_to_vec(black_box(data)).unwrap();
                black_box(bytes);
            });
        });

        // Bincode
        group.bench_with_input(BenchmarkId::new("bincode", size), &bin_data, |b, data| {
            let config = bincode::config::standard();
            b.iter(|| {
                let bytes = bincode::encode_to_vec(black_box(data), black_box(config)).unwrap();
                black_box(bytes);
            });
        });
    }

    group.finish();
}

fn bench_decoding_comparison(c: &mut Criterion) {
    let mut group = c.benchmark_group("decoding_comparison");

    for size in [100, 1000, 10000] {
        let data = BenchmarkData::new(size);
        let oxi_data = oxicode_types::BenchmarkData::from(&data);
        let bin_data = bincode_types::BenchmarkData::from(&data);

        // Pre-encode data
        let oxi_bytes = oxicode::encode_to_vec(&oxi_data).unwrap();
        let bin_bytes = bincode::encode_to_vec(&bin_data, bincode::config::standard()).unwrap();

        let estimated_bytes = size * (8 + 20 + 80);
        group.throughput(Throughput::Bytes(estimated_bytes as u64));

        // OxiCode
        group.bench_with_input(BenchmarkId::new("oxicode", size), &oxi_bytes, |b, bytes| {
            b.iter(|| {
                let (decoded, _): (oxicode_types::BenchmarkData, _) =
                    oxicode::decode_from_slice(black_box(bytes)).unwrap();
                black_box(decoded);
            });
        });

        // Bincode
        group.bench_with_input(BenchmarkId::new("bincode", size), &bin_bytes, |b, bytes| {
            let config = bincode::config::standard();
            b.iter(|| {
                let (decoded, _): (bincode_types::BenchmarkData, _) =
                    bincode::decode_from_slice(black_box(bytes), black_box(config)).unwrap();
                black_box(decoded);
            });
        });
    }

    group.finish();
}

fn bench_size_comparison(c: &mut Criterion) {
    let group = c.benchmark_group("size_comparison");

    for size in [100, 1000, 10000] {
        let data = BenchmarkData::new(size);
        let oxi_data = oxicode_types::BenchmarkData::from(&data);
        let bin_data = bincode_types::BenchmarkData::from(&data);

        // Encode with each library
        let oxi_bytes = oxicode::encode_to_vec(&oxi_data).unwrap();
        let bin_bytes = bincode::encode_to_vec(&bin_data, bincode::config::standard()).unwrap();

        // Print size comparison (not a benchmark, just informative)
        println!("\nSize comparison for {} items:", size);
        println!("  OxiCode: {} bytes", oxi_bytes.len());
        println!("  Bincode: {} bytes", bin_bytes.len());
        println!(
            "  Ratio: {:.2}%",
            (oxi_bytes.len() as f64 / bin_bytes.len() as f64) * 100.0
        );
    }

    group.finish();
}

#[cfg(feature = "simd")]
fn bench_simd_arrays(c: &mut Criterion) {
    let mut group = c.benchmark_group("simd_arrays");

    for size in [100, 1000, 10000] {
        let data_i32: Vec<i32> = (0..size).collect();
        let data_u32: Vec<u32> = (0..size as u32).collect();
        let data_i64: Vec<i64> = (0..size as i64).collect();
        let data_u64: Vec<u64> = (0..size as u64).collect();
        let data_f32: Vec<f32> = (0..size).map(|i| i as f32).collect();
        let data_f64: Vec<f64> = (0..size).map(|i| i as f64).collect();

        group.throughput(Throughput::Bytes((size * 8) as u64));

        // i32 arrays
        group.bench_with_input(
            BenchmarkId::new("oxicode_i32_simd", size),
            &data_i32,
            |b, data| {
                b.iter(|| {
                    let bytes = oxicode::encode_to_vec(black_box(data)).unwrap();
                    black_box(bytes);
                });
            },
        );

        // u32 arrays
        group.bench_with_input(
            BenchmarkId::new("oxicode_u32_simd", size),
            &data_u32,
            |b, data| {
                b.iter(|| {
                    let bytes = oxicode::encode_to_vec(black_box(data)).unwrap();
                    black_box(bytes);
                });
            },
        );

        // i64 arrays
        group.bench_with_input(
            BenchmarkId::new("oxicode_i64_simd", size),
            &data_i64,
            |b, data| {
                b.iter(|| {
                    let bytes = oxicode::encode_to_vec(black_box(data)).unwrap();
                    black_box(bytes);
                });
            },
        );

        // u64 arrays
        group.bench_with_input(
            BenchmarkId::new("oxicode_u64_simd", size),
            &data_u64,
            |b, data| {
                b.iter(|| {
                    let bytes = oxicode::encode_to_vec(black_box(data)).unwrap();
                    black_box(bytes);
                });
            },
        );

        // f32 arrays
        group.bench_with_input(
            BenchmarkId::new("oxicode_f32_simd", size),
            &data_f32,
            |b, data| {
                b.iter(|| {
                    let bytes = oxicode::encode_to_vec(black_box(data)).unwrap();
                    black_box(bytes);
                });
            },
        );

        // f64 arrays
        group.bench_with_input(
            BenchmarkId::new("oxicode_f64_simd", size),
            &data_f64,
            |b, data| {
                b.iter(|| {
                    let bytes = oxicode::encode_to_vec(black_box(data)).unwrap();
                    black_box(bytes);
                });
            },
        );
    }

    group.finish();
}

#[cfg(any(feature = "compression-lz4", feature = "compression-zstd"))]
fn bench_compression_comparison(c: &mut Criterion) {
    use oxicode::compression::{compress, decompress, Compression};

    let mut group = c.benchmark_group("compression");

    // Three data profiles
    let highly_compressible: Vec<u8> = (0..10000).map(|i: usize| (i % 4) as u8).collect();
    let moderate: Vec<u8> = (0..10000).map(|i: usize| (i * 7 % 256) as u8).collect();
    let random_like: Vec<u8> = (0..10000)
        .map(|i: usize| (i.wrapping_mul(2654435761) % 256) as u8)
        .collect();

    let profiles = [
        ("highly_compressible", highly_compressible.as_slice()),
        ("moderate", moderate.as_slice()),
        ("random_like", random_like.as_slice()),
    ];

    for (profile_name, data) in &profiles {
        group.throughput(Throughput::Bytes(data.len() as u64));

        #[cfg(feature = "compression-lz4")]
        {
            group.bench_with_input(
                BenchmarkId::new(format!("lz4_compress/{}", profile_name), data.len()),
                data,
                |b, d| {
                    b.iter(|| {
                        let compressed = compress(black_box(d), Compression::Lz4).unwrap();
                        black_box(compressed);
                    });
                },
            );

            let lz4_compressed = compress(data, Compression::Lz4).unwrap();
            group.bench_with_input(
                BenchmarkId::new(format!("lz4_decompress/{}", profile_name), data.len()),
                lz4_compressed.as_slice(),
                |b, d| {
                    b.iter(|| {
                        let decompressed = decompress(black_box(d)).unwrap();
                        black_box(decompressed);
                    });
                },
            );
        }

        #[cfg(feature = "compression-zstd")]
        {
            group.bench_with_input(
                BenchmarkId::new(format!("zstd_compress/{}", profile_name), data.len()),
                data,
                |b, d| {
                    b.iter(|| {
                        let compressed = compress(black_box(d), Compression::Zstd).unwrap();
                        black_box(compressed);
                    });
                },
            );

            let zstd_compressed = compress(data, Compression::Zstd).unwrap();
            group.bench_with_input(
                BenchmarkId::new(format!("zstd_decompress/{}", profile_name), data.len()),
                zstd_compressed.as_slice(),
                |b, d| {
                    b.iter(|| {
                        let decompressed = decompress(black_box(d)).unwrap();
                        black_box(decompressed);
                    });
                },
            );
        }
    }

    group.finish();
}

fn bench_primitives_scaling(c: &mut Criterion) {
    let mut group = c.benchmark_group("primitives_scaling");

    // Encode Vec<f64> at various sizes to show throughput scaling
    for size in [64usize, 256, 1024, 4096, 16384] {
        let data: Vec<f64> = (0..size).map(|i| i as f64 * 0.1).collect();
        let oxi_encoded = oxicode::encode_to_vec(&data).unwrap();

        group.throughput(Throughput::Bytes((size * 8) as u64));

        group.bench_with_input(
            BenchmarkId::new("oxicode_encode_f64", size),
            &data,
            |b, d| {
                b.iter(|| {
                    let bytes = oxicode::encode_to_vec(black_box(d)).unwrap();
                    black_box(bytes);
                });
            },
        );

        group.bench_with_input(
            BenchmarkId::new("oxicode_decode_f64", size),
            oxi_encoded.as_slice(),
            |b, bytes| {
                b.iter(|| {
                    let (decoded, _): (Vec<f64>, _) =
                        oxicode::decode_from_slice(black_box(bytes)).unwrap();
                    black_box(decoded);
                });
            },
        );

        // Compare with bincode
        let bin_encoded = bincode::encode_to_vec(&data, bincode::config::standard()).unwrap();
        group.bench_with_input(
            BenchmarkId::new("bincode_encode_f64", size),
            &data,
            |b, d| {
                let config = bincode::config::standard();
                b.iter(|| {
                    let bytes = bincode::encode_to_vec(black_box(d), config).unwrap();
                    black_box(bytes);
                });
            },
        );

        group.bench_with_input(
            BenchmarkId::new("bincode_decode_f64", size),
            bin_encoded.as_slice(),
            |b, bytes| {
                let config = bincode::config::standard();
                b.iter(|| {
                    let (decoded, _): (Vec<f64>, _) =
                        bincode::decode_from_slice(black_box(bytes), config).unwrap();
                    black_box(decoded);
                });
            },
        );
    }

    group.finish();
}

fn bench_primitive_scaling(c: &mut Criterion) {
    let mut group = c.benchmark_group("primitive_scaling");

    for size in [100usize, 1_000, 10_000, 100_000] {
        let data: Vec<u64> = (0..size).map(|i| i as u64).collect();

        group.throughput(Throughput::Bytes((size * 8) as u64));

        group.bench_with_input(BenchmarkId::new("oxicode", size), &data, |b, data| {
            b.iter(|| {
                let bytes = oxicode::encode_to_vec(black_box(data)).unwrap();
                black_box(bytes);
            });
        });

        group.bench_with_input(BenchmarkId::new("bincode_v2", size), &data, |b, data| {
            let config = bincode::config::standard();
            b.iter(|| {
                let bytes = bincode::encode_to_vec(black_box(data), black_box(config)).unwrap();
                black_box(bytes);
            });
        });
    }

    group.finish();
}

mod complex_bench_oxi {
    use oxicode::{Decode, Encode};

    #[derive(Clone, Encode, Decode)]
    pub struct ComplexOxi {
        pub id: u64,
        pub label: String,
        pub values: Vec<f64>,
        pub count: u32,
        pub enabled: bool,
    }
}

mod complex_bench_bin {
    use bincode::{Decode, Encode};

    #[derive(Clone, Encode, Decode)]
    pub struct ComplexBin {
        pub id: u64,
        pub label: String,
        pub values: Vec<f64>,
        pub count: u32,
        pub enabled: bool,
    }
}

fn bench_complex_struct(c: &mut Criterion) {
    use complex_bench_bin::ComplexBin;
    use complex_bench_oxi::ComplexOxi;

    let mut group = c.benchmark_group("complex_struct");

    let oxi_val = ComplexOxi {
        id: 1001,
        label: "benchmark_complex_struct".to_string(),
        values: (0..50).map(|i| i as f64 * 0.1).collect(),
        count: 42,
        enabled: true,
    };
    let bin_val = ComplexBin {
        id: 1001,
        label: "benchmark_complex_struct".to_string(),
        values: (0..50).map(|i| i as f64 * 0.1).collect(),
        count: 42,
        enabled: true,
    };

    let estimated_bytes: u64 = 8 + 24 + 50 * 8 + 4 + 1; // rough size estimate
    group.throughput(Throughput::Bytes(estimated_bytes));

    group.bench_function("oxicode_complex_struct_encode", |b| {
        b.iter(|| {
            let bytes = oxicode::encode_to_vec(black_box(&oxi_val)).unwrap();
            black_box(bytes);
        });
    });

    group.bench_function("bincode_complex_struct_encode", |b| {
        let config = bincode::config::standard();
        b.iter(|| {
            let bytes = bincode::encode_to_vec(black_box(&bin_val), black_box(config)).unwrap();
            black_box(bytes);
        });
    });

    // Decode benchmarks
    let oxi_bytes = oxicode::encode_to_vec(&oxi_val).unwrap();
    let bin_bytes = bincode::encode_to_vec(&bin_val, bincode::config::standard()).unwrap();

    group.bench_function("oxicode_complex_struct_decode", |b| {
        b.iter(|| {
            let (decoded, _): (ComplexOxi, _) =
                oxicode::decode_from_slice(black_box(&oxi_bytes)).unwrap();
            black_box(decoded);
        });
    });

    group.bench_function("bincode_complex_struct_decode", |b| {
        let config = bincode::config::standard();
        b.iter(|| {
            let (decoded, _): (ComplexBin, _) =
                bincode::decode_from_slice(black_box(&bin_bytes), black_box(config)).unwrap();
            black_box(decoded);
        });
    });

    group.finish();
}

fn bench_option_encoding(c: &mut Criterion) {
    let mut group = c.benchmark_group("option_encoding");

    let some_val: Option<Vec<u8>> = Some((0..=255u8).collect());
    let none_val: Option<Vec<u8>> = None;

    let payload_bytes = some_val.as_ref().map(|v| v.len()).unwrap_or(0) as u64;
    group.throughput(Throughput::Bytes(payload_bytes));

    // Some variant
    group.bench_function("oxicode_option_some_encode", |b| {
        b.iter(|| {
            let bytes = oxicode::encode_to_vec(black_box(&some_val)).unwrap();
            black_box(bytes);
        });
    });

    group.bench_function("bincode_option_some_encode", |b| {
        let config = bincode::config::standard();
        b.iter(|| {
            let bytes = bincode::encode_to_vec(black_box(&some_val), black_box(config)).unwrap();
            black_box(bytes);
        });
    });

    // None variant
    group.bench_function("oxicode_option_none_encode", |b| {
        b.iter(|| {
            let bytes = oxicode::encode_to_vec(black_box(&none_val)).unwrap();
            black_box(bytes);
        });
    });

    group.bench_function("bincode_option_none_encode", |b| {
        let config = bincode::config::standard();
        b.iter(|| {
            let bytes = bincode::encode_to_vec(black_box(&none_val), black_box(config)).unwrap();
            black_box(bytes);
        });
    });

    // Decode Some
    let oxi_some_bytes = oxicode::encode_to_vec(&some_val).unwrap();
    let bin_some_bytes = bincode::encode_to_vec(&some_val, bincode::config::standard()).unwrap();

    group.bench_function("oxicode_option_some_decode", |b| {
        b.iter(|| {
            let (decoded, _): (Option<Vec<u8>>, _) =
                oxicode::decode_from_slice(black_box(&oxi_some_bytes)).unwrap();
            black_box(decoded);
        });
    });

    group.bench_function("bincode_option_some_decode", |b| {
        let config = bincode::config::standard();
        b.iter(|| {
            let (decoded, _): (Option<Vec<u8>>, _) =
                bincode::decode_from_slice(black_box(&bin_some_bytes), black_box(config)).unwrap();
            black_box(decoded);
        });
    });

    group.finish();
}

fn bench_string_encoding(c: &mut Criterion) {
    let mut group = c.benchmark_group("string_encoding");

    let strings: Vec<String> = (0..1000).map(|i| format!("key_{}", i)).collect();

    // Estimate throughput as sum of string bytes
    let total_bytes: usize = strings.iter().map(|s| s.len()).sum();
    group.throughput(Throughput::Bytes(total_bytes as u64));

    group.bench_function("oxicode_vec_string", |b| {
        b.iter(|| {
            let bytes = oxicode::encode_to_vec(black_box(&strings)).unwrap();
            black_box(bytes);
        });
    });

    group.bench_function("bincode_v2_vec_string", |b| {
        let config = bincode::config::standard();
        b.iter(|| {
            let bytes = bincode::encode_to_vec(black_box(&strings), black_box(config)).unwrap();
            black_box(bytes);
        });
    });

    // Also benchmark decoding to round out the comparison
    let oxi_bytes = oxicode::encode_to_vec(&strings).unwrap();
    let bin_bytes = bincode::encode_to_vec(&strings, bincode::config::standard()).unwrap();

    group.bench_function("oxicode_vec_string_decode", |b| {
        b.iter(|| {
            let (decoded, _): (Vec<String>, _) =
                oxicode::decode_from_slice(black_box(&oxi_bytes)).unwrap();
            black_box(decoded);
        });
    });

    group.bench_function("bincode_v2_vec_string_decode", |b| {
        let config = bincode::config::standard();
        b.iter(|| {
            let (decoded, _): (Vec<String>, _) =
                bincode::decode_from_slice(black_box(&bin_bytes), black_box(config)).unwrap();
            black_box(decoded);
        });
    });

    group.finish();
}

fn bench_roundtrip(c: &mut Criterion) {
    let mut group = c.benchmark_group("roundtrip");

    for size in [100, 1000, 10000] {
        let data = BenchmarkData::new(size);
        let oxi_data = oxicode_types::BenchmarkData::from(&data);
        let bin_data = bincode_types::BenchmarkData::from(&data);

        let estimated_bytes = size * (8 + 20 + 80);
        group.throughput(Throughput::Bytes(estimated_bytes as u64));

        // OxiCode roundtrip
        group.bench_with_input(BenchmarkId::new("oxicode", size), &oxi_data, |b, data| {
            b.iter(|| {
                let bytes = oxicode::encode_to_vec(black_box(data)).unwrap();
                let (decoded, _): (oxicode_types::BenchmarkData, _) =
                    oxicode::decode_from_slice(&bytes).unwrap();
                black_box(decoded);
            });
        });

        // Bincode roundtrip
        group.bench_with_input(BenchmarkId::new("bincode", size), &bin_data, |b, data| {
            let config = bincode::config::standard();
            b.iter(|| {
                let bytes = bincode::encode_to_vec(black_box(data), config).unwrap();
                let (decoded, _): (bincode_types::BenchmarkData, _) =
                    bincode::decode_from_slice(&bytes, config).unwrap();
                black_box(decoded);
            });
        });
    }

    group.finish();
}

#[cfg(all(
    feature = "simd",
    any(feature = "compression-lz4", feature = "compression-zstd")
))]
criterion_group!(
    benches,
    bench_encoding_comparison,
    bench_decoding_comparison,
    bench_size_comparison,
    bench_simd_arrays,
    bench_roundtrip,
    bench_compression_comparison,
    bench_primitives_scaling,
    bench_primitive_scaling,
    bench_string_encoding,
    bench_complex_struct,
    bench_option_encoding
);

#[cfg(all(
    feature = "simd",
    not(any(feature = "compression-lz4", feature = "compression-zstd"))
))]
criterion_group!(
    benches,
    bench_encoding_comparison,
    bench_decoding_comparison,
    bench_size_comparison,
    bench_simd_arrays,
    bench_roundtrip,
    bench_primitives_scaling,
    bench_primitive_scaling,
    bench_string_encoding,
    bench_complex_struct,
    bench_option_encoding
);

#[cfg(all(
    not(feature = "simd"),
    any(feature = "compression-lz4", feature = "compression-zstd")
))]
criterion_group!(
    benches,
    bench_encoding_comparison,
    bench_decoding_comparison,
    bench_size_comparison,
    bench_roundtrip,
    bench_compression_comparison,
    bench_primitives_scaling,
    bench_primitive_scaling,
    bench_string_encoding,
    bench_complex_struct,
    bench_option_encoding
);

#[cfg(all(
    not(feature = "simd"),
    not(any(feature = "compression-lz4", feature = "compression-zstd"))
))]
criterion_group!(
    benches,
    bench_encoding_comparison,
    bench_decoding_comparison,
    bench_size_comparison,
    bench_roundtrip,
    bench_primitives_scaling,
    bench_primitive_scaling,
    bench_string_encoding,
    bench_complex_struct,
    bench_option_encoding
);

criterion_main!(benches);
