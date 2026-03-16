#![cfg(all(feature = "compression-lz4", feature = "compression-zstd"))]
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

// ── domain types ────────────────────────────────────────────────────────────

#[derive(Debug, PartialEq, Encode, Decode)]
enum VideoCodec {
    H264,
    H265,
    Vp8,
    Vp9,
    Av1,
}

#[derive(Debug, PartialEq, Encode, Decode)]
struct VideoChunk {
    sequence: u32,
    codec: VideoCodec,
    data: Vec<u8>,
    keyframe: bool,
    timestamp_ms: u64,
}

#[derive(Debug, PartialEq, Encode, Decode)]
struct StreamManifest {
    stream_id: u64,
    title: String,
    codec: VideoCodec,
    duration_ms: u64,
    chunk_count: u32,
}

// ── helpers ──────────────────────────────────────────────────────────────────

fn make_video_chunk(seq: u32, codec: VideoCodec, size: usize, keyframe: bool) -> VideoChunk {
    let data: Vec<u8> = (0..size)
        .map(|i| ((i * 3 + seq as usize) & 0xFF) as u8)
        .collect();
    VideoChunk {
        sequence: seq,
        codec,
        data,
        keyframe,
        timestamp_ms: seq as u64 * 33,
    }
}

fn make_manifest(stream_id: u64, codec: VideoCodec, chunk_count: u32) -> StreamManifest {
    StreamManifest {
        stream_id,
        title: format!("stream-{stream_id}-title"),
        codec,
        duration_ms: chunk_count as u64 * 33,
        chunk_count,
    }
}

// ── Test 1: VideoChunk H264 LZ4 roundtrip ────────────────────────────────────
#[test]
fn test_video_chunk_h264_lz4_roundtrip() {
    let chunk = make_video_chunk(0, VideoCodec::H264, 128, true);
    let encoded = encode_to_vec(&chunk).expect("encode VideoChunk H264");
    let compressed = compress(&encoded, Compression::Lz4).expect("lz4 compress VideoChunk H264");
    let decompressed = decompress(&compressed).expect("lz4 decompress VideoChunk H264");
    let (decoded, _): (VideoChunk, usize) =
        decode_from_slice(&decompressed).expect("decode VideoChunk H264 lz4");
    assert_eq!(chunk, decoded);
}

// ── Test 2: VideoChunk H265 Zstd roundtrip ───────────────────────────────────
#[test]
fn test_video_chunk_h265_zstd_roundtrip() {
    let chunk = make_video_chunk(1, VideoCodec::H265, 128, false);
    let encoded = encode_to_vec(&chunk).expect("encode VideoChunk H265");
    let compressed = compress(&encoded, Compression::Zstd).expect("zstd compress VideoChunk H265");
    let decompressed = decompress(&compressed).expect("zstd decompress VideoChunk H265");
    let (decoded, _): (VideoChunk, usize) =
        decode_from_slice(&decompressed).expect("decode VideoChunk H265 zstd");
    assert_eq!(chunk, decoded);
}

// ── Test 3: StreamManifest Vp8 LZ4 roundtrip ─────────────────────────────────
#[test]
fn test_stream_manifest_vp8_lz4_roundtrip() {
    let manifest = make_manifest(0xABCD_1234, VideoCodec::Vp8, 120);
    let encoded = encode_to_vec(&manifest).expect("encode StreamManifest Vp8");
    let compressed = compress(&encoded, Compression::Lz4).expect("lz4 compress StreamManifest Vp8");
    let decompressed = decompress(&compressed).expect("lz4 decompress StreamManifest Vp8");
    let (decoded, _): (StreamManifest, usize) =
        decode_from_slice(&decompressed).expect("decode StreamManifest Vp8 lz4");
    assert_eq!(manifest, decoded);
}

// ── Test 4: StreamManifest Vp9 Zstd roundtrip ────────────────────────────────
#[test]
fn test_stream_manifest_vp9_zstd_roundtrip() {
    let manifest = make_manifest(0xDEAD_BEEF, VideoCodec::Vp9, 240);
    let encoded = encode_to_vec(&manifest).expect("encode StreamManifest Vp9");
    let compressed =
        compress(&encoded, Compression::Zstd).expect("zstd compress StreamManifest Vp9");
    let decompressed = decompress(&compressed).expect("zstd decompress StreamManifest Vp9");
    let (decoded, _): (StreamManifest, usize) =
        decode_from_slice(&decompressed).expect("decode StreamManifest Vp9 zstd");
    assert_eq!(manifest, decoded);
}

