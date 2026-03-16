#![cfg(feature = "compression-zstd")]
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
use oxicode::compression::{compress, decompress, Compression};
use oxicode::{decode_from_slice, encode_to_vec, Decode, Encode};

// ── Domain types ─────────────────────────────────────────────────────────────

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
enum VideoCodec {
    H264,
    H265,
    VP9,
    AV1,
    Unknown,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
enum StreamQuality {
    SD,
    HD,
    FHD,
    UHD4K,
    UHD8K,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct VideoChunk {
    chunk_id: u64,
    codec: VideoCodec,
    quality: StreamQuality,
    data: Vec<u8>,
    duration_ms: u32,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct StreamManifest {
    stream_id: u64,
    title: String,
    chunks: Vec<VideoChunk>,
    total_duration_ms: u64,
}

// ── Helpers ───────────────────────────────────────────────────────────────────

fn make_chunk(id: u64, codec: VideoCodec, quality: StreamQuality, size: usize) -> VideoChunk {
    VideoChunk {
        chunk_id: id,
        codec,
        quality,
        data: (0..size).map(|i| (i & 0xFF) as u8).collect(),
        duration_ms: 1000,
    }
}

// ── Tests (exactly 22) ────────────────────────────────────────────────────────

#[test]
fn test_video_chunk_zstd_roundtrip_basic() {
    let chunk = make_chunk(1, VideoCodec::H264, StreamQuality::HD, 512);
    let encoded = encode_to_vec(&chunk).expect("encode failed");
    let compressed = compress(&encoded, Compression::Zstd).expect("compress failed");
    let decompressed = decompress(&compressed).expect("decompress failed");
    let (decoded, _): (VideoChunk, usize) =
        decode_from_slice(&decompressed).expect("decode failed");
    assert_eq!(chunk, decoded);
}

#[test]
fn test_stream_manifest_zstd_roundtrip() {
    let chunks = vec![
        make_chunk(0, VideoCodec::H265, StreamQuality::FHD, 256),
        make_chunk(1, VideoCodec::AV1, StreamQuality::UHD4K, 512),
        make_chunk(2, VideoCodec::VP9, StreamQuality::SD, 128),
    ];
    let manifest = StreamManifest {
        stream_id: 9999,
        title: "Test Stream".to_string(),
        total_duration_ms: chunks.iter().map(|c| c.duration_ms as u64).sum(),
        chunks,
    };
    let encoded = encode_to_vec(&manifest).expect("encode failed");
    let compressed = compress(&encoded, Compression::Zstd).expect("compress failed");
    let decompressed = decompress(&compressed).expect("decompress failed");
    let (decoded, _): (StreamManifest, usize) =
        decode_from_slice(&decompressed).expect("decode failed");
    assert_eq!(manifest, decoded);
}

#[test]
fn test_codec_variant_h264() {
    let chunk = make_chunk(10, VideoCodec::H264, StreamQuality::HD, 64);
    let encoded = encode_to_vec(&chunk).expect("encode failed");
    let compressed = compress(&encoded, Compression::Zstd).expect("compress failed");
    let decompressed = decompress(&compressed).expect("decompress failed");
    let (decoded, _): (VideoChunk, usize) =
        decode_from_slice(&decompressed).expect("decode failed");
    assert_eq!(decoded.codec, VideoCodec::H264);
}

#[test]
fn test_codec_variant_h265() {
    let chunk = make_chunk(11, VideoCodec::H265, StreamQuality::FHD, 64);
    let encoded = encode_to_vec(&chunk).expect("encode failed");
    let compressed = compress(&encoded, Compression::Zstd).expect("compress failed");
    let decompressed = decompress(&compressed).expect("decompress failed");
    let (decoded, _): (VideoChunk, usize) =
        decode_from_slice(&decompressed).expect("decode failed");
    assert_eq!(decoded.codec, VideoCodec::H265);
}

#[test]
fn test_codec_variant_vp9() {
    let chunk = make_chunk(12, VideoCodec::VP9, StreamQuality::SD, 64);
    let encoded = encode_to_vec(&chunk).expect("encode failed");
    let compressed = compress(&encoded, Compression::Zstd).expect("compress failed");
    let decompressed = decompress(&compressed).expect("decompress failed");
    let (decoded, _): (VideoChunk, usize) =
        decode_from_slice(&decompressed).expect("decode failed");
    assert_eq!(decoded.codec, VideoCodec::VP9);
}

#[test]
fn test_codec_variant_av1() {
    let chunk = make_chunk(13, VideoCodec::AV1, StreamQuality::UHD4K, 64);
    let encoded = encode_to_vec(&chunk).expect("encode failed");
    let compressed = compress(&encoded, Compression::Zstd).expect("compress failed");
    let decompressed = decompress(&compressed).expect("decompress failed");
    let (decoded, _): (VideoChunk, usize) =
        decode_from_slice(&decompressed).expect("decode failed");
    assert_eq!(decoded.codec, VideoCodec::AV1);
}

#[test]
fn test_codec_variant_unknown() {
    let chunk = make_chunk(14, VideoCodec::Unknown, StreamQuality::SD, 64);
    let encoded = encode_to_vec(&chunk).expect("encode failed");
    let compressed = compress(&encoded, Compression::Zstd).expect("compress failed");
    let decompressed = decompress(&compressed).expect("decompress failed");
    let (decoded, _): (VideoChunk, usize) =
        decode_from_slice(&decompressed).expect("decode failed");
    assert_eq!(decoded.codec, VideoCodec::Unknown);
}

#[test]
fn test_quality_variant_sd() {
    let chunk = make_chunk(20, VideoCodec::H264, StreamQuality::SD, 32);
    let encoded = encode_to_vec(&chunk).expect("encode failed");
    let compressed = compress(&encoded, Compression::Zstd).expect("compress failed");
    let decompressed = decompress(&compressed).expect("decompress failed");
    let (decoded, _): (VideoChunk, usize) =
        decode_from_slice(&decompressed).expect("decode failed");
    assert_eq!(decoded.quality, StreamQuality::SD);
}

#[test]
fn test_quality_variant_hd() {
    let chunk = make_chunk(21, VideoCodec::H264, StreamQuality::HD, 32);
    let encoded = encode_to_vec(&chunk).expect("encode failed");
    let compressed = compress(&encoded, Compression::Zstd).expect("compress failed");
    let decompressed = decompress(&compressed).expect("decompress failed");
    let (decoded, _): (VideoChunk, usize) =
        decode_from_slice(&decompressed).expect("decode failed");
    assert_eq!(decoded.quality, StreamQuality::HD);
}

#[test]
fn test_quality_variant_fhd() {
    let chunk = make_chunk(22, VideoCodec::H265, StreamQuality::FHD, 32);
    let encoded = encode_to_vec(&chunk).expect("encode failed");
    let compressed = compress(&encoded, Compression::Zstd).expect("compress failed");
    let decompressed = decompress(&compressed).expect("decompress failed");
    let (decoded, _): (VideoChunk, usize) =
        decode_from_slice(&decompressed).expect("decode failed");
    assert_eq!(decoded.quality, StreamQuality::FHD);
}

#[test]
fn test_quality_variant_uhd4k() {
    let chunk = make_chunk(23, VideoCodec::AV1, StreamQuality::UHD4K, 32);
    let encoded = encode_to_vec(&chunk).expect("encode failed");
    let compressed = compress(&encoded, Compression::Zstd).expect("compress failed");
    let decompressed = decompress(&compressed).expect("decompress failed");
    let (decoded, _): (VideoChunk, usize) =
        decode_from_slice(&decompressed).expect("decode failed");
    assert_eq!(decoded.quality, StreamQuality::UHD4K);
}

#[test]
fn test_quality_variant_uhd8k() {
    let chunk = make_chunk(24, VideoCodec::AV1, StreamQuality::UHD8K, 32);
    let encoded = encode_to_vec(&chunk).expect("encode failed");
    let compressed = compress(&encoded, Compression::Zstd).expect("compress failed");
    let decompressed = decompress(&compressed).expect("decompress failed");
    let (decoded, _): (VideoChunk, usize) =
        decode_from_slice(&decompressed).expect("decode failed");
    assert_eq!(decoded.quality, StreamQuality::UHD8K);
}

#[test]
fn test_large_repetitive_data_compression_ratio() {
    // 1000 chunks of repeated byte patterns — highly compressible
    let chunk = VideoChunk {
        chunk_id: 100,
        codec: VideoCodec::H265,
        quality: StreamQuality::UHD4K,
        data: vec![0xABu8; 4096],
        duration_ms: 33,
    };
    let chunks: Vec<VideoChunk> = (0u64..1000)
        .map(|i| VideoChunk {
            chunk_id: i,
            ..chunk.clone()
        })
        .collect();
    let encoded = encode_to_vec(&chunks).expect("encode failed");
    let compressed = compress(&encoded, Compression::Zstd).expect("compress failed");
    assert!(
        compressed.len() < encoded.len(),
        "Zstd compressed ({} bytes) should be smaller than encoded ({} bytes) for repetitive data",
        compressed.len(),
        encoded.len()
    );
}

#[test]
fn test_empty_chunk_data_roundtrip() {
    let chunk = VideoChunk {
        chunk_id: 0,
        codec: VideoCodec::Unknown,
        quality: StreamQuality::SD,
        data: vec![],
        duration_ms: 0,
    };
    let encoded = encode_to_vec(&chunk).expect("encode failed");
    let compressed = compress(&encoded, Compression::Zstd).expect("compress failed");
    let decompressed = decompress(&compressed).expect("decompress failed");
    let (decoded, _): (VideoChunk, usize) =
        decode_from_slice(&decompressed).expect("decode failed");
    assert_eq!(chunk, decoded);
    assert!(decoded.data.is_empty());
}

#[test]
fn test_truncated_data_returns_error() {
    let chunk = make_chunk(50, VideoCodec::H264, StreamQuality::HD, 256);
    let encoded = encode_to_vec(&chunk).expect("encode failed");
    let compressed = compress(&encoded, Compression::Zstd).expect("compress failed");
    // Truncate to just 2 bytes — too short for any valid header
    let truncated = compressed[..2].to_vec();
    let result = decompress(&truncated);
    assert!(
        result.is_err(),
        "decompress() should return Err on truncated data"
    );
}

#[test]
fn test_corrupted_header_returns_error() {
    let chunk = make_chunk(51, VideoCodec::VP9, StreamQuality::FHD, 256);
    let encoded = encode_to_vec(&chunk).expect("encode failed");
    let mut compressed = compress(&encoded, Compression::Zstd).expect("compress failed");
    // Zero out bytes at positions 5..12 — destroys the zstd magic/frame header
    for byte in compressed.iter_mut().skip(5).take(7) {
        *byte = 0x00;
    }
    let result = decompress(&compressed);
    assert!(
        result.is_err(),
        "decompress() should return Err when inner zstd frame is corrupted"
    );
}

#[test]
fn test_idempotent_decompression_does_not_double_decompress() {
    let chunk = make_chunk(60, VideoCodec::AV1, StreamQuality::UHD8K, 128);
    let encoded = encode_to_vec(&chunk).expect("encode failed");
    let compressed = compress(&encoded, Compression::Zstd).expect("compress failed");
    let decompressed_once = decompress(&compressed).expect("first decompress failed");
    // Decompressed bytes are raw oxicode data, not wrapped with our magic header.
    // Attempting to decompress again must return an error.
    let result = decompress(&decompressed_once);
    assert!(
        result.is_err(),
        "decompressing plain encoded bytes should fail (no magic header)"
    );
}

#[test]
fn test_vec_of_video_chunks_roundtrip() {
    let chunks: Vec<VideoChunk> = vec![
        make_chunk(0, VideoCodec::H264, StreamQuality::SD, 100),
        make_chunk(1, VideoCodec::H265, StreamQuality::HD, 200),
        make_chunk(2, VideoCodec::VP9, StreamQuality::FHD, 300),
        make_chunk(3, VideoCodec::AV1, StreamQuality::UHD4K, 400),
        make_chunk(4, VideoCodec::Unknown, StreamQuality::UHD8K, 50),
    ];
    let encoded = encode_to_vec(&chunks).expect("encode failed");
    let compressed = compress(&encoded, Compression::Zstd).expect("compress failed");
    let decompressed = decompress(&compressed).expect("decompress failed");
    let (decoded, _): (Vec<VideoChunk>, usize) =
        decode_from_slice(&decompressed).expect("decode failed");
    assert_eq!(chunks, decoded);
}

#[test]
fn test_zstd_level_compression_roundtrip_chunk() {
    let chunk = make_chunk(70, VideoCodec::H265, StreamQuality::UHD4K, 1024);
    let encoded = encode_to_vec(&chunk).expect("encode failed");
    for level in [1u8, 3, 9, 19] {
        let compressed =
            compress(&encoded, Compression::ZstdLevel(level)).expect("compress failed");
        let decompressed = decompress(&compressed).expect("decompress failed");
        let (decoded, _): (VideoChunk, usize) =
            decode_from_slice(&decompressed).expect("decode failed");
        assert_eq!(chunk, decoded, "roundtrip failed at zstd level {level}");
    }
}

#[test]
fn test_manifest_with_many_chunks_roundtrip() {
    let chunks: Vec<VideoChunk> = (0u64..50)
        .map(|i| VideoChunk {
            chunk_id: i,
            codec: if i % 2 == 0 {
                VideoCodec::H264
            } else {
                VideoCodec::AV1
            },
            quality: StreamQuality::HD,
            data: vec![(i & 0xFF) as u8; 256],
            duration_ms: 1000,
        })
        .collect();
    let total_ms: u64 = chunks.iter().map(|c| c.duration_ms as u64).sum();
    let manifest = StreamManifest {
        stream_id: 12345,
        title: "Multi-chunk Live Stream".to_string(),
        total_duration_ms: total_ms,
        chunks,
    };
    let encoded = encode_to_vec(&manifest).expect("encode failed");
    let compressed = compress(&encoded, Compression::Zstd).expect("compress failed");
    let decompressed = decompress(&compressed).expect("decompress failed");
    let (decoded, _): (StreamManifest, usize) =
        decode_from_slice(&decompressed).expect("decode failed");
    assert_eq!(manifest, decoded);
    assert_eq!(decoded.chunks.len(), 50);
}

#[test]
fn test_stream_manifest_empty_chunks_roundtrip() {
    let manifest = StreamManifest {
        stream_id: 0,
        title: String::new(),
        chunks: vec![],
        total_duration_ms: 0,
    };
    let encoded = encode_to_vec(&manifest).expect("encode failed");
    let compressed = compress(&encoded, Compression::Zstd).expect("compress failed");
    let decompressed = decompress(&compressed).expect("decompress failed");
    let (decoded, _): (StreamManifest, usize) =
        decode_from_slice(&decompressed).expect("decode failed");
    assert_eq!(manifest, decoded);
    assert!(decoded.chunks.is_empty());
}

#[test]
fn test_chunk_max_field_values_roundtrip() {
    let chunk = VideoChunk {
        chunk_id: u64::MAX,
        codec: VideoCodec::AV1,
        quality: StreamQuality::UHD8K,
        data: vec![0xFFu8; 512],
        duration_ms: u32::MAX,
    };
    let encoded = encode_to_vec(&chunk).expect("encode failed");
    let compressed = compress(&encoded, Compression::Zstd).expect("compress failed");
    let decompressed = decompress(&compressed).expect("decompress failed");
    let (decoded, _): (VideoChunk, usize) =
        decode_from_slice(&decompressed).expect("decode failed");
    assert_eq!(chunk, decoded);
    assert_eq!(decoded.chunk_id, u64::MAX);
    assert_eq!(decoded.duration_ms, u32::MAX);
}
