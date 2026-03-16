//! Advanced combined-feature integration tests for oxicode.
//!
//! Each test exercises two or more features together to verify correct
//! interoperability.  Feature guards ensure the suite compiles cleanly
//! regardless of which optional features are enabled.

#![allow(
    clippy::approx_constant,
    clippy::useless_vec,
    clippy::len_zero,
    clippy::unnecessary_cast,
    clippy::redundant_closure,
    clippy::too_many_arguments,
    clippy::type_complexity,
    clippy::needless_borrow,
    clippy::enum_variant_names,
    clippy::upper_case_acronyms,
    clippy::inconsistent_digit_grouping,
    clippy::unit_cmp,
    clippy::assertions_on_constants,
    clippy::iter_on_single_items,
    clippy::expect_fun_call,
    clippy::redundant_pattern_matching,
    variant_size_differences,
    clippy::absurd_extreme_comparisons,
    clippy::nonminimal_bool,
    clippy::for_kv_map,
    clippy::needless_range_loop,
    clippy::single_match,
    clippy::collapsible_if,
    clippy::needless_return,
    clippy::redundant_clone,
    clippy::map_entry,
    clippy::match_single_binding,
    clippy::bool_comparison,
    clippy::derivable_impls,
    clippy::manual_range_contains,
    clippy::needless_borrows_for_generic_args,
    clippy::manual_map,
    clippy::vec_init_then_push,
    clippy::identity_op,
    clippy::manual_flatten,
    clippy::single_char_pattern,
    clippy::search_is_some,
    clippy::option_map_unit_fn,
    clippy::while_let_on_iterator,
    clippy::clone_on_copy,
    clippy::box_collection,
    clippy::redundant_field_names,
    clippy::ptr_arg,
    clippy::large_enum_variant,
    clippy::match_ref_pats,
    clippy::needless_pass_by_value,
    clippy::unused_unit,
    clippy::let_and_return,
    clippy::suspicious_else_formatting,
    clippy::manual_strip,
    clippy::match_like_matches_macro,
    clippy::from_over_into,
    clippy::wrong_self_convention,
    clippy::inherent_to_string,
    clippy::new_without_default,
    clippy::unnecessary_wraps,
    clippy::field_reassign_with_default,
    clippy::manual_find,
    clippy::unnecessary_lazy_evaluations,
    clippy::should_implement_trait,
    clippy::missing_safety_doc,
    clippy::unusual_byte_groupings,
    clippy::bool_assert_comparison,
    clippy::zero_prefixed_literal,
    clippy::await_holding_lock,
    clippy::manual_saturating_arithmetic,
    clippy::explicit_counter_loop,
    clippy::needless_lifetimes,
    clippy::single_component_path_imports,
    clippy::uninlined_format_args,
    clippy::iter_cloned_collect,
    clippy::manual_str_repeat,
    clippy::excessive_precision,
    clippy::precedence,
    clippy::unnecessary_literal_unwrap
)]
use oxicode::{Decode, Encode};

// ── module 1: LZ4 + checksum ──────────────────────────────────────────────────

#[cfg(all(feature = "compression-lz4", feature = "checksum"))]
mod lz4_checksum_combined {
    use oxicode::compression::{compress, decompress, Compression};

    /// 1. LZ4 compress → checksum wrap → checksum verify → LZ4 decompress → decode.
    #[test]
    fn test_lz4_compress_checksum_roundtrip() {
        let data: Vec<u64> = (0u64..512).collect();
        let encoded = oxicode::encode_to_vec(&data).expect("encode failed");

        let compressed = compress(&encoded, Compression::Lz4).expect("lz4 compress failed");
        let wrapped = oxicode::checksum::wrap_with_checksum(&compressed);

        let verified =
            oxicode::checksum::verify_checksum(&wrapped).expect("checksum verify failed");
        let decompressed = decompress(verified).expect("lz4 decompress failed");
        let (decoded, _): (Vec<u64>, usize) =
            oxicode::decode_from_slice(&decompressed).expect("decode failed");

        assert_eq!(data, decoded);
    }