// ── Test 5: StreamManifest Av1 LZ4 roundtrip ─────────────────────────────────
#[test]
fn test_stream_manifest_av1_lz4_roundtrip() {
    let manifest = make_manifest(1, VideoCodec::Av1, 480);
    let encoded = encode_to_vec(&manifest).expect("encode StreamManifest Av1");
    let compressed = compress(&encoded, Compression::Lz4).expect("lz4 compress StreamManifest Av1");
    let decompressed = decompress(&compressed).expect("lz4 decompress StreamManifest Av1");
    let (decoded, _): (StreamManifest, usize) =
        decode_from_slice(&decompressed).expect("decode StreamManifest Av1 lz4");
    assert_eq!(manifest, decoded);
}

// ── Test 6: LZ4 vs Zstd produce different full byte sequences ─────────────────
#[test]
fn test_lz4_and_zstd_produce_different_full_byte_sequences() {
    let chunk = make_video_chunk(7, VideoCodec::H264, 256, true);
    let encoded = encode_to_vec(&chunk).expect("encode VideoChunk for byte-sequence comparison");
    let lz4_bytes =
        compress(&encoded, Compression::Lz4).expect("lz4 compress for byte-sequence comparison");
    let zstd_bytes =
        compress(&encoded, Compression::Zstd).expect("zstd compress for byte-sequence comparison");
    // The full byte sequences must differ (different algorithms, different output)
    assert_ne!(
        lz4_bytes, zstd_bytes,
        "LZ4 and Zstd must produce different full compressed byte sequences"
    );
}

// ── Test 7: LZ4 and Zstd both decompress to identical bytes ──────────────────
#[test]
fn test_lz4_and_zstd_both_decompress_to_same_bytes() {
    let chunk = make_video_chunk(10, VideoCodec::Vp9, 200, false);
    let original = encode_to_vec(&chunk).expect("encode VideoChunk for cross-decompress check");
    let lz4_compressed =
        compress(&original, Compression::Lz4).expect("lz4 compress for cross-decompress");
    let zstd_compressed =
        compress(&original, Compression::Zstd).expect("zstd compress for cross-decompress");
    let lz4_out = decompress(&lz4_compressed).expect("lz4 decompress cross-decompress");
    let zstd_out = decompress(&zstd_compressed).expect("zstd decompress cross-decompress");
    assert_eq!(
        lz4_out, zstd_out,
        "LZ4 and Zstd must decompress to identical bytes"
    );
    assert_eq!(
        original, lz4_out,
        "LZ4 decompressed bytes must match original encoded bytes"
    );
}

// ── Test 8: Repetitive video data compresses well with LZ4 ───────────────────
#[test]
fn test_repetitive_video_data_lz4_compression_ratio() {
    // A chunk whose data is all-zeros is maximally compressible
    let chunk = VideoChunk {
        sequence: 99,
        codec: VideoCodec::H264,
        data: vec![0u8; 8192],
        keyframe: true,
        timestamp_ms: 3267,
    };
    let encoded = encode_to_vec(&chunk).expect("encode repetitive VideoChunk");
    let compressed =
        compress(&encoded, Compression::Lz4).expect("lz4 compress repetitive VideoChunk");
    assert!(
        compressed.len() < encoded.len(),
        "LZ4 should compress repetitive video data: compressed={} raw={}",
        compressed.len(),
        encoded.len()
    );
}

// ── Test 9: Repetitive video data compresses well with Zstd ──────────────────
#[test]
fn test_repetitive_video_data_zstd_compression_ratio() {
    let chunk = VideoChunk {
        sequence: 100,
        codec: VideoCodec::Av1,
        data: vec![0xBBu8; 8192],
        keyframe: false,
        timestamp_ms: 3300,
    };
    let encoded = encode_to_vec(&chunk).expect("encode repetitive VideoChunk zstd");
    let compressed =
        compress(&encoded, Compression::Zstd).expect("zstd compress repetitive VideoChunk");
    assert!(
        compressed.len() < encoded.len(),
        "Zstd should compress repetitive video data: compressed={} raw={}",
        compressed.len(),
        encoded.len()
    );
}

