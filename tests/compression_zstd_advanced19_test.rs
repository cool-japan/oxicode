//! Advanced Zstd compression tests for OxiCode — Audio Engineering & Music Production domain.
//!
//! Covers encode → compress → decompress → decode round-trips for types that
//! model real-world audio engineering data: audio clip metadata, mixing console
//! channel strips, MIDI event sequences, synthesizer patches, mastering chains,
//! reverb/delay effects, DAW sessions, beat grids, spatial audio panning,
//! loudness metering, plugin preset banks, studio calibration, session musician
//! credits, streaming codec configs, and vinyl mastering specs.

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
// Domain types
// ---------------------------------------------------------------------------

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
enum BitDepth {
    Pcm16,
    Pcm24,
    Pcm32,
    Float32,
    Float64,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
enum ChannelLayout {
    Mono,
    Stereo,
    Surround51,
    Surround71,
    Atmos714,
    AmbisonicsFirstOrder,
    AmbisonicsSecondOrder,
    AmbisonicsThirdOrder,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
enum EqBandType {
    LowShelf,
    HighShelf,
    Bell,
    LowPass,
    HighPass,
    Notch,
    Bandpass,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
enum CompressorKneeType {
    Hard,
    Soft,
    VariableMu,
    OpticalSmooth,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
enum MidiMessageType {
    NoteOn,
    NoteOff,
    ControlChange,
    PitchBend,
    Aftertouch,
    ProgramChange,
    SysEx,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
enum OscillatorWaveform {
    Sine,
    Saw,
    Square,
    Triangle,
    WhiteNoise,
    PinkNoise,
    Wavetable,
    Fm,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
enum FilterType {
    Lowpass12,
    Lowpass24,
    Highpass12,
    Highpass24,
    Bandpass,
    Comb,
    FormantVowel,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
enum ReverbAlgorithm {
    Plate,
    Hall,
    Room,
    Chamber,
    Spring,
    Shimmer,
    ConvolutionIr,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
enum DelayType {
    SimpleDigital,
    PingPong,
    TapeEmulation,
    BucketBrigade,
    Granular,
    ReverseDelay,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
enum SpatialPanningMode {
    AmbisonicsAcn,
    DolbyAtmosObjectBased,
    DolbyAtmosBedBased,
    Binaural,
    Vbap,
    Dbap,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
enum StreamingCodec {
    AacLc,
    OpusVbr,
    OpusCbr,
    Mp3Cbr,
    FlacLossless,
    VorbisOgg,
    Alac,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
enum VinylCuttingStyle {
    HotStylus,
    DirectMetalMastering,
    Lacquer,
    HalfSpeed,
}

#[derive(Debug, PartialEq, Clone, Encode, Decode)]
enum PluginFormat {
    Vst3,
    AudioUnit,
    Aax,
    Clap,
    Lv2,
}

// ---------------------------------------------------------------------------
// Composite domain structs
// ---------------------------------------------------------------------------

/// Audio clip metadata — sample rate, bit depth, channels, duration.
#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct AudioClipMeta {
    clip_id: u64,
    name: String,
    sample_rate_hz: u32,
    bit_depth: BitDepth,
    channel_layout: ChannelLayout,
    duration_samples: u64,
    peak_amplitude_milli_dbfs: i32,
    file_size_bytes: u64,
    tags: Vec<String>,
}

/// Single EQ band on a mixing console channel strip.
#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct EqBand {
    band_type: EqBandType,
    frequency_hz_x10: u32,
    gain_milli_db: i32,
    q_factor_x100: u16,
    enabled: bool,
}

/// Compressor settings on a channel strip.
#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct CompressorSettings {
    threshold_milli_dbfs: i32,
    ratio_x10: u16,
    attack_us: u32,
    release_us: u32,
    makeup_gain_milli_db: i32,
    knee_type: CompressorKneeType,
    sidechain_hpf_hz: u16,
    enabled: bool,
}

/// Full mixing console channel strip.
#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct ChannelStrip {
    strip_index: u8,
    label: String,
    gain_milli_db: i32,
    pan_x100: i16,
    phase_invert: bool,
    mute: bool,
    solo: bool,
    eq_bands: Vec<EqBand>,
    compressor: CompressorSettings,
    send_levels_milli_db: Vec<i32>,
}

/// A single MIDI event.
#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct MidiEvent {
    tick: u64,
    channel: u8,
    message_type: MidiMessageType,
    data_byte_1: u8,
    data_byte_2: u8,
}

/// MIDI sequence with header info.
#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct MidiSequence {
    name: String,
    ticks_per_quarter_note: u16,
    tempo_bpm_x100: u32,
    events: Vec<MidiEvent>,
}

/// ADSR envelope for a synthesizer.
#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct AdsrEnvelope {
    attack_us: u32,
    decay_us: u32,
    sustain_level_x1000: u16,
    release_us: u32,
}

/// A single oscillator in a synth.
#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct Oscillator {
    waveform: OscillatorWaveform,
    detune_cents: i16,
    level_x1000: u16,
    octave_offset: i8,
    pulse_width_x1000: u16,
}

/// Synthesizer filter section.
#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct SynthFilter {
    filter_type: FilterType,
    cutoff_hz_x10: u32,
    resonance_x1000: u16,
    envelope_amount_x1000: i16,
    key_tracking_x100: u16,
}

/// Complete synthesizer patch.
#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct SynthPatch {
    patch_name: String,
    category: String,
    oscillators: Vec<Oscillator>,
    filter: SynthFilter,
    amp_envelope: AdsrEnvelope,
    filter_envelope: AdsrEnvelope,
    portamento_ms: u16,
    unison_voices: u8,
    unison_spread_cents: u16,
}

/// A single processor in a mastering chain.
#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct MasteringProcessor {
    slot_index: u8,
    plugin_name: String,
    plugin_format: PluginFormat,
    bypass: bool,
    wet_dry_x1000: u16,
}

