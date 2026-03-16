//! Advanced file I/O tests with audio processing / digital signal processing domain theme.
//!
//! Tests encode_to_file / decode_from_file for DSP types including sample formats,
//! channel layouts, audio configs, frames, and clips.

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
use oxicode::{decode_from_file, decode_from_slice, encode_to_file, encode_to_vec, Decode, Encode};

// ---------------------------------------------------------------------------
// Domain types
// ---------------------------------------------------------------------------

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
enum SampleFormat {
    PCM8,
    PCM16,
    PCM24,
    PCM32,
    Float32,
    Float64,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
enum ChannelLayout {
    Mono,
    Stereo,
    Surround51,
    Surround71,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct AudioConfig {
    sample_rate: u32,
    format: SampleFormat,
    channels: ChannelLayout,
    bit_depth: u8,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct AudioFrame {
    samples: Vec<f32>,
    frame_number: u64,
    timestamp_ms: f64,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct AudioClip {
    config: AudioConfig,
    name: String,
    frames: Vec<AudioFrame>,
    duration_ms: f64,
}

// ---------------------------------------------------------------------------
// Helpers
// ---------------------------------------------------------------------------

fn default_config() -> AudioConfig {
    AudioConfig {
        sample_rate: 44100,
        format: SampleFormat::PCM16,
        channels: ChannelLayout::Stereo,
        bit_depth: 16,
    }
}

fn stereo_frame(frame_number: u64, sample_count: usize) -> AudioFrame {
    let samples: Vec<f32> = (0..sample_count)
        .map(|i| (i as f32 / sample_count as f32) * 2.0 - 1.0)
        .collect();
    AudioFrame {
        samples,
        frame_number,
        timestamp_ms: frame_number as f64 * (1000.0 / 44100.0),
    }
}

// ---------------------------------------------------------------------------
// Tests (22 total)
// ---------------------------------------------------------------------------

#[test]
fn test_audio_config_pcm16_stereo_roundtrip() {
    let cfg = default_config();
    let path = std::env::temp_dir().join("oxi_dsp16_cfg_pcm16_stereo.bin");
    encode_to_file(&cfg, &path).expect("encode AudioConfig PCM16 stereo");
    let decoded: AudioConfig = decode_from_file(&path).expect("decode AudioConfig PCM16 stereo");
    assert_eq!(cfg, decoded);
    std::fs::remove_file(&path).expect("cleanup AudioConfig PCM16 stereo");
}

#[test]
fn test_audio_config_float32_mono_roundtrip() {
    let cfg = AudioConfig {
        sample_rate: 48000,
        format: SampleFormat::Float32,
        channels: ChannelLayout::Mono,
        bit_depth: 32,
    };
    let path = std::env::temp_dir().join("oxi_dsp16_cfg_float32_mono.bin");
    encode_to_file(&cfg, &path).expect("encode AudioConfig Float32 mono");
    let decoded: AudioConfig = decode_from_file(&path).expect("decode AudioConfig Float32 mono");
    assert_eq!(cfg, decoded);
    std::fs::remove_file(&path).expect("cleanup AudioConfig Float32 mono");
}

#[test]
fn test_audio_config_float64_surround51_roundtrip() {
    let cfg = AudioConfig {
        sample_rate: 96000,
        format: SampleFormat::Float64,
        channels: ChannelLayout::Surround51,
        bit_depth: 64,
    };
    let path = std::env::temp_dir().join("oxi_dsp16_cfg_float64_51.bin");
    encode_to_file(&cfg, &path).expect("encode AudioConfig Float64 5.1");
    let decoded: AudioConfig = decode_from_file(&path).expect("decode AudioConfig Float64 5.1");
    assert_eq!(cfg, decoded);
    std::fs::remove_file(&path).expect("cleanup AudioConfig Float64 5.1");
}

#[test]
fn test_audio_config_pcm24_surround71_roundtrip() {
    let cfg = AudioConfig {
        sample_rate: 192000,
        format: SampleFormat::PCM24,
        channels: ChannelLayout::Surround71,
        bit_depth: 24,
    };
    let path = std::env::temp_dir().join("oxi_dsp16_cfg_pcm24_71.bin");
    encode_to_file(&cfg, &path).expect("encode AudioConfig PCM24 7.1");
    let decoded: AudioConfig = decode_from_file(&path).expect("decode AudioConfig PCM24 7.1");
    assert_eq!(cfg, decoded);
    std::fs::remove_file(&path).expect("cleanup AudioConfig PCM24 7.1");
}

#[test]
fn test_sample_format_pcm8_roundtrip() {
    let fmt = SampleFormat::PCM8;
    let path = std::env::temp_dir().join("oxi_dsp16_sample_fmt_pcm8.bin");
    encode_to_file(&fmt, &path).expect("encode SampleFormat::PCM8");
    let decoded: SampleFormat = decode_from_file(&path).expect("decode SampleFormat::PCM8");
    assert_eq!(fmt, decoded);
    std::fs::remove_file(&path).expect("cleanup SampleFormat::PCM8");
}

#[test]
fn test_sample_format_pcm32_roundtrip() {
    let fmt = SampleFormat::PCM32;
    let path = std::env::temp_dir().join("oxi_dsp16_sample_fmt_pcm32.bin");
    encode_to_file(&fmt, &path).expect("encode SampleFormat::PCM32");
    let decoded: SampleFormat = decode_from_file(&path).expect("decode SampleFormat::PCM32");
    assert_eq!(fmt, decoded);
    std::fs::remove_file(&path).expect("cleanup SampleFormat::PCM32");
}

#[test]
fn test_channel_layout_surround71_roundtrip() {
    let layout = ChannelLayout::Surround71;
    let path = std::env::temp_dir().join("oxi_dsp16_layout_71.bin");
    encode_to_file(&layout, &path).expect("encode ChannelLayout::Surround71");
    let decoded: ChannelLayout = decode_from_file(&path).expect("decode ChannelLayout::Surround71");
    assert_eq!(layout, decoded);
    std::fs::remove_file(&path).expect("cleanup ChannelLayout::Surround71");
}

#[test]
fn test_audio_frame_small_roundtrip() {
    let frame = AudioFrame {
        samples: vec![0.0_f32, 0.25, 0.5, 0.75, 1.0],
        frame_number: 0,
        timestamp_ms: 0.0,
    };
    let path = std::env::temp_dir().join("oxi_dsp16_frame_small.bin");
    encode_to_file(&frame, &path).expect("encode AudioFrame small");
    let decoded: AudioFrame = decode_from_file(&path).expect("decode AudioFrame small");
    assert_eq!(frame, decoded);
    std::fs::remove_file(&path).expect("cleanup AudioFrame small");
}

#[test]
fn test_audio_frame_large_500_samples_roundtrip() {
    let frame = stereo_frame(42, 500);
    let path = std::env::temp_dir().join("oxi_dsp16_frame_500samples.bin");
    encode_to_file(&frame, &path).expect("encode AudioFrame 500 samples");
    let decoded: AudioFrame = decode_from_file(&path).expect("decode AudioFrame 500 samples");
    assert_eq!(frame.frame_number, decoded.frame_number);
    assert_eq!(frame.samples.len(), decoded.samples.len());
    for (a, b) in frame.samples.iter().zip(decoded.samples.iter()) {
        assert_eq!(a.to_bits(), b.to_bits());
    }
    std::fs::remove_file(&path).expect("cleanup AudioFrame 500 samples");
}

#[test]
fn test_audio_frame_1024_samples_roundtrip() {
    let frame = stereo_frame(7, 1024);
    let path = std::env::temp_dir().join("oxi_dsp16_frame_1024samples.bin");
    encode_to_file(&frame, &path).expect("encode AudioFrame 1024 samples");
    let decoded: AudioFrame = decode_from_file(&path).expect("decode AudioFrame 1024 samples");
    assert_eq!(frame, decoded);
    std::fs::remove_file(&path).expect("cleanup AudioFrame 1024 samples");
}

#[test]
fn test_audio_clip_simple_stereo_roundtrip() {
    let clip = AudioClip {
        config: default_config(),
        name: "test_clip_stereo".to_string(),
        frames: vec![
            stereo_frame(0, 512),
            stereo_frame(1, 512),
            stereo_frame(2, 512),
        ],
        duration_ms: 34.83,
    };
    let path = std::env::temp_dir().join("oxi_dsp16_clip_simple_stereo.bin");
    encode_to_file(&clip, &path).expect("encode AudioClip simple stereo");
    let decoded: AudioClip = decode_from_file(&path).expect("decode AudioClip simple stereo");
    assert_eq!(clip.name, decoded.name);
    assert_eq!(clip.frames.len(), decoded.frames.len());
    assert_eq!(clip.config, decoded.config);
    std::fs::remove_file(&path).expect("cleanup AudioClip simple stereo");
}

#[test]
fn test_audio_clip_surround51_roundtrip() {
    let cfg = AudioConfig {
        sample_rate: 48000,
        format: SampleFormat::Float32,
        channels: ChannelLayout::Surround51,
        bit_depth: 32,
    };
    let frames: Vec<AudioFrame> = (0..6).map(|i| stereo_frame(i, 256)).collect();
    let clip = AudioClip {
        config: cfg,
        name: "surround_5_1_clip".to_string(),
        frames,
        duration_ms: 32.0,
    };
    let path = std::env::temp_dir().join("oxi_dsp16_clip_51surround.bin");
    encode_to_file(&clip, &path).expect("encode AudioClip 5.1 surround");
    let decoded: AudioClip = decode_from_file(&path).expect("decode AudioClip 5.1 surround");
    assert_eq!(clip, decoded);
    std::fs::remove_file(&path).expect("cleanup AudioClip 5.1 surround");
}

#[test]
fn test_audio_clip_empty_frames_roundtrip() {
    let clip = AudioClip {
        config: AudioConfig {
            sample_rate: 22050,
            format: SampleFormat::PCM8,
            channels: ChannelLayout::Mono,
            bit_depth: 8,
        },
        name: "empty_clip".to_string(),
        frames: vec![],
        duration_ms: 0.0,
    };
    let path = std::env::temp_dir().join("oxi_dsp16_clip_empty.bin");
    encode_to_file(&clip, &path).expect("encode empty AudioClip");
    let decoded: AudioClip = decode_from_file(&path).expect("decode empty AudioClip");
    assert_eq!(clip, decoded);
    assert!(decoded.frames.is_empty());
    std::fs::remove_file(&path).expect("cleanup empty AudioClip");
}

#[test]
fn test_audio_clip_large_many_frames_roundtrip() {
    let frames: Vec<AudioFrame> = (0..20).map(|i| stereo_frame(i, 512)).collect();
    let clip = AudioClip {
        config: AudioConfig {
            sample_rate: 44100,
            format: SampleFormat::Float64,
            channels: ChannelLayout::Stereo,
            bit_depth: 64,
        },
        name: "large_clip_many_frames".to_string(),
        duration_ms: frames.len() as f64 * (512.0 / 44100.0) * 1000.0,
        frames,
    };
    let path = std::env::temp_dir().join("oxi_dsp16_clip_large_many_frames.bin");
    encode_to_file(&clip, &path).expect("encode AudioClip large many frames");
    let decoded: AudioClip = decode_from_file(&path).expect("decode AudioClip large many frames");
    assert_eq!(clip.frames.len(), decoded.frames.len());
    assert_eq!(clip.name, decoded.name);
    assert!((clip.duration_ms - decoded.duration_ms).abs() < 1e-9);
    std::fs::remove_file(&path).expect("cleanup AudioClip large many frames");
}

#[test]
fn test_file_bytes_match_encode_to_vec() {
    let cfg = default_config();
    let path = std::env::temp_dir().join("oxi_dsp16_bytes_match.bin");
    encode_to_file(&cfg, &path).expect("encode for bytes_match test");
    let file_bytes = std::fs::read(&path).expect("read file for bytes_match test");
    let vec_bytes = encode_to_vec(&cfg).expect("encode_to_vec for bytes_match test");
    assert_eq!(file_bytes, vec_bytes);
    std::fs::remove_file(&path).expect("cleanup bytes_match test");
}

#[test]
fn test_decode_missing_file_returns_error() {
    let path = std::env::temp_dir().join("oxi_dsp16_nonexistent_xyz_file.bin");
    let result = decode_from_file::<AudioConfig>(&path);
    assert!(result.is_err(), "expected error decoding missing file");
}

#[test]
fn test_multiple_writes_same_path_last_wins() {
    let path = std::env::temp_dir().join("oxi_dsp16_overwrite_same_path.bin");

    let cfg1 = AudioConfig {
        sample_rate: 8000,
        format: SampleFormat::PCM8,
        channels: ChannelLayout::Mono,
        bit_depth: 8,
    };
    let cfg2 = AudioConfig {
        sample_rate: 192000,
        format: SampleFormat::Float64,
        channels: ChannelLayout::Surround71,
        bit_depth: 64,
    };

    encode_to_file(&cfg1, &path).expect("first write to same path");
    encode_to_file(&cfg2, &path).expect("second write to same path");

    let decoded: AudioConfig = decode_from_file(&path).expect("decode after overwrite");
    assert_eq!(cfg2, decoded);
    std::fs::remove_file(&path).expect("cleanup overwrite same path");
}

#[test]
fn test_audio_frame_encode_to_vec_then_decode_from_slice() {
    let frame = stereo_frame(99, 600);
    let bytes = encode_to_vec(&frame).expect("encode_to_vec AudioFrame");
    let (decoded, consumed): (AudioFrame, _) =
        decode_from_slice(&bytes).expect("decode_from_slice AudioFrame");
    assert_eq!(frame, decoded);
    assert_eq!(consumed, bytes.len());
}

#[test]
fn test_audio_clip_surround71_high_resolution_roundtrip() {
    let cfg = AudioConfig {
        sample_rate: 192000,
        format: SampleFormat::PCM32,
        channels: ChannelLayout::Surround71,
        bit_depth: 32,
    };
    let frames: Vec<AudioFrame> = (0..8).map(|i| stereo_frame(i, 512)).collect();
    let clip = AudioClip {
        config: cfg,
        name: "hi_res_71_clip".to_string(),
        frames,
        duration_ms: 21.33,
    };
    let path = std::env::temp_dir().join("oxi_dsp16_clip_71hires.bin");
    encode_to_file(&clip, &path).expect("encode AudioClip 7.1 hi-res");
    let decoded: AudioClip = decode_from_file(&path).expect("decode AudioClip 7.1 hi-res");
    assert_eq!(clip, decoded);
    std::fs::remove_file(&path).expect("cleanup AudioClip 7.1 hi-res");
}

#[test]
fn test_temp_file_cleanup_verified() {
    let path = std::env::temp_dir().join("oxi_dsp16_cleanup_check.bin");
    let frame = stereo_frame(0, 256);
    encode_to_file(&frame, &path).expect("encode for cleanup check");
    assert!(path.exists(), "file should exist before removal");
    std::fs::remove_file(&path).expect("cleanup cleanup_check");
    assert!(!path.exists(), "file should not exist after removal");
}

#[test]
fn test_audio_config_all_sample_formats_sequential() {
    let formats = [
        SampleFormat::PCM8,
        SampleFormat::PCM16,
        SampleFormat::PCM24,
        SampleFormat::PCM32,
        SampleFormat::Float32,
        SampleFormat::Float64,
    ];
    for (idx, fmt) in formats.iter().enumerate() {
        let cfg = AudioConfig {
            sample_rate: 44100,
            format: fmt.clone(),
            channels: ChannelLayout::Stereo,
            bit_depth: 16,
        };
        let path = std::env::temp_dir().join(format!("oxi_dsp16_all_formats_{idx}.bin"));
        encode_to_file(&cfg, &path).expect("encode AudioConfig all formats");
        let decoded: AudioConfig = decode_from_file(&path).expect("decode AudioConfig all formats");
        assert_eq!(cfg, decoded);
        std::fs::remove_file(&path).expect("cleanup AudioConfig all formats");
    }
}

#[test]
fn test_audio_clip_name_unicode_roundtrip() {
    let clip = AudioClip {
        config: default_config(),
        name: "音声クリップ_DSP_тест".to_string(),
        frames: vec![stereo_frame(0, 128)],
        duration_ms: 2.9,
    };
    let path = std::env::temp_dir().join("oxi_dsp16_clip_unicode_name.bin");
    encode_to_file(&clip, &path).expect("encode AudioClip unicode name");
    let decoded: AudioClip = decode_from_file(&path).expect("decode AudioClip unicode name");
    assert_eq!(clip.name, decoded.name);
    std::fs::remove_file(&path).expect("cleanup AudioClip unicode name");
}

#[test]
fn test_audio_frame_timestamp_precision_roundtrip() {
    let precise_ts = 1234.5678901234_f64;
    let frame = AudioFrame {
        samples: vec![-1.0_f32, 0.0, 1.0],
        frame_number: u64::MAX,
        timestamp_ms: precise_ts,
    };
    let path = std::env::temp_dir().join("oxi_dsp16_frame_timestamp_precision.bin");
    encode_to_file(&frame, &path).expect("encode AudioFrame timestamp precision");
    let decoded: AudioFrame =
        decode_from_file(&path).expect("decode AudioFrame timestamp precision");
    assert_eq!(frame.frame_number, decoded.frame_number);
    assert_eq!(precise_ts.to_bits(), decoded.timestamp_ms.to_bits());
    assert_eq!(frame.samples, decoded.samples);
    std::fs::remove_file(&path).expect("cleanup AudioFrame timestamp precision");
}