    /// Corruption inside the compressed payload must be caught by the checksum.
    #[test]
    fn test_lz4_checksum_detects_corruption() {
        let data: Vec<u8> = vec![0xCC; 300];
        let encoded = oxicode::encode_to_vec(&data).expect("encode failed");
        let compressed = compress(&encoded, Compression::Lz4).expect("compress failed");
        let mut wrapped = oxicode::checksum::wrap_with_checksum(&compressed);

        let mid =
            oxicode::checksum::HEADER_SIZE + wrapped[oxicode::checksum::HEADER_SIZE..].len() / 2;
        wrapped[mid] ^= 0xFF;

        assert!(
            oxicode::checksum::verify_checksum(&wrapped).is_err(),
            "corrupted lz4+checksummed data must fail"
        );
    }
}

// ── module 2: zstd + derive ───────────────────────────────────────────────────

#[cfg(all(feature = "compression-zstd", feature = "derive"))]
mod zstd_derive_combined {
    use super::*;
    use oxicode::compression::{compress, decompress, Compression};

    #[derive(Debug, PartialEq, Encode, Decode)]
    struct ZstdRecord {
        id: u64,
        payload: Vec<u8>,
        label: String,
    }

    /// 2. Zstd-compress a derived struct and round-trip it.
    #[test]
    fn test_zstd_compressed_derive_roundtrip() {
        let rec = ZstdRecord {
            id: 7,
            payload: vec![0xAB; 2_000],
            label: "zstd_test".into(),
        };
        let encoded = oxicode::encode_to_vec(&rec).expect("encode failed");
        let compressed = compress(&encoded, Compression::Zstd).expect("zstd compress failed");
        assert!(compressed.len() < encoded.len(), "should compress");

        let decompressed = decompress(&compressed).expect("zstd decompress failed");
        let (decoded, _): (ZstdRecord, usize) =
            oxicode::decode_from_slice(&decompressed).expect("decode failed");
        assert_eq!(rec, decoded);
    }
}

// ── module 3: checksum + versioning ──────────────────────────────────────────

#[cfg(all(feature = "checksum", feature = "derive"))]
mod checksum_versioning_combined {
    use super::*;
    use oxicode::versioning::Version;

    #[derive(Debug, PartialEq, Encode, Decode)]
    struct VersionedPayload {
        name: String,
        score: f64,
    }

    /// 3. Encode with version header, wrap with checksum, verify, then decode versioned.
    #[test]
    fn test_checksum_versioning_roundtrip() {
        use std::f64::consts::PI;

        let payload = VersionedPayload {
            name: "pi_record".into(),
            score: PI,
        };
        let version = Version::new(2, 1, 0);

        let versioned =
            oxicode::encode_versioned_value(&payload, version).expect("versioned encode failed");
        let wrapped = oxicode::checksum::wrap_with_checksum(&versioned);
        let verified =
            oxicode::checksum::verify_checksum(&wrapped).expect("checksum verify failed");
        let (decoded, decoded_version, _): (VersionedPayload, Version, usize) =
            oxicode::decode_versioned_value(verified).expect("versioned decode failed");

        assert_eq!(payload, decoded);
        assert_eq!(version, decoded_version);
        assert!((decoded.score - PI).abs() < 1e-15);
    }

    /// Version mismatch resilience: different version values are faithfully preserved.
    #[test]
    fn test_checksum_versioning_multiple_versions() {
        let versions = [
            Version::new(1, 0, 0),
            Version::new(0, 9, 9),
            Version::new(100, 200, 255),
        ];
        for version in versions {
            let value: u32 = 42;
            let versioned =
                oxicode::encode_versioned_value(&value, version).expect("encode failed");
            let wrapped = oxicode::checksum::wrap_with_checksum(&versioned);
            let verified = oxicode::checksum::verify_checksum(&wrapped).expect("verify failed");
            let (decoded, decoded_ver, _): (u32, Version, usize) =
                oxicode::decode_versioned_value(verified).expect("decode failed");
            assert_eq!(value, decoded);
            assert_eq!(version, decoded_ver);
        }
    }
}