// ── Test 10: LZ4 corruption detection ────────────────────────────────────────
#[test]
fn test_lz4_corruption_detection() {
    let chunk = make_video_chunk(20, VideoCodec::H265, 512, true);
    let encoded = encode_to_vec(&chunk).expect("encode VideoChunk for lz4 corruption test");
    let compressed =
        compress(&encoded, Compression::Lz4).expect("lz4 compress for corruption test");
    let mut corrupted = compressed.clone();
    for b in corrupted[4..].iter_mut() {
        *b ^= 0xFF;
    }
    let result = decompress(&corrupted);
    assert!(
        result.is_err(),
        "Decompressing LZ4-corrupted data must return an error"
    );
}

// ── Test 11: Zstd corruption detection ───────────────────────────────────────
#[test]
fn test_zstd_corruption_detection() {
    let chunk = make_video_chunk(21, VideoCodec::Vp8, 512, false);
    let encoded = encode_to_vec(&chunk).expect("encode VideoChunk for zstd corruption test");
    let compressed =
        compress(&encoded, Compression::Zstd).expect("zstd compress for corruption test");
    let mut corrupted = compressed.clone();
    for b in corrupted[4..].iter_mut() {
        *b ^= 0xFF;
    }
    let result = decompress(&corrupted);
    assert!(
        result.is_err(),
        "Decompressing Zstd-corrupted data must return an error"
    );
}

// ── Test 12: Large VideoChunk LZ4 roundtrip ──────────────────────────────────
#[test]
fn test_large_video_chunk_lz4_roundtrip() {
    let chunk = make_video_chunk(30, VideoCodec::H264, 65536, true);
    let encoded = encode_to_vec(&chunk).expect("encode large VideoChunk");
    let compressed = compress(&encoded, Compression::Lz4).expect("lz4 compress large VideoChunk");
    let decompressed = decompress(&compressed).expect("lz4 decompress large VideoChunk");
    let (decoded, _): (VideoChunk, usize) =
        decode_from_slice(&decompressed).expect("decode large VideoChunk lz4");
    assert_eq!(chunk, decoded);
    assert_eq!(decoded.data.len(), 65536);
}

// ── Test 13: Large VideoChunk Zstd roundtrip ─────────────────────────────────
#[test]
fn test_large_video_chunk_zstd_roundtrip() {
    let chunk = make_video_chunk(31, VideoCodec::Av1, 65536, false);
    let encoded = encode_to_vec(&chunk).expect("encode large VideoChunk zstd");
    let compressed = compress(&encoded, Compression::Zstd).expect("zstd compress large VideoChunk");
    let decompressed = decompress(&compressed).expect("zstd decompress large VideoChunk");
    let (decoded, _): (VideoChunk, usize) =
        decode_from_slice(&decompressed).expect("decode large VideoChunk zstd");
    assert_eq!(chunk, decoded);
    assert_eq!(decoded.data.len(), 65536);
}

// ── Test 14: All VideoCodec variants round-trip via LZ4 ──────────────────────
#[test]
fn test_all_video_codec_variants_lz4_roundtrip() {
    let codecs = [
        VideoCodec::H264,
        VideoCodec::H265,
        VideoCodec::Vp8,
        VideoCodec::Vp9,
        VideoCodec::Av1,
    ];
    for (idx, codec) in codecs.into_iter().enumerate() {
        let chunk = make_video_chunk(idx as u32, codec, 64, idx % 2 == 0);
        let encoded = encode_to_vec(&chunk).expect("encode VideoChunk for all-codec lz4 test");
        let compressed = compress(&encoded, Compression::Lz4).expect("lz4 compress all-codec test");
        let decompressed = decompress(&compressed).expect("lz4 decompress all-codec test");
        let (decoded, _): (VideoChunk, usize) =
            decode_from_slice(&decompressed).expect("decode all-codec lz4 test");
        assert_eq!(
            chunk, decoded,
            "VideoCodec variant {idx} failed lz4 roundtrip"
        );
    }
}