/// Full mastering chain configuration.
#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct MasteringChain {
    session_name: String,
    target_lufs_x10: i16,
    true_peak_limit_milli_dbfs: i32,
    processors: Vec<MasteringProcessor>,
    dither_enabled: bool,
    output_bit_depth: BitDepth,
    output_sample_rate_hz: u32,
}

/// Reverb effect parameters.
#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct ReverbParams {
    algorithm: ReverbAlgorithm,
    pre_delay_us: u32,
    decay_time_ms: u32,
    damping_x1000: u16,
    diffusion_x1000: u16,
    room_size_x1000: u16,
    high_cut_hz: u16,
    low_cut_hz: u16,
    wet_x1000: u16,
    dry_x1000: u16,
    modulation_rate_mhz: u16,
    modulation_depth_x1000: u16,
}

/// Delay effect parameters.
#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct DelayParams {
    delay_type: DelayType,
    time_left_us: u32,
    time_right_us: u32,
    feedback_x1000: u16,
    high_cut_hz: u16,
    low_cut_hz: u16,
    saturation_x1000: u16,
    wet_x1000: u16,
    sync_to_tempo: bool,
}

/// Track metadata inside a DAW session.
#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct DawTrack {
    track_id: u32,
    name: String,
    color_rgb: u32,
    channel_layout: ChannelLayout,
    armed: bool,
    input_monitoring: bool,
    region_count: u32,
}

/// DAW session metadata.
#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct DawSession {
    session_name: String,
    sample_rate_hz: u32,
    bit_depth: BitDepth,
    tempo_bpm_x100: u32,
    time_signature_numerator: u8,
    time_signature_denominator: u8,
    total_bars: u32,
    tracks: Vec<DawTrack>,
    markers: Vec<(u64, String)>,
}

/// A single tempo event in a beat grid.
#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct TempoEvent {
    bar_number: u32,
    beat_within_bar: u8,
    tempo_bpm_x100: u32,
}

/// Beat grid / tempo map.
#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct BeatGrid {
    initial_tempo_bpm_x100: u32,
    time_signature_numerator: u8,
    time_signature_denominator: u8,
    tempo_changes: Vec<TempoEvent>,
    downbeat_offsets_samples: Vec<u64>,
}

/// Spatial audio panning descriptor.
#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct SpatialPanDescriptor {
    object_id: u32,
    panning_mode: SpatialPanningMode,
    azimuth_deg_x100: i32,
    elevation_deg_x100: i32,
    distance_x1000: u32,
    spread_x1000: u16,
    divergence_x1000: u16,
}

/// Loudness metering result.
#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct LoudnessMetering {
    integrated_lufs_x100: i32,
    short_term_lufs_x100: i32,
    momentary_lufs_x100: i32,
    loudness_range_lu_x100: u32,
    true_peak_milli_dbfs: i32,
    sample_peak_milli_dbfs: i32,
    measurement_duration_ms: u64,
}

/// A single preset in a plugin preset bank.
#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct PluginPreset {
    preset_name: String,
    category: String,
    author: String,
    parameter_values_x1000: Vec<u32>,
}

/// Plugin preset bank.
#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct PluginPresetBank {
    plugin_name: String,
    plugin_format: PluginFormat,
    plugin_version: String,
    presets: Vec<PluginPreset>,
}

/// Studio equipment calibration record.
#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct StudioCalibration {
    equipment_name: String,
    serial_number: String,
    calibration_date_unix: u64,
    frequency_response_points: Vec<(u32, i32)>,
    thd_plus_noise_ppm: u32,
    channel_separation_milli_db: i32,
    max_spl_milli_db: i32,
}

/// Session musician credit entry.
#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct MusicianCredit {
    name: String,
    instrument: String,
    role: String,
    tracks_played: Vec<u32>,
    session_date_unix: u64,
    studio_name: String,
}

/// Streaming codec configuration.
#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct StreamingCodecConfig {
    codec: StreamingCodec,
    bitrate_kbps: u32,
    sample_rate_hz: u32,
    channel_layout: ChannelLayout,
    vbr_quality_x10: u8,
    low_latency: bool,
    frame_size_samples: u32,
}

/// Vinyl mastering specifications.
#[derive(Debug, PartialEq, Clone, Encode, Decode)]
struct VinylMasteringSpec {
    cutting_style: VinylCuttingStyle,
    rpm: u16,
    groove_width_um: u16,
    land_width_um: u16,
    inner_diameter_mm: u16,
    outer_diameter_mm: u16,
    max_playing_time_sec: u32,
    riaa_eq_applied: bool,
    high_frequency_limit_hz: u16,
    rumble_filter_hz: u16,
    de_esser_threshold_milli_db: i32,
}

// ---------------------------------------------------------------------------
// Tests (22 total)
// ---------------------------------------------------------------------------

/// 1. Audio clip metadata round-trip.
#[test]
fn test_zstd_audio_clip_meta_roundtrip() {
    let clip = AudioClipMeta {
        clip_id: 10001,
        name: "kick_drum_close_mic.wav".to_string(),
        sample_rate_hz: 96_000,
        bit_depth: BitDepth::Pcm24,
        channel_layout: ChannelLayout::Mono,
        duration_samples: 96_000 * 2,
        peak_amplitude_milli_dbfs: -1200,
        file_size_bytes: 576_000,
        tags: vec![
            "drums".to_string(),
            "kick".to_string(),
            "close-mic".to_string(),
        ],
    };

    let encoded = encode_to_vec(&clip).expect("encode AudioClipMeta failed");
    let compressed = compress(&encoded, Compression::Zstd).expect("compress AudioClipMeta failed");
    let decompressed = decompress(&compressed).expect("decompress AudioClipMeta failed");
    let (decoded, _): (AudioClipMeta, usize) =
        decode_from_slice(&decompressed).expect("decode AudioClipMeta failed");
    assert_eq!(clip, decoded);
}