// ── module 4: streaming + derive ─────────────────────────────────────────────

#[cfg(all(feature = "std", feature = "derive"))]
mod streaming_derive_advanced {
    use super::*;
    use oxicode::streaming::{StreamingDecoder, StreamingEncoder};
    use std::io::Cursor;

    #[derive(Debug, PartialEq, Encode, Decode)]
    struct Telemetry {
        timestamp: u64,
        value: f32,
        sensor_id: u16,
    }

    /// 4. Stream-encode derived structs, then stream-decode and verify all items.
    #[test]
    fn test_streaming_derived_structs_roundtrip() {
        let items: Vec<Telemetry> = (0u64..20)
            .map(|i| Telemetry {
                timestamp: i * 1_000,
                value: (i as f32) * 0.1,
                sensor_id: (i % 8) as u16,
            })
            .collect();

        let mut buf = Vec::new();
        {
            let mut encoder = StreamingEncoder::new(&mut buf);
            for item in &items {
                encoder.write_item(item).expect("write_item failed");
            }
            encoder.finish().expect("finish failed");
        }

        let mut decoder = StreamingDecoder::new(Cursor::new(&buf));
        let decoded: Vec<Telemetry> = decoder.read_all().expect("read_all failed");

        assert_eq!(items, decoded);
    }
}

// ── module 5: serde + checksum ────────────────────────────────────────────────

#[cfg(all(feature = "serde", feature = "checksum"))]
mod serde_checksum_advanced {
    use serde::{Deserialize, Serialize};

    #[derive(Debug, PartialEq, Serialize, Deserialize)]
    struct SerdeRecord {
        id: u64,
        tags: Vec<String>,
        score: f64,
    }

    /// 5. Serde-encode, wrap with checksum, verify, serde-decode.
    #[test]
    fn test_serde_encode_with_checksum_verify() {
        use std::f64::consts::E;

        let record = SerdeRecord {
            id: 1,
            tags: vec!["alpha".into(), "beta".into()],
            score: E,
        };

        let encoded = oxicode::serde::encode_serde(&record).expect("serde encode failed");
        let wrapped = oxicode::checksum::wrap_with_checksum(&encoded);
        let verified =
            oxicode::checksum::verify_checksum(&wrapped).expect("checksum verify failed");
        let decoded: SerdeRecord =
            oxicode::serde::decode_serde(verified).expect("serde decode failed");

        assert_eq!(record, decoded);
        assert!((decoded.score - E).abs() < 1e-15);
    }

    /// Checksum must catch serde payload corruption.
    #[test]
    fn test_serde_checksum_corruption_detected() {
        let record = SerdeRecord {
            id: 999,
            tags: vec!["x".into()],
            score: 1.0,
        };
        let encoded = oxicode::serde::encode_serde(&record).expect("serde encode failed");
        let mut wrapped = oxicode::checksum::wrap_with_checksum(&encoded);
        wrapped[oxicode::checksum::HEADER_SIZE] ^= 0x01;

        assert!(
            oxicode::checksum::verify_checksum(&wrapped).is_err(),
            "corrupted serde+checksum must fail"
        );
    }
}

// ── module 6: LZ4 + Vec<derived struct> ──────────────────────────────────────

#[cfg(all(feature = "compression-lz4", feature = "derive"))]
mod lz4_derive_collection {
    use super::*;
    use oxicode::compression::{compress, decompress, Compression};

    #[derive(Debug, PartialEq, Encode, Decode)]
    struct Item {
        id: u32,
        value: Vec<u8>,
    }