// ── Test 15: All VideoCodec variants round-trip via Zstd ─────────────────────
#[test]
fn test_all_video_codec_variants_zstd_roundtrip() {
    let codecs = [
        VideoCodec::H264,
        VideoCodec::H265,
        VideoCodec::Vp8,
        VideoCodec::Vp9,
        VideoCodec::Av1,
    ];
    for (idx, codec) in codecs.into_iter().enumerate() {
        let chunk = make_video_chunk(idx as u32 + 50, codec, 64, idx % 2 != 0);
        let encoded = encode_to_vec(&chunk).expect("encode VideoChunk for all-codec zstd test");
        let compressed =
            compress(&encoded, Compression::Zstd).expect("zstd compress all-codec test");
        let decompressed = decompress(&compressed).expect("zstd decompress all-codec test");
        let (decoded, _): (VideoChunk, usize) =
            decode_from_slice(&decompressed).expect("decode all-codec zstd test");
        assert_eq!(
            chunk, decoded,
            "VideoCodec variant {idx} failed zstd roundtrip"
        );
    }
}

// ── Test 16: Vec<VideoChunk> LZ4 roundtrip ───────────────────────────────────
#[test]
fn test_vec_video_chunk_lz4_roundtrip() {
    let chunks: Vec<VideoChunk> = (0u32..12)
        .map(|i| make_video_chunk(i, VideoCodec::H264, 64 + i as usize * 8, i % 3 == 0))
        .collect();
    let encoded = encode_to_vec(&chunks).expect("encode Vec<VideoChunk>");
    let compressed = compress(&encoded, Compression::Lz4).expect("lz4 compress Vec<VideoChunk>");
    let decompressed = decompress(&compressed).expect("lz4 decompress Vec<VideoChunk>");
    let (decoded, _): (Vec<VideoChunk>, usize) =
        decode_from_slice(&decompressed).expect("decode Vec<VideoChunk> lz4");
    assert_eq!(chunks, decoded);
}

// ── Test 17: Vec<VideoChunk> Zstd roundtrip ──────────────────────────────────
#[test]
fn test_vec_video_chunk_zstd_roundtrip() {
    let chunks: Vec<VideoChunk> = (0u32..12)
        .map(|i| make_video_chunk(i + 200, VideoCodec::Vp9, 64 + i as usize * 8, i % 4 == 0))
        .collect();
    let encoded = encode_to_vec(&chunks).expect("encode Vec<VideoChunk> zstd");
    let compressed = compress(&encoded, Compression::Zstd).expect("zstd compress Vec<VideoChunk>");
    let decompressed = decompress(&compressed).expect("zstd decompress Vec<VideoChunk>");
    let (decoded, _): (Vec<VideoChunk>, usize) =
        decode_from_slice(&decompressed).expect("decode Vec<VideoChunk> zstd");
    assert_eq!(chunks, decoded);
}

// ── Test 18: decode_from_slice consumed-bytes check after LZ4 ────────────────
#[test]
fn test_consumed_bytes_lz4_video_chunk() {
    let chunk = make_video_chunk(40, VideoCodec::H265, 96, true);
    let encoded = encode_to_vec(&chunk).expect("encode VideoChunk for consumed-bytes lz4");
    let compressed = compress(&encoded, Compression::Lz4).expect("lz4 compress for consumed-bytes");
    let decompressed = decompress(&compressed).expect("lz4 decompress for consumed-bytes");
    let (decoded, consumed): (VideoChunk, usize) =
        decode_from_slice(&decompressed).expect("decode VideoChunk consumed-bytes lz4");
    assert_eq!(chunk, decoded);
    assert!(
        consumed > 0,
        "consumed bytes must be positive after LZ4 decode"
    );
    assert!(
        consumed <= decompressed.len(),
        "consumed ({consumed}) must not exceed decompressed length ({})",
        decompressed.len()
    );
}

// ── Test 19: decode_from_slice consumed-bytes check after Zstd ───────────────
#[test]
fn test_consumed_bytes_zstd_video_chunk() {
    let chunk = make_video_chunk(41, VideoCodec::Vp8, 96, false);
    let encoded = encode_to_vec(&chunk).expect("encode VideoChunk for consumed-bytes zstd");
    let compressed =
        compress(&encoded, Compression::Zstd).expect("zstd compress for consumed-bytes");
    let decompressed = decompress(&compressed).expect("zstd decompress for consumed-bytes");
    let (decoded, consumed): (VideoChunk, usize) =
        decode_from_slice(&decompressed).expect("decode VideoChunk consumed-bytes zstd");
    assert_eq!(chunk, decoded);
    assert!(
        consumed > 0,
        "consumed bytes must be positive after Zstd decode"
    );
    assert!(
        consumed <= decompressed.len(),
        "consumed ({consumed}) must not exceed decompressed length ({})",
        decompressed.len()
    );
}