/// 2. Mixing console channel strip round-trip.
#[test]
fn test_zstd_channel_strip_roundtrip() {
    let strip = ChannelStrip {
        strip_index: 1,
        label: "Lead Vocal".to_string(),
        gain_milli_db: 3500,
        pan_x100: 0,
        phase_invert: false,
        mute: false,
        solo: false,
        eq_bands: vec![
            EqBand {
                band_type: EqBandType::HighPass,
                frequency_hz_x10: 800,
                gain_milli_db: 0,
                q_factor_x100: 70,
                enabled: true,
            },
            EqBand {
                band_type: EqBandType::Bell,
                frequency_hz_x10: 32_000,
                gain_milli_db: 2500,
                q_factor_x100: 150,
                enabled: true,
            },
            EqBand {
                band_type: EqBandType::HighShelf,
                frequency_hz_x10: 120_000,
                gain_milli_db: 1500,
                q_factor_x100: 100,
                enabled: true,
            },
        ],
        compressor: CompressorSettings {
            threshold_milli_dbfs: -18_000,
            ratio_x10: 40,
            attack_us: 5_000,
            release_us: 120_000,
            makeup_gain_milli_db: 4000,
            knee_type: CompressorKneeType::Soft,
            sidechain_hpf_hz: 100,
            enabled: true,
        },
        send_levels_milli_db: vec![-12_000, -18_000, -24_000],
    };

    let encoded = encode_to_vec(&strip).expect("encode ChannelStrip failed");
    let compressed = compress(&encoded, Compression::Zstd).expect("compress ChannelStrip failed");
    let decompressed = decompress(&compressed).expect("decompress ChannelStrip failed");
    let (decoded, _): (ChannelStrip, usize) =
        decode_from_slice(&decompressed).expect("decode ChannelStrip failed");
    assert_eq!(strip, decoded);
}

/// 3. MIDI event sequence round-trip.
#[test]
fn test_zstd_midi_sequence_roundtrip() {
    let seq = MidiSequence {
        name: "Verse Piano Riff".to_string(),
        ticks_per_quarter_note: 480,
        tempo_bpm_x100: 12_000,
        events: vec![
            MidiEvent {
                tick: 0,
                channel: 0,
                message_type: MidiMessageType::NoteOn,
                data_byte_1: 60,
                data_byte_2: 100,
            },
            MidiEvent {
                tick: 240,
                channel: 0,
                message_type: MidiMessageType::NoteOff,
                data_byte_1: 60,
                data_byte_2: 0,
            },
            MidiEvent {
                tick: 240,
                channel: 0,
                message_type: MidiMessageType::NoteOn,
                data_byte_1: 64,
                data_byte_2: 90,
            },
            MidiEvent {
                tick: 480,
                channel: 0,
                message_type: MidiMessageType::NoteOff,
                data_byte_1: 64,
                data_byte_2: 0,
            },
            MidiEvent {
                tick: 480,
                channel: 0,
                message_type: MidiMessageType::ControlChange,
                data_byte_1: 64,
                data_byte_2: 127,
            },
            MidiEvent {
                tick: 960,
                channel: 0,
                message_type: MidiMessageType::ControlChange,
                data_byte_1: 64,
                data_byte_2: 0,
            },
        ],
    };

    let encoded = encode_to_vec(&seq).expect("encode MidiSequence failed");
    let compressed = compress(&encoded, Compression::Zstd).expect("compress MidiSequence failed");
    let decompressed = decompress(&compressed).expect("decompress MidiSequence failed");
    let (decoded, _): (MidiSequence, usize) =
        decode_from_slice(&decompressed).expect("decode MidiSequence failed");
    assert_eq!(seq, decoded);
}

/// 4. Synthesizer patch round-trip.
#[test]
fn test_zstd_synth_patch_roundtrip() {
    let patch = SynthPatch {
        patch_name: "Warm Analog Pad".to_string(),
        category: "Pad".to_string(),
        oscillators: vec![
            Oscillator {
                waveform: OscillatorWaveform::Saw,
                detune_cents: -7,
                level_x1000: 800,
                octave_offset: 0,
                pulse_width_x1000: 500,
            },
            Oscillator {
                waveform: OscillatorWaveform::Saw,
                detune_cents: 7,
                level_x1000: 800,
                octave_offset: 0,
                pulse_width_x1000: 500,
            },
            Oscillator {
                waveform: OscillatorWaveform::Square,
                detune_cents: 0,
                level_x1000: 400,
                octave_offset: -1,
                pulse_width_x1000: 300,
            },
        ],
        filter: SynthFilter {
            filter_type: FilterType::Lowpass24,
            cutoff_hz_x10: 8_000,
            resonance_x1000: 200,
            envelope_amount_x1000: 600,
            key_tracking_x100: 50,
        },
        amp_envelope: AdsrEnvelope {
            attack_us: 150_000,
            decay_us: 800_000,
            sustain_level_x1000: 700,
            release_us: 2_000_000,
        },
        filter_envelope: AdsrEnvelope {
            attack_us: 300_000,
            decay_us: 1_200_000,
            sustain_level_x1000: 300,
            release_us: 1_500_000,
        },
        portamento_ms: 20,
        unison_voices: 4,
        unison_spread_cents: 25,
    };

    let encoded = encode_to_vec(&patch).expect("encode SynthPatch failed");
    let compressed = compress(&encoded, Compression::Zstd).expect("compress SynthPatch failed");
    let decompressed = decompress(&compressed).expect("decompress SynthPatch failed");
    let (decoded, _): (SynthPatch, usize) =
        decode_from_slice(&decompressed).expect("decode SynthPatch failed");
    assert_eq!(patch, decoded);
}