    /// 6. Compress a Vec<derived struct> with LZ4 and verify roundtrip.
    #[test]
    fn test_lz4_compress_derive_collection() {
        let items: Vec<Item> = (0u32..50)
            .map(|i| Item {
                id: i,
                value: vec![(i % 256) as u8; 100],
            })
            .collect();

        let encoded = oxicode::encode_to_vec(&items).expect("encode failed");
        let compressed = compress(&encoded, Compression::Lz4).expect("compress failed");
        assert!(compressed.len() < encoded.len(), "should compress");

        let decompressed = decompress(&compressed).expect("decompress failed");
        let (decoded, _): (Vec<Item>, usize) =
            oxicode::decode_from_slice(&decompressed).expect("decode failed");

        assert_eq!(items, decoded);
    }
}

// ── module 7: versioning + LZ4 ───────────────────────────────────────────────

#[cfg(all(feature = "compression-lz4", feature = "derive"))]
mod versioning_lz4_combined {
    use super::*;
    use oxicode::compression::{compress, decompress, Compression};
    use oxicode::versioning::Version;

    #[derive(Debug, PartialEq, Encode, Decode)]
    struct Archive {
        entries: Vec<String>,
        created_at: u64,
    }

    /// 7. Version header → LZ4 compress → decompress → versioned decode.
    #[test]
    fn test_versioned_lz4_roundtrip() {
        let archive = Archive {
            entries: (0..30).map(|i| format!("entry_{:04}", i)).collect(),
            created_at: 1_700_000_000,
        };
        let version = Version::new(3, 0, 1);

        let versioned =
            oxicode::encode_versioned_value(&archive, version).expect("versioned encode failed");
        let compressed = compress(&versioned, Compression::Lz4).expect("compress failed");

        let decompressed = decompress(&compressed).expect("decompress failed");
        let (decoded, decoded_ver, _): (Archive, Version, usize) =
            oxicode::decode_versioned_value(&decompressed).expect("versioned decode failed");

        assert_eq!(archive, decoded);
        assert_eq!(version, decoded_ver);
    }
}

// ── module 8: async streaming + derive ───────────────────────────────────────

#[cfg(all(feature = "async-tokio", feature = "derive"))]
mod async_streaming_derive_advanced {
    use super::*;
    use oxicode::streaming::{AsyncStreamingDecoder, AsyncStreamingEncoder};
    use std::io::Cursor;

    #[derive(Debug, PartialEq, Encode, Decode)]
    struct AsyncEvent {
        seq: u32,
        data: Vec<u8>,
        label: String,
    }

    /// 8. Async stream-encode a derived struct, then async decode it.
    #[tokio::test]
    async fn test_async_stream_derived_struct_roundtrip() {
        let events: Vec<AsyncEvent> = (0u32..5)
            .map(|i| AsyncEvent {
                seq: i,
                data: vec![i as u8; 10],
                label: format!("event_{}", i),
            })
            .collect();

        let mut buf = Vec::new();
        {
            let cursor = Cursor::new(&mut buf);
            let mut encoder = AsyncStreamingEncoder::new(cursor);
            for event in &events {
                encoder
                    .write_item(event)
                    .await
                    .expect("async write_item failed");
            }
            encoder.finish().await.expect("async finish failed");
        }

        let cursor = Cursor::new(buf);
        let mut decoder = AsyncStreamingDecoder::new(cursor);
        let mut decoded: Vec<AsyncEvent> = Vec::new();
        while let Some(event) = decoder
            .read_item::<AsyncEvent>()
            .await
            .expect("async read failed")
        {
            decoded.push(event);
        }

        assert_eq!(events, decoded);
    }
}

// ── module 9: checksum + large data ──────────────────────────────────────────

#[cfg(feature = "checksum")]
mod checksum_large_data {
    /// 9. Wrap 50 KB of synthetic data with checksum, verify integrity.
    #[test]
    fn test_checksum_large_50kb_payload() {
        let data: Vec<u8> = (0u32..50_000).map(|i| (i % 256) as u8).collect();
        let encoded = oxicode::encode_to_vec(&data).expect("encode failed");

        let wrapped = oxicode::checksum::wrap_with_checksum(&encoded);
        assert_eq!(
            wrapped.len(),
            oxicode::checksum::HEADER_SIZE + encoded.len()
        );

        let verified = oxicode::checksum::verify_checksum(&wrapped).expect("verify failed");
        let (decoded, _): (Vec<u8>, usize) =
            oxicode::decode_from_slice(verified).expect("decode failed");

        assert_eq!(data, decoded);
    }
}

