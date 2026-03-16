#![cfg(feature = "compression-lz4")]
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

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
enum VideoCodec {
    H264,
    H265,
    VP8,
    VP9,
    AV1,
    MPEG4,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
enum AudioCodec {
    AAC,
    MP3,
    Opus,
    Vorbis,
    FLAC,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
enum StreamQuality {
    Low,
    Medium,
    High,
    Ultra,
    Adaptive,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
enum FrameType {
    Keyframe,
    PFrame,
    BFrame,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct VideoFrame {
    frame_id: u64,
    frame_type: FrameType,
    width: u16,
    height: u16,
    timestamp_ms: u64,
    data: Vec<u8>,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct StreamSegment {
    segment_id: u64,
    stream_id: u32,
    start_ms: u64,
    duration_ms: u32,
    keyframe_ids: Vec<u64>,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct MediaMetadata {
    stream_id: u32,
    title: String,
    codec: VideoCodec,
    audio_codec: AudioCodec,
    quality: StreamQuality,
    bitrate_kbps: u32,
    fps_x100: u32,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct BufferStats {
    client_id: u64,
    buffer_level_ms: u32,
    rebuffer_count: u16,
    avg_bitrate_kbps: u32,
    total_bytes: u64,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct Subtitle {
    stream_id: u32,
    start_ms: u64,
    end_ms: u64,
    text: String,
    language: String,
}

// Test 1: Basic VideoFrame roundtrip with H264 keyframe
#[test]
fn test_video_frame_h264_keyframe_roundtrip() {
    let frame = VideoFrame {
        frame_id: 1,
        frame_type: FrameType::Keyframe,
        width: 1920,
        height: 1080,
        timestamp_ms: 0,
        data: vec![0xAB, 0xCD, 0xEF, 0x12, 0x34],
    };
    let encoded = encode_to_vec(&frame).expect("encode VideoFrame H264 keyframe");
    let compressed =
        compress(&encoded, Compression::Lz4).expect("compress VideoFrame H264 keyframe");
    let decompressed = decompress(&compressed).expect("decompress VideoFrame H264 keyframe");
    let (decoded, _): (VideoFrame, usize) =
        decode_from_slice(&decompressed).expect("decode VideoFrame H264 keyframe");
    assert_eq!(frame, decoded);
}

// Test 2: VideoFrame with H265 PFrame roundtrip
#[test]
fn test_video_frame_h265_pframe_roundtrip() {
    let frame = VideoFrame {
        frame_id: 42,
        frame_type: FrameType::PFrame,
        width: 3840,
        height: 2160,
        timestamp_ms: 1666,
        data: vec![0x10, 0x20, 0x30, 0x40, 0x50, 0x60, 0x70, 0x80],
    };
    let encoded = encode_to_vec(&frame).expect("encode VideoFrame H265 PFrame");
    let compressed = compress(&encoded, Compression::Lz4).expect("compress VideoFrame H265 PFrame");
    let decompressed = decompress(&compressed).expect("decompress VideoFrame H265 PFrame");
    let (decoded, _): (VideoFrame, usize) =
        decode_from_slice(&decompressed).expect("decode VideoFrame H265 PFrame");
    assert_eq!(frame, decoded);
}

// Test 3: VideoFrame with BFrame roundtrip
#[test]
fn test_video_frame_bframe_roundtrip() {
    let frame = VideoFrame {
        frame_id: 7,
        frame_type: FrameType::BFrame,
        width: 1280,
        height: 720,
        timestamp_ms: 233,
        data: vec![0xFF; 64],
    };
    let encoded = encode_to_vec(&frame).expect("encode VideoFrame BFrame");
    let compressed = compress(&encoded, Compression::Lz4).expect("compress VideoFrame BFrame");
    let decompressed = decompress(&compressed).expect("decompress VideoFrame BFrame");
    let (decoded, _): (VideoFrame, usize) =
        decode_from_slice(&decompressed).expect("decode VideoFrame BFrame");
    assert_eq!(frame, decoded);
}

// Test 4: StreamSegment with large keyframe list roundtrip
#[test]
fn test_stream_segment_large_keyframe_list_roundtrip() {
    let keyframe_ids: Vec<u64> = (0u64..500).map(|i| i * 30).collect();
    let segment = StreamSegment {
        segment_id: 100,
        stream_id: 7,
        start_ms: 30000,
        duration_ms: 15000,
        keyframe_ids,
    };
    let encoded = encode_to_vec(&segment).expect("encode StreamSegment large keyframe list");
    let compressed =
        compress(&encoded, Compression::Lz4).expect("compress StreamSegment large keyframe list");
    let decompressed =
        decompress(&compressed).expect("decompress StreamSegment large keyframe list");
    let (decoded, _): (StreamSegment, usize) =
        decode_from_slice(&decompressed).expect("decode StreamSegment large keyframe list");
    assert_eq!(segment, decoded);
}

// Test 5: MediaMetadata AV1 / Opus / Ultra roundtrip
#[test]
fn test_media_metadata_av1_opus_ultra_roundtrip() {
    let meta = MediaMetadata {
        stream_id: 999,
        title: String::from("Live Concert Stream 4K HDR"),
        codec: VideoCodec::AV1,
        audio_codec: AudioCodec::Opus,
        quality: StreamQuality::Ultra,
        bitrate_kbps: 25000,
        fps_x100: 6000,
    };
    let encoded = encode_to_vec(&meta).expect("encode MediaMetadata AV1 Opus Ultra");
    let compressed =
        compress(&encoded, Compression::Lz4).expect("compress MediaMetadata AV1 Opus Ultra");
    let decompressed = decompress(&compressed).expect("decompress MediaMetadata AV1 Opus Ultra");
    let (decoded, _): (MediaMetadata, usize) =
        decode_from_slice(&decompressed).expect("decode MediaMetadata AV1 Opus Ultra");
    assert_eq!(meta, decoded);
}

// Test 6: BufferStats roundtrip
#[test]
fn test_buffer_stats_roundtrip() {
    let stats = BufferStats {
        client_id: 0xDEADBEEFCAFEBABE,
        buffer_level_ms: 8000,
        rebuffer_count: 3,
        avg_bitrate_kbps: 4500,
        total_bytes: 1_073_741_824,
    };
    let encoded = encode_to_vec(&stats).expect("encode BufferStats");
    let compressed = compress(&encoded, Compression::Lz4).expect("compress BufferStats");
    let decompressed = decompress(&compressed).expect("decompress BufferStats");
    let (decoded, _): (BufferStats, usize) =
        decode_from_slice(&decompressed).expect("decode BufferStats");
    assert_eq!(stats, decoded);
}

// Test 7: Subtitle roundtrip
#[test]
fn test_subtitle_roundtrip() {
    let sub = Subtitle {
        stream_id: 5,
        start_ms: 12500,
        end_ms: 15200,
        text: String::from("Welcome to the live stream!"),
        language: String::from("en"),
    };
    let encoded = encode_to_vec(&sub).expect("encode Subtitle");
    let compressed = compress(&encoded, Compression::Lz4).expect("compress Subtitle");
    let decompressed = decompress(&compressed).expect("decompress Subtitle");
    let (decoded, _): (Subtitle, usize) =
        decode_from_slice(&decompressed).expect("decode Subtitle");
    assert_eq!(sub, decoded);
}

// Test 8: Large video frame data compression ratio (1000+ repetitive bytes)
#[test]
fn test_large_video_frame_compression_ratio() {
    let repetitive_data: Vec<u8> = (0u32..1500)
        .flat_map(|_| [0x00u8, 0x00, 0x00, 0x01].iter().cloned())
        .collect();
    let frame = VideoFrame {
        frame_id: 9999,
        frame_type: FrameType::Keyframe,
        width: 1920,
        height: 1080,
        timestamp_ms: 333_333,
        data: repetitive_data,
    };
    let encoded = encode_to_vec(&frame).expect("encode large VideoFrame for ratio test");
    let compressed =
        compress(&encoded, Compression::Lz4).expect("compress large VideoFrame for ratio test");
    assert!(
        compressed.len() < encoded.len(),
        "LZ4 compression should reduce size of repetitive video frame data: encoded={} compressed={}",
        encoded.len(),
        compressed.len()
    );
    let decompressed = decompress(&compressed).expect("decompress large VideoFrame for ratio test");
    let (decoded, _): (VideoFrame, usize) =
        decode_from_slice(&decompressed).expect("decode large VideoFrame for ratio test");
    assert_eq!(frame, decoded);
}

// Test 9: Large segment list compression (1000+ entries)
#[test]
fn test_large_segment_list_compression() {
    let segments: Vec<StreamSegment> = (0u64..1200)
        .map(|i| StreamSegment {
            segment_id: i,
            stream_id: 1,
            start_ms: i * 2000,
            duration_ms: 2000,
            keyframe_ids: vec![i * 60, i * 60 + 30],
        })
        .collect();
    let encoded = encode_to_vec(&segments).expect("encode large segment list");
    let compressed = compress(&encoded, Compression::Lz4).expect("compress large segment list");
    assert!(
        compressed.len() < encoded.len(),
        "LZ4 compression should reduce size of large segment list: encoded={} compressed={}",
        encoded.len(),
        compressed.len()
    );
    let decompressed = decompress(&compressed).expect("decompress large segment list");
    let (decoded, _): (Vec<StreamSegment>, usize) =
        decode_from_slice(&decompressed).expect("decode large segment list");
    assert_eq!(segments, decoded);
}

// Test 10: Subtitle list compression
#[test]
fn test_subtitle_list_compression_roundtrip() {
    let subtitles: Vec<Subtitle> = (0u64..200)
        .map(|i| Subtitle {
            stream_id: 3,
            start_ms: i * 3000,
            end_ms: i * 3000 + 2500,
            text: format!("Subtitle line number {}: streaming in progress", i),
            language: String::from("en"),
        })
        .collect();
    let encoded = encode_to_vec(&subtitles).expect("encode subtitle list");
    let compressed = compress(&encoded, Compression::Lz4).expect("compress subtitle list");
    let decompressed = decompress(&compressed).expect("decompress subtitle list");
    let (decoded, _): (Vec<Subtitle>, usize) =
        decode_from_slice(&decompressed).expect("decode subtitle list");
    assert_eq!(subtitles, decoded);
}

// Test 11: Compressed bytes differ from original encoded bytes
#[test]
fn test_compressed_bytes_differ_from_encoded() {
    let frame = VideoFrame {
        frame_id: 55,
        frame_type: FrameType::PFrame,
        width: 640,
        height: 480,
        timestamp_ms: 7777,
        data: vec![0xAA; 256],
    };
    let encoded = encode_to_vec(&frame).expect("encode VideoFrame for diff check");
    let compressed =
        compress(&encoded, Compression::Lz4).expect("compress VideoFrame for diff check");
    assert_ne!(
        encoded, compressed,
        "Compressed bytes must differ from original encoded bytes"
    );
}

// Test 12: Decompressed length equals original encoded length
#[test]
fn test_decompressed_length_equals_encoded_length() {
    let meta = MediaMetadata {
        stream_id: 22,
        title: String::from("Sports Highlights HD"),
        codec: VideoCodec::VP9,
        audio_codec: AudioCodec::AAC,
        quality: StreamQuality::High,
        bitrate_kbps: 8000,
        fps_x100: 5994,
    };
    let encoded = encode_to_vec(&meta).expect("encode MediaMetadata for length check");
    let compressed =
        compress(&encoded, Compression::Lz4).expect("compress MediaMetadata for length check");
    let decompressed = decompress(&compressed).expect("decompress MediaMetadata for length check");
    assert_eq!(
        encoded.len(),
        decompressed.len(),
        "Decompressed length must match original encoded length"
    );
}

// Test 13: Multiple compress/decompress cycles preserve data integrity
#[test]
fn test_multiple_compress_decompress_cycles() {
    let stats = BufferStats {
        client_id: 12345678,
        buffer_level_ms: 5000,
        rebuffer_count: 1,
        avg_bitrate_kbps: 3200,
        total_bytes: 500_000_000,
    };
    let encoded = encode_to_vec(&stats).expect("encode BufferStats for multi-cycle");

    let mut current = encoded.clone();
    for cycle in 0..5 {
        let compressed = compress(&current, Compression::Lz4)
            .unwrap_or_else(|e| panic!("compress cycle {}: {}", cycle, e));
        let decompressed =
            decompress(&compressed).unwrap_or_else(|e| panic!("decompress cycle {}: {}", cycle, e));
        current = decompressed;
    }

    assert_eq!(
        encoded, current,
        "Data must be identical after multiple compress/decompress cycles"
    );
    let (decoded, _): (BufferStats, usize) =
        decode_from_slice(&current).expect("decode BufferStats after multi-cycle");
    assert_eq!(stats, decoded);
}

// Test 14: All VideoCodec variants roundtrip
#[test]
fn test_all_video_codec_variants_roundtrip() {
    let codecs = vec![
        VideoCodec::H264,
        VideoCodec::H265,
        VideoCodec::VP8,
        VideoCodec::VP9,
        VideoCodec::AV1,
        VideoCodec::MPEG4,
    ];
    for codec in codecs {
        let meta = MediaMetadata {
            stream_id: 1,
            title: format!("Test stream {:?}", codec),
            codec: codec.clone(),
            audio_codec: AudioCodec::AAC,
            quality: StreamQuality::Medium,
            bitrate_kbps: 4000,
            fps_x100: 2997,
        };
        let encoded = encode_to_vec(&meta)
            .unwrap_or_else(|e| panic!("encode MediaMetadata {:?}: {}", codec, e));
        let compressed = compress(&encoded, Compression::Lz4)
            .unwrap_or_else(|e| panic!("compress MediaMetadata {:?}: {}", codec, e));
        let decompressed = decompress(&compressed)
            .unwrap_or_else(|e| panic!("decompress MediaMetadata {:?}: {}", codec, e));
        let (decoded, _): (MediaMetadata, usize) = decode_from_slice(&decompressed)
            .unwrap_or_else(|e| panic!("decode MediaMetadata {:?}: {}", codec, e));
        assert_eq!(meta, decoded);
    }
}

// Test 15: All AudioCodec variants roundtrip
#[test]
fn test_all_audio_codec_variants_roundtrip() {
    let audio_codecs = vec![
        AudioCodec::AAC,
        AudioCodec::MP3,
        AudioCodec::Opus,
        AudioCodec::Vorbis,
        AudioCodec::FLAC,
    ];
    for audio_codec in audio_codecs {
        let meta = MediaMetadata {
            stream_id: 2,
            title: format!("Audio test {:?}", audio_codec),
            codec: VideoCodec::H264,
            audio_codec: audio_codec.clone(),
            quality: StreamQuality::High,
            bitrate_kbps: 320,
            fps_x100: 2500,
        };
        let encoded = encode_to_vec(&meta)
            .unwrap_or_else(|e| panic!("encode MediaMetadata audio {:?}: {}", audio_codec, e));
        let compressed = compress(&encoded, Compression::Lz4)
            .unwrap_or_else(|e| panic!("compress MediaMetadata audio {:?}: {}", audio_codec, e));
        let decompressed = decompress(&compressed)
            .unwrap_or_else(|e| panic!("decompress MediaMetadata audio {:?}: {}", audio_codec, e));
        let (decoded, _): (MediaMetadata, usize) = decode_from_slice(&decompressed)
            .unwrap_or_else(|e| panic!("decode MediaMetadata audio {:?}: {}", audio_codec, e));
        assert_eq!(meta, decoded);
    }
}

// Test 16: All StreamQuality variants roundtrip
#[test]
fn test_all_stream_quality_variants_roundtrip() {
    let qualities = vec![
        StreamQuality::Low,
        StreamQuality::Medium,
        StreamQuality::High,
        StreamQuality::Ultra,
        StreamQuality::Adaptive,
    ];
    for quality in qualities {
        let meta = MediaMetadata {
            stream_id: 3,
            title: format!("Quality test {:?}", quality),
            codec: VideoCodec::VP9,
            audio_codec: AudioCodec::Opus,
            quality: quality.clone(),
            bitrate_kbps: 2000,
            fps_x100: 3000,
        };
        let encoded = encode_to_vec(&meta)
            .unwrap_or_else(|e| panic!("encode MediaMetadata quality {:?}: {}", quality, e));
        let compressed = compress(&encoded, Compression::Lz4)
            .unwrap_or_else(|e| panic!("compress MediaMetadata quality {:?}: {}", quality, e));
        let decompressed = decompress(&compressed)
            .unwrap_or_else(|e| panic!("decompress MediaMetadata quality {:?}: {}", quality, e));
        let (decoded, _): (MediaMetadata, usize) = decode_from_slice(&decompressed)
            .unwrap_or_else(|e| panic!("decode MediaMetadata quality {:?}: {}", quality, e));
        assert_eq!(meta, decoded);
    }
}

// Test 17: VideoFrame with empty data roundtrip
#[test]
fn test_video_frame_empty_data_roundtrip() {
    let frame = VideoFrame {
        frame_id: 0,
        frame_type: FrameType::BFrame,
        width: 320,
        height: 240,
        timestamp_ms: 100,
        data: vec![],
    };
    let encoded = encode_to_vec(&frame).expect("encode VideoFrame empty data");
    let compressed = compress(&encoded, Compression::Lz4).expect("compress VideoFrame empty data");
    let decompressed = decompress(&compressed).expect("decompress VideoFrame empty data");
    let (decoded, _): (VideoFrame, usize) =
        decode_from_slice(&decompressed).expect("decode VideoFrame empty data");
    assert_eq!(frame, decoded);
}

// Test 18: StreamSegment with empty keyframe list roundtrip
#[test]
fn test_stream_segment_empty_keyframes_roundtrip() {
    let segment = StreamSegment {
        segment_id: 0,
        stream_id: 0,
        start_ms: 0,
        duration_ms: 0,
        keyframe_ids: vec![],
    };
    let encoded = encode_to_vec(&segment).expect("encode StreamSegment empty keyframes");
    let compressed =
        compress(&encoded, Compression::Lz4).expect("compress StreamSegment empty keyframes");
    let decompressed = decompress(&compressed).expect("decompress StreamSegment empty keyframes");
    let (decoded, _): (StreamSegment, usize) =
        decode_from_slice(&decompressed).expect("decode StreamSegment empty keyframes");
    assert_eq!(segment, decoded);
}

// Test 19: Large repetitive subtitle data compression ratio
#[test]
fn test_large_subtitle_list_compression_ratio() {
    let subtitles: Vec<Subtitle> = (0u64..1000)
        .map(|i| Subtitle {
            stream_id: 10,
            start_ms: i * 2000,
            end_ms: i * 2000 + 1800,
            text: String::from(
                "This is a repeated subtitle line for compression testing purposes.",
            ),
            language: String::from("en"),
        })
        .collect();
    let encoded = encode_to_vec(&subtitles).expect("encode large repetitive subtitle list");
    let compressed =
        compress(&encoded, Compression::Lz4).expect("compress large repetitive subtitle list");
    assert!(
        compressed.len() < encoded.len(),
        "LZ4 should compress repetitive subtitle data: encoded={} compressed={}",
        encoded.len(),
        compressed.len()
    );
    let decompressed = decompress(&compressed).expect("decompress large repetitive subtitle list");
    let (decoded, _): (Vec<Subtitle>, usize) =
        decode_from_slice(&decompressed).expect("decode large repetitive subtitle list");
    assert_eq!(subtitles, decoded);
}

// Test 20: Adaptive bitrate metadata list compression (1000+ entries)
#[test]
fn test_adaptive_bitrate_metadata_list_compression() {
    let meta_list: Vec<MediaMetadata> = (0u32..1000)
        .map(|i| MediaMetadata {
            stream_id: i,
            title: format!("Channel {}: Live Broadcast", i),
            codec: VideoCodec::H264,
            audio_codec: AudioCodec::AAC,
            quality: StreamQuality::Adaptive,
            bitrate_kbps: 2000 + i * 100,
            fps_x100: 3000,
        })
        .collect();
    let encoded = encode_to_vec(&meta_list).expect("encode adaptive bitrate metadata list");
    let compressed =
        compress(&encoded, Compression::Lz4).expect("compress adaptive bitrate metadata list");
    assert!(
        compressed.len() < encoded.len(),
        "LZ4 should compress repetitive metadata list: encoded={} compressed={}",
        encoded.len(),
        compressed.len()
    );
    let decompressed = decompress(&compressed).expect("decompress adaptive bitrate metadata list");
    let (decoded, _): (Vec<MediaMetadata>, usize) =
        decode_from_slice(&decompressed).expect("decode adaptive bitrate metadata list");
    assert_eq!(meta_list, decoded);
}

// Test 21: Mixed FrameType sequence compression roundtrip
#[test]
fn test_mixed_frame_type_sequence_compression_roundtrip() {
    let frame_types = [
        FrameType::Keyframe,
        FrameType::PFrame,
        FrameType::BFrame,
        FrameType::PFrame,
        FrameType::BFrame,
    ];
    let frames: Vec<VideoFrame> = (0u64..300)
        .map(|i| VideoFrame {
            frame_id: i,
            frame_type: frame_types[(i as usize) % frame_types.len()].clone(),
            width: 1920,
            height: 1080,
            timestamp_ms: i * 33,
            data: vec![(i % 256) as u8; 32],
        })
        .collect();
    let encoded = encode_to_vec(&frames).expect("encode mixed frame type sequence");
    let compressed =
        compress(&encoded, Compression::Lz4).expect("compress mixed frame type sequence");
    let decompressed = decompress(&compressed).expect("decompress mixed frame type sequence");
    let (decoded, _): (Vec<VideoFrame>, usize) =
        decode_from_slice(&decompressed).expect("decode mixed frame type sequence");
    assert_eq!(frames, decoded);
}

// Test 22: BufferStats list for multi-client tracking compression roundtrip
#[test]
fn test_buffer_stats_multi_client_compression_roundtrip() {
    let client_stats: Vec<BufferStats> = (0u64..1000)
        .map(|i| BufferStats {
            client_id: 0x1000_0000_0000_0000 + i,
            buffer_level_ms: 3000 + (i % 10000) as u32,
            rebuffer_count: (i % 20) as u16,
            avg_bitrate_kbps: 1500 + (i % 500) as u32,
            total_bytes: i * 1_000_000,
        })
        .collect();
    let encoded = encode_to_vec(&client_stats).expect("encode multi-client BufferStats list");
    let compressed =
        compress(&encoded, Compression::Lz4).expect("compress multi-client BufferStats list");
    assert!(
        compressed.len() < encoded.len(),
        "LZ4 should compress repetitive client stats: encoded={} compressed={}",
        encoded.len(),
        compressed.len()
    );
    let decompressed = decompress(&compressed).expect("decompress multi-client BufferStats list");
    assert_eq!(
        encoded.len(),
        decompressed.len(),
        "Decompressed multi-client stats length must match encoded length"
    );
    let (decoded, _): (Vec<BufferStats>, usize) =
        decode_from_slice(&decompressed).expect("decode multi-client BufferStats list");
    assert_eq!(client_stats, decoded);
}
