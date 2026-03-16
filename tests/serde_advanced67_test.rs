#![cfg(feature = "serde")]
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
use ::serde::{Deserialize, Serialize};
use oxicode::config;
use oxicode::serde::{decode_owned_from_slice, encode_to_vec};

// --- Broadcast media production and CDN domain types ---

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
struct VideoEncodingProfile {
    profile_id: u32,
    codec: String,
    bitrate_kbps: u32,
    width: u16,
    height: u16,
    framerate_num: u32,
    framerate_den: u32,
    keyframe_interval: u16,
    is_hdr: bool,
    color_space: String,
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
struct PlayoutScheduleEntry {
    entry_id: u64,
    channel_id: u32,
    media_asset_id: u64,
    start_epoch_ms: u64,
    duration_ms: u64,
    title: String,
    segment_type: String,
    is_live: bool,
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
struct Scte35Marker {
    marker_id: u64,
    splice_event_id: u32,
    pts_time: u64,
    duration_ticks: u64,
    avail_num: u16,
    avails_expected: u16,
    marker_type: Scte35Type,
    unique_program_id: u16,
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
enum Scte35Type {
    SpliceInsert,
    TimeSignal,
    SpliceNull,
    BandwidthReservation,
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
struct CdnEdgeNode {
    node_id: u32,
    hostname: String,
    region: String,
    capacity_gbps: f32,
    active_connections: u64,
    cache_hit_ratio_pct: f32,
    is_healthy: bool,
    supported_protocols: Vec<String>,
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
enum TranscodingJobState {
    Queued {
        priority: u8,
    },
    InProgress {
        percent_done: f32,
        worker_id: String,
    },
    Completed {
        output_size_bytes: u64,
        elapsed_ms: u64,
    },
    Failed {
        error_code: u32,
        message: String,
    },
    Cancelled {
        reason: String,
    },
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
struct TranscodingJob {
    job_id: u64,
    source_asset_id: u64,
    target_profile_id: u32,
    state: TranscodingJobState,
    created_epoch_ms: u64,
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
struct ContentRightsWindow {
    window_id: u64,
    content_id: u64,
    territory_codes: Vec<String>,
    start_epoch_ms: u64,
    end_epoch_ms: u64,
    license_type: String,
    is_exclusive: bool,
    max_concurrent_streams: u32,
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
struct EpgEntry {
    epg_id: u64,
    channel_id: u32,
    program_title: String,
    episode_title: Option<String>,
    season_number: Option<u16>,
    episode_number: Option<u16>,
    start_epoch_ms: u64,
    duration_ms: u32,
    genre: String,
    parental_rating: String,
    is_live: bool,
    has_audio_description: bool,
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
struct LiveStreamHealth {
    stream_id: u64,
    ingest_bitrate_kbps: u32,
    output_bitrate_kbps: u32,
    dropped_frames: u64,
    encoder_fps: f32,
    buffer_fill_pct: f32,
    latency_ms: u32,
    uptime_seconds: u64,
    error_count: u32,
    status: StreamStatus,
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
enum StreamStatus {
    Healthy,
    Degraded,
    Critical,
    Offline,
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
struct SubtitleTrack {
    track_id: u32,
    language_code: String,
    format: SubtitleFormat,
    is_default: bool,
    is_forced: bool,
    cue_count: u32,
    character_count: u64,
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
enum SubtitleFormat {
    WebVtt,
    Srt,
    Ttml,
    Eia608,
    Eia708,
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
struct AudioMixerChannel {
    channel_id: u8,
    label: String,
    fader_db: f32,
    pan: f32,
    is_muted: bool,
    is_solo: bool,
    eq_low_db: f32,
    eq_mid_db: f32,
    eq_high_db: f32,
    compressor_threshold_db: f32,
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
struct AudioMixConsoleState {
    console_id: u32,
    session_name: String,
    channels: Vec<AudioMixerChannel>,
    master_fader_db: f32,
    sample_rate_hz: u32,
    bit_depth: u8,
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
struct CameraSwitcherPreset {
    preset_id: u32,
    preset_name: String,
    active_input: u8,
    transition_type: TransitionType,
    transition_duration_frames: u16,
    input_labels: Vec<String>,
    tally_states: Vec<bool>,
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
enum TransitionType {
    Cut,
    Dissolve,
    Wipe { direction: u8 },
    Dve { effect_id: u16 },
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
struct MamAssetMetadata {
    asset_id: u64,
    title: String,
    description: String,
    duration_ms: u64,
    file_size_bytes: u64,
    container_format: String,
    video_codec: String,
    audio_codec: String,
    created_epoch_ms: u64,
    tags: Vec<String>,
    thumbnail_uri: String,
    approval_status: ApprovalStatus,
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
enum ApprovalStatus {
    Draft,
    PendingReview,
    Approved,
    Rejected { reason: String },
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
struct AdBreakSchedule {
    break_id: u64,
    channel_id: u32,
    scheduled_epoch_ms: u64,
    break_duration_ms: u32,
    ad_slots: Vec<AdSlot>,
    is_local_insert: bool,
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
struct AdSlot {
    slot_index: u8,
    advertiser_id: u64,
    creative_asset_id: u64,
    duration_ms: u32,
    cpm_micros: u64,
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
struct DrmPolicy {
    policy_id: u64,
    content_id: u64,
    system_name: String,
    license_server_url: String,
    key_rotation_interval_sec: u32,
    allow_offline: bool,
    max_resolution_tier: String,
    persistent_license: bool,
    rental_duration_hours: Option<u32>,
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
struct GraphicsOverlay {
    overlay_id: u32,
    template_name: String,
    layer: u8,
    x_position: f32,
    y_position: f32,
    width_pct: f32,
    height_pct: f32,
    opacity: f32,
    data_fields: Vec<GraphicsField>,
    is_visible: bool,
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
struct GraphicsField {
    field_name: String,
    field_value: String,
}

// --- Tests ---

#[test]
fn test_video_encoding_profile_roundtrip() {
    let val = VideoEncodingProfile {
        profile_id: 101,
        codec: "H.265/HEVC".to_string(),
        bitrate_kbps: 8000,
        width: 3840,
        height: 2160,
        framerate_num: 60000,
        framerate_den: 1001,
        keyframe_interval: 120,
        is_hdr: true,
        color_space: "BT.2020".to_string(),
    };
    let bytes = encode_to_vec(&val, config::standard()).expect("encode video profile");
    let (decoded, _): (VideoEncodingProfile, _) =
        decode_owned_from_slice(&bytes, config::standard()).expect("decode video profile");
    assert_eq!(val, decoded);
}

#[test]
fn test_playout_schedule_entry_roundtrip() {
    let val = PlayoutScheduleEntry {
        entry_id: 990_001,
        channel_id: 42,
        media_asset_id: 550_123,
        start_epoch_ms: 1_710_000_000_000,
        duration_ms: 3_600_000,
        title: "Evening News Bulletin".to_string(),
        segment_type: "PROGRAM".to_string(),
        is_live: true,
    };
    let bytes = encode_to_vec(&val, config::standard()).expect("encode playout entry");
    let (decoded, _): (PlayoutScheduleEntry, _) =
        decode_owned_from_slice(&bytes, config::standard()).expect("decode playout entry");
    assert_eq!(val, decoded);
}

#[test]
fn test_scte35_splice_insert_roundtrip() {
    let val = Scte35Marker {
        marker_id: 8001,
        splice_event_id: 67890,
        pts_time: 5_400_000_000,
        duration_ticks: 2_700_000,
        avail_num: 1,
        avails_expected: 4,
        marker_type: Scte35Type::SpliceInsert,
        unique_program_id: 1234,
    };
    let bytes = encode_to_vec(&val, config::standard()).expect("encode SCTE-35 marker");
    let (decoded, _): (Scte35Marker, _) =
        decode_owned_from_slice(&bytes, config::standard()).expect("decode SCTE-35 marker");
    assert_eq!(val, decoded);
}

#[test]
fn test_scte35_time_signal_roundtrip() {
    let val = Scte35Marker {
        marker_id: 8002,
        splice_event_id: 0,
        pts_time: 7_200_000_000,
        duration_ticks: 0,
        avail_num: 0,
        avails_expected: 0,
        marker_type: Scte35Type::TimeSignal,
        unique_program_id: 5678,
    };
    let bytes = encode_to_vec(&val, config::standard()).expect("encode time signal");
    let (decoded, _): (Scte35Marker, _) =
        decode_owned_from_slice(&bytes, config::standard()).expect("decode time signal");
    assert_eq!(val, decoded);
}

#[test]
fn test_cdn_edge_node_roundtrip() {
    let val = CdnEdgeNode {
        node_id: 301,
        hostname: "edge-nrt-01.cdn.example.com".to_string(),
        region: "ap-northeast-1".to_string(),
        capacity_gbps: 100.0,
        active_connections: 45_320,
        cache_hit_ratio_pct: 94.7,
        is_healthy: true,
        supported_protocols: vec!["HLS".to_string(), "DASH".to_string(), "CMAF".to_string()],
    };
    let bytes = encode_to_vec(&val, config::standard()).expect("encode CDN node");
    let (decoded, _): (CdnEdgeNode, _) =
        decode_owned_from_slice(&bytes, config::standard()).expect("decode CDN node");
    assert_eq!(val, decoded);
}

#[test]
fn test_transcoding_job_queued_roundtrip() {
    let val = TranscodingJob {
        job_id: 10_001,
        source_asset_id: 550_100,
        target_profile_id: 101,
        state: TranscodingJobState::Queued { priority: 5 },
        created_epoch_ms: 1_710_000_000_000,
    };
    let bytes = encode_to_vec(&val, config::standard()).expect("encode queued job");
    let (decoded, _): (TranscodingJob, _) =
        decode_owned_from_slice(&bytes, config::standard()).expect("decode queued job");
    assert_eq!(val, decoded);
}

#[test]
fn test_transcoding_job_in_progress_roundtrip() {
    let val = TranscodingJob {
        job_id: 10_002,
        source_asset_id: 550_200,
        target_profile_id: 203,
        state: TranscodingJobState::InProgress {
            percent_done: 67.5,
            worker_id: "worker-gpu-03".to_string(),
        },
        created_epoch_ms: 1_710_000_100_000,
    };
    let bytes = encode_to_vec(&val, config::standard()).expect("encode in-progress job");
    let (decoded, _): (TranscodingJob, _) =
        decode_owned_from_slice(&bytes, config::standard()).expect("decode in-progress job");
    assert_eq!(val, decoded);
}

#[test]
fn test_transcoding_job_failed_roundtrip() {
    let val = TranscodingJob {
        job_id: 10_003,
        source_asset_id: 550_300,
        target_profile_id: 305,
        state: TranscodingJobState::Failed {
            error_code: 4010,
            message: "Unsupported source codec: ProRes 4444 XQ".to_string(),
        },
        created_epoch_ms: 1_710_000_200_000,
    };
    let bytes = encode_to_vec(&val, config::standard()).expect("encode failed job");
    let (decoded, _): (TranscodingJob, _) =
        decode_owned_from_slice(&bytes, config::standard()).expect("decode failed job");
    assert_eq!(val, decoded);
}

#[test]
fn test_content_rights_window_roundtrip() {
    let val = ContentRightsWindow {
        window_id: 7001,
        content_id: 330_450,
        territory_codes: vec![
            "JP".to_string(),
            "US".to_string(),
            "GB".to_string(),
            "DE".to_string(),
        ],
        start_epoch_ms: 1_704_067_200_000,
        end_epoch_ms: 1_735_689_600_000,
        license_type: "SVOD".to_string(),
        is_exclusive: true,
        max_concurrent_streams: 3,
    };
    let bytes = encode_to_vec(&val, config::standard()).expect("encode rights window");
    let (decoded, _): (ContentRightsWindow, _) =
        decode_owned_from_slice(&bytes, config::standard()).expect("decode rights window");
    assert_eq!(val, decoded);
}

#[test]
fn test_epg_entry_with_episode_roundtrip() {
    let val = EpgEntry {
        epg_id: 2_000_001,
        channel_id: 12,
        program_title: "Detective Conan".to_string(),
        episode_title: Some("The Scarlet Return".to_string()),
        season_number: Some(28),
        episode_number: Some(927),
        start_epoch_ms: 1_710_075_600_000,
        duration_ms: 1_800_000,
        genre: "Animation".to_string(),
        parental_rating: "PG".to_string(),
        is_live: false,
        has_audio_description: false,
    };
    let bytes = encode_to_vec(&val, config::standard()).expect("encode EPG entry");
    let (decoded, _): (EpgEntry, _) =
        decode_owned_from_slice(&bytes, config::standard()).expect("decode EPG entry");
    assert_eq!(val, decoded);
}

#[test]
fn test_epg_entry_live_no_episode_roundtrip() {
    let val = EpgEntry {
        epg_id: 2_000_002,
        channel_id: 1,
        program_title: "World Cup Final".to_string(),
        episode_title: None,
        season_number: None,
        episode_number: None,
        start_epoch_ms: 1_710_090_000_000,
        duration_ms: 7_200_000,
        genre: "Sports".to_string(),
        parental_rating: "G".to_string(),
        is_live: true,
        has_audio_description: true,
    };
    let bytes = encode_to_vec(&val, config::standard()).expect("encode live EPG");
    let (decoded, _): (EpgEntry, _) =
        decode_owned_from_slice(&bytes, config::standard()).expect("decode live EPG");
    assert_eq!(val, decoded);
}

#[test]
fn test_live_stream_health_healthy_roundtrip() {
    let val = LiveStreamHealth {
        stream_id: 5001,
        ingest_bitrate_kbps: 15_000,
        output_bitrate_kbps: 12_000,
        dropped_frames: 0,
        encoder_fps: 59.94,
        buffer_fill_pct: 42.0,
        latency_ms: 850,
        uptime_seconds: 14_400,
        error_count: 0,
        status: StreamStatus::Healthy,
    };
    let bytes = encode_to_vec(&val, config::standard()).expect("encode healthy stream");
    let (decoded, _): (LiveStreamHealth, _) =
        decode_owned_from_slice(&bytes, config::standard()).expect("decode healthy stream");
    assert_eq!(val, decoded);
}

#[test]
fn test_live_stream_health_degraded_roundtrip() {
    let val = LiveStreamHealth {
        stream_id: 5002,
        ingest_bitrate_kbps: 6_000,
        output_bitrate_kbps: 4_500,
        dropped_frames: 1_247,
        encoder_fps: 29.97,
        buffer_fill_pct: 88.5,
        latency_ms: 4_200,
        uptime_seconds: 7_200,
        error_count: 23,
        status: StreamStatus::Degraded,
    };
    let bytes = encode_to_vec(&val, config::standard()).expect("encode degraded stream");
    let (decoded, _): (LiveStreamHealth, _) =
        decode_owned_from_slice(&bytes, config::standard()).expect("decode degraded stream");
    assert_eq!(val, decoded);
}

#[test]
fn test_subtitle_track_roundtrip() {
    let val = SubtitleTrack {
        track_id: 3,
        language_code: "ja".to_string(),
        format: SubtitleFormat::WebVtt,
        is_default: true,
        is_forced: false,
        cue_count: 1_842,
        character_count: 48_300,
    };
    let bytes = encode_to_vec(&val, config::standard()).expect("encode subtitle track");
    let (decoded, _): (SubtitleTrack, _) =
        decode_owned_from_slice(&bytes, config::standard()).expect("decode subtitle track");
    assert_eq!(val, decoded);
}

#[test]
fn test_subtitle_track_eia708_roundtrip() {
    let val = SubtitleTrack {
        track_id: 7,
        language_code: "en".to_string(),
        format: SubtitleFormat::Eia708,
        is_default: false,
        is_forced: true,
        cue_count: 3_210,
        character_count: 92_500,
    };
    let bytes = encode_to_vec(&val, config::standard()).expect("encode EIA-708 track");
    let (decoded, _): (SubtitleTrack, _) =
        decode_owned_from_slice(&bytes, config::standard()).expect("decode EIA-708 track");
    assert_eq!(val, decoded);
}

#[test]
fn test_audio_mix_console_state_roundtrip() {
    let channels = vec![
        AudioMixerChannel {
            channel_id: 1,
            label: "Host Mic".to_string(),
            fader_db: -6.0,
            pan: 0.0,
            is_muted: false,
            is_solo: false,
            eq_low_db: -2.0,
            eq_mid_db: 1.5,
            eq_high_db: 3.0,
            compressor_threshold_db: -18.0,
        },
        AudioMixerChannel {
            channel_id: 2,
            label: "Guest Mic".to_string(),
            fader_db: -8.0,
            pan: -0.3,
            is_muted: false,
            is_solo: false,
            eq_low_db: -4.0,
            eq_mid_db: 0.0,
            eq_high_db: 2.0,
            compressor_threshold_db: -20.0,
        },
        AudioMixerChannel {
            channel_id: 3,
            label: "Background Music".to_string(),
            fader_db: -24.0,
            pan: 0.0,
            is_muted: true,
            is_solo: false,
            eq_low_db: 0.0,
            eq_mid_db: 0.0,
            eq_high_db: -6.0,
            compressor_threshold_db: -12.0,
        },
    ];
    let val = AudioMixConsoleState {
        console_id: 77,
        session_name: "Live Morning Show 2026-03-15".to_string(),
        channels,
        master_fader_db: -3.0,
        sample_rate_hz: 48_000,
        bit_depth: 24,
    };
    let bytes = encode_to_vec(&val, config::standard()).expect("encode mix console");
    let (decoded, _): (AudioMixConsoleState, _) =
        decode_owned_from_slice(&bytes, config::standard()).expect("decode mix console");
    assert_eq!(val, decoded);
}

#[test]
fn test_camera_switcher_preset_cut_roundtrip() {
    let val = CameraSwitcherPreset {
        preset_id: 1,
        preset_name: "Interview 2-cam".to_string(),
        active_input: 1,
        transition_type: TransitionType::Cut,
        transition_duration_frames: 0,
        input_labels: vec![
            "Camera A (Wide)".to_string(),
            "Camera B (Close)".to_string(),
        ],
        tally_states: vec![false, true],
    };
    let bytes = encode_to_vec(&val, config::standard()).expect("encode switcher preset cut");
    let (decoded, _): (CameraSwitcherPreset, _) =
        decode_owned_from_slice(&bytes, config::standard()).expect("decode switcher preset cut");
    assert_eq!(val, decoded);
}

#[test]
fn test_camera_switcher_preset_dve_roundtrip() {
    let val = CameraSwitcherPreset {
        preset_id: 5,
        preset_name: "Sports Replay PiP".to_string(),
        active_input: 0,
        transition_type: TransitionType::Dve { effect_id: 204 },
        transition_duration_frames: 15,
        input_labels: vec![
            "Main Feed".to_string(),
            "Replay Server".to_string(),
            "Graphics".to_string(),
            "Crowd Cam".to_string(),
        ],
        tally_states: vec![true, false, false, false],
    };
    let bytes = encode_to_vec(&val, config::standard()).expect("encode switcher DVE");
    let (decoded, _): (CameraSwitcherPreset, _) =
        decode_owned_from_slice(&bytes, config::standard()).expect("decode switcher DVE");
    assert_eq!(val, decoded);
}

#[test]
fn test_mam_asset_metadata_roundtrip() {
    let val = MamAssetMetadata {
        asset_id: 1_200_345,
        title: "Sakura Season Documentary".to_string(),
        description: "A 4K documentary capturing cherry blossom season across Japan".to_string(),
        duration_ms: 5_400_000,
        file_size_bytes: 42_000_000_000,
        container_format: "MXF".to_string(),
        video_codec: "XAVC-I".to_string(),
        audio_codec: "PCM".to_string(),
        created_epoch_ms: 1_709_900_000_000,
        tags: vec![
            "documentary".to_string(),
            "nature".to_string(),
            "japan".to_string(),
            "4k".to_string(),
            "spring".to_string(),
        ],
        thumbnail_uri: "mam://assets/1200345/thumb_001.jpg".to_string(),
        approval_status: ApprovalStatus::Approved,
    };
    let bytes = encode_to_vec(&val, config::standard()).expect("encode MAM asset");
    let (decoded, _): (MamAssetMetadata, _) =
        decode_owned_from_slice(&bytes, config::standard()).expect("decode MAM asset");
    assert_eq!(val, decoded);
}

#[test]
fn test_ad_break_schedule_roundtrip() {
    let val = AdBreakSchedule {
        break_id: 60_001,
        channel_id: 42,
        scheduled_epoch_ms: 1_710_079_200_000,
        break_duration_ms: 180_000,
        ad_slots: vec![
            AdSlot {
                slot_index: 0,
                advertiser_id: 800_100,
                creative_asset_id: 900_200,
                duration_ms: 30_000,
                cpm_micros: 25_000_000,
            },
            AdSlot {
                slot_index: 1,
                advertiser_id: 800_200,
                creative_asset_id: 900_350,
                duration_ms: 60_000,
                cpm_micros: 42_000_000,
            },
            AdSlot {
                slot_index: 2,
                advertiser_id: 800_300,
                creative_asset_id: 900_410,
                duration_ms: 15_000,
                cpm_micros: 18_500_000,
            },
        ],
        is_local_insert: false,
    };
    let bytes = encode_to_vec(&val, config::standard()).expect("encode ad break");
    let (decoded, _): (AdBreakSchedule, _) =
        decode_owned_from_slice(&bytes, config::standard()).expect("decode ad break");
    assert_eq!(val, decoded);
}

#[test]
fn test_drm_policy_roundtrip() {
    let val = DrmPolicy {
        policy_id: 4001,
        content_id: 330_450,
        system_name: "Widevine".to_string(),
        license_server_url: "https://drm.example.com/widevine/license".to_string(),
        key_rotation_interval_sec: 3600,
        allow_offline: true,
        max_resolution_tier: "UHD".to_string(),
        persistent_license: true,
        rental_duration_hours: Some(48),
    };
    let bytes = encode_to_vec(&val, config::standard()).expect("encode DRM policy");
    let (decoded, _): (DrmPolicy, _) =
        decode_owned_from_slice(&bytes, config::standard()).expect("decode DRM policy");
    assert_eq!(val, decoded);
}

#[test]
fn test_graphics_overlay_with_data_fields_roundtrip() {
    let val = GraphicsOverlay {
        overlay_id: 55,
        template_name: "lower_third_score".to_string(),
        layer: 3,
        x_position: 0.05,
        y_position: 0.82,
        width_pct: 0.4,
        height_pct: 0.12,
        opacity: 0.95,
        data_fields: vec![
            GraphicsField {
                field_name: "home_team".to_string(),
                field_value: "Tokyo Verdy".to_string(),
            },
            GraphicsField {
                field_name: "away_team".to_string(),
                field_value: "Yokohama FC".to_string(),
            },
            GraphicsField {
                field_name: "score".to_string(),
                field_value: "2 - 1".to_string(),
            },
            GraphicsField {
                field_name: "match_time".to_string(),
                field_value: "78'".to_string(),
            },
        ],
        is_visible: true,
    };
    let bytes = encode_to_vec(&val, config::standard()).expect("encode graphics overlay");
    let (decoded, _): (GraphicsOverlay, _) =
        decode_owned_from_slice(&bytes, config::standard()).expect("decode graphics overlay");
    assert_eq!(val, decoded);
}