// ── Test 20: StreamManifest H264 + VideoChunk pipeline LZ4 ───────────────────
#[test]
fn test_stream_manifest_and_chunk_pipeline_lz4() {
    let manifest = make_manifest(42, VideoCodec::H264, 6);
    let chunks: Vec<VideoChunk> = (0u32..6)
        .map(|i| make_video_chunk(i, VideoCodec::H264, 128, i == 0))
        .collect();

    let enc_manifest = encode_to_vec(&manifest).expect("encode manifest pipeline lz4");
    let enc_chunks = encode_to_vec(&chunks).expect("encode chunks pipeline lz4");

    let comp_manifest =
        compress(&enc_manifest, Compression::Lz4).expect("lz4 compress manifest pipeline");
    let comp_chunks =
        compress(&enc_chunks, Compression::Lz4).expect("lz4 compress chunks pipeline");

    let decomp_manifest = decompress(&comp_manifest).expect("lz4 decompress manifest pipeline");
    let decomp_chunks = decompress(&comp_chunks).expect("lz4 decompress chunks pipeline");

    let (dec_manifest, _): (StreamManifest, usize) =
        decode_from_slice(&decomp_manifest).expect("decode manifest pipeline lz4");
    let (dec_chunks, _): (Vec<VideoChunk>, usize) =
        decode_from_slice(&decomp_chunks).expect("decode chunks pipeline lz4");

    assert_eq!(manifest, dec_manifest);
    assert_eq!(chunks, dec_chunks);
    assert_eq!(dec_manifest.chunk_count as usize, dec_chunks.len());
}

// ── Test 21: StreamManifest Av1 + VideoChunk pipeline Zstd ───────────────────
#[test]
fn test_stream_manifest_and_chunk_pipeline_zstd() {
    let manifest = make_manifest(99, VideoCodec::Av1, 4);
    let chunks: Vec<VideoChunk> = (0u32..4)
        .map(|i| make_video_chunk(i, VideoCodec::Av1, 192, i == 0))
        .collect();

    let enc_manifest = encode_to_vec(&manifest).expect("encode manifest pipeline zstd");
    let enc_chunks = encode_to_vec(&chunks).expect("encode chunks pipeline zstd");

    let comp_manifest =
        compress(&enc_manifest, Compression::Zstd).expect("zstd compress manifest pipeline");
    let comp_chunks =
        compress(&enc_chunks, Compression::Zstd).expect("zstd compress chunks pipeline");

    let decomp_manifest = decompress(&comp_manifest).expect("zstd decompress manifest pipeline");
    let decomp_chunks = decompress(&comp_chunks).expect("zstd decompress chunks pipeline");

    let (dec_manifest, _): (StreamManifest, usize) =
        decode_from_slice(&decomp_manifest).expect("decode manifest pipeline zstd");
    let (dec_chunks, _): (Vec<VideoChunk>, usize) =
        decode_from_slice(&decomp_chunks).expect("decode chunks pipeline zstd");

    assert_eq!(manifest, dec_manifest);
    assert_eq!(chunks, dec_chunks);
    assert_eq!(dec_manifest.chunk_count as usize, dec_chunks.len());
}

// ── Test 22: LZ4 → decompress → Zstd re-compress → decompress: bytes match ───
#[test]
fn test_lz4_decompress_then_zstd_recompress_video_chunk() {
    let chunk = make_video_chunk(50, VideoCodec::H265, 512, true);
    let original_encoded = encode_to_vec(&chunk).expect("encode VideoChunk for re-compress test");
    let lz4_compressed =
        compress(&original_encoded, Compression::Lz4).expect("initial lz4 compress");
    let lz4_decompressed =
        decompress(&lz4_compressed).expect("lz4 decompress step for re-compress test");
    let zstd_recompressed =
        compress(&lz4_decompressed, Compression::Zstd).expect("zstd re-compress step");
    let final_decompressed =
        decompress(&zstd_recompressed).expect("final zstd decompress after re-compress");
    assert_eq!(
        original_encoded, final_decompressed,
        "After LZ4 decompress → Zstd re-compress → decompress, bytes must match original"
    );
    let (decoded, _): (VideoChunk, usize) =
        decode_from_slice(&final_decompressed).expect("decode VideoChunk after re-compress");
    assert_eq!(chunk, decoded);
}