/// 5. Mastering chain configuration round-trip.
#[test]
fn test_zstd_mastering_chain_roundtrip() {
    let chain = MasteringChain {
        session_name: "Album Final Master v3".to_string(),
        target_lufs_x10: -140,
        true_peak_limit_milli_dbfs: -1000,
        processors: vec![
            MasteringProcessor {
                slot_index: 0,
                plugin_name: "Linear Phase EQ".to_string(),
                plugin_format: PluginFormat::Vst3,
                bypass: false,
                wet_dry_x1000: 1000,
            },
            MasteringProcessor {
                slot_index: 1,
                plugin_name: "Multiband Compressor".to_string(),
                plugin_format: PluginFormat::Vst3,
                bypass: false,
                wet_dry_x1000: 1000,
            },
            MasteringProcessor {
                slot_index: 2,
                plugin_name: "Stereo Enhancer".to_string(),
                plugin_format: PluginFormat::AudioUnit,
                bypass: false,
                wet_dry_x1000: 500,
            },
            MasteringProcessor {
                slot_index: 3,
                plugin_name: "Brickwall Limiter".to_string(),
                plugin_format: PluginFormat::Aax,
                bypass: false,
                wet_dry_x1000: 1000,
            },
        ],
        dither_enabled: true,
        output_bit_depth: BitDepth::Pcm16,
        output_sample_rate_hz: 44_100,
    };

    let encoded = encode_to_vec(&chain).expect("encode MasteringChain failed");
    let compressed = compress(&encoded, Compression::Zstd).expect("compress MasteringChain failed");
    let decompressed = decompress(&compressed).expect("decompress MasteringChain failed");
    let (decoded, _): (MasteringChain, usize) =
        decode_from_slice(&decompressed).expect("decode MasteringChain failed");
    assert_eq!(chain, decoded);
}

/// 6. Reverb parameters round-trip.
#[test]
fn test_zstd_reverb_params_roundtrip() {
    let reverb = ReverbParams {
        algorithm: ReverbAlgorithm::Hall,
        pre_delay_us: 25_000,
        decay_time_ms: 2_800,
        damping_x1000: 600,
        diffusion_x1000: 850,
        room_size_x1000: 900,
        high_cut_hz: 8_000,
        low_cut_hz: 200,
        wet_x1000: 350,
        dry_x1000: 1000,
        modulation_rate_mhz: 500,
        modulation_depth_x1000: 80,
    };

    let encoded = encode_to_vec(&reverb).expect("encode ReverbParams failed");
    let compressed = compress(&encoded, Compression::Zstd).expect("compress ReverbParams failed");
    let decompressed = decompress(&compressed).expect("decompress ReverbParams failed");
    let (decoded, _): (ReverbParams, usize) =
        decode_from_slice(&decompressed).expect("decode ReverbParams failed");
    assert_eq!(reverb, decoded);
}

/// 7. Delay parameters round-trip.
#[test]
fn test_zstd_delay_params_roundtrip() {
    let delay = DelayParams {
        delay_type: DelayType::TapeEmulation,
        time_left_us: 375_000,
        time_right_us: 500_000,
        feedback_x1000: 450,
        high_cut_hz: 6_000,
        low_cut_hz: 150,
        saturation_x1000: 300,
        wet_x1000: 280,
        sync_to_tempo: true,
    };

    let encoded = encode_to_vec(&delay).expect("encode DelayParams failed");
    let compressed = compress(&encoded, Compression::Zstd).expect("compress DelayParams failed");
    let decompressed = decompress(&compressed).expect("decompress DelayParams failed");
    let (decoded, _): (DelayParams, usize) =
        decode_from_slice(&decompressed).expect("decode DelayParams failed");
    assert_eq!(delay, decoded);
}

/// 8. DAW session metadata round-trip.
#[test]
fn test_zstd_daw_session_roundtrip() {
    let session = DawSession {
        session_name: "Sunset Boulevard EP - Track 04".to_string(),
        sample_rate_hz: 48_000,
        bit_depth: BitDepth::Float32,
        tempo_bpm_x100: 9_800,
        time_signature_numerator: 4,
        time_signature_denominator: 4,
        total_bars: 128,
        tracks: vec![
            DawTrack {
                track_id: 1,
                name: "Drums Bus".to_string(),
                color_rgb: 0xFF4444,
                channel_layout: ChannelLayout::Stereo,
                armed: false,
                input_monitoring: false,
                region_count: 12,
            },
            DawTrack {
                track_id: 2,
                name: "Bass DI".to_string(),
                color_rgb: 0x44FF44,
                channel_layout: ChannelLayout::Mono,
                armed: false,
                input_monitoring: false,
                region_count: 8,
            },
            DawTrack {
                track_id: 3,
                name: "Lead Vocal".to_string(),
                color_rgb: 0x4488FF,
                channel_layout: ChannelLayout::Mono,
                armed: true,
                input_monitoring: true,
                region_count: 22,
            },
            DawTrack {
                track_id: 4,
                name: "String Section".to_string(),
                color_rgb: 0xFFCC00,
                channel_layout: ChannelLayout::Stereo,
                armed: false,
                input_monitoring: false,
                region_count: 4,
            },
        ],
        markers: vec![
            (0, "Intro".to_string()),
            (16, "Verse 1".to_string()),
            (48, "Chorus".to_string()),
            (80, "Bridge".to_string()),
            (96, "Outro".to_string()),
        ],
    };

    let encoded = encode_to_vec(&session).expect("encode DawSession failed");
    let compressed = compress(&encoded, Compression::Zstd).expect("compress DawSession failed");
    let decompressed = decompress(&compressed).expect("decompress DawSession failed");
    let (decoded, _): (DawSession, usize) =
        decode_from_slice(&decompressed).expect("decode DawSession failed");
    assert_eq!(session, decoded);
}