// ── module 10: LZ4 + BTreeMap<String, Vec<u32>> ──────────────────────────────

#[cfg(feature = "compression-lz4")]
mod lz4_complex_collection {
    use oxicode::compression::{compress, decompress, Compression};
    use std::collections::BTreeMap;

    /// 10. Compress a BTreeMap<String, Vec<u32>> with LZ4 and round-trip.
    #[test]
    fn test_lz4_btreemap_string_vec_u32() {
        let mut map: BTreeMap<String, Vec<u32>> = BTreeMap::new();
        for i in 0u32..20 {
            map.insert(format!("key_{:02}", i), (i..i + 10).collect());
        }

        let encoded = oxicode::encode_to_vec(&map).expect("encode failed");
        let compressed = compress(&encoded, Compression::Lz4).expect("compress failed");

        let decompressed = decompress(&compressed).expect("decompress failed");
        let (decoded, _): (BTreeMap<String, Vec<u32>>, usize) =
            oxicode::decode_from_slice(&decompressed).expect("decode failed");

        assert_eq!(map, decoded);
    }
}

// ── module 11: simd + large array ────────────────────────────────────────────

#[cfg(feature = "simd")]
mod simd_large_array {
    /// 11. SIMD-feature encode/decode of a large f64 array using PI-derived values.
    #[test]
    fn test_simd_large_f64_array_roundtrip() {
        use std::f64::consts::PI;

        let data: Vec<f64> = (0..1024).map(|i| PI * (i as f64)).collect();
        let encoded = oxicode::encode_to_vec(&data).expect("encode failed");
        let (decoded, _): (Vec<f64>, usize) =
            oxicode::decode_from_slice(&encoded).expect("decode failed");

        assert_eq!(data.len(), decoded.len());
        for (a, b) in data.iter().zip(decoded.iter()) {
            assert!((a - b).abs() < 1e-15, "f64 mismatch: {} vs {}", a, b);
        }
    }
}

// ── module 12: config (fixed_int) + derive ───────────────────────────────────

#[cfg(feature = "derive")]
mod config_fixed_int_derive {
    use super::*;

    #[derive(Debug, PartialEq, Encode, Decode)]
    struct FixedIntStruct {
        a: u32,
        b: u64,
        c: i32,
    }

    /// 12. Encode with fixed_int_encoding config + derived struct.
    #[test]
    fn test_config_fixed_int_with_derived_struct() {
        let cfg = oxicode::config::standard().with_fixed_int_encoding();
        let s = FixedIntStruct {
            a: 0xFF_FF_FF_FF,
            b: 0xDEAD_BEEF_CAFE_1234,
            c: -42,
        };

        let encoded = oxicode::encode_to_vec_with_config(&s, cfg).expect("encode failed");
        // fixed-int: u32=4, u64=8, i32=4 = 16 bytes
        assert_eq!(
            encoded.len(),
            16,
            "fixed_int encoding must be exactly 16 bytes"
        );

        let (decoded, _): (FixedIntStruct, usize) =
            oxicode::decode_from_slice_with_config(&encoded, cfg).expect("decode failed");
        assert_eq!(s, decoded);
    }
}

// ── module 13: config (big_endian) + primitive types ─────────────────────────

