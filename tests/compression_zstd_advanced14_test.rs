//! Advanced Zstd compression tests for audio processing / digital signal processing domain.
//!
//! Covers encode → compress → decompress → decode roundtrips for audio/DSP structures,
//! large buffer compression ratios, MIDI event lists, spectral analysis frames, and
//! error/edge-case scenarios — all exercised exclusively via the Zstd codec.

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

// ---------------------------------------------------------------------------
// Domain model
// ---------------------------------------------------------------------------

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
enum AudioCodec {
    Pcm,
    Mp3,
    Aac,
    Opus,
    Flac,
    Vorbis,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct AudioSample {
    channel: u8,
    value_i16: i16,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct AudioFrame {
    frame_id: u64,
    samples: Vec<AudioSample>,
    sample_rate_hz: u32,
    bit_depth: u8,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct MidiNote {
    note_num: u8,
    velocity: u8,
    channel: u8,
    timestamp_ticks: u32,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct MidiEvent {
    event_id: u64,
    note: MidiNote,
    duration_ticks: u32,
    is_on: bool,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct SpectralBin {
    frequency_hz: f32,
    magnitude_db: f32,
    phase_rad: f32,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct FftFrame {
    frame_id: u64,
    bins: Vec<SpectralBin>,
    window_size: u32,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct AudioMetadata {
    title: String,
    artist: String,
    duration_ms: u32,
    sample_rate: u32,
    channels: u8,
    codec: AudioCodec,
}

// ---------------------------------------------------------------------------
// Helper: build a sine-wave i16 sample buffer with a fixed repetitive pattern.
// The values repeat with period `period`, producing a highly compressible byte
// sequence when encoded.
// ---------------------------------------------------------------------------
fn sine_wave_i16(count: usize, period: usize) -> Vec<i16> {
    let table: Vec<i16> = (0..period)
        .map(|i| {
            let angle = (i as f64 / period as f64) * 2.0 * std::f64::consts::PI;
            (angle.sin() * i16::MAX as f64) as i16
        })
        .collect();
    (0..count).map(|i| table[i % period]).collect()
}

// ---------------------------------------------------------------------------
// Test 1: AudioSample roundtrip
// ---------------------------------------------------------------------------
#[test]
fn test_audio_sample_roundtrip() {
    let sample = AudioSample {
        channel: 1,
        value_i16: -1234,
    };
    let encoded = encode_to_vec(&sample).expect("encode AudioSample failed");
    let compressed = compress(&encoded, Compression::Zstd).expect("compress AudioSample failed");
    let decompressed = decompress(&compressed).expect("decompress AudioSample failed");
    let (decoded, _): (AudioSample, usize) =
        decode_from_slice(&decompressed).expect("decode AudioSample failed");
    assert_eq!(sample, decoded);
}

// ---------------------------------------------------------------------------
// Test 2: AudioFrame roundtrip with stereo samples
// ---------------------------------------------------------------------------
#[test]
fn test_audio_frame_stereo_roundtrip() {
    let frame = AudioFrame {
        frame_id: 42,
        samples: vec![
            AudioSample {
                channel: 0,
                value_i16: 10000,
            },
            AudioSample {
                channel: 1,
                value_i16: -10000,
            },
            AudioSample {
                channel: 0,
                value_i16: 20000,
            },
            AudioSample {
                channel: 1,
                value_i16: -20000,
            },
        ],
        sample_rate_hz: 44100,
        bit_depth: 16,
    };
    let encoded = encode_to_vec(&frame).expect("encode AudioFrame failed");
    let compressed = compress(&encoded, Compression::Zstd).expect("compress AudioFrame failed");
    let decompressed = decompress(&compressed).expect("decompress AudioFrame failed");
    let (decoded, _): (AudioFrame, usize) =
        decode_from_slice(&decompressed).expect("decode AudioFrame failed");
    assert_eq!(frame, decoded);
}

// ---------------------------------------------------------------------------
// Test 3: MidiNote roundtrip
// ---------------------------------------------------------------------------
#[test]
fn test_midi_note_roundtrip() {
    let note = MidiNote {
        note_num: 60,
        velocity: 100,
        channel: 0,
        timestamp_ticks: 480,
    };
    let encoded = encode_to_vec(&note).expect("encode MidiNote failed");
    let compressed = compress(&encoded, Compression::Zstd).expect("compress MidiNote failed");
    let decompressed = decompress(&compressed).expect("decompress MidiNote failed");
    let (decoded, _): (MidiNote, usize) =
        decode_from_slice(&decompressed).expect("decode MidiNote failed");
    assert_eq!(note, decoded);
}

// ---------------------------------------------------------------------------
// Test 4: MidiEvent roundtrip (note-on and note-off variants)
// ---------------------------------------------------------------------------
#[test]
fn test_midi_event_note_on_off_roundtrip() {
    let events = vec![
        MidiEvent {
            event_id: 1,
            note: MidiNote {
                note_num: 69,
                velocity: 127,
                channel: 0,
                timestamp_ticks: 0,
            },
            duration_ticks: 960,
            is_on: true,
        },
        MidiEvent {
            event_id: 2,
            note: MidiNote {
                note_num: 69,
                velocity: 0,
                channel: 0,
                timestamp_ticks: 960,
            },
            duration_ticks: 0,
            is_on: false,
        },
    ];
    let encoded = encode_to_vec(&events).expect("encode MidiEvent vec failed");
    let compressed = compress(&encoded, Compression::Zstd).expect("compress MidiEvent vec failed");
    let decompressed = decompress(&compressed).expect("decompress MidiEvent vec failed");
    let (decoded, _): (Vec<MidiEvent>, usize) =
        decode_from_slice(&decompressed).expect("decode MidiEvent vec failed");
    assert_eq!(events, decoded);
}

// ---------------------------------------------------------------------------
// Test 5: SpectralBin roundtrip
// ---------------------------------------------------------------------------
#[test]
fn test_spectral_bin_roundtrip() {
    let bin = SpectralBin {
        frequency_hz: 440.0,
        magnitude_db: -6.0,
        phase_rad: std::f32::consts::FRAC_PI_2,
    };
    let encoded = encode_to_vec(&bin).expect("encode SpectralBin failed");
    let compressed = compress(&encoded, Compression::Zstd).expect("compress SpectralBin failed");
    let decompressed = decompress(&compressed).expect("decompress SpectralBin failed");
    let (decoded, _): (SpectralBin, usize) =
        decode_from_slice(&decompressed).expect("decode SpectralBin failed");
    assert_eq!(bin, decoded);
}

// ---------------------------------------------------------------------------
// Test 6: FftFrame roundtrip with realistic frequency bins
// ---------------------------------------------------------------------------
#[test]
fn test_fft_frame_roundtrip() {
    let bins: Vec<SpectralBin> = (0..512)
        .map(|i| SpectralBin {
            frequency_hz: i as f32 * 43.066_406,
            magnitude_db: -60.0 + (i as f32 * 0.1),
            phase_rad: 0.0,
        })
        .collect();
    let frame = FftFrame {
        frame_id: 100,
        bins,
        window_size: 1024,
    };
    let encoded = encode_to_vec(&frame).expect("encode FftFrame failed");
    let compressed = compress(&encoded, Compression::Zstd).expect("compress FftFrame failed");
    let decompressed = decompress(&compressed).expect("decompress FftFrame failed");
    let (decoded, _): (FftFrame, usize) =
        decode_from_slice(&decompressed).expect("decode FftFrame failed");
    assert_eq!(frame, decoded);
}

// ---------------------------------------------------------------------------
// Test 7: AudioMetadata roundtrip for all AudioCodec variants
// ---------------------------------------------------------------------------
#[test]
fn test_audio_metadata_all_codec_variants() {
    let codecs = vec![
        AudioCodec::Pcm,
        AudioCodec::Mp3,
        AudioCodec::Aac,
        AudioCodec::Opus,
        AudioCodec::Flac,
        AudioCodec::Vorbis,
    ];
    for codec in codecs {
        let meta = AudioMetadata {
            title: String::from("Test Track"),
            artist: String::from("COOLJAPAN DSP"),
            duration_ms: 210_000,
            sample_rate: 48000,
            channels: 2,
            codec: codec.clone(),
        };
        let encoded = encode_to_vec(&meta).expect("encode AudioMetadata failed");
        let compressed =
            compress(&encoded, Compression::Zstd).expect("compress AudioMetadata failed");
        let decompressed = decompress(&compressed).expect("decompress AudioMetadata failed");
        let (decoded, _): (AudioMetadata, usize) =
            decode_from_slice(&decompressed).expect("decode AudioMetadata failed");
        assert_eq!(meta, decoded, "roundtrip failed for codec {:?}", codec);
    }
}

// ---------------------------------------------------------------------------
// Test 8: Large audio buffer — 1000+ repetitive i16 samples as raw bytes,
//         verify compression ratio > 1.0 (compressed is smaller)
// ---------------------------------------------------------------------------
#[test]
fn test_large_repetitive_audio_buffer_compression_ratio() {
    // 8192 i16 samples repeating a 64-sample sine wave period
    let samples = sine_wave_i16(8192, 64);
    let raw_bytes: Vec<u8> = samples.iter().flat_map(|s| s.to_le_bytes()).collect();

    let compressed =
        compress(&raw_bytes, Compression::Zstd).expect("compress large audio buffer failed");

    let ratio = raw_bytes.len() as f64 / compressed.len() as f64;
    assert!(
        ratio > 1.0,
        "Compression ratio ({:.3}) must be > 1.0 for 8192-sample repetitive sine wave",
        ratio
    );

    let decompressed = decompress(&compressed).expect("decompress large audio buffer failed");
    assert_eq!(raw_bytes, decompressed);
}

// ---------------------------------------------------------------------------
// Test 9: Large audio frame (1024 samples) encode → compress → roundtrip
// ---------------------------------------------------------------------------
#[test]
fn test_large_audio_frame_roundtrip() {
    let raw_samples = sine_wave_i16(1024, 64);
    let samples: Vec<AudioSample> = raw_samples
        .into_iter()
        .enumerate()
        .map(|(i, v)| AudioSample {
            channel: (i % 2) as u8,
            value_i16: v,
        })
        .collect();
    let frame = AudioFrame {
        frame_id: 999,
        samples,
        sample_rate_hz: 44100,
        bit_depth: 16,
    };
    let encoded = encode_to_vec(&frame).expect("encode large AudioFrame failed");
    let compressed =
        compress(&encoded, Compression::Zstd).expect("compress large AudioFrame failed");
    let decompressed = decompress(&compressed).expect("decompress large AudioFrame failed");
    let (decoded, _): (AudioFrame, usize) =
        decode_from_slice(&decompressed).expect("decode large AudioFrame failed");
    assert_eq!(frame, decoded);
}

// ---------------------------------------------------------------------------
// Test 10: Large MIDI event list (1000+ events) — compression ratio check
// ---------------------------------------------------------------------------
#[test]
fn test_large_midi_event_list_compression_ratio() {
    let events: Vec<MidiEvent> = (0u64..1200)
        .map(|i| MidiEvent {
            event_id: i,
            note: MidiNote {
                note_num: (60 + (i % 12)) as u8,
                velocity: 100,
                channel: 0,
                timestamp_ticks: (i * 480) as u32,
            },
            duration_ticks: 240,
            is_on: i % 2 == 0,
        })
        .collect();

    let encoded = encode_to_vec(&events).expect("encode large MIDI event list failed");
    let compressed =
        compress(&encoded, Compression::Zstd).expect("compress large MIDI event list failed");

    let ratio = encoded.len() as f64 / compressed.len() as f64;
    assert!(
        ratio > 1.0,
        "Compression ratio ({:.3}) must be > 1.0 for 1200-event repetitive MIDI list",
        ratio
    );

    let decompressed = decompress(&compressed).expect("decompress large MIDI event list failed");
    let (decoded, _): (Vec<MidiEvent>, usize) =
        decode_from_slice(&decompressed).expect("decode large MIDI event list failed");
    assert_eq!(events, decoded);
}

// ---------------------------------------------------------------------------
// Test 11: Large MIDI event list (1000+ events) — full roundtrip correctness
// ---------------------------------------------------------------------------
#[test]
fn test_large_midi_event_list_roundtrip_correctness() {
    let events: Vec<MidiEvent> = (0u64..1000)
        .map(|i| MidiEvent {
            event_id: i,
            note: MidiNote {
                note_num: (i % 128) as u8,
                velocity: (i % 128) as u8,
                channel: (i % 16) as u8,
                timestamp_ticks: (i * 120) as u32,
            },
            duration_ticks: 120,
            is_on: true,
        })
        .collect();

    let encoded = encode_to_vec(&events).expect("encode MIDI events failed");
    let compressed = compress(&encoded, Compression::Zstd).expect("compress MIDI events failed");
    let decompressed = decompress(&compressed).expect("decompress MIDI events failed");
    let (decoded, _): (Vec<MidiEvent>, usize) =
        decode_from_slice(&decompressed).expect("decode MIDI events failed");

    assert_eq!(events.len(), decoded.len());
    for (original, restored) in events.iter().zip(decoded.iter()) {
        assert_eq!(original, restored);
    }
}

// ---------------------------------------------------------------------------
// Test 12: Multiple FftFrames sequence roundtrip
// ---------------------------------------------------------------------------
#[test]
fn test_multiple_fft_frames_roundtrip() {
    let frames: Vec<FftFrame> = (0u64..50)
        .map(|frame_id| {
            let bins: Vec<SpectralBin> = (0..256)
                .map(|b| SpectralBin {
                    frequency_hz: b as f32 * 86.132_81,
                    magnitude_db: -80.0 + (b as f32 * 0.25),
                    phase_rad: (b as f32 * 0.024_544),
                })
                .collect();
            FftFrame {
                frame_id,
                bins,
                window_size: 512,
            }
        })
        .collect();

    let encoded = encode_to_vec(&frames).expect("encode FftFrame vec failed");
    let compressed = compress(&encoded, Compression::Zstd).expect("compress FftFrame vec failed");
    let decompressed = decompress(&compressed).expect("decompress FftFrame vec failed");
    let (decoded, _): (Vec<FftFrame>, usize) =
        decode_from_slice(&decompressed).expect("decode FftFrame vec failed");
    assert_eq!(frames, decoded);
}

// ---------------------------------------------------------------------------
// Test 13: ZstdLevel variants roundtrip for AudioFrame
// ---------------------------------------------------------------------------
#[test]
fn test_zstd_level_variants_audio_frame_roundtrip() {
    let frame = AudioFrame {
        frame_id: 77,
        samples: vec![
            AudioSample {
                channel: 0,
                value_i16: 32767,
            },
            AudioSample {
                channel: 1,
                value_i16: -32768,
            },
        ],
        sample_rate_hz: 96000,
        bit_depth: 24,
    };
    let encoded = encode_to_vec(&frame).expect("encode AudioFrame failed");

    for level in [1u8, 3, 9, 15, 19] {
        let compressed =
            compress(&encoded, Compression::ZstdLevel(level)).expect("compress at level failed");
        let decompressed = decompress(&compressed).expect("decompress at level failed");
        let (decoded, _): (AudioFrame, usize) =
            decode_from_slice(&decompressed).expect("decode at level failed");
        assert_eq!(frame, decoded, "roundtrip mismatch at ZstdLevel({})", level);
    }
}

// ---------------------------------------------------------------------------
// Test 14: Multiple compress/decompress cycles preserve data integrity
// ---------------------------------------------------------------------------
#[test]
fn test_multiple_compress_decompress_cycles() {
    let meta = AudioMetadata {
        title: String::from("Cycle Test"),
        artist: String::from("Iterative DSP"),
        duration_ms: 5000,
        sample_rate: 44100,
        channels: 1,
        codec: AudioCodec::Opus,
    };

    let encoded = encode_to_vec(&meta).expect("encode AudioMetadata failed");
    let mut payload = encoded.clone();

    // Compress and decompress 5 times in a row
    for cycle in 1..=5 {
        let compressed = compress(&payload, Compression::Zstd)
            .unwrap_or_else(|_| panic!("compress cycle {} failed", cycle));
        let decompressed =
            decompress(&compressed).unwrap_or_else(|_| panic!("decompress cycle {} failed", cycle));
        payload = decompressed;
    }

    let (decoded, _): (AudioMetadata, usize) =
        decode_from_slice(&payload).expect("decode after cycles failed");
    assert_eq!(meta, decoded);
}

// ---------------------------------------------------------------------------
// Test 15: Error returned for truncated compressed data
// ---------------------------------------------------------------------------
#[test]
fn test_truncated_compressed_data_returns_error() {
    let frame = AudioFrame {
        frame_id: 1,
        samples: vec![AudioSample {
            channel: 0,
            value_i16: 100,
        }],
        sample_rate_hz: 44100,
        bit_depth: 16,
    };
    let encoded = encode_to_vec(&frame).expect("encode failed");
    let compressed = compress(&encoded, Compression::Zstd).expect("compress failed");

    // Truncate to roughly half the compressed payload
    let truncated = &compressed[..compressed.len() / 2];
    let result = decompress(truncated);
    assert!(
        result.is_err(),
        "decompress() must return an error for truncated Zstd data"
    );
}

// ---------------------------------------------------------------------------
// Test 16: AudioMetadata with Unicode title/artist roundtrip
// ---------------------------------------------------------------------------
#[test]
fn test_audio_metadata_unicode_roundtrip() {
    let meta = AudioMetadata {
        title: String::from("音楽テスト / Musik Test / 音乐测试"),
        artist: String::from("アーティスト — Künstler — 艺术家"),
        duration_ms: 300_000,
        sample_rate: 44100,
        channels: 2,
        codec: AudioCodec::Flac,
    };
    let encoded = encode_to_vec(&meta).expect("encode Unicode metadata failed");
    let compressed =
        compress(&encoded, Compression::Zstd).expect("compress Unicode metadata failed");
    let decompressed = decompress(&compressed).expect("decompress Unicode metadata failed");
    let (decoded, _): (AudioMetadata, usize) =
        decode_from_slice(&decompressed).expect("decode Unicode metadata failed");
    assert_eq!(meta, decoded);
}

// ---------------------------------------------------------------------------
// Test 17: AudioSample boundary values (i16::MIN, i16::MAX, 0)
// ---------------------------------------------------------------------------
#[test]
fn test_audio_sample_boundary_values_roundtrip() {
    let samples = vec![
        AudioSample {
            channel: 0,
            value_i16: i16::MIN,
        },
        AudioSample {
            channel: 1,
            value_i16: i16::MAX,
        },
        AudioSample {
            channel: 0,
            value_i16: 0,
        },
        AudioSample {
            channel: 255,
            value_i16: -1,
        },
    ];
    let encoded = encode_to_vec(&samples).expect("encode boundary samples failed");
    let compressed =
        compress(&encoded, Compression::Zstd).expect("compress boundary samples failed");
    let decompressed = decompress(&compressed).expect("decompress boundary samples failed");
    let (decoded, _): (Vec<AudioSample>, usize) =
        decode_from_slice(&decompressed).expect("decode boundary samples failed");
    assert_eq!(samples, decoded);
}

// ---------------------------------------------------------------------------
// Test 18: MidiNote boundary values roundtrip
// ---------------------------------------------------------------------------
#[test]
fn test_midi_note_boundary_values_roundtrip() {
    let notes = vec![
        MidiNote {
            note_num: 0,
            velocity: 0,
            channel: 0,
            timestamp_ticks: 0,
        },
        MidiNote {
            note_num: 127,
            velocity: 127,
            channel: 15,
            timestamp_ticks: u32::MAX,
        },
        MidiNote {
            note_num: 60,
            velocity: 64,
            channel: 9,
            timestamp_ticks: 1_920_000,
        },
    ];
    let encoded = encode_to_vec(&notes).expect("encode boundary MidiNotes failed");
    let compressed =
        compress(&encoded, Compression::Zstd).expect("compress boundary MidiNotes failed");
    let decompressed = decompress(&compressed).expect("decompress boundary MidiNotes failed");
    let (decoded, _): (Vec<MidiNote>, usize) =
        decode_from_slice(&decompressed).expect("decode boundary MidiNotes failed");
    assert_eq!(notes, decoded);
}

// ---------------------------------------------------------------------------
// Test 19: SpectralBin with NaN-free extreme float values roundtrip
// ---------------------------------------------------------------------------
#[test]
fn test_spectral_bin_extreme_float_values_roundtrip() {
    let bins = vec![
        SpectralBin {
            frequency_hz: 0.0,
            magnitude_db: -120.0,
            phase_rad: 0.0,
        },
        SpectralBin {
            frequency_hz: 22050.0,
            magnitude_db: 0.0,
            phase_rad: std::f32::consts::PI,
        },
        SpectralBin {
            frequency_hz: f32::MIN_POSITIVE,
            magnitude_db: -0.001,
            phase_rad: -std::f32::consts::PI,
        },
    ];
    let encoded = encode_to_vec(&bins).expect("encode extreme SpectralBins failed");
    let compressed =
        compress(&encoded, Compression::Zstd).expect("compress extreme SpectralBins failed");
    let decompressed = decompress(&compressed).expect("decompress extreme SpectralBins failed");
    let (decoded, _): (Vec<SpectralBin>, usize) =
        decode_from_slice(&decompressed).expect("decode extreme SpectralBins failed");
    assert_eq!(bins, decoded);
}

// ---------------------------------------------------------------------------
// Test 20: Large repetitive AudioFrame list — compression ratio > 1.0
// ---------------------------------------------------------------------------
#[test]
fn test_repetitive_audio_frame_list_compression_ratio() {
    // 50 identical frames → highly repetitive, must compress well
    let template_samples: Vec<AudioSample> = sine_wave_i16(64, 64)
        .into_iter()
        .enumerate()
        .map(|(i, v)| AudioSample {
            channel: (i % 2) as u8,
            value_i16: v,
        })
        .collect();

    let frames: Vec<AudioFrame> = (0u64..50)
        .map(|id| AudioFrame {
            frame_id: id,
            samples: template_samples.clone(),
            sample_rate_hz: 44100,
            bit_depth: 16,
        })
        .collect();

    let encoded = encode_to_vec(&frames).expect("encode repetitive AudioFrames failed");
    let compressed =
        compress(&encoded, Compression::Zstd).expect("compress repetitive AudioFrames failed");

    let ratio = encoded.len() as f64 / compressed.len() as f64;
    assert!(
        ratio > 1.0,
        "Compression ratio ({:.3}) must be > 1.0 for 50 repetitive AudioFrames",
        ratio
    );

    let decompressed = decompress(&compressed).expect("decompress repetitive AudioFrames failed");
    let (decoded, _): (Vec<AudioFrame>, usize) =
        decode_from_slice(&decompressed).expect("decode repetitive AudioFrames failed");
    assert_eq!(frames, decoded);
}

// ---------------------------------------------------------------------------
// Test 21: Mixed audio pipeline — metadata + frame + MIDI in a single encoded blob
// ---------------------------------------------------------------------------
#[test]
fn test_mixed_audio_pipeline_roundtrip() {
    let meta = AudioMetadata {
        title: String::from("Live Session"),
        artist: String::from("DSP Studio"),
        duration_ms: 180_000,
        sample_rate: 48000,
        channels: 2,
        codec: AudioCodec::Aac,
    };
    let frame = AudioFrame {
        frame_id: 0,
        samples: vec![
            AudioSample {
                channel: 0,
                value_i16: 1000,
            },
            AudioSample {
                channel: 1,
                value_i16: -1000,
            },
        ],
        sample_rate_hz: 48000,
        bit_depth: 16,
    };
    let event = MidiEvent {
        event_id: 0,
        note: MidiNote {
            note_num: 48,
            velocity: 90,
            channel: 1,
            timestamp_ticks: 0,
        },
        duration_ticks: 1920,
        is_on: true,
    };

    let meta_enc = encode_to_vec(&meta).expect("encode meta failed");
    let frame_enc = encode_to_vec(&frame).expect("encode frame failed");
    let event_enc = encode_to_vec(&event).expect("encode event failed");

    let meta_c = compress(&meta_enc, Compression::Zstd).expect("compress meta failed");
    let frame_c = compress(&frame_enc, Compression::Zstd).expect("compress frame failed");
    let event_c = compress(&event_enc, Compression::Zstd).expect("compress event failed");

    let meta_d = decompress(&meta_c).expect("decompress meta failed");
    let frame_d = decompress(&frame_c).expect("decompress frame failed");
    let event_d = decompress(&event_c).expect("decompress event failed");

    let (meta_out, _): (AudioMetadata, usize) =
        decode_from_slice(&meta_d).expect("decode meta failed");
    let (frame_out, _): (AudioFrame, usize) =
        decode_from_slice(&frame_d).expect("decode frame failed");
    let (event_out, _): (MidiEvent, usize) =
        decode_from_slice(&event_d).expect("decode event failed");

    assert_eq!(meta, meta_out);
    assert_eq!(frame, frame_out);
    assert_eq!(event, event_out);
}

// ---------------------------------------------------------------------------
// Test 22: 1000+ raw i16 audio samples as bytes, check compression ratio > 1.0
//          and exact byte-level roundtrip
// ---------------------------------------------------------------------------
#[test]
fn test_raw_i16_sample_bytes_compression_ratio_and_roundtrip() {
    // 4096 samples repeating a 32-sample sine, giving very high compressibility
    let samples = sine_wave_i16(4096, 32);
    assert!(samples.len() >= 1000, "test requires 1000+ samples");

    let raw_bytes: Vec<u8> = samples.iter().flat_map(|s| s.to_le_bytes()).collect();

    let compressed =
        compress(&raw_bytes, Compression::Zstd).expect("compress raw i16 bytes failed");

    let ratio = raw_bytes.len() as f64 / compressed.len() as f64;
    assert!(
        ratio > 1.0,
        "Compression ratio ({:.3}) must be > 1.0 for {}-byte repetitive i16 audio data",
        ratio,
        raw_bytes.len()
    );

    let decompressed = decompress(&compressed).expect("decompress raw i16 bytes failed");
    assert_eq!(
        raw_bytes, decompressed,
        "byte-level roundtrip must be exact for raw i16 audio samples"
    );
}