/// 9. Beat grid / tempo map round-trip.
#[test]
fn test_zstd_beat_grid_roundtrip() {
    let grid = BeatGrid {
        initial_tempo_bpm_x100: 12_000,
        time_signature_numerator: 4,
        time_signature_denominator: 4,
        tempo_changes: vec![
            TempoEvent {
                bar_number: 0,
                beat_within_bar: 0,
                tempo_bpm_x100: 12_000,
            },
            TempoEvent {
                bar_number: 32,
                beat_within_bar: 0,
                tempo_bpm_x100: 12_500,
            },
            TempoEvent {
                bar_number: 64,
                beat_within_bar: 0,
                tempo_bpm_x100: 11_800,
            },
            TempoEvent {
                bar_number: 96,
                beat_within_bar: 2,
                tempo_bpm_x100: 10_000,
            },
        ],
        downbeat_offsets_samples: (0u64..128).map(|i| i * 96_000).collect(),
    };

    let encoded = encode_to_vec(&grid).expect("encode BeatGrid failed");
    let compressed = compress(&encoded, Compression::Zstd).expect("compress BeatGrid failed");
    let decompressed = decompress(&compressed).expect("decompress BeatGrid failed");
    let (decoded, _): (BeatGrid, usize) =
        decode_from_slice(&decompressed).expect("decode BeatGrid failed");
    assert_eq!(grid, decoded);
}

/// 10. Spatial audio panning descriptor round-trip.
#[test]
fn test_zstd_spatial_pan_descriptor_roundtrip() {
    let pan = SpatialPanDescriptor {
        object_id: 42,
        panning_mode: SpatialPanningMode::DolbyAtmosObjectBased,
        azimuth_deg_x100: -4500,
        elevation_deg_x100: 3000,
        distance_x1000: 2500,
        spread_x1000: 150,
        divergence_x1000: 300,
    };

    let encoded = encode_to_vec(&pan).expect("encode SpatialPanDescriptor failed");
    let compressed =
        compress(&encoded, Compression::Zstd).expect("compress SpatialPanDescriptor failed");
    let decompressed = decompress(&compressed).expect("decompress SpatialPanDescriptor failed");
    let (decoded, _): (SpatialPanDescriptor, usize) =
        decode_from_slice(&decompressed).expect("decode SpatialPanDescriptor failed");
    assert_eq!(pan, decoded);
}

/// 11. Loudness metering result round-trip.
#[test]
fn test_zstd_loudness_metering_roundtrip() {
    let metering = LoudnessMetering {
        integrated_lufs_x100: -1400,
        short_term_lufs_x100: -1280,
        momentary_lufs_x100: -1050,
        loudness_range_lu_x100: 850,
        true_peak_milli_dbfs: -980,
        sample_peak_milli_dbfs: -650,
        measurement_duration_ms: 234_567,
    };

    let encoded = encode_to_vec(&metering).expect("encode LoudnessMetering failed");
    let compressed =
        compress(&encoded, Compression::Zstd).expect("compress LoudnessMetering failed");
    let decompressed = decompress(&compressed).expect("decompress LoudnessMetering failed");
    let (decoded, _): (LoudnessMetering, usize) =
        decode_from_slice(&decompressed).expect("decode LoudnessMetering failed");
    assert_eq!(metering, decoded);
}

/// 12. Plugin preset bank round-trip.
#[test]
fn test_zstd_plugin_preset_bank_roundtrip() {
    let bank = PluginPresetBank {
        plugin_name: "OxiVerb Pro".to_string(),
        plugin_format: PluginFormat::Clap,
        plugin_version: "2.4.1".to_string(),
        presets: vec![
            PluginPreset {
                preset_name: "Large Hall".to_string(),
                category: "Reverb".to_string(),
                author: "Factory".to_string(),
                parameter_values_x1000: vec![900, 650, 800, 500, 350, 1000, 80],
            },
            PluginPreset {
                preset_name: "Tight Room".to_string(),
                category: "Reverb".to_string(),
                author: "Factory".to_string(),
                parameter_values_x1000: vec![300, 400, 600, 700, 500, 1000, 20],
            },
            PluginPreset {
                preset_name: "Shimmer Verb".to_string(),
                category: "Creative".to_string(),
                author: "Sound Designer A".to_string(),
                parameter_values_x1000: vec![950, 800, 900, 300, 250, 800, 150],
            },
        ],
    };

    let encoded = encode_to_vec(&bank).expect("encode PluginPresetBank failed");
    let compressed =
        compress(&encoded, Compression::Zstd).expect("compress PluginPresetBank failed");
    let decompressed = decompress(&compressed).expect("decompress PluginPresetBank failed");
    let (decoded, _): (PluginPresetBank, usize) =
        decode_from_slice(&decompressed).expect("decode PluginPresetBank failed");
    assert_eq!(bank, decoded);
}

/// 13. Studio equipment calibration round-trip.
#[test]
fn test_zstd_studio_calibration_roundtrip() {
    let cal = StudioCalibration {
        equipment_name: "Neumann U87 Ai #3".to_string(),
        serial_number: "SN-87AI-00342".to_string(),
        calibration_date_unix: 1_740_000_000,
        frequency_response_points: vec![
            (20, -500),
            (50, -200),
            (100, 0),
            (1_000, 100),
            (5_000, 300),
            (10_000, 200),
            (15_000, -100),
            (20_000, -800),
        ],
        thd_plus_noise_ppm: 50,
        channel_separation_milli_db: 85_000,
        max_spl_milli_db: 127_000,
    };

    let encoded = encode_to_vec(&cal).expect("encode StudioCalibration failed");
    let compressed =
        compress(&encoded, Compression::Zstd).expect("compress StudioCalibration failed");
    let decompressed = decompress(&compressed).expect("decompress StudioCalibration failed");
    let (decoded, _): (StudioCalibration, usize) =
        decode_from_slice(&decompressed).expect("decode StudioCalibration failed");
    assert_eq!(cal, decoded);
}