/// 13. Big-endian config round-trip for multiple primitive types.
#[test]
fn test_config_big_endian_primitives_roundtrip() {
    let cfg = oxicode::config::standard()
        .with_big_endian()
        .with_fixed_int_encoding();

    let val_u32: u32 = 0x01_02_03_04;
    let val_u64: u64 = 0x0A_0B_0C_0D_0E_0F_10_11;
    let val_i16: i16 = -1234;

    let enc_u32 = oxicode::encode_to_vec_with_config(&val_u32, cfg).expect("encode u32");
    let enc_u64 = oxicode::encode_to_vec_with_config(&val_u64, cfg).expect("encode u64");
    let enc_i16 = oxicode::encode_to_vec_with_config(&val_i16, cfg).expect("encode i16");

    // BE layout verification
    assert_eq!(enc_u32, &[0x01, 0x02, 0x03, 0x04]);
    assert_eq!(enc_u64, &[0x0A, 0x0B, 0x0C, 0x0D, 0x0E, 0x0F, 0x10, 0x11]);

    let (dec_u32, _): (u32, _) =
        oxicode::decode_from_slice_with_config(&enc_u32, cfg).expect("decode u32");
    let (dec_u64, _): (u64, _) =
        oxicode::decode_from_slice_with_config(&enc_u64, cfg).expect("decode u64");
    let (dec_i16, _): (i16, _) =
        oxicode::decode_from_slice_with_config(&enc_i16, cfg).expect("decode i16");

    assert_eq!(val_u32, dec_u32);
    assert_eq!(val_u64, dec_u64);
    assert_eq!(val_i16, dec_i16);
}

// ── module 14: checksum + compression combo ───────────────────────────────────

#[cfg(all(feature = "compression-lz4", feature = "checksum"))]
mod checksum_of_compressed {
    use oxicode::compression::{compress, decompress, Compression};

    /// 14. Checksum the compressed form of data (inner checksum of compressed bytes).
    #[test]
    fn test_checksum_of_compressed_data() {
        let data: Vec<u32> = (0u32..200).collect();
        let encoded = oxicode::encode_to_vec(&data).expect("encode failed");
        let compressed = compress(&encoded, Compression::Lz4).expect("compress failed");

        // Wrap the *compressed* bytes with a checksum
        let wrapped = oxicode::checksum::wrap_with_checksum(&compressed);
        let verified = oxicode::checksum::verify_checksum(&wrapped).expect("verify failed");

        // Verified bytes are the compressed payload — decompress and decode
        let decompressed = decompress(verified).expect("decompress failed");
        let (decoded, _): (Vec<u32>, usize) =
            oxicode::decode_from_slice(&decompressed).expect("decode failed");

        assert_eq!(data, decoded);
    }
}

// ── module 15: versioning + derive enum ──────────────────────────────────────

#[cfg(feature = "derive")]
mod versioning_derive_enum {
    use super::*;
    use oxicode::versioning::Version;

    #[derive(Debug, PartialEq, Encode, Decode)]
    #[oxicode(tag_type = "u8")]
    enum Command {
        Ping,
        Pong,
        Data { payload: Vec<u8> },
        Error { code: u32, message: String },
    }

    /// 15. Versioned round-trip for a derived enum with multiple variants.
    #[test]
    fn test_versioned_derive_enum_roundtrip() {
        let commands = vec![
            Command::Ping,
            Command::Pong,
            Command::Data {
                payload: vec![1, 2, 3, 4],
            },
            Command::Error {
                code: 404,
                message: "not found".into(),
            },
        ];
        let version = Version::new(1, 5, 2);

        for cmd in &commands {
            let versioned =
                oxicode::encode_versioned_value(cmd, version).expect("versioned encode failed");
            let (decoded, decoded_ver, _): (Command, Version, usize) =
                oxicode::decode_versioned_value(&versioned).expect("versioned decode failed");
            assert_eq!(*cmd, decoded);
            assert_eq!(version, decoded_ver);
        }
    }
}

// ── module 16: streaming + checksum ──────────────────────────────────────────

#[cfg(all(feature = "std", feature = "checksum"))]
mod streaming_checksum_combined {
    use oxicode::streaming::{StreamingDecoder, StreamingEncoder};
    use std::io::Cursor;

    /// 16. Stream-encode items, then wrap the entire stream buffer with a checksum.
    #[test]
    fn test_streaming_then_checksum_wrap() {
        let items: Vec<u32> = (0u32..100).collect();

        let mut stream_buf = Vec::new();
        {
            let mut encoder = StreamingEncoder::new(&mut stream_buf);
            for item in &items {
                encoder.write_item(item).expect("write_item failed");
            }
            encoder.finish().expect("finish failed");
        }

        // Wrap the stream bytes with checksum for integrity
        let wrapped = oxicode::checksum::wrap_with_checksum(&stream_buf);
        let verified =
            oxicode::checksum::verify_checksum(&wrapped).expect("checksum verify failed");

        // Decode the verified stream
        let mut decoder = StreamingDecoder::new(Cursor::new(verified));
        let decoded: Vec<u32> = decoder.read_all().expect("read_all failed");
        assert_eq!(items, decoded);
    }
}

// ── module 17: compression ratio test ────────────────────────────────────────

#[cfg(feature = "compression-lz4")]
mod compression_ratio_test {
    use oxicode::compression::{compress_with_stats, Compression};

    /// 17. Verify compression ratio on highly-repetitive encoded data.
    #[test]
    fn test_lz4_compression_ratio_on_repetitive_data() {
        // Highly compressible: all zeros
        let data: Vec<u8> = vec![0u8; 10_000];
        let encoded = oxicode::encode_to_vec(&data).expect("encode failed");

        let (compressed, stats) =
            compress_with_stats(&encoded, Compression::Lz4).expect("compress failed");

        assert!(
            stats.ratio() > 1.5,
            "expected ratio > 1.5 for repetitive data, got {}",
            stats.ratio()
        );
        assert!(
            stats.savings_percent() > 30.0,
            "expected savings > 30%, got {}",
            stats.savings_percent()
        );
        assert!(
            compressed.len() < encoded.len(),
            "compressed must be smaller than original"
        );
    }
}

// ── module 18: BorrowDecode + checksum ───────────────────────────────────────

#[cfg(feature = "checksum")]
mod borrow_decode_checksum {
    /// 18. Wrap bytes with checksum, verify, borrow-decode back (zero-copy).
    #[test]
    fn test_borrow_decode_from_checksummed_bytes() {
        let original = "hello zero-copy world";
        let encoded = oxicode::encode_to_vec(&original).expect("encode failed");

        let wrapped = oxicode::checksum::wrap_with_checksum(&encoded);
        let verified =
            oxicode::checksum::verify_checksum(&wrapped).expect("checksum verify failed");

        // borrow_decode_from_slice yields a &str borrowing from `verified`
        let (decoded, _): (&str, usize) =
            oxicode::borrow_decode_from_slice(verified).expect("borrow decode failed");

        assert_eq!(original, decoded);
    }
}

// ── module 19: LZ4 + checksum + versioning + derive ──────────────────────────

#[cfg(all(feature = "compression-lz4", feature = "checksum", feature = "derive"))]
mod all_features_combined {
    use super::*;
    use oxicode::compression::{compress, decompress, Compression};
    use oxicode::versioning::Version;

    #[derive(Debug, PartialEq, Encode, Decode)]
    struct MegaRecord {
        id: u64,
        tags: Vec<String>,
        matrix: Vec<Vec<u32>>,
    }

    /// 19. LZ4 + checksum + versioning + derive all together.
    #[test]
    fn test_all_features_combined_roundtrip() {
        let record = MegaRecord {
            id: 0xDEAD_BEEF,
            tags: vec!["foo".into(), "bar".into(), "baz".into()],
            matrix: vec![vec![1, 2, 3], vec![4, 5, 6], vec![7, 8, 9]],
        };
        let version = Version::new(4, 2, 1);

        // Step 1: versioned encode
        let versioned =
            oxicode::encode_versioned_value(&record, version).expect("versioned encode failed");
        // Step 2: LZ4 compress
        let compressed = compress(&versioned, Compression::Lz4).expect("compress failed");
        // Step 3: checksum wrap
        let wrapped = oxicode::checksum::wrap_with_checksum(&compressed);

        // Decode in reverse order
        let verified = oxicode::checksum::verify_checksum(&wrapped).expect("verify failed");
        let decompressed = decompress(verified).expect("decompress failed");
        let (decoded, decoded_ver, _): (MegaRecord, Version, usize) =
            oxicode::decode_versioned_value(&decompressed).expect("versioned decode failed");

        assert_eq!(record, decoded);
        assert_eq!(version, decoded_ver);
    }
}