/// 14. Session musician credits round-trip.
#[test]
fn test_zstd_musician_credit_roundtrip() {
    let credit = MusicianCredit {
        name: "Takeshi Yamamoto".to_string(),
        instrument: "Electric Guitar".to_string(),
        role: "Lead Guitarist".to_string(),
        tracks_played: vec![1, 3, 5, 7, 11],
        session_date_unix: 1_742_000_000,
        studio_name: "Blue Note Studio Tokyo".to_string(),
    };

    let encoded = encode_to_vec(&credit).expect("encode MusicianCredit failed");
    let compressed = compress(&encoded, Compression::Zstd).expect("compress MusicianCredit failed");
    let decompressed = decompress(&compressed).expect("decompress MusicianCredit failed");
    let (decoded, _): (MusicianCredit, usize) =
        decode_from_slice(&decompressed).expect("decode MusicianCredit failed");
    assert_eq!(credit, decoded);
}

/// 15. Streaming codec configuration round-trip.
#[test]
fn test_zstd_streaming_codec_config_roundtrip() {
    let cfg = StreamingCodecConfig {
        codec: StreamingCodec::OpusVbr,
        bitrate_kbps: 256,
        sample_rate_hz: 48_000,
        channel_layout: ChannelLayout::Stereo,
        vbr_quality_x10: 90,
        low_latency: true,
        frame_size_samples: 960,
    };

    let encoded = encode_to_vec(&cfg).expect("encode StreamingCodecConfig failed");
    let compressed =
        compress(&encoded, Compression::Zstd).expect("compress StreamingCodecConfig failed");
    let decompressed = decompress(&compressed).expect("decompress StreamingCodecConfig failed");
    let (decoded, _): (StreamingCodecConfig, usize) =
        decode_from_slice(&decompressed).expect("decode StreamingCodecConfig failed");
    assert_eq!(cfg, decoded);
}

/// 16. Vinyl mastering specifications round-trip.
#[test]
fn test_zstd_vinyl_mastering_spec_roundtrip() {
    let spec = VinylMasteringSpec {
        cutting_style: VinylCuttingStyle::HalfSpeed,
        rpm: 33,
        groove_width_um: 55,
        land_width_um: 35,
        inner_diameter_mm: 121,
        outer_diameter_mm: 300,
        max_playing_time_sec: 1_320,
        riaa_eq_applied: true,
        high_frequency_limit_hz: 15_000,
        rumble_filter_hz: 30,
        de_esser_threshold_milli_db: -6_000,
    };

    let encoded = encode_to_vec(&spec).expect("encode VinylMasteringSpec failed");
    let compressed =
        compress(&encoded, Compression::Zstd).expect("compress VinylMasteringSpec failed");
    let decompressed = decompress(&compressed).expect("decompress VinylMasteringSpec failed");
    let (decoded, _): (VinylMasteringSpec, usize) =
        decode_from_slice(&decompressed).expect("decode VinylMasteringSpec failed");
    assert_eq!(spec, decoded);
}

/// 17. Compression ratio — 1000 identical MIDI events.
#[test]
fn test_zstd_large_midi_events_compression_ratio() {
    let events: Vec<MidiEvent> = (0u64..1_000)
        .map(|i| MidiEvent {
            tick: i * 480,
            channel: 0,
            message_type: MidiMessageType::NoteOn,
            data_byte_1: 60,
            data_byte_2: 100,
        })
        .collect();

    let encoded = encode_to_vec(&events).expect("encode large MIDI events failed");
    let compressed =
        compress(&encoded, Compression::Zstd).expect("zstd compress MIDI events failed");

    assert!(
        compressed.len() < encoded.len(),
        "Zstd compressed ({} bytes) should be smaller than encoded ({} bytes) for 1000 MIDI events",
        compressed.len(),
        encoded.len(),
    );

    let decompressed = decompress(&compressed).expect("zstd decompress MIDI events failed");
    assert_eq!(decompressed.len(), encoded.len());
    let (decoded, _): (Vec<MidiEvent>, usize) =
        decode_from_slice(&decompressed).expect("decode large MIDI events failed");
    assert_eq!(events, decoded);
}

/// 18. Compression ratio — 500 repetitive channel strips.
#[test]
fn test_zstd_large_channel_strips_compression_ratio() {
    let strips: Vec<ChannelStrip> = (0u8..250)
        .map(|i| ChannelStrip {
            strip_index: i,
            label: format!("Track {}", i),
            gain_milli_db: 0,
            pan_x100: 0,
            phase_invert: false,
            mute: false,
            solo: false,
            eq_bands: vec![
                EqBand {
                    band_type: EqBandType::LowShelf,
                    frequency_hz_x10: 1_000,
                    gain_milli_db: 0,
                    q_factor_x100: 70,
                    enabled: true,
                },
                EqBand {
                    band_type: EqBandType::Bell,
                    frequency_hz_x10: 10_000,
                    gain_milli_db: 0,
                    q_factor_x100: 100,
                    enabled: false,
                },
            ],
            compressor: CompressorSettings {
                threshold_milli_dbfs: -12_000,
                ratio_x10: 40,
                attack_us: 10_000,
                release_us: 100_000,
                makeup_gain_milli_db: 0,
                knee_type: CompressorKneeType::Soft,
                sidechain_hpf_hz: 80,
                enabled: false,
            },
            send_levels_milli_db: vec![-96_000, -96_000],
        })
        .collect();

    let encoded = encode_to_vec(&strips).expect("encode large channel strips failed");
    let compressed =
        compress(&encoded, Compression::Zstd).expect("zstd compress channel strips failed");

    assert!(
        compressed.len() < encoded.len(),
        "Zstd compressed ({} bytes) should be smaller than encoded ({} bytes) for 250 channel strips",
        compressed.len(),
        encoded.len(),
    );

    let decompressed = decompress(&compressed).expect("zstd decompress channel strips failed");
    assert_eq!(decompressed.len(), encoded.len());
    let (decoded, _): (Vec<ChannelStrip>, usize) =
        decode_from_slice(&decompressed).expect("decode large channel strips failed");
    assert_eq!(strips, decoded);
}