// ── module 20: serde + versioning ────────────────────────────────────────────

#[cfg(all(feature = "serde", feature = "derive"))]
mod serde_versioning_combined {
    use oxicode::versioning::Version;
    use serde::{Deserialize, Serialize};

    #[derive(Debug, PartialEq, Serialize, Deserialize)]
    struct SerdeVersionedConfig {
        endpoint: String,
        retries: u8,
        timeout_ms: u64,
    }

    /// 20. Serde-encode, wrap in a version header, decode versioned then serde-decode.
    #[test]
    fn test_serde_with_version_header() {
        let cfg = SerdeVersionedConfig {
            endpoint: "https://example.com/api".into(),
            retries: 3,
            timeout_ms: 5_000,
        };
        let version = Version::new(1, 2, 3);

        // Serde → encode → version-wrap
        let serde_bytes = oxicode::serde::encode_serde(&cfg).expect("serde encode failed");
        let versioned = oxicode::encode_versioned_value(&serde_bytes, version)
            .expect("versioned encode failed");

        // Decode version wrapper to recover serde bytes
        let (recovered_bytes, decoded_ver, _): (Vec<u8>, Version, usize) =
            oxicode::decode_versioned_value(&versioned).expect("versioned decode failed");

        assert_eq!(version, decoded_ver);

        let decoded: SerdeVersionedConfig =
            oxicode::serde::decode_serde(&recovered_bytes).expect("serde decode failed");
        assert_eq!(cfg, decoded);
    }
}

// ── module 21: config (limit) + compression ──────────────────────────────────

#[cfg(feature = "compression-lz4")]
mod config_limit_compression {
    use oxicode::compression::{compress, decompress, Compression};

    /// 21. Encode with a byte limit, then LZ4-compress the encoded result.
    ///
    /// The limit is set generously so that the encode succeeds; the test
    /// verifies that the encode-then-compress-then-decompress-then-decode
    /// cycle is correct when a custom limit config is applied during encode.
    #[test]
    fn test_encode_with_limit_then_compress() {
        let cfg = oxicode::config::standard().with_limit::<{ 1024 * 1024 }>();
        let data: Vec<u64> = (0u64..128).collect();

        let encoded = oxicode::encode_to_vec_with_config(&data, cfg).expect("encode failed");
        let compressed = compress(&encoded, Compression::Lz4).expect("compress failed");
        let decompressed = decompress(&compressed).expect("decompress failed");

        let (decoded, _): (Vec<u64>, usize) =
            oxicode::decode_from_slice_with_config(&decompressed, cfg).expect("decode failed");

        assert_eq!(data, decoded);
    }
}

// ── module 22: streaming large dataset ───────────────────────────────────────

#[cfg(feature = "std")]
mod streaming_large_dataset {
    use oxicode::streaming::{StreamingDecoder, StreamingEncoder};
    use std::io::Cursor;

    /// 22. Stream-encode 10 000 items, stream-decode all, verify count and values.
    #[test]
    fn test_streaming_10000_items() {
        const N: u32 = 10_000;
        let items: Vec<u32> = (0..N).collect();

        let mut buf = Vec::new();
        {
            let mut encoder = StreamingEncoder::new(&mut buf);
            for item in &items {
                encoder.write_item(item).expect("write_item failed");
            }
            encoder.finish().expect("finish failed");
        }

        let mut decoder = StreamingDecoder::new(Cursor::new(&buf));
        let decoded: Vec<u32> = decoder.read_all().expect("read_all failed");

        assert_eq!(decoded.len(), N as usize);
        for (expected, got) in items.iter().zip(decoded.iter()) {
            assert_eq!(expected, got);
        }
    }
}