/// 19. Compression ratio — 800 spatial audio objects.
#[test]
fn test_zstd_large_spatial_objects_compression_ratio() {
    let objects: Vec<SpatialPanDescriptor> = (0u32..800)
        .map(|i| SpatialPanDescriptor {
            object_id: i,
            panning_mode: SpatialPanningMode::DolbyAtmosObjectBased,
            azimuth_deg_x100: ((i as i32) * 45) % 36_000 - 18_000,
            elevation_deg_x100: ((i as i32) * 10) % 9_000,
            distance_x1000: 1_000 + (i % 5) * 500,
            spread_x1000: 200,
            divergence_x1000: 100,
        })
        .collect();

    let encoded = encode_to_vec(&objects).expect("encode large spatial objects failed");
    let compressed =
        compress(&encoded, Compression::Zstd).expect("zstd compress spatial objects failed");

    assert!(
        compressed.len() < encoded.len(),
        "Zstd compressed ({} bytes) should be smaller than encoded ({} bytes) for 800 spatial objects",
        compressed.len(),
        encoded.len(),
    );

    let decompressed = decompress(&compressed).expect("zstd decompress spatial objects failed");
    assert_eq!(decompressed.len(), encoded.len());
    let (decoded, _): (Vec<SpatialPanDescriptor>, usize) =
        decode_from_slice(&decompressed).expect("decode large spatial objects failed");
    assert_eq!(objects, decoded);
}

/// 20. Vec of loudness metering results round-trip.
#[test]
fn test_zstd_vec_loudness_metering_roundtrip() {
    let meters: Vec<LoudnessMetering> = (0u32..50)
        .map(|i| LoudnessMetering {
            integrated_lufs_x100: -1400 + (i as i32) * 5,
            short_term_lufs_x100: -1300 + (i as i32) * 10,
            momentary_lufs_x100: -1100 + (i as i32) * 20,
            loudness_range_lu_x100: 700 + i * 3,
            true_peak_milli_dbfs: -1000 + (i as i32) * 2,
            sample_peak_milli_dbfs: -800 + (i as i32) * 3,
            measurement_duration_ms: 10_000 + (i as u64) * 500,
        })
        .collect();

    let encoded = encode_to_vec(&meters).expect("encode Vec<LoudnessMetering> failed");
    let compressed =
        compress(&encoded, Compression::Zstd).expect("compress Vec<LoudnessMetering> failed");
    let decompressed = decompress(&compressed).expect("decompress Vec<LoudnessMetering> failed");
    let (decoded, _): (Vec<LoudnessMetering>, usize) =
        decode_from_slice(&decompressed).expect("decode Vec<LoudnessMetering> failed");
    assert_eq!(meters, decoded);
}

/// 21. Multiple musician credits for a full session roster.
#[test]
fn test_zstd_session_roster_roundtrip() {
    let roster: Vec<MusicianCredit> = vec![
        MusicianCredit {
            name: "Keiko Tanaka".to_string(),
            instrument: "Piano / Rhodes".to_string(),
            role: "Keyboard Player".to_string(),
            tracks_played: vec![1, 2, 4, 6, 8, 10],
            session_date_unix: 1_742_000_000,
            studio_name: "Sunset Sound Studio A".to_string(),
        },
        MusicianCredit {
            name: "Marcus Johnson".to_string(),
            instrument: "Fender Precision Bass".to_string(),
            role: "Bassist".to_string(),
            tracks_played: vec![1, 2, 3, 5, 7, 9, 10],
            session_date_unix: 1_742_000_000,
            studio_name: "Sunset Sound Studio A".to_string(),
        },
        MusicianCredit {
            name: "Elena Petrova".to_string(),
            instrument: "Violin / Viola".to_string(),
            role: "String Arranger & Performer".to_string(),
            tracks_played: vec![4, 6, 10],
            session_date_unix: 1_742_100_000,
            studio_name: "Abbey Road Studio 2".to_string(),
        },
        MusicianCredit {
            name: "David Chen".to_string(),
            instrument: "Drum Kit".to_string(),
            role: "Drummer".to_string(),
            tracks_played: vec![1, 2, 3, 5, 7, 9],
            session_date_unix: 1_742_000_000,
            studio_name: "Sunset Sound Studio A".to_string(),
        },
    ];

    let encoded = encode_to_vec(&roster).expect("encode session roster failed");
    let compressed = compress(&encoded, Compression::Zstd).expect("compress session roster failed");
    let decompressed = decompress(&compressed).expect("decompress session roster failed");
    let (decoded, _): (Vec<MusicianCredit>, usize) =
        decode_from_slice(&decompressed).expect("decode session roster failed");
    assert_eq!(roster, decoded);
}

/// 22. Combined audio production pipeline — full session with all components.
#[test]
fn test_zstd_full_production_pipeline_roundtrip() {
    #[derive(Debug, PartialEq, Clone, Encode, Decode)]
    struct ProductionSession {
        session: DawSession,
        beat_grid: BeatGrid,
        mastering: MasteringChain,
        reverb_send: ReverbParams,
        delay_send: DelayParams,
        loudness: LoudnessMetering,
        codec_export: StreamingCodecConfig,
        vinyl_spec: VinylMasteringSpec,
    }

    let production = ProductionSession {
        session: DawSession {
            session_name: "Neon Dreams - Single".to_string(),
            sample_rate_hz: 96_000,
            bit_depth: BitDepth::Float32,
            tempo_bpm_x100: 12_800,
            time_signature_numerator: 4,
            time_signature_denominator: 4,
            total_bars: 96,
            tracks: vec![
                DawTrack {
                    track_id: 1,
                    name: "Drums".to_string(),
                    color_rgb: 0xFF0000,
                    channel_layout: ChannelLayout::Stereo,
                    armed: false,
                    input_monitoring: false,
                    region_count: 8,
                },
                DawTrack {
                    track_id: 2,
                    name: "Synth Bass".to_string(),
                    color_rgb: 0x00FF00,
                    channel_layout: ChannelLayout::Mono,
                    armed: false,
                    input_monitoring: false,
                    region_count: 4,
                },
                DawTrack {
                    track_id: 3,
                    name: "Vocal".to_string(),
                    color_rgb: 0x0088FF,
                    channel_layout: ChannelLayout::Mono,
                    armed: false,
                    input_monitoring: false,
                    region_count: 16,
                },
            ],
            markers: vec![
                (0, "Intro".to_string()),
                (32, "Drop".to_string()),
                (64, "Breakdown".to_string()),
                (80, "Final Drop".to_string()),
            ],
        },
        beat_grid: BeatGrid {
            initial_tempo_bpm_x100: 12_800,
            time_signature_numerator: 4,
            time_signature_denominator: 4,
            tempo_changes: vec![
                TempoEvent {
                    bar_number: 0,
                    beat_within_bar: 0,
                    tempo_bpm_x100: 12_800,
                },
                TempoEvent {
                    bar_number: 64,
                    beat_within_bar: 0,
                    tempo_bpm_x100: 6_400,
                },
                TempoEvent {
                    bar_number: 66,
                    beat_within_bar: 0,
                    tempo_bpm_x100: 12_800,
                },
            ],
            downbeat_offsets_samples: (0u64..96).map(|i| i * 180_000).collect(),
        },
        mastering: MasteringChain {
            session_name: "Neon Dreams Master".to_string(),
            target_lufs_x10: -140,
            true_peak_limit_milli_dbfs: -1000,
            processors: vec![
                MasteringProcessor {
                    slot_index: 0,
                    plugin_name: "Surgical EQ".to_string(),
                    plugin_format: PluginFormat::Clap,
                    bypass: false,
                    wet_dry_x1000: 1000,
                },
                MasteringProcessor {
                    slot_index: 1,
                    plugin_name: "Glue Compressor".to_string(),
                    plugin_format: PluginFormat::Vst3,
                    bypass: false,
                    wet_dry_x1000: 1000,
                },
                MasteringProcessor {
                    slot_index: 2,
                    plugin_name: "True Peak Limiter".to_string(),
                    plugin_format: PluginFormat::Vst3,
                    bypass: false,
                    wet_dry_x1000: 1000,
                },
            ],
            dither_enabled: true,
            output_bit_depth: BitDepth::Pcm16,
            output_sample_rate_hz: 44_100,
        },
        reverb_send: ReverbParams {
            algorithm: ReverbAlgorithm::Plate,
            pre_delay_us: 15_000,
            decay_time_ms: 1_800,
            damping_x1000: 500,
            diffusion_x1000: 750,
            room_size_x1000: 600,
            high_cut_hz: 10_000,
            low_cut_hz: 250,
            wet_x1000: 1000,
            dry_x1000: 0,
            modulation_rate_mhz: 300,
            modulation_depth_x1000: 50,
        },
        delay_send: DelayParams {
            delay_type: DelayType::PingPong,
            time_left_us: 234_375,
            time_right_us: 468_750,
            feedback_x1000: 350,
            high_cut_hz: 5_000,
            low_cut_hz: 200,
            saturation_x1000: 100,
            wet_x1000: 1000,
            sync_to_tempo: true,
        },
        loudness: LoudnessMetering {
            integrated_lufs_x100: -1400,
            short_term_lufs_x100: -1250,
            momentary_lufs_x100: -980,
            loudness_range_lu_x100: 620,
            true_peak_milli_dbfs: -1010,
            sample_peak_milli_dbfs: -780,
            measurement_duration_ms: 210_000,
        },
        codec_export: StreamingCodecConfig {
            codec: StreamingCodec::OpusVbr,
            bitrate_kbps: 320,
            sample_rate_hz: 48_000,
            channel_layout: ChannelLayout::Stereo,
            vbr_quality_x10: 100,
            low_latency: false,
            frame_size_samples: 960,
        },
        vinyl_spec: VinylMasteringSpec {
            cutting_style: VinylCuttingStyle::HalfSpeed,
            rpm: 45,
            groove_width_um: 60,
            land_width_um: 40,
            inner_diameter_mm: 170,
            outer_diameter_mm: 300,
            max_playing_time_sec: 720,
            riaa_eq_applied: true,
            high_frequency_limit_hz: 16_000,
            rumble_filter_hz: 25,
            de_esser_threshold_milli_db: -8_000,
        },
    };

    let encoded = encode_to_vec(&production).expect("encode ProductionSession failed");
    let compressed =
        compress(&encoded, Compression::Zstd).expect("compress ProductionSession failed");

    assert!(
        compressed.len() < encoded.len(),
        "Zstd compressed ({} bytes) should be smaller than encoded ({} bytes) for full production session",
        compressed.len(),
        encoded.len(),
    );

    let decompressed = decompress(&compressed).expect("decompress ProductionSession failed");
    assert_eq!(decompressed.len(), encoded.len());
    let (decoded, _): (ProductionSession, usize) =
        decode_from_slice(&decompressed).expect("decode ProductionSession failed");
    assert_eq!(production, decoded);
}
